use approx::{AbsDiffEq, RelativeEq, UlpsEq};
use num_complex::Complex;
use std::fmt;

use crate::Scalar;
use crate::base::{Matrix2, Matrix3, Normed, Unit, Vector1, Vector2};
use crate::geometry::{Point2, Rotation2};
use simba::scalar::RealField;
use simba::simd::SimdRealField;
use std::cmp::{Eq, PartialEq};

/// A 2D rotation represented as a complex number with magnitude 1.
///
/// All the methods specific [`UnitComplex`] are listed here. You may also
/// read the documentation of the [`Complex`] type which
/// is used internally and accessible with `unit_complex.complex()`.
///
/// # Construction
/// * [Identity <span style="float:right;">`identity`</span>](#identity)
/// * [From a 2D rotation angle <span style="float:right;">`new`, `from_cos_sin_unchecked`…</span>](#construction-from-a-2d-rotation-angle)
/// * [From an existing 2D matrix or complex number <span style="float:right;">`from_matrix`, `rotation_to`, `powf`…</span>](#construction-from-an-existing-2d-matrix-or-complex-number)
/// * [From two vectors <span style="float:right;">`rotation_between`, `scaled_rotation_between_axis`…</span>](#construction-from-two-vectors)
///
/// # Transformation and composition
/// * [Angle extraction <span style="float:right;">`angle`, `angle_to`…</span>](#angle-extraction)
/// * [Transformation of a vector or a point <span style="float:right;">`transform_vector`, `inverse_transform_point`…</span>](#transformation-of-a-vector-or-a-point)
/// * [Conjugation and inversion <span style="float:right;">`conjugate`, `inverse_mut`…</span>](#conjugation-and-inversion)
/// * [Interpolation <span style="float:right;">`slerp`…</span>](#interpolation)
///
/// # Conversion
/// * [Conversion to a matrix <span style="float:right;">`to_rotation_matrix`, `to_homogeneous`…</span>](#conversion-to-a-matrix)
pub type UnitComplex<T> = Unit<Complex<T>>;

impl<T: Scalar + PartialEq> PartialEq for UnitComplex<T> {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        (**self).eq(&**rhs)
    }
}

impl<T: Scalar + Eq> Eq for UnitComplex<T> {}

impl<T: SimdRealField> Normed for Complex<T> {
    type Norm = T::SimdRealField;

    #[inline]
    fn norm(&self) -> T::SimdRealField {
        // We don't use `.norm_sqr()` because it requires
        // some very strong Num trait requirements.
        (self.re.clone() * self.re.clone() + self.im.clone() * self.im.clone()).simd_sqrt()
    }

    #[inline]
    fn norm_squared(&self) -> T::SimdRealField {
        // We don't use `.norm_sqr()` because it requires
        // some very strong Num trait requirements.
        self.re.clone() * self.re.clone() + self.im.clone() * self.im.clone()
    }

    #[inline]
    fn scale_mut(&mut self, n: Self::Norm) {
        self.re *= n.clone();
        self.im *= n;
    }

    #[inline]
    fn unscale_mut(&mut self, n: Self::Norm) {
        self.re /= n.clone();
        self.im /= n;
    }
}

/// # Angle extraction
impl<T: SimdRealField> UnitComplex<T>
where
    T::Element: SimdRealField,
{
    /// The rotation angle in `]-pi; pi]` of this unit complex number.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::UnitComplex;
    /// let rot = UnitComplex::new(1.78);
    /// assert_eq!(rot.angle(), 1.78);
    /// ```
    #[inline]
    #[must_use]
    pub fn angle(&self) -> T {
        self.im.clone().simd_atan2(self.re.clone())
    }

    /// The sine of the rotation angle.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::UnitComplex;
    /// let angle = 1.78f32;
    /// let rot = UnitComplex::new(angle);
    /// assert_eq!(rot.sin_angle(), angle.sin());
    /// ```
    #[inline]
    #[must_use]
    pub fn sin_angle(&self) -> T {
        self.im.clone()
    }

    /// The cosine of the rotation angle.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::UnitComplex;
    /// let angle = 1.78f32;
    /// let rot = UnitComplex::new(angle);
    /// assert_eq!(rot.cos_angle(),angle.cos());
    /// ```
    #[inline]
    #[must_use]
    pub fn cos_angle(&self) -> T {
        self.re.clone()
    }

    /// The rotation angle returned as a 1-dimensional vector.
    ///
    /// This is generally used in the context of generic programming. Using
    /// the `.angle()` method instead is more common.
    #[inline]
    #[must_use]
    pub fn scaled_axis(&self) -> Vector1<T> {
        Vector1::new(self.angle())
    }

    /// The rotation axis and angle in (0, pi] of this complex number.
    ///
    /// This is generally used in the context of generic programming. Using
    /// the `.angle()` method instead is more common.
    /// Returns `None` if the angle is zero.
    #[inline]
    #[must_use]
    pub fn axis_angle(&self) -> Option<(Unit<Vector1<T>>, T)>
    where
        T: RealField,
    {
        let ang = self.angle();

        if ang.is_zero() {
            None
        } else if ang.is_sign_positive() {
            Some((Unit::new_unchecked(Vector1::x()), ang))
        } else {
            Some((Unit::new_unchecked(-Vector1::<T>::x()), -ang))
        }
    }

    /// The rotation angle needed to make `self` and `other` coincide.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use nalgebra::UnitComplex;
    /// let rot1 = UnitComplex::new(0.1);
    /// let rot2 = UnitComplex::new(1.7);
    /// assert_relative_eq!(rot1.angle_to(&rot2), 1.6);
    /// ```
    #[inline]
    #[must_use]
    pub fn angle_to(&self, other: &Self) -> T {
        let delta = self.rotation_to(other);
        delta.angle()
    }
}

