//! [`UInt`] bitwise right shift operations.

use super::UInt;
use crate::Limb;
use core::ops::{Shr, ShrAssign};

impl<const LIMBS: usize> UInt<LIMBS> {
    /// Computes `self >> n`.
    ///
    /// NOTE: this operation is variable time with respect to `n` *ONLY*.
    ///
    /// When used with a fixed `n`, this function is constant-time with respect
    /// to `self`.
    #[inline(always)]
    pub const fn shr_vartime(&self, shift: usize) -> Self {
        let full_shifts = shift / Limb::BIT_SIZE;
        let small_shift = shift & (Limb::BIT_SIZE - 1);
        let mut limbs = [Limb::ZERO; LIMBS];

        if shift > Limb::BIT_SIZE * LIMBS {
            return Self { limbs };
        }

        let n = LIMBS - full_shifts;
        let mut i = 0;

        if small_shift == 0 {
            while i < n {
                limbs[i] = Limb(self.limbs[i + full_shifts].0);
                i += 1;
            }
        } else {
            while i < n {
                let mut lo = self.limbs[i + full_shifts].0 >> small_shift;

                if i < (LIMBS - 1) - full_shifts {
                    lo |= self.limbs[i + full_shifts + 1].0 << (Limb::BIT_SIZE - small_shift);
                }

                limbs[i] = Limb(lo);
                i += 1;
            }
        }

        Self { limbs }
    }
}

impl<const LIMBS: usize> Shr<usize> for UInt<LIMBS> {
    type Output = UInt<LIMBS>;

    /// NOTE: this operation is variable time with respect to `rhs` *ONLY*.
    ///
    /// When used with a fixed `rhs`, this function is constant-time with respect
    /// to `self`.
    fn shr(self, rhs: usize) -> UInt<LIMBS> {
        self.shr_vartime(rhs)
    }
}

impl<const LIMBS: usize> Shr<usize> for &UInt<LIMBS> {
    type Output = UInt<LIMBS>;

    /// NOTE: this operation is variable time with respect to `rhs` *ONLY*.
    ///
    /// When used with a fixed `rhs`, this function is constant-time with respect
    /// to `self`.
    fn shr(self, rhs: usize) -> UInt<LIMBS> {
        self.shr_vartime(rhs)
    }
}

impl<const LIMBS: usize> ShrAssign<usize> for UInt<LIMBS> {
    fn shr_assign(&mut self, rhs: usize) {
        *self = self.shr_vartime(rhs);
    }
}

#[cfg(test)]
mod tests {
    use crate::U256;

    const N: U256 =
        U256::from_be_hex("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141");

    const N_2: U256 =
        U256::from_be_hex("7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF5D576E7357A4501DDFE92F46681B20A0");

    #[test]
    fn shr1() {
        assert_eq!(N >> 1, N_2);
    }
}
