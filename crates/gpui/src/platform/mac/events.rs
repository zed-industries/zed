use crate::{
    platform::mac::NSStringExt, point, px, KeyCode, KeyDownEvent, KeyUpEvent, Keystroke, Modifiers,
    ModifiersChangedEvent, MouseButton, MouseDownEvent, MouseExitEvent, MouseMoveEvent,
    MouseUpEvent, NavigationDirection, Pixels, PlatformInput, ScrollDelta, ScrollWheelEvent,
    TouchPhase,
};
use cocoa::{
    appkit::{NSEvent, NSEventModifierFlags, NSEventPhase, NSEventType},
    base::{id, YES},
};
use std::borrow::Cow;

fn scancode_to_physicalkey(scancode: u16) -> Option<KeyCode> {
    Some(match scancode {
        0x00 => KeyCode::KeyA,
        0x01 => KeyCode::KeyS,
        0x02 => KeyCode::KeyD,
        0x03 => KeyCode::KeyF,
        0x04 => KeyCode::KeyH,
        0x05 => KeyCode::KeyG,
        0x06 => KeyCode::KeyZ,
        0x07 => KeyCode::KeyX,
        0x08 => KeyCode::KeyC,
        0x09 => KeyCode::KeyV,
        // 0x0a => World 1,
        0x0b => KeyCode::KeyB,
        0x0c => KeyCode::KeyQ,
        0x0d => KeyCode::KeyW,
        0x0e => KeyCode::KeyE,
        0x0f => KeyCode::KeyR,
        0x10 => KeyCode::KeyY,
        0x11 => KeyCode::KeyT,
        0x12 => KeyCode::Digit1,
        0x13 => KeyCode::Digit2,
        0x14 => KeyCode::Digit3,
        0x15 => KeyCode::Digit4,
        0x16 => KeyCode::Digit6,
        0x17 => KeyCode::Digit5,
        0x18 => KeyCode::Equal,
        0x19 => KeyCode::Digit9,
        0x1a => KeyCode::Digit7,
        0x1b => KeyCode::Minus,
        0x1c => KeyCode::Digit8,
        0x1d => KeyCode::Digit0,
        0x1e => KeyCode::BracketRight,
        0x1f => KeyCode::KeyO,
        0x20 => KeyCode::KeyU,
        0x21 => KeyCode::BracketLeft,
        0x22 => KeyCode::KeyI,
        0x23 => KeyCode::KeyP,
        0x24 => KeyCode::Enter,
        0x25 => KeyCode::KeyL,
        0x26 => KeyCode::KeyJ,
        0x27 => KeyCode::Quote,
        0x28 => KeyCode::KeyK,
        0x29 => KeyCode::Semicolon,
        0x2a => KeyCode::Backslash,
        0x2b => KeyCode::Comma,
        0x2c => KeyCode::Slash,
        0x2d => KeyCode::KeyN,
        0x2e => KeyCode::KeyM,
        0x2f => KeyCode::Period,
        0x30 => KeyCode::Tab,
        0x31 => KeyCode::Space,
        0x32 => KeyCode::Backquote,
        0x33 => KeyCode::Backspace,
        // 0x34 => unknown,
        0x35 => KeyCode::Escape,
        0x36 => KeyCode::SuperRight,
        0x37 => KeyCode::SuperLeft,
        0x38 => KeyCode::ShiftLeft,
        0x39 => KeyCode::CapsLock,
        0x3a => KeyCode::AltLeft,
        0x3b => KeyCode::ControlLeft,
        0x3c => KeyCode::ShiftRight,
        0x3d => KeyCode::AltRight,
        0x3e => KeyCode::ControlRight,
        0x3f => KeyCode::Fn,
        0x40 => KeyCode::F17,
        0x41 => KeyCode::NumpadDecimal,
        // 0x42 -> unknown,
        0x43 => KeyCode::NumpadMultiply,
        // 0x44 => unknown,
        0x45 => KeyCode::NumpadAdd,
        // 0x46 => unknown,
        0x47 => KeyCode::NumLock,
        // 0x48 => KeyCode::NumpadClear,

        // TODO: (Artur) for me, kVK_VolumeUp is 0x48
        // macOS 10.11
        // /System/Library/Frameworks/Carbon.framework/Versions/A/Frameworks/HIToolbox.framework/
        // Versions/A/Headers/Events.h
        0x49 => KeyCode::AudioVolumeUp,
        0x4a => KeyCode::AudioVolumeDown,
        0x4b => KeyCode::NumpadDivide,
        0x4c => KeyCode::NumpadEnter,
        // 0x4d => unknown,
        0x4e => KeyCode::NumpadSubtract,
        0x4f => KeyCode::F18,
        0x50 => KeyCode::F19,
        0x51 => KeyCode::NumpadEqual,
        0x52 => KeyCode::Numpad0,
        0x53 => KeyCode::Numpad1,
        0x54 => KeyCode::Numpad2,
        0x55 => KeyCode::Numpad3,
        0x56 => KeyCode::Numpad4,
        0x57 => KeyCode::Numpad5,
        0x58 => KeyCode::Numpad6,
        0x59 => KeyCode::Numpad7,
        0x5a => KeyCode::F20,
        0x5b => KeyCode::Numpad8,
        0x5c => KeyCode::Numpad9,
        0x5d => KeyCode::IntlYen,
        // 0x5e => JIS Ro,
        // 0x5f => unknown,
        0x60 => KeyCode::F5,
        0x61 => KeyCode::F6,
        0x62 => KeyCode::F7,
        0x63 => KeyCode::F3,
        0x64 => KeyCode::F8,
        0x65 => KeyCode::F9,
        // 0x66 => JIS Eisuu (macOS),
        0x67 => KeyCode::F11,
        // 0x68 => JIS Kanna (macOS),
        0x69 => KeyCode::F13,
        0x6a => KeyCode::F16,
        0x6b => KeyCode::F14,
        // 0x6c => unknown,
        0x6d => KeyCode::F10,
        // 0x6e => unknown,
        0x6f => KeyCode::F12,
        // 0x70 => unknown,
        0x71 => KeyCode::F15,
        0x72 => KeyCode::Insert,
        0x73 => KeyCode::Home,
        0x74 => KeyCode::PageUp,
        0x75 => KeyCode::Delete,
        0x76 => KeyCode::F4,
        0x77 => KeyCode::End,
        0x78 => KeyCode::F2,
        0x79 => KeyCode::PageDown,
        0x7a => KeyCode::F1,
        0x7b => KeyCode::ArrowLeft,
        0x7c => KeyCode::ArrowRight,
        0x7d => KeyCode::ArrowDown,
        0x7e => KeyCode::ArrowUp,
        // 0x7f =>  unknown,

        // 0xA is the caret (^) an macOS's German QERTZ layout. This key is at the same location as
        // backquote (`) on Windows' US layout.
        0xa => KeyCode::Backquote,
        _ => return None,
    })
}

