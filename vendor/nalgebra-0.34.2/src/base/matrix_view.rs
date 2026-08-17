use std::marker::PhantomData;
use std::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo};
use std::slice;

use crate::ReshapableStorage;
use crate::base::allocator::Allocator;
use crate::base::default_allocator::DefaultAllocator;
use crate::base::dimension::{Const, Dim, DimName, Dyn, IsNotStaticOne, U1};
use crate::base::iter::MatrixIter;
use crate::base::storage::{IsContiguous, Owned, RawStorage, RawStorageMut, Storage};
use crate::base::{Matrix, Scalar};
use crate::constraint::{DimEq, ShapeConstraint};

macro_rules! view_storage_impl (
    ($doc: expr_2021; $Storage: ident as $SRef: ty; $legacy_name:ident => $T: ident.$get_addr: ident ($Ptr: ty as $Ref: ty)) => {
        #[doc = $doc]
        #[derive(Debug)]
        pub struct $T<'a, T, R: Dim, C: Dim, RStride: Dim, CStride: Dim>  {
            ptr:       $Ptr,
            shape:     (R, C),
            strides:   (RStride, CStride),
            _phantoms: PhantomData<$Ref>,
        }

        #[doc = $doc]
        ///
        /// This type alias exists only for legacy purposes and is deprecated. It will be removed
        /// in a future release. Please use
        /// [`
        #[doc = stringify!($T)]
        /// `] instead. See [issue #1076](https://github.com/dimforge/nalgebra/issues/1076)
        /// for the rationale.
        #[deprecated = "Use ViewStorage(Mut) instead."]
        pub type $legacy_name<'a, T, R, C, RStride, CStride> = $T<'a, T, R, C, RStride, CStride>;

        unsafe impl<'a, T: Send, R: Dim, C: Dim, RStride: Dim, CStride: Dim> Send
            for $T<'a, T, R, C, RStride, CStride>
        {}

        unsafe impl<'a, T: Sync, R: Dim, C: Dim, RStride: Dim, CStride: Dim> Sync
            for $T<'a, T, R, C, RStride, CStride>
        {}

        impl<'a, T, R: Dim, C: Dim, RStride: Dim, CStride: Dim> $T<'a, T, R, C, RStride, CStride> {
            /// Create a new matrix view without bounds checking and from a raw pointer.
            ///
            /// # Safety
            ///
            /// `*ptr` must point to memory that is valid `[T; R * C]`.
            #[inline]
            pub const unsafe fn from_raw_parts(ptr:     $Ptr,
                                         shape:   (R, C),
                                         strides: (RStride, CStride))
                                        -> Self
                where RStride: Dim,
                      CStride: Dim {

                $T {
                    ptr,
                    shape,
                    strides,
                    _phantoms: PhantomData
                }
            }
        }

        // Dyn is arbitrary. It's just to be able to call the constructors with `Slice::`
        impl<'a, T, R: Dim, C: Dim> $T<'a, T, R, C, Dyn, Dyn> {
            /// Create a new matrix view without bounds checking.
            ///
            /// # Safety
            ///
            /// `storage` contains sufficient elements beyond `start + R * C` such that all
            /// accesses are within bounds.
            #[inline]
            pub unsafe fn new_unchecked<RStor, CStor, S>(storage: $SRef, start: (usize, usize), shape: (R, C))
                -> $T<'a, T, R, C, S::RStride, S::CStride>
                where RStor: Dim,
                      CStor: Dim,
                      S:     $Storage<T, RStor, CStor> { unsafe {

                let strides = storage.strides();
                $T::new_with_strides_unchecked(storage, start, shape, strides)
            }}

            /// Create a new matrix view without bounds checking.
            ///
            /// # Safety
            ///
            /// `strides` must be a valid stride indexing.
            #[inline]
            pub unsafe fn new_with_strides_unchecked<S, RStor, CStor, RStride, CStride>(storage: $SRef,
                                                                                        start:   (usize, usize),
                                                                                        shape:   (R, C),
                                                                                        strides: (RStride, CStride))
                -> $T<'a, T, R, C, RStride, CStride>
                where RStor: Dim,
                      CStor: Dim,
                      S: $Storage<T, RStor, CStor>,
                      RStride: Dim,
                      CStride: Dim { unsafe {
                $T::from_raw_parts(storage.$get_addr(start.0, start.1), shape, strides)
            }}
        }

        impl <'a, T, R: Dim, C: Dim, RStride: Dim, CStride: Dim>
            $T<'a, T, R, C, RStride, CStride>
        where
            Self: RawStorage<T, R, C> + IsContiguous
        {
            /// Extracts the original slice from this storage.
            pub fn into_slice(self) -> &'a [T] {
                let (nrows, ncols) = self.shape();
                if nrows.value() != 0 && ncols.value() != 0 {
                    let sz = self.linear_index(nrows.value() - 1, ncols.value() - 1);
                    unsafe { slice::from_raw_parts(self.ptr, sz + 1) }
                } else {
                    unsafe { slice::from_raw_parts(self.ptr, 0) }
                }
            }
        }
    }
);

view_storage_impl!("A matrix data storage for a matrix view. Only contains an internal reference \
                     to another matrix data storage.";
    RawStorage as &'a S; SliceStorage => ViewStorage.get_address_unchecked(*const T as &'a T));

view_storage_impl!("A mutable matrix data storage for mutable matrix view. Only contains an \
                     internal mutable reference to another matrix data storage.";
    RawStorageMut as &'a mut S; SliceStorageMut => ViewStorageMut.get_address_unchecked_mut(*mut T as &'a mut T)
);

impl<T: Scalar, R: Dim, C: Dim, RStride: Dim, CStride: Dim> Copy
    for ViewStorage<'_, T, R, C, RStride, CStride>
{
}

