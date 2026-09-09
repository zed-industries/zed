use std::{
    ffi::c_void,
    sync::{Arc, OnceLock},
    time::Instant,
};

use anyhow::{Context as _, Result};
use gpui::{
    Bounds, DevicePixels, GpuSpecs, PlatformAtlas, Point, Scene, Size, WindowBackgroundAppearance,
};
use gpui_software::{Damage, FontCorrection, SoftwareRenderer};
use windows::Win32::{
    Foundation::{HWND, RECT},
    Graphics::Gdi::{
        BI_RGB, BITMAPINFO, BITMAPINFOHEADER, DIB_RGB_COLORS, GetDC, GetUpdateRect, ReleaseDC,
        SetDIBitsToDevice,
    },
};

use crate::{DirectXDevices, DirectXRenderer};

pub(crate) enum WindowsRenderer {
    DirectX(DirectXRenderer),
    Software(SoftwareWindowRenderer),
}

impl WindowsRenderer {
    pub(crate) fn draw(
        &mut self,
        scene: &Scene,
        background: WindowBackgroundAppearance,
    ) -> Result<()> {
        match self {
            Self::DirectX(renderer) => renderer.draw(scene, background),
            Self::Software(renderer) => renderer.draw(scene),
        }
    }

    pub(crate) fn sprite_atlas(&self) -> Arc<dyn PlatformAtlas> {
        match self {
            Self::DirectX(renderer) => renderer.sprite_atlas(),
            Self::Software(renderer) => renderer.sprite_atlas(),
        }
    }

    pub(crate) fn gpu_specs(&self) -> Result<GpuSpecs> {
        match self {
            Self::DirectX(renderer) => renderer.gpu_specs(),
            Self::Software(renderer) => Ok(renderer.gpu_specs()),
        }
    }

    pub(crate) fn resize(&mut self, size: Size<DevicePixels>) -> Result<()> {
        match self {
            Self::DirectX(renderer) => renderer.resize(size),
            Self::Software(renderer) => {
                renderer.resize(size);
                Ok(())
            }
        }
    }

    pub(crate) fn handle_device_lost(&mut self, devices: &DirectXDevices) -> Result<()> {
        if let Self::DirectX(renderer) = self {
            renderer.handle_device_lost(devices)?;
        }
        Ok(())
    }

    pub(crate) fn mark_drawable(&mut self) {
        match self {
            Self::DirectX(renderer) => renderer.mark_drawable(),
            Self::Software(renderer) => renderer.force_presentation = true,
        }
    }

    pub(crate) fn requires_presentation(&self) -> bool {
        match self {
            Self::DirectX(_) => false,
            Self::Software(renderer) => {
                renderer.force_presentation || renderer.update_rect().is_some()
            }
        }
    }

    pub(crate) fn is_software(&self) -> bool {
        matches!(self, Self::Software(_))
    }

    #[cfg(any(test, feature = "test-support"))]
    pub(crate) fn render_to_image(
        &mut self,
        scene: &Scene,
        background: WindowBackgroundAppearance,
    ) -> Result<image::RgbaImage> {
        match self {
            Self::DirectX(renderer) => renderer.render_to_image(scene, background),
            Self::Software(renderer) => renderer.render_to_image(scene),
        }
    }
}

pub(crate) struct SoftwareWindowRenderer {
    hwnd: HWND,
    renderer: SoftwareRenderer,
    force_presentation: bool,
}

impl SoftwareWindowRenderer {
    pub(crate) fn new(
        hwnd: HWND,
        size: Size<DevicePixels>,
        font_correction: FontCorrection,
    ) -> Self {
        Self {
            hwnd,
            renderer: SoftwareRenderer::new(size, font_correction),
            force_presentation: true,
        }
    }

    fn draw(&mut self, scene: &Scene) -> Result<()> {
        let mut damage = self.renderer.draw(scene, false);
        let exposure = if self.force_presentation {
            Some(Bounds::new(
                Default::default(),
                self.renderer.framebuffer().size(),
            ))
        } else {
            self.update_rect()
        };
        if let Some(exposure) = exposure {
            include_exposure(&mut damage.rects, exposure);
        }
        let present_start = Instant::now();
        let result = self.present(&damage);
        self.force_presentation = result.is_err();
        if software_stats_enabled() {
            log::info!(
                "gpui_software: present={:?} damage_rects={}",
                present_start.elapsed(),
                damage.rects.len()
            );
        }
        result
    }

    fn sprite_atlas(&self) -> Arc<dyn PlatformAtlas> {
        self.renderer.atlas()
    }

    fn gpu_specs(&self) -> GpuSpecs {
        self.renderer.gpu_specs()
    }

    fn resize(&mut self, size: Size<DevicePixels>) {
        self.renderer.resize(size);
        self.force_presentation = true;
    }

