# [RustCrypto]: SEC1 Elliptic Curve Cryptography Formats

[![crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![Build Status][build-image]][build-link]
![Apache2/MIT licensed][license-image]
![Rust Version][rustc-image]
[![Project Chat][chat-image]][chat-link]

[Documentation][docs-link]

## About

Pure Rust implementation of [SEC1: Elliptic Curve Cryptography] encoding
formats including ASN.1 DER-serialized private keys (also described in
[RFC5915]) as well as the `Elliptic-Curve-Point-to-Octet-String` and
`Octet-String-to-Elliptic-Curve-Point` encoding algorithms.

## Minimum Supported Rust Version

This crate requires **Rust 1.57** at a minimum.

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

[crate-image]: https://buildstats.info/crate/sec1
[crate-link]: https://crates.io/crates/sec1
[docs-image]: https://docs.rs/sec1/badge.svg
[docs-link]: https://docs.rs/sec1/
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[rustc-image]: https://img.shields.io/badge/rustc-1.57+-blue.svg
[chat-image]: https://img.shields.io/badge/zulip-join_chat-blue.svg
[chat-link]: https://rustcrypto.zulipchat.com/#narrow/stream/300570-formats
[build-image]: https://github.com/RustCrypto/formats/workflows/sec1/badge.svg?branch=master&event=push
[build-link]: https://github.com/RustCrypto/formats/actions

[//]: # (links)

[RustCrypto]: https://github.com/rustcrypto
[SEC1: Elliptic Curve Cryptography]: https://www.secg.org/sec1-v2.pdf
[RFC5915]: https://datatracker.ietf.org/doc/html/rfc5915
