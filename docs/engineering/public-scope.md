# Public scope and review checklist

This document defines what belongs in the public distribution repository and what remains in the private product repository.

## Public by default

- release installers, update manifests, checksums, and corresponding source archives;
- user-facing installation and update instructions;
- architecture, verification, and public API documentation;
- reproducible GitHub Actions that do not contain secrets;
- sanitized acceptance evidence that contains no user paths or tokens;
- GPL license, third-party notices, and source provenance.

## Private by default

- unreleased implementation branches and internal design discussions;
- `.paneforge-task/`, `.playwright-cli/`, session transcripts, and local task ledgers;
- provider credentials, update signing keys, API tokens, and runner secrets;
- user workspace data, repository paths, screenshots, and diagnostic memory dumps;
- acceptance material that exposes private infrastructure or host identifiers;
- product work that is not present in the corresponding public installer.

## Release wording rules

Use `Preview` only when the public installer and its evidence cover the capability. Use `Development` for private-line work that is not yet in the installer. Use `Planned` for an idea without an implementation and acceptance run. Never describe a private design document as a released feature.

## Pre-push review

Before pushing this repository, check:

1. `git status` contains no generated evidence, archives, credentials, or user paths.
2. The README and release notes use PaneForge terminology and do not copy another project's screenshots, author contacts, QR codes, or release prose.
3. Every binary release has a matching source ZIP, SHA-256 entry, version, source commit, and product identity.
4. Workflows use repository secrets through GitHub Actions and do not embed signing keys.
5. Public links resolve to this repository or the intended release asset.
6. GPL notices and the modification record remain intact.
