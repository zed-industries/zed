//! # `🗜 presser`
//!
//! **Utilities to help make copying data around into raw, possibly-uninitialized buffers
//! easier and safer.**
//!
//! ## Motivation
//!
//! `presser` can help you when copying data into raw buffers. One primary use-case is copying data into
//! graphics-api-allocated buffers which will then be accessed by the GPU. Common methods for doing this
//! right now in Rust can often invoke UB in subtle and hard-to-see ways. For example, viewing an allocated
//! but uninitialized buffer as an `&mut [u8]` **is instantly undefined behavior**\*, and `transmute`ing even a
//! `T: Copy` type which has *any padding bytes in its layout* as a `&[u8]` to be the source of a copy is
//! **also instantly undefined behavior**, in both cases because it is *invalid* to create a reference to an invalid
//! value (and uninitialized memory is an invalid `u8`), *even if* your code never actually accesses that memory.
//! This immediately makes what seems like the most straightforward way to copy data into buffers unsound 😬.
//!
//! `presser` helps with this by allowing you to view raw allocated memory of some size as a "[`Slab`]" of memory and then
//! provides *safe, valid* ways to copy data into that memory. For example, you could implement [`Slab`] for your
//! GPU-allocated buffer type, or use the built-in [`RawAllocation`] workflow described below, then use
//! [`copy_to_offset_with_align`] to copy any `T: Copy` data into that buffer safely for use on the GPU.
//! Of course, if your `T` doesn't have the correct layout the GPU expects, accessing it on the GPU side may still be
//! unsound or at least give an error.
//! 
//! \* *If you're currently thinking to yourself "bah! what's the issue? surely an uninit u8 is just any random bit pattern
//! and that's fine we don't care," [check out this blog post](https://www.ralfj.de/blog/2019/07/14/uninit.html) by
//! @RalfJung, one of the people leading the effort to better define Rust's memory and execution model. As is explored
//! in that blog post, an *uninit* piece of memory is not simply *an arbitrary bit pattern*, it is a wholly separate
//! state about a piece of memory, outside of its value, which lets the compiler perform optimizations that reorder,
//! delete, and otherwise change the actual execution flow of your program in ways that cannot be described simply
//! by "the value could have *some* possible bit pattern". LLVM and Clang are changing themselves to require special
//! `noundef` attribute to perform many important optimizations that are otherwise unsound. For a concrete example
//! of the sorts of problems this can cause, 
//! [see this issue @scottmcm hit](https://github.com/rust-lang/rust/pull/98919#issuecomment-1186106387).*
//!
//! ## Introduction
//!
//! The main idea is to implement [`Slab`] on raw-buffer-esque-types (see [the `Slab` safety docs][Slab#Safety]),
//! which then enables the use of the other functions within the crate.
//!
//! Depending on your use case, you may be able to implement [`Slab`] directly for your buffer type, or it may
//! be more convenient or necessary to create a wrapping struct that borrows your raw buffer type and in turn
//! implements [`Slab`]. For an example of this, see [`RawAllocation`] and [`BorrowedRawAllocation`], which you
//! may also use directly. The idea is to create a [`RawAllocation`] to your buffer, which you then borrow into
//! a [`BorrowedRawAllocation`] (which implements [`Slab`]) by calling the unsafe function
//! [`RawAllocation::borrow_as_slab`]
//!
//! Once you have a slab, you can use the copy helper functions provided at the crate root, for example,
//! [`copy_to_offset`] and [`copy_to_offset_with_align`].
//! 
//! ### Example
//! 
//! ```rust,ignore
//! #[derive(Clone, Copy)]
//! #[repr(C)]
//! struct MyDataStruct {
//!     a: u8,
//!     b: u32,
//! }
//!
//! let my_data = MyDataStruct { a: 0, b: 42 };
//!
//! // allocate an uninit buffer of some size
//! let my_buffer: MyBufferType = some_api.alloc_buffer_size(2048);
//!
//! // use `RawAllocation` helper to allow access to a presser `Slab`.
//! // alternatively, you could implement the `Slab` on `MyByfferType` directly if that
//! // type is owned by your code!
//! let raw_allocation = presser::RawAllocation::from_raw_parts(my_buffer.ptr(), my_buffer.size());
//!
//! // here we assert that we have exclusive access to the data in the buffer, and get the actual
//! // `Slab` to use to copy into.
//! let slab = unsafe { raw_allocation.borrow_as_slab(); }
//!
//! // now we may safely copy `my_data` into `my_buffer`, starting at a minimum offset of 0 into the buffer
//! let copy_record = presser::copy_to_offset(&my_data, &mut slab, 0)?;
//!
//! // note that due to alignment requirements of `my_data`, the *actual* start of the bytes of
//! // `my_data` may be placed at a different offset than requested. so, we check the returned
//! // `CopyRecord` to check the actual start offset of the copied data.
//! let actual_start_offset = copy_record.copy_start_offset;
//! ```
//!
//! ### `#[no_std]`
//!
//! This crate supports `no_std` environments by building without the '`std`' feature. This will limit some
//! of the fuctions the crate can perform.
//!
//! # Safety
//!
//! An important note is that obeying the safety rules specified in the [`Slab`] safety documentation
//! *only* guarantees safety for the *direct results* of the copy operations performed by the
//! helper functions exported at the crate root (and the safe functions on [`Slab`]). **However**,
//! it is ***not*** guaranteed that operations which would previously have been safe to perform
//! using same backing memory that the [`Slab`] you copied into used are still safe.
//!
//! For example, say you have a fully-initialized
//! chunk of bytes (like a `Vec<u8>`), which you view as a [`Slab`], and then (safely) perform a copy
//! operation into using [`copy_to_offset`]. If the `T` you copied into it has any padding bytes in
//! its memory layout, then the memory locations where those padding bytes now exist in the underlying `Vec`'s
//! memory must now be treated as uninitialized. As such, taking any view into that byte vector which
//! relies on those newly-uninit bytes being initialized to be valid (for example, taking a `&[u8]` slice of the `Vec`
//! which includes those bytes, ***even if your code never actually reads from that slice***)
//! is now instant **undefined behavior**.
#![cfg_attr(not(feature = "std"), no_std)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
// only enables the `doc_auto_cfg` feature when
// the `docs_build` configuration attribute is defined
// this cfg is defined when building on docs.rs (defined thru the project
// Cargo.toml) and when building the docs for publishing on github pages (thru the
// .github/workflows/rustdoc-pages.yml workflow)
#![cfg_attr(docs_build, feature(doc_auto_cfg))]

