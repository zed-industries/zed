use alloc::vec::Vec;

use crate::{Features, TextureAspect, TextureSampleType, TextureUsages};

#[cfg(any(feature = "serde", test))]
use serde::{Deserialize, Serialize};

/// ASTC block dimensions
#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AstcBlock {
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px).
    B4x4,
    /// 5x4 block compressed texture. 16 bytes per block (6.4 bit/px).
    B5x4,
    /// 5x5 block compressed texture. 16 bytes per block (5.12 bit/px).
    B5x5,
    /// 6x5 block compressed texture. 16 bytes per block (4.27 bit/px).
    B6x5,
    /// 6x6 block compressed texture. 16 bytes per block (3.56 bit/px).
    B6x6,
    /// 8x5 block compressed texture. 16 bytes per block (3.2 bit/px).
    B8x5,
    /// 8x6 block compressed texture. 16 bytes per block (2.67 bit/px).
    B8x6,
    /// 8x8 block compressed texture. 16 bytes per block (2 bit/px).
    B8x8,
    /// 10x5 block compressed texture. 16 bytes per block (2.56 bit/px).
    B10x5,
    /// 10x6 block compressed texture. 16 bytes per block (2.13 bit/px).
    B10x6,
    /// 10x8 block compressed texture. 16 bytes per block (1.6 bit/px).
    B10x8,
    /// 10x10 block compressed texture. 16 bytes per block (1.28 bit/px).
    B10x10,
    /// 12x10 block compressed texture. 16 bytes per block (1.07 bit/px).
    B12x10,
    /// 12x12 block compressed texture. 16 bytes per block (0.89 bit/px).
    B12x12,
}

/// ASTC RGBA channel
#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AstcChannel {
    /// 8 bit integer RGBA, [0, 255] converted to/from linear-color float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ASTC`] must be enabled to use this channel.
    Unorm,
    /// 8 bit integer RGBA, Srgb-color [0, 255] converted to/from linear-color float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ASTC`] must be enabled to use this channel.
    UnormSrgb,
    /// floating-point RGBA, linear-color float can be outside of the [0, 1] range.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ASTC_HDR`] must be enabled to use this channel.
    Hdr,
}

bitflags::bitflags! {
    /// Represents the different format channels of a texture format.
    #[repr(transparent)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    #[cfg_attr(feature = "serde", serde(transparent))]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct TextureChannel: u16 {
        /// Texture format contains a `red` channel.
        const RED = 1 << 0;
        /// Texture format contains a `green` channel.
        const GREEN = 1 << 1;
        /// Texture format contains a `blue` channel.
        const BLUE = 1 << 2;
        /// Texture format contains an `alpha` channel.
        const ALPHA = 1 << 3;
        /// Texture format contains a `stencil` channel.
        const STENCIL = 1 << 4;
        /// Texture format contains a `depth` channel.
        const DEPTH = 1 << 5;
        /// Texture format contains a `luminance` channel.
        const LUMINANCE = 1 << 6;
        /// Texture format contains a `blue-difference` channel.
        const CHROMINANCE_BLUE = 1 << 7;
        /// Texture format contains a `red-difference` channel.
        const CHROMINANCE_RED = 1 << 8;

        /// Texture format contains channels for `red` and `green`.
        const RG = Self::RED.bits() | Self::GREEN.bits();
        /// Texture format contains channels for `red`, `green` and `blue`.
        const RGB = Self::RG.bits() | Self::BLUE.bits();
        /// Texture format contains channels for `red`, `green`, `blue` and `alpha`.
        const RGBA = Self::RGB.bits() | Self::ALPHA.bits();
        /// Texture format contains channels for `depth` and `stencil`.
        const DEPTH_STENCIL =  Self::DEPTH.bits() | Self::STENCIL.bits();
        /// Texture format contains a `luminance` (`Y`), `blue-difference` (`Cb`) and `red-difference` (`Cr`) channel.
        const LUMINANCE_CHROMINANCE = Self::LUMINANCE.bits() | Self::CHROMINANCE_BLUE.bits() | Self::CHROMINANCE_RED.bits();
    }
}

