//! ECDSA verification key.

use crate::{
    hazmat::{DigestPrimitive, VerifyPrimitive},
    Error, Result, Signature, SignatureSize,
};
use core::{cmp::Ordering, fmt::Debug};
use elliptic_curve::{
    generic_array::ArrayLength,
    ops::Reduce,
    sec1::{self, EncodedPoint, FromEncodedPoint, ToEncodedPoint},
    AffinePoint, FieldSize, PointCompression, PrimeCurve, ProjectiveArithmetic, PublicKey, Scalar,
};
use signature::{
    digest::{Digest, FixedOutput},
    hazmat::PrehashVerifier,
    DigestVerifier, Verifier,
};

#[cfg(feature = "pkcs8")]
use elliptic_curve::pkcs8::{self, AssociatedOid, DecodePublicKey};

#[cfg(feature = "pem")]
use elliptic_curve::pkcs8::EncodePublicKey;

#[cfg(feature = "pem")]
use core::str::FromStr;

#[cfg(all(feature = "pem", feature = "serde"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "pem", feature = "serde"))))]
use serdect::serde::{de, ser, Deserialize, Serialize};

/// ECDSA verification key (i.e. public key). Generic over elliptic curves.
///
/// Requires an [`elliptic_curve::ProjectiveArithmetic`] impl on the curve, and a
/// [`VerifyPrimitive`] impl on its associated `AffinePoint` type.
///
/// # `serde` support
///
/// When the `serde` feature of this crate is enabled, it provides support for
/// serializing and deserializing ECDSA signatures using the `Serialize` and
/// `Deserialize` traits.
///
/// The serialization leverages the encoding used by the [`PublicKey`] type,
/// which is a binary-oriented ASN.1 DER encoding.
#[cfg_attr(docsrs, doc(cfg(feature = "verify")))]
#[derive(Clone, Debug)]
pub struct VerifyingKey<C>
where
    C: PrimeCurve + ProjectiveArithmetic,
{
    pub(crate) inner: PublicKey<C>,
}

impl<C> VerifyingKey<C>
where
    C: PrimeCurve + ProjectiveArithmetic,
    AffinePoint<C>: FromEncodedPoint<C> + ToEncodedPoint<C>,
    FieldSize<C>: sec1::ModulusSize,
{
    /// Initialize [`VerifyingKey`] from a SEC1-encoded public key.
    pub fn from_sec1_bytes(bytes: &[u8]) -> Result<Self> {
        PublicKey::from_sec1_bytes(bytes)
            .map(|pk| Self { inner: pk })
            .map_err(|_| Error::new())
    }

    /// Initialize [`VerifyingKey`] from an affine point.
    ///
    /// Returns an [`Error`] if the given affine point is the additive identity
    /// (a.k.a. point at infinity).
    pub fn from_affine(affine: AffinePoint<C>) -> Result<Self> {
        Ok(Self {
            inner: PublicKey::from_affine(affine).map_err(|_| Error::new())?,
        })
    }

    /// Initialize [`VerifyingKey`] from an [`EncodedPoint`].
    pub fn from_encoded_point(public_key: &EncodedPoint<C>) -> Result<Self> {
        Option::from(PublicKey::<C>::from_encoded_point(public_key))
            .map(|public_key| Self { inner: public_key })
            .ok_or_else(Error::new)
    }

    /// Serialize this [`VerifyingKey`] as a SEC1 [`EncodedPoint`], optionally
    /// applying point compression.
    pub fn to_encoded_point(&self, compress: bool) -> EncodedPoint<C> {
        self.inner.to_encoded_point(compress)
    }

    /// Borrow the inner [`AffinePoint`] for this public key.
    pub fn as_affine(&self) -> &AffinePoint<C> {
        self.inner.as_affine()
    }
}

impl<C> AsRef<AffinePoint<C>> for VerifyingKey<C>
where
    C: PrimeCurve + ProjectiveArithmetic,
    AffinePoint<C>: FromEncodedPoint<C> + ToEncodedPoint<C>,
    FieldSize<C>: sec1::ModulusSize,
{
    fn as_ref(&self) -> &AffinePoint<C> {
        self.as_affine()
    }
}

