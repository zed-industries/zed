use crate::macros::assert_eq_and_type;
use nalgebra::{
    DMatrix, DVector, Matrix1x2, Matrix1x3, Matrix1x4, Matrix2, Matrix2x1, Matrix2x3, Matrix2x4,
    Matrix3, Matrix3x1, Matrix3x2, Matrix3x4, Matrix4, Matrix4x1, Matrix4x2, Matrix4x3, Point,
    Point1, Point2, Point3, Point4, Point5, Point6, SMatrix, SVector, Vector1, Vector2, Vector3,
    Vector4, Vector5, Vector6,
};
use nalgebra_macros::{dmatrix, dvector, matrix, point, vector};

// Skip rustfmt because it just makes the test bloated without making it more readable
#[rustfmt::skip]
#[test]
fn matrix_small_dims_exhaustive() {
    // 0x0
    assert_eq_and_type!(matrix![], SMatrix::<i32, 0, 0>::zeros());

    // 1xN
    assert_eq_and_type!(matrix![1], SMatrix::<i32, 1, 1>::new(1));
    assert_eq_and_type!(matrix![1, 2], Matrix1x2::new(1, 2));
    assert_eq_and_type!(matrix![1, 2, 3], Matrix1x3::new(1, 2, 3));
    assert_eq_and_type!(matrix![1, 2, 3, 4], Matrix1x4::new(1, 2, 3, 4));

    // 2xN
    assert_eq_and_type!(matrix![1; 2], Matrix2x1::new(1, 2));
    assert_eq_and_type!(matrix![1, 2; 3, 4], Matrix2::new(1, 2, 3, 4));
    assert_eq_and_type!(matrix![1, 2, 3; 4, 5, 6], Matrix2x3::new(1, 2, 3, 4, 5, 6));
    assert_eq_and_type!(matrix![1, 2, 3, 4; 5, 6, 7, 8], Matrix2x4::new(1, 2, 3, 4, 5, 6, 7, 8));

    // 3xN
    assert_eq_and_type!(matrix![1; 2; 3], Matrix3x1::new(1, 2, 3));
    assert_eq_and_type!(matrix![1, 2; 3, 4; 5, 6], Matrix3x2::new(1, 2, 3, 4, 5, 6));
    assert_eq_and_type!(matrix![1, 2, 3; 4, 5, 6; 7, 8, 9], Matrix3::new(1, 2, 3, 4, 5, 6, 7, 8, 9));
    assert_eq_and_type!(matrix![1, 2, 3, 4; 5, 6, 7, 8; 9, 10, 11, 12],
               Matrix3x4::new(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12));

    // 4xN
    assert_eq_and_type!(matrix![1; 2; 3; 4], Matrix4x1::new(1, 2, 3, 4));
    assert_eq_and_type!(matrix![1, 2; 3, 4; 5, 6; 7, 8], Matrix4x2::new(1, 2, 3, 4, 5, 6, 7, 8));
    assert_eq_and_type!(matrix![1, 2, 3; 4, 5, 6; 7, 8, 9; 10, 11, 12],
               Matrix4x3::new(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12));
    assert_eq_and_type!(matrix![1, 2, 3, 4; 5, 6, 7, 8; 9, 10, 11, 12; 13, 14, 15, 16],
               Matrix4::new(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16));
}

#[test]
fn matrix_const_fn() {
    // Ensure that matrix! can be used in const contexts
    const _: SMatrix<i32, 0, 0> = matrix![];
    const _: SMatrix<i32, 1, 2> = matrix![1, 2];
    const _: SMatrix<i32, 2, 3> = matrix![1, 2, 3; 4, 5, 6];
}

