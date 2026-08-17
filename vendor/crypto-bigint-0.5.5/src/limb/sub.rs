//! Limb subtraction

use crate::{Checked, CheckedSub, Limb, WideWord, Word, Wrapping, Zero};
use core::ops::{Sub, SubAssign};
use subtle::CtOption;

impl Limb {
    /// Computes `self - (rhs + borrow)`, returning the result along with the new borrow.
    #[inline(always)]
    pub const fn sbb(self, rhs: Limb, borrow: Limb) -> (Limb, Limb) {
        let a = self.0 as WideWord;
        let b = rhs.0 as WideWord;
        let borrow = (borrow.0 >> (Self::BITS - 1)) as WideWord;
        let ret = a.wrapping_sub(b + borrow);
        (Limb(ret as Word), Limb((ret >> Self::BITS) as Word))
    }

    /// Perform saturating subtraction.
    #[inline]
    pub const fn saturating_sub(&self, rhs: Self) -> Self {
        Limb(self.0.saturating_sub(rhs.0))
    }

    /// Perform wrapping subtraction, discarding underflow and wrapping around
    /// the boundary of the type.
    #[inline(always)]
    pub const fn wrapping_sub(&self, rhs: Self) -> Self {
        Limb(self.0.wrapping_sub(rhs.0))
    }
}

impl CheckedSub for Limb {
    type Output = Self;

    #[inline]
    fn checked_sub(&self, rhs: Self) -> CtOption<Self> {
        let (result, underflow) = self.sbb(rhs, Limb::ZERO);
        CtOption::new(result, underflow.is_zero())
    }
}

impl Sub for Wrapping<Limb> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Wrapping<Limb> {
        Wrapping(self.0.wrapping_sub(rhs.0))
    }
}

impl Sub<&Wrapping<Limb>> for Wrapping<Limb> {
    type Output = Wrapping<Limb>;

    fn sub(self, rhs: &Wrapping<Limb>) -> Wrapping<Limb> {
        Wrapping(self.0.wrapping_sub(rhs.0))
    }
}

impl Sub<Wrapping<Limb>> for &Wrapping<Limb> {
    type Output = Wrapping<Limb>;

    fn sub(self, rhs: Wrapping<Limb>) -> Wrapping<Limb> {
        Wrapping(self.0.wrapping_sub(rhs.0))
    }
}

impl Sub<&Wrapping<Limb>> for &Wrapping<Limb> {
    type Output = Wrapping<Limb>;

    fn sub(self, rhs: &Wrapping<Limb>) -> Wrapping<Limb> {
        Wrapping(self.0.wrapping_sub(rhs.0))
    }
}

impl SubAssign for Wrapping<Limb> {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl SubAssign<&Wrapping<Limb>> for Wrapping<Limb> {
    fn sub_assign(&mut self, other: &Self) {
        *self = *self - other;
    }
}

impl Sub for Checked<Limb> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Checked<Limb> {
        Checked(
            self.0
                .and_then(|lhs| rhs.0.and_then(|rhs| lhs.checked_sub(rhs))),
        )
    }
}

impl Sub<&Checked<Limb>> for Checked<Limb> {
    type Output = Checked<Limb>;

    fn sub(self, rhs: &Checked<Limb>) -> Checked<Limb> {
        Checked(
            self.0
                .and_then(|lhs| rhs.0.and_then(|rhs| lhs.checked_sub(rhs))),
        )
    }
}

impl Sub<Checked<Limb>> for &Checked<Limb> {
    type Output = Checked<Limb>;

    fn sub(self, rhs: Checked<Limb>) -> Checked<Limb> {
        Checked(
            self.0
                .and_then(|lhs| rhs.0.and_then(|rhs| lhs.checked_sub(rhs))),
        )
    }
}

impl Sub<&Checked<Limb>> for &Checked<Limb> {
    type Output = Checked<Limb>;

    fn sub(self, rhs: &Checked<Limb>) -> Checked<Limb> {
        Checked(
            self.0
                .and_then(|lhs| rhs.0.and_then(|rhs| lhs.checked_sub(rhs))),
        )
    }
}

impl SubAssign for Checked<Limb> {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl SubAssign<&Checked<Limb>> for Checked<Limb> {
    fn sub_assign(&mut self, other: &Self) {
        *self = *self - other;
    }
}

#[cfg(test)]
mod tests {
    use crate::{CheckedSub, Limb};

    #[test]
    fn sbb_no_borrow() {
        let (res, borrow) = Limb::ONE.sbb(Limb::ONE, Limb::ZERO);
        assert_eq!(res, Limb::ZERO);
        assert_eq!(borrow, Limb::ZERO);
    }

    #[test]
    fn sbb_with_borrow() {
        let (res, borrow) = Limb::ZERO.sbb(Limb::ONE, Limb::ZERO);

        assert_eq!(res, Limb::MAX);
        assert_eq!(borrow, Limb::MAX);
    }

    #[test]
    fn wrapping_sub_no_borrow() {
        assert_eq!(Limb::ONE.wrapping_sub(Limb::ONE), Limb::ZERO);
    }

    #[test]
    fn wrapping_sub_with_borrow() {
        assert_eq!(Limb::ZERO.wrapping_sub(Limb::ONE), Limb::MAX);
    }

    #[test]
    fn checked_sub_ok() {
        let result = Limb::ONE.checked_sub(Limb::ONE);
        assert_eq!(result.unwrap(), Limb::ZERO);
    }

    #[test]
    fn checked_sub_overflow() {
        let result = Limb::ZERO.checked_sub(Limb::ONE);
        assert!(!bool::from(result.is_some()));
    }
}
