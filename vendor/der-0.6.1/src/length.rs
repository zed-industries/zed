//! Length calculations for encoded ASN.1 DER values

use crate::{Decode, DerOrd, Encode, Error, ErrorKind, Reader, Result, SliceWriter, Writer};
use core::{
    cmp::Ordering,
    fmt,
    ops::{Add, Sub},
};

/// Maximum number of octets in a DER encoding of a [`Length`] using the
/// rules implemented by this crate.
const MAX_DER_OCTETS: usize = 5;

/// Maximum length as a `u32` (256 MiB).
const MAX_U32: u32 = 0xfff_ffff;

/// ASN.1-encoded length.
///
/// Maximum length is defined by the [`Length::MAX`] constant (256 MiB).
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct Length(u32);

impl Length {
    /// Length of `0`
    pub const ZERO: Self = Self(0);

    /// Length of `1`
    pub const ONE: Self = Self(1);

    /// Maximum length currently supported: 256 MiB
    pub const MAX: Self = Self(MAX_U32);

    /// Create a new [`Length`] for any value which fits inside of a [`u16`].
    ///
    /// This function is const-safe and therefore useful for [`Length`] constants.
    pub const fn new(value: u16) -> Self {
        Self(value as u32)
    }

    /// Is this length equal to zero?
    pub fn is_zero(self) -> bool {
        self == Self::ZERO
    }

    /// Get the length of DER Tag-Length-Value (TLV) encoded data if `self`
    /// is the length of the inner "value" portion of the message.
    pub fn for_tlv(self) -> Result<Self> {
        Self::ONE + self.encoded_len()? + self
    }

    /// Perform saturating addition of two lengths.
    pub fn saturating_add(self, rhs: Self) -> Self {
        Self(self.0.saturating_add(rhs.0))
    }

    /// Perform saturating subtraction of two lengths.
    pub fn saturating_sub(self, rhs: Self) -> Self {
        Self(self.0.saturating_sub(rhs.0))
    }

    /// Get initial octet of the encoded length (if one is required).
    ///
    /// From X.690 Section 8.1.3.5:
    /// > In the long form, the length octets shall consist of an initial octet
    /// > and one or more subsequent octets. The initial octet shall be encoded
    /// > as follows:
    /// >
    /// > a) bit 8 shall be one;
    /// > b) bits 7 to 1 shall encode the number of subsequent octets in the
    /// >    length octets, as an unsigned binary integer with bit 7 as the
    /// >    most significant bit;
    /// > c) the value 11111111₂ shall not be used.
    fn initial_octet(self) -> Option<u8> {
        match self.0 {
            0x80..=0xFF => Some(0x81),
            0x100..=0xFFFF => Some(0x82),
            0x10000..=0xFFFFFF => Some(0x83),
            0x1000000..=MAX_U32 => Some(0x84),
            _ => None,
        }
    }
}

impl Add for Length {
    type Output = Result<Self>;

    fn add(self, other: Self) -> Result<Self> {
        self.0
            .checked_add(other.0)
            .ok_or_else(|| ErrorKind::Overflow.into())
            .and_then(TryInto::try_into)
    }
}

impl Add<u8> for Length {
    type Output = Result<Self>;

    fn add(self, other: u8) -> Result<Self> {
        self + Length::from(other)
    }
}

impl Add<u16> for Length {
    type Output = Result<Self>;

    fn add(self, other: u16) -> Result<Self> {
        self + Length::from(other)
    }
}

impl Add<u32> for Length {
    type Output = Result<Self>;

    fn add(self, other: u32) -> Result<Self> {
        self + Length::try_from(other)?
    }
}

impl Add<usize> for Length {
    type Output = Result<Self>;

    fn add(self, other: usize) -> Result<Self> {
        self + Length::try_from(other)?
    }
}

impl Add<Length> for Result<Length> {
    type Output = Self;

    fn add(self, other: Length) -> Self {
        self? + other
    }
}

impl Sub for Length {
    type Output = Result<Self>;

    fn sub(self, other: Length) -> Result<Self> {
        self.0
            .checked_sub(other.0)
            .ok_or_else(|| ErrorKind::Overflow.into())
            .and_then(TryInto::try_into)
    }
}

impl Sub<Length> for Result<Length> {
    type Output = Self;

    fn sub(self, other: Length) -> Self {
        self? - other
    }
}

impl From<u8> for Length {
    fn from(len: u8) -> Length {
        Length(len.into())
    }
}

impl From<u16> for Length {
    fn from(len: u16) -> Length {
        Length(len.into())
    }
}

impl From<Length> for u32 {
    fn from(length: Length) -> u32 {
        length.0
    }
}

impl TryFrom<u32> for Length {
    type Error = Error;

    fn try_from(len: u32) -> Result<Length> {
        if len <= Self::MAX.0 {
            Ok(Length(len))
        } else {
            Err(ErrorKind::Overflow.into())
        }
    }
}

impl TryFrom<usize> for Length {
    type Error = Error;

    fn try_from(len: usize) -> Result<Length> {
        u32::try_from(len)
            .map_err(|_| ErrorKind::Overflow)?
            .try_into()
    }
}

impl TryFrom<Length> for usize {
    type Error = Error;

    fn try_from(len: Length) -> Result<usize> {
        len.0.try_into().map_err(|_| ErrorKind::Overflow.into())
    }
}

