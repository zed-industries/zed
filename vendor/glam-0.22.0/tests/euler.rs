#[macro_use]
mod support;

/// Helper to calculate the inner angle in the range [0, 2*PI)
trait AngleDiff {
    type Output;
    fn angle_diff(self, other: Self) -> Self::Output;
}

macro_rules! impl_angle_diff {
    ($t:ty, $pi:expr) => {
        impl AngleDiff for $t {
            type Output = $t;
            fn angle_diff(self, other: $t) -> $t {
                const PI2: $t = $pi + $pi;
                let s = self.rem_euclid(PI2);
                let o = other.rem_euclid(PI2);
                if s > o {
                    (s - o).min(PI2 + o - s)
                } else {
                    (o - s).min(PI2 + s - o)
                }
            }
        }
    };
}
impl_angle_diff!(f32, std::f32::consts::PI);
impl_angle_diff!(f64, std::f64::consts::PI);

macro_rules! assert_approx_angle {
    ($a:expr, $b:expr, $eps:expr) => {{
        let (a, b) = ($a, $b);
        let eps = $eps;
        let diff = a.angle_diff(b);
        assert!(
            diff < $eps,
            "assertion failed: `(left !== right)` \
             (left: `{:?}`, right: `{:?}`, expect diff: `{:?}`, real diff: `{:?}`)",
            a,
            b,
            eps,
            diff
        );
    }};
}

macro_rules! impl_3axis_test {
    ($name:ident, $t:ty, $quat:ident, $euler:path, $U:path, $V:path, $W:path, $vec:ident) => {
        glam_test!($name, {
            let euler = $euler;
            assert!($U != $W); // First and last axis must be different for three axis
            for u in (-176..=176).step_by(44) {
                for v in (-88..=88).step_by(44) {
                    for w in (-176..=176).step_by(44) {
                        let u1 = (u as $t).to_radians();
                        let v1 = (v as $t).to_radians();
                        let w1 = (w as $t).to_radians();

                        let q1: $quat = ($quat::from_axis_angle($U, u1)
                            * $quat::from_axis_angle($V, v1)
                            * $quat::from_axis_angle($W, w1))
                        .normalize();

                        // Test if the rotation is the expected
                        let q2: $quat = $quat::from_euler(euler, u1, v1, w1).normalize();
                        assert_approx_eq!(q1, q2, 1e-5);

                        // Test angle reconstruction
                        let (u2, v2, w2) = q1.to_euler(euler);
                        let q3 = $quat::from_euler(euler, u2, v2, w2).normalize();

                        assert_approx_angle!(u1, u2, 1e-4 as $t);
                        assert_approx_angle!(v1, v2, 1e-4 as $t);
                        assert_approx_angle!(w1, w2, 1e-4 as $t);

                        assert_approx_eq!(q1 * $vec::X, q3 * $vec::X, 1e-4);
                        assert_approx_eq!(q1 * $vec::Y, q3 * $vec::Y, 1e-4);
                        assert_approx_eq!(q1 * $vec::Z, q3 * $vec::Z, 1e-4);
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
    use super::AngleDiff;
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
