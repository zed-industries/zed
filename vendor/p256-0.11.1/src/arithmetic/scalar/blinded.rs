//! Random blinding support for [`Scalar`]

// TODO(tarcieri): make this generic (along with `Scalar::invert_vartime`)
// and extract it into the `elliptic-curve` crate so it can be reused across curves

use super::Scalar;
use core::borrow::Borrow;
use elliptic_curve::{
    group::ff::Field,
    ops::Invert,
    rand_core::{CryptoRng, RngCore},
    subtle::CtOption,
    zeroize::Zeroize,
};

/// Scalar blinded with a randomly generated masking value.
///
/// This provides a randomly blinded impl of [`Invert`] which is useful for
/// ECDSA ephemeral (`k`) scalars.
#[derive(Clone)]
#[cfg_attr(docsrs, doc(cfg(feature = "arithmetic")))]
pub struct BlindedScalar {
    /// Actual scalar value
    scalar: Scalar,

    /// Mask value
    mask: Scalar,
}

impl BlindedScalar {
    /// Create a new [`BlindedScalar`] from a scalar and a [`CryptoRng`]
    pub fn new(scalar: Scalar, rng: impl CryptoRng + RngCore) -> Self {
        Self {
            scalar,
            mask: Scalar::random(rng),
        }
    }
}

impl Borrow<Scalar> for BlindedScalar {
    fn borrow(&self) -> &Scalar {
        &self.scalar
    }
}

impl Invert for BlindedScalar {
    type Output = CtOption<Scalar>;

    fn invert(&self) -> CtOption<Scalar> {
        // prevent side channel analysis of scalar inversion by pre-and-post-multiplying
        // with the random masking scalar
        (self.scalar * self.mask)
            .invert_vartime()
            .map(|s| s * self.mask)
    }
}

impl Zeroize for BlindedScalar {
    fn zeroize(&mut self) {
        self.scalar.zeroize();
        self.mask.zeroize();
    }
}

impl Drop for BlindedScalar {
    fn drop(&mut self) {
        self.zeroize();
    }
}
