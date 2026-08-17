# Change Log

## [v0.3.0] - 2023-05-22

### Removed

* `FromFallibleIterator` trait is removed, `FallibleIterator::collect` now requires `std::iter::FromIterator`,
  which has strictly more implementations.

### Added

* add convenient wrapper for std::iter::Iterator
* more utilities mirroring std::iter
* Peekable methods
* `IteratorExt` for convenient conversion
* `FallibleIterator::unwrap`

## [v0.2.0] - 2019-03-10

### Changed

* All closures used by adaptor methods now return `Result`s.
* `FromFallibleIterator::from_fallible_iterator` has been renamed to `from_fallible_iter` and now takes an
    `IntoFallibleIterator`.
* `IntoFallibleIterator::into_fallible_iterator` has been renamed to `into_fallible_iter`.
* `IntoFallibleIterator::IntoIter` has been renamed to `IntoFallibleIter`.

### Removed

* `FallibleIterator::and_then` has been removed as `FallibleIterator::map` is now equivalent.

### Added

* Added `step_by`, `for_each`, `skip_while`, `take_while`, `skip`, `scan`, `flat_map`, `flatten`, `inspect`,
    `partition`, `find_map`, `max_by`, `min_by`, `unzip`, `cycle`, and `try_fold` to `FallibleIterator`.
* Added `rfold` and `try_rfold` to `DoubleEndedFallibleIterator`.

[Unreleased]: https://github.com/sfackler/rust-fallible-iterator/compare/v0.2.0...master
[v0.2.0]: https://github.com/sfackler/rust-fallible-iterator/compare/v0.1.5...v0.2.0
