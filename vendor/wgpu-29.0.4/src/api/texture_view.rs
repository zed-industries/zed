#[cfg(wgpu_core)]
use core::ops::Deref;

use crate::*;

/// Handle to a texture view.
///
/// A `TextureView` object refers to a [`Texture`], or a subset of its layers and mip levels, and
/// specifies an interpretation of the texture’s texels, which is needed to use a texture as a
/// binding in a [`BindGroup`] or as an attachment in a [`RenderPass`].
/// It can be created using [`Texture::create_view()`], which accepts a [`TextureViewDescriptor`]
/// specifying the properties of the view.
///
/// Corresponds to [WebGPU `GPUTextureView`](https://gpuweb.github.io/gpuweb/#gputextureview).
#[derive(Debug, Clone)]
pub struct TextureView {
    pub(crate) inner: dispatch::DispatchTextureView,
    pub(crate) texture: Texture,
}
#[cfg(send_sync)]
static_assertions::assert_impl_all!(TextureView: Send, Sync);

crate::cmp::impl_eq_ord_hash_proxy!(TextureView => .inner);

impl TextureView {
    /// Returns the [`Texture`] that this `TextureView` refers to.
    ///
    /// All wgpu resources are refcounted, so you can own the returned [`Texture`]
    /// by cloning it.
    pub fn texture(&self) -> &Texture {
        &self.texture
    }

    /// Get the [`wgpu_hal`] texture view from this `TextureView`.
    ///
    /// Find the Api struct corresponding to the active backend in [`wgpu_hal::api`],
    /// and pass that struct to the to the `A` type parameter.
    ///
    /// Returns a guard that dereferences to the type of the hal backend
    /// which implements [`A::TextureView`].
    ///
    /// # Types
    ///
    /// The returned type depends on the backend:
    ///
    #[doc = crate::macros::hal_type_vulkan!("TextureView")]
    #[doc = crate::macros::hal_type_metal!("TextureView")]
    #[doc = crate::macros::hal_type_dx12!("TextureView")]
    #[doc = crate::macros::hal_type_gles!("TextureView")]
    ///
    /// # Deadlocks
    ///
    /// - The returned guard holds a read-lock on a device-local "destruction"
    ///   lock, which will cause all calls to `destroy` to block until the
    ///   guard is released.
    ///
    /// # Errors
    ///
    /// This method will return None if:
    /// - The texture view is not from the backend specified by `A`.
    /// - The texture view is from the `webgpu` or `custom` backend.
    /// - The texture this view points to has had [`Texture::destroy()`] called on it.
    ///
    /// # Safety
    ///
    /// - The returned resource must not be destroyed unless the guard
    ///   is the last reference to it and it is not in use by the GPU.
    ///   The guard and handle may be dropped at any time however.
    /// - All the safety requirements of wgpu-hal must be upheld.
    ///
    /// [`A::TextureView`]: hal::Api::TextureView
    #[cfg(wgpu_core)]
    pub unsafe fn as_hal<A: hal::Api>(&self) -> Option<impl Deref<Target = A::TextureView>> {
        let view = self.inner.as_core_opt()?;
        unsafe { view.context.texture_view_as_hal::<A>(view) }
    }

    #[cfg(custom)]
    /// Returns custom implementation of TextureView (if custom backend and is internally T)
    pub fn as_custom<T: custom::TextureViewInterface>(&self) -> Option<&T> {
        self.inner.as_custom()
    }
}

/// Describes a [`TextureView`].
///
/// For use with [`Texture::create_view`].
///
/// Corresponds to [WebGPU `GPUTextureViewDescriptor`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gputextureviewdescriptor).
pub type TextureViewDescriptor<'a> = wgt::TextureViewDescriptor<Label<'a>>;
static_assertions::assert_impl_all!(TextureViewDescriptor<'_>: Send, Sync);
