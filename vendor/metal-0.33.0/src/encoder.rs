// Copyright 2017 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use super::*;

use std::ops::Range;

/// See <https://developer.apple.com/documentation/metal/mtlcounterdontsample>
pub const COUNTER_DONT_SAMPLE: NSUInteger = NSUInteger::MAX; // #define MTLCounterDontSample ((NSUInteger)-1)

/// See <https://developer.apple.com/documentation/metal/mtlprimitivetype>
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MTLPrimitiveType {
    Point = 0,
    Line = 1,
    LineStrip = 2,
    Triangle = 3,
    TriangleStrip = 4,
}

/// See <https://developer.apple.com/documentation/metal/mtlindextype>
#[repr(u64)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MTLIndexType {
    UInt16 = 0,
    UInt32 = 1,
}

/// See <https://developer.apple.com/documentation/metal/mtlvisibilityresultmode>
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MTLVisibilityResultMode {
    Disabled = 0,
    Boolean = 1,
    Counting = 2,
}

/// See <https://developer.apple.com/documentation/metal/mtlcullmode>
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MTLCullMode {
    None = 0,
    Front = 1,
    Back = 2,
}

/// See <https://developer.apple.com/documentation/metal/mtlwinding>
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MTLWinding {
    Clockwise = 0,
    CounterClockwise = 1,
}

/// See <https://developer.apple.com/documentation/metal/mtldepthclipmode>
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MTLDepthClipMode {
    Clip = 0,
    Clamp = 1,
}

/// See <https://developer.apple.com/documentation/metal/mtltrianglefillmode>
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MTLTriangleFillMode {
    Fill = 0,
    Lines = 1,
}

bitflags::bitflags! {
    /// https://developer.apple.com/documentation/metal/mtlblitoption
    #[allow(non_upper_case_globals)]
    #[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub struct MTLBlitOption: NSUInteger {
        /// https://developer.apple.com/documentation/metal/mtlblitoption/mtlblitoptionnone
        const None = 0;
        /// https://developer.apple.com/documentation/metal/mtlblitoption/mtlblitoptiondepthfromdepthstencil
        const DepthFromDepthStencil   = 1 << 0;
        /// https://developer.apple.com/documentation/metal/mtlblitoption/mtlblitoptionstencilfromdepthstencil
        const StencilFromDepthStencil = 1 << 1;
        /// https://developer.apple.com/documentation/metal/mtlblitoption/mtlblitoptionrowlinearpvrtc
        const RowLinearPVRTC          = 1 << 2;
    }
}

/// See <https://developer.apple.com/documentation/metal/mtlscissorrect>
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MTLScissorRect {
    pub x: NSUInteger,
    pub y: NSUInteger,
    pub width: NSUInteger,
    pub height: NSUInteger,
}

/// See <https://developer.apple.com/documentation/metal/mtlviewport>
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MTLViewport {
    pub originX: f64,
    pub originY: f64,
    pub width: f64,
    pub height: f64,
    pub znear: f64,
    pub zfar: f64,
}

/// See <https://developer.apple.com/documentation/metal/mtldrawprimitivesindirectarguments>
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MTLDrawPrimitivesIndirectArguments {
    pub vertexCount: u32,
    pub instanceCount: u32,
    pub vertexStart: u32,
    pub baseInstance: u32,
}

/// See <https://developer.apple.com/documentation/metal/mtldrawindexedprimitivesindirectarguments>
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MTLDrawIndexedPrimitivesIndirectArguments {
    pub indexCount: u32,
    pub instanceCount: u32,
    pub indexStart: u32,
    pub baseVertex: i32,
    pub baseInstance: u32,
}

/// See <https://developer.apple.com/documentation/metal/mtlvertexamplificationviewmapping>
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct VertexAmplificationViewMapping {
    pub renderTargetArrayIndexOffset: u32,
    pub viewportArrayIndexOffset: u32,
}

#[allow(dead_code)]
type MTLVertexAmplicationViewMapping = VertexAmplificationViewMapping;

/// See <https://developer.apple.com/documentation/metal/mtlcommandencoder>
pub enum MTLCommandEncoder {}

foreign_obj_type! {
    type CType = MTLCommandEncoder;
    pub struct CommandEncoder;
}

impl CommandEncoderRef {
    pub fn label(&self) -> &str {
        unsafe {
            let label = msg_send![self, label];
            crate::nsstring_as_str(label)
        }
    }

    pub fn set_label(&self, label: &str) {
        unsafe {
            let nslabel = crate::nsstring_from_str(label);
            msg_send![self, setLabel: nslabel]
        }
    }

    pub fn end_encoding(&self) {
        unsafe { msg_send![self, endEncoding] }
    }

    pub fn insert_debug_signpost(&self, name: &str) {
        unsafe {
            let nslabel = crate::nsstring_from_str(name);
            msg_send![self, insertDebugSignpost: nslabel]
        }
    }

    pub fn push_debug_group(&self, name: &str) {
        unsafe {
            let nslabel = crate::nsstring_from_str(name);
            msg_send![self, pushDebugGroup: nslabel]
        }
    }

    pub fn pop_debug_group(&self) {
        unsafe { msg_send![self, popDebugGroup] }
    }
}

/// See <https://developer.apple.com/documentation/metal/mtlparallelrendercommandencoder>
pub enum MTLParallelRenderCommandEncoder {}

foreign_obj_type! {
    type CType = MTLParallelRenderCommandEncoder;
    pub struct ParallelRenderCommandEncoder;
    type ParentType = CommandEncoder;
}

impl ParallelRenderCommandEncoderRef {
    pub fn render_command_encoder(&self) -> &RenderCommandEncoderRef {
        unsafe { msg_send![self, renderCommandEncoder] }
    }
}

/// See <https://developer.apple.com/documentation/metal/mtlrendercommandencoder/>
pub enum MTLRenderCommandEncoder {}

foreign_obj_type! {
    type CType = MTLRenderCommandEncoder;
    pub struct RenderCommandEncoder;
    type ParentType = CommandEncoder;
}

impl RenderCommandEncoderRef {
    pub fn set_render_pipeline_state(&self, pipeline_state: &RenderPipelineStateRef) {
        unsafe { msg_send![self, setRenderPipelineState: pipeline_state] }
    }

    pub fn set_viewport(&self, viewport: MTLViewport) {
        unsafe { msg_send![self, setViewport: viewport] }
    }

    pub fn set_viewports(&self, viewports: &[MTLViewport]) {
        unsafe { msg_send![self, setViewports: viewports.as_ptr() count: viewports.len() as u64] }
    }

    pub fn set_front_facing_winding(&self, winding: MTLWinding) {
        unsafe { msg_send![self, setFrontFacingWinding: winding] }
    }

    pub fn set_cull_mode(&self, mode: MTLCullMode) {
        unsafe { msg_send![self, setCullMode: mode] }
    }

    pub fn set_depth_clip_mode(&self, mode: MTLDepthClipMode) {
        unsafe { msg_send![self, setDepthClipMode: mode] }
    }

