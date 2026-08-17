use num::One;

use simba::scalar::RealField;

use crate::base::allocator::Allocator;
use crate::base::dimension::{DimNameAdd, DimNameSum, U1};
use crate::base::{Const, DefaultAllocator, OMatrix};

use crate::geometry::{TCategory, Transform};

impl<T: RealField, C: TCategory, const D: usize> Default for Transform<T, C, D>
where
    Const<D>: DimNameAdd<U1>,
    DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>,
{
    fn default() -> Self {
        Self::identity()
    }
}

impl<T: RealField, C: TCategory, const D: usize> Transform<T, C, D>
where
    Const<D>: DimNameAdd<U1>,
    DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>,
{
    /// Creates a new identity transform.
    ///
    /// # Example
    ///
    /// ```
    /// # use nalgebra::{Transform2, Projective2, Affine2, Transform3, Projective3, Affine3, Point2, Point3};
    ///
    /// let pt = Point2::new(1.0, 2.0);
    /// let t = Projective2::identity();
    /// assert_eq!(t * pt, pt);
    ///
    /// let aff = Affine2::identity();
    /// assert_eq!(aff * pt, pt);
    ///
    /// let aff = Transform2::identity();
    /// assert_eq!(aff * pt, pt);
    ///
    /// // Also works in 3D.
    /// let pt = Point3::new(1.0, 2.0, 3.0);
    /// let t = Projective3::identity();
    /// assert_eq!(t * pt, pt);
    ///
    /// let aff = Affine3::identity();
    /// assert_eq!(aff * pt, pt);
    ///
    /// let aff = Transform3::identity();
    /// assert_eq!(aff * pt, pt);
    /// ```
    #[inline]
    pub fn identity() -> Self {
        Self::from_matrix_unchecked(OMatrix::<
            _,
            DimNameSum<Const<D>, U1>,
            DimNameSum<Const<D>, U1>,
        >::identity())
    }
}

impl<T: RealField, C: TCategory, const D: usize> One for Transform<T, C, D>
where
    Const<D>: DimNameAdd<U1>,
    DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>,
{
    /// Creates a new identity transform.
    #[inline]
    fn one() -> Self {
        Self::identity()
    }
}
