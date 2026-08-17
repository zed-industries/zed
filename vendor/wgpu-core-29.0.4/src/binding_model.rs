use alloc::{
    borrow::{Cow, ToOwned},
    boxed::Box,
    string::String,
    sync::{Arc, Weak},
    vec::Vec,
};
use core::{fmt, mem::ManuallyDrop, ops::Range};

use arrayvec::ArrayVec;
use thiserror::Error;

#[cfg(feature = "serde")]
use serde::Deserialize;
#[cfg(feature = "serde")]
use serde::Serialize;

use wgt::error::{ErrorType, WebGpuError};

use crate::{
    device::{bgl, Device, DeviceError, MissingDownlevelFlags, MissingFeatures},
    id::{BindGroupLayoutId, BufferId, ExternalTextureId, SamplerId, TextureViewId, TlasId},
    init_tracker::{BufferInitTrackerAction, TextureInitTrackerAction},
    pipeline::{ComputePipeline, RenderPipeline},
    resource::{
        Buffer, DestroyedResourceError, ExternalTexture, InvalidResourceError, Labeled,
        MissingBufferUsageError, MissingTextureUsageError, RawResourceAccess, ResourceErrorIdent,
        Sampler, TextureView, Tlas, TrackingData,
    },
    resource_log,
    snatch::{SnatchGuard, Snatchable},
    track::{BindGroupStates, ResourceUsageCompatibilityError},
    Label,
};

