use num::{One, Zero};
use std::cmp;
#[cfg(any(feature = "std", feature = "alloc"))]
use std::iter::ExactSizeIterator;
use std::ptr;

use crate::base::allocator::{Allocator, Reallocator};
use crate::base::constraint::{DimEq, SameNumberOfColumns, SameNumberOfRows, ShapeConstraint};
#[cfg(any(feature = "std", feature = "alloc"))]
use crate::base::dimension::Dyn;
use crate::base::dimension::{Const, Dim, DimAdd, DimDiff, DimMin, DimMinimum, DimSub, DimSum, U1};
use crate::base::storage::{RawStorage, RawStorageMut, ReshapableStorage};
use crate::base::{DefaultAllocator, Matrix, OMatrix, RowVector, Scalar, Vector};
use crate::{Storage, UninitMatrix};
use std::mem::MaybeUninit;

/// # Triangular matrix extraction
impl<T: Scalar + Zero, R: Dim, C: Dim, S: Storage<T, R, C>> Matrix<T, R, C, S> {
    /// Extracts the upper triangular part of this matrix (including the diagonal).
    #[inline]
    #[must_use]
    pub fn upper_triangle(&self) -> OMatrix<T, R, C>
    where
        DefaultAllocator: Allocator<R, C>,
    {
        let mut res = self.clone_owned();
        res.fill_lower_triangle(T::zero(), 1);

        res
    }

    /// Extracts the lower triangular part of this matrix (including the diagonal).
    #[inline]
    #[must_use]
    pub fn lower_triangle(&self) -> OMatrix<T, R, C>
    where
        DefaultAllocator: Allocator<R, C>,
    {
        let mut res = self.clone_owned();
        res.fill_upper_triangle(T::zero(), 1);

        res
    }
}

/// # Rows and columns extraction
impl<T: Scalar, R: Dim, C: Dim, S: Storage<T, R, C>> Matrix<T, R, C, S> {
    /// Creates a new matrix by extracting the given set of rows from `self`.
    #[cfg(any(feature = "std", feature = "alloc"))]
    #[must_use]
    pub fn select_rows<'a, I>(&self, irows: I) -> OMatrix<T, Dyn, C>
    where
        I: IntoIterator<Item = &'a usize>,
        I::IntoIter: ExactSizeIterator + Clone,
        DefaultAllocator: Allocator<Dyn, C>,
    {
        let irows = irows.into_iter();
        let ncols = self.shape_generic().1;
        let mut res = Matrix::uninit(Dyn(irows.len()), ncols);

        // First, check that all the indices from irows are valid.
        // This will allow us to use unchecked access in the inner loop.
        for i in irows.clone() {
            assert!(*i < self.nrows(), "Row index out of bounds.")
        }

        for j in 0..ncols.value() {
            // TODO: use unchecked column indexing
            let mut res = res.column_mut(j);
            let src = self.column(j);

            for (destination, source) in irows.clone().enumerate() {
                // Safety: all indices are in range.
                unsafe {
                    *res.vget_unchecked_mut(destination) =
                        MaybeUninit::new(src.vget_unchecked(*source).clone());
                }
            }
        }

        // Safety: res is now fully initialized.
        unsafe { res.assume_init() }
    }

    /// Creates a new matrix by extracting the given set of columns from `self`.
    #[cfg(any(feature = "std", feature = "alloc"))]
    #[must_use]
    pub fn select_columns<'a, I>(&self, icols: I) -> OMatrix<T, R, Dyn>
    where
        I: IntoIterator<Item = &'a usize>,
        I::IntoIter: ExactSizeIterator,
        DefaultAllocator: Allocator<R, Dyn>,
    {
        let icols = icols.into_iter();
        let nrows = self.shape_generic().0;
        let mut res = Matrix::uninit(nrows, Dyn(icols.len()));

        for (destination, source) in icols.enumerate() {
            // NOTE: this is basically a copy_frow but wrapping the values insnide of MaybeUninit.
            res.column_mut(destination)
                .zip_apply(&self.column(*source), |out, e| *out = MaybeUninit::new(e));
        }

        // Safety: res is now fully initialized.
        unsafe { res.assume_init() }
    }
}

/// # Set rows, columns, and diagonal
impl<T: Scalar, R: Dim, C: Dim, S: RawStorageMut<T, R, C>> Matrix<T, R, C, S> {
    /// Fills the diagonal of this matrix with the content of the given vector.
    #[inline]
    pub fn set_diagonal<R2: Dim, S2>(&mut self, diag: &Vector<T, R2, S2>)
    where
        R: DimMin<C>,
        S2: RawStorage<T, R2>,
        ShapeConstraint: DimEq<DimMinimum<R, C>, R2>,
    {
        let (nrows, ncols) = self.shape();
        let min_nrows_ncols = cmp::min(nrows, ncols);
        assert_eq!(diag.len(), min_nrows_ncols, "Mismatched dimensions.");

        for i in 0..min_nrows_ncols {
            unsafe { *self.get_unchecked_mut((i, i)) = diag.vget_unchecked(i).clone() }
        }
    }

    /// Fills the diagonal of this matrix with the content of the given iterator.
    ///
    /// This will fill as many diagonal elements as the iterator yields, up to the
    /// minimum of the number of rows and columns of `self`, and starting with the
    /// diagonal element at index (0, 0).
    #[inline]
    pub fn set_partial_diagonal(&mut self, diag: impl Iterator<Item = T>) {
        let (nrows, ncols) = self.shape();
        let min_nrows_ncols = cmp::min(nrows, ncols);

        for (i, val) in diag.enumerate().take(min_nrows_ncols) {
            unsafe { *self.get_unchecked_mut((i, i)) = val }
        }
    }

    /// Fills the selected row of this matrix with the content of the given vector.
    #[inline]
    pub fn set_row<C2: Dim, S2>(&mut self, i: usize, row: &RowVector<T, C2, S2>)
    where
        S2: RawStorage<T, U1, C2>,
        ShapeConstraint: SameNumberOfColumns<C, C2>,
    {
        self.row_mut(i).copy_from(row);
    }

