# CHANGELOG

## [# 1.0.2 (2024-02-16)](https://github.com/boa-dev/ryu-js/compare/v1.0.1...v1.0.2)

### Internal Improvements

- [INTERNAL #48](https://github.com/boa-dev/ryu-js/pull/48): Sync `upstream/master`. (@HalidOdat)

## [# 1.0.1 (2024-03-02)](https://github.com/boa-dev/ryu-js/compare/v1.0.0...v1.0.1)

### Internal Improvements

- [INTERNAL #45](https://github.com/boa-dev/ryu-js/pull/45): Add release workflow. (@HalidOdat)
- [INTERNAL #43](https://github.com/boa-dev/ryu-js/pull/43): Add `#[inline]` to public functions. (@HalidOdat)
- [INTERNAL #42](https://github.com/boa-dev/ryu-js/pull/42): Sync upstream/master. (@HalidOdat, @jedel1043)

## [# 1.0.0 (2023-10-05)](https://github.com/boa-dev/ryu-js/compare/v0.2.2...v1.0.0) - ECMAScript compliant implementation of `Number.prototype.toFixed()`

### Breaking Changes

- Minimum rust version has been bumped from `1.36.0` to `1.64.0`.

### Feature Enhancements

- [FEATURE](https://github.com/boa-dev/ryu-js/pull/35):
  ECMAScript specification compliant `Number.prototype.toFixed()` implementation. (@HalidOdat)

### Internal Improvements

- [INTERNAL #1](https://github.com/boa-dev/ryu-js/pull/19): Added dependabot PRs. (@Razican)
- [INTERNAL #2](https://github.com/boa-dev/ryu-js/pull/21): Sync upstream/master. (@HalidOdat)
- [INTERNAL #3](https://github.com/boa-dev/ryu-js/pull/27): Add issue and PR templates. (@HalidOdat)
- [INTERNAL #4](https://github.com/boa-dev/ryu-js/pull/28): Switch to criterion for benchmarking. (@HalidOdat)
- [INTERNAL #5](https://github.com/boa-dev/ryu-js/pull/29): Benchmark CI. (@HalidOdat)
- [INTERNAL #6](https://github.com/boa-dev/ryu-js/pull/38): Enable merge queue. (@jedel1043)

## [# 0.2.2 (2020-12-16)](https://github.com/boa-dev/ryu-js/compare/v0.2.1...v0.2.2)

### Internal Improvements

  - [INTERNAL #17](https://github.com/boa-dev/ryu-js/pull/17) Sync to `dtolnay/ryu` master
  - [INTERNAL #16](https://github.com/boa-dev/ryu-js/pull/16) Sync to `dtolnay/ryu` master

## [# 0.2.1 (2020-11-11)](https://github.com/boa-dev/ryu-js/compare/v0.2.0...v0.2.1)

### Feature Enhancements

 - [FEATURE #11](https://github.com/boa-dev/ryu-js/pull/11):
  Null check in unsafe `format32` and `format64` (in debug mode). (@HalidOdat)

### Bug Fixes

 - BUG [#12](https://github.com/boa-dev/ryu-js/pull/12) [#13](https://github.com/boa-dev/ryu-js/pull/13):
  Documentation fixes (@HalidOdat)

## [# 0.2.0 (2020-07-14) - ECMAScript compliant `f32` conversions Release](https://github.com/boa-dev/ryu-js/compare/v0.1.0...v0.2.0)

### Feature Enhancements

 - [FEATURE #6](https://github.com/boa-dev/ryu-js/pull/6):
  ECMAScript specification complaint `f32` to string conversions. (@HalidOdat)

### Bug Fixes

 - [BUG #2](https://github.com/boa-dev/ryu-js/pull/2) (@HalidOdat):
   - Fixed compatibility with rust `1.31.0`.
   - Fixed converting from `-0.0` to `0`.
   - Fixed max length docs for `format32` and `format64`.

### Internal Improvements

 - [INTERNAL #2](https://github.com/boa-dev/ryu-js/pull/2):
  Optimized `0` and `-0` to string conversion (@HalidOdat)

# # 0.1.0 (2020-07-13) - ECMAScript compliant `f64` conversions Release

This is the initial release of this crate, it introduces ECMAScript compliant `f64` to string conversions.

### Feature Enhancements

- [FEATURE](https://github.com/boa-dev/ryu-js/commit/ed781f5772882e38c53d40707a60b4f11414b9c8):
  ECMAScript specification complaint `f64` to string conversions. (@Tropid)
- [FEATURE](https://github.com/boa-dev/ryu-js/commit/fe366fa397d04324fa693b5d85134851b09719b3):
  Change name from `ryu` to `ryu-js`. (@Tropid)

### Bug Fixes

- [BUG #1](https://github.com/boa-dev/ryu-js/pull/1):
  Fixed buffer overflow with length greater than 24 (max is 25). (@HalidOdat)

### Internal Improvements

 - [INTERNAL #1](https://github.com/boa-dev/ryu-js/pull/2):
  Fixed all clippy warnings/errors and tests (@HalidOdat)
