use na::{
    DMatrix, Matrix, Matrix3, Matrix3x4, Matrix3x5, Matrix4, Matrix4x3, Matrix4x5, Matrix5,
    Matrix5x3, Matrix5x4,
};
use na::{Dyn, U3, U5};

#[test]
#[rustfmt::skip]
fn upper_lower_triangular() {
    let m = Matrix4::new(
        11.0, 12.0, 13.0, 14.0,
        21.0, 22.0, 23.0, 24.0,
        31.0, 32.0, 33.0, 34.0,
        41.0, 42.0, 43.0, 44.0);

    let um = Matrix4::new(
        11.0, 12.0, 13.0, 14.0,
         0.0, 22.0, 23.0, 24.0,
         0.0,  0.0, 33.0, 34.0,
         0.0,  0.0,  0.0, 44.0);

    let lm = Matrix4::new(
        11.0,  0.0,  0.0,  0.0,
        21.0, 22.0,  0.0,  0.0,
        31.0, 32.0, 33.0,  0.0,
        41.0, 42.0, 43.0, 44.0);

    let computed_um = m.upper_triangle();
    let computed_lm = m.lower_triangle();

    assert_eq!(um, computed_um);
    assert_eq!(lm, computed_lm);

    let symm_um = Matrix4::new(
        11.0, 12.0, 13.0, 14.0,
        12.0, 22.0, 23.0, 24.0,
        13.0, 23.0, 33.0, 34.0,
        14.0, 24.0, 34.0, 44.0);

    let symm_lm = Matrix4::new(
        11.0, 21.0, 31.0, 41.0,
        21.0, 22.0, 32.0, 42.0,
        31.0, 32.0, 33.0, 43.0,
        41.0, 42.0, 43.0, 44.0);

    let mut computed_symm_um = m.clone();
    let mut computed_symm_lm = m.clone();

    computed_symm_um.fill_lower_triangle_with_upper_triangle();
    computed_symm_lm.fill_upper_triangle_with_lower_triangle();
    assert_eq!(symm_um, computed_symm_um);
    assert_eq!(symm_lm, computed_symm_lm);


    let m = Matrix5x3::new(
        11.0, 12.0, 13.0,
        21.0, 22.0, 23.0,
        31.0, 32.0, 33.0,
        41.0, 42.0, 43.0,
        51.0, 52.0, 53.0);

    let um = Matrix5x3::new(
        11.0, 12.0, 13.0,
         0.0, 22.0, 23.0,
         0.0,  0.0, 33.0,
         0.0,  0.0,  0.0,
         0.0,  0.0,  0.0);

    let lm = Matrix5x3::new(
        11.0,  0.0,  0.0,
        21.0, 22.0,  0.0,
        31.0, 32.0, 33.0,
        41.0, 42.0, 43.0,
        51.0, 52.0, 53.0);

    let computed_um = m.upper_triangle();
    let computed_lm = m.lower_triangle();

    assert_eq!(um, computed_um);
    assert_eq!(lm, computed_lm);


    let m = Matrix3x5::new(
        11.0, 12.0, 13.0, 14.0, 15.0,
        21.0, 22.0, 23.0, 24.0, 25.0,
        31.0, 32.0, 33.0, 34.0, 35.0);

    let um = Matrix3x5::new(
        11.0, 12.0, 13.0, 14.0, 15.0,
         0.0, 22.0, 23.0, 24.0, 25.0,
         0.0,  0.0, 33.0, 34.0, 35.0);

    let lm = Matrix3x5::new(
        11.0,  0.0,  0.0,  0.0, 0.0,
        21.0, 22.0,  0.0,  0.0, 0.0,
        31.0, 32.0, 33.0,  0.0, 0.0);

    let computed_um = m.upper_triangle();
    let computed_lm = m.lower_triangle();

    assert_eq!(um, computed_um);
    assert_eq!(lm, computed_lm);

    let mut m = Matrix4x5::new(
        11.0, 12.0, 13.0, 14.0, 15.0,
        21.0, 22.0, 23.0, 24.0, 25.0,
        31.0, 32.0, 33.0, 34.0, 35.0,
        41.0, 42.0, 43.0, 44.0, 45.0);

    let expected_m = Matrix4x5::new(
        11.0, 12.0,  0.0,  0.0,  0.0,
        21.0, 22.0, 23.0,  0.0,  0.0,
        31.0, 32.0, 33.0, 34.0,  0.0,
        41.0, 42.0, 43.0, 44.0, 45.0);

    m.fill_upper_triangle(0.0, 2);

    assert_eq!(m, expected_m);

    let mut m = Matrix4x5::new(
        11.0, 12.0, 13.0, 14.0, 15.0,
        21.0, 22.0, 23.0, 24.0, 25.0,
        31.0, 32.0, 33.0, 34.0, 35.0,
        41.0, 42.0, 43.0, 44.0, 45.0);

    let expected_m = Matrix4x5::new(
        11.0, 12.0, 13.0, 14.0, 15.0,
        21.0, 22.0, 23.0, 24.0, 25.0,
         0.0, 32.0, 33.0, 34.0, 35.0,
         0.0,  0.0, 43.0, 44.0, 45.0);

    m.fill_lower_triangle(0.0, 2);

    assert_eq!(m, expected_m);

    let mut m = Matrix5x4::new(
        11.0, 12.0, 13.0, 14.0,
        21.0, 22.0, 23.0, 24.0,
        31.0, 32.0, 33.0, 34.0,
        41.0, 42.0, 43.0, 44.0,
        51.0, 52.0, 53.0, 54.0);

    let expected_m = Matrix5x4::new(
        11.0, 12.0,  0.0,  0.0,
        21.0, 22.0, 23.0,  0.0,
        31.0, 32.0, 33.0, 34.0,
        41.0, 42.0, 43.0, 44.0,
        51.0, 52.0, 53.0, 54.0);

    m.fill_upper_triangle(0.0, 2);

    assert_eq!(m, expected_m);

    let mut m = Matrix5x4::new(
        11.0, 12.0, 13.0, 14.0,
        21.0, 22.0, 23.0, 24.0,
        31.0, 32.0, 33.0, 34.0,
        41.0, 42.0, 43.0, 44.0,
        51.0, 52.0, 53.0, 54.0);

    let expected_m = Matrix5x4::new(
        11.0, 12.0, 13.0, 14.0,
        21.0, 22.0, 23.0, 24.0,
         0.0, 32.0, 33.0, 34.0,
         0.0,  0.0, 43.0, 44.0,
         0.0,  0.0,  0.0, 54.0);

    m.fill_lower_triangle(0.0, 2);

    assert_eq!(m, expected_m);
}

