use gpui::{Action, FocusHandle, IntoElement};
use ui::{prelude::*, Tooltip};
use ui::{IconButton, IconButtonShape};

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
    .shape(IconButtonShape::Square)
    .on_click(|_, window, cx| window.dispatch_action(action.boxed_clone(), cx))
    .tooltip(move |window, cx| Tooltip::for_action_in(tooltip, action, &focus_handle, window, cx))
    .disabled(!active)
}
