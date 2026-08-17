//! Support for SEC1 elliptic curve encoding formats.
//!
//! <https://www.secg.org/sec1-v2.pdf>

pub use sec1::point::{Coordinates, ModulusSize, Tag};

use crate::{Curve, FieldSize, Result, SecretKey};
use generic_array::GenericArray;
use subtle::CtOption;

#[cfg(feature = "arithmetic")]
use crate::{AffinePoint, Error, ProjectiveArithmetic};

/// Encoded elliptic curve point with point compression.
pub type CompressedPoint<C> = GenericArray<u8, CompressedPointSize<C>>;

/// Size of a compressed elliptic curve point.
pub type CompressedPointSize<C> = <FieldSize<C> as ModulusSize>::CompressedPointSize;

/// Encoded elliptic curve point sized appropriately for a given curve.
pub type EncodedPoint<C> = sec1::point::EncodedPoint<FieldSize<C>>;

/// Encoded elliptic curve point *without* point compression.
pub type UncompressedPoint<C> = GenericArray<u8, UncompressedPointSize<C>>;

/// Size of an uncompressed elliptic curve point.
pub type UncompressedPointSize<C> = <FieldSize<C> as ModulusSize>::UncompressedPointSize;

/// Trait for deserializing a value from a SEC1 encoded curve point.
///
/// This is intended for use with the `AffinePoint` type for a given elliptic curve.
pub trait FromEncodedPoint<C>
where
    Self: Sized,
    C: Curve,
    FieldSize<C>: ModulusSize,
{
    /// Deserialize the type this trait is impl'd on from an [`EncodedPoint`].
    fn from_encoded_point(point: &EncodedPoint<C>) -> CtOption<Self>;
}

/// Trait for serializing a value to a SEC1 encoded curve point.
///
/// This is intended for use with the `AffinePoint` type for a given elliptic curve.
pub trait ToEncodedPoint<C>
where
    C: Curve,
    FieldSize<C>: ModulusSize,
{
    /// Serialize this value as a SEC1 [`EncodedPoint`], optionally applying
    /// point compression.
    fn to_encoded_point(&self, compress: bool) -> EncodedPoint<C>;
}

/// Trait for serializing a value to a SEC1 encoded curve point with compaction.
///
/// This is intended for use with the `AffinePoint` type for a given elliptic curve.
pub trait ToCompactEncodedPoint<C>
where
    C: Curve,
    FieldSize<C>: ModulusSize,
{
    /// Serialize this value as a SEC1 [`EncodedPoint`], optionally applying
    /// point compression.
    fn to_compact_encoded_point(&self) -> CtOption<EncodedPoint<C>>;
}

/// Validate that the given [`EncodedPoint`] represents the encoded public key
/// value of the given secret.
///
/// Curve implementations which also impl [`ProjectiveArithmetic`] will receive
/// a blanket default impl of this trait.
pub trait ValidatePublicKey
where
    Self: Curve,
    FieldSize<Self>: ModulusSize,
{
    /// Validate that the given [`EncodedPoint`] is a valid public key for the
    /// provided secret value.
    #[allow(unused_variables)]
    fn validate_public_key(
        secret_key: &SecretKey<Self>,
        public_key: &EncodedPoint<Self>,
    ) -> Result<()> {
        // Provide a default "always succeeds" implementation.
        // This is the intended default for curve implementations which
        // do not provide an arithmetic implementation, since they have no
        // way to verify this.
        //
        // Implementations with an arithmetic impl will receive a blanket impl
        // of this trait.
        Ok(())
    }
}

#[cfg(all(feature = "arithmetic"))]
impl<C> ValidatePublicKey for C
where
    C: Curve + ProjectiveArithmetic,
    AffinePoint<C>: FromEncodedPoint<C> + ToEncodedPoint<C>,
    FieldSize<C>: ModulusSize,
{
    fn validate_public_key(secret_key: &SecretKey<C>, public_key: &EncodedPoint<C>) -> Result<()> {
        let pk = secret_key
            .public_key()
            .to_encoded_point(public_key.is_compressed());

        if public_key == &pk {
            Ok(())
        } else {
            Err(Error)
        }
    }
}
