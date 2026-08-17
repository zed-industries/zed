//! Tests for proptest-related functionality.

#![allow(dead_code)]

use nalgebra::allocator::Allocator;
use nalgebra::base::dimension::*;
use nalgebra::proptest::{DimRange, MatrixStrategy};
use nalgebra::{
    DMatrix, DVector, DefaultAllocator, Dim, DualQuaternion, Isometry2, Isometry3, Matrix3,
    OMatrix, Point2, Point3, Quaternion, Rotation2, Rotation3, Scalar, Similarity3, Translation2,
    Translation3, U3, U4, UnitComplex, UnitDualQuaternion, UnitQuaternion, Vector3,
};
use num_complex::Complex;
use proptest::prelude::*;
use proptest::strategy::{Strategy, ValueTree};
use proptest::test_runner::TestRunner;
use std::ops::RangeInclusive;

pub const PROPTEST_MATRIX_DIM: RangeInclusive<usize> = 1..=20;
pub const PROPTEST_F64: RangeInclusive<f64> = -100.0..=100.0;

pub use nalgebra::proptest::{matrix, vector};

pub fn point2() -> impl Strategy<Value = Point2<f64>> {
    vector2().prop_map(|v| Point2::from(v))
}

pub fn point3() -> impl Strategy<Value = Point3<f64>> {
    vector3().prop_map(|v| Point3::from(v))
}

pub fn translation2() -> impl Strategy<Value = Translation2<f64>> {
    vector2().prop_map(|v| Translation2::from(v))
}

pub fn translation3() -> impl Strategy<Value = Translation3<f64>> {
    vector3().prop_map(|v| Translation3::from(v))
}

pub fn rotation2() -> impl Strategy<Value = Rotation2<f64>> {
    PROPTEST_F64.prop_map(|v| Rotation2::new(v))
}

pub fn rotation3() -> impl Strategy<Value = Rotation3<f64>> {
    vector3().prop_map(|v| Rotation3::new(v))
}

pub fn unit_complex() -> impl Strategy<Value = UnitComplex<f64>> {
    PROPTEST_F64.prop_map(|v| UnitComplex::new(v))
}

pub fn isometry2() -> impl Strategy<Value = Isometry2<f64>> {
    vector3().prop_map(|v| Isometry2::new(v.xy(), v.z))
}

pub fn isometry3() -> impl Strategy<Value = Isometry3<f64>> {
    vector6().prop_map(|v| Isometry3::new(v.xyz(), Vector3::new(v.w, v.a, v.b)))
}

// pub fn similarity2() -> impl Strategy<Value = Similarity2<f64>> {
//     vector4().prop_map(|v| Similarity2::new(v.xy(), v.z, v.w))
// }

pub fn similarity3() -> impl Strategy<Value = Similarity3<f64>> {
    vector(PROPTEST_F64, Const::<7>)
        .prop_map(|v| Similarity3::new(v.xyz(), Vector3::new(v[3], v[4], v[5]), v[6]))
}

pub fn unit_dual_quaternion() -> impl Strategy<Value = UnitDualQuaternion<f64>> {
    isometry3().prop_map(|iso| UnitDualQuaternion::from_isometry(&iso))
}

pub fn dual_quaternion() -> impl Strategy<Value = DualQuaternion<f64>> {
    vector(PROPTEST_F64, Const::<8>).prop_map(|v| {
        DualQuaternion::from_real_and_dual(
            Quaternion::new(v[0], v[1], v[2], v[3]),
            Quaternion::new(v[4], v[5], v[6], v[7]),
        )
    })
}

pub fn quaternion() -> impl Strategy<Value = Quaternion<f64>> {
    vector4().prop_map(|v| Quaternion::from(v))
}

pub fn unit_quaternion() -> impl Strategy<Value = UnitQuaternion<f64>> {
    vector3().prop_map(|v| UnitQuaternion::new(v))
}

