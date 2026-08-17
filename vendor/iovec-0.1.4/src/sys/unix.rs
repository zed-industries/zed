use libc;
use std::{mem, slice, usize};

pub struct IoVec {
    inner: [u8],
}

pub const MAX_LENGTH: usize = usize::MAX;

impl IoVec {
    pub fn as_ref(&self) -> &[u8] {
        unsafe {
            let vec = self.iovec();
            slice::from_raw_parts(vec.iov_base as *const u8, vec.iov_len)
        }
    }

    pub fn as_mut(&mut self) -> &mut [u8] {
        unsafe {
            let vec = self.iovec();
            slice::from_raw_parts_mut(vec.iov_base as *mut u8, vec.iov_len)
        }
    }

    unsafe fn iovec(&self) -> libc::iovec {
        mem::transmute(&self.inner)
    }
}

impl<'a> From<&'a [u8]> for &'a IoVec {
    fn from(src: &'a [u8]) -> Self {
        assert!(src.len() > 0);
        unsafe {
            mem::transmute(libc::iovec {
                iov_base: src.as_ptr() as *mut _,
                iov_len: src.len(),
            })
        }
    }
}

impl<'a> From<&'a mut [u8]> for &'a mut IoVec {
    fn from(src: &'a mut [u8]) -> Self {
        assert!(src.len() > 0);
        unsafe {
            mem::transmute(libc::iovec {
                iov_base: src.as_ptr() as *mut _,
                iov_len: src.len(),
            })
        }
    }
}
