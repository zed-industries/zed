#[macro_use]
mod support;

macro_rules! impl_mat3_tests {
    ($t:ident, $newmat3:ident, $mat3:ident, $mat2:ident, $mat4:ident, $quat:ident, $newvec3:ident, $vec3:ident, $vec2:ident) => {
        const IDENTITY: [[$t; 3]; 3] = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
        const ARRAY3X3: [[$t; 3]; 3] = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]];
        const ARRAY1X9: [$t; 9] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        const ARRAY4X4: [[$t; 4]; 4] = [
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ];

        glam_test!(test_const, {
            const M0: $mat3 = $mat3::from_cols(
                $newvec3(1.0, 2.0, 3.0),
                $newvec3(4.0, 5.0, 6.0),
                $newvec3(7.0, 8.0, 9.0),
            );
            const M1: $mat3 = $mat3::from_cols_array(&ARRAY1X9);
            const M2: $mat3 = $mat3::from_cols_array_2d(&ARRAY3X3);

            assert_eq!(ARRAY1X9, M0.to_cols_array());
            assert_eq!(ARRAY1X9, M1.to_cols_array());
            assert_eq!(ARRAY1X9, M2.to_cols_array());
        });

        glam_test!(test_mat3_identity, {
            assert_eq!(
                $mat3::IDENTITY,
                $mat3::from_cols_array(&[
                    1., 0., 0., //
                    0., 1., 0., //
                    0., 0., 1., //
                ])
            );
            let identity = $mat3::IDENTITY;
            assert_eq!(IDENTITY, identity.to_cols_array_2d());
            assert_eq!($mat3::from_cols_array_2d(&IDENTITY), identity);
            assert_eq!(identity, identity * identity);
            assert_eq!(identity, $mat3::default());
            assert_eq!(identity, $mat3::from_diagonal($vec3::ONE));
        });

        glam_test!(test_mat3_zero, {
            assert_eq!(
                $mat3::ZERO,
                $mat3::from_cols_array(&[0., 0., 0., 0., 0., 0., 0., 0., 0.])
            );
        });

        glam_test!(test_mat3_nan, {
            assert!($mat3::NAN.is_nan());
            assert!(!$mat3::NAN.is_finite());
        });

        glam_test!(test_mat3_accessors, {
            let mut m = $mat3::ZERO;
            m.x_axis = $newvec3(1.0, 2.0, 3.0);
            m.y_axis = $newvec3(4.0, 5.0, 6.0);
            m.z_axis = $newvec3(7.0, 8.0, 9.0);
            assert_eq!($mat3::from_cols_array_2d(&ARRAY3X3), m);
            assert_eq!($newvec3(1.0, 2.0, 3.0), m.x_axis);
            assert_eq!($newvec3(4.0, 5.0, 6.0), m.y_axis);
            assert_eq!($newvec3(7.0, 8.0, 9.0), m.z_axis);

            assert_eq!($newvec3(1.0, 2.0, 3.0), m.col(0));
            assert_eq!($newvec3(4.0, 5.0, 6.0), m.col(1));
            assert_eq!($newvec3(7.0, 8.0, 9.0), m.col(2));

            assert_eq!($newvec3(1.0, 4.0, 7.0), m.row(0));
            assert_eq!($newvec3(2.0, 5.0, 8.0), m.row(1));
            assert_eq!($newvec3(3.0, 6.0, 9.0), m.row(2));

            *m.col_mut(0) = m.col(0).zyx();
            *m.col_mut(1) = m.col(1).zyx();
            *m.col_mut(2) = m.col(2).zyx();
            assert_eq!($newvec3(3.0, 2.0, 1.0), m.col(0));
            assert_eq!($newvec3(6.0, 5.0, 4.0), m.col(1));
            assert_eq!($newvec3(9.0, 8.0, 7.0), m.col(2));

            should_panic!({ $mat3::ZERO.col(3) });
            should_panic!({
                let mut m = $mat3::ZERO;
                m.col_mut(3);
            });
            should_panic!({ $mat3::ZERO.row(3) });
        });

        glam_test!(test_mat3_from_axes, {
            let a = $mat3::from_cols_array_2d(&[[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
            assert_eq!(ARRAY3X3, a.to_cols_array_2d());
            let b = $mat3::from_cols(
                $newvec3(1.0, 2.0, 3.0),
                $newvec3(4.0, 5.0, 6.0),
                $newvec3(7.0, 8.0, 9.0),
            );
            assert_eq!(a, b);
            let c = $newmat3(
                $newvec3(1.0, 2.0, 3.0),
                $newvec3(4.0, 5.0, 6.0),
                $newvec3(7.0, 8.0, 9.0),
            );
            assert_eq!(a, c);
            let d = b.to_cols_array();
            let f = $mat3::from_cols_array(&d);
            assert_eq!(b, f);
        });

        glam_test!(test_from_rotation, {
            let rot_x1 = $mat3::from_rotation_x(deg(180.0));
            let rot_x2 = $mat3::from_axis_angle($vec3::X, deg(180.0));
            assert_approx_eq!(rot_x1, rot_x2);
            let rot_x3 = $mat3::from_quat($quat::from_rotation_x(deg(180.0)));
            assert_approx_eq!(rot_x1, rot_x3);

            let rot_y1 = $mat3::from_rotation_y(deg(180.0));
            let rot_y2 = $mat3::from_axis_angle($vec3::Y, deg(180.0));
            assert_approx_eq!(rot_y1, rot_y2);
            let rot_y3 = $mat3::from_quat($quat::from_rotation_y(deg(180.0)));
            assert_approx_eq!(rot_y1, rot_y3);

            let rot_z1 = $mat3::from_rotation_z(deg(180.0));
            let rot_z2 = $mat3::from_axis_angle($vec3::Z, deg(180.0));
            assert_approx_eq!(rot_z1, rot_z2);
            let rot_z3 = $mat3::from_quat($quat::from_rotation_z(deg(180.0)));
            assert_approx_eq!(rot_z1, rot_z3);

            should_glam_assert!({ $mat3::from_axis_angle($vec3::ZERO, 0.0) });
            should_glam_assert!({ $mat3::from_quat($quat::from_xyzw(0.0, 0.0, 0.0, 0.0)) });
        });

        glam_test!(test_mat3_mul, {
            let mat_a = $mat3::from_axis_angle($vec3::Z, deg(90.0));
            assert_approx_eq!($newvec3(-1.0, 0.0, 0.0), mat_a * $newvec3(0.0, 1.0, 0.0));
            assert_approx_eq!(
                $vec3::new(-1.0, 0.0, 0.0),
                mat_a.mul_vec3($vec3::new(0.0, 1.0, 0.0))
            );
        });

        glam_test!(test_mat3_transform2d, {
            let m = $mat3::from_translation($vec2::new(2.0, 4.0));
            assert_eq!($vec2::ZERO, m.transform_vector2($vec2::ZERO));
            assert_eq!($vec2::new(2.0, 4.0), m.transform_point2($vec2::ZERO));
            assert_eq!($vec2::ZERO, m.transform_point2($vec2::new(-2.0, -4.0)));

            let m = $mat3::from_angle($t::to_radians(90.0));
            assert_approx_eq!($vec2::Y, m.transform_vector2($vec2::X), 1e-7);
            assert_approx_eq!($vec2::Y, m.transform_point2($vec2::X), 1e-7);

            let m = $mat3::from_scale($vec2::new(2.0, 4.0));
            assert_eq!($vec2::new(2.0, 0.0), m.transform_vector2($vec2::X));
            assert_eq!($vec2::new(0.0, 4.0), m.transform_vector2($vec2::Y));
            assert_eq!($vec2::new(2.0, 0.0), m.transform_point2($vec2::X));
            assert_eq!($vec2::new(0.0, 4.0), m.transform_point2($vec2::Y));

            should_glam_assert!({ $mat3::from_scale($vec2::ZERO) });

            let m = $mat3::from_scale_angle_translation(
                $vec2::new(0.5, 1.5),
                $t::to_radians(90.0),
                $vec2::new(1.0, 2.0),
            );
            let result2 = m.transform_vector2($vec2::Y);
            assert_approx_eq!($vec2::new(-1.5, 0.0), result2, 1.0e-6);
            assert_approx_eq!(result2, (m * $vec2::Y.extend(0.0)).truncate());

            let result2 = m.transform_point2($vec2::Y);
            assert_approx_eq!($vec2::new(-0.5, 2.0), result2, 1.0e-6);
            assert_approx_eq!(result2, (m * $vec2::Y.extend(1.0)).truncate());
        });

        glam_test!(test_from_ypr, {
            use glam::EulerRot;
            let zero = deg(0.0);
            let yaw = deg(30.0);
            let pitch = deg(60.0);
            let roll = deg(90.0);
            let y0 = $mat3::from_rotation_y(yaw);
            let y1 = $mat3::from_euler(EulerRot::YXZ, yaw, zero, zero);
            assert_approx_eq!(y0, y1);

            let x0 = $mat3::from_rotation_x(pitch);
            let x1 = $mat3::from_euler(EulerRot::YXZ, zero, pitch, zero);
            assert_approx_eq!(x0, x1);

            let z0 = $mat3::from_rotation_z(roll);
            let z1 = $mat3::from_euler(EulerRot::YXZ, zero, zero, roll);
            assert_approx_eq!(z0, z1);

            let yx0 = y0 * x0;
            let yx1 = $mat3::from_euler(EulerRot::YXZ, yaw, pitch, zero);
            assert_approx_eq!(yx0, yx1, 1e-6);

            let yxz0 = y0 * x0 * z0;
            let yxz1 = $mat3::from_euler(EulerRot::YXZ, yaw, pitch, roll);
            assert_approx_eq!(yxz0, yxz1, 1e-6);
        });

        glam_test!(test_from_diagonal, {
            let m = $mat3::from_diagonal($vec3::new(2.0, 4.0, 8.0));
            assert_approx_eq!(m * $vec3::new(1.0, 1.0, 1.0), $vec3::new(2.0, 4.0, 8.0));
            assert_approx_eq!($newvec3(2.0, 0.0, 0.0), m.x_axis);
            assert_approx_eq!($newvec3(0.0, 4.0, 0.0), m.y_axis);
            assert_approx_eq!($newvec3(0.0, 0.0, 8.0), m.z_axis);
        });

        glam_test!(test_from_mat2, {
            let m2 = $mat2::from_cols_array_2d(&[[1.0, 2.0], [3.0, 4.0]]);
            let m3 = $mat3::from_mat2(m2);
            assert_eq!(
                $mat3::from_cols_array_2d(&[[1.0, 2.0, 0.0], [3.0, 4.0, 0.0], [0.0, 0.0, 1.0]]),
                m3
            );
        });

        glam_test!(test_from_mat4, {
            let m4 = $mat4::from_cols_array_2d(&ARRAY4X4);
            let m3 = $mat3::from_mat4(m4);
            assert_eq!(
                $mat3::from_cols_array_2d(&[[1.0, 2.0, 3.0], [5.0, 6.0, 7.0], [9.0, 10.0, 11.0]]),
                m3
            );
        });

        glam_test!(test_from_mat4_minor, {
            let m4 = $mat4::from_cols_array_2d(&ARRAY4X4);
            for i in 0..4 {
                for j in 0..4 {
                    let m3 = $mat3::from_mat4_minor(m4, i, j);
                    test_matrix_minor!(4, m3, m4, i, j);
                }
            }
            should_panic!({ $mat3::from_mat4_minor(m4, 4, 0) });
            should_panic!({ $mat3::from_mat4_minor(m4, 0, 4) });
        });

        glam_test!(test_mat3_transpose, {
            let m = $newmat3(
                $newvec3(1.0, 2.0, 3.0),
                $newvec3(4.0, 5.0, 6.0),
                $newvec3(7.0, 8.0, 9.0),
            );
            let mt = m.transpose();
            assert_eq!($newvec3(1.0, 4.0, 7.0), mt.x_axis);
            assert_eq!($newvec3(2.0, 5.0, 8.0), mt.y_axis);
            assert_eq!($newvec3(3.0, 6.0, 9.0), mt.z_axis);
        });

        glam_test!(test_mat3_det, {
            assert_eq!(0.0, $mat3::ZERO.determinant());
            assert_eq!(1.0, $mat3::IDENTITY.determinant());
            assert_eq!(1.0, $mat3::from_rotation_x(deg(90.0)).determinant());
            assert_eq!(1.0, $mat3::from_rotation_y(deg(180.0)).determinant());
            assert_eq!(1.0, $mat3::from_rotation_z(deg(270.0)).determinant());
            assert_eq!(
                2.0 * 2.0 * 2.0,
                $mat3::from_diagonal($vec3::new(2.0, 2.0, 2.0)).determinant()
            );
        });

        glam_test!(test_mat3_inverse, {
            // assert_eq!(None, $mat3::ZERO.inverse());
            let inv = $mat3::IDENTITY.inverse();
            // assert_ne!(None, inv);
            assert_approx_eq!($mat3::IDENTITY, inv);

            let rotz = $mat3::from_rotation_z(deg(90.0));
            let rotz_inv = rotz.inverse();
            // assert_ne!(None, rotz_inv);
            // let rotz_inv = rotz_inv.unwrap();
            assert_approx_eq!($mat3::IDENTITY, rotz * rotz_inv);
            assert_approx_eq!($mat3::IDENTITY, rotz_inv * rotz);

            let scale = $mat3::from_diagonal($vec3::new(4.0, 5.0, 6.0));
            let scale_inv = scale.inverse();
            // assert_ne!(None, scale_inv);
            // let scale_inv = scale_inv.unwrap();
            assert_approx_eq!($mat3::IDENTITY, scale * scale_inv);
            assert_approx_eq!($mat3::IDENTITY, scale_inv * scale);

            let m = scale * rotz;
            let m_inv = m.inverse();
            // assert_ne!(None, m_inv);
            // let m_inv = m_inv.unwrap();
            assert_approx_eq!($mat3::IDENTITY, m * m_inv);
            assert_approx_eq!($mat3::IDENTITY, m_inv * m);
            assert_approx_eq!(m_inv, rotz_inv * scale_inv);

            should_glam_assert!({ $mat3::ZERO.inverse() });
        });

        glam_test!(test_mat3_ops, {
            let m0 = $mat3::from_cols_array_2d(&ARRAY3X3);
            let m0x2 = $mat3::from_cols_array_2d(&[
                [2.0, 4.0, 6.0],
                [8.0, 10.0, 12.0],
                [14.0, 16.0, 18.0],
            ]);
            let m0_neg = $mat3::from_cols_array_2d(&[
                [-1.0, -2.0, -3.0],
                [-4.0, -5.0, -6.0],
                [-7.0, -8.0, -9.0],
            ]);
            assert_eq!(m0x2, m0 * 2.0);
            assert_eq!(m0x2, 2.0 * m0);
            assert_eq!(m0, m0x2 / 2.0);
            assert_eq!(m0, 2.0 / m0x2);
            assert_eq!(m0x2, m0 + m0);
            assert_eq!($mat3::ZERO, m0 - m0);
            assert_eq!(m0_neg, -m0);
            assert_approx_eq!(m0, m0 * $mat3::IDENTITY);
            assert_approx_eq!(m0, $mat3::IDENTITY * m0);

            let mut m1 = m0;
            m1 *= 2.0;
            assert_eq!(m0x2, m1);

            let mut m1 = m0x2;
            m1 /= 2.0;
            assert_eq!(m0, m1);

            let mut m1 = m0;
            m1 += m0;
            assert_eq!(m0x2, m1);

            let mut m1 = m0;
            m1 -= m0;
            assert_eq!($mat3::ZERO, m1);

            let mut m1 = $mat3::IDENTITY;
            m1 *= m0;
            assert_approx_eq!(m0, m1);
        });

        glam_test!(test_mat3_fmt, {
            let a = $mat3::from_cols_array_2d(&ARRAY3X3);
            assert_eq!(format!("{}", a), "[[1, 2, 3], [4, 5, 6], [7, 8, 9]]");
            assert_eq!(
                format!("{:.1}", a),
                "[[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]"
            );
        });

        glam_test!(test_mat3_to_from_slice, {
            let m = $mat3::from_cols_slice(&ARRAY1X9);
            assert_eq!($mat3::from_cols_array(&ARRAY1X9), m);
            let mut out: [$t; 9] = Default::default();
            m.write_cols_to_slice(&mut out);
            assert_eq!(ARRAY1X9, out);

            should_panic!({ $mat3::from_cols_slice(&[0.0; 8]) });
            should_panic!({ $mat3::IDENTITY.write_cols_to_slice(&mut [0.0; 8]) });
        });

        glam_test!(test_sum, {
            let id = $mat3::IDENTITY;
            assert_eq!([id, id].iter().sum::<$mat3>(), id + id);
            assert_eq!([id, id].into_iter().sum::<$mat3>(), id + id);
        });

        glam_test!(test_product, {
            let two = $mat3::IDENTITY + $mat3::IDENTITY;
            assert_eq!([two, two].iter().product::<$mat3>(), two * two);
            assert_eq!([two, two].into_iter().product::<$mat3>(), two * two);
        });

        glam_test!(test_mat3_is_finite, {
            assert!($mat3::IDENTITY.is_finite());
            assert!(!($mat3::IDENTITY * $t::INFINITY).is_finite());
            assert!(!($mat3::IDENTITY * $t::NEG_INFINITY).is_finite());
            assert!(!($mat3::IDENTITY * $t::NAN).is_finite());
        });
    };
}

macro_rules! impl_as_ref_tests {
    ($mat:ident) => {
        glam_test!(test_as_ref, {
            let m = $mat::from_cols_array_2d(&ARRAY3X3);
            assert_eq!(ARRAY1X9, *m.as_ref());
        });
        glam_test!(test_as_mut, {
            let mut m = $mat::ZERO;
            *m.as_mut() = ARRAY1X9;
            assert_eq!($mat::from_cols_array_2d(&ARRAY3X3), m);
        });
    };
}

mod mat3 {
    use super::support::deg;
    use glam::{mat3, swizzles::*, vec3, vec3a, Mat2, Mat3, Mat4, Quat, Vec2, Vec3, Vec3A};

    glam_test!(test_align, {
        use std::mem;
        assert_eq!(36, mem::size_of::<Mat3>());
        assert_eq!(mem::align_of::<Vec3>(), mem::align_of::<Mat3>());
    });

    glam_test!(test_mul_vec3a, {
        let mat_a = Mat3::from_axis_angle(Vec3::Z, deg(90.0));
        assert_approx_eq!(vec3a(-1.0, 0.0, 0.0), mat_a * Vec3A::Y);
        assert_approx_eq!(vec3a(-1.0, 0.0, 0.0), mat_a.mul_vec3a(Vec3A::Y));
    });

    glam_test!(test_as, {
        use glam::DMat3;
        assert_eq!(
            DMat3::from_cols_array(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]),
            Mat3::from_cols_array(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]).as_dmat3()
        );
        assert_eq!(
            Mat3::from_cols_array(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]),
            DMat3::from_cols_array(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]).as_mat3()
        );
    });

    impl_mat3_tests!(f32, mat3, Mat3, Mat2, Mat4, Quat, vec3, Vec3, Vec2);
    impl_as_ref_tests!(Mat3);
}