    #[cfg(any(test, feature = "test-support"))]
    fn render_to_image(&mut self, scene: &Scene) -> Result<image::RgbaImage> {
        self.renderer.draw(scene, true);
        let framebuffer = self.renderer.framebuffer();
        let size = framebuffer.size();
        let width = size.width.0.max(0) as u32;
        let height = size.height.0.max(0) as u32;
        let mut rgba = Vec::with_capacity(width as usize * height as usize * 4);
        for pixel in framebuffer.pixels() {
            rgba.extend_from_slice(&[
                (pixel >> 16) as u8,
                (pixel >> 8) as u8,
                *pixel as u8,
                (pixel >> 24) as u8,
            ]);
        }
        image::RgbaImage::from_raw(width, height, rgba)
            .context("Failed to build RgbaImage from software framebuffer")
    }

    fn update_rect(&self) -> Option<Bounds<DevicePixels>> {
        let mut update = RECT::default();
        if unsafe { GetUpdateRect(self.hwnd, Some(&mut update), false) }.as_bool() {
            let size = self.renderer.framebuffer().size();
            update.left = update.left.clamp(0, size.width.0.max(0));
            update.right = update.right.clamp(0, size.width.0.max(0));
            update.top = update.top.clamp(0, size.height.0.max(0));
            update.bottom = update.bottom.clamp(0, size.height.0.max(0));
        }
        (update.right > update.left && update.bottom > update.top).then_some(Bounds {
            origin: Point {
                x: DevicePixels(update.left),
                y: DevicePixels(update.top),
            },
            size: Size {
                width: DevicePixels(update.right - update.left),
                height: DevicePixels(update.bottom - update.top),
            },
        })
    }

    fn present(&self, damage: &Damage) -> Result<()> {
        if damage.rects.is_empty() {
            return Ok(());
        }
        let framebuffer = self.renderer.framebuffer();
        let size = framebuffer.size();
        if size.width.0 <= 0 || size.height.0 <= 0 {
            return Ok(());
        }
        let bitmap_info = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: size.width.0,
                biHeight: -size.height.0,
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB.0,
                ..Default::default()
            },
            ..Default::default()
        };
        let device_context = unsafe { GetDC(Some(self.hwnd)) };
        if device_context.is_invalid() {
            anyhow::bail!("GetDC failed while presenting a software frame");
        }
        let present_result = (|| {
            for rect in &damage.rects {
                let width = rect.size.width.0.max(0) as u32;
                let height = rect.size.height.0.max(0) as u32;
                if width == 0 || height == 0 {
                    continue;
                }
                let copied = unsafe {
                    SetDIBitsToDevice(
                        device_context,
                        rect.origin.x.0,
                        rect.origin.y.0,
                        width,
                        height,
                        rect.origin.x.0,
                        size.height.0 - rect.origin.y.0 - rect.size.height.0,
                        0,
                        size.height.0 as u32,
                        framebuffer.as_ptr().cast::<c_void>(),
                        &bitmap_info,
                        DIB_RGB_COLORS,
                    )
                };
                if copied == 0 {
                    return Err(std::io::Error::last_os_error())
                        .context("SetDIBitsToDevice failed while presenting a software frame");
                }
            }
            Ok(())
        })();
        let released = unsafe { ReleaseDC(Some(self.hwnd), device_context) };
        if released == 0 && present_result.is_ok() {
            return Err(std::io::Error::last_os_error())
                .context("ReleaseDC failed after presenting a software frame");
        }
        present_result
    }
}

fn include_exposure(damage: &mut Vec<Bounds<DevicePixels>>, exposure: Bounds<DevicePixels>) {
    let covers = |outer: &Bounds<DevicePixels>, inner: &Bounds<DevicePixels>| {
        outer.origin.x <= inner.origin.x
            && outer.origin.y <= inner.origin.y
            && outer.bottom_right().x >= inner.bottom_right().x
            && outer.bottom_right().y >= inner.bottom_right().y
    };
    if damage.iter().any(|rect| covers(rect, &exposure)) {
        return;
    }
    damage.retain(|rect| !covers(&exposure, rect));
    damage.push(exposure);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exposure_repairs_unchanged_pixels_without_duplicate_copies() {
        let full = Bounds::new(
            Default::default(),
            Size {
                width: DevicePixels(1600),
                height: DevicePixels(900),
            },
        );
        let caret = Bounds::new(
            Point {
                x: DevicePixels(40),
                y: DevicePixels(20),
            },
            Size {
                width: DevicePixels(2),
                height: DevicePixels(16),
            },
        );
        let mut damage = vec![caret];
        include_exposure(&mut damage, full);
        assert_eq!(damage, [full]);
        include_exposure(&mut damage, caret);
        assert_eq!(damage, [full]);
        include_exposure(&mut damage, full);
        assert_eq!(damage, [full]);
        damage.clear();
        include_exposure(&mut damage, caret);
        assert_eq!(damage, [caret]);
    }
}

fn software_stats_enabled() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| {
        std::env::var("GPUI_SOFTWARE_STATS").is_ok_and(|value| value == "1" || value == "true")
    })
}
