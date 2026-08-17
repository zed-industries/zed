#[macro_use]
mod support;

/// Helper to get the 'canonical' version of a `Quat`. We define the canonical of quat `q` as:
/// * `q`, if q.w > epsilon
/// * `-q`, if q.w < -epsilon
/// * `(0, 0, 0, 1)` otherwise
/// The rationale is that q and -q represent the same rotation, and any (_, _, _, 0) represent no rotation at all.
trait CanonicalQuat: Copy {
    fn canonical(self) -> Self;
}

macro_rules! impl_canonical_quat {
    ($t:ty) => {
        impl CanonicalQuat for $t {
            fn canonical(self) -> Self {
                match self {
                    _ if self.w >= 1e-5 => self,
                    _ if self.w <= -1e-5 => -self,
                    _ => <$t>::from_xyzw(0.0, 0.0, 0.0, 1.0),
                }
            }
        }
    };
}

impl_canonical_quat!(glam::Quat);
impl_canonical_quat!(glam::DQuat);

/// Helper to set some alternative epsilons based on the floating point type used
trait EulerEpsilon {
    /// epsilon for comparing quaterions built from eulers and axis-angles
    const Q_EPS: f32;

    /// epsilon for comparing quaternion round-tripped through eulers (quat -> euler -> quat)
    const E_EPS: f32;
}
impl EulerEpsilon for f32 {
    const Q_EPS: f32 = 1e-5;
    // TODO: sse2 impl passes with 1e-5 but scalar-math does not, why?
    const E_EPS: f32 = 2e-4;
}

impl EulerEpsilon for f64 {
    const Q_EPS: f32 = 1e-8;
    const E_EPS: f32 = 1e-8;
}

macro_rules! impl_3axis_test {
    ($name:ident, $t:ty, $quat:ident, $euler:path, $U:path, $V:path, $W:path, $vec:ident) => {
        glam_test!($name, {
            let euler = $euler;
            assert!($U != $W); // First and last axis must be different for three axis
            for u in (-180..=180).step_by(15) {
                for v in (-180..=180).step_by(15) {
                    for w in (-180..=180).step_by(15) {
                        let u1 = (u as $t).to_radians();
                        let v1 = (v as $t).to_radians();
                        let w1 = (w as $t).to_radians();

                        let q1: $quat = ($quat::from_axis_angle($U, u1)
                            * $quat::from_axis_angle($V, v1)
                            * $quat::from_axis_angle($W, w1))
                        .normalize();

                        // Test if the rotation is the expected
                        let q2: $quat = $quat::from_euler(euler, u1, v1, w1).normalize();
                        assert_approx_eq!(q1.canonical(), q2.canonical(), <$t>::Q_EPS);

                        // Test quat reconstruction from angles
                        let (u2, v2, w2) = q2.to_euler(euler);
                        let q3 = $quat::from_euler(euler, u2, v2, w2).normalize();
                        assert_approx_eq!(
                            q2.canonical(),
                            q3.canonical(),
                            <$t>::E_EPS,
                            format!(
                                "angles {:?} -> {:?}",
                                (u, v, w),
                                (u2.to_degrees(), v2.to_degrees(), w2.to_degrees())
                            )
                        );
                    }
                }
            }
        });
    };
}

macro_rules! impl_all_quat_tests_three_axis {
    ($t:ty, $q:ident, $v:ident) => {
        impl_3axis_test!(test_euler_zyx, $t, $q, ER::ZYX, $v::Z, $v::Y, $v::X, $v);
        impl_3axis_test!(test_euler_zxy, $t, $q, ER::ZXY, $v::Z, $v::X, $v::Y, $v);
        impl_3axis_test!(test_euler_yxz, $t, $q, ER::YXZ, $v::Y, $v::X, $v::Z, $v);
        impl_3axis_test!(test_euler_yzx, $t, $q, ER::YZX, $v::Y, $v::Z, $v::X, $v);
        impl_3axis_test!(test_euler_xyz, $t, $q, ER::XYZ, $v::X, $v::Y, $v::Z, $v);
        impl_3axis_test!(test_euler_xzy, $t, $q, ER::XZY, $v::X, $v::Z, $v::Y, $v);
    };
}

mod euler {
    use super::{CanonicalQuat, EulerEpsilon};
    use glam::*;
    type ER = EulerRot;

    mod quat {
        use super::*;

        impl_all_quat_tests_three_axis!(f32, Quat, Vec3);
    }

    mod dquat {
        use super::*;

        impl_all_quat_tests_three_axis!(f64, DQuat, DVec3);
    }
}
