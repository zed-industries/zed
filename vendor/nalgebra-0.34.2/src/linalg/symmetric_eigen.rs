#[cfg(feature = "serde-serialize-no-std")]
use serde::{Deserialize, Serialize};

use approx::AbsDiffEq;
use num::Zero;

use crate::allocator::Allocator;
use crate::base::{DefaultAllocator, Matrix2, OMatrix, OVector, SquareMatrix, Vector2};
use crate::dimension::{Dim, DimDiff, DimSub, U1};
use crate::storage::Storage;
use simba::scalar::ComplexField;

use crate::linalg::SymmetricTridiagonal;
use crate::linalg::givens::GivensRotation;

/// Eigendecomposition of a symmetric matrix.
#[cfg_attr(feature = "serde-serialize-no-std", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde-serialize-no-std",
    serde(bound(serialize = "DefaultAllocator: Allocator<D, D> +
                           Allocator<D>,
         OVector<T::RealField, D>: Serialize,
         OMatrix<T, D, D>: Serialize"))
)]
#[cfg_attr(
    feature = "serde-serialize-no-std",
    serde(bound(deserialize = "DefaultAllocator: Allocator<D, D> +
                           Allocator<D>,
         OVector<T::RealField, D>: Deserialize<'de>,
         OMatrix<T, D, D>: Deserialize<'de>"))
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Debug)]
pub struct SymmetricEigen<T: ComplexField, D: Dim>
where
    DefaultAllocator: Allocator<D, D> + Allocator<D>,
{
    /// The eigenvectors of the decomposed matrix.
    pub eigenvectors: OMatrix<T, D, D>,

    /// The unsorted eigenvalues of the decomposed matrix.
    pub eigenvalues: OVector<T::RealField, D>,
}

impl<T: ComplexField, D: Dim> Copy for SymmetricEigen<T, D>
where
    DefaultAllocator: Allocator<D, D> + Allocator<D>,
    OMatrix<T, D, D>: Copy,
    OVector<T::RealField, D>: Copy,
{
}

