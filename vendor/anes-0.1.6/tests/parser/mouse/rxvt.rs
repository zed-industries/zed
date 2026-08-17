use anes::parser::{KeyModifiers, Mouse, MouseButton, Sequence};

use crate::test_sequences;

#[test]
fn button_down() {
    test_sequences!(
        b"\x1B[0;30;40;M",
        Sequence::Mouse(
            Mouse::Down(MouseButton::Left, 30, 40),
            KeyModifiers::empty()
        ),
        b"\x1B[1;30;40;M",
        Sequence::Mouse(
            Mouse::Down(MouseButton::Middle, 30, 40),
            KeyModifiers::empty()
        ),
        b"\x1B[2;30;40;M",
        Sequence::Mouse(
            Mouse::Down(MouseButton::Right, 30, 40),
            KeyModifiers::empty()
        ),
    );
}

#[test]
fn button_down_with_modifiers() {
    test_sequences!(
        b"\x1B[4;30;40;M",
        Sequence::Mouse(Mouse::Down(MouseButton::Left, 30, 40), KeyModifiers::SHIFT),
        b"\x1B[5;30;40;M",
        Sequence::Mouse(
            Mouse::Down(MouseButton::Middle, 30, 40),
            KeyModifiers::SHIFT
        ),
        b"\x1B[6;30;40;M",
        Sequence::Mouse(Mouse::Down(MouseButton::Right, 30, 40), KeyModifiers::SHIFT),
    );
}

#[test]
fn button_up() {
    test_sequences!(
        b"\x1B[3;30;40;M",
        Sequence::Mouse(Mouse::Up(MouseButton::Any, 30, 40), KeyModifiers::empty()),
    );
}

#[test]
fn button_up_with_modifiers() {
    test_sequences!(
        b"\x1B[7;30;40;M",
        Sequence::Mouse(Mouse::Up(MouseButton::Any, 30, 40), KeyModifiers::SHIFT),
    );
}

#[test]
fn scroll() {
    test_sequences!(
        b"\x1B[96;30;40;M",
        Sequence::Mouse(Mouse::ScrollUp(30, 40), KeyModifiers::empty()),
        b"\x1B[97;30;40;M",
        Sequence::Mouse(Mouse::ScrollDown(30, 40), KeyModifiers::empty()),
    );
}

#[test]
fn scroll_with_modifiers() {
    test_sequences!(
        b"\x1B[100;30;40;M",
        Sequence::Mouse(Mouse::ScrollUp(30, 40), KeyModifiers::SHIFT),
        b"\x1B[101;30;40;M",
        Sequence::Mouse(Mouse::ScrollDown(30, 40), KeyModifiers::SHIFT),
    );
}

#[test]
fn drag() {
    test_sequences!(
        b"\x1B[64;30;40;M",
        Sequence::Mouse(
            Mouse::Drag(MouseButton::Left, 30, 40),
            KeyModifiers::empty()
        ),
        b"\x1B[65;30;40;M",
        Sequence::Mouse(
            Mouse::Drag(MouseButton::Middle, 30, 40),
            KeyModifiers::empty()
        ),
        b"\x1B[66;30;40;M",
        Sequence::Mouse(
            Mouse::Drag(MouseButton::Right, 30, 40),
            KeyModifiers::empty()
        ),
    );
}

#[test]
fn drag_with_modifiers() {
    test_sequences!(
        b"\x1B[64;30;40;M",
        Sequence::Mouse(
            Mouse::Drag(MouseButton::Left, 30, 40),
            KeyModifiers::empty()
        ),
        b"\x1B[65;30;40;M",
        Sequence::Mouse(
            Mouse::Drag(MouseButton::Middle, 30, 40),
            KeyModifiers::empty()
        ),
        b"\x1B[66;30;40;M",
        Sequence::Mouse(
            Mouse::Drag(MouseButton::Right, 30, 40),
            KeyModifiers::empty()
        ),
    );
}

#[test]
fn key_modifier_combinations() {
    test_sequences!(
        b"\x1B[4;20;10M",
        Sequence::Mouse(Mouse::Down(MouseButton::Left, 20, 10), KeyModifiers::SHIFT),
        b"\x1B[8;20;10M",
        Sequence::Mouse(Mouse::Down(MouseButton::Left, 20, 10), KeyModifiers::ALT),
        b"\x1B[16;20;10M",
        Sequence::Mouse(
            Mouse::Down(MouseButton::Left, 20, 10),
            KeyModifiers::CONTROL
        ),
        b"\x1B[12;20;10;M",
        Sequence::Mouse(
            Mouse::Down(MouseButton::Left, 20, 10),
            KeyModifiers::SHIFT | KeyModifiers::ALT
        ),
        b"\x1B[20;20;10;M",
        Sequence::Mouse(
            Mouse::Down(MouseButton::Left, 20, 10),
            KeyModifiers::SHIFT | KeyModifiers::CONTROL
        ),
        b"\x1B[24;20;10;M",
        Sequence::Mouse(
            Mouse::Down(MouseButton::Left, 20, 10),
            KeyModifiers::ALT | KeyModifiers::CONTROL
        ),
        b"\x1B[28;20;10;M",
        Sequence::Mouse(
            Mouse::Down(MouseButton::Left, 20, 10),
            KeyModifiers::SHIFT | KeyModifiers::ALT | KeyModifiers::CONTROL
        ),
    );
}
