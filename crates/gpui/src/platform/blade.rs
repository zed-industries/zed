mod blade_gles;
mod blade_hal;

use crate::{Bounds, DevicePixels, PlatformAtlas, Point, Size};
use blade_graphics as gpu;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

use std::sync::Arc;

pub trait BladeRenderer {
    fn update_drawable_size(&mut self, size: Size<DevicePixels>);
    fn update_transparency(&mut self, transparent: bool);
    fn viewport_size(&self) -> gpu::Extent;
    fn sprite_atlas(&self) -> Arc<dyn PlatformAtlas>;

    #[cfg(target_os = "macos")]
    fn layer(&self) -> metal::MetalLayer;

    #[cfg(target_os = "macos")]
    fn layer_ptr(&self) -> *mut metal::CAMetalLayer;

    fn destroy(&mut self);
    fn draw(&mut self, scene: &crate::Scene);
}

pub fn new_renderer<W: HasWindowHandle + HasDisplayHandle>(
    raw: &W,
    config: BladeSurfaceConfig,
) -> anyhow::Result<Box<dyn BladeRenderer>> {
    Ok(
        if std::env::var("USE_GLES")
            .ok()
            .filter(|t| !t.is_empty())
            .is_some()
        {
            Box::new(blade_gles::BladeRenderer::new_from_window(raw, config)?)
        } else {
            Box::new(blade_hal::BladeRenderer::new_from_window(raw, config)?)
        },
    )
}

pub struct BladeSurfaceConfig {
    pub size: gpu::Extent,
    pub transparent: bool,
}

impl From<Size<DevicePixels>> for etagere::Size {
    fn from(size: Size<DevicePixels>) -> Self {
        etagere::Size::new(size.width.into(), size.height.into())
    }
}

impl From<etagere::Point> for Point<DevicePixels> {
    fn from(value: etagere::Point) -> Self {
        Point {
            x: DevicePixels::from(value.x),
            y: DevicePixels::from(value.y),
        }
    }
}

impl From<etagere::Size> for Size<DevicePixels> {
    fn from(size: etagere::Size) -> Self {
        Size {
            width: DevicePixels::from(size.width),
            height: DevicePixels::from(size.height),
        }
    }
}

impl From<etagere::Rectangle> for Bounds<DevicePixels> {
    fn from(rectangle: etagere::Rectangle) -> Self {
        Bounds {
            origin: rectangle.min.into(),
            size: rectangle.size().into(),
        }
    }
}
