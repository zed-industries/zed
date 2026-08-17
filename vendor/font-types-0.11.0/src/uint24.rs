/// 24-bit unsigned integer.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::AnyBitPattern))]
#[repr(transparent)]
pub struct Uint24(u32);

impl Uint24 {
    /// The smallest value that can be represented by this integer type.
    pub const MIN: Self = Uint24(0);

    /// The largest value that can be represented by this integer type.
    pub const MAX: Self = Uint24(0xffffff);

    /// Create from a u32. Saturates on overflow.
    pub const fn new(raw: u32) -> Uint24 {
        let overflow = raw > Self::MAX.0;
        let raw = raw * !overflow as u32 + Self::MAX.0 * overflow as u32;
        Uint24(raw)
    }

    /// Create from a u32, returning `None` if the value overflows.
    pub const fn checked_new(raw: u32) -> Option<Uint24> {
        if raw > Self::MAX.0 {
            None
        } else {
            Some(Uint24(raw))
        }
    }

    /// Returns this value as an unsigned 32-bit integer.
    pub const fn to_u32(self) -> u32 {
        self.0
    }

    pub const fn to_be_bytes(self) -> [u8; 3] {
        let bytes = self.0.to_be_bytes();
        [bytes[1], bytes[2], bytes[3]]
    }

    pub const fn from_be_bytes(bytes: [u8; 3]) -> Self {
        Uint24::new(((bytes[0] as u32) << 16) | ((bytes[1] as u32) << 8) | bytes[2] as u32)
    }
}

impl From<Uint24> for u32 {
    fn from(src: Uint24) -> u32 {
        src.0
    }
}

impl From<Uint24> for usize {
    fn from(src: Uint24) -> usize {
        src.0 as usize
    }
}

/// Indicates an error converting an integer value into a Uint24 due to overflow.
#[derive(Debug)]
pub struct TryFromUint24Error;

impl std::fmt::Display for TryFromUint24Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "failed to convert usize value into Uint24.")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for TryFromUint24Error {}

impl TryFrom<usize> for Uint24 {
    type Error = TryFromUint24Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let u32_value = u32::try_from(value).map_err(|_| TryFromUint24Error)?;
        Uint24::checked_new(u32_value).ok_or(TryFromUint24Error)
    }
}

impl std::fmt::Display for Uint24 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor() {
        assert_eq!(Uint24::MAX, Uint24::new(u32::MAX));
        assert!(Uint24::checked_new(u32::MAX).is_none())
    }

    #[test]
    fn be_bytes() {
        let bytes = [0xff, 0b10101010, 0b11001100];
        let val = Uint24::from_be_bytes(bytes);
        assert_eq!(val.to_be_bytes(), bytes);
    }
}
