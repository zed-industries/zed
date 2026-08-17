// Copyright 2020 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use super::*;

use std::path::Path;

/// See <https://developer.apple.com/documentation/metal/mtlcapturedestination?language=objc>
#[repr(u64)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MTLCaptureDestination {
    DeveloperTools = 1,
    GpuTraceDocument = 2,
}

/// See <https://developer.apple.com/documentation/metal/mtlcapturedescriptor>
pub enum MTLCaptureDescriptor {}

foreign_obj_type! {
    type CType = MTLCaptureDescriptor;
    pub struct CaptureDescriptor;
}

impl CaptureDescriptor {
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLCaptureDescriptor);
            msg_send![class, new]
        }
    }
}

impl CaptureDescriptorRef {
    /// See <https://developer.apple.com/documentation/metal/mtlcapturedescriptor/3237248-captureobject>
    pub fn set_capture_device(&self, device: &DeviceRef) {
        unsafe { msg_send![self, setCaptureObject: device] }
    }

    /// See <https://developer.apple.com/documentation/metal/mtlcapturedescriptor/3237248-captureobject>
    pub fn set_capture_scope(&self, scope: &CaptureScopeRef) {
        unsafe { msg_send![self, setCaptureObject: scope] }
    }

    /// See <https://developer.apple.com/documentation/metal/mtlcapturedescriptor/3237248-captureobject>
    pub fn set_capture_command_queue(&self, command_queue: &CommandQueueRef) {
        unsafe { msg_send![self, setCaptureObject: command_queue] }
    }

    /// See <https://developer.apple.com/documentation/metal/mtlcapturedescriptor/3237250-outputurl>
    pub fn output_url(&self) -> &Path {
        let url: &URLRef = unsafe { msg_send![self, outputURL] };
        Path::new(url.path())
    }

    /// See <https://developer.apple.com/documentation/metal/mtlcapturedescriptor/3237250-outputurl>
    pub fn set_output_url<P: AsRef<Path>>(&self, output_url: P) {
        let output_url_string = String::from("file://") + output_url.as_ref().to_str().unwrap();
        let output_url = URL::new_with_string(&output_url_string);
        unsafe { msg_send![self, setOutputURL: output_url] }
    }

    /// See <https://developer.apple.com/documentation/metal/mtlcapturedescriptor?language=objc>
    pub fn destination(&self) -> MTLCaptureDestination {
        unsafe { msg_send![self, destination] }
    }

    /// See <https://developer.apple.com/documentation/metal/mtlcapturedescriptor?language=objc>
    pub fn set_destination(&self, destination: MTLCaptureDestination) {
        unsafe { msg_send![self, setDestination: destination] }
    }
}
