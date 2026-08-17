#![allow(unused_variables)]

use alloc::{string::String, vec, vec::Vec};
use core::{ptr, sync::atomic::Ordering, time::Duration};

#[cfg(supports_64bit_atomics)]
use core::sync::atomic::AtomicU64;
#[cfg(not(supports_64bit_atomics))]
use portable_atomic::AtomicU64;

use crate::TlasInstance;

mod buffer;
pub use buffer::Buffer;
mod command;
pub use command::CommandBuffer;

#[derive(Clone, Debug)]
pub struct Api;
pub struct Context;
#[derive(Debug)]
pub struct Encoder;
#[derive(Debug)]
pub struct Resource;

#[derive(Debug)]
pub struct Fence {
    value: AtomicU64,
}

type DeviceResult<T> = Result<T, crate::DeviceError>;

impl crate::Api for Api {
    const VARIANT: wgt::Backend = wgt::Backend::Noop;

    type Instance = Context;
    type Surface = Context;
    type Adapter = Context;
    type Device = Context;

    type Queue = Context;
    type CommandEncoder = CommandBuffer;
    type CommandBuffer = CommandBuffer;

    type Buffer = Buffer;
    type Texture = Resource;
    type SurfaceTexture = Resource;
    type TextureView = Resource;
    type Sampler = Resource;
    type QuerySet = Resource;
    type Fence = Fence;
    type AccelerationStructure = Resource;
    type PipelineCache = Resource;

    type BindGroupLayout = Resource;
    type BindGroup = Resource;
    type PipelineLayout = Resource;
    type ShaderModule = Resource;
    type RenderPipeline = Resource;
    type ComputePipeline = Resource;
}

crate::impl_dyn_resource!(Buffer, CommandBuffer, Context, Fence, Resource);

impl crate::DynAccelerationStructure for Resource {}
impl crate::DynBindGroup for Resource {}
impl crate::DynBindGroupLayout for Resource {}
impl crate::DynBuffer for Buffer {}
impl crate::DynCommandBuffer for CommandBuffer {}
impl crate::DynComputePipeline for Resource {}
impl crate::DynFence for Fence {}
impl crate::DynPipelineCache for Resource {}
impl crate::DynPipelineLayout for Resource {}
impl crate::DynQuerySet for Resource {}
impl crate::DynRenderPipeline for Resource {}
impl crate::DynSampler for Resource {}
impl crate::DynShaderModule for Resource {}
impl crate::DynSurfaceTexture for Resource {}
impl crate::DynTexture for Resource {}
impl crate::DynTextureView for Resource {}

impl core::borrow::Borrow<dyn crate::DynTexture> for Resource {
    fn borrow(&self) -> &dyn crate::DynTexture {
        self
    }
}

impl crate::Instance for Context {
    type A = Api;

    unsafe fn init(desc: &crate::InstanceDescriptor<'_>) -> Result<Self, crate::InstanceError> {
        let crate::InstanceDescriptor {
            backend_options:
                wgt::BackendOptions {
                    noop: wgt::NoopBackendOptions { enable },
                    ..
                },
            name: _,
            flags: _,
            memory_budget_thresholds: _,
            telemetry: _,
            display: _,
        } = *desc;
        if enable {
            Ok(Context)
        } else {
            Err(crate::InstanceError::new(String::from(
                "noop backend disabled because NoopBackendOptions::enable is false",
            )))
        }
    }
    unsafe fn create_surface(
        &self,
        _display_handle: raw_window_handle::RawDisplayHandle,
        _window_handle: raw_window_handle::RawWindowHandle,
    ) -> Result<Context, crate::InstanceError> {
        Ok(Context)
    }
    unsafe fn enumerate_adapters(
        &self,
        _surface_hint: Option<&Context>,
    ) -> Vec<crate::ExposedAdapter<Api>> {
        vec![crate::ExposedAdapter {
            adapter: Context,
            info: adapter_info(),
            features: wgt::Features::all(),
            capabilities: CAPABILITIES,
        }]
    }
}

