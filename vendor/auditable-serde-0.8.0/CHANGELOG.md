# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.8.0] - 2024-11-11
### Removed
- Removed the conversion from `cargo_metadata` structures. The `cargo_metadata` crate makes breaking changes quite frequently, and we need to be able to upgrade it without breaking semver on this crate.

## [0.7.0] - 2024-07-30
### Changed
- Removed the disabled-by-default conversion from the internal format to Cargo.lock. The Cargo.lock format is unstable, and the conversion to CycloneDX is a better idea these days.

## [0.6.1] - 2024-02-19
### Fixed
- `from_metadata` feature: Fixed creating a cyclic dependency graph under [certain conditions](https://github.com/rustsec/rustsec/issues/1043).

## [0.6.0] - 2023-04-27
### Changed
- `toml` feature: upgraded to `cargo-lock` crate v9.x

### Fixed
- Fixed changelog formatting

## [0.5.2] - 2022-10-24
### Changed
- `toml` feature: Versions are no longer roundtripped through `&str`, resulting in faster conversion.
- `toml` feature: `cargo_lock::Dependency.source` field is now populated when when converting into `cargo-lock` crate format.

### Added
- This changelog file

## [0.5.1] - 2022-10-02
### Added
- JSON schema (thanks to @tofay)
- A mention of the `auditable-info` crate in the crate documentation

## [0.5.0] - 2022-08-08
### Changed
- This is the first feature-complete release
