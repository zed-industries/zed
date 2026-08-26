//! Persistent, host-rendered panels for extensions.

use crate::serde_json::Value;

pub use crate::wit::zed::extension::panel::{PanelDescriptor, PanelLocation};

/// Opens a persistent extension panel in the requested dock.
pub fn open_panel(descriptor: &PanelDescriptor) -> Result<(), String> {
    crate::wit::zed::extension::panel::open_panel(descriptor)
}

/// Delivers one JSON-object event to a persistent extension panel.
pub fn send_event(panel_id: &str, kind: &str, payload: &Value) -> Result<(), String> {
    if !payload.is_object() {
        return Err("extension panel event payload must be a JSON object".to_string());
    }

    let payload = crate::serde_json::to_string(payload).map_err(|error| error.to_string())?;
    crate::wit::zed::extension::panel::send_event(panel_id, kind, &payload)
}
