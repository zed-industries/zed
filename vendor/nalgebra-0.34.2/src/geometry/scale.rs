// Needed otherwise the rkyv macros generate code incompatible with rust-2024
#![cfg_attr(feature = "rkyv-serialize", allow(unsafe_op_in_unsafe_fn))]

use approx::{AbsDiffEq, RelativeEq, UlpsEq};
use num::{One, Zero};
use std::fmt;
use std::hash;

#[cfg(feature = "serde-serialize-no-std")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::ClosedDivAssign;
use crate::ClosedMulAssign;
use crate::base::allocator::Allocator;
use crate::base::dimension::{DimNameAdd, DimNameSum, U1};
use crate::base::storage::Owned;
use crate::base::{Const, DefaultAllocator, OMatrix, OVector, SVector, Scalar};

use crate::geometry::Point;

#[cfg(feature = "rkyv-serialize")]
use rkyv::bytecheck;

/// A scale which supports non-uniform scaling.
#[repr(C)]
#[cfg_attr(
    feature = "rkyv-serialize-no-std",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize),
    archive(
        as = "Scale<T::Archived, D>",
        bound(archive = "
        T: rkyv::Archive,
        SVector<T, D>: rkyv::Archive<Archived = SVector<T::Archived, D>>
    ")
    )
)]
#[cfg_attr(feature = "rkyv-serialize", derive(bytecheck::CheckBytes))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone)]
pub struct Scale<T, const D: usize> {
    /// The scale coordinates, i.e., how much is multiplied to a point's coordinates when it is
    /// scaled.
    pub vector: SVector<T, D>,
}

impl<T: fmt::Debug, const D: usize> fmt::Debug for Scale<T, D> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        self.vector.as_slice().fmt(formatter)
    }
}

