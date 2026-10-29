# Media provenance

The Blueprint images in this directory are derived from the isolated Windows acceptance screenshots for PaneForge `0.0.3-alpha.3`, run [40461026994](https://github.com/lihuabin1516-design/PaneForge-Releases/actions/runs/40461026994). The workflow uses a temporary profile and generic fixture titles (`Desktop acceptance root` and `Desktop acceptance child`). No user workspace, real session, credential, or private repository path is present in the image.

The GIF is a two-frame animation made from two separate acceptance snapshots. Each frame is shown for 1.2 seconds. It is not a continuous capture or a simulation of hidden functionality. The bottom test-runner status strip was cropped; the application UI was otherwise kept intact.

| File | Meaning | SHA-256 |
| --- | --- | --- |
| `blueprint-alpha3-created.png` | Parent and child after creation, including the selected child's description and acceptance criterion | `2afc1012cf22f6fbea017ca31dc4e52d10e5c81a0c50fb0027a627aa12d659a0` |
| `blueprint-alpha3-reopened.png` | Separate frame after application restart; both tasks and their parent-child relationship remain, and the child stays selected | `4ab79b5deec164122a27ed58911495239d8b5b4475621030074b9670437c841f` |
| `blueprint-alpha3-acceptance.gif` | Two-snapshot animated sequence | `d6a798321f1bf1d358007a0908aa67dadb0600cf921912f3e764901128082a3e` |

The public README also contains a compact Control Plane tour assembled from the three historical DEV panel captures:

| File | Meaning | SHA-256 |
| --- | --- | --- |
| `control-plane-dev-tour.gif` | Three-frame sequence: task tree, scoped intervention/recovery history, and global delegation/plugin governance | `4fcdebfe00f6e7697b28fa3717d152ae3b56ea066009e83bf00e218999417e7c` |

The Control Plane GIF is a presentation derivative, not a new acceptance run. Each frame is a panel crop from the DEV captures described below, with synthetic `[TEST FIXTURE]` rows and no user data. It has no claim of continuous recording, live process execution, or alpha.3 installer identity.

Source evidence: the local acceptance result recorded `nativeVersion=0.0.3-alpha.3`, root and child creation, selection after refresh, renderer reload and application restart as passed, with empty `pageErrors` and `cleanupErrors`. The public run link is the canonical evidence owner; this README only records how its images were presented.

## Isolated product tour

The home screenshot and three two-frame GIFs were captured from a locally built Windows instance displaying PaneForge 0.0.3-alpha.3, using a separate demo data directory. They are product-interface examples, not acceptance evidence for the published installer.

The demo workspace, task titles, and repository changes are synthetic. PaneForge started one local PowerShell terminal in the demo repository. The workbench GIF shows read-only Git status and diff checks scoped to src/hello.ts; that fixture change existed before capture. No Agent CLI or model provider was launched. The settings GIF shows configuration controls only, not external tool execution. The bottom status strip containing the machine-local path was cropped from these images.

Each GIF presents two real UI states for 1.5 seconds per frame. They are short state-transition tours, not continuous screen recordings.

| File | What it shows | SHA-256 |
| --- | --- | --- |
| `paneforge-overview.png` | Workbench home, isolated demo project, waiting local terminal, and task overview | `A9D201E75DEC5F1FB53C0A7E19DDBEB593E1F8124E2CC4117752C6FB22D80EE7` |
| `paneforge-workbench-tour.gif` | Workspace tree, scoped Git status, and the corresponding source diff | `89D30BB005C0EE47B4CC73CECEBCDAA7EA950AE5ED782E4A1BF26BA3C3ED6E5E` |
| `paneforge-tasks-tour.gif` | Synthetic TodoList entry and task detail | `B77D52CA74754B73BA6D3055F837D5515C677A678C1EDD3EC3A2EC8FC317302F` |
| `paneforge-settings-tour.gif` | Run configuration summary and application theme selector | `608F79E814AA39E0E427FD4C5F5DD85CE0D6912E6E3654BF338EF8352CE69983` |

## Historical Control Plane Captures

The three `control-plane-*-dev.png` panels below were captured on 2026-09-19 during isolated Windows acceptance of the Control Plane views. The companion P1.2c UI acceptance snapshot records the tested app as `PaneForge DEV v0.0.1`; all three PNGs are panel crops without app chrome or version labels. These are historical feature-evidence images, not current alpha.3 installer screenshots. All visible rows are explicitly labelled `[TEST FIXTURE]` and use synthetic identities. They contain no real workspace, conversation, credential or customer record. Their full-screen acceptance reports include private local paths and process identifiers, so those reports are not copied into this public repository.

The image bytes match the corresponding files in the published `v0.0.3-alpha.3` source snapshot (peeled source commit `ecb76a4348e8139a48bacb4433a449f7a67ff6f1`). That establishes source-file identity, not that the 0.0.1 development UI is identical to the current release. The original development ledger recorded 29 focused tests for task-tree projection, 40 for scoped history and 51 for global governance; these are historical counts, not tests rerun as part of this documentation update.

| File | What to inspect | Original source path | SHA-256 |
| --- | --- | --- | --- |
| `control-plane-task-tree-dev.png` | Parent/child ordering and state; selected child details include distinct node, parent, session and attempt IDs, business state, runtime health and revision | `docs/acceptance/p12a-task-tree.png` | `8ce6269af596dc85ed9692839653cd40f435e3a633078cd2c6d9ed99f2169101` |
| `control-plane-history-dev.png` | Node-scoped intervention requests and session-scoped recovery transitions, with actor, reason, timestamp, current/older attempts and another node in the same session | `docs/acceptance/p12b-history.png` | `bd8fe8da2f9edf56b9898ff61552e13d7a4bd9e402a5784cfd382d64e8d32e09` |
| `control-plane-governance-dev.png` | Global delegation decisions and review reasons beside plugin registry entries with distinct registered/enabled states; an explicit execution-boundary notice | `docs/acceptance/p12c-governance.png` | `2e2fdc96588b4b4b86081d6f4b438cc3169fe2e5f59a584824ca8b3d0907863c` |

## Presentation Reference

This media index follows a PaneForge-first presentation: a native acceptance sequence appears before the module-specific evidence, and every historical panel is accompanied by its capture date, fixture boundary, and verification hash. No third-party product image, author profile, contact detail, or QR code is used as PaneForge branding.
