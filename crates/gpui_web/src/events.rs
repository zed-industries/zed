use std::{collections::HashMap, rc::Rc};

use gpui::{
    Capslock, ClipboardEntry, ClipboardItem, ClipboardString, DispatchEventResult, GestureTuning,
    Image, ImageFormat, KeyDownEvent, KeyUpEvent, Keystroke, Modifiers, ModifiersChangedEvent,
    MouseButton, MouseDownEvent, MouseExitEvent, MouseMoveEvent, MouseUpEvent, NavigationDirection,
    Pixels, PlatformInput, Point, ScrollDelta, ScrollWheelEvent, TouchEvent, TouchId, TouchPhase,
    point, px,
};
use wasm_bindgen::prelude::*;

use crate::ime_mirror::ImeMirror;
use crate::window::WebWindowInner;

pub struct WebEventListeners {
    _handles: Vec<EventListenerHandle>,
}

/// A DOM event listener that is removed from its target when dropped.
///
/// Dropping the `Closure` alone would leave the listener attached to the DOM
/// pointing at a freed function; the next event would then throw "closure
/// invoked after being dropped". Keeping the target alongside the closure
/// lets `Drop` unregister the listener first.
pub(crate) struct EventListenerHandle {
    target: web_sys::EventTarget,
    event_name: &'static str,
    closure: Closure<dyn FnMut(JsValue)>,
}

impl EventListenerHandle {
    pub(crate) fn add(
        target: &web_sys::EventTarget,
        event_name: &'static str,
        handler: impl FnMut(JsValue) + 'static,
    ) -> Self {
        let closure = Closure::<dyn FnMut(JsValue)>::new(handler);
        target
            .add_event_listener_with_callback(event_name, closure.as_ref().unchecked_ref())
            .ok();
        Self {
            target: target.clone(),
            event_name,
            closure,
        }
    }

    /// Registers with `{passive: false}` so that `preventDefault()` works.
    /// Needed for events like `wheel` which are passive by default in modern
    /// browsers. Removal does not need to match the `passive` option, so
    /// `Drop` works the same as for [`EventListenerHandle::add`].
    fn add_non_passive(
        target: &web_sys::EventTarget,
        event_name: &'static str,
        handler: impl FnMut(JsValue) + 'static,
    ) -> Self {
        let closure = Closure::<dyn FnMut(JsValue)>::new(handler);
        let target_js: &JsValue = target.as_ref();
        let callback_js: &JsValue = closure.as_ref();
        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"passive".into(), &false.into()).ok();
        if let Ok(add_fn_val) = js_sys::Reflect::get(target_js, &"addEventListener".into()) {
            if let Ok(add_fn) = add_fn_val.dyn_into::<js_sys::Function>() {
                add_fn
                    .call3(target_js, &event_name.into(), callback_js, &options)
                    .ok();
            }
        }
        Self {
            target: target.clone(),
            event_name,
            closure,
        }
    }
}

impl Drop for EventListenerHandle {
    fn drop(&mut self) {
        self.target
            .remove_event_listener_with_callback(
                self.event_name,
                self.closure.as_ref().unchecked_ref(),
            )
            .ok();
    }
}

pub(crate) struct ClickState {
    last_position: Point<Pixels>,
    last_time: f64,
    current_count: usize,
}

#[derive(Default)]
pub(crate) struct TouchIds {
    next: u64,
    active: HashMap<i32, TouchId>,
}

impl TouchIds {
    fn start(&mut self, pointer_id: i32) -> Option<TouchId> {
        let next = self.next.checked_add(1)?;
        let touch_id = TouchId(self.next);
        self.next = next;
        self.active.insert(pointer_id, touch_id);
        Some(touch_id)
    }

    fn active(&self, pointer_id: i32) -> Option<TouchId> {
        self.active.get(&pointer_id).copied()
    }

    fn end(&mut self, pointer_id: i32) -> Option<TouchId> {
        self.active.remove(&pointer_id)
    }
}

impl Default for ClickState {
    fn default() -> Self {
        Self {
            last_position: Point::default(),
            last_time: 0.0,
            current_count: 0,
        }
    }
}

impl ClickState {
    fn register_click(&mut self, position: Point<Pixels>, time: f64) -> usize {
        let distance = ((f32::from(position.x) - f32::from(self.last_position.x)).powi(2)
            + (f32::from(position.y) - f32::from(self.last_position.y)).powi(2))
        .sqrt();

        if (time - self.last_time) < 400.0 && distance < 5.0 {
            self.current_count += 1;
        } else {
            self.current_count = 1;
        }

        self.last_position = position;
        self.last_time = time;
        self.current_count
    }
}

impl WebWindowInner {
    pub fn register_event_listeners(self: &Rc<Self>) -> WebEventListeners {
        let mut handles = vec![
            self.register_pointer_down(),
            self.register_pointer_up(),
            self.register_pointer_cancel(),
            self.register_touch_end(),
            self.register_pointer_move(),
            self.register_pointer_leave(),
            self.register_wheel(),
            self.register_context_menu(),
            self.register_dragover(),
            self.register_drop(),
            self.register_key_down(),
            self.register_key_up(),
            self.register_before_input(),
            self.register_input(),
            self.register_paste(),
            self.register_composition_start(),
            self.register_composition_update(),
            self.register_composition_end(),
            self.register_focus(),
            self.register_blur(),
            self.register_pointer_enter(),
        ];
        handles.extend(self.register_selection_change());
        handles.extend(self.register_visibility_change());
        handles.extend(self.register_appearance_change());
        handles.extend(self.register_fullscreen_change());

        WebEventListeners { _handles: handles }
    }

    fn listen(
        self: &Rc<Self>,
        event_name: &'static str,
        handler: impl FnMut(JsValue) + 'static,
    ) -> EventListenerHandle {
        EventListenerHandle::add(self.canvas.as_ref(), event_name, handler)
    }

    fn listen_input(
        self: &Rc<Self>,
        event_name: &'static str,
        handler: impl FnMut(JsValue) + 'static,
    ) -> EventListenerHandle {
        EventListenerHandle::add(self.ime_mirror.event_target(), event_name, handler)
    }

    fn listen_non_passive(
        self: &Rc<Self>,
        event_name: &'static str,
        handler: impl FnMut(JsValue) + 'static,
    ) -> EventListenerHandle {
        EventListenerHandle::add_non_passive(self.canvas.as_ref(), event_name, handler)
    }