    /// Fills the selected column of this matrix with the content of the given vector.
    #[inline]
    pub fn set_column<R2: Dim, S2>(&mut self, i: usize, column: &Vector<T, R2, S2>)
    where
        S2: RawStorage<T, R2, U1>,
        ShapeConstraint: SameNumberOfRows<R, R2>,
    {
        self.column_mut(i).copy_from(column);
    }
}

/// # In-place filling
impl<T, R: Dim, C: Dim, S: RawStorageMut<T, R, C>> Matrix<T, R, C, S> {
    /// Sets all the elements of this matrix to the value returned by the closure.
    #[inline]
    pub fn fill_with(&mut self, val: impl Fn() -> T) {
        for e in self.iter_mut() {
            *e = val()
        }
    }

    /// Sets all the elements of this matrix to `val`.
    #[inline]
    pub fn fill(&mut self, val: T)
    where
        T: Scalar,
    {
        for e in self.iter_mut() {
            *e = val.clone()
        }
    }

    /// Fills `self` with the identity matrix.
    #[inline]
    pub fn fill_with_identity(&mut self)
    where
        T: Scalar + Zero + One,
    {
        self.fill(T::zero());
        self.fill_diagonal(T::one());
    }

    /// Sets all the diagonal elements of this matrix to `val`.
    #[inline]
    pub fn fill_diagonal(&mut self, val: T)
    where
        T: Scalar,
    {
        let (nrows, ncols) = self.shape();
        let n = cmp::min(nrows, ncols);

        for i in 0..n {
            unsafe { *self.get_unchecked_mut((i, i)) = val.clone() }
        }
    }

    /// Sets all the elements of the selected row to `val`.
    #[inline]
    pub fn fill_row(&mut self, i: usize, val: T)
    where
        T: Scalar,
    {
        assert!(i < self.nrows(), "Row index out of bounds.");
        for j in 0..self.ncols() {
            unsafe { *self.get_unchecked_mut((i, j)) = val.clone() }
        }
    }

    /// Sets all the elements of the selected column to `val`.
    #[inline]
    pub fn fill_column(&mut self, j: usize, val: T)
    where
        T: Scalar,
    {
        assert!(j < self.ncols(), "Row index out of bounds.");
        for i in 0..self.nrows() {
            unsafe { *self.get_unchecked_mut((i, j)) = val.clone() }
        }
    }

    /// Sets all the elements of the lower-triangular part of this matrix to `val`.
    ///
    /// The parameter `shift` allows some subdiagonals to be left untouched:
    /// * If `shift = 0` then the diagonal is overwritten as well.
    /// * If `shift = 1` then the diagonal is left untouched.
    /// * If `shift > 1`, then the diagonal and the first `shift - 1` subdiagonals are left
    ///   untouched.
    #[inline]
    pub fn fill_lower_triangle(&mut self, val: T, shift: usize)
    where
        T: Scalar,
    {
        for j in 0..self.ncols() {
            for i in (j + shift)..self.nrows() {
                unsafe { *self.get_unchecked_mut((i, j)) = val.clone() }
            }
        }
    }

    /// Sets all the elements of the upper-triangular part of this matrix to `val`.
    ///
    /// The parameter `shift` allows some superdiagonals to be left untouched:
    /// * If `shift = 0` then the diagonal is overwritten as well.
    /// * If `shift = 1` then the diagonal is left untouched.
    /// * If `shift > 1`, then the diagonal and the first `shift - 1` superdiagonals are left
    ///   untouched.
    #[inline]
    pub fn fill_upper_triangle(&mut self, val: T, shift: usize)
    where
        T: Scalar,
    {
        for j in shift..self.ncols() {
            // TODO: is there a more efficient way to avoid the min ?
            // (necessary for rectangular matrices)
            for i in 0..cmp::min(j + 1 - shift, self.nrows()) {
                unsafe { *self.get_unchecked_mut((i, j)) = val.clone() }
            }
        }
    }
}

impl<T: Scalar, D: Dim, S: RawStorageMut<T, D, D>> Matrix<T, D, D, S> {
    /// Copies the upper-triangle of this matrix to its lower-triangular part.
    ///
    /// This makes the matrix symmetric. Panics if the matrix is not square.
    pub fn fill_lower_triangle_with_upper_triangle(&mut self) {
        assert!(self.is_square(), "The input matrix should be square.");

        let dim = self.nrows();
        for j in 0..dim {
            for i in j + 1..dim {
                unsafe {
                    *self.get_unchecked_mut((i, j)) = self.get_unchecked((j, i)).clone();
                }
            }
        }
    }

    /// Copies the upper-triangle of this matrix to its upper-triangular part.
    ///
    /// This makes the matrix symmetric. Panics if the matrix is not square.
    pub fn fill_upper_triangle_with_lower_triangle(&mut self) {
        assert!(self.is_square(), "The input matrix should be square.");

        for j in 1..self.ncols() {
            for i in 0..j {
                unsafe {
                    *self.get_unchecked_mut((i, j)) = self.get_unchecked((j, i)).clone();
                }
            }
        }
    }
}

/// # In-place swapping
impl<T: Scalar, R: Dim, C: Dim, S: RawStorageMut<T, R, C>> Matrix<T, R, C, S> {
    /// Swaps two rows in-place.
    #[inline]
    pub fn swap_rows(&mut self, irow1: usize, irow2: usize) {
        assert!(irow1 < self.nrows() && irow2 < self.nrows());

        if irow1 != irow2 {
            // TODO: optimize that.
            for i in 0..self.ncols() {
                unsafe { self.swap_unchecked((irow1, i), (irow2, i)) }
            }
        }
        // Otherwise do nothing.
    }

