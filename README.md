# Editor

[![CI](https://github.com/the-code-editor-company/editor/actions/workflows/ci.yml/badge.svg)](https://github.com/the-code-editor-company/editor/actions/workflows/ci.yml)

Welcome to Editor, a high-performance, multiplayer code editor written in Rust.

---

### Installation

We're currently working on our fork so downloads are not yet available. You can follow the instructions below to build Editor from source.

### Developing Editor

- [Building Editor for macOS](./docs/src/development/macos.md)
- [Building Editor for Linux](./docs/src/development/linux.md)
- [Building Editor for Windows](./docs/src/development/windows.md)
- [Running Collaboration Locally](./docs/src/development/local-collaboration.md)

### Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for ways you can contribute to Editor.

### Licensing

License information for third party dependencies must be correctly provided for CI to pass.

We use [`cargo-about`](https://github.com/EmbarkStudios/cargo-about) to automatically comply with open source licenses. If CI is failing, check the following:

- Is it showing a `no license specified` error for a crate you've created? If so, add `publish = false` under `[package]` in your crate's Cargo.toml.
- Is the error `failed to satisfy license requirements` for a dependency? If so, first determine what license the project has and whether this system is sufficient to comply with this license's requirements. If you're unsure, ask a lawyer. Once you've verified that this system is acceptable add the license's SPDX identifier to the `accepted` array in `script/licenses/editor-licenses.toml`.
- Is `cargo-about` unable to find the license for a dependency? If so, add a clarification field at the end of `script/licenses/editor-licenses.toml`, as specified in the [cargo-about book](https://embarkstudios.github.io/cargo-about/cli/generate/config.html#crate-configuration).
