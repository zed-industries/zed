# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.8.0] - 2025-09-15

### Changed
- Bump MSRV to v1.81 as required by `time-core` v0.1.6.
- Don't publish test data to crates.io (#164).
- Publish using crates.io Trusted Publishing.

### Fixed
- Read binary plists with 24-bit integer offset tables (#165).

## [1.7.4] - 2025-07-07

### Changed
- Update `quick-xml` to v0.38.
- Run `cargo-semver-checks` on pull requests.

## [1.7.3] - 2025-07-05

### Fixed
- Fail deserialisation if input is not completely consumed (#149).

## [1.7.2] - 2025-06-11

### Changed
- Update `quick-xml` to v0.37.

[unreleased]: https://github.com/ebarnard/rust-plist/compare/v1.8.0...HEAD
[1.8.0]: https://github.com/ebarnard/rust-plist/compare/v1.7.4...v1.8.0
[1.7.4]: https://github.com/ebarnard/rust-plist/compare/v1.7.3...v1.7.4
[1.7.3]: https://github.com/ebarnard/rust-plist/compare/v1.7.2...v1.7.3
[1.7.2]: https://github.com/ebarnard/rust-plist/compare/v1.7.1...v1.7.2
