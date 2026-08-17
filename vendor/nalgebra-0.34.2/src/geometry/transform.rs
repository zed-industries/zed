use approx::{AbsDiffEq, RelativeEq, UlpsEq};
use std::any::Any;
use std::fmt::{self, Debug};
use std::hash;
use std::marker::PhantomData;

#[cfg(feature = "serde-serialize-no-std")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use simba::scalar::RealField;

use crate::base::allocator::Allocator;
use crate::base::dimension::{DimNameAdd, DimNameSum, U1};
use crate::base::storage::Owned;
use crate::base::{Const, DefaultAllocator, DimName, OMatrix, SVector};

use crate::geometry::Point;

/// Trait implemented by phantom types identifying the projective transformation type.
///
/// NOTE: this trait is not intended to be implemented outside of the `nalgebra` crate.
pub trait TCategory: Any + Debug + Copy + PartialEq + Send {
    /// Indicates whether a `Transform` with the category `Self` has a bottom-row different from
    /// `0 0 .. 1`.
    #[inline]
    fn has_normalizer() -> bool {
        true
    }

    /// Checks that the given matrix is a valid homogeneous representation of an element of the
    /// category `Self`.
    fn check_homogeneous_invariants<T: RealField, D: DimName>(mat: &OMatrix<T, D, D>) -> bool
    where
        T::Epsilon: Clone,
        DefaultAllocator: Allocator<D, D>;
}

/// Traits that gives the `Transform` category that is compatible with the result of the
/// multiplication of transformations with categories `Self` and `Other`.
pub trait TCategoryMul<Other: TCategory>: TCategory {
    /// The transform category that results from the multiplication of a `Transform<Self>` to a
    /// `Transform<Other>`. This is usually equal to `Self` or `Other`, whichever is the most
    /// general category.
    type Representative: TCategory;
}

/// Indicates that `Self` is a more general `Transform` category than `Other`.
pub trait SuperTCategoryOf<Other: TCategory>: TCategory {}

/// Indicates that `Self` is a more specific `Transform` category than `Other`.
///
/// Automatically implemented based on `SuperTCategoryOf`.
pub trait SubTCategoryOf<Other: TCategory>: TCategory {}
impl<T1, T2> SubTCategoryOf<T2> for T1
where
    T1: TCategory,
    T2: SuperTCategoryOf<T1>,
{
}

/// Tag representing the most general (not necessarily inversible) `Transform` type.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum TGeneral {}

/// Tag representing the most general inversible `Transform` type.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum TProjective {}

/// Tag representing an affine `Transform`. Its bottom-row is equal to `(0, 0 ... 0, 1)`.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum TAffine {}

impl TCategory for TGeneral {
    #[inline]
    fn check_homogeneous_invariants<T: RealField, D: DimName>(_: &OMatrix<T, D, D>) -> bool
    where
        T::Epsilon: Clone,
        DefaultAllocator: Allocator<D, D>,
    {
        true
    }
}

impl TCategory for TProjective {
    #[inline]
    fn check_homogeneous_invariants<T: RealField, D: DimName>(mat: &OMatrix<T, D, D>) -> bool
    where
        T::Epsilon: Clone,
        DefaultAllocator: Allocator<D, D>,
    {
        mat.is_invertible()
    }
}

impl TCategory for TAffine {
    #[inline]
    fn has_normalizer() -> bool {
        false
    }

    #[inline]
    fn check_homogeneous_invariants<T: RealField, D: DimName>(mat: &OMatrix<T, D, D>) -> bool
    where
        T::Epsilon: Clone,
        DefaultAllocator: Allocator<D, D>,
    {
        let last = D::DIM - 1;
        mat.is_invertible()
            && mat[(last, last)] == T::one()
            && (0..last).all(|i| mat[(last, i)].is_zero())
    }
}

macro_rules! category_mul_impl(
    ($($a: ident * $b: ident => $c: ty);* $(;)*) => {$(
        impl TCategoryMul<$a> for $b {
            type Representative = $c;
        }
    )*}
);

// We require stability upon multiplication.
impl<T: TCategory> TCategoryMul<T> for T {
    type Representative = T;
}

