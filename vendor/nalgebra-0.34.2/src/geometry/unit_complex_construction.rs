#[cfg(feature = "arbitrary")]
use quickcheck::{Arbitrary, Gen};

#[cfg(feature = "rand-no-std")]
use rand::{
    Rng,
    distr::{Distribution, StandardUniform},
};

use num::One;
use num_complex::Complex;

use crate::base::dimension::{U1, U2};
use crate::base::storage::Storage;
use crate::base::{Matrix2, Scalar, Unit, Vector, Vector2};
use crate::geometry::{Rotation2, UnitComplex};
use simba::scalar::{RealField, SupersetOf};
use simba::simd::SimdRealField;

impl<T: SimdRealField> Default for UnitComplex<T>
where
    T::Element: SimdRealField,
{
    fn default() -> Self {
        Self::identity()
    }
}

/// # Identity
impl<T: SimdRealField> UnitComplex<T>
where
    T::Element: SimdRealField,
{
    /// The unit complex number multiplicative identity.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::UnitComplex;
    /// let rot1 = UnitComplex::identity();
    /// let rot2 = UnitComplex::new(1.7);
    ///
    /// assert_eq!(rot1 * rot2, rot2);
    /// assert_eq!(rot2 * rot1, rot2);
    /// ```
    #[inline]
    pub fn identity() -> Self {
        Self::new_unchecked(Complex::new(T::one(), T::zero()))
    }
}

/// # Construction from a 2D rotation angle
impl<T: SimdRealField> UnitComplex<T>
where
    T::Element: SimdRealField,
{
    /// Builds the unit complex number corresponding to the rotation with the given angle.
    ///
    /// # Example
    ///
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use std::f32;
    /// # use nalgebra::{UnitComplex, Point2};
    /// let rot = UnitComplex::new(f32::consts::FRAC_PI_2);
    ///
    /// assert_relative_eq!(rot * Point2::new(3.0, 4.0), Point2::new(-4.0, 3.0));
    /// ```
    #[inline]
    pub fn new(angle: T) -> Self {
        let (sin, cos) = angle.simd_sin_cos();
        Self::from_cos_sin_unchecked(cos, sin)
    }

    /// Builds the unit complex number corresponding to the rotation with the angle.
    ///
    /// Same as `Self::new(angle)`.
    ///
    /// # Example
    ///
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use std::f32;
    /// # use nalgebra::{UnitComplex, Point2};
    /// let rot = UnitComplex::from_angle(f32::consts::FRAC_PI_2);
    ///
    /// assert_relative_eq!(rot * Point2::new(3.0, 4.0), Point2::new(-4.0, 3.0));
    /// ```
    // TODO: deprecate this.
    #[inline]
    pub fn from_angle(angle: T) -> Self {
        Self::new(angle)
    }

    /// Builds the unit complex number from the sinus and cosinus of the rotation angle.
    ///
    /// The input values are not checked to actually be cosines and sine of the same value.
    /// Is is generally preferable to use the `::new(angle)` constructor instead.
    ///
    /// # Example
    ///
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use std::f32;
    /// # use nalgebra::{UnitComplex, Vector2, Point2};
    /// let angle = f32::consts::FRAC_PI_2;
    /// let rot = UnitComplex::from_cos_sin_unchecked(angle.cos(), angle.sin());
    ///
    /// assert_relative_eq!(rot * Point2::new(3.0, 4.0), Point2::new(-4.0, 3.0));
    /// ```
    #[inline]
    pub const fn from_cos_sin_unchecked(cos: T, sin: T) -> Self {
        Self::new_unchecked(Complex::new(cos, sin))
    }

    /// Builds a unit complex rotation from an angle in radian wrapped in a 1-dimensional vector.
    ///
    /// This is generally used in the context of generic programming. Using
    /// the `::new(angle)` method instead is more common.
    #[inline]
    pub fn from_scaled_axis<SB: Storage<T, U1>>(axisangle: Vector<T, U1, SB>) -> Self {
        Self::from_angle(axisangle[0].clone())
    }
}

