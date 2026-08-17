use super::*;

/// See <https://developer.apple.com/documentation/metal/mtlcomputepassdescriptor>
pub enum MTLComputePassDescriptor {}

foreign_obj_type! {
    type CType = MTLComputePassDescriptor;
    pub struct ComputePassDescriptor;
}

impl ComputePassDescriptor {
    /// Creates a default compute pass descriptor with no attachments.
    pub fn new<'a>() -> &'a ComputePassDescriptorRef {
        unsafe { msg_send![class!(MTLComputePassDescriptor), computePassDescriptor] }
    }
}

impl ComputePassDescriptorRef {
    pub fn sample_buffer_attachments(
        &self,
    ) -> &ComputePassSampleBufferAttachmentDescriptorArrayRef {
        unsafe { msg_send![self, sampleBufferAttachments] }
    }

    pub fn set_dispatch_type(&self, ty: MTLDispatchType) {
        unsafe { msg_send![self, setDispatchType: ty] }
    }
}

/// See <https://developer.apple.com/documentation/metal/mtlcomputepasssamplebufferattachmentdescriptorarray>
pub enum MTLComputePassSampleBufferAttachmentDescriptorArray {}

foreign_obj_type! {
    type CType = MTLComputePassSampleBufferAttachmentDescriptorArray;
    pub struct ComputePassSampleBufferAttachmentDescriptorArray;
}

impl ComputePassSampleBufferAttachmentDescriptorArrayRef {
    pub fn object_at(
        &self,
        index: NSUInteger,
    ) -> Option<&ComputePassSampleBufferAttachmentDescriptorRef> {
        unsafe { msg_send![self, objectAtIndexedSubscript: index] }
    }

    pub fn set_object_at(
        &self,
        index: NSUInteger,
        attachment: Option<&ComputePassSampleBufferAttachmentDescriptorRef>,
    ) {
        unsafe {
            msg_send![self, setObject:attachment
                     atIndexedSubscript:index]
        }
    }
}

/// See <https://developer.apple.com/documentation/metal/mtlcomputepasssamplebufferattachmentdescriptor>
pub enum MTLComputePassSampleBufferAttachmentDescriptor {}

foreign_obj_type! {
    type CType = MTLComputePassSampleBufferAttachmentDescriptor;
    pub struct ComputePassSampleBufferAttachmentDescriptor;
}

impl ComputePassSampleBufferAttachmentDescriptor {
    pub fn new() -> Self {
        let class = class!(MTLComputePassSampleBufferAttachmentDescriptor);
        unsafe { msg_send![class, new] }
    }
}

impl ComputePassSampleBufferAttachmentDescriptorRef {
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
