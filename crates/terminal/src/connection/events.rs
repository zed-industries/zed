use alacritty_terminal::term::TermMode;
use gpui::{keymap::Keystroke, KeyDownEvent};

pub enum ModifierCombinations {
    None,
    Alt,
    Ctrl,
    Shift,
    Other,
}

impl ModifierCombinations {
    fn new(ks: &Keystroke) -> Self {
        match (ks.alt, ks.ctrl, ks.shift, ks.cmd) {
            (false, false, false, false) => ModifierCombinations::None,
            (true, false, false, false) => ModifierCombinations::Alt,
            (false, true, false, false) => ModifierCombinations::Ctrl,
            (false, false, true, false) => ModifierCombinations::Shift,
            _ => ModifierCombinations::Other,
        }
    }
}

pub fn to_esc_str(event: &KeyDownEvent, mode: &TermMode) -> Option<String> {
    let modifiers = ModifierCombinations::new(&event.keystroke);

    // Manual Bindings including modifiers
    let manual_esc_str = match (event.keystroke.key.as_ref(), modifiers) {
        ("l", ModifierCombinations::Ctrl) => Some("\x0c".to_string()),
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
        _ => None,
    };
    if manual_esc_str.is_some() {
        return manual_esc_str;
    }

    // Automated bindings applying modifiers
    let modifier_code = modifier_code(&event.keystroke);
    let modified_esc_str = match event.keystroke.key.as_ref() {
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
    return event.input.clone();
}

/*
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
