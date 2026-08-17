//! All matrix multiplication in this module is in row-vector notation,
//! i.e. a vector `v` is transformed with `v * T`, and if you want to apply `T1`
//! before `T2` you use `T1 * T2`

use crate::approxeq::ApproxEq;
use crate::trig::Trig;
use crate::{Rotation3D, Transform3D, UnknownUnit, Vector3D};

use core::{fmt, hash};

#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
use num_traits::real::Real;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A rigid transformation. All lengths are preserved under such a transformation.
///
///
/// Internally, this is a rotation and a translation, with the rotation
/// applied first (i.e. `Rotation * Translation`, in row-vector notation)
///
/// This can be more efficient to use over full matrices, especially if you
/// have to deal with the decomposed quantities often.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct RigidTransform3D<T, Src, Dst> {
    pub rotation: Rotation3D<T, Src, Dst>,
    pub translation: Vector3D<T, Dst>,
}

impl<T, Src, Dst> RigidTransform3D<T, Src, Dst> {
    /// Construct a new rigid transformation, where the `rotation` applies first
    #[inline]
    pub const fn new(rotation: Rotation3D<T, Src, Dst>, translation: Vector3D<T, Dst>) -> Self {
        Self {
            rotation,
            translation,
        }
    }
}

impl<T: Copy, Src, Dst> RigidTransform3D<T, Src, Dst> {
    pub fn cast_unit<Src2, Dst2>(&self) -> RigidTransform3D<T, Src2, Dst2> {
        RigidTransform3D {
            rotation: self.rotation.cast_unit(),
            translation: self.translation.cast_unit(),
        }
    }
}

impl<T: Real + ApproxEq<T>, Src, Dst> RigidTransform3D<T, Src, Dst> {
    /// Construct an identity transform
    #[inline]
    pub fn identity() -> Self {
        Self {
            rotation: Rotation3D::identity(),
            translation: Vector3D::zero(),
        }
    }

    /// Construct a new rigid transformation, where the `translation` applies first
    #[inline]
    pub fn new_from_reversed(
        translation: Vector3D<T, Src>,
        rotation: Rotation3D<T, Src, Dst>,
    ) -> Self {
        // T * R
        //   = (R * R^-1) * T * R
        //   = R * (R^-1 * T * R)
        //   = R * T'
        //
        // T' = (R^-1 * T * R) is also a translation matrix
        // It is equivalent to the translation matrix obtained by rotating the
        // translation by R

        let translation = rotation.transform_vector3d(translation);
        Self {
            rotation,
            translation,
        }
    }

    #[inline]
    pub fn from_rotation(rotation: Rotation3D<T, Src, Dst>) -> Self {
        Self {
            rotation,
            translation: Vector3D::zero(),
        }
    }

    #[inline]
    pub fn from_translation(translation: Vector3D<T, Dst>) -> Self {
        Self {
            translation,
            rotation: Rotation3D::identity(),
        }
    }

    /// Decompose this into a translation and an rotation to be applied in the opposite order
    ///
    /// i.e., the translation is applied _first_
    #[inline]
    pub fn decompose_reversed(&self) -> (Vector3D<T, Src>, Rotation3D<T, Src, Dst>) {
        // self = R * T
        //      = R * T * (R^-1 * R)
        //      = (R * T * R^-1) * R)
        //      = T' * R
        //
        // T' = (R^ * T * R^-1) is T rotated by R^-1

        let translation = self.rotation.inverse().transform_vector3d(self.translation);
        (translation, self.rotation)
    }

    /// Returns the multiplication of the two transforms such that
    /// other's transformation applies after self's transformation.
    ///
    /// i.e., this produces `self * other` in row-vector notation
    #[inline]
    pub fn then<Dst2>(
        &self,
        other: &RigidTransform3D<T, Dst, Dst2>,
    ) -> RigidTransform3D<T, Src, Dst2> {
        // self = R1 * T1
        // other = R2 * T2
        // result = R1 * T1 * R2 * T2
        //        = R1 * (R2 * R2^-1) * T1 * R2 * T2
        //        = (R1 * R2) * (R2^-1 * T1 * R2) * T2
        //        = R' * T' * T2
        //        = R' * T''
        //
        // (R2^-1 * T2 * R2^) = T' = T2 rotated by R2
        // R1 * R2  = R'
        // T' * T2 = T'' = vector addition of translations T2 and T'

        let t_prime = other.rotation.transform_vector3d(self.translation);
        let r_prime = self.rotation.then(&other.rotation);
        let t_prime2 = t_prime + other.translation;
        RigidTransform3D {
            rotation: r_prime,
            translation: t_prime2,
        }
    }

    /// Inverts the transformation
    #[inline]
    pub fn inverse(&self) -> RigidTransform3D<T, Dst, Src> {
        // result = (self)^-1
        //        = (R * T)^-1
        //        = T^-1 * R^-1
        //        = (R^-1 * R) * T^-1 * R^-1
        //        = R^-1 * (R * T^-1 * R^-1)
        //        = R' * T'
        //
        // T' = (R * T^-1 * R^-1) = (-T) rotated by R^-1
        // R' = R^-1
        //
        // An easier way of writing this is to use new_from_reversed() with R^-1 and T^-1

        RigidTransform3D::new_from_reversed(-self.translation, self.rotation.inverse())
    }

    pub fn to_transform(&self) -> Transform3D<T, Src, Dst>
    where
        T: Trig,
    {
        self.rotation
            .to_transform()
            .then(&self.translation.to_transform())
    }

