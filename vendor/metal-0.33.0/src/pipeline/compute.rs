// Copyright 2017 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use super::*;

use objc::runtime::{NO, YES};

/// See <https://developer.apple.com/documentation/metal/mtlattributeformat>
#[repr(u64)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum MTLAttributeFormat {
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

/// See <https://developer.apple.com/documentation/metal/mtlstepfunction>
#[repr(u64)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum MTLStepFunction {
    Constant = 0,
    PerInstance = 1,
    PerPatch = 2,
    PerPatchControlPoint = 3,
    PerVertex = 4,
    ThreadPositionInGridX = 5,
    ThreadPositionInGridXIndexed = 6,
    ThreadPositionInGridY = 7,
    ThreadPositionInGridYIndexed = 8,
}

/// See <https://developer.apple.com/documentation/metal/mtlcomputepipelinedescriptor>
pub enum MTLComputePipelineDescriptor {}

foreign_obj_type! {
    type CType = MTLComputePipelineDescriptor;
    pub struct ComputePipelineDescriptor;
}

impl ComputePipelineDescriptor {
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLComputePipelineDescriptor);
            msg_send![class, new]
        }
    }
}

impl ComputePipelineDescriptorRef {
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

    pub fn compute_function(&self) -> Option<&FunctionRef> {
        unsafe { msg_send![self, computeFunction] }
    }

    pub fn set_compute_function(&self, function: Option<&FunctionRef>) {
        unsafe { msg_send![self, setComputeFunction: function] }
    }

    pub fn thread_group_size_is_multiple_of_thread_execution_width(&self) -> bool {
        unsafe { msg_send_bool![self, threadGroupSizeIsMultipleOfThreadExecutionWidth] }
    }

    pub fn set_thread_group_size_is_multiple_of_thread_execution_width(
        &self,
        size_is_multiple_of_width: bool,
    ) {
        unsafe {
            msg_send![
                self,
                setThreadGroupSizeIsMultipleOfThreadExecutionWidth: size_is_multiple_of_width
            ]
        }
    }

    /// API_AVAILABLE(macos(10.14), ios(12.0));
    pub fn max_total_threads_per_threadgroup(&self) -> NSUInteger {
        unsafe { msg_send![self, maxTotalThreadsPerThreadgroup] }
    }

    /// API_AVAILABLE(macos(10.14), ios(12.0));
    pub fn set_max_total_threads_per_threadgroup(&self, max_total_threads: NSUInteger) {
        unsafe { msg_send![self, setMaxTotalThreadsPerThreadgroup: max_total_threads] }
    }

    /// API_AVAILABLE(ios(13.0),macos(11.0));
    pub fn support_indirect_command_buffers(&self) -> bool {
        unsafe { msg_send_bool![self, supportIndirectCommandBuffers] }
    }

    /// API_AVAILABLE(ios(13.0),macos(11.0));
    pub fn set_support_indirect_command_buffers(&self, support: bool) {
        unsafe { msg_send![self, setSupportIndirectCommandBuffers: support] }
    }

    /// API_AVAILABLE(macos(11.0), ios(14.0));
    pub fn support_adding_binary_functions(&self) -> bool {
        unsafe { msg_send_bool![self, supportAddingBinaryFunctions] }
    }

    /// API_AVAILABLE(macos(11.0), ios(14.0));
    pub fn set_support_adding_binary_functions(&self, support: bool) {
        unsafe { msg_send![self, setSupportAddingBinaryFunctions: support] }
    }

    /// API_AVAILABLE(macos(11.0), ios(14.0));
    pub fn max_call_stack_depth(&self) -> NSUInteger {
        unsafe { msg_send![self, maxCallStackDepth] }
    }

    /// API_AVAILABLE(macos(11.0), ios(14.0));
    pub fn set_max_call_stack_depth(&self, depth: NSUInteger) {
        unsafe { msg_send![self, setMaxCallStackDepth: depth] }
    }

    /// API_AVAILABLE(macos(11.0), ios(14.0));
    /// Marshal to Rust Vec
    pub fn insert_libraries(&self) -> Vec<DynamicLibrary> {
        unsafe {
            let libraries: *mut Object = msg_send![self, insertLibraries];
            let count: NSUInteger = msg_send![libraries, count];
            (0..count)
                .map(|i| {
                    let lib = msg_send![libraries, objectAtIndex: i];
                    DynamicLibrary::from_ptr(lib)
                })
                .collect()
        }
    }

    /// Marshal from Rust slice
    pub fn set_insert_libraries(&self, libraries: &[&DynamicLibraryRef]) {
        let ns_array = Array::<DynamicLibrary>::from_slice(libraries);
        unsafe { msg_send![self, setInsertLibraries: ns_array] }
    }

