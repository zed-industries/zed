//! Matrix iterators.

use core::fmt::Debug;
use core::ops::Range;
use std::iter::FusedIterator;
use std::marker::PhantomData;
use std::mem;

use crate::base::dimension::{Dim, U1};
use crate::base::storage::{RawStorage, RawStorageMut};
use crate::base::{Matrix, MatrixView, MatrixViewMut, Scalar, ViewStorage, ViewStorageMut};

#[derive(Clone, Debug)]
struct RawIter<Ptr, T, R: Dim, C: Dim, RStride: Dim, CStride: Dim> {
    ptr: Ptr,
    inner_ptr: Ptr,
    inner_end: Ptr,
    size: usize,
    strides: (RStride, CStride),
    _phantoms: PhantomData<(fn() -> T, R, C)>,
}

macro_rules! iterator {
    (struct $Name:ident for $Storage:ident.$ptr: ident -> $Ptr:ty, $Ref:ty, $SRef: ty, $($derives:ident),* $(,)?) => {
        // TODO: we need to specialize for the case where the matrix storage is owned (in which
        // case the iterator is trivial because it does not have any stride).
        impl<T, R: Dim, C: Dim, RStride: Dim, CStride: Dim>
            RawIter<$Ptr, T, R, C, RStride, CStride>
        {
            /// Creates a new iterator for the given matrix storage.
            fn new<'a, S: $Storage<T, R, C, RStride = RStride, CStride = CStride>>(
                storage: $SRef,
            ) -> Self {
                let shape = storage.shape();
                let strides = storage.strides();
                let inner_offset = shape.0.value() * strides.0.value();
                let size = shape.0.value() * shape.1.value();
                let ptr = storage.$ptr();

                // If we have a size of 0, 'ptr' must be
                // dangling. However, 'inner_offset' might
                // not be zero if only one dimension is zero, so
                // we don't want to call 'offset'.
                // This pointer will never actually get used
                // if our size is '0', so it's fine to use
                // 'ptr' for both the start and end.
                let inner_end = if size == 0 {
                    ptr
                } else {
                    // Safety:
                    // If 'size' is non-zero, we know that 'ptr'
                    // is not dangling, and 'inner_offset' must lie
                    // within the allocation
                    unsafe { ptr.add(inner_offset) }
                };

                RawIter {
                    ptr,
                    inner_ptr: ptr,
                    inner_end,
                    size: shape.0.value() * shape.1.value(),
                    strides,
                    _phantoms: PhantomData,
                }
            }
        }

        impl<T, R: Dim, C: Dim, RStride: Dim, CStride: Dim> Iterator
            for RawIter<$Ptr, T, R, C, RStride, CStride>
        {
            type Item = $Ptr;

            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                unsafe {
                    if self.size == 0 {
                        None
                    } else {
                        self.size -= 1;

                        // Jump to the next outer dimension if needed.
                        if self.ptr == self.inner_end {
                            let stride = self.strides.1.value() as isize;
                            // This might go past the end of the allocation,
                            // depending on the value of 'size'. We use
                            // `wrapping_offset` to avoid UB
                            self.inner_end = self.ptr.wrapping_offset(stride);
                            // This will always be in bounds, since
                            // we're going to dereference it
                            self.ptr = self.inner_ptr.offset(stride);
                            self.inner_ptr = self.ptr;
                        }

                        // Go to the next element.
                        let old = self.ptr;

                        // Don't offset `self.ptr` for the last element,
                        // as this will be out of bounds. Iteration is done
                        // at this point (the next call to `next` will return `None`)
                        // so this is not observable.
                        if self.size != 0 {
                            let stride = self.strides.0.value();
                            self.ptr = self.ptr.add(stride);
                        }

                        Some(old)
                    }
                }
            }

            #[inline]
            fn size_hint(&self) -> (usize, Option<usize>) {
                (self.size, Some(self.size))
            }

            #[inline]
            fn count(self) -> usize {
                self.size_hint().0
            }
        }

        impl<T, R: Dim, C: Dim, RStride: Dim, CStride: Dim> DoubleEndedIterator
            for RawIter<$Ptr, T, R, C, RStride, CStride>
        {
            #[inline]
            fn next_back(&mut self) -> Option<Self::Item> {
                unsafe {
                    if self.size == 0 {
                        None
                    } else {
                        // Pre-decrement `size` such that it now counts to the
                        // element we want to return.
                        self.size -= 1;

                        // Fetch strides
                        let inner_stride = self.strides.0.value();
                        let outer_stride = self.strides.1.value();

                        // Compute number of rows
                        // Division should be exact
                        let inner_raw_size = self.inner_end.offset_from(self.inner_ptr) as usize;
                        let inner_size = inner_raw_size / inner_stride;

                        // Compute rows and cols remaining
                        let outer_remaining = self.size / inner_size;
                        let inner_remaining = self.size % inner_size;

                        // Compute pointer to last element
                        let last = self
                            .ptr
                            .add((outer_remaining * outer_stride + inner_remaining * inner_stride));

                        Some(last)
                    }
                }
            }
        }

        impl<T, R: Dim, C: Dim, RStride: Dim, CStride: Dim> ExactSizeIterator
            for RawIter<$Ptr, T, R, C, RStride, CStride>
        {
            #[inline]
            fn len(&self) -> usize {
                self.size
            }
        }

        impl<T, R: Dim, C: Dim, RStride: Dim, CStride: Dim> FusedIterator
            for RawIter<$Ptr, T, R, C, RStride, CStride>
        {
        }

        /// An iterator through a dense matrix with arbitrary strides matrix.
        #[derive($($derives),*)]
        pub struct $Name<'a, T, R: Dim, C: Dim, S: 'a + $Storage<T, R, C>> {
            inner: RawIter<$Ptr, T, R, C, S::RStride, S::CStride>,
            _marker: PhantomData<$Ref>,
        }

        impl<'a, T, R: Dim, C: Dim, S: 'a + $Storage<T, R, C>> $Name<'a, T, R, C, S> {
            /// Creates a new iterator for the given matrix storage.
            pub fn new(storage: $SRef) -> Self {
                Self {
                    inner: RawIter::<$Ptr, T, R, C, S::RStride, S::CStride>::new(storage),
                    _marker: PhantomData,
                }
            }
        }

        impl<'a, T, R: Dim, C: Dim, S: 'a + $Storage<T, R, C>> Iterator for $Name<'a, T, R, C, S> {
            type Item = $Ref;

            #[inline(always)]
            fn next(&mut self) -> Option<Self::Item> {
                // We want either `& *last` or `&mut *last` here, depending
                // on the mutability of `$Ref`.
                #[allow(clippy::transmute_ptr_to_ref)]
                self.inner.next().map(|ptr| unsafe { mem::transmute(ptr) })
            }

            #[inline(always)]
            fn size_hint(&self) -> (usize, Option<usize>) {
                self.inner.size_hint()
            }

            #[inline(always)]
            fn count(self) -> usize {
                self.inner.count()
            }
        }

        impl<'a, T, R: Dim, C: Dim, S: 'a + $Storage<T, R, C>> DoubleEndedIterator
            for $Name<'a, T, R, C, S>
        {
            #[inline(always)]
            fn next_back(&mut self) -> Option<Self::Item> {
                // We want either `& *last` or `&mut *last` here, depending
                // on the mutability of `$Ref`.
                #[allow(clippy::transmute_ptr_to_ref)]
                self.inner
                    .next_back()
                    .map(|ptr| unsafe { mem::transmute(ptr) })
            }
        }

        impl<'a, T, R: Dim, C: Dim, S: 'a + $Storage<T, R, C>> ExactSizeIterator
            for $Name<'a, T, R, C, S>
        {
            #[inline(always)]
            fn len(&self) -> usize {
                self.inner.len()
            }
        }

        impl<'a, T, R: Dim, C: Dim, S: 'a + $Storage<T, R, C>> FusedIterator
            for $Name<'a, T, R, C, S>
        {
        }
    };
}

