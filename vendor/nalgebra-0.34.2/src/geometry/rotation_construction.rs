use num::{One, Zero};

use simba::scalar::{ClosedAddAssign, ClosedMulAssign, SupersetOf};

use crate::base::{SMatrix, Scalar};

use crate::geometry::Rotation;

impl<T, const D: usize> Default for Rotation<T, D>
where
    T: Scalar + Zero + One,
{
    fn default() -> Self {
        Self::identity()
    }
}

/// # Identity
impl<T, const D: usize> Rotation<T, D>
where
    T: Scalar + Zero + One,
{
    /// Creates a new square identity rotation of the given `dimension`.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::{Rotation2, Rotation3};
    /// # use nalgebra::Vector3;
    /// let rot1 = Rotation2::identity();
    /// let rot2 = Rotation2::new(std::f32::consts::FRAC_PI_2);
    ///
    /// assert_eq!(rot1 * rot2, rot2);
    /// assert_eq!(rot2 * rot1, rot2);
    ///
    /// let rot1 = Rotation3::identity();
    /// let rot2 = Rotation3::from_axis_angle(&Vector3::z_axis(), std::f32::consts::FRAC_PI_2);
    ///
    /// assert_eq!(rot1 * rot2, rot2);
    /// assert_eq!(rot2 * rot1, rot2);
    /// ```
    #[inline]
    pub fn identity() -> Rotation<T, D> {
        Self::from_matrix_unchecked(SMatrix::<T, D, D>::identity())
    }
}

impl<T: Scalar, const D: usize> Rotation<T, D> {
    /// Cast the components of `self` to another type.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::Rotation2;
    /// let rot = Rotation2::<f64>::identity();
    /// let rot2 = rot.cast::<f32>();
    /// assert_eq!(rot2, Rotation2::<f32>::identity());
    /// ```
    pub fn cast<To: Scalar>(self) -> Rotation<To, D>
    where
        Rotation<To, D>: SupersetOf<Self>,
    {
        crate::convert(self)
    }
}

impl<T, const D: usize> One for Rotation<T, D>
where
    T: Scalar + Zero + One + ClosedAddAssign + ClosedMulAssign,
{
    #[inline]
    fn one() -> Self {
        Self::identity()
    }
}
