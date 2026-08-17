use crate::{
    macros::declare_ref_type,
    os_types::base::{Boolean, CVPixelBufferLockFlags, CVReturn, SizeT, VoidPtr},
};

declare_ref_type!(CVPixelBufferRef);

impl CVPixelBufferRef {
    pub fn is_planar(&self) -> bool {
        unsafe { CVPixelBufferIsPlanar(self) == 1 }
    }
    pub fn plane_count(&self) -> SizeT {
        unsafe { CVPixelBufferGetPlaneCount(self) }
    }
    pub fn get_base_address(&self) -> VoidPtr {
        unsafe { CVPixelBufferGetBaseAddress(self) }
    }
    pub fn get_base_address_of_plane(&self, plane_index: SizeT) -> VoidPtr {
        unsafe { CVPixelBufferGetBaseAddressOfPlane(self, plane_index) }
    }
    pub fn lock_base_address(&self, lock_flags: CVPixelBufferLockFlags) -> CVReturn {
        unsafe { CVPixelBufferLockBaseAddress(self, lock_flags) }
    }
    pub fn unlock_base_address(&self, lock_flags: CVPixelBufferLockFlags) -> CVReturn {
        unsafe { CVPixelBufferUnlockBaseAddress(self, lock_flags) }
    }
}

extern "C" {
    pub static kCVPixelBufferLock_ReadOnly: CVPixelBufferLockFlags;
    fn CVPixelBufferGetBaseAddress(pixel_buf: *const CVPixelBufferRef) -> VoidPtr;
    fn CVPixelBufferGetBaseAddressOfPlane(
        pixel_buf: *const CVPixelBufferRef,
        plane_index: SizeT,
    ) -> VoidPtr;
    fn CVPixelBufferGetPlaneCount(pixel_buf: *const CVPixelBufferRef) -> SizeT;

    fn CVPixelBufferIsPlanar(pixel_buf: *const CVPixelBufferRef) -> Boolean;
    fn CVPixelBufferLockBaseAddress(
        pixel_buf: *const CVPixelBufferRef,
        lock_flags: CVPixelBufferLockFlags,
    ) -> CVReturn;
    fn CVPixelBufferUnlockBaseAddress(
        pixel_buf: *const CVPixelBufferRef,
        lock_flags: CVPixelBufferLockFlags,
    ) -> CVReturn;
}