/// Format in which a texture’s texels are stored in GPU memory.
///
/// Certain formats additionally specify a conversion.
/// When these formats are used in a shader, the conversion automatically takes place when loading
/// from or storing to the texture.
///
/// * `Unorm` formats linearly scale the integer range of the storage format to a floating-point
///   range of 0 to 1, inclusive.
/// * `Snorm` formats linearly scale the integer range of the storage format to a floating-point
///   range of &minus;1 to 1, inclusive, except that the most negative value
///   (&minus;128 for 8-bit, &minus;32768 for 16-bit) is excluded; on conversion,
///   it is treated as identical to the second most negative
///   (&minus;127 for 8-bit, &minus;32767 for 16-bit),
///   so that the positive and negative ranges are symmetric.
/// * `UnormSrgb` formats apply the [sRGB transfer function] so that the storage is sRGB encoded
///   while the shader works with linear intensity values.
/// * `Uint`, `Sint`, and `Float` formats perform no conversion.
///
/// Corresponds to [WebGPU `GPUTextureFormat`](
/// https://gpuweb.github.io/gpuweb/#enumdef-gputextureformat).
///
/// [sRGB transfer function]: https://en.wikipedia.org/wiki/SRGB#Transfer_function_(%22gamma%22)
#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum TextureFormat {
    // Normal 8 bit formats
    /// Red channel only. 8 bit integer per channel. [0, 255] converted to/from float [0, 1] in shader.
    R8Unorm,
    /// Red channel only. 8 bit integer per channel. [&minus;127, 127] converted to/from float [&minus;1, 1] in shader.
    R8Snorm,
    /// Red channel only. 8 bit integer per channel. Unsigned in shader.
    R8Uint,
    /// Red channel only. 8 bit integer per channel. Signed in shader.
    R8Sint,

    // Normal 16 bit formats
    /// Red channel only. 16 bit integer per channel. Unsigned in shader.
    R16Uint,
    /// Red channel only. 16 bit integer per channel. Signed in shader.
    R16Sint,
    /// Red channel only. 16 bit integer per channel. [0, 65535] converted to/from float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_FORMAT_16BIT_NORM`] must be enabled to use this texture format.
    R16Unorm,
    /// Red channel only. 16 bit integer per channel. [&minus;32767, 32767] converted to/from float [&minus;1, 1] in shader.
    ///
    /// [`Features::TEXTURE_FORMAT_16BIT_NORM`] must be enabled to use this texture format.
    R16Snorm,
    /// Red channel only. 16 bit float per channel. Float in shader.
    R16Float,
    /// Red and green channels. 8 bit integer per channel. [0, 255] converted to/from float [0, 1] in shader.
    Rg8Unorm,
    /// Red and green channels. 8 bit integer per channel. [&minus;127, 127] converted to/from float [&minus;1, 1] in shader.
    Rg8Snorm,
    /// Red and green channels. 8 bit integer per channel. Unsigned in shader.
    Rg8Uint,
    /// Red and green channels. 8 bit integer per channel. Signed in shader.
    Rg8Sint,

    // Normal 32 bit formats
    /// Red channel only. 32 bit integer per channel. Unsigned in shader.
    R32Uint,
    /// Red channel only. 32 bit integer per channel. Signed in shader.
    R32Sint,
    /// Red channel only. 32 bit float per channel. Float in shader.
    R32Float,
    /// Red and green channels. 16 bit integer per channel. Unsigned in shader.
    Rg16Uint,
    /// Red and green channels. 16 bit integer per channel. Signed in shader.
    Rg16Sint,
    /// Red and green channels. 16 bit integer per channel. [0, 65535] converted to/from float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_FORMAT_16BIT_NORM`] must be enabled to use this texture format.
    Rg16Unorm,
    /// Red and green channels. 16 bit integer per channel. [&minus;32767, 32767] converted to/from float [&minus;1, 1] in shader.
    ///
    /// [`Features::TEXTURE_FORMAT_16BIT_NORM`] must be enabled to use this texture format.
    Rg16Snorm,
    /// Red and green channels. 16 bit float per channel. Float in shader.
    Rg16Float,
    /// Red, green, blue, and alpha channels. 8 bit integer per channel. [0, 255] converted to/from float [0, 1] in shader.
    Rgba8Unorm,
    /// Red, green, blue, and alpha channels. 8 bit integer per channel. Srgb-color [0, 255] converted to/from linear-color float [0, 1] in shader.
    Rgba8UnormSrgb,
    /// Red, green, blue, and alpha channels. 8 bit integer per channel. [&minus;127, 127] converted to/from float [&minus;1, 1] in shader.
    Rgba8Snorm,
    /// Red, green, blue, and alpha channels. 8 bit integer per channel. Unsigned in shader.
    Rgba8Uint,
    /// Red, green, blue, and alpha channels. 8 bit integer per channel. Signed in shader.
    Rgba8Sint,
    /// Blue, green, red, and alpha channels. 8 bit integer per channel. [0, 255] converted to/from float [0, 1] in shader.
    Bgra8Unorm,
    /// Blue, green, red, and alpha channels. 8 bit integer per channel. Srgb-color [0, 255] converted to/from linear-color float [0, 1] in shader.
    Bgra8UnormSrgb,

    // Packed 32 bit formats
    /// Packed unsigned float with 9 bits mantisa for each RGB component, then a common 5 bits exponent
    Rgb9e5Ufloat,
    /// Red, green, blue, and alpha channels. 10 bit integer for RGB channels, 2 bit integer for alpha channel. Unsigned in shader.
    Rgb10a2Uint,
    /// Red, green, blue, and alpha channels. 10 bit integer for RGB channels, 2 bit integer for alpha channel. [0, 1023] ([0, 3] for alpha) converted to/from float [0, 1] in shader.
    Rgb10a2Unorm,
    /// Red, green, and blue channels. 11 bit float with no sign bit for RG channels. 10 bit float with no sign bit for blue channel. Float in shader.
    Rg11b10Ufloat,

    // Normal 64 bit formats
    /// Red channel only. 64 bit integer per channel. Unsigned in shader.
    ///
    /// [`Features::TEXTURE_INT64_ATOMIC`] must be enabled to use this texture format.
    R64Uint,
    /// Red and green channels. 32 bit integer per channel. Unsigned in shader.
    Rg32Uint,
    /// Red and green channels. 32 bit integer per channel. Signed in shader.
    Rg32Sint,
    /// Red and green channels. 32 bit float per channel. Float in shader.
    Rg32Float,
    /// Red, green, blue, and alpha channels. 16 bit integer per channel. Unsigned in shader.
    Rgba16Uint,
    /// Red, green, blue, and alpha channels. 16 bit integer per channel. Signed in shader.
    Rgba16Sint,
    /// Red, green, blue, and alpha channels. 16 bit integer per channel. [0, 65535] converted to/from float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_FORMAT_16BIT_NORM`] must be enabled to use this texture format.
    Rgba16Unorm,
    /// Red, green, blue, and alpha. 16 bit integer per channel. [&minus;32767, 32767] converted to/from float [&minus;1, 1] in shader.
    ///
    /// [`Features::TEXTURE_FORMAT_16BIT_NORM`] must be enabled to use this texture format.
    Rgba16Snorm,
    /// Red, green, blue, and alpha channels. 16 bit float per channel. Float in shader.
    Rgba16Float,

    // Normal 128 bit formats
    /// Red, green, blue, and alpha channels. 32 bit integer per channel. Unsigned in shader.
    Rgba32Uint,
    /// Red, green, blue, and alpha channels. 32 bit integer per channel. Signed in shader.
    Rgba32Sint,
    /// Red, green, blue, and alpha channels. 32 bit float per channel. Float in shader.
    Rgba32Float,

    // Depth and stencil formats
    /// Stencil format with 8 bit integer stencil.
    Stencil8,
    /// Special depth format with 16 bit integer depth.
    Depth16Unorm,
    /// Special depth format with at least 24 bit integer depth.
    Depth24Plus,
    /// Special depth/stencil format with at least 24 bit integer depth and 8 bits integer stencil.
    Depth24PlusStencil8,
    /// Special depth format with 32 bit floating point depth.
    Depth32Float,
    /// Special depth/stencil format with 32 bit floating point depth and 8 bits integer stencil.
    ///
    /// [`Features::DEPTH32FLOAT_STENCIL8`] must be enabled to use this texture format.
    Depth32FloatStencil8,

    /// YUV 4:2:0 chroma subsampled format.
    ///
    /// Contains two planes:
    /// - 0: Single 8 bit channel luminance.
    /// - 1: Dual 8 bit channel chrominance at half width and half height.
    ///
    /// Valid view formats for luminance are [`TextureFormat::R8Unorm`].
    ///
    /// Valid view formats for chrominance are [`TextureFormat::Rg8Unorm`].
    ///
    /// Width and height must be even.
    ///
    /// [`Features::TEXTURE_FORMAT_NV12`] must be enabled to use this texture format.
    NV12,

    /// YUV 4:2:0 chroma subsampled format.
    ///
    /// Contains two planes:
    /// - 0: Single 16 bit channel luminance, of which only the high 10 bits
    ///   are used.
    /// - 1: Dual 16 bit channel chrominance at half width and half height, of
    ///   which only the high 10 bits are used.
    ///
    /// Valid view formats for luminance are [`TextureFormat::R16Unorm`].
    ///
    /// Valid view formats for chrominance are [`TextureFormat::Rg16Unorm`].
    ///
    /// Width and height must be even.
    ///
    /// [`Features::TEXTURE_FORMAT_P010`] must be enabled to use this texture format.
    P010,

    // Compressed textures usable with `TEXTURE_COMPRESSION_BC` feature. `TEXTURE_COMPRESSION_SLICED_3D` is required to use with 3D textures.
    /// 4x4 block compressed texture. 8 bytes per block (4 bit/px). 4 color + alpha pallet. 5 bit R + 6 bit G + 5 bit B + 1 bit alpha.
    /// [0, 63] ([0, 1] for alpha) converted to/from float [0, 1] in shader.
    ///
    /// Also known as DXT1.
    ///
    /// [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
    /// [`Features::TEXTURE_COMPRESSION_BC_SLICED_3D`] must be enabled to use this texture format with 3D dimension.
    Bc1RgbaUnorm,
    /// 4x4 block compressed texture. 8 bytes per block (4 bit/px). 4 color + alpha pallet. 5 bit R + 6 bit G + 5 bit B + 1 bit alpha.
    /// Srgb-color [0, 63] ([0, 1] for alpha) converted to/from linear-color float [0, 1] in shader.
    ///
    /// Also known as DXT1.
    ///
    /// [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
    /// [`Features::TEXTURE_COMPRESSION_BC_SLICED_3D`] must be enabled to use this texture format with 3D dimension.
    Bc1RgbaUnormSrgb,
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px). 4 color pallet. 5 bit R + 6 bit G + 5 bit B + 4 bit alpha.
    /// [0, 63] ([0, 15] for alpha) converted to/from float [0, 1] in shader.
    ///
    /// Also known as DXT3.
    ///
    /// [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
    /// [`Features::TEXTURE_COMPRESSION_BC_SLICED_3D`] must be enabled to use this texture format with 3D dimension.
    Bc2RgbaUnorm,
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px). 4 color pallet. 5 bit R + 6 bit G + 5 bit B + 4 bit alpha.
    /// Srgb-color [0, 63] ([0, 255] for alpha) converted to/from linear-color float [0, 1] in shader.
    ///
    /// Also known as DXT3.
    ///
    /// [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
    /// [`Features::TEXTURE_COMPRESSION_BC_SLICED_3D`] must be enabled to use this texture format with 3D dimension.
    Bc2RgbaUnormSrgb,
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px). 4 color pallet + 8 alpha pallet. 5 bit R + 6 bit G + 5 bit B + 8 bit alpha.
    /// [0, 63] ([0, 255] for alpha) converted to/from float [0, 1] in shader.
    ///
    /// Also known as DXT5.
    ///
    /// [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
    /// [`Features::TEXTURE_COMPRESSION_BC_SLICED_3D`] must be enabled to use this texture format with 3D dimension.
    Bc3RgbaUnorm,
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px). 4 color pallet + 8 alpha pallet. 5 bit R + 6 bit G + 5 bit B + 8 bit alpha.
    /// Srgb-color [0, 63] ([0, 255] for alpha) converted to/from linear-color float [0, 1] in shader.
    ///
    /// Also known as DXT5.
    ///
    /// [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
    /// [`Features::TEXTURE_COMPRESSION_BC_SLICED_3D`] must be enabled to use this texture format with 3D dimension.
    Bc3RgbaUnormSrgb,
    /// 4x4 block compressed texture. 8 bytes per block (4 bit/px). 8 color pallet. 8 bit R.
    /// [0, 255] converted to/from float [0, 1] in shader.
    ///
    /// Also known as RGTC1.
    ///
    /// [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
    /// [`Features::TEXTURE_COMPRESSION_BC_SLICED_3D`] must be enabled to use this texture format with 3D dimension.
    Bc4RUnorm,
    /// 4x4 block compressed texture. 8 bytes per block (4 bit/px). 8 color pallet. 8 bit R.
    /// [&minus;127, 127] converted to/from float [&minus;1, 1] in shader.
    ///
    /// Also known as RGTC1.
    ///
    /// [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
    /// [`Features::TEXTURE_COMPRESSION_BC_SLICED_3D`] must be enabled to use this texture format with 3D dimension.
    Bc4RSnorm,
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px). 8 color red pallet + 8 color green pallet. 8 bit RG.
    /// [0, 255] converted to/from float [0, 1] in shader.
    ///
    /// Also known as RGTC2.
    ///
    /// [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
    /// [`Features::TEXTURE_COMPRESSION_BC_SLICED_3D`] must be enabled to use this texture format with 3D dimension.
    Bc5RgUnorm,
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px). 8 color red pallet + 8 color green pallet. 8 bit RG.
    /// [&minus;127, 127] converted to/from float [&minus;1, 1] in shader.
    ///
    /// Also known as RGTC2.
    ///
    /// [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
    /// [`Features::TEXTURE_COMPRESSION_BC_SLICED_3D`] must be enabled to use this texture format with 3D dimension.
    Bc5RgSnorm,
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px). Variable sized pallet. 16 bit unsigned float RGB. Float in shader.
    ///
    /// Also known as BPTC (float).
    ///
    /// [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
    /// [`Features::TEXTURE_COMPRESSION_BC_SLICED_3D`] must be enabled to use this texture format with 3D dimension.
    Bc6hRgbUfloat,
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px). Variable sized pallet. 16 bit signed float RGB. Float in shader.
    ///
    /// Also known as BPTC (float).
    ///
    /// [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
    /// [`Features::TEXTURE_COMPRESSION_BC_SLICED_3D`] must be enabled to use this texture format with 3D dimension.
    Bc6hRgbFloat,
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px). Variable sized pallet. 8 bit integer RGBA.
    /// [0, 255] converted to/from float [0, 1] in shader.
    ///
    /// Also known as BPTC (unorm).
    ///
    /// [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
    /// [`Features::TEXTURE_COMPRESSION_BC_SLICED_3D`] must be enabled to use this texture format with 3D dimension.
    Bc7RgbaUnorm,
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px). Variable sized pallet. 8 bit integer RGBA.
    /// Srgb-color [0, 255] converted to/from linear-color float [0, 1] in shader.
    ///
    /// Also known as BPTC (unorm).
    ///
    /// [`Features::TEXTURE_COMPRESSION_BC`] must be enabled to use this texture format.
    /// [`Features::TEXTURE_COMPRESSION_BC_SLICED_3D`] must be enabled to use this texture format with 3D dimension.
    Bc7RgbaUnormSrgb,
    /// 4x4 block compressed texture. 8 bytes per block (4 bit/px). Complex pallet. 8 bit integer RGB.
    /// [0, 255] converted to/from float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ETC2`] must be enabled to use this texture format.
    Etc2Rgb8Unorm,
    /// 4x4 block compressed texture. 8 bytes per block (4 bit/px). Complex pallet. 8 bit integer RGB.
    /// Srgb-color [0, 255] converted to/from linear-color float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ETC2`] must be enabled to use this texture format.
    Etc2Rgb8UnormSrgb,
    /// 4x4 block compressed texture. 8 bytes per block (4 bit/px). Complex pallet. 8 bit integer RGB + 1 bit alpha.
    /// [0, 255] ([0, 1] for alpha) converted to/from float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ETC2`] must be enabled to use this texture format.
    Etc2Rgb8A1Unorm,
    /// 4x4 block compressed texture. 8 bytes per block (4 bit/px). Complex pallet. 8 bit integer RGB + 1 bit alpha.
    /// Srgb-color [0, 255] ([0, 1] for alpha) converted to/from linear-color float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ETC2`] must be enabled to use this texture format.
    Etc2Rgb8A1UnormSrgb,
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px). Complex pallet. 8 bit integer RGB + 8 bit alpha.
    /// [0, 255] converted to/from float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ETC2`] must be enabled to use this texture format.
    Etc2Rgba8Unorm,
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px). Complex pallet. 8 bit integer RGB + 8 bit alpha.
    /// Srgb-color [0, 255] converted to/from linear-color float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ETC2`] must be enabled to use this texture format.
    Etc2Rgba8UnormSrgb,
    /// 4x4 block compressed texture. 8 bytes per block (4 bit/px). Complex pallet. 11 bit integer R.
    /// [0, 255] converted to/from float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ETC2`] must be enabled to use this texture format.
    EacR11Unorm,
    /// 4x4 block compressed texture. 8 bytes per block (4 bit/px). Complex pallet. 11 bit integer R.
    /// [&minus;127, 127] converted to/from float [&minus;1, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ETC2`] must be enabled to use this texture format.
    EacR11Snorm,
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px). Complex pallet. 11 bit integer R + 11 bit integer G.
    /// [0, 255] converted to/from float [0, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ETC2`] must be enabled to use this texture format.
    EacRg11Unorm,
    /// 4x4 block compressed texture. 16 bytes per block (8 bit/px). Complex pallet. 11 bit integer R + 11 bit integer G.
    /// [&minus;127, 127] converted to/from float [&minus;1, 1] in shader.
    ///
    /// [`Features::TEXTURE_COMPRESSION_ETC2`] must be enabled to use this texture format.
    EacRg11Snorm,
    /// block compressed texture. 16 bytes per block.
    ///
    /// Features [`TEXTURE_COMPRESSION_ASTC`] or [`TEXTURE_COMPRESSION_ASTC_HDR`]
    /// must be enabled to use this texture format.
    ///
    /// [`TEXTURE_COMPRESSION_ASTC`]: Features::TEXTURE_COMPRESSION_ASTC
    /// [`TEXTURE_COMPRESSION_ASTC_HDR`]: Features::TEXTURE_COMPRESSION_ASTC_HDR
    Astc {
        /// compressed block dimensions
        block: AstcBlock,
        /// ASTC RGBA channel
        channel: AstcChannel,
    },
}

// There are some additional texture format helpers in `wgpu-core/src/conv.rs`,
// that may need to be modified along with the ones here.
impl TextureFormat {
    /// Returns the aspect-specific format of the original format
    ///
    /// see <https://gpuweb.github.io/gpuweb/#abstract-opdef-resolving-gputextureaspect>
    #[must_use]
    pub fn aspect_specific_format(&self, aspect: TextureAspect) -> Option<Self> {
        match (*self, aspect) {
            (Self::Stencil8, TextureAspect::StencilOnly) => Some(*self),
            (
                Self::Depth16Unorm | Self::Depth24Plus | Self::Depth32Float,
                TextureAspect::DepthOnly,
            ) => Some(*self),
            (
                Self::Depth24PlusStencil8 | Self::Depth32FloatStencil8,
                TextureAspect::StencilOnly,
            ) => Some(Self::Stencil8),
            (Self::Depth24PlusStencil8, TextureAspect::DepthOnly) => Some(Self::Depth24Plus),
            (Self::Depth32FloatStencil8, TextureAspect::DepthOnly) => Some(Self::Depth32Float),
            (Self::NV12, TextureAspect::Plane0) => Some(Self::R8Unorm),
            (Self::NV12, TextureAspect::Plane1) => Some(Self::Rg8Unorm),
            (Self::P010, TextureAspect::Plane0) => Some(Self::R16Unorm),
            (Self::P010, TextureAspect::Plane1) => Some(Self::Rg16Unorm),
            // views to multi-planar formats must specify the plane
            (format, TextureAspect::All) if !format.is_multi_planar_format() => Some(format),
            _ => None,
        }
    }

    /// Returns `true` if `self` is a depth or stencil component of the given
    /// combined depth-stencil format
    #[must_use]
    pub fn is_depth_stencil_component(&self, combined_format: Self) -> bool {
        match (combined_format, *self) {
            (Self::Depth24PlusStencil8, Self::Depth24Plus | Self::Stencil8)
            | (Self::Depth32FloatStencil8, Self::Depth32Float | Self::Stencil8) => true,
            _ => false,
        }
    }

    /// Returns `true` if the format is a depth and/or stencil format
    ///
    /// see <https://gpuweb.github.io/gpuweb/#depth-formats>
    #[must_use]
    pub fn is_depth_stencil_format(&self) -> bool {
        self.channels().intersects(TextureChannel::DEPTH_STENCIL)
    }

    /// Returns `true` if the format is a combined depth-stencil format
    ///
    /// see <https://gpuweb.github.io/gpuweb/#combined-depth-stencil-format>
    #[must_use]
    pub fn is_combined_depth_stencil_format(&self) -> bool {
        self.channels().contains(TextureChannel::DEPTH_STENCIL)
    }

    /// Returns `true` if the format is a multi-planar format
    #[must_use]
    pub fn is_multi_planar_format(&self) -> bool {
        self.planes().is_some()
    }

    /// Returns the number of planes a multi-planar format has.
    #[must_use]
    pub fn planes(&self) -> Option<u32> {
        match *self {
            Self::NV12 => Some(2),
            Self::P010 => Some(2),
            _ => None,
        }
    }

    /// Returns the subsampling factor for the indicated plane of a multi-planar format.
    #[must_use]
    pub fn subsampling_factors(&self, plane: Option<u32>) -> (u32, u32) {
        match *self {
            Self::NV12 | Self::P010 => match plane {
                Some(0) => (1, 1),
                Some(1) => (2, 2),
                Some(plane) => unreachable!("plane {plane} is not valid for {self:?}"),
                None => unreachable!("the plane must be specified for multi-planar formats"),
            },
            _ => (1, 1),
        }
    }

