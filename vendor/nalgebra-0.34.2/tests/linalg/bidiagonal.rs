#[cfg(feature = "proptest-support")]
mod proptest_tests {
    macro_rules! gen_tests(
    ($module: ident, $scalar: expr) => {
        mod $module {
            #[allow(unused_imports)]
            use crate::core::helper::{RandScalar, RandComplex};

            use crate::proptest::*;
            use proptest::{prop_assert, proptest};

            proptest! {
                #[test]
                fn bidiagonal(m in dmatrix_($scalar)) {
                    let bidiagonal = m.clone().bidiagonalize();
                    let (u, d, v_t) = bidiagonal.unpack();

                    prop_assert!(relative_eq!(m, &u * d * &v_t, epsilon = 1.0e-7))
                }

                #[test]
                fn bidiagonal_static_5_3(m in matrix5x3_($scalar)) {
                    let bidiagonal = m.bidiagonalize();
                    let (u, d, v_t) = bidiagonal.unpack();

                    prop_assert!(relative_eq!(m, &u * d * &v_t, epsilon = 1.0e-7))
                }

                #[test]
                fn bidiagonal_static_3_5(m in matrix3x5_($scalar)) {
                    let bidiagonal = m.bidiagonalize();
                    let (u, d, v_t) = bidiagonal.unpack();

                    prop_assert!(relative_eq!(m, &u * d * &v_t, epsilon = 1.0e-7))
                }

                #[test]
                fn bidiagonal_static_square(m in matrix4_($scalar)) {
                    let bidiagonal = m.bidiagonalize();
                    let (u, d, v_t) = bidiagonal.unpack();

                    prop_assert!(relative_eq!(m, &u * d * &v_t, epsilon = 1.0e-7))
                }

                #[test]
                fn bidiagonal_static_square_2x2(m in matrix2_($scalar)) {
                    let bidiagonal = m.bidiagonalize();
                    let (u, d, v_t) = bidiagonal.unpack();

                    prop_assert!(relative_eq!(m, &u * d * &v_t, epsilon = 1.0e-7))
                }
            }
        }
    }
);

    gen_tests!(complex, complex_f64());
    gen_tests!(f64, PROPTEST_F64);
}

#[test]
fn bidiagonal_identity() {
    let m = na::DMatrix::<f64>::identity(10, 10);
    let bidiagonal = m.clone().bidiagonalize();
    let (u, d, v_t) = bidiagonal.unpack();
    assert_eq!(m, &u * d * &v_t);

    let m = na::DMatrix::<f64>::identity(10, 15);
    let bidiagonal = m.clone().bidiagonalize();
    let (u, d, v_t) = bidiagonal.unpack();
    assert_eq!(m, &u * d * &v_t);

    let m = na::DMatrix::<f64>::identity(15, 10);
    let bidiagonal = m.clone().bidiagonalize();
    let (u, d, v_t) = bidiagonal.unpack();
    assert_eq!(m, &u * d * &v_t);
}

#[test]
fn bidiagonal_regression_issue_1313() {
    let s = 6.123234e-16_f32;
    let mut m = nalgebra::dmatrix![
        10.0,   0.0, 0.0,  0.0, -10.0, 0.0, 0.0, 0.0;
        s,     10.0, 0.0, 10.0,     s, 0.0, 0.0, 0.0;
        20.0, -20.0, 0.0, 20.0,  20.0, 0.0, 0.0, 0.0;
    ];
    m.unscale_mut(m.camax());
    let bidiagonal = m.clone().bidiagonalize();
    let (u, d, v_t) = bidiagonal.unpack();
    let m2 = &u * d * &v_t;
    assert_relative_eq!(m, m2, epsilon = 1e-6);
}

#[test]
fn bidiagonal_regression_issue_1313_minimal() {
    let s = 6.123234e-17_f32;
    let m = nalgebra::dmatrix![
        1.0,   0.0, -1.0;
        s,     1.0,     s;
    ];
    let bidiagonal = m.clone().bidiagonalize();
    let (u, d, v_t) = bidiagonal.unpack();
    let m2 = &u * &d * &v_t;
    assert_relative_eq!(m, m2, epsilon = 1e-6);
}
