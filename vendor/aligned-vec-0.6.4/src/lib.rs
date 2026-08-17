#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! # aligned-vec
//!
//! This crate provides the `AVec<T>` and `ABox<T>` types, which are intended to have a similar API
//! to `Vec<T>` and `Box<T>`, but align the data they contain to a runtime alignment value.
//!
//! This is useful for situations where the alignment of the data matters, such as when working with
//! numerical data that can get performance benefits from being aligned to a SIMD-compatible memory address.
//!
//! # Features
//!
//! - `std` (default feature): Links this crate to the `std-crate` instead of the `core-crate`.
//! - `serde`: Implements serialization and deserialization features for `ABox` and `AVec`.

use core::{
    alloc::Layout,
    fmt::Debug,
    marker::PhantomData,
    mem::{align_of, size_of, ManuallyDrop},
    ops::{Deref, DerefMut},
    ptr::{null_mut, NonNull},
};
use equator::assert;
use raw::ARawVec;

mod raw;
extern crate alloc;

// https://rust-lang.github.io/hashbrown/src/crossbeam_utils/cache_padded.rs.html#128-130
pub const CACHELINE_ALIGN: usize = {
    #[cfg(any(
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "powerpc64",
    ))]
    {
        128
    }
    #[cfg(any(
        target_arch = "arm",
        target_arch = "mips",
        target_arch = "mips64",
        target_arch = "riscv64",
    ))]
    {
        32
    }
    #[cfg(target_arch = "s390x")]
    {
        256
    }
    #[cfg(not(any(
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "powerpc64",
        target_arch = "arm",
        target_arch = "mips",
        target_arch = "mips64",
        target_arch = "riscv64",
        target_arch = "s390x",
    )))]
    {
        64
    }
};

mod private {
    pub trait Seal {}
}

/// Trait for types that wrap an alignment value.
pub trait Alignment: Copy + private::Seal {
    /// Takes an alignment value and a minimum valid alignment,
    /// and returns an alignment wrapper that contains a power of two alignment that is greater
    /// than `minimum_align`, and if possible, greater than `align`.
    #[must_use]
    fn new(align: usize, minimum_align: usize) -> Self;
    /// Takes a minimum valid alignment,
    /// and returns an alignment wrapper that contains a power of two alignment that is greater
    /// than `minimum_align`, and if possible, greater than the contained value.
    #[must_use]
    fn alignment(self, minimum_align: usize) -> usize;
}

/// Type wrapping a runtime alignment value.
#[derive(Copy, Clone)]
pub struct RuntimeAlign {
    align: usize,
}

/// Type wrapping a compile-time alignment value.
#[derive(Copy, Clone)]
pub struct ConstAlign<const ALIGN: usize>;

impl private::Seal for RuntimeAlign {}
impl<const ALIGN: usize> private::Seal for ConstAlign<ALIGN> {}

impl<T, A: Alignment> core::convert::From<ABox<[T], A>> for AVec<T, A> {
    #[inline]
    fn from(value: ABox<[T], A>) -> Self {
        let len = (*value).len();
        let (ptr, align) = ABox::into_raw_parts(value);
        unsafe { AVec::<T, A>::from_raw_parts(ptr as *mut T, align, len, len) }
    }
}

impl Alignment for RuntimeAlign {
    #[inline]
    #[track_caller]
    fn new(align: usize, minimum_align: usize) -> Self {
        if align != 0 {
            assert!(
                align.is_power_of_two(),
                "alignment ({align}) is not a power of two.",
            );
        }
        RuntimeAlign {
            align: fix_alignment(align, minimum_align),
        }
    }

    #[inline]
    fn alignment(self, minimum_align: usize) -> usize {
        let _ = minimum_align;
        self.align
    }
}
impl<const ALIGN: usize> Alignment for ConstAlign<ALIGN> {
    #[inline]
    #[track_caller]
    fn new(align: usize, minimum_align: usize) -> Self {
        let _ = minimum_align;
        let max = Ord::max;
        if align != 0 {
            assert!(
                align.is_power_of_two(),
                "alignment ({align}) is not a power of two.",
            );
        }
        assert!(
            ALIGN.is_power_of_two(),
            "alignment ({ALIGN}) is not a power of two.",
        );
        assert!(
            align <= max(ALIGN, minimum_align),
            "provided alignment ({align}) is greater than the specified constant value ({ALIGN})",
        );
        ConstAlign::<ALIGN>
    }

    #[inline]
    fn alignment(self, minimum_align: usize) -> usize {
        fix_alignment(ALIGN, minimum_align)
    }
}

/// Aligned vector. See [`Vec`] for more info.
///
/// Note: passing an alignment value of `0` or a power of two that is less than the minimum alignment will cause the vector to use the minimum valid alignment for the type `T` and alignment type `A`.
pub struct AVec<T, A: Alignment = ConstAlign<CACHELINE_ALIGN>> {
    buf: ARawVec<T, A>,
    len: usize,
}

/// Aligned box. See [`Box`] for more info.
///
/// Note: passing an alignment value of `0` or a power of two that is less than the minimum alignment will cause the vector to use the minimum valid alignment for the type `T` and alignment type `A`.
pub struct ABox<T: ?Sized, A: Alignment = ConstAlign<CACHELINE_ALIGN>> {
    ptr: NonNull<T>,
    align: A,
    _marker: PhantomData<T>,
}

