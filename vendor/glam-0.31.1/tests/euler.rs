#[macro_use]
mod support;

mod euler {
    use glam::*;
    use std::ops::RangeInclusive;

    /// Helper to get the 'canonical' version of a `Quat`. We define the canonical of quat `q` as:
    ///
    /// * `q`, if q.w > epsilon
    /// * `-q`, if q.w < -epsilon
    /// * `(0, 0, 0, 1)` otherwise
    ///
    /// The rationale is that q and -q represent the same rotation, and any (_, _, _, 0) represent no rotation at all.
    trait CanonicalQuat: Copy {
        fn canonical(self) -> Self;
    }

    /// Helper to set some alternative epsilons based on the floating point type used
    trait EulerEpsilon {
        /// epsilon for comparing quaternion round-tripped through eulers (quat -> euler -> quat)
        const E_EPS: f32;
    }

    impl EulerEpsilon for f32 {
        const E_EPS: f32 = 2e-6;
    }

    impl EulerEpsilon for f64 {
        const E_EPS: f32 = 1e-8;
    }

    fn axis_order(order: EulerRot) -> (usize, usize, usize) {
        match order {
            EulerRot::XYZ => (0, 1, 2),
            EulerRot::XYX => (0, 1, 0),
            EulerRot::XZY => (0, 2, 1),
            EulerRot::XZX => (0, 2, 0),
            EulerRot::YZX => (1, 2, 0),
            EulerRot::YZY => (1, 2, 1),
            EulerRot::YXZ => (1, 0, 2),
            EulerRot::YXY => (1, 0, 1),
            EulerRot::ZXY => (2, 0, 1),
            EulerRot::ZXZ => (2, 0, 2),
            EulerRot::ZYX => (2, 1, 0),
            EulerRot::ZYZ => (2, 1, 2),
            EulerRot::ZYXEx => (2, 1, 0),
            EulerRot::XYXEx => (0, 1, 0),
            EulerRot::YZXEx => (1, 2, 0),
            EulerRot::XZXEx => (0, 2, 0),
            EulerRot::XZYEx => (0, 2, 1),
            EulerRot::YZYEx => (1, 2, 1),
            EulerRot::ZXYEx => (2, 0, 1),
            EulerRot::YXYEx => (1, 0, 1),
            EulerRot::YXZEx => (1, 0, 2),
            EulerRot::ZXZEx => (2, 0, 2),
            EulerRot::XYZEx => (0, 1, 2),
            EulerRot::ZYZEx => (2, 1, 2),
        }
    }

    fn is_intrinsic(order: EulerRot) -> bool {
        match order {
            EulerRot::XYZ
            | EulerRot::XYX
            | EulerRot::XZY
            | EulerRot::XZX
            | EulerRot::YZX
            | EulerRot::YZY
            | EulerRot::YXZ
            | EulerRot::YXY
            | EulerRot::ZXY
            | EulerRot::ZXZ
            | EulerRot::ZYX
            | EulerRot::ZYZ => true,
            EulerRot::ZYXEx
            | EulerRot::XYXEx
            | EulerRot::YZXEx
            | EulerRot::XZXEx
            | EulerRot::XZYEx
            | EulerRot::YZYEx
            | EulerRot::ZXYEx
            | EulerRot::YXYEx
            | EulerRot::YXZEx
            | EulerRot::ZXZEx
            | EulerRot::XYZEx
            | EulerRot::ZYZEx => false,
        }
    }

    mod f32 {
        pub fn deg_to_rad(a: i32, b: i32, c: i32) -> (f32, f32, f32) {
            (
                (a as f32).to_radians(),
                (b as f32).to_radians(),
                (c as f32).to_radians(),
            )
        }
    }

    mod f64 {
        pub fn deg_to_rad(a: i32, b: i32, c: i32) -> (f64, f64, f64) {
            (
                (a as f64).to_radians(),
                (b as f64).to_radians(),
                (c as f64).to_radians(),
            )
        }
    }

    fn test_order_angles<F: Fn(EulerRot, i32, i32, i32)>(order: EulerRot, test: &F) {
        const RANGE: RangeInclusive<i32> = -180..=180;
        const STEP: usize = 15;
        for i in RANGE.step_by(STEP) {
            for j in RANGE.step_by(STEP) {
                for k in RANGE.step_by(STEP) {
                    test(order, i, j, k);
                }
            }
        }
    }

