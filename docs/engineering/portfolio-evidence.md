# Implementation and acceptance evidence

This page gives reviewers a path from product claim to code and release evidence. The baseline is the shipped `0.0.3-alpha.3` build, not the current private development branch.

## Case study A: durable Blueprint task planning

### Problem

The previous task-tree UI was a projection of runtime task nodes. Users needed to save a plan before a CLI session existed, without inventing a second task state machine or making plan creation an execution command.

### Implementation

- The task model adds a planning brief and criteria while allowing an intentionally unassigned planned node.
- The service normalizes and bounds request data, validates UUIDs and parent/revision pairing, and rejects oversized input.
- The repository writes through an immediate SQLite transaction. Request UUID reuse with identical content is idempotent; reuse with different content is a conflict.
- Child creation rechecks that the parent still exists, still has the selected revision, and is not closed.
- Runtime projection writes cannot overwrite a manual Blueprint plan.
- The React form retains retry identity and selects the durable result. A saved plan remains after renderer reload and app restart.

### Browseable source

- [Task model](../../showcase/source/v0.0.3-alpha.3/paneforge-core/src/models/task_tree.rs)
- [SQLite repository](../../showcase/source/v0.0.3-alpha.3/paneforge-core/src/repository/task_tree_repository.rs)
- [Validation service](../../showcase/source/v0.0.3-alpha.3/paneforge-core/src/services/task_tree_service.rs)
- [Persistence and conflict tests](../../showcase/source/v0.0.3-alpha.3/paneforge-core/src/services/blueprint_task_creation_tests.rs)
- [React form](../../showcase/source/v0.0.3-alpha.3/web/components/controlplane/BlueprintTaskForm.tsx)
- [Frontend interaction tests](../../showcase/source/v0.0.3-alpha.3/web/components/controlplane/BlueprintCreation.test.tsx)

### Verification

- Focused frontend acceptance: 6 files, 36 tests passed.
- Core persistence coverage: 8 tests for migration, reopen, retry conflict, independent SQLite connections, parent validation, and rejected runtime overrides.
- Frozen source: commit `ecb76a4348e8139a48bacb4433a449f7a67ff6f1`; corresponding source ZIP SHA-256 `4feb8686e8165f3b7a1d3ae3cd2fd8ee3404420eeb14cce275f40005a42a1c85`.
- Windows source build [run 40452372948](https://github.com/lihuabin1516-design/PaneForge-Releases/actions/runs/40452372948): core suite 1772 passed / 0 failed / 6 ignored; desktop package and isolated installation passed. Its first UI smoke timed out before connection, so it is not used as UI acceptance evidence.
- Follow-up native Windows acceptance [run 40461026994](https://github.com/lihuabin1516-design/PaneForge-Releases/actions/runs/40461026994): root and child creation, refresh selection, renderer reload, and full application restart passed. The result recorded no page or cleanup errors.

## Case study B: default-deny plugin capability broker

### Problem

A plugin may request process, network, secret, filesystem, or UI capabilities. Treating an approved plugin manifest as blanket authority would turn approval into ambient host access.

### Implementation and deliberate limit

- Broker requests name the plugin, plan, expected manifest revision, channel, resource, and reason.
- The broker revalidates that the manifest is enabled, approved, at the expected revision, and includes the requested grant.
- Network destination, secret scope, and UI channel each require a separate host allowlist. Empty allowlists deny.
- The outcome is a typed authorization receipt. The alpha.3 production IPC does not construct the managed adapter; this is an authorization contract, not permission to perform the host operation.

Browse [capability model](../../showcase/source/v0.0.3-alpha.3/paneforge-core/src/models/plugin_host.rs), [broker adapter and tests](../../showcase/source/v0.0.3-alpha.3/paneforge-core/src/services/plugin_host_adapter.rs), and the [scoped acceptance record](../../showcase/source/v0.0.3-alpha.3/docs/acceptance/p15-capability-broker.md).

The acceptance record reports 14 focused host-adapter tests. It explicitly states that no arbitrary network, credential-store, DOM, or UI side effect is exposed by this slice. This claim must not be shortened to “plugins run in a sandbox.”

## Case study C: traceable Windows distribution

The public build workflow accepts an exact version, source commit, and source archive SHA-256. It downloads the matching source archive, verifies identity and lockfiles, runs source checks, builds an unsigned package in a read-only-public repository workflow, installs into a temporary program directory, and runs native smoke checks. Signing credentials are not available to the source-build job.

For alpha.3:

| Item | Value |
| --- | --- |
| Product version | `0.0.3-alpha.3` preview |
| Source commit | `ecb76a4348e8139a48bacb4433a449f7a67ff6f1` |
| Source ZIP SHA-256 | `4feb8686e8165f3b7a1d3ae3cd2fd8ee3404420eeb14cce275f40005a42a1c85` |
| Windows installer SHA-256 | `842b8c2a7714a18f24200185bf163079b6de908697f53d2b739233f52a464d6b` |
| Isolated native acceptance | [run 40461026994](https://github.com/lihuabin1516-design/PaneForge-Releases/actions/runs/40461026994) |
| Public asset set | installer, updater signature/feed, checksum file, corresponding GPL source ZIP |

The published source archive omits private Git history and user data. It remains the complete corresponding source for that binary; the selected files in this showcase do not replace it.

## Evidence limitations

- The UI screenshots are from an isolated GitHub-hosted Windows acceptance profile with generic fixture task names, not a user's workspace.
- The alpha.3 Windows run proves only the versioned source and listed smoke contract. It does not prove later private development features are shipped.
- The plugin broker result is a default-deny policy/receipt contract. It does not prove real plugin side effects are enabled or sandboxed.
- WSL/Linux tests do not substitute for Windows WebView2, installer, updater, tray, global shortcut, screenshot, or PTY acceptance.
