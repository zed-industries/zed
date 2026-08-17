//! [`Uint`] bitwise and operations.

use super::Uint;
use crate::{Limb, Wrapping};
use core::ops::{BitAnd, BitAndAssign};
use subtle::{Choice, CtOption};

impl<const LIMBS: usize> Uint<LIMBS> {
    /// Computes bitwise `a & b`.
    #[inline(always)]
    pub const fn bitand(&self, rhs: &Self) -> Self {
        let mut limbs = [Limb::ZERO; LIMBS];
        let mut i = 0;

        while i < LIMBS {
            limbs[i] = self.limbs[i].bitand(rhs.limbs[i]);
            i += 1;
        }

        Self { limbs }
    }

    /// Perform wrapping bitwise `AND`.
    ///
    /// There's no way wrapping could ever happen.
    /// This function exists so that all operations are accounted for in the wrapping operations
    pub const fn wrapping_and(&self, rhs: &Self) -> Self {
        self.bitand(rhs)
    }

    /// Perform checked bitwise `AND`, returning a [`CtOption`] which `is_some` always
    pub fn checked_and(&self, rhs: &Self) -> CtOption<Self> {
        let result = self.bitand(rhs);
        CtOption::new(result, Choice::from(1))
    }
}

impl<const LIMBS: usize> BitAnd for Uint<LIMBS> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Uint<LIMBS> {
        self.bitand(&rhs)
    }
}

impl<const LIMBS: usize> BitAnd<&Uint<LIMBS>> for Uint<LIMBS> {
    type Output = Uint<LIMBS>;

    #[allow(clippy::needless_borrow)]
    fn bitand(self, rhs: &Uint<LIMBS>) -> Uint<LIMBS> {
        (&self).bitand(rhs)
    }
}

impl<const LIMBS: usize> BitAnd<Uint<LIMBS>> for &Uint<LIMBS> {
    type Output = Uint<LIMBS>;

    fn bitand(self, rhs: Uint<LIMBS>) -> Uint<LIMBS> {
        self.bitand(&rhs)
    }
}

impl<const LIMBS: usize> BitAnd<&Uint<LIMBS>> for &Uint<LIMBS> {
    type Output = Uint<LIMBS>;

    fn bitand(self, rhs: &Uint<LIMBS>) -> Uint<LIMBS> {
        self.bitand(rhs)
    }
}

impl<const LIMBS: usize> BitAndAssign for Uint<LIMBS> {
    #[allow(clippy::assign_op_pattern)]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}

impl<const LIMBS: usize> BitAndAssign<&Uint<LIMBS>> for Uint<LIMBS> {
    #[allow(clippy::assign_op_pattern)]
    fn bitand_assign(&mut self, other: &Self) {
        *self = *self & other;
    }
}

impl<const LIMBS: usize> BitAnd for Wrapping<Uint<LIMBS>> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Wrapping<Uint<LIMBS>> {
        Wrapping(self.0.bitand(&rhs.0))
    }
}

impl<const LIMBS: usize> BitAnd<&Wrapping<Uint<LIMBS>>> for Wrapping<Uint<LIMBS>> {
    type Output = Wrapping<Uint<LIMBS>>;

    fn bitand(self, rhs: &Wrapping<Uint<LIMBS>>) -> Wrapping<Uint<LIMBS>> {
        Wrapping(self.0.bitand(&rhs.0))
    }
}

impl<const LIMBS: usize> BitAnd<Wrapping<Uint<LIMBS>>> for &Wrapping<Uint<LIMBS>> {
    type Output = Wrapping<Uint<LIMBS>>;

    fn bitand(self, rhs: Wrapping<Uint<LIMBS>>) -> Wrapping<Uint<LIMBS>> {
        Wrapping(self.0.bitand(&rhs.0))
    }
}

impl<const LIMBS: usize> BitAnd<&Wrapping<Uint<LIMBS>>> for &Wrapping<Uint<LIMBS>> {
    type Output = Wrapping<Uint<LIMBS>>;

    fn bitand(self, rhs: &Wrapping<Uint<LIMBS>>) -> Wrapping<Uint<LIMBS>> {
        Wrapping(self.0.bitand(&rhs.0))
    }
}

impl<const LIMBS: usize> BitAndAssign for Wrapping<Uint<LIMBS>> {
    #[allow(clippy::assign_op_pattern)]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}

impl<const LIMBS: usize> BitAndAssign<&Wrapping<Uint<LIMBS>>> for Wrapping<Uint<LIMBS>> {
    #[allow(clippy::assign_op_pattern)]
    fn bitand_assign(&mut self, other: &Self) {
        *self = *self & other;
    }
}

#[cfg(test)]
mod tests {
    use crate::U128;

    #[test]
    fn checked_and_ok() {
        let result = U128::ZERO.checked_and(&U128::ONE);
        assert_eq!(result.unwrap(), U128::ZERO);
    }

    #[test]
    fn overlapping_and_ok() {
        let result = U128::MAX.wrapping_and(&U128::ONE);
        assert_eq!(result, U128::ONE);
    }
}
