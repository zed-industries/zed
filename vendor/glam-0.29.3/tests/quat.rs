#![allow(clippy::excessive_precision)]

#[macro_use]
mod support;

macro_rules! impl_quat_tests {
    ($t:ident, $new:ident, $mat3:ident, $mat4:ident, $quat:ident, $vec2:ident, $vec3:ident, $vec4:ident) => {
        glam_test!(test_const, {
            const Q0: $quat = $quat::from_xyzw(1.0, 2.0, 3.0, 4.0);
            const Q1: $quat = $quat::from_array([1.0, 2.0, 3.0, 4.0]);
            assert_eq!([1.0, 2.0, 3.0, 4.0], *Q0.as_ref());
            assert_eq!([1.0, 2.0, 3.0, 4.0], *Q1.as_ref());
        });

        glam_test!(test_nan, {
            assert!($quat::NAN.is_nan());
            assert!(!$quat::NAN.is_finite());
        });

        glam_test!(test_new, {
            let ytheta = deg(45.0);
            let q0 = $quat::from_rotation_y(ytheta);

            let v1 = $vec4::new(0.0, (ytheta * 0.5).sin(), 0.0, (ytheta * 0.5).cos());
            assert_eq!(q0, $quat::from_vec4(v1));
            let q1 = $quat::from_vec4(v1);
            assert_eq!(v1, q1.into());

            assert_eq!(q0, $new(v1.x, v1.y, v1.z, v1.w));

            let a1: [$t; 4] = q1.into();
            assert_eq!([v1.x, v1.y, v1.z, v1.w], a1);

            assert_eq!(q1, $quat::from_array(a1));

            assert_eq!(a1, *q0.as_ref());
        });

        glam_test!(test_funcs, {
            let q0 = $quat::from_euler(EulerRot::YXZ, deg(45.0), deg(180.0), deg(90.0));
            assert!(q0.is_normalized());
            assert_approx_eq!(q0.length_squared(), 1.0);
            assert_approx_eq!(q0.length(), 1.0);
            assert_approx_eq!(q0.length_recip(), 1.0);
            assert_approx_eq!(q0, q0.normalize());

            assert_approx_eq!(q0.dot(q0), 1.0);
            assert_approx_eq!(q0.dot(q0), 1.0);

            let q1 = $quat::from_vec4($vec4::from(q0) * 2.0);
            assert!(!q1.is_normalized());
            assert_approx_eq!(q1.length_squared(), 4.0, 1.0e-6);
            assert_approx_eq!(q1.length(), 2.0);
            assert_approx_eq!(q1.length_recip(), 0.5);
            assert_approx_eq!(q0, q1.normalize());
            assert_approx_eq!(q0.dot(q1), 2.0, 1.0e-6);

            should_glam_assert!({ ($quat::IDENTITY * 0.0).normalize() });
        });

        glam_test!(test_rotation, {
            let zero = deg(0.0);
            let yaw = deg(30.0);
            let pitch = deg(60.0);
            let roll = deg(90.0);
            let y0 = $quat::from_rotation_y(yaw);
            assert!(y0.is_normalized());
            let (axis, angle) = y0.to_axis_angle();
            assert_approx_eq!(axis, $vec3::Y, 1.0e-6);
            assert_approx_eq!(angle, yaw);
            let y1 = $quat::from_euler(EulerRot::YXZ, yaw, zero, zero);
            assert_approx_eq!(y0, y1);
            let y2 = $quat::from_axis_angle($vec3::Y, yaw);
            assert_approx_eq!(y0, y2);
            let y3 = $quat::from_mat3(&$mat3::from_rotation_y(yaw));
            assert_approx_eq!(y0, y3);
            let y4 = $quat::from_mat3(&$mat3::from_quat(y0));
            assert_approx_eq!(y0, y4);

            let x0 = $quat::from_rotation_x(pitch);
            assert!(x0.is_normalized());
            let (axis, angle) = x0.to_axis_angle();
            assert_approx_eq!(axis, $vec3::X);
            assert_approx_eq!(angle, pitch, 1e-6);
            let x1 = $quat::from_euler(EulerRot::YXZ, zero, pitch, zero);
            assert_approx_eq!(x0, x1);
            let x2 = $quat::from_axis_angle($vec3::X, pitch);
            assert_approx_eq!(x0, x2);
            let x3 = $quat::from_mat4(&$mat4::from_rotation_x(deg(180.0)));
            assert!(x3.is_normalized());
            assert_approx_eq!($quat::from_rotation_x(deg(180.0)), x3);

            let z0 = $quat::from_rotation_z(roll);
            assert!(z0.is_normalized());
            let (axis, angle) = z0.to_axis_angle();
            assert_approx_eq!(axis, $vec3::Z);
            assert_approx_eq!(angle, roll);
            let z1 = $quat::from_euler(EulerRot::YXZ, zero, zero, roll);
            assert_approx_eq!(z0, z1);
            let z2 = $quat::from_axis_angle($vec3::Z, roll);
            assert_approx_eq!(z0, z2);
            let z3 = $quat::from_mat4(&$mat4::from_rotation_z(roll));
            assert_approx_eq!(z0, z3);

            let yx0 = y0 * x0;
            assert!(yx0.is_normalized());
            let yx1 = $quat::from_euler(EulerRot::YXZ, yaw, pitch, zero);
            assert_approx_eq!(yx0, yx1);

            let yxz0 = y0 * x0 * z0;
            assert!(yxz0.is_normalized());
            let yxz1 = $quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
            assert_approx_eq!(yxz0, yxz1);

            // use the conjugate of z0 to remove the rotation from yxz0
            let yx2 = yxz0 * z0.conjugate();
            assert_approx_eq!(yx0, yx2);
            assert!((yxz0 * yxz0.conjugate()).is_near_identity());

            // test inverse does the same
            let yx2 = yxz0 * z0.inverse();
            assert_approx_eq!(yx0, yx2);
            assert!((yxz0 * yxz0.inverse()).is_near_identity());

            let yxz2 = $quat::from_mat4(&$mat4::from_quat(yxz0));
            assert_approx_eq!(yxz0, yxz2);

            // if near identity, just returns x axis and 0 rotation
            let (axis, angle) = $quat::IDENTITY.to_axis_angle();
            assert_eq!(axis, $vec3::X);
            assert_eq!(angle, rad(0.0));

            let mut x0 = $quat::from_rotation_x(pitch);
            x0 *= x0;
            assert_approx_eq!(x0, $quat::from_rotation_x(pitch * 2.0));

            should_glam_assert!({ ($quat::IDENTITY * 2.0).inverse() });
            should_glam_assert!({ $quat::from_axis_angle($vec3::ZERO, 0.0) });
        });

        glam_test!(test_from_scaled_axis, {
            assert_eq!($quat::from_scaled_axis($vec3::ZERO), $quat::IDENTITY);
            assert_eq!(
                $quat::from_scaled_axis($vec3::Y * 1e-10),
                $quat::from_axis_angle($vec3::Y, 1e-10)
            );
            assert_eq!(
                $quat::from_scaled_axis($vec3::X * 1.0),
                $quat::from_axis_angle($vec3::X, 1.0)
            );
            assert_eq!(
                $quat::from_scaled_axis($vec3::Z * 2.0),
                $quat::from_axis_angle($vec3::Z, 2.0)
            );

            assert_eq!(
                $quat::from_scaled_axis($vec3::ZERO).to_scaled_axis(),
                $vec3::ZERO
            );
            for &v in &vec3_float_test_vectors!($vec3) {
                if v.length() < core::$t::consts::PI {
                    assert!(($quat::from_scaled_axis(v).to_scaled_axis() - v).length() < 1e-6,);
                }
            }
        });

        glam_test!(test_mul_vec3, {
            let qrz = $quat::from_rotation_z(deg(90.0));
            assert_approx_eq!($vec3::Y, qrz * $vec3::X);
            assert_approx_eq!($vec3::Y, qrz.mul_vec3($vec3::X));
            assert_approx_eq!($vec3::Y, -qrz * $vec3::X);
            assert_approx_eq!($vec3::Y, qrz.neg().mul_vec3($vec3::X));
            assert_approx_eq!(-$vec3::X, qrz * $vec3::Y);
            assert_approx_eq!(-$vec3::X, qrz.mul_vec3($vec3::Y));
            assert_approx_eq!(-$vec3::X, -qrz * $vec3::Y);
            assert_approx_eq!(-$vec3::X, qrz.neg().mul_vec3($vec3::Y));

            // check vec3 * mat3 is the same
            let mrz = $mat3::from_quat(qrz);
            assert_approx_eq!($vec3::Y, mrz * $vec3::X);
            assert_approx_eq!($vec3::Y, mrz.mul_vec3($vec3::X));
            // assert_approx_eq!($vec3::Y, -mrz * $vec3::X);
            assert_approx_eq!(-$vec3::X, mrz * $vec3::Y);
            assert_approx_eq!(-$vec3::X, mrz.mul_vec3($vec3::Y));

            let qrx = $quat::from_rotation_x(deg(90.0));
            assert_approx_eq!($vec3::X, qrx * $vec3::X);
            assert_approx_eq!($vec3::X, qrx.mul_vec3($vec3::X));
            assert_approx_eq!($vec3::X, -qrx * $vec3::X);
            assert_approx_eq!($vec3::X, qrx.neg().mul_vec3($vec3::X));
            assert_approx_eq!($vec3::Z, qrx * $vec3::Y);
            assert_approx_eq!($vec3::Z, qrx.mul_vec3($vec3::Y));
            assert_approx_eq!($vec3::Z, -qrx * $vec3::Y);
            assert_approx_eq!($vec3::Z, qrx.neg().mul_vec3($vec3::Y));

            // check vec3 * mat3 is the same
            let mrx = $mat3::from_quat(qrx);
            assert_approx_eq!($vec3::X, mrx * $vec3::X);
            assert_approx_eq!($vec3::X, mrx.mul_vec3($vec3::X));
            assert_approx_eq!($vec3::Z, mrx * $vec3::Y);
            assert_approx_eq!($vec3::Z, mrx.mul_vec3($vec3::Y));

            let qrxz = qrz * qrx;
            assert_approx_eq!($vec3::Y, qrxz * $vec3::X);
            assert_approx_eq!($vec3::Y, qrxz.mul_vec3($vec3::X));
            assert_approx_eq!($vec3::Z, qrxz * $vec3::Y);
            assert_approx_eq!($vec3::Z, qrxz.mul_vec3($vec3::Y));

            let mrxz = mrz * mrx;
            assert_approx_eq!($vec3::Y, mrxz * $vec3::X);
            assert_approx_eq!($vec3::Y, mrxz.mul_vec3($vec3::X));
            assert_approx_eq!($vec3::Z, mrxz * $vec3::Y);
            assert_approx_eq!($vec3::Z, mrxz.mul_vec3($vec3::Y));

            let qrzx = qrx * qrz;
            assert_approx_eq!($vec3::Z, qrzx * $vec3::X);
            assert_approx_eq!($vec3::Z, qrzx.mul_vec3($vec3::X));
            assert_approx_eq!(-$vec3::X, qrzx * $vec3::Y);
            assert_approx_eq!(-$vec3::X, qrzx.mul_vec3($vec3::Y));

            let mrzx = qrx * qrz;
            assert_approx_eq!($vec3::Z, mrzx * $vec3::X);
            assert_approx_eq!($vec3::Z, mrzx.mul_vec3($vec3::X));
            assert_approx_eq!(-$vec3::X, mrzx * $vec3::Y);
            assert_approx_eq!(-$vec3::X, mrzx.mul_vec3($vec3::Y));

            should_glam_assert!({ ($quat::IDENTITY * 0.5).mul_vec3($vec3::X) });
            should_glam_assert!({ ($quat::IDENTITY * 0.5) * $vec3::X });
        });

        glam_test!(test_angle_between, {
            const TAU: $t = 2.0 * core::$t::consts::PI;
            let eps = 10.0 * $t::EPSILON as f32;
            let q1 = $quat::from_euler(EulerRot::YXZ, 0.0, 0.0, 0.0);
            let q2 = $quat::from_euler(EulerRot::YXZ, TAU * 0.25, 0.0, 0.0);
            let q3 = $quat::from_euler(EulerRot::YXZ, TAU * 0.5, 0.0, 0.0);
            let q4 = $quat::from_euler(EulerRot::YXZ, 0.0, 0.0, TAU * 0.25);
            let q5 = $quat::from_axis_angle($vec3::new(1.0, 2.0, 3.0).normalize(), TAU * 0.3718);
            let q6 = $quat::from_axis_angle($vec3::new(-1.0, 5.0, 3.0).normalize(), TAU * 0.94);
            assert_approx_eq!(q1.angle_between(q2), TAU * 0.25, eps);
            assert_approx_eq!(q1.angle_between(q3), TAU * 0.5, eps);
            assert_approx_eq!(q3.angle_between(q3), 0.0, eps);
            assert_approx_eq!(q3.angle_between(-q3), 0.0, eps);
            assert_approx_eq!((q4 * q2 * q2).angle_between(q4 * q2), TAU * 0.25, eps);
            assert_approx_eq!(q1.angle_between(q5), TAU * 0.3718, eps);
            assert_approx_eq!(
                (q5 * q2 * q1).angle_between(q5 * q2 * q5),
                TAU * 0.3718,
                eps
            );
            assert_approx_eq!((q3 * q3).angle_between(q1), 0.0, eps);
            assert_approx_eq!((q3 * q3 * q3).angle_between(q3), 0.0, eps);
            assert_approx_eq!((q3 * q3 * q3 * q3).angle_between(q1), 0.0, eps);
            assert_approx_eq!(q1.angle_between(q6), TAU - TAU * 0.94, eps);
            assert_approx_eq!((q5 * q1).angle_between(q5 * q6), TAU - TAU * 0.94, eps);
            assert_approx_eq!((q1 * q5).angle_between(q6 * q5), TAU - TAU * 0.94, eps);
        });

        glam_test!(test_lerp, {
            let q0 = $quat::from_rotation_y(deg(0.0));
            assert_approx_eq!(q0, q0.slerp(q0, 0.0));
            assert_approx_eq!(q0, q0.slerp(q0, 1.0));

            let q1 = $quat::from_rotation_y(deg(90.0));
            assert_approx_eq!(q0, q0.lerp(q1, 0.0));
            assert_approx_eq!(q1, q0.lerp(q1, 1.0));
            assert_approx_eq!($quat::from_rotation_y(deg(45.0)), q0.lerp(q1, 0.5));

            should_glam_assert!({ $quat::lerp($quat::IDENTITY * 2.0, $quat::IDENTITY, 1.0) });
            should_glam_assert!({ $quat::lerp($quat::IDENTITY, $quat::IDENTITY * 0.5, 1.0) });
        });

        glam_test!(test_slerp, {
            let q0 = $quat::from_rotation_y(deg(0.0));
            assert_approx_eq!(q0, q0.slerp(q0, 0.0));
            assert_approx_eq!(q0, q0.slerp(q0, 1.0));

            let q1 = $quat::from_rotation_y(deg(90.0));
            assert_approx_eq!(q0, q0.slerp(q1, 0.0), 1.0e-3);
            assert_approx_eq!(q1, q0.slerp(q1, 1.0), 1.0e-3);
            assert_approx_eq!($quat::from_rotation_y(deg(45.0)), q0.slerp(q1, 0.5), 1.0e-3);

            should_glam_assert!({ $quat::slerp($quat::IDENTITY * 2.0, $quat::IDENTITY, 1.0) });
            should_glam_assert!({ $quat::slerp($quat::IDENTITY, $quat::IDENTITY * 0.5, 1.0) });
        });

        glam_test!(test_slerp_constant_speed, {
            let step = 0.01;
            let mut s = 0.0;
            while s <= 1.0 {
                let q0 = $quat::from_rotation_y(deg(0.0));
                let q1 = $quat::from_rotation_y(deg(90.0));
                let result = q0.slerp(q1, s);
                assert_approx_eq!($quat::from_rotation_y(deg(s * 90.0)), result, 1.0e-3);
                assert!(result.is_normalized());
                s += step;
            }
        });

        glam_test!(test_slerp_tau, {
            let q1 = $quat::IDENTITY;
            let q2 = $quat::from_rotation_x(core::$t::consts::TAU);
            let s = q1.slerp(q2, 1.);
            assert!(s.is_finite());
            assert!(s.is_normalized());
        });

        glam_test!(test_slerp_negative_tau, {
            let q1 = $quat::IDENTITY;
            let q2 = $quat::from_rotation_x(-core::$t::consts::TAU);
            let s = q1.slerp(q2, 1.);
            assert!(s.is_finite());
            assert!(s.is_normalized());
        });

        glam_test!(test_slerp_pi, {
            let q1 = $quat::IDENTITY;
            let q2 = $quat::from_rotation_x(core::$t::consts::PI);
            let s = q1.slerp(q2, 1.);
            assert!(s.is_finite());
            assert!(s.is_normalized());
        });

        glam_test!(test_slerp_negative_pi, {
            let q1 = $quat::IDENTITY;
            let q2 = $quat::from_rotation_x(-core::$t::consts::PI);
            let s = q1.slerp(q2, 1.);
            assert!(s.is_finite());
            assert!(s.is_normalized());
        });

        glam_test!(test_rotate_towards, {
            use core::$t::consts::{FRAC_PI_2, FRAC_PI_4};
            let eps = 10.0 * $t::EPSILON as f32;

            // Setup such that `q0` is `PI/2` and `-PI/2` radians away from `q1` and `q2` respectively.
            let q0 = $quat::from_euler(EulerRot::YXZ, 0.0, 0.0, 0.0);
            let q1 = $quat::from_euler(EulerRot::YXZ, FRAC_PI_2, 0.0, 0.0);
            let q2 = $quat::from_euler(EulerRot::YXZ, -FRAC_PI_2, 0.0, 0.0);

            // Positive delta
            assert_approx_eq!(q0, q0.rotate_towards(q1, 0.0), eps);
            assert_approx_eq!(
                $quat::from_euler(EulerRot::YXZ, FRAC_PI_4, 0.0, 0.0),
                q0.rotate_towards(q1, FRAC_PI_4),
                eps
            );
            assert_approx_eq!(q1, q0.rotate_towards(q1, FRAC_PI_2), eps);
            assert_approx_eq!(q1, q0.rotate_towards(q1, FRAC_PI_2 * 1.5), eps);

            // Negative delta
            assert_approx_eq!(
                $quat::from_euler(EulerRot::YXZ, -FRAC_PI_4, 0.0, 0.0),
                q0.rotate_towards(q1, -FRAC_PI_4),
                eps
            );
            assert_approx_eq!(q2, q0.rotate_towards(q1, -FRAC_PI_2), eps);
            assert_approx_eq!(q2, q0.rotate_towards(q1, -FRAC_PI_2 * 1.5), eps);
        });

        glam_test!(test_fmt, {
            let a = $quat::IDENTITY;
            assert_eq!(
                format!("{:?}", a),
                format!("{}(0.0, 0.0, 0.0, 1.0)", stringify!($quat))
            );
            // assert_eq!(
            //     format!("{:#?}", a),
            //     "$quat(\n    1.0,\n    2.0,\n    3.0,\n    4.0\n)"
            // );
            assert_eq!(format!("{}", a), "[0, 0, 0, 1]");
            assert_eq!(format!("{:.2}", a), "[0.00, 0.00, 0.00, 1.00]");
        });

        glam_test!(test_identity, {
            let identity = $quat::IDENTITY;
            assert!(identity.is_near_identity());
            assert!(identity.is_normalized());
            assert_eq!(identity, $quat::from_xyzw(0.0, 0.0, 0.0, 1.0));
            assert_eq!(identity, identity * identity);
            let q = $quat::from_euler(EulerRot::YXZ, deg(10.0), deg(-10.0), deg(45.0));
            assert_eq!(q, q * identity);
            assert_eq!(q, identity * q);
            assert_eq!(identity, $quat::default());
        });

        glam_test!(test_slice, {
            let a: [$t; 4] =
                $quat::from_euler(EulerRot::YXZ, deg(30.0), deg(60.0), deg(90.0)).into();
            let b = $quat::from_slice(&a);
            let c: [$t; 4] = b.into();
            assert_eq!(a, c);
            let mut d = [0.0, 0.0, 0.0, 0.0];
            b.write_to_slice(&mut d[..]);
            assert_eq!(a, d);

            should_panic!({ $quat::IDENTITY.write_to_slice(&mut [0 as $t; 3]) });
            should_panic!({ $quat::from_slice(&[0 as $t; 3]) });
        });

        glam_test!(test_elements, {
            let x = 1.0;
            let y = 2.0;
            let z = 3.0;
            let w = 4.0;

            let a = $quat::from_xyzw(x, y, z, w);
            assert!(a.x == x);
            assert!(a.y == y);
            assert!(a.z == z);
            assert!(a.w == w);

            assert_eq!($vec3::new(1.0, 2.0, 3.0), a.xyz());
        });

        glam_test!(test_addition, {
            let a = $quat::from_xyzw(1.0, 2.0, 3.0, 4.0);
            let b = $quat::from_xyzw(5.0, 6.0, 7.0, -9.0);
            assert_eq!(a + b, $quat::from_xyzw(6.0, 8.0, 10.0, -5.0));
        });

        glam_test!(test_subtraction, {
            let a = $quat::from_xyzw(6.0, 8.0, 10.0, -5.0);
            let b = $quat::from_xyzw(5.0, 6.0, 7.0, -9.0);
            assert_eq!(a - b, $quat::from_xyzw(1.0, 2.0, 3.0, 4.0));
        });

        glam_test!(test_scalar_multiplication, {
            let a = $quat::from_xyzw(1.0, 2.0, 3.0, 4.0);
            assert_eq!(a * 2.0, $quat::from_xyzw(2.0, 4.0, 6.0, 8.0));
        });

        glam_test!(test_scalar_division, {
            let a = $quat::from_xyzw(2.0, 4.0, 6.0, 8.0);
            assert_eq!(a / 2.0, $quat::from_xyzw(1.0, 2.0, 3.0, 4.0));
        });

        glam_test!(test_sum, {
            let two = $new(2.0, 2.0, 2.0, 2.0);
            assert_eq!([two, two].iter().sum::<$quat>(), two + two);
            assert_eq!([two, two].into_iter().sum::<$quat>(), two + two);
        });

        glam_test!(test_product, {
            let two = $new(2.0, 2.0, 2.0, 2.0).normalize();
            assert_eq!([two, two].iter().product::<$quat>(), two * two);
            assert_eq!([two, two].into_iter().product::<$quat>(), two * two);
        });

        glam_test!(test_is_finite, {
            assert!($quat::from_xyzw(0.0, 0.0, 0.0, 0.0).is_finite());
            assert!($quat::from_xyzw(-1e-10, 1.0, 1e10, 42.0).is_finite());
            assert!(!$quat::from_xyzw($t::INFINITY, 0.0, 0.0, 0.0).is_finite());
            assert!(!$quat::from_xyzw(0.0, $t::NAN, 0.0, 0.0).is_finite());
            assert!(!$quat::from_xyzw(0.0, 0.0, $t::NEG_INFINITY, 0.0).is_finite());
            assert!(!$quat::from_xyzw(0.0, 0.0, 0.0, $t::NAN).is_finite());
        });

        glam_test!(test_rotation_arc, {
            let eps = 2.0 * $t::EPSILON.sqrt();

            for &from in &vec3_float_test_vectors!($vec3) {
                let from = from.normalize();

                {
                    let q = $quat::from_rotation_arc(from, from);
                    assert!(q.is_near_identity(), "from: {}, q: {}", from, q);
                }

                {
                    let q = $quat::from_rotation_arc_colinear(from, from);
                    assert!(q.is_near_identity(), "from: {}, q: {}", from, q);
                }

                {
                    let to = -from;
                    let q = $quat::from_rotation_arc(from, to);
                    assert!(q.is_normalized());
                    assert!((q * from - to).length() < eps);
                }

                {
                    let to = -from;
                    let q = $quat::from_rotation_arc_colinear(from, to);
                    assert!(q.is_near_identity(), "from: {}, q: {}", from, q);
                }

                for &to in &vec3_float_test_vectors!($vec3) {
                    let to = to.normalize();

                    let q = $quat::from_rotation_arc(from, to);
                    assert!(q.is_normalized());
                    assert!((q * from - to).length() < eps);

                    let q = $quat::from_rotation_arc_colinear(from, to);
                    assert!(q.is_normalized());
                    let transformed = q * from;
                    assert!(
                        (transformed - to).length() < eps || (-transformed - to).length() < eps
                    );
                }
            }

            for &from in &vec2_float_test_vectors!($vec2) {
                let from = from.normalize();

                {
                    let q = $quat::from_rotation_arc_2d(from, from);
                    assert!(q.is_near_identity(), "from: {}, q: {}", from, q);
                }

                {
                    let to = -from;
                    let q = $quat::from_rotation_arc_2d(from, to);
                    assert!(q.is_normalized());
                    assert!((q * from.extend(0.0) - to.extend(0.0)).length() < eps);
                }

                for &to in &vec2_float_test_vectors!($vec2) {
                    let to = to.normalize();

                    let q = $quat::from_rotation_arc_2d(from, to);
                    assert!(q.is_normalized());
                    assert!((q * from.extend(0.0) - to.extend(0.0)).length() < eps);
                }
            }

            should_glam_assert!({ $quat::from_rotation_arc($vec3::ZERO, $vec3::X) });
            should_glam_assert!({ $quat::from_rotation_arc($vec3::X, $vec3::ZERO) });
            should_glam_assert!({ $quat::from_rotation_arc_colinear($vec3::ZERO, $vec3::X) });
            should_glam_assert!({ $quat::from_rotation_arc_colinear($vec3::X, $vec3::ZERO) });

            should_glam_assert!({ $quat::from_rotation_arc_2d($vec2::ZERO, $vec2::X) });
            should_glam_assert!({ $quat::from_rotation_arc_2d($vec2::X, $vec2::ZERO) });
        });

        glam_test!(test_to_array, {
            assert!($new(1.0, 2.0, 3.0, 4.0).to_array() == [1.0, 2.0, 3.0, 4.0]);
        });

        glam_test!(test_to_axis_angle, {
            {
                let q = $quat::from_xyzw(
                    5.28124762e-08,
                    -5.12559303e-03,
                    8.29266140e-08,
                    9.99986828e-01,
                );
                assert!(q.is_normalized());
                let (axis, angle) = q.to_axis_angle();
                assert!(axis.is_normalized());
                let q2 = $quat::from_axis_angle(axis, angle);
                assert!((q.dot(q2) - 1.0).abs() < 1e-6);
            }
            {
                let q = $quat::IDENTITY;
                let (axis, angle) = q.to_axis_angle();
                assert!(axis.is_normalized());
                let q2 = $quat::from_axis_angle(axis, angle);
                assert!((q.dot(q2) - 1.0).abs() < 1e-6);
            }
            {
                let q = $quat::from_xyzw(0.0, 1.0, 0.0, 0.0);
                assert!(q.is_normalized());
                let (axis, angle) = q.to_axis_angle();
                assert!(axis.is_normalized());
                let q2 = $quat::from_axis_angle(axis, angle);
                assert!((q.dot(q2) - 1.0).abs() < 1e-6);
            }
            {
                let axis = $vec3::Z;
                let angle = core::$t::consts::PI * 0.25;
                let q = $quat::from_axis_angle(axis, angle);
                assert!(q.is_normalized());
                let (axis2, angle2) = q.to_axis_angle();
                assert!(axis.is_normalized());
                assert_approx_eq!(axis, axis2);
                assert_approx_eq!(angle, angle2);
            }
        });
    };
}

