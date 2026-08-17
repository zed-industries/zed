// SPDX-License-Identifier: Apache-2.0

//! Low level CBOR parsing tools
//!
//! This crate contains low-level types for encoding and decoding items in
//! CBOR. This crate is usable in both `no_std` and `no_alloc` environments.
//! To understand how this crate works, first we will look at the structure
//! of a CBOR item on the wire.
//!
//! # Anatomy of a CBOR Item
//!
//! This is a brief anatomy of a CBOR item on the wire.
//!
//! ```text
//! +------------+-----------+
//! |            |           |
//! |   Major    |   Minor   |
//! |  (3bits)   |  (5bits)  |
//! |            |           |
//! +------------+-----------+
//! ^                        ^
//! |                        |
//! +-----+            +-----+
//!       |            |
//!       |            |
//!       +----------------------------+--------------+
//!       |            |               |              |
//!       |   Prefix   |     Affix     |    Suffix    |
//!       |  (1 byte)  |  (0-8 bytes)  |  (0+ bytes)  |
//!       |            |               |              |
//!       +------------+---------------+--------------+
//!
//!       |                            |              |
//!       +------------+---------------+--------------+
//!                    |                       |
//!                    v                       v
//!
//!                  Header                   Body
//! ```
//!
//! The `ciborium` crate works by providing the `Decoder` and `Encoder` types
//! which provide input and output for a CBOR header (see: `Header`). From
//! there, you can either handle the body yourself or use the provided utility
//! functions.
//!
//! For more information on the CBOR format, see
//! [RFC 7049](https://tools.ietf.org/html/rfc7049).
//!
//! # Decoding
//!
//! In order to decode CBOR, you will create a `Decoder` from a reader. The
//! decoder instance will allow you to `Decoder::pull()` `Header` instances
//! from the input.
//!
//! Most CBOR items are fully contained in their headers and therefore have no
//! body. These items can be evaluated directly from the `Header` instance.
//!
//! Bytes and text items have a body but do not contain child items. Since
//! both bytes and text values may be segmented, parsing them can be a bit
//! tricky. Therefore, we provide helper functions to parse these types. See
//! `Decoder::bytes()` and `Decoder::text()` for more details.
//!
//! Array and map items have a body which contains child items. These can be
//! parsed by simply doing `Decoder::pull()` to parse the child items.
//!
//! ## Example
//!
//! ```rust
//! use ciborium_ll::{Decoder, Header};
//! use ciborium_io::Read as _;
//!
//! let input = b"\x6dHello, World!";
//! let mut decoder = Decoder::from(&input[..]);
//! let mut chunks = 0;
//!
//! match decoder.pull().unwrap() {
//!     Header::Text(len) => {
//!         let mut segments = decoder.text(len);
//!         while let Some(mut segment) = segments.pull().unwrap() {
//!             let mut buffer = [0u8; 7];
//!             while let Some(chunk) = segment.pull(&mut buffer[..]).unwrap() {
//!                  match chunk {
//!                      "Hello, " if chunks == 0 => chunks = 1,
//!                      "World!" if chunks == 1 => chunks = 2,
//!                      _ => panic!("received unexpected chunk"),
//!                  }
//!             }
//!         }
//!     }
//!
//!     _ => panic!("received unexpected value"),
//! }
//!
//! assert_eq!(chunks, 2);
//! ```
//!
//! # Encoding
//!
//! To encode values to CBOR, create an `Encoder` from a writer. The encoder
//! instance provides the `Encoder::push()` method to write a `Header` value
//! to the wire. CBOR item bodies can be written directly.
//!
//! For bytes and text, there are the `Encoder::bytes()` and `Encoder::text()`
//! utility functions, respectively, which will properly segment the output
//! on the wire for you.
//!
//! ## Example
//!
//! ```rust
//! use ciborium_ll::{Encoder, Header};
//! use ciborium_io::Write as _;
//!
//! let mut buffer = [0u8; 19];
//! let mut encoder = Encoder::from(&mut buffer[..]);
//!
//! // Write the structure
//! encoder.push(Header::Map(Some(1))).unwrap();
//! encoder.push(Header::Positive(7)).unwrap();
//! encoder.text("Hello, World!", 7).unwrap();
//!
//! // Validate our output
//! encoder.flush().unwrap();
//! assert_eq!(b"\xa1\x07\x7f\x67Hello, \x66World!\xff", &buffer[..]);
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::cargo)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod dec;
mod enc;
mod hdr;
mod seg;

pub use dec::*;
pub use enc::*;
pub use hdr::*;
pub use seg::{Segment, Segments};

/// Simple value constants
pub mod simple {
    #![allow(missing_docs)]

    pub const FALSE: u8 = 20;
    pub const TRUE: u8 = 21;
    pub const NULL: u8 = 22;
    pub const UNDEFINED: u8 = 23;
}

/// Tag constants
pub mod tag {
    #![allow(missing_docs)]