    fn test_all_orders<F: Fn(EulerRot)>(test: &F) {
        test(EulerRot::XYZ);
        test(EulerRot::XZY);
        test(EulerRot::YZX);
        test(EulerRot::YXZ);
        test(EulerRot::ZXY);
        test(EulerRot::ZYX);

        test(EulerRot::XZX);
        test(EulerRot::XYX);
        test(EulerRot::YXY);
        test(EulerRot::YZY);
        test(EulerRot::ZYZ);
        test(EulerRot::ZXZ);

        test(EulerRot::XYZEx);
        test(EulerRot::XZYEx);
        test(EulerRot::YZXEx);
        test(EulerRot::YXZEx);
        test(EulerRot::ZXYEx);
        test(EulerRot::ZYXEx);

        test(EulerRot::XZXEx);
        test(EulerRot::XYXEx);
        test(EulerRot::YXYEx);
        test(EulerRot::YZYEx);
        test(EulerRot::ZYZEx);
        test(EulerRot::ZXZEx);
    }

    macro_rules! impl_quat_euler_test {
        ($quat:ident, $t:ident) => {
            use super::{
                axis_order, is_intrinsic, test_all_orders, test_order_angles, $t::deg_to_rad,
                CanonicalQuat, EulerEpsilon,
            };
            use glam::{$quat, EulerRot};

            const AXIS_ANGLE: [fn($t) -> $quat; 3] = [
                $quat::from_rotation_x,
                $quat::from_rotation_y,
                $quat::from_rotation_z,
            ];

            impl CanonicalQuat for $quat {
                fn canonical(self) -> Self {
                    match self {
                        _ if self.w >= 1e-5 => self,
                        _ if self.w <= -1e-5 => -self,
                        _ => $quat::from_xyzw(0.0, 0.0, 0.0, 1.0),
                    }
                }
            }

            fn test_euler(order: EulerRot, a: i32, b: i32, c: i32) {
                println!(
                    "test_euler: {} {order:?} ({a}, {b}, {c})",
                    stringify!($quat)
                );

                let (a, b, c) = deg_to_rad(a, b, c);
                let m = $quat::from_euler(order, a, b, c);

                let n = {
                    let (i, j, k) = m.to_euler(order);
                    $quat::from_euler(order, i, j, k)
                };
                assert_approx_eq!(m.canonical(), n.canonical(), $t::E_EPS);

                let o = {
                    let (i, j, k) = axis_order(order);
                    if is_intrinsic(order) {
                        AXIS_ANGLE[i](a) * AXIS_ANGLE[j](b) * AXIS_ANGLE[k](c)
                    } else {
                        AXIS_ANGLE[k](c) * AXIS_ANGLE[j](b) * AXIS_ANGLE[i](a)
                    }
                };
                assert_approx_eq!(m.canonical(), o.canonical(), $t::E_EPS);
            }

            #[test]
            fn test_all_euler_orders() {
                let test = |order| test_order_angles(order, &test_euler);
                test_all_orders(&test);
            }
        };
    }

    macro_rules! impl_mat_euler_test {
        ($mat:ident, $t:ident) => {
            use super::{
                axis_order, is_intrinsic, test_all_orders, test_order_angles, $t::deg_to_rad,
                EulerEpsilon,
            };
            use glam::{$mat, EulerRot};

            const AXIS_ANGLE: [fn($t) -> $mat; 3] = [
                $mat::from_rotation_x,
                $mat::from_rotation_y,
                $mat::from_rotation_z,
            ];

            fn test_euler(order: EulerRot, a: i32, b: i32, c: i32) {
                println!("test_euler: {} {order:?} ({a}, {b}, {c})", stringify!($mat));

                let (a, b, c) = deg_to_rad(a, b, c);
                let m = $mat::from_euler(order, a, b, c);
                let n = {
                    let (i, j, k) = m.to_euler(order);
                    $mat::from_euler(order, i, j, k)
                };
                assert_approx_eq!(m, n, $t::E_EPS);

                let o = {
                    let (i, j, k) = axis_order(order);
                    if is_intrinsic(order) {
                        AXIS_ANGLE[i](a) * AXIS_ANGLE[j](b) * AXIS_ANGLE[k](c)
                    } else {
                        AXIS_ANGLE[k](c) * AXIS_ANGLE[j](b) * AXIS_ANGLE[i](a)
                    }
                };
                assert_approx_eq!(m, o, $t::E_EPS);
            }

            #[test]
            fn test_all_euler_orders() {
                let test = |order| test_order_angles(order, &test_euler);
                test_all_orders(&test);
            }
        };
    }

    #[test]
    fn test_euler_default() {
        assert_eq!(EulerRot::YXZ, EulerRot::default());
    }

    mod quat {
        impl_quat_euler_test!(Quat, f32);
    }

    mod mat3 {
        impl_mat_euler_test!(Mat3, f32);
    }

    mod mat3a {
        impl_mat_euler_test!(Mat3A, f32);
    }

    mod mat4 {
        impl_mat_euler_test!(Mat4, f32);
    }

    mod dquat {
        impl_quat_euler_test!(DQuat, f64);
    }

    mod dmat3 {
        impl_mat_euler_test!(DMat3, f64);
    }

    mod dmat4 {
        impl_mat_euler_test!(DMat4, f64);
    }
}