impl<T: Scalar, R: Dim, C: Dim, RStride: Dim, CStride: Dim> Clone
    for ViewStorage<'_, T, R, C, RStride, CStride>
{
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, T: Scalar, R: Dim, C: Dim, RStride: Dim, CStride: Dim>
    ViewStorageMut<'a, T, R, C, RStride, CStride>
where
    Self: RawStorageMut<T, R, C> + IsContiguous,
{
    /// Extracts the original slice from this storage
    pub fn into_slice_mut(self) -> &'a mut [T] {
        let (nrows, ncols) = self.shape();
        if nrows.value() != 0 && ncols.value() != 0 {
            let sz = self.linear_index(nrows.value() - 1, ncols.value() - 1);
            unsafe { slice::from_raw_parts_mut(self.ptr, sz + 1) }
        } else {
            unsafe { slice::from_raw_parts_mut(self.ptr, 0) }
        }
    }
}

macro_rules! storage_impl(
    ($($T: ident),* $(,)*) => {$(
        unsafe impl<'a, T, R: Dim, C: Dim, RStride: Dim, CStride: Dim> RawStorage<T, R, C>
            for $T<'a, T, R, C, RStride, CStride> {

            type RStride = RStride;
            type CStride = CStride;

            #[inline]
            fn ptr(&self) -> *const T {
                self.ptr
            }

            #[inline]
            fn shape(&self) -> (R, C) {
                self.shape
            }

            #[inline]
            fn strides(&self) -> (Self::RStride, Self::CStride) {
                self.strides
            }

            #[inline]
            fn is_contiguous(&self) -> bool {
                // Common cases that can be deduced at compile-time even if one of the dimensions
                // is Dyn.
                if (RStride::is::<U1>() && C::is::<U1>()) || // Column vector.
                   (CStride::is::<U1>() && R::is::<U1>()) {  // Row vector.
                    true
                }
                else {
                    let (nrows, _)     = self.shape();
                    let (srows, scols) = self.strides();

                    srows.value() == 1 && scols.value() == nrows.value()
                }
            }

            #[inline]
            unsafe fn as_slice_unchecked(&self) -> &[T] { unsafe {
                let (nrows, ncols) = self.shape();
                if nrows.value() != 0 && ncols.value() != 0 {
                    let sz = self.linear_index(nrows.value() - 1, ncols.value() - 1);
                    slice::from_raw_parts(self.ptr, sz + 1)
                }
                else {
                    slice::from_raw_parts(self.ptr, 0)
                }
            }}
        }

        unsafe impl<'a, T: Scalar, R: Dim, C: Dim, RStride: Dim, CStride: Dim> Storage<T, R, C>
            for $T<'a, T, R, C, RStride, CStride> {
            #[inline]
            fn into_owned(self) -> Owned<T, R, C>
                where DefaultAllocator: Allocator<R, C> {
                self.clone_owned()
            }

            #[inline]
            fn clone_owned(&self) -> Owned<T, R, C>
                where DefaultAllocator: Allocator<R, C> {
                let (nrows, ncols) = self.shape();
                let it = MatrixIter::new(self).cloned();
                DefaultAllocator::allocate_from_iterator(nrows, ncols, it)
            }

            #[inline]
            fn forget_elements(self) {
                // No cleanup required.
            }
        }
    )*}
);

storage_impl!(ViewStorage, ViewStorageMut);

unsafe impl<T, R: Dim, C: Dim, RStride: Dim, CStride: Dim> RawStorageMut<T, R, C>
    for ViewStorageMut<'_, T, R, C, RStride, CStride>
{
    #[inline]
    fn ptr_mut(&mut self) -> *mut T {
        self.ptr
    }

    #[inline]
    unsafe fn as_mut_slice_unchecked(&mut self) -> &mut [T] {
        unsafe {
            let (nrows, ncols) = self.shape();
            if nrows.value() != 0 && ncols.value() != 0 {
                let sz = self.linear_index(nrows.value() - 1, ncols.value() - 1);
                slice::from_raw_parts_mut(self.ptr, sz + 1)
            } else {
                slice::from_raw_parts_mut(self.ptr, 0)
            }
        }
    }
}

unsafe impl<T, R: Dim, CStride: Dim> IsContiguous for ViewStorage<'_, T, R, U1, U1, CStride> {}
unsafe impl<T, R: Dim, CStride: Dim> IsContiguous for ViewStorageMut<'_, T, R, U1, U1, CStride> {}

unsafe impl<T, R: DimName, C: Dim + IsNotStaticOne> IsContiguous
    for ViewStorage<'_, T, R, C, U1, R>
{
}
unsafe impl<T, R: DimName, C: Dim + IsNotStaticOne> IsContiguous
    for ViewStorageMut<'_, T, R, C, U1, R>
{
}

impl<T, R: Dim, C: Dim, S: RawStorage<T, R, C>> Matrix<T, R, C, S> {
    #[inline]
    fn assert_view_index(
        &self,
        start: (usize, usize),
        shape: (usize, usize),
        steps: (usize, usize),
    ) {
        let my_shape = self.shape();
        // NOTE: we don't do any subtraction to avoid underflow for zero-sized matrices.
        //
        // Terms that would have been negative are moved to the other side of the inequality
        // instead.
        assert!(
            start.0 + (steps.0 + 1) * shape.0 <= my_shape.0 + steps.0,
            "Matrix slicing out of bounds."
        );
        assert!(
            start.1 + (steps.1 + 1) * shape.1 <= my_shape.1 + steps.1,
            "Matrix slicing out of bounds."
        );
    }
}

