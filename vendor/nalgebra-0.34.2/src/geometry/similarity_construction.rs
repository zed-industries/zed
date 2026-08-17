#[cfg(feature = "arbitrary")]
use crate::base::storage::Owned;
#[cfg(feature = "arbitrary")]
use quickcheck::{Arbitrary, Gen};

use num::One;
#[cfg(feature = "rand-no-std")]
use rand::{
    Rng,
    distr::{Distribution, StandardUniform},
};

use simba::scalar::SupersetOf;
use simba::simd::SimdRealField;

use crate::base::{Vector2, Vector3};

use crate::{
    AbstractRotation, Isometry, Point, Point3, Rotation2, Rotation3, Scalar, Similarity,
    Translation, UnitComplex, UnitQuaternion,
};

impl<T: SimdRealField, R, const D: usize> Default for Similarity<T, R, D>
where
    T::Element: SimdRealField,
    R: AbstractRotation<T, D>,
{
    fn default() -> Self {
        Self::identity()
    }
}

impl<T: SimdRealField, R, const D: usize> Similarity<T, R, D>
where
    T::Element: SimdRealField,
    R: AbstractRotation<T, D>,
{
    /// Creates a new identity similarity.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::{Similarity2, Point2, Similarity3, Point3};
    ///
    /// let sim = Similarity2::identity();
    /// let pt = Point2::new(1.0, 2.0);
    /// assert_eq!(sim * pt, pt);
    ///
    /// let sim = Similarity3::identity();
    /// let pt = Point3::new(1.0, 2.0, 3.0);
    /// assert_eq!(sim * pt, pt);
    /// ```
    #[inline]
    pub fn identity() -> Self {
        Self::from_isometry(Isometry::identity(), T::one())
    }
}

impl<T: SimdRealField, R, const D: usize> One for Similarity<T, R, D>
where
    T::Element: SimdRealField,
    R: AbstractRotation<T, D>,
{
    /// Creates a new identity similarity.
    #[inline]
    fn one() -> Self {
        Self::identity()
    }
}

#[cfg(feature = "rand-no-std")]
impl<T: crate::RealField, R, const D: usize> Distribution<Similarity<T, R, D>> for StandardUniform
where
    R: AbstractRotation<T, D>,
    StandardUniform: Distribution<T> + Distribution<R>,
{
    /// Generate an arbitrary random variate for testing purposes.
    #[inline]
    fn sample<'a, G: Rng + ?Sized>(&self, rng: &mut G) -> Similarity<T, R, D> {
        let mut s = rng.random();
        while relative_eq!(s, T::zero()) {
            s = rng.random()
        }

        Similarity::from_isometry(rng.random(), s)
    }
}

impl<T: SimdRealField, R, const D: usize> Similarity<T, R, D>
where
    T::Element: SimdRealField,
    R: AbstractRotation<T, D>,
{
    /// The similarity that applies the scaling factor `scaling`, followed by the rotation `r` with
    /// its axis passing through the point `p`.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use std::f32;
    /// # use nalgebra::{Similarity2, Point2, UnitComplex};
    /// let rot = UnitComplex::new(f32::consts::FRAC_PI_2);
    /// let pt = Point2::new(3.0, 2.0);
    /// let sim = Similarity2::rotation_wrt_point(rot, pt, 4.0);
    ///
    /// assert_relative_eq!(sim * Point2::new(1.0, 2.0), Point2::new(-3.0, 3.0), epsilon = 1.0e-6);
    /// ```
    #[inline]
    pub fn rotation_wrt_point(r: R, p: Point<T, D>, scaling: T) -> Self {
        let shift = r.transform_vector(&-&p.coords);
        Self::from_parts(Translation::from(shift + p.coords), r, scaling)
    }
}

