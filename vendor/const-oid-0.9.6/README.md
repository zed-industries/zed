# [RustCrypto]: Object Identifiers (OIDs)

[![crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![Build Status][build-image]][build-link]
![Apache2/MIT licensed][license-image]
![Rust Version][rustc-image]
[![Project Chat][chat-image]][chat-link]

Const-friendly implementation of the ISO/IEC Object Identifier (OID) standard
as defined in ITU [X.660], with support for BER/DER encoding/decoding as well
as heapless `no_std` (i.e. embedded) environments.

[Documentation][docs-link]

## About OIDs

Object Identifiers, a.k.a. OIDs, are an International Telecommunications
Union (ITU) and ISO/IEC standard for naming any object, concept, or "thing"
with a globally unambiguous persistent name.

The ITU's [X.660] standard provides the OID specification. Every OID is part of
a hierarchical namespace which begins with a *root OID*, which is either the
ITU's root OID (0), the ISO's root OID (1), or the joint ISO/ITU root OID (2).

The following is an example of an OID, in this case identifying the
`rsaEncryption` algorithm:

```text
1.2.840.113549.1.1.1
```

For more information, see: <https://en.wikipedia.org/wiki/Object_identifier>

## Implementation

This library supports parsing OIDs in const contexts, e.g.:

```rust
use const_oid::ObjectIdentifier;

pub const MY_OID: ObjectIdentifier = ObjectIdentifier::new_unwrap("1.2.840.113549.1.1.1");
```

The OID parser is implemented entirely in terms of `const fn` and without the
use of proc macros.

Additionally, it also includes a `const fn` OID serializer, and stores the OIDs
parsed from const contexts encoded using the BER/DER serialization
(sans header).

This allows `ObjectIdentifier` to impl `AsRef<[u8]>` which can be used to
obtain the BER/DER serialization of an OID, even one declared `const`.

Additionally, it impls `FromStr` and `TryFrom<&[u8]>` and functions just as
well as a runtime OID library.

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

[crate-image]: https://buildstats.info/crate/const-oid
[crate-link]: https://crates.io/crates/const-oid
[docs-image]: https://docs.rs/const-oid/badge.svg
[docs-link]: https://docs.rs/const-oid/
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[rustc-image]: https://img.shields.io/badge/rustc-1.57+-blue.svg
[chat-image]: https://img.shields.io/badge/zulip-join_chat-blue.svg
[chat-link]: https://rustcrypto.zulipchat.com/#narrow/stream/300570-formats
[build-image]: https://github.com/RustCrypto/formats/workflows/const-oid/badge.svg?branch=master&event=push
[build-link]: https://github.com/RustCrypto/formats/actions

[//]: # (links)

[RustCrypto]: https://github.com/rustcrypto
[X.660]: https://www.itu.int/rec/T-REC-X.660
