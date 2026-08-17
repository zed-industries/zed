# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [v0.3.0] - 2021-09-04

### Changed

- (breaking change) The guaranteed MSRV is now 1.31.0. ([#40])
- (breaking change) The `std` Cargo feature is no longer enabled by default. ([#44])
- 128-bit integer support is now always available by default. ([#37])

[#37]: https://github.com/japaric/cast.rs/pull/37
[#40]: https://github.com/japaric/cast.rs/pull/40
[#44]: https://github.com/japaric/cast.rs/pull/44

### Fixed

- fixed casting `255f32` to `u8` returning `Error::Overflow` ([#23], [#42])
- fixed intent of promote-and-back tests ([#39], [#43])

[#23]: https://github.com/japaric/cast.rs/issues/23
[#39]: https://github.com/japaric/cast.rs/issues/39
[#42]: https://github.com/japaric/cast.rs/pull/42
[#43]: https://github.com/japaric/cast.rs/pull/43

### Removed

- (breaking change) The `x128` Cargo feature has been removed
- removed `rustc_version` and `semver` build dependencies ([#35], [#37])
- removed all internal use of `unsafe` code ([#41])

[#35]: https://github.com/japaric/cast.rs/issues/35
[#41]: https://github.com/japaric/cast.rs/pull/41

## [v0.2.7] - 2021-07-03

### Changed

- Bumped `rustc_version` dependency to v0.4.0

## [v0.2.6] - 2021-05-15

### Changed

- Bumped `rustc_version` dependency

## [v0.2.5] - 2021-04-13

### Fixed

- Build on platforms with 32-bit pointers

## [v0.2.4] - 2021-04-11 - YANKED

## [v0.2.3] - 2018-11-17

### Changed

- Documented the guaranteed MSRV to be 1.13
- The `x128` feature now works on *stable* Rust 1.26+

### Fixed

- Overflow and underflow checks when casting a float to an unsigned integer

## [v0.2.2] - 2017-05-07

### Fixed

- UB in the checked cast from `f32` to `u128`.

## [v0.2.1] - 2017-05-06

### Added

- Support for 128-bit integers, behind the `x128` Cargo feature (nightly
  needed).

## [v0.2.0] - 2017-02-08

### Added

- Now `cast::Error` implements the `std::error::Error` trait among other traits
  like `Display`, `Clone`, etc.

### Changed

- [breaking-change] This crate now depends on the `std` crate by default but you
  can make it `no_std` by opting out of the `std` Cargo feature.

## v0.1.0 - 2016-02-07

Initial release

[Unreleased]: https://github.com/japaric/cast.rs/compare/v0.3.0...HEAD
[v0.3.0]: https://github.com/japaric/cast.rs/compare/v0.2.7...v0.3.0
[v0.2.7]: https://github.com/japaric/cast.rs/compare/v0.2.6...v0.2.7
[v0.2.6]: https://github.com/japaric/cast.rs/compare/v0.2.5...v0.2.6
[v0.2.5]: https://github.com/japaric/cast.rs/compare/v0.2.4...v0.2.5
[v0.2.4]: https://github.com/japaric/cast.rs/compare/v0.2.3...v0.2.4
[v0.2.3]: https://github.com/japaric/cast.rs/compare/v0.2.2...v0.2.3
[v0.2.2]: https://github.com/japaric/cast.rs/compare/v0.2.1...v0.2.2
[v0.2.1]: https://github.com/japaric/cast.rs/compare/v0.2.0...v0.2.1
[v0.2.0]: https://github.com/japaric/cast.rs/compare/v0.1.0...v0.2.0