mod quat {
    use crate::support::{deg, rad};
    use core::ops::Neg;
    use glam::{quat, EulerRot, Mat3, Mat4, Quat, Vec2, Vec3, Vec3A, Vec4};

    glam_test!(test_align, {
        use std::mem;
        assert_eq!(16, mem::size_of::<Quat>());
        if cfg!(feature = "scalar-math") {
            assert_eq!(4, mem::align_of::<Quat>());
        } else {
            assert_eq!(16, mem::align_of::<Quat>());
        }
    });

    glam_test!(test_mul_vec3a, {
        let qrz = Quat::from_rotation_z(deg(90.0));
        assert_approx_eq!(Vec3A::Y, qrz * Vec3A::X);
        assert_approx_eq!(Vec3A::Y, qrz.mul_vec3a(Vec3A::X));
        assert_approx_eq!(Vec3A::Y, -qrz * Vec3A::X);
        assert_approx_eq!(Vec3A::Y, qrz.neg().mul_vec3a(Vec3A::X));
        assert_approx_eq!(-Vec3A::X, qrz * Vec3A::Y);
        assert_approx_eq!(-Vec3A::X, qrz.mul_vec3a(Vec3A::Y));
        assert_approx_eq!(-Vec3A::X, -qrz * Vec3A::Y);
        assert_approx_eq!(-Vec3A::X, qrz.neg().mul_vec3a(Vec3A::Y));

        // check vec3 * mat3 is the same
        let mrz = Mat3::from_quat(qrz);
        assert_approx_eq!(Vec3A::Y, mrz * Vec3A::X);
        assert_approx_eq!(Vec3A::Y, mrz.mul_vec3a(Vec3A::X));
        // assert_approx_eq!(Vec3A::Y, -mrz * Vec3A::X);
        assert_approx_eq!(-Vec3A::X, mrz * Vec3A::Y);
        assert_approx_eq!(-Vec3A::X, mrz.mul_vec3a(Vec3A::Y));

        let qrx = Quat::from_rotation_x(deg(90.0));
        assert_approx_eq!(Vec3A::X, qrx * Vec3A::X);
        assert_approx_eq!(Vec3A::X, qrx.mul_vec3a(Vec3A::X));
        assert_approx_eq!(Vec3A::X, -qrx * Vec3A::X);
        assert_approx_eq!(Vec3A::X, qrx.neg().mul_vec3a(Vec3A::X));
        assert_approx_eq!(Vec3A::Z, qrx * Vec3A::Y);
        assert_approx_eq!(Vec3A::Z, qrx.mul_vec3a(Vec3A::Y));
        assert_approx_eq!(Vec3A::Z, -qrx * Vec3A::Y);
        assert_approx_eq!(Vec3A::Z, qrx.neg().mul_vec3a(Vec3A::Y));

        // check vec3 * mat3 is the same
        let mrx = Mat3::from_quat(qrx);
        assert_approx_eq!(Vec3A::X, mrx * Vec3A::X);
        assert_approx_eq!(Vec3A::X, mrx.mul_vec3a(Vec3A::X));
        assert_approx_eq!(Vec3A::Z, mrx * Vec3A::Y);
        assert_approx_eq!(Vec3A::Z, mrx.mul_vec3a(Vec3A::Y));

        let qrxz = qrz * qrx;
        assert_approx_eq!(Vec3A::Y, qrxz * Vec3A::X);
        assert_approx_eq!(Vec3A::Y, qrxz.mul_vec3a(Vec3A::X));
        assert_approx_eq!(Vec3A::Z, qrxz * Vec3A::Y);
        assert_approx_eq!(Vec3A::Z, qrxz.mul_vec3a(Vec3A::Y));

        let mrxz = mrz * mrx;
        assert_approx_eq!(Vec3A::Y, mrxz * Vec3A::X);
        assert_approx_eq!(Vec3A::Y, mrxz.mul_vec3a(Vec3A::X));
        assert_approx_eq!(Vec3A::Z, mrxz * Vec3A::Y);
        assert_approx_eq!(Vec3A::Z, mrxz.mul_vec3a(Vec3A::Y));

        let qrzx = qrx * qrz;
        assert_approx_eq!(Vec3A::Z, qrzx * Vec3A::X);
        assert_approx_eq!(Vec3A::Z, qrzx.mul_vec3a(Vec3A::X));
        assert_approx_eq!(-Vec3A::X, qrzx * Vec3A::Y);
        assert_approx_eq!(-Vec3A::X, qrzx.mul_vec3a(Vec3A::Y));

        let mrzx = qrx * qrz;
        assert_approx_eq!(Vec3A::Z, mrzx * Vec3A::X);
        assert_approx_eq!(Vec3A::Z, mrzx.mul_vec3a(Vec3A::X));
        assert_approx_eq!(-Vec3A::X, mrzx * Vec3A::Y);
        assert_approx_eq!(-Vec3A::X, mrzx.mul_vec3a(Vec3A::Y));
    });