    /// Returns a [TextureChannel] with the bits set where the texture format contains
    /// the respective channels.
    ///
    /// # Example
    /// ```rust
    /// # use wgpu_types::{TextureFormat, TextureChannel};
    ///
    /// // `Rgba` has a `red`, `green`, `blue` and `alpha` channel!
    /// assert!(TextureFormat::Rgba8Unorm.channels().contains(TextureChannel::RGBA));
    ///
    /// // `Rg` hasn't got an alpha channel...
    /// assert!(!TextureFormat::Rg8Unorm.channels().contains(TextureChannel::ALPHA));
    /// // ... but it has a red channel ...
    /// assert!(TextureFormat::Rg8Unorm.channels().contains(TextureChannel::RED));
    /// // ... and also a green channel (yay!)
    /// assert!(TextureFormat::Rg8Unorm.channels().contains(TextureChannel::GREEN));
    ///
    /// // A stencil texture has a... stencil channel
    /// assert!(TextureFormat::Stencil8.channels().contains(TextureChannel::STENCIL));
    /// // And the `NV12` format should have a `luminance`, `blue-difference` and `red-difference` channel.
    /// assert!(TextureFormat::NV12.channels().contains(TextureChannel::LUMINANCE_CHROMINANCE));
    /// ```
    #[must_use]
    pub fn channels(&self) -> TextureChannel {
        match self {
            Self::R8Unorm
            | Self::R8Snorm
            | Self::R8Uint
            | Self::R8Sint
            | Self::R16Uint
            | Self::R16Sint
            | Self::R16Unorm
            | Self::R16Snorm
            | Self::R32Uint
            | Self::R32Sint
            | Self::R32Float
            | Self::R16Float
            | Self::R64Uint
            | Self::Bc4RUnorm
            | Self::Bc4RSnorm
            | Self::EacR11Unorm
            | Self::EacR11Snorm => TextureChannel::RED,

            Self::Rg8Unorm
            | Self::Rg8Snorm
            | Self::Rg8Uint
            | Self::Rg8Sint
            | Self::Rg16Uint
            | Self::Rg16Sint
            | Self::Rg16Unorm
            | Self::Rg16Snorm
            | Self::Rg16Float
            | Self::Rg32Uint
            | Self::Rg32Sint
            | Self::Rg32Float
            | Self::Bc5RgUnorm
            | Self::Bc5RgSnorm
            | Self::EacRg11Unorm
            | Self::EacRg11Snorm => TextureChannel::RG,

            Self::Rgb9e5Ufloat
            | Self::Rg11b10Ufloat
            | Self::Bc6hRgbUfloat
            | Self::Bc6hRgbFloat
            | Self::Etc2Rgb8Unorm
            | Self::Etc2Rgb8UnormSrgb => TextureChannel::RGB,

            Self::Rgba8Unorm
            | Self::Rgba8UnormSrgb
            | Self::Rgba8Snorm
            | Self::Rgba8Uint
            | Self::Rgba8Sint
            | Self::Bgra8Unorm
            | Self::Bgra8UnormSrgb
            | Self::Rgb10a2Uint
            | Self::Rgb10a2Unorm
            | Self::Rgba16Uint
            | Self::Rgba16Sint
            | Self::Rgba16Unorm
            | Self::Rgba16Snorm
            | Self::Rgba16Float
            | Self::Rgba32Uint
            | Self::Rgba32Sint
            | Self::Rgba32Float
            | Self::Bc1RgbaUnorm
            | Self::Bc1RgbaUnormSrgb
            | Self::Bc2RgbaUnorm
            | Self::Bc2RgbaUnormSrgb
            | Self::Bc3RgbaUnorm
            | Self::Bc3RgbaUnormSrgb
            | Self::Bc7RgbaUnorm
            | Self::Bc7RgbaUnormSrgb
            | Self::Etc2Rgb8A1Unorm
            | Self::Etc2Rgb8A1UnormSrgb
            | Self::Etc2Rgba8Unorm
            | Self::Etc2Rgba8UnormSrgb
            | Self::Astc { .. } => TextureChannel::RGBA,

            Self::Stencil8 => TextureChannel::STENCIL,
            Self::Depth16Unorm | Self::Depth24Plus | Self::Depth32Float => TextureChannel::DEPTH,

            Self::Depth24PlusStencil8 | Self::Depth32FloatStencil8 => TextureChannel::DEPTH_STENCIL,

            Self::NV12 | Self::P010 => TextureChannel::LUMINANCE_CHROMINANCE,
        }
    }

    /// Returns `true` if the format has a color aspect
    #[must_use]
    pub fn has_color_aspect(&self) -> bool {
        self.channels().intersects(TextureChannel::RGBA)
    }

    /// Returns `true` if the format has a depth aspect
    #[must_use]
    pub fn has_depth_aspect(&self) -> bool {
        self.channels().intersects(TextureChannel::DEPTH)
    }

    /// Returns `true` if the format has a stencil aspect
    #[must_use]
    pub fn has_stencil_aspect(&self) -> bool {
        self.channels().intersects(TextureChannel::STENCIL)
    }

    /// Returns the size multiple requirement for a texture using this format.
    ///
    /// `create_texture` currently enforces a stricter restriction than this for
    /// mipmapped multi-planar formats.
    /// TODO(<https://github.com/gfx-rs/wgpu/issues/8491>): Remove this note.
    #[must_use]
    pub fn size_multiple_requirement(&self) -> (u32, u32) {
        match *self {
            Self::NV12 => (2, 2),
            Self::P010 => (2, 2),
            _ => self.block_dimensions(),
        }
    }

    /// Returns the dimension of a [block](https://gpuweb.github.io/gpuweb/#texel-block) of texels.
    ///
    /// Uncompressed formats have a block dimension of `(1, 1)`.
    #[must_use]
    pub fn block_dimensions(&self) -> (u32, u32) {
        match *self {
            Self::R8Unorm
            | Self::R8Snorm
            | Self::R8Uint
            | Self::R8Sint
            | Self::R16Uint
            | Self::R16Sint
            | Self::R16Unorm
            | Self::R16Snorm
            | Self::R16Float
            | Self::Rg8Unorm
            | Self::Rg8Snorm
            | Self::Rg8Uint
            | Self::Rg8Sint
            | Self::R32Uint
            | Self::R32Sint
            | Self::R32Float
            | Self::Rg16Uint
            | Self::Rg16Sint
            | Self::Rg16Unorm
            | Self::Rg16Snorm
            | Self::Rg16Float
            | Self::Rgba8Unorm
            | Self::Rgba8UnormSrgb
            | Self::Rgba8Snorm
            | Self::Rgba8Uint
            | Self::Rgba8Sint
            | Self::Bgra8Unorm
            | Self::Bgra8UnormSrgb
            | Self::Rgb9e5Ufloat
            | Self::Rgb10a2Uint
            | Self::Rgb10a2Unorm
            | Self::Rg11b10Ufloat
            | Self::R64Uint
            | Self::Rg32Uint
            | Self::Rg32Sint
            | Self::Rg32Float
            | Self::Rgba16Uint
            | Self::Rgba16Sint
            | Self::Rgba16Unorm
            | Self::Rgba16Snorm
            | Self::Rgba16Float
            | Self::Rgba32Uint
            | Self::Rgba32Sint
            | Self::Rgba32Float
            | Self::Stencil8
            | Self::Depth16Unorm
            | Self::Depth24Plus
            | Self::Depth24PlusStencil8
            | Self::Depth32Float
            | Self::Depth32FloatStencil8
            | Self::NV12
            | Self::P010 => (1, 1),

            Self::Bc1RgbaUnorm
            | Self::Bc1RgbaUnormSrgb
            | Self::Bc2RgbaUnorm
            | Self::Bc2RgbaUnormSrgb
            | Self::Bc3RgbaUnorm
            | Self::Bc3RgbaUnormSrgb
            | Self::Bc4RUnorm
            | Self::Bc4RSnorm
            | Self::Bc5RgUnorm
            | Self::Bc5RgSnorm
            | Self::Bc6hRgbUfloat
            | Self::Bc6hRgbFloat
            | Self::Bc7RgbaUnorm
            | Self::Bc7RgbaUnormSrgb => (4, 4),

            Self::Etc2Rgb8Unorm
            | Self::Etc2Rgb8UnormSrgb
            | Self::Etc2Rgb8A1Unorm
            | Self::Etc2Rgb8A1UnormSrgb
            | Self::Etc2Rgba8Unorm
            | Self::Etc2Rgba8UnormSrgb
            | Self::EacR11Unorm
            | Self::EacR11Snorm
            | Self::EacRg11Unorm
            | Self::EacRg11Snorm => (4, 4),

            Self::Astc { block, .. } => match block {
                AstcBlock::B4x4 => (4, 4),
                AstcBlock::B5x4 => (5, 4),
                AstcBlock::B5x5 => (5, 5),
                AstcBlock::B6x5 => (6, 5),
                AstcBlock::B6x6 => (6, 6),
                AstcBlock::B8x5 => (8, 5),
                AstcBlock::B8x6 => (8, 6),
                AstcBlock::B8x8 => (8, 8),
                AstcBlock::B10x5 => (10, 5),
                AstcBlock::B10x6 => (10, 6),
                AstcBlock::B10x8 => (10, 8),
                AstcBlock::B10x10 => (10, 10),
                AstcBlock::B12x10 => (12, 10),
                AstcBlock::B12x12 => (12, 12),
            },
        }
    }

    /// Returns `true` for compressed formats.
    #[must_use]
    pub fn is_compressed(&self) -> bool {
        self.block_dimensions() != (1, 1)
    }

    /// Returns `true` for BCn compressed formats.
    #[must_use]
    pub fn is_bcn(&self) -> bool {
        self.required_features() == Features::TEXTURE_COMPRESSION_BC
    }

    /// Returns `true` for ASTC compressed formats.
    #[must_use]
    pub fn is_astc(&self) -> bool {
        self.required_features() == Features::TEXTURE_COMPRESSION_ASTC
            || self.required_features() == Features::TEXTURE_COMPRESSION_ASTC_HDR
    }

    /// Returns the required features (if any) in order to use the texture.
    #[must_use]
    pub fn required_features(&self) -> Features {
        match *self {
            Self::R8Unorm
            | Self::R8Snorm
            | Self::R8Uint
            | Self::R8Sint
            | Self::R16Uint
            | Self::R16Sint
            | Self::R16Float
            | Self::Rg8Unorm
            | Self::Rg8Snorm
            | Self::Rg8Uint
            | Self::Rg8Sint
            | Self::R32Uint
            | Self::R32Sint
            | Self::R32Float
            | Self::Rg16Uint
            | Self::Rg16Sint
            | Self::Rg16Float
            | Self::Rgba8Unorm
            | Self::Rgba8UnormSrgb
            | Self::Rgba8Snorm
            | Self::Rgba8Uint
            | Self::Rgba8Sint
            | Self::Bgra8Unorm
            | Self::Bgra8UnormSrgb
            | Self::Rgb9e5Ufloat
            | Self::Rgb10a2Uint
            | Self::Rgb10a2Unorm
            | Self::Rg11b10Ufloat
            | Self::Rg32Uint
            | Self::Rg32Sint
            | Self::Rg32Float
            | Self::Rgba16Uint
            | Self::Rgba16Sint
            | Self::Rgba16Float
            | Self::Rgba32Uint
            | Self::Rgba32Sint
            | Self::Rgba32Float
            | Self::Stencil8
            | Self::Depth16Unorm
            | Self::Depth24Plus
            | Self::Depth24PlusStencil8
            | Self::Depth32Float => Features::empty(),

            Self::R64Uint => Features::TEXTURE_INT64_ATOMIC,

            Self::Depth32FloatStencil8 => Features::DEPTH32FLOAT_STENCIL8,

            Self::NV12 => Features::TEXTURE_FORMAT_NV12,
            Self::P010 => Features::TEXTURE_FORMAT_P010,

            Self::R16Unorm
            | Self::R16Snorm
            | Self::Rg16Unorm
            | Self::Rg16Snorm
            | Self::Rgba16Unorm
            | Self::Rgba16Snorm => Features::TEXTURE_FORMAT_16BIT_NORM,

            Self::Bc1RgbaUnorm
            | Self::Bc1RgbaUnormSrgb
            | Self::Bc2RgbaUnorm
            | Self::Bc2RgbaUnormSrgb
            | Self::Bc3RgbaUnorm
            | Self::Bc3RgbaUnormSrgb
            | Self::Bc4RUnorm
            | Self::Bc4RSnorm
            | Self::Bc5RgUnorm
            | Self::Bc5RgSnorm
            | Self::Bc6hRgbUfloat
            | Self::Bc6hRgbFloat
            | Self::Bc7RgbaUnorm
            | Self::Bc7RgbaUnormSrgb => Features::TEXTURE_COMPRESSION_BC,

            Self::Etc2Rgb8Unorm
            | Self::Etc2Rgb8UnormSrgb
            | Self::Etc2Rgb8A1Unorm
            | Self::Etc2Rgb8A1UnormSrgb
            | Self::Etc2Rgba8Unorm
            | Self::Etc2Rgba8UnormSrgb
            | Self::EacR11Unorm
            | Self::EacR11Snorm
            | Self::EacRg11Unorm
            | Self::EacRg11Snorm => Features::TEXTURE_COMPRESSION_ETC2,

            Self::Astc { channel, .. } => match channel {
                AstcChannel::Hdr => Features::TEXTURE_COMPRESSION_ASTC_HDR,
                AstcChannel::Unorm | AstcChannel::UnormSrgb => Features::TEXTURE_COMPRESSION_ASTC,
            },
        }
    }

