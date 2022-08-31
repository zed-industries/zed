#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

mod bindings;

use core_foundation::{
    base::{CFTypeID, TCFType},
    declare_TCFType, impl_CFTypeDescription, impl_TCFType,
};
use std::ffi::c_void;

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

    use super::*;
    pub use crate::bindings::kCVPixelFormatType_32BGRA;
    use crate::bindings::{kCVReturnSuccess, CVReturn, OSType};
    use anyhow::{anyhow, Result};
    use core_foundation::{
        base::kCFAllocatorDefault, dictionary::CFDictionaryRef, mach_port::CFAllocatorRef,
    };
    use foreign_types::ForeignTypeRef;
    use io_surface::{IOSurface, IOSurfaceRef};
    use metal::{MTLDevice, MTLPixelFormat};
    use std::ptr;

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
        pub fn new(metal_device: *mut MTLDevice) -> Result<Self> {
            unsafe {
                let mut this = ptr::null();
                let result = CVMetalTextureCacheCreate(
                    kCFAllocatorDefault,
                    ptr::null_mut(),
                    metal_device,
                    ptr::null_mut(),
                    &mut this,
                );
                if result == kCVReturnSuccess {
                    Ok(CVMetalTextureCache::wrap_under_create_rule(this))
                } else {
                    Err(anyhow!("could not create texture cache, code: {}", result))
                }
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
        ) -> Result<CVMetalTexture> {
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
                if result == kCVReturnSuccess {
                    Ok(CVMetalTexture::wrap_under_create_rule(this))
                } else {
                    Err(anyhow!("could not create texture, code: {}", result))
                }
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
        ) -> CVReturn;
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
        ) -> CVReturn;
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

pub mod core_media {
    #![allow(non_snake_case)]

    pub use crate::bindings::{
        kCMTimeInvalid, kCMVideoCodecType_H264, CMItemIndex, CMSampleTimingInfo, CMTime,
        CMTimeMake, CMVideoCodecType,
    };
    use crate::core_video::{CVImageBuffer, CVImageBufferRef};
    use anyhow::{anyhow, Result};
    use core_foundation::{
        array::{CFArray, CFArrayRef},
        base::{CFTypeID, OSStatus, TCFType},
        declare_TCFType,
        dictionary::CFDictionary,
        impl_CFTypeDescription, impl_TCFType,
        string::CFString,
    };
    use std::ffi::c_void;

    #[repr(C)]
    pub struct __CMSampleBuffer(c_void);
    // The ref type must be a pointer to the underlying struct.
    pub type CMSampleBufferRef = *const __CMSampleBuffer;

    declare_TCFType!(CMSampleBuffer, CMSampleBufferRef);
    impl_TCFType!(CMSampleBuffer, CMSampleBufferRef, CMSampleBufferGetTypeID);
    impl_CFTypeDescription!(CMSampleBuffer);

    impl CMSampleBuffer {
        pub fn attachments(&self) -> Vec<CFDictionary<CFString>> {
            unsafe {
                let attachments =
                    CMSampleBufferGetSampleAttachmentsArray(self.as_concrete_TypeRef(), true);
                CFArray::<CFDictionary>::wrap_under_get_rule(attachments)
                    .into_iter()
                    .map(|attachments| {
                        CFDictionary::wrap_under_get_rule(attachments.as_concrete_TypeRef())
                    })
                    .collect()
            }
        }

        pub fn image_buffer(&self) -> CVImageBuffer {
            unsafe {
                CVImageBuffer::wrap_under_get_rule(CMSampleBufferGetImageBuffer(
                    self.as_concrete_TypeRef(),
                ))
            }
        }

        pub fn sample_timing_info(&self, index: usize) -> Result<CMSampleTimingInfo> {
            unsafe {
                let mut timing_info = CMSampleTimingInfo {
                    duration: kCMTimeInvalid,
                    presentationTimeStamp: kCMTimeInvalid,
                    decodeTimeStamp: kCMTimeInvalid,
                };
                let result = CMSampleBufferGetSampleTimingInfo(
                    self.as_concrete_TypeRef(),
                    index as CMItemIndex,
                    &mut timing_info,
                );

                if result == 0 {
                    Ok(timing_info)
                } else {
                    Err(anyhow!("error getting sample timing info, code {}", result))
                }
            }
        }
    }

