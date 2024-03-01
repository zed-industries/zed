use std::rc::Rc;

use anyhow::Result;
use uuid::Uuid;

use crate::{available_monitors, Bounds, DisplayId, GlobalPixels, PlatformDisplay, Size};

#[derive(Debug)]
pub(crate) struct WindowsDisplay {
    pub display_id: DisplayId,
    bounds: Bounds<GlobalPixels>,
    uuid: Uuid,
}

impl WindowsDisplay {
    pub(crate) fn new(display_id: DisplayId) -> Self {
        let screen = available_monitors()
            .into_iter()
            .nth(display_id.0 as _)
            .unwrap();

        Self {
            display_id,
            bounds: Bounds {
                origin: Default::default(),
                size: Size {
                    width: GlobalPixels(screen.size().width as f32),
                    height: GlobalPixels(screen.size().height as f32),
                },
            },
            uuid: Uuid::from_bytes([0; 16]),
        }
    }

    pub fn displays() -> Vec<Rc<dyn PlatformDisplay>> {
        available_monitors()
            .into_iter()
            .enumerate()
            .map(|(id, _)| {
                Rc::new(WindowsDisplay::new(DisplayId(id as _))) as Rc<dyn PlatformDisplay>
            })
            .collect()
    }
}

impl PlatformDisplay for WindowsDisplay {
    fn id(&self) -> DisplayId {
        self.display_id
    }

    fn uuid(&self) -> Result<Uuid> {
        Ok(self.uuid)
    }

    fn bounds(&self) -> Bounds<GlobalPixels> {
        self.bounds
    }
}
