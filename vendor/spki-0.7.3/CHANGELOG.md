# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.7.3 (2023-11-28)
### Added
- public key to `SubjectPublicKeyInfoOwned` helper ([#1269])

[#1269]: https://github.com/RustCrypto/formats/pull/1269

## 0.7.2 (2023-05-04)

### Added
- `AlgorithmIdentifierWithOid` trait ([#986])
- `SignatureBitStringEncoding` trait ([#1047])

### Changed
-  Bump `hex-literal` to v0.4.1 ([#999])

[#986]: https://github.com/RustCrypto/formats/pull/986
[#999]: https://github.com/RustCrypto/formats/pull/999
[#1047]: https://github.com/RustCrypto/formats/pull/1047


## 0.7.1 (2023-04-04)
### Added
- `AssociatedAlgorithmIdentifier` trait ([#962], [#966])
- `DynAssociatedAlgorithmIdentifier` trait ([#962])
- `SignatureAlgorithmIdentifier` trait ([#967])
- `DynSignatureAlgorithmIdentifier` trait ([#967])

### Changed
- Bump `der` dependency to v0.7.2 ([#979])

[#962]: https://github.com/RustCrypto/formats/pull/962
[#966]: https://github.com/RustCrypto/formats/pull/966
[#967]: https://github.com/RustCrypto/formats/pull/967
[#979]: https://github.com/RustCrypto/formats/pull/979

## 0.7.0 (2023-02-26)
### Changed
- Make `AlgorithmIdentifier` generic around `Params` ([#769])
- Use blanket impls for `Decode*` traits ([#785])
- Make `SubjectPublicKeyInfo` own the public key ([#790])
- Rename `to_owned` method ([#835])
- Bump `der` dependency to v0.7 ([#899])

[#769]: https://github.com/RustCrypto/formats/pull/769
[#785]: https://github.com/RustCrypto/formats/pull/785
[#790]: https://github.com/RustCrypto/formats/pull/790
[#835]: https://github.com/RustCrypto/formats/pull/835
[#899]: https://github.com/RustCrypto/formats/pull/899

## 0.6.0 (2022-05-08)
### Added
- `AlgorithmIdentifier::oids()` helper function ([#443])
- Impl `PartialOrd` for `AlgorithmIdentifier` ([#476])
- Impl `DecodeValue` for `AlgorithmIdentifier` ([#449])
- Impl `ValueOrd` for `SubjectPublicKeyInfo` ([#522])

### Changed
- Replace `PublicKeyDocument` with `der` crate's `Document` type ([#571])
- Streaming fingerprint builder ([#616])
- Bump `der` crate dependency to v0.6 ([#653])

### Removed
- `PublicKeyDocument` ([#571])

[#443]: https://github.com/RustCrypto/formats/pull/443
[#449]: https://github.com/RustCrypto/formats/pull/449
[#476]: https://github.com/RustCrypto/formats/pull/476
[#522]: https://github.com/RustCrypto/formats/pull/522
[#571]: https://github.com/RustCrypto/formats/pull/571
[#616]: https://github.com/RustCrypto/formats/pull/616
[#653]: https://github.com/RustCrypto/formats/pull/653

## 0.5.4 (2022-01-05)
### Added
- `Error::KeyMalformed` variant ([#318])

[#318]: https://github.com/RustCrypto/formats/pull/318

## 0.5.3 (2021-12-19)
### Added
- Impl `ValueOrd` for `AlgorithmIdentifier` ([#289])

[#289]: https://github.com/RustCrypto/formats/pull/289

## 0.5.2 (2021-11-17)
### Changed
- Relax `base64ct` version requirement to `^1` ([#239])

[#239]: https://github.com/RustCrypto/formats/pull/239

## 0.5.1 (2021-11-17)
### Changed
- Replace `from_spki` with `TryFrom` ([#231])

[#231]: https://github.com/RustCrypto/formats/pull/231

## 0.5.0 (2021-11-15) [YANKED]
### Added
- SPKI fingerprint support ([#36])
- `PublicKeyDocument` type originally from `pkcs8` crate ([#118])
- `Error` type ([#143])

### Changed
- Rename `From/ToPublicKey` => `DecodePublicKey`/`EncodePublicKey` ([#119])
- Use `der::Document` to impl `PublicKeyDocument` ([#134])
- Rust 2021 edition upgrade; MSRV 1.56 ([#136])
- Bump `der` dependency to v0.5 ([#222])

[#36]: https://github.com/RustCrypto/formats/pull/36
[#118]: https://github.com/RustCrypto/formats/pull/118
[#119]: https://github.com/RustCrypto/formats/pull/119
[#134]: https://github.com/RustCrypto/formats/pull/134
[#136]: https://github.com/RustCrypto/formats/pull/136
[#143]: https://github.com/RustCrypto/formats/pull/143
[#222]: https://github.com/RustCrypto/formats/pull/222

## 0.4.1 (2021-09-14)
### Changed
- Moved to `formats` repo ([#2])

[#2]: https://github.com/RustCrypto/formats/pull/2

## 0.4.0 (2021-06-07)
### Added
- `AlgorithmIdentifier::assert_oids`

### Changed
- Bump `der` to v0.4

## 0.3.0 (2021-03-22)
### Changed
- Bump `der` to v0.3

### Removed
- `AlgorithmParameters` enum

## 0.2.1 (2021-02-22)
### Added
- Impl `Choice` for `AlgorithmParameters`

## 0.2.0 (2021-02-18)
### Changed
- Return `Result` from `AlgorithmIdentifier::params_*`

## 0.1.0 (2021-02-16)
- Initial release
