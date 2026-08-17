# RustCrypto: NIST P-256 (secp256r1) elliptic curve

[![crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![Build Status][build-image]][build-link]
![Apache2/MIT licensed][license-image]
![Rust Version][rustc-image]
[![Project Chat][chat-image]][chat-link]

Pure Rust implementation of the NIST P-256 (a.k.a. secp256r1, prime256v1)
elliptic curve with support for ECDH, ECDSA signing/verification, and general
purpose curve arithmetic support implemented in terms of traits from the
[`elliptic-curve`] crate.

[Documentation][docs-link]

## ⚠️ Security Warning

The elliptic curve arithmetic contained in this crate has never been
independently audited!

This crate has been designed with the goal of ensuring that secret-dependent
operations are performed in constant time (using the `subtle` crate and
constant-time formulas). However, it has not been thoroughly assessed to ensure
that generated assembly is constant time on common CPU architectures.

USE AT YOUR OWN RISK!

## Supported Algorithms

- [Elliptic Curve Diffie-Hellman (ECDH)][ECDH]: gated under the `ecdh` feature.
- [Elliptic Curve Digital Signature Algorithm (ECDSA)][ECDSA]: gated under the
  `ecdsa` feature.

## About NIST P-256

NIST P-256 is a Weierstrass curve specified in FIPS 186-4: Digital Signature
Standard (DSS):

<https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.186-4.pdf>

Also known as prime256v1 (ANSI X9.62) and secp256r1 (SECG), it's included in
the US National Security Agency's "Suite B" and is widely used in protocols
like TLS and the associated X.509 PKI.

## Minimum Supported Rust Version

Rust **1.57** or higher.

Minimum supported Rust version can be changed in the future, but it will be
done with a minor version bump.

## SemVer Policy

- All on-by-default features of this library are covered by SemVer
- MSRV is considered exempt from SemVer as noted above

## License

All crates licensed under either of

 * [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
 * [MIT license](http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[//]: # (badges)

[crate-image]: https://buildstats.info/crate/p256
[crate-link]: https://crates.io/crates/p256
[docs-image]: https://docs.rs/p256/badge.svg
[docs-link]: https://docs.rs/p256/
[build-image]: https://github.com/RustCrypto/elliptic-curves/actions/workflows/p256.yml/badge.svg
[build-link]: https://github.com/RustCrypto/elliptic-curves/actions/workflows/p256.yml
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[rustc-image]: https://img.shields.io/badge/rustc-1.57+-blue.svg
[chat-image]: https://img.shields.io/badge/zulip-join_chat-blue.svg
[chat-link]: https://rustcrypto.zulipchat.com/#narrow/stream/260040-elliptic-curves

[//]: # (general links)

[`elliptic-curve`]: https://github.com/RustCrypto/traits/tree/master/elliptic-curve
[ECDH]: https://en.wikipedia.org/wiki/Elliptic-curve_Diffie-Hellman
[ECDSA]: https://en.wikipedia.org/wiki/Elliptic_Curve_Digital_Signature_Algorithm