pub fn complex_f64() -> impl Strategy<Value = Complex<f64>> + Clone {
    vector(PROPTEST_F64, Const::<2>).prop_map(|v| Complex::new(v.x, v.y))
}

pub fn dmatrix() -> impl Strategy<Value = DMatrix<f64>> {
    matrix(PROPTEST_F64, PROPTEST_MATRIX_DIM, PROPTEST_MATRIX_DIM)
}

pub fn dvector() -> impl Strategy<Value = DVector<f64>> {
    vector(PROPTEST_F64, PROPTEST_MATRIX_DIM)
}

pub fn dmatrix_<ScalarStrategy>(
    scalar_strategy: ScalarStrategy,
) -> impl Strategy<Value = OMatrix<ScalarStrategy::Value, Dyn, Dyn>>
where
    ScalarStrategy: Strategy + Clone + 'static,
    ScalarStrategy::Value: Scalar,
    DefaultAllocator: Allocator<Dyn, Dyn>,
{
    matrix(scalar_strategy, PROPTEST_MATRIX_DIM, PROPTEST_MATRIX_DIM)
}

// pub fn dvector_<T>(range: RangeInclusive<T>) -> impl Strategy<Value = DVector<T>>
// where
//     RangeInclusive<T>: Strategy<Value = T>,
//     T: Scalar + PartialEq + Copy,
//     DefaultAllocator: Allocator<Dyn>,
// {
//     vector(range, PROPTEST_MATRIX_DIM)
// }

macro_rules! define_strategies(
    ($($strategy_: ident $strategy: ident<$nrows: literal, $ncols: literal>),*) => {$(
        pub fn $strategy() -> impl Strategy<Value = OMatrix<f64, Const<$nrows>, Const<$ncols>>> {
            matrix(PROPTEST_F64, Const::<$nrows>, Const::<$ncols>)
        }

        pub fn $strategy_<ScalarStrategy>(scalar_strategy: ScalarStrategy) -> impl Strategy<Value = OMatrix<ScalarStrategy::Value, Const<$nrows>, Const<$ncols>>>
            where
                ScalarStrategy: Strategy + Clone + 'static,
                ScalarStrategy::Value: Scalar, {
            matrix(scalar_strategy, Const::<$nrows>, Const::<$ncols>)
        }
    )*}
);

define_strategies!(
    matrix1_ matrix1<1, 1>,
    matrix2_ matrix2<2, 2>,
    matrix3_ matrix3<3, 3>,
    matrix4_ matrix4<4, 4>,
    matrix5_ matrix5<5, 5>,
    matrix6_ matrix6<6, 6>,

    matrix5x2_ matrix5x2<5, 2>,
    matrix2x5_ matrix2x5<2, 5>,
    matrix5x3_ matrix5x3<5, 3>,
    matrix3x5_ matrix3x5<3, 5>,
    matrix5x4_ matrix5x4<5, 4>,
    matrix4x5_ matrix4x5<4, 5>,

    vector1_ vector1<1, 1>,
    vector2_ vector2<2, 1>,
    vector3_ vector3<3, 1>,
    vector4_ vector4<4, 1>,
    vector5_ vector5<5, 1>,
    vector6_ vector6<6, 1>
);

/// Generate a proptest that tests that all matrices generated with the
/// provided rows and columns conform to the constraints defined by the
/// input.
macro_rules! generate_matrix_sanity_test {
    ($test_name:ident, $rows:expr, $cols:expr) => {
        proptest! {
            #[test]
            fn $test_name(a in matrix(-5 ..= 5i32, $rows, $cols)) {
                // let a: OMatrix<_, $rows, $cols> = a;
                let rows_range = DimRange::from($rows);
                let cols_range = DimRange::from($cols);
                prop_assert!(a.nrows() >= rows_range.lower_bound().value()
                          && a.nrows() <= rows_range.upper_bound().value());
                prop_assert!(a.ncols() >= cols_range.lower_bound().value()
                          && a.ncols() <= cols_range.upper_bound().value());
                prop_assert!(a.iter().all(|x_ij| *x_ij >= -5 && *x_ij <= 5));
            }
        }
    };
}

