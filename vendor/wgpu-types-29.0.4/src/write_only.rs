#![deny(
    elided_lifetimes_in_paths,
    reason = "make all lifetime relationships around our unsafe code explicit, \
             because they are important to soundness"
)]

//! The [`WriteOnly`] type.
//!
//! This type gets its own module in order to provide an encapsulation boundary around the
//! substantial `unsafe` code required to implement [`WriteOnly`].
//!
//! Portions of this code and documentation have been copied from the Rust standard library.

use core::{
    any::TypeId,
    fmt,
    marker::PhantomData,
    mem,
    ops::{Bound, RangeBounds},
    ptr::NonNull,
};

use crate::link_to_wgpu_item;

/// Like `&'a mut T`, but allows only write operations.
///
/// This pointer type is obtained from [`BufferViewMut`] and
/// [`QueueWriteBufferView`].
/// It is an unfortunate necessity due to the fact that mapped GPU memory may be [write combining],
/// which means it cannot work normally with all of the things that Rust `&mut` access allows you to
/// do.
///
/// ([`WriteOnly`] can also be used as an interface to write to *uninitialized* memory, but this is
/// not a feature which `wgpu` currently offers for GPU buffers.)
///
/// The methods of `WriteOnly<[T]>` are similar to those available for
/// [slice references, `&mut [T]`][primitive@slice],
/// with some changes to ownership intended to minimize the pain of explicit reborrowing.
///
// FIXME: Add an introduction to the necessity of explicit reborrowing.
///
/// [write combining]: https://en.wikipedia.org/wiki/Write_combining
#[doc = link_to_wgpu_item!(struct BufferViewMut)]
#[doc = link_to_wgpu_item!(struct QueueWriteBufferView)]
pub struct WriteOnly<'a, T: ?Sized> {
    /// The data which this write-only reference allows **writing** to.
    ///
    /// This field is not `&mut T`, because if it were, it would assert to the compiler
    /// that spurious reads may be inserted, and is is unclear whether those spurious reads
    /// are acceptable.
    ptr: NonNull<T>,

    /// Enforces that this type
    ///
    /// * is only valid for `'a`
    /// * is invariant in `T`
    /// * implements auto traits as a reference to `T`
    ///
    /// In theory, [`WriteOnly`] should be *contravariant* in `T`, but this would be tricky
    /// to implement (`ptr` would need to be type-erased) and is very unlikely to be useful.
    _phantom: PhantomData<&'a mut T>,
}

// SAFETY:
// `WriteOnly<T>` is like `&mut T` in that
// * It provides only exclusive access to the memory it points to, so `T: Sync` is not required.
// * Sending it creates the opportunity to send a `T`, so `T: Send` is required.
unsafe impl<T: Send> Send for WriteOnly<'_, T> {}

// SAFETY:
// `WriteOnly<T>` does not ever expose any `&T`, and therefore may unconditionally implement `Sync`.
unsafe impl<T: ?Sized> Sync for WriteOnly<'_, T> {}

impl<'a, T: ?Sized> WriteOnly<'a, T> {
    // Note: Every method is marked `#[inline]` because the premise of this API design is that
    // `WriteOnly` should be, when compiled, as cheap as manipulating `&mut` rather than
    // having any additional function call cost.

    /// Constructs a [`WriteOnly`] pointer from a raw pointer.
    ///
    /// # Safety
    ///
    /// By calling [`WriteOnly::new()`], you are giving safe code the opportunity to write to
    /// this memory if it is given the resulting [`WriteOnly`]. Therefore:
    ///
    /// * `ptr` must be valid for ordinary, non-`volatile`, writes.
    ///   (It need not be valid for reads, including reads that occur as part of atomic operations
    ///   — that’s the whole point.)
    /// * `ptr` must be aligned to at least the alignment of the type `T`.
    /// * No other accesses to the memory pointed to by `ptr` may be performed until the
    ///   lifetime `'a` ends. (Similar to
    ///   [the conditions to construct `&'a mut T`][std::ptr#pointer-to-reference-conversion].)
    ///
    /// The memory pointed to need not contain a valid `T`, but if it does, it still will after
    /// the `WriteOnly` pointer is used; that is, safe (or sound unsafe) use of `WriteOnly` will not
    /// “de-initialize” the memory.
    #[inline]
    #[must_use]
    pub unsafe fn new(ptr: NonNull<T>) -> Self {
        Self {
            ptr,
            _phantom: PhantomData,
        }
    }

    /// Constructs a [`WriteOnly`] pointer from an ordinary read-write `&mut` reference.
    ///
    /// This may be used to write code which can write either to a mapped GPU buffer or
    /// normal memory.
    ///
    /// # Example
    ///
    /// ```
    /// # use wgpu_types as wgpu;
    /// fn write_numbers(slice: wgpu::WriteOnly<[u32]>) {
    ///     for (i, mut elem) in slice.into_iter().enumerate() {
    ///         elem.write(i as u32);
    ///     }
    /// }
    ///
    /// let mut buf: [u32; 4] = [0; 4];
    /// write_numbers(wgpu::WriteOnly::from_mut(&mut buf));
    /// assert_eq!(buf, [0, 1, 2, 3]);
    /// ```
    #[inline]
    #[must_use]
    pub fn from_mut(reference: &mut T) -> Self {
        // SAFETY: `&mut`’s safety conditions imply ours.
        // FIXME: Use `NonNull::from_mut()` when MSRV ≥ 1.89.0
        unsafe { Self::new(NonNull::new_unchecked(&raw mut *reference)) }
    }

