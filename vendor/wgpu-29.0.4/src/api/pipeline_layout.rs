use crate::*;

/// Handle to a pipeline layout.
///
/// A `PipelineLayout` object describes the available binding groups of a pipeline.
/// It can be created with [`Device::create_pipeline_layout`].
///
/// Corresponds to [WebGPU `GPUPipelineLayout`](https://gpuweb.github.io/gpuweb/#gpupipelinelayout).
#[derive(Debug, Clone)]
pub struct PipelineLayout {
    pub(crate) inner: dispatch::DispatchPipelineLayout,
}
#[cfg(send_sync)]
static_assertions::assert_impl_all!(PipelineLayout: Send, Sync);

crate::cmp::impl_eq_ord_hash_proxy!(PipelineLayout => .inner);

impl PipelineLayout {
    #[cfg(custom)]
    /// Returns custom implementation of PipelineLayout (if custom backend and is internally T)
    pub fn as_custom<T: custom::PipelineLayoutInterface>(&self) -> Option<&T> {
        self.inner.as_custom()
    }
}

/// Describes a [`PipelineLayout`].
///
/// For use with [`Device::create_pipeline_layout`].
///
/// Corresponds to [WebGPU `GPUPipelineLayoutDescriptor`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpupipelinelayoutdescriptor).
#[derive(Clone, Debug, Default)]
pub struct PipelineLayoutDescriptor<'a> {
    /// Debug label of the pipeline layout. This will show up in graphics debuggers for easy identification.
    pub label: Label<'a>,
    /// Bind groups that this pipeline uses. The first entry will provide all the bindings for
    /// "set = 0", second entry will provide all the bindings for "set = 1" etc.
    pub bind_group_layouts: &'a [Option<&'a BindGroupLayout>],
    /// The number of bytes of immediate data that are allocated for use
    /// in the shader. The `var<immediate>`s in the shader attached to
    /// this pipeline must be equal or smaller than this size.
    ///
    /// If this value is non-zero, [`Features::IMMEDIATES`] must be enabled.
    pub immediate_size: u32,
}
#[cfg(send_sync)]
static_assertions::assert_impl_all!(PipelineLayoutDescriptor<'_>: Send, Sync);
