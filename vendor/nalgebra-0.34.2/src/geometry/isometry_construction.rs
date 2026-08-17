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
    AbstractRotation, Isometry, Isometry2, Isometry3, IsometryMatrix2, IsometryMatrix3, Point,
    Point3, Rotation, Rotation3, Scalar, Translation, Translation2, Translation3, UnitComplex,
    UnitQuaternion,
};

impl<T: SimdRealField, R: AbstractRotation<T, D>, const D: usize> Default for Isometry<T, R, D>
where
    T::Element: SimdRealField,
{
    fn default() -> Self {
        Self::identity()
    }
}

impl<T: SimdRealField, R: AbstractRotation<T, D>, const D: usize> Isometry<T, R, D>
where
    T::Element: SimdRealField,
{
    /// Creates a new identity isometry.
    ///
    /// # Example
    ///
    /// ```
    /// # use nalgebra::{Isometry2, Point2, Isometry3, Point3};
    ///
    /// let iso = Isometry2::identity();
    /// let pt = Point2::new(1.0, 2.0);
    /// assert_eq!(iso * pt, pt);
    ///
    /// let iso = Isometry3::identity();
    /// let pt = Point3::new(1.0, 2.0, 3.0);
    /// assert_eq!(iso * pt, pt);
    /// ```
    #[inline]
    pub fn identity() -> Self {
        Self::from_parts(Translation::identity(), R::identity())
    }

    /// The isometry that applies the rotation `r` with its axis passing through the point `p`.
    /// This effectively lets `p` invariant.
    ///
    /// # Example
    ///
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use std::f32;
    /// # use nalgebra::{Isometry2, Point2, UnitComplex};
    /// let rot = UnitComplex::new(f32::consts::PI);
    /// let pt = Point2::new(1.0, 0.0);
    /// let iso = Isometry2::rotation_wrt_point(rot, pt);
    ///
    /// assert_eq!(iso * pt, pt); // The rotation center is not affected.
    /// assert_relative_eq!(iso * Point2::new(1.0, 2.0), Point2::new(1.0, -2.0), epsilon = 1.0e-6);
    /// ```
    #[inline]
    pub fn rotation_wrt_point(r: R, p: Point<T, D>) -> Self {
        let shift = r.transform_vector(&-&p.coords);
        Self::from_parts(Translation::from(shift + p.coords), r)
    }
}

impl<T: SimdRealField, R: AbstractRotation<T, D>, const D: usize> One for Isometry<T, R, D>
where
    T::Element: SimdRealField,
{
    /// Creates a new identity isometry.
    #[inline]
    fn one() -> Self {
        Self::identity()
    }
}

#[cfg(feature = "rand-no-std")]
impl<T: crate::RealField, R, const D: usize> Distribution<Isometry<T, R, D>> for StandardUniform
where
    R: AbstractRotation<T, D>,
    StandardUniform: Distribution<T> + Distribution<R>,
{
    #[inline]
    fn sample<G: Rng + ?Sized>(&self, rng: &mut G) -> Isometry<T, R, D> {
        Isometry::from_parts(rng.random(), rng.random())
    }
}

#[cfg(feature = "arbitrary")]
impl<T, R, const D: usize> Arbitrary for Isometry<T, R, D>
where
    T: SimdRealField + Arbitrary + Send,
    T::Element: SimdRealField,
    R: AbstractRotation<T, D> + Arbitrary + Send,
    Owned<T, crate::Const<D>>: Send,
{
    #[inline]
    fn arbitrary(rng: &mut Gen) -> Self {
        Self::from_parts(Arbitrary::arbitrary(rng), Arbitrary::arbitrary(rng))
    }
}

/*
 *
 * Constructors for various static dimensions.
 *
 */

