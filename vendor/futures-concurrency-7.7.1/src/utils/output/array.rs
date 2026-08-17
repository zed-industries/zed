use core::{
    array,
    mem::{self, MaybeUninit},
};

use crate::utils;

/// A contiguous array of uninitialized data.
pub(crate) struct OutputArray<T, const N: usize> {
    data: [MaybeUninit<T>; N],
}

impl<T, const N: usize> OutputArray<T, N> {
    /// Initialize a new array as uninitialized
    pub(crate) fn uninit() -> Self {
        Self {
            data: array::from_fn(|_| MaybeUninit::uninit()),
        }
    }

    /// Write a value into memory at the index
    pub(crate) fn write(&mut self, idx: usize, value: T) {
        self.data[idx] = MaybeUninit::new(value);
    }

    /// Drop a value at the index
    ///
    /// # Safety
    ///
    /// The value at the index must be initialized
    pub(crate) unsafe fn drop(&mut self, idx: usize) {
        // SAFETY: The caller is responsible for ensuring this value is
        // initialized
        unsafe { self.data[idx].assume_init_drop() };
    }

    /// Assume all items are initialized and take the items,
    /// leaving behind uninitialized data.
    ///
    /// # Safety
    ///
    /// Make sure that all items are initialized prior to calling this method.
    pub(crate) unsafe fn take(&mut self) -> [T; N] {
        let mut data = array::from_fn(|_| MaybeUninit::uninit());
        mem::swap(&mut self.data, &mut data);
        // SAFETY: the caller is on the hook to ensure all items are initialized
        unsafe { utils::array_assume_init(data) }
    }
}
