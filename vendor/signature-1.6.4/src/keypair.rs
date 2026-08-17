//! Signing keypairs.

use crate::Signature;

/// Signing keypair with an associated verifying key.
///
/// This represents a type which holds both a signing key and a verifying key.
pub trait Keypair<S: Signature>: AsRef<Self::VerifyingKey> {
    /// Verifying key type for this keypair.
    type VerifyingKey;

    /// Get the verifying key which can verify signatures produced by the
    /// signing key portion of this keypair.
    fn verifying_key(&self) -> &Self::VerifyingKey {
        self.as_ref()
    }
}
