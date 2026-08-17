<!-- Allow this file to not have a first line heading -->
<!-- markdownlint-disable-file MD041 -->

<!-- inline html -->
<!-- markdownlint-disable-file MD033 MD036 -->

<div align="center">

# `🔥 minidumper`

**IPC implementation for creating a minidump for a crashed process**

[![Embark](https://img.shields.io/badge/embark-open%20source-blueviolet.svg)](https://embark.dev)
[![Embark](https://img.shields.io/badge/discord-ark-%237289da.svg?logo=discord)](https://discord.gg/dAuKfZS)
[![Crates.io](https://img.shields.io/crates/v/minidumper.svg)](https://crates.io/crates/minidumper)
[![Docs](https://docs.rs/minidumper/badge.svg)](https://docs.rs/minidumper)
[![dependency status](https://deps.rs/repo/github/EmbarkStudios/minidumper/status.svg)](https://deps.rs/repo/github/EmbarkStudios/minidumper)
[![Build status](https://github.com/EmbarkStudios/crash-handling/workflows/CI/badge.svg)](https://github.com/EmbarkStudios/crash-handling/actions)

</div>

This crate supplies a client and server IPC implementation for communicating between a process that _may_ crash (client) and a monitor (server) process.

The client can communicate application-specific state via [`Client::send_message`], and, if a crash occurs, can use [`Client::request_dump`] to request a minidump be created. The [`Server`] uses a user implemented [`ServerHandler`] to handle the messages sent by the client, and provides a way to create the minidump file where a requested crash can be written to, as well as a callback when a minidump is finished writing (both on failure and success) to perform whatever additional steps make sense for the application, such as transmission of the minidump to an external HTTP service for processing or the like.

## Contribution

[![Contributor Covenant](https://img.shields.io/badge/contributor%20covenant-v1.4-ff69b4.svg)](../CODE_OF_CONDUCT.md)

We welcome community contributions to this project.

Please read our [Contributor Guide](../CONTRIBUTING.md) for more information on how to get started.
Please also read our [Contributor Terms](../CONTRIBUTING.md#contributor-terms) before you make any contributions.

Any contribution intentionally submitted for inclusion in an Embark Studios project, shall comply with the Rust standard licensing model (MIT OR Apache 2.0) and therefore be dual licensed as described below, without any additional terms or conditions:

### License

This contribution is dual licensed under EITHER OF

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

For clarity, "your" refers to Embark or any other licensee/user of the contribution.
