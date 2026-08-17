use std::mem::MaybeUninit;

pub(crate) mod block;
pub(crate) mod frame;
pub(crate) mod hadamard;
pub(crate) mod mc;
pub(crate) mod motion;
pub(crate) mod plane;
pub(crate) mod prediction;
pub(crate) mod sad;
pub(crate) mod satd;
pub(crate) mod superblock;
pub(crate) mod tile;

/// Assume all the elements are initialized.
pub unsafe fn slice_assume_init_mut<T: Copy>(slice: &'_ mut [MaybeUninit<T>]) -> &'_ mut [T] {
    &mut *(slice as *mut [MaybeUninit<T>] as *mut [T])
}
