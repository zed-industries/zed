use na::{Matrix1, Matrix2, Matrix3, Matrix4, Matrix5};

#[test]
fn matrix1_try_inverse() {
    let a = Matrix1::new(3.0);
    let a_inv = a.try_inverse().expect("Matrix is invertible");

    assert_relative_eq!(a_inv, Matrix1::new(1.0 / 3.0));
}

#[test]
#[rustfmt::skip]
fn matrix2_try_inverse() {
    let a = Matrix2::new(  5.0,  -2.0,
                         -10.0,   1.0);
    let expected_inverse = Matrix2::new(-0.2 / 3.0, -2.0 / 15.0,
                                        -2.0 / 3.0, -1.0 / 3.0);
    let a_inv = a.try_inverse()
                 .expect("Matrix is invertible");

    assert_relative_eq!(a_inv, expected_inverse);
}

#[test]
#[rustfmt::skip]
fn matrix3_try_inverse() {
    let a = Matrix3::new(-3.0,  2.0,  0.0,
                         -6.0,  9.0, -2.0,
                          9.0, -6.0,  4.0);
    let expected_inverse = Matrix3::new(-0.40, 0.4 / 3.0, 0.2 / 3.00,
                                        -0.10,       0.2,       0.10,
                                         0.75,       0.0,       0.25);
    let a_inv = a.try_inverse()
                 .expect("Matrix is invertible");

    assert_relative_eq!(a_inv, expected_inverse);
}

#[test]
#[rustfmt::skip]
fn matrix4_try_inverse_issue_214() {
    let m1 = Matrix4::new(
        -0.34727043,         0.00000005397217,   -0.000000000000003822135, -0.000000000000003821371,
        0.0,                 -0.000000026986084, -1.0001999,               -1.0,
        0.000000030359345,   0.61736965,         -0.000000043720128,       -0.00000004371139,
        -0.0000000029144975, -0.05926739,        3.8007796,                4.0);


    let m2 = Matrix4::new(
        -0.34727043,         0.00000005397217,   -0.000000000000003822135, -0.000000000000003821371,
        0.0,                 -0.000000026986084, -1.0001999,               -1.0,
        0.000000030359345,   0.61736965,         -0.000000043720128,       -0.00000004371139,
        -0.0000000029448568, -0.05988476,        3.8007796,                4.0);

    assert!(m1.try_inverse().is_some());
    assert!(m2.try_inverse().is_some());
    assert!(m1.transpose().try_inverse().is_some());
    assert!(m2.transpose().try_inverse().is_some());
}

#[test]
#[rustfmt::skip]
fn matrix5_try_inverse() {
    // Dimension 5 is chosen so that the inversion happens by Gaussian elimination.
    // (at the time of writing dimensions <= 3 are implemented as analytic formulas, but we choose
    // 5 in the case that 4 also gets an analytic implementation)
    let a = Matrix5::new(-2.0,   0.0,   2.0,   5.0,  -5.0,
                         -6.0,   4.0,   4.0,  13.0, -15.0,
                          4.0,  16.0, -14.0, -19.0,  12.0,
                         12.0,  12.0, -22.0, -35.0,  34.0,
                         -8.0,   4.0,  12.0,  27.0, -31.0);
    let expected_inverse = Matrix5::new(
        3.9333e+00,  -1.5667e+00,   2.6667e-01,   6.6667e-02,  3.0000e-01,
       -1.2033e+01,   3.9667e+00,  -1.1167e+00,   2.8333e-01, -1.0000e-01,
       -1.8233e+01,   5.7667e+00,  -1.5667e+00,   2.3333e-01, -2.0000e-01,
       -4.3333e+00,   1.6667e+00,  -6.6667e-01,   3.3333e-01, -4.6950e-19,
       -1.3400e+01,   4.6000e+00,  -1.4000e+00,   4.0000e-01, -2.0000e-01);
    let a_inv = a.try_inverse().expect("Matrix is not invertible");

    assert_relative_eq!(a_inv, expected_inverse, max_relative=1e-4);
}

#[test]
fn matrix1_try_inverse_scaled_identity() {
    // A perfectly invertible matrix with
    // very small coefficients
    let a = Matrix1::new(1.0e-20);
    let expected_inverse = Matrix1::new(1.0e20);
    let a_inv = a.try_inverse().expect("Matrix should be invertible");

    assert_relative_eq!(a_inv, expected_inverse);
}

#[test]
#[rustfmt::skip]
fn matrix2_try_inverse_scaled_identity() {
    // A perfectly invertible matrix with
    // very small coefficients
    let a = Matrix2::new(1.0e-20,     0.0,
                             0.0, 1.0e-20);
    let expected_inverse = Matrix2::new(1.0e20,    0.0,
                                           0.0, 1.0e20);
    let a_inv = a.try_inverse().expect("Matrix should be invertible");

    assert_relative_eq!(a_inv, expected_inverse);
}

#[test]
#[rustfmt::skip]
fn matrix3_try_inverse_scaled_identity() {
    // A perfectly invertible matrix with
    // very small coefficients
    let a = Matrix3::new(1.0e-20,     0.0,     0.0,
                             0.0, 1.0e-20,     0.0,
                             0.0,     0.0, 1.0e-20);
    let expected_inverse = Matrix3::new(1.0e20,    0.0,    0.0,
                                           0.0, 1.0e20,    0.0,
                                           0.0,    0.0, 1.0e20);
    let a_inv = a.try_inverse().expect("Matrix should be invertible");

    assert_relative_eq!(a_inv, expected_inverse);
}

#[test]
#[rustfmt::skip]
fn matrix5_try_inverse_scaled_identity() {
    // A perfectly invertible matrix with
    // very small coefficients
    let a = Matrix5::new(1.0e-20,     0.0,     0.0,     0.0,     0.0,
                             0.0, 1.0e-20,     0.0,     0.0,     0.0,
                             0.0,     0.0, 1.0e-20,     0.0,     0.0,
                             0.0,     0.0,     0.0, 1.0e-20,     0.0,
                             0.0,     0.0,     0.0,     0.0, 1.0e-20);
    let expected_inverse = Matrix5::new(1.0e+20,     0.0,     0.0,     0.0,     0.0,
                                            0.0, 1.0e+20,     0.0,     0.0,     0.0,
                                            0.0,     0.0, 1.0e+20,     0.0,     0.0,
                                            0.0,     0.0,     0.0, 1.0e+20,     0.0,
                                            0.0,     0.0,     0.0,     0.0, 1.0e+20);
    let a_inv = a.try_inverse().expect("Matrix should be invertible");

    assert_relative_eq!(a_inv, expected_inverse);
}
