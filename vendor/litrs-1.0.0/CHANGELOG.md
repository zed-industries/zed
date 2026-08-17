# Changelog

All notable changes to this project will be documented in this file.


## [Unreleased]

## [1.0.0] - 2025-10-20
As announced, this is basically just v0.5. I'm really happy to finally have `litrs` in v1.x state, as the library was already fit for production usage for a long time. Now the version reflects that.

- **Breaking**: Mark `Literal` as `#[non_exhaustive]`
- Document semver guarantees

## [0.5.1] - 2025-08-01
- Add `Literal::raw_input`

## [0.5.0] - 2025-07-25
**Note**: this is supposed to be essentially a 1.0 release candidate.
If there are no problems with this release, it will be released as 1.0 after some time.

- **Breaking** Bump MSRC from 1.54 to 1.56
- **Breaking**: Remove `proc-macro2` from default features
- **Breaking**: Make `StringLit<String>::into_value` return `String` instead of `Cow<'static, str>`.
  Make `ByteStringLit<String>::into_value` return `Vec<u8>` instead of `Cow<'static, [u8]>`.
- Add support for C string literals (also breaking due to new enum variant)
- Limit number of `#` as delimiter of raw string literals to 256 (to align with spec)
- Fix license in `Cargo.toml` to use proper SPDX format (in #19, thanks @atouchet)
- Fix error span for `InvalidStartOfUnicodeEscape` errors


## [0.4.2] - 2025-07-25
- Fix incorrect byte string value with non-ASCII `\xHH` byte string escapes
    - `ByteStringLit::parse(r#"b"\xff""#).unwrap().value()` would return `[0xc3, 0xbf]` instead of `[0xff]`.
- Remove CR LF normalization to align with spec
    - This is technically a breaking change, but released with a minor version bump as it just aligns with the specification. It is extremely unlikely to affect any real world use case.
- Fix incorrect error span for out-of-range Unicode escapes

## [0.4.1] - 2023-10-18
- Fixed incorrectly labeling `27f32` a float literals in docs.
- Added hint to integer literal docs about parsing as `u128`.

## [0.4.0] - 2023-03-05
### Added
- Add ability to parse literals with arbitrary suffixes (e.g. `"foo"bla` or `23px`)
- Add `suffix()` method to all literal types except `BoolLit`
- Add `IntegerBase::value`
- Add `from_suffix` and `suffix` methods to `FloatType` and `IntegerType`
- Add `FromStr` and `Display` impls to `FloatType` and `IntegerType`

### Changed
- **Breaking**: Mark `FloatType` and `IntegerType` as `#[non_exhaustive]`
- **Breaking**: Fix integer parsing for cases like `27f32`. `Literal::parse`
  and `IntegerLit::parse` will both identify this as an integer literal.
- **Breaking**: Fix float parsing by correctly rejecting inputs like `27f32`. A
  float literal must have a period OR an exponent part, according to the spec.
  Previously decimal integers were accepted in `FloatLit::parse`.
- Improved some parts of the docs

### Removed
- **Breaking**: Remove `OwnedLiteral` and `SharedLiteral`

## [0.3.0] - 2022-12-19
### Breaking
- Bump MSRV (minimal supported Rust version) to 1.54

### Added
- Add `raw_input` and `into_raw_input` to non-bool `*Lit` types
- Add `impl From<*Lit> for pm::Literal` (for non-bool literals)
- Add `impl From<BoolLit> for pm::Ident`

### Fixed
- Fix link to reference and clarify bool literals ([#7](https://github.com/LukasKalbertodt/litrs/pull/7))

### Internals
- Move lots of parsing code into non-generic functions (this hopefully reduces compile times)
- To implement `[into_]raw_input` for integer and float literals, their
  internals were changed a bit so that they store the full input string now.

## [0.2.3] - 2021-06-09
### Changed
- Minor internal code change to bring MSRV from 1.52 to 1.42

## [0.2.2] - 2021-06-09
### Changed
- Fixed (byte) string literal parsing by:
    - Correctly handling "string continue" sequences
    - Correctly converting `\n\r` into `\n`

## [0.2.1] - 2021-06-04
### Changed
- Fixed the `expected` value of the error returned from `TryFrom<TokenTree>` impls in some cases

## [0.2.0] - 2021-05-28
### Changed
- **Breaking**: rename `Error` to `ParseError`. That describes its purpose more
    closely and is particular useful now that other error types exist in the library.

### Removed
- **Breaking**: remove `proc-macro` feature and instead offer the corresponding
    `impl`s unconditionally. Since the feature didn't enable/disable a
    dependency (`proc-macro` is a compiler provided crate) and since apparently
    it works fine in `no_std` environments, I dropped this feature. I don't
    currently see a reason why the corresponding impls should be conditional.

### Added
- `TryFrom<TokenTree> for litrs::Literal` impls
- `From<*Lit> for litrs::Literal` impls
- `TryFrom<proc_macro[2]::Literal> for *Lit`
- `TryFrom<TokenTree> for *Lit`
- `InvalidToken` error type for all new `TryFrom` impls


## [0.1.1] - 2021-05-25
### Added
- `From` impls to create a `Literal` from references to proc-macro literal types:
    - `From<&proc_macro::Literal>`
    - `From<&proc_macro2::Literal>`
- Better examples in README and repository

## 0.1.0 - 2021-05-24
### Added
- Everything


[Unreleased]: https://github.com/LukasKalbertodt/litrs/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/LukasKalbertodt/litrs/compare/v0.5.1...v1.0.0
[0.5.1]: https://github.com/LukasKalbertodt/litrs/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/LukasKalbertodt/litrs/compare/v0.4.2...v0.5.0
[0.4.2]: https://github.com/LukasKalbertodt/litrs/compare/v0.4.1...v0.4.2
[0.4.1]: https://github.com/LukasKalbertodt/litrs/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/LukasKalbertodt/litrs/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/LukasKalbertodt/litrs/compare/v0.2.3...v0.3.0
[0.2.3]: https://github.com/LukasKalbertodt/litrs/compare/v0.2.2...v0.2.3
[0.2.2]: https://github.com/LukasKalbertodt/litrs/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/LukasKalbertodt/litrs/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/LukasKalbertodt/litrs/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/LukasKalbertodt/litrs/compare/v0.1.0...v0.1.1
