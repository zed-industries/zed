#![cfg(any(target_os = "macos", target_os = "ios"))]
//! Metal renderer shared by GPUI's macOS and iOS platform implementations.

mod metal_atlas;
pub mod metal_renderer;

pub use metal_atlas::MetalAtlas;