    /// Returns the format features guaranteed by the WebGPU spec.
    ///
    /// Additional features are available if `Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES` is enabled.
    #[must_use]
    pub fn guaranteed_format_features(&self, device_features: Features) -> TextureFormatFeatures {
        // Multisampling
        let none = TextureFormatFeatureFlags::empty();
        let msaa = TextureFormatFeatureFlags::MULTISAMPLE_X4;
        let msaa_resolve = msaa | TextureFormatFeatureFlags::MULTISAMPLE_RESOLVE;

        let s_ro_wo = TextureFormatFeatureFlags::STORAGE_READ_ONLY
            | TextureFormatFeatureFlags::STORAGE_WRITE_ONLY;
        let s_all = s_ro_wo | TextureFormatFeatureFlags::STORAGE_READ_WRITE;

        // Flags
        let basic =
            TextureUsages::COPY_SRC | TextureUsages::COPY_DST | TextureUsages::TEXTURE_BINDING;
        let attachment = basic | TextureUsages::RENDER_ATTACHMENT | TextureUsages::TRANSIENT;
        let storage = basic | TextureUsages::STORAGE_BINDING;
        let binding = TextureUsages::TEXTURE_BINDING;
        let all_flags = attachment | storage | binding;
        let atomic_64 = if device_features.contains(Features::TEXTURE_ATOMIC) {
            storage | binding | TextureUsages::STORAGE_ATOMIC
        } else {
            storage | binding
        };
        let atomic = attachment | atomic_64;
        let (rg11b10f_f, rg11b10f_u) =
            if device_features.contains(Features::RG11B10UFLOAT_RENDERABLE) {
                (msaa_resolve, attachment)
            } else {
                (msaa, basic)
            };
        let (bgra8unorm_f, bgra8unorm) = if device_features.contains(Features::BGRA8UNORM_STORAGE) {
            (
                msaa_resolve | TextureFormatFeatureFlags::STORAGE_WRITE_ONLY,
                attachment | TextureUsages::STORAGE_BINDING,
            )
        } else {
            (msaa_resolve, attachment)
        };

        #[rustfmt::skip] // lets make a nice table
        let (
            mut flags,
            allowed_usages,
        ) = match *self {
            Self::R8Unorm =>              (msaa_resolve, attachment),
            Self::R8Snorm =>              (        none,      basic),
            Self::R8Uint =>               (        msaa, attachment),
            Self::R8Sint =>               (        msaa, attachment),
            Self::R16Uint =>              (        msaa, attachment),
            Self::R16Sint =>              (        msaa, attachment),
            Self::R16Float =>             (msaa_resolve, attachment),
            Self::Rg8Unorm =>             (msaa_resolve, attachment),
            Self::Rg8Snorm =>             (        none,      basic),
            Self::Rg8Uint =>              (        msaa, attachment),
            Self::Rg8Sint =>              (        msaa, attachment),
            Self::R32Uint =>              (       s_all,     atomic),
            Self::R32Sint =>              (       s_all,     atomic),
            Self::R32Float =>             (msaa | s_all,  all_flags),
            Self::Rg16Uint =>             (        msaa, attachment),
            Self::Rg16Sint =>             (        msaa, attachment),
            Self::Rg16Float =>            (msaa_resolve, attachment),
            Self::Rgba8Unorm =>           (msaa_resolve | s_ro_wo,  all_flags),
            Self::Rgba8UnormSrgb =>       (msaa_resolve, attachment),
            Self::Rgba8Snorm =>           (     s_ro_wo,    storage),
            Self::Rgba8Uint =>            (        msaa | s_ro_wo,  all_flags),
            Self::Rgba8Sint =>            (        msaa | s_ro_wo,  all_flags),
            Self::Bgra8Unorm =>           (bgra8unorm_f, bgra8unorm),
            Self::Bgra8UnormSrgb =>       (msaa_resolve, attachment),
            Self::Rgb10a2Uint =>          (        msaa, attachment),
            Self::Rgb10a2Unorm =>         (msaa_resolve, attachment),
            Self::Rg11b10Ufloat =>        (  rg11b10f_f, rg11b10f_u),
            Self::R64Uint =>              (     s_ro_wo,  atomic_64),
            Self::Rg32Uint =>             (     s_ro_wo,  all_flags),
            Self::Rg32Sint =>             (     s_ro_wo,  all_flags),
            Self::Rg32Float =>            (     s_ro_wo,  all_flags),
            Self::Rgba16Uint =>           (        msaa | s_ro_wo,  all_flags),
            Self::Rgba16Sint =>           (        msaa | s_ro_wo,  all_flags),
            Self::Rgba16Float =>          (msaa_resolve | s_ro_wo,  all_flags),
            Self::Rgba32Uint =>           (     s_ro_wo,  all_flags),
            Self::Rgba32Sint =>           (     s_ro_wo,  all_flags),
            Self::Rgba32Float =>          (     s_ro_wo,  all_flags),

            Self::Stencil8 =>             (        msaa, attachment),
            Self::Depth16Unorm =>         (        msaa, attachment),
            Self::Depth24Plus =>          (        msaa, attachment),
            Self::Depth24PlusStencil8 =>  (        msaa, attachment),
            Self::Depth32Float =>         (        msaa, attachment),
            Self::Depth32FloatStencil8 => (        msaa, attachment),

            // We only support sampling nv12 and p010 textures until we
            // implement transfer plane data.
            Self::NV12 =>                 (        none,    binding),
            Self::P010 =>                 (        none,    binding),

            Self::R16Unorm =>             (        msaa | s_ro_wo,    storage),
            Self::R16Snorm =>             (        msaa | s_ro_wo,    storage),
            Self::Rg16Unorm =>            (        msaa | s_ro_wo,    storage),
            Self::Rg16Snorm =>            (        msaa | s_ro_wo,    storage),
            Self::Rgba16Unorm =>          (        msaa | s_ro_wo,    storage),
            Self::Rgba16Snorm =>          (        msaa | s_ro_wo,    storage),

            Self::Rgb9e5Ufloat =>         (        none,      basic),

            Self::Bc1RgbaUnorm =>         (        none,      basic),
            Self::Bc1RgbaUnormSrgb =>     (        none,      basic),
            Self::Bc2RgbaUnorm =>         (        none,      basic),
            Self::Bc2RgbaUnormSrgb =>     (        none,      basic),
            Self::Bc3RgbaUnorm =>         (        none,      basic),
            Self::Bc3RgbaUnormSrgb =>     (        none,      basic),
            Self::Bc4RUnorm =>            (        none,      basic),
            Self::Bc4RSnorm =>            (        none,      basic),
            Self::Bc5RgUnorm =>           (        none,      basic),
            Self::Bc5RgSnorm =>           (        none,      basic),
            Self::Bc6hRgbUfloat =>        (        none,      basic),
            Self::Bc6hRgbFloat =>         (        none,      basic),
            Self::Bc7RgbaUnorm =>         (        none,      basic),
            Self::Bc7RgbaUnormSrgb =>     (        none,      basic),

            Self::Etc2Rgb8Unorm =>        (        none,      basic),
            Self::Etc2Rgb8UnormSrgb =>    (        none,      basic),
            Self::Etc2Rgb8A1Unorm =>      (        none,      basic),
            Self::Etc2Rgb8A1UnormSrgb =>  (        none,      basic),
            Self::Etc2Rgba8Unorm =>       (        none,      basic),
            Self::Etc2Rgba8UnormSrgb =>   (        none,      basic),
            Self::EacR11Unorm =>          (        none,      basic),
            Self::EacR11Snorm =>          (        none,      basic),
            Self::EacRg11Unorm =>         (        none,      basic),
            Self::EacRg11Snorm =>         (        none,      basic),

            Self::Astc { .. } =>          (        none,      basic),
        };

        // Get whether the format is filterable, taking features into account
        let sample_type1 = self.sample_type(None, Some(device_features));
        let is_filterable = sample_type1 == Some(TextureSampleType::Float { filterable: true });

        // Features that enable filtering don't affect blendability
        let sample_type2 = self.sample_type(None, None);
        let is_blendable = sample_type2 == Some(TextureSampleType::Float { filterable: true })
            || device_features.contains(Features::FLOAT32_BLENDABLE)
                && matches!(self, Self::R32Float | Self::Rg32Float | Self::Rgba32Float);

        flags.set(TextureFormatFeatureFlags::FILTERABLE, is_filterable);
        flags.set(TextureFormatFeatureFlags::BLENDABLE, is_blendable);
        flags.set(
            TextureFormatFeatureFlags::STORAGE_ATOMIC,
            allowed_usages.contains(TextureUsages::STORAGE_ATOMIC),
        );

        TextureFormatFeatures {
            allowed_usages,
            flags,
        }
    }

    /// Returns the sample type compatible with this format and aspect.
    ///
    /// Returns `None` only if this is a combined depth-stencil format or a multi-planar format
    /// and `TextureAspect::All` or no `aspect` was provided.
    #[must_use]
    pub fn sample_type(
        &self,
        aspect: Option<TextureAspect>,
        device_features: Option<Features>,
    ) -> Option<TextureSampleType> {
        let float = TextureSampleType::Float { filterable: true };
        let unfilterable_float = TextureSampleType::Float { filterable: false };
        let float32_sample_type = TextureSampleType::Float {
            filterable: device_features
                .unwrap_or(Features::empty())
                .contains(Features::FLOAT32_FILTERABLE),
        };
        let depth = TextureSampleType::Depth;
        let uint = TextureSampleType::Uint;
        let sint = TextureSampleType::Sint;

        match *self {
            Self::R8Unorm
            | Self::R8Snorm
            | Self::Rg8Unorm
            | Self::Rg8Snorm
            | Self::Rgba8Unorm
            | Self::Rgba8UnormSrgb
            | Self::Rgba8Snorm
            | Self::Bgra8Unorm
            | Self::Bgra8UnormSrgb
            | Self::R16Float
            | Self::Rg16Float
            | Self::Rgba16Float
            | Self::Rgb10a2Unorm
            | Self::Rg11b10Ufloat => Some(float),

            Self::R32Float | Self::Rg32Float | Self::Rgba32Float => Some(float32_sample_type),

            Self::R8Uint
            | Self::Rg8Uint
            | Self::Rgba8Uint
            | Self::R16Uint
            | Self::Rg16Uint
            | Self::Rgba16Uint
            | Self::R32Uint
            | Self::R64Uint
            | Self::Rg32Uint
            | Self::Rgba32Uint
            | Self::Rgb10a2Uint => Some(uint),

            Self::R8Sint
            | Self::Rg8Sint
            | Self::Rgba8Sint
            | Self::R16Sint
            | Self::Rg16Sint
            | Self::Rgba16Sint
            | Self::R32Sint
            | Self::Rg32Sint
            | Self::Rgba32Sint => Some(sint),

            Self::Stencil8 => Some(uint),
            Self::Depth16Unorm | Self::Depth24Plus | Self::Depth32Float => Some(depth),
            Self::Depth24PlusStencil8 | Self::Depth32FloatStencil8 => match aspect {
                Some(TextureAspect::DepthOnly) => Some(depth),
                Some(TextureAspect::StencilOnly) => Some(uint),
                _ => None,
            },

            Self::NV12 | Self::P010 => match aspect {
                Some(TextureAspect::Plane0) | Some(TextureAspect::Plane1) => {
                    Some(unfilterable_float)
                }
                _ => None,
            },

            Self::R16Unorm
            | Self::R16Snorm
            | Self::Rg16Unorm
            | Self::Rg16Snorm
            | Self::Rgba16Unorm
            | Self::Rgba16Snorm => Some(float),

            Self::Rgb9e5Ufloat => Some(float),

            Self::Bc1RgbaUnorm
            | Self::Bc1RgbaUnormSrgb
            | Self::Bc2RgbaUnorm
            | Self::Bc2RgbaUnormSrgb
            | Self::Bc3RgbaUnorm
            | Self::Bc3RgbaUnormSrgb
            | Self::Bc4RUnorm
            | Self::Bc4RSnorm
            | Self::Bc5RgUnorm
            | Self::Bc5RgSnorm
            | Self::Bc6hRgbUfloat
            | Self::Bc6hRgbFloat
            | Self::Bc7RgbaUnorm
            | Self::Bc7RgbaUnormSrgb => Some(float),

            Self::Etc2Rgb8Unorm
            | Self::Etc2Rgb8UnormSrgb
            | Self::Etc2Rgb8A1Unorm
            | Self::Etc2Rgb8A1UnormSrgb
            | Self::Etc2Rgba8Unorm
            | Self::Etc2Rgba8UnormSrgb
            | Self::EacR11Unorm
            | Self::EacR11Snorm
            | Self::EacRg11Unorm
            | Self::EacRg11Snorm => Some(float),

            Self::Astc { .. } => Some(float),
        }
    }

