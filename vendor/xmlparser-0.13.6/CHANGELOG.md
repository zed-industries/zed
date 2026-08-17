# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [0.13.6] - 2023-09-30
### Added
- `Token::span`, `Tokenizer::stream` and allow cloning of `Tokenizer`.
  Thanks to [@krtab](https://github.com/krtab).

### Changed
- Optimize `is_xml_char` function. Makes parsing 5-10% faster.
  Thanks to [@Simon-Martens](https://github.com/Simon-Martens).

## [0.13.5] - 2022-10-18
### Fixed
- Do no use recursive calls during parsing. Could lead to stack overflow on some input.
- Revert _Do not expand predefined references in `Stream::consume_reference`._
- Tests on Rust 1.61. Thanks to [@krtab](https://github.com/krtab).

## [0.13.4] - 2021-06-24
### Fixed
- Do not expand predefined references in `Stream::consume_reference`.
  Thanks to [@Jesse-Bakker](https://github.com/Jesse-Bakker).

## [0.13.3] - 2020-09-02
### Changed
- Documentation fixes by [@kneasle](https://github.com/kneasle).

### Fixed
- `DtdEnd` token parsing when `]` and `>` are separated by a whitespace.

## [0.13.2] - 2020-06-15
### Fixed
- Allow processing instruction before DTD.

## [0.13.1] - 2020-03-12
### Fixed
- Allow comments before DTD.

## [0.13.0] - 2020-01-07
### Changed
- Moved to Rust 2018.
- Completely new `Error` enum.
- New error messages.
- 10-20% faster parsing.
- Use `Tokenizer::from_fragment` instead of `Tokenizer::enable_fragment_mode`.

### Removed
- `TokenType`.

## [0.12.0] - 2019-12-21
### Changed
- `]]>` is no longer allowed inside a Text node.
- Only [XML characters](https://www.w3.org/TR/xml/#char32) are allowed now.
  Otherwise, `StreamError::NonXmlChar` will occur.
- Disallow `-` at the end of a comment. `<!--a--->` is an error now.
- A missing space between attributes is an error now.
- `StreamError::InvalidQuote` and `StreamError::InvalidSpace` signature changed.

## [0.11.0] - 2019-11-18
### Added
- `no_std` support thanks to [hugwijst](https://github.com/hugwijst).

### Changed
- `StreamError::InvalidString` doesn't store an actual string now.

## [0.10.0] - 2019-09-14
### Changed
- 10-15% faster parsing.
- Merge `ByteStream` and `Stream`.
- `StreamError::InvalidChar` signature changed.
- `StreamError::InvalidChar` was split into `InvalidChar` and `InvalidCharMultiple`.

### Fixed
- Check for [NameStartChar](https://www.w3.org/TR/xml/#NT-NameStartChar)
  during qualified name parsing.

  E.g. `<-p>` is an invalid tag name from now.
- Qualified name with multiple `:` is an error now.
- `]>` is a valid text/`CharData` now. Previously it was parsed as `DoctypeEnd`.

### Removed
- `StreamError::InvalidAttributeValue`. `StreamError::InvalidChar` will be emitted instead.

## [0.9.0] - 2019-02-27
### Added
- `span` field to all `Token` variants, which contains a whole token span in bytes.
- `Stream::try_consume_byte`.

### Changed
- All `Token` variants are structs now and not tuples.
- `StrSpan` contains an actual string span an not only region now.

  So we can use a non-panic and zero-cost `StrSpan::as_str` instead
  of `StrSpan::to_str`, that was performing slicing each time.
- Split `Stream` into `ByteStream` and `Stream`.
- `Stream::skip_spaces` will parse only ASCII whitespace now.
- Rename `StrSpan::to_str` into `StrSpan::as_str`.
- Rename `Reference::EntityRef` into `Reference::Entity`.
- Rename `Reference::CharRef` into `Reference::Char`.
- `StrSpan::from_substr` and `StrSpan::slice_region` are private now.

### Removed
- `Token::Whitespaces`. Will be parsed as `Token::Text`.
- `Stream::curr_char`.
- `Stream::is_curr_byte_eq`.
- `Stream::consume_either`.
- `Stream::skip_ascii_spaces`. Use `Stream::skip_spaces` instead.
- `StrSpan::trim`.
- `StrSpan::len`.
- `StrSpan::full_len`.
- `StrSpan::as_bytes`.

### Fixed
- Declaration attributes with mixed quotes parsing.

## [0.8.1] - 2019-01-02
### Changed
- Changed the crate category in the Cargo.toml

## [0.8.0] - 2018-12-13
### Added
- `Error::pos()`.

### Changed
- Rename `Stream::gen_error_pos` into `Stream::gen_text_pos`.
- Rename `Stream::gen_error_pos_from` into `Stream::gen_text_pos_from`.
- `Stream::gen_text_pos` speed up.

### Fixed
- `TextPos` is Unicode aware now.
- XML declaration parsing when file has a BOM.

## [0.7.0] - 2018-10-29
### Changed
- `<` inside an attribute value is an error now.
- `Token::Declaration` represents *standalone* as `bool` now.
- XML declaration must be defined only once now.
- XML declaration must start at 0 position.
- DTD must be defined only once now.

## [0.6.1] - 2018-10-08
### Added
- `Stream::curr_byte_unchecked`.

### Fixed
- UTF-8 BOM processing.

## [0.6.0] - 2018-08-31
### Changed
- `Reference::EntityRef` contains `&str` and not `StrSpan` now.
- Rename `Stream::try_consume_char_reference` into `try_consume_reference`.
  And it will return `Reference` and not `char` now.
- Rename `Tokenizer::set_fragment_mode` into `enable_fragment_mode`.
- Rename `ErrorPos` into `TextPos`.

### Fixed
- `TextPos` calculation via `Stream::gen_error_pos`.

### Removed
- `TextUnescape` and `XmlSpace` because useless.

## [0.5.0] - 2018-06-14
### Added
- `StreamError::InvalidChar`.
- `StreamError::InvalidSpace`.
- `StreamError::InvalidString`.

### Changed
- `Stream::consume_reference` will return only `InvalidReference` error from now.
- `Error::InvalidTokenWithCause` merged into `Error::InvalidToken`.
- `Stream::gen_error_pos_from` does not require `mut self` from now.
- `StreamError::InvalidChar` requires `Vec<u8>` and not `String` from now.
- `ErrorPos` uses `u32` and not `usize` from now.

### Removed
- `failure` dependency.
- `log` dependency.

## [0.4.1] - 2018-05-23
### Added
- An ability to parse an XML fragment.

## [0.4.0] - 2018-04-21
### Changed
- Relicense from MIT to MIT/Apache-2.0.

### Removed
- `FromSpan` trait.
- `from_str` and `from_span` methods are removed. Use the `From` trait instead.

## [0.3.0] - 2018-04-10
### Changed
- Use `failure` instead of `error-chain`.
- Minimum Rust version is 1.18.
- New error messages.
- `TokenType` is properly public now.

### Removed
- `ChainedError`

## [0.2.0] - 2018-03-11
### Added
- Qualified name parsing.

### Changed
- **Breaking**. `Token::ElementStart` and `Token::Attribute` contains prefix
  and local part of the qualified name now.

## [0.1.2] - 2018-02-12
### Added
- `Stream::skip_ascii_spaces`.
- Small performance optimizations.

## [0.1.1] - 2018-01-17
### Changed
- `log` 0.3 -> 0.4

[Unreleased]: https://github.com/RazrFalcon/xmlparser/compare/v0.13.6...HEAD
[0.13.6]: https://github.com/RazrFalcon/xmlparser/compare/v0.13.5...v0.13.6
[0.13.5]: https://github.com/RazrFalcon/xmlparser/compare/v0.13.4...v0.13.5
[0.13.4]: https://github.com/RazrFalcon/xmlparser/compare/v0.13.3...v0.13.4
[0.13.3]: https://github.com/RazrFalcon/xmlparser/compare/v0.13.2...v0.13.3
[0.13.2]: https://github.com/RazrFalcon/xmlparser/compare/v0.13.1...v0.13.2
[0.13.1]: https://github.com/RazrFalcon/xmlparser/compare/v0.13.0...v0.13.1
[0.13.0]: https://github.com/RazrFalcon/xmlparser/compare/v0.12.0...v0.13.0
[0.12.0]: https://github.com/RazrFalcon/xmlparser/compare/v0.11.0...v0.12.0
[0.11.0]: https://github.com/RazrFalcon/xmlparser/compare/v0.10.0...v0.11.0
[0.10.0]: https://github.com/RazrFalcon/xmlparser/compare/v0.9.0...v0.10.0
[0.9.0]: https://github.com/RazrFalcon/xmlparser/compare/v0.8.1...v0.9.0
[0.8.1]: https://github.com/RazrFalcon/xmlparser/compare/v0.8.0...v0.8.1
[0.8.0]: https://github.com/RazrFalcon/xmlparser/compare/v0.7.0...v0.8.0
[0.7.0]: https://github.com/RazrFalcon/xmlparser/compare/v0.6.1...v0.7.0
[0.6.1]: https://github.com/RazrFalcon/xmlparser/compare/v0.6.0...v0.6.1
[0.6.0]: https://github.com/RazrFalcon/xmlparser/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/RazrFalcon/xmlparser/compare/v0.4.1...v0.5.0
[0.4.1]: https://github.com/RazrFalcon/xmlparser/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/RazrFalcon/xmlparser/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/RazrFalcon/xmlparser/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/RazrFalcon/xmlparser/compare/v0.1.2...v0.2.0
[0.1.2]: https://github.com/RazrFalcon/xmlparser/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/RazrFalcon/xmlparser/compare/v0.1.0...v0.1.1
