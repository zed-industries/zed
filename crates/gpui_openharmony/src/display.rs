use gpui::{Bounds, DisplayId, Pixels, PlatformDisplay, Size, point, px};
use std::cell::Cell;
use std::fmt;
use uuid::Uuid;

pub(crate) struct OpenHarmonyDisplay {
    id: DisplayId,
    bounds: Cell<Bounds<Pixels>>,
    scale_factor: Cell<f32>,
}

impl OpenHarmonyDisplay {
    pub fn new(id: DisplayId, size: Size<Pixels>, scale_factor: f32) -> Self {
        Self {
            id,
            bounds: Cell::new(Bounds::new(point(px(0.), px(0.)), size)),
            scale_factor: Cell::new(scale_factor),
        }
    }

    pub fn set_size(&self, size: Size<Pixels>) {
        let mut bounds = self.bounds.get();
        bounds.size = size;
        self.bounds.set(bounds);
    }

    pub fn set_scale_factor(&self, scale_factor: f32) {
        self.scale_factor.set(scale_factor);
    }

    pub fn scale_factor(&self) -> f32 {
        self.scale_factor.get()
    }
}

impl fmt::Debug for OpenHarmonyDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OpenHarmonyDisplay")
            .field("id", &self.id)
            .field("bounds", &self.bounds.get())
            .finish()
    }
}

impl PlatformDisplay for OpenHarmonyDisplay {
    fn id(&self) -> DisplayId {
        self.id
    }

    fn uuid(&self) -> anyhow::Result<Uuid> {
        Ok(Uuid::from_u64_pair(u64::from(self.id), 0))
    }

    fn bounds(&self) -> Bounds<Pixels> {
        self.bounds.get()
    }

    fn visible_bounds(&self) -> Bounds<Pixels> {
        self.bounds.get()
    }
}
