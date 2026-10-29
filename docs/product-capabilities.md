# PaneForge product capabilities and workflow

PaneForge is a desktop workbench for developers who use AI coding CLIs as part of everyday software development. Starting from zero: add repositories to a workspace, launch locally installed coding CLIs in terminals, work in parallel, then inspect Git changes, files, session history, and task evidence in the same application. PaneForge does not provide a model account and does not treat a chat transcript as proof that code is correct.

The public installer is a preview. This page separates capabilities present in `0.0.3-alpha.3` from work still in the private development line.

![Two native acceptance snapshots of the Blueprint task tree](media/blueprint-alpha3-acceptance.gif)

*The animation alternates between the task tree after creation and a separate snapshot after app restart. It is assembled from acceptance screenshots, not a continuous recording. See [media provenance](media/README.md).*

## Product model

```text
Workspace
  └─ Project (repository and tool configuration)
      ├─ Task (intent and acceptance criteria)
      └─ Session (a concrete CLI process and PTY)
          └─ Evidence (Git changes, artifacts, checks, human review)
```

This is the product's conceptual work model, not a claim that every relationship is automated in the current release. In `0.0.3-alpha.3`, Blueprint tasks are saved as planned and unassigned; they are not yet automatically connected to a launched session or delivery decision.

The hierarchy gives operators a place to answer four questions without treating chat output as the source of truth:

1. Which workspace and repository does this work belong to?
2. What outcome and acceptance criteria were agreed for it?
3. Which process/session actually ran, and what happened to it?
4. What changed, and what evidence supports accepting the result?

## Desktop workbench included in alpha.3

PaneForge is more than the Blueprint and Control Plane screens. The public `0.0.3-alpha.3` installer also contains the broader desktop workspace used to run and review day-to-day development:

For readers new to AI development tooling: a CLI is a coding tool launched from a command line; a PTY is the operating-system terminal session it runs in; MCP (Model Context Protocol) connects an agent to external tools; and Skills are reusable instruction/workflow packages. PaneForge hosts these tools but does not supply their model accounts.

| Work area | Capability in the public preview | Origin and boundary |
| --- | --- | --- |
| Organize work | Use the workspace command center to see active sessions, recent projects, available CLIs, and usage context; organize repositories, project settings, layouts, and project order. | Modified desktop-workspace foundation. |
| Run terminals | Use PTY-backed terminals rendered by xterm.js, with tabs, resizable splits, saved layouts, scrollback, and recovery support. | Modified terminal foundation; recovery depends on runtime behavior. |
| Launch coding CLIs | Use adapters for Claude Code, Codex CLI, Gemini CLI, Kimi, Grok, OpenCode, and Cursor. The released adapter registry also contains Pi, OMP, and JCode; other commands can run as terminal processes. | Requires each CLI to be installed/configured; hooks, MCP, and resume support differ by CLI. |
| Select an environment | Launch against local, WSL, or SSH project routes where configured. | Host, credentials, network, and each CLI's runtime support affect availability. |
| Coordinate AI work | Use Agent Chat, bounded MCP leader/worker orchestration, Plans, Specs, Todos, Journals, session summaries, and Memory around active CLI sessions. | Included workflow foundation; it does not make model output self-verifying. |
| Configure tools | Reuse provider/model launch profiles and inspect MCP servers, shared/project Skills, hooks, and permission options at workspace/project/launch boundaries. | Provider accounts and third-party tool permissions remain user-controlled. |
| Review repositories | Inspect branch state, common pull/push/fetch/stash operations, Git history, diffs, worktrees, clone results, and merge conflicts. | Git remains the authoritative version-control history. |
| Work with files | Browse/search/create/rename/copy/move/delete project files; edit with Monaco; preview Markdown/images; compare labeled local snapshots and diffs. | Local file history is a review aid, not a replacement for Git backups. |
| Restore context | Review launch history, clean up broken-session records, and resume sessions supported by the selected CLI; capture multi-monitor screenshots with a configurable shortcut. | Process recovery is not task completion; exact resume behavior is CLI-specific. |
| Plan with PaneForge | Create persisted Blueprint roots/subtasks with descriptions and acceptance criteria; inspect Control Plane task, intervention, recovery, delegation, and plugin records. | PaneForge product layer; alpha.3 task creation does not dispatch a CLI. |
| Use desktop controls | Open the command palette, tray and quick-terminal controls, resource monitoring, themes, wallpaper, and terminal/background appearance settings. | Desktop utility features; host-specific behaviors require Windows verification. |
| Install and verify | Use the signed Windows x64 update channel and download the matching GPL source archive and checksums. | PaneForge release infrastructure; current public channel is Windows x64 preview. |

