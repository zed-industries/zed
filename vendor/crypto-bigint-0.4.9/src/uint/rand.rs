//! Random number generator support

use super::UInt;
use crate::{Limb, NonZero, Random, RandomMod};
use rand_core::{CryptoRng, RngCore};
use subtle::ConstantTimeLess;

#[cfg_attr(docsrs, doc(cfg(feature = "rand_core")))]
impl<const LIMBS: usize> Random for UInt<LIMBS> {
    /// Generate a cryptographically secure random [`UInt`].
    fn random(mut rng: impl CryptoRng + RngCore) -> Self {
        let mut limbs = [Limb::ZERO; LIMBS];

        for limb in &mut limbs {
            *limb = Limb::random(&mut rng)
        }

        limbs.into()
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "rand_core")))]
impl<const LIMBS: usize> RandomMod for UInt<LIMBS> {
    /// Generate a cryptographically secure random [`UInt`] which is less than
    /// a given `modulus`.
    ///
    /// This function uses rejection sampling, a method which produces an
    /// unbiased distribution of in-range values provided the underlying
    /// [`CryptoRng`] is unbiased, but runs in variable-time.
    ///
    /// The variable-time nature of the algorithm should not pose a security
    /// issue so long as the underlying random number generator is truly a
    /// [`CryptoRng`], where previous outputs are unrelated to subsequent
    /// outputs and do not reveal information about the RNG's internal state.
    fn random_mod(mut rng: impl CryptoRng + RngCore, modulus: &NonZero<Self>) -> Self {
        let mut n = Self::ZERO;

        // TODO(tarcieri): use `div_ceil` when available
        // See: https://github.com/rust-lang/rust/issues/88581
        let mut n_limbs = modulus.bits_vartime() / Limb::BIT_SIZE;
        if n_limbs < LIMBS {
            n_limbs += 1;
        }

        // Compute the highest limb of `modulus` as a `NonZero`.
        // Add one to ensure `Limb::random_mod` returns values inclusive of this limb.
        let modulus_hi =
            NonZero::new(modulus.limbs[n_limbs.saturating_sub(1)].saturating_add(Limb::ONE))
                .unwrap(); // Always at least one due to `saturating_add`

        loop {
            for i in 0..n_limbs {
                n.limbs[i] = if (i + 1 == n_limbs) && (*modulus_hi != Limb::MAX) {
                    // Highest limb
                    Limb::random_mod(&mut rng, &modulus_hi)
                } else {
                    Limb::random(&mut rng)
                }
            }

            if n.ct_lt(modulus).into() {
                return n;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{NonZero, RandomMod, U256};
    use rand_core::SeedableRng;

    #[test]
    fn random_mod() {
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(1);

        // Ensure `random_mod` runs in a reasonable amount of time
        let modulus = NonZero::new(U256::from(42u8)).unwrap();
        let res = U256::random_mod(&mut rng, &modulus);

        // Sanity check that the return value isn't zero
        assert_ne!(res, U256::ZERO);

        // Ensure `random_mod` runs in a reasonable amount of time
        // when the modulus is larger than 1 limb
        let modulus = NonZero::new(U256::from(0x10000000000000001u128)).unwrap();
        let res = U256::random_mod(&mut rng, &modulus);

        // Sanity check that the return value isn't zero
        assert_ne!(res, U256::ZERO);
    }
}
