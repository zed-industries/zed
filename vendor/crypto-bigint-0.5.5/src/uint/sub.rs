//! [`Uint`] addition operations.

use super::Uint;
use crate::{Checked, CheckedSub, CtChoice, Limb, Wrapping, Zero};
use core::ops::{Sub, SubAssign};
use subtle::CtOption;

impl<const LIMBS: usize> Uint<LIMBS> {
    /// Computes `a - (b + borrow)`, returning the result along with the new borrow.
    #[inline(always)]
    pub const fn sbb(&self, rhs: &Self, mut borrow: Limb) -> (Self, Limb) {
        let mut limbs = [Limb::ZERO; LIMBS];
        let mut i = 0;

        while i < LIMBS {
            let (w, b) = self.limbs[i].sbb(rhs.limbs[i], borrow);
            limbs[i] = w;
            borrow = b;
            i += 1;
        }

        (Self { limbs }, borrow)
    }

    /// Perform saturating subtraction, returning `ZERO` on underflow.
    pub const fn saturating_sub(&self, rhs: &Self) -> Self {
        let (res, underflow) = self.sbb(rhs, Limb::ZERO);
        Self::ct_select(&res, &Self::ZERO, CtChoice::from_mask(underflow.0))
    }

    /// Perform wrapping subtraction, discarding underflow and wrapping around
    /// the boundary of the type.
    pub const fn wrapping_sub(&self, rhs: &Self) -> Self {
        self.sbb(rhs, Limb::ZERO).0
    }

    /// Perform wrapping subtraction, returning the truthy value as the second element of the tuple
    /// if an underflow has occurred.
    pub(crate) const fn conditional_wrapping_sub(
        &self,
        rhs: &Self,
        choice: CtChoice,
    ) -> (Self, CtChoice) {
        let actual_rhs = Uint::ct_select(&Uint::ZERO, rhs, choice);
        let (res, borrow) = self.sbb(&actual_rhs, Limb::ZERO);
        (res, CtChoice::from_mask(borrow.0))
    }
}

impl<const LIMBS: usize> CheckedSub<&Uint<LIMBS>> for Uint<LIMBS> {
    type Output = Self;

    fn checked_sub(&self, rhs: &Self) -> CtOption<Self> {
        let (result, underflow) = self.sbb(rhs, Limb::ZERO);
        CtOption::new(result, underflow.is_zero())
    }
}

impl<const LIMBS: usize> Sub for Wrapping<Uint<LIMBS>> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Wrapping<Uint<LIMBS>> {
        Wrapping(self.0.wrapping_sub(&rhs.0))
    }
}

impl<const LIMBS: usize> Sub<&Wrapping<Uint<LIMBS>>> for Wrapping<Uint<LIMBS>> {
    type Output = Wrapping<Uint<LIMBS>>;

    fn sub(self, rhs: &Wrapping<Uint<LIMBS>>) -> Wrapping<Uint<LIMBS>> {
        Wrapping(self.0.wrapping_sub(&rhs.0))
    }
}

impl<const LIMBS: usize> Sub<Wrapping<Uint<LIMBS>>> for &Wrapping<Uint<LIMBS>> {
    type Output = Wrapping<Uint<LIMBS>>;

    fn sub(self, rhs: Wrapping<Uint<LIMBS>>) -> Wrapping<Uint<LIMBS>> {
        Wrapping(self.0.wrapping_sub(&rhs.0))
    }
}

impl<const LIMBS: usize> Sub<&Wrapping<Uint<LIMBS>>> for &Wrapping<Uint<LIMBS>> {
    type Output = Wrapping<Uint<LIMBS>>;

    fn sub(self, rhs: &Wrapping<Uint<LIMBS>>) -> Wrapping<Uint<LIMBS>> {
        Wrapping(self.0.wrapping_sub(&rhs.0))
    }
}