category_mul_impl!(
//  TGeneral * TGeneral    => TGeneral;
    TGeneral * TProjective => TGeneral;
    TGeneral * TAffine     => TGeneral;

    TProjective * TGeneral    => TGeneral;
//  TProjective * TProjective => TProjective;
    TProjective * TAffine     => TProjective;

    TAffine * TGeneral    => TGeneral;
    TAffine * TProjective => TProjective;
//  TAffine * TAffine     => TAffine;
);

macro_rules! super_tcategory_impl(
    ($($a: ident >= $b: ident);* $(;)*) => {$(
        impl SuperTCategoryOf<$b> for $a { }
    )*}
);

impl<T: TCategory> SuperTCategoryOf<T> for T {}

super_tcategory_impl!(
    TGeneral    >= TProjective;
    TGeneral    >= TAffine;
    TProjective >= TAffine;
);

/// A transformation matrix in homogeneous coordinates.
///
/// It is stored as a matrix with dimensions `(D + 1, D + 1)`, e.g., it stores a 4x4 matrix for a
/// 3D transformation.
#[repr(C)]
pub struct Transform<T: RealField, C: TCategory, const D: usize>
where
    Const<D>: DimNameAdd<U1>,
    DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>,
{
    matrix: OMatrix<T, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>,
    _phantom: PhantomData<C>,
}

impl<T: RealField + Debug, C: TCategory, const D: usize> Debug for Transform<T, C, D>
where
    Const<D>: DimNameAdd<U1>,
    DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        self.matrix.fmt(formatter)
    }
}

impl<T: RealField + hash::Hash, C: TCategory, const D: usize> hash::Hash for Transform<T, C, D>
where
    Const<D>: DimNameAdd<U1>,
    DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>,
    Owned<T, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>: hash::Hash,
{
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.matrix.hash(state);
    }
}

impl<T: RealField + Copy, C: TCategory, const D: usize> Copy for Transform<T, C, D>
where
    Const<D>: DimNameAdd<U1>,
    DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>,
    Owned<T, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>: Copy,
{
}

impl<T: RealField, C: TCategory, const D: usize> Clone for Transform<T, C, D>
where
    Const<D>: DimNameAdd<U1>,
    DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>,
{
    #[inline]
    fn clone(&self) -> Self {
        Transform::from_matrix_unchecked(self.matrix.clone())
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<T, C: TCategory, const D: usize> bytemuck::Zeroable for Transform<T, C, D>
where
    T: RealField + bytemuck::Zeroable,
    Const<D>: DimNameAdd<U1>,
    DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>,
    OMatrix<T, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>: bytemuck::Zeroable,
{
}

#[cfg(feature = "bytemuck")]
unsafe impl<T, C: TCategory, const D: usize> bytemuck::Pod for Transform<T, C, D>
where
    T: RealField + bytemuck::Pod,
    Const<D>: DimNameAdd<U1>,
    DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>,
    OMatrix<T, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>: bytemuck::Pod,
    Owned<T, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>: Copy,
{
}

#[cfg(feature = "serde-serialize-no-std")]
impl<T: RealField, C: TCategory, const D: usize> Serialize for Transform<T, C, D>
where
    Const<D>: DimNameAdd<U1>,
    DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>,
    Owned<T, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.matrix.serialize(serializer)
    }
}

#[cfg(feature = "serde-serialize-no-std")]
impl<'a, T: RealField, C: TCategory, const D: usize> Deserialize<'a> for Transform<T, C, D>
where
    Const<D>: DimNameAdd<U1>,
    DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>,
    Owned<T, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>: Deserialize<'a>,
{
    fn deserialize<Des>(deserializer: Des) -> Result<Self, Des::Error>
    where
        Des: Deserializer<'a>,
    {
        let matrix = OMatrix::<T, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>::deserialize(
            deserializer,
        )?;

        Ok(Transform::from_matrix_unchecked(matrix))
    }
}

impl<T: RealField + Eq, C: TCategory, const D: usize> Eq for Transform<T, C, D>
where
    Const<D>: DimNameAdd<U1>,
    DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>,
{
}

