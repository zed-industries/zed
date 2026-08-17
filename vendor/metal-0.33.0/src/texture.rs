// Copyright 2016 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use super::*;

use objc::runtime::{NO, YES};

/// See <https://developer.apple.com/documentation/metal/mtltexturetype>
#[repr(u64)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum MTLTextureType {
    D1 = 0,
    D1Array = 1,
    D2 = 2,
    D2Array = 3,
    D2Multisample = 4,
    Cube = 5,
    CubeArray = 6,
    D3 = 7,
    D2MultisampleArray = 8,
}

/// See <https://developer.apple.com/documentation/metal/mtltexturecompressiontype>
#[repr(u64)]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum MTLTextureCompressionType {
    Lossless = 0,
    Lossy = 1,
}

bitflags::bitflags! {
    /// See <https://developer.apple.com/documentation/metal/mtltextureusage>
    #[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub struct MTLTextureUsage: NSUInteger {
        const Unknown         = 0x0000;
        const ShaderRead      = 0x0001;
        const ShaderWrite     = 0x0002;
        const RenderTarget    = 0x0004;
        const PixelFormatView = 0x0010;
        const ShaderAtomic    = 0x0020;
    }
}

/// See <https://developer.apple.com/documentation/metal/mtltexturedescriptor>
pub enum MTLTextureDescriptor {}

foreign_obj_type! {
    type CType = MTLTextureDescriptor;
    pub struct TextureDescriptor;
}

impl TextureDescriptor {
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLTextureDescriptor);
            msg_send![class, new]
        }
    }
}

impl TextureDescriptorRef {
    pub fn texture_type(&self) -> MTLTextureType {
        unsafe { msg_send![self, textureType] }
    }

    pub fn set_texture_type(&self, texture_type: MTLTextureType) {
        unsafe { msg_send![self, setTextureType: texture_type] }
    }

    pub fn pixel_format(&self) -> MTLPixelFormat {
        unsafe { msg_send![self, pixelFormat] }
    }

    pub fn set_pixel_format(&self, pixel_format: MTLPixelFormat) {
        unsafe { msg_send![self, setPixelFormat: pixel_format] }
    }

    pub fn width(&self) -> NSUInteger {
        unsafe { msg_send![self, width] }
    }

    pub fn set_width(&self, width: NSUInteger) {
        unsafe { msg_send![self, setWidth: width] }
    }

    pub fn height(&self) -> NSUInteger {
        unsafe { msg_send![self, height] }
    }

    pub fn set_height(&self, height: NSUInteger) {
        unsafe { msg_send![self, setHeight: height] }
    }

    pub fn depth(&self) -> NSUInteger {
        unsafe { msg_send![self, depth] }
    }

    pub fn set_depth(&self, depth: NSUInteger) {
        unsafe { msg_send![self, setDepth: depth] }
    }

    pub fn mipmap_level_count(&self) -> NSUInteger {
        unsafe { msg_send![self, mipmapLevelCount] }
    }

    pub fn set_mipmap_level_count(&self, count: NSUInteger) {
        unsafe { msg_send![self, setMipmapLevelCount: count] }
    }

    pub fn set_mipmap_level_count_for_size(&self, size: MTLSize) {
        let MTLSize {
            width,
            height,
            depth,
        } = size;
        let count = (width.max(height).max(depth) as f64).log2().ceil() as u64;
        self.set_mipmap_level_count(count);
    }

    pub fn sample_count(&self) -> NSUInteger {
        unsafe { msg_send![self, sampleCount] }
    }

    pub fn set_sample_count(&self, count: NSUInteger) {
        unsafe { msg_send![self, setSampleCount: count] }
    }

    pub fn array_length(&self) -> NSUInteger {
        unsafe { msg_send![self, arrayLength] }
    }

    pub fn set_array_length(&self, length: NSUInteger) {
        unsafe { msg_send![self, setArrayLength: length] }
    }

    pub fn resource_options(&self) -> MTLResourceOptions {
        unsafe { msg_send![self, resourceOptions] }
    }

    pub fn set_resource_options(&self, options: MTLResourceOptions) {
        unsafe { msg_send![self, setResourceOptions: options] }
    }

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

    pub fn usage(&self) -> MTLTextureUsage {
        unsafe { msg_send![self, usage] }
    }

    pub fn set_usage(&self, usage: MTLTextureUsage) {
        unsafe { msg_send![self, setUsage: usage] }
    }

    pub fn compression_type(&self) -> MTLTextureCompressionType {
        unsafe { msg_send![self, compressionType] }
    }

    pub fn set_compression_type(&self, compression_type: MTLTextureCompressionType) {
        unsafe { msg_send![self, setCompressionType: compression_type] }
    }
}

/// See <https://developer.apple.com/documentation/metal/mtltexture>
pub enum MTLTexture {}

foreign_obj_type! {
    type CType = MTLTexture;
    pub struct Texture;
    type ParentType = Resource;
}

