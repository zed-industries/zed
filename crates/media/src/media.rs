#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

mod bindings;

#[cfg(target_os = "macos")]
use core_foundation::{
    base::{CFTypeID, TCFType},
    declare_TCFType, impl_CFTypeDescription, impl_TCFType,
};
#[cfg(target_os = "macos")]
use std::ffi::c_void;

#[cfg(target_os = "macos")]
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

#[cfg(target_os = "macos")]
pub mod core_video {
    #![allow(non_snake_case)]

    use super::*;
    pub use crate::bindings::{
        kCVPixelFormatType_420YpCbCr8BiPlanarFullRange,
        kCVPixelFormatType_420YpCbCr8BiPlanarVideoRange, kCVPixelFormatType_420YpCbCr8Planar,
    };
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

        pub fn plane_width(&self, plane: usize) -> usize {
            unsafe { CVPixelBufferGetWidthOfPlane(self.as_concrete_TypeRef(), plane) }
        }

        pub fn plane_height(&self, plane: usize) -> usize {
            unsafe { CVPixelBufferGetHeightOfPlane(self.as_concrete_TypeRef(), plane) }
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
        fn CVPixelBufferGetWidthOfPlane(buffer: CVImageBufferRef, plane: usize) -> usize;
        fn CVPixelBufferGetHeightOfPlane(buffer: CVImageBufferRef, plane: usize) -> usize;
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
        /// # Safety
        ///
        /// metal_device must be valid according to CVMetalTextureCacheCreate
        pub unsafe fn new(metal_device: *mut MTLDevice) -> Result<Self> {
            let mut this = ptr::null();
            let result = CVMetalTextureCacheCreate(
                kCFAllocatorDefault,
                ptr::null(),
                metal_device,
                ptr::null(),
                &mut this,
            );
            if result == kCVReturnSuccess {
                Ok(CVMetalTextureCache::wrap_under_create_rule(this))
            } else {
                Err(anyhow!("could not create texture cache, code: {}", result))
            }
        }

        /// # Safety
        ///
        /// The arguments to this function must be valid according to CVMetalTextureCacheCreateTextureFromImage
        pub unsafe fn create_texture_from_image(
            &self,
            source: CVImageBufferRef,
            texture_attributes: CFDictionaryRef,
            pixel_format: MTLPixelFormat,
            width: usize,
            height: usize,
            plane_index: usize,
        ) -> Result<CVMetalTexture> {
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
                metal::TextureRef::from_ptr(texture as *mut _)
            }
        }
    }

    #[link(name = "CoreVideo", kind = "framework")]
    extern "C" {
        fn CVMetalTextureGetTypeID() -> CFTypeID;
        fn CVMetalTextureGetTexture(texture: CVMetalTextureRef) -> *mut c_void;
    }
}

#[cfg(target_os = "macos")]
pub mod core_media {
    #![allow(non_snake_case)]

