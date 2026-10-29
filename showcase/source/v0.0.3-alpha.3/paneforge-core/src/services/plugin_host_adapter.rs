use crate::models::{
    validate_broker_snapshot, validate_host_snapshot, validate_process_identity,
    PluginBrokerChannel, PluginBrokerReceipt, PluginBrokerRequest, PluginCapabilityPolicy,
    PluginCapabilitySnapshot, PluginEnvironmentPolicy, PluginGrant, PluginHostExecutionRequest,
    PluginHostReceipt, PluginManifest, PluginProcessIdentity,
};
use crate::repository::PluginRepository;
use crate::utils::error::{AppError, AppResult};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::{Child, Command};
use std::sync::Mutex;

/// Boundary for a future sandboxed host. Implementations must perform the final
/// revision and capability check immediately before touching the operating system.
pub trait PluginHostAdapter: Send + Sync {
    fn execute(
        &self,
        manifest: &PluginManifest,
        request: &PluginHostExecutionRequest,
        adapter_actor: &str,
        now: i64,
    ) -> AppResult<PluginHostReceipt>;
}

pub trait PluginCapabilityBroker: Send + Sync {
    fn authorize(&self, snapshot: &PluginCapabilitySnapshot, grant: PluginGrant) -> AppResult<()>;
    fn authorize_filesystem_read(
        &self,
        snapshot: &PluginCapabilitySnapshot,
        relative_path: &str,
    ) -> AppResult<PathBuf>;
    fn authorize_network(
        &self,
        snapshot: &PluginCapabilitySnapshot,
        destination: &str,
    ) -> AppResult<()>;
    fn authorize_secret(&self, snapshot: &PluginCapabilitySnapshot, scope: &str) -> AppResult<()>;
    fn authorize_ui(&self, snapshot: &PluginCapabilitySnapshot, channel: &str) -> AppResult<()>;
}

#[derive(Debug, Clone)]
pub struct PluginHostLaunchPolicy {
    pub executable_root: PathBuf,
    pub allowed_executables: Vec<String>,
    pub working_directory: PathBuf,
    pub arguments: Vec<String>,
    pub environment: PluginEnvironmentPolicy,
}

struct ManagedProcess {
    child: Child,
    identity: PluginProcessIdentity,
    guard: ProcessGuard,
}

#[derive(Default)]
struct ProcessGuard {
    #[cfg(windows)]
    job: Option<windows::Win32::Foundation::HANDLE>,
    #[cfg(unix)]
    pgid: Option<i32>,
}

// SAFETY: the Windows Job Object handle is owned exclusively by ProcessGuard,
// accessed behind the adapter mutex, and closed exactly once in Drop.
#[cfg(windows)]
unsafe impl Send for ProcessGuard {}
#[cfg(windows)]
unsafe impl Sync for ProcessGuard {}

impl ProcessGuard {
    fn attach(child: &Child) -> AppResult<Self> {
        #[cfg(windows)]
        {
            use std::os::windows::io::AsRawHandle;
            use windows::core::PCWSTR;
            use windows::Win32::Foundation::CloseHandle;
            use windows::Win32::System::JobObjects::{
                AssignProcessToJobObject, CreateJobObjectW, JobObjectExtendedLimitInformation,
                SetInformationJobObject, JOBOBJECT_EXTENDED_LIMIT_INFORMATION,
                JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE,
            };
            use windows::Win32::System::Threading::OpenProcess;
            use windows::Win32::System::Threading::{PROCESS_SET_QUOTA, PROCESS_TERMINATE};
            // SAFETY: handles are checked and owned by the returned guard.
            unsafe {
                let job = CreateJobObjectW(None, PCWSTR::null())
                    .map_err(|e| AppError::coded("PLUGIN_JOB_CREATE_FAILED", e.to_string()))?;
                let mut info = JOBOBJECT_EXTENDED_LIMIT_INFORMATION::default();
                info.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
                if let Err(error) = SetInformationJobObject(
                    job,
                    JobObjectExtendedLimitInformation,
                    &info as *const _ as *const _,
                    std::mem::size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>() as u32,
                ) {
                    let _ = CloseHandle(job);
                    return Err(AppError::coded(
                        "PLUGIN_JOB_CONFIG_FAILED",
                        error.to_string(),
                    ));
                }
                let process = OpenProcess(PROCESS_TERMINATE | PROCESS_SET_QUOTA, false, child.id())
                    .map_err(|error| {
                        AppError::coded("PLUGIN_PROCESS_ATTACH_FAILED", error.to_string())
                    })?;
                let assigned = AssignProcessToJobObject(job, process);
                let _ = CloseHandle(process);
                if let Err(error) = assigned {
                    let _ = CloseHandle(job);
                    return Err(AppError::coded(
                        "PLUGIN_PROCESS_ATTACH_FAILED",
                        error.to_string(),
                    ));
                }
                let _ = child.as_raw_handle();
                Ok(Self { job: Some(job) })
            }
        }
        #[cfg(unix)]
        {
            Ok(Self {
                pgid: Some(child.id() as i32),
            })
        }
        #[cfg(not(any(windows, unix)))]
        {
            let _ = child;
            Ok(Self::default())
        }
    }

    fn terminate(&self, child: &mut Child) {
        #[cfg(windows)]
        if let Some(job) = self.job {
            use windows::Win32::System::JobObjects::TerminateJobObject;
            // SAFETY: job is owned by this guard until drop.
            unsafe {
                let _ = TerminateJobObject(job, 1);
            }
        }
        #[cfg(unix)]
        if let Some(pgid) = self.pgid {
            // SAFETY: pgid was created for this child by process_group(0).
            unsafe {
                let _ = libc::killpg(pgid, libc::SIGTERM);
            }
        }
        let _ = child.kill();
    }
}

impl Drop for ProcessGuard {
    fn drop(&mut self) {
        #[cfg(windows)]
        if let Some(job) = self.job.take() {
            use windows::Win32::Foundation::CloseHandle;
            // SAFETY: this guard owns the job handle exactly once.
            unsafe {
                let _ = CloseHandle(job);
            }
        }
    }
}

pub struct ManagedPluginHostAdapter {
    broker: StaticCapabilityBroker,
    policy: PluginHostLaunchPolicy,
    processes: Mutex<HashMap<(String, String), ManagedProcess>>,
}

pub struct PluginHostService<A> {
    repository: PluginRepository,
    adapter: A,
    adapter_actor: String,
}

