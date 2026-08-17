//! Abstract definition of a matrix data storage.

use std::ptr;

use crate::base::Scalar;
use crate::base::allocator::{Allocator, SameShapeC, SameShapeR};
use crate::base::default_allocator::DefaultAllocator;
use crate::base::dimension::{Dim, U1};

/*
 * Aliases for allocation results.
 */
/// The data storage for the sum of two matrices with dimensions `(R1, C1)` and `(R2, C2)`.
pub type SameShapeStorage<T, R1, C1, R2, C2> =
    <DefaultAllocator as Allocator<SameShapeR<R1, R2>, SameShapeC<C1, C2>>>::Buffer<T>;

// TODO: better name than Owned ?
/// The owned data storage that can be allocated from `S`.
pub type Owned<T, R, C = U1> = <DefaultAllocator as Allocator<R, C>>::Buffer<T>;

/// The owned data storage that can be allocated from `S`.
pub type OwnedUninit<T, R, C = U1> = <DefaultAllocator as Allocator<R, C>>::BufferUninit<T>;

/// The row-stride of the owned data storage for a buffer of dimension `(R, C)`.
pub type RStride<T, R, C = U1> =
    <<DefaultAllocator as Allocator<R, C>>::Buffer<T> as RawStorage<T, R, C>>::RStride;

/// The column-stride of the owned data storage for a buffer of dimension `(R, C)`.
pub type CStride<T, R, C = U1> =
    <<DefaultAllocator as Allocator<R, C>>::Buffer<T> as RawStorage<T, R, C>>::CStride;

/// The trait shared by all matrix data storage.
///
/// TODO: doc
/// # Safety
///
/// In generic code, it is recommended use the `Storage` trait bound instead. The `RawStorage`
/// trait bound is generally used by code that needs to work with storages that contains
/// `MaybeUninit<T>` elements.
///
/// Note that `Self` must always have a number of elements compatible with the matrix length (given
/// by `R` and `C` if they are known at compile-time). For example, implementors of this trait
/// should **not** allow the user to modify the size of the underlying buffer with safe methods
/// (for example the `VecStorage::data_mut` method is unsafe because the user could change the
/// vector's size so that it no longer contains enough elements: this will lead to UB.
pub unsafe trait RawStorage<T, R: Dim, C: Dim = U1>: Sized {
    /// The static stride of this storage's rows.
    type RStride: Dim;

    /// The static stride of this storage's columns.
    type CStride: Dim;

    /// The matrix data pointer.
    fn ptr(&self) -> *const T;

    /// The dimension of the matrix at run-time. Arr length of zero indicates the additive identity
    /// element of any dimension. Must be equal to `Self::dimension()` if it is not `None`.
    fn shape(&self) -> (R, C);

    /// The relative offset in the underlying storage corresponding to a change in position by one row or column respectively.
    ///
    /// For example this returns `(1, 5)` for a column-major matrix with 5 columns.
    fn strides(&self) -> (Self::RStride, Self::CStride);

    /// Compute the index corresponding to the irow-th row and icol-th column of this matrix. The
    /// index must be such that the following holds:
    ///
    /// ```ignore
    /// let lindex = self.linear_index(irow, icol);
    /// assert!(*self.get_unchecked(irow, icol) == *self.get_unchecked_linear(lindex))
    /// ```
    #[inline]
    fn linear_index(&self, irow: usize, icol: usize) -> usize {
        let (rstride, cstride) = self.strides();

        irow * rstride.value() + icol * cstride.value()
    }

    /// Gets the address of the i-th matrix component without performing bound-checking.
    ///
    /// # Safety
    /// If the index is out of bounds, dereferencing the result will cause undefined behavior.
    #[inline]
    fn get_address_unchecked_linear(&self, i: usize) -> *const T {
        self.ptr().wrapping_add(i)
    }

    /// Gets the address of the i-th matrix component without performing bound-checking.
    ///
    /// # Safety
    /// If the index is out of bounds, dereferencing the result will cause undefined behavior.
    #[inline]
    fn get_address_unchecked(&self, irow: usize, icol: usize) -> *const T {
        self.get_address_unchecked_linear(self.linear_index(irow, icol))
    }