    pub use crate::bindings::{
        kCMSampleAttachmentKey_NotSync, kCMTimeInvalid, kCMVideoCodecType_H264, CMItemIndex,
        CMSampleTimingInfo, CMTime, CMTimeMake, CMVideoCodecType,
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
    use std::{ffi::c_void, ptr};

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

        pub fn format_description(&self) -> CMFormatDescription {
            unsafe {
                CMFormatDescription::wrap_under_get_rule(CMSampleBufferGetFormatDescription(
                    self.as_concrete_TypeRef(),
                ))
            }
        }

        pub fn data(&self) -> CMBlockBuffer {
            unsafe {
                CMBlockBuffer::wrap_under_get_rule(CMSampleBufferGetDataBuffer(
                    self.as_concrete_TypeRef(),
                ))
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
        fn CMSampleBufferGetFormatDescription(buffer: CMSampleBufferRef) -> CMFormatDescriptionRef;
        fn CMSampleBufferGetDataBuffer(sample_buffer: CMSampleBufferRef) -> CMBlockBufferRef;
    }

    #[repr(C)]
    pub struct __CMFormatDescription(c_void);
    pub type CMFormatDescriptionRef = *const __CMFormatDescription;

    declare_TCFType!(CMFormatDescription, CMFormatDescriptionRef);
    impl_TCFType!(
        CMFormatDescription,
        CMFormatDescriptionRef,
        CMFormatDescriptionGetTypeID
    );
    impl_CFTypeDescription!(CMFormatDescription);

    impl CMFormatDescription {
        pub fn h264_parameter_set_count(&self) -> usize {
            unsafe {
                let mut count = 0;
                let result = CMVideoFormatDescriptionGetH264ParameterSetAtIndex(
                    self.as_concrete_TypeRef(),
                    0,
                    ptr::null_mut(),
                    ptr::null_mut(),
                    &mut count,
                    ptr::null_mut(),
                );
                assert_eq!(result, 0);
                count
            }
        }

        pub fn h264_parameter_set_at_index(&self, index: usize) -> Result<&[u8]> {
            unsafe {
                let mut bytes = ptr::null();
                let mut len = 0;
                let result = CMVideoFormatDescriptionGetH264ParameterSetAtIndex(
                    self.as_concrete_TypeRef(),
                    index,
                    &mut bytes,
                    &mut len,
                    ptr::null_mut(),
                    ptr::null_mut(),
                );
                if result == 0 {
                    Ok(std::slice::from_raw_parts(bytes, len))
                } else {
                    Err(anyhow!("error getting parameter set, code: {}", result))
                }
            }
        }
    }

    #[link(name = "CoreMedia", kind = "framework")]
    extern "C" {
        fn CMFormatDescriptionGetTypeID() -> CFTypeID;
        fn CMVideoFormatDescriptionGetH264ParameterSetAtIndex(
            video_desc: CMFormatDescriptionRef,
            parameter_set_index: usize,
            parameter_set_pointer_out: *mut *const u8,
            parameter_set_size_out: *mut usize,
            parameter_set_count_out: *mut usize,
            NALUnitHeaderLengthOut: *mut isize,
        ) -> OSStatus;
    }

    #[repr(C)]
    pub struct __CMBlockBuffer(c_void);
    pub type CMBlockBufferRef = *const __CMBlockBuffer;

    declare_TCFType!(CMBlockBuffer, CMBlockBufferRef);
    impl_TCFType!(CMBlockBuffer, CMBlockBufferRef, CMBlockBufferGetTypeID);
    impl_CFTypeDescription!(CMBlockBuffer);

    impl CMBlockBuffer {
        pub fn bytes(&self) -> &[u8] {
            unsafe {
                let mut bytes = ptr::null();
                let mut len = 0;
                let result = CMBlockBufferGetDataPointer(
                    self.as_concrete_TypeRef(),
                    0,
                    &mut 0,
                    &mut len,
                    &mut bytes,
                );
                assert!(result == 0, "could not get block buffer data");
                std::slice::from_raw_parts(bytes, len)
            }
        }
    }

    #[link(name = "CoreMedia", kind = "framework")]
    extern "C" {
        fn CMBlockBufferGetTypeID() -> CFTypeID;
        fn CMBlockBufferGetDataPointer(
            buffer: CMBlockBufferRef,
            offset: usize,
            length_at_offset_out: *mut usize,
            total_length_out: *mut usize,
            data_pointer_out: *mut *const u8,
        ) -> OSStatus;
    }
}

#[cfg(target_os = "macos")]
pub mod video_toolbox {
    #![allow(non_snake_case)]

    use super::*;
    use crate::{
        core_media::{CMSampleBufferRef, CMTime, CMVideoCodecType},
        core_video::CVImageBufferRef,
    };
    use anyhow::{anyhow, Result};
    pub use bindings::VTEncodeInfoFlags;
    use core_foundation::{base::OSStatus, dictionary::CFDictionaryRef, mach_port::CFAllocatorRef};
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
        /// Create a new compression session.
        ///
        /// # Safety
        ///
        /// The callback must be a valid function pointer. and the callback_data must be valid
        /// in whatever terms that callback expects.
        pub unsafe fn new(
            width: usize,
            height: usize,
            codec: CMVideoCodecType,
            callback: VTCompressionOutputCallback,
            callback_data: *const c_void,
        ) -> Result<Self> {
            let mut this = ptr::null();
            let result = VTCompressionSessionCreate(
                ptr::null(),
                width as i32,
                height as i32,
                codec,
                ptr::null(),
                ptr::null(),
                ptr::null(),
                callback,
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

        /// # Safety
        ///
        /// The arguments to this function must be valid according to VTCompressionSessionEncodeFrame
        pub unsafe fn encode_frame(
            &self,
            buffer: CVImageBufferRef,
            presentation_timestamp: CMTime,
            duration: CMTime,
        ) -> Result<()> {
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
