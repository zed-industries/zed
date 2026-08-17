use crate::macros::assert_eq_and_type;
use cool_asserts::assert_panics;
use na::VecStorage;
use nalgebra::dimension::U1;
use nalgebra::{
    DMatrix, DMatrixView, Dyn, Matrix, Matrix2, Matrix4, OMatrix, SMatrix, SMatrixView,
    SMatrixViewMut, Scalar, U2,
};
use nalgebra::{dmatrix, matrix, stack};
use nalgebra_macros::vector;
use num_traits::Zero;

/// Simple implementation that stacks dynamic matrices.
///
/// Used for verifying results of the stack! macro. `None` entries are considered to represent
/// a zero block.
fn stack_dyn<T: Scalar + Zero>(blocks: DMatrix<Option<DMatrix<T>>>) -> DMatrix<T> {
    let row_counts: Vec<usize> = blocks
        .row_iter()
        .map(|block_row| {
            block_row
                .iter()
                .map(|block_or_implicit_zero| {
                    block_or_implicit_zero.as_ref().map(|block| block.nrows())
                })
                .reduce(|nrows1, nrows2| match (nrows1, nrows2) {
                    (Some(_), None) => nrows1,
                    (None, Some(_)) => nrows2,
                    (None, None) => None,
                    (Some(nrows1), Some(nrows2)) if nrows1 == nrows2 => Some(nrows1),
                    _ => panic!("Number of rows must be consistent in each block row"),
                })
                .unwrap_or(Some(0))
                .expect("Each block row must have at least one entry which is not a zero literal")
        })
        .collect();
    let col_counts: Vec<usize> = blocks
        .column_iter()
        .map(|block_col| {
            block_col
                .iter()
                .map(|block_or_implicit_zero| {
                    block_or_implicit_zero.as_ref().map(|block| block.ncols())
                })
                .reduce(|ncols1, ncols2| match (ncols1, ncols2) {
                    (Some(_), None) => ncols1,
                    (None, Some(_)) => ncols2,
                    (None, None) => None,
                    (Some(ncols1), Some(ncols2)) if ncols1 == ncols2 => Some(ncols1),
                    _ => panic!("Number of columns must be consistent in each block column"),
                })
                .unwrap_or(Some(0))
                .expect(
                    "Each block column must have at least one entry which is not a zero literal",
                )
        })
        .collect();

    let nrows_total = row_counts.iter().sum();
    let ncols_total = col_counts.iter().sum();
    let mut output = DMatrix::zeros(nrows_total, ncols_total);

    let mut col_offset = 0;
    for j in 0..blocks.ncols() {
        let mut row_offset = 0;
        for i in 0..blocks.nrows() {
            if let Some(input_ij) = &blocks[(i, j)] {
                let (block_nrows, block_ncols) = input_ij.shape();
                output
                    .view_mut((row_offset, col_offset), (block_nrows, block_ncols))
                    .copy_from(&input_ij);
            }
            row_offset += row_counts[i];
        }
        col_offset += col_counts[j];
    }

    output
}

macro_rules! stack_dyn_convert_to_dmatrix_option {
    (0) => {
        None
    };
    ($entry:expr) => {
        Some($entry.as_view::<Dyn, Dyn, U1, Dyn>().clone_owned())
    };
}

/// Helper macro that compares the result of stack! with a simplified implementation that
/// works only with heap-allocated data.
///
/// This implementation is essentially radically different to the implementation in stack!,
/// so if they both match, then it's a good sign that the stack! impl is correct.
macro_rules! verify_stack {
    ($matrix_type:ty ; [$($($entry:expr),*);*]) => {
        {
            // Our input has the same syntax as the stack! macro (and matrix! macro, for that matter)
            let stack_result: $matrix_type = stack![$($($entry),*);*];
            // Use the dmatrix! macro to nest matrices into each other
            let dyn_result = stack_dyn(
                dmatrix![$($(stack_dyn_convert_to_dmatrix_option!($entry)),*);*]
            );
            // println!("{}", stack_result);
            // println!("{}", dyn_result);
            assert_eq!(stack_result, dyn_result);
        }
    }
}