// Skip rustfmt because it just makes the test bloated without making it more readable
#[rustfmt::skip]
#[test]
fn dmatrix_small_dims_exhaustive() {
    // 0x0
    assert_eq_and_type!(dmatrix![], DMatrix::<i32>::zeros(0, 0));

    // 1xN
    assert_eq_and_type!(dmatrix![1], DMatrix::from_row_slice(1, 1, &[1]));
    assert_eq_and_type!(dmatrix![1, 2], DMatrix::from_row_slice(1, 2, &[1, 2]));
    assert_eq_and_type!(dmatrix![1, 2, 3], DMatrix::from_row_slice(1, 3, &[1, 2, 3]));
    assert_eq_and_type!(dmatrix![1, 2, 3, 4], DMatrix::from_row_slice(1, 4, &[1, 2, 3, 4]));

    // 2xN
    assert_eq_and_type!(dmatrix![1; 2], DMatrix::from_row_slice(2, 1,  &[1, 2]));
    assert_eq_and_type!(dmatrix![1, 2; 3, 4], DMatrix::from_row_slice(2, 2,  &[1, 2, 3, 4]));
    assert_eq_and_type!(dmatrix![1, 2, 3; 4, 5, 6], DMatrix::from_row_slice(2, 3,  &[1, 2, 3, 4, 5, 6]));
    assert_eq_and_type!(dmatrix![1, 2, 3, 4; 5, 6, 7, 8], DMatrix::from_row_slice(2, 4,  &[1, 2, 3, 4, 5, 6, 7, 8]));

    // 3xN
    assert_eq_and_type!(dmatrix![1; 2; 3], DMatrix::from_row_slice(3, 1,  &[1, 2, 3]));
    assert_eq_and_type!(dmatrix![1, 2; 3, 4; 5, 6], DMatrix::from_row_slice(3, 2,  &[1, 2, 3, 4, 5, 6]));
    assert_eq_and_type!(dmatrix![1, 2, 3; 4, 5, 6; 7, 8, 9], DMatrix::from_row_slice(3, 3,  &[1, 2, 3, 4, 5, 6, 7, 8, 9]));
    assert_eq_and_type!(dmatrix![1, 2, 3, 4; 5, 6, 7, 8; 9, 10, 11, 12],
               DMatrix::from_row_slice(3, 4,  &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]));

    // 4xN
    assert_eq_and_type!(dmatrix![1; 2; 3; 4], DMatrix::from_row_slice(4, 1,  &[1, 2, 3, 4]));
    assert_eq_and_type!(dmatrix![1, 2; 3, 4; 5, 6; 7, 8], DMatrix::from_row_slice(4, 2,  &[1, 2, 3, 4, 5, 6, 7, 8]));
    assert_eq_and_type!(dmatrix![1, 2, 3; 4, 5, 6; 7, 8, 9; 10, 11, 12],
               DMatrix::from_row_slice(4, 3,  &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]));
    assert_eq_and_type!(dmatrix![1, 2, 3, 4; 5, 6, 7, 8; 9, 10, 11, 12; 13, 14, 15, 16],
               DMatrix::from_row_slice(4, 4,  &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]));
}

#[test]
fn matrix_trailing_semi() {
    matrix![1, 2;];
    dmatrix![1, 2;];
}

// Skip rustfmt because it just makes the test bloated without making it more readable
#[rustfmt::skip]
#[test]
fn vector_small_dims_exhaustive() {
    assert_eq_and_type!(vector![], SVector::<i32, 0>::zeros());
    assert_eq_and_type!(vector![1], Vector1::<i32>::new(1));
    assert_eq_and_type!(vector![1, 2], Vector2::new(1, 2));
    assert_eq_and_type!(vector![1, 2, 3], Vector3::new(1, 2, 3));
    assert_eq_and_type!(vector![1, 2, 3, 4], Vector4::new(1, 2, 3, 4));
    assert_eq_and_type!(vector![1, 2, 3, 4, 5], Vector5::new(1, 2, 3, 4, 5));
    assert_eq_and_type!(vector![1, 2, 3, 4, 5, 6], Vector6::new(1, 2, 3, 4, 5, 6));
}

// Skip rustfmt because it just makes the test bloated without making it more readable
#[rustfmt::skip]
#[test]
fn point_small_dims_exhaustive() {
    assert_eq_and_type!(point![], Point::<i32, 0>::origin());
    assert_eq_and_type!(point![1], Point1::<i32>::new(1));
    assert_eq_and_type!(point![1, 2], Point2::new(1, 2));
    assert_eq_and_type!(point![1, 2, 3], Point3::new(1, 2, 3));
    assert_eq_and_type!(point![1, 2, 3, 4], Point4::new(1, 2, 3, 4));
    assert_eq_and_type!(point![1, 2, 3, 4, 5], Point5::new(1, 2, 3, 4, 5));
    assert_eq_and_type!(point![1, 2, 3, 4, 5, 6], Point6::new(1, 2, 3, 4, 5, 6));
}

#[test]
fn vector_const_fn() {
    // Ensure that vector! can be used in const contexts
    const _: SVector<i32, 0> = vector![];
    const _: Vector1<i32> = vector![1];
    const _: Vector2<i32> = vector![1, 2];
    const _: Vector6<i32> = vector![1, 2, 3, 4, 5, 6];
}

#[test]
fn point_const_fn() {
    // Ensure that vector! can be used in const contexts
    const _: Point<i32, 0> = point![];
    const _: Point1<i32> = point![1];
    const _: Point2<i32> = point![1, 2];
    const _: Point6<i32> = point![1, 2, 3, 4, 5, 6];
}

