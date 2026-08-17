use crate::{Isometry2, Isometry3, IsometryMatrix2, IsometryMatrix3, RealField, SimdRealField};

/// # Interpolation
impl<T: SimdRealField> Isometry3<T> {
    /// Interpolates between two isometries using a linear interpolation for the translation part,
    /// and a spherical interpolation for the rotation part.
    ///
    /// Panics if the angle between both rotations is 180 degrees (in which case the interpolation
    /// is not well-defined). Use `.try_lerp_slerp` instead to avoid the panic.
    ///
    /// # Examples:
    ///
    /// ```
    /// # use nalgebra::{Vector3, Translation3, Isometry3, UnitQuaternion};
    ///
    /// let t1 = Translation3::new(1.0, 2.0, 3.0);
    /// let t2 = Translation3::new(4.0, 8.0, 12.0);
    /// let q1 = UnitQuaternion::from_euler_angles(std::f32::consts::FRAC_PI_4, 0.0, 0.0);
    /// let q2 = UnitQuaternion::from_euler_angles(-std::f32::consts::PI, 0.0, 0.0);
    /// let iso1 = Isometry3::from_parts(t1, q1);
    /// let iso2 = Isometry3::from_parts(t2, q2);
    ///
    /// let iso3 = iso1.lerp_slerp(&iso2, 1.0 / 3.0);
    ///
    /// assert_eq!(iso3.translation.vector, Vector3::new(2.0, 4.0, 6.0));
    /// assert_eq!(iso3.rotation.euler_angles(), (std::f32::consts::FRAC_PI_2, 0.0, 0.0));
    /// ```
    #[inline]
    #[must_use]
    pub fn lerp_slerp(&self, other: &Self, t: T) -> Self
    where
        T: RealField,
    {
        let tr = self
            .translation
            .vector
            .lerp(&other.translation.vector, t.clone());
        let rot = self.rotation.slerp(&other.rotation, t);
        Self::from_parts(tr.into(), rot)
    }

    /// Attempts to interpolate between two isometries using a linear interpolation for the translation part,
    /// and a spherical interpolation for the rotation part.
    ///
    /// Returns `None` if the angle between both rotations is 180 degrees (in which case the interpolation
    /// is not well-defined).
    ///
    /// # Examples:
    ///
    /// ```
    /// # use nalgebra::{Vector3, Translation3, Isometry3, UnitQuaternion};
    ///
    /// let t1 = Translation3::new(1.0, 2.0, 3.0);
    /// let t2 = Translation3::new(4.0, 8.0, 12.0);
    /// let q1 = UnitQuaternion::from_euler_angles(std::f32::consts::FRAC_PI_4, 0.0, 0.0);
    /// let q2 = UnitQuaternion::from_euler_angles(-std::f32::consts::PI, 0.0, 0.0);
    /// let iso1 = Isometry3::from_parts(t1, q1);
    /// let iso2 = Isometry3::from_parts(t2, q2);
    ///
    /// let iso3 = iso1.lerp_slerp(&iso2, 1.0 / 3.0);
    ///
    /// assert_eq!(iso3.translation.vector, Vector3::new(2.0, 4.0, 6.0));
    /// assert_eq!(iso3.rotation.euler_angles(), (std::f32::consts::FRAC_PI_2, 0.0, 0.0));
    /// ```
    #[inline]
    #[must_use]
    pub fn try_lerp_slerp(&self, other: &Self, t: T, epsilon: T) -> Option<Self>
    where
        T: RealField,
    {
        let tr = self
            .translation
            .vector
            .lerp(&other.translation.vector, t.clone());
        let rot = self.rotation.try_slerp(&other.rotation, t, epsilon)?;
        Some(Self::from_parts(tr.into(), rot))
    }
}

impl<T: SimdRealField> IsometryMatrix3<T> {
    /// Interpolates between two isometries using a linear interpolation for the translation part,
    /// and a spherical interpolation for the rotation part.
    ///
    /// Panics if the angle between both rotations is 180 degrees (in which case the interpolation
    /// is not well-defined). Use `.try_lerp_slerp` instead to avoid the panic.
    ///
    /// # Examples:
    ///
    /// ```
    /// # use nalgebra::{Vector3, Translation3, Rotation3, IsometryMatrix3};
    ///
    /// let t1 = Translation3::new(1.0, 2.0, 3.0);
    /// let t2 = Translation3::new(4.0, 8.0, 12.0);
    /// let q1 = Rotation3::from_euler_angles(std::f32::consts::FRAC_PI_4, 0.0, 0.0);
    /// let q2 = Rotation3::from_euler_angles(-std::f32::consts::PI, 0.0, 0.0);
    /// let iso1 = IsometryMatrix3::from_parts(t1, q1);
    /// let iso2 = IsometryMatrix3::from_parts(t2, q2);
    ///
    /// let iso3 = iso1.lerp_slerp(&iso2, 1.0 / 3.0);
    ///
    /// assert_eq!(iso3.translation.vector, Vector3::new(2.0, 4.0, 6.0));
    /// assert_eq!(iso3.rotation.euler_angles(), (std::f32::consts::FRAC_PI_2, 0.0, 0.0));
    /// ```
    #[inline]
    #[must_use]
    pub fn lerp_slerp(&self, other: &Self, t: T) -> Self
    where
        T: RealField,
    {
        let tr = self
            .translation
            .vector
            .lerp(&other.translation.vector, t.clone());
        let rot = self.rotation.slerp(&other.rotation, t);
        Self::from_parts(tr.into(), rot)
    }