macro_rules! matrix_view_impl (
    ($me: ident: $Me: ty, $MatrixView: ident, $ViewStorage: ident, $Storage: ident.$get_addr: ident (), $data: expr_2021;
     $row: ident,
     $row_part: ident,
     $rows: ident,
     $rows_with_step: ident,
     $fixed_rows: ident,
     $fixed_rows_with_step: ident,
     $rows_generic: ident,
     $rows_generic_with_step: ident,
     $column: ident,
     $column_part: ident,
     $columns: ident,
     $columns_with_step: ident,
     $fixed_columns: ident,
     $fixed_columns_with_step: ident,
     $columns_generic: ident,
     $columns_generic_with_step: ident,
     $slice: ident => $view:ident,
     $slice_with_steps: ident => $view_with_steps:ident,
     $fixed_slice: ident => $fixed_view:ident,
     $fixed_slice_with_steps: ident => $fixed_view_with_steps:ident,
     $generic_slice: ident => $generic_view:ident,
     $generic_slice_with_steps: ident => $generic_view_with_steps:ident,
     $rows_range_pair: ident,
     $columns_range_pair: ident) => {
        /*
         *
         * Row slicing.
         *
         */
        /// Returns a view containing the i-th row of this matrix.
        #[inline]
        pub fn $row($me: $Me, i: usize) -> $MatrixView<'_, T, U1, C, S::RStride, S::CStride> {
            $me.$fixed_rows::<1>(i)
        }

        /// Returns a view containing the `n` first elements of the i-th row of this matrix.
        #[inline]
        pub fn $row_part($me: $Me, i: usize, n: usize) -> $MatrixView<'_, T, U1, Dyn, S::RStride, S::CStride> {
            $me.$generic_view((i, 0), (Const::<1>, Dyn(n)))
        }

        /// Extracts from this matrix a set of consecutive rows.
        #[inline]
        pub fn $rows($me: $Me, first_row: usize, nrows: usize)
            -> $MatrixView<'_, T, Dyn, C, S::RStride, S::CStride> {

            $me.$rows_generic(first_row, Dyn(nrows))
        }

        /// Extracts from this matrix a set of consecutive rows regularly skipping `step` rows.
        #[inline]
        pub fn $rows_with_step($me: $Me, first_row: usize, nrows: usize, step: usize)
            -> $MatrixView<'_, T, Dyn, C, Dyn, S::CStride> {

            $me.$rows_generic_with_step(first_row, Dyn(nrows), step)
        }

        /// Extracts a compile-time number of consecutive rows from this matrix.
        #[inline]
        pub fn $fixed_rows<const RVIEW: usize>($me: $Me, first_row: usize)
            -> $MatrixView<'_, T, Const<RVIEW>, C, S::RStride, S::CStride> {

            $me.$rows_generic(first_row, Const::<RVIEW>)
        }

        /// Extracts from this matrix a compile-time number of rows regularly skipping `step`
        /// rows.
        #[inline]
        pub fn $fixed_rows_with_step<const RVIEW: usize>($me: $Me, first_row: usize, step: usize)
            -> $MatrixView<'_, T, Const<RVIEW>, C, Dyn, S::CStride> {

            $me.$rows_generic_with_step(first_row, Const::<RVIEW>, step)
        }

        /// Extracts from this matrix `nrows` rows regularly skipping `step` rows. Both
        /// argument may or may not be values known at compile-time.
        #[inline]
        pub fn $rows_generic<RView: Dim>($me: $Me, row_start: usize, nrows: RView)
            -> $MatrixView<'_, T, RView, C, S::RStride, S::CStride> {

            let my_shape   = $me.shape_generic();
            $me.assert_view_index((row_start, 0), (nrows.value(), my_shape.1.value()), (0, 0));

            let shape = (nrows, my_shape.1);

            unsafe {
                let data = $ViewStorage::new_unchecked($data, (row_start, 0), shape);
                Matrix::from_data_statically_unchecked(data)
            }
        }

        /// Extracts from this matrix `nrows` rows regularly skipping `step` rows. Both
        /// argument may or may not be values known at compile-time.
        #[inline]
        pub fn $rows_generic_with_step<RView>($me: $Me, row_start: usize, nrows: RView, step: usize)
            -> $MatrixView<'_, T, RView, C, Dyn, S::CStride>
            where RView: Dim {

            let my_shape   = $me.shape_generic();
            let my_strides = $me.data.strides();
            $me.assert_view_index((row_start, 0), (nrows.value(), my_shape.1.value()), (step, 0));

            let strides = (Dyn((step + 1) * my_strides.0.value()), my_strides.1);
            let shape   = (nrows, my_shape.1);

            unsafe {
                let data = $ViewStorage::new_with_strides_unchecked($data, (row_start, 0), shape, strides);
                Matrix::from_data_statically_unchecked(data)
            }
        }

        /*
         *
         * Column slicing.
         *
         */
        /// Returns a view containing the i-th column of this matrix.
        #[inline]
        pub fn $column($me: $Me, i: usize) -> $MatrixView<'_, T, R, U1, S::RStride, S::CStride> {
            $me.$fixed_columns::<1>(i)
        }

        /// Returns a view containing the `n` first elements of the i-th column of this matrix.
        #[inline]
        pub fn $column_part($me: $Me, i: usize, n: usize) -> $MatrixView<'_, T, Dyn, U1, S::RStride, S::CStride> {
            $me.$generic_view((0, i), (Dyn(n), Const::<1>))
        }

        /// Extracts from this matrix a set of consecutive columns.
        #[inline]
        pub fn $columns($me: $Me, first_col: usize, ncols: usize)
            -> $MatrixView<'_, T, R, Dyn, S::RStride, S::CStride> {

            $me.$columns_generic(first_col, Dyn(ncols))
        }

        /// Extracts from this matrix a set of consecutive columns regularly skipping `step`
        /// columns.
        #[inline]
        pub fn $columns_with_step($me: $Me, first_col: usize, ncols: usize, step: usize)
            -> $MatrixView<'_, T, R, Dyn, S::RStride, Dyn> {

            $me.$columns_generic_with_step(first_col, Dyn(ncols), step)
        }

        /// Extracts a compile-time number of consecutive columns from this matrix.
        #[inline]
        pub fn $fixed_columns<const CVIEW: usize>($me: $Me, first_col: usize)
            -> $MatrixView<'_, T, R, Const<CVIEW>, S::RStride, S::CStride> {

            $me.$columns_generic(first_col, Const::<CVIEW>)
        }

        /// Extracts from this matrix a compile-time number of columns regularly skipping
        /// `step` columns.
        #[inline]
        pub fn $fixed_columns_with_step<const CVIEW: usize>($me: $Me, first_col: usize, step: usize)
            -> $MatrixView<'_, T, R, Const<CVIEW>, S::RStride, Dyn> {

            $me.$columns_generic_with_step(first_col, Const::<CVIEW>, step)
        }

        /// Extracts from this matrix `ncols` columns. The number of columns may or may not be
        /// known at compile-time.
        #[inline]
        pub fn $columns_generic<CView: Dim>($me: $Me, first_col: usize, ncols: CView)
            -> $MatrixView<'_, T, R, CView, S::RStride, S::CStride> {

            let my_shape = $me.shape_generic();
            $me.assert_view_index((0, first_col), (my_shape.0.value(), ncols.value()), (0, 0));
            let shape = (my_shape.0, ncols);

            unsafe {
                let data = $ViewStorage::new_unchecked($data, (0, first_col), shape);
                Matrix::from_data_statically_unchecked(data)
            }
        }


        /// Extracts from this matrix `ncols` columns skipping `step` columns. Both argument may
        /// or may not be values known at compile-time.
        #[inline]
        pub fn $columns_generic_with_step<CView: Dim>($me: $Me, first_col: usize, ncols: CView, step: usize)
            -> $MatrixView<'_, T, R, CView, S::RStride, Dyn> {

            let my_shape   = $me.shape_generic();
            let my_strides = $me.data.strides();

            $me.assert_view_index((0, first_col), (my_shape.0.value(), ncols.value()), (0, step));

            let strides = (my_strides.0, Dyn((step + 1) * my_strides.1.value()));
            let shape   = (my_shape.0, ncols);

            unsafe {
                let data = $ViewStorage::new_with_strides_unchecked($data, (0, first_col), shape, strides);
                Matrix::from_data_statically_unchecked(data)
            }
        }

        /*
         *
         * General slicing.
         *
         */
        /// Slices this matrix starting at its component `(irow, icol)` and with `(nrows, ncols)`
        /// consecutive elements.
        #[inline]
        #[deprecated = slice_deprecation_note!($view)]
        pub fn $slice($me: $Me, start: (usize, usize), shape: (usize, usize))
            -> $MatrixView<'_, T, Dyn, Dyn, S::RStride, S::CStride> {
            $me.$view(start, shape)
        }

        /// Return a view of this matrix starting at its component `(irow, icol)` and with `(nrows, ncols)`
        /// consecutive elements.
        #[inline]
        pub fn $view($me: $Me, start: (usize, usize), shape: (usize, usize))
            -> $MatrixView<'_, T, Dyn, Dyn, S::RStride, S::CStride> {

            $me.assert_view_index(start, shape, (0, 0));
            let shape = (Dyn(shape.0), Dyn(shape.1));

            unsafe {
                let data = $ViewStorage::new_unchecked($data, start, shape);
                Matrix::from_data_statically_unchecked(data)
            }
        }

        /// Slices this matrix starting at its component `(start.0, start.1)` and with
        /// `(shape.0, shape.1)` components. Each row (resp. column) of the sliced matrix is
        /// separated by `steps.0` (resp. `steps.1`) ignored rows (resp. columns) of the
        /// original matrix.
        #[inline]
        #[deprecated = slice_deprecation_note!($view_with_steps)]
        pub fn $slice_with_steps($me: $Me, start: (usize, usize), shape: (usize, usize), steps: (usize, usize))
            -> $MatrixView<'_, T, Dyn, Dyn, Dyn, Dyn> {
            $me.$view_with_steps(start, shape, steps)
        }

        /// Return a view of this matrix starting at its component `(start.0, start.1)` and with
        /// `(shape.0, shape.1)` components. Each row (resp. column) of the matrix view is
        /// separated by `steps.0` (resp. `steps.1`) ignored rows (resp. columns) of the
        /// original matrix.
        #[inline]
        pub fn $view_with_steps($me: $Me, start: (usize, usize), shape: (usize, usize), steps: (usize, usize))
            -> $MatrixView<'_, T, Dyn, Dyn, Dyn, Dyn> {
            let shape = (Dyn(shape.0), Dyn(shape.1));
            $me.$generic_view_with_steps(start, shape, steps)
        }

        /// Slices this matrix starting at its component `(irow, icol)` and with `(RVIEW, CVIEW)`
        /// consecutive components.
        #[inline]
        #[deprecated = slice_deprecation_note!($fixed_view)]
        pub fn $fixed_slice<const RVIEW: usize, const CVIEW: usize>($me: $Me, irow: usize, icol: usize)
            -> $MatrixView<'_, T, Const<RVIEW>, Const<CVIEW>, S::RStride, S::CStride> {
            $me.$fixed_view(irow, icol)
        }

        /// Return a view of this matrix starting at its component `(irow, icol)` and with
        /// `(RVIEW, CVIEW)` consecutive components.
        #[inline]
        pub fn $fixed_view<const RVIEW: usize, const CVIEW: usize>($me: $Me, irow: usize, icol: usize)
            -> $MatrixView<'_, T, Const<RVIEW>, Const<CVIEW>, S::RStride, S::CStride> {

            $me.assert_view_index((irow, icol), (RVIEW, CVIEW), (0, 0));
            let shape = (Const::<RVIEW>, Const::<CVIEW>);

            unsafe {
                let data = $ViewStorage::new_unchecked($data, (irow, icol), shape);
                Matrix::from_data_statically_unchecked(data)
            }
        }

        /// Slices this matrix starting at its component `(start.0, start.1)` and with
        /// `(RVIEW, CVIEW)` components. Each row (resp. column) of the sliced
        /// matrix is separated by `steps.0` (resp. `steps.1`) ignored rows (resp. columns) of
        /// the original matrix.
        #[inline]
        #[deprecated = slice_deprecation_note!($fixed_view_with_steps)]
        pub fn $fixed_slice_with_steps<const RVIEW: usize, const CVIEW: usize>($me: $Me, start: (usize, usize), steps: (usize, usize))
            -> $MatrixView<'_, T, Const<RVIEW>, Const<CVIEW>, Dyn, Dyn> {
            $me.$fixed_view_with_steps(start, steps)
        }

        /// Returns a view of this matrix starting at its component `(start.0, start.1)` and with
        /// `(RVIEW, CVIEW)` components. Each row (resp. column) of the matrix view
        /// is separated by `steps.0` (resp. `steps.1`) ignored rows (resp. columns) of
        /// the original matrix.
        #[inline]
        pub fn $fixed_view_with_steps<const RVIEW: usize, const CVIEW: usize>($me: $Me, start: (usize, usize), steps: (usize, usize))
            -> $MatrixView<'_, T, Const<RVIEW>, Const<CVIEW>, Dyn, Dyn> {
            let shape = (Const::<RVIEW>, Const::<CVIEW>);
            $me.$generic_view_with_steps(start, shape, steps)
        }

        /// Creates a slice that may or may not have a fixed size and stride.
        #[inline]
        #[deprecated = slice_deprecation_note!($generic_view)]
        pub fn $generic_slice<RView, CView>($me: $Me, start: (usize, usize), shape: (RView, CView))
            -> $MatrixView<'_, T, RView, CView, S::RStride, S::CStride>
            where RView: Dim,
                  CView: Dim {
            $me.$generic_view(start, shape)
        }

        /// Creates a matrix view that may or may not have a fixed size and stride.
        #[inline]
        pub fn $generic_view<RView, CView>($me: $Me, start: (usize, usize), shape: (RView, CView))
            -> $MatrixView<'_, T, RView, CView, S::RStride, S::CStride>
            where RView: Dim,
                  CView: Dim {

            $me.assert_view_index(start, (shape.0.value(), shape.1.value()), (0, 0));

            unsafe {
                let data = $ViewStorage::new_unchecked($data, start, shape);
                Matrix::from_data_statically_unchecked(data)
            }
        }

        /// Creates a slice that may or may not have a fixed size and stride.
        #[inline]
        #[deprecated = slice_deprecation_note!($generic_view_with_steps)]
        pub fn $generic_slice_with_steps<RView, CView>($me: $Me,
                                                         start: (usize, usize),
                                                         shape: (RView, CView),
                                                         steps: (usize, usize))
            -> $MatrixView<'_, T, RView, CView, Dyn, Dyn>
            where RView: Dim,
                  CView: Dim {
            $me.$generic_view_with_steps(start, shape, steps)
        }

        /// Creates a matrix view that may or may not have a fixed size and stride.
        #[inline]
        pub fn $generic_view_with_steps<RView, CView>($me: $Me,
                                                         start: (usize, usize),
                                                         shape: (RView, CView),
                                                         steps: (usize, usize))
            -> $MatrixView<'_, T, RView, CView, Dyn, Dyn>
            where RView: Dim,
                  CView: Dim {

            $me.assert_view_index(start, (shape.0.value(), shape.1.value()), steps);

            let my_strides = $me.data.strides();
            let strides    = (Dyn((steps.0 + 1) * my_strides.0.value()),
                              Dyn((steps.1 + 1) * my_strides.1.value()));

            unsafe {
                let data = $ViewStorage::new_with_strides_unchecked($data, start, shape, strides);
                Matrix::from_data_statically_unchecked(data)
            }
        }

        /*
         *
         * Splitting.
         *
         */
        /// Splits this `NxM` matrix into two parts delimited by two ranges.
        ///
        /// Panics if the ranges overlap or if the first range is empty.
        #[inline]
        pub fn $rows_range_pair<Range1: DimRange<R>, Range2: DimRange<R>>($me: $Me, r1: Range1, r2: Range2)
            -> ($MatrixView<'_, T, Range1::Size, C, S::RStride, S::CStride>,
                $MatrixView<'_, T, Range2::Size, C, S::RStride, S::CStride>) {

            let (nrows, ncols) = $me.shape_generic();
            let strides        = $me.data.strides();

            let start1 = r1.begin(nrows);
            let start2 = r2.begin(nrows);

            let end1 = r1.end(nrows);
            let end2 = r2.end(nrows);

            let nrows1 = r1.size(nrows);
            let nrows2 = r2.size(nrows);

            assert!(start2 >= end1 || start1 >= end2, "Rows range pair: the ranges must not overlap.");
            assert!(end2 <= nrows.value(), "Rows range pair: index out of range.");

            unsafe {
                let ptr1 = $data.$get_addr(start1, 0);
                let ptr2 = $data.$get_addr(start2, 0);

                let data1  = $ViewStorage::from_raw_parts(ptr1, (nrows1, ncols), strides);
                let data2  = $ViewStorage::from_raw_parts(ptr2, (nrows2, ncols), strides);
                let view1 = Matrix::from_data_statically_unchecked(data1);
                let view2 = Matrix::from_data_statically_unchecked(data2);

                (view1, view2)
            }
        }

        /// Splits this `NxM` matrix into two parts delimited by two ranges.
        ///
        /// Panics if the ranges overlap or if the first range is empty.
        #[inline]
        pub fn $columns_range_pair<Range1: DimRange<C>, Range2: DimRange<C>>($me: $Me, r1: Range1, r2: Range2)
            -> ($MatrixView<'_, T, R, Range1::Size, S::RStride, S::CStride>,
                $MatrixView<'_, T, R, Range2::Size, S::RStride, S::CStride>) {

            let (nrows, ncols) = $me.shape_generic();
            let strides        = $me.data.strides();

            let start1 = r1.begin(ncols);
            let start2 = r2.begin(ncols);

            let end1 = r1.end(ncols);
            let end2 = r2.end(ncols);

            let ncols1 = r1.size(ncols);
            let ncols2 = r2.size(ncols);

            assert!(start2 >= end1 || start1 >= end2, "Columns range pair: the ranges must not overlap.");
            assert!(end2 <= ncols.value(), "Columns range pair: index out of range.");

            unsafe {
                let ptr1 = $data.$get_addr(0, start1);
                let ptr2 = $data.$get_addr(0, start2);

                let data1  = $ViewStorage::from_raw_parts(ptr1, (nrows, ncols1), strides);
                let data2  = $ViewStorage::from_raw_parts(ptr2, (nrows, ncols2), strides);
                let view1 = Matrix::from_data_statically_unchecked(data1);
                let view2 = Matrix::from_data_statically_unchecked(data2);

                (view1, view2)
            }
        }
    }
);

