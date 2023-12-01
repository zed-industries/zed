use gpui::{div, ClickEvent, Element, IntoElement, IntoListener, ParentElement};

use crate::{Color, Icon, IconButton, IconSize, Toggle};

pub fn disclosure_control(
    toggle: Toggle,
    on_toggle: Option<impl IntoListener<ClickEvent>>,
) -> impl Element {
    match (toggle.is_toggleable(), toggle.is_toggled()) {
        (false, _) => div(),
        (_, true) => div().child(
            IconButton::new("toggle", Icon::ChevronDown)
                .color(Color::Muted)
                .size(IconSize::Small)
                .when_some(on_toggle, move |el, on_toggle| {
                    el.on_click(on_toggle.into_listener())
                }),
        ),
        (_, false) => div().child(
            IconButton::new("toggle", Icon::ChevronRight)
                .color(Color::Muted)
                .size(IconSize::Small)
                .when_some(on_toggle, move |el, on_toggle| {
                    el.on_click(on_toggle.into_listener())
                }),
        ),
    }
}