    /// API_AVAILABLE(macos(11.0), ios(14.0));
    /// Marshal to Rust Vec
    pub fn binary_archives(&self) -> Vec<BinaryArchive> {
        unsafe {
            let archives: *mut Object = msg_send![self, binaryArchives];
            let count: NSUInteger = msg_send![archives, count];
            (0..count)
                .map(|i| {
                    let a = msg_send![archives, objectAtIndex: i];
                    BinaryArchive::from_ptr(a)
                })
                .collect()
        }
    }

    /// API_AVAILABLE(macos(11.0), ios(14.0));
    /// Marshal from Rust slice
    pub fn set_binary_archives(&self, archives: &[&BinaryArchiveRef]) {
        let ns_array = Array::<BinaryArchive>::from_slice(archives);
        unsafe { msg_send![self, setBinaryArchives: ns_array] }
    }

    /// API_AVAILABLE(macos(11.0), ios(14.0));
    pub fn linked_functions(&self) -> &LinkedFunctionsRef {
        unsafe { msg_send![self, linkedFunctions] }
    }

    /// API_AVAILABLE(macos(11.0), ios(14.0));
    pub fn set_linked_functions(&self, functions: &LinkedFunctionsRef) {
        unsafe { msg_send![self, setLinkedFunctions: functions] }
    }

    pub fn stage_input_descriptor(&self) -> Option<&StageInputOutputDescriptorRef> {
        unsafe { msg_send![self, stageInputDescriptor] }
    }

    pub fn set_stage_input_descriptor(&self, descriptor: Option<&StageInputOutputDescriptorRef>) {
        unsafe { msg_send![self, setStageInputDescriptor: descriptor] }
    }

    pub fn buffers(&self) -> Option<&PipelineBufferDescriptorArrayRef> {
        unsafe { msg_send![self, buffers] }
    }

    pub fn reset(&self) {
        unsafe { msg_send![self, reset] }
    }
}

/// See <https://developer.apple.com/documentation/metal/mtlcomputepipelinestate>
pub enum MTLComputePipelineState {}

foreign_obj_type! {
    type CType = MTLComputePipelineState;
    pub struct ComputePipelineState;
}

impl ComputePipelineStateRef {
    pub fn label(&self) -> &str {
        unsafe {
            let label = msg_send![self, label];
            crate::nsstring_as_str(label)
        }
    }

    pub fn max_total_threads_per_threadgroup(&self) -> NSUInteger {
        unsafe { msg_send![self, maxTotalThreadsPerThreadgroup] }
    }

    pub fn thread_execution_width(&self) -> NSUInteger {
        unsafe { msg_send![self, threadExecutionWidth] }
    }

    pub fn static_threadgroup_memory_length(&self) -> NSUInteger {
        unsafe { msg_send![self, staticThreadgroupMemoryLength] }
    }

    /// Only available on (ios(11.0), macos(11.0), macCatalyst(14.0)) NOT available on (tvos)
    pub fn imageblock_memory_length_for_dimensions(&self, dimensions: MTLSize) -> NSUInteger {
        unsafe { msg_send![self, imageblockMemoryLengthForDimensions: dimensions] }
    }

    /// Only available on (ios(13.0), macos(11.0))
    pub fn support_indirect_command_buffers(&self) -> bool {
        unsafe { msg_send_bool![self, supportIndirectCommandBuffers] }
    }

    /// Only available on (macos(11.0), ios(14.0))
    pub fn function_handle_with_function(
        &self,
        function: &FunctionRef,
    ) -> Option<&FunctionHandleRef> {
        unsafe { msg_send![self, functionHandleWithFunction: function] }
    }

    // API_AVAILABLE(macos(11.0), ios(14.0));
    // TODO: newComputePipelineStateWithAdditionalBinaryFunctions
    // - (nullable id <MTLComputePipelineState>)newComputePipelineStateWithAdditionalBinaryFunctions:(nonnull NSArray<id<MTLFunction>> *)functions error:(__autoreleasing NSError **)error

    // API_AVAILABLE(macos(11.0), ios(14.0));
    pub fn new_visible_function_table_with_descriptor(
        &self,
        descriptor: &VisibleFunctionTableDescriptorRef,
    ) -> VisibleFunctionTable {
        unsafe { msg_send![self, newVisibleFunctionTableWithDescriptor: descriptor ] }
    }

    /// Only available on (macos(11.0), ios(14.0))
    pub fn new_intersection_function_table_with_descriptor(
        &self,
        descriptor: &IntersectionFunctionTableDescriptorRef,
    ) -> IntersectionFunctionTable {
        unsafe { msg_send![self, newIntersectionFunctionTableWithDescriptor: descriptor] }
    }
}

/// See <https://developer.apple.com/documentation/metal/mtlstageinputoutputdescriptor>
pub enum MTLStageInputOutputDescriptor {}

foreign_obj_type! {
    type CType = MTLStageInputOutputDescriptor;
    pub struct StageInputOutputDescriptor;
}

