#![allow(clippy::suspicious_operation_groupings)]
#[cfg(feature = "serde-serialize-no-std")]
use serde::{Deserialize, Serialize};

use approx::AbsDiffEq;
use num_complex::Complex as NumComplex;
use num_traits::identities::Zero;
use simba::scalar::{ComplexField, RealField};
use std::cmp;

use crate::allocator::Allocator;
use crate::base::dimension::{Const, Dim, DimDiff, DimSub, Dyn, U1, U2};
use crate::base::storage::Storage;
use crate::base::{DefaultAllocator, OMatrix, OVector, SquareMatrix, Unit, Vector2, Vector3};

use crate::geometry::Reflection;
use crate::linalg::Hessenberg;
use crate::linalg::givens::GivensRotation;
use crate::linalg::householder;
use crate::{Matrix, UninitVector};
use std::mem::MaybeUninit;

/// Schur decomposition of a square matrix.
///
/// If this is a real matrix, this will be a `RealField` Schur decomposition.
#[cfg_attr(feature = "serde-serialize-no-std", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde-serialize-no-std",
    serde(bound(serialize = "DefaultAllocator: Allocator<D, D>,
         OMatrix<T, D, D>: Serialize"))
)]
#[cfg_attr(
    feature = "serde-serialize-no-std",
    serde(bound(deserialize = "DefaultAllocator: Allocator<D, D>,
         OMatrix<T, D, D>: Deserialize<'de>"))
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Debug)]
pub struct Schur<T: ComplexField, D: Dim>
where
    DefaultAllocator: Allocator<D, D>,
{
    q: OMatrix<T, D, D>,
    t: OMatrix<T, D, D>,
}

impl<T: ComplexField, D: Dim> Copy for Schur<T, D>
where
    DefaultAllocator: Allocator<D, D>,
    OMatrix<T, D, D>: Copy,
{
}