The capability list describes what is present in the PaneForge release source and installer. Third-party CLI behavior still varies by installed tool, provider, account, host, and runtime. The `0.0.3-alpha.3` public artifact and native acceptance scope are Windows x64 desktop; browser, mobile, and other host clients are outside this release promise. The exact source, license, and modification records are kept in the corresponding-source package and repository compliance files.

## Capabilities in the public preview

### Multi-session terminal workspace

PaneForge arranges terminal sessions in tabs and resizable split panes. Each terminal is backed by a PTY and rendered with xterm.js. Developers can keep several CLI sessions visible while moving between repositories and workspace layouts.

The terminal is an execution surface, not a success detector. An exited process, a `done` message, or a green-looking transcript is not by itself acceptance evidence.

### Workspace and project context

Workspaces group projects and their defaults. Projects provide the repository path and project-specific configuration used by launches. The application keeps its release and development profiles separate, so trying a development build does not silently reuse the release data directory.

### CLI launch configuration

PaneForge integrates with installed CLI tools through local process and terminal adapters. The released source includes adapters for Claude Code, Codex CLI, Gemini CLI, Kimi, Grok, OpenCode, and Cursor, while the PTY can also run other terminal commands. Launch context can include the selected project, local/WSL/SSH runtime, provider, Skills, MCP configuration, hooks, permission options, and session metadata. Availability and feature depth depend on which CLI tools and providers the user has installed and configured.

PaneForge does not provide the model or provider account. Prompt content, repository material, and telemetry may be sent by the selected CLI/provider according to its own settings and policies.

### Blueprint planning

The `0.0.3-alpha.3` preview supports creating a root task or subtask with a title, description, and individual acceptance criteria. Saved tasks persist in the local database and retain their parent-child relationship. A newly created task is selected so the developer can continue planning.

Task creation does not launch a CLI, assign a worker, or claim that the acceptance criteria passed. An unassigned task remains visibly unassigned.

### Repository and file work

The desktop combines common Git and worktree operations with a project file browser, code editor, local file history, and diff inspection. This makes the repository state easier to review alongside the relevant terminal session. These tools do not replace Git as the authoritative version-control history.

### Tooling and workflow context

MCP servers and Skills can be configured for the applicable workspace or project. PaneForge presents this configuration near the launch context so operators can inspect which tools are available. A configured third-party MCP server or CLI still has the permissions granted by its own runtime and account.

### Agent Control Plane records

The alpha.3 preview includes task-tree, scoped-history and governance-record surfaces:

#### Task tree and runtime identity

The task tree separates node, parent, session, attempt, business state, revision and runtime-health fields. Missing health is shown as not reported rather than inferred as healthy. Creating a Blueprint task does not start a session or imply that a CLI executed it.

#### Intervention and recovery history

Intervention history is scoped to a task node; runtime-recovery history is scoped to a session. The two ledgers can refresh or fail independently, and stale data is labelled instead of silently replaced with an empty state. Older attempts and recoveries for another node in the same session retain their provenance labels.

#### Global governance records

Delegation and plugin registries are labelled as global records. Their presence or approval status is not evidence that an Agent or plugin process executed. Governance screens expose review/audit context, not operating-system process isolation or permission to execute arbitrary plugin side effects.

