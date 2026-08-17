#[cfg(any(feature = "serde", test))]
use serde::{Deserialize, Serialize};

#[cfg(doc)]
use crate::TextureFormat;

/// Format of an `ExternalTexture`. This indicates the number of underlying
/// planes used by the `ExternalTexture` as well as each plane's format.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ExternalTextureFormat {
    /// Single [`TextureFormat::Rgba8Unorm`] or [`TextureFormat::Bgra8Unorm`] format plane.
    Rgba,
    /// [`TextureFormat::R8Unorm`] Y plane, and [`TextureFormat::Rg8Unorm`]
    /// interleaved CbCr plane.
    Nv12,
    /// Separate [`TextureFormat::R8Unorm`] Y, Cb, and Cr planes.
    Yu12,
}

/// Parameters describing a gamma encoding transfer function in the form
/// tf = { k * linear                   | linear < b
///      { a * pow(linear, 1/g) - (a-1) | linear >= b
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, bytemuck::Zeroable, bytemuck::Pod)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[allow(missing_docs)]
pub struct ExternalTextureTransferFunction {
    pub a: f32,
    pub b: f32,
    pub g: f32,
    pub k: f32,
}

impl Default for ExternalTextureTransferFunction {
    fn default() -> Self {
        Self {
            a: 1.0,
            b: 1.0,
            g: 1.0,
            k: 1.0,
        }
    }
}

/// Describes an [`ExternalTexture`](../wgpu/struct.ExternalTexture.html).
///
/// Note that [`width`] and [`height`] are the values that should be returned by
/// size queries in shader code; they do not necessarily match the dimensions of
/// the underlying plane texture(s). As a special case, if `(width, height)` is
/// `(0, 0)`, the actual size of the first underlying plane should be used instead.
///
/// The size given by [`width`] and [`height`] must be consistent with
/// [`sample_transform`]: they should be the size in texels of the rectangle
/// covered by the square (0,0)..(1,1) after [`sample_transform`] has been applied
/// to it.
///
/// [`width`]: Self::width
/// [`height`]: Self::height
/// [`sample_transform`]: Self::sample_transform
///
/// Corresponds to [WebGPU `GPUExternalTextureDescriptor`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpuexternaltexturedescriptor).
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExternalTextureDescriptor<L> {
    /// Debug label of the external texture. This will show up in graphics
    /// debuggers for easy identification.
    pub label: L,

    /// Width of the external texture.
    pub width: u32,

    /// Height of the external texture.
    pub height: u32,

    /// Format of the external texture.
    pub format: ExternalTextureFormat,

    /// 4x4 column-major matrix with which to convert sampled YCbCr values
    /// to RGBA.
    /// This is ignored when `format` is [`ExternalTextureFormat::Rgba`].
    pub yuv_conversion_matrix: [f32; 16],

    /// 3x3 column-major matrix to transform linear RGB values in the source
    /// color space to linear RGB values in the destination color space. In
    /// combination with [`Self::src_transfer_function`] and
    /// [`Self::dst_transfer_function`] this can be used to ensure that
    /// [`ImageSample`] and [`ImageLoad`] operations return values in the
    /// desired destination color space rather than the source color space of
    /// the underlying planes.
    ///
    /// [`ImageSample`]: https://docs.rs/naga/latest/naga/ir/enum.Expression.html#variant.ImageSample
    /// [`ImageLoad`]: https://docs.rs/naga/latest/naga/ir/enum.Expression.html#variant.ImageLoad
    pub gamut_conversion_matrix: [f32; 9],

    /// Transfer function for the source color space. The *inverse* of this
    /// will be applied to decode non-linear RGB to linear RGB in the source
    /// color space.
    pub src_transfer_function: ExternalTextureTransferFunction,

    /// Transfer function for the destination color space. This will be applied
    /// to encode linear RGB to non-linear RGB in the destination color space.
    pub dst_transfer_function: ExternalTextureTransferFunction,

    /// Transform to apply to [`ImageSample`] coordinates.
    ///
    /// This is a 3x2 column-major matrix representing an affine transform from
    /// normalized texture coordinates to the normalized coordinates that should
    /// be sampled from the external texture's underlying plane(s).
    ///
    /// This transform may scale, translate, flip, and rotate in 90-degree
    /// increments, but the result of transforming the rectangle (0,0)..(1,1)
    /// must be an axis-aligned rectangle that falls within the bounds of
    /// (0,0)..(1,1).
    ///
    /// [`ImageSample`]: https://docs.rs/naga/latest/naga/ir/enum.Expression.html#variant.ImageSample
    pub sample_transform: [f32; 6],

    /// Transform to apply to [`ImageLoad`] coordinates.
    ///
    /// This is a 3x2 column-major matrix representing an affine transform from
    /// non-normalized texel coordinates to the non-normalized coordinates of
    /// the texel that should be loaded from the external texture's underlying
    /// plane 0. For planes 1 and 2, if present, plane 0's coordinates are
    /// scaled according to the textures' relative sizes.
    ///
    /// This transform may scale, translate, flip, and rotate in 90-degree
    /// increments, but the result of transforming the rectangle (0,0)..([`width`],
    /// [`height`]) must be an axis-aligned rectangle that falls within the bounds
    /// of (0,0)..([`width`], [`height`]).
    ///
    /// [`ImageLoad`]: https://docs.rs/naga/latest/naga/ir/enum.Expression.html#variant.ImageLoad
    /// [`width`]: Self::width
    /// [`height`]: Self::height
    pub load_transform: [f32; 6],
}

impl<L> ExternalTextureDescriptor<L> {
    /// Takes a closure and maps the label of the external texture descriptor into another.
    #[must_use]
    pub fn map_label<K>(&self, fun: impl FnOnce(&L) -> K) -> ExternalTextureDescriptor<K> {
        ExternalTextureDescriptor {
            label: fun(&self.label),
            width: self.width,
            height: self.height,
            format: self.format,
            yuv_conversion_matrix: self.yuv_conversion_matrix,
            sample_transform: self.sample_transform,
            load_transform: self.load_transform,
            gamut_conversion_matrix: self.gamut_conversion_matrix,
            src_transfer_function: self.src_transfer_function,
            dst_transfer_function: self.dst_transfer_function,
        }
    }

    /// The number of underlying planes used by the external texture.
    pub fn num_planes(&self) -> usize {
        match self.format {
            ExternalTextureFormat::Rgba => 1,
            ExternalTextureFormat::Nv12 => 2,
            ExternalTextureFormat::Yu12 => 3,
        }
    }
}
