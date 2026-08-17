# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.9.10 (2026-01-06)
### Fixed
- do not panic on a prime being 1 when loading a secret key ([#624])

[#624]: https://github.com/RustCrypto/RSA/pull/624


## 0.9.9 (2025-11-13)
### Fixed
- Support for cryptographic operations with larger keys ([#594])

[#594]: https://github.com/RustCrypto/RSA/pull/594

## 0.9.8 (2025-03-12)
### Added
- Doc comments to specify the `rand` version ([#473])

[#473]: https://github.com/RustCrypto/RSA/pull/473

## 0.9.7 (2024-11-26)
### Fixed
- always validate keys in from_components
- do not crash when handling tiny keys in PKCS1v15

## 0.9.6 (2023-12-01)
### Added
- expose a `pss::get_default_pss_signature_algo_id` helper ([#393])
- expose `pkcs1v15::RsaSignatureAssociatedOid` ([#392])

[#392]: https://github.com/RustCrypto/RSA/pull/392
[#393]: https://github.com/RustCrypto/RSA/pull/393

## 0.9.5 (2023-11-27)
### Added
- Adds `RsaPrivateKey::from_primes` and `RsaPrivateKey::from_p_q` methods ([#386])

[#386]: https://github.com/RustCrypto/RSA/pull/386

## 0.9.4 (2023-11-20)
### Added
- Deterministic implementation of prime factors recovery ([#380])

[#380]: https://github.com/RustCrypto/RSA/pull/380

## 0.9.3 (2023-10-26)
### Added
- PKCS#8/SPKI decoding trait impls for `pkcs1v15` keys ([#346])
- `hazmat` feature as a replacement for `expose-internals` ([#352])

### Changed
- Bump `serde` dependency to 1.0.184 ([#360])

### Removed
- Unused dependencies ([#357])

[#346]: https://github.com/RustCrypto/RSA/pull/346
[#352]: https://github.com/RustCrypto/RSA/pull/352
[#357]: https://github.com/RustCrypto/RSA/pull/357
[#360]: https://github.com/RustCrypto/RSA/pull/360

## 0.9.2 (2023-05-08)
### Fixed
- pkcs1v15: have `fmt` impls call `SignatureEncoding::to_bytes` ([#330])

[#330]: https://github.com/RustCrypto/RSA/pull/330

## 0.9.1 (2023-05-03)
### Fixed
- Left pad signatures when encoding ([#325])

[#325]: https://github.com/RustCrypto/RSA/pull/325

## 0.9.0 (2023-04-27)
### Added
- Function to get salt length from RSA PSS keys ([#277])
- `AssociatedAlgorithmIdentifier` implementation ([#278])
- Random key generation for `pss::BlindedSigningKey` ([#295])
- Impl `Signer` for `pss::SigningKey` ([#297])
- Impl `core::hash::Hash` for `RsaPrivateKey` ([#308])
- Impl `ZeroizeOnDrop` for `RsaPrivateKey`, `SigningKey`, `DecryptingKey` ([#311])
- `u64_digit` feature; on-by-default ([#313])
- `AsRef<RsaPublicKey>` impl on `RsaPrivateKey` ([#317])

### Changed
- Use namespaced features for `serde` ([#268])
- Bump `pkcs1` to v0.7, `pkcs8` to v0.10; MSRV 1.65 ([#270])
- Rename PKCS#1v1.5 `*_with_prefix` methods ([#290])
  - `SigningKey::new` => `SigningKey::new_unprefixed`
  - `SigningKey::new_with_prefix` => `SigningKey::new`
  - `VerifyingKey::new` => `VerifyingKey::new_unprefixed`
  - `VerifyingKey::new_with_prefix` => `VerifyingKey::new`
- Rename `Pkcs1v15Sign::new_raw` to `Pkcs1v15Sign::new_unprefixed` ([#293])
- Use digest output size as default PSS salt length ([#294])
- Specify `salt_len` when verifying PSS signatures ([#294])
- Ensure signatures have the expected length and don't overflow the modulus ([#306])
- Improved public key checks ([#307])
- Rename `CRTValue` => `CrtValue` ([#314])
- Traits under `padding` module now located under `traits` module ([#315])
- `PublicKeyParts`/`PrivateKeyParts` now located under `traits` module ([#315])

### Removed
- "Unsalted" PSS support ([#294])
- `EncryptionPrimitive`/`DecriptionPrimitive` traits ([#300])
- `PublicKey`/`PrivateKey` traits ([#300])
- `Zeroize` impl on `RsaPrivateKey`; automatically zeroized on drop ([#311])
- `Deref<Target=RsaPublicKey>` impl on `RsaPrivateKey`; use `AsRef` instead ([#317])
- `expose-internals` feature and public access to all functions it gated ([#304])

[#268]: https://github.com/RustCrypto/RSA/pull/268
[#270]: https://github.com/RustCrypto/RSA/pull/270
[#277]: https://github.com/RustCrypto/RSA/pull/277
[#278]: https://github.com/RustCrypto/RSA/pull/278
[#290]: https://github.com/RustCrypto/RSA/pull/290
[#293]: https://github.com/RustCrypto/RSA/pull/293
[#294]: https://github.com/RustCrypto/RSA/pull/294
[#295]: https://github.com/RustCrypto/RSA/pull/295
[#297]: https://github.com/RustCrypto/RSA/pull/297
[#300]: https://github.com/RustCrypto/RSA/pull/300
[#306]: https://github.com/RustCrypto/RSA/pull/306
[#307]: https://github.com/RustCrypto/RSA/pull/307
[#308]: https://github.com/RustCrypto/RSA/pull/308
[#311]: https://github.com/RustCrypto/RSA/pull/311
[#313]: https://github.com/RustCrypto/RSA/pull/313
[#314]: https://github.com/RustCrypto/RSA/pull/314
[#315]: https://github.com/RustCrypto/RSA/pull/315
[#317]: https://github.com/RustCrypto/RSA/pull/317

## 0.8.2 (2023-03-01)
### Added
- Encryption-related traits ([#259])

### Fixed
- Possible panic in `internals::left_pad` ([#262])
- Correct PSS sign/verify when key length is multiple of 8+1 bits ([#263])

[#259]: https://github.com/RustCrypto/RSA/pull/259
[#262]: https://github.com/RustCrypto/RSA/pull/262
[#263]: https://github.com/RustCrypto/RSA/pull/263

## 0.8.1 (2023-01-20)
### Added
- `sha2` feature with `oid` subfeature enabled ([#255])

[#255]: https://github.com/RustCrypto/RSA/pull/255

## 0.8.0 (2023-01-17)
### Changed
- Bump `signature` crate dependency to v2 ([#217], [#249])
- Switch to `CryptoRngCore` marker trait ([#237])
- Make `padding` module private ([#243])
- Refactor `PaddingScheme` into a trait ([#244])

### Fixed
- Benchmark build ([#225])

[#217]: https://github.com/RustCrypto/RSA/pull/217
[#225]: https://github.com/RustCrypto/RSA/pull/225
[#237]: https://github.com/RustCrypto/RSA/pull/237
[#243]: https://github.com/RustCrypto/RSA/pull/243
[#244]: https://github.com/RustCrypto/RSA/pull/244
[#249]: https://github.com/RustCrypto/RSA/pull/249

## 0.7.2 (2022-11-14)
### Added
- Public accessor methods for `PrecomputedValues` ([#221])
- Re-export `signature` crate ([#223])

[#221]: https://github.com/RustCrypto/RSA/pull/221
[#223]: https://github.com/RustCrypto/RSA/pull/223


## 0.7.1 (2022-10-31)
### Added
- Documentation improvements ([#216])

### Changed
- Ensure `PaddingScheme` is `Send` and `Sync` ([#215])

[#215]: https://github.com/RustCrypto/RSA/pull/215
[#216]: https://github.com/RustCrypto/RSA/pull/216


## 0.7.0 (2022-10-10) [YANKED]

NOTE: when computing signatures with this release, make sure to enable the
`oid` crate feature of the digest crate you are using when computing the
signature (e.g. `sha2`, `sha3`). If the `oid` feature doesn't exist, make sure
you're using the latest versions.

### Added
- `pkcs1v15` and `pss` modules with `SigningKey`/`VerifyingKey` types
  ([#174], [#195], [#202], [#207], [#208])
- 4096-bit default max `RsaPublicKey` size ([#176])
- `RsaPublicKey::new_with_max_size` ([#176])
- `RsaPublicKey::new_unchecked` ([#206])

### Changed
- MSRV 1.57 ([#162])
- Bump `pkcs1` to 0.4 ([#162])
- Bump `pkcs8` to 0.9 ([#162])
- `RsaPrivateKey::from_components` is now fallible ([#167])
- pkcs1v15: use `AssociatedOid` for getting the RSA prefix ([#183])

### Removed
- `rng` member from PSS padding scheme ([#173])
- `Hash` removed in favor of using OIDs defined in digest crates ([#183])

[#162]: https://github.com/RustCrypto/RSA/pull/162
[#167]: https://github.com/RustCrypto/RSA/pull/167
[#173]: https://github.com/RustCrypto/RSA/pull/173
[#174]: https://github.com/RustCrypto/RSA/pull/174
[#176]: https://github.com/RustCrypto/RSA/pull/176
[#183]: https://github.com/RustCrypto/RSA/pull/183
[#195]: https://github.com/RustCrypto/RSA/pull/195
[#202]: https://github.com/RustCrypto/RSA/pull/202
[#206]: https://github.com/RustCrypto/RSA/pull/206
[#207]: https://github.com/RustCrypto/RSA/pull/207
[#208]: https://github.com/RustCrypto/RSA/pull/208


## 0.6.1 (2022-04-11)

## 0.6.0 (2022-04-08)

## 0.5.0 (2021-07-27)

## 0.4.1 (2021-07-26)

## 0.4.0 (2021-03-28)

## 0.3.0 (2020-06-11)

## 0.2.0 (2019-12-11)

## 0.1.4 (2019-10-13)

## 0.1.3 (2019-03-26)

## 0.1.2 (2019-02-25)

## 0.1.1 (2019-02-20)

## 0.1.0 (2018-12-05)