    glam_test!(test_from_mat3a, {
        use glam::Mat3A;
        let yaw = deg(30.0);
        let y0 = Quat::from_rotation_y(yaw);
        let y1 = Quat::from_mat3a(&Mat3A::from_rotation_y(yaw));
        assert_approx_eq!(y0, y1);
        let y2 = Quat::from_mat3a(&Mat3A::from_quat(y0));
        assert_approx_eq!(y0, y2);
    });

    glam_test!(test_as, {
        use glam::DQuat;
        assert_approx_eq!(
            DQuat::from_euler(EulerRot::YXZ, 1.0, 2.0, 3.0),
            Quat::from_euler(EulerRot::YXZ, 1.0, 2.0, 3.0).as_dquat()
        );
        assert_approx_eq!(
            Quat::from_euler(EulerRot::YXZ, 1.0, 2.0, 3.0),
            DQuat::from_euler(EulerRot::YXZ, 1.0, 2.0, 3.0).as_quat()
        );
    });

    impl_quat_tests!(f32, quat, Mat3, Mat4, Quat, Vec2, Vec3, Vec4);
}

mod dquat {
    use crate::support::{deg, rad};
    use core::ops::Neg;
    use glam::{dquat, DMat3, DMat4, DQuat, DVec2, DVec3, DVec4, EulerRot};

    glam_test!(test_align, {
        use std::mem;
        assert_eq!(32, mem::size_of::<DQuat>());
        assert_eq!(mem::align_of::<f64>(), mem::align_of::<DQuat>());
    });

    impl_quat_tests!(f64, dquat, DMat3, DMat4, DQuat, DVec2, DVec3, DVec4);
}
