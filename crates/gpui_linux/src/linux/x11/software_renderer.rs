use std::{num::NonZeroU32, rc::Rc, sync::Arc};

use anyhow::{Context as _, Result, anyhow};
use gpui::{DevicePixels, GpuSpecs, PlatformAtlas, Scene, Size};
use gpui_software::{FontCorrection, SoftwareRenderer};
use gpui_wgpu::{CompositorGpuHint, GpuContext, WgpuRenderer, WgpuSurfaceConfig};

use x11rb::{connection::Connection as _, xcb_ffi::XCBConnection};

use super::window::RawWindow;

pub(super) enum X11Renderer {
    Wgpu(WgpuRenderer),
    Software(Box<X11SoftwareRenderer>),
}

impl X11Renderer {
    pub(super) fn new(
        gpu_context: GpuContext,
        raw_window: RawWindow,
        config: WgpuSurfaceConfig,
        compositor_gpu: Option<CompositorGpuHint>,
    ) -> Result<Self> {
        if std::env::var("GPUI_RENDERER").is_ok_and(|value| value == "software") {
            let raw_window = Rc::new(raw_window);
            let context = softbuffer::Context::new(raw_window.clone())
                .map_err(|error| anyhow!("Creating X11 software context: {error}"))?;
            let surface = softbuffer::Surface::new(&context, raw_window)
                .map_err(|error| anyhow!("Creating X11 software surface: {error}"))?;
            Ok(Self::Software(Box::new(X11SoftwareRenderer {
                surface: Some(surface),
                renderer: SoftwareRenderer::new(config.size, FontCorrection::default()),
                font_correction: FontCorrection::default(),
                needs_redraw: false,
            })))
        } else {
            WgpuRenderer::new(gpu_context, &raw_window, config, compositor_gpu).map(Self::Wgpu)
        }
    }

    pub(super) fn draw(&mut self, scene: &Scene, connection: &XCBConnection) {
        match self {
            Self::Wgpu(renderer) => {
                renderer.draw(scene);
            }
            Self::Software(renderer) => {
                renderer.needs_redraw = match renderer.draw(scene).and_then(|()| {
                    // softbuffer queues XCB requests; an idle window may never acquire another
                    // buffer to flush them implicitly. Exposure repairs must reach the server now.
                    connection
                        .flush()
                        .context("Flushing X11 software presentation")
                }) {
                    Ok(()) => false,
                    Err(error) => {
                        log::error!("Failed to present X11 software frame: {error:#}");
                        true
                    }
                };
            }
        }
    }

    pub(super) fn sprite_atlas(&self) -> Arc<dyn PlatformAtlas> {
        match self {
            Self::Wgpu(renderer) => renderer.sprite_atlas().clone(),
            Self::Software(renderer) => renderer.renderer.atlas(),
        }
    }

    pub(super) fn gpu_specs(&self) -> GpuSpecs {
        match self {
            Self::Wgpu(renderer) => renderer.gpu_specs(),
            Self::Software(renderer) => renderer.renderer.gpu_specs(),
        }
    }

    pub(super) fn max_texture_size(&self) -> u32 {
        match self {
            Self::Wgpu(renderer) => renderer.max_texture_size(),
            Self::Software(_) => 16_384,
        }
    }

    pub(super) fn update_drawable_size(&mut self, size: Size<DevicePixels>) {
        match self {
            Self::Wgpu(renderer) => renderer.update_drawable_size(size),
            Self::Software(renderer) => renderer.renderer.resize(size),
        }
    }

    pub(super) fn set_subpixel_layout(&mut self, is_bgr: bool) {
        match self {
            Self::Wgpu(renderer) => renderer.set_subpixel_layout(is_bgr),
            Self::Software(renderer) => {
                renderer.font_correction.is_bgr = is_bgr;
                renderer
                    .renderer
                    .set_font_correction(renderer.font_correction);
            }
        }
    }

    pub(super) fn update_transparency(&mut self, transparent: bool) {
        if let Self::Wgpu(renderer) = self {
            renderer.update_transparency(transparent);
        }
    }

    pub(super) fn supports_subpixel_rendering(&self) -> bool {
        match self {
            Self::Wgpu(renderer) => renderer.supports_dual_source_blending(),
            Self::Software(_) => true,
        }
    }

    pub(super) fn device_lost(&self) -> bool {
        match self {
            Self::Wgpu(renderer) => renderer.device_lost(),
            Self::Software(_) => false,
        }
    }

    pub(super) fn recover(&mut self, raw_window: &RawWindow) -> Result<()> {
        if let Self::Wgpu(renderer) = self {
            renderer.recover(raw_window)?;
        }
        Ok(())
    }

    pub(super) fn needs_redraw(&mut self) -> bool {
        match self {
            Self::Wgpu(renderer) => renderer.needs_redraw(),
            Self::Software(renderer) => renderer.needs_redraw,
        }
    }

    pub(super) fn destroy(&mut self) {
        match self {
            Self::Wgpu(renderer) => renderer.destroy(),
            Self::Software(renderer) => {
                renderer.surface.take();
            }
        }
    }
}

pub(super) struct X11SoftwareRenderer {
    surface: Option<softbuffer::Surface<Rc<RawWindow>, Rc<RawWindow>>>,
    renderer: SoftwareRenderer,
    font_correction: FontCorrection,
    needs_redraw: bool,
}

impl X11SoftwareRenderer {
    fn draw(&mut self, scene: &Scene) -> Result<()> {
        self.renderer.draw(scene, false);
        let framebuffer = self.renderer.framebuffer();
        let size = framebuffer.size();
        let width = NonZeroU32::new(size.width.0 as u32).context("Empty X11 frame width")?;
        let height = NonZeroU32::new(size.height.0 as u32).context("Empty X11 frame height")?;
        let surface = self
            .surface
            .as_mut()
            .context("Destroyed X11 software surface")?;
        surface
            .resize(width, height)
            .map_err(|error| anyhow!("Resizing X11 software surface: {error}"))?;
        let mut buffer = surface
            .buffer_mut()
            .map_err(|error| anyhow!("Acquiring X11 software buffer: {error}"))?;
        buffer.copy_from_slice(framebuffer.pixels());
        buffer
            .present()
            .map_err(|error| anyhow!("Presenting X11 software buffer: {error}"))
    }
}
