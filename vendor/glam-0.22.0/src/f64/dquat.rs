// Generated from quat.rs.tera template. Edit the template, not the generated file.

use crate::{
    euler::{EulerFromQuaternion, EulerRot, EulerToQuaternion},
    DMat3, DMat4, DVec2, DVec3, DVec4, FloatEx, Quat,
};

#[cfg(feature = "libm")]
#[allow(unused_imports)]
use num_traits::Float;

#[cfg(not(target_arch = "spirv"))]
use core::fmt;
use core::iter::{Product, Sum};
use core::ops::{Add, Div, Mul, MulAssign, Neg, Sub};

/// Creates a quaternion from `x`, `y`, `z` and `w` values.
///
/// This should generally not be called manually unless you know what you are doing. Use
/// one of the other constructors instead such as `identity` or `from_axis_angle`.
#[inline]
pub const fn dquat(x: f64, y: f64, z: f64, w: f64) -> DQuat {
    DQuat::from_xyzw(x, y, z, w)
}

/// A quaternion representing an orientation.
///
/// This quaternion is intended to be of unit length but may denormalize due to
/// floating point "error creep" which can occur when successive quaternion
/// operations are applied.
#[derive(Clone, Copy)]
#[cfg_attr(not(target_arch = "spirv"), repr(C))]
#[cfg_attr(target_arch = "spirv", repr(simd))]
pub struct DQuat {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl DQuat {
    /// All zeros.
    const ZERO: Self = Self::from_array([0.0; 4]);

    /// The identity quaternion. Corresponds to no rotation.
    pub const IDENTITY: Self = Self::from_xyzw(0.0, 0.0, 0.0, 1.0);

    /// All NANs.
    pub const NAN: Self = Self::from_array([f64::NAN; 4]);

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
    pub const fn from_xyzw(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }

    /// Creates a rotation quaternion from an array.
    ///
    /// # Preconditions
    ///
    /// This function does not check if the input is normalized, it is up to the user to
    /// provide normalized input or to normalized the resulting quaternion.
    #[inline]
    pub const fn from_array(a: [f64; 4]) -> Self {
        Self::from_xyzw(a[0], a[1], a[2], a[3])
    }

    /// Creates a new rotation quaternion from a 4D vector.
    ///
    /// # Preconditions
    ///
    /// This function does not check if the input is normalized, it is up to the user to
    /// provide normalized input or to normalized the resulting quaternion.
    #[inline]
    pub fn from_vec4(v: DVec4) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
            w: v.w,
        }
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
    pub fn from_slice(slice: &[f64]) -> Self {
        Self::from_xyzw(slice[0], slice[1], slice[2], slice[3])
    }

    /// Writes the quaternion to an unaligned slice.
    ///
    /// # Panics
    ///
    /// Panics if `slice` length is less than 4.
    #[inline]
    pub fn write_to_slice(self, slice: &mut [f64]) {
        slice[0] = self.x;
        slice[1] = self.y;
        slice[2] = self.z;
        slice[3] = self.w;
    }

    /// Create a quaternion for a normalized rotation `axis` and `angle` (in radians).
    /// The axis must be normalized (unit-length).
    ///
    /// # Panics
    ///
    /// Will panic if `axis` is not normalized when `glam_assert` is enabled.
    #[inline]
    pub fn from_axis_angle(axis: DVec3, angle: f64) -> Self {
        glam_assert!(axis.is_normalized());
        let (s, c) = (angle * 0.5).sin_cos();
        let v = axis * s;
        Self::from_xyzw(v.x, v.y, v.z, c)
    }

    /// Create a quaternion that rotates `v.length()` radians around `v.normalize()`.
    ///
    /// `from_scaled_axis(Vec3::ZERO)` results in the identity quaternion.
    #[inline]
    pub fn from_scaled_axis(v: DVec3) -> Self {
        let length = v.length();
        if length == 0.0 {
            Self::IDENTITY
        } else {
            Self::from_axis_angle(v / length, length)
        }
    }

    /// Creates a quaternion from the `angle` (in radians) around the x axis.
    #[inline]
    pub fn from_rotation_x(angle: f64) -> Self {
        let (s, c) = (angle * 0.5).sin_cos();
        Self::from_xyzw(s, 0.0, 0.0, c)
    }

