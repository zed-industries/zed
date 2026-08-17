use na::{Complex, DMatrix, Matrix3};

#[cfg(feature = "proptest-support")]
mod proptest_tests {
    macro_rules! gen_tests(
        ($module: ident, $scalar: expr, $scalar_type: ty) => {
            mod $module {
                use na::DMatrix;
                #[allow(unused_imports)]
                use crate::core::helper::{RandScalar, RandComplex};
                use std::cmp;

                use crate::proptest::*;
                use proptest::{prop_assert, proptest};

                proptest! {
                    #[test]
                    fn symmetric_eigen(n in PROPTEST_MATRIX_DIM) {
                        let n      = cmp::max(1, cmp::min(n, 10));
                        let m      = DMatrix::<$scalar_type>::new_random(n, n).map(|e| e.0).hermitian_part();
                        let eig    = m.clone().symmetric_eigen();
                        let recomp = eig.recompose();

                        prop_assert!(relative_eq!(m.lower_triangle(), recomp.lower_triangle(), epsilon = 1.0e-5))
                    }

                    #[test]
                    fn symmetric_eigen_singular(n in PROPTEST_MATRIX_DIM) {
                        let n      = cmp::max(1, cmp::min(n, 10));
                        let mut m  = DMatrix::<$scalar_type>::new_random(n, n).map(|e| e.0).hermitian_part();
                        m.row_mut(n / 2).fill(na::zero());
                        m.column_mut(n / 2).fill(na::zero());
                        let eig    = m.clone().symmetric_eigen();
                        let recomp = eig.recompose();

                        prop_assert!(relative_eq!(m.lower_triangle(), recomp.lower_triangle(), epsilon = 1.0e-5))
                    }

                    #[test]
                    fn symmetric_eigen_static_square_4x4(m in matrix4_($scalar)) {
                        let m      = m.hermitian_part();
                        let eig    = m.symmetric_eigen();
                        let recomp = eig.recompose();

                        prop_assert!(relative_eq!(m.lower_triangle(), recomp.lower_triangle(), epsilon = 1.0e-5))
                    }

                    #[test]
                    fn symmetric_eigen_static_square_3x3(m in matrix3_($scalar)) {
                        let m      = m.hermitian_part();
                        let eig    = m.symmetric_eigen();
                        let recomp = eig.recompose();

                        prop_assert!(relative_eq!(m.lower_triangle(), recomp.lower_triangle(), epsilon = 1.0e-5))
                    }

                    #[test]
                    fn symmetric_eigen_static_square_2x2(m in matrix2_($scalar)) {
                        let m      = m.hermitian_part();
                        let eig    = m.symmetric_eigen();
                        let recomp = eig.recompose();

                        prop_assert!(relative_eq!(m.lower_triangle(), recomp.lower_triangle(), epsilon = 1.0e-5))
                    }
                }
            }
        }
    );

    gen_tests!(complex, complex_f64(), RandComplex<f64>);
    gen_tests!(f64, PROPTEST_F64, RandScalar<f64>);
}

// Test proposed on the issue #176 of rulinalg.
#[test]
#[rustfmt::skip]
fn symmetric_eigen_singular_24x24() {
    let m = DMatrix::from_row_slice(
        24,
        24,
        &[
            1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  0.0,  1.0,  1.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
            -1.0, -1.0, -1.0, -1.0, -1.0,  0.0,  1.0,  0.0,  0.0,  1.0,  1.0,  1.0,  1.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
            0.0,  0.0,  0.0,  0.0,  0.0, -1.0, -1.0, -1.0, -1.0,  0.0,  0.0,  0.0,  0.0,  1.0,  1.0,  1.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
            0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0, -1.0, -1.0, -1.0,  0.0,  0.0,  0.0,  0.0,  1.0,  1.0,  1.0,  1.0,  0.0,  0.0,  0.0,  0.0,
            0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0,  0.0,  1.0,  1.0,  1.0,
            0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
            0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
            0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
            0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
            0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
            0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
            0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  4.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
            0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
            0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
            0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  4.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
            0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
            0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
            0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
            0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  4.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
            0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
            0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
            0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
            0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0,
            0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0
        ],
    );

    let eig = m.clone().symmetric_eigen();
    let recomp = eig.recompose();

    assert_relative_eq!(
        m.lower_triangle(),
        recomp.lower_triangle(),
        epsilon = 1.0e-5
    );
}

// Regression test for #1368
#[test]
fn very_small_deviation_from_identity_issue_1368() {
    let m = Matrix3::<f32>::new(
        1.0,
        3.1575704e-23,
        8.1146196e-23,
        3.1575704e-23,
        1.0,
        1.7471054e-22,
        8.1146196e-23,
        1.7471054e-22,
        1.0,
    );

    for v in m
        .try_symmetric_eigen(f32::EPSILON, 0)
        .unwrap()
        .eigenvalues
        .into_iter()
    {
        assert_relative_eq!(*v, 1.);
    }
}