#[derive(Clone, Debug, Error)]
#[non_exhaustive]
pub enum BindGroupLayoutEntryError {
    #[error("Cube dimension is not expected for texture storage")]
    StorageTextureCube,
    #[error("Atomic storage textures are not allowed by baseline webgpu, they require the native only feature TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES")]
    StorageTextureAtomic,
    #[error("Arrays of bindings unsupported for this type of binding")]
    ArrayUnsupported,
    #[error("Multisampled binding with sample type `TextureSampleType::Float` must have filterable set to false.")]
    SampleTypeFloatFilterableBindingMultisampled,
    #[error("Multisampled texture binding view dimension must be 2d, got {0:?}")]
    Non2DMultisampled(wgt::TextureViewDimension),
    #[error(transparent)]
    MissingFeatures(#[from] MissingFeatures),
    #[error(transparent)]
    MissingDownlevelFlags(#[from] MissingDownlevelFlags),
}

#[derive(Clone, Debug, Error)]
#[non_exhaustive]
pub enum CreateBindGroupLayoutError {
    #[error(transparent)]
    Device(#[from] DeviceError),
    #[error("Conflicting binding at index {0}")]
    ConflictBinding(u32),
    #[error("Binding {binding} entry is invalid")]
    Entry {
        binding: u32,
        #[source]
        error: BindGroupLayoutEntryError,
    },
    #[error(transparent)]
    TooManyBindings(BindingTypeMaxCountError),
    #[error("Bind groups may not contain both a binding array and a dynamically offset buffer")]
    ContainsBothBindingArrayAndDynamicOffsetArray,
    #[error("Bind groups may not contain both a binding array and a uniform buffer")]
    ContainsBothBindingArrayAndUniformBuffer,
    #[error("Binding index {binding} is greater than the maximum number {maximum}")]
    InvalidBindingIndex { binding: u32, maximum: u32 },
    #[error("Invalid visibility {0:?}")]
    InvalidVisibility(wgt::ShaderStages),
    #[error("Binding index {binding}: {access:?} access to storage textures with format {format:?} is not supported")]
    UnsupportedStorageTextureAccess {
        binding: u32,
        access: wgt::StorageTextureAccess,
        format: wgt::TextureFormat,
    },
}

impl WebGpuError for CreateBindGroupLayoutError {
    fn webgpu_error_type(&self) -> ErrorType {
        match self {
            Self::Device(e) => e.webgpu_error_type(),

            Self::ConflictBinding(_)
            | Self::Entry { .. }
            | Self::TooManyBindings(_)
            | Self::InvalidBindingIndex { .. }
            | Self::InvalidVisibility(_)
            | Self::ContainsBothBindingArrayAndDynamicOffsetArray
            | Self::ContainsBothBindingArrayAndUniformBuffer
            | Self::UnsupportedStorageTextureAccess { .. } => ErrorType::Validation,
        }
    }
}

#[derive(Clone, Debug, Error)]
#[non_exhaustive]
pub enum BindingError {
    #[error(transparent)]
    DestroyedResource(#[from] DestroyedResourceError),
    #[error("Buffer {buffer}: Binding with size {binding_size} at offset {offset} would overflow buffer size of {buffer_size}")]
    BindingRangeTooLarge {
        buffer: ResourceErrorIdent,
        offset: wgt::BufferAddress,
        binding_size: u64,
        buffer_size: u64,
    },
    #[error("Buffer {buffer}: Binding offset {offset} is greater than buffer size {buffer_size}")]
    BindingOffsetTooLarge {
        buffer: ResourceErrorIdent,
        offset: wgt::BufferAddress,
        buffer_size: u64,
    },
}

impl WebGpuError for BindingError {
    fn webgpu_error_type(&self) -> ErrorType {
        match self {
            Self::DestroyedResource(e) => e.webgpu_error_type(),
            Self::BindingRangeTooLarge { .. } | Self::BindingOffsetTooLarge { .. } => {
                ErrorType::Validation
            }
        }
    }
}

// TODO: there may be additional variants here that can be extracted into
// `BindingError`.
#[derive(Clone, Debug, Error)]
#[non_exhaustive]
pub enum CreateBindGroupError {
    #[error(transparent)]
    Device(#[from] DeviceError),
    #[error(transparent)]
    DestroyedResource(#[from] DestroyedResourceError),
    #[error(transparent)]
    BindingError(#[from] BindingError),
    #[error(
        "Binding count declared with at most {expected} items, but {actual} items were provided"
    )]
    BindingArrayPartialLengthMismatch { actual: usize, expected: usize },
    #[error(
        "Binding count declared with exactly {expected} items, but {actual} items were provided"
    )]
    BindingArrayLengthMismatch { actual: usize, expected: usize },
    #[error("Array binding provided zero elements")]
    BindingArrayZeroLength,
    #[error("Binding size {actual} of {buffer} is less than minimum {min}")]
    BindingSizeTooSmall {
        buffer: ResourceErrorIdent,
        actual: u64,
        min: u64,
    },
    #[error("{0} binding size is zero")]
    BindingZeroSize(ResourceErrorIdent),
    #[error("Number of bindings in bind group descriptor ({actual}) does not match the number of bindings defined in the bind group layout ({expected})")]
    BindingsNumMismatch { actual: usize, expected: usize },
    #[error("Binding {0} is used at least twice in the descriptor")]
    DuplicateBinding(u32),
    #[error("Unable to find a corresponding declaration for the given binding {0}")]
    MissingBindingDeclaration(u32),
    #[error(transparent)]
    MissingBufferUsage(#[from] MissingBufferUsageError),
    #[error(transparent)]
    MissingTextureUsage(#[from] MissingTextureUsageError),
    #[error("Binding declared as a single item, but bind group is using it as an array")]
    SingleBindingExpected,
    #[error("Effective buffer binding size {size} for storage buffers is expected to align to {alignment}, but size is {size}")]
    UnalignedEffectiveBufferBindingSizeForStorage { alignment: u32, size: u64 },
    #[error("Buffer offset {0} does not respect device's requested `{1}` limit {2}")]
    UnalignedBufferOffset(wgt::BufferAddress, &'static str, u32),
    #[error(
        "Buffer binding {binding} range {given} exceeds `max_*_buffer_binding_size` limit {limit}"
    )]
    BufferRangeTooLarge {
        binding: u32,
        given: u64,
        limit: u64,
    },
    #[error("Binding {binding} has a different type ({actual:?}) than the one in the layout ({expected:?})")]
    WrongBindingType {
        // Index of the binding
        binding: u32,
        // The type given to the function
        actual: wgt::BindingType,
        // Human-readable description of expected types
        expected: &'static str,
    },
    #[error("Texture binding {binding} expects multisampled = {layout_multisampled}, but given a view with samples = {view_samples}")]
    InvalidTextureMultisample {
        binding: u32,
        layout_multisampled: bool,
        view_samples: u32,
    },
    #[error(
        "Texture binding {} expects sample type {:?}, but was given a view with format {:?} (sample type {:?})",
        binding,
        layout_sample_type,
        view_format,
        view_sample_type
    )]
    InvalidTextureSampleType {
        binding: u32,
        layout_sample_type: wgt::TextureSampleType,
        view_format: wgt::TextureFormat,
        view_sample_type: wgt::TextureSampleType,
    },
    #[error("Texture binding {binding} expects dimension = {layout_dimension:?}, but given a view with dimension = {view_dimension:?}")]
    InvalidTextureDimension {
        binding: u32,
        layout_dimension: wgt::TextureViewDimension,
        view_dimension: wgt::TextureViewDimension,
    },
    #[error("Storage texture binding {binding} expects format = {layout_format:?}, but given a view with format = {view_format:?}")]
    InvalidStorageTextureFormat {
        binding: u32,
        layout_format: wgt::TextureFormat,
        view_format: wgt::TextureFormat,
    },
    #[error("Storage texture bindings must have a single mip level, but given a view with mip_level_count = {mip_level_count:?} at binding {binding}")]
    InvalidStorageTextureMipLevelCount { binding: u32, mip_level_count: u32 },
    #[error("External texture bindings must have a single mip level, but given a view with mip_level_count = {mip_level_count:?} at binding {binding}")]
    InvalidExternalTextureMipLevelCount { binding: u32, mip_level_count: u32 },
    #[error("External texture bindings must have a format of `rgba8unorm`, `bgra8unorm`, or `rgba16float, but given a view with format = {format:?} at binding {binding}")]
    InvalidExternalTextureFormat {
        binding: u32,
        format: wgt::TextureFormat,
    },
    #[error("Sampler binding {binding} expects comparison = {layout_cmp}, but given a sampler with comparison = {sampler_cmp}")]
    WrongSamplerComparison {
        binding: u32,
        layout_cmp: bool,
        sampler_cmp: bool,
    },
    #[error("Sampler binding {binding} expects filtering = {layout_flt}, but given a sampler with filtering = {sampler_flt}")]
    WrongSamplerFiltering {
        binding: u32,
        layout_flt: bool,
        sampler_flt: bool,
    },
    #[error("TLAS binding {binding} is required to support vertex returns but is missing flag AccelerationStructureFlags::ALLOW_RAY_HIT_VERTEX_RETURN")]
    MissingTLASVertexReturn { binding: u32 },
    #[error("Bound texture views can not have both depth and stencil aspects enabled")]
    DepthStencilAspect,
    #[error(transparent)]
    ResourceUsageCompatibility(#[from] ResourceUsageCompatibilityError),
    #[error(transparent)]
    InvalidResource(#[from] InvalidResourceError),
}

impl WebGpuError for CreateBindGroupError {
    fn webgpu_error_type(&self) -> ErrorType {
        match self {
            Self::Device(e) => e.webgpu_error_type(),
            Self::DestroyedResource(e) => e.webgpu_error_type(),
            Self::BindingError(e) => e.webgpu_error_type(),
            Self::MissingBufferUsage(e) => e.webgpu_error_type(),
            Self::MissingTextureUsage(e) => e.webgpu_error_type(),
            Self::ResourceUsageCompatibility(e) => e.webgpu_error_type(),
            Self::InvalidResource(e) => e.webgpu_error_type(),
            Self::BindingArrayPartialLengthMismatch { .. }
            | Self::BindingArrayLengthMismatch { .. }
            | Self::BindingArrayZeroLength
            | Self::BindingSizeTooSmall { .. }
            | Self::BindingsNumMismatch { .. }
            | Self::BindingZeroSize(_)
            | Self::DuplicateBinding(_)
            | Self::MissingBindingDeclaration(_)
            | Self::SingleBindingExpected
            | Self::UnalignedEffectiveBufferBindingSizeForStorage { .. }
            | Self::UnalignedBufferOffset(_, _, _)
            | Self::BufferRangeTooLarge { .. }
            | Self::WrongBindingType { .. }
            | Self::InvalidTextureMultisample { .. }
            | Self::InvalidTextureSampleType { .. }
            | Self::InvalidTextureDimension { .. }
            | Self::InvalidStorageTextureFormat { .. }
            | Self::InvalidStorageTextureMipLevelCount { .. }
            | Self::WrongSamplerComparison { .. }
            | Self::WrongSamplerFiltering { .. }
            | Self::DepthStencilAspect
            | Self::MissingTLASVertexReturn { .. }
            | Self::InvalidExternalTextureMipLevelCount { .. }
            | Self::InvalidExternalTextureFormat { .. } => ErrorType::Validation,
        }
    }
}

#[derive(Clone, Debug, Error)]
pub enum BindingZone {
    #[error("Stage {0:?}")]
    Stage(wgt::ShaderStages),
    #[error("Whole pipeline")]
    Pipeline,
}

#[derive(Clone, Debug, Error)]
#[error("Too many bindings of type {kind:?} in {zone}, limit is {limit}, count was {count}. Check the limit `{}` passed to `Adapter::request_device`", .kind.to_config_str())]
pub struct BindingTypeMaxCountError {
    pub kind: BindingTypeMaxCountErrorKind,
    pub zone: BindingZone,
    pub limit: u32,
    pub count: u32,
}

impl WebGpuError for BindingTypeMaxCountError {
    fn webgpu_error_type(&self) -> ErrorType {
        ErrorType::Validation
    }
}

#[derive(Clone, Debug)]
pub enum BindingTypeMaxCountErrorKind {
    DynamicUniformBuffers,
    DynamicStorageBuffers,
    SampledTextures,
    Samplers,
    StorageBuffers,
    StorageTextures,
    UniformBuffers,
    BindingArrayElements,
    BindingArraySamplerElements,
    BindingArrayAccelerationStructureElements,
    AccelerationStructures,
}

impl BindingTypeMaxCountErrorKind {
    fn to_config_str(&self) -> &'static str {
        match self {
            BindingTypeMaxCountErrorKind::DynamicUniformBuffers => {
                "max_dynamic_uniform_buffers_per_pipeline_layout"
            }
            BindingTypeMaxCountErrorKind::DynamicStorageBuffers => {
                "max_dynamic_storage_buffers_per_pipeline_layout"
            }
            BindingTypeMaxCountErrorKind::SampledTextures => {
                "max_sampled_textures_per_shader_stage"
            }
            BindingTypeMaxCountErrorKind::Samplers => "max_samplers_per_shader_stage",
            BindingTypeMaxCountErrorKind::StorageBuffers => "max_storage_buffers_per_shader_stage",
            BindingTypeMaxCountErrorKind::StorageTextures => {
                "max_storage_textures_per_shader_stage"
            }
            BindingTypeMaxCountErrorKind::UniformBuffers => "max_uniform_buffers_per_shader_stage",
            BindingTypeMaxCountErrorKind::BindingArrayElements => {
                "max_binding_array_elements_per_shader_stage"
            }
            BindingTypeMaxCountErrorKind::BindingArraySamplerElements => {
                "max_binding_array_sampler_elements_per_shader_stage"
            }
            BindingTypeMaxCountErrorKind::BindingArrayAccelerationStructureElements => {
                "max_binding_array_acceleration_structure_elements_per_shader_stage"
            }
            BindingTypeMaxCountErrorKind::AccelerationStructures => {
                "max_acceleration_structures_per_shader_stage"
            }
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct PerStageBindingTypeCounter {
    vertex: u32,
    fragment: u32,
    compute: u32,
}

impl PerStageBindingTypeCounter {
    pub(crate) fn add(&mut self, stage: wgt::ShaderStages, count: u32) {
        if stage.contains(wgt::ShaderStages::VERTEX) {
            self.vertex += count;
        }
        if stage.contains(wgt::ShaderStages::FRAGMENT) {
            self.fragment += count;
        }
        if stage.contains(wgt::ShaderStages::COMPUTE) {
            self.compute += count;
        }
    }

    pub(crate) fn max(&self) -> (BindingZone, u32) {
        let max_value = self.vertex.max(self.fragment.max(self.compute));
        let mut stage = wgt::ShaderStages::NONE;
        if max_value == self.vertex {
            stage |= wgt::ShaderStages::VERTEX
        }
        if max_value == self.fragment {
            stage |= wgt::ShaderStages::FRAGMENT
        }
        if max_value == self.compute {
            stage |= wgt::ShaderStages::COMPUTE
        }
        (BindingZone::Stage(stage), max_value)
    }

    pub(crate) fn merge(&mut self, other: &Self) {
        self.vertex += other.vertex;
        self.fragment += other.fragment;
        self.compute += other.compute;
    }

    pub(crate) fn validate(
        &self,
        limit: u32,
        kind: BindingTypeMaxCountErrorKind,
    ) -> Result<(), BindingTypeMaxCountError> {
        let (zone, count) = self.max();
        if limit < count {
            Err(BindingTypeMaxCountError {
                kind,
                zone,
                limit,
                count,
            })
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct BindingTypeMaxCountValidator {
    dynamic_uniform_buffers: u32,
    dynamic_storage_buffers: u32,
    sampled_textures: PerStageBindingTypeCounter,
    samplers: PerStageBindingTypeCounter,
    storage_buffers: PerStageBindingTypeCounter,
    storage_textures: PerStageBindingTypeCounter,
    uniform_buffers: PerStageBindingTypeCounter,
    acceleration_structures: PerStageBindingTypeCounter,
    binding_array_elements: PerStageBindingTypeCounter,
    binding_array_sampler_elements: PerStageBindingTypeCounter,
    binding_array_acceleration_structure_elements: PerStageBindingTypeCounter,
    has_bindless_array: bool,
}

impl BindingTypeMaxCountValidator {
    pub(crate) fn add_binding(&mut self, binding: &wgt::BindGroupLayoutEntry) {
        let count = binding.count.map_or(1, |count| count.get());

        if binding.count.is_some() {
            self.binding_array_elements.add(binding.visibility, count);
            self.has_bindless_array = true;

            match binding.ty {
                wgt::BindingType::Sampler(_) => {
                    self.binding_array_sampler_elements
                        .add(binding.visibility, count);
                }
                wgt::BindingType::AccelerationStructure { .. } => {
                    self.binding_array_acceleration_structure_elements
                        .add(binding.visibility, count);
                }
                _ => {}
            }
        } else {
            match binding.ty {
                wgt::BindingType::Buffer {
                    ty: wgt::BufferBindingType::Uniform,
                    has_dynamic_offset,
                    ..
                } => {
                    self.uniform_buffers.add(binding.visibility, count);
                    if has_dynamic_offset {
                        self.dynamic_uniform_buffers += count;
                    }
                }
                wgt::BindingType::Buffer {
                    ty: wgt::BufferBindingType::Storage { .. },
                    has_dynamic_offset,
                    ..
                } => {
                    self.storage_buffers.add(binding.visibility, count);
                    if has_dynamic_offset {
                        self.dynamic_storage_buffers += count;
                    }
                }
                wgt::BindingType::Sampler { .. } => {
                    self.samplers.add(binding.visibility, count);
                }
                wgt::BindingType::Texture { .. } => {
                    self.sampled_textures.add(binding.visibility, count);
                }
                wgt::BindingType::StorageTexture { .. } => {
                    self.storage_textures.add(binding.visibility, count);
                }
                wgt::BindingType::AccelerationStructure { .. } => {
                    self.acceleration_structures.add(binding.visibility, count);
                }
                wgt::BindingType::ExternalTexture => {
                    // https://www.w3.org/TR/webgpu/#gpuexternaltexture
                    // In order to account for many possible representations,
                    // the binding conservatively uses the following, for each
                    // external texture:
                    // * Three sampled textures for up to 3 planes
                    // * One additional sampled texture for a 3D LUT
                    // * One sampler to sample the LUT
                    // * One uniform buffer for metadata
                    self.sampled_textures.add(binding.visibility, count * 4);
                    self.samplers.add(binding.visibility, count);
                    self.uniform_buffers.add(binding.visibility, count);
                }
            }
        }
    }

    pub(crate) fn merge(&mut self, other: &Self) {
        self.dynamic_uniform_buffers += other.dynamic_uniform_buffers;
        self.dynamic_storage_buffers += other.dynamic_storage_buffers;
        self.sampled_textures.merge(&other.sampled_textures);
        self.samplers.merge(&other.samplers);
        self.storage_buffers.merge(&other.storage_buffers);
        self.storage_textures.merge(&other.storage_textures);
        self.uniform_buffers.merge(&other.uniform_buffers);
        self.acceleration_structures
            .merge(&other.acceleration_structures);
        self.binding_array_elements
            .merge(&other.binding_array_elements);
        self.binding_array_sampler_elements
            .merge(&other.binding_array_sampler_elements);
        self.binding_array_acceleration_structure_elements
            .merge(&other.binding_array_acceleration_structure_elements);
    }

    pub(crate) fn validate(&self, limits: &wgt::Limits) -> Result<(), BindingTypeMaxCountError> {
        if limits.max_dynamic_uniform_buffers_per_pipeline_layout < self.dynamic_uniform_buffers {
            return Err(BindingTypeMaxCountError {
                kind: BindingTypeMaxCountErrorKind::DynamicUniformBuffers,
                zone: BindingZone::Pipeline,
                limit: limits.max_dynamic_uniform_buffers_per_pipeline_layout,
                count: self.dynamic_uniform_buffers,
            });
        }
        if limits.max_dynamic_storage_buffers_per_pipeline_layout < self.dynamic_storage_buffers {
            return Err(BindingTypeMaxCountError {
                kind: BindingTypeMaxCountErrorKind::DynamicStorageBuffers,
                zone: BindingZone::Pipeline,
                limit: limits.max_dynamic_storage_buffers_per_pipeline_layout,
                count: self.dynamic_storage_buffers,
            });
        }
        self.sampled_textures.validate(
            limits.max_sampled_textures_per_shader_stage,
            BindingTypeMaxCountErrorKind::SampledTextures,
        )?;
        self.samplers.validate(
            limits.max_samplers_per_shader_stage,
            BindingTypeMaxCountErrorKind::Samplers,
        )?;
        self.storage_buffers.validate(
            limits.max_storage_buffers_per_shader_stage,
            BindingTypeMaxCountErrorKind::StorageBuffers,
        )?;
        self.storage_textures.validate(
            limits.max_storage_textures_per_shader_stage,
            BindingTypeMaxCountErrorKind::StorageTextures,
        )?;
        self.uniform_buffers.validate(
            limits.max_uniform_buffers_per_shader_stage,
            BindingTypeMaxCountErrorKind::UniformBuffers,
        )?;
        self.binding_array_elements.validate(
            limits.max_binding_array_elements_per_shader_stage,
            BindingTypeMaxCountErrorKind::BindingArrayElements,
        )?;
        self.binding_array_sampler_elements.validate(
            limits.max_binding_array_sampler_elements_per_shader_stage,
            BindingTypeMaxCountErrorKind::BindingArraySamplerElements,
        )?;
        self.binding_array_acceleration_structure_elements
            .validate(
                limits.max_binding_array_acceleration_structure_elements_per_shader_stage,
                BindingTypeMaxCountErrorKind::BindingArrayAccelerationStructureElements,
            )?;
        self.acceleration_structures.validate(
            limits.max_acceleration_structures_per_shader_stage,
            BindingTypeMaxCountErrorKind::AccelerationStructures,
        )?;
        Ok(())
    }

    /// Validate that the bind group layout does not contain both a binding array and a dynamic offset array.
    ///
    /// This allows us to use `UPDATE_AFTER_BIND` on vulkan for bindless arrays. Vulkan does not allow
    /// `UPDATE_AFTER_BIND` on dynamic offset arrays. See <https://github.com/gfx-rs/wgpu/issues/6737>
    pub(crate) fn validate_binding_arrays(&self) -> Result<(), CreateBindGroupLayoutError> {
        let has_dynamic_offset_array =
            self.dynamic_uniform_buffers > 0 || self.dynamic_storage_buffers > 0;
        let has_uniform_buffer = self.uniform_buffers.max().1 > 0;
        if self.has_bindless_array && has_dynamic_offset_array {
            return Err(CreateBindGroupLayoutError::ContainsBothBindingArrayAndDynamicOffsetArray);
        }
        if self.has_bindless_array && has_uniform_buffer {
            return Err(CreateBindGroupLayoutError::ContainsBothBindingArrayAndUniformBuffer);
        }
        Ok(())
    }
}

/// Bindable resource and the slot to bind it to.
/// cbindgen:ignore
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BindGroupEntry<
    'a,
    B = BufferId,
    S = SamplerId,
    TV = TextureViewId,
    TLAS = TlasId,
    ET = ExternalTextureId,
> where
    [BufferBinding<B>]: ToOwned,
    [S]: ToOwned,
    [TV]: ToOwned,
    [TLAS]: ToOwned,
    <[BufferBinding<B>] as ToOwned>::Owned: fmt::Debug,
    <[S] as ToOwned>::Owned: fmt::Debug,
    <[TV] as ToOwned>::Owned: fmt::Debug,
    <[TLAS] as ToOwned>::Owned: fmt::Debug,
{
    /// Slot for which binding provides resource. Corresponds to an entry of the same
    /// binding index in the [`BindGroupLayoutDescriptor`].
    pub binding: u32,
    #[cfg_attr(
        feature = "serde",
        serde(bound(deserialize = "BindingResource<'a, B, S, TV, TLAS, ET>: Deserialize<'de>"))
    )]
    /// Resource to attach to the binding
    pub resource: BindingResource<'a, B, S, TV, TLAS, ET>,
}

/// cbindgen:ignore
pub type ResolvedBindGroupEntry<'a> = BindGroupEntry<
    'a,
    Arc<Buffer>,
    Arc<Sampler>,
    Arc<TextureView>,
    Arc<Tlas>,
    Arc<ExternalTexture>,
>;

/// Describes a group of bindings and the resources to be bound.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BindGroupDescriptor<
    'a,
    BGL = BindGroupLayoutId,
    B = BufferId,
    S = SamplerId,
    TV = TextureViewId,
    TLAS = TlasId,
    ET = ExternalTextureId,
> where
    [BufferBinding<B>]: ToOwned,
    [S]: ToOwned,
    [TV]: ToOwned,
    [TLAS]: ToOwned,
    <[BufferBinding<B>] as ToOwned>::Owned: fmt::Debug,
    <[S] as ToOwned>::Owned: fmt::Debug,
    <[TV] as ToOwned>::Owned: fmt::Debug,
    <[TLAS] as ToOwned>::Owned: fmt::Debug,
    [BindGroupEntry<'a, B, S, TV, TLAS, ET>]: ToOwned,
    <[BindGroupEntry<'a, B, S, TV, TLAS, ET>] as ToOwned>::Owned: fmt::Debug,
{
    /// Debug label of the bind group.
    ///
    /// This will show up in graphics debuggers for easy identification.
    pub label: Label<'a>,
    /// The [`BindGroupLayout`] that corresponds to this bind group.
    pub layout: BGL,
    #[cfg_attr(
        feature = "serde",
        serde(bound(
            deserialize = "<[BindGroupEntry<'a, B, S, TV, TLAS, ET>] as ToOwned>::Owned: Deserialize<'de>"
        ))
    )]
    /// The resources to bind to this bind group.
    #[allow(clippy::type_complexity)]
    pub entries: Cow<'a, [BindGroupEntry<'a, B, S, TV, TLAS, ET>]>,
}

/// cbindgen:ignore
pub type ResolvedBindGroupDescriptor<'a> = BindGroupDescriptor<
    'a,
    Arc<BindGroupLayout>,
    Arc<Buffer>,
    Arc<Sampler>,
    Arc<TextureView>,
    Arc<Tlas>,
    Arc<ExternalTexture>,
>;

/// Describes a [`BindGroupLayout`].
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BindGroupLayoutDescriptor<'a> {
    /// Debug label of the bind group layout.
    ///
    /// This will show up in graphics debuggers for easy identification.
    pub label: Label<'a>,
    /// Array of entries in this BindGroupLayout
    pub entries: Cow<'a, [wgt::BindGroupLayoutEntry]>,
}

/// Used by [`BindGroupLayout`]. It indicates whether the BGL must be
/// used with a specific pipeline. This constraint only happens when
/// the BGLs have been derived from a pipeline without a layout.
#[derive(Clone, Debug)]
pub(crate) enum ExclusivePipeline {
    None,
    Render(Weak<RenderPipeline>),
    Compute(Weak<ComputePipeline>),
}

impl fmt::Display for ExclusivePipeline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExclusivePipeline::None => f.write_str("None"),
            ExclusivePipeline::Render(p) => {
                if let Some(p) = p.upgrade() {
                    p.error_ident().fmt(f)
                } else {
                    f.write_str("RenderPipeline")
                }
            }
            ExclusivePipeline::Compute(p) => {
                if let Some(p) = p.upgrade() {
                    p.error_ident().fmt(f)
                } else {
                    f.write_str("ComputePipeline")
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum RawBindGroupLayout {
    Owning(ManuallyDrop<Box<dyn hal::DynBindGroupLayout>>),
    /// The empty BGL was created by the device and will be destroyed by the device.
    RefDeviceEmptyBGL,
}

/// Bind group layout.
#[derive(Debug)]
pub struct BindGroupLayout {
    pub(crate) raw: RawBindGroupLayout,
    pub(crate) device: Arc<Device>,
    pub(crate) entries: bgl::EntryMap,
    /// It is very important that we know if the bind group comes from the BGL pool.
    ///
    /// If it does, then we need to remove it from the pool when we drop it.
    ///
    /// We cannot unconditionally remove from the pool, as BGLs that don't come from the pool
    /// (derived BGLs) must not be removed.
    pub(crate) origin: bgl::Origin,
    pub(crate) exclusive_pipeline: crate::OnceCellOrLock<ExclusivePipeline>,
    pub(crate) binding_count_validator: BindingTypeMaxCountValidator,
    /// The `label` from the descriptor used to create the resource.
    pub(crate) label: String,
}

impl Drop for BindGroupLayout {
    fn drop(&mut self) {
        resource_log!("Destroy raw {}", self.error_ident());
        if matches!(self.origin, bgl::Origin::Pool) {
            self.device.bgl_pool.remove(&self.entries);
        }
        match self.raw {
            RawBindGroupLayout::Owning(ref mut raw) => {
                // SAFETY: We are in the Drop impl and we don't use self.raw anymore after this point.
                let raw = unsafe { ManuallyDrop::take(raw) };
                unsafe {
                    self.device.raw().destroy_bind_group_layout(raw);
                }
            }
            RawBindGroupLayout::RefDeviceEmptyBGL => {}
        }
    }
}

crate::impl_resource_type!(BindGroupLayout);
crate::impl_labeled!(BindGroupLayout);
crate::impl_parent_device!(BindGroupLayout);
crate::impl_storage_item!(BindGroupLayout);

impl BindGroupLayout {
    pub(crate) fn raw(&self) -> &dyn hal::DynBindGroupLayout {
        match &self.raw {
            RawBindGroupLayout::Owning(raw) => raw.as_ref(),
            RawBindGroupLayout::RefDeviceEmptyBGL => self.device.empty_bgl.as_ref(),
        }
    }

    fn empty(device: &Arc<Device>) -> Arc<Self> {
        let exclusive_pipeline = crate::OnceCellOrLock::new();
        exclusive_pipeline.set(ExclusivePipeline::None).unwrap();
        Arc::new(Self {
            raw: RawBindGroupLayout::RefDeviceEmptyBGL,
            device: device.clone(),
            entries: bgl::EntryMap::default(),
            origin: bgl::Origin::Derived,
            exclusive_pipeline,
            binding_count_validator: BindingTypeMaxCountValidator::default(),
            label: String::new(),
        })
    }
}

#[derive(Clone, Debug, Error)]
#[non_exhaustive]
pub enum CreatePipelineLayoutError {
    #[error(transparent)]
    Device(#[from] DeviceError),
    #[error(
        "Immediate data has range bound {size} which is not aligned to IMMEDIATE_DATA_ALIGNMENT ({})",
        wgt::IMMEDIATE_DATA_ALIGNMENT
    )]
    MisalignedImmediateSize { size: u32 },
    #[error(transparent)]
    MissingFeatures(#[from] MissingFeatures),
    #[error(
        "Immediate data has size {size} which exceeds device immediate data size limit 0..{max}"
    )]
    ImmediateRangeTooLarge { size: u32, max: u32 },
    #[error(transparent)]
    TooManyBindings(BindingTypeMaxCountError),
    #[error("Bind group layout count {actual} exceeds device bind group limit {max}")]
    TooManyGroups { actual: usize, max: usize },
    #[error(transparent)]
    InvalidResource(#[from] InvalidResourceError),
    #[error("Bind group layout at index {index} has an exclusive pipeline: {pipeline}")]
    BglHasExclusivePipeline { index: usize, pipeline: String },
}

impl WebGpuError for CreatePipelineLayoutError {
    fn webgpu_error_type(&self) -> ErrorType {
        match self {
            Self::Device(e) => e.webgpu_error_type(),
            Self::MissingFeatures(e) => e.webgpu_error_type(),
            Self::InvalidResource(e) => e.webgpu_error_type(),
            Self::TooManyBindings(e) => e.webgpu_error_type(),
            Self::MisalignedImmediateSize { .. }
            | Self::ImmediateRangeTooLarge { .. }
            | Self::TooManyGroups { .. }
            | Self::BglHasExclusivePipeline { .. } => ErrorType::Validation,
        }
    }
}

#[derive(Clone, Debug, Error)]
#[non_exhaustive]
pub enum ImmediateUploadError {
    #[error(
        "Start offset {start_offset} overruns the immediate data range with a size of {immediate_size}"
    )]
    StartOffsetOverrun {
        start_offset: u32,
        immediate_size: u32,
    },
    #[error(
        "Provided immediate data start offset {0} does not respect \
        `IMMEDIATE_DATA_ALIGNMENT` ({ida})",
        ida = wgt::IMMEDIATE_DATA_ALIGNMENT
    )]
    StartOffsetUnaligned(u32),
    #[error(
        "Provided immediate data byte size {0} does not respect \
        `IMMEDIATE_DATA_ALIGNMENT` ({ida})",
        ida = wgt::IMMEDIATE_DATA_ALIGNMENT
    )]
    SizeUnaligned(u32),
    #[error(
        "Provided immediate data start offset {} + size {} overruns the immediate data range \
        with a size of {}",
        start_offset,
        size,
        immediate_size
    )]
    EndOffsetOverrun {
        start_offset: u32,
        size: u32,
        immediate_size: u32,
    },
    #[error("Start index {start_index} overruns the value data range with {data_size} element(s)")]
    ValueStartIndexOverrun { start_index: u32, data_size: usize },
    #[error(
        "Start index {} + count of {} overruns the value data range \
        with {} element(s)",
        start_index,
        count,
        data_size
    )]
    ValueEndIndexOverrun {
        start_index: u32,
        count: u32,
        data_size: usize,
    },
}