    /// Swaps two columns in-place.
    #[inline]
    pub fn swap_columns(&mut self, icol1: usize, icol2: usize) {
        assert!(icol1 < self.ncols() && icol2 < self.ncols());

        if icol1 != icol2 {
            // TODO: optimize that.
            for i in 0..self.nrows() {
                unsafe { self.swap_unchecked((i, icol1), (i, icol2)) }
            }
        }
        // Otherwise do nothing.
    }
}

/*
 *
 * TODO: specialize all the following for slices.
 *
 */
/// # Rows and columns removal
impl<T: Scalar, R: Dim, C: Dim, S: Storage<T, R, C>> Matrix<T, R, C, S> {
    /*
     *
     * Column removal.
     *
     */
    /// Removes the `i`-th column from this matrix.
    #[inline]
    pub fn remove_column(self, i: usize) -> OMatrix<T, R, DimDiff<C, U1>>
    where
        C: DimSub<U1>,
        DefaultAllocator: Reallocator<T, R, C, R, DimDiff<C, U1>>,
    {
        self.remove_fixed_columns::<1>(i)
    }

    /// Removes all columns in `indices`   
    #[cfg(any(feature = "std", feature = "alloc"))]
    pub fn remove_columns_at(self, indices: &[usize]) -> OMatrix<T, R, Dyn>
    where
        C: DimSub<Dyn, Output = Dyn>,
        DefaultAllocator: Reallocator<T, R, C, R, Dyn>,
    {
        let mut m = self.into_owned();
        let (nrows, ncols) = m.shape_generic();
        let mut offset: usize = 0;
        let mut target: usize = 0;
        while offset + target < ncols.value() {
            if indices.contains(&(target + offset)) {
                // Safety: the resulting pointer is within range.
                let col_ptr = unsafe { m.data.ptr_mut().add((target + offset) * nrows.value()) };
                // Drop every element in the column we are about to overwrite.
                // We use the a similar technique as in `Vec::truncate`.
                let s = ptr::slice_from_raw_parts_mut(col_ptr, nrows.value());
                // Safety: we drop the column in-place, which is OK because we will overwrite these
                //         entries later in the loop, or discard them with the `reallocate_copy`
                //         afterwards.
                unsafe { ptr::drop_in_place(s) };

                offset += 1;
            } else {
                unsafe {
                    let ptr_source = m.data.ptr().add((target + offset) * nrows.value());
                    let ptr_target = m.data.ptr_mut().add(target * nrows.value());

                    // Copy the data, overwriting what we dropped.
                    ptr::copy(ptr_source, ptr_target, nrows.value());
                    target += 1;
                }
            }
        }

        // Safety: The new size is smaller than the old size, so
        //         DefaultAllocator::reallocate_copy will initialize
        //         every element of the new matrix which can then
        //         be assumed to be initialized.
        unsafe {
            let new_data = DefaultAllocator::reallocate_copy(
                nrows,
                ncols.sub(Dyn::from_usize(offset)),
                m.data,
            );

            Matrix::from_data(new_data).assume_init()
        }
    }

    /// Removes all rows in `indices`   
    #[cfg(any(feature = "std", feature = "alloc"))]
    pub fn remove_rows_at(self, indices: &[usize]) -> OMatrix<T, Dyn, C>
    where
        R: DimSub<Dyn, Output = Dyn>,
        DefaultAllocator: Reallocator<T, R, C, Dyn, C>,
    {
        let mut m = self.into_owned();
        let (nrows, ncols) = m.shape_generic();
        let mut offset: usize = 0;
        let mut target: usize = 0;
        while offset + target < nrows.value() * ncols.value() {
            if indices.contains(&((target + offset) % nrows.value())) {
                // Safety: the resulting pointer is within range.
                unsafe {
                    let elt_ptr = m.data.ptr_mut().add(target + offset);
                    // Safety: we drop the component in-place, which is OK because we will overwrite these
                    //         entries later in the loop, or discard them with the `reallocate_copy`
                    //         afterwards.
                    ptr::drop_in_place(elt_ptr)
                };
                offset += 1;
            } else {
                unsafe {
                    let ptr_source = m.data.ptr().add(target + offset);
                    let ptr_target = m.data.ptr_mut().add(target);

                    // Copy the data, overwriting what we dropped in the previous iterations.
                    ptr::copy(ptr_source, ptr_target, 1);
                    target += 1;
                }
            }
        }

        // Safety: The new size is smaller than the old size, so
        //         DefaultAllocator::reallocate_copy will initialize
        //         every element of the new matrix which can then
        //         be assumed to be initialized.
        unsafe {
            let new_data = DefaultAllocator::reallocate_copy(
                nrows.sub(Dyn::from_usize(offset / ncols.value())),
                ncols,
                m.data,
            );

            Matrix::from_data(new_data).assume_init()
        }
    }

    /// Removes `D::dim()` consecutive columns from this matrix, starting with the `i`-th
    /// (included).
    #[inline]
    pub fn remove_fixed_columns<const D: usize>(
        self,
        i: usize,
    ) -> OMatrix<T, R, DimDiff<C, Const<D>>>
    where
        C: DimSub<Const<D>>,
        DefaultAllocator: Reallocator<T, R, C, R, DimDiff<C, Const<D>>>,
    {
        self.remove_columns_generic(i, Const::<D>)
    }

    /// Removes `n` consecutive columns from this matrix, starting with the `i`-th (included).
    #[inline]
    #[cfg(any(feature = "std", feature = "alloc"))]
    pub fn remove_columns(self, i: usize, n: usize) -> OMatrix<T, R, Dyn>
    where
        C: DimSub<Dyn, Output = Dyn>,
        DefaultAllocator: Reallocator<T, R, C, R, Dyn>,
    {
        self.remove_columns_generic(i, Dyn(n))
    }

