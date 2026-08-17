// Copyright 2017 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use super::*;

use objc::runtime::{NO, YES};

/// See <https://developer.apple.com/documentation/metal/mtlblendfactor>
#[repr(u64)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum MTLBlendFactor {
    Zero = 0,
    One = 1,
    SourceColor = 2,
    OneMinusSourceColor = 3,
    SourceAlpha = 4,
    OneMinusSourceAlpha = 5,
    DestinationColor = 6,
    OneMinusDestinationColor = 7,
    DestinationAlpha = 8,
    OneMinusDestinationAlpha = 9,
    SourceAlphaSaturated = 10,
    BlendColor = 11,
    OneMinusBlendColor = 12,
    BlendAlpha = 13,
    OneMinusBlendAlpha = 14,
    Source1Color = 15,
    OneMinusSource1Color = 16,
    Source1Alpha = 17,
    OneMinusSource1Alpha = 18,
}

/// See <https://developer.apple.com/documentation/metal/mtlblendoperation>
#[repr(u64)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum MTLBlendOperation {
    Add = 0,
    Subtract = 1,
    ReverseSubtract = 2,
    Min = 3,
    Max = 4,
}

bitflags::bitflags! {
    /// See <https://developer.apple.com/documentation/metal/mtlcolorwritemask>
    #[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub struct MTLColorWriteMask: NSUInteger {
        const None  = 0;
        const Red   = 0x1 << 3;
        const Green = 0x1 << 2;
        const Blue  = 0x1 << 1;
        const Alpha = 0x1 << 0;
        const All   = 0xf;
    }
}

/// See <https://developer.apple.com/documentation/metal/mtlprimitivetopologyclass>
#[repr(u64)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum MTLPrimitiveTopologyClass {
    Unspecified = 0,
    Point = 1,
    Line = 2,
    Triangle = 3,
}

// TODO: MTLTessellationPartitionMode
// TODO: MTLTessellationFactorStepFunction
// TODO: MTLTessellationFactorFormat
// TODO: MTLTessellationControlPointIndexType

/// See <https://developer.apple.com/documentation/metal/mtlrenderpipelinecolorattachmentdescriptor>
pub enum MTLRenderPipelineColorAttachmentDescriptor {}

foreign_obj_type! {
    type CType = MTLRenderPipelineColorAttachmentDescriptor;
    pub struct RenderPipelineColorAttachmentDescriptor;
}

impl RenderPipelineColorAttachmentDescriptorRef {
    pub fn pixel_format(&self) -> MTLPixelFormat {
        unsafe { msg_send![self, pixelFormat] }
    }

    pub fn set_pixel_format(&self, pixel_format: MTLPixelFormat) {
        unsafe { msg_send![self, setPixelFormat: pixel_format] }
    }

    pub fn is_blending_enabled(&self) -> bool {
        unsafe { msg_send_bool![self, isBlendingEnabled] }
    }

    pub fn set_blending_enabled(&self, enabled: bool) {
        unsafe { msg_send![self, setBlendingEnabled: enabled] }
    }

    pub fn source_rgb_blend_factor(&self) -> MTLBlendFactor {
        unsafe { msg_send![self, sourceRGBBlendFactor] }
    }

    pub fn set_source_rgb_blend_factor(&self, blend_factor: MTLBlendFactor) {
        unsafe { msg_send![self, setSourceRGBBlendFactor: blend_factor] }
    }

    pub fn destination_rgb_blend_factor(&self) -> MTLBlendFactor {
        unsafe { msg_send![self, destinationRGBBlendFactor] }
    }

    pub fn set_destination_rgb_blend_factor(&self, blend_factor: MTLBlendFactor) {
        unsafe { msg_send![self, setDestinationRGBBlendFactor: blend_factor] }
    }

    pub fn rgb_blend_operation(&self) -> MTLBlendOperation {
        unsafe { msg_send![self, rgbBlendOperation] }
    }

    pub fn set_rgb_blend_operation(&self, blend_operation: MTLBlendOperation) {
        unsafe { msg_send![self, setRgbBlendOperation: blend_operation] }
    }

