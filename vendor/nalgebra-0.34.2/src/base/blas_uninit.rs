/*
 * This file implements some BLAS operations in such a way that they work
 * even if the first argument (the output parameter) is an uninitialized matrix.
 *
 * Because doing this makes the code harder to read, we only implemented the operations that we
 * know would benefit from this performance-wise, namely, GEMM (which we use for our matrix
 * multiplication code). If we identify other operations like that in the future, we could add
 * them here.
 */

#[cfg(feature = "std")]
use matrixmultiply;
use num::{One, Zero};
use simba::scalar::{ClosedAddAssign, ClosedMulAssign};
#[cfg(feature = "std")]
use std::{any::TypeId, mem};

use crate::base::constraint::{
    AreMultipliable, DimEq, SameNumberOfColumns, SameNumberOfRows, ShapeConstraint,
};
#[cfg(feature = "std")]
use crate::base::dimension::Dyn;
use crate::base::dimension::{Dim, U1};
use crate::base::storage::{RawStorage, RawStorageMut};
use crate::base::uninit::InitStatus;
use crate::base::{Matrix, Scalar, Vector};

// # Safety
// The content of `y` must only contain values for which
// `Status::assume_init_mut` is sound.
#[allow(clippy::too_many_arguments)]
unsafe fn array_axcpy<Status, T>(
    _: Status,
    y: &mut [Status::Value],
    a: T,
    x: &[T],
    c: T,
    beta: T,
    stride1: usize,
    stride2: usize,
    len: usize,
) where
    Status: InitStatus<T>,
    T: Scalar + Zero + ClosedAddAssign + ClosedMulAssign,
{
    unsafe {
        for i in 0..len {
            let y = Status::assume_init_mut(y.get_unchecked_mut(i * stride1));
            *y = a.clone() * x.get_unchecked(i * stride2).clone() * c.clone()
                + beta.clone() * y.clone();
        }
    }
}

fn array_axc<Status, T>(
    _: Status,
    y: &mut [Status::Value],
    a: T,
    x: &[T],
    c: T,
    stride1: usize,
    stride2: usize,
    len: usize,
) where
    Status: InitStatus<T>,
    T: Scalar + Zero + ClosedAddAssign + ClosedMulAssign,
{
    for i in 0..len {
        unsafe {
            Status::init(
                y.get_unchecked_mut(i * stride1),
                a.clone() * x.get_unchecked(i * stride2).clone() * c.clone(),
            );
        }
    }
}

/// Computes `y = a * x * c + b * y`.
///
/// If `b` is zero, `y` is never read from and may be uninitialized.
///
/// # Safety
/// This is UB if b != 0 and any component of `y` is uninitialized.
#[inline(always)]
#[allow(clippy::many_single_char_names)]
pub unsafe fn axcpy_uninit<Status, T, D1: Dim, D2: Dim, SA, SB>(
    status: Status,
    y: &mut Vector<Status::Value, D1, SA>,
    a: T,
    x: &Vector<T, D2, SB>,
    c: T,
    b: T,
) where
    T: Scalar + Zero + ClosedAddAssign + ClosedMulAssign,
    SA: RawStorageMut<Status::Value, D1>,
    SB: RawStorage<T, D2>,
    ShapeConstraint: DimEq<D1, D2>,
    Status: InitStatus<T>,
{
    unsafe {
        assert_eq!(y.nrows(), x.nrows(), "Axcpy: mismatched vector shapes.");

        let rstride1 = y.strides().0;
        let rstride2 = x.strides().0;

        // SAFETY: the conversion to slices is OK because we access the
        //         elements taking the strides into account.
        let y = y.data.as_mut_slice_unchecked();
        let x = x.data.as_slice_unchecked();

        if !b.is_zero() {
            array_axcpy(status, y, a, x, c, b, rstride1, rstride2, x.len());
        } else {
            array_axc(status, y, a, x, c, rstride1, rstride2, x.len());
        }
    }
}

/// Computes `y = alpha * a * x + beta * y`, where `a` is a matrix, `x` a vector, and
/// `alpha, beta` two scalars.
///
/// If `beta` is zero, `y` is never read from and may be uninitialized.
///
/// # Safety
/// This is UB if beta != 0 and any component of `y` is uninitialized.
#[inline(always)]
pub unsafe fn gemv_uninit<Status, T, D1: Dim, R2: Dim, C2: Dim, D3: Dim, SA, SB, SC>(
    status: Status,
    y: &mut Vector<Status::Value, D1, SA>,
    alpha: T,
    a: &Matrix<T, R2, C2, SB>,
    x: &Vector<T, D3, SC>,
    beta: T,
) where
    Status: InitStatus<T>,
    T: Scalar + Zero + One + ClosedAddAssign + ClosedMulAssign,
    SA: RawStorageMut<Status::Value, D1>,
    SB: RawStorage<T, R2, C2>,
    SC: RawStorage<T, D3>,
    ShapeConstraint: DimEq<D1, R2> + AreMultipliable<R2, C2, D3, U1>,
{
    unsafe {
        let dim1 = y.nrows();
        let (nrows2, ncols2) = a.shape();
        let dim3 = x.nrows();

        assert!(
            ncols2 == dim3 && dim1 == nrows2,
            "Gemv: dimensions mismatch."
        );

        if ncols2 == 0 {
            if beta.is_zero() {
                y.apply(|e| Status::init(e, T::zero()));
            } else {
                // SAFETY: this is UB if y is uninitialized.
                y.apply(|e| *Status::assume_init_mut(e) *= beta.clone());
            }
            return;
        }

        // TODO: avoid bound checks.
        let col2 = a.column(0);
        let val = x.vget_unchecked(0).clone();

        // SAFETY: this is the call that makes this method unsafe: it is UB if Status = Uninit and beta != 0.
        axcpy_uninit(status, y, alpha.clone(), &col2, val, beta);

        for j in 1..ncols2 {
            let col2 = a.column(j);
            let val = x.vget_unchecked(j).clone();

            // SAFETY: safe because y was initialized above.
            axcpy_uninit(status, y, alpha.clone(), &col2, val, T::one());
        }
    }
}

