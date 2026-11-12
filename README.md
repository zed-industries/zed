> [!IMPORTANT]
> Remove this line to confirm you've reviewed this PR before submitting.
# Zed

[![Zed](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/zed-industries/zed/main/assets/badge/v0.json)](https://zed.dev)
[![CI](https://github.com/zed-industries/zed/actions/workflows/run_tests.yml/badge.svg)](https://github.com/zed-industries/zed/actions/workflows/run_tests.yml)

Welcome to Zed, a high-performance, multiplayer code editor from the creators of [Atom](https://github.com/atom/atom) and [Tree-sitter](https://github.com/tree-sitter/tree-sitter).

---

### Installation

On macOS, Linux, and Windows you can [download Zed directly](https://zed.dev/download) or install Zed via your local package manager ([macOS](https://zed.dev/docs/installation#macos)/[Linux](https://zed.dev/docs/linux#installing-via-a-package-manager)/[Windows](https://zed.dev/docs/windows#package-managers)).

Other platforms are not yet available:

- Web ([tracking discussion](https://github.com/zed-industries/zed/discussions/26195))

### Updating this custom macOS build

From this checkout, run:

```sh
./script/update-local-zed
```

The script requires a clean `main`, fetches `origin/main`, fast-forwards without
rewriting local history, runs the focused pi review tests, and builds a signed
release bundle with official self-updates disabled. It uses `script/bundle-mac`,
including that script's build-tool installation and packaging dependencies.

Save drafts and quit all Zed instances before installation. The script never
quits them for you. To keep using Zed during the build:

```sh
./script/update-local-zed build
# Save drafts and quit Zed, then:
./script/update-local-zed install
```

Installation uses `sudo`, stages and verifies the replacement, and preserves the
previous app under `~/Library/Application Support/ZedPiReview/app-backups/`.
A failed replacement or CLI verification restores the previous app. A staging
failure leaves the installed app untouched; inspect any reported staging path
before retrying. The saved build commit and executable digest prevent installing
an outdated or subsequently overwritten build through `install`.

Your existing `zed` command continues to use `/Applications/Zed.app`; settings
and profiles are unchanged. Ubuntu remote servers and the pi adapter are not
deployed by this command. New official Zed releases must first be integrated and
reviewed in the fork; diverged Git history stops with instructions rather than
being automatically rebased or reset. Local signing does not provide official
notarization or universal-link entitlements.

Implementation decisions: track this fork's `origin/main`, use the existing
release bundler, keep installation separate when Zed is open, and preserve the
normal profile and existing CLI link. These choices avoid forced restarts,
implicit upstream-version changes, and configuration migration.

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