    /// Attempts to interpolate between two isometries using a linear interpolation for the translation part,
    /// and a spherical interpolation for the rotation part.
    ///
    /// Returns `None` if the angle between both rotations is 180 degrees (in which case the interpolation
    /// is not well-defined).
    ///
    /// # Examples:
    ///
    /// ```
    /// # use nalgebra::{Vector3, Translation3, Rotation3, IsometryMatrix3};
    ///
    /// let t1 = Translation3::new(1.0, 2.0, 3.0);
    /// let t2 = Translation3::new(4.0, 8.0, 12.0);
    /// let q1 = Rotation3::from_euler_angles(std::f32::consts::FRAC_PI_4, 0.0, 0.0);
    /// let q2 = Rotation3::from_euler_angles(-std::f32::consts::PI, 0.0, 0.0);
    /// let iso1 = IsometryMatrix3::from_parts(t1, q1);
    /// let iso2 = IsometryMatrix3::from_parts(t2, q2);
    ///
    /// let iso3 = iso1.lerp_slerp(&iso2, 1.0 / 3.0);
    ///
    /// assert_eq!(iso3.translation.vector, Vector3::new(2.0, 4.0, 6.0));
    /// assert_eq!(iso3.rotation.euler_angles(), (std::f32::consts::FRAC_PI_2, 0.0, 0.0));
    /// ```
    #[inline]
    #[must_use]
    pub fn try_lerp_slerp(&self, other: &Self, t: T, epsilon: T) -> Option<Self>
    where
        T: RealField,
    {
        let tr = self
            .translation
            .vector
            .lerp(&other.translation.vector, t.clone());
        let rot = self.rotation.try_slerp(&other.rotation, t, epsilon)?;
        Some(Self::from_parts(tr.into(), rot))
    }
}

impl<T: SimdRealField> Isometry2<T> {
    /// Interpolates between two isometries using a linear interpolation for the translation part,
    /// and a spherical interpolation for the rotation part.
    ///
    /// Panics if the angle between both rotations is 180 degrees (in which case the interpolation
    /// is not well-defined). Use `.try_lerp_slerp` instead to avoid the panic.
    ///
    /// # Examples:
    ///
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use nalgebra::{Vector2, Translation2, UnitComplex, Isometry2};
    ///
    /// let t1 = Translation2::new(1.0, 2.0);
    /// let t2 = Translation2::new(4.0, 8.0);
    /// let q1 = UnitComplex::new(std::f32::consts::FRAC_PI_4);
    /// let q2 = UnitComplex::new(-std::f32::consts::PI);
    /// let iso1 = Isometry2::from_parts(t1, q1);
    /// let iso2 = Isometry2::from_parts(t2, q2);
    ///
    /// let iso3 = iso1.lerp_slerp(&iso2, 1.0 / 3.0);
    ///
    /// assert_eq!(iso3.translation.vector, Vector2::new(2.0, 4.0));
    /// assert_relative_eq!(iso3.rotation.angle(), std::f32::consts::FRAC_PI_2);
    /// ```
    #[inline]
    #[must_use]
    pub fn lerp_slerp(&self, other: &Self, t: T) -> Self
    where
        T: RealField,
    {
        let tr = self
            .translation
            .vector
            .lerp(&other.translation.vector, t.clone());
        let rot = self.rotation.slerp(&other.rotation, t);
        Self::from_parts(tr.into(), rot)
    }
}

impl<T: SimdRealField> IsometryMatrix2<T> {
    /// Interpolates between two isometries using a linear interpolation for the translation part,
    /// and a spherical interpolation for the rotation part.
    ///
    /// Panics if the angle between both rotations is 180 degrees (in which case the interpolation
    /// is not well-defined). Use `.try_lerp_slerp` instead to avoid the panic.
    ///
    /// # Examples:
    ///
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use nalgebra::{Vector2, Translation2, Rotation2, IsometryMatrix2};
    ///
    /// let t1 = Translation2::new(1.0, 2.0);
    /// let t2 = Translation2::new(4.0, 8.0);
    /// let q1 = Rotation2::new(std::f32::consts::FRAC_PI_4);
    /// let q2 = Rotation2::new(-std::f32::consts::PI);
    /// let iso1 = IsometryMatrix2::from_parts(t1, q1);
    /// let iso2 = IsometryMatrix2::from_parts(t2, q2);
    ///
    /// let iso3 = iso1.lerp_slerp(&iso2, 1.0 / 3.0);
    ///
    /// assert_eq!(iso3.translation.vector, Vector2::new(2.0, 4.0));
    /// assert_relative_eq!(iso3.rotation.angle(), std::f32::consts::FRAC_PI_2);
    /// ```
    #[inline]
    #[must_use]
    pub fn lerp_slerp(&self, other: &Self, t: T) -> Self
    where
        T: RealField,
    {
        let tr = self
            .translation
            .vector
            .lerp(&other.translation.vector, t.clone());
        let rot = self.rotation.slerp(&other.rotation, t);
        Self::from_parts(tr.into(), rot)
    }
}
