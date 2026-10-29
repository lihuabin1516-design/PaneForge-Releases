# Security policy

## Scope

This policy covers the public PaneForge distribution, installer, updater, release workflows, and the code or documentation published in this repository.

## Reporting

Please do not publish secrets or an exploitable issue in a public issue. Use GitHub's private vulnerability reporting for `PaneForge-Releases` when available. If private reporting is unavailable, open a minimal issue requesting a private channel and do not include reproduction credentials, tokens, user paths, or session transcripts.

Include:

- affected release and Windows version;
- impact and a minimal reproduction;
- whether the issue requires a configured provider, MCP server, Skill, or local policy;
- sanitized logs or a small proof of concept.

## Release safety rules

- Never commit updater signing keys, provider credentials, or GitHub tokens.
- Redact bearer tokens, passwords, secrets, and user paths from diagnostics.
- Treat a source archive, installer hash, and update signature as a single release record.
- Do not upload private workspace data as acceptance evidence.

Security fixes may be released before a detailed public explanation when disclosure would increase user risk.
