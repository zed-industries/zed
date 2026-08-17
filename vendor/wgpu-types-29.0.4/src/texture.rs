use core::ops::Range;

use crate::{link_to_wgpu_docs, link_to_wgpu_item, Extent3d, Origin3d};

#[cfg(any(feature = "serde", test))]
use serde::{Deserialize, Serialize};

#[cfg(doc)]
use crate::{BindingType, Features};

mod external_image;
mod external_texture;
mod format;

pub use external_image::*;
pub use external_texture::*;
pub use format::*;

/// Dimensionality of a texture.
///
/// Corresponds to [WebGPU `GPUTextureDimension`](
/// https://gpuweb.github.io/gpuweb/#enumdef-gputexturedimension).
#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TextureDimension {
    /// 1D texture
    #[cfg_attr(feature = "serde", serde(rename = "1d"))]
    D1,
    /// 2D texture
    #[cfg_attr(feature = "serde", serde(rename = "2d"))]
    D2,
    /// 3D texture
    #[cfg_attr(feature = "serde", serde(rename = "3d"))]
    D3,
}

/// Order in which texture data is laid out in memory.
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash)]
pub enum TextureDataOrder {
    /// The texture is laid out densely in memory as:
    ///
    /// ```text
    /// Layer0Mip0 Layer0Mip1 Layer0Mip2
    /// Layer1Mip0 Layer1Mip1 Layer1Mip2
    /// Layer2Mip0 Layer2Mip1 Layer2Mip2
    /// ````
    ///
    /// This is the layout used by dds files.
    #[default]
    LayerMajor,
    /// The texture is laid out densely in memory as:
    ///
    /// ```text
    /// Layer0Mip0 Layer1Mip0 Layer2Mip0
    /// Layer0Mip1 Layer1Mip1 Layer2Mip1
    /// Layer0Mip2 Layer1Mip2 Layer2Mip2
    /// ```
    ///
    /// This is the layout used by ktx and ktx2 files.
    MipMajor,
}

/// Dimensions of a particular texture view.
///
/// Corresponds to [WebGPU `GPUTextureViewDimension`](
/// https://gpuweb.github.io/gpuweb/#enumdef-gputextureviewdimension).
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TextureViewDimension {
    /// A one dimensional texture. `texture_1d` in WGSL and `texture1D` in GLSL.
    #[cfg_attr(feature = "serde", serde(rename = "1d"))]
    D1,
    /// A two dimensional texture. `texture_2d` in WGSL and `texture2D` in GLSL.
    #[cfg_attr(feature = "serde", serde(rename = "2d"))]
    #[default]
    D2,
    /// A two dimensional array texture. `texture_2d_array` in WGSL and `texture2DArray` in GLSL.
    #[cfg_attr(feature = "serde", serde(rename = "2d-array"))]
    D2Array,
    /// A cubemap texture. `texture_cube` in WGSL and `textureCube` in GLSL.
    #[cfg_attr(feature = "serde", serde(rename = "cube"))]
    Cube,
    /// A cubemap array texture. `texture_cube_array` in WGSL and `textureCubeArray` in GLSL.
    #[cfg_attr(feature = "serde", serde(rename = "cube-array"))]
    CubeArray,
    /// A three dimensional texture. `texture_3d` in WGSL and `texture3D` in GLSL.
    #[cfg_attr(feature = "serde", serde(rename = "3d"))]
    D3,
}

impl TextureViewDimension {
    /// Get the texture dimension required of this texture view dimension.
    #[must_use]
    pub fn compatible_texture_dimension(self) -> TextureDimension {
        match self {
            Self::D1 => TextureDimension::D1,
            Self::D2 | Self::D2Array | Self::Cube | Self::CubeArray => TextureDimension::D2,
            Self::D3 => TextureDimension::D3,
        }
    }
}

/// Selects a subset of the data a [`Texture`] holds.
///
/// Used in [texture views](TextureViewDescriptor) and
/// [texture copy operations](TexelCopyTextureInfo).
///
/// Corresponds to [WebGPU `GPUTextureAspect`](
/// https://gpuweb.github.io/gpuweb/#enumdef-gputextureaspect).
///
#[doc = link_to_wgpu_item!(struct Texture)]
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum TextureAspect {
    /// Depth, Stencil, and Color.
    #[default]
    All,
    /// Stencil.
    StencilOnly,
    /// Depth.
    DepthOnly,
    /// Plane 0.
    Plane0,
    /// Plane 1.
    Plane1,
    /// Plane 2.
    Plane2,
}

impl TextureAspect {
    /// Returns the texture aspect for a given plane.
    #[must_use]
    pub fn from_plane(plane: u32) -> Option<Self> {
        Some(match plane {
            0 => Self::Plane0,
            1 => Self::Plane1,
            2 => Self::Plane2,
            _ => return None,
        })
    }