    fn dispatch_input(&self, input: PlatformInput) -> Option<DispatchEventResult> {
        self.with_callback(|callbacks| &mut callbacks.input, |callback| callback(input))
    }

    /// Records the latest modifier state and reports whether it changed, so
    /// that `ModifiersChanged` is only dispatched on actual transitions
    /// rather than for every key event.
    fn update_modifiers(&self, modifiers: Modifiers, capslock: Capslock) -> bool {
        let mut current_state = self.state.borrow_mut();
        let changed = current_state.modifiers != modifiers || current_state.capslock != capslock;
        current_state.modifiers = modifiers;
        current_state.capslock = capslock;
        changed
    }

    fn register_pointer_down(self: &Rc<Self>) -> EventListenerHandle {
        let this = Rc::clone(self);
        self.listen("pointerdown", move |event: JsValue| {
            let event: web_sys::PointerEvent = event.unchecked_into();
            event.prevent_default();

            let pointer_type = event.pointer_type();
            let position = pointer_position_in_element(&event);
            this.gesture_start_visual_viewport_height
                .set(this.visual_viewport_height());

            // Capture the pointer so drags that leave the canvas keep
            // delivering pointermove/pointerup here; otherwise a release
            // outside the canvas is never seen and `pressed_button` stays
            // stuck. The capture is released implicitly on pointerup.
            this.canvas.set_pointer_capture(event.pointer_id()).ok();

            if pointer_type == "touch" {
                let Some(touch_id) = this.touch_ids.borrow_mut().start(event.pointer_id()) else {
                    log::error!("exhausted touch identifiers");
                    return;
                };
                this.state.borrow_mut().mouse_position = position;
                if this.touch_tap_candidate.get().is_none() {
                    this.touch_tap_candidate
                        .set(Some((event.pointer_id(), position)));
                }
                this.dispatch_input(PlatformInput::Touch(TouchEvent {
                    id: touch_id,
                    phase: TouchPhase::Started,
                    position,
                    predicted_position: None,
                    force: None,
                }));
                // Keyboard and IME focus intentionally do not change here:
                // whether this touch is a tap or a pan is only known at
                // release, and only a tap may affect them (see
                // `touch_tap_candidate`). The release handler still runs
                // within a user gesture, as keyboard summoning requires.
                return;
            }

            let button = dom_mouse_button_to_gpui(event.button());
            let modifiers = modifiers_from_mouse_event(&event, this.is_mac);
            let time = js_sys::Date::now();

            this.pressed_button.set(Some(button));
            let click_count = this.click_state.borrow_mut().register_click(position, time);

            {
                let mut current_state = this.state.borrow_mut();
                current_state.mouse_position = position;
                current_state.modifiers = modifiers;
            }

            this.dispatch_input(PlatformInput::MouseDown(MouseDownEvent {
                button,
                position,
                modifiers,
                click_count,
                first_mouse: false,
            }));

            this.ime_mirror.focus();
        })
    }

    fn pointer_targets_text_input(&self, position: Point<Pixels>) -> bool {
        self.with_input_handler(|handler| {
            handler.query_accepts_text_input()
                && handler
                    .element_bounds()
                    .is_some_and(|bounds| bounds.contains(&position))
        })
        .unwrap_or(false)
    }

    fn focused_input_accepts_text(&self) -> bool {
        self.with_input_handler(|handler| handler.query_accepts_text_input())
            .unwrap_or(false)
    }

    fn register_pointer_up(self: &Rc<Self>) -> EventListenerHandle {
        let this = Rc::clone(self);
        self.listen("pointerup", move |event: JsValue| {
            let event: web_sys::PointerEvent = event.unchecked_into();
            event.prevent_default();

            let position = pointer_position_in_element(&event);

            if event.pointer_type() == "touch" {
                let Some(touch_id) = this.touch_ids.borrow_mut().end(event.pointer_id()) else {
                    return;
                };
                this.state.borrow_mut().mouse_position = position;
                let completes_tap = match this.touch_tap_candidate.get() {
                    Some((pointer_id, _)) if pointer_id == event.pointer_id() => {
                        this.touch_tap_candidate.set(None);
                        true
                    }
                    _ => false,
                };
                let focused_input_accepted_text_before_tap = this.focused_input_accepts_text();
                // A recognized tap is dispatched synchronously inside this
                // call, so the text-input check below sees the state the tap
                // produced.
                let dispatch_result = this.dispatch_input(PlatformInput::Touch(TouchEvent {
                    id: touch_id,
                    phase: TouchPhase::Ended,
                    position,
                    predicted_position: None,
                    force: None,
                }));

                // A keyboard opening or closing mid-gesture reflows the
                // layout, so the release position no longer refers to the
                // content the user aimed at (a tap that summoned the keyboard
                // often ends up below the shrunken layout, which would
                // immediately dismiss it again). Skip the sync then, and for
                // anything that wasn't a tap: pans and flings must not move
                // keyboard or IME focus at all.
                let viewport_stable = this.gesture_start_visual_viewport_height.get()
                    == this.visual_viewport_height();
                if completes_tap && viewport_stable {
                    let preserve_focused_input = should_preserve_focused_input(
                        focused_input_accepted_text_before_tap,
                        this.focused_input_accepts_text(),
                        dispatch_result,
                    );
                    if !preserve_focused_input {
                        this.sync_virtual_keyboard(this.pointer_targets_text_input(position));
                    }
                }
                this.schedule_ime_mirror_sync();
                return;
            }

            let button = dom_mouse_button_to_gpui(event.button());
            let modifiers = modifiers_from_mouse_event(&event, this.is_mac);

            this.pressed_button.set(None);
            let click_count = this.click_state.borrow().current_count;

            {
                let mut current_state = this.state.borrow_mut();
                current_state.mouse_position = position;
                current_state.modifiers = modifiers;
            }

            this.dispatch_input(PlatformInput::MouseUp(MouseUpEvent {
                button,
                position,
                modifiers,
                click_count,
            }));

            this.schedule_ime_mirror_sync();
        })
    }

    /// The visual viewport's current height in layout pixels, or zero when
    /// the API is unavailable.
    fn visual_viewport_height(&self) -> f64 {
        self.browser_window
            .visual_viewport()
            .map_or(0.0, |viewport| viewport.height() * viewport.scale())
    }

