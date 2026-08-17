// Generated from quat.rs.tera template. Edit the template, not the generated file.

use crate::{
    coresimd::*,
    euler::{EulerFromQuaternion, EulerRot, EulerToQuaternion},
    f32::math,
    DQuat, Mat3, Mat3A, Mat4, Vec2, Vec3, Vec3A, Vec4,
};

use core::simd::*;

#[cfg(not(target_arch = "spirv"))]
use core::fmt;
use core::iter::{Product, Sum};
use core::ops::{Add, Deref, DerefMut, Div, Mul, MulAssign, Neg, Sub};

/// Creates a quaternion from `x`, `y`, `z` and `w` values.
///
/// This should generally not be called manually unless you know what you are doing. Use
/// one of the other constructors instead such as `identity` or `from_axis_angle`.
#[inline]
#[must_use]
pub const fn quat(x: f32, y: f32, z: f32, w: f32) -> Quat {
    Quat::from_xyzw(x, y, z, w)
}

/// A quaternion representing an orientation.
///
/// This quaternion is intended to be of unit length but may denormalize due to
/// floating point "error creep" which can occur when successive quaternion
/// operations are applied.
///
/// SIMD vector types are used for storage on supported platforms.
///
/// This type is 16 byte aligned.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Quat(pub(crate) f32x4);

impl Quat {
    /// All zeros.
    const ZERO: Self = Self::from_array([0.0; 4]);

    /// The identity quaternion. Corresponds to no rotation.
    pub const IDENTITY: Self = Self::from_xyzw(0.0, 0.0, 0.0, 1.0);

    /// All NANs.
    pub const NAN: Self = Self::from_array([f32::NAN; 4]);

    /// Creates a new rotation quaternion.
    ///
    /// This should generally not be called manually unless you know what you are doing.
    /// Use one of the other constructors instead such as `identity` or `from_axis_angle`.
    ///
    /// `from_xyzw` is mostly used by unit tests and `serde` deserialization.
    ///
    /// # Preconditions
    ///
    /// This function does not check if the input is normalized, it is up to the user to
    /// provide normalized input or to normalized the resulting quaternion.
    #[inline(always)]
    #[must_use]
    pub const fn from_xyzw(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self(f32x4::from_array([x, y, z, w]))
    }

    /// Creates a rotation quaternion from an array.
    ///
    /// # Preconditions
    ///
    /// This function does not check if the input is normalized, it is up to the user to
    /// provide normalized input or to normalized the resulting quaternion.
    #[inline]
    #[must_use]
    pub const fn from_array(a: [f32; 4]) -> Self {
        Self(f32x4::from_array(a))
    }

    /// Creates a new rotation quaternion from a 4D vector.
    ///
    /// # Preconditions
    ///
    /// This function does not check if the input is normalized, it is up to the user to
    /// provide normalized input or to normalized the resulting quaternion.
    #[inline]
    #[must_use]
    pub const fn from_vec4(v: Vec4) -> Self {
        Self(v.0)
    }

    /// Creates a rotation quaternion from a slice.
    ///
    /// # Preconditions
    ///
    /// This function does not check if the input is normalized, it is up to the user to
    /// provide normalized input or to normalized the resulting quaternion.
    ///
    /// # Panics
    ///
    /// Panics if `slice` length is less than 4.
    #[inline]
    #[must_use]
    pub fn from_slice(slice: &[f32]) -> Self {
        Self::from_xyzw(slice[0], slice[1], slice[2], slice[3])
    }

    /// Writes the quaternion to an unaligned slice.
    ///
    /// # Panics
    ///
    /// Panics if `slice` length is less than 4.
    #[inline]
    pub fn write_to_slice(self, slice: &mut [f32]) {
        slice[0] = self.x;
        slice[1] = self.y;
        slice[2] = self.z;
        slice[3] = self.w;
    }