    /// Returns the plane for a given texture aspect.
    #[must_use]
    pub fn to_plane(&self) -> Option<u32> {
        match self {
            TextureAspect::Plane0 => Some(0),
            TextureAspect::Plane1 => Some(1),
            TextureAspect::Plane2 => Some(2),
            _ => None,
        }
    }
}

bitflags::bitflags! {
    /// Different ways that you can use a texture.
    ///
    /// The usages determine what kind of memory the texture is allocated from and what
    /// actions the texture can partake in.
    ///
    /// Corresponds to [WebGPU `GPUTextureUsageFlags`](
    /// https://gpuweb.github.io/gpuweb/#typedefdef-gputextureusageflags).
    #[repr(transparent)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    #[cfg_attr(feature = "serde", serde(transparent))]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct TextureUsages: u32 {
        //
        // ---- Start numbering at 1 << 0 ----
        //
        // WebGPU features:
        //
        /// Allows a texture to be the source in a [`CommandEncoder::copy_texture_to_buffer`] or
        /// [`CommandEncoder::copy_texture_to_texture`] operation.
        const COPY_SRC = 1 << 0;
        /// Allows a texture to be the destination in a  [`CommandEncoder::copy_buffer_to_texture`],
        /// [`CommandEncoder::copy_texture_to_texture`], or [`Queue::write_texture`] operation.
        const COPY_DST = 1 << 1;
        /// Allows a texture to be a [`BindingType::Texture`] in a bind group.
        const TEXTURE_BINDING = 1 << 2;
        /// Allows a texture to be a [`BindingType::StorageTexture`] in a bind group.
        const STORAGE_BINDING = 1 << 3;
        /// Allows a texture to be an output attachment of a render pass.
        ///
        /// Consider adding [`TextureUsages::TRANSIENT`] if the contents are not reused.
        const RENDER_ATTACHMENT = 1 << 4;

        //
        // ---- Restart Numbering for Native Features ---
        //
        // Native Features:
        //
        /// Allows a texture to be used with image atomics. Requires [`Features::TEXTURE_ATOMIC`].
        const STORAGE_ATOMIC = 1 << 16;
        /// Specifies the contents of this texture will not be used in another pass to potentially reduce memory usage and bandwidth.
        ///
        /// No-op on platforms on platforms that do not benefit from transient textures.
        /// Generally mobile and Apple chips care about this.
        ///
        /// Incompatible with ALL other usages except [`TextureUsages::RENDER_ATTACHMENT`] and requires it.
        ///
        /// Requires [`StoreOp::Discard`].
        const TRANSIENT = 1 << 17;
    }
}

bitflags::bitflags! {
    /// Similar to `TextureUsages`, but used only for `CommandEncoder::transition_resources`.
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    #[cfg_attr(feature = "serde", serde(transparent))]
    pub struct TextureUses: u16 {
        /// The texture is in unknown state.
        const UNINITIALIZED = 1 << 0;
        /// Ready to present image to the surface.
        const PRESENT = 1 << 1;
        /// The source of a hardware copy.
        /// cbindgen:ignore
        const COPY_SRC = 1 << 2;
        /// The destination of a hardware copy.
        /// cbindgen:ignore
        const COPY_DST = 1 << 3;
        /// Read-only sampled or fetched resource.
        const RESOURCE = 1 << 4;
        /// The color target of a renderpass.
        const COLOR_TARGET = 1 << 5;
        /// Read-only depth stencil usage.
        const DEPTH_STENCIL_READ = 1 << 6;
        /// Read-write depth stencil usage
        const DEPTH_STENCIL_WRITE = 1 << 7;
        /// Read-only storage texture usage. Corresponds to a UAV in d3d, so is exclusive, despite being read only.
        /// cbindgen:ignore
        const STORAGE_READ_ONLY = 1 << 8;
        /// Write-only storage texture usage.
        /// cbindgen:ignore
        const STORAGE_WRITE_ONLY = 1 << 9;
        /// Read-write storage texture usage.
        /// cbindgen:ignore
        const STORAGE_READ_WRITE = 1 << 10;
        /// Image atomic enabled storage.
        /// cbindgen:ignore
        const STORAGE_ATOMIC = 1 << 11;
        /// Transient texture that may not have any backing memory. Not a resource state stored in the trackers, only used for passing down usages to create_texture.
        const TRANSIENT = 1 << 12;
        /// The combination of states that a texture may be in _at the same time_.
        /// cbindgen:ignore
        const INCLUSIVE = Self::COPY_SRC.bits() | Self::RESOURCE.bits() | Self::DEPTH_STENCIL_READ.bits() | Self::STORAGE_READ_ONLY.bits();
        /// The combination of states that a texture must exclusively be in.
        /// cbindgen:ignore
        const EXCLUSIVE = Self::COPY_DST.bits() | Self::COLOR_TARGET.bits() | Self::DEPTH_STENCIL_WRITE.bits() | Self::STORAGE_WRITE_ONLY.bits() | Self::STORAGE_READ_WRITE.bits() | Self::STORAGE_ATOMIC.bits() | Self::PRESENT.bits();

        /// Flag used by the wgpu-core texture tracker to say a texture is in different states for every sub-resource
        const COMPLEX = 1 << 13;
        /// Flag used by the wgpu-core texture tracker to say that the tracker does not know the state of the sub-resource.
        /// This is different from UNINITIALIZED as that says the tracker does know, but the texture has not been initialized.
        const UNKNOWN = 1 << 14;
    }
}

