#[macro_use]
mod support;

macro_rules! impl_mat4_tests {
    ($t:ident, $newmat4:ident, $newvec4:ident, $newvec3:ident, $mat4:ident, $mat3:ident, $quat:ident, $vec4:ident, $vec3:ident) => {
        const IDENTITY: [[$t; 4]; 4] = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        const ARRAY4X4: [[$t; 4]; 4] = [
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ];
        const ARRAY1X16: [$t; 16] = [
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        ];
        const ARRAY3X3: [[$t; 3]; 3] = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]];

        glam_test!(test_const, {
            const M0: $mat4 = $mat4::from_cols(
                $newvec4(1.0, 2.0, 3.0, 4.0),
                $newvec4(5.0, 6.0, 7.0, 8.0),
                $newvec4(9.0, 10.0, 11.0, 12.0),
                $newvec4(13.0, 14.0, 15.0, 16.0),
            );
            const M1: $mat4 = $mat4::from_cols_array(&ARRAY1X16);
            const M2: $mat4 = $mat4::from_cols_array_2d(&ARRAY4X4);

            assert_eq!(ARRAY1X16, M0.to_cols_array());
            assert_eq!(ARRAY1X16, M1.to_cols_array());
            assert_eq!(ARRAY1X16, M2.to_cols_array());
        });

        glam_test!(test_mat4_identity, {
            assert_eq!(
                $mat4::IDENTITY,
                $mat4::from_cols_array(&[
                    1., 0., 0., 0., //
                    0., 1., 0., 0., //
                    0., 0., 1., 0., //
                    0., 0., 0., 1., //
                ])
            );
            let identity = $mat4::IDENTITY;
            assert_eq!(IDENTITY, identity.to_cols_array_2d());
            assert_eq!($mat4::from_cols_array_2d(&IDENTITY), identity);
            assert_eq!(identity, identity * identity);
            assert_eq!(identity, $mat4::default());
            assert_eq!(identity, $mat4::from_diagonal($vec4::ONE));
        });

        glam_test!(test_mat4_zero, {
            assert_eq!(
                $mat4::ZERO,
                $mat4::from_cols_array(&[
                    0., 0., 0., 0., //
                    0., 0., 0., 0., //
                    0., 0., 0., 0., //
                    0., 0., 0., 0., //
                ])
            );
        });

        glam_test!(test_mat4_nan, {
            assert!($mat4::NAN.is_nan());
            assert!(!$mat4::NAN.is_finite());
        });

        glam_test!(test_mat4_accessors, {
            let mut m = $mat4::ZERO;
            m.x_axis = $vec4::new(1.0, 2.0, 3.0, 4.0);
            m.y_axis = $vec4::new(5.0, 6.0, 7.0, 8.0);
            m.z_axis = $vec4::new(9.0, 10.0, 11.0, 12.0);
            m.w_axis = $vec4::new(13.0, 14.0, 15.0, 16.0);
            assert_eq!($mat4::from_cols_array_2d(&ARRAY4X4), m);
            assert_eq!($vec4::new(1.0, 2.0, 3.0, 4.0), m.x_axis);
            assert_eq!($vec4::new(5.0, 6.0, 7.0, 8.0), m.y_axis);
            assert_eq!($vec4::new(9.0, 10.0, 11.0, 12.0), m.z_axis);
            assert_eq!($vec4::new(13.0, 14.0, 15.0, 16.0), m.w_axis);

            assert_eq!($vec4::new(1.0, 2.0, 3.0, 4.0), m.col(0));
            assert_eq!($vec4::new(5.0, 6.0, 7.0, 8.0), m.col(1));
            assert_eq!($vec4::new(9.0, 10.0, 11.0, 12.0), m.col(2));
            assert_eq!($vec4::new(13.0, 14.0, 15.0, 16.0), m.col(3));

            assert_eq!($newvec4(1.0, 5.0, 9.0, 13.0), m.row(0));
            assert_eq!($newvec4(2.0, 6.0, 10.0, 14.0), m.row(1));
            assert_eq!($newvec4(3.0, 7.0, 11.0, 15.0), m.row(2));
            assert_eq!($newvec4(4.0, 8.0, 12.0, 16.0), m.row(3));

            *m.col_mut(0) = m.col(0).wzyx();
            *m.col_mut(1) = m.col(1).wzyx();
            *m.col_mut(2) = m.col(2).wzyx();
            *m.col_mut(3) = m.col(3).wzyx();
            assert_eq!($newvec4(4.0, 3.0, 2.0, 1.0), m.col(0));
            assert_eq!($newvec4(8.0, 7.0, 6.0, 5.0), m.col(1));
            assert_eq!($newvec4(12.0, 11.0, 10.0, 9.0), m.col(2));
            assert_eq!($newvec4(16.0, 15.0, 14.0, 13.0), m.col(3));

            should_panic!({ $mat4::ZERO.col(4) });
            should_panic!({
                let mut m = $mat4::ZERO;
                m.col_mut(4);
            });
            should_panic!({ $mat4::ZERO.row(4) });
        });

        glam_test!(test_mat4_from_axes, {
            let a = $mat4::from_cols_array_2d(&[
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 10.0, 11.0, 12.0],
                [13.0, 14.0, 15.0, 16.0],
            ]);
            assert_eq!(ARRAY4X4, a.to_cols_array_2d());
            let b = $mat4::from_cols(
                $newvec4(1.0, 2.0, 3.0, 4.0),
                $newvec4(5.0, 6.0, 7.0, 8.0),
                $newvec4(9.0, 10.0, 11.0, 12.0),
                $newvec4(13.0, 14.0, 15.0, 16.0),
            );
            assert_eq!(a, b);
            let c = $newmat4(
                $newvec4(1.0, 2.0, 3.0, 4.0),
                $newvec4(5.0, 6.0, 7.0, 8.0),
                $newvec4(9.0, 10.0, 11.0, 12.0),
                $newvec4(13.0, 14.0, 15.0, 16.0),
            );
            assert_eq!(a, c);
            let d = b.to_cols_array();
            let f = $mat4::from_cols_array(&d);
            assert_eq!(b, f);
        });

        glam_test!(test_mat4_translation, {
            let translate = $mat4::from_translation($newvec3(1.0, 2.0, 3.0));
            assert_eq!(
                $mat4::from_cols(
                    $newvec4(1.0, 0.0, 0.0, 0.0),
                    $newvec4(0.0, 1.0, 0.0, 0.0),
                    $newvec4(0.0, 0.0, 1.0, 0.0),
                    $newvec4(1.0, 2.0, 3.0, 1.0)
                ),
                translate
            );
        });

        glam_test!(test_from_rotation, {
            let rot_x1 = $mat4::from_rotation_x(deg(180.0));
            let rot_x2 = $mat4::from_axis_angle($vec3::X, deg(180.0));
            assert_approx_eq!(rot_x1, rot_x2);
            let rot_y1 = $mat4::from_rotation_y(deg(180.0));
            let rot_y2 = $mat4::from_axis_angle($vec3::Y, deg(180.0));
            assert_approx_eq!(rot_y1, rot_y2);
            let rot_z1 = $mat4::from_rotation_z(deg(180.0));
            let rot_z2 = $mat4::from_axis_angle($vec3::Z, deg(180.0));
            assert_approx_eq!(rot_z1, rot_z2);

            assert_approx_eq!($mat4::IDENTITY, $mat4::from_quat($quat::IDENTITY));

            should_glam_assert!({ $mat4::from_axis_angle($vec3::ZERO, 0.0) });
            should_glam_assert!({ $mat4::from_quat($quat::from_xyzw(0.0, 0.0, 0.0, 0.0)) });
        });

        glam_test!(test_from_mat3, {
            let m3 =
                $mat3::from_cols_array_2d(&ARRAY3X3);
            let m4 = $mat4::from_mat3(m3);
            assert_eq!(
                $mat4::from_cols_array_2d(&[
                    [1.0, 2.0, 3.0, 0.0],
                    [4.0, 5.0, 6.0, 0.0],
                    [7.0, 8.0, 9.0, 0.0],
                    [0.0, 0.0, 0.0, 1.0]
                ]),
                m4
            );
        });

        glam_test!(test_mat4_mul, {
            let m = $mat4::from_axis_angle($vec3::Z, deg(90.0));
            let result3 = m.transform_vector3($vec3::Y);
            assert_approx_eq!($newvec3(-1.0, 0.0, 0.0), result3);
            assert_approx_eq!(result3, (m * $vec3::Y.extend(0.0)).truncate().into());
            let result4 = m * $vec4::Y;
            assert_approx_eq!($newvec4(-1.0, 0.0, 0.0, 0.0), result4);
            assert_approx_eq!(result4, m * $vec4::Y);

            let m = $mat4::from_scale_rotation_translation(
                $vec3::new(0.5, 1.5, 2.0),
                $quat::from_rotation_x(deg(90.0)),
                $vec3::new(1.0, 2.0, 3.0),
            );
            let result3 = m.transform_vector3($vec3::Y);
            assert_approx_eq!($newvec3(0.0, 0.0, 1.5), result3, 1.0e-6);
            assert_approx_eq!(result3, (m * $vec3::Y.extend(0.0)).truncate().into());

            let result3 = m.transform_point3($vec3::Y);
            assert_approx_eq!($newvec3(1.0, 2.0, 4.5), result3, 1.0e-6);
            assert_approx_eq!(result3, (m * $vec3::Y.extend(1.0)).truncate().into());

            let m = $mat4::from_cols(
                $newvec4(8.0, 0.0, 0.0, 0.0),
                $newvec4(0.0, 4.0, 0.0, 0.0),
                $newvec4(0.0, 0.0, 2.0, 2.0),
                $newvec4(0.0, 0.0, 0.0, 0.0),
            );
            assert_approx_eq!(
                $newvec3(4.0, 2.0, 1.0),
                m.project_point3($newvec3(2.0, 2.0, 2.0))
            );

            should_glam_assert!({ $mat4::ZERO.transform_vector3($vec3::X) });
            should_glam_assert!({ $mat4::ZERO.transform_point3($vec3::X) });
        });

        glam_test!(test_from_ypr, {
            use glam::EulerRot;
            let zero = deg(0.0);
            let yaw = deg(30.0);
            let pitch = deg(60.0);
            let roll = deg(90.0);
            let y0 = $mat4::from_rotation_y(yaw);
            let y1 = $mat4::from_euler(EulerRot::YXZ, yaw, zero, zero);
            assert_approx_eq!(y0, y1);

            let x0 = $mat4::from_rotation_x(pitch);
            let x1 = $mat4::from_euler(EulerRot::YXZ, zero, pitch, zero);
            assert_approx_eq!(x0, x1);

            let z0 = $mat4::from_rotation_z(roll);
            let z1 = $mat4::from_euler(EulerRot::YXZ, zero, zero, roll);
            assert_approx_eq!(z0, z1);

            let yx0 = y0 * x0;
            let yx1 = $mat4::from_euler(EulerRot::YXZ, yaw, pitch, zero);
            assert_approx_eq!(yx0, yx1, 1e-6);

            let yxz0 = y0 * x0 * z0;
            let yxz1 = $mat4::from_euler(EulerRot::YXZ, yaw, pitch, roll);
            assert_approx_eq!(yxz0, yxz1, 1e-6);
        });

        glam_test!(test_from_scale, {
            let m = $mat4::from_scale($vec3::new(2.0, 4.0, 8.0));
            assert_approx_eq!($vec4::X * 2.0, m.x_axis);
            assert_approx_eq!($vec4::Y * 4.0, m.y_axis);
            assert_approx_eq!($vec4::Z * 8.0, m.z_axis);
            assert_approx_eq!($vec4::W, m.w_axis);
            assert_approx_eq!(
                m.transform_point3($vec3::new(1.0, 1.0, 1.0)),
                $vec3::new(2.0, 4.0, 8.0)
            );

            should_glam_assert!({ $mat4::from_scale($vec3::ZERO) });
        });

        glam_test!(test_mat4_transpose, {
            let m = $newmat4(
                $newvec4(1.0, 2.0, 3.0, 4.0),
                $newvec4(5.0, 6.0, 7.0, 8.0),
                $newvec4(9.0, 10.0, 11.0, 12.0),
                $newvec4(13.0, 14.0, 15.0, 16.0),
            );
            let mt = m.transpose();
            assert_eq!($newvec4(1.0, 5.0, 9.0, 13.0), mt.x_axis);
            assert_eq!($newvec4(2.0, 6.0, 10.0, 14.0), mt.y_axis);
            assert_eq!($newvec4(3.0, 7.0, 11.0, 15.0), mt.z_axis);
            assert_eq!($newvec4(4.0, 8.0, 12.0, 16.0), mt.w_axis);
        });

        glam_test!(test_mat4_det, {
            assert_eq!(0.0, $mat4::ZERO.determinant());
            assert_eq!(1.0, $mat4::IDENTITY.determinant());
            assert_eq!(1.0, $mat4::from_rotation_x(deg(90.0)).determinant());
            assert_eq!(1.0, $mat4::from_rotation_y(deg(180.0)).determinant());
            assert_eq!(1.0, $mat4::from_rotation_z(deg(270.0)).determinant());
            assert_eq!(
                2.0 * 2.0 * 2.0,
                $mat4::from_scale($newvec3(2.0, 2.0, 2.0)).determinant()
            );
            assert_eq!(
                1.0,
                $newmat4(
                    $newvec4(0.0, 0.0, 0.0, 1.0),
                    $newvec4(1.0, 0.0, 0.0, 0.0),
                    $newvec4(0.0, 0.0, 1.0, 0.0),
                    $newvec4(0.0, 1.0, 0.0, 0.0),
                )
                .determinant()
            );
        });

        glam_test!(test_mat4_inverse, {
            // assert_eq!(None, $mat4::ZERO.inverse());
            let inv = $mat4::IDENTITY.inverse();
            // assert_ne!(None, inv);
            assert_approx_eq!($mat4::IDENTITY, inv);

            let rotz = $mat4::from_rotation_z(deg(90.0));
            let rotz_inv = rotz.inverse();
            // assert_ne!(None, rotz_inv);
            // let rotz_inv = rotz_inv.unwrap();
            assert_approx_eq!($mat4::IDENTITY, rotz * rotz_inv);
            assert_approx_eq!($mat4::IDENTITY, rotz_inv * rotz);

            let trans = $mat4::from_translation($newvec3(1.0, 2.0, 3.0));
            let trans_inv = trans.inverse();
            // assert_ne!(None, trans_inv);
            // let trans_inv = trans_inv.unwrap();
            assert_approx_eq!($mat4::IDENTITY, trans * trans_inv);
            assert_approx_eq!($mat4::IDENTITY, trans_inv * trans);

            let scale = $mat4::from_scale($newvec3(4.0, 5.0, 6.0));
            let scale_inv = scale.inverse();
            // assert_ne!(None, scale_inv);
            // let scale_inv = scale_inv.unwrap();
            assert_approx_eq!($mat4::IDENTITY, scale * scale_inv);
            assert_approx_eq!($mat4::IDENTITY, scale_inv * scale);

            let m = scale * rotz * trans;
            let m_inv = m.inverse();
            // assert_ne!(None, m_inv);
            // let m_inv = m_inv.unwrap();
            assert_approx_eq!($mat4::IDENTITY, m * m_inv, 1.0e-5);
            assert_approx_eq!($mat4::IDENTITY, m_inv * m, 1.0e-5);
            assert_approx_eq!(m_inv, trans_inv * rotz_inv * scale_inv, 1.0e-6);

            // Make sure we can invert a "random" matrix:
            let m = $mat4::from_cols(
                $newvec4(1.0, -0.3, 1.0, 1.0),
                $newvec4(0.5, 0.6, 0.7, 0.8),
                $newvec4(-0.9, -0.3, 0.0, 12.0),
                $newvec4(0.13, 0.14, 0.15, 0.16),
            );
            let m_inv = m.inverse();
            assert_approx_eq!($mat4::IDENTITY, m * m_inv, 1.0e-5);
            assert_approx_eq!($mat4::IDENTITY, m_inv * m, 1.0e-5);

            should_glam_assert!({ $mat4::ZERO.inverse() });
        });

        glam_test!(test_mat4_decompose, {
            // identity
            let (out_scale, out_rotation, out_translation) =
                $mat4::IDENTITY.to_scale_rotation_translation();
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
                $mat4::from_scale_rotation_translation(in_scale, in_rotation, in_translation);
            let (out_scale, out_rotation, out_translation) = in_mat.to_scale_rotation_translation();
            assert_approx_eq!(in_scale, out_scale, 1e-6);
            // out_rotation is different but produces the same matrix
            // assert_approx_eq!(in_rotation, out_rotation);
            assert_approx_eq!(in_translation, out_translation);
            assert_approx_eq!(
                in_mat,
                $mat4::from_scale_rotation_translation(out_scale, out_rotation, out_translation),
                1e-6
            );

            // positive scale
            let in_scale = $vec3::new(1.0, 2.0, 4.0);
            let in_mat =
                $mat4::from_scale_rotation_translation(in_scale, in_rotation, in_translation);
            let (out_scale, out_rotation, out_translation) = in_mat.to_scale_rotation_translation();
            assert_approx_eq!(in_scale, out_scale, 1e-6);
            // out_rotation is different but produces the same matrix
            // assert_approx_eq!(in_rotation, out_rotation);
            assert_approx_eq!(in_translation, out_translation);
            assert_approx_eq!(
                in_mat,
                $mat4::from_scale_rotation_translation(out_scale, out_rotation, out_translation),
                1e-5
            );

            // negative scale
            let in_scale = $vec3::new(-4.0, 1.0, 2.0);
            let in_mat =
                $mat4::from_scale_rotation_translation(in_scale, in_rotation, in_translation);
            let (out_scale, out_rotation, out_translation) = in_mat.to_scale_rotation_translation();
            assert_approx_eq!(in_scale, out_scale, 1e-6);
            // out_rotation is different but produces the same matrix
            // assert_approx_eq!(in_rotation, out_rotation);
            assert_approx_eq!(in_translation, out_translation);
            assert_approx_eq!(
                in_mat,
                $mat4::from_scale_rotation_translation(out_scale, out_rotation, out_translation),
                1e-5
            );

            // negative scale
            let in_scale = $vec3::new(4.0, -1.0, -2.0);
            let in_mat =
                $mat4::from_scale_rotation_translation(in_scale, in_rotation, in_translation);
            let (out_scale, out_rotation, out_translation) = in_mat.to_scale_rotation_translation();
            // out_scale and out_rotation are different but they produce the same matrix
            // assert_approx_eq!(in_scale, out_scale, 1e-6);
            // assert_approx_eq!(in_rotation, out_rotation);
            assert_approx_eq!(in_translation, out_translation);
            assert_approx_eq!(
                in_mat,
                $mat4::from_scale_rotation_translation(out_scale, out_rotation, out_translation),
                1e-6
            );

            should_glam_assert!({
                $mat4::from_scale_rotation_translation(
                    $vec3::ONE,
                    $quat::from_xyzw(0.0, 0.0, 0.0, 0.0),
                    $vec3::ZERO,
                )
            });
            should_glam_assert!({
                $mat4::from_rotation_translation($quat::from_xyzw(0.0, 0.0, 0.0, 0.0), $vec3::ZERO)
            });
            // TODO: should check scale
            // should_glam_assert!({ $mat4::from_scale_rotation_translation($vec3::ZERO, $quat::IDENTITY, $vec3::ZERO) });
            should_glam_assert!({ $mat4::ZERO.to_scale_rotation_translation() });
        });

        glam_test!(test_mat4_look_at, {
            let eye = $vec3::new(0.0, 0.0, -5.0);
            let center = $vec3::new(0.0, 0.0, 0.0);
            let up = $vec3::new(1.0, 0.0, 0.0);

            let point = $vec3::new(1.0, 0.0, 0.0);

            let lh = $mat4::look_at_lh(eye, center, up);
            let rh = $mat4::look_at_rh(eye, center, up);
            assert_approx_eq!(lh.transform_point3(point), $vec3::new(0.0, 1.0, 5.0));
            assert_approx_eq!(rh.transform_point3(point), $vec3::new(0.0, 1.0, -5.0));

            let dir = center - eye;
            let lh = $mat4::look_to_lh(eye, dir, up);
            let rh = $mat4::look_to_rh(eye, dir, up);
            assert_approx_eq!(lh.transform_point3(point), $vec3::new(0.0, 1.0, 5.0));
            assert_approx_eq!(rh.transform_point3(point), $vec3::new(0.0, 1.0, -5.0));

            should_glam_assert!({ $mat4::look_at_lh($vec3::ONE, $vec3::ZERO, $vec3::ZERO) });
            should_glam_assert!({ $mat4::look_at_rh($vec3::ONE, $vec3::ZERO, $vec3::ZERO) });
        });

        glam_test!(test_mat4_perspective_gl_rh, {
            let projection = $mat4::perspective_rh_gl($t::to_radians(90.0), 2.0, 5.0, 15.0);

            let original = $vec3::new(5.0, 5.0, -15.0);
            let projected = projection * original.extend(1.0);
            assert_approx_eq!($vec4::new(2.5, 5.0, 15.0, 15.0), projected);

            let original = $vec3::new(5.0, 5.0, -5.0);
            let projected = projection * original.extend(1.0);
            assert_approx_eq!($vec4::new(2.5, 5.0, -5.0, 5.0), projected);
        });

        glam_test!(test_mat4_perspective_lh, {
            let projection = $mat4::perspective_lh($t::to_radians(90.0), 2.0, 5.0, 15.0);

            let original = $vec3::new(5.0, 5.0, 15.0);
            let projected = projection * original.extend(1.0);
            assert_approx_eq!($vec4::new(2.5, 5.0, 15.0, 15.0), projected, 1e-6);

            let original = $vec3::new(5.0, 5.0, 5.0);
            let projected = projection * original.extend(1.0);
            assert_approx_eq!($vec4::new(2.5, 5.0, 0.0, 5.0), projected, 1e-6);

            should_glam_assert!({ $mat4::perspective_lh(0.0, 1.0, 1.0, 0.0) });
            should_glam_assert!({ $mat4::perspective_lh(0.0, 1.0, 0.0, 1.0) });
        });

        glam_test!(test_mat4_perspective_infinite_lh, {
            let projection = $mat4::perspective_infinite_lh($t::to_radians(90.0), 2.0, 5.0);

            let original = $vec3::new(5.0, 5.0, 15.0);
            let projected = projection * original.extend(1.0);
            assert_approx_eq!($vec4::new(2.5, 5.0, 10.0, 15.0), projected, 1e-6);

            let original = $vec3::new(5.0, 5.0, 5.0);
            let projected = projection * original.extend(1.0);
            assert_approx_eq!($vec4::new(2.5, 5.0, 0.0, 5.0), projected, 1e-6);

            should_glam_assert!({ $mat4::perspective_infinite_lh(0.0, 1.0, 0.0) });
        });

        glam_test!(test_mat4_perspective_infinite_reverse_lh, {
            let projection = $mat4::perspective_infinite_reverse_lh($t::to_radians(90.0), 2.0, 5.0);

            let original = $vec3::new(5.0, 5.0, 15.0);
            let projected = projection * original.extend(1.0);
            assert_approx_eq!($vec4::new(2.5, 5.0, 5.0, 15.0), projected, 1e-6);

            let original = $vec3::new(5.0, 5.0, 5.0);
            let projected = projection * original.extend(1.0);
            assert_approx_eq!($vec4::new(2.5, 5.0, 5.0, 5.0), projected, 1e-6);

            should_glam_assert!({ $mat4::perspective_infinite_reverse_lh(0.0, 1.0, 0.0) });
        });

        glam_test!(test_mat4_perspective_rh, {
            let projection = $mat4::perspective_rh($t::to_radians(90.0), 2.0, 5.0, 15.0);

            let original = $vec3::new(5.0, 5.0, 15.0);
            let projected = projection * original.extend(1.0);
            assert_approx_eq!($vec4::new(2.5, 5.0, -30.0, -15.0), projected, 1e-6);

            let original = $vec3::new(5.0, 5.0, 5.0);
            let projected = projection * original.extend(1.0);
            assert_approx_eq!($vec4::new(2.5, 5.0, -15.0, -5.0), projected, 1e-6);

            should_glam_assert!({ $mat4::perspective_rh(0.0, 1.0, 1.0, 0.0) });
            should_glam_assert!({ $mat4::perspective_rh(0.0, 1.0, 0.0, 1.0) });
        });

        glam_test!(test_mat4_perspective_infinite_rh, {
            let projection = $mat4::perspective_infinite_rh($t::to_radians(90.0), 2.0, 5.0);

            let original = $vec3::new(5.0, 5.0, 15.0);
            let projected = projection * original.extend(1.0);
            assert_approx_eq!($vec4::new(2.5, 5.0, -20.0, -15.0), projected);

            let original = $vec3::new(5.0, 5.0, 5.0);
            let projected = projection * original.extend(1.0);
            assert_approx_eq!($vec4::new(2.5, 5.0, -10.0, -5.0), projected);

            should_glam_assert!({ $mat4::perspective_infinite_rh(0.0, 1.0, 0.0) });
        });

        glam_test!(test_mat4_perspective_infinite_reverse_rh, {
            let projection = $mat4::perspective_infinite_reverse_rh($t::to_radians(90.0), 2.0, 5.0);

            let original = $vec3::new(5.0, 5.0, 15.0);
            let projected = projection * original.extend(1.0);
            assert_approx_eq!($vec4::new(2.5, 5.0, 5.0, -15.0), projected);

            let original = $vec3::new(5.0, 5.0, 5.0);
            let projected = projection * original.extend(1.0);
            assert_approx_eq!($vec4::new(2.5, 5.0, 5.0, -5.0), projected);

            should_glam_assert!({ $mat4::perspective_infinite_reverse_rh(0.0, 1.0, 0.0) });
        });

        glam_test!(test_mat4_orthographic_gl_rh, {
            let projection = $mat4::orthographic_rh_gl(-10.0, 10.0, -5.0, 5.0, 0.0, -10.0);
            let original = $vec4::new(5.0, 5.0, -5.0, 1.0);
            let projected = projection.mul_vec4(original);
            assert_approx_eq!(projected, $vec4::new(0.5, 1.0, -2.0, 1.0));
        });

        glam_test!(test_mat4_orthographic_rh, {
            let projection = $mat4::orthographic_rh(-10.0, 10.0, -5.0, 5.0, -10.0, 10.0);
            let original = $vec4::new(5.0, 5.0, -5.0, 1.0);
            let projected = projection.mul_vec4(original);
            assert_approx_eq!(projected, $vec4::new(0.5, 1.0, 0.75, 1.0));

            let original = $vec4::new(5.0, 5.0, 5.0, 1.0);
            let projected = projection.mul_vec4(original);
            assert_approx_eq!(projected, $vec4::new(0.5, 1.0, 0.25, 1.0));
        });

        glam_test!(test_mat4_orthographic_lh, {
            let projection = $mat4::orthographic_lh(-10.0, 10.0, -5.0, 5.0, -10.0, 10.0);
            let original = $vec4::new(5.0, 5.0, -5.0, 1.0);
            let projected = projection.mul_vec4(original);
            assert_approx_eq!(projected, $vec4::new(0.5, 1.0, 0.25, 1.0));

            let original = $vec4::new(5.0, 5.0, 5.0, 1.0);
            let projected = projection.mul_vec4(original);
            assert_approx_eq!(projected, $vec4::new(0.5, 1.0, 0.75, 1.0));
        });

        glam_test!(test_mat4_ops, {
            let m0 = $mat4::from_cols_array_2d(&ARRAY4X4);
            let m0x2 = $mat4::from_cols_array_2d(&[
                [2.0, 4.0, 6.0, 8.0],
                [10.0, 12.0, 14.0, 16.0],
                [18.0, 20.0, 22.0, 24.0],
                [26.0, 28.0, 30.0, 32.0],
            ]);
            let m0_neg = $mat4::from_cols_array_2d(&[
                [-1.0, -2.0, -3.0, -4.0],
                [-5.0, -6.0, -7.0, -8.0],
                [-9.0, -10.0, -11.0, -12.0],
                [-13.0, -14.0, -15.0, -16.0],
            ]);
            assert_eq!(m0x2, m0 * 2.0);
            assert_eq!(m0x2, 2.0 * m0);
            assert_eq!(m0, m0x2 / 2.0);
            assert_eq!(m0, 2.0 / m0x2);
            assert_eq!(m0x2, m0 + m0);
            assert_eq!($mat4::ZERO, m0 - m0);
            assert_eq!(m0_neg, -m0);
            assert_approx_eq!(m0, m0 * $mat4::IDENTITY);
            assert_approx_eq!(m0, $mat4::IDENTITY * m0);

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
            assert_eq!($mat4::ZERO, m1);

            let mut m1 = $mat4::IDENTITY;
            m1 *= m0;
            assert_approx_eq!(m0, m1);
        });

        glam_test!(test_mat4_fmt, {
            let a = $mat4::from_cols_array_2d(&ARRAY4X4);
            assert_eq!(
                format!("{}", a),
                "[[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12], [13, 14, 15, 16]]"
            );
            assert_eq!(
                format!("{:.1}", a),
                "[[1.0, 2.0, 3.0, 4.0], [5.0, 6.0, 7.0, 8.0], [9.0, 10.0, 11.0, 12.0], [13.0, 14.0, 15.0, 16.0]]"
            );
        });

        glam_test!(test_mat4_to_from_slice, {
            let m = $mat4::from_cols_slice(&ARRAY1X16);
            assert_eq!($mat4::from_cols_array(&ARRAY1X16), m);
            let mut out: [$t; 16] = Default::default();
            m.write_cols_to_slice(&mut out);
            assert_eq!(ARRAY1X16, out);

            should_panic!({ $mat4::from_cols_slice(&[0.0; 15]) });
            should_panic!({ $mat4::IDENTITY.write_cols_to_slice(&mut [0.0; 15]) });
        });

        glam_test!(test_sum, {
            let id = $mat4::IDENTITY;
            assert_eq!([id, id].iter().sum::<$mat4>(), id + id);
            assert_eq!([id, id].into_iter().sum::<$mat4>(), id + id);
        });

        glam_test!(test_product, {
            let two = $mat4::IDENTITY + $mat4::IDENTITY;
            assert_eq!([two, two].iter().product::<$mat4>(), two * two);
            assert_eq!([two, two].into_iter().product::<$mat4>(), two * two);
        });

        glam_test!(test_mat4_is_finite, {
            assert!($mat4::IDENTITY.is_finite());
            assert!(!($mat4::IDENTITY * $t::INFINITY).is_finite());
            assert!(!($mat4::IDENTITY * $t::NEG_INFINITY).is_finite());
            assert!(!($mat4::IDENTITY * $t::NAN).is_finite());
        });

        glam_test!(test_mat4_abs, {
            let neg = $mat4::IDENTITY * -1.0;
            assert_eq!(neg.abs(), $mat4::IDENTITY);

            let partial_neg = $mat4::from_cols_array_2d(&[
                [1.0, -2.0, 3.0, -4.0],
                [-5.0, 6.0, -7.0, 8.0],
                [-9.0, 10.0, 11.0, -12.0],
                [13.0, -14.0, -15.0, 16.0],
            ]);

            assert_eq!(
                partial_neg.abs(),
                $mat4::from_cols_array_2d(&[
                    [1.0, 2.0, 3.0, 4.0],
                    [5.0, 6.0, 7.0, 8.0],
                    [9.0, 10.0, 11.0, 12.0],
                    [13.0, 14.0, 15.0, 16.0],
                ])
            );
        });
    };
}