    /// Writes `value` into the memory pointed to by `self`.
    ///
    /// This can only be used when `T` is a [`Sized`] type.
    /// For slices, use [`copy_from_slice()`][Self::copy_from_slice] or
    /// [`write_iter()`][Self::write_iter] instead.
    #[inline]
    pub fn write(self, value: T)
    where
        // Ideally, we want "does not have a destructor" to avoid any need for dropping (which
        // would imply reading) or forgetting the values that write operations overwrite.
        // However, there is no such trait bound and `T: Copy` is the closest approximation.
        T: Copy,
    {
        // SAFETY:
        // `self.ptr` is valid for writes, and `self`’s lifetime ensures the write cannot alias.
        //
        // Not forgetting values:
        // `T` is `Copy`, so overwriting the old value of `*self.ptr` is trivial and does not
        // forget anything.
        unsafe { self.ptr.write(value) }
    }

    /// Returns a raw pointer to the memory this [`WriteOnly`] refers to.
    ///
    /// This operation may be used to manually perform writes in situations where the safe API of
    /// [`WriteOnly`] is not sufficient, e.g. for random access from multiple threads.
    ///
    /// You must take care when using this pointer:
    ///
    /// * The `WriteOnly` type makes no guarantee that the memory pointed to by this pointer is
    ///   readable or initialized. Therefore, it must not be converted to `&mut T`, nor read any
    ///   other way.
    /// * You may not write an invalid value unless you also overwrite it with a valid value
    ///   later. That is, you may not make the memory less initialized than it already was.
    ///
    /// See also [`as_raw_element_ptr()`][WriteOnly::as_raw_element_ptr], which returns a pointer
    /// to the first element of a slice.
    ///
    /// [write combining]: https://en.wikipedia.org/wiki/Write_combining
    #[inline]
    pub fn as_raw_ptr(&mut self) -> NonNull<T> {
        self.ptr
    }
}

/// Methods for write-only references to slices.
impl<'a, T> WriteOnly<'a, [T]> {
    /// Returns the length of the referenced slice; the number of elements that may be written.
    ///
    /// # Example
    ///
    /// ```
    /// # use wgpu_types as wgpu;
    /// let example_slice: &mut [u8] = &mut [0; 10];
    /// assert_eq!(wgpu::WriteOnly::from_mut(example_slice).len(), example_slice.len());
    /// ```
    #[inline]
    #[must_use]
    pub const fn len(&self) -> usize {
        self.ptr.len()
    }

