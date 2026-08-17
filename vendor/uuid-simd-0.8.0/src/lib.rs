//! SIMD-accelerated UUID operations.
//!
//! # Examples
//!
//! ```
//! # #[cfg(feature="uuid")]
//! # {
//! use uuid::Uuid;
//! use uuid_simd::UuidExt;
//!
//! let text = "67e55044-10b1-426f-9247-bb680e5fe0c8";
//! let uuid: Uuid = Uuid::parse(text.as_bytes()).unwrap();
//! println!("{}", uuid.format_simple())
//! # }
//! ```
//!
#![doc=vsimd::shared_docs!()]
//
#![cfg_attr(not(any(feature = "std", test)), no_std)]
#![cfg_attr(feature = "unstable", feature(stdsimd))]
#![cfg_attr(feature = "unstable", feature(arm_target_feature))]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(test, deny(warnings))]
//
#![deny(
    missing_debug_implementations,
    missing_docs,
    clippy::all,
    clippy::pedantic,
    clippy::cargo,
    clippy::missing_inline_in_public_items
)]
#![warn(clippy::todo)]
#![allow(
    clippy::inline_always,
    clippy::wildcard_imports,
    clippy::module_name_repetitions,
    clippy::cast_sign_loss,
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::items_after_statements
)]

#[macro_use]
mod error;
pub use self::error::Error;

mod spec;

mod format;
mod parse;

mod multiversion;

#[cfg(feature = "uuid")]
vsimd::item_group! {
    mod ext;
    pub use self::ext::*;
}

pub use outref::{AsOut, Out};
pub use vsimd::ascii::AsciiCase;

// -------------------------------------------------------------------------------------------------

use vsimd::tools::read;

/// Parses an UUID from arbitrary bytes.
///
/// # Errors
/// This function returns `Err` if:
///
/// + The length of `src` doesn't match any UUID format variants.
/// + The content of `src` is invalid.
#[inline]
pub fn parse<'d>(src: &[u8], mut dst: Out<'d, [u8; 16]>) -> Result<&'d mut [u8; 16], Error> {
    let n = src.len();

    if n == 32 {
        unsafe {
            let src = src.as_ptr();
            let dst = dst.as_mut_ptr().cast::<u8>();
            crate::multiversion::parse_simple::auto(src, dst)?;
            return Ok(&mut *dst.cast());
        }
    }

    unsafe {
        let src = match n {
            36 => src.as_ptr(),
            // Microsoft GUID
            38 => {
                let src = src.as_ptr();
                ensure!(read(src, 0) == b'{' && read(src, 37) == b'}');
                src.add(1)
            }
            // URN prefixed UUID
            45 => src.strip_prefix(b"urn:uuid:").ok_or_else(Error::new)?.as_ptr(),
            _ => return Err(Error::new()),
        };
        let dst = dst.as_mut_ptr().cast::<u8>();
        crate::multiversion::parse_hyphenated::auto(src, dst)?;
        Ok(&mut *dst.cast())
    }
}

/// Parses a simple UUID from arbitrary bytes.
///
/// # Errors
/// This function returns `Err` if:
///
/// + The length of `src` doesn't match the "simple" format.
/// + The content of `src` is invalid.
#[inline]
pub fn parse_simple<'d>(src: &[u8], mut dst: Out<'d, [u8; 16]>) -> Result<&'d mut [u8; 16], Error> {
    ensure!(src.len() == 32);
    unsafe {
        let src = src.as_ptr();
        let dst = dst.as_mut_ptr().cast::<u8>();
        crate::multiversion::parse_simple::auto(src, dst)?;
        Ok(&mut *dst.cast())
    }
}

/// Parses a hyphenated UUID from arbitrary bytes.
///
/// # Errors
/// This function returns `Err` if:
///
/// + The length of `src` doesn't match the "hyphenated" format.
/// + The content of `src` is invalid.
#[inline]
pub fn parse_hyphenated<'d>(src: &[u8], mut dst: Out<'d, [u8; 16]>) -> Result<&'d mut [u8; 16], Error> {
    ensure!(src.len() == 36);
    unsafe {
        let src = src.as_ptr();
        let dst = dst.as_mut_ptr().cast::<u8>();
        crate::multiversion::parse_hyphenated::auto(src, dst)?;
        Ok(&mut *dst.cast())
    }
}

/// Formats an UUID to a simple UUID string.
#[inline]
#[must_use]
pub fn format_simple<'d>(src: &[u8; 16], mut dst: Out<'d, [u8; 32]>, case: AsciiCase) -> &'d mut [u8; 32] {
    unsafe {
        let src = src.as_ptr();
        let dst = dst.as_mut_ptr().cast::<u8>();
        crate::multiversion::format_simple::auto(src, dst, case);
        &mut *dst.cast()
    }
}

/// Formats an UUID to a hyphenated UUID string.
#[inline]
#[must_use]
pub fn format_hyphenated<'d>(src: &[u8; 16], mut dst: Out<'d, [u8; 36]>, case: AsciiCase) -> &'d mut [u8; 36] {
    unsafe {
        let src = src.as_ptr();
        let dst = dst.as_mut_ptr().cast::<u8>();
        crate::multiversion::format_hyphenated::auto(src, dst, case);
        &mut *dst.cast()
    }
}
