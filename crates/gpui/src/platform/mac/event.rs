use crate::{geometry::vector::vec2f, keymap::Keystroke, platform::Event};
use cocoa::{
    appkit::{NSEvent, NSEventModifierFlags, NSEventType},
    base::{id, nil, YES},
    foundation::NSString as _,
};
use std::{ffi::CStr, os::raw::c_char};

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
                let mut input = None;
                let modifiers = native_event.modifierFlags();
                let unmodified_chars = CStr::from_ptr(
                    native_event.charactersIgnoringModifiers().UTF8String() as *mut c_char,
                )
                .to_str()
                .unwrap();

                let unmodified_chars = if let Some(first_char) = unmodified_chars.chars().next() {
                    use cocoa::appkit::*;
                    const BACKSPACE_KEY: u16 = 0x7f;
                    const ENTER_KEY: u16 = 0x0d;
                    const ESCAPE_KEY: u16 = 0x1b;
                    const TAB_KEY: u16 = 0x09;
                    const SHIFT_TAB_KEY: u16 = 0x19;
                    const SPACE_KEY: u16 = b' ' as u16;

                    #[allow(non_upper_case_globals)]
                    match first_char as u16 {
                        SPACE_KEY => {
                            input = Some(" ".to_string());
                            "space"
                        }
                        BACKSPACE_KEY => "backspace",
                        ENTER_KEY => "enter",
                        ESCAPE_KEY => "escape",
                        TAB_KEY => "tab",
                        SHIFT_TAB_KEY => "tab",

                        NSUpArrowFunctionKey => "up",
                        NSDownArrowFunctionKey => "down",
                        NSLeftArrowFunctionKey => "left",
                        NSRightArrowFunctionKey => "right",
                        NSPageUpFunctionKey => "pageup",
                        NSPageDownFunctionKey => "pagedown",
                        NSDeleteFunctionKey => "delete",
                        NSF1FunctionKey => "f1",
                        NSF2FunctionKey => "f2",
                        NSF3FunctionKey => "f3",
                        NSF4FunctionKey => "f4",
                        NSF5FunctionKey => "f5",
                        NSF6FunctionKey => "f6",
                        NSF7FunctionKey => "f7",
                        NSF8FunctionKey => "f8",
                        NSF9FunctionKey => "f9",
                        NSF10FunctionKey => "f10",
                        NSF11FunctionKey => "f11",
                        NSF12FunctionKey => "f12",

                        _ => {
                            input = Some(
                                CStr::from_ptr(
                                    native_event.characters().UTF8String() as *mut c_char
                                )
                                .to_str()
                                .unwrap()
                                .into(),
                            );
                            unmodified_chars
                        }
                    }
                } else {
                    return None;
                };

                Some(Self::KeyDown {
                    keystroke: Keystroke {
                        ctrl: modifiers.contains(NSEventModifierFlags::NSControlKeyMask),
                        alt: modifiers.contains(NSEventModifierFlags::NSAlternateKeyMask),
                        shift: modifiers.contains(NSEventModifierFlags::NSShiftKeyMask),
                        cmd: modifiers.contains(NSEventModifierFlags::NSCommandKeyMask),
                        key: unmodified_chars.into(),
                    },
                    input,
                    is_held: native_event.isARepeat() == YES,
                })
            }
            NSEventType::NSLeftMouseDown => {
                let modifiers = native_event.modifierFlags();
                window_height.map(|window_height| Self::LeftMouseDown {
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
                left_mouse_down: NSEvent::pressedMouseButtons(nil) & 1 != 0,
            }),
            _ => None,
        }
    }
}
