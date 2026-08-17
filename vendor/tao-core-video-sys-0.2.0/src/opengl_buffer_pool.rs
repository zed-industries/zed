use core_foundation_sys::{
    base::{CFAllocatorRef, CFTypeID},
    dictionary::CFDictionaryRef,
    string::CFStringRef,
};

use crate::{image_buffer::CVImageBufferRef, opengl_buffer::CVOpenGLBufferRef, return_::CVReturn};

pub type CVOpenGLBufferPoolRef = CVImageBufferRef;

extern "C" {
    pub static kCVOpenGLBufferPoolMinimumBufferCountKey: CFStringRef;
    pub static kCVOpenGLBufferPoolMaximumBufferAgeKey: CFStringRef;

    pub fn CVOpenGLBufferPoolGetTypeID() -> CFTypeID;
    pub fn CVOpenGLBufferPoolRetain(
        openGLBufferPool: CVOpenGLBufferPoolRef,
    ) -> CVOpenGLBufferPoolRef;
    pub fn CVOpenGLBufferPoolRelease(openGLBufferPool: CVOpenGLBufferPoolRef);
    pub fn CVOpenGLBufferPoolCreate(
        allocator: CFAllocatorRef,
        poolAttributes: CFDictionaryRef,
        openGLBufferAttributes: CFDictionaryRef,
        poolOut: *mut CVOpenGLBufferPoolRef,
    ) -> CVReturn;
    pub fn CVOpenGLBufferPoolGetAttributes(pool: CVOpenGLBufferPoolRef) -> CFDictionaryRef;
    pub fn CVOpenGLBufferPoolCreateOpenGLBuffer(
        allocator: CFAllocatorRef,
        openGLBufferPool: CVOpenGLBufferPoolRef,
        openGLBufferOut: *mut CVOpenGLBufferRef,
    ) -> CVReturn;
}
