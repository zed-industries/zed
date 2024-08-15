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
use std::borrow::Cow;

const BACKSPACE_KEY: u16 = 0x7f;
const SPACE_KEY: u16 = b' ' as u16;

fn physical_key_from_scancode(scancode: u16) -> Option<&'static str> {
    match scancode {
        0x00 => Some("a"),
        0x01 => Some("s"),
        0x02 => Some("d"),
        0x03 => Some("f"),
        0x04 => Some("h"),
        0x05 => Some("g"),
        0x06 => Some("z"),
        0x07 => Some("x"),
        0x08 => Some("c"),
        0x09 => Some("v"),
        0x0b => Some("b"),
        0x0c => Some("q"),
        0x0d => Some("w"),
        0x0e => Some("e"),
        0x0f => Some("r"),
        0x10 => Some("y"),
        0x11 => Some("t"),
        0x12 => Some("1"),
        0x13 => Some("2"),
        0x14 => Some("3"),
        0x15 => Some("4"),
        0x16 => Some("6"),
        0x17 => Some("5"),
        0x18 => Some("="),
        0x19 => Some("9"),
        0x1a => Some("7"),
        0x1b => Some("-"),
        0x1c => Some("8"),
        0x1d => Some("0"),
        0x1e => Some("]"),
        0x1f => Some("o"),
        0x20 => Some("u"),
        0x21 => Some("["),
        0x22 => Some("i"),
        0x23 => Some("p"),
        0x24 => Some("enter"),
        0x25 => Some("l"),
        0x26 => Some("j"),
        0x27 => Some("'"),
        0x28 => Some("k"),
        0x29 => Some(";"),
        0x2a => Some("\\"),
        0x2b => Some(","),
        0x2c => Some("/"),
        0x2d => Some("n"),
        0x2e => Some("m"),
        0x2f => Some("."),
        0x30 => Some("tab"),
        0x31 => Some("space"),
        0x32 => Some("`"),
        0x33 => Some("backspace"),
        0x35 => Some("escape"),
        0x40 => Some("f17"),
        0x41 => Some("."),
        0x43 => Some("*"),
        0x45 => Some("+"),
        0x4b => Some("/"),
        0x4c => Some("enter"),
        0x4e => Some("-"),
        0x4f => Some("f18"),
        0x50 => Some("f19"),
        0x51 => Some("="),
        0x52 => Some("0"),
        0x53 => Some("1"),
        0x54 => Some("2"),
        0x55 => Some("3"),
        0x56 => Some("4"),
        0x57 => Some("5"),
        0x58 => Some("6"),
        0x59 => Some("7"),
        0x5a => Some("f20"),
        0x5b => Some("8"),
        0x5c => Some("9"),
        0x5d => Some("¥"),
        0x60 => Some("f5"),
        0x61 => Some("f6"),
        0x62 => Some("f7"),
        0x63 => Some("f3"),
        0x64 => Some("f8"),
        0x65 => Some("f9"),
        0x67 => Some("f11"),
        0x69 => Some("f13"),
        0x6a => Some("f16"),
        0x6b => Some("f14"),
        0x6d => Some("f10"),
        0x6f => Some("f12"),
        0x71 => Some("f15"),
        0x72 => Some("insert"),
        0x73 => Some("home"),
        0x74 => Some("pageup"),
        0x75 => Some("delete"),
        0x76 => Some("f4"),
        0x77 => Some("end"),
        0x78 => Some("f2"),
        0x79 => Some("pagedown"),
        0x7a => Some("f1"),
        0x7b => Some("left"),
        0x7c => Some("right"),
        0x7d => Some("down"),
        0x7e => Some("up"),
        0xa => Some("`"),
        _ => {
            log::error!("Unknown scancode: 0x{:x}", scancode);
            None
        }
    }
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

    let ime_key = native_event.characters().to_str().to_string();

    let mut modifiers = read_modifiers(native_event);
    modifiers.function &= !ime_key.chars().next().map_or(false, |ch| {
        matches!(ch as u16, NSUpArrowFunctionKey..=NSModeSwitchFunctionKey)
    });

    let key = physical_key_from_scancode(native_event.keyCode())
        .unwrap_or_default()
        .to_string();

    let ime_key = (!ime_key.is_empty()).then(|| ime_key);

    Keystroke {
        modifiers,
        key,
        ime_key,
        ime_inputs: Default::default(),
    }
}