    pub fn set_depth_bias(&self, bias: f32, scale: f32, clamp: f32) {
        unsafe {
            msg_send![self, setDepthBias:bias
                              slopeScale:scale
                                   clamp:clamp]
        }
    }

    pub fn set_scissor_rect(&self, rect: MTLScissorRect) {
        unsafe { msg_send![self, setScissorRect: rect] }
    }

    pub fn set_scissor_rects(&self, rects: &[MTLScissorRect]) {
        unsafe { msg_send![self, setScissorRects: rects.as_ptr() count: rects.len()] }
    }

    pub fn set_triangle_fill_mode(&self, mode: MTLTriangleFillMode) {
        unsafe { msg_send![self, setTriangleFillMode: mode] }
    }

    pub fn set_blend_color(&self, red: f32, green: f32, blue: f32, alpha: f32) {
        unsafe {
            msg_send![self, setBlendColorRed:red
                                         green:green
                                          blue:blue
                                         alpha:alpha]
        }
    }

    pub fn set_depth_stencil_state(&self, depth_stencil_state: &DepthStencilStateRef) {
        unsafe { msg_send![self, setDepthStencilState: depth_stencil_state] }
    }

    pub fn set_stencil_reference_value(&self, value: u32) {
        unsafe { msg_send![self, setStencilReferenceValue: value] }
    }

    pub fn set_stencil_front_back_reference_value(&self, front: u32, back: u32) {
        unsafe {
            msg_send![self, setStencilFrontReferenceValue:front
                                       backReferenceValue:back]
        }
    }

    pub fn set_visibility_result_mode(&self, mode: MTLVisibilityResultMode, offset: NSUInteger) {
        unsafe {
            msg_send![self, setVisibilityResultMode:mode
                                             offset:offset]
        }
    }

    pub fn set_vertex_amplification_count(
        &self,
        count: NSUInteger,
        view_mappings: Option<&[VertexAmplificationViewMapping]>,
    ) {
        unsafe {
            msg_send! [self, setVertexAmplificationCount: count viewMappings: view_mappings.map_or(std::ptr::null(), |vm| vm.as_ptr())]
        }
    }

    // Specifying Resources for a Vertex Shader Function

    pub fn set_vertex_bytes(
        &self,
        index: NSUInteger,
        length: NSUInteger,
        bytes: *const std::ffi::c_void,
    ) {
        unsafe {
            msg_send![self,
                setVertexBytes:bytes
                length:length
                atIndex:index
            ]
        }
    }

    pub fn set_vertex_buffer(
        &self,
        index: NSUInteger,
        buffer: Option<&BufferRef>,
        offset: NSUInteger,
    ) {
        unsafe {
            msg_send![self,
                setVertexBuffer:buffer
                offset:offset
                atIndex:index
            ]
        }
    }

    pub fn set_vertex_buffer_offset(&self, index: NSUInteger, offset: NSUInteger) {
        unsafe {
            msg_send![self,
                setVertexBufferOffset:offset
                atIndex:index
            ]
        }
    }

    pub fn set_vertex_buffers(
        &self,
        start_index: NSUInteger,
        data: &[Option<&BufferRef>],
        offsets: &[NSUInteger],
    ) {
        debug_assert_eq!(offsets.len(), data.len());
        unsafe {
            msg_send![self,
                setVertexBuffers: data.as_ptr()
                offsets: offsets.as_ptr()
                withRange: NSRange {
                    location: start_index,
                    length: data.len() as _,
                }
            ]
        }
    }

    pub fn set_vertex_texture(&self, index: NSUInteger, texture: Option<&TextureRef>) {
        unsafe {
            msg_send![self,
                setVertexTexture:texture
                atIndex:index
            ]
        }
    }

    pub fn set_vertex_textures(&self, start_index: NSUInteger, data: &[Option<&TextureRef>]) {
        unsafe {
            msg_send![self,
                setVertexTextures: data.as_ptr()
                withRange: NSRange {
                    location: start_index,
                    length: data.len() as _,
                }
            ]
        }
    }

    pub fn set_vertex_sampler_state(&self, index: NSUInteger, sampler: Option<&SamplerStateRef>) {
        unsafe {
            msg_send![self,
                setVertexSamplerState:sampler
                atIndex:index
            ]
        }
    }

    pub fn set_vertex_sampler_states(
        &self,
        start_index: NSUInteger,
        data: &[Option<&SamplerStateRef>],
    ) {
        unsafe {
            msg_send![self,
                setVertexSamplerStates: data.as_ptr()
                withRange: NSRange {
                    location: start_index,
                    length: data.len() as _,
                }
            ]
        }
    }

    pub fn set_vertex_sampler_state_with_lod(
        &self,
        index: NSUInteger,
        sampler: Option<&SamplerStateRef>,
        lod_clamp: Range<f32>,
    ) {
        unsafe {
            msg_send![self,
                setVertexSamplerState:sampler
                lodMinClamp:lod_clamp.start
                lodMaxClamp:lod_clamp.end
                atIndex:index
            ]
        }
    }

    /// Only available in (macos(12.0), ios(15.0))
    pub fn set_vertex_acceleration_structure(
        &self,
        index: NSUInteger,
        accel: Option<&AccelerationStructureRef>,
    ) {
        unsafe {
            msg_send![
                self,
                setVertexAccelerationStructure: accel
                atBufferIndex: index
            ]
        }
    }

    /// Only available in (macos(12.0), ios(15.0))
    pub fn set_vertex_intersection_function_table(
        &self,
        index: NSUInteger,
        table: Option<&IntersectionFunctionTableRef>,
    ) {
        unsafe {
            msg_send![
                self,
                setVertexIntersectionFunctionTable: table
                atBufferIndex: index
            ]
        }
    }

    pub fn set_vertex_visible_function_table(
        &self,
        buffer_index: NSUInteger,
        visible_function_table: Option<&VisibleFunctionTableRef>,
    ) {
        unsafe {
            msg_send![self,
            setVertexVisibleFunctionTable:visible_function_table
            atBufferIndex:buffer_index]
        }
    }

    pub fn set_vertex_visible_function_tables(
        &self,
        buffer_start_index: NSUInteger,
        visible_function_tables: &[&VisibleFunctionTableRef],
    ) {
        unsafe {
            msg_send![self,
                setVertexVisibleFunctionTables:visible_function_tables.as_ptr()
                withBufferRange: NSRange {
                    location: buffer_start_index,
                    length: visible_function_tables.len() as _,
                }
            ]
        }
    }

    // Specifying Resources for a Object Shader Function

    /// Only available in (macos(13.0), ios(16.0))
    pub fn set_object_buffer(
        &self,
        index: NSUInteger,
        buffer: Option<&BufferRef>,
        offset: NSUInteger,
    ) {
        unsafe {
            msg_send![self,
                setObjectBuffer:buffer
                offset:offset
                atIndex:index
            ]
        }
    }

    /// Only available in (macos(13.0), ios(16.0))
    pub fn set_object_buffer_offset(&self, index: NSUInteger, offset: NSUInteger) {
        unsafe {
            msg_send![self,
                setObjectBufferOffset:offset
                atIndex:index
            ]
        }
    }

