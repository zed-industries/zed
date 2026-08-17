#[macro_use]
mod support;

macro_rules! impl_affine3_tests {
    ($t:ident, $affine3:ident, $quat:ident, $vec3:ident, $mat3:ident, $mat4:ident) => {
        const MATRIX1D: [$t; 12] = [
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0,
        ];
        const MATRIX2D: [[$t; 3]; 4] = [
            [1.0, 2.0, 3.0],
            [4.0, 5.0, 6.0],
            [7.0, 8.0, 9.0],
            [10.0, 11.0, 12.0],
        ];

        use core::$t::NAN;
        use core::$t::NEG_INFINITY;

        glam_test!(test_affine3_identity, {
            assert_eq!($affine3::IDENTITY, $affine3::IDENTITY * $affine3::IDENTITY);
            assert_eq!($affine3::IDENTITY, $affine3::default());
        });

        glam_test!(test_affine3_zero, {
            assert_eq!(
                $affine3::ZERO.transform_point3($vec3::new(1., 2., 3.)),
                $vec3::ZERO
            );
        });

        glam_test!(test_affine3_nan, {
            assert!($affine3::NAN.is_nan());
            assert!(!$affine3::NAN.is_finite());
        });

        glam_test!(test_affine3_from_cols, {
            let a = $affine3::from_cols(
                $vec3::from_array(MATRIX2D[0]).into(),
                $vec3::from_array(MATRIX2D[1]).into(),
                $vec3::from_array(MATRIX2D[2]).into(),
                $vec3::from_array(MATRIX2D[3]).into(),
            );
            assert_eq!(MATRIX2D, a.to_cols_array_2d());

            let a = $affine3::from_cols_array(&MATRIX1D);
            assert_eq!(MATRIX1D, a.to_cols_array());

            let a = $affine3::from_cols_array_2d(&MATRIX2D);
            assert_eq!(MATRIX2D, a.to_cols_array_2d());
        });

        glam_test!(test_affine3_deref, {
            let a = $affine3::from_cols_array_2d(&MATRIX2D);
            assert_eq!(MATRIX2D[0], a.x_axis.to_array());
            assert_eq!(MATRIX2D[1], a.y_axis.to_array());
            assert_eq!(MATRIX2D[2], a.z_axis.to_array());
            assert_eq!(MATRIX2D[3], a.w_axis.to_array());

            let mut b = a;
            b.x_axis *= 0.0;
            b.y_axis *= 0.0;
            b.z_axis *= 0.0;
            b.w_axis *= 0.0;
            assert_eq!($affine3::ZERO, b);
        });

        glam_test!(test_affine3_from_mat3, {
            let m = $mat3::from_cols_array_2d(&[MATRIX2D[0], MATRIX2D[1], MATRIX2D[2]]);
            let a = $affine3::from_mat3(m);
            assert_eq!(m, a.matrix3.into());
            assert_eq!($vec3::ZERO, a.translation.into());

            let t = $vec3::from_array(MATRIX2D[3]);
            let a = $affine3::from_mat3_translation(m, t);
            assert_eq!(MATRIX2D, a.to_cols_array_2d());
        });

        glam_test!(test_affine2_from_mat4, {
            let m = $mat4::from_cols_array_2d(&[
                [1.0, 2.0, 3.0, 0.0],
                [4.0, 5.0, 6.0, 0.0],
                [7.0, 8.0, 9.0, 0.0],
                [10.0, 11.0, 12.0, 1.0],
            ]);
            let a = $affine3::from_mat4(m);
            assert_eq!(MATRIX2D, a.to_cols_array_2d());

            assert_eq!(m, $mat4::from(a));
        });

        glam_test!(test_affine3_translation, {
            let translate = $affine3::from_translation($vec3::new(1.0, 2.0, 3.0));
            assert_eq!(translate.translation, $vec3::new(1.0, 2.0, 3.0).into());
            assert_eq!(
                translate.transform_point3($vec3::new(2.0, 3.0, 4.0)),
                $vec3::new(3.0, 5.0, 7.0),
            );
        });

        glam_test!(test_from_rotation, {
            let eps = 2.0 * core::f32::EPSILON;
            let rot_x1 = $affine3::from_rotation_x(deg(180.0));
            let rot_x2 = $affine3::from_axis_angle($vec3::X, deg(180.0));
            assert_approx_eq!(rot_x1, rot_x2, eps);
            let rot_y1 = $affine3::from_rotation_y(deg(180.0));
            let rot_y2 = $affine3::from_axis_angle($vec3::Y, deg(180.0));
            assert_approx_eq!(rot_y1, rot_y2, eps);
            let rot_z1 = $affine3::from_rotation_z(deg(180.0));
            let rot_z2 = $affine3::from_axis_angle($vec3::Z, deg(180.0));
            assert_approx_eq!(rot_z1, rot_z2, eps);

            assert_approx_eq!(
                $affine3::from_rotation_x(deg(180.0)),
                $affine3::from_quat($quat::from_rotation_x(deg(180.0)))
            );

            assert_approx_eq!(
                $quat::from_affine3(&$affine3::from_rotation_x(deg(180.0))),
                $quat::from_rotation_x(deg(180.0))
            );

            let m = $affine3::from_rotation_translation(
                $quat::from_rotation_x(deg(90.0)),
                $vec3::new(1.0, 2.0, 3.0),
            );
            let result3 = m.transform_vector3($vec3::Y);
            assert_approx_eq!($vec3::new(0.0, 0.0, 1.0), result3, 1.0e-6);

            should_glam_assert!({ $affine3::from_axis_angle($vec3::ZERO, 0.0) });
            should_glam_assert!({ $affine3::from_quat($quat::IDENTITY * 2.0) });
        });

        glam_test!(test_affine3_mul, {
            let m = $affine3::from_axis_angle($vec3::Z, deg(90.0));
            let result3 = m.transform_vector3($vec3::Y);
            assert_approx_eq!($vec3::new(-1.0, 0.0, 0.0), result3);

            let m = $affine3::from_scale_rotation_translation(
                $vec3::new(0.5, 1.5, 2.0),
                $quat::from_rotation_x(deg(90.0)),
                $vec3::new(1.0, 2.0, 3.0),
            );
            let result3 = m.transform_vector3($vec3::Y);
            assert_approx_eq!($vec3::new(0.0, 0.0, 1.5), result3, 1.0e-6);

            let result3 = m.transform_point3($vec3::Y);
            assert_approx_eq!($vec3::new(1.0, 2.0, 4.5), result3, 1.0e-6);
        });

        glam_test!(test_from_scale, {
            let m = $affine3::from_scale($vec3::new(2.0, 4.0, 8.0));
            assert_approx_eq!(
                m.transform_point3($vec3::new(1.0, 1.0, 1.0)),
                $vec3::new(2.0, 4.0, 8.0)
            );
        });

        glam_test!(test_affine3_inverse, {
            let inv = $affine3::IDENTITY.inverse();
            assert_approx_eq!($affine3::IDENTITY, inv);

            let rotz = $affine3::from_rotation_z(deg(90.0));
            let rotz_inv = rotz.inverse();
            assert_approx_eq!($affine3::IDENTITY, rotz * rotz_inv);
            assert_approx_eq!($affine3::IDENTITY, rotz_inv * rotz);

            let trans = $affine3::from_translation($vec3::new(1.0, 2.0, 3.0));
            let trans_inv = trans.inverse();
            assert_approx_eq!($affine3::IDENTITY, trans * trans_inv);
            assert_approx_eq!($affine3::IDENTITY, trans_inv * trans);

            let scale = $affine3::from_scale($vec3::new(4.0, 5.0, 6.0));
            let scale_inv = scale.inverse();
            assert_approx_eq!($affine3::IDENTITY, scale * scale_inv);
            assert_approx_eq!($affine3::IDENTITY, scale_inv * scale);

            let m = scale * rotz * trans;
            let m_inv = m.inverse();
            assert_approx_eq!($affine3::IDENTITY, m * m_inv, 1.0e-5);
            assert_approx_eq!($affine3::IDENTITY, m_inv * m, 1.0e-5);
            assert_approx_eq!(m_inv, trans_inv * rotz_inv * scale_inv, 1.0e-6);

            // Make sure we can invert a shear matrix:
            let m = $affine3::from_axis_angle($vec3::X, 0.5)
                * $affine3::from_scale($vec3::new(1.0, 0.5, 2.0))
                * $affine3::from_axis_angle($vec3::X, -0.5);
            let m_inv = m.inverse();
            assert_approx_eq!($affine3::IDENTITY, m * m_inv, 1.0e-5);
            assert_approx_eq!($affine3::IDENTITY, m_inv * m, 1.0e-5);

            should_glam_assert!({ $affine3::ZERO.inverse() });
        });

        glam_test!(test_affine3_decompose, {
            // identity
            let (out_scale, out_rotation, out_translation) =
                $affine3::IDENTITY.to_scale_rotation_translation();
            assert_approx_eq!($vec3::ONE, out_scale);
            assert!(out_rotation.is_near_identity());
            assert_approx_eq!($vec3::ZERO, out_translation);

            // no scale
            let in_scale = $vec3::ONE;
            let in_translation = $vec3::new(-2.0, 4.0, -0.125);
            let in_rotation = $quat::from_euler(
                glam::EulerRot::YXZ,
                $t::to_radians(-45.0),
                $t::to_radians(180.0),
                $t::to_radians(270.0),
            );
            let in_mat =
                $affine3::from_scale_rotation_translation(in_scale, in_rotation, in_translation);
            let (out_scale, out_rotation, out_translation) = in_mat.to_scale_rotation_translation();
            assert_approx_eq!(in_scale, out_scale, 1e-6);
            // out_rotation is different but produces the same matrix
            // assert_approx_eq!(in_rotation, out_rotation);
            assert_approx_eq!(in_translation, out_translation);
            assert_approx_eq!(
                in_mat,
                $affine3::from_scale_rotation_translation(out_scale, out_rotation, out_translation),
                1e-6
            );

            // positive scale
            let in_scale = $vec3::new(1.0, 2.0, 4.0);
            let in_mat =
                $affine3::from_scale_rotation_translation(in_scale, in_rotation, in_translation);
            let (out_scale, out_rotation, out_translation) = in_mat.to_scale_rotation_translation();
            assert_approx_eq!(in_scale, out_scale, 1e-6);
            // out_rotation is different but produces the same matrix
            // assert_approx_eq!(in_rotation, out_rotation);
            assert_approx_eq!(in_translation, out_translation);
            assert_approx_eq!(
                in_mat,
                $affine3::from_scale_rotation_translation(out_scale, out_rotation, out_translation),
                1e-5
            );

            // negative scale
            let in_scale = $vec3::new(-4.0, 1.0, 2.0);
            let in_mat =
                $affine3::from_scale_rotation_translation(in_scale, in_rotation, in_translation);
            let (out_scale, out_rotation, out_translation) = in_mat.to_scale_rotation_translation();
            assert_approx_eq!(in_scale, out_scale, 1e-6);
            // out_rotation is different but produces the same matrix
            // assert_approx_eq!(in_rotation, out_rotation);
            assert_approx_eq!(in_translation, out_translation);
            assert_approx_eq!(
                in_mat,
                $affine3::from_scale_rotation_translation(out_scale, out_rotation, out_translation),
                1e-5
            );

            // negative scale
            let in_scale = $vec3::new(4.0, -1.0, -2.0);
            let in_mat =
                $affine3::from_scale_rotation_translation(in_scale, in_rotation, in_translation);
            let (out_scale, out_rotation, out_translation) = in_mat.to_scale_rotation_translation();
            // out_scale and out_rotation are different but they produce the same matrix
            // assert_approx_eq!(in_scale, out_scale, 1e-6);
            // assert_approx_eq!(in_rotation, out_rotation);
            assert_approx_eq!(in_translation, out_translation);
            assert_approx_eq!(
                in_mat,
                $affine3::from_scale_rotation_translation(out_scale, out_rotation, out_translation),
                1e-6
            );
        });

        glam_test!(test_affine3_look_at, {
            let eye = $vec3::new(0.0, 0.0, -5.0);
            let center = $vec3::new(0.0, 0.0, 0.0);
            let up = $vec3::new(1.0, 0.0, 0.0);

            let point = $vec3::new(1.0, 0.0, 0.0);

            let lh = $affine3::look_at_lh(eye, center, up);
            let rh = $affine3::look_at_rh(eye, center, up);
            assert_approx_eq!(lh.transform_point3(point), $vec3::new(0.0, 1.0, 5.0));
            assert_approx_eq!(rh.transform_point3(point), $vec3::new(0.0, 1.0, -5.0));

            let dir = center - eye;
            let lh = $affine3::look_to_lh(eye, dir, up);
            let rh = $affine3::look_to_rh(eye, dir, up);
            assert_approx_eq!(lh.transform_point3(point), $vec3::new(0.0, 1.0, 5.0));
            assert_approx_eq!(rh.transform_point3(point), $vec3::new(0.0, 1.0, -5.0));

            should_glam_assert!({ $affine3::look_at_lh($vec3::ONE, $vec3::ZERO, $vec3::ZERO) });
            should_glam_assert!({ $affine3::look_at_rh($vec3::ONE, $vec3::ZERO, $vec3::ZERO) });
        });

        glam_test!(test_affine3_ops, {
            let m0 = $affine3::from_cols_array_2d(&MATRIX2D);
            assert_approx_eq!(m0, m0 * $affine3::IDENTITY);
            assert_approx_eq!(m0, $affine3::IDENTITY * m0);

            let mat4 = $mat4::from(m0);
            assert_approx_eq!(mat4, $affine3::IDENTITY * mat4);
            assert_approx_eq!(mat4, mat4 * $affine3::IDENTITY);
        });

        glam_test!(test_affine3_fmt, {
            let a = $affine3::from_cols_array_2d(&MATRIX2D);
            assert_eq!(
                format!("{}", a),
                "[[1, 2, 3], [4, 5, 6], [7, 8, 9], [10, 11, 12]]"
            );
        });

        glam_test!(test_affine3_to_from_slice, {
            let m = $affine3::from_cols_slice(&MATRIX1D);
            assert_eq!($affine3::from_cols_array(&MATRIX1D), m);
            assert_eq!(MATRIX1D, m.to_cols_array());
            assert_eq!(MATRIX2D, m.to_cols_array_2d());
            let mut out: [$t; 12] = Default::default();
            m.write_cols_to_slice(&mut out);
            assert_eq!(MATRIX1D, out);
            assert_eq!(
                m,
                $affine3::from_cols(
                    MATRIX2D[0].into(),
                    MATRIX2D[1].into(),
                    MATRIX2D[2].into(),
                    MATRIX2D[3].into()
                )
            );

            should_panic!({ $affine3::from_cols_slice(&[0.0; 11]) });
            should_panic!({ $affine3::IDENTITY.write_cols_to_slice(&mut [0.0; 11]) });
        });

        glam_test!(test_product, {
            let ident = $affine3::IDENTITY;
            assert_eq!(
                vec![ident, ident].iter().product::<$affine3>(),
                ident * ident
            );
        });

        glam_test!(test_affine3_is_finite, {
            assert!($affine3::from_scale($vec3::new(1.0, 1.0, 1.0)).is_finite());
            assert!($affine3::from_scale($vec3::new(0.0, 1.0, 1.0)).is_finite());
            assert!(!$affine3::from_scale($vec3::new(1.0, NAN, 1.0)).is_finite());
            assert!(!$affine3::from_scale($vec3::new(1.0, 1.0, NEG_INFINITY)).is_finite());
        });
    };
}