#[test]
fn stack_simple() {
    let m = stack![
        Matrix2::<usize>::identity(), 0;
        0, &Matrix2::identity();
    ];

    assert_eq_and_type!(m, Matrix4::identity());
}

#[test]
fn stack_diag() {
    let m = stack![
        0, matrix![1, 2; 3, 4;];
        matrix![5, 6; 7, 8;], 0;
    ];

    let res = matrix![
        0, 0, 1, 2;
        0, 0, 3, 4;
        5, 6, 0, 0;
        7, 8, 0, 0;
    ];

    assert_eq_and_type!(m, res);
}

#[test]
fn stack_dynamic() {
    let m = stack![
        matrix![ 1, 2; 3, 4; ], 0;
        0, dmatrix![7, 8, 9; 10, 11, 12; ];
    ];

    let res = dmatrix![
        1, 2, 0, 0, 0;
        3, 4, 0, 0, 0;
        0, 0, 7, 8, 9;
        0, 0, 10, 11, 12;
    ];

    assert_eq_and_type!(m, res);
}

#[test]
fn stack_nested() {
    let m = stack![
        stack![ matrix![1, 2; 3, 4;]; matrix![5, 6;]],
        stack![ matrix![7;9;10;], matrix![11; 12; 13;] ];
    ];

    let res = matrix![
        1, 2, 7, 11;
        3, 4, 9, 12;
        5, 6, 10, 13;
    ];

    assert_eq_and_type!(m, res);
}

#[test]
fn stack_single() {
    let a = matrix![1, 2; 3, 4];
    let b = stack![a];

    assert_eq_and_type!(a, b);
}

#[test]
fn stack_single_row() {
    let a = matrix![1, 2; 3, 4];
    let m = stack![a, a];

    let res = matrix![
        1, 2, 1, 2;
        3, 4, 3, 4;
    ];

    assert_eq_and_type!(m, res);
}

#[test]
fn stack_single_col() {
    let a = matrix![1, 2; 3, 4];
    let m = stack![a; a];

    let res = matrix![
        1, 2;
        3, 4;
        1, 2;
        3, 4;
    ];

    assert_eq_and_type!(m, res);
}

#[test]
#[rustfmt::skip]
fn stack_expr() {
    let a = matrix![1, 2; 3, 4];
    let b = matrix![5, 6; 7, 8];
    let m = stack![a + b; 2i32 * b - a];

    let res = matrix![
         6,  8;
        10, 12;
         9, 10;
        11, 12;
    ];

    assert_eq_and_type!(m, res);
}

#[test]
fn stack_edge_cases() {
    {
        // Empty stack should return zero matrix with specified type
        let _: SMatrix<i32, 0, 0> = stack![];
        let _: SMatrix<f64, 0, 0> = stack![];
    }

    {
        // Case suggested by @tpdickso: https://github.com/dimforge/nalgebra/pull/1080#discussion_r1435871752
        let a = matrix![1, 2;
                        3, 4];
        let b = DMatrix::from_data(VecStorage::new(Dyn(2), Dyn(0), vec![]));
        assert_eq!(
            stack![a, 0;
                   0, b],
            matrix![1, 2;
                    3, 4;
                    0, 0;
                    0, 0]
        );
    }
}

