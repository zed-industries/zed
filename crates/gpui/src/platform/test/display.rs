use anyhow::{Ok, Result};

use crate::{Bounds, DisplayId, GlobalPixels, PlatformDisplay, Point};

#[derive(Debug)]
pub(crate) struct TestDisplay {
    id: DisplayId,
    uuid: uuid::Uuid,
    bounds: Bounds<GlobalPixels>,
}

impl TestDisplay {
    pub fn new() -> Self {
        TestDisplay {
            id: DisplayId(1),
            uuid: uuid::Uuid::new_v4(),
            bounds: Bounds::from_corners(
                Point::default(),
                Point::new(GlobalPixels(1920.), GlobalPixels(1080.)),
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

    fn bounds(&self) -> crate::Bounds<crate::GlobalPixels> {
        self.bounds
    }
}
