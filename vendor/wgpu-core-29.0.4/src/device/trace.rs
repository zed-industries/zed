#[cfg(feature = "trace")]
mod record;
#[cfg(feature = "replay")]
mod replay;

use core::convert::Infallible;

use alloc::{string::String, vec::Vec};
use macro_rules_attribute::apply;

use crate::{
    command::{serde_object_reference_struct, BasePass, Command, ReferenceType, RenderCommand},
    id::{markers, PointerId},
    pipeline::GeneralRenderPipelineDescriptor,
};

#[cfg(feature = "trace")]
pub use record::*;
#[cfg(feature = "replay")]
pub use replay::*;

type FileName = String;

pub const FILE_NAME: &str = "trace.ron";

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Data {
    File(FileName),
    String(DataKind, String),
    Binary(DataKind, Vec<u8>),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "lowercase")
)]
pub enum DataKind {
    Bin,
    Wgsl,

    /// IR of Naga module, serialized in RON format
    Ron,
    Spv,
    Dxil,
    Hlsl,
    MetalLib,
    Msl,
    Glsl,
}

impl core::fmt::Display for DataKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let s = match self {
            DataKind::Bin => "bin",
            DataKind::Wgsl => "wgsl",
            DataKind::Ron => "ron",
            DataKind::Spv => "spv",
            DataKind::Dxil => "dxil",
            DataKind::Hlsl => "hlsl",
            DataKind::MetalLib => "metallib",
            DataKind::Msl => "metal",
            DataKind::Glsl => "glsl",
        };
        write!(f, "{s}")
    }
}

impl DataKind {
    #[cfg(feature = "replay")]
    fn is_string(&self) -> bool {
        match *self {
            DataKind::Wgsl | DataKind::Ron | DataKind::Hlsl | DataKind::Msl | DataKind::Glsl => {
                true
            }
            DataKind::Bin | DataKind::Spv | DataKind::Dxil | DataKind::MetalLib => false,
        }
    }
}

