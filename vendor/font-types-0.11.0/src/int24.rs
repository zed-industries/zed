/// 24-bit unsigned integer.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "bytemuck", derive(bytemuck::AnyBitPattern))]
#[repr(transparent)]
pub struct Int24(i32);

impl Int24 {
    /// The smallest value that can be represented by this integer type.
    pub const MIN: Self = Int24(-0x80_00_00);

    /// The largest value that can be represented by this integer type.
    pub const MAX: Self = Int24(0x7F_FF_FF);

    /// Create from a i32. Saturates on overflow.
    pub const fn new(raw: i32) -> Int24 {
        let overflow = raw > Self::MAX.0;
        let underflow = raw < Self::MIN.0;
        let raw = raw * !(overflow || underflow) as i32
            + Self::MAX.0 * overflow as i32
            + Self::MIN.0 * underflow as i32;
        Int24(raw)
    }

    /// Create from a i32, returning `None` if the value overflows.
    pub const fn checked_new(raw: i32) -> Option<Int24> {
        if raw > Self::MAX.0 || raw < Self::MIN.0 {
            None
        } else {
            Some(Int24(raw))
        }
    }

    /// Returns this value as an unsigned 32-bit integer.
    pub const fn to_i32(self) -> i32 {
        self.0
    }

    pub const fn to_be_bytes(self) -> [u8; 3] {
        let bytes = self.0.to_be_bytes();
        [bytes[1], bytes[2], bytes[3]]
    }

    pub const fn from_be_bytes(bytes: [u8; 3]) -> Self {
        let extra_byte = ((bytes[0] & 0b10000000) >> 7) * 0b11111111;
        let extra_byte = (extra_byte as i32) << 24;
        Int24::new(
            extra_byte | ((bytes[0] as i32) << 16) | ((bytes[1] as i32) << 8) | bytes[2] as i32,
        )
    }
}

impl From<Int24> for i32 {
    fn from(src: Int24) -> i32 {
        src.0
    }
}

impl std::fmt::Display for Int24 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor() {
        assert_eq!(Int24::MAX, Int24::new(i32::MAX));
        assert_eq!(Int24::MIN, Int24::new(i32::MIN));
        assert_eq!(-10, Int24::new(-10).to_i32());
        assert_eq!(10, Int24::new(10).to_i32());
    }

    #[test]
    fn to_be_bytes() {
        assert_eq!(
            Int24::new(0).to_be_bytes(),
            [0b00000000, 0b00000000, 0b00000000]
        );

        assert_eq!(
            Int24::new(123_456).to_be_bytes(),
            [0b00000001, 0b11100010, 0b01000000]
        );
        assert_eq!(
            Int24::new(-123_456).to_be_bytes(),
            [0b11111110, 0b00011101, 0b11000000]
        );

        assert_eq!(
            Int24::new(0x7F_FF_FF).to_be_bytes(),
            [0b01111111, 0b11111111, 0b11111111]
        );
        assert_eq!(
            Int24::new(-0x80_00_00).to_be_bytes(),
            [0b10000000, 0b00000000, 0b00000000]
        );
    }

    #[test]
    fn from_be_bytes() {
        assert_eq!(
            Int24::from_be_bytes([0b00000000, 0b00000000, 0b00000000]),
            Int24::new(0)
        );

        assert_eq!(
            Int24::from_be_bytes([0b00000001, 0b11100010, 0b01000000]),
            Int24::new(123_456)
        );
        assert_eq!(
            Int24::from_be_bytes([0b11111110, 0b00011101, 0b11000000]),
            Int24::new(-123_456)
        );

        assert_eq!(
            Int24::from_be_bytes([0b01111111, 0b11111111, 0b11111111]),
            Int24::new(0x7F_FF_FF)
        );
        assert_eq!(
            Int24::from_be_bytes([0b10000000, 0b00000000, 0b00000000]),
            Int24::new(-0x80_00_00)
        );
    }

    #[test]
    fn round_trip() {
        for v in Int24::MIN.to_i32()..=Int24::MAX.to_i32() {
            let int = Int24::new(v);
            let bytes = int.to_be_bytes();
            let int_prime = Int24::from_be_bytes(bytes);
            assert_eq!(int_prime, int);
        }
    }
}