    /// Whether the software keyboard is likely hidden — a heuristic, since
    /// no cross-browser keyboard-visibility signal exists. It infers from
    /// the visual viewport: a shown keyboard shrinks its height well below
    /// the greatest height seen at the current width (the width only changes
    /// on rotation, which restarts the calibration). `window.innerHeight`
    /// can't serve as the reference because Android shrinks it along with
    /// the keyboard. Unknown states err toward "visible" so ordinary
    /// editable taps don't gratuitously restart the IME session.
    ///
    /// Restricted to coarse-pointer environments: elsewhere (desktop
    /// browsers, including touchscreen laptops) viewport height tracks
    /// user window resizes rather than a software keyboard, so the
    /// calibration would misfire. Split-screen resizes on mobile can still
    /// fool it; tracking `visualViewport` resize events around focus
    /// transitions would be sturdier.
    fn keyboard_likely_dismissed(&self) -> bool {
        let coarse_pointer = self
            .browser_window
            .match_media("(pointer: coarse)")
            .ok()
            .flatten()
            .is_some_and(|media_query_list| media_query_list.matches());
        if !coarse_pointer {
            return false;
        }
        let Some(viewport) = self.browser_window.visual_viewport() else {
            return false;
        };
        let width = viewport.width() * viewport.scale();
        let height = viewport.height() * viewport.scale();
        let (probe_width, probe_height) = self.visual_viewport_probe.get();
        let max_height = if width == probe_width {
            probe_height.max(height)
        } else {
            height
        };
        self.visual_viewport_probe.set((width, max_height));
        height >= max_height * 0.85
    }

    /// The browser or OS took over the pointer (native scrolling, a system
    /// gesture, the pointer being removed): no pointerup will follow, so the
    /// gesture must unwind rather than complete.
    fn register_pointer_cancel(self: &Rc<Self>) -> EventListenerHandle {
        let this = Rc::clone(self);
        self.listen("pointercancel", move |event: JsValue| {
            let event: web_sys::PointerEvent = event.unchecked_into();
            if event.pointer_type() == "touch" {
                let Some(touch_id) = this.touch_ids.borrow_mut().end(event.pointer_id()) else {
                    return;
                };
                if let Some((pointer_id, _)) = this.touch_tap_candidate.get()
                    && pointer_id == event.pointer_id()
                {
                    this.touch_tap_candidate.set(None);
                }
                this.dispatch_input(PlatformInput::Touch(TouchEvent {
                    id: touch_id,
                    phase: TouchPhase::Cancelled,
                    position: pointer_position_in_element(&event),
                    predicted_position: None,
                    force: None,
                }));
            } else {
                this.pressed_button.set(None);
            }
        })
    }

    /// Cancels touch default handling separately because iOS does not consistently
    /// transfer pointer-event cancellation to the corresponding touch event.
    fn register_touch_end(self: &Rc<Self>) -> EventListenerHandle {
        self.listen_non_passive("touchend", move |event: JsValue| {
            let event: web_sys::Event = event.unchecked_into();
            event.prevent_default();
        })
    }

    /// See [`ImeMirror::schedule_sync`].
    fn schedule_ime_mirror_sync(self: &Rc<Self>) {
        ImeMirror::schedule_sync(self);
    }

    /// Aligns the software keyboard with the text input targeted by a touch tap.
    ///
    /// Mobile browsers show the keyboard only when an editable element is
    /// focused from within a user gesture, so this runs while the tap's
    /// `pointerup` is still on the stack (by which point GPUI has usually
    /// painted a frame since the `MouseDown`, so the input handler reflects
    /// the tap's focus change). `readOnly` suppresses the keyboard while
    /// keeping the hidden input available to the IME. Leaving it blurred after
    /// a non-editable tap lets the next editable tap establish a new input
    /// session instead of relying on a same-task blur/focus cycle, which iOS
    /// may coalesce.
    ///
    /// We don't use `navigator.virtualKeyboard` here because it's
    /// Chromium-only.
    pub(crate) fn sync_virtual_keyboard(self: &Rc<Self>, editable: bool) {
        let was_editable = !self.ime_mirror.read_only();
        self.ime_mirror.set_read_only(!editable);
        // Trigger a focus event only when the keyboard actually needs
        // summoning. Cycling focus on every tap would restart the IME
        // connection right as the keyboard reads the tapped caret's context,
        // racing its word segmentation. But `focus()` on an already-focused
        // element is a no-op, so a dismissed keyboard would otherwise never
        // return for taps that stay within editable content: detect that
        // through the visual viewport and force a fresh focus event.
        let editable_needs_focus_event = editable
            && (!was_editable || !self.ime_mirror.is_focused() || self.keyboard_likely_dismissed());
        if editable_needs_focus_event || (!editable && was_editable) {
            self.suppress_focus_status_events.set(true);
            if editable {
                // A same-task blur/focus cycle may be coalesced by iOS, but
                // this branch only runs when the keyboard is already gone,
                // so a coalesced cycle loses nothing.
                if self.ime_mirror.is_focused() {
                    self.ime_mirror.blur();
                }
                self.ime_mirror.focus();
            } else {
                self.ime_mirror.blur();
            }
            self.suppress_focus_status_events.set(false);

            if editable {
                let callback = wasm_bindgen::closure::Closure::once_into_js({
                    let this = Rc::clone(self);
                    move || {
                        this.state.borrow_mut().is_active = true;
                        this.with_callback(
                            |callbacks| &mut callbacks.active_status_change,
                            |callback| callback(true),
                        );
                    }
                });
                if let Err(error) = self
                    .browser_window
                    .set_timeout_with_callback(callback.unchecked_ref())
                {
                    log::warn!("failed to defer web window activation: {error:?}");
                }
            }
        }
    }

    /// Dispatches a full key press for editing intents that arrive without a
    /// usable key event (Android IMEs send `key: "Unidentified"` placeholders
    /// and express backspace/enter through `beforeinput` instead), so they
    /// run through the same keybinding path as hardware keys.
    fn dispatch_synthetic_keystroke(&self, key: &str, modifiers: Modifiers) {
        let keystroke = Keystroke {
            modifiers,
            key: key.to_string(),
            key_char: None,
        };
        self.dispatch_input(PlatformInput::KeyDown(KeyDownEvent {
            keystroke: keystroke.clone(),
            is_held: false,
            prefer_character_input: false,
        }));
        self.dispatch_input(PlatformInput::KeyUp(KeyUpEvent { keystroke }));
    }