impl<'a> Decode<'a> for Length {
    fn decode<R: Reader<'a>>(reader: &mut R) -> Result<Length> {
        match reader.read_byte()? {
            // Note: per X.690 Section 8.1.3.6.1 the byte 0x80 encodes indefinite
            // lengths, which are not allowed in DER, so disallow that byte.
            len if len < 0x80 => Ok(len.into()),
            // 1-4 byte variable-sized length prefix
            tag @ 0x81..=0x84 => {
                let nbytes = tag.checked_sub(0x80).ok_or(ErrorKind::Overlength)? as usize;
                debug_assert!(nbytes <= 4);

                let mut decoded_len = 0u32;
                for _ in 0..nbytes {
                    decoded_len = decoded_len.checked_shl(8).ok_or(ErrorKind::Overflow)?
                        | u32::from(reader.read_byte()?);
                }

                let length = Length::try_from(decoded_len)?;

                // X.690 Section 10.1: DER lengths must be encoded with a minimum
                // number of octets
                if length.initial_octet() == Some(tag) {
                    Ok(length)
                } else {
                    Err(ErrorKind::Overlength.into())
                }
            }
            _ => {
                // We specialize to a maximum 4-byte length (including initial octet)
                Err(ErrorKind::Overlength.into())
            }
        }
    }
}

impl Encode for Length {
    fn encoded_len(&self) -> Result<Length> {
        match self.0 {
            0..=0x7F => Ok(Length(1)),
            0x80..=0xFF => Ok(Length(2)),
            0x100..=0xFFFF => Ok(Length(3)),
            0x10000..=0xFFFFFF => Ok(Length(4)),
            0x1000000..=MAX_U32 => Ok(Length(5)),
            _ => Err(ErrorKind::Overflow.into()),
        }
    }

    fn encode(&self, writer: &mut dyn Writer) -> Result<()> {
        match self.initial_octet() {
            Some(tag_byte) => {
                writer.write_byte(tag_byte)?;

                // Strip leading zeroes
                match self.0.to_be_bytes() {
                    [0, 0, 0, byte] => writer.write_byte(byte),
                    [0, 0, bytes @ ..] => writer.write(&bytes),
                    [0, bytes @ ..] => writer.write(&bytes),
                    bytes => writer.write(&bytes),
                }
            }
            #[allow(clippy::cast_possible_truncation)]
            None => writer.write_byte(self.0 as u8),
        }
    }
}

impl DerOrd for Length {
    fn der_cmp(&self, other: &Self) -> Result<Ordering> {
        let mut buf1 = [0u8; MAX_DER_OCTETS];
        let mut buf2 = [0u8; MAX_DER_OCTETS];

        let mut encoder1 = SliceWriter::new(&mut buf1);
        encoder1.encode(self)?;

        let mut encoder2 = SliceWriter::new(&mut buf2);
        encoder2.encode(other)?;

        Ok(encoder1.finish()?.cmp(encoder2.finish()?))
    }
}

impl fmt::Display for Length {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::Length;
    use crate::{Decode, DerOrd, Encode, ErrorKind};
    use core::cmp::Ordering;

    #[test]
    fn decode() {
        assert_eq!(Length::ZERO, Length::from_der(&[0x00]).unwrap());

        assert_eq!(Length::from(0x7Fu8), Length::from_der(&[0x7F]).unwrap());

        assert_eq!(
            Length::from(0x80u8),
            Length::from_der(&[0x81, 0x80]).unwrap()
        );

        assert_eq!(
            Length::from(0xFFu8),
            Length::from_der(&[0x81, 0xFF]).unwrap()
        );

        assert_eq!(
            Length::from(0x100u16),
            Length::from_der(&[0x82, 0x01, 0x00]).unwrap()
        );

        assert_eq!(
            Length::try_from(0x10000u32).unwrap(),
            Length::from_der(&[0x83, 0x01, 0x00, 0x00]).unwrap()
        );
    }

    #[test]
    fn encode() {
        let mut buffer = [0u8; 4];

        assert_eq!(&[0x00], Length::ZERO.encode_to_slice(&mut buffer).unwrap());

        assert_eq!(
            &[0x7F],
            Length::from(0x7Fu8).encode_to_slice(&mut buffer).unwrap()
        );

        assert_eq!(
            &[0x81, 0x80],
            Length::from(0x80u8).encode_to_slice(&mut buffer).unwrap()
        );

        assert_eq!(
            &[0x81, 0xFF],
            Length::from(0xFFu8).encode_to_slice(&mut buffer).unwrap()
        );

        assert_eq!(
            &[0x82, 0x01, 0x00],
            Length::from(0x100u16).encode_to_slice(&mut buffer).unwrap()
        );

        assert_eq!(
            &[0x83, 0x01, 0x00, 0x00],
            Length::try_from(0x10000u32)
                .unwrap()
                .encode_to_slice(&mut buffer)
                .unwrap()
        );
    }

    #[test]
    fn reject_indefinite_lengths() {
        assert!(Length::from_der(&[0x80]).is_err());
    }

    #[test]
    fn add_overflows_when_max_length_exceeded() {
        let result = Length::MAX + Length::ONE;
        assert_eq!(
            result.err().map(|err| err.kind()),
            Some(ErrorKind::Overflow)
        );
    }

    #[test]
    fn der_ord() {
        assert_eq!(Length::ONE.der_cmp(&Length::MAX).unwrap(), Ordering::Less);
    }
}
