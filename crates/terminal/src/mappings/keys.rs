/// The mappings defined in this file where created from reading the alacritty source
use alacritty_terminal::term::TermMode;
use gpui::{KeyCode, KeyboardMapper, Keystroke, Modifiers};

use collections::HashMap;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum AlacModifiers {
    None,
    Alt,
    Ctrl,
    Shift,
    CtrlShift,
    Other,
}

impl AlacModifiers {
    fn new(ks: &Keystroke) -> Self {
        match (
            ks.modifiers.alt,
            ks.modifiers.control,
            ks.modifiers.shift,
            ks.modifiers.platform,
        ) {
            (false, false, false, false) => AlacModifiers::None,
            (true, false, false, false) => AlacModifiers::Alt,
            (false, true, false, false) => AlacModifiers::Ctrl,
            (false, false, true, false) => AlacModifiers::Shift,
            (false, true, true, false) => AlacModifiers::CtrlShift,
            _ => AlacModifiers::Other,
        }
    }

    fn any(&self) -> bool {
        match &self {
            AlacModifiers::None => false,
            AlacModifiers::Alt => true,
            AlacModifiers::Ctrl => true,
            AlacModifiers::Shift => true,
            AlacModifiers::CtrlShift => true,
            AlacModifiers::Other => true,
        }
    }
}

pub fn generate_esc_str_mapper(
    keyboard_mapper: &dyn KeyboardMapper,
) -> HashMap<(KeyCode, Modifiers), String> {
    let mut esc_str_mapper = HashMap::new();
    let (key_at, modifiers) = keyboard_mapper.parse("@", true).unwrap();
    esc_str_mapper.insert(
        (key_at, modifiers | Modifiers::control()),
        "\x00".to_string(),
    ); //0
    let (key_left_bracket, modifiers) = keyboard_mapper.parse("[", true).unwrap();
    esc_str_mapper.insert(
        (key_left_bracket, modifiers | Modifiers::control()),
        "\x1b".to_string(),
    ); //27
    if let Some((key_backslash, modifiers)) = keyboard_mapper.parse("\\", true) {
        esc_str_mapper.insert(
            (key_backslash, modifiers | Modifiers::control()),
            "\x1c".to_string(),
        ); //28
    }
    let (key_right_bracket, modifiers) = keyboard_mapper.parse("]", true).unwrap();
    esc_str_mapper.insert(
        (key_right_bracket, modifiers | Modifiers::control()),
        "\x1d".to_string(),
    ); //29
    if let Some((key_caret, modifiers)) = keyboard_mapper.parse("^", true) {
        esc_str_mapper.insert(
            (key_caret, modifiers | Modifiers::control()),
            "\x1e".to_string(),
        ); //30
    }
    let (key_underscore, modifiers) = keyboard_mapper.parse("_", true).unwrap();
    esc_str_mapper.insert(
        (key_underscore, modifiers | Modifiers::control()),
        "\x1f".to_string(),
    ); //31
    let (key_question, modifiers) = keyboard_mapper.parse("?", true).unwrap();
    esc_str_mapper.insert(
        (key_question, modifiers | Modifiers::control()),
        "\x7f".to_string(),
    ); //127

    esc_str_mapper
}