/// Returns the adapter info for the noop backend.
///
/// This is used in the test harness to construct info about
/// the noop backend adapter without actually initializing wgpu.
pub fn adapter_info() -> wgt::AdapterInfo {
    wgt::AdapterInfo {
        name: String::from("noop wgpu backend"),
        vendor: 0,
        device: 0,
        device_type: wgt::DeviceType::Cpu,
        device_pci_bus_id: String::new(),
        driver: String::from("wgpu"),
        driver_info: String::new(),
        backend: wgt::Backend::Noop,
        subgroup_min_size: wgt::MINIMUM_SUBGROUP_MIN_SIZE,
        subgroup_max_size: wgt::MAXIMUM_SUBGROUP_MAX_SIZE,
        transient_saves_memory: false,
    }
}

/// The capabilities of the noop backend.
///
/// This is used in the test harness to construct capabilities
/// of the noop backend without actually initializing wgpu.
pub const CAPABILITIES: crate::Capabilities = {
    /// Guaranteed to be no bigger than isize::MAX which is the maximum size of an allocation,
    /// except on 16-bit platforms which we certainly don’t fit in.
    const ALLOC_MAX_U32: u32 = i32::MAX as u32;
    /// Guaranteed to be no bigger than isize::MAX which is the maximum size of an allocation,
    /// except on 16-bit platforms which we certainly don’t fit in.
    const ALLOC_MAX_U64: u64 = i32::MAX as u64;

    crate::Capabilities {
        limits: wgt::Limits {
            // All maximally permissive
            max_texture_dimension_1d: ALLOC_MAX_U32,
            max_texture_dimension_2d: ALLOC_MAX_U32,
            max_texture_dimension_3d: ALLOC_MAX_U32,
            max_texture_array_layers: ALLOC_MAX_U32,
            max_bind_groups: ALLOC_MAX_U32,
            max_bindings_per_bind_group: ALLOC_MAX_U32,
            max_dynamic_uniform_buffers_per_pipeline_layout: ALLOC_MAX_U32,
            max_dynamic_storage_buffers_per_pipeline_layout: ALLOC_MAX_U32,
            max_sampled_textures_per_shader_stage: ALLOC_MAX_U32,
            max_samplers_per_shader_stage: ALLOC_MAX_U32,
            max_storage_buffers_per_shader_stage: ALLOC_MAX_U32,
            max_storage_textures_per_shader_stage: ALLOC_MAX_U32,
            max_uniform_buffers_per_shader_stage: ALLOC_MAX_U32,
            max_binding_array_elements_per_shader_stage: ALLOC_MAX_U32,
            max_binding_array_sampler_elements_per_shader_stage: ALLOC_MAX_U32,
            max_binding_array_acceleration_structure_elements_per_shader_stage: ALLOC_MAX_U32,
            max_uniform_buffer_binding_size: ALLOC_MAX_U64,
            max_storage_buffer_binding_size: ALLOC_MAX_U64,
            max_vertex_buffers: ALLOC_MAX_U32,
            max_buffer_size: ALLOC_MAX_U64,
            max_vertex_attributes: ALLOC_MAX_U32,
            max_vertex_buffer_array_stride: ALLOC_MAX_U32,
            max_inter_stage_shader_variables: ALLOC_MAX_U32,
            min_uniform_buffer_offset_alignment: 1,
            min_storage_buffer_offset_alignment: 1,
            max_color_attachments: ALLOC_MAX_U32,
            max_color_attachment_bytes_per_sample: ALLOC_MAX_U32,
            max_compute_workgroup_storage_size: ALLOC_MAX_U32,
            max_compute_invocations_per_workgroup: ALLOC_MAX_U32,
            max_compute_workgroup_size_x: ALLOC_MAX_U32,
            max_compute_workgroup_size_y: ALLOC_MAX_U32,
            max_compute_workgroup_size_z: ALLOC_MAX_U32,
            max_compute_workgroups_per_dimension: ALLOC_MAX_U32,
            max_immediate_size: ALLOC_MAX_U32,
            max_non_sampler_bindings: ALLOC_MAX_U32,

            max_task_mesh_workgroup_total_count: ALLOC_MAX_U32,
            max_task_mesh_workgroups_per_dimension: ALLOC_MAX_U32,
            max_task_invocations_per_workgroup: ALLOC_MAX_U32,
            max_task_invocations_per_dimension: ALLOC_MAX_U32,
            max_mesh_invocations_per_workgroup: ALLOC_MAX_U32,
            max_mesh_invocations_per_dimension: ALLOC_MAX_U32,
            max_task_payload_size: ALLOC_MAX_U32,
            max_mesh_output_vertices: ALLOC_MAX_U32,
            max_mesh_output_primitives: ALLOC_MAX_U32,
            max_mesh_output_layers: ALLOC_MAX_U32,
            max_mesh_multiview_view_count: ALLOC_MAX_U32,

            max_blas_primitive_count: ALLOC_MAX_U32,
            max_blas_geometry_count: ALLOC_MAX_U32,
            max_tlas_instance_count: ALLOC_MAX_U32,
            max_acceleration_structures_per_shader_stage: ALLOC_MAX_U32,

            max_multiview_view_count: ALLOC_MAX_U32,
        },
        alignments: crate::Alignments {
            // All maximally permissive
            buffer_copy_offset: wgt::BufferSize::MIN,
            buffer_copy_pitch: wgt::BufferSize::MIN,
            uniform_bounds_check_alignment: wgt::BufferSize::MIN,
            raw_tlas_instance_size: 0,
            ray_tracing_scratch_buffer_alignment: 1,
        },
        downlevel: wgt::DownlevelCapabilities {
            flags: wgt::DownlevelFlags::all(),
            limits: wgt::DownlevelLimits {},
            shader_model: wgt::ShaderModel::Sm5,
        },
        cooperative_matrix_properties: Vec::new(),
    }
};