    /// The number of bytes one [texel block](https://gpuweb.github.io/gpuweb/#texel-block) occupies during an image copy, if applicable.
    ///
    /// Known as the [texel block copy footprint](https://gpuweb.github.io/gpuweb/#texel-block-copy-footprint).
    ///
    /// Note that for uncompressed formats this is the same as the size of a single texel,
    /// since uncompressed formats have a block size of 1x1.
    ///
    /// Returns `None` if any of the following are true:
    ///  - the format is a combined depth-stencil and no `aspect` was provided
    ///  - the format is a multi-planar format and no `aspect` was provided
    ///  - the format is `Depth24Plus`
    ///  - the format is `Depth24PlusStencil8` and `aspect` is depth.
    #[deprecated(since = "0.19.0", note = "Use `block_copy_size` instead.")]
    #[must_use]
    pub fn block_size(&self, aspect: Option<TextureAspect>) -> Option<u32> {
        self.block_copy_size(aspect)
    }

    /// The number of bytes one [texel block](https://gpuweb.github.io/gpuweb/#texel-block) occupies during an image copy, if applicable.
    ///
    /// Known as the [texel block copy footprint](https://gpuweb.github.io/gpuweb/#texel-block-copy-footprint).
    ///
    /// Note that for uncompressed formats this is the same as the size of a single texel,
    /// since uncompressed formats have a block size of 1x1.
    ///
    /// Returns `None` if any of the following are true:
    ///  - the format is a combined depth-stencil and no `aspect` was provided
    ///  - the format is a multi-planar format and no `aspect` was provided
    ///  - the format is `Depth24Plus`
    ///  - the format is `Depth24PlusStencil8` and `aspect` is depth.
    #[must_use]
    pub fn block_copy_size(&self, aspect: Option<TextureAspect>) -> Option<u32> {
        match *self {
            Self::R8Unorm | Self::R8Snorm | Self::R8Uint | Self::R8Sint => Some(1),

            Self::Rg8Unorm | Self::Rg8Snorm | Self::Rg8Uint | Self::Rg8Sint => Some(2),
            Self::R16Unorm | Self::R16Snorm | Self::R16Uint | Self::R16Sint | Self::R16Float => {
                Some(2)
            }

            Self::Rgba8Unorm
            | Self::Rgba8UnormSrgb
            | Self::Rgba8Snorm
            | Self::Rgba8Uint
            | Self::Rgba8Sint
            | Self::Bgra8Unorm
            | Self::Bgra8UnormSrgb => Some(4),
            Self::Rg16Unorm
            | Self::Rg16Snorm
            | Self::Rg16Uint
            | Self::Rg16Sint
            | Self::Rg16Float => Some(4),
            Self::R32Uint | Self::R32Sint | Self::R32Float => Some(4),
            Self::Rgb9e5Ufloat | Self::Rgb10a2Uint | Self::Rgb10a2Unorm | Self::Rg11b10Ufloat => {
                Some(4)
            }

            Self::Rgba16Unorm
            | Self::Rgba16Snorm
            | Self::Rgba16Uint
            | Self::Rgba16Sint
            | Self::Rgba16Float => Some(8),
            Self::R64Uint | Self::Rg32Uint | Self::Rg32Sint | Self::Rg32Float => Some(8),

            Self::Rgba32Uint | Self::Rgba32Sint | Self::Rgba32Float => Some(16),

            Self::Stencil8 => Some(1),
            Self::Depth16Unorm => Some(2),
            Self::Depth32Float => Some(4),
            Self::Depth24Plus => None,
            Self::Depth24PlusStencil8 => match aspect {
                Some(TextureAspect::DepthOnly) => None,
                Some(TextureAspect::StencilOnly) => Some(1),
                _ => None,
            },
            Self::Depth32FloatStencil8 => match aspect {
                Some(TextureAspect::DepthOnly) => Some(4),
                Some(TextureAspect::StencilOnly) => Some(1),
                _ => None,
            },

            Self::NV12 => match aspect {
                Some(TextureAspect::Plane0) => Some(1),
                Some(TextureAspect::Plane1) => Some(2),
                _ => None,
            },

            Self::P010 => match aspect {
                Some(TextureAspect::Plane0) => Some(2),
                Some(TextureAspect::Plane1) => Some(4),
                _ => None,
            },

            Self::Bc1RgbaUnorm | Self::Bc1RgbaUnormSrgb | Self::Bc4RUnorm | Self::Bc4RSnorm => {
                Some(8)
            }
            Self::Bc2RgbaUnorm
            | Self::Bc2RgbaUnormSrgb
            | Self::Bc3RgbaUnorm
            | Self::Bc3RgbaUnormSrgb
            | Self::Bc5RgUnorm
            | Self::Bc5RgSnorm
            | Self::Bc6hRgbUfloat
            | Self::Bc6hRgbFloat
            | Self::Bc7RgbaUnorm
            | Self::Bc7RgbaUnormSrgb => Some(16),

            Self::Etc2Rgb8Unorm
            | Self::Etc2Rgb8UnormSrgb
            | Self::Etc2Rgb8A1Unorm
            | Self::Etc2Rgb8A1UnormSrgb
            | Self::EacR11Unorm
            | Self::EacR11Snorm => Some(8),
            Self::Etc2Rgba8Unorm
            | Self::Etc2Rgba8UnormSrgb
            | Self::EacRg11Unorm
            | Self::EacRg11Snorm => Some(16),

            Self::Astc { .. } => Some(16),
        }
    }

    /// The largest number that can be returned by [`Self::target_pixel_byte_cost`].
    pub const MAX_TARGET_PIXEL_BYTE_COST: u32 = 16;

    /// The number of bytes occupied per pixel in a color attachment
    /// <https://gpuweb.github.io/gpuweb/#render-target-pixel-byte-cost>
    #[must_use]
    pub fn target_pixel_byte_cost(&self) -> Option<u32> {
        match *self {
            Self::R8Unorm | Self::R8Snorm | Self::R8Uint | Self::R8Sint => Some(1),
            Self::Rg8Unorm
            | Self::Rg8Snorm
            | Self::Rg8Uint
            | Self::Rg8Sint
            | Self::R16Uint
            | Self::R16Sint
            | Self::R16Unorm
            | Self::R16Snorm
            | Self::R16Float => Some(2),
            Self::Rgba8Uint
            | Self::Rgba8Sint
            | Self::Rg16Uint
            | Self::Rg16Sint
            | Self::Rg16Unorm
            | Self::Rg16Snorm
            | Self::Rg16Float
            | Self::R32Uint
            | Self::R32Sint
            | Self::R32Float => Some(4),
            // Despite being 4 bytes per pixel, these are 8 bytes per pixel in the table
            Self::Rgba8Unorm
            | Self::Rgba8UnormSrgb
            | Self::Rgba8Snorm
            | Self::Bgra8Unorm
            | Self::Bgra8UnormSrgb
            // ---
            | Self::Rgba16Uint
            | Self::Rgba16Sint
            | Self::Rgba16Unorm
            | Self::Rgba16Snorm
            | Self::Rgba16Float
            | Self::R64Uint
            | Self::Rg32Uint
            | Self::Rg32Sint
            | Self::Rg32Float
            | Self::Rgb10a2Uint
            | Self::Rgb10a2Unorm
            | Self::Rg11b10Ufloat => Some(8),
            Self::Rgba32Uint | Self::Rgba32Sint | Self::Rgba32Float => Some(16),
            // ⚠️ If you add formats with larger sizes, make sure you change `MAX_TARGET_PIXEL_BYTE_COST`` ⚠️
            Self::Stencil8
            | Self::Depth16Unorm
            | Self::Depth24Plus
            | Self::Depth24PlusStencil8
            | Self::Depth32Float
            | Self::Depth32FloatStencil8
            | Self::NV12
            | Self::P010
            | Self::Rgb9e5Ufloat
            | Self::Bc1RgbaUnorm
            | Self::Bc1RgbaUnormSrgb
            | Self::Bc2RgbaUnorm
            | Self::Bc2RgbaUnormSrgb
            | Self::Bc3RgbaUnorm
            | Self::Bc3RgbaUnormSrgb
            | Self::Bc4RUnorm
            | Self::Bc4RSnorm
            | Self::Bc5RgUnorm
            | Self::Bc5RgSnorm
            | Self::Bc6hRgbUfloat
            | Self::Bc6hRgbFloat
            | Self::Bc7RgbaUnorm
            | Self::Bc7RgbaUnormSrgb
            | Self::Etc2Rgb8Unorm
            | Self::Etc2Rgb8UnormSrgb
            | Self::Etc2Rgb8A1Unorm
            | Self::Etc2Rgb8A1UnormSrgb
            | Self::Etc2Rgba8Unorm
            | Self::Etc2Rgba8UnormSrgb
            | Self::EacR11Unorm
            | Self::EacR11Snorm
            | Self::EacRg11Unorm
            | Self::EacRg11Snorm
            | Self::Astc { .. } => None,
        }
    }

    /// See <https://gpuweb.github.io/gpuweb/#render-target-component-alignment>
    #[must_use]
    pub fn target_component_alignment(&self) -> Option<u32> {
        match *self {
            Self::R8Unorm
            | Self::R8Snorm
            | Self::R8Uint
            | Self::R8Sint
            | Self::Rg8Unorm
            | Self::Rg8Snorm
            | Self::Rg8Uint
            | Self::Rg8Sint
            | Self::Rgba8Unorm
            | Self::Rgba8UnormSrgb
            | Self::Rgba8Snorm
            | Self::Rgba8Uint
            | Self::Rgba8Sint
            | Self::Bgra8Unorm
            | Self::Bgra8UnormSrgb => Some(1),
            Self::R16Uint
            | Self::R16Sint
            | Self::R16Unorm
            | Self::R16Snorm
            | Self::R16Float
            | Self::Rg16Uint
            | Self::Rg16Sint
            | Self::Rg16Unorm
            | Self::Rg16Snorm
            | Self::Rg16Float
            | Self::Rgba16Uint
            | Self::Rgba16Sint
            | Self::Rgba16Unorm
            | Self::Rgba16Snorm
            | Self::Rgba16Float => Some(2),
            Self::R32Uint
            | Self::R32Sint
            | Self::R32Float
            | Self::R64Uint
            | Self::Rg32Uint
            | Self::Rg32Sint
            | Self::Rg32Float
            | Self::Rgba32Uint
            | Self::Rgba32Sint
            | Self::Rgba32Float
            | Self::Rgb10a2Uint
            | Self::Rgb10a2Unorm
            | Self::Rg11b10Ufloat => Some(4),
            Self::Stencil8
            | Self::Depth16Unorm
            | Self::Depth24Plus
            | Self::Depth24PlusStencil8
            | Self::Depth32Float
            | Self::Depth32FloatStencil8
            | Self::NV12
            | Self::P010
            | Self::Rgb9e5Ufloat
            | Self::Bc1RgbaUnorm
            | Self::Bc1RgbaUnormSrgb
            | Self::Bc2RgbaUnorm
            | Self::Bc2RgbaUnormSrgb
            | Self::Bc3RgbaUnorm
            | Self::Bc3RgbaUnormSrgb
            | Self::Bc4RUnorm
            | Self::Bc4RSnorm
            | Self::Bc5RgUnorm
            | Self::Bc5RgSnorm
            | Self::Bc6hRgbUfloat
            | Self::Bc6hRgbFloat
            | Self::Bc7RgbaUnorm
            | Self::Bc7RgbaUnormSrgb
            | Self::Etc2Rgb8Unorm
            | Self::Etc2Rgb8UnormSrgb
            | Self::Etc2Rgb8A1Unorm
            | Self::Etc2Rgb8A1UnormSrgb
            | Self::Etc2Rgba8Unorm
            | Self::Etc2Rgba8UnormSrgb
            | Self::EacR11Unorm
            | Self::EacR11Snorm
            | Self::EacRg11Unorm
            | Self::EacRg11Snorm
            | Self::Astc { .. } => None,
        }
    }

    /// Returns the number of components this format has.
    #[must_use]
    pub fn components(&self) -> u8 {
        self.components_with_aspect(TextureAspect::All)
    }

