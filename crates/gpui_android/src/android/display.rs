use gpui::{Bounds, DisplayId, Pixels, PlatformDisplay, Point, Size, px};
use parking_lot::RwLock;

/// Default fallback bounds when no real display metrics have been published
/// from the JNI side. Roughly a 1080p phone in portrait at scale 1.
const FALLBACK_BOUNDS: Bounds<Pixels> = Bounds {
    origin: Point {
        x: px(0.),
        y: px(0.),
    },
    size: Size {
        width: px(1080.),
        height: px(2400.),
    },
};

#[derive(Debug)]
pub(crate) struct AndroidDisplay {
    id: DisplayId,
    uuid: uuid::Uuid,
    bounds: RwLock<Bounds<Pixels>>,
}

impl AndroidDisplay {
    pub(crate) fn new() -> Self {
        Self {
            id: DisplayId::new(1),
            // Android does not have a stable cross-process display UUID, so we
            // generate one per-process. Display identity within a process is
            // stable; that's all GPUI relies on today.
            uuid: uuid::Uuid::new_v4(),
            bounds: RwLock::new(FALLBACK_BOUNDS),
        }
    }

    /// Update the cached display bounds. Wired up to the JNI surface-changed
    /// callback once the bridge is in place.
    #[allow(dead_code)]
    pub(crate) fn set_bounds(&self, bounds: Bounds<Pixels>) {
        *self.bounds.write() = bounds;
    }
}

impl PlatformDisplay for AndroidDisplay {
    fn id(&self) -> DisplayId {
        self.id
    }

    fn uuid(&self) -> anyhow::Result<uuid::Uuid> {
        Ok(self.uuid)
    }

    fn bounds(&self) -> Bounds<Pixels> {
        *self.bounds.read()
    }
}
