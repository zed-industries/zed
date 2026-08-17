use alloc::{borrow::Cow, sync::Arc, vec::Vec};
use core::{convert::Infallible, fmt, num::NonZeroU32, ops::Range, str};
use smallvec::SmallVec;

use arrayvec::ArrayVec;
use thiserror::Error;
use wgt::{
    error::{ErrorType, WebGpuError},
    BufferAddress, BufferSize, BufferUsages, Color, DynamicOffset, IndexFormat, TextureSelector,
    TextureUsages, TextureViewDimension, VertexStepMode,
};

use crate::{
    api_log,
    binding_model::{BindError, ImmediateUploadError},
    command::{
        bind::Binder,
        memory_init::{fixup_discarded_surfaces, SurfacesInDiscardState, TextureSurfaceDiscard},
        pass::{self, flush_bindings_helper},
        pass_base, pass_try,
        query::{
            end_occlusion_query, end_pipeline_statistics_query, validate_and_begin_occlusion_query,
            validate_and_begin_pipeline_statistics_query, QueryResetMap,
        },
        render_command::ArcRenderCommand,
        ArcCommand, ArcPassTimestampWrites, BasePass, BindGroupStateChange,
        CommandBufferTextureMemoryActions, CommandEncoder, CommandEncoderError, DebugGroupError,
        DrawCommandFamily, DrawError, DrawKind, EncoderStateError, EncodingState, ExecutionError,
        InnerCommandEncoder, MapPassErr, PassErrorScope, PassStateError, PassTimestampWrites,
        QueryUseError, Rect, RenderCommandError, StateChange, TimestampWritesError,
    },
    device::{
        AttachmentData, Device, DeviceError, MissingDownlevelFlags, MissingFeatures,
        RenderPassCompatibilityError, RenderPassContext,
    },
    global::Global,
    hal_label, id,
    init_tracker::{MemoryInitKind, TextureInitRange, TextureInitTrackerAction},
    pipeline::{PipelineFlags, RenderPipeline, VertexStep},
    resource::{
        DestroyedResourceError, InvalidResourceError, Labeled, MissingBufferUsageError,
        MissingTextureUsageError, ParentDevice, QuerySet, RawResourceAccess, ResourceErrorIdent,
        Texture, TextureView, TextureViewNotRenderableReason,
    },
    snatch::SnatchGuard,
    track::{ResourceUsageCompatibilityError, Tracker, UsageScope},
    validation, Label,
};

#[cfg(feature = "serde")]
use serde::Deserialize;
#[cfg(feature = "serde")]
use serde::Serialize;

pub use wgt::{LoadOp, StoreOp};

fn load_hal_ops<V>(load: LoadOp<V>) -> hal::AttachmentOps {
    match load {
        LoadOp::Load => hal::AttachmentOps::LOAD,
        LoadOp::Clear(_) => hal::AttachmentOps::LOAD_CLEAR,
        LoadOp::DontCare(_) => hal::AttachmentOps::LOAD_DONT_CARE,
    }
}

fn store_hal_ops(store: StoreOp) -> hal::AttachmentOps {
    match store {
        StoreOp::Store => hal::AttachmentOps::STORE,
        StoreOp::Discard => hal::AttachmentOps::STORE_DISCARD,
    }
}

/// Describes an individual channel within a render pass, such as color, depth, or stencil.
///
/// A channel must either be read-only, or it must specify both load and store
/// operations. See [`ResolvedPassChannel`] for a validated version.
#[repr(C)]
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PassChannel<V> {
    /// Operation to perform to the output attachment at the start of a
    /// renderpass.
    ///
    /// This must be clear if it is the first renderpass rendering to a swap
    /// chain image.
    pub load_op: Option<LoadOp<V>>,
    /// Operation to perform to the output attachment at the end of a renderpass.
    pub store_op: Option<StoreOp>,
    /// If true, the relevant channel is not changed by a renderpass, and the
    /// corresponding attachment can be used inside the pass by other read-only
    /// usages.
    pub read_only: bool,
}

impl<V: Copy + Default> PassChannel<Option<V>> {
    fn resolve(
        &self,
        handle_clear: impl Fn(Option<V>) -> Result<V, AttachmentError>,
    ) -> Result<ResolvedPassChannel<V>, AttachmentError> {
        if self.read_only {
            if self.load_op.is_some() {
                return Err(AttachmentError::ReadOnlyWithLoad);
            }
            if self.store_op.is_some() {
                return Err(AttachmentError::ReadOnlyWithStore);
            }
            Ok(ResolvedPassChannel::ReadOnly)
        } else {
            Ok(ResolvedPassChannel::Operational(wgt::Operations {
                load: match self.load_op.ok_or(AttachmentError::NoLoad)? {
                    LoadOp::Clear(clear_value) => LoadOp::Clear(handle_clear(clear_value)?),
                    LoadOp::DontCare(token) => LoadOp::DontCare(token),
                    LoadOp::Load => LoadOp::Load,
                },
                store: self.store_op.ok_or(AttachmentError::NoStore)?,
            }))
        }
    }
}

/// Describes an individual channel within a render pass, such as color, depth, or stencil.
///
/// Unlike [`PassChannel`], this version uses the Rust type system to guarantee
/// a valid specification.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ResolvedPassChannel<V> {
    ReadOnly,
    Operational(wgt::Operations<V>),
}

impl<V: Copy + Default> ResolvedPassChannel<V> {
    fn load_op(&self) -> LoadOp<V> {
        match self {
            ResolvedPassChannel::ReadOnly => LoadOp::Load,
            ResolvedPassChannel::Operational(wgt::Operations { load, .. }) => *load,
        }
    }

    fn store_op(&self) -> StoreOp {
        match self {
            ResolvedPassChannel::ReadOnly => StoreOp::Store,
            ResolvedPassChannel::Operational(wgt::Operations { store, .. }) => *store,
        }
    }

    fn clear_value(&self) -> V {
        match self {
            Self::Operational(wgt::Operations {
                load: LoadOp::Clear(clear_value),
                ..
            }) => *clear_value,
            _ => Default::default(),
        }
    }

    fn is_readonly(&self) -> bool {
        matches!(self, Self::ReadOnly)
    }

    fn hal_ops(&self) -> hal::AttachmentOps {
        load_hal_ops(self.load_op()) | store_hal_ops(self.store_op())
    }
}

/// Describes a color attachment to a render pass.
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RenderPassColorAttachment<TV = id::TextureViewId> {
    /// The view to use as an attachment.
    pub view: TV,
    /// The depth slice index of a 3D view. It must not be provided if the view is not 3D.
    pub depth_slice: Option<u32>,
    /// The view that will receive the resolved output if multisampling is used.
    pub resolve_target: Option<TV>,
    /// Operation to perform to the output attachment at the start of a
    /// renderpass.
    ///
    /// This must be clear if it is the first renderpass rendering to a swap
    /// chain image.
    pub load_op: LoadOp<Color>,
    /// Operation to perform to the output attachment at the end of a renderpass.
    pub store_op: StoreOp,
}

pub type ArcRenderPassColorAttachment = RenderPassColorAttachment<Arc<TextureView>>;

// Avoid allocation in the common case that there is only one color attachment,
// but don't bloat `ArcCommand::RunRenderPass` excessively.
pub type ColorAttachments<TV = Arc<TextureView>> =
    SmallVec<[Option<RenderPassColorAttachment<TV>>; 1]>;

impl ArcRenderPassColorAttachment {
    fn hal_ops(&self) -> hal::AttachmentOps {
        load_hal_ops(self.load_op) | store_hal_ops(self.store_op)
    }

    fn clear_value(&self) -> Color {
        match self.load_op {
            LoadOp::Clear(clear_value) => clear_value,
            LoadOp::DontCare(_) | LoadOp::Load => Color::default(),
        }
    }
}

/// Describes a depth/stencil attachment to a render pass.
///
/// This version uses the unvalidated [`PassChannel`].
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RenderPassDepthStencilAttachment<TV> {
    /// The view to use as an attachment.
    pub view: TV,
    /// What operations will be performed on the depth part of the attachment.
    pub depth: PassChannel<Option<f32>>,
    /// What operations will be performed on the stencil part of the attachment.
    pub stencil: PassChannel<Option<u32>>,
}

/// Describes a depth/stencil attachment to a render pass.
///
/// This version uses the validated [`ResolvedPassChannel`].
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ResolvedRenderPassDepthStencilAttachment<TV> {
    /// The view to use as an attachment.
    pub view: TV,
    /// What operations will be performed on the depth part of the attachment.
    pub depth: ResolvedPassChannel<f32>,
    /// What operations will be performed on the stencil part of the attachment.
    pub stencil: ResolvedPassChannel<u32>,
}

/// Describes the attachments of a render pass.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RenderPassDescriptor<'a> {
    pub label: Label<'a>,
    /// The color attachments of the render pass.
    pub color_attachments: Cow<'a, [Option<RenderPassColorAttachment>]>,
    /// The depth and stencil attachment of the render pass, if any.
    pub depth_stencil_attachment: Option<&'a RenderPassDepthStencilAttachment<id::TextureViewId>>,
    /// Defines where and when timestamp values will be written for this pass.
    pub timestamp_writes: Option<&'a PassTimestampWrites>,
    /// Defines where the occlusion query results will be stored for this pass.
    pub occlusion_query_set: Option<id::QuerySetId>,
    /// The multiview array layers that will be used
    pub multiview_mask: Option<NonZeroU32>,
}

/// Describes the attachments of a render pass.
struct ArcRenderPassDescriptor<'a> {
    pub label: &'a Label<'a>,
    /// The color attachments of the render pass.
    pub color_attachments:
        ArrayVec<Option<ArcRenderPassColorAttachment>, { hal::MAX_COLOR_ATTACHMENTS }>,
    /// The depth and stencil attachment of the render pass, if any.
    pub depth_stencil_attachment:
        Option<ResolvedRenderPassDepthStencilAttachment<Arc<TextureView>>>,
    /// Defines where and when timestamp values will be written for this pass.
    pub timestamp_writes: Option<ArcPassTimestampWrites>,
    /// Defines where the occlusion query results will be stored for this pass.
    pub occlusion_query_set: Option<Arc<QuerySet>>,
    /// The multiview array layers that will be used
    pub multiview_mask: Option<NonZeroU32>,
}

pub type RenderBasePass = BasePass<ArcRenderCommand, RenderPassError>;

/// A pass's [encoder state](https://www.w3.org/TR/webgpu/#encoder-state) and
/// its validity are two distinct conditions, i.e., the full matrix of
/// (open, ended) x (valid, invalid) is possible.
///
/// The presence or absence of the `parent` `Option` indicates the pass's state.
/// The presence or absence of an error in `base.error` indicates the pass's
/// validity.
pub struct RenderPass {
    /// All pass data & records is stored here.
    base: BasePass<ArcRenderCommand, RenderPassError>,

    /// Parent command encoder that this pass records commands into.
    ///
    /// If this is `Some`, then the pass is in WebGPU's "open" state. If it is
    /// `None`, then the pass is in the "ended" state.
    /// See <https://www.w3.org/TR/webgpu/#encoder-state>
    parent: Option<Arc<CommandEncoder>>,

    color_attachments:
        ArrayVec<Option<ArcRenderPassColorAttachment>, { hal::MAX_COLOR_ATTACHMENTS }>,
    depth_stencil_attachment: Option<ResolvedRenderPassDepthStencilAttachment<Arc<TextureView>>>,
    timestamp_writes: Option<ArcPassTimestampWrites>,
    occlusion_query_set: Option<Arc<QuerySet>>,
    multiview_mask: Option<NonZeroU32>,

    // Resource binding dedupe state.
    current_bind_groups: BindGroupStateChange,
    current_pipeline: StateChange<id::RenderPipelineId>,
}

impl RenderPass {
    /// If the parent command encoder is invalid, the returned pass will be invalid.
    fn new(parent: Arc<CommandEncoder>, desc: ArcRenderPassDescriptor) -> Self {
        let ArcRenderPassDescriptor {
            label,
            timestamp_writes,
            color_attachments,
            depth_stencil_attachment,
            occlusion_query_set,
            multiview_mask,
        } = desc;

        Self {
            base: BasePass::new(label),
            parent: Some(parent),
            color_attachments,
            depth_stencil_attachment,
            timestamp_writes,
            occlusion_query_set,
            multiview_mask,

            current_bind_groups: BindGroupStateChange::new(),
            current_pipeline: StateChange::new(),
        }
    }

    fn new_invalid(parent: Arc<CommandEncoder>, label: &Label, err: RenderPassError) -> Self {
        Self {
            base: BasePass::new_invalid(label, err),
            parent: Some(parent),
            color_attachments: ArrayVec::new(),
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
            current_bind_groups: BindGroupStateChange::new(),
            current_pipeline: StateChange::new(),
        }
    }

    #[inline]
    pub fn label(&self) -> Option<&str> {
        self.base.label.as_deref()
    }
}

impl fmt::Debug for RenderPass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RenderPass")
            .field("label", &self.label())
            .field("color_attachments", &self.color_attachments)
            .field("depth_stencil_target", &self.depth_stencil_attachment)
            .field("command count", &self.base.commands.len())
            .field("dynamic offset count", &self.base.dynamic_offsets.len())
            .field("immediate data u32 count", &self.base.immediates_data.len())
            .field("multiview mask", &self.multiview_mask)
            .finish()
    }
}

#[derive(Debug, PartialEq)]
enum OptionalState {
    Unused,
    Required,
    Set,
}

impl OptionalState {
    fn require(&mut self, require: bool) {
        if require && *self == Self::Unused {
            *self = Self::Required;
        }
    }
}

#[derive(Debug, Default)]
struct IndexState {
    buffer_format: Option<IndexFormat>,
    limit: u64,
}

impl IndexState {
    fn update_buffer(&mut self, range: Range<BufferAddress>, format: IndexFormat) {
        self.buffer_format = Some(format);
        let shift = match format {
            IndexFormat::Uint16 => 1,
            IndexFormat::Uint32 => 2,
        };
        self.limit = (range.end - range.start) >> shift;
    }

    fn reset(&mut self) {
        self.buffer_format = None;
        self.limit = 0;
    }
}

#[derive(Debug, Default)]
pub(crate) struct VertexLimits {
    /// Length of the shortest vertex rate vertex buffer
    pub(crate) vertex_limit: u64,
    /// Buffer slot which the shortest vertex rate vertex buffer is bound to
    vertex_limit_slot: u32,
    /// Length of the shortest instance rate vertex buffer
    pub(crate) instance_limit: u64,
    /// Buffer slot which the shortest instance rate vertex buffer is bound to
    instance_limit_slot: u32,
}

impl VertexLimits {
    pub(crate) fn new(
        buffer_sizes: impl Iterator<Item = Option<BufferAddress>>,
        pipeline_steps: &[VertexStep],
    ) -> Self {
        // Implements the validation from https://gpuweb.github.io/gpuweb/#dom-gpurendercommandsmixin-draw
        // Except that the formula is shuffled to extract the number of vertices in order
        // to carry the bulk of the computation when changing states instead of when producing
        // draws. Draw calls tend to happen at a higher frequency. Here we determine vertex
        // limits that can be cheaply checked for each draw call.

        let mut vertex_limit = u64::MAX;
        let mut vertex_limit_slot = 0;
        let mut instance_limit = u64::MAX;
        let mut instance_limit_slot = 0;

        for (idx, (buffer_size, step)) in buffer_sizes.zip(pipeline_steps).enumerate() {
            let Some(buffer_size) = buffer_size else {
                // Missing required vertex buffer
                return Self::default();
            };

            let limit = if buffer_size < step.last_stride {
                // The buffer cannot fit the last vertex.
                0
            } else {
                if step.stride == 0 {
                    // We already checked that the last stride fits, the same
                    // vertex will be repeated so this slot can accommodate any number of
                    // vertices.
                    continue;
                }

                // The general case.
                (buffer_size - step.last_stride) / step.stride + 1
            };

            match step.mode {
                VertexStepMode::Vertex => {
                    if limit < vertex_limit {
                        vertex_limit = limit;
                        vertex_limit_slot = idx as _;
                    }
                }
                VertexStepMode::Instance => {
                    if limit < instance_limit {
                        instance_limit = limit;
                        instance_limit_slot = idx as _;
                    }
                }
            }
        }

        Self {
            vertex_limit,
            vertex_limit_slot,
            instance_limit,
            instance_limit_slot,
        }
    }