/// A texture transition for use with `CommandEncoder::transition_resources`.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TextureTransition<T> {
    /// The texture to transition.
    pub texture: T,
    /// An optional selector to transition only part of the texture.
    ///
    /// If None, the entire texture will be transitioned.
    pub selector: Option<TextureSelector>,
    /// The new state to transition to.
    pub state: TextureUses,
}

/// Specifies a particular set of subresources in a texture.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TextureSelector {
    /// Range of mips to use.
    pub mips: Range<u32>,
    /// Range of layers to use.
    pub layers: Range<u32>,
}

/// Specific type of a sample in a texture binding.
///
/// Corresponds to [WebGPU `GPUTextureSampleType`](
/// https://gpuweb.github.io/gpuweb/#enumdef-gputexturesampletype).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TextureSampleType {
    /// Sampling returns floats.
    ///
    /// Example WGSL syntax:
    /// ```rust,ignore
    /// @group(0) @binding(0)
    /// var t: texture_2d<f32>;
    /// ```
    ///
    /// Example GLSL syntax:
    /// ```cpp,ignore
    /// layout(binding = 0)
    /// uniform texture2D t;
    /// ```
    Float {
        /// If this is `false`, the texture can't be sampled with
        /// a filtering sampler.
        ///
        /// Even if this is `true`, it's possible to sample with
        /// a **non-filtering** sampler.
        filterable: bool,
    },
    /// Sampling does the depth reference comparison.
    ///
    /// This is also compatible with a non-filtering sampler.
    ///
    /// Example WGSL syntax:
    /// ```rust,ignore
    /// @group(0) @binding(0)
    /// var t: texture_depth_2d;
    /// ```
    ///
    /// Example GLSL syntax:
    /// ```cpp,ignore
    /// layout(binding = 0)
    /// uniform texture2DShadow t;
    /// ```
    Depth,
    /// Sampling returns signed integers.
    ///
    /// Example WGSL syntax:
    /// ```rust,ignore
    /// @group(0) @binding(0)
    /// var t: texture_2d<i32>;
    /// ```
    ///
    /// Example GLSL syntax:
    /// ```cpp,ignore
    /// layout(binding = 0)
    /// uniform itexture2D t;
    /// ```
    Sint,
    /// Sampling returns unsigned integers.
    ///
    /// Example WGSL syntax:
    /// ```rust,ignore
    /// @group(0) @binding(0)
    /// var t: texture_2d<u32>;
    /// ```
    ///
    /// Example GLSL syntax:
    /// ```cpp,ignore
    /// layout(binding = 0)
    /// uniform utexture2D t;
    /// ```
    Uint,
}

impl Default for TextureSampleType {
    fn default() -> Self {
        Self::Float { filterable: true }
    }
}

/// Specific type of a sample in a texture binding.
///
/// For use in [`BindingType::StorageTexture`].
///
/// Corresponds to [WebGPU `GPUStorageTextureAccess`](
/// https://gpuweb.github.io/gpuweb/#enumdef-gpustoragetextureaccess).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum StorageTextureAccess {
    /// The texture can only be written in the shader and it:
    /// - may or may not be annotated with `write` (WGSL).
    /// - must be annotated with `writeonly` (GLSL).
    ///
    /// Example WGSL syntax:
    /// ```rust,ignore
    /// @group(0) @binding(0)
    /// var my_storage_image: texture_storage_2d<r32float, write>;
    /// ```
    ///
    /// Example GLSL syntax:
    /// ```cpp,ignore
    /// layout(set=0, binding=0, r32f) writeonly uniform image2D myStorageImage;
    /// ```
    WriteOnly,
    /// The texture can only be read in the shader and it must be annotated with `read` (WGSL) or
    /// `readonly` (GLSL).
    ///
    /// [`Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES`] must be enabled to use this access
    /// mode. This is a native-only extension.
    ///
    /// Example WGSL syntax:
    /// ```rust,ignore
    /// @group(0) @binding(0)
    /// var my_storage_image: texture_storage_2d<r32float, read>;
    /// ```
    ///
    /// Example GLSL syntax:
    /// ```cpp,ignore
    /// layout(set=0, binding=0, r32f) readonly uniform image2D myStorageImage;
    /// ```
    ReadOnly,
    /// The texture can be both read and written in the shader and must be annotated with
    /// `read_write` in WGSL.
    ///
    /// [`Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES`] must be enabled to use this access
    /// mode.  This is a nonstandard, native-only extension.
    ///
    /// Example WGSL syntax:
    /// ```rust,ignore
    /// @group(0) @binding(0)
    /// var my_storage_image: texture_storage_2d<r32float, read_write>;
    /// ```
    ///
    /// Example GLSL syntax:
    /// ```cpp,ignore
    /// layout(set=0, binding=0, r32f) uniform image2D myStorageImage;
    /// ```
    ReadWrite,
    /// The texture can be both read and written in the shader via atomics and must be annotated
    /// with `read_write` in WGSL.
    ///
    /// [`Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES`] must be enabled to use this access
    /// mode.  This is a nonstandard, native-only extension.
    ///
    /// Example WGSL syntax:
    /// ```rust,ignore
    /// @group(0) @binding(0)
    /// var my_storage_image: texture_storage_2d<r32uint, atomic>;
    /// ```
    Atomic,
}