impl<T: RealField, C: TCategory, const D: usize> PartialEq for Transform<T, C, D>
where
    Const<D>: DimNameAdd<U1>,
    DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>,
{
    #[inline]
    fn eq(&self, right: &Self) -> bool {
        self.matrix == right.matrix
    }
}

impl<T: RealField, C: TCategory, const D: usize> Transform<T, C, D>
where
    Const<D>: DimNameAdd<U1>,
    DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>,
{
    /// Creates a new transformation from the given homogeneous matrix. The transformation category
    /// of `Self` is not checked to be verified by the given matrix.
    #[inline]
    pub const fn from_matrix_unchecked(
        matrix: OMatrix<T, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>,
    ) -> Self {
        Transform {
            matrix,
            _phantom: PhantomData,
        }
    }

    /// Retrieves the underlying matrix.
    ///
    /// # Examples
    /// ```
    /// # use nalgebra::{Matrix3, Transform2};
    ///
    /// let m = Matrix3::new(1.0, 2.0, 0.0,
    ///                      3.0, 4.0, 0.0,
    ///                      0.0, 0.0, 1.0);
    /// let t = Transform2::from_matrix_unchecked(m);
    /// assert_eq!(t.into_inner(), m);
    /// ```
    #[inline]
    pub fn into_inner(self) -> OMatrix<T, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>> {
        self.matrix
    }

    /// Retrieves the underlying matrix.
    /// Deprecated: Use [`Transform::into_inner`] instead.
    #[deprecated(note = "use `.into_inner()` instead")]
    #[inline]
    pub fn unwrap(self) -> OMatrix<T, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>> {
        self.matrix
    }

    /// A reference to the underlying matrix.
    ///
    /// # Examples
    /// ```
    /// # use nalgebra::{Matrix3, Transform2};
    ///
    /// let m = Matrix3::new(1.0, 2.0, 0.0,
    ///                      3.0, 4.0, 0.0,
    ///                      0.0, 0.0, 1.0);
    /// let t = Transform2::from_matrix_unchecked(m);
    /// assert_eq!(*t.matrix(), m);
    /// ```
    #[inline]
    #[must_use]
    pub const fn matrix(&self) -> &OMatrix<T, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>> {
        &self.matrix
    }

    /// A mutable reference to the underlying matrix.
    ///
    /// It is `_unchecked` because direct modifications of this matrix may break invariants
    /// identified by this transformation category.
    ///
    /// # Examples
    /// ```
    /// # use nalgebra::{Matrix3, Transform2};
    ///
    /// let m = Matrix3::new(1.0, 2.0, 0.0,
    ///                      3.0, 4.0, 0.0,
    ///                      0.0, 0.0, 1.0);
    /// let mut t = Transform2::from_matrix_unchecked(m);
    /// t.matrix_mut_unchecked().m12 = 42.0;
    /// t.matrix_mut_unchecked().m23 = 90.0;
    ///
    ///
    /// let expected = Matrix3::new(1.0, 42.0, 0.0,
    ///                             3.0, 4.0,  90.0,
    ///                             0.0, 0.0,  1.0);
    /// assert_eq!(*t.matrix(), expected);
    /// ```
    #[inline]
    pub const fn matrix_mut_unchecked(
        &mut self,
    ) -> &mut OMatrix<T, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>> {
        &mut self.matrix
    }

    /// Sets the category of this transform.
    ///
    /// This can be done only if the new category is more general than the current one, e.g., a
    /// transform with category `TProjective` cannot be converted to a transform with category
    /// `TAffine` because not all projective transformations are affine (the other way-round is
    /// valid though).
    #[inline]
    pub fn set_category<CNew: SuperTCategoryOf<C>>(self) -> Transform<T, CNew, D> {
        Transform::from_matrix_unchecked(self.matrix)
    }

    /// Clones this transform into one that owns its data.
    #[inline]
    #[deprecated(
        note = "This method is redundant with automatic `Copy` and the `.clone()` method and will be removed in a future release."
    )]
    pub fn clone_owned(&self) -> Transform<T, C, D> {
        Transform::from_matrix_unchecked(self.matrix.clone_owned())
    }

    /// Converts this transform into its equivalent homogeneous transformation matrix.
    ///
    /// # Examples
    /// ```
    /// # use nalgebra::{Matrix3, Transform2};
    ///
    /// let m = Matrix3::new(1.0, 2.0, 0.0,
    ///                      3.0, 4.0, 0.0,
    ///                      0.0, 0.0, 1.0);
    /// let t = Transform2::from_matrix_unchecked(m);
    /// assert_eq!(t.to_homogeneous(), m);
    /// ```
    #[inline]
    #[must_use]
    pub fn to_homogeneous(&self) -> OMatrix<T, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>> {
        self.matrix().clone_owned()
    }

    /// Attempts to invert this transformation. You may use `.inverse` instead of this
    /// transformation has a subcategory of `TProjective` (i.e. if it is a `Projective{2,3}` or `Affine{2,3}`).
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use nalgebra::{Matrix3, Transform2};
    ///
    /// let m = Matrix3::new(2.0, 2.0, -0.3,
    ///                      3.0, 4.0, 0.1,
    ///                      0.0, 0.0, 1.0);
    /// let t = Transform2::from_matrix_unchecked(m);
    /// let inv_t = t.try_inverse().unwrap();
    /// assert_relative_eq!(t * inv_t, Transform2::identity());
    /// assert_relative_eq!(inv_t * t, Transform2::identity());
    ///
    /// // Non-invertible case.
    /// let m = Matrix3::new(0.0, 2.0, 1.0,
    ///                      3.0, 0.0, 5.0,
    ///                      0.0, 0.0, 0.0);
    /// let t = Transform2::from_matrix_unchecked(m);
    /// assert!(t.try_inverse().is_none());
    /// ```
    #[inline]
    #[must_use = "Did you mean to use try_inverse_mut()?"]
    pub fn try_inverse(self) -> Option<Transform<T, C, D>> {
        self.matrix
            .try_inverse()
            .map(Transform::from_matrix_unchecked)
    }

    /// Inverts this transformation. Use `.try_inverse` if this transform has the `TGeneral`
    /// category (i.e., a `Transform{2,3}` may not be invertible).
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use nalgebra::{Matrix3, Projective2};
    ///
    /// let m = Matrix3::new(2.0, 2.0, -0.3,
    ///                      3.0, 4.0, 0.1,
    ///                      0.0, 0.0, 1.0);
    /// let proj = Projective2::from_matrix_unchecked(m);
    /// let inv_t = proj.inverse();
    /// assert_relative_eq!(proj * inv_t, Projective2::identity());
    /// assert_relative_eq!(inv_t * proj, Projective2::identity());
    /// ```
    #[inline]
    #[must_use = "Did you mean to use inverse_mut()?"]
    pub fn inverse(self) -> Transform<T, C, D>
    where
        C: SubTCategoryOf<TProjective>,
    {
        // TODO: specialize for TAffine?
        Transform::from_matrix_unchecked(self.matrix.try_inverse().unwrap())
    }

    /// Attempts to invert this transformation in-place. You may use `.inverse_mut` instead of this
    /// transformation has a subcategory of `TProjective`.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use nalgebra::{Matrix3, Transform2};
    ///
    /// let m = Matrix3::new(2.0, 2.0, -0.3,
    ///                      3.0, 4.0, 0.1,
    ///                      0.0, 0.0, 1.0);
    /// let t = Transform2::from_matrix_unchecked(m);
    /// let mut inv_t = t;
    /// assert!(inv_t.try_inverse_mut());
    /// assert_relative_eq!(t * inv_t, Transform2::identity());
    /// assert_relative_eq!(inv_t * t, Transform2::identity());
    ///
    /// // Non-invertible case.
    /// let m = Matrix3::new(0.0, 2.0, 1.0,
    ///                      3.0, 0.0, 5.0,
    ///                      0.0, 0.0, 0.0);
    /// let mut t = Transform2::from_matrix_unchecked(m);
    /// assert!(!t.try_inverse_mut());
    /// ```
    #[inline]
    pub fn try_inverse_mut(&mut self) -> bool {
        self.matrix.try_inverse_mut()
    }

    /// Inverts this transformation in-place. Use `.try_inverse_mut` if this transform has the
    /// `TGeneral` category (it may not be invertible).
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use nalgebra::{Matrix3, Projective2};
    ///
    /// let m = Matrix3::new(2.0, 2.0, -0.3,
    ///                      3.0, 4.0, 0.1,
    ///                      0.0, 0.0, 1.0);
    /// let proj = Projective2::from_matrix_unchecked(m);
    /// let mut inv_t = proj;
    /// inv_t.inverse_mut();
    /// assert_relative_eq!(proj * inv_t, Projective2::identity());
    /// assert_relative_eq!(inv_t * proj, Projective2::identity());
    /// ```
    #[inline]
    pub fn inverse_mut(&mut self)
    where
        C: SubTCategoryOf<TProjective>,
    {
        let _ = self.matrix.try_inverse_mut();
    }
}

