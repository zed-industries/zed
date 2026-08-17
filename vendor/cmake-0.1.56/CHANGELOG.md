# Changelog

## [Unreleased]

## [0.1.56](https://github.com/rust-lang/cmake-rs/compare/v0.1.55...v0.1.56) - 2025-12-13

### Other

- Fallback to bundled CMake if present ([#264](https://github.com/rust-lang/cmake-rs/pull/264))
- Use `cmake -B` only for v3.13 and later ([#262](https://github.com/rust-lang/cmake-rs/pull/262))

## [0.1.55](https://github.com/rust-lang/cmake-rs/compare/v0.1.54...v0.1.55) - 2025-12-11

### Other

- Remove the `\\?\` prefix from paths ([#259](https://github.com/rust-lang/cmake-rs/pull/259))
- Add Visual Studio 2026 support ([#255](https://github.com/rust-lang/cmake-rs/pull/255))
- Make sure that cmake generate build files in current dir ([#194](https://github.com/rust-lang/cmake-rs/pull/194))
- Set the MSRV to 1.65 and test this in CI
- Canonicalize the build directory
- Use `eprintln` instead to print the command running next ([#191](https://github.com/rust-lang/cmake-rs/pull/191))

## [0.1.54](https://github.com/rust-lang/cmake-rs/compare/v0.1.53...v0.1.54) - 2025-02-10

### Other

- Remove workaround for broken `cc-rs` versions ([#235](https://github.com/rust-lang/cmake-rs/pull/235))
- Be more precise in the description of `register_dep` ([#238](https://github.com/rust-lang/cmake-rs/pull/238))

## [0.1.53](https://github.com/rust-lang/cmake-rs/compare/v0.1.52...v0.1.53) - 2025-01-27

### Other

- Disable broken Make jobserver support on OSX to fix parallel builds ([#229](https://github.com/rust-lang/cmake-rs/pull/229))

## [0.1.52](https://github.com/rust-lang/cmake-rs/compare/v0.1.51...v0.1.52) - 2024-11-25

### Other

- Expose cc-rs no_default_flags for hassle-free cross-compilation ([#225](https://github.com/rust-lang/cmake-rs/pull/225))
- Add a `success` job to CI
- Change `--build` to use an absolute path
- Merge pull request [#195](https://github.com/rust-lang/cmake-rs/pull/195) from meowtec/feat/improve-fail-hint
- Improve hint for cmake not installed in Linux (code 127)

## [0.1.51](https://github.com/rust-lang/cmake-rs/compare/v0.1.50...v0.1.51) - 2024-08-15

### Added

- Add JOM generator support ([#183](https://github.com/rust-lang/cmake-rs/pull/183))
- Improve visionOS support ([#209](https://github.com/rust-lang/cmake-rs/pull/209))
- Use `Generic` for bare-metal systems ([#187](https://github.com/rust-lang/cmake-rs/pull/187))

### Fixed

- Fix cross compilation on android armv7 and x86 ([#186](https://github.com/rust-lang/cmake-rs/pull/186))

