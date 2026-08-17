// Copyright 2023 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use super::*;

bitflags::bitflags! {
    #[derive(Copy, Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub struct MTLAccelerationStructureInstanceOptions: u32 {
        const None = 0;
        const DisableTriangleCulling = (1 << 0);
        const TriangleFrontFacingWindingCounterClockwise = (1 << 1);
        const Opaque = (1 << 2);
        const NonOpaque = (1 << 3);
    }
}

/// See <https://developer.apple.com/documentation/metal/mtlaccelerationstructureinstancedescriptortype>
#[repr(u64)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum MTLAccelerationStructureInstanceDescriptorType {
    Default = 0,
    UserID = 1,
    Motion = 2,
    Indirect = 3,
}

#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[repr(C)]
pub struct MTLAccelerationStructureInstanceDescriptor {
    pub transformation_matrix: [[f32; 3]; 4],
    pub options: MTLAccelerationStructureInstanceOptions,
    pub mask: u32,
    pub intersection_function_table_offset: u32,
    pub acceleration_structure_index: u32,
}

#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[repr(C)]
pub struct MTLAccelerationStructureUserIDInstanceDescriptor {
    pub transformation_matrix: [[f32; 3]; 4],
    pub options: MTLAccelerationStructureInstanceOptions,
    pub mask: u32,
    pub intersection_function_table_offset: u32,
    pub acceleration_structure_index: u32,
    pub user_id: u32,
}

#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[repr(C)]
pub struct MTLIndirectAccelerationStructureInstanceDescriptor {
    pub transformation_matrix: [[f32; 3]; 4],
    pub options: MTLAccelerationStructureInstanceOptions,
    pub mask: u32,
    pub intersection_function_table_offset: u32,
    pub user_id: u32,
    pub acceleration_structure_id: u64,
}

pub enum MTLAccelerationStructureDescriptor {}

foreign_obj_type! {
    type CType = MTLAccelerationStructureDescriptor;
    pub struct AccelerationStructureDescriptor;
    type ParentType = NsObject;
}

pub enum MTLPrimitiveAccelerationStructureDescriptor {}

foreign_obj_type! {
    type CType = MTLPrimitiveAccelerationStructureDescriptor;
    pub struct PrimitiveAccelerationStructureDescriptor;
    type ParentType = AccelerationStructureDescriptor;
}

impl PrimitiveAccelerationStructureDescriptor {
    pub fn descriptor() -> Self {
        unsafe {
            let class = class!(MTLPrimitiveAccelerationStructureDescriptor);
            let ptr: *mut Object = msg_send![class, descriptor];
            let ptr: *mut Object = msg_send![ptr, retain];
            Self::from_ptr(ptr as _)
        }
    }
}

impl PrimitiveAccelerationStructureDescriptorRef {
    pub fn set_geometry_descriptors(
        &self,
        descriptors: &ArrayRef<AccelerationStructureGeometryDescriptor>,
    ) {
        unsafe { msg_send![self, setGeometryDescriptors: descriptors] }
    }
}

pub enum MTLAccelerationStructure {}

foreign_obj_type! {
    type CType = MTLAccelerationStructure;
    pub struct AccelerationStructure;
    type ParentType = Resource;
}

impl AccelerationStructureRef {
    pub fn gpu_resource_id(&self) -> MTLResourceID {
        unsafe { msg_send![self, gpuResourceID] }
    }
}

pub enum MTLAccelerationStructureGeometryDescriptor {}

foreign_obj_type! {
    type CType = MTLAccelerationStructureGeometryDescriptor;
    pub struct AccelerationStructureGeometryDescriptor;
    type ParentType = NsObject;
}

impl AccelerationStructureGeometryDescriptorRef {
    pub fn set_opaque(&self, opaque: bool) {
        unsafe { msg_send![self, setOpaque: opaque] }
    }
    pub fn set_primitive_data_buffer(&self, buffer: Option<&BufferRef>) {
        unsafe { msg_send![self, setPrimitiveDataBuffer: buffer] }
    }

