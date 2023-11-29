use std::rc::Rc;

use gpui::{div, ClickEvent, Element, IntoElement, ParentElement, WindowContext};

use crate::{Color, Icon, IconButton, IconSize, Toggle};

pub fn disclosure_control(
    toggle: Toggle,
    on_toggle: Option<Rc<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>>,
) -> impl Element {
    match (toggle.is_toggleable(), toggle.is_toggled()) {
        (false, _) => div(),
        (_, true) => div().child(
            IconButton::new("toggle", Icon::ChevronDown)
                .color(Color::Muted)
                .size(IconSize::Small)
                .when_some(on_toggle, move |el, on_toggle| {
                    el.on_click(move |e, cx| on_toggle(e, cx))
                }),
        ),
        (_, false) => div().child(
            IconButton::new("toggle", Icon::ChevronRight)
                .color(Color::Muted)
                .size(IconSize::Small)
                .when_some(on_toggle, move |el, on_toggle| {
                    el.on_click(move |e, cx| on_toggle(e, cx))
                }),
        ),
    }
}