impl crate::Surface for Context {
    type A = Api;

    unsafe fn configure(
        &self,
        device: &Context,
        config: &crate::SurfaceConfiguration,
    ) -> Result<(), crate::SurfaceError> {
        Ok(())
    }

    unsafe fn unconfigure(&self, device: &Context) {}

    unsafe fn acquire_texture(
        &self,
        _timeout: Option<Duration>,
        _fence: &Fence,
    ) -> Result<crate::AcquiredSurfaceTexture<Api>, crate::SurfaceError> {
        Err(crate::SurfaceError::Timeout)
    }
    unsafe fn discard_texture(&self, texture: Resource) {}
}

impl crate::Adapter for Context {
    type A = Api;

    unsafe fn open(
        &self,
        features: wgt::Features,
        _limits: &wgt::Limits,
        _memory_hints: &wgt::MemoryHints,
    ) -> DeviceResult<crate::OpenDevice<Api>> {
        Ok(crate::OpenDevice {
            device: Context,
            queue: Context,
        })
    }
    unsafe fn texture_format_capabilities(
        &self,
        format: wgt::TextureFormat,
    ) -> crate::TextureFormatCapabilities {
        crate::TextureFormatCapabilities::empty()
    }

    unsafe fn surface_capabilities(&self, surface: &Context) -> Option<crate::SurfaceCapabilities> {
        None
    }

    unsafe fn get_presentation_timestamp(&self) -> wgt::PresentationTimestamp {
        wgt::PresentationTimestamp::INVALID_TIMESTAMP
    }

    fn get_ordered_buffer_usages(&self) -> wgt::BufferUses {
        wgt::BufferUses::INCLUSIVE | wgt::BufferUses::MAP_WRITE
    }

