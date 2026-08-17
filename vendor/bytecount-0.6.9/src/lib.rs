//! count occurrences of a given byte, or the number of UTF-8 code points, in a
//! byte slice, fast.
//!
//! This crate has the [`count`](fn.count.html) method to count byte
//! occurrences (for example newlines) in a larger `&[u8]` slice.
//!
//! For example:
//!
//! ```rust
//! assert_eq!(5, bytecount::count(b"Hello, this is the bytecount crate!", b' '));
//! ```
//!
//! Also there is a [`num_chars`](fn.num_chars.html) method to count
//! the number of UTF8 characters in a slice. It will work the same as
//! `str::chars().count()` for byte slices of correct UTF-8 character
//! sequences. The result will likely be off for invalid sequences,
//! although the result is guaranteed to be between `0` and
//! `[_]::len()`, inclusive.
//!
//! Example:
//!
//! ```rust
//! let sequence = "Wenn ich ein Vöglein wär, flög ich zu Dir!";
//! assert_eq!(sequence.chars().count(),
//!            bytecount::num_chars(sequence.as_bytes()));
//! ```
//!
//! For completeness and easy comparison, the "naive" versions of both
//! count and num_chars are provided. Those are also faster if used on
//! predominantly small strings. The
//! [`naive_count_32`](fn.naive_count_32.html) method can be faster
//! still on small strings.

#![cfg_attr(feature = "generic-simd", feature(portable_simd))]
#![deny(missing_docs)]
#![cfg_attr(not(feature = "runtime-dispatch-simd"), no_std)]

#[cfg(not(feature = "runtime-dispatch-simd"))]
use core::mem;
#[cfg(feature = "runtime-dispatch-simd")]
use std::mem;

mod naive;
pub use naive::*;
mod integer_simd;

#[cfg(any(
    all(
        feature = "runtime-dispatch-simd",
        any(target_arch = "x86", target_arch = "x86_64")
    ),
    all(target_arch = "aarch64", target_endian = "little"),
    target_arch = "wasm32",
    feature = "generic-simd"
))]
mod simd;

/// Count occurrences of a byte in a slice of bytes, fast
///
/// # Examples
///
/// ```
/// let s = b"This is a Text with spaces";
/// let number_of_spaces = bytecount::count(s, b' ');
/// assert_eq!(number_of_spaces, 5);
/// ```
pub fn count(haystack: &[u8], needle: u8) -> usize {
    if haystack.len() >= 32 {
        #[cfg(all(feature = "runtime-dispatch-simd", target_arch = "x86_64"))]
        {
            if is_x86_feature_detected!("avx2") {
                unsafe {
                    return simd::x86_avx2::chunk_count(haystack, needle);
                }
            }
        }

        #[cfg(feature = "generic-simd")]
        return simd::generic::chunk_count(haystack, needle);
    }

    if haystack.len() >= 16 {
        #[cfg(all(
            feature = "runtime-dispatch-simd",
            any(target_arch = "x86", target_arch = "x86_64"),
            not(feature = "generic-simd")
        ))]
        {
            if is_x86_feature_detected!("sse2") {
                unsafe {
                    return simd::x86_sse2::chunk_count(haystack, needle);
                }
            }
        }
        #[cfg(all(
            target_arch = "aarch64",
            target_endian = "little",
            not(feature = "generic-simd")
        ))]
        {
            unsafe {
                return simd::aarch64::chunk_count(haystack, needle);
            }
        }

        #[cfg(target_arch = "wasm32")]
        {
            unsafe {
                return simd::wasm::chunk_count(haystack, needle);
            }
        }
    }

    if haystack.len() >= mem::size_of::<usize>() {
        return integer_simd::chunk_count(haystack, needle);
    }

    naive_count(haystack, needle)
}

/// Count the number of UTF-8 encoded Unicode codepoints in a slice of bytes, fast
///
/// This function is safe to use on any byte array, valid UTF-8 or not,
/// but the output is only meaningful for well-formed UTF-8.
///
/// # Example
///
/// ```
/// let swordfish = "メカジキ";
/// let char_count = bytecount::num_chars(swordfish.as_bytes());
/// assert_eq!(char_count, 4);
/// ```
pub fn num_chars(utf8_chars: &[u8]) -> usize {
    if utf8_chars.len() >= 32 {
        #[cfg(all(feature = "runtime-dispatch-simd", target_arch = "x86_64"))]
        {
            if is_x86_feature_detected!("avx2") {
                unsafe {
                    return simd::x86_avx2::chunk_num_chars(utf8_chars);
                }
            }
        }

        #[cfg(feature = "generic-simd")]
        return simd::generic::chunk_num_chars(utf8_chars);
    }

    if utf8_chars.len() >= 16 {
        #[cfg(all(
            feature = "runtime-dispatch-simd",
            any(target_arch = "x86", target_arch = "x86_64"),
            not(feature = "generic-simd")
        ))]
        {
            if is_x86_feature_detected!("sse2") {
                unsafe {
                    return simd::x86_sse2::chunk_num_chars(utf8_chars);
                }
            }
        }
        #[cfg(all(
            target_arch = "aarch64",
            target_endian = "little",
            not(feature = "generic-simd")
        ))]
        {
            unsafe {
                return simd::aarch64::chunk_num_chars(utf8_chars);
            }
        }

        #[cfg(target_arch = "wasm32")]
        {
            unsafe {
                return simd::wasm::chunk_num_chars(utf8_chars);
            }
        }
    }

    if utf8_chars.len() >= mem::size_of::<usize>() {
        return integer_simd::chunk_num_chars(utf8_chars);
    }

    naive_num_chars(utf8_chars)
}