    /// Returns `true` if the referenced slice has a length of 0.
    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns another slice reference borrowing from this one,
    /// covering a sub-range and with a shorter lifetime.
    ///
    /// You can also use `.slice(..)` to perform an explicit reborrow without shrinking.
    ///
    /// See also [`into_slice()`][Self::into_slice] when the same lifetime is needed.
    ///
    /// # Example
    ///
    /// ```
    /// # use wgpu_types as wgpu;
    /// // Ordinarily you would get a `WriteOnly` from `wgpu::Buffer` instead.
    /// let mut data: [u8; 9] = [0; 9];
    /// let mut wo = wgpu::WriteOnly::from_mut(data.as_mut_slice());
    ///
    /// wo.slice(..3).copy_from_slice(&[1, 2, 3]);
    /// wo.slice(3..6).copy_from_slice(&[4, 5, 6]);
    /// wo.slice(6..).copy_from_slice(&[7, 8, 9]);
    ///
    /// assert_eq!(data, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
    /// ```
    #[inline]
    #[must_use]
    pub fn slice<'b, S: RangeBounds<usize>>(&'b mut self, bounds: S) -> WriteOnly<'b, [T]> {
        // SAFETY: We are duplicating `self.ptr`, but the lifetime annotations on this function
        // ensure exclusive access.
        let reborrow = unsafe { WriteOnly::<'b, [T]>::new(self.ptr) };

        reborrow.into_slice(bounds)
    }

    /// Shrinks this slice reference in the same way as [`slice()`](Self::slice), but
    /// consumes `self` and returns a slice reference with the same lifetime,
    /// instead of a shorter lifetime.
    #[inline]
    #[must_use]
    pub fn into_slice<S: RangeBounds<usize>>(mut self, bounds: S) -> Self {
        let (checked_start, checked_new_len) =
            checked_range_to_start_len(self.len(), bounds.start_bound(), bounds.end_bound());

        WriteOnly {
            // FIXME: When `feature(slice_ptr_get)` <https://github.com/rust-lang/rust/issues/74265>
            // is stable, replace this with `NonNull::get_unchecked_mut()`.
            // Unfortunately, we’ll still need to do explicit destructuring of `bounds`
            // for bounds checking.
            ptr: NonNull::slice_from_raw_parts(
                // SAFETY of add(): we already did a bounds check.
                unsafe { self.as_raw_element_ptr().add(checked_start) },
                checked_new_len,
            ),
            _phantom: PhantomData,
        }
    }

    /// Writes the items of `iter` into `self`.
    ///
    /// The iterator must produce exactly `self.len()` items.
    ///
    /// If the items are in a slice, use [`copy_from_slice()`][Self::copy_from_slice] instead.
    ///
    /// # Panics
    ///
    /// Panics if `iter` produces more or fewer items than `self.len()`.
    ///
    /// # Example
    ///
    /// ```
    /// # use wgpu_types as wgpu;
    /// // Ordinarily you would get a `WriteOnly` from `wgpu::Buffer` instead.
    /// let mut buf: [u8; 10] = [0; 10];
    /// let wo = wgpu::WriteOnly::from_mut(buf.as_mut_slice());
    ///
    /// wo.write_iter((1..).take(10));
    ///
    /// assert_eq!(buf, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    /// ```
    #[inline]
    #[track_caller]
    pub fn write_iter<I>(self, iter: I)
    where
        T: Copy, // required by write()
        I: IntoIterator<Item = T>,
    {
        let self_len = self.len();
        let mut slot_iter = self.into_iter();

        // Call `for_each()` to take advantage of the iterator’s custom implementation, if it has
        // one. This may be superior to a `for` loop for `chain()`ed iterators and other cases where
        // the implementation of `Iterator::next()` would need to branch, and is typically
        // equivalent to a `for` loop for other iterators.
        iter.into_iter().for_each(|item| {
            let Some(slot) = slot_iter.next() else {
                panic!("iterator given to write_iter() produced more than {self_len} elements");
            };

            slot.write(item);
        });

        let remaining_len = slot_iter.len();
        if remaining_len != 0 {
            panic!(
                "iterator given to write_iter() produced {iter_len} elements \
                    but must produce {self_len} elements",
                // infer how many elements the iterator produced by how many of ours were consumed
                iter_len = self_len - remaining_len,
            );
        };
    }

    /// Writes copies of `value` to every element of `self`.
    ///
    /// # Example
    ///
    /// ```
    /// # use wgpu_types as wgpu;
    /// // Ordinarily you would get a `WriteOnly` from `wgpu::Buffer` instead.
    /// let mut buf = vec![0; 10];
    /// let mut wo = wgpu::WriteOnly::from_mut(buf.as_mut_slice());
    ///
    /// wo.fill(1);
    ///
    /// assert_eq!(buf, [1; 10]);
    /// ```
    #[inline]
    pub fn fill(&mut self, value: T)
    where
        // Ideally, we want "does not have a destructor" to avoid any need for dropping (which
        // would imply reading) or forgetting the values that write operations overwrite.
        // However, there is no such trait bound and `T: Copy` is the closest approximation.
        T: Copy + 'static,
    {
        let ty = TypeId::of::<T>();
        if ty == TypeId::of::<u8>() || ty == TypeId::of::<i8>() || ty == TypeId::of::<bool>() {
            // The type consists of a single _initialized_ byte, so we can call out to
            // `write_bytes()` (a.k.a. `memset` in C).
            //
            // Note that we cannot just check that the size is 1, because some types may allow
            // uninitialized bytes (trivially, `MaybeUninit<u8>`)

            // SAFETY:
            // * We just checked that `T` can soundly be transmuted to `u8`.
            // * `T` is `Copy` so we don’t need to worry about duplicating it with `transmute_copy`.
            // * `write_bytes()` is given a pointer which is guaranteed by our own invariants
            //   to be valid to write to.
            unsafe {
                let value_as_byte = mem::transmute_copy::<T, u8>(&value);
                self.as_raw_element_ptr()
                    .cast::<u8>()
                    .write_bytes(value_as_byte, self.len());
            }
        } else {
            // Generic loop for all other types.
            self.slice(..)
                .into_iter()
                .for_each(|elem| elem.write(value));
        }
    }

    /// Copies all elements from src into `self`.
    ///
    /// # Panics
    ///
    /// Panics if the length of `src` is not the same as `self`.
    ///
    /// # Example
    ///
    /// ```
    /// # use wgpu_types as wgpu;
    /// // Ordinarily you would get a `WriteOnly` from `wgpu::Buffer` instead.
    /// let mut buf = vec![0; 5];
    /// let mut wo = wgpu::WriteOnly::from_mut(buf.as_mut_slice());
    ///
    /// wo.copy_from_slice(&[2, 3, 5, 7, 11]);
    ///
    /// assert_eq!(*buf, [2, 3, 5, 7, 11]);
    #[inline]
    #[track_caller]
    pub fn copy_from_slice(&mut self, src: &[T])
    where
        // Ideally, we want "does not have a destructor" to avoid any need for dropping (which
        // would imply reading) or forgetting the values that write operations overwrite.
        // However, there is no such trait bound and `T: Copy` is the closest approximation.
        T: Copy,
    {
        let src_len = src.len();
        let dst_len = self.len();
        if src_len != dst_len {
            // wording chosen to match <[_]>::copy_from_slice()'s message
            panic!(
                "source slice length ({src_len}) does not match \
                    destination slice length ({dst_len})"
            );
        }

        let src_ptr: *const T = src.as_ptr();
        let dst_ptr: *mut T = self.as_raw_element_ptr().as_ptr();

        // SAFETY:
        // * `src_ptr` is readable because it was constructed from a reference.
        // * `dst_ptr` is writable because that is an invariant of `WriteOnly`.
        // * `dst_ptr` cannot alias `src_ptr` because `self` is exclusive *and*
        //   because `src_ptr` is immutable.
        // * We checked that the byte lengths match.
        // * Lack of data races will be enforced by the type
        unsafe { dst_ptr.copy_from_nonoverlapping(src_ptr, src.len()) }
    }

    /// Splits this slice reference into `N`-element arrays, starting at the beginning of the slice,
    /// and a reference to the remainder with length strictly less than `N`.
    ///
    /// This method is analogous to [`<[T]>::as_chunks_mut()`][slice::as_chunks_mut]
    /// but for `WriteOnly<[T]>` access.
    /// (It takes ownership instead of `&mut self` in order to avoid reborrowing issues.
    /// Use [`.slice(..)`][Self::slice] first if reborrowing is needed.)
    ///
    /// # Panics
    ///
    /// Panics if `N` is zero.
    ///
    /// # Example
    ///
    /// `into_chunks()` is useful for writing a sequence of elements from CPU memory to GPU memory
    /// when a transformation is required.
    /// (If a transformation is not required, use [`WriteOnly::copy_from_slice()`].)
    ///
    /// ```
    /// # use wgpu_types as wgpu;
    /// fn write_text_as_chars(text: &str, output: wgpu::WriteOnly<[u8]>) {
    ///     let (mut output, _remainder) = output.into_chunks::<{ size_of::<u32>() }>();
    ///     output.write_iter(text.chars().map(|ch| (ch as u32).to_ne_bytes()));
    /// }
    /// #
    /// # let mut buf = [255; 8];
    /// # write_text_as_chars("hi", wgpu::WriteOnly::from_mut(buf.as_mut_slice()));
    /// # assert_eq!(
    /// #     buf,
    /// #     [
    /// #          u32::from(b'h').to_ne_bytes(),
    /// #          u32::from(b'i').to_ne_bytes(),
    /// #     ].as_flattened(),
    /// # );
    /// ```
    #[inline]
    #[must_use]
    pub fn into_chunks<const N: usize>(self) -> (WriteOnly<'a, [[T; N]]>, WriteOnly<'a, [T]>) {
        // This implementation is identical to the Rust standard library implementation as of
        // Rust 1.93.0, except for being broken down into fewer pieces and less uncheckedness.

        assert!(N != 0, "chunk size must be non-zero");
        let len_in_chunks = self.len() / N;
        let len_in_elements_rounded_down = len_in_chunks * N;
        let (multiple_of_n, remainder) = self.split_at(len_in_elements_rounded_down);
        // SAFETY: We already panicked for zero, and ensured by construction
        // that the length of the subslice is a multiple of N.
        let array_slice = unsafe {
            WriteOnly::new(NonNull::slice_from_raw_parts(
                multiple_of_n.ptr.cast::<[T; N]>(),
                len_in_chunks,
            ))
        };
        (array_slice, remainder)
    }

    /// Divides one write-only slice reference into two at an index.
    ///
    /// The first will contain all indices from `[0, mid)` (excluding
    /// the index `mid` itself) and the second will contain all
    /// indices from `[mid, len)` (excluding the index `len` itself).
    ///
    /// # Panics
    ///
    /// Panics if `mid > len`.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn split_at(self, mid: usize) -> (WriteOnly<'a, [T]>, WriteOnly<'a, [T]>) {
        match self.split_at_checked(mid) {
            Ok(slices) => slices,
            Err(_) => panic!("mid > len"),
        }
    }

    /// Divides one write-only slice reference into two at an index, returning [`Err`] if the
    /// slice is too short.
    ///
    /// If `mid ≤ len`, returns a pair of slices where the first will contain all
    /// indices from `[0, mid)` (excluding the index `mid` itself) and the
    /// second will contain all indices from `[mid, len)` (excluding the index
    /// `len` itself).
    ///
    /// Otherwise, if `mid > len`, returns [`Err`] with the original slice.
    #[inline]
    pub fn split_at_checked(self, mid: usize) -> Result<(Self, Self), Self> {
        if mid <= self.len() {
            let Self { ptr, _phantom: _ } = self;
            let element_ptr = ptr.cast::<T>();
            Ok(unsafe {
                (
                    Self::new(NonNull::slice_from_raw_parts(element_ptr, mid)),
                    Self::new(NonNull::slice_from_raw_parts(
                        element_ptr.add(mid),
                        ptr.len() - mid,
                    )),
                )
            })
        } else {
            Err(self)
        }
    }

    /// Removes the subslice corresponding to the given range and returns a mutable reference to it.
    ///
    /// Returns [`None`] and does not modify the slice if the given range is out of bounds.
    ///
    /// # Panics
    ///
    /// Panics if `R` is not a one-sided range such as `..n` or `n..`.
    // (The `OneSidedRange` trait `std` uses to statically enforce this is unstable.)
    pub fn split_off<R>(&mut self, range: R) -> Option<Self>
    where
        R: RangeBounds<usize>,
    {
        match (range.start_bound(), range.end_bound()) {
            (Bound::Included(&mid), Bound::Unbounded) => {
                match mem::take(self).split_at_checked(mid) {
                    Ok((front, back)) => {
                        *self = front;
                        Some(back)
                    }
                    Err(short) => {
                        *self = short;
                        None
                    }
                }
            }
            (Bound::Excluded(&before_mid), Bound::Unbounded) => {
                let mid = before_mid.checked_add(1)?;
                match mem::take(self).split_at_checked(mid) {
                    Ok((front, back)) => {
                        *self = front;
                        Some(back)
                    }
                    Err(short) => {
                        *self = short;
                        None
                    }
                }
            }
            (Bound::Unbounded, Bound::Included(&before_mid)) => {
                let mid = before_mid.checked_add(1)?;
                match mem::take(self).split_at_checked(mid) {
                    Ok((front, back)) => {
                        *self = back;
                        Some(front)
                    }
                    Err(short) => {
                        *self = short;
                        None
                    }
                }
            }
            (Bound::Unbounded, Bound::Excluded(&mid)) => {
                match mem::take(self).split_at_checked(mid) {
                    Ok((front, back)) => {
                        *self = back;
                        Some(front)
                    }
                    Err(short) => {
                        *self = short;
                        None
                    }
                }
            }
            _ => {
                panic!("split_off() requires a one-sided range")
            }
        }
    }

    /// Shrinks `self` to no longer refer to its first element, and returns a reference to that
    /// element.
    ///
    /// Returns `None` if `self` is empty.
    #[inline]
    #[must_use]
    pub fn split_off_first(&mut self) -> Option<WriteOnly<'a, T>> {
        let len = self.len();
        if let Some(new_len) = len.checked_sub(1) {
            let ptr: NonNull<T> = self.as_raw_element_ptr();

            // SAFETY: covers exactly everything but the first element
            *self = unsafe { WriteOnly::new(NonNull::slice_from_raw_parts(ptr.add(1), new_len)) };

            // SAFETY: self was not empty so ptr is not dangling, and we will avoid aliasing
            Some(unsafe { WriteOnly::new(ptr) })
        } else {
            None
        }
    }

    /// Shrinks `self` to no longer refer to its last element, and returns a reference to that
    /// element.
    ///
    /// Returns `None` if `self` is empty.
    #[inline]
    #[must_use]
    pub fn split_off_last(&mut self) -> Option<WriteOnly<'a, T>> {
        let len = self.len();
        if let Some(new_len) = len.checked_sub(1) {
            let ptr: NonNull<T> = self.as_raw_element_ptr();

            // SAFETY: covers exactly everything but the first element
            *self = unsafe { WriteOnly::new(NonNull::slice_from_raw_parts(ptr, new_len)) };

            // SAFETY: self was not empty so ptr is not dangling, and we will avoid aliasing
            Some(unsafe { WriteOnly::new(ptr.add(new_len)) })
        } else {
            None
        }
    }

    /// Reinterprets a reference to `[T]` as a reference to `[U]`.
    ///
    /// This may be used, for example, to copy a slice of `struct`s into a `[u8]` buffer.
    ///
    /// This method is `unsafe`, can easily be used incorrectly, and its use is often not necessary;
    /// consider converting your data to bytes explicitly instead.
    /// Consider using [`.into_chunks()`][Self::into_chunks] instead if possible.
    /// When this method is used, consider wrapping it in a function that provides a narrower
    /// type signature that can be safe.
    ///
    /// # Safety
    ///
    /// All values of type `U` must also be valid values of type `T`.
    ///
    /// Note that this is a requirement which is significant even if `T = [u8; N]`.
    /// For example, if `T` contains any padding (uninitialized) bytes, then it is not valid to
    /// interpret those bytes as `u8`s, and such a cast is unsound.
    ///
    /// A way to ensure soundness of this operation is to ensure that `T` and `U` satisfy traits
    /// from a helper library, such as `T: bytemuck::AnyBitPattern, U: bytemuck::NoUninit`.
    ///
    /// # Panics
    ///
    /// Panics if the size of type `U` does not equal the size of type `T`,
    /// or if the alignment of type `U` is greater than the alignment of type `T`.
    ///
    /// This panic occurs regardless of the run-time length or alignment of the slice;
    /// any call to `cast_elements()` with a particular type `T` and typ` U` will
    /// either always succeed or always fail.
    #[inline]
    #[track_caller]
    pub unsafe fn cast_elements<U>(self) -> WriteOnly<'a, [U]> {
        assert_eq!(
            size_of::<T>(),
            size_of::<U>(),
            "sizes of the two element types must be equal"
        );
        assert!(
            align_of::<U>() <= align_of::<T>(),
            "alignment of the new element type must be \
            less than or equal to the alignment of the old element type"
        );
        unsafe {
            WriteOnly::new(NonNull::slice_from_raw_parts(
                self.ptr.cast::<U>(),
                self.len(),
            ))
        }
    }

    /// Returns a raw pointer to the first element of this [`WriteOnly`] slice reference.
    ///
    /// See [`WriteOnly::as_raw_ptr()`] for information on how this pointer is, or is not,
    /// sound to use.
    #[inline]
    pub fn as_raw_element_ptr(&mut self) -> NonNull<T> {
        self.ptr.cast::<T>()
    }
}

