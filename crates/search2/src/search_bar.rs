use gpui::{Action, IntoElement};
use ui::{prelude::*, Tooltip};
use ui::{Button, IconButton};

use crate::mode::SearchMode;

pub(super) fn render_nav_button(
    icon: ui::Icon,
    active: bool,
    tooltip: &'static str,
    action: &'static dyn Action,
) -> impl IntoElement {
    IconButton::new(
        SharedString::from(format!("search-nav-button-{}", action.name())),
        icon,
    )
    .on_click(|_, cx| cx.dispatch_action(action.boxed_clone()))
    .tooltip(move |cx| Tooltip::for_action(tooltip, action, cx))
    .disabled(!active)
}

pub(crate) fn render_search_mode_button(mode: SearchMode, is_active: bool) -> Button {
    Button::new(mode.label(), mode.label())
        .selected(is_active)
        .on_click({
            let action = mode.action();
            move |_, cx| {
                cx.dispatch_action(action.boxed_clone());
            }
        })
        .tooltip({
            let action = mode.action();
            let tooltip_text = mode.tooltip();
            move |cx| Tooltip::for_action(tooltip_text.clone(), &*action, cx)
        })
}
