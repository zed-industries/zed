//! Optimization primitives for copyable types.

#[cfg(has_atomics)]
use core::sync::atomic::{
    AtomicBool, AtomicI16, AtomicI32, AtomicI8, AtomicU16, AtomicU32, AtomicU8,
};
#[cfg(has_atomics_64)]
use core::sync::atomic::{AtomicI64, AtomicU64};
use core::{
    marker::{PhantomData, PhantomPinned},
    num::{
        NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
        NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
    },
};

/// A type that is `Copy` and can be archived without additional processing.
///
/// This trait is similar to `Copy` in that it's automatically implemented for all types composed
/// entirely of other `ArchiveCopy` types. `Copy` is necessary, but not sufficient for `ArchiveCopy`
/// as some `Copy` type representations may vary from platform to platform.
#[rustc_unsafe_specialization_marker]
pub auto trait ArchiveCopy {}

// (), PhantomData, PhantomPinned, bool, i8, u8, NonZeroI8, and NonZeroU8 are always ArchiveCopy
impl<T: ?Sized> ArchiveCopy for PhantomData<T> {}

// Multibyte integers are not ArchiveCopy if the target does not match the archive endianness
#[cfg(any(
    all(target_endian = "little", feature = "archive_be"),
    all(target_endian = "big", feature = "archive_le"),
))]
const _: () = {
    impl !ArchiveCopy for i16 {}
    impl !ArchiveCopy for i32 {}
    impl !ArchiveCopy for i64 {}
    impl !ArchiveCopy for i128 {}
    impl !ArchiveCopy for u16 {}
    impl !ArchiveCopy for u32 {}
    impl !ArchiveCopy for u64 {}
    impl !ArchiveCopy for u128 {}
    impl !ArchiveCopy for f32 {}
    impl !ArchiveCopy for f64 {}
    impl !ArchiveCopy for char {}
    impl !ArchiveCopy for NonZeroI16 {}
    impl !ArchiveCopy for NonZeroI32 {}
    impl !ArchiveCopy for NonZeroI64 {}
    impl !ArchiveCopy for NonZeroI128 {}
    impl !ArchiveCopy for NonZeroU16 {}
    impl !ArchiveCopy for NonZeroU32 {}
    impl !ArchiveCopy for NonZeroU64 {}
    impl !ArchiveCopy for NonZeroU128 {}
};

// Pointer-sized integers are not ArchiveCopy if the target pointer width does not match the archive
// pointer width
#[cfg(any(
    all(target_pointer_width = "16", not(feature = "size_16")),
    all(target_pointer_width = "32", not(feature = "size_32")),
    all(target_pointer_width = "64", not(feature = "size_64"))
))]
const _: () = {
    impl !ArchiveCopy for isize {}
    impl !ArchiveCopy for usize {}
    impl !ArchiveCopy for NonZeroIsize {}
    impl !ArchiveCopy for NonZeroUsize {}
};

// Atomics are not ArchiveCopy if the platform supports them
#[cfg(has_atomics)]
const _: () = {
    impl !ArchiveCopy for AtomicBool {}
    impl !ArchiveCopy for AtomicI8 {}
    impl !ArchiveCopy for AtomicI16 {}
    impl !ArchiveCopy for AtomicI32 {}
    impl !ArchiveCopy for AtomicU8 {}
    impl !ArchiveCopy for AtomicU16 {}
    impl !ArchiveCopy for AtomicU32 {}
};
#[cfg(has_atomics_64)]
const _: () = {
    impl !ArchiveCopy for AtomicI64 {}
    impl !ArchiveCopy for AtomicU64 {}
};

// Pointers and references are never ArchiveCopy
impl<T: ?Sized> !ArchiveCopy for *const T {}
impl<T: ?Sized> !ArchiveCopy for *mut T {}
impl<T: ?Sized> !ArchiveCopy for &T {}
impl<T: ?Sized> !ArchiveCopy for &mut T {}

impl<O> !ArchiveCopy for crate::rel_ptr::RawRelPtr<O> {}

/// Types that are `ArchiveCopy` and have no padding.
///
/// These types are always safe to `memcpy` around because they will never contain uninitialized
/// padding.
#[rustc_unsafe_specialization_marker]
pub unsafe trait ArchiveCopySafe: ArchiveCopy + Sized {}

