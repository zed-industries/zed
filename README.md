# Zed

[![Zed](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/zed-industries/zed/main/assets/badge/v0.json)](https://zed.dev)
[![CI](https://github.com/zed-industries/zed/actions/workflows/run_tests.yml/badge.svg)](https://github.com/zed-industries/zed/actions/workflows/run_tests.yml)

Welcome to Zed, a high-performance, multiplayer code editor from the creators of [Atom](https://github.com/atom/atom) and [Tree-sitter](https://github.com/tree-sitter/tree-sitter).

Zed reached its 1.0 milestone in April 2026 and has shipped weekly stable releases since. See the stable release changelog at [zed.dev/releases/stable](https://zed.dev/releases/stable) for the latest version information.

## Features

In addition to editing and multiplayer collaboration, Zed includes:

- Agent Panel with support for the Agent Client Protocol (ACP)
- Inline diff rendering from external agents
- Open-weight edit prediction model (Zeta)
- Debug Adapter Protocol (DAP) debugger for Rust, C/C++, JavaScript, Go, and Python
- WebAssembly-based extension system
- Hosted and local AI inference support through providers such as Anthropic, OpenAI, Google, AWS Bedrock, and `llama.cpp`

## Editions and Pricing

Zed offers Personal, Pro, and Business plans.

For the latest plan information, features, and pricing, see:

- https://zed.dev/pricing

## Security and Compliance Status

As of this writing, SSO, SAML/OIDC, SCIM, and SOC 2 certification are not yet available.

Zed does not store prompts or use them to train models. Hosted model agreements are based on zero-data-retention policies, with limited exceptions where noted by individual providers.

Organizations with regulatory requirements should evaluate Zed according to their own compliance requirements and verify current status before deployment.

For additional information, contact:

- sales@zed.dev

---

## Installation

On macOS, Linux, and Windows, you can download Zed directly or install it through your local package manager.

- [Download Zed](https://zed.dev/download)
- [Install on macOS](https://zed.dev/docs/installation#macos)
- [Install on Linux](https://zed.dev/docs/linux#installing-via-a-package-manager)
- [Install on Windows](https://zed.dev/docs/windows#package-managers)

Other platforms are not yet available:

- Web ([tracking discussion](https://github.com/zed-industries/zed/discussions/26195))

## Developing Zed

- [Building Zed for macOS](./docs/src/development/macos.md)
- [Building Zed for Linux](./docs/src/development/linux.md)
- [Building Zed for Windows](./docs/src/development/windows.md)

The required Rust toolchain version is pinned in [`rust-toolchain.toml`](./rust-toolchain.toml).

Extensions are WebAssembly-based. See the extension documentation if you are building an extension rather than the editor itself.

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for information about contributing to Zed.

Interested in working on Zed full-time? See our [jobs](https://zed.dev/jobs) page for open roles.

## Community

For support and general discussion, use [Zed's community channels](https://discord.com/invite/zedindustries).

GitHub Issues are intended for bug reports and feature requests.

## Licensing

Zed source code is licensed primarily under GPL-3.0-or-later, with Apache-2.0 components where marked. Server-side components are licensed under AGPL-3.0. See `LICENSE-GPL`, `LICENSE-APACHE`, and `LICENSE-AGPL` in the repo root, and check individual file headers when in doubt — this matters if you're redistributing or building a commercial product on top of Zed's code.

License information for third party dependencies must be correctly provided for CI to pass.

We use [`cargo-about`](https://github.com/EmbarkStudios/cargo-about) to automatically comply with open source licenses. If CI is failing, check the following:

- Is it showing a `no license specified` error for a crate you've created? If so, add `publish = false` under `[package]` in your crate's Cargo.toml.
- Is the error `failed to satisfy license requirements` for a dependency? If so, first determine what license the project has and whether this system is sufficient to comply with this license's requirements. If you're unsure, ask a lawyer. Once you've verified that this system is acceptable add the license's SPDX identifier to the `accepted` array in `script/licenses/zed-licenses.toml`.
- Is `cargo-about` unable to find the license for a dependency? If so, add a clarification field at the end of `script/licenses/zed-licenses.toml`, as specified in the [cargo-about book](https://embarkstudios.github.io/cargo-about/cli/generate/config.html#crate-configuration).

## Sponsorship

Zed is developed by **Zed Industries, Inc.**, a venture-backed, for-profit company.

The editor is free and open source. Company revenue comes primarily from paid Pro and Business plans.

If you would like to support the project financially, you can do so through GitHub Sponsors.

Sponsorships go directly to Zed Industries and are treated as general company revenue. Sponsorship does not include additional product benefits or entitlements.
