//! [`UInt`] addition operations.

use crate::{Checked, CheckedMul, Concat, Limb, UInt, Wrapping, Zero};
use core::ops::{Mul, MulAssign};
use subtle::CtOption;

impl<const LIMBS: usize> UInt<LIMBS> {
    /// Compute "wide" multiplication, with a product twice the size of the input.
    ///
    /// Returns a tuple containing the `(lo, hi)` components of the product.
    ///
    /// # Ordering note
    ///
    /// Releases of `crypto-bigint` prior to v0.3 used `(hi, lo)` ordering
    /// instead. This has been changed for better consistency with the rest of
    /// the APIs in this crate.
    ///
    /// For more info see: <https://github.com/RustCrypto/crypto-bigint/issues/4>
    // TODO(tarcieri): use `concat` to construct a wide output
    pub const fn mul_wide(&self, rhs: &Self) -> (Self, Self) {
        let mut i = 0;
        let mut lo = Self::ZERO;
        let mut hi = Self::ZERO;

        // Schoolbook multiplication.
        // TODO(tarcieri): use Karatsuba for better performance?
        while i < LIMBS {
            let mut j = 0;
            let mut carry = Limb::ZERO;

            while j < LIMBS {
                let k = i + j;

                if k >= LIMBS {
                    let (n, c) = hi.limbs[k - LIMBS].mac(self.limbs[i], rhs.limbs[j], carry);
                    hi.limbs[k - LIMBS] = n;
                    carry = c;
                } else {
                    let (n, c) = lo.limbs[k].mac(self.limbs[i], rhs.limbs[j], carry);
                    lo.limbs[k] = n;
                    carry = c;
                }

                j += 1;
            }

            hi.limbs[i + j - LIMBS] = carry;
            i += 1;
        }

        (lo, hi)
    }

    /// Perform saturating multiplication, returning `MAX` on overflow.
    pub const fn saturating_mul(&self, rhs: &Self) -> Self {
        let (res, overflow) = self.mul_wide(rhs);

        let mut i = 0;
        let mut accumulator = 0;

        while i < LIMBS {
            accumulator |= overflow.limbs[i].0;
            i += 1;
        }

        if accumulator == 0 {
            res
        } else {
            Self::MAX
        }
    }

    /// Perform wrapping multiplication, discarding overflow.
    pub const fn wrapping_mul(&self, rhs: &Self) -> Self {
        self.mul_wide(rhs).0
    }

    /// Square self, returning a "wide" result.
    pub fn square(&self) -> <Self as Concat>::Output
    where
        Self: Concat,
    {
        let (lo, hi) = self.mul_wide(self);
        hi.concat(&lo)
    }
}

impl<const LIMBS: usize> CheckedMul<&UInt<LIMBS>> for UInt<LIMBS> {
    type Output = Self;

    fn checked_mul(&self, rhs: &Self) -> CtOption<Self> {
        let (lo, hi) = self.mul_wide(rhs);
        CtOption::new(lo, hi.is_zero())
    }
}

impl<const LIMBS: usize> Mul for Wrapping<UInt<LIMBS>> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Wrapping<UInt<LIMBS>> {
        Wrapping(self.0.wrapping_mul(&rhs.0))
    }
}

impl<const LIMBS: usize> Mul<&Wrapping<UInt<LIMBS>>> for Wrapping<UInt<LIMBS>> {
    type Output = Wrapping<UInt<LIMBS>>;

    fn mul(self, rhs: &Wrapping<UInt<LIMBS>>) -> Wrapping<UInt<LIMBS>> {
        Wrapping(self.0.wrapping_mul(&rhs.0))
    }
}

impl<const LIMBS: usize> Mul<Wrapping<UInt<LIMBS>>> for &Wrapping<UInt<LIMBS>> {
    type Output = Wrapping<UInt<LIMBS>>;

    fn mul(self, rhs: Wrapping<UInt<LIMBS>>) -> Wrapping<UInt<LIMBS>> {
        Wrapping(self.0.wrapping_mul(&rhs.0))
    }
}

impl<const LIMBS: usize> Mul<&Wrapping<UInt<LIMBS>>> for &Wrapping<UInt<LIMBS>> {
    type Output = Wrapping<UInt<LIMBS>>;

    fn mul(self, rhs: &Wrapping<UInt<LIMBS>>) -> Wrapping<UInt<LIMBS>> {
        Wrapping(self.0.wrapping_mul(&rhs.0))
    }
}

