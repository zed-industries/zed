use crate::{
    geometry::vector::vec2f,
    keymap::Keystroke,
    platform::{Event, NavigationDirection},
    KeyDownEvent, KeyUpEvent, ModifiersChangedEvent, MouseButton, MouseButtonEvent,
    MouseMovedEvent, ScrollWheelEvent,
};
use cocoa::{
    appkit::{NSEvent, NSEventModifierFlags, NSEventType},
    base::{id, YES},
    foundation::NSString as _,
};
use core_graphics::{
    event::{CGEvent, CGEventFlags, CGKeyCode},
    event_source::{CGEventSource, CGEventSourceStateID},
};
use objc::{class, msg_send, sel, sel_impl};
use std::{borrow::Cow, ffi::CStr, os::raw::c_char};

const BACKSPACE_KEY: u16 = 0x7f;
const SPACE_KEY: u16 = b' ' as u16;
const ENTER_KEY: u16 = 0x0d;
const NUMPAD_ENTER_KEY: u16 = 0x03;
const ESCAPE_KEY: u16 = 0x1b;
const TAB_KEY: u16 = 0x09;
const SHIFT_TAB_KEY: u16 = 0x19;

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
        "delete" => NSDeleteFunctionKey,
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
        _ => return Cow::Borrowed(key),
    };
    Cow::Owned(String::from_utf16(&[code]).unwrap())
}