#[rustfmt::skip]
#[test]
fn stack_many_tests() {
    // s prefix means static, d prefix means dynamic
    // Static matrices
    let s_0x0: SMatrix<i32, 0, 0> = matrix![];
    let s_0x1: SMatrix<i32, 0, 1> = Matrix::default();
    let s_1x0: SMatrix<i32, 1, 0> = Matrix::default();
    let s_1x1: SMatrix<i32, 1, 1> = matrix![1];
    let s_2x2: SMatrix<i32, 2, 2> = matrix![6, 7; 8, 9];
    let s_2x3: SMatrix<i32, 2, 3> = matrix![16, 17, 18; 19, 20, 21];
    let s_3x3: SMatrix<i32, 3, 3> = matrix![28, 29, 30; 31, 32, 33; 34, 35, 36];

    // Dynamic matrices
    let d_0x0: DMatrix<i32> = dmatrix![];
    let d_1x2: DMatrix<i32> = dmatrix![9, 10];
    let d_2x2: DMatrix<i32> = dmatrix![5, 6; 7, 8];
    let d_4x4: DMatrix<i32> = dmatrix![10, 11, 12, 13; 14, 15, 16, 17; 18, 19, 20, 21; 22, 23, 24, 25];

    // Check for weirdness with matrices that have zero row/cols
    verify_stack!(SMatrix<_, 0, 0>; [s_0x0]);
    verify_stack!(SMatrix<_, 0, 1>; [s_0x1]);
    verify_stack!(SMatrix<_, 1, 0>; [s_1x0]);
    verify_stack!(SMatrix<_, 0, 0>; [s_0x0; s_0x0]);
    verify_stack!(SMatrix<_, 0, 0>; [s_0x0, s_0x0; s_0x0, s_0x0]);
    verify_stack!(SMatrix<_, 0, 2>; [s_0x1, s_0x1]);
    verify_stack!(SMatrix<_, 2, 0>; [s_1x0; s_1x0]);
    verify_stack!(SMatrix<_, 1, 0>; [s_1x0, s_1x0]);
    verify_stack!(DMatrix<_>; [d_0x0]);

    // Horizontal stacking
    verify_stack!(SMatrix<_, 1, 2>; [s_1x1, s_1x1]);
    verify_stack!(SMatrix<_, 2, 4>; [s_2x2, s_2x2]);
    verify_stack!(DMatrix<_>; [d_1x2, d_1x2]);

    // Vertical stacking
    verify_stack!(SMatrix<_, 2, 1>; [s_1x1; s_1x1]);
    verify_stack!(SMatrix<_, 4, 2>; [s_2x2; s_2x2]);
    verify_stack!(DMatrix<_>; [d_2x2; d_2x2]);

    // Mix static and dynamic matrices
    verify_stack!(OMatrix<_, U2, Dyn>; [s_2x2, d_2x2]);
    verify_stack!(OMatrix<_, Dyn, U2>; [s_2x2; d_1x2]);

    // Stack more than two matrices
    verify_stack!(SMatrix<_, 1, 3>; [s_1x1, s_1x1, s_1x1]);
    verify_stack!(DMatrix<_>; [d_1x2, d_1x2, d_1x2]);

    // Slightly larger dims
    verify_stack!(SMatrix<_, 3, 6>; [s_3x3, s_3x3]);
    verify_stack!(DMatrix<_>; [d_4x4; d_4x4]);
    verify_stack!(SMatrix<_, 4, 7>; [s_2x2, s_2x3, d_2x2;
                                     d_2x2, s_2x3, s_2x2]);

    // Mix of references and owned
    verify_stack!(OMatrix<_, Dyn, U2>; [&s_2x2; &d_1x2]);
    verify_stack!(SMatrix<_, 4, 7>; [ s_2x2, &s_2x3, d_2x2;
                                     &d_2x2, s_2x3, &s_2x2]);

    // Views
    let s_2x2_v: SMatrixView<_, 2, 2> = s_2x2.as_view();
    let s_2x3_v: SMatrixView<_, 2, 3> = s_2x3.as_view();
    let d_2x2_v: DMatrixView<_> = d_2x2.as_view();
    let mut s_2x2_vm = s_2x2.clone();
    let s_2x2_vm: SMatrixViewMut<_, 2, 2> = s_2x2_vm.as_view_mut();
    let mut s_2x3_vm = s_2x3.clone();
    let s_2x3_vm: SMatrixViewMut<_, 2, 3> = s_2x3_vm.as_view_mut();
    verify_stack!(SMatrix<_, 4, 7>; [ s_2x2_vm, &s_2x3_vm,  d_2x2_v;
                                      &d_2x2_v,   s_2x3_v, &s_2x2_v]);

    // Expressions
    let matrix_fn = |matrix: &DMatrix<_>| matrix.map(|x_ij| x_ij * 3);
    verify_stack!(SMatrix<_, 2, 5>; [ 2 * s_2x2 - 3 * &d_2x2, s_2x3 + 2 * s_2x3]);
    verify_stack!(DMatrix<_>; [ 2 * matrix_fn(&d_2x2) ]);
    verify_stack!(SMatrix<_, 2, 5>; [ (|matrix| 4 * matrix)(s_2x2), s_2x3 ]);
}

