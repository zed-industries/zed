use crate::{
    keyboard_layouts::KeyboardLayoutMapping, platform::mac::NSStringExt, point, px, KeyDownEvent,
    KeyUpEvent, Keystroke, Modifiers, ModifiersChangedEvent, MouseButton, MouseDownEvent,
    MouseExitEvent, MouseMoveEvent, MouseUpEvent, NavigationDirection, Pixels, PlatformInput,
    ScrollDelta, ScrollWheelEvent, TouchPhase, VirtualKeyCode,
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

// TODO:
pub fn key_to_native(key: &VirtualKeyCode) -> Cow<str> {
    use cocoa::appkit::*;
    let code = match key {
        VirtualKeyCode::Space => SPACE_KEY,
        VirtualKeyCode::Backspace => BACKSPACE_KEY,
        VirtualKeyCode::Up => NSUpArrowFunctionKey,
        VirtualKeyCode::Down => NSDownArrowFunctionKey,
        VirtualKeyCode::Left => NSLeftArrowFunctionKey,
        VirtualKeyCode::Right => NSRightArrowFunctionKey,
        VirtualKeyCode::PageUp => NSPageUpFunctionKey,
        VirtualKeyCode::PageDown => NSPageDownFunctionKey,
        VirtualKeyCode::Home => NSHomeFunctionKey,
        VirtualKeyCode::End => NSEndFunctionKey,
        VirtualKeyCode::Delete => NSDeleteFunctionKey,
        VirtualKeyCode::Insert => NSHelpFunctionKey,
        VirtualKeyCode::F1 => NSF1FunctionKey,
        VirtualKeyCode::F2 => NSF2FunctionKey,
        VirtualKeyCode::F3 => NSF3FunctionKey,
        VirtualKeyCode::F4 => NSF4FunctionKey,
        VirtualKeyCode::F5 => NSF5FunctionKey,
        VirtualKeyCode::F6 => NSF6FunctionKey,
        VirtualKeyCode::F7 => NSF7FunctionKey,
        VirtualKeyCode::F8 => NSF8FunctionKey,
        VirtualKeyCode::F9 => NSF9FunctionKey,
        VirtualKeyCode::F10 => NSF10FunctionKey,
        VirtualKeyCode::F11 => NSF11FunctionKey,
        VirtualKeyCode::F12 => NSF12FunctionKey,
        VirtualKeyCode::F13 => NSF13FunctionKey,
        VirtualKeyCode::F14 => NSF14FunctionKey,
        VirtualKeyCode::F15 => NSF15FunctionKey,
        VirtualKeyCode::F16 => NSF16FunctionKey,
        VirtualKeyCode::F17 => NSF17FunctionKey,
        VirtualKeyCode::F18 => NSF18FunctionKey,
        VirtualKeyCode::F19 => NSF19FunctionKey,
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
    let key = keyboard_event_to_virtual_keycodes(native_event, layout_mapping).unwrap_or_default();
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
fn keyboard_event_to_virtual_keycodes(
    native_event: id,
    layout_mapping: &Option<KeyboardLayoutMapping>,
) -> Option<VirtualKeyCode> {
    let x = unsafe { native_event.characters().to_str().to_string() };
    let y = unsafe {
        native_event
            .charactersIgnoringModifiers()
            .to_str()
            .to_string()
    };
    println!("  ==> {}, {}, 0x{:02X}", x, y, unsafe {
        native_event.keyCode()
    });
    let keycode = unsafe { native_event.keyCode() };
    // numeric keys 0-9 should always return VKCode 0-9
    if !is_keypad_or_numeric_key_event(native_event) {
        if let Some(mapping) = layout_mapping {
            if let Some(virtual_key) = mapping.get(&keycode) {
                return Some(*virtual_key);
            }
        }
        // Handle Dvorak-QWERTY cmd case
        let chars = unsafe { native_event.characters().to_str().to_string() };
        if let Some(ch) = chars.chars().next() {
            if let Some(key) = virtual_keycode_from_char(ch) {
                return Some(key);
            }
        }
        let chars = unsafe {
            native_event
                .charactersIgnoringModifiers()
                .to_str()
                .to_string()
        };
        if let Some(ch) = chars.chars().next() {
            if let Some(key) = virtual_keycode_from_char(ch) {
                return Some(key);
            }
        }
    }
    virtual_keycode_from_keycode(unsafe { native_event.keyCode() })
}

fn is_keypad_or_numeric_key_event(native_event: id) -> bool {
    match unsafe { native_event.keyCode() } {
        DIGITAL_0 | DIGITAL_1 | DIGITAL_2 | DIGITAL_3 | DIGITAL_4 | DIGITAL_5 | DIGITAL_6
        | DIGITAL_7 | DIGITAL_8 | DIGITAL_9 | KEYPAD_0 | KEYPAD_1 | KEYPAD_2 | KEYPAD_3
        | KEYPAD_4 | KEYPAD_5 | KEYPAD_6 | KEYPAD_7 | KEYPAD_8 | KEYPAD_9 | KEYPAD_DECIMAL
        | KEYPAD_MULTIPLY | KEYPAD_PLUS | KEYPAD_CLEAR | KEYPAD_DIVIDE | KEYPAD_ENTER
        | KEYPAD_MINUS | KEYPAD_EQUALS => true,
        _ => false,
    }
}

fn virtual_keycode_from_char(ch: char) -> Option<VirtualKeyCode> {
    match ch {
        'a' | 'A' => Some(VirtualKeyCode::A),
        'b' | 'B' => Some(VirtualKeyCode::B),
        'c' | 'C' => Some(VirtualKeyCode::C),
        'd' | 'D' => Some(VirtualKeyCode::D),
        'e' | 'E' => Some(VirtualKeyCode::E),
        'f' | 'F' => Some(VirtualKeyCode::F),
        'g' | 'G' => Some(VirtualKeyCode::G),
        'h' | 'H' => Some(VirtualKeyCode::H),
        'i' | 'I' => Some(VirtualKeyCode::I),
        'j' | 'J' => Some(VirtualKeyCode::J),
        'k' | 'K' => Some(VirtualKeyCode::K),
        'l' | 'L' => Some(VirtualKeyCode::L),
        'm' | 'M' => Some(VirtualKeyCode::M),
        'n' | 'N' => Some(VirtualKeyCode::N),
        'o' | 'O' => Some(VirtualKeyCode::O),
        'p' | 'P' => Some(VirtualKeyCode::P),
        'q' | 'Q' => Some(VirtualKeyCode::Q),
        'r' | 'R' => Some(VirtualKeyCode::R),
        's' | 'S' => Some(VirtualKeyCode::S),
        't' | 'T' => Some(VirtualKeyCode::T),
        'u' | 'U' => Some(VirtualKeyCode::U),
        'v' | 'V' => Some(VirtualKeyCode::V),
        'w' | 'W' => Some(VirtualKeyCode::W),
        'x' | 'X' => Some(VirtualKeyCode::X),
        'y' | 'Y' => Some(VirtualKeyCode::Y),
        'z' | 'Z' => Some(VirtualKeyCode::Z),
        '0' => Some(VirtualKeyCode::Digital0),
        '1' => Some(VirtualKeyCode::Digital1),
        '2' => Some(VirtualKeyCode::Digital2),
        '3' => Some(VirtualKeyCode::Digital3),
        '4' => Some(VirtualKeyCode::Digital4),
        '5' => Some(VirtualKeyCode::Digital5),
        '6' => Some(VirtualKeyCode::Digital6),
        '7' => Some(VirtualKeyCode::Digital7),
        '8' => Some(VirtualKeyCode::Digital8),
        '9' => Some(VirtualKeyCode::Digital9),
        ';' => Some(VirtualKeyCode::OEM1),
        ':' => Some(VirtualKeyCode::OEM1),
        '=' => Some(VirtualKeyCode::OEMPlus),
        '+' => Some(VirtualKeyCode::OEMPlus),
        ',' => Some(VirtualKeyCode::OEMComma),
        '<' => Some(VirtualKeyCode::OEMComma),
        '-' => Some(VirtualKeyCode::OEMMinus),
        '_' => Some(VirtualKeyCode::OEMMinus),
        '.' => Some(VirtualKeyCode::OEMPeriod),
        '>' => Some(VirtualKeyCode::OEMPeriod),
        '/' => Some(VirtualKeyCode::OEM2),
        '?' => Some(VirtualKeyCode::OEM2),
        '`' => Some(VirtualKeyCode::OEM3),
        '~' => Some(VirtualKeyCode::OEM3),
        '[' => Some(VirtualKeyCode::OEM4),
        '{' => Some(VirtualKeyCode::OEM4),
        '\\' => Some(VirtualKeyCode::OEM5),
        '|' => Some(VirtualKeyCode::OEM5),
        ']' => Some(VirtualKeyCode::OEM6),
        '}' => Some(VirtualKeyCode::OEM6),
        '\'' => Some(VirtualKeyCode::OEM7),
        '"' => Some(VirtualKeyCode::OEM7),
        ch => {
            let ch = ch as u16;
            match ch {
                NSPauseFunctionKey => Some(VirtualKeyCode::Pause),
                NSSelectFunctionKey => Some(VirtualKeyCode::Select),
                NSPrintFunctionKey => Some(VirtualKeyCode::Print),
                NSExecuteFunctionKey => Some(VirtualKeyCode::Execute),
                NSPrintScreenFunctionKey => Some(VirtualKeyCode::PrintScreen),
                NSInsertFunctionKey => Some(VirtualKeyCode::Insert),
                NSF21FunctionKey => Some(VirtualKeyCode::F21),
                NSF22FunctionKey => Some(VirtualKeyCode::F22),
                NSF23FunctionKey => Some(VirtualKeyCode::F23),
                NSF24FunctionKey => Some(VirtualKeyCode::F24),
                NSScrollLockFunctionKey => Some(VirtualKeyCode::ScrollLock),
                _ => None,
            }
        }
    }
}

fn virtual_keycode_from_keycode(keycode: u16) -> Option<VirtualKeyCode> {
    KEYBOARD_CODES.get(keycode as usize).copied()
}

static KEYBOARD_CODES: [VirtualKeyCode; 128] = [
    VirtualKeyCode::A, // 0x00
    VirtualKeyCode::S,
    VirtualKeyCode::D,
    VirtualKeyCode::F,
    VirtualKeyCode::H,
    VirtualKeyCode::G,
    VirtualKeyCode::Z,
    VirtualKeyCode::X,
    VirtualKeyCode::C,
    VirtualKeyCode::V,
    VirtualKeyCode::OEM3, // Section key
    VirtualKeyCode::B,
    VirtualKeyCode::Q,
    VirtualKeyCode::W,
    VirtualKeyCode::E,
    VirtualKeyCode::R,
    VirtualKeyCode::Y,
    VirtualKeyCode::T,
    VirtualKeyCode::Digital1,
    VirtualKeyCode::Digital2,
    VirtualKeyCode::Digital3,
    VirtualKeyCode::Digital4,
    VirtualKeyCode::Digital6,
    VirtualKeyCode::Digital5,
    VirtualKeyCode::OEMPlus, // =+
    VirtualKeyCode::Digital9,
    VirtualKeyCode::Digital7,
    VirtualKeyCode::OEMMinus, // -_
    VirtualKeyCode::Digital8,
    VirtualKeyCode::Digital0,
    VirtualKeyCode::OEM6, // ]}
    VirtualKeyCode::O,
    VirtualKeyCode::U,
    VirtualKeyCode::OEM4, // [{
    VirtualKeyCode::I,
    VirtualKeyCode::P,
    VirtualKeyCode::Enter,
    VirtualKeyCode::L,
    VirtualKeyCode::J,
    VirtualKeyCode::OEM7, // '"
    VirtualKeyCode::K,
    VirtualKeyCode::OEM1,     // ;:
    VirtualKeyCode::OEM5,     // \|
    VirtualKeyCode::OEMComma, // ,<
    VirtualKeyCode::OEM2,     // /?
    VirtualKeyCode::N,
    VirtualKeyCode::M,
    VirtualKeyCode::OEMPeriod, // .>
    VirtualKeyCode::Tab,
    VirtualKeyCode::Space,
    VirtualKeyCode::OEM3, // `~
    VirtualKeyCode::Backspace,
    VirtualKeyCode::Unknown, // n/a
    VirtualKeyCode::Escape,
    VirtualKeyCode::App, // Right command
    VirtualKeyCode::LeftPlatform,
    VirtualKeyCode::Shift,
    VirtualKeyCode::Capital,  // Capslock
    VirtualKeyCode::Alt,      // Left option
    VirtualKeyCode::Control,  // Left control
    VirtualKeyCode::Shift,    // Right shift
    VirtualKeyCode::Alt,      // Right option
    VirtualKeyCode::Control,  // Right control
    VirtualKeyCode::Function, // TODO: VK_UNKNOWN on Chrome
    VirtualKeyCode::F17,
    VirtualKeyCode::Decimal,  // Numpad .
    VirtualKeyCode::Unknown,  // n/a
    VirtualKeyCode::Multiply, // Numpad *
    VirtualKeyCode::Unknown,  // n/a
    VirtualKeyCode::Add,      // Numpad +
    VirtualKeyCode::Unknown,  // n/a
    VirtualKeyCode::Clear,    // Numpad clear
    VirtualKeyCode::VolumeUp,
    VirtualKeyCode::VolumeDown,
    VirtualKeyCode::VolumeMute,
    VirtualKeyCode::Divide,   // Numpad /
    VirtualKeyCode::Enter,    // Numpad enter
    VirtualKeyCode::Unknown,  // n/a
    VirtualKeyCode::Subtract, // Numpad -
    VirtualKeyCode::F18,
    VirtualKeyCode::F19,
    VirtualKeyCode::OEMPlus, // Numpad =.
    VirtualKeyCode::Numpad0,
    VirtualKeyCode::Numpad1,
    VirtualKeyCode::Numpad2,
    VirtualKeyCode::Numpad3,
    VirtualKeyCode::Numpad4,
    VirtualKeyCode::Numpad5,
    VirtualKeyCode::Numpad6,
    VirtualKeyCode::Numpad7,
    VirtualKeyCode::F20,
    VirtualKeyCode::Numpad8,
    VirtualKeyCode::Numpad9,
    VirtualKeyCode::Unknown, // Yen, JIS keyboad only
    VirtualKeyCode::Unknown, // Underscore, JIS keyboard only
    VirtualKeyCode::Unknown, // Keypad comma, JIS keyboard only
    VirtualKeyCode::F5,
    VirtualKeyCode::F6,
    VirtualKeyCode::F7,
    VirtualKeyCode::F3,
    VirtualKeyCode::F8,
    VirtualKeyCode::F9,
    VirtualKeyCode::Unknown, // Eisu, JIS keyboard only
    VirtualKeyCode::F11,
    VirtualKeyCode::Unknown, // Kana, JIS keyboard only
    VirtualKeyCode::F13,
    VirtualKeyCode::F16,
    VirtualKeyCode::F14,
    VirtualKeyCode::Unknown, // n/a
    VirtualKeyCode::F10,
    VirtualKeyCode::App, // Context menu key
    VirtualKeyCode::F12,
    VirtualKeyCode::Unknown, // n/a
    VirtualKeyCode::F15,
    VirtualKeyCode::Insert, // Help
    VirtualKeyCode::Home,   // Home
    VirtualKeyCode::PageUp,
    VirtualKeyCode::Delete, // Forward delete
    VirtualKeyCode::F4,
    VirtualKeyCode::End,
    VirtualKeyCode::F2,
    VirtualKeyCode::PageDown,
    VirtualKeyCode::F1,
    VirtualKeyCode::Left,
    VirtualKeyCode::Right,
    VirtualKeyCode::Down,
    VirtualKeyCode::Up,
    VirtualKeyCode::Unknown, // n/a
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
const KEYPAD_CLEAR: u16 = 0x47;
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
