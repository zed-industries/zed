use core_foundation_sys::{
    base::{CFAllocatorRef, CFTypeID, CFTypeRef},
    dictionary::CFDictionaryRef,
    string::CFStringRef,
};
use libc::c_void;

use crate::{base::CVOptionFlags, pixel_buffer::CVPixelBufferRef, return_::CVReturn};

pub type CVPixelBufferPoolRef = CFTypeRef;
pub type CVPixelBufferPoolFlushFlags = CVOptionFlags;

pub const kCVPixelBufferPoolFlushExcessBuffers: CVPixelBufferPoolFlushFlags = 1;

extern "C" {
    pub static kCVPixelBufferPoolMinimumBufferCountKey: CFStringRef;
    pub static kCVPixelBufferPoolMaximumBufferAgeKey: CFStringRef;

    pub static kCVPixelBufferPoolAllocationThresholdKey: CFStringRef;
    pub static kCVPixelBufferPoolFreeBufferNotification: CFStringRef;

    pub fn CVPixelBufferPoolGetTypeID() -> CFTypeID;
    pub fn CVPixelBufferPoolRetain(pixelBufferPool: CVPixelBufferPoolRef) -> CVPixelBufferPoolRef;
    pub fn CVPixelBufferPoolRelease(pixelBufferPool: CVPixelBufferPoolRef) -> c_void;
    pub fn CVPixelBufferPoolCreate(
        allocator: CFAllocatorRef,
        poolAttributes: CFDictionaryRef,
        pixelBufferAttributes: CFDictionaryRef,
        poolOut: *mut CVPixelBufferPoolRef,
    ) -> CVReturn;
    pub fn CVPixelBufferPoolGetAttributes(pool: CVPixelBufferPoolRef) -> CFDictionaryRef;
    pub fn CVPixelBufferPoolGetPixelBufferAttributes(pool: CVPixelBufferPoolRef)
        -> CFDictionaryRef;
    pub fn CVPixelBufferPoolCreatePixelBuffer(
        allocator: CFAllocatorRef,
        pixelBufferPool: CVPixelBufferPoolRef,
        pixelBufferOut: *mut CVPixelBufferRef,
    ) -> CVReturn;
    pub fn CVPixelBufferPoolCreatePixelBufferWithAuxAttributes(
        allocator: CFAllocatorRef,
        pixelBufferPool: CVPixelBufferPoolRef,
        auxAttributes: CFDictionaryRef,
        pixelBufferOut: *mut CVPixelBufferRef,
    ) -> CVReturn;
    pub fn CVPixelBufferPoolFlush(pool: CVPixelBufferPoolRef, options: CVPixelBufferPoolFlushFlags);
}
