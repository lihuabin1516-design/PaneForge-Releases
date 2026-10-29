# PaneForge source provenance

PaneForge is maintained as an independent product by `lihuabin1516-design`. Its product identity, namespaces, data layout, control-plane additions, public documentation, release automation, and acceptance criteria are maintained in the PaneForge project.

The codebase also contains modified GPL-covered foundation code derived from the open-source CC-Panes project. The upstream project is available at <https://github.com/wuxiran/cc-pane>. This link is provided for legal provenance and comparison; PaneForge is not the upstream project's official distribution, website, or support channel.

## Recorded foundation revisions

- Initial PaneForge foundation: upstream `0.12.20`, commit `a2a954efab569ec6e425c59f25695c530c205ba7`.
- Tracked upstream baseline for the current development line: `0.12.21`, commit `a313dd1d0e88e520781e48ed684977364b4e7da4` (tag `a784f43adbdbf4b5f437e1d180523199cc0fd08e`).
- The private repository's adoption matrix records which upstream changes were actually ported, adapted, and tested. A version label alone must not be read as a claim that every upstream change is present.

## What is independent PaneForge work

The following are maintained as PaneForge product work and are not presented as upstream features:

- PaneForge branding, executable names, bundle identifiers, deep links, ports, environment namespace, and `%USERPROFILE%\.paneforge` data routing;
- the control-plane records for workspaces, projects, tasks, sessions, launch bindings, evidence, and recovery;
- capability Broker and plugin-host policy, native bootstrap hardening, and replay fencing;
- Blueprint task creation and the development-line launch preview, team freezing, human handoff, and delivery-view design;
- public release verification, corresponding-source packaging, bilingual documentation, and this repository split.

## Modification and distribution obligations

The complete corresponding source for a public binary is provided as the versioned `PaneForge_<version>_source.zip` attached to the same GitHub Release. The archive includes the GPL license and modification notices but excludes private Git history, user data, credentials, and signing keys.

This document is an attribution and modification record. It does not rebrand the upstream project, remove its copyright notices, or claim that the modified foundation was written from scratch.
