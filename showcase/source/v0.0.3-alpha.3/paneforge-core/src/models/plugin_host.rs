use super::{PluginGrant, PluginManifest, PluginStatus};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PluginBrokerChannel {
    Network,
    Secret,
    Ui,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PluginBrokerRequest {
    pub plugin_id: String,
    pub plan_id: String,
    pub expected_revision: i64,
    pub channel: PluginBrokerChannel,
    pub resource: String,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginBrokerReceipt {
    pub plugin_id: String,
    pub plan_id: String,
    pub expected_revision: i64,
    pub observed_revision: i64,
    pub channel: PluginBrokerChannel,
    pub resource: String,
    pub allowed: bool,
    pub adapter_actor: String,
    pub occurred_at: i64,
    pub reason: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PluginHostOperation {
    Start,
    Stop,
    Reconcile,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PluginExecutionOutcome {
    Started,
    Stopped,
    Denied,
    Failed,
    Crashed,
    Recovered,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PluginLifecycleState {
    Planned,
    Starting,
    Started,
    Stopping,
    Stopped,
    Failed,
    Crashed,
    Recovered,
}

impl PluginLifecycleState {
    /// Validate a durable lifecycle transition before it is persisted.
    /// Pending states are intentionally explicit so restart reconciliation can
    /// distinguish an unfinished operation from a successful one.
    pub fn can_transition_to(self, next: Self) -> bool {
        matches!(
            (self, next),
            (Self::Planned, Self::Starting)
                | (Self::Starting, Self::Started)
                | (Self::Starting, Self::Failed)
                | (Self::Started, Self::Stopping)
                | (Self::Started, Self::Crashed)
                | (Self::Stopping, Self::Stopped)
                | (Self::Stopping, Self::Failed)
                | (Self::Crashed, Self::Recovered)
        )
    }
}

pub fn validate_plugin_lifecycle_transition(
    current: PluginLifecycleState,
    next: PluginLifecycleState,
) -> Result<(), &'static str> {
    if current.can_transition_to(next) {
        Ok(())
    } else {
        Err("plugin_lifecycle_transition")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PluginHostExecutionRequest {
    pub plugin_id: String,
    pub plan_id: String,
    pub expected_revision: i64,
    pub operation: PluginHostOperation,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginCapabilitySnapshot {
    pub plugin_id: String,
    pub revision: i64,
    pub grants: Vec<PluginGrant>,
    pub entrypoint: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PluginCapabilityPolicy {
    pub filesystem_root: Option<String>,
    pub allow_network: bool,
    pub allow_secrets: bool,
    pub allow_ui_contribution: bool,
    #[serde(default)]
    pub allowed_network_hosts: Vec<String>,
    #[serde(default)]
    pub allowed_secret_scopes: Vec<String>,
    #[serde(default)]
    pub allowed_ui_channels: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PluginEnvironmentPolicy {
    pub allowed_keys: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginProcessIdentity {
    pub pid: u32,
    pub process_start_token: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginLifecycleRecord {
    pub plugin_id: String,
    pub plan_id: String,
    pub expected_revision: i64,
    pub observed_revision: i64,
    pub state: PluginLifecycleState,
    pub process: Option<PluginProcessIdentity>,
    pub occurred_at: i64,
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginHostReceipt {
    pub plugin_id: String,
    pub plan_id: String,
    pub expected_revision: i64,
    pub observed_revision: i64,
    pub operation: PluginHostOperation,
    pub outcome: PluginExecutionOutcome,
    pub adapter_actor: String,
    pub process: Option<PluginProcessIdentity>,
    pub capability_snapshot: PluginCapabilitySnapshot,
    pub occurred_at: i64,
    pub error: Option<String>,
}

pub fn validate_host_snapshot(
    manifest: &PluginManifest,
    request: &PluginHostExecutionRequest,
) -> Result<PluginCapabilitySnapshot, &'static str> {
    if request.plugin_id.trim().is_empty() || request.plan_id.trim().is_empty() {
        return Err("identity");
    }
    if request.expected_revision < 0 || request.reason.trim().is_empty() {
        return Err("request");
    }
    if manifest.plugin_id != request.plugin_id || manifest.revision != request.expected_revision {
        return Err("revision");
    }
    if manifest.status != PluginStatus::Enabled
        || manifest.approved_at.is_none()
        || manifest.approved_by.as_deref().is_none_or(str::is_empty)
    {
        return Err("approval");
    }
    if !manifest
        .requested_grants
        .contains(&PluginGrant::ProcessSpawn)
    {
        return Err("process_spawn");
    }
    let mut entrypoint = manifest.clone();
    entrypoint.status = PluginStatus::Registered;
    entrypoint.approved_by = None;
    entrypoint.validate().map_err(|_| "entrypoint")?;
    Ok(PluginCapabilitySnapshot {
        plugin_id: manifest.plugin_id.clone(),
        revision: manifest.revision,
        grants: manifest.requested_grants.clone(),
        entrypoint: manifest.entrypoint.clone(),
    })
}

pub fn validate_broker_snapshot(
    manifest: &PluginManifest,
    plugin_id: &str,
    expected_revision: i64,
    grant: PluginGrant,
) -> Result<PluginCapabilitySnapshot, &'static str> {
    if plugin_id.trim().is_empty()
        || expected_revision < 0
        || manifest.plugin_id != plugin_id
        || manifest.revision != expected_revision
    {
        return Err("revision");
    }
    if manifest.status != PluginStatus::Enabled
        || manifest.approved_at.is_none()
        || manifest.approved_by.as_deref().is_none_or(str::is_empty)
    {
        return Err("approval");
    }
    if !manifest.requested_grants.contains(&grant) {
        return Err("grant");
    }
    Ok(PluginCapabilitySnapshot {
        plugin_id: manifest.plugin_id.clone(),
        revision: manifest.revision,
        grants: manifest.requested_grants.clone(),
        entrypoint: manifest.entrypoint.clone(),
    })
}

pub fn validate_process_identity(identity: &PluginProcessIdentity) -> Result<(), &'static str> {
    if identity.pid == 0 || identity.process_start_token.trim().is_empty() {
        return Err("process_identity");
    }
    Ok(())
}
