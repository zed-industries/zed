use std::fmt::Debug;

use uuid::Uuid;

use crate::{Bounds, DisplayId, GlobalPixels, PlatformDisplay, Size};

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
    fn bounds(&self) -> Bounds<GlobalPixels> {
        Bounds {
            origin: Default::default(),
            size: Size {
                width: GlobalPixels(1000f32),
                height: GlobalPixels(500f32),
            },
        } // return some fake data so it doesn't panic
    }
}
