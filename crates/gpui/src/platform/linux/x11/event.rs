use x11rb::protocol::{
    xinput,
    xproto::{self, ModMask},
};

use crate::{Modifiers, MouseButton, NavigationDirection};

pub(crate) fn button_of_key(detail: xproto::Button) -> Option<MouseButton> {
    Some(match detail {
        1 => MouseButton::Left,
        2 => MouseButton::Middle,
        3 => MouseButton::Right,
        8 => MouseButton::Navigate(NavigationDirection::Back),
        9 => MouseButton::Navigate(NavigationDirection::Forward),
        _ => return None,
    })
}

pub(crate) fn modifiers_from_state(state: xproto::KeyButMask) -> Modifiers {
    Modifiers {
        control: state.contains(xproto::KeyButMask::CONTROL),
        alt: state.contains(xproto::KeyButMask::MOD1),
        shift: state.contains(xproto::KeyButMask::SHIFT),
        platform: state.contains(xproto::KeyButMask::MOD4),
        function: false,
    }
}

pub(crate) fn modifiers_from_xinput_info(modifier_info: xinput::ModifierInfo) -> Modifiers {
    Modifiers {
        control: modifier_info.effective as u16 & ModMask::CONTROL.bits()
            == ModMask::CONTROL.bits(),
        alt: modifier_info.effective as u16 & ModMask::M1.bits() == ModMask::M1.bits(),
        shift: modifier_info.effective as u16 & ModMask::SHIFT.bits() == ModMask::SHIFT.bits(),
        platform: modifier_info.effective as u16 & ModMask::M4.bits() == ModMask::M4.bits(),
        function: false,
    }
}

pub(crate) fn button_from_state(state: xproto::KeyButMask) -> Option<MouseButton> {
    Some(if state.contains(xproto::KeyButMask::BUTTON1) {
        MouseButton::Left
    } else if state.contains(xproto::KeyButMask::BUTTON2) {
        MouseButton::Middle
    } else if state.contains(xproto::KeyButMask::BUTTON3) {
        MouseButton::Right
    } else {
        return None;
    })
}
