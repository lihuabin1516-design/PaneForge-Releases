# Media provenance

The images in this directory are derived from the isolated Windows acceptance screenshots for PaneForge `0.0.3-alpha.3`, run [40461026994](https://github.com/lihuabin1516-design/PaneForge-Releases/actions/runs/40461026994). The workflow uses a temporary profile and generic fixture titles (`Desktop acceptance root` and `Desktop acceptance child`). No user workspace, real session, credential, or private repository path is present in the image.

The GIF is a two-frame animation made from two separate acceptance snapshots. Each frame is shown for 1.2 seconds. It is not a continuous capture or a simulation of hidden functionality. The bottom test-runner status strip was cropped; the application UI was otherwise kept intact.

| File | Meaning | SHA-256 |
| --- | --- | --- |
| `blueprint-alpha3-created.png` | Tree after root/child task creation | `2afc1012cf22f6fbea017ca31dc4e52d10e5c81a0c50fb0027a627aa12d659a0` |
| `blueprint-alpha3-reopened.png` | Separate frame after application restart | `4ab79b5deec164122a27ed58911495239d8b5b4475621030074b9670437c841f` |
| `blueprint-alpha3-acceptance.gif` | Two-snapshot animated sequence | `d6a798321f1bf1d358007a0908aa67dadb0600cf921912f3e764901128082a3e` |

Source evidence: the local acceptance result recorded `nativeVersion=0.0.3-alpha.3`, root and child creation, selection after refresh, renderer reload and application restart as passed, with empty `pageErrors` and `cleanupErrors`. The public run link is the canonical evidence owner; this README only records how its images were presented.