    /// Drop the units, preserving only the numeric value.
    #[inline]
    pub fn to_untyped(&self) -> RigidTransform3D<T, UnknownUnit, UnknownUnit> {
        RigidTransform3D {
            rotation: self.rotation.to_untyped(),
            translation: self.translation.to_untyped(),
        }
    }

    /// Tag a unitless value with units.
    #[inline]
    pub fn from_untyped(transform: &RigidTransform3D<T, UnknownUnit, UnknownUnit>) -> Self {
        RigidTransform3D {
            rotation: Rotation3D::from_untyped(&transform.rotation),
            translation: Vector3D::from_untyped(transform.translation),
        }
    }
}

impl<T: fmt::Debug, Src, Dst> fmt::Debug for RigidTransform3D<T, Src, Dst> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RigidTransform3D")
            .field("rotation", &self.rotation)
            .field("translation", &self.translation)
            .finish()
    }
}

impl<T: PartialEq, Src, Dst> PartialEq for RigidTransform3D<T, Src, Dst> {
    fn eq(&self, other: &Self) -> bool {
        self.rotation == other.rotation && self.translation == other.translation
    }
}
impl<T: Eq, Src, Dst> Eq for RigidTransform3D<T, Src, Dst> {}

impl<T: hash::Hash, Src, Dst> hash::Hash for RigidTransform3D<T, Src, Dst> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.rotation.hash(state);
        self.translation.hash(state);
    }
}

impl<T: Copy, Src, Dst> Copy for RigidTransform3D<T, Src, Dst> {}

impl<T: Clone, Src, Dst> Clone for RigidTransform3D<T, Src, Dst> {
    fn clone(&self) -> Self {
        RigidTransform3D {
            rotation: self.rotation.clone(),
            translation: self.translation.clone(),
        }
    }
}

#[cfg(feature = "arbitrary")]
impl<'a, T, Src, Dst> arbitrary::Arbitrary<'a> for RigidTransform3D<T, Src, Dst>
where
    T: arbitrary::Arbitrary<'a>,
{
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        Ok(RigidTransform3D {
            rotation: arbitrary::Arbitrary::arbitrary(u)?,
            translation: arbitrary::Arbitrary::arbitrary(u)?,
        })
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<T: Zeroable, Src, Dst> Zeroable for RigidTransform3D<T, Src, Dst> {}

#[cfg(feature = "bytemuck")]
unsafe impl<T: Pod, Src: 'static, Dst: 'static> Pod for RigidTransform3D<T, Src, Dst> {}

impl<T: Real + ApproxEq<T>, Src, Dst> From<Rotation3D<T, Src, Dst>>
    for RigidTransform3D<T, Src, Dst>
{
    fn from(rot: Rotation3D<T, Src, Dst>) -> Self {
        Self::from_rotation(rot)
    }
}

impl<T: Real + ApproxEq<T>, Src, Dst> From<Vector3D<T, Dst>> for RigidTransform3D<T, Src, Dst> {
    fn from(t: Vector3D<T, Dst>) -> Self {
        Self::from_translation(t)
    }
}

#[cfg(test)]
mod test {
    use super::RigidTransform3D;
    use crate::default::{Rotation3D, Transform3D, Vector3D};

    #[test]
    fn test_rigid_construction() {
        let translation = Vector3D::new(12.1, 17.8, -5.5);
        let rotation = Rotation3D::unit_quaternion(0.5, -7.8, 2.2, 4.3);

        let rigid = RigidTransform3D::new(rotation, translation);
        assert!(rigid
            .to_transform()
            .approx_eq(&rotation.to_transform().then(&translation.to_transform())));

        let rigid = RigidTransform3D::new_from_reversed(translation, rotation);
        assert!(rigid
            .to_transform()
            .approx_eq(&translation.to_transform().then(&rotation.to_transform())));
    }

    #[test]
    fn test_rigid_decomposition() {
        let translation = Vector3D::new(12.1, 17.8, -5.5);
        let rotation = Rotation3D::unit_quaternion(0.5, -7.8, 2.2, 4.3);

        let rigid = RigidTransform3D::new(rotation, translation);
        let (t2, r2) = rigid.decompose_reversed();
        assert!(rigid
            .to_transform()
            .approx_eq(&t2.to_transform().then(&r2.to_transform())));
    }

    #[test]
    fn test_rigid_inverse() {
        let translation = Vector3D::new(12.1, 17.8, -5.5);
        let rotation = Rotation3D::unit_quaternion(0.5, -7.8, 2.2, 4.3);

        let rigid = RigidTransform3D::new(rotation, translation);
        let inverse = rigid.inverse();
        assert!(rigid
            .then(&inverse)
            .to_transform()
            .approx_eq(&Transform3D::identity()));
        assert!(inverse
            .to_transform()
            .approx_eq(&rigid.to_transform().inverse().unwrap()));
    }

    #[test]
    fn test_rigid_multiply() {
        let translation = Vector3D::new(12.1, 17.8, -5.5);
        let rotation = Rotation3D::unit_quaternion(0.5, -7.8, 2.2, 4.3);
        let translation2 = Vector3D::new(9.3, -3.9, 1.1);
        let rotation2 = Rotation3D::unit_quaternion(0.1, 0.2, 0.3, -0.4);
        let rigid = RigidTransform3D::new(rotation, translation);
        let rigid2 = RigidTransform3D::new(rotation2, translation2);

        assert!(rigid
            .then(&rigid2)
            .to_transform()
            .approx_eq(&rigid.to_transform().then(&rigid2.to_transform())));
        assert!(rigid2
            .then(&rigid)
            .to_transform()
            .approx_eq(&rigid2.to_transform().then(&rigid.to_transform())));
    }
}
