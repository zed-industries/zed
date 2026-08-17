use std::ptr::{null, null_mut};

use core_foundation::{
    base::{kCFAllocatorDefault, CFAllocatorRef, CFType, CFTypeID, TCFType},
    declare_TCFType,
    dictionary::{CFDictionary, CFDictionaryRef},
    impl_CFTypeDescription, impl_TCFType,
    string::{CFString, CFStringRef},
};
use libc::{c_void, size_t};

use crate::{
    base::CVOptionFlags,
    image_buffer::CVImageBufferRef,
    metal_texture::{CVMetalTexture, CVMetalTextureRef},
    r#return::{kCVReturnSuccess, CVReturn},
};

#[repr(C)]
pub struct __CVMetalTextureCache(c_void);

pub type CVMetalTextureCacheRef = *mut __CVMetalTextureCache;

extern "C" {
    pub static kCVMetalTextureCacheMaximumTextureAgeKey: CFStringRef;

    pub fn CVMetalTextureCacheGetTypeID() -> CFTypeID;
    pub fn CVMetalTextureCacheCreate(
        allocator: CFAllocatorRef,
        cacheAttributes: CFDictionaryRef,
        metalDevice: metal::Device,
        textureAttributes: CFDictionaryRef,
        cacheOut: *mut CVMetalTextureCacheRef,
    ) -> CVReturn;
    pub fn CVMetalTextureCacheCreateTextureFromImage(
        allocator: CFAllocatorRef,
        textureCache: CVMetalTextureCacheRef,
        sourceImage: CVImageBufferRef,
        textureAttributes: CFDictionaryRef,
        pixelFormat: metal::MTLPixelFormat,
        width: size_t,
        height: size_t,
        planeIndex: size_t,
        textureOut: *mut CVMetalTextureRef,
    ) -> CVReturn;
    pub fn CVMetalTextureCacheFlush(textureCache: CVMetalTextureCacheRef, options: CVOptionFlags);
}

pub enum CVMetalTextureCacheKeys {
    MaximumTextureAge,
}

impl From<CVMetalTextureCacheKeys> for CFStringRef {
    fn from(key: CVMetalTextureCacheKeys) -> Self {
        unsafe {
            match key {
                CVMetalTextureCacheKeys::MaximumTextureAge => kCVMetalTextureCacheMaximumTextureAgeKey,
            }
        }
    }
}

impl From<CVMetalTextureCacheKeys> for CFString {
    fn from(key: CVMetalTextureCacheKeys) -> Self {
        unsafe { CFString::wrap_under_get_rule(CFStringRef::from(key)) }
    }
}

declare_TCFType!(CVMetalTextureCache, CVMetalTextureCacheRef);
impl_TCFType!(CVMetalTextureCache, CVMetalTextureCacheRef, CVMetalTextureCacheGetTypeID);
impl_CFTypeDescription!(CVMetalTextureCache);

impl CVMetalTextureCache {
    #[inline]
    pub fn new(
        cache_attributes: Option<&CFDictionary<CFString, CFType>>,
        metal_device: metal::Device,
        texture_attributes: Option<&CFDictionary<CFString, CFType>>,
    ) -> Result<CVMetalTextureCache, CVReturn> {
        let mut cache: CVMetalTextureCacheRef = null_mut();
        let status = unsafe {
            CVMetalTextureCacheCreate(
                kCFAllocatorDefault,
                cache_attributes.map_or(null(), |attrs| attrs.as_concrete_TypeRef()),
                metal_device,
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
        source_image: CVImageBufferRef,
        texture_attributes: Option<&CFDictionary<CFString, CFType>>,
        pixel_format: metal::MTLPixelFormat,
        width: size_t,
        height: size_t,
        plane_index: size_t,
    ) -> Result<CVMetalTexture, CVReturn> {
        let mut texture: CVMetalTextureRef = null_mut();
        let status = unsafe {
            CVMetalTextureCacheCreateTextureFromImage(
                kCFAllocatorDefault,
                self.as_concrete_TypeRef(),
                source_image,
                texture_attributes.map_or(null(), |attrs| attrs.as_concrete_TypeRef()),
                pixel_format,
                width,
                height,
                plane_index,
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
        unsafe { CVMetalTextureCacheFlush(self.as_concrete_TypeRef(), options) }
    }
}
