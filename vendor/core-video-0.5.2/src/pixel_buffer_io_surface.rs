use std::ptr::{null, null_mut};

use core_foundation::{
    base::{kCFAllocatorDefault, CFAllocatorRef, CFType, TCFType},
    dictionary::{CFDictionary, CFDictionaryRef},
    string::{CFString, CFStringRef},
};
use io_surface::{IOSurface, IOSurfaceRef};

use crate::{
    pixel_buffer::{CVPixelBuffer, CVPixelBufferRef},
    r#return::{kCVReturnSuccess, CVReturn},
};

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

pub enum CVPixelBufferIOSurfaceKeys {
    OpenGLTextureCompatibility,
    OpenGLFBOCompatibility,
    CoreAnimationCompatibility,
    OpenGLESTextureCompatibility,
    OpenGLESFBOCompatibility,
}

impl From<CVPixelBufferIOSurfaceKeys> for CFStringRef {
    fn from(key: CVPixelBufferIOSurfaceKeys) -> Self {
        unsafe {
            match key {
                CVPixelBufferIOSurfaceKeys::OpenGLTextureCompatibility => kCVPixelBufferIOSurfaceOpenGLTextureCompatibilityKey,
                CVPixelBufferIOSurfaceKeys::OpenGLFBOCompatibility => kCVPixelBufferIOSurfaceOpenGLFBOCompatibilityKey,
                CVPixelBufferIOSurfaceKeys::CoreAnimationCompatibility => kCVPixelBufferIOSurfaceCoreAnimationCompatibilityKey,
                CVPixelBufferIOSurfaceKeys::OpenGLESTextureCompatibility => kCVPixelBufferIOSurfaceOpenGLESTextureCompatibilityKey,
                CVPixelBufferIOSurfaceKeys::OpenGLESFBOCompatibility => kCVPixelBufferIOSurfaceOpenGLESFBOCompatibilityKey,
            }
        }
    }
}

impl From<CVPixelBufferIOSurfaceKeys> for CFString {
    fn from(key: CVPixelBufferIOSurfaceKeys) -> Self {
        unsafe { CFString::wrap_under_get_rule(CFStringRef::from(key)) }
    }
}

impl CVPixelBuffer {
    #[inline]
    pub fn from_io_surface(io_surface: &IOSurface, options: Option<&CFDictionary<CFString, CFType>>) -> Result<CVPixelBuffer, CVReturn> {
        let mut pixel_buffer: CVPixelBufferRef = null_mut();
        let status = unsafe {
            CVPixelBufferCreateWithIOSurface(
                kCFAllocatorDefault,
                io_surface.as_concrete_TypeRef(),
                options.map_or(null(), |options| options.as_concrete_TypeRef()),
                &mut pixel_buffer,
            )
        };
        if status == kCVReturnSuccess {
            Ok(unsafe { TCFType::wrap_under_create_rule(pixel_buffer) })
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn get_io_surface(&self) -> Option<IOSurface> {
        unsafe {
            let surface = CVPixelBufferGetIOSurface(self.as_concrete_TypeRef());
            if surface.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(surface))
            }
        }
    }
}