impl<A: PluginHostAdapter> PluginHostService<A> {
    pub fn new(repository: PluginRepository, adapter: A, adapter_actor: String) -> AppResult<Self> {
        if adapter_actor.trim().is_empty() {
            return Err(AppError::coded(
                "PLUGIN_HOST_REQUEST_INVALID",
                "Adapter actor is required",
            ));
        }
        Ok(Self {
            repository,
            adapter,
            adapter_actor,
        })
    }

    pub fn repository(&self) -> &PluginRepository {
        &self.repository
    }

    pub fn execute(
        &self,
        request: &PluginHostExecutionRequest,
        now: i64,
    ) -> AppResult<PluginHostReceipt> {
        let manifest = self
            .repository
            .get(&request.plugin_id)?
            .ok_or_else(|| AppError::coded("PLUGIN_MISSING", "Plugin does not exist"))?;
        let key = (&request.plugin_id, &request.plan_id);
        match request.operation {
            crate::models::PluginHostOperation::Start => {
                let planned = crate::models::PluginLifecycleRecord {
                    plugin_id: request.plugin_id.clone(),
                    plan_id: request.plan_id.clone(),
                    expected_revision: request.expected_revision,
                    observed_revision: manifest.revision,
                    state: crate::models::PluginLifecycleState::Planned,
                    process: None,
                    occurred_at: now,
                    error: None,
                };
                if self.repository.lifecycle(key.0, key.1)?.is_none() {
                    self.repository.transition_lifecycle(&planned, None)?;
                }
                let starting = crate::models::PluginLifecycleRecord {
                    state: crate::models::PluginLifecycleState::Starting,
                    ..planned.clone()
                };
                self.repository.transition_lifecycle(
                    &starting,
                    Some(crate::models::PluginLifecycleState::Planned),
                )?;
            }
            crate::models::PluginHostOperation::Stop => {
                let current = self.repository.lifecycle(key.0, key.1)?.ok_or_else(|| {
                    AppError::coded(
                        "PLUGIN_LIFECYCLE_CONFLICT",
                        "No durable lifecycle record exists",
                    )
                })?;
                let stopping = crate::models::PluginLifecycleRecord {
                    state: crate::models::PluginLifecycleState::Stopping,
                    occurred_at: now,
                    ..current.clone()
                };
                self.repository.transition_lifecycle(
                    &stopping,
                    Some(crate::models::PluginLifecycleState::Started),
                )?;
            }
            crate::models::PluginHostOperation::Reconcile => {}
        }
        let receipt = match self
            .adapter
            .execute(&manifest, request, &self.adapter_actor, now)
        {
            Ok(receipt) => receipt,
            Err(error) => {
                let current = self.repository.lifecycle(key.0, key.1)?;
                let failure = match (request.operation, current) {
                    (crate::models::PluginHostOperation::Start, Some(current))
                        if current.state == crate::models::PluginLifecycleState::Starting =>
                    {
                        Some((crate::models::PluginLifecycleState::Starting, None))
                    }
                    (crate::models::PluginHostOperation::Stop, Some(current))
                        if current.state == crate::models::PluginLifecycleState::Stopping =>
                    {
                        Some((
                            crate::models::PluginLifecycleState::Stopping,
                            current.process,
                        ))
                    }
                    (crate::models::PluginHostOperation::Reconcile, Some(current))
                        if current.state == crate::models::PluginLifecycleState::Started =>
                    {
                        Some((
                            crate::models::PluginLifecycleState::Started,
                            current.process,
                        ))
                    }
                    _ => None,
                };
                if let Some((expected, process)) = failure {
                    let state =
                        if request.operation == crate::models::PluginHostOperation::Reconcile {
                            crate::models::PluginLifecycleState::Crashed
                        } else {
                            crate::models::PluginLifecycleState::Failed
                        };
                    let failed = crate::models::PluginLifecycleRecord {
                        plugin_id: request.plugin_id.clone(),
                        plan_id: request.plan_id.clone(),
                        expected_revision: request.expected_revision,
                        observed_revision: manifest.revision,
                        state,
                        process,
                        occurred_at: now,
                        error: Some(error.to_string()),
                    };
                    let _ = self
                        .repository
                        .transition_lifecycle(&failed, Some(expected));
                }
                return Err(error);
            }
        };
        let next_state = match receipt.outcome {
            crate::models::PluginExecutionOutcome::Started => {
                Some(crate::models::PluginLifecycleState::Started)
            }
            crate::models::PluginExecutionOutcome::Stopped => {
                Some(crate::models::PluginLifecycleState::Stopped)
            }
            _ => None,
        };
        if let Some(state) = next_state {
            let current = self.repository.lifecycle(key.0, key.1)?;
            let expected = current.as_ref().map(|record| record.state);
            let record = crate::models::PluginLifecycleRecord {
                plugin_id: request.plugin_id.clone(),
                plan_id: request.plan_id.clone(),
                expected_revision: request.expected_revision,
                observed_revision: receipt.observed_revision,
                state,
                process: receipt.process.clone(),
                occurred_at: now,
                error: receipt.error.clone(),
            };
            self.repository.transition_lifecycle(&record, expected)?;
        }
        Ok(receipt)
    }
}

impl ManagedPluginHostAdapter {
    pub fn new(broker: StaticCapabilityBroker, policy: PluginHostLaunchPolicy) -> AppResult<Self> {
        let root = std::fs::canonicalize(&policy.executable_root).map_err(|error| {
            AppError::coded("PLUGIN_EXECUTABLE_ROOT_INVALID", error.to_string())
        })?;
        let working = std::fs::canonicalize(&policy.working_directory).map_err(|error| {
            AppError::coded("PLUGIN_WORKING_DIRECTORY_INVALID", error.to_string())
        })?;
        if !working.starts_with(&root) {
            return Err(AppError::coded(
                "PLUGIN_WORKING_DIRECTORY_DENIED",
                "Working directory must be inside executable root",
            ));
        }
        Ok(Self {
            broker,
            policy: PluginHostLaunchPolicy {
                executable_root: root,
                working_directory: working,
                ..policy
            },
            processes: Mutex::new(HashMap::new()),
        })
    }

    fn resolve_entrypoint(&self, manifest: &PluginManifest) -> AppResult<PathBuf> {
        let relative = Path::new(&manifest.entrypoint);
        let candidate = self.policy.executable_root.join(relative);
        let canonical = std::fs::canonicalize(candidate)
            .map_err(|error| AppError::coded("PLUGIN_EXECUTABLE_MISSING", error.to_string()))?;
        if !canonical.starts_with(&self.policy.executable_root)
            || !canonical.is_file()
            || !self.policy.allowed_executables.iter().any(|name| {
                canonical.file_name().and_then(|value| value.to_str()) == Some(name.as_str())
            })
        {
            return Err(AppError::coded(
                "PLUGIN_EXECUTABLE_DENIED",
                "Entrypoint is outside the executable allowlist",
            ));
        }
        Ok(canonical)
    }
}

