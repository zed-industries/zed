use crate::window::AndroidWindowInner;
use android_activity::input::{
    InputEvent, KeyAction, KeyCharacterMap, KeyMapChar, Keycode, MetaState, MotionAction, Pointer,
};
use android_activity::{AndroidApp, InputStatus};
use gpui::{
    KeyDownEvent, KeyUpEvent, Keystroke, Modifiers, ModifiersChangedEvent, PlatformInput,
    TouchEvent, TouchId, TouchPhase, point, px,
};
use std::collections::HashMap;

/// Maps Android pointer identifiers, which the system reuses as soon as a
/// finger lifts, to [`TouchId`]s that stay unique for the window's lifetime.
#[derive(Default)]
pub(crate) struct TouchIds {
    next: u64,
    active: HashMap<i32, TouchId>,
}

impl TouchIds {
    fn start(&mut self, pointer_id: i32) -> TouchId {
        let touch_id = TouchId(self.next);
        self.next += 1;
        self.active.insert(pointer_id, touch_id);
        touch_id
    }
}

pub(crate) fn handle_input_event(
    event: &InputEvent<'_>,
    window: &AndroidWindowInner,
    touch_ids: &mut TouchIds,
    key_maps: &mut HashMap<i32, KeyCharacterMap>,
    app: &AndroidApp,
) -> InputStatus {
    match event {
        InputEvent::MotionEvent(motion_event) => {
            let scale = window.state.borrow().scale_factor;
            let position =
                |pointer: &Pointer<'_>| point(px(pointer.x() / scale), px(pointer.y() / scale));
            let dispatch = |id, phase, position| {
                window.state.borrow_mut().mouse_position = position;
                window.dispatch_input(PlatformInput::Touch(TouchEvent {
                    id,
                    phase,
                    position,
                    predicted_position: None,
                    force: None,
                }))
            };
            let acting_pointer = motion_event.pointer_at_index(motion_event.pointer_index());

            match motion_event.action() {
                MotionAction::Down | MotionAction::PointerDown => {
                    let id = touch_ids.start(acting_pointer.pointer_id());
                    dispatch(id, TouchPhase::Started, position(&acting_pointer));
                }
                MotionAction::Move => {
                    for pointer in motion_event.pointers() {
                        if let Some(&id) = touch_ids.active.get(&pointer.pointer_id()) {
                            dispatch(id, TouchPhase::Moved, position(&pointer));
                        }
                    }
                }
                MotionAction::Up | MotionAction::PointerUp => {
                    let Some(id) = touch_ids.active.remove(&acting_pointer.pointer_id()) else {
                        return InputStatus::Unhandled;
                    };
                    let position = position(&acting_pointer);
                    let result = dispatch(id, TouchPhase::Ended, position);
                    // A recognized tap is dispatched synchronously inside the
                    // call above. A consumed tap may have navigated to an
                    // editor under this coordinate, which is not a request to
                    // type; controls that intend typing request the keyboard.
                    let targets_text_input = result.is_some_and(|result| !result.default_prevented)
                        && window
                            .with_input_handler(|handler| {
                                handler.query_accepts_text_input()
                                    && handler
                                        .element_bounds()
                                        .is_some_and(|bounds| bounds.contains(&position))
                            })
                            .unwrap_or(false);
                    if targets_text_input {
                        app.show_soft_input(true);
                    }
                }
                MotionAction::Cancel => {
                    for pointer in motion_event.pointers() {
                        if let Some(id) = touch_ids.active.remove(&pointer.pointer_id()) {
                            dispatch(id, TouchPhase::Cancelled, position(&pointer));
                        }
                    }
                }
                _ => return InputStatus::Unhandled,
            }
            InputStatus::Handled
        }
        InputEvent::KeyEvent(key_event) => {
            let keycode = key_event.key_code();
            let Some(key) = keycode_to_key(keycode) else {
                return InputStatus::Unhandled;
            };
            let meta_state = key_event.meta_state();
            let modifiers = modifiers_from_meta_state(meta_state);

            {
                let mut state = window.state.borrow_mut();
                state.modifiers = modifiers;
                state.capslock = gpui::Capslock {
                    on: meta_state.caps_lock_on(),
                };
            }
            window.dispatch_input(PlatformInput::ModifiersChanged(ModifiersChangedEvent {
                modifiers,
                capslock: gpui::Capslock {
                    on: meta_state.caps_lock_on(),
                },
            }));

            if key.is_empty() {
                return InputStatus::Handled;
            }

            let key_char = key_char_for(key_event.device_id(), keycode, meta_state, key_maps, app);
            let keystroke = Keystroke {
                modifiers,
                key: key.to_owned(),
                key_char: key_char.clone(),
            };

            match key_event.action() {
                KeyAction::Down => {
                    let result = window.dispatch_input(PlatformInput::KeyDown(KeyDownEvent {
                        keystroke,
                        is_held: false,
                        prefer_character_input: false,
                    }));

                    let propagate = result.is_none_or(|result| result.propagate);
                    if propagate
                        && modifiers.is_subset_of(&Modifiers::shift())
                        && let Some(text) = key_char
                    {
                        window.with_input_handler(|handler| {
                            handler.replace_text_in_range(None, &text);
                        });
                    }
                    InputStatus::Handled
                }
                KeyAction::Up => {
                    window.dispatch_input(PlatformInput::KeyUp(KeyUpEvent { keystroke }));
                    InputStatus::Handled
                }
                _ => InputStatus::Unhandled,
            }
        }
        _ => InputStatus::Unhandled,
    }
}