impl<T: ?Sized, A: Alignment> Deref for ABox<T, A> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr.as_ptr() }
    }
}

impl<T: ?Sized, A: Alignment> DerefMut for ABox<T, A> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.ptr.as_ptr() }
    }
}

impl<T: ?Sized, A: Alignment> AsRef<T> for ABox<T, A> {
    #[inline]
    fn as_ref(&self) -> &T {
        &**self
    }
}

impl<T: ?Sized, A: Alignment> AsMut<T> for ABox<T, A> {
    #[inline]
    fn as_mut(&mut self) -> &mut T {
        &mut **self
    }
}

struct AllocDrop {
    ptr: *mut u8,
    size_bytes: usize,
    align: usize,
}
impl Drop for AllocDrop {
    #[inline]
    fn drop(&mut self) {
        if self.size_bytes > 0 {
            unsafe {
                alloc::alloc::dealloc(
                    self.ptr,
                    alloc::alloc::Layout::from_size_align_unchecked(self.size_bytes, self.align),
                )
            }
        }
    }
}

impl<T: ?Sized, A: Alignment> Drop for ABox<T, A> {
    #[inline]
    fn drop(&mut self) {
        let size_bytes = core::mem::size_of_val(self.deref_mut());
        let align_bytes = core::mem::align_of_val(self.deref_mut());
        let ptr = self.deref_mut() as *mut T;
        let _alloc_drop = AllocDrop {
            ptr: ptr as *mut u8,
            size_bytes,
            align: self.align.alignment(align_bytes),
        };
        unsafe { ptr.drop_in_place() };
    }
}

impl<T, A: Alignment> Deref for AVec<T, A> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}
impl<T, A: Alignment> DerefMut for AVec<T, A> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}

impl<T, A: Alignment> AsRef<[T]> for AVec<T, A> {
    #[inline]
    fn as_ref(&self) -> &[T] {
        &**self
    }
}

impl<T, A: Alignment> AsMut<[T]> for AVec<T, A> {
    #[inline]
    fn as_mut(&mut self) -> &mut [T] {
        &mut **self
    }
}

impl<T, A: Alignment> ABox<T, A> {
    /// Creates a new [`ABox<T>`] containing `value` at an address aligned to `align` bytes.
    #[inline]
    #[track_caller]
    pub fn new(align: usize, value: T) -> Self {
        let align = A::new(align, align_of::<T>()).alignment(align_of::<T>());
        let ptr = if size_of::<T>() == 0 {
            null_mut::<u8>().wrapping_add(align) as *mut T
        } else {
            unsafe { raw::with_capacity_unchecked(1, align, size_of::<T>()) as *mut T }
        };
        unsafe { ptr.write(value) };
        unsafe { Self::from_raw_parts(align, ptr) }
    }

    /// Returns the alignment of the box.
    #[inline]
    pub fn alignment(&self) -> usize {
        self.align.alignment(align_of::<T>())
    }
}

impl<T: ?Sized, A: Alignment> ABox<T, A> {
    /// Creates a new [`ABox<T>`] from its raw parts.
    ///
    /// # Safety
    ///
    /// The arguments to this function must be acquired from a previous call to
    /// [`Self::into_raw_parts`].
    #[inline]
    #[track_caller]
    pub unsafe fn from_raw_parts(align: usize, ptr: *mut T) -> Self {
        Self {
            ptr: NonNull::<T>::new_unchecked(ptr),
            align: A::new(align, core::mem::align_of_val(&*ptr)),
            _marker: PhantomData,
        }
    }

    /// Decomposes a [`ABox<T>`] into its raw parts: `(ptr, alignment)`.
    #[inline]
    pub fn into_raw_parts(this: Self) -> (*mut T, usize) {
        let this = ManuallyDrop::new(this);
        let align = core::mem::align_of_val(unsafe { &*this.ptr.as_ptr() });
        (this.ptr.as_ptr(), this.align.alignment(align))
    }
}

impl<T, A: Alignment> Drop for AVec<T, A> {
    #[inline]
    fn drop(&mut self) {
        // SAFETY: dropping initialized elements
        unsafe { (self.as_mut_slice() as *mut [T]).drop_in_place() }
    }
}

#[inline]
fn fix_alignment(align: usize, base_align: usize) -> usize {
    align.max(base_align)
}

#[derive(Copy, Clone, Debug)]
pub enum TryReserveError {
    CapacityOverflow,
    AllocError { layout: Layout },
}

