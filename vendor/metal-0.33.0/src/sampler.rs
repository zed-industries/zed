// Copyright 2016 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use super::{depthstencil::MTLCompareFunction, DeviceRef, MTLResourceID, NSUInteger};

/// See <https://developer.apple.com/documentation/metal/mtlsamplerminmagfilter>
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MTLSamplerMinMagFilter {
    Nearest = 0,
    Linear = 1,
}

/// See <https://developer.apple.com/documentation/metal/mtlsamplermipfilter>
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MTLSamplerMipFilter {
    NotMipmapped = 0,
    Nearest = 1,
    Linear = 2,
}

/// See <https://developer.apple.com/documentation/metal/mtlsampleraddressmode>
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MTLSamplerAddressMode {
    ClampToEdge = 0,
    MirrorClampToEdge = 1,
    Repeat = 2,
    MirrorRepeat = 3,
    ClampToZero = 4,
    ClampToBorderColor = 5,
}

/// See <https://developer.apple.com/documentation/metal/mtlsamplerbordercolor>
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MTLSamplerBorderColor {
    TransparentBlack = 0,
    OpaqueBlack = 1,
    OpaqueWhite = 2,
}

/// See <https://developer.apple.com/documentation/metal/mtlsamplerdescriptor>
pub enum MTLSamplerDescriptor {}

foreign_obj_type! {
    type CType = MTLSamplerDescriptor;
    pub struct SamplerDescriptor;
}

impl SamplerDescriptor {
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLSamplerDescriptor);
            msg_send![class, new]
        }
    }
}

impl SamplerDescriptorRef {
    pub fn set_min_filter(&self, filter: MTLSamplerMinMagFilter) {
        unsafe { msg_send![self, setMinFilter: filter] }
    }

    pub fn set_mag_filter(&self, filter: MTLSamplerMinMagFilter) {
        unsafe { msg_send![self, setMagFilter: filter] }
    }

    pub fn set_mip_filter(&self, filter: MTLSamplerMipFilter) {
        unsafe { msg_send![self, setMipFilter: filter] }
    }

    pub fn set_address_mode_s(&self, mode: MTLSamplerAddressMode) {
        unsafe { msg_send![self, setSAddressMode: mode] }
    }

    pub fn set_address_mode_t(&self, mode: MTLSamplerAddressMode) {
        unsafe { msg_send![self, setTAddressMode: mode] }
    }

    pub fn set_address_mode_r(&self, mode: MTLSamplerAddressMode) {
        unsafe { msg_send![self, setRAddressMode: mode] }
    }

    pub fn set_max_anisotropy(&self, anisotropy: NSUInteger) {
        unsafe { msg_send![self, setMaxAnisotropy: anisotropy] }
    }

    pub fn set_compare_function(&self, func: MTLCompareFunction) {
        unsafe { msg_send![self, setCompareFunction: func] }
    }

    #[cfg(feature = "private")]
    pub unsafe fn set_lod_bias(&self, bias: f32) {
        msg_send![self, setLodBias: bias]
    }

    pub fn set_lod_min_clamp(&self, clamp: f32) {
        unsafe { msg_send![self, setLodMinClamp: clamp] }
    }

    pub fn set_lod_max_clamp(&self, clamp: f32) {
        unsafe { msg_send![self, setLodMaxClamp: clamp] }
    }

    pub fn set_lod_average(&self, enable: bool) {
        unsafe { msg_send![self, setLodAverage: enable] }
    }

    pub fn set_normalized_coordinates(&self, enable: bool) {
        unsafe { msg_send![self, setNormalizedCoordinates: enable] }
    }

    pub fn set_support_argument_buffers(&self, enable: bool) {
        unsafe { msg_send![self, setSupportArgumentBuffers: enable] }
    }

    pub fn set_border_color(&self, color: MTLSamplerBorderColor) {
        unsafe { msg_send![self, setBorderColor: color] }
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

/// See <https://developer.apple.com/documentation/metal/mtlsamplerstate>
pub enum MTLSamplerState {}

foreign_obj_type! {
    type CType = MTLSamplerState;
    pub struct SamplerState;
}

impl SamplerStateRef {
    pub fn device(&self) -> &DeviceRef {
        unsafe { msg_send![self, device] }
    }

    pub fn label(&self) -> &str {
        unsafe {
            let label = msg_send![self, label];
            crate::nsstring_as_str(label)
        }
    }

    pub fn gpu_resource_id(&self) -> MTLResourceID {
        unsafe { msg_send![self, gpuResourceID] }
    }
}