    /// Create a quaternion for a normalized rotation `axis` and `angle` (in radians).
    ///
    /// The axis must be a unit vector.
    ///
    /// # Panics
    ///
    /// Will panic if `axis` is not normalized when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn from_axis_angle(axis: Vec3, angle: f32) -> Self {
        glam_assert!(axis.is_normalized());
        let (s, c) = math::sin_cos(angle * 0.5);
        let v = axis * s;
        Self::from_xyzw(v.x, v.y, v.z, c)
    }

    /// Create a quaternion that rotates `v.length()` radians around `v.normalize()`.
    ///
    /// `from_scaled_axis(Vec3::ZERO)` results in the identity quaternion.
    #[inline]
    #[must_use]
    pub fn from_scaled_axis(v: Vec3) -> Self {
        let length = v.length();
        if length == 0.0 {
            Self::IDENTITY
        } else {
            Self::from_axis_angle(v / length, length)
        }
    }

    /// Creates a quaternion from the `angle` (in radians) around the x axis.
    #[inline]
    #[must_use]
    pub fn from_rotation_x(angle: f32) -> Self {
        let (s, c) = math::sin_cos(angle * 0.5);
        Self::from_xyzw(s, 0.0, 0.0, c)
    }

    /// Creates a quaternion from the `angle` (in radians) around the y axis.
    #[inline]
    #[must_use]
    pub fn from_rotation_y(angle: f32) -> Self {
        let (s, c) = math::sin_cos(angle * 0.5);
        Self::from_xyzw(0.0, s, 0.0, c)
    }

    /// Creates a quaternion from the `angle` (in radians) around the z axis.
    #[inline]
    #[must_use]
    pub fn from_rotation_z(angle: f32) -> Self {
        let (s, c) = math::sin_cos(angle * 0.5);
        Self::from_xyzw(0.0, 0.0, s, c)
    }

    /// Creates a quaternion from the given Euler rotation sequence and the angles (in radians).
    #[inline]
    #[must_use]
    pub fn from_euler(euler: EulerRot, a: f32, b: f32, c: f32) -> Self {
        euler.new_quat(a, b, c)
    }

    /// From the columns of a 3x3 rotation matrix.
    #[inline]
    #[must_use]
    pub(crate) fn from_rotation_axes(x_axis: Vec3, y_axis: Vec3, z_axis: Vec3) -> Self {
        // Based on https://github.com/microsoft/DirectXMath `XM$quaternionRotationMatrix`
        let (m00, m01, m02) = x_axis.into();
        let (m10, m11, m12) = y_axis.into();
        let (m20, m21, m22) = z_axis.into();
        if m22 <= 0.0 {
            // x^2 + y^2 >= z^2 + w^2
            let dif10 = m11 - m00;
            let omm22 = 1.0 - m22;
            if dif10 <= 0.0 {
                // x^2 >= y^2
                let four_xsq = omm22 - dif10;
                let inv4x = 0.5 / math::sqrt(four_xsq);
                Self::from_xyzw(
                    four_xsq * inv4x,
                    (m01 + m10) * inv4x,
                    (m02 + m20) * inv4x,
                    (m12 - m21) * inv4x,
                )
            } else {
                // y^2 >= x^2
                let four_ysq = omm22 + dif10;
                let inv4y = 0.5 / math::sqrt(four_ysq);
                Self::from_xyzw(
                    (m01 + m10) * inv4y,
                    four_ysq * inv4y,
                    (m12 + m21) * inv4y,
                    (m20 - m02) * inv4y,
                )
            }
        } else {
            // z^2 + w^2 >= x^2 + y^2
            let sum10 = m11 + m00;
            let opm22 = 1.0 + m22;
            if sum10 <= 0.0 {
                // z^2 >= w^2
                let four_zsq = opm22 - sum10;
                let inv4z = 0.5 / math::sqrt(four_zsq);
                Self::from_xyzw(
                    (m02 + m20) * inv4z,
                    (m12 + m21) * inv4z,
                    four_zsq * inv4z,
                    (m01 - m10) * inv4z,
                )
            } else {
                // w^2 >= z^2
                let four_wsq = opm22 + sum10;
                let inv4w = 0.5 / math::sqrt(four_wsq);
                Self::from_xyzw(
                    (m12 - m21) * inv4w,
                    (m20 - m02) * inv4w,
                    (m01 - m10) * inv4w,
                    four_wsq * inv4w,
                )
            }
        }
    }

    /// Creates a quaternion from a 3x3 rotation matrix.
    #[inline]
    #[must_use]
    pub fn from_mat3(mat: &Mat3) -> Self {
        Self::from_rotation_axes(mat.x_axis, mat.y_axis, mat.z_axis)
    }

    /// Creates a quaternion from a 3x3 SIMD aligned rotation matrix.
    #[inline]
    #[must_use]
    pub fn from_mat3a(mat: &Mat3A) -> Self {
        Self::from_rotation_axes(mat.x_axis.into(), mat.y_axis.into(), mat.z_axis.into())
    }

    /// Creates a quaternion from a 3x3 rotation matrix inside a homogeneous 4x4 matrix.
    #[inline]
    #[must_use]
    pub fn from_mat4(mat: &Mat4) -> Self {
        Self::from_rotation_axes(
            mat.x_axis.truncate(),
            mat.y_axis.truncate(),
            mat.z_axis.truncate(),
        )
    }

    /// Gets the minimal rotation for transforming `from` to `to`.  The rotation is in the
    /// plane spanned by the two vectors.  Will rotate at most 180 degrees.
    ///
    /// The inputs must be unit vectors.
    ///
    /// `from_rotation_arc(from, to) * from ≈ to`.
    ///
    /// For near-singular cases (from≈to and from≈-to) the current implementation
    /// is only accurate to about 0.001 (for `f32`).
    ///
    /// # Panics
    ///
    /// Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.
    #[must_use]
    pub fn from_rotation_arc(from: Vec3, to: Vec3) -> Self {
        glam_assert!(from.is_normalized());
        glam_assert!(to.is_normalized());

        const ONE_MINUS_EPS: f32 = 1.0 - 2.0 * f32::EPSILON;
        let dot = from.dot(to);
        if dot > ONE_MINUS_EPS {
            // 0° singularity: from ≈ to
            Self::IDENTITY
        } else if dot < -ONE_MINUS_EPS {
            // 180° singularity: from ≈ -to
            use core::f32::consts::PI; // half a turn = 𝛕/2 = 180°
            Self::from_axis_angle(from.any_orthonormal_vector(), PI)
        } else {
            let c = from.cross(to);
            Self::from_xyzw(c.x, c.y, c.z, 1.0 + dot).normalize()
        }
    }

    /// Gets the minimal rotation for transforming `from` to either `to` or `-to`.  This means
    /// that the resulting quaternion will rotate `from` so that it is colinear with `to`.
    ///
    /// The rotation is in the plane spanned by the two vectors.  Will rotate at most 90
    /// degrees.
    ///
    /// The inputs must be unit vectors.
    ///
    /// `to.dot(from_rotation_arc_colinear(from, to) * from).abs() ≈ 1`.
    ///
    /// # Panics
    ///
    /// Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn from_rotation_arc_colinear(from: Vec3, to: Vec3) -> Self {
        if from.dot(to) < 0.0 {
            Self::from_rotation_arc(from, -to)
        } else {
            Self::from_rotation_arc(from, to)
        }
    }

    /// Gets the minimal rotation for transforming `from` to `to`.  The resulting rotation is
    /// around the z axis. Will rotate at most 180 degrees.
    ///
    /// The inputs must be unit vectors.
    ///
    /// `from_rotation_arc_2d(from, to) * from ≈ to`.
    ///
    /// For near-singular cases (from≈to and from≈-to) the current implementation
    /// is only accurate to about 0.001 (for `f32`).
    ///
    /// # Panics
    ///
    /// Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.
    #[must_use]
    pub fn from_rotation_arc_2d(from: Vec2, to: Vec2) -> Self {
        glam_assert!(from.is_normalized());
        glam_assert!(to.is_normalized());

        const ONE_MINUS_EPSILON: f32 = 1.0 - 2.0 * f32::EPSILON;
        let dot = from.dot(to);
        if dot > ONE_MINUS_EPSILON {
            // 0° singularity: from ≈ to
            Self::IDENTITY
        } else if dot < -ONE_MINUS_EPSILON {
            // 180° singularity: from ≈ -to
            const COS_FRAC_PI_2: f32 = 0.0;
            const SIN_FRAC_PI_2: f32 = 1.0;
            // rotation around z by PI radians
            Self::from_xyzw(0.0, 0.0, SIN_FRAC_PI_2, COS_FRAC_PI_2)
        } else {
            // vector3 cross where z=0
            let z = from.x * to.y - to.x * from.y;
            let w = 1.0 + dot;
            // calculate length with x=0 and y=0 to normalize
            let len_rcp = 1.0 / math::sqrt(z * z + w * w);
            Self::from_xyzw(0.0, 0.0, z * len_rcp, w * len_rcp)
        }
    }

    /// Returns the rotation axis (normalized) and angle (in radians) of `self`.
    #[inline]
    #[must_use]
    pub fn to_axis_angle(self) -> (Vec3, f32) {
        const EPSILON: f32 = 1.0e-8;
        let v = Vec3::new(self.x, self.y, self.z);
        let length = v.length();
        if length >= EPSILON {
            let angle = 2.0 * math::atan2(length, self.w);
            let axis = v / length;
            (axis, angle)
        } else {
            (Vec3::X, 0.0)
        }
    }

    /// Returns the rotation axis scaled by the rotation in radians.
    #[inline]
    #[must_use]
    pub fn to_scaled_axis(self) -> Vec3 {
        let (axis, angle) = self.to_axis_angle();
        axis * angle
    }

    /// Returns the rotation angles for the given euler rotation sequence.
    #[inline]
    #[must_use]
    pub fn to_euler(self, euler: EulerRot) -> (f32, f32, f32) {
        euler.convert_quat(self)
    }

    /// `[x, y, z, w]`
    #[inline]
    #[must_use]
    pub fn to_array(&self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
    }

    /// Returns the vector part of the quaternion.
    #[inline]
    #[must_use]
    pub fn xyz(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }

    /// Returns the quaternion conjugate of `self`. For a unit quaternion the
    /// conjugate is also the inverse.
    #[inline]
    #[must_use]
    pub fn conjugate(self) -> Self {
        const SIGN: f32x4 = f32x4::from_array([-1.0, -1.0, -1.0, 1.0]);
        Self(self.0.mul(SIGN))
    }

    /// Returns the inverse of a normalized quaternion.
    ///
    /// Typically quaternion inverse returns the conjugate of a normalized quaternion.
    /// Because `self` is assumed to already be unit length this method *does not* normalize
    /// before returning the conjugate.
    ///
    /// # Panics
    ///
    /// Will panic if `self` is not normalized when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn inverse(self) -> Self {
        glam_assert!(self.is_normalized());
        self.conjugate()
    }

    /// Computes the dot product of `self` and `rhs`. The dot product is
    /// equal to the cosine of the angle between two quaternion rotations.
    #[inline]
    #[must_use]
    pub fn dot(self, rhs: Self) -> f32 {
        Vec4::from(self).dot(Vec4::from(rhs))
    }

    /// Computes the length of `self`.
    #[doc(alias = "magnitude")]
    #[inline]
    #[must_use]
    pub fn length(self) -> f32 {
        Vec4::from(self).length()
    }

    /// Computes the squared length of `self`.
    ///
    /// This is generally faster than `length()` as it avoids a square
    /// root operation.
    #[doc(alias = "magnitude2")]
    #[inline]
    #[must_use]
    pub fn length_squared(self) -> f32 {
        Vec4::from(self).length_squared()
    }

    /// Computes `1.0 / length()`.
    ///
    /// For valid results, `self` must _not_ be of length zero.
    #[inline]
    #[must_use]
    pub fn length_recip(self) -> f32 {
        Vec4::from(self).length_recip()
    }

    /// Returns `self` normalized to length 1.0.
    ///
    /// For valid results, `self` must _not_ be of length zero.
    ///
    /// Panics
    ///
    /// Will panic if `self` is zero length when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn normalize(self) -> Self {
        Self::from_vec4(Vec4::from(self).normalize())
    }

    /// Returns `true` if, and only if, all elements are finite.
    /// If any element is either `NaN`, positive or negative infinity, this will return `false`.
    #[inline]
    #[must_use]
    pub fn is_finite(self) -> bool {
        Vec4::from(self).is_finite()
    }

    #[inline]
    #[must_use]
    pub fn is_nan(self) -> bool {
        Vec4::from(self).is_nan()
    }

    /// Returns whether `self` of length `1.0` or not.
    ///
    /// Uses a precision threshold of `1e-6`.
    #[inline]
    #[must_use]
    pub fn is_normalized(self) -> bool {
        Vec4::from(self).is_normalized()
    }

    #[inline]
    #[must_use]
    pub fn is_near_identity(self) -> bool {
        // Based on https://github.com/nfrechette/rtm `rtm::quat_near_identity`
        let threshold_angle = 0.002_847_144_6;
        // Because of floating point precision, we cannot represent very small rotations.
        // The closest f32 to 1.0 that is not 1.0 itself yields:
        // 0.99999994.acos() * 2.0  = 0.000690533954 rad
        //
        // An error threshold of 1.e-6 is used by default.
        // (1.0 - 1.e-6).acos() * 2.0 = 0.00284714461 rad
        // (1.0 - 1.e-7).acos() * 2.0 = 0.00097656250 rad
        //
        // We don't really care about the angle value itself, only if it's close to 0.
        // This will happen whenever quat.w is close to 1.0.
        // If the quat.w is close to -1.0, the angle will be near 2*PI which is close to
        // a negative 0 rotation. By forcing quat.w to be positive, we'll end up with
        // the shortest path.
        let positive_w_angle = math::acos_approx(math::abs(self.w)) * 2.0;
        positive_w_angle < threshold_angle
    }

    /// Returns the angle (in radians) for the minimal rotation
    /// for transforming this quaternion into another.
    ///
    /// Both quaternions must be normalized.
    ///
    /// # Panics
    ///
    /// Will panic if `self` or `rhs` are not normalized when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn angle_between(self, rhs: Self) -> f32 {
        glam_assert!(self.is_normalized() && rhs.is_normalized());
        math::acos_approx(math::abs(self.dot(rhs))) * 2.0
    }

    /// Rotates towards `rhs` up to `max_angle` (in radians).
    ///
    /// When `max_angle` is `0.0`, the result will be equal to `self`. When `max_angle` is equal to
    /// `self.angle_between(rhs)`, the result will be equal to `rhs`. If `max_angle` is negative,
    /// rotates towards the exact opposite of `rhs`. Will not go past the target.
    ///
    /// Both quaternions must be normalized.
    ///
    /// # Panics
    ///
    /// Will panic if `self` or `rhs` are not normalized when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn rotate_towards(&self, rhs: Self, max_angle: f32) -> Self {
        glam_assert!(self.is_normalized() && rhs.is_normalized());
        let angle = self.angle_between(rhs);
        if angle <= 1e-4 {
            return *self;
        }
        let s = (max_angle / angle).clamp(-1.0, 1.0);
        self.slerp(rhs, s)
    }

    /// Returns true if the absolute difference of all elements between `self` and `rhs`
    /// is less than or equal to `max_abs_diff`.
    ///
    /// This can be used to compare if two quaternions contain similar elements. It works
    /// best when comparing with a known value. The `max_abs_diff` that should be used used
    /// depends on the values being compared against.
    ///
    /// For more see
    /// [comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
    #[inline]
    #[must_use]
    pub fn abs_diff_eq(self, rhs: Self, max_abs_diff: f32) -> bool {
        Vec4::from(self).abs_diff_eq(Vec4::from(rhs), max_abs_diff)
    }

    /// Performs a linear interpolation between `self` and `rhs` based on
    /// the value `s`.
    ///
    /// When `s` is `0.0`, the result will be equal to `self`.  When `s`
    /// is `1.0`, the result will be equal to `rhs`.
    ///
    /// # Panics
    ///
    /// Will panic if `self` or `end` are not normalized when `glam_assert` is enabled.
    #[doc(alias = "mix")]
    #[inline]
    #[must_use]
    pub fn lerp(self, end: Self, s: f32) -> Self {
        glam_assert!(self.is_normalized());
        glam_assert!(end.is_normalized());

        const NEG_ZERO: f32x4 = f32x4::from_array([-0.0; 4]);
        let start = self.0;
        let end = end.0;
        let dot = dot4_into_f32x4(start, end);
        // Calculate the bias, if the dot product is positive or zero, there is no bias
        // but if it is negative, we want to flip the 'end' rotation XYZW components
        let bias = f32x4_bitand(dot, NEG_ZERO);
        let interpolated = start + ((f32x4_bitxor(end, bias) - start) * f32x4::splat(s));
        Quat(interpolated).normalize()
    }

    /// Performs a spherical linear interpolation between `self` and `end`
    /// based on the value `s`.
    ///
    /// When `s` is `0.0`, the result will be equal to `self`.  When `s`
    /// is `1.0`, the result will be equal to `end`.
    ///
    /// # Panics
    ///
    /// Will panic if `self` or `end` are not normalized when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn slerp(self, mut end: Self, s: f32) -> Self {
        // http://number-none.com/product/Understanding%20Slerp,%20Then%20Not%20Using%20It/
        glam_assert!(self.is_normalized());
        glam_assert!(end.is_normalized());

        const DOT_THRESHOLD: f32 = 0.9995;

        // Note that a rotation can be represented by two quaternions: `q` and
        // `-q`. The slerp path between `q` and `end` will be different from the
        // path between `-q` and `end`. One path will take the long way around and
        // one will take the short way. In order to correct for this, the `dot`
        // product between `self` and `end` should be positive. If the `dot`
        // product is negative, slerp between `self` and `-end`.
        let mut dot = self.dot(end);
        if dot < 0.0 {
            end = -end;
            dot = -dot;
        }

        if dot > DOT_THRESHOLD {
            // assumes lerp returns a normalized quaternion
            self.lerp(end, s)
        } else {
            let theta = math::acos_approx(dot);

            let x = math::sin(theta * (1.0 - s));
            let y = math::sin(theta * s);
            let z = math::sin(theta);
            let w = 0.0;
            let tmp = f32x4::from_array([x, y, z, w]);

            let scale1 = simd_swizzle!(tmp, [0, 0, 0, 0]);
            let scale2 = simd_swizzle!(tmp, [1, 1, 1, 1]);
            let theta_sin = simd_swizzle!(tmp, [2, 2, 2, 2]);

            Self(self.0.mul(scale1).add(end.0.mul(scale2)).div(theta_sin))
        }
    }

    /// Multiplies a quaternion and a 3D vector, returning the rotated vector.
    ///
    /// # Panics
    ///
    /// Will panic if `self` is not normalized when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn mul_vec3(self, rhs: Vec3) -> Vec3 {
        glam_assert!(self.is_normalized());

        self.mul_vec3a(rhs.into()).into()
    }

    /// Multiplies two quaternions. If they each represent a rotation, the result will
    /// represent the combined rotation.
    ///
    /// Note that due to floating point rounding the result may not be perfectly normalized.
    ///
    /// # Panics
    ///
    /// Will panic if `self` or `rhs` are not normalized when `glam_assert` is enabled.
    #[inline]
    #[must_use]
    pub fn mul_quat(self, rhs: Self) -> Self {
        glam_assert!(self.is_normalized());
        glam_assert!(rhs.is_normalized());

        let lhs = self.0;
        let rhs = rhs.0;

        const CONTROL_WZYX: f32x4 = f32x4::from_array([1.0, -1.0, 1.0, -1.0]);
        const CONTROL_ZWXY: f32x4 = f32x4::from_array([1.0, 1.0, -1.0, -1.0]);
        const CONTROL_YXWZ: f32x4 = f32x4::from_array([-1.0, 1.0, 1.0, -1.0]);

        let r_xxxx = simd_swizzle!(lhs, [0, 0, 0, 0]);
        let r_yyyy = simd_swizzle!(lhs, [1, 1, 1, 1]);
        let r_zzzz = simd_swizzle!(lhs, [2, 2, 2, 2]);
        let r_wwww = simd_swizzle!(lhs, [3, 3, 3, 3]);

        let lxrw_lyrw_lzrw_lwrw = r_wwww * rhs;
        let l_wzyx = simd_swizzle!(rhs, [3, 2, 1, 0]);

        let lwrx_lzrx_lyrx_lxrx = r_xxxx * l_wzyx;
        let l_zwxy = simd_swizzle!(l_wzyx, [1, 0, 3, 2]);

        let lwrx_nlzrx_lyrx_nlxrx = lwrx_lzrx_lyrx_lxrx * CONTROL_WZYX;

        let lzry_lwry_lxry_lyry = r_yyyy * l_zwxy;
        let l_yxwz = simd_swizzle!(l_zwxy, [3, 2, 1, 0]);

        let lzry_lwry_nlxry_nlyry = lzry_lwry_lxry_lyry * CONTROL_ZWXY;

        let lyrz_lxrz_lwrz_lzrz = r_zzzz * l_yxwz;
        let result0 = lxrw_lyrw_lzrw_lwrw + lwrx_nlzrx_lyrx_nlxrx;

        let nlyrz_lxrz_lwrz_wlzrz = lyrz_lxrz_lwrz_lzrz * CONTROL_YXWZ;
        let result1 = lzry_lwry_nlxry_nlyry + nlyrz_lxrz_lwrz_wlzrz;
        Self(result0 + result1)
    }

    /// Creates a quaternion from a 3x3 rotation matrix inside a 3D affine transform.
    #[inline]
    #[must_use]
    pub fn from_affine3(a: &crate::Affine3A) -> Self {
        #[allow(clippy::useless_conversion)]
        Self::from_rotation_axes(
            a.matrix3.x_axis.into(),
            a.matrix3.y_axis.into(),
            a.matrix3.z_axis.into(),
        )
    }

    /// Multiplies a quaternion and a 3D vector, returning the rotated vector.
    #[inline]
    #[must_use]
    pub fn mul_vec3a(self, rhs: Vec3A) -> Vec3A {
        const TWO: f32x4 = f32x4::from_array([2.0; 4]);
        let w = simd_swizzle!(self.0, [3, 3, 3, 3]);
        let b = self.0;
        let b2 = dot3_into_f32x4(b, b);
        Vec3A(
            rhs.0
                .mul(w.mul(w).sub(b2))
                .add(b.mul(dot3_into_f32x4(rhs.0, b).mul(TWO)))
                .add(Vec3A(b).cross(rhs).0.mul(w.mul(TWO))),
        )
    }

    #[inline]
    #[must_use]
    pub fn as_dquat(self) -> DQuat {
        DQuat::from_xyzw(self.x as f64, self.y as f64, self.z as f64, self.w as f64)
    }
}

