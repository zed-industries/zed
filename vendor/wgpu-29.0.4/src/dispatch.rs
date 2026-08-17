//! Infrastructure for dispatching calls to the appropriate "backend". The "backends" are:
//!
//! - `wgpu_core`: An implementation of the the wgpu api on top of various native graphics APIs.
//! - `webgpu`: An implementation of the wgpu api which calls WebGPU directly.
//!
//! The interface traits are all object safe and listed in the `InterfaceTypes` trait.
//!
//! The method for dispatching should optimize well if only one backend is
//! compiled in, as-if there was no dispatching at all. See the comments on
//! [`dispatch_types`] for details.
//!
//! [`dispatch_types`]: macro.dispatch_types.html

#![allow(
    drop_bounds,
    reason = "This exists to remind implementors to impl drop."
)]
#![allow(clippy::too_many_arguments, reason = "It's fine.")]
#![allow(
    missing_docs,
    clippy::missing_safety_doc,
    reason = "Interfaces are not documented"
)]
#![allow(
    clippy::len_without_is_empty,
    reason = "trait is minimal, not ergonomic"
)]

use crate::{Blas, Tlas, WasmNotSend, WasmNotSendSync, WriteOnly};

use alloc::{boxed::Box, string::String, sync::Arc, vec::Vec};
use core::{any::Any, fmt::Debug, future::Future, hash::Hash, ops::Range, pin::Pin};

#[cfg(custom)]
use crate::backend::custom::*;
#[cfg(webgpu)]
use crate::backend::webgpu::*;
#[cfg(wgpu_core)]
use crate::backend::wgpu_core::*;

/// Create a single trait with the given supertraits and a blanket impl for all types that implement them.
///
/// This is useful for creating a trait alias as a shorthand.
macro_rules! trait_alias {
    ($name:ident: $($bound:tt)+) => {
        pub trait $name: $($bound)+ {}
        impl<T: $($bound)+> $name for T {}
    };
}

