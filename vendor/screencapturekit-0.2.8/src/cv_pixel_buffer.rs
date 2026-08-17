use std::ffi::c_void;

use screencapturekit_sys::{cv_pixel_buffer_ref::CVPixelBufferRef, os_types::rc::ShareId};

#[derive(Debug)]
pub struct CVPixelBuffer {
    pub is_planar: bool,
    pub plane_count: u64,
    unsafe_ref: ShareId<CVPixelBufferRef>,
}

impl CVPixelBuffer {
    pub fn new(unsafe_ref: ShareId<CVPixelBufferRef>) -> Self {
        let is_planar = unsafe_ref.is_planar();
        let plane_count = unsafe_ref.plane_count();
        Self {
            unsafe_ref,
            plane_count,
            is_planar,
        }
    }
    pub fn lock(&self) -> bool {
        self.unsafe_ref.lock_base_address(0) == 0
    }
    pub fn unlock(&self) -> bool {
        self.unsafe_ref.unlock_base_address(0) == 0
    }
    pub fn get_base_adress(&self) -> *mut c_void {
        self.unsafe_ref.get_base_address()
    }
    pub fn get_base_adress_of_plane(&self, plane_index: u64) -> *mut c_void {
        self.unsafe_ref.get_base_address_of_plane(plane_index)
    }
}
