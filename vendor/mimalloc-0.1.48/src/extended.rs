use crate::MiMalloc;
use core::ffi::c_void;

impl MiMalloc {
    /// Get the mimalloc version.
    ///
    /// For mimalloc version 1.8.6, this will return 186.
    pub fn version(&self) -> u32 {
        unsafe { ffi::mi_version() as u32 }
    }

    /// Return the amount of available bytes in a memory block.
    ///
    /// # Safety
    /// `ptr` must point to a memory block allocated by mimalloc, or be null.
    #[inline]
    pub unsafe fn usable_size(&self, ptr: *const u8) -> usize {
        ffi::mi_usable_size(ptr as *const c_void)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use core::alloc::GlobalAlloc;
    use core::alloc::Layout;

    #[test]
    fn it_gets_version() {
        let version = MiMalloc.version();
        assert!(version != 0);
    }

    #[test]
    fn it_checks_usable_size() {
        unsafe {
            let layout = Layout::from_size_align(8, 8).unwrap();
            let alloc = MiMalloc;

            let ptr = alloc.alloc(layout);
            let usable_size = alloc.usable_size(ptr);
            alloc.dealloc(ptr, layout);
            assert!(usable_size >= 8);
        }
    }
}