use core::alloc::Layout;
use core::alloc::LayoutError;
use core::marker::PhantomData;
use core::mem::MaybeUninit;
use core::ptr::NonNull;

/// Represents a contiguous piece of a single allocation with some layout that is used as a
/// data copying destination. May be wholly or partially uninitialized.
///
/// This trait is *basically* equivalent to implementing `Deref`/`DerefMut` with
/// `Target = [MaybeUninit<u8>]` in terms of safety requirements. It is a separate
/// trait for the extra flexibility having a trait we own provides: namely, the ability
/// to implement it on foreign types.
///
/// # Safety
///
/// Implementors of this trait must ensure these guarantees:
///
/// - The memory range represented by `base_ptr` and `size` **may** be wholly or partially uninitialized
/// - `base_ptr` **must** point to a valid, single allocation of at least `size` bytes.
/// - `size` **must not** be greater than `isize::MAX`
///
/// Assume the lifetime of a shared borrow of self is named `'a`:
///
/// - `base_ptr` **must** be [valid][`core::ptr#safety`] for `'a`
/// - `base_ptr` **must *not*** be mutably aliased for `'a`
///     - It is necessary but not sufficient for this requirement that
/// **no outside *mutable* references** may exist to its data, even if they are unused by user code.
///
/// Assume the lifetime of a mutable borrow of self is named `'a`:
///
/// - `base_ptr_mut` **must** be [valid][`core::ptr#safety`] for `'a`
/// - `base_ptr_mut` **must *not*** be aliased at all for `'a`
///     - It is necessary but not sufficient for this requirement that
/// **no outside references** may exist to its data, even if they are unused by user code.
///
/// Also see the [crate-level safety documentation][`crate#safety`].
pub unsafe trait Slab {
    /// Get a pointer to the beginning of the allocation represented by `self`.
    fn base_ptr(&self) -> *const u8;

    /// Get a pointer to the beginning of the allocation represented by `self`.
    fn base_ptr_mut(&mut self) -> *mut u8;

    /// Get the size of the allocation represented by `self`.
    fn size(&self) -> usize;

