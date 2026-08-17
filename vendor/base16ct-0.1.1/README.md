# [RustCrypto]: Constant-Time Base16 (hexadecimal)

[![crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![Build Status][build-image]][build-link]
![Apache2/MIT licensed][license-image]
![Rust Version][rustc-image]
[![Project Chat][chat-image]][chat-link]

Pure Rust implementation of Base16 ([RFC 4648]).

Implements lower and upper case Base16 variants without data-dependent branches
or lookup  tables, thereby providing portable "best effort" constant-time
operation.

Supports `no_std` environments and avoids heap allocations in the core API
(but also provides optional `alloc` support for convenience).

[Documentation][docs-link]

## Minimum Supported Rust Version

This crate requires **Rust 1.56** at a minimum.

We may change the MSRV in the future, but it will be accompanied by a minor
version bump.

## License

Licensed under either of:

 * [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
 * [MIT license](http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/base16ct.svg
[crate-link]: https://crates.io/crates/base16ct
[docs-image]: https://docs.rs/base16ct/badge.svg
[docs-link]: https://docs.rs/base16ct/
[build-image]: https://github.com/RustCrypto/formats/actions/workflows/base16ct.yml/badge.svg
[build-link]: https://github.com/RustCrypto/formats/actions/workflows/base16ct.yml
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[rustc-image]: https://img.shields.io/badge/rustc-1.56+-blue.svg
[chat-image]: https://img.shields.io/badge/zulip-join_chat-blue.svg
[chat-link]: https://rustcrypto.zulipchat.com/#narrow/stream/300570-formats

[//]: # (links)

[RustCrypto]: https://github.com/rustcrypto
[RFC 4648]: https://tools.ietf.org/html/rfc4648
[Util::Lookup]: https://arxiv.org/pdf/2108.04600.pdf