iterator!(struct MatrixIter for RawStorage.ptr -> *const T, &'a T, &'a S, Clone, Debug);
iterator!(struct MatrixIterMut for RawStorageMut.ptr_mut -> *mut T, &'a mut T, &'a mut S, Debug);

impl<'a, T, R: Dim, C: Dim, RStride: Dim, CStride: Dim>
    MatrixIter<'a, T, R, C, ViewStorage<'a, T, R, C, RStride, CStride>>
{
    /// Creates a new iterator for the given matrix storage view.
    pub fn new_owned(storage: ViewStorage<'a, T, R, C, RStride, CStride>) -> Self {
        Self {
            inner: RawIter::<*const T, T, R, C, RStride, CStride>::new(&storage),
            _marker: PhantomData,
        }
    }
}

impl<'a, T, R: Dim, C: Dim, RStride: Dim, CStride: Dim>
    MatrixIterMut<'a, T, R, C, ViewStorageMut<'a, T, R, C, RStride, CStride>>
{
    /// Creates a new iterator for the given matrix storage view.
    pub fn new_owned_mut(mut storage: ViewStorageMut<'a, T, R, C, RStride, CStride>) -> Self {
        Self {
            inner: RawIter::<*mut T, T, R, C, RStride, CStride>::new(&mut storage),
            _marker: PhantomData,
        }
    }
}

