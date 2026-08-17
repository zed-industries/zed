use super::Limb;

impl Limb {
    /// Calculate the number of bits needed to represent this number.
    pub const fn bits(self) -> usize {
        Limb::BIT_SIZE - (self.0.leading_zeros() as usize)
    }
}
