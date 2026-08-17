use anes::parser::{KeyModifiers, Mouse, MouseButton, Sequence};

use crate::test_sequences;

#[test]
fn button_down() {
    test_sequences!(
        b"\x1B[<0;20;10;M",
        Sequence::Mouse(
            Mouse::Down(MouseButton::Left, 20, 10),
            KeyModifiers::empty()
        ),
        b"\x1B[<1;20;10;M",
        Sequence::Mouse(
            Mouse::Down(MouseButton::Middle, 20, 10),
            KeyModifiers::empty()
        ),
        b"\x1B[<2;20;10;M",
        Sequence::Mouse(
            Mouse::Down(MouseButton::Right, 20, 10),
            KeyModifiers::empty()
        ),
    );
}

#[test]
fn button_down_with_key_modifiers() {
    test_sequences!(
        b"\x1B[<4;20;10;M",
        Sequence::Mouse(Mouse::Down(MouseButton::Left, 20, 10), KeyModifiers::SHIFT),
        b"\x1B[<5;20;10;M",
        Sequence::Mouse(
            Mouse::Down(MouseButton::Middle, 20, 10),
            KeyModifiers::SHIFT
        ),
        b"\x1B[<6;20;10;M",
        Sequence::Mouse(Mouse::Down(MouseButton::Right, 20, 10), KeyModifiers::SHIFT),
    );
}

#[test]
fn button_up() {
    test_sequences!(
        b"\x1B[<0;20;10;m",
        Sequence::Mouse(Mouse::Up(MouseButton::Left, 20, 10), KeyModifiers::empty()),
        b"\x1B[<1;20;10;m",
        Sequence::Mouse(
            Mouse::Up(MouseButton::Middle, 20, 10),
            KeyModifiers::empty()
        ),
        b"\x1B[<2;20;10;m",
        Sequence::Mouse(Mouse::Up(MouseButton::Right, 20, 10), KeyModifiers::empty()),
    );
}

#[test]
fn button_up_with_key_modifiers() {
    test_sequences!(
        b"\x1B[<4;20;10;m",
        Sequence::Mouse(Mouse::Up(MouseButton::Left, 20, 10), KeyModifiers::SHIFT),
        b"\x1B[<5;20;10;m",
        Sequence::Mouse(Mouse::Up(MouseButton::Middle, 20, 10), KeyModifiers::SHIFT),
        b"\x1B[<6;20;10;m",
        Sequence::Mouse(Mouse::Up(MouseButton::Right, 20, 10), KeyModifiers::SHIFT),
    );
}

#[test]
fn scroll() {
    test_sequences!(
        b"\x1B[<64;20;10;m",
        Sequence::Mouse(Mouse::ScrollUp(20, 10), KeyModifiers::empty()),
        b"\x1B[<65;20;10;m",
        Sequence::Mouse(Mouse::ScrollDown(20, 10), KeyModifiers::empty()),
    );
}

#[test]
fn scroll_with_key_modifiers() {
    test_sequences!(
        b"\x1B[<68;20;10;m",
        Sequence::Mouse(Mouse::ScrollUp(20, 10), KeyModifiers::SHIFT),
        b"\x1B[<69;20;10;m",
        Sequence::Mouse(Mouse::ScrollDown(20, 10), KeyModifiers::SHIFT),
    );
}

#[test]
fn drag() {
    test_sequences!(
        b"\x1B[<32;20;10;M",
        Sequence::Mouse(
            Mouse::Drag(MouseButton::Left, 20, 10),
            KeyModifiers::empty()
        ),
        b"\x1B[<33;20;10;M",
        Sequence::Mouse(
            Mouse::Drag(MouseButton::Middle, 20, 10),
            KeyModifiers::empty()
        ),
        b"\x1B[<34;20;10;M",
        Sequence::Mouse(
            Mouse::Drag(MouseButton::Right, 20, 10),
            KeyModifiers::empty()
        ),
    );
}

#[test]
fn drag_with_key_modifiers() {
    test_sequences!(
        b"\x1B[<36;20;10;M",
        Sequence::Mouse(Mouse::Drag(MouseButton::Left, 20, 10), KeyModifiers::SHIFT),
        b"\x1B[<37;20;10;M",
        Sequence::Mouse(
            Mouse::Drag(MouseButton::Middle, 20, 10),
            KeyModifiers::SHIFT,
        ),
        b"\x1B[<38;20;10;M",
        Sequence::Mouse(Mouse::Drag(MouseButton::Right, 20, 10), KeyModifiers::SHIFT),
    );
}

#[test]
fn key_modifier_combinations() {
    test_sequences!(
        b"\x1B[<4;20;10;m",
        Sequence::Mouse(Mouse::Up(MouseButton::Left, 20, 10), KeyModifiers::SHIFT),
        b"\x1B[<8;20;10;m",
        Sequence::Mouse(Mouse::Up(MouseButton::Left, 20, 10), KeyModifiers::ALT),
        b"\x1B[<16;20;10;m",
        Sequence::Mouse(Mouse::Up(MouseButton::Left, 20, 10), KeyModifiers::CONTROL),
        b"\x1B[<12;20;10;m",
        Sequence::Mouse(
            Mouse::Up(MouseButton::Left, 20, 10),
            KeyModifiers::SHIFT | KeyModifiers::ALT
        ),
        b"\x1B[<20;20;10;m",
        Sequence::Mouse(
            Mouse::Up(MouseButton::Left, 20, 10),
            KeyModifiers::SHIFT | KeyModifiers::CONTROL
        ),
        b"\x1B[<24;20;10;m",
        Sequence::Mouse(
            Mouse::Up(MouseButton::Left, 20, 10),
            KeyModifiers::ALT | KeyModifiers::CONTROL
        ),
        b"\x1B[<28;20;10;m",
        Sequence::Mouse(
            Mouse::Up(MouseButton::Left, 20, 10),
            KeyModifiers::SHIFT | KeyModifiers::ALT | KeyModifiers::CONTROL
        ),
    );
}