/*
 *
 * Row iterators.
 *
 */
#[derive(Clone, Debug)]
/// An iterator through the rows of a matrix.
pub struct RowIter<'a, T, R: Dim, C: Dim, S: RawStorage<T, R, C>> {
    mat: &'a Matrix<T, R, C, S>,
    curr: usize,
}

impl<'a, T, R: Dim, C: Dim, S: 'a + RawStorage<T, R, C>> RowIter<'a, T, R, C, S> {
    pub(crate) const fn new(mat: &'a Matrix<T, R, C, S>) -> Self {
        RowIter { mat, curr: 0 }
    }
}

impl<'a, T, R: Dim, C: Dim, S: 'a + RawStorage<T, R, C>> Iterator for RowIter<'a, T, R, C, S> {
    type Item = MatrixView<'a, T, U1, C, S::RStride, S::CStride>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr < self.mat.nrows() {
            let res = self.mat.row(self.curr);
            self.curr += 1;
            Some(res)
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (
            self.mat.nrows() - self.curr,
            Some(self.mat.nrows() - self.curr),
        )
    }

    #[inline]
    fn count(self) -> usize {
        self.mat.nrows() - self.curr
    }
}

impl<'a, T: Scalar, R: Dim, C: Dim, S: 'a + RawStorage<T, R, C>> ExactSizeIterator
    for RowIter<'a, T, R, C, S>
{
    #[inline]
    fn len(&self) -> usize {
        self.mat.nrows() - self.curr
    }
}

/// An iterator through the mutable rows of a matrix.
#[derive(Debug)]
pub struct RowIterMut<'a, T, R: Dim, C: Dim, S: RawStorageMut<T, R, C>> {
    mat: *mut Matrix<T, R, C, S>,
    curr: usize,
    phantom: PhantomData<&'a mut Matrix<T, R, C, S>>,
}

impl<'a, T, R: Dim, C: Dim, S: 'a + RawStorageMut<T, R, C>> RowIterMut<'a, T, R, C, S> {
    pub(crate) const fn new(mat: &'a mut Matrix<T, R, C, S>) -> Self {
        RowIterMut {
            mat,
            curr: 0,
            phantom: PhantomData,
        }
    }

    fn nrows(&self) -> usize {
        unsafe { (*self.mat).nrows() }
    }
}

impl<'a, T, R: Dim, C: Dim, S: 'a + RawStorageMut<T, R, C>> Iterator
    for RowIterMut<'a, T, R, C, S>
{
    type Item = MatrixViewMut<'a, T, U1, C, S::RStride, S::CStride>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr < self.nrows() {
            let res = unsafe { (*self.mat).row_mut(self.curr) };
            self.curr += 1;
            Some(res)
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.nrows() - self.curr, Some(self.nrows() - self.curr))
    }

    #[inline]
    fn count(self) -> usize {
        self.nrows() - self.curr
    }
}

impl<'a, T: Scalar, R: Dim, C: Dim, S: 'a + RawStorageMut<T, R, C>> ExactSizeIterator
    for RowIterMut<'a, T, R, C, S>
{
    #[inline]
    fn len(&self) -> usize {
        self.nrows() - self.curr
    }
}

/*
 * Column iterators.
 *
 */
#[derive(Clone, Debug)]
/// An iterator through the columns of a matrix.
pub struct ColumnIter<'a, T, R: Dim, C: Dim, S: RawStorage<T, R, C>> {
    mat: &'a Matrix<T, R, C, S>,
    range: Range<usize>,
}

impl<'a, T, R: Dim, C: Dim, S: 'a + RawStorage<T, R, C>> ColumnIter<'a, T, R, C, S> {
    /// a new column iterator covering all columns of the matrix
    pub(crate) fn new(mat: &'a Matrix<T, R, C, S>) -> Self {
        ColumnIter {
            mat,
            range: 0..mat.ncols(),
        }
    }

    #[cfg(feature = "rayon")]
    pub(crate) fn split_at(self, index: usize) -> (Self, Self) {
        // SAFETY: this makes sure the generated ranges are valid.
        let split_pos = (self.range.start + index).min(self.range.end);

        let left_iter = ColumnIter {
            mat: self.mat,
            range: self.range.start..split_pos,
        };

        let right_iter = ColumnIter {
            mat: self.mat,
            range: split_pos..self.range.end,
        };

        (left_iter, right_iter)
    }
}