impl<T: Scalar + hash::Hash, const D: usize> hash::Hash for Scale<T, D>
where
    Owned<T, Const<D>>: hash::Hash,
{
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.vector.hash(state)
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<T, const D: usize> bytemuck::Zeroable for Scale<T, D>
where
    T: Scalar + bytemuck::Zeroable,
    SVector<T, D>: bytemuck::Zeroable,
{
}

#[cfg(feature = "bytemuck")]
unsafe impl<T, const D: usize> bytemuck::Pod for Scale<T, D>
where
    T: Scalar + bytemuck::Pod,
    SVector<T, D>: bytemuck::Pod,
{
}

#[cfg(feature = "serde-serialize-no-std")]
impl<T: Scalar, const D: usize> Serialize for Scale<T, D>
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
impl<'a, T: Scalar, const D: usize> Deserialize<'a> for Scale<T, D>
where
    Owned<T, Const<D>>: Deserialize<'a>,
{
    fn deserialize<Des>(deserializer: Des) -> Result<Self, Des::Error>
    where
        Des: Deserializer<'a>,
    {
        let matrix = SVector::<T, D>::deserialize(deserializer)?;

        Ok(Scale::from(matrix))
    }
}

impl<T: Scalar, const D: usize> Scale<T, D> {
    /// Inverts `self`.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::{Scale2, Scale3};
    /// let t = Scale3::new(1.0, 2.0, 3.0);
    /// assert_eq!(t * t.try_inverse().unwrap(), Scale3::identity());
    /// assert_eq!(t.try_inverse().unwrap() * t, Scale3::identity());
    ///
    /// // Work in all dimensions.
    /// let t = Scale2::new(1.0, 2.0);
    /// assert_eq!(t * t.try_inverse().unwrap(), Scale2::identity());
    /// assert_eq!(t.try_inverse().unwrap() * t, Scale2::identity());
    ///
    /// // Returns None if any coordinate is 0.
    /// let t = Scale2::new(0.0, 2.0);
    /// assert_eq!(t.try_inverse(), None);
    /// ```
    #[inline]
    #[must_use = "Did you mean to use try_inverse_mut()?"]
    pub fn try_inverse(&self) -> Option<Scale<T, D>>
    where
        T: ClosedDivAssign + One + Zero,
    {
        for i in 0..D {
            if self.vector[i] == T::zero() {
                return None;
            }
        }
        Some(self.vector.map(|e| T::one() / e).into())
    }

    /// Inverts `self`.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::{Scale2, Scale3};
    ///
    /// unsafe {
    ///     let t = Scale3::new(1.0, 2.0, 3.0);
    ///     assert_eq!(t * t.inverse_unchecked(), Scale3::identity());
    ///     assert_eq!(t.inverse_unchecked() * t, Scale3::identity());
    ///
    ///     // Work in all dimensions.
    ///     let t = Scale2::new(1.0, 2.0);
    ///     assert_eq!(t * t.inverse_unchecked(), Scale2::identity());
    ///     assert_eq!(t.inverse_unchecked() * t, Scale2::identity());
    /// }
    /// ```
    ///
    /// # Safety
    ///
    /// Should only be used if all scaling is known to be non-zero.
    #[inline]
    #[must_use]
    pub unsafe fn inverse_unchecked(&self) -> Scale<T, D>
    where
        T: ClosedDivAssign + One,
    {
        self.vector.map(|e| T::one() / e).into()
    }

    /// Inverts `self`.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::{Scale2, Scale3};
    /// let t = Scale3::new(1.0, 2.0, 3.0);
    /// assert_eq!(t * t.pseudo_inverse(), Scale3::identity());
    /// assert_eq!(t.pseudo_inverse() * t, Scale3::identity());
    ///
    /// // Work in all dimensions.
    /// let t = Scale2::new(1.0, 2.0);
    /// assert_eq!(t * t.pseudo_inverse(), Scale2::identity());
    /// assert_eq!(t.pseudo_inverse() * t, Scale2::identity());
    ///
    /// // Inverts only non-zero coordinates.
    /// let t = Scale2::new(0.0, 2.0);
    /// assert_eq!(t * t.pseudo_inverse(), Scale2::new(0.0, 1.0));
    /// assert_eq!(t.pseudo_inverse() * t, Scale2::new(0.0, 1.0));
    /// ```
    #[inline]
    #[must_use]
    pub fn pseudo_inverse(&self) -> Scale<T, D>
    where
        T: ClosedDivAssign + One + Zero,
    {
        self.vector
            .map(|e| {
                if e != T::zero() {
                    T::one() / e
                } else {
                    T::zero()
                }
            })
            .into()
    }

    /// Converts this Scale into its equivalent homogeneous transformation matrix.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::{Scale2, Scale3, Matrix3, Matrix4};
    /// let t = Scale3::new(10.0, 20.0, 30.0);
    /// let expected = Matrix4::new(10.0, 0.0, 0.0, 0.0,
    ///                             0.0, 20.0, 0.0, 0.0,
    ///                             0.0, 0.0, 30.0, 0.0,
    ///                             0.0, 0.0, 0.0, 1.0);
    /// assert_eq!(t.to_homogeneous(), expected);
    ///
    /// let t = Scale2::new(10.0, 20.0);
    /// let expected = Matrix3::new(10.0, 0.0, 0.0,
    ///                             0.0, 20.0, 0.0,
    ///                             0.0, 0.0, 1.0);
    /// assert_eq!(t.to_homogeneous(), expected);
    /// ```
    #[inline]
    #[must_use]
    pub fn to_homogeneous(&self) -> OMatrix<T, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>
    where
        T: Zero + One + Clone,
        Const<D>: DimNameAdd<U1>,
        DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>
            + Allocator<DimNameSum<Const<D>, U1>, U1>,
    {
        // TODO: use self.vector.push() instead. We can’t right now because
        //       that would require the DimAdd bound (but here we use DimNameAdd).
        //       This should be fixable once Rust gets a more complete support of
        //       const-generics.
        let mut v = OVector::from_element(T::one());
        for i in 0..D {
            v[i] = self.vector[i].clone();
        }
        OMatrix::from_diagonal(&v)
    }

    /// Inverts `self` in-place.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::{Scale2, Scale3};
    /// let t = Scale3::new(1.0, 2.0, 3.0);
    /// let mut inv_t = Scale3::new(1.0, 2.0, 3.0);
    /// assert!(inv_t.try_inverse_mut());
    /// assert_eq!(t * inv_t, Scale3::identity());
    /// assert_eq!(inv_t * t, Scale3::identity());
    ///
    /// // Work in all dimensions.
    /// let t = Scale2::new(1.0, 2.0);
    /// let mut inv_t = Scale2::new(1.0, 2.0);
    /// assert!(inv_t.try_inverse_mut());
    /// assert_eq!(t * inv_t, Scale2::identity());
    /// assert_eq!(inv_t * t, Scale2::identity());
    ///
    /// // Does not perform any operation if a coordinate is 0.
    /// let mut t = Scale2::new(0.0, 2.0);
    /// assert!(!t.try_inverse_mut());
    /// ```
    #[inline]
    pub fn try_inverse_mut(&mut self) -> bool
    where
        T: ClosedDivAssign + One + Zero,
    {
        match self.try_inverse() {
            Some(v) => {
                self.vector = v.vector;
                true
            }
            None => false,
        }
    }
}

impl<T: Scalar + ClosedMulAssign, const D: usize> Scale<T, D> {
    /// Translate the given point.
    ///
    /// This is the same as the multiplication `self * pt`.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::{Scale3, Point3};
    /// let t = Scale3::new(1.0, 2.0, 3.0);
    /// let transformed_point = t.transform_point(&Point3::new(4.0, 5.0, 6.0));
    /// assert_eq!(transformed_point, Point3::new(4.0, 10.0, 18.0));
    /// ```
    #[inline]
    #[must_use]
    pub fn transform_point(&self, pt: &Point<T, D>) -> Point<T, D> {
        self * pt
    }
}

impl<T: Scalar + ClosedDivAssign + ClosedMulAssign + One + Zero, const D: usize> Scale<T, D> {
    /// Translate the given point by the inverse of this Scale.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::{Scale3, Point3};
    /// let t = Scale3::new(1.0, 2.0, 3.0);
    /// let transformed_point = t.try_inverse_transform_point(&Point3::new(4.0, 6.0, 6.0)).unwrap();
    /// assert_eq!(transformed_point, Point3::new(4.0, 3.0, 2.0));
    ///
    /// // Returns None if the inverse doesn't exist.
    /// let t = Scale3::new(1.0, 0.0, 3.0);
    /// let transformed_point = t.try_inverse_transform_point(&Point3::new(4.0, 6.0, 6.0));
    /// assert_eq!(transformed_point, None);
    /// ```
    #[inline]
    #[must_use]
    pub fn try_inverse_transform_point(&self, pt: &Point<T, D>) -> Option<Point<T, D>> {
        self.try_inverse().map(|s| s * pt)
    }
}

impl<T: Scalar + Eq, const D: usize> Eq for Scale<T, D> {}

impl<T: Scalar + PartialEq, const D: usize> PartialEq for Scale<T, D> {
    #[inline]
    fn eq(&self, right: &Scale<T, D>) -> bool {
        self.vector == right.vector
    }
}

impl<T: Scalar + AbsDiffEq, const D: usize> AbsDiffEq for Scale<T, D>
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

impl<T: Scalar + RelativeEq, const D: usize> RelativeEq for Scale<T, D>
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

impl<T: Scalar + UlpsEq, const D: usize> UlpsEq for Scale<T, D>
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
impl<T: Scalar + fmt::Display, const D: usize> fmt::Display for Scale<T, D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let precision = f.precision().unwrap_or(3);

        writeln!(f, "Scale {{")?;
        write!(f, "{:.*}", precision, self.vector)?;
        writeln!(f, "}}")
    }
}
