// Needed otherwise the rkyv macros generate code incompatible with rust-2024
#![cfg_attr(feature = "rkyv-serialize", allow(unsafe_op_in_unsafe_fn))]

use approx::{AbsDiffEq, RelativeEq, UlpsEq};
use num::{One, Zero};
use std::fmt;
use std::hash;

#[cfg(feature = "serde-serialize-no-std")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use simba::scalar::{ClosedAddAssign, ClosedNeg, ClosedSubAssign};

use crate::base::allocator::Allocator;
use crate::base::dimension::{DimNameAdd, DimNameSum, U1};
use crate::base::storage::Owned;
use crate::base::{Const, DefaultAllocator, OMatrix, SVector, Scalar};

use crate::geometry::Point;

#[cfg(feature = "rkyv-serialize")]
use rkyv::bytecheck;

/// A translation.
#[repr(C)]
#[cfg_attr(
    feature = "rkyv-serialize-no-std",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize),
    archive(
        as = "Translation<T::Archived, D>",
        bound(archive = "
        T: rkyv::Archive,
        SVector<T, D>: rkyv::Archive<Archived = SVector<T::Archived, D>>
    ")
    )
)]
#[cfg_attr(feature = "rkyv-serialize", derive(bytecheck::CheckBytes))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone)]
pub struct Translation<T, const D: usize> {
    /// The translation coordinates, i.e., how much is added to a point's coordinates when it is
    /// translated.
    pub vector: SVector<T, D>,
}

impl<T: fmt::Debug, const D: usize> fmt::Debug for Translation<T, D> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        self.vector.as_slice().fmt(formatter)
    }
}

impl<T: Scalar + hash::Hash, const D: usize> hash::Hash for Translation<T, D>
where
    Owned<T, Const<D>>: hash::Hash,
{
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.vector.hash(state)
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<T, const D: usize> bytemuck::Zeroable for Translation<T, D>
where
    T: Scalar + bytemuck::Zeroable,
    SVector<T, D>: bytemuck::Zeroable,
{
}

#[cfg(feature = "bytemuck")]
unsafe impl<T, const D: usize> bytemuck::Pod for Translation<T, D>
where
    T: Scalar + bytemuck::Pod,
    SVector<T, D>: bytemuck::Pod,
{
}

#[cfg(feature = "serde-serialize-no-std")]
impl<T: Scalar, const D: usize> Serialize for Translation<T, D>
where
    Owned<T, Const<D>>: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.vector.serialize(serializer)
    }
}

#[cfg(feature = "serde-serialize-no-std")]
impl<'a, T: Scalar, const D: usize> Deserialize<'a> for Translation<T, D>
where
    Owned<T, Const<D>>: Deserialize<'a>,
{
    fn deserialize<Des>(deserializer: Des) -> Result<Self, Des::Error>
    where
        Des: Deserializer<'a>,
    {
        let matrix = SVector::<T, D>::deserialize(deserializer)?;

        Ok(Translation::from(matrix))
    }
}

impl<T: Scalar, const D: usize> Translation<T, D> {
    /// Creates a new translation from the given vector.
    #[inline]
    #[deprecated(note = "Use `::from` instead.")]
    pub const fn from_vector(vector: SVector<T, D>) -> Translation<T, D> {
        Translation { vector }
    }

    /// Inverts `self`.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::{Translation2, Translation3};
    /// let t = Translation3::new(1.0, 2.0, 3.0);
    /// assert_eq!(t * t.inverse(), Translation3::identity());
    /// assert_eq!(t.inverse() * t, Translation3::identity());
    ///
    /// // Work in all dimensions.
    /// let t = Translation2::new(1.0, 2.0);
    /// assert_eq!(t * t.inverse(), Translation2::identity());
    /// assert_eq!(t.inverse() * t, Translation2::identity());
    /// ```
    #[inline]
    #[must_use = "Did you mean to use inverse_mut()?"]
    pub fn inverse(&self) -> Translation<T, D>
    where
        T: ClosedNeg,
    {
        Translation::from(-&self.vector)
    }

