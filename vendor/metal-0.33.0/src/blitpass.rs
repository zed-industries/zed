use super::*;

/// See <https://developer.apple.com/documentation/metal/mtlblitpassdescriptor>
pub enum MTLBlitPassDescriptor {}

foreign_obj_type! {
    type CType = MTLBlitPassDescriptor;
    pub struct BlitPassDescriptor;
}

impl BlitPassDescriptor {
    /// Creates a default blit command pass descriptor with no attachments.
    pub fn new<'a>() -> &'a BlitPassDescriptorRef {
        unsafe { msg_send![class!(MTLBlitPassDescriptor), blitPassDescriptor] }
    }
}

impl BlitPassDescriptorRef {
    // See <https://developer.apple.com/documentation/metal/mtlblitpassdescriptor>
    pub fn sample_buffer_attachments(&self) -> &BlitPassSampleBufferAttachmentDescriptorArrayRef {
        unsafe { msg_send![self, sampleBufferAttachments] }
    }
}

/// See <https://developer.apple.com/documentation/metal/mtlblitpasssamplebufferattachmentdescriptorarray>
pub enum MTLBlitPassSampleBufferAttachmentDescriptorArray {}

foreign_obj_type! {
    type CType = MTLBlitPassSampleBufferAttachmentDescriptorArray;
    pub struct BlitPassSampleBufferAttachmentDescriptorArray;
}

impl BlitPassSampleBufferAttachmentDescriptorArrayRef {
    pub fn object_at(
        &self,
        index: NSUInteger,
    ) -> Option<&BlitPassSampleBufferAttachmentDescriptorRef> {
        unsafe { msg_send![self, objectAtIndexedSubscript: index] }
    }

    pub fn set_object_at(
        &self,
        index: NSUInteger,
        attachment: Option<&BlitPassSampleBufferAttachmentDescriptorRef>,
    ) {
        unsafe {
            msg_send![self, setObject:attachment
                     atIndexedSubscript:index]
        }
    }
}

/// See <https://developer.apple.com/documentation/metal/mtlblitpasssamplebufferattachmentdescriptor>
pub enum MTLBlitPassSampleBufferAttachmentDescriptor {}

foreign_obj_type! {
    type CType = MTLBlitPassSampleBufferAttachmentDescriptor;
    pub struct BlitPassSampleBufferAttachmentDescriptor;
}

impl BlitPassSampleBufferAttachmentDescriptor {
    pub fn new() -> Self {
        let class = class!(MTLBlitPassSampleBufferAttachmentDescriptor);
        unsafe { msg_send![class, new] }
    }
}

impl BlitPassSampleBufferAttachmentDescriptorRef {
    pub fn sample_buffer(&self) -> Option<&CounterSampleBufferRef> {
        unsafe { msg_send![self, sampleBuffer] }
    }

    pub fn set_sample_buffer(&self, sample_buffer: &CounterSampleBufferRef) {
        unsafe { msg_send![self, setSampleBuffer: sample_buffer] }
    }

    pub fn start_of_encoder_sample_index(&self) -> NSUInteger {
        unsafe { msg_send![self, startOfEncoderSampleIndex] }
    }

    pub fn set_start_of_encoder_sample_index(&self, start_of_encoder_sample_index: NSUInteger) {
        unsafe {
            msg_send![
                self,
                setStartOfEncoderSampleIndex: start_of_encoder_sample_index
            ]
        }
    }

    pub fn end_of_encoder_sample_index(&self) -> NSUInteger {
        unsafe { msg_send![self, endOfEncoderSampleIndex] }
    }

    pub fn set_end_of_encoder_sample_index(&self, end_of_encoder_sample_index: NSUInteger) {
        unsafe {
            msg_send![
                self,
                setEndOfEncoderSampleIndex: end_of_encoder_sample_index
            ]
        }
    }
}