    /// Only available in (macos(13.0), ios(16.0))
    pub fn set_object_bytes(
        &self,
        index: NSUInteger,
        length: NSUInteger,
        bytes: *const std::ffi::c_void,
    ) {
        unsafe {
            msg_send![self,
                setObjectBytes:bytes
                length:length
                atIndex:index
            ]
        }
    }

    /// Only available in (macos(13.0), ios(16.0))
    pub fn set_object_sampler_state(&self, index: NSUInteger, sampler: Option<&SamplerStateRef>) {
        unsafe {
            msg_send![self,
                setObjectSamplerState:sampler
                atIndex:index
            ]
        }
    }

    /// Only available in (macos(13.0), ios(16.0))
    pub fn set_object_sampler_state_with_lod(
        &self,
        index: NSUInteger,
        sampler: Option<&SamplerStateRef>,
        lod_clamp: Range<f32>,
    ) {
        unsafe {
            msg_send![self,
                setObjectSamplerState:sampler
                lodMinClamp:lod_clamp.start
                lodMaxClamp:lod_clamp.end
                atIndex:index
            ]
        }
    }

    /// Only available in (macos(13.0), ios(16.0))
    pub fn set_object_texture(&self, index: NSUInteger, texture: Option<&TextureRef>) {
        unsafe {
            msg_send![self,
                setObjectTexture:texture
                atIndex:index
            ]
        }
    }

    /// Only available in (macos(13.0), ios(16.0))
    pub fn set_object_threadgroup_memory_length(&self, index: NSUInteger, length: NSUInteger) {
        unsafe {
            msg_send![self,
                setObjectThreadgroupMemoryLength: length
                atIndex: index
            ]
        }
    }

    /// Only available in (macos(13.0), ios(16.0))
    pub fn set_object_buffers(
        &self,
        start_index: NSUInteger,
        data: &[Option<&BufferRef>],
        offsets: &[NSUInteger],
    ) {
        debug_assert_eq!(offsets.len(), data.len());
        unsafe {
            msg_send![self,
                setObjectBuffers: data.as_ptr()
                offsets: offsets.as_ptr()
                withRange: NSRange {
                    location: start_index,
                    length: data.len() as _,
                }
            ]
        }
    }

    /// Only available in (macos(13.0), ios(16.0))
    pub fn set_object_sampler_states(
        &self,
        start_index: NSUInteger,
        data: &[Option<&SamplerStateRef>],
    ) {
        unsafe {
            msg_send![self,
                setObjectSamplerStates: data.as_ptr()
                withRange: NSRange {
                    location: start_index,
                    length: data.len() as _,
                }
            ]
        }
    }

    /// Only available in (macos(13.0), ios(16.0))
    pub fn set_object_textures(&self, start_index: NSUInteger, data: &[Option<&TextureRef>]) {
        unsafe {
            msg_send![self,
                setObjectTextures: data.as_ptr()
                withRange: NSRange {
                    location: start_index,
                    length: data.len() as _,
                }
            ]
        }
    }

    // Specifying Resources for a Mesh Shader

    /// Only available in (macos(13.0), ios(16.0))
    pub fn set_mesh_buffer(
        &self,
        index: NSUInteger,
        buffer: Option<&BufferRef>,
        offset: NSUInteger,
    ) {
        unsafe {
            msg_send![self,
                setMeshBuffer:buffer
                offset:offset
                atIndex:index
            ]
        }
    }

    /// Only available in (macos(13.0), ios(16.0))
    pub fn set_mesh_buffer_offset(&self, index: NSUInteger, offset: NSUInteger) {
        unsafe {
            msg_send![self,
                setMeshBufferOffset:offset
                atIndex:index
            ]
        }
    }

    /// Only available in (macos(13.0), ios(16.0))
    pub fn set_mesh_bytes(
        &self,
        index: NSUInteger,
        length: NSUInteger,
        bytes: *const std::ffi::c_void,
    ) {
        unsafe {
            msg_send![self,
                setMeshBytes:bytes
                length:length
                atIndex:index
            ]
        }
    }

    /// Only available in (macos(13.0), ios(16.0))
    pub fn set_mesh_sampler_state(&self, index: NSUInteger, sampler: Option<&SamplerStateRef>) {
        unsafe {
            msg_send![self,
                setMeshSamplerState:sampler
                atIndex:index
            ]
        }
    }

    /// Only available in (macos(13.0), ios(16.0))
    pub fn set_mesh_sampler_state_with_lod(
        &self,
        index: NSUInteger,
        sampler: Option<&SamplerStateRef>,
        lod_clamp: Range<f32>,
    ) {
        unsafe {
            msg_send![self,
                setMeshSamplerState:sampler
                lodMinClamp:lod_clamp.start
                lodMaxClamp:lod_clamp.end
                atIndex:index
            ]
        }
    }

    /// Only available in (macos(13.0), ios(16.0))
    pub fn set_mesh_texture(&self, index: NSUInteger, texture: Option<&TextureRef>) {
        unsafe {
            msg_send![self,
                setMeshTexture:texture
                atIndex:index
            ]
        }
    }

    /// Only available in (macos(13.0), ios(16.0))
    pub fn set_mesh_buffers(
        &self,
        start_index: NSUInteger,
        data: &[Option<&BufferRef>],
        offsets: &[NSUInteger],
    ) {
        debug_assert_eq!(offsets.len(), data.len());
        unsafe {
            msg_send![self,
                setMeshBuffers: data.as_ptr()
                offsets: offsets.as_ptr()
                withRange: NSRange {
                    location: start_index,
                    length: data.len() as _,
                }
            ]
        }
    }

    /// Only available in (macos(13.0), ios(16.0))
    pub fn set_mesh_sampler_states(
        &self,
        start_index: NSUInteger,
        data: &[Option<&SamplerStateRef>],
    ) {
        unsafe {
            msg_send![self,
                setMeshSamplerStates: data.as_ptr()
                withRange: NSRange {
                    location: start_index,
                    length: data.len() as _,
                }
            ]
        }
    }

    /// Only available in (macos(13.0), ios(16.0))
    pub fn set_mesh_textures(&self, start_index: NSUInteger, data: &[Option<&TextureRef>]) {
        unsafe {
            msg_send![self,
                setMeshTextures: data.as_ptr()
                withRange: NSRange {
                    location: start_index,
                    length: data.len() as _,
                }
            ]
        }
    }

    // Specifying Resources for a Fragment Shader Function

    pub fn set_fragment_bytes(
        &self,
        index: NSUInteger,
        length: NSUInteger,
        bytes: *const std::ffi::c_void,
    ) {
        unsafe {
            msg_send![self,
                setFragmentBytes:bytes
                length:length
                atIndex:index
            ]
        }
    }

    pub fn set_fragment_buffer(
        &self,
        index: NSUInteger,
        buffer: Option<&BufferRef>,
        offset: NSUInteger,
    ) {
        unsafe {
            msg_send![self,
                setFragmentBuffer:buffer
                offset:offset
                atIndex:index
            ]
        }
    }

    pub fn set_fragment_buffer_offset(&self, index: NSUInteger, offset: NSUInteger) {
        unsafe {
            msg_send![self,
                setFragmentBufferOffset:offset
                atIndex:index
            ]
        }
    }

