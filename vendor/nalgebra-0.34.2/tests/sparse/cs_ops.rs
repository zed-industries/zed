#![cfg_attr(rustfmt, rustfmt_skip)]


use na::{Matrix3x4, Matrix4x5, Matrix3x5, CsMatrix, Vector5, CsVector};

#[test]
fn axpy_cs() {
    let mut v1 = Vector5::new(1.0, 2.0, 3.0, 4.0, 5.0);
    let v2 = Vector5::new(10.0, 0.0, 30.0, 0.0, 50.0);
    let expected = 5.0 * v2 + 10.0 * v1;

    let cs: CsVector<_, _> = v2.into();
    v1.axpy_cs(5.0, &cs, 10.0);

    assert!(cs.is_sorted());
    assert_eq!(v1, expected)
}


#[test]
fn cs_mat_mul() {
    let m1 = Matrix3x4::new(
        0.0, 1.0, 4.0, 0.0,
        5.0, 6.0, 0.0, 8.0,
        9.0, 10.0, 11.0, 12.0,
    );

    let m2 = Matrix4x5::new(
        5.0, 6.0, 0.0, 8.0, 15.0,
        9.0, 10.0, 11.0, 12.0, 0.0,
        0.0, 0.0, 13.0, 0.0, 0.0,
        0.0, 1.0, 4.0, 0.0, 14.0,
    );

    let sm1: CsMatrix<_, _, _> = m1.into();
    let sm2: CsMatrix<_, _, _> = m2.into();

    let mul = &sm1 * &sm2;

    assert!(sm1.is_sorted());
    assert!(sm2.is_sorted());
    assert!(mul.is_sorted());
    assert_eq!(Matrix3x5::from(mul), m1 * m2);
}


#[test]
fn cs_mat_add() {
    let m1 = Matrix4x5::new(
        4.0, 1.0, 4.0, 0.0, 0.0,
        5.0, 6.0, 0.0, 8.0, 0.0,
        9.0, 10.0, 11.0, 12.0, 0.0,
        0.0, 0.0, 1.0, 0.0, 10.0
    );

    let m2 = Matrix4x5::new(
        0.0, 1.0, 4.0, 0.0, 14.0,
        5.0, 6.0, 0.0, 8.0, 15.0,
        9.0, 10.0, 11.0, 12.0, 0.0,
        0.0, 0.0, 13.0, 0.0, 0.0,
    );

    let sm1: CsMatrix<_, _, _> = m1.into();
    let sm2: CsMatrix<_, _, _> = m2.into();

    let sum = &sm1 + &sm2;

    assert!(sm1.is_sorted());
    assert!(sm2.is_sorted());
    assert!(sum.is_sorted());
    assert_eq!(Matrix4x5::from(sum), m1 + m2);
}
