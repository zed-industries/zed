#[macro_use]
mod support;

macro_rules! impl_affine2_tests {
    ($t:ident, $affine2:ident, $vec2:ident) => {
        const MATRIX2D: [[$t; 2]; 3] = [[1.0, 2.0], [3.0, 4.0], [5.0, 6.0]];

        use core::$t::NAN;
        use core::$t::NEG_INFINITY;

        glam_test!(test_affine2_identity, {
            assert_eq!($affine2::IDENTITY, $affine2::IDENTITY * $affine2::IDENTITY);
            assert_eq!($affine2::IDENTITY, $affine2::default());
        });

        glam_test!(test_affine2_zero, {
            assert_eq!(
                $affine2::ZERO.transform_point2($vec2::new(1., 2.)),
                $vec2::ZERO
            );
        });

        glam_test!(test_affine2_nan, {
            assert!($affine2::NAN.is_nan());
            assert!(!$affine2::NAN.is_finite());
        });

        glam_test!(test_affine2_translation, {
            let translate = $affine2::from_translation($vec2::new(1.0, 2.0));
            assert_eq!(translate.translation, $vec2::new(1.0, 2.0).into());
            assert_eq!(
                translate.transform_point2($vec2::new(2.0, 3.0)),
                $vec2::new(3.0, 5.0),
            );
        });

        glam_test!(test_affine2_mul, {
            let m = $affine2::from_angle(deg(90.0));
            let result3 = m.transform_vector2($vec2::Y);
            assert_approx_eq!($vec2::new(-1.0, 0.0), result3);

            let m = $affine2::from_scale_angle_translation(
                $vec2::new(0.5, 1.5),
                deg(90.0),
                $vec2::new(1.0, 2.0),
            );
            let result3 = m.transform_vector2($vec2::Y);
            assert_approx_eq!($vec2::new(-1.5, 0.0), result3, 1.0e-6);

            let result3 = m.transform_point2($vec2::Y);
            assert_approx_eq!($vec2::new(-0.5, 2.0), result3, 1.0e-6);
        });

        glam_test!(test_from_scale, {
            let m = $affine2::from_scale($vec2::new(2.0, 4.0));
            assert_approx_eq!(
                m.transform_point2($vec2::new(1.0, 1.0)),
                $vec2::new(2.0, 4.0)
            );
        });

        glam_test!(test_affine2_inverse, {
            let inv = $affine2::IDENTITY.inverse();
            assert_approx_eq!($affine2::IDENTITY, inv);

            let rot = $affine2::from_angle(deg(90.0));
            let rot_inv = rot.inverse();
            assert_approx_eq!($affine2::IDENTITY, rot * rot_inv);
            assert_approx_eq!($affine2::IDENTITY, rot_inv * rot);

            let trans = $affine2::from_translation($vec2::new(1.0, 2.0));
            let trans_inv = trans.inverse();
            assert_approx_eq!($affine2::IDENTITY, trans * trans_inv);
            assert_approx_eq!($affine2::IDENTITY, trans_inv * trans);

            let scale = $affine2::from_scale($vec2::new(4.0, 5.0));
            let scale_inv = scale.inverse();
            assert_approx_eq!($affine2::IDENTITY, scale * scale_inv);
            assert_approx_eq!($affine2::IDENTITY, scale_inv * scale);

            let m = scale * rot * trans;
            let m_inv = m.inverse();
            assert_approx_eq!($affine2::IDENTITY, m * m_inv, 1.0e-5);
            assert_approx_eq!($affine2::IDENTITY, m_inv * m, 1.0e-5);
            assert_approx_eq!(m_inv, trans_inv * rot_inv * scale_inv, 1.0e-6);

            // Make sure we can invert a shear matrix:
            let m = $affine2::from_angle(0.5)
                * $affine2::from_scale($vec2::new(1.0, 0.5))
                * $affine2::from_angle(-0.5);
            let m_inv = m.inverse();
            assert_approx_eq!($affine2::IDENTITY, m * m_inv, 1.0e-5);
            assert_approx_eq!($affine2::IDENTITY, m_inv * m, 1.0e-5);

            should_glam_assert!({ $affine2::ZERO.inverse() });
        });

        glam_test!(test_affine2_ops, {
            let m0 = $affine2::from_cols_array_2d(&MATRIX2D);
            let m0x2 = $affine2::from_cols_array_2d(&[[2.0, 4.0], [6.0, 8.0], [10.0, 12.0]]);
            assert_eq!(m0x2, m0 * 2.0);
            assert_eq!(m0x2, 2.0 * m0);
            assert_eq!(m0x2, m0 + m0);
            assert_eq!($affine2::ZERO, m0 - m0);
            assert_approx_eq!(m0, m0 * $affine2::IDENTITY);
            assert_approx_eq!(m0, $affine2::IDENTITY * m0);
        });

        glam_test!(test_affine2_fmt, {
            let a = $affine2::from_cols_array_2d(&MATRIX2D);
            assert_eq!(format!("{}", a), "[[1, 2], [3, 4], [5, 6]]");
        });

        glam_test!(test_affine2_to_from_slice, {
            const MATRIX1D: [$t; 6] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
            let m = $affine2::from_cols_slice(&MATRIX1D);
            assert_eq!($affine2::from_cols_array(&MATRIX1D), m);
            assert_eq!(MATRIX1D, m.to_cols_array());
            assert_eq!(MATRIX2D, m.to_cols_array_2d());
            let mut out: [$t; 6] = Default::default();
            m.write_cols_to_slice(&mut out);
            assert_eq!(MATRIX1D, out);
            assert_eq!(
                m,
                $affine2::from_cols(MATRIX2D[0].into(), MATRIX2D[1].into(), MATRIX2D[2].into())
            );

            should_panic!({ $affine2::from_cols_slice(&[0.0; 5]) });
            should_panic!({ $affine2::IDENTITY.write_cols_to_slice(&mut [0.0; 5]) });
        });

        glam_test!(test_product, {
            let ident = $affine2::IDENTITY;
            assert_eq!(
                vec![ident, ident].iter().product::<$affine2>(),
                ident * ident
            );
        });

        glam_test!(test_affine2_is_finite, {
            assert!($affine2::from_scale($vec2::new(1.0, 1.0)).is_finite());
            assert!($affine2::from_scale($vec2::new(0.0, 1.0)).is_finite());
            assert!(!$affine2::from_scale($vec2::new(1.0, NAN)).is_finite());
            assert!(!$affine2::from_scale($vec2::new(1.0, NEG_INFINITY)).is_finite());
        });
    };
}

