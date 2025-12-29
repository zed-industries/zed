#[cfg(target_os = "macos")]
mod apple_compat;
mod blade_atlas;
mod blade_context;
mod blade_renderer;

#[cfg(target_os = "macos")]
pub(crate) use apple_compat::*;
pub(crate) use blade_atlas::*;
pub(crate) use blade_context::*;
pub(crate) use blade_renderer::*;

// Public re-exports for custom render pass feature
#[cfg(feature = "custom_render_pass")]
pub use blade_renderer::{BladeRenderPassContext, CustomRenderPass};