impl PluginHostAdapter for ManagedPluginHostAdapter {
    fn execute(
        &self,
        manifest: &PluginManifest,
        request: &PluginHostExecutionRequest,
        adapter_actor: &str,
        now: i64,
    ) -> AppResult<PluginHostReceipt> {
        let snapshot = validate_execution_request(manifest, request, adapter_actor, now)?;
        let key = (request.plugin_id.clone(), request.plan_id.clone());
        let mut processes = self.processes.lock().map_err(|_| {
            AppError::coded("PLUGIN_HOST_LOCK_FAILED", "Plugin host state lock poisoned")
        })?;
        let (outcome, process, error) = match request.operation {
            crate::models::PluginHostOperation::Start => {
                if processes.contains_key(&key) {
                    return Err(AppError::coded(
                        "PLUGIN_LIFECYCLE_CONFLICT",
                        "Plugin is already started",
                    ));
                }
                let executable = self.resolve_entrypoint(manifest)?;
                let environment =
                    self.broker
                        .sanitize_environment(&snapshot, &self.policy.environment, &[])?;
                let mut command = Command::new(executable);
                command
                    .args(&self.policy.arguments)
                    .current_dir(&self.policy.working_directory)
                    .env_clear()
                    .envs(environment);
                #[cfg(unix)]
                {
                    use std::os::unix::process::CommandExt;
                    command.process_group(0);
                }
                let child = command.spawn().map_err(|error| {
                    AppError::coded("PLUGIN_PROCESS_START_FAILED", error.to_string())
                })?;
                let guard = match ProcessGuard::attach(&child) {
                    Ok(guard) => guard,
                    Err(error) => {
                        let mut child = child;
                        let _ = child.kill();
                        let _ = child.wait();
                        return Err(error);
                    }
                };
                let identity = match observe_process_identity(child.id()) {
                    Ok(identity) => identity,
                    Err(error) => {
                        let mut child = child;
                        guard.terminate(&mut child);
                        let _ = child.wait();
                        return Err(error);
                    }
                };
                processes.insert(
                    key.clone(),
                    ManagedProcess {
                        child,
                        identity: identity.clone(),
                        guard,
                    },
                );
                (
                    crate::models::PluginExecutionOutcome::Started,
                    Some(identity),
                    None,
                )
            }
            crate::models::PluginHostOperation::Stop => {
                let mut managed = processes.remove(&key).ok_or_else(|| {
                    AppError::coded("PLUGIN_PROCESS_MISSING", "No managed plugin process exists")
                })?;
                let observed = observe_process_identity(managed.child.id())?;
                if let Err(error) = verify_process_identity(&managed.identity, &observed) {
                    processes.insert(key, managed);
                    return Err(error);
                }
                managed.guard.terminate(&mut managed.child);
                let _ = managed.child.wait();
                (
                    crate::models::PluginExecutionOutcome::Stopped,
                    Some(observed),
                    None,
                )
            }
            crate::models::PluginHostOperation::Reconcile => {
                let managed = processes.get(&key).ok_or_else(|| {
                    AppError::coded("PLUGIN_PROCESS_MISSING", "No managed plugin process exists")
                })?;
                let observed = observe_process_identity(managed.child.id())?;
                verify_process_identity(&managed.identity, &observed)?;
                (
                    crate::models::PluginExecutionOutcome::Recovered,
                    Some(observed),
                    None,
                )
            }
        };
        Ok(PluginHostReceipt {
            plugin_id: request.plugin_id.clone(),
            plan_id: request.plan_id.clone(),
            expected_revision: request.expected_revision,
            observed_revision: snapshot.revision,
            operation: request.operation,
            outcome,
            adapter_actor: adapter_actor.into(),
            process,
            capability_snapshot: snapshot,
            occurred_at: now,
            error,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StaticCapabilityBroker {
    policy: PluginCapabilityPolicy,
}

impl StaticCapabilityBroker {
    pub fn new(policy: PluginCapabilityPolicy) -> Self {
        Self { policy }
    }
    pub fn policy(&self) -> &PluginCapabilityPolicy {
        &self.policy
    }

    pub fn canonicalize_filesystem_read(
        &self,
        snapshot: &PluginCapabilitySnapshot,
        relative_path: &str,
    ) -> AppResult<PathBuf> {
        let candidate = self.authorize_filesystem_read(snapshot, relative_path)?;
        let root = self.policy.filesystem_root.as_deref().ok_or_else(|| {
            AppError::coded(
                "PLUGIN_FILESYSTEM_ROOT_MISSING",
                "Filesystem root is not configured",
            )
        })?;
        let canonical_root = std::fs::canonicalize(root).map_err(|error| {
            AppError::coded("PLUGIN_FILESYSTEM_ROOT_INVALID", error.to_string())
        })?;
        let canonical = std::fs::canonicalize(&candidate).map_err(|error| {
            AppError::coded("PLUGIN_FILESYSTEM_PATH_MISSING", error.to_string())
        })?;
        if !canonical.starts_with(&canonical_root) {
            return Err(AppError::coded(
                "PLUGIN_FILESYSTEM_SYMLINK_ESCAPE",
                "Filesystem path resolves outside the broker root",
            ));
        }
        if !std::fs::metadata(&canonical)
            .map(|metadata| metadata.is_file())
            .unwrap_or(false)
        {
            return Err(AppError::coded(
                "PLUGIN_FILESYSTEM_PATH_DENIED",
                "Filesystem capability only permits regular files",
            ));
        }
        Ok(canonical)
    }

    pub fn sanitize_environment(
        &self,
        snapshot: &PluginCapabilitySnapshot,
        policy: &PluginEnvironmentPolicy,
        environment: &[(String, String)],
    ) -> AppResult<Vec<(String, String)>> {
        self.authorize(snapshot, PluginGrant::ProcessSpawn)?;
        let mut result = Vec::with_capacity(environment.len());
        for (key, value) in environment {
            if key.trim().is_empty()
                || key.contains('\0')
                || key.contains('=')
                || value.contains('\0')
                || !policy.allowed_keys.iter().any(|allowed| allowed == key)
                || is_sensitive_environment_key(key)
            {
                return Err(AppError::coded(
                    "PLUGIN_ENVIRONMENT_DENIED",
                    "Environment key is not explicitly allowlisted",
                ));
            }
            if result.iter().any(|(existing, _)| existing == key) {
                return Err(AppError::coded(
                    "PLUGIN_ENVIRONMENT_DUPLICATE",
                    "Environment key appears more than once",
                ));
            }
            result.push((key.clone(), value.clone()));
        }
        Ok(result)
    }
}

fn is_sensitive_environment_key(key: &str) -> bool {
    let upper = key.to_ascii_uppercase();
    matches!(
        upper.as_str(),
        "PATH" | "PATHEXT" | "COMSPEC" | "SYSTEMROOT" | "HOME" | "USERPROFILE"
    ) || upper.contains("TOKEN")
        || upper.contains("SECRET")
        || upper.contains("PASSWORD")
        || upper.contains("API_KEY")
}

impl PluginCapabilityBroker for StaticCapabilityBroker {
    fn authorize(&self, snapshot: &PluginCapabilitySnapshot, grant: PluginGrant) -> AppResult<()> {
        if !snapshot.grants.contains(&grant) {
            return Err(AppError::coded(
                "PLUGIN_CAPABILITY_NOT_APPROVED",
                "Capability is absent from the approved snapshot",
            ));
        }
        let supported = match grant {
            PluginGrant::FilesystemRead => self.policy.filesystem_root.is_some(),
            PluginGrant::ProcessSpawn => true,
            PluginGrant::Network => self.policy.allow_network,
            PluginGrant::Secrets => self.policy.allow_secrets,
            PluginGrant::UiContribution => self.policy.allow_ui_contribution,
        };
        if !supported {
            return Err(AppError::coded(
                "PLUGIN_CAPABILITY_UNSUPPORTED",
                "Capability is disabled by the host policy",
            ));
        }
        Ok(())
    }

    fn authorize_filesystem_read(
        &self,
        snapshot: &PluginCapabilitySnapshot,
        relative_path: &str,
    ) -> AppResult<PathBuf> {
        self.authorize(snapshot, PluginGrant::FilesystemRead)?;
        let root = self.policy.filesystem_root.as_deref().ok_or_else(|| {
            AppError::coded(
                "PLUGIN_FILESYSTEM_ROOT_MISSING",
                "Filesystem root is not configured",
            )
        })?;
        let path = Path::new(relative_path);
        if relative_path.trim().is_empty()
            || relative_path.contains('\0')
            // Treat Windows separators as traversal syntax on every platform;
            // otherwise a manifest validated on Unix could escape when reused
            // by a Windows host (and vice versa).
            || relative_path.contains('\\')
            || path.is_absolute()
            || path.has_root()
            || relative_path
                .as_bytes()
                .get(1)
                .is_some_and(|byte| *byte == b':')
            || path
                .components()
                .any(|component| matches!(component, std::path::Component::ParentDir))
        {
            return Err(AppError::coded(
                "PLUGIN_FILESYSTEM_PATH_DENIED",
                "Filesystem path must be a relative path inside the broker root",
            ));
        }
        let root = PathBuf::from(root);
        let candidate = root.join(path);
        if !candidate.starts_with(&root) {
            return Err(AppError::coded(
                "PLUGIN_FILESYSTEM_PATH_DENIED",
                "Filesystem path escapes the broker root",
            ));
        }
        Ok(candidate)
    }

    fn authorize_network(
        &self,
        snapshot: &PluginCapabilitySnapshot,
        destination: &str,
    ) -> AppResult<()> {
        self.authorize(snapshot, PluginGrant::Network)?;
        let host = destination
            .split_once("://")
            .map(|(_, value)| value)
            .unwrap_or(destination)
            .split(['/', '?', '#'])
            .next()
            .unwrap_or_default()
            .trim();
        if host.is_empty()
            || !self
                .policy
                .allowed_network_hosts
                .iter()
                .any(|allowed| allowed.eq_ignore_ascii_case(host))
        {
            return Err(AppError::coded(
                "PLUGIN_NETWORK_DESTINATION_DENIED",
                "Network destination is not explicitly allowlisted",
            ));
        }
        Ok(())
    }

    fn authorize_secret(&self, snapshot: &PluginCapabilitySnapshot, scope: &str) -> AppResult<()> {
        self.authorize(snapshot, PluginGrant::Secrets)?;
        if scope.trim().is_empty()
            || !self
                .policy
                .allowed_secret_scopes
                .iter()
                .any(|allowed| allowed == scope)
        {
            return Err(AppError::coded(
                "PLUGIN_SECRET_SCOPE_DENIED",
                "Secret scope is not explicitly allowlisted",
            ));
        }
        Ok(())
    }

    fn authorize_ui(&self, snapshot: &PluginCapabilitySnapshot, channel: &str) -> AppResult<()> {
        self.authorize(snapshot, PluginGrant::UiContribution)?;
        if channel.trim().is_empty()
            || !self
                .policy
                .allowed_ui_channels
                .iter()
                .any(|allowed| allowed == channel)
        {
            return Err(AppError::coded(
                "PLUGIN_UI_CHANNEL_DENIED",
                "UI contribution channel is not explicitly allowlisted",
            ));
        }
        Ok(())
    }
}

/// Explicitly unavailable adapter used until the sandbox and lifecycle contract is
/// implemented. It proves callers cannot mistake planning for execution.
#[derive(Debug, Default, Clone, Copy)]
pub struct UnavailablePluginHostAdapter;

impl PluginHostAdapter for UnavailablePluginHostAdapter {
    fn execute(
        &self,
        _manifest: &PluginManifest,
        _request: &PluginHostExecutionRequest,
        _adapter_actor: &str,
        _now: i64,
    ) -> AppResult<PluginHostReceipt> {
        Err(AppError::coded(
            "PLUGIN_HOST_UNAVAILABLE",
            "Plugin host execution requires an accepted sandbox adapter",
        ))
    }
}

pub fn validate_execution_request(
    manifest: &PluginManifest,
    request: &PluginHostExecutionRequest,
    adapter_actor: &str,
    now: i64,
) -> AppResult<crate::models::PluginCapabilitySnapshot> {
    if adapter_actor.trim().is_empty() || now < 0 {
        return Err(AppError::coded(
            "PLUGIN_HOST_REQUEST_INVALID",
            "Adapter actor and occurrence time are required",
        ));
    }
    validate_host_snapshot(manifest, request)
        .map_err(|field| AppError::coded("PLUGIN_HOST_DENIED", field))
}

pub fn authorize_broker_request<B: PluginCapabilityBroker>(
    repository: &PluginRepository,
    broker: &B,
    request: &PluginBrokerRequest,
    adapter_actor: &str,
    now: i64,
) -> AppResult<PluginBrokerReceipt> {
    if adapter_actor.trim().is_empty() || now < 0 || request.reason.trim().is_empty() {
        return Err(AppError::coded(
            "PLUGIN_BROKER_REQUEST_INVALID",
            "Actor, reason and occurrence time are required",
        ));
    }
    let manifest = repository
        .get(&request.plugin_id)?
        .ok_or_else(|| AppError::coded("PLUGIN_MISSING", "Plugin does not exist"))?;
    let grant = match request.channel {
        PluginBrokerChannel::Network => PluginGrant::Network,
        PluginBrokerChannel::Secret => PluginGrant::Secrets,
        PluginBrokerChannel::Ui => PluginGrant::UiContribution,
    };
    let snapshot = validate_broker_snapshot(
        &manifest,
        &request.plugin_id,
        request.expected_revision,
        grant,
    )
    .map_err(|field| AppError::coded("PLUGIN_BROKER_DENIED", field))?;
    match request.channel {
        PluginBrokerChannel::Network => broker.authorize_network(&snapshot, &request.resource)?,
        PluginBrokerChannel::Secret => broker.authorize_secret(&snapshot, &request.resource)?,
        PluginBrokerChannel::Ui => broker.authorize_ui(&snapshot, &request.resource)?,
    }
    Ok(PluginBrokerReceipt {
        plugin_id: request.plugin_id.clone(),
        plan_id: request.plan_id.clone(),
        expected_revision: request.expected_revision,
        observed_revision: snapshot.revision,
        channel: request.channel,
        resource: request.resource.clone(),
        allowed: true,
        adapter_actor: adapter_actor.to_string(),
        occurred_at: now,
        reason: request.reason.clone(),
    })
}

/// Read an OS-backed process identity token without opening a control/termination
/// handle. The token must be persisted with a PID; a later stop/reconcile operation
/// must compare both values before touching the process.
pub fn observe_process_identity(pid: u32) -> AppResult<PluginProcessIdentity> {
    if pid == 0 {
        return Err(AppError::coded(
            "PLUGIN_PROCESS_MISSING",
            "Process ID is required",
        ));
    }
    let identity = observe_process_identity_platform(pid)?;
    validate_process_identity(&identity)
        .map_err(|field| AppError::coded("PLUGIN_PROCESS_IDENTITY_INVALID", field))?;
    Ok(identity)
}

/// Require both PID and the OS start token to match before a future stop or
/// reconciliation operation may act on a process. A recycled PID is rejected.
pub fn verify_process_identity(
    expected: &PluginProcessIdentity,
    observed: &PluginProcessIdentity,
) -> AppResult<()> {
    validate_process_identity(expected)
        .map_err(|field| AppError::coded("PLUGIN_PROCESS_IDENTITY_INVALID", field))?;
    validate_process_identity(observed)
        .map_err(|field| AppError::coded("PLUGIN_PROCESS_IDENTITY_INVALID", field))?;
    if expected.pid != observed.pid || expected.process_start_token != observed.process_start_token
    {
        return Err(AppError::coded(
            "PLUGIN_PROCESS_IDENTITY_MISMATCH",
            "Observed process identity does not match the durable record",
        ));
    }
    Ok(())
}

#[cfg(windows)]
fn observe_process_identity_platform(pid: u32) -> AppResult<PluginProcessIdentity> {
    use windows::Win32::Foundation::{CloseHandle, FILETIME};
    use windows::Win32::System::Threading::{
        GetProcessTimes, OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION,
    };
    // SAFETY: OpenProcess returns an owned handle; GetProcessTimes writes only to
    // the local FILETIME values and CloseHandle is called exactly once.
    unsafe {
        let process = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid)
            .map_err(|error| AppError::coded("PLUGIN_PROCESS_MISSING", error.to_string()))?;
        let mut creation = FILETIME::default();
        let mut exit = FILETIME::default();
        let mut kernel = FILETIME::default();
        let mut user = FILETIME::default();
        let result = GetProcessTimes(process, &mut creation, &mut exit, &mut kernel, &mut user)
            .map_err(|error| AppError::coded("PLUGIN_PROCESS_IDENTITY", error.to_string()));
        let _ = CloseHandle(process);
        result.map(|_| PluginProcessIdentity {
            pid,
            process_start_token: format!(
                "win-filetime:{:08x}{:08x}",
                creation.dwHighDateTime, creation.dwLowDateTime
            ),
        })
    }
}

#[cfg(unix)]
fn observe_process_identity_platform(pid: u32) -> AppResult<PluginProcessIdentity> {
    let path = format!("/proc/{pid}/stat");
    let stat = std::fs::read_to_string(path)
        .map_err(|error| AppError::coded("PLUGIN_PROCESS_MISSING", error.to_string()))?;
    let fields = stat
        .rsplit_once(')')
        .map(|(_, rest)| rest.split_whitespace().collect::<Vec<_>>())
        .ok_or_else(|| AppError::coded("PLUGIN_PROCESS_IDENTITY", "Malformed process stat"))?;
    let start_time = fields
        .get(19)
        .ok_or_else(|| AppError::coded("PLUGIN_PROCESS_IDENTITY", "Missing process start time"))?;
    Ok(PluginProcessIdentity {
        pid,
        process_start_token: format!("linux-proc-start:{start_time}"),
    })
}

#[cfg(not(any(windows, unix)))]
fn observe_process_identity_platform(_pid: u32) -> AppResult<PluginProcessIdentity> {
    Err(AppError::coded(
        "PLUGIN_PROCESS_IDENTITY_UNSUPPORTED",
        "Process identity is unsupported on this platform",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{
        validate_plugin_lifecycle_transition, PluginGrant, PluginLifecycleState, PluginStatus,
    };
    #[cfg(any(unix, windows))]
    use crate::models::{PluginExecutionOutcome, PluginHostOperation};
    use crate::services::PluginService;

    fn manifest() -> PluginManifest {
        PluginManifest {
            plugin_id: "fixture".into(),
            version: "1".into(),
            display_name: "Fixture".into(),
            entrypoint: "plugins/fixture.dll".into(),
            requested_grants: vec![PluginGrant::ProcessSpawn],
            status: PluginStatus::Enabled,
            approved_by: Some("desktop:reviewer".into()),
            idempotency_key: "register".into(),
            registered_at: 1,
            revision: 1,
            registered_by: Some("desktop:reviewer".into()),
            approved_at: Some(2),
            last_action_by: Some("desktop:reviewer".into()),
            last_action_reason: Some("approve".into()),
            changed_at: Some(2),
        }
    }
    fn request(revision: i64) -> PluginHostExecutionRequest {
        PluginHostExecutionRequest {
            plugin_id: "fixture".into(),
            plan_id: "plan-1".into(),
            expected_revision: revision,
            operation: crate::models::PluginHostOperation::Start,
            reason: "fixture".into(),
        }
    }

    #[test]
    fn final_host_validation_rechecks_approval_revision_and_grant() {
        let snapshot =
            validate_execution_request(&manifest(), &request(1), "system:adapter", 3).unwrap();
        assert_eq!(snapshot.revision, 1);
        assert_eq!(snapshot.grants, vec![PluginGrant::ProcessSpawn]);
        assert_eq!(
            validate_execution_request(&manifest(), &request(0), "system:adapter", 3)
                .unwrap_err()
                .code(),
            Some("PLUGIN_HOST_DENIED")
        );
        let mut disabled = manifest();
        disabled.status = PluginStatus::Disabled;
        assert_eq!(
            validate_execution_request(&disabled, &request(1), "system:adapter", 3)
                .unwrap_err()
                .code(),
            Some("PLUGIN_HOST_DENIED")
        );
        let mut ui_only = manifest();
        ui_only.requested_grants = vec![PluginGrant::UiContribution];
        assert_eq!(
            validate_execution_request(&ui_only, &request(1), "system:adapter", 3)
                .unwrap_err()
                .code(),
            Some("PLUGIN_HOST_DENIED")
        );
    }

    #[test]
    fn unavailable_adapter_never_claims_started_or_returns_process_identity() {
        let error = UnavailablePluginHostAdapter
            .execute(&manifest(), &request(1), "system:adapter", 3)
            .unwrap_err();
        assert_eq!(error.code(), Some("PLUGIN_HOST_UNAVAILABLE"));
    }

    #[test]
    fn request_and_process_identity_reject_reusable_or_empty_tokens() {
        assert!(
            crate::models::validate_process_identity(&crate::models::PluginProcessIdentity {
                pid: 0,
                process_start_token: "token".into()
            })
            .is_err()
        );
        assert!(
            crate::models::validate_process_identity(&crate::models::PluginProcessIdentity {
                pid: 42,
                process_start_token: "".into()
            })
            .is_err()
        );
        assert!(validate_execution_request(&manifest(), &request(1), " ", 3).is_err());
        assert!(
            validate_execution_request(&manifest(), &request(1), "system:adapter", -1).is_err()
        );
    }

    #[test]
    fn current_process_has_an_os_backed_identity_token() {
        let identity = observe_process_identity(std::process::id()).unwrap();
        assert_eq!(identity.pid, std::process::id());
        assert!(identity.process_start_token.contains(':'));
    }

    #[test]
    fn process_identity_verification_rejects_pid_reuse_and_token_change() {
        let expected = PluginProcessIdentity {
            pid: 42,
            process_start_token: "start-a".into(),
        };
        assert!(verify_process_identity(&expected, &expected).is_ok());
        for observed in [
            PluginProcessIdentity {
                pid: 43,
                process_start_token: "start-a".into(),
            },
            PluginProcessIdentity {
                pid: 42,
                process_start_token: "start-b".into(),
            },
        ] {
            assert_eq!(
                verify_process_identity(&expected, &observed)
                    .unwrap_err()
                    .code(),
                Some("PLUGIN_PROCESS_IDENTITY_MISMATCH")
            );
        }
    }

    #[cfg(unix)]
    #[test]
    fn managed_fixture_adapter_starts_reconciles_and_stops_owned_process() {
        use std::os::unix::fs::PermissionsExt;
        let directory = tempfile::tempdir().unwrap();
        let script = directory.path().join("fixture.sh");
        std::fs::write(&script, "#!/bin/sh\nsleep 30\n").unwrap();
        std::fs::set_permissions(&script, std::fs::Permissions::from_mode(0o700)).unwrap();
        let broker = StaticCapabilityBroker::new(PluginCapabilityPolicy::default());
        let adapter = ManagedPluginHostAdapter::new(
            broker,
            PluginHostLaunchPolicy {
                executable_root: directory.path().to_path_buf(),
                allowed_executables: vec!["fixture.sh".into()],
                working_directory: directory.path().to_path_buf(),
                arguments: vec![],
                environment: PluginEnvironmentPolicy::default(),
            },
        )
        .unwrap();
        let mut fixture = manifest();
        fixture.entrypoint = "fixture.sh".into();
        let started = adapter
            .execute(&fixture, &request(1), "system:fixture", 3)
            .unwrap();
        assert_eq!(started.outcome, PluginExecutionOutcome::Started);
        assert!(started.process.is_some());
        let recovered = adapter
            .execute(
                &fixture,
                &PluginHostExecutionRequest {
                    operation: PluginHostOperation::Reconcile,
                    ..request(1)
                },
                "system:fixture",
                4,
            )
            .unwrap();
        assert_eq!(recovered.outcome, PluginExecutionOutcome::Recovered);
        let stopped = adapter
            .execute(
                &fixture,
                &PluginHostExecutionRequest {
                    operation: PluginHostOperation::Stop,
                    ..request(1)
                },
                "system:fixture",
                5,
            )
            .unwrap();
        assert_eq!(stopped.outcome, PluginExecutionOutcome::Stopped);
    }

    #[cfg(windows)]
    #[test]
    fn managed_windows_fixture_adapter_uses_job_object_for_start_and_stop() {
        let root = PathBuf::from(r"C:\Windows\System32");
        let broker = StaticCapabilityBroker::new(PluginCapabilityPolicy::default());
        let adapter = ManagedPluginHostAdapter::new(
            broker,
            PluginHostLaunchPolicy {
                executable_root: root.clone(),
                allowed_executables: vec!["cmd.exe".into()],
                working_directory: root,
                arguments: vec!["/C".into(), "ping -n 30 127.0.0.1 > nul".into()],
                environment: PluginEnvironmentPolicy::default(),
            },
        )
        .unwrap();
        let mut fixture = manifest();
        fixture.entrypoint = "cmd.exe".into();
        let started = adapter
            .execute(&fixture, &request(1), "system:windows-fixture", 3)
            .unwrap();
        assert_eq!(started.outcome, PluginExecutionOutcome::Started);
        assert!(started.process.is_some());
        let stopped = adapter
            .execute(
                &fixture,
                &PluginHostExecutionRequest {
                    operation: PluginHostOperation::Stop,
                    ..request(1)
                },
                "system:windows-fixture",
                4,
            )
            .unwrap();
        assert_eq!(stopped.outcome, PluginExecutionOutcome::Stopped);
    }

    #[test]
    fn host_service_records_failed_start_without_claiming_execution() {
        let repository = crate::repository::PluginRepository::in_memory().unwrap();
        let mut registered = manifest();
        registered.status = PluginStatus::Registered;
        registered.approved_by = None;
        repository.register(&registered).unwrap();
        let service = PluginHostService::new(
            repository,
            UnavailablePluginHostAdapter,
            "system:adapter".into(),
        )
        .unwrap();
        assert!(service.execute(&request(0), 3).is_err());
        assert_eq!(
            service
                .repository()
                .lifecycle("fixture", "plan-1")
                .unwrap()
                .unwrap()
                .state,
            PluginLifecycleState::Failed
        );
    }

    #[test]
    fn capability_broker_is_default_deny_and_requires_manifest_grants() {
        let snapshot =
            validate_execution_request(&manifest(), &request(1), "system:adapter", 3).unwrap();
        let broker = StaticCapabilityBroker::new(Default::default());
        assert!(broker
            .authorize(&snapshot, PluginGrant::ProcessSpawn)
            .is_ok());
        assert_eq!(
            broker
                .authorize(&snapshot, PluginGrant::Network)
                .unwrap_err()
                .code(),
            Some("PLUGIN_CAPABILITY_NOT_APPROVED")
        );
        let mut expanded = snapshot.clone();
        expanded.grants.push(PluginGrant::Network);
        assert_eq!(
            broker
                .authorize(&expanded, PluginGrant::Network)
                .unwrap_err()
                .code(),
            Some("PLUGIN_CAPABILITY_UNSUPPORTED")
        );
        let mut policy = PluginCapabilityPolicy::default();
        policy.allow_network = true;
        let broker = StaticCapabilityBroker::new(policy);
        assert!(broker.authorize(&expanded, PluginGrant::Network).is_ok());
    }

    #[test]
    fn broker_channels_require_explicit_resource_allowlists() {
        let mut snapshot = validate_execution_request(
            &manifest_with_grants(vec![
                PluginGrant::ProcessSpawn,
                PluginGrant::Network,
                PluginGrant::Secrets,
                PluginGrant::UiContribution,
            ]),
            &request(1),
            "system:adapter",
            3,
        )
        .unwrap();
        let mut policy = PluginCapabilityPolicy {
            allow_network: true,
            allow_secrets: true,
            allow_ui_contribution: true,
            ..Default::default()
        };
        let broker = StaticCapabilityBroker::new(policy.clone());
        assert_eq!(
            broker
                .authorize_network(&snapshot, "https://example.test/api")
                .unwrap_err()
                .code(),
            Some("PLUGIN_NETWORK_DESTINATION_DENIED")
        );
        policy.allowed_network_hosts.push("example.test".into());
        policy.allowed_secret_scopes.push("plugin/read".into());
        policy.allowed_ui_channels.push("status".into());
        let broker = StaticCapabilityBroker::new(policy);
        assert!(broker
            .authorize_network(&snapshot, "https://example.test/api")
            .is_ok());
        assert!(broker.authorize_secret(&snapshot, "plugin/read").is_ok());
        assert!(broker.authorize_ui(&snapshot, "status").is_ok());
        assert_eq!(
            broker
                .authorize_secret(&snapshot, "plugin/write")
                .unwrap_err()
                .code(),
            Some("PLUGIN_SECRET_SCOPE_DENIED")
        );
        snapshot
            .grants
            .retain(|grant| *grant != PluginGrant::UiContribution);
        assert_eq!(
            broker.authorize_ui(&snapshot, "status").unwrap_err().code(),
            Some("PLUGIN_CAPABILITY_NOT_APPROVED")
        );
    }

    #[test]
    fn broker_request_returns_audited_receipt_only_after_revision_and_policy_checks() {
        let service = PluginService::new(crate::repository::PluginRepository::in_memory().unwrap());
        service
            .register(
                crate::models::PluginRegistrationInput {
                    plugin_id: "fixture".into(),
                    version: "1".into(),
                    display_name: "Fixture".into(),
                    entrypoint: "plugins/fixture.dll".into(),
                    requested_grants: vec![PluginGrant::Network],
                },
                crate::models::ControlPlaneCommandMetadata {
                    command_id: "register".into(),
                    idempotency_key: "register".into(),
                    expected_revision: 0,
                    requested_at: 1,
                    reason: Some("fixture".into()),
                },
                &crate::models::ControlPlaneActorContext::local_desktop("fixture"),
                1,
            )
            .unwrap();
        let approved = service
            .approve(
                crate::models::PluginApprovalInput {
                    plugin_id: "fixture".into(),
                    confirmed_grants: vec![PluginGrant::Network],
                },
                crate::models::ControlPlaneCommandMetadata {
                    command_id: "approve".into(),
                    idempotency_key: "approve".into(),
                    expected_revision: 0,
                    requested_at: 2,
                    reason: Some("grant network".into()),
                },
                &crate::models::ControlPlaneActorContext::local_desktop("fixture"),
                2,
            )
            .unwrap();
        let mut policy = PluginCapabilityPolicy {
            allow_network: true,
            ..Default::default()
        };
        policy.allowed_network_hosts.push("example.test".into());
        let request = PluginBrokerRequest {
            plugin_id: "fixture".into(),
            plan_id: "plan-1".into(),
            expected_revision: approved.plugin.revision,
            channel: PluginBrokerChannel::Network,
            resource: "https://example.test/api".into(),
            reason: "health check".into(),
        };
        let receipt = authorize_broker_request(
            service.repository(),
            &StaticCapabilityBroker::new(policy),
            &request,
            "system:adapter",
            3,
        )
        .unwrap();
        assert!(receipt.allowed);
        assert_eq!(receipt.observed_revision, approved.plugin.revision);
        assert_eq!(receipt.adapter_actor, "system:adapter");
        assert_eq!(
            authorize_broker_request(
                service.repository(),
                &StaticCapabilityBroker::new(PluginCapabilityPolicy::default()),
                &request,
                "system:adapter",
                4,
            )
            .unwrap_err()
            .code(),
            Some("PLUGIN_CAPABILITY_UNSUPPORTED")
        );
    }

    fn manifest_with_grants(grants: Vec<PluginGrant>) -> PluginManifest {
        let mut value = manifest();
        value.requested_grants = grants;
        value
    }

    #[test]
    fn filesystem_reads_require_root_and_reject_escape_paths() {
        let mut snapshot =
            validate_execution_request(&manifest(), &request(1), "system:adapter", 3).unwrap();
        snapshot.grants.push(PluginGrant::FilesystemRead);
        let mut policy = PluginCapabilityPolicy::default();
        policy.filesystem_root = Some(if cfg!(windows) {
            r"C:\PaneForge\plugins\fixture".into()
        } else {
            "/tmp/paneforge/plugins/fixture".into()
        });
        let broker = StaticCapabilityBroker::new(policy);
        let path = broker
            .authorize_filesystem_read(&snapshot, "config/settings.json")
            .unwrap();
        assert!(path.ends_with(Path::new("config/settings.json")));
        for escaped in [
            "../secret.txt",
            "..\\secret.txt",
            "/etc/passwd",
            "C:secret.txt",
            "",
        ] {
            assert_eq!(
                broker
                    .authorize_filesystem_read(&snapshot, escaped)
                    .unwrap_err()
                    .code(),
                Some("PLUGIN_FILESYSTEM_PATH_DENIED")
            );
        }
        let mut no_grant = snapshot.clone();
        no_grant
            .grants
            .retain(|grant| *grant != PluginGrant::FilesystemRead);
        assert_eq!(
            broker
                .authorize_filesystem_read(&no_grant, "config/settings.json")
                .unwrap_err()
                .code(),
            Some("PLUGIN_CAPABILITY_NOT_APPROVED")
        );
    }

    #[test]
    fn environment_is_empty_by_default_and_never_inherits_sensitive_keys() {
        let snapshot =
            validate_execution_request(&manifest(), &request(1), "system:adapter", 3).unwrap();
        let empty = PluginEnvironmentPolicy::default();
        assert_eq!(
            StaticCapabilityBroker::new(Default::default())
                .sanitize_environment(
                    &snapshot,
                    &empty,
                    &[("PLUGIN_MODE".into(), "fixture".into())]
                )
                .unwrap_err()
                .code(),
            Some("PLUGIN_ENVIRONMENT_DENIED")
        );
        let policy = PluginEnvironmentPolicy {
            allowed_keys: vec!["PLUGIN_MODE".into(), "PATH".into(), "PLUGIN_TOKEN".into()],
        };
        let broker = StaticCapabilityBroker::new(Default::default());
        assert_eq!(
            broker
                .sanitize_environment(&snapshot, &policy, &[("PATH".into(), "host-path".into())])
                .unwrap_err()
                .code(),
            Some("PLUGIN_ENVIRONMENT_DENIED")
        );
        assert_eq!(
            broker
                .sanitize_environment(
                    &snapshot,
                    &policy,
                    &[("PLUGIN_TOKEN".into(), "secret".into())]
                )
                .unwrap_err()
                .code(),
            Some("PLUGIN_ENVIRONMENT_DENIED")
        );
        assert_eq!(
            broker
                .sanitize_environment(
                    &snapshot,
                    &policy,
                    &[
                        ("PLUGIN_MODE".into(), "fixture".into()),
                        ("PLUGIN_MODE".into(), "duplicate".into())
                    ]
                )
                .unwrap_err()
                .code(),
            Some("PLUGIN_ENVIRONMENT_DUPLICATE")
        );
        assert_eq!(
            broker
                .sanitize_environment(
                    &snapshot,
                    &policy,
                    &[("PLUGIN_MODE".into(), "fixture".into())]
                )
                .unwrap(),
            vec![("PLUGIN_MODE".into(), "fixture".into())]
        );
    }

    #[test]
    fn canonical_filesystem_reads_reject_missing_and_symlink_escape() {
        let directory = tempfile::tempdir().unwrap();
        let root = directory.path().join("root");
        let outside = directory.path().join("outside");
        std::fs::create_dir_all(&root).unwrap();
        std::fs::create_dir_all(&outside).unwrap();
        std::fs::write(root.join("allowed.txt"), "fixture").unwrap();
        std::fs::write(outside.join("secret.txt"), "secret").unwrap();
        let mut policy = PluginCapabilityPolicy::default();
        policy.filesystem_root = Some(root.to_string_lossy().into_owned());
        let broker = StaticCapabilityBroker::new(policy);
        let mut snapshot =
            validate_execution_request(&manifest(), &request(1), "system:adapter", 3).unwrap();
        snapshot.grants.push(PluginGrant::FilesystemRead);
        assert_eq!(
            broker
                .canonicalize_filesystem_read(&snapshot, "allowed.txt")
                .unwrap(),
            std::fs::canonicalize(root.join("allowed.txt")).unwrap()
        );
        assert_eq!(
            broker
                .canonicalize_filesystem_read(&snapshot, "missing.txt")
                .unwrap_err()
                .code(),
            Some("PLUGIN_FILESYSTEM_PATH_MISSING")
        );

        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(outside.join("secret.txt"), root.join("escape.txt"))
                .unwrap();
            assert_eq!(
                broker
                    .canonicalize_filesystem_read(&snapshot, "escape.txt")
                    .unwrap_err()
                    .code(),
                Some("PLUGIN_FILESYSTEM_SYMLINK_ESCAPE")
            );
        }
    }

    #[test]
    fn lifecycle_transition_matrix_requires_explicit_pending_and_recovery_states() {
        assert!(validate_plugin_lifecycle_transition(
            PluginLifecycleState::Planned,
            PluginLifecycleState::Starting
        )
        .is_ok());
        assert!(validate_plugin_lifecycle_transition(
            PluginLifecycleState::Starting,
            PluginLifecycleState::Failed
        )
        .is_ok());
        assert!(validate_plugin_lifecycle_transition(
            PluginLifecycleState::Started,
            PluginLifecycleState::Crashed
        )
        .is_ok());
        assert!(validate_plugin_lifecycle_transition(
            PluginLifecycleState::Crashed,
            PluginLifecycleState::Recovered
        )
        .is_ok());
        for (current, next) in [
            (PluginLifecycleState::Planned, PluginLifecycleState::Started),
            (
                PluginLifecycleState::Starting,
                PluginLifecycleState::Stopped,
            ),
            (PluginLifecycleState::Crashed, PluginLifecycleState::Started),
            (
                PluginLifecycleState::Stopped,
                PluginLifecycleState::Starting,
            ),
        ] {
            assert!(validate_plugin_lifecycle_transition(current, next).is_err());
        }
    }
}
