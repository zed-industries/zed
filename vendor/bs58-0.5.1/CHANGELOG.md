# Changelog

## Unreleased

## 0.5.1 - 2024-03-19

 * Make it possible to decode in `const`-context (by @joncinque)

## 0.5.0 - 2023-05-23

 * Breaking change: make encoding onto resizable buffers not clear them, instead appending onto any existing data
 * Breaking change: rename `into` methods to `onto` to allow for implementing `Into` in the future (or a similar inherent method)
 * Add new `cb58` feature to support injecting and verifying that checksum (by @Zondax)
 * Update `sha2` to 0.10 (by @madninja)
 * Tighten max-encoded length estimation to reduce overallocation of resizable buffers (by @mina86)
 * Add optional support for encoding/decoding to `smallvec::SmallVec` (by @mina86)
 * Add optional support for encoding/decoding to `tinyvec`'s various types

## 0.4.0 - 2020-11-06

 * Correct documentation on version mismatch error (by @taoweicn)
 * Update `sha2` from 0.8 to 0.9
 * Switch error enums to use new `#[non_exhaustive]` attribute
 * Use new `const fn` features to drastically simplify construction of a prepared alphabet
 * Update documentation and examples to use `?` instead of `unwrap`
 * Remove the non-prepared alphabet APIs, update alphabet construction to pre-verify requirements of a consistent alphabet

## 0.3.1 - 2020-04-20

 * Removed an unnecessary unsafe block (by @fanatid)
 * Internal code cleanup (by @fanatid)
 * Add ability to pre-prepare the alphabet for performance (by @fanatid)
 * Add function to append the version onto the data automatically for Base58Check encoding (by @fanatid)

## 0.3.0 - 2019-09-16
## 0.2.5 - 2019-08-30
## 0.2.4 - 2019-08-19
## 0.2.3 - 2019-08-19
## 0.2.2 - 2018-09-15

 * Base58Check support (thanks to @devin-fisher)

## 0.2.1 - 2018-06-12

 * Fix tests on Rust 1.27+
 * Fix potential unsoundness when encoding with a custom alphabet

## 0.2.0 - 2017-01-07

 * Major refactor to use a builder pattern instead of traits
   * Traits still kept, but deprecated and likely to disappear in next major version
 * Now supports writing output to a provided buffer for better performance/heapless code.

## 0.1.3 - 2016-11-05
## 0.1.2 - 2016-11-02
## 0.1.1 - 2016-11-02
## 0.1.0 - 2016-11-02
