// Based on Ken Shoemake. 1994. Euler angle conversion. Graphics gems IV.  Academic Press
// Professional, Inc., USA, 222–229.
use crate::{DMat3, DMat4, DQuat, DVec3, Mat3, Mat3A, Mat4, Quat, Vec3, Vec3A, Vec3Swizzles};

/// Euler rotation sequences.
///
/// The three elemental rotations may be extrinsic (rotations about the axes xyz of the original
/// coordinate system, which is assumed to remain motionless), or intrinsic(rotations about the
/// axes of the rotating coordinate system XYZ, solidary with the moving body, which changes its
/// orientation after each elemental rotation).
///
/// ```
/// # use glam::{EulerRot, Mat3};
/// # let i = core::f32::consts::FRAC_PI_2;
/// # let j = core::f32::consts::FRAC_PI_4;
/// # let k = core::f32::consts::FRAC_PI_8;
/// let m_intrinsic = Mat3::from_rotation_x(i) * Mat3::from_rotation_y(j) * Mat3::from_rotation_z(k);
/// let n_intrinsic = Mat3::from_euler(EulerRot::XYZ, i, j, k);
/// assert!(m_intrinsic.abs_diff_eq(n_intrinsic, 2e-6));
///
/// let m_extrinsic = Mat3::from_rotation_z(k) * Mat3::from_rotation_y(j) * Mat3::from_rotation_x(i);
/// let n_extrinsic = Mat3::from_euler(EulerRot::XYZEx, i, j, k);
/// assert!(m_extrinsic.abs_diff_eq(n_extrinsic, 2e-6));
/// ```
///
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

    /// Intrinsic two-axis rotation ZYZ
    ZYZ,
    /// Intrinsic two-axis rotation ZXZ
    ZXZ,
    /// Intrinsic two-axis rotation YXY
    YXY,
    /// Intrinsic two-axis rotation YZY
    YZY,
    /// Intrinsic two-axis rotation XYX
    XYX,
    /// Intrinsic two-axis rotation XZX
    XZX,

    /// Extrinsic three-axis rotation ZYX
    ZYXEx,
    /// Extrinsic three-axis rotation ZXY
    ZXYEx,
    /// Extrinsic three-axis rotation YXZ
    YXZEx,
    /// Extrinsic three-axis rotation YZX
    YZXEx,
    /// Extrinsic three-axis rotation XYZ
    XYZEx,
    /// Extrinsic three-axis rotation XZY
    XZYEx,

    /// Extrinsic two-axis rotation ZYZ
    ZYZEx,
    /// Extrinsic two-axis rotation ZXZ
    ZXZEx,
    /// Extrinsic two-axis rotation YXY
    YXYEx,
    /// Extrinsic two-axis rotation YZY
    YZYEx,
    /// Extrinsic two-axis rotation XYX
    XYXEx,
    /// Extrinsic two-axis rotation XZX
    XZXEx,
}

impl Default for EulerRot {
    /// Default `YXZ` as yaw (y-axis), pitch (x-axis), roll (z-axis).
    fn default() -> Self {
        Self::YXZ
    }
}

pub(crate) trait ToEuler {
    type Scalar;
    fn to_euler_angles(self, order: EulerRot) -> (Self::Scalar, Self::Scalar, Self::Scalar);
}