mod affine3a {
    use super::support::{deg, FloatCompare};
    use glam::{Affine3A, Mat3, Mat4, Quat, Vec3, Vec3A};

    impl FloatCompare for Affine3A {
        #[inline]
        fn approx_eq(&self, other: &Self, max_abs_diff: f32) -> bool {
            self.abs_diff_eq(*other, max_abs_diff)
        }
        #[inline]
        fn abs_diff(&self, other: &Self) -> Self {
            Self {
                matrix3: self.matrix3.abs_diff(&other.matrix3),
                translation: self.translation.abs_diff(&other.translation),
            }
        }
    }

    glam_test!(test_align, {
        use std::mem;
        assert_eq!(64, mem::size_of::<Affine3A>());
        assert_eq!(mem::align_of::<Vec3A>(), mem::align_of::<Affine3A>());
    });

    glam_test!(test_affine3_mul_vec3a, {
        let m = Affine3A::from_axis_angle(Vec3::Z, deg(90.0));
        let result3 = m.transform_vector3a(Vec3A::Y);
        assert_approx_eq!(Vec3A::new(-1.0, 0.0, 0.0), result3);

        let m = Affine3A::from_scale_rotation_translation(
            Vec3::new(0.5, 1.5, 2.0),
            Quat::from_rotation_x(deg(90.0)),
            Vec3::new(1.0, 2.0, 3.0),
        );
        let result3 = m.transform_vector3a(Vec3A::Y);
        assert_approx_eq!(Vec3A::new(0.0, 0.0, 1.5), result3, 1.0e-6);

        let result3 = m.transform_point3a(Vec3A::Y);
        assert_approx_eq!(Vec3A::new(1.0, 2.0, 4.5), result3, 1.0e-6);
    });

    impl_affine3_tests!(f32, Affine3A, Quat, Vec3, Mat3, Mat4);
}

mod daffine3 {
    use super::support::{deg, FloatCompare};
    use glam::{DAffine3, DMat3, DMat4, DQuat, DVec3};

    impl FloatCompare for DAffine3 {
        #[inline]
        fn approx_eq(&self, other: &Self, max_abs_diff: f32) -> bool {
            self.abs_diff_eq(*other, max_abs_diff as f64)
        }
        #[inline]
        fn abs_diff(&self, other: &Self) -> Self {
            Self {
                matrix3: self.matrix3.abs_diff(&other.matrix3),
                translation: self.translation.abs_diff(&other.translation),
            }
        }
    }

    glam_test!(test_align, {
        use std::mem;
        assert_eq!(96, mem::size_of::<DAffine3>());
        assert_eq!(mem::align_of::<f64>(), mem::align_of::<DAffine3>());
    });

    impl_affine3_tests!(f64, DAffine3, DQuat, DVec3, DMat3, DMat4);
}