/// # Conjugation and inversion
impl<T: SimdRealField> UnitComplex<T>
where
    T::Element: SimdRealField,
{
    /// Compute the conjugate of this unit complex number.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::UnitComplex;
    /// let rot = UnitComplex::new(1.78);
    /// let conj = rot.conjugate();
    /// assert_eq!(rot.complex().im, -conj.complex().im);
    /// assert_eq!(rot.complex().re, conj.complex().re);
    /// ```
    #[inline]
    #[must_use = "Did you mean to use conjugate_mut()?"]
    pub fn conjugate(&self) -> Self {
        Self::new_unchecked(self.conj())
    }

    /// Inverts this complex number if it is not zero.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use nalgebra::UnitComplex;
    /// let rot = UnitComplex::new(1.2);
    /// let inv = rot.inverse();
    /// assert_relative_eq!(rot * inv, UnitComplex::identity(), epsilon = 1.0e-6);
    /// assert_relative_eq!(inv * rot, UnitComplex::identity(), epsilon = 1.0e-6);
    /// ```
    #[inline]
    #[must_use = "Did you mean to use inverse_mut()?"]
    pub fn inverse(&self) -> Self {
        self.conjugate()
    }

    /// Compute in-place the conjugate of this unit complex number.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use nalgebra::UnitComplex;
    /// let angle = 1.7;
    /// let rot = UnitComplex::new(angle);
    /// let mut conj = UnitComplex::new(angle);
    /// conj.conjugate_mut();
    /// assert_eq!(rot.complex().im, -conj.complex().im);
    /// assert_eq!(rot.complex().re, conj.complex().re);
    /// ```
    #[inline]
    pub fn conjugate_mut(&mut self) {
        let me = self.as_mut_unchecked();
        me.im = -me.im.clone();
    }

    /// Inverts in-place this unit complex number.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use nalgebra::UnitComplex;
    /// let angle = 1.7;
    /// let mut rot = UnitComplex::new(angle);
    /// rot.inverse_mut();
    /// assert_relative_eq!(rot * UnitComplex::new(angle), UnitComplex::identity());
    /// assert_relative_eq!(UnitComplex::new(angle) * rot, UnitComplex::identity());
    /// ```
    #[inline]
    pub fn inverse_mut(&mut self) {
        self.conjugate_mut()
    }
}

/// # Conversion to a matrix
impl<T: SimdRealField> UnitComplex<T>
where
    T::Element: SimdRealField,
{
    /// Builds the rotation matrix corresponding to this unit complex number.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::{UnitComplex, Rotation2};
    /// # use std::f32;
    /// let rot = UnitComplex::new(f32::consts::FRAC_PI_6);
    /// let expected = Rotation2::new(f32::consts::FRAC_PI_6);
    /// assert_eq!(rot.to_rotation_matrix(), expected);
    /// ```
    #[inline]
    #[must_use]
    pub fn to_rotation_matrix(self) -> Rotation2<T> {
        let r = self.re.clone();
        let i = self.im.clone();

        Rotation2::from_matrix_unchecked(Matrix2::new(r.clone(), -i.clone(), i, r))
    }

    /// Converts this unit complex number into its equivalent homogeneous transformation matrix.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::{UnitComplex, Matrix3};
    /// # use std::f32;
    /// let rot = UnitComplex::new(f32::consts::FRAC_PI_6);
    /// let expected = Matrix3::new(0.8660254, -0.5,      0.0,
    ///                             0.5,       0.8660254, 0.0,
    ///                             0.0,       0.0,       1.0);
    /// assert_eq!(rot.to_homogeneous(), expected);
    /// ```
    #[inline]
    #[must_use]
    pub fn to_homogeneous(self) -> Matrix3<T> {
        self.to_rotation_matrix().to_homogeneous()
    }
}