    /// Interpret a portion of `self` as a slice of [`MaybeUninit<u8>`]. This is likely not
    /// incredibly useful, you probably want to use [`Slab::as_maybe_uninit_bytes_mut`]
    fn as_maybe_uninit_bytes(&self) -> &[MaybeUninit<u8>] {
        // SAFETY: Safe so long as top level safety guarantees are held, since
        // `MaybeUninit` has same layout as bare type.
        unsafe { core::slice::from_raw_parts(self.base_ptr().cast(), self.size()) }
    }

    /// Interpret a portion of `self` as a mutable slice of [`MaybeUninit<u8>`].
    fn as_maybe_uninit_bytes_mut(&mut self) -> &mut [MaybeUninit<u8>] {
        // SAFETY: Safe so long as top level safety guarantees are held, since
        // `MaybeUninit` has same layout as bare type.
        unsafe { core::slice::from_raw_parts_mut(self.base_ptr_mut().cast(), self.size()) }
    }

    /// Interpret `self` as a byte slice. This assumes that **all bytes**
    /// in `self` are initialized.
    ///
    /// # Safety
    ///
    /// Assuming that the safety guarantees for creating `self` were followed,
    /// the only extra requirement for this to be safe is that **all memory**
    /// within the range of `self` must be **initialized**. If *any bytes* within
    /// this range are not initialized, using this function is *instantly **undefined
    /// behavior***, even if you *do noting* with the result.
    ///
    /// Also see the [crate-level Safety documentation][`crate#safety`] for more.
    unsafe fn assume_initialized_as_bytes(&self) -> &[u8] {
        // SAFETY: same requirements as function-level safety assuming the requirements
        // for creating `self` are met
        unsafe { core::slice::from_raw_parts(self.base_ptr().cast(), self.size()) }
    }

    /// Interpret `self` as a mutable byte slice. This assumes that **all bytes**
    /// in `self` are initialized.
    ///
    /// # Safety
    ///
    /// Assuming that the safety guarantees for creating `self` were followed,
    /// the only extra requirement for this to be safe is that **all memory**
    /// within the range of `self` must be **initialized**. If *any bytes* within
    /// this range are not initialized, using this function is *instantly **undefined
    /// behavior***, even if you *do noting* with the result.
    ///
    /// Also see the [crate-level Safety documentation][`crate#safety`] for more.
    unsafe fn assume_initialized_as_bytes_mut(&mut self) -> &mut [u8] {
        // SAFETY: same requirements as function-level safety assuming the requirements
        // for creating `self` are met
        unsafe { core::slice::from_raw_parts_mut(self.base_ptr_mut().cast(), self.size()) }
    }

    /// Interpret a range of `self` as a byte slice. This assumes that **all bytes**
    /// within `range` are initialized.
    ///
    /// In the future, this will hopefully not be needed as this operation will be equivalent to
    /// something like `self.as_maybe_uninit_bytes_mut()[range].assume_init()`, but the `core`/`std`
    /// implementation for this is still being scaffolded.
    ///
    /// # Safety
    ///
    /// Assuming that the safety guarantees for creating `self` were followed,
    /// the only extra requirement for this to be safe is that **all memory**
    /// within `range` must be **initialized**. If *any bytes* within
    /// this range are not initialized, using this function is *instantly **undefined
    /// behavior***, even if you *do noting* with the result.
    ///
    /// Also see the [crate-level Safety documentation][`crate#safety`] for more.
    unsafe fn assume_range_initialized_as_bytes<R>(&self, range: R) -> &[u8]
    where
        R: core::slice::SliceIndex<[MaybeUninit<u8>], Output = [MaybeUninit<u8>]>,
    {
        let maybe_uninit_slice = &self.as_maybe_uninit_bytes()[range];
        // SAFETY: same requirements as function-level safety assuming the requirements
        // for creating `self` are met since `MaybeUnint<T>` has same layout as `T`
        unsafe {
            core::slice::from_raw_parts(
                maybe_uninit_slice.as_ptr().cast(),
                maybe_uninit_slice.len(),
            )
        }
    }

