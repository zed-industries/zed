//! ECDSA signing key.

// TODO(tarcieri): support for hardware crypto accelerators

use crate::{
    hazmat::{DigestPrimitive, SignPrimitive},
    Error, Result, Signature, SignatureSize,
};
use core::fmt::{self, Debug};
use elliptic_curve::{
    generic_array::ArrayLength,
    group::ff::PrimeField,
    ops::{Invert, Reduce},
    subtle::{Choice, ConstantTimeEq, CtOption},
    zeroize::{Zeroize, ZeroizeOnDrop},
    FieldBytes, FieldSize, NonZeroScalar, PrimeCurve, ProjectiveArithmetic, Scalar, SecretKey,
};
use signature::{
    digest::{core_api::BlockSizeUser, Digest, FixedOutput, FixedOutputReset},
    hazmat::PrehashSigner,
    rand_core::{CryptoRng, RngCore},
    DigestSigner, RandomizedDigestSigner, RandomizedSigner, Signer,
};

#[cfg(feature = "pem")]
use {
    crate::elliptic_curve::pkcs8::{EncodePrivateKey, SecretDocument},
    core::str::FromStr,
};

#[cfg(feature = "pkcs8")]
use crate::elliptic_curve::{
    pkcs8::{self, AssociatedOid, DecodePrivateKey},
    sec1::{self, FromEncodedPoint, ToEncodedPoint},
    AffinePoint,
};

#[cfg(feature = "verify")]
use {crate::verify::VerifyingKey, elliptic_curve::PublicKey, signature::Keypair};

/// ECDSA signing key. Generic over elliptic curves.
///
/// Requires an [`elliptic_curve::ProjectiveArithmetic`] impl on the curve, and a
/// [`SignPrimitive`] impl on its associated `Scalar` type.
#[derive(Clone)]
#[cfg_attr(docsrs, doc(cfg(feature = "sign")))]
pub struct SigningKey<C>
where
    C: PrimeCurve + ProjectiveArithmetic,
    Scalar<C>: Invert<Output = CtOption<Scalar<C>>> + Reduce<C::UInt> + SignPrimitive<C>,
    SignatureSize<C>: ArrayLength<u8>,
{
    /// ECDSA signing keys are non-zero elements of a given curve's scalar field.
    secret_scalar: NonZeroScalar<C>,

    /// Verifying key which corresponds to this signing key.
    #[cfg(feature = "verify")]
    verifying_key: VerifyingKey<C>,
}

impl<C> SigningKey<C>
where
    C: PrimeCurve + ProjectiveArithmetic,
    Scalar<C>: Invert<Output = CtOption<Scalar<C>>> + Reduce<C::UInt> + SignPrimitive<C>,
    SignatureSize<C>: ArrayLength<u8>,
{
    /// Generate a cryptographically random [`SigningKey`].
    pub fn random(rng: impl CryptoRng + RngCore) -> Self {
        NonZeroScalar::<C>::random(rng).into()
    }

    /// Initialize signing key from a raw scalar serialized as a byte slice.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        SecretKey::<C>::from_be_bytes(bytes)
            .map(|sk| sk.to_nonzero_scalar().into())
            .map_err(|_| Error::new())
    }

    /// Serialize this [`SigningKey`] as bytes
    pub fn to_bytes(&self) -> FieldBytes<C> {
        self.secret_scalar.to_repr()
    }

    /// Borrow the secret [`NonZeroScalar`] value for this key.
    ///
    /// # ⚠️ Warning
    ///
    /// This value is key material.
    ///
    /// Please treat it with the care it deserves!
    pub fn as_nonzero_scalar(&self) -> &NonZeroScalar<C> {
        &self.secret_scalar
    }

    /// Get the [`VerifyingKey`] which corresponds to this [`SigningKey`]
    // TODO(tarcieri): make this return `&VerifyingKey<C>` in the next breaking release
    #[cfg(feature = "verify")]
    #[cfg_attr(docsrs, doc(cfg(feature = "verify")))]
    pub fn verifying_key(&self) -> VerifyingKey<C> {
        self.verifying_key
    }
}

#[cfg(feature = "verify")]
#[cfg_attr(docsrs, doc(cfg(feature = "verify")))]
impl<C> AsRef<VerifyingKey<C>> for SigningKey<C>
where
    C: PrimeCurve + ProjectiveArithmetic,
    Scalar<C>: Invert<Output = CtOption<Scalar<C>>> + Reduce<C::UInt> + SignPrimitive<C>,
    SignatureSize<C>: ArrayLength<u8>,
{
    fn as_ref(&self) -> &VerifyingKey<C> {
        &self.verifying_key
    }
}