impl<T, A: Alignment> AVec<T, A> {
    /// Returns a new [`AVec<T>`] with the provided alignment.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn new(align: usize) -> Self {
        unsafe {
            Self {
                buf: ARawVec::new_unchecked(
                    A::new(align, align_of::<T>()).alignment(align_of::<T>()),
                ),
                len: 0,
            }
        }
    }

    /// Creates a new empty vector with enough capacity for at least `capacity` elements to
    /// be inserted in the vector. If `capacity` is 0, the vector will not allocate.
    ///
    /// # Panics
    ///
    /// Panics if the capacity exceeds `isize::MAX` bytes.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn with_capacity(align: usize, capacity: usize) -> Self {
        unsafe {
            Self {
                buf: ARawVec::with_capacity_unchecked(
                    capacity,
                    A::new(align, align_of::<T>()).alignment(align_of::<T>()),
                ),
                len: 0,
            }
        }
    }

    /// Returns a new [`AVec<T>`] from its raw parts.
    ///
    /// # Safety
    ///
    /// The arguments to this function must be acquired from a previous call to
    /// [`Self::into_raw_parts`].
    #[inline]
    #[must_use]
    pub unsafe fn from_raw_parts(ptr: *mut T, align: usize, len: usize, capacity: usize) -> Self {
        Self {
            buf: ARawVec::from_raw_parts(ptr, capacity, align),
            len,
        }
    }

    /// Decomposes an [`AVec<T>`] into its raw parts: `(ptr, alignment, length, capacity)`.
    #[inline]
    pub fn into_raw_parts(self) -> (*mut T, usize, usize, usize) {
        let mut this = ManuallyDrop::new(self);
        let len = this.len();
        let cap = this.capacity();
        let align = this.alignment();
        let ptr = this.as_mut_ptr();
        (ptr, align, len, cap)
    }

    /// Returns the length of the vector.
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the vector's length is equal to `0`, and false otherwise.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the number of elements the vector can hold without needing to reallocate.
    #[inline]
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.buf.capacity()
    }

    /// Reserves enough capacity for at least `additional` more elements to be inserted in the
    /// vector. After this call to `reserve`, capacity will be greater than or equal to `self.len() + additional`.
    /// Does nothing if the capacity is already sufficient.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds `isize::MAX` bytes.
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        if additional > self.capacity().wrapping_sub(self.len) {
            unsafe { self.buf.grow_amortized(self.len, additional) };
        }
    }

    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        if additional > self.capacity().wrapping_sub(self.len) {
            unsafe { self.buf.try_grow_amortized(self.len, additional) }
        } else {
            Ok(())
        }
    }

    /// Reserves enough capacity for exactly `additional` more elements to be inserted in the
    /// vector. After this call to `reserve`, capacity will be greater than or equal to `self.len() + additional`.
    /// Does nothing if the capacity is already sufficient.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds `isize::MAX` bytes.
    #[inline]
    pub fn reserve_exact(&mut self, additional: usize) {
        if additional > self.capacity().wrapping_sub(self.len) {
            unsafe { self.buf.grow_exact(self.len, additional) };
        }
    }

    pub fn try_reserve_exact(&mut self, additional: usize) -> Result<(), TryReserveError> {
        if additional > self.capacity().wrapping_sub(self.len) {
            unsafe { self.buf.try_grow_exact(self.len, additional) }
        } else {
            Ok(())
        }
    }

    /// Returns the alignment of the vector.
    #[inline]
    #[must_use]
    pub fn alignment(&self) -> usize {
        self.buf.align()
    }

    /// Returns a pointer to the objects held by the vector.
    #[inline]
    #[must_use]
    pub fn as_ptr(&self) -> *const T {
        self.buf.as_ptr()
    }

    /// Returns a mutable pointer to the objects held by the vector.
    #[inline]
    #[must_use]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.buf.as_mut_ptr()
    }

    /// Returns a reference to a slice over the objects held by the vector.
    #[inline]
    #[must_use]
    pub fn as_slice(&self) -> &[T] {
        let len = self.len();
        let ptr = self.as_ptr();

        // ptr points to `len` initialized elements and is properly aligned since
        // self.align is at least `align_of::<T>()`
        unsafe { core::slice::from_raw_parts(ptr, len) }
    }

    /// Returns a mutable reference to a slice over the objects held by the vector.
    #[inline]
    #[must_use]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        let len = self.len();
        let ptr = self.as_mut_ptr();

        // ptr points to `len` initialized elements and is properly aligned since
        // self.align is at least `align_of::<T>()`
        unsafe { core::slice::from_raw_parts_mut(ptr, len) }
    }

    /// Push the given value to the end of the vector, reallocating if needed.
    #[inline]
    pub fn push(&mut self, value: T) {
        if self.len == self.capacity() {
            unsafe { self.buf.grow_amortized(self.len, 1) };
        }

        // SAFETY: self.capacity is greater than self.len so the write is valid
        unsafe {
            let past_the_end = self.as_mut_ptr().add(self.len);
            past_the_end.write(value);
            self.len += 1;
        }
    }

    /// Remove the last value from the vector if it exists, otherwise returns `None`.
    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            // SAFETY: the len was greater than one so we had one valid element at the last address
            Some(unsafe { self.as_mut_ptr().add(self.len()).read() })
        }
    }

    /// Shrinks the capacity of the vector with a lower bound.  
    /// The capacity will remain at least as large as both the length and the supplied value.  
    /// If the current capacity is less than the lower limit, this is a no-op.
    #[inline]
    pub fn shrink_to(&mut self, min_capacity: usize) {
        let min_capacity = min_capacity.max(self.len());
        if self.capacity() > min_capacity {
            unsafe { self.buf.shrink_to(min_capacity) };
        }
    }

    /// Shrinks the capacity of the vector as much as possible without dropping any elements.  
    #[inline]
    pub fn shrink_to_fit(&mut self) {
        if self.capacity() > self.len {
            unsafe { self.buf.shrink_to(self.len) };
        }
    }

    /// Drops the last elements of the vector until its length is equal to `len`.  
    /// If `len` is greater than or equal to `self.len()`, this is a no-op.
    #[inline]
    pub fn truncate(&mut self, len: usize) {
        if len < self.len {
            let old_len = self.len;
            self.len = len;
            unsafe {
                let ptr = self.as_mut_ptr();
                core::ptr::slice_from_raw_parts_mut(ptr.add(len), old_len - len).drop_in_place()
            }
        }
    }

    /// Drops the all the elements of the vector, setting its length to `0`.
    #[inline]
    pub fn clear(&mut self) {
        let old_len = self.len;
        self.len = 0;
        unsafe {
            let ptr = self.as_mut_ptr();
            core::ptr::slice_from_raw_parts_mut(ptr, old_len).drop_in_place()
        }
    }

    /// Converts the vector into [`ABox<T>`].  
    /// This will drop any excess capacity.
    #[inline]
    pub fn into_boxed_slice(self) -> ABox<[T], A> {
        let mut this = self;
        this.shrink_to_fit();
        let (ptr, align, len, _) = this.into_raw_parts();
        unsafe {
            ABox::<[T], A>::from_raw_parts(align, core::ptr::slice_from_raw_parts_mut(ptr, len))
        }
    }

    /// Inserts an element at position `index` within the vector, shifting all elements after it to the right.
    ///
    /// # Panics
    ///
    /// Panics if `index > len`.
    #[track_caller]
    pub fn insert(&mut self, index: usize, element: T) {
        // Copied somewhat from the standard library
        #[cold]
        #[inline(never)]
        #[track_caller]
        fn assert_failed(index: usize, len: usize) -> ! {
            panic!("insertion index (is {index}) should be <= len (is {len})");
        }

        let len = self.len();

        // Add space for the new element
        self.reserve(1);

        unsafe {
            let p = self.as_mut_ptr().add(index);
            if index < len {
                // Shift everything over to make space. (Duplicating the
                // `index`th element into two consecutive places.)
                core::ptr::copy(p, p.add(1), len - index);
            } else if index == len {
                // No elements need shifting.
            } else {
                assert_failed(index, len);
            }
            core::ptr::write(p, element);

            self.len += 1;
        }
    }

    /// Removes and returns the element at position `index` within the vector,
    /// shifting all elements after it to the left.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds.
    #[track_caller]
    pub fn remove(&mut self, index: usize) -> T {
        // Copied somewhat from the standard library
        #[cold]
        #[inline(never)]
        #[track_caller]
        fn assert_failed(index: usize, len: usize) -> ! {
            panic!("removal index (is {index}) should be < len (is {len})");
        }

        let len = self.len();
        if index >= len {
            assert_failed(index, len);
        }

        unsafe {
            // The place we are taking from.
            let ptr = self.as_mut_ptr().add(index);
            // Copy it out, unsafely having a copy of the value on
            // the stack and in the vector at the same time.
            let ret = core::ptr::read(ptr);

            // Shift everything down to fill in that spot.
            core::ptr::copy(ptr.add(1), ptr, len - index - 1);

            self.len -= 1;

            ret
        }
    }

    /// Collects an iterator into an [`AVec<T>`] with the provided alignment.
    #[inline]
    pub fn from_iter<I: IntoIterator<Item = T>>(align: usize, iter: I) -> Self {
        Self::from_iter_impl(iter.into_iter(), align)
    }

    /// Collects a slice into an [`AVec<T>`] with the provided alignment.
    #[inline]
    pub fn from_slice(align: usize, slice: &[T]) -> Self
    where
        T: Clone,
    {
        let len = slice.len();
        let mut vec = AVec::with_capacity(align, len);
        {
            let len = &mut vec.len;
            let ptr: *mut T = vec.buf.ptr.as_ptr();

            for (i, item) in slice.iter().enumerate() {
                unsafe { ptr.add(i).write(item.clone()) };
                *len += 1;
            }
        }
        vec
    }

    fn from_iter_impl<I: Iterator<Item = T>>(mut iter: I, align: usize) -> Self {
        let (lower_bound, upper_bound) = iter.size_hint();
        let mut this = Self::with_capacity(align, lower_bound);

        if upper_bound == Some(lower_bound) {
            let len = &mut this.len;
            let ptr = this.buf.ptr.as_ptr();

            let first_chunk = iter.take(lower_bound);
            first_chunk.enumerate().for_each(|(i, item)| {
                unsafe { ptr.add(i).write(item) };
                *len += 1;
            });
        } else {
            let len = &mut this.len;
            let ptr = this.buf.ptr.as_ptr();

            let first_chunk = (&mut iter).take(lower_bound);
            first_chunk.enumerate().for_each(|(i, item)| {
                unsafe { ptr.add(i).write(item) };
                *len += 1;
            });
            iter.for_each(|item| {
                this.push(item);
            });
        }

        this
    }

    #[inline]
    pub unsafe fn set_len(&mut self, new_len: usize) {
        self.len = new_len;
    }

    pub fn append<OtherA: Alignment>(&mut self, other: &mut AVec<T, OtherA>) {
        unsafe {
            let len = self.len();
            let count = other.len();
            self.reserve(count);
            core::ptr::copy_nonoverlapping(other.as_ptr(), self.as_mut_ptr().add(len), count);
            self.len += count;
            other.len = 0;
        }
    }

    #[inline(always)]
    #[doc(hidden)]
    pub fn __from_elem(align: usize, elem: T, count: usize) -> Self
    where
        T: Clone,
    {
        Self::from_iter(align, core::iter::repeat(elem).take(count))
    }

    #[inline(always)]
    #[doc(hidden)]
    /// this is unsafe do not call this in user code
    pub fn __copy_from_ptr(align: usize, src: *const T, len: usize) -> Self {
        let mut v = Self::with_capacity(align, len);
        let dst = v.as_mut_ptr();
        unsafe { core::ptr::copy_nonoverlapping(src, dst, len) };
        v.len = len;
        v
    }
}

