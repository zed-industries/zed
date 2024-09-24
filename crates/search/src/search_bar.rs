use gpui::{Action, FocusHandle, IntoElement};
use ui::IconButton;
use ui::{prelude::*, Tooltip};

pub(super) fn render_nav_button(
    icon: ui::IconName,
    active: bool,
    tooltip: &'static str,
    action: &'static dyn Action,
    focus_handle: FocusHandle,
) -> impl IntoElement {
    IconButton::new(
        SharedString::from(format!("search-nav-button-{}", action.name())),
        icon,
    )
    .on_click(|_, cx| cx.dispatch_action(action.boxed_clone()))
    .tooltip(move |cx| Tooltip::for_action_in(tooltip, action, &focus_handle, cx))
    .disabled(!active)
}