// Test all fixed-size matrices with row/col dimensions up to 3
generate_matrix_sanity_test!(test_matrix_u0_u0, Const::<0>, Const::<0>);
generate_matrix_sanity_test!(test_matrix_u1_u0, Const::<1>, Const::<0>);
generate_matrix_sanity_test!(test_matrix_u0_u1, Const::<0>, Const::<1>);
generate_matrix_sanity_test!(test_matrix_u1_u1, Const::<1>, Const::<1>);
generate_matrix_sanity_test!(test_matrix_u2_u1, Const::<2>, Const::<1>);
generate_matrix_sanity_test!(test_matrix_u1_u2, Const::<1>, Const::<2>);
generate_matrix_sanity_test!(test_matrix_u2_u2, Const::<2>, Const::<2>);
generate_matrix_sanity_test!(test_matrix_u3_u2, Const::<3>, Const::<2>);
generate_matrix_sanity_test!(test_matrix_u2_u3, Const::<2>, Const::<3>);
generate_matrix_sanity_test!(test_matrix_u3_u3, Const::<3>, Const::<3>);

// Similarly test all heap-allocated but fixed dim ranges
generate_matrix_sanity_test!(test_matrix_0_0, 0, 0);
generate_matrix_sanity_test!(test_matrix_0_1, 0, 1);
generate_matrix_sanity_test!(test_matrix_1_0, 1, 0);
generate_matrix_sanity_test!(test_matrix_1_1, 1, 1);
generate_matrix_sanity_test!(test_matrix_2_1, 2, 1);
generate_matrix_sanity_test!(test_matrix_1_2, 1, 2);
generate_matrix_sanity_test!(test_matrix_2_2, 2, 2);
generate_matrix_sanity_test!(test_matrix_3_2, 3, 2);
generate_matrix_sanity_test!(test_matrix_2_3, 2, 3);
generate_matrix_sanity_test!(test_matrix_3_3, 3, 3);

// Test arbitrary inputs
generate_matrix_sanity_test!(test_matrix_input_1, Const::<5>, 1..=5);
generate_matrix_sanity_test!(test_matrix_input_2, 3..=4, 1..=5);
generate_matrix_sanity_test!(test_matrix_input_3, 1..=2, Const::<3>);
generate_matrix_sanity_test!(test_matrix_input_4, 3, Const::<4>);

#[test]
fn test_matrix_output_types() {
    // Test that the dimension types are correct for the given inputs
    let _: MatrixStrategy<_, U3, U4> = matrix(-5..5, Const::<3>, Const::<4>);
    let _: MatrixStrategy<_, U3, U3> = matrix(-5..5, Const::<3>, Const::<3>);
    let _: MatrixStrategy<_, U3, Dyn> = matrix(-5..5, Const::<3>, 1..=5);
    let _: MatrixStrategy<_, Dyn, U3> = matrix(-5..5, 1..=5, Const::<3>);
    let _: MatrixStrategy<_, Dyn, Dyn> = matrix(-5..5, 1..=5, 1..=5);
}

// Below we have some tests to ensure that specific instances of OMatrix are usable
// in a typical proptest scenario where we (implicitly) use the `Arbitrary` trait
proptest! {
    #[test]
    fn ensure_arbitrary_test_compiles_matrix3(_: Matrix3<i32>) {}

    #[test]
    fn ensure_arbitrary_test_compiles_matrixmn_u3_dynamic(_: OMatrix<i32, U3, Dyn>) {}

    #[test]
    fn ensure_arbitrary_test_compiles_matrixmn_dynamic_u3(_: OMatrix<i32, Dyn, U3>) {}

    #[test]
    fn ensure_arbitrary_test_compiles_dmatrix(_: DMatrix<i32>) {}

    #[test]
    fn ensure_arbitrary_test_compiles_vector3(_: Vector3<i32>) {}

    #[test]
    fn ensure_arbitrary_test_compiles_dvector(_: DVector<i32>) {}
}