    pub const BIGPOS: u64 = 2;
    pub const BIGNEG: u64 = 3;
}

#[derive(Debug)]
struct InvalidError(());

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Major {
    Positive,
    Negative,
    Bytes,
    Text,
    Array,
    Map,
    Tag,
    Other,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Minor {
    This(u8),
    Next1([u8; 1]),
    Next2([u8; 2]),
    Next4([u8; 4]),
    Next8([u8; 8]),
    More,
}

impl AsRef<[u8]> for Minor {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        match self {
            Self::More => &[],
            Self::This(..) => &[],
            Self::Next1(x) => x.as_ref(),
            Self::Next2(x) => x.as_ref(),
            Self::Next4(x) => x.as_ref(),
            Self::Next8(x) => x.as_ref(),
        }
    }
}

impl AsMut<[u8]> for Minor {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        match self {
            Self::More => &mut [],
            Self::This(..) => &mut [],
            Self::Next1(x) => x.as_mut(),
            Self::Next2(x) => x.as_mut(),
            Self::Next4(x) => x.as_mut(),
            Self::Next8(x) => x.as_mut(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Title(pub Major, pub Minor);

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! neg {
        ($i:expr) => {
            Header::Negative((($i as i128) ^ !0) as u64)
        };
    }

    #[allow(clippy::excessive_precision)]
    #[test]
    fn leaf() {
        use core::f64::{INFINITY, NAN};

        let data = &[
            (Header::Positive(0), "00", true),
            (Header::Positive(1), "01", true),
            (Header::Positive(10), "0a", true),
            (Header::Positive(23), "17", true),
            (Header::Positive(24), "1818", true),
            (Header::Positive(25), "1819", true),
            (Header::Positive(100), "1864", true),
            (Header::Positive(1000), "1903e8", true),
            (Header::Positive(1000000), "1a000f4240", true),
            (Header::Positive(1000000000000), "1b000000e8d4a51000", true),
            (
                Header::Positive(18446744073709551615),
                "1bffffffffffffffff",
                true,
            ),
            (neg!(-18446744073709551616), "3bffffffffffffffff", true),
            (neg!(-1), "20", true),
            (neg!(-10), "29", true),
            (neg!(-100), "3863", true),
            (neg!(-1000), "3903e7", true),
            (Header::Float(0.0), "f90000", true),
            (Header::Float(-0.0), "f98000", true),
            (Header::Float(1.0), "f93c00", true),
            (Header::Float(1.1), "fb3ff199999999999a", true),
            (Header::Float(1.5), "f93e00", true),
            (Header::Float(65504.0), "f97bff", true),
            (Header::Float(100000.0), "fa47c35000", true),
            (Header::Float(3.4028234663852886e+38), "fa7f7fffff", true),
            (Header::Float(1.0e+300), "fb7e37e43c8800759c", true),
            (Header::Float(5.960464477539063e-8), "f90001", true),
            (Header::Float(0.00006103515625), "f90400", true),
            (Header::Float(-4.0), "f9c400", true),
            (Header::Float(-4.1), "fbc010666666666666", true),
            (Header::Float(INFINITY), "f97c00", true),
            (Header::Float(NAN), "f97e00", true),
            (Header::Float(-INFINITY), "f9fc00", true),
            (Header::Float(INFINITY), "fa7f800000", false),
            (Header::Float(NAN), "fa7fc00000", false),
            (Header::Float(-INFINITY), "faff800000", false),
            (Header::Float(INFINITY), "fb7ff0000000000000", false),
            (Header::Float(NAN), "fb7ff8000000000000", false),
            (Header::Float(-INFINITY), "fbfff0000000000000", false),
            (Header::Simple(simple::FALSE), "f4", true),
            (Header::Simple(simple::TRUE), "f5", true),
            (Header::Simple(simple::NULL), "f6", true),
            (Header::Simple(simple::UNDEFINED), "f7", true),
            (Header::Simple(16), "f0", true),
            (Header::Simple(24), "f818", true),
            (Header::Simple(255), "f8ff", true),
            (Header::Tag(0), "c0", true),
            (Header::Tag(1), "c1", true),
            (Header::Tag(23), "d7", true),
            (Header::Tag(24), "d818", true),
            (Header::Tag(32), "d820", true),
            (Header::Bytes(Some(0)), "40", true),
            (Header::Bytes(Some(4)), "44", true),
            (Header::Text(Some(0)), "60", true),
            (Header::Text(Some(4)), "64", true),
        ];

        for (header, bytes, encode) in data.iter().cloned() {
            let bytes = hex::decode(bytes).unwrap();

            let mut decoder = Decoder::from(&bytes[..]);
            match (header, decoder.pull().unwrap()) {
                // NaN equality...
                (Header::Float(l), Header::Float(r)) if l.is_nan() && r.is_nan() => (),

                // Everything else...
                (l, r) => assert_eq!(l, r),
            }

            if encode {
                let mut buffer = [0u8; 1024];
                let mut writer = &mut buffer[..];
                let mut encoder = Encoder::from(&mut writer);
                encoder.push(header).unwrap();

                let len = writer.len();
                assert_eq!(&bytes[..], &buffer[..1024 - len]);
            }
        }
    }

    #[test]
    fn node() {
        let data: &[(&str, &[Header])] = &[
            ("80", &[Header::Array(Some(0))]),
            (
                "83010203",
                &[
                    Header::Array(Some(3)),
                    Header::Positive(1),
                    Header::Positive(2),
                    Header::Positive(3),
                ],
            ),
            (
                "98190102030405060708090a0b0c0d0e0f101112131415161718181819",
                &[
                    Header::Array(Some(25)),
                    Header::Positive(1),
                    Header::Positive(2),
                    Header::Positive(3),
                    Header::Positive(4),
                    Header::Positive(5),
                    Header::Positive(6),
                    Header::Positive(7),
                    Header::Positive(8),
                    Header::Positive(9),
                    Header::Positive(10),
                    Header::Positive(11),
                    Header::Positive(12),
                    Header::Positive(13),
                    Header::Positive(14),
                    Header::Positive(15),
                    Header::Positive(16),
                    Header::Positive(17),
                    Header::Positive(18),
                    Header::Positive(19),
                    Header::Positive(20),
                    Header::Positive(21),
                    Header::Positive(22),
                    Header::Positive(23),
                    Header::Positive(24),
                    Header::Positive(25),
                ],
            ),
            ("a0", &[Header::Map(Some(0))]),
            (
                "a201020304",
                &[
                    Header::Map(Some(2)),
                    Header::Positive(1),
                    Header::Positive(2),
                    Header::Positive(3),
                    Header::Positive(4),
                ],
            ),
            ("9fff", &[Header::Array(None), Header::Break]),
            (
                "9f018202039f0405ffff",
                &[
                    Header::Array(None),
                    Header::Positive(1),
                    Header::Array(Some(2)),
                    Header::Positive(2),
                    Header::Positive(3),
                    Header::Array(None),
                    Header::Positive(4),
                    Header::Positive(5),
                    Header::Break,
                    Header::Break,
                ],
            ),
            (
                "9f01820203820405ff",
                &[
                    Header::Array(None),
                    Header::Positive(1),
                    Header::Array(Some(2)),
                    Header::Positive(2),
                    Header::Positive(3),
                    Header::Array(Some(2)),
                    Header::Positive(4),
                    Header::Positive(5),
                    Header::Break,
                ],
            ),
            (
                "83018202039f0405ff",
                &[
                    Header::Array(Some(3)),
                    Header::Positive(1),
                    Header::Array(Some(2)),
                    Header::Positive(2),
                    Header::Positive(3),
                    Header::Array(None),
                    Header::Positive(4),
                    Header::Positive(5),
                    Header::Break,
                ],
            ),
            (
                "83019f0203ff820405",
                &[
                    Header::Array(Some(3)),
                    Header::Positive(1),
                    Header::Array(None),
                    Header::Positive(2),
                    Header::Positive(3),
                    Header::Break,
                    Header::Array(Some(2)),
                    Header::Positive(4),
                    Header::Positive(5),
                ],
            ),
            (
                "9f0102030405060708090a0b0c0d0e0f101112131415161718181819ff",
                &[
                    Header::Array(None),
                    Header::Positive(1),
                    Header::Positive(2),
                    Header::Positive(3),
                    Header::Positive(4),
                    Header::Positive(5),
                    Header::Positive(6),
                    Header::Positive(7),
                    Header::Positive(8),
                    Header::Positive(9),
                    Header::Positive(10),
                    Header::Positive(11),
                    Header::Positive(12),
                    Header::Positive(13),
                    Header::Positive(14),
                    Header::Positive(15),
                    Header::Positive(16),
                    Header::Positive(17),
                    Header::Positive(18),
                    Header::Positive(19),
                    Header::Positive(20),
                    Header::Positive(21),
                    Header::Positive(22),
                    Header::Positive(23),
                    Header::Positive(24),
                    Header::Positive(25),
                    Header::Break,
                ],
            ),
        ];

        for (bytes, headers) in data {
            let bytes = hex::decode(bytes).unwrap();

            // Test decoding
            let mut decoder = Decoder::from(&bytes[..]);
            for header in headers.iter().cloned() {
                assert_eq!(header, decoder.pull().unwrap());
            }

            // Test encoding
            let mut buffer = [0u8; 1024];
            let mut writer = &mut buffer[..];
            let mut encoder = Encoder::from(&mut writer);

            for header in headers.iter().cloned() {
                encoder.push(header).unwrap();
            }

            let len = writer.len();
            assert_eq!(&bytes[..], &buffer[..1024 - len]);
        }
    }
}
