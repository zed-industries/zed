//! Conversion from `android-activity` input events to GPUI's [`PlatformInput`].
//!
//! On Android, primary input is touch (single or multi-finger) plus an optional
//! hardware keyboard. We map a single touch finger onto the left mouse button
//! so existing GPUI gesture/click code "just works"; multi-touch is collapsed
//! to whichever pointer is the most recent (a placeholder until a real
//! `Touch` event surface is added to GPUI).

use android_activity::input::{InputEvent, KeyAction, KeyEvent, Keycode, MotionAction, MotionEvent};
use gpui::{
    KeyDownEvent, KeyUpEvent, Keystroke, Modifiers, MouseButton, MouseDownEvent, MouseMoveEvent,
    MouseUpEvent, PlatformInput, Point, px,
};

/// Outcome of translating one android-activity input event.
pub(crate) enum Translated {
    /// Touch / mouse / stylus events. Carries the source `MotionEvent`'s
    /// `event_time` (nanoseconds since boot) so downstream code can compute
    /// release velocities — `Instant::now()` is wrong here because
    /// android-activity delivers a whole batch of pointer events in a
    /// single `InputAvailable` poll, so all the wall-clock timestamps would
    /// land within a millisecond of each other regardless of when the
    /// finger was actually at each position.
    Motion {
        events: Vec<PlatformInput>,
        event_time_nanos: i64,
    },
    /// Hardware keyboard input.
    Key(PlatformInput),
    /// IME state delivered via `MainEvent::InputAvailable` + `TextEvent`.
    /// Routed to `PlatformInputHandler` instead of the input callback.
    TextState(android_activity::input::TextInputState),
    /// No-op event (text-action button, unrecognized variants, etc.).
    None,
}

/// Translates a single android-activity input event for routing.
///
/// `scale_factor` is the window's logical-pixel scale. Android delivers
/// motion-event coordinates in *physical* pixels; GPUI works in logical
/// `Pixels`, so we divide by the scale factor here.
pub(crate) fn translate(event: &InputEvent, scale_factor: f32) -> Translated {
    match event {
        InputEvent::MotionEvent(motion) => Translated::Motion {
            events: translate_motion(motion, scale_factor),
            event_time_nanos: motion.event_time(),
        },
        InputEvent::KeyEvent(key) => match translate_key(key) {
            Some(input) => Translated::Key(input),
            None => Translated::None,
        },
        InputEvent::TextEvent(state) => Translated::TextState(state.clone()),
        // android-activity adds new variants over time; fall back to no-op.
        _ => Translated::None,
    }
}

fn translate_motion(event: &MotionEvent, scale_factor: f32) -> Vec<PlatformInput> {
    // Find the active pointer. For ACTION_POINTER_DOWN/UP we want the index
    // encoded in the action; otherwise we use pointer 0.
    let pointer_index = match event.action() {
        MotionAction::PointerDown | MotionAction::PointerUp => event.pointer_index(),
        _ => 0,
    };
    let Some(pointer) = event.pointers().nth(pointer_index) else {
        return Vec::new();
    };
    let position = Point {
        x: px(pointer.x() / scale_factor),
        y: px(pointer.y() / scale_factor),
    };
    let modifiers = Modifiers::default();

    match event.action() {
        MotionAction::Down | MotionAction::PointerDown => {
            vec![PlatformInput::MouseDown(MouseDownEvent {
                button: MouseButton::Left,
                position,
                modifiers,
                click_count: 1,
                first_mouse: false,
            })]
        }
        MotionAction::Up | MotionAction::PointerUp => {
            vec![PlatformInput::MouseUp(MouseUpEvent {
                button: MouseButton::Left,
                position,
                modifiers,
                click_count: 1,
            })]
        }
        MotionAction::Move => vec![PlatformInput::MouseMove(MouseMoveEvent {
            position,
            pressed_button: Some(MouseButton::Left),
            modifiers,
        })],
        MotionAction::HoverMove => vec![PlatformInput::MouseMove(MouseMoveEvent {
            position,
            pressed_button: None,
            modifiers,
        })],
        MotionAction::Cancel => vec![PlatformInput::MouseUp(MouseUpEvent {
            button: MouseButton::Left,
            position,
            modifiers,
            click_count: 0,
        })],
        _ => Vec::new(),
    }
}

fn translate_key(event: &KeyEvent) -> Option<PlatformInput> {
    let keystroke = key_to_keystroke(event)?;
    match event.action() {
        KeyAction::Down => Some(PlatformInput::KeyDown(KeyDownEvent {
            keystroke,
            is_held: event.repeat_count() > 0,
            prefer_character_input: false,
        })),
        KeyAction::Up => Some(PlatformInput::KeyUp(KeyUpEvent { keystroke })),
        // android-activity may report multi-key events; ignore them for v0.
        _ => None,
    }
}

fn key_to_keystroke(event: &KeyEvent) -> Option<Keystroke> {
    let key = key_name(event.key_code())?;
    let meta_state = event.meta_state();
    let modifiers = Modifiers {
        control: meta_state.ctrl_on(),
        alt: meta_state.alt_on(),
        shift: meta_state.shift_on(),
        platform: meta_state.meta_on(),
        // Android's MetaState has no Fn-key bit; physical fn keys ride
        // through KEYCODE_FUNCTION instead.
        function: false,
    };
    let key_char = if !modifiers.control && !modifiers.alt && !modifiers.platform {
        printable_char(event.key_code(), modifiers.shift).map(|c| c.to_string())
    } else {
        None
    };
    Some(Keystroke {
        modifiers,
        key: key.to_owned(),
        key_char,
    })
}

