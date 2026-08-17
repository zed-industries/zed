use na::{DMatrix, DVector};

#[test]
fn empty_matrix_mul_vector() {
    // Issue #644
    let m = DMatrix::<f32>::zeros(8, 0);
    let v = DVector::<f32>::zeros(0);
    assert_eq!(m * v, DVector::zeros(8));
}

#[test]
fn empty_matrix_mul_matrix() {
    let m1 = DMatrix::<f32>::zeros(3, 0);
    let m2 = DMatrix::<f32>::zeros(0, 4);
    assert_eq!(m1 * m2, DMatrix::zeros(3, 4));

    // Still works with larger matrices.
    let m1 = DMatrix::<f32>::zeros(13, 0);
    let m2 = DMatrix::<f32>::zeros(0, 14);
    assert_eq!(m1 * m2, DMatrix::zeros(13, 14));
}

#[test]
fn empty_matrix_tr_mul_vector() {
    let m = DMatrix::<f32>::zeros(0, 5);
    let v = DVector::<f32>::zeros(0);
    assert_eq!(m.tr_mul(&v), DVector::zeros(5));
}

#[test]
fn empty_matrix_tr_mul_matrix() {
    let m1 = DMatrix::<f32>::zeros(0, 3);
    let m2 = DMatrix::<f32>::zeros(0, 4);
    assert_eq!(m1.tr_mul(&m2), DMatrix::zeros(3, 4));
}

#[test]
fn empty_matrix_gemm() {
    let mut res = DMatrix::repeat(3, 4, 1.0);
    let m1 = DMatrix::<f32>::zeros(3, 0);
    let m2 = DMatrix::<f32>::zeros(0, 4);
    res.gemm(1.0, &m1, &m2, 0.5);
    assert_eq!(res, DMatrix::repeat(3, 4, 0.5));

    // Still works with lager matrices.
    let mut res = DMatrix::repeat(13, 14, 1.0);
    let m1 = DMatrix::<f32>::zeros(13, 0);
    let m2 = DMatrix::<f32>::zeros(0, 14);
    res.gemm(1.0, &m1, &m2, 0.5);
    assert_eq!(res, DMatrix::repeat(13, 14, 0.5));
}

#[test]
fn empty_matrix_gemm_tr() {
    let mut res = DMatrix::repeat(3, 4, 1.0);
    let m1 = DMatrix::<f32>::zeros(0, 3);
    let m2 = DMatrix::<f32>::zeros(0, 4);
    res.gemm_tr(1.0, &m1, &m2, 0.5);
    assert_eq!(res, DMatrix::repeat(3, 4, 0.5));
}
