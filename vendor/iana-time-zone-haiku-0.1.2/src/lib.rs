#![warn(clippy::all)]
#![warn(clippy::cargo)]
#![warn(clippy::undocumented_unsafe_blocks)]
#![allow(unknown_lints)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]
#![warn(trivial_casts, trivial_numeric_casts)]
#![warn(unsafe_op_in_unsafe_fn)]
#![warn(unused_qualifications)]
#![warn(variant_size_differences)]

//! # iana-time-zone-haiku
//!
//! [![Crates.io](https://img.shields.io/crates/v/iana-time-zone-haiku.svg)](https://crates.io/crates/iana-time-zone-haiku)
//! [![Documentation](https://docs.rs/iana-time-zone/badge.svg)](https://docs.rs/iana-time-zone/)
//! [![Crate License](https://img.shields.io/crates/l/iana-time-zone-haiku-haiku.svg)](https://crates.io/crates/iana-time-zone-haiku)
//! [![build](https://github.com/strawlab/iana-time-zone/workflows/build/badge.svg?branch=main)](https://github.com/strawlab/iana-time-zone/actions?query=branch%3Amain)
//!
//! [iana-time-zone](https://github.com/strawlab/iana-time-zone) support crate for Haiku OS.

use std::os::raw::c_char;

extern "C" {
    fn iana_time_zone_haiku_get_tz(buf: *mut c_char, buf_size: usize) -> usize;
}

/// Get the current IANA time zone as a string.
///
/// On Haiku platforms this function will return [`Some`] with the timezone string
/// or [`None`] if an error occurs. On all other platforms, [`None`] is returned.
///
/// # Examples
///
/// ```
/// let timezone = iana_time_zone_haiku::get_timezone();
/// ```
#[must_use]
pub fn get_timezone() -> Option<String> {
    // The longest name in the IANA time zone database is 25 ASCII characters long.
    let mut buf = [0u8; 32];
    // SAFETY: a valid, aligned, non-NULL pointer and length are given which
    // point to a single allocation.
    let len = unsafe {
        let buf_size = buf.len();
        let buf_ptr = buf.as_mut_ptr().cast::<c_char>();
        iana_time_zone_haiku_get_tz(buf_ptr, buf_size)
    };
    // The name should not be empty, or excessively long.
    match buf.get(..len)? {
        b"" => None,
        s => Some(std::str::from_utf8(s).ok()?.to_owned()),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[cfg(not(target_os = "haiku"))]
    fn test_fallback_on_non_haiku_platforms() {
        assert!(super::get_timezone().is_none());
    }

    #[test]
    #[cfg(target_os = "haiku")]
    fn test_retrieve_time_zone_on_haiku_platforms() {
        let timezone = super::get_timezone().unwrap();
        assert!(!timezone.is_empty());
    }
}