impl Data {
    pub fn kind(&self) -> DataKind {
        match self {
            Data::File(file) => {
                if file.ends_with(".bin") {
                    DataKind::Bin
                } else if file.ends_with(".wgsl") {
                    DataKind::Wgsl
                } else if file.ends_with(".ron") {
                    DataKind::Ron
                } else if file.ends_with(".spv") {
                    DataKind::Spv
                } else if file.ends_with(".dxil") {
                    DataKind::Dxil
                } else if file.ends_with(".hlsl") {
                    DataKind::Hlsl
                } else if file.ends_with(".metallib") {
                    DataKind::MetalLib
                } else if file.ends_with(".metal") {
                    DataKind::Msl
                } else if file.ends_with(".glsl") {
                    DataKind::Glsl
                } else {
                    panic!("unknown data file extension: {file}");
                }
            }
            Data::String(kind, _) => *kind,
            Data::Binary(kind, _) => *kind,
        }
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
#[apply(serde_object_reference_struct)]
pub enum Action<'a, R: ReferenceType> {
    Init {
        desc: crate::device::DeviceDescriptor<'a>,
        backend: wgt::Backend,
    },
    ConfigureSurface(
        R::Surface,
        wgt::SurfaceConfiguration<Vec<wgt::TextureFormat>>,
    ),
    CreateBuffer(R::Buffer, crate::resource::BufferDescriptor<'a>),
    FreeBuffer(R::Buffer),
    DestroyBuffer(R::Buffer),
    CreateTexture(R::Texture, crate::resource::TextureDescriptor<'a>),
    FreeTexture(R::Texture),
    DestroyTexture(R::Texture),
    CreateTextureView {
        id: R::TextureView,
        parent: R::Texture,
        desc: crate::resource::TextureViewDescriptor<'a>,
    },
    DestroyTextureView(R::TextureView),
    CreateExternalTexture {
        id: R::ExternalTexture,
        desc: crate::resource::ExternalTextureDescriptor<'a>,
        planes: alloc::boxed::Box<[R::TextureView]>,
    },
    FreeExternalTexture(R::ExternalTexture),
    DestroyExternalTexture(R::ExternalTexture),
    CreateSampler(
        PointerId<markers::Sampler>,
        crate::resource::SamplerDescriptor<'a>,
    ),
    DestroySampler(PointerId<markers::Sampler>),
    GetSurfaceTexture {
        id: R::Texture,
        parent: R::Surface,
    },
    Present(R::Surface),
    DiscardSurfaceTexture(R::Surface),
    CreateBindGroupLayout(
        PointerId<markers::BindGroupLayout>,
        crate::binding_model::BindGroupLayoutDescriptor<'a>,
    ),
    GetRenderPipelineBindGroupLayout {
        id: PointerId<markers::BindGroupLayout>,
        pipeline: PointerId<markers::RenderPipeline>,
        index: u32,
    },
    GetComputePipelineBindGroupLayout {
        id: PointerId<markers::BindGroupLayout>,
        pipeline: PointerId<markers::ComputePipeline>,
        index: u32,
    },
    DestroyBindGroupLayout(PointerId<markers::BindGroupLayout>),
    CreatePipelineLayout(
        PointerId<markers::PipelineLayout>,
        crate::binding_model::ResolvedPipelineLayoutDescriptor<
            'a,
            PointerId<markers::BindGroupLayout>,
        >,
    ),
    DestroyPipelineLayout(PointerId<markers::PipelineLayout>),
    CreateBindGroup(PointerId<markers::BindGroup>, TraceBindGroupDescriptor<'a>),
    DestroyBindGroup(PointerId<markers::BindGroup>),
    CreateShaderModule {
        id: PointerId<markers::ShaderModule>,
        desc: crate::pipeline::ShaderModuleDescriptor<'a>,
        data: Data,
    },
    CreateShaderModulePassthrough {
        id: PointerId<markers::ShaderModule>,
        data: Vec<Data>,

        label: crate::Label<'a>,
        num_workgroups: (u32, u32, u32),
    },
    DestroyShaderModule(PointerId<markers::ShaderModule>),
    CreateComputePipeline {
        id: Option<PointerId<markers::ComputePipeline>>,
        desc: TraceComputePipelineDescriptor<'a>,
    },
    DestroyComputePipeline(PointerId<markers::ComputePipeline>),
    CreateGeneralRenderPipeline {
        id: Option<PointerId<markers::RenderPipeline>>,
        desc: TraceGeneralRenderPipelineDescriptor<'a>,
    },
    DestroyRenderPipeline(PointerId<markers::RenderPipeline>),
    CreatePipelineCache {
        id: PointerId<markers::PipelineCache>,
        desc: crate::pipeline::PipelineCacheDescriptor<'a>,
    },
    DestroyPipelineCache(PointerId<markers::PipelineCache>),
    CreateRenderBundle {
        id: R::RenderBundle,
        desc: crate::command::RenderBundleEncoderDescriptor<'a>,
        base: BasePass<RenderCommand<R>, Infallible>,
    },
    DestroyRenderBundle(PointerId<markers::RenderBundle>),
    CreateQuerySet {
        id: PointerId<markers::QuerySet>,
        desc: crate::resource::QuerySetDescriptor<'a>,
    },
    DestroyQuerySet(PointerId<markers::QuerySet>),
    WriteBuffer {
        id: R::Buffer,
        data: Data,
        offset: wgt::BufferAddress,
        size: wgt::BufferAddress,
        queued: bool,
    },
    WriteTexture {
        to: wgt::TexelCopyTextureInfo<R::Texture>,
        data: Data,
        layout: wgt::TexelCopyBufferLayout,
        size: wgt::Extent3d,
    },
    Submit(crate::SubmissionIndex, Vec<Command<R>>),
    FailedCommands {
        commands: Option<Vec<Command<R>>>,
        /// If `None`, then encoding failed due to a validation error (returned
        /// from `CommandEncoder::finish`). If `Some`, submission failed due to
        /// a resource having been destroyed.
        failed_at_submit: Option<crate::SubmissionIndex>,
        error: String,
    },
    CreateBlas {
        id: R::Blas,
        desc: crate::resource::BlasDescriptor<'a>,
        sizes: wgt::BlasGeometrySizeDescriptors,
    },
    DestroyBlas(R::Blas),
    CreateTlas {
        id: R::Tlas,
        desc: crate::resource::TlasDescriptor<'a>,
    },
    DestroyTlas(R::Tlas),
}

/// cbindgen:ignore
pub type TraceBindGroupDescriptor<'a> = crate::binding_model::BindGroupDescriptor<
    'a,
    PointerId<markers::BindGroupLayout>,
    PointerId<markers::Buffer>,
    PointerId<markers::Sampler>,
    PointerId<markers::TextureView>,
    PointerId<markers::Tlas>,
    PointerId<markers::ExternalTexture>,
>;

/// Not a public API. For use by `player` only.
///
/// cbindgen:ignore
#[doc(hidden)]
pub type TraceGeneralRenderPipelineDescriptor<'a> = GeneralRenderPipelineDescriptor<
    'a,
    PointerId<markers::PipelineLayout>,
    PointerId<markers::ShaderModule>,
    PointerId<markers::PipelineCache>,
>;

/// Not a public API. For use by `player` only.
///
/// cbindgen:ignore
#[doc(hidden)]
pub type TraceComputePipelineDescriptor<'a> = crate::pipeline::ComputePipelineDescriptor<
    'a,
    PointerId<markers::PipelineLayout>,
    PointerId<markers::ShaderModule>,
    PointerId<markers::PipelineCache>,
>;
