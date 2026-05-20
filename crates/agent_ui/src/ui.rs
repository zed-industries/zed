mod agent_notification;
mod end_trial_upsell;
mod hold_for_default;
mod mention_crease;
mod model_selector_components;
mod undo_reject_toast;

pub use agent_notification::*;
pub use end_trial_upsell::*;
pub use hold_for_default::*;
pub use mention_crease::*;
pub use model_selector_components::*;
pub use undo_reject_toast::*;

/// Returns the appropriate [`DocumentationSide`] for documentation asides
/// in the agent panel, based on the current dock position.
pub fn documentation_aside_side(cx: &gpui::App) -> ui::DocumentationSide {
    use agent_settings::AgentSettings;
    use settings::Settings;
    use ui::DocumentationSide;

    match AgentSettings::get_global(cx).dock {
        settings::DockPosition::Left => DocumentationSide::Right,
        settings::DockPosition::Bottom | settings::DockPosition::Right => DocumentationSide::Left,
    }
}