    pub fn source_alpha_blend_factor(&self) -> MTLBlendFactor {
        unsafe { msg_send![self, sourceAlphaBlendFactor] }
    }

    pub fn set_source_alpha_blend_factor(&self, blend_factor: MTLBlendFactor) {
        unsafe { msg_send![self, setSourceAlphaBlendFactor: blend_factor] }
    }

    pub fn destination_alpha_blend_factor(&self) -> MTLBlendFactor {
        unsafe { msg_send![self, destinationAlphaBlendFactor] }
    }

    pub fn set_destination_alpha_blend_factor(&self, blend_factor: MTLBlendFactor) {
        unsafe { msg_send![self, setDestinationAlphaBlendFactor: blend_factor] }
    }

    pub fn alpha_blend_operation(&self) -> MTLBlendOperation {
        unsafe { msg_send![self, alphaBlendOperation] }
    }

    pub fn set_alpha_blend_operation(&self, blend_operation: MTLBlendOperation) {
        unsafe { msg_send![self, setAlphaBlendOperation: blend_operation] }
    }

    pub fn write_mask(&self) -> MTLColorWriteMask {
        unsafe { msg_send![self, writeMask] }
    }

    pub fn set_write_mask(&self, mask: MTLColorWriteMask) {
        unsafe { msg_send![self, setWriteMask: mask] }
    }
}

/// See <https://developer.apple.com/documentation/metal/mtlrenderpipelinereflection>
pub enum MTLRenderPipelineReflection {}

foreign_obj_type! {
    type CType = MTLRenderPipelineReflection;
    pub struct RenderPipelineReflection;
}

impl RenderPipelineReflection {
    #[cfg(feature = "private")]
    pub unsafe fn new(
        vertex_data: *mut std::ffi::c_void,
        fragment_data: *mut std::ffi::c_void,
        vertex_desc: *mut std::ffi::c_void,
        device: &DeviceRef,
        options: u64,
        flags: u64,
    ) -> Self {
        let class = class!(MTLRenderPipelineReflection);
        let this: RenderPipelineReflection = msg_send![class, alloc];
        let this_alias: *mut Object = msg_send![this.as_ref(), initWithVertexData:vertex_data
                                                                fragmentData:fragment_data
                                                serializedVertexDescriptor:vertex_desc
                                                                    device:device
                                                                    options:options
                                                                        flags:flags];
        if this_alias.is_null() {
            panic!("[MTLRenderPipelineReflection init] failed");
        }
        this
    }
}

impl RenderPipelineReflectionRef {
    /// An array of objects that describe the arguments of a fragment function.
    pub fn fragment_arguments(&self) -> &ArgumentArrayRef {
        unsafe { msg_send![self, fragmentArguments] }
    }

    /// An array of objects that describe the arguments of a vertex function.
    pub fn vertex_arguments(&self) -> &ArgumentArrayRef {
        unsafe { msg_send![self, vertexArguments] }
    }

    /// An array of objects that describe the arguments of a tile shading function.
    pub fn tile_arguments(&self) -> &ArgumentArrayRef {
        unsafe { msg_send![self, tileArguments] }
    }
}

/// TODO: Find documentation link.
pub enum MTLArgumentArray {}

foreign_obj_type! {
    type CType = MTLArgumentArray;
    pub struct ArgumentArray;
}

impl ArgumentArrayRef {
    pub fn object_at(&self, index: NSUInteger) -> Option<&ArgumentRef> {
        unsafe { msg_send![self, objectAtIndexedSubscript: index] }
    }

    pub fn count(&self) -> NSUInteger {
        unsafe { msg_send![self, count] }
    }
}

/// See <https://developer.apple.com/documentation/metal/mtlcomputepipelinereflection>
pub enum MTLComputePipelineReflection {}

foreign_obj_type! {
    type CType = MTLComputePipelineReflection;
    pub struct ComputePipelineReflection;
}

impl ComputePipelineReflectionRef {
    /// An array of objects that describe the arguments of a compute function.
    pub fn arguments(&self) -> &ArgumentArrayRef {
        unsafe { msg_send![self, arguments] }
    }
}

/// See <https://developer.apple.com/documentation/metal/mtlmeshrenderpipelinedescriptor>
/// Only available in (macos(13.0), ios(16.0))
pub enum MTLMeshRenderPipelineDescriptor {}

