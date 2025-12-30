//! Off-Screen Rendering (OSR) support for GPUI.
use crate::{DevicePixels, Scene, Size};
use std::sync::Arc;

/// Pixel format for off-screen rendered images.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelFormat {
    /// 8-bit BGRA, unsigned normalized (common on Windows/Metal)
    Bgra8Unorm,
    /// 8-bit RGBA, unsigned normalized
    Rgba8Unorm,
}

impl PixelFormat {
    /// Returns the number of bytes per pixel for this format.
    pub fn bytes_per_pixel(&self) -> u32 {
        match self {
            PixelFormat::Bgra8Unorm | PixelFormat::Rgba8Unorm => 4,
        }
    }
}

/// CPU-accessible image data from off-screen rendering.
///
/// This struct holds pixel data that has been copied from GPU memory.
/// For zero-copy access, use [`SharedTextureHandle`] instead.
#[derive(Debug, Clone)]
pub struct OffScreenImage {
    /// Raw pixel data in the specified format.
    pub data: Vec<u8>,
    /// Width in pixels.
    pub width: u32,
    /// Height in pixels.
    pub height: u32,
    /// Pixel format of the data.
    pub format: PixelFormat,
    /// Number of bytes per row.
    pub row_stride: u32,
}

impl OffScreenImage {
    /// Creates a new off-screen image with the given parameters.
    pub fn new(data: Vec<u8>, width: u32, height: u32, format: PixelFormat) -> Self {
        let row_stride = width * format.bytes_per_pixel();
        Self {
            data,
            width,
            height,
            format,
            row_stride,
        }
    }

    /// Creates a new off-screen image with explicit row stride.
    pub fn with_stride(
        data: Vec<u8>,
        width: u32,
        height: u32,
        format: PixelFormat,
        row_stride: u32,
    ) -> Self {
        Self {
            data,
            width,
            height,
            format,
            row_stride,
        }
    }

    /// Returns the expected size of the data buffer in bytes.
    pub fn expected_data_size(&self) -> usize {
        (self.row_stride * self.height) as usize
    }

    /// Returns true if the data buffer has the expected size.
    pub fn is_valid(&self) -> bool {
        self.data.len() >= self.expected_data_size()
    }
}

/// Platform-specific shared texture handle for zero-copy GPU resource sharing.
///
/// These handles can be passed to other processes or graphics APIs to access
/// the rendered texture directly on the GPU, avoiding expensive CPU copies.
///
/// # Platform Support
///
/// - **Windows**: DXGI shared handles via `ID3D11Texture2D`
/// - **macOS**: IOSurface references
/// - **Linux**: DMA-BUF file descriptors
///
/// # Safety
///
/// Shared texture handles represent GPU resources that may be accessed by
/// multiple processes. Users must ensure proper synchronization when accessing
/// shared textures.
#[derive(Debug)]
pub enum SharedTextureHandle {
    /// Windows DirectX 11 shared texture.
    #[cfg(target_os = "windows")]
    DirectX(D3D11SharedTexture),

    /// macOS Metal texture backed by IOSurface.
    #[cfg(target_os = "macos")]
    Metal(MetalSharedTexture),

    /// Linux Vulkan texture with DMA-BUF export.
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    Vulkan(VulkanSharedTexture),
}

/// Windows DirectX 11 shared texture information.
#[cfg(target_os = "windows")]
#[derive(Debug, Clone)]
pub struct D3D11SharedTexture {
    /// DXGI shared handle that can be opened by other D3D11 devices.
    /// This handle should be duplicated before passing to other processes.
    pub shared_handle: windows::Win32::Foundation::HANDLE,
    /// Width in pixels.
    pub width: u32,
    /// Height in pixels.
    pub height: u32,
    /// DXGI format of the texture.
    pub format: u32,
}

/// macOS Metal shared texture information.
#[cfg(target_os = "macos")]
#[derive(Debug)]
pub struct MetalSharedTexture {
    /// IOSurface reference for cross-process sharing.
    /// IOSurface can be shared via Mach ports or XPC.
    pub io_surface_id: u32,
    /// Width in pixels.
    pub width: u32,
    /// Height in pixels.
    pub height: u32,
    /// CoreVideo pixel format type.
    pub pixel_format: u32,
}

/// Linux Vulkan shared texture with DMA-BUF export.
#[cfg(any(target_os = "linux", target_os = "freebsd"))]
#[derive(Debug)]
pub struct VulkanSharedTexture {
    /// DMA-BUF file descriptor for zero-copy sharing.
    /// This fd can be passed to other processes via Unix domain sockets.
    pub dmabuf_fd: std::os::unix::io::RawFd,
    /// DRM format fourcc code.
    pub drm_format: u32,
    /// DRM format modifier (for tiling, compression, etc.)
    pub drm_modifier: u64,
    /// Width in pixels.
    pub width: u32,
    /// Height in pixels.
    pub height: u32,
    /// Row stride in bytes.
    pub stride: u32,
    /// Offset to the first pixel in the buffer.
    pub offset: u32,
}

