use std::sync::Arc;

use gpui::{RenderOnce, ViewContext};
use ui::{Button, ButtonVariant, IconButton};

use crate::mode::SearchMode;

pub(super) fn render_nav_button<V: 'static>(
    icon: ui::Icon,
    _active: bool,
    on_click: impl Fn(&mut V, &mut ViewContext<V>) + 'static + Send + Sync,
) -> impl RenderOnce<V> {
    // let tooltip_style = cx.theme().tooltip.clone();
    // let cursor_style = if active {
    //     CursorStyle::PointingHand
    // } else {
    //     CursorStyle::default()
    // };
    // enum NavButton {}
    IconButton::new("search-nav-button", icon).on_click(on_click)
}

pub(crate) fn render_search_mode_button<V: 'static>(
    mode: SearchMode,
    is_active: bool,
    on_click: impl Fn(&mut V, &mut ViewContext<V>) + 'static + Send + Sync,
) -> Button<V> {
    let button_variant = if is_active {
        ButtonVariant::Filled
    } else {
        ButtonVariant::Ghost
    };

    Button::new(mode.label())
        .on_click(Arc::new(on_click))
        .variant(button_variant)
}
