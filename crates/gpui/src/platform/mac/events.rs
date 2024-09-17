use crate::{
    keyboard_layouts::KeyboardLayoutMapping, platform::mac::NSStringExt, point, px, KeyCodes,
    KeyDownEvent, KeyPosition, KeyUpEvent, Keystroke, Modifiers, ModifiersChangedEvent,
    MouseButton, MouseDownEvent, MouseExitEvent, MouseMoveEvent, MouseUpEvent, NavigationDirection,
    Pixels, PlatformInput, ScrollDelta, ScrollWheelEvent, TouchPhase,
};
use cocoa::{
    appkit::{
        NSEvent, NSEventModifierFlags, NSEventPhase, NSEventType, NSExecuteFunctionKey,
        NSF21FunctionKey, NSF22FunctionKey, NSF23FunctionKey, NSF24FunctionKey,
        NSInsertFunctionKey, NSPauseFunctionKey, NSPrintFunctionKey, NSPrintScreenFunctionKey,
        NSScrollLockFunctionKey, NSSelectFunctionKey,
    },
    base::{id, YES},
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
// const ENTER_KEY: u16 = 0x0d;
// const NUMPAD_ENTER_KEY: u16 = 0x03;
// const ESCAPE_KEY: u16 = 0x1b;
// const TAB_KEY: u16 = 0x09;
// const SHIFT_TAB_KEY: u16 = 0x19;

fn synthesize_keyboard_event(code: CGKeyCode) -> CGEvent {
    static mut EVENT_SOURCE: core_graphics::sys::CGEventSourceRef = std::ptr::null_mut();
    static INIT_EVENT_SOURCE: Once = Once::new();

    INIT_EVENT_SOURCE.call_once(|| {
        let source = CGEventSource::new(CGEventSourceStateID::Private).unwrap();
        unsafe {
            EVENT_SOURCE = source.as_ptr();
        };
        std::mem::forget(source);
    });

    let source = unsafe { core_graphics::event_source::CGEventSource::from_ptr(EVENT_SOURCE) };
    let event = CGEvent::new_keyboard_event(source.clone(), code, true).unwrap();
    std::mem::forget(source);
    event
}

// TODO:
pub fn key_to_native(key: &KeyCodes) -> Cow<str> {
    use cocoa::appkit::*;
    let code = match key {
        KeyCodes::Space => SPACE_KEY,
        KeyCodes::Backspace => BACKSPACE_KEY,
        KeyCodes::Up => NSUpArrowFunctionKey,
        KeyCodes::Down => NSDownArrowFunctionKey,
        KeyCodes::Left => NSLeftArrowFunctionKey,
        KeyCodes::Right => NSRightArrowFunctionKey,
        KeyCodes::PageUp => NSPageUpFunctionKey,
        KeyCodes::PageDown => NSPageDownFunctionKey,
        KeyCodes::Home => NSHomeFunctionKey,
        KeyCodes::End => NSEndFunctionKey,
        KeyCodes::Delete => NSDeleteFunctionKey,
        KeyCodes::Insert => NSHelpFunctionKey,
        KeyCodes::F1 => NSF1FunctionKey,
        KeyCodes::F2 => NSF2FunctionKey,
        KeyCodes::F3 => NSF3FunctionKey,
        KeyCodes::F4 => NSF4FunctionKey,
        KeyCodes::F5 => NSF5FunctionKey,
        KeyCodes::F6 => NSF6FunctionKey,
        KeyCodes::F7 => NSF7FunctionKey,
        KeyCodes::F8 => NSF8FunctionKey,
        KeyCodes::F9 => NSF9FunctionKey,
        KeyCodes::F10 => NSF10FunctionKey,
        KeyCodes::F11 => NSF11FunctionKey,
        KeyCodes::F12 => NSF12FunctionKey,
        KeyCodes::F13 => NSF13FunctionKey,
        KeyCodes::F14 => NSF14FunctionKey,
        KeyCodes::F15 => NSF15FunctionKey,
        KeyCodes::F16 => NSF16FunctionKey,
        KeyCodes::F17 => NSF17FunctionKey,
        KeyCodes::F18 => NSF18FunctionKey,
        KeyCodes::F19 => NSF19FunctionKey,
        _ => return Cow::Owned(key.to_string()),
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
        layout_mapping: &Option<KeyboardLayoutMapping>,
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
                keystroke: parse_keystroke(native_event, layout_mapping),
                is_held: native_event.isARepeat() == YES,
            })),
            NSEventType::NSKeyUp => Some(Self::KeyUp(KeyUpEvent {
                keystroke: parse_keystroke(native_event, layout_mapping),
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

unsafe fn parse_keystroke(
    native_event: id,
    layout_mapping: &Option<KeyboardLayoutMapping>,
) -> Keystroke {
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
    // let key = chars_ignoring_modifiers;
    let key = key_string_from_keycode(unsafe { native_event.keyCode() });
    let code = keyboard_event_to_virtual_keycodes(native_event, layout_mapping).unwrap_or_default();
    let result = Keystroke {
        modifiers: Modifiers {
            control,
            alt,
            shift,
            platform: command,
            function,
        },
        key,
        code,
        ime_key: None,
    };
    println!("Keyboard event: {:#?}", result);
    result
}

// fn chars_for_modified_key(code: CGKeyCode, cmd: bool, shift: bool) -> String {
//     // Ideally, we would use `[NSEvent charactersByApplyingModifiers]` but that
//     // always returns an empty string with certain keyboards, e.g. Japanese. Synthesizing
//     // an event with the given flags instead lets us access `characters`, which always
//     // returns a valid string.
//     let event = synthesize_keyboard_event(code);

//     let mut flags = CGEventFlags::empty();
//     if cmd {
//         flags |= CGEventFlags::CGEventFlagCommand;
//     }
//     if shift {
//         flags |= CGEventFlags::CGEventFlagShift;
//     }
//     event.set_flags(flags);

//     unsafe {
//         let event: id = msg_send![class!(NSEvent), eventWithCGEvent: &*event];
//         event.characters().to_str().to_string()
//     }
// }

fn key_string_from_keycode(code: CGKeyCode) -> String {
    let event = synthesize_keyboard_event(code);
    unsafe {
        let event: id = msg_send![class!(NSEvent), eventWithCGEvent: &*event];
        event.characters().to_str().to_string()
    }
}

/// TODO:
fn keyboard_event_to_virtual_keycodes(
    native_event: id,
    layout_mapping: &Option<KeyboardLayoutMapping>,
) -> Option<KeyCodes> {
    // let x = unsafe { native_event.characters().to_str().to_string() };
    // let y = unsafe {
    //     native_event
    //         .charactersIgnoringModifiers()
    //         .to_str()
    //         .to_string()
    // };
    // println!("  ==> {}, {}, 0x{:02X}", x, y, unsafe {
    //     native_event.keyCode()
    // });
    // let keycode = unsafe { native_event.keyCode() };
    // numeric keys 0-9 should always return VKCode 0-9
    // if !is_keypad_or_numeric_key_event(native_event) {
    //     if let Some(mapping) = layout_mapping {
    //         if let Some(virtual_key) = mapping.get(&keycode) {
    //             return Some(*virtual_key);
    //         }
    //     }
    //     // Handle Dvorak-QWERTY cmd case
    //     let chars = unsafe { native_event.characters().to_str().to_string() };
    //     if let Some(ch) = chars.chars().next() {
    //         if let Some(key) = virtual_keycode_from_char(ch) {
    //             return Some(key);
    //         }
    //     }
    //     let chars = unsafe {
    //         native_event
    //             .charactersIgnoringModifiers()
    //             .to_str()
    //             .to_string()
    //     };
    //     if let Some(ch) = chars.chars().next() {
    //         if let Some(key) = virtual_keycode_from_char(ch) {
    //             return Some(key);
    //         }
    //     }
    // }
    virtual_keycode_from_keycode(unsafe { native_event.keyCode() })
}

// fn is_keypad_or_numeric_key_event(native_event: id) -> bool {
//     match unsafe { native_event.keyCode() } {
//         DIGITAL_0 | DIGITAL_1 | DIGITAL_2 | DIGITAL_3 | DIGITAL_4 | DIGITAL_5 | DIGITAL_6
//         | DIGITAL_7 | DIGITAL_8 | DIGITAL_9 | KEYPAD_0 | KEYPAD_1 | KEYPAD_2 | KEYPAD_3
//         | KEYPAD_4 | KEYPAD_5 | KEYPAD_6 | KEYPAD_7 | KEYPAD_8 | KEYPAD_9 | KEYPAD_DECIMAL
//         | KEYPAD_MULTIPLY | KEYPAD_PLUS | KEYPAD_CLEAR | KEYPAD_DIVIDE | KEYPAD_ENTER
//         | KEYPAD_MINUS | KEYPAD_EQUALS => true,
//         _ => false,
//     }
// }

// #[allow(non_upper_case_globals)]
// fn virtual_keycode_from_char(ch: char) -> Option<KeyCodes> {
//     match ch {
//         'a' | 'A' => Some(KeyCodes::A),
//         'b' | 'B' => Some(KeyCodes::B),
//         'c' | 'C' => Some(KeyCodes::C),
//         'd' | 'D' => Some(KeyCodes::D),
//         'e' | 'E' => Some(KeyCodes::E),
//         'f' | 'F' => Some(KeyCodes::F),
//         'g' | 'G' => Some(KeyCodes::G),
//         'h' | 'H' => Some(KeyCodes::H),
//         'i' | 'I' => Some(KeyCodes::I),
//         'j' | 'J' => Some(KeyCodes::J),
//         'k' | 'K' => Some(KeyCodes::K),
//         'l' | 'L' => Some(KeyCodes::L),
//         'm' | 'M' => Some(KeyCodes::M),
//         'n' | 'N' => Some(KeyCodes::N),
//         'o' | 'O' => Some(KeyCodes::O),
//         'p' | 'P' => Some(KeyCodes::P),
//         'q' | 'Q' => Some(KeyCodes::Q),
//         'r' | 'R' => Some(KeyCodes::R),
//         's' | 'S' => Some(KeyCodes::S),
//         't' | 'T' => Some(KeyCodes::T),
//         'u' | 'U' => Some(KeyCodes::U),
//         'v' | 'V' => Some(KeyCodes::V),
//         'w' | 'W' => Some(KeyCodes::W),
//         'x' | 'X' => Some(KeyCodes::X),
//         'y' | 'Y' => Some(KeyCodes::Y),
//         'z' | 'Z' => Some(KeyCodes::Z),
//         '0' => Some(KeyCodes::Digital0),
//         '1' => Some(KeyCodes::Digital1),
//         '2' => Some(KeyCodes::Digital2),
//         '3' => Some(KeyCodes::Digital3),
//         '4' => Some(KeyCodes::Digital4),
//         '5' => Some(KeyCodes::Digital5),
//         '6' => Some(KeyCodes::Digital6),
//         '7' => Some(KeyCodes::Digital7),
//         '8' => Some(KeyCodes::Digital8),
//         '9' => Some(KeyCodes::Digital9),
//         ';' => Some(KeyCodes::OEM1),
//         ':' => Some(KeyCodes::OEM1),
//         '=' => Some(KeyCodes::OEMPlus),
//         '+' => Some(KeyCodes::OEMPlus),
//         ',' => Some(KeyCodes::OEMComma),
//         '<' => Some(KeyCodes::OEMComma),
//         '-' => Some(KeyCodes::OEMMinus),
//         '_' => Some(KeyCodes::OEMMinus),
//         '.' => Some(KeyCodes::OEMPeriod),
//         '>' => Some(KeyCodes::OEMPeriod),
//         '/' => Some(KeyCodes::OEM2),
//         '?' => Some(KeyCodes::OEM2),
//         '`' => Some(KeyCodes::OEM3),
//         '~' => Some(KeyCodes::OEM3),
//         '[' => Some(KeyCodes::OEM4),
//         '{' => Some(KeyCodes::OEM4),
//         '\\' => Some(KeyCodes::OEM5),
//         '|' => Some(KeyCodes::OEM5),
//         ']' => Some(KeyCodes::OEM6),
//         '}' => Some(KeyCodes::OEM6),
//         '\'' => Some(KeyCodes::OEM7),
//         '"' => Some(KeyCodes::OEM7),
//         ch => {
//             let ch = ch as u16;
//             match ch {
//                 NSPauseFunctionKey => Some(KeyCodes::Pause),
//                 NSSelectFunctionKey => Some(KeyCodes::Select),
//                 NSPrintFunctionKey => Some(KeyCodes::Print),
//                 NSExecuteFunctionKey => Some(KeyCodes::Execute),
//                 NSPrintScreenFunctionKey => Some(KeyCodes::PrintScreen),
//                 NSInsertFunctionKey => Some(KeyCodes::Insert),
//                 NSF21FunctionKey => Some(KeyCodes::F21),
//                 NSF22FunctionKey => Some(KeyCodes::F22),
//                 NSF23FunctionKey => Some(KeyCodes::F23),
//                 NSF24FunctionKey => Some(KeyCodes::F24),
//                 NSScrollLockFunctionKey => Some(KeyCodes::ScrollLock),
//                 _ => None,
//             }
//         }
//     }
// }

fn virtual_keycode_from_keycode(keycode: u16) -> Option<KeyCodes> {
    KEYBOARD_CODES.get(keycode as usize).copied()
}

static KEYBOARD_CODES: [KeyCodes; 128] = [
    KeyCodes::A, // 0x00
    KeyCodes::S,
    KeyCodes::D,
    KeyCodes::F,
    KeyCodes::H,
    KeyCodes::G,
    KeyCodes::Z,
    KeyCodes::X,
    KeyCodes::C,
    KeyCodes::V,
    KeyCodes::Unknown, // Section key
    KeyCodes::B,
    KeyCodes::Q,
    KeyCodes::W,
    KeyCodes::E,
    KeyCodes::R,
    KeyCodes::Y,
    KeyCodes::T,
    KeyCodes::Digital1,
    KeyCodes::Digital2,
    KeyCodes::Digital3,
    KeyCodes::Digital4,
    KeyCodes::Digital6,
    KeyCodes::Digital5,
    KeyCodes::Plus, // =+
    KeyCodes::Digital9,
    KeyCodes::Digital7,
    KeyCodes::Minus, // -_
    KeyCodes::Digital8,
    KeyCodes::Digital0,
    KeyCodes::RightBracket, // ]}
    KeyCodes::O,
    KeyCodes::U,
    KeyCodes::LeftBracket, // [{
    KeyCodes::I,
    KeyCodes::P,
    KeyCodes::Enter,
    KeyCodes::L,
    KeyCodes::J,
    KeyCodes::Quote, // '"
    KeyCodes::K,
    KeyCodes::Semicolon, // ;:
    KeyCodes::Backslash, // \|
    KeyCodes::Comma,     // ,<
    KeyCodes::Slash,     // /?
    KeyCodes::N,
    KeyCodes::M,
    KeyCodes::Period, // .>
    KeyCodes::Tab,
    KeyCodes::Space,
    KeyCodes::Tilde, // `~
    KeyCodes::Backspace,
    KeyCodes::Unknown, // n/a
    KeyCodes::Escape,
    KeyCodes::App, // Right command
    KeyCodes::Platform(KeyPosition::Left),
    KeyCodes::Shift(KeyPosition::Left),
    KeyCodes::Capital,                     // Capslock
    KeyCodes::Alt(KeyPosition::Left),      // Left option
    KeyCodes::Control(KeyPosition::Left),  // Left control
    KeyCodes::Shift(KeyPosition::Right),   // Right shift
    KeyCodes::Alt(KeyPosition::Right),     // Right option
    KeyCodes::Control(KeyPosition::Right), // Right control
    KeyCodes::Function,                    // TODO: VK_UNKNOWN on Chrome
    KeyCodes::F17,
    KeyCodes::Decimal,  // Numpad .
    KeyCodes::Unknown,  // n/a
    KeyCodes::Multiply, // Numpad *
    KeyCodes::Unknown,  // n/a
    KeyCodes::Add,      // Numpad +
    KeyCodes::Unknown,  // n/a
    KeyCodes::Clear,    // Numpad clear
    KeyCodes::VolumeUp,
    KeyCodes::VolumeDown,
    KeyCodes::VolumeMute,
    KeyCodes::Divide,   // Numpad /
    KeyCodes::Enter,    // Numpad enter
    KeyCodes::Unknown,  // n/a
    KeyCodes::Subtract, // Numpad -
    KeyCodes::F18,
    KeyCodes::F19,
    KeyCodes::Plus, // Numpad =.
    KeyCodes::Numpad0,
    KeyCodes::Numpad1,
    KeyCodes::Numpad2,
    KeyCodes::Numpad3,
    KeyCodes::Numpad4,
    KeyCodes::Numpad5,
    KeyCodes::Numpad6,
    KeyCodes::Numpad7,
    KeyCodes::F20,
    KeyCodes::Numpad8,
    KeyCodes::Numpad9,
    KeyCodes::Unknown, // Yen, JIS keyboad only
    KeyCodes::Unknown, // Underscore, JIS keyboard only
    KeyCodes::Unknown, // Keypad comma, JIS keyboard only
    KeyCodes::F5,
    KeyCodes::F6,
    KeyCodes::F7,
    KeyCodes::F3,
    KeyCodes::F8,
    KeyCodes::F9,
    KeyCodes::Unknown, // Eisu, JIS keyboard only
    KeyCodes::F11,
    KeyCodes::Unknown, // Kana, JIS keyboard only
    KeyCodes::F13,
    KeyCodes::F16,
    KeyCodes::F14,
    KeyCodes::Unknown, // n/a
    KeyCodes::F10,
    KeyCodes::App, // Context menu key
    KeyCodes::F12,
    KeyCodes::Unknown, // n/a
    KeyCodes::F15,
    KeyCodes::Insert, // Help
    KeyCodes::Home,   // Home
    KeyCodes::PageUp,
    KeyCodes::Delete, // Forward delete
    KeyCodes::F4,
    KeyCodes::End,
    KeyCodes::F2,
    KeyCodes::PageDown,
    KeyCodes::F1,
    KeyCodes::Left,
    KeyCodes::Right,
    KeyCodes::Down,
    KeyCodes::Up,
    KeyCodes::Unknown, // n/a
];