#[test]
fn matrix_shrinking_satisfies_constraints() {
    // We use a deterministic test runner to make the test "stable".
    let mut runner = TestRunner::deterministic();

    let strategy = matrix(-1..=2, 1..=3, 2..=4);

    let num_matrices = 25;

    macro_rules! maybeprintln {
        ($($arg:tt)*) => {
            // Uncomment the below line to enable printing of matrix sequences. This is handy
            // for manually inspecting the sequences of simplified matrices.
            // println!($($arg)*)
        };
    }

    maybeprintln!("========================== (begin generation process)");

    for _ in 0..num_matrices {
        let mut tree = strategy
            .new_tree(&mut runner)
            .expect("Tree generation should not fail.");

        let mut current = Some(tree.current());

        maybeprintln!("------------------");

        while let Some(matrix) = current {
            maybeprintln!("{}", matrix);

            assert!(
                matrix.iter().all(|&v| v >= -1 && v <= 2),
                "All matrix elements must satisfy constraints"
            );
            assert!(
                matrix.nrows() >= 1 && matrix.nrows() <= 3,
                "Number of rows in matrix must satisfy constraints."
            );
            assert!(
                matrix.ncols() >= 2 && matrix.ncols() <= 4,
                "Number of columns in matrix must satisfy constraints."
            );

            current = if tree.simplify() {
                Some(tree.current())
            } else {
                None
            }
        }
    }

    maybeprintln!("========================== (end of generation process)");
}

#[cfg(feature = "slow-tests")]
mod slow {
    use super::*;
    use itertools::Itertools;
    use std::collections::HashSet;
    use std::iter::repeat;

    #[cfg(feature = "slow-tests")]
    #[test]
    fn matrix_samples_all_possible_outputs() {
        // Test that the proptest generation covers all possible outputs for a small space of inputs
        // given enough samples.

        // We use a deterministic test runner to make the test "stable".
        let mut runner = TestRunner::deterministic();

        // This number needs to be high enough so that we with high probability sample
        // all possible cases
        let num_generated_matrices = 200000;

        let values = -1..=1;
        let rows = 0..=2;
        let cols = 0..=3;
        let strategy = matrix(values.clone(), rows.clone(), cols.clone());

        // Enumerate all possible combinations
        let mut all_combinations = HashSet::new();
        for nrows in rows {
            for ncols in cols.clone() {
                // For the given number of rows and columns
                let n_values = nrows * ncols;

                if n_values == 0 {
                    // If we have zero rows or columns, the set of matrices with the given
                    // rows and columns is a single element: an empty matrix
                    all_combinations.insert(DMatrix::from_row_slice(nrows, ncols, &[]));
                } else {
                    // Otherwise, we need to sample all possible matrices.
                    // To do this, we generate the values as the (multi) Cartesian product
                    // of the value sets. For example, for a 2x2 matrices, we consider
                    // all possible 4-element arrays that the matrices can take by
                    // considering all elements in the cartesian product
                    //  V x V x V x V
                    // where V is the set of eligible values, e.g. V := -1 ..= 1
                    for matrix_values in repeat(values.clone())
                        .take(n_values)
                        .multi_cartesian_product()
                    {
                        all_combinations.insert(DMatrix::from_row_slice(
                            nrows,
                            ncols,
                            &matrix_values,
                        ));
                    }
                }
            }
        }

        let mut visited_combinations = HashSet::new();
        for _ in 0..num_generated_matrices {
            let tree = strategy
                .new_tree(&mut runner)
                .expect("Tree generation should not fail");
            let matrix = tree.current();
            visited_combinations.insert(matrix.clone());
        }

        assert_eq!(
            visited_combinations, all_combinations,
            "Did not sample all possible values."
        );
    }
}
