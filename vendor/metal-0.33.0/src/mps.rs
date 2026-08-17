// Copyright 2020 GFX developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use super::*;

use objc::runtime::BOOL;

#[cfg_attr(
    feature = "link",
    link(name = "MetalPerformanceShaders", kind = "framework")
)]
extern "C" {
    fn MPSSupportsMTLDevice(device: *const std::ffi::c_void) -> BOOL;
}

pub fn mps_supports_device(device: &DeviceRef) -> bool {
    let b: BOOL = unsafe {
        let ptr: *const DeviceRef = device;
        MPSSupportsMTLDevice(ptr as _)
    };
    b == YES
}

/// See <https://developer.apple.com/documentation/metalperformanceshaders/mpskernel>
pub enum MPSKernel {}

foreign_obj_type! {
    type CType = MPSKernel;
    pub struct Kernel;
}

/// See <https://developer.apple.com/documentation/metalperformanceshaders/mpsraydatatype>
pub enum MPSRayDataType {
    OriginDirection = 0,
    OriginMinDistanceDirectionMaxDistance = 1,
    OriginMaskDirectionMaxDistance = 2,
}

bitflags::bitflags! {
    /// See <https://developer.apple.com/documentation/metalperformanceshaders/mpsraymaskoptions>
    #[allow(non_upper_case_globals)]
    #[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub struct MPSRayMaskOptions: NSUInteger {
        /// Enable primitive masks
        const Primitive = 1;
        /// Enable instance masks
        const Instance = 2;
    }
}

/// Options that determine the data contained in an intersection result.
///
/// See <https://developer.apple.com/documentation/metalperformanceshaders/mpsintersectiondatatype>
pub enum MPSIntersectionDataType {
    Distance = 0,
    DistancePrimitiveIndex = 1,
    DistancePrimitiveIndexCoordinates = 2,
    DistancePrimitiveIndexInstanceIndex = 3,
    DistancePrimitiveIndexInstanceIndexCoordinates = 4,
}

/// See <https://developer.apple.com/documentation/metalperformanceshaders/mpsintersectiontype>
pub enum MPSIntersectionType {
    /// Find the closest intersection to the ray's origin along the ray direction.
    /// This is potentially slower than `Any` but is well suited to primary visibility rays.
    Nearest = 0,
    /// Find any intersection along the ray direction. This is potentially faster than `Nearest` and
    /// is well suited to shadow and occlusion rays.
    Any = 1,
}

/// See <https://developer.apple.com/documentation/metalperformanceshaders/mpsraymaskoperator>
pub enum MPSRayMaskOperator {
    /// Accept the intersection if `(primitive mask & ray mask) != 0`.
    And = 0,
    /// Accept the intersection if `~(primitive mask & ray mask) != 0`.
    NotAnd = 1,
    /// Accept the intersection if `(primitive mask | ray mask) != 0`.
    Or = 2,
    /// Accept the intersection if `~(primitive mask | ray mask) != 0`.
    NotOr = 3,
    /// Accept the intersection if `(primitive mask ^ ray mask) != 0`.
    /// Note that this is equivalent to the "!=" operator.
    Xor = 4,
    /// Accept the intersection if `~(primitive mask ^ ray mask) != 0`.
    /// Note that this is equivalent to the "==" operator.
    NotXor = 5,
    /// Accept the intersection if `(primitive mask < ray mask) != 0`.
    LessThan = 6,
    /// Accept the intersection if `(primitive mask <= ray mask) != 0`.
    LessThanOrEqualTo = 7,
    /// Accept the intersection if `(primitive mask > ray mask) != 0`.
    GreaterThan = 8,
    /// Accept the intersection if `(primitive mask >= ray mask) != 0`.
    GreaterThanOrEqualTo = 9,
}

/// See <https://developer.apple.com/documentation/metalperformanceshaders/mpstriangleintersectiontesttype>
pub enum MPSTriangleIntersectionTestType {
    /// Use the default ray/triangle intersection test
    Default = 0,
    /// Use a watertight ray/triangle intersection test which avoids gaps along shared triangle edges.
    /// Shared vertices may still have gaps.
    /// This intersection test may be slower than `Default`.
    Watertight = 1,
}

