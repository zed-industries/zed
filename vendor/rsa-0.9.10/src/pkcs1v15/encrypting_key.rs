use super::encrypt;
use crate::{traits::RandomizedEncryptor, Result, RsaPublicKey};
use alloc::vec::Vec;
use rand_core::CryptoRngCore;

/// Encryption key for PKCS#1 v1.5 encryption as described in [RFC8017 § 7.2].
///
/// [RFC8017 § 7.2]: https://datatracker.ietf.org/doc/html/rfc8017#section-7.2
#[derive(Debug, Clone)]
pub struct EncryptingKey {
    pub(super) inner: RsaPublicKey,
}

impl EncryptingKey {
    /// Create a new verifying key from an RSA public key.
    pub fn new(key: RsaPublicKey) -> Self {
        Self { inner: key }
    }
}

impl RandomizedEncryptor for EncryptingKey {
    fn encrypt_with_rng<R: CryptoRngCore + ?Sized>(
        &self,
        rng: &mut R,
        msg: &[u8],
    ) -> Result<Vec<u8>> {
        encrypt(rng, &self.inner, msg)
    }
}
