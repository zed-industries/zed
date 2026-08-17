# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.4](https://github.com/eyre-rs/indenter/compare/v0.3.3...v0.3.4) - 2025-08-06
### Fixed
- Fix line number being reset on each Indented write_str call
- Set MSRV (1.39), remove conflicting lints, revert use of names args.
### Other
- add release-plz to ci ([#23](https://github.com/eyre-rs/indenter/pull/23))
- Remove unused flag.
- Denote in the "cargo check" job name that it uses the stable toolchain
- Appease Clippy.
- Fix and modernize GitHub CI Workflow
- include LICENSE files for Apache-2.0 and MIT ([#20](https://github.com/eyre-rs/indenter/pull/20))
- Fix formatting of 'Cargo.toml' file ([#13](https://github.com/eyre-rs/indenter/pull/13))

## [0.3.3] - 2021-02-22
### Added
- Implement new code dedenting / indenting formatter by cecton

## [0.3.2] - 2021-01-04
### Fixed
- Changed indentation logic to better support trailing newlines and improve
  overall formatting consistency

## [0.3.1] - 2020-12-21
### Added
- `with_str` helper method for indenting with static strings
### Changed
- Relaxed `Sized` bound on inner writers


<!-- next-url -->
[Unreleased]: https://github.com/yaahc/indenter/compare/v0.3.3...HEAD
[0.3.3]: https://github.com/yaahc/indenter/compare/v0.3.2...v0.3.3
[0.3.2]: https://github.com/yaahc/indenter/compare/v0.3.1...v0.3.2
[0.3.1]: https://github.com/yaahc/indenter/releases/tag/v0.3.1