/// # Transformation of a vector or a point
impl<T: SimdRealField> UnitComplex<T>
where
    T::Element: SimdRealField,
{
    /// Rotate the given point by this unit complex number.
    ///
    /// This is the same as the multiplication `self * pt`.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use nalgebra::{UnitComplex, Point2};
    /// # use std::f32;
    /// let rot = UnitComplex::new(f32::consts::FRAC_PI_2);
    /// let transformed_point = rot.transform_point(&Point2::new(1.0, 2.0));
    /// assert_relative_eq!(transformed_point, Point2::new(-2.0, 1.0), epsilon = 1.0e-6);
    /// ```
    #[inline]
    #[must_use]
    pub fn transform_point(&self, pt: &Point2<T>) -> Point2<T> {
        self * pt
    }

    /// Rotate the given vector by this unit complex number.
    ///
    /// This is the same as the multiplication `self * v`.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use nalgebra::{UnitComplex, Vector2};
    /// # use std::f32;
    /// let rot = UnitComplex::new(f32::consts::FRAC_PI_2);
    /// let transformed_vector = rot.transform_vector(&Vector2::new(1.0, 2.0));
    /// assert_relative_eq!(transformed_vector, Vector2::new(-2.0, 1.0), epsilon = 1.0e-6);
    /// ```
    #[inline]
    #[must_use]
    pub fn transform_vector(&self, v: &Vector2<T>) -> Vector2<T> {
        self * v
    }

    /// Rotate the given point by the inverse of this unit complex number.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use nalgebra::{UnitComplex, Point2};
    /// # use std::f32;
    /// let rot = UnitComplex::new(f32::consts::FRAC_PI_2);
    /// let transformed_point = rot.inverse_transform_point(&Point2::new(1.0, 2.0));
    /// assert_relative_eq!(transformed_point, Point2::new(2.0, -1.0), epsilon = 1.0e-6);
    /// ```
    #[inline]
    #[must_use]
    pub fn inverse_transform_point(&self, pt: &Point2<T>) -> Point2<T> {
        // TODO: would it be useful performance-wise not to call inverse explicitly (i-e. implement
        // the inverse transformation explicitly here) ?
        self.inverse() * pt
    }

    /// Rotate the given vector by the inverse of this unit complex number.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use nalgebra::{UnitComplex, Vector2};
    /// # use std::f32;
    /// let rot = UnitComplex::new(f32::consts::FRAC_PI_2);
    /// let transformed_vector = rot.inverse_transform_vector(&Vector2::new(1.0, 2.0));
    /// assert_relative_eq!(transformed_vector, Vector2::new(2.0, -1.0), epsilon = 1.0e-6);
    /// ```
    #[inline]
    #[must_use]
    pub fn inverse_transform_vector(&self, v: &Vector2<T>) -> Vector2<T> {
        self.inverse() * v
    }

    /// Rotate the given vector by the inverse of this unit complex number.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use nalgebra::{UnitComplex, Vector2};
    /// # use std::f32;
    /// let rot = UnitComplex::new(f32::consts::FRAC_PI_2);
    /// let transformed_vector = rot.inverse_transform_unit_vector(&Vector2::x_axis());
    /// assert_relative_eq!(transformed_vector, -Vector2::y_axis(), epsilon = 1.0e-6);
    /// ```
    #[inline]
    #[must_use]
    pub fn inverse_transform_unit_vector(&self, v: &Unit<Vector2<T>>) -> Unit<Vector2<T>> {
        self.inverse() * v
    }
}

/// # Interpolation
impl<T: SimdRealField> UnitComplex<T>
where
    T::Element: SimdRealField,
{
    /// Spherical linear interpolation between two rotations represented as unit complex numbers.
    ///
    /// # Examples:
    ///
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use nalgebra::geometry::UnitComplex;
    ///
    /// let rot1 = UnitComplex::new(std::f32::consts::FRAC_PI_4);
    /// let rot2 = UnitComplex::new(-std::f32::consts::PI);
    ///
    /// let rot = rot1.slerp(&rot2, 1.0 / 3.0);
    ///
    /// assert_relative_eq!(rot.angle(), std::f32::consts::FRAC_PI_2);
    /// ```
    #[inline]
    #[must_use]
    pub fn slerp(&self, other: &Self, t: T) -> Self {
        let delta = other / self;
        self * Self::new(delta.angle() * t)
    }
}

impl<T: RealField + fmt::Display> fmt::Display for UnitComplex<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UnitComplex angle: {}", self.angle())
    }
}

impl<T: RealField> AbsDiffEq for UnitComplex<T> {
    type Epsilon = T;

    #[inline]
    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.re.abs_diff_eq(&other.re, epsilon.clone()) && self.im.abs_diff_eq(&other.im, epsilon)
    }
}

impl<T: RealField> RelativeEq for UnitComplex<T> {
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
        self.re
            .relative_eq(&other.re, epsilon.clone(), max_relative.clone())
            && self.im.relative_eq(&other.im, epsilon, max_relative)
    }
}

impl<T: RealField> UlpsEq for UnitComplex<T> {
    #[inline]
    fn default_max_ulps() -> u32 {
        T::default_max_ulps()
    }

    #[inline]
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        self.re.ulps_eq(&other.re, epsilon.clone(), max_ulps)
            && self.im.ulps_eq(&other.im, epsilon, max_ulps)
    }
}
