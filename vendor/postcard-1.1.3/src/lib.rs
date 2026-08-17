#![cfg_attr(not(any(test, feature = "use-std")), no_std)]
#![warn(missing_docs)]
#![cfg_attr(not(doctest), doc = include_str!("../README.md"))]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod accumulator;
mod de;

mod eio;

mod error;
pub mod fixint;
mod ser;
mod varint;

// Still experimental! Don't make pub pub.
pub(crate) mod max_size;

/// # Experimental Postcard Features
///
/// Items inside this module require various feature flags, and are not
/// subject to SemVer stability. Items may be removed or deprecated at
/// any point.
///
/// ## Derive
///
/// The `experimental-derive` feature enables one experimental feature:
///
/// * Max size calculation
///
/// ### Max Size Calculation
///
/// This features enables calculation of the Max serialized size of a message as
/// an associated `usize` constant called `POSTCARD_MAX_SIZE`. It also provides a
/// `#[derive(MaxSize)]` macro that can be used for calculating user types.
///
/// This is useful for determining the maximum buffer size needed when receiving
/// or sending a message that has been serialized.
///
/// NOTE: This only covers the size of "plain" flavored messages, e.g. not with COBS
/// or any other Flavors applied. The overhead for these flavors must be calculated
/// separately.
///
/// Please report any missing types, or any incorrectly calculated values.
///
/// ### Message Schema Generation
///
/// This now lives in the `postcard-schema` crate.
pub mod experimental {
    /// Compile time max-serialization size calculation
    #[cfg(feature = "experimental-derive")]
    #[cfg_attr(docsrs, doc(cfg(feature = "experimental-derive")))]
    pub mod max_size {
        // NOTE: This is the trait...
        pub use crate::max_size::MaxSize;
        // NOTE: ...and this is the derive macro
        pub use postcard_derive::MaxSize;
    }

    pub use crate::ser::serialized_size;
}

pub use de::deserializer::Deserializer;
pub use de::flavors as de_flavors;
pub use de::{from_bytes, from_bytes_cobs, take_from_bytes, take_from_bytes_cobs};
pub use error::{Error, Result};
pub use ser::flavors as ser_flavors;
pub use ser::{serialize_with_flavor, serializer::Serializer, to_extend, to_slice, to_slice_cobs};

#[cfg(feature = "heapless")]
pub use ser::{to_vec, to_vec_cobs};

#[cfg(any(feature = "embedded-io-04", feature = "embedded-io-06"))]
pub use ser::to_eio;

#[cfg(any(feature = "embedded-io-04", feature = "embedded-io-06"))]
pub use de::from_eio;

#[cfg(feature = "use-std")]
pub use ser::{to_io, to_stdvec, to_stdvec_cobs};

#[cfg(feature = "use-std")]
pub use de::from_io;

#[cfg(feature = "alloc")]
pub use ser::{to_allocvec, to_allocvec_cobs};

#[cfg(feature = "use-crc")]
pub use {
    de::{from_bytes_crc32, take_from_bytes_crc32},
    ser::to_slice_crc32,
};

#[cfg(all(feature = "use-crc", feature = "heapless"))]
pub use ser::to_vec_crc32;

#[cfg(all(feature = "use-crc", feature = "use-std"))]
pub use ser::to_stdvec_crc32;

#[cfg(all(feature = "use-crc", feature = "alloc"))]
pub use ser::to_allocvec_crc32;

#[cfg(test)]
mod test {
    #[test]
    fn varint_boundary_canon() {
        let x = u32::MAX;
        let mut buf = [0u8; 5];
        let used = crate::to_slice(&x, &mut buf).unwrap();
        let deser: u32 = crate::from_bytes(used).unwrap();
        assert_eq!(deser, u32::MAX);
        assert_eq!(used, &mut [0xFF, 0xFF, 0xFF, 0xFF, 0x0F]);
        let deser: Result<u32, crate::Error> = crate::from_bytes(&[0xFF, 0xFF, 0xFF, 0xFF, 0x1F]);
        assert_eq!(deser, Err(crate::Error::DeserializeBadVarint));
    }

    #[test]
    fn signed_int128() {
        let x = -19490127978232325886905073712831_i128;
        let mut buf = [0u8; 32];
        let used = crate::to_slice(&x, &mut buf).unwrap();
        let deser: i128 = crate::from_bytes(used).unwrap();
        assert_eq!(deser, x);
    }
}
