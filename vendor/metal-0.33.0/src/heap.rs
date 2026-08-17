// Copyright 2016 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use super::*;

/// Only available on macos(10.15), ios(13.0)
///
/// See <https://developer.apple.com/documentation/metal/mtlheaptype/>
#[repr(u64)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum MTLHeapType {
    Automatic = 0,
    Placement = 1,
    /// Only available on macos(11.0), macCatalyst(14.0)
    Sparse = 2,
}

/// See <https://developer.apple.com/documentation/metal/mtlheap/>
pub enum MTLHeap {}

foreign_obj_type! {
    type CType = MTLHeap;
    pub struct Heap;
}

impl HeapRef {
    pub fn device(&self) -> &DeviceRef {
        unsafe { msg_send![self, device] }
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

    pub fn cpu_cache_mode(&self) -> MTLCPUCacheMode {
        unsafe { msg_send![self, cpuCacheMode] }
    }

    pub fn storage_mode(&self) -> MTLStorageMode {
        unsafe { msg_send![self, storageMode] }
    }

    /// Only available on macos(10.15), ios(13.0)
    pub fn hazard_tracking_mode(&self) -> MTLHazardTrackingMode {
        unsafe { msg_send![self, hazardTrackingMode] }
    }

    /// Only available on macos(10.15), ios(13.0)
    pub fn resource_options(&self) -> MTLResourceOptions {
        unsafe { msg_send![self, resourceOptions] }
    }

    pub fn set_purgeable_state(&self, state: MTLPurgeableState) -> MTLPurgeableState {
        unsafe { msg_send![self, setPurgeableState: state] }
    }

    pub fn size(&self) -> NSUInteger {
        unsafe { msg_send![self, size] }
    }

    pub fn used_size(&self) -> NSUInteger {
        unsafe { msg_send![self, usedSize] }
    }

    /// Only available on macos(10.15), ios(13.0)
    pub fn heap_type(&self) -> MTLHeapType {
        unsafe { msg_send![self, type] }
    }

    /// Only available on macos(10.13), ios(11.0)
    pub fn current_allocated_size(&self) -> NSUInteger {
        unsafe { msg_send![self, currentAllocatedSize] }
    }

    pub fn max_available_size_with_alignment(&self, alignment: NSUInteger) -> NSUInteger {
        unsafe { msg_send![self, maxAvailableSizeWithAlignment: alignment] }
    }

    pub fn new_buffer(&self, length: u64, options: MTLResourceOptions) -> Option<Buffer> {
        unsafe {
            let ptr: *mut MTLBuffer = msg_send![self, newBufferWithLength:length
                                                                  options:options];
            if !ptr.is_null() {
                Some(Buffer::from_ptr(ptr))
            } else {
                None
            }
        }
    }

    pub fn new_texture(&self, descriptor: &TextureDescriptorRef) -> Option<Texture> {
        unsafe {
            let ptr: *mut MTLTexture = msg_send![self, newTextureWithDescriptor: descriptor];
            if !ptr.is_null() {
                Some(Texture::from_ptr(ptr))
            } else {
                None
            }
        }
    }

    /// Only available on macOS 10.15+ & iOS 13.0+
    pub fn new_buffer_with_offset(
        &self,
        length: u64,
        options: MTLResourceOptions,
        offset: u64,
    ) -> Option<Buffer> {
        unsafe {
            let ptr: *mut MTLBuffer = msg_send![self, newBufferWithLength:length
                                                                  options:options
                                                                  offset:offset];
            if !ptr.is_null() {
                Some(Buffer::from_ptr(ptr))
            } else {
                None
            }
        }
    }

    /// Only available on macOS 10.15+ & iOS 13.0+
    pub fn new_texture_with_offset(
        &self,
        descriptor: &TextureDescriptorRef,
        offset: u64,
    ) -> Option<Texture> {
        unsafe {
            let ptr: *mut MTLTexture = msg_send![self, newTextureWithDescriptor:descriptor
                                                                         offset:offset];
            if !ptr.is_null() {
                Some(Texture::from_ptr(ptr))
            } else {
                None
            }
        }
    }

    /// Only available on macOS 13.0+ & iOS 16.0+
    pub fn new_acceleration_structure_with_descriptor(
        &self,
        descriptor: &AccelerationStructureDescriptorRef,
    ) -> Option<AccelerationStructure> {
        unsafe {
            let ptr: *mut MTLAccelerationStructure =
                msg_send![self, newAccelerationStructureWithDescriptor: descriptor];
            if !ptr.is_null() {
                Some(AccelerationStructure::from_ptr(ptr))
            } else {
                None
            }
        }
    }

    /// Only available on macOS 13.0+ & iOS 16.0+
    pub fn new_acceleration_structure_with_descriptor_offset(
        &self,
        descriptor: &AccelerationStructureDescriptorRef,
        offset: u64,
    ) -> Option<AccelerationStructure> {
        unsafe {
            let ptr: *mut MTLAccelerationStructure = msg_send![self, newAccelerationStructureWithDescriptor:descriptor
                                                                     offset:offset];
            if !ptr.is_null() {
                Some(AccelerationStructure::from_ptr(ptr))
            } else {
                None
            }
        }
    }

    /// Only available on macOS 13.0+ & iOS 16.0+
    pub fn new_acceleration_structure_with_size(&self, size: u64) -> Option<AccelerationStructure> {
        unsafe {
            let ptr: *mut MTLAccelerationStructure =
                msg_send![self, newAccelerationStructureWithSize:size];
            if !ptr.is_null() {
                Some(AccelerationStructure::from_ptr(ptr))
            } else {
                None
            }
        }
    }

    /// Only available on macOS 13.0+ & iOS 16.0+
    pub fn new_acceleration_structure_with_size_offset(
        &self,
        size: u64,
        offset: u64,
    ) -> Option<AccelerationStructure> {
        unsafe {
            let ptr: *mut MTLAccelerationStructure = msg_send![self, newAccelerationStructureWithSize:size
                                                                     offset:offset];
            if !ptr.is_null() {
                Some(AccelerationStructure::from_ptr(ptr))
            } else {
                None
            }
        }
    }
}

/// See <https://developer.apple.com/documentation/metal/mtlheapdescriptor/>
pub enum MTLHeapDescriptor {}

foreign_obj_type! {
    type CType = MTLHeapDescriptor;
    pub struct HeapDescriptor;
}

impl HeapDescriptor {
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLHeapDescriptor);
            msg_send![class, new]
        }
    }
}

