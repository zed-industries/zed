use crate::*;

/// Handle to a compute pipeline.
///
/// A `ComputePipeline` object represents a compute pipeline and its single shader stage.
/// It can be created with [`Device::create_compute_pipeline`].
///
/// Corresponds to [WebGPU `GPUComputePipeline`](https://gpuweb.github.io/gpuweb/#compute-pipeline).
#[derive(Debug, Clone)]
pub struct ComputePipeline {
    pub(crate) inner: dispatch::DispatchComputePipeline,
}
#[cfg(send_sync)]
static_assertions::assert_impl_all!(ComputePipeline: Send, Sync);

crate::cmp::impl_eq_ord_hash_proxy!(ComputePipeline => .inner);

impl ComputePipeline {
    /// Get an object representing the bind group layout at a given index.
    ///
    /// If this pipeline was created with a [default layout][ComputePipelineDescriptor::layout],
    /// then bind groups created with the returned `BindGroupLayout` can only be used with this
    /// pipeline.
    ///
    /// This method will raise a validation error if there is no bind group layout at `index`.
    pub fn get_bind_group_layout(&self, index: u32) -> BindGroupLayout {
        let bind_group = self.inner.get_bind_group_layout(index);
        BindGroupLayout { inner: bind_group }
    }

    #[cfg(custom)]
    /// Returns custom implementation of ComputePipeline (if custom backend and is internally T)
    pub fn as_custom<T: custom::ComputePipelineInterface>(&self) -> Option<&T> {
        self.inner.as_custom()
    }
}

/// Describes a compute pipeline.
///
/// For use with [`Device::create_compute_pipeline`].
///
/// Corresponds to [WebGPU `GPUComputePipelineDescriptor`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpucomputepipelinedescriptor).
#[derive(Clone, Debug)]
pub struct ComputePipelineDescriptor<'a> {
    /// Debug label of the pipeline. This will show up in graphics debuggers for easy identification.
    pub label: Label<'a>,
    /// The layout of bind groups for this pipeline.
    ///
    /// If this is set, then [`Device::create_compute_pipeline`] will raise a validation error if
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
    /// You can use [`ComputePipeline::get_bind_group_layout`] to create bind groups for use with
    /// the default layout. However, these bind groups cannot be used with any other pipelines. This
    /// is convenient for simple pipelines, but using an explicit layout is recommended in most
    /// cases.
    ///
    /// [default layout]: https://www.w3.org/TR/webgpu/#default-pipeline-layout
    pub layout: Option<&'a PipelineLayout>,
    /// The compiled shader module for this stage.
    pub module: &'a ShaderModule,
    /// The name of the entry point in the compiled shader to use.
    ///
    /// If [`Some`], there must be a compute shader entry point with this name in `module`.
    /// Otherwise, expect exactly one compute shader entry point in `module`, which will be
    /// selected.
    // NOTE: keep phrasing in sync. with `FragmentState::entry_point`
    // NOTE: keep phrasing in sync. with `VertexState::entry_point`
    pub entry_point: Option<&'a str>,
    /// Advanced options for when this pipeline is compiled
    ///
    /// This implements `Default`, and for most users can be set to `Default::default()`
    pub compilation_options: PipelineCompilationOptions<'a>,
    /// The pipeline cache to use when creating this pipeline.
    pub cache: Option<&'a PipelineCache>,
}
#[cfg(send_sync)]
static_assertions::assert_impl_all!(ComputePipelineDescriptor<'_>: Send, Sync);