    fn get_ordered_texture_usages(&self) -> wgt::TextureUses {
        wgt::TextureUses::INCLUSIVE
            | wgt::TextureUses::COLOR_TARGET
            | wgt::TextureUses::DEPTH_STENCIL_WRITE
    }
}

impl crate::Queue for Context {
    type A = Api;

    unsafe fn submit(
        &self,
        command_buffers: &[&CommandBuffer],
        surface_textures: &[&Resource],
        (fence, fence_value): (&mut Fence, crate::FenceValue),
    ) -> DeviceResult<()> {
        // All commands are executed synchronously.
        for cb in command_buffers {
            // SAFETY: Caller is responsible for ensuring synchronization between commands and
            // other mutations.
            unsafe {
                cb.execute();
            }
        }
        fence.value.store(fence_value, Ordering::Release);
        Ok(())
    }
    unsafe fn present(
        &self,
        surface: &Context,
        texture: Resource,
    ) -> Result<(), crate::SurfaceError> {
        Ok(())
    }

    unsafe fn get_timestamp_period(&self) -> f32 {
        1.0
    }
}

impl crate::Device for Context {
    type A = Api;

    unsafe fn create_buffer(&self, desc: &crate::BufferDescriptor) -> DeviceResult<Buffer> {
        Buffer::new(desc)
    }

    unsafe fn destroy_buffer(&self, buffer: Buffer) {}
    unsafe fn add_raw_buffer(&self, _buffer: &Buffer) {}

    unsafe fn map_buffer(
        &self,
        buffer: &Buffer,
        range: crate::MemoryRange,
    ) -> DeviceResult<crate::BufferMapping> {
        // Safety: the `wgpu-core` validation layer will prevent any user-accessible aliasing
        // mappings from being created, so we don’t need to perform any checks here, except for
        // bounds checks on the range which are built into `get_slice_ptr()`.
        Ok(crate::BufferMapping {
            ptr: ptr::NonNull::new(buffer.get_slice_ptr(range).cast::<u8>()).unwrap(),
            is_coherent: true,
        })
    }
    unsafe fn unmap_buffer(&self, buffer: &Buffer) {}
    unsafe fn flush_mapped_ranges<I>(&self, buffer: &Buffer, ranges: I) {}
    unsafe fn invalidate_mapped_ranges<I>(&self, buffer: &Buffer, ranges: I) {}

    unsafe fn create_texture(&self, desc: &crate::TextureDescriptor) -> DeviceResult<Resource> {
        Ok(Resource)
    }
    unsafe fn destroy_texture(&self, texture: Resource) {}
    unsafe fn add_raw_texture(&self, _texture: &Resource) {}

    unsafe fn create_texture_view(
        &self,
        texture: &Resource,
        desc: &crate::TextureViewDescriptor,
    ) -> DeviceResult<Resource> {
        Ok(Resource)
    }
    unsafe fn destroy_texture_view(&self, view: Resource) {}
    unsafe fn create_sampler(&self, desc: &crate::SamplerDescriptor) -> DeviceResult<Resource> {
        Ok(Resource)
    }
    unsafe fn destroy_sampler(&self, sampler: Resource) {}

    unsafe fn create_command_encoder(
        &self,
        desc: &crate::CommandEncoderDescriptor<Context>,
    ) -> DeviceResult<CommandBuffer> {
        Ok(CommandBuffer::new())
    }

    unsafe fn create_bind_group_layout(
        &self,
        desc: &crate::BindGroupLayoutDescriptor,
    ) -> DeviceResult<Resource> {
        Ok(Resource)
    }
    unsafe fn destroy_bind_group_layout(&self, bg_layout: Resource) {}
    unsafe fn create_pipeline_layout(
        &self,
        desc: &crate::PipelineLayoutDescriptor<Resource>,
    ) -> DeviceResult<Resource> {
        Ok(Resource)
    }
    unsafe fn destroy_pipeline_layout(&self, pipeline_layout: Resource) {}
    unsafe fn create_bind_group(
        &self,
        desc: &crate::BindGroupDescriptor<Resource, Buffer, Resource, Resource, Resource>,
    ) -> DeviceResult<Resource> {
        Ok(Resource)
    }
    unsafe fn destroy_bind_group(&self, group: Resource) {}

