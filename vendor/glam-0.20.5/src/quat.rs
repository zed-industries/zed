use crate::core::traits::{
    quaternion::Quaternion,
    vector::{FloatVector4, MaskVector4, Vector, Vector4, Vector4Const},
};
use crate::euler::{EulerFromQuaternion, EulerRot, EulerToQuaternion};
use crate::{DMat3, DMat4, DVec2, DVec3, DVec4};
use crate::{Mat3, Mat4, Vec2, Vec3, Vec3A, Vec4};

#[cfg(not(feature = "std"))]
use num_traits::Float;

#[cfg(all(
    target_arch = "x86",
    target_feature = "sse2",
    not(feature = "scalar-math")
))]
use core::arch::x86::*;
#[cfg(all(
    target_arch = "x86_64",
    target_feature = "sse2",
    not(feature = "scalar-math")
))]
use core::arch::x86_64::*;

#[cfg(all(target_feature = "simd128", not(feature = "scalar-math")))]
use core::arch::wasm32::v128;

#[cfg(not(target_arch = "spirv"))]
use core::fmt;
use core::iter::{Product, Sum};
use core::ops::{Add, Deref, Div, Mul, MulAssign, Neg, Sub};

macro_rules! impl_quat_methods {
    ($t:ident, $quat:ident, $vec2:ident, $vec3:ident, $vec4:ident, $mat3:ident, $mat4:ident, $inner:ident) => {
        /// The identity quaternion. Corresponds to no rotation.
        pub const IDENTITY: Self = Self($inner::W);

        /// All NAN:s.
        pub const NAN: Self = Self(<$inner as crate::core::traits::scalar::NanConstEx>::NAN);

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
        pub fn from_xyzw(x: $t, y: $t, z: $t, w: $t) -> Self {
            Self(Vector4::new(x, y, z, w))
        }

        /// Creates a rotation quaternion from an array.
        ///
        /// # Preconditions
        ///
        /// This function does not check if the input is normalized, it is up to the user to
        /// provide normalized input or to normalized the resulting quaternion.
        #[inline(always)]
        pub fn from_array(a: [$t; 4]) -> Self {
            let q = Vector4::from_array(a);
            Self(q)
        }

        /// Creates a new rotation quaternion from a 4D vector.
        ///
        /// # Preconditions
        ///
        /// This function does not check if the input is normalized, it is up to the user to
        /// provide normalized input or to normalized the resulting quaternion.
        #[inline(always)]
        pub fn from_vec4(v: $vec4) -> Self {
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
        #[inline(always)]
        pub fn from_slice(slice: &[$t]) -> Self {
            Self(Vector4::from_slice_unaligned(slice))
        }

        /// Writes the quaternion to an unaligned slice.
        ///
        /// # Panics
        ///
        /// Panics if `slice` length is less than 4.
        #[inline(always)]
        pub fn write_to_slice(self, slice: &mut [$t]) {
            Vector4::write_to_slice_unaligned(self.0, slice)
        }

        /// Create a quaternion for a normalized rotation `axis` and `angle` (in radians).
        /// The axis must be normalized (unit-length).
        ///
        /// # Panics
        ///
        /// Will panic if `axis` is not normalized when `glam_assert` is enabled.
        #[inline(always)]
        pub fn from_axis_angle(axis: $vec3, angle: $t) -> Self {
            Self($inner::from_axis_angle(axis.0, angle))
        }

        /// Create a quaternion that rotates `v.length()` radians around `v.normalize()`.
        ///
        /// `from_scaled_axis(Vec3::ZERO)` results in the identity quaternion.
        #[inline(always)]
        pub fn from_scaled_axis(v: $vec3) -> Self {
            // Self($inner::from_scaled_axis(v.0))
            let length = v.length();
            if length == 0.0 {
                Self::IDENTITY
            } else {
                Self::from_axis_angle(v / length, length)
            }
        }

        /// Creates a quaternion from the `angle` (in radians) around the x axis.
        #[inline(always)]
        pub fn from_rotation_x(angle: $t) -> Self {
            Self($inner::from_rotation_x(angle))
        }

        /// Creates a quaternion from the `angle` (in radians) around the y axis.
        #[inline(always)]
        pub fn from_rotation_y(angle: $t) -> Self {
            Self($inner::from_rotation_y(angle))
        }

        /// Creates a quaternion from the `angle` (in radians) around the z axis.
        #[inline(always)]
        pub fn from_rotation_z(angle: $t) -> Self {
            Self($inner::from_rotation_z(angle))
        }

        #[inline(always)]
        /// Creates a quaternion from the given euler rotation sequence and the angles (in radians).
        pub fn from_euler(euler: EulerRot, a: $t, b: $t, c: $t) -> Self {
            euler.new_quat(a, b, c)
        }

        /// Creates a quaternion from a 3x3 rotation matrix.
        #[inline]
        pub fn from_mat3(mat: &$mat3) -> Self {
            Self(Quaternion::from_rotation_axes(
                mat.x_axis.0,
                mat.y_axis.0,
                mat.z_axis.0,
            ))
        }

        /// Creates a quaternion from a 3x3 rotation matrix inside a homogeneous 4x4 matrix.
        #[inline]
        pub fn from_mat4(mat: &$mat4) -> Self {
            Self(Quaternion::from_rotation_axes(
                mat.x_axis.0.into(),
                mat.y_axis.0.into(),
                mat.z_axis.0.into(),
            ))
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
        pub fn from_rotation_arc(from: $vec3, to: $vec3) -> Self {
            glam_assert!(from.is_normalized());
            glam_assert!(to.is_normalized());

            const ONE_MINUS_EPS: $t = 1.0 - 2.0 * core::$t::EPSILON;
            let dot = from.dot(to);
            if dot > ONE_MINUS_EPS {
                // 0° singulary: from ≈ to
                Self::IDENTITY
            } else if dot < -ONE_MINUS_EPS {
                // 180° singulary: from ≈ -to
                use core::$t::consts::PI; // half a turn = 𝛕/2 = 180°
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
        pub fn from_rotation_arc_colinear(from: $vec3, to: $vec3) -> Self {
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
        pub fn from_rotation_arc_2d(from: $vec2, to: $vec2) -> Self {
            glam_assert!(from.is_normalized());
            glam_assert!(to.is_normalized());

            const ONE_MINUS_EPSILON: $t = 1.0 - 2.0 * core::$t::EPSILON;
            let dot = from.dot(to);
            if dot > ONE_MINUS_EPSILON {
                // 0° singulary: from ≈ to
                Self::IDENTITY
            } else if dot < -ONE_MINUS_EPSILON {
                // 180° singulary: from ≈ -to
                const COS_FRAC_PI_2: $t = 0.0;
                const SIN_FRAC_PI_2: $t = 1.0;
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
        #[inline(always)]
        pub fn to_axis_angle(self) -> ($vec3, $t) {
            let (axis, angle) = self.0.to_axis_angle();
            ($vec3(axis), angle)
        }

        /// Returns the rotation axis scaled by the rotation in radians.
        #[inline(always)]
        pub fn to_scaled_axis(self) -> $vec3 {
            let (axis, angle) = self.0.to_axis_angle();
            $vec3(axis) * angle
        }

        /// Returns the rotation angles for the given euler rotation sequence.
        #[inline(always)]
        pub fn to_euler(self, euler: EulerRot) -> ($t, $t, $t) {
            euler.convert_quat(self)
        }

        /// `[x, y, z, w]`
        #[inline(always)]
        pub fn to_array(&self) -> [$t; 4] {
            [self.x, self.y, self.z, self.w]
        }

        /// Returns the vector part of the quaternion.
        #[inline(always)]
        pub fn xyz(self) -> $vec3 {
            $vec3::new(self.x, self.y, self.z)
        }

        /// Returns the quaternion conjugate of `self`. For a unit quaternion the
        /// conjugate is also the inverse.
        #[must_use]
        #[inline(always)]
        pub fn conjugate(self) -> Self {
            Self(self.0.conjugate())
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
        #[inline(always)]
        pub fn inverse(self) -> Self {
            glam_assert!(self.is_normalized());
            self.conjugate()
        }

        /// Computes the dot product of `self` and `other`. The dot product is
        /// equal to the the cosine of the angle between two quaternion rotations.
        #[inline(always)]
        pub fn dot(self, other: Self) -> $t {
            Vector4::dot(self.0, other.0)
        }

        /// Computes the length of `self`.
        #[doc(alias = "magnitude")]
        #[inline(always)]
        pub fn length(self) -> $t {
            FloatVector4::length(self.0)
        }

        /// Computes the squared length of `self`.
        ///
        /// This is generally faster than `length()` as it avoids a square
        /// root operation.
        #[doc(alias = "magnitude2")]
        #[inline(always)]
        pub fn length_squared(self) -> $t {
            FloatVector4::length_squared(self.0)
        }

        /// Computes `1.0 / length()`.
        ///
        /// For valid results, `self` must _not_ be of length zero.
        #[inline(always)]
        pub fn length_recip(self) -> $t {
            FloatVector4::length_recip(self.0)
        }

        /// Returns `self` normalized to length 1.0.
        ///
        /// For valid results, `self` must _not_ be of length zero.
        ///
        /// Panics
        ///
        /// Will panic if `self` is zero length when `glam_assert` is enabled.
        #[must_use]
        #[inline(always)]
        pub fn normalize(self) -> Self {
            Self(FloatVector4::normalize(self.0))
        }

        /// Returns `true` if, and only if, all elements are finite.
        /// If any element is either `NaN`, positive or negative infinity, this will return `false`.
        #[inline(always)]
        pub fn is_finite(self) -> bool {
            FloatVector4::is_finite(self.0)
        }

        #[inline(always)]
        pub fn is_nan(self) -> bool {
            FloatVector4::is_nan(self.0)
        }

        /// Returns whether `self` of length `1.0` or not.
        ///
        /// Uses a precision threshold of `1e-6`.
        #[inline(always)]
        pub fn is_normalized(self) -> bool {
            FloatVector4::is_normalized(self.0)
        }

        #[inline(always)]
        pub fn is_near_identity(self) -> bool {
            self.0.is_near_identity()
        }

        /// Returns the angle (in radians) for the minimal rotation
        /// for transforming this quaternion into another.
        ///
        /// Both quaternions must be normalized.
        ///
        /// # Panics
        ///
        /// Will panic if `self` or `other` are not normalized when `glam_assert` is enabled.
        pub fn angle_between(self, other: Self) -> $t {
            glam_assert!(self.is_normalized() && other.is_normalized());
            use crate::core::traits::scalar::FloatEx;
            self.dot(other).abs().acos_approx() * 2.0
        }

        /// Returns true if the absolute difference of all elements between `self` and `other`
        /// is less than or equal to `max_abs_diff`.
        ///
        /// This can be used to compare if two quaternions contain similar elements. It works
        /// best when comparing with a known value. The `max_abs_diff` that should be used used
        /// depends on the values being compared against.
        ///
        /// For more see
        /// [comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
        #[inline(always)]
        pub fn abs_diff_eq(self, other: Self, max_abs_diff: $t) -> bool {
            FloatVector4::abs_diff_eq(self.0, other.0, max_abs_diff)
        }

        /// Performs a linear interpolation between `self` and `other` based on
        /// the value `s`.
        ///
        /// When `s` is `0.0`, the result will be equal to `self`.  When `s`
        /// is `1.0`, the result will be equal to `other`.
        ///
        /// # Panics
        ///
        /// Will panic if `self` or `end` are not normalized when `glam_assert` is enabled.
        #[inline(always)]
        #[doc(alias = "mix")]
        pub fn lerp(self, end: Self, s: $t) -> Self {
            Self(self.0.lerp(end.0, s))
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
        #[inline(always)]
        pub fn slerp(self, end: Self, s: $t) -> Self {
            Self(self.0.slerp(end.0, s))
        }

        /// Multiplies a quaternion and a 3D vector, returning the rotated vector.
        ///
        /// # Panics
        ///
        /// Will panic if `self` is not normalized when `glam_assert` is enabled.
        #[inline(always)]
        pub fn mul_vec3(self, other: $vec3) -> $vec3 {
            $vec3(self.0.mul_vector3(other.0))
        }

        /// Multiplies two quaternions. If they each represent a rotation, the result will
        /// represent the combined rotation.
        ///
        /// Note that due to floating point rounding the result may not be perfectly normalized.
        ///
        /// # Panics
        ///
        /// Will panic if `self` or `other` are not normalized when `glam_assert` is enabled.
        #[inline(always)]
        pub fn mul_quat(self, other: Self) -> Self {
            Self(self.0.mul_quaternion(other.0))
        }
    };
}

macro_rules! impl_quat_traits {
    ($t:ty, $new:ident, $quat:ident, $vec3:ident, $vec4:ident, $inner:ident) => {
        /// Creates a quaternion from `x`, `y`, `z` and `w` values.
        ///
        /// This should generally not be called manually unless you know what you are doing. Use
        /// one of the other constructors instead such as `identity` or `from_axis_angle`.
        #[inline]
        pub fn $new(x: $t, y: $t, z: $t, w: $t) -> $quat {
            $quat::from_xyzw(x, y, z, w)
        }

        #[cfg(not(target_arch = "spirv"))]
        impl fmt::Debug for $quat {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_tuple(stringify!($quat))
                    .field(&self.x)
                    .field(&self.y)
                    .field(&self.z)
                    .field(&self.w)
                    .finish()
            }
        }

        #[cfg(not(target_arch = "spirv"))]
        impl fmt::Display for $quat {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(fmt, "[{}, {}, {}, {}]", self.x, self.y, self.z, self.w)
            }
        }

        impl Add<$quat> for $quat {
            type Output = Self;
            /// Adds two quaternions.
            ///
            /// The sum is not guaranteed to be normalized.
            ///
            /// Note that addition is not the same as combining the rotations represented by the
            /// two quaternions! That corresponds to multiplication.
            #[inline]
            fn add(self, other: Self) -> Self {
                Self(self.0.add(other.0))
            }
        }

        impl Sub<$quat> for $quat {
            type Output = Self;
            /// Subtracts the other quaternion from self.
            ///
            /// The difference is not guaranteed to be normalized.
            #[inline]
            fn sub(self, other: Self) -> Self {
                Self(self.0.sub(other.0))
            }
        }

        impl Mul<$t> for $quat {
            type Output = Self;
            /// Multiplies a quaternion by a scalar value.
            ///
            /// The product is not guaranteed to be normalized.
            #[inline]
            fn mul(self, other: $t) -> Self {
                Self(self.0.scale(other))
            }
        }

        impl Div<$t> for $quat {
            type Output = Self;
            /// Divides a quaternion by a scalar value.
            /// The quotient is not guaranteed to be normalized.
            #[inline]
            fn div(self, other: $t) -> Self {
                Self(self.0.scale(other.recip()))
            }
        }

        impl Mul<$quat> for $quat {
            type Output = Self;
            /// Multiplies two quaternions. If they each represent a rotation, the result will
            /// represent the combined rotation.
            ///
            /// Note that due to floating point rounding the result may not be perfectly
            /// normalized.
            ///
            /// # Panics
            ///
            /// Will panic if `self` or `other` are not normalized when `glam_assert` is enabled.
            #[inline]
            fn mul(self, other: Self) -> Self {
                Self(self.0.mul_quaternion(other.0))
            }
        }

        impl MulAssign<$quat> for $quat {
            /// Multiplies two quaternions. If they each represent a rotation, the result will
            /// represent the combined rotation.
            ///
            /// Note that due to floating point rounding the result may not be perfectly
            /// normalized.
            ///
            /// # Panics
            ///
            /// Will panic if `self` or `other` are not normalized when `glam_assert` is enabled.
            #[inline]
            fn mul_assign(&mut self, other: Self) {
                self.0 = self.0.mul_quaternion(other.0);
            }
        }

        impl Mul<$vec3> for $quat {
            type Output = $vec3;
            /// Multiplies a quaternion and a 3D vector, returning the rotated vector.
            ///
            /// # Panics
            ///
            /// Will panic if `self` is not normalized when `glam_assert` is enabled.
            #[inline]
            fn mul(self, other: $vec3) -> Self::Output {
                $vec3(self.0.mul_vector3(other.0))
            }
        }

        impl Neg for $quat {
            type Output = Self;
            #[inline]
            fn neg(self) -> Self {
                Self(self.0.scale(-1.0))
            }
        }

        impl Default for $quat {
            #[inline]
            fn default() -> Self {
                Self::IDENTITY
            }
        }

        impl PartialEq for $quat {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                MaskVector4::all(self.0.cmpeq(other.0))
            }
        }

        #[cfg(not(target_arch = "spirv"))]
        impl AsRef<[$t; 4]> for $quat {
            #[inline(always)]
            fn as_ref(&self) -> &[$t; 4] {
                unsafe { &*(self as *const Self as *const [$t; 4]) }
            }
        }

        #[cfg(not(target_arch = "spirv"))]
        impl AsMut<[$t; 4]> for $quat {
            #[inline(always)]
            fn as_mut(&mut self) -> &mut [$t; 4] {
                unsafe { &mut *(self as *mut Self as *mut [$t; 4]) }
            }
        }

        impl From<$quat> for $vec4 {
            #[inline(always)]
            fn from(q: $quat) -> Self {
                $vec4(q.0)
            }
        }

        impl From<$quat> for ($t, $t, $t, $t) {
            #[inline(always)]
            fn from(q: $quat) -> Self {
                Vector4::into_tuple(q.0)
            }
        }

        impl From<$quat> for [$t; 4] {
            #[inline(always)]
            fn from(q: $quat) -> Self {
                Vector4::into_array(q.0)
            }
        }

        impl From<$quat> for $inner {
            // TODO: write test
            #[inline(always)]
            fn from(q: $quat) -> Self {
                q.0
            }
        }

        impl Deref for $quat {
            type Target = crate::XYZW<$t>;
            #[inline(always)]
            fn deref(&self) -> &Self::Target {
                self.0.as_ref_xyzw()
            }
        }

        impl<'a> Sum<&'a Self> for $quat {
            fn sum<I>(iter: I) -> Self
            where
                I: Iterator<Item = &'a Self>,
            {
                use crate::core::traits::vector::VectorConst;
                iter.fold(Self($inner::ZERO), |a, &b| Self::add(a, b))
            }
        }

        impl<'a> Product<&'a Self> for $quat {
            fn product<I>(iter: I) -> Self
            where
                I: Iterator<Item = &'a Self>,
            {
                iter.fold(Self::IDENTITY, |a, &b| Self::mul(a, b))
            }
        }
    };
}

#[cfg(all(target_feature = "sse2", not(feature = "scalar-math")))]
type InnerF32 = __m128;

#[cfg(all(target_feature = "simd128", not(feature = "scalar-math")))]
type InnerF32 = v128;

#[cfg(any(
    not(any(target_feature = "sse2", target_feature = "simd128")),
    feature = "scalar-math"
))]
type InnerF32 = crate::XYZW<f32>;

/// A quaternion representing an orientation.
///
/// This quaternion is intended to be of unit length but may denormalize due to
/// floating point "error creep" which can occur when successive quaternion
/// operations are applied.
///
/// This type is 16 byte aligned.
#[derive(Clone, Copy)]
#[cfg_attr(
    not(any(
        feature = "scalar-math",
        target_arch = "spirv",
        target_feature = "sse2",
        target_feature = "simd128"
    )),
    repr(C, align(16))
)]
#[cfg_attr(
    any(
        feature = "scalar-math",
        target_arch = "spirv",
        target_feature = "sse2",
        target_feature = "simd128"
    ),
    repr(transparent)
)]
pub struct Quat(pub(crate) InnerF32);

