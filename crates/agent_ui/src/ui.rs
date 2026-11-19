mod agent_notification;
mod end_trial_upsell;
mod mention_crease;
mod sandbox_status_tooltip;
mod session_notice;
mod terminal_tool_header;
mod undo_reject_toast;

pub use agent_notification::*;
pub use end_trial_upsell::*;
pub use language_model_selector::{
    ModelSelectorFooter, ModelSelectorHeader, ModelSelectorListItem, ModelSelectorTooltip,
};
pub use mention_crease::*;
pub use sandbox_status_tooltip::*;
pub use session_notice::*;
pub use terminal_tool_header::*;
pub use undo_reject_toast::*;

use gpui::{App, ElementId, SharedString};
use ui::{ButtonLike, IconButton, TintColor, prelude::*};

pub(crate) fn fast_mode_button(id: impl Into<ElementId>, enabled: bool, cx: &App) -> IconButton {
    let (icon, color) = if enabled {
        (IconName::FastForward, Color::Accent)
    } else {
        (
            IconName::FastForwardOff,
            Color::Custom(cx.theme().colors().icon_disabled.opacity(0.8)),
        )
    };
    IconButton::new(id, icon)
        .icon_size(IconSize::Small)
        .icon_color(color)
}

pub(crate) fn thinking_effort_trigger(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    menu_open: bool,
    standalone: bool,
) -> ButtonLike {
    let (label_color, icon) = if menu_open {
        (Color::Accent, IconName::ChevronUp)
    } else {
        (Color::Muted, IconName::ChevronDown)
    };
    let label = Label::new(label).size(LabelSize::Small).color(label_color);
    let chevron = Icon::new(icon).size(IconSize::XSmall).color(Color::Muted);
    let trigger = if standalone {
        ButtonLike::new(id).child(
            h_flex()
                .gap_1()
                .child(
                    Icon::new(IconName::ThinkingMode)
                        .size(IconSize::Small)
                        .color(Color::Accent),
                )
                .child(label)
                .child(chevron),
        )
    } else {
        ButtonLike::new_rounded_right(id)
            .child(label)
            .child(chevron)
    };
    trigger.selected_style(ButtonStyle::Tinted(TintColor::Accent))
}

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
