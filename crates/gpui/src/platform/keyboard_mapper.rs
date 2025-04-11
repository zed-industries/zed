use super::{KeyCode, Modifiers};

/// TODO:
pub trait KeyboardMapper {
    /// TODO:
    fn parse(&self, input: &str, char_matching: bool) -> Option<(KeyCode, Modifiers)>;
    /// TODO:
    fn keycode_to_face(&self, code: KeyCode) -> Option<String>;
    /// TODO:
    fn keycode_to_face_with_shift(&self, code: KeyCode, shift: bool) -> Option<String>;
}

/// TODO:
pub struct UsLayoutMapper;

impl KeyboardMapper for UsLayoutMapper {
    fn parse(&self, input: &str, _char_matching: bool) -> Option<(KeyCode, Modifiers)> {
        let (code, modifiers, _) = KeyCode::parse_us_layout(input, &EmptyMapper);
        Some((code, modifiers))
    }

    fn keycode_to_face(&self, code: KeyCode) -> Option<String> {
        match code {
            KeyCode::A => Some("a".to_string()),
            KeyCode::B => Some("b".to_string()),
            KeyCode::C => Some("c".to_string()),
            KeyCode::D => Some("d".to_string()),
            KeyCode::E => Some("e".to_string()),
            KeyCode::F => Some("f".to_string()),
            KeyCode::G => Some("g".to_string()),
            KeyCode::H => Some("h".to_string()),
            KeyCode::I => Some("i".to_string()),
            KeyCode::J => Some("j".to_string()),
            KeyCode::K => Some("k".to_string()),
            KeyCode::L => Some("l".to_string()),
            KeyCode::M => Some("m".to_string()),
            KeyCode::N => Some("n".to_string()),
            KeyCode::O => Some("o".to_string()),
            KeyCode::P => Some("p".to_string()),
            KeyCode::Q => Some("q".to_string()),
            KeyCode::R => Some("r".to_string()),
            KeyCode::S => Some("s".to_string()),
            KeyCode::T => Some("t".to_string()),
            KeyCode::U => Some("u".to_string()),
            KeyCode::V => Some("v".to_string()),
            KeyCode::W => Some("w".to_string()),
            KeyCode::X => Some("x".to_string()),
            KeyCode::Y => Some("y".to_string()),
            KeyCode::Z => Some("z".to_string()),
            KeyCode::Tilde => Some("`".to_string()),
            KeyCode::Digital0 => Some("0".to_string()),
            KeyCode::Digital1 => Some("1".to_string()),
            KeyCode::Digital2 => Some("2".to_string()),
            KeyCode::Digital3 => Some("3".to_string()),
            KeyCode::Digital4 => Some("4".to_string()),
            KeyCode::Digital5 => Some("5".to_string()),
            KeyCode::Digital6 => Some("6".to_string()),
            KeyCode::Digital7 => Some("7".to_string()),
            KeyCode::Digital8 => Some("8".to_string()),
            KeyCode::Digital9 => Some("9".to_string()),
            KeyCode::Minus => Some("-".to_string()),
            KeyCode::Plus => Some("=".to_string()),
            KeyCode::LeftBracket => Some("[".to_string()),
            KeyCode::RightBracket => Some("]".to_string()),
            KeyCode::Backslash => Some("\\".to_string()),
            KeyCode::Semicolon => Some(";".to_string()),
            KeyCode::Quote => Some("'".to_string()),
            KeyCode::Comma => Some(",".to_string()),
            KeyCode::Period => Some(".".to_string()),
            KeyCode::Slash => Some("/".to_string()),
            _ => None,
        }
    }

    fn keycode_to_face_with_shift(&self, code: KeyCode, shift: bool) -> Option<String> {
        if !shift {
            return self.keycode_to_face(code);
        }
        if let Some(key) = self.keycode_to_face(code) {
            let upper = key.clone().to_ascii_uppercase();
            if key != upper {
                return Some(upper);
            }
        }
        match code {
            KeyCode::Tilde => Some("~".to_string()),
            KeyCode::Digital0 => Some(")".to_string()),
            KeyCode::Digital1 => Some("!".to_string()),
            KeyCode::Digital2 => Some("@".to_string()),
            KeyCode::Digital3 => Some("#".to_string()),
            KeyCode::Digital4 => Some("$".to_string()),
            KeyCode::Digital5 => Some("%".to_string()),
            KeyCode::Digital6 => Some("^".to_string()),
            KeyCode::Digital7 => Some("&".to_string()),
            KeyCode::Digital8 => Some("*".to_string()),
            KeyCode::Digital9 => Some("(".to_string()),
            KeyCode::Minus => Some("_".to_string()),
            KeyCode::Plus => Some("+".to_string()),
            KeyCode::LeftBracket => Some("{".to_string()),
            KeyCode::RightBracket => Some("}".to_string()),
            KeyCode::Backslash => Some("|".to_string()),
            KeyCode::Semicolon => Some(":".to_string()),
            KeyCode::Quote => Some("\"".to_string()),
            KeyCode::Comma => Some("<".to_string()),
            KeyCode::Period => Some(">".to_string()),
            KeyCode::Slash => Some("?".to_string()),
            _ => None,
        }
    }
}

struct EmptyMapper;

impl KeyboardMapper for EmptyMapper {
    fn parse(&self, _: &str, _: bool) -> Option<(KeyCode, Modifiers)> {
        None
    }

    fn keycode_to_face(&self, _: KeyCode) -> Option<String> {
        None
    }

    fn keycode_to_face_with_shift(&self, _: KeyCode, _: bool) -> Option<String> {
        None
    }
}
