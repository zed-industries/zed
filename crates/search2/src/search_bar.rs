use gpui::{ClickEvent, IntoElement, WindowContext};
use ui::prelude::*;
use ui::{ButtonVariant, IconButton, OldButton};

use crate::mode::SearchMode;

pub(super) fn render_nav_button(
    icon: ui::Icon,
    _active: bool,
    on_click: impl Fn(&ClickEvent, &mut WindowContext) + 'static,
) -> impl IntoElement {
    // let tooltip_style = cx.theme().tooltip.clone();
    // let cursor_style = if active {
    //     CursorStyle::PointingHand
    // } else {
    //     CursorStyle::default()
    // };
    // enum NavButton {}
    IconButton::new("search-nav-button", icon).on_click(on_click)
}

pub(crate) fn render_search_mode_button(
    mode: SearchMode,
    is_active: bool,
    on_click: impl Fn(&ClickEvent, &mut WindowContext) + 'static,
) -> OldButton {
    let button_variant = if is_active {
        ButtonVariant::Filled
    } else {
        ButtonVariant::Ghost
    };

    OldButton::new(mode.label())
        .on_click(on_click)
        .variant(button_variant)
}