    #[link(name = "CoreMedia", kind = "framework")]
    extern "C" {
        fn CMSampleBufferGetTypeID() -> CFTypeID;
        fn CMSampleBufferGetSampleAttachmentsArray(
            buffer: CMSampleBufferRef,
            create_if_necessary: bool,
        ) -> CFArrayRef;
        fn CMSampleBufferGetImageBuffer(buffer: CMSampleBufferRef) -> CVImageBufferRef;
        fn CMSampleBufferGetSampleTimingInfo(
            buffer: CMSampleBufferRef,
            index: CMItemIndex,
            timing_info_out: *mut CMSampleTimingInfo,
        ) -> OSStatus;
    }
}

pub mod video_toolbox {
    #![allow(non_snake_case)]

    use super::*;
    use crate::{
        core_media::{CMSampleBufferRef, CMTime, CMVideoCodecType},
        core_video::CVImageBufferRef,
    };
    use anyhow::{anyhow, Result};
    use bindings::VTEncodeInfoFlags;
    use core_foundation::{
        base::OSStatus,
        dictionary::{CFDictionary, CFDictionaryRef, CFMutableDictionary},
        mach_port::CFAllocatorRef,
    };
    use std::ptr;

    #[repr(C)]
    pub struct __VTCompressionSession(c_void);
    // The ref type must be a pointer to the underlying struct.
    pub type VTCompressionSessionRef = *const __VTCompressionSession;

    declare_TCFType!(VTCompressionSession, VTCompressionSessionRef);
    impl_TCFType!(
        VTCompressionSession,
        VTCompressionSessionRef,
        VTCompressionSessionGetTypeID
    );
    impl_CFTypeDescription!(VTCompressionSession);

    impl VTCompressionSession {
        pub fn new(
            width: usize,
            height: usize,
            codec: CMVideoCodecType,
            callback: VTCompressionOutputCallback,
            callback_data: *const c_void,
        ) -> Result<Self> {
            unsafe {
                let mut this = ptr::null();
                let result = VTCompressionSessionCreate(
                    ptr::null(),
                    width as i32,
                    height as i32,
                    codec,
                    ptr::null(),
                    ptr::null(),
                    ptr::null(),
                    Some(Self::output),
                    callback_data,
                    &mut this,
                );

                if result == 0 {
                    Ok(Self::wrap_under_create_rule(this))
                } else {
                    Err(anyhow!(
                        "error creating compression session, code {}",
                        result
                    ))
                }
            }
        }

        extern "C" fn output(
            outputCallbackRefCon: *mut c_void,
            sourceFrameRefCon: *mut c_void,
            status: OSStatus,
            infoFlags: VTEncodeInfoFlags,
            sampleBuffer: CMSampleBufferRef,
        ) {
            println!("YO!");
        }

        pub fn encode_frame(
            &self,
            buffer: CVImageBufferRef,
            presentation_timestamp: CMTime,
            duration: CMTime,
        ) -> Result<()> {
            unsafe {
                let result = VTCompressionSessionEncodeFrame(
                    self.as_concrete_TypeRef(),
                    buffer,
                    presentation_timestamp,
                    duration,
                    ptr::null(),
                    ptr::null(),
                    ptr::null_mut(),
                );
                if result == 0 {
                    Ok(())
                } else {
                    Err(anyhow!("error encoding frame, code {}", result))
                }
            }
        }
    }

    type VTCompressionOutputCallback = Option<
        unsafe extern "C" fn(
            outputCallbackRefCon: *mut c_void,
            sourceFrameRefCon: *mut c_void,
            status: OSStatus,
            infoFlags: VTEncodeInfoFlags,
            sampleBuffer: CMSampleBufferRef,
        ),
    >;

    #[link(name = "VideoToolbox", kind = "framework")]
    extern "C" {
        fn VTCompressionSessionGetTypeID() -> CFTypeID;
        fn VTCompressionSessionCreate(
            allocator: CFAllocatorRef,
            width: i32,
            height: i32,
            codec_type: CMVideoCodecType,
            encoder_specification: CFDictionaryRef,
            source_image_buffer_attributes: CFDictionaryRef,
            compressed_data_allocator: CFAllocatorRef,
            output_callback: VTCompressionOutputCallback,
            output_callback_ref_con: *const c_void,
            compression_session_out: *mut VTCompressionSessionRef,
        ) -> OSStatus;
        fn VTCompressionSessionEncodeFrame(
            session: VTCompressionSessionRef,
            image_buffer: CVImageBufferRef,
            presentation_timestamp: CMTime,
            duration: CMTime,
            frame_properties: CFDictionaryRef,
            source_frame_ref_con: *const c_void,
            output_flags: *mut VTEncodeInfoFlags,
        ) -> OSStatus;
    }
}
