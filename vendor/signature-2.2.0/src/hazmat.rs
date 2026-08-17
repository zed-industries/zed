//! Hazardous Materials: low-level APIs which can be insecure if misused.
//!
//! The traits in this module are not generally recommended, and should only be
//! used in special cases where they are specifically needed.
//!
//! Using them incorrectly can introduce security vulnerabilities. Please
//! carefully read the documentation before attempting to use them.

use crate::Error;

#[cfg(feature = "rand_core")]
use crate::rand_core::CryptoRngCore;

/// Sign the provided message prehash, returning a digital signature.
pub trait PrehashSigner<S> {
    /// Attempt to sign the given message digest, returning a digital signature
    /// on success, or an error if something went wrong.
    ///
    /// The `prehash` parameter should be the output of a secure cryptographic
    /// hash function.
    ///
    /// This API takes a `prehash` byte slice as there can potentially be many
    /// compatible lengths for the message digest for a given concrete signature
    /// algorithm.
    ///
    /// Allowed lengths are algorithm-dependent and up to a particular
    /// implementation to decide.
    fn sign_prehash(&self, prehash: &[u8]) -> Result<S, Error>;
}

/// Sign the provided message prehash using the provided external randomness source, returning a digital signature.
#[cfg(feature = "rand_core")]
pub trait RandomizedPrehashSigner<S> {
    /// Attempt to sign the given message digest, returning a digital signature
    /// on success, or an error if something went wrong.
    ///
    /// The `prehash` parameter should be the output of a secure cryptographic
    /// hash function.
    ///
    /// This API takes a `prehash` byte slice as there can potentially be many
    /// compatible lengths for the message digest for a given concrete signature
    /// algorithm.
    ///
    /// Allowed lengths are algorithm-dependent and up to a particular
    /// implementation to decide.
    fn sign_prehash_with_rng(
        &self,
        rng: &mut impl CryptoRngCore,
        prehash: &[u8],
    ) -> Result<S, Error>;
}

/// Verify the provided message prehash using `Self` (e.g. a public key)
pub trait PrehashVerifier<S> {
    /// Use `Self` to verify that the provided signature for a given message
    /// `prehash` is authentic.
    ///
    /// The `prehash` parameter should be the output of a secure cryptographic
    /// hash function.
    ///
    /// Returns `Error` if it is inauthentic or some other error occurred, or
    /// otherwise returns `Ok(())`.
    ///
    /// # ⚠️ Security Warning
    ///
    /// If `prehash` is something other than the output of a cryptographically
    /// secure hash function, an attacker can potentially forge signatures by
    /// solving a system of linear equations.
    fn verify_prehash(&self, prehash: &[u8], signature: &S) -> Result<(), Error>;
}