/// See <https://developer.apple.com/documentation/metalperformanceshaders/mpsaccelerationstructurestatus>
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum MPSAccelerationStructureStatus {
    Unbuilt = 0,
    Built = 1,
}

bitflags::bitflags! {
    /// See <https://developer.apple.com/documentation/metalperformanceshaders/mpsaccelerationstructureusage>
    #[allow(non_upper_case_globals)]
    #[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
    pub struct MPSAccelerationStructureUsage: NSUInteger {
        /// No usage options specified
        const None = 0;
        /// Option that enables support for refitting the acceleration structure after it has been built.
        const Refit = 1;
        /// Option indicating that the acceleration structure will be rebuilt frequently.
        const FrequentRebuild = 2;
        const PreferGPUBuild = 4;
        const PreferCPUBuild = 8;
    }
}

/// A common bit for all floating point data types.
const MPSDataTypeFloatBit: isize = 0x10000000;
const MPSDataTypeSignedBit: isize = 0x20000000;
const MPSDataTypeNormalizedBit: isize = 0x40000000;

/// See <https://developer.apple.com/documentation/metalperformanceshaders/mpsdatatype>
pub enum MPSDataType {
    Invalid = 0,

    Float32 = MPSDataTypeFloatBit | 32,
    Float16 = MPSDataTypeFloatBit | 16,

    // Signed integers.
    Int8 = MPSDataTypeSignedBit | 8,
    Int16 = MPSDataTypeSignedBit | 16,
    Int32 = MPSDataTypeSignedBit | 32,

    // Unsigned integers. Range: [0, UTYPE_MAX]
    UInt8 = 8,
    UInt16 = 16,
    UInt32 = 32,

    // Unsigned normalized. Range: [0, 1.0]
    Unorm1 = MPSDataTypeNormalizedBit | 1,
    Unorm8 = MPSDataTypeNormalizedBit | 8,
}

/// A kernel that performs intersection tests between rays and geometry.
///
/// See <https://developer.apple.com/documentation/metalperformanceshaders/mpsrayintersector>
pub enum MPSRayIntersector {}

foreign_obj_type! {
    type CType = MPSRayIntersector;
    pub struct RayIntersector;
    type ParentType = Kernel;
}

impl RayIntersector {
    pub fn from_device(device: &DeviceRef) -> Option<Self> {
        unsafe {
            let intersector: RayIntersector = msg_send![class!(MPSRayIntersector), alloc];
            let ptr: *mut Object = msg_send![intersector.as_ref(), initWithDevice: device];
            if ptr.is_null() {
                None
            } else {
                Some(intersector)
            }
        }
    }
}

impl RayIntersectorRef {
    pub fn set_cull_mode(&self, mode: MTLCullMode) {
        unsafe { msg_send![self, setCullMode: mode] }
    }

    pub fn set_front_facing_winding(&self, winding: MTLWinding) {
        unsafe { msg_send![self, setFrontFacingWinding: winding] }
    }

    pub fn set_intersection_data_type(&self, options: MPSIntersectionDataType) {
        unsafe { msg_send![self, setIntersectionDataType: options] }
    }

    pub fn set_intersection_stride(&self, stride: NSUInteger) {
        unsafe { msg_send![self, setIntersectionStride: stride] }
    }

    pub fn set_ray_data_type(&self, ty: MPSRayDataType) {
        unsafe { msg_send![self, setRayDataType: ty] }
    }

    pub fn set_ray_index_data_type(&self, ty: MPSDataType) {
        unsafe { msg_send![self, setRayIndexDataType: ty] }
    }

    pub fn set_ray_mask(&self, ray_mask: u32) {
        unsafe { msg_send![self, setRayMask: ray_mask] }
    }

    pub fn set_ray_mask_operator(&self, operator: MPSRayMaskOperator) {
        unsafe { msg_send![self, setRayMaskOperator: operator] }
    }

    pub fn set_ray_mask_options(&self, options: MPSRayMaskOptions) {
        unsafe { msg_send![self, setRayMaskOptions: options] }
    }