impl<const LIMBS: usize> MulAssign for Wrapping<UInt<LIMBS>> {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl<const LIMBS: usize> MulAssign<&Wrapping<UInt<LIMBS>>> for Wrapping<UInt<LIMBS>> {
    fn mul_assign(&mut self, other: &Self) {
        *self = *self * other;
    }
}

impl<const LIMBS: usize> Mul for Checked<UInt<LIMBS>> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Checked<UInt<LIMBS>> {
        Checked(self.0.and_then(|a| rhs.0.and_then(|b| a.checked_mul(&b))))
    }
}

impl<const LIMBS: usize> Mul<&Checked<UInt<LIMBS>>> for Checked<UInt<LIMBS>> {
    type Output = Checked<UInt<LIMBS>>;

    fn mul(self, rhs: &Checked<UInt<LIMBS>>) -> Checked<UInt<LIMBS>> {
        Checked(self.0.and_then(|a| rhs.0.and_then(|b| a.checked_mul(&b))))
    }
}

impl<const LIMBS: usize> Mul<Checked<UInt<LIMBS>>> for &Checked<UInt<LIMBS>> {
    type Output = Checked<UInt<LIMBS>>;

    fn mul(self, rhs: Checked<UInt<LIMBS>>) -> Checked<UInt<LIMBS>> {
        Checked(self.0.and_then(|a| rhs.0.and_then(|b| a.checked_mul(&b))))
    }
}

impl<const LIMBS: usize> Mul<&Checked<UInt<LIMBS>>> for &Checked<UInt<LIMBS>> {
    type Output = Checked<UInt<LIMBS>>;

    fn mul(self, rhs: &Checked<UInt<LIMBS>>) -> Checked<UInt<LIMBS>> {
        Checked(self.0.and_then(|a| rhs.0.and_then(|b| a.checked_mul(&b))))
    }
}

impl<const LIMBS: usize> MulAssign for Checked<UInt<LIMBS>> {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl<const LIMBS: usize> MulAssign<&Checked<UInt<LIMBS>>> for Checked<UInt<LIMBS>> {
    fn mul_assign(&mut self, other: &Self) {
        *self = *self * other;
    }
}

#[cfg(test)]
mod tests {
    use crate::{CheckedMul, Zero, U64};

    #[test]
    fn mul_wide_zero_and_one() {
        assert_eq!(U64::ZERO.mul_wide(&U64::ZERO), (U64::ZERO, U64::ZERO));
        assert_eq!(U64::ZERO.mul_wide(&U64::ONE), (U64::ZERO, U64::ZERO));
        assert_eq!(U64::ONE.mul_wide(&U64::ZERO), (U64::ZERO, U64::ZERO));
        assert_eq!(U64::ONE.mul_wide(&U64::ONE), (U64::ONE, U64::ZERO));
    }

    #[test]
    fn mul_wide_lo_only() {
        let primes: &[u32] = &[3, 5, 17, 256, 65537];

        for &a_int in primes {
            for &b_int in primes {
                let (lo, hi) = U64::from_u32(a_int).mul_wide(&U64::from_u32(b_int));
                let expected = U64::from_u64(a_int as u64 * b_int as u64);
                assert_eq!(lo, expected);
                assert!(bool::from(hi.is_zero()));
            }
        }
    }

    #[test]
    fn checked_mul_ok() {
        let n = U64::from_u32(0xffff_ffff);
        assert_eq!(
            n.checked_mul(&n).unwrap(),
            U64::from_u64(0xffff_fffe_0000_0001)
        );
    }

    #[test]
    fn checked_mul_overflow() {
        let n = U64::from_u64(0xffff_ffff_ffff_ffff);
        assert!(bool::from(n.checked_mul(&n).is_none()));
    }

    #[test]
    fn saturating_mul_no_overflow() {
        let n = U64::from_u8(8);
        assert_eq!(n.saturating_mul(&n), U64::from_u8(64));
    }

    #[test]
    fn saturating_mul_overflow() {
        let a = U64::from(0xffff_ffff_ffff_ffffu64);
        let b = U64::from(2u8);
        assert_eq!(a.saturating_mul(&b), U64::MAX);
    }

    #[test]
    fn square() {
        let n = U64::from_u64(0xffff_ffff_ffff_ffff);
        let (hi, lo) = n.square().split();
        assert_eq!(lo, U64::from_u64(1));
        assert_eq!(hi, U64::from_u64(0xffff_ffff_ffff_fffe));
    }
}