#[cfg(not(target_arch = "spirv"))]
impl fmt::Debug for Quat {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_tuple(stringify!(Quat))
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .field(&self.w)
            .finish()
    }
}

#[cfg(not(target_arch = "spirv"))]
impl fmt::Display for Quat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(p) = f.precision() {
            write!(
                f,
                "[{:.*}, {:.*}, {:.*}, {:.*}]",
                p, self.x, p, self.y, p, self.z, p, self.w
            )
        } else {
            write!(f, "[{}, {}, {}, {}]", self.x, self.y, self.z, self.w)
        }
    }
}

impl Add<Quat> for Quat {
    type Output = Self;
    /// Adds two quaternions.
    ///
    /// The sum is not guaranteed to be normalized.
    ///
    /// Note that addition is not the same as combining the rotations represented by the
    /// two quaternions! That corresponds to multiplication.
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self::from_vec4(Vec4::from(self) + Vec4::from(rhs))
    }
}

impl Sub<Quat> for Quat {
    type Output = Self;
    /// Subtracts the `rhs` quaternion from `self`.
    ///
    /// The difference is not guaranteed to be normalized.
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self::from_vec4(Vec4::from(self) - Vec4::from(rhs))
    }
}

impl Mul<f32> for Quat {
    type Output = Self;
    /// Multiplies a quaternion by a scalar value.
    ///
    /// The product is not guaranteed to be normalized.
    #[inline]
    fn mul(self, rhs: f32) -> Self {
        Self::from_vec4(Vec4::from(self) * rhs)
    }
}

