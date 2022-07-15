use alacritty_terminal::term::TermMode;
use gpui::keymap::Keystroke;

pub enum ModifierCombinations {
    None,
    Alt,
    Ctrl,
    Shift,
    CtrlShift,
    Other,
}

impl ModifierCombinations {
    fn new(ks: &Keystroke) -> Self {
        match (ks.alt, ks.ctrl, ks.shift, ks.cmd) {
            (false, false, false, false) => ModifierCombinations::None,
            (true, false, false, false) => ModifierCombinations::Alt,
            (false, true, false, false) => ModifierCombinations::Ctrl,
            (false, false, true, false) => ModifierCombinations::Shift,
            (false, true, true, false) => ModifierCombinations::CtrlShift,
            _ => ModifierCombinations::Other,
        }
    }
}

pub fn to_esc_str(keystroke: &Keystroke, mode: &TermMode) -> Option<String> {
    let modifiers = ModifierCombinations::new(&keystroke);

    // Manual Bindings including modifiers
    let manual_esc_str = match (keystroke.key.as_ref(), modifiers) {
        //Basic special keys
        ("space", ModifierCombinations::None) => Some(" ".to_string()),
        ("tab", ModifierCombinations::None) => Some("\x09".to_string()),
        ("escape", ModifierCombinations::None) => Some("\x1b".to_string()),
        ("enter", ModifierCombinations::None) => Some("\x0d".to_string()),
        ("backspace", ModifierCombinations::None) => Some("\x7f".to_string()),
        //Interesting escape codes
        ("tab", ModifierCombinations::Shift) => Some("\x1b[Z".to_string()),
        ("backspace", ModifierCombinations::Alt) => Some("\x1b\x7f".to_string()),
        ("backspace", ModifierCombinations::Shift) => Some("\x7f".to_string()),
        ("home", ModifierCombinations::Shift) if mode.contains(TermMode::ALT_SCREEN) => {
            Some("\x1b[1;2H".to_string())
        }
        ("end", ModifierCombinations::Shift) if mode.contains(TermMode::ALT_SCREEN) => {
            Some("\x1b[1;2F".to_string())
        }
        ("pageup", ModifierCombinations::Shift) if mode.contains(TermMode::ALT_SCREEN) => {
            Some("\x1b[5;2~".to_string())
        }
        ("pagedown", ModifierCombinations::Shift) if mode.contains(TermMode::ALT_SCREEN) => {
            Some("\x1b[6;2~".to_string())
        }
        ("home", ModifierCombinations::None) if mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1bOH".to_string())
        }
        ("home", ModifierCombinations::None) if !mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1b[H".to_string())
        }
        ("end", ModifierCombinations::None) if mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1bOF".to_string())
        }
        ("end", ModifierCombinations::None) if !mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1b[F".to_string())
        }
        ("up", ModifierCombinations::None) if mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1bOA".to_string())
        }
        ("up", ModifierCombinations::None) if !mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1b[A".to_string())
        }
        ("down", ModifierCombinations::None) if mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1bOB".to_string())
        }
        ("down", ModifierCombinations::None) if !mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1b[B".to_string())
        }
        ("right", ModifierCombinations::None) if mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1bOC".to_string())
        }
        ("right", ModifierCombinations::None) if !mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1b[C".to_string())
        }
        ("left", ModifierCombinations::None) if mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1bOD".to_string())
        }
        ("left", ModifierCombinations::None) if !mode.contains(TermMode::APP_CURSOR) => {
            Some("\x1b[D".to_string())
        }
        ("back", ModifierCombinations::None) => Some("\x7f".to_string()),
        ("insert", ModifierCombinations::None) => Some("\x1b[2~".to_string()),
        ("delete", ModifierCombinations::None) => Some("\x1b[3~".to_string()),
        ("pageup", ModifierCombinations::None) => Some("\x1b[5~".to_string()),
        ("pagedown", ModifierCombinations::None) => Some("\x1b[6~".to_string()),
        ("f1", ModifierCombinations::None) => Some("\x1bOP".to_string()),
        ("f2", ModifierCombinations::None) => Some("\x1bOQ".to_string()),
        ("f3", ModifierCombinations::None) => Some("\x1bOR".to_string()),
        ("f4", ModifierCombinations::None) => Some("\x1bOS".to_string()),
        ("f5", ModifierCombinations::None) => Some("\x1b[15~".to_string()),
        ("f6", ModifierCombinations::None) => Some("\x1b[17~".to_string()),
        ("f7", ModifierCombinations::None) => Some("\x1b[18~".to_string()),
        ("f8", ModifierCombinations::None) => Some("\x1b[19~".to_string()),
        ("f9", ModifierCombinations::None) => Some("\x1b[20~".to_string()),
        ("f10", ModifierCombinations::None) => Some("\x1b[21~".to_string()),
        ("f11", ModifierCombinations::None) => Some("\x1b[23~".to_string()),
        ("f12", ModifierCombinations::None) => Some("\x1b[24~".to_string()),
        ("f13", ModifierCombinations::None) => Some("\x1b[25~".to_string()),
        ("f14", ModifierCombinations::None) => Some("\x1b[26~".to_string()),
        ("f15", ModifierCombinations::None) => Some("\x1b[28~".to_string()),
        ("f16", ModifierCombinations::None) => Some("\x1b[29~".to_string()),
        ("f17", ModifierCombinations::None) => Some("\x1b[31~".to_string()),
        ("f18", ModifierCombinations::None) => Some("\x1b[32~".to_string()),
        ("f19", ModifierCombinations::None) => Some("\x1b[33~".to_string()),
        ("f20", ModifierCombinations::None) => Some("\x1b[34~".to_string()),
        // NumpadEnter, Action::Esc("\n".into());
        //Mappings for caret notation keys
        ("a", ModifierCombinations::Ctrl) => Some("\x01".to_string()), //1
        ("A", ModifierCombinations::CtrlShift) => Some("\x01".to_string()), //1
        ("b", ModifierCombinations::Ctrl) => Some("\x02".to_string()), //2
        ("B", ModifierCombinations::CtrlShift) => Some("\x02".to_string()), //2
        ("c", ModifierCombinations::Ctrl) => Some("\x03".to_string()), //3
        ("C", ModifierCombinations::CtrlShift) => Some("\x03".to_string()), //3
        ("d", ModifierCombinations::Ctrl) => Some("\x04".to_string()), //4
        ("D", ModifierCombinations::CtrlShift) => Some("\x04".to_string()), //4
        ("e", ModifierCombinations::Ctrl) => Some("\x05".to_string()), //5
        ("E", ModifierCombinations::CtrlShift) => Some("\x05".to_string()), //5
        ("f", ModifierCombinations::Ctrl) => Some("\x06".to_string()), //6
        ("F", ModifierCombinations::CtrlShift) => Some("\x06".to_string()), //6
        ("g", ModifierCombinations::Ctrl) => Some("\x07".to_string()), //7
        ("G", ModifierCombinations::CtrlShift) => Some("\x07".to_string()), //7
        ("h", ModifierCombinations::Ctrl) => Some("\x08".to_string()), //8
        ("H", ModifierCombinations::CtrlShift) => Some("\x08".to_string()), //8
        ("i", ModifierCombinations::Ctrl) => Some("\x09".to_string()), //9
        ("I", ModifierCombinations::CtrlShift) => Some("\x09".to_string()), //9
        ("j", ModifierCombinations::Ctrl) => Some("\x0a".to_string()), //10
        ("J", ModifierCombinations::CtrlShift) => Some("\x0a".to_string()), //10
        ("k", ModifierCombinations::Ctrl) => Some("\x0b".to_string()), //11
        ("K", ModifierCombinations::CtrlShift) => Some("\x0b".to_string()), //11
        ("l", ModifierCombinations::Ctrl) => Some("\x0c".to_string()), //12
        ("L", ModifierCombinations::CtrlShift) => Some("\x0c".to_string()), //12
        ("m", ModifierCombinations::Ctrl) => Some("\x0d".to_string()), //13
        ("M", ModifierCombinations::CtrlShift) => Some("\x0d".to_string()), //13
        ("n", ModifierCombinations::Ctrl) => Some("\x0e".to_string()), //14
        ("N", ModifierCombinations::CtrlShift) => Some("\x0e".to_string()), //14
        ("o", ModifierCombinations::Ctrl) => Some("\x0f".to_string()), //15
        ("O", ModifierCombinations::CtrlShift) => Some("\x0f".to_string()), //15
        ("p", ModifierCombinations::Ctrl) => Some("\x10".to_string()), //16
        ("P", ModifierCombinations::CtrlShift) => Some("\x10".to_string()), //16
        ("q", ModifierCombinations::Ctrl) => Some("\x11".to_string()), //17
        ("Q", ModifierCombinations::CtrlShift) => Some("\x11".to_string()), //17
        ("r", ModifierCombinations::Ctrl) => Some("\x12".to_string()), //18
        ("R", ModifierCombinations::CtrlShift) => Some("\x12".to_string()), //18
        ("s", ModifierCombinations::Ctrl) => Some("\x13".to_string()), //19
        ("S", ModifierCombinations::CtrlShift) => Some("\x13".to_string()), //19
        ("t", ModifierCombinations::Ctrl) => Some("\x14".to_string()), //20
        ("T", ModifierCombinations::CtrlShift) => Some("\x14".to_string()), //20
        ("u", ModifierCombinations::Ctrl) => Some("\x15".to_string()), //21
        ("U", ModifierCombinations::CtrlShift) => Some("\x15".to_string()), //21
        ("v", ModifierCombinations::Ctrl) => Some("\x16".to_string()), //22
        ("V", ModifierCombinations::CtrlShift) => Some("\x16".to_string()), //22
        ("w", ModifierCombinations::Ctrl) => Some("\x17".to_string()), //23
        ("W", ModifierCombinations::CtrlShift) => Some("\x17".to_string()), //23
        ("x", ModifierCombinations::Ctrl) => Some("\x18".to_string()), //24
        ("X", ModifierCombinations::CtrlShift) => Some("\x18".to_string()), //24
        ("y", ModifierCombinations::Ctrl) => Some("\x19".to_string()), //25
        ("Y", ModifierCombinations::CtrlShift) => Some("\x19".to_string()), //25
        ("z", ModifierCombinations::Ctrl) => Some("\x1a".to_string()), //26
        ("Z", ModifierCombinations::CtrlShift) => Some("\x1a".to_string()), //26
        ("@", ModifierCombinations::Ctrl) => Some("\x00".to_string()), //0
        ("[", ModifierCombinations::Ctrl) => Some("\x1b".to_string()), //27
        ("\\", ModifierCombinations::Ctrl) => Some("\x1c".to_string()), //28
        ("]", ModifierCombinations::Ctrl) => Some("\x1d".to_string()), //29
        ("^", ModifierCombinations::Ctrl) => Some("\x1e".to_string()), //30
        ("_", ModifierCombinations::Ctrl) => Some("\x1f".to_string()), //31
        ("?", ModifierCombinations::Ctrl) => Some("\x7f".to_string()), //127
        _ => None,
    };
    if manual_esc_str.is_some() {
        return manual_esc_str;
    }

    // Automated bindings applying modifiers
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

    // Fallback to keystroke input sent directly
    if keystroke.key.chars().count() == 1 {
        dbg!("This should catch space", &keystroke.key);
        return Some(keystroke.key.clone());
    } else {
        None
    }
}

