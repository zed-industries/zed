#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::vec::Vec;

use core::pin::Pin;
use core::slice::SliceIndex;

// From: `futures_rs::join_all!` -- https://github.com/rust-lang/futures-rs/blob/b48eb2e9a9485ef7388edc2f177094a27e08e28b/futures-util/src/future/join_all.rs#L18-L23
pub(crate) fn iter_pin_mut<T>(slice: Pin<&mut [T]>) -> impl Iterator<Item = Pin<&mut T>> {
    // SAFETY: `std` _could_ make this unsound if it were to decide Pin's
    // invariants aren't required to transmit through slices. Otherwise this has
    // the same safety as a normal field pin projection.
    unsafe { slice.get_unchecked_mut() }
        .iter_mut()
        .map(|t| unsafe { Pin::new_unchecked(t) })
}

// From: `futures_rs::join_all!` -- https://github.com/rust-lang/futures-rs/blob/b48eb2e9a9485ef7388edc2f177094a27e08e28b/futures-util/src/future/join_all.rs#L18-L23
#[cfg(feature = "alloc")]
pub(crate) fn iter_pin_mut_vec<T>(slice: Pin<&mut Vec<T>>) -> impl Iterator<Item = Pin<&mut T>> {
    // SAFETY: `std` _could_ make this unsound if it were to decide Pin's
    // invariants aren't required to transmit through slices. Otherwise this has
    // the same safety as a normal field pin projection.
    unsafe { slice.get_unchecked_mut() }
        .iter_mut()
        .map(|t| unsafe { Pin::new_unchecked(t) })
}

/// Returns a pinned mutable reference to an element or subslice depending on the
/// type of index (see `get`) or `None` if the index is out of bounds.
// From: https://github.com/rust-lang/rust/pull/78370/files
#[inline]
pub(crate) fn get_pin_mut<T, I>(slice: Pin<&mut [T]>, index: I) -> Option<Pin<&mut I::Output>>
where
    I: SliceIndex<[T]>,
{
    // SAFETY: `get_unchecked_mut` is never used to move the slice inside `self` (`SliceIndex`
    // is sealed and all `SliceIndex::get_mut` implementations never move elements).
    // `x` is guaranteed to be pinned because it comes from `self` which is pinned.
    unsafe {
        slice
            .get_unchecked_mut()
            .get_mut(index)
            .map(|x| Pin::new_unchecked(x))
    }
}

// NOTE: If this is implemented through the trait, this will work on both vecs and
// slices.
//
// From: https://github.com/rust-lang/rust/pull/78370/files
#[cfg(feature = "alloc")]
pub(crate) fn get_pin_mut_from_vec<T, I>(
    slice: Pin<&mut Vec<T>>,
    index: I,
) -> Option<Pin<&mut I::Output>>
where
    I: SliceIndex<[T]>,
{
    // SAFETY: `get_unchecked_mut` is never used to move the slice inside `self` (`SliceIndex`
    // is sealed and all `SliceIndex::get_mut` implementations never move elements).
    // `x` is guaranteed to be pinned because it comes from `self` which is pinned.
    unsafe {
        slice
            .get_unchecked_mut()
            .get_mut(index)
            .map(|x| Pin::new_unchecked(x))
    }
}
