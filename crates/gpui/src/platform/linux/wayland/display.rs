use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use uuid::Uuid;
use wayland_backend::client::ObjectId;

use crate::{Bounds, DevicePixels, DisplayId, PlatformDisplay};

#[derive(Debug, Clone)]
pub(crate) struct WaylandDisplay {
    /// The ID of the wl_output object
    pub id: ObjectId,
    pub bounds: Bounds<DevicePixels>,
}

impl Hash for WaylandDisplay {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PlatformDisplay for WaylandDisplay {
    fn id(&self) -> DisplayId {
        DisplayId(self.id.protocol_id())
    }

    fn uuid(&self) -> anyhow::Result<Uuid> {
        Err(anyhow::anyhow!("Display UUID is not supported on Wayland"))
    }

    fn bounds(&self) -> Bounds<DevicePixels> {
        self.bounds
    }
}
