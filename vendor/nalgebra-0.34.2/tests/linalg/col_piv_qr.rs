#[cfg_attr(rustfmt, rustfmt_skip)]

use na::Matrix4;

#[test]
fn col_piv_qr() {
    let m = Matrix4::new(
        1.0, -1.0, 2.0, 1.0, -1.0, 3.0, -1.0, -1.0, 3.0, -5.0, 5.0, 3.0, 1.0, 2.0, 1.0, -2.0,
    );
    let col_piv_qr = m.col_piv_qr();
    assert!(relative_eq!(
        col_piv_qr.determinant(),
        0.0,
        epsilon = 1.0e-7
    ));

    let (q, r, p) = col_piv_qr.unpack();

    let mut qr = q * r;
    p.inv_permute_columns(&mut qr);

    assert!(relative_eq!(m, qr, epsilon = 1.0e-7));
}

#[cfg(feature = "proptest-support")]
mod proptest_tests {
    macro_rules! gen_tests(
        ($module: ident, $scalar: expr ,$scalar_type: ty) => {
            mod $module {
                use na::{DMatrix, DVector, Matrix4x3, Vector4};
                use std::cmp;

                #[allow(unused_imports)]
                use crate::core::helper::{RandComplex, RandScalar};
                use crate::proptest::*;
                use proptest::{prop_assert, proptest};

                proptest! {
                    #[test]
                    fn col_piv_qr(m in dmatrix_($scalar)) {
                        let col_piv_qr = m.clone().col_piv_qr();
                        let (q, r, p) = col_piv_qr.unpack();
                        let mut qr = &q * &r;
                        p.inv_permute_columns(&mut qr);

                        prop_assert!(relative_eq!(m, &qr, epsilon = 1.0e-7));
                        prop_assert!(q.is_orthogonal(1.0e-7));
                    }

                    #[test]
                    fn col_piv_qr_static_5_3(m in matrix5x3_($scalar)) {
                        let col_piv_qr = m.col_piv_qr();
                        let (q, r, p) = col_piv_qr.unpack();
                        let mut qr = q * r;
                        p.inv_permute_columns(&mut qr);

                        prop_assert!(relative_eq!(m, qr, epsilon = 1.0e-7));
                        prop_assert!(q.is_orthogonal(1.0e-7));
                    }

                    #[test]
                    fn col_piv_qr_static_3_5(m in matrix3x5_($scalar)) {
                        let col_piv_qr = m.col_piv_qr();
                        let (q, r, p) = col_piv_qr.unpack();
                        let mut qr = q * r;
                        p.inv_permute_columns(&mut qr);

                        prop_assert!(relative_eq!(m, qr, epsilon = 1.0e-7));
                        prop_assert!(q.is_orthogonal(1.0e-7));
                    }

                    #[test]
                    fn col_piv_qr_static_square(m in matrix4_($scalar)) {
                        let col_piv_qr = m.col_piv_qr();
                        let (q, r, p) = col_piv_qr.unpack();
                        let mut qr = q * r;
                        p.inv_permute_columns(&mut qr);

                        prop_assert!(relative_eq!(m, qr, epsilon = 1.0e-7));
                        prop_assert!(q.is_orthogonal(1.0e-7));
                    }

                    #[test]
                    fn col_piv_qr_solve(n in PROPTEST_MATRIX_DIM, nb in PROPTEST_MATRIX_DIM) {
                        if n != 0 && nb != 0 {
                            let n  = cmp::min(n, 50);  // To avoid slowing down the test too much.
                            let nb = cmp::min(nb, 50); // To avoid slowing down the test too much.
                            let m  = DMatrix::<$scalar_type>::new_random(n, n).map(|e| e.0);

                            let col_piv_qr = m.clone().col_piv_qr();
                            let b1 = DVector::<$scalar_type>::new_random(n).map(|e| e.0);
                            let b2 = DMatrix::<$scalar_type>::new_random(n, nb).map(|e| e.0);

                            if col_piv_qr.is_invertible() {
                                let sol1 = col_piv_qr.solve(&b1).unwrap();
                                let sol2 = col_piv_qr.solve(&b2).unwrap();

                                prop_assert!(relative_eq!(&m * sol1, b1, epsilon = 1.0e-6));
                                prop_assert!(relative_eq!(&m * sol2, b2, epsilon = 1.0e-6));
                            }
                        }
                    }

                    #[test]
                    fn col_piv_qr_solve_static(m in matrix4_($scalar)) {
                         let col_piv_qr = m.col_piv_qr();
                         let b1 = Vector4::<$scalar_type>::new_random().map(|e| e.0);
                         let b2 = Matrix4x3::<$scalar_type>::new_random().map(|e| e.0);

                         if col_piv_qr.is_invertible() {
                             let sol1 = col_piv_qr.solve(&b1).unwrap();
                             let sol2 = col_piv_qr.solve(&b2).unwrap();

                             prop_assert!(relative_eq!(m * sol1, b1, epsilon = 1.0e-6));
                             prop_assert!(relative_eq!(m * sol2, b2, epsilon = 1.0e-6));
                         }
                    }

                    #[test]
                    fn col_piv_qr_inverse(n in PROPTEST_MATRIX_DIM) {
                        let n = cmp::max(1, cmp::min(n, 15)); // To avoid slowing down the test too much.
                        let m = DMatrix::<$scalar_type>::new_random(n, n).map(|e| e.0);

                        if let Some(m1) = m.clone().col_piv_qr().try_inverse() {
                            let id1 = &m  * &m1;
                            let id2 = &m1 * &m;

                            prop_assert!(id1.is_identity(1.0e-5));
                            prop_assert!(id2.is_identity(1.0e-5));
                        }
                    }

                    #[test]
                    fn col_piv_qr_inverse_static(m in matrix4_($scalar)) {
                        let col_piv_qr = m.col_piv_qr();

                        if let Some(m1) = col_piv_qr.try_inverse() {
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