impl<T: Clone, A: Alignment> AVec<T, A> {
    /// Resizes the `Vec` in-place so that `len` is equal to `new_len`.
    ///
    /// If `new_len` is greater than `len`, the `Vec` is extended by the
    /// difference, with each additional slot filled with `value`.
    /// If `new_len` is less than `len`, the `Vec` is simply truncated.
    pub fn resize(&mut self, new_len: usize, value: T) {
        // Copied somewhat from the standard library
        let len = self.len();

        if new_len > len {
            self.extend_with(new_len - len, value)
        } else {
            self.truncate(new_len);
        }
    }

    /// Extend the vector by `n` clones of value.
    fn extend_with(&mut self, n: usize, value: T) {
        // Copied somewhat from the standard library
        self.reserve(n);

        unsafe {
            let mut ptr = self.as_mut_ptr().add(self.len());

            // Write all elements except the last one
            for _ in 1..n {
                core::ptr::write(ptr, value.clone());
                ptr = ptr.add(1);
                // Increment the length in every step in case clone() panics
                self.len += 1;
            }

            if n > 0 {
                // We can write the last element directly without cloning needlessly
                core::ptr::write(ptr, value);
                self.len += 1;
            }
        }
    }

    /// Clones and appends all elements in a slice to the `Vec`.
    pub fn extend_from_slice(&mut self, other: &[T]) {
        // Copied somewhat from the standard library
        let count = other.len();
        self.reserve(count);
        let len = self.len();
        unsafe {
            core::ptr::copy_nonoverlapping(other.as_ptr(), self.as_mut_ptr().add(len), count)
        };
        self.len += count;
    }
}