#[test]
fn stack_trybuild_tests() {
    let t = trybuild::TestCases::new();

    // Verify error message when a row or column only contains a zero entry
    t.compile_fail("tests/macros/trybuild/stack_empty_row.rs");
    t.compile_fail("tests/macros/trybuild/stack_empty_col.rs");
    t.compile_fail("tests/macros/trybuild/stack_incompatible_block_dimensions.rs");
    t.compile_fail("tests/macros/trybuild/stack_incompatible_block_dimensions2.rs");
}

#[test]
fn stack_mismatched_dimensions_runtime_panics() {
    // s prefix denotes static, d dynamic
    let s_2x2 = matrix![1, 2; 3, 4];
    let d_2x3 = dmatrix![5, 6, 7; 8, 9, 10];
    let d_1x2 = dmatrix![11, 12];
    let d_1x3 = dmatrix![13, 14, 15];

    assert_panics!(
        stack![s_2x2, d_1x2],
        includes("All blocks in block row 0 must have the same number of rows")
    );

    assert_panics!(
        stack![s_2x2; d_2x3],
        includes("All blocks in block column 0 must have the same number of columns")
    );

    assert_panics!(
        stack![s_2x2, s_2x2; d_1x2, d_2x3],
        includes("All blocks in block row 1 must have the same number of rows")
    );

    assert_panics!(
        stack![s_2x2, s_2x2; d_1x2, d_1x3],
        includes("All blocks in block column 1 must have the same number of columns")
    );

    assert_panics!(
        {
            // Edge case suggested by @tpdickso: https://github.com/dimforge/nalgebra/pull/1080#discussion_r1435871752
            let d_3x0 = DMatrix::from_data(VecStorage::new(Dyn(3), Dyn(0), Vec::<i32>::new()));
            stack![s_2x2, d_3x0]
        },
        includes("All blocks in block row 0 must have the same number of rows")
    );
}

#[test]
fn stack_test_builtin_types() {
    // Other than T: Zero, there's nothing type-specific in the logic for stack!
    // These tests are just sanity tests, to make sure it works with the common built-in types
    let a = matrix![1, 2; 3, 4];
    let b = vector![5, 6];
    let c = matrix![7, 8];

    let expected = matrix![ 1, 2, 5;
                            3, 4, 6;
                            7, 8, 0 ];

    macro_rules! check_builtin {
        ($T:ty) => {{
            // Cannot use .cast::<$T> because we cannot convert between unsigned and signed
            let stacked = stack![a.map(|a_ij| a_ij as $T), b.map(|b_ij| b_ij as $T);
                                 c.map(|c_ij| c_ij as $T),                        0];
            assert_eq!(stacked, expected.map(|e_ij| e_ij as $T));
        }}
    }

    check_builtin!(i8);
    check_builtin!(i16);
    check_builtin!(i32);
    check_builtin!(i64);
    check_builtin!(i128);
    check_builtin!(u8);
    check_builtin!(u16);
    check_builtin!(u32);
    check_builtin!(u64);
    check_builtin!(u128);
    check_builtin!(f32);
    check_builtin!(f64);
}

#[test]
fn stack_test_complex() {
    use num_complex::Complex as C;
    type C32 = C<f32>;
    let a = matrix![C::new(1.0, 1.0), C::new(2.0, 2.0); C::new(3.0, 3.0), C::new(4.0, 4.0)];
    let b = vector![C::new(5.0, 5.0), C::new(6.0, 6.0)];
    let c = matrix![C::new(7.0, 7.0), C::new(8.0, 8.0)];

    let expected = matrix![ 1, 2, 5;
                            3, 4, 6;
                            7, 8, 0 ]
    .map(|x| C::new(x as f64, x as f64));

    assert_eq!(stack![a, b; c, 0], expected);
    assert_eq!(
        stack![a.cast::<C32>(), b.cast::<C32>(); c.cast::<C32>(), 0],
        expected.cast::<C32>()
    );
}
