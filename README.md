# Zed

[![CI](https://github.com/zed-industries/zed/actions/workflows/ci.yml/badge.svg)](https://github.com/zed-industries/zed/actions/workflows/ci.yml)

Welcome to Zed, a high-performance, multiplayer code editor from the creators of [Atom](https://github.com/atom/atom) and [Tree-sitter](https://github.com/tree-sitter/tree-sitter).

## Installation

You can [download](https://zed.dev/download) Zed today for macOS (v10.15+).

Support for additional platforms is on our [roadmap](https://zed.dev/roadmap):

- Linux ([tracking issue](https://github.com/zed-industries/zed/issues/7015))
- Windows ([tracking issue](https://github.com/zed-industries/zed/issues/5394))
- Web ([tracking issue](https://github.com/zed-industries/zed/issues/5396))

For macOS users, you can also install Zed from Homebrew:
```sh
brew install zed
```

## Developing Zed

- [Building Zed for macOS](./docs/src/developing_zed__building_zed_macos.md)
- [Building Zed for Linux](./docs/src/developing_zed__building_zed_linux.md)
- [Building Zed for Windows](./docs/src/developing_zed__building_zed_windows.md)
- [Running Collaboration Locally](./docs/src/developing_zed__local_collaboration.md)

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for ways you can contribute to Zed.

## Licensing

License information for third party dependencies must be correctly provided for CI to pass.

We use [`cargo-about`](https://github.com/EmbarkStudios/cargo-about) to automatically comply with open source licenses. If CI is failing, check the following:

- Is it showing a `no license specified` error for a crate you've created? If so, add `publish = false` under `[package]` in your crate's Cargo.toml.
- Is the error `failed to satisfy license requirements` for a dependency? If so, first determine what license the project has and whether this system is sufficient to comply with this license's requirements. If you're unsure, ask a lawyer. Once you've verified that this system is acceptable add the license's SPDX identifier to the `accepted` array in `script/licenses/zed-licenses.toml`.
- Is `cargo-about` unable to find the license for a dependency? If so, add a clarification field at the end of `script/licenses/zed-licenses.toml`, as specified in the [cargo-about book](https://embarkstudios.github.io/cargo-about/cli/generate/config.html#crate-configuration).
