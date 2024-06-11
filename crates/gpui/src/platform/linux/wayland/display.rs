use std::{
    fmt::Debug,
    hash::{Hash, Hasher},
};

use uuid::Uuid;
use wayland_backend::client::ObjectId;

use crate::{Bounds, DisplayId, Pixels, PlatformDisplay};

#[derive(Debug, Clone)]
pub(crate) struct WaylandDisplay {
    /// The ID of the wl_output object
    pub id: ObjectId,
    pub name: Option<String>,
    pub bounds: Bounds<Pixels>,
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
        if let Some(name) = &self.name {
            Ok(Uuid::new_v5(&Uuid::NAMESPACE_DNS, name.as_bytes()))
        } else {
            Err(anyhow::anyhow!("Wayland display does not have a name"))
        }
    }

    fn bounds(&self) -> Bounds<Pixels> {
        self.bounds
    }
}
