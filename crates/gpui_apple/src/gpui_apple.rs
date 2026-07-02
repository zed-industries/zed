#![cfg(any(target_os = "macos", target_os = "ios"))]
//! Platform pieces shared by GPUI's macOS and iOS implementations: the Metal
//! renderer, the GCD-backed dispatcher, and the Core Text based text system.

mod dispatcher;
mod metal_atlas;
pub mod metal_renderer;

#[cfg(feature = "font-kit")]
mod open_type;
#[cfg(feature = "font-kit")]
mod text_system;

pub use dispatcher::AppleDispatcher;
pub use metal_atlas::MetalAtlas;

#[cfg(feature = "font-kit")]
pub use text_system::CoreTextSystem;
