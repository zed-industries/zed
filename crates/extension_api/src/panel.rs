//! Persistent, host-rendered panels for extensions.

use crate::serde_json::Value;

pub use crate::wit::zed::extension::panel::{PanelAction, PanelDescriptor, PanelLocation};

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

/// Returns the root path of the active workspace's first visible worktree.
pub fn active_worktree_root() -> Result<String, String> {
    crate::wit::zed::extension::panel::active_worktree_root()
}

/// Reads one relative file from the active workspace's visible worktree.
pub fn read_active_worktree_file(path: &str) -> Result<String, String> {
    crate::wit::zed::extension::panel::read_active_worktree_file(path)
}
