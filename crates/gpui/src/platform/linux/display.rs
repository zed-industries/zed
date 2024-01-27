use crate::{point, size, Bounds, DisplayId, GlobalPixels, PlatformDisplay};
use anyhow::Result;
use uuid::Uuid;

#[derive(Debug)]
pub(crate) struct LinuxDisplay;

impl PlatformDisplay for LinuxDisplay {
    fn id(&self) -> DisplayId {
        DisplayId(0)
    }

    fn uuid(&self) -> Result<Uuid> {
        Ok(Uuid::from_bytes([0; 16]))
    }

    fn bounds(&self) -> Bounds<GlobalPixels> {
        Bounds {
            origin: point(GlobalPixels(0.0), GlobalPixels(0.0)),
            size: size(GlobalPixels(100.0), GlobalPixels(100.0)),
        }
    }
}