#[test]
#[rustfmt::skip]
fn swap_rows() {
    let mut m = Matrix5x3::new(
        11.0, 12.0, 13.0,
        21.0, 22.0, 23.0,
        31.0, 32.0, 33.0,
        41.0, 42.0, 43.0,
        51.0, 52.0, 53.0);

    let expected = Matrix5x3::new(
        11.0, 12.0, 13.0,
        41.0, 42.0, 43.0,
        31.0, 32.0, 33.0,
        21.0, 22.0, 23.0,
        51.0, 52.0, 53.0);

    m.swap_rows(1, 3);

    assert_eq!(m, expected);
}

#[test]
#[rustfmt::skip]
fn swap_columns() {
    let mut m = Matrix3x5::new(
        11.0, 12.0, 13.0, 14.0, 15.0,
        21.0, 22.0, 23.0, 24.0, 25.0,
        31.0, 32.0, 33.0, 34.0, 35.0);

    let expected = Matrix3x5::new(
        11.0, 14.0, 13.0, 12.0, 15.0,
        21.0, 24.0, 23.0, 22.0, 25.0,
        31.0, 34.0, 33.0, 32.0, 35.0);

    m.swap_columns(1, 3);

    assert_eq!(m, expected);
}

#[test]
#[rustfmt::skip]
fn remove_columns() {
    let m = Matrix3x5::new(
        11, 12, 13, 14, 15,
        21, 22, 23, 24, 25,
        31, 32, 33, 34, 35);

    let expected_a1 = Matrix3x4::new(
        12, 13, 14, 15,
        22, 23, 24, 25,
        32, 33, 34, 35);

    let expected_a2 = Matrix3x4::new(
        11, 12, 13, 14,
        21, 22, 23, 24,
        31, 32, 33, 34);

    let expected_a3 = Matrix3x4::new(
        11, 12, 14, 15,
        21, 22, 24, 25,
        31, 32, 34, 35);

    assert_eq!(m.remove_column(0), expected_a1);
    assert_eq!(m.remove_column(4), expected_a2);
    assert_eq!(m.remove_column(2), expected_a3);

    let expected_b1 = Matrix3::new(
        13, 14, 15,
        23, 24, 25,
        33, 34, 35);

    let expected_b2 = Matrix3::new(
        11, 12, 13,
        21, 22, 23,
        31, 32, 33);

    let expected_b3 = Matrix3::new(
        11, 12, 15,
        21, 22, 25,
        31, 32, 35);

    assert_eq!(m.remove_fixed_columns::<2>(0), expected_b1);
    assert_eq!(m.remove_fixed_columns::<2>(3), expected_b2);
    assert_eq!(m.remove_fixed_columns::<2>(2), expected_b3);

    // The following is just to verify that the return type dimensions is correctly inferred.
    let computed: Matrix<_, U3, Dyn, _> = m.remove_columns(3, 2);
    assert!(computed.eq(&expected_b2));

    /*
     * Same thing but using a non-copy scalar type.
     */
    let m = m.map(Box::new);
    let expected_a1 = expected_a1.map(Box::new);
    let expected_a2 = expected_a2.map(Box::new);
    let expected_a3 = expected_a3.map(Box::new);

    assert_eq!(m.clone().remove_column(0), expected_a1);
    assert_eq!(m.clone().remove_column(4), expected_a2);
    assert_eq!(m.clone().remove_column(2), expected_a3);
    
    let expected_b1 = expected_b1.map(Box::new);
    let expected_b2 = expected_b2.map(Box::new);
    let expected_b3 = expected_b3.map(Box::new);

    assert_eq!(m.clone().remove_fixed_columns::<2>(0), expected_b1);
    assert_eq!(m.clone().remove_fixed_columns::<2>(3), expected_b2);
    assert_eq!(m.remove_fixed_columns::<2>(2), expected_b3);
}

