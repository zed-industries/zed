#![cfg(any(target_os = "macos", target_os = "ios"))]
//! Shared Apple GPU rendering for GPUI.
//!
//! This crate renders GPUI scenes directly with Metal on Apple platforms. It
//! owns GPU resources and shaders while leaving application lifecycle,
//! windowing, and input to each platform backend.

mod metal_atlas;
pub mod metal_renderer;
