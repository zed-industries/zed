# [RustCrypto]: PKCS#1 (RSA)

[![crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![Build Status][build-image]][build-link]
![Apache2/MIT licensed][license-image]
![Rust Version][rustc-image]
[![Project Chat][chat-image]][chat-link]

Pure Rust implementation of Public-Key Cryptography Standards (PKCS) #1:
RSA Cryptography Specifications Version 2.2 ([RFC 8017]).

[Documentation][docs-link]

## About

This crate supports encoding and decoding RSA private and public keys
in either PKCS#1 DER (binary) or PEM (text) formats.

PEM encoded RSA private keys begin with:

```text
-----BEGIN RSA PRIVATE KEY-----
```

PEM encoded RSA public keys begin with:

```text
-----BEGIN RSA PUBLIC KEY-----
```

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

[crate-image]: https://buildstats.info/crate/pkcs1
[crate-link]: https://crates.io/crates/pkcs1
[docs-image]: https://docs.rs/pkcs1/badge.svg
[docs-link]: https://docs.rs/pkcs1/
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[rustc-image]: https://img.shields.io/badge/rustc-1.65+-blue.svg
[chat-image]: https://img.shields.io/badge/zulip-join_chat-blue.svg
[chat-link]: https://rustcrypto.zulipchat.com/#narrow/stream/300570-formats
[build-image]: https://github.com/RustCrypto/formats/workflows/pkcs1/badge.svg?branch=master&event=push
[build-link]: https://github.com/RustCrypto/formats/actions

[//]: # (links)

[RustCrypto]: https://github.com/rustcrypto
[RFC 8017]: https://datatracker.ietf.org/doc/html/rfc8017
