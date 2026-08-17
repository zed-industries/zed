/*
Conversion from quaternions to Euler rotation sequences.

From: http://bediyap.com/programming/convert-quaternion-to-euler-rotations/
*/

use super::quat::{DQuat, Quat};
use crate::core::traits::scalar::*;

/// Euler rotation sequences.
///
/// The angles are applied starting from the right.
/// E.g. XYZ will first apply the z-axis rotation.
///
/// YXZ can be used for yaw (y-axis), pitch (x-axis), roll (z-axis).
///
/// The two-axis rotations (e.g. ZYZ) are not fully tested and have to be treated with caution.
#[derive(Debug, Clone, Copy)]
pub enum EulerRot {
    /// Intrinsic three-axis rotation ZYX
    ZYX,
    /// Intrinsic three-axis rotation ZXY
    ZXY,
    /// Intrinsic three-axis rotation YXZ
    YXZ,
    /// Intrinsic three-axis rotation YZX
    YZX,
    /// Intrinsic three-axis rotation XYZ
    XYZ,
    /// Intrinsic three-axis rotation XZY
    XZY,

    /// Intrinsic two-axis rotation ZYZ
    #[deprecated(note = "Untested! Use at own risk!")]
    ZYZ,
    /// Intrinsic two-axis rotation ZXZ
    #[deprecated(note = "Untested! Use at own risk!")]
    ZXZ,
    /// Intrinsic two-axis rotation YXY
    #[deprecated(note = "Untested! Use at own risk!")]
    YXY,
    /// Intrinsic two-axis rotation YZY
    #[deprecated(note = "Untested! Use at own risk!")]
    YZY,
    /// Intrinsic two-axis rotation XYX
    #[deprecated(note = "Untested! Use at own risk!")]
    XYX,
    /// Intrinsic two-axis rotation XZX
    #[deprecated(note = "Untested! Use at own risk!")]
    XZX,
}

impl Default for EulerRot {
    /// Default `YXZ` as yaw (y-axis), pitch (x-axis), roll (z-axis).
    fn default() -> Self {
        Self::YXZ
    }
}

/// Conversion from quaternion to euler angles.
pub trait EulerFromQuaternion<Q: Copy>: Sized + Copy {
    type Output: FloatEx;
    /// Compute the angle of the first axis (X-x-x)
    fn first(self, q: Q) -> Self::Output;
    /// Compute then angle of the second axis (x-X-x)
    fn second(self, q: Q) -> Self::Output;
    /// Compute then angle of the third axis (x-x-X)
    fn third(self, q: Q) -> Self::Output;

    /// Compute all angles of a rotation in the notation order
    fn convert_quat(self, q: Q) -> (Self::Output, Self::Output, Self::Output) {
        (self.first(q), self.second(q), self.third(q))
    }
}

/// Conversion from euler angles to quaternion.
pub trait EulerToQuaternion<T>: Copy {
    type Output;
    /// Create the rotation quaternion for the three angles of this euler rotation sequence.
    fn new_quat(self, u: T, v: T, w: T) -> Self::Output;
}

/// Adds a atan2 that handles the negative zero case.
/// Basically forces positive zero in the x-argument for atan2.
pub trait Atan2Fixed<T = Self> {
    fn atan2_fixed(self, other: T) -> T;
}

impl Atan2Fixed for f32 {
    fn atan2_fixed(self, other: f32) -> f32 {
        self.atan2(if other == 0.0f32 { 0.0f32 } else { other })
    }
}
impl Atan2Fixed for f64 {
    fn atan2_fixed(self, other: f64) -> f64 {
        self.atan2(if other == 0.0f64 { 0.0f64 } else { other })
    }
}