    fn register_pointer_move(self: &Rc<Self>) -> EventListenerHandle {
        let this = Rc::clone(self);
        self.listen("pointermove", move |event: JsValue| {
            let event: web_sys::PointerEvent = event.unchecked_into();
            event.prevent_default();

            let position = pointer_position_in_element(&event);

            if event.pointer_type() == "touch" {
                let Some(touch_id) = this.touch_ids.borrow().active(event.pointer_id()) else {
                    return;
                };
                this.state.borrow_mut().mouse_position = position;
                // Mirror the slop rule of gpui's tap recognizer: once the
                // touch travels beyond it, its release must not affect the
                // keyboard. Only gpui knows what the gesture truly resolved
                // to; this platform-side shadow exists because the keyboard
                // decision must be made synchronously inside the browser's
                // pointerup handler.
                if let Some((pointer_id, start_position)) = this.touch_tap_candidate.get()
                    && pointer_id == event.pointer_id()
                    && (position - start_position).magnitude()
                        > f64::from(GestureTuning::default().touch_slop)
                {
                    this.touch_tap_candidate.set(None);
                }
                this.dispatch_input(PlatformInput::Touch(TouchEvent {
                    id: touch_id,
                    phase: TouchPhase::Moved,
                    position,
                    predicted_position: predicted_pointer_position(&event, position),
                    force: None,
                }));
                return;
            }

            let modifiers = modifiers_from_mouse_event(&event, this.is_mac);
            let current_pressed = this.pressed_button.get();

            {
                let mut current_state = this.state.borrow_mut();
                current_state.mouse_position = position;
                current_state.modifiers = modifiers;
            }

            this.dispatch_input(PlatformInput::MouseMove(MouseMoveEvent {
                position,
                pressed_button: current_pressed,
                modifiers,
            }));
        })
    }

    fn register_pointer_leave(self: &Rc<Self>) -> EventListenerHandle {
        let this = Rc::clone(self);
        self.listen("pointerleave", move |event: JsValue| {
            let event: web_sys::PointerEvent = event.unchecked_into();

            let position = pointer_position_in_element(&event);
            let modifiers = modifiers_from_mouse_event(&event, this.is_mac);
            let current_pressed = this.pressed_button.get();

            {
                let mut current_state = this.state.borrow_mut();
                current_state.mouse_position = position;
                current_state.modifiers = modifiers;
                current_state.is_hovered = false;
            }

            this.dispatch_input(PlatformInput::MouseExited(MouseExitEvent {
                position,
                pressed_button: current_pressed,
                modifiers,
            }));

            this.with_callback(
                |callbacks| &mut callbacks.hover_status_change,
                |callback| callback(false),
            );
        })
    }

    fn register_wheel(self: &Rc<Self>) -> EventListenerHandle {
        let this = Rc::clone(self);
        self.listen_non_passive("wheel", move |event: JsValue| {
            let event: web_sys::WheelEvent = event.unchecked_into();
            event.prevent_default();

            let mouse_event: &web_sys::MouseEvent = event.as_ref();
            let position = mouse_position_in_element(mouse_event);
            let modifiers = modifiers_from_wheel_event(mouse_event, this.is_mac);

            let delta_mode = event.delta_mode();
            let delta = if delta_mode == 1 {
                ScrollDelta::Lines(point(-event.delta_x() as f32, -event.delta_y() as f32))
            } else {
                ScrollDelta::Pixels(point(
                    px(-event.delta_x() as f32),
                    px(-event.delta_y() as f32),
                ))
            };

            {
                let mut current_state = this.state.borrow_mut();
                current_state.modifiers = modifiers;
            }

            this.dispatch_input(PlatformInput::ScrollWheel(ScrollWheelEvent {
                position,
                delta,
                modifiers,
                touch_phase: TouchPhase::Moved,
            }));
        })
    }

    fn register_context_menu(self: &Rc<Self>) -> EventListenerHandle {
        self.listen("contextmenu", move |event: JsValue| {
            let event: web_sys::Event = event.unchecked_into();
            event.prevent_default();
        })
    }

    /// Browsers only expose dropped files as `File` objects, never as
    /// filesystem paths, so no `FileDrop` input can be synthesized: GPUI's
    /// `ExternalPaths` consumers would try to read paths that don't exist.
    /// The events are still intercepted so the browser doesn't navigate to
    /// the dropped file. Delivering actual file drops would require plumbing
    /// `File` contents through a web-specific channel.
    fn register_dragover(self: &Rc<Self>) -> EventListenerHandle {
        self.listen("dragover", move |event: JsValue| {
            let event: web_sys::DragEvent = event.unchecked_into();
            event.prevent_default();
        })
    }

    fn register_drop(self: &Rc<Self>) -> EventListenerHandle {
        self.listen("drop", move |event: JsValue| {
            let event: web_sys::DragEvent = event.unchecked_into();
            event.prevent_default();
        })
    }

    fn register_key_down(self: &Rc<Self>) -> EventListenerHandle {
        let this = Rc::clone(self);
        self.listen_input("keydown", move |event: JsValue| {
            let event: web_sys::KeyboardEvent = event.unchecked_into();

            let modifiers = modifiers_from_keyboard_event(&event, this.is_mac);
            let capslock = capslock_from_keyboard_event(&event);

            if this.update_modifiers(modifiers, capslock) {
                this.dispatch_input(PlatformInput::ModifiersChanged(ModifiersChangedEvent {
                    modifiers,
                    capslock,
                }));
            }

            let key = dom_key_to_gpui_key(&event);

            if is_modifier_only_key(&key) {
                return;
            }

            let is_held = event.repeat();
            let key_char = compute_key_char(&event, &key, &modifiers);

            let keystroke = Keystroke {
                modifiers,
                key,
                key_char: key_char.clone(),
            };

            let result = this.dispatch_input(PlatformInput::KeyDown(KeyDownEvent {
                keystroke,
                is_held,
                prefer_character_input: false,
            }));

            if let Some(result) = result {
                if !result.propagate {
                    event.prevent_default();
                    this.schedule_ime_mirror_sync();
                    return;
                }
            }

            if this.is_composing.get() || event.is_composing() {
                event.prevent_default();
                return;
            }

            if keystroke_inserts_text(&modifiers, this.is_mac)
                && let Some(text) = key_char
            {
                this.with_input_handler(|handler| {
                    handler.replace_text_in_range(None, &text);
                });
                // The character went into the input handler; suppress browser
                // side-effects for the same keystroke (space scrolling the
                // page, quick-find, etc.). Everything not handled above falls
                // through so browser shortcuts keep their defaults.
                event.prevent_default();
            }
            this.schedule_ime_mirror_sync();
        })
    }

