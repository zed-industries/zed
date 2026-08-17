//! [`Limits`] and downlevel-related types.

use core::cmp::Ordering;

#[cfg(any(feature = "serde", test))]
use serde::{Deserialize, Serialize};

#[cfg(doc)]
use crate::{Features, TextureFormat};

/// Invoke a macro for each of the limits.
///
/// The supplied macro should take two arguments. The first is a limit name, as
/// an identifier, typically used to access a member of `struct Limits`. The
/// second is `Ordering::Less` if valid values are less than the limit (the
/// common case), or `Ordering::Greater` if valid values are more than the limit
/// (for limits like alignments, which are minima instead of maxima).
macro_rules! with_limits {
    ($macro_name:ident) => {
        $macro_name!(max_texture_dimension_1d, Ordering::Less);
        $macro_name!(max_texture_dimension_1d, Ordering::Less);
        $macro_name!(max_texture_dimension_2d, Ordering::Less);
        $macro_name!(max_texture_dimension_3d, Ordering::Less);
        $macro_name!(max_texture_array_layers, Ordering::Less);
        $macro_name!(max_bind_groups, Ordering::Less);
        $macro_name!(max_bindings_per_bind_group, Ordering::Less);
        $macro_name!(
            max_dynamic_uniform_buffers_per_pipeline_layout,
            Ordering::Less
        );
        $macro_name!(
            max_dynamic_storage_buffers_per_pipeline_layout,
            Ordering::Less
        );
        $macro_name!(max_sampled_textures_per_shader_stage, Ordering::Less);
        $macro_name!(max_samplers_per_shader_stage, Ordering::Less);
        $macro_name!(max_storage_buffers_per_shader_stage, Ordering::Less);
        $macro_name!(max_storage_textures_per_shader_stage, Ordering::Less);
        $macro_name!(max_uniform_buffers_per_shader_stage, Ordering::Less);
        $macro_name!(max_binding_array_elements_per_shader_stage, Ordering::Less);
        $macro_name!(
            max_binding_array_acceleration_structure_elements_per_shader_stage,
            Ordering::Less
        );
        $macro_name!(max_uniform_buffer_binding_size, Ordering::Less);
        $macro_name!(max_storage_buffer_binding_size, Ordering::Less);
        $macro_name!(max_vertex_buffers, Ordering::Less);
        $macro_name!(max_buffer_size, Ordering::Less);
        $macro_name!(max_vertex_attributes, Ordering::Less);
        $macro_name!(max_vertex_buffer_array_stride, Ordering::Less);
        $macro_name!(max_inter_stage_shader_variables, Ordering::Less);
        $macro_name!(min_uniform_buffer_offset_alignment, Ordering::Greater);
        $macro_name!(min_storage_buffer_offset_alignment, Ordering::Greater);
        $macro_name!(max_color_attachments, Ordering::Less);
        $macro_name!(max_color_attachment_bytes_per_sample, Ordering::Less);
        $macro_name!(max_compute_workgroup_storage_size, Ordering::Less);
        $macro_name!(max_compute_invocations_per_workgroup, Ordering::Less);
        $macro_name!(max_compute_workgroup_size_x, Ordering::Less);
        $macro_name!(max_compute_workgroup_size_y, Ordering::Less);
        $macro_name!(max_compute_workgroup_size_z, Ordering::Less);
        $macro_name!(max_compute_workgroups_per_dimension, Ordering::Less);

        $macro_name!(max_immediate_size, Ordering::Less);
        $macro_name!(max_non_sampler_bindings, Ordering::Less);

        $macro_name!(max_task_mesh_workgroup_total_count, Ordering::Less);
        $macro_name!(max_task_mesh_workgroups_per_dimension, Ordering::Less);
        $macro_name!(max_task_invocations_per_workgroup, Ordering::Less);
        $macro_name!(max_task_invocations_per_dimension, Ordering::Less);
        $macro_name!(max_mesh_invocations_per_workgroup, Ordering::Less);
        $macro_name!(max_mesh_invocations_per_dimension, Ordering::Less);

        $macro_name!(max_task_payload_size, Ordering::Less);
        $macro_name!(max_mesh_output_vertices, Ordering::Less);
        $macro_name!(max_mesh_output_primitives, Ordering::Less);
        $macro_name!(max_mesh_output_layers, Ordering::Less);
        $macro_name!(max_mesh_multiview_view_count, Ordering::Less);

        $macro_name!(max_blas_primitive_count, Ordering::Less);
        $macro_name!(max_blas_geometry_count, Ordering::Less);
        $macro_name!(max_tlas_instance_count, Ordering::Less);

        $macro_name!(max_multiview_view_count, Ordering::Less);
    };
}

