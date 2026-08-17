# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.11.0 (2022-03-28)
### Changed
- Bump `password-hash` dependency to v0.4; MSRV 1.57 ([#283])
- 2021 edition upgrade ([#284])

[#283]: https://github.com/RustCrypto/password-hashes/pull/283
[#284]: https://github.com/RustCrypto/password-hashes/pull/284

## 0.10.1 (2022-02-17)
### Fixed
- Minimal versions build ([#273])

[#273]: https://github.com/RustCrypto/password-hashes/pull/273

## 0.10.0 (2021-11-25)
### Changed
- Migrate from `crypto-mac` to `digest` v0.10 ([#254])

[#254]: https://github.com/RustCrypto/password-hashes/pull/254

## 0.9.0 (2021-08-27)
### Added
- GOST test vectors ([#191])

### Changed
- Bump `password-hash` to v0.3 ([#217], [RustCrypto/traits#724])
- Use `resolver = "2"`; MSRV 1.51+ ([#220])

### Removed
- `McfHasher` impl on `Pbkdf2` ([#219])

[#191]: https://github.com/RustCrypto/password-hashing/pull/191
[#217]: https://github.com/RustCrypto/password-hashing/pull/217
[#219]: https://github.com/RustCrypto/password-hashing/pull/219
[#220]: https://github.com/RustCrypto/password-hashing/pull/220
[RustCrypto/traits#724]: https://github.com/RustCrypto/traits/pull/724

## 0.8.0 (2021-04-29)
### Changed
- Bump `password-hash` crate dependency to v0.2 ([#164])
- Bump `hmac` and `crypto-mac` crate deps to v0.11 ([#165])

[#164]: https://github.com/RustCrypto/password-hashing/pull/164
[#165]: https://github.com/RustCrypto/password-hashing/pull/165

## 0.7.5 (2021-03-27)
### Fixed
- Pin `password-hash` to v0.1.2 or newer ([#151])

[#151]: https://github.com/RustCrypto/password-hashing/pull/151

## 0.7.4 (2021-03-17)
### Changed
- Bump `base64ct` dependency to v1.0 ([#144])

[#144]: https://github.com/RustCrypto/password-hashing/pull/144

## 0.7.3 (2021-02-08)
### Changed
- Enable `rand_core` feature of `password-hash` ([#130])

[#130]: https://github.com/RustCrypto/password-hashing/pull/130

## 0.7.2 (2021-02-01)
### Changed
- Bump `base64ct` dependency to v0.2 ([#119])

[#119]: https://github.com/RustCrypto/password-hashing/pull/119

## 0.7.1 (2021-01-29)
### Removed
- `alloc` dependencies for `simple` feature ([#107])

[#107]: https://github.com/RustCrypto/password-hashing/pull/107

## 0.7.0 (2021-01-29)
### Added
- PHC hash format support using `password-hash` crate ([#82])

### Changed
- Rename `include_simple` features to `simple` ([#99])

### Removed
- Legacy `simple` API ([#98])

[#82]: https://github.com/RustCrypto/password-hashing/pull/82
[#98]: https://github.com/RustCrypto/password-hashing/pull/98
[#99]: https://github.com/RustCrypto/password-hashing/pull/99

## 0.6.0 (2020-10-18)
### Changed
- Bump `crypto-mac` dependency to v0.10 ([#58])
- Bump `hmac` dependency to v0.10 ([#58])

[#58]: https://github.com/RustCrypto/password-hashing/pull/58

## 0.5.0 (2020-08-18)
### Changed
- Bump `crypto-mac` dependency to v0.9 ([#44])

[#44]: https://github.com/RustCrypto/password-hashing/pull/44

## 0.4.0 (2020-06-10)
### Changed
- Code improvements ([#33])
- Bump `rand` dependency to v0.7 ([#31])
- Bump `hmac` to v0.8 ([#30])
- Bump `sha2` to v0.9 ([#30])
- Bump `subtle` to v2 ([#13])
- MSRV 1.41+ ([#30])
- Upgrade to Rust 2018 edition ([#24])

[#33]: https://github.com/RustCrypto/password-hashing/pull/33
[#31]: https://github.com/RustCrypto/password-hashing/pull/31
[#30]: https://github.com/RustCrypto/password-hashing/pull/30
[#24]: https://github.com/RustCrypto/password-hashing/pull/24
[#13]: https://github.com/RustCrypto/password-hashing/pull/13

## 0.3.0 (2018-10-08)

## 0.2.3 (2018-08-30)

## 0.2.2 (2018-08-15)

## 0.2.1 (2018-08-06)

## 0.2.0 (2018-03-30)

## 0.1.0 (2017-08-16)