foreign_obj_type! {
    type CType = MTLMeshRenderPipelineDescriptor;
    pub struct MeshRenderPipelineDescriptor;
}

impl MeshRenderPipelineDescriptor {
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLMeshRenderPipelineDescriptor);
            msg_send![class, new]
        }
    }
}

impl MeshRenderPipelineDescriptorRef {
    pub fn color_attachments(&self) -> &RenderPipelineColorAttachmentDescriptorArrayRef {
        unsafe { msg_send![self, colorAttachments] }
    }

    pub fn depth_attachment_pixel_format(&self) -> MTLPixelFormat {
        unsafe { msg_send![self, depthAttachmentPixelFormat] }
    }

    pub fn set_depth_attachment_pixel_format(&self, pixel_format: MTLPixelFormat) {
        unsafe { msg_send![self, setDepthAttachmentPixelFormat: pixel_format] }
    }

    pub fn fragment_buffers(&self) -> Option<&PipelineBufferDescriptorArrayRef> {
        unsafe { msg_send![self, fragmentBuffers] }
    }

    pub fn fragment_function(&self) -> Option<&FunctionRef> {
        unsafe { msg_send![self, fragmentFunction] }
    }

    pub fn set_fragment_function(&self, function: Option<&FunctionRef>) {
        unsafe { msg_send![self, setFragmentFunction: function] }
    }

    pub fn is_alpha_to_coverage_enabled(&self) -> bool {
        unsafe { msg_send_bool![self, isAlphaToCoverageEnabled] }
    }

    pub fn set_alpha_to_coverage_enabled(&self, enabled: bool) {
        unsafe { msg_send![self, setAlphaToCoverageEnabled: enabled] }
    }

    pub fn is_alpha_to_one_enabled(&self) -> bool {
        unsafe { msg_send_bool![self, isAlphaToOneEnabled] }
    }

    pub fn set_alpha_to_one_enabled(&self, enabled: bool) {
        unsafe { msg_send![self, setAlphaToOneEnabled: enabled] }
    }

    pub fn is_rasterization_enabled(&self) -> bool {
        unsafe { msg_send_bool![self, isRasterizationEnabled] }
    }

