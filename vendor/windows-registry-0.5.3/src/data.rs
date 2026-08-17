use super::*;

// Minimal `Vec<u8>` replacement providing at least `u16` alignment so that it can be used for wide strings.
pub struct Data {
    ptr: *mut u8,
    len: usize,
}

impl Data {
    // Creates a buffer with the specified length of zero bytes.
    pub fn new(len: usize) -> Self {
        unsafe {
            let bytes = Self::alloc(len);

            if len > 0 {
                core::ptr::write_bytes(bytes.ptr, 0, len);
            }

            bytes
        }
    }

    // Returns the buffer as a slice of u16 for reading wide characters.
    pub fn as_wide(&self) -> &[u16] {
        if self.ptr.is_null() {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.ptr as *const u16, self.len / 2) }
        }
    }

    // Creates a buffer by copying the bytes from the slice.
    pub fn from_slice(slice: &[u8]) -> Self {
        unsafe {
            let bytes = Self::alloc(slice.len());

            if !slice.is_empty() {
                core::ptr::copy_nonoverlapping(slice.as_ptr(), bytes.ptr, slice.len());
            }

            bytes
        }
    }

    // Allocates an uninitialized buffer.
    unsafe fn alloc(len: usize) -> Self {
        if len == 0 {
            Self {
                ptr: null_mut(),
                len: 0,
            }
        } else {
            // This pointer will have at least 8 byte alignment.
            let ptr = unsafe { HeapAlloc(GetProcessHeap(), 0, len) as *mut u8 };

            if ptr.is_null() {
                panic!("allocation failed");
            }

            Self { ptr, len }
        }
    }
}

impl Drop for Data {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                HeapFree(GetProcessHeap(), 0, self.ptr as *mut _);
            }
        }
    }
}

impl Deref for Data {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        if self.ptr.is_null() {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.ptr, self.len) }
        }
    }
}

impl core::ops::DerefMut for Data {
    fn deref_mut(&mut self) -> &mut [u8] {
        if self.ptr.is_null() {
            &mut []
        } else {
            unsafe { core::slice::from_raw_parts_mut(self.ptr, self.len) }
        }
    }
}

impl Clone for Data {
    fn clone(&self) -> Self {
        Self::from_slice(self)
    }
}

impl PartialEq for Data {
    fn eq(&self, other: &Self) -> bool {
        self.deref() == other.deref()
    }
}

impl Eq for Data {}

impl core::fmt::Debug for Data {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.deref().fmt(f)
    }
}

impl<const N: usize> From<[u8; N]> for Data {
    fn from(from: [u8; N]) -> Self {
        Self::from_slice(&from)
    }
}