/// A matrix slice.
///
/// This type alias exists only for legacy purposes and is deprecated. It will be removed
/// in a future release. Please use [`MatrixView`] instead.
/// See [issue #1076](https://github.com/dimforge/nalgebra/issues/1076)
/// for the rationale.
#[deprecated = "Use MatrixView instead."]
pub type MatrixSlice<'a, T, R, C, RStride = U1, CStride = R> =
    MatrixView<'a, T, R, C, RStride, CStride>;

/// A matrix view.
pub type MatrixView<'a, T, R, C, RStride = U1, CStride = R> =
    Matrix<T, R, C, ViewStorage<'a, T, R, C, RStride, CStride>>;

/// A mutable matrix slice.
///
/// This type alias exists only for legacy purposes and is deprecated. It will be removed
/// in a future release. Please use [`MatrixViewMut`] instead.
/// See [issue #1076](https://github.com/dimforge/nalgebra/issues/1076)
/// for the rationale.
#[deprecated = "Use MatrixViewMut instead."]
pub type MatrixSliceMut<'a, T, R, C, RStride = U1, CStride = R> =
    MatrixViewMut<'a, T, R, C, RStride, CStride>;

/// A mutable matrix view.
pub type MatrixViewMut<'a, T, R, C, RStride = U1, CStride = R> =
    Matrix<T, R, C, ViewStorageMut<'a, T, R, C, RStride, CStride>>;