macro_rules! impl_from_quat {
    ($t:ty, $quat:ident) => {
        impl EulerFromQuaternion<$quat> for EulerRot {
            type Output = $t;
            fn first(self, q: $quat) -> $t {
                use EulerRot::*;
                match self {
                    ZYX => (Self::Output::TWO * (q.x * q.y + q.w * q.z))
                        .atan2(q.w * q.w + q.x * q.x - q.y * q.y - q.z * q.z),
                    ZXY => (-Self::Output::TWO * (q.x * q.y - q.w * q.z))
                        .atan2(q.w * q.w - q.x * q.x + q.y * q.y - q.z * q.z),
                    YXZ => (Self::Output::TWO * (q.x * q.z + q.w * q.y))
                        .atan2(q.w * q.w - q.x * q.x - q.y * q.y + q.z * q.z),
                    YZX => (-Self::Output::TWO * (q.x * q.z - q.w * q.y))
                        .atan2(q.w * q.w + q.x * q.x - q.y * q.y - q.z * q.z),
                    XYZ => (-Self::Output::TWO * (q.y * q.z - q.w * q.x))
                        .atan2(q.w * q.w - q.x * q.x - q.y * q.y + q.z * q.z),
                    XZY => (Self::Output::TWO * (q.y * q.z + q.w * q.x))
                        .atan2(q.w * q.w - q.x * q.x + q.y * q.y - q.z * q.z),
                    #[allow(deprecated)]
                    ZYZ => (Self::Output::TWO * (q.y * q.z + q.w * q.x))
                        .atan2_fixed(-Self::Output::TWO * (q.x * q.z - q.w * q.y)),
                    #[allow(deprecated)]
                    ZXZ => (Self::Output::TWO * (q.x * q.z - q.w * q.y))
                        .atan2_fixed(Self::Output::TWO * (q.y * q.z + q.w * q.x)),
                    #[allow(deprecated)]
                    YXY => (Self::Output::TWO * (q.x * q.y + q.w * q.z))
                        .atan2_fixed(-Self::Output::TWO * (q.y * q.z - q.w * q.x)),
                    #[allow(deprecated)]
                    YZY => (Self::Output::TWO * (q.y * q.z - q.w * q.x))
                        .atan2_fixed(Self::Output::TWO * (q.x * q.y + q.w * q.z)),
                    #[allow(deprecated)]
                    XYX => (Self::Output::TWO * (q.x * q.y - q.w * q.z))
                        .atan2_fixed(Self::Output::TWO * (q.x * q.z + q.w * q.y)),
                    #[allow(deprecated)]
                    XZX => (Self::Output::TWO * (q.x * q.z + q.w * q.y))
                        .atan2_fixed(-Self::Output::TWO * (q.x * q.y - q.w * q.z)),
                }
            }

            fn second(self, q: $quat) -> $t {
                use EulerRot::*;

                /// Clamp number to range [-1,1](-1,1) for asin() and acos(), else NaN is possible.
                #[inline(always)]
                fn arc_clamp<T: FloatEx>(val: T) -> T {
                    NumEx::min(NumEx::max(val, T::NEG_ONE), T::ONE)
                }

                match self {
                    ZYX => arc_clamp(-Self::Output::TWO * (q.x * q.z - q.w * q.y)).asin(),
                    ZXY => arc_clamp(Self::Output::TWO * (q.y * q.z + q.w * q.x)).asin(),
                    YXZ => arc_clamp(-Self::Output::TWO * (q.y * q.z - q.w * q.x)).asin(),
                    YZX => arc_clamp(Self::Output::TWO * (q.x * q.y + q.w * q.z)).asin(),
                    XYZ => arc_clamp(Self::Output::TWO * (q.x * q.z + q.w * q.y)).asin(),
                    XZY => arc_clamp(-Self::Output::TWO * (q.x * q.y - q.w * q.z)).asin(),
                    #[allow(deprecated)]
                    ZYZ => arc_clamp(q.w * q.w - q.x * q.x - q.y * q.y + q.z * q.z).acos(),
                    #[allow(deprecated)]
                    ZXZ => arc_clamp(q.w * q.w - q.x * q.x - q.y * q.y + q.z * q.z).acos(),
                    #[allow(deprecated)]
                    YXY => arc_clamp(q.w * q.w - q.x * q.x + q.y * q.y - q.z * q.z).acos(),
                    #[allow(deprecated)]
                    YZY => arc_clamp(q.w * q.w - q.x * q.x + q.y * q.y - q.z * q.z).acos(),
                    #[allow(deprecated)]
                    XYX => arc_clamp(q.w * q.w + q.x * q.x - q.y * q.y - q.z * q.z).acos(),
                    #[allow(deprecated)]
                    XZX => arc_clamp(q.w * q.w + q.x * q.x - q.y * q.y - q.z * q.z).acos(),
                }
            }

            fn third(self, q: $quat) -> $t {
                use EulerRot::*;
                #[allow(deprecated)]
                match self {
                    ZYX => (Self::Output::TWO * (q.y * q.z + q.w * q.x))
                        .atan2(q.w * q.w - q.x * q.x - q.y * q.y + q.z * q.z),
                    ZXY => (-Self::Output::TWO * (q.x * q.z - q.w * q.y))
                        .atan2(q.w * q.w - q.x * q.x - q.y * q.y + q.z * q.z),
                    YXZ => (Self::Output::TWO * (q.x * q.y + q.w * q.z))
                        .atan2(q.w * q.w - q.x * q.x + q.y * q.y - q.z * q.z),
                    YZX => (-Self::Output::TWO * (q.y * q.z - q.w * q.x))
                        .atan2(q.w * q.w - q.x * q.x + q.y * q.y - q.z * q.z),
                    XYZ => (-Self::Output::TWO * (q.x * q.y - q.w * q.z))
                        .atan2(q.w * q.w + q.x * q.x - q.y * q.y - q.z * q.z),
                    XZY => (Self::Output::TWO * (q.x * q.z + q.w * q.y))
                        .atan2(q.w * q.w + q.x * q.x - q.y * q.y - q.z * q.z),
                    #[allow(deprecated)]
                    ZYZ => (Self::Output::TWO * (q.y * q.z - q.w * q.x))
                        .atan2_fixed(Self::Output::TWO * (q.x * q.z + q.w * q.y)),
                    #[allow(deprecated)]
                    ZXZ => (Self::Output::TWO * (q.x * q.z + q.w * q.y))
                        .atan2_fixed(-Self::Output::TWO * (q.y * q.z - q.w * q.x)),
                    #[allow(deprecated)]
                    YXY => (Self::Output::TWO * (q.x * q.y - q.w * q.z))
                        .atan2_fixed(Self::Output::TWO * (q.y * q.z + q.w * q.x)),
                    #[allow(deprecated)]
                    YZY => (Self::Output::TWO * (q.y * q.z + q.w * q.x))
                        .atan2_fixed(-Self::Output::TWO * (q.x * q.y - q.w * q.z)),
                    #[allow(deprecated)]
                    XYX => (Self::Output::TWO * (q.x * q.y + q.w * q.z))
                        .atan2_fixed(-Self::Output::TWO * (q.x * q.z - q.w * q.y)),
                    #[allow(deprecated)]
                    XZX => (Self::Output::TWO * (q.x * q.z - q.w * q.y))
                        .atan2_fixed(Self::Output::TWO * (q.x * q.y + q.w * q.z)),
                }
            }
        }
        // End - impl EulerFromQuaternion
    };
}