impl WebGpuError for ImmediateUploadError {
    fn webgpu_error_type(&self) -> ErrorType {
        ErrorType::Validation
    }
}

/// Describes a pipeline layout.
///
/// A `PipelineLayoutDescriptor` can be used to create a pipeline layout.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(bound = "BGL: Serialize"))]
pub struct PipelineLayoutDescriptor<'a, BGL = BindGroupLayoutId>
where
    [Option<BGL>]: ToOwned,
    <[Option<BGL>] as ToOwned>::Owned: fmt::Debug,
{
    /// Debug label of the pipeline layout.
    ///
    /// This will show up in graphics debuggers for easy identification.
    pub label: Label<'a>,
    /// Bind groups that this pipeline uses. The first entry will provide all the bindings for
    /// "set = 0", second entry will provide all the bindings for "set = 1" etc.
    #[cfg_attr(
        feature = "serde",
        serde(bound(deserialize = "<[Option<BGL>] as ToOwned>::Owned: Deserialize<'de>"))
    )]
    pub bind_group_layouts: Cow<'a, [Option<BGL>]>,
    /// The number of bytes of immediate data that are allocated for use
    /// in the shader. The `var<immediate>`s in the shader attached to
    /// this pipeline must be equal or smaller than this size.
    ///
    /// If this value is non-zero, [`wgt::Features::IMMEDIATES`] must be enabled.
    pub immediate_size: u32,
}

