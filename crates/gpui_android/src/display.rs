use anyhow::Result;
use gpui::{Bounds, DisplayId, Pixels, PlatformDisplay, Point, Size, px};
use std::cell::Cell;

#[derive(Debug)]
pub struct AndroidDisplay {
    id: DisplayId,
    uuid: uuid::Uuid,
    size: Cell<Size<Pixels>>,
}

// Safety: the display is only accessed from the android_main thread; GPUI's
// PlatformDisplay trait requires Send + Sync but never shares it across threads.
unsafe impl Send for AndroidDisplay {}
unsafe impl Sync for AndroidDisplay {}

impl AndroidDisplay {
    pub fn new() -> Self {
        AndroidDisplay {
            id: DisplayId::new(1),
            uuid: uuid::Uuid::new_v4(),
            size: Cell::new(Size {
                width: px(412.),
                height: px(915.),
            }),
        }
    }

    pub(crate) fn set_size(&self, size: Size<Pixels>) {
        self.size.set(size);
    }
}

impl PlatformDisplay for AndroidDisplay {
    fn id(&self) -> DisplayId {
        self.id
    }

    fn uuid(&self) -> Result<uuid::Uuid> {
        Ok(self.uuid)
    }

    fn bounds(&self) -> Bounds<Pixels> {
        Bounds {
            origin: Point::default(),
            size: self.size.get(),
        }
    }
}
