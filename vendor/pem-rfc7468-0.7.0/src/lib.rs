#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/RustCrypto/media/6ee8e381/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/RustCrypto/media/6ee8e381/logo.svg"
)]
#![deny(unsafe_code)]
#![warn(
    clippy::integer_arithmetic,
    clippy::mod_module_files,
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::unwrap_used,
    missing_docs,
    rust_2018_idioms,
    unused_lifetimes,
    unused_qualifications
)]

//! # Usage
//!
#![cfg_attr(feature = "std", doc = " ```")]
#![cfg_attr(not(feature = "std"), doc = " ```ignore")]
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! /// Example PEM document
//! /// NOTE: do not actually put private key literals into your source code!!!
//! let example_pem = "\
//! -----BEGIN PRIVATE KEY-----
//! MC4CAQAwBQYDK2VwBCIEIBftnHPp22SewYmmEoMcX8VwI4IHwaqd+9LFPj/15eqF
//! -----END PRIVATE KEY-----
//! ";
//!
//! // Decode PEM
//! let (type_label, data) = pem_rfc7468::decode_vec(example_pem.as_bytes())?;
//! assert_eq!(type_label, "PRIVATE KEY");
//! assert_eq!(
//!     data,
//!     &[
//!         48, 46, 2, 1, 0, 48, 5, 6, 3, 43, 101, 112, 4, 34, 4, 32, 23, 237, 156, 115, 233, 219,
//!         100, 158, 193, 137, 166, 18, 131, 28, 95, 197, 112, 35, 130, 7, 193, 170, 157, 251,
//!         210, 197, 62, 63, 245, 229, 234, 133
//!     ]
//! );
//!
//! // Encode PEM
//! use pem_rfc7468::LineEnding;
//! let encoded_pem = pem_rfc7468::encode_string(type_label, LineEnding::default(), &data)?;
//! assert_eq!(&encoded_pem, example_pem);
//! # Ok(())
//! # }
//! ```

#[cfg(feature = "alloc")]
#[macro_use]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

mod decoder;
mod encoder;
mod error;
mod grammar;

pub use crate::{
    decoder::{decode, decode_label, Decoder},
    encoder::{encapsulated_len, encapsulated_len_wrapped, encode, encoded_len, Encoder},
    error::{Error, Result},
};
pub use base64ct::LineEnding;

#[cfg(feature = "alloc")]
pub use crate::{decoder::decode_vec, encoder::encode_string};

/// The pre-encapsulation boundary appears before the encapsulated text.
///
/// From RFC 7468 Section 2:
/// > There are exactly five hyphen-minus (also known as dash) characters ("-")
/// > on both ends of the encapsulation boundaries, no more, no less.
const PRE_ENCAPSULATION_BOUNDARY: &[u8] = b"-----BEGIN ";

/// The post-encapsulation boundary appears immediately after the encapsulated text.
const POST_ENCAPSULATION_BOUNDARY: &[u8] = b"-----END ";

/// Delimiter of encapsulation boundaries.
const ENCAPSULATION_BOUNDARY_DELIMITER: &[u8] = b"-----";

/// Width at which the Base64 body of RFC7468-compliant PEM is wrapped.
///
/// From [RFC7468 § 2]:
///
/// > Generators MUST wrap the base64-encoded lines so that each line
/// > consists of exactly 64 characters except for the final line, which
/// > will encode the remainder of the data (within the 64-character line
/// > boundary), and they MUST NOT emit extraneous whitespace.  Parsers MAY
/// > handle other line sizes.
///
/// [RFC7468 § 2]: https://datatracker.ietf.org/doc/html/rfc7468#section-2
pub const BASE64_WRAP_WIDTH: usize = 64;

/// Buffered Base64 decoder type.
pub type Base64Decoder<'i> = base64ct::Decoder<'i, base64ct::Base64>;

/// Buffered Base64 encoder type.
pub type Base64Encoder<'o> = base64ct::Encoder<'o, base64ct::Base64>;

/// Marker trait for types with an associated PEM type label.
pub trait PemLabel {
    /// Expected PEM type label for a given document, e.g. `"PRIVATE KEY"`
    const PEM_LABEL: &'static str;

    /// Validate that a given label matches the expected label.
    fn validate_pem_label(actual: &str) -> Result<()> {
        if Self::PEM_LABEL == actual {
            Ok(())
        } else {
            Err(Error::UnexpectedTypeLabel {
                expected: Self::PEM_LABEL,
            })
        }
    }
}
