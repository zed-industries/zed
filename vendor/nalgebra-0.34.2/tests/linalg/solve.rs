#![cfg(feature = "proptest-support")]

macro_rules! gen_tests(
    ($module: ident, $scalar: expr) => {
        mod $module {
            use na::{Matrix4, ComplexField};
            #[allow(unused_imports)]
            use crate::core::helper::{RandScalar, RandComplex};
            use crate::proptest::*;
            use proptest::{prop_assert, proptest};

            fn unzero_diagonal<T: ComplexField>(a: &mut Matrix4<T>) {
                for i in 0..4 {
                    if a[(i, i)].clone().norm1() < na::convert(1.0e-7) {
                        a[(i, i)] = T::one();
                    }
                }
            }

            proptest! {
                #[test]
                fn solve_lower_triangular(a in matrix4_($scalar), b in matrix4x5_($scalar)) {
                    let mut a = a;
                    unzero_diagonal(&mut a);
                    let tri = a.lower_triangle();
                    let x   = a.solve_lower_triangular(&b).unwrap();

                    prop_assert!(relative_eq!(tri * x, b, epsilon = 1.0e-7))
                }

                #[test]
                fn solve_upper_triangular(a in matrix4_($scalar), b in matrix4x5_($scalar)) {
                    let mut a = a;
                    unzero_diagonal(&mut a);
                    let tri = a.upper_triangle();
                    let x   = a.solve_upper_triangular(&b).unwrap();

                    prop_assert!(relative_eq!(tri * x, b, epsilon = 1.0e-7))
                }

                #[test]
                fn tr_solve_lower_triangular(a in matrix4_($scalar), b in matrix4x5_($scalar)) {
                    let mut a = a;
                    unzero_diagonal(&mut a);
                    let tri = a.lower_triangle();
                    let x   = a.tr_solve_lower_triangular(&b).unwrap();

                    prop_assert!(relative_eq!(tri.transpose() * x, b, epsilon = 1.0e-7))
                }

                #[test]
                fn tr_solve_upper_triangular(a in matrix4_($scalar), b in matrix4x5_($scalar)) {
                    let mut a = a;
                    unzero_diagonal(&mut a);
                    let tri = a.upper_triangle();
                    let x   = a.tr_solve_upper_triangular(&b).unwrap();

                    prop_assert!(relative_eq!(tri.transpose() * x, b, epsilon = 1.0e-7))
                }
            }
        }
    }
);

gen_tests!(complex, complex_f64());
gen_tests!(f64, PROPTEST_F64);
