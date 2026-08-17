# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.10.6 (2023-09-22)
### Added
- `asm!`-based backend for LoongArch64 targets gated behind `loongarch64_asm` feature [#505]

[#505]: https://github.com/RustCrypto/hashes/pull/505

## 0.10.5 (2022-09-22)
### Added
- Feature-gated OID support ([#413])

[#413]: https://github.com/RustCrypto/hashes/pull/413

## 0.10.4 (2022-09-02)
### Fixed
- MSRV issue which was not resolved by v0.10.3 ([#401])

[#401]: https://github.com/RustCrypto/hashes/pull/401

## 0.10.3 (2022-09-02)
### Fixed
- MSRV issue caused by publishing v0.10.2 using a buggy Nightly toolchain ([#399])

[#399]: https://github.com/RustCrypto/hashes/pull/399

## 0.10.2 (2022-08-30)
### Changed
- Ignore `asm` feature on unsupported targets ([#388])

[#388]: https://github.com/RustCrypto/hashes/pull/388

## 0.10.1 (2022-02-17)
### Fixed
- Minimal versions build ([#363])

[#363]: https://github.com/RustCrypto/hashes/pull/363

## 0.10.0 (2021-12-07)
### Changed
- Update to `digest` v0.10 ([#217])

[#217]: https://github.com/RustCrypto/hashes/pull/217

## 0.9.1 (2020-06-28)
### Changed
- Update to `block-buffer` v0.9 ([#164])
- Update to `opaque-debug` v0.3 ([#168])

[#164]: https://github.com/RustCrypto/hashes/pull/164
[#168]: https://github.com/RustCrypto/hashes/pull/168

## 0.9.0 (2020-06-09)
### Changed
- Update to `digest` v0.9 release; MSRV 1.41+ ([#155])
- Use new `*Dirty` traits from the `digest` crate ([#153])
- Bump `block-buffer` to v0.8 release ([#151])
- Rename `*result*` to `finalize` ([#148])
- Update to Rust 2018 edition ([#128])

[#155]: https://github.com/RustCrypto/hashes/pull/155
[#153]: https://github.com/RustCrypto/hashes/pull/153
[#151]: https://github.com/RustCrypto/hashes/pull/151
[#148]: https://github.com/RustCrypto/hashes/pull/148
[#128]: https://github.com/RustCrypto/hashes/pull/128

## 0.8.0 (2018-10-02)

## 0.7.0 (2017-11-15)

## 0.5.2 (2017-06-13)

## 0.5.1 (2017-06-13)

## 0.5.0 (2017-06-12)

## 0.4.4 (2017-06-02)

## 0.4.3 (2017-05-09)

## 0.4.2 (2017-05-02)

## 0.4.1 (2017-04-18)

## 0.4.0 (2017-04-06)