    /// Retrieves a reference to the i-th element without bound-checking.
    ///
    /// # Safety
    /// If the index is out of bounds, the method will cause undefined behavior.
    #[inline]
    unsafe fn get_unchecked_linear(&self, i: usize) -> &T {
        unsafe { &*self.get_address_unchecked_linear(i) }
    }

    /// Retrieves a reference to the i-th element without bound-checking.
    ///
    /// # Safety
    /// If the index is out of bounds, the method will cause undefined behavior.
    #[inline]
    unsafe fn get_unchecked(&self, irow: usize, icol: usize) -> &T {
        unsafe { self.get_unchecked_linear(self.linear_index(irow, icol)) }
    }

    /// Indicates whether this data buffer stores its elements contiguously.
    ///
    /// # Safety
    /// This function must not return `true` if the underlying storage is not contiguous,
    /// or undefined behaviour will occur.
    fn is_contiguous(&self) -> bool;

    /// Retrieves the data buffer as a contiguous slice.
    ///
    /// # Safety
    /// The matrix components may not be stored in a contiguous way, depending on the strides.
    /// This method is unsafe because this can yield to invalid aliasing when called on some pairs
    /// of matrix views originating from the same matrix with strides.
    ///
    /// Call the safe alternative `matrix.as_slice()` instead.
    unsafe fn as_slice_unchecked(&self) -> &[T];
}

/// Trait shared by all matrix data storage that don’t contain any uninitialized elements.
///
/// # Safety
///
/// Note that `Self` must always have a number of elements compatible with the matrix length (given
/// by `R` and `C` if they are known at compile-time). For example, implementors of this trait
/// should **not** allow the user to modify the size of the underlying buffer with safe methods
/// (for example the `VecStorage::data_mut` method is unsafe because the user could change the
/// vector's size so that it no longer contains enough elements: this will lead to UB.
pub unsafe trait Storage<T: Scalar, R: Dim, C: Dim = U1>: RawStorage<T, R, C> {
    /// Builds a matrix data storage that does not contain any reference.
    fn into_owned(self) -> Owned<T, R, C>
    where
        DefaultAllocator: Allocator<R, C>;

    /// Clones this data storage to one that does not contain any reference.
    fn clone_owned(&self) -> Owned<T, R, C>
    where
        DefaultAllocator: Allocator<R, C>;

    /// Drops the storage without calling the destructors on the contained elements.
    fn forget_elements(self);
}

/// Trait implemented by matrix data storage that can provide a mutable access to its elements.
///
/// # Safety
///
/// In generic code, it is recommended use the `StorageMut` trait bound instead. The
/// `RawStorageMut` trait bound is generally used by code that needs to work with storages that
/// contains `MaybeUninit<T>` elements.
///
/// Note that a mutable access does not mean that the matrix owns its data. For example, a mutable
/// matrix view can provide mutable access to its elements even if it does not own its data (it
/// contains only an internal reference to them).
pub unsafe trait RawStorageMut<T, R: Dim, C: Dim = U1>: RawStorage<T, R, C> {
    /// The matrix mutable data pointer.
    fn ptr_mut(&mut self) -> *mut T;

    /// Gets the mutable address of the i-th matrix component without performing bound-checking.
    ///
    /// # Safety
    /// If the index is out of bounds, dereferencing the result will cause undefined behavior.
    #[inline]
    fn get_address_unchecked_linear_mut(&mut self, i: usize) -> *mut T {
        self.ptr_mut().wrapping_add(i)
    }

    /// Gets the mutable address of the i-th matrix component without performing bound-checking.
    ///
    /// # Safety
    /// If the index is out of bounds, dereferencing the result will cause undefined behavior.
    #[inline]
    fn get_address_unchecked_mut(&mut self, irow: usize, icol: usize) -> *mut T {
        let lid = self.linear_index(irow, icol);
        self.get_address_unchecked_linear_mut(lid)
    }

    /// Retrieves a mutable reference to the i-th element without bound-checking.
    ///
    /// # Safety
    /// If the index is out of bounds, the method will cause undefined behavior.
    unsafe fn get_unchecked_linear_mut(&mut self, i: usize) -> &mut T {
        unsafe { &mut *self.get_address_unchecked_linear_mut(i) }
    }

