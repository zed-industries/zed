use std::{mem, slice, usize};

#[derive(Clone)]
pub struct WasmIoVec {
    ptr: *const u8,
    len: usize,
}

pub struct IoVec {
    inner: [u8],
}

pub const MAX_LENGTH: usize = usize::MAX;

impl IoVec {
    pub fn as_ref(&self) -> &[u8] {
        unsafe {
            let vec = self.iovec();
            slice::from_raw_parts(vec.ptr as *const u8, vec.len)
        }
    }

    pub fn as_mut(&mut self) -> &mut [u8] {
        unsafe {
            let vec = self.iovec();
            slice::from_raw_parts_mut(vec.ptr as *mut u8, vec.len)
        }
    }

    unsafe fn iovec(&self) -> WasmIoVec {
        mem::transmute(&self.inner)
    }
}

impl<'a> From<&'a [u8]> for &'a IoVec {
    fn from(src: &'a [u8]) -> Self {
        assert!(src.len() > 0);
        unsafe {
            mem::transmute(WasmIoVec {
                ptr: src.as_ptr() as *mut _,
                len: src.len(),
            })
        }
    }
}

impl<'a> From<&'a mut [u8]> for &'a mut IoVec {
    fn from(src: &'a mut [u8]) -> Self {
        assert!(src.len() > 0);
        unsafe {
            mem::transmute(WasmIoVec {
                ptr: src.as_ptr() as *mut _,
                len: src.len(),
            })
        }
    }
}
