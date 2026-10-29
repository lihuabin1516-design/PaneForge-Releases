# PaneForge

PaneForge is a local-first control plane for parallel AI engineering. It gives developers one desktop workspace for running CLI agents, terminals, repositories, tasks, tools, and verification evidence without turning the project directory into an opaque automation service.

This repository is the public distribution and engineering showcase for PaneForge. It contains release artifacts, reproducible source references, release verification workflows, and product documentation. The complete development repository remains private.

## What PaneForge provides

| Area | Public preview status | What it does |
| --- | --- | --- |
| Workspace and project control | Preview | Keeps projects, runtime choices, providers, layouts, and task context grouped by workspace. |
| Multi-CLI terminal | Preview | Runs several CLI sessions in split panes and tabs through PTY-backed terminals. |
| Agent launch context | Preview | Preserves the project path, runtime, provider, skill, MCP, and session metadata used for a launch. |
| Blueprint task board | Preview | Creates parent and child tasks with descriptions and acceptance criteria; planning is saved locally before a session is started. |
| Git and worktree tools | Preview | Exposes branch status and common repository operations next to the active terminal. |
| File history and editor | Preview | Provides a local history timeline, diffs, file browsing, and an in-app code editor. |
| MCP and Skill configuration | Preview | Keeps tool and skill selection visible at workspace/project boundaries. |
| Updates and provenance | Preview | Ships Windows x64 preview packages, updater signatures, checksums, corresponding source archives, and automated verification. |
| Blueprint execution policy | Development | Run profiles, frozen team templates, human handoff cards, and delivery evidence are being integrated in the private development line. |

The status labels are deliberate. A capability marked `Development` must not be inferred from the current public installer. See [public scope](docs/engineering/public-scope.md) for the exact boundary.

## Current download

The current public channel is the Windows x64 preview channel:

- [Latest PaneForge release](https://github.com/lihuabin1516-design/PaneForge-Releases/releases)
- [Windows x64 preview installer](https://github.com/lihuabin1516-design/PaneForge-Releases/releases/tag/v0.0.3-alpha.3)
- [Chinese documentation](README.zh-CN.md)

Install the preview package once. Later preview releases can be discovered from **Settings -> About -> Check for updates**. Save active sessions before installing an update; the installer may restart the desktop process.

PaneForge keeps release data under `%USERPROFILE%\.paneforge`. Development builds use `%USERPROFILE%\.paneforge-dev`, so a development build does not silently take over an installed release. The release channel currently targets Windows x64; native verification is performed on Windows runners.

## Architecture at a glance

```text
React UI
  -> Zustand stores and service wrappers
  -> Tauri IPC
  -> Rust commands
  -> domain services and repositories
  -> SQLite, filesystem, PTY, Git, and optional remote adapters
```

The important design boundary is that a CLI session is an observed runtime, not the database itself. PaneForge records launch intent, session identity, task binding, output evidence, and recovery state separately. This lets the UI restore a workspace without pretending that an interrupted process completed its task.

More detail:

- [Product capabilities and workflow](docs/product-capabilities.md)
- [System overview](docs/architecture/system-overview.md)
- [Control-plane lifecycle](docs/architecture/control-plane.md)
- [Runtime and capability boundaries](docs/architecture/runtime-boundaries.md)
- [Verification and evidence](docs/engineering/verification.md)

## Public and private repositories

PaneForge uses two repository roles:

1. The private product repository contains the complete source tree, development history, unreleased work, internal workflow files, and private acceptance material.
2. This public distribution repository contains only material intended for users and reviewers: release assets, corresponding-source references, public verification workflows, and product documentation.

No user workspace, provider secret, session transcript, private task ledger, signing key, or internal Git history belongs in this repository. The public scope is reviewed before each release.

## Build and verification

The release workflow can fetch the exact source archive attached to a release and verify its commit, SHA-256, lockfiles, product identity, frontend checks, Rust checks, package build, and isolated Windows smoke evidence. It is intentionally separate from the local development tree.

See [the release process](docs/engineering/release-process.md) and [the verification contract](docs/engineering/verification.md). A public source archive is named `PaneForge_<version>_source.zip` and is paired with `SHA256SUMS.txt` in the corresponding release.

## Legal and provenance

PaneForge is released under [GPL-3.0](LICENSE). The product name, runtime namespace, data directory, release process, documentation, control-plane additions, and acceptance material are maintained as PaneForge work. The distribution also contains modified GPL-covered foundation code; the required attribution and modification record is kept in [upstream provenance](docs/compliance/upstream-provenance.md), rather than copied into the product narrative.

Third-party notices and source obligations are summarized in [NOTICE.md](NOTICE.md) and [third-party notices](docs/compliance/third-party-notices.md).

## Feedback

Use [GitHub Issues](https://github.com/lihuabin1516-design/PaneForge-Releases/issues) for reproducible public-release problems. Do not attach API keys, private workspace archives, session transcripts, or signed build secrets. Security reports should follow [SECURITY.md](SECURITY.md).

---

## 中文简介

PaneForge 是一个面向并行 AI 工程的本地优先桌面控制平面：把多个 CLI Agent、PTY 终端、工作空间、项目、任务、Git、MCP、Skill 和验收证据放进同一套可恢复、可审查的工作流。应用元数据保存在本地；Agent CLI 和用户配置的 Provider 可能会按照各自策略传输提示词、项目内容或遥测数据，PaneForge 不承诺对话内容永不离开设备。

本仓库是公开分发仓库和求职展示入口，包含 Windows 安装包、对应源码索引、校验值、发布验证工作流和产品文档；完整开发源码、未发布功能和内部工作流仍保留在私有仓库。当前公开安装包是 Windows x64 抢先版，公开能力与开发线能力的边界见 [公开范围说明](docs/engineering/public-scope.md)、[产品能力与工作流](docs/product-capabilities.md) 和 [中文说明](README.zh-CN.md)。

PaneForge 的发布数据目录是 `%USERPROFILE%\.paneforge`，开发版使用 `%USERPROFILE%\.paneforge-dev`。安装包、对应源码 ZIP、SHA-256 校验和以及 GPL-3.0 合规信息会在每个版本的 GitHub Release 中一并提供。
