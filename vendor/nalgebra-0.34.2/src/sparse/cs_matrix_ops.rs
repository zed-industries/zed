use num::{One, Zero};
use simba::scalar::{ClosedAddAssign, ClosedMulAssign};
use std::ops::{Add, Mul};

use crate::allocator::Allocator;
use crate::constraint::{AreMultipliable, DimEq, ShapeConstraint};
use crate::sparse::{CsMatrix, CsStorage, CsStorageMut, CsVector};
use crate::storage::StorageMut;
use crate::{Const, DefaultAllocator, Dim, Matrix, OVector, Scalar, Vector};

impl<T: Scalar, R: Dim, C: Dim, S: CsStorage<T, R, C>> CsMatrix<T, R, C, S> {
    fn scatter<R2: Dim, C2: Dim>(
        &self,
        j: usize,
        beta: T,
        timestamps: &mut [usize],
        timestamp: usize,
        workspace: &mut [T],
        mut nz: usize,
        res: &mut CsMatrix<T, R2, C2>,
    ) -> usize
    where
        T: ClosedAddAssign + ClosedMulAssign,
        DefaultAllocator: Allocator<C2>,
    {
        for (i, val) in self.data.column_entries(j) {
            if timestamps[i] < timestamp {
                timestamps[i] = timestamp;
                res.data.i[nz] = i;
                nz += 1;
                workspace[i] = val * beta.clone();
            } else {
                workspace[i] += val * beta.clone();
            }
        }

        nz
    }
}

/*
impl<T: Scalar, R, S> CsVector<T, R, S> {
    pub fn axpy(&mut self, alpha: T, x: CsVector<T, R, S>, beta: T) {
        // First, compute the number of non-zero entries.
        let mut nnzero = 0;

        // Allocate a size large enough.
        self.data.set_column_len(0, nnzero);

        // Fill with the axpy.
        let mut i = self.len();
        let mut j = x.len();
        let mut k = nnzero - 1;
        let mut rid1 = self.data.row_index(0, i - 1);
        let mut rid2 = x.data.row_index(0, j - 1);

        while k > 0 {
            if rid1 == rid2 {
                self.data.set_row_index(0, k, rid1);
                self[k] = alpha * x[j] + beta * self[k];
                i -= 1;
                j -= 1;
            } else if rid1 < rid2 {
                self.data.set_row_index(0, k, rid1);
                self[k] = beta * self[i];
                i -= 1;
            } else {
                self.data.set_row_index(0, k, rid2);
                self[k] = alpha * x[j];
                j -= 1;
            }

            k -= 1;
        }
    }
}
*/

impl<T: Scalar + Zero + ClosedAddAssign + ClosedMulAssign, D: Dim, S: StorageMut<T, D>>
    Vector<T, D, S>
{
    /// Perform a sparse axpy operation: `self = alpha * x + beta * self` operation.
    pub fn axpy_cs<D2: Dim, S2>(&mut self, alpha: T, x: &CsVector<T, D2, S2>, beta: T)
    where
        S2: CsStorage<T, D2>,
        ShapeConstraint: DimEq<D, D2>,
    {
        if beta.is_zero() {
            for i in 0..x.len() {
                unsafe {
                    let k = x.data.row_index_unchecked(i);
                    let y = self.vget_unchecked_mut(k);
                    *y = alpha.clone() * x.data.get_value_unchecked(i).clone();
                }
            }
        } else {
            // Needed to be sure even components not present on `x` are multiplied.
            *self *= beta.clone();

            for i in 0..x.len() {
                unsafe {
                    let k = x.data.row_index_unchecked(i);
                    let y = self.vget_unchecked_mut(k);
                    *y += alpha.clone() * x.data.get_value_unchecked(i).clone();
                }
            }
        }
    }

    /*
    pub fn gemv_sparse<R2: Dim, C2: Dim, S2>(&mut self, alpha: T, a: &CsMatrix<T, R2, C2, S2>, x: &DVector<T>, beta: T)
        where
            S2: CsStorage<T, R2, C2> {
        let col2 = a.column(0);
        let val = unsafe { *x.vget_unchecked(0) };
        self.axpy_sparse(alpha * val, &col2, beta);

        for j in 1..ncols2 {
            let col2 = a.column(j);
            let val = unsafe { *x.vget_unchecked(j) };

            self.axpy_sparse(alpha * val, &col2, T::one());
        }
    }
    */
}