    pub fn set_ray_stride(&self, stride: NSUInteger) {
        unsafe { msg_send![self, setRayStride: stride] }
    }

    pub fn set_triangle_intersection_test_type(&self, test_type: MPSTriangleIntersectionTestType) {
        unsafe { msg_send![self, setTriangleIntersectionTestType: test_type] }
    }

    pub fn encode_intersection_to_command_buffer(
        &self,
        command_buffer: &CommandBufferRef,
        intersection_type: MPSIntersectionType,
        ray_buffer: &BufferRef,
        ray_buffer_offset: NSUInteger,
        intersection_buffer: &BufferRef,
        intersection_buffer_offset: NSUInteger,
        ray_count: NSUInteger,
        acceleration_structure: &AccelerationStructureRef,
    ) {
        unsafe {
            msg_send![
                self,
                encodeIntersectionToCommandBuffer: command_buffer
                intersectionType: intersection_type
                rayBuffer: ray_buffer
                rayBufferOffset: ray_buffer_offset
                intersectionBuffer: intersection_buffer
                intersectionBufferOffset: intersection_buffer_offset
                rayCount: ray_count
                accelerationStructure: acceleration_structure
            ]
        }
    }

    pub fn recommended_minimum_ray_batch_size_for_ray_count(
        &self,
        ray_count: NSUInteger,
    ) -> NSUInteger {
        unsafe { msg_send![self, recommendedMinimumRayBatchSizeForRayCount: ray_count] }
    }
}

/// A group of acceleration structures which may be used together in an instance acceleration structure.
///
/// See <https://developer.apple.com/documentation/metalperformanceshaders/mpsaccelerationstructuregroup>
pub enum MPSAccelerationStructureGroup {}

foreign_obj_type! {
    type CType = MPSAccelerationStructureGroup;
    pub struct AccelerationStructureGroup;
}

impl AccelerationStructureGroup {
    pub fn new_with_device(device: &DeviceRef) -> Option<Self> {
        unsafe {
            let group: AccelerationStructureGroup =
                msg_send![class!(MPSAccelerationStructureGroup), alloc];
            let ptr: *mut Object = msg_send![group.as_ref(), initWithDevice: device];
            if ptr.is_null() {
                None
            } else {
                Some(group)
            }
        }
    }
}

impl AccelerationStructureGroupRef {
    pub fn device(&self) -> &DeviceRef {
        unsafe { msg_send![self, device] }
    }
}

/// The base class for data structures that are built over geometry and used to accelerate ray tracing.
///
/// See <https://developer.apple.com/documentation/metalperformanceshaders/mpsaccelerationstructure>
pub enum MPSAccelerationStructure {}

foreign_obj_type! {
    type CType = MPSAccelerationStructure;
    pub struct AccelerationStructure;
}

impl AccelerationStructureRef {
    pub fn status(&self) -> MPSAccelerationStructureStatus {
        unsafe { msg_send![self, status] }
    }

    pub fn usage(&self) -> MPSAccelerationStructureUsage {
        unsafe { msg_send![self, usage] }
    }

    pub fn set_usage(&self, usage: MPSAccelerationStructureUsage) {
        unsafe { msg_send![self, setUsage: usage] }
    }

    pub fn group(&self) -> &AccelerationStructureGroupRef {
        unsafe { msg_send![self, group] }
    }

    pub fn encode_refit_to_command_buffer(&self, buffer: &CommandBufferRef) {
        unsafe { msg_send![self, encodeRefitToCommandBuffer: buffer] }
    }

    pub fn rebuild(&self) {
        unsafe { msg_send![self, rebuild] }
    }
}

/// See <https://developer.apple.com/documentation/metalperformanceshaders/mpspolygonaccelerationstructure>
pub enum MPSPolygonAccelerationStructure {}

foreign_obj_type! {
    type CType = MPSPolygonAccelerationStructure;
    pub struct PolygonAccelerationStructure;
    type ParentType = AccelerationStructure;
}

impl PolygonAccelerationStructureRef {
    pub fn set_index_buffer(&self, buffer: Option<&BufferRef>) {
        unsafe { msg_send![self, setIndexBuffer: buffer] }
    }