    fn register_key_up(self: &Rc<Self>) -> EventListenerHandle {
        let this = Rc::clone(self);
        self.listen_input("keyup", move |event: JsValue| {
            let event: web_sys::KeyboardEvent = event.unchecked_into();

            let modifiers = modifiers_from_keyboard_event(&event, this.is_mac);
            let capslock = capslock_from_keyboard_event(&event);

            if this.update_modifiers(modifiers, capslock) {
                this.dispatch_input(PlatformInput::ModifiersChanged(ModifiersChangedEvent {
                    modifiers,
                    capslock,
                }));
            }

            let key = dom_key_to_gpui_key(&event);

            if is_modifier_only_key(&key) {
                return;
            }

            let key_char = compute_key_char(&event, &key, &modifiers);

            let keystroke = Keystroke {
                modifiers,
                key,
                key_char,
            };

            let result = this.dispatch_input(PlatformInput::KeyUp(KeyUpEvent { keystroke }));
            if let Some(result) = result {
                if !result.propagate {
                    event.prevent_default();
                }
            }
        })
    }

    /// Imports IME edits from the hidden input into the app.
    ///
    /// Text-editing `beforeinput` events are deliberately left uncancelled,
    /// so the browser applies them to the mirror element exactly as the IME
    /// expects (cancelling them and echoing the edit back programmatically
    /// restarts the IME connection on every keystroke, which desynchronizes
    /// the keyboard's internal state — e.g. Gboard then swallows backspaces
    /// against a stale private buffer). The resulting `input` event is
    /// diffed against the last known mirror text; IME edits are contiguous,
    /// so a common prefix/suffix diff recovers them exactly.
    ///
    /// The diff supplies only the *shape* of the edit — how many UTF-16
    /// units were removed before/after the element's pre-edit selection and
    /// what text replaced them. It never supplies document coordinates:
    /// mirror offsets captured at sync time go stale whenever the document
    /// changes underneath (this is a live collaborative document). The
    /// position comes from `selected_text_range()` queried in this same
    /// synchronous callback — the editor resolves its selection through
    /// anchors, so the freshly-fetched offsets are exact, and nothing can
    /// run between the query and the edit below.
    fn register_input(self: &Rc<Self>) -> EventListenerHandle {
        let this = Rc::clone(self);
        self.listen_input("input", move |event: JsValue| {
            let event: web_sys::InputEvent = event.unchecked_into();

            // Composition text is delivered through the composition events;
            // the mirror is reconciled once on compositionend.
            if this.is_composing.get() || event.is_composing() {
                return;
            }

            let new_value = this.ime_mirror.value();
            let old_value = this.ime_mirror.stored_text();
            if new_value == old_value {
                return;
            }

            let old_units: Vec<u16> = old_value.encode_utf16().collect();
            let new_units: Vec<u16> = new_value.encode_utf16().collect();

            // A prefix/suffix diff is ambiguous when the inserted text
            // shares characters with what follows it (inserting "pactor "
            // before "pact" also reads as inserting "or pact" four units
            // later). The edit's true position is not ambiguous: the browser
            // leaves the caret at the end of an IME edit, so the suffix is
            // anchored as "everything after the post-edit caret", and the
            // prefix is capped to fit. Greedy matching is only a fallback
            // for edits where the anchored suffix doesn't verify.
            let post_edit_caret = this
                .ime_mirror
                .selection_start()
                .map(|caret| caret as usize);
            let anchored_suffix_length = post_edit_caret
                .map(|caret| new_units.len().saturating_sub(caret))
                .filter(|&suffix_length| {
                    suffix_length <= old_units.len()
                        && old_units[old_units.len() - suffix_length..]
                            == new_units[new_units.len() - suffix_length..]
                });
            let suffix_length = anchored_suffix_length.unwrap_or_else(|| {
                old_units
                    .iter()
                    .rev()
                    .zip(new_units.iter().rev())
                    .take_while(|(old_unit, new_unit)| old_unit == new_unit)
                    .count()
            });
            let prefix_length = old_units
                .iter()
                .zip(&new_units)
                .take_while(|(old_unit, new_unit)| old_unit == new_unit)
                .count()
                .min(old_units.len() - suffix_length)
                .min(new_units.len() - suffix_length);

            let inserted_text = String::from_utf16_lossy(
                &new_units[prefix_length..new_units.len() - suffix_length],
            );
            let replaced_old_end = old_units.len() - suffix_length;

            // The edit's shape relative to the element's pre-edit selection.
            // The element is private to the IME and these syncs, so the
            // stored selection is exact.
            let (element_selection_start, element_selection_end) =
                this.ime_mirror.stored_selection();
            let removed_before_selection =
                (element_selection_start as usize).saturating_sub(prefix_length);
            let removed_after_selection =
                replaced_old_end.saturating_sub(element_selection_end as usize);

            let applied = this.with_input_handler(|handler| {
                let Some(selection) = handler.selected_text_range(false) else {
                    return false;
                };
                let range = selection
                    .range
                    .start
                    .saturating_sub(removed_before_selection)
                    ..selection.range.end + removed_after_selection;
                handler.replace_text_in_range(Some(range), &inserted_text);
                true
            });
            if applied != Some(true) {
                return;
            }

            this.ime_mirror.adopt_element_state();
        })
    }

