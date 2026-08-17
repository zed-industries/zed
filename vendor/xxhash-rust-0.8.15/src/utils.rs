//! Utilities of the crate
use core::{ptr, mem};

#[inline(always)]
pub const fn get_aligned_chunk_ref<T: Copy>(input: &[u8], offset: usize) -> &T {
    debug_assert!(mem::size_of::<T>() > 0); //Size MUST be positive
    debug_assert!(mem::size_of::<T>() <= input.len().saturating_sub(offset)); //Must fit

    unsafe {
        &*(input.as_ptr().add(offset) as *const T)
    }
}

#[allow(unused)]
#[inline(always)]
pub const fn get_aligned_chunk<T: Copy>(input: &[u8], offset: usize) -> T {
    *get_aligned_chunk_ref(input, offset)
}

#[inline(always)]
pub fn get_unaligned_chunk<T: Copy>(input: &[u8], offset: usize) -> T {
    debug_assert!(mem::size_of::<T>() > 0); //Size MUST be positive
    debug_assert!(mem::size_of::<T>() <= input.len().saturating_sub(offset)); //Must fit

    unsafe {
        ptr::read_unaligned(input.as_ptr().add(offset) as *const T)
    }
}

#[derive(Debug)]
pub struct Buffer<T> {
    pub ptr: T,
    pub len: usize,
    pub offset: usize,
}

impl Buffer<*mut u8> {
    #[inline(always)]
    pub fn copy_from_slice(&self, src: &[u8]) {
        self.copy_from_slice_by_size(src, src.len())
    }

    #[inline(always)]
    pub fn copy_from_slice_by_size(&self, src: &[u8], len: usize) {
        debug_assert!(self.len.saturating_sub(self.offset) >= len);

        unsafe {
            ptr::copy_nonoverlapping(src.as_ptr(), self.ptr.add(self.offset), len);
        }
    }
}