impl Quat {
    impl_quat_methods!(f32, Quat, Vec2, Vec3, Vec4, Mat3, Mat4, InnerF32);

    /// Multiplies a quaternion and a 3D vector, returning the rotated vector.
    #[inline(always)]
    pub fn mul_vec3a(self, other: Vec3A) -> Vec3A {
        #[allow(clippy::useless_conversion)]
        Vec3A(self.0.mul_float4_as_vector3(other.0.into()).into())
    }

    #[inline(always)]
    pub fn as_f64(self) -> DQuat {
        DQuat::from_xyzw(self.x as f64, self.y as f64, self.z as f64, self.w as f64)
    }

    /// Creates a quaternion from a 3x3 rotation matrix inside a 3D affine transform.
    #[inline]
    pub fn from_affine3(mat: &crate::Affine3A) -> Self {
        Self(Quaternion::from_rotation_axes(
            mat.x_axis.0.into(),
            mat.y_axis.0.into(),
            mat.z_axis.0.into(),
        ))
    }
}
impl_quat_traits!(f32, quat, Quat, Vec3, Vec4, InnerF32);

impl Mul<Vec3A> for Quat {
    type Output = Vec3A;
    #[inline(always)]
    fn mul(self, other: Vec3A) -> Self::Output {
        self.mul_vec3a(other)
    }
}