/// # Views based on index and length
impl<T, R: Dim, C: Dim, S: RawStorage<T, R, C>> Matrix<T, R, C, S> {
    matrix_view_impl!(
     self: &Self, MatrixView, ViewStorage, RawStorage.get_address_unchecked(), &self.data;
     row,
     row_part,
     rows,
     rows_with_step,
     fixed_rows,
     fixed_rows_with_step,
     rows_generic,
     rows_generic_with_step,
     column,
     column_part,
     columns,
     columns_with_step,
     fixed_columns,
     fixed_columns_with_step,
     columns_generic,
     columns_generic_with_step,
     slice => view,
     slice_with_steps => view_with_steps,
     fixed_slice => fixed_view,
     fixed_slice_with_steps => fixed_view_with_steps,
     generic_slice => generic_view,
     generic_slice_with_steps => generic_view_with_steps,
     rows_range_pair,
     columns_range_pair);
}

/// # Mutable views based on index and length
impl<T, R: Dim, C: Dim, S: RawStorageMut<T, R, C>> Matrix<T, R, C, S> {
    matrix_view_impl!(
     self: &mut Self, MatrixViewMut, ViewStorageMut, RawStorageMut.get_address_unchecked_mut(), &mut self.data;
     row_mut,
     row_part_mut,
     rows_mut,
     rows_with_step_mut,
     fixed_rows_mut,
     fixed_rows_with_step_mut,
     rows_generic_mut,
     rows_generic_with_step_mut,
     column_mut,
     column_part_mut,
     columns_mut,
     columns_with_step_mut,
     fixed_columns_mut,
     fixed_columns_with_step_mut,
     columns_generic_mut,
     columns_generic_with_step_mut,
     slice_mut => view_mut,
     slice_with_steps_mut => view_with_steps_mut,
     fixed_slice_mut => fixed_view_mut,
     fixed_slice_with_steps_mut => fixed_view_with_steps_mut,
     generic_slice_mut => generic_view_mut,
     generic_slice_with_steps_mut => generic_view_with_steps_mut,
     rows_range_pair_mut,
     columns_range_pair_mut);
}

