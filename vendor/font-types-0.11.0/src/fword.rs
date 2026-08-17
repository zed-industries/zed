//! 16-bit signed and unsigned font-units

use super::Fixed;

/// 16-bit signed quantity in font design units.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::AnyBitPattern))]
#[repr(transparent)]
pub struct FWord(i16);

/// 16-bit unsigned quantity in font design units.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::AnyBitPattern))]
#[repr(transparent)]
pub struct UfWord(u16);

impl FWord {
    pub const fn new(raw: i16) -> Self {
        Self(raw)
    }

    pub const fn to_i16(self) -> i16 {
        self.0
    }

    /// Converts this number to a 16.16 fixed point value.
    pub const fn to_fixed(self) -> Fixed {
        Fixed::from_i32(self.0 as i32)
    }

    /// The representation of this number as a big-endian byte array.
    pub const fn to_be_bytes(self) -> [u8; 2] {
        self.0.to_be_bytes()
    }
}

impl UfWord {
    pub const fn new(raw: u16) -> Self {
        Self(raw)
    }

    pub const fn to_u16(self) -> u16 {
        self.0
    }

    /// Converts this number to a 16.16 fixed point value.
    pub const fn to_fixed(self) -> Fixed {
        Fixed::from_i32(self.0 as i32)
    }

    /// The representation of this number as a big-endian byte array.
    pub const fn to_be_bytes(self) -> [u8; 2] {
        self.0.to_be_bytes()
    }
}

impl std::fmt::Display for FWord {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::fmt::Display for UfWord {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<u16> for UfWord {
    fn from(src: u16) -> Self {
        UfWord(src)
    }
}

impl From<i16> for FWord {
    fn from(src: i16) -> Self {
        FWord(src)
    }
}

impl From<FWord> for i16 {
    fn from(src: FWord) -> Self {
        src.0
    }
}

impl From<UfWord> for u16 {
    fn from(src: UfWord) -> Self {
        src.0
    }
}

crate::newtype_scalar!(FWord, [u8; 2]);
crate::newtype_scalar!(UfWord, [u8; 2]);
//TODO: we can add addition/etc as needed
