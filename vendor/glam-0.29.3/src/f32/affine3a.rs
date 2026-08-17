// Generated from affine.rs.tera template. Edit the template, not the generated file.

use crate::{Mat3, Mat3A, Mat4, Quat, Vec3, Vec3A};
use core::ops::{Deref, DerefMut, Mul, MulAssign};

/// A 3D affine transform, which can represent translation, rotation, scaling and shear.
///
/// This type is 16 byte aligned.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Affine3A {
    pub matrix3: Mat3A,
    pub translation: Vec3A,
}

impl Affine3A {
    /// The degenerate zero transform.
    ///
    /// This transforms any finite vector and point to zero.
    /// The zero transform is non-invertible.
    pub const ZERO: Self = Self {
        matrix3: Mat3A::ZERO,
        translation: Vec3A::ZERO,
    };

    /// The identity transform.
    ///
    /// Multiplying a vector with this returns the same vector.
    pub const IDENTITY: Self = Self {
        matrix3: Mat3A::IDENTITY,
        translation: Vec3A::ZERO,
    };

    /// All NAN:s.
    pub const NAN: Self = Self {
        matrix3: Mat3A::NAN,
        translation: Vec3A::NAN,
    };

    /// Creates an affine transform from three column vectors.
    #[inline(always)]
    #[must_use]
    pub const fn from_cols(x_axis: Vec3A, y_axis: Vec3A, z_axis: Vec3A, w_axis: Vec3A) -> Self {
        Self {
            matrix3: Mat3A::from_cols(x_axis, y_axis, z_axis),
            translation: w_axis,
        }
    }

    /// Creates an affine transform from a `[f32; 12]` array stored in column major order.
    #[inline]
    #[must_use]
    pub fn from_cols_array(m: &[f32; 12]) -> Self {
        Self {
            matrix3: Mat3A::from_cols_array(&[
                m[0], m[1], m[2], m[3], m[4], m[5], m[6], m[7], m[8],
            ]),
            translation: Vec3A::from_array([m[9], m[10], m[11]]),
        }
    }

    /// Creates a `[f32; 12]` array storing data in column major order.
    #[inline]
    #[must_use]
    pub fn to_cols_array(&self) -> [f32; 12] {
        let x = &self.matrix3.x_axis;
        let y = &self.matrix3.y_axis;
        let z = &self.matrix3.z_axis;
        let w = &self.translation;
        [x.x, x.y, x.z, y.x, y.y, y.z, z.x, z.y, z.z, w.x, w.y, w.z]
    }

    /// Creates an affine transform from a `[[f32; 3]; 4]`
    /// 3D array stored in column major order.
    /// If your data is in row major order you will need to `transpose` the returned
    /// matrix.
    #[inline]
    #[must_use]
    pub fn from_cols_array_2d(m: &[[f32; 3]; 4]) -> Self {
        Self {
            matrix3: Mat3A::from_cols(m[0].into(), m[1].into(), m[2].into()),
            translation: m[3].into(),
        }
    }

    /// Creates a `[[f32; 3]; 4]` 3D array storing data in
    /// column major order.
    /// If you require data in row major order `transpose` the matrix first.
    #[inline]
    #[must_use]
    pub fn to_cols_array_2d(&self) -> [[f32; 3]; 4] {
        [
            self.matrix3.x_axis.into(),
            self.matrix3.y_axis.into(),
            self.matrix3.z_axis.into(),
            self.translation.into(),
        ]
    }

    /// Creates an affine transform from the first 12 values in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than 12 elements long.
    #[inline]
    #[must_use]
    pub fn from_cols_slice(slice: &[f32]) -> Self {
        Self {
            matrix3: Mat3A::from_cols_slice(&slice[0..9]),
            translation: Vec3A::from_slice(&slice[9..12]),
        }
    }

    /// Writes the columns of `self` to the first 12 elements in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than 12 elements long.
    #[inline]
    pub fn write_cols_to_slice(self, slice: &mut [f32]) {
        self.matrix3.write_cols_to_slice(&mut slice[0..9]);
        self.translation.write_to_slice(&mut slice[9..12]);
    }

