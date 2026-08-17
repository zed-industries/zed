# Changelog
All notable changes to this library will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this library adheres to Rust's notion of
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.12.1] - 2022-10-28
### Fixed
- `ff_derive` previously generated a `Field::random` implementation that would
  overflow for fields that needed a full 64-bit spare limb.

## [0.12.0] - 2022-05-04
### Changed

- MSRV is now 1.56.0.
- Bumped `bitvec` to 1.0.

## [0.11.1] - 2022-05-04
### Fixed
- `ff_derive` procedural macro can now be invoked within regular macros.
- Previously, `ff_derive`'s procedural macro would generate implementations of
  `PrimeFieldBits` even when the `bits` crate feature was disabled. `ff_derive`
  can now be used without a dependency on `bitvec` by disabling feature
  features. The new crate feature `derive_bits` can be used to force the
  generation of `PrimeFieldBits` implementations. This new crate feature will be
  removed once our MSRV is at least 1.60 and we have access to [weak dependency
  features](https://blog.rust-lang.org/2022/04/07/Rust-1.60.0.html#new-syntax-for-cargo-features).

## [0.11.0] - 2021-09-02
### Added
- `subtle::ConstantTimeEq` bound on `ff::Field`
- `Copy + Send + Sync + 'static` bounds on `ff::PrimeField::Repr`
- `ff::derive` module behind the `derive` feature flag, containing dependencies for the
  `PrimeField` derive macro:
  - Re-exports of required crates.
  - `adc, mac, sbb` constant-time const helper functions.
- `ff::Field::is_zero_vartime`
- `ff::PrimeField::from_repr_vartime`

### Changed
- `ff::Field::is_zero` now returns `subtle::Choice`.
- `ff::PrimeField::{is_odd, is_even}` now return `subtle::Choice`.
- `ff::PrimeField::from_repr` now return `subtle::CtOption<Self>`.
- `ff::PrimeField::from_str` has been renamed to `PrimeField::from_str_vartime`.

### Removed
- `ff::{adc, mac_with_carry, sbb}` (replaced by `ff::derive::{adc, mac, sbb}`).

## [0.10.1] - 2021-08-11
### Added
- `ff::BatchInvert` extension trait, implemented for iterators over mutable field elements
  which allows those field elements to be inverted in a batch. This trait is behind the
  new `alloc` feature flag.
- `ff::BatchInverter` struct, which provides methods for non-allocating batch inversion of
  field elements contained within slices.

## [0.10.0] - 2021-06-01
### Added
- `ff::PrimeFieldBits: PrimeField` trait, behind a `bits` feature flag.

### Changed
- MSRV is now 1.51.0.
- Bumped `bitvec` to 0.22 to enable fixing a performance regression in `ff 0.9`.
  The `bitvec::view::BitView` re-export has been replaced by
  `bitvec::view::BitViewSized`.
- The `bitvec` dependency and its re-exports have been gated behind the `bits`
  feature flag.

### Removed
- `ff::PrimeField::{ReprBits, char_le_bits, to_le_bits}` (replaced by
  `ff::PrimeFieldBits` trait).

### Fixed
- `#[derive(PrimeField)]` now works on small moduli (that fit in a single `u64`
  limb).

## [0.9.0] - 2021-01-05
### Added
- Re-export of `bitvec::view::BitView`.
- `ff::FieldBits<V>` type alias for the return type of
  `ff::PrimeField::{char_le_bits, to_le_bits}`.

### Changed
- Bumped `bitvec` to 0.20, `rand_core` to 0.6.

### Removed
- `From<Self>` and `From<&Self>` bounds on `ff::PrimeField::Repr`.

## [0.8.0] - 2020-09-08
### Added
- `ff::PrimeField::{ReprBits, char_le_bits, to_le_bits}`, and a public
  dependency on `bitvec 0.18`.
- `ff::Field::cube` method with provided implementation.
- `Send + Sync` bounds on `ff::PrimeField::ReprBits`

### Changed
- MSRV is now 1.44.0.
- `ff::Field::random<R: RngCore + ?Sized>(rng: &mut R) -> Self` has been changed
  to `Field::random(rng: impl RngCore) -> Self`, to aligh with
  `group::Group::random`.

### Removed
- `fmt::Display` bound on `ff::Field`.
- `ff::PrimeField::char` (replaced by `ff::PrimeField::char_le_bits`).
- `ff::{BitIterator, Endianness, PrimeField::ReprEndianness` (replaced by
  `ff::PrimeField::to_le_bits`).
