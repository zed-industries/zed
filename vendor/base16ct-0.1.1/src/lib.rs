//! Pure Rust implementation of Base16 ([RFC 4648], a.k.a. hex).
//!
//! Implements lower and upper case Base16 variants without data-dependent branches
//! or lookup  tables, thereby providing portable "best effort" constant-time
//! operation. Not constant-time with respect to message length (only data).
//!
//! Supports `no_std` environments and avoids heap allocations in the core API
//! (but also provides optional `alloc` support for convenience).
//!
//! Based on code from: <https://github.com/Sc00bz/ConstTimeEncoding/blob/master/hex.cpp>
//!
//! # Examples
//! ```
//! let lower_hex_str = "abcd1234";
//! let upper_hex_str = "ABCD1234";
//! let mixed_hex_str = "abCD1234";
//! let raw = b"\xab\xcd\x12\x34";
//!
//! let mut buf = [0u8; 16];
//! // length of return slice can be different from the input buffer!
//! let res = base16ct::lower::decode(lower_hex_str, &mut buf).unwrap();
//! assert_eq!(res, raw);
//! let res = base16ct::lower::encode(raw, &mut buf).unwrap();
//! assert_eq!(res, lower_hex_str.as_bytes());
//! // you also can use `encode_str` and `encode_string` to get
//! // `&str` and `String` respectively
//! let res: &str = base16ct::lower::encode_str(raw, &mut buf).unwrap();
//! assert_eq!(res, lower_hex_str);
//!
//! let res = base16ct::upper::decode(upper_hex_str, &mut buf).unwrap();
//! assert_eq!(res, raw);
//! let res = base16ct::upper::encode(raw, &mut buf).unwrap();
//! assert_eq!(res, upper_hex_str.as_bytes());
//!
//! // In cases when you don't know if input contains upper or lower
//! // hex-encoded value, then use functions from the `mixed` module
//! let res = base16ct::mixed::decode(lower_hex_str, &mut buf).unwrap();
//! assert_eq!(res, raw);
//! let res = base16ct::mixed::decode(upper_hex_str, &mut buf).unwrap();
//! assert_eq!(res, raw);
//! let res = base16ct::mixed::decode(mixed_hex_str, &mut buf).unwrap();
//! assert_eq!(res, raw);
//! ```
//!
//! [RFC 4648]: https://tools.ietf.org/html/rfc4648

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs, rust_2018_idioms)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/RustCrypto/media/6ee8e381/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/RustCrypto/media/6ee8e381/logo.svg",
    html_root_url = "https://docs.rs/base16ct/0.1.1"
)]

#[cfg(feature = "alloc")]
#[macro_use]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

/// Function for decoding and encoding lower Base16 (hex)
pub mod lower;
/// Function for decoding mixed Base16 (hex)
pub mod mixed;
/// Function for decoding and encoding upper Base16 (hex)
pub mod upper;

/// Display formatter for hex.
mod display;
/// Error types.
mod error;

pub use crate::{
    display::HexDisplay,
    error::{Error, Result},
};

#[cfg(feature = "alloc")]
use alloc::{string::String, vec::Vec};

/// Compute decoded length of the given hex-encoded input.
#[inline(always)]
pub fn decoded_len(bytes: &[u8]) -> Result<usize> {
    if bytes.len() & 1 == 0 {
        Ok(bytes.len() / 2)
    } else {
        Err(Error::InvalidLength)
    }
}

/// Get the length of Base16 (hex) produced by encoding the given bytes.
#[inline(always)]
pub fn encoded_len(bytes: &[u8]) -> usize {
    bytes.len() * 2
}

fn decode_inner<'a>(
    src: &[u8],
    dst: &'a mut [u8],
    decode_nibble: impl Fn(u8) -> u16,
) -> Result<&'a [u8]> {
    let dst = dst
        .get_mut(..decoded_len(src)?)
        .ok_or(Error::InvalidLength)?;

    let mut err: u16 = 0;
    for (src, dst) in src.chunks_exact(2).zip(dst.iter_mut()) {
        let byte = (decode_nibble(src[0]) << 4) | decode_nibble(src[1]);
        err |= byte >> 8;
        *dst = byte as u8;
    }

    match err {
        0 => Ok(dst),
        _ => Err(Error::InvalidEncoding),
    }
}
