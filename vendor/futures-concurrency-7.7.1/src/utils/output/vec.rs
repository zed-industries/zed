#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::{vec, vec::Vec};

use core::mem::{self, MaybeUninit};

/// A contiguous vector of uninitialized data.
pub(crate) struct OutputVec<T> {
    data: Vec<T>,
    capacity: usize,
}

impl<T> OutputVec<T> {
    /// Initialize a new vector as uninitialized
    pub(crate) fn uninit(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            capacity,
        }
    }

    /// Write a value into memory at the index
    pub(crate) fn write(&mut self, idx: usize, value: T) {
        let data = self.data.spare_capacity_mut();
        data[idx] = MaybeUninit::new(value);
    }

    /// Drop a value at the index
    ///
    /// # Safety
    ///
    /// The value at the index must be initialized
    pub(crate) unsafe fn drop(&mut self, idx: usize) {
        // SAFETY: The caller is responsible for ensuring this value is
        // initialized
        let data = self.data.spare_capacity_mut();
        unsafe { data[idx].assume_init_drop() };
    }

    /// Assume all items are initialized and take the items,
    /// leaving behind an empty vector
    ///
    /// # Safety
    ///
    /// Make sure that all items are initialized prior to calling this method.
    pub(crate) unsafe fn take(&mut self) -> Vec<T> {
        let mut data = vec![];
        mem::swap(&mut self.data, &mut data);
        // SAFETY: the caller is on the hook to ensure all items are initialized
        unsafe { data.set_len(self.capacity) };
        data
    }
}
