use std::rc::Rc;

use gpui::{
    Capslock, ClipboardEntry, ClipboardItem, ClipboardString, DispatchEventResult, Image,
    ImageFormat, KeyDownEvent, KeyUpEvent, Keystroke, Modifiers, ModifiersChangedEvent,
    MouseButton, MouseDownEvent, MouseExitEvent, MouseMoveEvent, MouseUpEvent, NavigationDirection,
    Pixels, PlatformInput, Point, ScrollDelta, ScrollWheelEvent, TouchPhase, point, px,
};
use wasm_bindgen::prelude::*;

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
        EventListenerHandle::add(self.input_element.as_ref(), event_name, handler)
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

            this.input_element.focus().ok();

            // Capture the pointer so drags that leave the canvas keep
            // delivering pointermove/pointerup here; otherwise a release
            // outside the canvas is never seen and `pressed_button` stays
            // stuck. The capture is released implicitly on pointerup.
            this.canvas.set_pointer_capture(event.pointer_id()).ok();

            let button = dom_mouse_button_to_gpui(event.button());
            let position = pointer_position_in_element(&event);
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
        })
    }

    fn register_pointer_up(self: &Rc<Self>) -> EventListenerHandle {
        let this = Rc::clone(self);
        self.listen("pointerup", move |event: JsValue| {
            let event: web_sys::PointerEvent = event.unchecked_into();
            event.prevent_default();

            let button = dom_mouse_button_to_gpui(event.button());
            let position = pointer_position_in_element(&event);
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

            if event.pointer_type() == "touch" {
                this.sync_virtual_keyboard();
            }
            this.schedule_ime_mirror_sync();
        })
    }

    /// Schedules a coalesced `sync_ime_mirror` for the next task.
    ///
    /// Event handlers must not write to the mirror element mid-gesture:
    /// every write (value, selection) is observed by the IME, and a
    /// sequence of writes inside one gesture desynchronizes its model of
    /// the field (every native-behaving reference — a plain textarea —
    /// performs at most one such change per gesture). Deferring to a
    /// zero-delay timeout coalesces all sync requests from one gesture into
    /// a single write that lands after the browser has finished processing
    /// the gesture's events.
    fn schedule_ime_mirror_sync(self: &Rc<Self>) {
        if self.ime_mirror_sync_scheduled.replace(true) {
            return;
        }
        let this = Rc::clone(self);
        let closure = wasm_bindgen::closure::Closure::once_into_js(move || {
            this.ime_mirror_sync_scheduled.set(false);
            this.sync_ime_mirror();
        });
        self.browser_window
            .set_timeout_with_callback(closure.unchecked_ref())
            .ok();
    }

    /// Mirrors the text surrounding the selection into the hidden input.
    ///
    /// IMEs decide what backspace, autocorrect, and suggestions mean by
    /// inspecting the editable element's value and selection: with an empty
    /// element, Gboard deletes against its private buffer (the keypress
    /// reaches the page only as an `"Unidentified"` placeholder) and its
    /// suggestion strip has no context. Mirroring a window of real text
    /// makes those operations arrive as interpretable `beforeinput` events.
    ///
    /// All offsets are UTF-16 code units on both sides: GPUI's input-handler
    /// protocol and JavaScript string indexing agree by construction.
    ///
    /// Writing to the element is a last resort: any rewrite of its value or
    /// selection makes the browser restart the IME's input connection,
    /// which resets the keyboard's state — fatal in the middle of a
    /// keyboard's multi-step edit sequence (suggestion picks arrive as
    /// delete-then-insert pairs). After `register_input` imports an edit,
    /// the element already *is* a faithful — if off-center — window of the
    /// document, so this first verifies the element against the document at
    /// its current alignment and skips every write while that holds. The
    /// window is rebuilt only when the app changed independently (caret
    /// moved by tap or keybinding, remote edit inside the window) or the
    /// selection drifted too close to the window's edge to give the IME
    /// context.
    fn sync_ime_mirror(&self) {
        if self.is_composing.get() {
            return;
        }
        let selection = self
            .with_input_handler(|handler| handler.selected_text_range(false))
            .flatten();
        let Some(selection) = selection else {
            if !self.ime_mirror_text.borrow().is_empty() {
                self.input_element.set_value("");
                self.ime_mirror_text.borrow_mut().clear();
            }
            self.ime_mirror_selection.set((0, 0));
            return;
        };

        const IME_MIRROR_CONTEXT_CHARS: usize = 512;
        /// The element is left alone until the selection gets this close to
        /// the mirrored window's edge (unless it desynchronizes outright).
        const IME_MIRROR_MIN_EDGE_CHARS: usize = 64;

        if self.ime_mirror_is_consistent(&selection.range, IME_MIRROR_MIN_EDGE_CHARS) {
            return;
        }

        // A caret move within the existing window (a tap into nearby text)
        // must update only the element's selection, like a native tap in a
        // plain textarea. Rewriting the value restarts the IME connection,
        // which desynchronizes the keyboard's word model right when it is
        // about to act on the tapped word.
        if self.ime_mirror_move_selection_within_window(&selection.range, IME_MIRROR_MIN_EDGE_CHARS)
        {
            return;
        }

        let window_range = selection
            .range
            .start
            .saturating_sub(IME_MIRROR_CONTEXT_CHARS)
            ..selection.range.end + IME_MIRROR_CONTEXT_CHARS;
        let mut adjusted = None;
        let text = self
            .with_input_handler(|handler| {
                handler.text_for_range(window_range.clone(), &mut adjusted)
            })
            .flatten()
            .unwrap_or_default();
        let window_start = adjusted.unwrap_or(window_range).start;

        if *self.ime_mirror_text.borrow() != text || self.input_element.value() != text {
            self.input_element.set_value(&text);
            *self.ime_mirror_text.borrow_mut() = text;
        }

        self.ime_mirror_window_hint.set(window_start);
        let selection_start = selection.range.start.saturating_sub(window_start) as u32;
        let selection_end = selection.range.end.saturating_sub(window_start) as u32;
        if self.input_element.selection_start().ok().flatten() != Some(selection_start)
            || self.input_element.selection_end().ok().flatten() != Some(selection_end)
        {
            self.input_element
                .set_selection_range(selection_start, selection_end)
                .ok();
        }
        // Read the selection back rather than trusting the computed values:
        // the browser clamps out-of-bounds positions, and a stored selection
        // the element doesn't actually have would corrupt the next diff.
        let actual_start = self.input_element.selection_start().ok().flatten();
        let actual_end = self.input_element.selection_end().ok().flatten();
        self.ime_mirror_selection.set((
            actual_start.unwrap_or(selection_start),
            actual_end.unwrap_or(selection_end),
        ));
    }

    /// Attempts to represent a changed app selection as a pure element
    /// selection move within the existing mirror window.
    ///
    /// The stored window-start hint is re-verified textually against the
    /// document before use, so a stale hint (remote edit, any drift) fails
    /// verification and falls through to a full window rebuild rather than
    /// mispositioning the selection.
    fn ime_mirror_move_selection_within_window(
        &self,
        app_selection: &std::ops::Range<usize>,
        min_edge: usize,
    ) -> bool {
        let stored_text = self.ime_mirror_text.borrow().clone();
        let stored_length = stored_text.encode_utf16().count();
        if stored_length == 0 || self.input_element.value() != stored_text {
            return false;
        }
        let window_start = self.ime_mirror_window_hint.get();

        // The new selection must sit inside the window with enough context
        // on both sides — except where the window is pinned to a document
        // boundary, where less context is all the context there is. This is
        // the common case: a chat thread's caret usually sits at the end of
        // the document, where the window has no right margin at all.
        let Some(selection_start) = app_selection.start.checked_sub(window_start) else {
            return false;
        };
        let selection_end = selection_start + (app_selection.end - app_selection.start);
        if selection_end > stored_length {
            return false;
        }
        if selection_start < min_edge && window_start != 0 {
            return false;
        }

        // Verify the hint: the stored window text must still equal the
        // document at this alignment. Asking for one unit extra also
        // determines whether the window reaches the document's end, which
        // excuses a missing right margin.
        let mut adjusted = None;
        let document_text = self
            .with_input_handler(|handler| {
                handler.text_for_range(
                    window_start..window_start + stored_length + 1,
                    &mut adjusted,
                )
            })
            .flatten()
            .unwrap_or_default();
        let document_text_length = document_text.encode_utf16().count();
        let window_at_document_end = document_text_length == stored_length;
        if selection_end + min_edge > stored_length && !window_at_document_end {
            return false;
        }
        if !document_text.starts_with(stored_text.as_str())
            || document_text_length > stored_length + 1
        {
            return false;
        }

        self.input_element
            .set_selection_range(selection_start as u32, selection_end as u32)
            .ok();
        let actual_start = self.input_element.selection_start().ok().flatten();
        let actual_end = self.input_element.selection_end().ok().flatten();
        if actual_start != Some(selection_start as u32) || actual_end != Some(selection_end as u32)
        {
            return false;
        }
        self.ime_mirror_selection
            .set((selection_start as u32, selection_end as u32));
        true
    }

    /// Whether the hidden input, at its current window alignment, is still
    /// an accurate mirror of the document around the app selection with
    /// enough context on both sides. When this holds, a sync must not touch
    /// the element (see `sync_ime_mirror` on why writes are harmful).
    fn ime_mirror_is_consistent(
        &self,
        app_selection: &std::ops::Range<usize>,
        min_edge: usize,
    ) -> bool {
        let (element_selection_start, element_selection_end) = self.ime_mirror_selection.get();
        let element_selection_start = element_selection_start as usize;
        let element_selection_end = element_selection_end as usize;
        let stored_text = self.ime_mirror_text.borrow().clone();
        let stored_length = stored_text.encode_utf16().count();

        if stored_length == 0 {
            return false;
        }
        // The element's real selection must match what we believe it is.
        if self.input_element.selection_start().ok().flatten()
            != Some(element_selection_start as u32)
            || self.input_element.selection_end().ok().flatten()
                != Some(element_selection_end as u32)
        {
            return false;
        }
        // Enough context on both sides of the selection, unless the window
        // is pinned to a document boundary (start of window at document
        // offset 0, or window end at document end — approximated by the
        // stored window being shorter than requested on that side).
        let app_window_start = match app_selection.start.checked_sub(element_selection_start) {
            Some(start) => start,
            None => return false,
        };
        let has_left_context = element_selection_start >= min_edge || app_window_start == 0;
        let right_context = stored_length.saturating_sub(element_selection_end);
        if !has_left_context || right_context < min_edge {
            // A short right side is fine when the window genuinely reaches
            // the end of the document; verify by asking for one unit past
            // the stored window.
            let mut adjusted = None;
            let past_end = app_window_start + stored_length;
            let more = self
                .with_input_handler(|handler| {
                    handler.text_for_range(past_end..past_end + 1, &mut adjusted)
                })
                .flatten()
                .unwrap_or_default();
            if !has_left_context || !more.is_empty() {
                return false;
            }
        }
        // The stored window must still equal the document at this alignment
        // (a remote edit inside the window invalidates it), and the element
        // must still hold exactly the stored text.
        let mut adjusted = None;
        let document_text = self
            .with_input_handler(|handler| {
                handler.text_for_range(
                    app_window_start..app_window_start + stored_length,
                    &mut adjusted,
                )
            })
            .flatten()
            .unwrap_or_default();
        if document_text != stored_text {
            return false;
        }
        if self.input_element.value() != stored_text {
            return false;
        }
        // The element selection corresponds to the app selection end too?
        app_selection.end.checked_sub(app_window_start) == Some(element_selection_end)
    }

    /// Aligns the software keyboard with GPUI's focus after a touch tap.
    ///
    /// Mobile browsers show the keyboard only when an editable element is
    /// focused from within a user gesture, so this runs while the tap's
    /// `pointerup` is still on the stack (by which point GPUI has usually
    /// painted a frame since the `MouseDown`, so the input handler reflects
    /// the tap's focus change). `readOnly` suppresses the keyboard while
    /// keeping hardware key events flowing to the hidden input; the
    /// blur/focus cycle is what makes the browser re-evaluate keyboard
    /// visibility, since `focus()` on an already-focused element is a no-op.
    fn sync_virtual_keyboard(&self) {
        let editable = self.state.borrow().input_handler.is_some();
        let was_editable = !self.input_element.read_only();
        self.input_element.set_read_only(!editable);
        // Cycle only on an actual editability transition. Cycling on every
        // tap would restart the IME connection right as the keyboard reads
        // the tapped caret's context, racing its word segmentation.
        if editable != was_editable {
            self.suppress_focus_status_events.set(true);
            self.input_element.blur().ok();
            self.input_element.focus().ok();
            self.suppress_focus_status_events.set(false);
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

            let new_value = this.input_element.value();
            let old_value = this.ime_mirror_text.borrow().clone();
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
                .input_element
                .selection_start()
                .ok()
                .flatten()
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
            let (element_selection_start, element_selection_end) = this.ime_mirror_selection.get();
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

            *this.ime_mirror_text.borrow_mut() = new_value;
            let post_edit_selection_start = this
                .input_element
                .selection_start()
                .ok()
                .flatten()
                .unwrap_or(0);
            let post_edit_selection_end = this
                .input_element
                .selection_end()
                .ok()
                .flatten()
                .unwrap_or(post_edit_selection_start);
            this.ime_mirror_selection
                .set((post_edit_selection_start, post_edit_selection_end));
        })
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
            *this.ime_mirror_text.borrow_mut() = this.input_element.value();
            let selection_start = this
                .input_element
                .selection_start()
                .ok()
                .flatten()
                .unwrap_or(0);
            let selection_end = this
                .input_element
                .selection_end()
                .ok()
                .flatten()
                .unwrap_or(selection_start);
            this.ime_mirror_selection
                .set((selection_start, selection_end));
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

fn mouse_position_in_element(event: &web_sys::MouseEvent) -> Point<Pixels> {
    // offset_x/offset_y give position relative to the target element's padding edge
    point(px(event.offset_x() as f32), px(event.offset_y() as f32))
}