    /// Imports IME-driven selection moves on the mirror element into the app.
    ///
    /// Some IME gestures preview their effect by moving the field's
    /// selection before committing an edit — Android's slide-on-backspace
    /// grows a selection over the text it will delete. A native field
    /// renders that selection itself; this import gives the app the same
    /// chance. Like edit imports, the move is expressed relative to the
    /// element's stored selection and applied to the app selection queried
    /// in the same synchronous callback, never through document coordinates,
    /// which go stale in a collaborative document.
    ///
    /// Self-inflicted events are filtered by state, not by suppression
    /// flags: `selectionchange` dispatches asynchronously, after the sync or
    /// import that caused it has already adopted the element's selection, so
    /// a stored-state match means there is nothing to import.
    ///
    /// Registered on the document: Chrome dispatches text-control selection
    /// changes there, not on the element.
    fn register_selection_change(self: &Rc<Self>) -> Option<EventListenerHandle> {
        let document = self.browser_window.document()?;
        let this = Rc::clone(self);
        Some(EventListenerHandle::add(
            document.as_ref(),
            "selectionchange",
            move |_event: JsValue| {
                if this.is_composing.get() || !this.ime_mirror.is_focused() {
                    return;
                }
                // An in-flight edit owns the selection; its import adopts it.
                if this.ime_mirror.value() != this.ime_mirror.stored_text() {
                    return;
                }
                let Some(element_start) = this.ime_mirror.selection_start() else {
                    return;
                };
                let element_end = this
                    .ime_mirror
                    .element_selection_end()
                    .unwrap_or(element_start);
                let (stored_start, stored_end) = this.ime_mirror.stored_selection();
                if (element_start, element_end) == (stored_start, stored_end) {
                    return;
                }
                let applied = this.with_input_handler(|handler| {
                    let Some(selection) = handler.selected_text_range(false) else {
                        return false;
                    };
                    // The app range corresponding to the stored element
                    // selection is exactly `selection`; a single consistent
                    // alignment between the two maps the moved endpoints.
                    // Disagreeing alignments mean the app selection changed
                    // underneath and the pending resync owns the element.
                    let alignment = selection.range.start.checked_sub(stored_start as usize);
                    if alignment.is_none()
                        || alignment != selection.range.end.checked_sub(stored_end as usize)
                    {
                        return false;
                    }
                    let alignment = alignment.unwrap();
                    handler.set_selected_text_range(
                        alignment + element_start as usize..alignment + element_end as usize,
                    );
                    true
                });
                if applied == Some(true) {
                    this.ime_mirror.adopt_element_state();
                } else {
                    // No import is coming for this move; without a forced
                    // sync the mirror would keep deferring to it and show
                    // the IME a selection the app never adopted.
                    this.ime_mirror.reject_selection_import();
                    this.schedule_ime_mirror_sync();
                }
            },
        ))
    }

    /// Software keyboards (IMEs) express editing through `beforeinput`
    /// rather than key events: Android IMEs emit only a placeholder key
    /// event (`key: "Unidentified"`, `keyCode` 229). This handler only
    /// intercepts the intents that must not mutate the mirror element;
    /// ordinary edits deliberately proceed to the element and are imported
    /// by `register_input`. Desktop keystrokes never reach this handler,
    /// because `register_key_down` calls `preventDefault()` for every
    /// keystroke it inserts, which cancels the corresponding `beforeinput`.
    fn register_before_input(self: &Rc<Self>) -> EventListenerHandle {
        let this = Rc::clone(self);
        self.listen_input("beforeinput", move |event: JsValue| {
            let event: web_sys::InputEvent = event.unchecked_into();

            // During composition the composition{update,end} handlers own
            // the text.
            if this.is_composing.get() || event.is_composing() {
                return;
            }

            match event.input_type().as_str() {
                // Enter means "submit", not "insert a newline into the
                // mirror": run it through the keybinding path instead of
                // letting it mutate the element.
                "insertLineBreak" | "insertParagraph" => {
                    event.prevent_default();
                    this.dispatch_synthetic_keystroke("enter", Modifiers::default());
                    this.schedule_ime_mirror_sync();
                }
                // Everything else (insertText, deleteContent*, autocorrect's
                // insertReplacementText, ...) is left to the browser's
                // default action on the mirror element; `register_input`
                // imports the resulting element diff into the editor.
                _ => {}
            }
        })
    }

    /// Paste is delivered through the DOM `paste` event rather than
    /// `Platform::read_from_clipboard`: the browser's asynchronous clipboard
    /// read API cannot fit that synchronous signature, while `ClipboardEvent`
    /// exposes `clipboardData` synchronously inside the event. It fires for
    /// any browser-initiated paste (keyboard, menu bar, context menu).
    ///
    /// Text-only pastes reach the input handler synchronously. Pasted image
    /// files only expose their bytes through asynchronous blob reads, so
    /// pastes containing images are delivered once those reads resolve — to
    /// whichever input handler is focused at that point, matching how an
    /// application-level paste action would behave.
    fn register_paste(self: &Rc<Self>) -> EventListenerHandle {
        let this = Rc::clone(self);
        self.listen_input("paste", move |event: JsValue| {
            let event: web_sys::ClipboardEvent = event.unchecked_into();
            let Some(clipboard_data) = event.clipboard_data() else {
                return;
            };
            let text = clipboard_data
                .get_data("text/plain")
                .ok()
                .filter(|text| !text.is_empty());

            // File handles must be collected synchronously: the browser
            // clears `clipboardData`'s item list once this handler returns,
            // while the `File`s themselves stay readable afterwards.
            let mut image_files = Vec::new();
            let items = clipboard_data.items();
            for index in 0..items.length() {
                let Some(item) = items.get(index) else {
                    continue;
                };
                if item.kind() != "file" {
                    continue;
                }
                let Some(format) = ImageFormat::from_mime_type(&item.type_()) else {
                    continue;
                };
                if let Ok(Some(file)) = item.get_as_file() {
                    image_files.push((format, file));
                }
            }

            if text.is_none() && image_files.is_empty() {
                return;
            }
            event.prevent_default();

            if image_files.is_empty() {
                if let Some(text) = text {
                    this.with_input_handler(|handler| {
                        handler.paste(ClipboardItem::new_string(text));
                    });
                }
                return;
            }

            let this = Rc::clone(&this);
            wasm_bindgen_futures::spawn_local(async move {
                let mut entries = Vec::new();
                if let Some(text) = text {
                    entries.push(ClipboardEntry::String(ClipboardString::new(text)));
                }
                for (format, file) in image_files {
                    match crate::platform::read_blob_bytes(&file).await {
                        Ok(bytes) => {
                            entries.push(ClipboardEntry::Image(Image::from_bytes(format, bytes)));
                        }
                        Err(error) => {
                            log::error!(
                                "failed to read pasted image: {}",
                                crate::platform::js_error_message(&error)
                            );
                        }
                    }
                }
                if entries.is_empty() {
                    return;
                }
                this.with_input_handler(|handler| {
                    handler.paste(ClipboardItem { entries });
                });
            });
        })
    }

