//! Limb multiplication

use crate::{Checked, CheckedMul, Limb, WideWord, Word, Wrapping, Zero};
use core::ops::{Mul, MulAssign};
use subtle::CtOption;

impl Limb {
    /// Computes `self + (b * c) + carry`, returning the result along with the new carry.
    #[inline(always)]
    pub const fn mac(self, b: Limb, c: Limb, carry: Limb) -> (Limb, Limb) {
        let a = self.0 as WideWord;
        let b = b.0 as WideWord;
        let c = c.0 as WideWord;
        let carry = carry.0 as WideWord;
        let ret = a + (b * c) + carry;
        (Limb(ret as Word), Limb((ret >> Self::BITS) as Word))
    }

    /// Perform saturating multiplication.
    #[inline]
    pub const fn saturating_mul(&self, rhs: Self) -> Self {
        Limb(self.0.saturating_mul(rhs.0))
    }

    /// Perform wrapping multiplication, discarding overflow.
    #[inline(always)]
    pub const fn wrapping_mul(&self, rhs: Self) -> Self {
        Limb(self.0.wrapping_mul(rhs.0))
    }

    /// Compute "wide" multiplication, with a product twice the size of the input.
    pub(crate) const fn mul_wide(&self, rhs: Self) -> WideWord {
        (self.0 as WideWord) * (rhs.0 as WideWord)
    }
}

impl CheckedMul for Limb {
    type Output = Self;

    #[inline]
    fn checked_mul(&self, rhs: Self) -> CtOption<Self> {
        let result = self.mul_wide(rhs);
        let overflow = Limb((result >> Self::BITS) as Word);
        CtOption::new(Limb(result as Word), overflow.is_zero())
    }
}

impl Mul for Wrapping<Limb> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Wrapping<Limb> {
        Wrapping(self.0.wrapping_mul(rhs.0))
    }
}

impl Mul<&Wrapping<Limb>> for Wrapping<Limb> {
    type Output = Wrapping<Limb>;

    fn mul(self, rhs: &Wrapping<Limb>) -> Wrapping<Limb> {
        Wrapping(self.0.wrapping_mul(rhs.0))
    }
}

impl Mul<Wrapping<Limb>> for &Wrapping<Limb> {
    type Output = Wrapping<Limb>;

    fn mul(self, rhs: Wrapping<Limb>) -> Wrapping<Limb> {
        Wrapping(self.0.wrapping_mul(rhs.0))
    }
}

impl Mul<&Wrapping<Limb>> for &Wrapping<Limb> {
    type Output = Wrapping<Limb>;

    fn mul(self, rhs: &Wrapping<Limb>) -> Wrapping<Limb> {
        Wrapping(self.0.wrapping_mul(rhs.0))
    }
}

impl MulAssign for Wrapping<Limb> {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl MulAssign<&Wrapping<Limb>> for Wrapping<Limb> {
    fn mul_assign(&mut self, other: &Self) {
        *self = *self * other;
    }
}

impl Mul for Checked<Limb> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Checked<Limb> {
        Checked(
            self.0
                .and_then(|lhs| rhs.0.and_then(|rhs| lhs.checked_mul(rhs))),
        )
    }
}

impl Mul<&Checked<Limb>> for Checked<Limb> {
    type Output = Checked<Limb>;

    fn mul(self, rhs: &Checked<Limb>) -> Checked<Limb> {
        Checked(
            self.0
                .and_then(|lhs| rhs.0.and_then(|rhs| lhs.checked_mul(rhs))),
        )
    }
}

impl Mul<Checked<Limb>> for &Checked<Limb> {
    type Output = Checked<Limb>;

    fn mul(self, rhs: Checked<Limb>) -> Checked<Limb> {
        Checked(
            self.0
                .and_then(|lhs| rhs.0.and_then(|rhs| lhs.checked_mul(rhs))),
        )
    }
}

impl Mul<&Checked<Limb>> for &Checked<Limb> {
    type Output = Checked<Limb>;

    fn mul(self, rhs: &Checked<Limb>) -> Checked<Limb> {
        Checked(
            self.0
                .and_then(|lhs| rhs.0.and_then(|rhs| lhs.checked_mul(rhs))),
        )
    }
}

impl MulAssign for Checked<Limb> {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl MulAssign<&Checked<Limb>> for Checked<Limb> {
    fn mul_assign(&mut self, other: &Self) {
        *self = *self * other;
    }
}

#[cfg(test)]
mod tests {
    use super::{CheckedMul, Limb, WideWord};

    #[test]
    fn mul_wide_zero_and_one() {
        assert_eq!(Limb::ZERO.mul_wide(Limb::ZERO), 0);
        assert_eq!(Limb::ZERO.mul_wide(Limb::ONE), 0);
        assert_eq!(Limb::ONE.mul_wide(Limb::ZERO), 0);
        assert_eq!(Limb::ONE.mul_wide(Limb::ONE), 1);
    }

    #[test]
    fn mul_wide() {
        let primes: &[u32] = &[3, 5, 17, 257, 65537];

        for &a_int in primes {
            for &b_int in primes {
                let actual = Limb::from_u32(a_int).mul_wide(Limb::from_u32(b_int));
                let expected = a_int as WideWord * b_int as WideWord;
                assert_eq!(actual, expected);
            }
        }
    }

    #[test]
    #[cfg(target_pointer_width = "32")]
    fn checked_mul_ok() {
        let n = Limb::from_u16(0xffff);
        assert_eq!(n.checked_mul(n).unwrap(), Limb::from_u32(0xfffe_0001));
    }

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn checked_mul_ok() {
        let n = Limb::from_u32(0xffff_ffff);
        assert_eq!(
            n.checked_mul(n).unwrap(),
            Limb::from_u64(0xffff_fffe_0000_0001)
        );
    }

    #[test]
    fn checked_mul_overflow() {
        let n = Limb::MAX;
        assert!(bool::from(n.checked_mul(n).is_none()));
    }
}
