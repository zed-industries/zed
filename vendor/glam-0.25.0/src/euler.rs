/*
Conversion from quaternions to Euler rotation sequences.

From: http://bediyap.com/programming/convert-quaternion-to-euler-rotations/
*/

use crate::{DQuat, Quat};

/// Euler rotation sequences.
///
/// The angles are applied starting from the right.
/// E.g. XYZ will first apply the z-axis rotation.
///
/// YXZ can be used for yaw (y-axis), pitch (x-axis), roll (z-axis).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
}

impl Default for EulerRot {
    /// Default `YXZ` as yaw (y-axis), pitch (x-axis), roll (z-axis).
    fn default() -> Self {
        Self::YXZ
    }
}

/// Conversion from quaternion to euler angles.
pub(crate) trait EulerFromQuaternion<Q: Copy>: Sized + Copy {
    type Output;
    /// Compute the angle of the first axis (X-x-x)
    fn first(self, q: Q) -> Self::Output;
    /// Compute then angle of the second axis (x-X-x)
    fn second(self, q: Q) -> Self::Output;
    /// Compute then angle of the third axis (x-x-X)
    fn third(self, q: Q) -> Self::Output;

    /// Compute all angles of a rotation in the notation order
    fn convert_quat(self, q: Q) -> (Self::Output, Self::Output, Self::Output);

    #[doc(hidden)]
    fn sine_theta(self, q: Q) -> Self::Output;
}

/// Conversion from euler angles to quaternion.
pub(crate) trait EulerToQuaternion<T>: Copy {
    type Output;
    /// Create the rotation quaternion for the three angles of this euler rotation sequence.
    fn new_quat(self, u: T, v: T, w: T) -> Self::Output;
}

