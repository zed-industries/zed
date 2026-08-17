//! Limb addition

use crate::{Checked, CheckedAdd, Limb, WideWord, Word, Wrapping, Zero};
use core::ops::{Add, AddAssign};
use subtle::CtOption;

impl Limb {
    /// Computes `self + rhs + carry`, returning the result along with the new carry.
    #[inline(always)]
    pub const fn adc(self, rhs: Limb, carry: Limb) -> (Limb, Limb) {
        let a = self.0 as WideWord;
        let b = rhs.0 as WideWord;
        let carry = carry.0 as WideWord;
        let ret = a + b + carry;
        (Limb(ret as Word), Limb((ret >> Self::BITS) as Word))
    }

    /// Perform saturating addition.
    #[inline]
    pub const fn saturating_add(&self, rhs: Self) -> Self {
        Limb(self.0.saturating_add(rhs.0))
    }

    /// Perform wrapping addition, discarding overflow.
    #[inline(always)]
    pub const fn wrapping_add(&self, rhs: Self) -> Self {
        Limb(self.0.wrapping_add(rhs.0))
    }
}

impl CheckedAdd for Limb {
    type Output = Self;

    #[inline]
    fn checked_add(&self, rhs: Self) -> CtOption<Self> {
        let (result, carry) = self.adc(rhs, Limb::ZERO);
        CtOption::new(result, carry.is_zero())
    }
}

impl Add for Wrapping<Limb> {
    type Output = Self;

    fn add(self, rhs: Self) -> Wrapping<Limb> {
        Wrapping(self.0.wrapping_add(rhs.0))
    }
}

impl Add<&Wrapping<Limb>> for Wrapping<Limb> {
    type Output = Wrapping<Limb>;

    fn add(self, rhs: &Wrapping<Limb>) -> Wrapping<Limb> {
        Wrapping(self.0.wrapping_add(rhs.0))
    }
}

impl Add<Wrapping<Limb>> for &Wrapping<Limb> {
    type Output = Wrapping<Limb>;

    fn add(self, rhs: Wrapping<Limb>) -> Wrapping<Limb> {
        Wrapping(self.0.wrapping_add(rhs.0))
    }
}

impl Add<&Wrapping<Limb>> for &Wrapping<Limb> {
    type Output = Wrapping<Limb>;

    fn add(self, rhs: &Wrapping<Limb>) -> Wrapping<Limb> {
        Wrapping(self.0.wrapping_add(rhs.0))
    }
}

impl AddAssign for Wrapping<Limb> {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl AddAssign<&Wrapping<Limb>> for Wrapping<Limb> {
    fn add_assign(&mut self, other: &Self) {
        *self = *self + other;
    }
}

impl Add for Checked<Limb> {
    type Output = Self;

    fn add(self, rhs: Self) -> Checked<Limb> {
        Checked(
            self.0
                .and_then(|lhs| rhs.0.and_then(|rhs| lhs.checked_add(rhs))),
        )
    }
}

impl Add<&Checked<Limb>> for Checked<Limb> {
    type Output = Checked<Limb>;

    fn add(self, rhs: &Checked<Limb>) -> Checked<Limb> {
        Checked(
            self.0
                .and_then(|lhs| rhs.0.and_then(|rhs| lhs.checked_add(rhs))),
        )
    }
}

impl Add<Checked<Limb>> for &Checked<Limb> {
    type Output = Checked<Limb>;

    fn add(self, rhs: Checked<Limb>) -> Checked<Limb> {
        Checked(
            self.0
                .and_then(|lhs| rhs.0.and_then(|rhs| lhs.checked_add(rhs))),
        )
    }
}

impl Add<&Checked<Limb>> for &Checked<Limb> {
    type Output = Checked<Limb>;

    fn add(self, rhs: &Checked<Limb>) -> Checked<Limb> {
        Checked(
            self.0
                .and_then(|lhs| rhs.0.and_then(|rhs| lhs.checked_add(rhs))),
        )
    }
}

impl AddAssign for Checked<Limb> {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl AddAssign<&Checked<Limb>> for Checked<Limb> {
    fn add_assign(&mut self, other: &Self) {
        *self = *self + other;
    }
}

#[cfg(test)]
mod tests {
    use crate::{CheckedAdd, Limb};

    #[test]
    fn adc_no_carry() {
        let (res, carry) = Limb::ZERO.adc(Limb::ONE, Limb::ZERO);
        assert_eq!(res, Limb::ONE);
        assert_eq!(carry, Limb::ZERO);
    }

    #[test]
    fn adc_with_carry() {
        let (res, carry) = Limb::MAX.adc(Limb::ONE, Limb::ZERO);
        assert_eq!(res, Limb::ZERO);
        assert_eq!(carry, Limb::ONE);
    }

    #[test]
    fn wrapping_add_no_carry() {
        assert_eq!(Limb::ZERO.wrapping_add(Limb::ONE), Limb::ONE);
    }

    #[test]
    fn wrapping_add_with_carry() {
        assert_eq!(Limb::MAX.wrapping_add(Limb::ONE), Limb::ZERO);
    }

    #[test]
    fn checked_add_ok() {
        let result = Limb::ZERO.checked_add(Limb::ONE);
        assert_eq!(result.unwrap(), Limb::ONE);
    }

    #[test]
    fn checked_add_overflow() {
        let result = Limb::MAX.checked_add(Limb::ONE);
        assert!(!bool::from(result.is_some()));
    }
}
