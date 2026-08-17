#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::vec::Vec;

use crate::base::allocator::Allocator;
use crate::base::constraint::{SameNumberOfRows, ShapeConstraint};
use crate::base::default_allocator::DefaultAllocator;
use crate::base::dimension::{Dim, DimName, Dyn, U1};
use crate::base::storage::{IsContiguous, Owned, RawStorage, RawStorageMut, ReshapableStorage};
use crate::base::{Scalar, Vector};

#[cfg(feature = "serde-serialize-no-std")]
use serde::{
    de::{Deserialize, Deserializer, Error},
    ser::{Serialize, Serializer},
};

use crate::Storage;
use std::mem::MaybeUninit;

/*
 *
 * RawStorage.
 *
 */
/// A Vec-based matrix data storage. It may be dynamically-sized.
#[repr(C)]
#[derive(Eq, Debug, Clone, PartialEq)]
pub struct VecStorage<T, R: Dim, C: Dim> {
    data: Vec<T>,
    nrows: R,
    ncols: C,
}

impl<T> Default for VecStorage<T, Dyn, Dyn> {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            nrows: Dyn::from_usize(0),
            ncols: Dyn::from_usize(0),
        }
    }
}

impl<T, R: DimName> Default for VecStorage<T, R, Dyn> {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            nrows: R::name(),
            ncols: Dyn::from_usize(0),
        }
    }
}

impl<T, C: DimName> Default for VecStorage<T, Dyn, C> {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            nrows: Dyn::from_usize(0),
            ncols: C::name(),
        }
    }
}

impl<T: Default, R: DimName, C: DimName> Default for VecStorage<T, R, C> {
    fn default() -> Self {
        let nrows = R::name();
        let ncols = C::name();
        let mut data = Vec::new();
        data.resize_with(nrows.value() * ncols.value(), Default::default);
        Self { data, nrows, ncols }
    }
}

#[cfg(feature = "serde-serialize")]
impl<T, R: Dim, C: Dim> Serialize for VecStorage<T, R, C>
where
    T: Serialize,
    R: Serialize,
    C: Serialize,
{
    fn serialize<Ser>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error>
    where
        Ser: Serializer,
    {
        (&self.data, &self.nrows, &self.ncols).serialize(serializer)
    }
}

#[cfg(feature = "serde-serialize")]
impl<'a, T, R: Dim, C: Dim> Deserialize<'a> for VecStorage<T, R, C>
where
    T: Deserialize<'a>,
    R: Deserialize<'a>,
    C: Deserialize<'a>,
{
    fn deserialize<Des>(deserializer: Des) -> Result<Self, Des::Error>
    where
        Des: Deserializer<'a>,
    {
        let (data, nrows, ncols): (Vec<T>, R, C) = Deserialize::deserialize(deserializer)?;

        // SAFETY: make sure the data we deserialize have the
        //         correct number of elements.
        if nrows.value() * ncols.value() != data.len() {
            return Err(Des::Error::custom(format!(
                "Expected {} components, found {}",
                nrows.value() * ncols.value(),
                data.len()
            )));
        }

        Ok(Self { data, nrows, ncols })
    }
}

#[deprecated(note = "renamed to `VecStorage`")]
/// Renamed to [`VecStorage`].
pub type MatrixVec<T, R, C> = VecStorage<T, R, C>;

impl<T, R: Dim, C: Dim> VecStorage<T, R, C> {
    /// Creates a new dynamic matrix data storage from the given vector and shape.
    #[inline]
    pub fn new(nrows: R, ncols: C, data: Vec<T>) -> Self {
        assert!(
            nrows.value() * ncols.value() == data.len(),
            "Data storage buffer dimension mismatch."
        );
        Self { data, nrows, ncols }
    }

    /// The underlying data storage.
    #[inline]
    #[must_use]
    pub const fn as_vec(&self) -> &Vec<T> {
        &self.data
    }

    /// The underlying mutable data storage.
    ///
    /// # Safety
    /// This is unsafe because this may cause UB if the size of the vector is changed
    /// by the user.
    #[inline]
    pub const unsafe fn as_vec_mut(&mut self) -> &mut Vec<T> {
        &mut self.data
    }

    /// Resizes the underlying mutable data storage and unwraps it.
    ///
    /// # Safety
    /// - If `sz` is larger than the current size, additional elements are uninitialized.
    /// - If `sz` is smaller than the current size, additional elements are truncated but **not** dropped.
    ///   It is the responsibility of the caller of this method to drop these elements.
    #[inline]
    pub unsafe fn resize(mut self, sz: usize) -> Vec<MaybeUninit<T>> {
        unsafe {
            let len = self.len();

            let new_data = if sz < len {
                // Use `set_len` instead of `truncate` because we don’t want to
                // drop the removed elements (it’s the caller’s responsibility).
                self.data.set_len(sz);
                self.data.shrink_to_fit();

                // Safety:
                // - MaybeUninit<T> has the same alignment and layout as T.
                // - The length and capacity come from a valid vector.
                Vec::from_raw_parts(
                    self.data.as_mut_ptr() as *mut MaybeUninit<T>,
                    self.data.len(),
                    self.data.capacity(),
                )
            } else {
                self.data.reserve_exact(sz - len);

                // Safety:
                // - MaybeUninit<T> has the same alignment and layout as T.
                // - The length and capacity come from a valid vector.
                let mut new_data = Vec::from_raw_parts(
                    self.data.as_mut_ptr() as *mut MaybeUninit<T>,
                    self.data.len(),
                    self.data.capacity(),
                );

                // Safety: we can set the length here because MaybeUninit is always assumed
                //         to be initialized.
                new_data.set_len(sz);
                new_data
            };

            // Avoid double-free by forgetting `self` because its data buffer has
            // been transferred to `new_data`.
            std::mem::forget(self);
            new_data
        }
    }

