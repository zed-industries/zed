use super::decrypt_digest;
use crate::{
    dummy_rng::DummyRng,
    traits::{Decryptor, RandomizedDecryptor},
    Result, RsaPrivateKey,
};
use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use core::marker::PhantomData;
use digest::{Digest, FixedOutputReset};
use rand_core::CryptoRngCore;
use zeroize::ZeroizeOnDrop;

/// Decryption key for PKCS#1 v1.5 decryption as described in [RFC8017 § 7.1].
///
/// [RFC8017 § 7.1]: https://datatracker.ietf.org/doc/html/rfc8017#section-7.1
#[derive(Debug, Clone)]
pub struct DecryptingKey<D, MGD = D>
where
    D: Digest,
    MGD: Digest + FixedOutputReset,
{
    inner: RsaPrivateKey,
    label: Option<String>,
    phantom: PhantomData<D>,
    mg_phantom: PhantomData<MGD>,
}

impl<D, MGD> DecryptingKey<D, MGD>
where
    D: Digest,
    MGD: Digest + FixedOutputReset,
{
    /// Create a new verifying key from an RSA public key.
    pub fn new(key: RsaPrivateKey) -> Self {
        Self {
            inner: key,
            label: None,
            phantom: Default::default(),
            mg_phantom: Default::default(),
        }
    }

    /// Create a new verifying key from an RSA public key using provided label
    pub fn new_with_label<S: AsRef<str>>(key: RsaPrivateKey, label: S) -> Self {
        Self {
            inner: key,
            label: Some(label.as_ref().to_string()),
            phantom: Default::default(),
            mg_phantom: Default::default(),
        }
    }
}

impl<D, MGD> Decryptor for DecryptingKey<D, MGD>
where
    D: Digest,
    MGD: Digest + FixedOutputReset,
{
    fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>> {
        decrypt_digest::<DummyRng, D, MGD>(
            None,
            &self.inner,
            ciphertext,
            self.label.as_ref().cloned(),
        )
    }
}

impl<D, MGD> RandomizedDecryptor for DecryptingKey<D, MGD>
where
    D: Digest,
    MGD: Digest + FixedOutputReset,
{
    fn decrypt_with_rng<R: CryptoRngCore + ?Sized>(
        &self,
        rng: &mut R,
        ciphertext: &[u8],
    ) -> Result<Vec<u8>> {
        decrypt_digest::<_, D, MGD>(
            Some(rng),
            &self.inner,
            ciphertext,
            self.label.as_ref().cloned(),
        )
    }
}

impl<D, MGD> ZeroizeOnDrop for DecryptingKey<D, MGD>
where
    D: Digest,
    MGD: Digest + FixedOutputReset,
{
}
