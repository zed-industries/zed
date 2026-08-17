use wgt::{BufferAddress, BufferSize, Color};

use super::{DrawCommandFamily, Rect};
#[cfg(feature = "serde")]
use crate::command::serde_object_reference_struct;
use crate::command::{ArcReferences, ReferenceType};

#[cfg(feature = "serde")]
use macro_rules_attribute::apply;

/// cbindgen:ignore
#[doc(hidden)]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", apply(serde_object_reference_struct))]
pub enum RenderCommand<R: ReferenceType> {
    SetBindGroup {
        index: u32,
        num_dynamic_offsets: usize,
        bind_group: Option<R::BindGroup>,
    },
    SetPipeline(R::RenderPipeline),
    SetIndexBuffer {
        buffer: R::Buffer,
        index_format: wgt::IndexFormat,
        offset: BufferAddress,
        size: Option<BufferSize>,
    },
    SetVertexBuffer {
        slot: u32,
        buffer: R::Buffer,
        offset: BufferAddress,
        size: Option<BufferSize>,
    },
    SetBlendConstant(Color),
    SetStencilReference(u32),
    SetViewport {
        rect: Rect<f32>,
        //TODO: use half-float to reduce the size?
        depth_min: f32,
        depth_max: f32,
    },
    SetScissor(Rect<u32>),

    /// Set a range of immediates to values stored in [`BasePass::immediates_data`].
    ///
    /// See [`wgpu::RenderPass::set_immediates`] for a detailed explanation
    /// of the restrictions these commands must satisfy.
    SetImmediate {
        /// The byte offset within the immediate data storage to write to.  This
        /// must be a multiple of four.
        offset: u32,

        /// The number of bytes to write. This must be a multiple of four.
        size_bytes: u32,

        /// Index in [`BasePass::immediates_data`] of the start of the data
        /// to be written.
        ///
        /// Note: this is not a byte offset like `offset`. Rather, it is the
        /// index of the first `u32` element in `immediates_data` to read.
        ///
        /// `None` means zeros should be written to the destination range, and
        /// there is no corresponding data in `immediates_data`. This is used
        /// by render bundles, which explicitly clear out any state that
        /// post-bundle code might see.
        values_offset: Option<u32>,
    },
    Draw {
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    },
    DrawIndexed {
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        base_vertex: i32,
        first_instance: u32,
    },
    DrawMeshTasks {
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    },
    DrawIndirect {
        buffer: R::Buffer,
        offset: BufferAddress,
        count: u32,
        family: DrawCommandFamily,
        /// This limit is only populated for commands in a finished [`RenderBundle`].
        vertex_or_index_limit: Option<u64>,
        /// This limit is only populated for commands in a finished [`RenderBundle`].
        instance_limit: Option<u64>,
    },
    MultiDrawIndirectCount {
        buffer: R::Buffer,
        offset: BufferAddress,
        count_buffer: R::Buffer,
        count_buffer_offset: BufferAddress,
        max_count: u32,
        family: DrawCommandFamily,
    },
    PushDebugGroup {
        color: u32,
        len: usize,
    },
    PopDebugGroup,
    InsertDebugMarker {
        color: u32,
        len: usize,
    },
    WriteTimestamp {
        query_set: R::QuerySet,
        query_index: u32,
    },
    BeginOcclusionQuery {
        query_index: u32,
    },
    EndOcclusionQuery,
    BeginPipelineStatisticsQuery {
        query_set: R::QuerySet,
        query_index: u32,
    },
    EndPipelineStatisticsQuery,
    ExecuteBundle(R::RenderBundle),
}

/// Equivalent to `RenderCommand` with the Ids resolved into resource Arcs.
///
/// In a render pass, commands are stored in this format between when they are
/// added to the pass, and when the pass is `end()`ed and the commands are
/// replayed to the HAL encoder. Validation occurs when the pass is ended, which
/// means that parameters stored in an `ArcRenderCommand` for a pass operation
/// have generally not been validated.
///
/// In a render bundle, commands are stored in this format between when the bundle
/// is `finish()`ed and when the bundle is executed. Validation occurs when the
/// bundle is finished, which means that parameters stored in an `ArcRenderCommand`
/// for a render bundle operation must have been validated.
///
/// cbindgen:ignore
pub type ArcRenderCommand = RenderCommand<ArcReferences>;
