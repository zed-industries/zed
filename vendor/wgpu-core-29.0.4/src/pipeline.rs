use alloc::{
    borrow::Cow,
    boxed::Box,
    string::{String, ToString as _},
    sync::Arc,
    vec::Vec,
};
use core::{marker::PhantomData, mem::ManuallyDrop, num::NonZeroU32};

use arrayvec::ArrayVec;
use naga::error::ShaderError;
use thiserror::Error;
use wgt::error::{ErrorType, WebGpuError};

pub use crate::pipeline_cache::PipelineCacheValidationError;
use crate::{
    binding_model::{
        BindGroupLayout, CreateBindGroupLayoutError, CreatePipelineLayoutError,
        GetBindGroupLayoutError, PipelineLayout,
    },
    command::ColorAttachmentError,
    device::{Device, DeviceError, MissingDownlevelFlags, MissingFeatures, RenderPassContext},
    id::{PipelineCacheId, PipelineLayoutId, ShaderModuleId},
    resource::{InvalidResourceError, Labeled, TrackingData},
    resource_log, validation, Label,
};

/// Information about buffer bindings, which
/// is validated against the shader (and pipeline)
/// at draw time as opposed to initialization time.
#[derive(Debug, Default)]
pub(crate) struct LateSizedBufferGroup {
    // The order has to match `BindGroup::late_buffer_binding_sizes`.
    pub(crate) shader_sizes: Vec<wgt::BufferAddress>,
}

