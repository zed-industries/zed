//! Elliptic Curve Diffie-Hellman Support.
//!
//! This module contains a generic ECDH implementation which is usable with
//! any elliptic curve which implements the [`ProjectiveArithmetic`] trait (presently
//! the `k256` and `p256` crates)
//!
//! # ECDH Ephemeral (ECDHE) Usage
//!
//! Ephemeral Diffie-Hellman provides a one-time key exchange between two peers
//! using a randomly generated set of keys for each exchange.
//!
//! In practice ECDHE is used as part of an [Authenticated Key Exchange (AKE)][AKE]
//! protocol (e.g. [SIGMA]), where an existing cryptographic trust relationship
//! can be used to determine the authenticity of the ephemeral keys, such as
//! a digital signature. Without such an additional step, ECDHE is insecure!
//! (see security warning below)
//!
//! See the documentation for the [`EphemeralSecret`] type for more information
//! on performing ECDH ephemeral key exchanges.
//!
//! # Static ECDH Usage
//!
//! Static ECDH key exchanges are supported via the low-level
//! [`diffie_hellman`] function.
//!
//! [AKE]: https://en.wikipedia.org/wiki/Authenticated_Key_Exchange
//! [SIGMA]: https://webee.technion.ac.il/~hugo/sigma-pdf.pdf

use crate::{
    AffineArithmetic, AffinePoint, AffineXCoordinate, Curve, FieldBytes, NonZeroScalar,
    ProjectiveArithmetic, ProjectivePoint, PublicKey,
};
use core::borrow::Borrow;
use digest::{crypto_common::BlockSizeUser, Digest};
use group::Curve as _;
use hkdf::{hmac::SimpleHmac, Hkdf};
use rand_core::{CryptoRng, RngCore};
use zeroize::{Zeroize, ZeroizeOnDrop};

/// Low-level Elliptic Curve Diffie-Hellman (ECDH) function.
///
/// Whenever possible, we recommend using the high-level ECDH ephemeral API
/// provided by [`EphemeralSecret`].
///
/// However, if you are implementing a protocol which requires a static scalar
/// value as part of an ECDH exchange, this API can be used to compute a
/// [`SharedSecret`] from that value.
///
/// Note that this API operates on the low-level [`NonZeroScalar`] and
/// [`AffinePoint`] types. If you are attempting to use the higher-level
/// [`SecretKey`][`crate::SecretKey`] and [`PublicKey`] types, you will
/// need to use the following conversions:
///
/// ```ignore
/// let shared_secret = elliptic_curve::ecdh::diffie_hellman(
///     secret_key.to_nonzero_scalar(),
///     public_key.as_affine()
/// );
/// ```
pub fn diffie_hellman<C>(
    secret_key: impl Borrow<NonZeroScalar<C>>,
    public_key: impl Borrow<AffinePoint<C>>,
) -> SharedSecret<C>
where
    C: Curve + ProjectiveArithmetic,
{
    let public_point = ProjectivePoint::<C>::from(*public_key.borrow());
    let secret_point = (public_point * secret_key.borrow().as_ref()).to_affine();
    SharedSecret::new(secret_point)
}

/// Ephemeral Diffie-Hellman Secret.
///
/// These are ephemeral "secret key" values which are deliberately designed
/// to avoid being persisted.
///
/// To perform an ephemeral Diffie-Hellman exchange, do the following:
///
/// - Have each participant generate an [`EphemeralSecret`] value
/// - Compute the [`PublicKey`] for that value
/// - Have each peer provide their [`PublicKey`] to their counterpart
/// - Use [`EphemeralSecret`] and the other participant's [`PublicKey`]
///   to compute a [`SharedSecret`] value.
///
/// # ⚠️ SECURITY WARNING ⚠️
///
/// Ephemeral Diffie-Hellman exchanges are unauthenticated and without a
/// further authentication step are trivially vulnerable to man-in-the-middle
/// attacks!
///
/// These exchanges should be performed in the context of a protocol which
/// takes further steps to authenticate the peers in a key exchange.
pub struct EphemeralSecret<C>
where
    C: Curve + ProjectiveArithmetic,
{
    scalar: NonZeroScalar<C>,
}