#[test]
#[rustfmt::skip]
fn remove_columns_at() {
    let m = DMatrix::from_row_slice(5, 5, &[
        11, 12, 13, 14, 15,
        21, 22, 23, 24, 25,
        31, 32, 33, 34, 35,
        41, 42, 43, 44, 45,
        51, 52, 53, 54, 55
    ]);

    let expected1 = DMatrix::from_row_slice(5, 4, &[
        12, 13, 14, 15,
        22, 23, 24, 25,
        32, 33, 34, 35,
        42, 43, 44, 45,
        52, 53, 54, 55
    ]);

    assert_eq!(m.remove_columns_at(&[0]), expected1);

    let m = DMatrix::from_row_slice(5, 5, &[
        11, 12, 13, 14, 15,
        21, 22, 23, 24, 25,
        31, 32, 33, 34, 35,
        41, 42, 43, 44, 45,
        51, 52, 53, 54, 55
    ]);

    let expected2 = DMatrix::from_row_slice(5, 3, &[
        11, 13, 15,
        21, 23, 25,
        31, 33, 35,
        41, 43, 45,
        51, 53, 55
    ]);
    
    assert_eq!(m.remove_columns_at(&[1,3]), expected2);

    let m = DMatrix::from_row_slice(5, 5, &[
        11, 12, 13, 14, 15,
        21, 22, 23, 24, 25,
        31, 32, 33, 34, 35,
        41, 42, 43, 44, 45,
        51, 52, 53, 54, 55
    ]);

    let expected3 = DMatrix::from_row_slice(5, 2, &[
        12, 13,
        22, 23,
        32, 33,
        42, 43,
        52, 53, 
    ]);

    assert_eq!(m.remove_columns_at(&[0,3,4]), expected3);
}

#[test]
#[rustfmt::skip]
fn remove_rows() {
    let m = Matrix5x3::new(
        11, 12, 13,
        21, 22, 23,
        31, 32, 33,
        41, 42, 43,
        51, 52, 53);

    let expected1 = Matrix4x3::new(
        21, 22, 23,
        31, 32, 33,
        41, 42, 43,
        51, 52, 53);

    let expected2 = Matrix4x3::new(
        11, 12, 13,
        21, 22, 23,
        31, 32, 33,
        41, 42, 43);

    let expected3 = Matrix4x3::new(
        11, 12, 13,
        21, 22, 23,
        41, 42, 43,
        51, 52, 53);

    assert_eq!(m.remove_row(0), expected1);
    assert_eq!(m.remove_row(4), expected2);
    assert_eq!(m.remove_row(2), expected3);

    let expected1 = Matrix3::new(
        31, 32, 33,
        41, 42, 43,
        51, 52, 53);

    let expected2 = Matrix3::new(
        11, 12, 13,
        21, 22, 23,
        31, 32, 33);

    let expected3 = Matrix3::new(
        11, 12, 13,
        21, 22, 23,
        51, 52, 53);

    assert_eq!(m.remove_fixed_rows::<2>(0), expected1);
    assert_eq!(m.remove_fixed_rows::<2>(3), expected2);
    assert_eq!(m.remove_fixed_rows::<2>(2), expected3);

    // The following is just to verify that the return type dimensions is correctly inferred.
    let computed: Matrix<_, Dyn, U3, _> = m.remove_rows(3, 2);
    assert!(computed.eq(&expected2));
}

