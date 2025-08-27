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

pub(crate) fn get_valuator_axis_index(
    valuator_mask: &Vec<u32>,
    valuator_number: u16,
) -> Option<usize> {
    // XInput valuator masks have a 1 at the bit indexes corresponding to each
    // valuator present in this event's axisvalues. Axisvalues is ordered from
    // lowest valuator number to highest, so counting bits before the 1 bit for
    // this valuator yields the index in axisvalues.
    if bit_is_set_in_vec(valuator_mask, valuator_number) {
        Some(popcount_upto_bit_index(valuator_mask, valuator_number) as usize)
    } else {
        None
    }
}

/// Returns the number of 1 bits in `bit_vec` for all bits where `i < bit_index`.
fn popcount_upto_bit_index(bit_vec: &Vec<u32>, bit_index: u16) -> u32 {
    let array_index = bit_index as usize / 32;
    let popcount: u32 = bit_vec
        .get(array_index)
        .map_or(0, |bits| keep_bits_upto(*bits, bit_index % 32).count_ones());
    if array_index == 0 {
        popcount
    } else {
        // Valuator numbers over 32 probably never occur for scroll position, but may as well
        // support it.
        let leading_popcount: u32 = bit_vec
            .iter()
            .take(array_index)
            .map(|bits| bits.count_ones())
            .sum();
        popcount + leading_popcount
    }
}

fn bit_is_set_in_vec(bit_vec: &Vec<u32>, bit_index: u16) -> bool {
    let array_index = bit_index as usize / 32;
    bit_vec
        .get(array_index)
        .is_some_and(|bits| bit_is_set(*bits, bit_index % 32))
}

fn bit_is_set(bits: u32, bit_index: u16) -> bool {
    bits & (1 << bit_index) != 0
}

/// Sets every bit with `i >= bit_index` to 0.
fn keep_bits_upto(bits: u32, bit_index: u16) -> u32 {
    if bit_index == 0 {
        0
    } else if bit_index >= 32 {
        u32::MAX
    } else {
        bits & ((1 << bit_index) - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_valuator_axis_index() {
        assert!(get_valuator_axis_index(&vec![0b11], 0) == Some(0));
        assert!(get_valuator_axis_index(&vec![0b11], 1) == Some(1));
        assert!(get_valuator_axis_index(&vec![0b11], 2) == None);

        assert!(get_valuator_axis_index(&vec![0b100], 0) == None);
        assert!(get_valuator_axis_index(&vec![0b100], 1) == None);
        assert!(get_valuator_axis_index(&vec![0b100], 2) == Some(0));
        assert!(get_valuator_axis_index(&vec![0b100], 3) == None);

        assert!(get_valuator_axis_index(&vec![0b1010, 0], 0) == None);
        assert!(get_valuator_axis_index(&vec![0b1010, 0], 1) == Some(0));
        assert!(get_valuator_axis_index(&vec![0b1010, 0], 2) == None);
        assert!(get_valuator_axis_index(&vec![0b1010, 0], 3) == Some(1));

        assert!(get_valuator_axis_index(&vec![0b1010, 0b1], 0) == None);
        assert!(get_valuator_axis_index(&vec![0b1010, 0b1], 1) == Some(0));
        assert!(get_valuator_axis_index(&vec![0b1010, 0b1], 2) == None);
        assert!(get_valuator_axis_index(&vec![0b1010, 0b1], 3) == Some(1));
        assert!(get_valuator_axis_index(&vec![0b1010, 0b1], 32) == Some(2));
        assert!(get_valuator_axis_index(&vec![0b1010, 0b1], 33) == None);

        assert!(get_valuator_axis_index(&vec![0b1010, 0b101], 34) == Some(3));
    }
}
