# Change Log

All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/).

## [v0.4.3] - 2025-12-14

### Added

- Support indexing with most `core::ops::Range*` types.

### Changed

### Removed

## [v0.4.2] - 2024-03-24

### Added

- Add `A1` type
- Make the `Alignment` trait public, but unimplementable
- Add some `Borrow`/`BorrowMut` trait impls

### Changed

### Removed

## [v0.4.1] - 2022-05-30

### Added

- `Aligned<A, T>` implements `Copy` if `T` implements `Copy`

## [v0.4.0] - 2021-07-24

### Changed

- `as-slice` dependency bumped to 0.2.0

### Removed

- [breaking-change] removed compatibility withe version 0.1.0 of `AsSlice` and `AsMutSlice` traits (`as-slice` crate)

## [v0.3.5] - 2021-07-17

- added `A32` and `A64` types

## [v0.3.4] - 2020-07-31

### Added

- `Aligned` now implements the `PartialEq`, `Eq`, `PartialOrd`, `Ord` and `Hash` traits.

## [v0.3.3] - 2020-07-28

### Added

- `Aligned` now implements the `Clone`, `Default`, `Debug` and `Display` traits.
- `Aligned` has been marked `repr(C)`

## [v0.3.2] - 2019-11-26

### Added

- `Aligned<_, [T]>` now implements the `Index<RangeTo<usize>>` trait; slicing
  this value to end returns an `Aligned<_, [T]>` slice.

## [v0.3.1] - 2018-11-07

### Changed

- Make deref work on Aligned<Ax, $DST>.

## [v0.3.0] - 2018-11-05

### Changed

- [breaking-change] the alignment type parameter of `Aligned` must now be one
  of: `A2`, `A4`, `A8` or `A16`.

### Removed

- [breaking-change] removed the `const-fn` feature. Const functions are now
  provided by default. This crate now requires Rust 1.31+ to build.

## [v0.2.0] - 2018-05-10

### Changed

- [breaking-change] `const-fn` is no longer a default feature (i.e. a feature that's enabled by
  default). The consequence is that this crate now compiles on 1.27 (beta) by default, and opting
  into `const-fn` requires nightly.

## [v0.1.2] - 2018-04-25

### Added

- an opt-out "const-fn" Cargo feature. Disabling this feature removes all `const` constructors and
  makes this crate compilable on stable.

## [v0.1.1] - 2017-05-30

### Added

- support for aligned slices

## v0.1.0 - 2017-05-29

- Initial release

[Unreleased]: https://github.com/japaric/aligned/compare/v0.4.1...HEAD
[v0.4.1]: https://github.com/japaric/aligned/compare/v0.4.0...v0.4.1
[v0.4.0]: https://github.com/japaric/aligned/compare/v0.3.5...v0.4.0
[v0.3.5]: https://github.com/japaric/aligned/compare/v0.3.4...v0.3.5
[v0.3.4]: https://github.com/japaric/aligned/compare/v0.3.3...v0.3.4
[v0.3.3]: https://github.com/japaric/aligned/compare/v0.3.2...v0.3.3
[v0.3.2]: https://github.com/japaric/aligned/compare/v0.3.1...v0.3.2
[v0.3.1]: https://github.com/japaric/aligned/compare/v0.3.0...v0.3.1
[v0.3.0]: https://github.com/japaric/aligned/compare/v0.2.0...v0.3.0
[v0.2.0]: https://github.com/japaric/aligned/compare/v0.1.2...v0.2.0
[v0.1.2]: https://github.com/japaric/aligned/compare/v0.1.1...v0.1.2
[v0.1.1]: https://github.com/japaric/aligned/compare/v0.1.0...v0.1.1
