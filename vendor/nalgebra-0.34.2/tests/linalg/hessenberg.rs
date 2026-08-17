#![cfg(feature = "proptest-support")]

use na::Matrix2;

#[test]
fn hessenberg_simple() {
    let m = Matrix2::new(1.0, 0.0, 1.0, 3.0);
    let hess = m.hessenberg();
    let (p, h) = hess.unpack();
    assert!(relative_eq!(m, p * h * p.transpose(), epsilon = 1.0e-7))
}

macro_rules! gen_tests(
    ($module: ident, $scalar: expr, $scalar_type: ty) => {
         mod $module {
            use na::DMatrix;
            #[allow(unused_imports)]
            use crate::core::helper::{RandScalar, RandComplex};

            use crate::proptest::*;
            use proptest::{prop_assert, proptest};

            proptest! {
                #[test]
                fn hessenberg(n in PROPTEST_MATRIX_DIM) {
                    let m  = DMatrix::<$scalar_type>::new_random(n, n).map(|e| e.0);
                    let hess = m.clone().hessenberg();
                    let (p, h) = hess.unpack();
                    prop_assert!(relative_eq!(m, &p * h * p.adjoint(), epsilon = 1.0e-7))
                }

                #[test]
                fn hessenberg_static_mat2(m in matrix2_($scalar)) {
                    let hess = m.hessenberg();
                    let (p, h) = hess.unpack();
                    prop_assert!(relative_eq!(m, p * h * p.adjoint(), epsilon = 1.0e-7))
                }

                #[test]
                fn hessenberg_static(m in matrix4_($scalar)) {
                    let hess = m.hessenberg();
                    let (p, h) = hess.unpack();
                    prop_assert!(relative_eq!(m, p * h * p.adjoint(), epsilon = 1.0e-7))
                }
            }
         }
    }
);

gen_tests!(complex, complex_f64(), RandComplex<f64>);
gen_tests!(f64, PROPTEST_F64, RandScalar<f64>);