impl HeapDescriptorRef {
    pub fn cpu_cache_mode(&self) -> MTLCPUCacheMode {
        unsafe { msg_send![self, cpuCacheMode] }
    }

    pub fn set_cpu_cache_mode(&self, mode: MTLCPUCacheMode) {
        unsafe { msg_send![self, setCpuCacheMode: mode] }
    }

    pub fn storage_mode(&self) -> MTLStorageMode {
        unsafe { msg_send![self, storageMode] }
    }

    pub fn set_storage_mode(&self, mode: MTLStorageMode) {
        unsafe { msg_send![self, setStorageMode: mode] }
    }

    pub fn size(&self) -> NSUInteger {
        unsafe { msg_send![self, size] }
    }

    pub fn set_size(&self, size: NSUInteger) {
        unsafe { msg_send![self, setSize: size] }
    }

    /// Only available on macos(10.15), ios(13.0)
    pub fn hazard_tracking_mode(&self) -> MTLHazardTrackingMode {
        unsafe { msg_send![self, hazardTrackingMode] }
    }

    /// Only available on macos(10.15), ios(13.0)
    pub fn set_hazard_tracking_mode(&self, hazard_tracking_mode: MTLHazardTrackingMode) {
        unsafe { msg_send![self, setHazardTrackingMode: hazard_tracking_mode] }
    }

    /// Only available on macos(10.15), ios(13.0)
    pub fn resource_options(&self) -> MTLResourceOptions {
        unsafe { msg_send![self, resourceOptions] }
    }

    /// Only available on macos(10.15), ios(13.0)
    pub fn heap_type(&self) -> MTLHeapType {
        unsafe { msg_send![self, type] }
    }
    /// Only available on macos(10.15), ios(13.0)
    pub fn set_heap_type(&self, type_: MTLHeapType) {
        unsafe { msg_send![self, setType: type_] }
    }
}