/// cbindgen:ignore
pub type ResolvedPipelineLayoutDescriptor<'a, BGL = Arc<BindGroupLayout>> =
    PipelineLayoutDescriptor<'a, BGL>;

#[derive(Debug)]
pub struct PipelineLayout {
    pub(crate) raw: ManuallyDrop<Box<dyn hal::DynPipelineLayout>>,
    pub(crate) device: Arc<Device>,
    /// The `label` from the descriptor used to create the resource.
    pub(crate) label: String,
    pub(crate) bind_group_layouts: ArrayVec<Option<Arc<BindGroupLayout>>, { hal::MAX_BIND_GROUPS }>,
    pub(crate) immediate_size: u32,
}

impl Drop for PipelineLayout {
    fn drop(&mut self) {
        resource_log!("Destroy raw {}", self.error_ident());
        // SAFETY: We are in the Drop impl and we don't use self.raw anymore after this point.
        let raw = unsafe { ManuallyDrop::take(&mut self.raw) };
        unsafe {
            self.device.raw().destroy_pipeline_layout(raw);
        }
    }
}

impl PipelineLayout {
    pub(crate) fn raw(&self) -> &dyn hal::DynPipelineLayout {
        self.raw.as_ref()
    }

    pub fn get_bind_group_layout(
        self: &Arc<Self>,
        index: u32,
    ) -> Result<Arc<BindGroupLayout>, GetBindGroupLayoutError> {
        let max_bind_groups = self.device.limits.max_bind_groups;
        if index >= max_bind_groups {
            return Err(GetBindGroupLayoutError::IndexOutOfRange {
                index,
                max: max_bind_groups,
            });
        }
        Ok(self
            .bind_group_layouts
            .get(index as usize)
            .cloned()
            .flatten()
            .unwrap_or_else(|| BindGroupLayout::empty(&self.device)))
    }

