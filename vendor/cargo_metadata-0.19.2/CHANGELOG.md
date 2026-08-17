# Changelog

## Unreleased

- n/a

### Added

- n/a

### Changed

- Updated dependencies:
  - `thiserror` from `1.0.31` to `2.0.3`
  - `derive_builder` from `0.12` to `0.20`

### Removed

- n/a

### Fixed

- n/a

## [0.19.0] - 2024-11-20

### Added

- Re-exported `semver` crate directly.
- Added implementation of `std::ops::Index<&PackageId>` for `Resolve`.
- Added `pub fn is_kind(&self, name: TargetKind) -> bool` to `Target`.
- Added derived implementations of `PartialEq`, `Eq` and `Hash` for `Metadata` and its members' types.
- Added default fields to `PackageBuilder`.
- Added `pub fn new(name:version:id:path:) -> Self` to `PackageBuilder` for providing all required fields upfront.

### Changed

- Bumped MSRV from `1.42.0` to `1.56.0`.
- Made `parse_stream` more versatile by accepting anything that implements `Read`.
- Converted `TargetKind` and `CrateType` to an enum representation.

### Removed

- Removed re-exports for `BuildMetadata` and `Prerelease` from `semver` crate.
- Removed `.is_lib(…)`, `.is_bin(…)`, `.is_example(…)`, `.is_test(…)`, `.is_bench(…)`, `.is_custom_build(…)`, and `.is_proc_macro(…)` from `Target` (in favor of adding `.is_kind(…)`).

### Fixed

- Added missing `manifest_path` field to `Artifact`. Fixes #187.

## [0.15.0] - 2022-06-22

### Added

- Re-exported `BuildMetadata` and `Prerelease` from `semver` crate.
- Added `workspace_packages` function.
- Added `Edition` enum to better parse edition field.
- Added `rust-version` field to Cargo manifest.

### Changed

- Bumped msrv from `1.40.0` to `1.42.0`.

### Internal Changes

- Updated `derive_builder` to the latest version.
- Made use of `matches!` macros where possible.
- Fixed some tests

## [0.15.1] - 2022-10-13

### Added

- Added `TestMessage`, `TestEvent`, `SuiteEvent` for parsing the `cargo test -- --format json` output.