impl<const LIMBS: usize> SubAssign for Wrapping<Uint<LIMBS>> {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl<const LIMBS: usize> SubAssign<&Wrapping<Uint<LIMBS>>> for Wrapping<Uint<LIMBS>> {
    fn sub_assign(&mut self, other: &Self) {
        *self = *self - other;
    }
}

impl<const LIMBS: usize> Sub for Checked<Uint<LIMBS>> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Checked<Uint<LIMBS>> {
        Checked(
            self.0
                .and_then(|lhs| rhs.0.and_then(|rhs| lhs.checked_sub(&rhs))),
        )
    }
}

impl<const LIMBS: usize> Sub<&Checked<Uint<LIMBS>>> for Checked<Uint<LIMBS>> {
    type Output = Checked<Uint<LIMBS>>;

    fn sub(self, rhs: &Checked<Uint<LIMBS>>) -> Checked<Uint<LIMBS>> {
        Checked(
            self.0
                .and_then(|lhs| rhs.0.and_then(|rhs| lhs.checked_sub(&rhs))),
        )
    }
}

impl<const LIMBS: usize> Sub<Checked<Uint<LIMBS>>> for &Checked<Uint<LIMBS>> {
    type Output = Checked<Uint<LIMBS>>;

    fn sub(self, rhs: Checked<Uint<LIMBS>>) -> Checked<Uint<LIMBS>> {
        Checked(
            self.0
                .and_then(|lhs| rhs.0.and_then(|rhs| lhs.checked_sub(&rhs))),
        )
    }
}

impl<const LIMBS: usize> Sub<&Checked<Uint<LIMBS>>> for &Checked<Uint<LIMBS>> {
    type Output = Checked<Uint<LIMBS>>;

    fn sub(self, rhs: &Checked<Uint<LIMBS>>) -> Checked<Uint<LIMBS>> {
        Checked(
            self.0
                .and_then(|lhs| rhs.0.and_then(|rhs| lhs.checked_sub(&rhs))),
        )
    }
}

impl<const LIMBS: usize> SubAssign for Checked<Uint<LIMBS>> {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl<const LIMBS: usize> SubAssign<&Checked<Uint<LIMBS>>> for Checked<Uint<LIMBS>> {
    fn sub_assign(&mut self, other: &Self) {
        *self = *self - other;
    }
}

#[cfg(test)]
mod tests {
    use crate::{CheckedSub, Limb, U128};

    #[test]
    fn sbb_no_borrow() {
        let (res, borrow) = U128::ONE.sbb(&U128::ONE, Limb::ZERO);
        assert_eq!(res, U128::ZERO);
        assert_eq!(borrow, Limb::ZERO);
    }

    #[test]
    fn sbb_with_borrow() {
        let (res, borrow) = U128::ZERO.sbb(&U128::ONE, Limb::ZERO);

        assert_eq!(res, U128::MAX);
        assert_eq!(borrow, Limb::MAX);
    }

    #[test]
    fn saturating_sub_no_borrow() {
        assert_eq!(
            U128::from(5u64).saturating_sub(&U128::ONE),
            U128::from(4u64)
        );
    }

    #[test]
    fn saturating_sub_with_borrow() {
        assert_eq!(
            U128::from(4u64).saturating_sub(&U128::from(5u64)),
            U128::ZERO
        );
    }

    #[test]
    fn wrapping_sub_no_borrow() {
        assert_eq!(U128::ONE.wrapping_sub(&U128::ONE), U128::ZERO);
    }

    #[test]
    fn wrapping_sub_with_borrow() {
        assert_eq!(U128::ZERO.wrapping_sub(&U128::ONE), U128::MAX);
    }

    #[test]
    fn checked_sub_ok() {
        let result = U128::ONE.checked_sub(&U128::ONE);
        assert_eq!(result.unwrap(), U128::ZERO);
    }

    #[test]
    fn checked_sub_overflow() {
        let result = U128::ZERO.checked_sub(&U128::ONE);
        assert!(!bool::from(result.is_some()));
    }
}