    /// The number of elements on the underlying vector.
    #[inline]
    #[must_use]
    pub const fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns true if the underlying vector contains no elements.
    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// A slice containing all the components stored in this storage in column-major order.
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        &self.data[..]
    }

    /// A mutable slice containing all the components stored in this storage in column-major order.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.data[..]
    }
}

impl<T, R: Dim, C: Dim> From<VecStorage<T, R, C>> for Vec<T> {
    fn from(vec: VecStorage<T, R, C>) -> Self {
        vec.data
    }
}

/*
 *
 * Dyn − Static
 * Dyn − Dyn
 *
 */
unsafe impl<T, C: Dim> RawStorage<T, Dyn, C> for VecStorage<T, Dyn, C> {
    type RStride = U1;
    type CStride = Dyn;

    #[inline]
    fn ptr(&self) -> *const T {
        self.data.as_ptr()
    }

    #[inline]
    fn shape(&self) -> (Dyn, C) {
        (self.nrows, self.ncols)
    }

    #[inline]
    fn strides(&self) -> (Self::RStride, Self::CStride) {
        (Self::RStride::name(), self.nrows)
    }

    #[inline]
    fn is_contiguous(&self) -> bool {
        true
    }

    #[inline]
    unsafe fn as_slice_unchecked(&self) -> &[T] {
        &self.data
    }
}

unsafe impl<T: Scalar, C: Dim> Storage<T, Dyn, C> for VecStorage<T, Dyn, C>
where
    DefaultAllocator: Allocator<Dyn, C, Buffer<T> = Self>,
{
    #[inline]
    fn into_owned(self) -> Owned<T, Dyn, C>
    where
        DefaultAllocator: Allocator<Dyn, C>,
    {
        self
    }

    #[inline]
    fn clone_owned(&self) -> Owned<T, Dyn, C>
    where
        DefaultAllocator: Allocator<Dyn, C>,
    {
        self.clone()
    }

    #[inline]
    fn forget_elements(mut self) {
        // SAFETY: setting the length to zero is always sound, as it does not
        // cause any memory to be deemed initialized. If the previous length was
        // non-zero, it is equivalent to using mem::forget to leak each element.
        // Then, when this function returns, self.data is dropped, freeing the
        // allocated memory, but the elements are not dropped because they are
        // now considered uninitialized.
        unsafe { self.data.set_len(0) };
    }
}

unsafe impl<T, R: DimName> RawStorage<T, R, Dyn> for VecStorage<T, R, Dyn> {
    type RStride = U1;
    type CStride = R;

    #[inline]
    fn ptr(&self) -> *const T {
        self.data.as_ptr()
    }

    #[inline]
    fn shape(&self) -> (R, Dyn) {
        (self.nrows, self.ncols)
    }

    #[inline]
    fn strides(&self) -> (Self::RStride, Self::CStride) {
        (Self::RStride::name(), self.nrows)
    }

    #[inline]
    fn is_contiguous(&self) -> bool {
        true
    }

    #[inline]
    unsafe fn as_slice_unchecked(&self) -> &[T] {
        &self.data
    }
}

unsafe impl<T: Scalar, R: DimName> Storage<T, R, Dyn> for VecStorage<T, R, Dyn>
where
    DefaultAllocator: Allocator<R, Dyn, Buffer<T> = Self>,
{
    #[inline]
    fn into_owned(self) -> Owned<T, R, Dyn>
    where
        DefaultAllocator: Allocator<R, Dyn>,
    {
        self
    }

    #[inline]
    fn clone_owned(&self) -> Owned<T, R, Dyn>
    where
        DefaultAllocator: Allocator<R, Dyn>,
    {
        self.clone()
    }

    #[inline]
    fn forget_elements(mut self) {
        // SAFETY: setting the length to zero is always sound, as it does not
        // cause any memory to be deemed initialized. If the previous length was
        // non-zero, it is equivalent to using mem::forget to leak each element.
        // Then, when this function returns, self.data is dropped, freeing the
        // allocated memory, but the elements are not dropped because they are
        // now considered uninitialized.
        unsafe { self.data.set_len(0) };
    }
}

/*
 *
 * RawStorageMut, ContiguousStorage.
 *
 */
unsafe impl<T, C: Dim> RawStorageMut<T, Dyn, C> for VecStorage<T, Dyn, C> {
    #[inline]
    fn ptr_mut(&mut self) -> *mut T {
        self.data.as_mut_ptr()
    }