    /// Interpret a range of `self` as a mutable byte slice. This assumes that **all bytes**
    /// within `range` are initialized.
    ///
    /// In the future, this will hopefully not be needed as this operation will be equivalent to
    /// something like `self.as_maybe_uninit_bytes_mut()[range].assume_init()`, but the `core`/`std`
    /// implementation for this is still being scaffolded.
    ///
    /// # Safety
    ///
    /// Assuming that the safety guarantees for creating `self` were followed,
    /// the only extra requirement for this to be safe is that **all memory**
    /// within `range` must be **initialized**. If *any bytes* within
    /// this range are not initialized, using this function is *instantly **undefined
    /// behavior***, even if you *do noting* with the result.
    ///
    /// Also see the [crate-level Safety documentation][`crate#safety`] for more.
    unsafe fn assume_range_initialized_as_bytes_mut<R>(&mut self, range: R) -> &mut [u8]
    where
        R: core::slice::SliceIndex<[MaybeUninit<u8>], Output = [MaybeUninit<u8>]>,
    {
        let maybe_uninit_slice = &mut self.as_maybe_uninit_bytes_mut()[range];
        // SAFETY: same requirements as function-level safety assuming the requirements
        // for creating `self` are met since `MaybeUnint<T>` has same layout as `T`
        unsafe {
            core::slice::from_raw_parts_mut(
                maybe_uninit_slice.as_mut_ptr().cast(),
                maybe_uninit_slice.len(),
            )
        }
    }
}

// SAFETY: The captured `[MaybeUninit<u8>]` will all be part of the same allocation object, and borrowck
// will assure that the borrows that occur on `self` on the relevant methods live long enough since they are
// native borrows anyway.
unsafe impl Slab for [MaybeUninit<u8>] {
    fn base_ptr(&self) -> *const u8 {
        self.as_ptr().cast()
    }

    fn base_ptr_mut(&mut self) -> *mut u8 {
        self.as_mut_ptr().cast()
    }

    fn size(&self) -> usize {
        core::mem::size_of_val(self)
    }
}

/// Takes a `Vec` and unsafely resizes it to the given length, returning a mutable slice to `MaybeUninit<T>` for each
/// item in the newly-resized `Vec`.
///
/// # Safety
///
/// You promise that the given `Vec` already has at least `length` capacity. You also promise to either fill all items before dropping
/// the returned slice, or to continue to not violate validity rules for any items that you do not initialize.
#[cfg(feature = "std")]
pub unsafe fn maybe_uninit_slice_from_vec<T>(
    vec: &mut Vec<T>,
    length: usize,
) -> &mut [MaybeUninit<T>] {
    // SAFETY: As long as the function-level safety rules are met, this is valid
    unsafe {
        #[allow(clippy::uninit_vec)]
        vec.set_len(length);
    }

    // SAFETY: If function-level safety is met, then we are constructing a slice within a single allocation. `MaybeUninit<T>` is valid
    // even for uninit memory, and has the same memory layout as `T`.
    unsafe { core::slice::from_raw_parts_mut(vec.as_mut_ptr().cast::<MaybeUninit<T>>(), length) }
}

/// Copies the elements from `src` to `dst`, returning a mutable reference to the now initialized contents of `dst`.
///
/// If `T` does not implement `Copy`, use [`clone_into_maybe_uninit_slice`]
///
/// This is similar to [`slice::copy_from_slice`]. This is identical to the implementation of the method
/// `write_to_slice` on [`MaybeUninit`], but that API is as yet unstable.
///
/// # Panics
///
/// This function will panic if the two slices have different lengths.
pub fn copy_into_maybe_uninit_slice<'a, T>(src: &[T], dst: &'a mut [MaybeUninit<T>]) -> &'a mut [T]
where
    T: Copy,
{
    let uninit_src: &[MaybeUninit<T>] =
        // SAFETY: &[T] and &[MaybeUninit<T>] have the same layout
        unsafe { &*(src as *const [T] as *const [MaybeUninit<T>]) };

    dst.copy_from_slice(uninit_src);

    // SAFETY: Valid elements have just been copied into `this` so it is initialized
    unsafe { &mut *(dst as *mut [MaybeUninit<T>] as *mut [T]) }
}

