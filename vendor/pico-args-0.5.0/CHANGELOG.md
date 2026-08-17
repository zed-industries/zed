# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [0.5.0] - 2022-06-04
### Changed
- The `eq-separator` build feature is no longer enabled by default.
- Small changes to the `Display` output for `Error`.

## [0.4.2] - 2021-06-03
### Fixed
- Ignore long options when parsing short options when `combined-flags` feature is enabled.
  Thanks to [@riquito](https://github.com/riquito).
- `Arguments::contains` docs. Thanks to [@jneem](https://github.com/jneem).

## [0.4.1] - 2021-05-03
### Added
- `combined-flags` feature. Thanks to [@alexwennerberg](https://github.com/alexwennerberg).

## [0.4.0] - 2021-01-03
### Added
- `Arguments::opt_free_from_*`.

### Changed
- `Arguments::finish` no longer returns an error and simply returns remaining arguments as is.
- `Arguments::free_from_*` methods no longer check that argument doesn't start with `-`.
- `Arguments::free_from_*` methods return `T` instead of `Option<T>` now.
  Use `Arguments::opt_free_from_*` instead.

### Removed
- `Arguments::free` and `Arguments::free_os`. You should use `Arguments::free_from_*` methods
  or parse them manually after calling `Arguments::finish`.
- `Error::UnusedArgsLeft`. This should be handled by the caller now.

## [0.3.4] - 2020-08-09
### Added
- `short-space-opt` build feature. Thanks to [@hdamron17](https://github.com/hdamron17).

## [0.3.3] - 2020-06-26
### Added
- `values_from_str`, `values_from_fn` and `values_from_os_str`.<br>
  Those functions can be used to parse arguments like:<br>
  `--file /path1 --file /path2 --file /path3`<br>
  But not `--file /path1 /path2 /path3`.

## [0.3.2] - 2020-06-15
### Added
- `eq-separator` build feature.

## [0.3.1] - 2020-01-08
### Added
- `Arguments::subcommand`. Thanks to [@matklad](https://github.com/matklad).

## [0.3.0] - 2019-09-23
### Added
- Required arguments support.
- `Error::MissingOption` when option is required but not present.

### Changed
- Rename `value_from_str` into `opt_value_from_str`.
- Rename `value_from_fn` into `opt_value_from_fn`.
- Rename `value_from_os_str` into `opt_value_from_os_str`.
- `value_from_str`, `value_from_fn` and `value_from_os_str` will return `T` and not `Option<T>`
  from now.

## [0.2.0] - 2019-07-26
### Added
- Non UTF-8 arguments support.
- `free_from_str`, `free_from_fn` and `free_from_os_str`.
- `value_from_os_str`.

### Changed
- `value_from_fn` allows any error type that implements `Display` now
  and not only `String`.
- `from_args` -> `from_vec`. And it accepts `Vec<OsString>` now.
- The `Error` enum.

### Fixed
- Do not panic while parsing non UTF-8 arguments.

[Unreleased]: https://github.com/RazrFalcon/pico-args/compare/v0.5.0...HEAD
[0.5.0]: https://github.com/RazrFalcon/pico-args/compare/v0.4.2...v0.5.0
[0.4.2]: https://github.com/RazrFalcon/pico-args/compare/v0.4.1...v0.4.2
[0.4.1]: https://github.com/RazrFalcon/pico-args/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/RazrFalcon/pico-args/compare/v0.3.4...v0.4.0
[0.3.4]: https://github.com/RazrFalcon/pico-args/compare/v0.3.3...v0.3.4
[0.3.3]: https://github.com/RazrFalcon/pico-args/compare/v0.3.2...v0.3.3
[0.3.2]: https://github.com/RazrFalcon/pico-args/compare/v0.3.1...v0.3.2
[0.3.1]: https://github.com/RazrFalcon/pico-args/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/RazrFalcon/pico-args/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/RazrFalcon/pico-args/compare/v0.1.0...v0.2.0
