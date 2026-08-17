//! Tests for `matrixcompare` integration.
//!
//! The `matrixcompare` crate itself is responsible for testing the actual comparison.
//! The tests here only check that the necessary trait implementations are correctly implemented,
//! in addition to some sanity checks with example input.

use matrixcompare::assert_matrix_eq;
use nalgebra::{OMatrix, U4, U5};

#[cfg(feature = "proptest-support")]
use {
    crate::proptest::*,
    matrixcompare::DenseAccess,
    nalgebra::DMatrix,
    proptest::{prop_assert_eq, proptest},
};

#[cfg(feature = "proptest-support")]
proptest! {
    #[test]
    fn fetch_single_is_equivalent_to_index_f64(matrix in dmatrix()) {
        for i in 0 .. matrix.nrows() {
            for j in 0 .. matrix.ncols() {
                prop_assert_eq!(matrix.fetch_single(i, j), *matrix.index((i, j)));
            }
        }
    }

    #[test]
    fn matrixcompare_shape_agrees_with_matrix(matrix in dmatrix()) {
        prop_assert_eq!(matrix.nrows(), <DMatrix<f64> as matrixcompare::Matrix<f64>>::rows(&matrix));
        prop_assert_eq!(matrix.ncols(), <DMatrix<f64> as matrixcompare::Matrix<f64>>::cols(&matrix));
    }
}

#[test]
fn assert_matrix_eq_dense_positive_comparison() {
    #[rustfmt::skip]
    let a = OMatrix::<_, U4, U5>::from_row_slice(&[
        1210, 1320, 1430, 1540, 1650,
        2310, 2420, 2530, 2640, 2750,
        3410, 3520, 3630, 3740, 3850,
        4510, 4620, 4730, 4840, 4950,
    ]);

    #[rustfmt::skip]
    let b = OMatrix::<_, U4, U5>::from_row_slice(&[
        1210, 1320, 1430, 1540, 1650,
        2310, 2420, 2530, 2640, 2750,
        3410, 3520, 3630, 3740, 3850,
        4510, 4620, 4730, 4840, 4950,
    ]);

    // Test matrices of static size
    assert_matrix_eq!(a, b);
    assert_matrix_eq!(&a, b);
    assert_matrix_eq!(a, &b);
    assert_matrix_eq!(&a, &b);

    // Test matrices of dynamic size
    let a_dyn = a.index((0..4, 0..5));
    let b_dyn = b.index((0..4, 0..5));
    assert_matrix_eq!(a_dyn, b_dyn);
    assert_matrix_eq!(a_dyn, &b_dyn);
    assert_matrix_eq!(&a_dyn, b_dyn);
    assert_matrix_eq!(&a_dyn, &b_dyn);
}

#[test]
#[should_panic]
fn assert_matrix_eq_dense_negative_comparison() {
    #[rustfmt::skip]
    let a = OMatrix::<_, U4, U5>::from_row_slice(&[
        1210, 1320, 1430, 1540, 1650,
        2310, 2420, 2530, 2640, 2750,
        3410, 3520, 3630, 3740, 3850,
        4510, 4620, -4730, 4840, 4950,
    ]);

    #[rustfmt::skip]
    let b = OMatrix::<_, U4, U5>::from_row_slice(&[
        1210, 1320, 1430, 1540, 1650,
        2310, 2420, 2530, 2640, 2750,
        3410, 3520, 3630, 3740, 3850,
        4510, 4620, 4730, 4840, 4950,
    ]);

    assert_matrix_eq!(a, b);
}
