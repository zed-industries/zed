# Zed

[![Zed](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/zed-industries/zed/main/assets/badge/v0.json)](https://zed.dev)
[![CI](https://github.com/zed-industries/zed/actions/workflows/run_tests.yml/badge.svg)](https://github.com/zed-industries/zed/actions/workflows/run_tests.yml)

Welcome to Zed, a high-performance, multiplayer code editor from the creators of [Atom](https://github.com/atom/atom) and [Tree-sitter](https://github.com/tree-sitter/tree-sitter).

> **Fork of `zed-industries/zed`.** This repository contains fork-specific CI and a
> fork-specific auto-update endpoint. See [Fork builds & auto-update](#fork-builds--auto-update) below.

---

### Fork builds & auto-update

This fork keeps `main` in sync with upstream and publishes a ready-to-install
Windows build:

- [`sync-fork`](.github/workflows/sync-fork.yml) runs daily at 03:00 UTC and merges
  `zed-industries/zed` `main` into this fork's `main` (local fork commits are preserved
  via a merge commit).
- [`build-binaries`](.github/workflows/build-binaries.yml) runs after each successful
  sync (or manually via **Actions → build-binaries → Run workflow**). It builds the
  Windows installer (`.exe`) and publishes a GitHub release on this fork. Each build
  bumps the release version, so the built binary detects the new version.

The auto-update check in `crates/auto_update` is patched to query **this fork's
GitHub releases** instead of `zed.dev`:

- Release assets are named `zed-{os}-{arch}.{ext}`. The current release contains
  `zed-windows-x86_64.exe` and the Windows remote server archive.
- The fork builds on the `stable` release channel (`crates/zed/RELEASE_CHANNEL`), so
  `zed update` polls for updates and installs them in place on Windows.
- To point the update at a different fork, set `ZED_FORK_REPO` (e.g. `owner/repo`) at
  build time.
- macOS binaries are not produced by this workflow. Build macOS locally with
  `cargo run -p zed` or `./script/bundle-mac -d -o aarch64-apple-darwin`.

If `sync-fork` hits a merge conflict (e.g. upstream changed `crates/auto_update`),
resolve it locally and push, then re-run the workflow.


### Local macOS builds

The GitHub binary workflow intentionally skips macOS. To run this fork locally:

```sh
cargo run -p zed
```

For a local `.app` bundle:

```sh
./script/bundle-mac -d -o aarch64-apple-darwin
```

An ad-hoc-signed local app may be blocked by Gatekeeper. Remove quarantine
from that local copy only:

```sh
xattr -dr com.apple.quarantine /Applications/Zed.app
open /Applications/Zed.app
```

### Installation

On macOS, Linux, and Windows you can [download Zed directly](https://zed.dev/download) or install Zed via your local package manager ([macOS](https://zed.dev/docs/installation#macos)/[Linux](https://zed.dev/docs/linux#installing-via-a-package-manager)/[Windows](https://zed.dev/docs/windows#package-managers)).

Other platforms are not yet available:

- Web ([tracking discussion](https://github.com/zed-industries/zed/discussions/26195))

### Developing Zed

- [Building Zed for macOS](./docs/src/development/macos.md)
- [Building Zed for Linux](./docs/src/development/linux.md)
- [Building Zed for Windows](./docs/src/development/windows.md)

### Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for ways you can contribute to Zed.

Also... we're hiring! Check out our [jobs](https://zed.dev/jobs) page for open roles.

### Licensing

Zed source code is licensed primarily under GPL-3.0-or-later, with Apache-2.0 components where marked.

License information for third party dependencies must be correctly provided for CI to pass.

We use [`cargo-about`](https://github.com/EmbarkStudios/cargo-about) to automatically comply with open source licenses. If CI is failing, check the following:

- Is it showing a `no license specified` error for a crate you've created? If so, add `publish = false` under `[package]` in your crate's Cargo.toml.
- Is the error `failed to satisfy license requirements` for a dependency? If so, first determine what license the project has and whether this system is sufficient to comply with this license's requirements. If you're unsure, ask a lawyer. Once you've verified that this system is acceptable add the license's SPDX identifier to the `accepted` array in `script/licenses/zed-licenses.toml`.
- Is `cargo-about` unable to find the license for a dependency? If so, add a clarification field at the end of `script/licenses/zed-licenses.toml`, as specified in the [cargo-about book](https://embarkstudios.github.io/cargo-about/cli/generate/config.html#crate-configuration).

## Sponsorship

Zed is developed by **Zed Industries, Inc.**, a for-profit company.

If you’d like to financially support the project, you can do so via GitHub Sponsors.
Sponsorships go directly to Zed Industries and are used as general company revenue.
There are no perks or entitlements associated with sponsorship.