/// A range with a size that may be known at compile-time.
///
/// This may be:
/// * A single `usize` index, e.g., `4`
/// * A left-open range `std::ops::RangeTo`, e.g., `.. 4`
/// * A right-open range `std::ops::RangeFrom`, e.g., `4 ..`
/// * A full range `std::ops::RangeFull`, e.g., `..`
pub trait DimRange<D: Dim> {
    /// Type of the range size. May be a type-level integer.
    type Size: Dim;

    /// The start index of the range.
    fn begin(&self, shape: D) -> usize;
    // NOTE: this is the index immediately after the last index.
    /// The index immediately after the last index inside of the range.
    fn end(&self, shape: D) -> usize;
    /// The number of elements of the range, i.e., `self.end - self.begin`.
    fn size(&self, shape: D) -> Self::Size;
}

/// A range with a size that may be known at compile-time.
///
/// This is merely a legacy trait alias to minimize breakage. Use the [`DimRange`] trait instead.
#[deprecated = slice_deprecation_note!(DimRange)]
pub trait SliceRange<D: Dim>: DimRange<D> {}

#[allow(deprecated)]
impl<R: DimRange<D>, D: Dim> SliceRange<D> for R {}

impl<D: Dim> DimRange<D> for usize {
    type Size = U1;

    #[inline(always)]
    fn begin(&self, _: D) -> usize {
        *self
    }

    #[inline(always)]
    fn end(&self, _: D) -> usize {
        *self + 1
    }

    #[inline(always)]
    fn size(&self, _: D) -> Self::Size {
        Const::<1>
    }
}

impl<D: Dim> DimRange<D> for Range<usize> {
    type Size = Dyn;

    #[inline(always)]
    fn begin(&self, _: D) -> usize {
        self.start
    }

    #[inline(always)]
    fn end(&self, _: D) -> usize {
        self.end
    }