impl StageInputOutputDescriptor {
    pub fn new<'a>() -> &'a StageInputOutputDescriptorRef {
        unsafe {
            let class = class!(MTLStageInputOutputDescriptor);
            msg_send![class, stageInputOutputDescriptor]
        }
    }
}

impl StageInputOutputDescriptorRef {
    pub fn attributes(&self) -> Option<&AttributeDescriptorArrayRef> {
        unsafe { msg_send![self, attributes] }
    }

    pub fn index_buffer_index(&self) -> NSUInteger {
        unsafe { msg_send![self, indexBufferIndex] }
    }

    pub fn set_index_buffer_index(&self, idx_buffer_idx: NSUInteger) {
        unsafe { msg_send![self, setIndexBufferIndex: idx_buffer_idx] }
    }

    pub fn index_type(&self) -> MTLIndexType {
        unsafe { msg_send![self, indexType] }
    }

    pub fn set_index_type(&self, index_ty: MTLIndexType) {
        unsafe { msg_send![self, setIndexType: index_ty] }
    }

    pub fn layouts(&self) -> Option<&BufferLayoutDescriptorArrayRef> {
        unsafe { msg_send![self, layouts] }
    }

    pub fn reset(&self) {
        unsafe { msg_send![self, reset] }
    }
}

/// See <https://developer.apple.com/documentation/metal/mtlattributedescriptorarray>
pub enum MTLAttributeDescriptorArray {}

foreign_obj_type! {
    type CType = MTLAttributeDescriptorArray;
    pub struct AttributeDescriptorArray;
}

impl AttributeDescriptorArrayRef {
    pub fn object_at(&self, index: NSUInteger) -> Option<&AttributeDescriptorRef> {
        unsafe { msg_send![self, objectAtIndexedSubscript: index] }
    }

    pub fn set_object_at(&self, index: NSUInteger, buffer_desc: Option<&AttributeDescriptorRef>) {
        unsafe { msg_send![self, setObject:buffer_desc atIndexedSubscript:index] }
    }
}

/// See <https://developer.apple.com/documentation/metal/mtlattributedescriptor>
pub enum MTLAttributeDescriptor {}

foreign_obj_type! {
    type CType = MTLAttributeDescriptor;
    pub struct AttributeDescriptor;
}

impl AttributeDescriptorRef {
    pub fn buffer_index(&self) -> NSUInteger {
        unsafe { msg_send![self, bufferIndex] }
    }

    pub fn set_buffer_index(&self, buffer_index: NSUInteger) {
        unsafe { msg_send![self, setBufferIndex: buffer_index] }
    }

    pub fn format(&self) -> MTLAttributeFormat {
        unsafe { msg_send![self, format] }
    }

    pub fn set_format(&self, format: MTLAttributeFormat) {
        unsafe { msg_send![self, setFormat: format] }
    }

    pub fn offset(&self) -> NSUInteger {
        unsafe { msg_send![self, offset] }
    }

    pub fn set_offset(&self, offset: NSUInteger) {
        unsafe { msg_send![self, setOffset: offset] }
    }
}

/// See <https://developer.apple.com/documentation/metal/mtlbufferlayoutdescriptorarray>
pub enum MTLBufferLayoutDescriptorArray {}

foreign_obj_type! {
    type CType = MTLBufferLayoutDescriptorArray;
    pub struct BufferLayoutDescriptorArray;
}

impl BufferLayoutDescriptorArrayRef {
    pub fn object_at(&self, index: NSUInteger) -> Option<&BufferLayoutDescriptorRef> {
        unsafe { msg_send![self, objectAtIndexedSubscript: index] }
    }

    pub fn set_object_at(
        &self,
        index: NSUInteger,
        buffer_desc: Option<&BufferLayoutDescriptorRef>,
    ) {
        unsafe { msg_send![self, setObject:buffer_desc atIndexedSubscript:index] }
    }
}

/// See <https://developer.apple.com/documentation/metal/mtlbufferlayoutdescriptor>
pub enum MTLBufferLayoutDescriptor {}

foreign_obj_type! {
    type CType = MTLBufferLayoutDescriptor;
    pub struct BufferLayoutDescriptor;
}

impl BufferLayoutDescriptorRef {
    pub fn step_function(&self) -> MTLStepFunction {
        unsafe { msg_send![self, stepFunction] }
    }

    pub fn set_step_function(&self, step_function: MTLStepFunction) {
        unsafe { msg_send![self, setStepFunction: step_function] }
    }

    pub fn step_rate(&self) -> NSUInteger {
        unsafe { msg_send![self, stepRate] }
    }

    pub fn set_step_rate(&self, step_rate: NSUInteger) {
        unsafe { msg_send![self, setStepRate: step_rate] }
    }

    pub fn stride(&self) -> NSUInteger {
        unsafe { msg_send![self, stride] }
    }

    pub fn set_stride(&self, stride: NSUInteger) {
        unsafe { msg_send![self, setStride: stride] }
    }
}
