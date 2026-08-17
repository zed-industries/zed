use anes::parser::{KeyCode, KeyModifiers, Sequence};

use crate::test_sequences;

#[test]
fn esc_o_f_keys() {
    test_sequences!(
        b"\x1BOP",
        Sequence::Key(KeyCode::F(1), KeyModifiers::empty()),
        b"\x1BOQ",
        Sequence::Key(KeyCode::F(2), KeyModifiers::empty()),
        b"\x1BOR",
        Sequence::Key(KeyCode::F(3), KeyModifiers::empty()),
        b"\x1BOS",
        Sequence::Key(KeyCode::F(4), KeyModifiers::empty()),
    );
}

#[test]
fn csi_key_codes() {
    test_sequences!(
        b"\x1B[A",
        Sequence::Key(KeyCode::Up, KeyModifiers::empty()),
        b"\x1B[B",
        Sequence::Key(KeyCode::Down, KeyModifiers::empty()),
        b"\x1B[C",
        Sequence::Key(KeyCode::Right, KeyModifiers::empty()),
        b"\x1B[D",
        Sequence::Key(KeyCode::Left, KeyModifiers::empty()),
        b"\x1B[H",
        Sequence::Key(KeyCode::Home, KeyModifiers::empty()),
        b"\x1B[F",
        Sequence::Key(KeyCode::End, KeyModifiers::empty()),
        b"\x1B[Z",
        Sequence::Key(KeyCode::BackTab, KeyModifiers::empty()),
    );
}

#[test]
fn csi_arrow_key_modifiers() {
    test_sequences!(
        b"\x1B[50A",
        Sequence::Key(KeyCode::Up, KeyModifiers::SHIFT),
        b"\x1B[53A",
        Sequence::Key(KeyCode::Up, KeyModifiers::CONTROL),
    );
}

#[test]
fn csi_tilde_key_modifiers() {
    test_sequences!(
        b"\x1B[1~",
        Sequence::Key(KeyCode::Home, KeyModifiers::empty()),
        b"\x1B[1;0~",
        Sequence::Key(KeyCode::Home, KeyModifiers::empty()),
        b"\x1B[1;1~",
        Sequence::Key(KeyCode::Home, KeyModifiers::empty()),
        b"\x1B[1;2~",
        Sequence::Key(KeyCode::Home, KeyModifiers::SHIFT),
        b"\x1B[1;3~",
        Sequence::Key(KeyCode::Home, KeyModifiers::ALT),
        b"\x1B[1;4~",
        Sequence::Key(KeyCode::Home, KeyModifiers::SHIFT | KeyModifiers::ALT),
        b"\x1B[1;5~",
        Sequence::Key(KeyCode::Home, KeyModifiers::CONTROL),
        b"\x1B[1;6~",
        Sequence::Key(KeyCode::Home, KeyModifiers::SHIFT | KeyModifiers::CONTROL),
        b"\x1B[1;7~",
        Sequence::Key(KeyCode::Home, KeyModifiers::ALT | KeyModifiers::CONTROL),
        b"\x1B[1;8~",
        Sequence::Key(
            KeyCode::Home,
            KeyModifiers::SHIFT | KeyModifiers::ALT | KeyModifiers::CONTROL
        ),
        b"\x1B[1;9~",
        Sequence::Key(KeyCode::Home, KeyModifiers::META),
        b"\x1B[1;10~",
        Sequence::Key(KeyCode::Home, KeyModifiers::META | KeyModifiers::SHIFT),
        b"\x1B[1;11~",
        Sequence::Key(KeyCode::Home, KeyModifiers::META | KeyModifiers::ALT),
        b"\x1B[1;12~",
        Sequence::Key(
            KeyCode::Home,
            KeyModifiers::META | KeyModifiers::SHIFT | KeyModifiers::ALT
        ),
        b"\x1B[1;13~",
        Sequence::Key(KeyCode::Home, KeyModifiers::META | KeyModifiers::CONTROL),
        b"\x1B[1;14~",
        Sequence::Key(
            KeyCode::Home,
            KeyModifiers::META | KeyModifiers::SHIFT | KeyModifiers::CONTROL
        ),
        b"\x1B[1;15~",
        Sequence::Key(
            KeyCode::Home,
            KeyModifiers::META | KeyModifiers::ALT | KeyModifiers::CONTROL
        ),
        b"\x1B[1;16~",
        Sequence::Key(
            KeyCode::Home,
            KeyModifiers::META | KeyModifiers::SHIFT | KeyModifiers::ALT | KeyModifiers::CONTROL
        ),
        b"\x1B[1;17~",
        Sequence::Key(KeyCode::Home, KeyModifiers::empty()),
    );
}

#[test]
fn csi_tilde_f_keys() {
    test_sequences!(
        b"\x1B[11~",
        Sequence::Key(KeyCode::F(1), KeyModifiers::empty()),
        b"\x1B[12~",
        Sequence::Key(KeyCode::F(2), KeyModifiers::empty()),
        b"\x1B[13~",
        Sequence::Key(KeyCode::F(3), KeyModifiers::empty()),
        b"\x1B[14~",
        Sequence::Key(KeyCode::F(4), KeyModifiers::empty()),
        b"\x1B[15~",
        Sequence::Key(KeyCode::F(5), KeyModifiers::empty()),
        b"\x1B[17~",
        Sequence::Key(KeyCode::F(6), KeyModifiers::empty()),
        b"\x1B[18~",
        Sequence::Key(KeyCode::F(7), KeyModifiers::empty()),
        b"\x1B[19~",
        Sequence::Key(KeyCode::F(8), KeyModifiers::empty()),
        b"\x1B[20~",
        Sequence::Key(KeyCode::F(9), KeyModifiers::empty()),
        b"\x1B[21~",
        Sequence::Key(KeyCode::F(10), KeyModifiers::empty()),
        b"\x1B[23~",
        Sequence::Key(KeyCode::F(11), KeyModifiers::empty()),
        b"\x1B[24~",
        Sequence::Key(KeyCode::F(12), KeyModifiers::empty()),
    );
}

#[test]
fn csi_tilde_key_codes() {
    test_sequences!(
        b"\x1B[1~",
        Sequence::Key(KeyCode::Home, KeyModifiers::empty()),
        b"\x1B[2~",
        Sequence::Key(KeyCode::Insert, KeyModifiers::empty()),
        b"\x1B[3~",
        Sequence::Key(KeyCode::Delete, KeyModifiers::empty()),
        b"\x1B[4~",
        Sequence::Key(KeyCode::End, KeyModifiers::empty()),
        b"\x1B[5~",
        Sequence::Key(KeyCode::PageUp, KeyModifiers::empty()),
        b"\x1B[6~",
        Sequence::Key(KeyCode::PageDown, KeyModifiers::empty()),
        b"\x1B[7~",
        Sequence::Key(KeyCode::Home, KeyModifiers::empty()),
        b"\x1B[8~",
        Sequence::Key(KeyCode::End, KeyModifiers::empty()),
    );
}