/// Represents the sets of limits an adapter/device supports.
///
/// We provide three different defaults.
/// - [`Limits::downlevel_defaults()`]. This is a set of limits that is guaranteed to work on almost
///   all backends, including "downlevel" backends such as OpenGL and D3D11, other than WebGL. For
///   most applications we recommend using these limits, assuming they are high enough for your
///   application, and you do not intend to support WebGL.
/// - [`Limits::downlevel_webgl2_defaults()`] This is a set of limits that is lower even than the
///   [`downlevel_defaults()`], configured to be low enough to support running in the browser using
///   WebGL2.
/// - [`Limits::default()`]. This is the set of limits that is guaranteed to work on all modern
///   backends and is guaranteed to be supported by WebGPU. Applications needing more modern
///   features can use this as a reasonable set of limits if they are targeting only desktop and
///   modern mobile devices.
///
/// We recommend starting with the most restrictive limits you can and manually increasing the
/// limits you need boosted. This will let you stay running on all hardware that supports the limits
/// you need.
///
/// Limits "better" than the default must be supported by the adapter and requested when requesting
/// a device. If limits "better" than the adapter supports are requested, requesting a device will
/// panic. Once a device is requested, you may only use resources up to the limits requested _even_
/// if the adapter supports "better" limits.
///
/// Requesting limits that are "better" than you need may cause performance to decrease because the
/// implementation needs to support more than is needed. You should ideally only request exactly
/// what you need.
///
/// Corresponds to [WebGPU `GPUSupportedLimits`](
/// https://gpuweb.github.io/gpuweb/#gpusupportedlimits).
///
/// [`downlevel_defaults()`]: Limits::downlevel_defaults
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase", default))]
pub struct Limits {
    /// Maximum allowed value for the `size.width` of a texture created with `TextureDimension::D1`.
    /// Defaults to 8192. Higher is "better".
    #[cfg_attr(feature = "serde", serde(rename = "maxTextureDimension1D"))]
    pub max_texture_dimension_1d: u32,
    /// Maximum allowed value for the `size.width` and `size.height` of a texture created with `TextureDimension::D2`.
    /// Defaults to 8192. Higher is "better".
    #[cfg_attr(feature = "serde", serde(rename = "maxTextureDimension2D"))]
    pub max_texture_dimension_2d: u32,
    /// Maximum allowed value for the `size.width`, `size.height`, and `size.depth_or_array_layers`
    /// of a texture created with `TextureDimension::D3`.
    /// Defaults to 2048. Higher is "better".
    #[cfg_attr(feature = "serde", serde(rename = "maxTextureDimension3D"))]
    pub max_texture_dimension_3d: u32,
    /// Maximum allowed value for the `size.depth_or_array_layers` of a texture created with `TextureDimension::D2`.
    /// Defaults to 256. Higher is "better".
    pub max_texture_array_layers: u32,
    /// Amount of bind groups that can be attached to a pipeline at the same time. Defaults to 4. Higher is "better".
    pub max_bind_groups: u32,
    /// Maximum binding index allowed in `create_bind_group_layout`. Defaults to 1000. Higher is "better".
    pub max_bindings_per_bind_group: u32,
    /// Amount of uniform buffer bindings that can be dynamic in a single pipeline. Defaults to 8. Higher is "better".
    pub max_dynamic_uniform_buffers_per_pipeline_layout: u32,
    /// Amount of storage buffer bindings that can be dynamic in a single pipeline. Defaults to 4. Higher is "better".
    pub max_dynamic_storage_buffers_per_pipeline_layout: u32,
    /// Amount of sampled textures visible in a single shader stage. Defaults to 16. Higher is "better".
    pub max_sampled_textures_per_shader_stage: u32,
    /// Amount of samplers visible in a single shader stage. Defaults to 16. Higher is "better".
    pub max_samplers_per_shader_stage: u32,
    /// Amount of storage buffers visible in a single shader stage. Defaults to 8. Higher is "better".
    pub max_storage_buffers_per_shader_stage: u32,
    /// Amount of storage textures visible in a single shader stage. Defaults to 4. Higher is "better".
    pub max_storage_textures_per_shader_stage: u32,
    /// Amount of uniform buffers visible in a single shader stage. Defaults to 12. Higher is "better".
    pub max_uniform_buffers_per_shader_stage: u32,
    /// Amount of individual resources within binding arrays that can be accessed in a single shader stage. Applies
    /// to all types of bindings except samplers.
    ///
    /// This "defaults" to 0. However if binding arrays are supported, all devices can support 500,000. Higher is "better".
    pub max_binding_array_elements_per_shader_stage: u32,
    /// Amount of individual acceleration structures within binding arrays that can be accessed in a single shader stage.
    ///
    /// This "defaults" to 0. Higher is "better".
    pub max_binding_array_acceleration_structure_elements_per_shader_stage: u32,
    /// Amount of individual samplers within binding arrays that can be accessed in a single shader stage.
    ///
    /// This "defaults" to 0. However if binding arrays are supported, all devices can support 1,000. Higher is "better".
    pub max_binding_array_sampler_elements_per_shader_stage: u32,
    /// Maximum size in bytes of a binding to a uniform buffer. Defaults to 64 KiB. Higher is "better".
    pub max_uniform_buffer_binding_size: u64,
    /// Maximum size in bytes of a binding to a storage buffer. Defaults to 128 MiB. Higher is "better".
    pub max_storage_buffer_binding_size: u64,
    /// Maximum length of `VertexState::buffers` when creating a `RenderPipeline`.
    /// Defaults to 8. Higher is "better".
    pub max_vertex_buffers: u32,
    /// A limit above which buffer allocations are guaranteed to fail.
    /// Defaults to 256 MiB. Higher is "better".
    ///
    /// Buffer allocations below the maximum buffer size may not succeed depending on available memory,
    /// fragmentation and other factors.
    pub max_buffer_size: u64,
    /// Maximum length of `VertexBufferLayout::attributes`, summed over all `VertexState::buffers`,
    /// when creating a `RenderPipeline`.
    /// Defaults to 16. Higher is "better".
    pub max_vertex_attributes: u32,
    /// Maximum value for `VertexBufferLayout::array_stride` when creating a `RenderPipeline`.
    /// Defaults to 2048. Higher is "better".
    pub max_vertex_buffer_array_stride: u32,
    /// Maximum value for the number of input or output variables for inter-stage communication
    /// (like vertex outputs or fragment inputs) `@location(…)`s (in WGSL parlance)
    /// when creating a `RenderPipeline`.
    /// Defaults to 16. Higher is "better".
    pub max_inter_stage_shader_variables: u32,
    /// Required `BufferBindingType::Uniform` alignment for `BufferBinding::offset`
    /// when creating a `BindGroup`, or for `set_bind_group` `dynamicOffsets`.
    /// Defaults to 256. Lower is "better".
    pub min_uniform_buffer_offset_alignment: u32,
    /// Required `BufferBindingType::Storage` alignment for `BufferBinding::offset`
    /// when creating a `BindGroup`, or for `set_bind_group` `dynamicOffsets`.
    /// Defaults to 256. Lower is "better".
    pub min_storage_buffer_offset_alignment: u32,
    /// The maximum allowed number of color attachments.
    pub max_color_attachments: u32,
    /// The maximum number of bytes necessary to hold one sample (pixel or subpixel) of render
    /// pipeline output data, across all color attachments as described by [`TextureFormat::target_pixel_byte_cost`]
    /// and [`TextureFormat::target_component_alignment`]. Defaults to 32. Higher is "better".
    ///
    /// ⚠️ `Rgba8Unorm`/`Rgba8Snorm`/`Bgra8Unorm`/`Bgra8Snorm` are deceptively 8 bytes per sample. ⚠️
    pub max_color_attachment_bytes_per_sample: u32,
    /// Maximum number of bytes used for workgroup memory in a compute entry point. Defaults to
    /// 16384. Higher is "better".
    pub max_compute_workgroup_storage_size: u32,
    /// Maximum value of the product of the `workgroup_size` dimensions for a compute entry-point.
    /// Defaults to 256. Higher is "better".
    pub max_compute_invocations_per_workgroup: u32,
    /// The maximum value of the `workgroup_size` X dimension for a compute stage `ShaderModule` entry-point.
    /// Defaults to 256. Higher is "better".
    pub max_compute_workgroup_size_x: u32,
    /// The maximum value of the `workgroup_size` Y dimension for a compute stage `ShaderModule` entry-point.
    /// Defaults to 256. Higher is "better".
    pub max_compute_workgroup_size_y: u32,
    /// The maximum value of the `workgroup_size` Z dimension for a compute stage `ShaderModule` entry-point.
    /// Defaults to 64. Higher is "better".
    pub max_compute_workgroup_size_z: u32,
    /// The maximum value for each dimension of a `ComputePass::dispatch(x, y, z)` operation.
    /// Defaults to 65535. Higher is "better".
    pub max_compute_workgroups_per_dimension: u32,

