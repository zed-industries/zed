# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.8.0]
- Add override-variables
- Bump MSRV to 1.81 (due to dependencies)

## [0.7.7]
- Fix `NUM_JOB` to 1 if `SOURCE_DATE_EPOCH` is set, in order to better support reproducible builds
- Set MSRV to 1.74 (due to dependencies)

## [0.7.6]
- Do not depend on `fmt::Debug`-output (`fmt-debug=none`)
- Bump `git2` to 0.20
- Use `static`- instead of `const`-items throughout

## [0.7.5] - 2024-10-17
### Changed
- Bump `cargo-lock` to 10.0

## [0.7.4] - 2024-07-07
### Added
- Honor `SOURCE_DATE_EPOCH` in `BUILT_TIME_UTC`

### Changed
- Bump `git2` to 0.19

## [0.7.3] - 2024-05-21
### Added
- Search for lockfile in manifest's parent directory (crates in workspaces)

## [0.7.2] - 2024-04-09
### Changed
- Fixed hard error in case `rustdoc` is missing

## [0.7.1] - 2023-10-14
### Changed
- Fixed `no_std` builds

## [0.7.0] - 2023-08-09
### Changed
- The `Options`-type has been removed in favor of controlling `built`'s behavior by means of feature-flags.
- `cargo-lock` is now an optional dependency
- Bump `git2` to 0.18

## [0.6.1] - 2023-06-19
### Changed
- Bump `git2` to 0.17
- Bump `cargo-lock` to 9.0

### Added
- `FEATURES_LOWERCASE` and `FEATURES_LOWERCASE_STR`

## [0.6.0] - 2023-02-09
### Changed
- Identical re-release after yanking 0.5.3, due to semver failure

## [0.5.3] - 2023-02-08
### Changed
- Bump `git2` to 0.16, mitigating GHSA-8643-3wh5-rmjq

### Added
- Add `GIT_COMMIT_HASH_SHORT`

## [0.5.2] - 2022-12-03
### Changed
- Removed unused transitive dependency on `time`
- Bump `cargo-lock` to 8.0
- Bump `git2` to 0.15
- Fix unescaped quotes in literals

### Added
- Added GitHub Actions to the list of detected CI platforms

## [0.5.1] - 2021-05-27
### Changed
- Bump `cargo-lock` to 7.0
- Bump `semver` to 1.0

## [0.5.0] - 2021-05-01
### Fixed
- Fix possibly wrong `CFG_` values in cross-compilation scenarios
- Fix handling of backspaces in doc-attributes

### Changed
- Switch deprecated `tempdir` to `tempfile`
- Add `#allow(dead_code)` to generated code
- Bump `cargo-lock` to 6.0
- Bump `semver` to 0.11
- Publicly re-export dependencies

## [0.4.4] - 2020-12-01
### Added
- Added `PKG_LICENSE` and `PKG_REPOSITORY`

## [0.4.3] - 2020-08-19
### Fixed
- Fix handling of unescaped special characters

## [0.4.2] - 2020-05-27
### Added
- Add `GIT_DIRTY`
- Add `GIT_COMMIT_HASH` and `GIT_HEAD_REF`

### Changed
- Bump `semver` to 0.10

[unreleased]: https://github.com/lukaslueg/built/compare/0.8.0...master
[0.8.0]: https://github.com/lukaslueg/built/compare/0.7.7...0.8.0
[0.7.7]: https://github.com/lukaslueg/built/compare/0.7.6...0.7.7
[0.7.6]: https://github.com/lukaslueg/built/compare/0.7.5...0.7.6
[0.7.5]: https://github.com/lukaslueg/built/compare/0.7.4...0.7.5
[0.7.4]: https://github.com/lukaslueg/built/compare/0.7.3...0.7.4
[0.7.3]: https://github.com/lukaslueg/built/compare/0.7.2...0.7.3
[0.7.2]: https://github.com/lukaslueg/built/compare/0.7.1...0.7.2
[0.7.1]: https://github.com/lukaslueg/built/compare/0.7.0...0.7.1
[0.7.0]: https://github.com/lukaslueg/built/compare/0.6.1...0.7.0
[0.6.1]: https://github.com/lukaslueg/built/compare/0.6.0...0.6.1
[0.6.0]: https://github.com/lukaslueg/built/compare/0.5.3...0.6.0
[0.5.3]: https://github.com/lukaslueg/built/compare/0.5.2...0.5.3
[0.5.2]: https://github.com/lukaslueg/built/compare/0.5.1...0.5.2
[0.5.1]: https://github.com/lukaslueg/built/compare/0.5.0...0.5.1
[0.5.0]: https://github.com/lukaslueg/built/compare/0.4.4...0.5.0
[0.4.4]: https://github.com/lukaslueg/built/compare/0.4.3...0.4.4
[0.4.3]: https://github.com/lukaslueg/built/compare/0.4.2...0.4.3
[0.4.2]: https://github.com/lukaslueg/built/compare/0.4.1...0.4.2
