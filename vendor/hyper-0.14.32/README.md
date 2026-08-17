# [hyper](https://hyper.rs)

[![crates.io](https://img.shields.io/crates/v/hyper.svg)](https://crates.io/crates/hyper)
[![Released API docs](https://docs.rs/hyper/badge.svg)](https://docs.rs/hyper)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![CI](https://github.com/hyperium/hyper/workflows/CI/badge.svg)](https://github.com/hyperium/hyper/actions?query=workflow%3ACI)
[![Discord chat][discord-badge]][discord-url]

A **fast** and **correct** HTTP implementation for Rust.

- HTTP/1 and HTTP/2
- Asynchronous design
- Leading in performance
- Tested and **correct**
- Extensive production use
- Client and Server APIs

**Get started** by looking over the [guides](https://hyper.rs/guides).

## "Low-level"

hyper is a relatively low-level library, meant to be a building block for
libraries and applications.

If you are looking for a convenient HTTP client, then you may wish to consider
[reqwest](https://github.com/seanmonstar/reqwest). If you are looking for a
convenient HTTP server, then you may wish to consider [Axum](https://github.com/tokio-rs/tokio).
Both are built on top of this library.

## Contributing

To get involved, take a look at [CONTRIBUTING](CONTRIBUTING.md).

If you prefer chatting, there is an active community in the [Discord server][discord-url].

## License

hyper is provided under the MIT license. See [LICENSE](LICENSE).

[discord-badge]: https://img.shields.io/discord/500028886025895936.svg?logo=discord
[discord-url]: https://discord.gg/kkwpueZ
