# [RustCrypto]: PEM Encoding ([RFC 7468])

[![crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![Build Status][build-image]][build-link]
![Apache2/MIT licensed][license-image]
![Rust Version][rustc-image]
[![Project Chat][chat-image]][chat-link]

Pure Rust implementation of PEM Encoding ([RFC 7468]) for PKIX, PKCS, and
CMS Structures, a strict subset of the original Privacy-Enhanced Mail encoding
intended  specifically for use with cryptographic keys, certificates, and other
messages.

Provides a `no_std`-friendly, constant-time implementation suitable for use with
cryptographic private keys.

[Documentation][docs-link]

## About

Many cryptography-related document formats, such as certificates (PKIX),
private and public keys/keypairs (PKCS), and other cryptographic messages (CMS)
provide an ASCII encoding which can be traced back to Privacy-Enhanced Mail
(PEM) as defined [RFC 1421], which look like the following:

```text
-----BEGIN PRIVATE KEY-----
MC4CAQAwBQYDK2VwBCIEIBftnHPp22SewYmmEoMcX8VwI4IHwaqd+9LFPj/15eqF
-----END PRIVATE KEY-----
```

However, all of these formats actually implement a text-based encoding that is
similar but *not* identical to the legacy PEM encoding as described in
[RFC 1421].

For this reason, [RFC 7468] was created to describe a stricter form of
"PEM encoding" for use in these applications which codifies the previously
de facto rules that most implementations operate by, and makes recommendations
to promote interoperability.

This crate provides a strict interpretation of the [RFC 7468] rules,
implementing MUSTs and SHOULDs while avoiding the MAYs, targeting the
"ABNF (Strict)" subset of the grammar as described in
[RFC 7468 Section 3 Figure 3 (p6)][RFC 7468 p6].

## Implementation notes

- `no_std`-friendly core implementation which requires no heap allocations
  and avoids copies and temporary buffers.
- Optional `alloc`-dependent convenience features and buffered decoder/encoder.
- Uses the [`base64ct`] crate to decode/encode Base64 in constant-time.
- PEM parser avoids branching on potentially secret data as much as possible.

The paper [Util::Lookup: Exploiting key decoding in cryptographic libraries][Util::Lookup]
demonstrates how the leakage from non-constant-time PEM parsers can be used
to practically extract RSA private keys from SGX enclaves.

## Minimum Supported Rust Version

This crate requires **Rust 1.60** at a minimum.

We may change the MSRV in the future, but it will be accompanied by a minor
version bump.

## License

Licensed under either of:

- [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
- [MIT license](http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[//]: # (badges)

[crate-image]: https://buildstats.info/crate/pem-rfc7468
[crate-link]: https://crates.io/crates/pem-rfc7468
[docs-image]: https://docs.rs/pem-rfc7468/badge.svg
[docs-link]: https://docs.rs/pem-rfc7468/
[build-image]: https://github.com/RustCrypto/formats/actions/workflows/pem-rfc7468.yml/badge.svg
[build-link]: https://github.com/RustCrypto/formats/actions/workflows/pem-rfc7468.yml
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[rustc-image]: https://img.shields.io/badge/rustc-1.60+-blue.svg
[chat-image]: https://img.shields.io/badge/zulip-join_chat-blue.svg
[chat-link]: https://rustcrypto.zulipchat.com/#narrow/stream/300570-formats

[//]: # (links)

[RustCrypto]: https://github.com/rustcrypto
[RFC 1421]: https://datatracker.ietf.org/doc/html/rfc1421
[RFC 7468]: https://datatracker.ietf.org/doc/html/rfc7468
[RFC 7468 p6]: https://datatracker.ietf.org/doc/html/rfc7468#page-6
[`base64ct`]: https://github.com/RustCrypto/formats/tree/master/base64ct
[Util::Lookup]: https://arxiv.org/pdf/2108.04600.pdf
