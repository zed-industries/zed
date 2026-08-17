//! [`Uint`] bitwise or operations.

use super::Uint;
use crate::{Limb, Wrapping};
use core::ops::{BitOr, BitOrAssign};
use subtle::{Choice, CtOption};

impl<const LIMBS: usize> Uint<LIMBS> {
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

impl<const LIMBS: usize> BitOr for Uint<LIMBS> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Uint<LIMBS> {
        self.bitor(&rhs)
    }
}

impl<const LIMBS: usize> BitOr<&Uint<LIMBS>> for Uint<LIMBS> {
    type Output = Uint<LIMBS>;

    #[allow(clippy::needless_borrow)]
    fn bitor(self, rhs: &Uint<LIMBS>) -> Uint<LIMBS> {
        (&self).bitor(rhs)
    }
}

impl<const LIMBS: usize> BitOr<Uint<LIMBS>> for &Uint<LIMBS> {
    type Output = Uint<LIMBS>;

    fn bitor(self, rhs: Uint<LIMBS>) -> Uint<LIMBS> {
        self.bitor(&rhs)
    }
}

impl<const LIMBS: usize> BitOr<&Uint<LIMBS>> for &Uint<LIMBS> {
    type Output = Uint<LIMBS>;

    fn bitor(self, rhs: &Uint<LIMBS>) -> Uint<LIMBS> {
        self.bitor(rhs)
    }
}

impl<const LIMBS: usize> BitOrAssign for Uint<LIMBS> {
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}

impl<const LIMBS: usize> BitOrAssign<&Uint<LIMBS>> for Uint<LIMBS> {
    fn bitor_assign(&mut self, other: &Self) {
        *self = *self | other;
    }
}

impl<const LIMBS: usize> BitOr for Wrapping<Uint<LIMBS>> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Wrapping<Uint<LIMBS>> {
        Wrapping(self.0.bitor(&rhs.0))
    }
}

impl<const LIMBS: usize> BitOr<&Wrapping<Uint<LIMBS>>> for Wrapping<Uint<LIMBS>> {
    type Output = Wrapping<Uint<LIMBS>>;

    fn bitor(self, rhs: &Wrapping<Uint<LIMBS>>) -> Wrapping<Uint<LIMBS>> {
        Wrapping(self.0.bitor(&rhs.0))
    }
}

impl<const LIMBS: usize> BitOr<Wrapping<Uint<LIMBS>>> for &Wrapping<Uint<LIMBS>> {
    type Output = Wrapping<Uint<LIMBS>>;

    fn bitor(self, rhs: Wrapping<Uint<LIMBS>>) -> Wrapping<Uint<LIMBS>> {
        Wrapping(self.0.bitor(&rhs.0))
    }
}

impl<const LIMBS: usize> BitOr<&Wrapping<Uint<LIMBS>>> for &Wrapping<Uint<LIMBS>> {
    type Output = Wrapping<Uint<LIMBS>>;

    fn bitor(self, rhs: &Wrapping<Uint<LIMBS>>) -> Wrapping<Uint<LIMBS>> {
        Wrapping(self.0.bitor(&rhs.0))
    }
}

impl<const LIMBS: usize> BitOrAssign for Wrapping<Uint<LIMBS>> {
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}

impl<const LIMBS: usize> BitOrAssign<&Wrapping<Uint<LIMBS>>> for Wrapping<Uint<LIMBS>> {
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