    pub(crate) fn get_bgl_entry(
        &self,
        group: u32,
        binding: u32,
    ) -> Option<&wgt::BindGroupLayoutEntry> {
        let bgl = self.bind_group_layouts.get(group as usize)?;
        let bgl = bgl.as_ref()?;
        bgl.entries.get(binding)
    }

    /// Validate immediates match up with expected ranges.
    pub(crate) fn validate_immediates_ranges(
        &self,
        offset: u32,
        size_bytes: u32,
    ) -> Result<(), ImmediateUploadError> {
        // Don't need to validate size against the immediate data size limit here,
        // as immediate data ranges are already validated to be within bounds,
        // and we validate that they are within the ranges.

        if !offset.is_multiple_of(wgt::IMMEDIATE_DATA_ALIGNMENT) {
            return Err(ImmediateUploadError::StartOffsetUnaligned(offset));
        }

        if !size_bytes.is_multiple_of(wgt::IMMEDIATE_DATA_ALIGNMENT) {
            return Err(ImmediateUploadError::SizeUnaligned(offset));
        }

        if offset > self.immediate_size {
            return Err(ImmediateUploadError::StartOffsetOverrun {
                start_offset: offset,
                immediate_size: self.immediate_size,
            });
        }

        if size_bytes > self.immediate_size - offset {
            return Err(ImmediateUploadError::EndOffsetOverrun {
                start_offset: offset,
                size: size_bytes,
                immediate_size: self.immediate_size,
            });
        }

        Ok(())
    }
}

