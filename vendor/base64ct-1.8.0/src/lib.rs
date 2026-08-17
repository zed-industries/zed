#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/RustCrypto/media/6ee8e381/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/RustCrypto/media/6ee8e381/logo.svg"
)]
#![doc = include_str!("../README.md")]
#![warn(
    clippy::arithmetic_side_effects,
    clippy::mod_module_files,
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::unwrap_used,
    missing_docs,
    rust_2018_idioms,
    unsafe_code,
    unused_lifetimes,
    unused_qualifications
)]

//! # Usage
//!
//! ## Allocating (enable `alloc` crate feature)
//!
//! ```
//! # #[cfg(feature = "alloc")]
//! # {
//! use base64ct::{Base64, Encoding};
//!
//! let bytes = b"example bytestring!";
//! let encoded = Base64::encode_string(bytes);
//! assert_eq!(encoded, "ZXhhbXBsZSBieXRlc3RyaW5nIQ==");
//!
//! let decoded = Base64::decode_vec(&encoded).unwrap();
//! assert_eq!(decoded, bytes);
//! # }
//! ```
//!
//! ## Heapless `no_std` usage
//!
//! ```
//! use base64ct::{Base64, Encoding};
//!
//! const BUF_SIZE: usize = 128;
//!
//! let bytes = b"example bytestring!";
//! assert!(Base64::encoded_len(bytes) <= BUF_SIZE);
//!
//! let mut enc_buf = [0u8; BUF_SIZE];
//! let encoded = Base64::encode(bytes, &mut enc_buf).unwrap();
//! assert_eq!(encoded, "ZXhhbXBsZSBieXRlc3RyaW5nIQ==");
//!
//! let mut dec_buf = [0u8; BUF_SIZE];
//! let decoded = Base64::decode(encoded, &mut dec_buf).unwrap();
//! assert_eq!(decoded, bytes);
//! ```
//!
//! # Implementation
//!
//! Implemented using integer arithmetic alone without any lookup tables or
//! data-dependent branches, thereby providing portable "best effort"
//! constant-time operation.
//!
//! Not constant-time with respect to message length (only data).
//!
//! Adapted from the following constant-time C++ implementation of Base64:
//!
//! <https://github.com/Sc00bz/ConstTimeEncoding/blob/master/base64.cpp>
//!
//! Copyright (c) 2014 Steve "Sc00bz" Thomas (steve at tobtu dot com).
//! Derived code is dual licensed MIT + Apache 2 (with permission from Sc00bz).

#[cfg(feature = "alloc")]
#[macro_use]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

mod alphabet;
mod decoder;
mod encoder;
mod encoding;
mod errors;
mod line_ending;

#[cfg(test)]
mod test_vectors;

pub use crate::{
    alphabet::{
        bcrypt::Base64Bcrypt,
        crypt::Base64Crypt,
        shacrypt::Base64ShaCrypt,
        standard::{Base64, Base64Unpadded},
        url::{Base64Url, Base64UrlUnpadded},
    },
    decoder::Decoder,
    encoder::Encoder,
    encoding::Encoding,
    errors::{Error, InvalidEncodingError, InvalidLengthError},
    line_ending::LineEnding,
};

/// Minimum supported line width.
const MIN_LINE_WIDTH: usize = 4;
