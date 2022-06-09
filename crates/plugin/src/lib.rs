pub use bincode;
pub use serde;

// TODO: move the implementation to one place?
pub struct __Buffer {
    pub ptr: u32, // *const u8,
    pub len: u32, // usize,
}

impl __Buffer {
    pub fn into_u64(self) -> u64 {
        ((self.ptr as u64) << 32) | (self.len as u64)
    }

    pub fn from_u64(packed: u64) -> Self {
        __Buffer {
            ptr: (packed >> 32) as u32,
            len: packed as u32,
        }
    }
}

/// Allocates a buffer with an exact size.
/// We don't return the size because it has to be passed in anyway.
#[no_mangle]
pub extern "C" fn __alloc_buffer(len: u32) -> u32 {
    let vec = vec![0; len as usize];
    let buffer = unsafe { __Buffer::from_vec(vec) };
    return buffer.ptr;
}

/// Frees a given buffer, requires the size.
#[no_mangle]
pub extern "C" fn __free_buffer(buffer: u64) {
    let vec = unsafe { __Buffer::from_u64(buffer).to_vec() };
    std::mem::drop(vec);
}

impl __Buffer {
    #[inline(always)]
    pub unsafe fn to_vec(&self) -> Vec<u8> {
        core::slice::from_raw_parts(self.ptr as *const u8, self.len as usize).to_vec()
    }

    #[inline(always)]
    pub unsafe fn from_vec(mut vec: Vec<u8>) -> __Buffer {
        vec.shrink_to(0);
        let ptr = vec.as_ptr() as u32;
        let len = vec.len() as u32;
        std::mem::forget(vec);
        __Buffer { ptr, len }
    }
}

pub mod prelude {
    pub use super::{__Buffer, __alloc_buffer};
    pub use plugin_macros::{export, import};
}
