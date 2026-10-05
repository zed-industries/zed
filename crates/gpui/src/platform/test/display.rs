use crate::{Bounds, DisplayId, DisplayPower, DisplayState, Pixels, PlatformDisplay, Point, px};
use anyhow::{Ok, Result};
use std::time::Duration;

#[derive(Debug)]
pub(crate) struct TestDisplay {
    id: DisplayId,
    uuid: uuid::Uuid,
    bounds: Bounds<Pixels>,
    state: DisplayState,
}

impl TestDisplay {
    pub fn new() -> Self {
        TestDisplay {
            id: DisplayId(1),
            uuid: uuid::Uuid::new_v4(),
            bounds: Bounds::from_corners(Point::default(), Point::new(px(1920.), px(1080.))),
            state: DisplayState {
                refresh_interval: Some(Duration::from_secs(1) / 60),
                power: DisplayPower::On,
            },
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

    fn bounds(&self) -> crate::Bounds<crate::Pixels> {
        self.bounds
    }

    fn state(&self) -> DisplayState {
        self.state
    }
}
