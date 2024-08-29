use crate::{
    platform::mac::NSStringExt, point, px, KeyDownEvent, KeyUpEvent, Keystroke, Modifiers,
    ModifiersChangedEvent, MouseButton, MouseDownEvent, MouseExitEvent, MouseMoveEvent,
    MouseUpEvent, NavigationDirection, Pixels, PlatformInput, ScrollDelta, ScrollWheelEvent,
    TouchPhase,
};
use cocoa::{
    appkit::{NSEvent, NSEventModifierFlags, NSEventPhase, NSEventType},
    base::{id, YES},
    foundation::NSString,
};
use core_foundation::{
    array::{CFArray, CFArrayRef},
    base::TCFType,
    string::CFString,
};
use core_graphics::{
    event::{CGEvent, CGEventFlags, CGKeyCode},
    event_source::{CGEventSource, CGEventSourceStateID},
};
use metal::foreign_types::ForeignType as _;
use objc::{class, msg_send, sel, sel_impl};
use std::{borrow::Cow, mem, ptr, sync::Once};

const BACKSPACE_KEY: u16 = 0x7f;
const SPACE_KEY: u16 = b' ' as u16;
const ENTER_KEY: u16 = 0x0d;
const NUMPAD_ENTER_KEY: u16 = 0x03;
const ESCAPE_KEY: u16 = 0x1b;
const TAB_KEY: u16 = 0x09;
const SHIFT_TAB_KEY: u16 = 0x19;

fn synthesize_keyboard_event(code: CGKeyCode) -> CGEvent {
    static mut EVENT_SOURCE: core_graphics::sys::CGEventSourceRef = ptr::null_mut();
    static INIT_EVENT_SOURCE: Once = Once::new();

    INIT_EVENT_SOURCE.call_once(|| {
        let source = CGEventSource::new(CGEventSourceStateID::Private).unwrap();
        unsafe {
            EVENT_SOURCE = source.as_ptr();
        };
        mem::forget(source);
    });

    let source = unsafe { core_graphics::event_source::CGEventSource::from_ptr(EVENT_SOURCE) };
    let event = CGEvent::new_keyboard_event(source.clone(), code, true).unwrap();
    mem::forget(source);
    event
}

pub fn key_to_native(key: &str) -> Cow<str> {
    use cocoa::appkit::*;
    let code = match key {
        "space" => SPACE_KEY,
        "backspace" => BACKSPACE_KEY,
        "up" => NSUpArrowFunctionKey,
        "down" => NSDownArrowFunctionKey,
        "left" => NSLeftArrowFunctionKey,
        "right" => NSRightArrowFunctionKey,
        "pageup" => NSPageUpFunctionKey,
        "pagedown" => NSPageDownFunctionKey,
        "home" => NSHomeFunctionKey,
        "end" => NSEndFunctionKey,
        "delete" => NSDeleteFunctionKey,
        "insert" => NSHelpFunctionKey,
        "f1" => NSF1FunctionKey,
        "f2" => NSF2FunctionKey,
        "f3" => NSF3FunctionKey,
        "f4" => NSF4FunctionKey,
        "f5" => NSF5FunctionKey,
        "f6" => NSF6FunctionKey,
        "f7" => NSF7FunctionKey,
        "f8" => NSF8FunctionKey,
        "f9" => NSF9FunctionKey,
        "f10" => NSF10FunctionKey,
        "f11" => NSF11FunctionKey,
        "f12" => NSF12FunctionKey,
        "f13" => NSF13FunctionKey,
        "f14" => NSF14FunctionKey,
        "f15" => NSF15FunctionKey,
        "f16" => NSF16FunctionKey,
        "f17" => NSF17FunctionKey,
        "f18" => NSF18FunctionKey,
        "f19" => NSF19FunctionKey,
        _ => return Cow::Borrowed(key),
    };
    Cow::Owned(String::from_utf16(&[code]).unwrap())
}