    pub(crate) fn validate_vertex_limit(
        &self,
        first_vertex: u32,
        vertex_count: u32,
    ) -> Result<(), DrawError> {
        let last_vertex = first_vertex as u64 + vertex_count as u64;
        let vertex_limit = self.vertex_limit;
        if last_vertex > vertex_limit {
            return Err(DrawError::VertexBeyondLimit {
                last_vertex,
                vertex_limit,
                slot: self.vertex_limit_slot,
            });
        }

        Ok(())
    }

    pub(crate) fn validate_instance_limit(
        &self,
        first_instance: u32,
        instance_count: u32,
    ) -> Result<(), DrawError> {
        let last_instance = first_instance as u64 + instance_count as u64;
        let instance_limit = self.instance_limit;
        if last_instance > instance_limit {
            return Err(DrawError::InstanceBeyondLimit {
                last_instance,
                instance_limit,
                slot: self.instance_limit_slot,
            });
        }

        Ok(())
    }
}

#[derive(Debug, Default)]
struct VertexState {
    buffer_sizes: [Option<BufferAddress>; hal::MAX_VERTEX_BUFFERS],
    limits: VertexLimits,
}

impl VertexState {
    fn update_limits(&mut self, pipeline_steps: &[VertexStep]) {
        self.limits = VertexLimits::new(self.buffer_sizes.iter().copied(), pipeline_steps);
    }
}

struct State<'scope, 'snatch_guard, 'cmd_enc> {
    pipeline_flags: PipelineFlags,
    blend_constant: OptionalState,
    stencil_reference: u32,
    pipeline: Option<Arc<RenderPipeline>>,
    index: IndexState,
    vertex: VertexState,

    info: RenderPassInfo,

    pass: pass::PassState<'scope, 'snatch_guard, 'cmd_enc>,

    active_occlusion_query: Option<(Arc<QuerySet>, u32)>,
    active_pipeline_statistics_query: Option<(Arc<QuerySet>, u32)>,
}

impl<'scope, 'snatch_guard, 'cmd_enc> State<'scope, 'snatch_guard, 'cmd_enc> {
    fn is_ready(&self, family: DrawCommandFamily) -> Result<(), DrawError> {
        if let Some(pipeline) = self.pipeline.as_ref() {
            self.pass.binder.check_compatibility(pipeline.as_ref())?;
            self.pass.binder.check_late_buffer_bindings()?;

            if self.blend_constant == OptionalState::Required {
                return Err(DrawError::MissingBlendConstant);
            }

            // Determine how many vertex buffers have already been bound
            let vertex_buffer_count = self
                .vertex
                .buffer_sizes
                .iter()
                .take_while(|v| v.is_some())
                .count() as u32;
            // Compare with the needed quantity
            if vertex_buffer_count < pipeline.vertex_steps.len() as u32 {
                return Err(DrawError::MissingVertexBuffer {
                    pipeline: pipeline.error_ident(),
                    index: vertex_buffer_count,
                });
            }

            if family == DrawCommandFamily::DrawIndexed {
                // Pipeline expects an index buffer
                // We have a buffer bound
                let buffer_index_format = self
                    .index
                    .buffer_format
                    .ok_or(DrawError::MissingIndexBuffer)?;

                if pipeline.topology.is_strip()
                    && pipeline.strip_index_format != Some(buffer_index_format)
                {
                    return Err(DrawError::UnmatchedStripIndexFormat {
                        pipeline: pipeline.error_ident(),
                        strip_index_format: pipeline.strip_index_format,
                        buffer_format: buffer_index_format,
                    });
                }
            }
            if (family == DrawCommandFamily::DrawMeshTasks) != pipeline.is_mesh {
                return Err(DrawError::WrongPipelineType {
                    wanted_mesh_pipeline: !pipeline.is_mesh,
                });
            }
            Ok(())
        } else {
            Err(DrawError::MissingPipeline(pass::MissingPipeline))
        }
    }

    /// Flush binding state in preparation for a draw call.
    ///
    /// See the compute pass version for an explanation of some ways that
    /// `flush_bindings` differs between the two types of passes.
    fn flush_bindings(&mut self) -> Result<(), RenderPassErrorInner> {
        flush_bindings_helper(&mut self.pass)?;
        Ok(())
    }

    /// Reset the `RenderBundle`-related states.
    fn reset_bundle(&mut self) {
        self.pass.binder.reset();
        self.pipeline = None;
        self.index.reset();
        self.vertex = Default::default();
    }
}

/// Describes an attachment location in words.
///
/// Can be used as "the {loc} has..." or "{loc} has..."
#[derive(Debug, Copy, Clone)]
pub enum AttachmentErrorLocation {
    Color { index: usize, resolve: bool },
    Depth,
}

impl fmt::Display for AttachmentErrorLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            AttachmentErrorLocation::Color {
                index,
                resolve: false,
            } => write!(f, "color attachment at index {index}'s texture view"),
            AttachmentErrorLocation::Color {
                index,
                resolve: true,
            } => write!(
                f,
                "color attachment at index {index}'s resolve texture view"
            ),
            AttachmentErrorLocation::Depth => write!(f, "depth attachment's texture view"),
        }
    }
}

#[derive(Clone, Debug, Error)]
#[non_exhaustive]
pub enum ColorAttachmentError {
    #[error("Attachment format {0:?} is not a color format")]
    InvalidFormat(wgt::TextureFormat),
    #[error("The number of color attachments {given} exceeds the limit {limit}")]
    TooMany { given: usize, limit: usize },
    #[error("The total number of bytes per sample in color attachments {total} exceeds the limit {limit}")]
    TooManyBytesPerSample { total: u32, limit: u32 },
    #[error("Depth slice must be less than {limit} but is {given}")]
    DepthSliceLimit { given: u32, limit: u32 },
    #[error("Color attachment's view is 3D and requires depth slice to be provided")]
    MissingDepthSlice,
    #[error("Depth slice was provided but the color attachment's view is not 3D")]
    UnneededDepthSlice,
    #[error("{view}'s subresource at mip {mip_level} and depth/array layer {depth_or_array_layer} is already attached to this render pass")]
    SubresourceOverlap {
        view: ResourceErrorIdent,
        mip_level: u32,
        depth_or_array_layer: u32,
    },
    #[error("Color attachment's usage contains {0:?}. This can only be used with StoreOp::{1:?}, but StoreOp::{2:?} was provided")]
    InvalidUsageForStoreOp(TextureUsages, StoreOp, StoreOp),
}

impl WebGpuError for ColorAttachmentError {
    fn webgpu_error_type(&self) -> ErrorType {
        ErrorType::Validation
    }
}

#[derive(Clone, Debug, Error)]
#[non_exhaustive]
pub enum AttachmentError {
    #[error("The format of the depth-stencil attachment ({0:?}) is not a depth-or-stencil format")]
    InvalidDepthStencilAttachmentFormat(wgt::TextureFormat),
    #[error("LoadOp must be None for read-only attachments")]
    ReadOnlyWithLoad,
    #[error("StoreOp must be None for read-only attachments")]
    ReadOnlyWithStore,
    #[error("Attachment without load")]
    NoLoad,
    #[error("Attachment without store")]
    NoStore,
    #[error("LoadOp is `Clear` but no clear value was provided")]
    NoClearValue,
    #[error("Clear value ({0}) must be between 0.0 and 1.0, inclusive")]
    ClearValueOutOfRange(f32),
}

impl WebGpuError for AttachmentError {
    fn webgpu_error_type(&self) -> ErrorType {
        ErrorType::Validation
    }
}