impl<T, C, const D: usize> Transform<T, C, D>
where
    T: RealField,
    C: TCategory,
    Const<D>: DimNameAdd<U1>,
    DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>
        + Allocator<DimNameSum<Const<D>, U1>>, // + Allocator<D, D>
                                               // + Allocator<D>
{
    /// Transform the given point by this transformation.
    ///
    /// This is the same as the multiplication `self * pt`.
    #[inline]
    #[must_use]
    pub fn transform_point(&self, pt: &Point<T, D>) -> Point<T, D> {
        self * pt
    }

    /// Transform the given vector by this transformation, ignoring the
    /// translational component of the transformation.
    ///
    /// This is the same as the multiplication `self * v`.
    #[inline]
    #[must_use]
    pub fn transform_vector(&self, v: &SVector<T, D>) -> SVector<T, D> {
        self * v
    }
}

impl<T: RealField, C: TCategory, const D: usize> Transform<T, C, D>
where
    Const<D>: DimNameAdd<U1>,
    C: SubTCategoryOf<TProjective>,
    DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>
        + Allocator<DimNameSum<Const<D>, U1>>, // + Allocator<D, D>
                                               // + Allocator<D>
{
    /// Transform the given point by the inverse of this transformation.
    /// This may be cheaper than inverting the transformation and transforming
    /// the point.
    #[inline]
    #[must_use]
    pub fn inverse_transform_point(&self, pt: &Point<T, D>) -> Point<T, D> {
        self.clone().inverse() * pt
    }

    /// Transform the given vector by the inverse of this transformation.
    /// This may be cheaper than inverting the transformation and transforming
    /// the vector.
    #[inline]
    #[must_use]
    pub fn inverse_transform_vector(&self, v: &SVector<T, D>) -> SVector<T, D> {
        self.clone().inverse() * v
    }
}