// Regression test for #1528
#[test]
#[rustfmt::skip]
fn eigenvalues_search_should_not_hang_issue_1528() {
    let m = DMatrix::from_row_slice(
        4,
        4,
        &[
            0.0_f64,            0.5773502691896257, 0.0,                0.0,
            0.5773502691896257, 0.0,                0.5163977794943222, 0.0,
            0.0,                0.5163977794943222, 0.0,                0.50709255283711,
            0.0,                0.0,                0.50709255283711,   0.0,
        ],
    );
    let complex_eigenvals = m.complex_eigenvalues();

    assert_relative_eq!(
        complex_eigenvals.iter().sum::<Complex<f64>>().re,
        m.trace(),
        epsilon = 1e-10
    );

    assert_relative_eq!(
        complex_eigenvals.iter().product::<Complex<f64>>().re,
        m.determinant(),
        epsilon = 1e-10
    );
}

//  #[cfg(feature = "arbitrary")]
//  quickcheck! {
// TODO: full eigendecomposition is not implemented yet because of its complexity when some
// eigenvalues have multiplicity > 1.
//
//    /*
//     * NOTE: for the following tests, we use only upper-triangular matrices.
//     * This ensures the schur decomposition will work, and allows use to test the eigenvector
//     * computation.
//     */
//    fn eigen(n: usize) -> bool {
//        let n = cmp::max(1, cmp::min(n, 10));
//        let m = DMatrix::<f64>::new_random(n, n).upper_triangle();
//
//        let eig = RealEigen::new(m.clone()).unwrap();
//        verify_eigenvectors(m, eig)
//    }
//
//    fn eigen_with_adjacent_duplicate_diagonals(n: usize) -> bool {
//        let n = cmp::max(1, cmp::min(n, 10));
//        let mut m = DMatrix::<f64>::new_random(n, n).upper_triangle();
//
//        // Suplicate some adjacent diagonal elements.
//        for i in 0 .. n / 2 {
//            m[(i * 2 + 1, i * 2 + 1)] = m[(i * 2, i * 2)];
//        }
//
//        let eig = RealEigen::new(m.clone()).unwrap();
//        verify_eigenvectors(m, eig)
//    }
//
//    fn eigen_with_nonadjacent_duplicate_diagonals(n: usize) -> bool {
//        let n = cmp::max(3, cmp::min(n, 10));
//        let mut m = DMatrix::<f64>::new_random(n, n).upper_triangle();
//
//        // Suplicate some diagonal elements.
//        for i in n / 2 .. n {
//            m[(i, i)] = m[(i - n / 2, i - n / 2)];
//        }
//
//        let eig = RealEigen::new(m.clone()).unwrap();
//        verify_eigenvectors(m, eig)
//    }
//
//    fn eigen_static_square_4x4(m: Matrix4<f64>) -> bool {
//        let m = m.upper_triangle();
//        let eig = RealEigen::new(m.clone()).unwrap();
//        verify_eigenvectors(m, eig)
//    }
//
//    fn eigen_static_square_3x3(m: Matrix3<f64>) -> bool {
//        let m = m.upper_triangle();
//        let eig = RealEigen::new(m.clone()).unwrap();
//        verify_eigenvectors(m, eig)
//    }
//
//    fn eigen_static_square_2x2(m: Matrix2<f64>) -> bool {
//        let m = m.upper_triangle();
//        println!("{}", m);
//        let eig = RealEigen::new(m.clone()).unwrap();
//        verify_eigenvectors(m, eig)
//    }
//  }
//
// fn verify_eigenvectors<D: Dim>(m: OMatrix<f64, D>, mut eig: RealEigen<f64, D>) -> bool
//     where DefaultAllocator: Allocator<f64, D, D>   +
//                             Allocator<f64, D>      +
//                             Allocator<D, D> +
//                             Allocator<D>,
//           OMatrix<f64, D>: Display,
//           OVector<f64, D>: Display {
//     let mv = &m * &eig.eigenvectors;
//
//     println!("eigenvalues: {}eigenvectors: {}", eig.eigenvalues, eig.eigenvectors);
//
//     let dim = m.nrows();
//     for i in 0 .. dim {
//         let mut col = eig.eigenvectors.column_mut(i);
//         col *= eig.eigenvalues[i];
//     }
//
//     println!("{}{:.5}{:.5}", m, mv, eig.eigenvectors);
//
//     relative_eq!(eig.eigenvectors, mv, epsilon = 1.0e-5)
// }