#[test]
#[rustfmt::skip]
fn remove_rows_at() {
    let m = DMatrix::from_row_slice(5, 5, &[
        11, 12, 13, 14, 15,
        21, 22, 23, 24, 25,
        31, 32, 33, 34, 35,
        41, 42, 43, 44, 45,
        51, 52, 53, 54, 55
    ]);

    let expected1 = DMatrix::from_row_slice(4, 5, &[
        21, 22, 23, 24, 25,
        31, 32, 33, 34, 35,
        41, 42, 43, 44, 45,
        51, 52, 53, 54, 55
    ]);

    assert_eq!(m.remove_rows_at(&[0]), expected1);

    let m = DMatrix::from_row_slice(5, 5, &[
        11, 12, 13, 14, 15,
        21, 22, 23, 24, 25,
        31, 32, 33, 34, 35,
        41, 42, 43, 44, 45,
        51, 52, 53, 54, 55
    ]);

    let expected2 = DMatrix::from_row_slice(3, 5, &[
        11, 12, 13, 14, 15,
        31, 32, 33, 34, 35,
        51, 52, 53, 54, 55
    ]);
    
    assert_eq!(m.remove_rows_at(&[1,3]), expected2);

    let m = DMatrix::from_row_slice(5, 5, &[
        11, 12, 13, 14, 15,
        21, 22, 23, 24, 25,
        31, 32, 33, 34, 35,
        41, 42, 43, 44, 45,
        51, 52, 53, 54, 55
    ]);

    let expected3 = DMatrix::from_row_slice(2, 5, &[
        21, 22, 23, 24, 25,
        31, 32, 33, 34, 35
    ]);

    assert_eq!(m.remove_rows_at(&[0,3,4]), expected3);
}

#[test]
#[rustfmt::skip]
fn insert_columns() {
    let m = Matrix5x3::new(
        11, 12, 13,
        21, 22, 23,
        31, 32, 33,
        41, 42, 43,
        51, 52, 53);

    let expected1 = Matrix5x4::new(
        0, 11, 12, 13,
        0, 21, 22, 23,
        0, 31, 32, 33,
        0, 41, 42, 43,
        0, 51, 52, 53);

    let expected2 = Matrix5x4::new(
        11, 12, 13, 0,
        21, 22, 23, 0,
        31, 32, 33, 0,
        41, 42, 43, 0,
        51, 52, 53, 0);

    let expected3 = Matrix5x4::new(
        11, 12, 0, 13,
        21, 22, 0, 23,
        31, 32, 0, 33,
        41, 42, 0, 43,
        51, 52, 0, 53);

    assert_eq!(m.insert_column(0, 0), expected1);
    assert_eq!(m.insert_column(3, 0), expected2);
    assert_eq!(m.insert_column(2, 0), expected3);

    let expected1 = Matrix5::new(
        0, 0, 11, 12, 13,
        0, 0, 21, 22, 23,
        0, 0, 31, 32, 33,
        0, 0, 41, 42, 43,
        0, 0, 51, 52, 53);

    let expected2 = Matrix5::new(
        11, 12, 13, 0, 0,
        21, 22, 23, 0, 0,
        31, 32, 33, 0, 0,
        41, 42, 43, 0, 0,
        51, 52, 53, 0, 0);

    let expected3 = Matrix5::new(
        11, 12, 0, 0, 13,
        21, 22, 0, 0, 23,
        31, 32, 0, 0, 33,
        41, 42, 0, 0, 43,
        51, 52, 0, 0, 53);

    assert_eq!(m.insert_fixed_columns::<2>(0, 0), expected1);
    assert_eq!(m.insert_fixed_columns::<2>(3, 0), expected2);
    assert_eq!(m.insert_fixed_columns::<2>(2, 0), expected3);

    // The following is just to verify that the return type dimensions is correctly inferred.
    let computed: Matrix<_, U5, Dyn, _> = m.insert_columns(3, 2, 0);
    assert!(computed.eq(&expected2));
}

