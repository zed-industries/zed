//! Off-screen rendering context for GPUI.

use crate::{
    BoxedDrawableOffScreenTarget, DevicePixels, DrawableOffScreenTarget, OffScreenImage, Scene,
    SharedTextureHandle, Size,
};
use anyhow::Result;

/// Renders a pre-built scene to an off-screen render target.
///
/// This is the low-level API for off-screen rendering. It takes a scene that
/// has already been built (e.g., from a Window's render pass) and draws it
/// to the off-screen target.
///
/// # Arguments
///
/// * `target` - The off-screen render target to draw to
/// * `scene` - The scene to render
///
/// # Example
///
/// ```ignore
/// let config = OffScreenTargetConfig::new(size(DevicePixels(800), DevicePixels(600)));
/// let mut target = cx.create_offscreen_target(config).unwrap();
///
/// // Build a scene (typically from window rendering)
/// let scene = build_scene();
///
/// render_scene_offscreen(&mut target, &scene);
/// let image = target.read_pixels()?;
/// ```
pub(crate) fn render_scene_offscreen(
    target: &mut dyn DrawableOffScreenTarget,
    scene: &Scene,
) -> Result<()> {
    target.draw(scene);
    target.finish_frame();
    Ok(())
}

/// Renders a scene to an off-screen target and returns the pixel data.
///
/// This is a convenience function that combines rendering and readback
/// in a single call.
///
/// # Arguments
///
/// * `target` - The off-screen render target to draw to
/// * `scene` - The scene to render
///
/// # Returns
///
/// The rendered image data, or an error if rendering or readback failed.
pub(crate) fn render_scene_to_image(
    target: &mut dyn DrawableOffScreenTarget,
    scene: &Scene,
) -> Result<OffScreenImage> {
    render_scene_offscreen(target, scene)?;
    target.read_pixels()
}

/// A helper struct for managing off-screen rendering operations.
///
/// This provides a higher-level interface for off-screen rendering,
/// handling target management and providing convenience methods.
///
/// Note: The `render()` and `render_to_image()` methods that take a `Scene`
/// are internal because `Scene` is a `pub(crate)` type. Public users should
/// use the other methods to read pixels, resize, or get shared handles.
pub struct OffScreenRenderer {
    target: BoxedDrawableOffScreenTarget,
}

impl OffScreenRenderer {
    /// Creates a new off-screen renderer with the given target.
    pub(crate) fn new(target: BoxedDrawableOffScreenTarget) -> Self {
        Self { target }
    }

    /// Returns a reference to the underlying render target.
    pub(crate) fn target(&self) -> &dyn DrawableOffScreenTarget {
        self.target.as_ref()
    }

    /// Returns a mutable reference to the underlying render target.
    pub(crate) fn target_mut(&mut self) -> &mut dyn DrawableOffScreenTarget {
        self.target.as_mut()
    }

    /// Renders a scene to the off-screen target.
    pub(crate) fn render(&mut self, scene: &Scene) -> Result<()> {
        render_scene_offscreen(self.target.as_mut(), scene)
    }

    /// Renders a scene and returns the pixel data.
    pub(crate) fn render_to_image(&mut self, scene: &Scene) -> Result<OffScreenImage> {
        render_scene_to_image(self.target.as_mut(), scene)
    }

    /// Reads the current contents of the render target without re-rendering.
    pub fn read_pixels(&self) -> Result<OffScreenImage> {
        self.target.read_pixels()
    }

    /// Resizes the render target.
    ///
    /// This may reallocate GPU resources. Any previously rendered content
    /// will be lost.
    pub fn resize(&mut self, size: Size<DevicePixels>) {
        self.target.resize(size);
    }

    /// Returns the current size of the render target.
    pub fn size(&self) -> Size<DevicePixels> {
        self.target.size()
    }

    /// Returns the pixel format used by this render target.
    pub fn pixel_format(&self) -> crate::PixelFormat {
        self.target.pixel_format()
    }

    /// Returns the shared texture handle if available.
    ///
    /// This can be used for zero-copy sharing with other processes or
    /// graphics APIs.
    pub fn shared_texture_handle(&self) -> Option<SharedTextureHandle> {
        self.target.shared_texture_handle()
    }

    /// Returns whether this target supports zero-copy texture sharing.
    pub fn supports_shared_textures(&self) -> bool {
        self.target.supports_shared_textures()
    }