    /// Amount of storage available for immediates in bytes. Defaults to 0. Higher is "better".
    /// Requesting more than 0 during device creation requires [`Features::IMMEDIATES`] to be enabled.
    ///
    /// Expect the size to be:
    /// - Vulkan: 128-256 bytes
    /// - DX12: 128 bytes
    /// - Metal: 4096 bytes
    /// - OpenGL doesn't natively support immediates, and are emulated with uniforms,
    ///   so this number is less useful but likely 256.
    pub max_immediate_size: u32,
    /// Maximum number of live non-sampler bindings.
    ///
    /// <div class="warning">
    /// The default value is **1_000_000**, On systems with integrated GPUs (iGPUs)—particularly on Windows using the D3D12
    /// backend—this can lead to significant system RAM consumption since iGPUs share system memory directly with the CPU.
    /// </div>
    ///
    /// This limit only affects the d3d12 backend. Using a large number will allow the device
    /// to create many bind groups at the cost of a large up-front allocation at device creation.
    pub max_non_sampler_bindings: u32,

    /// The maximum total value for a `RenderPass::draw_mesh_tasks(x, y, z)` operation or the
    /// `@builtin(mesh_task_size)` returned from a task shader.  Higher is "better".
    pub max_task_mesh_workgroup_total_count: u32,
    /// The maximum value for each dimension of a `RenderPass::draw_mesh_tasks(x, y, z)` operation.
    /// Also for task shader outputs. Higher is "better".
    pub max_task_mesh_workgroups_per_dimension: u32,
    // These are fundamentally different. It is very common for limits on mesh shaders to be much lower.
    /// Maximum total number of invocations, or threads, per task shader workgroup. Higher is "better".
    pub max_task_invocations_per_workgroup: u32,
    /// The maximum value for each dimension of a task shader's workgroup size. Higher is "better".
    pub max_task_invocations_per_dimension: u32,
    /// Maximum total number of invocations, or threads, per mesh shader workgroup. Higher is "better".
    pub max_mesh_invocations_per_workgroup: u32,
    /// The maximum value for each dimension of a mesh shader's workgroup size. Higher is "better".
    pub max_mesh_invocations_per_dimension: u32,

    /// The maximum size of the payload passed from task to mesh shader. Higher is "better".
    pub max_task_payload_size: u32,
    /// The maximum number of vertices that a mesh shader may output. Higher is "better".
    pub max_mesh_output_vertices: u32,
    /// The maximum number of primitives that a mesh shader may output. Higher is "better".
    pub max_mesh_output_primitives: u32,
    /// The maximum number of layers that can be output from a mesh shader. Higher is "better".
    /// See [#8509](https://github.com/gfx-rs/wgpu/issues/8509).
    pub max_mesh_output_layers: u32,
    /// The maximum number of views that can be used by a mesh shader in multiview rendering.
    /// Higher is "better".
    pub max_mesh_multiview_view_count: u32,

    /// The maximum number of primitive (ex: triangles, aabbs) a BLAS is allowed to have. Requesting
    /// more than 0 during device creation only makes sense if [`Features::EXPERIMENTAL_RAY_QUERY`]
    /// is enabled.
    pub max_blas_primitive_count: u32,
    /// The maximum number of geometry descriptors a BLAS is allowed to have. Requesting
    /// more than 0 during device creation only makes sense if [`Features::EXPERIMENTAL_RAY_QUERY`]
    /// is enabled.
    pub max_blas_geometry_count: u32,
    /// The maximum number of instances a TLAS is allowed to have. Requesting more than 0 during
    /// device creation only makes sense if [`Features::EXPERIMENTAL_RAY_QUERY`]
    /// is enabled.
    pub max_tlas_instance_count: u32,
    /// The maximum number of acceleration structures allowed to be used in a shader stage.
    /// Requesting more than 0 during device creation only makes sense if [`Features::EXPERIMENTAL_RAY_QUERY`]
    /// is enabled.
    pub max_acceleration_structures_per_shader_stage: u32,

    /// The maximum number of views that can be used in multiview rendering
    pub max_multiview_view_count: u32,
}

impl Default for Limits {
    fn default() -> Self {
        Self::defaults()
    }
}