/// # Construction from an existing 2D matrix or complex number
impl<T: SimdRealField> UnitComplex<T>
where
    T::Element: SimdRealField,
{
    /// Cast the components of `self` to another type.
    ///
    /// # Example
    /// ```
    /// #[macro_use] extern crate approx;
    /// # use nalgebra::UnitComplex;
    /// let c = UnitComplex::new(1.0f64);
    /// let c2 = c.cast::<f32>();
    /// assert_relative_eq!(c2, UnitComplex::new(1.0f32));
    /// ```
    pub fn cast<To: Scalar>(self) -> UnitComplex<To>
    where
        UnitComplex<To>: SupersetOf<Self>,
    {
        crate::convert(self)
    }

    /// The underlying complex number.
    ///
    /// Same as `self.as_ref()`.
    ///
    /// # Example
    /// ```
    /// # extern crate num_complex;
    /// # use num_complex::Complex;
    /// # use nalgebra::UnitComplex;
    /// let angle = 1.78f32;
    /// let rot = UnitComplex::new(angle);
    /// assert_eq!(*rot.complex(), Complex::new(angle.cos(), angle.sin()));
    /// ```
    #[inline]
    #[must_use]
    pub fn complex(&self) -> &Complex<T> {
        self.as_ref()
    }

    /// Creates a new unit complex number from a complex number.
    ///
    /// The input complex number will be normalized.
    #[inline]
    pub fn from_complex(q: Complex<T>) -> Self {
        Self::from_complex_and_get(q).0
    }

    /// Creates a new unit complex number from a complex number.
    ///
    /// The input complex number will be normalized. Returns the norm of the complex number as well.
    #[inline]
    pub fn from_complex_and_get(q: Complex<T>) -> (Self, T) {
        let norm = (q.im.clone() * q.im.clone() + q.re.clone() * q.re.clone()).simd_sqrt();
        (Self::new_unchecked(q / norm.clone()), norm)
    }

    /// Builds the unit complex number from the corresponding 2D rotation matrix.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::{Rotation2, UnitComplex};
    /// let rot = Rotation2::new(1.7);
    /// let complex = UnitComplex::from_rotation_matrix(&rot);
    /// assert_eq!(complex, UnitComplex::new(1.7));
    /// ```
    // TODO: add UnitComplex::from(...) instead?
    #[inline]
    pub fn from_rotation_matrix(rotmat: &Rotation2<T>) -> Self {
        Self::new_unchecked(Complex::new(rotmat[(0, 0)].clone(), rotmat[(1, 0)].clone()))
    }

    /// Builds a rotation from a basis assumed to be orthonormal.
    ///
    /// In order to get a valid unit-quaternion, the input must be an
    /// orthonormal basis, i.e., all vectors are normalized, and the are
    /// all orthogonal to each other. These invariants are not checked
    /// by this method.
    pub fn from_basis_unchecked(basis: &[Vector2<T>; 2]) -> Self {
        let mat = Matrix2::from_columns(&basis[..]);
        let rot = Rotation2::from_matrix_unchecked(mat);
        Self::from_rotation_matrix(&rot)
    }

    /// Builds an unit complex by extracting the rotation part of the given transformation `m`.
    ///
    /// This is an iterative method. See `.from_matrix_eps` to provide mover
    /// convergence parameters and starting solution.
    /// This implements "A Robust Method to Extract the Rotational Part of Deformations" by Müller et al.
    pub fn from_matrix(m: &Matrix2<T>) -> Self
    where
        T: RealField,
    {
        Rotation2::from_matrix(m).into()
    }

    /// Builds an unit complex by extracting the rotation part of the given transformation `m`.
    ///
    /// This implements "A Robust Method to Extract the Rotational Part of Deformations" by Müller et al.
    ///
    /// # Parameters
    ///
    /// * `m`: the matrix from which the rotational part is to be extracted.
    /// * `eps`: the angular errors tolerated between the current rotation and the optimal one.
    /// * `max_iter`: the maximum number of iterations. Loops indefinitely until convergence if set to `0`.
    /// * `guess`: an estimate of the solution. Convergence will be significantly faster if an initial solution close
    ///   to the actual solution is provided. Can be set to `UnitQuaternion::identity()` if no other
    ///   guesses come to mind.
    pub fn from_matrix_eps(m: &Matrix2<T>, eps: T, max_iter: usize, guess: Self) -> Self
    where
        T: RealField,
    {
        let guess = Rotation2::from(guess);
        Rotation2::from_matrix_eps(m, eps, max_iter, guess).into()
    }

    /// The unit complex number needed to make `self` and `other` coincide.
    ///
    /// The result is such that: `self.rotation_to(other) * self == other`.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use nalgebra::UnitComplex;
    /// let rot1 = UnitComplex::new(0.1);
    /// let rot2 = UnitComplex::new(1.7);
    /// let rot_to = rot1.rotation_to(&rot2);
    ///
    /// assert_relative_eq!(rot_to * rot1, rot2);
    /// assert_relative_eq!(rot_to.inverse() * rot2, rot1);
    /// ```
    #[inline]
    #[must_use]
    pub fn rotation_to(&self, other: &Self) -> Self {
        other / self
    }

    /// Raise this unit complex number to a given floating power.
    ///
    /// This returns the unit complex number that identifies a rotation angle equal to
    /// `self.angle() × n`.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use nalgebra::UnitComplex;
    /// let rot = UnitComplex::new(0.78);
    /// let pow = rot.powf(2.0);
    /// assert_relative_eq!(pow.angle(), 2.0 * 0.78);
    /// ```
    #[inline]
    #[must_use]
    pub fn powf(&self, n: T) -> Self {
        Self::from_angle(self.angle() * n)
    }
}