    /// Returns whether double-buffering is enabled for this target.
    ///
    /// When double-buffering is enabled, the target uses two staging buffers
    /// to allow rendering to continue while a previous frame is being read.
    /// This can improve throughput for continuous rendering scenarios.
    pub fn is_double_buffered(&self) -> bool {
        self.target.is_double_buffered()
    }

    /// Acquires exclusive access to the shared texture for rendering.
    ///
    /// When texture sharing is enabled, this should be called before rendering
    /// to ensure proper synchronization with consumers.
    ///
    /// # Arguments
    ///
    /// * `key` - The synchronization key (typically 0 for the producer)
    /// * `timeout_ms` - Timeout in milliseconds, or `u32::MAX` for infinite
    ///
    /// # Returns
    ///
    /// * `Ok(true)` - Sync acquired successfully
    /// * `Ok(false)` - Timeout occurred
    /// * `Err(_)` - Acquisition failed
    pub fn acquire_sync(&self, key: u64, timeout_ms: u32) -> anyhow::Result<bool> {
        self.target.acquire_sync(key, timeout_ms)
    }

    /// Releases exclusive access to the shared texture.
    ///
    /// This should be called after rendering is complete to signal that
    /// consumers can now access the texture.
    ///
    /// # Arguments
    ///
    /// * `key` - The synchronization key for consumers (typically 0 or 1)
    pub fn release_sync(&self, key: u64) -> anyhow::Result<()> {
        self.target.release_sync(key)
    }

    /// Returns whether this target supports synchronization primitives.
    ///
    /// When true, `acquire_sync` and `release_sync` can be used to
    /// coordinate access to the shared texture between processes.
    pub fn supports_sync(&self) -> bool {
        self.target.supports_sync()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{OffScreenImage, OffScreenRenderTarget, PixelFormat};

    // Mock implementation for testing
    struct MockOffScreenTarget {
        width: u32,
        height: u32,
        draw_count: std::cell::Cell<u32>,
    }

    impl MockOffScreenTarget {
        fn new(width: u32, height: u32) -> Self {
            Self {
                width,
                height,
                draw_count: std::cell::Cell::new(0),
            }
        }
    }

    impl OffScreenRenderTarget for MockOffScreenTarget {
        fn size(&self) -> Size<DevicePixels> {
            Size {
                width: DevicePixels(self.width as i32),
                height: DevicePixels(self.height as i32),
            }
        }

        fn resize(&mut self, size: Size<DevicePixels>) {
            self.width = size.width.0.max(1) as u32;
            self.height = size.height.0.max(1) as u32;
        }

        fn pixel_format(&self) -> PixelFormat {
            PixelFormat::Bgra8Unorm
        }

        fn read_pixels(&self) -> Result<OffScreenImage> {
            let data = vec![0u8; (self.width * self.height * 4) as usize];
            Ok(OffScreenImage::new(
                data,
                self.width,
                self.height,
                PixelFormat::Bgra8Unorm,
            ))
        }

        fn shared_texture_handle(&self) -> Option<SharedTextureHandle> {
            None
        }
    }

    impl DrawableOffScreenTarget for MockOffScreenTarget {
        fn draw(&mut self, _scene: &Scene) {
            self.draw_count.set(self.draw_count.get() + 1);
        }

        fn finish_frame(&mut self) {
            // No-op for mock
        }
    }

    #[test]
    fn test_offscreen_renderer_creation() {
        let target = Box::new(MockOffScreenTarget::new(100, 100));
        let renderer = OffScreenRenderer::new(target);
        assert_eq!(renderer.size().width.0, 100);
        assert_eq!(renderer.size().height.0, 100);
    }

    #[test]
    fn test_offscreen_renderer_resize() {
        let target = Box::new(MockOffScreenTarget::new(100, 100));
        let mut renderer = OffScreenRenderer::new(target);

        renderer.resize(crate::size(DevicePixels(200), DevicePixels(150)));
        assert_eq!(renderer.size().width.0, 200);
        assert_eq!(renderer.size().height.0, 150);
    }

    #[test]
    fn test_offscreen_renderer_read_pixels() {
        let target = Box::new(MockOffScreenTarget::new(100, 100));
        let renderer = OffScreenRenderer::new(target);

        let image = renderer.read_pixels().unwrap();
        assert_eq!(image.width, 100);
        assert_eq!(image.height, 100);
        assert!(image.is_valid());
    }
}