impl Limits {
    /// These default limits are guaranteed to to work on all modern
    /// backends and guaranteed to be supported by WebGPU
    ///
    /// Those limits are as follows:
    /// ```rust
    /// # use wgpu_types::Limits;
    /// assert_eq!(Limits::defaults(), Limits {
    ///     max_texture_dimension_1d: 8192,
    ///     max_texture_dimension_2d: 8192,
    ///     max_texture_dimension_3d: 2048,
    ///     max_texture_array_layers: 256,
    ///     max_bind_groups: 4,
    ///     max_bindings_per_bind_group: 1000,
    ///     max_dynamic_uniform_buffers_per_pipeline_layout: 8,
    ///     max_dynamic_storage_buffers_per_pipeline_layout: 4,
    ///     max_sampled_textures_per_shader_stage: 16,
    ///     max_samplers_per_shader_stage: 16,
    ///     max_storage_buffers_per_shader_stage: 8,
    ///     max_storage_textures_per_shader_stage: 4,
    ///     max_uniform_buffers_per_shader_stage: 12,
    ///     max_binding_array_elements_per_shader_stage: 0,
    ///     max_binding_array_acceleration_structure_elements_per_shader_stage: 0,
    ///     max_binding_array_sampler_elements_per_shader_stage: 0,
    ///     max_uniform_buffer_binding_size: 64 << 10, // (64 KiB)
    ///     max_storage_buffer_binding_size: 128 << 20, // (128 MiB)
    ///     max_vertex_buffers: 8,
    ///     max_buffer_size: 256 << 20, // (256 MiB)
    ///     max_vertex_attributes: 16,
    ///     max_vertex_buffer_array_stride: 2048,
    ///     max_inter_stage_shader_variables: 16,
    ///     min_uniform_buffer_offset_alignment: 256,
    ///     min_storage_buffer_offset_alignment: 256,
    ///     max_color_attachments: 8,
    ///     max_color_attachment_bytes_per_sample: 32,
    ///     max_compute_workgroup_storage_size: 16384,
    ///     max_compute_invocations_per_workgroup: 256,
    ///     max_compute_workgroup_size_x: 256,
    ///     max_compute_workgroup_size_y: 256,
    ///     max_compute_workgroup_size_z: 64,
    ///     max_compute_workgroups_per_dimension: 65535,
    ///     max_immediate_size: 0,
    ///     max_non_sampler_bindings: 1_000_000,
    ///     max_task_mesh_workgroup_total_count: 0,
    ///     max_task_mesh_workgroups_per_dimension: 0,
    ///     max_task_invocations_per_workgroup: 0,
    ///     max_task_invocations_per_dimension: 0,
    ///     max_mesh_invocations_per_workgroup: 0,
    ///     max_mesh_invocations_per_dimension: 0,
    ///     max_task_payload_size: 0,
    ///     max_mesh_output_vertices: 0,
    ///     max_mesh_output_primitives: 0,
    ///     max_mesh_output_layers: 0,
    ///     max_mesh_multiview_view_count: 0,
    ///     max_blas_primitive_count: 0,
    ///     max_blas_geometry_count: 0,
    ///     max_tlas_instance_count: 0,
    ///     max_acceleration_structures_per_shader_stage: 0,
    ///     max_multiview_view_count: 0,
    /// });
    /// ```
    ///
    /// Rust doesn't allow const in trait implementations, so we break this out
    /// to allow reusing these defaults in const contexts
    #[must_use]
    pub const fn defaults() -> Self {
        Self {
            max_texture_dimension_1d: 8192,
            max_texture_dimension_2d: 8192,
            max_texture_dimension_3d: 2048,
            max_texture_array_layers: 256,
            max_bind_groups: 4,
            max_bindings_per_bind_group: 1000,
            max_dynamic_uniform_buffers_per_pipeline_layout: 8,
            max_dynamic_storage_buffers_per_pipeline_layout: 4,
            max_sampled_textures_per_shader_stage: 16,
            max_samplers_per_shader_stage: 16,
            max_storage_buffers_per_shader_stage: 8,
            max_storage_textures_per_shader_stage: 4,
            max_uniform_buffers_per_shader_stage: 12,
            max_binding_array_elements_per_shader_stage: 0,
            max_binding_array_acceleration_structure_elements_per_shader_stage: 0,
            max_binding_array_sampler_elements_per_shader_stage: 0,
            max_uniform_buffer_binding_size: 64 << 10, // (64 KiB)
            max_storage_buffer_binding_size: 128 << 20, // (128 MiB)
            max_vertex_buffers: 8,
            max_buffer_size: 256 << 20, // (256 MiB)
            max_vertex_attributes: 16,
            max_vertex_buffer_array_stride: 2048,
            max_inter_stage_shader_variables: 16,
            min_uniform_buffer_offset_alignment: 256,
            min_storage_buffer_offset_alignment: 256,
            max_color_attachments: 8,
            max_color_attachment_bytes_per_sample: 32,
            max_compute_workgroup_storage_size: 16384,
            max_compute_invocations_per_workgroup: 256,
            max_compute_workgroup_size_x: 256,
            max_compute_workgroup_size_y: 256,
            max_compute_workgroup_size_z: 64,
            max_compute_workgroups_per_dimension: 65535,
            max_immediate_size: 0,
            max_non_sampler_bindings: 1_000_000,

            max_task_mesh_workgroup_total_count: 0,
            max_task_mesh_workgroups_per_dimension: 0,
            max_task_invocations_per_workgroup: 0,
            max_task_invocations_per_dimension: 0,
            max_mesh_invocations_per_workgroup: 0,
            max_mesh_invocations_per_dimension: 0,
            max_task_payload_size: 0,
            max_mesh_output_vertices: 0,
            max_mesh_output_primitives: 0,
            max_mesh_output_layers: 0,
            max_mesh_multiview_view_count: 0,

            max_blas_primitive_count: 0,
            max_blas_geometry_count: 0,
            max_tlas_instance_count: 0,
            max_acceleration_structures_per_shader_stage: 0,

            max_multiview_view_count: 0,
        }
    }

