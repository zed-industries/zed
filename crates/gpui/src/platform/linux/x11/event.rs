use x11rb::protocol::{
    xinput,
    xproto::{self, ModMask},
};

use crate::{Modifiers, MouseButton, NavigationDirection};

pub(crate) enum ButtonOrScroll {
    Button(MouseButton),
    Scroll(ScrollDirection),
}

pub(crate) enum ScrollDirection {
    Up,
    Down,
    Left,
    Right,
}

pub(crate) fn button_or_scroll_from_event_detail(detail: u32) -> Option<ButtonOrScroll> {
    Some(match detail {
        1 => ButtonOrScroll::Button(MouseButton::Left),
        2 => ButtonOrScroll::Button(MouseButton::Middle),
        3 => ButtonOrScroll::Button(MouseButton::Right),
        4 => ButtonOrScroll::Scroll(ScrollDirection::Up),
        5 => ButtonOrScroll::Scroll(ScrollDirection::Down),
        6 => ButtonOrScroll::Scroll(ScrollDirection::Left),
        7 => ButtonOrScroll::Scroll(ScrollDirection::Right),
        8 => ButtonOrScroll::Button(MouseButton::Navigate(NavigationDirection::Back)),
        9 => ButtonOrScroll::Button(MouseButton::Navigate(NavigationDirection::Forward)),
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

pub(crate) fn pressed_button_from_mask(button_mask: u32) -> Option<MouseButton> {
    Some(if button_mask & 2 == 2 {
        MouseButton::Left
    } else if button_mask & 4 == 4 {
        MouseButton::Middle
    } else if button_mask & 8 == 8 {
        MouseButton::Right
    } else {
        return None;
    })
}