/// # Construction from a 2D vector and/or a rotation angle
impl<T: SimdRealField> IsometryMatrix2<T>
where
    T::Element: SimdRealField,
{
    /// Creates a new 2D isometry from a translation and a rotation angle.
    ///
    /// Its rotational part is represented as a 2x2 rotation matrix.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::f32;
    /// # use nalgebra::{Isometry2, Vector2, Point2};
    /// let iso = Isometry2::new(Vector2::new(1.0, 2.0), f32::consts::FRAC_PI_2);
    ///
    /// assert_eq!(iso * Point2::new(3.0, 4.0), Point2::new(-3.0, 5.0));
    /// ```
    #[inline]
    pub fn new(translation: Vector2<T>, angle: T) -> Self {
        Self::from_parts(Translation::from(translation), Rotation::<T, 2>::new(angle))
    }

    /// Creates a new isometry from the given translation coordinates.
    #[inline]
    pub fn translation(x: T, y: T) -> Self {
        Self::new(Vector2::new(x, y), T::zero())
    }

    /// Creates a new isometry from the given rotation angle.
    #[inline]
    pub fn rotation(angle: T) -> Self {
        Self::new(Vector2::zeros(), angle)
    }

    /// Cast the components of `self` to another type.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::IsometryMatrix2;
    /// let iso = IsometryMatrix2::<f64>::identity();
    /// let iso2 = iso.cast::<f32>();
    /// assert_eq!(iso2, IsometryMatrix2::<f32>::identity());
    /// ```
    pub fn cast<To: Scalar>(self) -> IsometryMatrix2<To>
    where
        IsometryMatrix2<To>: SupersetOf<Self>,
    {
        crate::convert(self)
    }
}

impl<T: SimdRealField> Isometry2<T>
where
    T::Element: SimdRealField,
{
    /// Creates a new 2D isometry from a translation and a rotation angle.
    ///
    /// Its rotational part is represented as an unit complex number.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::f32;
    /// # use nalgebra::{IsometryMatrix2, Point2, Vector2};
    /// let iso = IsometryMatrix2::new(Vector2::new(1.0, 2.0), f32::consts::FRAC_PI_2);
    ///
    /// assert_eq!(iso * Point2::new(3.0, 4.0), Point2::new(-3.0, 5.0));
    /// ```
    #[inline]
    pub fn new(translation: Vector2<T>, angle: T) -> Self {
        Self::from_parts(
            Translation::from(translation),
            UnitComplex::from_angle(angle),
        )
    }

    /// Creates a new isometry from the given translation coordinates.
    #[inline]
    pub fn translation(x: T, y: T) -> Self {
        Self::from_parts(Translation2::new(x, y), UnitComplex::identity())
    }

    /// Creates a new isometry from the given rotation angle.
    #[inline]
    pub fn rotation(angle: T) -> Self {
        Self::new(Vector2::zeros(), angle)
    }

    /// Cast the components of `self` to another type.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::Isometry2;
    /// let iso = Isometry2::<f64>::identity();
    /// let iso2 = iso.cast::<f32>();
    /// assert_eq!(iso2, Isometry2::<f32>::identity());
    /// ```
    pub fn cast<To: Scalar>(self) -> Isometry2<To>
    where
        Isometry2<To>: SupersetOf<Self>,
    {
        crate::convert(self)
    }
}

// 3D rotation.
macro_rules! basic_isometry_construction_impl(
    ($RotId: ident < $($RotParams: ident),*>) => {
        /// Creates a new isometry from a translation and a rotation axis-angle.
        ///
        /// # Example
        ///
        /// ```
        /// # #[macro_use] extern crate approx;
        /// # use std::f32;
        /// # use nalgebra::{Isometry3, IsometryMatrix3, Point3, Vector3};
        /// let axisangle = Vector3::y() * f32::consts::FRAC_PI_2;
        /// let translation = Vector3::new(1.0, 2.0, 3.0);
        /// // Point and vector being transformed in the tests.
        /// let pt = Point3::new(4.0, 5.0, 6.0);
        /// let vec = Vector3::new(4.0, 5.0, 6.0);
        ///
        /// // Isometry with its rotation part represented as a UnitQuaternion
        /// let iso = Isometry3::new(translation, axisangle);
        /// assert_relative_eq!(iso * pt, Point3::new(7.0, 7.0, -1.0), epsilon = 1.0e-6);
        /// assert_relative_eq!(iso * vec, Vector3::new(6.0, 5.0, -4.0), epsilon = 1.0e-6);
        ///
        /// // Isometry with its rotation part represented as a Rotation3 (a 3x3 rotation matrix).
        /// let iso = IsometryMatrix3::new(translation, axisangle);
        /// assert_relative_eq!(iso * pt, Point3::new(7.0, 7.0, -1.0), epsilon = 1.0e-6);
        /// assert_relative_eq!(iso * vec, Vector3::new(6.0, 5.0, -4.0), epsilon = 1.0e-6);
        /// ```
        #[inline]
        pub fn new(translation: Vector3<T>, axisangle: Vector3<T>) -> Self {
            Self::from_parts(
                Translation::from(translation),
                $RotId::<$($RotParams),*>::from_scaled_axis(axisangle))
        }

        /// Creates a new isometry from the given translation coordinates.
        #[inline]
        pub fn translation(x: T, y: T, z: T) -> Self {
            Self::from_parts(Translation3::new(x, y, z), $RotId::identity())
        }

        /// Creates a new isometry from the given rotation angle.
        #[inline]
        pub fn rotation(axisangle: Vector3<T>) -> Self {
            Self::new(Vector3::zeros(), axisangle)
        }
    }
);

