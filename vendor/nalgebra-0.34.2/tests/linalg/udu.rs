use na::Matrix3;

#[test]
#[rustfmt::skip]
fn udu_simple() {
    let m = Matrix3::new(
        2.0, -1.0,  0.0,
       -1.0,  2.0, -1.0,
        0.0, -1.0,  2.0);

    let udu = m.udu().unwrap();
    
    // Rebuild
    let p = udu.u * udu.d_matrix() * udu.u.transpose();

    assert!(relative_eq!(m, p, epsilon = 3.0e-16));
}

#[test]
#[should_panic]
#[rustfmt::skip]
fn udu_non_sym_panic() {
    let m = Matrix3::new(
        2.0, -1.0,  0.0,
        1.0, -2.0,  3.0,
       -2.0,  1.0,  0.3);

    let udu = m.udu().unwrap();
    // Rebuild
    let p = udu.u * udu.d_matrix() * udu.u.transpose();

    assert!(relative_eq!(m, p, epsilon = 3.0e-16));
}

#[cfg(feature = "proptest-support")]
mod proptest_tests {
    #[allow(unused_imports)]
    use crate::core::helper::{RandComplex, RandScalar};

    macro_rules! gen_tests(
        ($module: ident, $scalar: expr) => {
            mod $module {
                #[allow(unused_imports)]
                use crate::core::helper::{RandScalar, RandComplex};
                use crate::proptest::*;
                use proptest::{prop_assert, proptest};

                proptest! {
                    #[test]
                    fn udu(m in dmatrix_($scalar)) {
                        let m = &m * m.adjoint();

                        if let Some(udu) = m.clone().udu() {
                            let p = &udu.u * &udu.d_matrix() * &udu.u.transpose();
                            println!("m: {}, p: {}", m, p);

                            prop_assert!(relative_eq!(m, p, epsilon = 1.0e-7));
                        }
                    }

                    #[test]
                    fn udu_static(m in matrix4_($scalar)) {
                        let m = m.hermitian_part();

                        if let Some(udu) = m.udu() {
                            let p = udu.u * udu.d_matrix() * udu.u.transpose();
                            prop_assert!(relative_eq!(m, p, epsilon = 1.0e-7));
                        }
                    }
                }
            }
        }
    );

    gen_tests!(f64, PROPTEST_F64);
}