crate::impl_resource_type!(PipelineLayout);
crate::impl_labeled!(PipelineLayout);
crate::impl_parent_device!(PipelineLayout);
crate::impl_storage_item!(PipelineLayout);

#[repr(C)]
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BufferBinding<B = BufferId> {
    pub buffer: B,
    pub offset: wgt::BufferAddress,

    /// Size of the binding. If `None`, the binding spans from `offset` to the
    /// end of the buffer.
    ///
    /// We use `BufferAddress` to allow a size of zero on this `wgpu_core` type,
    /// because JavaScript bindings cannot readily express `Option<NonZeroU64>`.
    /// The `wgpu` API uses `Option<BufferSize>` (i.e. `NonZeroU64`) for this
    /// field.
    pub size: Option<wgt::BufferAddress>,
}

pub type ResolvedBufferBinding = BufferBinding<Arc<Buffer>>;

// Note: Duplicated in `wgpu-rs` as `BindingResource`
// They're different enough that it doesn't make sense to share a common type
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BindingResource<
    'a,
    B = BufferId,
    S = SamplerId,
    TV = TextureViewId,
    TLAS = TlasId,
    ET = ExternalTextureId,
> where
    [BufferBinding<B>]: ToOwned,
    [S]: ToOwned,
    [TV]: ToOwned,
    [TLAS]: ToOwned,
    <[BufferBinding<B>] as ToOwned>::Owned: fmt::Debug,
    <[S] as ToOwned>::Owned: fmt::Debug,
    <[TV] as ToOwned>::Owned: fmt::Debug,
    <[TLAS] as ToOwned>::Owned: fmt::Debug,
{
    Buffer(BufferBinding<B>),
    #[cfg_attr(
        feature = "serde",
        serde(bound(deserialize = "<[BufferBinding<B>] as ToOwned>::Owned: Deserialize<'de>"))
    )]
    BufferArray(Cow<'a, [BufferBinding<B>]>),
    Sampler(S),
    #[cfg_attr(
        feature = "serde",
        serde(bound(deserialize = "<[S] as ToOwned>::Owned: Deserialize<'de>"))
    )]
    SamplerArray(Cow<'a, [S]>),
    TextureView(TV),
    #[cfg_attr(
        feature = "serde",
        serde(bound(deserialize = "<[TV] as ToOwned>::Owned: Deserialize<'de>"))
    )]
    TextureViewArray(Cow<'a, [TV]>),
    AccelerationStructure(TLAS),
    #[cfg_attr(
        feature = "serde",
        serde(bound(deserialize = "<[TLAS] as ToOwned>::Owned: Deserialize<'de>"))
    )]
    AccelerationStructureArray(Cow<'a, [TLAS]>),
    ExternalTexture(ET),
}