// Skip rustfmt because it just makes the test bloated without making it more readable
#[rustfmt::skip]
#[test]
fn dvector_small_dims_exhaustive() {
    assert_eq_and_type!(dvector![], DVector::<i32>::zeros(0));
    assert_eq_and_type!(dvector![1], DVector::from_column_slice(&[1]));
    assert_eq_and_type!(dvector![1, 2], DVector::from_column_slice(&[1, 2]));
    assert_eq_and_type!(dvector![1, 2, 3], DVector::from_column_slice(&[1, 2, 3]));
    assert_eq_and_type!(dvector![1, 2, 3, 4], DVector::from_column_slice(&[1, 2, 3, 4]));
    assert_eq_and_type!(dvector![1, 2, 3, 4, 5], DVector::from_column_slice(&[1, 2, 3, 4, 5]));
    assert_eq_and_type!(dvector![1, 2, 3, 4, 5, 6], DVector::from_column_slice(&[1, 2, 3, 4, 5, 6]));
}

#[test]
fn vector_trailing_comma() {
    vector![1, 2,];
    point![1, 2,];
    dvector![1, 2,];
}

#[test]
fn matrix_trybuild_tests() {
    let t = trybuild::TestCases::new();

    // Verify error message when we give a matrix with mismatched dimensions
    t.compile_fail("tests/macros/trybuild/matrix_mismatched_dimensions.rs");
}

#[test]
fn dmatrix_trybuild_tests() {
    let t = trybuild::TestCases::new();

    // Verify error message when we give a matrix with mismatched dimensions
    t.compile_fail("tests/macros/trybuild/dmatrix_mismatched_dimensions.rs");
}

#[test]
fn matrix_builtin_types() {
    // Check that matrix! compiles for all built-in types
    const _: SMatrix<i8, 2, 2> = matrix![0, 1; 2, 3];
    const _: SMatrix<i16, 2, 2> = matrix![0, 1; 2, 3];
    const _: SMatrix<i32, 2, 2> = matrix![0, 1; 2, 3];
    const _: SMatrix<i64, 2, 2> = matrix![0, 1; 2, 3];
    const _: SMatrix<isize, 2, 2> = matrix![0, 1; 2, 3];
    const _: SMatrix<u8, 2, 2> = matrix![0, 1; 2, 3];
    const _: SMatrix<u16, 2, 2> = matrix![0, 1; 2, 3];
    const _: SMatrix<u32, 2, 2> = matrix![0, 1; 2, 3];
    const _: SMatrix<u64, 2, 2> = matrix![0, 1; 2, 3];
    const _: SMatrix<usize, 2, 2> = matrix![0, 1; 2, 3];
    const _: SMatrix<f32, 2, 2> = matrix![0.0, 1.0; 2.0, 3.0];
    const _: SMatrix<f64, 2, 2> = matrix![0.0, 1.0; 2.0, 3.0];
}

#[test]
fn vector_builtin_types() {
    // Check that vector! compiles for all built-in types
    const _: SVector<i8, 4> = vector![0, 1, 2, 3];
    const _: SVector<i16, 4> = vector![0, 1, 2, 3];
    const _: SVector<i32, 4> = vector![0, 1, 2, 3];
    const _: SVector<i64, 4> = vector![0, 1, 2, 3];
    const _: SVector<isize, 4> = vector![0, 1, 2, 3];
    const _: SVector<u8, 4> = vector![0, 1, 2, 3];
    const _: SVector<u16, 4> = vector![0, 1, 2, 3];
    const _: SVector<u32, 4> = vector![0, 1, 2, 3];
    const _: SVector<u64, 4> = vector![0, 1, 2, 3];
    const _: SVector<usize, 4> = vector![0, 1, 2, 3];
    const _: SVector<f32, 4> = vector![0.0, 1.0, 2.0, 3.0];
    const _: SVector<f64, 4> = vector![0.0, 1.0, 2.0, 3.0];
}

#[test]
fn dmatrix_builtin_types() {
    // Check that dmatrix! compiles for all built-in types
    let _: DMatrix<i8> = dmatrix![0, 1; 2, 3];
    let _: DMatrix<i16> = dmatrix![0, 1; 2, 3];
    let _: DMatrix<i32> = dmatrix![0, 1; 2, 3];
    let _: DMatrix<i64> = dmatrix![0, 1; 2, 3];
    let _: DMatrix<isize> = dmatrix![0, 1; 2, 3];
    let _: DMatrix<u8> = dmatrix![0, 1; 2, 3];
    let _: DMatrix<u16> = dmatrix![0, 1; 2, 3];
    let _: DMatrix<u32> = dmatrix![0, 1; 2, 3];
    let _: DMatrix<u64> = dmatrix![0, 1; 2, 3];
    let _: DMatrix<usize> = dmatrix![0, 1; 2, 3];
    let _: DMatrix<f32> = dmatrix![0.0, 1.0; 2.0, 3.0];
    let _: DMatrix<f64> = dmatrix![0.0, 1.0; 2.0, 3.0];
}

