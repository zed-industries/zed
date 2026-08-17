#[cfg(any(feature = "serde", test))]
use serde::{Deserialize, Serialize};

use crate::TextureDimension;

/// Origin of a copy from a 2D image.
///
/// Corresponds to [WebGPU `GPUOrigin2D`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpuorigin2ddict).
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Origin2d {
    #[allow(missing_docs)]
    pub x: u32,
    #[allow(missing_docs)]
    pub y: u32,
}

impl Origin2d {
    /// Zero origin.
    pub const ZERO: Self = Self { x: 0, y: 0 };

    /// Adds the third dimension to this origin
    #[must_use]
    pub fn to_3d(self, z: u32) -> Origin3d {
        Origin3d {
            x: self.x,
            y: self.y,
            z,
        }
    }
}

impl core::fmt::Debug for Origin2d {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        (self.x, self.y).fmt(f)
    }
}

/// Origin of a copy to/from a texture.
///
/// Corresponds to [WebGPU `GPUOrigin3D`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpuorigin3ddict).
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Origin3d {
    /// X position of the origin
    pub x: u32,
    /// Y position of the origin
    pub y: u32,
    /// Z position of the origin
    pub z: u32,
}

impl Origin3d {
    /// Zero origin.
    pub const ZERO: Self = Self { x: 0, y: 0, z: 0 };

    /// Removes the third dimension from this origin
    #[must_use]
    pub fn to_2d(self) -> Origin2d {
        Origin2d {
            x: self.x,
            y: self.y,
        }
    }
}

impl Default for Origin3d {
    fn default() -> Self {
        Self::ZERO
    }
}

impl core::fmt::Debug for Origin3d {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        (self.x, self.y, self.z).fmt(f)
    }
}

/// Extent of a texture related operation.
///
/// Corresponds to [WebGPU `GPUExtent3D`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpuextent3ddict).
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Extent3d {
    /// Width of the extent
    pub width: u32,
    /// Height of the extent
    pub height: u32,
    /// The depth of the extent or the number of array layers
    #[cfg_attr(feature = "serde", serde(default = "default_depth"))]
    pub depth_or_array_layers: u32,
}

impl core::fmt::Debug for Extent3d {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        (self.width, self.height, self.depth_or_array_layers).fmt(f)
    }
}

#[cfg(feature = "serde")]
fn default_depth() -> u32 {
    1
}

impl Default for Extent3d {
    fn default() -> Self {
        Self {
            width: 1,
            height: 1,
            depth_or_array_layers: 1,
        }
    }
}

impl Extent3d {
    /// Calculates the [physical size] backing a texture of the given
    /// format and extent.  This includes padding to the block width
    /// and height of the format.
    ///
    /// This is the texture extent that you must upload at when uploading to _mipmaps_ of compressed textures.
    ///
    /// [physical size]: https://gpuweb.github.io/gpuweb/#physical-miplevel-specific-texture-extent
    #[must_use]
    pub fn physical_size(&self, format: crate::TextureFormat) -> Self {
        let (block_width, block_height) = format.block_dimensions();

        let width = self.width.div_ceil(block_width) * block_width;
        let height = self.height.div_ceil(block_height) * block_height;

        Self {
            width,
            height,
            depth_or_array_layers: self.depth_or_array_layers,
        }
    }

    /// Calculates the maximum possible count of mipmaps.
    ///
    /// Treats the depth as part of the mipmaps. If calculating
    /// for a 2DArray texture, which does not mipmap depth, set depth to 1.
    #[must_use]
    pub fn max_mips(&self, dim: TextureDimension) -> u32 {
        match dim {
            TextureDimension::D1 => 1,
            TextureDimension::D2 => {
                let max_dim = self.width.max(self.height);
                32 - max_dim.leading_zeros()
            }
            TextureDimension::D3 => {
                let max_dim = self.width.max(self.height.max(self.depth_or_array_layers));
                32 - max_dim.leading_zeros()
            }
        }
    }

    /// Calculates the extent at a given mip level.
    ///
    /// This is a low-level helper for internal use.
    ///
    /// It does *not* account for memory size being a multiple of block size.
    ///
    /// TODO(<https://github.com/gfx-rs/wgpu/issues/8491>): It also does not
    /// consider whether an even dimension is required due to chroma
    /// subsampling, but it probably should.
    ///
    /// <https://gpuweb.github.io/gpuweb/#logical-miplevel-specific-texture-extent>
    #[doc(hidden)]
    #[must_use]
    pub fn mip_level_size(&self, level: u32, dim: TextureDimension) -> Self {
        Self {
            width: u32::max(1, self.width >> level),
            height: match dim {
                TextureDimension::D1 => 1,
                _ => u32::max(1, self.height >> level),
            },
            depth_or_array_layers: match dim {
                TextureDimension::D1 => 1,
                TextureDimension::D2 => self.depth_or_array_layers,
                TextureDimension::D3 => u32::max(1, self.depth_or_array_layers >> level),
            },
        }
    }
}