impl TextureRef {
    #[deprecated(since = "0.13.0")]
    pub fn root_resource(&self) -> Option<&ResourceRef> {
        unsafe { msg_send![self, rootResource] }
    }

    pub fn parent_texture(&self) -> Option<&TextureRef> {
        unsafe { msg_send![self, parentTexture] }
    }

    pub fn parent_relative_level(&self) -> NSUInteger {
        unsafe { msg_send![self, parentRelativeLevel] }
    }

    pub fn parent_relative_slice(&self) -> NSUInteger {
        unsafe { msg_send![self, parentRelativeSlice] }
    }

    pub fn buffer(&self) -> Option<&BufferRef> {
        unsafe { msg_send![self, buffer] }
    }

    pub fn buffer_offset(&self) -> NSUInteger {
        unsafe { msg_send![self, bufferOffset] }
    }

    pub fn buffer_stride(&self) -> NSUInteger {
        unsafe { msg_send![self, bufferBytesPerRow] }
    }

    pub fn texture_type(&self) -> MTLTextureType {
        unsafe { msg_send![self, textureType] }
    }

    pub fn pixel_format(&self) -> MTLPixelFormat {
        unsafe { msg_send![self, pixelFormat] }
    }

    pub fn width(&self) -> NSUInteger {
        unsafe { msg_send![self, width] }
    }

    pub fn height(&self) -> NSUInteger {
        unsafe { msg_send![self, height] }
    }

    pub fn depth(&self) -> NSUInteger {
        unsafe { msg_send![self, depth] }
    }

    pub fn mipmap_level_count(&self) -> NSUInteger {
        unsafe { msg_send![self, mipmapLevelCount] }
    }

    pub fn sample_count(&self) -> NSUInteger {
        unsafe { msg_send![self, sampleCount] }
    }

    pub fn array_length(&self) -> NSUInteger {
        unsafe { msg_send![self, arrayLength] }
    }

    pub fn usage(&self) -> MTLTextureUsage {
        unsafe { msg_send![self, usage] }
    }

    /// [framebufferOnly Apple Docs](https://developer.apple.com/documentation/metal/mtltexture/1515749-framebufferonly?language=objc)
    pub fn framebuffer_only(&self) -> bool {
        unsafe { msg_send_bool![self, isFramebufferOnly] }
    }

    pub fn get_bytes(
        &self,
        bytes: *mut std::ffi::c_void,
        stride: NSUInteger,
        region: MTLRegion,
        mipmap_level: NSUInteger,
    ) {
        unsafe {
            msg_send![self, getBytes:bytes
                         bytesPerRow:stride
                          fromRegion:region
                         mipmapLevel:mipmap_level]
        }
    }

    pub fn get_bytes_in_slice(
        &self,
        bytes: *mut std::ffi::c_void,
        stride: NSUInteger,
        image_stride: NSUInteger,
        region: MTLRegion,
        mipmap_level: NSUInteger,
        slice: NSUInteger,
    ) {
        unsafe {
            msg_send![self, getBytes:bytes
                         bytesPerRow:stride
                       bytesPerImage:image_stride
                          fromRegion:region
                         mipmapLevel:mipmap_level
                               slice:slice]
        }
    }

    pub fn replace_region(
        &self,
        region: MTLRegion,
        mipmap_level: NSUInteger,
        bytes: *const std::ffi::c_void,
        stride: NSUInteger,
    ) {
        unsafe {
            msg_send![self, replaceRegion:region
                              mipmapLevel:mipmap_level
                                withBytes:bytes
                              bytesPerRow:stride]
        }
    }

    pub fn replace_region_in_slice(
        &self,
        region: MTLRegion,
        mipmap_level: NSUInteger,
        slice: NSUInteger,
        bytes: *const std::ffi::c_void,
        stride: NSUInteger,
        image_stride: NSUInteger,
    ) {
        unsafe {
            msg_send![self, replaceRegion:region
                              mipmapLevel:mipmap_level
                                    slice:slice
                                withBytes:bytes
                              bytesPerRow:stride
                            bytesPerImage:image_stride]
        }
    }

    pub fn new_texture_view(&self, pixel_format: MTLPixelFormat) -> Texture {
        unsafe { msg_send![self, newTextureViewWithPixelFormat: pixel_format] }
    }

    pub fn new_texture_view_from_slice(
        &self,
        pixel_format: MTLPixelFormat,
        texture_type: MTLTextureType,
        mipmap_levels: crate::NSRange,
        slices: crate::NSRange,
    ) -> Texture {
        unsafe {
            msg_send![self, newTextureViewWithPixelFormat:pixel_format
                                                textureType:texture_type
                                                     levels:mipmap_levels
                                                     slices:slices]
        }
    }

    pub fn gpu_resource_id(&self) -> MTLResourceID {
        unsafe { msg_send![self, gpuResourceID] }
    }
}