pub fn to_esc_str(
    keystroke: &Keystroke,
    mode: &TermMode,
    alt_is_meta: bool,
    esc_str_mapper: &HashMap<(KeyCode, Modifiers), String>,
) -> Option<String> {
    let modifiers = AlacModifiers::new(keystroke);

    // Manual Bindings including modifiers
    // todo(zjk)
    let manual_esc_str = match (keystroke.code, modifiers) {
        //Basic special keys
        (KeyCode::Tab, AlacModifiers::None) => Some("\x09".to_string()),
        (KeyCode::Escape, AlacModifiers::None) => Some("\x1b".to_string()),
        (KeyCode::Enter, AlacModifiers::None) => Some("\x0d".to_string()),
        (KeyCode::Enter, AlacModifiers::Shift) => Some("\x0d".to_string()),
        (KeyCode::Enter, AlacModifiers::Alt) => Some("\x1b\x0d".to_string()),
        (KeyCode::Backspace, AlacModifiers::None) => Some("\x7f".to_string()),
        //Interesting escape codes
        (KeyCode::Tab, AlacModifiers::Shift) => Some("\x1b[Z".to_string()),
        (KeyCode::Backspace, AlacModifiers::Ctrl) => Some("\x08".to_string()),
        (KeyCode::Backspace, AlacModifiers::Alt) => Some("\x1b\x7f".to_string()),
        (KeyCode::Backspace, AlacModifiers::Shift) => Some("\x7f".to_string()),
        (KeyCode::Space, AlacModifiers::Ctrl) => Some("\x00".to_string()),
        (KeyCode::Home, AlacModifiers::Shift) if mode.contains(TermMode::ALT_SCREEN) => {
            Some("\x1b[1;2H".to_string())
        }
        (KeyCode::End, AlacModifiers::Shift) if mode.contains(TermMode::ALT_SCREEN) => {
            Some("\x1b[1;2F".to_string())
        }
        (KeyCode::PageUp, AlacModifiers::Shift) if mode.contains(TermMode::ALT_SCREEN) => {
            Some("\x1b[5;2~".to_string())
        }
        (KeyCode::PageDown, AlacModifiers::Shift) if mode.contains(TermMode::ALT_SCREEN) => {
            Some("\x1b[6;2~".to_string())
        }
        (KeyCode::Home, AlacModifiers::None) if mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1bOH".to_string())
        }
        (KeyCode::Home, AlacModifiers::None) if !mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1b[H".to_string())
        }
        (KeyCode::End, AlacModifiers::None) if mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1bOF".to_string())
        }
        (KeyCode::End, AlacModifiers::None) if !mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1b[F".to_string())
        }
        (KeyCode::Up, AlacModifiers::None) if mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1bOA".to_string())
        }
        (KeyCode::Up, AlacModifiers::None) if !mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1b[A".to_string())
        }
        (KeyCode::Down, AlacModifiers::None) if mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1bOB".to_string())
        }
        (KeyCode::Down, AlacModifiers::None) if !mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1b[B".to_string())
        }
        (KeyCode::Right, AlacModifiers::None) if mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1bOC".to_string())
        }
        (KeyCode::Right, AlacModifiers::None) if !mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1b[C".to_string())
        }
        (KeyCode::Left, AlacModifiers::None) if mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1bOD".to_string())
        }
        (KeyCode::Left, AlacModifiers::None) if !mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1b[D".to_string())
        }
        (KeyCode::BrowserBack, AlacModifiers::None) => Some("\x7f".to_string()),
        (KeyCode::Insert, AlacModifiers::None) => Some("\x1b[2~".to_string()),
        (KeyCode::Delete, AlacModifiers::None) => Some("\x1b[3~".to_string()),
        (KeyCode::PageUp, AlacModifiers::None) => Some("\x1b[5~".to_string()),
        (KeyCode::PageDown, AlacModifiers::None) => Some("\x1b[6~".to_string()),
        (KeyCode::F1, AlacModifiers::None) => Some("\x1bOP".to_string()),
        (KeyCode::F2, AlacModifiers::None) => Some("\x1bOQ".to_string()),
        (KeyCode::F3, AlacModifiers::None) => Some("\x1bOR".to_string()),
        (KeyCode::F4, AlacModifiers::None) => Some("\x1bOS".to_string()),
        (KeyCode::F5, AlacModifiers::None) => Some("\x1b[15~".to_string()),
        (KeyCode::F6, AlacModifiers::None) => Some("\x1b[17~".to_string()),
        (KeyCode::F7, AlacModifiers::None) => Some("\x1b[18~".to_string()),
        (KeyCode::F8, AlacModifiers::None) => Some("\x1b[19~".to_string()),
        (KeyCode::F9, AlacModifiers::None) => Some("\x1b[20~".to_string()),
        (KeyCode::F10, AlacModifiers::None) => Some("\x1b[21~".to_string()),
        (KeyCode::F11, AlacModifiers::None) => Some("\x1b[23~".to_string()),
        (KeyCode::F12, AlacModifiers::None) => Some("\x1b[24~".to_string()),
        (KeyCode::F13, AlacModifiers::None) => Some("\x1b[25~".to_string()),
        (KeyCode::F14, AlacModifiers::None) => Some("\x1b[26~".to_string()),
        (KeyCode::F15, AlacModifiers::None) => Some("\x1b[28~".to_string()),
        (KeyCode::F16, AlacModifiers::None) => Some("\x1b[29~".to_string()),
        (KeyCode::F17, AlacModifiers::None) => Some("\x1b[31~".to_string()),
        (KeyCode::F18, AlacModifiers::None) => Some("\x1b[32~".to_string()),
        (KeyCode::F19, AlacModifiers::None) => Some("\x1b[33~".to_string()),
        (KeyCode::F20, AlacModifiers::None) => Some("\x1b[34~".to_string()),
        // NumpadEnter, Action::Esc("\n".into());
        //Mappings for caret notation keys
        (KeyCode::A, AlacModifiers::Ctrl) => Some("\x01".to_string()), //1
        (KeyCode::A, AlacModifiers::CtrlShift) => Some("\x01".to_string()), //1
        (KeyCode::B, AlacModifiers::Ctrl) => Some("\x02".to_string()), //2
        (KeyCode::B, AlacModifiers::CtrlShift) => Some("\x02".to_string()), //2
        (KeyCode::C, AlacModifiers::Ctrl) => Some("\x03".to_string()), //3
        (KeyCode::C, AlacModifiers::CtrlShift) => Some("\x03".to_string()), //3
        (KeyCode::D, AlacModifiers::Ctrl) => Some("\x04".to_string()), //4
        (KeyCode::D, AlacModifiers::CtrlShift) => Some("\x04".to_string()), //4
        (KeyCode::E, AlacModifiers::Ctrl) => Some("\x05".to_string()), //5
        (KeyCode::E, AlacModifiers::CtrlShift) => Some("\x05".to_string()), //5
        (KeyCode::F, AlacModifiers::Ctrl) => Some("\x06".to_string()), //6
        (KeyCode::F, AlacModifiers::CtrlShift) => Some("\x06".to_string()), //6
        (KeyCode::G, AlacModifiers::Ctrl) => Some("\x07".to_string()), //7
        (KeyCode::G, AlacModifiers::CtrlShift) => Some("\x07".to_string()), //7
        (KeyCode::H, AlacModifiers::Ctrl) => Some("\x08".to_string()), //8
        (KeyCode::H, AlacModifiers::CtrlShift) => Some("\x08".to_string()), //8
        (KeyCode::I, AlacModifiers::Ctrl) => Some("\x09".to_string()), //9
        (KeyCode::I, AlacModifiers::CtrlShift) => Some("\x09".to_string()), //9
        (KeyCode::J, AlacModifiers::Ctrl) => Some("\x0a".to_string()), //10
        (KeyCode::J, AlacModifiers::CtrlShift) => Some("\x0a".to_string()), //10
        (KeyCode::K, AlacModifiers::Ctrl) => Some("\x0b".to_string()), //11
        (KeyCode::K, AlacModifiers::CtrlShift) => Some("\x0b".to_string()), //11
        (KeyCode::L, AlacModifiers::Ctrl) => Some("\x0c".to_string()), //12
        (KeyCode::L, AlacModifiers::CtrlShift) => Some("\x0c".to_string()), //12
        (KeyCode::M, AlacModifiers::Ctrl) => Some("\x0d".to_string()), //13
        (KeyCode::M, AlacModifiers::CtrlShift) => Some("\x0d".to_string()), //13
        (KeyCode::N, AlacModifiers::Ctrl) => Some("\x0e".to_string()), //14
        (KeyCode::N, AlacModifiers::CtrlShift) => Some("\x0e".to_string()), //14
        (KeyCode::O, AlacModifiers::Ctrl) => Some("\x0f".to_string()), //15
        (KeyCode::O, AlacModifiers::CtrlShift) => Some("\x0f".to_string()), //15
        (KeyCode::P, AlacModifiers::Ctrl) => Some("\x10".to_string()), //16
        (KeyCode::P, AlacModifiers::CtrlShift) => Some("\x10".to_string()), //16
        (KeyCode::Q, AlacModifiers::Ctrl) => Some("\x11".to_string()), //17
        (KeyCode::Q, AlacModifiers::CtrlShift) => Some("\x11".to_string()), //17
        (KeyCode::R, AlacModifiers::Ctrl) => Some("\x12".to_string()), //18
        (KeyCode::R, AlacModifiers::CtrlShift) => Some("\x12".to_string()), //18
        (KeyCode::S, AlacModifiers::Ctrl) => Some("\x13".to_string()), //19
        (KeyCode::S, AlacModifiers::CtrlShift) => Some("\x13".to_string()), //19
        (KeyCode::T, AlacModifiers::Ctrl) => Some("\x14".to_string()), //20
        (KeyCode::T, AlacModifiers::CtrlShift) => Some("\x14".to_string()), //20
        (KeyCode::U, AlacModifiers::Ctrl) => Some("\x15".to_string()), //21
        (KeyCode::U, AlacModifiers::CtrlShift) => Some("\x15".to_string()), //21
        (KeyCode::V, AlacModifiers::Ctrl) => Some("\x16".to_string()), //22
        (KeyCode::V, AlacModifiers::CtrlShift) => Some("\x16".to_string()), //22
        (KeyCode::W, AlacModifiers::Ctrl) => Some("\x17".to_string()), //23
        (KeyCode::W, AlacModifiers::CtrlShift) => Some("\x17".to_string()), //23
        (KeyCode::X, AlacModifiers::Ctrl) => Some("\x18".to_string()), //24
        (KeyCode::X, AlacModifiers::CtrlShift) => Some("\x18".to_string()), //24
        (KeyCode::Y, AlacModifiers::Ctrl) => Some("\x19".to_string()), //25
        (KeyCode::Y, AlacModifiers::CtrlShift) => Some("\x19".to_string()), //25
        (KeyCode::Z, AlacModifiers::Ctrl) => Some("\x1a".to_string()), //26
        (KeyCode::Z, AlacModifiers::CtrlShift) => Some("\x1a".to_string()), //26
        _ => esc_str_mapper
            .get(&(keystroke.code, keystroke.modifiers))
            .cloned(),
    };
    if manual_esc_str.is_some() {
        return manual_esc_str;
    }

    // Automated bindings applying modifiers
    if modifiers.any() {
        let modifier_code = modifier_code(keystroke);
        // todo(zjk)
        let modified_esc_str = match keystroke.code {
            KeyCode::Up => Some(format!("\x1b[1;{}A", modifier_code)),
            KeyCode::Down => Some(format!("\x1b[1;{}B", modifier_code)),
            KeyCode::Right => Some(format!("\x1b[1;{}C", modifier_code)),
            KeyCode::Left => Some(format!("\x1b[1;{}D", modifier_code)),
            KeyCode::F1 => Some(format!("\x1b[1;{}P", modifier_code)),
            KeyCode::F2 => Some(format!("\x1b[1;{}Q", modifier_code)),
            KeyCode::F3 => Some(format!("\x1b[1;{}R", modifier_code)),
            KeyCode::F4 => Some(format!("\x1b[1;{}S", modifier_code)),
            KeyCode::F5 => Some(format!("\x1b[15;{}~", modifier_code)),
            KeyCode::F6 => Some(format!("\x1b[17;{}~", modifier_code)),
            KeyCode::F7 => Some(format!("\x1b[18;{}~", modifier_code)),
            KeyCode::F8 => Some(format!("\x1b[19;{}~", modifier_code)),
            KeyCode::F9 => Some(format!("\x1b[20;{}~", modifier_code)),
            KeyCode::F10 => Some(format!("\x1b[21;{}~", modifier_code)),
            KeyCode::F11 => Some(format!("\x1b[23;{}~", modifier_code)),
            KeyCode::F12 => Some(format!("\x1b[24;{}~", modifier_code)),
            KeyCode::F13 => Some(format!("\x1b[25;{}~", modifier_code)),
            KeyCode::F14 => Some(format!("\x1b[26;{}~", modifier_code)),
            KeyCode::F15 => Some(format!("\x1b[28;{}~", modifier_code)),
            KeyCode::F16 => Some(format!("\x1b[29;{}~", modifier_code)),
            KeyCode::F17 => Some(format!("\x1b[31;{}~", modifier_code)),
            KeyCode::F18 => Some(format!("\x1b[32;{}~", modifier_code)),
            KeyCode::F19 => Some(format!("\x1b[33;{}~", modifier_code)),
            KeyCode::F20 => Some(format!("\x1b[34;{}~", modifier_code)),
            _ if modifier_code == 2 => None,
            KeyCode::Insert => Some(format!("\x1b[2;{}~", modifier_code)),
            KeyCode::PageUp => Some(format!("\x1b[5;{}~", modifier_code)),
            KeyCode::PageDown => Some(format!("\x1b[6;{}~", modifier_code)),
            KeyCode::End => Some(format!("\x1b[1;{}F", modifier_code)),
            KeyCode::Home => Some(format!("\x1b[1;{}H", modifier_code)),
            _ => None,
        };
        if modified_esc_str.is_some() {
            return modified_esc_str;
        }
    }

    if alt_is_meta && modifiers == AlacModifiers::Alt {
        if let Some(key) = retrieve_letter(keystroke.code, keystroke.modifiers.shift) {
            return Some(format!("\x1b{}", key));
        }
    }

    None
}

