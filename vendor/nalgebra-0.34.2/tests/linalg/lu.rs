use na::Matrix3;

#[test]
#[rustfmt::skip]
fn lu_simple() {
    let m = Matrix3::new(
        2.0, -1.0,  0.0,
       -1.0,  2.0, -1.0,
        0.0, -1.0,  2.0);

    let lu = m.lu();
    assert_eq!(lu.determinant(), 4.0);

    let (p, l, u) = lu.unpack();

    let mut lu = l * u;
    p.inv_permute_rows(&mut lu);

    assert!(relative_eq!(m, lu, epsilon = 1.0e-7));
}

#[test]
#[rustfmt::skip]
fn lu_simple_with_pivot() {
    let m = Matrix3::new(
        0.0, -1.0,  2.0,
       -1.0,  2.0, -1.0,
        2.0, -1.0,  0.0);

    let lu = m.lu();
    assert_eq!(lu.determinant(), -4.0);

    let (p, l, u) = lu.unpack();

    let mut lu = l * u;
    p.inv_permute_rows(&mut lu);

    assert!(relative_eq!(m, lu, epsilon = 1.0e-7));
}

#[cfg(feature = "proptest-support")]
mod proptest_tests {
    macro_rules! gen_tests(
        ($module: ident, $scalar: expr, $scalar_type: ty) => {
            mod $module {
                use na::{DMatrix, Matrix4x3, DVector, Vector4};
                #[allow(unused_imports)]
                use crate::core::helper::{RandScalar, RandComplex};
                use crate::proptest::*;
                use proptest::{prop_assert, proptest};

                proptest! {
                    #[test]
                    fn lu(m in dmatrix_($scalar)) {
                        let lu = m.clone().lu();
                        let (p, l, u) = lu.unpack();
                        let mut lu = l * u;
                        p.inv_permute_rows(&mut lu);

                        prop_assert!(relative_eq!(m, lu, epsilon = 1.0e-7))
                    }

                    #[test]
                    fn lu_static_3_5(m in matrix3x5_($scalar)) {
                        let lu = m.lu();
                        let (p, l, u) = lu.unpack();
                        let mut lu = l * u;
                        p.inv_permute_rows(&mut lu);

                        prop_assert!(relative_eq!(m, lu, epsilon = 1.0e-7))
                    }

                    fn lu_static_5_3(m in matrix5x3_($scalar)) {
                        let lu = m.lu();
                        let (p, l, u) = lu.unpack();
                        let mut lu = l * u;
                        p.inv_permute_rows(&mut lu);

                        prop_assert!(relative_eq!(m, lu, epsilon = 1.0e-7));
                    }

                    #[test]
                    fn lu_static_square(m in matrix4_($scalar)) {
                        let lu = m.lu();
                        let (p, l, u) = lu.unpack();
                        let mut lu = l * u;
                        p.inv_permute_rows(&mut lu);

                        prop_assert!(relative_eq!(m, lu, epsilon = 1.0e-7));
                    }

                    #[test]
                    fn lu_solve(n in PROPTEST_MATRIX_DIM, nb in PROPTEST_MATRIX_DIM) {
                        let m  = DMatrix::<$scalar_type>::new_random(n, n).map(|e| e.0);

                        let lu = m.clone().lu();
                        let b1 = DVector::<$scalar_type>::new_random(n).map(|e| e.0);
                        let b2 = DMatrix::<$scalar_type>::new_random(n, nb).map(|e| e.0);

                        let sol1 = lu.solve(&b1);
                        let sol2 = lu.solve(&b2);

                        prop_assert!(sol1.is_none() || relative_eq!(&m * sol1.unwrap(), b1, epsilon = 1.0e-6));
                        prop_assert!(sol2.is_none() || relative_eq!(&m * sol2.unwrap(), b2, epsilon = 1.0e-6));
                    }

                    #[test]
                    fn lu_solve_static(m in matrix4_($scalar)) {
                         let lu = m.lu();
                         let b1 = Vector4::<$scalar_type>::new_random().map(|e| e.0);
                         let b2 = Matrix4x3::<$scalar_type>::new_random().map(|e| e.0);

                         let sol1 = lu.solve(&b1);
                         let sol2 = lu.solve(&b2);

                         prop_assert!(sol1.is_none() || relative_eq!(&m * sol1.unwrap(), b1, epsilon = 1.0e-6));
                         prop_assert!(sol2.is_none() || relative_eq!(&m * sol2.unwrap(), b2, epsilon = 1.0e-6));
                    }

                    #[test]
                    fn lu_inverse(n in PROPTEST_MATRIX_DIM) {
                        let m  = DMatrix::<$scalar_type>::new_random(n, n).map(|e| e.0);
                        let mut l = m.lower_triangle();
                        let mut u = m.upper_triangle();

                        // Ensure the matrix is well conditioned for inversion.
                        l.fill_diagonal(na::one());
                        u.fill_diagonal(na::one());
                        let m = l * u;

                        let m1  = m.clone().lu().try_inverse().unwrap();
                        let id1 = &m  * &m1;
                        let id2 = &m1 * &m;

                        prop_assert!(id1.is_identity(1.0e-5));
                        prop_assert!(id2.is_identity(1.0e-5));
                    }

                    #[test]
                    fn lu_inverse_static(m in matrix4_($scalar)) {
                        let lu  = m.lu();

                        if let Some(m1) = lu.try_inverse() {
                            let id1 = &m  * &m1;
                            let id2 = &m1 * &m;

                            prop_assert!(id1.is_identity(1.0e-5));
                            prop_assert!(id2.is_identity(1.0e-5));
                        }
                    }
                }
            }
        }
    );

    gen_tests!(complex, complex_f64(), RandComplex<f64>);
    gen_tests!(f64, PROPTEST_F64, RandScalar<f64>);
}
