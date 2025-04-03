/// The mappings defined in this file where created from reading the alacritty source
use alacritty_terminal::term::TermMode;
use gpui::{KeyCode, Keystroke, Modifiers, KeyboardMapper};

use collections::HashMap;

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

pub fn generate_esc_str_mapper(
    keyboard_mapper: &KeyboardMapper,
) -> HashMap<(KeyCode, Modifiers), String> {
    let mut esc_str_mapper = HashMap::new();
    //Mappings for caret notation keys
    let (key_a, _) = keyboard_mapper.parse("a", true).unwrap();
    esc_str_mapper.insert((key_a, Modifiers::control()), "\x01".to_string()); //1
    esc_str_mapper.insert((key_a, Modifiers::control_shift()), "\x01".to_string()); //1
    let (key_b, _) = keyboard_mapper.parse("b", true).unwrap();
    esc_str_mapper.insert((key_b, Modifiers::control()), "\x02".to_string()); //2
    esc_str_mapper.insert((key_b, Modifiers::control_shift()), "\x02".to_string()); //2
    let (key_c, _) = keyboard_mapper.parse("c", true).unwrap();
    esc_str_mapper.insert((key_c, Modifiers::control()), "\x03".to_string()); //3
    esc_str_mapper.insert((key_c, Modifiers::control_shift()), "\x03".to_string()); //3
    let (key_d, _) = keyboard_mapper.parse("d", true).unwrap();
    esc_str_mapper.insert((key_d, Modifiers::control()), "\x04".to_string()); //4
    esc_str_mapper.insert((key_d, Modifiers::control_shift()), "\x04".to_string()); //4
    let (key_e, _) = keyboard_mapper.parse("e", true).unwrap();
    esc_str_mapper.insert((key_e, Modifiers::control()), "\x05".to_string()); //5
    esc_str_mapper.insert((key_e, Modifiers::control_shift()), "\x05".to_string()); //5
    let (key_f, _) = keyboard_mapper.parse("f", true).unwrap();
    esc_str_mapper.insert((key_f, Modifiers::control()), "\x06".to_string()); //6
    esc_str_mapper.insert((key_f, Modifiers::control_shift()), "\x06".to_string()); //6
    let (key_g, _) = keyboard_mapper.parse("g", true).unwrap();
    esc_str_mapper.insert((key_g, Modifiers::control()), "\x07".to_string()); //7
    esc_str_mapper.insert((key_g, Modifiers::control_shift()), "\x07".to_string()); //7
    let (key_h, _) = keyboard_mapper.parse("h", true).unwrap();
    esc_str_mapper.insert((key_h, Modifiers::control()), "\x08".to_string()); //8
    esc_str_mapper.insert((key_h, Modifiers::control_shift()), "\x08".to_string()); //8
    let (key_i, _) = keyboard_mapper.parse("i", true).unwrap();
    esc_str_mapper.insert((key_i, Modifiers::control()), "\x09".to_string()); //9
    esc_str_mapper.insert((key_i, Modifiers::control_shift()), "\x09".to_string()); //9
    let (key_j, _) = keyboard_mapper.parse("j", true).unwrap();
    esc_str_mapper.insert((key_j, Modifiers::control()), "\x0a".to_string()); //10
    esc_str_mapper.insert((key_j, Modifiers::control_shift()), "\x0a".to_string()); //10
    let (key_k, _) = keyboard_mapper.parse("k", true).unwrap();
    esc_str_mapper.insert((key_k, Modifiers::control()), "\x0b".to_string()); //11
    esc_str_mapper.insert((key_k, Modifiers::control_shift()), "\x0b".to_string()); //11
    let (key_l, _) = keyboard_mapper.parse("l", true).unwrap();
    esc_str_mapper.insert((key_l, Modifiers::control()), "\x0c".to_string()); //12
    esc_str_mapper.insert((key_l, Modifiers::control_shift()), "\x0c".to_string()); //12
    let (key_m, _) = keyboard_mapper.parse("m", true).unwrap();
    esc_str_mapper.insert((key_m, Modifiers::control()), "\x0d".to_string()); //13
    esc_str_mapper.insert((key_m, Modifiers::control_shift()), "\x0d".to_string()); //13
    let (key_n, _) = keyboard_mapper.parse("n", true).unwrap();
    esc_str_mapper.insert((key_n, Modifiers::control()), "\x0e".to_string()); //14
    esc_str_mapper.insert((key_n, Modifiers::control_shift()), "\x0e".to_string()); //14
    let (key_o, _) = keyboard_mapper.parse("o", true).unwrap();
    esc_str_mapper.insert((key_o, Modifiers::control()), "\x0f".to_string()); //15
    esc_str_mapper.insert((key_o, Modifiers::control_shift()), "\x0f".to_string()); //15
    let (key_p, _) = keyboard_mapper.parse("p", true).unwrap();
    esc_str_mapper.insert((key_p, Modifiers::control()), "\x10".to_string()); //16
    esc_str_mapper.insert((key_p, Modifiers::control_shift()), "\x10".to_string()); //16
    let (key_q, _) = keyboard_mapper.parse("q", true).unwrap();
    esc_str_mapper.insert((key_q, Modifiers::control()), "\x11".to_string()); //17
    esc_str_mapper.insert((key_q, Modifiers::control_shift()), "\x11".to_string()); //17
    let (key_r, _) = keyboard_mapper.parse("r", true).unwrap();
    esc_str_mapper.insert((key_r, Modifiers::control()), "\x12".to_string()); //18
    esc_str_mapper.insert((key_r, Modifiers::control_shift()), "\x12".to_string()); //18
    let (key_s, _) = keyboard_mapper.parse("s", true).unwrap();
    esc_str_mapper.insert((key_s, Modifiers::control()), "\x13".to_string()); //19
    esc_str_mapper.insert((key_s, Modifiers::control_shift()), "\x13".to_string()); //19
    let (key_t, _) = keyboard_mapper.parse("t", true).unwrap();
    esc_str_mapper.insert((key_t, Modifiers::control()), "\x14".to_string()); //20
    esc_str_mapper.insert((key_t, Modifiers::control_shift()), "\x14".to_string()); //20
    let (key_u, _) = keyboard_mapper.parse("u", true).unwrap();
    esc_str_mapper.insert((key_u, Modifiers::control()), "\x15".to_string()); //21
    esc_str_mapper.insert((key_u, Modifiers::control_shift()), "\x15".to_string()); //21
    let (key_v, _) = keyboard_mapper.parse("v", true).unwrap();
    esc_str_mapper.insert((key_v, Modifiers::control()), "\x16".to_string()); //22
    esc_str_mapper.insert((key_v, Modifiers::control_shift()), "\x16".to_string()); //22
    let (key_w, _) = keyboard_mapper.parse("w", true).unwrap();
    esc_str_mapper.insert((key_w, Modifiers::control()), "\x17".to_string()); //23
    esc_str_mapper.insert((key_w, Modifiers::control_shift()), "\x17".to_string()); //23
    let (key_x, _) = keyboard_mapper.parse("x", true).unwrap();
    esc_str_mapper.insert((key_x, Modifiers::control()), "\x18".to_string()); //24
    esc_str_mapper.insert((key_x, Modifiers::control_shift()), "\x18".to_string()); //24
    let (key_y, _) = keyboard_mapper.parse("y", true).unwrap();
    esc_str_mapper.insert((key_y, Modifiers::control()), "\x19".to_string()); //25
    esc_str_mapper.insert((key_y, Modifiers::control_shift()), "\x19".to_string()); //25
    let (key_z, _) = keyboard_mapper.parse("z", true).unwrap();
    esc_str_mapper.insert((key_z, Modifiers::control()), "\x1a".to_string()); //26
    esc_str_mapper.insert((key_z, Modifiers::control_shift()), "\x1a".to_string()); //26
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
    let (key_backslash, modifiers) = keyboard_mapper.parse("\\", true).unwrap();
    esc_str_mapper.insert(
        (key_backslash, modifiers | Modifiers::control()),
        "\x1c".to_string(),
    ); //28
    let (key_right_bracket, modifiers) = keyboard_mapper.parse("]", true).unwrap();
    esc_str_mapper.insert(
        (key_right_bracket, modifiers | Modifiers::control()),
        "\x1d".to_string(),
    ); //29
    let (key_caret, modifiers) = keyboard_mapper.parse("^", true).unwrap();
    esc_str_mapper.insert(
        (key_caret, modifiers | Modifiers::control()),
        "\x1e".to_string(),
    ); //30
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

pub fn to_esc_str(keystroke: &Keystroke, mode: &TermMode, alt_is_meta: bool) -> Option<String> {
    let modifiers = AlacModifiers::new(keystroke);

    // Manual Bindings including modifiers
    // todo(zjk)
    let manual_esc_str = match (keystroke.face.as_ref(), &modifiers) {
        //Basic special keys
        ("tab", AlacModifiers::None) => Some("\x09".to_string()),
        ("escape", AlacModifiers::None) => Some("\x1b".to_string()),
        ("enter", AlacModifiers::None) => Some("\x0d".to_string()),
        ("enter", AlacModifiers::Shift) => Some("\x0d".to_string()),
        ("enter", AlacModifiers::Alt) => Some("\x1b\x0d".to_string()),
        ("backspace", AlacModifiers::None) => Some("\x7f".to_string()),
        //Interesting escape codes
        ("tab", AlacModifiers::Shift) => Some("\x1b[Z".to_string()),
        ("backspace", AlacModifiers::Ctrl) => Some("\x08".to_string()),
        ("backspace", AlacModifiers::Alt) => Some("\x1b\x7f".to_string()),
        ("backspace", AlacModifiers::Shift) => Some("\x7f".to_string()),
        ("space", AlacModifiers::Ctrl) => Some("\x00".to_string()),
        ("home", AlacModifiers::Shift) if mode.contains(TermMode::ALT_SCREEN) => {
            Some("\x1b[1;2H".to_string())
        }
        ("end", AlacModifiers::Shift) if mode.contains(TermMode::ALT_SCREEN) => {
            Some("\x1b[1;2F".to_string())
        }
        ("pageup", AlacModifiers::Shift) if mode.contains(TermMode::ALT_SCREEN) => {
            Some("\x1b[5;2~".to_string())
        }
        ("pagedown", AlacModifiers::Shift) if mode.contains(TermMode::ALT_SCREEN) => {
            Some("\x1b[6;2~".to_string())
        }
        ("home", AlacModifiers::None) if mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1bOH".to_string())
        }
        ("home", AlacModifiers::None) if !mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1b[H".to_string())
        }
        ("end", AlacModifiers::None) if mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1bOF".to_string())
        }
        ("end", AlacModifiers::None) if !mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1b[F".to_string())
        }
        ("up", AlacModifiers::None) if mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1bOA".to_string())
        }
        ("up", AlacModifiers::None) if !mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1b[A".to_string())
        }
        ("down", AlacModifiers::None) if mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1bOB".to_string())
        }
        ("down", AlacModifiers::None) if !mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1b[B".to_string())
        }
        ("right", AlacModifiers::None) if mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1bOC".to_string())
        }
        ("right", AlacModifiers::None) if !mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1b[C".to_string())
        }
        ("left", AlacModifiers::None) if mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1bOD".to_string())
        }
        ("left", AlacModifiers::None) if !mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1b[D".to_string())
        }
        ("back", AlacModifiers::None) => Some("\x7f".to_string()),
        ("insert", AlacModifiers::None) => Some("\x1b[2~".to_string()),
        ("delete", AlacModifiers::None) => Some("\x1b[3~".to_string()),
        ("pageup", AlacModifiers::None) => Some("\x1b[5~".to_string()),
        ("pagedown", AlacModifiers::None) => Some("\x1b[6~".to_string()),
        ("f1", AlacModifiers::None) => Some("\x1bOP".to_string()),
        ("f2", AlacModifiers::None) => Some("\x1bOQ".to_string()),
        ("f3", AlacModifiers::None) => Some("\x1bOR".to_string()),
        ("f4", AlacModifiers::None) => Some("\x1bOS".to_string()),
        ("f5", AlacModifiers::None) => Some("\x1b[15~".to_string()),
        ("f6", AlacModifiers::None) => Some("\x1b[17~".to_string()),
        ("f7", AlacModifiers::None) => Some("\x1b[18~".to_string()),
        ("f8", AlacModifiers::None) => Some("\x1b[19~".to_string()),
        ("f9", AlacModifiers::None) => Some("\x1b[20~".to_string()),
        ("f10", AlacModifiers::None) => Some("\x1b[21~".to_string()),
        ("f11", AlacModifiers::None) => Some("\x1b[23~".to_string()),
        ("f12", AlacModifiers::None) => Some("\x1b[24~".to_string()),
        ("f13", AlacModifiers::None) => Some("\x1b[25~".to_string()),
        ("f14", AlacModifiers::None) => Some("\x1b[26~".to_string()),
        ("f15", AlacModifiers::None) => Some("\x1b[28~".to_string()),
        ("f16", AlacModifiers::None) => Some("\x1b[29~".to_string()),
        ("f17", AlacModifiers::None) => Some("\x1b[31~".to_string()),
        ("f18", AlacModifiers::None) => Some("\x1b[32~".to_string()),
        ("f19", AlacModifiers::None) => Some("\x1b[33~".to_string()),
        ("f20", AlacModifiers::None) => Some("\x1b[34~".to_string()),
        // NumpadEnter, Action::Esc("\n".into());
        //Mappings for caret notation keys
        ("a", AlacModifiers::Ctrl) => Some("\x01".to_string()), //1
        ("A", AlacModifiers::CtrlShift) => Some("\x01".to_string()), //1
        ("b", AlacModifiers::Ctrl) => Some("\x02".to_string()), //2
        ("B", AlacModifiers::CtrlShift) => Some("\x02".to_string()), //2
        ("c", AlacModifiers::Ctrl) => Some("\x03".to_string()), //3
        ("C", AlacModifiers::CtrlShift) => Some("\x03".to_string()), //3
        ("d", AlacModifiers::Ctrl) => Some("\x04".to_string()), //4
        ("D", AlacModifiers::CtrlShift) => Some("\x04".to_string()), //4
        ("e", AlacModifiers::Ctrl) => Some("\x05".to_string()), //5
        ("E", AlacModifiers::CtrlShift) => Some("\x05".to_string()), //5
        ("f", AlacModifiers::Ctrl) => Some("\x06".to_string()), //6
        ("F", AlacModifiers::CtrlShift) => Some("\x06".to_string()), //6
        ("g", AlacModifiers::Ctrl) => Some("\x07".to_string()), //7
        ("G", AlacModifiers::CtrlShift) => Some("\x07".to_string()), //7
        ("h", AlacModifiers::Ctrl) => Some("\x08".to_string()), //8
        ("H", AlacModifiers::CtrlShift) => Some("\x08".to_string()), //8
        ("i", AlacModifiers::Ctrl) => Some("\x09".to_string()), //9
        ("I", AlacModifiers::CtrlShift) => Some("\x09".to_string()), //9
        ("j", AlacModifiers::Ctrl) => Some("\x0a".to_string()), //10
        ("J", AlacModifiers::CtrlShift) => Some("\x0a".to_string()), //10
        ("k", AlacModifiers::Ctrl) => Some("\x0b".to_string()), //11
        ("K", AlacModifiers::CtrlShift) => Some("\x0b".to_string()), //11
        ("l", AlacModifiers::Ctrl) => Some("\x0c".to_string()), //12
        ("L", AlacModifiers::CtrlShift) => Some("\x0c".to_string()), //12
        ("m", AlacModifiers::Ctrl) => Some("\x0d".to_string()), //13
        ("M", AlacModifiers::CtrlShift) => Some("\x0d".to_string()), //13
        ("n", AlacModifiers::Ctrl) => Some("\x0e".to_string()), //14
        ("N", AlacModifiers::CtrlShift) => Some("\x0e".to_string()), //14
        ("o", AlacModifiers::Ctrl) => Some("\x0f".to_string()), //15
        ("O", AlacModifiers::CtrlShift) => Some("\x0f".to_string()), //15
        ("p", AlacModifiers::Ctrl) => Some("\x10".to_string()), //16
        ("P", AlacModifiers::CtrlShift) => Some("\x10".to_string()), //16
        ("q", AlacModifiers::Ctrl) => Some("\x11".to_string()), //17
        ("Q", AlacModifiers::CtrlShift) => Some("\x11".to_string()), //17
        ("r", AlacModifiers::Ctrl) => Some("\x12".to_string()), //18
        ("R", AlacModifiers::CtrlShift) => Some("\x12".to_string()), //18
        ("s", AlacModifiers::Ctrl) => Some("\x13".to_string()), //19
        ("S", AlacModifiers::CtrlShift) => Some("\x13".to_string()), //19
        ("t", AlacModifiers::Ctrl) => Some("\x14".to_string()), //20
        ("T", AlacModifiers::CtrlShift) => Some("\x14".to_string()), //20
        ("u", AlacModifiers::Ctrl) => Some("\x15".to_string()), //21
        ("U", AlacModifiers::CtrlShift) => Some("\x15".to_string()), //21
        ("v", AlacModifiers::Ctrl) => Some("\x16".to_string()), //22
        ("V", AlacModifiers::CtrlShift) => Some("\x16".to_string()), //22
        ("w", AlacModifiers::Ctrl) => Some("\x17".to_string()), //23
        ("W", AlacModifiers::CtrlShift) => Some("\x17".to_string()), //23
        ("x", AlacModifiers::Ctrl) => Some("\x18".to_string()), //24
        ("X", AlacModifiers::CtrlShift) => Some("\x18".to_string()), //24
        ("y", AlacModifiers::Ctrl) => Some("\x19".to_string()), //25
        ("Y", AlacModifiers::CtrlShift) => Some("\x19".to_string()), //25
        ("z", AlacModifiers::Ctrl) => Some("\x1a".to_string()), //26
        ("Z", AlacModifiers::CtrlShift) => Some("\x1a".to_string()), //26
        ("@", AlacModifiers::Ctrl) => Some("\x00".to_string()), //0
        ("[", AlacModifiers::Ctrl) => Some("\x1b".to_string()), //27
        ("\\", AlacModifiers::Ctrl) => Some("\x1c".to_string()), //28
        ("]", AlacModifiers::Ctrl) => Some("\x1d".to_string()), //29
        ("^", AlacModifiers::Ctrl) => Some("\x1e".to_string()), //30
        ("_", AlacModifiers::Ctrl) => Some("\x1f".to_string()), //31
        ("?", AlacModifiers::Ctrl) => Some("\x7f".to_string()), //127
        _ => None,
    };
    if manual_esc_str.is_some() {
        return manual_esc_str;
    }

    // Automated bindings applying modifiers
    if modifiers.any() {
        let modifier_code = modifier_code(keystroke);
        // todo(zjk)
        let modified_esc_str = match keystroke.face.as_ref() {
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

    if alt_is_meta {
        let is_alt_lowercase_ascii = modifiers == AlacModifiers::Alt && keystroke.key.is_ascii();
        let is_alt_uppercase_ascii =
            keystroke.modifiers.alt && keystroke.modifiers.shift && keystroke.key.is_ascii();
        if is_alt_lowercase_ascii || is_alt_uppercase_ascii {
            let key = if is_alt_uppercase_ascii {
                &keystroke.key.to_ascii_uppercase()
            } else {
                &keystroke.key
            };
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