impl<T: Debug, A: Alignment> Debug for AVec<T, A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<T: Debug + ?Sized, A: Alignment> Debug for ABox<T, A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        (&**self).fmt(f)
    }
}

impl<T: Clone, A: Alignment> Clone for AVec<T, A> {
    fn clone(&self) -> Self {
        Self::from_slice(self.alignment(), self.deref())
    }
}

impl<T: Clone, A: Alignment> Clone for ABox<T, A> {
    fn clone(&self) -> Self {
        ABox::new(self.align.alignment(align_of::<T>()), self.deref().clone())
    }
}

impl<T: Clone, A: Alignment> Clone for ABox<[T], A> {
    fn clone(&self) -> Self {
        AVec::from_slice(self.align.alignment(align_of::<T>()), self.deref()).into_boxed_slice()
    }
}

impl<T: PartialEq, A: Alignment> PartialEq for AVec<T, A> {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice().eq(other.as_slice())
    }
}
impl<T: Eq, A: Alignment> Eq for AVec<T, A> {}
impl<T: PartialOrd, A: Alignment> PartialOrd for AVec<T, A> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.as_slice().partial_cmp(other.as_slice())
    }
}
impl<T: Ord, A: Alignment> Ord for AVec<T, A> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.as_slice().cmp(other.as_slice())
    }
}

impl<T: PartialEq + ?Sized, A: Alignment> PartialEq for ABox<T, A> {
    fn eq(&self, other: &Self) -> bool {
        (&**self).eq(&**other)
    }
}
impl<T: Eq + ?Sized, A: Alignment> Eq for ABox<T, A> {}
impl<T: PartialOrd + ?Sized, A: Alignment> PartialOrd for ABox<T, A> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        (&**self).partial_cmp(&**other)
    }
}
impl<T: Ord + ?Sized, A: Alignment> Ord for ABox<T, A> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        (&**self).cmp(&**other)
    }
}
unsafe impl<T: Sync, A: Alignment + Sync> Sync for AVec<T, A> {}
unsafe impl<T: Send, A: Alignment + Send> Send for AVec<T, A> {}
unsafe impl<T: ?Sized + Sync, A: Alignment + Sync> Sync for ABox<T, A> {}
unsafe impl<T: ?Sized + Send, A: Alignment + Send> Send for ABox<T, A> {}

