pub(crate) use blade_graphics::gles as hal;

#[path = "blade_atlas.rs"]
mod blade_atlas;

#[path = "blade_renderer.rs"]
mod blade_renderer;

unsafe impl Send for blade_atlas::BladeAtlasState {}

pub(crate) use blade_atlas::*;
pub(crate) use blade_renderer::*;
