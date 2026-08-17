use super::types::{KeyCode, KeyModifiers, Mouse, MouseButton, Sequence};

pub(crate) fn parse_char(ch: char, esc_o: bool) -> Option<Sequence> {
    if esc_o {
        return match ch {
            'P'..='S' => Some(Sequence::Key(
                KeyCode::F(ch as u8 - b'P' + 1),
                KeyModifiers::empty(),
            )),
            _ => None,
        };
    }

    let code = match ch {
        '\r' | '\n' => KeyCode::Enter,
        '\t' => KeyCode::Tab,
        '\x7F' => KeyCode::BackTab,
        '\x1B' => KeyCode::Esc,
        '\0' => KeyCode::Null,
        _ => KeyCode::Char(ch),
    };
    Some(Sequence::Key(code, KeyModifiers::empty()))
}

pub(crate) fn parse_esc_sequence(ch: char) -> Option<Sequence> {
    // EscO[P-S] is handled in the Performer, see parse_char & esc_o argument
    // No need to handle other cases here? It's just Alt+$char
    Some(Sequence::Key(KeyCode::Char(ch), KeyModifiers::ALT))
}

pub(crate) fn parse_csi_sequence(
    parameters: &[u64],
    _ignored_count: usize,
    ch: char,
) -> Option<Sequence> {
    match ch {
        'A' => Some(Sequence::Key(
            KeyCode::Up,
            parse_csi_arrow_key_modifiers(parameters.first().cloned()),
        )),
        'B' => Some(Sequence::Key(
            KeyCode::Down,
            parse_csi_arrow_key_modifiers(parameters.first().cloned()),
        )),
        'C' => Some(Sequence::Key(
            KeyCode::Right,
            parse_csi_arrow_key_modifiers(parameters.first().cloned()),
        )),
        'D' => Some(Sequence::Key(
            KeyCode::Left,
            parse_csi_arrow_key_modifiers(parameters.first().cloned()),
        )),
        'H' => Some(Sequence::Key(KeyCode::Home, KeyModifiers::empty())),
        'F' => Some(Sequence::Key(KeyCode::End, KeyModifiers::empty())),
        'Z' => Some(Sequence::Key(KeyCode::BackTab, KeyModifiers::empty())),
        'R' => parse_csi_cursor_position(parameters),
        'm' => parse_csi_xterm_mouse(parameters, ch),
        'M' if parameters.first() == Some(&0x3C) => parse_csi_xterm_mouse(parameters, ch),
        'M' => parse_csi_rxvt_mouse(parameters),
        '~' => parse_csi_tilde_key_code(parameters),
        _ => None,
    }
}

fn parse_csi_arrow_key_modifiers(parameter: Option<u64>) -> KeyModifiers {
    parse_key_modifiers(parameter.map(|x| x.saturating_sub(48)))
}

fn parse_key_modifiers(parameter: Option<u64>) -> KeyModifiers {
    if let Some(parameter) = parameter {
        match parameter {
            2 => KeyModifiers::SHIFT,
            3 => KeyModifiers::ALT,
            4 => KeyModifiers::SHIFT | KeyModifiers::ALT,
            5 => KeyModifiers::CONTROL,
            6 => KeyModifiers::SHIFT | KeyModifiers::CONTROL,
            7 => KeyModifiers::ALT | KeyModifiers::CONTROL,
            8 => KeyModifiers::SHIFT | KeyModifiers::ALT | KeyModifiers::CONTROL,
            9 => KeyModifiers::META,
            10 => KeyModifiers::META | KeyModifiers::SHIFT,
            11 => KeyModifiers::META | KeyModifiers::ALT,
            12 => KeyModifiers::META | KeyModifiers::SHIFT | KeyModifiers::ALT,
            13 => KeyModifiers::META | KeyModifiers::CONTROL,
            14 => KeyModifiers::META | KeyModifiers::SHIFT | KeyModifiers::CONTROL,
            15 => KeyModifiers::META | KeyModifiers::ALT | KeyModifiers::CONTROL,
            16 => {
                KeyModifiers::META | KeyModifiers::SHIFT | KeyModifiers::ALT | KeyModifiers::CONTROL
            }
            _ => KeyModifiers::empty(),
        }
    } else {
        KeyModifiers::empty()
    }
}

fn parse_csi_tilde_key_code(parameters: &[u64]) -> Option<Sequence> {
    if parameters.is_empty() {
        return None;
    }

    let modifiers = parse_key_modifiers(parameters.get(1).cloned());

    let code = match parameters[0] {
        1 | 7 => KeyCode::Home,
        2 => KeyCode::Insert,
        3 => KeyCode::Delete,
        4 | 8 => KeyCode::End,
        5 => KeyCode::PageUp,
        6 => KeyCode::PageDown,
        p @ 11..=15 => KeyCode::F(p as u8 - 10),
        p @ 17..=21 => KeyCode::F(p as u8 - 11),
        p @ 23..=24 => KeyCode::F(p as u8 - 12),
        _ => return None,
    };

    Some(Sequence::Key(code, modifiers))
}

