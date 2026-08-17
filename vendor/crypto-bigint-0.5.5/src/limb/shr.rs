//! Limb right bitshift

use crate::{Limb, Word};
use core::ops::{Shr, ShrAssign};

impl Limb {
    /// Computes `self >> rhs`.
    /// Panics if `rhs` overflows `Limb::BITS`.
    #[inline(always)]
    pub const fn shr(self, rhs: Self) -> Self {
        Limb(self.0 >> rhs.0)
    }
}

impl Shr for Limb {
    type Output = Self;

    #[inline(always)]
    fn shr(self, rhs: Self) -> Self::Output {
        self.shr(rhs)
    }
}

impl Shr<usize> for Limb {
    type Output = Self;

    #[inline(always)]
    fn shr(self, rhs: usize) -> Self::Output {
        self.shr(Limb(rhs as Word))
    }
}

impl ShrAssign for Limb {
    #[inline(always)]
    fn shr_assign(&mut self, other: Self) {
        *self = self.shr(other);
    }
}

impl ShrAssign<usize> for Limb {
    #[inline(always)]
    fn shr_assign(&mut self, other: usize) {
        *self = self.shr(Limb(other as Word));
    }
}

#[cfg(test)]
mod tests {
    use crate::Limb;

    #[test]
    fn shr1() {
        assert_eq!(Limb(2) >> 1, Limb(1));
    }

    #[test]
    fn shr2() {
        assert_eq!(Limb(16) >> 2, Limb(4));
    }

    #[test]
    fn shr_assign1() {
        let mut l = Limb::ONE;
        l >>= 1;
        assert_eq!(l, Limb::ZERO);
    }

    #[test]
    fn shr_assign2() {
        let mut l = Limb(32);
        l >>= 2;
        assert_eq!(l, Limb(8));
    }
}
