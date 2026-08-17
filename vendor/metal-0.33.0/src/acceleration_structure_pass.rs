use super::{CounterSampleBufferRef, NSUInteger};

/// See <https://developer.apple.com/documentation/metal/mtlaccelerationstructurepassdescriptor>
pub enum MTLAccelerationStructurePassDescriptor {}

foreign_obj_type! {
    type CType = MTLAccelerationStructurePassDescriptor;
    pub struct AccelerationStructurePassDescriptor;
}

impl AccelerationStructurePassDescriptor {
    /// Creates a default compute pass descriptor with no attachments.
    pub fn new<'a>() -> &'a AccelerationStructurePassDescriptorRef {
        unsafe {
            msg_send![
                class!(MTLAccelerationStructurePassDescriptor),
                accelerationStructurePassDescriptor
            ]
        }
    }
}

impl AccelerationStructurePassDescriptorRef {
    pub fn sample_buffer_attachments(
        &self,
    ) -> &AccelerationStructurePassSampleBufferAttachmentDescriptorArrayRef {
        unsafe { msg_send![self, sampleBufferAttachments] }
    }
}

/// See <https://developer.apple.com/documentation/metal/mtlaccelerationstructurepasssamplebufferattachmentdescriptorarray>
pub enum MTLAccelerationStructurePassSampleBufferAttachmentDescriptorArray {}

foreign_obj_type! {
    type CType = MTLAccelerationStructurePassSampleBufferAttachmentDescriptorArray;
    pub struct AccelerationStructurePassSampleBufferAttachmentDescriptorArray;
}

impl AccelerationStructurePassSampleBufferAttachmentDescriptorArrayRef {
    pub fn object_at(
        &self,
        index: NSUInteger,
    ) -> Option<&AccelerationStructurePassSampleBufferAttachmentDescriptorRef> {
        unsafe { msg_send![self, objectAtIndexedSubscript: index] }
    }

    pub fn set_object_at(
        &self,
        index: NSUInteger,
        attachment: Option<&AccelerationStructurePassSampleBufferAttachmentDescriptorRef>,
    ) {
        unsafe {
            msg_send![self, setObject:attachment
                     atIndexedSubscript:index]
        }
    }
}

/// See <https://developer.apple.com/documentation/metal/mtlaccelerationstructurepasssamplebufferattachmentdescriptor>
pub enum MTLAccelerationStructurePassSampleBufferAttachmentDescriptor {}

foreign_obj_type! {
    type CType = MTLAccelerationStructurePassSampleBufferAttachmentDescriptor;
    pub struct AccelerationStructurePassSampleBufferAttachmentDescriptor;
}

impl AccelerationStructurePassSampleBufferAttachmentDescriptor {
    pub fn new() -> Self {
        let class = class!(MTLAccelerationStructurePassSampleBufferAttachmentDescriptor);
        unsafe { msg_send![class, new] }
    }
}

impl AccelerationStructurePassSampleBufferAttachmentDescriptorRef {
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
