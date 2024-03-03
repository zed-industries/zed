use anyhow::{anyhow, Result};
use uuid::Uuid;

use crate::{Bounds, DisplayId, GlobalPixels, PlatformDisplay, Point, Size};

#[derive(Debug)]
pub(crate) struct WindowsDisplay;

impl WindowsDisplay {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl PlatformDisplay for WindowsDisplay {
    // todo!("windows")
    fn id(&self) -> DisplayId {
        DisplayId(1)
    }

    // todo!("windows")
    fn uuid(&self) -> Result<Uuid> {
        Err(anyhow!("not implemented yet."))
    }

    // todo!("windows")
    fn bounds(&self) -> Bounds<GlobalPixels> {
        Bounds::new(
            Point::new(0.0.into(), 0.0.into()),
            Size {
                width: 1920.0.into(),
                height: 1280.0.into(),
            },
        )
    }
}