impl<C> ConstantTimeEq for SigningKey<C>
where
    C: PrimeCurve + ProjectiveArithmetic,
    Scalar<C>: Invert<Output = CtOption<Scalar<C>>> + Reduce<C::UInt> + SignPrimitive<C>,
    SignatureSize<C>: ArrayLength<u8>,
{
    fn ct_eq(&self, other: &Self) -> Choice {
        self.secret_scalar.ct_eq(&other.secret_scalar)
    }
}

impl<C> Debug for SigningKey<C>
where
    C: PrimeCurve + ProjectiveArithmetic,
    Scalar<C>: Invert<Output = CtOption<Scalar<C>>> + Reduce<C::UInt> + SignPrimitive<C>,
    SignatureSize<C>: ArrayLength<u8>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SigningKey").finish_non_exhaustive()
    }
}

impl<C> Drop for SigningKey<C>
where
    C: PrimeCurve + ProjectiveArithmetic,
    Scalar<C>: Invert<Output = CtOption<Scalar<C>>> + Reduce<C::UInt> + SignPrimitive<C>,
    SignatureSize<C>: ArrayLength<u8>,
{
    fn drop(&mut self) {
        self.secret_scalar.zeroize();
    }
}

impl<C> ZeroizeOnDrop for SigningKey<C>
where
    C: PrimeCurve + ProjectiveArithmetic,
    Scalar<C>: Invert<Output = CtOption<Scalar<C>>> + Reduce<C::UInt> + SignPrimitive<C>,
    SignatureSize<C>: ArrayLength<u8>,
{
}

/// Constant-time comparison
impl<C> Eq for SigningKey<C>
where
    C: PrimeCurve + ProjectiveArithmetic,
    Scalar<C>: Invert<Output = CtOption<Scalar<C>>> + Reduce<C::UInt> + SignPrimitive<C>,
    SignatureSize<C>: ArrayLength<u8>,
{
}

/// Constant-time comparison
impl<C> PartialEq for SigningKey<C>
where
    C: PrimeCurve + ProjectiveArithmetic,
    Scalar<C>: Invert<Output = CtOption<Scalar<C>>> + Reduce<C::UInt> + SignPrimitive<C>,
    SignatureSize<C>: ArrayLength<u8>,
{
    fn eq(&self, other: &SigningKey<C>) -> bool {
        self.ct_eq(other).into()
    }
}

impl<C> From<SecretKey<C>> for SigningKey<C>
where
    C: PrimeCurve + ProjectiveArithmetic,
    Scalar<C>: Invert<Output = CtOption<Scalar<C>>> + Reduce<C::UInt> + SignPrimitive<C>,
    SignatureSize<C>: ArrayLength<u8>,
{
    fn from(secret_key: SecretKey<C>) -> Self {
        Self::from(&secret_key)
    }
}

impl<C> From<&SecretKey<C>> for SigningKey<C>
where
    C: PrimeCurve + ProjectiveArithmetic,
    Scalar<C>: Invert<Output = CtOption<Scalar<C>>> + Reduce<C::UInt> + SignPrimitive<C>,
    SignatureSize<C>: ArrayLength<u8>,
{
    fn from(secret_key: &SecretKey<C>) -> Self {
        secret_key.to_nonzero_scalar().into()
    }
}

impl<C> From<SigningKey<C>> for SecretKey<C>
where
    C: PrimeCurve + ProjectiveArithmetic,
    Scalar<C>: Invert<Output = CtOption<Scalar<C>>> + Reduce<C::UInt> + SignPrimitive<C>,
    SignatureSize<C>: ArrayLength<u8>,
{
    fn from(key: SigningKey<C>) -> Self {
        key.secret_scalar.into()
    }
}

impl<C> From<&SigningKey<C>> for SecretKey<C>
where
    C: PrimeCurve + ProjectiveArithmetic,
    Scalar<C>: Invert<Output = CtOption<Scalar<C>>> + Reduce<C::UInt> + SignPrimitive<C>,
    SignatureSize<C>: ArrayLength<u8>,
{
    fn from(secret_key: &SigningKey<C>) -> Self {
        secret_key.secret_scalar.into()
    }
}

