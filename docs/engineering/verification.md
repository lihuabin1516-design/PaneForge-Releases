# Verification and evidence

The public release is evaluated at three levels. Passing one level does not replace the others.

## Source level

- product identity and bundle namespace checks;
- dependency lockfile consistency;
- TypeScript, frontend tests, tooling tests, formatting, Rust check, Clippy, and Rust tests;
- core regression tests for the control-plane and native bootstrap paths.

## Package level

- Windows x64 package creation from the exact corresponding source archive;
- installer hash and source archive hash recorded in build provenance;
- isolated installation into a temporary program directory;
- update manifest and signature verification where applicable.

## Native behavior level

- ordinary-user launch rather than an elevated acceptance shell;
- Tauri target readiness before browser connection;
- blueprint task creation and persistence smoke;
- process, WebView2 environment, listening-port, and application-error diagnostics;
- redaction of bearer tokens, passwords, and secrets before artifact upload.

Evidence is a record of what was observed. It is not a guarantee that a third-party CLI, provider, repository, or user's operating-system policy will behave identically on another machine.
