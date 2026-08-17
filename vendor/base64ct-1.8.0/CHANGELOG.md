# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 1.8.0 (2025-06-04)
### Changed
- Bump edition to 2024; MSRV 1.85 ([#1839])

[#1839]: https://github.com/RustCrypto/formats/pull/1839

## 1.7.3 (2025-03-13)
### Changed
- Don't fail with `InvalidLength` when reading nothing at end of data ([#1716]).

[#1716]: https://github.com/RustCrypto/formats/pull/1716

## 1.7.2 (2025-03-13)
### Changed
- Revert [#1387]: reject zero-length decode requests as it's a breaking change ([#1714])

[#1714]: https://github.com/RustCrypto/formats/pull/1714

## 1.7.1 (2025-03-10)
### Changed
- MSRV 1.81 - edition downgraded to 2021 from yanked 1.7.0 release ([#1702])

[#1702]: https://github.com/RustCrypto/formats/pull/1702

## 1.7.0 (2025-02-25) [YANKED]
### Added
- derive additional traits on alphabets ([#1578])

### Changed
- ~~MSRV 1.85 // Edition 2024 ([#1670])~~
- ~~reject zero-length decode requests ([#1387])~~
- use `core::error::Error` ([#1681])

[#1387]: https://github.com/RustCrypto/formats/pull/1387
[#1578]: https://github.com/RustCrypto/formats/pull/1578
[#1670]: https://github.com/RustCrypto/formats/pull/1670
[#1681]: https://github.com/RustCrypto/formats/pull/1681

## 1.6.0 (2023-02-26)
### Changed
- MSRV 1.60 ([#802])
- Lint improvements ([#824])

[#802]: https://github.com/RustCrypto/formats/pull/802
[#824]: https://github.com/RustCrypto/formats/pull/824

## 1.5.3 (2022-10-18)
### Added
- `Base64ShaCrypt` alphabet ([#742])

### Changed
- Use `RangeInclusive` for `DecodeStep` ([#713])

[#713]: https://github.com/RustCrypto/formats/pull/713
[#742]: https://github.com/RustCrypto/formats/pull/742

## 1.5.2 (2022-08-22)
### Fixed
- Return `Ok(0)` in `io::Read` impl to signal end of stream ([#704])

[#704]: https://github.com/RustCrypto/formats/pull/704

## 1.5.1 (2022-06-26)
### Fixed
- Last block validation ([#680])

[#680]: https://github.com/RustCrypto/formats/pull/680

## 1.5.0 (2022-03-29)
### Fixed
- Ensure checked arithmetic with `clippy::integer_arithmetic` lint ([#557])
- Prevent foreign impls of `Encoding` by bounding sealed `Variant` trait ([#562])

[#557]: https://github.com/RustCrypto/formats/pull/557
[#562]: https://github.com/RustCrypto/formats/pull/562

## 1.4.1 (2022-03-11)
### Changed
- Rename `Decoder::decoded_len` => `::remaining_len` ([#500])

[#500]: https://github.com/RustCrypto/formats/pull/500

## 1.4.0 (2022-03-10) [YANKED]
### Added
- Buffered `Encoder` type ([#366], [#455], [#457])
- `Decoder::decoded_len` method ([#403])
- Impl `std::io::Read` for `Decoder` ([#404])
- Bounds for `Encoding`/`Variant` ZSTs ([#405], [#408])

[#366]: https://github.com/RustCrypto/formats/pull/366
[#403]: https://github.com/RustCrypto/formats/pull/403
[#404]: https://github.com/RustCrypto/formats/pull/404
[#405]: https://github.com/RustCrypto/formats/pull/405
[#408]: https://github.com/RustCrypto/formats/pull/408
[#455]: https://github.com/RustCrypto/formats/pull/455
[#457]: https://github.com/RustCrypto/formats/pull/457

## 1.3.3 (2021-12-28)
### Fixed
- Potential infinite loop in `Decoder::decode` ([#305])

[#305]: https://github.com/RustCrypto/formats/pull/305

## 1.3.2 (2021-12-26) [YANKED]
### Fixed
- `Decoder` unpadding ([#299])
- Edge case when using `Decoder::new_wrapped` ([#300])

[#299]: https://github.com/RustCrypto/formats/pull/299
[#300]: https://github.com/RustCrypto/formats/pull/300

## 1.3.1 (2021-12-20) [YANKED]
### Added
- `Decoder::new_wrapped` with support for line-wrapped Base64 ([#292], [#293], [#294])

[#292]: https://github.com/RustCrypto/formats/pull/292
[#293]: https://github.com/RustCrypto/formats/pull/292
[#294]: https://github.com/RustCrypto/formats/pull/294

## 1.3.0 (2021-12-02) [YANKED]
### Added
- Stateful `Decoder` type ([#266])

[#266]: https://github.com/RustCrypto/formats/pull/266

## 1.2.0 (2021-11-03)
### Changed
- Rust 2021 edition upgrade; MSRV 1.56 ([#136])

### Fixed
- Benchmarks ([#135])

[#135]: https://github.com/RustCrypto/formats/pull/135
[#136]: https://github.com/RustCrypto/formats/pull/136

## 1.1.1 (2021-10-14)
### Changed
- Update `Util::Lookup` paper references ([#32])

[#32]: https://github.com/RustCrypto/formats/pull/32

## 1.1.0 (2021-09-14)
### Changed
- Moved to `formats` repo; MSRV 1.51+ ([#2])

[#2]: https://github.com/RustCrypto/formats/pull/2

## 1.0.1 (2021-08-14)
### Fixed
- Make `Encoding::decode` reject invalid padding

## 1.0.0 (2021-03-17)
### Changed
- Bump MSRV to 1.47+

### Fixed
- MSRV-dependent TODOs in implementation

## 0.2.1 (2021-03-07)
### Fixed
- MSRV docs

## 0.2.0 (2021-02-01)
### Changed
- Refactor with `Encoding` trait
- Internal refactoring

## 0.1.2 (2021-01-31)
### Added
- bcrypt encoding
- `crypt(3)` encoding

### Changed
- Internal refactoring

## 0.1.1 (2021-01-27)
- Minor code improvements

## 0.1.0 (2021-01-26)
- Initial release
