// Copyright 2018 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use super::*;
use std::ffi::CStr;

/// See <https://developer.apple.com/documentation/metal/mtlcapturescope>
pub enum MTLCaptureScope {}

foreign_obj_type! {
    type CType = MTLCaptureScope;
    pub struct CaptureScope;
}

impl CaptureScopeRef {
    pub fn begin_scope(&self) {
        unsafe { msg_send![self, beginScope] }
    }

    pub fn end_scope(&self) {
        unsafe { msg_send![self, endScope] }
    }

    pub fn label(&self) -> &str {
        unsafe {
            let label = msg_send![self, label];
            crate::nsstring_as_str(label)
        }
    }
}

/// See <https://developer.apple.com/documentation/metal/mtlcapturemanager>
pub enum MTLCaptureManager {}

foreign_obj_type! {
    type CType = MTLCaptureManager;
    pub struct CaptureManager;
}

impl CaptureManager {
    pub fn shared<'a>() -> &'a CaptureManagerRef {
        unsafe {
            let class = class!(MTLCaptureManager);
            msg_send![class, sharedCaptureManager]
        }
    }
}

impl CaptureManagerRef {
    pub fn new_capture_scope_with_device(&self, device: &DeviceRef) -> CaptureScope {
        unsafe { msg_send![self, newCaptureScopeWithDevice: device] }
    }

    pub fn new_capture_scope_with_command_queue(
        &self,
        command_queue: &CommandQueueRef,
    ) -> CaptureScope {
        unsafe { msg_send![self, newCaptureScopeWithCommandQueue: command_queue] }
    }

    pub fn default_capture_scope(&self) -> Option<&CaptureScopeRef> {
        unsafe { msg_send![self, defaultCaptureScope] }
    }

    pub fn set_default_capture_scope(&self, scope: &CaptureScopeRef) {
        unsafe { msg_send![self, setDefaultCaptureScope: scope] }
    }

    /// Starts capturing with the capture session defined by a descriptor object.
    ///
    /// This function will panic if Metal capture is not enabled.  Capture can be enabled by
    /// either:
    ///   1. Running from Xcode
    ///   2. Setting the environment variable `METAL_CAPTURE_ENABLED=1`
    ///   3. Adding an info.plist file containing the `MetalCaptureEnabled` key set to `YES`
    pub fn start_capture(&self, descriptor: &CaptureDescriptorRef) -> Result<(), String> {
        #[allow(clippy::unit_arg)]
        unsafe {
            Ok(try_objc! { err =>
                msg_send![self, startCaptureWithDescriptor: descriptor
                                error: &mut err]
            })
        }
    }

    pub fn start_capture_with_device(&self, device: &DeviceRef) {
        unsafe { msg_send![self, startCaptureWithDevice: device] }
    }

    pub fn start_capture_with_command_queue(&self, command_queue: &CommandQueueRef) {
        unsafe { msg_send![self, startCaptureWithCommandQueue: command_queue] }
    }

    pub fn start_capture_with_scope(&self, scope: &CaptureScopeRef) {
        unsafe { msg_send![self, startCaptureWithScope: scope] }
    }

    pub fn stop_capture(&self) {
        unsafe { msg_send![self, stopCapture] }
    }

    pub fn is_capturing(&self) -> bool {
        unsafe { msg_send![self, isCapturing] }
    }

    /// See <https://developer.apple.com/documentation/metal/mtlcapturemanager/3237260-supportsdestination?language=objc>
    pub fn supports_destination(&self, destination: MTLCaptureDestination) -> bool {
        unsafe { msg_send![self, supportsDestination: destination] }
    }
}
