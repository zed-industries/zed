//! # `cobs`
//!
//! This is an implementation of the Consistent Overhead Byte Stuffing (COBS) algorithm in Rust.
//!
//! COBS is an algorithm for transforming a message into an encoding where a specific value (the
//! "sentinel" value) is not used. This value can then be used to mark frame boundaries in a serial
//! communication channel.
//!
//! See the [wikipedia article](https://www.wikipedia.org/wiki/Consistent_Overhead_Byte_Stuffing) for details.
//!
//! ## Features
//!
//! `cobs` supports various runtime environments and is also suitable for `no_std` environments.
//!
//! ### Default features
//!
//! - [`std`](https://doc.rust-lang.org/std/): Enables functionality relying on the standard library
//!   and also activates the `alloc` feature. Currently only adds [std::error::Error] support for the
//!   library error types.
//! - [`alloc`](https://doc.rust-lang.org/alloc/): Enables features which operate on containers
//!   like [alloc::vec::Vec](https://doc.rust-lang.org/beta/alloc/vec/struct.Vec.html).
//!   Enabled by the `std` feature.
//!
//! ### Optional features
//!
//! - [`defmt`](https://docs.rs/defmt/latest/defmt/): Adds `defmt::Format` derives on some data
//!   structures and error types.
//! - [`serde`](https://serde.rs/): Adds `serde` derives on some data structures and error types.
#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

// In the future, don't do this.
mod dec;
mod enc;
pub use crate::dec::*;
pub use crate::enc::*;

/// Calculates the maximum overhead when encoding a message with the given length.
/// The overhead is a maximum of [n/254] bytes (one in 254 bytes) rounded up.
#[inline]
pub const fn max_encoding_overhead(source_len: usize) -> usize {
    source_len.div_ceil(254)
}

/// Calculates the maximum possible size of an encoded message given the length
/// of the source message. This may be useful for calculating how large the
/// `dest` buffer needs to be in the encoding functions.
#[inline]
pub const fn max_encoding_length(source_len: usize) -> usize {
    source_len + max_encoding_overhead(source_len)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Usable in const context
    const ENCODED_BUF: [u8; max_encoding_length(5)] = [0; max_encoding_length(5)];

    #[test]
    fn test_buf_len() {
        assert_eq!(ENCODED_BUF.len(), 6);
    }

    #[test]
    fn test_overhead_empty() {
        assert_eq!(max_encoding_overhead(0), 0);
    }

    #[test]
    fn test_overhead_one() {
        assert_eq!(max_encoding_overhead(1), 1);
    }

    #[test]
    fn test_overhead_larger() {
        assert_eq!(max_encoding_overhead(253), 1);
        assert_eq!(max_encoding_overhead(254), 1);
    }

    #[test]
    fn test_overhead_two() {
        assert_eq!(max_encoding_overhead(255), 2);
    }
}
