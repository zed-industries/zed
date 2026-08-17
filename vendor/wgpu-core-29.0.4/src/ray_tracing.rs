// Ray tracing
// Major missing optimizations (no api surface changes needed):
// - use custom tracker to track build state
// - no forced rebuilt (build mode deduction)
// - lazy instance buffer allocation
// - maybe share scratch and instance staging buffer allocation
// - partial instance buffer uploads (api surface already designed with this in mind)
// - Batch BLAS read-backs (if it shows up in performance).
// - ([non performance] extract function in build (rust function extraction with guards is a pain))

use alloc::{boxed::Box, sync::Arc, vec::Vec};

#[cfg(feature = "serde")]
use macro_rules_attribute::apply;
use thiserror::Error;
use wgt::{
    error::{ErrorType, WebGpuError},
    AccelerationStructureGeometryFlags, BufferAddress, IndexFormat, VertexFormat,
};

#[cfg(feature = "serde")]
use crate::command::serde_object_reference_struct;
use crate::{
    command::{ArcReferences, EncoderStateError, IdReferences, ReferenceType},
    device::{DeviceError, MissingFeatures},
    id::{BlasId, BufferId, TlasId},
    resource::{
        Blas, BlasCompactCallback, BlasPrepareCompactResult, DestroyedResourceError,
        InvalidResourceError, MissingBufferUsageError, ResourceErrorIdent, Tlas,
    },
};

#[derive(Clone, Debug, Error)]
pub enum CreateBlasError {
    #[error(transparent)]
    Device(#[from] DeviceError),
    #[error(transparent)]
    MissingFeatures(#[from] MissingFeatures),
    #[error(
        "Only one of 'index_count' and 'index_format' was provided (either provide both or none)"
    )]
    MissingIndexData,
    #[error("Provided format was not within allowed formats. Provided format: {0:?}. Allowed formats: {1:?}")]
    InvalidVertexFormat(VertexFormat, Vec<VertexFormat>),
    #[error("Limit `max_blas_geometry_count` is {0}, but the BLAS had {1} geometries")]
    TooManyGeometries(u32, u32),
    #[error(
        "Limit `max_blas_primitive_count` is {0}, but the BLAS had a maximum of {1} primitives"
    )]
    TooManyPrimitives(u32, u32),
}

impl WebGpuError for CreateBlasError {
    fn webgpu_error_type(&self) -> ErrorType {
        match self {
            Self::Device(e) => e.webgpu_error_type(),
            Self::MissingFeatures(e) => e.webgpu_error_type(),
            Self::MissingIndexData
            | Self::InvalidVertexFormat(..)
            | Self::TooManyGeometries(..)
            | Self::TooManyPrimitives(..) => ErrorType::Validation,
        }
    }
}