    /// Removes `nremove.value()` columns from this matrix, starting with the `i`-th (included).
    ///
    /// This is the generic implementation of `.remove_columns(...)` and
    /// `.remove_fixed_columns(...)` which have nicer API interfaces.
    #[inline]
    pub fn remove_columns_generic<D>(self, i: usize, nremove: D) -> OMatrix<T, R, DimDiff<C, D>>
    where
        D: Dim,
        C: DimSub<D>,
        DefaultAllocator: Reallocator<T, R, C, R, DimDiff<C, D>>,
    {
        let mut m = self.into_owned();
        let (nrows, ncols) = m.shape_generic();
        assert!(
            i + nremove.value() <= ncols.value(),
            "Column index out of range."
        );

        let need_column_shifts = nremove.value() != 0 && i + nremove.value() < ncols.value();
        if need_column_shifts {
            // The first `deleted_i * nrows` are left untouched.
            let copied_value_start = i + nremove.value();

            unsafe {
                let ptr_in = m.data.ptr().add(copied_value_start * nrows.value());
                let ptr_out = m.data.ptr_mut().add(i * nrows.value());

                // Drop all the elements of the columns we are about to overwrite.
                // We use the a similar technique as in `Vec::truncate`.
                let s = ptr::slice_from_raw_parts_mut(ptr_out, nremove.value() * nrows.value());
                // Safety: we drop the column in-place, which is OK because we will overwrite these
                //         entries with `ptr::copy` afterward.
                ptr::drop_in_place(s);

                ptr::copy(
                    ptr_in,
                    ptr_out,
                    (ncols.value() - copied_value_start) * nrows.value(),
                );
            }
        } else {
            // All the columns to remove are at the end of the buffer. Drop them.
            unsafe {
                let ptr = m.data.ptr_mut().add(i * nrows.value());
                let s = ptr::slice_from_raw_parts_mut(ptr, nremove.value() * nrows.value());
                ptr::drop_in_place(s)
            };
        }

        // Safety: The new size is smaller than the old size, so
        //         DefaultAllocator::reallocate_copy will initialize
        //         every element of the new matrix which can then
        //         be assumed to be initialized.
        unsafe {
            let new_data = DefaultAllocator::reallocate_copy(nrows, ncols.sub(nremove), m.data);
            Matrix::from_data(new_data).assume_init()
        }
    }

    /*
     *
     * Row removal.
     *
     */
    /// Removes the `i`-th row from this matrix.
    #[inline]
    pub fn remove_row(self, i: usize) -> OMatrix<T, DimDiff<R, U1>, C>
    where
        R: DimSub<U1>,
        DefaultAllocator: Reallocator<T, R, C, DimDiff<R, U1>, C>,
    {
        self.remove_fixed_rows::<1>(i)
    }

    /// Removes `D::dim()` consecutive rows from this matrix, starting with the `i`-th (included).
    #[inline]
    pub fn remove_fixed_rows<const D: usize>(self, i: usize) -> OMatrix<T, DimDiff<R, Const<D>>, C>
    where
        R: DimSub<Const<D>>,
        DefaultAllocator: Reallocator<T, R, C, DimDiff<R, Const<D>>, C>,
    {
        self.remove_rows_generic(i, Const::<D>)
    }

    /// Removes `n` consecutive rows from this matrix, starting with the `i`-th (included).
    #[inline]
    #[cfg(any(feature = "std", feature = "alloc"))]
    pub fn remove_rows(self, i: usize, n: usize) -> OMatrix<T, Dyn, C>
    where
        R: DimSub<Dyn, Output = Dyn>,
        DefaultAllocator: Reallocator<T, R, C, Dyn, C>,
    {
        self.remove_rows_generic(i, Dyn(n))
    }

    /// Removes `nremove.value()` rows from this matrix, starting with the `i`-th (included).
    ///
    /// This is the generic implementation of `.remove_rows(...)` and `.remove_fixed_rows(...)`
    /// which have nicer API interfaces.
    #[inline]
    pub fn remove_rows_generic<D>(self, i: usize, nremove: D) -> OMatrix<T, DimDiff<R, D>, C>
    where
        D: Dim,
        R: DimSub<D>,
        DefaultAllocator: Reallocator<T, R, C, DimDiff<R, D>, C>,
    {
        let mut m = self.into_owned();
        let (nrows, ncols) = m.shape_generic();
        assert!(
            i + nremove.value() <= nrows.value(),
            "Row index out of range."
        );

        if nremove.value() != 0 {
            unsafe {
                compress_rows(
                    m.as_mut_slice(),
                    nrows.value(),
                    ncols.value(),
                    i,
                    nremove.value(),
                );
            }
        }

        // Safety: The new size is smaller than the old size, so
        //         DefaultAllocator::reallocate_copy will initialize
        //         every element of the new matrix which can then
        //         be assumed to be initialized.
        unsafe {
            let new_data = DefaultAllocator::reallocate_copy(nrows.sub(nremove), ncols, m.data);
            Matrix::from_data(new_data).assume_init()
        }
    }
}

/// # Rows and columns insertion
impl<T: Scalar, R: Dim, C: Dim, S: Storage<T, R, C>> Matrix<T, R, C, S> {
    /*
     *
     * Columns insertion.
     *
     */
    /// Inserts a column filled with `val` at the `i-th` position.
    #[inline]
    pub fn insert_column(self, i: usize, val: T) -> OMatrix<T, R, DimSum<C, U1>>
    where
        C: DimAdd<U1>,
        DefaultAllocator: Reallocator<T, R, C, R, DimSum<C, U1>>,
    {
        self.insert_fixed_columns::<1>(i, val)
    }

    /// Inserts `D` columns filled with `val` starting at the `i-th` position.
    #[inline]
    pub fn insert_fixed_columns<const D: usize>(
        self,
        i: usize,
        val: T,
    ) -> OMatrix<T, R, DimSum<C, Const<D>>>
    where
        C: DimAdd<Const<D>>,
        DefaultAllocator: Reallocator<T, R, C, R, DimSum<C, Const<D>>>,
    {
        let mut res = unsafe { self.insert_columns_generic_uninitialized(i, Const::<D>) };
        res.fixed_columns_mut::<D>(i)
            .fill_with(|| MaybeUninit::new(val.clone()));

        // Safety: the result is now fully initialized. The added columns have
        //         been initialized by the `fill_with` above, and the rest have
        //         been initialized by `insert_columns_generic_uninitialized`.
        unsafe { res.assume_init() }
    }

