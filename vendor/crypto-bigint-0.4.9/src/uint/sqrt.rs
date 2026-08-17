//! [`UInt`] square root operations.

use super::UInt;
use crate::{Limb, Word};
use subtle::{ConstantTimeEq, CtOption};

impl<const LIMBS: usize> UInt<LIMBS> {
    /// Computes √(`self`)
    /// Uses Brent & Zimmermann, Modern Computer Arithmetic, v0.5.9, Algorithm 1.13
    ///
    /// Callers can check if `self` is a square by squaring the result
    pub const fn sqrt(&self) -> Self {
        let max_bits = (self.bits_vartime() + 1) >> 1;
        let cap = Self::ONE.shl_vartime(max_bits);
        let mut guess = cap; // ≥ √(`self`)
        let mut xn = {
            let q = self.wrapping_div(&guess);
            let t = guess.wrapping_add(&q);
            t.shr_vartime(1)
        };

        // If guess increased, the initial guess was low.
        // Repeat until reverse course.
        while guess.ct_cmp(&xn) == -1 {
            // Sometimes an increase is too far, especially with large
            // powers, and then takes a long time to walk back.  The upper
            // bound is based on bit size, so saturate on that.
            let res = Limb::ct_cmp(Limb(xn.bits_vartime() as Word), Limb(max_bits as Word)) - 1;
            let le = Limb::is_nonzero(Limb(res as Word));
            guess = Self::ct_select(cap, xn, le);
            xn = {
                let q = self.wrapping_div(&guess);
                let t = guess.wrapping_add(&q);
                t.shr_vartime(1)
            };
        }

        // Repeat while guess decreases.
        while guess.ct_cmp(&xn) == 1 && xn.ct_is_nonzero() == Word::MAX {
            guess = xn;
            xn = {
                let q = self.wrapping_div(&guess);
                let t = guess.wrapping_add(&q);
                t.shr_vartime(1)
            };
        }

        Self::ct_select(Self::ZERO, guess, self.ct_is_nonzero())
    }

    /// Wrapped sqrt is just normal √(`self`)
    /// There’s no way wrapping could ever happen.
    /// This function exists, so that all operations are accounted for in the wrapping operations.
    pub const fn wrapping_sqrt(&self) -> Self {
        self.sqrt()
    }

    /// Perform checked sqrt, returning a [`CtOption`] which `is_some`
    /// only if the √(`self`)² == self
    pub fn checked_sqrt(&self) -> CtOption<Self> {
        let r = self.sqrt();
        let s = r.wrapping_mul(&r);
        CtOption::new(r, self.ct_eq(&s))
    }
}

#[cfg(test)]
mod tests {
    use crate::{Limb, U256};

    #[cfg(feature = "rand")]
    use {
        crate::{CheckedMul, Random, U512},
        rand_chacha::ChaChaRng,
        rand_core::{RngCore, SeedableRng},
    };

    #[test]
    fn edge() {
        assert_eq!(U256::ZERO.sqrt(), U256::ZERO);
        assert_eq!(U256::ONE.sqrt(), U256::ONE);
        let mut half = U256::ZERO;
        for i in 0..half.limbs.len() / 2 {
            half.limbs[i] = Limb::MAX;
        }
        assert_eq!(U256::MAX.sqrt(), half,);
    }

    #[test]
    fn simple() {
        let tests = [
            (4u8, 2u8),
            (9, 3),
            (16, 4),
            (25, 5),
            (36, 6),
            (49, 7),
            (64, 8),
            (81, 9),
            (100, 10),
            (121, 11),
            (144, 12),
            (169, 13),
        ];
        for (a, e) in &tests {
            let l = U256::from(*a);
            let r = U256::from(*e);
            assert_eq!(l.sqrt(), r);
            assert_eq!(l.checked_sqrt().is_some().unwrap_u8(), 1u8);
        }
    }

    #[test]
    fn nonsquares() {
        assert_eq!(U256::from(2u8).sqrt(), U256::from(1u8));
        assert_eq!(U256::from(2u8).checked_sqrt().is_some().unwrap_u8(), 0);
        assert_eq!(U256::from(3u8).sqrt(), U256::from(1u8));
        assert_eq!(U256::from(3u8).checked_sqrt().is_some().unwrap_u8(), 0);
        assert_eq!(U256::from(5u8).sqrt(), U256::from(2u8));
        assert_eq!(U256::from(6u8).sqrt(), U256::from(2u8));
        assert_eq!(U256::from(7u8).sqrt(), U256::from(2u8));
        assert_eq!(U256::from(8u8).sqrt(), U256::from(2u8));
        assert_eq!(U256::from(10u8).sqrt(), U256::from(3u8));
    }

    #[cfg(feature = "rand")]
    #[test]
    fn fuzz() {
        let mut rng = ChaChaRng::from_seed([7u8; 32]);
        for _ in 0..50 {
            let t = rng.next_u32() as u64;
            let s = U256::from(t);
            let s2 = s.checked_mul(&s).unwrap();
            assert_eq!(s2.sqrt(), s);
            assert_eq!(s2.checked_sqrt().is_some().unwrap_u8(), 1);
        }

        for _ in 0..50 {
            let s = U256::random(&mut rng);
            let mut s2 = U512::ZERO;
            s2.limbs[..s.limbs.len()].copy_from_slice(&s.limbs);
            assert_eq!(s.square().sqrt(), s2);
        }
    }
}
