# Media provenance

The Blueprint images in this directory are derived from the isolated Windows acceptance screenshots for PaneForge `0.0.3-alpha.3`, run [40461026994](https://github.com/lihuabin1516-design/PaneForge-Releases/actions/runs/40461026994). The workflow uses a temporary profile and generic fixture titles (`Desktop acceptance root` and `Desktop acceptance child`). No user workspace, real session, credential, or private repository path is present in the image.

The GIF is a two-frame animation made from two separate acceptance snapshots. Each frame is shown for 1.2 seconds. It is not a continuous capture or a simulation of hidden functionality. The bottom test-runner status strip was cropped; the application UI was otherwise kept intact.

| File | Meaning | SHA-256 |
| --- | --- | --- |
| `blueprint-alpha3-created.png` | Parent and child after creation, including the selected child's description and acceptance criterion | `2afc1012cf22f6fbea017ca31dc4e52d10e5c81a0c50fb0027a627aa12d659a0` |
| `blueprint-alpha3-reopened.png` | Separate frame after application restart; both tasks and their parent-child relationship remain, and the child stays selected | `4ab79b5deec164122a27ed58911495239d8b5b4475621030074b9670437c841f` |
| `blueprint-alpha3-acceptance.gif` | Two-snapshot animated sequence | `d6a798321f1bf1d358007a0908aa67dadb0600cf921912f3e764901128082a3e` |

Source evidence: the local acceptance result recorded `nativeVersion=0.0.3-alpha.3`, root and child creation, selection after refresh, renderer reload and application restart as passed, with empty `pageErrors` and `cleanupErrors`. The public run link is the canonical evidence owner; this README only records how its images were presented.

## Historical Control Plane Captures

The three `control-plane-*-dev.png` panels below were captured on 2026-09-19 during isolated Windows acceptance of the Control Plane views. The companion P1.2c UI acceptance snapshot records the tested app as `PaneForge DEV v0.0.1`; all three PNGs are panel crops without app chrome or version labels. These are historical feature-evidence images, not current alpha.3 installer screenshots. All visible rows are explicitly labelled `[TEST FIXTURE]` and use synthetic identities. They contain no real workspace, conversation, credential or customer record. Their full-screen acceptance reports include private local paths and process identifiers, so those reports are not copied into this public repository.

The image bytes match the corresponding files in the published `v0.0.3-alpha.3` source snapshot (peeled source commit `ecb76a4348e8139a48bacb4433a449f7a67ff6f1`). That establishes source-file identity, not that the 0.0.1 development UI is identical to the current release. The original development ledger recorded 29 focused tests for task-tree projection, 40 for scoped history and 51 for global governance; these are historical counts, not tests rerun as part of this documentation update.

| File | What to inspect | Original source path | SHA-256 |
| --- | --- | --- | --- |
| `control-plane-task-tree-dev.png` | Parent/child ordering and state; selected child details include distinct node, parent, session and attempt IDs, business state, runtime health and revision | `docs/acceptance/p12a-task-tree.png` | `8ce6269af596dc85ed9692839653cd40f435e3a633078cd2c6d9ed99f2169101` |
| `control-plane-history-dev.png` | Node-scoped intervention requests and session-scoped recovery transitions, with actor, reason, timestamp, current/older attempts and another node in the same session | `docs/acceptance/p12b-history.png` | `bd8fe8da2f9edf56b9898ff61552e13d7a4bd9e402a5784cfd382d64e8d32e09` |
| `control-plane-governance-dev.png` | Global delegation decisions and review reasons beside plugin registry entries with distinct registered/enabled states; an explicit execution-boundary notice | `docs/acceptance/p12c-governance.png` | `2e2fdc96588b4b4b86081d6f4b438cc3169fe2e5f59a584824ca8b3d0907863c` |

## Presentation Reference

The official [CC-Panes README](https://github.com/wuxiran/cc-pane/blob/a313dd1d0e88e520781e48ed684977364b4e7da4/README.md) was checked on 2026-10-29 at upstream `main` HEAD `a313dd1d0e88e520781e48ed684977364b4e7da4` (commit dated 2026-10-12; its changelog identifies `0.12.21`). It places a hero capture first, then presents features one at a time with explanatory copy, a guide link, and a feature-specific GIF with a static-image fallback. PaneForge follows that structure for the three Control Plane views and explains visible fields, scope, and evidence limits beside each image. No upstream README image asset is reused.