macro_rules! look_at_isometry_construction_impl(
    ($RotId: ident < $($RotParams: ident),*>) => {
        /// Creates an isometry that corresponds to the local frame of an observer standing at the
        /// point `eye` and looking toward `target`.
        ///
        /// It maps the `z` axis to the view direction `target - eye`and the origin to the `eye`.
        ///
        /// # Arguments
        ///   * eye - The observer position.
        ///   * target - The target position.
        ///   * up - Vertical direction. The only requirement of this parameter is to not be collinear
        ///     to `eye - at`. Non-collinearity is not checked.
        ///
        /// # Example
        ///
        /// ```
        /// # #[macro_use] extern crate approx;
        /// # use std::f32;
        /// # use nalgebra::{Isometry3, IsometryMatrix3, Point3, Vector3};
        /// let eye = Point3::new(1.0, 2.0, 3.0);
        /// let target = Point3::new(2.0, 2.0, 3.0);
        /// let up = Vector3::y();
        ///
        /// // Isometry with its rotation part represented as a UnitQuaternion
        /// let iso = Isometry3::face_towards(&eye, &target, &up);
        /// assert_eq!(iso * Point3::origin(), eye);
        /// assert_relative_eq!(iso * Vector3::z(), Vector3::x());
        ///
        /// // Isometry with its rotation part represented as Rotation3 (a 3x3 rotation matrix).
        /// let iso = IsometryMatrix3::face_towards(&eye, &target, &up);
        /// assert_eq!(iso * Point3::origin(), eye);
        /// assert_relative_eq!(iso * Vector3::z(), Vector3::x());
        /// ```
        #[inline]
        pub fn face_towards(eye:    &Point3<T>,
                            target: &Point3<T>,
                            up:     &Vector3<T>)
                            -> Self {
            Self::from_parts(
                Translation::from(eye.coords.clone()),
                $RotId::face_towards(&(target - eye), up))
        }

        /// Deprecated: Use [`Isometry::face_towards`] instead.
        #[deprecated(note="renamed to `face_towards`")]
        pub fn new_observer_frame(eye:    &Point3<T>,
                                  target: &Point3<T>,
                                  up:     &Vector3<T>)
                                  -> Self {
            Self::face_towards(eye, target, up)
        }

        /// Builds a right-handed look-at view matrix.
        ///
        /// It maps the view direction `target - eye` to the **negative** `z` axis to and the `eye` to the origin.
        /// This conforms to the common notion of right handed camera look-at **view matrix** from
        /// the computer graphics community, i.e. the camera is assumed to look toward its local `-z` axis.
        ///
        /// # Arguments
        ///   * eye - The eye position.
        ///   * target - The target position.
        ///   * up - A vector approximately aligned with required the vertical axis. The only
        ///     requirement of this parameter is to not be collinear to `target - eye`.
        ///
        /// # Example
        ///
        /// ```
        /// # #[macro_use] extern crate approx;
        /// # use std::f32;
        /// # use nalgebra::{Isometry3, IsometryMatrix3, Point3, Vector3};
        /// let eye = Point3::new(1.0, 2.0, 3.0);
        /// let target = Point3::new(2.0, 2.0, 3.0);
        /// let up = Vector3::y();
        ///
        /// // Isometry with its rotation part represented as a UnitQuaternion
        /// let iso = Isometry3::look_at_rh(&eye, &target, &up);
        /// assert_eq!(iso * eye, Point3::origin());
        /// assert_relative_eq!(iso * Vector3::x(), -Vector3::z());
        ///
        /// // Isometry with its rotation part represented as Rotation3 (a 3x3 rotation matrix).
        /// let iso = IsometryMatrix3::look_at_rh(&eye, &target, &up);
        /// assert_eq!(iso * eye, Point3::origin());
        /// assert_relative_eq!(iso * Vector3::x(), -Vector3::z());
        /// ```
        #[inline]
        pub fn look_at_rh(eye:    &Point3<T>,
                          target: &Point3<T>,
                          up:     &Vector3<T>)
                          -> Self {
            let rotation = $RotId::look_at_rh(&(target - eye), up);
            let trans    = &rotation * (-eye);

            Self::from_parts(Translation::from(trans.coords), rotation)
        }

        /// Builds a left-handed look-at view matrix.
        ///
        /// It maps the view direction `target - eye` to the **positive** `z` axis and the `eye` to the origin.
        /// This conforms to the common notion of left handed camera look-at **view matrix** from
        /// the computer graphics community, i.e. the camera is assumed to look toward its local `z` axis.
        ///
        /// # Arguments
        ///   * eye - The eye position.
        ///   * target - The target position.
        ///   * up - A vector approximately aligned with required the vertical axis. The only
        ///     requirement of this parameter is to not be collinear to `target - eye`.
        ///
        /// # Example
        ///
        /// ```
        /// # #[macro_use] extern crate approx;
        /// # use std::f32;
        /// # use nalgebra::{Isometry3, IsometryMatrix3, Point3, Vector3};
        /// let eye = Point3::new(1.0, 2.0, 3.0);
        /// let target = Point3::new(2.0, 2.0, 3.0);
        /// let up = Vector3::y();
        ///
        /// // Isometry with its rotation part represented as a UnitQuaternion
        /// let iso = Isometry3::look_at_lh(&eye, &target, &up);
        /// assert_eq!(iso * eye, Point3::origin());
        /// assert_relative_eq!(iso * Vector3::x(), Vector3::z());
        ///
        /// // Isometry with its rotation part represented as Rotation3 (a 3x3 rotation matrix).
        /// let iso = IsometryMatrix3::look_at_lh(&eye, &target, &up);
        /// assert_eq!(iso * eye, Point3::origin());
        /// assert_relative_eq!(iso * Vector3::x(), Vector3::z());
        /// ```
        #[inline]
        pub fn look_at_lh(eye:    &Point3<T>,
                          target: &Point3<T>,
                          up:     &Vector3<T>)
                          -> Self {
            let rotation = $RotId::look_at_lh(&(target - eye), up);
            let trans    = &rotation * (-eye);

            Self::from_parts(Translation::from(trans.coords), rotation)
        }
    }
);

