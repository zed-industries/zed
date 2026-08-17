//! Limb negation

use crate::{Limb, Wrapping};
use core::ops::Neg;

impl Neg for Wrapping<Limb> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(self.0.wrapping_neg())
    }
}

impl Limb {
    /// Perform wrapping negation.
    #[inline(always)]
    pub const fn wrapping_neg(self) -> Self {
        Limb(self.0.wrapping_neg())
    }
}
