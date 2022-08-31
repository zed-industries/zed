#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

mod bindings;

use core_foundation::{
    base::{CFTypeID, TCFType},
    declare_TCFType, impl_CFTypeDescription, impl_TCFType,
};
use objc::runtime;
use std::ffi::c_void;

pub type id = *mut runtime::Object;

pub mod io_surface {
    use super::*;

    #[repr(C)]
    pub struct __IOSurface(c_void);
    // The ref type must be a pointer to the underlying struct.
    pub type IOSurfaceRef = *const __IOSurface;

    declare_TCFType!(IOSurface, IOSurfaceRef);
    impl_TCFType!(IOSurface, IOSurfaceRef, IOSurfaceGetTypeID);
    impl_CFTypeDescription!(IOSurface);

    #[link(name = "IOSurface", kind = "framework")]
    extern "C" {
        fn IOSurfaceGetTypeID() -> CFTypeID;
    }
}

pub mod core_video {
    #![allow(non_snake_case)]

    use std::ptr;

    use super::*;
    pub use crate::bindings::*;
    use core_foundation::{
        base::kCFAllocatorDefault, dictionary::CFDictionaryRef, mach_port::CFAllocatorRef,
    };
    use foreign_types::ForeignTypeRef;
    use io_surface::{IOSurface, IOSurfaceRef};
    use metal::{MTLDevice, MTLPixelFormat, MTLTexture};

    #[repr(C)]
    pub struct __CVImageBuffer(c_void);
    // The ref type must be a pointer to the underlying struct.
    pub type CVImageBufferRef = *const __CVImageBuffer;

    declare_TCFType!(CVImageBuffer, CVImageBufferRef);
    impl_TCFType!(CVImageBuffer, CVImageBufferRef, CVImageBufferGetTypeID);
    impl_CFTypeDescription!(CVImageBuffer);

    impl CVImageBuffer {
        pub fn io_surface(&self) -> IOSurface {
            unsafe {
                IOSurface::wrap_under_get_rule(CVPixelBufferGetIOSurface(
                    self.as_concrete_TypeRef(),
                ))
            }
        }

        pub fn width(&self) -> usize {
            unsafe { CVPixelBufferGetWidth(self.as_concrete_TypeRef()) }
        }

        pub fn height(&self) -> usize {
            unsafe { CVPixelBufferGetHeight(self.as_concrete_TypeRef()) }
        }

        pub fn pixel_format_type(&self) -> OSType {
            unsafe { CVPixelBufferGetPixelFormatType(self.as_concrete_TypeRef()) }
        }
    }

    #[link(name = "CoreVideo", kind = "framework")]
    extern "C" {
        fn CVImageBufferGetTypeID() -> CFTypeID;
        fn CVPixelBufferGetIOSurface(buffer: CVImageBufferRef) -> IOSurfaceRef;
        fn CVPixelBufferGetWidth(buffer: CVImageBufferRef) -> usize;
        fn CVPixelBufferGetHeight(buffer: CVImageBufferRef) -> usize;
        fn CVPixelBufferGetPixelFormatType(buffer: CVImageBufferRef) -> OSType;
    }

    #[repr(C)]
    pub struct __CVMetalTextureCache(c_void);
    pub type CVMetalTextureCacheRef = *const __CVMetalTextureCache;

    declare_TCFType!(CVMetalTextureCache, CVMetalTextureCacheRef);
    impl_TCFType!(
        CVMetalTextureCache,
        CVMetalTextureCacheRef,
        CVMetalTextureCacheGetTypeID
    );
    impl_CFTypeDescription!(CVMetalTextureCache);

    impl CVMetalTextureCache {
        pub fn new(metal_device: *mut MTLDevice) -> Self {
            unsafe {
                let mut this = ptr::null();
                let result = CVMetalTextureCacheCreate(
                    kCFAllocatorDefault,
                    ptr::null_mut(),
                    metal_device,
                    ptr::null_mut(),
                    &mut this,
                );
                // TODO: Check result
                CVMetalTextureCache::wrap_under_create_rule(this)
            }
        }

        pub fn create_texture_from_image(
            &self,
            source: CVImageBufferRef,
            texture_attributes: CFDictionaryRef,
            pixel_format: MTLPixelFormat,
            width: usize,
            height: usize,
            plane_index: usize,
        ) -> CVMetalTexture {
            unsafe {
                let mut this = ptr::null();
                let result = CVMetalTextureCacheCreateTextureFromImage(
                    kCFAllocatorDefault,
                    self.as_concrete_TypeRef(),
                    source,
                    texture_attributes,
                    pixel_format,
                    width,
                    height,
                    plane_index,
                    &mut this,
                );
                // TODO: Check result
                CVMetalTexture::wrap_under_create_rule(this)
            }
        }
    }

    #[link(name = "CoreVideo", kind = "framework")]
    extern "C" {
        fn CVMetalTextureCacheGetTypeID() -> CFTypeID;
        fn CVMetalTextureCacheCreate(
            allocator: CFAllocatorRef,
            cache_attributes: CFDictionaryRef,
            metal_device: *const MTLDevice,
            texture_attributes: CFDictionaryRef,
            cache_out: *mut CVMetalTextureCacheRef,
        ) -> i32; // TODO: This should be a CVReturn enum
        fn CVMetalTextureCacheCreateTextureFromImage(
            allocator: CFAllocatorRef,
            texture_cache: CVMetalTextureCacheRef,
            source_image: CVImageBufferRef,
            texture_attributes: CFDictionaryRef,
            pixel_format: MTLPixelFormat,
            width: usize,
            height: usize,
            plane_index: usize,
            texture_out: *mut CVMetalTextureRef,
        ) -> i32;
    }

    #[repr(C)]
    pub struct __CVMetalTexture(c_void);
    pub type CVMetalTextureRef = *const __CVMetalTexture;

    declare_TCFType!(CVMetalTexture, CVMetalTextureRef);
    impl_TCFType!(CVMetalTexture, CVMetalTextureRef, CVMetalTextureGetTypeID);
    impl_CFTypeDescription!(CVMetalTexture);

    impl CVMetalTexture {
        pub fn as_texture_ref(&self) -> &metal::TextureRef {
            unsafe {
                let texture = CVMetalTextureGetTexture(self.as_concrete_TypeRef());
                &metal::TextureRef::from_ptr(texture as *mut _)
            }
        }
    }

    #[link(name = "CoreVideo", kind = "framework")]
    extern "C" {
        fn CVMetalTextureGetTypeID() -> CFTypeID;
        fn CVMetalTextureGetTexture(texture: CVMetalTextureRef) -> *mut c_void;
    }
}