    pub fn set_index_buffer_offset(&self, offset: NSUInteger) {
        unsafe { msg_send![self, setIndexBufferOffset: offset] }
    }

    pub fn set_index_type(&self, data_type: MPSDataType) {
        unsafe { msg_send![self, setIndexType: data_type] }
    }

    pub fn set_mask_buffer(&self, buffer: Option<&BufferRef>) {
        unsafe { msg_send![self, setMaskBuffer: buffer] }
    }

    pub fn set_mask_buffer_offset(&self, offset: NSUInteger) {
        unsafe { msg_send![self, setMaskBufferOffset: offset] }
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
}

/// An acceleration structure built over triangles.
///
/// See <https://developer.apple.com/documentation/metalperformanceshaders/mpstriangleaccelerationstructure>
pub enum MPSTriangleAccelerationStructure {}

foreign_obj_type! {
    type CType = MPSTriangleAccelerationStructure;
    pub struct TriangleAccelerationStructure;
    type ParentType = PolygonAccelerationStructure;
}

impl TriangleAccelerationStructure {
    pub fn from_device(device: &DeviceRef) -> Option<Self> {
        unsafe {
            let structure: TriangleAccelerationStructure =
                msg_send![class!(MPSTriangleAccelerationStructure), alloc];
            let ptr: *mut Object = msg_send![structure.as_ref(), initWithDevice: device];
            if ptr.is_null() {
                None
            } else {
                Some(structure)
            }
        }
    }
}

impl TriangleAccelerationStructureRef {
    pub fn triangle_count(&self) -> NSUInteger {
        unsafe { msg_send![self, triangleCount] }
    }

    pub fn set_triangle_count(&self, count: NSUInteger) {
        unsafe { msg_send![self, setTriangleCount: count] }
    }
}

/// See <https://developer.apple.com/documentation/metalperformanceshaders/mpstransformtype>
#[repr(u64)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum MPSTransformType {
    Float4x4 = 0,
    Identity = 1,
}

/// An acceleration structure built over instances of other acceleration structures
///
/// See <https://developer.apple.com/documentation/metalperformanceshaders/mpsinstanceaccelerationstructure>
pub enum MPSInstanceAccelerationStructure {}

foreign_obj_type! {
    type CType = MPSInstanceAccelerationStructure;
    pub struct InstanceAccelerationStructure;
    type ParentType = AccelerationStructure;
}

impl InstanceAccelerationStructure {
    pub fn init_with_group(group: &AccelerationStructureGroupRef) -> Option<Self> {
        unsafe {
            let structure: InstanceAccelerationStructure =
                msg_send![class!(MPSInstanceAccelerationStructure), alloc];
            let ptr: *mut Object = msg_send![structure.as_ref(), initWithGroup: group];
            if ptr.is_null() {
                None
            } else {
                Some(structure)
            }
        }
    }
}

impl InstanceAccelerationStructureRef {
    /// Marshal to Rust Vec
    pub fn acceleration_structures(&self) -> Vec<PolygonAccelerationStructure> {
        unsafe {
            let acs: *mut Object = msg_send![self, accelerationStructures];
            let count: NSUInteger = msg_send![acs, count];
            (0..count)
                .map(|i| {
                    let ac = msg_send![acs, objectAtIndex: i];
                    PolygonAccelerationStructure::from_ptr(ac)
                })
                .collect()
        }
    }

    /// Marshal from Rust slice
    pub fn set_acceleration_structures(&self, acs: &[&PolygonAccelerationStructureRef]) {
        let ns_array = Array::<PolygonAccelerationStructure>::from_slice(acs);
        unsafe { msg_send![self, setAccelerationStructures: ns_array] }
    }

    pub fn instance_buffer(&self) -> &BufferRef {
        unsafe { msg_send![self, instanceBuffer] }
    }

    pub fn set_instance_buffer(&self, buffer: &BufferRef) {
        unsafe { msg_send![self, setInstanceBuffer: buffer] }
    }

    pub fn instance_buffer_offset(&self) -> NSUInteger {
        unsafe { msg_send![self, instanceBufferOffset] }
    }