    pub fn set_primitive_data_stride(&self, stride: NSUInteger) {
        unsafe { msg_send![self, setPrimitiveDataStride: stride] }
    }

    pub fn set_primitive_data_element_size(&self, size: NSUInteger) {
        unsafe { msg_send![self, setPrimitiveDataElementSize: size] }
    }

    pub fn set_intersection_function_table_offset(&self, offset: NSUInteger) {
        unsafe { msg_send![self, setIntersectionFunctionTableOffset: offset] }
    }
}

pub enum MTLAccelerationStructureTriangleGeometryDescriptor {}

foreign_obj_type! {
    type CType = MTLAccelerationStructureTriangleGeometryDescriptor;
    pub struct AccelerationStructureTriangleGeometryDescriptor;
    type ParentType = AccelerationStructureGeometryDescriptor;
}

impl AccelerationStructureTriangleGeometryDescriptor {
    pub fn descriptor() -> Self {
        unsafe {
            let class = class!(MTLAccelerationStructureTriangleGeometryDescriptor);
            let ptr: *mut Object = msg_send![class, descriptor];
            let ptr: *mut Object = msg_send![ptr, retain];
            Self::from_ptr(ptr as _)
        }
    }
}

impl AccelerationStructureTriangleGeometryDescriptorRef {
    pub fn set_index_buffer(&self, buffer: Option<&BufferRef>) {
        unsafe { msg_send![self, setIndexBuffer: buffer] }
    }

    pub fn set_index_buffer_offset(&self, offset: NSUInteger) {
        unsafe { msg_send![self, setIndexBufferOffset: offset] }
    }

    pub fn set_index_type(&self, t: MTLIndexType) {
        unsafe { msg_send![self, setIndexType: t] }
    }

    pub fn set_vertex_buffer(&self, buffer: Option<&BufferRef>) {
        unsafe { msg_send![self, setVertexBuffer: buffer] }
    }

    pub fn set_vertex_buffer_offset(&self, offset: NSUInteger) {
        unsafe { msg_send![self, setVertexBufferOffset: offset] }
    }

    pub fn set_vertex_stride(&self, stride: NSUInteger) {
        unsafe { msg_send![self, setVertexStride: stride] }
    }

    pub fn set_triangle_count(&self, count: NSUInteger) {
        unsafe { msg_send![self, setTriangleCount: count] }
    }

    pub fn set_vertex_format(&self, format: MTLAttributeFormat) {
        unsafe { msg_send![self, setVertexFormat: format] }
    }

    pub fn set_transformation_matrix_buffer(&self, buffer: Option<&BufferRef>) {
        unsafe { msg_send![self, setTransformationMatrixBuffer: buffer] }
    }

    pub fn set_transformation_matrix_buffer_offset(&self, offset: NSUInteger) {
        unsafe { msg_send![self, setTransformationMatrixBufferOffset: offset] }
    }
}

pub enum MTLAccelerationStructureBoundingBoxGeometryDescriptor {}

foreign_obj_type! {
    type CType = MTLAccelerationStructureBoundingBoxGeometryDescriptor;
    pub struct AccelerationStructureBoundingBoxGeometryDescriptor;
    type ParentType = AccelerationStructureGeometryDescriptor;
}

impl AccelerationStructureBoundingBoxGeometryDescriptor {
    pub fn descriptor() -> Self {
        unsafe {
            let class = class!(MTLAccelerationStructureBoundingBoxGeometryDescriptor);
            let ptr: *mut Object = msg_send![class, descriptor];
            let ptr: *mut Object = msg_send![ptr, retain];
            Self::from_ptr(ptr as _)
        }
    }
}

impl AccelerationStructureBoundingBoxGeometryDescriptorRef {
    pub fn set_bounding_box_buffer(&self, buffer: Option<&BufferRef>) {
        unsafe { msg_send![self, setBoundingBoxBuffer: buffer] }
    }

    pub fn set_bounding_box_count(&self, count: NSUInteger) {
        unsafe { msg_send![self, setBoundingBoxCount: count] }
    }

    pub fn set_bounding_box_stride(&self, stride: NSUInteger) {
        unsafe { msg_send![self, setBoundingBoxStride: stride] }
    }

