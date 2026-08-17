use core_foundation_sys::{base::CFAllocatorRef, dictionary::CFDictionaryRef, string::CFStringRef};

use crate::{pixel_buffer::CVPixelBufferRef, return_::CVReturn};

// https://developer.apple.com/documentation/iosurface/iosurfaceref?language=objc
#[derive(Debug, Copy, Clone)]
pub enum _IOSurface {}
pub type IOSurfaceRef = *mut _IOSurface;

extern "C" {
    pub static kCVPixelBufferIOSurfaceOpenGLTextureCompatibilityKey: CFStringRef;
    pub static kCVPixelBufferIOSurfaceOpenGLFBOCompatibilityKey: CFStringRef;
    pub static kCVPixelBufferIOSurfaceCoreAnimationCompatibilityKey: CFStringRef;

    pub static kCVPixelBufferIOSurfaceOpenGLESTextureCompatibilityKey: CFStringRef;
    pub static kCVPixelBufferIOSurfaceOpenGLESFBOCompatibilityKey: CFStringRef;

    pub fn CVPixelBufferGetIOSurface(pixelBuffer: CVPixelBufferRef) -> IOSurfaceRef;
    pub fn CVPixelBufferCreateWithIOSurface(
        allocator: CFAllocatorRef,
        surface: IOSurfaceRef,
        pixelBufferAttributes: CFDictionaryRef,
        pixelBufferOut: *mut CVPixelBufferRef,
    ) -> CVReturn;
}