impl<C> EphemeralSecret<C>
where
    C: Curve + ProjectiveArithmetic,
{
    /// Generate a cryptographically random [`EphemeralSecret`].
    pub fn random(rng: impl CryptoRng + RngCore) -> Self {
        Self {
            scalar: NonZeroScalar::random(rng),
        }
    }

    /// Get the public key associated with this ephemeral secret.
    ///
    /// The `compress` flag enables point compression.
    pub fn public_key(&self) -> PublicKey<C> {
        PublicKey::from_secret_scalar(&self.scalar)
    }

    /// Compute a Diffie-Hellman shared secret from an ephemeral secret and the
    /// public key of the other participant in the exchange.
    pub fn diffie_hellman(&self, public_key: &PublicKey<C>) -> SharedSecret<C> {
        diffie_hellman(&self.scalar, public_key.as_affine())
    }
}

impl<C> From<&EphemeralSecret<C>> for PublicKey<C>
where
    C: Curve + ProjectiveArithmetic,
{
    fn from(ephemeral_secret: &EphemeralSecret<C>) -> Self {
        ephemeral_secret.public_key()
    }
}

impl<C> Zeroize for EphemeralSecret<C>
where
    C: Curve + ProjectiveArithmetic,
{
    fn zeroize(&mut self) {
        self.scalar.zeroize()
    }
}

impl<C> ZeroizeOnDrop for EphemeralSecret<C> where C: Curve + ProjectiveArithmetic {}

impl<C> Drop for EphemeralSecret<C>
where
    C: Curve + ProjectiveArithmetic,
{
    fn drop(&mut self) {
        self.zeroize();
    }
}

/// Shared secret value computed via ECDH key agreement.
pub struct SharedSecret<C: Curve> {
    /// Computed secret value
    secret_bytes: FieldBytes<C>,
}

impl<C: Curve> SharedSecret<C> {
    /// Create a new [`SharedSecret`] from an [`AffinePoint`] for this curve.
    #[inline]
    fn new(point: AffinePoint<C>) -> Self
    where
        C: AffineArithmetic,
    {
        Self {
            secret_bytes: point.x(),
        }
    }

    /// Use [HKDF] (HMAC-based Extract-and-Expand Key Derivation Function) to
    /// extract entropy from this shared secret.
    ///
    /// This method can be used to transform the shared secret into uniformly
    /// random values which are suitable as key material.
    ///
    /// The `D` type parameter is a cryptographic digest function.
    /// `sha2::Sha256` is a common choice for use with HKDF.
    ///
    /// The `salt` parameter can be used to supply additional randomness.
    /// Some examples include:
    ///
    /// - randomly generated (but authenticated) string
    /// - fixed application-specific value
    /// - previous shared secret used for rekeying (as in TLS 1.3 and Noise)
    ///
    /// After initializing HKDF, use [`Hkdf::expand`] to obtain output key
    /// material.
    ///
    /// [HKDF]: https://en.wikipedia.org/wiki/HKDF
    pub fn extract<D>(&self, salt: Option<&[u8]>) -> Hkdf<D, SimpleHmac<D>>
    where
        D: BlockSizeUser + Clone + Digest,
    {
        Hkdf::new(salt, &self.secret_bytes)
    }

    /// This value contains the raw serialized x-coordinate of the elliptic curve
    /// point computed from a Diffie-Hellman exchange, serialized as bytes.
    ///
    /// When in doubt, use [`SharedSecret::extract`] instead.
    ///
    /// # ⚠️ WARNING: NOT UNIFORMLY RANDOM! ⚠️
    ///
    /// This value is not uniformly random and should not be used directly
    /// as a cryptographic key for anything which requires that property
    /// (e.g. symmetric ciphers).
    ///
    /// Instead, the resulting value should be used as input to a Key Derivation
    /// Function (KDF) or cryptographic hash function to produce a symmetric key.
    /// The [`SharedSecret::extract`] function will do this for you.
    pub fn raw_secret_bytes(&self) -> &FieldBytes<C> {
        &self.secret_bytes
    }
}

impl<C: Curve> From<FieldBytes<C>> for SharedSecret<C> {
    /// NOTE: this impl is intended to be used by curve implementations to
    /// instantiate a [`SharedSecret`] value from their respective
    /// [`AffinePoint`] type.
    ///
    /// Curve implementations should provide the field element representing
    /// the affine x-coordinate as `secret_bytes`.
    fn from(secret_bytes: FieldBytes<C>) -> Self {
        Self { secret_bytes }
    }
}

impl<C: Curve> ZeroizeOnDrop for SharedSecret<C> {}

impl<C: Curve> Drop for SharedSecret<C> {
    fn drop(&mut self) {
        self.secret_bytes.zeroize()
    }
}