    /// Inserts `n` columns filled with `val` starting at the `i-th` position.
    #[inline]
    #[cfg(any(feature = "std", feature = "alloc"))]
    pub fn insert_columns(self, i: usize, n: usize, val: T) -> OMatrix<T, R, Dyn>
    where
        C: DimAdd<Dyn, Output = Dyn>,
        DefaultAllocator: Reallocator<T, R, C, R, Dyn>,
    {
        let mut res = unsafe { self.insert_columns_generic_uninitialized(i, Dyn(n)) };
        res.columns_mut(i, n)
            .fill_with(|| MaybeUninit::new(val.clone()));

        // Safety: the result is now fully initialized. The added columns have
        //         been initialized by the `fill_with` above, and the rest have
        //         been initialized by `insert_columns_generic_uninitialized`.
        unsafe { res.assume_init() }
    }

    /// Inserts `ninsert.value()` columns starting at the `i-th` place of this matrix.
    ///
    /// # Safety
    /// The output matrix has all its elements initialized except for the the components of the
    /// added columns.
    #[inline]
    pub unsafe fn insert_columns_generic_uninitialized<D>(
        self,
        i: usize,
        ninsert: D,
    ) -> UninitMatrix<T, R, DimSum<C, D>>
    where
        D: Dim,
        C: DimAdd<D>,
        DefaultAllocator: Reallocator<T, R, C, R, DimSum<C, D>>,
    {
        unsafe {
            let m = self.into_owned();
            let (nrows, ncols) = m.shape_generic();
            let mut res = Matrix::from_data(DefaultAllocator::reallocate_copy(
                nrows,
                ncols.add(ninsert),
                m.data,
            ));

            assert!(i <= ncols.value(), "Column insertion index out of range.");

            if ninsert.value() != 0 && i != ncols.value() {
                let ptr_in = res.data.ptr().add(i * nrows.value());
                let ptr_out = res
                    .data
                    .ptr_mut()
                    .add((i + ninsert.value()) * nrows.value());

                ptr::copy(ptr_in, ptr_out, (ncols.value() - i) * nrows.value())
            }

            res
        }
    }

    /*
     *
     * Rows insertion.
     *
     */
    /// Inserts a row filled with `val` at the `i-th` position.
    #[inline]
    pub fn insert_row(self, i: usize, val: T) -> OMatrix<T, DimSum<R, U1>, C>
    where
        R: DimAdd<U1>,
        DefaultAllocator: Reallocator<T, R, C, DimSum<R, U1>, C>,
    {
        self.insert_fixed_rows::<1>(i, val)
    }

    /// Inserts `D::dim()` rows filled with `val` starting at the `i-th` position.
    #[inline]
    pub fn insert_fixed_rows<const D: usize>(
        self,
        i: usize,
        val: T,
    ) -> OMatrix<T, DimSum<R, Const<D>>, C>
    where
        R: DimAdd<Const<D>>,
        DefaultAllocator: Reallocator<T, R, C, DimSum<R, Const<D>>, C>,
    {
        let mut res = unsafe { self.insert_rows_generic_uninitialized(i, Const::<D>) };
        res.fixed_rows_mut::<D>(i)
            .fill_with(|| MaybeUninit::new(val.clone()));

        // Safety: the result is now fully initialized. The added rows have
        //         been initialized by the `fill_with` above, and the rest have
        //         been initialized by `insert_rows_generic_uninitialized`.
        unsafe { res.assume_init() }
    }

    /// Inserts `n` rows filled with `val` starting at the `i-th` position.
    #[inline]
    #[cfg(any(feature = "std", feature = "alloc"))]
    pub fn insert_rows(self, i: usize, n: usize, val: T) -> OMatrix<T, Dyn, C>
    where
        R: DimAdd<Dyn, Output = Dyn>,
        DefaultAllocator: Reallocator<T, R, C, Dyn, C>,
    {
        let mut res = unsafe { self.insert_rows_generic_uninitialized(i, Dyn(n)) };
        res.rows_mut(i, n)
            .fill_with(|| MaybeUninit::new(val.clone()));

        // Safety: the result is now fully initialized. The added rows have
        //         been initialized by the `fill_with` above, and the rest have
        //         been initialized by `insert_rows_generic_uninitialized`.
        unsafe { res.assume_init() }
    }

    /// Inserts `ninsert.value()` rows at the `i-th` place of this matrix.
    ///
    /// # Safety
    /// The added rows values are not initialized.
    /// This is the generic implementation of `.insert_rows(...)` and
    /// `.insert_fixed_rows(...)` which have nicer API interfaces.
    #[inline]
    pub unsafe fn insert_rows_generic_uninitialized<D>(
        self,
        i: usize,
        ninsert: D,
    ) -> UninitMatrix<T, DimSum<R, D>, C>
    where
        D: Dim,
        R: DimAdd<D>,
        DefaultAllocator: Reallocator<T, R, C, DimSum<R, D>, C>,
    {
        unsafe {
            let m = self.into_owned();
            let (nrows, ncols) = m.shape_generic();
            let mut res = Matrix::from_data(DefaultAllocator::reallocate_copy(
                nrows.add(ninsert),
                ncols,
                m.data,
            ));

            assert!(i <= nrows.value(), "Row insertion index out of range.");

            if ninsert.value() != 0 {
                extend_rows(
                    res.as_mut_slice(),
                    nrows.value(),
                    ncols.value(),
                    i,
                    ninsert.value(),
                );
            }

            res
        }
    }
}

