//! [`UInt`] bitwise or operations.

use super::UInt;
use crate::{Limb, Wrapping};
use core::ops::{BitOr, BitOrAssign};
use subtle::{Choice, CtOption};

impl<const LIMBS: usize> UInt<LIMBS> {
    /// Computes bitwise `a & b`.
    #[inline(always)]
    pub const fn bitor(&self, rhs: &Self) -> Self {
        let mut limbs = [Limb::ZERO; LIMBS];
        let mut i = 0;

        while i < LIMBS {
            limbs[i] = self.limbs[i].bitor(rhs.limbs[i]);
            i += 1;
        }

        Self { limbs }
    }

    /// Perform wrapping bitwise `OR`.
    ///
    /// There's no way wrapping could ever happen.
    /// This function exists so that all operations are accounted for in the wrapping operations
    pub const fn wrapping_or(&self, rhs: &Self) -> Self {
        self.bitor(rhs)
    }

    /// Perform checked bitwise `OR`, returning a [`CtOption`] which `is_some` always
    pub fn checked_or(&self, rhs: &Self) -> CtOption<Self> {
        let result = self.bitor(rhs);
        CtOption::new(result, Choice::from(1))
    }
}

impl<const LIMBS: usize> BitOr for UInt<LIMBS> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> UInt<LIMBS> {
        self.bitor(&rhs)
    }
}

impl<const LIMBS: usize> BitOr<&UInt<LIMBS>> for UInt<LIMBS> {
    type Output = UInt<LIMBS>;

    fn bitor(self, rhs: &UInt<LIMBS>) -> UInt<LIMBS> {
        (&self).bitor(rhs)
    }
}

impl<const LIMBS: usize> BitOr<UInt<LIMBS>> for &UInt<LIMBS> {
    type Output = UInt<LIMBS>;

    fn bitor(self, rhs: UInt<LIMBS>) -> UInt<LIMBS> {
        self.bitor(&rhs)
    }
}

impl<const LIMBS: usize> BitOr<&UInt<LIMBS>> for &UInt<LIMBS> {
    type Output = UInt<LIMBS>;

    fn bitor(self, rhs: &UInt<LIMBS>) -> UInt<LIMBS> {
        self.bitor(rhs)
    }
}

impl<const LIMBS: usize> BitOrAssign for UInt<LIMBS> {
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}

impl<const LIMBS: usize> BitOrAssign<&UInt<LIMBS>> for UInt<LIMBS> {
    fn bitor_assign(&mut self, other: &Self) {
        *self = *self | other;
    }
}

impl<const LIMBS: usize> BitOr for Wrapping<UInt<LIMBS>> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Wrapping<UInt<LIMBS>> {
        Wrapping(self.0.bitor(&rhs.0))
    }
}

impl<const LIMBS: usize> BitOr<&Wrapping<UInt<LIMBS>>> for Wrapping<UInt<LIMBS>> {
    type Output = Wrapping<UInt<LIMBS>>;

    fn bitor(self, rhs: &Wrapping<UInt<LIMBS>>) -> Wrapping<UInt<LIMBS>> {
        Wrapping(self.0.bitor(&rhs.0))
    }
}

impl<const LIMBS: usize> BitOr<Wrapping<UInt<LIMBS>>> for &Wrapping<UInt<LIMBS>> {
    type Output = Wrapping<UInt<LIMBS>>;

    fn bitor(self, rhs: Wrapping<UInt<LIMBS>>) -> Wrapping<UInt<LIMBS>> {
        Wrapping(self.0.bitor(&rhs.0))
    }
}

impl<const LIMBS: usize> BitOr<&Wrapping<UInt<LIMBS>>> for &Wrapping<UInt<LIMBS>> {
    type Output = Wrapping<UInt<LIMBS>>;

    fn bitor(self, rhs: &Wrapping<UInt<LIMBS>>) -> Wrapping<UInt<LIMBS>> {
        Wrapping(self.0.bitor(&rhs.0))
    }
}

impl<const LIMBS: usize> BitOrAssign for Wrapping<UInt<LIMBS>> {
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}

impl<const LIMBS: usize> BitOrAssign<&Wrapping<UInt<LIMBS>>> for Wrapping<UInt<LIMBS>> {
    fn bitor_assign(&mut self, other: &Self) {
        *self = *self | other;
    }
}

#[cfg(test)]
mod tests {
    use crate::U128;

    #[test]
    fn checked_or_ok() {
        let result = U128::ZERO.checked_or(&U128::ONE);
        assert_eq!(result.unwrap(), U128::ONE);
    }

    #[test]
    fn overlapping_or_ok() {
        let result = U128::MAX.wrapping_or(&U128::ONE);
        assert_eq!(result, U128::MAX);
    }
}
