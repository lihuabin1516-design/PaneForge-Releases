# Runtime and capability boundaries

PaneForge runs several kinds of processes: the desktop shell, PTY hosts, CLI adapters, optional daemon/web components, and external tools such as Git. Each process has a distinct lifecycle and data boundary.

## Boundary table

| Boundary | Owns | Must not assume |
| --- | --- | --- |
| React UI | user intent, rendering, optimistic state | that a process started or completed |
| Tauri command | input validation and IPC authorization | that a filesystem path is safe without validation |
| Domain service | state transitions and invariants | that a child process is still alive after a launch response |
| Repository | durable records and migrations | that records represent live process state |
| PTY/CLI adapter | process, input, output, exit status | that output text is proof of acceptance |
| Verification runner | reproducible checks and evidence | that a green local check proves a Windows installer works |

## Capability policy

Operations with meaningful side effects are represented as capabilities. Examples include starting a process, reading a secret, opening a network connection, controlling a browser, and changing a repository. The default product posture is deny-by-default for undeclared capabilities; a host adapter must provide the explicit allowlist and revision expected by the caller.

This is a product boundary, not a promise that every third-party CLI is sandboxed. A CLI can still perform operations allowed by its own account and operating-system permissions. Users should review provider, shell, and repository permissions before starting a session.

## Windows-specific validation

Windows installer, WebView2, tray, global shortcut, screenshot, updater, and Windows PTY behavior are Windows-host concerns. Linux or WSL checks can validate source and most business logic but cannot replace the native acceptance workflow. The public workflows therefore upload the exact installer hash, source hash, process diagnostics, and smoke evidence separately.