impl<C> Copy for VerifyingKey<C> where C: PrimeCurve + ProjectiveArithmetic {}

impl<C, D> DigestVerifier<D, Signature<C>> for VerifyingKey<C>
where
    C: PrimeCurve + ProjectiveArithmetic,
    D: Digest + FixedOutput<OutputSize = FieldSize<C>>,
    AffinePoint<C>: VerifyPrimitive<C>,
    Scalar<C>: Reduce<C::UInt>,
    SignatureSize<C>: ArrayLength<u8>,
{
    fn verify_digest(&self, msg_digest: D, signature: &Signature<C>) -> Result<()> {
        self.inner.as_affine().verify_digest(msg_digest, signature)
    }
}

impl<C> PrehashVerifier<Signature<C>> for VerifyingKey<C>
where
    C: PrimeCurve + ProjectiveArithmetic + DigestPrimitive,
    AffinePoint<C>: VerifyPrimitive<C>,
    Scalar<C>: Reduce<C::UInt>,
    SignatureSize<C>: ArrayLength<u8>,
{
    fn verify_prehash(&self, prehash: &[u8], signature: &Signature<C>) -> Result<()> {
        let prehash = C::prehash_to_field_bytes(prehash)?;
        self.inner.as_affine().verify_prehashed(prehash, signature)
    }
}

impl<C> Verifier<Signature<C>> for VerifyingKey<C>
where
    C: PrimeCurve + ProjectiveArithmetic + DigestPrimitive,
    C::Digest: FixedOutput<OutputSize = FieldSize<C>>,
    AffinePoint<C>: VerifyPrimitive<C>,
    Scalar<C>: Reduce<C::UInt>,
    SignatureSize<C>: ArrayLength<u8>,
{
    fn verify(&self, msg: &[u8], signature: &Signature<C>) -> Result<()> {
        self.verify_digest(C::Digest::new_with_prefix(msg), signature)
    }
}

impl<C> From<&VerifyingKey<C>> for EncodedPoint<C>
where
    C: PrimeCurve + ProjectiveArithmetic + PointCompression,
    AffinePoint<C>: FromEncodedPoint<C> + ToEncodedPoint<C>,
    FieldSize<C>: sec1::ModulusSize,
{
    fn from(verifying_key: &VerifyingKey<C>) -> EncodedPoint<C> {
        verifying_key.to_encoded_point(C::COMPRESS_POINTS)
    }
}

impl<C> From<PublicKey<C>> for VerifyingKey<C>
where
    C: PrimeCurve + ProjectiveArithmetic,
{
    fn from(public_key: PublicKey<C>) -> VerifyingKey<C> {
        VerifyingKey { inner: public_key }
    }
}

impl<C> From<&PublicKey<C>> for VerifyingKey<C>
where
    C: PrimeCurve + ProjectiveArithmetic,
{
    fn from(public_key: &PublicKey<C>) -> VerifyingKey<C> {
        (*public_key).into()
    }
}

impl<C> From<VerifyingKey<C>> for PublicKey<C>
where
    C: PrimeCurve + ProjectiveArithmetic,
{
    fn from(verifying_key: VerifyingKey<C>) -> PublicKey<C> {
        verifying_key.inner
    }
}

impl<C> From<&VerifyingKey<C>> for PublicKey<C>
where
    C: PrimeCurve + ProjectiveArithmetic,
{
    fn from(verifying_key: &VerifyingKey<C>) -> PublicKey<C> {
        (*verifying_key).into()
    }
}

impl<C> Eq for VerifyingKey<C> where C: PrimeCurve + ProjectiveArithmetic {}

impl<C> PartialEq for VerifyingKey<C>
where
    C: PrimeCurve + ProjectiveArithmetic,
{
    fn eq(&self, other: &Self) -> bool {
        self.inner.eq(&other.inner)
    }
}

impl<C> PartialOrd for VerifyingKey<C>
where
    C: PrimeCurve + ProjectiveArithmetic,
    AffinePoint<C>: FromEncodedPoint<C> + ToEncodedPoint<C>,
    FieldSize<C>: sec1::ModulusSize,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.inner.partial_cmp(&other.inner)
    }
}