mod affine2 {
    use super::support::{deg, FloatCompare};
    use glam::{Affine2, Vec2};

    impl FloatCompare for Affine2 {
        #[inline]
        fn approx_eq(&self, other: &Self, max_abs_diff: f32) -> bool {
            self.abs_diff_eq(*other, max_abs_diff)
        }
        #[inline]
        fn abs_diff(&self, other: &Self) -> Self {
            Self {
                matrix2: self.matrix2.abs_diff(&other.matrix2),
                translation: self.translation.abs_diff(&other.translation),
            }
        }
    }

    glam_test!(test_align, {
        use std::mem;
        if cfg!(not(feature = "scalar-math")) {
            assert_eq!(32, mem::size_of::<Affine2>());
            assert_eq!(16, mem::align_of::<Affine2>());
        } else if cfg!(feature = "cuda") {
            assert_eq!(24, mem::size_of::<Affine2>());
            assert_eq!(8, mem::align_of::<Affine2>());
        } else {
            assert_eq!(24, mem::size_of::<Affine2>());
            assert_eq!(4, mem::align_of::<Affine2>());
        }
    });

    impl_affine2_tests!(f32, Affine2, Vec2);
}

mod daffine2 {
    use super::support::{deg, FloatCompare};
    use glam::{DAffine2, DVec2};

    impl FloatCompare for DAffine2 {
        #[inline]
        fn approx_eq(&self, other: &Self, max_abs_diff: f32) -> bool {
            self.abs_diff_eq(*other, max_abs_diff as f64)
        }
        #[inline]
        fn abs_diff(&self, other: &Self) -> Self {
            Self {
                matrix2: self.matrix2.abs_diff(&other.matrix2),
                translation: self.translation.abs_diff(&other.translation),
            }
        }
    }

    #[cfg(not(feature = "cuda"))]
    glam_test!(test_align, {
        use std::mem;
        assert_eq!(48, mem::size_of::<DAffine2>());
        assert_eq!(mem::align_of::<f64>(), mem::align_of::<DAffine2>());
    });

    #[cfg(feature = "cuda")]
    glam_test!(test_align, {
        use std::mem;
        assert_eq!(48, mem::size_of::<DAffine2>());
        assert_eq!(16, mem::align_of::<DAffine2>());
    });

    impl_affine2_tests!(f64, DAffine2, DVec2);
}