/// # Construction from a 3D vector and/or an axis-angle
impl<T: SimdRealField> Isometry3<T>
where
    T::Element: SimdRealField,
{
    basic_isometry_construction_impl!(UnitQuaternion<T>);

    /// Cast the components of `self` to another type.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::Isometry3;
    /// let iso = Isometry3::<f64>::identity();
    /// let iso2 = iso.cast::<f32>();
    /// assert_eq!(iso2, Isometry3::<f32>::identity());
    /// ```
    pub fn cast<To: Scalar>(self) -> Isometry3<To>
    where
        Isometry3<To>: SupersetOf<Self>,
    {
        crate::convert(self)
    }
}

impl<T: SimdRealField> IsometryMatrix3<T>
where
    T::Element: SimdRealField,
{
    basic_isometry_construction_impl!(Rotation3<T>);

    /// Cast the components of `self` to another type.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::IsometryMatrix3;
    /// let iso = IsometryMatrix3::<f64>::identity();
    /// let iso2 = iso.cast::<f32>();
    /// assert_eq!(iso2, IsometryMatrix3::<f32>::identity());
    /// ```
    pub fn cast<To: Scalar>(self) -> IsometryMatrix3<To>
    where
        IsometryMatrix3<To>: SupersetOf<Self>,
    {
        crate::convert(self)
    }
}

/// # Construction from a 3D eye position and target point
impl<T: SimdRealField> Isometry3<T>
where
    T::Element: SimdRealField,
{
    look_at_isometry_construction_impl!(UnitQuaternion<T>);
}

impl<T: SimdRealField> IsometryMatrix3<T>
where
    T::Element: SimdRealField,
{
    look_at_isometry_construction_impl!(Rotation3<T>);
}