#[cfg(feature = "arbitrary")]
impl<T, R, const D: usize> Arbitrary for Similarity<T, R, D>
where
    T: crate::RealField + Arbitrary + Send,
    T::Element: crate::RealField,
    R: AbstractRotation<T, D> + Arbitrary + Send,
    Owned<T, crate::Const<D>>: Send,
{
    #[inline]
    fn arbitrary(rng: &mut Gen) -> Self {
        let mut s: T = Arbitrary::arbitrary(rng);
        while s.is_zero() {
            s = Arbitrary::arbitrary(rng)
        }

        Self::from_isometry(Arbitrary::arbitrary(rng), s)
    }
}

/*
 *
 * Constructors for various static dimensions.
 *
 */

// 2D similarity.
impl<T: SimdRealField> Similarity<T, Rotation2<T>, 2>
where
    T::Element: SimdRealField,
{
    /// Creates a new similarity from a translation, a rotation, and an uniform scaling factor.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use std::f32;
    /// # use nalgebra::{SimilarityMatrix2, Vector2, Point2};
    /// let sim = SimilarityMatrix2::new(Vector2::new(1.0, 2.0), f32::consts::FRAC_PI_2, 3.0);
    ///
    /// assert_relative_eq!(sim * Point2::new(2.0, 4.0), Point2::new(-11.0, 8.0), epsilon = 1.0e-6);
    /// ```
    #[inline]
    pub fn new(translation: Vector2<T>, angle: T, scaling: T) -> Self {
        Self::from_parts(
            Translation::from(translation),
            Rotation2::new(angle),
            scaling,
        )
    }

    /// Cast the components of `self` to another type.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::SimilarityMatrix2;
    /// let sim = SimilarityMatrix2::<f64>::identity();
    /// let sim2 = sim.cast::<f32>();
    /// assert_eq!(sim2, SimilarityMatrix2::<f32>::identity());
    /// ```
    pub fn cast<To: Scalar>(self) -> Similarity<To, Rotation2<To>, 2>
    where
        Similarity<To, Rotation2<To>, 2>: SupersetOf<Self>,
    {
        crate::convert(self)
    }
}

impl<T: SimdRealField> Similarity<T, UnitComplex<T>, 2>
where
    T::Element: SimdRealField,
{
    /// Creates a new similarity from a translation and a rotation angle.
    ///
    /// # Example
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use std::f32;
    /// # use nalgebra::{Similarity2, Vector2, Point2};
    /// let sim = Similarity2::new(Vector2::new(1.0, 2.0), f32::consts::FRAC_PI_2, 3.0);
    ///
    /// assert_relative_eq!(sim * Point2::new(2.0, 4.0), Point2::new(-11.0, 8.0), epsilon = 1.0e-6);
    /// ```
    #[inline]
    pub fn new(translation: Vector2<T>, angle: T, scaling: T) -> Self {
        Self::from_parts(
            Translation::from(translation),
            UnitComplex::new(angle),
            scaling,
        )
    }

    /// Cast the components of `self` to another type.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::Similarity2;
    /// let sim = Similarity2::<f64>::identity();
    /// let sim2 = sim.cast::<f32>();
    /// assert_eq!(sim2, Similarity2::<f32>::identity());
    /// ```
    pub fn cast<To: Scalar>(self) -> Similarity<To, UnitComplex<To>, 2>
    where
        Similarity<To, UnitComplex<To>, 2>: SupersetOf<Self>,
    {
        crate::convert(self)
    }
}

