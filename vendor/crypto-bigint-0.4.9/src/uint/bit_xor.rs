//! [`UInt`] bitwise xor operations.

use super::UInt;
use crate::{Limb, Wrapping};
use core::ops::{BitXor, BitXorAssign};
use subtle::{Choice, CtOption};

impl<const LIMBS: usize> UInt<LIMBS> {
    /// Computes bitwise `a ^ b`.
    #[inline(always)]
    pub const fn bitxor(&self, rhs: &Self) -> Self {
        let mut limbs = [Limb::ZERO; LIMBS];
        let mut i = 0;

        while i < LIMBS {
            limbs[i] = self.limbs[i].bitxor(rhs.limbs[i]);
            i += 1;
        }

        Self { limbs }
    }

    /// Perform wrapping bitwise `XOR``.
    ///
    /// There's no way wrapping could ever happen.
    /// This function exists so that all operations are accounted for in the wrapping operations
    pub const fn wrapping_xor(&self, rhs: &Self) -> Self {
        self.bitxor(rhs)
    }

    /// Perform checked bitwise `XOR`, returning a [`CtOption`] which `is_some` always
    pub fn checked_xor(&self, rhs: &Self) -> CtOption<Self> {
        let result = self.bitxor(rhs);
        CtOption::new(result, Choice::from(1))
    }
}

impl<const LIMBS: usize> BitXor for UInt<LIMBS> {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> UInt<LIMBS> {
        self.bitxor(&rhs)
    }
}

impl<const LIMBS: usize> BitXor<&UInt<LIMBS>> for UInt<LIMBS> {
    type Output = UInt<LIMBS>;

    fn bitxor(self, rhs: &UInt<LIMBS>) -> UInt<LIMBS> {
        (&self).bitxor(rhs)
    }
}

impl<const LIMBS: usize> BitXor<UInt<LIMBS>> for &UInt<LIMBS> {
    type Output = UInt<LIMBS>;

    fn bitxor(self, rhs: UInt<LIMBS>) -> UInt<LIMBS> {
        self.bitxor(&rhs)
    }
}

impl<const LIMBS: usize> BitXor<&UInt<LIMBS>> for &UInt<LIMBS> {
    type Output = UInt<LIMBS>;

    fn bitxor(self, rhs: &UInt<LIMBS>) -> UInt<LIMBS> {
        self.bitxor(rhs)
    }
}

impl<const LIMBS: usize> BitXorAssign for UInt<LIMBS> {
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}

impl<const LIMBS: usize> BitXorAssign<&UInt<LIMBS>> for UInt<LIMBS> {
    fn bitxor_assign(&mut self, other: &Self) {
        *self = *self ^ other;
    }
}

impl<const LIMBS: usize> BitXor for Wrapping<UInt<LIMBS>> {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Wrapping<UInt<LIMBS>> {
        Wrapping(self.0.bitxor(&rhs.0))
    }
}

impl<const LIMBS: usize> BitXor<&Wrapping<UInt<LIMBS>>> for Wrapping<UInt<LIMBS>> {
    type Output = Wrapping<UInt<LIMBS>>;

    fn bitxor(self, rhs: &Wrapping<UInt<LIMBS>>) -> Wrapping<UInt<LIMBS>> {
        Wrapping(self.0.bitxor(&rhs.0))
    }
}

impl<const LIMBS: usize> BitXor<Wrapping<UInt<LIMBS>>> for &Wrapping<UInt<LIMBS>> {
    type Output = Wrapping<UInt<LIMBS>>;

    fn bitxor(self, rhs: Wrapping<UInt<LIMBS>>) -> Wrapping<UInt<LIMBS>> {
        Wrapping(self.0.bitxor(&rhs.0))
    }
}

impl<const LIMBS: usize> BitXor<&Wrapping<UInt<LIMBS>>> for &Wrapping<UInt<LIMBS>> {
    type Output = Wrapping<UInt<LIMBS>>;

    fn bitxor(self, rhs: &Wrapping<UInt<LIMBS>>) -> Wrapping<UInt<LIMBS>> {
        Wrapping(self.0.bitxor(&rhs.0))
    }
}

impl<const LIMBS: usize> BitXorAssign for Wrapping<UInt<LIMBS>> {
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}

impl<const LIMBS: usize> BitXorAssign<&Wrapping<UInt<LIMBS>>> for Wrapping<UInt<LIMBS>> {
    fn bitxor_assign(&mut self, other: &Self) {
        *self = *self ^ other;
    }
}

#[cfg(test)]
mod tests {
    use crate::U128;

    #[test]
    fn checked_xor_ok() {
        let result = U128::ZERO.checked_xor(&U128::ONE);
        assert_eq!(result.unwrap(), U128::ONE);
    }

    #[test]
    fn overlapping_xor_ok() {
        let result = U128::ZERO.wrapping_xor(&U128::ONE);
        assert_eq!(result, U128::ONE);
    }
}
