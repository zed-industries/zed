## 0.8.0

### Breaking Changes

- The minimum supported Rust version has been increased to 1.84.0. ([\#612](https://github.com/proptest-rs/proptest/pull/612))

## 0.7.0

### Breaking Changes

- The minimum supported Rust version has been increased to 1.82.0. ([\#605](https://github.com/proptest-rs/proptest/pull/605))

## 0.6.0

### Other Notes

- Fixed URLs in proptest-derive error messages ([\#574](https://github.com/proptest-rs/proptest/pull/574))
- Updated `rand` dependency from 0.8 to 0.9.
- Bump all dependencies to latest compatible with MSRV 1.66.

## 0.5.1

- Fix non-local impl nightly warning with allow(non_local_definitions)
  ([\#531](https://github.com/proptest-rs/proptest/pull/531))
- Adds support for re-exporting crate. `proptest-derive` now works correctly
  when `proptest` is re-exported from another crate. This removes the
  requirement for `proptest` to be a direct dependency.
  ([\#530](https://github.com/proptest-rs/proptest/pull/530))
- Fix bounds generation for generics in derive(Arbitrary). The implementation
  of UseTracker expects that iteration over items of used_map gives items in
  insertion order. However, the order of BTreeSet is based on Ord, not
  insertion. ([\#511](https://github.com/proptest-rs/proptest/pull/511))

## 0.5

### Features

- Add `boxed_union` feature which when turned on uses heap allocation for
  `#[derive(Arbitrary)]` strategy synthesis preventing stack overflow for
  exceptionally large structures.

### Dependencies

- Upgraded `syn` to 2.x
- Upgraded `compiletest_rs` 0.10 to 0.11

### Other Notes

- Fixed various clippies and diagnostic issues

### 0.4.0

### Other Notes

- Upgraded `compiletest_rs` from 0.9 to 0.10
- Upgraded `syn`, `quote`, and `proc-macro2` to 1.0

## 0.3.0

### Breaking changes

- The minimum supported Rust version has been increased to 1.50.0.

### Bug Fixes

- Certain `enum`s could not be derived before, and now can be.

- Structs with more than 10 fields can now be derived.

## 0.2.0

### Breaking changes

- Generated code now requires `proptest` 0.10.0.

## 0.1.2

### Other Notes

- Derived enums now use `LazyTupleUnion` instead of `TupleUnion` for better
  efficiency.

## 0.1.1

This is a minor release to correct a packaging error. The license files are now
included in the files published to crates.io.
