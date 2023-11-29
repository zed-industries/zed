use gpui::{ClickEvent, IntoElement, WindowContext};
use ui::prelude::*;
use ui::{Button, IconButton};

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
) -> Button {
    Button::new(mode.label(), mode.label())
        .selected(is_active)
        .on_click(on_click)
}