#[test]
fn point_builtin_types() {
    // Check that point! compiles for all built-in types
    const _: Point<i8, 4> = point![0, 1, 2, 3];
    const _: Point<i16, 4> = point![0, 1, 2, 3];
    const _: Point<i32, 4> = point![0, 1, 2, 3];
    const _: Point<i64, 4> = point![0, 1, 2, 3];
    const _: Point<isize, 4> = point![0, 1, 2, 3];
    const _: Point<u8, 4> = point![0, 1, 2, 3];
    const _: Point<u16, 4> = point![0, 1, 2, 3];
    const _: Point<u32, 4> = point![0, 1, 2, 3];
    const _: Point<u64, 4> = point![0, 1, 2, 3];
    const _: Point<usize, 4> = point![0, 1, 2, 3];
    const _: Point<f32, 4> = point![0.0, 1.0, 2.0, 3.0];
    const _: Point<f64, 4> = point![0.0, 1.0, 2.0, 3.0];
}

#[test]
fn dvector_builtin_types() {
    // Check that dvector! compiles for all built-in types
    let _: DVector<i8> = dvector![0, 1, 2, 3];
    let _: DVector<i16> = dvector![0, 1, 2, 3];
    let _: DVector<i32> = dvector![0, 1, 2, 3];
    let _: DVector<i64> = dvector![0, 1, 2, 3];
    let _: DVector<isize> = dvector![0, 1, 2, 3];
    let _: DVector<u8> = dvector![0, 1, 2, 3];
    let _: DVector<u16> = dvector![0, 1, 2, 3];
    let _: DVector<u32> = dvector![0, 1, 2, 3];
    let _: DVector<u64> = dvector![0, 1, 2, 3];
    let _: DVector<usize> = dvector![0, 1, 2, 3];
    let _: DVector<f32> = dvector![0.0, 1.0, 2.0, 3.0];
    let _: DVector<f64> = dvector![0.0, 1.0, 2.0, 3.0];
}

/// Black box function that's just used for testing macros with function call expressions.
fn f<T>(x: T) -> T {
    x
}

#[rustfmt::skip]
#[test]
fn matrix_arbitrary_expressions() {
    // Test that matrix! supports arbitrary expressions for its elements
    let a = matrix![1 + 2       ,     2 * 3;
                    4 * f(5 + 6), 7 - 8 * 9];
    let a_expected = Matrix2::new(1 + 2       ,     2 * 3,
                                  4 * f(5 + 6), 7 - 8 * 9);
    assert_eq_and_type!(a, a_expected);
}

#[rustfmt::skip]
#[test]
fn dmatrix_arbitrary_expressions() {
    // Test that dmatrix! supports arbitrary expressions for its elements
    let a = dmatrix![1 + 2       ,     2 * 3;
                     4 * f(5 + 6), 7 - 8 * 9];
    let a_expected = DMatrix::from_row_slice(2, 2, &[1 + 2       ,     2 * 3,
        4 * f(5 + 6), 7 - 8 * 9]);
    assert_eq_and_type!(a, a_expected);
}

#[rustfmt::skip]
#[test]
fn vector_arbitrary_expressions() {
    // Test that vector! supports arbitrary expressions for its elements
    let a = vector![1 + 2, 2 * 3, 4 * f(5 + 6), 7 - 8 * 9];
    let a_expected = Vector4::new(1 + 2, 2 * 3, 4 * f(5 + 6), 7 - 8 * 9);
    assert_eq_and_type!(a, a_expected);
}

#[rustfmt::skip]
#[test]
fn point_arbitrary_expressions() {
    // Test that point! supports arbitrary expressions for its elements
    let a = point![1 + 2, 2 * 3, 4 * f(5 + 6), 7 - 8 * 9];
    let a_expected = Point4::new(1 + 2, 2 * 3, 4 * f(5 + 6), 7 - 8 * 9);
    assert_eq_and_type!(a, a_expected);
}

#[rustfmt::skip]
#[test]
fn dvector_arbitrary_expressions() {
    // Test that dvector! supports arbitrary expressions for its elements
    let a = dvector![1 + 2, 2 * 3, 4 * f(5 + 6), 7 - 8 * 9];
    let a_expected = DVector::from_column_slice(&[1 + 2, 2 * 3, 4 * f(5 + 6), 7 - 8 * 9]);
    assert_eq_and_type!(a, a_expected);
}