unsafe fn read_modifiers(native_event: id) -> Modifiers {
    let modifiers = native_event.modifierFlags();
    let control = modifiers.contains(NSEventModifierFlags::NSControlKeyMask);
    let alt = modifiers.contains(NSEventModifierFlags::NSAlternateKeyMask);
    let shift = modifiers.contains(NSEventModifierFlags::NSShiftKeyMask);
    let command = modifiers.contains(NSEventModifierFlags::NSCommandKeyMask);
    let function = modifiers.contains(NSEventModifierFlags::NSFunctionKeyMask);

    Modifiers {
        control,
        alt,
        shift,
        platform: command,
        function,
    }
}

impl PlatformInput {
    pub(crate) unsafe fn from_native(
        native_event: id,
        window_height: Option<Pixels>,
    ) -> Option<Self> {
        let event_type = native_event.eventType();

        // Filter out event types that aren't in the NSEventType enum.
        // See https://github.com/servo/cocoa-rs/issues/155#issuecomment-323482792 for details.
        match event_type as u64 {
            0 | 21 | 32 | 33 | 35 | 36 | 37 => {
                return None;
            }
            _ => {}
        }

        match event_type {
            NSEventType::NSFlagsChanged => Some(Self::ModifiersChanged(ModifiersChangedEvent {
                modifiers: read_modifiers(native_event),
            })),
            NSEventType::NSKeyDown => Some(Self::KeyDown(KeyDownEvent {
                keystroke: parse_keystroke(native_event),
                is_held: native_event.isARepeat() == YES,
            })),
            NSEventType::NSKeyUp => Some(Self::KeyUp(KeyUpEvent {
                keystroke: parse_keystroke(native_event),
            })),
            NSEventType::NSLeftMouseDown
            | NSEventType::NSRightMouseDown
            | NSEventType::NSOtherMouseDown => {
                let button = match native_event.buttonNumber() {
                    0 => MouseButton::Left,
                    1 => MouseButton::Right,
                    2 => MouseButton::Middle,
                    3 => MouseButton::Navigate(NavigationDirection::Back),
                    4 => MouseButton::Navigate(NavigationDirection::Forward),
                    // Other mouse buttons aren't tracked currently
                    _ => return None,
                };
                window_height.map(|window_height| {
                    Self::MouseDown(MouseDownEvent {
                        button,
                        position: point(
                            px(native_event.locationInWindow().x as f32),
                            // MacOS screen coordinates are relative to bottom left
                            window_height - px(native_event.locationInWindow().y as f32),
                        ),
                        modifiers: read_modifiers(native_event),
                        click_count: native_event.clickCount() as usize,
                        first_mouse: false,
                    })
                })
            }
            NSEventType::NSLeftMouseUp
            | NSEventType::NSRightMouseUp
            | NSEventType::NSOtherMouseUp => {
                let button = match native_event.buttonNumber() {
                    0 => MouseButton::Left,
                    1 => MouseButton::Right,
                    2 => MouseButton::Middle,
                    3 => MouseButton::Navigate(NavigationDirection::Back),
                    4 => MouseButton::Navigate(NavigationDirection::Forward),
                    // Other mouse buttons aren't tracked currently
                    _ => return None,
                };

                window_height.map(|window_height| {
                    Self::MouseUp(MouseUpEvent {
                        button,
                        position: point(
                            px(native_event.locationInWindow().x as f32),
                            window_height - px(native_event.locationInWindow().y as f32),
                        ),
                        modifiers: read_modifiers(native_event),
                        click_count: native_event.clickCount() as usize,
                    })
                })
            }
            NSEventType::NSScrollWheel => window_height.map(|window_height| {
                let phase = match native_event.phase() {
                    NSEventPhase::NSEventPhaseMayBegin | NSEventPhase::NSEventPhaseBegan => {
                        TouchPhase::Started
                    }
                    NSEventPhase::NSEventPhaseEnded => TouchPhase::Ended,
                    _ => TouchPhase::Moved,
                };

                let raw_data = point(
                    native_event.scrollingDeltaX() as f32,
                    native_event.scrollingDeltaY() as f32,
                );

                let delta = if native_event.hasPreciseScrollingDeltas() == YES {
                    ScrollDelta::Pixels(raw_data.map(px))
                } else {
                    ScrollDelta::Lines(raw_data)
                };

                Self::ScrollWheel(ScrollWheelEvent {
                    position: point(
                        px(native_event.locationInWindow().x as f32),
                        window_height - px(native_event.locationInWindow().y as f32),
                    ),
                    delta,
                    touch_phase: phase,
                    modifiers: read_modifiers(native_event),
                })
            }),
            NSEventType::NSLeftMouseDragged
            | NSEventType::NSRightMouseDragged
            | NSEventType::NSOtherMouseDragged => {
                let pressed_button = match native_event.buttonNumber() {
                    0 => MouseButton::Left,
                    1 => MouseButton::Right,
                    2 => MouseButton::Middle,
                    3 => MouseButton::Navigate(NavigationDirection::Back),
                    4 => MouseButton::Navigate(NavigationDirection::Forward),
                    // Other mouse buttons aren't tracked currently
                    _ => return None,
                };

                window_height.map(|window_height| {
                    Self::MouseMove(MouseMoveEvent {
                        pressed_button: Some(pressed_button),
                        position: point(
                            px(native_event.locationInWindow().x as f32),
                            window_height - px(native_event.locationInWindow().y as f32),
                        ),
                        modifiers: read_modifiers(native_event),
                    })
                })
            }
            NSEventType::NSMouseMoved => window_height.map(|window_height| {
                Self::MouseMove(MouseMoveEvent {
                    position: point(
                        px(native_event.locationInWindow().x as f32),
                        window_height - px(native_event.locationInWindow().y as f32),
                    ),
                    pressed_button: None,
                    modifiers: read_modifiers(native_event),
                })
            }),
            NSEventType::NSMouseExited => window_height.map(|window_height| {
                Self::MouseExited(MouseExitEvent {
                    position: point(
                        px(native_event.locationInWindow().x as f32),
                        window_height - px(native_event.locationInWindow().y as f32),
                    ),

                    pressed_button: None,
                    modifiers: read_modifiers(native_event),
                })
            }),
            _ => None,
        }
    }
}