impl<C, D> DigestSigner<D, Signature<C>> for SigningKey<C>
where
    C: PrimeCurve + ProjectiveArithmetic,
    C::UInt: for<'a> From<&'a Scalar<C>>,
    D: Digest + BlockSizeUser + FixedOutput<OutputSize = FieldSize<C>> + FixedOutputReset,
    Scalar<C>: Invert<Output = CtOption<Scalar<C>>> + Reduce<C::UInt> + SignPrimitive<C>,

    SignatureSize<C>: ArrayLength<u8>,
{
    /// Sign message digest using a deterministic ephemeral scalar (`k`)
    /// computed using the algorithm described in [RFC6979 § 3.2].
    ///
    /// [RFC6979 § 3.2]: https://tools.ietf.org/html/rfc6979#section-3
    fn try_sign_digest(&self, msg_digest: D) -> Result<Signature<C>> {
        Ok(self
            .secret_scalar
            .try_sign_digest_rfc6979::<D>(msg_digest, &[])?
            .0)
    }
}

#[cfg(feature = "verify")]
#[cfg_attr(docsrs, doc(cfg(feature = "verify")))]
impl<C> Keypair<Signature<C>> for SigningKey<C>
where
    C: PrimeCurve + ProjectiveArithmetic,
    Scalar<C>: Invert<Output = CtOption<Scalar<C>>> + Reduce<C::UInt> + SignPrimitive<C>,
    SignatureSize<C>: ArrayLength<u8>,
{
    type VerifyingKey = VerifyingKey<C>;
}

impl<C> PrehashSigner<Signature<C>> for SigningKey<C>
where
    C: PrimeCurve + ProjectiveArithmetic + DigestPrimitive,
    C::Digest: BlockSizeUser + FixedOutput<OutputSize = FieldSize<C>> + FixedOutputReset,
    C::UInt: for<'a> From<&'a Scalar<C>>,
    Scalar<C>: Invert<Output = CtOption<Scalar<C>>> + Reduce<C::UInt> + SignPrimitive<C>,
    SignatureSize<C>: ArrayLength<u8>,
{
    fn sign_prehash(&self, prehash: &[u8]) -> Result<Signature<C>> {
        let prehash = C::prehash_to_field_bytes(prehash)?;

        Ok(self
            .secret_scalar
            .try_sign_prehashed_rfc6979::<C::Digest>(prehash, &[])?
            .0)
    }
}

impl<C> Signer<Signature<C>> for SigningKey<C>
where
    Self: DigestSigner<C::Digest, Signature<C>>,
    C: PrimeCurve + ProjectiveArithmetic + DigestPrimitive,
    Scalar<C>: Invert<Output = CtOption<Scalar<C>>> + Reduce<C::UInt> + SignPrimitive<C>,
    SignatureSize<C>: ArrayLength<u8>,
{
    fn try_sign(&self, msg: &[u8]) -> Result<Signature<C>> {
        self.try_sign_digest(C::Digest::new_with_prefix(msg))
    }
}

impl<C, D> RandomizedDigestSigner<D, Signature<C>> for SigningKey<C>
where
    C: PrimeCurve + ProjectiveArithmetic,
    C::UInt: for<'a> From<&'a Scalar<C>>,
    D: Digest + BlockSizeUser + FixedOutput<OutputSize = FieldSize<C>> + FixedOutputReset,
    Scalar<C>: Invert<Output = CtOption<Scalar<C>>> + Reduce<C::UInt> + SignPrimitive<C>,
    SignatureSize<C>: ArrayLength<u8>,
{
    /// Sign message prehash using an ephemeral scalar (`k`) derived according
    /// to a variant of RFC 6979 (Section 3.6) which supplies additional
    /// entropy from an RNG.
    fn try_sign_digest_with_rng(
        &self,
        mut rng: impl CryptoRng + RngCore,
        msg_digest: D,
    ) -> Result<Signature<C>> {
        let mut ad = FieldBytes::<C>::default();
        rng.fill_bytes(&mut ad);
        Ok(self
            .secret_scalar
            .try_sign_digest_rfc6979::<D>(msg_digest, &ad)?
            .0)
    }
}

impl<C> RandomizedSigner<Signature<C>> for SigningKey<C>
where
    Self: RandomizedDigestSigner<C::Digest, Signature<C>>,
    C: PrimeCurve + ProjectiveArithmetic + DigestPrimitive,
    Scalar<C>: Invert<Output = CtOption<Scalar<C>>> + Reduce<C::UInt> + SignPrimitive<C>,
    SignatureSize<C>: ArrayLength<u8>,
{
    fn try_sign_with_rng(&self, rng: impl CryptoRng + RngCore, msg: &[u8]) -> Result<Signature<C>> {
        self.try_sign_digest_with_rng(rng, C::Digest::new_with_prefix(msg))
    }
}

