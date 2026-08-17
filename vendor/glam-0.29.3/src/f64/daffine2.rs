// Generated from affine.rs.tera template. Edit the template, not the generated file.

use crate::{DMat2, DMat3, DVec2};
use core::ops::{Deref, DerefMut, Mul, MulAssign};

/// A 2D affine transform, which can represent translation, rotation, scaling and shear.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DAffine2 {
    pub matrix2: DMat2,
    pub translation: DVec2,
}

impl DAffine2 {
    /// The degenerate zero transform.
    ///
    /// This transforms any finite vector and point to zero.
    /// The zero transform is non-invertible.
    pub const ZERO: Self = Self {
        matrix2: DMat2::ZERO,
        translation: DVec2::ZERO,
    };

    /// The identity transform.
    ///
    /// Multiplying a vector with this returns the same vector.
    pub const IDENTITY: Self = Self {
        matrix2: DMat2::IDENTITY,
        translation: DVec2::ZERO,
    };

    /// All NAN:s.
    pub const NAN: Self = Self {
        matrix2: DMat2::NAN,
        translation: DVec2::NAN,
    };

    /// Creates an affine transform from three column vectors.
    #[inline(always)]
    #[must_use]
    pub const fn from_cols(x_axis: DVec2, y_axis: DVec2, z_axis: DVec2) -> Self {
        Self {
            matrix2: DMat2::from_cols(x_axis, y_axis),
            translation: z_axis,
        }
    }

    /// Creates an affine transform from a `[f64; 6]` array stored in column major order.
    #[inline]
    #[must_use]
    pub fn from_cols_array(m: &[f64; 6]) -> Self {
        Self {
            matrix2: DMat2::from_cols_array(&[m[0], m[1], m[2], m[3]]),
            translation: DVec2::from_array([m[4], m[5]]),
        }
    }

    /// Creates a `[f64; 6]` array storing data in column major order.
    #[inline]
    #[must_use]
    pub fn to_cols_array(&self) -> [f64; 6] {
        let x = &self.matrix2.x_axis;
        let y = &self.matrix2.y_axis;
        let z = &self.translation;
        [x.x, x.y, y.x, y.y, z.x, z.y]
    }

    /// Creates an affine transform from a `[[f64; 2]; 3]`
    /// 2D array stored in column major order.
    /// If your data is in row major order you will need to `transpose` the returned
    /// matrix.
    #[inline]
    #[must_use]
    pub fn from_cols_array_2d(m: &[[f64; 2]; 3]) -> Self {
        Self {
            matrix2: DMat2::from_cols(m[0].into(), m[1].into()),
            translation: m[2].into(),
        }
    }

    /// Creates a `[[f64; 2]; 3]` 2D array storing data in
    /// column major order.
    /// If you require data in row major order `transpose` the matrix first.
    #[inline]
    #[must_use]
    pub fn to_cols_array_2d(&self) -> [[f64; 2]; 3] {
        [
            self.matrix2.x_axis.into(),
            self.matrix2.y_axis.into(),
            self.translation.into(),
        ]
    }

    /// Creates an affine transform from the first 6 values in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than 6 elements long.
    #[inline]
    #[must_use]
    pub fn from_cols_slice(slice: &[f64]) -> Self {
        Self {
            matrix2: DMat2::from_cols_slice(&slice[0..4]),
            translation: DVec2::from_slice(&slice[4..6]),
        }
    }

    /// Writes the columns of `self` to the first 6 elements in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than 6 elements long.
    #[inline]
    pub fn write_cols_to_slice(self, slice: &mut [f64]) {
        self.matrix2.write_cols_to_slice(&mut slice[0..4]);
        self.translation.write_to_slice(&mut slice[4..6]);
    }

    /// Creates an affine transform that changes scale.
    /// Note that if any scale is zero the transform will be non-invertible.
    #[inline]
    #[must_use]
    pub fn from_scale(scale: DVec2) -> Self {
        Self {
            matrix2: DMat2::from_diagonal(scale),
            translation: DVec2::ZERO,
        }
    }

