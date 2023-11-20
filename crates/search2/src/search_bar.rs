use gpui::{MouseDownEvent, RenderOnce, WindowContext};
use ui::{Button, ButtonVariant, IconButton};

use crate::mode::SearchMode;

pub(super) fn render_nav_button(
    icon: ui::Icon,
    _active: bool,
    on_click: impl Fn(&MouseDownEvent, &mut WindowContext) + 'static,
) -> impl RenderOnce {
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
    on_click: impl Fn(&MouseDownEvent, &mut WindowContext) + 'static,
) -> Button {
    let button_variant = if is_active {
        ButtonVariant::Filled
    } else {
        ButtonVariant::Ghost
    };

    Button::new(mode.label())
        .on_click(on_click)
        .variant(button_variant)
}