impl<T: ComplexField, D: Dim> SymmetricEigen<T, D>
where
    DefaultAllocator: Allocator<D, D> + Allocator<D>,
{
    /// Computes the eigendecomposition of the given symmetric matrix.
    ///
    /// Only the lower-triangular parts (including its diagonal) of `m` is read.
    pub fn new(m: OMatrix<T, D, D>) -> Self
    where
        D: DimSub<U1>,
        DefaultAllocator: Allocator<DimDiff<D, U1>> + Allocator<DimDiff<D, U1>>,
    {
        Self::try_new(m, T::RealField::default_epsilon(), 0).unwrap()
    }

    /// Computes the eigendecomposition of the given symmetric matrix with user-specified
    /// convergence parameters.
    ///
    /// Only the lower-triangular part (including its diagonal) of `m` is read.
    ///
    /// # Arguments
    ///
    /// * `eps`       − tolerance used to determine when a value converged to 0.
    /// * `max_niter` − maximum total number of iterations performed by the algorithm. If this
    ///   number of iteration is exceeded, `None` is returned. If `niter == 0`, then the algorithm
    ///   continues indefinitely until convergence.
    pub fn try_new(m: OMatrix<T, D, D>, eps: T::RealField, max_niter: usize) -> Option<Self>
    where
        D: DimSub<U1>,
        DefaultAllocator: Allocator<DimDiff<D, U1>> + Allocator<DimDiff<D, U1>>,
    {
        Self::do_decompose(m, true, eps, max_niter).map(|(vals, vecs)| SymmetricEigen {
            eigenvectors: vecs.unwrap(),
            eigenvalues: vals,
        })
    }

    fn do_decompose(
        mut matrix: OMatrix<T, D, D>,
        eigenvectors: bool,
        eps: T::RealField,
        max_niter: usize,
    ) -> Option<(OVector<T::RealField, D>, Option<OMatrix<T, D, D>>)>
    where
        D: DimSub<U1>,
        DefaultAllocator: Allocator<DimDiff<D, U1>> + Allocator<DimDiff<D, U1>>,
    {
        assert!(
            matrix.is_square(),
            "Unable to compute the eigendecomposition of a non-square matrix."
        );
        let dim = matrix.nrows();
        let m_amax = matrix.camax();

        if !m_amax.is_zero() {
            matrix.unscale_mut(m_amax.clone());
        }

        let (mut q_mat, mut diag, mut off_diag);

        if eigenvectors {
            let res = SymmetricTridiagonal::new(matrix).unpack();
            q_mat = Some(res.0);
            diag = res.1;
            off_diag = res.2;
        } else {
            let res = SymmetricTridiagonal::new(matrix).unpack_tridiagonal();
            q_mat = None;
            diag = res.0;
            off_diag = res.1;
        }

        if dim == 1 {
            diag.scale_mut(m_amax);
            return Some((diag, q_mat));
        }

        let mut niter = 0;
        let (mut start, mut end) =
            Self::delimit_subproblem(&diag, &mut off_diag, dim - 1, eps.clone());

        while end != start {
            let subdim = end - start + 1;

            #[allow(clippy::comparison_chain)]
            if subdim > 2 {
                let m = end - 1;
                let n = end;

                let mut vec = Vector2::new(
                    diag[start].clone()
                        - wilkinson_shift(
                            diag[m].clone().clone(),
                            diag[n].clone(),
                            off_diag[m].clone().clone(),
                        ),
                    off_diag[start].clone(),
                );

                for i in start..n {
                    let j = i + 1;

                    match GivensRotation::cancel_y(&vec) {
                        Some((rot, norm)) => {
                            if i > start {
                                // Not the first iteration.
                                off_diag[i - 1] = norm;
                            }

                            let mii = diag[i].clone();
                            let mjj = diag[j].clone();
                            let mij = off_diag[i].clone();

                            let cc = rot.c() * rot.c();
                            let ss = rot.s() * rot.s();
                            let cs = rot.c() * rot.s();

                            let b = cs.clone() * crate::convert(2.0) * mij.clone();

                            diag[i] =
                                (cc.clone() * mii.clone() + ss.clone() * mjj.clone()) - b.clone();
                            diag[j] = (ss.clone() * mii.clone() + cc.clone() * mjj.clone()) + b;
                            off_diag[i] = cs * (mii - mjj) + mij * (cc - ss);

                            if i != n - 1 {
                                vec.x = off_diag[i].clone();
                                vec.y = -rot.s() * off_diag[i + 1].clone();
                                off_diag[i + 1] *= rot.c();
                            }

                            if let Some(ref mut q) = q_mat {
                                let rot =
                                    GivensRotation::new_unchecked(rot.c(), T::from_real(rot.s()));
                                rot.inverse().rotate_rows(&mut q.fixed_columns_mut::<2>(i));
                            }
                        }
                        None => {
                            break;
                        }
                    }
                }

                if off_diag[m].clone().norm1()
                    <= eps.clone() * (diag[m].clone().norm1() + diag[n].clone().norm1())
                {
                    end -= 1;
                }
            } else if subdim == 2 {
                let m = Matrix2::new(
                    diag[start].clone(),
                    off_diag[start].clone().conjugate(),
                    off_diag[start].clone(),
                    diag[start + 1].clone(),
                );
                let eigvals = m.eigenvalues().unwrap();

                // Choose the basis least likely to experience cancellation
                let basis = if (eigvals.x.clone() - diag[start + 1].clone()).abs()
                    > (eigvals.x.clone() - diag[start].clone()).abs()
                {
                    Vector2::new(
                        eigvals.x.clone() - diag[start + 1].clone(),
                        off_diag[start].clone(),
                    )
                } else {
                    Vector2::new(
                        off_diag[start].clone(),
                        eigvals.x.clone() - diag[start].clone(),
                    )
                };

                diag[start] = eigvals[0].clone();
                diag[start + 1] = eigvals[1].clone();

                if let Some(ref mut q) = q_mat {
                    if let Some((rot, _)) =
                        GivensRotation::try_new(basis.x.clone(), basis.y.clone(), eps.clone())
                    {
                        let rot = GivensRotation::new_unchecked(rot.c(), T::from_real(rot.s()));
                        rot.rotate_rows(&mut q.fixed_columns_mut::<2>(start));
                    }
                }

                end -= 1;
            }

            // Re-delimit the subproblem in case some decoupling occurred.
            let sub = Self::delimit_subproblem(&diag, &mut off_diag, end, eps.clone());

            start = sub.0;
            end = sub.1;

            niter += 1;
            if niter == max_niter {
                return None;
            }
        }

        diag.scale_mut(m_amax);

        Some((diag, q_mat))
    }

    fn delimit_subproblem(
        diag: &OVector<T::RealField, D>,
        off_diag: &mut OVector<T::RealField, DimDiff<D, U1>>,
        end: usize,
        eps: T::RealField,
    ) -> (usize, usize)
    where
        D: DimSub<U1>,
        DefaultAllocator: Allocator<DimDiff<D, U1>>,
    {
        let mut n = end;

        while n > 0 {
            let m = n - 1;

            if off_diag[m].clone().norm1()
                > eps.clone() * (diag[n].clone().norm1() + diag[m].clone().norm1())
            {
                break;
            }

            n -= 1;
        }

        if n == 0 {
            return (0, 0);
        }

        let mut new_start = n - 1;
        while new_start > 0 {
            let m = new_start - 1;

            if off_diag[m].clone().is_zero()
                || off_diag[m].clone().norm1()
                    <= eps.clone() * (diag[new_start].clone().norm1() + diag[m].clone().norm1())
            {
                off_diag[m] = T::RealField::zero();
                break;
            }

            new_start -= 1;
        }

        (new_start, n)
    }

    /// Rebuild the original matrix.
    ///
    /// This is useful if some of the eigenvalues have been manually modified.
    #[must_use]
    pub fn recompose(&self) -> OMatrix<T, D, D> {
        let mut u_t = self.eigenvectors.clone();
        for i in 0..self.eigenvalues.len() {
            let val = self.eigenvalues[i].clone();
            u_t.column_mut(i).scale_mut(val);
        }
        u_t.adjoint_mut();
        &self.eigenvectors * u_t
    }
}

