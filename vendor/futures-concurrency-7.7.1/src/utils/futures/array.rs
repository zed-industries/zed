use core::{
    mem::{self, ManuallyDrop, MaybeUninit},
    pin::Pin,
};

/// An array of futures which can be dropped in-place, intended to be
/// constructed once and then accessed through pin projections.
pub(crate) struct FutureArray<T, const N: usize> {
    futures: [ManuallyDrop<T>; N],
}

impl<T, const N: usize> FutureArray<T, N> {
    /// Create a new instance of `FutureArray`
    pub(crate) fn new(futures: [T; N]) -> Self {
        // Implementation copied from: https://doc.rust-lang.org/src/core/mem/maybe_uninit.rs.html#1292
        let futures = MaybeUninit::new(futures);
        // SAFETY: T and MaybeUninit<T> have the same layout
        let futures = unsafe { mem::transmute_copy(&mem::ManuallyDrop::new(futures)) };
        Self { futures }
    }

    /// Create an iterator of pinned references.
    pub(crate) fn iter(self: Pin<&mut Self>) -> impl Iterator<Item = Pin<&mut ManuallyDrop<T>>> {
        // SAFETY: `std` _could_ make this unsound if it were to decide Pin's
        // invariants aren't required to transmit through slices. Otherwise this has
        // the same safety as a normal field pin projection.
        unsafe { self.get_unchecked_mut() }
            .futures
            .iter_mut()
            .map(|t| unsafe { Pin::new_unchecked(t) })
    }

    /// Drop a future at the given index.
    ///
    /// # Safety
    ///
    /// The future is held in a `ManuallyDrop`, so no double-dropping, etc
    pub(crate) unsafe fn drop(mut self: Pin<&mut Self>, idx: usize) {
        unsafe {
            let futures = self.as_mut().get_unchecked_mut().futures.as_mut();
            ManuallyDrop::drop(&mut futures[idx]);
        };
    }
}
