//! Settings for the Convergio Panel (simplified for MVP)

use gpui::{App, Pixels};
use ui::px;
use workspace::dock::DockPosition;

/// Settings for the Convergio Panel
pub struct ConvergioPanelSettings {
    pub button: bool,
    pub dock: DockPosition,
    pub default_width: Pixels,
}

impl ConvergioPanelSettings {
    pub fn get_global(_cx: &App) -> Self {
        Self::default()
    }
}

impl Default for ConvergioPanelSettings {
    fn default() -> Self {
        Self {
            button: true,
            dock: DockPosition::Left,
            default_width: px(260.),
        }
    }
}

pub fn init(_cx: &mut App) {
    // Settings registration will be added later
}
