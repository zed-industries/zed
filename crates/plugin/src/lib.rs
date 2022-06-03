#[repr(C)]
pub struct __Buffer {
    pub ptr: *const u8,
    pub len: usize,
}

/// Allocates a buffer with an exact size.
/// We don't return the size because it has to be passed in anyway.
#[no_mangle]
pub extern "C" fn __alloc_buffer(len: usize) -> *const u8 {
    let vec = vec![0; len];
    let buffer = unsafe { __Buffer::from_vec(vec) };
    return buffer.ptr;
}

// /// Frees a given buffer, requires the size.
// #[no_mangle]
// pub extern "C" fn __free_buffer(ptr: *const u8, len: usize) {
//     let buffer = Buffer { ptr, len };
//     let vec = unsafe { buffer.to_vec() };
//     std::mem::drop(vec);
// }

impl __Buffer {
    #[inline(always)]
    pub unsafe fn to_vec(&self) -> Vec<u8> {
        core::slice::from_raw_parts(self.ptr, self.len).to_vec()
    }

    #[inline(always)]
    pub unsafe fn from_vec(mut vec: Vec<u8>) -> __Buffer {
        vec.shrink_to(0);
        let ptr = vec.as_ptr();
        let len = vec.len();
        std::mem::forget(vec);
        __Buffer { ptr, len }
    }

    #[inline(always)]
    pub fn leak_to_heap(self) -> *const __Buffer {
        let boxed = Box::new(self);
        let ptr = Box::<__Buffer>::into_raw(boxed) as *const __Buffer;
        return ptr;
    }
}

pub mod prelude {
    pub use super::{__Buffer, __alloc_buffer};
    pub use plugin_macros::bind;
}