    /// Creates an affine transform from the given rotation `angle`.
    #[inline]
    #[must_use]
    pub fn from_angle(angle: f64) -> Self {
        Self {
            matrix2: DMat2::from_angle(angle),
            translation: DVec2::ZERO,
        }
    }

    /// Creates an affine transformation from the given 2D `translation`.
    #[inline]
    #[must_use]
    pub fn from_translation(translation: DVec2) -> Self {
        Self {
            matrix2: DMat2::IDENTITY,
            translation,
        }
    }

    /// Creates an affine transform from a 2x2 matrix (expressing scale, shear and rotation)
    #[inline]
    #[must_use]
    pub fn from_mat2(matrix2: DMat2) -> Self {
        Self {
            matrix2,
            translation: DVec2::ZERO,
        }
    }

    /// Creates an affine transform from a 2x2 matrix (expressing scale, shear and rotation) and a
    /// translation vector.
    ///
    /// Equivalent to
    /// `DAffine2::from_translation(translation) * DAffine2::from_mat2(mat2)`
    #[inline]
    #[must_use]
    pub fn from_mat2_translation(matrix2: DMat2, translation: DVec2) -> Self {
        Self {
            matrix2,
            translation,
        }
    }

    /// Creates an affine transform from the given 2D `scale`, rotation `angle` (in radians) and
    /// `translation`.
    ///
    /// Equivalent to `DAffine2::from_translation(translation) *
    /// DAffine2::from_angle(angle) * DAffine2::from_scale(scale)`
    #[inline]
    #[must_use]
    pub fn from_scale_angle_translation(scale: DVec2, angle: f64, translation: DVec2) -> Self {
        let rotation = DMat2::from_angle(angle);
        Self {
            matrix2: DMat2::from_cols(rotation.x_axis * scale.x, rotation.y_axis * scale.y),
            translation,
        }
    }

    /// Creates an affine transform from the given 2D rotation `angle` (in radians) and
    /// `translation`.
    ///
    /// Equivalent to `DAffine2::from_translation(translation) * DAffine2::from_angle(angle)`
    #[inline]
    #[must_use]
    pub fn from_angle_translation(angle: f64, translation: DVec2) -> Self {
        Self {
            matrix2: DMat2::from_angle(angle),
            translation,
        }
    }

    /// The given `DMat3` must be an affine transform,
    #[inline]
    #[must_use]
    pub fn from_mat3(m: DMat3) -> Self {
        use crate::swizzles::Vec3Swizzles;
        Self {
            matrix2: DMat2::from_cols(m.x_axis.xy(), m.y_axis.xy()),
            translation: m.z_axis.xy(),
        }
    }

    /// Extracts `scale`, `angle` and `translation` from `self`.
    ///
    /// The transform is expected to be non-degenerate and without shearing, or the output
    /// will be invalid.
    ///
    /// # Panics
    ///
    /// Will panic if the determinant `self.matrix2` is zero or if the resulting scale
    /// vector contains any zero elements when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn to_scale_angle_translation(self) -> (DVec2, f64, DVec2) {
        use crate::f64::math;
        let det = self.matrix2.determinant();
        glam_assert!(det != 0.0);

        let scale = DVec2::new(
            self.matrix2.x_axis.length() * math::signum(det),
            self.matrix2.y_axis.length(),
        );

        glam_assert!(scale.cmpne(DVec2::ZERO).all());

        let angle = math::atan2(-self.matrix2.y_axis.x, self.matrix2.y_axis.y);