/// Clones the elements from `src` to `dst`, returning a mutable reference to the now initialized contents of `dst`.
/// Any already initialized elements will not be dropped.
///
/// If `T` implements `Copy`, use [`copy_into_maybe_uninit_slice`]
///
/// This is similar to [`slice::clone_from_slice`] but does not drop existing elements. This is identical to the implementation of
/// the method `write_to_slice_cloned` on [`MaybeUninit`], but that API is as yet unstable.
///
/// # Panics
///
/// This function will panic if the two slices have different lengths, or if the implementation of `Clone` panics.
///
/// If there is a panic, the already cloned elements will be dropped.
pub fn clone_into_maybe_uninit_slice<'a, T>(src: &[T], dst: &'a mut [MaybeUninit<T>]) -> &'a mut [T]
where
    T: Clone,
{
    // unlike copy_from_slice this does not call clone_from_slice on the slice
    // this is because `MaybeUninit<T: Clone>` does not implement Clone.

    struct Guard<'a, T> {
        slice: &'a mut [MaybeUninit<T>],
        initialized: usize,
    }

    impl<'a, T> Drop for Guard<'a, T> {
        fn drop(&mut self) {
            let initialized_part = &mut self.slice[..self.initialized];
            // SAFETY: this raw slice will contain only initialized objects
            // that's why, it is allowed to drop it.
            unsafe {
                core::ptr::drop_in_place(
                    &mut *(initialized_part as *mut [MaybeUninit<T>] as *mut [T]),
                );
            }
        }
    }

    assert_eq!(
        dst.len(),
        src.len(),
        "destination and source slices have different lengths"
    );
    // NOTE: We need to explicitly slice them to the same length
    // for bounds checking to be elided, and the optimizer will
    // generate memcpy for simple cases (for example T = u8).
    let len = dst.len();
    let src = &src[..len];

    // guard is needed b/c panic might happen during a clone
    let mut guard = Guard {
        slice: dst,
        initialized: 0,
    };

    #[allow(clippy::needless_range_loop)]
    for i in 0..len {
        guard.slice[i].write(src[i].clone());
        guard.initialized += 1;
    }

    #[allow(clippy::mem_forget)]
    core::mem::forget(guard);

    // SAFETY: Valid elements have just been written into `this` so it is initialized
    unsafe { &mut *(dst as *mut [MaybeUninit<T>] as *mut [T]) }
}

/// Represents a contiguous piece of a single allocation with some layout.
/// May be wholly or partially uninitialized.
///
/// This exists as a convenient way to get access to a type implementing [`Slab`]
/// when dealing with your own raw allocations/buffers if you don't want to or
/// cannot implement [`Slab`] for another native type.
pub struct RawAllocation {
    /// A pointer to the base address of the allocation
    pub base_ptr: NonNull<u8>,

    /// The size of the allocation in bytes
    pub size: usize,
}

impl RawAllocation {
    /// Create a new [`RawAllocation`] from a pointer and size.
    ///
    /// # Safety
    ///
    /// This function is safe in and of itself, as nothing will be done
    /// with the pointer and size upon creation.
    pub fn from_raw_parts(base_ptr: NonNull<u8>, size: usize) -> Self {
        Self { base_ptr, size }
    }

    /// Asserts that we are uniquely borrowing the memory range represented by `self` for
    /// the duration of the borrow, giving us a [`BorrowedRawAllocation`] which implements [`Slab`].
    ///
    /// # Safety
    ///
    /// Using this method makes some strong guarantees about the contained `base_ptr` and `size`
    /// for the duration of the borrow. See the [safety][`Slab#safety`] documentation for the
    /// [`Slab`] trait for a list of the guarantees you must make to use this method.
    ///
    /// Also see the [top-level safety documentation][`crate#safety`]
    #[allow(clippy::needless_lifetimes)] // Important to be explicit in this case because of unsafety
    pub unsafe fn borrow_as_slab<'a>(&'a mut self) -> BorrowedRawAllocation<'a> {
        BorrowedRawAllocation {
            base_ptr: self.base_ptr,
            size: self.size,
            phantom: PhantomData,
        }
    }
}

/// Represents the unique borrow of a contiguous piece of a single allocation with some layout that is used as a
/// data copying destination. May be wholly or partially uninitialized.
///
/// This type can only be obtained through the [`borrow_as_slab`][`RawAllocation::borrow_as_slab`] method on [`RawAllocation`].
pub struct BorrowedRawAllocation<'a> {
    base_ptr: NonNull<u8>,
    size: usize,
    phantom: PhantomData<&'a ()>,
}

// SAFETY: So long as the safety requirements of `borrow_as_slab` are met, this is also safe
// since it's just a basic pass-thru of info.
unsafe impl<'a> Slab for BorrowedRawAllocation<'a> {
    fn base_ptr(&self) -> *const u8 {
        self.base_ptr.as_ptr() as *const u8
    }