    fn register_composition_start(self: &Rc<Self>) -> EventListenerHandle {
        let this = Rc::clone(self);
        self.listen_input("compositionstart", move |_event: JsValue| {
            this.is_composing.set(true);
        })
    }

    fn register_composition_update(self: &Rc<Self>) -> EventListenerHandle {
        let this = Rc::clone(self);
        self.listen_input("compositionupdate", move |event: JsValue| {
            let event: web_sys::CompositionEvent = event.unchecked_into();
            let data = event.data().unwrap_or_default();
            this.is_composing.set(true);
            this.with_input_handler(|handler| {
                handler.replace_and_mark_text_in_range(None, &data, None);
            });
        })
    }

    fn register_composition_end(self: &Rc<Self>) -> EventListenerHandle {
        let this = Rc::clone(self);
        self.listen_input("compositionend", move |event: JsValue| {
            let event: web_sys::CompositionEvent = event.unchecked_into();
            let data = event.data().unwrap_or_default();
            this.is_composing.set(false);
            this.with_input_handler(|handler| {
                // Only commit the final text when a marked range still
                // exists. When a caret move ended the composition, the
                // editor has already unmarked (keeping the composed text as
                // committed content); inserting `data` at the selection
                // would duplicate the word at the new caret position.
                if handler.marked_text_range().is_some() {
                    handler.replace_text_in_range(None, &data);
                }
                handler.unmark_text();
            });
            // Adopt the element's post-composition state as the mirror
            // baseline without writing anything: the browser applied the
            // commit to the element itself, and a write here would restart
            // the IME mid-commit. The deferred sync reconciles any
            // app-side divergence afterwards.
            this.ime_mirror.adopt_element_state();
            this.schedule_ime_mirror_sync();
        })
    }

    fn register_focus(self: &Rc<Self>) -> EventListenerHandle {
        let this = Rc::clone(self);
        self.listen_input("focus", move |_event: JsValue| {
            if this.suppress_focus_status_events.get() {
                return;
            }
            {
                let mut state = this.state.borrow_mut();
                state.is_active = true;
            }
            this.with_callback(
                |callbacks| &mut callbacks.active_status_change,
                |callback| callback(true),
            );
        })
    }

    fn register_blur(self: &Rc<Self>) -> EventListenerHandle {
        let this = Rc::clone(self);
        self.listen_input("blur", move |_event: JsValue| {
            if this.suppress_focus_status_events.get() {
                return;
            }
            {
                let mut state = this.state.borrow_mut();
                state.is_active = false;
            }
            this.with_callback(
                |callbacks| &mut callbacks.active_status_change,
                |callback| callback(false),
            );
        })
    }

    fn register_pointer_enter(self: &Rc<Self>) -> EventListenerHandle {
        let this = Rc::clone(self);
        self.listen("pointerenter", move |_event: JsValue| {
            {
                let mut state = this.state.borrow_mut();
                state.is_hovered = true;
            }
            this.with_callback(
                |callbacks| &mut callbacks.hover_status_change,
                |callback| callback(true),
            );
        })
    }
}

fn dom_key_to_gpui_key(event: &web_sys::KeyboardEvent) -> String {
    let key = event.key();
    match key.as_str() {
        "Enter" => "enter".to_string(),
        "Backspace" => "backspace".to_string(),
        "Tab" => "tab".to_string(),
        "Escape" => "escape".to_string(),
        "Delete" => "delete".to_string(),
        " " => "space".to_string(),
        "ArrowLeft" => "left".to_string(),
        "ArrowRight" => "right".to_string(),
        "ArrowUp" => "up".to_string(),
        "ArrowDown" => "down".to_string(),
        "Home" => "home".to_string(),
        "End" => "end".to_string(),
        "PageUp" => "pageup".to_string(),
        "PageDown" => "pagedown".to_string(),
        "Insert" => "insert".to_string(),
        "Control" => "control".to_string(),
        "Alt" => "alt".to_string(),
        "Shift" => "shift".to_string(),
        "Meta" => "platform".to_string(),
        "CapsLock" => "capslock".to_string(),
        other => {
            if let Some(rest) = other.strip_prefix('F') {
                if let Ok(number) = rest.parse::<u8>() {
                    if (1..=35).contains(&number) {
                        return format!("f{number}");
                    }
                }
            }
            other.to_lowercase()
        }
    }
}

fn dom_mouse_button_to_gpui(button: i16) -> MouseButton {
    match button {
        0 => MouseButton::Left,
        1 => MouseButton::Middle,
        2 => MouseButton::Right,
        3 => MouseButton::Navigate(NavigationDirection::Back),
        4 => MouseButton::Navigate(NavigationDirection::Forward),
        _ => MouseButton::Left,
    }
}

fn modifiers_from_keyboard_event(event: &web_sys::KeyboardEvent, _is_mac: bool) -> Modifiers {
    Modifiers {
        control: event.ctrl_key(),
        alt: event.alt_key(),
        shift: event.shift_key(),
        platform: event.meta_key(),
        function: false,
    }
}

fn modifiers_from_mouse_event(event: &web_sys::PointerEvent, _is_mac: bool) -> Modifiers {
    let mouse_event: &web_sys::MouseEvent = event.as_ref();
    Modifiers {
        control: mouse_event.ctrl_key(),
        alt: mouse_event.alt_key(),
        shift: mouse_event.shift_key(),
        platform: mouse_event.meta_key(),
        function: false,
    }
}

fn modifiers_from_wheel_event(event: &web_sys::MouseEvent, _is_mac: bool) -> Modifiers {
    Modifiers {
        control: event.ctrl_key(),
        alt: event.alt_key(),
        shift: event.shift_key(),
        platform: event.meta_key(),
        function: false,
    }
}

fn capslock_from_keyboard_event(event: &web_sys::KeyboardEvent) -> Capslock {
    Capslock {
        on: event.get_modifier_state("CapsLock"),
    }
}

fn should_preserve_focused_input(
    accepted_text_before_tap: bool,
    accepts_text_after_tap: bool,
    dispatch_result: Option<DispatchEventResult>,
) -> bool {
    accepted_text_before_tap
        && accepts_text_after_tap
        && dispatch_result.is_some_and(|result| result.default_prevented)
}

