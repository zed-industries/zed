# [RustCrypto]: X.509 Subject Public Key Info (SPKI)

[![crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![Build Status][build-image]][build-link]
![Apache2/MIT licensed][license-image]
![Rust Version][rustc-image]
[![Project Chat][chat-image]][chat-link]

[X.509] Subject Public Key Info types describing public keys as well as their
associated AlgorithmIdentifiers (i.e. OIDs).

Specified in [RFC 5280 § 4.1].

[Documentation][docs-link]

## Minimum Supported Rust Version

This crate requires **Rust 1.65** at a minimum.

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

[crate-image]: https://buildstats.info/crate/spki
[crate-link]: https://crates.io/crates/spki
[docs-image]: https://docs.rs/spki/badge.svg
[docs-link]: https://docs.rs/spki/
[build-image]: https://github.com/RustCrypto/formats/actions/workflows/spki.yml/badge.svg
[build-link]: https://github.com/RustCrypto/formats/actions/workflows/spki.yml
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[rustc-image]: https://img.shields.io/badge/rustc-1.65+-blue.svg
[chat-image]: https://img.shields.io/badge/zulip-join_chat-blue.svg
[chat-link]: https://rustcrypto.zulipchat.com/#narrow/stream/300570-formats

[//]: # (links)

[RustCrypto]: https://github.com/rustcrypto
[X.509]: https://en.wikipedia.org/wiki/X.509
[RFC 5280 § 4.1]: https://tools.ietf.org/html/rfc5280#section-4.1