impl Div<f32> for Quat {
    type Output = Self;
    /// Divides a quaternion by a scalar value.
    /// The quotient is not guaranteed to be normalized.
    #[inline]
    fn div(self, rhs: f32) -> Self {
        Self::from_vec4(Vec4::from(self) / rhs)
    }
}

impl Mul<Quat> for Quat {
    type Output = Self;
    /// Multiplies two quaternions. If they each represent a rotation, the result will
    /// represent the combined rotation.
    ///
    /// Note that due to floating point rounding the result may not be perfectly
    /// normalized.
    ///
    /// # Panics
    ///
    /// Will panic if `self` or `rhs` are not normalized when `glam_assert` is enabled.
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        self.mul_quat(rhs)
    }
}

impl MulAssign<Quat> for Quat {
    /// Multiplies two quaternions. If they each represent a rotation, the result will
    /// represent the combined rotation.
    ///
    /// Note that due to floating point rounding the result may not be perfectly
    /// normalized.
    ///
    /// # Panics
    ///
    /// Will panic if `self` or `rhs` are not normalized when `glam_assert` is enabled.
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.mul_quat(rhs);
    }
}

impl Mul<Vec3> for Quat {
    type Output = Vec3;
    /// Multiplies a quaternion and a 3D vector, returning the rotated vector.
    ///
    /// # Panics
    ///
    /// Will panic if `self` is not normalized when `glam_assert` is enabled.
    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        self.mul_vec3(rhs)
    }
}

