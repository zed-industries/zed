// Copyright 2016 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use super::*;

/// See <https://developer.apple.com/documentation/metal/mtlbuffer>
pub enum MTLBuffer {}

foreign_obj_type! {
    type CType = MTLBuffer;
    pub struct Buffer;
    type ParentType = Resource;
}

impl BufferRef {
    pub fn length(&self) -> u64 {
        unsafe { msg_send![self, length] }
    }

    pub fn contents(&self) -> *mut std::ffi::c_void {
        unsafe { msg_send![self, contents] }
    }

    pub fn did_modify_range(&self, range: crate::NSRange) {
        unsafe { msg_send![self, didModifyRange: range] }
    }

    pub fn new_texture_with_descriptor(
        &self,
        descriptor: &TextureDescriptorRef,
        offset: u64,
        bytes_per_row: u64,
    ) -> Texture {
        unsafe {
            msg_send![self,
                newTextureWithDescriptor:descriptor
                offset:offset
                bytesPerRow:bytes_per_row
            ]
        }
    }

    /// Only available on macos(10.15), NOT available on (ios)
    pub fn remote_storage_buffer(&self) -> &BufferRef {
        unsafe { msg_send![self, remoteStorageBuffer] }
    }

    /// Only available on (macos(10.15), NOT available on (ios)
    pub fn new_remote_buffer_view_for_device(&self, device: &DeviceRef) -> Buffer {
        unsafe { msg_send![self, newRemoteBufferViewForDevice: device] }
    }

    pub fn add_debug_marker(&self, name: &str, range: crate::NSRange) {
        unsafe {
            let name = crate::nsstring_from_str(name);
            msg_send![self, addDebugMarker:name range:range]
        }
    }

    pub fn remove_all_debug_markers(&self) {
        unsafe { msg_send![self, removeAllDebugMarkers] }
    }

    pub fn gpu_address(&self) -> u64 {
        unsafe { msg_send![self, gpuAddress] }
    }

    pub fn gpu_resource_id(&self) -> MTLResourceID {
        unsafe { msg_send![self, gpuResourceID] }
    }
}
