//! Screen capture vocabulary shared by `gpui` and its platform backends.

use gpui_shared_string::SharedString;
use gpui_types::{DevicePixels, Size};

/// Metadata for a given screen capture source
#[derive(Clone)]
pub struct SourceMetadata {
    /// Opaque identifier of this screen.
    pub id: u64,
    /// Human-readable label for this source.
    pub label: Option<SharedString>,
    /// Whether this source is the main display.
    pub is_main: Option<bool>,
    /// Video resolution of this source.
    pub resolution: Size<DevicePixels>,
}
