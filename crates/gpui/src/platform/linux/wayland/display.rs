use std::fmt::Debug;

use uuid::Uuid;

use crate::{Bounds, DevicePixels, DisplayId, PlatformDisplay, Size};

#[derive(Debug)]
pub(crate) struct WaylandDisplay {}

impl PlatformDisplay for WaylandDisplay {
    // todo(linux)
    fn id(&self) -> DisplayId {
        DisplayId(123) // return some fake data so it doesn't panic
    }

    // todo(linux)
    fn uuid(&self) -> anyhow::Result<Uuid> {
        Ok(Uuid::from_bytes([0; 16])) // return some fake data so it doesn't panic
    }

    // todo(linux)
    fn bounds(&self) -> Bounds<DevicePixels> {
        Bounds {
            origin: Default::default(),
            size: Size {
                width: DevicePixels(1000),
                height: DevicePixels(500),
            },
        } // return some fake data so it doesn't panic
    }
}