/// # Construction from two vectors
impl<T: SimdRealField> UnitComplex<T>
where
    T::Element: SimdRealField,
{
    /// The unit complex needed to make `a` and `b` be collinear and point toward the same
    /// direction.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use nalgebra::{Vector2, UnitComplex};
    /// let a = Vector2::new(1.0, 2.0);
    /// let b = Vector2::new(2.0, 1.0);
    /// let rot = UnitComplex::rotation_between(&a, &b);
    /// assert_relative_eq!(rot * a, b);
    /// assert_relative_eq!(rot.inverse() * b, a);
    /// ```
    #[inline]
    pub fn rotation_between<SB, SC>(a: &Vector<T, U2, SB>, b: &Vector<T, U2, SC>) -> Self
    where
        T: RealField,
        SB: Storage<T, U2>,
        SC: Storage<T, U2>,
    {
        Self::scaled_rotation_between(a, b, T::one())
    }

    /// The smallest rotation needed to make `a` and `b` collinear and point toward the same
    /// direction, raised to the power `s`.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use nalgebra::{Vector2, UnitComplex};
    /// let a = Vector2::new(1.0, 2.0);
    /// let b = Vector2::new(2.0, 1.0);
    /// let rot2 = UnitComplex::scaled_rotation_between(&a, &b, 0.2);
    /// let rot5 = UnitComplex::scaled_rotation_between(&a, &b, 0.5);
    /// assert_relative_eq!(rot2 * rot2 * rot2 * rot2 * rot2 * a, b, epsilon = 1.0e-6);
    /// assert_relative_eq!(rot5 * rot5 * a, b, epsilon = 1.0e-6);
    /// ```
    #[inline]
    pub fn scaled_rotation_between<SB, SC>(
        a: &Vector<T, U2, SB>,
        b: &Vector<T, U2, SC>,
        s: T,
    ) -> Self
    where
        T: RealField,
        SB: Storage<T, U2>,
        SC: Storage<T, U2>,
    {
        // TODO: code duplication with Rotation.
        match (
            Unit::try_new(a.clone_owned(), T::zero()),
            Unit::try_new(b.clone_owned(), T::zero()),
        ) {
            (Some(na), Some(nb)) => Self::scaled_rotation_between_axis(&na, &nb, s),
            _ => Self::identity(),
        }
    }

    /// The unit complex needed to make `a` and `b` be collinear and point toward the same
    /// direction.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use nalgebra::{Unit, Vector2, UnitComplex};
    /// let a = Unit::new_normalize(Vector2::new(1.0, 2.0));
    /// let b = Unit::new_normalize(Vector2::new(2.0, 1.0));
    /// let rot = UnitComplex::rotation_between_axis(&a, &b);
    /// assert_relative_eq!(rot * a, b);
    /// assert_relative_eq!(rot.inverse() * b, a);
    /// ```
    #[inline]
    pub fn rotation_between_axis<SB, SC>(
        a: &Unit<Vector<T, U2, SB>>,
        b: &Unit<Vector<T, U2, SC>>,
    ) -> Self
    where
        SB: Storage<T, U2>,
        SC: Storage<T, U2>,
    {
        Self::scaled_rotation_between_axis(a, b, T::one())
    }

    /// The smallest rotation needed to make `a` and `b` collinear and point toward the same
    /// direction, raised to the power `s`.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use nalgebra::{Unit, Vector2, UnitComplex};
    /// let a = Unit::new_normalize(Vector2::new(1.0, 2.0));
    /// let b = Unit::new_normalize(Vector2::new(2.0, 1.0));
    /// let rot2 = UnitComplex::scaled_rotation_between_axis(&a, &b, 0.2);
    /// let rot5 = UnitComplex::scaled_rotation_between_axis(&a, &b, 0.5);
    /// assert_relative_eq!(rot2 * rot2 * rot2 * rot2 * rot2 * a, b, epsilon = 1.0e-6);
    /// assert_relative_eq!(rot5 * rot5 * a, b, epsilon = 1.0e-6);
    /// ```
    #[inline]
    pub fn scaled_rotation_between_axis<SB, SC>(
        na: &Unit<Vector<T, U2, SB>>,
        nb: &Unit<Vector<T, U2, SC>>,
        s: T,
    ) -> Self
    where
        SB: Storage<T, U2>,
        SC: Storage<T, U2>,
    {
        let sang = na.perp(nb);
        let cang = na.dot(nb);

        Self::from_angle(sang.simd_atan2(cang) * s)
    }
}

impl<T: SimdRealField> One for UnitComplex<T>
where
    T::Element: SimdRealField,
{
    #[inline]
    fn one() -> Self {
        Self::identity()
    }
}

#[cfg(feature = "rand")]
impl<T: SimdRealField> Distribution<UnitComplex<T>> for StandardUniform
where
    T::Element: SimdRealField,
    rand_distr::UnitCircle: Distribution<[T; 2]>,
{
    /// Generate a uniformly distributed random `UnitComplex`.
    #[inline]
    fn sample<'a, R: Rng + ?Sized>(&self, rng: &mut R) -> UnitComplex<T> {
        let x = rng.sample(rand_distr::UnitCircle);
        UnitComplex::new_unchecked(Complex::new(x[0].clone(), x[1].clone()))
    }
}

#[cfg(feature = "arbitrary")]
impl<T: SimdRealField + Arbitrary> Arbitrary for UnitComplex<T>
where
    T::Element: SimdRealField,
{
    #[inline]
    fn arbitrary(g: &mut Gen) -> Self {
        UnitComplex::from_angle(T::arbitrary(g))
    }
}
