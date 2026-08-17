//! Traits for handling hash to curve.

use super::{hash_to_field, ExpandMsg, FromOkm, MapToCurve};
use crate::{ProjectiveArithmetic, ProjectivePoint, Result};
use group::cofactor::CofactorGroup;

/// Adds hashing arbitrary byte sequences to a valid group element
pub trait GroupDigest: ProjectiveArithmetic
where
    ProjectivePoint<Self>: CofactorGroup,
{
    /// The field element representation for a group value with multiple elements
    type FieldElement: FromOkm + MapToCurve<Output = ProjectivePoint<Self>> + Default + Copy;

    /// Computes the hash to curve routine.
    ///
    /// From <https://www.ietf.org/archive/id/draft-irtf-cfrg-hash-to-curve-13.html>:
    ///
    /// > Uniform encoding from byte strings to points in G.
    /// > That is, the distribution of its output is statistically close
    /// > to uniform in G.
    /// > This function is suitable for most applications requiring a random
    /// > oracle returning points in G assuming a cryptographically secure
    /// > hash function is used.
    ///
    /// # Examples
    ///
    /// ## Using a fixed size hash function
    ///
    /// ```ignore
    /// let pt = ProjectivePoint::hash_from_bytes::<ExpandMsgXmd<sha2::Sha256>>(b"test data", b"CURVE_XMD:SHA-256_SSWU_RO_");
    /// ```
    ///
    /// ## Using an extendable output function
    ///
    /// ```ignore
    /// let pt = ProjectivePoint::hash_from_bytes::<ExpandMsgXof<sha3::Shake256>>(b"test data", b"CURVE_XOF:SHAKE-256_SSWU_RO_");
    /// ```
    ///
    /// # Errors
    /// See implementors of [`ExpandMsg`] for errors:
    /// - [`ExpandMsgXmd`]
    /// - [`ExpandMsgXof`]
    ///
    /// `len_in_bytes = <Self::FieldElement as FromOkm>::Length * 2`
    ///
    /// [`ExpandMsgXmd`]: crate::hash2curve::ExpandMsgXmd
    /// [`ExpandMsgXof`]: crate::hash2curve::ExpandMsgXof
    fn hash_from_bytes<'a, X: ExpandMsg<'a>>(
        msgs: &[&[u8]],
        dst: &'a [u8],
    ) -> Result<ProjectivePoint<Self>> {
        let mut u = [Self::FieldElement::default(), Self::FieldElement::default()];
        hash_to_field::<X, _>(msgs, dst, &mut u)?;
        let q0 = u[0].map_to_curve();
        let q1 = u[1].map_to_curve();
        // Ideally we could add and then clear cofactor once
        // thus saving a call but the field elements may not
        // add properly due to the underlying implementation
        // which could result in an incorrect subgroup.
        // This is caused curve coefficients being different than
        // what is usually implemented.
        // FieldElement expects the `a` and `b` to be the original values
        // isogenies are different with curves like k256 and bls12-381.
        // This problem doesn't manifest for curves with no isogeny like p256.
        // For k256 and p256 clear_cofactor doesn't do anything anyway so it will be a no-op.
        Ok(q0.clear_cofactor().into() + q1.clear_cofactor())
    }

    /// Computes the encode to curve routine.
    ///
    /// From <https://www.ietf.org/archive/id/draft-irtf-cfrg-hash-to-curve-13.html>:
    ///
    /// > Nonuniform encoding from byte strings to
    /// > points in G. That is, the distribution of its output is not
    /// > uniformly random in G: the set of possible outputs of
    /// > encode_to_curve is only a fraction of the points in G, and some
    /// > points in this set are more likely to be output than others.
    ///
    /// # Errors
    /// See implementors of [`ExpandMsg`] for errors:
    /// - [`ExpandMsgXmd`]
    /// - [`ExpandMsgXof`]
    ///
    /// `len_in_bytes = <Self::FieldElement as FromOkm>::Length`
    ///
    /// [`ExpandMsgXmd`]: crate::hash2curve::ExpandMsgXmd
    /// [`ExpandMsgXof`]: crate::hash2curve::ExpandMsgXof
    fn encode_from_bytes<'a, X: ExpandMsg<'a>>(
        msgs: &[&[u8]],
        dst: &'a [u8],
    ) -> Result<ProjectivePoint<Self>> {
        let mut u = [Self::FieldElement::default()];
        hash_to_field::<X, _>(msgs, dst, &mut u)?;
        let q0 = u[0].map_to_curve();
        Ok(q0.clear_cofactor().into())
    }

    /// Computes the hash to field routine according to
    /// <https://www.ietf.org/archive/id/draft-irtf-cfrg-hash-to-curve-13.html#section-5>
    /// and returns a scalar.
    ///
    /// # Errors
    /// See implementors of [`ExpandMsg`] for errors:
    /// - [`ExpandMsgXmd`]
    /// - [`ExpandMsgXof`]
    ///
    /// `len_in_bytes = <Self::Scalar as FromOkm>::Length`
    ///
    /// [`ExpandMsgXmd`]: crate::hash2curve::ExpandMsgXmd
    /// [`ExpandMsgXof`]: crate::hash2curve::ExpandMsgXof
    fn hash_to_scalar<'a, X: ExpandMsg<'a>>(msgs: &[&[u8]], dst: &'a [u8]) -> Result<Self::Scalar>
    where
        Self::Scalar: FromOkm,
    {
        let mut u = [Self::Scalar::default()];
        hash_to_field::<X, _>(msgs, dst, &mut u)?;
        Ok(u[0])
    }
}
