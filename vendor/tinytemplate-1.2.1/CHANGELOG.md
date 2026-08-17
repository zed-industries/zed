# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.2.1] - 2021-03-03
### Fixed
- Fixed a compile error on some nightly compiler versions.

## [1.2.0] - 2020-01-03
### Fixed
 - Fixed numeric values being truthy when zero, rather than when non-zero. (For real this time)
### Added
 - Allow numeric indexes to be used in paths, to index into JSON arrays.

## [1.1.0] - 2020-05-31
  - Added `TinyTemplate::set_default_formatter` which, for example, allows to dissable HTML-scaping

## [1.0.4] - 2020-04-25
### Added
- Added `@root` keyword which allows printing, branching on or iterating over the root context
  object. This is saves having to wrap simple context values in a struct.

## [1.0.3] - 2019-12-26
### Fixed
- Fixed the @last keyword never evaluating to true
- Fixed numeric values being truthy when zero, rather than when non-zero.

## [1.0.2] - 2019-05-16
### Fixed
- Fixed possible panic when compiling templates with escaped curly braces.

## [1.0.1] - 2019-01-19
### Added
- Added support for older versions of Rust (back to 1.26).

## 1.0.0 - 2019-01-19
### Added
- Initial release on Crates.io.

[Unreleased]: https://github.com/bheisler/TinyTemplate/compare/1.2.0...HEAD
[1.0.1]: https://github.com/bheisler/TinyTemplate/compare/1.0.0...1.0.1
[1.0.2]: https://github.com/bheisler/TinyTemplate/compare/1.0.1...1.0.2
[1.0.3]: https://github.com/bheisler/TinyTemplate/compare/1.0.2...1.0.3
[1.0.4]: https://github.com/bheisler/TinyTemplate/compare/1.0.3...1.0.4
[1.1.0]: https://github.com/bheisler/TinyTemplate/compare/1.0.4...1.1.0
[1.2.0]: https://github.com/bheisler/TinyTemplate/compare/1.1.0...1.2.0
[1.2.1]: https://github.com/bheisler/TinyTemplate/compare/1.2.0...1.2.1