mod mat3a {
    use super::support::deg;
    use glam::{mat3a, swizzles::*, vec3a, Mat2, Mat3A, Mat4, Quat, Vec2, Vec3, Vec3A};

    glam_test!(test_align, {
        use std::mem;
        assert_eq!(48, mem::size_of::<Mat3A>());
        assert_eq!(mem::align_of::<Vec3A>(), mem::align_of::<Mat3A>());
    });

    glam_test!(test_mul_vec3a, {
        let mat_a = Mat3A::from_axis_angle(Vec3::Z, deg(90.0));
        assert_approx_eq!(vec3a(-1.0, 0.0, 0.0), mat_a * Vec3A::Y);
        assert_approx_eq!(vec3a(-1.0, 0.0, 0.0), mat_a.mul_vec3a(Vec3A::Y));
    });

    glam_test!(test_as, {
        use glam::DMat3;
        assert_eq!(
            DMat3::from_cols_array(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]),
            Mat3A::from_cols_array(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]).as_dmat3()
        );
    });

    impl_mat3_tests!(f32, mat3a, Mat3A, Mat2, Mat4, Quat, vec3a, Vec3, Vec2);
}

mod dmat3 {
    use super::support::deg;
    use glam::{dmat3, dvec3, swizzles::*, DMat2, DMat3, DMat4, DQuat, DVec2, DVec3};

    glam_test!(test_align, {
        use std::mem;
        assert_eq!(72, mem::size_of::<DMat3>());
        assert_eq!(mem::align_of::<DVec3>(), mem::align_of::<DMat3>());
    });

    impl_mat3_tests!(f64, dmat3, DMat3, DMat2, DMat4, DQuat, dvec3, DVec3, DVec2);
    impl_as_ref_tests!(DMat3);
}
