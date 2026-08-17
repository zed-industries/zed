// Copyright 2016 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use super::*;

use block::Block;

/// See <https://developer.apple.com/documentation/metal/mtlcommandbufferstatus>
#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MTLCommandBufferStatus {
    NotEnqueued = 0,
    Enqueued = 1,
    Committed = 2,
    Scheduled = 3,
    Completed = 4,
    Error = 5,
}

/// See <https://developer.apple.com/documentation/metal/mtlcommandbuffererror>
#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MTLCommandBufferError {
    None = 0,
    Internal = 1,
    Timeout = 2,
    PageFault = 3,
    Blacklisted = 4,
    NotPermitted = 7,
    OutOfMemory = 8,
    InvalidResource = 9,
    Memoryless = 10,
    DeviceRemoved = 11,
}

/// See <https://developer.apple.com/documentation/metal/mtldispatchtype>
#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MTLDispatchType {
    Serial = 0,
    Concurrent = 1,
}

type CommandBufferHandler<'a> = Block<(&'a CommandBufferRef,), ()>;

/// See <https://developer.apple.com/documentation/metal/mtlcommandbuffer>.
pub enum MTLCommandBuffer {}

foreign_obj_type! {
    type CType = MTLCommandBuffer;
    pub struct CommandBuffer;
}

impl CommandBufferRef {
    pub fn label(&self) -> &str {
        unsafe {
            let label = msg_send![self, label];
            crate::nsstring_as_str(label)
        }
    }

    pub fn set_label(&self, label: &str) {
        unsafe {
            let nslabel = crate::nsstring_from_str(label);
            let () = msg_send![self, setLabel: nslabel];
        }
    }

    pub fn enqueue(&self) {
        unsafe { msg_send![self, enqueue] }
    }

    pub fn commit(&self) {
        unsafe { msg_send![self, commit] }
    }

    pub fn status(&self) -> MTLCommandBufferStatus {
        unsafe { msg_send![self, status] }
    }

    pub fn present_drawable(&self, drawable: &DrawableRef) {
        unsafe { msg_send![self, presentDrawable: drawable] }
    }

    pub fn wait_until_completed(&self) {
        unsafe { msg_send![self, waitUntilCompleted] }
    }

    pub fn wait_until_scheduled(&self) {
        unsafe { msg_send![self, waitUntilScheduled] }
    }

    pub fn add_completed_handler(&self, block: &CommandBufferHandler) {
        unsafe { msg_send![self, addCompletedHandler: block] }
    }

    pub fn add_scheduled_handler(&self, block: &CommandBufferHandler) {
        unsafe { msg_send![self, addScheduledHandler: block] }
    }

    /// Create a blit command encoder.
    ///
    /// Although this method is named with a `new_` prefix, the actual Metal
    /// method is not, and it returns an object that has been added to the
    /// autorelease pool.  See <https://github.com/gfx-rs/metal-rs/issues/128>.
    pub fn new_blit_command_encoder(&self) -> &BlitCommandEncoderRef {
        unsafe { msg_send![self, blitCommandEncoder] }
    }

    pub fn blit_command_encoder_with_descriptor(
        &self,
        descriptor: &BlitPassDescriptorRef,
    ) -> &BlitCommandEncoderRef {
        unsafe { msg_send![self, blitCommandEncoderWithDescriptor: descriptor] }
    }

    /// Create a compute command encoder.
    ///
    /// Although this method is named with a `new_` prefix, the actual Metal
    /// method is not, and it returns an object that has been added to the
    /// autorelease pool.  See <https://github.com/gfx-rs/metal-rs/issues/128>.
    pub fn new_compute_command_encoder(&self) -> &ComputeCommandEncoderRef {
        unsafe { msg_send![self, computeCommandEncoder] }
    }

    pub fn compute_command_encoder_with_dispatch_type(
        &self,
        ty: MTLDispatchType,
    ) -> &ComputeCommandEncoderRef {
        unsafe { msg_send![self, computeCommandEncoderWithDispatchType: ty] }
    }

    pub fn compute_command_encoder_with_descriptor(
        &self,
        descriptor: &ComputePassDescriptorRef,
    ) -> &ComputeCommandEncoderRef {
        unsafe { msg_send![self, computeCommandEncoderWithDescriptor: descriptor] }
    }

    /// Create a render command encoder.
    ///
    /// Although this method is named with a `new_` prefix, the actual Metal
    /// method is not, and it returns an object that has been added to the
    /// autorelease pool.  See <https://github.com/gfx-rs/metal-rs/issues/128>.
    pub fn new_render_command_encoder(
        &self,
        descriptor: &RenderPassDescriptorRef,
    ) -> &RenderCommandEncoderRef {
        unsafe { msg_send![self, renderCommandEncoderWithDescriptor: descriptor] }
    }

    /// Create a parallel render command encoder.
    ///
    /// Although this method is named with a `new_` prefix, the actual Metal
    /// method is not, and it returns an object that has been added to the
    /// autorelease pool.  See <https://github.com/gfx-rs/metal-rs/issues/128>.
    pub fn new_parallel_render_command_encoder(
        &self,
        descriptor: &RenderPassDescriptorRef,
    ) -> &ParallelRenderCommandEncoderRef {
        unsafe { msg_send![self, parallelRenderCommandEncoderWithDescriptor: descriptor] }
    }

    /// Create an acceleration structure command encoder.
    ///
    /// Although this method is named with a `new_` prefix, the actual Metal
    /// method is not, and it returns an object that has been added to the
    /// autorelease pool.  See <https://github.com/gfx-rs/metal-rs/issues/128>.
    pub fn new_acceleration_structure_command_encoder(
        &self,
    ) -> &AccelerationStructureCommandEncoderRef {
        unsafe { msg_send![self, accelerationStructureCommandEncoder] }
    }

    /// Create an acceleration structure command encoder.
    ///
    /// Although this method is named with a `new_` prefix, the actual Metal
    /// method is not, and it returns an object that has been added to the
    /// autorelease pool.  See <https://github.com/gfx-rs/metal-rs/issues/128>.
    pub fn acceleration_structure_command_encoder_with_descriptor(
        &self,
        descriptor: &AccelerationStructurePassDescriptorRef,
    ) -> &AccelerationStructureCommandEncoderRef {
        unsafe { msg_send![self, accelerationStructureCommandEncoderWithDescriptor: descriptor] }
    }

    pub fn encode_signal_event(&self, event: &EventRef, new_value: u64) {
        unsafe {
            msg_send![self,
                encodeSignalEvent: event
                value: new_value
            ]
        }
    }

    pub fn encode_wait_for_event(&self, event: &EventRef, value: u64) {
        unsafe {
            msg_send![self,
                encodeWaitForEvent: event
                value: value
            ]
        }
    }

    pub fn push_debug_group(&self, name: &str) {
        unsafe {
            let nslabel = crate::nsstring_from_str(name);
            msg_send![self, pushDebugGroup: nslabel]
        }
    }

    pub fn pop_debug_group(&self) {
        unsafe { msg_send![self, popDebugGroup] }
    }
}