/// Describes a [`TextureView`].
///
/// For use with [`Texture::create_view()`].
///
/// Corresponds to [WebGPU `GPUTextureViewDescriptor`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gputextureviewdescriptor).
///
#[doc = link_to_wgpu_item!(struct TextureView)]
#[doc = link_to_wgpu_docs!(["`Texture::create_view()`"]: "struct.Texture.html#method.create_view")]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TextureViewDescriptor<L> {
    /// Debug label of the texture view. This will show up in graphics debuggers for easy identification.
    pub label: L,
    /// Format of the texture view. Either must be the same as the texture format or in the list
    /// of `view_formats` in the texture's descriptor.
    pub format: Option<TextureFormat>,
    /// The dimension of the texture view. For 1D textures, this must be `D1`. For 2D textures it must be one of
    /// `D2`, `D2Array`, `Cube`, and `CubeArray`. For 3D textures it must be `D3`
    pub dimension: Option<TextureViewDimension>,
    /// The allowed usage(s) for the texture view. Must be a subset of the usage flags of the texture.
    /// If not provided, defaults to the full set of usage flags of the texture.
    pub usage: Option<TextureUsages>,
    /// Aspect of the texture. Color textures must be [`TextureAspect::All`].
    pub aspect: TextureAspect,
    /// Base mip level.
    pub base_mip_level: u32,
    /// Mip level count.
    /// If `Some(count)`, `base_mip_level + count` must be less or equal to underlying texture mip count.
    /// If `None`, considered to include the rest of the mipmap levels, but at least 1 in total.
    pub mip_level_count: Option<u32>,
    /// Base array layer.
    pub base_array_layer: u32,
    /// Layer count.
    /// If `Some(count)`, `base_array_layer + count` must be less or equal to the underlying array count.
    /// If `None`, considered to include the rest of the array layers, but at least 1 in total.
    pub array_layer_count: Option<u32>,
}

/// Describes a [`Texture`](../wgpu/struct.Texture.html).
///
/// Corresponds to [WebGPU `GPUTextureDescriptor`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gputexturedescriptor).
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TextureDescriptor<L, V> {
    /// Debug label of the texture. This will show up in graphics debuggers for easy identification.
    pub label: L,
    /// Size of the texture. All components must be greater than zero. For a
    /// regular 1D/2D texture, the unused sizes will be 1. For 2DArray textures,
    /// Z is the number of 2D textures in that array.
    pub size: Extent3d,
    /// Mip count of texture. For a texture with no extra mips, this must be 1.
    pub mip_level_count: u32,
    /// Sample count of texture. If this is not 1, texture must have [`BindingType::Texture::multisampled`] set to true.
    pub sample_count: u32,
    /// Dimensions of the texture.
    pub dimension: TextureDimension,
    /// Format of the texture.
    pub format: TextureFormat,
    /// Allowed usages of the texture. If used in other ways, the operation will panic.
    pub usage: TextureUsages,
    /// Specifies what view formats will be allowed when calling `Texture::create_view` on this texture.
    ///
    /// View formats of the same format as the texture are always allowed.
    ///
    /// Note: currently, only the srgb-ness is allowed to change. (ex: `Rgba8Unorm` texture + `Rgba8UnormSrgb` view)
    pub view_formats: V,
}

impl<L, V> TextureDescriptor<L, V> {
    /// Takes a closure and maps the label of the texture descriptor into another.
    #[must_use]
    pub fn map_label<K>(&self, fun: impl FnOnce(&L) -> K) -> TextureDescriptor<K, V>
    where
        V: Clone,
    {
        TextureDescriptor {
            label: fun(&self.label),
            size: self.size,
            mip_level_count: self.mip_level_count,
            sample_count: self.sample_count,
            dimension: self.dimension,
            format: self.format,
            usage: self.usage,
            view_formats: self.view_formats.clone(),
        }
    }

