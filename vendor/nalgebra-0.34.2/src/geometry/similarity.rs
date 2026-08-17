// Needed otherwise the rkyv macros generate code incompatible with rust-2024
#![cfg_attr(feature = "rkyv-serialize", allow(unsafe_op_in_unsafe_fn))]

use approx::{AbsDiffEq, RelativeEq, UlpsEq};
use num::Zero;
use std::fmt;
use std::hash;

#[cfg(feature = "serde-serialize-no-std")]
use serde::{Deserialize, Serialize};

use simba::scalar::{RealField, SubsetOf};
use simba::simd::SimdRealField;

use crate::base::allocator::Allocator;
use crate::base::dimension::{DimNameAdd, DimNameSum, U1};
use crate::base::storage::Owned;
use crate::base::{Const, DefaultAllocator, OMatrix, SVector, Scalar};
use crate::geometry::{AbstractRotation, Isometry, Point, Translation};

#[cfg(feature = "rkyv-serialize")]
use rkyv::bytecheck;

/// A similarity, i.e., an uniform scaling, followed by a rotation, followed by a translation.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde-serialize-no-std", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde-serialize-no-std",
    serde(bound(serialize = "T: Scalar + Serialize,
                     R: Serialize,
                     DefaultAllocator: Allocator<Const<D>>,
                     Owned<T, Const<D>>: Serialize"))
)]
#[cfg_attr(
    feature = "serde-serialize-no-std",
    serde(bound(deserialize = "T: Scalar + Deserialize<'de>,
                       R: Deserialize<'de>,
                       DefaultAllocator: Allocator<Const<D>>,
                       Owned<T, Const<D>>: Deserialize<'de>"))
)]
#[cfg_attr(feature = "rkyv-serialize", derive(bytecheck::CheckBytes))]
#[cfg_attr(
    feature = "rkyv-serialize-no-std",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize),
    archive(
        as = "Similarity<T::Archived, R::Archived, D>",
        bound(archive = "
        T: rkyv::Archive,
        R: rkyv::Archive,
        Isometry<T, R, D>: rkyv::Archive<Archived = Isometry<T::Archived, R::Archived, D>>
    ")
    )
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Similarity<T, R, const D: usize> {
    /// The part of this similarity that does not include the scaling factor.
    pub isometry: Isometry<T, R, D>,
    scaling: T,
}

impl<T: Scalar + hash::Hash, R: hash::Hash, const D: usize> hash::Hash for Similarity<T, R, D>
where
    Owned<T, Const<D>>: hash::Hash,
{
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.isometry.hash(state);
        self.scaling.hash(state);
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl<T: Scalar, R, const D: usize> bytemuck::Zeroable for Similarity<T, R, D> where
    Isometry<T, R, D>: bytemuck::Zeroable
{
}

#[cfg(feature = "bytemuck")]
unsafe impl<T: Scalar, R, const D: usize> bytemuck::Pod for Similarity<T, R, D>
where
    Isometry<T, R, D>: bytemuck::Pod,
    R: Copy,
    T: Copy,
{
}

impl<T: Scalar + Zero, R, const D: usize> Similarity<T, R, D>
where
    R: AbstractRotation<T, D>,
{
    /// Creates a new similarity from its rotational and translational parts.
    #[inline]
    pub fn from_parts(translation: Translation<T, D>, rotation: R, scaling: T) -> Self {
        Self::from_isometry(Isometry::from_parts(translation, rotation), scaling)
    }

    /// Creates a new similarity from its rotational and translational parts.
    #[inline]
    pub fn from_isometry(isometry: Isometry<T, R, D>, scaling: T) -> Self {
        assert!(!scaling.is_zero(), "The scaling factor must not be zero.");

        Self { isometry, scaling }
    }

    /// The scaling factor of this similarity transformation.
    #[inline]
    pub fn set_scaling(&mut self, scaling: T) {
        assert!(
            !scaling.is_zero(),
            "The similarity scaling factor must not be zero."
        );

        self.scaling = scaling;
    }
}

impl<T: Scalar, R, const D: usize> Similarity<T, R, D> {
    /// The scaling factor of this similarity transformation.
    #[inline]
    #[must_use]
    pub fn scaling(&self) -> T {
        self.scaling.clone()
    }
}

impl<T: SimdRealField, R, const D: usize> Similarity<T, R, D>
where
    T::Element: SimdRealField,
    R: AbstractRotation<T, D>,
{
    /// Creates a new similarity that applies only a scaling factor.
    #[inline]
    pub fn from_scaling(scaling: T) -> Self {
        Self::from_isometry(Isometry::identity(), scaling)
    }

    /// Inverts `self`.
    #[inline]
    #[must_use = "Did you mean to use inverse_mut()?"]
    pub fn inverse(&self) -> Self {
        let mut res = self.clone();
        res.inverse_mut();
        res
    }

    /// Inverts `self` in-place.
    #[inline]
    pub fn inverse_mut(&mut self) {
        self.scaling = T::one() / self.scaling.clone();
        self.isometry.inverse_mut();
        self.isometry.translation.vector *= self.scaling.clone();
    }

    /// The similarity transformation that applies a scaling factor `scaling` before `self`.
    #[inline]
    #[must_use = "Did you mean to use prepend_scaling_mut()?"]
    pub fn prepend_scaling(&self, scaling: T) -> Self {
        assert!(
            !scaling.is_zero(),
            "The similarity scaling factor must not be zero."
        );

        Self::from_isometry(self.isometry.clone(), self.scaling.clone() * scaling)
    }

    /// The similarity transformation that applies a scaling factor `scaling` after `self`.
    #[inline]
    #[must_use = "Did you mean to use append_scaling_mut()?"]
    pub fn append_scaling(&self, scaling: T) -> Self {
        assert!(
            !scaling.is_zero(),
            "The similarity scaling factor must not be zero."
        );

        Self::from_parts(
            Translation::from(&self.isometry.translation.vector * scaling.clone()),
            self.isometry.rotation.clone(),
            self.scaling.clone() * scaling,
        )
    }

    /// Sets `self` to the similarity transformation that applies a scaling factor `scaling` before `self`.
    #[inline]
    pub fn prepend_scaling_mut(&mut self, scaling: T) {
        assert!(
            !scaling.is_zero(),
            "The similarity scaling factor must not be zero."
        );

        self.scaling *= scaling
    }

    /// Sets `self` to the similarity transformation that applies a scaling factor `scaling` after `self`.
    #[inline]
    pub fn append_scaling_mut(&mut self, scaling: T) {
        assert!(
            !scaling.is_zero(),
            "The similarity scaling factor must not be zero."
        );

        self.isometry.translation.vector *= scaling.clone();
        self.scaling *= scaling;
    }

    /// Appends to `self` the given translation in-place.
    #[inline]
    pub fn append_translation_mut(&mut self, t: &Translation<T, D>) {
        self.isometry.append_translation_mut(t)
    }

    /// Appends to `self` the given rotation in-place.
    #[inline]
    pub fn append_rotation_mut(&mut self, r: &R) {
        self.isometry.append_rotation_mut(r)
    }

    /// Appends in-place to `self` a rotation centered at the point `p`, i.e., the rotation that
    /// lets `p` invariant.
    #[inline]
    pub fn append_rotation_wrt_point_mut(&mut self, r: &R, p: &Point<T, D>) {
        self.isometry.append_rotation_wrt_point_mut(r, p)
    }

    /// Appends in-place to `self` a rotation centered at the point with coordinates
    /// `self.translation`.
    #[inline]
    pub fn append_rotation_wrt_center_mut(&mut self, r: &R) {
        self.isometry.append_rotation_wrt_center_mut(r)
    }

    /// Transform the given point by this similarity.
    ///
    /// This is the same as the multiplication `self * pt`.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use std::f32;
    /// # use nalgebra::{Point3, Similarity3, Vector3};
    /// let axisangle = Vector3::y() * f32::consts::FRAC_PI_2;
    /// let translation = Vector3::new(1.0, 2.0, 3.0);
    /// let sim = Similarity3::new(translation, axisangle, 3.0);
    /// let transformed_point = sim.transform_point(&Point3::new(4.0, 5.0, 6.0));
    /// assert_relative_eq!(transformed_point, Point3::new(19.0, 17.0, -9.0), epsilon = 1.0e-5);
    /// ```
    #[inline]
    #[must_use]
    pub fn transform_point(&self, pt: &Point<T, D>) -> Point<T, D> {
        self * pt
    }

    /// Transform the given vector by this similarity, ignoring the translational
    /// component.
    ///
    /// This is the same as the multiplication `self * t`.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use std::f32;
    /// # use nalgebra::{Similarity3, Vector3};
    /// let axisangle = Vector3::y() * f32::consts::FRAC_PI_2;
    /// let translation = Vector3::new(1.0, 2.0, 3.0);
    /// let sim = Similarity3::new(translation, axisangle, 3.0);
    /// let transformed_vector = sim.transform_vector(&Vector3::new(4.0, 5.0, 6.0));
    /// assert_relative_eq!(transformed_vector, Vector3::new(18.0, 15.0, -12.0), epsilon = 1.0e-5);
    /// ```
    #[inline]
    #[must_use]
    pub fn transform_vector(&self, v: &SVector<T, D>) -> SVector<T, D> {
        self * v
    }

    /// Transform the given point by the inverse of this similarity. This may
    /// be cheaper than inverting the similarity and then transforming the
    /// given point.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use std::f32;
    /// # use nalgebra::{Point3, Similarity3, Vector3};
    /// let axisangle = Vector3::y() * f32::consts::FRAC_PI_2;
    /// let translation = Vector3::new(1.0, 2.0, 3.0);
    /// let sim = Similarity3::new(translation, axisangle, 2.0);
    /// let transformed_point = sim.inverse_transform_point(&Point3::new(4.0, 5.0, 6.0));
    /// assert_relative_eq!(transformed_point, Point3::new(-1.5, 1.5, 1.5), epsilon = 1.0e-5);
    /// ```
    #[inline]
    #[must_use]
    pub fn inverse_transform_point(&self, pt: &Point<T, D>) -> Point<T, D> {
        self.isometry.inverse_transform_point(pt) / self.scaling()
    }

    /// Transform the given vector by the inverse of this similarity,
    /// ignoring the translational component. This may be cheaper than
    /// inverting the similarity and then transforming the given vector.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use std::f32;
    /// # use nalgebra::{Similarity3, Vector3};
    /// let axisangle = Vector3::y() * f32::consts::FRAC_PI_2;
    /// let translation = Vector3::new(1.0, 2.0, 3.0);
    /// let sim = Similarity3::new(translation, axisangle, 2.0);
    /// let transformed_vector = sim.inverse_transform_vector(&Vector3::new(4.0, 5.0, 6.0));
    /// assert_relative_eq!(transformed_vector, Vector3::new(-3.0, 2.5, 2.0), epsilon = 1.0e-5);
    /// ```
    #[inline]
    #[must_use]
    pub fn inverse_transform_vector(&self, v: &SVector<T, D>) -> SVector<T, D> {
        self.isometry.inverse_transform_vector(v) / self.scaling()
    }
}

// NOTE: we don't require `R: Rotation<...>` here because this is not useful for the implementation
// and makes it harder to use it, e.g., for Transform × Isometry implementation.
// This is OK since all constructors of the isometry enforce the Rotation bound already (and
// explicit struct construction is prevented by the private scaling factor).
impl<T: SimdRealField, R, const D: usize> Similarity<T, R, D> {
    /// Converts this similarity into its equivalent homogeneous transformation matrix.
    #[inline]
    #[must_use]
    pub fn to_homogeneous(&self) -> OMatrix<T, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>
    where
        Const<D>: DimNameAdd<U1>,
        R: SubsetOf<OMatrix<T, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>>,
        DefaultAllocator: Allocator<DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>,
    {
        let mut res = self.isometry.to_homogeneous();

        for e in res.fixed_view_mut::<D, D>(0, 0).iter_mut() {
            *e *= self.scaling.clone()
        }

        res
    }
}

impl<T: SimdRealField, R, const D: usize> Eq for Similarity<T, R, D> where
    R: AbstractRotation<T, D> + Eq
{
}

impl<T: SimdRealField, R, const D: usize> PartialEq for Similarity<T, R, D>
where
    R: AbstractRotation<T, D> + PartialEq,
{
    #[inline]
    fn eq(&self, right: &Self) -> bool {
        self.isometry == right.isometry && self.scaling == right.scaling
    }
}

impl<T: RealField, R, const D: usize> AbsDiffEq for Similarity<T, R, D>
where
    R: AbstractRotation<T, D> + AbsDiffEq<Epsilon = T::Epsilon>,
    T::Epsilon: Clone,
{
    type Epsilon = T::Epsilon;

    #[inline]
    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.isometry.abs_diff_eq(&other.isometry, epsilon.clone())
            && self.scaling.abs_diff_eq(&other.scaling, epsilon)
    }
}

impl<T: RealField, R, const D: usize> RelativeEq for Similarity<T, R, D>
where
    R: AbstractRotation<T, D> + RelativeEq<Epsilon = T::Epsilon>,
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
        self.isometry
            .relative_eq(&other.isometry, epsilon.clone(), max_relative.clone())
            && self
                .scaling
                .relative_eq(&other.scaling, epsilon, max_relative)
    }
}

impl<T: RealField, R, const D: usize> UlpsEq for Similarity<T, R, D>
where
    R: AbstractRotation<T, D> + UlpsEq<Epsilon = T::Epsilon>,
    T::Epsilon: Clone,
{
    #[inline]
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    #[inline]
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        self.isometry
            .ulps_eq(&other.isometry, epsilon.clone(), max_ulps)
            && self.scaling.ulps_eq(&other.scaling, epsilon, max_ulps)
    }
}

/*
 *
 * Display
 *
 */
impl<T, R, const D: usize> fmt::Display for Similarity<T, R, D>
where
    T: RealField + fmt::Display,
    R: AbstractRotation<T, D> + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let precision = f.precision().unwrap_or(3);

        writeln!(f, "Similarity {{")?;
        write!(f, "{:.*}", precision, self.isometry)?;
        write!(f, "Scaling: {:.*}", precision, self.scaling)?;
        writeln!(f, "}}")
    }
}
