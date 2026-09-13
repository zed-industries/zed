//! The renderer contract that engines implement.

use crate::{PlatformAtlas, Scene};
#[cfg(any(test, feature = "test-support", feature = "bench-support"))]
use gpui_types::{DevicePixels, Size};
use std::sync::Arc;

/// A renderer that presents a [`Scene`] to some target.
///
/// OS backends and alternative engines implement this; `gpui` produces the
/// [`Scene`] and submits it through the window's platform implementation.
///
/// Image capture and offscreen rendering are capabilities of the renderer
/// rather than of the OS window, so onscreen GPU renderers only need to
/// implement [`SceneRenderer::draw`] and [`SceneRenderer::sprite_atlas`].
pub trait SceneRenderer: 'static {
    /// Encodes and submits `scene`.
    fn draw(&mut self, scene: &Scene);

    /// Returns the sprite atlas used by this renderer.
    fn sprite_atlas(&self) -> Arc<dyn PlatformAtlas>;

    /// Render a scene and return the result as an RGBA image.
    #[cfg(any(test, feature = "test-support", feature = "bench-support"))]
    fn render_scene_to_image(
        &mut self,
        _scene: &Scene,
        _size: Size<DevicePixels>,
    ) -> anyhow::Result<image::RgbaImage> {
        anyhow::bail!("render_scene_to_image is not supported by this renderer")
    }

    /// Render a scene to an offscreen target without reading the result back.
    ///
    /// This is the headless analogue of presenting a frame: it performs the
    /// same CPU-side scene encoding and GPU submission as drawing to a real
    /// window, but doesn't block on GPU completion or copy pixels back.
    #[cfg(any(test, feature = "test-support", feature = "bench-support"))]
    fn render_scene(&mut self, _scene: &Scene, _size: Size<DevicePixels>) -> anyhow::Result<()> {
        Ok(())
    }
}