    pub fn set_bounding_box_buffer_offset(&self, offset: NSUInteger) {
        unsafe { msg_send![self, setBoundingBoxBufferOffset: offset] }
    }
}

pub enum MTLInstanceAccelerationStructureDescriptor {}

foreign_obj_type! {
    type CType = MTLInstanceAccelerationStructureDescriptor;
    pub struct InstanceAccelerationStructureDescriptor;
    type ParentType = AccelerationStructureDescriptor;
}

impl InstanceAccelerationStructureDescriptor {
    pub fn descriptor() -> Self {
        unsafe {
            let class = class!(MTLInstanceAccelerationStructureDescriptor);
            let ptr: *mut Object = msg_send![class, descriptor];
            let ptr: *mut Object = msg_send![ptr, retain];
            Self::from_ptr(ptr as _)
        }
    }
}

impl InstanceAccelerationStructureDescriptorRef {
    pub fn set_instance_descriptor_type(&self, ty: MTLAccelerationStructureInstanceDescriptorType) {
        unsafe { msg_send![self, setInstanceDescriptorType: ty] }
    }

    pub fn set_instanced_acceleration_structures(
        &self,
        instances: &ArrayRef<AccelerationStructure>,
    ) {
        unsafe { msg_send![self, setInstancedAccelerationStructures: instances] }
    }

    pub fn set_instance_count(&self, count: NSUInteger) {
        unsafe { msg_send![self, setInstanceCount: count] }
    }

    pub fn set_instance_descriptor_buffer(&self, buffer: &BufferRef) {
        unsafe { msg_send![self, setInstanceDescriptorBuffer: buffer] }
    }

    pub fn set_instance_descriptor_buffer_offset(&self, offset: NSUInteger) {
        unsafe { msg_send![self, setInstanceDescriptorBufferOffset: offset] }
    }

    pub fn set_instance_descriptor_stride(&self, stride: NSUInteger) {
        unsafe { msg_send![self, setInstanceDescriptorStride: stride] }
    }
}

pub enum MTLIndirectInstanceAccelerationStructureDescriptor {}

foreign_obj_type! {
    type CType = MTLIndirectInstanceAccelerationStructureDescriptor;
    pub struct IndirectInstanceAccelerationStructureDescriptor;
    type ParentType = AccelerationStructureDescriptor;
}

impl IndirectInstanceAccelerationStructureDescriptor {
    pub fn descriptor() -> Self {
        unsafe {
            let class = class!(MTLIndirectInstanceAccelerationStructureDescriptor);
            let ptr: *mut Object = msg_send![class, descriptor];
            let ptr: *mut Object = msg_send![ptr, retain];
            Self::from_ptr(ptr as _)
        }
    }
}

impl IndirectInstanceAccelerationStructureDescriptorRef {
    pub fn set_instance_descriptor_buffer(&self, buffer: &BufferRef) {
        unsafe { msg_send![self, setInstanceDescriptorBuffer: buffer] }
    }

    pub fn set_instance_descriptor_buffer_offset(&self, offset: NSUInteger) {
        unsafe { msg_send![self, setInstanceDescriptorBufferOffset: offset] }
    }

    pub fn set_instance_descriptor_stride(&self, stride: NSUInteger) {
        unsafe { msg_send![self, setInstanceDescriptorStride: stride] }
    }

    pub fn set_max_instance_count(&self, count: NSUInteger) {
        unsafe { msg_send![self, setMaxInstanceCount: count] }
    }

    pub fn set_instance_count_buffer(&self, buffer: &BufferRef) {
        unsafe { msg_send![self, setInstanceCountBuffer: buffer] }
    }

    pub fn set_instance_count_buffer_offset(&self, offset: NSUInteger) {
        unsafe { msg_send![self, setInstanceCountBufferOffset: offset] }
    }

    pub fn set_instance_descriptor_type(&self, ty: MTLAccelerationStructureInstanceDescriptorType) {
        unsafe { msg_send![self, setInstanceDescriptorType: ty] }
    }