macro_rules! impl_as_ref_tests {
    ($mat:ident) => {
        glam_test!(test_as_ref, {
            let m = $mat::from_cols_array_2d(&ARRAY4X4);
            assert_eq!(ARRAY1X16, *m.as_ref());
        });
        glam_test!(test_as_mut, {
            let mut m = $mat::ZERO;
            *m.as_mut() = ARRAY1X16;
            assert_eq!($mat::from_cols_array_2d(&ARRAY4X4), m);
        });
    };
}

mod mat4 {
    use super::support::deg;
    use glam::{mat4, swizzles::*, vec3, vec4, Mat3, Mat4, Quat, Vec3, Vec4};

    glam_test!(test_align, {
        use std::mem;
        assert_eq!(mem::align_of::<Vec4>(), mem::align_of::<Mat4>());
        assert_eq!(64, mem::size_of::<Mat4>());
    });

    glam_test!(test_from_mat3a, {
        use glam::Mat3A;
        let m3 = Mat3A::from_cols_array_2d(&ARRAY3X3);
        let m4 = Mat4::from_mat3a(m3);
        assert_eq!(
            Mat4::from_cols_array_2d(&[
                [1.0, 2.0, 3.0, 0.0],
                [4.0, 5.0, 6.0, 0.0],
                [7.0, 8.0, 9.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ]),
            m4
        );
    });

    glam_test!(test_transform_vec3a, {
        use glam::Vec3A;
        let m = Mat4::from_axis_angle(Vec3::Z, deg(90.0));
        let result3 = m.transform_vector3a(Vec3A::Y);
        assert_approx_eq!(Vec3A::new(-1.0, 0.0, 0.0), result3);

        let m = Mat4::from_scale_rotation_translation(
            Vec3::new(0.5, 1.5, 2.0),
            Quat::from_rotation_x(deg(90.0)),
            Vec3::new(1.0, 2.0, 3.0),
        );
        let result3 = m.transform_vector3a(Vec3A::Y);
        assert_approx_eq!(Vec3A::new(0.0, 0.0, 1.5), result3, 1.0e-6);

        let result3 = m.transform_point3a(Vec3A::Y);
        assert_approx_eq!(Vec3A::new(1.0, 2.0, 4.5), result3, 1.0e-6);

        let m = Mat4::from_cols(
            vec4(8.0, 0.0, 0.0, 0.0),
            vec4(0.0, 4.0, 0.0, 0.0),
            vec4(0.0, 0.0, 2.0, 2.0),
            vec4(0.0, 0.0, 0.0, 0.0),
        );
        assert_approx_eq!(
            Vec3A::new(4.0, 2.0, 1.0),
            m.project_point3a(Vec3A::new(2.0, 2.0, 2.0))
        );
    });

    glam_test!(test_as, {
        use glam::DMat4;
        assert_eq!(
            DMat4::from_cols_array_2d(&[
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 10.0, 11.0, 12.0],
                [13.0, 14.0, 15.0, 16.0]
            ]),
            Mat4::from_cols_array_2d(&[
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 10.0, 11.0, 12.0],
                [13.0, 14.0, 15.0, 16.0]
            ])
            .as_dmat4()
        );
        assert_eq!(
            Mat4::from_cols_array_2d(&[
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 10.0, 11.0, 12.0],
                [13.0, 14.0, 15.0, 16.0]
            ]),
            DMat4::from_cols_array_2d(&[
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 10.0, 11.0, 12.0],
                [13.0, 14.0, 15.0, 16.0]
            ])
            .as_mat4()
        );
    });

    impl_mat4_tests!(f32, mat4, vec4, vec3, Mat4, Mat3, Quat, Vec4, Vec3);
    impl_as_ref_tests!(Mat4);
}

mod dmat4 {
    use super::support::deg;
    use glam::{dmat4, dvec3, dvec4, swizzles::*, DMat3, DMat4, DQuat, DVec3, DVec4};

    glam_test!(test_align, {
        use std::mem;
        assert_eq!(mem::align_of::<DVec4>(), mem::align_of::<DMat4>());
        assert_eq!(128, mem::size_of::<DMat4>());
    });

    impl_mat4_tests!(f64, dmat4, dvec4, dvec3, DMat4, DMat3, DQuat, DVec4, DVec3);
    impl_as_ref_tests!(DMat4);
}