    fn base_ptr_mut(&mut self) -> *mut u8 {
        self.base_ptr.as_ptr()
    }

    fn size(&self) -> usize {
        self.size
    }
}

/// Given pointer and offset, returns a new offset aligned to `align`.
///
/// `align` *must* be a power of two and >= 1 or else the result is meaningless.
fn align_offset_up_to(ptr: usize, offset: usize, align: usize) -> Option<usize> {
    let offsetted_ptr = ptr.checked_add(offset)?;
    let aligned_ptr = offsetted_ptr.checked_add(align - 1)? & !(align - 1);
    // don't need to check since we know aligned_ptr is >= ptr at this point
    Some(aligned_ptr - ptr)
}

/// Compute and validate offsets for a copy operation with the given parameters.
fn compute_offsets<S: Slab>(
    dst: &S,
    start_offset: usize,
    t_layout: Layout,
    min_alignment: usize,
) -> Result<CopyRecord, CopyError> {
    let copy_layout = t_layout.align_to(min_alignment.next_power_of_two())?;

    let copy_start_offset =
        align_offset_up_to(dst.base_ptr() as usize, start_offset, copy_layout.align())
            .ok_or(CopyError::InvalidLayout)?;
    let copy_end_offset = copy_start_offset
        .checked_add(copy_layout.size())
        .ok_or(CopyError::InvalidLayout)?;
    let copy_end_offset_padded = copy_start_offset
        .checked_add(copy_layout.pad_to_align().size())
        .ok_or(CopyError::InvalidLayout)?;

    // check start is inside slab
    // if within slab, we also know that copy_start_offset is <= isize::MAX since slab.size() must be <= isize::MAX
    if copy_start_offset > dst.size() {
        return Err(CopyError::OffsetOutOfBounds);
    }

    // check end is inside slab
    if copy_end_offset_padded > dst.size() {
        return Err(CopyError::OutOfMemory);
    }

    Ok(CopyRecord {
        copy_start_offset,
        copy_end_offset,
        copy_end_offset_padded,
    })
}

/// An error that may occur during a copy operation.
#[derive(Debug)]
pub enum CopyError {
    /// Copy would exceed the end of the allocation
    OutOfMemory,
    /// Requested to copy to an offset outside the bounds of the allocation
    OffsetOutOfBounds,
    /// Computed invalid layout for copy operation, probably caused by incredibly large size, offset, or min-alignment parameters
    InvalidLayout,
}

impl core::fmt::Display for CopyError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", match self {
            Self::OutOfMemory => "Copy would exceed the end of the allocation",
            Self::OffsetOutOfBounds => "Requested copy to a location starting outside the allocation",
            Self::InvalidLayout => "Invalid layout, probably caused by incredibly large size, offset, or alignment parameters",
        })
    }
}

#[cfg(feature = "std")]
impl std::error::Error for CopyError {}

impl From<LayoutError> for CopyError {
    fn from(_err: LayoutError) -> Self {
        Self::InvalidLayout
    }
}

/// Record of the results of a copy operation
#[derive(Debug, Copy, Clone)]
pub struct CopyRecord {
    /// The offset from the start of the allocation, in bytes, at which the
    /// copy operation began to write data.
    ///
    /// Not necessarily equal to the `start_offset`, since this offset
    /// includes necessary padding to assure alignment.
    pub copy_start_offset: usize,

    /// The offset from the start of the allocation, in bytes, at which the
    /// copy operation no longer wrote data.
    ///
    /// This does not include any padding at the end necessary to maintain
    /// alignment requirements.
    pub copy_end_offset: usize,

    /// The offset from the start of the allocation, in bytes, at which the
    /// copy operation no longer wrote data, plus any padding necessary to
    /// maintain derived alignment requirements.
    pub copy_end_offset_padded: usize,
}

