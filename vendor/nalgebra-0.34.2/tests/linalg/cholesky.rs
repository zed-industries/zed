#![cfg(all(feature = "proptest-support", feature = "debug"))]

#[test]
// #[rustfmt::skip]
fn cholesky_with_substitute() {
    // Make a tiny covariance matrix with a small covariance value.
    let m = na::Matrix2::new(1.0, f64::NAN, 1.0, 1e-32);
    // Show that the cholesky fails for our matrix. We then try again with a substitute.
    assert!(na::Cholesky::new(m).is_none());
    // ...and show that we get some result this time around.
    assert!(na::Cholesky::new_with_substitute(m, 1e-8).is_some());
}

macro_rules! gen_tests(
    ($module: ident, $scalar: ty) => {
        mod $module {
            use na::debug::RandomSDP;
            use na::dimension::{Const, Dyn};
            use na::{DMatrix, DVector, Matrix4x3, Vector4};
            use rand::{random, distr::uniform::{UniformUsize, UniformSampler}};
            use simba::scalar::ComplexField;
            #[allow(unused_imports)]
            use crate::core::helper::{RandScalar, RandComplex};

            use crate::proptest::*;
            use proptest::{prop_assert, proptest};

            fn random_usize() -> usize {
                let sampler = UniformUsize::new(usize::MIN, usize::MAX).unwrap();
                sampler.sample(&mut rand::rng())
            }

            proptest! {
                #[test]
                fn cholesky(n in PROPTEST_MATRIX_DIM) {
                    let m = RandomSDP::new(Dyn(n), || random::<$scalar>().0).unwrap();
                    let l = m.clone().cholesky().unwrap().unpack();
                    prop_assert!(relative_eq!(m, &l * l.adjoint(), epsilon = 1.0e-7));
                }

                #[test]
                fn cholesky_static(_n in PROPTEST_MATRIX_DIM) {
                    let m = RandomSDP::new(Const::<4>, || random::<$scalar>().0).unwrap();
                    let chol = m.cholesky().unwrap();
                    let l    = chol.unpack();

                    prop_assert!(relative_eq!(m, &l * l.adjoint(), epsilon = 1.0e-7));
                }

                #[test]
                fn cholesky_solve(n in PROPTEST_MATRIX_DIM, nb in PROPTEST_MATRIX_DIM) {
                    let m = RandomSDP::new(Dyn(n), || random::<$scalar>().0).unwrap();

                    let chol = m.clone().cholesky().unwrap();
                    let b1 = DVector::<$scalar>::new_random(n).map(|e| e.0);
                    let b2 = DMatrix::<$scalar>::new_random(n, nb).map(|e| e.0);

                    let sol1 = chol.solve(&b1);
                    let sol2 = chol.solve(&b2);

                    prop_assert!(relative_eq!(&m * &sol1, b1, epsilon = 1.0e-7));
                    prop_assert!(relative_eq!(&m * &sol2, b2, epsilon = 1.0e-7));
                }

                #[test]
                fn cholesky_solve_static(_n in PROPTEST_MATRIX_DIM) {
                    let m = RandomSDP::new(Const::<4>, || random::<$scalar>().0).unwrap();
                    let chol = m.clone().cholesky().unwrap();
                    let b1 = Vector4::<$scalar>::new_random().map(|e| e.0);
                    let b2 = Matrix4x3::<$scalar>::new_random().map(|e| e.0);

                    let sol1 = chol.solve(&b1);
                    let sol2 = chol.solve(&b2);

                    prop_assert!(relative_eq!(m * sol1, b1, epsilon = 1.0e-7));
                    prop_assert!(relative_eq!(m * sol2, b2, epsilon = 1.0e-7));
                }

                #[test]
                fn cholesky_inverse(n in PROPTEST_MATRIX_DIM) {
                    let m = RandomSDP::new(Dyn(n), || random::<$scalar>().0).unwrap();
                    let m1 = m.clone().cholesky().unwrap().inverse();
                    let id1 = &m  * &m1;
                    let id2 = &m1 * &m;

                    prop_assert!(id1.is_identity(1.0e-7) && id2.is_identity(1.0e-7));
                }

                #[test]
                fn cholesky_inverse_static(_n in PROPTEST_MATRIX_DIM) {
                    let m = RandomSDP::new(Const::<4>, || random::<$scalar>().0).unwrap();
                    let m1 = m.clone().cholesky().unwrap().inverse();
                    let id1 = &m  * &m1;
                    let id2 = &m1 * &m;

                    prop_assert!(id1.is_identity(1.0e-7) && id2.is_identity(1.0e-7));
                }

                #[test]
                fn cholesky_determinant(n in PROPTEST_MATRIX_DIM) {
                    let m = RandomSDP::new(Dyn(n), || random::<$scalar>().0).unwrap();
                    let lu_det = m.clone().lu().determinant();
                    assert_relative_eq!(lu_det.imaginary(), 0., epsilon = 1.0e-7);
                    let chol_det = m.cholesky().unwrap().determinant();

                    prop_assert!(relative_eq!(lu_det.real(), chol_det, epsilon = 1.0e-7));
                }

                #[test]
                fn cholesky_determinant_static(_n in PROPTEST_MATRIX_DIM) {
                    let m = RandomSDP::new(Const::<4>, || random::<$scalar>().0).unwrap();
                    let lu_det = m.clone().lu().determinant();
                    assert_relative_eq!(lu_det.imaginary(), 0., epsilon = 1.0e-7);
                    let chol_det = m.cholesky().unwrap().determinant();

                    prop_assert!(relative_eq!(lu_det.real(), chol_det, epsilon = 1.0e-7));
                }

                #[test]
                fn cholesky_rank_one_update(_n in PROPTEST_MATRIX_DIM) {
                    let mut m = RandomSDP::new(Const::<4>, || random::<$scalar>().0).unwrap();
                    let x = Vector4::<$scalar>::new_random().map(|e| e.0);

                    // this is dirty but $scalar is not a scalar type (its a Rand) in this file
                    let zero = random::<$scalar>().0 * 0.;
                    let one = zero + 1.;
                    let sigma = random::<f64>(); // needs to be a real
                    let sigma_scalar = zero + sigma;

                    // updates cholesky decomposition and reconstructs m updated
                    let mut chol = m.clone().cholesky().unwrap();
                    chol.rank_one_update(&x, sigma);
                    let m_chol_updated = chol.l() * chol.l().adjoint();

                    // updates m manually
                    m.gerc(sigma_scalar, &x, &x, one); // m += sigma * x * x.adjoint()

                    prop_assert!(relative_eq!(m, m_chol_updated, epsilon = 1.0e-7));
                }

                #[test]
                fn cholesky_insert_column(n in PROPTEST_MATRIX_DIM) {
                    let n = n.max(1).min(10);

                    let j = random_usize() % n;
                    let m_updated = RandomSDP::new(Dyn(n), || random::<$scalar>().0).unwrap();

                    // build m and col from m_updated
                    let col = m_updated.column(j);
                    let m = m_updated.clone().remove_column(j).remove_row(j);

                    // remove column from cholesky decomposition and rebuild m
                    let chol = m.clone().cholesky().unwrap().insert_column(j, col);
                    let m_chol_updated = chol.l() * chol.l().adjoint();

                    prop_assert!(relative_eq!(m_updated, m_chol_updated, epsilon = 1.0e-7));
                }

                #[test]
                fn cholesky_remove_column(n in PROPTEST_MATRIX_DIM) {
                    let n = n.max(1).min(10);
                    let j = random_usize() % n;
                    let m = RandomSDP::new(Dyn(n), || random::<$scalar>().0).unwrap();

                    // remove column from cholesky decomposition and rebuild m
                    let chol = m.clone().cholesky().unwrap().remove_column(j);
                    let m_chol_updated = chol.l() * chol.l().adjoint();

                    // remove column from m
                    let m_updated = m.remove_column(j).remove_row(j);

                    prop_assert!(relative_eq!(m_updated, m_chol_updated, epsilon = 1.0e-7));
                }
            }
        }
    }
);

gen_tests!(complex, RandComplex<f64>);
gen_tests!(f64, RandScalar<f64>);
