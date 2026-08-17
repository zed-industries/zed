# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.10.2 (2023-04-04)
### Changed
- Bump `spki` to v0.7.1 ([#981])

[#981]: https://github.com/RustCrypto/formats/pull/981

## 0.10.1 (2023-03-05)
### Added
- `sha1-insecure` feature ([#913])

[#913]: https://github.com/RustCrypto/formats/pull/913

## 0.10.0 (2023-02-26) [YANKED]
### Changed
- Use blanket impls for `Decode*` traits ([#785])
- Bump `der` dependency to v0.7 ([#899])
- Bump `spki` dependency to v0.7 ([#900])
- Bump `pkcs5` dependency to v0.7 ([#901])

[#785]: https://github.com/RustCrypto/formats/pull/785
[#899]: https://github.com/RustCrypto/formats/pull/899
[#900]: https://github.com/RustCrypto/formats/pull/900
[#901]: https://github.com/RustCrypto/formats/pull/901

## 0.9.0 (2022-05-08)
### Added
- Error conversion support to `pkcs8::spki::Error` ([#335])
- Re-export `AssociatedOid` ([#645])

### Changed
- Use `finish_non_exhaustive` in `Debug` impls ([#245])
- Replace `PrivateKeyDocument` with `der::SecretDocument` ([#571])
- Bump `der` to v0.6 ([#653])
- Bump `spki` to v0.6 ([#654])
- Bump `pkcs5` to v0.5 ([#655])

### Removed
- `PrivateKeyDocument` ([#571])

[#245]: https://github.com/RustCrypto/formats/pull/245
[#335]: https://github.com/RustCrypto/formats/pull/335
[#571]: https://github.com/RustCrypto/formats/pull/571
[#645]: https://github.com/RustCrypto/formats/pull/645
[#653]: https://github.com/RustCrypto/formats/pull/653
[#654]: https://github.com/RustCrypto/formats/pull/654
[#655]: https://github.com/RustCrypto/formats/pull/655

## 0.8.0 (2021-11-16)
### Added
- Re-export `spki` crate ([#210])

### Changed
- Replace usages of `expect` with fallible methods ([#108])
- Impl `From*Key`/`To*Key` traits on `Document` types ([#110])
- Rename `From/ToPrivateKey` => `DecodePrivateKey`/`EncodePrivateKey` ([#121])
- Rust 2021 edition upgrade; MSRV 1.56 ([#136])
- Use `der::Document` to impl `*PrivateKeyDocument` ([#140])
- Rename `Error::Crypto` => `Error::EncryptedPrivateKey` ([#213], [#214])
- Bump `der` dependency to v0.5 ([#222])
- Bump `spki` dependency to v0.5 ([#223])
- Bump `pkcs5` dependency to v0.4 ([#224])
- Replace `from_pkcs8_private_key_info` with `TryFrom` ([#230])

### Removed
- `*_with_le` PEM encoding methods ([#109])
- PKCS#1 support; moved to `pkcs1` crate ([#124])
- I/O related errors from key format crates ([#158])
- `der::pem` export ([#211])

[#108]: https://github.com/RustCrypto/formats/pull/108
[#109]: https://github.com/RustCrypto/formats/pull/109
[#110]: https://github.com/RustCrypto/formats/pull/110
[#121]: https://github.com/RustCrypto/formats/pull/121
[#124]: https://github.com/RustCrypto/formats/pull/124
[#136]: https://github.com/RustCrypto/formats/pull/136
[#140]: https://github.com/RustCrypto/formats/pull/140
[#158]: https://github.com/RustCrypto/formats/pull/158
[#210]: https://github.com/RustCrypto/formats/pull/210
[#211]: https://github.com/RustCrypto/formats/pull/211
[#213]: https://github.com/RustCrypto/formats/pull/213
[#214]: https://github.com/RustCrypto/formats/pull/214
[#222]: https://github.com/RustCrypto/formats/pull/222
[#223]: https://github.com/RustCrypto/formats/pull/223
[#224]: https://github.com/RustCrypto/formats/pull/224
[#230]: https://github.com/RustCrypto/formats/pull/230

## 0.7.6 (2021-09-14)
### Added
- `3des` and `des-insecure` features
- `sha1` feature
- Support for AES-192-CBC

### Changed
- Moved to `formats` repo ([#2])

[#2]: https://github.com/RustCrypto/formats/pull/2

## 0.7.5 (2021-07-26)
### Added
- Support for customizing PEM `LineEnding`

### Changed
- Bump `pem-rfc7468` dependency to v0.2

## 0.7.4 (2021-07-25)
### Added
- PKCS#1 support

## 0.7.3 (2021-07-24)
### Changed
- Use `pem-rfc7468` crate

## 0.7.2 (2021-07-20)
### Added
- `Error::ParametersMalformed` variant

## 0.7.1 (2021-07-20)
### Added
- `Error::KeyMalformed` variant

## 0.7.0 (2021-06-07)
### Added
- ASN.1 error improvements

### Changed
- Merge `OneAsymmetricKey` into `PrivateKeyInfo`
- Use scrypt as the default PBES2 KDF
- Return `Result`(s) when encoding 
- Bump `der` to v0.4
- Bump `spki` to v0.4
- Bump `pkcs5` to v0.3

## 0.6.1 (2021-05-24)
### Added
- Support for RFC5958's `OneAsymmetricKey`

### Changed
- Bump `der` to v0.3.5

## 0.6.0 (2021-03-22)
### Changed
- Bump `der` dependency to v0.3
- Bump `spki` dependency to v0.3
- Bump `pkcs5` dependency to v0.2

## 0.5.5 (2021-03-17)
### Changed
- Bump `base64ct` dependency to v1.0

## 0.5.4 (2021-02-24)
### Added
- Encryption helper methods for `FromPrivateKey`/`ToPrivateKey`

## 0.5.3 (2021-02-23)
### Added
- Support for decrypting/encrypting `EncryptedPrivateKeyInfo`
- PEM support for `EncryptedPrivateKeyInfo`
- `Error::Crypto` variant

## 0.5.2 (2021-02-20)
### Changed
- Use `pkcs5` crate

## 0.5.1 (2021-02-18) [YANKED]
### Added
- `pkcs5` feature

### Changed
- Bump `spki` dependency to v0.2.0

## 0.5.0 (2021-02-16) [YANKED]
### Added
- Initial `EncryptedPrivateKeyInfo` support

### Changed
- Extract SPKI-related types into the `spki` crate

## 0.4.1 (2021-02-01)
### Changed
- Bump `basec4ct` dependency to v0.2

## 0.4.0 (2021-01-26)
### Changed
- Bump `der` crate dependency to v0.2
- Use `base64ct` v0.1 for PEM encoding

## 0.3.3 (2020-12-21)
### Changed
- Use `der` crate for decoding/encoding ASN.1 DER

## 0.3.2 (2020-12-16)
### Added
- `AlgorithmIdentifier::parameters_oid` method

## 0.3.1 (2020-12-16)
### Changed
- Bump `const-oid` dependency to v0.4

## 0.3.0 (2020-12-16) [YANKED]
### Added
- `AlgorithmParameters` enum

## 0.2.2 (2020-12-14)
### Fixed
- Decoding/encoding support for Ed25519 keys

## 0.2.1 (2020-12-14)
### Added
- rustdoc improvements

## 0.2.0 (2020-12-14)
### Added
- File writing methods for public/private keys
- Methods for loading `*Document` types from files
- DER encoding support
- PEM encoding support
- `ToPrivateKey`/`ToPublicKey` traits

### Changed
- `Error` enum
- Rename `load_*_file` methods to `read_*_file`

## 0.1.1 (2020-12-06)
### Added
- Helper methods to load keys from the local filesystem

## 0.1.0 (2020-12-05)
- Initial release
