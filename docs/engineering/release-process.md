# Public release process

The distribution repository is intentionally smaller than the private product repository. A release is a signed, traceable handoff from a private source revision to public assets.

## Release inputs

Each release records:

- PaneForge version and channel (`stable` or `preview`);
- exact private source commit;
- corresponding source ZIP and SHA-256;
- package hash and update signature;
- lockfile hashes and toolchain versions;
- Windows native acceptance run and evidence links.

## Release sequence

1. Freeze the source revision and generate the corresponding source ZIP without private Git history or user data.
2. Run source checks and native Windows acceptance from that exact archive.
3. Build and sign the installer and updater manifest using repository secrets.
4. Publish the installer, manifest, checksums, source ZIP, and bilingual release notes to GitHub Releases.
5. Run the public source-build workflow against the published source archive.
6. Update this repository's release index and verify the remote commit from a clean clone.

## Update behavior

The installer uses the PaneForge release namespace and preserves `%USERPROFILE%\.paneforge`. An update must not migrate or overwrite another application's data directory. Update signatures authenticate the release artifact; they are not the same thing as Windows Authenticode signing.

## Rollback

If a release fails acceptance, stop the preview channel, keep the previous release available, and publish a corrective release with a new source commit and new hashes. Do not replace an asset in place after publication.
