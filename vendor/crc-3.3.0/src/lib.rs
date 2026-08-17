//! # crc
//! Rust implementation of CRC.
//!
//! ### Examples
//! Using a well-known algorithm:
//! ```rust
//! const X25: crc::Crc<u16> = crc::Crc::<u16>::new(&crc::CRC_16_IBM_SDLC);
//! assert_eq!(X25.checksum(b"123456789"), 0x906e);
//! ```
//!
//! Using a custom algorithm:
//! ```rust
//! const CUSTOM_ALG: crc::Algorithm<u16> = crc::Algorithm {
//!     width: 16,
//!     poly: 0x8005,
//!     init: 0xffff,
//!     refin: false,
//!     refout: false,
//!     xorout: 0x0000,
//!     check: 0xaee7,
//!     residue: 0x0000
//! };
//! let crc = crc::Crc::<u16>::new(&CUSTOM_ALG);
//! let mut digest = crc.digest();
//! digest.update(b"123456789");
//! assert_eq!(digest.finalize(), 0xaee7);
//! ```
#![no_std]
#![forbid(unsafe_code)]

pub use crc_catalog::algorithm::*;
pub use crc_catalog::{Algorithm, Width};

mod crc128;
mod crc16;
mod crc32;
mod crc64;
mod crc8;
mod table;
mod util;

/// A trait for CRC implementations.
pub trait Implementation {
    /// Associated data necessary for the implementation (e.g. lookup tables).
    type Data<W>;
}

/// A table-based implementation of the CRC algorithm, with `L` lanes.
/// The number of entries in the lookup table is `L * 256`.
#[derive(Copy, Clone)]
pub struct Table<const L: usize> {}

/// An implementation of the CRC algorithm with no lookup table.
pub type NoTable = Table<0>;

type DefaultImpl = Table<1>;

impl<const L: usize> Implementation for Table<L> {
    type Data<W> = [[W; 256]; L];
}

mod private {
    pub trait Sealed {}
    impl Sealed for super::Table<0> {}
    impl Sealed for super::Table<1> {}
    impl Sealed for super::Table<16> {}
}

/// Crc instance with a specific width, algorithm, and implementation.
#[derive(Clone)]
pub struct Crc<W: Width, I: Implementation = DefaultImpl> {
    pub algorithm: &'static Algorithm<W>,
    data: I::Data<W>,
}

#[derive(Clone)]
pub struct Digest<'a, W: Width, I: Implementation = DefaultImpl> {
    crc: &'a Crc<W, I>,
    value: W,
}

#[cfg(test)]
mod test {
    use super::{Crc, CRC_32_ISCSI};

    #[test]
    fn test_clone() {
        const CRC: Crc<u32> = Crc::<u32>::new(&CRC_32_ISCSI);
        let crc = CRC.clone();
        let digest = crc.digest();
        let _digest = digest.clone();
    }
}
