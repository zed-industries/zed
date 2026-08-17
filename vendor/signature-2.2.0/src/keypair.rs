//! Signing keypairs.

/// Signing keypair with an associated verifying key.
///
/// This represents a type which holds both a signing key and a verifying key.
pub trait Keypair {
    /// Verifying key type for this keypair.
    type VerifyingKey: Clone;

    /// Get the verifying key which can verify signatures produced by the
    /// signing key portion of this keypair.
    fn verifying_key(&self) -> Self::VerifyingKey;
}

/// Signing keypair with an associated verifying key.
///
/// This represents a type which holds both a signing key and a verifying key.
pub trait KeypairRef: AsRef<Self::VerifyingKey> {
    /// Verifying key type for this keypair.
    type VerifyingKey: Clone;
}

impl<K: KeypairRef> Keypair for K {
    type VerifyingKey = <Self as KeypairRef>::VerifyingKey;

    fn verifying_key(&self) -> Self::VerifyingKey {
        self.as_ref().clone()
    }
}
