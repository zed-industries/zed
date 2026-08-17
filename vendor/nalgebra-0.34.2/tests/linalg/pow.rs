#[cfg(feature = "proptest-support")]
mod proptest_tests {
    macro_rules! gen_tests(
        ($module: ident, $scalar: expr, $scalar_type: ty) => {
            mod $module {
                use na::DMatrix;
                #[allow(unused_imports)]
                use crate::core::helper::{RandScalar, RandComplex};
                use std::cmp;

                use crate::proptest::*;
                use proptest::{prop_assert, proptest};

                proptest! {
                    #[test]
                    fn pow(n in PROPTEST_MATRIX_DIM, p in 0u32..=4) {
                        let n = cmp::max(1, cmp::min(n, 10));
                        let m = DMatrix::<$scalar_type>::new_random(n, n).map(|e| e.0);
                        let m_pow = m.pow(p);
                        let mut expected = m.clone();
                        expected.fill_with_identity();

                        for _ in 0..p {
                            expected = &m * &expected;
                        }

                        prop_assert!(relative_eq!(m_pow, expected, epsilon = 1.0e-5))
                    }

                    #[test]
                    fn pow_static_square_4x4(m in matrix4_($scalar), p in 0u32..=4) {
                        let mut expected = m.clone();
                        let m_pow = m.pow(p);
                        expected.fill_with_identity();

                        for _ in 0..p {
                            expected = &m * &expected;
                        }

                        prop_assert!(relative_eq!(m_pow, expected, epsilon = 1.0e-5))
                    }
                }
            }
        }
    );

    gen_tests!(complex, complex_f64(), RandComplex<f64>);
    gen_tests!(f64, PROPTEST_F64, RandScalar<f64>);
}