    /// Creates an affine transform that changes scale.
    /// Note that if any scale is zero the transform will be non-invertible.
    #[inline]
    #[must_use]
    pub fn from_scale(scale: Vec3) -> Self {
        Self {
            matrix3: Mat3A::from_diagonal(scale),
            translation: Vec3A::ZERO,
        }
    }
    /// Creates an affine transform from the given `rotation` quaternion.
    #[inline]
    #[must_use]
    pub fn from_quat(rotation: Quat) -> Self {
        Self {
            matrix3: Mat3A::from_quat(rotation),
            translation: Vec3A::ZERO,
        }
    }

    /// Creates an affine transform containing a 3D rotation around a normalized
    /// rotation `axis` of `angle` (in radians).
    #[inline]
    #[must_use]
    pub fn from_axis_angle(axis: Vec3, angle: f32) -> Self {
        Self {
            matrix3: Mat3A::from_axis_angle(axis, angle),
            translation: Vec3A::ZERO,
        }
    }

    /// Creates an affine transform containing a 3D rotation around the x axis of
    /// `angle` (in radians).
    #[inline]
    #[must_use]
    pub fn from_rotation_x(angle: f32) -> Self {
        Self {
            matrix3: Mat3A::from_rotation_x(angle),
            translation: Vec3A::ZERO,
        }
    }

    /// Creates an affine transform containing a 3D rotation around the y axis of
    /// `angle` (in radians).
    #[inline]
    #[must_use]
    pub fn from_rotation_y(angle: f32) -> Self {
        Self {
            matrix3: Mat3A::from_rotation_y(angle),
            translation: Vec3A::ZERO,
        }
    }

    /// Creates an affine transform containing a 3D rotation around the z axis of
    /// `angle` (in radians).
    #[inline]
    #[must_use]
    pub fn from_rotation_z(angle: f32) -> Self {
        Self {
            matrix3: Mat3A::from_rotation_z(angle),
            translation: Vec3A::ZERO,
        }
    }

    /// Creates an affine transformation from the given 3D `translation`.
    #[inline]
    #[must_use]
    pub fn from_translation(translation: Vec3) -> Self {
        #[allow(clippy::useless_conversion)]
        Self {
            matrix3: Mat3A::IDENTITY,
            translation: translation.into(),
        }
    }

    /// Creates an affine transform from a 3x3 matrix (expressing scale, shear and
    /// rotation)
    #[inline]
    #[must_use]
    pub fn from_mat3(mat3: Mat3) -> Self {
        #[allow(clippy::useless_conversion)]
        Self {
            matrix3: mat3.into(),
            translation: Vec3A::ZERO,
        }
    }

    /// Creates an affine transform from a 3x3 matrix (expressing scale, shear and rotation)
    /// and a translation vector.
    ///
    /// Equivalent to `Affine3A::from_translation(translation) * Affine3A::from_mat3(mat3)`
    #[inline]
    #[must_use]
    pub fn from_mat3_translation(mat3: Mat3, translation: Vec3) -> Self {
        #[allow(clippy::useless_conversion)]
        Self {
            matrix3: mat3.into(),
            translation: translation.into(),
        }
    }

    /// Creates an affine transform from the given 3D `scale`, `rotation` and
    /// `translation`.
    ///
    /// Equivalent to `Affine3A::from_translation(translation) *
    /// Affine3A::from_quat(rotation) * Affine3A::from_scale(scale)`
    #[inline]
    #[must_use]
    pub fn from_scale_rotation_translation(scale: Vec3, rotation: Quat, translation: Vec3) -> Self {
        let rotation = Mat3A::from_quat(rotation);
        #[allow(clippy::useless_conversion)]
        Self {
            matrix3: Mat3A::from_cols(
                rotation.x_axis * scale.x,
                rotation.y_axis * scale.y,
                rotation.z_axis * scale.z,
            ),
            translation: translation.into(),
        }
    }

    /// Creates an affine transform from the given 3D `rotation` and `translation`.
    ///
    /// Equivalent to `Affine3A::from_translation(translation) * Affine3A::from_quat(rotation)`
    #[inline]
    #[must_use]
    pub fn from_rotation_translation(rotation: Quat, translation: Vec3) -> Self {
        #[allow(clippy::useless_conversion)]
        Self {
            matrix3: Mat3A::from_quat(rotation),
            translation: translation.into(),
        }
    }