    pub fn set_fragment_buffers(
        &self,
        start_index: NSUInteger,
        data: &[Option<&BufferRef>],
        offsets: &[NSUInteger],
    ) {
        debug_assert_eq!(offsets.len(), data.len());
        unsafe {
            msg_send![self,
                setFragmentBuffers: data.as_ptr()
                offsets: offsets.as_ptr()
                withRange: NSRange {
                    location: start_index,
                    length: data.len() as _,
                }
            ]
        }
    }

    pub fn set_fragment_texture(&self, index: NSUInteger, texture: Option<&TextureRef>) {
        unsafe {
            msg_send![self,
                setFragmentTexture:texture
                atIndex:index
            ]
        }
    }

    pub fn set_fragment_textures(&self, start_index: NSUInteger, data: &[Option<&TextureRef>]) {
        unsafe {
            msg_send![self,
                setFragmentTextures: data.as_ptr()
                withRange: NSRange {
                    location: start_index,
                    length: data.len() as _,
                }
            ]
        }
    }

    pub fn set_fragment_sampler_state(&self, index: NSUInteger, sampler: Option<&SamplerStateRef>) {
        unsafe {
            msg_send![self, setFragmentSamplerState:sampler
                                            atIndex:index]
        }
    }

    pub fn set_fragment_sampler_states(
        &self,
        start_index: NSUInteger,
        data: &[Option<&SamplerStateRef>],
    ) {
        unsafe {
            msg_send![self,
                setFragmentSamplerStates: data.as_ptr()
                withRange: NSRange {
                    location: start_index,
                    length: data.len() as _,
                }
            ]
        }
    }

    pub fn set_fragment_sampler_state_with_lod(
        &self,
        index: NSUInteger,
        sampler: Option<&SamplerStateRef>,
        lod_clamp: Range<f32>,
    ) {
        unsafe {
            msg_send![self,
                setFragmentSamplerState:sampler
                lodMinClamp:lod_clamp.start
                lodMaxClamp:lod_clamp.end
                atIndex:index
            ]
        }
    }

    /// Only available in (macos(12.0), ios(15.0))
    pub fn set_fragment_acceleration_structure(
        &self,
        index: NSUInteger,
        accel: Option<&AccelerationStructureRef>,
    ) {
        unsafe {
            msg_send![
                self,
                setFragmentAccelerationStructure: accel
                atBufferIndex: index
            ]
        }
    }

    /// Only available in (macos(12.0), ios(15.0))
    pub fn set_fragment_intersection_function_table(
        &self,
        index: NSUInteger,
        table: Option<&IntersectionFunctionTableRef>,
    ) {
        unsafe {
            msg_send![
                self,
                setFragmentIntersectionFunctionTable: table
                atBufferIndex: index
            ]
        }
    }

    pub fn set_fragment_visible_function_table(
        &self,
        buffer_index: NSUInteger,
        visible_function_table: Option<&VisibleFunctionTableRef>,
    ) {
        unsafe {
            msg_send![self,
            setFragmentVisibleFunctionTable:visible_function_table
            atBufferIndex:buffer_index]
        }
    }

    pub fn set_fragment_visible_function_tables(
        &self,
        buffer_start_index: NSUInteger,
        visible_function_tables: &[&VisibleFunctionTableRef],
    ) {
        unsafe {
            msg_send![self,
                setFragmentVisibleFunctionTables:visible_function_tables.as_ptr()
                withBufferRange: NSRange {
                    location: buffer_start_index,
                    length: visible_function_tables.len() as _,
                }
            ]
        }
    }

    // Drawing Geometric Primitives

    pub fn draw_primitives(
        &self,
        primitive_type: MTLPrimitiveType,
        vertex_start: NSUInteger,
        vertex_count: NSUInteger,
    ) {
        unsafe {
            msg_send![self,
                drawPrimitives: primitive_type
                vertexStart: vertex_start
                vertexCount: vertex_count
            ]
        }
    }

    pub fn draw_primitives_instanced(
        &self,
        primitive_type: MTLPrimitiveType,
        vertex_start: NSUInteger,
        vertex_count: NSUInteger,
        instance_count: NSUInteger,
    ) {
        unsafe {
            msg_send![self,
                drawPrimitives: primitive_type
                vertexStart: vertex_start
                vertexCount: vertex_count
                instanceCount: instance_count
            ]
        }
    }

    pub fn draw_primitives_instanced_base_instance(
        &self,
        primitive_type: MTLPrimitiveType,
        vertex_start: NSUInteger,
        vertex_count: NSUInteger,
        instance_count: NSUInteger,
        base_instance: NSUInteger,
    ) {
        unsafe {
            msg_send![self,
                drawPrimitives: primitive_type
                vertexStart: vertex_start
                vertexCount: vertex_count
                instanceCount: instance_count
                baseInstance: base_instance
            ]
        }
    }

    pub fn draw_primitives_indirect(
        &self,
        primitive_type: MTLPrimitiveType,
        indirect_buffer: &BufferRef,
        indirect_buffer_offset: NSUInteger,
    ) {
        unsafe {
            msg_send![self,
                drawPrimitives: primitive_type
                indirectBuffer: indirect_buffer
                indirectBufferOffset: indirect_buffer_offset
            ]
        }
    }

    pub fn draw_indexed_primitives(
        &self,
        primitive_type: MTLPrimitiveType,
        index_count: NSUInteger,
        index_type: MTLIndexType,
        index_buffer: &BufferRef,
        index_buffer_offset: NSUInteger,
    ) {
        unsafe {
            msg_send![self,
                drawIndexedPrimitives: primitive_type
                indexCount: index_count
                indexType: index_type
                indexBuffer: index_buffer
                indexBufferOffset: index_buffer_offset
            ]
        }
    }

