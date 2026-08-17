use crate::*;

/// Pre-prepared reusable bundle of GPU operations.
///
/// It only supports a handful of render commands, but it makes them reusable. Executing a
/// [`RenderBundle`] is often more efficient than issuing the underlying commands manually.
///
/// It can be created by use of a [`RenderBundleEncoder`], and executed onto a [`CommandEncoder`]
/// using [`RenderPass::execute_bundles`].
///
/// Corresponds to [WebGPU `GPURenderBundle`](https://gpuweb.github.io/gpuweb/#render-bundle).
#[derive(Debug, Clone)]
pub struct RenderBundle {
    pub(crate) inner: dispatch::DispatchRenderBundle,
}
#[cfg(send_sync)]
static_assertions::assert_impl_all!(RenderBundle: Send, Sync);

crate::cmp::impl_eq_ord_hash_proxy!(RenderBundle => .inner);

impl RenderBundle {
    #[cfg(custom)]
    /// Returns custom implementation of RenderBundle (if custom backend and is internally T)
    pub fn as_custom<T: custom::RenderBundleInterface>(&self) -> Option<&T> {
        self.inner.as_custom()
    }
}

/// Describes a [`RenderBundle`].
///
/// For use with [`RenderBundleEncoder::finish`].
///
/// Corresponds to [WebGPU `GPURenderBundleDescriptor`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpurenderbundledescriptor).
pub type RenderBundleDescriptor<'a> = wgt::RenderBundleDescriptor<Label<'a>>;
static_assertions::assert_impl_all!(RenderBundleDescriptor<'_>: Send, Sync);