    pub fn set_motion_transform_buffer(&self, buffer: &BufferRef) {
        unsafe { msg_send![self, setMotionTransformBuffer: buffer] }
    }

    pub fn set_motion_transform_buffer_offset(&self, offset: NSUInteger) {
        unsafe { msg_send![self, setMotionTransformBufferOffset: offset] }
    }

    pub fn set_max_motion_transform_count(&self, count: NSUInteger) {
        unsafe { msg_send![self, setMaxMotionTransformCount: count] }
    }

    pub fn set_motion_transform_count_buffer(&self, buffer: &BufferRef) {
        unsafe { msg_send![self, setMotionTransformCountBuffer: buffer] }
    }

    pub fn set_motion_transform_count_buffer_offset(&self, offset: NSUInteger) {
        unsafe { msg_send![self, setMotionTransformCountBufferOffset: offset] }
    }
}

pub enum MTLAccelerationStructureCommandEncoder {}

foreign_obj_type! {
    type CType = MTLAccelerationStructureCommandEncoder;
    pub struct  AccelerationStructureCommandEncoder;
    type ParentType = CommandEncoder;
}

impl AccelerationStructureCommandEncoderRef {
    pub fn build_acceleration_structure(
        &self,
        acceleration_structure: &self::AccelerationStructureRef,
        descriptor: &self::AccelerationStructureDescriptorRef,
        scratch_buffer: &BufferRef,
        scratch_buffer_offset: NSUInteger,
    ) {
        unsafe {
            msg_send![
                self,
                buildAccelerationStructure: acceleration_structure
                descriptor: descriptor
                scratchBuffer: scratch_buffer
                scratchBufferOffset: scratch_buffer_offset
            ]
        }
    }

    pub fn copy_acceleration_structure(
        &self,
        source_acceleration_structure: &AccelerationStructureRef,
        destination_acceleration_structure: &AccelerationStructureRef,
    ) {
        unsafe {
            msg_send![
                self,
                copyAccelerationStructure: source_acceleration_structure
                toAccelerationStructure: destination_acceleration_structure
            ]
        }
    }

    pub fn write_compacted_acceleration_structure_size(
        &self,
        acceleration_structure: &AccelerationStructureRef,
        to_buffer: &BufferRef,
        offset: NSUInteger,
    ) {
        unsafe {
            msg_send![
                self,
                writeCompactedAccelerationStructureSize: acceleration_structure
                toBuffer: to_buffer
                offset: offset
            ]
        }
    }

    pub fn write_compacted_acceleration_structure_size_with_type(
        &self,
        acceleration_structure: &AccelerationStructureRef,
        to_buffer: &BufferRef,
        offset: NSUInteger,
        size_data_type: MTLDataType,
    ) {
        unsafe {
            msg_send![
                self,
                writeCompactedAccelerationStructureSize: acceleration_structure
                toBuffer: to_buffer
                offset: offset
                sizeDataType: size_data_type
            ]
        }
    }

    pub fn copy_and_compact_acceleration_structure(
        &self,
        source: &AccelerationStructureRef,
        destination: &AccelerationStructureRef,
    ) {
        unsafe {
            msg_send![
                self,
                copyAndCompactAccelerationStructure: source
                toAccelerationStructure: destination
            ]
        }
    }

    pub fn refit_acceleration_structure(
        &self,
        source_acceleration_structure: &AccelerationStructureRef,
        descriptor: &self::AccelerationStructureDescriptorRef,
        destination_acceleration_structure: Option<&AccelerationStructureRef>,
        scratch_buffer: &BufferRef,
        scratch_buffer_offset: NSUInteger,
    ) {
        unsafe {
            msg_send![
                self,
                refitAccelerationStructure: source_acceleration_structure
                descriptor: descriptor
                destination: destination_acceleration_structure
                scratchBuffer: scratch_buffer
                scratchBufferOffset: scratch_buffer_offset
            ]
        }
    }

    pub fn update_fence(&self, fence: &FenceRef) {
        unsafe { msg_send![self, updateFence: fence] }
    }

