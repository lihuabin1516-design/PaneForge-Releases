# PaneForge product capabilities and workflow

PaneForge is designed for developers who supervise more than one AI coding session and need the sessions to stay attached to real project context. Its job is to make parallel work visible and recoverable: which project a CLI is working in, what task it is serving, which runtime was started, what changed, and what evidence supports delivery.

The public installer is a preview. This page separates capabilities present in `0.0.3-alpha.3` from work still in the private development line.

![Two native acceptance snapshots of the Blueprint task tree](media/blueprint-alpha3-acceptance.gif)

*The animation alternates between the task tree after creation and a separate snapshot after app restart. It is assembled from acceptance screenshots, not a continuous recording. See [media provenance](media/README.md).*

## Product model

```text
Workspace
  └─ Project (repository and tool configuration)
      └─ Task (intent and acceptance criteria)
          └─ Session (a concrete CLI process and PTY)
              └─ Evidence (Git changes, artifacts, checks, human review)
```

The hierarchy gives operators a place to answer four questions without treating chat output as the source of truth:

1. Which workspace and repository does this work belong to?
2. What outcome and acceptance criteria were agreed for it?
3. Which process/session actually ran, and what happened to it?
4. What changed, and what evidence supports accepting the result?

## Capabilities in the public preview

### Multi-session terminal workspace

PaneForge arranges terminal sessions in tabs and resizable split panes. Each terminal is backed by a PTY and rendered with xterm.js. Developers can keep several CLI sessions visible while moving between repositories and workspace layouts.

The terminal is an execution surface, not a success detector. An exited process, a `done` message, or a green-looking transcript is not by itself acceptance evidence.

### Workspace and project context

Workspaces group projects and their defaults. Projects provide the repository path and project-specific configuration used by launches. The application keeps its release and development profiles separate, so trying a development build does not silently reuse the release data directory.

### CLI launch configuration

PaneForge integrates with installed CLI tools through local process and terminal adapters. Launch context can include the selected project, runtime, provider, skills, MCP configuration, and session metadata. Availability depends on which CLI tools and providers the user has installed and configured.

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

