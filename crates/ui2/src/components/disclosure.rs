use gpui::{div, Element, ParentElement};

use crate::{Color, Icon, IconElement, IconSize, Toggle};

pub fn disclosure_control(toggle: Toggle) -> impl Element {
    match (toggle.is_toggleable(), toggle.is_toggled()) {
        (false, _) => div(),
        (_, true) => div().child(
            IconElement::new(Icon::ChevronDown)
                .color(Color::Muted)
                .size(IconSize::Small),
        ),
        (_, false) => div().child(
            IconElement::new(Icon::ChevronRight)
                .color(Color::Muted)
                .size(IconSize::Small),
        ),
    }
}
