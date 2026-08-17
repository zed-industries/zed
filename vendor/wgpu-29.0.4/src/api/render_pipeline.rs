use core::num::NonZeroU32;

use crate::*;

/// Handle to a rendering (graphics) pipeline.
///
/// A `RenderPipeline` object represents a graphics pipeline and its stages, bindings, vertex
/// buffers and targets. It can be created with [`Device::create_render_pipeline`].
///
/// Corresponds to [WebGPU `GPURenderPipeline`](https://gpuweb.github.io/gpuweb/#render-pipeline).
#[derive(Debug, Clone)]
pub struct RenderPipeline {
    pub(crate) inner: dispatch::DispatchRenderPipeline,
}
#[cfg(send_sync)]
static_assertions::assert_impl_all!(RenderPipeline: Send, Sync);

crate::cmp::impl_eq_ord_hash_proxy!(RenderPipeline => .inner);

impl RenderPipeline {
    /// Get an object representing the bind group layout at a given index.
    ///
    /// If this pipeline was created with a [default layout][RenderPipelineDescriptor::layout], then
    /// bind groups created with the returned `BindGroupLayout` can only be used with this pipeline.
    ///
    /// This method will raise a validation error if there is no bind group layout at `index`.
    pub fn get_bind_group_layout(&self, index: u32) -> BindGroupLayout {
        let layout = self.inner.get_bind_group_layout(index);
        BindGroupLayout { inner: layout }
    }

    #[cfg(custom)]
    /// Returns custom implementation of RenderPipeline (if custom backend and is internally T)
    pub fn as_custom<T: custom::RenderPipelineInterface>(&self) -> Option<&T> {
        self.inner.as_custom()
    }
}