    pub fn set_rasterization_enabled(&self, enabled: bool) {
        unsafe { msg_send![self, setRasterizationEnabled: enabled] }
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

    pub fn max_total_threadgroups_per_mesh_grid(&self) -> NSUInteger {
        unsafe { msg_send![self, maxTotalThreadgroupsPerMeshGrid] }
    }

    pub fn set_max_total_threadgroups_per_mesh_grid(
        &self,
        max_total_threadgroups_per_mesh_grid: NSUInteger,
    ) {
        unsafe {
            msg_send![
                self,
                setMaxTotalThreadgroupsPerMeshGrid: max_total_threadgroups_per_mesh_grid
            ]
        }
    }

    pub fn max_total_threads_per_mesh_threadgroup(&self) -> NSUInteger {
        unsafe { msg_send![self, maxTotalThreadsPerMeshThreadgroup] }
    }

    pub fn set_max_total_threads_per_mesh_threadgroup(
        &self,
        max_total_threads_per_mesh_threadgroup: NSUInteger,
    ) {
        unsafe {
            msg_send![
                self,
                setMaxTotalThreadsPerMeshThreadgroup: max_total_threads_per_mesh_threadgroup
            ]
        }
    }

    pub fn max_total_threads_per_object_threadgroup(&self) -> NSUInteger {
        unsafe { msg_send![self, maxTotalThreadsPerObjectThreadgroup] }
    }

    pub fn set_max_total_threads_per_object_threadgroup(
        &self,
        max_total_threads_per_object_threadgroup: NSUInteger,
    ) {
        unsafe {
            msg_send![
                self,
                setMaxTotalThreadsPerObjectThreadgroup: max_total_threads_per_object_threadgroup
            ]
        }
    }

    pub fn max_vertex_amplification_count(&self) -> NSUInteger {
        unsafe { msg_send![self, maxVertexAmplificationCount] }
    }

    pub fn set_max_vertex_amplification_count(&self, max_vertex_amplification_count: NSUInteger) {
        unsafe {
            msg_send![
                self,
                setMaxVertexAmplificationCount: max_vertex_amplification_count
            ]
        }
    }

    pub fn mesh_buffers(&self) -> Option<&PipelineBufferDescriptorArrayRef> {
        unsafe { msg_send![self, meshBuffers] }
    }

    pub fn mesh_function(&self) -> Option<&FunctionRef> {
        unsafe { msg_send![self, meshFunction] }
    }

    pub fn set_mesh_function(&self, function: Option<&FunctionRef>) {
        unsafe { msg_send![self, setMeshFunction: function] }
    }

    pub fn mesh_threadgroup_size_is_multiple_of_thread_execution_width(&self) -> bool {
        unsafe { msg_send_bool![self, isMeshThreadgroupSizeIsMultipleOfThreadExecutionWidth] }
    }

    pub fn set_mesh_threadgroup_size_is_multiple_of_thread_execution_width(
        &self,
        mesh_threadgroup_size_is_multiple_of_thread_execution_width: bool,
    ) {
        unsafe {
            msg_send![
                self,
                setMeshThreadgroupSizeIsMultipleOfThreadExecutionWidth:
                    mesh_threadgroup_size_is_multiple_of_thread_execution_width
            ]
        }
    }

    pub fn object_buffers(&self) -> Option<&PipelineBufferDescriptorArrayRef> {
        unsafe { msg_send![self, objectBuffers] }
    }

    pub fn object_function(&self) -> Option<&FunctionRef> {
        unsafe { msg_send![self, objectFunction] }
    }

    pub fn set_object_function(&self, function: Option<&FunctionRef>) {
        unsafe { msg_send![self, setObjectFunction: function] }
    }

    pub fn object_threadgroup_size_is_multiple_of_thread_execution_width(&self) -> bool {
        unsafe {
            msg_send_bool![
                self,
                isObjectThreadgroupSizeIsMultipleOfThreadExecutionWidth
            ]
        }
    }

    pub fn set_object_threadgroup_size_is_multiple_of_thread_execution_width(
        &self,
        object_threadgroup_size_is_multiple_of_thread_execution_width: bool,
    ) {
        unsafe {
            msg_send![
                self,
                setObjectThreadgroupSizeIsMultipleOfThreadExecutionWidth:
                    object_threadgroup_size_is_multiple_of_thread_execution_width
            ]
        }
    }

    pub fn payload_memory_length(&self) -> NSUInteger {
        unsafe { msg_send![self, payloadMemoryLength] }
    }

    pub fn set_payload_memory_length(&self, payload_memory_length: NSUInteger) {
        unsafe { msg_send![self, setPayloadMemoryLength: payload_memory_length] }
    }

    pub fn raster_sample_count(&self) -> NSUInteger {
        unsafe { msg_send![self, rasterSampleCount] }
    }

    pub fn set_raster_sample_count(&self, raster_sample_count: NSUInteger) {
        unsafe { msg_send![self, setRasterSampleCount: raster_sample_count] }
    }

    pub fn stencil_attachment_pixel_format(&self) -> MTLPixelFormat {
        unsafe { msg_send![self, stencilAttachmentPixelFormat] }
    }

    pub fn set_stencil_attachment_pixel_format(&self, pixel_format: MTLPixelFormat) {
        unsafe { msg_send![self, setStencilAttachmentPixelFormat: pixel_format] }
    }

    pub fn reset(&self) {
        unsafe { msg_send![self, reset] }
    }
}

/// See <https://developer.apple.com/documentation/metal/mtlrenderpipelinedescriptor>
pub enum MTLRenderPipelineDescriptor {}

foreign_obj_type! {
    type CType = MTLRenderPipelineDescriptor;
    pub struct RenderPipelineDescriptor;
}

impl RenderPipelineDescriptor {
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLRenderPipelineDescriptor);
            msg_send![class, new]
        }
    }
}

impl RenderPipelineDescriptorRef {
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

    pub fn vertex_function(&self) -> Option<&FunctionRef> {
        unsafe { msg_send![self, vertexFunction] }
    }