// This impl does not have `T: ?Sized` so we can have a separate impl for slices
impl<T> fmt::Debug for WriteOnly<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "WriteOnly({ty})", ty = core::any::type_name::<T>())
    }
}
impl<T> fmt::Debug for WriteOnly<'_, [T]> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // We don't format this as `[{ty}; {len}]` in order to not mislead readers into
        // thinking the type is an array type.
        write!(
            f,
            "WriteOnly([{ty}], len = {len})",
            ty = core::any::type_name::<T>(),
            len = self.len(),
        )
    }
}

impl<'a, T> Default for WriteOnly<'a, [T]> {
    /// Returns an empty slice reference, just like `<&mut [T]>::default()` would.
    ///
    /// This may be used as a placeholder value for operations like
    /// [`mem::take()`][core::mem::take].
    /// It is equivalent to `WriteOnly::from_mut(&mut [])`.
    fn default() -> Self {
        Self::from_mut(&mut [])
    }
}

impl<'a, T> Default for WriteOnly<'a, [T; 0]> {
    fn default() -> Self {
        Self::from_mut(&mut [])
    }
}

impl<'a, 'b: 'a, T: ?Sized> From<&'b mut T> for WriteOnly<'a, T> {
    /// Equivalent to [`WriteOnly::from_mut()`].
    fn from(reference: &'a mut T) -> WriteOnly<'a, T> {
        Self::from_mut(reference)
    }
}