pub(crate) trait FromEuler {
    type Scalar;
    fn from_euler_angles(
        order: EulerRot,
        i: Self::Scalar,
        j: Self::Scalar,
        k: Self::Scalar,
    ) -> Self;
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Axis {
    X = 0,
    Y = 1,
    Z = 2,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Parity {
    Odd = 0,
    Even = 1,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Repeated {
    No = 0,
    Yes = 1,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Frame {
    Relative = 0,
    Static = 1,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Order {
    initial_axis: Axis,
    parity_even: bool,
    initial_repeated: bool,
    frame_static: bool,
}

impl Order {
    const fn new(
        initial_axis: Axis,
        parity: Parity,
        initial_repeated: Repeated,
        frame: Frame,
    ) -> Self {
        Self {
            initial_axis,
            parity_even: parity as u32 == Parity::Even as u32,
            initial_repeated: initial_repeated as u32 == Repeated::Yes as u32,
            frame_static: frame as u32 == Frame::Static as u32,
        }
    }

    const fn from_euler(euler: EulerRot) -> Self {
        match euler {
            EulerRot::XYZ => Self::new(Axis::X, Parity::Even, Repeated::No, Frame::Static),
            EulerRot::XYX => Self::new(Axis::X, Parity::Even, Repeated::Yes, Frame::Static),
            EulerRot::XZY => Self::new(Axis::X, Parity::Odd, Repeated::No, Frame::Static),
            EulerRot::XZX => Self::new(Axis::X, Parity::Odd, Repeated::Yes, Frame::Static),
            EulerRot::YZX => Self::new(Axis::Y, Parity::Even, Repeated::No, Frame::Static),
            EulerRot::YZY => Self::new(Axis::Y, Parity::Even, Repeated::Yes, Frame::Static),
            EulerRot::YXZ => Self::new(Axis::Y, Parity::Odd, Repeated::No, Frame::Static),
            EulerRot::YXY => Self::new(Axis::Y, Parity::Odd, Repeated::Yes, Frame::Static),
            EulerRot::ZXY => Self::new(Axis::Z, Parity::Even, Repeated::No, Frame::Static),
            EulerRot::ZXZ => Self::new(Axis::Z, Parity::Even, Repeated::Yes, Frame::Static),
            EulerRot::ZYX => Self::new(Axis::Z, Parity::Odd, Repeated::No, Frame::Static),
            EulerRot::ZYZ => Self::new(Axis::Z, Parity::Odd, Repeated::Yes, Frame::Static),
            EulerRot::ZYXEx => Self::new(Axis::X, Parity::Even, Repeated::No, Frame::Relative),
            EulerRot::XYXEx => Self::new(Axis::X, Parity::Even, Repeated::Yes, Frame::Relative),
            EulerRot::YZXEx => Self::new(Axis::X, Parity::Odd, Repeated::No, Frame::Relative),
            EulerRot::XZXEx => Self::new(Axis::X, Parity::Odd, Repeated::Yes, Frame::Relative),
            EulerRot::XZYEx => Self::new(Axis::Y, Parity::Even, Repeated::No, Frame::Relative),
            EulerRot::YZYEx => Self::new(Axis::Y, Parity::Even, Repeated::Yes, Frame::Relative),
            EulerRot::ZXYEx => Self::new(Axis::Y, Parity::Odd, Repeated::No, Frame::Relative),
            EulerRot::YXYEx => Self::new(Axis::Y, Parity::Odd, Repeated::Yes, Frame::Relative),
            EulerRot::YXZEx => Self::new(Axis::Z, Parity::Even, Repeated::No, Frame::Relative),
            EulerRot::ZXZEx => Self::new(Axis::Z, Parity::Even, Repeated::Yes, Frame::Relative),
            EulerRot::XYZEx => Self::new(Axis::Z, Parity::Odd, Repeated::No, Frame::Relative),
            EulerRot::ZYZEx => Self::new(Axis::Z, Parity::Odd, Repeated::Yes, Frame::Relative),
        }
    }

    const fn next_axis(i: usize) -> usize {
        (i + 1) % 3
    }

    const fn prev_axis(i: usize) -> usize {
        if i > 0 {
            i - 1
        } else {
            2
        }
    }

    const fn angle_order(self) -> (usize, usize, usize) {
        let i = self.initial_axis as usize;
        let j = if self.parity_even {
            Order::next_axis(i)
        } else {
            Order::prev_axis(i)
        };
        let k = if self.parity_even {
            Order::prev_axis(i)
        } else {
            Order::next_axis(i)
        };
        (i, j, k)
    }
}

macro_rules! impl_mat3_from_euler {
    ($scalar:ident, $mat3:ident, $vec3:ident) => {
        impl FromEuler for $mat3 {
            type Scalar = $scalar;
            fn from_euler_angles(
                euler: EulerRot,
                x: Self::Scalar,
                y: Self::Scalar,
                z: Self::Scalar,
            ) -> Self {
                use crate::$scalar::math;

                let order = Order::from_euler(euler);
                let (i, j, k) = order.angle_order();

                let mut angles = if order.frame_static {
                    $vec3::new(x, y, z)
                } else {
                    $vec3::new(z, y, x)
                };

                // rotation direction is reverse from original paper
                if order.parity_even {
                    angles = -angles;
                }

                let (si, ci) = math::sin_cos(angles.x);
                let (sj, cj) = math::sin_cos(angles.y);
                let (sh, ch) = math::sin_cos(angles.z);

                let cc = ci * ch;
                let cs = ci * sh;
                let sc = si * ch;
                let ss = si * sh;

                let mut m = [[0.0; 3]; 3];

                if order.initial_repeated {
                    m[i][i] = cj;
                    m[i][j] = sj * si;
                    m[i][k] = sj * ci;
                    m[j][i] = sj * sh;
                    m[j][j] = -cj * ss + cc;
                    m[j][k] = -cj * cs - sc;
                    m[k][i] = -sj * ch;
                    m[k][j] = cj * sc + cs;
                    m[k][k] = cj * cc - ss;
                } else {
                    m[i][i] = cj * ch;
                    m[i][j] = sj * sc - cs;
                    m[i][k] = sj * cc + ss;
                    m[j][i] = cj * sh;
                    m[j][j] = sj * ss + cc;
                    m[j][k] = sj * cs - sc;
                    m[k][i] = -sj;
                    m[k][j] = cj * si;
                    m[k][k] = cj * ci;
                }

                $mat3::from_cols_array_2d(&m)
            }
        }
    };
}

macro_rules! impl_mat4_from_euler {
    ($scalar:ident, $mat4:ident, $mat3:ident) => {
        impl FromEuler for $mat4 {
            type Scalar = $scalar;
            fn from_euler_angles(
                euler: EulerRot,
                x: Self::Scalar,
                y: Self::Scalar,
                z: Self::Scalar,
            ) -> Self {
                $mat4::from_mat3($mat3::from_euler_angles(euler, x, y, z))
            }
        }
    };
}

macro_rules! impl_quat_from_euler {
    ($scalar:ident, $quat:ident, $vec3:ident) => {
        impl FromEuler for $quat {
            type Scalar = $scalar;
            fn from_euler_angles(
                euler: EulerRot,
                x: Self::Scalar,
                y: Self::Scalar,
                z: Self::Scalar,
            ) -> Self {
                use crate::$scalar::math;

                let order = Order::from_euler(euler);
                let (i, j, k) = order.angle_order();

                let mut angles = if order.frame_static {
                    $vec3::new(x, y, z)
                } else {
                    $vec3::new(z, y, x)
                };

                if order.parity_even {
                    angles.y = -angles.y;
                }

                let ti = angles.x * 0.5;
                let tj = angles.y * 0.5;
                let th = angles.z * 0.5;
                let (si, ci) = math::sin_cos(ti);
                let (sj, cj) = math::sin_cos(tj);
                let (sh, ch) = math::sin_cos(th);
                let cc = ci * ch;
                let cs = ci * sh;
                let sc = si * ch;
                let ss = si * sh;

                let parity = if !order.parity_even { 1.0 } else { -1.0 };

                let mut a = [0.0; 4];

                if order.initial_repeated {
                    a[i] = cj * (cs + sc);
                    a[j] = sj * (cc + ss) * parity;
                    a[k] = sj * (cs - sc);
                    a[3] = cj * (cc - ss);
                } else {
                    a[i] = cj * sc - sj * cs;
                    a[j] = (cj * ss + sj * cc) * parity;
                    a[k] = cj * cs - sj * sc;
                    a[3] = cj * cc + sj * ss;
                }

                $quat::from_array(a)
            }
        }
    };
}

macro_rules! impl_mat3_to_euler {
    ($scalar:ident, $mat3:ident, $vec3:ident) => {
        impl ToEuler for $mat3 {
            type Scalar = $scalar;
            fn to_euler_angles(
                self,
                euler: EulerRot,
            ) -> (Self::Scalar, Self::Scalar, Self::Scalar) {
                use crate::$scalar::math;

                let order = Order::from_euler(euler);
                let (i, j, k) = order.angle_order();

                let mut ea = $vec3::ZERO;
                if order.initial_repeated {
                    let sy = math::sqrt(
                        self.col(i)[j] * self.col(i)[j] + self.col(i)[k] * self.col(i)[k],
                    );
                    if (sy > 16.0 * $scalar::EPSILON) {
                        ea.x = math::atan2(self.col(i)[j], self.col(i)[k]);
                        ea.y = math::atan2(sy, self.col(i)[i]);
                        ea.z = math::atan2(self.col(j)[i], -self.col(k)[i]);
                    } else {
                        ea.x = math::atan2(-self.col(j)[k], self.col(j)[j]);
                        ea.y = math::atan2(sy, self.col(i)[i]);
                    }
                } else {
                    let cy = math::sqrt(
                        self.col(i)[i] * self.col(i)[i] + self.col(j)[i] * self.col(j)[i],
                    );
                    if (cy > 16.0 * $scalar::EPSILON) {
                        ea.x = math::atan2(self.col(k)[j], self.col(k)[k]);
                        ea.y = math::atan2(-self.col(k)[i], cy);
                        ea.z = math::atan2(self.col(j)[i], self.col(i)[i]);
                    } else {
                        ea.x = math::atan2(-self.col(j)[k], self.col(j)[j]);
                        ea.y = math::atan2(-self.col(k)[i], cy);
                    }
                }

                // reverse rotation angle of original code
                if order.parity_even {
                    ea = -ea;
                }

                if !order.frame_static {
                    ea = ea.zyx();
                }

                (ea.x, ea.y, ea.z)
            }
        }
    };
}

macro_rules! impl_mat4_to_euler {
    ($scalar:ident, $mat4:ident, $mat3:ident) => {
        impl ToEuler for $mat4 {
            type Scalar = $scalar;
            fn to_euler_angles(
                self,
                order: EulerRot,
            ) -> (Self::Scalar, Self::Scalar, Self::Scalar) {
                $mat3::from_mat4(self).to_euler_angles(order)
            }
        }
    };
}

macro_rules! impl_quat_to_euler {
    ($scalar:ident, $quat:ident, $mat3:ident) => {
        impl ToEuler for $quat {
            type Scalar = $scalar;
            fn to_euler_angles(
                self,
                order: EulerRot,
            ) -> (Self::Scalar, Self::Scalar, Self::Scalar) {
                $mat3::from_quat(self).to_euler_angles(order)
            }
        }
    };
}

impl_mat3_to_euler!(f32, Mat3, Vec3);
impl_mat3_from_euler!(f32, Mat3, Vec3);
impl_mat3_to_euler!(f32, Mat3A, Vec3A);
impl_mat3_from_euler!(f32, Mat3A, Vec3A);
impl_mat4_from_euler!(f32, Mat4, Mat3);
impl_mat4_to_euler!(f32, Mat4, Mat3);
impl_quat_to_euler!(f32, Quat, Mat3);
impl_quat_from_euler!(f32, Quat, Vec3);

impl_mat3_to_euler!(f64, DMat3, DVec3);
impl_mat3_from_euler!(f64, DMat3, DVec3);
impl_mat4_to_euler!(f64, DMat4, DMat3);
impl_mat4_from_euler!(f64, DMat4, DMat3);
impl_quat_to_euler!(f64, DQuat, DMat3);
impl_quat_from_euler!(f64, DQuat, DVec3);
