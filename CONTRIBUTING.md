# Contributing to PaneForge public distribution

This repository is the public distribution and documentation surface. It is not the place to submit private source snapshots, user data, or generated build output.

## Suitable changes

- clarify installation, update, architecture, or verification documentation;
- improve public release workflows without embedding secrets;
- correct checksums, links, or release metadata;
- add sanitized acceptance guidance that can be reproduced on a clean Windows runner.

Feature implementation belongs in the private product repository until it has an explicit public-release decision and a corresponding source archive.

## Pull requests

Use a clear title and explain:

1. what public behavior or documentation changes;
2. which release or workflow is affected;
3. how links, hashes, and sensitive-content checks were verified.

Use Conventional Commit prefixes such as `docs:`, `ci:`, `fix:`, or `chore:`. Do not commit `.paneforge` data, `.ccpanes-task`, `.playwright-cli`, session transcripts, API keys, update signing material, or local absolute paths.

## Licensing

Contributions are accepted under GPL-3.0. Keep source-provenance and third-party notices intact when changing release material.