// Ideally we'd also implement CoerceUnsized for this same conversion, but that’s unstable.
// <https://doc.rust-lang.org/std/ops/trait.CoerceUnsized.html>
impl<'a, 'b: 'a, T, const N: usize> From<WriteOnly<'b, [T; N]>> for WriteOnly<'a, [T]> {
    fn from(array_wo: WriteOnly<'b, [T; N]>) -> WriteOnly<'a, [T]> {
        WriteOnly {
            _phantom: PhantomData,
            ptr: array_wo.ptr, // implicit unsizing coercion of the pointer value
        }
    }
}

impl<'a, T> IntoIterator for WriteOnly<'a, [T]> {
    type Item = WriteOnly<'a, T>;
    type IntoIter = WriteOnlyIter<'a, T>;

    /// Produces an iterator over [`WriteOnly<T>`][WriteOnly] for each element of
    /// this `WriteOnly<[T]>`.
    ///
    /// See also [`WriteOnly::write_iter()`] for the case where you already have an iterator
    /// of data to write.
    fn into_iter(self) -> Self::IntoIter {
        WriteOnlyIter { slice: self }
    }
}
impl<'a, T, const N: usize> IntoIterator for WriteOnly<'a, [T; N]> {
    type Item = WriteOnly<'a, T>;
    type IntoIter = WriteOnlyIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        WriteOnlyIter { slice: self.into() }
    }
}

