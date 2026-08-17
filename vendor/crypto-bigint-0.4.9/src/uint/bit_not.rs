//! [`UInt`] bitwise not operations.

use super::UInt;
use crate::{Limb, Wrapping};
use core::ops::Not;

impl<const LIMBS: usize> UInt<LIMBS> {
    /// Computes bitwise `!a`.
    #[inline(always)]
    pub const fn not(&self) -> Self {
        let mut limbs = [Limb::ZERO; LIMBS];
        let mut i = 0;

        while i < LIMBS {
            limbs[i] = self.limbs[i].not();
            i += 1;
        }

        Self { limbs }
    }
}

impl<const LIMBS: usize> Not for UInt<LIMBS> {
    type Output = Self;

    fn not(self) -> <Self as Not>::Output {
        (&self).not()
    }
}

impl<const LIMBS: usize> Not for Wrapping<UInt<LIMBS>> {
    type Output = Self;

    fn not(self) -> <Self as Not>::Output {
        Wrapping(self.0.not())
    }
}

#[cfg(test)]
mod tests {
    use crate::U128;

    #[test]
    fn bitnot_ok() {
        assert_eq!(U128::ZERO.not(), U128::MAX);
        assert_eq!(U128::MAX.not(), U128::ZERO);
    }
}
