//! Identifiers for tiles in the sprite atlas.

use gpui_types::{Bounds, DevicePixels};

/// A tile within a sprite atlas texture.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct AtlasTile {
    /// The texture this tile belongs to.
    pub texture_id: AtlasTextureId,
    /// The unique ID of this tile within its texture.
    pub tile_id: TileId,
    /// Padding around the tile content in pixels.
    pub padding: u32,
    /// The bounds of this tile within the texture.
    pub bounds: Bounds<DevicePixels>,
}

/// Identifies a texture within the sprite atlas.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct AtlasTextureId {
    // We use u32 instead of usize for Metal Shader Language compatibility
    /// The index of this texture in the atlas.
    pub index: u32,
    /// The kind of content stored in this texture.
    pub kind: AtlasTextureKind,
}

/// The kind of content stored in an atlas texture.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
#[cfg_attr(
    all(
        any(target_os = "linux", target_os = "freebsd"),
        not(any(feature = "x11", feature = "wayland"))
    ),
    allow(dead_code)
)]
pub enum AtlasTextureKind {
    /// Single-channel coverage.
    Monochrome = 0,
    /// Full-color pixels.
    Polychrome = 1,
    /// Subpixel-antialiased coverage.
    Subpixel = 2,
}

/// The unique ID of a tile within its texture.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct TileId(pub u32);

impl From<etagere::AllocId> for TileId {
    fn from(id: etagere::AllocId) -> Self {
        Self(id.serialize())
    }
}

impl From<TileId> for etagere::AllocId {
    fn from(id: TileId) -> Self {
        Self::deserialize(id.0)
    }
}