pub type ResolvedBindingResource<'a> = BindingResource<
    'a,
    Arc<Buffer>,
    Arc<Sampler>,
    Arc<TextureView>,
    Arc<Tlas>,
    Arc<ExternalTexture>,
>;

#[derive(Clone, Debug, Error)]
#[non_exhaustive]
pub enum BindError {
    #[error(
        "Dynamic offsets not expected with null bind group at index {group}. However {actual} dynamic offset{s1} were provided.",
        s1 = if *.actual >= 2 { "s" } else { "" },
    )]
    DynamicOffsetCountNotZero { group: u32, actual: usize },
    #[error(
        "{bind_group} {group} expects {expected} dynamic offset{s0}. However {actual} dynamic offset{s1} were provided.",
        s0 = if *.expected >= 2 { "s" } else { "" },
        s1 = if *.actual >= 2 { "s" } else { "" },
    )]
    MismatchedDynamicOffsetCount {
        bind_group: ResourceErrorIdent,
        group: u32,
        actual: usize,
        expected: usize,
    },
    #[error(
        "Dynamic binding index {idx} (targeting {bind_group} {group}, binding {binding}) with value {offset}, does not respect device's requested `{limit_name}` limit: {alignment}"
    )]
    UnalignedDynamicBinding {
        bind_group: ResourceErrorIdent,
        idx: usize,
        group: u32,
        binding: u32,
        offset: u32,
        alignment: u32,
        limit_name: &'static str,
    },
    #[error(
        "Dynamic binding offset index {idx} with offset {offset} would overrun the buffer bound to {bind_group} {group} -> binding {binding}. \
         Buffer size is {buffer_size} bytes, the binding binds bytes {binding_range:?}, meaning the maximum the binding can be offset is {maximum_dynamic_offset} bytes",
    )]
    DynamicBindingOutOfBounds {
        bind_group: ResourceErrorIdent,
        idx: usize,
        group: u32,
        binding: u32,
        offset: u32,
        buffer_size: wgt::BufferAddress,
        binding_range: Range<wgt::BufferAddress>,
        maximum_dynamic_offset: wgt::BufferAddress,
    },
}

impl WebGpuError for BindError {
    fn webgpu_error_type(&self) -> ErrorType {
        ErrorType::Validation
    }
}

