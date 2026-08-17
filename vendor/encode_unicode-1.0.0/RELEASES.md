Version 1.0.0 (2022-08-07)
==========================
* Replace error types `InvalidUtf8Array`, `InvalidUtf8Slice`, `InvalidUtf8FirstByte` and `InvalidUtf8` with `Utf8Error` plus `Utf8ErrorKind`.  
  Which of the new error kind variants is reported don't map 1:1 to the old enum variants:
  For example `Utf8ErrorKind::NonUtf8Byte` is returned for sequences that would previously have been reported as too high codepoint or overlong encoding.  
* Rename many other error types for consistency:
  * `InvalidCodepoint` -> `CodepointError`
  * `InvalidUtf16FirstUnit` -> `Utf16FirstUnitError`
  * `InvalidUtf16Array` -> `Utf16ArrayError`
  * `InvalidUtf16Slice` -> `Utf16SliceError`
  * `1InvalidUtf16Tuple` -> `Utf16TupleError`
* Change return type of `CodepointError::error_range()` to `RangeInclusive`.
* Rename some errors variants:
  * `Utf16SliceError::FirstLowSurrogate` -> `FirstIsTrailingSurrogate`
  * `Utf16SliceError::SecondNotLowSurrogate` -> `SecondIsNotTrailingSurrogate`
  * `Utf16TupleError::InvalidSecond` -> `SecondIsNotTrailingSurrogate`
* Expose the error type of `Utf16Char::from_bmp()` and rename it to  `NonBmpError`.
* Remove re-exports of `Utf8CharIterator` and `Utf16CharIterator` from the crate root.  
  (They are still exposed via the `iterator` module.)
* Remove impls of the deprecated `AsciiExt` trait,
  and make the methods available in `#![no_std]`-mode.
* Make many of the previously `AsciiExt` methods take self by value.
* Drop support for pre-1.0 versions of the ascii crate.
* Remove `iter_bytes()` and `iter_units()`.
* Increase minimum Rust version to 1.56 and change the minimum Rust version policy.
* Fix possible UB or panic in `Utf8Char::from_slice_start_unchecked()` when passed an empty slice.  
  (relates to [#12](https://github.com/tormol/encode_unicode/issues/12).)
* Make many methods `const fn`.
* Add `const fn`s `Utf8Char::new()` and `Utf16Char::new()`.

Version 0.3.6 (2019-08-23)
==========================
* Fix pointless undefined behavior in `Utf16Char.to_ascii_char()` (which is part of ascii feature)
* Widen ascii version requirement to include 1.\*.
* Add `[u16; 2]` UTF-16 array alternatives to `(u16, Some(u16))` UTF-16 tuple methods.
* Add `Utf16Char.is_bmp()`.

Version 0.3.5 (2018-10-23)
==========================
* Fix docs.rs build failure

Version 0.3.4 (2018-10-23)
==========================
* Fix UB in UTF-8 validation which lead to invalid codepoints being accepted in release mode.
* Add fallible decoding iterator adapters `Utf8CharMerger` and `Utf16CharMerger`
  and slice-based iterators `Utf8CharDecoder` and `Utf16CharDecoder`
* Widen ascii version requirement from 0.8.\* to 0.8.0 - 0.10.\*
* Implement creating / extending `String`s from `Utf16Char`-producing iterators

Version 0.3.3 (2018-10-16)
==========================
* Fix UTF-8 overlong check. (`from_array()` and `from_slice()` accepted two-byte encodings of ASCII characters >= '@', which includes all letters)
* Implement `FromStr` for `Utf16Char`
* Add `from_str_start()` to `Utf8Char` and `Utf16Char`
* Add `Utf{8,16}Char{s,Indices}`: `str`-based iterators for `Utf8Char` and `Utf16Char` equivalent to `char`'s `Chars` and `CharIndices`.
* Add `StrExt` with functions to create the above iterators.
* Implement `FromIterator` and `Extend` for `Vec<{u8,u16}>` with reference-producing `Utf{8,16}Char` iterators too.
* Add `Utf8CharSplitter` and `Utf16CharSplitter`: `Utf{8,16}Char`-to-`u{8,16}` iterator adapters.
* Add `IterExt`, `iter_bytes()` and `iter_units()` to create the above splitting iterators.
* Add `Utf8Char::from_ascii()`, `Utf16Char::from_bmp()` with `_unchecked` versions of both.
* Add cross-type `PartialEq` and `PartialOrd` implementations.
* Change the `description()` for a few error types.

Version 0.3.2 (2018-08-08)
==========================
* Hide `AsciiExt` deprecation warning and add replacement methods.
* Correct documentation for `U8UtfExt::extra_utf8_bytes()`.
* Fix misspellings in some error descriptions.
* Avoid potentially bad transmutes.

Version 0.3.1 (2017-06-16)
==========================
* Implement `Display` for `Utf8Char` and `Utf16Char`.

Version 0.3.0 (2017-03-29)
==========================
* Replace the "no_std" feature with opt-out "std".
  * Upgrade ascii to v0.8.
  * Make tests compile on stable.
* Remove `CharExt::write_utf{8,16}()` because `encode_utf{8,16}()` has been stabilized.
* Return a proper error from `U16UtfExt::utf16_needs_extra_unit()` instead of `None`.
* Rename `U16UtfExt::utf_is_leading_surrogate()` to `is_utf16_leading_surrogate()`.
* Rename `Utf16Char::from_slice()` to `from_slice_start()`  and `CharExt::from_utf{8,16}_slice()`
  to `from_utf{8,16}_slice_start()` to be consistent with `Utf8Char`.
* Fix a bug where `CharExt::from_slice()` would accept some trailing surrogates
  as standalone codepoints.

Version 0.2.0 (2016-07-24)
==========================
* Change `CharExt::write_utf{8,16}()` to panic instead of returning `None`
  if the slice is too short.
* Fix bug where `CharExt::write_utf8()` and `Utf8Char::to_slice()` could change bytes it shouldn't.
* Rename lots of errors with search and replace:
  * CodePoint -> Codepoint
  * Several -> Multiple
* Update the ascii feature to use [ascii](https://tomprogrammer.github.io/rust-ascii/ascii/index.html) v0.7.
* Support `#[no_std]`; see 70e090ee for differences.
* Ungate impls of `AsciiExt`. (doesn't require ascii or nightly)
* Make the tests compile (and pass) again.
  (They still require nightly).

Version 0.1.* (2016-04-07)
==========================
First release.