/// Iterator over the elements of [`WriteOnly<[T]>`][WriteOnly].
///
/// It can be created by calling [`IntoIterator::into_iter()`] on a [`WriteOnly<[T]>`][WriteOnly].
///
/// See also [`WriteOnly::write_iter()`].
pub struct WriteOnlyIter<'a, T> {
    // Note: This is not the same as a [`slice::IterMut`], and may be less efficient.
    // We’re being less ambitious in exchange for less unsafe code.
    slice: WriteOnly<'a, [T]>,
}

impl<'a, T> Iterator for WriteOnlyIter<'a, T> {
    type Item = WriteOnly<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.slice.split_off_first()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.slice.len();
        (len, Some(len))
    }
}
impl<'a, T> ExactSizeIterator for WriteOnlyIter<'a, T> {}

impl<'a, T> DoubleEndedIterator for WriteOnlyIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.slice.split_off_last()
    }
}

#[track_caller]
#[inline]
fn checked_range_to_start_len(
    len: usize,
    slice_start: Bound<&usize>,
    slice_end: Bound<&usize>,
) -> (usize, usize) {
    // FIXME: cleaner panic messages
    let start: usize = match slice_start {
        Bound::Included(&i) => i,
        Bound::Excluded(&i) => i
            .checked_add(1)
            .expect("range bounds must be in numeric range"),
        Bound::Unbounded => 0,
    };
    let end: usize = match slice_end {
        Bound::Included(&i) => i
            .checked_add(1)
            .expect("range bounds must be in numeric range"),
        Bound::Excluded(&i) => i,
        Bound::Unbounded => len,
    };
    let new_len: usize = end
        .checked_sub(start)
        .expect("range must not have end > start");
    assert!(end <= len, "provided range was outside slice");
    // We checked start <= end and end <= len, so we also know that start <= self.len() here.

    (start, new_len)
}

/// Note: These tests are most useful if run under Miri to detect undefined behavior.
#[cfg(test)]
mod tests {
    use alloc::format;
    use alloc::string::String;
    use core::panic::{AssertUnwindSafe, UnwindSafe};

    use super::*;