    /// Retrieves a mutable reference to the element at `(irow, icol)` without bound-checking.
    ///
    /// # Safety
    /// If the index is out of bounds, the method will cause undefined behavior.
    #[inline]
    unsafe fn get_unchecked_mut(&mut self, irow: usize, icol: usize) -> &mut T {
        unsafe { &mut *self.get_address_unchecked_mut(irow, icol) }
    }

    /// Swaps two elements using their linear index without bound-checking.
    ///
    /// # Safety
    /// If the indices are out of bounds, the method will cause undefined behavior.
    ///
    /// # Validity
    /// The default implementation of this trait function is only guaranteed to be
    /// sound if invocations of `self.ptr_mut()` and `self.get_address_unchecked_linear_mut()`
    /// result in stable references. If any of the data pointed to by these trait methods
    /// moves as a consequence of invoking either of these methods then this default
    /// trait implementation may be invalid or unsound and should be overridden.
    #[inline]
    unsafe fn swap_unchecked_linear(&mut self, i1: usize, i2: usize) {
        unsafe {
            // we can't just use the pointers returned from `get_address_unchecked_linear_mut` because calling a
            // method taking self mutably invalidates any existing (mutable) pointers. since `get_address_unchecked_linear_mut` can
            // also be overridden by a custom implementation, we can't just use `wrapping_add` assuming that's what the method does.
            // instead, we use `offset_from` to compute the re-calculate the pointers from the base pointer.
            // this is sound as long as this trait matches the Validity preconditions
            // (and it's the caller's responsibility to ensure the indices are in-bounds).
            let base = self.ptr_mut();
            let offset1 = self.get_address_unchecked_linear_mut(i1).offset_from(base);
            let offset2 = self.get_address_unchecked_linear_mut(i2).offset_from(base);

            let base = self.ptr_mut();
            let a = base.offset(offset1);
            let b = base.offset(offset2);

            ptr::swap(a, b);
        }
    }

    /// Swaps two elements without bound-checking.
    ///
    /// # Safety
    /// If the indices are out of bounds, the method will cause undefined behavior.
    #[inline]
    unsafe fn swap_unchecked(&mut self, row_col1: (usize, usize), row_col2: (usize, usize)) {
        unsafe {
            let lid1 = self.linear_index(row_col1.0, row_col1.1);
            let lid2 = self.linear_index(row_col2.0, row_col2.1);

            self.swap_unchecked_linear(lid1, lid2)
        }
    }

    /// Retrieves the mutable data buffer as a contiguous slice.
    ///
    /// Matrix components may not be contiguous, depending on its strides.    
    ///
    /// # Safety
    /// The matrix components may not be stored in a contiguous way, depending on the strides.
    /// This method is unsafe because this can yield to invalid aliasing when called on some pairs
    /// of matrix slices originating from the same matrix with strides.
    unsafe fn as_mut_slice_unchecked(&mut self) -> &mut [T];
}

/// Trait shared by all mutable matrix data storage that don’t contain any uninitialized elements.
///
/// # Safety
///
/// See safety note for `Storage`, `RawStorageMut`.
pub unsafe trait StorageMut<T: Scalar, R: Dim, C: Dim = U1>:
    Storage<T, R, C> + RawStorageMut<T, R, C>
{
}

unsafe impl<S, T: Scalar, R, C> StorageMut<T, R, C> for S
where
    R: Dim,
    C: Dim,
    S: Storage<T, R, C> + RawStorageMut<T, R, C>,
{
}

/// Marker trait indicating that a storage is stored contiguously in memory.
///
/// # Safety
///
/// The storage requirement means that for any value of `i` in `[0, nrows * ncols - 1]`, the value
/// `.get_unchecked_linear` returns one of the matrix component. This trait is unsafe because
/// failing to comply to this may cause Undefined Behaviors.
pub unsafe trait IsContiguous {}

/// A matrix storage that can be reshaped in-place.
pub trait ReshapableStorage<T, R1, C1, R2, C2>: RawStorage<T, R1, C1>
where
    T: Scalar,
    R1: Dim,
    C1: Dim,
    R2: Dim,
    C2: Dim,
{
    /// The reshaped storage type.
    type Output: RawStorage<T, R2, C2>;

    /// Reshapes the storage into the output storage type.
    fn reshape_generic(self, nrows: R2, ncols: C2) -> Self::Output;
}
