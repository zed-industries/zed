use crate::*;

/// Handle to an external texture on the GPU.
///
/// It can be created with [`Device::create_external_texture`].
///
/// Corresponds to [WebGPU `GPUExternalTexture`](https://gpuweb.github.io/gpuweb/#gpuexternaltexture).
#[derive(Debug, Clone)]
pub struct ExternalTexture {
    pub(crate) inner: dispatch::DispatchExternalTexture,
}
#[cfg(send_sync)]
static_assertions::assert_impl_all!(ExternalTexture: Send, Sync);

crate::cmp::impl_eq_ord_hash_proxy!(ExternalTexture => .inner);

impl ExternalTexture {
    /// Destroy the associated native resources as soon as possible.
    pub fn destroy(&self) {
        self.inner.destroy();
    }
}

/// Describes an [`ExternalTexture`].
///
/// For use with [`Device::create_external_texture`].
///
/// Corresponds to [WebGPU `GPUExternalTextureDescriptor`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpuexternaltexturedescriptor).
pub type ExternalTextureDescriptor<'a> = wgt::ExternalTextureDescriptor<Label<'a>>;
static_assertions::assert_impl_all!(ExternalTextureDescriptor<'_>: Send, Sync);