    pub fn wait_for_fence(&self, fence: &FenceRef) {
        unsafe { msg_send![self, waitForFence: fence] }
    }

    pub fn use_heap(&self, heap: &HeapRef) {
        unsafe { msg_send![self, useHeap: heap] }
    }

    pub fn use_heaps(&self, heaps: &[&HeapRef]) {
        unsafe {
            msg_send![self,
                useHeaps: heaps.as_ptr()
                count: heaps.len() as NSUInteger
            ]
        }
    }

    pub fn use_resource(&self, resource: &ResourceRef, usage: MTLResourceUsage) {
        unsafe {
            msg_send![self,
                useResource: resource
                usage: usage
            ]
        }
    }

    pub fn use_resources(&self, resources: &[&ResourceRef], usage: MTLResourceUsage) {
        unsafe {
            msg_send![self,
                useResources: resources.as_ptr()
                count: resources.len() as NSUInteger
                usage: usage
            ]
        }
    }

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

pub enum MTLIntersectionFunctionTableDescriptor {}

foreign_obj_type! {
    type CType = MTLIntersectionFunctionTableDescriptor;
    pub struct IntersectionFunctionTableDescriptor;
    type ParentType = NsObject;
}

impl IntersectionFunctionTableDescriptor {
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLIntersectionFunctionTableDescriptor);
            let this: *mut <Self as ForeignType>::CType = msg_send![class, alloc];
            msg_send![this, init]
        }
    }
}

impl IntersectionFunctionTableDescriptorRef {
    pub fn set_function_count(&self, count: NSUInteger) {
        unsafe { msg_send![self, setFunctionCount: count] }
    }
}

pub enum MTLIntersectionFunctionTable {}

foreign_obj_type! {
    type CType = MTLIntersectionFunctionTable;
    pub struct IntersectionFunctionTable;
    type ParentType = Resource;
}

impl IntersectionFunctionTableRef {
    pub fn set_function(&self, function: &FunctionHandleRef, index: NSUInteger) {
        unsafe { msg_send![self, setFunction: function atIndex: index] }
    }

    pub fn set_functions(&self, functions: &[&FunctionHandleRef], start_index: NSUInteger) {
        unsafe {
            msg_send![self, setFunctions: functions.as_ptr() withRange: NSRange { location: start_index, length: functions.len() as _ }]
        }
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
            }]
        }
    }

    pub fn gpu_resource_id(&self) -> MTLResourceID {
        unsafe { msg_send![self, gpuResourceID] }
    }
}

/// See <https://developer.apple.com/documentation/metal/mtlvisiblefunctiontabledescriptor>
pub enum MTLVisibleFunctionTableDescriptor {}

foreign_obj_type! {
    type CType = MTLVisibleFunctionTableDescriptor;
    pub struct VisibleFunctionTableDescriptor;
    type ParentType = NsObject;
}

impl VisibleFunctionTableDescriptor {
    pub fn new() -> Self {
        unsafe {
            let class = class!(MTLVisibleFunctionTableDescriptor);
            msg_send![class, new]
        }
    }
}

impl VisibleFunctionTableDescriptorRef {
    pub fn set_function_count(&self, count: NSUInteger) {
        unsafe { msg_send![self, setFunctionCount: count] }
    }
}

/// See <https://developer.apple.com/documentation/metal/mtlvisiblefunctiontable>
pub enum MTLVisibleFunctionTable {}

foreign_obj_type! {
    type CType = MTLVisibleFunctionTable;
    pub struct VisibleFunctionTable;
    type ParentType = Resource;
}

impl VisibleFunctionTableRef {
    pub fn set_functions(&self, functions: &[&FunctionRef]) {
        let ns_array = Array::<Function>::from_slice(functions);
        unsafe { msg_send![self, setFunctions: ns_array] }
    }

    pub fn set_function(&self, index: NSUInteger, function: &FunctionHandleRef) {
        unsafe { msg_send![self, setFunction: function atIndex: index] }
    }

    pub fn gpu_resource_id(&self) -> MTLResourceID {
        unsafe { msg_send![self, gpuResourceID] }
    }
}