pub(crate) fn is_mac_platform(browser_window: &web_sys::Window) -> bool {
    let navigator = browser_window.navigator();

    #[allow(deprecated)]
    // navigator.platform() is deprecated but navigator.userAgentData is not widely available yet
    if let Ok(platform) = navigator.platform() {
        if platform.contains("Mac") {
            return true;
        }
    }

    if let Ok(user_agent) = navigator.user_agent() {
        return user_agent.contains("Mac");
    }

    false
}

fn is_modifier_only_key(key: &str) -> bool {
    matches!(
        key,
        "control" | "alt" | "shift" | "platform" | "capslock" | "compose" | "process"
    )
}

/// Whether a keystroke with these modifiers produces text to insert.
///
/// On macOS, Option participates in text entry (e.g. option-n composes "~"
/// or accented characters), so only Command and Control disqualify. Elsewhere,
/// plain Alt is a shortcut modifier, but AltGr is reported by browsers as
/// control+alt and `event.key()` then carries the composed character.
fn keystroke_inserts_text(modifiers: &Modifiers, is_mac: bool) -> bool {
    if is_mac {
        !modifiers.platform && !modifiers.control
    } else {
        modifiers.is_subset_of(&Modifiers::shift()) || (modifiers.control && modifiers.alt)
    }
}

fn compute_key_char(
    event: &web_sys::KeyboardEvent,
    gpui_key: &str,
    modifiers: &Modifiers,
) -> Option<String> {
    // AltGr arrives as control+alt with the composed character in
    // `event.key()`; bare Command/Control combinations are not text.
    if (modifiers.platform || modifiers.control) && !(modifiers.control && modifiers.alt) {
        return None;
    }

    if is_modifier_only_key(gpui_key) {
        return None;
    }

    if gpui_key == "space" {
        return Some(" ".to_string());
    }

    let raw_key = event.key();

    if raw_key.len() == 1 {
        return Some(raw_key);
    }

    None
}

fn pointer_position_in_element(event: &web_sys::PointerEvent) -> Point<Pixels> {
    let mouse_event: &web_sys::MouseEvent = event.as_ref();
    mouse_position_in_element(mouse_event)
}

/// How far ahead of the raw pointer position predictions may reach.
///
/// Browsers predict much further (Chrome offers samples out to 25ms), but
/// prediction error grows with the horizon and surfaces as jitter: the
/// emitted pan deltas gain a term proportional to lead x change in velocity,
/// which at long leads visibly reverses direction mid-drag. Measurements
/// show leads up to ~10ms track at or below the raw stream's frame-to-frame
/// variation; AOSP similarly caps touch resampling extrapolation at 8ms
/// (`RESAMPLE_MAX_PREDICTION` in `InputTransport.cpp`).
const MAX_PREDICTION_LEAD_MS: f64 = 10.;

/// The predicted pointer position closest to [`MAX_PREDICTION_LEAD_MS`]
/// ahead of `event`, from `getPredictedEvents()`, or `None` when the browser
/// offers no prediction (Safari lacks the method, Firefox returns an empty
/// array). A prediction further out than the cap is linearly scaled back to
/// it.
///
/// Accessed through `Reflect` because calling a missing method through the
/// web-sys binding would throw, and predicted events' `offsetX`/`offsetY`
/// are unreliable across browsers (their target may be detached), so the
/// position is derived from the client-coordinate delta against the parent
/// event, anchored to the parent's element-relative `position`.
fn predicted_pointer_position(
    event: &web_sys::PointerEvent,
    position: Point<Pixels>,
) -> Option<Point<Pixels>> {
    let method = js_sys::Reflect::get(event, &JsValue::from_str("getPredictedEvents")).ok()?;
    let method = method.dyn_ref::<js_sys::Function>()?;
    let predicted_events: js_sys::Array = method.call0(event).ok()?.dyn_into().ok()?;
    let mut best: Option<(f64, web_sys::PointerEvent)> = None;
    for predicted in predicted_events.iter() {
        let Ok(predicted) = predicted.dyn_into::<web_sys::PointerEvent>() else {
            continue;
        };
        let lead = predicted.time_stamp() - event.time_stamp();
        if lead <= 0. {
            continue;
        }
        let distance_to_cap = (lead - MAX_PREDICTION_LEAD_MS).abs();
        if best
            .as_ref()
            .is_none_or(|(best_distance, _)| distance_to_cap < *best_distance)
        {
            best = Some((distance_to_cap, predicted));
        }
    }
    let (_, predicted) = best?;
    let lead = predicted.time_stamp() - event.time_stamp();
    let scale = (MAX_PREDICTION_LEAD_MS / lead).min(1.) as f32;
    let event: &web_sys::MouseEvent = event.as_ref();
    let predicted: &web_sys::MouseEvent = predicted.as_ref();
    Some(point(
        position.x + px((predicted.client_x() - event.client_x()) as f32 * scale),
        position.y + px((predicted.client_y() - event.client_y()) as f32 * scale),
    ))
}

fn mouse_position_in_element(event: &web_sys::MouseEvent) -> Point<Pixels> {
    // offset_x/offset_y give position relative to the target element's padding edge
    point(px(event.offset_x() as f32), px(event.offset_y() as f32))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn browser_pointer_id_reuse_gets_a_new_touch_id() {
        let mut touch_ids = TouchIds::default();
        let first = touch_ids.start(7).expect("first touch id");
        let concurrent = touch_ids.start(8).expect("concurrent touch id");

        assert_ne!(first, concurrent);
        assert_eq!(touch_ids.active(7), Some(first));
        assert_eq!(touch_ids.end(7), Some(first));
        assert_eq!(touch_ids.active(7), None);

        let reused = touch_ids.start(7).expect("reused pointer touch id");
        assert_ne!(reused, first);
        assert_ne!(reused, concurrent);
    }

    #[test]
    fn handled_tap_preserves_unchanged_text_input() {
        assert!(should_preserve_focused_input(
            true,
            true,
            Some(DispatchEventResult {
                propagate: false,
                default_prevented: true,
            }),
        ));
    }

    #[test]
    fn tap_does_not_preserve_unhandled_or_unfocused_input() {
        let result = |default_prevented| {
            Some(DispatchEventResult {
                propagate: false,
                default_prevented,
            })
        };

        assert!(!should_preserve_focused_input(true, true, result(false),));
        assert!(!should_preserve_focused_input(false, true, result(true),));
        assert!(!should_preserve_focused_input(true, false, result(true),));
    }
}
