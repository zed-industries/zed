# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.5.5 (2023-11-18)
### Added
- Multi-exponentiation ([#248])
- `const_assert_eq!` and `const_assert_ne!` macros ([#293])

[#248]: https://github.com/RustCrypto/crypto-bigint/pull/248
[#293]: https://github.com/RustCrypto/crypto-bigint/pull/293

## 0.5.4 (2023-11-12)
### Added
- `trailing_ones[_vartime]()`, `trailing_zeros_vartime()`, `leading_zeros_vartime()` ([#282])
- Implement `ArrayEncoding` for `U832` ([#288])

### Changed
- Make `Uint::random_mod()` work identically on 32- and 64-bit targets ([#285])

[#282]: https://github.com/RustCrypto/crypto-bigint/pull/282
[#285]: https://github.com/RustCrypto/crypto-bigint/pull/285
[#288]: https://github.com/RustCrypto/crypto-bigint/pull/288

## 0.5.3 (2023-09-04)
### Added
- `BoxedUint`: heap-allocated fixed-precision integers ([#221])
- `extra-sizes` feature ([#229])
- `U4224` and `U4352` ([#233])
- Zeroizing support for `DynResidue` ([#235])
- `cmp_vartime`, `ct_cmp` ([#238])
- Expose Montgomery form in `Residue`/`DynResidue` ([#239])
- Make `Uint::pow` work with different sized exponents ([#251])
- Expose `wrapping_neg` ([#252])
- Make `concat`, `split`, and multiply work with different sized operands ([#253])
- `U16384` and `U32768` ([#255])
- `Uint::{inv_mod, inv_mod2k_vartime}` ([#263])
- `const fn` constructors for `NonZero<Uint>` and `NonZero<Limb>` ([#266])
- Constant-time `Uint::shr()` and `Uint::shl()` ([#267])
- Subtle trait impls for `DynResidue` and `DynResidueParams` ([#269])

### Changed
- Modular inversion improvements ([#263])

### Fixed
- `serdect` usage ([#222])
- Enforce valid modulus for `DynResidueParams` ([#240])
- Enforce valid modulus for `Residue` and associated macros ([#243])
- Make `Uint::{from_be_hex, from_le_hex}` constant-time ([#254])
- Remove conditionals in `Uint::saturating_add()` and `saturating_mul()` ([#256])
- More logical checks in the `Uint::random_mod()` test ([#256])
- Mark `sqrt` for renaming, to explicitly describe it as vartime ([#256])

[#221]: https://github.com/RustCrypto/crypto-bigint/pull/221
[#222]: https://github.com/RustCrypto/crypto-bigint/pull/222
[#229]: https://github.com/RustCrypto/crypto-bigint/pull/229
[#233]: https://github.com/RustCrypto/crypto-bigint/pull/233
[#235]: https://github.com/RustCrypto/crypto-bigint/pull/235
[#238]: https://github.com/RustCrypto/crypto-bigint/pull/238
[#239]: https://github.com/RustCrypto/crypto-bigint/pull/239
[#240]: https://github.com/RustCrypto/crypto-bigint/pull/240
[#243]: https://github.com/RustCrypto/crypto-bigint/pull/243
[#251]: https://github.com/RustCrypto/crypto-bigint/pull/251
[#252]: https://github.com/RustCrypto/crypto-bigint/pull/252
[#253]: https://github.com/RustCrypto/crypto-bigint/pull/253
[#254]: https://github.com/RustCrypto/crypto-bigint/pull/254
[#255]: https://github.com/RustCrypto/crypto-bigint/pull/255
[#256]: https://github.com/RustCrypto/crypto-bigint/pull/256
[#263]: https://github.com/RustCrypto/crypto-bigint/pull/263
[#266]: https://github.com/RustCrypto/crypto-bigint/pull/266
[#267]: https://github.com/RustCrypto/crypto-bigint/pull/267
[#269]: https://github.com/RustCrypto/crypto-bigint/pull/269

## 0.5.2 (2023-04-26)
### Added
- Expose residue params and modulus in `DynResidue` ([#197])
- Impl `DefaultIsZeroes` for `Residue` ([#210])
- `div_by_2()` method for integers in Montgomery form ([#211], [#212])

### Changed
- Montgomery multiplication improvements ([#203])

[#197]: https://github.com/RustCrypto/crypto-bigint/pull/197
[#203]: https://github.com/RustCrypto/crypto-bigint/pull/203
[#210]: https://github.com/RustCrypto/crypto-bigint/pull/210
[#211]: https://github.com/RustCrypto/crypto-bigint/pull/211
[#212]: https://github.com/RustCrypto/crypto-bigint/pull/212

## 0.5.1 (2023-03-13)
### Changed
- Improve `Debug` impls on `Limb` and `Uint` ([#195])

### Fixed
- `const_residue` macro accessibility bug ([#193])

[#193]: https://github.com/RustCrypto/crypto-bigint/pull/193
[#195]: https://github.com/RustCrypto/crypto-bigint/pull/195

## 0.5.0 (2023-02-27)
### Added
- `Residue`: modular arithmetic with static compile-time moduli ([#130])
- `DynResidue`: modular arithmetic with dynamic runtime moduli ([#134])
- Constant-time division by a single `Limb` ([#141])
- Windowed exponentiation for `(Dyn)Residue` ([#147])
- `SubResidue` trait and impls for `Residue` and `DynResidue` ([#149])
- `Pow`, `Invert` and `Square` ([#155])
- `CtChoice` type ([#159])
- `BITS`, `BYTES`, and `LIMBS` to `Integer` trait ([#161])
- Impl `Random` for `Wrapping` ([#168])
- Support to concat `U320` and `U640` ([#173])
- Define `U224` and `U544` on 32-bit platforms ([#179], [#180])

### Changed
- Rename `UInt` -> `Uint` ([#143])
- Rename `Uint` methods ([#144])
  - `limbs` -> `as_limbs`
  - `limbs_mut` -> `as_limbs_mut`
  - `into_limbs` -> `to_limbs`
- Faster `random_mod` ([#146])
- Constant-time `leading_zeros()`, `trailing_zeros()`, `bits()`, and `bit()` for `Uint` ([#153])
- Rename `BIT_SIZE` -> `BITS`, `BYTE_SIZE` -> `BYTES` ([#157])
- More efficient squaring operation ([#133])
- Use `CryptoRngCore` ([#164])
- Bump `serdect` to 0.2 ([#185])
- Bump `der` dependency to v0.7; MSRV 1.65 ([#187])

### Fixed
- Integer overflow in `div2by1()` ([#156])
- Convert from tuple element ordering ([#183])

[#130]: https://github.com/RustCrypto/crypto-bigint/pull/130
[#134]: https://github.com/RustCrypto/crypto-bigint/pull/134
[#141]: https://github.com/RustCrypto/crypto-bigint/pull/141
[#143]: https://github.com/RustCrypto/crypto-bigint/pull/143
[#144]: https://github.com/RustCrypto/crypto-bigint/pull/144
[#146]: https://github.com/RustCrypto/crypto-bigint/pull/146
[#147]: https://github.com/RustCrypto/crypto-bigint/pull/147
[#149]: https://github.com/RustCrypto/crypto-bigint/pull/149
[#153]: https://github.com/RustCrypto/crypto-bigint/pull/153
[#155]: https://github.com/RustCrypto/crypto-bigint/pull/155
[#156]: https://github.com/RustCrypto/crypto-bigint/pull/156
[#157]: https://github.com/RustCrypto/crypto-bigint/pull/157
[#159]: https://github.com/RustCrypto/crypto-bigint/pull/159
[#161]: https://github.com/RustCrypto/crypto-bigint/pull/161
[#164]: https://github.com/RustCrypto/crypto-bigint/pull/164
[#168]: https://github.com/RustCrypto/crypto-bigint/pull/168
[#173]: https://github.com/RustCrypto/crypto-bigint/pull/173
[#179]: https://github.com/RustCrypto/crypto-bigint/pull/179
[#180]: https://github.com/RustCrypto/crypto-bigint/pull/180
[#183]: https://github.com/RustCrypto/crypto-bigint/pull/183
[#185]: https://github.com/RustCrypto/crypto-bigint/pull/185
[#187]: https://github.com/RustCrypto/crypto-bigint/pull/187

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