#[test]
#[rustfmt::skip]
fn insert_columns_to_empty_matrix() {
    let m1 = DMatrix::repeat(0, 0, 0);
    let m2 = DMatrix::repeat(3, 0, 0);

    let expected1 = DMatrix::repeat(0, 5, 42);
    let expected2 = DMatrix::repeat(3, 5, 42);

    assert_eq!(expected1, m1.insert_columns(0, 5, 42));
    assert_eq!(expected2, m2.insert_columns(0, 5, 42));
}

#[test]
#[rustfmt::skip]
fn insert_rows() {
    let m = Matrix3x5::new(
        11, 12, 13, 14, 15,
        21, 22, 23, 24, 25,
        31, 32, 33, 34, 35);

    let expected1 = Matrix4x5::new(
         0,  0,  0,  0,  0,
        11, 12, 13, 14, 15,
        21, 22, 23, 24, 25,
        31, 32, 33, 34, 35);

    let expected2 = Matrix4x5::new(
        11, 12, 13, 14, 15,
        21, 22, 23, 24, 25,
        31, 32, 33, 34, 35,
         0,  0,  0,  0,  0);

    let expected3 = Matrix4x5::new(
        11, 12, 13, 14, 15,
        21, 22, 23, 24, 25,
         0,  0,  0,  0,  0,
        31, 32, 33, 34, 35);

    assert_eq!(m.insert_row(0, 0), expected1);
    assert_eq!(m.insert_row(3, 0), expected2);
    assert_eq!(m.insert_row(2, 0), expected3);

    let expected1 = Matrix5::new(
         0,  0,  0,  0,  0,
         0,  0,  0,  0,  0,
        11, 12, 13, 14, 15,
        21, 22, 23, 24, 25,
        31, 32, 33, 34, 35);

    let expected2 = Matrix5::new(
        11, 12, 13, 14, 15,
        21, 22, 23, 24, 25,
        31, 32, 33, 34, 35,
         0,  0,  0,  0,  0,
         0,  0,  0,  0,  0);

    let expected3 = Matrix5::new(
        11, 12, 13, 14, 15,
        21, 22, 23, 24, 25,
         0,  0,  0,  0,  0,
         0,  0,  0,  0,  0,
        31, 32, 33, 34, 35);

    assert_eq!(m.insert_fixed_rows::<2>(0, 0), expected1);
    assert_eq!(m.insert_fixed_rows::<2>(3, 0), expected2);
    assert_eq!(m.insert_fixed_rows::<2>(2, 0), expected3);

    // The following is just to verify that the return type dimensions is correctly inferred.
    let computed: Matrix<_, Dyn, U5, _> = m.insert_rows(3, 2, 0);
    assert!(computed.eq(&expected2));
}

#[test]
fn insert_rows_to_empty_matrix() {
    let m1 = DMatrix::repeat(0, 0, 0);
    let m2 = DMatrix::repeat(0, 5, 0);

    let expected1 = DMatrix::repeat(3, 0, 42);
    let expected2 = DMatrix::repeat(3, 5, 42);

    assert_eq!(expected1, m1.insert_rows(0, 3, 42));
    assert_eq!(expected2, m2.insert_rows(0, 3, 42));
}

