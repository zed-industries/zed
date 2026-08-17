use na::{Matrix2, Vector3, geometry::Quaternion};
use num_traits::{One, Zero};

#[test]
fn gemm_noncommutative() {
    type Qf64 = Quaternion<f64>;
    let i = Qf64::from_imag(Vector3::new(1.0, 0.0, 0.0));
    let j = Qf64::from_imag(Vector3::new(0.0, 1.0, 0.0));
    let k = Qf64::from_imag(Vector3::new(0.0, 0.0, 1.0));

    let m1 = Matrix2::new(k, Qf64::zero(), j, i);
    // this is the inverse of m1
    let m2 = Matrix2::new(-k, Qf64::zero(), Qf64::one(), -i);

    let mut res: Matrix2<Qf64> = Matrix2::zero();
    res.gemm(Qf64::one(), &m1, &m2, Qf64::zero());
    assert_eq!(res, Matrix2::identity());

    let mut res: Matrix2<Qf64> = Matrix2::identity();
    res.gemm(k, &m1, &m2, -k);
    assert_eq!(res, Matrix2::zero());
}

#[cfg(feature = "proptest-support")]
mod blas_proptest {
    use crate::proptest::{PROPTEST_F64, PROPTEST_MATRIX_DIM};
    use na::{DMatrix, DVector};
    use proptest::{prop_assert, proptest};

    proptest! {
        /*
         *
         * Symmetric operators.
         *
         */
        #[test]
        fn gemv_symm(n in PROPTEST_MATRIX_DIM, alpha in PROPTEST_F64, beta in PROPTEST_F64) {
            let a = DMatrix::<f64>::new_random(n, n);
            let a = &a * a.transpose();

            let x = DVector::new_random(n);
            let mut y1 = DVector::new_random(n);
            let mut y2 = y1.clone();

            y1.gemv(alpha, &a, &x, beta);
            y2.sygemv(alpha, &a.lower_triangle(), &x, beta);

            prop_assert!(relative_eq!(y1, y2, epsilon = 1.0e-10));

            y1.gemv(alpha, &a, &x, 0.0);
            y2.sygemv(alpha, &a.lower_triangle(), &x, 0.0);

            prop_assert!(relative_eq!(y1, y2, epsilon = 1.0e-10))
        }

        #[test]
        fn gemv_tr(n in PROPTEST_MATRIX_DIM, alpha in PROPTEST_F64, beta in PROPTEST_F64) {
            let a = DMatrix::<f64>::new_random(n, n);
            let x = DVector::new_random(n);
            let mut y1 = DVector::new_random(n);
            let mut y2 = y1.clone();

            y1.gemv(alpha, &a, &x, beta);
            y2.gemv_tr(alpha, &a.transpose(), &x, beta);

            prop_assert!(relative_eq!(y1, y2, epsilon = 1.0e-10));

            y1.gemv(alpha, &a, &x, 0.0);
            y2.gemv_tr(alpha, &a.transpose(), &x, 0.0);

            prop_assert!(relative_eq!(y1, y2, epsilon = 1.0e-10))
        }

        #[test]
        fn ger_symm(n in PROPTEST_MATRIX_DIM, alpha in PROPTEST_F64, beta in PROPTEST_F64) {
            let a = DMatrix::<f64>::new_random(n, n);
            let mut a1 = &a * a.transpose();
            let mut a2 = a1.lower_triangle();

            let x = DVector::new_random(n);
            let y = DVector::new_random(n);

            a1.ger(alpha, &x, &y, beta);
            a2.syger(alpha, &x, &y, beta);

            prop_assert!(relative_eq!(a1.lower_triangle(), a2));

            a1.ger(alpha, &x, &y, 0.0);
            a2.syger(alpha, &x, &y, 0.0);

            prop_assert!(relative_eq!(a1.lower_triangle(), a2))
        }

        #[test]
        fn quadform(n in PROPTEST_MATRIX_DIM, alpha in PROPTEST_F64, beta in PROPTEST_F64) {
            let rhs     = DMatrix::<f64>::new_random(6, n);
            let mid     = DMatrix::<f64>::new_random(6, 6);
            let mut res = DMatrix::new_random(n, n);

            let expected = &res * beta + rhs.transpose() * &mid * &rhs * alpha;

            res.quadform(alpha, &mid, &rhs, beta);

            prop_assert!(relative_eq!(res, expected, epsilon = 1.0e-7))
        }

        #[test]
        fn quadform_tr(n in PROPTEST_MATRIX_DIM, alpha in PROPTEST_F64, beta in PROPTEST_F64) {
            let lhs     = DMatrix::<f64>::new_random(6, n);
            let mid     = DMatrix::<f64>::new_random(n, n);
            let mut res = DMatrix::new_random(6, 6);

            let expected = &res * beta + &lhs * &mid * lhs.transpose() * alpha;

            res.quadform_tr(alpha, &lhs, &mid , beta);

            prop_assert!(relative_eq!(res, expected, epsilon = 1.0e-7))
        }
    }
}