    /// These default limits are guaranteed to be compatible with GLES-3.1, and D3D11
    ///
    /// Those limits are as follows (different from default are marked with *):
    /// ```rust
    /// # use wgpu_types::Limits;
    /// assert_eq!(Limits::downlevel_defaults(), Limits {
    ///     max_texture_dimension_1d: 2048, // *
    ///     max_texture_dimension_2d: 2048, // *
    ///     max_texture_dimension_3d: 256, // *
    ///     max_texture_array_layers: 256,
    ///     max_bind_groups: 4,
    ///     max_bindings_per_bind_group: 1000,
    ///     max_dynamic_uniform_buffers_per_pipeline_layout: 8,
    ///     max_dynamic_storage_buffers_per_pipeline_layout: 4,
    ///     max_sampled_textures_per_shader_stage: 16,
    ///     max_samplers_per_shader_stage: 16,
    ///     max_storage_buffers_per_shader_stage: 4, // *
    ///     max_storage_textures_per_shader_stage: 4,
    ///     max_uniform_buffers_per_shader_stage: 12,
    ///     max_binding_array_elements_per_shader_stage: 0,
    ///     max_binding_array_acceleration_structure_elements_per_shader_stage: 0,
    ///     max_binding_array_sampler_elements_per_shader_stage: 0,
    ///     max_uniform_buffer_binding_size: 16 << 10, // * (16 KiB)
    ///     max_storage_buffer_binding_size: 128 << 20, // (128 MiB)
    ///     max_vertex_buffers: 8,
    ///     max_vertex_attributes: 16,
    ///     max_vertex_buffer_array_stride: 2048,
    ///     max_immediate_size: 0,
    ///     min_uniform_buffer_offset_alignment: 256,
    ///     min_storage_buffer_offset_alignment: 256,
    ///     max_inter_stage_shader_variables: 15,
    ///     max_color_attachments: 4,
    ///     max_color_attachment_bytes_per_sample: 32,
    ///     max_compute_workgroup_storage_size: 16352, // *
    ///     max_compute_invocations_per_workgroup: 256,
    ///     max_compute_workgroup_size_x: 256,
    ///     max_compute_workgroup_size_y: 256,
    ///     max_compute_workgroup_size_z: 64,
    ///     max_compute_workgroups_per_dimension: 65535,
    ///     max_buffer_size: 256 << 20, // (256 MiB)
    ///     max_non_sampler_bindings: 1_000_000,
    ///
    ///     max_task_mesh_workgroup_total_count: 0,
    ///     max_task_mesh_workgroups_per_dimension: 0,
    ///     max_task_invocations_per_workgroup: 0,
    ///     max_task_invocations_per_dimension: 0,
    ///     max_mesh_invocations_per_workgroup: 0,
    ///     max_mesh_invocations_per_dimension: 0,
    ///     max_task_payload_size: 0,
    ///     max_mesh_output_vertices: 0,
    ///     max_mesh_output_primitives: 0,
    ///     max_mesh_output_layers: 0,
    ///     max_mesh_multiview_view_count: 0,
    ///
    ///     max_blas_primitive_count: 0,
    ///     max_blas_geometry_count: 0,
    ///     max_tlas_instance_count: 0,
    ///     max_acceleration_structures_per_shader_stage: 0,
    ///
    ///     max_multiview_view_count: 0,
    /// });
    /// ```
    #[must_use]
    pub const fn downlevel_defaults() -> Self {
        Self {
            max_texture_dimension_1d: 2048,
            max_texture_dimension_2d: 2048,
            max_texture_dimension_3d: 256,
            max_storage_buffers_per_shader_stage: 4,
            max_uniform_buffer_binding_size: 16 << 10, // (16 KiB)
            max_inter_stage_shader_variables: 15,
            max_color_attachments: 4,
            // see: https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf#page=7
            max_compute_workgroup_storage_size: 16352,
            ..Self::defaults()
        }
    }

    /// These default limits are guaranteed to be compatible with GLES-3.0, and D3D11, and WebGL2
    ///
    /// Those limits are as follows (different from `downlevel_defaults` are marked with +,
    /// *'s from `downlevel_defaults` shown as well.):
    /// ```rust
    /// # use wgpu_types::Limits;
    /// assert_eq!(Limits::downlevel_webgl2_defaults(), Limits {
    ///     max_texture_dimension_1d: 2048, // *
    ///     max_texture_dimension_2d: 2048, // *
    ///     max_texture_dimension_3d: 256, // *
    ///     max_texture_array_layers: 256,
    ///     max_bind_groups: 4,
    ///     max_bindings_per_bind_group: 1000,
    ///     max_dynamic_uniform_buffers_per_pipeline_layout: 8,
    ///     max_dynamic_storage_buffers_per_pipeline_layout: 0, // +
    ///     max_sampled_textures_per_shader_stage: 16,
    ///     max_samplers_per_shader_stage: 16,
    ///     max_storage_buffers_per_shader_stage: 0, // * +
    ///     max_storage_textures_per_shader_stage: 0, // +
    ///     max_uniform_buffers_per_shader_stage: 11, // +
    ///     max_binding_array_elements_per_shader_stage: 0,
    ///     max_binding_array_acceleration_structure_elements_per_shader_stage: 0,
    ///     max_binding_array_sampler_elements_per_shader_stage: 0,
    ///     max_uniform_buffer_binding_size: 16 << 10, // * (16 KiB)
    ///     max_storage_buffer_binding_size: 0, // * +
    ///     max_vertex_buffers: 8,
    ///     max_vertex_attributes: 16,
    ///     max_vertex_buffer_array_stride: 255, // +
    ///     max_immediate_size: 0,
    ///     min_uniform_buffer_offset_alignment: 256,
    ///     min_storage_buffer_offset_alignment: 256,
    ///     max_inter_stage_shader_variables: 15,
    ///     max_color_attachments: 4,
    ///     max_color_attachment_bytes_per_sample: 32,
    ///     max_compute_workgroup_storage_size: 0, // +
    ///     max_compute_invocations_per_workgroup: 0, // +
    ///     max_compute_workgroup_size_x: 0, // +
    ///     max_compute_workgroup_size_y: 0, // +
    ///     max_compute_workgroup_size_z: 0, // +
    ///     max_compute_workgroups_per_dimension: 0, // +
    ///     max_buffer_size: 256 << 20, // (256 MiB),
    ///     max_non_sampler_bindings: 1_000_000,
    ///
    ///     max_task_mesh_workgroup_total_count: 0,
    ///     max_task_mesh_workgroups_per_dimension: 0,
    ///     max_task_invocations_per_workgroup: 0,
    ///     max_task_invocations_per_dimension: 0,
    ///     max_mesh_invocations_per_workgroup: 0,
    ///     max_mesh_invocations_per_dimension: 0,
    ///     max_task_payload_size: 0,
    ///     max_mesh_output_vertices: 0,
    ///     max_mesh_output_primitives: 0,
    ///     max_mesh_output_layers: 0,
    ///     max_mesh_multiview_view_count: 0,
    ///
    ///     max_blas_primitive_count: 0,
    ///     max_blas_geometry_count: 0,
    ///     max_tlas_instance_count: 0,
    ///     max_acceleration_structures_per_shader_stage: 0,
    ///
    ///     max_multiview_view_count: 0,
    /// });
    /// ```
    #[must_use]
    pub const fn downlevel_webgl2_defaults() -> Self {
        Self {
            max_uniform_buffers_per_shader_stage: 11,
            max_storage_buffers_per_shader_stage: 0,
            max_storage_textures_per_shader_stage: 0,
            max_dynamic_storage_buffers_per_pipeline_layout: 0,
            max_storage_buffer_binding_size: 0,
            max_vertex_buffer_array_stride: 255,
            max_compute_workgroup_storage_size: 0,
            max_compute_invocations_per_workgroup: 0,
            max_compute_workgroup_size_x: 0,
            max_compute_workgroup_size_y: 0,
            max_compute_workgroup_size_z: 0,
            max_compute_workgroups_per_dimension: 0,

            // Value supported by Intel Celeron B830 on Windows (OpenGL 3.1)
            max_inter_stage_shader_variables: 15,

            // Most of the values should be the same as the downlevel defaults
            ..Self::downlevel_defaults()
        }
    }