/// # Resizing and reshaping
impl<T: Scalar, R: Dim, C: Dim, S: Storage<T, R, C>> Matrix<T, R, C, S> {
    /// Resizes this matrix so that it contains `new_nrows` rows and `new_ncols` columns.
    ///
    /// The values are copied such that `self[(i, j)] == result[(i, j)]`. If the result has more
    /// rows and/or columns than `self`, then the extra rows or columns are filled with `val`.
    #[cfg(any(feature = "std", feature = "alloc"))]
    pub fn resize(self, new_nrows: usize, new_ncols: usize, val: T) -> OMatrix<T, Dyn, Dyn>
    where
        DefaultAllocator: Reallocator<T, R, C, Dyn, Dyn>,
    {
        self.resize_generic(Dyn(new_nrows), Dyn(new_ncols), val)
    }

    /// Resizes this matrix vertically, i.e., so that it contains `new_nrows` rows while keeping the same number of columns.
    ///
    /// The values are copied such that `self[(i, j)] == result[(i, j)]`. If the result has more
    /// rows than `self`, then the extra rows are filled with `val`.
    #[cfg(any(feature = "std", feature = "alloc"))]
    pub fn resize_vertically(self, new_nrows: usize, val: T) -> OMatrix<T, Dyn, C>
    where
        DefaultAllocator: Reallocator<T, R, C, Dyn, C>,
    {
        let ncols = self.shape_generic().1;
        self.resize_generic(Dyn(new_nrows), ncols, val)
    }

    /// Resizes this matrix horizontally, i.e., so that it contains `new_ncolumns` columns while keeping the same number of columns.
    ///
    /// The values are copied such that `self[(i, j)] == result[(i, j)]`. If the result has more
    /// columns than `self`, then the extra columns are filled with `val`.
    #[cfg(any(feature = "std", feature = "alloc"))]
    pub fn resize_horizontally(self, new_ncols: usize, val: T) -> OMatrix<T, R, Dyn>
    where
        DefaultAllocator: Reallocator<T, R, C, R, Dyn>,
    {
        let nrows = self.shape_generic().0;
        self.resize_generic(nrows, Dyn(new_ncols), val)
    }

    /// Resizes this matrix so that it contains `R2::value()` rows and `C2::value()` columns.
    ///
    /// The values are copied such that `self[(i, j)] == result[(i, j)]`. If the result has more
    /// rows and/or columns than `self`, then the extra rows or columns are filled with `val`.
    pub fn fixed_resize<const R2: usize, const C2: usize>(
        self,
        val: T,
    ) -> OMatrix<T, Const<R2>, Const<C2>>
    where
        DefaultAllocator: Reallocator<T, R, C, Const<R2>, Const<C2>>,
    {
        self.resize_generic(Const::<R2>, Const::<C2>, val)
    }

    /// Resizes `self` such that it has dimensions `new_nrows × new_ncols`.
    ///
    /// The values are copied such that `self[(i, j)] == result[(i, j)]`. If the result has more
    /// rows and/or columns than `self`, then the extra rows or columns are filled with `val`.
    #[inline]
    pub fn resize_generic<R2: Dim, C2: Dim>(
        self,
        new_nrows: R2,
        new_ncols: C2,
        val: T,
    ) -> OMatrix<T, R2, C2>
    where
        DefaultAllocator: Reallocator<T, R, C, R2, C2>,
    {
        let (nrows, ncols) = self.shape();
        let mut data = self.into_owned();

        if new_nrows.value() == nrows {
            if new_ncols.value() < ncols {
                unsafe {
                    let num_cols_to_delete = ncols - new_ncols.value();
                    let col_ptr = data.data.ptr_mut().add(new_ncols.value() * nrows);
                    let s = ptr::slice_from_raw_parts_mut(col_ptr, num_cols_to_delete * nrows);
                    // Safety: drop the elements of the deleted columns.
                    //         these are the elements that will be truncated
                    //         by the `reallocate_copy` afterward.
                    ptr::drop_in_place(s)
                };
            }

            let res = unsafe { DefaultAllocator::reallocate_copy(new_nrows, new_ncols, data.data) };
            let mut res = Matrix::from_data(res);

            if new_ncols.value() > ncols {
                res.columns_range_mut(ncols..)
                    .fill_with(|| MaybeUninit::new(val.clone()));
            }

            // Safety: the result is now fully initialized by `reallocate_copy` and
            //         `fill_with` (if the output has more columns than the input).
            unsafe { res.assume_init() }
        } else {
            let mut res;

            unsafe {
                if new_nrows.value() < nrows {
                    compress_rows(
                        data.as_mut_slice(),
                        nrows,
                        ncols,
                        new_nrows.value(),
                        nrows - new_nrows.value(),
                    );
                    res = Matrix::from_data(DefaultAllocator::reallocate_copy(
                        new_nrows, new_ncols, data.data,
                    ));
                } else {
                    res = Matrix::from_data(DefaultAllocator::reallocate_copy(
                        new_nrows, new_ncols, data.data,
                    ));
                    extend_rows(
                        res.as_mut_slice(),
                        nrows,
                        new_ncols.value(),
                        nrows,
                        new_nrows.value() - nrows,
                    );
                }
            }

            if new_ncols.value() > ncols {
                res.columns_range_mut(ncols..)
                    .fill_with(|| MaybeUninit::new(val.clone()));
            }

            if new_nrows.value() > nrows {
                res.view_range_mut(nrows.., ..cmp::min(ncols, new_ncols.value()))
                    .fill_with(|| MaybeUninit::new(val.clone()));
            }

            // Safety: the result is now fully initialized by `reallocate_copy` and
            //         `fill_with` (whenever applicable).
            unsafe { res.assume_init() }
        }
    }