#[test]
#[rustfmt::skip]
fn resize() {
    let m = Matrix3x5::new(
        11, 12, 13, 14, 15,
        21, 22, 23, 24, 25,
        31, 32, 33, 34, 35);

    let add_colls = DMatrix::from_row_slice(3, 6, &[
        11, 12, 13, 14, 15, 42,
        21, 22, 23, 24, 25, 42,
        31, 32, 33, 34, 35, 42]);
        
    let del_colls = DMatrix::from_row_slice(3, 4, &[
        11, 12, 13, 14,
        21, 22, 23, 24,
        31, 32, 33, 34]);

    let add_rows = DMatrix::from_row_slice(4, 5, &[
        11, 12, 13, 14, 15,
        21, 22, 23, 24, 25,
        31, 32, 33, 34, 35,
        42, 42, 42, 42, 42]);

    let del_rows = DMatrix::from_row_slice(2, 5, &[
        11, 12, 13, 14, 15,
        21, 22, 23, 24, 25]);

    let add_add = DMatrix::from_row_slice(5, 6, &[
        11, 12, 13, 14, 15, 42,
        21, 22, 23, 24, 25, 42,
        31, 32, 33, 34, 35, 42,
        42, 42, 42, 42, 42, 42,
        42, 42, 42, 42, 42, 42]);

    let del_del = DMatrix::from_row_slice(1, 2, &[11, 12]);

    let add_del = DMatrix::from_row_slice(5, 2, &[
        11, 12,
        21, 22,
        31, 32,
        42, 42,
        42, 42]);

    let del_add = DMatrix::from_row_slice(1, 8, &[
        11, 12, 13, 14, 15, 42, 42, 42]);

    assert_eq!(add_colls, m.resize(3, 6, 42));
    assert_eq!(del_colls, m.resize(3, 4, 42));
    assert_eq!(add_rows, m.resize(4, 5, 42));
    assert_eq!(del_rows, m.resize(2, 5, 42));
    assert_eq!(del_del, m.resize(1, 2, 42));
    assert_eq!(add_add, m.resize(5, 6, 42));
    assert_eq!(add_del, m.resize(5, 2, 42));
    assert_eq!(del_add, m.resize(1, 8, 42));
}

#[test]
fn resize_empty_matrix() {
    let m1 = DMatrix::repeat(0, 0, 0);
    let m2 = DMatrix::repeat(1, 0, 0); // Less rows than target size.
    let m3 = DMatrix::repeat(3, 0, 0); // Same rows as target size.
    let m4 = DMatrix::repeat(9, 0, 0); // More rows than target size.
    let m5 = DMatrix::repeat(0, 1, 0); // Less columns than target size.
    let m6 = DMatrix::repeat(0, 5, 0); // Same columns as target size.
    let m7 = DMatrix::repeat(0, 9, 0); // More columns than target size.

    let resized = DMatrix::repeat(3, 5, 42);
    let resized_wo_rows = DMatrix::repeat(0, 5, 42);
    let resized_wo_cols = DMatrix::repeat(3, 0, 42);

    assert_eq!(resized, m1.clone().resize(3, 5, 42));
    assert_eq!(resized, m2.clone().resize(3, 5, 42));
    assert_eq!(resized, m3.clone().resize(3, 5, 42));
    assert_eq!(resized, m4.clone().resize(3, 5, 42));
    assert_eq!(resized, m5.clone().resize(3, 5, 42));
    assert_eq!(resized, m6.clone().resize(3, 5, 42));
    assert_eq!(resized, m7.clone().resize(3, 5, 42));

    assert_eq!(resized_wo_rows, m1.clone().resize(0, 5, 42));
    assert_eq!(resized_wo_rows, m2.clone().resize(0, 5, 42));
    assert_eq!(resized_wo_rows, m3.clone().resize(0, 5, 42));
    assert_eq!(resized_wo_rows, m4.clone().resize(0, 5, 42));
    assert_eq!(resized_wo_rows, m5.clone().resize(0, 5, 42));
    assert_eq!(resized_wo_rows, m6.clone().resize(0, 5, 42));
    assert_eq!(resized_wo_rows, m7.clone().resize(0, 5, 42));

    assert_eq!(resized_wo_cols, m1.clone().resize(3, 0, 42));
    assert_eq!(resized_wo_cols, m2.clone().resize(3, 0, 42));
    assert_eq!(resized_wo_cols, m3.clone().resize(3, 0, 42));
    assert_eq!(resized_wo_cols, m4.clone().resize(3, 0, 42));
    assert_eq!(resized_wo_cols, m5.clone().resize(3, 0, 42));
    assert_eq!(resized_wo_cols, m6.clone().resize(3, 0, 42));
    assert_eq!(resized_wo_cols, m7.clone().resize(3, 0, 42));

    assert_eq!(m1, m1.clone().resize(0, 0, 42));
    assert_eq!(m1, m2.resize(0, 0, 42));
    assert_eq!(m1, m3.resize(0, 0, 42));
    assert_eq!(m1, m4.resize(0, 0, 42));
    assert_eq!(m1, m5.resize(0, 0, 42));
    assert_eq!(m1, m6.resize(0, 0, 42));
    assert_eq!(m1, m7.resize(0, 0, 42));
}
