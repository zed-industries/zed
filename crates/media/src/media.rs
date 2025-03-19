#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

mod bindings;

#[cfg(target_os = "macos")]
pub mod core_media {
    #![allow(non_snake_case)]

    pub use crate::bindings::{
        kCMSampleAttachmentKey_NotSync, kCMTimeInvalid, kCMVideoCodecType_H264, CMItemIndex,
        CMSampleTimingInfo, CMTime, CMTimeMake, CMVideoCodecType,
    };
    use anyhow::{anyhow, Result};
    use core_foundation::{
        array::{CFArray, CFArrayRef},
        base::{CFTypeID, OSStatus, TCFType},
        declare_TCFType,
        dictionary::CFDictionary,
        impl_CFTypeDescription, impl_TCFType,
        string::CFString,
    };
    use core_video::image_buffer::{CVImageBuffer, CVImageBufferRef};
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

        pub fn image_buffer(&self) -> Option<CVImageBuffer> {
            unsafe {
                let ptr = CMSampleBufferGetImageBuffer(self.as_concrete_TypeRef());
                if ptr.is_null() {
                    None
                } else {
                    Some(CVImageBuffer::wrap_under_get_rule(ptr))
                }
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