type InnerF64 = crate::XYZW<f64>;

/// A quaternion representing an orientation.
///
/// This quaternion is intended to be of unit length but may denormalize due to
/// floating point "error creep" which can occur when successive quaternion
/// operations are applied.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct DQuat(pub(crate) InnerF64);

impl DQuat {
    impl_quat_methods!(f64, DQuat, DVec2, DVec3, DVec4, DMat3, DMat4, InnerF64);

    #[inline(always)]
    pub fn as_f32(self) -> Quat {
        Quat::from_xyzw(self.x as f32, self.y as f32, self.z as f32, self.w as f32)
    }

    /// Creates a quaternion from a 3x3 rotation matrix inside a 3D affine transform.
    #[inline]
    pub fn from_affine3(mat: &crate::DAffine3) -> Self {
        Self(Quaternion::from_rotation_axes(
            mat.x_axis.0,
            mat.y_axis.0,
            mat.z_axis.0,
        ))
    }
}
impl_quat_traits!(f64, dquat, DQuat, DVec3, DVec4, InnerF64);

#[cfg(any(feature = "scalar-math", target_arch = "spirv"))]
mod const_test_quat {
    const_assert_eq!(
        core::mem::align_of::<f32>(),
        core::mem::align_of::<super::Quat>()
    );
    const_assert_eq!(16, core::mem::size_of::<super::Quat>());
}

#[cfg(not(any(feature = "scalar-math", target_arch = "spirv")))]
mod const_test_quat {
    const_assert_eq!(16, core::mem::align_of::<super::Quat>());
    const_assert_eq!(16, core::mem::size_of::<super::Quat>());
}

mod const_test_dquat {
    const_assert_eq!(
        core::mem::align_of::<f64>(),
        core::mem::align_of::<super::DQuat>()
    );
    const_assert_eq!(32, core::mem::size_of::<super::DQuat>());
}
