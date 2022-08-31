use anyhow::Result;
use core_foundation::base::{OSStatus, TCFType};
use media::{
    core_media::{CMSampleBufferRef, CMSampleTimingInfo, CMVideoCodecType},
    core_video::CVImageBuffer,
    video_toolbox::{VTCompressionSession, VTEncodeInfoFlags},
};
use std::ffi::c_void;

pub struct CompressionSession<F> {
    session: VTCompressionSession,
    output_callback: Box<F>,
}

impl<F: 'static + Send + FnMut(OSStatus, VTEncodeInfoFlags, CMSampleBufferRef)>
    CompressionSession<F>
{
    pub fn new(width: usize, height: usize, codec: CMVideoCodecType, callback: F) -> Result<Self> {
        let callback = Box::new(callback);
        let session = VTCompressionSession::new(
            width,
            height,
            codec,
            Some(Self::output_callback),
            callback.as_ref() as *const _ as *const c_void,
        )?;
        Ok(Self {
            session,
            output_callback: callback,
        })
    }

    pub fn encode_frame(&self, buffer: &CVImageBuffer, timing: CMSampleTimingInfo) -> Result<()> {
        self.session.encode_frame(
            buffer.as_concrete_TypeRef(),
            timing.presentationTimeStamp,
            timing.duration,
        )
    }

    extern "C" fn output_callback(
        output_callback_ref_con: *mut c_void,
        _: *mut c_void,
        status: OSStatus,
        flags: VTEncodeInfoFlags,
        sample_buffer: CMSampleBufferRef,
    ) {
        let callback = unsafe { &mut *(output_callback_ref_con as *mut F) };
        callback(status, flags, sample_buffer);
    }
}

// unsafe extern "C" fn output(
//     output_callback_ref_con: *mut c_void,
//     source_frame_ref_con: *mut c_void,
//     status: OSStatus,
//     info_flags: VTEncodeInfoFlags,
//     sample_buffer: CMSampleBufferRef,
// ) {
//     if status != 0 {
//         println!("error encoding frame, code: {}", status);
//         return;
//     }
//     let sample_buffer = CMSampleBuffer::wrap_under_get_rule(sample_buffer);

//     let mut is_iframe = false;
//     let attachments = sample_buffer.attachments();
//     if let Some(attachments) = attachments.first() {
//         is_iframe = attachments
//             .find(bindings::kCMSampleAttachmentKey_NotSync as CFStringRef)
//             .map_or(true, |not_sync| {
//                 CFBooleanGetValue(*not_sync as CFBooleanRef)
//             });
//     }

//     const START_CODE: [u8; 4] = [0x00, 0x00, 0x00, 0x01];
//     if is_iframe {
//         let format_description = sample_buffer.format_description();
//         for ix in 0..format_description.h264_parameter_set_count() {
//             let parameter_set = format_description.h264_parameter_set_at_index(ix);
//             stream.extend(START_CODE);
//             stream.extend(parameter_set);
//         }
//     }

//     println!("YO!");
// }

// static void videoFrameFinishedEncoding(void *outputCallbackRefCon,
//                                        void *sourceFrameRefCon,
//                                        OSStatus status,
//                                        VTEncodeInfoFlags infoFlags,
//                                        CMSampleBufferRef sampleBuffer) {
//     // Check if there were any errors encoding
//     if (status != noErr) {
//         NSLog(@"Error encoding video, err=%lld", (int64_t)status);
//         return;
//     }

//     // In this example we will use a NSMutableData object to store the
//     // elementary stream.
//     NSMutableData *elementaryStream = [NSMutableData data];

//     // Find out if the sample buffer contains an I-Frame.
//     // If so we will write the SPS and PPS NAL units to the elementary stream.
//     BOOL isIFrame = NO;
//     CFArrayRef attachmentsArray = CMSampleBufferGetSampleAttachmentsArray(sampleBuffer, 0);
//     if (CFArrayGetCount(attachmentsArray)) {
//         CFBooleanRef notSync;
//         CFDictionaryRef dict = CFArrayGetValueAtIndex(attachmentsArray, 0);
//         BOOL keyExists = CFDictionaryGetValueIfPresent(dict,
//                                                        kCMSampleAttachmentKey_NotSync,
//                                                        (const void **)&notSync);
//         // An I-Frame is a sync frame
//         isIFrame = !keyExists || !CFBooleanGetValue(notSync);
//     }

//     // This is the start code that we will write to
//     // the elementary stream before every NAL unit
//     static const size_t startCodeLength = 4;
//     static const uint8_t startCode[] = {0x00, 0x00, 0x00, 0x01};

//     // Write the SPS and PPS NAL units to the elementary stream before every I-Frame
//     if (isIFrame) {
//         CMFormatDescriptionRef description = CMSampleBufferGetFormatDescription(sampleBuffer);

//         // Find out how many parameter sets there are
//         size_t numberOfParameterSets;
//         CMVideoFormatDescriptionGetH264ParameterSetAtIndex(description,
//                                                            0, NULL, NULL,
//                                                            &numberOfParameterSets,
//                                                            NULL);

//         // Write each parameter set to the elementary stream
//         for (int i = 0; i < numberOfParameterSets; i++) {
//             const uint8_t *parameterSetPointer;
//             size_t parameterSetLength;
//             CMVideoFormatDescriptionGetH264ParameterSetAtIndex(description,
//                                                                i,
//                                                                &parameterSetPointer,
//                                                                &parameterSetLength,
//                                                                NULL, NULL);

//             // Write the parameter set to the elementary stream
//             [elementaryStream appendBytes:startCode length:startCodeLength];
//             [elementaryStream appendBytes:parameterSetPointer length:parameterSetLength];
//         }
//     }

//     // Get a pointer to the raw AVCC NAL unit data in the sample buffer
//     size_t blockBufferLength;
//     uint8_t *bufferDataPointer = NULL;
//     CMBlockBufferGetDataPointer(CMSampleBufferGetDataBuffer(sampleBuffer),
//                                 0,
//                                 NULL,
//                                 &blockBufferLength,
//                                 (char **)&bufferDataPointer);

//     // Loop through all the NAL units in the block buffer
//     // and write them to the elementary stream with
//     // start codes instead of AVCC length headers
//     size_t bufferOffset = 0;
//     static const int AVCCHeaderLength = 4;
//     while (bufferOffset < blockBufferLength - AVCCHeaderLength) {
//         // Read the NAL unit length
//         uint32_t NALUnitLength = 0;
//         memcpy(&NALUnitLength, bufferDataPointer + bufferOffset, AVCCHeaderLength);
//         // Convert the length value from Big-endian to Little-endian
//         NALUnitLength = CFSwapInt32BigToHost(NALUnitLength);
//         // Write start code to the elementary stream
//         [elementaryStream appendBytes:startCode length:startCodeLength];
//         // Write the NAL unit without the AVCC length header to the elementary stream
//         [elementaryStream appendBytes:bufferDataPointer + bufferOffset + AVCCHeaderLength
//                                length:NALUnitLength];
//         // Move to the next NAL unit in the block buffer
//         bufferOffset += AVCCHeaderLength + NALUnitLength;
//     }
// }