#[derive(Clone, Debug, Error)]
pub enum CreateTlasError {
    #[error(transparent)]
    Device(#[from] DeviceError),
    #[error(transparent)]
    MissingFeatures(#[from] MissingFeatures),
    #[error("Flag {0:?} is not allowed on a TLAS")]
    DisallowedFlag(wgt::AccelerationStructureFlags),
    #[error("Limit `max_tlas_instance_count` is {0}, but the TLAS had a maximum of {1} instances")]
    TooManyInstances(u32, u32),
}

impl WebGpuError for CreateTlasError {
    fn webgpu_error_type(&self) -> ErrorType {
        match self {
            Self::Device(e) => e.webgpu_error_type(),
            Self::MissingFeatures(e) => e.webgpu_error_type(),
            Self::DisallowedFlag(..) | Self::TooManyInstances(..) => ErrorType::Validation,
        }
    }
}

/// Error encountered while attempting to do a copy on a command encoder.
#[derive(Clone, Debug, Error)]
pub enum BuildAccelerationStructureError {
    #[error(transparent)]
    EncoderState(#[from] EncoderStateError),

    #[error(transparent)]
    Device(#[from] DeviceError),

    #[error(transparent)]
    InvalidResource(#[from] InvalidResourceError),

    #[error(transparent)]
    DestroyedResource(#[from] DestroyedResourceError),

    #[error(transparent)]
    MissingBufferUsage(#[from] MissingBufferUsageError),

    #[error(transparent)]
    MissingFeatures(#[from] MissingFeatures),

    #[error(
        "Buffer {0:?} size is insufficient for provided size information (size: {1}, required: {2}"
    )]
    InsufficientBufferSize(ResourceErrorIdent, u64, u64),

    #[error("Buffer {0:?} associated offset doesn't align with the index type")]
    UnalignedIndexBufferOffset(ResourceErrorIdent),

    #[error("Buffer {0:?} associated offset is unaligned")]
    UnalignedTransformBufferOffset(ResourceErrorIdent),

    #[error("Buffer {0:?} associated index count not divisible by 3 (count: {1}")]
    InvalidIndexCount(ResourceErrorIdent, u32),

    #[error("Buffer {0:?} associated data contains None")]
    MissingAssociatedData(ResourceErrorIdent),

    #[error(
        "Blas {0:?} build sizes to may be greater than the descriptor at build time specified"
    )]
    IncompatibleBlasBuildSizes(ResourceErrorIdent),

    #[error("Blas {0:?} flags are different, creation flags: {1:?}, provided: {2:?}")]
    IncompatibleBlasFlags(
        ResourceErrorIdent,
        AccelerationStructureGeometryFlags,
        AccelerationStructureGeometryFlags,
    ),

    #[error("Blas {0:?} build vertex count is greater than creation count (needs to be less than or equal to), creation: {1:?}, build: {2:?}")]
    IncompatibleBlasVertexCount(ResourceErrorIdent, u32, u32),

    #[error("Blas {0:?} vertex formats are different, creation format: {1:?}, provided: {2:?}")]
    DifferentBlasVertexFormats(ResourceErrorIdent, VertexFormat, VertexFormat),

    #[error("Blas {0:?} stride was required to be at least {1} but stride given was {2}")]
    VertexStrideTooSmall(ResourceErrorIdent, u64, u64),

    #[error("Blas {0:?} stride was required to be a multiple of {1} but stride given was {2}")]
    VertexStrideUnaligned(ResourceErrorIdent, u64, u64),

    #[error("Blas {0:?} index count was provided at creation or building, but not the other")]
    BlasIndexCountProvidedMismatch(ResourceErrorIdent),

    #[error("Blas {0:?} build index count is greater than creation count (needs to be less than or equal to), creation: {1:?}, build: {2:?}")]
    IncompatibleBlasIndexCount(ResourceErrorIdent, u32, u32),

    #[error("Blas {0:?} index formats are different, creation format: {1:?}, provided: {2:?}")]
    DifferentBlasIndexFormats(ResourceErrorIdent, Option<IndexFormat>, Option<IndexFormat>),

    #[error("Blas {0:?} is compacted and so cannot be built")]
    CompactedBlas(ResourceErrorIdent),

    #[error("Blas {0:?} build sizes require index buffer but none was provided")]
    MissingIndexBuffer(ResourceErrorIdent),

    #[error(
        "Tlas {0:?} an associated instances contains an invalid custom index (more than 24bits)"
    )]
    TlasInvalidCustomIndex(ResourceErrorIdent),

    #[error(
        "Tlas {0:?} has {1} active instances but only {2} are allowed as specified by the descriptor at creation"
    )]
    TlasInstanceCountExceeded(ResourceErrorIdent, u32, u32),

    #[error("Blas {0:?} has flag USE_TRANSFORM but the transform buffer is missing")]
    TransformMissing(ResourceErrorIdent),

    #[error("Blas {0:?} is missing the flag USE_TRANSFORM but the transform buffer is set")]
    UseTransformMissing(ResourceErrorIdent),
    #[error(
        "Tlas {0:?} dependent {1:?} is missing AccelerationStructureFlags::ALLOW_RAY_HIT_VERTEX_RETURN"
    )]
    TlasDependentMissingVertexReturn(ResourceErrorIdent, ResourceErrorIdent),
}

impl WebGpuError for BuildAccelerationStructureError {
    fn webgpu_error_type(&self) -> ErrorType {
        match self {
            Self::EncoderState(e) => e.webgpu_error_type(),
            Self::Device(e) => e.webgpu_error_type(),
            Self::InvalidResource(e) => e.webgpu_error_type(),
            Self::DestroyedResource(e) => e.webgpu_error_type(),
            Self::MissingBufferUsage(e) => e.webgpu_error_type(),
            Self::MissingFeatures(e) => e.webgpu_error_type(),
            Self::InsufficientBufferSize(..)
            | Self::UnalignedIndexBufferOffset(..)
            | Self::UnalignedTransformBufferOffset(..)
            | Self::InvalidIndexCount(..)
            | Self::MissingAssociatedData(..)
            | Self::IncompatibleBlasBuildSizes(..)
            | Self::IncompatibleBlasFlags(..)
            | Self::IncompatibleBlasVertexCount(..)
            | Self::DifferentBlasVertexFormats(..)
            | Self::VertexStrideTooSmall(..)
            | Self::VertexStrideUnaligned(..)
            | Self::BlasIndexCountProvidedMismatch(..)
            | Self::IncompatibleBlasIndexCount(..)
            | Self::DifferentBlasIndexFormats(..)
            | Self::CompactedBlas(..)
            | Self::MissingIndexBuffer(..)
            | Self::TlasInvalidCustomIndex(..)
            | Self::TlasInstanceCountExceeded(..)
            | Self::TransformMissing(..)
            | Self::UseTransformMissing(..)
            | Self::TlasDependentMissingVertexReturn(..) => ErrorType::Validation,
        }
    }
}