/// Error encountered when performing a render pass.
#[derive(Clone, Debug, Error)]
pub enum RenderPassErrorInner {
    #[error(transparent)]
    Device(#[from] DeviceError),
    #[error(transparent)]
    ColorAttachment(#[from] ColorAttachmentError),
    #[error(transparent)]
    InvalidAttachment(#[from] AttachmentError),
    #[error(transparent)]
    EncoderState(#[from] EncoderStateError),
    #[error("Parent encoder is invalid")]
    InvalidParentEncoder,
    #[error(transparent)]
    DebugGroupError(#[from] DebugGroupError),
    #[error("The format of the {location} ({format:?}) is not resolvable")]
    UnsupportedResolveTargetFormat {
        location: AttachmentErrorLocation,
        format: wgt::TextureFormat,
    },
    #[error("No color attachments or depth attachments were provided, at least one attachment of any kind must be provided")]
    MissingAttachments,
    #[error("The {location} is not renderable:")]
    TextureViewIsNotRenderable {
        location: AttachmentErrorLocation,
        #[source]
        reason: TextureViewNotRenderableReason,
    },
    #[error("Attachments have differing sizes: the {expected_location} has extent {expected_extent:?} but is followed by the {actual_location} which has {actual_extent:?}")]
    AttachmentsDimensionMismatch {
        expected_location: AttachmentErrorLocation,
        expected_extent: wgt::Extent3d,
        actual_location: AttachmentErrorLocation,
        actual_extent: wgt::Extent3d,
    },
    #[error("Attachments have differing sample counts: the {expected_location} has count {expected_samples:?} but is followed by the {actual_location} which has count {actual_samples:?}")]
    AttachmentSampleCountMismatch {
        expected_location: AttachmentErrorLocation,
        expected_samples: u32,
        actual_location: AttachmentErrorLocation,
        actual_samples: u32,
    },
    #[error("The resolve source, {location}, must be multi-sampled (has {src} samples) while the resolve destination must not be multisampled (has {dst} samples)")]
    InvalidResolveSampleCounts {
        location: AttachmentErrorLocation,
        src: u32,
        dst: u32,
    },
    #[error(
        "Resource source, {location}, format ({src:?}) must match the resolve destination format ({dst:?})"
    )]
    MismatchedResolveTextureFormat {
        location: AttachmentErrorLocation,
        src: wgt::TextureFormat,
        dst: wgt::TextureFormat,
    },
    #[error("Unable to clear non-present/read-only depth")]
    InvalidDepthOps,
    #[error("Unable to clear non-present/read-only stencil")]
    InvalidStencilOps,
    #[error(transparent)]
    InvalidValuesOffset(#[from] pass::InvalidValuesOffset),
    #[error(transparent)]
    MissingFeatures(#[from] MissingFeatures),
    #[error(transparent)]
    MissingDownlevelFlags(#[from] MissingDownlevelFlags),
    #[error("Indirect buffer offset {0:?} is not a multiple of 4")]
    UnalignedIndirectBufferOffset(BufferAddress),
    #[error("Indirect draw uses bytes {offset}..{end_offset} using count {count} which overruns indirect buffer of size {buffer_size}")]
    IndirectBufferOverrun {
        count: u32,
        offset: u64,
        end_offset: u64,
        buffer_size: u64,
    },
    #[error("Indirect draw uses bytes {begin_count_offset}..{end_count_offset} which overruns indirect buffer of size {count_buffer_size}")]
    IndirectCountBufferOverrun {
        begin_count_offset: u64,
        end_count_offset: u64,
        count_buffer_size: u64,
    },
    #[error(transparent)]
    ResourceUsageCompatibility(#[from] ResourceUsageCompatibilityError),
    #[error("Render bundle has incompatible targets, {0}")]
    IncompatibleBundleTargets(#[from] RenderPassCompatibilityError),
    #[error(
        "Render bundle has incompatible read-only flags: \
             bundle has flags depth = {bundle_depth} and stencil = {bundle_stencil}, \
             while the pass has flags depth = {pass_depth} and stencil = {pass_stencil}. \
             Read-only renderpasses are only compatible with read-only bundles for that aspect."
    )]
    IncompatibleBundleReadOnlyDepthStencil {
        pass_depth: bool,
        pass_stencil: bool,
        bundle_depth: bool,
        bundle_stencil: bool,
    },
    #[error(transparent)]
    RenderCommand(#[from] RenderCommandError),
    #[error(transparent)]
    Draw(#[from] DrawError),
    #[error(transparent)]
    Bind(#[from] BindError),
    #[error("Immediate data offset must be aligned to 4 bytes")]
    ImmediateOffsetAlignment,
    #[error("Immediate data size must be aligned to 4 bytes")]
    ImmediateDataizeAlignment,
    #[error("Ran out of immediate data space. Don't set 4gb of immediates per ComputePass.")]
    ImmediateOutOfMemory,
    #[error(transparent)]
    QueryUse(#[from] QueryUseError),
    #[error("Multiview layer count must match")]
    MultiViewMismatch,
    #[error(
        "Multiview pass texture views with more than one array layer must have D2Array dimension"
    )]
    MultiViewDimensionMismatch,
    #[error("Multiview view count limit violated")]
    TooManyMultiviewViews,
    #[error("missing occlusion query set")]
    MissingOcclusionQuerySet,
    #[error(transparent)]
    DestroyedResource(#[from] DestroyedResourceError),
    #[error("The compute pass has already been ended and no further commands can be recorded")]
    PassEnded,
    #[error(transparent)]
    InvalidResource(#[from] InvalidResourceError),
    #[error(transparent)]
    TimestampWrites(#[from] TimestampWritesError),
}

impl From<MissingBufferUsageError> for RenderPassErrorInner {
    fn from(error: MissingBufferUsageError) -> Self {
        Self::RenderCommand(error.into())
    }
}

impl From<MissingTextureUsageError> for RenderPassErrorInner {
    fn from(error: MissingTextureUsageError) -> Self {
        Self::RenderCommand(error.into())
    }
}

impl From<pass::BindGroupIndexOutOfRange> for RenderPassErrorInner {
    fn from(error: pass::BindGroupIndexOutOfRange) -> Self {
        Self::RenderCommand(RenderCommandError::BindGroupIndexOutOfRange(error))
    }
}

impl From<pass::MissingPipeline> for RenderPassErrorInner {
    fn from(error: pass::MissingPipeline) -> Self {
        Self::Draw(DrawError::MissingPipeline(error))
    }
}

impl From<ImmediateUploadError> for RenderPassErrorInner {
    fn from(error: ImmediateUploadError) -> Self {
        Self::RenderCommand(error.into())
    }
}

/// Error encountered when performing a render pass.
#[derive(Clone, Debug, Error)]
#[error("{scope}")]
pub struct RenderPassError {
    pub scope: PassErrorScope,
    #[source]
    pub(super) inner: RenderPassErrorInner,
}

impl<E: Into<RenderPassErrorInner>> MapPassErr<RenderPassError> for E {
    fn map_pass_err(self, scope: PassErrorScope) -> RenderPassError {
        RenderPassError {
            scope,
            inner: self.into(),
        }
    }
}

impl WebGpuError for RenderPassError {
    fn webgpu_error_type(&self) -> ErrorType {
        let Self { scope: _, inner } = self;
        match inner {
            RenderPassErrorInner::Device(e) => e.webgpu_error_type(),
            RenderPassErrorInner::ColorAttachment(e) => e.webgpu_error_type(),
            RenderPassErrorInner::EncoderState(e) => e.webgpu_error_type(),
            RenderPassErrorInner::DebugGroupError(e) => e.webgpu_error_type(),
            RenderPassErrorInner::MissingFeatures(e) => e.webgpu_error_type(),
            RenderPassErrorInner::MissingDownlevelFlags(e) => e.webgpu_error_type(),
            RenderPassErrorInner::RenderCommand(e) => e.webgpu_error_type(),
            RenderPassErrorInner::Draw(e) => e.webgpu_error_type(),
            RenderPassErrorInner::Bind(e) => e.webgpu_error_type(),
            RenderPassErrorInner::QueryUse(e) => e.webgpu_error_type(),
            RenderPassErrorInner::DestroyedResource(e) => e.webgpu_error_type(),
            RenderPassErrorInner::InvalidResource(e) => e.webgpu_error_type(),
            RenderPassErrorInner::IncompatibleBundleTargets(e) => e.webgpu_error_type(),
            RenderPassErrorInner::InvalidAttachment(e) => e.webgpu_error_type(),
            RenderPassErrorInner::TimestampWrites(e) => e.webgpu_error_type(),
            RenderPassErrorInner::InvalidValuesOffset(e) => e.webgpu_error_type(),

            RenderPassErrorInner::InvalidParentEncoder
            | RenderPassErrorInner::UnsupportedResolveTargetFormat { .. }
            | RenderPassErrorInner::MissingAttachments
            | RenderPassErrorInner::TextureViewIsNotRenderable { .. }
            | RenderPassErrorInner::AttachmentsDimensionMismatch { .. }
            | RenderPassErrorInner::AttachmentSampleCountMismatch { .. }
            | RenderPassErrorInner::InvalidResolveSampleCounts { .. }
            | RenderPassErrorInner::MismatchedResolveTextureFormat { .. }
            | RenderPassErrorInner::InvalidDepthOps
            | RenderPassErrorInner::InvalidStencilOps
            | RenderPassErrorInner::UnalignedIndirectBufferOffset(..)
            | RenderPassErrorInner::IndirectBufferOverrun { .. }
            | RenderPassErrorInner::IndirectCountBufferOverrun { .. }
            | RenderPassErrorInner::ResourceUsageCompatibility(..)
            | RenderPassErrorInner::IncompatibleBundleReadOnlyDepthStencil { .. }
            | RenderPassErrorInner::ImmediateOffsetAlignment
            | RenderPassErrorInner::ImmediateDataizeAlignment
            | RenderPassErrorInner::ImmediateOutOfMemory
            | RenderPassErrorInner::MultiViewMismatch
            | RenderPassErrorInner::MultiViewDimensionMismatch
            | RenderPassErrorInner::TooManyMultiviewViews
            | RenderPassErrorInner::MissingOcclusionQuerySet
            | RenderPassErrorInner::PassEnded => ErrorType::Validation,
        }
    }
}

struct RenderAttachment {
    texture: Arc<Texture>,
    selector: TextureSelector,
    usage: wgt::TextureUses,
}

impl TextureView {
    fn to_render_attachment(&self, usage: wgt::TextureUses) -> RenderAttachment {
        RenderAttachment {
            texture: self.parent.clone(),
            selector: self.selector.clone(),
            usage,
        }
    }
}

const MAX_TOTAL_ATTACHMENTS: usize = hal::MAX_COLOR_ATTACHMENTS + hal::MAX_COLOR_ATTACHMENTS + 1;
type AttachmentDataVec<T> = ArrayVec<T, MAX_TOTAL_ATTACHMENTS>;

struct RenderPassInfo {
    context: RenderPassContext,
    /// All render attachments, including depth/stencil
    render_attachments: AttachmentDataVec<RenderAttachment>,
    is_depth_read_only: bool,
    is_stencil_read_only: bool,
    extent: wgt::Extent3d,

    divergent_discarded_depth_stencil_aspect: Option<(wgt::TextureAspect, Arc<TextureView>)>,
    multiview_mask: Option<NonZeroU32>,
}

impl RenderPassInfo {
    fn add_pass_texture_init_actions<V>(
        load_op: LoadOp<V>,
        store_op: StoreOp,
        texture_memory_actions: &mut CommandBufferTextureMemoryActions,
        view: &TextureView,
        pending_discard_init_fixups: &mut SurfacesInDiscardState,
    ) {
        if matches!(load_op, LoadOp::Load) {
            pending_discard_init_fixups.extend(texture_memory_actions.register_init_action(
                &TextureInitTrackerAction {
                    texture: view.parent.clone(),
                    range: TextureInitRange::from(view.selector.clone()),
                    // Note that this is needed even if the target is discarded,
                    kind: MemoryInitKind::NeedsInitializedMemory,
                },
            ));
        } else if store_op == StoreOp::Store {
            // Clear + Store
            texture_memory_actions.register_implicit_init(
                &view.parent,
                TextureInitRange::from(view.selector.clone()),
            );
        }
        if store_op == StoreOp::Discard {
            // the discard happens at the *end* of a pass, but recording the
            // discard right away be alright since the texture can't be used
            // during the pass anyways
            texture_memory_actions.discard(TextureSurfaceDiscard {
                texture: view.parent.clone(),
                mip_level: view.selector.mips.start,
                layer: view.selector.layers.start,
            });
        }
    }

    fn start(
        device: &Arc<Device>,
        hal_label: Option<&str>,
        color_attachments: &[Option<ArcRenderPassColorAttachment>],
        mut depth_stencil_attachment: Option<
            ResolvedRenderPassDepthStencilAttachment<Arc<TextureView>>,
        >,
        mut timestamp_writes: Option<ArcPassTimestampWrites>,
        mut occlusion_query_set: Option<Arc<QuerySet>>,
        encoder: &mut dyn hal::DynCommandEncoder,
        trackers: &mut Tracker,
        texture_memory_actions: &mut CommandBufferTextureMemoryActions,
        pending_query_resets: &mut QueryResetMap,
        pending_discard_init_fixups: &mut SurfacesInDiscardState,
        snatch_guard: &SnatchGuard<'_>,
        multiview_mask: Option<NonZeroU32>,
    ) -> Result<Self, RenderPassErrorInner> {
        profiling::scope!("RenderPassInfo::start");

        // We default to false intentionally, even if depth-stencil isn't used at all.
        // This allows us to use the primary raw pipeline in `RenderPipeline`,
        // instead of the special read-only one, which would be `None`.
        let mut is_depth_read_only = false;
        let mut is_stencil_read_only = false;

        let mut render_attachments = AttachmentDataVec::<RenderAttachment>::new();
        let mut discarded_surfaces = AttachmentDataVec::new();
        let mut divergent_discarded_depth_stencil_aspect = None;

        let mut attachment_location = AttachmentErrorLocation::Color {
            index: usize::MAX,
            resolve: false,
        };
        let mut extent = None;
        let mut sample_count = 0;

        let mut detected_multiview: Option<Option<NonZeroU32>> = None;

        let mut check_multiview = |view: &TextureView| {
            // Get the multiview configuration for this texture view
            let layers = view.selector.layers.end - view.selector.layers.start;
            let this_multiview = if layers >= 2 {
                // Trivially proven by the if above
                Some(unsafe { NonZeroU32::new_unchecked(layers) })
            } else {
                None
            };

            // Make sure that if this view is a multiview, it is set to be an array
            if this_multiview.is_some() && view.desc.dimension != TextureViewDimension::D2Array {
                return Err(RenderPassErrorInner::MultiViewDimensionMismatch);
            }

            // Validate matching first, or store the first one
            if let Some(multiview) = detected_multiview {
                if multiview != this_multiview {
                    return Err(RenderPassErrorInner::MultiViewMismatch);
                }
            } else {
                // Multiview is only supported if the feature is enabled
                if let Some(this_multiview) = this_multiview {
                    device.require_features(wgt::Features::MULTIVIEW)?;
                    if this_multiview.get() > device.limits.max_multiview_view_count {
                        return Err(RenderPassErrorInner::TooManyMultiviewViews);
                    }
                }

                detected_multiview = Some(this_multiview);
            }

            Ok(())
        };
        let mut add_view = |view: &TextureView, location| {
            let render_extent = view.render_extent.map_err(|reason| {
                RenderPassErrorInner::TextureViewIsNotRenderable { location, reason }
            })?;
            if let Some(ex) = extent {
                if ex != render_extent {
                    return Err(RenderPassErrorInner::AttachmentsDimensionMismatch {
                        expected_location: attachment_location,
                        expected_extent: ex,
                        actual_location: location,
                        actual_extent: render_extent,
                    });
                }
            } else {
                extent = Some(render_extent);
            }
            if sample_count == 0 {
                sample_count = view.samples;
            } else if sample_count != view.samples {
                return Err(RenderPassErrorInner::AttachmentSampleCountMismatch {
                    expected_location: attachment_location,
                    expected_samples: sample_count,
                    actual_location: location,
                    actual_samples: view.samples,
                });
            }
            attachment_location = location;
            Ok(())
        };

        let mut depth_stencil = None;

        if let Some(at) = depth_stencil_attachment.as_ref() {
            let view = &at.view;
            check_multiview(view)?;
            add_view(view, AttachmentErrorLocation::Depth)?;

            let ds_aspects = view.desc.aspects();

            if !ds_aspects.contains(hal::FormatAspects::STENCIL)
                || (at.stencil.load_op().eq_variant(at.depth.load_op())
                    && at.stencil.store_op() == at.depth.store_op())
            {
                Self::add_pass_texture_init_actions(
                    at.depth.load_op(),
                    at.depth.store_op(),
                    texture_memory_actions,
                    view,
                    pending_discard_init_fixups,
                );
            } else if !ds_aspects.contains(hal::FormatAspects::DEPTH) {
                Self::add_pass_texture_init_actions(
                    at.stencil.load_op(),
                    at.stencil.store_op(),
                    texture_memory_actions,
                    view,
                    pending_discard_init_fixups,
                );
            } else {
                // This is the only place (anywhere in wgpu) where Stencil &
                // Depth init state can diverge.
                //
                // To safe us the overhead of tracking init state of texture
                // aspects everywhere, we're going to cheat a little bit in
                // order to keep the init state of both Stencil and Depth
                // aspects in sync. The expectation is that we hit this path
                // extremely rarely!
                //
                // Diverging LoadOp, i.e. Load + Clear:
                //
                // Record MemoryInitKind::NeedsInitializedMemory for the entire
                // surface, a bit wasteful on unit but no negative effect!
                //
                // Rationale: If the loaded channel is uninitialized it needs
                // clearing, the cleared channel doesn't care. (If everything is
                // already initialized nothing special happens)
                //
                // (possible minor optimization: Clear caused by
                // NeedsInitializedMemory should know that it doesn't need to
                // clear the aspect that was set to C)
                let need_init_beforehand =
                    at.depth.load_op() == LoadOp::Load || at.stencil.load_op() == LoadOp::Load;
                if need_init_beforehand {
                    pending_discard_init_fixups.extend(
                        texture_memory_actions.register_init_action(&TextureInitTrackerAction {
                            texture: view.parent.clone(),
                            range: TextureInitRange::from(view.selector.clone()),
                            kind: MemoryInitKind::NeedsInitializedMemory,
                        }),
                    );
                }

                // Diverging Store, i.e. Discard + Store:
                //
                // Immediately zero out channel that is set to discard after
                // we're done with the render pass. This allows us to set the
                // entire surface to MemoryInitKind::ImplicitlyInitialized (if
                // it isn't already set to NeedsInitializedMemory).
                //
                // (possible optimization: Delay and potentially drop this zeroing)
                if at.depth.store_op() != at.stencil.store_op() {
                    if !need_init_beforehand {
                        texture_memory_actions.register_implicit_init(
                            &view.parent,
                            TextureInitRange::from(view.selector.clone()),
                        );
                    }
                    divergent_discarded_depth_stencil_aspect = Some((
                        if at.depth.store_op() == StoreOp::Discard {
                            wgt::TextureAspect::DepthOnly
                        } else {
                            wgt::TextureAspect::StencilOnly
                        },
                        view.clone(),
                    ));
                } else if at.depth.store_op() == StoreOp::Discard {
                    // Both are discarded using the regular path.
                    discarded_surfaces.push(TextureSurfaceDiscard {
                        texture: view.parent.clone(),
                        mip_level: view.selector.mips.start,
                        layer: view.selector.layers.start,
                    });
                }
            }

            is_depth_read_only = at.depth.is_readonly();
            is_stencil_read_only = at.stencil.is_readonly();

            let usage = if is_depth_read_only
                && is_stencil_read_only
                && device
                    .downlevel
                    .flags
                    .contains(wgt::DownlevelFlags::READ_ONLY_DEPTH_STENCIL)
            {
                wgt::TextureUses::DEPTH_STENCIL_READ | wgt::TextureUses::RESOURCE
            } else {
                wgt::TextureUses::DEPTH_STENCIL_WRITE
            };
            render_attachments.push(view.to_render_attachment(usage));

            depth_stencil = Some(hal::DepthStencilAttachment {
                target: hal::Attachment {
                    view: view.try_raw(snatch_guard)?,
                    usage,
                },
                depth_ops: at.depth.hal_ops(),
                stencil_ops: at.stencil.hal_ops(),
                clear_value: (at.depth.clear_value(), at.stencil.clear_value()),
            });
        }

        let mut attachment_set = crate::FastHashSet::default();

        let mut color_attachments_hal =
            ArrayVec::<Option<hal::ColorAttachment<_>>, { hal::MAX_COLOR_ATTACHMENTS }>::new();
        for (index, attachment) in color_attachments.iter().enumerate() {
            let at = if let Some(attachment) = attachment.as_ref() {
                attachment
            } else {
                color_attachments_hal.push(None);
                continue;
            };
            let color_view: &TextureView = &at.view;
            color_view.same_device(device)?;
            check_multiview(color_view)?;
            add_view(
                color_view,
                AttachmentErrorLocation::Color {
                    index,
                    resolve: false,
                },
            )?;

            if !color_view.desc.aspects().intersects(
                hal::FormatAspects::COLOR
                    | hal::FormatAspects::PLANE_0
                    | hal::FormatAspects::PLANE_1
                    | hal::FormatAspects::PLANE_2,
            ) {
                return Err(RenderPassErrorInner::ColorAttachment(
                    ColorAttachmentError::InvalidFormat(color_view.desc.format),
                ));
            }

            if color_view.desc.dimension == TextureViewDimension::D3 {
                if let Some(depth_slice) = at.depth_slice {
                    let mip = color_view.desc.range.base_mip_level;
                    let mip_size = color_view
                        .parent
                        .desc
                        .size
                        .mip_level_size(mip, color_view.parent.desc.dimension);
                    let limit = mip_size.depth_or_array_layers;
                    if depth_slice >= limit {
                        return Err(RenderPassErrorInner::ColorAttachment(
                            ColorAttachmentError::DepthSliceLimit {
                                given: depth_slice,
                                limit,
                            },
                        ));
                    }
                } else {
                    return Err(RenderPassErrorInner::ColorAttachment(
                        ColorAttachmentError::MissingDepthSlice,
                    ));
                }
            } else if at.depth_slice.is_some() {
                return Err(RenderPassErrorInner::ColorAttachment(
                    ColorAttachmentError::UnneededDepthSlice,
                ));
            }

            validation::validate_color_attachment_bytes_per_sample(
                color_attachments
                    .iter()
                    .flatten()
                    .map(|at| at.view.desc.format),
                device.limits.max_color_attachment_bytes_per_sample,
            )
            .map_err(RenderPassErrorInner::ColorAttachment)?;

            fn check_attachment_overlap(
                attachment_set: &mut crate::FastHashSet<(crate::track::TrackerIndex, u32, u32)>,
                view: &TextureView,
                depth_slice: Option<u32>,
            ) -> Result<(), ColorAttachmentError> {
                let mut insert = |slice| {
                    let mip_level = view.desc.range.base_mip_level;
                    if attachment_set.insert((
                        view.parent.tracking_data.tracker_index(),
                        mip_level,
                        slice,
                    )) {
                        Ok(())
                    } else {
                        Err(ColorAttachmentError::SubresourceOverlap {
                            view: view.error_ident(),
                            mip_level,
                            depth_or_array_layer: slice,
                        })
                    }
                };
                match view.desc.dimension {
                    TextureViewDimension::D2 => {
                        insert(view.desc.range.base_array_layer)?;
                    }
                    TextureViewDimension::D2Array => {
                        for layer in view.selector.layers.clone() {
                            insert(layer)?;
                        }
                    }
                    TextureViewDimension::D3 => {
                        insert(depth_slice.unwrap())?;
                    }
                    _ => unreachable!(),
                };
                Ok(())
            }

            check_attachment_overlap(&mut attachment_set, color_view, at.depth_slice)?;

            Self::add_pass_texture_init_actions(
                at.load_op,
                at.store_op,
                texture_memory_actions,
                color_view,
                pending_discard_init_fixups,
            );
            render_attachments
                .push(color_view.to_render_attachment(wgt::TextureUses::COLOR_TARGET));

            let mut hal_resolve_target = None;
            if let Some(resolve_view) = &at.resolve_target {
                resolve_view.same_device(device)?;
                check_multiview(resolve_view)?;

                check_attachment_overlap(&mut attachment_set, resolve_view, None)?;

                let resolve_location = AttachmentErrorLocation::Color {
                    index,
                    resolve: true,
                };

                let render_extent = resolve_view.render_extent.map_err(|reason| {
                    RenderPassErrorInner::TextureViewIsNotRenderable {
                        location: resolve_location,
                        reason,
                    }
                })?;
                if color_view.render_extent.unwrap() != render_extent {
                    return Err(RenderPassErrorInner::AttachmentsDimensionMismatch {
                        expected_location: attachment_location,
                        expected_extent: extent.unwrap_or_default(),
                        actual_location: resolve_location,
                        actual_extent: render_extent,
                    });
                }
                if color_view.samples == 1 || resolve_view.samples != 1 {
                    return Err(RenderPassErrorInner::InvalidResolveSampleCounts {
                        location: resolve_location,
                        src: color_view.samples,
                        dst: resolve_view.samples,
                    });
                }
                if color_view.desc.format != resolve_view.desc.format {
                    return Err(RenderPassErrorInner::MismatchedResolveTextureFormat {
                        location: resolve_location,
                        src: color_view.desc.format,
                        dst: resolve_view.desc.format,
                    });
                }
                if !resolve_view
                    .format_features
                    .flags
                    .contains(wgt::TextureFormatFeatureFlags::MULTISAMPLE_RESOLVE)
                {
                    return Err(RenderPassErrorInner::UnsupportedResolveTargetFormat {
                        location: resolve_location,
                        format: resolve_view.desc.format,
                    });
                }

                texture_memory_actions.register_implicit_init(
                    &resolve_view.parent,
                    TextureInitRange::from(resolve_view.selector.clone()),
                );
                render_attachments
                    .push(resolve_view.to_render_attachment(wgt::TextureUses::COLOR_TARGET));

                hal_resolve_target = Some(hal::Attachment {
                    view: resolve_view.try_raw(snatch_guard)?,
                    usage: wgt::TextureUses::COLOR_TARGET,
                });
            }

            color_attachments_hal.push(Some(hal::ColorAttachment {
                target: hal::Attachment {
                    view: color_view.try_raw(snatch_guard)?,
                    usage: wgt::TextureUses::COLOR_TARGET,
                },
                depth_slice: at.depth_slice,
                resolve_target: hal_resolve_target,
                ops: at.hal_ops(),
                clear_value: at.clear_value(),
            }));
        }

        let extent = extent.ok_or(RenderPassErrorInner::MissingAttachments)?;

        let detected_multiview =
            detected_multiview.expect("Multiview was not detected, no attachments");
        if let Some(mask) = multiview_mask {
            // 0x01 will have msb 0
            let mask_msb = 31 - mask.leading_zeros();
            let detected_mv = detected_multiview.map(NonZeroU32::get).unwrap_or(1);
            if mask_msb >= detected_mv {
                return Err(RenderPassErrorInner::MultiViewMismatch);
            }
            if mask.get() != (1 << detected_mv) - 1 {
                device.require_features(wgt::Features::SELECTIVE_MULTIVIEW)?;
            }
        }

        let attachment_formats = AttachmentData {
            colors: color_attachments
                .iter()
                .map(|at| at.as_ref().map(|at| at.view.desc.format))
                .collect(),
            resolves: color_attachments
                .iter()
                .filter_map(|at| {
                    at.as_ref().and_then(|at| {
                        at.resolve_target
                            .as_ref()
                            .map(|resolve| resolve.desc.format)
                    })
                })
                .collect(),
            depth_stencil: depth_stencil_attachment
                .as_ref()
                .map(|at| at.view.desc.format),
        };

        let context = RenderPassContext {
            attachments: attachment_formats,
            sample_count,
            multiview_mask,
        };

        let timestamp_writes_hal = if let Some(tw) = timestamp_writes.as_ref() {
            let query_set = &tw.query_set;
            query_set.same_device(device)?;

            if let Some(index) = tw.beginning_of_pass_write_index {
                pending_query_resets.use_query_set(query_set, index);
            }
            if let Some(index) = tw.end_of_pass_write_index {
                pending_query_resets.use_query_set(query_set, index);
            }

            Some(hal::PassTimestampWrites {
                query_set: query_set.raw(),
                beginning_of_pass_write_index: tw.beginning_of_pass_write_index,
                end_of_pass_write_index: tw.end_of_pass_write_index,
            })
        } else {
            None
        };

        let occlusion_query_set_hal = if let Some(query_set) = occlusion_query_set.as_ref() {
            query_set.same_device(device)?;
            Some(query_set.raw())
        } else {
            None
        };

        let hal_desc = hal::RenderPassDescriptor {
            label: hal_label,
            extent,
            sample_count,
            color_attachments: &color_attachments_hal,
            depth_stencil_attachment: depth_stencil,
            multiview_mask,
            timestamp_writes: timestamp_writes_hal,
            occlusion_query_set: occlusion_query_set_hal,
        };
        unsafe {
            encoder
                .begin_render_pass(&hal_desc)
                .map_err(|e| device.handle_hal_error(e))?;
        };
        drop(color_attachments_hal); // Drop, so we can consume `color_attachments` for the tracker.

        // Can't borrow the tracker more than once, so have to add to the tracker after the `begin_render_pass` hal call.
        if let Some(tw) = timestamp_writes.take() {
            trackers.query_sets.insert_single(tw.query_set);
        };
        if let Some(occlusion_query_set) = occlusion_query_set.take() {
            trackers.query_sets.insert_single(occlusion_query_set);
        };
        if let Some(at) = depth_stencil_attachment.take() {
            trackers.views.insert_single(at.view.clone());
        }
        for at in color_attachments.iter().flatten() {
            trackers.views.insert_single(at.view.clone());
            if let Some(resolve_target) = at.resolve_target.clone() {
                trackers.views.insert_single(resolve_target);
            }
        }

        Ok(Self {
            context,
            render_attachments,
            is_depth_read_only,
            is_stencil_read_only,
            extent,
            divergent_discarded_depth_stencil_aspect,
            multiview_mask,
        })
    }

    fn finish(
        self,
        device: &Device,
        raw: &mut dyn hal::DynCommandEncoder,
        snatch_guard: &SnatchGuard,
        scope: &mut UsageScope<'_>,
        instance_flags: wgt::InstanceFlags,
    ) -> Result<(), RenderPassErrorInner> {
        profiling::scope!("RenderPassInfo::finish");
        unsafe {
            raw.end_render_pass();
        }

        for ra in self.render_attachments {
            let texture = &ra.texture;
            texture.check_usage(TextureUsages::RENDER_ATTACHMENT)?;

            // the tracker set of the pass is always in "extend" mode
            unsafe {
                scope
                    .textures
                    .merge_single(texture, Some(ra.selector.clone()), ra.usage)?
            };
        }

        // If either only stencil or depth was discarded, we put in a special
        // clear pass to keep the init status of the aspects in sync. We do this
        // so we don't need to track init state for depth/stencil aspects
        // individually.
        //
        // Note that we don't go the usual route of "brute force" initializing
        // the texture when need arises here, since this path is actually
        // something a user may genuinely want (where as the other cases are
        // more seen along the lines as gracefully handling a user error).
        if let Some((aspect, view)) = self.divergent_discarded_depth_stencil_aspect {
            let (depth_ops, stencil_ops) = if aspect == wgt::TextureAspect::DepthOnly {
                (
                    hal::AttachmentOps::LOAD_CLEAR | hal::AttachmentOps::STORE, // clear depth
                    hal::AttachmentOps::LOAD | hal::AttachmentOps::STORE,       // unchanged stencil
                )
            } else {
                (
                    hal::AttachmentOps::LOAD | hal::AttachmentOps::STORE, // unchanged stencil
                    hal::AttachmentOps::LOAD_CLEAR | hal::AttachmentOps::STORE, // clear depth
                )
            };
            let desc = hal::RenderPassDescriptor::<'_, _, dyn hal::DynTextureView> {
                label: hal_label(
                    Some("(wgpu internal) Zero init discarded depth/stencil aspect"),
                    instance_flags,
                ),
                extent: view.render_extent.unwrap(),
                sample_count: view.samples,
                color_attachments: &[],
                depth_stencil_attachment: Some(hal::DepthStencilAttachment {
                    target: hal::Attachment {
                        view: view.try_raw(snatch_guard)?,
                        usage: wgt::TextureUses::DEPTH_STENCIL_WRITE,
                    },
                    depth_ops,
                    stencil_ops,
                    clear_value: (0.0, 0),
                }),
                multiview_mask: self.multiview_mask,
                timestamp_writes: None,
                occlusion_query_set: None,
            };
            unsafe {
                raw.begin_render_pass(&desc)
                    .map_err(|e| device.handle_hal_error(e))?;
                raw.end_render_pass();
            }
        }

        Ok(())
    }
}

impl Global {
    /// Creates a render pass.
    ///
    /// If creation fails, an invalid pass is returned. Attempting to record
    /// commands into an invalid pass is permitted, but a validation error will
    /// ultimately be generated when the parent encoder is finished, and it is
    /// not possible to run any commands from the invalid pass.
    ///
    /// If successful, puts the encoder into the [`Locked`] state.
    ///
    /// [`Locked`]: crate::command::CommandEncoderStatus::Locked
    pub fn command_encoder_begin_render_pass(
        &self,
        encoder_id: id::CommandEncoderId,
        desc: &RenderPassDescriptor<'_>,
    ) -> (RenderPass, Option<CommandEncoderError>) {
        use EncoderStateError as SErr;

        fn fill_arc_desc(
            hub: &crate::hub::Hub,
            desc: &RenderPassDescriptor<'_>,
            arc_desc: &mut ArcRenderPassDescriptor,
            device: &Device,
        ) -> Result<(), RenderPassErrorInner> {
            device.check_is_valid()?;

            let query_sets = hub.query_sets.read();
            let texture_views = hub.texture_views.read();

            let max_color_attachments = device.limits.max_color_attachments as usize;
            if desc.color_attachments.len() > max_color_attachments {
                return Err(RenderPassErrorInner::ColorAttachment(
                    ColorAttachmentError::TooMany {
                        given: desc.color_attachments.len(),
                        limit: max_color_attachments,
                    },
                ));
            }

            for color_attachment in desc.color_attachments.iter() {
                if let Some(RenderPassColorAttachment {
                    view: view_id,
                    depth_slice,
                    resolve_target,
                    load_op,
                    store_op,
                }) = color_attachment
                {
                    let view = texture_views.get(*view_id).get()?;
                    view.same_device(device)?;

                    if view.desc.usage.contains(TextureUsages::TRANSIENT)
                        && *store_op != StoreOp::Discard
                    {
                        return Err(RenderPassErrorInner::ColorAttachment(
                            ColorAttachmentError::InvalidUsageForStoreOp(
                                TextureUsages::TRANSIENT,
                                StoreOp::Discard,
                                *store_op,
                            ),
                        ));
                    }

                    let resolve_target = if let Some(resolve_target_id) = resolve_target {
                        let rt_arc = texture_views.get(*resolve_target_id).get()?;
                        rt_arc.same_device(device)?;

                        Some(rt_arc)
                    } else {
                        None
                    };

                    arc_desc
                        .color_attachments
                        .push(Some(ArcRenderPassColorAttachment {
                            view,
                            depth_slice: *depth_slice,
                            resolve_target,
                            load_op: *load_op,
                            store_op: *store_op,
                        }));
                } else {
                    arc_desc.color_attachments.push(None);
                }
            }

            arc_desc.depth_stencil_attachment =
            // https://gpuweb.github.io/gpuweb/#abstract-opdef-gpurenderpassdepthstencilattachment-gpurenderpassdepthstencilattachment-valid-usage
                if let Some(depth_stencil_attachment) = desc.depth_stencil_attachment {
                    let view = texture_views.get(depth_stencil_attachment.view).get()?;
                    view.same_device(device)?;

                    let format = view.desc.format;
                    if !format.is_depth_stencil_format() {
                        return Err(RenderPassErrorInner::InvalidAttachment(AttachmentError::InvalidDepthStencilAttachmentFormat(
                            view.desc.format,
                        )));
                    }

                    Some(ResolvedRenderPassDepthStencilAttachment {
                        view,
                        depth: if format.has_depth_aspect() {
                            depth_stencil_attachment.depth.resolve(|clear| if let Some(clear) = clear {
                                // If this.depthLoadOp is "clear", this.depthClearValue must be provided and must be between 0.0 and 1.0, inclusive.
                                if !(0.0..=1.0).contains(&clear) {
                                    Err(AttachmentError::ClearValueOutOfRange(clear))
                                } else {
                                    Ok(clear)
                                }
                            } else {
                                Err(AttachmentError::NoClearValue)
                            })?
                        } else {
                            ResolvedPassChannel::ReadOnly
                        },
                        stencil: if format.has_stencil_aspect() {
                            depth_stencil_attachment.stencil.resolve(|clear| Ok(clear.unwrap_or_default()))?
                        } else {
                            ResolvedPassChannel::ReadOnly
                        },
                    })
                } else {
                    None
                };

            arc_desc.timestamp_writes = desc
                .timestamp_writes
                .map(|tw| {
                    Global::validate_pass_timestamp_writes::<RenderPassErrorInner>(
                        device,
                        &query_sets,
                        tw,
                    )
                })
                .transpose()?;

            arc_desc.occlusion_query_set =
                if let Some(occlusion_query_set) = desc.occlusion_query_set {
                    let query_set = query_sets.get(occlusion_query_set).get()?;
                    query_set.same_device(device)?;

                    if !matches!(query_set.desc.ty, wgt::QueryType::Occlusion) {
                        return Err(QueryUseError::IncompatibleType {
                            set_type: query_set.desc.ty.into(),
                            query_type: super::SimplifiedQueryType::Occlusion,
                        }
                        .into());
                    }

                    Some(query_set)
                } else {
                    None
                };

            arc_desc.multiview_mask = desc.multiview_mask;

            Ok(())
        }

        let scope = PassErrorScope::Pass;
        let hub = &self.hub;

        let cmd_enc = hub.command_encoders.get(encoder_id);
        let mut cmd_buf_data = cmd_enc.data.lock();

        match cmd_buf_data.lock_encoder() {
            Ok(()) => {
                drop(cmd_buf_data);
                let mut arc_desc = ArcRenderPassDescriptor {
                    label: &desc.label,
                    timestamp_writes: None,
                    color_attachments: ArrayVec::new(),
                    depth_stencil_attachment: None,
                    occlusion_query_set: None,
                    multiview_mask: None,
                };
                match fill_arc_desc(hub, desc, &mut arc_desc, &cmd_enc.device) {
                    Ok(()) => (RenderPass::new(cmd_enc, arc_desc), None),
                    Err(err) => (
                        RenderPass::new_invalid(cmd_enc, &desc.label, err.map_pass_err(scope)),
                        None,
                    ),
                }
            }
            Err(err @ SErr::Locked) => {
                // Attempting to open a new pass while the encoder is locked
                // invalidates the encoder, but does not generate a validation
                // error.
                cmd_buf_data.invalidate(err.clone());
                drop(cmd_buf_data);
                (
                    RenderPass::new_invalid(cmd_enc, &desc.label, err.map_pass_err(scope)),
                    None,
                )
            }
            Err(err @ (SErr::Ended | SErr::Submitted)) => {
                // Attempting to open a new pass after the encode has ended
                // generates an immediate validation error.
                drop(cmd_buf_data);
                (
                    RenderPass::new_invalid(cmd_enc, &desc.label, err.clone().map_pass_err(scope)),
                    Some(err.into()),
                )
            }
            Err(err @ SErr::Invalid) => {
                // Passes can be opened even on an invalid encoder. Such passes
                // are even valid, but since there's no visible side-effect of
                // the pass being valid and there's no point in storing recorded
                // commands that will ultimately be discarded, we open an
                // invalid pass to save that work.
                drop(cmd_buf_data);
                (
                    RenderPass::new_invalid(cmd_enc, &desc.label, err.map_pass_err(scope)),
                    None,
                )
            }
            Err(SErr::Unlocked) => {
                unreachable!("lock_encoder cannot fail due to the encoder being unlocked")
            }
        }
    }

    pub fn render_pass_end(&self, pass: &mut RenderPass) -> Result<(), EncoderStateError> {
        profiling::scope!(
            "CommandEncoder::run_render_pass {}",
            pass.base.label.as_deref().unwrap_or("")
        );

        let cmd_enc = pass.parent.take().ok_or(EncoderStateError::Ended)?;
        let mut cmd_buf_data = cmd_enc.data.lock();

        cmd_buf_data.unlock_encoder()?;

        let base = pass.base.take();

        if let Err(RenderPassError {
            inner:
                RenderPassErrorInner::EncoderState(
                    err @ (EncoderStateError::Locked | EncoderStateError::Ended),
                ),
            scope: _,
        }) = base
        {
            // Most encoding errors are detected and raised within `finish()`.
            //
            // However, we raise a validation error here if the pass was opened
            // within another pass, or on a finished encoder. The latter is
            // particularly important, because in that case reporting errors via
            // `CommandEncoder::finish` is not possible.
            return Err(err.clone());
        }

        cmd_buf_data.push_with(|| -> Result<_, RenderPassError> {
            Ok(ArcCommand::RunRenderPass {
                pass: base?,
                color_attachments: SmallVec::from(pass.color_attachments.as_slice()),
                depth_stencil_attachment: pass.depth_stencil_attachment.take(),
                timestamp_writes: pass.timestamp_writes.take(),
                occlusion_query_set: pass.occlusion_query_set.take(),
                multiview_mask: pass.multiview_mask,
            })
        })
    }
}

pub(super) fn encode_render_pass(
    parent_state: &mut EncodingState<InnerCommandEncoder>,
    mut base: BasePass<ArcRenderCommand, Infallible>,
    color_attachments: ColorAttachments<Arc<TextureView>>,
    mut depth_stencil_attachment: Option<
        ResolvedRenderPassDepthStencilAttachment<Arc<TextureView>>,
    >,
    mut timestamp_writes: Option<ArcPassTimestampWrites>,
    occlusion_query_set: Option<Arc<QuerySet>>,
    multiview_mask: Option<NonZeroU32>,
) -> Result<(), RenderPassError> {
    let pass_scope = PassErrorScope::Pass;

    let device = parent_state.device;

    let mut indirect_draw_validation_batcher = crate::indirect_validation::DrawBatcher::new();

    // We automatically keep extending command buffers over time, and because
    // we want to insert a command buffer _before_ what we're about to record,
    // we need to make sure to close the previous one.
    parent_state
        .raw_encoder
        .close_if_open()
        .map_pass_err(pass_scope)?;
    let raw_encoder = parent_state
        .raw_encoder
        .open_pass(base.label.as_deref())
        .map_pass_err(pass_scope)?;

    let (scope, pending_discard_init_fixups, mut pending_query_resets) = {
        let mut pending_query_resets = QueryResetMap::new();
        let mut pending_discard_init_fixups = SurfacesInDiscardState::new();

        let info = RenderPassInfo::start(
            device,
            hal_label(base.label.as_deref(), device.instance_flags),
            &color_attachments,
            depth_stencil_attachment.take(),
            timestamp_writes.take(),
            // Still needed down the line.
            // TODO(wumpf): by restructuring the code, we could get rid of some of this Arc clone.
            occlusion_query_set.clone(),
            raw_encoder,
            parent_state.tracker,
            parent_state.texture_memory_actions,
            &mut pending_query_resets,
            &mut pending_discard_init_fixups,
            parent_state.snatch_guard,
            multiview_mask,
        )
        .map_pass_err(pass_scope)?;

        let indices = &device.tracker_indices;
        parent_state
            .tracker
            .buffers
            .set_size(indices.buffers.size());
        parent_state
            .tracker
            .textures
            .set_size(indices.textures.size());

        let mut debug_scope_depth = 0;

        let mut state = State {
            pipeline_flags: PipelineFlags::empty(),
            blend_constant: OptionalState::Unused,
            stencil_reference: 0,
            pipeline: None,
            index: IndexState::default(),
            vertex: VertexState::default(),

            info,

            pass: pass::PassState {
                base: EncodingState {
                    device,
                    raw_encoder,
                    tracker: parent_state.tracker,
                    buffer_memory_init_actions: parent_state.buffer_memory_init_actions,
                    texture_memory_actions: parent_state.texture_memory_actions,
                    as_actions: parent_state.as_actions,
                    temp_resources: parent_state.temp_resources,
                    indirect_draw_validation_resources: parent_state
                        .indirect_draw_validation_resources,
                    snatch_guard: parent_state.snatch_guard,
                    debug_scope_depth: &mut debug_scope_depth,
                },
                pending_discard_init_fixups,
                scope: device.new_usage_scope(),
                binder: Binder::new(),

                temp_offsets: Vec::new(),
                dynamic_offset_count: 0,

                string_offset: 0,
            },

            active_occlusion_query: None,
            active_pipeline_statistics_query: None,
        };

        for command in base.commands.drain(..) {
            match command {
                ArcRenderCommand::SetBindGroup {
                    index,
                    num_dynamic_offsets,
                    bind_group,
                } => {
                    let scope = PassErrorScope::SetBindGroup;
                    pass::set_bind_group::<RenderPassErrorInner>(
                        &mut state.pass,
                        device,
                        &base.dynamic_offsets,
                        index,
                        num_dynamic_offsets,
                        bind_group,
                        true,
                    )
                    .map_pass_err(scope)?;
                }
                ArcRenderCommand::SetPipeline(pipeline) => {
                    let scope = PassErrorScope::SetPipelineRender;
                    set_pipeline(&mut state, device, pipeline).map_pass_err(scope)?;
                }
                ArcRenderCommand::SetIndexBuffer {
                    buffer,
                    index_format,
                    offset,
                    size,
                } => {
                    let scope = PassErrorScope::SetIndexBuffer;
                    set_index_buffer(&mut state, device, buffer, index_format, offset, size)
                        .map_pass_err(scope)?;
                }
                ArcRenderCommand::SetVertexBuffer {
                    slot,
                    buffer,
                    offset,
                    size,
                } => {
                    let scope = PassErrorScope::SetVertexBuffer;
                    set_vertex_buffer(&mut state, device, slot, buffer, offset, size)
                        .map_pass_err(scope)?;
                }
                ArcRenderCommand::SetBlendConstant(ref color) => {
                    set_blend_constant(&mut state, color);
                }
                ArcRenderCommand::SetStencilReference(value) => {
                    set_stencil_reference(&mut state, value);
                }
                ArcRenderCommand::SetViewport {
                    rect,
                    depth_min,
                    depth_max,
                } => {
                    let scope = PassErrorScope::SetViewport;
                    set_viewport(&mut state, rect, depth_min, depth_max).map_pass_err(scope)?;
                }
                ArcRenderCommand::SetImmediate {
                    offset,
                    size_bytes,
                    values_offset,
                } => {
                    let scope = PassErrorScope::SetImmediate;
                    pass::set_immediates::<RenderPassErrorInner, _>(
                        &mut state.pass,
                        &base.immediates_data,
                        offset,
                        size_bytes,
                        values_offset,
                        |_| {},
                    )
                    .map_pass_err(scope)?;
                }
                ArcRenderCommand::SetScissor(rect) => {
                    let scope = PassErrorScope::SetScissorRect;
                    set_scissor(&mut state, rect).map_pass_err(scope)?;
                }
                ArcRenderCommand::Draw {
                    vertex_count,
                    instance_count,
                    first_vertex,
                    first_instance,
                } => {
                    let scope = PassErrorScope::Draw {
                        kind: DrawKind::Draw,
                        family: DrawCommandFamily::Draw,
                    };
                    draw(
                        &mut state,
                        vertex_count,
                        instance_count,
                        first_vertex,
                        first_instance,
                    )
                    .map_pass_err(scope)?;
                }
                ArcRenderCommand::DrawIndexed {
                    index_count,
                    instance_count,
                    first_index,
                    base_vertex,
                    first_instance,
                } => {
                    let scope = PassErrorScope::Draw {
                        kind: DrawKind::Draw,
                        family: DrawCommandFamily::DrawIndexed,
                    };
                    draw_indexed(
                        &mut state,
                        index_count,
                        instance_count,
                        first_index,
                        base_vertex,
                        first_instance,
                    )
                    .map_pass_err(scope)?;
                }
                ArcRenderCommand::DrawMeshTasks {
                    group_count_x,
                    group_count_y,
                    group_count_z,
                } => {
                    let scope = PassErrorScope::Draw {
                        kind: DrawKind::Draw,
                        family: DrawCommandFamily::DrawMeshTasks,
                    };
                    draw_mesh_tasks(&mut state, group_count_x, group_count_y, group_count_z)
                        .map_pass_err(scope)?;
                }
                ArcRenderCommand::DrawIndirect {
                    buffer,
                    offset,
                    count,
                    family,

                    vertex_or_index_limit: _,
                    instance_limit: _,
                } => {
                    let scope = PassErrorScope::Draw {
                        kind: if count != 1 {
                            DrawKind::MultiDrawIndirect
                        } else {
                            DrawKind::DrawIndirect
                        },
                        family,
                    };
                    multi_draw_indirect(
                        &mut state,
                        &mut indirect_draw_validation_batcher,
                        device,
                        buffer,
                        offset,
                        count,
                        family,
                    )
                    .map_pass_err(scope)?;
                }
                ArcRenderCommand::MultiDrawIndirectCount {
                    buffer,
                    offset,
                    count_buffer,
                    count_buffer_offset,
                    max_count,
                    family,
                } => {
                    let scope = PassErrorScope::Draw {
                        kind: DrawKind::MultiDrawIndirectCount,
                        family,
                    };
                    multi_draw_indirect_count(
                        &mut state,
                        device,
                        buffer,
                        offset,
                        count_buffer,
                        count_buffer_offset,
                        max_count,
                        family,
                    )
                    .map_pass_err(scope)?;
                }
                ArcRenderCommand::PushDebugGroup { color: _, len } => {
                    pass::push_debug_group(&mut state.pass, &base.string_data, len);
                }
                ArcRenderCommand::PopDebugGroup => {
                    let scope = PassErrorScope::PopDebugGroup;
                    pass::pop_debug_group::<RenderPassErrorInner>(&mut state.pass)
                        .map_pass_err(scope)?;
                }
                ArcRenderCommand::InsertDebugMarker { color: _, len } => {
                    pass::insert_debug_marker(&mut state.pass, &base.string_data, len);
                }
                ArcRenderCommand::WriteTimestamp {
                    query_set,
                    query_index,
                } => {
                    let scope = PassErrorScope::WriteTimestamp;
                    pass::write_timestamp::<RenderPassErrorInner>(
                        &mut state.pass,
                        device,
                        Some(&mut pending_query_resets),
                        query_set,
                        query_index,
                    )
                    .map_pass_err(scope)?;
                }
                ArcRenderCommand::BeginOcclusionQuery { query_index } => {
                    api_log!("RenderPass::begin_occlusion_query {query_index}");
                    let scope = PassErrorScope::BeginOcclusionQuery;

                    let query_set = occlusion_query_set
                        .clone()
                        .ok_or(RenderPassErrorInner::MissingOcclusionQuerySet)
                        .map_pass_err(scope)?;

                    validate_and_begin_occlusion_query(
                        query_set,
                        state.pass.base.raw_encoder,
                        &mut state.pass.base.tracker.query_sets,
                        query_index,
                        Some(&mut pending_query_resets),
                        &mut state.active_occlusion_query,
                    )
                    .map_pass_err(scope)?;
                }
                ArcRenderCommand::EndOcclusionQuery => {
                    api_log!("RenderPass::end_occlusion_query");
                    let scope = PassErrorScope::EndOcclusionQuery;

                    end_occlusion_query(
                        state.pass.base.raw_encoder,
                        &mut state.active_occlusion_query,
                    )
                    .map_pass_err(scope)?;
                }
                ArcRenderCommand::BeginPipelineStatisticsQuery {
                    query_set,
                    query_index,
                } => {
                    api_log!(
                        "RenderPass::begin_pipeline_statistics_query {query_index} {}",
                        query_set.error_ident()
                    );
                    let scope = PassErrorScope::BeginPipelineStatisticsQuery;

                    validate_and_begin_pipeline_statistics_query(
                        query_set,
                        state.pass.base.raw_encoder,
                        &mut state.pass.base.tracker.query_sets,
                        device,
                        query_index,
                        Some(&mut pending_query_resets),
                        &mut state.active_pipeline_statistics_query,
                    )
                    .map_pass_err(scope)?;
                }
                ArcRenderCommand::EndPipelineStatisticsQuery => {
                    api_log!("RenderPass::end_pipeline_statistics_query");
                    let scope = PassErrorScope::EndPipelineStatisticsQuery;

                    end_pipeline_statistics_query(
                        state.pass.base.raw_encoder,
                        &mut state.active_pipeline_statistics_query,
                    )
                    .map_pass_err(scope)?;
                }
                ArcRenderCommand::ExecuteBundle(bundle) => {
                    let scope = PassErrorScope::ExecuteBundle;
                    execute_bundle(
                        &mut state,
                        &mut indirect_draw_validation_batcher,
                        device,
                        bundle,
                    )
                    .map_pass_err(scope)?;
                }
            }
        }

        if *state.pass.base.debug_scope_depth > 0 {
            Err(
                RenderPassErrorInner::DebugGroupError(DebugGroupError::MissingPop)
                    .map_pass_err(pass_scope),
            )?;
        }
        if state.active_occlusion_query.is_some() {
            Err(RenderPassErrorInner::QueryUse(QueryUseError::MissingEnd {
                query_type: super::SimplifiedQueryType::Occlusion,
            })
            .map_pass_err(pass_scope))?;
        }
        if state.active_pipeline_statistics_query.is_some() {
            Err(RenderPassErrorInner::QueryUse(QueryUseError::MissingEnd {
                query_type: super::SimplifiedQueryType::PipelineStatistics,
            })
            .map_pass_err(pass_scope))?;
        }

        state
            .info
            .finish(
                device,
                state.pass.base.raw_encoder,
                state.pass.base.snatch_guard,
                &mut state.pass.scope,
                device.instance_flags,
            )
            .map_pass_err(pass_scope)?;

        let trackers = state.pass.scope;

        let pending_discard_init_fixups = state.pass.pending_discard_init_fixups;

        parent_state.raw_encoder.close().map_pass_err(pass_scope)?;
        (trackers, pending_discard_init_fixups, pending_query_resets)
    };

    let encoder = &mut parent_state.raw_encoder;
    let tracker = &mut parent_state.tracker;

    {
        let transit = encoder
            .open_pass(hal_label(
                Some("(wgpu internal) Pre Pass"),
                device.instance_flags,
            ))
            .map_pass_err(pass_scope)?;

        fixup_discarded_surfaces(
            pending_discard_init_fixups.into_iter(),
            transit,
            &mut tracker.textures,
            device,
            parent_state.snatch_guard,
        );

        pending_query_resets.reset_queries(transit);

        CommandEncoder::insert_barriers_from_scope(
            transit,
            tracker,
            &scope,
            parent_state.snatch_guard,
        );

        if let Some(ref indirect_validation) = device.indirect_validation {
            indirect_validation
                .draw
                .inject_validation_pass(
                    device,
                    parent_state.snatch_guard,
                    parent_state.indirect_draw_validation_resources,
                    parent_state.temp_resources,
                    transit,
                    indirect_draw_validation_batcher,
                )
                .map_pass_err(pass_scope)?;
        }
    }

    encoder.close_and_swap().map_pass_err(pass_scope)?;

    Ok(())
}

fn set_pipeline(
    state: &mut State,
    device: &Arc<Device>,
    pipeline: Arc<RenderPipeline>,
) -> Result<(), RenderPassErrorInner> {
    api_log!("RenderPass::set_pipeline {}", pipeline.error_ident());

    state.pipeline = Some(pipeline.clone());

    let pipeline = state
        .pass
        .base
        .tracker
        .render_pipelines
        .insert_single(pipeline)
        .clone();

    pipeline.same_device(device)?;

    state
        .info
        .context
        .check_compatible(&pipeline.pass_context, pipeline.as_ref())
        .map_err(RenderCommandError::IncompatiblePipelineTargets)?;

    state.pipeline_flags = pipeline.flags;

    if pipeline.flags.contains(PipelineFlags::WRITES_DEPTH) && state.info.is_depth_read_only {
        return Err(RenderCommandError::IncompatibleDepthAccess(pipeline.error_ident()).into());
    }
    if pipeline.flags.contains(PipelineFlags::WRITES_STENCIL) && state.info.is_stencil_read_only {
        return Err(RenderCommandError::IncompatibleStencilAccess(pipeline.error_ident()).into());
    }

    state
        .blend_constant
        .require(pipeline.flags.contains(PipelineFlags::BLEND_CONSTANT));

    unsafe {
        state
            .pass
            .base
            .raw_encoder
            .set_render_pipeline(pipeline.raw());
    }

    if pipeline.flags.contains(PipelineFlags::STENCIL_REFERENCE) {
        unsafe {
            state
                .pass
                .base
                .raw_encoder
                .set_stencil_reference(state.stencil_reference);
        }
    }

    // Rebind resource
    pass::change_pipeline_layout::<RenderPassErrorInner, _>(
        &mut state.pass,
        &pipeline.layout,
        &pipeline.late_sized_buffer_groups,
        || {},
    )?;

    // Update vertex buffer limits.
    state.vertex.update_limits(&pipeline.vertex_steps);
    Ok(())
}

// This function is duplicative of `bundle::set_index_buffer`.
fn set_index_buffer(
    state: &mut State,
    device: &Arc<Device>,
    buffer: Arc<crate::resource::Buffer>,
    index_format: IndexFormat,
    offset: u64,
    size: Option<BufferSize>,
) -> Result<(), RenderPassErrorInner> {
    api_log!("RenderPass::set_index_buffer {}", buffer.error_ident());

    state
        .pass
        .scope
        .buffers
        .merge_single(&buffer, wgt::BufferUses::INDEX)?;

    buffer.same_device(device)?;

    buffer.check_usage(BufferUsages::INDEX)?;

    if !offset.is_multiple_of(u64::try_from(index_format.byte_size()).unwrap()) {
        return Err(RenderCommandError::UnalignedIndexBuffer {
            offset,
            alignment: index_format.byte_size(),
        }
        .into());
    }
    let (binding, resolved_size) = buffer
        .binding(offset, size, state.pass.base.snatch_guard)
        .map_err(RenderCommandError::from)?;
    let end = offset + resolved_size;
    state.index.update_buffer(offset..end, index_format);

    state.pass.base.buffer_memory_init_actions.extend(
        buffer.initialization_status.read().create_action(
            &buffer,
            offset..end,
            MemoryInitKind::NeedsInitializedMemory,
        ),
    );

    unsafe {
        hal::DynCommandEncoder::set_index_buffer(
            state.pass.base.raw_encoder,
            binding,
            index_format,
        );
    }
    Ok(())
}

// This function is duplicative of `render::set_vertex_buffer`.
fn set_vertex_buffer(
    state: &mut State,
    device: &Arc<Device>,
    slot: u32,
    buffer: Arc<crate::resource::Buffer>,
    offset: u64,
    size: Option<BufferSize>,
) -> Result<(), RenderPassErrorInner> {
    api_log!(
        "RenderPass::set_vertex_buffer {slot} {}",
        buffer.error_ident()
    );

    state
        .pass
        .scope
        .buffers
        .merge_single(&buffer, wgt::BufferUses::VERTEX)?;

    buffer.same_device(device)?;

    let max_vertex_buffers = state.pass.base.device.limits.max_vertex_buffers;
    if slot >= max_vertex_buffers {
        return Err(RenderCommandError::VertexBufferIndexOutOfRange {
            index: slot,
            max: max_vertex_buffers,
        }
        .into());
    }

    buffer.check_usage(BufferUsages::VERTEX)?;

    if !offset.is_multiple_of(wgt::VERTEX_ALIGNMENT) {
        return Err(RenderCommandError::UnalignedVertexBuffer { slot, offset }.into());
    }
    let (binding, buffer_size) = buffer
        .binding(offset, size, state.pass.base.snatch_guard)
        .map_err(RenderCommandError::from)?;
    state.vertex.buffer_sizes[slot as usize] = Some(buffer_size);

    state.pass.base.buffer_memory_init_actions.extend(
        buffer.initialization_status.read().create_action(
            &buffer,
            offset..(offset + buffer_size),
            MemoryInitKind::NeedsInitializedMemory,
        ),
    );

    unsafe {
        hal::DynCommandEncoder::set_vertex_buffer(state.pass.base.raw_encoder, slot, binding);
    }
    if let Some(pipeline) = state.pipeline.as_ref() {
        state.vertex.update_limits(&pipeline.vertex_steps);
    }
    Ok(())
}

fn set_blend_constant(state: &mut State, color: &Color) {
    api_log!("RenderPass::set_blend_constant");

    state.blend_constant = OptionalState::Set;
    let array = [
        color.r as f32,
        color.g as f32,
        color.b as f32,
        color.a as f32,
    ];
    unsafe {
        state.pass.base.raw_encoder.set_blend_constants(&array);
    }
}

fn set_stencil_reference(state: &mut State, value: u32) {
    api_log!("RenderPass::set_stencil_reference {value}");

    state.stencil_reference = value;
    if state
        .pipeline_flags
        .contains(PipelineFlags::STENCIL_REFERENCE)
    {
        unsafe {
            state.pass.base.raw_encoder.set_stencil_reference(value);
        }
    }
}

fn set_viewport(
    state: &mut State,
    rect: Rect<f32>,
    depth_min: f32,
    depth_max: f32,
) -> Result<(), RenderPassErrorInner> {
    api_log!("RenderPass::set_viewport {rect:?}");

    if rect.w < 0.0
        || rect.h < 0.0
        || rect.w > state.pass.base.device.limits.max_texture_dimension_2d as f32
        || rect.h > state.pass.base.device.limits.max_texture_dimension_2d as f32
    {
        return Err(RenderCommandError::InvalidViewportRectSize {
            w: rect.w,
            h: rect.h,
            max: state.pass.base.device.limits.max_texture_dimension_2d,
        }
        .into());
    }

    let max_viewport_range = state.pass.base.device.limits.max_texture_dimension_2d as f32 * 2.0;

    if rect.x < -max_viewport_range
        || rect.y < -max_viewport_range
        || rect.x + rect.w > max_viewport_range - 1.0
        || rect.y + rect.h > max_viewport_range - 1.0
    {
        return Err(RenderCommandError::InvalidViewportRectPosition {
            rect,
            min: -max_viewport_range,
            max: max_viewport_range - 1.0,
        }
        .into());
    }
    if !(0.0..=1.0).contains(&depth_min)
        || !(0.0..=1.0).contains(&depth_max)
        || depth_min > depth_max
    {
        return Err(RenderCommandError::InvalidViewportDepth(depth_min, depth_max).into());
    }
    let r = hal::Rect {
        x: rect.x,
        y: rect.y,
        w: rect.w,
        h: rect.h,
    };
    unsafe {
        state
            .pass
            .base
            .raw_encoder
            .set_viewport(&r, depth_min..depth_max);
    }
    Ok(())
}

fn set_scissor(state: &mut State, rect: Rect<u32>) -> Result<(), RenderPassErrorInner> {
    api_log!("RenderPass::set_scissor_rect {rect:?}");

    if rect.x.saturating_add(rect.w) > state.info.extent.width
        || rect.y.saturating_add(rect.h) > state.info.extent.height
    {
        return Err(RenderCommandError::InvalidScissorRect(rect, state.info.extent).into());
    }
    let r = hal::Rect {
        x: rect.x,
        y: rect.y,
        w: rect.w,
        h: rect.h,
    };
    unsafe {
        state.pass.base.raw_encoder.set_scissor_rect(&r);
    }
    Ok(())
}

fn validate_mesh_draw_multiview(state: &State) -> Result<(), RenderPassErrorInner> {
    if let Some(mv) = state.info.multiview_mask {
        let highest_bit = 31 - mv.leading_zeros();

        let features = state.pass.base.device.features;

        if !features.contains(wgt::Features::EXPERIMENTAL_MESH_SHADER_MULTIVIEW)
            || highest_bit > state.pass.base.device.limits.max_mesh_multiview_view_count
        {
            return Err(RenderPassErrorInner::Draw(
                DrawError::MeshPipelineMultiviewLimitsViolated {
                    highest_view_index: highest_bit,
                    max_multiviews: state.pass.base.device.limits.max_mesh_multiview_view_count,
                },
            ));
        }
    }
    Ok(())
}

fn draw(
    state: &mut State,
    vertex_count: u32,
    instance_count: u32,
    first_vertex: u32,
    first_instance: u32,
) -> Result<(), RenderPassErrorInner> {
    api_log!("RenderPass::draw {vertex_count} {instance_count} {first_vertex} {first_instance}");

    state.is_ready(DrawCommandFamily::Draw)?;
    state.flush_bindings()?;

    state
        .vertex
        .limits
        .validate_vertex_limit(first_vertex, vertex_count)?;
    state
        .vertex
        .limits
        .validate_instance_limit(first_instance, instance_count)?;

    unsafe {
        if instance_count > 0 && vertex_count > 0 {
            state.pass.base.raw_encoder.draw(
                first_vertex,
                vertex_count,
                first_instance,
                instance_count,
            );
        }
    }
    Ok(())
}

fn draw_indexed(
    state: &mut State,
    index_count: u32,
    instance_count: u32,
    first_index: u32,
    base_vertex: i32,
    first_instance: u32,
) -> Result<(), RenderPassErrorInner> {
    api_log!("RenderPass::draw_indexed {index_count} {instance_count} {first_index} {base_vertex} {first_instance}");

    state.is_ready(DrawCommandFamily::DrawIndexed)?;
    state.flush_bindings()?;

    let last_index = first_index as u64 + index_count as u64;
    let index_limit = state.index.limit;
    if last_index > index_limit {
        return Err(DrawError::IndexBeyondLimit {
            last_index,
            index_limit,
        }
        .into());
    }
    state
        .vertex
        .limits
        .validate_instance_limit(first_instance, instance_count)?;

    unsafe {
        if instance_count > 0 && index_count > 0 {
            state.pass.base.raw_encoder.draw_indexed(
                first_index,
                index_count,
                base_vertex,
                first_instance,
                instance_count,
            );
        }
    }
    Ok(())
}

fn draw_mesh_tasks(
    state: &mut State,
    group_count_x: u32,
    group_count_y: u32,
    group_count_z: u32,
) -> Result<(), RenderPassErrorInner> {
    api_log!("RenderPass::draw_mesh_tasks {group_count_x} {group_count_y} {group_count_z}");

    state.is_ready(DrawCommandFamily::DrawMeshTasks)?;

    state.flush_bindings()?;
    validate_mesh_draw_multiview(state)?;

    let groups_size_limit = state
        .pass
        .base
        .device
        .limits
        .max_task_mesh_workgroups_per_dimension;
    let max_groups = state
        .pass
        .base
        .device
        .limits
        .max_task_mesh_workgroup_total_count;
    if group_count_x > groups_size_limit
        || group_count_y > groups_size_limit
        || group_count_z > groups_size_limit
        || group_count_x * group_count_y * group_count_z > max_groups
    {
        return Err(DrawError::InvalidGroupSize {
            current: [group_count_x, group_count_y, group_count_z],
            limit: groups_size_limit,
            max_total: max_groups,
        }
        .into());
    }

    unsafe {
        if group_count_x > 0 && group_count_y > 0 && group_count_z > 0 {
            state.pass.base.raw_encoder.draw_mesh_tasks(
                group_count_x,
                group_count_y,
                group_count_z,
            );
        }
    }
    Ok(())
}

fn multi_draw_indirect(
    state: &mut State,
    indirect_draw_validation_batcher: &mut crate::indirect_validation::DrawBatcher,
    device: &Arc<Device>,
    indirect_buffer: Arc<crate::resource::Buffer>,
    offset: u64,
    count: u32,
    family: DrawCommandFamily,
) -> Result<(), RenderPassErrorInner> {
    api_log!(
        "RenderPass::draw_indirect (family:{family:?}) {} {offset} {count:?}",
        indirect_buffer.error_ident()
    );

    state.is_ready(family)?;
    state.flush_bindings()?;

    if family == DrawCommandFamily::DrawMeshTasks {
        validate_mesh_draw_multiview(state)?;
    }

    state
        .pass
        .base
        .device
        .require_downlevel_flags(wgt::DownlevelFlags::INDIRECT_EXECUTION)?;

    indirect_buffer.same_device(device)?;
    indirect_buffer.check_usage(BufferUsages::INDIRECT)?;
    indirect_buffer.check_destroyed(state.pass.base.snatch_guard)?;

    if !offset.is_multiple_of(4) {
        return Err(RenderPassErrorInner::UnalignedIndirectBufferOffset(offset));
    }

    let stride = get_src_stride_of_indirect_args(family);

    let end_offset = offset + stride * count as u64;
    if end_offset > indirect_buffer.size {
        return Err(RenderPassErrorInner::IndirectBufferOverrun {
            count,
            offset,
            end_offset,
            buffer_size: indirect_buffer.size,
        });
    }

    state.pass.base.buffer_memory_init_actions.extend(
        indirect_buffer.initialization_status.read().create_action(
            &indirect_buffer,
            offset..end_offset,
            MemoryInitKind::NeedsInitializedMemory,
        ),
    );

    fn draw(
        raw_encoder: &mut dyn hal::DynCommandEncoder,
        family: DrawCommandFamily,
        indirect_buffer: &dyn hal::DynBuffer,
        offset: u64,
        count: u32,
    ) {
        match family {
            DrawCommandFamily::Draw => unsafe {
                raw_encoder.draw_indirect(indirect_buffer, offset, count);
            },
            DrawCommandFamily::DrawIndexed => unsafe {
                raw_encoder.draw_indexed_indirect(indirect_buffer, offset, count);
            },
            DrawCommandFamily::DrawMeshTasks => unsafe {
                raw_encoder.draw_mesh_tasks_indirect(indirect_buffer, offset, count);
            },
        }
    }

    if state.pass.base.device.indirect_validation.is_some() {
        state
            .pass
            .scope
            .buffers
            .merge_single(&indirect_buffer, wgt::BufferUses::STORAGE_READ_ONLY)?;

        struct DrawData {
            buffer_index: usize,
            offset: u64,
            count: u32,
        }

        struct DrawContext<'a> {
            raw_encoder: &'a mut dyn hal::DynCommandEncoder,
            device: &'a Device,

            indirect_draw_validation_resources: &'a mut crate::indirect_validation::DrawResources,
            indirect_draw_validation_batcher: &'a mut crate::indirect_validation::DrawBatcher,

            indirect_buffer: Arc<crate::resource::Buffer>,
            family: DrawCommandFamily,
            vertex_or_index_limit: u64,
            instance_limit: u64,
        }

        impl<'a> DrawContext<'a> {
            fn add(&mut self, offset: u64) -> Result<DrawData, DeviceError> {
                let (dst_resource_index, dst_offset) = self.indirect_draw_validation_batcher.add(
                    self.indirect_draw_validation_resources,
                    self.device,
                    &self.indirect_buffer,
                    offset,
                    self.family,
                    self.vertex_or_index_limit,
                    self.instance_limit,
                )?;
                Ok(DrawData {
                    buffer_index: dst_resource_index,
                    offset: dst_offset,
                    count: 1,
                })
            }
            fn draw(&mut self, draw_data: DrawData) {
                let dst_buffer = self
                    .indirect_draw_validation_resources
                    .get_dst_buffer(draw_data.buffer_index);
                draw(
                    self.raw_encoder,
                    self.family,
                    dst_buffer,
                    draw_data.offset,
                    draw_data.count,
                );
            }
        }

        let mut draw_ctx = DrawContext {
            raw_encoder: state.pass.base.raw_encoder,
            device: state.pass.base.device,
            indirect_draw_validation_resources: state.pass.base.indirect_draw_validation_resources,
            indirect_draw_validation_batcher,
            indirect_buffer,
            family,
            vertex_or_index_limit: if family == DrawCommandFamily::DrawIndexed {
                state.index.limit
            } else {
                state.vertex.limits.vertex_limit
            },
            instance_limit: state.vertex.limits.instance_limit,
        };

        let mut current_draw_data = draw_ctx.add(offset)?;

        for i in 1..count {
            let draw_data = draw_ctx.add(offset + stride * i as u64)?;

            if draw_data.buffer_index == current_draw_data.buffer_index {
                #[cfg(debug_assertions)]
                {
                    let dst_stride =
                        get_dst_stride_of_indirect_args(state.pass.base.device.backend(), family);
                    debug_assert_eq!(
                        draw_data.offset,
                        current_draw_data.offset + dst_stride * current_draw_data.count as u64
                    );
                }
                current_draw_data.count += 1;
            } else {
                draw_ctx.draw(current_draw_data);
                current_draw_data = draw_data;
            }
        }

        draw_ctx.draw(current_draw_data);
    } else {
        state
            .pass
            .scope
            .buffers
            .merge_single(&indirect_buffer, wgt::BufferUses::INDIRECT)?;

        draw(
            state.pass.base.raw_encoder,
            family,
            indirect_buffer.try_raw(state.pass.base.snatch_guard)?,
            offset,
            count,
        );
    };

    Ok(())
}

fn multi_draw_indirect_count(
    state: &mut State,
    device: &Arc<Device>,
    indirect_buffer: Arc<crate::resource::Buffer>,
    offset: u64,
    count_buffer: Arc<crate::resource::Buffer>,
    count_buffer_offset: u64,
    max_count: u32,
    family: DrawCommandFamily,
) -> Result<(), RenderPassErrorInner> {
    api_log!(
        "RenderPass::multi_draw_indirect_count (family:{family:?}) {} {offset} {} {count_buffer_offset:?} {max_count:?}",
        indirect_buffer.error_ident(),
        count_buffer.error_ident()
    );

    state.is_ready(family)?;
    state.flush_bindings()?;

    if family == DrawCommandFamily::DrawMeshTasks {
        validate_mesh_draw_multiview(state)?;
    }

    let stride = get_src_stride_of_indirect_args(family);

    state
        .pass
        .base
        .device
        .require_features(wgt::Features::MULTI_DRAW_INDIRECT_COUNT)?;
    state
        .pass
        .base
        .device
        .require_downlevel_flags(wgt::DownlevelFlags::INDIRECT_EXECUTION)?;

    indirect_buffer.same_device(device)?;
    count_buffer.same_device(device)?;

    state
        .pass
        .scope
        .buffers
        .merge_single(&indirect_buffer, wgt::BufferUses::INDIRECT)?;

    indirect_buffer.check_usage(BufferUsages::INDIRECT)?;
    let indirect_raw = indirect_buffer.try_raw(state.pass.base.snatch_guard)?;

    state
        .pass
        .scope
        .buffers
        .merge_single(&count_buffer, wgt::BufferUses::INDIRECT)?;

    count_buffer.check_usage(BufferUsages::INDIRECT)?;
    let count_raw = count_buffer.try_raw(state.pass.base.snatch_guard)?;

    if !offset.is_multiple_of(4) {
        return Err(RenderPassErrorInner::UnalignedIndirectBufferOffset(offset));
    }

    let end_offset = offset + stride * max_count as u64;
    if end_offset > indirect_buffer.size {
        return Err(RenderPassErrorInner::IndirectBufferOverrun {
            count: 1,
            offset,
            end_offset,
            buffer_size: indirect_buffer.size,
        });
    }
    state.pass.base.buffer_memory_init_actions.extend(
        indirect_buffer.initialization_status.read().create_action(
            &indirect_buffer,
            offset..end_offset,
            MemoryInitKind::NeedsInitializedMemory,
        ),
    );

    let begin_count_offset = count_buffer_offset;
    let end_count_offset = count_buffer_offset + 4;
    if end_count_offset > count_buffer.size {
        return Err(RenderPassErrorInner::IndirectCountBufferOverrun {
            begin_count_offset,
            end_count_offset,
            count_buffer_size: count_buffer.size,
        });
    }
    state.pass.base.buffer_memory_init_actions.extend(
        count_buffer.initialization_status.read().create_action(
            &count_buffer,
            count_buffer_offset..end_count_offset,
            MemoryInitKind::NeedsInitializedMemory,
        ),
    );

    match family {
        DrawCommandFamily::Draw => unsafe {
            state.pass.base.raw_encoder.draw_indirect_count(
                indirect_raw,
                offset,
                count_raw,
                count_buffer_offset,
                max_count,
            );
        },
        DrawCommandFamily::DrawIndexed => unsafe {
            state.pass.base.raw_encoder.draw_indexed_indirect_count(
                indirect_raw,
                offset,
                count_raw,
                count_buffer_offset,
                max_count,
            );
        },
        DrawCommandFamily::DrawMeshTasks => unsafe {
            state.pass.base.raw_encoder.draw_mesh_tasks_indirect_count(
                indirect_raw,
                offset,
                count_raw,
                count_buffer_offset,
                max_count,
            );
        },
    }
    Ok(())
}

fn execute_bundle(
    state: &mut State,
    indirect_draw_validation_batcher: &mut crate::indirect_validation::DrawBatcher,
    device: &Arc<Device>,
    bundle: Arc<super::RenderBundle>,
) -> Result<(), RenderPassErrorInner> {
    api_log!("RenderPass::execute_bundle {}", bundle.error_ident());

    let bundle = state.pass.base.tracker.bundles.insert_single(bundle);

    bundle.same_device(device)?;

    state
        .info
        .context
        .check_compatible(&bundle.context, bundle.as_ref())
        .map_err(RenderPassErrorInner::IncompatibleBundleTargets)?;

    if (state.info.is_depth_read_only && !bundle.is_depth_read_only)
        || (state.info.is_stencil_read_only && !bundle.is_stencil_read_only)
    {
        return Err(
            RenderPassErrorInner::IncompatibleBundleReadOnlyDepthStencil {
                pass_depth: state.info.is_depth_read_only,
                pass_stencil: state.info.is_stencil_read_only,
                bundle_depth: bundle.is_depth_read_only,
                bundle_stencil: bundle.is_stencil_read_only,
            },
        );
    }

    state.pass.base.buffer_memory_init_actions.extend(
        bundle
            .buffer_memory_init_actions
            .iter()
            .filter_map(|action| {
                action
                    .buffer
                    .initialization_status
                    .read()
                    .check_action(action)
            }),
    );
    for action in bundle.texture_memory_init_actions.iter() {
        state.pass.pending_discard_init_fixups.extend(
            state
                .pass
                .base
                .texture_memory_actions
                .register_init_action(action),
        );
    }

    unsafe {
        bundle.execute(
            state.pass.base.raw_encoder,
            state.pass.base.indirect_draw_validation_resources,
            indirect_draw_validation_batcher,
            state.pass.base.snatch_guard,
        )
    }
    .map_err(|e| match e {
        ExecutionError::Device(e) => RenderPassErrorInner::Device(e),
        ExecutionError::DestroyedResource(e) => {
            RenderPassErrorInner::RenderCommand(RenderCommandError::DestroyedResource(e))
        }
        ExecutionError::Unimplemented(what) => {
            RenderPassErrorInner::RenderCommand(RenderCommandError::Unimplemented(what))
        }
    })?;

    unsafe {
        state.pass.scope.merge_render_bundle(&bundle.used)?;
    };
    state.reset_bundle();
    Ok(())
}

// Recording a render pass.
//
// The only error that should be returned from these methods is
// `EncoderStateError::Ended`, when the pass has already ended and an immediate
// validation error is raised.
//
// All other errors should be stored in the pass for later reporting when
// `CommandEncoder.finish()` is called.
//
// The `pass_try!` macro should be used to handle errors appropriately. Note
// that the `pass_try!` and `pass_base!` macros may return early from the
// function that invokes them, like the `?` operator.
impl Global {
    pub fn render_pass_set_bind_group(
        &self,
        pass: &mut RenderPass,
        index: u32,
        bind_group_id: Option<id::BindGroupId>,
        offsets: &[DynamicOffset],
    ) -> Result<(), PassStateError> {
        let scope = PassErrorScope::SetBindGroup;

        // This statement will return an error if the pass is ended. It's
        // important the error check comes before the early-out for
        // `set_and_check_redundant`.
        let base = pass_base!(pass, scope);

        if pass.current_bind_groups.set_and_check_redundant(
            bind_group_id,
            index,
            &mut base.dynamic_offsets,
            offsets,
        ) {
            return Ok(());
        }

        let mut bind_group = None;
        if let Some(bind_group_id) = bind_group_id {
            let hub = &self.hub;
            bind_group = Some(pass_try!(
                base,
                scope,
                hub.bind_groups.get(bind_group_id).get(),
            ));
        }

        base.commands.push(ArcRenderCommand::SetBindGroup {
            index,
            num_dynamic_offsets: offsets.len(),
            bind_group,
        });

        Ok(())
    }

    pub fn render_pass_set_pipeline(
        &self,
        pass: &mut RenderPass,
        pipeline_id: id::RenderPipelineId,
    ) -> Result<(), PassStateError> {
        let scope = PassErrorScope::SetPipelineRender;

        let redundant = pass.current_pipeline.set_and_check_redundant(pipeline_id);

        // This statement will return an error if the pass is ended.
        // Its important the error check comes before the early-out for `redundant`.
        let base = pass_base!(pass, scope);

        if redundant {
            return Ok(());
        }

        let hub = &self.hub;
        let pipeline = pass_try!(base, scope, hub.render_pipelines.get(pipeline_id).get());

        base.commands.push(ArcRenderCommand::SetPipeline(pipeline));

        Ok(())
    }

    pub fn render_pass_set_index_buffer(
        &self,
        pass: &mut RenderPass,
        buffer_id: id::BufferId,
        index_format: IndexFormat,
        offset: BufferAddress,
        size: Option<BufferSize>,
    ) -> Result<(), PassStateError> {
        let scope = PassErrorScope::SetIndexBuffer;
        let base = pass_base!(pass, scope);

        base.commands.push(ArcRenderCommand::SetIndexBuffer {
            buffer: pass_try!(base, scope, self.resolve_buffer_id(buffer_id)),
            index_format,
            offset,
            size,
        });

        Ok(())
    }

    pub fn render_pass_set_vertex_buffer(
        &self,
        pass: &mut RenderPass,
        slot: u32,
        buffer_id: id::BufferId,
        offset: BufferAddress,
        size: Option<BufferSize>,
    ) -> Result<(), PassStateError> {
        let scope = PassErrorScope::SetVertexBuffer;
        let base = pass_base!(pass, scope);

        base.commands.push(ArcRenderCommand::SetVertexBuffer {
            slot,
            buffer: pass_try!(base, scope, self.resolve_buffer_id(buffer_id)),
            offset,
            size,
        });

        Ok(())
    }

    pub fn render_pass_set_blend_constant(
        &self,
        pass: &mut RenderPass,
        color: Color,
    ) -> Result<(), PassStateError> {
        let scope = PassErrorScope::SetBlendConstant;
        let base = pass_base!(pass, scope);

        base.commands
            .push(ArcRenderCommand::SetBlendConstant(color));

        Ok(())
    }

    pub fn render_pass_set_stencil_reference(
        &self,
        pass: &mut RenderPass,
        value: u32,
    ) -> Result<(), PassStateError> {
        let scope = PassErrorScope::SetStencilReference;
        let base = pass_base!(pass, scope);

        base.commands
            .push(ArcRenderCommand::SetStencilReference(value));

        Ok(())
    }

    pub fn render_pass_set_viewport(
        &self,
        pass: &mut RenderPass,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        depth_min: f32,
        depth_max: f32,
    ) -> Result<(), PassStateError> {
        let scope = PassErrorScope::SetViewport;
        let base = pass_base!(pass, scope);

        base.commands.push(ArcRenderCommand::SetViewport {
            rect: Rect { x, y, w, h },
            depth_min,
            depth_max,
        });

        Ok(())
    }

    pub fn render_pass_set_scissor_rect(
        &self,
        pass: &mut RenderPass,
        x: u32,
        y: u32,
        w: u32,
        h: u32,
    ) -> Result<(), PassStateError> {
        let scope = PassErrorScope::SetScissorRect;
        let base = pass_base!(pass, scope);

        base.commands
            .push(ArcRenderCommand::SetScissor(Rect { x, y, w, h }));

        Ok(())
    }

    pub fn render_pass_set_immediates(
        &self,
        pass: &mut RenderPass,
        offset: u32,
        data: &[u8],
    ) -> Result<(), PassStateError> {
        let scope = PassErrorScope::SetImmediate;
        let base = pass_base!(pass, scope);

        if offset & (wgt::IMMEDIATE_DATA_ALIGNMENT - 1) != 0 {
            pass_try!(
                base,
                scope,
                Err(RenderPassErrorInner::ImmediateOffsetAlignment)
            );
        }
        if data.len() as u32 & (wgt::IMMEDIATE_DATA_ALIGNMENT - 1) != 0 {
            pass_try!(
                base,
                scope,
                Err(RenderPassErrorInner::ImmediateDataizeAlignment)
            );
        }

        let value_offset = pass_try!(
            base,
            scope,
            base.immediates_data
                .len()
                .try_into()
                .map_err(|_| RenderPassErrorInner::ImmediateOutOfMemory),
        );

        base.immediates_data.extend(
            data.chunks_exact(wgt::IMMEDIATE_DATA_ALIGNMENT as usize)
                .map(|arr| u32::from_ne_bytes([arr[0], arr[1], arr[2], arr[3]])),
        );

        base.commands.push(ArcRenderCommand::SetImmediate {
            offset,
            size_bytes: data.len() as u32,
            values_offset: Some(value_offset),
        });

        Ok(())
    }

    pub fn render_pass_draw(
        &self,
        pass: &mut RenderPass,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) -> Result<(), PassStateError> {
        let scope = PassErrorScope::Draw {
            kind: DrawKind::Draw,
            family: DrawCommandFamily::Draw,
        };
        let base = pass_base!(pass, scope);

        base.commands.push(ArcRenderCommand::Draw {
            vertex_count,
            instance_count,
            first_vertex,
            first_instance,
        });

        Ok(())
    }

    pub fn render_pass_draw_indexed(
        &self,
        pass: &mut RenderPass,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        base_vertex: i32,
        first_instance: u32,
    ) -> Result<(), PassStateError> {
        let scope = PassErrorScope::Draw {
            kind: DrawKind::Draw,
            family: DrawCommandFamily::DrawIndexed,
        };
        let base = pass_base!(pass, scope);

        base.commands.push(ArcRenderCommand::DrawIndexed {
            index_count,
            instance_count,
            first_index,
            base_vertex,
            first_instance,
        });

        Ok(())
    }

    pub fn render_pass_draw_mesh_tasks(
        &self,
        pass: &mut RenderPass,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) -> Result<(), RenderPassError> {
        let scope = PassErrorScope::Draw {
            kind: DrawKind::Draw,
            family: DrawCommandFamily::DrawMeshTasks,
        };
        let base = pass_base!(pass, scope);

        base.commands.push(ArcRenderCommand::DrawMeshTasks {
            group_count_x,
            group_count_y,
            group_count_z,
        });
        Ok(())
    }

    pub fn render_pass_draw_indirect(
        &self,
        pass: &mut RenderPass,
        buffer_id: id::BufferId,
        offset: BufferAddress,
    ) -> Result<(), PassStateError> {
        let scope = PassErrorScope::Draw {
            kind: DrawKind::DrawIndirect,
            family: DrawCommandFamily::Draw,
        };
        let base = pass_base!(pass, scope);

        base.commands.push(ArcRenderCommand::DrawIndirect {
            buffer: pass_try!(base, scope, self.resolve_buffer_id(buffer_id)),
            offset,
            count: 1,
            family: DrawCommandFamily::Draw,

            vertex_or_index_limit: None,
            instance_limit: None,
        });

        Ok(())
    }

    pub fn render_pass_draw_indexed_indirect(
        &self,
        pass: &mut RenderPass,
        buffer_id: id::BufferId,
        offset: BufferAddress,
    ) -> Result<(), PassStateError> {
        let scope = PassErrorScope::Draw {
            kind: DrawKind::DrawIndirect,
            family: DrawCommandFamily::DrawIndexed,
        };
        let base = pass_base!(pass, scope);

        base.commands.push(ArcRenderCommand::DrawIndirect {
            buffer: pass_try!(base, scope, self.resolve_buffer_id(buffer_id)),
            offset,
            count: 1,
            family: DrawCommandFamily::DrawIndexed,

            vertex_or_index_limit: None,
            instance_limit: None,
        });

        Ok(())
    }

    pub fn render_pass_draw_mesh_tasks_indirect(
        &self,
        pass: &mut RenderPass,
        buffer_id: id::BufferId,
        offset: BufferAddress,
    ) -> Result<(), RenderPassError> {
        let scope = PassErrorScope::Draw {
            kind: DrawKind::DrawIndirect,
            family: DrawCommandFamily::DrawMeshTasks,
        };
        let base = pass_base!(pass, scope);

        base.commands.push(ArcRenderCommand::DrawIndirect {
            buffer: pass_try!(base, scope, self.resolve_buffer_id(buffer_id)),
            offset,
            count: 1,
            family: DrawCommandFamily::DrawMeshTasks,

            vertex_or_index_limit: None,
            instance_limit: None,
        });

        Ok(())
    }

    pub fn render_pass_multi_draw_indirect(
        &self,
        pass: &mut RenderPass,
        buffer_id: id::BufferId,
        offset: BufferAddress,
        count: u32,
    ) -> Result<(), PassStateError> {
        let scope = PassErrorScope::Draw {
            kind: DrawKind::MultiDrawIndirect,
            family: DrawCommandFamily::Draw,
        };
        let base = pass_base!(pass, scope);

        base.commands.push(ArcRenderCommand::DrawIndirect {
            buffer: pass_try!(base, scope, self.resolve_buffer_id(buffer_id)),
            offset,
            count,
            family: DrawCommandFamily::Draw,

            vertex_or_index_limit: None,
            instance_limit: None,
        });

        Ok(())
    }

    pub fn render_pass_multi_draw_indexed_indirect(
        &self,
        pass: &mut RenderPass,
        buffer_id: id::BufferId,
        offset: BufferAddress,
        count: u32,
    ) -> Result<(), PassStateError> {
        let scope = PassErrorScope::Draw {
            kind: DrawKind::MultiDrawIndirect,
            family: DrawCommandFamily::DrawIndexed,
        };
        let base = pass_base!(pass, scope);

        base.commands.push(ArcRenderCommand::DrawIndirect {
            buffer: pass_try!(base, scope, self.resolve_buffer_id(buffer_id)),
            offset,
            count,
            family: DrawCommandFamily::DrawIndexed,

            vertex_or_index_limit: None,
            instance_limit: None,
        });

        Ok(())
    }

    pub fn render_pass_multi_draw_mesh_tasks_indirect(
        &self,
        pass: &mut RenderPass,
        buffer_id: id::BufferId,
        offset: BufferAddress,
        count: u32,
    ) -> Result<(), RenderPassError> {
        let scope = PassErrorScope::Draw {
            kind: DrawKind::MultiDrawIndirect,
            family: DrawCommandFamily::DrawMeshTasks,
        };
        let base = pass_base!(pass, scope);

        base.commands.push(ArcRenderCommand::DrawIndirect {
            buffer: pass_try!(base, scope, self.resolve_buffer_id(buffer_id)),
            offset,
            count,
            family: DrawCommandFamily::DrawMeshTasks,

            vertex_or_index_limit: None,
            instance_limit: None,
        });

        Ok(())
    }

    pub fn render_pass_multi_draw_indirect_count(
        &self,
        pass: &mut RenderPass,
        buffer_id: id::BufferId,
        offset: BufferAddress,
        count_buffer_id: id::BufferId,
        count_buffer_offset: BufferAddress,
        max_count: u32,
    ) -> Result<(), PassStateError> {
        let scope = PassErrorScope::Draw {
            kind: DrawKind::MultiDrawIndirectCount,
            family: DrawCommandFamily::Draw,
        };
        let base = pass_base!(pass, scope);

        base.commands
            .push(ArcRenderCommand::MultiDrawIndirectCount {
                buffer: pass_try!(base, scope, self.resolve_buffer_id(buffer_id)),
                offset,
                count_buffer: pass_try!(base, scope, self.resolve_buffer_id(count_buffer_id)),
                count_buffer_offset,
                max_count,
                family: DrawCommandFamily::Draw,
            });

        Ok(())
    }

    pub fn render_pass_multi_draw_indexed_indirect_count(
        &self,
        pass: &mut RenderPass,
        buffer_id: id::BufferId,
        offset: BufferAddress,
        count_buffer_id: id::BufferId,
        count_buffer_offset: BufferAddress,
        max_count: u32,
    ) -> Result<(), PassStateError> {
        let scope = PassErrorScope::Draw {
            kind: DrawKind::MultiDrawIndirectCount,
            family: DrawCommandFamily::DrawIndexed,
        };
        let base = pass_base!(pass, scope);

        base.commands
            .push(ArcRenderCommand::MultiDrawIndirectCount {
                buffer: pass_try!(base, scope, self.resolve_buffer_id(buffer_id)),
                offset,
                count_buffer: pass_try!(base, scope, self.resolve_buffer_id(count_buffer_id)),
                count_buffer_offset,
                max_count,
                family: DrawCommandFamily::DrawIndexed,
            });

        Ok(())
    }

    pub fn render_pass_multi_draw_mesh_tasks_indirect_count(
        &self,
        pass: &mut RenderPass,
        buffer_id: id::BufferId,
        offset: BufferAddress,
        count_buffer_id: id::BufferId,
        count_buffer_offset: BufferAddress,
        max_count: u32,
    ) -> Result<(), RenderPassError> {
        let scope = PassErrorScope::Draw {
            kind: DrawKind::MultiDrawIndirectCount,
            family: DrawCommandFamily::DrawMeshTasks,
        };
        let base = pass_base!(pass, scope);

        base.commands
            .push(ArcRenderCommand::MultiDrawIndirectCount {
                buffer: pass_try!(base, scope, self.resolve_buffer_id(buffer_id)),
                offset,
                count_buffer: pass_try!(base, scope, self.resolve_buffer_id(count_buffer_id)),
                count_buffer_offset,
                max_count,
                family: DrawCommandFamily::DrawMeshTasks,
            });

        Ok(())
    }

    pub fn render_pass_push_debug_group(
        &self,
        pass: &mut RenderPass,
        label: &str,
        color: u32,
    ) -> Result<(), PassStateError> {
        let base = pass_base!(pass, PassErrorScope::PushDebugGroup);

        let bytes = label.as_bytes();
        base.string_data.extend_from_slice(bytes);

        base.commands.push(ArcRenderCommand::PushDebugGroup {
            color,
            len: bytes.len(),
        });

        Ok(())
    }

    pub fn render_pass_pop_debug_group(&self, pass: &mut RenderPass) -> Result<(), PassStateError> {
        let base = pass_base!(pass, PassErrorScope::PopDebugGroup);

        base.commands.push(ArcRenderCommand::PopDebugGroup);

        Ok(())
    }

    pub fn render_pass_insert_debug_marker(
        &self,
        pass: &mut RenderPass,
        label: &str,
        color: u32,
    ) -> Result<(), PassStateError> {
        let base = pass_base!(pass, PassErrorScope::InsertDebugMarker);

        let bytes = label.as_bytes();
        base.string_data.extend_from_slice(bytes);

        base.commands.push(ArcRenderCommand::InsertDebugMarker {
            color,
            len: bytes.len(),
        });

        Ok(())
    }

    pub fn render_pass_write_timestamp(
        &self,
        pass: &mut RenderPass,
        query_set_id: id::QuerySetId,
        query_index: u32,
    ) -> Result<(), PassStateError> {
        let scope = PassErrorScope::WriteTimestamp;
        let base = pass_base!(pass, scope);

        base.commands.push(ArcRenderCommand::WriteTimestamp {
            query_set: pass_try!(base, scope, self.resolve_query_set(query_set_id)),
            query_index,
        });

        Ok(())
    }

    pub fn render_pass_begin_occlusion_query(
        &self,
        pass: &mut RenderPass,
        query_index: u32,
    ) -> Result<(), PassStateError> {
        let scope = PassErrorScope::BeginOcclusionQuery;
        let base = pass_base!(pass, scope);

        base.commands
            .push(ArcRenderCommand::BeginOcclusionQuery { query_index });

        Ok(())
    }

    pub fn render_pass_end_occlusion_query(
        &self,
        pass: &mut RenderPass,
    ) -> Result<(), PassStateError> {
        let scope = PassErrorScope::EndOcclusionQuery;
        let base = pass_base!(pass, scope);

        base.commands.push(ArcRenderCommand::EndOcclusionQuery);

        Ok(())
    }

    pub fn render_pass_begin_pipeline_statistics_query(
        &self,
        pass: &mut RenderPass,
        query_set_id: id::QuerySetId,
        query_index: u32,
    ) -> Result<(), PassStateError> {
        let scope = PassErrorScope::BeginPipelineStatisticsQuery;
        let base = pass_base!(pass, scope);

        base.commands
            .push(ArcRenderCommand::BeginPipelineStatisticsQuery {
                query_set: pass_try!(base, scope, self.resolve_query_set(query_set_id)),
                query_index,
            });

        Ok(())
    }

    pub fn render_pass_end_pipeline_statistics_query(
        &self,
        pass: &mut RenderPass,
    ) -> Result<(), PassStateError> {
        let scope = PassErrorScope::EndPipelineStatisticsQuery;
        let base = pass_base!(pass, scope);

        base.commands
            .push(ArcRenderCommand::EndPipelineStatisticsQuery);

        Ok(())
    }

    pub fn render_pass_execute_bundles(
        &self,
        pass: &mut RenderPass,
        render_bundle_ids: &[id::RenderBundleId],
    ) -> Result<(), PassStateError> {
        let scope = PassErrorScope::ExecuteBundle;
        let base = pass_base!(pass, scope);

        let hub = &self.hub;
        let bundles = hub.render_bundles.read();

        for &bundle_id in render_bundle_ids {
            let bundle = pass_try!(base, scope, bundles.get(bundle_id).get());

            base.commands.push(ArcRenderCommand::ExecuteBundle(bundle));
        }
        pass.current_pipeline.reset();
        pass.current_bind_groups.reset();

        Ok(())
    }
}

pub(crate) const fn get_src_stride_of_indirect_args(family: DrawCommandFamily) -> u64 {
    match family {
        DrawCommandFamily::Draw => size_of::<wgt::DrawIndirectArgs>() as u64,
        DrawCommandFamily::DrawIndexed => size_of::<wgt::DrawIndexedIndirectArgs>() as u64,
        DrawCommandFamily::DrawMeshTasks => size_of::<wgt::DispatchIndirectArgs>() as u64,
    }
}

pub(crate) const fn get_dst_stride_of_indirect_args(
    backend: wgt::Backend,
    family: DrawCommandFamily,
) -> u64 {
    // space for D3D12 special constants
    let extra = if matches!(backend, wgt::Backend::Dx12) {
        3 * size_of::<u32>() as u64
    } else {
        0
    };
    extra + get_src_stride_of_indirect_args(family)
}