/// Copies `src` into the memory represented by `dst` starting at a minimum location
/// of `start_offset` bytes past the start of `dst`.
///
/// - `start_offset` is the offset into the allocation represented by `dst`,
/// in bytes, before which any copied data will *certainly not* be placed. However,
/// the actual beginning of the copied data may not be exactly at `start_offset` if
/// padding bytes are needed to satisfy alignment requirements. The actual beginning
/// of the copied bytes is contained in the returned [`CopyRecord`].
///
/// # Safety
///
/// This function is safe on its own, however it is very possible to do unsafe
/// things if you read the copied data in the wrong way. See the
/// [crate-level Safety documentation][`crate#safety`] for more.
#[inline]
pub fn copy_to_offset<T: Copy, S: Slab>(
    src: &T,
    dst: &mut S,
    start_offset: usize,
) -> Result<CopyRecord, CopyError> {
    copy_to_offset_with_align(src, dst, start_offset, 1)
}

/// Copies `src` into the memory represented by `dst` starting at a minimum location
/// of `start_offset` bytes past the start of `dst` and with minimum alignment
/// `min_alignment`.
///
/// - `start_offset` is the offset into the allocation represented by `dst`,
/// in bytes, before which any copied data will *certainly not* be placed. However,
/// the actual beginning of the copied data may not be exactly at `start_offset` if
/// padding bytes are needed to satisfy alignment requirements. The actual beginning
/// of the copied bytes is contained in the returned [`CopyRecord`].
/// - `min_alignment` is the minimum alignment to which the copy will be aligned. The
/// copy may not actually be aligned to `min_alignment` depending on the alignment requirements
/// of `T`.
///
/// # Safety
///
/// This function is safe on its own, however it is very possible to do unsafe
/// things if you read the copied data in the wrong way. See the
/// [crate-level Safety documentation][`crate#safety`] for more.
pub fn copy_to_offset_with_align<T: Copy, S: Slab>(
    src: &T,
    dst: &mut S,
    start_offset: usize,
    min_alignment: usize,
) -> Result<CopyRecord, CopyError> {
    let t_layout = Layout::new::<T>();
    let record = compute_offsets(&*dst, start_offset, t_layout, min_alignment)?;

    // SAFETY: if compute_offsets succeeded, this has already been checked to be safe.
    let dst_ptr = unsafe { dst.base_ptr_mut().add(record.copy_start_offset) }.cast::<T>();

    // SAFETY:
    // - src is valid as we have a reference to it
    // - dst is valid so long as requirements for `slab` were met, i.e.
    // we have unique access to the region described and that it is valid for the duration
    // of 'a.
    // - areas not overlapping as long as safety requirements of creation of `self` were met,
    // i.e. that we have exclusive access to the region of memory described.
    // - dst aligned at least to align_of::<T>()
    // - checked that copy stays within bounds of our allocation
    unsafe {
        core::ptr::copy_nonoverlapping(src as *const T, dst_ptr, 1);
    }

    Ok(record)
}

/// Copies from `slice` into the memory represented by `dst` starting at a minimum location
/// of `start_offset` bytes past the start of `self`.
///
/// - `start_offset` is the offset into the allocation represented by `dst`,
/// in bytes, before which any copied data will *certainly not* be placed. However,
/// the actual beginning of the copied data may not be exactly at `start_offset` if
/// padding bytes are needed to satisfy alignment requirements. The actual beginning
/// of the copied bytes is contained in the returned [`CopyRecord`].
///
/// # Safety
///
/// This function is safe on its own, however it is very possible to do unsafe
/// things if you read the copied data in the wrong way. See the
/// [crate-level Safety documentation][`crate#safety`] for more.
#[inline]
pub fn copy_from_slice_to_offset<T: Copy, S: Slab>(
    src: &[T],
    dst: &mut S,
    start_offset: usize,
) -> Result<CopyRecord, CopyError> {
    copy_from_slice_to_offset_with_align(src, dst, start_offset, 1)
}

