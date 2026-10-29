# PaneForge 面试阅读简报

这份简报给招聘方和技术面试官提供一条短路径：先理解产品价值，再验证一个真实功能，最后进入架构、测试和发布工程。所有结论以 `0.0.3-alpha.3` 的公开源码、发布资产和 Windows 验收记录为准。

## 给 HR：30 秒

PaneForge 是一个 Windows AI 工程工作台。它把多个代码仓库、编程 CLI 会话、任务计划、Git 改动和交付检查集中到一个工作空间里，帮助开发者看清楚工作属于哪个项目、任务原本要完成什么、过程留下了哪些可审查证据。

当前公开版本是 `0.0.3-alpha.3` 抢先版，已针对 Windows x64 做原生验收。它不是模型服务，也不替用户购买模型账号；它负责把本地工程工作组织起来，并把“计划、运行、改动、验收”之间的边界呈现出来。

## 给 HR：3 分钟

可以用下面四个问题判断项目是否值得继续了解：

| 观察点 | PaneForge 的回答 | 公开证据 |
| --- | --- | --- |
| 做什么 | 在一个 Windows 工作空间里管理多个项目和 AI 编程会话，并保留审查上下文。 | [首页产品定位](../README.zh-CN.md) |
| 解决什么 | 减少终端串台、任务意图丢失和“日志看起来成功但无法验收”的问题。 | [五个真实场景](../README.zh-CN.md#为什么要做这个产品) |
| 现在做到哪 | 蓝图任务创建与重启持久化已在 Windows 原生验收；更完整的派发、模板、人工收件箱和交付视图仍在开发。 | [版本状态](../README.zh-CN.md#版本测试与状态) |
| 如何证明 | 有安装包、对应源码 ZIP、SHA-256、自动化构建和原生验收运行。 | [发布证据](engineering/portfolio-evidence.md) |

## 给技术面试官：15 分钟路线

### 1. 从用户动作开始

下载 Windows 包后，进入工作空间，打开 **Agent 控制台 → Blueprint → 新建任务**，创建一个主任务和子任务，填写验收标准，然后刷新并重启应用。预期结果是父子关系、选中项、描述、标准、`planned` 状态和未分配会话仍然存在。

这条路径有真实截图和两帧动态图：[创建截图](media/blueprint-alpha3-created.png)、[重启后截图](media/blueprint-alpha3-reopened.png)、[验收动态图](media/blueprint-alpha3-acceptance.gif)。

### 2. 顺着一次请求读代码

1. [React 表单](../showcase/source/v0.0.3-alpha.3/web/components/controlplane/BlueprintTaskForm.tsx)：输入标准化、请求身份和错误反馈。
2. [任务模型](../showcase/source/v0.0.3-alpha.3/paneforge-core/src/models/task_tree.rs)：主/子任务、revision、状态和未分配语义。
3. [服务层](../showcase/source/v0.0.3-alpha.3/paneforge-core/src/services/task_tree_service.rs)：输入边界、父节点验证和业务规则。
4. [SQLite Repository](../showcase/source/v0.0.3-alpha.3/paneforge-core/src/repository/task_tree_repository.rs)：事务、幂等重试和持久化。
5. [聚焦测试](../showcase/source/v0.0.3-alpha.3/paneforge-core/src/services/blueprint_task_creation_tests.rs)：冲突、重开、父节点校验和运行时覆盖保护。

### 3. 讨论四个工程取舍

| 题目 | 期望讨论方向 | 证据和限制 |
| --- | --- | --- |
| 为什么新建任务不自动启动 CLI？ | 计划和执行是两个生命周期；先保存意图才能避免把 UI 点击误认为执行结果。 | alpha.3 的任务保持未分配；派发是后续开发线工作。 |
| 为什么 request UUID 需要进入事务？ | 用户重试可能发生在网络或渲染器边界；相同请求应复用结果，不同内容应冲突。 | 8 个核心持久化测试覆盖该路径。 |
| 为什么能力 Broker 返回收据而不是直接执行？ | 授权记录和宿主副作用必须分开；空 allowlist 默认拒绝。 | 14 个宿主适配器测试；这不是 OS 沙箱。 |
| 为什么要把运行时健康单独建模？ | 任务状态、会话状态、进程健康和人工决定有不同的所有者和作用域。 | [运行时边界](architecture/runtime-boundaries.md)和控制平面案例。 |

### 4. 最后看发布工程

核对版本 `0.0.3-alpha.3`、源码提交 `ecb76a4348e8139a48bacb4433a449f7a67ff6f1`、源码 ZIP SHA-256、安装包 SHA-256，以及 [Windows 原生验收运行](https://github.com/lihuabin1516-design/PaneForge-Releases/actions/runs/40461026994)。这一步可以把“功能演示”与“可追溯发布”区分开。

## 可以现场展示什么

- 工作空间和项目入口：展示产品如何组织上下文。
- Blueprint 主/子任务：展示任务说明、验收标准和持久化。
- 控制平面历史图：展示作用域、记录来源和证据边界；需说明它们是历史合成 fixture。
- 架构图和源码切片：展示前端、类型化服务、Tauri IPC、Rust 服务、Repository 和本地持久化之间的调用方向。
- 发布资产和校验值：展示安装包与对应源码如何绑定。

## 需要诚实说明的限制

- 当前公开原生验收范围是 Windows x64，不能据此承诺其他平台。
- 当前视觉材料没有活跃 CLI、Git Diff、编辑器和远程运行的脱敏原生截图。
- 蓝图创建不会自动派发 Worker，也不会自动判定验收通过。
- 控制平面历史图片用于解释记录模型，不能当作当前安装包的实时进程轨迹。
- 插件能力 Broker 是默认拒绝的授权契约，不是操作系统级沙箱。
- 逐提交作者账本不在公开展示仓库；个人贡献说明只使用当前公开材料可追溯的范围。

## 面试材料索引

- [项目首页](../README.zh-CN.md)
- [产品能力与工作流](product-capabilities.md)
- [工程案例与验收证据](engineering/portfolio-evidence.md)
- [架构与任务生命周期](architecture/product-map.md)
- [运行时与能力边界](architecture/runtime-boundaries.md)
- [可浏览源码切片](../showcase/README.md)
- [许可证与修改记录](compliance/source-attribution.md)
