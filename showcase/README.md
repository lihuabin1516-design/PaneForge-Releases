# Browseable source showcase

This folder contains selected, unmodified source files from the already-published PaneForge `0.0.3-alpha.3` source revision. It exists so a reviewer can inspect representative code directly in the public GitHub repository instead of first downloading a 44 MB source archive.

## Exact source identity

- Source commit: `ecb76a4348e8139a48bacb4433a449f7a67ff6f1`
- Corresponding source ZIP: `PaneForge_0.0.3-alpha.3_source.zip`
- Source ZIP SHA-256: `4feb8686e8165f3b7a1d3ae3cd2fd8ee3404420eeb14cce275f40005a42a1c85`
- Release: [PaneForge 0.0.3-alpha.3](https://github.com/lihuabin1516-design/PaneForge-Releases/releases/tag/v0.0.3-alpha.3)

All files under [`source/v0.0.3-alpha.3`](source/v0.0.3-alpha.3) are copied verbatim from that tag. The application is GPL-3.0; see the repository [LICENSE](../LICENSE) and [source provenance](../docs/compliance/upstream-provenance.md).

## Read the Blueprint persistence slice

1. [Task wire/data model](source/v0.0.3-alpha.3/paneforge-core/src/models/task_tree.rs)
2. [SQLite schema and repository](source/v0.0.3-alpha.3/paneforge-core/src/repository/task_tree_repository.rs)
3. [Input normalization and validation](source/v0.0.3-alpha.3/paneforge-core/src/services/task_tree_service.rs)
4. [Persistence, retry, parent revision and migration tests](source/v0.0.3-alpha.3/paneforge-core/src/services/blueprint_task_creation_tests.rs)
5. [React task form](source/v0.0.3-alpha.3/web/components/controlplane/BlueprintTaskForm.tsx)
6. [Frontend interaction and retry tests](source/v0.0.3-alpha.3/web/components/controlplane/BlueprintCreation.test.tsx)

The code demonstrates explicit planned/unassigned state, bounded fields, same-request idempotency, conflicting UUID rejection, an immediate SQLite transaction, parent revision checks, and protection against a runtime projection overwriting a planning record.

## Read the capability-broker slice

- [Capability and receipt contracts](source/v0.0.3-alpha.3/paneforge-core/src/models/plugin_host.rs)
- [Broker policy, authorization path, and tests](source/v0.0.3-alpha.3/paneforge-core/src/services/plugin_host_adapter.rs)
- [Scoped acceptance record](source/v0.0.3-alpha.3/docs/acceptance/p15-capability-broker.md)

The implementation checks manifest revision and approval, requested grants, and explicit resource allowlists. Empty resource lists deny. The published alpha.3 boundary does not expose plugin host side effects; a policy receipt is not the operation itself.

## What this folder is not

- It is not a buildable subset or a second source of truth.
- It does not mirror private development changes or private Git history.
- It does not replace the exact corresponding source ZIP required for the published binary.
- It does not imply a private development feature is included in alpha.3.

Use the full source archive for builds, dependency inspection, complete license review, and reproducibility. Use this folder for fast code navigation. The diagrams and screenshots live under [docs](../docs/architecture/product-map.md) and [docs/media](../docs/media/README.md).