impl<T: ComplexField, D: Dim> Schur<T, D>
where
    D: DimSub<U1>, // For Hessenberg.
    DefaultAllocator:
        Allocator<D, DimDiff<D, U1>> + Allocator<DimDiff<D, U1>> + Allocator<D, D> + Allocator<D>,
{
    /// Computes the Schur decomposition of a square matrix.
    pub fn new(m: OMatrix<T, D, D>) -> Self {
        Self::try_new(m, T::RealField::default_epsilon(), 0).unwrap()
    }

    /// Attempts to compute the Schur decomposition of a square matrix.
    ///
    /// If only eigenvalues are needed, it is more efficient to call the matrix method
    /// `.eigenvalues()` instead.
    ///
    /// # Arguments
    ///
    /// * `eps`       − tolerance used to determine when a value converged to 0.
    /// * `max_niter` − maximum total number of iterations performed by the algorithm. If this
    ///   number of iteration is exceeded, `None` is returned. If `niter == 0`, then the algorithm
    ///   continues indefinitely until convergence.
    pub fn try_new(m: OMatrix<T, D, D>, eps: T::RealField, max_niter: usize) -> Option<Self> {
        let mut work = Matrix::zeros_generic(m.shape_generic().0, Const::<1>);

        Self::do_decompose(m, &mut work, eps, max_niter, true)
            .map(|(q, t)| Schur { q: q.unwrap(), t })
    }

    fn do_decompose(
        mut m: OMatrix<T, D, D>,
        work: &mut OVector<T, D>,
        eps: T::RealField,
        max_niter: usize,
        compute_q: bool,
    ) -> Option<(Option<OMatrix<T, D, D>>, OMatrix<T, D, D>)> {
        assert!(
            m.is_square(),
            "Unable to compute the eigenvectors and eigenvalues of a non-square matrix."
        );

        let dim = m.shape_generic().0;

        // Specialization would make this easier.
        if dim.value() == 0 {
            let vecs = Some(OMatrix::from_element_generic(dim, dim, T::zero()));
            let vals = OMatrix::from_element_generic(dim, dim, T::zero());
            return Some((vecs, vals));
        } else if dim.value() == 1 {
            if compute_q {
                let q = OMatrix::from_element_generic(dim, dim, T::one());
                return Some((Some(q), m));
            } else {
                return Some((None, m));
            }
        } else if dim.value() == 2 {
            return decompose_2x2(m, compute_q);
        }

        let amax_m = m.camax();
        // if amax_m == 0 (i.e. the matrix is the zero matrix),
        // then the unscale_mut call will turn the entire matrix into NaNs
        // see https://github.com/dimforge/nalgebra/issues/1291
        if !amax_m.is_zero() {
            m.unscale_mut(amax_m.clone());
        }

        let hess = Hessenberg::new_with_workspace(m, work);
        let mut q;
        let mut t;

        if compute_q {
            // TODO: could we work without unpacking? Using only the internal representation of
            // hessenberg decomposition.
            let (vecs, vals) = hess.unpack();
            q = Some(vecs);
            t = vals;
        } else {
            q = None;
            t = hess.unpack_h()
        }

        // Implicit double-shift QR method.
        let mut niter = 0;
        let (mut start, mut end) = Self::delimit_subproblem(&mut t, eps.clone(), dim.value() - 1);

        while end != start {
            let subdim = end - start + 1;

            if subdim > 2 {
                let m = end - 1;
                let n = end;

                let h11 = t[(start, start)].clone();
                let h12 = t[(start, start + 1)].clone();
                let h21 = t[(start + 1, start)].clone();
                let h22 = t[(start + 1, start + 1)].clone();
                let h32 = t[(start + 2, start + 1)].clone();

                let hnn = t[(n, n)].clone();
                let hmm = t[(m, m)].clone();
                let hnm = t[(n, m)].clone();
                let hmn = t[(m, n)].clone();

                let tra = hnn.clone() + hmm.clone();
                let det = hnn * hmm - hnm * hmn;

                let mut axis = Vector3::new(
                    h11.clone() * h11.clone() + h12 * h21.clone() - tra.clone() * h11.clone() + det,
                    h21.clone() * (h11 + h22 - tra),
                    h21 * h32,
                );

                for k in start..n - 1 {
                    let (norm, not_zero) = householder::reflection_axis_mut(&mut axis);

                    if not_zero {
                        if k > start {
                            t[(k, k - 1)] = norm;
                            t[(k + 1, k - 1)] = T::zero();
                            t[(k + 2, k - 1)] = T::zero();
                        }

                        let refl = Reflection::new(Unit::new_unchecked(axis.clone()), T::zero());

                        {
                            let krows = cmp::min(k + 4, end + 1);
                            let mut work = work.rows_mut(0, krows);
                            refl.reflect(
                                &mut t.generic_view_mut((k, k), (Const::<3>, Dyn(dim.value() - k))),
                            );
                            refl.reflect_rows(
                                &mut t.generic_view_mut((0, k), (Dyn(krows), Const::<3>)),
                                &mut work,
                            );
                        }

                        if let Some(ref mut q) = q {
                            refl.reflect_rows(
                                &mut q.generic_view_mut((0, k), (dim, Const::<3>)),
                                work,
                            );
                        }
                    }

                    axis.x = t[(k + 1, k)].clone();
                    axis.y = t[(k + 2, k)].clone();

                    if k < n - 2 {
                        axis.z = t[(k + 3, k)].clone();
                    }
                }

                let mut axis = Vector2::new(axis.x.clone(), axis.y.clone());
                let (norm, not_zero) = householder::reflection_axis_mut(&mut axis);

                if not_zero {
                    let refl = Reflection::new(Unit::new_unchecked(axis), T::zero());

                    t[(m, m - 1)] = norm;
                    t[(n, m - 1)] = T::zero();

                    {
                        let mut work = work.rows_mut(0, end + 1);
                        refl.reflect(
                            &mut t.generic_view_mut((m, m), (Const::<2>, Dyn(dim.value() - m))),
                        );
                        refl.reflect_rows(
                            &mut t.generic_view_mut((0, m), (Dyn(end + 1), Const::<2>)),
                            &mut work,
                        );
                    }

                    if let Some(ref mut q) = q {
                        refl.reflect_rows(&mut q.generic_view_mut((0, m), (dim, Const::<2>)), work);
                    }
                }
            } else {
                // Decouple the 2x2 block if it has real eigenvalues.
                if let Some(rot) = compute_2x2_basis(&t.fixed_view::<2, 2>(start, start)) {
                    let inv_rot = rot.inverse();
                    inv_rot.rotate(
                        &mut t.generic_view_mut(
                            (start, start),
                            (Const::<2>, Dyn(dim.value() - start)),
                        ),
                    );
                    rot.rotate_rows(
                        &mut t.generic_view_mut((0, start), (Dyn(end + 1), Const::<2>)),
                    );
                    t[(end, start)] = T::zero();

                    if let Some(ref mut q) = q {
                        rot.rotate_rows(&mut q.generic_view_mut((0, start), (dim, Const::<2>)));
                    }
                }

                // Check if we reached the beginning of the matrix.
                if end > 2 {
                    end -= 2;
                } else {
                    break;
                }
            }

            let sub = Self::delimit_subproblem(&mut t, eps.clone(), end);

            start = sub.0;
            end = sub.1;

            niter += 1;
            if niter == max_niter {
                return None;
            }
        }

        t.scale_mut(amax_m);

        Some((q, t))
    }

    /// Computes the eigenvalues of the decomposed matrix.
    fn do_eigenvalues(t: &OMatrix<T, D, D>, out: &mut OVector<T, D>) -> bool {
        let dim = t.nrows();
        let mut m = 0;

        while m < dim - 1 {
            let n = m + 1;

            if t[(n, m)].is_zero() {
                out[m] = t[(m, m)].clone();
                m += 1;
            } else {
                // Complex eigenvalue.
                return false;
            }
        }

        if m == dim - 1 {
            out[m] = t[(m, m)].clone();
        }

        true
    }

    /// Computes the complex eigenvalues of the decomposed matrix.
    fn do_complex_eigenvalues(t: &OMatrix<T, D, D>, out: &mut UninitVector<NumComplex<T>, D>)
    where
        T: RealField,
        DefaultAllocator: Allocator<D>,
    {
        let dim = t.nrows();
        let mut m = 0;

        while m < dim - 1 {
            let n = m + 1;

            if t[(n, m)].is_zero() {
                out[m] = MaybeUninit::new(NumComplex::new(t[(m, m)].clone(), T::zero()));
                m += 1;
            } else {
                // Solve the 2x2 eigenvalue subproblem.
                let hmm = t[(m, m)].clone();
                let hnm = t[(n, m)].clone();
                let hmn = t[(m, n)].clone();
                let hnn = t[(n, n)].clone();

                // NOTE: use the same algorithm as in compute_2x2_eigvals.
                let val = (hmm.clone() - hnn.clone()) * crate::convert(0.5);
                let discr = hnm * hmn + val.clone() * val;

                // All 2x2 blocks have negative discriminant because we already decoupled those
                // with positive eigenvalues.
                let sqrt_discr = NumComplex::new(T::zero(), (-discr).sqrt());

                let half_tra = (hnn + hmm) * crate::convert(0.5);
                out[m] = MaybeUninit::new(
                    NumComplex::new(half_tra.clone(), T::zero()) + sqrt_discr.clone(),
                );
                out[m + 1] =
                    MaybeUninit::new(NumComplex::new(half_tra, T::zero()) - sqrt_discr.clone());

                m += 2;
            }
        }

        if m == dim - 1 {
            out[m] = MaybeUninit::new(NumComplex::new(t[(m, m)].clone(), T::zero()));
        }
    }

    fn delimit_subproblem(t: &mut OMatrix<T, D, D>, eps: T::RealField, end: usize) -> (usize, usize)
    where
        D: DimSub<U1>,
        DefaultAllocator: Allocator<DimDiff<D, U1>>,
    {
        let mut n = end;
        // Equivalent to SMLNUM in LAPACK DLAHQR. Since SMLNUM depends on the machine
        // precision, we use eps^2 here as best approximation.
        // This is justified because the relative threshold use eps * ( t[n,n] + t[m,m] )
        let absolute_threshold = eps.clone() * eps.clone();

        while n > 0 {
            let m = n - 1;
            let off_diag_norm = t[(n, m)].clone().norm1();

            if off_diag_norm <= absolute_threshold
                || off_diag_norm
                    <= eps.clone() * (t[(n, n)].clone().norm1() + t[(m, m)].clone().norm1())
            {
                t[(n, m)] = T::zero();
            } else {
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

            let off_diag = t[(new_start, m)].clone();
            let off_diag_norm = off_diag.clone().norm1();

            if off_diag.is_zero()
                || off_diag_norm <= absolute_threshold
                || off_diag_norm
                    <= eps.clone()
                        * (t[(new_start, new_start)].clone().norm1() + t[(m, m)].clone().norm1())
            {
                t[(new_start, m)] = T::zero();
                break;
            }

            new_start -= 1;
        }

        (new_start, n)
    }

    /// Retrieves the unitary matrix `Q` and the upper-quasitriangular matrix `T` such that the
    /// decomposed matrix equals `Q * T * Q.transpose()`.
    pub fn unpack(self) -> (OMatrix<T, D, D>, OMatrix<T, D, D>) {
        (self.q, self.t)
    }

    /// Computes the real eigenvalues of the decomposed matrix.
    ///
    /// Return `None` if some eigenvalues are complex.
    #[must_use]
    pub fn eigenvalues(&self) -> Option<OVector<T, D>> {
        let mut out = Matrix::zeros_generic(self.t.shape_generic().0, Const::<1>);
        if Self::do_eigenvalues(&self.t, &mut out) {
            Some(out)
        } else {
            None
        }
    }

    /// Computes the complex eigenvalues of the decomposed matrix.
    #[must_use]
    pub fn complex_eigenvalues(&self) -> OVector<NumComplex<T>, D>
    where
        T: RealField,
        DefaultAllocator: Allocator<D>,
    {
        let mut out = Matrix::uninit(self.t.shape_generic().0, Const::<1>);
        Self::do_complex_eigenvalues(&self.t, &mut out);
        // Safety: out has been fully initialized by do_complex_eigenvalues.
        unsafe { out.assume_init() }
    }
}

fn decompose_2x2<T: ComplexField, D: Dim>(
    mut m: OMatrix<T, D, D>,
    compute_q: bool,
) -> Option<(Option<OMatrix<T, D, D>>, OMatrix<T, D, D>)>
where
    DefaultAllocator: Allocator<D, D>,
{
    let dim = m.shape_generic().0;
    let mut q = None;
    match compute_2x2_basis(&m.fixed_view::<2, 2>(0, 0)) {
        Some(rot) => {
            let mut m = m.fixed_view_mut::<2, 2>(0, 0);
            let inv_rot = rot.inverse();
            inv_rot.rotate(&mut m);
            rot.rotate_rows(&mut m);
            m[(1, 0)] = T::zero();

            if compute_q {
                // XXX: we have to build the matrix manually because
                // rot.to_rotation_matrix().unwrap() causes an ICE.
                let c = T::from_real(rot.c());
                q = Some(OMatrix::from_column_slice_generic(
                    dim,
                    dim,
                    &[c.clone(), rot.s(), -rot.s().conjugate(), c],
                ));
            }
        }
        None => {
            if compute_q {
                q = Some(OMatrix::identity_generic(dim, dim));
            }
        }
    };

    Some((q, m))
}

fn compute_2x2_eigvals<T: ComplexField, S: Storage<T, U2, U2>>(
    m: &SquareMatrix<T, U2, S>,
) -> Option<(T, T)> {
    // Solve the 2x2 eigenvalue subproblem.
    let h00 = m[(0, 0)].clone();
    let h10 = m[(1, 0)].clone();
    let h01 = m[(0, 1)].clone();
    let h11 = m[(1, 1)].clone();

    // NOTE: this discriminant computation is more stable than the
    // one based on the trace and determinant: 0.25 * tra * tra - det
    // because it ensures positiveness for symmetric matrices.
    let val = (h00.clone() - h11.clone()) * crate::convert(0.5);
    let discr = h10 * h01 + val.clone() * val;

    discr.try_sqrt().map(|sqrt_discr| {
        let half_tra = (h00 + h11) * crate::convert(0.5);
        (half_tra.clone() + sqrt_discr.clone(), half_tra - sqrt_discr)
    })
}

// Computes the 2x2 transformation that upper-triangulates a 2x2 matrix with real eigenvalues.
/// Computes the singular vectors for a 2x2 matrix.
///
/// Returns `None` if the matrix has complex eigenvalues, or is upper-triangular. In both case,
/// the basis is the identity.
fn compute_2x2_basis<T: ComplexField, S: Storage<T, U2, U2>>(
    m: &SquareMatrix<T, U2, S>,
) -> Option<GivensRotation<T>> {
    let h10 = m[(1, 0)].clone();

    if h10.is_zero() {
        return None;
    }

    let (eigval1, eigval2) = compute_2x2_eigvals(m)?;
    let x1 = eigval1 - m[(1, 1)].clone();
    let x2 = eigval2 - m[(1, 1)].clone();

    // NOTE: Choose the one that yields a larger x component.
    // This is necessary for numerical stability of the normalization of the complex
    // number.
    if x1.clone().norm1() > x2.clone().norm1() {
        Some(GivensRotation::new(x1, h10).0)
    } else {
        Some(GivensRotation::new(x2, h10).0)
    }
}

impl<T: ComplexField, D: Dim, S: Storage<T, D, D>> SquareMatrix<T, D, S>
where
    D: DimSub<U1>, // For Hessenberg.
    DefaultAllocator:
        Allocator<D, DimDiff<D, U1>> + Allocator<DimDiff<D, U1>> + Allocator<D, D> + Allocator<D>,
{
    /// Computes the eigenvalues of this matrix.
    #[must_use]
    pub fn eigenvalues(&self) -> Option<OVector<T, D>> {
        assert!(
            self.is_square(),
            "Unable to compute eigenvalues of a non-square matrix."
        );

        let mut work = Matrix::zeros_generic(self.shape_generic().0, Const::<1>);

        // Special case for 2x2 matrices.
        if self.nrows() == 2 {
            // TODO: can we avoid this slicing
            // (which is needed here just to transform D to U2)?
            let me = self.fixed_view::<2, 2>(0, 0);
            return match compute_2x2_eigvals(&me) {
                Some((a, b)) => {
                    work[0] = a;
                    work[1] = b;
                    Some(work)
                }
                None => None,
            };
        }

        // TODO: add balancing?
        let schur = Schur::do_decompose(
            self.clone_owned(),
            &mut work,
            T::RealField::default_epsilon(),
            0,
            false,
        )
        .unwrap();

        if Schur::do_eigenvalues(&schur.1, &mut work) {
            Some(work)
        } else {
            None
        }
    }

    /// Computes the eigenvalues of this matrix.
    #[must_use]
    pub fn complex_eigenvalues(&self) -> OVector<NumComplex<T>, D>
    // TODO: add balancing?
    where
        T: RealField,
        DefaultAllocator: Allocator<D>,
    {
        let dim = self.shape_generic().0;
        let mut work = Matrix::zeros_generic(dim, Const::<1>);

        let schur = Schur::do_decompose(
            self.clone_owned(),
            &mut work,
            T::default_epsilon(),
            0,
            false,
        )
        .unwrap();
        let mut eig = Matrix::uninit(dim, Const::<1>);
        Schur::do_complex_eigenvalues(&schur.1, &mut eig);
        // Safety: eig has been fully initialized by do_complex_eigenvalues.
        unsafe { eig.assume_init() }
    }
}
