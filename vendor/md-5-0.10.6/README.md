# RustCrypto: MD5

[![crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
![Apache2/MIT licensed][license-image]
![Rust Version][rustc-image]
[![Project Chat][chat-image]][chat-link]
[![Build Status][build-image]][build-link]

Pure Rust implementation of the [MD5 hash function][1].

[Documentation][docs-link]

## ⚠️ Security Warning

This crate is provided for the purposes of legacy interoperability with
protocols and systems which mandate the use of MD5.

However, MD5 is [cryptographically broken and unsuitable for further use][2].

Collision attacks against MD5 are both practical and trivial, and
[theoretical attacks against MD5's preimage resistance have been found][3].

[RFC6151][4] advises no new IETF protocols can be designed MD5-based constructions,
including HMAC-MD5.

## Minimum Supported Rust Version

Rust **1.41** or higher.

Minimum supported Rust version can be changed in the future, but it will be
done with a minor version bump.

## SemVer Policy

- All on-by-default features of this library are covered by SemVer
- MSRV is considered exempt from SemVer as noted above

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

[crate-image]: https://img.shields.io/crates/v/md-5.svg
[crate-link]: https://crates.io/crates/md-5
[docs-image]: https://docs.rs/md-5/badge.svg
[docs-link]: https://docs.rs/md-5/
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[rustc-image]: https://img.shields.io/badge/rustc-1.41+-blue.svg
[chat-image]: https://img.shields.io/badge/zulip-join_chat-blue.svg
[chat-link]: https://rustcrypto.zulipchat.com/#narrow/stream/260041-hashes
[build-image]: https://github.com/RustCrypto/hashes/workflows/md5/badge.svg?branch=master
[build-link]: https://github.com/RustCrypto/hashes/actions?query=workflow%3Amd5

[//]: # (general links)

[1]: https://en.wikipedia.org/wiki/MD5
[2]: https://www.kb.cert.org/vuls/id/836068
[3]: https://dl.acm.org/citation.cfm?id=1724151
[4]: https://tools.ietf.org/html/rfc6151