impl<C> Ord for VerifyingKey<C>
where
    C: PrimeCurve + ProjectiveArithmetic,
    AffinePoint<C>: FromEncodedPoint<C> + ToEncodedPoint<C>,
    FieldSize<C>: sec1::ModulusSize,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.inner.cmp(&other.inner)
    }
}

impl<C> TryFrom<&[u8]> for VerifyingKey<C>
where
    C: PrimeCurve + ProjectiveArithmetic,
    AffinePoint<C>: FromEncodedPoint<C> + ToEncodedPoint<C>,
    FieldSize<C>: sec1::ModulusSize,
{
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self> {
        Self::from_sec1_bytes(bytes)
    }
}

#[cfg(feature = "pkcs8")]
#[cfg_attr(docsrs, doc(cfg(feature = "pkcs8")))]
impl<C> TryFrom<pkcs8::SubjectPublicKeyInfo<'_>> for VerifyingKey<C>
where
    C: PrimeCurve + AssociatedOid + ProjectiveArithmetic + PointCompression,
    AffinePoint<C>: FromEncodedPoint<C> + ToEncodedPoint<C>,
    FieldSize<C>: sec1::ModulusSize,
{
    type Error = pkcs8::spki::Error;

    fn try_from(spki: pkcs8::SubjectPublicKeyInfo<'_>) -> pkcs8::spki::Result<Self> {
        PublicKey::try_from(spki).map(|inner| Self { inner })
    }
}

#[cfg(feature = "pkcs8")]
#[cfg_attr(docsrs, doc(cfg(feature = "pkcs8")))]
impl<C> DecodePublicKey for VerifyingKey<C>
where
    C: PrimeCurve + AssociatedOid + ProjectiveArithmetic + PointCompression,
    AffinePoint<C>: FromEncodedPoint<C> + ToEncodedPoint<C>,
    FieldSize<C>: sec1::ModulusSize,
{
}

#[cfg(feature = "pem")]
#[cfg_attr(docsrs, doc(cfg(feature = "pem")))]
impl<C> EncodePublicKey for VerifyingKey<C>
where
    C: PrimeCurve + AssociatedOid + ProjectiveArithmetic + PointCompression,
    AffinePoint<C>: FromEncodedPoint<C> + ToEncodedPoint<C>,
    FieldSize<C>: sec1::ModulusSize,
{
    fn to_public_key_der(&self) -> pkcs8::spki::Result<pkcs8::Document> {
        self.inner.to_public_key_der()
    }
}

#[cfg(feature = "pem")]
#[cfg_attr(docsrs, doc(cfg(feature = "pem")))]
impl<C> FromStr for VerifyingKey<C>
where
    C: PrimeCurve + AssociatedOid + ProjectiveArithmetic + PointCompression,
    AffinePoint<C>: FromEncodedPoint<C> + ToEncodedPoint<C>,
    FieldSize<C>: sec1::ModulusSize,
{
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Self::from_public_key_pem(s).map_err(|_| Error::new())
    }
}

#[cfg(all(feature = "pem", feature = "serde"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "pem", feature = "serde"))))]
impl<C> Serialize for VerifyingKey<C>
where
    C: PrimeCurve + AssociatedOid + ProjectiveArithmetic + PointCompression,
    AffinePoint<C>: FromEncodedPoint<C> + ToEncodedPoint<C>,
    FieldSize<C>: sec1::ModulusSize,
{
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self.inner.serialize(serializer)
    }
}

#[cfg(all(feature = "pem", feature = "serde"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "pem", feature = "serde"))))]
impl<'de, C> Deserialize<'de> for VerifyingKey<C>
where
    C: PrimeCurve + AssociatedOid + ProjectiveArithmetic + PointCompression,
    AffinePoint<C>: FromEncodedPoint<C> + ToEncodedPoint<C>,
    FieldSize<C>: sec1::ModulusSize,
{
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        PublicKey::<C>::deserialize(deserializer).map(Into::into)
    }
}