impl<'b, T, R1, R2, C1, C2, S1, S2> Mul<&'b CsMatrix<T, R2, C2, S2>> for &'_ CsMatrix<T, R1, C1, S1>
where
    T: Scalar + ClosedAddAssign + ClosedMulAssign + Zero,
    R1: Dim,
    C1: Dim,
    R2: Dim,
    C2: Dim,
    S1: CsStorage<T, R1, C1>,
    S2: CsStorage<T, R2, C2>,
    ShapeConstraint: AreMultipliable<R1, C1, R2, C2>,
    DefaultAllocator: Allocator<C2> + Allocator<R1> + Allocator<R1>,
{
    type Output = CsMatrix<T, R1, C2>;

    fn mul(self, rhs: &'b CsMatrix<T, R2, C2, S2>) -> Self::Output {
        let (nrows1, ncols1) = self.data.shape();
        let (nrows2, ncols2) = rhs.data.shape();
        assert_eq!(
            ncols1.value(),
            nrows2.value(),
            "Mismatched dimensions for matrix multiplication."
        );

        let mut res = CsMatrix::new_uninitialized_generic(nrows1, ncols2, self.len() + rhs.len());
        let mut workspace = OVector::<T, R1>::zeros_generic(nrows1, Const::<1>);
        let mut nz = 0;

        for j in 0..ncols2.value() {
            res.data.p[j] = nz;
            let new_size_bound = nz + nrows1.value();
            res.data.i.resize(new_size_bound, 0);
            res.data.vals.resize(new_size_bound, T::zero());

            for (i, beta) in rhs.data.column_entries(j) {
                for (k, val) in self.data.column_entries(i) {
                    workspace[k] += val.clone() * beta.clone();
                }
            }

            for (i, val) in workspace.as_mut_slice().iter_mut().enumerate() {
                if !val.is_zero() {
                    res.data.i[nz] = i;
                    res.data.vals[nz] = val.clone();
                    *val = T::zero();
                    nz += 1;
                }
            }
        }

        // NOTE: the following has a lower complexity, but is slower in many cases, likely because
        // of branching inside of the inner loop.
        //
        // let mut res = CsMatrix::new_uninitialized_generic(nrows1, ncols2, self.len() + rhs.len());
        // let mut timestamps = OVector::zeros_generic(nrows1, Const::<)>;
        // let mut workspace = unsafe { OVector::new_uninitialized_generic(nrows1, Const::<)> };
        // let mut nz = 0;
        //
        // for j in 0..ncols2.value() {
        //     res.data.p[j] = nz;
        //     let new_size_bound = nz + nrows1.value();
        //     res.data.i.resize(new_size_bound, 0);
        //     res.data.vals.resize(new_size_bound, T::zero());
        //
        //     for (i, val) in rhs.data.column_entries(j) {
        //         nz = self.scatter(
        //             i,
        //             val,
        //             timestamps.as_mut_slice(),
        //             j + 1,
        //             workspace.as_mut_slice(),
        //             nz,
        //             &mut res,
        //         );
        //     }
        //
        //     // Keep the output sorted.
        //     let range = res.data.p[j]..nz;
        //     res.data.i[range.clone()].sort();
        //
        //     for p in range {
        //         res.data.vals[p] = workspace[res.data.i[p]]
        //     }
        // }

        res.data.i.truncate(nz);
        res.data.i.shrink_to_fit();
        res.data.vals.truncate(nz);
        res.data.vals.shrink_to_fit();
        res
    }
}

impl<'b, T, R1, R2, C1, C2, S1, S2> Add<&'b CsMatrix<T, R2, C2, S2>> for &'_ CsMatrix<T, R1, C1, S1>
where
    T: Scalar + ClosedAddAssign + ClosedMulAssign + Zero + One,
    R1: Dim,
    C1: Dim,
    R2: Dim,
    C2: Dim,
    S1: CsStorage<T, R1, C1>,
    S2: CsStorage<T, R2, C2>,
    ShapeConstraint: DimEq<R1, R2> + DimEq<C1, C2>,
    DefaultAllocator: Allocator<C2> + Allocator<R1> + Allocator<R1>,
{
    type Output = CsMatrix<T, R1, C2>;

    fn add(self, rhs: &'b CsMatrix<T, R2, C2, S2>) -> Self::Output {
        let (nrows1, ncols1) = self.data.shape();
        let (nrows2, ncols2) = rhs.data.shape();
        assert_eq!(
            (nrows1.value(), ncols1.value()),
            (nrows2.value(), ncols2.value()),
            "Mismatched dimensions for matrix sum."
        );

        let mut res = CsMatrix::new_uninitialized_generic(nrows1, ncols2, self.len() + rhs.len());
        let mut timestamps = OVector::zeros_generic(nrows1, Const::<1>);
        let mut workspace = Matrix::zeros_generic(nrows1, Const::<1>);
        let mut nz = 0;

        for j in 0..ncols2.value() {
            res.data.p[j] = nz;

            nz = self.scatter(
                j,
                T::one(),
                timestamps.as_mut_slice(),
                j + 1,
                workspace.as_mut_slice(),
                nz,
                &mut res,
            );

            nz = rhs.scatter(
                j,
                T::one(),
                timestamps.as_mut_slice(),
                j + 1,
                workspace.as_mut_slice(),
                nz,
                &mut res,
            );

            // Keep the output sorted.
            let range = res.data.p[j]..nz;
            res.data.i[range.clone()].sort_unstable();

            for p in range {
                res.data.vals[p] = workspace[res.data.i[p]].clone()
            }
        }

        res.data.i.truncate(nz);
        res.data.i.shrink_to_fit();
        res.data.vals.truncate(nz);
        res.data.vals.shrink_to_fit();
        res
    }
}

impl<T, R, C, S> Mul<T> for CsMatrix<T, R, C, S>
where
    T: Scalar + ClosedAddAssign + ClosedMulAssign + Zero,
    R: Dim,
    C: Dim,
    S: CsStorageMut<T, R, C>,
{
    type Output = Self;

    fn mul(mut self, rhs: T) -> Self::Output {
        for e in self.values_mut() {
            *e *= rhs.clone()
        }

        self
    }
}
