#![cfg(any(target_os = "macos", target_os = "ios"))]
//! Shared Apple rendering, task dispatch, and system services for GPUI.
//!
//! This crate renders GPUI scenes directly with Metal on Apple platforms. It
//! owns GPU resources, text rendering, Keychain access, and thermal notifications
//! while leaving application lifecycle, windowing, and input to each platform backend.

mod dispatcher;
pub mod keychain;
mod metal_atlas;
pub mod metal_renderer;
pub mod thermal;

#[cfg(feature = "font-kit")]
mod open_type;
#[cfg(feature = "font-kit")]
mod text_system;

pub use dispatcher::{AppleActivity, AppleDispatcher};
#[cfg(feature = "font-kit")]
pub use text_system::AppleTextSystem;