impl Neg for Quat {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        self * -1.0
    }
}

impl Default for Quat {
    #[inline]
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl PartialEq for Quat {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        Vec4::from(*self).eq(&Vec4::from(*rhs))
    }
}

#[cfg(not(target_arch = "spirv"))]
impl AsRef<[f32; 4]> for Quat {
    #[inline]
    fn as_ref(&self) -> &[f32; 4] {
        unsafe { &*(self as *const Self as *const [f32; 4]) }
    }
}

impl Sum<Self> for Quat {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::ZERO, Self::add)
    }
}

impl<'a> Sum<&'a Self> for Quat {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ZERO, |a, &b| Self::add(a, b))
    }
}

impl Product for Quat {
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::IDENTITY, Self::mul)
    }
}

impl<'a> Product<&'a Self> for Quat {
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::IDENTITY, |a, &b| Self::mul(a, b))
    }
}

impl Mul<Vec3A> for Quat {
    type Output = Vec3A;
    #[inline]
    fn mul(self, rhs: Vec3A) -> Self::Output {
        self.mul_vec3a(rhs)
    }
}

impl From<Quat> for Vec4 {
    #[inline]
    fn from(q: Quat) -> Self {
        Self(q.0)
    }
}

impl From<Quat> for (f32, f32, f32, f32) {
    #[inline]
    fn from(q: Quat) -> Self {
        Vec4::from(q).into()
    }
}

impl From<Quat> for [f32; 4] {
    #[inline]
    fn from(q: Quat) -> Self {
        Vec4::from(q).into()
    }
}

impl From<Quat> for f32x4 {
    #[inline]
    fn from(q: Quat) -> Self {
        q.0
    }
}

impl Deref for Quat {
    type Target = crate::deref::Vec4<f32>;
    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const Self).cast() }
    }
}

impl DerefMut for Quat {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self as *mut Self).cast() }
    }
}
