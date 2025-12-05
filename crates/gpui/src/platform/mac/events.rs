use crate::{
    Capslock, KeyDownEvent, KeyUpEvent, Keystroke, Modifiers, ModifiersChangedEvent, MouseButton,
    MouseDownEvent, MouseExitEvent, MouseMoveEvent, MouseUpEvent, NavigationDirection, Pixels,
    PlatformInput, ScrollDelta, ScrollWheelEvent, TouchPhase,
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
use objc::{msg_send, sel, sel_impl};
use std::{borrow::Cow, ffi::c_void};

const BACKSPACE_KEY: u16 = 0x7f;
const SPACE_KEY: u16 = b' ' as u16;
const ENTER_KEY: u16 = 0x0d;
const NUMPAD_ENTER_KEY: u16 = 0x03;
pub(crate) const ESCAPE_KEY: u16 = 0x1b;
const TAB_KEY: u16 = 0x09;
const SHIFT_TAB_KEY: u16 = 0x19;

pub fn key_to_native(key: &str) -> Cow<'_, str> {
    use cocoa::appkit::*;
    let code = match key {
        "space" => SPACE_KEY,
        "backspace" => BACKSPACE_KEY,
        "escape" => ESCAPE_KEY,
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
                        capslock: Capslock {
                            on: native_event
                                .modifierFlags()
                                .contains(NSEventModifierFlags::NSAlphaShiftKeyMask),
                        },
                    }))
                }
                NSEventType::NSKeyDown => Some(Self::KeyDown(KeyDownEvent {
                    keystroke: parse_keystroke(native_event),
                    is_held: native_event.isARepeat() == YES,
                    prefer_character_input: false,
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

        let mut characters = native_event
            .charactersIgnoringModifiers()
            .to_str()
            .to_string();
        let mut key_char = None;
        let first_char = characters.chars().next().map(|ch| ch as u16);
        let modifiers = native_event.modifierFlags();

        let control = modifiers.contains(NSEventModifierFlags::NSControlKeyMask);
        let alt = modifiers.contains(NSEventModifierFlags::NSAlternateKeyMask);
        let mut shift = modifiers.contains(NSEventModifierFlags::NSShiftKeyMask);
        let command = modifiers.contains(NSEventModifierFlags::NSCommandKeyMask);
        let function = modifiers.contains(NSEventModifierFlags::NSFunctionKeyMask)
            && first_char
                .is_none_or(|ch| !(NSUpArrowFunctionKey..=NSModeSwitchFunctionKey).contains(&ch));

        #[allow(non_upper_case_globals)]
        let key = match first_char {
            Some(SPACE_KEY) => {
                key_char = Some(" ".to_string());
                "space".to_string()
            }
            Some(TAB_KEY) => {
                key_char = Some("\t".to_string());
                "tab".to_string()
            }
            Some(ENTER_KEY) | Some(NUMPAD_ENTER_KEY) => {
                key_char = Some("\n".to_string());
                "enter".to_string()
            }
            Some(BACKSPACE_KEY) => "backspace".to_string(),
            Some(ESCAPE_KEY) => "escape".to_string(),
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
            Some(NSF20FunctionKey) => "f20".to_string(),
            Some(NSF21FunctionKey) => "f21".to_string(),
            Some(NSF22FunctionKey) => "f22".to_string(),
            Some(NSF23FunctionKey) => "f23".to_string(),
            Some(NSF24FunctionKey) => "f24".to_string(),
            Some(NSF25FunctionKey) => "f25".to_string(),
            Some(NSF26FunctionKey) => "f26".to_string(),
            Some(NSF27FunctionKey) => "f27".to_string(),
            Some(NSF28FunctionKey) => "f28".to_string(),
            Some(NSF29FunctionKey) => "f29".to_string(),
            Some(NSF30FunctionKey) => "f30".to_string(),
            Some(NSF31FunctionKey) => "f31".to_string(),
            Some(NSF32FunctionKey) => "f32".to_string(),
            Some(NSF33FunctionKey) => "f33".to_string(),
            Some(NSF34FunctionKey) => "f34".to_string(),
            Some(NSF35FunctionKey) => "f35".to_string(),
            _ => {
                // Cases to test when modifying this:
                //
                //           qwerty key | none | cmd   | cmd-shift
                // * Armenian         s | ս    | cmd-s | cmd-shift-s  (layout is non-ASCII, so we use cmd layout)
                // * Dvorak+QWERTY    s | o    | cmd-s | cmd-shift-s  (layout switches on cmd)
                // * Ukrainian+QWERTY s | с    | cmd-s | cmd-shift-s  (macOS reports cmd-s instead of cmd-S)
                // * Czech            7 | ý    | cmd-ý | cmd-7        (layout has shifted numbers)
                // * Norwegian        7 | 7    | cmd-7 | cmd-/        (macOS reports cmd-shift-7 instead of cmd-/)
                // * Russian          7 | 7    | cmd-7 | cmd-&        (shift-7 is . but when cmd is down, should use cmd layout)
                // * German QWERTZ    ; | ö    | cmd-ö | cmd-Ö        (Zed's shift special case only applies to a-z)
                //
                let mut chars_ignoring_modifiers =
                    chars_for_modified_key(native_event.keyCode(), NO_MOD);
                let mut chars_with_shift =
                    chars_for_modified_key(native_event.keyCode(), SHIFT_MOD);
                let always_use_cmd_layout = always_use_command_layout();

                // Handle Dvorak+QWERTY / Russian / Armenian
                if command || always_use_cmd_layout {
                    let chars_with_cmd = chars_for_modified_key(native_event.keyCode(), CMD_MOD);
                    let chars_with_both =
                        chars_for_modified_key(native_event.keyCode(), CMD_MOD | SHIFT_MOD);

                    // We don't do this in the case that the shifted command key generates
                    // the same character as the unshifted command key (Norwegian, e.g.)
                    if chars_with_both != chars_with_cmd {
                        chars_with_shift = chars_with_both;

                    // Handle edge-case where cmd-shift-s reports cmd-s instead of
                    // cmd-shift-s (Ukrainian, etc.)
                    } else if chars_with_cmd.to_ascii_uppercase() != chars_with_cmd {
                        chars_with_shift = chars_with_cmd.to_ascii_uppercase();
                    }
                    chars_ignoring_modifiers = chars_with_cmd;
                }

                if !control && !command && !function {
                    let mut mods = NO_MOD;
                    if shift {
                        mods |= SHIFT_MOD;
                    }
                    if alt {
                        mods |= OPTION_MOD;
                    }

                    key_char = Some(chars_for_modified_key(native_event.keyCode(), mods));
                }

                if shift
                    && chars_ignoring_modifiers
                        .chars()
                        .all(|c| c.is_ascii_lowercase())
                {
                    chars_ignoring_modifiers
                } else if shift {
                    shift = false;
                    chars_with_shift
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
            key_char,
        }
    }
}

fn always_use_command_layout() -> bool {
    if chars_for_modified_key(0, NO_MOD).is_ascii() {
        return false;
    }

    chars_for_modified_key(0, CMD_MOD).is_ascii()
}

const NO_MOD: u32 = 0;
const CMD_MOD: u32 = 1;
const SHIFT_MOD: u32 = 2;
const OPTION_MOD: u32 = 8;

fn chars_for_modified_key(code: CGKeyCode, modifiers: u32) -> String {
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