impl<C> From<NonZeroScalar<C>> for SigningKey<C>
where
    C: PrimeCurve + ProjectiveArithmetic,
    Scalar<C>: Invert<Output = CtOption<Scalar<C>>> + Reduce<C::UInt> + SignPrimitive<C>,
    SignatureSize<C>: ArrayLength<u8>,
{
    fn from(secret_scalar: NonZeroScalar<C>) -> Self {
        #[cfg(feature = "verify")]
        let public_key = PublicKey::from_secret_scalar(&secret_scalar);

        Self {
            secret_scalar,
            #[cfg(feature = "verify")]
            verifying_key: public_key.into(),
        }
    }
}

impl<C> TryFrom<&[u8]> for SigningKey<C>
where
    C: PrimeCurve + ProjectiveArithmetic,
    Scalar<C>: Invert<Output = CtOption<Scalar<C>>> + Reduce<C::UInt> + SignPrimitive<C>,
    SignatureSize<C>: ArrayLength<u8>,
{
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self> {
        Self::from_bytes(bytes)
    }
}

#[cfg(feature = "verify")]
impl<C> From<&SigningKey<C>> for VerifyingKey<C>
where
    C: PrimeCurve + ProjectiveArithmetic,
    Scalar<C>: Invert<Output = CtOption<Scalar<C>>> + Reduce<C::UInt> + SignPrimitive<C>,
    SignatureSize<C>: ArrayLength<u8>,
{
    fn from(signing_key: &SigningKey<C>) -> VerifyingKey<C> {
        signing_key.verifying_key()
    }
}

#[cfg(feature = "pkcs8")]
#[cfg_attr(docsrs, doc(cfg(feature = "pkcs8")))]
impl<C> TryFrom<pkcs8::PrivateKeyInfo<'_>> for SigningKey<C>
where
    C: PrimeCurve + AssociatedOid + ProjectiveArithmetic,
    AffinePoint<C>: FromEncodedPoint<C> + ToEncodedPoint<C>,
    FieldSize<C>: sec1::ModulusSize,
    Scalar<C>: Invert<Output = CtOption<Scalar<C>>> + Reduce<C::UInt> + SignPrimitive<C>,
    SignatureSize<C>: ArrayLength<u8>,
{
    type Error = pkcs8::Error;

    fn try_from(private_key_info: pkcs8::PrivateKeyInfo<'_>) -> pkcs8::Result<Self> {
        SecretKey::try_from(private_key_info).map(Into::into)
    }
}

#[cfg(feature = "pem")]
#[cfg_attr(docsrs, doc(cfg(feature = "pem")))]
impl<C> EncodePrivateKey for SigningKey<C>
where
    C: AssociatedOid + PrimeCurve + ProjectiveArithmetic,
    AffinePoint<C>: FromEncodedPoint<C> + ToEncodedPoint<C>,
    FieldSize<C>: sec1::ModulusSize,
    Scalar<C>: Invert<Output = CtOption<Scalar<C>>> + Reduce<C::UInt> + SignPrimitive<C>,
    SignatureSize<C>: ArrayLength<u8>,
{
    fn to_pkcs8_der(&self) -> pkcs8::Result<SecretDocument> {
        SecretKey::from(self.secret_scalar).to_pkcs8_der()
    }
}

#[cfg(feature = "pkcs8")]
#[cfg_attr(docsrs, doc(cfg(feature = "pkcs8")))]
impl<C> DecodePrivateKey for SigningKey<C>
where
    C: PrimeCurve + AssociatedOid + ProjectiveArithmetic,
    AffinePoint<C>: FromEncodedPoint<C> + ToEncodedPoint<C>,
    FieldSize<C>: sec1::ModulusSize,
    Scalar<C>: Invert<Output = CtOption<Scalar<C>>> + Reduce<C::UInt> + SignPrimitive<C>,
    SignatureSize<C>: ArrayLength<u8>,
{
}

#[cfg(feature = "pem")]
#[cfg_attr(docsrs, doc(cfg(feature = "pem")))]
impl<C> FromStr for SigningKey<C>
where
    C: PrimeCurve + AssociatedOid + ProjectiveArithmetic,
    AffinePoint<C>: FromEncodedPoint<C> + ToEncodedPoint<C>,
    FieldSize<C>: sec1::ModulusSize,
    Scalar<C>: Invert<Output = CtOption<Scalar<C>>> + Reduce<C::UInt> + SignPrimitive<C>,
    SignatureSize<C>: ArrayLength<u8>,
{
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_pkcs8_pem(s).map_err(|_| Error::new())
    }
}