    /// Reshapes `self` such that it has dimensions `new_nrows × new_ncols`.
    ///
    /// This will reinterpret `self` as if it is a matrix with `new_nrows` rows and `new_ncols`
    /// columns. The arrangements of the component in the output matrix are the same as what
    /// would be obtained by `Matrix::from_slice_generic(self.as_slice(), new_nrows, new_ncols)`.
    ///
    /// If `self` is a dynamically-sized matrix, then its components are neither copied nor moved.
    /// If `self` is staticyll-sized, then a copy may happen in some situations.
    /// This function will panic if the given dimensions are such that the number of elements of
    /// the input matrix are not equal to the number of elements of the output matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// # use nalgebra::{Matrix3x2, Matrix2x3, DMatrix, Const, Dyn};
    ///
    /// let m1 = Matrix2x3::new(
    ///     1.1, 1.2, 1.3,
    ///     2.1, 2.2, 2.3
    /// );
    /// let m2 = Matrix3x2::new(
    ///     1.1, 2.2,
    ///     2.1, 1.3,
    ///     1.2, 2.3
    /// );
    /// let reshaped = m1.reshape_generic(Const::<3>, Const::<2>);
    /// assert_eq!(reshaped, m2);
    ///
    /// let dm1 = DMatrix::from_row_slice(
    ///     4,
    ///     3,
    ///     &[
    ///         1.0, 0.0, 0.0,
    ///         0.0, 0.0, 1.0,
    ///         0.0, 0.0, 0.0,
    ///         0.0, 1.0, 0.0
    ///     ],
    /// );
    /// let dm2 = DMatrix::from_row_slice(
    ///     6,
    ///     2,
    ///     &[
    ///         1.0, 0.0,
    ///         0.0, 1.0,
    ///         0.0, 0.0,
    ///         0.0, 1.0,
    ///         0.0, 0.0,
    ///         0.0, 0.0,
    ///     ],
    /// );
    /// let reshaped = dm1.reshape_generic(Dyn(6), Dyn(2));
    /// assert_eq!(reshaped, dm2);
    /// ```
    pub fn reshape_generic<R2, C2>(
        self,
        new_nrows: R2,
        new_ncols: C2,
    ) -> Matrix<T, R2, C2, S::Output>
    where
        R2: Dim,
        C2: Dim,
        S: ReshapableStorage<T, R, C, R2, C2>,
    {
        let data = self.data.reshape_generic(new_nrows, new_ncols);
        Matrix::from_data(data)
    }
}

