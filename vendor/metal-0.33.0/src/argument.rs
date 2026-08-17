// Copyright 2017 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use super::{MTLTextureType, NSUInteger};
use objc::runtime::{NO, YES};

/// See <https://developer.apple.com/documentation/metal/mtldatatype>
#[repr(u64)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MTLDataType {
    None = 0,

    Struct = 1,
    Array = 2,

    Float = 3,
    Float2 = 4,
    Float3 = 5,
    Float4 = 6,

    Float2x2 = 7,
    Float2x3 = 8,
    Float2x4 = 9,

    Float3x2 = 10,
    Float3x3 = 11,
    Float3x4 = 12,

    Float4x2 = 13,
    Float4x3 = 14,
    Float4x4 = 15,

    Half = 16,
    Half2 = 17,
    Half3 = 18,
    Half4 = 19,

    Half2x2 = 20,
    Half2x3 = 21,
    Half2x4 = 22,

    Half3x2 = 23,
    Half3x3 = 24,
    Half3x4 = 25,

    Half4x2 = 26,
    Half4x3 = 27,
    Half4x4 = 28,

    Int = 29,
    Int2 = 30,
    Int3 = 31,
    Int4 = 32,

    UInt = 33,
    UInt2 = 34,
    UInt3 = 35,
    UInt4 = 36,

    Short = 37,
    Short2 = 38,
    Short3 = 39,
    Short4 = 40,

    UShort = 41,
    UShort2 = 42,
    UShort3 = 43,
    UShort4 = 44,

    Char = 45,
    Char2 = 46,
    Char3 = 47,
    Char4 = 48,

    UChar = 49,
    UChar2 = 50,
    UChar3 = 51,
    UChar4 = 52,

    Bool = 53,
    Bool2 = 54,
    Bool3 = 55,
    Bool4 = 56,

    Texture = 58,
    Sampler = 59,
    Pointer = 60,
    R8Unorm = 62,
    R8Snorm = 63,
    R16Unorm = 64,
    R16Snorm = 65,
    RG8Unorm = 66,
    RG8Snorm = 67,
    RG16Unorm = 68,
    RG16Snorm = 69,
    RGBA8Unorm = 70,
    RGBA8Unorm_sRGB = 71,
    RGBA8Snorm = 72,
    RGBA16Unorm = 73,
    RGBA16Snorm = 74,
    RGB10A2Unorm = 75,
    RG11B10Float = 76,
    RGB9E5Float = 77,

    RenderPipeline = 78,
    ComputePipeline = 79,
    IndirectCommandBuffer = 80,

    Long = 81,
    Long2 = 82,
    Long3 = 83,
    Long4 = 84,

    ULong = 85,
    ULong2 = 86,
    ULong3 = 87,
    ULong4 = 88,

    VisibleFunctionTable = 115,
    IntersectionFunctionTable = 116,
    PrimitiveAccelerationStructure = 117,
    InstanceAccelerationStructure = 118,

    BFloat = 121,
    BFloat2 = 122,
    BFloat3 = 123,
    BFloat4 = 124,
}

/// See <https://developer.apple.com/documentation/metal/mtlargumenttype>
#[repr(u64)]
#[deprecated(
    note = "Since: iOS 8.0–16.0, iPadOS 8.0–16.0, macOS 10.11–13.0, Mac Catalyst 13.1–16.0, tvOS 9.0–16.0"
)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MTLArgumentType {
    Buffer = 0,
    ThreadgroupMemory = 1,
    Texture = 2,
    Sampler = 3,
    ImageblockData = 16,
    Imageblock = 17,
}

/// See <https://developer.apple.com/documentation/metal/mtlargumentaccess>
#[repr(u64)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MTLArgumentAccess {
    ReadOnly = 0,
    ReadWrite = 1,
    WriteOnly = 2,
}

/// See <https://developer.apple.com/documentation/metal/mtlstructmember>
pub enum MTLStructMember {}

foreign_obj_type! {
    type CType = MTLStructMember;
    pub struct StructMember;
}

impl StructMemberRef {
    pub fn name(&self) -> &str {
        unsafe {
            let name = msg_send![self, name];
            crate::nsstring_as_str(name)
        }
    }

    pub fn offset(&self) -> NSUInteger {
        unsafe { msg_send![self, offset] }
    }

    pub fn data_type(&self) -> MTLDataType {
        unsafe { msg_send![self, dataType] }
    }

    pub fn struct_type(&self) -> MTLStructType {
        unsafe { msg_send![self, structType] }
    }

    pub fn array_type(&self) -> MTLArrayType {
        unsafe { msg_send![self, arrayType] }
    }
}

pub enum MTLStructMemberArray {}

foreign_obj_type! {
    type CType = MTLStructMemberArray;
    pub struct StructMemberArray;
}