    #[inline(always)]
    fn size(&self, _: D) -> Self::Size {
        Dyn(self.end - self.start)
    }
}

impl<D: Dim> DimRange<D> for RangeFrom<usize> {
    type Size = Dyn;

    #[inline(always)]
    fn begin(&self, _: D) -> usize {
        self.start
    }

    #[inline(always)]
    fn end(&self, dim: D) -> usize {
        dim.value()
    }

    #[inline(always)]
    fn size(&self, dim: D) -> Self::Size {
        Dyn(dim.value() - self.start)
    }
}

impl<D: Dim> DimRange<D> for RangeTo<usize> {
    type Size = Dyn;

    #[inline(always)]
    fn begin(&self, _: D) -> usize {
        0
    }

    #[inline(always)]
    fn end(&self, _: D) -> usize {
        self.end
    }

    #[inline(always)]
    fn size(&self, _: D) -> Self::Size {
        Dyn(self.end)
    }
}

impl<D: Dim> DimRange<D> for RangeFull {
    type Size = D;

    #[inline(always)]
    fn begin(&self, _: D) -> usize {
        0
    }

    #[inline(always)]
    fn end(&self, dim: D) -> usize {
        dim.value()
    }

    #[inline(always)]
    fn size(&self, dim: D) -> Self::Size {
        dim
    }
}

impl<D: Dim> DimRange<D> for RangeInclusive<usize> {
    type Size = Dyn;

    #[inline(always)]
    fn begin(&self, _: D) -> usize {
        *self.start()
    }

    #[inline(always)]
    fn end(&self, _: D) -> usize {
        *self.end() + 1
    }

    #[inline(always)]
    fn size(&self, _: D) -> Self::Size {
        Dyn(*self.end() + 1 - *self.start())
    }
}

// TODO: see how much of this overlaps with the general indexing
// methods from indexing.rs.
impl<T, R: Dim, C: Dim, S: RawStorage<T, R, C>> Matrix<T, R, C, S> {
    /// Slices a sub-matrix containing the rows indexed by the range `rows` and the columns indexed
    /// by the range `cols`.
    #[inline]
    #[must_use]
    #[deprecated = slice_deprecation_note!(view_range)]
    pub fn slice_range<RowRange, ColRange>(
        &self,
        rows: RowRange,
        cols: ColRange,
    ) -> MatrixView<'_, T, RowRange::Size, ColRange::Size, S::RStride, S::CStride>
    where
        RowRange: DimRange<R>,
        ColRange: DimRange<C>,
    {
        let (nrows, ncols) = self.shape_generic();
        self.generic_view(
            (rows.begin(nrows), cols.begin(ncols)),
            (rows.size(nrows), cols.size(ncols)),
        )
    }

    /// Returns a view containing the rows indexed by the range `rows` and the columns indexed
    /// by the range `cols`.
    #[inline]
    #[must_use]
    pub fn view_range<RowRange, ColRange>(
        &self,
        rows: RowRange,
        cols: ColRange,
    ) -> MatrixView<'_, T, RowRange::Size, ColRange::Size, S::RStride, S::CStride>
    where
        RowRange: DimRange<R>,
        ColRange: DimRange<C>,
    {
        let (nrows, ncols) = self.shape_generic();
        self.generic_view(
            (rows.begin(nrows), cols.begin(ncols)),
            (rows.size(nrows), cols.size(ncols)),
        )
    }

    /// View containing all the rows indexed by the range `rows`.
    #[inline]
    #[must_use]
    pub fn rows_range<RowRange: DimRange<R>>(
        &self,
        rows: RowRange,
    ) -> MatrixView<'_, T, RowRange::Size, C, S::RStride, S::CStride> {
        self.view_range(rows, ..)
    }

    /// View containing all the columns indexed by the range `rows`.
    #[inline]
    #[must_use]
    pub fn columns_range<ColRange: DimRange<C>>(
        &self,
        cols: ColRange,
    ) -> MatrixView<'_, T, R, ColRange::Size, S::RStride, S::CStride> {
        self.view_range(.., cols)
    }
}

// TODO: see how much of this overlaps with the general indexing
// methods from indexing.rs.
impl<T, R: Dim, C: Dim, S: RawStorageMut<T, R, C>> Matrix<T, R, C, S> {
    /// Slices a mutable sub-matrix containing the rows indexed by the range `rows` and the columns
    /// indexed by the range `cols`.
    #[deprecated = slice_deprecation_note!(view_range_mut)]
    pub fn slice_range_mut<RowRange, ColRange>(
        &mut self,
        rows: RowRange,
        cols: ColRange,
    ) -> MatrixViewMut<'_, T, RowRange::Size, ColRange::Size, S::RStride, S::CStride>
    where
        RowRange: DimRange<R>,
        ColRange: DimRange<C>,
    {
        self.view_range_mut(rows, cols)
    }

    /// Return a mutable view containing the rows indexed by the range `rows` and the columns
    /// indexed by the range `cols`.
    pub fn view_range_mut<RowRange, ColRange>(
        &mut self,
        rows: RowRange,
        cols: ColRange,
    ) -> MatrixViewMut<'_, T, RowRange::Size, ColRange::Size, S::RStride, S::CStride>
    where
        RowRange: DimRange<R>,
        ColRange: DimRange<C>,
    {
        let (nrows, ncols) = self.shape_generic();
        self.generic_view_mut(
            (rows.begin(nrows), cols.begin(ncols)),
            (rows.size(nrows), cols.size(ncols)),
        )
    }

    /// Mutable view containing all the rows indexed by the range `rows`.
    #[inline]
    pub fn rows_range_mut<RowRange: DimRange<R>>(
        &mut self,
        rows: RowRange,
    ) -> MatrixViewMut<'_, T, RowRange::Size, C, S::RStride, S::CStride> {
        self.view_range_mut(rows, ..)
    }

    /// Mutable view containing all the columns indexed by the range `cols`.
    #[inline]
    pub fn columns_range_mut<ColRange: DimRange<C>>(
        &mut self,
        cols: ColRange,
    ) -> MatrixViewMut<'_, T, R, ColRange::Size, S::RStride, S::CStride> {
        self.view_range_mut(.., cols)
    }
}

