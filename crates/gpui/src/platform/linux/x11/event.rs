use xcb::x;

use crate::{Modifiers, MouseButton, NavigationDirection};

pub(crate) fn button_of_key(detail: x::Button) -> Option<MouseButton> {
    Some(match detail {
        1 => MouseButton::Left,
        2 => MouseButton::Middle,
        3 => MouseButton::Right,
        8 => MouseButton::Navigate(NavigationDirection::Back),
        9 => MouseButton::Navigate(NavigationDirection::Forward),
        _ => return None,
    })
}

pub(crate) fn modifiers_from_state(state: x::KeyButMask) -> Modifiers {
    Modifiers {
        control: state.contains(x::KeyButMask::CONTROL),
        alt: state.contains(x::KeyButMask::MOD1),
        shift: state.contains(x::KeyButMask::SHIFT),
        command: state.contains(x::KeyButMask::MOD4),
        function: false,
    }
}

pub(crate) fn button_from_state(state: x::KeyButMask) -> Option<MouseButton> {
    Some(if state.contains(x::KeyButMask::BUTTON1) {
        MouseButton::Left
    } else if state.contains(x::KeyButMask::BUTTON2) {
        MouseButton::Middle
    } else if state.contains(x::KeyButMask::BUTTON3) {
        MouseButton::Right
    } else {
        return None;
    })
}