macro_rules! impl_from_quat {
    ($t:ident, $quat:ident) => {
        impl EulerFromQuaternion<$quat> for EulerRot {
            type Output = $t;

            fn sine_theta(self, q: $quat) -> $t {
                use EulerRot::*;
                match self {
                    ZYX => -2.0 * (q.x * q.z - q.w * q.y),
                    ZXY => 2.0 * (q.y * q.z + q.w * q.x),
                    YXZ => -2.0 * (q.y * q.z - q.w * q.x),
                    YZX => 2.0 * (q.x * q.y + q.w * q.z),
                    XYZ => 2.0 * (q.x * q.z + q.w * q.y),
                    XZY => -2.0 * (q.x * q.y - q.w * q.z),
                }
                .clamp(-1.0, 1.0)
            }

            fn first(self, q: $quat) -> $t {
                use crate::$t::math;
                use EulerRot::*;

                let sine_theta = self.sine_theta(q);
                if math::abs(sine_theta) > 0.99999 {
                    let scale = 2.0 * math::signum(sine_theta);

                    match self {
                        ZYX => scale * math::atan2(-q.x, q.w),
                        ZXY => scale * math::atan2(q.y, q.w),
                        YXZ => scale * math::atan2(-q.z, q.w),
                        YZX => scale * math::atan2(q.x, q.w),
                        XYZ => scale * math::atan2(q.z, q.w),
                        XZY => scale * math::atan2(-q.y, q.w),
                    }
                } else {
                    match self {
                        ZYX => math::atan2(
                            2.0 * (q.x * q.y + q.w * q.z),
                            q.w * q.w + q.x * q.x - q.y * q.y - q.z * q.z,
                        ),
                        ZXY => math::atan2(
                            -2.0 * (q.x * q.y - q.w * q.z),
                            q.w * q.w - q.x * q.x + q.y * q.y - q.z * q.z,
                        ),
                        YXZ => math::atan2(
                            2.0 * (q.x * q.z + q.w * q.y),
                            q.w * q.w - q.x * q.x - q.y * q.y + q.z * q.z,
                        ),
                        YZX => math::atan2(
                            -2.0 * (q.x * q.z - q.w * q.y),
                            q.w * q.w + q.x * q.x - q.y * q.y - q.z * q.z,
                        ),
                        XYZ => math::atan2(
                            -2.0 * (q.y * q.z - q.w * q.x),
                            q.w * q.w - q.x * q.x - q.y * q.y + q.z * q.z,
                        ),
                        XZY => math::atan2(
                            2.0 * (q.y * q.z + q.w * q.x),
                            q.w * q.w - q.x * q.x + q.y * q.y - q.z * q.z,
                        ),
                    }
                }
            }

            fn second(self, q: $quat) -> $t {
                use crate::$t::math;
                math::asin(self.sine_theta(q))
            }

            fn third(self, q: $quat) -> $t {
                use crate::$t::math;
                use EulerRot::*;
                if math::abs(self.sine_theta(q)) > 0.99999 {
                    0.0
                } else {
                    match self {
                        ZYX => math::atan2(
                            2.0 * (q.y * q.z + q.w * q.x),
                            q.w * q.w - q.x * q.x - q.y * q.y + q.z * q.z,
                        ),
                        ZXY => math::atan2(
                            -2.0 * (q.x * q.z - q.w * q.y),
                            q.w * q.w - q.x * q.x - q.y * q.y + q.z * q.z,
                        ),
                        YXZ => math::atan2(
                            2.0 * (q.x * q.y + q.w * q.z),
                            q.w * q.w - q.x * q.x + q.y * q.y - q.z * q.z,
                        ),
                        YZX => math::atan2(
                            -2.0 * (q.y * q.z - q.w * q.x),
                            q.w * q.w - q.x * q.x + q.y * q.y - q.z * q.z,
                        ),
                        XYZ => math::atan2(
                            -2.0 * (q.x * q.y - q.w * q.z),
                            q.w * q.w + q.x * q.x - q.y * q.y - q.z * q.z,
                        ),
                        XZY => math::atan2(
                            2.0 * (q.x * q.z + q.w * q.y),
                            q.w * q.w + q.x * q.x - q.y * q.y - q.z * q.z,
                        ),
                    }
                }
            }

            fn convert_quat(self, q: $quat) -> ($t, $t, $t) {
                use crate::$t::math;
                use EulerRot::*;

                let sine_theta = self.sine_theta(q);
                let second = math::asin(sine_theta);

                if math::abs(sine_theta) > 0.99999 {
                    let scale = 2.0 * math::signum(sine_theta);

                    return match self {
                        ZYX => (scale * math::atan2(-q.x, q.w), second, 0.0),
                        ZXY => (scale * math::atan2(q.y, q.w), second, 0.0),
                        YXZ => (scale * math::atan2(-q.z, q.w), second, 0.0),
                        YZX => (scale * math::atan2(q.x, q.w), second, 0.0),
                        XYZ => (scale * math::atan2(q.z, q.w), second, 0.0),
                        XZY => (scale * math::atan2(-q.y, q.w), second, 0.0),
                    };
                }

                let first = match self {
                    ZYX => math::atan2(
                        2.0 * (q.x * q.y + q.w * q.z),
                        q.w * q.w + q.x * q.x - q.y * q.y - q.z * q.z,
                    ),
                    ZXY => math::atan2(
                        -2.0 * (q.x * q.y - q.w * q.z),
                        q.w * q.w - q.x * q.x + q.y * q.y - q.z * q.z,
                    ),
                    YXZ => math::atan2(
                        2.0 * (q.x * q.z + q.w * q.y),
                        q.w * q.w - q.x * q.x - q.y * q.y + q.z * q.z,
                    ),
                    YZX => math::atan2(
                        -2.0 * (q.x * q.z - q.w * q.y),
                        q.w * q.w + q.x * q.x - q.y * q.y - q.z * q.z,
                    ),
                    XYZ => math::atan2(
                        -2.0 * (q.y * q.z - q.w * q.x),
                        q.w * q.w - q.x * q.x - q.y * q.y + q.z * q.z,
                    ),
                    XZY => math::atan2(
                        2.0 * (q.y * q.z + q.w * q.x),
                        q.w * q.w - q.x * q.x + q.y * q.y - q.z * q.z,
                    ),
                };

                let third = match self {
                    ZYX => math::atan2(
                        2.0 * (q.y * q.z + q.w * q.x),
                        q.w * q.w - q.x * q.x - q.y * q.y + q.z * q.z,
                    ),
                    ZXY => math::atan2(
                        -2.0 * (q.x * q.z - q.w * q.y),
                        q.w * q.w - q.x * q.x - q.y * q.y + q.z * q.z,
                    ),
                    YXZ => math::atan2(
                        2.0 * (q.x * q.y + q.w * q.z),
                        q.w * q.w - q.x * q.x + q.y * q.y - q.z * q.z,
                    ),
                    YZX => math::atan2(
                        -2.0 * (q.y * q.z - q.w * q.x),
                        q.w * q.w - q.x * q.x + q.y * q.y - q.z * q.z,
                    ),
                    XYZ => math::atan2(
                        -2.0 * (q.x * q.y - q.w * q.z),
                        q.w * q.w + q.x * q.x - q.y * q.y - q.z * q.z,
                    ),
                    XZY => math::atan2(
                        2.0 * (q.x * q.z + q.w * q.y),
                        q.w * q.w + q.x * q.x - q.y * q.y - q.z * q.z,
                    ),
                };

                (first, second, third)
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
