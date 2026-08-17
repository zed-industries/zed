//! Random number generator support

use super::Limb;
use crate::{Encoding, NonZero, Random, RandomMod};
use rand_core::{CryptoRng, RngCore};
use subtle::ConstantTimeLess;

#[cfg_attr(docsrs, doc(cfg(feature = "rand_core")))]
impl Random for Limb {
    #[cfg(target_pointer_width = "32")]
    fn random(mut rng: impl CryptoRng + RngCore) -> Self {
        Self(rng.next_u32())
    }

    #[cfg(target_pointer_width = "64")]
    fn random(mut rng: impl CryptoRng + RngCore) -> Self {
        Self(rng.next_u64())
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "rand_core")))]
impl RandomMod for Limb {
    fn random_mod(mut rng: impl CryptoRng + RngCore, modulus: &NonZero<Self>) -> Self {
        let mut bytes = <Self as Encoding>::Repr::default();

        // TODO(tarcieri): use `div_ceil` when available
        // See: https://github.com/rust-lang/rust/issues/88581
        let mut n_bytes = modulus.bits() / 8;

        // Ensure the randomly generated value can always be larger than
        // the modulus in order to ensure a uniform distribution
        if n_bytes < Self::BYTE_SIZE {
            n_bytes += 1;
        }

        loop {
            rng.fill_bytes(&mut bytes[..n_bytes]);
            let n = Limb::from_le_bytes(bytes);

            if n.ct_lt(modulus).into() {
                return n;
            }
        }
    }
}