    /// Helper for tests explicitly checking panics rather than using `#[should_panic]`
    fn expect_panic(f: impl FnOnce()) -> String {
        let payload = std::panic::catch_unwind(AssertUnwindSafe(f))
            .expect_err("function should have panicked");

        match payload.downcast::<String>() {
            Ok(string) => *string,
            Err(payload) => {
                if let Some(&string) = payload.downcast_ref::<&'static str>() {
                    String::from(string)
                } else {
                    panic!("non-string panic payload with type {:?}", payload.type_id());
                }
            }
        }
    }

    #[test]
    fn debug() {
        let mut arr = [1u8, 2, 3];
        assert_eq!(
            format!("{:#?}", WriteOnly::from_mut(&mut arr)),
            "WriteOnly([u8; 3])"
        );
        assert_eq!(
            format!("{:#?}", WriteOnly::from_mut(arr.as_mut_slice())),
            "WriteOnly([u8], len = 3)"
        );
        assert_eq!(
            format!("{:#?}", WriteOnly::from_mut(&mut arr[0])),
            "WriteOnly(u8)"
        );
    }

    #[test]
    fn default() {
        let empty = WriteOnly::<[u8]>::default();
        assert_eq!(empty.len(), 0);

        WriteOnly::<[char; 0]>::default().write([]);
    }

    #[test]
    fn array_to_slice() {
        let mut array = [0u8; 3];
        let array_wo = WriteOnly::from_mut(&mut array);

        // Ideally this could be an implicit unsizing coercion too, but that's not stable.
        let mut slice_wo: WriteOnly<'_, [u8]> = array_wo.into();
        slice_wo.copy_from_slice(&[1, 2, 3]);

        assert_eq!(array, [1, 2, 3]);
    }

    /// The rest of the tests and examples use `from_mut()` on `[T]` or arrays only,
    /// so let’s have at least one test of a type that hasn’t got any `[` or `]` in it.
    #[test]
    fn from_mut_for_non_slice() {
        let mut val = 1u32;
        let wo = WriteOnly::from_mut(&mut val);
        wo.write(2);
        assert_eq!(val, 2);
    }

    #[test]
    #[should_panic = "iterator given to write_iter() produced 3 elements but must produce 4 elements"]
    fn write_iter_too_short() {
        let mut buf = [0u8; 4];
        let wo = WriteOnly::from_mut(buf.as_mut_slice());

        wo.write_iter(1..=3);
    }

    #[test]
    #[should_panic = "iterator given to write_iter() produced more than 4 elements"]
    fn write_iter_too_long() {
        let mut buf = [0u8; 4];
        let wo = WriteOnly::from_mut(buf.as_mut_slice());

        wo.write_iter(1..=5);
    }

    #[test]
    fn write_iter_to_empty_slice_success() {
        let mut buf: [u8; 0] = [];
        let wo = WriteOnly::from_mut(buf.as_mut_slice());

        // does nothing, but shouldn’t panic
        wo.write_iter(core::iter::empty());
    }

    #[test]
    #[should_panic = "iterator given to write_iter() produced more than 0 elements"]
    fn write_iter_to_empty_slice_too_long() {
        let mut buf: [u8; 0] = [];
        let wo = WriteOnly::from_mut(buf.as_mut_slice());
        wo.write_iter(core::iter::once(1));
    }

    /// Tests that the slice length from `into_chunks()` is correct and that iteration works.
    #[test]
    fn into_chunks_has_correct_length_and_iterator_iterates() {
        let mut buf = [0u32; 8];

        let wo = WriteOnly::from_mut(buf.as_mut_slice());
        assert_eq!(wo.len(), 8);

        let (chunks, remainder): (WriteOnly<'_, [[u32; 4]]>, WriteOnly<'_, [u32]>) =
            wo.into_chunks::<4>();
        assert_eq!((chunks.len(), remainder.len()), (2, 0));

        for elem in chunks {
            elem.write([1, 2, 3, 4]);
        }
        assert_eq!(buf, [1, 2, 3, 4, 1, 2, 3, 4]);
    }

    #[test]
    fn into_chunks_with_remainder() {
        let mut buf = [0u8; 5];
        let wo = WriteOnly::from_mut(buf.as_mut_slice());

        let (mut chunks, mut remainder) = wo.into_chunks::<2>();
        chunks.fill([1, 2]);
        remainder.fill(100);

        assert_eq!(buf, [1, 2, 1, 2, 100]);
    }

    #[test]
    fn double_ended_iterator() {
        let mut buf = [0u8; 3];
        let mut iter = WriteOnly::from_mut(buf.as_mut_slice()).into_iter();

        iter.next_back().unwrap().write(3);
        iter.next().unwrap().write(1);
        iter.next_back().unwrap().write(2);

        assert!(iter.next().is_none());
        assert!(iter.next_back().is_none());
        assert_eq!(buf, [1, 2, 3]);
    }

    /// Test that slicing correctly panics on an out-of-bounds range.
    #[test]
    #[expect(clippy::reversed_empty_ranges)]
    fn slice_bounds_check_failures() {
        // RangeBounds isn’t dyn compatible, so we can’t make a list of test cases and have to
        // use a generic function.
        fn assert_oob(range: impl RangeBounds<usize> + UnwindSafe + fmt::Debug + Clone) {
            let panic_message_1 = expect_panic({
                let range = range.clone();
                let target: WriteOnly<'_, [char]> =
                    WriteOnly::from_mut(['a', 'b', 'c', 'd'].as_mut_slice());
                || {
                    _ = { target }.slice(range);
                }
            });
            // TODO: have more consistent errors so this assertion can be stronger
            assert!(
                panic_message_1.contains("range"),
                "expected .slice({range:?}) to panic with an out-of-bounds report,
                but got {panic_message_1:?}"
            );

            let panic_message_2 = expect_panic({
                let range = range.clone();
                let target: WriteOnly<'_, [char]> =
                    WriteOnly::from_mut(['a', 'b', 'c', 'd'].as_mut_slice());
                || {
                    _ = target.into_slice(range);
                }
            });
            assert!(
                panic_message_2.contains("range"),
                "expected .into_slice({range:?}) to panic with an out-of-bounds report,
                but got {panic_message_2:?}"
            );
        }

        assert_oob(..5);
        assert_oob(..=4);
        assert_oob(..usize::MAX);
        assert_oob(..=usize::MAX);
        assert_oob(2..5);
        assert_oob(2..=4);
        assert_oob(2..usize::MAX);
        assert_oob(2..=usize::MAX);
        assert_oob(5..4);
        assert_oob(5..=3);
    }

    #[test]
    fn slice_full_range() {
        let mut buf = [0u8; 4];
        let mut wo = WriteOnly::from_mut(buf.as_mut_slice());
        let mut wo2 = wo.slice(..);
        wo2.fill(7);
        assert_eq!(buf, [7, 7, 7, 7]);
    }

    #[test]
    fn split_off_out_of_bounds() {
        let mut buf = ['X'; 2];
        let mut wo = WriteOnly::from_mut(buf.as_mut_slice());

        assert!(wo.split_off(3..).is_none());
        assert!(wo.split_off(..3).is_none());

        // wo is unchanged by the attempts
        assert_eq!(wo.len(), 2);
    }

    /// Tests [`WriteOnly::split_off()`] with every kind of range it supports.
    #[test]
    fn split_off_success() {
        let mut buf = ['X'; 5];
        let mut wo = WriteOnly::from_mut(buf.as_mut_slice());

        // this particular combination of `Bound`s has no corresponding `Range*` type
        wo.split_off((Bound::Excluded(3), Bound::Unbounded))
            .unwrap()
            .copy_from_slice(&['e']);
        assert_eq!(wo.len(), 4);

        wo.split_off((Bound::Included(3), Bound::Unbounded))
            .unwrap()
            .copy_from_slice(&['d']);
        assert_eq!(wo.len(), 3);

        wo.split_off(..=0).unwrap().copy_from_slice(&['a']);
        assert_eq!(wo.len(), 2);

        wo.split_off(..1).unwrap().copy_from_slice(&['b']);
        assert_eq!(wo.len(), 1);

        wo.copy_from_slice(&['c']);

        assert_eq!(buf, ['a', 'b', 'c', 'd', 'e']);
    }

    #[test]
    #[should_panic = "split_off() requires a one-sided range"]
    fn split_off_interior_range() {
        _ = WriteOnly::from_mut([1, 2, 3].as_mut_slice()).split_off(1..2);
    }

    /// Tests both [`WriteOnly::split_off_first()`] and [`WriteOnly::split_off_last()`],
    /// with the same sequence of operations as [`split_off_success()`].
    #[test]
    fn split_off_first_and_last_success() {
        let mut buf = ['X'; 5];
        let mut wo = WriteOnly::from_mut(buf.as_mut_slice());

        wo.split_off_last().unwrap().write('e');
        wo.split_off_last().unwrap().write('d');
        wo.split_off_first().unwrap().write('a');
        wo.split_off_first().unwrap().write('b');
        wo.copy_from_slice(&['c']);

        assert_eq!(buf, ['a', 'b', 'c', 'd', 'e']);
    }

    #[test]
    fn split_off_first_and_last_empty() {
        let mut buf: [i32; 0] = [];
        let mut wo = WriteOnly::from_mut(buf.as_mut_slice());

        assert!(wo.split_off_first().is_none());
        assert!(wo.split_off_last().is_none());
    }

    #[test]
    #[should_panic(expected = "sizes of the two element types must be equal")]
    fn cast_elements_size_mismatch() {
        let mut buf = [0u8; 4];
        let wo = WriteOnly::from_mut(buf.as_mut_slice());
        unsafe { wo.cast_elements::<u16>() };
    }

    #[test]
    #[should_panic(expected = "alignment of the new element type must be \
                                  less than or equal to the alignment of the old element type")]
    fn cast_elements_alignment_mismatch() {
        #[repr(align(8))]
        struct BigAlign {
            _unused: u64,
        }

        // arrays are only as aligned as their elements
        let mut buf = [[0u8; 8]; 1];
        let wo = WriteOnly::from_mut(buf.as_mut_slice());

        unsafe { wo.cast_elements::<BigAlign>() };
    }

    // fill() has specialized implementation for byte-like types, so test all those, and
    // also a type that isn’t byte sized, and a type that is byte sized but not initialized.
    #[test]
    fn fill_byte_u8() {
        let mut buf = [0u8; 5];
        WriteOnly::from_mut(buf.as_mut_slice()).fill(42);
        assert_eq!(buf, [42; 5]);
    }
    #[test]
    fn fill_byte_i8() {
        let mut buf = [0i8; 5];
        WriteOnly::from_mut(buf.as_mut_slice()).fill(-42);
        assert_eq!(buf, [-42; 5]);
    }
    #[test]
    fn fill_byte_bool() {
        let mut buf = [false; 5];
        WriteOnly::from_mut(buf.as_mut_slice()).fill(true);
        assert_eq!(buf, [true; 5]);
    }
    #[test]
    fn fill_nonbyte_u16() {
        let mut buf = [0u16; 5];
        WriteOnly::from_mut(buf.as_mut_slice()).fill(12345);
        assert_eq!(buf, [12345; 5]);
    }
    #[test]
    fn fill_nonbyte_uninit() {
        let mut buf = [mem::MaybeUninit::<u8>::uninit(); 5];
        WriteOnly::from_mut(buf.as_mut_slice()).fill(mem::MaybeUninit::uninit());
        // Can't do a comparison, but we can at least let Miri notice if we just did UB.
    }
}
