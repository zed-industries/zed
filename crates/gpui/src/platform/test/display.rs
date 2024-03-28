use anyhow::{Ok, Result};

use crate::{Bounds, DevicePixels, DisplayId, PlatformDisplay, Point};

#[derive(Debug)]
pub(crate) struct TestDisplay {
    id: DisplayId,
    uuid: uuid::Uuid,
    bounds: Bounds<DevicePixels>,
}

impl TestDisplay {
    pub fn new() -> Self {
        TestDisplay {
            id: DisplayId(1),
            uuid: uuid::Uuid::new_v4(),
            bounds: Bounds::from_corners(
                Point::default(),
                Point::new(DevicePixels(1920), DevicePixels(1080)),
            ),
        }
    }
}

impl PlatformDisplay for TestDisplay {
    fn id(&self) -> crate::DisplayId {
        self.id
    }

    fn uuid(&self) -> Result<uuid::Uuid> {
        Ok(self.uuid)
    }

    fn bounds(&self) -> crate::Bounds<crate::DevicePixels> {
        self.bounds
    }
}