unsafe fn parse_keystroke(native_event: id) -> Keystroke {
    use cocoa::appkit::*;

    let mut chars_ignoring_modifiers = native_event
        .charactersIgnoringModifiers()
        .to_str()
        .to_string();
    let first_char = chars_ignoring_modifiers.chars().next().map(|ch| ch as u16);
    let modifiers = native_event.modifierFlags();

    let control = modifiers.contains(NSEventModifierFlags::NSControlKeyMask);
    let alt = modifiers.contains(NSEventModifierFlags::NSAlternateKeyMask);
    let mut shift = modifiers.contains(NSEventModifierFlags::NSShiftKeyMask);
    let command = modifiers.contains(NSEventModifierFlags::NSCommandKeyMask);
    let function = modifiers.contains(NSEventModifierFlags::NSFunctionKeyMask)
        && first_char.map_or(true, |ch| {
            !(NSUpArrowFunctionKey..=NSModeSwitchFunctionKey).contains(&ch)
        });

    // #[allow(non_upper_case_globals)]
    // let key = match first_char {
    //     Some(SPACE_KEY) => "space".to_string(),
    //     Some(BACKSPACE_KEY) => "backspace".to_string(),
    //     Some(ENTER_KEY) | Some(NUMPAD_ENTER_KEY) => "enter".to_string(),
    //     Some(ESCAPE_KEY) => "escape".to_string(),
    //     Some(TAB_KEY) => "tab".to_string(),
    //     Some(SHIFT_TAB_KEY) => "tab".to_string(),
    //     Some(NSUpArrowFunctionKey) => "up".to_string(),
    //     Some(NSDownArrowFunctionKey) => "down".to_string(),
    //     Some(NSLeftArrowFunctionKey) => "left".to_string(),
    //     Some(NSRightArrowFunctionKey) => "right".to_string(),
    //     Some(NSPageUpFunctionKey) => "pageup".to_string(),
    //     Some(NSPageDownFunctionKey) => "pagedown".to_string(),
    //     Some(NSHomeFunctionKey) => "home".to_string(),
    //     Some(NSEndFunctionKey) => "end".to_string(),
    //     Some(NSDeleteFunctionKey) => "delete".to_string(),
    //     // Observed Insert==NSHelpFunctionKey not NSInsertFunctionKey.
    //     Some(NSHelpFunctionKey) => "insert".to_string(),
    //     Some(NSF1FunctionKey) => "f1".to_string(),
    //     Some(NSF2FunctionKey) => "f2".to_string(),
    //     Some(NSF3FunctionKey) => "f3".to_string(),
    //     Some(NSF4FunctionKey) => "f4".to_string(),
    //     Some(NSF5FunctionKey) => "f5".to_string(),
    //     Some(NSF6FunctionKey) => "f6".to_string(),
    //     Some(NSF7FunctionKey) => "f7".to_string(),
    //     Some(NSF8FunctionKey) => "f8".to_string(),
    //     Some(NSF9FunctionKey) => "f9".to_string(),
    //     Some(NSF10FunctionKey) => "f10".to_string(),
    //     Some(NSF11FunctionKey) => "f11".to_string(),
    //     Some(NSF12FunctionKey) => "f12".to_string(),
    //     Some(NSF13FunctionKey) => "f13".to_string(),
    //     Some(NSF14FunctionKey) => "f14".to_string(),
    //     Some(NSF15FunctionKey) => "f15".to_string(),
    //     Some(NSF16FunctionKey) => "f16".to_string(),
    //     Some(NSF17FunctionKey) => "f17".to_string(),
    //     Some(NSF18FunctionKey) => "f18".to_string(),
    //     Some(NSF19FunctionKey) => "f19".to_string(),
    //     _ => {
    //         let mut chars_ignoring_modifiers_and_shift =
    //             chars_for_modified_key(native_event.keyCode(), false, false);

    //         // Honor ⌘ when Dvorak-QWERTY is used.
    //         let chars_with_cmd = chars_for_modified_key(native_event.keyCode(), true, false);
    //         if command && chars_ignoring_modifiers_and_shift != chars_with_cmd {
    //             chars_ignoring_modifiers =
    //                 chars_for_modified_key(native_event.keyCode(), true, shift);
    //             chars_ignoring_modifiers_and_shift = chars_with_cmd;
    //         }

    //         if shift {
    //             if chars_ignoring_modifiers_and_shift
    //                 == chars_ignoring_modifiers.to_ascii_lowercase()
    //             {
    //                 chars_ignoring_modifiers_and_shift
    //             } else if chars_ignoring_modifiers_and_shift != chars_ignoring_modifiers {
    //                 shift = false;
    //                 chars_ignoring_modifiers
    //             } else {
    //                 chars_ignoring_modifiers
    //             }
    //         } else {
    //             chars_ignoring_modifiers
    //         }
    //     }
    // };
    let key = keyboard_event_to_key(native_event).unwrap_or("default".to_string());
    let result = Keystroke {
        modifiers: Modifiers {
            control,
            alt,
            shift,
            platform: command,
            function,
        },
        key,
        ime_key: None,
    };
    println!("Keyboard event: {:#?}", result);
    result
}