    /// Modify the current limits to use the resolution limits of the other.
    ///
    /// This is useful because the swapchain might need to be larger than any other image in the application.
    ///
    /// If your application only needs 512x512, you might be running on a 4k display and need extremely high resolution limits.
    #[must_use]
    pub const fn using_resolution(self, other: Self) -> Self {
        Self {
            max_texture_dimension_1d: other.max_texture_dimension_1d,
            max_texture_dimension_2d: other.max_texture_dimension_2d,
            max_texture_dimension_3d: other.max_texture_dimension_3d,
            ..self
        }
    }

    /// Modify the current limits to use the buffer alignment limits of the adapter.
    ///
    /// This is useful for when you'd like to dynamically use the "best" supported buffer alignments.
    #[must_use]
    pub const fn using_alignment(self, other: Self) -> Self {
        Self {
            min_uniform_buffer_offset_alignment: other.min_uniform_buffer_offset_alignment,
            min_storage_buffer_offset_alignment: other.min_storage_buffer_offset_alignment,
            ..self
        }
    }

    /// The minimum guaranteed limits for acceleration structures if you enable [`Features::EXPERIMENTAL_RAY_QUERY`]
    #[must_use]
    pub const fn using_minimum_supported_acceleration_structure_values(self) -> Self {
        Self {
            max_blas_geometry_count: (1 << 24) - 1, // 2^24 - 1: Vulkan's minimum
            max_tlas_instance_count: (1 << 24) - 1, // 2^24 - 1: Vulkan's minimum
            max_blas_primitive_count: 1 << 28,      // 2^28: Metal's minimum
            max_acceleration_structures_per_shader_stage: 16, // Vulkan's minimum
            ..self
        }
    }

    /// Modify the current limits to use the acceleration structure limits of `other` (`other` could
    /// be the limits of the adapter).
    #[must_use]
    pub const fn using_acceleration_structure_values(self, other: Self) -> Self {
        Self {
            max_blas_geometry_count: other.max_blas_geometry_count,
            max_tlas_instance_count: other.max_tlas_instance_count,
            max_blas_primitive_count: other.max_blas_primitive_count,
            max_acceleration_structures_per_shader_stage: other
                .max_acceleration_structures_per_shader_stage,
            ..self
        }
    }

    /// The recommended minimum limits for mesh shaders if you enable [`Features::EXPERIMENTAL_MESH_SHADER`]
    ///
    /// These are chosen somewhat arbitrarily. They are small enough that they should cover all physical devices,
    /// but not necessarily all use cases.
    #[must_use]
    pub const fn using_recommended_minimum_mesh_shader_values(self) -> Self {
        Self {
            // This limitation comes from metal
            max_task_mesh_workgroup_total_count: 1024,
            // This is a DirectX limitation
            max_task_mesh_workgroups_per_dimension: 256,
            // Nvidia limit on vulkan
            max_task_invocations_per_workgroup: 128,
            max_task_invocations_per_dimension: 64,

            // DX12 limitation, revisit for vulkan
            max_mesh_invocations_per_workgroup: 128,
            max_mesh_invocations_per_dimension: 128,

            // Metal specifies this as its max
            max_task_payload_size: 16384 - 32,
            // DX12 limitation, revisit for vulkan
            max_mesh_output_vertices: 256,
            max_mesh_output_primitives: 256,
            // llvmpipe once again requires this to be 8. An RTX 3060 supports well over 1024.
            // Also DX12 vaguely suggests going over this is illegal in some cases.
            max_mesh_output_layers: 8,
            // llvmpipe reports 0 multiview count, which just means no multiview is allowed
            max_mesh_multiview_view_count: 0,
            ..self
        }
    }

    /// Compares every limits within self is within the limits given in `allowed`.
    ///
    /// If you need detailed information on failures, look at [`Limits::check_limits_with_fail_fn`].
    #[must_use]
    pub fn check_limits(&self, allowed: &Self) -> bool {
        let mut within = true;
        self.check_limits_with_fail_fn(allowed, true, |_, _, _| within = false);
        within
    }