    /// Creates a quaternion from the `angle` (in radians) around the y axis.
    #[inline]
    pub fn from_rotation_y(angle: f64) -> Self {
        let (s, c) = (angle * 0.5).sin_cos();
        Self::from_xyzw(0.0, s, 0.0, c)
    }

    /// Creates a quaternion from the `angle` (in radians) around the z axis.
    #[inline]
    pub fn from_rotation_z(angle: f64) -> Self {
        let (s, c) = (angle * 0.5).sin_cos();
        Self::from_xyzw(0.0, 0.0, s, c)
    }

    #[inline]
    /// Creates a quaternion from the given Euler rotation sequence and the angles (in radians).
    pub fn from_euler(euler: EulerRot, a: f64, b: f64, c: f64) -> Self {
        euler.new_quat(a, b, c)
    }

    /// From the columns of a 3x3 rotation matrix.
    #[inline]
    pub(crate) fn from_rotation_axes(x_axis: DVec3, y_axis: DVec3, z_axis: DVec3) -> Self {
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
                let inv4x = 0.5 / four_xsq.sqrt();
                Self::from_xyzw(
                    four_xsq * inv4x,
                    (m01 + m10) * inv4x,
                    (m02 + m20) * inv4x,
                    (m12 - m21) * inv4x,
                )
            } else {
                // y^2 >= x^2
                let four_ysq = omm22 + dif10;
                let inv4y = 0.5 / four_ysq.sqrt();
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
                let inv4z = 0.5 / four_zsq.sqrt();
                Self::from_xyzw(
                    (m02 + m20) * inv4z,
                    (m12 + m21) * inv4z,
                    four_zsq * inv4z,
                    (m01 - m10) * inv4z,
                )
            } else {
                // w^2 >= z^2
                let four_wsq = opm22 + sum10;
                let inv4w = 0.5 / four_wsq.sqrt();
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
    pub fn from_mat3(mat: &DMat3) -> Self {
        Self::from_rotation_axes(mat.x_axis, mat.y_axis, mat.z_axis)
    }

    /// Creates a quaternion from a 3x3 rotation matrix inside a homogeneous 4x4 matrix.
    #[inline]
    pub fn from_mat4(mat: &DMat4) -> Self {
        Self::from_rotation_axes(
            mat.x_axis.truncate(),
            mat.y_axis.truncate(),
            mat.z_axis.truncate(),
        )
    }

    /// Gets the minimal rotation for transforming `from` to `to`.  The rotation is in the
    /// plane spanned by the two vectors.  Will rotate at most 180 degrees.
    ///
    /// The input vectors must be normalized (unit-length).
    ///
    /// `from_rotation_arc(from, to) * from ≈ to`.
    ///
    /// For near-singular cases (from≈to and from≈-to) the current implementation
    /// is only accurate to about 0.001 (for `f32`).
    ///
    /// # Panics
    ///
    /// Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.
    pub fn from_rotation_arc(from: DVec3, to: DVec3) -> Self {
        glam_assert!(from.is_normalized());
        glam_assert!(to.is_normalized());

        const ONE_MINUS_EPS: f64 = 1.0 - 2.0 * core::f64::EPSILON;
        let dot = from.dot(to);
        if dot > ONE_MINUS_EPS {
            // 0° singulary: from ≈ to
            Self::IDENTITY
        } else if dot < -ONE_MINUS_EPS {
            // 180° singulary: from ≈ -to
            use core::f64::consts::PI; // half a turn = 𝛕/2 = 180°
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
    /// The input vectors must be normalized (unit-length).
    ///
    /// `to.dot(from_rotation_arc_colinear(from, to) * from).abs() ≈ 1`.
    ///
    /// # Panics
    ///
    /// Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.
    #[inline]
    pub fn from_rotation_arc_colinear(from: DVec3, to: DVec3) -> Self {
        if from.dot(to) < 0.0 {
            Self::from_rotation_arc(from, -to)
        } else {
            Self::from_rotation_arc(from, to)
        }
    }

    /// Gets the minimal rotation for transforming `from` to `to`.  The resulting rotation is
    /// around the z axis. Will rotate at most 180 degrees.
    ///
    /// The input vectors must be normalized (unit-length).
    ///
    /// `from_rotation_arc_2d(from, to) * from ≈ to`.
    ///
    /// For near-singular cases (from≈to and from≈-to) the current implementation
    /// is only accurate to about 0.001 (for `f32`).
    ///
    /// # Panics
    ///
    /// Will panic if `from` or `to` are not normalized when `glam_assert` is enabled.
    pub fn from_rotation_arc_2d(from: DVec2, to: DVec2) -> Self {
        glam_assert!(from.is_normalized());
        glam_assert!(to.is_normalized());

        const ONE_MINUS_EPSILON: f64 = 1.0 - 2.0 * core::f64::EPSILON;
        let dot = from.dot(to);
        if dot > ONE_MINUS_EPSILON {
            // 0° singulary: from ≈ to
            Self::IDENTITY
        } else if dot < -ONE_MINUS_EPSILON {
            // 180° singulary: from ≈ -to
            const COS_FRAC_PI_2: f64 = 0.0;
            const SIN_FRAC_PI_2: f64 = 1.0;
            // rotation around z by PI radians
            Self::from_xyzw(0.0, 0.0, SIN_FRAC_PI_2, COS_FRAC_PI_2)
        } else {
            // vector3 cross where z=0
            let z = from.x * to.y - to.x * from.y;
            let w = 1.0 + dot;
            // calculate length with x=0 and y=0 to normalize
            let len_rcp = 1.0 / (z * z + w * w).sqrt();
            Self::from_xyzw(0.0, 0.0, z * len_rcp, w * len_rcp)
        }
    }

    /// Returns the rotation axis and angle (in radians) of `self`.
    #[inline]
    pub fn to_axis_angle(self) -> (DVec3, f64) {
        const EPSILON: f64 = 1.0e-8;
        const EPSILON_SQUARED: f64 = EPSILON * EPSILON;
        let w = self.w;
        let angle = w.acos_approx() * 2.0;
        let scale_sq = f64::max(1.0 - w * w, 0.0);
        if scale_sq >= EPSILON_SQUARED {
            (
                DVec3::new(self.x, self.y, self.z) * scale_sq.sqrt().recip(),
                angle,
            )
        } else {
            (DVec3::X, angle)
        }
    }

    /// Returns the rotation axis scaled by the rotation in radians.
    #[inline]
    pub fn to_scaled_axis(self) -> DVec3 {
        let (axis, angle) = self.to_axis_angle();
        axis * angle
    }

    /// Returns the rotation angles for the given euler rotation sequence.
    #[inline]
    pub fn to_euler(self, euler: EulerRot) -> (f64, f64, f64) {
        euler.convert_quat(self)
    }

    /// `[x, y, z, w]`
    #[inline]
    pub fn to_array(&self) -> [f64; 4] {
        [self.x, self.y, self.z, self.w]
    }

    /// Returns the vector part of the quaternion.
    #[inline]
    pub fn xyz(self) -> DVec3 {
        DVec3::new(self.x, self.y, self.z)
    }

    /// Returns the quaternion conjugate of `self`. For a unit quaternion the
    /// conjugate is also the inverse.
    #[must_use]
    #[inline]
    pub fn conjugate(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: self.w,
        }
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
    #[must_use]
    #[inline]
    pub fn inverse(self) -> Self {
        glam_assert!(self.is_normalized());
        self.conjugate()
    }

    /// Computes the dot product of `self` and `rhs`. The dot product is
    /// equal to the cosine of the angle between two quaternion rotations.
    #[inline]
    pub fn dot(self, rhs: Self) -> f64 {
        DVec4::from(self).dot(DVec4::from(rhs))
    }

    /// Computes the length of `self`.
    #[doc(alias = "magnitude")]
    #[inline]
    pub fn length(self) -> f64 {
        DVec4::from(self).length()
    }

    /// Computes the squared length of `self`.
    ///
    /// This is generally faster than `length()` as it avoids a square
    /// root operation.
    #[doc(alias = "magnitude2")]
    #[inline]
    pub fn length_squared(self) -> f64 {
        DVec4::from(self).length_squared()
    }

    /// Computes `1.0 / length()`.
    ///
    /// For valid results, `self` must _not_ be of length zero.
    #[inline]
    pub fn length_recip(self) -> f64 {
        DVec4::from(self).length_recip()
    }

    /// Returns `self` normalized to length 1.0.
    ///
    /// For valid results, `self` must _not_ be of length zero.
    ///
    /// Panics
    ///
    /// Will panic if `self` is zero length when `glam_assert` is enabled.
    #[must_use]
    #[inline]
    pub fn normalize(self) -> Self {
        Self::from_vec4(DVec4::from(self).normalize())
    }

    /// Returns `true` if, and only if, all elements are finite.
    /// If any element is either `NaN`, positive or negative infinity, this will return `false`.
    #[inline]
    pub fn is_finite(self) -> bool {
        DVec4::from(self).is_finite()
    }

    #[inline]
    pub fn is_nan(self) -> bool {
        DVec4::from(self).is_nan()
    }

    /// Returns whether `self` of length `1.0` or not.
    ///
    /// Uses a precision threshold of `1e-6`.
    #[inline]
    pub fn is_normalized(self) -> bool {
        DVec4::from(self).is_normalized()
    }

    #[inline]
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
        let positive_w_angle = self.w.abs().acos_approx() * 2.0;
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
    pub fn angle_between(self, rhs: Self) -> f64 {
        glam_assert!(self.is_normalized() && rhs.is_normalized());
        self.dot(rhs).abs().acos_approx() * 2.0
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
    pub fn abs_diff_eq(self, rhs: Self, max_abs_diff: f64) -> bool {
        DVec4::from(self).abs_diff_eq(DVec4::from(rhs), max_abs_diff)
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
    #[inline]
    #[doc(alias = "mix")]
    pub fn lerp(self, end: Self, s: f64) -> Self {
        glam_assert!(self.is_normalized());
        glam_assert!(end.is_normalized());

        let start = self;
        let dot = start.dot(end);
        let bias = if dot >= 0.0 { 1.0 } else { -1.0 };
        let interpolated = start.add(end.mul(bias).sub(start).mul(s));
        interpolated.normalize()
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
    pub fn slerp(self, mut end: Self, s: f64) -> Self {
        // http://number-none.com/product/Understanding%20Slerp,%20Then%20Not%20Using%20It/
        glam_assert!(self.is_normalized());
        glam_assert!(end.is_normalized());

        const DOT_THRESHOLD: f64 = 0.9995;

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
            let theta = dot.acos_approx();

            let scale1 = (theta * (1.0 - s)).sin();
            let scale2 = (theta * s).sin();
            let theta_sin = theta.sin();

            self.mul(scale1).add(end.mul(scale2)).mul(theta_sin.recip())
        }
    }

    /// Multiplies a quaternion and a 3D vector, returning the rotated vector.
    ///
    /// # Panics
    ///
    /// Will panic if `self` is not normalized when `glam_assert` is enabled.
    #[inline]
    pub fn mul_vec3(self, rhs: DVec3) -> DVec3 {
        glam_assert!(self.is_normalized());

        let w = self.w;
        let b = DVec3::new(self.x, self.y, self.z);
        let b2 = b.dot(b);
        rhs.mul(w * w - b2)
            .add(b.mul(rhs.dot(b) * 2.0))
            .add(b.cross(rhs).mul(w * 2.0))
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
    pub fn mul_quat(self, rhs: Self) -> Self {
        glam_assert!(self.is_normalized());
        glam_assert!(rhs.is_normalized());

        let (x0, y0, z0, w0) = self.into();
        let (x1, y1, z1, w1) = rhs.into();
        Self::from_xyzw(
            w0 * x1 + x0 * w1 + y0 * z1 - z0 * y1,
            w0 * y1 - x0 * z1 + y0 * w1 + z0 * x1,
            w0 * z1 + x0 * y1 - y0 * x1 + z0 * w1,
            w0 * w1 - x0 * x1 - y0 * y1 - z0 * z1,
        )
    }

    /// Creates a quaternion from a 3x3 rotation matrix inside a 3D affine transform.
    #[inline]
    pub fn from_affine3(a: &crate::DAffine3) -> Self {
        #[allow(clippy::useless_conversion)]
        Self::from_rotation_axes(
            a.matrix3.x_axis.into(),
            a.matrix3.y_axis.into(),
            a.matrix3.z_axis.into(),
        )
    }

    #[inline]
    pub fn as_f32(self) -> Quat {
        Quat::from_xyzw(self.x as f32, self.y as f32, self.z as f32, self.w as f32)
    }
}

#[cfg(not(target_arch = "spirv"))]
impl fmt::Debug for DQuat {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_tuple(stringify!(DQuat))
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .field(&self.w)
            .finish()
    }
}

#[cfg(not(target_arch = "spirv"))]
impl fmt::Display for DQuat {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "[{}, {}, {}, {}]", self.x, self.y, self.z, self.w)
    }
}

impl Add<DQuat> for DQuat {
    type Output = Self;
    /// Adds two quaternions.
    ///
    /// The sum is not guaranteed to be normalized.
    ///
    /// Note that addition is not the same as combining the rotations represented by the
    /// two quaternions! That corresponds to multiplication.
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self::from_vec4(DVec4::from(self) + DVec4::from(rhs))
    }
}

impl Sub<DQuat> for DQuat {
    type Output = Self;
    /// Subtracts the `rhs` quaternion from `self`.
    ///
    /// The difference is not guaranteed to be normalized.
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self::from_vec4(DVec4::from(self) - DVec4::from(rhs))
    }
}

impl Mul<f64> for DQuat {
    type Output = Self;
    /// Multiplies a quaternion by a scalar value.
    ///
    /// The product is not guaranteed to be normalized.
    #[inline]
    fn mul(self, rhs: f64) -> Self {
        Self::from_vec4(DVec4::from(self) * rhs)
    }
}

impl Div<f64> for DQuat {
    type Output = Self;
    /// Divides a quaternion by a scalar value.
    /// The quotient is not guaranteed to be normalized.
    #[inline]
    fn div(self, rhs: f64) -> Self {
        Self::from_vec4(DVec4::from(self) / rhs)
    }
}

impl Mul<DQuat> for DQuat {
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

impl MulAssign<DQuat> for DQuat {
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

impl Mul<DVec3> for DQuat {
    type Output = DVec3;
    /// Multiplies a quaternion and a 3D vector, returning the rotated vector.
    ///
    /// # Panics
    ///
    /// Will panic if `self` is not normalized when `glam_assert` is enabled.
    #[inline]
    fn mul(self, rhs: DVec3) -> Self::Output {
        self.mul_vec3(rhs)
    }
}

impl Neg for DQuat {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        self * -1.0
    }
}

impl Default for DQuat {
    #[inline]
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl PartialEq for DQuat {
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        DVec4::from(*self).eq(&DVec4::from(*rhs))
    }
}

#[cfg(not(target_arch = "spirv"))]
impl AsRef<[f64; 4]> for DQuat {
    #[inline]
    fn as_ref(&self) -> &[f64; 4] {
        unsafe { &*(self as *const Self as *const [f64; 4]) }
    }
}

impl Sum<Self> for DQuat {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::ZERO, Self::add)
    }
}

impl<'a> Sum<&'a Self> for DQuat {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ZERO, |a, &b| Self::add(a, b))
    }
}

impl Product for DQuat {
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::IDENTITY, Self::mul)
    }
}

impl<'a> Product<&'a Self> for DQuat {
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::IDENTITY, |a, &b| Self::mul(a, b))
    }
}

impl From<DQuat> for DVec4 {
    #[inline]
    fn from(q: DQuat) -> Self {
        Self::new(q.x, q.y, q.z, q.w)
    }
}

impl From<DQuat> for (f64, f64, f64, f64) {
    #[inline]
    fn from(q: DQuat) -> Self {
        (q.x, q.y, q.z, q.w)
    }
}

impl From<DQuat> for [f64; 4] {
    #[inline]
    fn from(q: DQuat) -> Self {
        [q.x, q.y, q.z, q.w]
    }
}