#[derive(Clone, Debug, Error)]
pub enum ValidateAsActionsError {
    #[error(transparent)]
    DestroyedResource(#[from] DestroyedResourceError),

    #[error("Tlas {0:?} is used before it is built")]
    UsedUnbuiltTlas(ResourceErrorIdent),

    #[error("Blas {0:?} is used before it is built (in Tlas {1:?})")]
    UsedUnbuiltBlas(ResourceErrorIdent, ResourceErrorIdent),

    #[error("Blas {0:?} is newer than the containing Tlas {1:?}")]
    BlasNewerThenTlas(ResourceErrorIdent, ResourceErrorIdent),
}

impl WebGpuError for ValidateAsActionsError {
    fn webgpu_error_type(&self) -> ErrorType {
        match self {
            Self::DestroyedResource(e) => e.webgpu_error_type(),
            Self::UsedUnbuiltTlas(..) | Self::UsedUnbuiltBlas(..) | Self::BlasNewerThenTlas(..) => {
                ErrorType::Validation
            }
        }
    }
}

#[derive(Debug)]
pub struct BlasTriangleGeometry<'a> {
    pub size: &'a wgt::BlasTriangleGeometrySizeDescriptor,
    pub vertex_buffer: BufferId,
    pub index_buffer: Option<BufferId>,
    pub transform_buffer: Option<BufferId>,
    pub first_vertex: u32,
    pub vertex_stride: BufferAddress,
    pub first_index: Option<u32>,
    pub transform_buffer_offset: Option<BufferAddress>,
}

pub enum BlasGeometries<'a> {
    TriangleGeometries(Box<dyn Iterator<Item = BlasTriangleGeometry<'a>> + 'a>),
}

pub struct BlasBuildEntry<'a> {
    pub blas_id: BlasId,
    pub geometries: BlasGeometries<'a>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TlasBuildEntry {
    pub tlas_id: TlasId,
    pub instance_buffer_id: BufferId,
    pub instance_count: u32,
}

#[derive(Debug)]
pub struct TlasInstance<'a> {
    pub blas_id: BlasId,
    pub transform: &'a [f32; 12],
    pub custom_data: u32,
    pub mask: u8,
}

pub struct TlasPackage<'a> {
    pub tlas_id: TlasId,
    pub instances: Box<dyn Iterator<Item = Option<TlasInstance<'a>>> + 'a>,
    pub lowest_unmodified: u32,
}

