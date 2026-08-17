# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.7.5 (2023-04-24)
### Fixed
- Import failure ([#1021])

[#1021]: https://github.com/RustCrypto/formats/pull/1021

## 0.7.4 (2023-04-21)
### Changed
- Have `alloc` feature only weakly activate `pkcs8?/alloc` ([#1013])
- Have `pem` feature only weakly activate `pkcs8?/pem` ([#1013])

[#1013]: https://github.com/RustCrypto/formats/pull/1013

## 0.7.3 (2023-04-18)
### Added
- Provide functions to construct `RsaPss` and `RsaOaepParams` ([#1010])

### Changed
- Use `NULL` parameters for SHA `AlgorithmIdentifier`s ([#1010])

[#1010]: https://github.com/RustCrypto/formats/pull/1010

## 0.7.2 (2023-04-04)
### Added
- `RsaPssParams::SALT_LEN_DEFAULT` ([#953])

[#953]: https://github.com/RustCrypto/formats/pull/953

## 0.7.1 (2023-03-05)
### Fixed
- `DecodeRsaPublicKey` blanket impl ([#916])

[#916]: https://github.com/RustCrypto/formats/pull/916

## 0.7.0 (2023-02-26) [YANKED]
### Changed
- Make PSS/OAEP params use generic `AlgorithmIdentifier` ([#799])
- Bump `der` dependency to v0.7 ([#899])
- Bump `spki` dependency to v0.7 ([#900])
- Bump `pkcs8` to v0.10 ([#902])

[#799]: https://github.com/RustCrypto/formats/pull/799
[#899]: https://github.com/RustCrypto/formats/pull/899
[#900]: https://github.com/RustCrypto/formats/pull/900
[#902]: https://github.com/RustCrypto/formats/pull/902

## 0.6.0 (Skipped)
- Skipped to synchronize version number with `der` and `spki`

## 0.5.0 (Skipped)
- Skipped to synchronize version number with `der` and `spki`

## 0.4.1 (2022-10-10)
### Added
- `RsaPssParams` support ([#698])
- `RsaOaepParams` support ([#733])

[#698]: https://github.com/RustCrypto/formats/pull/698
[#733]: https://github.com/RustCrypto/formats/pull/733

## 0.4.0 (2022-05-08)
### Changed
- Replace document types with `doc::{Document, SecretDocument}` types ([#571])
- Bump `der` to v0.6 ([#653])
- Bump `pkcs8` to v0.9 ([#656])

[#571]: https://github.com/RustCrypto/formats/pull/571
[#653]: https://github.com/RustCrypto/formats/pull/653
[#656]: https://github.com/RustCrypto/formats/pull/656

## 0.3.3 (2022-01-16)
### Added
- Error conversion support to `pkcs8::spki::Error` ([#333])

[#333]: https://github.com/RustCrypto/formats/pull/331

## 0.3.2 (2022-01-16)
### Added
- Error conversion support to `pkcs8::Error` ([#331])

[#331]: https://github.com/RustCrypto/formats/pull/331

## 0.3.1 (2021-11-29)
### Changed
- Use `finish_non_exhaustive` in Debug impls ([#245])

[#245]: https://github.com/RustCrypto/formats/pull/245

## 0.3.0 (2021-11-17)
### Added
- Support for multi-prime RSA keys ([#115])
- `pkcs8` feature ([#227], [#233])

### Changed
- Rename `From/ToRsa*Key` => `DecodeRsa*Key`/`EncodeRsa*Key` ([#120])
- Use `der::Document` to impl `RsaPrivateKeyDocument` ([#131])
- Rust 2021 edition upgrade; MSRV 1.56 ([#136])
- Make `RsaPrivateKey::version` implicit ([#188])
- Bump `der` crate dependency to v0.5 ([#222])
- Activate `pkcs8/pem` when `pem` feature is enabled ([#232])

### Removed
- `*_with_le` PEM encoding methods ([#109])
- I/O related errors ([#158])

[#109]: https://github.com/RustCrypto/formats/pull/109
[#115]: https://github.com/RustCrypto/formats/pull/115
[#120]: https://github.com/RustCrypto/formats/pull/120
[#131]: https://github.com/RustCrypto/formats/pull/131
[#136]: https://github.com/RustCrypto/formats/pull/136
[#158]: https://github.com/RustCrypto/formats/pull/158
[#188]: https://github.com/RustCrypto/formats/pull/188
[#222]: https://github.com/RustCrypto/formats/pull/222
[#227]: https://github.com/RustCrypto/formats/pull/227
[#232]: https://github.com/RustCrypto/formats/pull/232
[#233]: https://github.com/RustCrypto/formats/pull/233

## 0.2.4 (2021-09-14)
### Changed
- Moved to `formats` repo ([#2])

[#2]: https://github.com/RustCrypto/formats/pull/2

## 0.2.3 (2021-07-26)
### Added
- Support for customizing PEM `LineEnding`

### Changed
- Bump `pem-rfc7468` dependency to v0.2

## 0.2.2 (2021-07-25)
### Fixed
- `Version` encoder

## 0.2.1 (2021-07-25)
### Added
- `Error::Crypto` variant

## 0.2.0 (2021-07-25)
### Added
- `From*`/`To*` traits for `RsaPrivateKey`/`RsaPublicKey`

### Changed
- Use `FromRsa*`/`ToRsa*` traits with `*Document` types

## 0.1.1 (2021-07-24)
### Added
- Re-export `der` crate and `der::UIntBytes`

### Changed
- Replace `Error::{Decode, Encode}` with `Error::Asn1`

## 0.1.0 (2021-07-24) [YANKED]
- Initial release
