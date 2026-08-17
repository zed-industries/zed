# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.4.2 (2022-06-27)
### Fixed
- docs.rs metadata ([#1031])

[#1031]: https://github.com/RustCrypto/traits/pull/1031

## 0.4.1 (2022-04-22)
### Added
- `authentication` category to Cargo.toml ([#976])

[#976]: https://github.com/RustCrypto/traits/pull/976

## 0.4.0 (2022-03-09)
### Changed
- Leverage `const_panic`; MSRV 1.57 ([#896])
- Rust 2021 edition upgrade ([#897])
- Make `Ident::new` fallible; add `Ident::new_unwrap` ([#896], [#960])

### Fixed
- Better `Debug`/`Display` impls for `SaltString` ([#804])

### Removed
- `base64ct` version restrictions ([#914])

[#804]: https://github.com/RustCrypto/traits/pull/804
[#896]: https://github.com/RustCrypto/traits/pull/896
[#897]: https://github.com/RustCrypto/traits/pull/897
[#897]: https://github.com/RustCrypto/traits/pull/897
[#914]: https://github.com/RustCrypto/traits/pull/914
[#960]: https://github.com/RustCrypto/traits/pull/960

## 0.3.2 (2021-09-15)
### Fixed
- Remove unused lifetimes ([#760])

[#760]: https://github.com/RustCrypto/traits/pull/760

## 0.3.1 (2021-09-14) [YANKED]
### Added
- `PasswordHashString` ([#758])

### Fixed
- Handling of empty salts in `fmt::Display` impl for PasswordHash ([#748])
- MSRV regression from `base64ct` ([#757])

[#748]: https://github.com/RustCrypto/traits/pull/748
[#757]: https://github.com/RustCrypto/traits/pull/757
[#758]: https://github.com/RustCrypto/traits/pull/758

## 0.3.0 (2021-08-27) [YANKED]
### Added
- More details to `ParamValueInvalid` ([#713])
- `SaltInvalid` error ([#713])
- `version` param to `PasswordHasher` ([#719])
- `ParamsString::add_b64_bytes` method ([#722])

### Changed
- Rename `PasswordHash::hash_password_simple` => `PasswordHash::hash_password` ([#720])
- Rename `PasswordHash::hash_password` => `PasswordHash::hash_password_customized` ([#720])
- Rename `Error::B64` => `Error::B64Encoding` ([#721])

[#713]: https://github.com/RustCrypto/traits/pull/713
[#719]: https://github.com/RustCrypto/traits/pull/719
[#720]: https://github.com/RustCrypto/traits/pull/720
[#721]: https://github.com/RustCrypto/traits/pull/721
[#722]: https://github.com/RustCrypto/traits/pull/722

## 0.2.3 (2021-08-23)
### Changed
- Make max lengths of `Value` and `Salt` both 64 ([#707])

[#707]: https://github.com/RustCrypto/traits/pull/707

## 0.2.2 (2021-07-20)
### Changed
- Pin `subtle` dependency to v2.4 ([#689])

### Added
- Re-export `rand_core` ([#683])

[#683]: https://github.com/RustCrypto/traits/pull/683
[#689]: https://github.com/RustCrypto/traits/pull/689

## 0.2.1 (2021-05-05)
### Changed
- Use `subtle` crate for comparing hash `Output` ([#631])

[#631]: https://github.com/RustCrypto/traits/pull/631

## 0.2.0 (2021-04-29)
### Changed
- Allow specifying output length and version with params ([#615])
- Allow passing `&str`, `&Salt`, or `&SaltString` as salt ([#615])
- Simplify error handling ([#615])

[#615]: https://github.com/RustCrypto/traits/pull/615

## 0.1.4 (2021-04-19)
### Added
- Length constants ([#600])

### Changed
- Deprecate functions for obtaining length constants ([#600])

[#600]: https://github.com/RustCrypto/traits/pull/600

## 0.1.3 (2021-04-17)
### Changed
- Update docs for PHC string <version> field ([#593])

### Fixed
- Broken `b64` links in rustdoc ([#594])

[#593]: https://github.com/RustCrypto/traits/pull/593
[#594]: https://github.com/RustCrypto/traits/pull/594

## 0.1.2 (2021-03-17)
### Changed
- Bump `base64ct` dependency to v1.0 ([#579])

[#579]: https://github.com/RustCrypto/traits/pull/579

## 0.1.1 (2021-02-01)
### Added
- `Encoding` enum with bcrypt and `crypt(3)` Base64 support ([#515])
- Support for using `PasswordHash` with an alternate `Encoding` ([#518])

### Changed
- Bump `base64ct` dependency to v0.2 ([#519])

[#515]: https://github.com/RustCrypto/traits/pull/515
[#518]: https://github.com/RustCrypto/traits/pull/518
[#519]: https://github.com/RustCrypto/traits/pull/519

## 0.1.0 (2021-01-28)
- Initial release