    pub fn set_instance_buffer_offset(&self, offset: NSUInteger) {
        unsafe { msg_send![self, setInstanceBufferOffset: offset] }
    }

    pub fn transform_buffer(&self) -> &BufferRef {
        unsafe { msg_send![self, transformBuffer] }
    }

    pub fn set_transform_buffer(&self, buffer: &BufferRef) {
        unsafe { msg_send![self, setTransformBuffer: buffer] }
    }

    pub fn transform_buffer_offset(&self) -> NSUInteger {
        unsafe { msg_send![self, transformBufferOffset] }
    }

    pub fn set_transform_buffer_offset(&self, offset: NSUInteger) {
        unsafe { msg_send![self, setTransformBufferOffset: offset] }
    }

    pub fn transform_type(&self) -> MPSTransformType {
        unsafe { msg_send![self, transformType] }
    }

    pub fn set_transform_type(&self, transform_type: MPSTransformType) {
        unsafe { msg_send![self, setTransformType: transform_type] }
    }

    pub fn mask_buffer(&self) -> &BufferRef {
        unsafe { msg_send![self, maskBuffer] }
    }

    pub fn set_mask_buffer(&self, buffer: &BufferRef) {
        unsafe { msg_send![self, setMaskBuffer: buffer] }
    }

    pub fn mask_buffer_offset(&self) -> NSUInteger {
        unsafe { msg_send![self, maskBufferOffset] }
    }

    pub fn set_mask_buffer_offset(&self, offset: NSUInteger) {
        unsafe { msg_send![self, setMaskBufferOffset: offset] }
    }

    pub fn instance_count(&self) -> NSUInteger {
        unsafe { msg_send![self, instanceCount] }
    }

    pub fn set_instance_count(&self, count: NSUInteger) {
        unsafe { msg_send![self, setInstanceCount: count] }
    }
}

#[repr(C)]
pub struct MPSPackedFloat3 {
    pub elements: [f32; 3],
}

/// Represents a 3D ray with an origin, a direction, and an intersection distance range from the origin.
///
/// See <https://developer.apple.com/documentation/metalperformanceshaders/mpsrayoriginmindistancedirectionmaxdistance>
#[repr(C)]
pub struct MPSRayOriginMinDistanceDirectionMaxDistance {
    /// Ray origin. The intersection test will be skipped if the origin contains NaNs or infinities.
    pub origin: MPSPackedFloat3,
    /// Minimum intersection distance from the origin along the ray direction.
    /// The intersection test will be skipped if the minimum distance is equal to positive infinity or NaN.
    pub min_distance: f32,
    /// Ray direction. Does not need to be normalized. The intersection test will be skipped if
    /// the direction has length zero or contains NaNs or infinities.
    pub direction: MPSPackedFloat3,
    /// Maximum intersection distance from the origin along the ray direction. May be infinite.
    /// The intersection test will be skipped if the maximum distance is less than zero, NaN, or
    /// less than the minimum intersection distance.
    pub max_distance: f32,
}

/// Intersection result which contains the distance from the ray origin to the intersection point,
/// the index of the intersected primitive, and the first two barycentric coordinates of the intersection point.
///
/// See <https://developer.apple.com/documentation/metalperformanceshaders/mpsintersectiondistanceprimitiveindexcoordinates>
#[repr(C)]
pub struct MPSIntersectionDistancePrimitiveIndexCoordinates {
    /// Distance from the ray origin to the intersection point along the ray direction vector such
    /// that `intersection = ray.origin + ray.direction * distance`.
    /// Is negative if there is no intersection. If the intersection type is `MPSIntersectionTypeAny`,
    /// is a positive value for a hit or a negative value for a miss.
    pub distance: f32,
    /// Index of the intersected primitive. Undefined if the ray does not intersect a primitive or
    /// if the intersection type is `MPSIntersectionTypeAny`.
    pub primitive_index: u32,
    /// The first two barycentric coordinates `U` and `V` of the intersection point.
    /// The third coordinate `W = 1 - U - V`. Undefined if the ray does not intersect a primitive or
    /// if the intersection type is `MPSIntersectionTypeAny`.
    pub coordinates: [f32; 2],
}