    /// Maps the label and view formats of the texture descriptor into another.
    #[must_use]
    pub fn map_label_and_view_formats<K, M>(
        &self,
        l_fun: impl FnOnce(&L) -> K,
        v_fun: impl FnOnce(V) -> M,
    ) -> TextureDescriptor<K, M>
    where
        V: Clone,
    {
        TextureDescriptor {
            label: l_fun(&self.label),
            size: self.size,
            mip_level_count: self.mip_level_count,
            sample_count: self.sample_count,
            dimension: self.dimension,
            format: self.format,
            usage: self.usage,
            view_formats: v_fun(self.view_formats.clone()),
        }
    }

    /// Calculates the extent at a given mip level.
    ///
    /// If the given mip level is larger than possible, returns None.
    ///
    /// Treats the depth as part of the mipmaps. If calculating
    /// for a 2DArray texture, which does not mipmap depth, set depth to 1.
    ///
    /// ```rust
    /// # use wgpu_types as wgpu;
    /// # type TextureDescriptor<'a> = wgpu::TextureDescriptor<(), &'a [wgpu::TextureFormat]>;
    /// let desc  = TextureDescriptor {
    ///   label: (),
    ///   size: wgpu::Extent3d { width: 100, height: 60, depth_or_array_layers: 1 },
    ///   mip_level_count: 7,
    ///   sample_count: 1,
    ///   dimension: wgpu::TextureDimension::D3,
    ///   format: wgpu::TextureFormat::Rgba8Sint,
    ///   usage: wgpu::TextureUsages::empty(),
    ///   view_formats: &[],
    /// };
    ///
    /// assert_eq!(desc.mip_level_size(0), Some(wgpu::Extent3d { width: 100, height: 60, depth_or_array_layers: 1 }));
    /// assert_eq!(desc.mip_level_size(1), Some(wgpu::Extent3d { width: 50, height: 30, depth_or_array_layers: 1 }));
    /// assert_eq!(desc.mip_level_size(2), Some(wgpu::Extent3d { width: 25, height: 15, depth_or_array_layers: 1 }));
    /// assert_eq!(desc.mip_level_size(3), Some(wgpu::Extent3d { width: 12, height: 7, depth_or_array_layers: 1 }));
    /// assert_eq!(desc.mip_level_size(4), Some(wgpu::Extent3d { width: 6, height: 3, depth_or_array_layers: 1 }));
    /// assert_eq!(desc.mip_level_size(5), Some(wgpu::Extent3d { width: 3, height: 1, depth_or_array_layers: 1 }));
    /// assert_eq!(desc.mip_level_size(6), Some(wgpu::Extent3d { width: 1, height: 1, depth_or_array_layers: 1 }));
    /// assert_eq!(desc.mip_level_size(7), None);
    /// ```
    #[must_use]
    pub fn mip_level_size(&self, level: u32) -> Option<Extent3d> {
        if level >= self.mip_level_count {
            return None;
        }

        Some(self.size.mip_level_size(level, self.dimension))
    }

    /// Computes the render extent of this texture.
    ///
    /// This is a low-level helper exported for use by wgpu-core.
    ///
    /// <https://gpuweb.github.io/gpuweb/#abstract-opdef-compute-render-extent>
    ///
    /// # Panics
    ///
    /// If the mip level is out of range.
    #[doc(hidden)]
    #[must_use]
    pub fn compute_render_extent(&self, mip_level: u32, plane: Option<u32>) -> Extent3d {
        let Extent3d {
            width,
            height,
            depth_or_array_layers: _,
        } = self.mip_level_size(mip_level).expect("invalid mip level");

        let (w_subsampling, h_subsampling) = self.format.subsampling_factors(plane);

        let width = width / w_subsampling;
        let height = height / h_subsampling;

        Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        }
    }

    /// Returns the number of array layers.
    ///
    /// <https://gpuweb.github.io/gpuweb/#abstract-opdef-array-layer-count>
    #[must_use]
    pub fn array_layer_count(&self) -> u32 {
        match self.dimension {
            TextureDimension::D1 | TextureDimension::D3 => 1,
            TextureDimension::D2 => self.size.depth_or_array_layers,
        }
    }
}

/// Describes a `Sampler`.
///
/// For use with `Device::create_sampler`.
///
/// Corresponds to [WebGPU `GPUSamplerDescriptor`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpusamplerdescriptor).
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SamplerDescriptor<L> {
    /// Debug label of the sampler. This will show up in graphics debuggers for easy identification.
    pub label: L,
    /// How to deal with out of bounds accesses in the u (i.e. x) direction
    pub address_mode_u: AddressMode,
    /// How to deal with out of bounds accesses in the v (i.e. y) direction
    pub address_mode_v: AddressMode,
    /// How to deal with out of bounds accesses in the w (i.e. z) direction
    pub address_mode_w: AddressMode,
    /// How to filter the texture when it needs to be magnified (made larger)
    pub mag_filter: FilterMode,
    /// How to filter the texture when it needs to be minified (made smaller)
    pub min_filter: FilterMode,
    /// How to filter between mip map levels
    pub mipmap_filter: MipmapFilterMode,
    /// Minimum level of detail (i.e. mip level) to use
    pub lod_min_clamp: f32,
    /// Maximum level of detail (i.e. mip level) to use
    pub lod_max_clamp: f32,
    /// If this is enabled, this is a comparison sampler using the given comparison function.
    pub compare: Option<crate::CompareFunction>,
    /// Must be at least 1. If this is not 1, all filter modes must be linear.
    pub anisotropy_clamp: u16,
    /// Border color to use when `address_mode` is [`AddressMode::ClampToBorder`]
    pub border_color: Option<SamplerBorderColor>,
}

