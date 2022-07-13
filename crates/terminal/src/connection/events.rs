use alacritty_terminal::term::TermMode;
use gpui::KeyDownEvent;

pub fn _to_esc_str(event: &KeyDownEvent, _mode: TermMode) -> String {
    let key = event.keystroke.key.clone();
    let modifiers = (
        event.keystroke.alt,
        event.keystroke.cmd,
        event.keystroke.ctrl,
        event.keystroke.shift,
    );
    match (key.as_str(), modifiers) {
        //NOTE TO SELF: Terminals can rewrite the color index with OSC, use alacritty colors properly.

        //ctrl-l
        //shift-tab
        //alt-back
        //shift-back
        //shift + Home, end, page up, page down + NOT alt screen => We handle those
        //shift + Home, end, page up, page down + alt screen => Send escape sequence
        ("l", (false, false, true, false)) => "\x0c".to_string(),
        _ => event.input.clone().unwrap().clone(),
    }
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

NOTE, THE FOLLOWING HAS 2 BINDINGS:
K, ModifiersState::LOGO, Action::Esc("\x0c".into());
K, ModifiersState::LOGO, Action::ClearHistory; => ctx.terminal_mut().clear_screen(ClearMode::Saved),


Handled in therminal:
L,    ModifiersState::CTRL, Action::Esc("\x0c".into());
Tab,  ModifiersState::SHIFT, Action::Esc("\x1b[Z".into());
Backspace, ModifiersState::ALT, Action::Esc("\x1b\x7f".into());
Backspace, ModifiersState::SHIFT, Action::Esc("\x7f".into());
Home,     ModifiersState::SHIFT, +BindingMode::ALT_SCREEN, Action::Esc("\x1b[1;2H".into());
End,      ModifiersState::SHIFT, +BindingMode::ALT_SCREEN, Action::Esc("\x1b[1;2F".into());
PageUp,   ModifiersState::SHIFT, +BindingMode::ALT_SCREEN, Action::Esc("\x1b[5;2~".into());
PageDown, ModifiersState::SHIFT, +BindingMode::ALT_SCREEN, Action::Esc("\x1b[6;2~".into());
Home,  +BindingMode::APP_CURSOR, Action::Esc("\x1bOH".into());
Home,  ~BindingMode::APP_CURSOR, Action::Esc("\x1b[H".into());
End,   +BindingMode::APP_CURSOR, Action::Esc("\x1bOF".into());
End,   ~BindingMode::APP_CURSOR, Action::Esc("\x1b[F".into());
Up,    +BindingMode::APP_CURSOR, Action::Esc("\x1bOA".into());
Up,    ~BindingMode::APP_CURSOR, Action::Esc("\x1b[A".into());
Down,  +BindingMode::APP_CURSOR, Action::Esc("\x1bOB".into());
Down,  ~BindingMode::APP_CURSOR, Action::Esc("\x1b[B".into());
Right, +BindingMode::APP_CURSOR, Action::Esc("\x1bOC".into());
Right, ~BindingMode::APP_CURSOR, Action::Esc("\x1b[C".into());
Left,  +BindingMode::APP_CURSOR, Action::Esc("\x1bOD".into());
Left,  ~BindingMode::APP_CURSOR, Action::Esc("\x1b[D".into());
Back,        Action::Esc("\x7f".into());
Insert,      Action::Esc("\x1b[2~".into());
Delete,      Action::Esc("\x1b[3~".into());
PageUp,      Action::Esc("\x1b[5~".into());
PageDown,    Action::Esc("\x1b[6~".into());
F1,          Action::Esc("\x1bOP".into());
F2,          Action::Esc("\x1bOQ".into());
F3,          Action::Esc("\x1bOR".into());
F4,          Action::Esc("\x1bOS".into());
F5,          Action::Esc("\x1b[15~".into());
F6,          Action::Esc("\x1b[17~".into());
F7,          Action::Esc("\x1b[18~".into());
F8,          Action::Esc("\x1b[19~".into());
F9,          Action::Esc("\x1b[20~".into());
F10,         Action::Esc("\x1b[21~".into());
F11,         Action::Esc("\x1b[23~".into());
F12,         Action::Esc("\x1b[24~".into());
F13,         Action::Esc("\x1b[25~".into());
F14,         Action::Esc("\x1b[26~".into());
F15,         Action::Esc("\x1b[28~".into());
F16,         Action::Esc("\x1b[29~".into());
F17,         Action::Esc("\x1b[31~".into());
F18,         Action::Esc("\x1b[32~".into());
F19,         Action::Esc("\x1b[33~".into());
F20,         Action::Esc("\x1b[34~".into());
NumpadEnter, Action::Esc("\n".into());

MAC:
Insert, ModifiersState::SHIFT,  Action::Esc("\x1b[2;2~".into());

*/
