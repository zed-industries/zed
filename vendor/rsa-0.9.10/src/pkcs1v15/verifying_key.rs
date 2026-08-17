use super::{oid, pkcs1v15_generate_prefix, verify, Signature};
use crate::RsaPublicKey;
use alloc::vec::Vec;
use core::marker::PhantomData;
use digest::Digest;
use pkcs8::{
    spki::{
        der::AnyRef, AlgorithmIdentifierRef, AssociatedAlgorithmIdentifier,
        SignatureAlgorithmIdentifier,
    },
    AssociatedOid, Document, EncodePublicKey,
};
use signature::{hazmat::PrehashVerifier, DigestVerifier, Verifier};

/// Verifying key for `RSASSA-PKCS1-v1_5` signatures as described in [RFC8017 § 8.2].
///
/// [RFC8017 § 8.2]: https://datatracker.ietf.org/doc/html/rfc8017#section-8.2
#[derive(Debug)]
pub struct VerifyingKey<D>
where
    D: Digest,
{
    pub(super) inner: RsaPublicKey,
    pub(super) prefix: Vec<u8>,
    pub(super) phantom: PhantomData<D>,
}

impl<D> VerifyingKey<D>
where
    D: Digest + AssociatedOid,
{
    /// Create a new verifying key with a prefix for the digest `D`.
    pub fn new(key: RsaPublicKey) -> Self {
        Self {
            inner: key,
            prefix: pkcs1v15_generate_prefix::<D>(),
            phantom: Default::default(),
        }
    }

    /// Create a new verifying key with a prefix for the digest `D`.
    #[deprecated(since = "0.9.0", note = "use VerifyingKey::new instead")]
    pub fn new_with_prefix(key: RsaPublicKey) -> Self {
        Self::new(key)
    }
}

impl<D> VerifyingKey<D>
where
    D: Digest,
{
    /// Create a new verifying key from an RSA public key with an empty prefix.
    ///
    /// ## Note: unprefixed signatures are uncommon
    ///
    /// In most cases you'll want to use [`VerifyingKey::new`] instead.
    pub fn new_unprefixed(key: RsaPublicKey) -> Self {
        Self {
            inner: key,
            prefix: Vec::new(),
            phantom: Default::default(),
        }
    }
}

//
// `*Verifier` trait impls
//

impl<D> DigestVerifier<D, Signature> for VerifyingKey<D>
where
    D: Digest,
{
    fn verify_digest(&self, digest: D, signature: &Signature) -> signature::Result<()> {
        verify(
            &self.inner,
            &self.prefix,
            &digest.finalize(),
            &signature.inner,
            signature.len,
        )
        .map_err(|e| e.into())
    }
}

impl<D> PrehashVerifier<Signature> for VerifyingKey<D>
where
    D: Digest,
{
    fn verify_prehash(&self, prehash: &[u8], signature: &Signature) -> signature::Result<()> {
        verify(
            &self.inner,
            &self.prefix,
            prehash,
            &signature.inner,
            signature.len,
        )
        .map_err(|e| e.into())
    }
}

impl<D> Verifier<Signature> for VerifyingKey<D>
where
    D: Digest,
{
    fn verify(&self, msg: &[u8], signature: &Signature) -> signature::Result<()> {
        verify(
            &self.inner,
            &self.prefix.clone(),
            &D::digest(msg),
            &signature.inner,
            signature.len,
        )
        .map_err(|e| e.into())
    }
}

//
// Other trait impls
//

impl<D> AsRef<RsaPublicKey> for VerifyingKey<D>
where
    D: Digest,
{
    fn as_ref(&self) -> &RsaPublicKey {
        &self.inner
    }
}

impl<D> AssociatedAlgorithmIdentifier for VerifyingKey<D>
where
    D: Digest,
{
    type Params = AnyRef<'static>;

    const ALGORITHM_IDENTIFIER: AlgorithmIdentifierRef<'static> = pkcs1::ALGORITHM_ID;
}

// Implemented manually so we don't have to bind D with Clone
impl<D> Clone for VerifyingKey<D>
where
    D: Digest,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            prefix: self.prefix.clone(),
            phantom: Default::default(),
        }
    }
}

impl<D> EncodePublicKey for VerifyingKey<D>
where
    D: Digest,
{
    fn to_public_key_der(&self) -> pkcs8::spki::Result<Document> {
        self.inner.to_public_key_der()
    }
}

impl<D> From<RsaPublicKey> for VerifyingKey<D>
where
    D: Digest,
{
    fn from(key: RsaPublicKey) -> Self {
        Self::new_unprefixed(key)
    }
}

impl<D> From<VerifyingKey<D>> for RsaPublicKey
where
    D: Digest,
{
    fn from(key: VerifyingKey<D>) -> Self {
        key.inner
    }
}

impl<D> SignatureAlgorithmIdentifier for VerifyingKey<D>
where
    D: Digest + oid::RsaSignatureAssociatedOid,
{
    type Params = AnyRef<'static>;

    const SIGNATURE_ALGORITHM_IDENTIFIER: AlgorithmIdentifierRef<'static> =
        AlgorithmIdentifierRef {
            oid: D::OID,
            parameters: Some(AnyRef::NULL),
        };
}

impl<D> TryFrom<pkcs8::SubjectPublicKeyInfoRef<'_>> for VerifyingKey<D>
where
    D: Digest + AssociatedOid,
{
    type Error = pkcs8::spki::Error;

    fn try_from(spki: pkcs8::SubjectPublicKeyInfoRef<'_>) -> pkcs8::spki::Result<Self> {
        RsaPublicKey::try_from(spki).map(Self::new)
    }
}