impl<L: Default> Default for SamplerDescriptor<L> {
    fn default() -> Self {
        Self {
            label: Default::default(),
            address_mode_u: Default::default(),
            address_mode_v: Default::default(),
            address_mode_w: Default::default(),
            mag_filter: Default::default(),
            min_filter: Default::default(),
            mipmap_filter: Default::default(),
            lod_min_clamp: 0.0,
            lod_max_clamp: 32.0,
            compare: None,
            anisotropy_clamp: 1,
            border_color: None,
        }
    }
}

/// How edges should be handled in texture addressing.
///
/// Corresponds to [WebGPU `GPUAddressMode`](
/// https://gpuweb.github.io/gpuweb/#enumdef-gpuaddressmode).
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum AddressMode {
    /// Clamp the value to the edge of the texture
    ///
    /// -0.25 -> 0.0
    /// 1.25  -> 1.0
    #[default]
    ClampToEdge = 0,
    /// Repeat the texture in a tiling fashion
    ///
    /// -0.25 -> 0.75
    /// 1.25 -> 0.25
    Repeat = 1,
    /// Repeat the texture, mirroring it every repeat
    ///
    /// -0.25 -> 0.25
    /// 1.25 -> 0.75
    MirrorRepeat = 2,
    /// Clamp the value to the border of the texture
    /// Requires feature [`Features::ADDRESS_MODE_CLAMP_TO_BORDER`]
    ///
    /// -0.25 -> border
    /// 1.25 -> border
    ClampToBorder = 3,
}

/// Texel mixing mode when sampling between texels.
///
/// Corresponds to [WebGPU `GPUFilterMode`](
/// https://gpuweb.github.io/gpuweb/#enumdef-gpufiltermode).
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum FilterMode {
    /// Nearest neighbor sampling.
    ///
    /// This creates a pixelated effect.
    #[default]
    Nearest = 0,
    /// Linear Interpolation
    ///
    /// This makes textures smooth but blurry.
    Linear = 1,
}

/// Texel mixing mode when sampling between texels.
///
/// Corresponds to [WebGPU `GPUMipmapFilterMode`](
/// https://gpuweb.github.io/gpuweb/#enumdef-gpumipmapfiltermode).
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum MipmapFilterMode {
    /// Nearest neighbor sampling.
    ///
    /// Return the value of the texel nearest to the texture coordinates.
    #[default]
    Nearest = 0,
    /// Linear Interpolation
    ///
    /// Select two texels in each dimension and return a linear interpolation between their values.
    Linear = 1,
}

/// Color variation to use when sampler addressing mode is [`AddressMode::ClampToBorder`]
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SamplerBorderColor {
    /// [0, 0, 0, 0]
    TransparentBlack,
    /// [0, 0, 0, 1]
    OpaqueBlack,
    /// [1, 1, 1, 1]
    OpaqueWhite,

    /// On the Metal backend, this is equivalent to `TransparentBlack` for
    /// textures that have an alpha component, and equivalent to `OpaqueBlack`
    /// for textures that do not have an alpha component. On other backends,
    /// this is equivalent to `TransparentBlack`. Requires
    /// [`Features::ADDRESS_MODE_CLAMP_TO_ZERO`]. Not supported on the web.
    Zero,
}

