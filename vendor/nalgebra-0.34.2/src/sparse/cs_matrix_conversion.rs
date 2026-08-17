use num::Zero;
use simba::scalar::ClosedAddAssign;

use crate::allocator::Allocator;
use crate::sparse::cs_utils;
use crate::sparse::{CsMatrix, CsStorage};
use crate::storage::Storage;
use crate::{DefaultAllocator, Dim, Dyn, Matrix, OMatrix, Scalar};

impl<T: Scalar + Zero + ClosedAddAssign> CsMatrix<T> {
    /// Creates a column-compressed sparse matrix from a sparse matrix in triplet form.
    pub fn from_triplet(
        nrows: usize,
        ncols: usize,
        irows: &[usize],
        icols: &[usize],
        vals: &[T],
    ) -> Self {
        Self::from_triplet_generic(Dyn(nrows), Dyn(ncols), irows, icols, vals)
    }
}

impl<T: Scalar + Zero + ClosedAddAssign, R: Dim, C: Dim> CsMatrix<T, R, C>
where
    DefaultAllocator: Allocator<C> + Allocator<R>,
{
    /// Creates a column-compressed sparse matrix from a sparse matrix in triplet form.
    pub fn from_triplet_generic(
        nrows: R,
        ncols: C,
        irows: &[usize],
        icols: &[usize],
        vals: &[T],
    ) -> Self {
        assert!(vals.len() == irows.len());
        assert!(vals.len() == icols.len());

        let mut res = CsMatrix::new_uninitialized_generic(nrows, ncols, vals.len());
        let mut workspace = res.data.p.clone();

        // Column count.
        for j in icols.iter().cloned() {
            workspace[j] += 1;
        }

        let _ = cs_utils::cumsum(&mut workspace, &mut res.data.p);

        // Fill i and vals.
        for ((i, j), val) in irows
            .iter()
            .cloned()
            .zip(icols.iter().cloned())
            .zip(vals.iter().cloned())
        {
            let offset = workspace[j];
            res.data.i[offset] = i;
            res.data.vals[offset] = val;
            workspace[j] = offset + 1;
        }

        // Sort the result.
        res.sort();
        res.dedup();
        res
    }
}

impl<T: Scalar + Zero, R: Dim, C: Dim, S> From<CsMatrix<T, R, C, S>> for OMatrix<T, R, C>
where
    S: CsStorage<T, R, C>,
    DefaultAllocator: Allocator<R, C>,
{
    fn from(m: CsMatrix<T, R, C, S>) -> Self {
        let (nrows, ncols) = m.data.shape();
        let mut res = OMatrix::zeros_generic(nrows, ncols);

        for j in 0..ncols.value() {
            for (i, val) in m.data.column_entries(j) {
                res[(i, j)] = val;
            }
        }

        res
    }
}

impl<T: Scalar + Zero, R: Dim, C: Dim, S> From<Matrix<T, R, C, S>> for CsMatrix<T, R, C>
where
    S: Storage<T, R, C>,
    DefaultAllocator: Allocator<R, C> + Allocator<C>,
{
    fn from(m: Matrix<T, R, C, S>) -> Self {
        let (nrows, ncols) = m.data.shape();
        let len = m.iter().filter(|e| !e.is_zero()).count();
        let mut res = CsMatrix::new_uninitialized_generic(nrows, ncols, len);
        let mut nz = 0;

        for j in 0..ncols.value() {
            let column = m.column(j);
            res.data.p[j] = nz;

            for i in 0..nrows.value() {
                if !column[i].is_zero() {
                    res.data.i[nz] = i;
                    res.data.vals[nz] = column[i].clone();
                    nz += 1;
                }
            }
        }

        res
    }
}