        (scale, angle, self.translation)
    }

    /// Transforms the given 2D point, applying shear, scale, rotation and translation.
    #[inline]
    #[must_use]
    pub fn transform_point2(&self, rhs: DVec2) -> DVec2 {
        self.matrix2 * rhs + self.translation
    }

    /// Transforms the given 2D vector, applying shear, scale and rotation (but NOT
    /// translation).
    ///
    /// To also apply translation, use [`Self::transform_point2()`] instead.
    #[inline]
    pub fn transform_vector2(&self, rhs: DVec2) -> DVec2 {
        self.matrix2 * rhs
    }

    /// Returns `true` if, and only if, all elements are finite.
    ///
    /// If any element is either `NaN`, positive or negative infinity, this will return
    /// `false`.
    #[inline]
    #[must_use]
    pub fn is_finite(&self) -> bool {
        self.matrix2.is_finite() && self.translation.is_finite()
    }

    /// Returns `true` if any elements are `NaN`.
    #[inline]
    #[must_use]
    pub fn is_nan(&self) -> bool {
        self.matrix2.is_nan() || self.translation.is_nan()
    }

    /// Returns true if the absolute difference of all elements between `self` and `rhs`
    /// is less than or equal to `max_abs_diff`.
    ///
    /// This can be used to compare if two 3x4 matrices contain similar elements. It works
    /// best when comparing with a known value. The `max_abs_diff` that should be used used
    /// depends on the values being compared against.
    ///
    /// For more see
    /// [comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
    #[inline]
    #[must_use]
    pub fn abs_diff_eq(&self, rhs: Self, max_abs_diff: f64) -> bool {
        self.matrix2.abs_diff_eq(rhs.matrix2, max_abs_diff)
            && self.translation.abs_diff_eq(rhs.translation, max_abs_diff)
    }

    /// Return the inverse of this transform.
    ///
    /// Note that if the transform is not invertible the result will be invalid.
    #[inline]
    #[must_use]
    pub fn inverse(&self) -> Self {
        let matrix2 = self.matrix2.inverse();
        // transform negative translation by the matrix inverse:
        let translation = -(matrix2 * self.translation);

        Self {
            matrix2,
            translation,
        }
    }
}

impl Default for DAffine2 {
    #[inline(always)]
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Deref for DAffine2 {
    type Target = crate::deref::Cols3<DVec2>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const Self as *const Self::Target) }
    }
}

impl DerefMut for DAffine2 {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self as *mut Self as *mut Self::Target) }
    }
}

impl PartialEq for DAffine2 {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        self.matrix2.eq(&rhs.matrix2) && self.translation.eq(&rhs.translation)
    }
}

impl core::fmt::Debug for DAffine2 {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        fmt.debug_struct(stringify!(DAffine2))
            .field("matrix2", &self.matrix2)
            .field("translation", &self.translation)
            .finish()
    }
}

impl core::fmt::Display for DAffine2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if let Some(p) = f.precision() {
            write!(
                f,
                "[{:.*}, {:.*}, {:.*}]",
                p, self.matrix2.x_axis, p, self.matrix2.y_axis, p, self.translation
            )
        } else {
            write!(
                f,
                "[{}, {}, {}]",
                self.matrix2.x_axis, self.matrix2.y_axis, self.translation
            )
        }
    }
}

impl<'a> core::iter::Product<&'a Self> for DAffine2 {
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::IDENTITY, |a, &b| a * b)
    }
}

impl Mul for DAffine2 {
    type Output = DAffine2;

    #[inline]
    fn mul(self, rhs: DAffine2) -> Self::Output {
        Self {
            matrix2: self.matrix2 * rhs.matrix2,
            translation: self.matrix2 * rhs.translation + self.translation,
        }
    }
}

impl MulAssign for DAffine2 {
    #[inline]
    fn mul_assign(&mut self, rhs: DAffine2) {
        *self = self.mul(rhs);
    }
}

impl From<DAffine2> for DMat3 {
    #[inline]
    fn from(m: DAffine2) -> DMat3 {
        Self::from_cols(
            m.matrix2.x_axis.extend(0.0),
            m.matrix2.y_axis.extend(0.0),
            m.translation.extend(1.0),
        )
    }
}

impl Mul<DMat3> for DAffine2 {
    type Output = DMat3;

    #[inline]
    fn mul(self, rhs: DMat3) -> Self::Output {
        DMat3::from(self) * rhs
    }
}

impl Mul<DAffine2> for DMat3 {
    type Output = DMat3;

    #[inline]
    fn mul(self, rhs: DAffine2) -> Self::Output {
        self * DMat3::from(rhs)
    }
}
