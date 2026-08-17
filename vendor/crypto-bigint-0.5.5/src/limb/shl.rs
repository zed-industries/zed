//! Limb left bitshift

use crate::{Limb, Word};
use core::ops::{Shl, ShlAssign};

impl Limb {
    /// Computes `self << rhs`.
    /// Panics if `rhs` overflows `Limb::BITS`.
    #[inline(always)]
    pub const fn shl(self, rhs: Self) -> Self {
        Limb(self.0 << rhs.0)
    }
}

impl Shl for Limb {
    type Output = Self;

    #[inline(always)]
    fn shl(self, rhs: Self) -> Self::Output {
        self.shl(rhs)
    }
}

impl Shl<usize> for Limb {
    type Output = Self;

    #[inline(always)]
    fn shl(self, rhs: usize) -> Self::Output {
        self.shl(Limb(rhs as Word))
    }
}

impl ShlAssign for Limb {
    #[inline(always)]
    fn shl_assign(&mut self, other: Self) {
        *self = self.shl(other);
    }
}

impl ShlAssign<usize> for Limb {
    #[inline(always)]
    fn shl_assign(&mut self, other: usize) {
        *self = self.shl(Limb(other as Word));
    }
}

#[cfg(test)]
mod tests {
    use crate::Limb;

    #[test]
    fn shl1() {
        assert_eq!(Limb(1) << 1, Limb(2));
    }

    #[test]
    fn shl2() {
        assert_eq!(Limb(1) << 2, Limb(4));
    }

    #[test]
    fn shl_assign1() {
        let mut l = Limb(1);
        l <<= 1;
        assert_eq!(l, Limb(2));
    }

    #[test]
    fn shl_assign2() {
        let mut l = Limb(1);
        l <<= 2;
        assert_eq!(l, Limb(4));
    }
}
