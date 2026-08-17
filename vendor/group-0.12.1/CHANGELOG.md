# Changelog
All notable changes to this library will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this library adheres to Rust's notion of
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.12.1] - 2022-10-13
### Added
- `group::{WnafBase, WnafScalar}` structs for caching precomputations of both
  bases and scalars, for improved many-base many-scalar multiplication
  performance.
- `impl memuse::DynamicUsage for group::{Wnaf WnafBase, WnafScalar}`, behind the
  new `wnaf-memuse` feature flag, to enable the heap usage of these types to be
  measured at runtime.

### Changed
- Removed temporary allocations from `Wnaf` internals for improved performance.

## [0.12.0] - 2022-05-04
### Changed
- MSRV is now 1.56.0.
- Bumped `ff` to `0.12`

## [0.11.0] - 2021-09-02
### Fixed
- The affine scalar multiplication bounds on the following traits had typos that
  prevented multiplying by `&Self::Scalar`, which has now been fixed:
  - `group::cofactor::{CofactorCurve::Affine, CofactorCurveAffine}`
  - `group::prime::{PrimeCurve::Affine, PrimeCurveAffine}`

### Added
- `Copy + Send + Sync + 'static` bounds on `group::GroupEncoding::Repr`.

### Changed
- Bumped `ff` to 0.11.

## [0.10.0] - 2021-06-01
### Added
- `group::ff`, which re-exports the `ff` crate to make version-matching easier.

### Changed
- MSRV is now 1.51.0.
- Bumped `ff` to 0.10.

### Removed
- `group::cofactor::CofactorGroup::is_torsion_free` provided implementation
  (trait implementors must now implement this method themselves). This avoids
  a hard dependency on the `ff/bits` feature flag.

## [0.9.0] - 2021-01-06
### Changed
- Bumped dependencies to `ff 0.9`, `rand_core 0.6`, `rand 0.8`.

## [0.8.0] - 2020-09-08
### Added
- `no_std` support.

### Changed
- MSRV is now 1.44.0.
- Bumped `ff` to 0.8.
- `group::{wnaf, Wnaf, WnafGroup}` are now gated behind the (default-enabled)
  `alloc` feature flag. The `byteorder` dependency is now optional.
- `group::tests` is now gated behind the `tests` feature flag. The `rand` and
  `rand_xorshift` dependencies are now optional.

### Removed
- `fmt::Display` bound from the following traits:
  - `group::Group`
  - `group::cofactor::CofactorCurveAffine`
  - `group::prime::PrimeCurveAffine`