impl StructMemberArrayRef {
    pub fn object_at(&self, index: NSUInteger) -> Option<&StructMemberRef> {
        unsafe { msg_send![self, objectAtIndexedSubscript: index] }
    }

    pub fn count(&self) -> NSUInteger {
        unsafe { msg_send![self, count] }
    }
}

/// See <https://developer.apple.com/documentation/metal/mtlstructtype>
pub enum MTLStructType {}

foreign_obj_type! {
    type CType = MTLStructType;
    pub struct StructType;
}

impl StructTypeRef {
    pub fn members(&self) -> &StructMemberArrayRef {
        unsafe { msg_send![self, members] }
    }

    pub fn member_from_name(&self, name: &str) -> Option<&StructMemberRef> {
        let nsname = crate::nsstring_from_str(name);

        unsafe { msg_send![self, memberByName: nsname] }
    }
}

/// See <https://developer.apple.com/documentation/metal/mtlarraytype>
pub enum MTLArrayType {}

foreign_obj_type! {
    type CType = MTLArrayType;
    pub struct ArrayType;
}

impl ArrayTypeRef {
    pub fn array_length(&self) -> NSUInteger {
        unsafe { msg_send![self, arrayLength] }
    }

    pub fn stride(&self) -> NSUInteger {
        unsafe { msg_send![self, stride] }
    }

    pub fn element_type(&self) -> MTLDataType {
        unsafe { msg_send![self, elementType] }
    }

    pub fn element_struct_type(&self) -> MTLStructType {
        unsafe { msg_send![self, elementStructType] }
    }

    pub fn element_array_type(&self) -> MTLArrayType {
        unsafe { msg_send![self, elementArrayType] }
    }
}

/// <https://developer.apple.com/documentation/metal/mtlargument>
#[deprecated(
    note = "Since iOS 8.0–16.0, iPadOS 8.0–16.0, macOS 10.11–13.0, Mac Catalyst 13.1–16.0, tvOS 9.0–16.0"
)]
pub enum MTLArgument {}

foreign_obj_type! {
    type CType = MTLArgument;
    pub struct Argument;
}

impl ArgumentRef {
    pub fn name(&self) -> &str {
        unsafe {
            let name = msg_send![self, name];
            crate::nsstring_as_str(name)
        }
    }

    pub fn type_(&self) -> MTLArgumentType {
        unsafe { msg_send![self, type] }
    }

    pub fn access(&self) -> MTLArgumentAccess {
        unsafe { msg_send![self, access] }
    }

    pub fn index(&self) -> NSUInteger {
        unsafe { msg_send![self, index] }
    }

    pub fn is_active(&self) -> bool {
        unsafe { msg_send_bool![self, isActive] }
    }

    pub fn buffer_alignment(&self) -> NSUInteger {
        unsafe { msg_send![self, bufferAlignment] }
    }

    pub fn buffer_data_size(&self) -> NSUInteger {
        unsafe { msg_send![self, bufferDataSize] }
    }

    pub fn buffer_data_type(&self) -> MTLDataType {
        unsafe { msg_send![self, bufferDataType] }
    }

    pub fn buffer_struct_type(&self) -> &StructTypeRef {
        unsafe { msg_send![self, bufferStructType] }
    }

    pub fn threadgroup_memory_alignment(&self) -> NSUInteger {
        unsafe { msg_send![self, threadgroupMemoryAlignment] }
    }

    pub fn threadgroup_memory_data_size(&self) -> NSUInteger {
        unsafe { msg_send![self, threadgroupMemoryDataSize] }
    }

    pub fn texture_type(&self) -> MTLTextureType {
        unsafe { msg_send![self, textureType] }
    }

    pub fn texture_data_type(&self) -> MTLDataType {
        unsafe { msg_send![self, textureDataType] }
    }
}

/// See <https://developer.apple.com/documentation/metal/mtlargumentdescriptor>
pub enum MTLArgumentDescriptor {}

foreign_obj_type! {
    type CType = MTLArgumentDescriptor;
    pub struct ArgumentDescriptor;
}

impl ArgumentDescriptor {
    pub fn new<'a>() -> &'a ArgumentDescriptorRef {
        unsafe {
            let class = class!(MTLArgumentDescriptor);
            msg_send![class, argumentDescriptor]
        }
    }
}

impl ArgumentDescriptorRef {
    pub fn set_data_type(&self, ty: MTLDataType) {
        unsafe { msg_send![self, setDataType: ty] }
    }

    pub fn set_index(&self, index: NSUInteger) {
        unsafe { msg_send![self, setIndex: index] }
    }

    pub fn set_access(&self, access: MTLArgumentAccess) {
        unsafe { msg_send![self, setAccess: access] }
    }

    pub fn set_array_length(&self, length: NSUInteger) {
        unsafe { msg_send![self, setArrayLength: length] }
    }

    pub fn set_texture_type(&self, ty: MTLTextureType) {
        unsafe { msg_send![self, setTextureType: ty] }
    }
}