///   Code     Modifiers
/// ---------+---------------------------
///    2     | Shift
///    3     | Alt
///    4     | Shift + Alt
///    5     | Control
///    6     | Shift + Control
///    7     | Alt + Control
///    8     | Shift + Alt + Control
/// ---------+---------------------------
/// from: https://invisible-island.net/xterm/ctlseqs/ctlseqs.html#h2-PC-Style-Function-Keys
fn modifier_code(keystroke: &Keystroke) -> u32 {
    let mut modifier_code = 0;
    if keystroke.modifiers.shift {
        modifier_code |= 1;
    }
    if keystroke.modifiers.alt {
        modifier_code |= 1 << 1;
    }
    if keystroke.modifiers.control {
        modifier_code |= 1 << 2;
    }
    modifier_code + 1
}

fn retrieve_letter(keycode: KeyCode, shift: bool) -> Option<char> {
    match keycode {
        KeyCode::A => Some(if shift { 'A' } else { 'a' }),
        KeyCode::B => Some(if shift { 'B' } else { 'b' }),
        KeyCode::C => Some(if shift { 'C' } else { 'c' }),
        KeyCode::D => Some(if shift { 'D' } else { 'd' }),
        KeyCode::E => Some(if shift { 'E' } else { 'e' }),
        KeyCode::F => Some(if shift { 'F' } else { 'f' }),
        KeyCode::G => Some(if shift { 'G' } else { 'g' }),
        KeyCode::H => Some(if shift { 'H' } else { 'h' }),
        KeyCode::I => Some(if shift { 'I' } else { 'i' }),
        KeyCode::J => Some(if shift { 'J' } else { 'j' }),
        KeyCode::K => Some(if shift { 'K' } else { 'k' }),
        KeyCode::L => Some(if shift { 'L' } else { 'l' }),
        KeyCode::M => Some(if shift { 'M' } else { 'm' }),
        KeyCode::N => Some(if shift { 'N' } else { 'n' }),
        KeyCode::O => Some(if shift { 'O' } else { 'o' }),
        KeyCode::P => Some(if shift { 'P' } else { 'p' }),
        KeyCode::Q => Some(if shift { 'Q' } else { 'q' }),
        KeyCode::R => Some(if shift { 'R' } else { 'r' }),
        KeyCode::S => Some(if shift { 'S' } else { 's' }),
        KeyCode::T => Some(if shift { 'T' } else { 't' }),
        KeyCode::U => Some(if shift { 'U' } else { 'u' }),
        KeyCode::V => Some(if shift { 'V' } else { 'v' }),
        KeyCode::W => Some(if shift { 'W' } else { 'w' }),
        KeyCode::X => Some(if shift { 'X' } else { 'x' }),
        KeyCode::Y => Some(if shift { 'Y' } else { 'y' }),
        KeyCode::Z => Some(if shift { 'Z' } else { 'z' }),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use gpui::Modifiers;

    use super::*;

    #[test]
    fn test_scroll_keys() {
        //These keys should be handled by the scrolling element directly
        //Need to signify this by returning 'None'
        let shift_pageup = Keystroke::parse("shift-pageup").unwrap();
        let shift_pagedown = Keystroke::parse("shift-pagedown").unwrap();
        let shift_home = Keystroke::parse("shift-home").unwrap();
        let shift_end = Keystroke::parse("shift-end").unwrap();

        let none = TermMode::NONE;
        assert_eq!(to_esc_str(&shift_pageup, &none, false), None);
        assert_eq!(to_esc_str(&shift_pagedown, &none, false), None);
        assert_eq!(to_esc_str(&shift_home, &none, false), None);
        assert_eq!(to_esc_str(&shift_end, &none, false), None);

        let alt_screen = TermMode::ALT_SCREEN;
        assert_eq!(
            to_esc_str(&shift_pageup, &alt_screen, false),
            Some("\x1b[5;2~".to_string())
        );
        assert_eq!(
            to_esc_str(&shift_pagedown, &alt_screen, false),
            Some("\x1b[6;2~".to_string())
        );
        assert_eq!(
            to_esc_str(&shift_home, &alt_screen, false),
            Some("\x1b[1;2H".to_string())
        );
        assert_eq!(
            to_esc_str(&shift_end, &alt_screen, false),
            Some("\x1b[1;2F".to_string())
        );

        let pageup = Keystroke::parse("pageup").unwrap();
        let pagedown = Keystroke::parse("pagedown").unwrap();
        let any = TermMode::ANY;

        assert_eq!(
            to_esc_str(&pageup, &any, false),
            Some("\x1b[5~".to_string())
        );
        assert_eq!(
            to_esc_str(&pagedown, &any, false),
            Some("\x1b[6~".to_string())
        );
    }

    #[test]
    fn test_plain_inputs() {
        let ks = Keystroke {
            modifiers: Modifiers {
                control: false,
                alt: false,
                shift: false,
                platform: false,
                function: false,
            },
            key: "🖖🏻".to_string(), //2 char string
            key_char: None,
        };
        assert_eq!(to_esc_str(&ks, &TermMode::NONE, false), None);
    }

    #[test]
    fn test_application_mode() {
        let app_cursor = TermMode::APP_CURSOR;
        let none = TermMode::NONE;

        let up = Keystroke::parse("up").unwrap();
        let down = Keystroke::parse("down").unwrap();
        let left = Keystroke::parse("left").unwrap();
        let right = Keystroke::parse("right").unwrap();

        assert_eq!(to_esc_str(&up, &none, false), Some("\x1b[A".to_string()));
        assert_eq!(to_esc_str(&down, &none, false), Some("\x1b[B".to_string()));
        assert_eq!(to_esc_str(&right, &none, false), Some("\x1b[C".to_string()));
        assert_eq!(to_esc_str(&left, &none, false), Some("\x1b[D".to_string()));

        assert_eq!(
            to_esc_str(&up, &app_cursor, false),
            Some("\x1bOA".to_string())
        );
        assert_eq!(
            to_esc_str(&down, &app_cursor, false),
            Some("\x1bOB".to_string())
        );
        assert_eq!(
            to_esc_str(&right, &app_cursor, false),
            Some("\x1bOC".to_string())
        );
        assert_eq!(
            to_esc_str(&left, &app_cursor, false),
            Some("\x1bOD".to_string())
        );
    }

    #[test]
    fn test_ctrl_codes() {
        let letters_lower = 'a'..='z';
        let letters_upper = 'A'..='Z';
        let mode = TermMode::ANY;

        for (lower, upper) in letters_lower.zip(letters_upper) {
            assert_eq!(
                to_esc_str(
                    &Keystroke::parse(&format!("ctrl-shift-{}", lower)).unwrap(),
                    &mode,
                    false
                ),
                to_esc_str(
                    &Keystroke::parse(&format!("ctrl-{}", upper)).unwrap(),
                    &mode,
                    false
                ),
                "On letter: {}/{}",
                lower,
                upper
            )
        }
    }

    #[test]
    fn alt_is_meta() {
        let ascii_printable = ' '..='~';
        for character in ascii_printable {
            assert_eq!(
                to_esc_str(
                    &Keystroke::parse(&format!("alt-{}", character)).unwrap(),
                    &TermMode::NONE,
                    true
                )
                .unwrap(),
                format!("\x1b{}", character)
            );
        }

        let gpui_keys = [
            "up", "down", "right", "left", "f1", "f2", "f3", "f4", "F5", "f6", "f7", "f8", "f9",
            "f10", "f11", "f12", "f13", "f14", "f15", "f16", "f17", "f18", "f19", "f20", "insert",
            "pageup", "pagedown", "end", "home",
        ];

        for key in gpui_keys {
            assert_ne!(
                to_esc_str(
                    &Keystroke::parse(&format!("alt-{}", key)).unwrap(),
                    &TermMode::NONE,
                    true
                )
                .unwrap(),
                format!("\x1b{}", key)
            );
        }
    }

    #[test]
    fn test_modifier_code_calc() {
        //   Code     Modifiers
        // ---------+---------------------------
        //    2     | Shift
        //    3     | Alt
        //    4     | Shift + Alt
        //    5     | Control
        //    6     | Shift + Control
        //    7     | Alt + Control
        //    8     | Shift + Alt + Control
        // ---------+---------------------------
        // from: https://invisible-island.net/xterm/ctlseqs/ctlseqs.html#h2-PC-Style-Function-Keys
        assert_eq!(2, modifier_code(&Keystroke::parse("shift-a").unwrap()));
        assert_eq!(3, modifier_code(&Keystroke::parse("alt-a").unwrap()));
        assert_eq!(4, modifier_code(&Keystroke::parse("shift-alt-a").unwrap()));
        assert_eq!(5, modifier_code(&Keystroke::parse("ctrl-a").unwrap()));
        assert_eq!(6, modifier_code(&Keystroke::parse("shift-ctrl-a").unwrap()));
        assert_eq!(7, modifier_code(&Keystroke::parse("alt-ctrl-a").unwrap()));
        assert_eq!(
            8,
            modifier_code(&Keystroke::parse("shift-ctrl-alt-a").unwrap())
        );
    }
}