    /// Returns the number of components this format has taking into account the `aspect`.
    ///
    /// The `aspect` is only relevant for combined depth-stencil formats and multi-planar formats.
    #[must_use]
    pub fn components_with_aspect(&self, aspect: TextureAspect) -> u8 {
        match *self {
            Self::R8Unorm
            | Self::R8Snorm
            | Self::R8Uint
            | Self::R8Sint
            | Self::R16Unorm
            | Self::R16Snorm
            | Self::R16Uint
            | Self::R16Sint
            | Self::R16Float
            | Self::R32Uint
            | Self::R32Sint
            | Self::R32Float
            | Self::R64Uint => 1,

            Self::Rg8Unorm
            | Self::Rg8Snorm
            | Self::Rg8Uint
            | Self::Rg8Sint
            | Self::Rg16Unorm
            | Self::Rg16Snorm
            | Self::Rg16Uint
            | Self::Rg16Sint
            | Self::Rg16Float
            | Self::Rg32Uint
            | Self::Rg32Sint
            | Self::Rg32Float => 2,

            Self::Rgba8Unorm
            | Self::Rgba8UnormSrgb
            | Self::Rgba8Snorm
            | Self::Rgba8Uint
            | Self::Rgba8Sint
            | Self::Bgra8Unorm
            | Self::Bgra8UnormSrgb
            | Self::Rgba16Unorm
            | Self::Rgba16Snorm
            | Self::Rgba16Uint
            | Self::Rgba16Sint
            | Self::Rgba16Float
            | Self::Rgba32Uint
            | Self::Rgba32Sint
            | Self::Rgba32Float => 4,

            Self::Rgb9e5Ufloat | Self::Rg11b10Ufloat => 3,
            Self::Rgb10a2Uint | Self::Rgb10a2Unorm => 4,

            Self::Stencil8 | Self::Depth16Unorm | Self::Depth24Plus | Self::Depth32Float => 1,

            Self::Depth24PlusStencil8 | Self::Depth32FloatStencil8 => match aspect {
                TextureAspect::DepthOnly | TextureAspect::StencilOnly => 1,
                _ => 2,
            },

            Self::NV12 | Self::P010 => match aspect {
                TextureAspect::Plane0 => 1,
                TextureAspect::Plane1 => 2,
                _ => 3,
            },

            Self::Bc4RUnorm | Self::Bc4RSnorm => 1,
            Self::Bc5RgUnorm | Self::Bc5RgSnorm => 2,
            Self::Bc6hRgbUfloat | Self::Bc6hRgbFloat => 3,
            Self::Bc1RgbaUnorm
            | Self::Bc1RgbaUnormSrgb
            | Self::Bc2RgbaUnorm
            | Self::Bc2RgbaUnormSrgb
            | Self::Bc3RgbaUnorm
            | Self::Bc3RgbaUnormSrgb
            | Self::Bc7RgbaUnorm
            | Self::Bc7RgbaUnormSrgb => 4,

            Self::EacR11Unorm | Self::EacR11Snorm => 1,
            Self::EacRg11Unorm | Self::EacRg11Snorm => 2,
            Self::Etc2Rgb8Unorm | Self::Etc2Rgb8UnormSrgb => 3,
            Self::Etc2Rgb8A1Unorm
            | Self::Etc2Rgb8A1UnormSrgb
            | Self::Etc2Rgba8Unorm
            | Self::Etc2Rgba8UnormSrgb => 4,

            Self::Astc { .. } => 4,
        }
    }

    /// Strips the `Srgb` suffix from the given texture format.
    #[must_use]
    pub fn remove_srgb_suffix(&self) -> TextureFormat {
        match *self {
            Self::Rgba8UnormSrgb => Self::Rgba8Unorm,
            Self::Bgra8UnormSrgb => Self::Bgra8Unorm,
            Self::Bc1RgbaUnormSrgb => Self::Bc1RgbaUnorm,
            Self::Bc2RgbaUnormSrgb => Self::Bc2RgbaUnorm,
            Self::Bc3RgbaUnormSrgb => Self::Bc3RgbaUnorm,
            Self::Bc7RgbaUnormSrgb => Self::Bc7RgbaUnorm,
            Self::Etc2Rgb8UnormSrgb => Self::Etc2Rgb8Unorm,
            Self::Etc2Rgb8A1UnormSrgb => Self::Etc2Rgb8A1Unorm,
            Self::Etc2Rgba8UnormSrgb => Self::Etc2Rgba8Unorm,
            Self::Astc {
                block,
                channel: AstcChannel::UnormSrgb,
            } => Self::Astc {
                block,
                channel: AstcChannel::Unorm,
            },
            _ => *self,
        }
    }

    /// Adds an `Srgb` suffix to the given texture format, if the format supports it.
    #[must_use]
    pub fn add_srgb_suffix(&self) -> TextureFormat {
        match *self {
            Self::Rgba8Unorm => Self::Rgba8UnormSrgb,
            Self::Bgra8Unorm => Self::Bgra8UnormSrgb,
            Self::Bc1RgbaUnorm => Self::Bc1RgbaUnormSrgb,
            Self::Bc2RgbaUnorm => Self::Bc2RgbaUnormSrgb,
            Self::Bc3RgbaUnorm => Self::Bc3RgbaUnormSrgb,
            Self::Bc7RgbaUnorm => Self::Bc7RgbaUnormSrgb,
            Self::Etc2Rgb8Unorm => Self::Etc2Rgb8UnormSrgb,
            Self::Etc2Rgb8A1Unorm => Self::Etc2Rgb8A1UnormSrgb,
            Self::Etc2Rgba8Unorm => Self::Etc2Rgba8UnormSrgb,
            Self::Astc {
                block,
                channel: AstcChannel::Unorm,
            } => Self::Astc {
                block,
                channel: AstcChannel::UnormSrgb,
            },
            _ => *self,
        }
    }

    /// Returns `true` for srgb formats.
    #[must_use]
    pub fn is_srgb(&self) -> bool {
        *self != self.remove_srgb_suffix()
    }

    /// Returns the theoretical memory footprint of a texture with the given format and dimensions.
    ///
    /// Actual memory usage may greatly exceed this value due to alignment and padding.
    #[must_use]
    pub fn theoretical_memory_footprint(&self, size: crate::Extent3d) -> u64 {
        let (block_width, block_height) = self.block_dimensions();

        let block_size = self.block_copy_size(None);

        let approximate_block_size = match block_size {
            Some(size) => size,
            None => match self {
                // One f16 per pixel
                Self::Depth16Unorm => 2,
                // One u24 per pixel, padded to 4 bytes
                Self::Depth24Plus => 4,
                // One u24 per pixel, plus one u8 per pixel
                Self::Depth24PlusStencil8 => 4,
                // One f32 per pixel
                Self::Depth32Float => 4,
                // One f32 per pixel, plus one u8 per pixel, with 3 bytes intermediary padding
                Self::Depth32FloatStencil8 => 8,
                // One u8 per pixel
                Self::Stencil8 => 1,
                // Two chroma bytes per block, one luma byte per block
                Self::NV12 => 3,
                // Two chroma u16s and one luma u16 per block
                Self::P010 => 6,
                f => {
                    unimplemented!("Memory footprint for format {f:?} is not implemented");
                }
            },
        };

        let width_blocks = size.width.div_ceil(block_width) as u64;
        let height_blocks = size.height.div_ceil(block_height) as u64;

        let total_blocks = width_blocks * height_blocks * size.depth_or_array_layers as u64;

        total_blocks * approximate_block_size as u64
    }
}

#[cfg(any(feature = "serde", test))]
impl<'de> Deserialize<'de> for TextureFormat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{self, Error, Unexpected};

        struct TextureFormatVisitor;

        impl de::Visitor<'_> for TextureFormatVisitor {
            type Value = TextureFormat;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_str("a valid texture format")
            }

            fn visit_str<E: Error>(self, s: &str) -> Result<Self::Value, E> {
                let format = match s {
                    "r8unorm" => TextureFormat::R8Unorm,
                    "r8snorm" => TextureFormat::R8Snorm,
                    "r8uint" => TextureFormat::R8Uint,
                    "r8sint" => TextureFormat::R8Sint,
                    "r16uint" => TextureFormat::R16Uint,
                    "r16sint" => TextureFormat::R16Sint,
                    "r16unorm" => TextureFormat::R16Unorm,
                    "r16snorm" => TextureFormat::R16Snorm,
                    "r16float" => TextureFormat::R16Float,
                    "rg8unorm" => TextureFormat::Rg8Unorm,
                    "rg8snorm" => TextureFormat::Rg8Snorm,
                    "rg8uint" => TextureFormat::Rg8Uint,
                    "rg8sint" => TextureFormat::Rg8Sint,
                    "r32uint" => TextureFormat::R32Uint,
                    "r32sint" => TextureFormat::R32Sint,
                    "r32float" => TextureFormat::R32Float,
                    "rg16uint" => TextureFormat::Rg16Uint,
                    "rg16sint" => TextureFormat::Rg16Sint,
                    "rg16unorm" => TextureFormat::Rg16Unorm,
                    "rg16snorm" => TextureFormat::Rg16Snorm,
                    "rg16float" => TextureFormat::Rg16Float,
                    "rgba8unorm" => TextureFormat::Rgba8Unorm,
                    "rgba8unorm-srgb" => TextureFormat::Rgba8UnormSrgb,
                    "rgba8snorm" => TextureFormat::Rgba8Snorm,
                    "rgba8uint" => TextureFormat::Rgba8Uint,
                    "rgba8sint" => TextureFormat::Rgba8Sint,
                    "bgra8unorm" => TextureFormat::Bgra8Unorm,
                    "bgra8unorm-srgb" => TextureFormat::Bgra8UnormSrgb,
                    "rgb10a2uint" => TextureFormat::Rgb10a2Uint,
                    "rgb10a2unorm" => TextureFormat::Rgb10a2Unorm,
                    "rg11b10ufloat" => TextureFormat::Rg11b10Ufloat,
                    "r64uint" => TextureFormat::R64Uint,
                    "rg32uint" => TextureFormat::Rg32Uint,
                    "rg32sint" => TextureFormat::Rg32Sint,
                    "rg32float" => TextureFormat::Rg32Float,
                    "rgba16uint" => TextureFormat::Rgba16Uint,
                    "rgba16sint" => TextureFormat::Rgba16Sint,
                    "rgba16unorm" => TextureFormat::Rgba16Unorm,
                    "rgba16snorm" => TextureFormat::Rgba16Snorm,
                    "rgba16float" => TextureFormat::Rgba16Float,
                    "rgba32uint" => TextureFormat::Rgba32Uint,
                    "rgba32sint" => TextureFormat::Rgba32Sint,
                    "rgba32float" => TextureFormat::Rgba32Float,
                    "stencil8" => TextureFormat::Stencil8,
                    "depth32float" => TextureFormat::Depth32Float,
                    "depth32float-stencil8" => TextureFormat::Depth32FloatStencil8,
                    "depth16unorm" => TextureFormat::Depth16Unorm,
                    "depth24plus" => TextureFormat::Depth24Plus,
                    "depth24plus-stencil8" => TextureFormat::Depth24PlusStencil8,
                    "nv12" => TextureFormat::NV12,
                    "p010" => TextureFormat::P010,
                    "rgb9e5ufloat" => TextureFormat::Rgb9e5Ufloat,
                    "bc1-rgba-unorm" => TextureFormat::Bc1RgbaUnorm,
                    "bc1-rgba-unorm-srgb" => TextureFormat::Bc1RgbaUnormSrgb,
                    "bc2-rgba-unorm" => TextureFormat::Bc2RgbaUnorm,
                    "bc2-rgba-unorm-srgb" => TextureFormat::Bc2RgbaUnormSrgb,
                    "bc3-rgba-unorm" => TextureFormat::Bc3RgbaUnorm,
                    "bc3-rgba-unorm-srgb" => TextureFormat::Bc3RgbaUnormSrgb,
                    "bc4-r-unorm" => TextureFormat::Bc4RUnorm,
                    "bc4-r-snorm" => TextureFormat::Bc4RSnorm,
                    "bc5-rg-unorm" => TextureFormat::Bc5RgUnorm,
                    "bc5-rg-snorm" => TextureFormat::Bc5RgSnorm,
                    "bc6h-rgb-ufloat" => TextureFormat::Bc6hRgbUfloat,
                    "bc6h-rgb-float" => TextureFormat::Bc6hRgbFloat,
                    "bc7-rgba-unorm" => TextureFormat::Bc7RgbaUnorm,
                    "bc7-rgba-unorm-srgb" => TextureFormat::Bc7RgbaUnormSrgb,
                    "etc2-rgb8unorm" => TextureFormat::Etc2Rgb8Unorm,
                    "etc2-rgb8unorm-srgb" => TextureFormat::Etc2Rgb8UnormSrgb,
                    "etc2-rgb8a1unorm" => TextureFormat::Etc2Rgb8A1Unorm,
                    "etc2-rgb8a1unorm-srgb" => TextureFormat::Etc2Rgb8A1UnormSrgb,
                    "etc2-rgba8unorm" => TextureFormat::Etc2Rgba8Unorm,
                    "etc2-rgba8unorm-srgb" => TextureFormat::Etc2Rgba8UnormSrgb,
                    "eac-r11unorm" => TextureFormat::EacR11Unorm,
                    "eac-r11snorm" => TextureFormat::EacR11Snorm,
                    "eac-rg11unorm" => TextureFormat::EacRg11Unorm,
                    "eac-rg11snorm" => TextureFormat::EacRg11Snorm,
                    other => {
                        if let Some(parts) = other.strip_prefix("astc-") {
                            let (block, channel) = parts
                                .split_once('-')
                                .ok_or_else(|| E::invalid_value(Unexpected::Str(s), &self))?;

                            let block = match block {
                                "4x4" => AstcBlock::B4x4,
                                "5x4" => AstcBlock::B5x4,
                                "5x5" => AstcBlock::B5x5,
                                "6x5" => AstcBlock::B6x5,
                                "6x6" => AstcBlock::B6x6,
                                "8x5" => AstcBlock::B8x5,
                                "8x6" => AstcBlock::B8x6,
                                "8x8" => AstcBlock::B8x8,
                                "10x5" => AstcBlock::B10x5,
                                "10x6" => AstcBlock::B10x6,
                                "10x8" => AstcBlock::B10x8,
                                "10x10" => AstcBlock::B10x10,
                                "12x10" => AstcBlock::B12x10,
                                "12x12" => AstcBlock::B12x12,
                                _ => return Err(E::invalid_value(Unexpected::Str(s), &self)),
                            };

                            let channel = match channel {
                                "unorm" => AstcChannel::Unorm,
                                "unorm-srgb" => AstcChannel::UnormSrgb,
                                "hdr" => AstcChannel::Hdr,
                                _ => return Err(E::invalid_value(Unexpected::Str(s), &self)),
                            };

                            TextureFormat::Astc { block, channel }
                        } else {
                            return Err(E::invalid_value(Unexpected::Str(s), &self));
                        }
                    }
                };

                Ok(format)
            }
        }

        deserializer.deserialize_str(TextureFormatVisitor)
    }
}