    unsafe fn create_shader_module(
        &self,
        desc: &crate::ShaderModuleDescriptor,
        shader: crate::ShaderInput,
    ) -> Result<Resource, crate::ShaderError> {
        Ok(Resource)
    }
    unsafe fn destroy_shader_module(&self, module: Resource) {}
    unsafe fn create_render_pipeline(
        &self,
        desc: &crate::RenderPipelineDescriptor<Resource, Resource, Resource>,
    ) -> Result<Resource, crate::PipelineError> {
        Ok(Resource)
    }
    unsafe fn destroy_render_pipeline(&self, pipeline: Resource) {}
    unsafe fn create_compute_pipeline(
        &self,
        desc: &crate::ComputePipelineDescriptor<Resource, Resource, Resource>,
    ) -> Result<Resource, crate::PipelineError> {
        Ok(Resource)
    }
    unsafe fn destroy_compute_pipeline(&self, pipeline: Resource) {}
    unsafe fn create_pipeline_cache(
        &self,
        desc: &crate::PipelineCacheDescriptor<'_>,
    ) -> Result<Resource, crate::PipelineCacheError> {
        Ok(Resource)
    }
    unsafe fn destroy_pipeline_cache(&self, cache: Resource) {}

    unsafe fn create_query_set(
        &self,
        desc: &wgt::QuerySetDescriptor<crate::Label>,
    ) -> DeviceResult<Resource> {
        Ok(Resource)
    }
    unsafe fn destroy_query_set(&self, set: Resource) {}
    unsafe fn create_fence(&self) -> DeviceResult<Fence> {
        Ok(Fence {
            value: AtomicU64::new(0),
        })
    }
    unsafe fn destroy_fence(&self, fence: Fence) {}
    unsafe fn get_fence_value(&self, fence: &Fence) -> DeviceResult<crate::FenceValue> {
        Ok(fence.value.load(Ordering::Acquire))
    }
    unsafe fn wait(
        &self,
        fence: &Fence,
        value: crate::FenceValue,
        timeout: Option<Duration>,
    ) -> DeviceResult<bool> {
        // The relevant commands must have already been submitted, and noop-backend commands are
        // executed synchronously, so there is no waiting — either it is already done,
        // or this method was called incorrectly.
        assert!(
            fence.value.load(Ordering::Acquire) >= value,
            "submission must have already been done"
        );
        Ok(true)
    }

    unsafe fn start_graphics_debugger_capture(&self) -> bool {
        false
    }
    unsafe fn stop_graphics_debugger_capture(&self) {}
    unsafe fn create_acceleration_structure(
        &self,
        desc: &crate::AccelerationStructureDescriptor,
    ) -> DeviceResult<Resource> {
        Ok(Resource)
    }
    unsafe fn get_acceleration_structure_build_sizes<'a>(
        &self,
        _desc: &crate::GetAccelerationStructureBuildSizesDescriptor<'a, Buffer>,
    ) -> crate::AccelerationStructureBuildSizes {
        Default::default()
    }
    unsafe fn get_acceleration_structure_device_address(
        &self,
        _acceleration_structure: &Resource,
    ) -> wgt::BufferAddress {
        Default::default()
    }
    unsafe fn destroy_acceleration_structure(&self, _acceleration_structure: Resource) {}

    fn tlas_instance_to_bytes(&self, instance: TlasInstance) -> Vec<u8> {
        vec![]
    }

    fn get_internal_counters(&self) -> wgt::HalCounters {
        Default::default()
    }

    fn check_if_oom(&self) -> DeviceResult<()> {
        Ok(())
    }
}