The [Control Plane evidence](../README.md#control-plane-evidence) section explains three historical Windows acceptance captures, including the visible fields and the evidence boundary for each view. The companion P1.2c UI acceptance snapshot records the tested app as `PaneForge DEV v0.0.1`; all three PNGs are cropped without version labels. All three use synthetic fixtures and none is a screenshot of the current alpha.3 installer. The current release's Blueprint persistence acceptance is shown at the top of this page.

## Blueprint workflow in development

The private development line is extending task planning toward an auditable execution and delivery loop:

- **Launch preview:** select project, working directory, runtime, and launch profile; report stale or invalid configuration before dispatch.
- **Frozen role/team templates:** reuse a team setup while snapshotting the resolved choices on each task, so editing a template later does not rewrite old work.
- **Human collaboration inbox:** direct a question or handoff to a named person while preserving the default isolation between sessions.
- **Delivery view:** bring task requirements, Git changes, artifacts, and actual acceptance evidence together for review.
- **Governed execution:** keep process, network, secret, and other side-effecting capabilities behind explicit policy and host boundaries.

These are marked `Development`, not public preview. They become public capabilities only when included in a versioned installer and covered by the release's source and native acceptance evidence.

## Typical public-preview workflow

1. Install PaneForge, create or import a workspace, and add one or more repositories.
2. Confirm the project path and choose an available local, WSL, or SSH route and locally installed CLI.
3. Open one or more CLI sessions in tabs or split panes; use the configured MCP servers, Skills, and launch profile where appropriate.
4. Use Agent Chat, plans, Todos, orchestration, or memory as needed by the project workflow.
5. Create a Blueprint task with observable acceptance criteria when task planning is useful; in alpha.3 it remains a saved, unassigned plan.
6. Review Git status/diffs, worktrees, file history, and edited files independently of the terminal transcript.
7. Save the result and evidence through the project's delivery process; a process ending does not itself mean acceptance passed.

The alpha.3 native acceptance run covers Blueprint task creation and persistence across reload and restart. Full task dispatch, frozen templates, collaboration inbox routing, and consolidated delivery review are still being completed in the development line.

## Release lineage and evidence

The public product release described here is `0.0.3-alpha.3`, whose corresponding GPL source is published with the [versioned release](https://github.com/lihuabin1516-design/PaneForge-Releases/releases/tag/v0.0.3-alpha.3). The release archive, source hash, checksums, and Windows acceptance record identify this exact PaneForge build; they are the authority for what this version contains.

## Design principles

- **Local-first state:** workspace and application metadata are stored on the user's machine; external CLIs and providers retain their own data handling behavior.
- **Visible boundaries:** workspace, project, task, process, and provider are separate concepts rather than hidden defaults.
- **Recoverable lifecycle:** process restart and task completion are different events.
- **Evidence before claims:** a task's acceptance state should be grounded in inspected changes and reproducible checks.
- **Least surprise:** creating a plan does not execute it, a stopped CLI does not imply success, and development and release profiles stay isolated.
- **Release traceability:** each public installer is tied to a source archive, hashes, workflow inputs, and Windows verification.

## Current limitations

- The public update channel currently targets Windows x64 preview packages.
- The published artifact and native acceptance scope is the Windows x64 desktop app. Browser, Android, macOS, and Linux packages are outside the current public-release promise.
- PaneForge does not include model access or guarantee a specific CLI/provider's uptime or behavior.
- Process isolation and capability governance do not sandbox arbitrary third-party CLI behavior beyond the operating-system and tool permissions in effect.
- Blueprint execution and delivery features described as `Development` are not part of the alpha.3 product promise.
- Windows desktop behavior must be verified on Windows; source checks in WSL or Linux alone are insufficient.

---

## 中文说明

PaneForge 面向需要监督多个 AI 编程会话的开发者。它的目标是让并行工作可见、可恢复：CLI 正在什么项目目录工作、关联什么任务、启动了哪个运行时、产生了哪些改动，以及有什么证据支持交付。

当前公开安装包为 `0.0.3-alpha.3` 抢先版。以下明确区分已公开能力和仍在私有开发线推进的能力。

### 产品对象

```text
工作空间
  └─ 项目（仓库与工具配置）
      ├─ 任务（目标与验收标准）
      └─ 会话（真实 CLI 进程与 PTY）
          └─ 证据（Git 改动、产物、检查结果、人工审查）
```

这个分层用于回答四个问题：工作属于哪个仓库和工作空间；预期结果与验收标准是什么；真实启动过哪个进程、生命周期如何；产生了什么改动、有哪些证据支持验收。终端输出本身不是权威状态源。

这是产品概念模型，不代表当前版本已自动接通所有关系。公开版 `0.0.3-alpha.3` 中，新建蓝图任务保存为已规划且未分配状态，尚不会自动绑定到会话或交付决定。

### alpha.3 已包含的完整桌面工作台

PaneForge 不只有蓝图和控制平面页面。当前公开安装包还包含用于日常开发的桌面工作区基础能力：

术语说明：CLI 是从命令行启动的编程工具；PTY 是操作系统提供的终端会话；MCP（Model Context Protocol）用于让 Agent 连接外部工具；Skill 是可复用的指令/工作流包。PaneForge 管理这些工具的工作环境，但不提供模型账号。

| 工作区域 | 当前公开版能力 | 来源与边界 |
| --- | --- | --- |
| 组织开发工作 | 通过工作台概览查看活跃会话、近期项目、可用 CLI 和使用情况，并组织仓库、项目设置、布局及项目顺序。 | 修改后的桌面工作空间基础。 |
| 运行终端 | 通过 PTY 运行真实终端、用 xterm.js 显示，支持标签、可调分屏、保存布局、滚屏及恢复。 | 修改后的终端基础；恢复能力还取决于运行环境。 |
| 启动编程 CLI | 适配 Claude Code、Codex CLI、Gemini CLI、Kimi、Grok、OpenCode、Cursor；发布源码的适配器注册表还包含 Pi、OMP、JCode，其他命令也可作为终端进程运行。 | 需用户安装/配置 CLI；Hooks、MCP、会话恢复等能力因 CLI 而异。 |
| 选择运行环境 | 在配置可用时使用本机、WSL 或 SSH 项目路径。 | 是否可用取决于主机、凭据、网络和 CLI 自身支持。 |
| 协调 Agent 工作 | 在终端会话旁使用 Agent Chat、MCP 主控/Worker 编排、Plans、Specs、Todos、Journal、会话摘要和 Memory。 | 工作流基础功能，不代表模型输出会自动验证。 |
| 配置工具 | 复用 Provider/模型启动档，并按适用范围查看 MCP Server、共享/项目 Skill、Hooks 和权限选项。 | Provider 账号及第三方工具自身权限仍由用户控制。 |
| 检查仓库 | 查看分支，执行常见 pull/push/fetch/stash，检查 Git 历史、Diff、Worktree 和克隆结果并处理冲突。 | Git 仍然是版本历史的权威来源。 |
| 浏览和编辑文件 | 浏览/搜索/创建/重命名/复制/移动/删除项目文件，使用 Monaco 编辑器、Markdown/图片预览，并比较带标签的本地快照与 Diff。 | 本地文件历史是审查辅助，不替代 Git 备份。 |
| 延续工作上下文 | 查看启动历史、清理失效会话记录、恢复 CLI 支持的会话，并用可配置快捷键进行多显示器截图。 | 进程恢复不等于任务完成；恢复行为因 CLI 而异。 |
| PaneForge 任务规划 | 创建并持久化蓝图主任务/子任务及其说明、验收标准；查看控制平面任务、干预、恢复、委派和插件记录。 | PaneForge 产品层；alpha.3 创建任务不会派发 CLI。 |
| 桌面工具与外观 | 使用命令面板、系统托盘、快捷终端、资源监控、主题、壁纸及终端/背景外观设置。 | 桌面工具功能；依赖 Windows 的行为仍需在 Windows 验证。 |
| 安装与验证 | 使用 Windows x64 签名更新通道，并下载对应 GPL 源码压缩包和校验值。 | PaneForge 发布基础设施；当前公开更新通道为 Windows x64 抢先版。 |

基础能力表描述的是当前 PaneForge 发布源码和安装包中存在的功能，不意味着每个第三方 CLI 的能力完全相同或默认启用。`0.0.3-alpha.3` 的公开发布及原生验收范围是 Windows x64 桌面端；浏览器、Android、macOS 和 Linux 客户端不属于本版公开承诺。准确的源码、许可证和修改记录以对应源码压缩包及仓库合规文件为准。

### 当前公开抢先版

- **多会话终端工作区：**使用标签页和可调整分屏组织多个 PTY 会话，通过 xterm.js 渲染终端。终端是执行界面，不是成功检测器；进程退出、输出“完成”或看似绿色的日志都不能单独证明验收通过。
- **工作空间和项目上下文：**工作空间组织多个项目及默认配置；项目提供仓库路径和项目级配置。发布版与开发版使用隔离的数据目录，开发构建不会悄悄接管发布版数据。
- **CLI 启动配置：**PaneForge 通过本机进程和终端适配器调用已安装的 CLI。发布源码包含 Claude Code、Codex CLI、Gemini CLI、Kimi、Grok、OpenCode 和 Cursor 的适配器，PTY 也能运行其他终端命令。启动上下文可包含项目、本机/WSL/SSH 运行环境、Provider、Skill、MCP、Hooks、权限选项和会话元数据；具体能力取决于用户安装和配置的 CLI/Provider。
- **蓝图任务规划：**alpha.3 支持新建主任务和子任务，填写标题、说明及逐条验收标准，并保存到本地数据库、保留父子关系。新任务会被选中以便继续规划。
- **仓库与文件工作：**在桌面中结合常用 Git/Worktree 操作、文件浏览器、代码编辑器、本地文件历史和 Diff，便于在终端旁检查仓库状态；这些功能不取代 Git 作为版本历史的权威来源。
- **MCP 与 Skill：**在适用的工作空间或项目范围配置工具和技能，并靠近启动上下文进行检查。第三方 CLI/MCP 仍具有其运行时和账户本身授予的权限。
- **Agent 控制平面记录：**任务树区分节点、父节点、会话、尝试、业务状态、修订号和运行健康；人工干预按节点、运行恢复按会话查询；委派及插件注册记录标记为全局范围。记录或审批状态不能证明 Agent/插件进程实际执行。

PaneForge 不提供模型或 Provider 账号。应用元数据保存在本地；选用的 CLI/Provider 可能依照自身设置和策略传输提示词、项目内容或遥测数据。

创建任务不会启动 CLI、自动指派 Worker，也不会宣称验收标准已经通过。尚未关联会话的任务会保持未分配状态。

[控制平面证据](../README.zh-CN.md#控制平面证据)逐张说明三幅历史 Windows 验收画面中的可见字段及证据边界。配套 P1.2c UI 验收快照记录测试应用为 `PaneForge DEV v0.0.1`；三张 PNG 本身都裁去了版本标识。它们使用合成 fixture，且都不是当前 alpha.3 安装包画面。当前版蓝图持久化验收图位于本页顶部；图像版本、来源和哈希见[媒体说明](media/README.md)。

### 蓝图开发线方向，尚未纳入 alpha.3 承诺

- **启动预览：**选择项目、工作目录、运行环境和启动配置，并在派发前显示失效配置。
- **冻结角色/团队模板：**复用团队配置，同时把本次解析后的选择冻结在任务上，后续修改模板不改写旧任务。
- **人工协作收件箱：**把问题或交接明确路由给指定人员，同时保留会话默认隔离。
- **交付视图：**集中展示任务要求、Git 改动、产物和真实验收证据。
- **受治理执行：**将进程、网络、凭据等有副作用的能力限制在明确的策略和宿主边界内。

这些能力标记为 `Development`。只有进入有版本号的安装包，并经过该版本源码及 Windows 验收证据覆盖后，才成为公开能力。

### 典型公开版流程

1. 安装 PaneForge，创建或导入工作空间并添加代码仓库。
2. 确认项目路径，选择已配置的本机、WSL 或 SSH 环境及本机安装的 CLI。
3. 在标签或分屏中启动一个或多个会话；需要时使用已配置的 MCP Server、Skill 和启动配置。
4. 按项目工作流使用 Agent Chat、Plans、Todos、编排或 Memory。
5. 需要拆解任务时创建带有可观察验收标准的蓝图任务；alpha.3 中它仍是已保存、未分配的计划。
6. 独立检查 Git 状态/Diff、Worktree、文件历史和编辑过的文件，不把终端聊天记录当作唯一依据。
7. 通过项目交付流程保留结果和证据；进程结束本身不表示验收通过。

alpha.3 原生验收覆盖了蓝图任务创建，以及刷新和应用重启后的持久化。完整任务派发、冻结模板、收件箱路由和集中交付审查仍在开发线完成中。

### 发布版本与来源

本文描述的公开产品版本是 `0.0.3-alpha.3`，对应 GPL 源码随[版本发布页](https://github.com/lihuabin1516-design/PaneForge-Releases/releases/tag/v0.0.3-alpha.3)提供。源码提交、源代码哈希、安装包校验值和 Windows 原生验收记录共同定义本版 PaneForge 的实际范围。

### 当前限制

- 公开更新通道目前面向 Windows x64 抢先版。
- 当前发布安装包和原生验收范围是 Windows x64 桌面端；浏览器、Android、macOS 和 Linux 客户端不属于当前公开版承诺范围。
- PaneForge 不包含模型访问，也不保证具体 CLI/Provider 的可用性和行为。
- 会话隔离及能力治理不等于对任意第三方 CLI 实施操作系统级沙箱。
- 标记为 `Development` 的蓝图执行及交付能力不是 alpha.3 的产品承诺。
- Windows 桌面特性必须在 Windows 环境真实验证；仅在 WSL/Linux 通过源码检查不足以验收。