macro_rules! impl_to_quat {
    ($t:ty, $quat:ident) => {
        impl EulerToQuaternion<$t> for EulerRot {
            type Output = $quat;
            #[inline(always)]
            fn new_quat(self, u: $t, v: $t, w: $t) -> $quat {
                use EulerRot::*;
                #[inline(always)]
                fn rot_x(a: $t) -> $quat {
                    $quat::from_rotation_x(a)
                }
                #[inline(always)]
                fn rot_y(a: $t) -> $quat {
                    $quat::from_rotation_y(a)
                }
                #[inline(always)]
                fn rot_z(a: $t) -> $quat {
                    $quat::from_rotation_z(a)
                }
                match self {
                    ZYX => rot_z(u) * rot_y(v) * rot_x(w),
                    ZXY => rot_z(u) * rot_x(v) * rot_y(w),
                    YXZ => rot_y(u) * rot_x(v) * rot_z(w),
                    YZX => rot_y(u) * rot_z(v) * rot_x(w),
                    XYZ => rot_x(u) * rot_y(v) * rot_z(w),
                    XZY => rot_x(u) * rot_z(v) * rot_y(w),
                    #[allow(deprecated)]
                    ZYZ => rot_z(u) * rot_y(v) * rot_z(w),
                    #[allow(deprecated)]
                    ZXZ => rot_z(u) * rot_x(v) * rot_z(w),
                    #[allow(deprecated)]
                    YXY => rot_y(u) * rot_x(v) * rot_y(w),
                    #[allow(deprecated)]
                    YZY => rot_y(u) * rot_z(v) * rot_y(w),
                    #[allow(deprecated)]
                    XYX => rot_x(u) * rot_y(v) * rot_x(w),
                    #[allow(deprecated)]
                    XZX => rot_x(u) * rot_z(v) * rot_x(w),
                }
                .normalize()
            }
        }
        // End - impl EulerToQuaternion
    };
}

impl_from_quat!(f32, Quat);
impl_from_quat!(f64, DQuat);
impl_to_quat!(f32, Quat);
impl_to_quat!(f64, DQuat);