fn parse_csi_cursor_position(parameters: &[u64]) -> Option<Sequence> {
    // ESC [ Cy ; Cx R

    if parameters.len() < 2 {
        return None;
    }

    let y = parameters[0] as u16;
    let x = parameters[1] as u16;

    Some(Sequence::CursorPosition(x, y))
}

fn parse_csi_xterm_mouse(parameters: &[u64], ch: char) -> Option<Sequence> {
    // ESC [ < Cb ; Cx ; Cy (;) (M or m)

    if parameters.len() < 4 {
        return None;
    }

    let cb = parameters[1] as u8;
    let cx = parameters[2] as u16;
    let cy = parameters[3] as u16;

    let up = match ch {
        'm' => true,
        'M' => false,
        _ => return None,
    };

    let mut modifiers = KeyModifiers::empty();

    if cb & 0b0000_0100 == 0b0000_0100 {
        modifiers |= KeyModifiers::SHIFT;
    }

    if cb & 0b0000_1000 == 0b0000_1000 {
        modifiers |= KeyModifiers::ALT;
    }

    if cb & 0b0001_0000 == 0b0001_0000 {
        modifiers |= KeyModifiers::CONTROL;
    }

    let mouse = if cb & 0b0100_0000 == 0b0100_0000 {
        if cb & 0b0000_0001 == 0b0000_0001 {
            Mouse::ScrollDown(cx, cy)
        } else {
            Mouse::ScrollUp(cx, cy)
        }
    } else {
        let drag = cb & 0b0010_0000 == 0b0010_0000;

        match (cb & 0b0000_0011, up, drag) {
            (0, true, _) => Mouse::Up(MouseButton::Left, cx, cy),
            (0, false, false) => Mouse::Down(MouseButton::Left, cx, cy),
            (0, false, true) => Mouse::Drag(MouseButton::Left, cx, cy),
            (1, true, _) => Mouse::Up(MouseButton::Middle, cx, cy),
            (1, false, false) => Mouse::Down(MouseButton::Middle, cx, cy),
            (1, false, true) => Mouse::Drag(MouseButton::Middle, cx, cy),
            (2, true, _) => Mouse::Up(MouseButton::Right, cx, cy),
            (2, false, false) => Mouse::Down(MouseButton::Right, cx, cy),
            (2, false, true) => Mouse::Drag(MouseButton::Right, cx, cy),
            _ => return None,
        }
    };

    Some(Sequence::Mouse(mouse, modifiers))
}

fn parse_csi_rxvt_mouse(parameters: &[u64]) -> Option<Sequence> {
    // ESC [ Cb ; Cx ; Cy ; M

    if parameters.len() < 3 {
        return None;
    }

    let cb = parameters[0];
    let cx = parameters[1] as u16;
    let cy = parameters[2] as u16;

    let mut modifiers = KeyModifiers::empty();

    if cb & 0b0000_0100 == 0b0000_0100 {
        modifiers |= KeyModifiers::SHIFT;
    }

    if cb & 0b0000_1000 == 0b0000_1000 {
        modifiers |= KeyModifiers::ALT;
    }

    if cb & 0b0001_0000 == 0b0001_0000 {
        modifiers |= KeyModifiers::CONTROL;
    }

    let mouse = if cb & 0b0110_0000 == 0b0110_0000 {
        if cb & 0b0000_0001 == 0b0000_0001 {
            Mouse::ScrollDown(cx, cy)
        } else {
            Mouse::ScrollUp(cx, cy)
        }
    } else {
        let drag = cb & 0b0100_0000 == 0b0100_0000;

        match (cb & 0b0000_0011, drag) {
            (0b0000_0000, false) => Mouse::Down(MouseButton::Left, cx, cy),
            (0b0000_0010, false) => Mouse::Down(MouseButton::Right, cx, cy),
            (0b0000_0001, false) => Mouse::Down(MouseButton::Middle, cx, cy),

            (0b0000_0000, true) => Mouse::Drag(MouseButton::Left, cx, cy),
            (0b0000_0010, true) => Mouse::Drag(MouseButton::Right, cx, cy),
            (0b0000_0001, true) => Mouse::Drag(MouseButton::Middle, cx, cy),

            (0b0000_0011, false) => Mouse::Up(MouseButton::Any, cx, cy),

            _ => return None,
        }
    };

    Some(Sequence::Mouse(mouse, modifiers))
}