// (), PhantomData, PhantomPinned, bool, i8, u8, NonZeroI8, and NonZeroU8 are always ArchiveCopySafe
unsafe impl ArchiveCopySafe for () {}
unsafe impl<T: ?Sized> ArchiveCopySafe for PhantomData<T> {}
unsafe impl ArchiveCopySafe for PhantomPinned {}
unsafe impl ArchiveCopySafe for bool {}
unsafe impl ArchiveCopySafe for i8 {}
unsafe impl ArchiveCopySafe for u8 {}
unsafe impl ArchiveCopySafe for NonZeroI8 {}
unsafe impl ArchiveCopySafe for NonZeroU8 {}

// Multibyte integers are ArchiveCopySafe if the target matches the archived endianness
#[cfg(not(any(
    all(target_endian = "little", feature = "archive_be"),
    all(target_endian = "big", feature = "archive_le"),
)))]
const _: () = {
    unsafe impl ArchiveCopySafe for i16 {}
    unsafe impl ArchiveCopySafe for i32 {}
    unsafe impl ArchiveCopySafe for i64 {}
    unsafe impl ArchiveCopySafe for i128 {}
    unsafe impl ArchiveCopySafe for u16 {}
    unsafe impl ArchiveCopySafe for u32 {}
    unsafe impl ArchiveCopySafe for u64 {}
    unsafe impl ArchiveCopySafe for u128 {}
    unsafe impl ArchiveCopySafe for f32 {}
    unsafe impl ArchiveCopySafe for f64 {}
    unsafe impl ArchiveCopySafe for char {}
    unsafe impl ArchiveCopySafe for NonZeroI16 {}
    unsafe impl ArchiveCopySafe for NonZeroI32 {}
    unsafe impl ArchiveCopySafe for NonZeroI64 {}
    unsafe impl ArchiveCopySafe for NonZeroI128 {}
    unsafe impl ArchiveCopySafe for NonZeroU16 {}
    unsafe impl ArchiveCopySafe for NonZeroU32 {}
    unsafe impl ArchiveCopySafe for NonZeroU64 {}
    unsafe impl ArchiveCopySafe for NonZeroU128 {}
};

// Pointer-sized integers are ArchiveCopySafe if the target pointer width matches the archive
// pointer width
#[cfg(not(any(
    all(target_pointer_width = "16", not(feature = "size_16")),
    all(target_pointer_width = "32", not(feature = "size_32")),
    all(target_pointer_width = "64", not(feature = "size_64"))
)))]
const _: () = {
    unsafe impl ArchiveCopySafe for isize {}
    unsafe impl ArchiveCopySafe for usize {}
    unsafe impl ArchiveCopySafe for NonZeroIsize {}
    unsafe impl ArchiveCopySafe for NonZeroUsize {}
};

macro_rules! impl_tuple {
    () => {};
    (T, $($ts:ident,)*) => {
        unsafe impl<T: ArchiveCopySafe> ArchiveCopySafe for (T, $($ts,)*) {}

        impl_tuple!($($ts,)*);
    };
}

impl_tuple!(T, T, T, T, T, T, T, T, T, T, T,);

unsafe impl<T: ArchiveCopySafe, const N: usize> ArchiveCopySafe for [T; N] {}

/// Types that may be copy optimized.
///
/// By default, only [`ArchiveCopySafe`] types may be copy optimized. By enabling the `copy_unsafe`
/// feature, all types that are [`ArchiveCopy`] may be copy optimized.
#[cfg(not(feature = "copy_unsafe"))]
#[rustc_unsafe_specialization_marker]
pub trait ArchiveCopyOptimize: ArchiveCopySafe {}

#[cfg(not(feature = "copy_unsafe"))]
impl<T: ArchiveCopySafe> ArchiveCopyOptimize for T {}

/// Types that may be copy optimized.
///
/// By default, only [`ArchiveCopySafe`] types may be copy optimized. By enabling the `copy_unsafe`
/// feature, all types that are [`ArchiveCopy`] may be copy optimized.
#[cfg(feature = "copy_unsafe")]
#[rustc_unsafe_specialization_marker]
pub trait ArchiveCopyOptimize: ArchiveCopy {}

#[cfg(feature = "copy_unsafe")]
impl<T: ArchiveCopy> ArchiveCopyOptimize for T {}