    /// Compares every limits within self is within the limits given in `allowed`.
    /// For an easy to use binary choice, use [`Limits::check_limits`].
    ///
    /// If a value is not within the allowed limit, this function calls the `fail_fn`
    /// with the:
    ///  - limit name
    ///  - self's limit
    ///  - allowed's limit.
    ///
    /// If fatal is true, a single failure bails out the comparison after a single failure.
    pub fn check_limits_with_fail_fn(
        &self,
        allowed: &Self,
        fatal: bool,
        mut fail_fn: impl FnMut(&'static str, u64, u64),
    ) {
        macro_rules! check_with_fail_fn {
            ($name:ident, $ordering:expr) => {
                let invalid_ord = $ordering.reverse();
                if self.$name.cmp(&allowed.$name) == invalid_ord {
                    fail_fn(stringify!($name), self.$name as u64, allowed.$name as u64);
                    if fatal {
                        return;
                    }
                }
            };
        }

        with_limits!(check_with_fail_fn);
    }

    /// For each limit in `other` that is better than the value in `self`,
    /// replace the value in `self` with the value from `other`.
    ///
    /// A request for a limit value less than the WebGPU-specified default must
    /// be ignored. This function is used to clamp such requests to the default
    /// value.
    ///
    /// This function is not for clamping requests for values beyond the
    /// supported limits. For that purpose the desired function would be
    /// `or_worse_values_from`.
    #[must_use]
    pub fn or_better_values_from(mut self, other: &Self) -> Self {
        macro_rules! or_better_value_from {
            ($name:ident, $ordering:expr) => {
                match $ordering {
                    // Limits that are maximum values (most of them)
                    Ordering::Less => self.$name = self.$name.max(other.$name),
                    // Limits that are minimum values
                    Ordering::Greater => self.$name = self.$name.min(other.$name),
                    Ordering::Equal => unreachable!(),
                }
            };
        }

        with_limits!(or_better_value_from);

        self
    }

    /// For each limit in `other` that is worse than the value in `self`,
    /// replace the value in `self` with the value from `other`.
    ///
    /// This function is for clamping requests for values beyond the
    /// supported limits.
    #[must_use]
    pub fn or_worse_values_from(mut self, other: &Self) -> Self {
        macro_rules! or_worse_value_from {
            ($name:ident, $ordering:expr) => {
                match $ordering {
                    // Limits that are maximum values (most of them)
                    Ordering::Less => self.$name = self.$name.min(other.$name),
                    // Limits that are minimum values
                    Ordering::Greater => self.$name = self.$name.max(other.$name),
                    Ordering::Equal => unreachable!(),
                }
            };
        }

        with_limits!(or_worse_value_from);

        self
    }
}

/// Represents the sets of additional limits on an adapter,
/// which take place when running on downlevel backends.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DownlevelLimits {}

#[allow(clippy::derivable_impls)]
impl Default for DownlevelLimits {
    fn default() -> Self {
        DownlevelLimits {}
    }
}

/// Lists various ways the underlying platform does not conform to the WebGPU standard.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DownlevelCapabilities {
    /// Combined boolean flags.
    pub flags: DownlevelFlags,
    /// Additional limits
    pub limits: DownlevelLimits,
    /// Which collections of features shaders support. Defined in terms of D3D's shader models.
    pub shader_model: ShaderModel,
}

impl Default for DownlevelCapabilities {
    fn default() -> Self {
        Self {
            flags: DownlevelFlags::all(),
            limits: DownlevelLimits::default(),
            shader_model: ShaderModel::Sm5,
        }
    }
}

impl DownlevelCapabilities {
    /// Returns true if the underlying platform offers complete support of the baseline WebGPU standard.
    ///
    /// If this returns false, some parts of the API will result in validation errors where they would not normally.
    /// These parts can be determined by the values in this structure.
    #[must_use]
    pub fn is_webgpu_compliant(&self) -> bool {
        self.flags.contains(DownlevelFlags::compliant())
            && self.limits == DownlevelLimits::default()
            && self.shader_model >= ShaderModel::Sm5
    }
}