/*
New keybindings test plan:

Is the terminal still usable?  YES!
Do ctrl-shift-[X] and ctrl-[x] do the same thing? I THINK SO
Does ctrl-l work? YES
Does tab work? YES
Do all the global overrides (up, down, enter, escape, ctrl-c) work? => YES
Space also doesn't work YES!



So, to match  alacritty keyboard handling, we need to check APP_CURSOR, and ALT_SCREEN

And we need to convert the strings that GPUI returns to keys

And we need a way of easily declaring and matching a modifier pattern on those keys

And we need to block writing the input to the pty if any of these match

And I need to figure out how to express this in a cross platform way

And a way of optionally interfacing this with actions for rebinding in defaults.json

Design notes:
I would like terminal mode checking to be concealed behind the TerminalConnection in as many ways as possible.
Alacritty has a lot of stuff intermixed for it's input handling. TerminalConnection should be in charge
of anything that needs to conform to a standard that isn't handled by Term, e.g.:
- Reporting mouse events correctly.
- Reporting scrolls -> Depends on MOUSE_MODE, ALT_SCREEN, and ALTERNATE_SCROLL, etc.
- Correctly bracketing a paste
- Storing changed colors
- Focus change sequence

Scrolling might be handled internally or externally, need a way to ask. Everything else should probably happen internally.

Standards/OS compliance is in connection.rs.
This takes GPUI events and translates them to the correct terminal stuff
This means that standards compliance outside of connection should be kept to a minimum. Yes, this feels good.
Connection needs to be split up then, into a bunch of event handlers

Punting on these by pushing them up to a scrolling element
(either on dispatch_event directly or a seperate scrollbar)
        Home,     ModifiersState::SHIFT, ~BindingMode::ALT_SCREEN; Action::ScrollToTop;
        End,      ModifiersState::SHIFT, ~BindingMode::ALT_SCREEN; Action::ScrollToBottom;
        PageUp,   ModifiersState::SHIFT, ~BindingMode::ALT_SCREEN; Action::ScrollPageUp;
        PageDown, ModifiersState::SHIFT, ~BindingMode::ALT_SCREEN; Action::ScrollPageDown;



NOTE, THE FOLLOWING HAS 2 BINDINGS:
K, ModifiersState::LOGO, Action::Esc("\x0c".into());
K, ModifiersState::LOGO, Action::ClearHistory; => ctx.terminal_mut().clear_screen(ClearMode::Saved),

*/

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
    fn test_match_alacritty_keybindings() {
        // let bindings = alacritty::config::bindings::default_key_bindings();
        //TODO
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
        // assert_eq!(2, modifier_code(Keystroke::parse("shift-A").unwrap()));
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