impl<T: RealField, const D: usize> Transform<T, TGeneral, D>
where
    Const<D>: DimNameAdd<U1>,
    DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>,
{
    /// A mutable reference to underlying matrix. Use `.matrix_mut_unchecked` instead if this
    /// transformation category is not `TGeneral`.
    #[inline]
    pub const fn matrix_mut(
        &mut self,
    ) -> &mut OMatrix<T, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>> {
        self.matrix_mut_unchecked()
    }
}

impl<T: RealField, C: TCategory, const D: usize> AbsDiffEq for Transform<T, C, D>
where
    Const<D>: DimNameAdd<U1>,
    T::Epsilon: Clone,
    DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>,
{
    type Epsilon = T::Epsilon;

    #[inline]
    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.matrix.abs_diff_eq(&other.matrix, epsilon)
    }
}

impl<T: RealField, C: TCategory, const D: usize> RelativeEq for Transform<T, C, D>
where
    Const<D>: DimNameAdd<U1>,
    T::Epsilon: Clone,
    DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>,
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
        self.matrix
            .relative_eq(&other.matrix, epsilon, max_relative)
    }
}

impl<T: RealField, C: TCategory, const D: usize> UlpsEq for Transform<T, C, D>
where
    Const<D>: DimNameAdd<U1>,
    T::Epsilon: Clone,
    DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>,
{
    #[inline]
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    #[inline]
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        self.matrix.ulps_eq(&other.matrix, epsilon, max_ulps)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::base::Matrix4;

    #[test]
    fn checks_homogeneous_invariants_of_square_identity_matrix() {
        assert!(TAffine::check_homogeneous_invariants(
            &Matrix4::<f32>::identity()
        ));
    }
}