fn key_char_for(
    device_id: i32,
    keycode: Keycode,
    meta_state: MetaState,
    key_maps: &mut HashMap<i32, KeyCharacterMap>,
    app: &AndroidApp,
) -> Option<String> {
    const VIRTUAL_KEYBOARD_DEVICE_ID: i32 = -1;

    if !key_maps.contains_key(&device_id) {
        // Some devices (e.g. the emulator's forwarded host keyboard, id 0)
        // have no per-device map; the built-in virtual keyboard map always
        // exists. Cache whichever we got under the original id.
        let map = app.device_key_character_map(device_id).or_else(|error| {
            log::info!(
                "no key character map for device {device_id} ({error:?}); \
                 falling back to the virtual keyboard map"
            );
            app.device_key_character_map(VIRTUAL_KEYBOARD_DEVICE_ID)
        });
        match map {
            Ok(map) => {
                key_maps.insert(device_id, map);
            }
            Err(error) => {
                log::warn!("failed to load key character map for device {device_id}: {error:?}");
                return None;
            }
        }
    }
    let map = key_maps.get(&device_id)?;
    match map.get(keycode, meta_state) {
        Ok(KeyMapChar::Unicode(character)) => Some(character.to_string()),
        Ok(_) => None,
        Err(error) => {
            log::warn!("KeyCharacterMap.get failed: {error:?}");
            None
        }
    }
}

fn modifiers_from_meta_state(meta_state: MetaState) -> Modifiers {
    Modifiers {
        control: meta_state.ctrl_on(),
        alt: meta_state.alt_on(),
        shift: meta_state.shift_on(),
        platform: meta_state.meta_on(),
        function: meta_state.function_on(),
    }
}

/// Maps an Android keycode to GPUI's key names (see `Keystroke::parse`).
/// Returns `Some("")` for modifier keys (handled via ModifiersChanged) and
/// `None` for keys we don't handle so the OS can apply default behavior
/// (volume, back, etc.).
fn keycode_to_key(keycode: Keycode) -> Option<&'static str> {
    use Keycode::*;
    Some(match keycode {
        A => "a",
        B => "b",
        C => "c",
        D => "d",
        E => "e",
        F => "f",
        G => "g",
        H => "h",
        I => "i",
        J => "j",
        K => "k",
        L => "l",
        M => "m",
        N => "n",
        O => "o",
        P => "p",
        Q => "q",
        R => "r",
        S => "s",
        T => "t",
        U => "u",
        V => "v",
        W => "w",
        X => "x",
        Y => "y",
        Z => "z",
        Keycode0 => "0",
        Keycode1 => "1",
        Keycode2 => "2",
        Keycode3 => "3",
        Keycode4 => "4",
        Keycode5 => "5",
        Keycode6 => "6",
        Keycode7 => "7",
        Keycode8 => "8",
        Keycode9 => "9",
        Space => "space",
        Enter | NumpadEnter => "enter",
        Tab => "tab",
        Del => "backspace",
        ForwardDel => "delete",
        Escape => "escape",
        DpadUp => "up",
        DpadDown => "down",
        DpadLeft => "left",
        DpadRight => "right",
        PageUp => "pageup",
        PageDown => "pagedown",
        MoveHome => "home",
        MoveEnd => "end",
        Comma => ",",
        Period => ".",
        Minus => "-",
        Equals => "=",
        LeftBracket => "[",
        RightBracket => "]",
        Backslash => "\\",
        Semicolon => ";",
        Apostrophe => "'",
        Slash => "/",
        Grave => "`",
        ShiftLeft | ShiftRight | CtrlLeft | CtrlRight | AltLeft | AltRight | MetaLeft
        | MetaRight | CapsLock => "",
        _ => return None,
    })
}
