//! CRC polynomials and their aliases.
//!
//! These polynomials are collected from the following catalogues:
//! - [Wikipedia](https://wikipedia.org/wiki/Cyclic_redundancy_check#Polynomial_representations_of_cyclic_redundancy_checks)
//! - [Catalogue of parametrised CRC algorithms](https://reveng.sourceforge.io/crc-catalogue/all.htm)
//! - [CRC Polynomial Zoo](https://users.ece.cmu.edu/~koopman/crc/crc32.html)

pub const CRC_16: u16 = 0x8005;
pub const CRC_16_IBM: u16 = CRC_16;
pub const CRC_16_ANSI: u16 = CRC_16;

pub const CRC_32: u32 = 0x04c11db7;
pub const IEEE_802_3: u32 = CRC_32;