/// Layout of a texture in a buffer's memory.
///
/// The bytes per row and rows per image can be hard to figure out so here are some examples:
///
/// | Resolution | Format | Bytes per block | Pixels per block | Bytes per row                          | Rows per image               |
/// |------------|--------|-----------------|------------------|----------------------------------------|------------------------------|
/// | 256x256    | RGBA8  | 4               | 1 * 1 * 1        | 256 * 4 = Some(1024)                   | None                         |
/// | 32x16x8    | RGBA8  | 4               | 1 * 1 * 1        | 32 * 4 = 128 padded to 256 = Some(256) | None                         |
/// | 256x256    | BC3    | 16              | 4 * 4 * 1        | 16 * (256 / 4) = 1024 = Some(1024)     | None                         |
/// | 64x64x8    | BC3    | 16              | 4 * 4 * 1        | 16 * (64 / 4) = 256 = Some(256)        | 64 / 4 = 16 = Some(16)       |
///
/// Corresponds to [WebGPU `GPUTexelCopyBufferLayout`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpuimagedatalayout).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TexelCopyBufferLayout {
    /// Offset into the buffer that is the start of the texture. Must be a multiple of texture block size.
    /// For non-compressed textures, this is 1.
    pub offset: crate::BufferAddress,
    /// Bytes per "row" in an image.
    ///
    /// A row is one row of pixels or of compressed blocks in the x direction.
    ///
    /// This value is required if there are multiple rows (i.e. height or depth is more than one pixel or pixel block for compressed textures)
    ///
    /// Must be a multiple of 256 for [`CommandEncoder::copy_buffer_to_texture`][CEcbtt]
    /// and [`CommandEncoder::copy_texture_to_buffer`][CEcttb]. You must manually pad the
    /// buffer as if the image width is a multiple of 256. An image of size (500, 500) can be
    /// written to a buffer of size (512, 500) with `bytes_per_row` of 512,
    ///
    /// [`Queue::write_texture`][Qwt] does not have this requirement.
    ///
    /// Must be a multiple of the texture block size. For non-compressed textures, this is 1.
    ///
    #[doc = link_to_wgpu_docs!(["CEcbtt"]: "struct.CommandEncoder.html#method.copy_buffer_to_texture")]
    #[doc = link_to_wgpu_docs!(["CEcttb"]: "struct.CommandEncoder.html#method.copy_texture_to_buffer")]
    #[doc = link_to_wgpu_docs!(["Qwt"]: "struct.Queue.html#method.write_texture")]
    pub bytes_per_row: Option<u32>,
    /// "Rows" that make up a single "image".
    ///
    /// A row is one row of pixels or of compressed blocks in the x direction.
    ///
    /// An image is one layer in the z direction of a 3D image or 2DArray texture.
    ///
    /// The amount of rows per image may be larger than the actual amount of rows of data.
    ///
    /// Required if there are multiple images (i.e. the depth is more than one).
    pub rows_per_image: Option<u32>,
}

/// View of a buffer which can be used to copy to/from a texture.
///
/// Corresponds to [WebGPU `GPUTexelCopyBufferInfo`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpuimagecopybuffer).
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TexelCopyBufferInfo<B> {
    /// The buffer to be copied to/from.
    pub buffer: B,
    /// The layout of the texture data in this buffer.
    pub layout: TexelCopyBufferLayout,
}

/// View of a texture which can be used to copy to/from a buffer/texture.
///
/// Corresponds to [WebGPU `GPUTexelCopyTextureInfo`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpuimagecopytexture).
#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TexelCopyTextureInfo<T> {
    /// The texture to be copied to/from.
    pub texture: T,
    /// The target mip level of the texture.
    pub mip_level: u32,
    /// The base texel of the texture in the selected `mip_level`. Together
    /// with the `copy_size` argument to copy functions, defines the
    /// sub-region of the texture to copy.
    #[cfg_attr(feature = "serde", serde(default))]
    pub origin: Origin3d,
    /// The copy aspect.
    #[cfg_attr(feature = "serde", serde(default))]
    pub aspect: TextureAspect,
}

impl<T> TexelCopyTextureInfo<T> {
    /// Adds color space and premultiplied alpha information to make this
    /// descriptor tagged.
    pub fn to_tagged(
        self,
        color_space: PredefinedColorSpace,
        premultiplied_alpha: bool,
    ) -> CopyExternalImageDestInfo<T> {
        CopyExternalImageDestInfo {
            texture: self.texture,
            mip_level: self.mip_level,
            origin: self.origin,
            aspect: self.aspect,
            color_space,
            premultiplied_alpha,
        }
    }
}

/// Subresource range within an image
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct ImageSubresourceRange {
    /// Aspect of the texture. Color textures must be [`TextureAspect::All`][TAA].
    ///
    #[doc = link_to_wgpu_docs!(["TAA"]: "enum.TextureAspect.html#variant.All")]
    pub aspect: TextureAspect,
    /// Base mip level.
    pub base_mip_level: u32,
    /// Mip level count.
    /// If `Some(count)`, `base_mip_level + count` must be less or equal to underlying texture mip count.
    /// If `None`, considered to include the rest of the mipmap levels, but at least 1 in total.
    pub mip_level_count: Option<u32>,
    /// Base array layer.
    pub base_array_layer: u32,
    /// Layer count.
    /// If `Some(count)`, `base_array_layer + count` must be less or equal to the underlying array count.
    /// If `None`, considered to include the rest of the array layers, but at least 1 in total.
    pub array_layer_count: Option<u32>,
}

