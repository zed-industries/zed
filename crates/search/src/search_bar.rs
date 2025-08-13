use gpui::{Action, FocusHandle, Hsla, IntoElement};
use ui::{IconButton, IconButtonShape};
use ui::{Tooltip, prelude::*};

use crate::ToggleReplace;

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
    .on_click({
        let focus_handle = focus_handle.clone();
        move |_, window, cx| {
            if !focus_handle.is_focused(&window) {
                window.focus(&focus_handle);
            }
            window.dispatch_action(action.boxed_clone(), cx)
        }
    })
    .tooltip(move |window, cx| Tooltip::for_action_in(tooltip, action, &focus_handle, window, cx))
    .disabled(!active)
}

pub(crate) fn input_base_styles(border_color: Hsla, map: impl FnOnce(Div) -> Div) -> Div {
    h_flex()
        .min_w_32()
        .map(map)
        .h_8()
        .pl_2()
        .pr_1()
        .py_1()
        .border_1()
        .border_color(border_color)
        .rounded_lg()
}

pub(crate) fn toggle_replace_button(
    id: &'static str,
    focus_handle: FocusHandle,
    replace_enabled: bool,
    on_click: impl Fn(&gpui::ClickEvent, &mut Window, &mut App) + 'static,
) -> IconButton {
    IconButton::new(id, IconName::Replace)
        .shape(IconButtonShape::Square)
        .style(ButtonStyle::Subtle)
        .when(replace_enabled, |button| button.style(ButtonStyle::Filled))
        .on_click(on_click)
        .toggle_state(replace_enabled)
        .tooltip({
            move |window, cx| {
                Tooltip::for_action_in("Toggle Replace", &ToggleReplace, &focus_handle, window, cx)
            }
        })
}
