// Copyright 2016 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::DeviceRef;
use objc::runtime::{NO, YES};

/// See <https://developer.apple.com/documentation/metal/mtlcomparefunction>
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MTLCompareFunction {
    Never = 0,
    Less = 1,
    Equal = 2,
    LessEqual = 3,
    Greater = 4,
    NotEqual = 5,
    GreaterEqual = 6,
    Always = 7,
}

/// See <https://developer.apple.com/documentation/metal/mtlstenciloperation>
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MTLStencilOperation {
    Keep = 0,
    Zero = 1,
    Replace = 2,
    IncrementClamp = 3,
    DecrementClamp = 4,
    Invert = 5,
    IncrementWrap = 6,
    DecrementWrap = 7,
}

/// See <https://developer.apple.com/documentation/metal/mtlstencildescriptor>
pub enum MTLStencilDescriptor {}

foreign_obj_type! {
    type CType = MTLStencilDescriptor;
    pub struct StencilDescriptor;
}

impl StencilDescriptor {
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLStencilDescriptor);
            msg_send![class, new]
        }
    }
}

impl StencilDescriptorRef {
    pub fn stencil_compare_function(&self) -> MTLCompareFunction {
        unsafe { msg_send![self, stencilCompareFunction] }
    }

    pub fn set_stencil_compare_function(&self, func: MTLCompareFunction) {
        unsafe { msg_send![self, setStencilCompareFunction: func] }
    }

    pub fn stencil_failure_operation(&self) -> MTLStencilOperation {
        unsafe { msg_send![self, stencilFailureOperation] }
    }

    pub fn set_stencil_failure_operation(&self, operation: MTLStencilOperation) {
        unsafe { msg_send![self, setStencilFailureOperation: operation] }
    }

    pub fn depth_failure_operation(&self) -> MTLStencilOperation {
        unsafe { msg_send![self, depthFailureOperation] }
    }

    pub fn set_depth_failure_operation(&self, operation: MTLStencilOperation) {
        unsafe { msg_send![self, setDepthFailureOperation: operation] }
    }

    pub fn depth_stencil_pass_operation(&self) -> MTLStencilOperation {
        unsafe { msg_send![self, depthStencilPassOperation] }
    }

    pub fn set_depth_stencil_pass_operation(&self, operation: MTLStencilOperation) {
        unsafe { msg_send![self, setDepthStencilPassOperation: operation] }
    }

    pub fn read_mask(&self) -> u32 {
        unsafe { msg_send![self, readMask] }
    }

    pub fn set_read_mask(&self, mask: u32) {
        unsafe { msg_send![self, setReadMask: mask] }
    }

    pub fn write_mask(&self) -> u32 {
        unsafe { msg_send![self, writeMask] }
    }

    pub fn set_write_mask(&self, mask: u32) {
        unsafe { msg_send![self, setWriteMask: mask] }
    }
}

/// See <https://developer.apple.com/documentation/metal/mtldepthstencildescriptor>
pub enum MTLDepthStencilDescriptor {}

foreign_obj_type! {
    type CType = MTLDepthStencilDescriptor;
    pub struct DepthStencilDescriptor;
}

impl DepthStencilDescriptor {
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLDepthStencilDescriptor);
            msg_send![class, new]
        }
    }
}

impl DepthStencilDescriptorRef {
    pub fn depth_compare_function(&self) -> MTLCompareFunction {
        unsafe { msg_send![self, depthCompareFunction] }
    }

    pub fn set_depth_compare_function(&self, func: MTLCompareFunction) {
        unsafe { msg_send![self, setDepthCompareFunction: func] }
    }

    pub fn depth_write_enabled(&self) -> bool {
        unsafe { msg_send_bool![self, isDepthWriteEnabled] }
    }

    pub fn set_depth_write_enabled(&self, enabled: bool) {
        unsafe { msg_send![self, setDepthWriteEnabled: enabled] }
    }

    pub fn front_face_stencil(&self) -> Option<&StencilDescriptorRef> {
        unsafe { msg_send![self, frontFaceStencil] }
    }

    pub fn set_front_face_stencil(&self, descriptor: Option<&StencilDescriptorRef>) {
        unsafe { msg_send![self, setFrontFaceStencil: descriptor] }
    }

    pub fn back_face_stencil(&self) -> Option<&StencilDescriptorRef> {
        unsafe { msg_send![self, backFaceStencil] }
    }

    pub fn set_back_face_stencil(&self, descriptor: Option<&StencilDescriptorRef>) {
        unsafe { msg_send![self, setBackFaceStencil: descriptor] }
    }

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
}

/// See <https://developer.apple.com/documentation/metal/mtldepthstencilstate>
pub enum MTLDepthStencilState {}

foreign_obj_type! {
    type CType = MTLDepthStencilState;
    pub struct DepthStencilState;
}

impl DepthStencilStateRef {
    pub fn device(&self) -> &DeviceRef {
        unsafe { msg_send![self, device] }
    }

    pub fn label(&self) -> &str {
        unsafe {
            let label = msg_send![self, label];
            crate::nsstring_as_str(label)
        }
    }
}
