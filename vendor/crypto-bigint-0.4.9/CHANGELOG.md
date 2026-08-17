# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.4.9 (2022-10-11)
### Added
- `UInt::from_word` and `::from_wide_word` ([#105])
- `UInt` modulo operations for special moduli ([#108])
- Non-const `UInt` decoding from an array ([#110])
- `const fn` impls of `concat` and `split` ([#111])
- `Limb` left/right bitshifts ([#112])
- `UInt::LIMBS` constant ([#114])

### Changed
- Optimize `UInt::neg_mod` by simply calling `::sub_mod` ([#106])
- Relax bounds for `UInt::add_mod` and `::sub_mod` ([#104])
- Always inline `Limb::bitand` ([#109])
- Faster const decoding of UInt ([#113])
- Optimize `UInt::neg_mod` ([#127])
- Faster comparisons ([#128])
- `UInt::resize` ([#129])
- `UInt::bit` accessor methods ([#122])

### Fixed
- Constant-time behaviour for `ct_reduce`/`ct_div_rem` ([#117])

[#104]: https://github.com/RustCrypto/crypto-bigint/pull/104
[#105]: https://github.com/RustCrypto/crypto-bigint/pull/105
[#106]: https://github.com/RustCrypto/crypto-bigint/pull/106
[#108]: https://github.com/RustCrypto/crypto-bigint/pull/108
[#109]: https://github.com/RustCrypto/crypto-bigint/pull/109
[#110]: https://github.com/RustCrypto/crypto-bigint/pull/110
[#111]: https://github.com/RustCrypto/crypto-bigint/pull/111
[#112]: https://github.com/RustCrypto/crypto-bigint/pull/112
[#113]: https://github.com/RustCrypto/crypto-bigint/pull/113
[#114]: https://github.com/RustCrypto/crypto-bigint/pull/114
[#117]: https://github.com/RustCrypto/crypto-bigint/pull/117
[#122]: https://github.com/RustCrypto/crypto-bigint/pull/122
[#127]: https://github.com/RustCrypto/crypto-bigint/pull/127
[#128]: https://github.com/RustCrypto/crypto-bigint/pull/128
[#129]: https://github.com/RustCrypto/crypto-bigint/pull/129

## 0.4.8 (2022-06-30)
### Added
- `Word` as a replacement for `LimbUInt` ([#88])
- `WideWord` as a replacement for `WideLimbUInt` ([#88])
- `UInt::*_words` as a replacement for `UInt::*_uint_array` ([#88])

### Changed
- Deprecated `*LimbUInt` and `UInt::*_uint_array` ([#88])

[#88]: https://github.com/RustCrypto/crypto-bigint/pull/88

## 0.4.7 (2022-06-12)
### Added
- `Encoding` tests ([#93])

### Changed
- Use const generic impls of `*Mod` traits ([#98])

[#93]: https://github.com/RustCrypto/crypto-bigint/pull/93
[#98]: https://github.com/RustCrypto/crypto-bigint/pull/98

## 0.4.6 (2022-06-12)
### Added
- Impl `ArrayEncoding` for `U576` ([#96])

[#96]: https://github.com/RustCrypto/crypto-bigint/pull/96

## 0.4.5 (2022-06-12)
### Added
- `serde` support ([#73])
- `U576` type alias ([#94])

[#73]: https://github.com/RustCrypto/crypto-bigint/pull/73
[#94]: https://github.com/RustCrypto/crypto-bigint/pull/94

## 0.4.4 (2022-06-02)
### Added
- `UInt::as_uint_array` ([#91])

[#91]: https://github.com/RustCrypto/crypto-bigint/pull/91

## 0.4.3 (2022-05-31)
### Added
- Impl `AsRef`/`AsMut<[LimbUInt]>` for `UInt` ([#89])

[#89]: https://github.com/RustCrypto/crypto-bigint/pull/89

## 0.4.2 (2022-05-18)
### Added
- `UInt::inv_mod2k` ([#86])

### Fixed
- Wrong results for remainder ([#84])

[#84]: https://github.com/RustCrypto/crypto-bigint/pull/84
[#86]: https://github.com/RustCrypto/crypto-bigint/pull/86

## 0.4.1 (2022-05-10)
### Fixed
- Bug in `from_le_slice` ([#82])

[#82]: https://github.com/RustCrypto/crypto-bigint/pull/82

## 0.4.0 (2022-05-08) [YANKED]

NOTE: this release was yanked due to [#82].

### Added
- Const-friendly `NonZero` from `UInt` ([#56])
- Optional `der` feature ([#61], [#80])

### Changed
- Use `const_panic`; MSRV 1.57 ([#60])
- 2021 edition ([#60])

### Fixed
- Pad limbs with zeros when displaying hexadecimal representation ([#74])

[#56]: https://github.com/RustCrypto/crypto-bigint/pull/56
[#60]: https://github.com/RustCrypto/crypto-bigint/pull/60
[#61]: https://github.com/RustCrypto/crypto-bigint/pull/61
[#74]: https://github.com/RustCrypto/crypto-bigint/pull/74
[#80]: https://github.com/RustCrypto/crypto-bigint/pull/80

## 0.3.2 (2021-11-17)
### Added
- `Output = Self` to all bitwise ops on `Integer` trait ([#53])

[#53]: https://github.com/RustCrypto/crypto-bigint/pull/53

## 0.3.1 (2021-11-17)
### Added
- Bitwise ops to `Integer` trait ([#51])

[#51]: https://github.com/RustCrypto/crypto-bigint/pull/51

## 0.3.0 (2021-11-14) [YANKED]
### Added
- Bitwise `Xor`/`Not` operations ([#27])
- `Zero` trait ([#35])
- `Checked*` traits ([#41])
- `prelude` module ([#45])
- `saturating_*` ops ([#47])

### Changed
- Rust 2021 edition upgrade; MSRV 1.56 ([#33])
- Reverse ordering of `UInt::mul_wide` return tuple ([#34])
- Have `Div` and `Rem` impls always take `NonZero` args ([#39])
- Rename `limb::Inner` to `LimbUInt` ([#40])
- Make `limb` module private ([#40])
- Use `Zero`/`Integer` traits for `is_zero`, `is_odd`, and `is_even` ([#46])

### Fixed
- `random_mod` performance for small moduli ([#36])
- `NonZero` moduli ([#36])

### Removed
- Deprecated `LIMB_BYTES` constant ([#43])

[#27]: https://github.com/RustCrypto/crypto-bigint/pull/27
[#33]: https://github.com/RustCrypto/crypto-bigint/pull/33
[#34]: https://github.com/RustCrypto/crypto-bigint/pull/34
[#35]: https://github.com/RustCrypto/crypto-bigint/pull/35
[#36]: https://github.com/RustCrypto/crypto-bigint/pull/36
[#39]: https://github.com/RustCrypto/crypto-bigint/pull/39
[#40]: https://github.com/RustCrypto/crypto-bigint/pull/40
[#41]: https://github.com/RustCrypto/crypto-bigint/pull/41
[#43]: https://github.com/RustCrypto/crypto-bigint/pull/43
[#45]: https://github.com/RustCrypto/crypto-bigint/pull/45
[#46]: https://github.com/RustCrypto/crypto-bigint/pull/46
[#47]: https://github.com/RustCrypto/crypto-bigint/pull/47

## 0.2.11 (2021-10-16)
### Added
- `AddMod` proptests ([#24])
- Bitwise `And`/`Or` operations ([#25])

[#24]: https://github.com/RustCrypto/crypto-bigint/pull/24
[#25]: https://github.com/RustCrypto/crypto-bigint/pull/25

## 0.2.10 (2021-09-21)
### Added
- `ArrayDecoding` trait ([#12])
- `NonZero` wrapper ([#13], [#16])
- Impl `Div`/`Rem` for `NonZero<UInt>` ([#14])

[#12]: https://github.com/RustCrypto/crypto-bigint/pull/12
[#13]: https://github.com/RustCrypto/crypto-bigint/pull/13
[#14]: https://github.com/RustCrypto/crypto-bigint/pull/14
[#16]: https://github.com/RustCrypto/crypto-bigint/pull/16

## 0.2.9 (2021-09-16)
### Added
- `UInt::sqrt` ([#9])

### Changed
- Make `UInt` division similar to other interfaces ([#8])

[#8]: https://github.com/RustCrypto/crypto-bigint/pull/8
[#9]: https://github.com/RustCrypto/crypto-bigint/pull/9

## 0.2.8 (2021-09-14) [YANKED]
### Added
- Implement constant-time division and modulo operations

### Changed
- Moved from RustCrypto/utils to RustCrypto/crypto-bigint repo ([#2])

[#2]: https://github.com/RustCrypto/crypto-bigint/pull/2

## 0.2.7 (2021-09-12)
### Added
- `UInt::shl_vartime` 

### Fixed
- `add_mod` overflow handling

## 0.2.6 (2021-09-08)
### Added
- `Integer` trait
- `ShrAssign` impl for `UInt`
- Recursive Length Prefix (RLP) encoding support for `UInt`

## 0.2.5 (2021-09-02)
### Fixed
- `ConditionallySelectable` impl for `UInt`

## 0.2.4 (2021-08-23) [YANKED]
### Added
- Expose `limb` module
- `[limb::Inner; LIMBS]` conversions for `UInt`
- Bitwise right shift support for `UInt` ([#586], [#590])

## 0.2.3 (2021-08-16) [YANKED]
### Fixed
- `UInt::wrapping_mul`

### Added
- Implement the `Hash` trait for `UInt` and `Limb`

## 0.2.2 (2021-06-26) [YANKED]
### Added
- `Limb::is_odd` and `UInt::is_odd`
- `UInt::new`
- `rand` feature

### Changed
- Deprecate `LIMB_BYTES` constant
- Make `Limb`'s `Inner` value public

## 0.2.1 (2021-06-21) [YANKED]
### Added
- `Limb` newtype
- Target-specific rustdocs

## 0.2.0 (2021-06-07) [YANKED]
### Added
- `ConstantTimeGreater`/`ConstantTimeLess` impls for UInt
- `From` conversions between `UInt` and limb arrays
- `zeroize` feature
- Additional `ArrayEncoding::ByteSize` bounds
- `UInt::into_limbs`
- `Encoding` trait

### Removed
- `NumBits`/`NumBytes` traits; use `Encoding` instead

## 0.1.0 (2021-05-30)
- Initial release