#[cfg(feature = "serde")]
mod serde {
    use super::*;
    use ::serde::{Deserialize, Serialize};

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl<T: ?Sized + Serialize, A: Alignment> Serialize for ABox<T, A> {
        #[inline]
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::Serializer,
        {
            (&**self).serialize(serializer)
        }
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl<T: Serialize, A: Alignment> Serialize for AVec<T, A> {
        #[inline]
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::Serializer,
        {
            (&**self).serialize(serializer)
        }
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl<'de, T: Deserialize<'de>, const N: usize> Deserialize<'de> for ABox<T, ConstAlign<N>> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Ok(ABox::<T, ConstAlign<N>>::new(
                N,
                T::deserialize(deserializer)?,
            ))
        }
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl<'de, T: Deserialize<'de>, const N: usize> Deserialize<'de> for ABox<[T], ConstAlign<N>> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            Ok(AVec::<T, ConstAlign<N>>::deserialize(deserializer)?.into_boxed_slice())
        }
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
    impl<'de, T: Deserialize<'de>, const N: usize> Deserialize<'de> for AVec<T, ConstAlign<N>> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: ::serde::Deserializer<'de>,
        {
            struct AVecVisitor<T, const N: usize> {
                _marker: PhantomData<fn() -> AVec<T, ConstAlign<N>>>,
            }

            impl<'de, T: Deserialize<'de>, const N: usize> ::serde::de::Visitor<'de> for AVecVisitor<T, N> {
                type Value = AVec<T, ConstAlign<N>>;

                fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                    formatter.write_str("a sequence")
                }

                fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
                where
                    S: ::serde::de::SeqAccess<'de>,
                {
                    let mut vec =
                        AVec::<T, ConstAlign<N>>::with_capacity(N, cautious::<T>(seq.size_hint()));

                    while let Some(elem) = seq.next_element::<T>()? {
                        vec.push(elem)
                    }

                    Ok(vec)
                }
            }

            deserializer.deserialize_seq(AVecVisitor {
                _marker: PhantomData,
            })
        }
    }

    pub fn cautious<Element>(hint: Option<usize>) -> usize {
        use core::{cmp, mem};

        const MAX_PREALLOC_BYTES: usize = 1024 * 1024;

        if mem::size_of::<Element>() == 0 {
            0
        } else {
            cmp::min(
                hint.unwrap_or(0),
                MAX_PREALLOC_BYTES / mem::size_of::<Element>(),
            )
        }
    }
}

/// Creates a [`AVec`] containing the arguments.
///
/// `avec!` follows similar syntax to `vec!` but allows for specifying an alignment value.
/// You can either specifiy the alignment value explicitly
/// ```rust
/// use aligned_vec::{avec, CACHELINE_ALIGN};
/// let v = avec![[64]| 1, 2, 3, 4];
/// assert_eq!(v[0], 1);
/// assert_eq!(v.alignment(), 64);
/// assert_eq!(v.as_ptr().align_offset(64), 0);
/// ```
/// or dont specify it, which will use the default alignment value of `CACHELINE_ALIGN`
/// ```rust
/// use aligned_vec::{avec, CACHELINE_ALIGN};
/// let v = avec![1, 2, 3, 4];
/// assert_eq!(v[0], 1);
/// assert_eq!(v.alignment(), CACHELINE_ALIGN);
/// assert_eq!(v.as_ptr().align_offset(CACHELINE_ALIGN), 0);
/// ```
#[macro_export]
macro_rules! avec {
    () => {
        $crate::AVec::<_>::new(0)
    };
    ([$align: expr]| ) => {
        $crate::AVec::<_, $crate::ConstAlign::<$align>>::new(0)
    };
    ([$align: expr]| $elem: expr; $count: expr) => {
        $crate::AVec::<_, $crate::ConstAlign::<$align>>::__from_elem(0, $elem, $count)
    };
    ([$align: expr]| $($elem: expr),*) => {
        {
            let __data = &::core::mem::ManuallyDrop::new([$($elem,)*]);
            let __len = __data.len();
            let __ptr = __data.as_ptr();
            let mut __aligned_vec = $crate::AVec::<_, $crate::ConstAlign::<$align>>::__copy_from_ptr(0, __ptr, __len);
            __aligned_vec
        }
    };
    ($elem: expr; $count: expr) => {
        $crate::AVec::<_>::__from_elem(0, $elem, $count)
    };
    ($($elem: expr),*) => {
        {
            let __data = &::core::mem::ManuallyDrop::new([$($elem,)*]);
            let __len = __data.len();
            let __ptr = __data.as_ptr();
            let mut __aligned_vec = $crate::AVec::<_>::__copy_from_ptr(0, __ptr, __len);
            __aligned_vec
        }
    };
}

