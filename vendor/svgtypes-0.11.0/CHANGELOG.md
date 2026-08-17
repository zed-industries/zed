# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [0.11.0] - 2023-03-25
### Added
- `SimplifyingPathParser` handles implicit MoveTo commands after ClosePath now.
  Previously, `M 10 20 L 30 40 Z L 50 60` would have been parsed as is,
  but now it will be parsed as `M 10 20 L 30 40 Z M 10 20 L 50 60`.

## [0.10.0] - 2023-02-04
### Changed
- Bump `kurbo`
- Bump MSRV to 1.65 (because of `kurbo`)

## [0.9.0] - 2022-12-25
### Added
- `SimplifyingPathParser` that allows parsing an already simplified Path Data.

## [0.8.2] - 2022-10-23
### Added
- `paint-order`

## [0.8.1] - 2022-06-11
### Added
- Support #RRGGBBAA and #RGBA color notation as per CSS Color 4.
  [@demurgos](https://github.com/demurgos)

## [0.8.0] - 2021-09-12
### Added
- `EnableBackground`
- `Number`. Previously accessible via `Steam::parse_number`.
- `IRI`. Previously accessible via `Steam::parse_iri`.
- `FuncIRI`. Previously accessible via `Steam::parse_func_iri`.

### Changed
- `Stream` is now private.

## [0.7.0] - 2021-09-04

**Breaking**: Almost a complete rewrite. This crate is strictly a parser from now.

### Added
- [`<filter-value-list>`](https://www.w3.org/TR/filter-effects-1/#typedef-filter-value-list)
  parsing using `FilterValueListParser`.
- `ViewBoxError`

### Removed
- Writing support.
- Container types. Only stack allocated types and pull-based parsers are available.
- `FuzzyEq` and `FuzzyZero`.

## [0.6.0] - 2021-08-22
### Added
- CSS3 colors support (`rgba`, `hsl`, `hsla`, `transparent`).
- `turn` angle unit.
- `Angle::to_degrees`.

### Changed
- Move to Rust 2018 edition.
- Rename `Stream::skip_string` into `Stream::consume_string`.
- Rename `Color::new` into `Color::new_rgb`.
- `Color` struct gained an `alpha` field.
- Rename `Angle::num` into `Angle::number`.
- Rename `Length::num` into `Length::number`.

## [0.5.0] - 2019-08-12
### Added
- Implement `Default` for `Length`, `LengthList`, `NumberList`, `Points` and `Path`.

### Changed
- The minimum Rust version is 1.31

### Removed
- `PathBuilder`. Use `Path::push_*` instead.
- `Style` parser. Use an external CSS parser instead, like `simplecss`.
- `ElementId` and `AttributeId`.
- `phf` dependency. Only `siphasher` is used now.

## [0.4.4] - 2019-06-11
- Update `float-cmp`.

## [0.4.3] - 2019-06-10
### Added
- `Transform::prepend`.
- Implement `FuzzyEq` and `FuzzyZero` for `f32`.
- Parsing of `Color`, `Paint`, `ElementId` and `AttributeId` can be disabled now.

## [0.4.2] - 2019-03-15
### Changed
- The `XmlByteExt` trait is private now.

## [0.4.1] - 2019-01-06
### Fixed
- Style with comments parsing.

## [0.4.0] - 2019-01-02
### Added
- An [`angle`](https://www.w3.org/TR/SVG11/types.html#DataTypeAngle) value type.

### Changed
- `Length::from_str` will return an error if an input string has trailing data.
  So length like `1mmx` was previously parsed without errors.

## [0.3.0] - 2018-12-13
### Changed
- `PathParser` will return `Result<PathSegment>` instead of `PathSegment` from now.
- `Error` was rewritten.

### Removed
- `FromSpan` trait. Use `FromStr`.
- `StrSpan`. All strings are `&str` now.
- `TextPos`. All errors have position in characters now.
- `xmlparser` dependency.
- `log` dependency.

## [0.2.0] - 2018-09-12
### Added
- `black`, `white`, `gray`, `red`, `green` and `blue` constructors to the `Color` struct.

### Changed
- `StyleParser` will return `(StrSpan, StrSpan)` and not `StyleToken` from now.
- `StyleParser` requires entity references to be resolved before parsing from now.

### Removed
- `failure` dependency.
- `StyleToken`.
- `Error::InvalidEntityRef`.

## [0.1.1] - 2018-05-23
### Added
- `encoding` and `standalone` to AttributeId.
- `new_translate`, `new_scale`, `new_rotate`, `new_rotate_at`, `new_skew_x`, `new_skew_y`
  and `rotate_at` methods to the `Transform`.

### Changed
- `StreamExt::parse_iri` and `StreamExt::parse_func_iri` will parse
  not only well-formed data now.

### Fixed
- `Paint::from_span` poor performance.

[Unreleased]: https://github.com/RazrFalcon/svgtypes/compare/v0.11.0...HEAD
[0.11.0]: https://github.com/RazrFalcon/svgtypes/compare/v0.10.0...v0.11.0
[0.10.0]: https://github.com/RazrFalcon/svgtypes/compare/v0.9.0...v0.10.0
[0.9.0]: https://github.com/RazrFalcon/svgtypes/compare/v0.8.2...v0.9.0
[0.8.2]: https://github.com/RazrFalcon/svgtypes/compare/v0.8.1...v0.8.2
[0.8.1]: https://github.com/RazrFalcon/svgtypes/compare/v0.8.0...v0.8.1
[0.8.0]: https://github.com/RazrFalcon/svgtypes/compare/v0.7.0...v0.8.0
[0.7.0]: https://github.com/RazrFalcon/svgtypes/compare/v0.6.0...v0.7.0
[0.6.0]: https://github.com/RazrFalcon/svgtypes/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/RazrFalcon/svgtypes/compare/v0.4.4...v0.5.0
[0.4.4]: https://github.com/RazrFalcon/svgtypes/compare/v0.4.3...v0.4.4
[0.4.3]: https://github.com/RazrFalcon/svgtypes/compare/v0.4.2...v0.4.3
[0.4.2]: https://github.com/RazrFalcon/svgtypes/compare/v0.4.1...v0.4.2
[0.4.1]: https://github.com/RazrFalcon/svgtypes/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/RazrFalcon/svgtypes/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/RazrFalcon/svgtypes/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/RazrFalcon/svgtypes/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/RazrFalcon/svgtypes/compare/v0.1.0...v0.1.1
