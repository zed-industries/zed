use crate::{
    geometry::vector::vec2f,
    keymap::Keystroke,
    platform::{Event, NavigationDirection},
};
use cocoa::{
    appkit::{NSEvent, NSEventModifierFlags, NSEventType},
    base::{id, nil, YES},
    foundation::NSString as _,
};
use std::{borrow::Cow, ffi::CStr, os::raw::c_char};

pub fn key_to_native(key: &str) -> Cow<str> {
    use cocoa::appkit::*;
    let code = match key {
        "backspace" => 0x7F,
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

                Some(Self::ModifiersChanged {
                    ctrl,
                    alt,
                    shift,
                    cmd,
                })
            }
            NSEventType::NSKeyDown => {
                let modifiers = native_event.modifierFlags();
                let ctrl = modifiers.contains(NSEventModifierFlags::NSControlKeyMask);
                let alt = modifiers.contains(NSEventModifierFlags::NSAlternateKeyMask);
                let shift = modifiers.contains(NSEventModifierFlags::NSShiftKeyMask);
                let cmd = modifiers.contains(NSEventModifierFlags::NSCommandKeyMask);
                let function = modifiers.contains(NSEventModifierFlags::NSFunctionKeyMask);

                let (unmodified_chars, input) = get_key_text(native_event, cmd, ctrl, function)?;

                Some(Self::KeyDown {
                    keystroke: Keystroke {
                        ctrl,
                        alt,
                        shift,
                        cmd,
                        key: unmodified_chars.into(),
                    },
                    input,
                    is_held: native_event.isARepeat() == YES,
                })
            }
            NSEventType::NSKeyUp => {
                let modifiers = native_event.modifierFlags();
                let ctrl = modifiers.contains(NSEventModifierFlags::NSControlKeyMask);
                let alt = modifiers.contains(NSEventModifierFlags::NSAlternateKeyMask);
                let shift = modifiers.contains(NSEventModifierFlags::NSShiftKeyMask);
                let cmd = modifiers.contains(NSEventModifierFlags::NSCommandKeyMask);
                let function = modifiers.contains(NSEventModifierFlags::NSFunctionKeyMask);

                let (unmodified_chars, input) = get_key_text(native_event, cmd, ctrl, function)?;

                Some(Self::KeyUp {
                    keystroke: Keystroke {
                        ctrl,
                        alt,
                        shift,
                        cmd,
                        key: unmodified_chars.into(),
                    },
                    input,
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
                click_count: native_event.clickCount() as usize,
            }),
            NSEventType::NSRightMouseDown => {
                let modifiers = native_event.modifierFlags();
                window_height.map(|window_height| Self::RightMouseDown {
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
            NSEventType::NSRightMouseUp => window_height.map(|window_height| Self::RightMouseUp {
                position: vec2f(
                    native_event.locationInWindow().x as f32,
                    window_height - native_event.locationInWindow().y as f32,
                ),
                click_count: native_event.clickCount() as usize,
            }),
            NSEventType::NSOtherMouseDown => {
                let direction = match native_event.buttonNumber() {
                    3 => NavigationDirection::Back,
                    4 => NavigationDirection::Forward,
                    // Other mouse buttons aren't tracked currently
                    _ => return None,
                };

                let modifiers = native_event.modifierFlags();
                window_height.map(|window_height| Self::NavigateMouseDown {
                    position: vec2f(
                        native_event.locationInWindow().x as f32,
                        window_height - native_event.locationInWindow().y as f32,
                    ),
                    direction,
                    ctrl: modifiers.contains(NSEventModifierFlags::NSControlKeyMask),
                    alt: modifiers.contains(NSEventModifierFlags::NSAlternateKeyMask),
                    shift: modifiers.contains(NSEventModifierFlags::NSShiftKeyMask),
                    cmd: modifiers.contains(NSEventModifierFlags::NSCommandKeyMask),
                    click_count: native_event.clickCount() as usize,
                })
            }
            NSEventType::NSOtherMouseUp => {
                let direction = match native_event.buttonNumber() {
                    3 => NavigationDirection::Back,
                    4 => NavigationDirection::Forward,
                    // Other mouse buttons aren't tracked currently
                    _ => return None,
                };

                window_height.map(|window_height| Self::NavigateMouseUp {
                    position: vec2f(
                        native_event.locationInWindow().x as f32,
                        window_height - native_event.locationInWindow().y as f32,
                    ),
                    direction,
                })
            }
            NSEventType::NSLeftMouseDragged => window_height.map(|window_height| {
                let modifiers = native_event.modifierFlags();
                Self::LeftMouseDragged {
                    position: vec2f(
                        native_event.locationInWindow().x as f32,
                        window_height - native_event.locationInWindow().y as f32,
                    ),
                    ctrl: modifiers.contains(NSEventModifierFlags::NSControlKeyMask),
                    alt: modifiers.contains(NSEventModifierFlags::NSAlternateKeyMask),
                    shift: modifiers.contains(NSEventModifierFlags::NSShiftKeyMask),
                    cmd: modifiers.contains(NSEventModifierFlags::NSCommandKeyMask),
                }
            }),
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
            NSEventType::NSMouseMoved => window_height.map(|window_height| {
                let modifiers = native_event.modifierFlags();
                Self::MouseMoved {
                    position: vec2f(
                        native_event.locationInWindow().x as f32,
                        window_height - native_event.locationInWindow().y as f32,
                    ),
                    left_mouse_down: NSEvent::pressedMouseButtons(nil) & 1 != 0,
                    ctrl: modifiers.contains(NSEventModifierFlags::NSControlKeyMask),
                    alt: modifiers.contains(NSEventModifierFlags::NSAlternateKeyMask),
                    shift: modifiers.contains(NSEventModifierFlags::NSShiftKeyMask),
                    cmd: modifiers.contains(NSEventModifierFlags::NSCommandKeyMask),
                }
            }),
            _ => None,
        }
    }
}

unsafe fn get_key_text(
    native_event: id,
    cmd: bool,
    ctrl: bool,
    function: bool,
) -> Option<(&'static str, Option<String>)> {
    let unmodified_chars =
        CStr::from_ptr(native_event.charactersIgnoringModifiers().UTF8String() as *mut c_char)
            .to_str()
            .unwrap();

    let mut input = None;
    let first_char = unmodified_chars.chars().next()?;
    use cocoa::appkit::*;
    const BACKSPACE_KEY: u16 = 0x7f;
    const ENTER_KEY: u16 = 0x0d;
    const NUMPAD_ENTER_KEY: u16 = 0x03;
    const ESCAPE_KEY: u16 = 0x1b;
    const TAB_KEY: u16 = 0x09;
    const SHIFT_TAB_KEY: u16 = 0x19;
    const SPACE_KEY: u16 = b' ' as u16;

    #[allow(non_upper_case_globals)]
    let unmodified_chars = match first_char as u16 {
        SPACE_KEY => {
            input = Some(" ".to_string());
            "space"
        }
        BACKSPACE_KEY => "backspace",
        ENTER_KEY | NUMPAD_ENTER_KEY => "enter",
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
            if !cmd && !ctrl && !function {
                input = Some(
                    CStr::from_ptr(native_event.characters().UTF8String() as *mut c_char)
                        .to_str()
                        .unwrap()
                        .into(),
                );
            }
            unmodified_chars
        }
    };

    Some((unmodified_chars, input))
}