/// Copies from `slice` into the memory represented by `dst` starting at a minimum location
/// of `start_offset` bytes past the start of `dst`.
///
/// - `start_offset` is the offset into the allocation represented by `dst`,
/// in bytes, before which any copied data will *certainly not* be placed. However,
/// the actual beginning of the copied data may not be exactly at `start_offset` if
/// padding bytes are needed to satisfy alignment requirements. The actual beginning
/// of the copied bytes is contained in the returned [`CopyRecord`].
/// - `min_alignment` is the minimum alignment to which the copy will be aligned. The
/// copy may not actually be aligned to `min_alignment` depending on the alignment requirements
/// of `T` and the underlying allocation.
///     - The whole data of the slice will be copied directly, so, alignment between elements
/// ignores `min_alignment`.
///
/// # Safety
///
/// This function is safe on its own, however it is very possible to do unsafe
/// things if you read the copied data in the wrong way. See the
/// [crate-level Safety documentation][`crate#safety`] for more.
pub fn copy_from_slice_to_offset_with_align<T: Copy, S: Slab>(
    src: &[T],
    dst: &mut S,
    start_offset: usize,
    min_alignment: usize,
) -> Result<CopyRecord, CopyError> {
    let t_layout = Layout::for_value(src);
    let record = compute_offsets(&*dst, start_offset, t_layout, min_alignment)?;

    // SAFETY: if compute_offsets succeeded, this has already been checked to be safe.
    let dst_ptr = unsafe { dst.base_ptr_mut().add(record.copy_start_offset) }.cast::<T>();

    // SAFETY:
    // - src is valid as we have a reference to it
    // - dst is valid so long as requirements for `slab` were met, i.e.
    // we have unique access to the region described and that it is valid for the duration
    // of 'a.
    // - areas not overlapping as long as safety requirements of creation of `self` were met,
    // i.e. that we have exclusive access to the region of memory described.
    // - dst aligned at least to align_of::<T>()
    // - checked that copy stays within bounds of our allocation
    unsafe {
        core::ptr::copy_nonoverlapping(src.as_ptr(), dst_ptr, src.len());
    }

    Ok(record)
}

/// Copies from `src` iterator into the memory represented by `dst` starting at a minimum location
/// of `start_offset` bytes past the start of `dst`.
///
/// Returns a vector of [`CopyRecord`]s, one for each item in the `src` iterator.
///
/// - `start_offset` is the offset into the allocation represented by `dst`,
/// in bytes, before which any copied data will *certainly not* be placed. However,
/// the actual beginning of the copied data may not be exactly at `start_offset` if
/// padding bytes are needed to satisfy alignment requirements. The actual beginning
/// of the copied bytes is contained in the returned [`CopyRecord`]s.
/// - `min_alignment` is the minimum alignment to which the copy will be aligned. The
/// copy may not actually be aligned to `min_alignment` depending on the alignment requirements
/// of `T`.
/// - For this variation, `min_alignment` will also be respected *between* elements yielded by
/// the iterator. To copy inner elements aligned only to `align_of::<T>()`, see
/// [`copy_from_iter_to_offset_with_align_packed`]
///
/// # Safety
///
/// This function is safe on its own, however it is very possible to do unsafe
/// things if you read the copied data in the wrong way. See the
/// [crate-level Safety documentation][`crate#safety`] for more.
#[cfg(feature = "std")]
pub fn copy_from_iter_to_offset_with_align<T: Copy, Iter: Iterator<Item = T>, S: Slab>(
    src: Iter,
    dst: &mut S,
    start_offset: usize,
    min_alignment: usize,
) -> Result<Vec<CopyRecord>, CopyError> {
    let mut offset = start_offset;

    src.map(|item| {
        let copy_record = copy_to_offset_with_align(&item, dst, offset, min_alignment)?;
        offset = copy_record.copy_end_offset;
        Ok(copy_record)
    })
    .collect::<Result<Vec<_>, _>>()
}

/// Like [`copy_from_iter_to_offset_with_align`] except that
/// alignment between elements yielded by the iterator will ignore `min_alignment`
/// and rather only be aligned to the alignment of `T`.
///
/// Because of this, only one [`CopyRecord`] is returned specifying the record of the
/// entire block of copied data. If the `src` iterator is empty, returns `None`.
pub fn copy_from_iter_to_offset_with_align_packed<T: Copy, Iter: Iterator<Item = T>, S: Slab>(
    mut src: Iter,
    dst: &mut S,
    start_offset: usize,
    min_alignment: usize,
) -> Result<Option<CopyRecord>, CopyError> {
    let first_record = if let Some(first_item) = src.next() {
        copy_to_offset_with_align(&first_item, dst, start_offset, min_alignment)?
    } else {
        return Ok(None);
    };

    let mut prev_record = first_record;

    for item in src {
        let copy_record = copy_to_offset_with_align(&item, dst, prev_record.copy_end_offset, 1)?;
        prev_record = copy_record;
    }

    Ok(Some(CopyRecord {
        copy_start_offset: first_record.copy_start_offset,
        copy_end_offset: prev_record.copy_end_offset,
        copy_end_offset_padded: prev_record.copy_end_offset_padded,
    }))
}