/// Create a vector that is aligned to a runtime alignment value.
#[macro_export]
macro_rules! avec_rt {
    ([$align: expr]$(|)?) => {
        $crate::AVec::<_, $crate::RuntimeAlign>::new($align)
    };
    ([$align: expr]| $elem: expr; $count: expr) => {
        $crate::AVec::<_, $crate::RuntimeAlign>::__from_elem($align, $elem, $count)
    };
    ([$align: expr]| $($elem: expr),*) => {
        {
            let __data = &::core::mem::ManuallyDrop::new([$($elem,)*]);
            let __len = __data.len();
            let __ptr = __data.as_ptr();
            let mut __aligned_vec = $crate::AVec::<_>::__copy_from_ptr($align, __ptr, __len);
            __aligned_vec
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;
    use core::iter::repeat;
    use equator::assert;

    #[test]
    fn new() {
        let v = AVec::<i32>::new(32);
        assert_eq!(v.len(), 0);
        assert_eq!(v.capacity(), 0);
        assert_eq!(v.alignment(), CACHELINE_ALIGN);
        assert_eq!(v.as_ptr().align_offset(CACHELINE_ALIGN), 0);
        let v = AVec::<()>::new(32);
        assert_eq!(v.len(), 0);
        assert_eq!(v.capacity(), usize::MAX);
        assert_eq!(v.alignment(), CACHELINE_ALIGN);
        assert_eq!(v.as_ptr().align_offset(CACHELINE_ALIGN), 0);

        #[repr(align(4096))]
        struct OverAligned;
        let v = AVec::<OverAligned>::new(32);
        assert_eq!(v.len(), 0);
        assert_eq!(v.capacity(), usize::MAX);
        assert_eq!(v.alignment(), 4096);
        assert_eq!(v.as_ptr().align_offset(CACHELINE_ALIGN), 0);
        assert_eq!(v.as_ptr().align_offset(4096), 0);
    }

    #[test]
    fn collect() {
        let v = AVec::<_>::from_iter(64, 0..4);
        assert_eq!(&*v, &[0, 1, 2, 3]);
        let v = AVec::<_>::from_iter(64, repeat(()).take(4));
        assert_eq!(&*v, &[(), (), (), ()]);
    }

    #[test]
    fn push() {
        let mut v = AVec::<i32>::new(16);
        v.push(0);
        v.push(1);
        v.push(2);
        v.push(3);
        assert_eq!(&*v, &[0, 1, 2, 3]);

        let mut v = AVec::<_>::from_iter(64, 0..4);
        v.push(4);
        v.push(5);
        v.push(6);
        v.push(7);
        assert_eq!(&*v, &[0, 1, 2, 3, 4, 5, 6, 7]);

        let mut v = AVec::<_>::from_iter(64, repeat(()).take(4));
        v.push(());
        v.push(());
        v.push(());
        v.push(());
        assert_eq!(&*v, &[(), (), (), (), (), (), (), ()]);
    }

    #[test]
    fn insert() {
        let mut v = AVec::<i32>::new(16);
        v.insert(0, 1);
        v.insert(1, 3);
        v.insert(1, 2);
        v.insert(0, 0);
        assert_eq!(&*v, &[0, 1, 2, 3]);

        let mut v = AVec::<_>::from_iter(64, 0..4);
        v.insert(0, -1);
        v.insert(5, 5);
        v.insert(5, 4);
        v.insert(1, 0);
        v.insert(2, 0);
        assert_eq!(&*v, &[-1, 0, 0, 0, 1, 2, 3, 4, 5]);

        let mut v = AVec::<_>::from_iter(64, repeat(()).take(4));
        v.insert(3, ());
        v.insert(0, ());
        v.insert(2, ());
        v.insert(7, ());
        assert_eq!(&*v, &[(), (), (), (), (), (), (), ()]);
    }

    #[test]
    fn pop() {
        let mut v = AVec::<i32>::new(16);
        v.push(0);
        v.push(1);
        v.push(2);
        v.push(3);
        assert_eq!(v.pop(), Some(3));
        assert_eq!(v.pop(), Some(2));
        assert_eq!(v.pop(), Some(1));
        assert_eq!(v.pop(), Some(0));
        assert_eq!(v.pop(), None);
        assert_eq!(v.pop(), None);
        assert_eq!(&*v, &[]);
        assert!(v.is_empty());

        let mut v = AVec::<()>::new(16);
        v.push(());
        v.push(());
        v.push(());
        v.push(());
        assert_eq!(v.pop(), Some(()));
        assert_eq!(v.pop(), Some(()));
        assert_eq!(v.pop(), Some(()));
        assert_eq!(v.pop(), Some(()));
        assert_eq!(v.pop(), None);
        assert_eq!(v.pop(), None);
        assert_eq!(&*v, &[]);
        assert!(v.is_empty());
    }

    #[test]
    fn remove() {
        let mut v = AVec::<i32>::new(16);
        v.push(0);
        v.push(1);
        v.push(2);
        v.push(3);
        assert_eq!(v.remove(2), 2);
        assert_eq!(v.remove(2), 3);
        assert_eq!(v.remove(0), 0);
        assert_eq!(v.remove(0), 1);
        assert_eq!(&*v, &[]);
        assert!(v.is_empty());

        let mut v = AVec::<()>::new(16);
        v.push(());
        v.push(());
        v.push(());
        v.push(());
        assert_eq!(v.remove(0), ());
        assert_eq!(v.remove(0), ());
        assert_eq!(v.remove(0), ());
        assert_eq!(v.remove(0), ());
        assert_eq!(&*v, &[]);
        assert!(v.is_empty());
    }

    #[test]
    fn shrink() {
        let mut v = AVec::<i32>::with_capacity(16, 10);
        v.push(0);
        v.push(1);
        v.push(2);

        assert_eq!(v.capacity(), 10);
        v.shrink_to_fit();
        assert_eq!(v.len(), 3);
        assert_eq!(v.capacity(), 3);

        let mut v = AVec::<i32>::with_capacity(16, 10);
        v.push(0);
        v.push(1);
        v.push(2);

        assert_eq!(v.capacity(), 10);
        v.shrink_to(0);
        assert_eq!(v.len(), 3);
        assert_eq!(v.capacity(), 3);
    }

    #[test]
    fn truncate() {
        let mut v = AVec::<i32>::new(16);
        v.push(0);
        v.push(1);
        v.push(2);

        v.truncate(1);
        assert_eq!(v.len(), 1);
        assert_eq!(&*v, &[0]);

        v.clear();
        assert_eq!(v.len(), 0);
        assert_eq!(&*v, &[]);

        let mut v = AVec::<()>::new(16);
        v.push(());
        v.push(());
        v.push(());

        v.truncate(1);
        assert_eq!(v.len(), 1);
        assert_eq!(&*v, &[()]);

        v.clear();
        assert_eq!(v.len(), 0);
        assert_eq!(&*v, &[]);
    }

    #[test]
    fn extend_from_slice() {
        let mut v = AVec::<i32>::new(16);
        v.extend_from_slice(&[0, 1, 2, 3]);
        v.extend_from_slice(&[4, 5, 6, 7, 8]);
        assert_eq!(&*v, &[0, 1, 2, 3, 4, 5, 6, 7, 8]);

        let mut v = AVec::<()>::new(16);
        v.extend_from_slice(&[(), (), (), ()]);
        v.extend_from_slice(&[(), (), ()]);
        assert_eq!(&*v, &[(), (), (), (), (), (), ()]);
    }

    #[test]
    fn resize() {
        let mut v = AVec::<i32>::new(16);
        v.push(0);
        v.push(1);
        v.push(2);

        v.resize(1, 10);
        assert_eq!(v.len(), 1);
        assert_eq!(&*v, &[0]);

        v.resize(3, 20);
        assert_eq!(v.len(), 3);
        assert_eq!(&*v, &[0, 20, 20]);

        let mut v = AVec::<()>::new(16);
        v.push(());
        v.push(());
        v.push(());

        v.resize(2, ());
        assert_eq!(v.len(), 2);
        assert_eq!(&*v, &[(), ()]);

        v.resize(3, ());
        assert_eq!(v.len(), 3);
        assert_eq!(&*v, &[(), (), ()]);
    }

    #[test]
    fn into_boxed_slice() {
        let mut v = AVec::<i32>::new(16);
        v.push(0);
        v.push(1);
        v.push(2);

        let boxed = v.into_boxed_slice();
        assert_eq!(&*boxed, &[0, 1, 2]);
    }

    #[test]
    fn box_new() {
        let boxed = ABox::<_>::new(64, 3);
        assert_eq!(&*boxed, &3);
    }

    #[test]
    fn box_clone() {
        let boxed = ABox::<_>::new(64, 3);
        assert_eq!(boxed, boxed.clone());
    }

    #[test]
    fn box_slice_clone() {
        let boxed = AVec::<_>::from_iter(64, 0..123).into_boxed_slice();
        assert_eq!(boxed, boxed.clone());
    }

    #[test]
    fn macros() {
        let u: AVec<()> = avec![];
        assert_eq!(u.len(), 0);
        assert_eq!(u.as_ptr().align_offset(CACHELINE_ALIGN), 0);

        let v = avec![0; 4];
        assert_eq!(v.len(), 4);
        assert_eq!(v.as_ptr().align_offset(CACHELINE_ALIGN), 0);

        let mut w = avec![vec![0, 1], vec![3, 4], vec![5, 6], vec![7, 8]];
        w[0].push(2);
        w[3].pop();
        assert_eq!(w.len(), 4);
        assert_eq!(w.as_ptr().align_offset(CACHELINE_ALIGN), 0);
        assert_eq!(w[0], vec![0, 1, 2]);
        assert_eq!(w[1], vec![3, 4]);
        assert_eq!(w[2], vec![5, 6]);
        assert_eq!(w[3], vec![7]);
    }

    #[test]
    fn macros_2() {
        let u: AVec<(), _> = avec![[4096]| ];
        assert_eq!(u.len(), 0);
        assert_eq!(u.as_ptr().align_offset(4096), 0);

        let v = avec![[4096]| 0; 4];
        assert_eq!(v.len(), 4);
        assert_eq!(v.as_ptr().align_offset(4096), 0);

        let mut w = avec![[4096] | vec![0, 1], vec![3, 4], vec![5, 6], vec![7, 8]];
        w[0].push(2);
        w[3].pop();
        assert_eq!(w.len(), 4);
        assert_eq!(w.as_ptr().align_offset(4096), 0);
        assert_eq!(w[0], vec![0, 1, 2]);
        assert_eq!(w[1], vec![3, 4]);
        assert_eq!(w[2], vec![5, 6]);
        assert_eq!(w[3], vec![7]);
    }

    #[test]
    fn macros_rt() {
        let u: AVec<(), _> = avec_rt![[32]];
        assert_eq!(u.len(), 0);
        assert_eq!(u.as_ptr().align_offset(32), 0);

        let v = avec_rt![[32]| 0; 4];
        assert_eq!(v.len(), 4);
        assert_eq!(v.as_ptr().align_offset(32), 0);

        let mut w = avec_rt![[64] | vec![0, 1], vec![3, 4], vec![5, 6], vec![7, 8]];
        w[0].push(2);
        w[3].pop();
        assert_eq!(w.len(), 4);
        assert_eq!(w.as_ptr().align_offset(64), 0);
        assert_eq!(w[0], vec![0, 1, 2]);
        assert_eq!(w[1], vec![3, 4]);
        assert_eq!(w[2], vec![5, 6]);
        assert_eq!(w[3], vec![7]);
    }
}

#[cfg(all(test, feature = "serde"))]
mod serde_tests {
    use super::*;

    use ::serde::Deserialize;
    use bincode::{DefaultOptions, Deserializer, Options};

    #[test]
    fn can_limit_deserialization_size() {
        // Malformed serialized data indicating a sequence of length u64::MAX.
        let ser = vec![
            253, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 1, 1, 1, 1, 1, 1, 1, 1,
        ];

        let options = DefaultOptions::new().with_limit(12);

        let mut deserializer = Deserializer::from_slice(&ser, options);
        let result = <AVec<u32> as Deserialize>::deserialize(&mut deserializer);

        let err = match result {
            Ok(_) => panic!("Expected a failure"),
            Err(e) => e,
        };

        match *err {
            bincode::ErrorKind::SizeLimit => {}
            _ => panic!("Expected ErrorKind::SizeLimit, got {err:#?}"),
        };
    }
}