#[allow(clippy::large_enum_variant)]
pub enum ShaderModuleSource<'a> {
    #[cfg(feature = "wgsl")]
    Wgsl(Cow<'a, str>),
    #[cfg(feature = "glsl")]
    Glsl(Cow<'a, str>, naga::front::glsl::Options),
    #[cfg(feature = "spirv")]
    SpirV(Cow<'a, [u32]>, naga::front::spv::Options),
    Naga(Cow<'static, naga::Module>),
    /// Dummy variant because `Naga` doesn't have a lifetime and without enough active features it
    /// could be the last one active.
    #[doc(hidden)]
    Dummy(PhantomData<&'a ()>),
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ShaderModuleDescriptor<'a> {
    pub label: Label<'a>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub runtime_checks: wgt::ShaderRuntimeChecks,
}

pub type ShaderModuleDescriptorPassthrough<'a> =
    wgt::CreateShaderModuleDescriptorPassthrough<'a, Label<'a>>;

#[derive(Debug)]
pub struct ShaderModule {
    pub(crate) raw: ManuallyDrop<Box<dyn hal::DynShaderModule>>,
    pub(crate) device: Arc<Device>,
    pub(crate) interface: Option<validation::Interface>,
    /// The `label` from the descriptor used to create the resource.
    pub(crate) label: String,
}

impl Drop for ShaderModule {
    fn drop(&mut self) {
        resource_log!("Destroy raw {}", self.error_ident());
        // SAFETY: We are in the Drop impl and we don't use self.raw anymore after this point.
        let raw = unsafe { ManuallyDrop::take(&mut self.raw) };
        unsafe {
            self.device.raw().destroy_shader_module(raw);
        }
    }
}

crate::impl_resource_type!(ShaderModule);
crate::impl_labeled!(ShaderModule);
crate::impl_parent_device!(ShaderModule);
crate::impl_storage_item!(ShaderModule);

impl ShaderModule {
    pub(crate) fn raw(&self) -> &dyn hal::DynShaderModule {
        self.raw.as_ref()
    }

    pub(crate) fn finalize_entry_point_name(
        &self,
        stage: naga::ShaderStage,
        entry_point: Option<&str>,
    ) -> Result<String, validation::StageError> {
        match &self.interface {
            Some(interface) => interface.finalize_entry_point_name(stage, entry_point),
            None => entry_point
                .map(|ep| ep.to_string())
                .ok_or(validation::StageError::NoEntryPointFound),
        }
    }
}

//Note: `Clone` would require `WithSpan: Clone`.
#[derive(Clone, Debug, Error)]
#[non_exhaustive]
pub enum CreateShaderModuleError {
    #[cfg(feature = "wgsl")]
    #[error(transparent)]
    Parsing(#[from] ShaderError<naga::front::wgsl::ParseError>),
    #[cfg(feature = "glsl")]
    #[error(transparent)]
    ParsingGlsl(#[from] ShaderError<naga::front::glsl::ParseErrors>),
    #[cfg(feature = "spirv")]
    #[error(transparent)]
    ParsingSpirV(#[from] ShaderError<naga::front::spv::Error>),
    #[error("Failed to generate the backend-specific code")]
    Generation,
    #[error(transparent)]
    Device(#[from] DeviceError),
    #[error(transparent)]
    Validation(#[from] ShaderError<naga::WithSpan<naga::valid::ValidationError>>),
    #[error(transparent)]
    MissingFeatures(#[from] MissingFeatures),
    #[error(
        "Shader global {bind:?} uses a group index {group} that exceeds the max_bind_groups limit of {limit}."
    )]
    InvalidGroupIndex {
        bind: naga::ResourceBinding,
        group: u32,
        limit: u32,
    },
    #[error("Generic shader passthrough does not contain any code compatible with this backend.")]
    NotCompiledForBackend,
}

impl WebGpuError for CreateShaderModuleError {
    fn webgpu_error_type(&self) -> ErrorType {
        match self {
            Self::Device(e) => e.webgpu_error_type(),
            Self::MissingFeatures(e) => e.webgpu_error_type(),

            Self::Generation => ErrorType::Internal,

            Self::Validation(..) | Self::InvalidGroupIndex { .. } => ErrorType::Validation,
            #[cfg(feature = "wgsl")]
            Self::Parsing(..) => ErrorType::Validation,
            #[cfg(feature = "glsl")]
            Self::ParsingGlsl(..) => ErrorType::Validation,
            #[cfg(feature = "spirv")]
            Self::ParsingSpirV(..) => ErrorType::Validation,
            Self::NotCompiledForBackend => ErrorType::Validation,
        }
    }
}

/// Describes a programmable pipeline stage.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProgrammableStageDescriptor<'a, SM = ShaderModuleId> {
    /// The compiled shader module for this stage.
    pub module: SM,
    /// The name of the entry point in the compiled shader. The name is selected using the
    /// following logic:
    ///
    /// * If `Some(name)` is specified, there must be a function with this name in the shader.
    /// * If a single entry point associated with this stage must be in the shader, then proceed as
    ///   if `Some(…)` was specified with that entry point's name.
    pub entry_point: Option<Cow<'a, str>>,
    /// Specifies the values of pipeline-overridable constants in the shader module.
    ///
    /// If an `@id` attribute was specified on the declaration,
    /// the key must be the pipeline constant ID as a decimal ASCII number; if not,
    /// the key must be the constant's identifier name.
    ///
    /// The value may represent any of WGSL's concrete scalar types.
    pub constants: naga::back::PipelineConstants,
    /// Whether workgroup scoped memory will be initialized with zero values for this stage.
    ///
    /// This is required by the WebGPU spec, but may have overhead which can be avoided
    /// for cross-platform applications
    pub zero_initialize_workgroup_memory: bool,
}

/// cbindgen:ignore
pub type ResolvedProgrammableStageDescriptor<'a> =
    ProgrammableStageDescriptor<'a, Arc<ShaderModule>>;

/// Number of implicit bind groups derived at pipeline creation.
pub type ImplicitBindGroupCount = u8;

#[derive(Clone, Debug, Error)]
#[non_exhaustive]
pub enum ImplicitLayoutError {
    #[error("Unable to reflect the shader {0:?} interface")]
    ReflectionError(wgt::ShaderStages),
    #[error(transparent)]
    BindGroup(#[from] CreateBindGroupLayoutError),
    #[error(transparent)]
    Pipeline(#[from] CreatePipelineLayoutError),
    #[error("Unable to create implicit pipeline layout from passthrough shader stage: {0:?}")]
    Passthrough(wgt::ShaderStages),
}

impl WebGpuError for ImplicitLayoutError {
    fn webgpu_error_type(&self) -> ErrorType {
        match self {
            Self::ReflectionError(_) => ErrorType::Validation,
            Self::BindGroup(e) => e.webgpu_error_type(),
            Self::Pipeline(e) => e.webgpu_error_type(),
            Self::Passthrough(_) => ErrorType::Validation,
        }
    }
}

/// Describes a compute pipeline.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ComputePipelineDescriptor<
    'a,
    PLL = PipelineLayoutId,
    SM = ShaderModuleId,
    PLC = PipelineCacheId,
> {
    pub label: Label<'a>,
    /// The layout of bind groups for this pipeline.
    pub layout: Option<PLL>,
    /// The compiled compute stage and its entry point.
    pub stage: ProgrammableStageDescriptor<'a, SM>,
    /// The pipeline cache to use when creating this pipeline.
    pub cache: Option<PLC>,
}

/// cbindgen:ignore
pub type ResolvedComputePipelineDescriptor<'a> =
    ComputePipelineDescriptor<'a, Arc<PipelineLayout>, Arc<ShaderModule>, Arc<PipelineCache>>;

#[derive(Clone, Debug, Error)]
#[non_exhaustive]
pub enum CreateComputePipelineError {
    #[error(transparent)]
    Device(#[from] DeviceError),
    #[error("Unable to derive an implicit layout")]
    Implicit(#[from] ImplicitLayoutError),
    #[error("Error matching shader requirements against the pipeline")]
    Stage(#[from] validation::StageError),
    #[error("Internal error: {0}")]
    Internal(String),
    #[error("Pipeline constant error: {0}")]
    PipelineConstants(String),
    #[error(transparent)]
    MissingDownlevelFlags(#[from] MissingDownlevelFlags),
    #[error(transparent)]
    InvalidResource(#[from] InvalidResourceError),
}

impl WebGpuError for CreateComputePipelineError {
    fn webgpu_error_type(&self) -> ErrorType {
        match self {
            Self::Device(e) => e.webgpu_error_type(),
            Self::InvalidResource(e) => e.webgpu_error_type(),
            Self::MissingDownlevelFlags(e) => e.webgpu_error_type(),
            Self::Implicit(e) => e.webgpu_error_type(),
            Self::Stage(e) => e.webgpu_error_type(),
            Self::Internal(_) => ErrorType::Internal,
            Self::PipelineConstants(_) => ErrorType::Validation,
        }
    }
}

#[derive(Debug)]
pub struct ComputePipeline {
    pub(crate) raw: ManuallyDrop<Box<dyn hal::DynComputePipeline>>,
    pub(crate) layout: Arc<PipelineLayout>,
    pub(crate) device: Arc<Device>,
    pub(crate) _shader_module: Arc<ShaderModule>,
    pub(crate) late_sized_buffer_groups: ArrayVec<LateSizedBufferGroup, { hal::MAX_BIND_GROUPS }>,
    /// The `label` from the descriptor used to create the resource.
    pub(crate) label: String,
    pub(crate) tracking_data: TrackingData,
}

impl Drop for ComputePipeline {
    fn drop(&mut self) {
        resource_log!("Destroy raw {}", self.error_ident());
        // SAFETY: We are in the Drop impl and we don't use self.raw anymore after this point.
        let raw = unsafe { ManuallyDrop::take(&mut self.raw) };
        unsafe {
            self.device.raw().destroy_compute_pipeline(raw);
        }
    }
}

crate::impl_resource_type!(ComputePipeline);
crate::impl_labeled!(ComputePipeline);
crate::impl_parent_device!(ComputePipeline);
crate::impl_storage_item!(ComputePipeline);
crate::impl_trackable!(ComputePipeline);

impl ComputePipeline {
    pub(crate) fn raw(&self) -> &dyn hal::DynComputePipeline {
        self.raw.as_ref()
    }

    pub fn get_bind_group_layout(
        self: &Arc<Self>,
        index: u32,
    ) -> Result<Arc<BindGroupLayout>, GetBindGroupLayoutError> {
        self.layout.get_bind_group_layout(index)
    }
}

#[derive(Clone, Debug, Error)]
#[non_exhaustive]
pub enum CreatePipelineCacheError {
    #[error(transparent)]
    Device(#[from] DeviceError),
    #[error("Pipeline cache validation failed")]
    Validation(#[from] PipelineCacheValidationError),
    #[error(transparent)]
    MissingFeatures(#[from] MissingFeatures),
}

impl WebGpuError for CreatePipelineCacheError {
    fn webgpu_error_type(&self) -> ErrorType {
        match self {
            Self::Device(e) => e.webgpu_error_type(),
            Self::Validation(e) => e.webgpu_error_type(),
            Self::MissingFeatures(e) => e.webgpu_error_type(),
        }
    }
}

#[derive(Debug)]
pub struct PipelineCache {
    pub(crate) raw: ManuallyDrop<Box<dyn hal::DynPipelineCache>>,
    pub(crate) device: Arc<Device>,
    /// The `label` from the descriptor used to create the resource.
    pub(crate) label: String,
}

impl Drop for PipelineCache {
    fn drop(&mut self) {
        resource_log!("Destroy raw {}", self.error_ident());
        // SAFETY: We are in the Drop impl and we don't use self.raw anymore after this point.
        let raw = unsafe { ManuallyDrop::take(&mut self.raw) };
        unsafe {
            self.device.raw().destroy_pipeline_cache(raw);
        }
    }
}

crate::impl_resource_type!(PipelineCache);
crate::impl_labeled!(PipelineCache);
crate::impl_parent_device!(PipelineCache);
crate::impl_storage_item!(PipelineCache);

impl PipelineCache {
    pub(crate) fn raw(&self) -> &dyn hal::DynPipelineCache {
        self.raw.as_ref()
    }
}

/// Describes how the vertex buffer is interpreted.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct VertexBufferLayout<'a> {
    /// The stride, in bytes, between elements of this buffer.
    pub array_stride: wgt::BufferAddress,
    /// How often this vertex buffer is "stepped" forward.
    pub step_mode: wgt::VertexStepMode,
    /// The list of attributes which comprise a single vertex.
    pub attributes: Cow<'a, [wgt::VertexAttribute]>,
}

/// A null vertex buffer layout that may be placed in unused slots.
impl Default for VertexBufferLayout<'_> {
    fn default() -> Self {
        Self {
            array_stride: Default::default(),
            step_mode: Default::default(),
            attributes: Cow::Borrowed(&[]),
        }
    }
}

/// Describes the vertex process in a render pipeline.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VertexState<'a, SM = ShaderModuleId> {
    /// The compiled vertex stage and its entry point.
    pub stage: ProgrammableStageDescriptor<'a, SM>,
    /// The format of any vertex buffers used with this pipeline.
    pub buffers: Cow<'a, [VertexBufferLayout<'a>]>,
}

/// cbindgen:ignore
pub type ResolvedVertexState<'a> = VertexState<'a, Arc<ShaderModule>>;

/// Describes fragment processing in a render pipeline.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FragmentState<'a, SM = ShaderModuleId> {
    /// The compiled fragment stage and its entry point.
    pub stage: ProgrammableStageDescriptor<'a, SM>,
    /// The effect of draw calls on the color aspect of the output target.
    pub targets: Cow<'a, [Option<wgt::ColorTargetState>]>,
}

/// cbindgen:ignore
pub type ResolvedFragmentState<'a> = FragmentState<'a, Arc<ShaderModule>>;

/// Describes the task shader in a mesh shader pipeline.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TaskState<'a, SM = ShaderModuleId> {
    /// The compiled task stage and its entry point.
    pub stage: ProgrammableStageDescriptor<'a, SM>,
}

pub type ResolvedTaskState<'a> = TaskState<'a, Arc<ShaderModule>>;

/// Describes the mesh shader in a mesh shader pipeline.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MeshState<'a, SM = ShaderModuleId> {
    /// The compiled mesh stage and its entry point.
    pub stage: ProgrammableStageDescriptor<'a, SM>,
}

pub type ResolvedMeshState<'a> = MeshState<'a, Arc<ShaderModule>>;

/// Describes a vertex processor for either a conventional or mesh shading
/// pipeline architecture.
///
/// This is not a public API. It is for use by `player` only. The public APIs
/// are [`VertexState`], [`TaskState`], and [`MeshState`].
///
/// cbindgen:ignore
#[doc(hidden)]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RenderPipelineVertexProcessor<'a, SM = ShaderModuleId> {
    Vertex(VertexState<'a, SM>),
    Mesh(Option<TaskState<'a, SM>>, MeshState<'a, SM>),
}

/// Describes a render (graphics) pipeline.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RenderPipelineDescriptor<
    'a,
    PLL = PipelineLayoutId,
    SM = ShaderModuleId,
    PLC = PipelineCacheId,
> {
    pub label: Label<'a>,
    /// The layout of bind groups for this pipeline.
    pub layout: Option<PLL>,
    /// The vertex processing state for this pipeline.
    pub vertex: VertexState<'a, SM>,
    /// The properties of the pipeline at the primitive assembly and rasterization level.
    #[cfg_attr(feature = "serde", serde(default))]
    pub primitive: wgt::PrimitiveState,
    /// The effect of draw calls on the depth and stencil aspects of the output target, if any.
    #[cfg_attr(feature = "serde", serde(default))]
    pub depth_stencil: Option<wgt::DepthStencilState>,
    /// The multi-sampling properties of the pipeline.
    #[cfg_attr(feature = "serde", serde(default))]
    pub multisample: wgt::MultisampleState,
    /// The fragment processing state for this pipeline.
    pub fragment: Option<FragmentState<'a, SM>>,
    /// If the pipeline will be used with a multiview render pass, this indicates how many array
    /// layers the attachments will have.
    pub multiview_mask: Option<NonZeroU32>,
    /// The pipeline cache to use when creating this pipeline.
    pub cache: Option<PLC>,
}
/// Describes a mesh shader pipeline.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MeshPipelineDescriptor<
    'a,
    PLL = PipelineLayoutId,
    SM = ShaderModuleId,
    PLC = PipelineCacheId,
> {
    pub label: Label<'a>,
    /// The layout of bind groups for this pipeline.
    pub layout: Option<PLL>,
    /// The task processing state for this pipeline.
    pub task: Option<TaskState<'a, SM>>,
    /// The mesh processing state for this pipeline
    pub mesh: MeshState<'a, SM>,
    /// The properties of the pipeline at the primitive assembly and rasterization level.
    #[cfg_attr(feature = "serde", serde(default))]
    pub primitive: wgt::PrimitiveState,
    /// The effect of draw calls on the depth and stencil aspects of the output target, if any.
    #[cfg_attr(feature = "serde", serde(default))]
    pub depth_stencil: Option<wgt::DepthStencilState>,
    /// The multi-sampling properties of the pipeline.
    #[cfg_attr(feature = "serde", serde(default))]
    pub multisample: wgt::MultisampleState,
    /// The fragment processing state for this pipeline.
    pub fragment: Option<FragmentState<'a, SM>>,
    /// If the pipeline will be used with a multiview render pass, this indicates how many array
    /// layers the attachments will have.
    pub multiview: Option<NonZeroU32>,
    /// The pipeline cache to use when creating this pipeline.
    pub cache: Option<PLC>,
}

/// Describes a render (graphics) pipeline, with either conventional or mesh
/// shading architecture.
///
/// This is not a public API. It is for use by `player` only. The public APIs
/// are [`RenderPipelineDescriptor`] and [`MeshPipelineDescriptor`].
///
/// cbindgen:ignore
#[doc(hidden)]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GeneralRenderPipelineDescriptor<
    'a,
    PLL = PipelineLayoutId,
    SM = ShaderModuleId,
    PLC = PipelineCacheId,
> {
    pub label: Label<'a>,
    /// The layout of bind groups for this pipeline.
    pub layout: Option<PLL>,
    /// The vertex processing state for this pipeline.
    pub vertex: RenderPipelineVertexProcessor<'a, SM>,
    /// The properties of the pipeline at the primitive assembly and rasterization level.
    #[cfg_attr(feature = "serde", serde(default))]
    pub primitive: wgt::PrimitiveState,
    /// The effect of draw calls on the depth and stencil aspects of the output target, if any.
    #[cfg_attr(feature = "serde", serde(default))]
    pub depth_stencil: Option<wgt::DepthStencilState>,
    /// The multi-sampling properties of the pipeline.
    #[cfg_attr(feature = "serde", serde(default))]
    pub multisample: wgt::MultisampleState,
    /// The fragment processing state for this pipeline.
    pub fragment: Option<FragmentState<'a, SM>>,
    /// If the pipeline will be used with a multiview render pass, this indicates how many array
    /// layers the attachments will have.
    pub multiview_mask: Option<NonZeroU32>,
    /// The pipeline cache to use when creating this pipeline.
    pub cache: Option<PLC>,
}
impl<'a, PLL, SM, PLC> From<RenderPipelineDescriptor<'a, PLL, SM, PLC>>
    for GeneralRenderPipelineDescriptor<'a, PLL, SM, PLC>
{
    fn from(value: RenderPipelineDescriptor<'a, PLL, SM, PLC>) -> Self {
        Self {
            label: value.label,
            layout: value.layout,
            vertex: RenderPipelineVertexProcessor::Vertex(value.vertex),
            primitive: value.primitive,
            depth_stencil: value.depth_stencil,
            multisample: value.multisample,
            fragment: value.fragment,
            multiview_mask: value.multiview_mask,
            cache: value.cache,
        }
    }
}
impl<'a, PLL, SM, PLC> From<MeshPipelineDescriptor<'a, PLL, SM, PLC>>
    for GeneralRenderPipelineDescriptor<'a, PLL, SM, PLC>
{
    fn from(value: MeshPipelineDescriptor<'a, PLL, SM, PLC>) -> Self {
        Self {
            label: value.label,
            layout: value.layout,
            vertex: RenderPipelineVertexProcessor::Mesh(value.task, value.mesh),
            primitive: value.primitive,
            depth_stencil: value.depth_stencil,
            multisample: value.multisample,
            fragment: value.fragment,
            multiview_mask: value.multiview,
            cache: value.cache,
        }
    }
}

/// Not a public API. For use by `player` only.
///
/// cbindgen:ignore
pub type ResolvedGeneralRenderPipelineDescriptor<'a> =
    GeneralRenderPipelineDescriptor<'a, Arc<PipelineLayout>, Arc<ShaderModule>, Arc<PipelineCache>>;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PipelineCacheDescriptor<'a> {
    pub label: Label<'a>,
    pub data: Option<Cow<'a, [u8]>>,
    pub fallback: bool,
}

#[derive(Clone, Debug, Error)]
#[non_exhaustive]
pub enum ColorStateError {
    #[error("Format {0:?} is not renderable")]
    FormatNotRenderable(wgt::TextureFormat),
    #[error("Format {0:?} is not blendable")]
    FormatNotBlendable(wgt::TextureFormat),
    #[error("Format {0:?} does not have a color aspect")]
    FormatNotColor(wgt::TextureFormat),
    #[error("Sample count {0} is not supported by format {1:?} on this device. The WebGPU spec guarantees {2:?} samples are supported by this format. With the TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES feature your device supports {3:?}.")]
    InvalidSampleCount(u32, wgt::TextureFormat, Vec<u32>, Vec<u32>),
    #[error("Output format {pipeline} is incompatible with the shader {shader}")]
    IncompatibleFormat {
        pipeline: validation::NumericType,
        shader: validation::NumericType,
    },
    #[error("Invalid write mask {0:?}")]
    InvalidWriteMask(wgt::ColorWrites),
    #[error("Using the blend factor {factor:?} for render target {target} is not possible. Only the first render target may be used when dual-source blending.")]
    BlendFactorOnUnsupportedTarget {
        factor: wgt::BlendFactor,
        target: u32,
    },
    #[error(
        "Blend factor {factor:?} for render target {target} is not valid. Blend factor must be `one` when using min/max blend operations."
    )]
    InvalidMinMaxBlendFactor {
        factor: wgt::BlendFactor,
        target: u32,
    },
}

#[derive(Clone, Debug, Error)]
#[non_exhaustive]
pub enum DepthStencilStateError {
    #[error("Format {0:?} is not renderable")]
    FormatNotRenderable(wgt::TextureFormat),
    #[error("Format {0:?} is not a depth/stencil format")]
    FormatNotDepthOrStencil(wgt::TextureFormat),
    #[error("Format {0:?} does not have a depth aspect, but depth test/write is enabled")]
    FormatNotDepth(wgt::TextureFormat),
    #[error("Format {0:?} does not have a stencil aspect, but stencil test/write is enabled")]
    FormatNotStencil(wgt::TextureFormat),
    #[error("Sample count {0} is not supported by format {1:?} on this device. The WebGPU spec guarantees {2:?} samples are supported by this format. With the TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES feature your device supports {3:?}.")]
    InvalidSampleCount(u32, wgt::TextureFormat, Vec<u32>, Vec<u32>),
    #[error("Depth bias is not compatible with non-triangle topology {0:?}")]
    DepthBiasWithIncompatibleTopology(wgt::PrimitiveTopology),
    #[error("Depth compare function must be specified for depth format {0:?}")]
    MissingDepthCompare(wgt::TextureFormat),
    #[error("Depth write enabled must be specified for depth format {0:?}")]
    MissingDepthWriteEnabled(wgt::TextureFormat),
}

#[derive(Clone, Debug, Error)]
#[non_exhaustive]
pub enum CreateRenderPipelineError {
    #[error(transparent)]
    ColorAttachment(#[from] ColorAttachmentError),
    #[error(transparent)]
    Device(#[from] DeviceError),
    #[error("Unable to derive an implicit layout")]
    Implicit(#[from] ImplicitLayoutError),
    #[error("Color state [{0}] is invalid")]
    ColorState(u8, #[source] ColorStateError),
    #[error("Depth/stencil state is invalid")]
    DepthStencilState(#[from] DepthStencilStateError),
    #[error("Invalid sample count {0}")]
    InvalidSampleCount(u32),
    #[error("The number of vertex buffers {given} exceeds the limit {limit}")]
    TooManyVertexBuffers { given: u32, limit: u32 },
    #[error("The total number of vertex attributes {given} exceeds the limit {limit}")]
    TooManyVertexAttributes { given: u32, limit: u32 },
    #[error("Vertex attribute location {given} must be less than limit {limit}")]
    VertexAttributeLocationTooLarge { given: u32, limit: u32 },
    #[error("Vertex buffer {index} stride {given} exceeds the limit {limit}")]
    VertexStrideTooLarge { index: u32, given: u32, limit: u32 },
    #[error("Vertex attribute at location {location} stride {given} exceeds the limit {limit}")]
    VertexAttributeStrideTooLarge {
        location: wgt::ShaderLocation,
        given: u32,
        limit: u32,
    },
    #[error("Vertex buffer {index} stride {stride} does not respect `VERTEX_ALIGNMENT`")]
    UnalignedVertexStride {
        index: u32,
        stride: wgt::BufferAddress,
    },
    #[error("Vertex attribute at location {location} has invalid offset {offset}")]
    InvalidVertexAttributeOffset {
        location: wgt::ShaderLocation,
        offset: wgt::BufferAddress,
    },
    #[error("Two or more vertex attributes were assigned to the same location in the shader: {0}")]
    ShaderLocationClash(u32),
    #[error("Strip index format was not set to None but to {strip_index_format:?} while using the non-strip topology {topology:?}")]
    StripIndexFormatForNonStripTopology {
        strip_index_format: Option<wgt::IndexFormat>,
        topology: wgt::PrimitiveTopology,
    },
    #[error("Conservative Rasterization is only supported for wgt::PolygonMode::Fill")]
    ConservativeRasterizationNonFillPolygonMode,
    #[error(transparent)]
    MissingFeatures(#[from] MissingFeatures),
    #[error(transparent)]
    MissingDownlevelFlags(#[from] MissingDownlevelFlags),
    #[error("Error matching {stage:?} shader requirements against the pipeline")]
    Stage {
        stage: wgt::ShaderStages,
        #[source]
        error: validation::StageError,
    },
    #[error("Internal error in {stage:?} shader: {error}")]
    Internal {
        stage: wgt::ShaderStages,
        error: String,
    },
    #[error("Pipeline constant error in {stage:?} shader: {error}")]
    PipelineConstants {
        stage: wgt::ShaderStages,
        error: String,
    },
    #[error("In the provided shader, the type given for group {group} binding {binding} has a size of {size}. As the device does not support `DownlevelFlags::BUFFER_BINDINGS_NOT_16_BYTE_ALIGNED`, the type must have a size that is a multiple of 16 bytes.")]
    UnalignedShader { group: u32, binding: u32, size: u64 },
    #[error("Dual-source blending requires exactly one color target, but {count} color targets are present")]
    DualSourceBlendingWithMultipleColorTargets { count: usize },
    #[error("{}", concat!(
        "At least one color attachment or depth-stencil attachment was expected, ",
        "but no render target for the pipeline was specified."
    ))]
    NoTargetSpecified,
    #[error(transparent)]
    InvalidResource(#[from] InvalidResourceError),
}

impl WebGpuError for CreateRenderPipelineError {
    fn webgpu_error_type(&self) -> ErrorType {
        match self {
            Self::Device(e) => e.webgpu_error_type(),
            Self::InvalidResource(e) => e.webgpu_error_type(),
            Self::MissingFeatures(e) => e.webgpu_error_type(),
            Self::MissingDownlevelFlags(e) => e.webgpu_error_type(),

            Self::Internal { .. } => ErrorType::Internal,

            Self::ColorAttachment(_)
            | Self::Implicit(_)
            | Self::ColorState(_, _)
            | Self::DepthStencilState(_)
            | Self::InvalidSampleCount(_)
            | Self::TooManyVertexBuffers { .. }
            | Self::TooManyVertexAttributes { .. }
            | Self::VertexAttributeLocationTooLarge { .. }
            | Self::VertexStrideTooLarge { .. }
            | Self::UnalignedVertexStride { .. }
            | Self::InvalidVertexAttributeOffset { .. }
            | Self::ShaderLocationClash(_)
            | Self::StripIndexFormatForNonStripTopology { .. }
            | Self::ConservativeRasterizationNonFillPolygonMode
            | Self::Stage { .. }
            | Self::UnalignedShader { .. }
            | Self::DualSourceBlendingWithMultipleColorTargets { .. }
            | Self::NoTargetSpecified
            | Self::PipelineConstants { .. }
            | Self::VertexAttributeStrideTooLarge { .. } => ErrorType::Validation,
        }
    }
}

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct PipelineFlags: u32 {
        const BLEND_CONSTANT = 1 << 0;
        const STENCIL_REFERENCE = 1 << 1;
        const WRITES_DEPTH = 1 << 2;
        const WRITES_STENCIL = 1 << 3;
    }
}

/// How a render pipeline will retrieve attributes from a particular vertex buffer.
#[derive(Clone, Copy, Debug)]
pub struct VertexStep {
    /// The byte stride in the buffer between one attribute value and the next.
    pub stride: wgt::BufferAddress,

    /// The byte size required to fit the last vertex in the stream.
    pub last_stride: wgt::BufferAddress,

    /// Whether the buffer is indexed by vertex number or instance number.
    pub mode: wgt::VertexStepMode,
}

impl Default for VertexStep {
    fn default() -> Self {
        Self {
            stride: 0,
            last_stride: 0,
            mode: wgt::VertexStepMode::Vertex,
        }
    }
}

#[derive(Debug)]
pub struct RenderPipeline {
    pub(crate) raw: ManuallyDrop<Box<dyn hal::DynRenderPipeline>>,
    pub(crate) device: Arc<Device>,
    pub(crate) layout: Arc<PipelineLayout>,
    pub(crate) _shader_modules: ArrayVec<Arc<ShaderModule>, { hal::MAX_CONCURRENT_SHADER_STAGES }>,
    pub(crate) pass_context: RenderPassContext,
    pub(crate) flags: PipelineFlags,
    pub(crate) topology: wgt::PrimitiveTopology,
    pub(crate) strip_index_format: Option<wgt::IndexFormat>,
    pub(crate) vertex_steps: Vec<VertexStep>,
    pub(crate) late_sized_buffer_groups: ArrayVec<LateSizedBufferGroup, { hal::MAX_BIND_GROUPS }>,
    /// The `label` from the descriptor used to create the resource.
    pub(crate) label: String,
    pub(crate) tracking_data: TrackingData,
    /// Whether this is a mesh shader pipeline
    pub(crate) is_mesh: bool,
}

impl Drop for RenderPipeline {
    fn drop(&mut self) {
        resource_log!("Destroy raw {}", self.error_ident());
        // SAFETY: We are in the Drop impl and we don't use self.raw anymore after this point.
        let raw = unsafe { ManuallyDrop::take(&mut self.raw) };
        unsafe {
            self.device.raw().destroy_render_pipeline(raw);
        }
    }
}

crate::impl_resource_type!(RenderPipeline);
crate::impl_labeled!(RenderPipeline);
crate::impl_parent_device!(RenderPipeline);
crate::impl_storage_item!(RenderPipeline);
crate::impl_trackable!(RenderPipeline);

impl RenderPipeline {
    pub(crate) fn raw(&self) -> &dyn hal::DynRenderPipeline {
        self.raw.as_ref()
    }

    pub fn get_bind_group_layout(
        self: &Arc<Self>,
        index: u32,
    ) -> Result<Arc<BindGroupLayout>, GetBindGroupLayoutError> {
        self.layout.get_bind_group_layout(index)
    }
}
