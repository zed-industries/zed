#[cfg(wgpu_core)]
use core::ops::Deref;

use crate::*;

/// Handle to a texture on the GPU.
///
/// It can be created with [`Device::create_texture`].
///
/// Corresponds to [WebGPU `GPUTexture`](https://gpuweb.github.io/gpuweb/#texture-interface).
#[derive(Debug, Clone)]
pub struct Texture {
    pub(crate) inner: dispatch::DispatchTexture,
    pub(crate) descriptor: TextureDescriptor<'static>,
}
#[cfg(send_sync)]
static_assertions::assert_impl_all!(Texture: Send, Sync);

crate::cmp::impl_eq_ord_hash_proxy!(Texture => .inner);

impl Texture {
    /// Get the [`wgpu_hal`] texture from this `Texture`.
    ///
    /// Find the Api struct corresponding to the active backend in [`wgpu_hal::api`],
    /// and pass that struct to the to the `A` type parameter.
    ///
    /// Returns a guard that dereferences to the type of the hal backend
    /// which implements [`A::Texture`].
    ///
    /// # Types
    ///
    /// The returned type depends on the backend:
    ///
    #[doc = crate::macros::hal_type_vulkan!("Texture")]
    #[doc = crate::macros::hal_type_metal!("Texture")]
    #[doc = crate::macros::hal_type_dx12!("Texture")]
    #[doc = crate::macros::hal_type_gles!("Texture")]
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
    /// - The texture is not from the backend specified by `A`.
    /// - The texture is from the `webgpu` or `custom` backend.
    /// - The texture has had [`Self::destroy()`] called on it.
    ///
    /// # Safety
    ///
    /// - The returned resource must not be destroyed unless the guard
    ///   is the last reference to it and it is not in use by the GPU.
    ///   The guard and handle may be dropped at any time however.
    /// - All the safety requirements of wgpu-hal must be upheld.
    ///
    /// [`A::Texture`]: hal::Api::Texture
    #[cfg(wgpu_core)]
    pub unsafe fn as_hal<A: hal::Api>(&self) -> Option<impl Deref<Target = A::Texture>> {
        let texture = self.inner.as_core_opt()?;
        unsafe { texture.context.texture_as_hal::<A>(texture) }
    }

    #[cfg(custom)]
    /// Returns custom implementation of Texture (if custom backend and is internally T)
    pub fn as_custom<T: custom::TextureInterface>(&self) -> Option<&T> {
        self.inner.as_custom()
    }

    #[cfg(custom)]
    /// Creates a texture from already created custom implementation with the given description
    pub fn from_custom<T: custom::TextureInterface>(
        texture: T,
        desc: &TextureDescriptor<'_>,
    ) -> Self {
        Self {
            inner: dispatch::DispatchTexture::custom(texture),
            descriptor: TextureDescriptor {
                label: None,
                view_formats: &[],
                ..desc.clone()
            },
        }
    }

    /// Creates a view of this texture, specifying an interpretation of its texels and
    /// possibly a subset of its layers and mip levels.
    ///
    /// Texture views are needed to use a texture as a binding in a [`BindGroup`]
    /// or as an attachment in a [`RenderPass`].
    pub fn create_view(&self, desc: &TextureViewDescriptor<'_>) -> TextureView {
        let view = self.inner.create_view(desc);

        TextureView {
            inner: view,
            texture: self.clone(),
        }
    }

    /// Destroy the associated native resources as soon as possible.
    pub fn destroy(&self) {
        self.inner.destroy();
    }

    /// Make an `TexelCopyTextureInfo` representing the whole texture.
    pub fn as_image_copy(&self) -> TexelCopyTextureInfo<'_> {
        TexelCopyTextureInfo {
            texture: self,
            mip_level: 0,
            origin: Origin3d::ZERO,
            aspect: TextureAspect::All,
        }
    }

    /// Returns the size of this `Texture`.
    ///
    /// This is always equal to the `size` that was specified when creating the texture.
    pub fn size(&self) -> Extent3d {
        self.descriptor.size
    }

    /// Returns the width of this `Texture`.
    ///
    /// This is always equal to the `size.width` that was specified when creating the texture.
    pub fn width(&self) -> u32 {
        self.descriptor.size.width
    }

    /// Returns the height of this `Texture`.
    ///
    /// This is always equal to the `size.height` that was specified when creating the texture.
    pub fn height(&self) -> u32 {
        self.descriptor.size.height
    }

    /// Returns the depth or layer count of this `Texture`.
    ///
    /// This is always equal to the `size.depth_or_array_layers` that was specified when creating the texture.
    pub fn depth_or_array_layers(&self) -> u32 {
        self.descriptor.size.depth_or_array_layers
    }

    /// Returns the mip_level_count of this `Texture`.
    ///
    /// This is always equal to the `mip_level_count` that was specified when creating the texture.
    pub fn mip_level_count(&self) -> u32 {
        self.descriptor.mip_level_count
    }

    /// Returns the sample_count of this `Texture`.
    ///
    /// This is always equal to the `sample_count` that was specified when creating the texture.
    pub fn sample_count(&self) -> u32 {
        self.descriptor.sample_count
    }

    /// Returns the dimension of this `Texture`.
    ///
    /// This is always equal to the `dimension` that was specified when creating the texture.
    pub fn dimension(&self) -> TextureDimension {
        self.descriptor.dimension
    }

    /// Returns the format of this `Texture`.
    ///
    /// This is always equal to the `format` that was specified when creating the texture.
    pub fn format(&self) -> TextureFormat {
        self.descriptor.format
    }

    /// Returns the allowed usages of this `Texture`.
    ///
    /// This is always equal to the `usage` that was specified when creating the texture.
    pub fn usage(&self) -> TextureUsages {
        self.descriptor.usage
    }
}

/// Describes a [`Texture`].
///
/// For use with [`Device::create_texture`].
///
/// Corresponds to [WebGPU `GPUTextureDescriptor`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gputexturedescriptor).
pub type TextureDescriptor<'a> = wgt::TextureDescriptor<Label<'a>, &'a [TextureFormat]>;
static_assertions::assert_impl_all!(TextureDescriptor<'_>: Send, Sync);