// 3D rotation.
macro_rules! similarity_construction_impl(
    ($Rot: ident) => {
        impl<T: SimdRealField> Similarity<T, $Rot<T>, 3>
        where T::Element: SimdRealField {
            /// Creates a new similarity from a translation, rotation axis-angle, and scaling
            /// factor.
            ///
            /// # Example
            /// ```
            /// # #[macro_use] extern crate approx;
            /// # use std::f32;
            /// # use nalgebra::{Similarity3, SimilarityMatrix3, Point3, Vector3};
            /// let axisangle = Vector3::y() * f32::consts::FRAC_PI_2;
            /// let translation = Vector3::new(1.0, 2.0, 3.0);
            /// // Point and vector being transformed in the tests.
            /// let pt = Point3::new(4.0, 5.0, 6.0);
            /// let vec = Vector3::new(4.0, 5.0, 6.0);
            ///
            /// // Similarity with its rotation part represented as a UnitQuaternion
            /// let sim = Similarity3::new(translation, axisangle, 3.0);
            /// assert_relative_eq!(sim * pt, Point3::new(19.0, 17.0, -9.0), epsilon = 1.0e-5);
            /// assert_relative_eq!(sim * vec, Vector3::new(18.0, 15.0, -12.0), epsilon = 1.0e-5);
            ///
            /// // Similarity with its rotation part represented as a Rotation3 (a 3x3 rotation matrix).
            /// let sim = SimilarityMatrix3::new(translation, axisangle, 3.0);
            /// assert_relative_eq!(sim * pt, Point3::new(19.0, 17.0, -9.0), epsilon = 1.0e-5);
            /// assert_relative_eq!(sim * vec, Vector3::new(18.0, 15.0, -12.0), epsilon = 1.0e-5);
            /// ```
            #[inline]
            pub fn new(translation: Vector3<T>, axisangle: Vector3<T>, scaling: T) -> Self
            {
                Self::from_isometry(Isometry::<_, $Rot<T>, 3>::new(translation, axisangle), scaling)
            }

            /// Cast the components of `self` to another type.
            ///
            /// # Example
            /// ```
            /// # use nalgebra::Similarity3;
            /// let sim = Similarity3::<f64>::identity();
            /// let sim2 = sim.cast::<f32>();
            /// assert_eq!(sim2, Similarity3::<f32>::identity());
            /// ```
            pub fn cast<To: Scalar>(self) -> Similarity<To, $Rot<To>, 3>
            where
                Similarity<To, $Rot<To>, 3>: SupersetOf<Self>,
            {
                crate::convert(self)
            }

            /// Creates an similarity that corresponds to a scaling factor and a local frame of
            /// an observer standing at the point `eye` and looking toward `target`.
            ///
            /// It maps the view direction `target - eye` to the positive `z` axis and the origin to the
            /// `eye`.
            ///
            /// # Arguments
            ///   * eye - The observer position.
            ///   * target - The target position.
            ///   * up - Vertical direction. The only requirement of this parameter is to not be collinear
            ///     to `eye - at`. Non-collinearity is not checked.
            ///
            /// # Example
            /// ```
            /// # #[macro_use] extern crate approx;
            /// # use std::f32;
            /// # use nalgebra::{Similarity3, SimilarityMatrix3, Point3, Vector3};
            /// let eye = Point3::new(1.0, 2.0, 3.0);
            /// let target = Point3::new(2.0, 2.0, 3.0);
            /// let up = Vector3::y();
            ///
            /// // Similarity with its rotation part represented as a UnitQuaternion
            /// let sim = Similarity3::face_towards(&eye, &target, &up, 3.0);
            /// assert_eq!(sim * Point3::origin(), eye);
            /// assert_relative_eq!(sim * Vector3::z(), Vector3::x() * 3.0, epsilon = 1.0e-6);
            ///
            /// // Similarity with its rotation part represented as Rotation3 (a 3x3 rotation matrix).
            /// let sim = SimilarityMatrix3::face_towards(&eye, &target, &up, 3.0);
            /// assert_eq!(sim * Point3::origin(), eye);
            /// assert_relative_eq!(sim * Vector3::z(), Vector3::x() * 3.0, epsilon = 1.0e-6);
            /// ```
            #[inline]
            pub fn face_towards(eye:    &Point3<T>,
                                target: &Point3<T>,
                                up:     &Vector3<T>,
                                scaling: T)
                                -> Self {
                Self::from_isometry(Isometry::<_, $Rot<T>, 3>::face_towards(eye, target, up), scaling)
            }

            /// Deprecated: Use [`SimilarityMatrix3::face_towards`](Self::face_towards) instead.
            #[deprecated(note="renamed to `face_towards`")]
            pub fn new_observer_frames(eye:    &Point3<T>,
                                       target: &Point3<T>,
                                       up:     &Vector3<T>,
                                       scaling: T)
                                       -> Self {
                Self::face_towards(eye, target, up, scaling)
            }

            /// Builds a right-handed look-at view matrix including scaling factor.
            ///
            /// This conforms to the common notion of right handed look-at matrix from the computer
            /// graphics community.
            ///
            /// # Arguments
            ///   * eye - The eye position.
            ///   * target - The target position.
            ///   * up - A vector approximately aligned with required the vertical axis. The only
            ///     requirement of this parameter is to not be collinear to `target - eye`.
            ///
            /// # Example
            /// ```
            /// # #[macro_use] extern crate approx;
            /// # use std::f32;
            /// # use nalgebra::{Similarity3, SimilarityMatrix3, Point3, Vector3};
            /// let eye = Point3::new(1.0, 2.0, 3.0);
            /// let target = Point3::new(2.0, 2.0, 3.0);
            /// let up = Vector3::y();
            ///
            /// // Similarity with its rotation part represented as a UnitQuaternion
            /// let iso = Similarity3::look_at_rh(&eye, &target, &up, 3.0);
            /// assert_relative_eq!(iso * Vector3::x(), -Vector3::z() * 3.0, epsilon = 1.0e-6);
            ///
            /// // Similarity with its rotation part represented as Rotation3 (a 3x3 rotation matrix).
            /// let iso = SimilarityMatrix3::look_at_rh(&eye, &target, &up, 3.0);
            /// assert_relative_eq!(iso * Vector3::x(), -Vector3::z() * 3.0, epsilon = 1.0e-6);
            /// ```
            #[inline]
            pub fn look_at_rh(eye:     &Point3<T>,
                              target:  &Point3<T>,
                              up:      &Vector3<T>,
                              scaling: T)
                              -> Self {
                Self::from_isometry(Isometry::<_, $Rot<T>, 3>::look_at_rh(eye, target, up), scaling)
            }

            /// Builds a left-handed look-at view matrix including a scaling factor.
            ///
            /// This conforms to the common notion of left handed look-at matrix from the computer
            /// graphics community.
            ///
            /// # Arguments
            ///   * eye - The eye position.
            ///   * target - The target position.
            ///   * up - A vector approximately aligned with required the vertical axis. The only
            ///     requirement of this parameter is to not be collinear to `target - eye`.
            ///
            /// # Example
            /// ```
            /// # #[macro_use] extern crate approx;
            /// # use std::f32;
            /// # use nalgebra::{Similarity3, SimilarityMatrix3, Point3, Vector3};
            /// let eye = Point3::new(1.0, 2.0, 3.0);
            /// let target = Point3::new(2.0, 2.0, 3.0);
            /// let up = Vector3::y();
            ///
            /// // Similarity with its rotation part represented as a UnitQuaternion
            /// let sim = Similarity3::look_at_lh(&eye, &target, &up, 3.0);
            /// assert_relative_eq!(sim * Vector3::x(), Vector3::z() * 3.0, epsilon = 1.0e-6);
            ///
            /// // Similarity with its rotation part represented as Rotation3 (a 3x3 rotation matrix).
            /// let sim = SimilarityMatrix3::look_at_lh(&eye, &target, &up, 3.0);
            /// assert_relative_eq!(sim * Vector3::x(), Vector3::z() * 3.0, epsilon = 1.0e-6);
            /// ```
            #[inline]
            pub fn look_at_lh(eye:     &Point3<T>,
                              target:  &Point3<T>,
                              up:      &Vector3<T>,
                              scaling: T)
                              -> Self {
                Self::from_isometry(Isometry::<_, $Rot<T>, 3>::look_at_lh(eye, target, up), scaling)
            }
        }
    }
);

similarity_construction_impl!(Rotation3);
similarity_construction_impl!(UnitQuaternion);
