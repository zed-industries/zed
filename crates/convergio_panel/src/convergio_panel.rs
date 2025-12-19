//! Convergio Panel - Multi-agent panel for Convergio AI assistants
//!
//! This panel displays all available Convergio agents and allows
//! starting conversations with each one.

mod panel;
mod settings;

pub use panel::ConvergioPanel;
pub use settings::ConvergioPanelSettings;

use gpui::App;

pub fn init(cx: &mut App) {
    settings::init(cx);
    panel::init(cx);
}