/// Computes `y = alpha * a * b + beta * y`, where `a, b, y` are matrices.
/// `alpha` and `beta` are scalar.
///
/// If `beta` is zero, `y` is never read from and may be uninitialized.
///
/// # Safety
/// This is UB if beta != 0 and any component of `y` is uninitialized.
#[inline(always)]
pub unsafe fn gemm_uninit<
    Status,
    T,
    R1: Dim,
    C1: Dim,
    R2: Dim,
    C2: Dim,
    R3: Dim,
    C3: Dim,
    SA,
    SB,
    SC,
>(
    status: Status,
    y: &mut Matrix<Status::Value, R1, C1, SA>,
    alpha: T,
    a: &Matrix<T, R2, C2, SB>,
    b: &Matrix<T, R3, C3, SC>,
    beta: T,
) where
    Status: InitStatus<T>,
    T: Scalar + Zero + One + ClosedAddAssign + ClosedMulAssign,
    SA: RawStorageMut<Status::Value, R1, C1>,
    SB: RawStorage<T, R2, C2>,
    SC: RawStorage<T, R3, C3>,
    ShapeConstraint:
        SameNumberOfRows<R1, R2> + SameNumberOfColumns<C1, C3> + AreMultipliable<R2, C2, R3, C3>,
{
    unsafe {
        let ncols1 = y.ncols();

        #[cfg(feature = "std")]
        {
            // We assume large matrices will be Dyn but small matrices static.
            // We could use matrixmultiply for large statically-sized matrices but the performance
            // threshold to activate it would be different from SMALL_DIM because our code optimizes
            // better for statically-sized matrices.
            if R1::is::<Dyn>()
                || C1::is::<Dyn>()
                || R2::is::<Dyn>()
                || C2::is::<Dyn>()
                || R3::is::<Dyn>()
                || C3::is::<Dyn>()
            {
                // matrixmultiply can be used only if the std feature is available.
                let nrows1 = y.nrows();
                let (nrows2, ncols2) = a.shape();
                let (nrows3, ncols3) = b.shape();

                // Threshold determined empirically.
                const SMALL_DIM: usize = 5;

                if nrows1 > SMALL_DIM
                    && ncols1 > SMALL_DIM
                    && nrows2 > SMALL_DIM
                    && ncols2 > SMALL_DIM
                {
                    assert_eq!(
                        ncols2, nrows3,
                        "gemm: dimensions mismatch for multiplication."
                    );
                    assert_eq!(
                        (nrows1, ncols1),
                        (nrows2, ncols3),
                        "gemm: dimensions mismatch for addition."
                    );

                    // NOTE: this case should never happen because we enter this
                    // codepath only when ncols2 > SMALL_DIM. Though we keep this
                    // here just in case if in the future we change the conditions to
                    // enter this codepath.
                    if ncols2 == 0 {
                        // NOTE: we can't just always multiply by beta
                        // because we documented the guaranty that `self` is
                        // never read if `beta` is zero.
                        if beta.is_zero() {
                            y.apply(|e| Status::init(e, T::zero()));
                        } else {
                            // SAFETY: this is UB if Status = Uninit
                            y.apply(|e| *Status::assume_init_mut(e) *= beta.clone());
                        }
                        return;
                    }

                    if TypeId::of::<T>() == TypeId::of::<f32>() {
                        let (rsa, csa) = a.strides();
                        let (rsb, csb) = b.strides();
                        let (rsc, csc) = y.strides();

                        matrixmultiply::sgemm(
                            nrows2,
                            ncols2,
                            ncols3,
                            mem::transmute_copy(&alpha),
                            a.data.ptr() as *const f32,
                            rsa as isize,
                            csa as isize,
                            b.data.ptr() as *const f32,
                            rsb as isize,
                            csb as isize,
                            mem::transmute_copy(&beta),
                            y.data.ptr_mut() as *mut f32,
                            rsc as isize,
                            csc as isize,
                        );
                        return;
                    } else if TypeId::of::<T>() == TypeId::of::<f64>() {
                        let (rsa, csa) = a.strides();
                        let (rsb, csb) = b.strides();
                        let (rsc, csc) = y.strides();

                        matrixmultiply::dgemm(
                            nrows2,
                            ncols2,
                            ncols3,
                            mem::transmute_copy(&alpha),
                            a.data.ptr() as *const f64,
                            rsa as isize,
                            csa as isize,
                            b.data.ptr() as *const f64,
                            rsb as isize,
                            csb as isize,
                            mem::transmute_copy(&beta),
                            y.data.ptr_mut() as *mut f64,
                            rsc as isize,
                            csc as isize,
                        );
                        return;
                    }
                }
            }
        }

        for j1 in 0..ncols1 {
            // TODO: avoid bound checks.
            // SAFETY: this is UB if Status = Uninit && beta != 0
            gemv_uninit(
                status,
                &mut y.column_mut(j1),
                alpha.clone(),
                a,
                &b.column(j1),
                beta.clone(),
            );
        }
    }
}
