use crate::{point, size, Bounds, DisplayId, GlobalPixels, PlatformDisplay, Size};
use anyhow::Result;
use uuid::Uuid;
use x11rb::{connection::Connection as _, rust_connection::RustConnection};

#[derive(Debug)]
pub(crate) struct LinuxDisplay {
    x11_screen_index: usize,
    bounds: Bounds<GlobalPixels>,
    uuid: Uuid,
}

impl LinuxDisplay {
    pub(crate) fn new(xc: &RustConnection, x11_screen_index: usize) -> Self {
        let screen = &xc.setup().roots[x11_screen_index];
        Self {
            x11_screen_index,
            bounds: Bounds {
                origin: Default::default(),
                size: Size {
                    width: GlobalPixels(screen.width_in_pixels as f32),
                    height: GlobalPixels(screen.height_in_pixels as f32),
                },
            },
            uuid: Uuid::from_bytes([0; 16]),
        }
    }
}

impl PlatformDisplay for LinuxDisplay {
    fn id(&self) -> DisplayId {
        DisplayId(self.x11_screen_index as u32)
    }

    fn uuid(&self) -> Result<Uuid> {
        Ok(self.uuid)
    }

    fn bounds(&self) -> Bounds<GlobalPixels> {
        self.bounds
    }
}
