/// The mappings defined in this file where created from reading the alacritty source
use alacritty_terminal::term::TermMode;
use gpui::{Keys, Keystroke};

#[derive(Debug, PartialEq, Eq)]
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

pub fn to_esc_str(keystroke: &Keystroke, mode: &TermMode, alt_is_meta: bool) -> Option<String> {
    let modifiers = AlacModifiers::new(keystroke);

    // Manual Bindings including modifiers
    let manual_esc_str = match (keystroke.key, &modifiers) {
        //Basic special keys
        (Keys::Tab, AlacModifiers::None) => Some("\x09".to_string()),
        (Keys::Escape, AlacModifiers::None) => Some("\x1b".to_string()),
        (Keys::Enter, AlacModifiers::None) => Some("\x0d".to_string()),
        (Keys::Enter, AlacModifiers::Shift) => Some("\x0d".to_string()),
        (Keys::Backspace, AlacModifiers::None) => Some("\x7f".to_string()),
        //Interesting escape codes
        (Keys::Tab, AlacModifiers::Shift) => Some("\x1b[Z".to_string()),
        (Keys::Backspace, AlacModifiers::Ctrl) => Some("\x08".to_string()),
        (Keys::Backspace, AlacModifiers::Alt) => Some("\x1b\x7f".to_string()),
        (Keys::Backspace, AlacModifiers::Shift) => Some("\x7f".to_string()),
        (Keys::Space, AlacModifiers::Ctrl) => Some("\x00".to_string()),
        (Keys::Home, AlacModifiers::Shift) if mode.contains(TermMode::ALT_SCREEN) => {
            Some("\x1b[1;2H".to_string())
        }
        (Keys::End, AlacModifiers::Shift) if mode.contains(TermMode::ALT_SCREEN) => {
            Some("\x1b[1;2F".to_string())
        }
        (Keys::PageUp, AlacModifiers::Shift) if mode.contains(TermMode::ALT_SCREEN) => {
            Some("\x1b[5;2~".to_string())
        }
        (Keys::PageDown, AlacModifiers::Shift) if mode.contains(TermMode::ALT_SCREEN) => {
            Some("\x1b[6;2~".to_string())
        }
        (Keys::Home, AlacModifiers::None) if mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1bOH".to_string())
        }
        (Keys::Home, AlacModifiers::None) if !mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1b[H".to_string())
        }
        (Keys::End, AlacModifiers::None) if mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1bOF".to_string())
        }
        (Keys::End, AlacModifiers::None) if !mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1b[F".to_string())
        }
        (Keys::Up, AlacModifiers::None) if mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1bOA".to_string())
        }
        (Keys::Up, AlacModifiers::None) if !mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1b[A".to_string())
        }
        (Keys::Down, AlacModifiers::None) if mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1bOB".to_string())
        }
        (Keys::Down, AlacModifiers::None) if !mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1b[B".to_string())
        }
        (Keys::Right, AlacModifiers::None) if mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1bOC".to_string())
        }
        (Keys::Right, AlacModifiers::None) if !mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1b[C".to_string())
        }
        (Keys::Left, AlacModifiers::None) if mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1bOD".to_string())
        }
        (Keys::Left, AlacModifiers::None) if !mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1b[D".to_string())
        }
        // TODO: correct?
        // ("back", AlacModifiers::None) => Some("\x7f".to_string()),
        // (VirtualKeyCode::Backspace, AlacModifiers::None) => Some("\x7f".to_string()),
        (Keys::Insert, AlacModifiers::None) => Some("\x1b[2~".to_string()),
        (Keys::Delete, AlacModifiers::None) => Some("\x1b[3~".to_string()),
        (Keys::PageUp, AlacModifiers::None) => Some("\x1b[5~".to_string()),
        (Keys::PageDown, AlacModifiers::None) => Some("\x1b[6~".to_string()),
        (Keys::F1, AlacModifiers::None) => Some("\x1bOP".to_string()),
        (Keys::F2, AlacModifiers::None) => Some("\x1bOQ".to_string()),
        (Keys::F3, AlacModifiers::None) => Some("\x1bOR".to_string()),
        (Keys::F4, AlacModifiers::None) => Some("\x1bOS".to_string()),
        (Keys::F5, AlacModifiers::None) => Some("\x1b[15~".to_string()),
        (Keys::F6, AlacModifiers::None) => Some("\x1b[17~".to_string()),
        (Keys::F7, AlacModifiers::None) => Some("\x1b[18~".to_string()),
        (Keys::F8, AlacModifiers::None) => Some("\x1b[19~".to_string()),
        (Keys::F9, AlacModifiers::None) => Some("\x1b[20~".to_string()),
        (Keys::F10, AlacModifiers::None) => Some("\x1b[21~".to_string()),
        (Keys::F11, AlacModifiers::None) => Some("\x1b[23~".to_string()),
        (Keys::F12, AlacModifiers::None) => Some("\x1b[24~".to_string()),
        (Keys::F13, AlacModifiers::None) => Some("\x1b[25~".to_string()),
        (Keys::F14, AlacModifiers::None) => Some("\x1b[26~".to_string()),
        (Keys::F15, AlacModifiers::None) => Some("\x1b[28~".to_string()),
        (Keys::F16, AlacModifiers::None) => Some("\x1b[29~".to_string()),
        (Keys::F17, AlacModifiers::None) => Some("\x1b[31~".to_string()),
        (Keys::F18, AlacModifiers::None) => Some("\x1b[32~".to_string()),
        (Keys::F19, AlacModifiers::None) => Some("\x1b[33~".to_string()),
        (Keys::F20, AlacModifiers::None) => Some("\x1b[34~".to_string()),
        // NumpadEnter, Action::Esc("\n".into());
        //Mappings for caret notation keys
        // TODO:
        (Keys::A, AlacModifiers::Ctrl) => Some("\x01".to_string()), //1
        (Keys::A, AlacModifiers::CtrlShift) => Some("\x01".to_string()), //1
        (Keys::B, AlacModifiers::Ctrl) => Some("\x02".to_string()), //2
        (Keys::B, AlacModifiers::CtrlShift) => Some("\x02".to_string()), //2
        (Keys::C, AlacModifiers::Ctrl) => Some("\x03".to_string()), //3
        (Keys::C, AlacModifiers::CtrlShift) => Some("\x03".to_string()), //3
        (Keys::D, AlacModifiers::Ctrl) => Some("\x04".to_string()), //4
        (Keys::D, AlacModifiers::CtrlShift) => Some("\x04".to_string()), //4
        (Keys::E, AlacModifiers::Ctrl) => Some("\x05".to_string()), //5
        (Keys::E, AlacModifiers::CtrlShift) => Some("\x05".to_string()), //5
        (Keys::F, AlacModifiers::Ctrl) => Some("\x06".to_string()), //6
        (Keys::F, AlacModifiers::CtrlShift) => Some("\x06".to_string()), //6
        (Keys::G, AlacModifiers::Ctrl) => Some("\x07".to_string()), //7
        (Keys::G, AlacModifiers::CtrlShift) => Some("\x07".to_string()), //7
        (Keys::H, AlacModifiers::Ctrl) => Some("\x08".to_string()), //8
        (Keys::H, AlacModifiers::CtrlShift) => Some("\x08".to_string()), //8
        (Keys::I, AlacModifiers::Ctrl) => Some("\x09".to_string()), //9
        (Keys::I, AlacModifiers::CtrlShift) => Some("\x09".to_string()), //9
        (Keys::J, AlacModifiers::Ctrl) => Some("\x0a".to_string()), //10
        (Keys::J, AlacModifiers::CtrlShift) => Some("\x0a".to_string()), //10
        (Keys::K, AlacModifiers::Ctrl) => Some("\x0b".to_string()), //11
        (Keys::K, AlacModifiers::CtrlShift) => Some("\x0b".to_string()), //11
        (Keys::L, AlacModifiers::Ctrl) => Some("\x0c".to_string()), //12
        (Keys::L, AlacModifiers::CtrlShift) => Some("\x0c".to_string()), //12
        (Keys::M, AlacModifiers::Ctrl) => Some("\x0d".to_string()), //13
        (Keys::M, AlacModifiers::CtrlShift) => Some("\x0d".to_string()), //13
        (Keys::N, AlacModifiers::Ctrl) => Some("\x0e".to_string()), //14
        (Keys::N, AlacModifiers::CtrlShift) => Some("\x0e".to_string()), //14
        (Keys::O, AlacModifiers::Ctrl) => Some("\x0f".to_string()), //15
        (Keys::O, AlacModifiers::CtrlShift) => Some("\x0f".to_string()), //15
        (Keys::P, AlacModifiers::Ctrl) => Some("\x10".to_string()), //16
        (Keys::P, AlacModifiers::CtrlShift) => Some("\x10".to_string()), //16
        (Keys::Q, AlacModifiers::Ctrl) => Some("\x11".to_string()), //17
        (Keys::Q, AlacModifiers::CtrlShift) => Some("\x11".to_string()), //17
        (Keys::R, AlacModifiers::Ctrl) => Some("\x12".to_string()), //18
        (Keys::R, AlacModifiers::CtrlShift) => Some("\x12".to_string()), //18
        (Keys::S, AlacModifiers::Ctrl) => Some("\x13".to_string()), //19
        (Keys::S, AlacModifiers::CtrlShift) => Some("\x13".to_string()), //19
        (Keys::T, AlacModifiers::Ctrl) => Some("\x14".to_string()), //20
        (Keys::T, AlacModifiers::CtrlShift) => Some("\x14".to_string()), //20
        (Keys::U, AlacModifiers::Ctrl) => Some("\x15".to_string()), //21
        (Keys::U, AlacModifiers::CtrlShift) => Some("\x15".to_string()), //21
        (Keys::V, AlacModifiers::Ctrl) => Some("\x16".to_string()), //22
        (Keys::V, AlacModifiers::CtrlShift) => Some("\x16".to_string()), //22
        (Keys::W, AlacModifiers::Ctrl) => Some("\x17".to_string()), //23
        (Keys::W, AlacModifiers::CtrlShift) => Some("\x17".to_string()), //23
        (Keys::X, AlacModifiers::Ctrl) => Some("\x18".to_string()), //24
        (Keys::X, AlacModifiers::CtrlShift) => Some("\x18".to_string()), //24
        (Keys::Y, AlacModifiers::Ctrl) => Some("\x19".to_string()), //25
        (Keys::Y, AlacModifiers::CtrlShift) => Some("\x19".to_string()), //25
        (Keys::Z, AlacModifiers::Ctrl) => Some("\x1a".to_string()), //26
        (Keys::Z, AlacModifiers::CtrlShift) => Some("\x1a".to_string()), //26
        // TODO:
        // No @ key, just VirtualKeyCode::Digital2 + VirtualKeyCode::Shift
        // ("@", AlacModifiers::Ctrl) => Some("\x00".to_string()), //0
        (Keys::LeftBracket, AlacModifiers::Ctrl) => Some("\x1b".to_string()), //27
        (Keys::Backslash, AlacModifiers::Ctrl) => Some("\x1c".to_string()),   //28
        (Keys::RightBracket, AlacModifiers::Ctrl) => Some("\x1d".to_string()), //29
        // TODO:
        // No ^ key, VirtualKeyCode::Digital6 + VirtualKeyCode::Shift
        // ("^", AlacModifiers::Ctrl) => Some("\x1e".to_string()), //30
        // TODO:
        // No _ key, VirtualKeyCode::OEMMinus + VirtualKeyCode::Shift
        // ("_", AlacModifiers::Ctrl) => Some("\x1f".to_string()), //31
        // TODO:
        // No ? key, VirtualKeyCode::OEM2 + VirtualKeyCode::Shift
        // ("?", AlacModifiers::Ctrl) => Some("\x7f".to_string()), //127
        _ => None,
    };
    if manual_esc_str.is_some() {
        return manual_esc_str;
    }

    // Automated bindings applying modifiers
    if modifiers.any() {
        let modifier_code = modifier_code(keystroke);
        let modified_esc_str = match keystroke.key {
            Keys::Up => Some(format!("\x1b[1;{}A", modifier_code)),
            Keys::Down => Some(format!("\x1b[1;{}B", modifier_code)),
            Keys::Right => Some(format!("\x1b[1;{}C", modifier_code)),
            Keys::Left => Some(format!("\x1b[1;{}D", modifier_code)),
            Keys::F1 => Some(format!("\x1b[1;{}P", modifier_code)),
            Keys::F2 => Some(format!("\x1b[1;{}Q", modifier_code)),
            Keys::F3 => Some(format!("\x1b[1;{}R", modifier_code)),
            Keys::F4 => Some(format!("\x1b[1;{}S", modifier_code)),
            Keys::F5 => Some(format!("\x1b[15;{}~", modifier_code)),
            Keys::F6 => Some(format!("\x1b[17;{}~", modifier_code)),
            Keys::F7 => Some(format!("\x1b[18;{}~", modifier_code)),
            Keys::F8 => Some(format!("\x1b[19;{}~", modifier_code)),
            Keys::F9 => Some(format!("\x1b[20;{}~", modifier_code)),
            Keys::F10 => Some(format!("\x1b[21;{}~", modifier_code)),
            Keys::F11 => Some(format!("\x1b[23;{}~", modifier_code)),
            Keys::F12 => Some(format!("\x1b[24;{}~", modifier_code)),
            Keys::F13 => Some(format!("\x1b[25;{}~", modifier_code)),
            Keys::F14 => Some(format!("\x1b[26;{}~", modifier_code)),
            Keys::F15 => Some(format!("\x1b[28;{}~", modifier_code)),
            Keys::F16 => Some(format!("\x1b[29;{}~", modifier_code)),
            Keys::F17 => Some(format!("\x1b[31;{}~", modifier_code)),
            Keys::F18 => Some(format!("\x1b[32;{}~", modifier_code)),
            Keys::F19 => Some(format!("\x1b[33;{}~", modifier_code)),
            Keys::F20 => Some(format!("\x1b[34;{}~", modifier_code)),
            _ if modifier_code == 2 => None,
            Keys::Insert => Some(format!("\x1b[2;{}~", modifier_code)),
            Keys::PageUp => Some(format!("\x1b[5;{}~", modifier_code)),
            Keys::PageDown => Some(format!("\x1b[6;{}~", modifier_code)),
            Keys::End => Some(format!("\x1b[1;{}F", modifier_code)),
            Keys::Home => Some(format!("\x1b[1;{}H", modifier_code)),
            _ => None,
        };
        if modified_esc_str.is_some() {
            return modified_esc_str;
        }
    }

    let alt_meta_binding =
        // TODO:
        // if alt_is_meta && modifiers == AlacModifiers::Alt && keystroke.key.is_ascii() {
        if alt_is_meta && modifiers == AlacModifiers::Alt {
            // TODO:
            // Some(format!("\x1b{=}", keystroke.key))
            Some(format!("\x1b{:?}", keystroke.key))
        } else {
            None
        };

    if alt_meta_binding.is_some() {
        return alt_meta_binding;
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

#[cfg(test)]
mod test {
    // use gpui::Modifiers;

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

    // TODO:
    // Under VirtualKeyCode system, anthing that is considered "input", should go into
    // ime_key field.
    // #[test]
    // fn test_plain_inputs() {
    //     let ks = Keystroke {
    //         modifiers: Modifiers {
    //             control: false,
    //             alt: false,
    //             shift: false,
    //             platform: false,
    //             function: false,
    //         },
    //         key: "🖖🏻".to_string(), //2 char string
    //         ime_key: None,
    //     };
    //     assert_eq!(to_esc_str(&ks, &TermMode::NONE, false), None);
    // }

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
                    &Keystroke::parse(&format!("ctrl-{}", lower)).unwrap(),
                    &mode,
                    false
                ),
                to_esc_str(
                    &Keystroke::parse(&format!("ctrl-shift-{}", upper)).unwrap(),
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
        assert_eq!(2, modifier_code(&Keystroke::parse("shift-A").unwrap()));
        assert_eq!(3, modifier_code(&Keystroke::parse("alt-A").unwrap()));
        assert_eq!(4, modifier_code(&Keystroke::parse("shift-alt-A").unwrap()));
        assert_eq!(5, modifier_code(&Keystroke::parse("ctrl-A").unwrap()));
        assert_eq!(6, modifier_code(&Keystroke::parse("shift-ctrl-A").unwrap()));
        assert_eq!(7, modifier_code(&Keystroke::parse("alt-ctrl-A").unwrap()));
        assert_eq!(
            8,
            modifier_code(&Keystroke::parse("shift-ctrl-alt-A").unwrap())
        );
    }
}