impl ImageSubresourceRange {
    /// Returns if the given range represents a full resource, with a texture of the given
    /// layer count and mip count.
    ///
    /// ```rust
    /// # use wgpu_types as wgpu;
    ///
    /// let range_none = wgpu::ImageSubresourceRange {
    ///     aspect: wgpu::TextureAspect::All,
    ///     base_mip_level: 0,
    ///     mip_level_count: None,
    ///     base_array_layer: 0,
    ///     array_layer_count: None,
    /// };
    /// assert_eq!(range_none.is_full_resource(wgpu::TextureFormat::Stencil8, 5, 10), true);
    ///
    /// let range_some = wgpu::ImageSubresourceRange {
    ///     aspect: wgpu::TextureAspect::All,
    ///     base_mip_level: 0,
    ///     mip_level_count: Some(5),
    ///     base_array_layer: 0,
    ///     array_layer_count: Some(10),
    /// };
    /// assert_eq!(range_some.is_full_resource(wgpu::TextureFormat::Stencil8, 5, 10), true);
    ///
    /// let range_mixed = wgpu::ImageSubresourceRange {
    ///     aspect: wgpu::TextureAspect::StencilOnly,
    ///     base_mip_level: 0,
    ///     // Only partial resource
    ///     mip_level_count: Some(3),
    ///     base_array_layer: 0,
    ///     array_layer_count: None,
    /// };
    /// assert_eq!(range_mixed.is_full_resource(wgpu::TextureFormat::Stencil8, 5, 10), false);
    /// ```
    #[must_use]
    pub fn is_full_resource(
        &self,
        format: TextureFormat,
        mip_levels: u32,
        array_layers: u32,
    ) -> bool {
        // Mip level count and array layer count need to deal with both the None and Some(count) case.
        let mip_level_count = self.mip_level_count.unwrap_or(mip_levels);
        let array_layer_count = self.array_layer_count.unwrap_or(array_layers);

        let aspect_eq = Some(format) == format.aspect_specific_format(self.aspect);

        let base_mip_level_eq = self.base_mip_level == 0;
        let mip_level_count_eq = mip_level_count == mip_levels;

        let base_array_layer_eq = self.base_array_layer == 0;
        let array_layer_count_eq = array_layer_count == array_layers;

        aspect_eq
            && base_mip_level_eq
            && mip_level_count_eq
            && base_array_layer_eq
            && array_layer_count_eq
    }

    /// Returns the mip level range of a subresource range describes for a specific texture.
    #[must_use]
    pub fn mip_range(&self, mip_level_count: u32) -> Range<u32> {
        self.base_mip_level..match self.mip_level_count {
            Some(mip_level_count) => self.base_mip_level + mip_level_count,
            None => mip_level_count,
        }
    }

    /// Returns the layer range of a subresource range describes for a specific texture.
    #[must_use]
    pub fn layer_range(&self, array_layer_count: u32) -> Range<u32> {
        self.base_array_layer..match self.array_layer_count {
            Some(array_layer_count) => self.base_array_layer + array_layer_count,
            None => array_layer_count,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Extent3d;

    #[test]
    fn test_physical_size() {
        let format = TextureFormat::Bc1RgbaUnormSrgb; // 4x4 blocks
        assert_eq!(
            Extent3d {
                width: 7,
                height: 7,
                depth_or_array_layers: 1
            }
            .physical_size(format),
            Extent3d {
                width: 8,
                height: 8,
                depth_or_array_layers: 1
            }
        );
        // Doesn't change, already aligned
        assert_eq!(
            Extent3d {
                width: 8,
                height: 8,
                depth_or_array_layers: 1
            }
            .physical_size(format),
            Extent3d {
                width: 8,
                height: 8,
                depth_or_array_layers: 1
            }
        );
        let format = TextureFormat::Astc {
            block: AstcBlock::B8x5,
            channel: AstcChannel::Unorm,
        }; // 8x5 blocks
        assert_eq!(
            Extent3d {
                width: 7,
                height: 7,
                depth_or_array_layers: 1
            }
            .physical_size(format),
            Extent3d {
                width: 8,
                height: 10,
                depth_or_array_layers: 1
            }
        );
    }

    #[test]
    fn test_max_mips() {
        // 1D
        assert_eq!(
            Extent3d {
                width: 240,
                height: 1,
                depth_or_array_layers: 1
            }
            .max_mips(TextureDimension::D1),
            1
        );
        // 2D
        assert_eq!(
            Extent3d {
                width: 1,
                height: 1,
                depth_or_array_layers: 1
            }
            .max_mips(TextureDimension::D2),
            1
        );
        assert_eq!(
            Extent3d {
                width: 60,
                height: 60,
                depth_or_array_layers: 1
            }
            .max_mips(TextureDimension::D2),
            6
        );
        assert_eq!(
            Extent3d {
                width: 240,
                height: 1,
                depth_or_array_layers: 1000
            }
            .max_mips(TextureDimension::D2),
            8
        );
        // 3D
        assert_eq!(
            Extent3d {
                width: 16,
                height: 30,
                depth_or_array_layers: 60
            }
            .max_mips(TextureDimension::D3),
            6
        );
    }
}