/// Specifies an interpretation of the bytes of a vertex buffer as vertex attributes.
///
/// Use this in a [`RenderPipelineDescriptor`] to describe the format of the vertex buffers that
/// are passed to [`RenderPass::set_vertex_buffer()`].
///
/// Corresponds to [WebGPU `GPUVertexBufferLayout`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpuvertexbufferlayout).
///
/// # Example
///
/// The following example defines a `struct` with three fields,
/// and a [`VertexBufferLayout`] that contains [`VertexAttribute`]s for each field,
/// using the [`vertex_attr_array!`] macro to compute attribute offsets:
///
/// ```
/// #[repr(C, packed)]
/// struct Vertex {
///     foo: [f32; 2],
///     bar: f32,
///     baz: [u16; 4],
/// }
///
/// impl Vertex {
///     /// Layout to use with a buffer whose contents are a `[Vertex]`.
///     pub const LAYOUT: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
///         array_stride: size_of::<Self>() as wgpu::BufferAddress,
///         step_mode: wgpu::VertexStepMode::Vertex,
///         attributes: &wgpu::vertex_attr_array![
///             0 => Float32x2,
///             1 => Float32,
///             2 => Uint16x4,
///         ],
///     };
/// }
///
/// # assert_eq!(Vertex::LAYOUT.attributes[2].offset, Vertex::LAYOUT.array_stride - 2 * 4);
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct VertexBufferLayout<'a> {
    /// The stride, in bytes, between elements of this buffer (between vertices).
    ///
    /// This must be a multiple of [`VERTEX_ALIGNMENT`].
    pub array_stride: BufferAddress,
    /// How often this vertex buffer is "stepped" forward.
    pub step_mode: VertexStepMode,
    /// The list of attributes which comprise a single vertex.
    pub attributes: &'a [VertexAttribute],
}
static_assertions::assert_impl_all!(VertexBufferLayout<'_>: Send, Sync);

/// Describes the vertex processing in a render pipeline.
///
/// For use in [`RenderPipelineDescriptor`].
///
/// Corresponds to [WebGPU `GPUVertexState`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpuvertexstate).
#[derive(Clone, Debug)]
pub struct VertexState<'a> {
    /// The compiled shader module for this stage.
    pub module: &'a ShaderModule,
    /// The name of the entry point in the compiled shader to use.
    ///
    /// If [`Some`], there must be a vertex-stage shader entry point with this name in `module`.
    /// Otherwise, expect exactly one vertex-stage entry point in `module`, which will be
    /// selected.
    // NOTE: keep phrasing in sync. with `ComputePipelineDescriptor::entry_point`
    // NOTE: keep phrasing in sync. with `FragmentState::entry_point`
    pub entry_point: Option<&'a str>,
    /// Advanced options for when this pipeline is compiled
    ///
    /// This implements `Default`, and for most users can be set to `Default::default()`
    pub compilation_options: PipelineCompilationOptions<'a>,
    /// The format of any vertex buffers used with this pipeline via
    /// [`RenderPass::set_vertex_buffer()`].
    ///
    /// The attribute locations and types specified in this layout must match the
    /// locations and types of the inputs to the `entry_point` function.
    pub buffers: &'a [VertexBufferLayout<'a>],
}
#[cfg(send_sync)]
static_assertions::assert_impl_all!(VertexState<'_>: Send, Sync);

/// Describes the fragment processing in a render pipeline.
///
/// For use in [`RenderPipelineDescriptor`].
///
/// Corresponds to [WebGPU `GPUFragmentState`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpufragmentstate).
#[derive(Clone, Debug)]
pub struct FragmentState<'a> {
    /// The compiled shader module for this stage.
    pub module: &'a ShaderModule,
    /// The name of the entry point in the compiled shader to use.
    ///
    /// If [`Some`], there must be a `@fragment` shader entry point with this name in `module`.
    /// Otherwise, expect exactly one fragment-stage entry point in `module`, which will be
    /// selected.
    // NOTE: keep phrasing in sync. with `ComputePipelineDescriptor::entry_point`
    // NOTE: keep phrasing in sync. with `VertexState::entry_point`
    pub entry_point: Option<&'a str>,
    /// Advanced options for when this pipeline is compiled
    ///
    /// This implements `Default`, and for most users can be set to `Default::default()`
    pub compilation_options: PipelineCompilationOptions<'a>,
    /// The color state of the render targets.
    pub targets: &'a [Option<ColorTargetState>],
}
#[cfg(send_sync)]
static_assertions::assert_impl_all!(FragmentState<'_>: Send, Sync);

/// Describes the task shader stage in a mesh shader pipeline.
///
/// For use in [`MeshPipelineDescriptor`]
#[derive(Clone, Debug)]
pub struct TaskState<'a> {
    /// The compiled shader module for this stage.
    pub module: &'a ShaderModule,

    /// The name of the task shader entry point in the shader module to use.
    ///
    /// If [`Some`], there must be a task shader entry point with the given name
    /// in `module`. Otherwise, there must be exactly one task shader entry
    /// point in `module`, which will be selected.
    pub entry_point: Option<&'a str>,

    /// Advanced options for when this pipeline is compiled.
    ///
    /// This implements `Default`, and for most users can be set to `Default::default()`
    pub compilation_options: PipelineCompilationOptions<'a>,
}
#[cfg(send_sync)]
static_assertions::assert_impl_all!(TaskState<'_>: Send, Sync);

/// Describes the mesh shader stage in a mesh shader pipeline.
///
/// For use in [`MeshPipelineDescriptor`]
#[derive(Clone, Debug)]
pub struct MeshState<'a> {
    /// The compiled shader module for this stage.
    pub module: &'a ShaderModule,
    /// The name of the entry point in the compiled shader to use.
    ///
    /// If [`Some`], there must be a vertex-stage shader entry point with this name in `module`.
    /// Otherwise, expect exactly one vertex-stage entry point in `module`, which will be
    /// selected.
    pub entry_point: Option<&'a str>,
    /// Advanced options for when this pipeline is compiled
    ///
    /// This implements `Default`, and for most users can be set to `Default::default()`
    pub compilation_options: PipelineCompilationOptions<'a>,
}
#[cfg(send_sync)]
static_assertions::assert_impl_all!(MeshState<'_>: Send, Sync);

/// Describes a render (graphics) pipeline.
///
/// For use with [`Device::create_render_pipeline`].
///
/// Corresponds to [WebGPU `GPURenderPipelineDescriptor`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpurenderpipelinedescriptor).
#[derive(Clone, Debug)]
pub struct RenderPipelineDescriptor<'a> {
    /// Debug label of the pipeline. This will show up in graphics debuggers for easy identification.
    pub label: Label<'a>,
    /// The layout of bind groups for this pipeline.
    ///
    /// If this is set, then [`Device::create_render_pipeline`] will raise a validation error if
    /// the layout doesn't match what the shader module(s) expect.
    ///
    /// Using the same [`PipelineLayout`] for many [`RenderPipeline`] or [`ComputePipeline`]
    /// pipelines guarantees that you don't have to rebind any resources when switching between
    /// those pipelines.
    ///
    /// ## Default pipeline layout
    ///
    /// If `layout` is `None`, then the pipeline has a [default layout] created and used instead.
    /// The default layout is deduced from the shader modules.
    ///
    /// You can use [`RenderPipeline::get_bind_group_layout`] to create bind groups for use with the
    /// default layout. However, these bind groups cannot be used with any other pipelines. This is
    /// convenient for simple pipelines, but using an explicit layout is recommended in most cases.
    ///
    /// [default layout]: https://www.w3.org/TR/webgpu/#default-pipeline-layout
    pub layout: Option<&'a PipelineLayout>,
    /// The compiled vertex stage, its entry point, and the input buffers layout.
    pub vertex: VertexState<'a>,
    /// The properties of the pipeline at the primitive assembly and rasterization level.
    pub primitive: PrimitiveState,
    /// The effect of draw calls on the depth and stencil aspects of the output target, if any.
    pub depth_stencil: Option<DepthStencilState>,
    /// The multi-sampling properties of the pipeline.
    pub multisample: MultisampleState,
    /// The compiled fragment stage, its entry point, and the color targets.
    pub fragment: Option<FragmentState<'a>>,
    /// If the pipeline will be used with a multiview render pass, this indicates what multiview
    /// mask the render pass will be used with. The masks must match exactly.
    ///
    /// For example, if you wish to render to the first 2 layers, you would use 3=0b11. If you
    /// wanted to render to only the 2nd layer, you would use 2=0b10. If you aren't using
    /// multiview this should be `None`.
    pub multiview_mask: Option<NonZeroU32>,
    /// The pipeline cache to use when creating this pipeline.
    pub cache: Option<&'a PipelineCache>,
}
#[cfg(send_sync)]
static_assertions::assert_impl_all!(RenderPipelineDescriptor<'_>: Send, Sync);

/// Describes a mesh shader (graphics) pipeline.
///
/// For use with [`Device::create_mesh_pipeline`]. A mesh pipeline is very much
/// like a render pipeline, except that instead of [`RenderPass::draw`] it is
/// invoked with [`RenderPass::draw_mesh_tasks`], and instead of a vertex shader
/// and a fragment shader:
///
/// - [`task`] specifies an optional task shader entry point, which determines how
///   many groups of mesh shaders to dispatch.
///
/// - [`mesh`] specifies a mesh shader entry point, which generates groups of
///   primitives to draw
///
/// - [`fragment`] specifies as fragment shader for drawing those primitives,
///   just like in an ordinary render pipeline.
///
/// The key difference is that, whereas a vertex shader is invoked on the
/// elements of vertex buffers, the task shader gets to decide how many mesh
/// shader workgroups to make, and then each mesh shader workgroup gets to
/// decide which primitives it wants to generate, and what their vertex
/// attributes are. Task and mesh shaders can use whatever they please as
/// inputs, like a compute shader. However, they cannot use specialized vertex
/// or index buffers.
///
/// A mesh pipeline is invoked by [`RenderPass::draw_mesh_tasks`], which looks
/// like a compute shader dispatch with [`ComputePass::dispatch_workgroups`]:
/// you pass `x`, `y`, and `z` values indicating the number of task shaders to
/// invoke in parallel. The output value of the first thread in a task shader
/// workgroup determines how many mesh workgroups should be dispatched from there.
/// Those mesh workgroups also get a special payload passed from the task shader.
///
/// If the task shader is omitted, then the (`x`, `y`, `z`) parameters to
/// `draw_mesh_tasks` are used to decide how many invocations of the mesh shader
/// to invoke directly, without a task payload.
///
/// [vertex formats]: wgpu_types::VertexFormat
/// [`task`]: Self::task
/// [`mesh`]: Self::mesh
/// [`fragment`]: Self::fragment
#[derive(Clone, Debug)]
pub struct MeshPipelineDescriptor<'a> {
    /// Debug label of the pipeline. This will show up in graphics debuggers for easy identification.
    pub label: Label<'a>,
    /// The layout of bind groups for this pipeline.
    ///
    /// If this is set, then [`Device::create_render_pipeline`] will raise a validation error if
    /// the layout doesn't match what the shader module(s) expect.
    ///
    /// Using the same [`PipelineLayout`] for many [`RenderPipeline`] or [`ComputePipeline`]
    /// pipelines guarantees that you don't have to rebind any resources when switching between
    /// those pipelines.
    ///
    /// ## Default pipeline layout
    ///
    /// If `layout` is `None`, then the pipeline has a [default layout] created and used instead.
    /// The default layout is deduced from the shader modules.
    ///
    /// You can use [`RenderPipeline::get_bind_group_layout`] to create bind groups for use with the
    /// default layout. However, these bind groups cannot be used with any other pipelines. This is
    /// convenient for simple pipelines, but using an explicit layout is recommended in most cases.
    ///
    /// [default layout]: https://www.w3.org/TR/webgpu/#default-pipeline-layout
    pub layout: Option<&'a PipelineLayout>,

    /// The mesh pipeline's task shader.
    ///
    /// If this is `None`, the mesh pipeline has no task shader. Executing a
    /// mesh drawing command simply dispatches a grid of mesh shaders directly.
    ///
    /// [`draw_mesh_tasks`]: RenderPass::draw_mesh_tasks
    pub task: Option<TaskState<'a>>,

    /// The compiled mesh stage and its entry point
    pub mesh: MeshState<'a>,
    /// The properties of the pipeline at the primitive assembly and rasterization level.
    pub primitive: PrimitiveState,
    /// The effect of draw calls on the depth and stencil aspects of the output target, if any.
    pub depth_stencil: Option<DepthStencilState>,
    /// The multi-sampling properties of the pipeline.
    pub multisample: MultisampleState,
    /// The compiled fragment stage, its entry point, and the color targets.
    pub fragment: Option<FragmentState<'a>>,
    /// If the pipeline will be used with a multiview render pass, this indicates how many array
    /// layers the attachments will have.
    pub multiview: Option<NonZeroU32>,
    /// The pipeline cache to use when creating this pipeline.
    pub cache: Option<&'a PipelineCache>,
}
#[cfg(send_sync)]
static_assertions::assert_impl_all!(MeshPipelineDescriptor<'_>: Send, Sync);
