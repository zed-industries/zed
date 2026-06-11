use crate::window::AndroidWindowInner;
use android_activity::input::{
    InputEvent, KeyAction, KeyCharacterMap, KeyMapChar, Keycode, MetaState, MotionAction,
};
use android_activity::{AndroidApp, InputStatus};
use gpui::{
    KeyDownEvent, KeyUpEvent, Keystroke, Modifiers, ModifiersChangedEvent, MouseButton,
    MouseDownEvent, MouseMoveEvent, MouseUpEvent, Pixels, PlatformInput, Point, ScrollDelta,
    ScrollWheelEvent, TouchPhase, point, px,
};
use std::collections::HashMap;
use std::time::Instant;

/// Distance (logical px) a touch may travel before it stops being a tap and
/// becomes a scroll.
const TOUCH_SLOP: f32 = 8.0;
const DOUBLE_TAP_MILLIS: u128 = 400;
const DOUBLE_TAP_DISTANCE: f32 = 16.0;

pub(crate) struct ClickState {
    last_position: Point<Pixels>,
    last_time: Option<Instant>,
    current_count: usize,
}

impl Default for ClickState {
    fn default() -> Self {
        Self {
            last_position: Point::default(),
            last_time: None,
            current_count: 0,
        }
    }
}

impl ClickState {
    fn register_click(&mut self, position: Point<Pixels>) -> usize {
        let now = Instant::now();
        let distance = ((f32::from(position.x) - f32::from(self.last_position.x)).powi(2)
            + (f32::from(position.y) - f32::from(self.last_position.y)).powi(2))
        .sqrt();

        let within_double_tap = self
            .last_time
            .is_some_and(|last| now.duration_since(last).as_millis() < DOUBLE_TAP_MILLIS);
        if within_double_tap && distance < DOUBLE_TAP_DISTANCE {
            self.current_count += 1;
        } else {
            self.current_count = 1;
        }

        self.last_position = position;
        self.last_time = Some(now);
        self.current_count
    }
}

/// Single-finger gesture recognizer. GPUI has no touch event variants, so we
/// synthesize: a tap becomes MouseDown + MouseUp, and a drag past the slop
/// becomes a ScrollWheel stream with touch phases (content follows the finger).
#[derive(Default)]
pub(crate) enum TouchGesture {
    #[default]
    None,
    Pending {
        start: Point<Pixels>,
        last: Point<Pixels>,
    },
    Scrolling {
        last: Point<Pixels>,
    },
}

pub(crate) fn handle_input_event(
    event: &InputEvent<'_>,
    window: &AndroidWindowInner,
    gesture: &mut TouchGesture,
    key_maps: &mut HashMap<i32, KeyCharacterMap>,
    app: &AndroidApp,
) -> InputStatus {
    match event {
        InputEvent::MotionEvent(motion_event) => {
            let scale = window.state.borrow().scale_factor;
            let pointer_index = motion_event.pointer_index();
            let pointer = motion_event.pointer_at_index(pointer_index);
            let position = point(px(pointer.x() / scale), px(pointer.y() / scale));
            window.state.borrow_mut().mouse_position = position;

            match motion_event.action() {
                MotionAction::Down => {
                    *gesture = TouchGesture::Pending {
                        start: position,
                        last: position,
                    };
                }
                MotionAction::Move => match gesture {
                    TouchGesture::Pending { start, last } => {
                        let moved = ((f32::from(position.x) - f32::from(start.x)).powi(2)
                            + (f32::from(position.y) - f32::from(start.y)).powi(2))
                        .sqrt();
                        if moved > TOUCH_SLOP {
                            let delta = point(position.x - last.x, position.y - last.y);
                            let anchor = *start;
                            *gesture = TouchGesture::Scrolling { last: position };
                            window.dispatch_input(PlatformInput::ScrollWheel(ScrollWheelEvent {
                                position: anchor,
                                delta: ScrollDelta::Pixels(delta),
                                modifiers: Modifiers::default(),
                                touch_phase: TouchPhase::Started,
                            }));
                        } else {
                            *last = position;
                        }
                    }
                    TouchGesture::Scrolling { last } => {
                        let delta = point(position.x - last.x, position.y - last.y);
                        *last = position;
                        window.dispatch_input(PlatformInput::ScrollWheel(ScrollWheelEvent {
                            position,
                            delta: ScrollDelta::Pixels(delta),
                            modifiers: Modifiers::default(),
                            touch_phase: TouchPhase::Moved,
                        }));
                    }
                    TouchGesture::None => {}
                },
                MotionAction::Up => match std::mem::take(gesture) {
                    TouchGesture::Pending { start, .. } => {
                        let click_count = window.click_state.borrow_mut().register_click(start);
                        window.dispatch_input(PlatformInput::MouseMove(MouseMoveEvent {
                            position: start,
                            pressed_button: None,
                            modifiers: Modifiers::default(),
                        }));
                        window.dispatch_input(PlatformInput::MouseDown(MouseDownEvent {
                            button: MouseButton::Left,
                            position: start,
                            modifiers: Modifiers::default(),
                            click_count,
                            first_mouse: false,
                        }));
                        window.dispatch_input(PlatformInput::MouseUp(MouseUpEvent {
                            button: MouseButton::Left,
                            position: start,
                            modifiers: Modifiers::default(),
                            click_count,
                        }));
                    }
                    TouchGesture::Scrolling { .. } => {
                        window.dispatch_input(PlatformInput::ScrollWheel(ScrollWheelEvent {
                            position,
                            delta: ScrollDelta::Pixels(Point::default()),
                            modifiers: Modifiers::default(),
                            touch_phase: TouchPhase::Ended,
                        }));
                    }
                    TouchGesture::None => {}
                },
                MotionAction::Cancel => {
                    if matches!(gesture, TouchGesture::Scrolling { .. }) {
                        window.dispatch_input(PlatformInput::ScrollWheel(ScrollWheelEvent {
                            position,
                            delta: ScrollDelta::Pixels(Point::default()),
                            modifiers: Modifiers::default(),
                            touch_phase: TouchPhase::Ended,
                        }));
                    }
                    *gesture = TouchGesture::None;
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
    if !key_maps.contains_key(&device_id) {
        match app.device_key_character_map(device_id) {
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