/// Mapping from android `Keycode` to GPUI key name (matching the macOS/Linux
/// vocabulary so cross-platform keymaps still work).
fn key_name(code: Keycode) -> Option<&'static str> {
    Some(match code {
        Keycode::Keycode0 => "0",
        Keycode::Keycode1 => "1",
        Keycode::Keycode2 => "2",
        Keycode::Keycode3 => "3",
        Keycode::Keycode4 => "4",
        Keycode::Keycode5 => "5",
        Keycode::Keycode6 => "6",
        Keycode::Keycode7 => "7",
        Keycode::Keycode8 => "8",
        Keycode::Keycode9 => "9",
        Keycode::A => "a",
        Keycode::B => "b",
        Keycode::C => "c",
        Keycode::D => "d",
        Keycode::E => "e",
        Keycode::F => "f",
        Keycode::G => "g",
        Keycode::H => "h",
        Keycode::I => "i",
        Keycode::J => "j",
        Keycode::K => "k",
        Keycode::L => "l",
        Keycode::M => "m",
        Keycode::N => "n",
        Keycode::O => "o",
        Keycode::P => "p",
        Keycode::Q => "q",
        Keycode::R => "r",
        Keycode::S => "s",
        Keycode::T => "t",
        Keycode::U => "u",
        Keycode::V => "v",
        Keycode::W => "w",
        Keycode::X => "x",
        Keycode::Y => "y",
        Keycode::Z => "z",
        Keycode::Space => "space",
        Keycode::Enter => "enter",
        Keycode::Tab => "tab",
        Keycode::Del => "backspace",
        Keycode::ForwardDel => "delete",
        Keycode::Escape => "escape",
        Keycode::DpadUp => "up",
        Keycode::DpadDown => "down",
        Keycode::DpadLeft => "left",
        Keycode::DpadRight => "right",
        Keycode::PageUp => "pageup",
        Keycode::PageDown => "pagedown",
        Keycode::MoveHome => "home",
        Keycode::MoveEnd => "end",
        Keycode::F1 => "f1",
        Keycode::F2 => "f2",
        Keycode::F3 => "f3",
        Keycode::F4 => "f4",
        Keycode::F5 => "f5",
        Keycode::F6 => "f6",
        Keycode::F7 => "f7",
        Keycode::F8 => "f8",
        Keycode::F9 => "f9",
        Keycode::F10 => "f10",
        Keycode::F11 => "f11",
        Keycode::F12 => "f12",
        Keycode::Comma => ",",
        Keycode::Period => ".",
        Keycode::Slash => "/",
        Keycode::Backslash => "\\",
        Keycode::Semicolon => ";",
        Keycode::Apostrophe => "'",
        Keycode::Grave => "`",
        Keycode::LeftBracket => "[",
        Keycode::RightBracket => "]",
        Keycode::Equals => "=",
        Keycode::Minus => "-",
        Keycode::Plus => "+",
        Keycode::ShiftLeft | Keycode::ShiftRight => "shift",
        Keycode::AltLeft | Keycode::AltRight => "alt",
        Keycode::CtrlLeft | Keycode::CtrlRight => "control",
        Keycode::MetaLeft | Keycode::MetaRight => "platform",
        _ => return None,
    })
}

/// A best-effort character mapping for keys that produce one. Android's IME
/// usually handles printable-character input for soft keyboards; this only
/// matters for hardware keyboards in edge cases (e.g. shortcut handlers that
/// inspect `key_char`).
fn printable_char(code: Keycode, shift: bool) -> Option<char> {
    let lower = match code {
        Keycode::A => 'a',
        Keycode::B => 'b',
        Keycode::C => 'c',
        Keycode::D => 'd',
        Keycode::E => 'e',
        Keycode::F => 'f',
        Keycode::G => 'g',
        Keycode::H => 'h',
        Keycode::I => 'i',
        Keycode::J => 'j',
        Keycode::K => 'k',
        Keycode::L => 'l',
        Keycode::M => 'm',
        Keycode::N => 'n',
        Keycode::O => 'o',
        Keycode::P => 'p',
        Keycode::Q => 'q',
        Keycode::R => 'r',
        Keycode::S => 's',
        Keycode::T => 't',
        Keycode::U => 'u',
        Keycode::V => 'v',
        Keycode::W => 'w',
        Keycode::X => 'x',
        Keycode::Y => 'y',
        Keycode::Z => 'z',
        Keycode::Space => ' ',
        Keycode::Keycode0 => '0',
        Keycode::Keycode1 => '1',
        Keycode::Keycode2 => '2',
        Keycode::Keycode3 => '3',
        Keycode::Keycode4 => '4',
        Keycode::Keycode5 => '5',
        Keycode::Keycode6 => '6',
        Keycode::Keycode7 => '7',
        Keycode::Keycode8 => '8',
        Keycode::Keycode9 => '9',
        _ => return None,
    };
    if shift {
        Some(lower.to_ascii_uppercase())
    } else {
        Some(lower)
    }
}

