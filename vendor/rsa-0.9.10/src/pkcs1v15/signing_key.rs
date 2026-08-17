use super::{oid, pkcs1v15_generate_prefix, sign, Signature, VerifyingKey};
use crate::{dummy_rng::DummyRng, Result, RsaPrivateKey};
use alloc::vec::Vec;
use core::marker::PhantomData;
use digest::Digest;
use pkcs8::{
    spki::{
        der::AnyRef, AlgorithmIdentifierRef, AssociatedAlgorithmIdentifier,
        SignatureAlgorithmIdentifier,
    },
    AssociatedOid, EncodePrivateKey, SecretDocument,
};
use rand_core::CryptoRngCore;
use signature::{
    hazmat::PrehashSigner, DigestSigner, Keypair, RandomizedDigestSigner, RandomizedSigner, Signer,
};
use zeroize::ZeroizeOnDrop;

/// Signing key for `RSASSA-PKCS1-v1_5` signatures as described in [RFC8017 § 8.2].
///
/// [RFC8017 § 8.2]: https://datatracker.ietf.org/doc/html/rfc8017#section-8.2
#[derive(Debug, Clone)]
pub struct SigningKey<D>
where
    D: Digest,
{
    inner: RsaPrivateKey,
    prefix: Vec<u8>,
    phantom: PhantomData<D>,
}

impl<D> SigningKey<D>
where
    D: Digest + AssociatedOid,
{
    /// Create a new signing key with a prefix for the digest `D`.
    pub fn new(key: RsaPrivateKey) -> Self {
        Self {
            inner: key,
            prefix: pkcs1v15_generate_prefix::<D>(),
            phantom: Default::default(),
        }
    }

    /// Generate a new signing key with a prefix for the digest `D`.
    pub fn random<R: CryptoRngCore + ?Sized>(rng: &mut R, bit_size: usize) -> Result<Self> {
        Ok(Self {
            inner: RsaPrivateKey::new(rng, bit_size)?,
            prefix: pkcs1v15_generate_prefix::<D>(),
            phantom: Default::default(),
        })
    }

    /// Create a new signing key with a prefix for the digest `D`.
    #[deprecated(since = "0.9.0", note = "use SigningKey::new instead")]
    pub fn new_with_prefix(key: RsaPrivateKey) -> Self {
        Self::new(key)
    }

    /// Generate a new signing key with a prefix for the digest `D`.
    #[deprecated(since = "0.9.0", note = "use SigningKey::random instead")]
    pub fn random_with_prefix<R: CryptoRngCore + ?Sized>(
        rng: &mut R,
        bit_size: usize,
    ) -> Result<Self> {
        Self::random(rng, bit_size)
    }
}

impl<D> SigningKey<D>
where
    D: Digest,
{
    /// Create a new signing key from the give RSA private key with an empty prefix.
    ///
    /// ## Note: unprefixed signatures are uncommon
    ///
    /// In most cases you'll want to use [`SigningKey::new`].
    pub fn new_unprefixed(key: RsaPrivateKey) -> Self {
        Self {
            inner: key,
            prefix: Vec::new(),
            phantom: Default::default(),
        }
    }

    /// Generate a new signing key with an empty prefix.
    pub fn random_unprefixed<R: CryptoRngCore + ?Sized>(
        rng: &mut R,
        bit_size: usize,
    ) -> Result<Self> {
        Ok(Self {
            inner: RsaPrivateKey::new(rng, bit_size)?,
            prefix: Vec::new(),
            phantom: Default::default(),
        })
    }
}

//
// `*Signer` trait impls
//

impl<D> DigestSigner<D, Signature> for SigningKey<D>
where
    D: Digest,
{
    fn try_sign_digest(&self, digest: D) -> signature::Result<Signature> {
        sign::<DummyRng>(None, &self.inner, &self.prefix, &digest.finalize())?
            .as_slice()
            .try_into()
    }
}

impl<D> PrehashSigner<Signature> for SigningKey<D>
where
    D: Digest,
{
    fn sign_prehash(&self, prehash: &[u8]) -> signature::Result<Signature> {
        sign::<DummyRng>(None, &self.inner, &self.prefix, prehash)?
            .as_slice()
            .try_into()
    }
}

impl<D> RandomizedDigestSigner<D, Signature> for SigningKey<D>
where
    D: Digest,
{
    fn try_sign_digest_with_rng(
        &self,
        rng: &mut impl CryptoRngCore,
        digest: D,
    ) -> signature::Result<Signature> {
        sign(Some(rng), &self.inner, &self.prefix, &digest.finalize())?
            .as_slice()
            .try_into()
    }
}

impl<D> RandomizedSigner<Signature> for SigningKey<D>
where
    D: Digest,
{
    fn try_sign_with_rng(
        &self,
        rng: &mut impl CryptoRngCore,
        msg: &[u8],
    ) -> signature::Result<Signature> {
        sign(Some(rng), &self.inner, &self.prefix, &D::digest(msg))?
            .as_slice()
            .try_into()
    }
}

impl<D> Signer<Signature> for SigningKey<D>
where
    D: Digest,
{
    fn try_sign(&self, msg: &[u8]) -> signature::Result<Signature> {
        sign::<DummyRng>(None, &self.inner, &self.prefix, &D::digest(msg))?
            .as_slice()
            .try_into()
    }
}

//
// Other trait impls
//

impl<D> AsRef<RsaPrivateKey> for SigningKey<D>
where
    D: Digest,
{
    fn as_ref(&self) -> &RsaPrivateKey {
        &self.inner
    }
}

impl<D> AssociatedAlgorithmIdentifier for SigningKey<D>
where
    D: Digest,
{
    type Params = AnyRef<'static>;

    const ALGORITHM_IDENTIFIER: AlgorithmIdentifierRef<'static> = pkcs1::ALGORITHM_ID;
}

impl<D> EncodePrivateKey for SigningKey<D>
where
    D: Digest,
{
    fn to_pkcs8_der(&self) -> pkcs8::Result<SecretDocument> {
        self.inner.to_pkcs8_der()
    }
}

impl<D> From<RsaPrivateKey> for SigningKey<D>
where
    D: Digest,
{
    fn from(key: RsaPrivateKey) -> Self {
        Self::new_unprefixed(key)
    }
}

impl<D> From<SigningKey<D>> for RsaPrivateKey
where
    D: Digest,
{
    fn from(key: SigningKey<D>) -> Self {
        key.inner
    }
}

impl<D> Keypair for SigningKey<D>
where
    D: Digest,
{
    type VerifyingKey = VerifyingKey<D>;

    fn verifying_key(&self) -> Self::VerifyingKey {
        VerifyingKey {
            inner: self.inner.to_public_key(),
            prefix: self.prefix.clone(),
            phantom: Default::default(),
        }
    }
}

impl<D> SignatureAlgorithmIdentifier for SigningKey<D>
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

impl<D> TryFrom<pkcs8::PrivateKeyInfo<'_>> for SigningKey<D>
where
    D: Digest + AssociatedOid,
{
    type Error = pkcs8::Error;

    fn try_from(private_key_info: pkcs8::PrivateKeyInfo<'_>) -> pkcs8::Result<Self> {
        RsaPrivateKey::try_from(private_key_info).map(Self::new)
    }
}

impl<D> ZeroizeOnDrop for SigningKey<D> where D: Digest {}