bitflags::bitflags! {
    /// Binary flags listing features that may or may not be present on downlevel adapters.
    ///
    /// A downlevel adapter is a GPU adapter that wgpu supports, but with potentially limited
    /// features, due to the lack of hardware feature support.
    ///
    /// Flags that are **not** present for a downlevel adapter or device usually indicates
    /// non-compliance with the WebGPU specification, but not always.
    ///
    /// You can check whether a set of flags is compliant through the
    /// [`DownlevelCapabilities::is_webgpu_compliant()`] function.
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    #[cfg_attr(feature = "serde", serde(transparent))]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DownlevelFlags: u32 {
        /// The device supports compiling and using compute shaders.
        ///
        /// WebGL2, and GLES3.0 devices do not support compute.
        const COMPUTE_SHADERS = 1 << 0;
        /// Supports binding storage buffers and textures to fragment shaders.
        const FRAGMENT_WRITABLE_STORAGE = 1 << 1;
        /// Supports indirect drawing and dispatching.
        ///
        /// [`Self::COMPUTE_SHADERS`] must be present for this flag.
        ///
        /// WebGL2, GLES 3.0, and Metal on Apple1/Apple2 GPUs do not support indirect.
        const INDIRECT_EXECUTION = 1 << 2;
        /// Supports non-zero `base_vertex` parameter to direct indexed draw calls.
        ///
        /// Indirect calls, if supported, always support non-zero `base_vertex`.
        ///
        /// Supported by:
        /// - Vulkan
        /// - DX12
        /// - Metal on Apple3+ or Mac1+
        /// - OpenGL 3.2+
        /// - OpenGL ES 3.2
        const BASE_VERTEX = 1 << 3;
        /// Supports reading from a depth/stencil texture while using it as a read-only
        /// depth/stencil attachment.
        ///
        /// The WebGL2 and GLES backends do not support RODS.
        const READ_ONLY_DEPTH_STENCIL = 1 << 4;
        /// Supports textures with mipmaps which have a non power of two size.
        const NON_POWER_OF_TWO_MIPMAPPED_TEXTURES = 1 << 5;
        /// Supports textures that are cube arrays.
        const CUBE_ARRAY_TEXTURES = 1 << 6;
        /// Supports comparison samplers.
        const COMPARISON_SAMPLERS = 1 << 7;
        /// Supports different blend operations per color attachment.
        const INDEPENDENT_BLEND = 1 << 8;
        /// Supports storage buffers in vertex shaders.
        const VERTEX_STORAGE = 1 << 9;

        /// Supports samplers with anisotropic filtering. Note this isn't actually required by
        /// WebGPU, the implementation is allowed to completely ignore aniso clamp. This flag is
        /// here for native backends so they can communicate to the user of aniso is enabled.
        ///
        /// All backends and all devices support anisotropic filtering.
        const ANISOTROPIC_FILTERING = 1 << 10;

        /// Supports storage buffers in fragment shaders.
        const FRAGMENT_STORAGE = 1 << 11;

        /// Supports sample-rate shading.
        const MULTISAMPLED_SHADING = 1 << 12;

        /// Supports copies between depth textures and buffers.
        ///
        /// GLES/WebGL don't support this.
        const DEPTH_TEXTURE_AND_BUFFER_COPIES = 1 << 13;

        /// Supports all the texture usages described in WebGPU. If this isn't supported, you
        /// should call `get_texture_format_features` to get how you can use textures of a given format
        const WEBGPU_TEXTURE_FORMAT_SUPPORT = 1 << 14;

        /// Supports buffer bindings with sizes that aren't a multiple of 16.
        ///
        /// WebGL doesn't support this.
        const BUFFER_BINDINGS_NOT_16_BYTE_ALIGNED = 1 << 15;

        /// Supports buffers to combine [`BufferUsages::INDEX`] with usages other than [`BufferUsages::COPY_DST`] and [`BufferUsages::COPY_SRC`].
        /// Furthermore, in absence of this feature it is not allowed to copy index buffers from/to buffers with a set of usage flags containing
        /// [`BufferUsages::VERTEX`]/[`BufferUsages::UNIFORM`]/[`BufferUsages::STORAGE`] or [`BufferUsages::INDIRECT`].
        ///
        /// WebGL doesn't support this.
        const UNRESTRICTED_INDEX_BUFFER = 1 << 16;

        /// Supports full 32-bit range indices (2^32-1 as opposed to 2^24-1 without this flag)
        ///
        /// Corresponds to Vulkan's `VkPhysicalDeviceFeatures.fullDrawIndexUint32`
        const FULL_DRAW_INDEX_UINT32 = 1 << 17;

        /// Supports depth bias clamping
        ///
        /// Corresponds to Vulkan's `VkPhysicalDeviceFeatures.depthBiasClamp`
        const DEPTH_BIAS_CLAMP = 1 << 18;

        /// Supports specifying which view format values are allowed when create_view() is called on a texture.
        ///
        /// The WebGL and GLES backends doesn't support this.
        const VIEW_FORMATS = 1 << 19;

        /// With this feature not present, there are the following restrictions on `Queue::copy_external_image_to_texture`:
        /// - The source must not be [`web_sys::OffscreenCanvas`]
        /// - [`CopyExternalImageSourceInfo::origin`] must be zero.
        /// - [`CopyExternalImageDestInfo::color_space`] must be srgb.
        /// - If the source is an [`web_sys::ImageBitmap`]:
        ///   - [`CopyExternalImageSourceInfo::flip_y`] must be false.
        ///   - [`CopyExternalImageDestInfo::premultiplied_alpha`] must be false.
        ///
        /// WebGL doesn't support this. WebGPU does.
        const UNRESTRICTED_EXTERNAL_TEXTURE_COPIES = 1 << 20;

        /// Supports specifying which view formats are allowed when calling create_view on the texture returned by
        /// `Surface::get_current_texture`.
        ///
        /// The GLES/WebGL and Vulkan on Android doesn't support this.
        const SURFACE_VIEW_FORMATS = 1 << 21;

        /// If this is true, calls to `CommandEncoder::resolve_query_set` will be performed on the queue timeline.
        ///
        /// If this is false, calls to `CommandEncoder::resolve_query_set` will be performed on the device (i.e. cpu) timeline
        /// and will block that timeline until the query has data. You may work around this limitation by waiting until the submit
        /// whose queries you are resolving is fully finished (through use of `queue.on_submitted_work_done`) and only
        /// then submitting the resolve_query_set command. The queries will be guaranteed finished, so will not block.
        ///
        /// Supported by:
        /// - Vulkan,
        /// - DX12
        /// - Metal
        /// - OpenGL 4.4+
        ///
        /// Not Supported by:
        /// - GL ES / WebGL
        const NONBLOCKING_QUERY_RESOLVE = 1 << 22;

        /// Allows shaders to use `quantizeToF16`, `pack2x16float`, and `unpack2x16float`, which
        /// operate on `f16`-precision values stored in `f32`s.
        ///
        /// Not supported by Vulkan on Mesa when [`Features::SHADER_F16`] is absent.
        const SHADER_F16_IN_F32 = 1 << 23;
    }
}

impl DownlevelFlags {
    /// All flags that indicate if the backend is WebGPU compliant
    #[must_use]
    pub const fn compliant() -> Self {
        // We use manual bit twiddling to make this a const fn as `Sub` and `.remove` aren't const

        // WebGPU doesn't actually require aniso
        Self::from_bits_truncate(Self::all().bits() & !Self::ANISOTROPIC_FILTERING.bits())
    }
}

/// Collections of shader features a device supports if they support less than WebGPU normally allows.
// TODO: Fill out the differences between shader models more completely
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ShaderModel {
    /// Extremely limited shaders, including a total instruction limit.
    Sm2,
    /// Missing minor features and storage images.
    Sm4,
    /// WebGPU supports shader module 5.
    Sm5,
}
