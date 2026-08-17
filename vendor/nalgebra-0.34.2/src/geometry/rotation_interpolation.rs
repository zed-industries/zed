use crate::{RealField, Rotation2, Rotation3, SimdRealField, UnitComplex, UnitQuaternion};

/// # Interpolation
impl<T: SimdRealField> Rotation2<T> {
    /// Spherical linear interpolation between two rotation matrices.
    ///
    /// # Examples:
    ///
    /// ```
    /// # #[macro_use] extern crate approx;
    /// # use nalgebra::geometry::Rotation2;
    ///
    /// let rot1 = Rotation2::new(std::f32::consts::FRAC_PI_4);
    /// let rot2 = Rotation2::new(-std::f32::consts::PI);
    ///
    /// let rot = rot1.slerp(&rot2, 1.0 / 3.0);
    ///
    /// assert_relative_eq!(rot.angle(), std::f32::consts::FRAC_PI_2);
    /// ```
    #[inline]
    #[must_use]
    pub fn slerp(&self, other: &Self, t: T) -> Self
    where
        T::Element: SimdRealField,
    {
        let c1 = UnitComplex::from(self.clone());
        let c2 = UnitComplex::from(other.clone());
        c1.slerp(&c2, t).into()
    }
}

impl<T: SimdRealField> Rotation3<T> {
    /// Spherical linear interpolation between two rotation matrices.
    ///
    /// Panics if the angle between both rotations is 180 degrees (in which case the interpolation
    /// is not well-defined). Use `.try_slerp` instead to avoid the panic.
    ///
    /// # Examples:
    ///
    /// ```
    /// # use nalgebra::geometry::Rotation3;
    ///
    /// let q1 = Rotation3::from_euler_angles(std::f32::consts::FRAC_PI_4, 0.0, 0.0);
    /// let q2 = Rotation3::from_euler_angles(-std::f32::consts::PI, 0.0, 0.0);
    ///
    /// let q = q1.slerp(&q2, 1.0 / 3.0);
    ///
    /// assert_eq!(q.euler_angles(), (std::f32::consts::FRAC_PI_2, 0.0, 0.0));
    /// ```
    #[inline]
    #[must_use]
    pub fn slerp(&self, other: &Self, t: T) -> Self
    where
        T: RealField,
    {
        let q1 = UnitQuaternion::from(self.clone());
        let q2 = UnitQuaternion::from(other.clone());
        q1.slerp(&q2, t).into()
    }

    /// Computes the spherical linear interpolation between two rotation matrices or returns `None`
    /// if both rotations are approximately 180 degrees apart (in which case the interpolation is
    /// not well-defined).
    ///
    /// # Arguments
    /// * `self`: the first rotation to interpolate from.
    /// * `other`: the second rotation to interpolate toward.
    /// * `t`: the interpolation parameter. Should be between 0 and 1.
    /// * `epsilon`: the value below which the sinus of the angle separating both rotations
    ///   must be to return `None`.
    #[inline]
    #[must_use]
    pub fn try_slerp(&self, other: &Self, t: T, epsilon: T) -> Option<Self>
    where
        T: RealField,
    {
        let q1 = UnitQuaternion::from(self.clone());
        let q2 = UnitQuaternion::from(other.clone());
        q1.try_slerp(&q2, t, epsilon).map(|q| q.into())
    }
}