    /// Converts this translation into its equivalent homogeneous transformation matrix.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::{Translation2, Translation3, Matrix3, Matrix4};
    /// let t = Translation3::new(10.0, 20.0, 30.0);
    /// let expected = Matrix4::new(1.0, 0.0, 0.0, 10.0,
    ///                             0.0, 1.0, 0.0, 20.0,
    ///                             0.0, 0.0, 1.0, 30.0,
    ///                             0.0, 0.0, 0.0, 1.0);
    /// assert_eq!(t.to_homogeneous(), expected);
    ///
    /// let t = Translation2::new(10.0, 20.0);
    /// let expected = Matrix3::new(1.0, 0.0, 10.0,
    ///                             0.0, 1.0, 20.0,
    ///                             0.0, 0.0, 1.0);
    /// assert_eq!(t.to_homogeneous(), expected);
    /// ```
    #[inline]
    #[must_use]
    pub fn to_homogeneous(&self) -> OMatrix<T, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>
    where
        T: Zero + One,
        Const<D>: DimNameAdd<U1>,
        DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>,
    {
        let mut res = OMatrix::<T, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>::identity();
        res.fixed_view_mut::<D, 1>(0, D).copy_from(&self.vector);

        res
    }

    /// Inverts `self` in-place.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::{Translation2, Translation3};
    /// let t = Translation3::new(1.0, 2.0, 3.0);
    /// let mut inv_t = Translation3::new(1.0, 2.0, 3.0);
    /// inv_t.inverse_mut();
    /// assert_eq!(t * inv_t, Translation3::identity());
    /// assert_eq!(inv_t * t, Translation3::identity());
    ///
    /// // Work in all dimensions.
    /// let t = Translation2::new(1.0, 2.0);
    /// let mut inv_t = Translation2::new(1.0, 2.0);
    /// inv_t.inverse_mut();
    /// assert_eq!(t * inv_t, Translation2::identity());
    /// assert_eq!(inv_t * t, Translation2::identity());
    /// ```
    #[inline]
    pub fn inverse_mut(&mut self)
    where
        T: ClosedNeg,
    {
        self.vector.neg_mut()
    }
}

impl<T: Scalar + ClosedAddAssign, const D: usize> Translation<T, D> {
    /// Translate the given point.
    ///
    /// This is the same as the multiplication `self * pt`.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::{Translation3, Point3};
    /// let t = Translation3::new(1.0, 2.0, 3.0);
    /// let transformed_point = t.transform_point(&Point3::new(4.0, 5.0, 6.0));
    /// assert_eq!(transformed_point, Point3::new(5.0, 7.0, 9.0));
    /// ```
    #[inline]
    #[must_use]
    pub fn transform_point(&self, pt: &Point<T, D>) -> Point<T, D> {
        pt + &self.vector
    }
}

impl<T: Scalar + ClosedSubAssign, const D: usize> Translation<T, D> {
    /// Translate the given point by the inverse of this translation.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::{Translation3, Point3};
    /// let t = Translation3::new(1.0, 2.0, 3.0);
    /// let transformed_point = t.inverse_transform_point(&Point3::new(4.0, 5.0, 6.0));
    /// assert_eq!(transformed_point, Point3::new(3.0, 3.0, 3.0));
    /// ```
    #[inline]
    #[must_use]
    pub fn inverse_transform_point(&self, pt: &Point<T, D>) -> Point<T, D> {
        pt - &self.vector
    }
}

impl<T: Scalar + Eq, const D: usize> Eq for Translation<T, D> {}

impl<T: Scalar + PartialEq, const D: usize> PartialEq for Translation<T, D> {
    #[inline]
    fn eq(&self, right: &Translation<T, D>) -> bool {
        self.vector == right.vector
    }
}

impl<T: Scalar + AbsDiffEq, const D: usize> AbsDiffEq for Translation<T, D>
where
    T::Epsilon: Clone,
{
    type Epsilon = T::Epsilon;

    #[inline]
    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.vector.abs_diff_eq(&other.vector, epsilon)
    }
}

impl<T: Scalar + RelativeEq, const D: usize> RelativeEq for Translation<T, D>
where
    T::Epsilon: Clone,
{
    #[inline]
    fn default_max_relative() -> Self::Epsilon {
        T::default_max_relative()
    }

    #[inline]
    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.vector
            .relative_eq(&other.vector, epsilon, max_relative)
    }
}

impl<T: Scalar + UlpsEq, const D: usize> UlpsEq for Translation<T, D>
where
    T::Epsilon: Clone,
{
    #[inline]
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    #[inline]
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        self.vector.ulps_eq(&other.vector, epsilon, max_ulps)
    }
}

/*
 *
 * Display
 *
 */
impl<T: Scalar + fmt::Display, const D: usize> fmt::Display for Translation<T, D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let precision = f.precision().unwrap_or(3);

        writeln!(f, "Translation {{")?;
        write!(f, "{:.*}", precision, self.vector)?;
        writeln!(f, "}}")
    }
}