impl Event {
    pub unsafe fn from_native(native_event: id, window_height: Option<f32>) -> Option<Self> {
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
            NSEventType::NSFlagsChanged => {
                let modifiers = native_event.modifierFlags();
                let ctrl = modifiers.contains(NSEventModifierFlags::NSControlKeyMask);
                let alt = modifiers.contains(NSEventModifierFlags::NSAlternateKeyMask);
                let shift = modifiers.contains(NSEventModifierFlags::NSShiftKeyMask);
                let cmd = modifiers.contains(NSEventModifierFlags::NSCommandKeyMask);

                Some(Self::ModifiersChanged(ModifiersChangedEvent {
                    ctrl,
                    alt,
                    shift,
                    cmd,
                }))
            }
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
                let modifiers = native_event.modifierFlags();

                window_height.map(|window_height| {
                    Self::MouseDown(MouseButtonEvent {
                        button,
                        position: vec2f(
                            native_event.locationInWindow().x as f32,
                            window_height - native_event.locationInWindow().y as f32,
                        ),
                        ctrl: modifiers.contains(NSEventModifierFlags::NSControlKeyMask),
                        alt: modifiers.contains(NSEventModifierFlags::NSAlternateKeyMask),
                        shift: modifiers.contains(NSEventModifierFlags::NSShiftKeyMask),
                        cmd: modifiers.contains(NSEventModifierFlags::NSCommandKeyMask),
                        click_count: native_event.clickCount() as usize,
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
                    let modifiers = native_event.modifierFlags();
                    Self::MouseUp(MouseButtonEvent {
                        button,
                        position: vec2f(
                            native_event.locationInWindow().x as f32,
                            window_height - native_event.locationInWindow().y as f32,
                        ),
                        ctrl: modifiers.contains(NSEventModifierFlags::NSControlKeyMask),
                        alt: modifiers.contains(NSEventModifierFlags::NSAlternateKeyMask),
                        shift: modifiers.contains(NSEventModifierFlags::NSShiftKeyMask),
                        cmd: modifiers.contains(NSEventModifierFlags::NSCommandKeyMask),
                        click_count: native_event.clickCount() as usize,
                    })
                })
            }
            NSEventType::NSScrollWheel => window_height.map(|window_height| {
                Self::ScrollWheel(ScrollWheelEvent {
                    position: vec2f(
                        native_event.locationInWindow().x as f32,
                        window_height - native_event.locationInWindow().y as f32,
                    ),
                    delta: vec2f(
                        native_event.scrollingDeltaX() as f32,
                        native_event.scrollingDeltaY() as f32,
                    ),
                    precise: native_event.hasPreciseScrollingDeltas() == YES,
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
                    let modifiers = native_event.modifierFlags();
                    Self::MouseMoved(MouseMovedEvent {
                        pressed_button: Some(pressed_button),
                        position: vec2f(
                            native_event.locationInWindow().x as f32,
                            window_height - native_event.locationInWindow().y as f32,
                        ),
                        ctrl: modifiers.contains(NSEventModifierFlags::NSControlKeyMask),
                        alt: modifiers.contains(NSEventModifierFlags::NSAlternateKeyMask),
                        shift: modifiers.contains(NSEventModifierFlags::NSShiftKeyMask),
                        cmd: modifiers.contains(NSEventModifierFlags::NSCommandKeyMask),
                    })
                })
            }
            NSEventType::NSMouseMoved => window_height.map(|window_height| {
                let modifiers = native_event.modifierFlags();
                Self::MouseMoved(MouseMovedEvent {
                    position: vec2f(
                        native_event.locationInWindow().x as f32,
                        window_height - native_event.locationInWindow().y as f32,
                    ),
                    pressed_button: None,
                    ctrl: modifiers.contains(NSEventModifierFlags::NSControlKeyMask),
                    alt: modifiers.contains(NSEventModifierFlags::NSAlternateKeyMask),
                    shift: modifiers.contains(NSEventModifierFlags::NSShiftKeyMask),
                    cmd: modifiers.contains(NSEventModifierFlags::NSCommandKeyMask),
                })
            }),
            _ => None,
        }
    }
}

unsafe fn parse_keystroke(native_event: id) -> Keystroke {
    use cocoa::appkit::*;

    let mut chars_ignoring_modifiers =
        CStr::from_ptr(native_event.charactersIgnoringModifiers().UTF8String() as *mut c_char)
            .to_str()
            .unwrap();
    let first_char = chars_ignoring_modifiers.chars().next().map(|ch| ch as u16);
    let modifiers = native_event.modifierFlags();

    let ctrl = modifiers.contains(NSEventModifierFlags::NSControlKeyMask);
    let alt = modifiers.contains(NSEventModifierFlags::NSAlternateKeyMask);
    let mut shift = modifiers.contains(NSEventModifierFlags::NSShiftKeyMask);
    let cmd = modifiers.contains(NSEventModifierFlags::NSCommandKeyMask);
    let function = modifiers.contains(NSEventModifierFlags::NSFunctionKeyMask)
        && first_char.map_or(true, |ch| {
            ch < NSUpArrowFunctionKey || ch > NSModeSwitchFunctionKey
        });

    #[allow(non_upper_case_globals)]
    let key = match first_char {
        Some(SPACE_KEY) => "space",
        Some(BACKSPACE_KEY) => "backspace",
        Some(ENTER_KEY) | Some(NUMPAD_ENTER_KEY) => "enter",
        Some(ESCAPE_KEY) => "escape",
        Some(TAB_KEY) => "tab",
        Some(SHIFT_TAB_KEY) => "tab",
        Some(NSUpArrowFunctionKey) => "up",
        Some(NSDownArrowFunctionKey) => "down",
        Some(NSLeftArrowFunctionKey) => "left",
        Some(NSRightArrowFunctionKey) => "right",
        Some(NSPageUpFunctionKey) => "pageup",
        Some(NSPageDownFunctionKey) => "pagedown",
        Some(NSDeleteFunctionKey) => "delete",
        Some(NSF1FunctionKey) => "f1",
        Some(NSF2FunctionKey) => "f2",
        Some(NSF3FunctionKey) => "f3",
        Some(NSF4FunctionKey) => "f4",
        Some(NSF5FunctionKey) => "f5",
        Some(NSF6FunctionKey) => "f6",
        Some(NSF7FunctionKey) => "f7",
        Some(NSF8FunctionKey) => "f8",
        Some(NSF9FunctionKey) => "f9",
        Some(NSF10FunctionKey) => "f10",
        Some(NSF11FunctionKey) => "f11",
        Some(NSF12FunctionKey) => "f12",
        _ => {
            let mut chars_ignoring_modifiers_and_shift =
                chars_for_modified_key(native_event.keyCode(), false, false);

            // Honor ⌘ when Dvorak-QWERTY is used.
            let chars_with_cmd = chars_for_modified_key(native_event.keyCode(), true, false);
            if cmd && chars_ignoring_modifiers_and_shift != chars_with_cmd {
                chars_ignoring_modifiers =
                    chars_for_modified_key(native_event.keyCode(), true, shift);
                chars_ignoring_modifiers_and_shift = chars_with_cmd;
            }

            if shift {
                if chars_ignoring_modifiers_and_shift
                    == chars_ignoring_modifiers.to_ascii_lowercase()
                {
                    chars_ignoring_modifiers_and_shift
                } else if chars_ignoring_modifiers_and_shift != chars_ignoring_modifiers {
                    shift = false;
                    chars_ignoring_modifiers
                } else {
                    chars_ignoring_modifiers
                }
            } else {
                chars_ignoring_modifiers
            }
        }
    };

    Keystroke {
        ctrl,
        alt,
        shift,
        cmd,
        function,
        key: key.into(),
    }
}

fn chars_for_modified_key<'a>(code: CGKeyCode, cmd: bool, shift: bool) -> &'a str {
    // Ideally, we would use `[NSEvent charactersByApplyingModifiers]` but that
    // always returns an empty string with certain keyboards, e.g. Japanese. Synthesizing
    // an event with the given flags instead lets us access `characters`, which always
    // returns a valid string.
    let event = CGEvent::new_keyboard_event(
        CGEventSource::new(CGEventSourceStateID::Private).unwrap(),
        code,
        true,
    )
    .unwrap();
    let mut flags = CGEventFlags::empty();
    if cmd {
        flags |= CGEventFlags::CGEventFlagCommand;
    }
    if shift {
        flags |= CGEventFlags::CGEventFlagShift;
    }
    event.set_flags(flags);

    let event: id = unsafe { msg_send![class!(NSEvent), eventWithCGEvent: event] };
    unsafe {
        CStr::from_ptr(event.characters().UTF8String())
            .to_str()
            .unwrap()
    }
}