    #[inline]
    unsafe fn as_mut_slice_unchecked(&mut self) -> &mut [T] {
        &mut self.data[..]
    }
}

unsafe impl<T, R: Dim, C: Dim> IsContiguous for VecStorage<T, R, C> {}

impl<T, C1, C2> ReshapableStorage<T, Dyn, C1, Dyn, C2> for VecStorage<T, Dyn, C1>
where
    T: Scalar,
    C1: Dim,
    C2: Dim,
{
    type Output = VecStorage<T, Dyn, C2>;

    fn reshape_generic(self, nrows: Dyn, ncols: C2) -> Self::Output {
        assert_eq!(nrows.value() * ncols.value(), self.data.len());
        VecStorage {
            data: self.data,
            nrows,
            ncols,
        }
    }
}

impl<T, C1, R2> ReshapableStorage<T, Dyn, C1, R2, Dyn> for VecStorage<T, Dyn, C1>
where
    T: Scalar,
    C1: Dim,
    R2: DimName,
{
    type Output = VecStorage<T, R2, Dyn>;

    fn reshape_generic(self, nrows: R2, ncols: Dyn) -> Self::Output {
        assert_eq!(nrows.value() * ncols.value(), self.data.len());
        VecStorage {
            data: self.data,
            nrows,
            ncols,
        }
    }
}

unsafe impl<T, R: DimName> RawStorageMut<T, R, Dyn> for VecStorage<T, R, Dyn> {
    #[inline]
    fn ptr_mut(&mut self) -> *mut T {
        self.data.as_mut_ptr()
    }

    #[inline]
    unsafe fn as_mut_slice_unchecked(&mut self) -> &mut [T] {
        &mut self.data[..]
    }
}

impl<T, R1, C2> ReshapableStorage<T, R1, Dyn, Dyn, C2> for VecStorage<T, R1, Dyn>
where
    T: Scalar,
    R1: DimName,
    C2: Dim,
{
    type Output = VecStorage<T, Dyn, C2>;

    fn reshape_generic(self, nrows: Dyn, ncols: C2) -> Self::Output {
        assert_eq!(nrows.value() * ncols.value(), self.data.len());
        VecStorage {
            data: self.data,
            nrows,
            ncols,
        }
    }
}

impl<T, R1, R2> ReshapableStorage<T, R1, Dyn, R2, Dyn> for VecStorage<T, R1, Dyn>
where
    T: Scalar,
    R1: DimName,
    R2: DimName,
{
    type Output = VecStorage<T, R2, Dyn>;

    fn reshape_generic(self, nrows: R2, ncols: Dyn) -> Self::Output {
        assert_eq!(nrows.value() * ncols.value(), self.data.len());
        VecStorage {
            data: self.data,
            nrows,
            ncols,
        }
    }
}

impl<T, R: Dim> Extend<T> for VecStorage<T, R, Dyn> {
    /// Extends the number of columns of the `VecStorage` with elements
    /// from the given iterator.
    ///
    /// # Panics
    /// This function panics if the number of elements yielded by the
    /// given iterator is not a multiple of the number of rows of the
    /// `VecStorage`.
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.data.extend(iter);
        self.ncols = Dyn(self.data.len() / self.nrows.value());
        assert!(
            self.data.len().is_multiple_of(self.nrows.value()),
            "The number of elements produced by the given iterator was not a multiple of the number of rows."
        );
    }
}

impl<'a, T: 'a + Copy, R: Dim> Extend<&'a T> for VecStorage<T, R, Dyn> {
    /// Extends the number of columns of the `VecStorage` with elements
    /// from the given iterator.
    ///
    /// # Panics
    /// This function panics if the number of elements yielded by the
    /// given iterator is not a multiple of the number of rows of the
    /// `VecStorage`.
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        self.extend(iter.into_iter().copied())
    }
}

impl<T, R, RV, SV> Extend<Vector<T, RV, SV>> for VecStorage<T, R, Dyn>
where
    T: Scalar,
    R: Dim,
    RV: Dim,
    SV: RawStorage<T, RV>,
    ShapeConstraint: SameNumberOfRows<R, RV>,
{
    /// Extends the number of columns of the `VecStorage` with vectors
    /// from the given iterator.
    ///
    /// # Panics
    /// This function panics if the number of rows of each `Vector`
    /// yielded by the iterator is not equal to the number of rows
    /// of this `VecStorage`.
    fn extend<I: IntoIterator<Item = Vector<T, RV, SV>>>(&mut self, iter: I) {
        let nrows = self.nrows.value();
        let iter = iter.into_iter();
        let (lower, _upper) = iter.size_hint();
        self.data.reserve(nrows * lower);
        for vector in iter {
            assert_eq!(nrows, vector.shape().0);
            self.data.extend(vector.iter().cloned());
        }
        self.ncols = Dyn(self.data.len() / nrows);
    }
}

impl<T> Extend<T> for VecStorage<T, Dyn, U1> {
    /// Extends the number of rows of the `VecStorage` with elements
    /// from the given iterator.
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.data.extend(iter);
        self.nrows = Dyn(self.data.len());
    }
}
