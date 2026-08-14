#![cfg(any(target_os = "macos", target_os = "ios"))]
//! Shared Apple platform support for GPUI.
//!
//! This crate contains the Metal renderer and GPU resource management shared
//! by GPUI's Apple platform backends.

mod metal_atlas;
pub mod metal_renderer;