The [control-plane visual tour](../README.md#control-plane-visual-tour) explains three historical Windows acceptance captures, including the visible fields and the evidence boundary for each view. The companion P1.2c UI acceptance snapshot records the tested app as `PaneForge DEV v0.0.1`; all three PNGs are cropped without version labels. All three use synthetic fixtures and none is a screenshot of the current alpha.3 installer. The current release's Blueprint persistence acceptance is shown at the top of this page.

## Blueprint workflow in development

The private development line is extending task planning toward an auditable execution and delivery loop:

- **Launch preview:** select project, working directory, runtime, and launch profile; report stale or invalid configuration before dispatch.
- **Frozen role/team templates:** reuse a team setup while snapshotting the resolved choices on each task, so editing a template later does not rewrite old work.
- **Human collaboration inbox:** direct a question or handoff to a named person while preserving the default isolation between sessions.
- **Delivery view:** bring task requirements, Git changes, artifacts, and actual acceptance evidence together for review.
- **Governed execution:** keep process, network, secret, and other side-effecting capabilities behind explicit policy and host boundaries.

These are marked `Development`, not public preview. They become public capabilities only when included in a versioned installer and covered by the release's source and native acceptance evidence.

## Typical public-preview workflow

1. Install PaneForge and open or create a workspace.
2. Add a project and confirm its repository path and available CLI tools.
3. Open the Blueprint panel and create a task with observable acceptance criteria.
4. Start a CLI session in the appropriate project context and work in its terminal pane.
5. Inspect the repository diff, local file history, and task state independently of the terminal transcript.
6. Record the result and evidence through the current project workflow; do not mark a plan accepted merely because a process ended.

The first three planning steps are covered by the alpha.3 Blueprint smoke. Full task dispatch, frozen templates, collaboration inbox routing, and consolidated delivery review are still being completed in the development line.

## Design principles

- **Local-first state:** workspace and application metadata are stored on the user's machine; external CLIs and providers retain their own data handling behavior.
- **Visible boundaries:** workspace, project, task, process, and provider are separate concepts rather than hidden defaults.
- **Recoverable lifecycle:** process restart and task completion are different events.
- **Evidence before claims:** a task's acceptance state should be grounded in inspected changes and reproducible checks.
- **Least surprise:** creating a plan does not execute it, a stopped CLI does not imply success, and development and release profiles stay isolated.
- **Release traceability:** each public installer is tied to a source archive, hashes, workflow inputs, and Windows verification.

## Current limitations

- The public update channel currently targets Windows x64 preview packages.
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
      └─ 任务（目标与验收标准）
          └─ 会话（真实 CLI 进程与 PTY）
              └─ 证据（Git 改动、产物、检查结果、人工审查）
```

这个分层用于回答四个问题：工作属于哪个仓库和工作空间；预期结果与验收标准是什么；真实启动过哪个进程、生命周期如何；产生了什么改动、有哪些证据支持验收。终端输出本身不是权威状态源。

### 当前公开抢先版

- **多会话终端工作区：**使用标签页和可调整分屏组织多个 PTY 会话，通过 xterm.js 渲染终端。终端是执行界面，不是成功检测器；进程退出、输出“完成”或看似绿色的日志都不能单独证明验收通过。
- **工作空间和项目上下文：**工作空间组织多个项目及默认配置；项目提供仓库路径和项目级配置。发布版与开发版使用隔离的数据目录，开发构建不会悄悄接管发布版数据。
- **CLI 启动配置：**PaneForge 通过本机进程和终端适配器调用已安装的 CLI。启动上下文可包含项目、运行环境、Provider、Skill、MCP 和会话元数据；具体能力取决于用户安装和配置的 CLI/Provider。
- **蓝图任务规划：**alpha.3 支持新建主任务和子任务，填写标题、说明及逐条验收标准，并保存到本地数据库、保留父子关系。新任务会被选中以便继续规划。
- **仓库与文件工作：**在桌面中结合常用 Git/Worktree 操作、文件浏览器、代码编辑器、本地文件历史和 Diff，便于在终端旁检查仓库状态；这些功能不取代 Git 作为版本历史的权威来源。
- **MCP 与 Skill：**在适用的工作空间或项目范围配置工具和技能，并靠近启动上下文进行检查。第三方 CLI/MCP 仍具有其运行时和账户本身授予的权限。
- **Agent 控制平面记录：**任务树区分节点、父节点、会话、尝试、业务状态、修订号和运行健康；人工干预按节点、运行恢复按会话查询；委派及插件注册记录标记为全局范围。记录或审批状态不能证明 Agent/插件进程实际执行。

PaneForge 不提供模型或 Provider 账号。应用元数据保存在本地；选用的 CLI/Provider 可能依照自身设置和策略传输提示词、项目内容或遥测数据。

创建任务不会启动 CLI、自动指派 Worker，也不会宣称验收标准已经通过。尚未关联会话的任务会保持未分配状态。

[控制平面图集](../README.zh-CN.md#控制平面图集)逐张说明三幅历史 Windows 验收画面中的可见字段及证据边界。配套 P1.2c UI 验收快照记录测试应用为 `PaneForge DEV v0.0.1`；三张 PNG 本身都裁去了版本标识。它们使用合成 fixture，且都不是当前 alpha.3 安装包画面。当前版蓝图持久化验收图位于本页顶部；图像版本、来源和哈希见[媒体说明](media/README.md)。

### 蓝图开发线方向，尚未纳入 alpha.3 承诺

- **启动预览：**选择项目、工作目录、运行环境和启动配置，并在派发前显示失效配置。
- **冻结角色/团队模板：**复用团队配置，同时把本次解析后的选择冻结在任务上，后续修改模板不改写旧任务。
- **人工协作收件箱：**把问题或交接明确路由给指定人员，同时保留会话默认隔离。
- **交付视图：**集中展示任务要求、Git 改动、产物和真实验收证据。
- **受治理执行：**将进程、网络、凭据等有副作用的能力限制在明确的策略和宿主边界内。

这些能力标记为 `Development`。只有进入有版本号的安装包，并经过该版本源码及 Windows 验收证据覆盖后，才成为公开能力。

### 典型公开版流程

1. 安装 PaneForge，打开或创建工作空间。
2. 添加项目并确认仓库路径及本机可用的 CLI。
3. 在蓝图面板创建任务，并写出可观察的验收标准。
4. 在对应项目上下文启动 CLI 会话，在终端面板中工作。
5. 独立检查仓库 Diff、本地文件历史和任务状态，不把终端聊天记录当作唯一依据。
6. 按当前项目流程记录结果和证据；不能仅因进程退出就把计划标记为验收通过。

前三个规划步骤由 alpha.3 蓝图冒烟验收覆盖。完整任务派发、冻结模板、收件箱路由和集中交付审查仍在开发线完成中。

### 当前限制

- 公开更新通道目前面向 Windows x64 抢先版。
- PaneForge 不包含模型访问，也不保证具体 CLI/Provider 的可用性和行为。
- 会话隔离及能力治理不等于对任意第三方 CLI 实施操作系统级沙箱。
- 标记为 `Development` 的蓝图执行及交付能力不是 alpha.3 的产品承诺。
- Windows 桌面特性必须在 Windows 环境真实验证；仅在 WSL/Linux 通过源码检查不足以验收。