    /// The given `Mat4` must be an affine transform,
    /// i.e. contain no perspective transform.
    #[inline]
    #[must_use]
    pub fn from_mat4(m: Mat4) -> Self {
        Self {
            matrix3: Mat3A::from_cols(
                Vec3A::from_vec4(m.x_axis),
                Vec3A::from_vec4(m.y_axis),
                Vec3A::from_vec4(m.z_axis),
            ),
            translation: Vec3A::from_vec4(m.w_axis),
        }
    }

    /// Extracts `scale`, `rotation` and `translation` from `self`.
    ///
    /// The transform is expected to be non-degenerate and without shearing, or the output
    /// will be invalid.
    ///
    /// # Panics
    ///
    /// Will panic if the determinant `self.matrix3` is zero or if the resulting scale
    /// vector contains any zero elements when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn to_scale_rotation_translation(&self) -> (Vec3, Quat, Vec3) {
        use crate::f32::math;
        let det = self.matrix3.determinant();
        glam_assert!(det != 0.0);

        let scale = Vec3::new(
            self.matrix3.x_axis.length() * math::signum(det),
            self.matrix3.y_axis.length(),
            self.matrix3.z_axis.length(),
        );

        glam_assert!(scale.cmpne(Vec3::ZERO).all());

        let inv_scale = scale.recip();

        #[allow(clippy::useless_conversion)]
        let rotation = Quat::from_mat3(&Mat3::from_cols(
            (self.matrix3.x_axis * inv_scale.x).into(),
            (self.matrix3.y_axis * inv_scale.y).into(),
            (self.matrix3.z_axis * inv_scale.z).into(),
        ));

        #[allow(clippy::useless_conversion)]
        (scale, rotation, self.translation.into())
    }

    /// Creates a left-handed view transform using a camera position, an up direction, and a facing
    /// direction.
    ///
    /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
    #[inline]
    #[must_use]
    pub fn look_to_lh(eye: Vec3, dir: Vec3, up: Vec3) -> Self {
        Self::look_to_rh(eye, -dir, up)
    }

    /// Creates a right-handed view transform using a camera position, an up direction, and a facing
    /// direction.
    ///
    /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
    #[inline]
    #[must_use]
    pub fn look_to_rh(eye: Vec3, dir: Vec3, up: Vec3) -> Self {
        let f = dir.normalize();
        let s = f.cross(up).normalize();
        let u = s.cross(f);

        Self {
            matrix3: Mat3A::from_cols(
                Vec3A::new(s.x, u.x, -f.x),
                Vec3A::new(s.y, u.y, -f.y),
                Vec3A::new(s.z, u.z, -f.z),
            ),
            translation: Vec3A::new(-eye.dot(s), -eye.dot(u), eye.dot(f)),
        }
    }

    /// Creates a left-handed view transform using a camera position, an up direction, and a focal
    /// point.
    /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
    ///
    /// # Panics
    ///
    /// Will panic if `up` is not normalized when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn look_at_lh(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        glam_assert!(up.is_normalized());
        Self::look_to_lh(eye, center - eye, up)
    }

    /// Creates a right-handed view transform using a camera position, an up direction, and a focal
    /// point.
    /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
    ///
    /// # Panics
    ///
    /// Will panic if `up` is not normalized when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn look_at_rh(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        glam_assert!(up.is_normalized());
        Self::look_to_rh(eye, center - eye, up)
    }

    /// Transforms the given 3D points, applying shear, scale, rotation and translation.
    #[inline]
    pub fn transform_point3(&self, rhs: Vec3) -> Vec3 {
        #[allow(clippy::useless_conversion)]
        ((self.matrix3.x_axis * rhs.x)
            + (self.matrix3.y_axis * rhs.y)
            + (self.matrix3.z_axis * rhs.z)
            + self.translation)
            .into()
    }

    /// Transforms the given 3D vector, applying shear, scale and rotation (but NOT
    /// translation).
    ///
    /// To also apply translation, use [`Self::transform_point3()`] instead.
    #[inline]
    #[must_use]
    pub fn transform_vector3(&self, rhs: Vec3) -> Vec3 {
        #[allow(clippy::useless_conversion)]
        ((self.matrix3.x_axis * rhs.x)
            + (self.matrix3.y_axis * rhs.y)
            + (self.matrix3.z_axis * rhs.z))
            .into()
    }

    /// Transforms the given [`Vec3A`], applying shear, scale, rotation and translation.
    #[inline]
    #[must_use]
    pub fn transform_point3a(&self, rhs: Vec3A) -> Vec3A {
        self.matrix3 * rhs + self.translation
    }

    /// Transforms the given [`Vec3A`], applying shear, scale and rotation (but NOT
    /// translation).
    ///
    /// To also apply translation, use [`Self::transform_point3a()`] instead.
    #[inline]
    #[must_use]
    pub fn transform_vector3a(&self, rhs: Vec3A) -> Vec3A {
        self.matrix3 * rhs
    }

    /// Returns `true` if, and only if, all elements are finite.
    ///
    /// If any element is either `NaN`, positive or negative infinity, this will return
    /// `false`.
    #[inline]
    #[must_use]
    pub fn is_finite(&self) -> bool {
        self.matrix3.is_finite() && self.translation.is_finite()
    }

    /// Returns `true` if any elements are `NaN`.
    #[inline]
    #[must_use]
    pub fn is_nan(&self) -> bool {
        self.matrix3.is_nan() || self.translation.is_nan()
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
    pub fn abs_diff_eq(&self, rhs: Self, max_abs_diff: f32) -> bool {
        self.matrix3.abs_diff_eq(rhs.matrix3, max_abs_diff)
            && self.translation.abs_diff_eq(rhs.translation, max_abs_diff)
    }

    /// Return the inverse of this transform.
    ///
    /// Note that if the transform is not invertible the result will be invalid.
    #[inline]
    #[must_use]
    pub fn inverse(&self) -> Self {
        let matrix3 = self.matrix3.inverse();
        // transform negative translation by the matrix inverse:
        let translation = -(matrix3 * self.translation);

        Self {
            matrix3,
            translation,
        }
    }
}