impl<'a, T, R, C, RStride, CStride> From<MatrixViewMut<'a, T, R, C, RStride, CStride>>
    for MatrixView<'a, T, R, C, RStride, CStride>
where
    R: Dim,
    C: Dim,
    RStride: Dim,
    CStride: Dim,
{
    fn from(view_mut: MatrixViewMut<'a, T, R, C, RStride, CStride>) -> Self {
        let data = ViewStorage {
            ptr: view_mut.data.ptr,
            shape: view_mut.data.shape,
            strides: view_mut.data.strides,
            _phantoms: PhantomData,
        };

        unsafe { Matrix::from_data_statically_unchecked(data) }
    }
}

impl<T, R, C, S> Matrix<T, R, C, S>
where
    R: Dim,
    C: Dim,
    S: RawStorage<T, R, C>,
{
    /// Returns this matrix as a view.
    ///
    /// The returned view type is generally ambiguous unless specified.
    /// This is particularly useful when working with functions or methods that take
    /// matrix views as input.
    ///
    /// # Panics
    /// Panics if the dimensions of the view and the matrix are not compatible and this cannot
    /// be proven at compile-time. This might happen, for example, when constructing a static
    /// view of size 3x3 from a dynamically sized matrix of dimension 5x5.
    ///
    /// # Examples
    /// ```
    /// use nalgebra::{DMatrixSlice, SMatrixView};
    ///
    /// fn consume_view(_: DMatrixSlice<f64>) {}
    ///
    /// let matrix = nalgebra::Matrix3::zeros();
    /// consume_view(matrix.as_view());
    ///
    /// let dynamic_view: DMatrixSlice<f64> = matrix.as_view();
    /// let static_view_from_dyn: SMatrixView<f64, 3, 3> = dynamic_view.as_view();
    /// ```
    pub fn as_view<RView, CView, RViewStride, CViewStride>(
        &self,
    ) -> MatrixView<'_, T, RView, CView, RViewStride, CViewStride>
    where
        RView: Dim,
        CView: Dim,
        RViewStride: Dim,
        CViewStride: Dim,
        ShapeConstraint: DimEq<R, RView>
            + DimEq<C, CView>
            + DimEq<RViewStride, S::RStride>
            + DimEq<CViewStride, S::CStride>,
    {
        // Defer to (&matrix).into()
        self.into()
    }
}

impl<T, R, C, S> Matrix<T, R, C, S>
where
    R: Dim,
    C: Dim,
    S: RawStorageMut<T, R, C>,
{
    /// Returns this matrix as a mutable view.
    ///
    /// The returned view type is generally ambiguous unless specified.
    /// This is particularly useful when working with functions or methods that take
    /// matrix views as input.
    ///
    /// # Panics
    /// Panics if the dimensions of the view and the matrix are not compatible and this cannot
    /// be proven at compile-time. This might happen, for example, when constructing a static
    /// view of size 3x3 from a dynamically sized matrix of dimension 5x5.
    ///
    /// # Examples
    /// ```
    /// use nalgebra::{DMatrixViewMut, SMatrixViewMut};
    ///
    /// fn consume_view(_: DMatrixViewMut<f64>) {}
    ///
    /// let mut matrix = nalgebra::Matrix3::zeros();
    /// consume_view(matrix.as_view_mut());
    ///
    /// let mut dynamic_view: DMatrixViewMut<f64> = matrix.as_view_mut();
    /// let static_view_from_dyn: SMatrixViewMut<f64, 3, 3> = dynamic_view.as_view_mut();
    /// ```
    pub fn as_view_mut<RView, CView, RViewStride, CViewStride>(
        &mut self,
    ) -> MatrixViewMut<'_, T, RView, CView, RViewStride, CViewStride>
    where
        RView: Dim,
        CView: Dim,
        RViewStride: Dim,
        CViewStride: Dim,
        ShapeConstraint: DimEq<R, RView>
            + DimEq<C, CView>
            + DimEq<RViewStride, S::RStride>
            + DimEq<CViewStride, S::CStride>,
    {
        // Defer to (&mut matrix).into()
        self.into()
    }
}

// TODO: Arbitrary strides?
impl<'a, T, R1, C1, R2, C2> ReshapableStorage<T, R1, C1, R2, C2>
    for ViewStorage<'a, T, R1, C1, U1, R1>
where
    T: Scalar,
    R1: Dim,
    C1: Dim,
    R2: Dim,
    C2: Dim,
{
    type Output = ViewStorage<'a, T, R2, C2, U1, R2>;

    fn reshape_generic(self, nrows: R2, ncols: C2) -> Self::Output {
        let (r1, c1) = self.shape();
        assert_eq!(nrows.value() * ncols.value(), r1.value() * c1.value());
        let ptr = self.ptr();
        let new_shape = (nrows, ncols);
        let strides = (U1::name(), nrows);
        unsafe { ViewStorage::from_raw_parts(ptr, new_shape, strides) }
    }
}

// TODO: Arbitrary strides?
impl<'a, T, R1, C1, R2, C2> ReshapableStorage<T, R1, C1, R2, C2>
    for ViewStorageMut<'a, T, R1, C1, U1, R1>
where
    T: Scalar,
    R1: Dim,
    C1: Dim,
    R2: Dim,
    C2: Dim,
{
    type Output = ViewStorageMut<'a, T, R2, C2, U1, R2>;

    fn reshape_generic(mut self, nrows: R2, ncols: C2) -> Self::Output {
        let (r1, c1) = self.shape();
        assert_eq!(nrows.value() * ncols.value(), r1.value() * c1.value());
        let ptr = self.ptr_mut();
        let new_shape = (nrows, ncols);
        let strides = (U1::name(), nrows);
        unsafe { ViewStorageMut::from_raw_parts(ptr, new_shape, strides) }
    }
}