    pub fn set_vertex_function(&self, function: Option<&FunctionRef>) {
        unsafe { msg_send![self, setVertexFunction: function] }
    }

    pub fn fragment_function(&self) -> Option<&FunctionRef> {
        unsafe { msg_send![self, fragmentFunction] }
    }

    pub fn set_fragment_function(&self, function: Option<&FunctionRef>) {
        unsafe { msg_send![self, setFragmentFunction: function] }
    }

    pub fn vertex_descriptor(&self) -> Option<&VertexDescriptorRef> {
        unsafe { msg_send![self, vertexDescriptor] }
    }

    pub fn set_vertex_descriptor(&self, descriptor: Option<&VertexDescriptorRef>) {
        unsafe { msg_send![self, setVertexDescriptor: descriptor] }
    }

    /// DEPRECATED - aliases rasterSampleCount property
    pub fn sample_count(&self) -> NSUInteger {
        unsafe { msg_send![self, sampleCount] }
    }

    /// DEPRECATED - aliases rasterSampleCount property
    pub fn set_sample_count(&self, count: NSUInteger) {
        unsafe { msg_send![self, setSampleCount: count] }
    }

    pub fn raster_sample_count(&self) -> NSUInteger {
        unsafe { msg_send![self, rasterSampleCount] }
    }

    pub fn set_raster_sample_count(&self, count: NSUInteger) {
        unsafe { msg_send![self, setRasterSampleCount: count] }
    }

    pub fn max_vertex_amplification_count(&self) -> NSUInteger {
        unsafe { msg_send![self, maxVertexAmplificationCount] }
    }

    pub fn set_max_vertex_amplification_count(&self, count: NSUInteger) {
        unsafe { msg_send![self, setMaxVertexAmplificationCount: count] }
    }

    pub fn is_alpha_to_coverage_enabled(&self) -> bool {
        unsafe { msg_send_bool![self, isAlphaToCoverageEnabled] }
    }

    pub fn set_alpha_to_coverage_enabled(&self, enabled: bool) {
        unsafe { msg_send![self, setAlphaToCoverageEnabled: enabled] }
    }

    pub fn is_alpha_to_one_enabled(&self) -> bool {
        unsafe { msg_send_bool![self, isAlphaToOneEnabled] }
    }

    pub fn set_alpha_to_one_enabled(&self, enabled: bool) {
        unsafe { msg_send![self, setAlphaToOneEnabled: enabled] }
    }

    pub fn is_rasterization_enabled(&self) -> bool {
        unsafe { msg_send_bool![self, isRasterizationEnabled] }
    }

    pub fn set_rasterization_enabled(&self, enabled: bool) {
        unsafe { msg_send![self, setRasterizationEnabled: enabled] }
    }

    pub fn color_attachments(&self) -> &RenderPipelineColorAttachmentDescriptorArrayRef {
        unsafe { msg_send![self, colorAttachments] }
    }

    pub fn depth_attachment_pixel_format(&self) -> MTLPixelFormat {
        unsafe { msg_send![self, depthAttachmentPixelFormat] }
    }

    pub fn set_depth_attachment_pixel_format(&self, pixel_format: MTLPixelFormat) {
        unsafe { msg_send![self, setDepthAttachmentPixelFormat: pixel_format] }
    }

    pub fn stencil_attachment_pixel_format(&self) -> MTLPixelFormat {
        unsafe { msg_send![self, stencilAttachmentPixelFormat] }
    }

    pub fn set_stencil_attachment_pixel_format(&self, pixel_format: MTLPixelFormat) {
        unsafe { msg_send![self, setStencilAttachmentPixelFormat: pixel_format] }
    }

    pub fn input_primitive_topology(&self) -> MTLPrimitiveTopologyClass {
        unsafe { msg_send![self, inputPrimitiveTopology] }
    }

    pub fn set_input_primitive_topology(&self, topology: MTLPrimitiveTopologyClass) {
        unsafe { msg_send![self, setInputPrimitiveTopology: topology] }
    }

    #[cfg(feature = "private")]
    pub unsafe fn serialize_vertex_data(&self) -> *mut std::ffi::c_void {
        use std::ptr;
        let flags = 0;
        let err: *mut Object = ptr::null_mut();
        msg_send![self, newSerializedVertexDataWithFlags:flags
                                                    error:err]
    }

