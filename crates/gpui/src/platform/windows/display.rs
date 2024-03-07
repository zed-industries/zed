use anyhow::{anyhow, Result};
use uuid::Uuid;
use windows::{
    core::PCSTR,
    Win32::Graphics::Gdi::{EnumDisplaySettingsA, DEVMODEA, ENUM_CURRENT_SETTINGS},
};

use crate::{Bounds, DisplayId, GlobalPixels, PlatformDisplay, Point, Size};

#[derive(Debug)]
pub(crate) struct WindowsDisplay;

impl WindowsDisplay {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl PlatformDisplay for WindowsDisplay {
    // todo(windows)
    fn id(&self) -> DisplayId {
        DisplayId(1)
    }

    // todo(windows)
    fn uuid(&self) -> Result<Uuid> {
        Err(anyhow!("not implemented yet."))
    }

    fn bounds(&self) -> Bounds<GlobalPixels> {
        let mut dev = DEVMODEA {
            dmSize: std::mem::size_of::<DEVMODEA>() as _,
            ..unsafe { std::mem::zeroed() }
        };
        unsafe { EnumDisplaySettingsA(PCSTR::null(), ENUM_CURRENT_SETTINGS, &mut dev) };
        let w = dev.dmPelsWidth;
        let h = dev.dmPelsHeight;

        log::debug!("Screen size: {w} {h}");
        Bounds::new(
            Point::new(0.0.into(), 0.0.into()),
            Size {
                width: GlobalPixels(w as f32),
                height: GlobalPixels(h as f32),
            },
        )
    }
}
