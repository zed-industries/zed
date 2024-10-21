use crate::{
    platform::mac::NSStringExt, point, px, KeyDownEvent, KeyUpEvent, Keystroke, Modifiers,
    ModifiersChangedEvent, MouseButton, MouseDownEvent, MouseExitEvent, MouseMoveEvent,
    MouseUpEvent, NavigationDirection, Pixels, PlatformInput, ScrollDelta, ScrollWheelEvent,
    TouchPhase,
};
use cocoa::{
    appkit::{NSEvent, NSEventModifierFlags, NSEventPhase, NSEventType},
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

    #[allow(non_upper_case_globals)]
    let key = match first_char {
        Some(SPACE_KEY) => "space".to_string(),
        Some(BACKSPACE_KEY) => "backspace".to_string(),
        Some(ENTER_KEY) | Some(NUMPAD_ENTER_KEY) => "enter".to_string(),
        Some(ESCAPE_KEY) => "escape".to_string(),
        Some(TAB_KEY) => "tab".to_string(),
        Some(SHIFT_TAB_KEY) => "tab".to_string(),
        Some(NSUpArrowFunctionKey) => "up".to_string(),
        Some(NSDownArrowFunctionKey) => "down".to_string(),
        Some(NSLeftArrowFunctionKey) => "left".to_string(),
        Some(NSRightArrowFunctionKey) => "right".to_string(),
        Some(NSPageUpFunctionKey) => "pageup".to_string(),
        Some(NSPageDownFunctionKey) => "pagedown".to_string(),
        Some(NSHomeFunctionKey) => "home".to_string(),
        Some(NSEndFunctionKey) => "end".to_string(),
        Some(NSDeleteFunctionKey) => "delete".to_string(),
        // Observed Insert==NSHelpFunctionKey not NSInsertFunctionKey.
        Some(NSHelpFunctionKey) => "insert".to_string(),
        Some(NSF1FunctionKey) => "f1".to_string(),
        Some(NSF2FunctionKey) => "f2".to_string(),
        Some(NSF3FunctionKey) => "f3".to_string(),
        Some(NSF4FunctionKey) => "f4".to_string(),
        Some(NSF5FunctionKey) => "f5".to_string(),
        Some(NSF6FunctionKey) => "f6".to_string(),
        Some(NSF7FunctionKey) => "f7".to_string(),
        Some(NSF8FunctionKey) => "f8".to_string(),
        Some(NSF9FunctionKey) => "f9".to_string(),
        Some(NSF10FunctionKey) => "f10".to_string(),
        Some(NSF11FunctionKey) => "f11".to_string(),
        Some(NSF12FunctionKey) => "f12".to_string(),
        Some(NSF13FunctionKey) => "f13".to_string(),
        Some(NSF14FunctionKey) => "f14".to_string(),
        Some(NSF15FunctionKey) => "f15".to_string(),
        Some(NSF16FunctionKey) => "f16".to_string(),
        Some(NSF17FunctionKey) => "f17".to_string(),
        Some(NSF18FunctionKey) => "f18".to_string(),
        Some(NSF19FunctionKey) => "f19".to_string(),
        _ => {
            let mut chars_ignoring_modifiers_and_shift =
                chars_for_modified_key(native_event.keyCode(), false, false);

            // Honor ⌘ when Dvorak-QWERTY is used.
            let chars_with_cmd = chars_for_modified_key(native_event.keyCode(), true, false);
            if command && chars_ignoring_modifiers_and_shift != chars_with_cmd {
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
        modifiers: Modifiers {
            control,
            alt,
            shift,
            platform: command,
            function,
        },
        key,
        ime_key: None,
    }
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
