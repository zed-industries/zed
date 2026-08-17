use core::{convert::Infallible, num::NonZero};

use alloc::{string::String, sync::Arc, vec::Vec};
#[cfg(feature = "serde")]
use macro_rules_attribute::{apply, attribute_alias};

use crate::{
    command::ColorAttachments,
    id,
    instance::Surface,
    resource::{Buffer, QuerySet, Texture},
};

pub trait ReferenceType {
    type Buffer: Clone + core::fmt::Debug;
    type Surface: Clone; // Surface does not implement Debug, although it probably could.
    type Texture: Clone + core::fmt::Debug;
    type TextureView: Clone + core::fmt::Debug;
    type ExternalTexture: Clone + core::fmt::Debug;
    type QuerySet: Clone + core::fmt::Debug;
    type BindGroup: Clone + core::fmt::Debug;
    type RenderPipeline: Clone + core::fmt::Debug;
    type RenderBundle: Clone + core::fmt::Debug;
    type ComputePipeline: Clone + core::fmt::Debug;
    type Blas: Clone + core::fmt::Debug;
    type Tlas: Clone + core::fmt::Debug;
}

/// Reference wgpu objects via numeric IDs assigned by [`crate::identity::IdentityManager`].
#[derive(Clone, Debug)]
pub struct IdReferences;

/// Reference wgpu objects via the integer value of pointers.
///
/// This is used for trace recording and playback. Recording stores the pointer
/// value of `Arc` references in the trace. Playback uses the integer values
/// as keys to a `HashMap`.
#[cfg(any(feature = "trace", feature = "replay"))]
#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct PointerReferences;

/// Reference wgpu objects via `Arc`s.
#[derive(Clone, Debug)]
pub struct ArcReferences;

impl ReferenceType for IdReferences {
    type Buffer = id::BufferId;
    type Surface = id::SurfaceId;
    type Texture = id::TextureId;
    type TextureView = id::TextureViewId;
    type ExternalTexture = id::ExternalTextureId;
    type QuerySet = id::QuerySetId;
    type BindGroup = id::BindGroupId;
    type RenderPipeline = id::RenderPipelineId;
    type RenderBundle = id::RenderBundleId;
    type ComputePipeline = id::ComputePipelineId;
    type Blas = id::BlasId;
    type Tlas = id::TlasId;
}

#[cfg(any(feature = "trace", feature = "replay"))]
impl ReferenceType for PointerReferences {
    type Buffer = id::PointerId<id::markers::Buffer>;
    type Surface = id::PointerId<id::markers::Surface>;
    type Texture = id::PointerId<id::markers::Texture>;
    type TextureView = id::PointerId<id::markers::TextureView>;
    type ExternalTexture = id::PointerId<id::markers::ExternalTexture>;
    type QuerySet = id::PointerId<id::markers::QuerySet>;
    type BindGroup = id::PointerId<id::markers::BindGroup>;
    type RenderPipeline = id::PointerId<id::markers::RenderPipeline>;
    type RenderBundle = id::PointerId<id::markers::RenderBundle>;
    type ComputePipeline = id::PointerId<id::markers::ComputePipeline>;
    type Blas = id::PointerId<id::markers::Blas>;
    type Tlas = id::PointerId<id::markers::Tlas>;
}

impl ReferenceType for ArcReferences {
    type Buffer = Arc<Buffer>;
    type Surface = Arc<Surface>;
    type Texture = Arc<Texture>;
    type TextureView = Arc<crate::resource::TextureView>;
    type ExternalTexture = Arc<crate::resource::ExternalTexture>;
    type QuerySet = Arc<QuerySet>;
    type BindGroup = Arc<crate::binding_model::BindGroup>;
    type RenderPipeline = Arc<crate::pipeline::RenderPipeline>;
    type RenderBundle = Arc<crate::command::RenderBundle>;
    type ComputePipeline = Arc<crate::pipeline::ComputePipeline>;
    type Blas = Arc<crate::resource::Blas>;
    type Tlas = Arc<crate::resource::Tlas>;
}

