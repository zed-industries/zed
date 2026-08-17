# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.7.0 (2023-02-26)
### Changed
- MSRV 1.60 ([#802])
- Lint improvements ([#824])

[#802]: https://github.com/RustCrypto/formats/pull/802
[#824]: https://github.com/RustCrypto/formats/pull/824

## 0.6.0 (2022-04-26)
### Added
- `encapsulated_len_wrapped` ([#619])

### Changed
- `encapsulated_len` now accepts the length of the raw input bytes prior to
  Base64 encoding, and computes the length of the full PEM encoded document
  including newlines when the resulting Base64 is linewrapped ([#619])

[#619]: https://github.com/RustCrypto/formats/pull/619

## 0.5.1 (2022-03-30)
### Changed
- Rename `PemLabel::TYPE_LABEL` => `::PEM_LABEL` ([#568])

[#568]: https://github.com/RustCrypto/formats/pull/568

## 0.5.0 (2022-03-29) [YANKED]
### Added
- Clippy lints for checked arithmetic and panics ([#564])

### Changed
- Use `str::from_utf8_unchecked` in `encode` ([#565])

[#564]: https://github.com/RustCrypto/formats/pull/564
[#565]: https://github.com/RustCrypto/formats/pull/565

## 0.4.0 (2022-03-12)
### Added
- Buffered `Decoder` type ([#406])
- Buffered `Encoder` type ([#463], [#474])

### Changed
- Return `str` from `encode` ([#482])

[#406]: https://github.com/RustCrypto/formats/pull/406
[#463]: https://github.com/RustCrypto/formats/pull/463
[#474]: https://github.com/RustCrypto/formats/pull/474
[#482]: https://github.com/RustCrypto/formats/pull/482

## 0.3.1 (2021-11-17)
### Changed
- Relax `base64ct` version requirement to `^1` ([#239])

[#239]: https://github.com/RustCrypto/formats/pull/239

## 0.3.0 (2021-11-14)
### Added
- `Decoder` struct ([#177])

### Changed
- Rust 2021 edition upgrade; MSRV 1.56 ([#136])
- Bump `base64ct` dependency to v1.2 ([#175])

[#136]: https://github.com/RustCrypto/formats/pull/136
[#175]: https://github.com/RustCrypto/formats/pull/175
[#177]: https://github.com/RustCrypto/formats/pull/177

## 0.2.4 (2021-11-07)
### Changed
- Restrict `base64ct` dependency to `<1.2` to prevent MSRV breakages

## 0.2.3 (2021-10-17)
### Added
- `PemLabel` trait ([#117])

[#117]: https://github.com/RustCrypto/formats/pull/117

## 0.2.2 (2021-09-16)
### Changed
- Allow for data before PEM encapsulation boundary ([#40])

[#40]: https://github.com/RustCrypto/formats/pull/40

## 0.2.1 (2021-09-14)
### Added
- `decode_label` ([#22])
- `Error::HeaderDisallowed` ([#13], [#19], [#21])

### Changed
- Moved to `formats` repo ([#2])

[#2]: https://github.com/RustCrypto/formats/pull/2
[#13]: https://github.com/RustCrypto/formats/pull/13
[#19]: https://github.com/RustCrypto/formats/pull/19
[#21]: https://github.com/RustCrypto/formats/pull/21
[#22]: https://github.com/RustCrypto/formats/pull/22

## 0.2.0 (2021-07-26)
### Added
- Support for customizing PEM line endings

## 0.1.1 (2021-07-24)
### Changed
- Increase LF precedence in EOL stripping functions

### Fixed
- Bug in the size calculation for `decode_vec`

## 0.1.0 (2021-07-23)
- Initial release
