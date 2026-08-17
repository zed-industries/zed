//! Limb bit and operations.

use super::Limb;
use core::ops::BitAnd;

impl Limb {
    /// Calculates `a & b`.
    #[inline(always)]
    pub const fn bitand(self, rhs: Self) -> Self {
        Limb(self.0 & rhs.0)
    }
}

impl BitAnd for Limb {
    type Output = Limb;

    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        self.bitand(rhs)
    }
}
