# Zed

[![Zed](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/zed-industries/zed/main/assets/badge/v0.json)](https://zed.dev)
[![CI](https://github.com/zed-industries/zed/actions/workflows/run_tests.yml/badge.svg)](https://github.com/zed-industries/zed/actions/workflows/run_tests.yml)

Welcome to Zed, a high-performance, multiplayer code editor from the creators of [Atom](https://github.com/atom/atom) and [Tree-sitter](https://github.com/tree-sitter/tree-sitter).

Zed reached its 1.0 milestone in April 2026 and has shipped weekly stable releases since — v1.10.1 as of July 9, 2026 (changelog: [zed.dev/releases/stable](https://zed.dev/releases/stable)). Don't rely on this README for the exact current version; check the changelog link, since it updates weekly. Core capabilities beyond editing and multiplayer: an Agent Panel supporting the Agent Client Protocol (ACP), which lets external agents (Claude Code, Codex, OpenCode, etc.) drive the editor directly with diffs rendered inline; an open-weight edit-prediction model (Zeta); a Debug Adapter Protocol (DAP) debugger for Rust, C/C++, JavaScript, Go, and Python; and a WebAssembly-based extension system. AI inference can run against hosted providers (Anthropic, OpenAI, Google, AWS Bedrock, and others via BYOK) or fully locally via `llama.cpp`.

### Editions and pricing

- **Personal** — free. Full editor, unlimited use of BYOK/external agents, 2,000 accepted edit predictions/month.
- **Pro** — $10/month. Unlimited edit predictions plus bundled hosted-AI token credits (overage billed at provider list price).
- **Business** — $30/seat/month. Org-wide AI policy controls, prompt-sharing and edit-prediction training disabled by default, BYOK for major providers at no added fee.

Current as of July 2026; confirm details at [zed.dev/pricing](https://zed.dev/pricing) before procurement, as tiers have changed more than once this year.

### Security and compliance status

For sysadmins and enterprise evaluators: **SSO, SAML/OIDC, SCIM, and SOC 2 certification are not yet available** as of this writing — they are on Zed's roadmap but not shipped. Zed does not store prompts or use them to train models, and hosted-model agreements are zero-data-retention, with one documented exception (certain Anthropic Mythos-class models retain prompts/outputs briefly for safety review). Regulated environments should treat Zed as **not yet compliance-certified** and evaluate on that basis rather than assuming parity with certified competitors. Contact `sales@zed.dev` for current status or to be notified when these ship.

---

### Installation

On macOS, Linux, and Windows you can [download Zed directly](https://zed.dev/download) or install Zed via your local package manager ([macOS](https://zed.dev/docs/installation#macos)/[Linux](https://zed.dev/docs/linux#installing-via-a-package-manager)/[Windows](https://zed.dev/docs/windows#package-managers)). All three platforms are stable and at feature parity.

Other platforms are not yet available:

- Web ([tracking discussion](https://github.com/zed-industries/zed/discussions/26195))

### Developing Zed

- [Building Zed for macOS](./docs/src/development/macos.md)
- [Building Zed for Linux](./docs/src/development/linux.md)
- [Building Zed for Windows](./docs/src/development/windows.md)

Required Rust toolchain version is pinned in [`rust-toolchain.toml`](./rust-toolchain.toml). Extensions are WebAssembly-based; see the extension docs if you're building one rather than the editor itself.

### Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for ways you can contribute to Zed.

Also... we're hiring! Check out our [jobs](https://zed.dev/jobs) page for open roles.

### Community

For support and general discussion, use [Zed's community channels](https://discord.com/invite/zedindustries) rather than GitHub Issues (Issues are for bugs and feature requests).

### Licensing

Zed source code is licensed primarily under GPL-3.0-or-later, with Apache-2.0 components where marked. Server-side components are licensed under AGPL-3.0. See `LICENSE-GPL`, `LICENSE-APACHE`, and `LICENSE-AGPL` in the repo root, and check individual file headers when in doubt — this matters if you're redistributing or building a commercial product on top of Zed's code.

License information for third party dependencies must be correctly provided for CI to pass.

We use [`cargo-about`](https://github.com/EmbarkStudios/cargo-about) to automatically comply with open source licenses. If CI is failing, check the following:

- Is it showing a `no license specified` error for a crate you've created? If so, add `publish = false` under `[package]` in your crate's Cargo.toml.
- Is the error `failed to satisfy license requirements` for a dependency? If so, first determine what license the project has and whether this system is sufficient to comply with this license's requirements. If you're unsure, ask a lawyer. Once you've verified that this system is acceptable add the license's SPDX identifier to the `accepted` array in `script/licenses/zed-licenses.toml`.
- Is `cargo-about` unable to find the license for a dependency? If so, add a clarification field at the end of `script/licenses/zed-licenses.toml`, as specified in the [cargo-about book](https://embarkstudios.github.io/cargo-about/cli/generate/config.html#crate-configuration).

## Sponsorship

Zed is developed by **Zed Industries, Inc.**, a venture-backed, for-profit company. The editor is free and open source; the company's revenue comes primarily from paid Pro and Business plans, not from sponsorships.

If you'd like to financially support the project, you can do so via GitHub Sponsors.
Sponsorships go directly to Zed Industries and are used as general company revenue.
There are no perks or entitlements associated with sponsorship.