fn chars_for_modified_key(code: CGKeyCode, cmd: bool, shift: bool) -> String {
    // Ideally, we would use `[NSEvent charactersByApplyingModifiers]` but that
    // always returns an empty string with certain keyboards, e.g. Japanese. Synthesizing
    // an event with the given flags instead lets us access `characters`, which always
    // returns a valid string.
    let event = synthesize_keyboard_event(code);

    let mut flags = CGEventFlags::empty();
    if cmd {
        flags |= CGEventFlags::CGEventFlagCommand;
    }
    if shift {
        flags |= CGEventFlags::CGEventFlagShift;
    }
    event.set_flags(flags);

    unsafe {
        let event: id = msg_send![class!(NSEvent), eventWithCGEvent: &*event];
        event.characters().to_str().to_string()
    }
}

/// TODO:
fn keyboard_event_to_key(native_event: id) -> Option<String> {
    // numeric keys 0-9 should always return VKCode 0-9
    if !is_keyboard_event_numeric_or_keypad(native_event) {
        // Handle Dvorak-QWERTY cmd case
        let chars = unsafe { native_event.characters().to_str().to_string() };
        if chars.len() > 0 {
            if let Some(key) = key_from_char(chars) {
                return Some(key);
            }
        }
        let chars = unsafe {
            native_event
                .charactersIgnoringModifiers()
                .to_str()
                .to_string()
        };
        if chars.len() > 0 {
            if let Some(key) = key_from_char(chars) {
                return Some(key);
            }
        }
    }
    key_from_keycode(unsafe { native_event.keyCode() })
}