/// # In-place resizing
#[cfg(any(feature = "std", feature = "alloc"))]
impl<T: Scalar> OMatrix<T, Dyn, Dyn> {
    /// Resizes this matrix in-place.
    ///
    /// The values are copied such that `self[(i, j)] == result[(i, j)]`. If the result has more
    /// rows and/or columns than `self`, then the extra rows or columns are filled with `val`.
    ///
    /// Defined only for owned fully-dynamic matrices, i.e., `DMatrix`.
    pub fn resize_mut(&mut self, new_nrows: usize, new_ncols: usize, val: T)
    where
        DefaultAllocator: Reallocator<T, Dyn, Dyn, Dyn, Dyn>,
    {
        // TODO: avoid the clone.
        *self = self.clone().resize(new_nrows, new_ncols, val);
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<T: Scalar, C: Dim> OMatrix<T, Dyn, C>
where
    DefaultAllocator: Allocator<Dyn, C>,
{
    /// Changes the number of rows of this matrix in-place.
    ///
    /// The values are copied such that `self[(i, j)] == result[(i, j)]`. If the result has more
    /// rows than `self`, then the extra rows are filled with `val`.
    ///
    /// Defined only for owned matrices with a dynamic number of rows (for example, `DVector`).
    #[cfg(any(feature = "std", feature = "alloc"))]
    pub fn resize_vertically_mut(&mut self, new_nrows: usize, val: T)
    where
        DefaultAllocator: Reallocator<T, Dyn, C, Dyn, C>,
    {
        // TODO: avoid the clone.
        *self = self.clone().resize_vertically(new_nrows, val);
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<T: Scalar, R: Dim> OMatrix<T, R, Dyn>
where
    DefaultAllocator: Allocator<R, Dyn>,
{
    /// Changes the number of column of this matrix in-place.
    ///
    /// The values are copied such that `self[(i, j)] == result[(i, j)]`. If the result has more
    /// columns than `self`, then the extra columns are filled with `val`.
    ///
    /// Defined only for owned matrices with a dynamic number of columns (for example, `DVector`).
    #[cfg(any(feature = "std", feature = "alloc"))]
    pub fn resize_horizontally_mut(&mut self, new_ncols: usize, val: T)
    where
        DefaultAllocator: Reallocator<T, R, Dyn, R, Dyn>,
    {
        // TODO: avoid the clone.
        *self = self.clone().resize_horizontally(new_ncols, val);
    }
}

// Move the elements of `data` in such a way that the matrix with
// the rows `[i, i + nremove[` deleted is represented in a contiguous
// way in `data` after this method completes.
// Every deleted element are manually dropped by this method.
unsafe fn compress_rows<T: Scalar>(
    data: &mut [T],
    nrows: usize,
    ncols: usize,
    i: usize,
    nremove: usize,
) {
    unsafe {
        let new_nrows = nrows - nremove;

        if nremove == 0 {
            return; // Nothing to remove or drop.
        }

        if new_nrows == 0 || ncols == 0 {
            // The output matrix is empty, drop everything.
            ptr::drop_in_place(data);
            return;
        }

        // Safety: because `nremove != 0`, the pointers given to `ptr::copy`
        //         won’t alias.
        let ptr_in = data.as_ptr();
        let ptr_out = data.as_mut_ptr();

        let mut curr_i = i;

        for k in 0..ncols - 1 {
            // Safety: we drop the row elements in-place because we will overwrite these
            //         entries later with the `ptr::copy`.
            let s = ptr::slice_from_raw_parts_mut(ptr_out.add(curr_i), nremove);
            ptr::drop_in_place(s);
            ptr::copy(
                ptr_in.add(curr_i + (k + 1) * nremove),
                ptr_out.add(curr_i),
                new_nrows,
            );

            curr_i += new_nrows;
        }

        /*
         * Deal with the last column from which less values have to be copied.
         */
        // Safety: we drop the row elements in-place because we will overwrite these
        //         entries later with the `ptr::copy`.
        let s = ptr::slice_from_raw_parts_mut(ptr_out.add(curr_i), nremove);
        ptr::drop_in_place(s);
        let remaining_len = nrows - i - nremove;
        ptr::copy(
            ptr_in.add(nrows * ncols - remaining_len),
            ptr_out.add(curr_i),
            remaining_len,
        );
    }
}

// Moves entries of a matrix buffer to make place for `ninsert` empty rows starting at the `i-th` row index.
// The `data` buffer is assumed to contained at least `(nrows + ninsert) * ncols` elements.
unsafe fn extend_rows<T>(data: &mut [T], nrows: usize, ncols: usize, i: usize, ninsert: usize) {
    unsafe {
        let new_nrows = nrows + ninsert;

        if new_nrows == 0 || ncols == 0 {
            return; // Nothing to do as the output matrix is empty.
        }

        let ptr_in = data.as_ptr();
        let ptr_out = data.as_mut_ptr();

        let remaining_len = nrows - i;
        let mut curr_i = new_nrows * ncols - remaining_len;

        // Deal with the last column from which less values have to be copied.
        ptr::copy(
            ptr_in.add(nrows * ncols - remaining_len),
            ptr_out.add(curr_i),
            remaining_len,
        );

        for k in (0..ncols - 1).rev() {
            curr_i -= new_nrows;

            ptr::copy(ptr_in.add(k * nrows + i), ptr_out.add(curr_i), nrows);
        }
    }
}

/// Extend the number of columns of the `Matrix` with elements from
/// a given iterator.
#[cfg(any(feature = "std", feature = "alloc"))]
impl<T, R, S> Extend<T> for Matrix<T, R, Dyn, S>
where
    T: Scalar,
    R: Dim,
    S: Extend<T>,
{
    /// Extend the number of columns of the `Matrix` with elements
    /// from the given iterator.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::{DMatrix, Dyn, Matrix, OMatrix, Matrix3};
    ///
    /// let data = vec![0, 1, 2,      // column 1
    ///                 3, 4, 5];     // column 2
    ///
    /// let mut matrix = DMatrix::from_vec(3, 2, data);
    ///
    /// matrix.extend(vec![6, 7, 8]); // column 3
    ///
    /// assert!(matrix.eq(&Matrix3::new(0, 3, 6,
    ///                                 1, 4, 7,
    ///                                 2, 5, 8)));
    /// ```
    ///
    /// # Panics
    /// This function panics if the number of elements yielded by the
    /// given iterator is not a multiple of the number of rows of the
    /// `Matrix`.
    ///
    /// ```should_panic
    /// # use nalgebra::{DMatrix, Dyn, OMatrix};
    /// let data = vec![0, 1, 2,  // column 1
    ///                 3, 4, 5]; // column 2
    ///
    /// let mut matrix = DMatrix::from_vec(3, 2, data);
    ///
    /// // The following panics because the vec length is not a multiple of 3.
    /// matrix.extend(vec![6, 7, 8, 9]);
    /// ```
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.data.extend(iter);
    }
}

/// Extend the number of rows of the `Vector` with elements from
/// a given iterator.
#[cfg(any(feature = "std", feature = "alloc"))]
impl<T, S> Extend<T> for Matrix<T, Dyn, U1, S>
where
    T: Scalar,
    S: Extend<T>,
{
    /// Extend the number of rows of a `Vector` with elements
    /// from the given iterator.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::DVector;
    /// let mut vector = DVector::from_vec(vec![0, 1, 2]);
    /// vector.extend(vec![3, 4, 5]);
    /// assert!(vector.eq(&DVector::from_vec(vec![0, 1, 2, 3, 4, 5])));
    /// ```
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.data.extend(iter);
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<T, R, S, RV, SV> Extend<Vector<T, RV, SV>> for Matrix<T, R, Dyn, S>
where
    T: Scalar,
    R: Dim,
    S: Extend<Vector<T, RV, SV>>,
    RV: Dim,
    SV: RawStorage<T, RV>,
    ShapeConstraint: SameNumberOfRows<R, RV>,
{
    /// Extends the number of columns of a `Matrix` with `Vector`s
    /// from a given iterator.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::{DMatrix, Vector3, Matrix3x4};
    ///
    /// let data = vec![0, 1, 2,          // column 1
    ///                 3, 4, 5];         // column 2
    ///
    /// let mut matrix = DMatrix::from_vec(3, 2, data);
    ///
    /// matrix.extend(
    ///   vec![Vector3::new(6,  7,  8),   // column 3
    ///        Vector3::new(9, 10, 11)]); // column 4
    ///
    /// assert!(matrix.eq(&Matrix3x4::new(0, 3, 6,  9,
    ///                                   1, 4, 7, 10,
    ///                                   2, 5, 8, 11)));
    /// ```
    ///
    /// # Panics
    /// This function panics if the dimension of each `Vector` yielded
    /// by the given iterator is not equal to the number of rows of
    /// this `Matrix`.
    ///
    /// ```should_panic
    /// # use nalgebra::{DMatrix, Vector2, Matrix3x4};
    /// let mut matrix =
    ///   DMatrix::from_vec(3, 2,
    ///                     vec![0, 1, 2,   // column 1
    ///                          3, 4, 5]); // column 2
    ///
    /// // The following panics because this matrix can only be extended with 3-dimensional vectors.
    /// matrix.extend(
    ///   vec![Vector2::new(6,  7)]); // too few dimensions!
    /// ```
    ///
    /// ```should_panic
    /// # use nalgebra::{DMatrix, Vector4, Matrix3x4};
    /// let mut matrix =
    ///   DMatrix::from_vec(3, 2,
    ///                     vec![0, 1, 2,   // column 1
    ///                          3, 4, 5]); // column 2
    ///
    /// // The following panics because this matrix can only be extended with 3-dimensional vectors.
    /// matrix.extend(
    ///   vec![Vector4::new(6, 7, 8, 9)]); // too few dimensions!
    /// ```
    fn extend<I: IntoIterator<Item = Vector<T, RV, SV>>>(&mut self, iter: I) {
        self.data.extend(iter);
    }
}
