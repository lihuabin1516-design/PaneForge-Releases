# PaneForge

## 让 AI 辅助开发过程可见、可恢复、可审查的 Windows 工作台

PaneForge 把代码仓库、多个本地编程 CLI、任务计划、Git 改动和交付证据放进同一个工作空间。它解决的是一个实际问题：当开发工作分散在多个终端和项目目录时，开发者很难回答“哪个项目被改了、原本要完成什么、真实运行了什么、现在还缺哪一步检查”。

这里的 **AI 编程 CLI** 指从终端启动的编程助手。PaneForge 负责组织这些工具及其项目上下文，不提供模型账号，也不替代用户选择的模型服务商（Provider）。

[![最新版本](https://img.shields.io/github/v/release/lihuabin1516-design/PaneForge-Releases?display_name=tag&label=release)](https://github.com/lihuabin1516-design/PaneForge-Releases/releases/latest)
[![Windows x64](https://img.shields.io/badge/platform-Windows%20x64-2563eb)](#下载与首次运行)
[![GPL-3.0](https://img.shields.io/badge/license-GPL--3.0-2ea44f)](LICENSE)
[![源码构建](https://github.com/lihuabin1516-design/PaneForge-Releases/actions/workflows/windows-source-build.yml/badge.svg)](https://github.com/lihuabin1516-design/PaneForge-Releases/actions/workflows/windows-source-build.yml)

**当前版本：** `0.0.3-alpha.3` 抢先版 · **原生验收：** Windows x64 · **发布时间：** 2026-10-27

[下载 Windows 安装包](https://github.com/lihuabin1516-design/PaneForge-Releases/releases/download/v0.0.3-alpha.3/PaneForge_0.0.3-alpha.3_x64-setup.exe) · [查看对应版本发布页](https://github.com/lihuabin1516-design/PaneForge-Releases/releases/tag/v0.0.3-alpha.3) · [查看发布证据](docs/engineering/portfolio-evidence.md) · [校验 SHA-256](https://github.com/lihuabin1516-design/PaneForge-Releases/releases/download/v0.0.3-alpha.3/SHA256SUMS.txt)

![PaneForge 工作台首页：项目、终端与任务概览](docs/media/paneforge-overview.png)

该截图来自本地构建的 PaneForge 0.0.3-alpha.3 隔离演示实例，不是对应 Release 的验收证据。界面中的项目和任务均为合成演示数据；终端是本机 PowerShell，状态为“等待输入”，不代表 Agent 已执行工作。

## 60 秒看懂 PaneForge

| 读者 | 他们面对的问题 | PaneForge 提供什么 | 可核验材料 |
| --- | --- | --- | --- |
| HR / 产品读者 | AI 编程工作散落在终端和项目目录里。 | 一个集中管理项目、会话、计划和交付检查的 Windows 工作台。 | [产品能力地图](docs/product-capabilities.md) |
| 开发者 | 多个仓库和 CLI 会话容易串台。 | 工作空间/项目上下文、标签页、分屏终端、启动历史、Git 与文件工具。 | [已发布能力列表](docs/product-capabilities.md#desktop-workbench-included-in-alpha3) |
| 代码审查者 | 看起来成功的聊天记录不能证明代码正确。 | 明确的任务标准、仓库改动、产物、检查结果和审查边界。 | [验证契约](docs/engineering/verification.md) |
| 技术面试官 | 产品宣传很难对应到真实实现。 | 可浏览源码切片、架构图、测试记录、发布哈希和 Windows 原生证据。 | [工程案例与验收证据](docs/engineering/portfolio-evidence.md) |

### 这个版本到底证明了什么？

- **已在 Windows 原生验收：** 蓝图主/子任务创建、验收标准、渲染器刷新和完整应用重启后的持久化、幂等重试与冲突处理、父节点 revision 检查，以及 Windows 发布路径。
- **已包含在发布源码和安装包能力范围：** 多会话 PTY 终端、工作空间和项目、已安装 CLI 适配器、按配置启用的本机/WSL/SSH 路径、Git 与文件审查工具、MCP/Skill 配置界面、本地历史、会话历史和控制平面记录。
- **仍在开发中：** 启动预览与失效配置提示、冻结角色/团队模板、定向人工收件箱、汇总交付视图。这些内容不会被写成已发布功能。

公开页坚持区分三类证据：设计目标、发布源码中的能力、Windows 原生验收结果，不能互相替代。

## 为什么要做这个产品

当每个 Agent 都只是另一个终端窗口时，AI 辅助开发很难被监督。PaneForge 针对五个高频时刻提供结构化入口：

1. **上下文容易丢失：** 工作空间把仓库、项目默认值、布局、启动历史和会话身份组织在一起。
2. **意图容易埋在聊天里：** 蓝图允许在 CLI 会话存在之前保存任务说明和验收标准；未绑定执行对象的任务会保持“已规划、未分配”。
3. **终端输出不能代替代码审查：** Git 状态、Diff、Worktree、文件、编辑器、本地快照可以和终端并排检查。
4. **重启结果应该可见：** 工作空间、任务、进程/会话和运行时观察分别记录；重启不会被悄悄解释成“任务完成”。
5. **授权不等于执行：** 任务、干预、恢复、委派和插件记录带有明确的作用域和证据标签，审批记录不会冒充操作系统副作用。

完整能力和边界见[产品能力与工作流](docs/product-capabilities.md)。

## 五分钟产品导览

### 1. 组织项目

创建或导入工作空间，添加仓库，选择启动所使用的项目上下文。发布版和开发版使用隔离的数据目录，试用开发构建不会悄悄复用发布数据。

### 2. 打开工作面板

在标签页或可调整分屏中打开已安装的编程 CLI。终端是真实的 PTY 执行面，由 xterm.js 渲染；它不是“成功检测器”。

### 3. 先写计划

进入 **Agent 控制台 → Blueprint → 新建任务**，填写说明和逐条验收标准，保存主任务后添加子任务。公开 alpha.3 的流程到这里为止：任务会持久化，但不会自动派发 Worker。

![蓝图创建与重启后持久化](docs/media/blueprint-alpha3-acceptance.gif)

### 4. 独立检查改动

CLI 结束后查看 Git 状态、Diff、变更文件、本地历史和产物。进程退出或日志看起来成功，仍然需要独立验收。

### 5. 带着证据交付

把任务标准、仓库改动、测试输出和发布元数据放在一起，决定接受、返工或交接。当前公开版已经把审查边界呈现出来；集中式交付视图仍在开发中。

## 产品界面与真实素材

公开视觉材料区分本地隔离演示界面、版本原生验收证据与历史功能说明。

### 工作空间、终端与 Git 变更

![工作空间项目树、终端标签和 Git 变更检查](docs/media/paneforge-workbench-tour.gif)

两帧动图依次展示工作空间项目树、本机 PowerShell 终端中的 Git 状态和差异。演示仓库的 `src/hello.ts` 变更在采集前预置；动图中只执行了针对该文件的只读状态与差异检查。它展示的是 PaneForge 终端和代码审查入口，不表示 Agent 创建了这项改动，也不表示验收通过。

### TodoList 任务管理

![TodoList 从任务概览切换到任务详情](docs/media/paneforge-tasks-tour.gif)

动图展示合成演示任务从清单进入详情，包含状态、优先级、工作空间、描述和活动记录。任务标题与内容不来自真实项目或用户数据。

### 运行配置与应用外观

![运行配置和应用主题设置](docs/media/paneforge-settings-tour.gif)

动图展示运行配置摘要与主题选择页面。配置界面中的 CLI、MCP 和 Skill 信息仅证明相应配置入口存在，不证明本次演示调用了外部工具、模型服务或远程环境；画面不包含凭据值。

### 原生蓝图证据

![PaneForge 工作空间和蓝图入口](docs/media/blueprint-alpha3-created.png)

原生截图展示工作空间入口、项目/终端入口、Agent 入口和已持久化的蓝图树。[重启后的截图](docs/media/blueprint-alpha3-reopened.png)是另一张验收帧。来源、哈希和 fixture 边界记录在[媒体说明](docs/media/README.md)中。

### 控制平面证据

![控制平面记录导览](docs/media/control-plane-dev-tour.gif)

这组三帧动态图由历史 Windows 只读面板截图组成，行内容使用合成 fixture。它用于解释任务、干预/恢复和全局治理的作用域，不是实时运行轨迹，也不冒充当前安装包截图。可分别查看[任务树](docs/media/control-plane-task-tree-dev.png)、[历史记录](docs/media/control-plane-history-dev.png)和[治理记录](docs/media/control-plane-governance-dev.png)。

当前公开材料仍缺少真实 Agent/CLI 对话、代码编辑器、会话恢复和远程运行环境的脱敏原生截图。本轮素材补充的是本机终端、任务清单及配置界面，不将它们描述成 Agent 执行证据。

## 复现已验收流程

```mermaid
flowchart LR
  A[安装 Windows x64 抢先版] --> B[创建工作空间和项目]
  B --> C[创建蓝图主任务和子任务]
  C --> D[刷新并重启 PaneForge]
  D --> E[检查父子关系、标准、状态和选中项]
  E --> F[交接前独立检查 Git 证据]
```

alpha.3 的预期结果是：渲染器刷新和完整应用重启后，父子关系、选中任务、说明、验收标准、`planned` 状态和未分配会话仍然存在。创建任务不表示 CLI 已经运行，也不表示验收已经通过。

## 给技术面试官的工程入口

以下内容按“工程问题 → 设计选择 → 代码入口 → 验证边界”组织，避免只罗列技术名词。

| 工程问题 | 设计选择 | 代码/证据入口 | 已验证内容与限制 |
| --- | --- | --- | --- |
| 重试不能创建两份计划 | React 保留 request UUID；服务层限制输入；Repository 使用 SQLite immediate transaction。相同内容幂等，不同内容冲突。 | [任务模型](showcase/source/v0.0.3-alpha.3/paneforge-core/src/models/task_tree.rs)、[Repository](showcase/source/v0.0.3-alpha.3/paneforge-core/src/repository/task_tree_repository.rs)、[测试](showcase/source/v0.0.3-alpha.3/paneforge-core/src/services/blueprint_task_creation_tests.rs) | 36 个前端聚焦测试、8 个核心持久化测试；原生重启路径通过。 |
| 父子写入可能竞态 | 事务内重新检查父节点 revision 和开放状态；运行时投影不能覆盖手工计划。 | [创建时序](docs/architecture/product-map.md#2-blueprint-task-creation-sequence) | 持久化聚焦测试覆盖；alpha.3 不包含派发。 |
| UI 状态不能冒充运行时事实 | React/Zustand 是投影；类型化服务跨 Tauri IPC；Rust 服务和 Repository 管持久记录；PTY 观察管进程事实。 | [架构源文件](docs/assets/paneforge-architecture.mmd)、[系统概览](docs/architecture/system-overview.md) | 边界有源码依据；可选 CLI/Provider 是否可用仍取决于主机。 |
| 审批不能变成插件的无限权限 | 请求携带 manifest revision、授权、资源、通道和原因；空 allowlist 默认拒绝；返回类型化授权收据。 | [能力模型](showcase/source/v0.0.3-alpha.3/paneforge-core/src/models/plugin_host.rs)、[Broker](showcase/source/v0.0.3-alpha.3/paneforge-core/src/services/plugin_host_adapter.rs) | 14 个宿主适配器聚焦测试；这是策略契约，不等于操作系统沙箱。 |
| 二进制必须能追溯到可检查源码 | 发布元数据绑定版本、源码提交、源码 ZIP 哈希、安装包哈希、锁文件/工具链、更新签名和原生验收运行。 | [发布案例](docs/engineering/portfolio-evidence.md#case-study-d-traceable-windows-distribution) | `0.0.3-alpha.3` 源码构建与 Windows 验收已记录；源码浏览目录不是完整源码包。 |

### 面试时可以这样阅读

1. 从 [BlueprintTaskForm.tsx](showcase/source/v0.0.3-alpha.3/web/components/controlplane/BlueprintTaskForm.tsx) 开始，沿着服务层和 Repository 跟踪一次请求。
2. 讨论为什么“已规划任务”必须保持未分配，再对照原生验收动态图。
3. 查看 Broker 收据，说明为什么 allowlist 不等于沙箱。
4. 打开架构源文件，沿着“前端 → 类型化服务 → Tauri IPC → Rust 服务 → Repository”追踪边界。
5. 最后核对发布证据表里的安装包和源码哈希。

## 架构图

![PaneForge 架构：界面、Tauri IPC、Rust 服务、Repository、本地持久化、PTY 适配器和外部工具](docs/assets/paneforge-architecture.svg)

可编辑源文件是 [paneforge-architecture.mmd](docs/assets/paneforge-architecture.mmd)。图中只表达已核实的调用和权责边界，不是运行录制，也不代表每台机器都安装了所有可选 CLI 或 Provider。

## 我的个人贡献

公开材料可以直接支持以下 PaneForge 产品工作：

| 工作范围 | 产品中可见的贡献 | 证据边界 |
| --- | --- | --- |
| 产品化 | 产品身份、Windows Bundle 身份、`%USERPROFILE%\.paneforge` 数据路由、更新通道、双语发布页以及公开/私有分发拆分。 | 发布工作流和身份检查公开可核验；逐行作者账本不放入这个发布仓库。 |
| 规划与审查 | 蓝图主/子任务、验收标准、输入限制、幂等重试、父节点 revision 检查、持久化和重启后选中状态。 | 可浏览源码和 Windows 原生验收运行。 |
| 控制平面 | 任务、干预/恢复、委派和插件记录按作用域保存，并明确证据边界。 | 历史 UI fixture 和作用域投影案例。 |
| 能力策略 | revision 绑定授权、资源 allowlist、默认拒绝和类型化收据。 | 公开源码切片和验收记录；alpha.3 不开放任意插件副作用。 |
| 发布工程 | 对应源码打包、SHA-256 映射、Windows 源码构建工作流、隔离安装和原生 smoke 证据。 | 发布资产和 GitHub Actions 记录。 |

这里的贡献声明只覆盖公开材料能够追溯的产品工作。发行包仍保留 GPL 版权和许可证声明，不把每个依赖的每一行都写成个人从零实现。完整说明见[修改记录](docs/compliance/source-attribution.md)、[NOTICE](NOTICE.md)和[LICENSE](LICENSE)。

## 下载与首次运行

### 下载对应版本资产

- [Windows x64 安装包](https://github.com/lihuabin1516-design/PaneForge-Releases/releases/download/v0.0.3-alpha.3/PaneForge_0.0.3-alpha.3_x64-setup.exe)
- [对应版本发布页](https://github.com/lihuabin1516-design/PaneForge-Releases/releases/tag/v0.0.3-alpha.3)
- [更新清单](https://github.com/lihuabin1516-design/PaneForge-Releases/releases/download/v0.0.3-alpha.3/latest.json)
- [安装包签名](https://github.com/lihuabin1516-design/PaneForge-Releases/releases/download/v0.0.3-alpha.3/PaneForge_0.0.3-alpha.3_x64-setup.exe.sig)
- [SHA-256 校验文件](https://github.com/lihuabin1516-design/PaneForge-Releases/releases/download/v0.0.3-alpha.3/SHA256SUMS.txt)
- [对应 GPL 源码 ZIP](https://github.com/lihuabin1516-design/PaneForge-Releases/releases/download/v0.0.3-alpha.3/PaneForge_0.0.3-alpha.3_source.zip)

| 资产 | SHA-256 |
| --- | --- |
| 安装包 | `842b8c2a7714a18f24200185bf163079b6de908697f53d2b739233f52a464d6b` |
| 源码 ZIP | `4feb8686e8165f3b7a1d3ae3cd2fd8ee3404420eeb14cce275f40005a42a1c85` |

```powershell
Get-FileHash .\PaneForge_0.0.3-alpha.3_x64-setup.exe -Algorithm SHA256
Get-FileHash .\PaneForge_0.0.3-alpha.3_source.zip -Algorithm SHA256
```

### 复现最小场景

1. 安装 Windows x64 包。安装包没有 Authenticode 签名时，Windows 可能显示安全提示；更新签名是另一层发布完整性机制。
2. 启动 PaneForge，点击 **Enter Workspace** 或 **Workspace Manager**。
3. 打开 **Agent 控制台 → Blueprint → 新建任务**。
4. 添加子任务，刷新渲染器并重启应用。
5. 确认父子关系、选中详情、`planned` 状态和未分配会话仍然存在。

发布版数据目录为 `%USERPROFILE%\.paneforge`，开发版使用 `%USERPROFILE%\.paneforge-dev`。更新前请保存活跃会话，因为安装可能重启桌面进程。当前公开验证范围是 Windows x64，其他平台及特定 CLI 行为不属于本版承诺。

## 版本、测试与状态

### `0.0.3-alpha.3` 已完成并验证

- 蓝图主任务/子任务创建，支持说明和逐条验收标准。
- 渲染器刷新和完整应用重启后的持久化。
- 任务创建的幂等重试与冲突处理。
- 父节点 revision/开放状态检查，以及防止运行时投影覆盖计划。
- 默认拒绝能力 Broker 契约和作用域清晰的控制平面投影。
- Windows x64 源码构建、安装包、隔离安装和蓝图原生 smoke。

证据包括[36 个前端聚焦测试和 8 个核心持久化测试](docs/engineering/portfolio-evidence.md#verification)、源码构建运行中的 1772 个核心测试，以及后续 Windows 原生验收运行。控制平面的历史测试数量已明确标为历史记录，本次文档修改没有重新运行它们。

### 开发中或待补充

- 启动预览和失效配置提示。
- 冻结角色/团队模板。
- 定向人工交接收件箱。
- 汇总 Git、产物和真实验收证据的交付视图。
- 活跃 CLI、Git Diff、编辑器、会话恢复和远程运行环境的脱敏原生截图。
- 精确到提交和作者的个人贡献账本。

这些内容只有进入有版本号的安装包，并有对应源码和 Windows 证据后，才会升级为公开版能力。

## 源码、许可证与公开边界

PaneForge 使用 GPL-3.0 发布。每个版本都提供对应源码、保留的版权/许可证声明、修改信息和 `SHA256SUMS.txt`。`showcase/` 目录从对应版本源码快照中复制精选文件，方便面试官直接开始阅读；它不是第二套构建树。完整对应源码压缩包才是该二进制的法律和技术记录。

本公开仓库只放发布资产、验证工作流、源码导航和面向审阅者的文档。用户工作空间、凭据、会话全文、签名密钥、私有历史和未审查诊断信息不会进入公开分发。

## 延伸阅读

- [面试阅读简报](docs/interview-brief.zh-CN.md)
- [产品能力与工作流](docs/product-capabilities.md)
- [架构图与任务生命周期](docs/architecture/product-map.md)
- [运行时与能力边界](docs/architecture/runtime-boundaries.md)
- [实现案例与验收证据](docs/engineering/portfolio-evidence.md)
- [验证契约](docs/engineering/verification.md)
- [公开发布流程](docs/engineering/release-process.md)
- [源码浏览目录](showcase/README.md)
- [English README](README.md)

## 反馈

可在 [GitHub Issues](https://github.com/lihuabin1516-design/PaneForge-Releases/issues) 提交可复现的公开版问题。不要上传 API 密钥、私有工作空间、会话全文或签名材料。安全问题请阅读 [SECURITY.md](SECURITY.md)。