// Various return futures in the API.
trait_alias!(RequestAdapterFuture: Future<Output = Result<DispatchAdapter, wgt::RequestAdapterError>> + WasmNotSend + 'static);
trait_alias!(RequestDeviceFuture: Future<Output = Result<(DispatchDevice, DispatchQueue), crate::RequestDeviceError>> + WasmNotSend + 'static);
trait_alias!(PopErrorScopeFuture: Future<Output = Option<crate::Error>> + WasmNotSend + 'static);
trait_alias!(ShaderCompilationInfoFuture: Future<Output = crate::CompilationInfo> + WasmNotSend + 'static);
trait_alias!(EnumerateAdapterFuture: Future<Output = Vec<DispatchAdapter>> + WasmNotSend + 'static);

// We can't use trait aliases here, as you can't convert from a dyn Trait to dyn Supertrait _yet_.
#[cfg(send_sync)]
pub type BoxDeviceLostCallback = Box<dyn FnOnce(crate::DeviceLostReason, String) + Send + 'static>;
#[cfg(not(send_sync))]
pub type BoxDeviceLostCallback = Box<dyn FnOnce(crate::DeviceLostReason, String) + 'static>;
#[cfg(send_sync)]
pub type BoxSubmittedWorkDoneCallback = Box<dyn FnOnce() + Send + 'static>;
#[cfg(not(send_sync))]
pub type BoxSubmittedWorkDoneCallback = Box<dyn FnOnce() + 'static>;
#[cfg(send_sync)]
pub type BufferMapCallback = Box<dyn FnOnce(Result<(), crate::BufferAsyncError>) + Send + 'static>;
#[cfg(not(send_sync))]
pub type BufferMapCallback = Box<dyn FnOnce(Result<(), crate::BufferAsyncError>) + 'static>;

#[cfg(send_sync)]
pub type BlasCompactCallback = Box<dyn FnOnce(Result<(), crate::BlasAsyncError>) + Send + 'static>;
#[cfg(not(send_sync))]
pub type BlasCompactCallback = Box<dyn FnOnce(Result<(), crate::BlasAsyncError>) + 'static>;

// remove when rust 1.86
#[cfg_attr(not(custom), expect(dead_code))]
pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
}

impl<T: 'static> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Common traits on all the interface traits
trait_alias!(CommonTraits: AsAny + Any + Debug + WasmNotSendSync);

pub trait InstanceInterface: CommonTraits {
    fn new(desc: crate::InstanceDescriptor) -> Self
    where
        Self: Sized;

    unsafe fn create_surface(
        &self,
        target: crate::SurfaceTargetUnsafe,
    ) -> Result<DispatchSurface, crate::CreateSurfaceError>;

    fn request_adapter(
        &self,
        options: &crate::RequestAdapterOptions<'_, '_>,
    ) -> Pin<Box<dyn RequestAdapterFuture>>;

    fn poll_all_devices(&self, force_wait: bool) -> bool;

    #[cfg(feature = "wgsl")]
    fn wgsl_language_features(&self) -> crate::WgslLanguageFeatures;

    fn enumerate_adapters(&self, backends: crate::Backends)
        -> Pin<Box<dyn EnumerateAdapterFuture>>;
}

pub trait AdapterInterface: CommonTraits {
    fn request_device(
        &self,
        desc: &crate::DeviceDescriptor<'_>,
    ) -> Pin<Box<dyn RequestDeviceFuture>>;

    fn is_surface_supported(&self, surface: &DispatchSurface) -> bool;

    fn features(&self) -> crate::Features;

    fn limits(&self) -> crate::Limits;

    fn downlevel_capabilities(&self) -> crate::DownlevelCapabilities;

    fn get_info(&self) -> crate::AdapterInfo;

    fn get_texture_format_features(
        &self,
        format: crate::TextureFormat,
    ) -> crate::TextureFormatFeatures;

    fn get_presentation_timestamp(&self) -> crate::PresentationTimestamp;

    fn cooperative_matrix_properties(&self) -> Vec<crate::wgt::CooperativeMatrixProperties>;
}

pub trait DeviceInterface: CommonTraits {
    fn features(&self) -> crate::Features;
    fn limits(&self) -> crate::Limits;
    fn adapter_info(&self) -> crate::AdapterInfo;

    fn create_shader_module(
        &self,
        desc: crate::ShaderModuleDescriptor<'_>,
        shader_bound_checks: crate::ShaderRuntimeChecks,
    ) -> DispatchShaderModule;

    unsafe fn create_shader_module_passthrough(
        &self,
        desc: &crate::ShaderModuleDescriptorPassthrough<'_>,
    ) -> DispatchShaderModule;

    fn create_bind_group_layout(
        &self,
        desc: &crate::BindGroupLayoutDescriptor<'_>,
    ) -> DispatchBindGroupLayout;
    fn create_bind_group(&self, desc: &crate::BindGroupDescriptor<'_>) -> DispatchBindGroup;
    fn create_pipeline_layout(
        &self,
        desc: &crate::PipelineLayoutDescriptor<'_>,
    ) -> DispatchPipelineLayout;
    fn create_render_pipeline(
        &self,
        desc: &crate::RenderPipelineDescriptor<'_>,
    ) -> DispatchRenderPipeline;
    fn create_mesh_pipeline(
        &self,
        desc: &crate::MeshPipelineDescriptor<'_>,
    ) -> DispatchRenderPipeline;
    fn create_compute_pipeline(
        &self,
        desc: &crate::ComputePipelineDescriptor<'_>,
    ) -> DispatchComputePipeline;
    unsafe fn create_pipeline_cache(
        &self,
        desc: &crate::PipelineCacheDescriptor<'_>,
    ) -> DispatchPipelineCache;
    fn create_buffer(&self, desc: &crate::BufferDescriptor<'_>) -> DispatchBuffer;
    fn create_texture(&self, desc: &crate::TextureDescriptor<'_>) -> DispatchTexture;
    fn create_external_texture(
        &self,
        desc: &crate::ExternalTextureDescriptor<'_>,
        planes: &[&crate::TextureView],
    ) -> DispatchExternalTexture;
    fn create_blas(
        &self,
        desc: &crate::CreateBlasDescriptor<'_>,
        sizes: crate::BlasGeometrySizeDescriptors,
    ) -> (Option<u64>, DispatchBlas);
    fn create_tlas(&self, desc: &crate::CreateTlasDescriptor<'_>) -> DispatchTlas;
    fn create_sampler(&self, desc: &crate::SamplerDescriptor<'_>) -> DispatchSampler;
    fn create_query_set(&self, desc: &crate::QuerySetDescriptor<'_>) -> DispatchQuerySet;
    fn create_command_encoder(
        &self,
        desc: &crate::CommandEncoderDescriptor<'_>,
    ) -> DispatchCommandEncoder;
    fn create_render_bundle_encoder(
        &self,
        desc: &crate::RenderBundleEncoderDescriptor<'_>,
    ) -> DispatchRenderBundleEncoder;

    fn set_device_lost_callback(&self, device_lost_callback: BoxDeviceLostCallback);

    fn on_uncaptured_error(&self, handler: Arc<dyn crate::UncapturedErrorHandler>);
    // Returns index on the stack of the pushed error scope.
    fn push_error_scope(&self, filter: crate::ErrorFilter) -> u32;
    fn pop_error_scope(&self, index: u32) -> Pin<Box<dyn PopErrorScopeFuture>>;

    unsafe fn start_graphics_debugger_capture(&self);
    unsafe fn stop_graphics_debugger_capture(&self);

    fn poll(&self, poll_type: wgt::PollType<u64>) -> Result<crate::PollStatus, crate::PollError>;

    fn get_internal_counters(&self) -> crate::InternalCounters;
    fn generate_allocator_report(&self) -> Option<crate::AllocatorReport>;

    fn destroy(&self);
}

pub trait QueueInterface: CommonTraits {
    fn write_buffer(&self, buffer: &DispatchBuffer, offset: crate::BufferAddress, data: &[u8]);

    fn create_staging_buffer(&self, size: crate::BufferSize) -> Option<DispatchQueueWriteBuffer>;
    fn validate_write_buffer(
        &self,
        buffer: &DispatchBuffer,
        offset: crate::BufferAddress,
        size: crate::BufferSize,
    ) -> Option<()>;
    fn write_staging_buffer(
        &self,
        buffer: &DispatchBuffer,
        offset: crate::BufferAddress,
        staging_buffer: &DispatchQueueWriteBuffer,
    );

    fn write_texture(
        &self,
        texture: crate::TexelCopyTextureInfo<'_>,
        data: &[u8],
        data_layout: crate::TexelCopyBufferLayout,
        size: crate::Extent3d,
    );
    #[cfg(web)]
    fn copy_external_image_to_texture(
        &self,
        source: &crate::CopyExternalImageSourceInfo,
        dest: crate::CopyExternalImageDestInfo<&crate::api::Texture>,
        size: crate::Extent3d,
    );

    /// Submit must always drain the iterator, even in the case of error.
    fn submit(&self, command_buffers: &mut dyn Iterator<Item = DispatchCommandBuffer>) -> u64;

    fn get_timestamp_period(&self) -> f32;
    fn on_submitted_work_done(&self, callback: BoxSubmittedWorkDoneCallback);

    fn compact_blas(&self, blas: &DispatchBlas) -> (Option<u64>, DispatchBlas);
}

pub trait ShaderModuleInterface: CommonTraits {
    fn get_compilation_info(&self) -> Pin<Box<dyn ShaderCompilationInfoFuture>>;
}
pub trait BindGroupLayoutInterface: CommonTraits {}
pub trait BindGroupInterface: CommonTraits {}
pub trait TextureViewInterface: CommonTraits {}
pub trait SamplerInterface: CommonTraits {}
pub trait BufferInterface: CommonTraits {
    fn map_async(
        &self,
        mode: crate::MapMode,
        range: Range<crate::BufferAddress>,
        callback: BufferMapCallback,
    );
    fn get_mapped_range(&self, sub_range: Range<crate::BufferAddress>)
        -> DispatchBufferMappedRange;

    fn unmap(&self);

    fn destroy(&self);
}
pub trait TextureInterface: CommonTraits {
    fn create_view(&self, desc: &crate::TextureViewDescriptor<'_>) -> DispatchTextureView;

    fn destroy(&self);
}
pub trait ExternalTextureInterface: CommonTraits {
    fn destroy(&self);
}
pub trait BlasInterface: CommonTraits {
    fn prepare_compact_async(&self, callback: BlasCompactCallback);
    fn ready_for_compaction(&self) -> bool;
}
pub trait TlasInterface: CommonTraits {}
pub trait QuerySetInterface: CommonTraits {}
pub trait PipelineLayoutInterface: CommonTraits {}
pub trait RenderPipelineInterface: CommonTraits {
    fn get_bind_group_layout(&self, index: u32) -> DispatchBindGroupLayout;
}
pub trait ComputePipelineInterface: CommonTraits {
    fn get_bind_group_layout(&self, index: u32) -> DispatchBindGroupLayout;
}
pub trait PipelineCacheInterface: CommonTraits {
    fn get_data(&self) -> Option<Vec<u8>>;
}
pub trait CommandEncoderInterface: CommonTraits {
    fn copy_buffer_to_buffer(
        &self,
        source: &DispatchBuffer,
        source_offset: crate::BufferAddress,
        destination: &DispatchBuffer,
        destination_offset: crate::BufferAddress,
        copy_size: Option<crate::BufferAddress>,
    );
    fn copy_buffer_to_texture(
        &self,
        source: crate::TexelCopyBufferInfo<'_>,
        destination: crate::TexelCopyTextureInfo<'_>,
        copy_size: crate::Extent3d,
    );
    fn copy_texture_to_buffer(
        &self,
        source: crate::TexelCopyTextureInfo<'_>,
        destination: crate::TexelCopyBufferInfo<'_>,
        copy_size: crate::Extent3d,
    );
    fn copy_texture_to_texture(
        &self,
        source: crate::TexelCopyTextureInfo<'_>,
        destination: crate::TexelCopyTextureInfo<'_>,
        copy_size: crate::Extent3d,
    );

    fn begin_compute_pass(&self, desc: &crate::ComputePassDescriptor<'_>) -> DispatchComputePass;
    fn begin_render_pass(&self, desc: &crate::RenderPassDescriptor<'_>) -> DispatchRenderPass;
    fn finish(&mut self) -> DispatchCommandBuffer;

    fn clear_texture(
        &self,
        texture: &DispatchTexture,
        subresource_range: &crate::ImageSubresourceRange,
    );
    fn clear_buffer(
        &self,
        buffer: &DispatchBuffer,
        offset: crate::BufferAddress,
        size: Option<crate::BufferAddress>,
    );

    fn insert_debug_marker(&self, label: &str);
    fn push_debug_group(&self, label: &str);
    fn pop_debug_group(&self);

    fn write_timestamp(&self, query_set: &DispatchQuerySet, query_index: u32);
    fn resolve_query_set(
        &self,
        query_set: &DispatchQuerySet,
        first_query: u32,
        query_count: u32,
        destination: &DispatchBuffer,
        destination_offset: crate::BufferAddress,
    );
    fn mark_acceleration_structures_built<'a>(
        &self,
        blas: &mut dyn Iterator<Item = &'a Blas>,
        tlas: &mut dyn Iterator<Item = &'a Tlas>,
    );

    fn build_acceleration_structures<'a>(
        &self,
        blas: &mut dyn Iterator<Item = &'a crate::BlasBuildEntry<'a>>,
        tlas: &mut dyn Iterator<Item = &'a crate::Tlas>,
    );

    fn transition_resources<'a>(
        &mut self,
        buffer_transitions: &mut dyn Iterator<Item = wgt::BufferTransition<&'a DispatchBuffer>>,
        texture_transitions: &mut dyn Iterator<Item = wgt::TextureTransition<&'a DispatchTexture>>,
    );
}
pub trait ComputePassInterface: CommonTraits + Drop {
    fn set_pipeline(&mut self, pipeline: &DispatchComputePipeline);
    fn set_bind_group(
        &mut self,
        index: u32,
        bind_group: Option<&DispatchBindGroup>,
        offsets: &[crate::DynamicOffset],
    );
    fn set_immediates(&mut self, offset: u32, data: &[u8]);

    fn insert_debug_marker(&mut self, label: &str);
    fn push_debug_group(&mut self, group_label: &str);
    fn pop_debug_group(&mut self);

    fn write_timestamp(&mut self, query_set: &DispatchQuerySet, query_index: u32);
    fn begin_pipeline_statistics_query(&mut self, query_set: &DispatchQuerySet, query_index: u32);
    fn end_pipeline_statistics_query(&mut self);

    fn dispatch_workgroups(&mut self, x: u32, y: u32, z: u32);
    fn dispatch_workgroups_indirect(
        &mut self,
        indirect_buffer: &DispatchBuffer,
        indirect_offset: crate::BufferAddress,
    );
}
pub trait RenderPassInterface: CommonTraits + Drop {
    fn set_pipeline(&mut self, pipeline: &DispatchRenderPipeline);
    fn set_bind_group(
        &mut self,
        index: u32,
        bind_group: Option<&DispatchBindGroup>,
        offsets: &[crate::DynamicOffset],
    );
    fn set_index_buffer(
        &mut self,
        buffer: &DispatchBuffer,
        index_format: crate::IndexFormat,
        offset: crate::BufferAddress,
        size: Option<crate::BufferSize>,
    );
    fn set_vertex_buffer(
        &mut self,
        slot: u32,
        buffer: &DispatchBuffer,
        offset: crate::BufferAddress,
        size: Option<crate::BufferSize>,
    );
    fn set_immediates(&mut self, offset: u32, data: &[u8]);
    fn set_blend_constant(&mut self, color: crate::Color);
    fn set_scissor_rect(&mut self, x: u32, y: u32, width: u32, height: u32);
    fn set_viewport(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        min_depth: f32,
        max_depth: f32,
    );
    fn set_stencil_reference(&mut self, reference: u32);

    fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>);
    fn draw_indexed(&mut self, indices: Range<u32>, base_vertex: i32, instances: Range<u32>);
    fn draw_mesh_tasks(&mut self, group_count_x: u32, group_count_y: u32, group_count_z: u32);
    fn draw_indirect(
        &mut self,
        indirect_buffer: &DispatchBuffer,
        indirect_offset: crate::BufferAddress,
    );
    fn draw_indexed_indirect(
        &mut self,
        indirect_buffer: &DispatchBuffer,
        indirect_offset: crate::BufferAddress,
    );
    fn draw_mesh_tasks_indirect(
        &mut self,
        indirect_buffer: &DispatchBuffer,
        indirect_offset: crate::BufferAddress,
    );

    fn multi_draw_indirect(
        &mut self,
        indirect_buffer: &DispatchBuffer,
        indirect_offset: crate::BufferAddress,
        count: u32,
    );
    fn multi_draw_indexed_indirect(
        &mut self,
        indirect_buffer: &DispatchBuffer,
        indirect_offset: crate::BufferAddress,
        count: u32,
    );
    fn multi_draw_indirect_count(
        &mut self,
        indirect_buffer: &DispatchBuffer,
        indirect_offset: crate::BufferAddress,
        count_buffer: &DispatchBuffer,
        count_buffer_offset: crate::BufferAddress,
        max_count: u32,
    );
    fn multi_draw_mesh_tasks_indirect(
        &mut self,
        indirect_buffer: &DispatchBuffer,
        indirect_offset: crate::BufferAddress,
        count: u32,
    );
    fn multi_draw_indexed_indirect_count(
        &mut self,
        indirect_buffer: &DispatchBuffer,
        indirect_offset: crate::BufferAddress,
        count_buffer: &DispatchBuffer,
        count_buffer_offset: crate::BufferAddress,
        max_count: u32,
    );
    fn multi_draw_mesh_tasks_indirect_count(
        &mut self,
        indirect_buffer: &DispatchBuffer,
        indirect_offset: crate::BufferAddress,
        count_buffer: &DispatchBuffer,
        count_buffer_offset: crate::BufferAddress,
        max_count: u32,
    );

    fn insert_debug_marker(&mut self, label: &str);
    fn push_debug_group(&mut self, group_label: &str);
    fn pop_debug_group(&mut self);

    fn write_timestamp(&mut self, query_set: &DispatchQuerySet, query_index: u32);
    fn begin_occlusion_query(&mut self, query_index: u32);
    fn end_occlusion_query(&mut self);
    fn begin_pipeline_statistics_query(&mut self, query_set: &DispatchQuerySet, query_index: u32);
    fn end_pipeline_statistics_query(&mut self);

    fn execute_bundles(&mut self, render_bundles: &mut dyn Iterator<Item = &DispatchRenderBundle>);
}

pub trait RenderBundleEncoderInterface: CommonTraits {
    fn set_pipeline(&mut self, pipeline: &DispatchRenderPipeline);
    fn set_bind_group(
        &mut self,
        index: u32,
        bind_group: Option<&DispatchBindGroup>,
        offsets: &[crate::DynamicOffset],
    );
    fn set_index_buffer(
        &mut self,
        buffer: &DispatchBuffer,
        index_format: crate::IndexFormat,
        offset: crate::BufferAddress,
        size: Option<crate::BufferSize>,
    );
    fn set_vertex_buffer(
        &mut self,
        slot: u32,
        buffer: &DispatchBuffer,
        offset: crate::BufferAddress,
        size: Option<crate::BufferSize>,
    );
    fn set_immediates(&mut self, offset: u32, data: &[u8]);

    fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>);
    fn draw_indexed(&mut self, indices: Range<u32>, base_vertex: i32, instances: Range<u32>);
    fn draw_indirect(
        &mut self,
        indirect_buffer: &DispatchBuffer,
        indirect_offset: crate::BufferAddress,
    );
    fn draw_indexed_indirect(
        &mut self,
        indirect_buffer: &DispatchBuffer,
        indirect_offset: crate::BufferAddress,
    );

    fn finish(self, desc: &crate::RenderBundleDescriptor<'_>) -> DispatchRenderBundle
    where
        Self: Sized;
}

pub trait CommandBufferInterface: CommonTraits {}
pub trait RenderBundleInterface: CommonTraits {}

pub trait SurfaceInterface: CommonTraits {
    fn get_capabilities(&self, adapter: &DispatchAdapter) -> crate::SurfaceCapabilities;

    fn configure(&self, device: &DispatchDevice, config: &crate::SurfaceConfiguration);
    fn get_current_texture(
        &self,
    ) -> (
        Option<DispatchTexture>,
        crate::SurfaceStatus,
        DispatchSurfaceOutputDetail,
    );
}

pub trait SurfaceOutputDetailInterface: CommonTraits {
    fn present(&self);
    fn texture_discard(&self);
}

pub trait QueueWriteBufferInterface: CommonTraits {
    fn len(&self) -> usize;

    /// # Safety
    ///
    /// Must only be used on write, not read, mappings.
    unsafe fn write_slice(&mut self) -> WriteOnly<'_, [u8]>;
}

pub trait BufferMappedRangeInterface: CommonTraits {
    fn len(&self) -> usize;

    /// # Safety
    ///
    /// Must only be used on read, not write, mappings.
    unsafe fn read_slice(&self) -> &[u8];

    /// # Safety
    ///
    /// Must only be used on write, not read, mappings.
    unsafe fn write_slice(&mut self) -> WriteOnly<'_, [u8]>;

    #[cfg(webgpu)]
    fn as_uint8array(&self) -> &js_sys::Uint8Array;
}

/// Generates a dispatch type for some `wgpu` API type.
///
/// Invocations of this macro take one of the following forms:
///
/// ```ignore
/// dispatch_types! {mut type D: I = Core, Web, Dyn }
/// dispatch_types! {ref type D: I = Core, Web, Dyn }
/// ```
///
/// This defines `D` as a type that dereferences to a `dyn I` trait object. Most uses of
/// `D` in the rest of this crate just call the methods from the `dyn I` object, not from
/// `D` itself.
///
/// Internally, `D` is an enum with up to three variants holding values of type `Core`,
/// `Web`, and `Dyn`, all of which must implement `I`. `Core`, `Web` and `Dyn` are the
/// types from the `wgpu_core`, `webgpu`, and `custom` submodules of `wgpu::backend` that
/// correspond to `D`. The macro generates `Deref` and `DerefMut` implementations that
/// match on this enum and produce a `dyn I` reference for each variant.
///
/// The macro's `mut type` form defines `D` as the unique owner of the backend type, with
/// a `DerefMut` implementation, and `as_*_mut` methods that return `&mut` references.
/// This `D` does not implement `Clone`.
///
/// The macro's `ref type` form defines `D` to hold an `Arc` pointing to the backend type,
/// permitting `Clone` and `Deref`, but losing exclusive, mutable access.
///
/// For example:
///
/// ```ignore
/// dispatch_types! {ref type DispatchBuffer: BufferInterface =
///                  CoreBuffer, WebBuffer, DynBuffer}
/// ```
///
/// This defines `DispatchBuffer` as a type that dereferences to `&dyn BufferInterface`,
/// which has methods like `map_async` and `destroy`. The enum would be:
///
/// ```ignore
/// pub enum DispatchBuffer {
///     #[cfg(wgpu_core)]
///     Core(Arc<CoreBuffer>),
///     #[cfg(webgpu)]
///     WebGPU(WebBuffer),
///     #[cfg(custom)]
///     Custom(DynBuffer),
/// }
/// ```
///
/// This macro also defines `as_*` methods so that the backend implementations can
/// dereference other arguments.
///
/// ## Devirtualization
///
/// The dispatch types generated by this macro are carefully designed to allow the
/// compiler to completely devirtualize calls in most circumstances.
///
/// Note that every variant of the enum generated by this macro is under a `#[cfg]`.
/// Naturally, the `match` expressions in the `Deref` and `DerefMut` implementations have
/// matching `#[cfg]` attributes on each match arm.
///
/// In practice, when `wgpu`'s `"custom"` feature is not enabled, there is usually only
/// one variant in the `enum`, making it effectively a newtype around the sole variant's
/// data: it has no discriminant to branch on, and the `match` expressions are removed
/// entirely by the compiler.
///
/// In this case, when we invoke a method from the interface trait `I` on a dispatch type,
/// the `Deref` and `DerefMut` implementations' `match` statements build a `&dyn I` for
/// the data, on which we immediately invoke a method. The vtable is a constant, allowing
/// the Rust compiler to turn the `dyn` method call into an ordinary method call. This
/// creates opportunities for inlining.
///
/// Similarly, the `as_*` methods are free when there is only one backend.
macro_rules! dispatch_types {
    (
        ref type $name:ident: $interface:ident = $core_type:ident,$webgpu_type:ident,$custom_type:ident
    ) => {
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
        pub enum $name {
            #[cfg(wgpu_core)]
            Core(Arc<$core_type>),
            #[cfg(webgpu)]
            WebGPU($webgpu_type),
            #[allow(clippy::allow_attributes, private_interfaces)]
            #[cfg(custom)]
            Custom($custom_type),
        }

        impl $name {
            #[cfg(wgpu_core)]
            #[inline]
            #[allow(clippy::allow_attributes, unused)]
            pub fn as_core(&self) -> &$core_type {
                match self {
                    Self::Core(value) => value,
                    _ => panic!(concat!(stringify!($name), " is not core")),
                }
            }

            #[cfg(wgpu_core)]
            #[inline]
            #[allow(clippy::allow_attributes, unused)]
            pub fn as_core_opt(&self) -> Option<&$core_type> {
                match self {
                    Self::Core(value) => Some(value),
                    _ => None,
                }
            }

            #[cfg(custom)]
            #[inline]
            #[allow(clippy::allow_attributes, unused)]
            pub fn as_custom<T: $interface>(&self) -> Option<&T> {
                match self {
                    Self::Custom(value) => value.downcast(),
                    _ => None,
                }
            }

            #[cfg(webgpu)]
            #[inline]
            #[allow(clippy::allow_attributes, unused)]
            pub fn as_webgpu(&self) -> &$webgpu_type {
                match self {
                    Self::WebGPU(value) => value,
                    _ => panic!(concat!(stringify!($name), " is not webgpu")),
                }
            }

            #[cfg(webgpu)]
            #[inline]
            #[allow(clippy::allow_attributes, unused)]
            pub fn as_webgpu_opt(&self) -> Option<&$webgpu_type> {
                match self {
                    Self::WebGPU(value) => Some(value),
                    _ => None,
                }
            }

            #[cfg(custom)]
            #[inline]
            pub fn custom<T: $interface>(t: T) -> Self {
                Self::Custom($custom_type::new(t))
            }
        }

        #[cfg(wgpu_core)]
        impl From<$core_type> for $name {
            #[inline]
            fn from(value: $core_type) -> Self {
                Self::Core(Arc::new(value))
            }
        }

        #[cfg(webgpu)]
        impl From<$webgpu_type> for $name {
            #[inline]
            fn from(value: $webgpu_type) -> Self {
                Self::WebGPU(value)
            }
        }

        impl core::ops::Deref for $name {
            type Target = dyn $interface;

            #[inline]
            fn deref(&self) -> &Self::Target {
                match self {
                    #[cfg(wgpu_core)]
                    Self::Core(value) => value.as_ref(),
                    #[cfg(webgpu)]
                    Self::WebGPU(value) => value,
                    #[cfg(custom)]
                    Self::Custom(value) => value.deref(),
                    #[cfg(not(any(wgpu_core, webgpu)))]
                    _ => panic!("No context available. You need to enable one of wgpu's backend feature build flags."),
                }
            }
        }
    };
    (
        mut type $name:ident: $interface:ident = $core_type:ident,$webgpu_type:ident,$custom_type:ident
    ) => {
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum $name {
            #[cfg(wgpu_core)]
            Core($core_type),
            #[cfg(webgpu)]
            WebGPU($webgpu_type),
            #[allow(clippy::allow_attributes, private_interfaces)]
            #[cfg(custom)]
            Custom($custom_type),
        }

        impl $name {
            #[cfg(wgpu_core)]
            #[inline]
            #[allow(clippy::allow_attributes, unused)]
            pub fn as_core(&self) -> &$core_type {
                match self {
                    Self::Core(value) => value,
                    _ => panic!(concat!(stringify!($name), " is not core")),
                }
            }

            #[cfg(wgpu_core)]
            #[inline]
            #[allow(clippy::allow_attributes, unused)]
            pub fn as_core_mut(&mut self) -> &mut $core_type {
                match self {
                    Self::Core(value) => value,
                    _ => panic!(concat!(stringify!($name), " is not core")),
                }
            }

            #[cfg(wgpu_core)]
            #[inline]
            #[allow(clippy::allow_attributes, unused)]
            pub fn as_core_opt(&self) -> Option<&$core_type> {
                match self {
                    Self::Core(value) => Some(value),
                    _ => None,
                }
            }

            #[cfg(wgpu_core)]
            #[inline]
            #[allow(clippy::allow_attributes, unused)]
            pub fn as_core_mut_opt(
                &mut self,
            ) -> Option<&mut $core_type> {
                match self {
                    Self::Core(value) => Some(value),
                    _ => None,
                }
            }

            #[cfg(custom)]
            #[inline]
            #[allow(clippy::allow_attributes, unused)]
            pub fn as_custom<T: $interface>(&self) -> Option<&T> {
                match self {
                    Self::Custom(value) => value.downcast(),
                    _ => None,
                }
            }

            #[cfg(webgpu)]
            #[inline]
            #[allow(clippy::allow_attributes, unused)]
            pub fn as_webgpu(&self) -> &$webgpu_type {
                match self {
                    Self::WebGPU(value) => value,
                    _ => panic!(concat!(stringify!($name), " is not webgpu")),
                }
            }

            #[cfg(webgpu)]
            #[inline]
            #[allow(clippy::allow_attributes, unused)]
            pub fn as_webgpu_mut(&mut self) -> &mut $webgpu_type {
                match self {
                    Self::WebGPU(value) => value,
                    _ => panic!(concat!(stringify!($name), " is not webgpu")),
                }
            }

            #[cfg(webgpu)]
            #[inline]
            #[allow(clippy::allow_attributes, unused)]
            pub fn as_webgpu_opt(&self) -> Option<&$webgpu_type> {
                match self {
                    Self::WebGPU(value) => Some(value),
                    _ => None,
                }
            }

            #[cfg(webgpu)]
            #[inline]
            #[allow(clippy::allow_attributes, unused)]
            pub fn as_webgpu_mut_opt(
                &mut self,
            ) -> Option<&mut $webgpu_type> {
                match self {
                    Self::WebGPU(value) => Some(value),
                    _ => None,
                }
            }

            #[cfg(custom)]
            #[inline]
            pub fn custom<T: $interface>(t: T) -> Self {
                Self::Custom($custom_type::new(t))
            }
        }

        #[cfg(wgpu_core)]
        impl From<$core_type> for $name {
            #[inline]
            fn from(value: $core_type) -> Self {
                Self::Core(value)
            }
        }

        #[cfg(webgpu)]
        impl From<$webgpu_type> for $name {
            #[inline]
            fn from(value: $webgpu_type) -> Self {
                Self::WebGPU(value)
            }
        }

        impl core::ops::Deref for $name {
            type Target = dyn $interface;

            #[inline]
            fn deref(&self) -> &Self::Target {
                match self {
                    #[cfg(wgpu_core)]
                    Self::Core(value) => value,
                    #[cfg(webgpu)]
                    Self::WebGPU(value) => value,
                    #[cfg(custom)]
                    Self::Custom(value) => value.deref(),
                    #[cfg(not(any(wgpu_core, webgpu)))]
                    _ => panic!("No context available. You need to enable one of wgpu's backend feature build flags."),
                }
            }
        }

        impl core::ops::DerefMut for $name {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                match self {
                    #[cfg(wgpu_core)]
                    Self::Core(value) => value,
                    #[cfg(webgpu)]
                    Self::WebGPU(value) => value,
                    #[cfg(custom)]
                    Self::Custom(value) => value.deref_mut(),
                    #[cfg(not(any(wgpu_core, webgpu)))]
                    _ => panic!("No context available. You need to enable one of wgpu's backend feature build flags."),
                }
            }
        }
    };
}

dispatch_types! {ref type DispatchInstance: InstanceInterface = ContextWgpuCore, ContextWebGpu, DynContext}
dispatch_types! {ref type DispatchAdapter: AdapterInterface = CoreAdapter, WebAdapter, DynAdapter}
dispatch_types! {ref type DispatchDevice: DeviceInterface = CoreDevice, WebDevice, DynDevice}
dispatch_types! {ref type DispatchQueue: QueueInterface = CoreQueue, WebQueue, DynQueue}
dispatch_types! {ref type DispatchShaderModule: ShaderModuleInterface = CoreShaderModule, WebShaderModule, DynShaderModule}
dispatch_types! {ref type DispatchBindGroupLayout: BindGroupLayoutInterface = CoreBindGroupLayout, WebBindGroupLayout, DynBindGroupLayout}
dispatch_types! {ref type DispatchBindGroup: BindGroupInterface = CoreBindGroup, WebBindGroup, DynBindGroup}
dispatch_types! {ref type DispatchTextureView: TextureViewInterface = CoreTextureView, WebTextureView, DynTextureView}
dispatch_types! {ref type DispatchSampler: SamplerInterface = CoreSampler, WebSampler, DynSampler}
dispatch_types! {ref type DispatchBuffer: BufferInterface = CoreBuffer, WebBuffer, DynBuffer}
dispatch_types! {ref type DispatchTexture: TextureInterface = CoreTexture, WebTexture, DynTexture}
dispatch_types! {ref type DispatchExternalTexture: ExternalTextureInterface = CoreExternalTexture, WebExternalTexture, DynExternalTexture}
dispatch_types! {ref type DispatchBlas: BlasInterface = CoreBlas, WebBlas, DynBlas}
dispatch_types! {ref type DispatchTlas: TlasInterface = CoreTlas, WebTlas, DynTlas}
dispatch_types! {ref type DispatchQuerySet: QuerySetInterface = CoreQuerySet, WebQuerySet, DynQuerySet}
dispatch_types! {ref type DispatchPipelineLayout: PipelineLayoutInterface = CorePipelineLayout, WebPipelineLayout, DynPipelineLayout}
dispatch_types! {ref type DispatchRenderPipeline: RenderPipelineInterface = CoreRenderPipeline, WebRenderPipeline, DynRenderPipeline}
dispatch_types! {ref type DispatchComputePipeline: ComputePipelineInterface = CoreComputePipeline, WebComputePipeline, DynComputePipeline}
dispatch_types! {ref type DispatchPipelineCache: PipelineCacheInterface = CorePipelineCache, WebPipelineCache, DynPipelineCache}
dispatch_types! {mut type DispatchCommandEncoder: CommandEncoderInterface = CoreCommandEncoder, WebCommandEncoder, DynCommandEncoder}
dispatch_types! {mut type DispatchComputePass: ComputePassInterface = CoreComputePass, WebComputePassEncoder, DynComputePass}
dispatch_types! {mut type DispatchRenderPass: RenderPassInterface = CoreRenderPass, WebRenderPassEncoder, DynRenderPass}
dispatch_types! {mut type DispatchCommandBuffer: CommandBufferInterface = CoreCommandBuffer, WebCommandBuffer, DynCommandBuffer}
dispatch_types! {mut type DispatchRenderBundleEncoder: RenderBundleEncoderInterface = CoreRenderBundleEncoder, WebRenderBundleEncoder, DynRenderBundleEncoder}
dispatch_types! {ref type DispatchRenderBundle: RenderBundleInterface = CoreRenderBundle, WebRenderBundle, DynRenderBundle}
dispatch_types! {ref type DispatchSurface: SurfaceInterface = CoreSurface, WebSurface, DynSurface}
dispatch_types! {ref type DispatchSurfaceOutputDetail: SurfaceOutputDetailInterface = CoreSurfaceOutputDetail, WebSurfaceOutputDetail, DynSurfaceOutputDetail}
dispatch_types! {mut type DispatchQueueWriteBuffer: QueueWriteBufferInterface = CoreQueueWriteBuffer, WebQueueWriteBuffer, DynQueueWriteBuffer}
dispatch_types! {mut type DispatchBufferMappedRange: BufferMappedRangeInterface = CoreBufferMappedRange, WebBufferMappedRange, DynBufferMappedRange}
