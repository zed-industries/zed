// Copyright 2016 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use super::NSUInteger;

/// See <https://developer.apple.com/documentation/metal/mtlvertexformat>
#[repr(u64)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MTLVertexFormat {
    Invalid = 0,
    UChar2 = 1,
    UChar3 = 2,
    UChar4 = 3,
    Char2 = 4,
    Char3 = 5,
    Char4 = 6,
    UChar2Normalized = 7,
    UChar3Normalized = 8,
    UChar4Normalized = 9,
    Char2Normalized = 10,
    Char3Normalized = 11,
    Char4Normalized = 12,
    UShort2 = 13,
    UShort3 = 14,
    UShort4 = 15,
    Short2 = 16,
    Short3 = 17,
    Short4 = 18,
    UShort2Normalized = 19,
    UShort3Normalized = 20,
    UShort4Normalized = 21,
    Short2Normalized = 22,
    Short3Normalized = 23,
    Short4Normalized = 24,
    Half2 = 25,
    Half3 = 26,
    Half4 = 27,
    Float = 28,
    Float2 = 29,
    Float3 = 30,
    Float4 = 31,
    Int = 32,
    Int2 = 33,
    Int3 = 34,
    Int4 = 35,
    UInt = 36,
    UInt2 = 37,
    UInt3 = 38,
    UInt4 = 39,
    Int1010102Normalized = 40,
    UInt1010102Normalized = 41,
    UChar4Normalized_BGRA = 42,
    UChar = 45,
    Char = 46,
    UCharNormalized = 47,
    CharNormalized = 48,
    UShort = 49,
    Short = 50,
    UShortNormalized = 51,
    ShortNormalized = 52,
    Half = 53,
}

/// See <https://developer.apple.com/documentation/metal/mtlvertexstepfunction>
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MTLVertexStepFunction {
    Constant = 0,
    PerVertex = 1,
    PerInstance = 2,
    PerPatch = 3,
    PerPatchControlPoint = 4,
}

/// See <https://developer.apple.com/documentation/metal/mtlvertexbufferlayoutdescriptor>
pub enum MTLVertexBufferLayoutDescriptor {}

foreign_obj_type! {
    type CType = MTLVertexBufferLayoutDescriptor;
    pub struct VertexBufferLayoutDescriptor;
}

impl VertexBufferLayoutDescriptor {
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLVertexBufferLayoutDescriptor);
            msg_send![class, new]
        }
    }
}

impl VertexBufferLayoutDescriptorRef {
    pub fn stride(&self) -> NSUInteger {
        unsafe { msg_send![self, stride] }
    }

    pub fn set_stride(&self, stride: NSUInteger) {
        unsafe { msg_send![self, setStride: stride] }
    }

    pub fn step_function(&self) -> MTLVertexStepFunction {
        unsafe { msg_send![self, stepFunction] }
    }

    pub fn set_step_function(&self, func: MTLVertexStepFunction) {
        unsafe { msg_send![self, setStepFunction: func] }
    }

    pub fn step_rate(&self) -> NSUInteger {
        unsafe { msg_send![self, stepRate] }
    }

    pub fn set_step_rate(&self, step_rate: NSUInteger) {
        unsafe { msg_send![self, setStepRate: step_rate] }
    }
}

/// See <https://developer.apple.com/documentation/metal/mtlvertexbufferlayoutdescriptorarray>
pub enum MTLVertexBufferLayoutDescriptorArray {}

foreign_obj_type! {
    type CType = MTLVertexBufferLayoutDescriptorArray;
    pub struct VertexBufferLayoutDescriptorArray;
}

impl VertexBufferLayoutDescriptorArrayRef {
    pub fn object_at(&self, index: NSUInteger) -> Option<&VertexBufferLayoutDescriptorRef> {
        unsafe { msg_send![self, objectAtIndexedSubscript: index] }
    }

    pub fn set_object_at(
        &self,
        index: NSUInteger,
        layout: Option<&VertexBufferLayoutDescriptorRef>,
    ) {
        unsafe {
            msg_send![self, setObject:layout
                   atIndexedSubscript:index]
        }
    }
}

/// See <https://developer.apple.com/documentation/metal/mtlvertexattributedescriptor>
pub enum MTLVertexAttributeDescriptor {}

foreign_obj_type! {
    type CType = MTLVertexAttributeDescriptor;
    pub struct VertexAttributeDescriptor;
}

impl VertexAttributeDescriptor {
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLVertexAttributeDescriptor);
            msg_send![class, new]
        }
    }
}

impl VertexAttributeDescriptorRef {
    pub fn format(&self) -> MTLVertexFormat {
        unsafe { msg_send![self, format] }
    }

    pub fn set_format(&self, format: MTLVertexFormat) {
        unsafe { msg_send![self, setFormat: format] }
    }

    pub fn offset(&self) -> NSUInteger {
        unsafe { msg_send![self, offset] }
    }

    pub fn set_offset(&self, offset: NSUInteger) {
        unsafe { msg_send![self, setOffset: offset] }
    }

    pub fn buffer_index(&self) -> NSUInteger {
        unsafe { msg_send![self, bufferIndex] }
    }

    pub fn set_buffer_index(&self, index: NSUInteger) {
        unsafe { msg_send![self, setBufferIndex: index] }
    }
}

/// See <https://developer.apple.com/documentation/metal/mtlvertexattributedescriptorarray>
pub enum MTLVertexAttributeDescriptorArray {}

foreign_obj_type! {
    type CType = MTLVertexAttributeDescriptorArray;
    pub struct VertexAttributeDescriptorArray;
}

impl VertexAttributeDescriptorArrayRef {
    pub fn object_at(&self, index: NSUInteger) -> Option<&VertexAttributeDescriptorRef> {
        unsafe { msg_send![self, objectAtIndexedSubscript: index] }
    }

    pub fn set_object_at(
        &self,
        index: NSUInteger,
        attribute: Option<&VertexAttributeDescriptorRef>,
    ) {
        unsafe {
            msg_send![self, setObject:attribute
                   atIndexedSubscript:index]
        }
    }
}

/// See <https://developer.apple.com/documentation/metal/mtlvertexdescriptor>
pub enum MTLVertexDescriptor {}

foreign_obj_type! {
    type CType = MTLVertexDescriptor;
    pub struct VertexDescriptor;
}

impl VertexDescriptor {
    pub fn new<'a>() -> &'a VertexDescriptorRef {
        unsafe {
            let class = class!(MTLVertexDescriptor);
            msg_send![class, vertexDescriptor]
        }
    }
}

impl VertexDescriptorRef {
    pub fn layouts(&self) -> &VertexBufferLayoutDescriptorArrayRef {
        unsafe { msg_send![self, layouts] }
    }

    pub fn attributes(&self) -> &VertexAttributeDescriptorArrayRef {
        unsafe { msg_send![self, attributes] }
    }

    #[cfg(feature = "private")]
    pub unsafe fn serialize_descriptor(&self) -> *mut std::ffi::c_void {
        msg_send![self, newSerializedDescriptor]
    }

    pub fn reset(&self) {
        unsafe { msg_send![self, reset] }
    }
}
