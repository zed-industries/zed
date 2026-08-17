//! Limb bit or operations.

use super::Limb;
use core::ops::BitOr;

impl Limb {
    /// Calculates `a | b`.
    pub const fn bitor(self, rhs: Self) -> Self {
        Limb(self.0 | rhs.0)
    }
}

impl BitOr for Limb {
    type Output = Limb;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.bitor(rhs)
    }
}