pub fn key_to_native(key: &str) -> Cow<str> {
    use cocoa::appkit::*;
    const BACKSPACE_KEY: u16 = 0x7f;
    const SPACE_KEY: u16 = b' ' as u16;
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
        "f13" => NSF13FunctionKey,
        "f14" => NSF14FunctionKey,
        "f15" => NSF15FunctionKey,
        "f16" => NSF16FunctionKey,
        "f17" => NSF17FunctionKey,
        "f18" => NSF18FunctionKey,
        "f19" => NSF19FunctionKey,
        "f20" => NSF20FunctionKey,
        "f21" => NSF21FunctionKey,
        "f22" => NSF22FunctionKey,
        "f23" => NSF23FunctionKey,
        "f24" => NSF24FunctionKey,
        "f25" => NSF25FunctionKey,
        "f26" => NSF26FunctionKey,
        "f27" => NSF27FunctionKey,
        "f28" => NSF28FunctionKey,
        "f29" => NSF29FunctionKey,
        "f30" => NSF30FunctionKey,
        "f31" => NSF31FunctionKey,
        "f32" => NSF32FunctionKey,
        "f33" => NSF33FunctionKey,
        "f34" => NSF34FunctionKey,
        "f35" => NSF35FunctionKey,
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

    let mut chars = native_event.characters().to_str().to_string();

    let mut modifiers = read_modifiers(native_event);
    modifiers.function &= !chars.chars().next().map_or(false, |ch| {
        matches!(ch as u16, NSUpArrowFunctionKey..=NSModeSwitchFunctionKey)
    });

    let code = scancode_to_physicalkey(scancode);

    Keystroke {
        modifiers,
        code,
        ime_inputs: Default::default(),
    }
}
