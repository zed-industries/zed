use crate::{geometry::vector::vec2f, keymap::Keystroke, platform::Event};
use cocoa::appkit::{
    NSDeleteFunctionKey as DELETE_KEY, NSDownArrowFunctionKey as ARROW_DOWN_KEY,
    NSLeftArrowFunctionKey as ARROW_LEFT_KEY, NSPageDownFunctionKey as PAGE_DOWN_KEY,
    NSPageUpFunctionKey as PAGE_UP_KEY, NSRightArrowFunctionKey as ARROW_RIGHT_KEY,
    NSUpArrowFunctionKey as ARROW_UP_KEY,
};
use cocoa::{
    appkit::{NSEvent as _, NSEventModifierFlags, NSEventType},
    base::{id, YES},
    foundation::NSString as _,
};
use std::{ffi::CStr, os::raw::c_char};

const BACKSPACE_KEY: u16 = 0x7f;
const ENTER_KEY: u16 = 0x0d;
const ESCAPE_KEY: u16 = 0x1b;
const TAB_KEY: u16 = 0x09;

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
            NSEventType::NSKeyDown => {
                let modifiers = native_event.modifierFlags();
                let unmodified_chars = native_event.charactersIgnoringModifiers();
                let unmodified_chars = CStr::from_ptr(unmodified_chars.UTF8String() as *mut c_char)
                    .to_str()
                    .unwrap();

                let unmodified_chars = if let Some(first_char) = unmodified_chars.chars().next() {
                    match first_char as u16 {
                        ARROW_UP_KEY => "up",
                        ARROW_DOWN_KEY => "down",
                        ARROW_LEFT_KEY => "left",
                        ARROW_RIGHT_KEY => "right",
                        PAGE_UP_KEY => "pageup",
                        PAGE_DOWN_KEY => "pagedown",
                        BACKSPACE_KEY => "backspace",
                        ENTER_KEY => "enter",
                        DELETE_KEY => "delete",
                        ESCAPE_KEY => "escape",
                        TAB_KEY => "tab",
                        _ => unmodified_chars,
                    }
                } else {
                    return None;
                };

                let chars = native_event.characters();
                let chars = CStr::from_ptr(chars.UTF8String() as *mut c_char)
                    .to_str()
                    .unwrap()
                    .into();

                Some(Self::KeyDown {
                    keystroke: Keystroke {
                        ctrl: modifiers.contains(NSEventModifierFlags::NSControlKeyMask),
                        alt: modifiers.contains(NSEventModifierFlags::NSAlternateKeyMask),
                        shift: modifiers.contains(NSEventModifierFlags::NSShiftKeyMask),
                        cmd: modifiers.contains(NSEventModifierFlags::NSCommandKeyMask),
                        key: unmodified_chars.into(),
                    },
                    chars,
                    is_held: native_event.isARepeat() == YES,
                })
            }
            NSEventType::NSLeftMouseDown => {
                window_height.map(|window_height| Self::LeftMouseDown {
                    position: vec2f(
                        native_event.locationInWindow().x as f32,
                        window_height - native_event.locationInWindow().y as f32,
                    ),
                    cmd: native_event
                        .modifierFlags()
                        .contains(NSEventModifierFlags::NSCommandKeyMask),
                })
            }
            NSEventType::NSLeftMouseUp => window_height.map(|window_height| Self::LeftMouseUp {
                position: vec2f(
                    native_event.locationInWindow().x as f32,
                    window_height - native_event.locationInWindow().y as f32,
                ),
            }),
            NSEventType::NSLeftMouseDragged => {
                window_height.map(|window_height| Self::LeftMouseDragged {
                    position: vec2f(
                        native_event.locationInWindow().x as f32,
                        window_height - native_event.locationInWindow().y as f32,
                    ),
                })
            }
            NSEventType::NSScrollWheel => window_height.map(|window_height| Self::ScrollWheel {
                position: vec2f(
                    native_event.locationInWindow().x as f32,
                    window_height - native_event.locationInWindow().y as f32,
                ),
                delta: vec2f(
                    native_event.scrollingDeltaX() as f32,
                    native_event.scrollingDeltaY() as f32,
                ),
                precise: native_event.hasPreciseScrollingDeltas() == YES,
            }),
            NSEventType::NSMouseMoved => window_height.map(|window_height| Self::MouseMoved {
                position: vec2f(
                    native_event.locationInWindow().x as f32,
                    window_height - native_event.locationInWindow().y as f32,
                ),
            }),
            _ => None,
        }
    }
}
