# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.12.3 (2022-08-01)
### Added
- Aliases for SEC1 compressed/uncompressed points ([#1067])

### Fixed
- `arithmetic` + `serde` feature combo ([#1066])

[#1066]: https://github.com/RustCrypto/traits/pull/1066
[#1067]: https://github.com/RustCrypto/traits/pull/1067

## 0.12.2 (2022-07-01)
### Changed
- Bump `crypto-bigint` to v0.4.8 ([#1039])

[#1039]: https://github.com/RustCrypto/traits/pull/1039

## 0.12.1 (2022-06-12)
### Added
- `impl_field_element!` macro ([#1021])
- Generic impl of complete prime order formulas ([#1022])

### Changed
- Bump `crypto-bigint` to v0.4.4 ([#1018], [#1020])

[#1018]: https://github.com/RustCrypto/traits/pull/1018
[#1020]: https://github.com/RustCrypto/traits/pull/1020
[#1021]: https://github.com/RustCrypto/traits/pull/1021
[#1022]: https://github.com/RustCrypto/traits/pull/1022

## 0.12.0 (2022-05-08)
### Added
- `ecdh::SharedSecret::extract` HKDF helper ([#1007])

### Changed
- Bump `digest` dependency to v0.10 ([#883], [#904])
- Make `NonZeroScalar::invert` infallible ([#894])
- `ToCompactEncodedPoint` now returns `CtOption` ([#895])
- Move `hash2field` into `hash2curve` module ([#903])
- Bump `ff` and `group` dependencies to v0.12 ([#994])
- Use `serdect` crate ([#996])
- Replace `AlgorithmParamters` with `AssociatedOid` ([#1001])
- Bump `crypto-bigint` dependency to v0.4 ([#1005])
- Bump `der` dependency to v0.6 ([#1006])
- Bump `pkcs8` dependency to v0.9 ([#1006])
- Bump `sec1` dependency to v0.3 ([#1006])
- Bump `pem-rfc7468` dependency to v0.6 ([#1009])

### Removed
- `Zeroize` impl from `ecdh::SharedSecret` ([#978])

[#883]: https://github.com/RustCrypto/traits/pull/883
[#894]: https://github.com/RustCrypto/traits/pull/894
[#895]: https://github.com/RustCrypto/traits/pull/895
[#903]: https://github.com/RustCrypto/traits/pull/903
[#904]: https://github.com/RustCrypto/traits/pull/904
[#978]: https://github.com/RustCrypto/traits/pull/978
[#994]: https://github.com/RustCrypto/traits/pull/994
[#996]: https://github.com/RustCrypto/traits/pull/996
[#1001]: https://github.com/RustCrypto/traits/pull/1001
[#1005]: https://github.com/RustCrypto/traits/pull/1005
[#1006]: https://github.com/RustCrypto/traits/pull/1006
[#1007]: https://github.com/RustCrypto/traits/pull/1007
[#1009]: https://github.com/RustCrypto/traits/pull/1009

## 0.11.12 (2022-01-30)
### Changed
- Disable `bits` feature on docs.rs due to nightly breakage ([#927])

[#927]: https://github.com/RustCrypto/traits/pull/927

## 0.11.11 (2022-01-30)
- No changes; triggering a docs.rs rebuild

## 0.11.10 (2022-01-27)
### Changed
- Revert [#884] to support a wider range of `zeroize` versions ([#923])

[#923]: https://github.com/RustCrypto/traits/pull/891

## 0.11.9 (2022-01-17) [YANKED]
### Changed
- Activate `bits`, `hash2curve`, and `voprf` features on docs.rs ([#891])

[#891]: https://github.com/RustCrypto/traits/pull/891

## 0.11.8 (2022-01-15) [YANKED]
### Added
- Impl `ZeroizeOnDrop` on appropriate items ([#884])

### Changed
- Use the `base16ct` crate for hex serialization ([#886], [#887], [#888])

[#884]: https://github.com/RustCrypto/traits/pull/884
[#886]: https://github.com/RustCrypto/traits/pull/886
[#887]: https://github.com/RustCrypto/traits/pull/887
[#888]: https://github.com/RustCrypto/traits/pull/888

## 0.11.7 (2022-01-14) [YANKED]
### Added
- Initial hash-to-field support ([#854], [#855], [#871], [#874])
- Initial hash-to-curve support ([#865], [#876])
- Impl `Mul` for `NonZeroScalar` * `NonZeroScalar` ([#857], [#862])
- `Reduce::from_*e_digest_reduced` ([#869])
- `VoprfParameters` trait ([#878])

[#854]: https://github.com/RustCrypto/traits/pull/854
[#855]: https://github.com/RustCrypto/traits/pull/855
[#857]: https://github.com/RustCrypto/traits/pull/857
[#862]: https://github.com/RustCrypto/traits/pull/862
[#865]: https://github.com/RustCrypto/traits/pull/865
[#869]: https://github.com/RustCrypto/traits/pull/869
[#871]: https://github.com/RustCrypto/traits/pull/871
[#874]: https://github.com/RustCrypto/traits/pull/874
[#876]: https://github.com/RustCrypto/traits/pull/876
[#878]: https://github.com/RustCrypto/traits/pull/878

## 0.11.6 (2021-12-20)
### Added
- Type conversions chart ([#852])

[#852]: https://github.com/RustCrypto/traits/pull/852

## 0.11.5 (2021-12-05)
### Changed
- Revised `LinearCombination` trait ([#835])

[#835]: https://github.com/RustCrypto/traits/pull/835

## 0.11.4 (2021-12-04) [YANKED]
### Added
- `LinearCombination` trait ([#832])

[#832]: https://github.com/RustCrypto/traits/pull/832

## 0.11.3 (2021-12-03) [YANKED]
### Added
- `ReduceNonZero` trait ([#827])

[#827]: https://github.com/RustCrypto/traits/pull/827

## 0.11.2 (2021-12-03) [YANKED]
### Changed
- Bump `pem-rfc7468` dependency to v0.3 ([#825])

[#825]: https://github.com/RustCrypto/traits/pull/825

## 0.11.1 (2021-11-21) [YANKED]
### Added
- `NonZeroScalar::from_uint` ([#822])

[#822]: https://github.com/RustCrypto/traits/pull/822

## 0.11.0 (2021-11-19) [YANKED]
### Added
- `ScalarCore<C>` type ([#732])
- `PrimeCurveArithmetic` trait ([#739])
- SEC1 private key support ([#762])
- `Reduce` trait ([#768])
- Re-export `ff` and `PrimeField` ([#796])
- `Encoding` bound on `Curve::UInt` ([#806])
- `scalar::IsHigh` trait ([#814], [#815])
- `Neg` impl for `NonZeroScalar<C>` ([#816])
- `AffineXCoordinate` trait ([#817])
- `serde` support for scalar and `PublicKey` types ([#818])

### Changed
- Bump `ff` + `group` to v0.11 ([#730])
- Make `SecretKey::to_jwk_string` self-zeroizing ([#742])
- Use `sec1` crate's `EncodedPoint` ([#771])
- Make `FromEncodedPoint` return a `CtOption` ([#782])
- Rust 2021 edition upgrade; MSRV to 1.56 ([#795])
- Bump `crypto-bigint` dependency to v0.3 ([#807])
- Use `sec1` crate for `pkcs8` support ([#809])
- Bump `spki` dependency to v0.5 release ([#810])
- `NonZeroScalar` is now bounded on `ScalarArithmetic` instead of
  `ProjectiveArithmetic` ([#812])

### Fixed
- `Zeroize` impl on `NonZeroScalar` ([#785])

[#730]: https://github.com/RustCrypto/traits/pull/730
[#732]: https://github.com/RustCrypto/traits/pull/732
[#739]: https://github.com/RustCrypto/traits/pull/739
[#742]: https://github.com/RustCrypto/traits/pull/742
[#762]: https://github.com/RustCrypto/traits/pull/762
[#768]: https://github.com/RustCrypto/traits/pull/768
[#771]: https://github.com/RustCrypto/traits/pull/771
[#782]: https://github.com/RustCrypto/traits/pull/782
[#785]: https://github.com/RustCrypto/traits/pull/785
[#795]: https://github.com/RustCrypto/traits/pull/795
[#796]: https://github.com/RustCrypto/traits/pull/796
[#806]: https://github.com/RustCrypto/traits/pull/806
[#807]: https://github.com/RustCrypto/traits/pull/807
[#809]: https://github.com/RustCrypto/traits/pull/809
[#810]: https://github.com/RustCrypto/traits/pull/810
[#812]: https://github.com/RustCrypto/traits/pull/812
[#814]: https://github.com/RustCrypto/traits/pull/814
[#815]: https://github.com/RustCrypto/traits/pull/815
[#816]: https://github.com/RustCrypto/traits/pull/816
[#817]: https://github.com/RustCrypto/traits/pull/817
[#818]: https://github.com/RustCrypto/traits/pull/818

## 0.10.6 (2021-08-23)
### Changed
- Bump `crypto-bigint` dependency to v0.2.4 ([#710])

[#710]: https://github.com/RustCrypto/traits/pull/710

## 0.10.5 (2021-07-20)
### Changed
- Pin `zeroize` dependency to v1.4 and `subtle` to v2.4 ([#689])

[#689]: https://github.com/RustCrypto/traits/pull/689

## 0.10.4 (2021-07-12)
### Added
- Re-export `rand_core` ([#683])

[#683]: https://github.com/RustCrypto/traits/pull/683

## 0.10.3 (2021-06-21)
### Changed
- Bump `crypto-bigint` to v0.2.1 ([#673])

[#673]: https://github.com/RustCrypto/traits/pull/673

## 0.10.2 (2021-06-14) [YANKED]
### Added
- `ConstantTimeEq` impl for `NonZeroScalar` ([#669])

[#669]: https://github.com/RustCrypto/traits/pull/669

## 0.10.1 (2021-06-09) [YANKED]
### Added
- Explicit `Copy` bounds on `PublicKey` ([#667])

[#667]: https://github.com/RustCrypto/traits/pull/667

## 0.10.0 (2021-06-07) [YANKED]
### Added
- `ScalarBytes::from_uint` ([#651])
- `dev::ScalarBytes` ([#652])
- `ScalarArithmetic` trait ([#654])
- `AffineArithmetic` trait ([#658])
- `PointCompaction` trait and SEC1 tag support ([#659])

### Changed
- Bump `ff` and `group` to v0.10; MSRV 1.51+ ([#643])
- Merge `Curve` and `Order` traits ([#644])
- Use `crypto-bigint` to represent `Curve::ORDER` ([#645])
- Source `FieldSize<C>` from `C::UInt` type ([#646])
- Impl `ScalarBytes<C>` using `C::UInt` ([#647])
- Make `ScalarBytes<C>` the `SecretKey<C>` internal repr ([#649])
- Bump `crypto-bigint` to v0.2 ([#662])
- Bump `pkcs8` to v0.7 ([#662])

### Removed
- `util` module ([#648])

[#643]: https://github.com/RustCrypto/traits/pull/643
[#644]: https://github.com/RustCrypto/traits/pull/644
[#645]: https://github.com/RustCrypto/traits/pull/645
[#646]: https://github.com/RustCrypto/traits/pull/646
[#647]: https://github.com/RustCrypto/traits/pull/647
[#648]: https://github.com/RustCrypto/traits/pull/648
[#649]: https://github.com/RustCrypto/traits/pull/649
[#651]: https://github.com/RustCrypto/traits/pull/651
[#652]: https://github.com/RustCrypto/traits/pull/652
[#654]: https://github.com/RustCrypto/traits/pull/654
[#658]: https://github.com/RustCrypto/traits/pull/658
[#659]: https://github.com/RustCrypto/traits/pull/659
[#662]: https://github.com/RustCrypto/traits/pull/662

## 0.9.12 (2021-05-18)
### Added
- `Ord` and `PartialOrd` impls on `PublicKey` ([#637])

[#637]: https://github.com/RustCrypto/traits/pull/637

## 0.9.11 (2021-04-21)
### Added
- Impl `subtle` traits on `ScalarBytes<C>` ([#612])

### Fixed
- Always re-export ScalarBytes ([#613])

[#612]: https://github.com/RustCrypto/traits/pull/612
[#613]: https://github.com/RustCrypto/traits/pull/613

## 0.9.10 (2021-04-21)
### Added
- `ScalarBytes` type ([#610])

[#610]: https://github.com/RustCrypto/traits/pull/610

## 0.9.9 (2021-04-21) [YANKED]
### Added
- `Order::is_scalar_repr_in_range` ([#608])

[#608]: https://github.com/RustCrypto/traits/pull/608

## 0.9.8 (2021-04-21)
### Added
- Define `Order` for `MockCurve` ([#606])

[#606]: https://github.com/RustCrypto/traits/pull/606

## 0.9.7 (2021-04-21)
### Added
- `Order` trait ([#603])

### Fixed
- Warnings from `pkcs8` imports ([#604])

[#603]: https://github.com/RustCrypto/traits/pull/603
[#604]: https://github.com/RustCrypto/traits/pull/604

## 0.9.6 (2021-03-22)
### Changed
- Bump `pkcs8` dependency to v0.6 ([#585])

[#585]: https://github.com/RustCrypto/traits/pull/585

## 0.9.5 (2021-03-17) [YANKED]
### Added
- Implement `{to,char}_le_bits` for `MockCurve` ([#565])
- Implement `one()` for mock `Scalar` ([#566])

### Changed
- Use string-based OID constants ([#561])
- Bump `base64ct` dependency to v1.0 ([#581])

[#561]: https://github.com/RustCrypto/traits/pull/561
[#565]: https://github.com/RustCrypto/traits/pull/565
[#566]: https://github.com/RustCrypto/traits/pull/566
[#581]: https://github.com/RustCrypto/traits/pull/581

## 0.9.4 (2021-02-18) [YANKED]
### Fixed
- Breakage related to the `pkcs8` v0.5.1 crate ([#556]) 

[#556]: https://github.com/RustCrypto/traits/pull/556

## 0.9.3 (2021-02-16) [YANKED]
### Changed
- Bump `pkcs8` dependency to v0.5.0 ([#549])

### Fixed
- Workaround for bitvecto-rs/bitvec#105 ([#550])

[#549]: https://github.com/RustCrypto/traits/pull/549
[#550]: https://github.com/RustCrypto/traits/pull/550

## 0.9.2 (2021-02-12) [YANKED]
### Changed
- Flatten `weierstrass` module ([#542])

[#542]: https://github.com/RustCrypto/traits/pull/542

## 0.9.1 (2021-02-11) [YANKED]
### Removed
- `BitView` re-export ([#540])

[#540]: https://github.com/RustCrypto/traits/pull/540

## 0.9.0 (2021-02-10) [YANKED]
### Added
- JWK support ([#483])
- `sec1::ValidatePublicKey` trait ([#485])
- `hazmat` crate feature ([#487])
- `Result` alias ([#534])

### Changed
- Bump `ff` and `group` crates to v0.9 ([#452])
- Simplify ECDH trait bounds ([#475])
- Flatten API ([#487])
- Bump `pkcs8` crate dependency to v0.4 ([#493])

### Removed
- Direct `bitvec` dependency ([#484])
- `FromDigest` trait ([#532])

[#452]: https://github.com/RustCrypto/traits/pull/452
[#475]: https://github.com/RustCrypto/traits/pull/475
[#483]: https://github.com/RustCrypto/traits/pull/483
[#484]: https://github.com/RustCrypto/traits/pull/484
[#485]: https://github.com/RustCrypto/traits/pull/485
[#487]: https://github.com/RustCrypto/traits/pull/487
[#493]: https://github.com/RustCrypto/traits/pull/493
[#432]: https://github.com/RustCrypto/traits/pull/432
[#532]: https://github.com/RustCrypto/traits/pull/532
[#534]: https://github.com/RustCrypto/traits/pull/534

## 0.8.5 (2021-02-17)
### Fixed
- Workaround for bitvecto-rs/bitvec#105 ([#553])

[#553]: https://github.com/RustCrypto/traits/pull/553

## 0.8.4 (2020-12-23)
### Fixed
- Rust `nightly` regression ([#432])

[#432]: https://github.com/RustCrypto/traits/pull/432

## 0.8.3 (2020-12-22)
### Fixed
- Regression in combination of `pem`+`zeroize` features ([#429])

[#429]: https://github.com/RustCrypto/traits/pull/429

## 0.8.2 (2020-12-22) [YANKED]
### Added
- Low-level ECDH API ([#418])
- `dev` module ([#419])
- Impl `pkcs8::ToPrivateKey` for `SecretKey<C>` ([#423])
- Impl `pkcs8::ToPublicKey` for `PublicKey<C>` ([#427])

### Changed
- Bump `subtle` dependency to 2.4.0 ([#414])
- Bump `pkcs8` dependency to v0.3.3 ([#425])
- Use `der` crate to parse `SecretKey` ([#422])

### Fixed
- Make `PublicKey::from_encoded_point` go through `PublicKey::from_affine` ([#416])

[#414]: https://github.com/RustCrypto/traits/pull/414
[#416]: https://github.com/RustCrypto/traits/pull/416
[#418]: https://github.com/RustCrypto/traits/pull/418
[#419]: https://github.com/RustCrypto/traits/pull/419
[#422]: https://github.com/RustCrypto/traits/pull/422
[#423]: https://github.com/RustCrypto/traits/pull/423
[#425]: https://github.com/RustCrypto/traits/pull/425
[#427]: https://github.com/RustCrypto/traits/pull/427

## 0.8.1 (2020-12-16) [YANKED]
### Fixed
- Builds on Rust `nightly` compiler ([#412])

[#412]: https://github.com/RustCrypto/traits/pull/412

## 0.8.0 (2020-12-16) [YANKED]
### Added
- Impl `subtle::ConditionallySelectable` for `sec1::EncodedPoint` ([#409])
- `sec1::EncodedPoint::identity()` method ([#408])
- `sec1::Coordinates::tag` method ([#407])
- Support for SEC1 identity encoding ([#401])

### Changed
- Bump `pkcs8` crate dependency to v0.3 ([#405])
- Ensure `PublicKey<C>` is not the identity point ([#404])
- Have `SecretKey::secret_scalar` return `NonZeroScalar` ([#402])

### Removed
- `SecretKey::secret_value` ([#403])

[#409]: https://github.com/RustCrypto/traits/pull/409
[#408]: https://github.com/RustCrypto/traits/pull/408
[#407]: https://github.com/RustCrypto/traits/pull/407
[#405]: https://github.com/RustCrypto/traits/pull/405
[#404]: https://github.com/RustCrypto/traits/pull/404
[#403]: https://github.com/RustCrypto/traits/pull/403
[#402]: https://github.com/RustCrypto/traits/pull/402
[#401]: https://github.com/RustCrypto/traits/pull/401

## 0.7.1 (2020-12-07)
### Changed
- Have `SecretKey::secret_value` always return `NonZeroScalar` ([#390])

[#390]: https://github.com/RustCrypto/traits/pull/390

## 0.7.0 (2020-12-06) [YANKED]
### Added
- Impl `pkcs8::FromPublicKey` for `PublicKey<C>` ([#385])
- Impl `pkcs8::FromPrivateKey` trait for `SecretKey<C>` ([#381], [#383])
- PKCS#8 PEM support ([#382])
- `SecretKey::secret_value()` method ([#375])
- `PublicKey<C>` type ([#363], [#366])

### Changed
- Rename `PublicKey::from_bytes()` to `::from_sec1_bytes()` ([#376])
- `sec1::EncodedPoint` uses `Option` instead of `subtle::CtOption` ([#367])
- Bump `const-oid` to v0.3; MSRV 1.46+ ([#365], [#381])

### Fixed
- `ecdh` rustdoc ([#364])

[#385]: https://github.com/RustCrypto/traits/pull/385
[#383]: https://github.com/RustCrypto/traits/pull/383
[#382]: https://github.com/RustCrypto/traits/pull/382
[#381]: https://github.com/RustCrypto/traits/pull/381
[#376]: https://github.com/RustCrypto/traits/pull/376
[#375]: https://github.com/RustCrypto/traits/pull/375
[#367]: https://github.com/RustCrypto/traits/pull/367
[#366]: https://github.com/RustCrypto/traits/pull/366
[#365]: https://github.com/RustCrypto/traits/pull/365
[#364]: https://github.com/RustCrypto/traits/pull/364
[#363]: https://github.com/RustCrypto/traits/pull/363

## 0.6.6 (2020-10-08)
### Added
- Derive `Clone` on `SecretBytes` ([#330])

[#300]: https://github.com/RustCrypto/traits/pull/300

## 0.6.5 (2020-10-08)
### Fixed
- Work around `nightly-2020-10-06` breakage ([#328])

[#328]: https://github.com/RustCrypto/traits/pull/328

## 0.6.4 (2020-10-08)
### Added
- Impl `From<SecretBytes<C>>` for `FieldBytes<C>` ([#326])

[#326]: https://github.com/RustCrypto/traits/pull/326

## 0.6.3 (2020-10-08)
### Added
- `SecretBytes` newtype ([#324])

[#324]: https://github.com/RustCrypto/traits/pull/324

## 0.6.2 (2020-09-24)
### Added
- `sec1::EncodedPoint::to_untagged_bytes()` method ([#312])

[#312]: https://github.com/RustCrypto/traits/pull/312

## 0.6.1 (2020-09-21)
### Fixed
- `sec1::EncodedPoint::decompress` ([#309])

[#309]: https://github.com/RustCrypto/traits/pull/309

## 0.6.0 (2020-09-11) [YANKED]
### Added
- `arithmetic` feature ([#293])
- Generic curve/field arithmetic using the `ff` and `group` crates
  ([#287], [#291], [#292])
- `sec1::Coordinates` ([#286])
- `weierstrass::point::Compression` trait ([#283], [#300])
- Arithmetic helper functions ([#281])
- `digest` feature and `FromDigest` trait ([#279])
- impl `Deref` for `NonZeroScalar` ([#278])
- Conditionally impl `Invert` for `NonZeroScalar` ([#277])
- `NonZeroScalar::to_bytes` ([#276])
- `EncodedPoint::decompress` ([#275])
- `sec1::Tag` ([#270])
- `weierstrass::point::Decompress` trait ([#266])
- `alloc` feature + `EncodedPoint::to_bytes()` ([#265])

### Changed
- Renamed `Arithmetic` trait to `point::ProjectiveArithmetic` ([#300])
- Replaced `Arithmetic::Scalar` and `Arithmetic::AffinePoint`
  with `Scalar<C>` and `AffinePoint<C>` ([#300])
- Made `SecretKey<C>` inner type generic ([#297])
- Renamed `ElementBytes<C>` to `FieldBytes<C>` ([#296])
- MSRV 1.44 ([#292])
- Minimum `subtle` version now v2.3 ([#290])
- Renamed `Curve::ElementSize` to `::FieldSize` ([#282])
- Refactor `PublicKey` into `sec1::EncodedPoint` ([#264])

### Removed
- `FromBytes` trait ([#300])
- `Generate` trait ([#295])

[#300]: https://github.com/RustCrypto/traits/pull/300
[#297]: https://github.com/RustCrypto/traits/pull/297
[#296]: https://github.com/RustCrypto/traits/pull/296
[#295]: https://github.com/RustCrypto/traits/pull/295
[#293]: https://github.com/RustCrypto/traits/pull/293
[#292]: https://github.com/RustCrypto/traits/pull/292
[#291]: https://github.com/RustCrypto/traits/pull/291
[#290]: https://github.com/RustCrypto/traits/pull/290
[#287]: https://github.com/RustCrypto/traits/pull/293
[#286]: https://github.com/RustCrypto/traits/pull/286
[#283]: https://github.com/RustCrypto/traits/pull/283
[#282]: https://github.com/RustCrypto/traits/pull/282
[#281]: https://github.com/RustCrypto/traits/pull/281
[#279]: https://github.com/RustCrypto/traits/pull/279
[#278]: https://github.com/RustCrypto/traits/pull/278
[#277]: https://github.com/RustCrypto/traits/pull/277
[#276]: https://github.com/RustCrypto/traits/pull/276
[#275]: https://github.com/RustCrypto/traits/pull/275
[#270]: https://github.com/RustCrypto/traits/pull/270
[#266]: https://github.com/RustCrypto/traits/pull/266
[#265]: https://github.com/RustCrypto/traits/pull/265
[#264]: https://github.com/RustCrypto/traits/pull/264

## 0.5.0 (2020-08-10)
### Added
- `Arithmetic` trait ([#219])
- `Generate` trait ([#220], [#226])
- Toplevel `Curve` trait ([#223])
- `Invert` trait ([#228])
- `FromPublicKey` trait ([#229], [#248])
- Re-export `zeroize` ([#233])
- OID support ([#240], [#245])
- `NonZeroScalar` type ([#241])
- `Generator` trait ([#241])
- `weierstrass::PublicKey::compress` method ([#243])
- Derive `Clone` on `SecretKey` ([#244])
- Generic Elliptic Curve Diffie-Hellman support ([#251])

### Changed
- Moved repo to https://github.com/RustCrypto/traits ([#213])
- Rename `ScalarBytes` to `ElementBytes` ([#246])
- Rename `CompressedCurvePoint`/`UncompressedCurvePoint` to
  `CompressedPoint`/`UncompressedPoint`

[#213]: https://github.com/RustCrypto/traits/pull/213
[#219]: https://github.com/RustCrypto/traits/pull/219
[#220]: https://github.com/RustCrypto/traits/pull/220
[#223]: https://github.com/RustCrypto/traits/pull/223
[#226]: https://github.com/RustCrypto/traits/pull/226
[#228]: https://github.com/RustCrypto/traits/pull/228
[#229]: https://github.com/RustCrypto/traits/pull/229
[#233]: https://github.com/RustCrypto/traits/pull/233
[#240]: https://github.com/RustCrypto/traits/pull/240
[#241]: https://github.com/RustCrypto/traits/pull/241
[#243]: https://github.com/RustCrypto/traits/pull/243
[#244]: https://github.com/RustCrypto/traits/pull/244
[#245]: https://github.com/RustCrypto/traits/pull/245
[#246]: https://github.com/RustCrypto/traits/pull/246
[#248]: https://github.com/RustCrypto/traits/pull/248
[#251]: https://github.com/RustCrypto/traits/pull/251

## 0.4.0 (2020-06-04)
### Changed
- Bump `generic-array` dependency from v0.12 to v0.14

## 0.3.0 (2020-01-15)
### Added
- `Scalar` struct type

### Changed
- Repository moved to <https://github.com/RustCrypto/elliptic-curves>

### Removed
- Curve definitions/arithmetic extracted out into per-curve crates

## 0.2.0 (2019-12-11)
### Added
- `secp256r1` (P-256) point compression and decompression

### Changed
- Bump MSRV to 1.37

## 0.1.0 (2019-12-06)
- Initial release
