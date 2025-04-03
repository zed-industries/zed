use crate::{
    KeyCode, KeyDownEvent, KeyPosition, KeyUpEvent, Keystroke, Modifiers,
    ModifiersChangedEvent, MouseButton, MouseDownEvent, MouseExitEvent, MouseMoveEvent,
    MouseUpEvent, NavigationDirection, Pixels, PlatformInput, ScrollDelta, ScrollWheelEvent,
    TouchPhase,
    platform::mac::{
        LMGetKbdType, NSStringExt, TISCopyCurrentKeyboardLayoutInputSource,
        TISGetInputSourceProperty, UCKeyTranslate, kTISPropertyUnicodeKeyLayoutData,
    },
    point, px,
};
use cocoa::{
    appkit::{NSEvent, NSEventModifierFlags, NSEventPhase, NSEventType},
    base::{YES, id},
};
use core_foundation::data::{CFDataGetBytePtr, CFDataRef};
use core_graphics::event::CGKeyCode;
use objc::{msg_send, runtime::Object, sel, sel_impl};
use std::{
    borrow::Cow,
    ffi::{c_void, CStr},
};

use super::kTISPropertyInputSourceID;

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
    unsafe {
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
}

impl PlatformInput {
    pub(crate) unsafe fn from_native(
        native_event: id,
        window_height: Option<Pixels>,
    ) -> Option<Self> {
        unsafe {
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
                    Some(Self::ModifiersChanged(ModifiersChangedEvent {
                        modifiers: read_modifiers(native_event),
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
                // Some mice (like Logitech MX Master) send navigation buttons as swipe events
                NSEventType::NSEventTypeSwipe => {
                    let navigation_direction = match native_event.phase() {
                        NSEventPhase::NSEventPhaseEnded => match native_event.deltaX() {
                            x if x > 0.0 => Some(NavigationDirection::Back),
                            x if x < 0.0 => Some(NavigationDirection::Forward),
                            _ => return None,
                        },
                        _ => return None,
                    };

                    match navigation_direction {
                        Some(direction) => window_height.map(|window_height| {
                            Self::MouseDown(MouseDownEvent {
                                button: MouseButton::Navigate(direction),
                                position: point(
                                    px(native_event.locationInWindow().x as f32),
                                    window_height - px(native_event.locationInWindow().y as f32),
                                ),
                                modifiers: read_modifiers(native_event),
                                click_count: 1,
                                first_mouse: false,
                            })
                        }),
                        _ => None,
                    }
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
}

unsafe fn parse_keystroke(native_event: id) -> Keystroke {
    unsafe {
        use cocoa::appkit::*;

    let scan_code = native_event.keyCode();
        let mut characters = native_event
            .charactersIgnoringModifiers()
            .to_str()
            .to_string();
        // let mut key_char = None;
        let first_char = characters.chars().next().map(|ch| ch as u16);
        let modifiers = native_event.modifierFlags();

        let control = modifiers.contains(NSEventModifierFlags::NSControlKeyMask);
        let alt = modifiers.contains(NSEventModifierFlags::NSAlternateKeyMask);
        let mut shift = modifiers.contains(NSEventModifierFlags::NSShiftKeyMask);
        let command = modifiers.contains(NSEventModifierFlags::NSCommandKeyMask);
        let function = modifiers.contains(NSEventModifierFlags::NSFunctionKeyMask)
            && first_char.map_or(true, |ch| {
                !(NSUpArrowFunctionKey..=NSModeSwitchFunctionKey).contains(&ch)
            });
    let has_key_char = !control && !command && !function;
    let key = chars_for_modified_key(scan_code, NO_MOD);

    if let Some((code, key, key_char)) = parse_immutable_keys(scan_code, has_key_char) {
        println!(
            "parse_immutable_keys: {:#?}, {:?}, {}",
            modifiers, code, key
        );
        return Keystroke {
            modifiers: Modifiers {
                control,
                alt,
                shift,
                platform: command,
                function,
            },
            code,
            face: key,
            key_char,
        };
    }

    let code = parse_other_keys(scan_code);
    let key_char = if has_key_char {
        let mut mods = NO_MOD;
        if shift {
            mods |= SHIFT_MOD;
        }
        if alt {
            mods |= OPTION_MOD;
        }
        Some(chars_for_modified_key(scan_code, mods))
    } else {
        None
    };

    println!("parse_other_keys: {:#?}, {:?}, {}", modifiers, code, key);
    let ret = Keystroke {
        modifiers: Modifiers {
            control,
            alt,
            shift,
            platform: command,
            function,
        },
        code,
        face: key,
            key_char,
        }
    };
    println!("parse_keystroke: {:#?}", ret);
    ret
}

pub fn always_use_command_layout() -> bool {
    if chars_for_modified_key(0, NO_MOD).is_ascii() {
        return false;
    }

    chars_for_modified_key(0, CMD_MOD).is_ascii()
}

const NO_MOD: u32 = 0;
const CMD_MOD: u32 = 1;
const SHIFT_MOD: u32 = 2;
const OPTION_MOD: u32 = 8;

pub fn chars_for_modified_key(code: CGKeyCode, modifiers: u32) -> String {
    // Values from: https://github.com/phracker/MacOSX-SDKs/blob/master/MacOSX10.6.sdk/System/Library/Frameworks/Carbon.framework/Versions/A/Frameworks/HIToolbox.framework/Versions/A/Headers/Events.h#L126
    // shifted >> 8 for UCKeyTranslate
    const CG_SPACE_KEY: u16 = 49;
    // https://github.com/phracker/MacOSX-SDKs/blob/master/MacOSX10.6.sdk/System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/CarbonCore.framework/Versions/A/Headers/UnicodeUtilities.h#L278
    #[allow(non_upper_case_globals)]
    const kUCKeyActionDown: u16 = 0;
    #[allow(non_upper_case_globals)]
    const kUCKeyTranslateNoDeadKeysMask: u32 = 0;

    let keyboard_type = unsafe { LMGetKbdType() as u32 };
    const BUFFER_SIZE: usize = 4;
    let mut dead_key_state = 0;
    let mut buffer: [u16; BUFFER_SIZE] = [0; BUFFER_SIZE];
    let mut buffer_size: usize = 0;

    let keyboard = unsafe { TISCopyCurrentKeyboardLayoutInputSource() };
    if keyboard.is_null() {
        return "".to_string();
    }
    let layout_data = unsafe {
        TISGetInputSourceProperty(keyboard, kTISPropertyUnicodeKeyLayoutData as *const c_void)
            as CFDataRef
    };
    if layout_data.is_null() {
        unsafe {
            let _: () = msg_send![keyboard, release];
        }
        return "".to_string();
    }
    let keyboard_layout = unsafe { CFDataGetBytePtr(layout_data) };

    unsafe {
        UCKeyTranslate(
            keyboard_layout as *const c_void,
            code,
            kUCKeyActionDown,
            modifiers,
            keyboard_type,
            kUCKeyTranslateNoDeadKeysMask,
            &mut dead_key_state,
            BUFFER_SIZE,
            &mut buffer_size as *mut usize,
            &mut buffer as *mut u16,
        );
        if dead_key_state != 0 {
            UCKeyTranslate(
                keyboard_layout as *const c_void,
                CG_SPACE_KEY,
                kUCKeyActionDown,
                modifiers,
                keyboard_type,
                kUCKeyTranslateNoDeadKeysMask,
                &mut dead_key_state,
                BUFFER_SIZE,
                &mut buffer_size as *mut usize,
                &mut buffer as *mut u16,
            );
        }
        let _: () = msg_send![keyboard, release];
    }
    String::from_utf16(&buffer[..buffer_size]).unwrap_or_default()
}

fn parse_immutable_keys(
    scan_code: u16,
    has_key_char: bool,
) -> Option<(KeyCode, String, Option<String>)> {
    let mut key_char = None;
    let (code, key) = match scan_code {
        0x033 => (KeyCode::Backspace, "backspace".to_string()),
        0x030 => {
            key_char = Some("\t".to_string());
            (KeyCode::Tab, "tab".to_string())
        }
        // Enter key and numpad enter key
        0x0024 | 0x004c => {
            key_char = Some("\n".to_string());
            (KeyCode::Enter, "enter".to_string())
        }
        0x0038 => (KeyCode::Shift(KeyPosition::Left), "shift".to_string()),
        0x003c => (KeyCode::Shift(KeyPosition::Right), "shift".to_string()),
        0x003b => (KeyCode::Control(KeyPosition::Left), "control".to_string()),
        0x003e => (KeyCode::Control(KeyPosition::Right), "control".to_string()),
        0x003a => (KeyCode::Alt(KeyPosition::Left), "alt".to_string()),
        0x003d => (KeyCode::Alt(KeyPosition::Right), "alt".to_string()),
        0x0039 => (KeyCode::Capital, "capslock".to_string()),
        0x0035 => (KeyCode::Escape, "escape".to_string()),
        0x0031 => {
            key_char = Some(" ".to_string());
            (KeyCode::Space, "space".to_string())
        }
        0x0074 => (KeyCode::PageUp, "pageup".to_string()),
        0x0079 => (KeyCode::PageDown, "pagedown".to_string()),
        0x0077 => (KeyCode::End, "end".to_string()),
        0x0073 => (KeyCode::Home, "home".to_string()),
        0x007b => (KeyCode::Left, "left".to_string()),
        0x007e => (KeyCode::Up, "up".to_string()),
        0x007c => (KeyCode::Right, "right".to_string()),
        0x007d => (KeyCode::Down, "down".to_string()),
        // PrintScreen is effectively F13 on Mac OS X.
        // 0xffff => KeyCode::PrintScreen,
        0x0072 => (KeyCode::Insert, "insert".to_string()),
        0x0075 => (KeyCode::Delete, "delete".to_string()),
        0x0037 => (KeyCode::Platform(KeyPosition::Left), "cmd".to_string()),
        0x0036 => (KeyCode::Platform(KeyPosition::Right), "cmd".to_string()),

        0x006e => (KeyCode::ContextMenu, "menu".to_string()),
        0x007a => (KeyCode::F1, "f1".to_string()),
        0x0078 => (KeyCode::F2, "f2".to_string()),
        0x0063 => (KeyCode::F3, "f3".to_string()),
        0x0076 => (KeyCode::F4, "f4".to_string()),
        0x0060 => (KeyCode::F5, "f5".to_string()),
        0x0061 => (KeyCode::F6, "f6".to_string()),
        0x0062 => (KeyCode::F7, "f7".to_string()),
        0x0064 => (KeyCode::F8, "f8".to_string()),
        0x0065 => (KeyCode::F9, "f9".to_string()),
        0x006d => (KeyCode::F10, "f10".to_string()),
        0x0067 => (KeyCode::F11, "f11".to_string()),
        0x006f => (KeyCode::F12, "f12".to_string()),
        0x0069 => (KeyCode::F13, "f13".to_string()),
        0x006b => (KeyCode::F14, "f14".to_string()),
        0x0071 => (KeyCode::F15, "f15".to_string()),
        0x006a => (KeyCode::F16, "f16".to_string()),
        0x0040 => (KeyCode::F17, "f17".to_string()),
        0x004f => (KeyCode::F18, "f18".to_string()),
        0x0050 => (KeyCode::F19, "f19".to_string()),
        0x005a => (KeyCode::F20, "f20".to_string()),
        _ => return None,
    };
    Some((code, key, if has_key_char { key_char } else { None }))
}

fn parse_other_keys(scan_code: u16) -> KeyCode {
    match scan_code {
        0x001d => KeyCode::Digital0,
        0x0012 => KeyCode::Digital1,
        0x0013 => KeyCode::Digital2,
        0x0014 => KeyCode::Digital3,
        0x0015 => KeyCode::Digital4,
        0x0017 => KeyCode::Digital5,
        0x0016 => KeyCode::Digital6,
        0x001a => KeyCode::Digital7,
        0x001c => KeyCode::Digital8,
        0x0019 => KeyCode::Digital9,
        0x0029 => KeyCode::Semicolon,
        0x0018 => KeyCode::Plus,
        0x002b => KeyCode::Comma,
        0x001b => KeyCode::Minus,
        0x002f => KeyCode::Period,
        0x002c => KeyCode::Slash,
        0x0032 => KeyCode::Tilde,
        0x0021 => KeyCode::LeftBracket,
        0x002a => KeyCode::Backslash,
        0x001e => KeyCode::RightBracket,
        0x0027 => KeyCode::Quote,
        0x0000 => KeyCode::A,
        0x000b => KeyCode::B,
        0x0008 => KeyCode::C,
        0x0002 => KeyCode::D,
        0x000e => KeyCode::E,
        0x0003 => KeyCode::F,
        0x0005 => KeyCode::G,
        0x0004 => KeyCode::H,
        0x0022 => KeyCode::I,
        0x0026 => KeyCode::J,
        0x0028 => KeyCode::K,
        0x0025 => KeyCode::L,
        0x002e => KeyCode::M,
        0x002d => KeyCode::N,
        0x001f => KeyCode::O,
        0x0023 => KeyCode::P,
        0x000c => KeyCode::Q,
        0x000f => KeyCode::R,
        0x0001 => KeyCode::S,
        0x0011 => KeyCode::T,
        0x0020 => KeyCode::U,
        0x0009 => KeyCode::V,
        0x000d => KeyCode::W,
        0x0007 => KeyCode::X,
        0x0010 => KeyCode::Y,
        0x0006 => KeyCode::Z,
        _ => KeyCode::Unknown,
    }
}

pub(crate) fn keyboard_layout() -> String {
    unsafe {
        let current_keyboard = TISCopyCurrentKeyboardLayoutInputSource();

        let input_source_id: *mut Object =
            TISGetInputSourceProperty(current_keyboard, kTISPropertyInputSourceID as *const c_void);
        let input_source_id: *const std::os::raw::c_char = msg_send![input_source_id, UTF8String];
        let input_source_id = CStr::from_ptr(input_source_id).to_str().unwrap();

        input_source_id.to_string()
    }
}
