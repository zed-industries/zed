extern crate libc;

use std::marker::PhantomData;
use std::ops::Deref;
use std::slice;
use libc::c_void;

struct MallocPtr(*mut c_void);

impl Drop for MallocPtr {
    fn drop(&mut self) {
        unsafe {
            libc::free(self.0);
        }
    }
}

/// A type that represents a `malloc`'d chunk of memory.
pub struct MallocBuffer<T> {
    ptr: MallocPtr,
    len: usize,
    items: PhantomData<[T]>,
}

impl<T: Copy> MallocBuffer<T> {
    /// Constructs a new `MallocBuffer` for a `malloc`'d buffer
    /// with the given length at the given pointer.
    /// Returns `None` if the given pointer is null and the length is not 0.
    ///
    /// When this `MallocBuffer` drops, the buffer will be `free`'d.
    ///
    /// Unsafe because there must be `len` contiguous, valid instances of `T`
    /// at `ptr`.
    pub unsafe fn new(ptr: *mut T, len: usize) -> Option<MallocBuffer<T>> {
        if len > 0 && ptr.is_null() {
            None
        } else {
            Some(MallocBuffer {
                ptr: MallocPtr(ptr as *mut c_void),
                len: len,
                items: PhantomData,
            })
        }
    }
}

impl<T> Deref for MallocBuffer<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        let ptr = if self.len == 0 && self.ptr.0.is_null() {
            // Even a 0-size slice cannot be null, so just use another pointer
            0x1 as *const T
        } else {
            self.ptr.0 as *const T
        };
        unsafe {
            slice::from_raw_parts(ptr, self.len)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ptr;
    use libc;

    use super::MallocBuffer;

    #[test]
    fn test_null_buf() {
        let buf = unsafe {
            MallocBuffer::<u32>::new(ptr::null_mut(), 0).unwrap()
        };
        assert!(&*buf == []);
        assert!(Some(&*buf) == Some(&[]));

        let buf = unsafe {
            MallocBuffer::<u32>::new(ptr::null_mut(), 7)
        };
        assert!(buf.is_none());
    }

    #[test]
    fn test_buf() {
        let buf = unsafe {
            let ptr = libc::malloc(12) as *mut u32;
            *ptr = 1;
            *ptr.offset(1) = 2;
            *ptr.offset(2) = 3;
            MallocBuffer::new(ptr, 3).unwrap()
        };
        assert!(&*buf == [1, 2, 3]);
    }
}