impl Default for Affine3A {
    #[inline(always)]
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Deref for Affine3A {
    type Target = crate::deref::Cols4<Vec3A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const Self as *const Self::Target) }
    }
}

impl DerefMut for Affine3A {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self as *mut Self as *mut Self::Target) }
    }
}

impl PartialEq for Affine3A {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        self.matrix3.eq(&rhs.matrix3) && self.translation.eq(&rhs.translation)
    }
}

impl core::fmt::Debug for Affine3A {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        fmt.debug_struct(stringify!(Affine3A))
            .field("matrix3", &self.matrix3)
            .field("translation", &self.translation)
            .finish()
    }
}

impl core::fmt::Display for Affine3A {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if let Some(p) = f.precision() {
            write!(
                f,
                "[{:.*}, {:.*}, {:.*}, {:.*}]",
                p,
                self.matrix3.x_axis,
                p,
                self.matrix3.y_axis,
                p,
                self.matrix3.z_axis,
                p,
                self.translation
            )
        } else {
            write!(
                f,
                "[{}, {}, {}, {}]",
                self.matrix3.x_axis, self.matrix3.y_axis, self.matrix3.z_axis, self.translation
            )
        }
    }
}

impl<'a> core::iter::Product<&'a Self> for Affine3A {
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::IDENTITY, |a, &b| a * b)
    }
}

impl Mul for Affine3A {
    type Output = Affine3A;

    #[inline]
    fn mul(self, rhs: Affine3A) -> Self::Output {
        Self {
            matrix3: self.matrix3 * rhs.matrix3,
            translation: self.matrix3 * rhs.translation + self.translation,
        }
    }
}

impl MulAssign for Affine3A {
    #[inline]
    fn mul_assign(&mut self, rhs: Affine3A) {
        *self = self.mul(rhs);
    }
}

impl From<Affine3A> for Mat4 {
    #[inline]
    fn from(m: Affine3A) -> Mat4 {
        Mat4::from_cols(
            m.matrix3.x_axis.extend(0.0),
            m.matrix3.y_axis.extend(0.0),
            m.matrix3.z_axis.extend(0.0),
            m.translation.extend(1.0),
        )
    }
}

impl Mul<Mat4> for Affine3A {
    type Output = Mat4;

    #[inline]
    fn mul(self, rhs: Mat4) -> Self::Output {
        Mat4::from(self) * rhs
    }
}

impl Mul<Affine3A> for Mat4 {
    type Output = Mat4;

    #[inline]
    fn mul(self, rhs: Affine3A) -> Self::Output {
        self * Mat4::from(rhs)
    }
}
