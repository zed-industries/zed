use std::ptr::{null, null_mut};

use core_foundation::{
    base::{kCFAllocatorDefault, CFAllocatorRef, CFType, CFTypeID, TCFType},
    dictionary::{CFDictionary, CFDictionaryRef},
    impl_CFTypeDescription, impl_TCFType,
    string::{CFString, CFStringRef},
};
use libc::c_void;

use crate::{
    base::CVOptionFlags,
    image_buffer::{CVImageBuffer, CVImageBufferRef},
    opengl_texture::{CVOpenGLTexture, CVOpenGLTextureRef},
    r#return::{kCVReturnSuccess, CVReturn},
    CGLContextObj, CGLPixelFormatObj,
};

#[repr(C)]
pub struct __CVOpenGLTextureCache(c_void);

pub type CVOpenGLTextureCacheRef = *mut __CVOpenGLTextureCache;

extern "C" {
    pub static kCVOpenGLTextureCacheChromaSamplingModeKey: CFStringRef;
    pub static kCVOpenGLTextureCacheChromaSamplingModeAutomatic: CFStringRef;
    pub static kCVOpenGLTextureCacheChromaSamplingModeHighestQuality: CFStringRef;
    pub static kCVOpenGLTextureCacheChromaSamplingModeBestPerformance: CFStringRef;

    pub fn CVOpenGLTextureCacheGetTypeID() -> CFTypeID;
    pub fn CVOpenGLTextureCacheRetain(textureCache: CVOpenGLTextureCacheRef) -> CVOpenGLTextureCacheRef;
    pub fn CVOpenGLTextureCacheRelease(textureCache: CVOpenGLTextureCacheRef);
    pub fn CVOpenGLTextureCacheCreate(
        allocator: CFAllocatorRef,
        cacheAttributes: CFDictionaryRef,
        cglContext: CGLContextObj,
        cglPixelFormat: CGLPixelFormatObj,
        textureAttributes: CFDictionaryRef,
        cacheOut: *mut CVOpenGLTextureCacheRef,
    ) -> CVReturn;
    pub fn CVOpenGLTextureCacheCreateTextureFromImage(
        allocator: CFAllocatorRef,
        textureCache: CVOpenGLTextureCacheRef,
        sourceImage: CVImageBufferRef,
        attributes: CFDictionaryRef,
        textureOut: *mut CVOpenGLTextureRef,
    ) -> CVReturn;
    pub fn CVOpenGLTextureCacheFlush(textureCache: CVOpenGLTextureCacheRef, options: CVOptionFlags);
}

pub enum CVOpenGLTextureCacheKeys {
    ChromaSamplingMode,
}

impl From<CVOpenGLTextureCacheKeys> for CFStringRef {
    fn from(key: CVOpenGLTextureCacheKeys) -> Self {
        unsafe {
            match key {
                CVOpenGLTextureCacheKeys::ChromaSamplingMode => kCVOpenGLTextureCacheChromaSamplingModeKey,
            }
        }
    }
}

impl From<CVOpenGLTextureCacheKeys> for CFString {
    fn from(key: CVOpenGLTextureCacheKeys) -> Self {
        unsafe { CFString::wrap_under_get_rule(CFStringRef::from(key)) }
    }
}

pub enum CVOpenGLTextureCacheChromaSamplingMode {
    Automatic,
    HighestQuality,
    BestPerformance,
}

impl From<CVOpenGLTextureCacheChromaSamplingMode> for CFStringRef {
    fn from(mode: CVOpenGLTextureCacheChromaSamplingMode) -> Self {
        unsafe {
            match mode {
                CVOpenGLTextureCacheChromaSamplingMode::Automatic => kCVOpenGLTextureCacheChromaSamplingModeAutomatic,
                CVOpenGLTextureCacheChromaSamplingMode::HighestQuality => kCVOpenGLTextureCacheChromaSamplingModeHighestQuality,
                CVOpenGLTextureCacheChromaSamplingMode::BestPerformance => kCVOpenGLTextureCacheChromaSamplingModeBestPerformance,
            }
        }
    }
}

impl From<CVOpenGLTextureCacheChromaSamplingMode> for CFString {
    fn from(mode: CVOpenGLTextureCacheChromaSamplingMode) -> Self {
        unsafe { CFString::wrap_under_get_rule(CFStringRef::from(mode)) }
    }
}

pub struct CVOpenGLTextureCache(CVOpenGLTextureCacheRef);

impl Drop for CVOpenGLTextureCache {
    fn drop(&mut self) {
        unsafe { CVOpenGLTextureCacheRelease(self.0) }
    }
}

impl_TCFType!(CVOpenGLTextureCache, CVOpenGLTextureCacheRef, CVOpenGLTextureCacheGetTypeID);
impl_CFTypeDescription!(CVOpenGLTextureCache);

impl CVOpenGLTextureCache {
    #[inline]
    pub unsafe fn new(
        cache_attributes: Option<&CFDictionary<CFString, CFString>>,
        cgl_context: CGLContextObj,
        cgl_pixel_format: CGLPixelFormatObj,
        texture_attributes: Option<&CFDictionary<CFString, CFType>>,
    ) -> Result<CVOpenGLTextureCache, CVReturn> {
        let mut cache: CVOpenGLTextureCacheRef = null_mut();
        let status = unsafe {
            CVOpenGLTextureCacheCreate(
                kCFAllocatorDefault,
                cache_attributes.map_or(null(), |attrs| attrs.as_concrete_TypeRef()),
                cgl_context,
                cgl_pixel_format,
                texture_attributes.map_or(null(), |attrs| attrs.as_concrete_TypeRef()),
                &mut cache,
            )
        };
        if status == kCVReturnSuccess {
            Ok(unsafe { TCFType::wrap_under_create_rule(cache) })
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn create_texture_from_image(
        &self,
        source_image: &CVImageBuffer,
        attributes: Option<&CFDictionary<CFString, CFType>>,
    ) -> Result<CVOpenGLTexture, CVReturn> {
        let mut texture: CVOpenGLTextureRef = null_mut();
        let status = unsafe {
            CVOpenGLTextureCacheCreateTextureFromImage(
                kCFAllocatorDefault,
                self.as_concrete_TypeRef(),
                source_image.as_concrete_TypeRef(),
                attributes.map_or(null(), |attrs| attrs.as_concrete_TypeRef()),
                &mut texture,
            )
        };
        if status == kCVReturnSuccess {
            Ok(unsafe { TCFType::wrap_under_create_rule(texture) })
        } else {
            Err(status)
        }
    }

    #[inline]
    pub fn flush(&self, options: CVOptionFlags) {
        unsafe { CVOpenGLTextureCacheFlush(self.as_concrete_TypeRef(), options) }
    }
}