impl<'a, T, R: Dim, C: Dim, S: 'a + RawStorage<T, R, C>> Iterator for ColumnIter<'a, T, R, C, S> {
    type Item = MatrixView<'a, T, R, U1, S::RStride, S::CStride>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        debug_assert!(self.range.start <= self.range.end);
        if self.range.start < self.range.end {
            let res = self.mat.column(self.range.start);
            self.range.start += 1;
            Some(res)
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let hint = self.range.len();
        (hint, Some(hint))
    }

    #[inline]
    fn count(self) -> usize {
        self.range.len()
    }
}

impl<'a, T, R: Dim, C: Dim, S: 'a + RawStorage<T, R, C>> DoubleEndedIterator
    for ColumnIter<'a, T, R, C, S>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        debug_assert!(self.range.start <= self.range.end);
        if !self.range.is_empty() {
            self.range.end -= 1;
            debug_assert!(self.range.end < self.mat.ncols());
            debug_assert!(self.range.end >= self.range.start);
            Some(self.mat.column(self.range.end))
        } else {
            None
        }
    }
}

impl<'a, T: Scalar, R: Dim, C: Dim, S: 'a + RawStorage<T, R, C>> ExactSizeIterator
    for ColumnIter<'a, T, R, C, S>
{
    #[inline]
    fn len(&self) -> usize {
        self.range.end - self.range.start
    }
}

/// An iterator through the mutable columns of a matrix.
#[derive(Debug)]
pub struct ColumnIterMut<'a, T, R: Dim, C: Dim, S: RawStorageMut<T, R, C>> {
    mat: *mut Matrix<T, R, C, S>,
    range: Range<usize>,
    phantom: PhantomData<&'a mut Matrix<T, R, C, S>>,
}

impl<'a, T, R: Dim, C: Dim, S: 'a + RawStorageMut<T, R, C>> ColumnIterMut<'a, T, R, C, S> {
    pub(crate) fn new(mat: &'a mut Matrix<T, R, C, S>) -> Self {
        let range = 0..mat.ncols();
        ColumnIterMut {
            mat,
            range,
            phantom: Default::default(),
        }
    }

    #[cfg(feature = "rayon")]
    pub(crate) fn split_at(self, index: usize) -> (Self, Self) {
        // SAFETY: this makes sure the generated ranges are valid.
        let split_pos = (self.range.start + index).min(self.range.end);

        let left_iter = ColumnIterMut {
            mat: self.mat,
            range: self.range.start..split_pos,
            phantom: Default::default(),
        };

        let right_iter = ColumnIterMut {
            mat: self.mat,
            range: split_pos..self.range.end,
            phantom: Default::default(),
        };

        (left_iter, right_iter)
    }

    fn ncols(&self) -> usize {
        unsafe { (*self.mat).ncols() }
    }
}

impl<'a, T, R: Dim, C: Dim, S: 'a + RawStorageMut<T, R, C>> Iterator
    for ColumnIterMut<'a, T, R, C, S>
{
    type Item = MatrixViewMut<'a, T, R, U1, S::RStride, S::CStride>;

    #[inline]
    fn next(&'_ mut self) -> Option<Self::Item> {
        debug_assert!(self.range.start <= self.range.end);
        if self.range.start < self.range.end {
            let res = unsafe { (*self.mat).column_mut(self.range.start) };
            self.range.start += 1;
            Some(res)
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let hint = self.range.len();
        (hint, Some(hint))
    }

    #[inline]
    fn count(self) -> usize {
        self.range.len()
    }
}

impl<'a, T: Scalar, R: Dim, C: Dim, S: 'a + RawStorageMut<T, R, C>> ExactSizeIterator
    for ColumnIterMut<'a, T, R, C, S>
{
    #[inline]
    fn len(&self) -> usize {
        self.range.len()
    }
}

impl<'a, T: Scalar, R: Dim, C: Dim, S: 'a + RawStorageMut<T, R, C>> DoubleEndedIterator
    for ColumnIterMut<'a, T, R, C, S>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        debug_assert!(self.range.start <= self.range.end);
        if !self.range.is_empty() {
            self.range.end -= 1;
            debug_assert!(self.range.end < self.ncols());
            debug_assert!(self.range.end >= self.range.start);
            Some(unsafe { (*self.mat).column_mut(self.range.end) })
        } else {
            None
        }
    }
}