    #[cfg(feature = "private")]
    pub unsafe fn serialize_fragment_data(&self) -> *mut std::ffi::c_void {
        msg_send![self, serializeFragmentData]
    }

    pub fn support_indirect_command_buffers(&self) -> bool {
        unsafe { msg_send_bool![self, supportIndirectCommandBuffers] }
    }

    pub fn set_support_indirect_command_buffers(&self, support: bool) {
        unsafe { msg_send![self, setSupportIndirectCommandBuffers: support] }
    }

    pub fn vertex_buffers(&self) -> Option<&PipelineBufferDescriptorArrayRef> {
        unsafe { msg_send![self, vertexBuffers] }
    }

    pub fn fragment_buffers(&self) -> Option<&PipelineBufferDescriptorArrayRef> {
        unsafe { msg_send![self, fragmentBuffers] }
    }

    // TODO: tesselation stuff

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
    pub fn fragment_linked_functions(&self) -> &LinkedFunctionsRef {
        unsafe { msg_send![self, fragmentLinkedFunctions] }
    }

    /// API_AVAILABLE(macos(11.0), ios(14.0));
    pub fn set_fragment_linked_functions(&self, functions: &LinkedFunctionsRef) {
        unsafe { msg_send![self, setFragmentLinkedFunctions: functions] }
    }

    pub fn reset(&self) {
        unsafe { msg_send![self, reset] }
    }
}

/// See <https://developer.apple.com/documentation/metal/mtlrenderpipelinestate>
pub enum MTLRenderPipelineState {}

foreign_obj_type! {
    type CType = MTLRenderPipelineState;
    pub struct RenderPipelineState;
}

impl RenderPipelineStateRef {
    pub fn device(&self) -> &DeviceRef {
        unsafe { msg_send![self, device] }
    }

    pub fn label(&self) -> &str {
        unsafe {
            let label = msg_send![self, label];
            crate::nsstring_as_str(label)
        }
    }

    /// Only available on (macos(11.0), ios(14.0))
    pub fn new_intersection_function_table_with_descriptor(
        &self,
        descriptor: &IntersectionFunctionTableDescriptorRef,
        stage: MTLRenderStages,
    ) -> IntersectionFunctionTable {
        unsafe {
            msg_send![self, newIntersectionFunctionTableWithDescriptor: descriptor
                                                                            stage:stage]
        }
    }

    /// Only available on (macos(11.0), ios(14.0))
    pub fn function_handle_with_function(
        &self,
        function: &FunctionRef,
        stage: MTLRenderStages,
    ) -> Option<&FunctionHandleRef> {
        unsafe {
            msg_send![self, functionHandleWithFunction: function
                                                            stage:stage]
        }
    }

    /// Only available on (macos(11.0), ios(14.0))
    pub fn new_visible_function_table_with_descriptor(
        &self,
        descriptor: &VisibleFunctionTableDescriptorRef,
        stage: MTLRenderStages,
    ) -> VisibleFunctionTable {
        unsafe { msg_send![self, newVisibleFunctionTableWithDescriptor: descriptor stage:stage] }
    }
}

/// See <https://developer.apple.com/documentation/metal/mtlrenderpipelinecolorattachmentdescriptorarray>
pub enum MTLRenderPipelineColorAttachmentDescriptorArray {}

foreign_obj_type! {
    type CType = MTLRenderPipelineColorAttachmentDescriptorArray;
    pub struct RenderPipelineColorAttachmentDescriptorArray;
}

impl RenderPipelineColorAttachmentDescriptorArrayRef {
    pub fn object_at(
        &self,
        index: NSUInteger,
    ) -> Option<&RenderPipelineColorAttachmentDescriptorRef> {
        unsafe { msg_send![self, objectAtIndexedSubscript: index] }
    }

    pub fn set_object_at(
        &self,
        index: NSUInteger,
        attachment: Option<&RenderPipelineColorAttachmentDescriptorRef>,
    ) {
        unsafe {
            msg_send![self, setObject:attachment
                   atIndexedSubscript:index]
        }
    }
}