/// Computes the wilkinson shift, i.e., the 2x2 symmetric matrix eigenvalue to its tailing
/// component `tnn`.
///
/// The inputs are interpreted as the 2x2 matrix:
///     tmm  tmn
///     tmn  tnn
pub fn wilkinson_shift<T: ComplexField>(tmm: T, tnn: T, tmn: T) -> T {
    let sq_tmn = tmn.clone() * tmn;
    if !sq_tmn.is_zero() {
        // We have the guarantee that the denominator won't be zero.
        let d = (tmm - tnn.clone()) * crate::convert(0.5);
        tnn - sq_tmn.clone() / (d.clone() + d.clone().signum() * (d.clone() * d + sq_tmn).sqrt())
    } else {
        tnn
    }
}

/*
 *
 * Computations of eigenvalues for symmetric matrices.
 *
 */
impl<T: ComplexField, D: DimSub<U1>, S: Storage<T, D, D>> SquareMatrix<T, D, S>
where
    DefaultAllocator:
        Allocator<D, D> + Allocator<DimDiff<D, U1>> + Allocator<D> + Allocator<DimDiff<D, U1>>,
{
    /// Computes the eigenvalues of this symmetric matrix.
    ///
    /// Only the lower-triangular part of the matrix is read.
    #[must_use]
    pub fn symmetric_eigenvalues(&self) -> OVector<T::RealField, D> {
        SymmetricEigen::do_decompose(
            self.clone_owned(),
            false,
            T::RealField::default_epsilon(),
            0,
        )
        .unwrap()
        .0
    }
}

#[cfg(test)]
mod test {
    use crate::base::{Matrix2, Matrix4};

    /// Exercises bug reported in issue #1109 of nalgebra (https://github.com/dimforge/nalgebra/issues/1109)
    #[test]
    fn symmetric_eigen_regression_issue_1109() {
        let m = Matrix4::new(
            -19884.07f64,
            -10.07188,
            11.277279,
            -188560.63,
            -10.07188,
            12.518197,
            1.3770627,
            -102.97504,
            11.277279,
            1.3770627,
            14.587362,
            113.26099,
            -188560.63,
            -102.97504,
            113.26099,
            -1788112.3,
        );
        let eig = m.symmetric_eigen();
        assert!(relative_eq!(
            m.lower_triangle(),
            eig.recompose().lower_triangle(),
            epsilon = 1.0e-5
        ));
    }

    fn expected_shift(m: Matrix2<f64>) -> f64 {
        let vals = m.eigenvalues().unwrap();

        if (vals.x - m.m22).abs() < (vals.y - m.m22).abs() {
            vals.x
        } else {
            vals.y
        }
    }

    #[cfg(feature = "rand")]
    #[test]
    fn wilkinson_shift_random() {
        for _ in 0..1000 {
            let m = Matrix2::<f64>::new_random();
            let m = m * m.transpose();

            let expected = expected_shift(m);
            let computed = super::wilkinson_shift(m.m11, m.m22, m.m12);
            assert!(relative_eq!(expected, computed, epsilon = 1.0e-7));
        }
    }

    #[test]
    fn wilkinson_shift_zero() {
        let m = Matrix2::new(0.0, 0.0, 0.0, 0.0);
        assert!(relative_eq!(
            expected_shift(m),
            super::wilkinson_shift(m.m11, m.m22, m.m12)
        ));
    }

    #[test]
    fn wilkinson_shift_zero_diagonal() {
        let m = Matrix2::new(0.0, 42.0, 42.0, 0.0);
        assert!(relative_eq!(
            expected_shift(m),
            super::wilkinson_shift(m.m11, m.m22, m.m12)
        ));
    }

    #[test]
    fn wilkinson_shift_zero_off_diagonal() {
        let m = Matrix2::new(42.0, 0.0, 0.0, 64.0);
        assert!(relative_eq!(
            expected_shift(m),
            super::wilkinson_shift(m.m11, m.m22, m.m12)
        ));
    }

    #[test]
    fn wilkinson_shift_zero_trace() {
        let m = Matrix2::new(42.0, 20.0, 20.0, -42.0);
        assert!(relative_eq!(
            expected_shift(m),
            super::wilkinson_shift(m.m11, m.m22, m.m12)
        ));
    }

    #[test]
    fn wilkinson_shift_zero_diag_diff_and_zero_off_diagonal() {
        let m = Matrix2::new(42.0, 0.0, 0.0, 42.0);
        assert!(relative_eq!(
            expected_shift(m),
            super::wilkinson_shift(m.m11, m.m22, m.m12)
        ));
    }

    #[test]
    fn wilkinson_shift_zero_det() {
        let m = Matrix2::new(2.0, 4.0, 4.0, 8.0);
        assert!(relative_eq!(
            expected_shift(m),
            super::wilkinson_shift(m.m11, m.m22, m.m12)
        ));
    }
}