/// A render target that can receive GPUI scene rendering.
///
/// This trait abstracts over different types of render targets:
/// - Window surfaces (existing behavior)
/// - Off-screen textures (new OSR behavior)
pub(crate) trait RenderTarget: Send + Sync {
    /// Returns the current size of the render target in device pixels.
    fn size(&self) -> Size<DevicePixels>;

    /// Resizes the render target.
    ///
    /// This may reallocate GPU resources. The new size takes effect
    /// on the next call to [`draw`](Self::draw).
    fn resize(&mut self, size: Size<DevicePixels>);

    /// Draws a scene to this render target.
    ///
    /// This submits GPU commands to render the scene. The rendering
    /// may not be complete when this method returns; call
    /// [`finish_frame`](Self::finish_frame) to ensure completion.
    fn draw(&mut self, scene: &Scene);

    /// Signals that the current frame is complete.
    ///
    /// For window surfaces, this typically presents the frame.
    /// For off-screen targets, this ensures all GPU commands have completed.
    fn finish_frame(&mut self);
}

/// An off-screen render target that supports pixel readback and texture sharing.
///
/// This trait extends [`RenderTarget`] with capabilities specific to
/// off-screen rendering, including:
///
/// - Reading pixels back to CPU memory
/// - Obtaining shared texture handles for zero-copy access
/// - Querying the pixel format
pub(crate) trait OffScreenRenderTarget: RenderTarget {
    /// Returns the pixel format used by this render target.
    fn pixel_format(&self) -> PixelFormat;

    /// Reads the rendered pixels back to CPU memory.
    ///
    /// This operation requires a GPU->CPU copy and may block until
    /// rendering is complete. For better performance with repeated
    /// readbacks, consider using double-buffering.
    ///
    /// # Returns
    ///
    /// An [`OffScreenImage`] containing the pixel data, or an error
    /// if the readback failed.
    fn read_pixels(&self) -> anyhow::Result<OffScreenImage>;

    /// Returns a shared texture handle for zero-copy access.
    ///
    /// This allows other processes or graphics APIs to access the
    /// rendered texture directly on the GPU.
    ///
    /// # Returns
    ///
    /// - `Some(handle)` if the platform supports texture sharing
    /// - `None` if texture sharing is not available
    fn shared_texture_handle(&self) -> Option<SharedTextureHandle>;

    /// Returns whether this target supports zero-copy texture sharing.
    fn supports_shared_textures(&self) -> bool {
        self.shared_texture_handle().is_some()
    }
}

/// Configuration for creating an off-screen render target.
#[derive(Debug, Clone)]
pub struct OffScreenTargetConfig {
    /// Initial size of the render target.
    pub size: Size<DevicePixels>,
    /// Whether to enable shared texture support for zero-copy access.
    /// Enabling this may have a small performance cost on some platforms.
    pub enable_sharing: bool,
    /// Preferred pixel format. The implementation may choose a different
    /// format if the preferred one is not supported.
    pub preferred_format: Option<PixelFormat>,
}

impl OffScreenTargetConfig {
    /// Creates a new configuration with the given size.
    pub fn new(size: Size<DevicePixels>) -> Self {
        Self {
            size,
            enable_sharing: false,
            preferred_format: None,
        }
    }

    /// Enables shared texture support for zero-copy access.
    pub fn with_sharing(mut self) -> Self {
        self.enable_sharing = true;
        self
    }

    /// Sets the preferred pixel format.
    pub fn with_format(mut self, format: PixelFormat) -> Self {
        self.preferred_format = Some(format);
        self
    }
}

/// A boxed off-screen render target for dynamic dispatch.
pub(crate) type BoxedOffScreenTarget = Box<dyn OffScreenRenderTarget>;

/// An Arc-wrapped off-screen render target for shared ownership.
pub(crate) type SharedOffScreenTarget = Arc<dyn OffScreenRenderTarget>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::size;

    #[test]
    fn test_pixel_format_bytes_per_pixel() {
        assert_eq!(PixelFormat::Bgra8Unorm.bytes_per_pixel(), 4);
        assert_eq!(PixelFormat::Rgba8Unorm.bytes_per_pixel(), 4);
    }

    #[test]
    fn test_offscreen_image_creation() {
        let data = vec![0u8; 100 * 100 * 4];
        let image = OffScreenImage::new(data.clone(), 100, 100, PixelFormat::Bgra8Unorm);

        assert_eq!(image.width, 100);
        assert_eq!(image.height, 100);
        assert_eq!(image.row_stride, 400);
        assert!(image.is_valid());
    }

    #[test]
    fn test_offscreen_image_with_stride() {
        // Stride with padding (e.g., 512-byte aligned rows)
        let row_stride = 512;
        let data = vec![0u8; (row_stride * 100) as usize];
        let image =
            OffScreenImage::with_stride(data, 100, 100, PixelFormat::Bgra8Unorm, row_stride);

        assert_eq!(image.row_stride, 512);
        assert!(image.is_valid());
    }

    #[test]
    fn test_offscreen_target_config() {
        let config =
            OffScreenTargetConfig::new(size(crate::DevicePixels(800), crate::DevicePixels(600)))
                .with_sharing()
                .with_format(PixelFormat::Bgra8Unorm);

        assert!(config.enable_sharing);
        assert_eq!(config.preferred_format, Some(PixelFormat::Bgra8Unorm));
    }
}
