# PaneForge system overview

## Product boundary

PaneForge is a desktop control plane, not a hosted agent service. Workspace metadata and application settings are stored locally. CLI tools and configured providers may transmit prompts, project content, or telemetry under their own policies; PaneForge does not claim that agent conversations never leave the device.

```text
┌─────────────────────────────────────────────────────────────┐
│ React 19 UI                                                 │
│ panes · workspace · blueprint · Git · editor · settings      │
└──────────────┬──────────────────────────────────────────────┘
               │ typed service wrappers
┌──────────────▼──────────────────────────────────────────────┐
│ Tauri IPC and Rust command boundary                         │
│ validation · capability checks · lifecycle events            │
└──────────────┬──────────────────────────────────────────────┘
               │ domain services
┌──────────────▼──────────────────────────────────────────────┐
│ Workspace / task / session / provider / history services     │
└──────────────┬──────────────────────────────────────────────┘
               │ repositories and adapters
┌──────────────▼──────────────────────────────────────────────┐
│ SQLite · filesystem · PTY · Git · CLI adapters · WebSocket   │
└─────────────────────────────────────────────────────────────┘
```

## Core concepts

- **Workspace** is the isolation boundary for a group of projects, defaults, launch history, and journals.
- **Project** binds a repository path, local history, Git state, and project-level tool configuration.
- **Task** captures intent, acceptance criteria, ownership, and evidence. A task may exist before it has a running session.
- **Session** identifies a concrete CLI process and its PTY lifecycle. A session can stop, reconnect, or become orphaned without changing the task's historical evidence.
- **Capability** describes a privileged operation such as network access, secret access, process control, or browser control. Capability checks happen at the boundary that performs the operation.

## Persistence

Release data is stored below `%USERPROFILE%\.paneforge`; development data uses `%USERPROFILE%\.paneforge-dev`. Project-local files can contain `.paneforge` metadata such as history, specs, plans, and workflow notes. The public distribution repository never contains those user directories.

## Why the separation matters

The UI may be restarted while a CLI continues, a CLI may exit before a task is complete, and a workspace may be opened from another process. Separating intent, runtime observations, and evidence makes those cases explicit and makes recovery reviewable instead of silently rewriting history.
