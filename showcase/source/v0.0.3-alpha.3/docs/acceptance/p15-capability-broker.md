# P1.5 production capability broker acceptance

> Status: accepted as a default-deny broker contract; no plugin side effect is exposed

## Delivered

- Added typed broker channels for network destinations, secret scopes and UI
  contribution channels.
- Added revision-bound broker requests and immutable adapter receipts.
- Added a backend broker authorization seam that re-reads the enabled, approved
  plugin manifest before every decision.
- Network, secret and UI resources require explicit host allowlists in addition
  to the approved manifest grant. Empty policy lists deny by default.
- No inherited sockets, credential-store access, DOM injection or arbitrary
  network/secret/UI side effect exists in this slice.
- Production Tauri IPC remains default-deny and does not construct the managed
  adapter; authorization is transport-neutral until a separate execution review.

## Evidence

- Host adapter focused suite: 14 passed, including resource allowlist denial,
  broker receipt revision checks, process identity, filesystem escape and native
  Windows Job Object fixture coverage.
- Full frontend suite with bounded Vitest concurrency: 609/609 files and
  5808/5808 tests passed.
- TypeScript, Rust formatting, workspace check, Clippy and `git diff --check`
  are required gates for completion.

## Security boundary

P1.5 authorizes only a typed policy decision and receipt. It does not authorize
the host operation itself. A future P1.6 execution command must add separate
reviews for credential, network, filesystem and UI side effects, revocation,
user confirmation and native Windows non-interference.