#[cfg(any(feature = "serde", test))]
impl Serialize for TextureFormat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s: alloc::string::String;
        let name = match *self {
            TextureFormat::R8Unorm => "r8unorm",
            TextureFormat::R8Snorm => "r8snorm",
            TextureFormat::R8Uint => "r8uint",
            TextureFormat::R8Sint => "r8sint",
            TextureFormat::R16Uint => "r16uint",
            TextureFormat::R16Sint => "r16sint",
            TextureFormat::R16Unorm => "r16unorm",
            TextureFormat::R16Snorm => "r16snorm",
            TextureFormat::R16Float => "r16float",
            TextureFormat::Rg8Unorm => "rg8unorm",
            TextureFormat::Rg8Snorm => "rg8snorm",
            TextureFormat::Rg8Uint => "rg8uint",
            TextureFormat::Rg8Sint => "rg8sint",
            TextureFormat::R32Uint => "r32uint",
            TextureFormat::R32Sint => "r32sint",
            TextureFormat::R32Float => "r32float",
            TextureFormat::Rg16Uint => "rg16uint",
            TextureFormat::Rg16Sint => "rg16sint",
            TextureFormat::Rg16Unorm => "rg16unorm",
            TextureFormat::Rg16Snorm => "rg16snorm",
            TextureFormat::Rg16Float => "rg16float",
            TextureFormat::Rgba8Unorm => "rgba8unorm",
            TextureFormat::Rgba8UnormSrgb => "rgba8unorm-srgb",
            TextureFormat::Rgba8Snorm => "rgba8snorm",
            TextureFormat::Rgba8Uint => "rgba8uint",
            TextureFormat::Rgba8Sint => "rgba8sint",
            TextureFormat::Bgra8Unorm => "bgra8unorm",
            TextureFormat::Bgra8UnormSrgb => "bgra8unorm-srgb",
            TextureFormat::Rgb10a2Uint => "rgb10a2uint",
            TextureFormat::Rgb10a2Unorm => "rgb10a2unorm",
            TextureFormat::Rg11b10Ufloat => "rg11b10ufloat",
            TextureFormat::R64Uint => "r64uint",
            TextureFormat::Rg32Uint => "rg32uint",
            TextureFormat::Rg32Sint => "rg32sint",
            TextureFormat::Rg32Float => "rg32float",
            TextureFormat::Rgba16Uint => "rgba16uint",
            TextureFormat::Rgba16Sint => "rgba16sint",
            TextureFormat::Rgba16Unorm => "rgba16unorm",
            TextureFormat::Rgba16Snorm => "rgba16snorm",
            TextureFormat::Rgba16Float => "rgba16float",
            TextureFormat::Rgba32Uint => "rgba32uint",
            TextureFormat::Rgba32Sint => "rgba32sint",
            TextureFormat::Rgba32Float => "rgba32float",
            TextureFormat::Stencil8 => "stencil8",
            TextureFormat::Depth32Float => "depth32float",
            TextureFormat::Depth16Unorm => "depth16unorm",
            TextureFormat::Depth32FloatStencil8 => "depth32float-stencil8",
            TextureFormat::Depth24Plus => "depth24plus",
            TextureFormat::Depth24PlusStencil8 => "depth24plus-stencil8",
            TextureFormat::NV12 => "nv12",
            TextureFormat::P010 => "p010",
            TextureFormat::Rgb9e5Ufloat => "rgb9e5ufloat",
            TextureFormat::Bc1RgbaUnorm => "bc1-rgba-unorm",
            TextureFormat::Bc1RgbaUnormSrgb => "bc1-rgba-unorm-srgb",
            TextureFormat::Bc2RgbaUnorm => "bc2-rgba-unorm",
            TextureFormat::Bc2RgbaUnormSrgb => "bc2-rgba-unorm-srgb",
            TextureFormat::Bc3RgbaUnorm => "bc3-rgba-unorm",
            TextureFormat::Bc3RgbaUnormSrgb => "bc3-rgba-unorm-srgb",
            TextureFormat::Bc4RUnorm => "bc4-r-unorm",
            TextureFormat::Bc4RSnorm => "bc4-r-snorm",
            TextureFormat::Bc5RgUnorm => "bc5-rg-unorm",
            TextureFormat::Bc5RgSnorm => "bc5-rg-snorm",
            TextureFormat::Bc6hRgbUfloat => "bc6h-rgb-ufloat",
            TextureFormat::Bc6hRgbFloat => "bc6h-rgb-float",
            TextureFormat::Bc7RgbaUnorm => "bc7-rgba-unorm",
            TextureFormat::Bc7RgbaUnormSrgb => "bc7-rgba-unorm-srgb",
            TextureFormat::Etc2Rgb8Unorm => "etc2-rgb8unorm",
            TextureFormat::Etc2Rgb8UnormSrgb => "etc2-rgb8unorm-srgb",
            TextureFormat::Etc2Rgb8A1Unorm => "etc2-rgb8a1unorm",
            TextureFormat::Etc2Rgb8A1UnormSrgb => "etc2-rgb8a1unorm-srgb",
            TextureFormat::Etc2Rgba8Unorm => "etc2-rgba8unorm",
            TextureFormat::Etc2Rgba8UnormSrgb => "etc2-rgba8unorm-srgb",
            TextureFormat::EacR11Unorm => "eac-r11unorm",
            TextureFormat::EacR11Snorm => "eac-r11snorm",
            TextureFormat::EacRg11Unorm => "eac-rg11unorm",
            TextureFormat::EacRg11Snorm => "eac-rg11snorm",
            TextureFormat::Astc { block, channel } => {
                let block = match block {
                    AstcBlock::B4x4 => "4x4",
                    AstcBlock::B5x4 => "5x4",
                    AstcBlock::B5x5 => "5x5",
                    AstcBlock::B6x5 => "6x5",
                    AstcBlock::B6x6 => "6x6",
                    AstcBlock::B8x5 => "8x5",
                    AstcBlock::B8x6 => "8x6",
                    AstcBlock::B8x8 => "8x8",
                    AstcBlock::B10x5 => "10x5",
                    AstcBlock::B10x6 => "10x6",
                    AstcBlock::B10x8 => "10x8",
                    AstcBlock::B10x10 => "10x10",
                    AstcBlock::B12x10 => "12x10",
                    AstcBlock::B12x12 => "12x12",
                };

                let channel = match channel {
                    AstcChannel::Unorm => "unorm",
                    AstcChannel::UnormSrgb => "unorm-srgb",
                    AstcChannel::Hdr => "hdr",
                };

                s = alloc::format!("astc-{block}-{channel}");
                &s
            }
        };
        serializer.serialize_str(name)
    }
}

bitflags::bitflags! {
    /// Feature flags for a texture format.
    #[repr(transparent)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    #[cfg_attr(feature = "serde", serde(transparent))]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct TextureFormatFeatureFlags: u32 {
        /// If not present, the texture can't be sampled with a filtering sampler.
        /// This may overwrite TextureSampleType::Float.filterable
        const FILTERABLE = 1 << 0;
        /// Allows [`TextureDescriptor::sample_count`] to be `2`.
        const MULTISAMPLE_X2 = 1 << 1;
        /// Allows [`TextureDescriptor::sample_count`] to be `4`.
        const MULTISAMPLE_X4 = 1 << 2 ;
        /// Allows [`TextureDescriptor::sample_count`] to be `8`.
        const MULTISAMPLE_X8 = 1 << 3 ;
        /// Allows [`TextureDescriptor::sample_count`] to be `16`.
        const MULTISAMPLE_X16 = 1 << 4;
        /// Allows a texture of this format to back a view passed as `resolve_target`
        /// to a render pass for an automatic driver-implemented resolve.
        const MULTISAMPLE_RESOLVE = 1 << 5;
        /// When used as a STORAGE texture, then a texture with this format can be bound with
        /// [`StorageTextureAccess::ReadOnly`].
        const STORAGE_READ_ONLY = 1 << 6;
        /// When used as a STORAGE texture, then a texture with this format can be bound with
        /// [`StorageTextureAccess::WriteOnly`].
        const STORAGE_WRITE_ONLY = 1 << 7;
        /// When used as a STORAGE texture, then a texture with this format can be bound with
        /// [`StorageTextureAccess::ReadWrite`].
        const STORAGE_READ_WRITE = 1 << 8;
        /// When used as a STORAGE texture, then a texture with this format can be bound with
        /// [`StorageTextureAccess::Atomic`].
        const STORAGE_ATOMIC = 1 << 9;
        /// If not present, the texture can't be blended into the render target.
        const BLENDABLE = 1 << 10;
    }
}

impl TextureFormatFeatureFlags {
    /// Sample count supported by a given texture format.
    ///
    /// returns `true` if `count` is a supported sample count.
    #[must_use]
    pub fn sample_count_supported(&self, count: u32) -> bool {
        use TextureFormatFeatureFlags as tfsc;

        match count {
            1 => true,
            2 => self.contains(tfsc::MULTISAMPLE_X2),
            4 => self.contains(tfsc::MULTISAMPLE_X4),
            8 => self.contains(tfsc::MULTISAMPLE_X8),
            16 => self.contains(tfsc::MULTISAMPLE_X16),
            _ => false,
        }
    }

    /// A `Vec` of supported sample counts.
    #[must_use]
    pub fn supported_sample_counts(&self) -> Vec<u32> {
        let all_possible_sample_counts: [u32; 5] = [1, 2, 4, 8, 16];
        all_possible_sample_counts
            .into_iter()
            .filter(|&sc| self.sample_count_supported(sc))
            .collect()
    }
}

