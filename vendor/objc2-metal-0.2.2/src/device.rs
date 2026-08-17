#![cfg(commented_out)]
use crate::MTLDevice;

pub trait MTLDeviceExtension {
    // pub fn system_default() -> Option<Self> {
    //     MTLCreateSystemDefaultDevice()
    // }
    //
    // pub fn all() -> Retained<NSArray<Self>> {
    //     #[cfg(target_os = "macos")]
    //     MTLCopyAllDevices()
    //     #[cfg(not(target_os = "macos"))]
    //     NSArray::from(MTLCreateSystemDefaultDevice())
    // }
}

impl<P: MTLDevice> MTLDeviceExtension for P {}
