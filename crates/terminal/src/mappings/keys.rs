use alacritty_terminal::term::TermMode;
use gpui::keymap::Keystroke;

#[derive(Debug)]
pub enum Modifiers {
    None,
    Alt,
    Ctrl,
    Shift,
    CtrlShift,
    Other,
}

impl Modifiers {
    fn new(ks: &Keystroke) -> Self {
        match (ks.alt, ks.ctrl, ks.shift, ks.cmd) {
            (false, false, false, false) => Modifiers::None,
            (true, false, false, false) => Modifiers::Alt,
            (false, true, false, false) => Modifiers::Ctrl,
            (false, false, true, false) => Modifiers::Shift,
            (false, true, true, false) => Modifiers::CtrlShift,
            _ => Modifiers::Other,
        }
    }

    fn any(&self) -> bool {
        match &self {
            Modifiers::None => false,
            Modifiers::Alt => true,
            Modifiers::Ctrl => true,
            Modifiers::Shift => true,
            Modifiers::CtrlShift => true,
            Modifiers::Other => true,
        }
    }
}

pub fn to_esc_str(keystroke: &Keystroke, mode: &TermMode) -> Option<String> {
    let modifiers = Modifiers::new(&keystroke);

    // Manual Bindings including modifiers
    let manual_esc_str = match (keystroke.key.as_ref(), &modifiers) {
        //Basic special keys
        ("space", Modifiers::None) => Some(" ".to_string()),
        ("tab", Modifiers::None) => Some("\x09".to_string()),
        ("escape", Modifiers::None) => Some("\x1b".to_string()),
        ("enter", Modifiers::None) => Some("\x0d".to_string()),
        ("backspace", Modifiers::None) => Some("\x7f".to_string()),
        //Interesting escape codes
        ("tab", Modifiers::Shift) => Some("\x1b[Z".to_string()),
        ("backspace", Modifiers::Alt) => Some("\x1b\x7f".to_string()),
        ("backspace", Modifiers::Shift) => Some("\x7f".to_string()),
        ("home", Modifiers::Shift) if mode.contains(TermMode::ALT_SCREEN) => {
            Some("\x1b[1;2H".to_string())
        }
        ("end", Modifiers::Shift) if mode.contains(TermMode::ALT_SCREEN) => {
            Some("\x1b[1;2F".to_string())
        }
        ("pageup", Modifiers::Shift) if mode.contains(TermMode::ALT_SCREEN) => {
            Some("\x1b[5;2~".to_string())
        }
        ("pagedown", Modifiers::Shift) if mode.contains(TermMode::ALT_SCREEN) => {
            Some("\x1b[6;2~".to_string())
        }
        ("home", Modifiers::None) if mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1bOH".to_string())
        }
        ("home", Modifiers::None) if !mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1b[H".to_string())
        }
        ("end", Modifiers::None) if mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1bOF".to_string())
        }
        ("end", Modifiers::None) if !mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1b[F".to_string())
        }
        ("up", Modifiers::None) if mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1bOA".to_string())
        }
        ("up", Modifiers::None) if !mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1b[A".to_string())
        }
        ("down", Modifiers::None) if mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1bOB".to_string())
        }
        ("down", Modifiers::None) if !mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1b[B".to_string())
        }
        ("right", Modifiers::None) if mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1bOC".to_string())
        }
        ("right", Modifiers::None) if !mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1b[C".to_string())
        }
        ("left", Modifiers::None) if mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1bOD".to_string())
        }
        ("left", Modifiers::None) if !mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1b[D".to_string())
        }
        ("back", Modifiers::None) => Some("\x7f".to_string()),
        ("insert", Modifiers::None) => Some("\x1b[2~".to_string()),
        ("delete", Modifiers::None) => Some("\x1b[3~".to_string()),
        ("pageup", Modifiers::None) => Some("\x1b[5~".to_string()),
        ("pagedown", Modifiers::None) => Some("\x1b[6~".to_string()),
        ("f1", Modifiers::None) => Some("\x1bOP".to_string()),
        ("f2", Modifiers::None) => Some("\x1bOQ".to_string()),
        ("f3", Modifiers::None) => Some("\x1bOR".to_string()),
        ("f4", Modifiers::None) => Some("\x1bOS".to_string()),
        ("f5", Modifiers::None) => Some("\x1b[15~".to_string()),
        ("f6", Modifiers::None) => Some("\x1b[17~".to_string()),
        ("f7", Modifiers::None) => Some("\x1b[18~".to_string()),
        ("f8", Modifiers::None) => Some("\x1b[19~".to_string()),
        ("f9", Modifiers::None) => Some("\x1b[20~".to_string()),
        ("f10", Modifiers::None) => Some("\x1b[21~".to_string()),
        ("f11", Modifiers::None) => Some("\x1b[23~".to_string()),
        ("f12", Modifiers::None) => Some("\x1b[24~".to_string()),
        ("f13", Modifiers::None) => Some("\x1b[25~".to_string()),
        ("f14", Modifiers::None) => Some("\x1b[26~".to_string()),
        ("f15", Modifiers::None) => Some("\x1b[28~".to_string()),
        ("f16", Modifiers::None) => Some("\x1b[29~".to_string()),
        ("f17", Modifiers::None) => Some("\x1b[31~".to_string()),
        ("f18", Modifiers::None) => Some("\x1b[32~".to_string()),
        ("f19", Modifiers::None) => Some("\x1b[33~".to_string()),
        ("f20", Modifiers::None) => Some("\x1b[34~".to_string()),
        // NumpadEnter, Action::Esc("\n".into());
        //Mappings for caret notation keys
        ("a", Modifiers::Ctrl) => Some("\x01".to_string()), //1
        ("A", Modifiers::CtrlShift) => Some("\x01".to_string()), //1
        ("b", Modifiers::Ctrl) => Some("\x02".to_string()), //2
        ("B", Modifiers::CtrlShift) => Some("\x02".to_string()), //2
        ("c", Modifiers::Ctrl) => Some("\x03".to_string()), //3
        ("C", Modifiers::CtrlShift) => Some("\x03".to_string()), //3
        ("d", Modifiers::Ctrl) => Some("\x04".to_string()), //4
        ("D", Modifiers::CtrlShift) => Some("\x04".to_string()), //4
        ("e", Modifiers::Ctrl) => Some("\x05".to_string()), //5
        ("E", Modifiers::CtrlShift) => Some("\x05".to_string()), //5
        ("f", Modifiers::Ctrl) => Some("\x06".to_string()), //6
        ("F", Modifiers::CtrlShift) => Some("\x06".to_string()), //6
        ("g", Modifiers::Ctrl) => Some("\x07".to_string()), //7
        ("G", Modifiers::CtrlShift) => Some("\x07".to_string()), //7
        ("h", Modifiers::Ctrl) => Some("\x08".to_string()), //8
        ("H", Modifiers::CtrlShift) => Some("\x08".to_string()), //8
        ("i", Modifiers::Ctrl) => Some("\x09".to_string()), //9
        ("I", Modifiers::CtrlShift) => Some("\x09".to_string()), //9
        ("j", Modifiers::Ctrl) => Some("\x0a".to_string()), //10
        ("J", Modifiers::CtrlShift) => Some("\x0a".to_string()), //10
        ("k", Modifiers::Ctrl) => Some("\x0b".to_string()), //11
        ("K", Modifiers::CtrlShift) => Some("\x0b".to_string()), //11
        ("l", Modifiers::Ctrl) => Some("\x0c".to_string()), //12
        ("L", Modifiers::CtrlShift) => Some("\x0c".to_string()), //12
        ("m", Modifiers::Ctrl) => Some("\x0d".to_string()), //13
        ("M", Modifiers::CtrlShift) => Some("\x0d".to_string()), //13
        ("n", Modifiers::Ctrl) => Some("\x0e".to_string()), //14
        ("N", Modifiers::CtrlShift) => Some("\x0e".to_string()), //14
        ("o", Modifiers::Ctrl) => Some("\x0f".to_string()), //15
        ("O", Modifiers::CtrlShift) => Some("\x0f".to_string()), //15
        ("p", Modifiers::Ctrl) => Some("\x10".to_string()), //16
        ("P", Modifiers::CtrlShift) => Some("\x10".to_string()), //16
        ("q", Modifiers::Ctrl) => Some("\x11".to_string()), //17
        ("Q", Modifiers::CtrlShift) => Some("\x11".to_string()), //17
        ("r", Modifiers::Ctrl) => Some("\x12".to_string()), //18
        ("R", Modifiers::CtrlShift) => Some("\x12".to_string()), //18
        ("s", Modifiers::Ctrl) => Some("\x13".to_string()), //19
        ("S", Modifiers::CtrlShift) => Some("\x13".to_string()), //19
        ("t", Modifiers::Ctrl) => Some("\x14".to_string()), //20
        ("T", Modifiers::CtrlShift) => Some("\x14".to_string()), //20
        ("u", Modifiers::Ctrl) => Some("\x15".to_string()), //21
        ("U", Modifiers::CtrlShift) => Some("\x15".to_string()), //21
        ("v", Modifiers::Ctrl) => Some("\x16".to_string()), //22
        ("V", Modifiers::CtrlShift) => Some("\x16".to_string()), //22
        ("w", Modifiers::Ctrl) => Some("\x17".to_string()), //23
        ("W", Modifiers::CtrlShift) => Some("\x17".to_string()), //23
        ("x", Modifiers::Ctrl) => Some("\x18".to_string()), //24
        ("X", Modifiers::CtrlShift) => Some("\x18".to_string()), //24
        ("y", Modifiers::Ctrl) => Some("\x19".to_string()), //25
        ("Y", Modifiers::CtrlShift) => Some("\x19".to_string()), //25
        ("z", Modifiers::Ctrl) => Some("\x1a".to_string()), //26
        ("Z", Modifiers::CtrlShift) => Some("\x1a".to_string()), //26
        ("@", Modifiers::Ctrl) => Some("\x00".to_string()), //0
        ("[", Modifiers::Ctrl) => Some("\x1b".to_string()), //27
        ("\\", Modifiers::Ctrl) => Some("\x1c".to_string()), //28
        ("]", Modifiers::Ctrl) => Some("\x1d".to_string()), //29
        ("^", Modifiers::Ctrl) => Some("\x1e".to_string()), //30
        ("_", Modifiers::Ctrl) => Some("\x1f".to_string()), //31
        ("?", Modifiers::Ctrl) => Some("\x7f".to_string()), //127
        _ => None,
    };
    if manual_esc_str.is_some() {
        return manual_esc_str;
    }

    // Automated bindings applying modifiers
    if modifiers.any() {
        let modifier_code = modifier_code(&keystroke);
        let modified_esc_str = match keystroke.key.as_ref() {
            "up" => Some(format!("\x1b[1;{}A", modifier_code)),
            "down" => Some(format!("\x1b[1;{}B", modifier_code)),
            "right" => Some(format!("\x1b[1;{}C", modifier_code)),
            "left" => Some(format!("\x1b[1;{}D", modifier_code)),
            "f1" => Some(format!("\x1b[1;{}P", modifier_code)),
            "f2" => Some(format!("\x1b[1;{}Q", modifier_code)),
            "f3" => Some(format!("\x1b[1;{}R", modifier_code)),
            "f4" => Some(format!("\x1b[1;{}S", modifier_code)),
            "F5" => Some(format!("\x1b[15;{}~", modifier_code)),
            "f6" => Some(format!("\x1b[17;{}~", modifier_code)),
            "f7" => Some(format!("\x1b[18;{}~", modifier_code)),
            "f8" => Some(format!("\x1b[19;{}~", modifier_code)),
            "f9" => Some(format!("\x1b[20;{}~", modifier_code)),
            "f10" => Some(format!("\x1b[21;{}~", modifier_code)),
            "f11" => Some(format!("\x1b[23;{}~", modifier_code)),
            "f12" => Some(format!("\x1b[24;{}~", modifier_code)),
            "f13" => Some(format!("\x1b[25;{}~", modifier_code)),
            "f14" => Some(format!("\x1b[26;{}~", modifier_code)),
            "f15" => Some(format!("\x1b[28;{}~", modifier_code)),
            "f16" => Some(format!("\x1b[29;{}~", modifier_code)),
            "f17" => Some(format!("\x1b[31;{}~", modifier_code)),
            "f18" => Some(format!("\x1b[32;{}~", modifier_code)),
            "f19" => Some(format!("\x1b[33;{}~", modifier_code)),
            "f20" => Some(format!("\x1b[34;{}~", modifier_code)),
            _ if modifier_code == 2 => None,
            "insert" => Some(format!("\x1b[2;{}~", modifier_code)),
            "pageup" => Some(format!("\x1b[5;{}~", modifier_code)),
            "pagedown" => Some(format!("\x1b[6;{}~", modifier_code)),
            "end" => Some(format!("\x1b[1;{}F", modifier_code)),
            "home" => Some(format!("\x1b[1;{}H", modifier_code)),
            _ => None,
        };
        if modified_esc_str.is_some() {
            return modified_esc_str;
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
    if keystroke.shift {
        modifier_code |= 1;
    }
    if keystroke.alt {
        modifier_code |= 1 << 1;
    }
    if keystroke.ctrl {
        modifier_code |= 1 << 2;
    }
    modifier_code + 1
}

#[cfg(test)]
mod test {
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
        assert_eq!(to_esc_str(&shift_pageup, &none), None);
        assert_eq!(to_esc_str(&shift_pagedown, &none), None);
        assert_eq!(to_esc_str(&shift_home, &none), None);
        assert_eq!(to_esc_str(&shift_end, &none), None);

        let alt_screen = TermMode::ALT_SCREEN;
        assert_eq!(
            to_esc_str(&shift_pageup, &alt_screen),
            Some("\x1b[5;2~".to_string())
        );
        assert_eq!(
            to_esc_str(&shift_pagedown, &alt_screen),
            Some("\x1b[6;2~".to_string())
        );
        assert_eq!(
            to_esc_str(&shift_home, &alt_screen),
            Some("\x1b[1;2H".to_string())
        );
        assert_eq!(
            to_esc_str(&shift_end, &alt_screen),
            Some("\x1b[1;2F".to_string())
        );

        let pageup = Keystroke::parse("pageup").unwrap();
        let pagedown = Keystroke::parse("pagedown").unwrap();
        let any = TermMode::ANY;

        assert_eq!(to_esc_str(&pageup, &any), Some("\x1b[5~".to_string()));
        assert_eq!(to_esc_str(&pagedown, &any), Some("\x1b[6~".to_string()));
    }

    #[test]
    fn test_plain_inputs() {
        let ks = Keystroke {
            ctrl: false,
            alt: false,
            shift: false,
            cmd: false,
            key: "🖖🏻".to_string(), //2 char string
        };
        assert_eq!(to_esc_str(&ks, &TermMode::NONE), None);
    }

    #[test]
    fn test_application_mode() {
        let app_cursor = TermMode::APP_CURSOR;
        let none = TermMode::NONE;

        let up = Keystroke::parse("up").unwrap();
        let down = Keystroke::parse("down").unwrap();
        let left = Keystroke::parse("left").unwrap();
        let right = Keystroke::parse("right").unwrap();

        assert_eq!(to_esc_str(&up, &none), Some("\x1b[A".to_string()));
        assert_eq!(to_esc_str(&down, &none), Some("\x1b[B".to_string()));
        assert_eq!(to_esc_str(&right, &none), Some("\x1b[C".to_string()));
        assert_eq!(to_esc_str(&left, &none), Some("\x1b[D".to_string()));

        assert_eq!(to_esc_str(&up, &app_cursor), Some("\x1bOA".to_string()));
        assert_eq!(to_esc_str(&down, &app_cursor), Some("\x1bOB".to_string()));
        assert_eq!(to_esc_str(&right, &app_cursor), Some("\x1bOC".to_string()));
        assert_eq!(to_esc_str(&left, &app_cursor), Some("\x1bOD".to_string()));
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
                    &mode
                ),
                to_esc_str(
                    &Keystroke::parse(&format!("ctrl-shift-{}", upper)).unwrap(),
                    &mode
                ),
                "On letter: {}/{}",
                lower,
                upper
            )
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