#[derive(Debug, Clone)]
pub(crate) struct TlasBuild {
    pub tlas: Arc<Tlas>,
    pub dependencies: Vec<Arc<Blas>>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct AsBuild {
    pub blas_s_built: Vec<Arc<Blas>>,
    pub tlas_s_built: Vec<TlasBuild>,
}

impl AsBuild {
    pub(crate) fn with_capacity(blas: usize, tlas: usize) -> Self {
        Self {
            blas_s_built: Vec::with_capacity(blas),
            tlas_s_built: Vec::with_capacity(tlas),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum AsAction {
    Build(AsBuild),
    UseTlas(Arc<Tlas>),
}

/// Like [`BlasTriangleGeometry`], but with owned data.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", apply(serde_object_reference_struct))]
pub struct OwnedBlasTriangleGeometry<R: ReferenceType> {
    pub size: wgt::BlasTriangleGeometrySizeDescriptor,
    pub vertex_buffer: R::Buffer,
    pub index_buffer: Option<R::Buffer>,
    pub transform_buffer: Option<R::Buffer>,
    pub first_vertex: u32,
    pub vertex_stride: BufferAddress,
    pub first_index: Option<u32>,
    pub transform_buffer_offset: Option<BufferAddress>,
}

pub type ArcBlasTriangleGeometry = OwnedBlasTriangleGeometry<ArcReferences>;
pub type TraceBlasTriangleGeometry = OwnedBlasTriangleGeometry<IdReferences>;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", apply(serde_object_reference_struct))]
pub enum OwnedBlasGeometries<R: ReferenceType> {
    TriangleGeometries(Vec<OwnedBlasTriangleGeometry<R>>),
}

pub type ArcBlasGeometries = OwnedBlasGeometries<ArcReferences>;
pub type TraceBlasGeometries = OwnedBlasGeometries<IdReferences>;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", apply(serde_object_reference_struct))]
pub struct OwnedBlasBuildEntry<R: ReferenceType> {
    pub blas: R::Blas,
    pub geometries: OwnedBlasGeometries<R>,
}

pub type ArcBlasBuildEntry = OwnedBlasBuildEntry<ArcReferences>;
pub type TraceBlasBuildEntry = OwnedBlasBuildEntry<IdReferences>;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", apply(serde_object_reference_struct))]
pub struct OwnedTlasInstance<R: ReferenceType> {
    pub blas: R::Blas,
    pub transform: [f32; 12],
    pub custom_data: u32,
    pub mask: u8,
}

pub type ArcTlasInstance = OwnedTlasInstance<ArcReferences>;
pub type TraceTlasInstance = OwnedTlasInstance<IdReferences>;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", apply(serde_object_reference_struct))]
pub struct OwnedTlasPackage<R: ReferenceType> {
    pub tlas: R::Tlas,
    pub instances: Vec<Option<OwnedTlasInstance<R>>>,
    pub lowest_unmodified: u32,
}

pub type TraceTlasPackage = OwnedTlasPackage<IdReferences>;
pub type ArcTlasPackage = OwnedTlasPackage<ArcReferences>;

/// [`BlasTriangleGeometry`], without the resources.
#[derive(Debug, Clone)]
pub struct BlasTriangleGeometryInfo {
    pub size: wgt::BlasTriangleGeometrySizeDescriptor,
    pub first_vertex: u32,
    pub vertex_stride: BufferAddress,
    pub first_index: Option<u32>,
    pub transform_buffer_offset: Option<BufferAddress>,
}

#[derive(Clone, Debug, Error)]
pub enum BlasPrepareCompactError {
    #[error(transparent)]
    Device(#[from] DeviceError),
    #[error(transparent)]
    InvalidResource(#[from] InvalidResourceError),
    #[error("Compaction is already being prepared")]
    CompactionPreparingAlready,
    #[error("Cannot compact an already compacted BLAS")]
    DoubleCompaction,
    #[error("BLAS is not yet built")]
    NotBuilt,
    #[error("BLAS does not support compaction (is AccelerationStructureFlags::ALLOW_COMPACTION missing?)")]
    CompactionUnsupported,
}

impl WebGpuError for BlasPrepareCompactError {
    fn webgpu_error_type(&self) -> ErrorType {
        match self {
            Self::Device(e) => e.webgpu_error_type(),
            Self::InvalidResource(e) => e.webgpu_error_type(),
            Self::CompactionPreparingAlready
            | Self::DoubleCompaction
            | Self::NotBuilt
            | Self::CompactionUnsupported => ErrorType::Validation,
        }
    }
}

#[derive(Clone, Debug, Error)]
pub enum CompactBlasError {
    #[error(transparent)]
    Encoder(#[from] EncoderStateError),

    #[error(transparent)]
    Device(#[from] DeviceError),

    #[error(transparent)]
    InvalidResource(#[from] InvalidResourceError),

    #[error(transparent)]
    DestroyedResource(#[from] DestroyedResourceError),

    #[error(transparent)]
    MissingFeatures(#[from] MissingFeatures),

    #[error("BLAS was not prepared for compaction")]
    BlasNotReady,
}

impl WebGpuError for CompactBlasError {
    fn webgpu_error_type(&self) -> ErrorType {
        match self {
            Self::Encoder(e) => e.webgpu_error_type(),
            Self::Device(e) => e.webgpu_error_type(),
            Self::InvalidResource(e) => e.webgpu_error_type(),
            Self::DestroyedResource(e) => e.webgpu_error_type(),
            Self::MissingFeatures(e) => e.webgpu_error_type(),
            Self::BlasNotReady => ErrorType::Validation,
        }
    }
}

pub type BlasCompactReadyPendingClosure = (Option<BlasCompactCallback>, BlasPrepareCompactResult);