#[derive(Debug)]
pub struct BindGroupDynamicBindingData {
    /// The index of the binding.
    ///
    /// Used for more descriptive errors.
    pub(crate) binding_idx: u32,
    /// The size of the buffer.
    ///
    /// Used for more descriptive errors.
    pub(crate) buffer_size: wgt::BufferAddress,
    /// The range that the binding covers.
    ///
    /// Used for more descriptive errors.
    pub(crate) binding_range: Range<wgt::BufferAddress>,
    /// The maximum value the dynamic offset can have before running off the end of the buffer.
    pub(crate) maximum_dynamic_offset: wgt::BufferAddress,
    /// The binding type.
    pub(crate) binding_type: wgt::BufferBindingType,
}

pub(crate) fn buffer_binding_type_alignment(
    limits: &wgt::Limits,
    binding_type: wgt::BufferBindingType,
) -> (u32, &'static str) {
    match binding_type {
        wgt::BufferBindingType::Uniform => (
            limits.min_uniform_buffer_offset_alignment,
            "min_uniform_buffer_offset_alignment",
        ),
        wgt::BufferBindingType::Storage { .. } => (
            limits.min_storage_buffer_offset_alignment,
            "min_storage_buffer_offset_alignment",
        ),
    }
}

pub(crate) fn buffer_binding_type_bounds_check_alignment(
    alignments: &hal::Alignments,
    binding_type: wgt::BufferBindingType,
) -> wgt::BufferAddress {
    match binding_type {
        wgt::BufferBindingType::Uniform => alignments.uniform_bounds_check_alignment.get(),
        wgt::BufferBindingType::Storage { .. } => wgt::COPY_BUFFER_ALIGNMENT,
    }
}

#[derive(Debug)]
pub(crate) struct BindGroupLateBufferBindingInfo {
    /// The normal binding index in the bind group.
    pub binding_index: u32,
    /// The size that exists at bind time.
    pub size: wgt::BufferSize,
}

#[derive(Debug)]
pub struct BindGroup {
    pub(crate) raw: Snatchable<Box<dyn hal::DynBindGroup>>,
    pub(crate) device: Arc<Device>,
    pub(crate) layout: Arc<BindGroupLayout>,
    /// The `label` from the descriptor used to create the resource.
    pub(crate) label: String,
    pub(crate) tracking_data: TrackingData,
    pub(crate) used: BindGroupStates,
    pub(crate) used_buffer_ranges: Vec<BufferInitTrackerAction>,
    pub(crate) used_texture_ranges: Vec<TextureInitTrackerAction>,
    pub(crate) dynamic_binding_info: Vec<BindGroupDynamicBindingData>,
    /// Actual binding sizes for buffers that don't have `min_binding_size`
    /// specified in BGL. Listed in the order of iteration of `BGL.entries`.
    pub(crate) late_buffer_binding_infos: Vec<BindGroupLateBufferBindingInfo>,
}

impl Drop for BindGroup {
    fn drop(&mut self) {
        if let Some(raw) = self.raw.take() {
            resource_log!("Destroy raw {}", self.error_ident());
            unsafe {
                self.device.raw().destroy_bind_group(raw);
            }
        }
    }
}

impl BindGroup {
    pub(crate) fn try_raw<'a>(
        &'a self,
        guard: &'a SnatchGuard,
    ) -> Result<&'a dyn hal::DynBindGroup, DestroyedResourceError> {
        // Clippy insist on writing it this way. The idea is to return None
        // if any of the raw buffer is not valid anymore.
        for buffer in &self.used_buffer_ranges {
            buffer.buffer.try_raw(guard)?;
        }
        for texture in &self.used_texture_ranges {
            texture.texture.try_raw(guard)?;
        }

        self.raw
            .get(guard)
            .map(|raw| raw.as_ref())
            .ok_or_else(|| DestroyedResourceError(self.error_ident()))
    }

    pub(crate) fn validate_dynamic_bindings(
        &self,
        bind_group_index: u32,
        offsets: &[wgt::DynamicOffset],
    ) -> Result<(), BindError> {
        if self.dynamic_binding_info.len() != offsets.len() {
            return Err(BindError::MismatchedDynamicOffsetCount {
                bind_group: self.error_ident(),
                group: bind_group_index,
                expected: self.dynamic_binding_info.len(),
                actual: offsets.len(),
            });
        }

        for (idx, (info, &offset)) in self
            .dynamic_binding_info
            .iter()
            .zip(offsets.iter())
            .enumerate()
        {
            let (alignment, limit_name) =
                buffer_binding_type_alignment(&self.device.limits, info.binding_type);
            if !(offset as wgt::BufferAddress).is_multiple_of(alignment as u64) {
                return Err(BindError::UnalignedDynamicBinding {
                    bind_group: self.error_ident(),
                    group: bind_group_index,
                    binding: info.binding_idx,
                    idx,
                    offset,
                    alignment,
                    limit_name,
                });
            }

            if offset as wgt::BufferAddress > info.maximum_dynamic_offset {
                return Err(BindError::DynamicBindingOutOfBounds {
                    bind_group: self.error_ident(),
                    group: bind_group_index,
                    binding: info.binding_idx,
                    idx,
                    offset,
                    buffer_size: info.buffer_size,
                    binding_range: info.binding_range.clone(),
                    maximum_dynamic_offset: info.maximum_dynamic_offset,
                });
            }
        }

        Ok(())
    }
}

crate::impl_resource_type!(BindGroup);
crate::impl_labeled!(BindGroup);
crate::impl_parent_device!(BindGroup);
crate::impl_storage_item!(BindGroup);
crate::impl_trackable!(BindGroup);

#[derive(Clone, Debug, Error)]
#[non_exhaustive]
pub enum GetBindGroupLayoutError {
    #[error("Bind group layout index {index} is greater than the device's configured `max_bind_groups` limit {max}")]
    IndexOutOfRange { index: u32, max: u32 },
    #[error(transparent)]
    InvalidResource(#[from] InvalidResourceError),
}

impl WebGpuError for GetBindGroupLayoutError {
    fn webgpu_error_type(&self) -> ErrorType {
        match self {
            Self::IndexOutOfRange { .. } => ErrorType::Validation,
            Self::InvalidResource(e) => e.webgpu_error_type(),
        }
    }
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
#[error(
    "In bind group index {group_index}, the buffer bound at binding index {binding_index} \
     is bound with size {bound_size} where the shader expects {shader_size}."
)]
pub struct LateMinBufferBindingSizeMismatch {
    pub group_index: u32,
    pub binding_index: u32,
    pub shader_size: wgt::BufferAddress,
    pub bound_size: wgt::BufferAddress,
}