fn is_keyboard_event_numeric_or_keypad(native_event: id) -> bool {
    match unsafe { native_event.keyCode() } {
        DIGITAL_0 | DIGITAL_1 | DIGITAL_2 | DIGITAL_3 | DIGITAL_4 | DIGITAL_5 | DIGITAL_6
        | DIGITAL_7 | DIGITAL_8 | DIGITAL_9 | KEYPAD_0 | KEYPAD_1 | KEYPAD_2 | KEYPAD_3
        | KEYPAD_4 | KEYPAD_5 | KEYPAD_6 | KEYPAD_7 | KEYPAD_8 | KEYPAD_9 | KEYPAD_DECIMAL
        | KEYPAD_MULTIPLY | KEYPAD_PLUS | KEYPAD_CLEAR | KEYPAD_DIVIDE | KEYPAD_ENTER
        | KEYPAD_MINUS | KEYPAD_EQUALS => true,
        _ => false,
    }
}

fn key_from_char(chars: String) -> Option<String> {
    match chars.as_str() {
        "a" | "A" => Some("a".to_string()),
        "b" | "B" => Some("b".to_string()),
        "c" | "C" => Some("c".to_string()),
        "d" | "D" => Some("c".to_string()),
        "e" | "E" => Some("e".to_string()),
        "f" | "F" => Some("f".to_string()),
        "g" | "G" => Some("g".to_string()),
        "h" | "H" => Some("h".to_string()),
        "i" | "I" => Some("i".to_string()),
        "j" | "J" => Some("j".to_string()),
        "k" | "K" => Some("k".to_string()),
        "l" | "L" => Some("l".to_string()),
        "m" | "M" => Some("m".to_string()),
        "n" | "N" => Some("n".to_string()),
        "o" | "O" => Some("o".to_string()),
        "p" | "P" => Some("p".to_string()),
        "q" | "Q" => Some("q".to_string()),
        "r" | "R" => Some("r".to_string()),
        "s" | "S" => Some("s".to_string()),
        "t" | "T" => Some("t".to_string()),
        "u" | "U" => Some("u".to_string()),
        "v" | "V" => Some("v".to_string()),
        "w" | "W" => Some("w".to_string()),
        "x" | "X" => Some("x".to_string()),
        "y" | "Y" => Some("y".to_string()),
        "z" | "Z" => Some("z".to_string()),
        "0" => Some("0".to_string()),
        "1" => Some("1".to_string()),
        "2" => Some("2".to_string()),
        "3" => Some("3".to_string()),
        "4" => Some("4".to_string()),
        "5" => Some("5".to_string()),
        "6" => Some("6".to_string()),
        "7" => Some("7".to_string()),
        "8" => Some("8".to_string()),
        "9" => Some("9".to_string()),
        ";" => Some(";".to_string()),
        ":" => Some(":".to_string()),
        "=" => Some("=".to_string()),
        "+" => Some("+".to_string()),
        "," => Some(",".to_string()),
        "<" => Some("<".to_string()),
        "-" => Some("-".to_string()),
        "_" => Some("_".to_string()),
        "." => Some(".".to_string()),
        ">" => Some(">".to_string()),
        "/" => Some("/".to_string()),
        "?" => Some("?".to_string()),
        "`" => Some("`".to_string()),
        "~" => Some("~".to_string()),
        "[" => Some("[".to_string()),
        "{" => Some("{".to_string()),
        "\\" => Some("\\".to_string()),
        "|" => Some("|".to_string()),
        "[" => Some("[".to_string()),
        "}" => Some("}".to_string()),
        "'" => Some("'".to_string()),
        "\"" => Some("\"".to_string()),
        _ => None,
    }
}

