# PaneForge Releases

PaneForge 是基于 CC-Panes 的多 CLI 并行开发工作台。本仓库用于公开分发安装包、签名、更新清单、中英文更新说明和对应版本源码。

[下载 Windows x64 抢先版](https://github.com/lihuabin1516-design/PaneForge-Releases/releases/tag/v0.0.3-alpha.2)

首次使用请下载 `PaneForge_0.0.3-alpha.2_x64-setup.exe`。旧的 `0.0.3-alpha.1` 需要手动安装一次；从 `0.0.3-alpha.2` 开始，可在“设置 → 关于 → 检查更新”检查 preview 通道的新版本。安装前请保存工作。

PaneForge 使用 `%USERPROFILE%\.paneforge` 独立数据目录。安装更新保留用户数据。当前公开更新通道仅支持 Windows x64；抢先版仍在持续完善。

每个版本同时提供 `PaneForge_VERSION_source.zip` 对应源码和 `SHA256SUMS.txt`。PaneForge 基于 [CC-Panes](https://github.com/wuxiran/cc-pane)，按 GPL-3.0 发布；许可证及第三方声明包含在源码 ZIP 中。源码 ZIP 不包含私有开发仓库的 Git 历史或用户数据。Tauri 更新签名用于验证更新包，不等同于 Windows Authenticode 代码签名。

---

PaneForge is a multi-CLI development workbench based on CC-Panes. This repository distributes signed Windows x64 preview installers, update manifests, bilingual release notes, checksums, and the corresponding GPL-3.0 source ZIP for each version.

Install `0.0.3-alpha.2` once to enable the preview update channel. Then use **Settings → About → Check for updates** for subsequent releases. Save your work before installing. User data remains in `%USERPROFILE%\.paneforge`.

The `preview` release contains the channel manifest. Versioned releases contain installers and source archives. Source ZIPs contain the corresponding source revision without private Git history or user data.