/// Features supported by a given texture format
///
/// Features are defined by WebGPU specification unless [`Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES`] is enabled.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextureFormatFeatures {
    /// Valid bits for `TextureDescriptor::Usage` provided for format creation.
    pub allowed_usages: TextureUsages,
    /// Additional property flags for the format.
    pub flags: TextureFormatFeatureFlags,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn texture_format_serialize() {
        use alloc::string::ToString;

        assert_eq!(
            serde_json::to_string(&TextureFormat::R8Unorm).unwrap(),
            "\"r8unorm\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::R8Snorm).unwrap(),
            "\"r8snorm\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::R8Uint).unwrap(),
            "\"r8uint\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::R8Sint).unwrap(),
            "\"r8sint\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::R16Uint).unwrap(),
            "\"r16uint\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::R16Sint).unwrap(),
            "\"r16sint\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::R16Unorm).unwrap(),
            "\"r16unorm\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::R16Snorm).unwrap(),
            "\"r16snorm\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::R16Float).unwrap(),
            "\"r16float\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::Rg8Unorm).unwrap(),
            "\"rg8unorm\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::Rg8Snorm).unwrap(),
            "\"rg8snorm\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::Rg8Uint).unwrap(),
            "\"rg8uint\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::Rg8Sint).unwrap(),
            "\"rg8sint\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::R32Uint).unwrap(),
            "\"r32uint\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::R32Sint).unwrap(),
            "\"r32sint\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::R32Float).unwrap(),
            "\"r32float\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::Rg16Uint).unwrap(),
            "\"rg16uint\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::Rg16Sint).unwrap(),
            "\"rg16sint\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::Rg16Unorm).unwrap(),
            "\"rg16unorm\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::Rg16Snorm).unwrap(),
            "\"rg16snorm\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::Rg16Float).unwrap(),
            "\"rg16float\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::Rgba8Unorm).unwrap(),
            "\"rgba8unorm\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::Rgba8UnormSrgb).unwrap(),
            "\"rgba8unorm-srgb\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::Rgba8Snorm).unwrap(),
            "\"rgba8snorm\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::Rgba8Uint).unwrap(),
            "\"rgba8uint\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::Rgba8Sint).unwrap(),
            "\"rgba8sint\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::Bgra8Unorm).unwrap(),
            "\"bgra8unorm\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::Bgra8UnormSrgb).unwrap(),
            "\"bgra8unorm-srgb\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::Rgb10a2Uint).unwrap(),
            "\"rgb10a2uint\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::Rgb10a2Unorm).unwrap(),
            "\"rgb10a2unorm\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::Rg11b10Ufloat).unwrap(),
            "\"rg11b10ufloat\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::R64Uint).unwrap(),
            "\"r64uint\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::Rg32Uint).unwrap(),
            "\"rg32uint\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::Rg32Sint).unwrap(),
            "\"rg32sint\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::Rg32Float).unwrap(),
            "\"rg32float\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::Rgba16Uint).unwrap(),
            "\"rgba16uint\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::Rgba16Sint).unwrap(),
            "\"rgba16sint\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::Rgba16Unorm).unwrap(),
            "\"rgba16unorm\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::Rgba16Snorm).unwrap(),
            "\"rgba16snorm\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::Rgba16Float).unwrap(),
            "\"rgba16float\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::Rgba32Uint).unwrap(),
            "\"rgba32uint\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::Rgba32Sint).unwrap(),
            "\"rgba32sint\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::Rgba32Float).unwrap(),
            "\"rgba32float\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::Stencil8).unwrap(),
            "\"stencil8\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::Depth32Float).unwrap(),
            "\"depth32float\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::Depth16Unorm).unwrap(),
            "\"depth16unorm\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::Depth32FloatStencil8).unwrap(),
            "\"depth32float-stencil8\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::Depth24Plus).unwrap(),
            "\"depth24plus\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::Depth24PlusStencil8).unwrap(),
            "\"depth24plus-stencil8\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::Rgb9e5Ufloat).unwrap(),
            "\"rgb9e5ufloat\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::Bc1RgbaUnorm).unwrap(),
            "\"bc1-rgba-unorm\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::Bc1RgbaUnormSrgb).unwrap(),
            "\"bc1-rgba-unorm-srgb\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::Bc2RgbaUnorm).unwrap(),
            "\"bc2-rgba-unorm\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::Bc2RgbaUnormSrgb).unwrap(),
            "\"bc2-rgba-unorm-srgb\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::Bc3RgbaUnorm).unwrap(),
            "\"bc3-rgba-unorm\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::Bc3RgbaUnormSrgb).unwrap(),
            "\"bc3-rgba-unorm-srgb\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::Bc4RUnorm).unwrap(),
            "\"bc4-r-unorm\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::Bc4RSnorm).unwrap(),
            "\"bc4-r-snorm\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::Bc5RgUnorm).unwrap(),
            "\"bc5-rg-unorm\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::Bc5RgSnorm).unwrap(),
            "\"bc5-rg-snorm\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::Bc6hRgbUfloat).unwrap(),
            "\"bc6h-rgb-ufloat\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::Bc6hRgbFloat).unwrap(),
            "\"bc6h-rgb-float\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::Bc7RgbaUnorm).unwrap(),
            "\"bc7-rgba-unorm\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::Bc7RgbaUnormSrgb).unwrap(),
            "\"bc7-rgba-unorm-srgb\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::Etc2Rgb8Unorm).unwrap(),
            "\"etc2-rgb8unorm\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::Etc2Rgb8UnormSrgb).unwrap(),
            "\"etc2-rgb8unorm-srgb\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::Etc2Rgb8A1Unorm).unwrap(),
            "\"etc2-rgb8a1unorm\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::Etc2Rgb8A1UnormSrgb).unwrap(),
            "\"etc2-rgb8a1unorm-srgb\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::Etc2Rgba8Unorm).unwrap(),
            "\"etc2-rgba8unorm\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::Etc2Rgba8UnormSrgb).unwrap(),
            "\"etc2-rgba8unorm-srgb\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::EacR11Unorm).unwrap(),
            "\"eac-r11unorm\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::EacR11Snorm).unwrap(),
            "\"eac-r11snorm\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::EacRg11Unorm).unwrap(),
            "\"eac-rg11unorm\"".to_string()
        );
        assert_eq!(
            serde_json::to_string(&TextureFormat::EacRg11Snorm).unwrap(),
            "\"eac-rg11snorm\"".to_string()
        );
    }

    #[test]
    fn texture_format_deserialize() {
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"r8unorm\"").unwrap(),
            TextureFormat::R8Unorm
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"r8snorm\"").unwrap(),
            TextureFormat::R8Snorm
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"r8uint\"").unwrap(),
            TextureFormat::R8Uint
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"r8sint\"").unwrap(),
            TextureFormat::R8Sint
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"r16uint\"").unwrap(),
            TextureFormat::R16Uint
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"r16sint\"").unwrap(),
            TextureFormat::R16Sint
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"r16unorm\"").unwrap(),
            TextureFormat::R16Unorm
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"r16snorm\"").unwrap(),
            TextureFormat::R16Snorm
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"r16float\"").unwrap(),
            TextureFormat::R16Float
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"rg8unorm\"").unwrap(),
            TextureFormat::Rg8Unorm
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"rg8snorm\"").unwrap(),
            TextureFormat::Rg8Snorm
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"rg8uint\"").unwrap(),
            TextureFormat::Rg8Uint
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"rg8sint\"").unwrap(),
            TextureFormat::Rg8Sint
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"r32uint\"").unwrap(),
            TextureFormat::R32Uint
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"r32sint\"").unwrap(),
            TextureFormat::R32Sint
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"r32float\"").unwrap(),
            TextureFormat::R32Float
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"rg16uint\"").unwrap(),
            TextureFormat::Rg16Uint
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"rg16sint\"").unwrap(),
            TextureFormat::Rg16Sint
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"rg16unorm\"").unwrap(),
            TextureFormat::Rg16Unorm
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"rg16snorm\"").unwrap(),
            TextureFormat::Rg16Snorm
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"rg16float\"").unwrap(),
            TextureFormat::Rg16Float
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"rgba8unorm\"").unwrap(),
            TextureFormat::Rgba8Unorm
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"rgba8unorm-srgb\"").unwrap(),
            TextureFormat::Rgba8UnormSrgb
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"rgba8snorm\"").unwrap(),
            TextureFormat::Rgba8Snorm
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"rgba8uint\"").unwrap(),
            TextureFormat::Rgba8Uint
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"rgba8sint\"").unwrap(),
            TextureFormat::Rgba8Sint
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"bgra8unorm\"").unwrap(),
            TextureFormat::Bgra8Unorm
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"bgra8unorm-srgb\"").unwrap(),
            TextureFormat::Bgra8UnormSrgb
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"rgb10a2uint\"").unwrap(),
            TextureFormat::Rgb10a2Uint
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"rgb10a2unorm\"").unwrap(),
            TextureFormat::Rgb10a2Unorm
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"rg11b10ufloat\"").unwrap(),
            TextureFormat::Rg11b10Ufloat
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"r64uint\"").unwrap(),
            TextureFormat::R64Uint
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"rg32uint\"").unwrap(),
            TextureFormat::Rg32Uint
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"rg32sint\"").unwrap(),
            TextureFormat::Rg32Sint
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"rg32float\"").unwrap(),
            TextureFormat::Rg32Float
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"rgba16uint\"").unwrap(),
            TextureFormat::Rgba16Uint
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"rgba16sint\"").unwrap(),
            TextureFormat::Rgba16Sint
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"rgba16unorm\"").unwrap(),
            TextureFormat::Rgba16Unorm
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"rgba16snorm\"").unwrap(),
            TextureFormat::Rgba16Snorm
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"rgba16float\"").unwrap(),
            TextureFormat::Rgba16Float
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"rgba32uint\"").unwrap(),
            TextureFormat::Rgba32Uint
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"rgba32sint\"").unwrap(),
            TextureFormat::Rgba32Sint
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"rgba32float\"").unwrap(),
            TextureFormat::Rgba32Float
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"stencil8\"").unwrap(),
            TextureFormat::Stencil8
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"depth32float\"").unwrap(),
            TextureFormat::Depth32Float
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"depth16unorm\"").unwrap(),
            TextureFormat::Depth16Unorm
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"depth32float-stencil8\"").unwrap(),
            TextureFormat::Depth32FloatStencil8
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"depth24plus\"").unwrap(),
            TextureFormat::Depth24Plus
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"depth24plus-stencil8\"").unwrap(),
            TextureFormat::Depth24PlusStencil8
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"rgb9e5ufloat\"").unwrap(),
            TextureFormat::Rgb9e5Ufloat
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"bc1-rgba-unorm\"").unwrap(),
            TextureFormat::Bc1RgbaUnorm
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"bc1-rgba-unorm-srgb\"").unwrap(),
            TextureFormat::Bc1RgbaUnormSrgb
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"bc2-rgba-unorm\"").unwrap(),
            TextureFormat::Bc2RgbaUnorm
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"bc2-rgba-unorm-srgb\"").unwrap(),
            TextureFormat::Bc2RgbaUnormSrgb
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"bc3-rgba-unorm\"").unwrap(),
            TextureFormat::Bc3RgbaUnorm
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"bc3-rgba-unorm-srgb\"").unwrap(),
            TextureFormat::Bc3RgbaUnormSrgb
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"bc4-r-unorm\"").unwrap(),
            TextureFormat::Bc4RUnorm
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"bc4-r-snorm\"").unwrap(),
            TextureFormat::Bc4RSnorm
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"bc5-rg-unorm\"").unwrap(),
            TextureFormat::Bc5RgUnorm
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"bc5-rg-snorm\"").unwrap(),
            TextureFormat::Bc5RgSnorm
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"bc6h-rgb-ufloat\"").unwrap(),
            TextureFormat::Bc6hRgbUfloat
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"bc6h-rgb-float\"").unwrap(),
            TextureFormat::Bc6hRgbFloat
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"bc7-rgba-unorm\"").unwrap(),
            TextureFormat::Bc7RgbaUnorm
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"bc7-rgba-unorm-srgb\"").unwrap(),
            TextureFormat::Bc7RgbaUnormSrgb
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"etc2-rgb8unorm\"").unwrap(),
            TextureFormat::Etc2Rgb8Unorm
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"etc2-rgb8unorm-srgb\"").unwrap(),
            TextureFormat::Etc2Rgb8UnormSrgb
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"etc2-rgb8a1unorm\"").unwrap(),
            TextureFormat::Etc2Rgb8A1Unorm
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"etc2-rgb8a1unorm-srgb\"").unwrap(),
            TextureFormat::Etc2Rgb8A1UnormSrgb
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"etc2-rgba8unorm\"").unwrap(),
            TextureFormat::Etc2Rgba8Unorm
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"etc2-rgba8unorm-srgb\"").unwrap(),
            TextureFormat::Etc2Rgba8UnormSrgb
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"eac-r11unorm\"").unwrap(),
            TextureFormat::EacR11Unorm
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"eac-r11snorm\"").unwrap(),
            TextureFormat::EacR11Snorm
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"eac-rg11unorm\"").unwrap(),
            TextureFormat::EacRg11Unorm
        );
        assert_eq!(
            serde_json::from_str::<TextureFormat>("\"eac-rg11snorm\"").unwrap(),
            TextureFormat::EacRg11Snorm
        );
    }

    /// test if `TextureFormat::is_depth_stencil_format` is behaving correctly
    #[test]
    fn is_depth_stencil_format() {
        let depth_stencil_formats = [
            TextureFormat::Stencil8,
            TextureFormat::Depth16Unorm,
            TextureFormat::Depth24Plus,
            TextureFormat::Depth24PlusStencil8,
            TextureFormat::Depth32Float,
            TextureFormat::Depth32FloatStencil8,
        ];

        for format in depth_stencil_formats {
            assert!(format.is_depth_stencil_format());
        }

        // some non-depth-stencil-formats shouldn't be accepted
        let non_depth_stencil_formats = [
            TextureFormat::R8Unorm,
            TextureFormat::Rgba8Unorm,
            TextureFormat::Rg16Uint,
            TextureFormat::NV12,
        ];

        for format in non_depth_stencil_formats {
            assert!(!format.is_depth_stencil_format());
        }
    }

    /// test if `TextureFormat::is_combined_depth_stencil_format` is behaving correctly
    #[test]
    fn is_combined_depth_stencil_format() {
        let valid_formats = [
            TextureFormat::Depth24PlusStencil8,
            TextureFormat::Depth32FloatStencil8,
        ];

        for format in valid_formats {
            assert!(format.is_combined_depth_stencil_format());
        }

        let some_invalid_formats = [
            TextureFormat::Depth16Unorm,
            TextureFormat::Depth24Plus,
            TextureFormat::Stencil8,
            TextureFormat::Rgba8UnormSrgb,
            TextureFormat::Rgba8Unorm,
        ];

        for format in some_invalid_formats {
            assert!(!format.is_combined_depth_stencil_format());
        }
    }

    /// test if `TextureFormat::has_color_aspect` is behaving correctly
    #[test]
    fn has_color_aspect() {
        let some_valid_formats = [
            TextureFormat::R8Unorm,
            TextureFormat::Rg8Unorm,
            TextureFormat::Bc7RgbaUnorm,
            TextureFormat::Rgba8Unorm,
        ];

        for format in some_valid_formats {
            assert!(format.has_color_aspect());
        }

        let some_invalid_formats = [
            TextureFormat::NV12,
            TextureFormat::Stencil8,
            TextureFormat::Depth24PlusStencil8,
        ];

        for format in some_invalid_formats {
            assert!(!format.has_color_aspect());
        }
    }

    /// test if `TextureFormat::has_depth_aspect` is behaving correctly
    #[test]
    fn has_depth_aspect() {
        let valid_formats = [
            TextureFormat::Depth16Unorm,
            TextureFormat::Depth24Plus,
            TextureFormat::Depth24PlusStencil8,
            TextureFormat::Depth32Float,
            TextureFormat::Depth32FloatStencil8,
        ];

        for format in valid_formats {
            assert!(format.has_depth_aspect());
        }

        let some_invalid_formats = [
            TextureFormat::Rgba8Unorm,
            TextureFormat::Bc7RgbaUnorm,
            TextureFormat::Stencil8,
        ];

        for format in some_invalid_formats {
            assert!(!format.has_depth_aspect());
        }
    }

    /// test if `TextureFormat::has_stencil_aspect` is behaving correctly
    #[test]
    fn has_stencil_aspect() {
        let valid_formats = [
            TextureFormat::Stencil8,
            TextureFormat::Depth24PlusStencil8,
            TextureFormat::Depth32FloatStencil8,
        ];

        for format in valid_formats {
            assert!(format.has_stencil_aspect());
        }

        let some_invalid_formats = [
            TextureFormat::R8Unorm,
            TextureFormat::Depth16Unorm,
            TextureFormat::NV12,
        ];

        for format in some_invalid_formats {
            assert!(!format.has_stencil_aspect());
        }
    }
}