fn key_from_keycode(keycode: u16) -> Option<String> {
    if keycode >= 0x80 {
        return None;
    }
    Some(KEYBOARD_CODES[keycode as usize].to_string())
}

static KEYBOARD_CODES: [&str; 128] = [
    "a",
    "s",
    "d",
    "f",
    "h",
    "g",
    "z",
    "x",
    "c",
    "v",
    "`",
    "b",
    "q",
    "w",
    "e",
    "r",
    "y",
    "t",
    "1",
    "2",
    "3",
    "4",
    "6",
    "5",
    "=",
    "9",
    "7",
    "-",
    "8",
    "0",
    "]",
    "o",
    "u",
    "[",
    "i",
    "p",
    "enter",
    "l",
    "j",
    "'",
    "k",
    ";",
    "\\",
    ",",
    "/",
    "n",
    "m",
    ".",
    "tab",
    "space",
    "`",
    "backspace",
    "unknown",
    "escape",
    "command",
    "command",
    "shift",
    "capslock",
    "alt",
    "control",
    "shift",
    "alt",
    "control",
    "function",
    "f17",
    ".",
    "unknown",
    "*",
    "unknown",
    "+",
    "unknown",
    "clear",
    "unknown",
    "unknown",
    "unknown",
    "/",
    "enter",
    "unknown",
    "-",
    "f18",
    "f19",
    "+",
    "0",
    "1",
    "2",
    "3",
    "4",
    "5",
    "6",
    "7",
    "f20",
    "8",
    "9",
    "unknown",
    "unknown",
    "unknown",
    "f5",
    "f6",
    "f7",
    "f3",
    "f8",
    "f9",
    "unknown",
    "f11",
    "unknown",
    "f13",
    "f16",
    "f14",
    "unknown",
    "f10",
    "unknown",
    "f12",
    "unknown",
    "f15",
    "unknown",
    "unknown",
    "pageup",
    "delete",
    "f4",
    "end",
    "f2",
    "pagedown",
    "f1",
    "left",
    "right",
    "down",
    "up",
    "unknown",
];
const DIGITAL_1: u16 = 0x12;
const DIGITAL_2: u16 = 0x13;
const DIGITAL_3: u16 = 0x14;
const DIGITAL_4: u16 = 0x15;
const DIGITAL_5: u16 = 0x17;
const DIGITAL_6: u16 = 0x16;
const DIGITAL_7: u16 = 0x1a;
const DIGITAL_8: u16 = 0x1c;
const DIGITAL_9: u16 = 0x19;
const DIGITAL_0: u16 = 0x1d;
const KEYPAD_DECIMAL: u16 = 0x41;
const KEYPAD_MULTIPLY: u16 = 0x43;
const KEYPAD_PLUS: u16 = 0x45;
const KEYPAD_CLEAR: u16 = 0x46;
const KEYPAD_DIVIDE: u16 = 0x4b;
const KEYPAD_ENTER: u16 = 0x4c;
const KEYPAD_MINUS: u16 = 0x4e;
const KEYPAD_EQUALS: u16 = 0x51;
const KEYPAD_0: u16 = 0x52;
const KEYPAD_1: u16 = 0x53;
const KEYPAD_2: u16 = 0x54;
const KEYPAD_3: u16 = 0x55;
const KEYPAD_4: u16 = 0x56;
const KEYPAD_5: u16 = 0x57;
const KEYPAD_6: u16 = 0x58;
const KEYPAD_7: u16 = 0x59;
const KEYPAD_8: u16 = 0x5b;
const KEYPAD_9: u16 = 0x5c;
