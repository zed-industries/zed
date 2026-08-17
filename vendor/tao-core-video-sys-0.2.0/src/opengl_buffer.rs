use crate::{image_buffer::CVImageBufferRef, return_::CVReturn, GLenum, GLint};
use core_foundation_sys::{
    base::{CFAllocatorRef, CFTypeID},
    dictionary::CFDictionaryRef,
    string::CFStringRef,
};
use libc::size_t;

pub type CVOpenGLBufferRef = CVImageBufferRef;
// https://developer.apple.com/documentation/appkit/nsopenglcontext/1436158-cglcontextobj?language=objc
pub type CGLContextObj = CVImageBufferRef;
// https://developer.apple.com/documentation/appkit/nsopenglpixelformat/1436148-cglpixelformatobj
pub type CGLPixelFormatObj = CVImageBufferRef;

extern "C" {
    pub static kCVOpenGLBufferWidth: CFStringRef;
    pub static kCVOpenGLBufferHeight: CFStringRef;
    pub static kCVOpenGLBufferTarget: CFStringRef;
    pub static kCVOpenGLBufferInternalFormat: CFStringRef;
    pub static kCVOpenGLBufferMaximumMipmapLevel: CFStringRef;

    pub fn CVOpenGLBufferGetTypeID() -> CFTypeID;
    pub fn CVOpenGLBufferRetain(buffer: CVOpenGLBufferRef) -> CVOpenGLBufferRef;
    pub fn CVOpenGLBufferRelease(buffer: CVOpenGLBufferRef);
    pub fn CVOpenGLBufferCreate(
        allocator: CFAllocatorRef,
        width: size_t,
        height: size_t,
        attributes: CFDictionaryRef,
        bufferOut: *mut CVOpenGLBufferRef,
    ) -> CVReturn;
    pub fn CVOpenGLBufferGetAttributes(openGLBuffer: CVOpenGLBufferRef) -> CFDictionaryRef;
    pub fn CVOpenGLBufferAttach(
        openGLBuffer: CVOpenGLBufferRef,
        cglContext: CGLContextObj,
        face: GLenum,
        level: GLint,
        screen: GLint,
    ) -> CVReturn;
}
