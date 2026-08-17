use super::Limb;

impl Limb {
    /// Calculate the number of bits needed to represent this number.
    pub const fn bits(self) -> usize {
        Limb::BITS - (self.0.leading_zeros() as usize)
    }

    /// Calculate the number of leading zeros in the binary representation of this number.
    pub const fn leading_zeros(self) -> usize {
        self.0.leading_zeros() as usize
    }

    /// Calculate the number of trailing zeros in the binary representation of this number.
    pub const fn trailing_zeros(self) -> usize {
        self.0.trailing_zeros() as usize
    }

    /// Calculate the number of trailing ones the binary representation of this number.
    pub const fn trailing_ones(self) -> usize {
        self.0.trailing_ones() as usize
    }
}
