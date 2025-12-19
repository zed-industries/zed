//! Settings for the Ali Panel (simplified for MVP)

use gpui::{App, Pixels};
use ui::px;
use workspace::dock::DockPosition;

/// Settings for the Ali Panel
pub struct AliPanelSettings {
    pub button: bool,
    pub dock: DockPosition,
    pub default_height: Pixels,
}

impl AliPanelSettings {
    pub fn get_global(_cx: &App) -> Self {
        Self::default()
    }

    pub fn register(_cx: &mut App) {
        // Settings registration will be added later
    }
}

impl Default for AliPanelSettings {
    fn default() -> Self {
        Self {
            button: true,
            dock: DockPosition::Bottom,
            default_height: px(200.),
        }
    }
}