#[cfg(feature = "serde")]
attribute_alias! {
    #[apply(serde_object_reference_struct)] =
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(bound =
         "R::Buffer: serde::Serialize + for<'d> serde::Deserialize<'d>,\
          R::Surface: serde::Serialize + for<'d> serde::Deserialize<'d>,\
          R::Texture: serde::Serialize + for<'d> serde::Deserialize<'d>,\
          R::TextureView: serde::Serialize + for<'d> serde::Deserialize<'d>,\
          R::ExternalTexture: serde::Serialize + for<'d> serde::Deserialize<'d>,\
          R::QuerySet: serde::Serialize + for<'d> serde::Deserialize<'d>,\
          R::BindGroup: serde::Serialize + for<'d> serde::Deserialize<'d>,\
          R::RenderPipeline: serde::Serialize + for<'d> serde::Deserialize<'d>,\
          R::RenderBundle: serde::Serialize + for<'d> serde::Deserialize<'d>,\
          R::ComputePipeline: serde::Serialize + for<'d> serde::Deserialize<'d>,\
          R::Blas: serde::Serialize + for<'d> serde::Deserialize<'d>,\
          R::Tlas: serde::Serialize + for<'d> serde::Deserialize<'d>,\
          wgt::BufferTransition<R::Buffer>: serde::Serialize + for<'d> serde::Deserialize<'d>,\
          wgt::TextureTransition<R::Texture>: serde::Serialize + for<'d> serde::Deserialize<'d>"
    )];
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", apply(serde_object_reference_struct))]
pub enum Command<R: ReferenceType> {
    CopyBufferToBuffer {
        src: R::Buffer,
        src_offset: wgt::BufferAddress,
        dst: R::Buffer,
        dst_offset: wgt::BufferAddress,
        size: Option<wgt::BufferAddress>,
    },
    CopyBufferToTexture {
        src: wgt::TexelCopyBufferInfo<R::Buffer>,
        dst: wgt::TexelCopyTextureInfo<R::Texture>,
        size: wgt::Extent3d,
    },
    CopyTextureToBuffer {
        src: wgt::TexelCopyTextureInfo<R::Texture>,
        dst: wgt::TexelCopyBufferInfo<R::Buffer>,
        size: wgt::Extent3d,
    },
    CopyTextureToTexture {
        src: wgt::TexelCopyTextureInfo<R::Texture>,
        dst: wgt::TexelCopyTextureInfo<R::Texture>,
        size: wgt::Extent3d,
    },
    ClearBuffer {
        dst: R::Buffer,
        offset: wgt::BufferAddress,
        size: Option<wgt::BufferAddress>,
    },
    ClearTexture {
        dst: R::Texture,
        subresource_range: wgt::ImageSubresourceRange,
    },
    WriteTimestamp {
        query_set: R::QuerySet,
        query_index: u32,
    },
    ResolveQuerySet {
        query_set: R::QuerySet,
        start_query: u32,
        query_count: u32,
        destination: R::Buffer,
        destination_offset: wgt::BufferAddress,
    },
    PushDebugGroup(String),
    PopDebugGroup,
    InsertDebugMarker(String),
    RunComputePass {
        pass: crate::command::BasePass<crate::command::ComputeCommand<R>, Infallible>,
        timestamp_writes: Option<crate::command::PassTimestampWrites<R::QuerySet>>,
    },
    RunRenderPass {
        pass: crate::command::BasePass<crate::command::RenderCommand<R>, Infallible>,
        color_attachments: ColorAttachments<R::TextureView>,
        depth_stencil_attachment:
            Option<crate::command::ResolvedRenderPassDepthStencilAttachment<R::TextureView>>,
        timestamp_writes: Option<crate::command::PassTimestampWrites<R::QuerySet>>,
        occlusion_query_set: Option<R::QuerySet>,
        multiview_mask: Option<NonZero<u32>>,
    },
    BuildAccelerationStructures {
        blas: Vec<crate::ray_tracing::OwnedBlasBuildEntry<R>>,
        tlas: Vec<crate::ray_tracing::OwnedTlasPackage<R>>,
    },
    TransitionResources {
        buffer_transitions: Vec<wgt::BufferTransition<R::Buffer>>,
        texture_transitions: Vec<wgt::TextureTransition<R::Texture>>,
    },
}

pub type ArcCommand = Command<ArcReferences>;
