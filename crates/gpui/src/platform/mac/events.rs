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

fn physical_key_from_scancode(scancode: u16, shift: bool) -> Option<&'static str> {
    match (scancode, shift) {
        (0x00, false) => Some("a"),
        (0x00, true) => Some("A"),

        (0x01, false) => Some("s"),
        (0x01, true) => Some("S"),

        (0x02, false) => Some("d"),
        (0x02, true) => Some("D"),

        (0x03, false) => Some("f"),
        (0x03, true) => Some("F"),

        (0x04, false) => Some("h"),
        (0x04, true) => Some("H"),

        (0x05, false) => Some("g"),
        (0x05, true) => Some("G"),

        (0x06, false) => Some("z"),
        (0x06, true) => Some("Z"),

        (0x07, false) => Some("x"),
        (0x07, true) => Some("X"),

        (0x08, false) => Some("c"),
        (0x08, true) => Some("C"),

        (0x09, false) => Some("v"),
        (0x09, true) => Some("V"),

        (0x0b, false) => Some("b"),
        (0x0b, true) => Some("B"),

        (0x0c, false) => Some("q"),
        (0x0c, true) => Some("Q"),

        (0x0d, false) => Some("w"),
        (0x0d, true) => Some("W"),

        (0x0e, false) => Some("e"),
        (0x0e, true) => Some("E"),

        (0x0f, false) => Some("r"),
        (0x0f, true) => Some("R"),

        (0x10, false) => Some("y"),
        (0x10, true) => Some("Y"),

        (0x11, false) => Some("t"),
        (0x11, true) => Some("T"),

        (0x12, false) => Some("1"),
        (0x12, true) => Some("!"),

        (0x13, false) => Some("2"),
        (0x13, true) => Some("@"),

        (0x14, false) => Some("3"),
        (0x14, true) => Some("#"),

        (0x15, false) => Some("4"),
        (0x15, true) => Some("$"),

        (0x16, false) => Some("6"),
        (0x16, true) => Some("^"),

        (0x17, false) => Some("5"),
        (0x17, true) => Some("%"),

        (0x18, false) => Some("="),
        (0x18, true) => Some("+"),

        (0x19, false) => Some("9"),
        (0x19, true) => Some("("),

        (0x1a, false) => Some("7"),
        (0x1a, true) => Some("&"),

        (0x1b, false) => Some("-"),
        (0x1b, true) => Some("_"),

        (0x1c, false) => Some("8"),
        (0x1c, true) => Some("*"),

        (0x1d, false) => Some("0"),
        (0x1d, true) => Some(")"),

        (0x1e, false) => Some("]"),
        (0x1e, true) => Some("}"),

        (0x1f, false) => Some("o"),
        (0x1f, true) => Some("O"),

        (0x20, false) => Some("u"),
        (0x20, true) => Some("U"),

        (0x21, false) => Some("["),
        (0x21, true) => Some("{"),

        (0x22, false) => Some("i"),
        (0x22, true) => Some("I"),

        (0x23, false) => Some("p"),
        (0x23, true) => Some("P"),

        (0x24, _) => Some("enter"),

        (0x25, false) => Some("l"),
        (0x25, true) => Some("L"),

        (0x26, false) => Some("j"),
        (0x26, true) => Some("J"),

        (0x27, false) => Some("'"),
        (0x27, true) => Some("\""),

        (0x28, false) => Some("k"),
        (0x28, true) => Some("K"),

        (0x29, false) => Some(";"),
        (0x29, true) => Some(":"),

        (0x2a, false) => Some("\\"),
        (0x2a, true) => Some("|"),

        (0x2b, false) => Some(","),
        (0x2b, true) => Some("<"),

        (0x2c, false) => Some("/"),
        (0x2c, true) => Some("?"),

        (0x2d, false) => Some("n"),
        (0x2d, true) => Some("N"),

        (0x2e, false) => Some("m"),
        (0x2e, true) => Some("M"),

        (0x2f, false) => Some("."),
        (0x2f, true) => Some(">"),

        (0x30, _) => Some("tab"),

        (0x31, _) => Some("space"),

        (0x32, false) => Some("`"),
        (0x32, true) => Some("~"),

        (0x33, _) => Some("backspace"),

        (0x35, _) => Some("escape"),

        (0x40, _) => Some("f17"),

        (0x41, false) => Some("."),
        (0x41, true) => Some(">"),

        (0x43, _) => Some("*"),

        (0x45, _) => Some("+"),

        (0x4b, _) => Some("/"),

        (0x4c, _) => Some("enter"),

        (0x4e, _) => Some("-"),

        (0x4f, _) => Some("f18"),

        (0x50, _) => Some("f19"),

        (0x51, false) => Some("="),
        (0x51, true) => Some("+"),

        (0x52, false) => Some("0"),
        (0x52, true) => Some(")"),

        (0x53, false) => Some("1"),
        (0x53, true) => Some("!"),

        (0x54, false) => Some("2"),
        (0x54, true) => Some("@"),

        (0x55, false) => Some("3"),
        (0x55, true) => Some("#"),

        (0x56, false) => Some("4"),
        (0x56, true) => Some("$"),

        (0x57, false) => Some("5"),
        (0x57, true) => Some("%"),

        (0x58, false) => Some("6"),
        (0x58, true) => Some("^"),

        (0x59, false) => Some("7"),
        (0x59, true) => Some("&"),

        (0x5a, _) => Some("f20"),

        (0x5b, false) => Some("8"),
        (0x5b, true) => Some("*"),

        (0x5c, false) => Some("9"),
        (0x5c, true) => Some("("),

        (0x5d, _) => Some("¥"),

        (0x60, _) => Some("f5"),

        (0x61, _) => Some("f6"),

        (0x62, _) => Some("f7"),

        (0x63, _) => Some("f3"),

        (0x64, _) => Some("f8"),

        (0x65, _) => Some("f9"),

        (0x67, _) => Some("f11"),

        (0x69, _) => Some("f13"),

        (0x6a, _) => Some("f16"),

        (0x6b, _) => Some("f14"),

        (0x6d, _) => Some("f10"),

        (0x6f, _) => Some("f12"),

        (0x71, _) => Some("f15"),

        (0x72, _) => Some("insert"),

        (0x73, _) => Some("home"),

        (0x74, _) => Some("pageup"),

        (0x75, _) => Some("delete"),

        (0x76, _) => Some("f4"),

        (0x77, _) => Some("end"),

        (0x78, _) => Some("f2"),

        (0x79, _) => Some("pagedown"),

        (0x7a, _) => Some("f1"),

        (0x7b, _) => Some("left"),

        (0x7c, _) => Some("right"),

        (0x7d, _) => Some("down"),

        (0x7e, _) => Some("up"),

        (0xa, false) => Some("`"),
        (0xa, true) => Some("~"),

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

    let scancode = native_event.keyCode();

    let mut ime_key = native_event.characters().to_str().to_string();

    let mut modifiers = read_modifiers(native_event);
    modifiers.function &= !ime_key.chars().next().map_or(false, |ch| {
        matches!(ch as u16, NSUpArrowFunctionKey..=NSModeSwitchFunctionKey)
    });

    let key = physical_key_from_scancode(scancode, false)
        .unwrap_or_default()
        .to_string();

    let ime_key = if modifiers.shift && (modifiers.control || modifiers.platform) {
        physical_key_from_scancode(scancode, true).map(str::to_string)
    } else {
        Some(ime_key)
    }
    .take_if(|ime_key| !ime_key.is_empty());

    let keystroke = Keystroke {
        modifiers,
        key,
        ime_key,
        ime_inputs: Default::default(),
    };
    keystroke
}
