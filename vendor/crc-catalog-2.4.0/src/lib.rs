#![no_std]

#[rustfmt::skip]
pub mod algorithm;
pub mod poly;
pub use algorithm::*;

pub trait Width: Sized + 'static {}
impl Width for u8 {}
impl Width for u16 {}
impl Width for u32 {}
impl Width for u64 {}
impl Width for u128 {}

/// This struct describes a CRC algorithm using the fields specified by the [Catalogue of
/// parametrised CRC algorithms](https://reveng.sourceforge.io/crc-catalogue/all.htm).
pub struct Algorithm<W: Width> {
    /// The number of bit cells in the linear feedback shift register; the degree of the generator
    /// polynomial, minus one.
    pub width: u8,
    /// The generator polynomial that sets the feedback tap positions of the shift register. The
    /// least significant bit corresponds to the inward end of the shift register, and is always
    /// set. The highest-order term is omitted.
    pub poly: W,
    /// The settings of the bit cells at the start of each calculation, before reading the first
    /// message bit. The least significant bit corresponds to the inward end of the shift register.
    pub init: W,
    /// If equal to `false`, specifies that the characters of the message are read bit-by-bit, most
    /// significant bit (MSB) first; if equal to `true`, the characters are read bit-by-bit, least
    /// significant bit (LSB) first. Each sampled message bit is then XORed with the bit being
    /// simultaneously shifted out of the register at the most significant end, and the result is
    /// passed to the feedback taps.
    pub refin: bool,
    /// If equal to `false`, specifies that the contents of the register after reading the last
    /// message bit are unreflected before presentation; if equal to `true`, it specifies that they
    /// are reflected, character-by-character, before presentation. For the purpose of this
    /// definition, the reflection is performed by swapping the content of each cell with that of
    /// the cell an equal distance from the opposite end of the register; the characters of the CRC
    /// are then true images of parts of the reflected register, the character containing the
    /// original MSB always appearing first.
    pub refout: bool,
    /// The XOR value applied to the contents of the register after the last message bit has been
    /// read and after the optional reflection. It has the same endianness as the CRC such that its
    /// true image appears in the characters of the CRC.
    pub xorout: W,
    /// The contents of the register after initialising, reading the UTF-8 string `"123456789"` (as
    /// 8-bit characters), optionally reflecting, and applying the final XOR.
    pub check: W,
    /// The contents of the register after initialising, reading an error-free codeword and
    /// optionally reflecting the register (if [`refout`](Algorithm::refout)=`true`), but not
    /// applying the final XOR. This is mathematically equivalent to initialising the register with
    /// the xorout parameter, reflecting it as described (if [`refout`](Algorithm::refout)=`true`),
    /// reading as many zero bits as there are cells in the register, and reflecting the result (if
    /// [`refin`](Algorithm::refin)=`true`). The residue of a crossed-endian model is calculated
    /// assuming that the characters of the received CRC are specially reflected before submitting
    /// the codeword.
    pub residue: W,
}