    pub fn draw_indexed_primitives_instanced(
        &self,
        primitive_type: MTLPrimitiveType,
        index_count: NSUInteger,
        index_type: MTLIndexType,
        index_buffer: &BufferRef,
        index_buffer_offset: NSUInteger,
        instance_count: NSUInteger,
    ) {
        unsafe {
            msg_send![self,
                drawIndexedPrimitives: primitive_type
                indexCount: index_count
                indexType: index_type
                indexBuffer: index_buffer
                indexBufferOffset: index_buffer_offset
                instanceCount: instance_count
            ]
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn draw_indexed_primitives_instanced_base_instance(
        &self,
        primitive_type: MTLPrimitiveType,
        index_count: NSUInteger,
        index_type: MTLIndexType,
        index_buffer: &BufferRef,
        index_buffer_offset: NSUInteger,
        instance_count: NSUInteger,
        base_vertex: NSInteger,
        base_instance: NSUInteger,
    ) {
        unsafe {
            msg_send![self,
                drawIndexedPrimitives: primitive_type
                indexCount: index_count
                indexType: index_type
                indexBuffer: index_buffer
                indexBufferOffset: index_buffer_offset
                instanceCount: instance_count
                baseVertex: base_vertex
                baseInstance: base_instance
            ]
        }
    }

    pub fn draw_indexed_primitives_indirect(
        &self,
        primitive_type: MTLPrimitiveType,
        index_type: MTLIndexType,
        index_buffer: &BufferRef,
        index_buffer_offset: NSUInteger,
        indirect_buffer: &BufferRef,
        indirect_buffer_offset: NSUInteger,
    ) {
        unsafe {
            msg_send![self,
                drawIndexedPrimitives: primitive_type
                indexType: index_type
                indexBuffer: index_buffer
                indexBufferOffset: index_buffer_offset
                indirectBuffer: indirect_buffer
                indirectBufferOffset: indirect_buffer_offset
            ]
        }
    }

    // TODO: more draws
    // fn setVertexBufferOffset_atIndex(self, offset: NSUInteger, index: NSUInteger);
    // fn setVertexBuffers_offsets_withRange(self, buffers: *const id, offsets: *const NSUInteger, range: NSRange);
    // fn setVertexSamplerStates_lodMinClamps_lodMaxClamps_withRange(self, samplers: *const id, lodMinClamps: *const f32, lodMaxClamps: *const f32, range: NSRange);

    /// Only available in (macos(13.0), ios(16.0))
    pub fn draw_mesh_threadgroups(
        &self,
        threadgroups_per_grid: MTLSize,
        threads_per_object_threadgroup: MTLSize,
        threads_per_mesh_threadgroup: MTLSize,
    ) {
        unsafe {
            msg_send![self,
                drawMeshThreadgroups: threadgroups_per_grid
                threadsPerObjectThreadgroup: threads_per_object_threadgroup
                threadsPerMeshThreadgroup: threads_per_mesh_threadgroup
            ]
        }
    }

    /// Only available in (macos(13.0), ios(16.0))
    pub fn draw_mesh_threadgroups_with_indirect_buffer(
        &self,
        indirect_buffer: &BufferRef,
        indirect_buffer_offset: NSUInteger,
        threads_per_object_threadgroup: MTLSize,
        threads_per_mesh_threadgroup: MTLSize,
    ) {
        unsafe {
            msg_send![self,
                drawMeshThreadgroupsWithIndirectBuffer: indirect_buffer
                indirectBufferOffset: indirect_buffer_offset
                threadsPerObjectThreadgroup: threads_per_object_threadgroup
                threadsPerMeshThreadgroup: threads_per_mesh_threadgroup
            ]
        }
    }

    /// Only available in (macos(13.0), ios(16.0))
    pub fn draw_mesh_threads(
        &self,
        threads_per_grid: MTLSize,
        threads_per_object_threadgroup: MTLSize,
        threads_per_mesh_threadgroup: MTLSize,
    ) {
        unsafe {
            msg_send![self,
                drawMeshThreads: threads_per_grid
                threadsPerObjectThreadgroup: threads_per_object_threadgroup
                threadsPerMeshThreadgroup: threads_per_mesh_threadgroup
            ]
        }
    }

    /// Adds an untracked resource to the render pass.
    ///
    /// Availability: iOS 11.0+, macOS 10.13+
    ///
    /// # Arguments
    /// * `resource`: A resource within an argument buffer.
    /// * `usage`: Options for describing how a graphics function uses the resource.
    ///
    /// See <https://developer.apple.com/documentation/metal/mtlrendercommandencoder/2866168-useresource?language=objc>
    #[deprecated(note = "Use use_resource_at instead")]
    pub fn use_resource(&self, resource: &ResourceRef, usage: MTLResourceUsage) {
        unsafe {
            msg_send![self,
                useResource:resource
                usage:usage
            ]
        }
    }

    /// Adds an untracked resource to the render pass, specifying which render stages need it.
    ///
    /// Availability: iOS 13.0+, macOS 10.15+
    ///
    /// # Arguments
    /// * `resource`: A resource within an argument buffer.
    /// * `usage`: Options for describing how a graphics function uses the resource.
    /// * `stages`: The render stages where the resource must be resident.
    ///
    /// See <https://developer.apple.com/documentation/metal/mtlrendercommandencoder/3043404-useresource>
    pub fn use_resource_at(
        &self,
        resource: &ResourceRef,
        usage: MTLResourceUsage,
        stages: MTLRenderStages,
    ) {
        unsafe {
            msg_send![self,
                useResource: resource
                usage: usage
                stages: stages
            ]
        }
    }

    /// Adds an array of untracked resources to the render pass, specifying which stages need them.
    ///
    /// When working with color render targets, call this method as late as possible to improve performance.
    ///
    /// Availability: iOS 13.0+, macOS 10.15+
    ///
    /// # Arguments
    /// * `resources`: A slice of resources within an argument buffer.
    /// * `usage`: Options for describing how a graphics function uses the resources.
    /// * `stages`: The render stages where the resources must be resident.
    pub fn use_resources(
        &self,
        resources: &[&ResourceRef],
        usage: MTLResourceUsage,
        stages: MTLRenderStages,
    ) {
        unsafe {
            msg_send![self,
                useResources: resources.as_ptr()
                count: resources.len() as NSUInteger
                usage: usage
                stages: stages
            ]
        }
    }

    /// Adds the resources in a heap to the render pass.
    ///
    /// Availability: iOS 11.0+, macOS 10.13+
    ///
    /// # Arguments:
    /// * `heap`: A heap that contains resources within an argument buffer.
    ///
    /// See <https://developer.apple.com/documentation/metal/mtlrendercommandencoder/2866163-useheap?language=objc>
    #[deprecated(note = "Use use_heap_at instead")]
    pub fn use_heap(&self, heap: &HeapRef) {
        unsafe { msg_send![self, useHeap: heap] }
    }

    /// Adds the resources in a heap to the render pass, specifying which render stages need them.
    ///
    /// Availability: iOS 13.0+, macOS 10.15+
    ///
    /// # Arguments
    /// * `heap`: A heap that contains resources within an argument buffer.
    /// * `stages`: The render stages where the resources must be resident.
    ///
    pub fn use_heap_at(&self, heap: &HeapRef, stages: MTLRenderStages) {
        unsafe {
            msg_send![self,
                useHeap: heap
                stages: stages
            ]
        }
    }

    /// Adds the resources in an array of heaps to the render pass, specifying which render stages need them.
    ///
    /// Availability: iOS 13.0+, macOS 10.15+
    ///
    /// # Arguments
    ///
    /// * `heaps`: A slice of heaps that contains resources within an argument buffer.
    /// * `stages`: The render stages where the resources must be resident.
    pub fn use_heaps(&self, heaps: &[&HeapRef], stages: MTLRenderStages) {
        unsafe {
            msg_send![self,
                useHeaps: heaps.as_ptr()
                count: heaps.len() as NSUInteger
                stages: stages
            ]
        }
    }

    pub fn execute_commands_in_buffer(
        &self,
        buffer: &IndirectCommandBufferRef,
        with_range: NSRange,
    ) {
        unsafe { msg_send![self, executeCommandsInBuffer:buffer withRange:with_range] }
    }

    pub fn update_fence(&self, fence: &FenceRef, after_stages: MTLRenderStages) {
        unsafe {
            msg_send![self,
                updateFence: fence
                afterStages: after_stages
            ]
        }
    }

    pub fn wait_for_fence(&self, fence: &FenceRef, before_stages: MTLRenderStages) {
        unsafe {
            msg_send![self,
                waitForFence: fence
                beforeStages: before_stages
            ]
        }
    }

    /// See: <https://developer.apple.com/documentation/metal/mtlrendercommandencoder/3194379-samplecountersinbuffer>
    pub fn sample_counters_in_buffer(
        &self,
        sample_buffer: &CounterSampleBufferRef,
        sample_index: NSUInteger,
        with_barrier: bool,
    ) {
        unsafe {
            msg_send![self,
                sampleCountersInBuffer: sample_buffer
                atSampleIndex: sample_index
                withBarrier: with_barrier
            ]
        }
    }
}

/// See <https://developer.apple.com/documentation/metal/mtlblitcommandencoder/>
pub enum MTLBlitCommandEncoder {}

foreign_obj_type! {
    type CType = MTLBlitCommandEncoder;
    pub struct BlitCommandEncoder;
    type ParentType = CommandEncoder;
}

impl BlitCommandEncoderRef {
    pub fn synchronize_resource(&self, resource: &ResourceRef) {
        unsafe { msg_send![self, synchronizeResource: resource] }
    }

    pub fn fill_buffer(&self, destination_buffer: &BufferRef, range: crate::NSRange, value: u8) {
        unsafe {
            msg_send![self,
                fillBuffer: destination_buffer
                range: range
                value: value
            ]
        }
    }

    pub fn generate_mipmaps(&self, texture: &TextureRef) {
        unsafe { msg_send![self, generateMipmapsForTexture: texture] }
    }

    pub fn copy_from_buffer(
        &self,
        source_buffer: &BufferRef,
        source_offset: NSUInteger,
        destination_buffer: &BufferRef,
        destination_offset: NSUInteger,
        size: NSUInteger,
    ) {
        unsafe {
            msg_send![self,
                copyFromBuffer: source_buffer
                sourceOffset: source_offset
                toBuffer: destination_buffer
                destinationOffset: destination_offset
                size: size
            ]
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn copy_from_texture(
        &self,
        source_texture: &TextureRef,
        source_slice: NSUInteger,
        source_level: NSUInteger,
        source_origin: MTLOrigin,
        source_size: MTLSize,
        destination_texture: &TextureRef,
        destination_slice: NSUInteger,
        destination_level: NSUInteger,
        destination_origin: MTLOrigin,
    ) {
        unsafe {
            msg_send![self,
                copyFromTexture: source_texture
                sourceSlice: source_slice
                sourceLevel: source_level
                sourceOrigin: source_origin
                sourceSize: source_size
                toTexture: destination_texture
                destinationSlice: destination_slice
                destinationLevel: destination_level
                destinationOrigin: destination_origin
            ]
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn copy_from_buffer_to_texture(
        &self,
        source_buffer: &BufferRef,
        source_offset: NSUInteger,
        source_bytes_per_row: NSUInteger,
        source_bytes_per_image: NSUInteger,
        source_size: MTLSize,
        destination_texture: &TextureRef,
        destination_slice: NSUInteger,
        destination_level: NSUInteger,
        destination_origin: MTLOrigin,
        options: MTLBlitOption,
    ) {
        unsafe {
            msg_send![self,
                copyFromBuffer: source_buffer
                sourceOffset: source_offset
                sourceBytesPerRow: source_bytes_per_row
                sourceBytesPerImage: source_bytes_per_image
                sourceSize: source_size
                toTexture: destination_texture
                destinationSlice: destination_slice
                destinationLevel: destination_level
                destinationOrigin: destination_origin
                options: options
            ]
        }
    }

    /// See <https://developer.apple.com/documentation/metal/mtlblitcommandencoder/1400756-copy>
    #[allow(clippy::too_many_arguments)]
    pub fn copy_from_texture_to_buffer(
        &self,
        source_texture: &TextureRef,
        source_slice: NSUInteger,
        source_level: NSUInteger,
        source_origin: MTLOrigin,
        source_size: MTLSize,
        destination_buffer: &BufferRef,
        destination_offset: NSUInteger,
        destination_bytes_per_row: NSUInteger,
        destination_bytes_per_image: NSUInteger,
        options: MTLBlitOption,
    ) {
        unsafe {
            msg_send![self,
                copyFromTexture: source_texture
                sourceSlice: source_slice
                sourceLevel: source_level
                sourceOrigin: source_origin
                sourceSize: source_size
                toBuffer: destination_buffer
                destinationOffset: destination_offset
                destinationBytesPerRow: destination_bytes_per_row
                destinationBytesPerImage: destination_bytes_per_image
                options: options
            ]
        }
    }

    pub fn optimize_contents_for_gpu_access(&self, texture: &TextureRef) {
        unsafe { msg_send![self, optimizeContentsForGPUAccess: texture] }
    }

    pub fn optimize_contents_for_gpu_access_slice_level(
        &self,
        texture: &TextureRef,
        slice: NSUInteger,
        level: NSUInteger,
    ) {
        unsafe {
            msg_send![self,
                optimizeContentsForGPUAccess: texture
                slice: slice
                level: level
            ]
        }
    }

    pub fn optimize_contents_for_cpu_access(&self, texture: &TextureRef) {
        unsafe { msg_send![self, optimizeContentsForCPUAccess: texture] }
    }

    pub fn optimize_contents_for_cpu_access_slice_level(
        &self,
        texture: &TextureRef,
        slice: NSUInteger,
        level: NSUInteger,
    ) {
        unsafe {
            msg_send![self,
                optimizeContentsForCPUAccess: texture
                slice: slice
                level: level
            ]
        }
    }

    pub fn update_fence(&self, fence: &FenceRef) {
        unsafe { msg_send![self, updateFence: fence] }
    }

    pub fn wait_for_fence(&self, fence: &FenceRef) {
        unsafe { msg_send![self, waitForFence: fence] }
    }

    pub fn copy_indirect_command_buffer(
        &self,
        source: &IndirectCommandBufferRef,
        source_range: NSRange,
        destination: &IndirectCommandBufferRef,
        destination_index: NSUInteger,
    ) {
        unsafe {
            msg_send![self,
                copyIndirectCommandBuffer: source
                sourceRange: source_range
                destination: destination
                destinationIndex: destination_index
            ]
        }
    }

    pub fn reset_commands_in_buffer(&self, buffer: &IndirectCommandBufferRef, range: NSRange) {
        unsafe {
            msg_send![self,
                resetCommandsInBuffer: buffer
                withRange: range
            ]
        }
    }

    pub fn optimize_indirect_command_buffer(
        &self,
        buffer: &IndirectCommandBufferRef,
        range: NSRange,
    ) {
        unsafe {
            msg_send![self,
                optimizeIndirectCommandBuffer: buffer
                withRange: range
            ]
        }
    }

    /// See: <https://developer.apple.com/documentation/metal/mtlblitcommandencoder/3194348-samplecountersinbuffer>
    pub fn sample_counters_in_buffer(
        &self,
        sample_buffer: &CounterSampleBufferRef,
        sample_index: NSUInteger,
        with_barrier: bool,
    ) {
        unsafe {
            msg_send![self,
                sampleCountersInBuffer: sample_buffer
                atSampleIndex: sample_index
                withBarrier: with_barrier
            ]
        }
    }

    /// See: <https://developer.apple.com/documentation/metal/mtlblitcommandencoder/3194347-resolvecounters>
    pub fn resolve_counters(
        &self,
        sample_buffer: &CounterSampleBufferRef,
        range: crate::NSRange,
        destination_buffer: &BufferRef,
        destination_offset: NSUInteger,
    ) {
        unsafe {
            msg_send![self,
                resolveCounters: sample_buffer
                inRange: range
                destinationBuffer: destination_buffer
                destinationOffset: destination_offset
            ]
        }
    }
}

/// See <https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/>
pub enum MTLComputeCommandEncoder {}

foreign_obj_type! {
    type CType = MTLComputeCommandEncoder;
    pub struct ComputeCommandEncoder;
    type ParentType = CommandEncoder;
}

impl ComputeCommandEncoderRef {
    pub fn set_compute_pipeline_state(&self, state: &ComputePipelineStateRef) {
        unsafe { msg_send![self, setComputePipelineState: state] }
    }

    pub fn set_buffer(&self, index: NSUInteger, buffer: Option<&BufferRef>, offset: NSUInteger) {
        unsafe { msg_send![self, setBuffer:buffer offset:offset atIndex:index] }
    }

    pub fn set_buffers(
        &self,
        start_index: NSUInteger,
        data: &[Option<&BufferRef>],
        offsets: &[NSUInteger],
    ) {
        debug_assert_eq!(offsets.len(), data.len());
        unsafe {
            msg_send![self,
                setBuffers: data.as_ptr()
                offsets: offsets.as_ptr()
                withRange: NSRange {
                    location: start_index,
                    length: data.len() as _,
                }
            ]
        }
    }

    pub fn set_texture(&self, index: NSUInteger, texture: Option<&TextureRef>) {
        unsafe {
            msg_send![self,
                setTexture:texture
                atIndex:index
            ]
        }
    }

    pub fn set_textures(&self, start_index: NSUInteger, data: &[Option<&TextureRef>]) {
        unsafe {
            msg_send![self,
                setTextures: data.as_ptr()
                withRange: NSRange {
                    location: start_index,
                    length: data.len() as _,
                }
            ]
        }
    }

    pub fn set_sampler_state(&self, index: NSUInteger, sampler: Option<&SamplerStateRef>) {
        unsafe {
            msg_send![self,
                setSamplerState:sampler
                atIndex:index
            ]
        }
    }

    pub fn set_sampler_states(&self, start_index: NSUInteger, data: &[Option<&SamplerStateRef>]) {
        unsafe {
            msg_send![self,
                setSamplerStates: data.as_ptr()
                withRange: NSRange {
                    location: start_index,
                    length: data.len() as _,
                }
            ]
        }
    }

    pub fn set_sampler_state_with_lod(
        &self,
        index: NSUInteger,
        sampler: Option<&SamplerStateRef>,
        lod_clamp: Range<f32>,
    ) {
        unsafe {
            msg_send![self,
                setSamplerState:sampler
                lodMinClamp:lod_clamp.start
                lodMaxClamp:lod_clamp.end
                atIndex:index
            ]
        }
    }

    pub fn set_bytes(&self, index: NSUInteger, length: NSUInteger, bytes: *const std::ffi::c_void) {
        unsafe {
            msg_send![self,
                setBytes: bytes
                length: length
                atIndex: index
            ]
        }
    }

    pub fn set_visible_function_table(
        &self,
        buffer_index: NSUInteger,
        visible_function_table: Option<&VisibleFunctionTableRef>,
    ) {
        unsafe {
            msg_send![self,
            setVisibleFunctionTable:visible_function_table
            atBufferIndex:buffer_index]
        }
    }

    pub fn set_visible_function_tables(
        &self,
        buffer_start_index: NSUInteger,
        visible_function_tables: &[&VisibleFunctionTableRef],
    ) {
        unsafe {
            msg_send![self,
                setVisibleFunctionTables:visible_function_tables.as_ptr()
                withBufferRange: NSRange {
                    location: buffer_start_index,
                    length: visible_function_tables.len() as _,
                }
            ]
        }
    }

    pub fn dispatch_thread_groups(
        &self,
        thread_groups_count: MTLSize,
        threads_per_threadgroup: MTLSize,
    ) {
        unsafe {
            msg_send![self,
                dispatchThreadgroups:thread_groups_count
                threadsPerThreadgroup:threads_per_threadgroup
            ]
        }
    }

    pub fn dispatch_threads(&self, threads_per_grid: MTLSize, threads_per_thread_group: MTLSize) {
        unsafe {
            msg_send![self,
                dispatchThreads:threads_per_grid
                threadsPerThreadgroup:threads_per_thread_group
            ]
        }
    }

    pub fn dispatch_thread_groups_indirect(
        &self,
        buffer: &BufferRef,
        offset: NSUInteger,
        threads_per_threadgroup: MTLSize,
    ) {
        unsafe {
            msg_send![self,
                dispatchThreadgroupsWithIndirectBuffer:buffer
                indirectBufferOffset:offset
                threadsPerThreadgroup:threads_per_threadgroup
            ]
        }
    }

    pub fn set_threadgroup_memory_length(&self, at_index: NSUInteger, size: NSUInteger) {
        unsafe {
            msg_send![self,
                setThreadgroupMemoryLength:size
                atIndex: at_index
            ]
        }
    }

    /// Encodes a barrier so that changes to a set of resources made by commands encoded before the
    /// barrier are completed before further commands are executed.
    ///
    /// Availability: iOS 12.0+, macOS 10.14+
    ///
    /// # Arguments
    /// * `resources`: A slice of resources.
    pub fn memory_barrier_with_resources(&self, resources: &[&ResourceRef]) {
        unsafe {
            msg_send![self,
                memoryBarrierWithResources: resources.as_ptr()
                count: resources.len() as NSUInteger
            ]
        }
    }

    /// Specifies that a resource in an argument buffer can be safely used by a compute pass.
    ///
    /// Availability: iOS 11.0+, macOS 10.13+
    ///
    /// # Arguments
    /// * `resource`: A specific resource within an argument buffer.
    /// * `usage`: The options that describe how the resource will be used by a compute function.
    pub fn use_resource(&self, resource: &ResourceRef, usage: MTLResourceUsage) {
        unsafe {
            msg_send![self,
                useResource: resource
                usage: usage
            ]
        }
    }

    /// Specifies that an array of resources in an argument buffer can be safely used by a compute pass.
    ///
    /// Availability: iOS 11.0+, macOS 10.13+
    ///
    /// See <https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/2866561-useresources>
    ///
    /// # Arguments
    /// * `resources`: A slice of resources within an argument buffer.
    /// * `usage`: The options that describe how the array of resources will be used by a compute function.
    pub fn use_resources(&self, resources: &[&ResourceRef], usage: MTLResourceUsage) {
        unsafe {
            msg_send![self,
                useResources: resources.as_ptr()
                count: resources.len() as NSUInteger
                usage: usage
            ]
        }
    }

    /// Specifies that a heap containing resources in an argument buffer can be safely used by a compute pass.
    ///
    /// Availability: iOS 11.0+, macOS 10.13+
    ///
    /// See <https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/2866530-useheap>
    ///
    /// # Arguments
    /// * `heap`: A heap that contains resources within an argument buffer.
    pub fn use_heap(&self, heap: &HeapRef) {
        unsafe { msg_send![self, useHeap: heap] }
    }

    /// Specifies that an array of heaps containing resources in an argument buffer can be safely
    /// used by a compute pass.
    ///
    /// Availability: iOS 11.0+, macOS 10.13+
    ///
    /// # Arguments
    /// * `heaps`: A slice of heaps that contains resources within an argument buffer.
    pub fn use_heaps(&self, heaps: &[&HeapRef]) {
        unsafe {
            msg_send![self,
                useHeaps: heaps.as_ptr()
                count: heaps.len() as NSUInteger
            ]
        }
    }

    pub fn update_fence(&self, fence: &FenceRef) {
        unsafe { msg_send![self, updateFence: fence] }
    }

    pub fn wait_for_fence(&self, fence: &FenceRef) {
        unsafe { msg_send![self, waitForFence: fence] }
    }

    /// Only available in (macos(11.0), ios(14.0))
    pub fn set_acceleration_structure(
        &self,
        index: NSUInteger,
        accel: Option<&AccelerationStructureRef>,
    ) {
        unsafe {
            msg_send![
                self,
                setAccelerationStructure: accel
                atBufferIndex: index
            ]
        }
    }

    /// Only available in (macos(11.0), ios(14.0))
    pub fn set_intersection_function_table(
        &self,
        index: NSUInteger,
        table: Option<&IntersectionFunctionTableRef>,
    ) {
        unsafe {
            msg_send![
                self,
                setIntersectionFunctionTable: table
                atBufferIndex: index
            ]
        }
    }

    /// See: <https://developer.apple.com/documentation/metal/mtlcomputecommandencoder/3194349-samplecountersinbuffer>
    pub fn sample_counters_in_buffer(
        &self,
        sample_buffer: &CounterSampleBufferRef,
        sample_index: NSUInteger,
        with_barrier: bool,
    ) {
        unsafe {
            msg_send![self,
                sampleCountersInBuffer: sample_buffer
                atSampleIndex: sample_index
                withBarrier: with_barrier
            ]
        }
    }
}

/// See <https://developer.apple.com/documentation/metal/mtlargumentencoder/>
pub enum MTLArgumentEncoder {}

foreign_obj_type! {
    type CType = MTLArgumentEncoder;
    pub struct ArgumentEncoder;
}

impl ArgumentEncoderRef {
    pub fn encoded_length(&self) -> NSUInteger {
        unsafe { msg_send![self, encodedLength] }
    }

    pub fn alignment(&self) -> NSUInteger {
        unsafe { msg_send![self, alignment] }
    }

    pub fn set_argument_buffer(&self, buffer: &BufferRef, offset: NSUInteger) {
        unsafe {
            msg_send![self,
                setArgumentBuffer: buffer
                offset: offset
            ]
        }
    }

    pub fn set_argument_buffer_to_element(
        &self,
        array_element: NSUInteger,
        buffer: &BufferRef,
        offset: NSUInteger,
    ) {
        unsafe {
            msg_send![self,
                setArgumentBuffer: buffer
                startOffset: offset
                arrayElement: array_element
            ]
        }
    }

    pub fn set_buffer(&self, at_index: NSUInteger, buffer: &BufferRef, offset: NSUInteger) {
        unsafe {
            msg_send![self,
                setBuffer: buffer
                offset: offset
                atIndex: at_index
            ]
        }
    }

    pub fn set_buffers(
        &self,
        start_index: NSUInteger,
        data: &[&BufferRef],
        offsets: &[NSUInteger],
    ) {
        assert_eq!(offsets.len(), data.len());
        unsafe {
            msg_send![self,
                setBuffers: data.as_ptr()
                offsets: offsets.as_ptr()
                withRange: NSRange {
                    location: start_index,
                    length: data.len() as _,
                }
            ]
        }
    }

    pub fn set_texture(&self, at_index: NSUInteger, texture: &TextureRef) {
        unsafe {
            msg_send![self,
                setTexture: texture
                atIndex: at_index
            ]
        }
    }

    pub fn set_textures(&self, start_index: NSUInteger, data: &[&TextureRef]) {
        unsafe {
            msg_send![self,
                setTextures: data.as_ptr()
                withRange: NSRange {
                    location: start_index,
                    length: data.len() as _,
                }
            ]
        }
    }

    pub fn set_sampler_state(&self, at_index: NSUInteger, sampler_state: &SamplerStateRef) {
        unsafe {
            msg_send![self,
                setSamplerState: sampler_state
                atIndex: at_index
            ]
        }
    }

    pub fn set_sampler_states(&self, start_index: NSUInteger, data: &[&SamplerStateRef]) {
        unsafe {
            msg_send![self,
                setSamplerStates: data.as_ptr()
                withRange: NSRange {
                    location: start_index,
                    length: data.len() as _,
                }
            ]
        }
    }

    pub fn set_render_pipeline_state(
        &self,
        at_index: NSUInteger,
        pipeline: &RenderPipelineStateRef,
    ) {
        unsafe {
            msg_send![self,
                setRenderPipelineState: pipeline
                atIndex: at_index
            ]
        }
    }

    pub fn set_render_pipeline_states(
        &self,
        start_index: NSUInteger,
        pipelines: &[&RenderPipelineStateRef],
    ) {
        unsafe {
            msg_send![self,
                setRenderPipelineStates: pipelines.as_ptr()
                withRange: NSRange {
                    location: start_index,
                    length: pipelines.len() as _,
                }
            ]
        }
    }

    pub fn constant_data(&self, at_index: NSUInteger) -> *mut std::ffi::c_void {
        unsafe { msg_send![self, constantDataAtIndex: at_index] }
    }

    pub fn set_indirect_command_buffer(
        &self,
        at_index: NSUInteger,
        buffer: &IndirectCommandBufferRef,
    ) {
        unsafe {
            msg_send![self,
                setIndirectCommandBuffer: buffer
                atIndex: at_index
            ]
        }
    }

    pub fn set_indirect_command_buffers(
        &self,
        start_index: NSUInteger,
        data: &[&IndirectCommandBufferRef],
    ) {
        unsafe {
            msg_send![self,
                setIndirectCommandBuffers: data.as_ptr()
                withRange: NSRange {
                    location: start_index,
                    length: data.len() as _,
                }
            ]
        }
    }

    pub fn new_argument_encoder_for_buffer(&self, index: NSUInteger) -> ArgumentEncoder {
        unsafe {
            let ptr = msg_send![self, newArgumentEncoderForBufferAtIndex: index];
            ArgumentEncoder::from_ptr(ptr)
        }
    }
}
