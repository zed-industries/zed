#![cfg(feature = "proptest-support")]

macro_rules! gen_tests(
    ($module: ident, $scalar: expr) => {
            mod $module {
            #[allow(unused_imports)]
            use crate::core::helper::{RandScalar, RandComplex};
            use crate::proptest::*;
            use proptest::{prop_assert, proptest};

            proptest! {
                #[test]
                fn symm_tridiagonal(m in dmatrix_($scalar)) {
                    let m = &m * m.adjoint();
                    let tri = m.clone().symmetric_tridiagonalize();
                    let recomp = tri.recompose();

                    prop_assert!(relative_eq!(m.lower_triangle(), recomp.lower_triangle(), epsilon = 1.0e-7));
                }

                #[test]
                fn symm_tridiagonal_singular(m in dmatrix_($scalar)) {
                    let mut m = &m * m.adjoint();
                    let n = m.nrows();
                    m.row_mut(n / 2).fill(na::zero());
                    m.column_mut(n / 2).fill(na::zero());
                    let tri = m.clone().symmetric_tridiagonalize();
                    let recomp = tri.recompose();

                    prop_assert!(relative_eq!(m.lower_triangle(), recomp.lower_triangle(), epsilon = 1.0e-7));
                }

                #[test]
                fn symm_tridiagonal_static_square(m in matrix4_($scalar)) {
                    let m = m.hermitian_part();
                    let tri = m.symmetric_tridiagonalize();
                    let recomp = tri.recompose();

                    prop_assert!(relative_eq!(m.lower_triangle(), recomp.lower_triangle(), epsilon = 1.0e-7));
                }

                #[test]
                fn symm_tridiagonal_static_square_2x2(m in matrix2_($scalar)) {
                    let m = m.hermitian_part();
                    let tri = m.symmetric_tridiagonalize();
                    let recomp = tri.recompose();

                    prop_assert!(relative_eq!(m.lower_triangle(), recomp.lower_triangle(), epsilon = 1.0e-7));
                }
            }
        }
    }
);

gen_tests!(complex, complex_f64());
gen_tests!(f64, PROPTEST_F64);
