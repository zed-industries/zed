use std::rc::Rc;

use gpui::{
    Capslock, DispatchEventResult, GestureTuning, KeyDownEvent, KeyUpEvent, Keystroke, Modifiers,
    ModifiersChangedEvent, MouseButton, MouseDownEvent, MouseExitEvent, MouseMoveEvent,
    MouseUpEvent, NavigationDirection, PinchEvent, Pixels, PlatformInput, Point, ScrollDelta,
    ScrollWheelEvent, TouchPhase, point, px,
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
    fn register_click(&mut self, position: Point<Pixels>, time: f64, tap_slop: f32) -> usize {
        let distance = distance_between(position, self.last_position);

        if (time - self.last_time) < 400.0 && distance < tap_slop {
            self.current_count += 1;
        } else {
            self.current_count = 1;
        }

        self.last_position = position;
        self.last_time = time;
        self.current_count
    }
}

/// Distance in logical pixels a mouse click may move from the previous one
/// and still accumulate a multi-click count. Touch taps use the looser
/// [`GestureTuning::multi_tap_slop`] instead.
const MOUSE_MULTI_CLICK_SLOP: f32 = 5.0;

/// How far back from the latest sample, in milliseconds, finger positions
/// are considered when computing the release velocity for fling momentum.
/// Short, because a flick accelerates right up to release: a long mean
/// reports the slower average speed and launches flings weaker than the
/// finger's actual speed at lift. Long enough to smooth sampling jitter.
const VELOCITY_WINDOW_MS: f64 = 60.0;

/// A finger that rests at least this long before lifting releases with no
/// velocity, even though older samples from before the pause are still
/// buffered.
const RELEASE_PAUSE_MS: f64 = 100.0;

/// How far ahead of the finger, in milliseconds along its smoothed velocity,
/// drag deltas are extrapolated. This compensates part of the browser's
/// input-to-display latency the way native input resampling does. A
/// continuous extrapolation from our own velocity estimate stays smooth
/// where per-event browser prediction (`getPredictedEvents`) proved
/// intermittent, oscillating between led and raw positions. Kept below the
/// pipeline latency it offsets (measured at roughly 50-65ms for a canvas
/// app at 60Hz) so direction reversals don't visibly overshoot.
const TOUCH_EXTRAPOLATION_LEAD_MS: f32 = 40.0;

/// Time constant, in milliseconds, of the exponential moving average applied
/// to the extrapolation lead vector. The raw lead is velocity times
/// [`TOUCH_EXTRAPOLATION_LEAD_MS`], which amplifies per-event velocity noise
/// into visible jitter; smoothing the lead keeps steady motion tightly led
/// while wobble in the estimate averages out.
const TOUCH_LEAD_SMOOTHING_MS: f32 = 60.0;

/// Samples older than this are pruned from a gesture's history; it only needs
/// to comfortably cover [`VELOCITY_WINDOW_MS`] before the release event.
const SAMPLE_RETENTION_MS: f64 = 250.0;

/// Single-finger touch gesture recognizer state.
///
/// Browsers surface touches as pointer events, but mapping them 1:1 to mouse
/// events would turn every finger drag into a left-button drag (text
/// selection). Instead, touch pointers are held back until the gesture is
/// disambiguated: a touch that stays within the slop and lifts becomes a
/// synthesized left click, a drag past the slop becomes a [`ScrollWheelEvent`]
/// stream that follows the finger (with fling momentum on release), and a
/// long press becomes a synthesized right click. Mouse and pen pointers keep
/// the direct mapping.
#[derive(Default)]
pub(crate) enum TouchGestureState {
    #[default]
    None,
    /// Still within the slop: may yet become a tap, long press, scroll, or
    /// pinch.
    Pending(PendingTouch),
    Scrolling(ScrollingTouch),
    /// Two fingers are down; their separation drives [`PinchEvent`]s.
    Pinching(PinchingTouch),
    /// The touch was already consumed — it stopped a momentum fling or was
    /// recognized as a long press — so its remaining events are ignored
    /// until it ends.
    Consumed {
        pointer_id: i32,
    },
}

impl TouchGestureState {
    fn involves_pointer(&self, pointer_id: i32) -> bool {
        match self {
            TouchGestureState::None => false,
            TouchGestureState::Pending(pending) => pending.pointer_id == pointer_id,
            TouchGestureState::Scrolling(scrolling) => scrolling.pointer_id == pointer_id,
            TouchGestureState::Pinching(pinching) => {
                pinching.first_pointer_id == pointer_id || pinching.second_pointer_id == pointer_id
            }
            TouchGestureState::Consumed { pointer_id: id } => *id == pointer_id,
        }
    }
}

pub(crate) struct PendingTouch {
    pointer_id: i32,
    start: Point<Pixels>,
    samples: Vec<TouchSample>,
    /// Dropping this state (tap, scroll hand-off, cancel) clears the timer,
    /// so a long press can only fire while the touch is still pending.
    _long_press_timer: Option<LongPressTimer>,
}

pub(crate) struct ScrollingTouch {
    pointer_id: i32,
    /// Scroll events are anchored at the gesture's starting position so the
    /// scroll container under the initial touch keeps receiving them even
    /// when the finger drifts outside its bounds.
    anchor: Point<Pixels>,
    /// The position the last dispatched delta ended at; leads the real
    /// finger by [`TOUCH_EXTRAPOLATION_LEAD_MS`] along its velocity.
    last: Point<Pixels>,
    /// Time of the previous move event, for the lead smoothing step.
    last_time: f64,
    /// Smoothed extrapolation lead, in logical pixels.
    lead: Point<f32>,
    /// Recent *real* (unextrapolated) finger positions, kept for velocity
    /// estimates.
    samples: Vec<TouchSample>,
}

pub(crate) struct PinchingTouch {
    first_pointer_id: i32,
    second_pointer_id: i32,
    first_position: Point<Pixels>,
    second_position: Point<Pixels>,
    /// False until the first [`PinchEvent`] is dispatched with
    /// [`TouchPhase::Started`].
    began: bool,
}

pub(crate) struct TouchSample {
    time: f64,
    position: Point<Pixels>,
}

fn push_touch_sample(samples: &mut Vec<TouchSample>, time: f64, position: Point<Pixels>) {
    samples.push(TouchSample { time, position });
    let cutoff = time - SAMPLE_RETENTION_MS;
    samples.retain(|sample| sample.time >= cutoff);
}

/// Finger velocity at release, in logical pixels per second: the mean over
/// the trailing [`VELOCITY_WINDOW_MS`] of samples. The window is referenced
/// to the latest sample rather than to `release_time`, so a `pointerup`
/// arriving a frame late doesn't shrink it; a pause before lifting is
/// detected separately via [`RELEASE_PAUSE_MS`].
fn velocity_from_samples(samples: &[TouchSample], release_time: f64) -> Point<f32> {
    let Some(latest) = samples.last() else {
        return point(0.0, 0.0);
    };
    if release_time - latest.time > RELEASE_PAUSE_MS {
        return point(0.0, 0.0);
    }
    let window_start = latest.time - VELOCITY_WINDOW_MS;
    let Some(first) = samples.iter().find(|sample| sample.time >= window_start) else {
        return point(0.0, 0.0);
    };
    let elapsed_ms = (latest.time - first.time) as f32;
    if elapsed_ms <= 0.0 {
        return point(0.0, 0.0);
    }
    point(
        (f32::from(latest.position.x) - f32::from(first.position.x)) / elapsed_ms * 1000.0,
        (f32::from(latest.position.y) - f32::from(first.position.y)) / elapsed_ms * 1000.0,
    )
}

/// A pending `setTimeout` that is cleared when dropped.
pub(crate) struct LongPressTimer {
    browser_window: web_sys::Window,
    handle: i32,
    _closure: Closure<dyn FnMut()>,
}

impl Drop for LongPressTimer {
    fn drop(&mut self) {
        self.browser_window.clear_timeout_with_handle(self.handle);
    }
}

/// A momentum scroll in flight, ticked once per render frame after a pan
/// gesture ends with sufficient velocity. The fling continues the pan's
/// scroll gesture: it emits [`TouchPhase::Moved`] deltas until the curve
/// finishes, and only then closes the gesture with [`TouchPhase::Ended`].
///
/// Motion follows a closed-form curve evaluated against the release time;
/// each tick dispatches the difference from the offset already delivered.
/// Timing noise from the millisecond-clamped clock therefore only shifts
/// *when* the curve is sampled, and never accumulates into the scrolled
/// distance.
pub(crate) struct FlingState {
    position: Point<Pixels>,
    /// Release time on the monotonic clock shared with animation frame
    /// timestamps.
    start_time: f64,
    curve: FlingCurve,
    /// Curve offset already dispatched, in logical pixels.
    dispatched_offset: Point<f32>,
}

/// Each platform's users expect their platform's deceleration feel, so the
/// fling curve is chosen per platform at release time.
enum FlingCurve {
    /// UIKit-flavored exponential velocity decay: long, gradual tails.
    /// Driven by [`GestureTuning::momentum_decay_per_ms`] and closed out
    /// below [`GestureTuning::min_fling_velocity`].
    ExponentialDecay {
        /// Release velocity in logical pixels per second.
        initial_velocity: Point<f32>,
    },
    /// Android's `OverScroller` friction feel: shorter, more decisive stops.
    /// Constants and the cubic penetration polynomial are borrowed from
    /// Flutter's `ClampingScrollSimulation` (BSD-3-Clause), which reproduces
    /// the platform scroller for canvas-rendered content.
    AndroidScroller {
        /// Unit vector along the release velocity.
        direction: Point<f32>,
        /// Total distance the fling covers, in logical pixels.
        distance: f32,
        duration_ms: f32,
    },
}

/// `ln(0.78) / ln(0.9)`, Android's deceleration rate exponent.
const ANDROID_DECELERATION_RATE: f32 = 2.358_201_8;
/// The cubic penetration polynomial's initial slope; total fling distance is
/// `velocity * duration / this`.
const ANDROID_INITIAL_VELOCITY_PENETRATION: f32 = 3.065;

/// Android fling duration in seconds for a release speed in logical pixels
/// per second, per Flutter's `ClampingScrollSimulation`: friction 0.015
/// scaled by `0.84 * 61774.04968` (a tuning of `9.8 m/s²` expressed in
/// logical pixels).
fn android_fling_duration_seconds(speed: f32) -> f32 {
    let scaled_friction = 0.015 * 0.84 * 61774.05;
    ((0.35 * speed / scaled_friction).ln() / (ANDROID_DECELERATION_RATE - 1.0)).exp()
}

/// Fraction of an Android fling's total distance covered at normalized time
/// `t` in `0..=1`: `1.2t³ - 3.27t² + 3.065t`.
fn android_fling_distance_penetration(t: f32) -> f32 {
    ((1.2 * t - 3.27) * t + ANDROID_INITIAL_VELOCITY_PENETRATION) * t
}

fn distance_between(a: Point<Pixels>, b: Point<Pixels>) -> f32 {
    ((f32::from(a.x) - f32::from(b.x)).powi(2) + (f32::from(a.y) - f32::from(b.y)).powi(2)).sqrt()
}

/// Milliseconds on the same monotonic clock as `requestAnimationFrame`
/// timestamps, so gesture times and frame times are directly comparable.
fn monotonic_time_ms(browser_window: &web_sys::Window) -> f64 {
    browser_window
        .performance()
        .map_or_else(js_sys::Date::now, |performance| performance.now())
}

fn midpoint(a: Point<Pixels>, b: Point<Pixels>) -> Point<Pixels> {
    point((a.x + b.x) / 2.0, (a.y + b.y) / 2.0)
}

impl WebWindowInner {
    pub fn register_event_listeners(self: &Rc<Self>) -> WebEventListeners {
        let mut handles = vec![
            self.register_pointer_down(),
            self.register_pointer_up(),
            self.register_pointer_move(),
            self.register_pointer_cancel(),
            self.register_pointer_leave(),
            self.register_wheel(),
            self.register_context_menu(),
            self.register_dragover(),
            self.register_drop(),
            self.register_key_down(),
            self.register_key_up(),
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

            // Any new contact takes over scrolling from a momentum fling.
            let caught_fling = this.stop_fling();

            // Capture the pointer so drags that leave the canvas keep
            // delivering pointermove/pointerup here; otherwise a release
            // outside the canvas is never seen and `pressed_button` stays
            // stuck. The capture is released implicitly on pointerup.
            this.canvas.set_pointer_capture(event.pointer_id()).ok();

            if event.pointer_type() == "touch" {
                this.handle_touch_down(&event, caught_fling);
                return;
            }

            this.input_element.focus().ok();

            let button = dom_mouse_button_to_gpui(event.button());
            let position = pointer_position_in_element(&event);
            let modifiers = modifiers_from_mouse_event(&event, this.is_mac);
            let time = monotonic_time_ms(&this.browser_window);

            this.pressed_button.set(Some(button));
            let click_count = this.click_state.borrow_mut().register_click(
                position,
                time,
                MOUSE_MULTI_CLICK_SLOP,
            );

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

            if event.pointer_type() == "touch" {
                this.handle_touch_up(&event);
                return;
            }

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
        })
    }

    fn register_pointer_move(self: &Rc<Self>) -> EventListenerHandle {
        let this = Rc::clone(self);
        self.listen("pointermove", move |event: JsValue| {
            let event: web_sys::PointerEvent = event.unchecked_into();
            event.prevent_default();

            if event.pointer_type() == "touch" {
                this.handle_touch_move(&event);
                return;
            }

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

    fn register_pointer_cancel(self: &Rc<Self>) -> EventListenerHandle {
        let this = Rc::clone(self);
        self.listen("pointercancel", move |event: JsValue| {
            let event: web_sys::PointerEvent = event.unchecked_into();
            if event.pointer_type() == "touch" {
                this.handle_touch_cancel(&event);
                return;
            }
            this.pressed_button.set(None);
        })
    }

    /// Begins gesture recognition for a touch contact. Nothing is dispatched
    /// yet: the gesture is disambiguated by later movement and timing.
    fn handle_touch_down(self: &Rc<Self>, event: &web_sys::PointerEvent, caught_fling: bool) {
        let pointer_id = event.pointer_id();
        let position = pointer_position_in_element(event);
        let time = monotonic_time_ms(&self.browser_window);

        // The scroll gesture a second finger interrupts is closed out after
        // the state transition, once the borrow is released.
        let mut interrupted_scroll_anchor = None;
        let mut started_scroll = false;
        {
            let mut gesture = self.touch_gesture.borrow_mut();
            match std::mem::take(&mut *gesture) {
                TouchGestureState::None => {
                    let mut samples = Vec::new();
                    push_touch_sample(&mut samples, time, position);

                    // A touch that catches a moving fling continues the
                    // scroll interaction, matching native scroll views: it
                    // must not click or long-press whatever happens to be
                    // under the finger, and dragging resumes scrolling
                    // immediately, without a new slop.
                    if caught_fling {
                        started_scroll = true;
                        *gesture = TouchGestureState::Scrolling(ScrollingTouch {
                            pointer_id,
                            anchor: position,
                            last: position,
                            last_time: time,
                            lead: point(0.0, 0.0),
                            samples,
                        });
                    } else {
                        *gesture = TouchGestureState::Pending(PendingTouch {
                            pointer_id,
                            start: position,
                            samples,
                            _long_press_timer: self.start_long_press_timer(pointer_id),
                        });
                    }
                }
                // A second finger turns the gesture into a pinch; tap, long
                // press, and scroll possibilities end here.
                TouchGestureState::Pending(pending) => {
                    *gesture = TouchGestureState::Pinching(PinchingTouch {
                        first_pointer_id: pending.pointer_id,
                        second_pointer_id: pointer_id,
                        first_position: pending.start,
                        second_position: position,
                        began: false,
                    });
                }
                TouchGestureState::Scrolling(scrolling) => {
                    interrupted_scroll_anchor = Some(scrolling.anchor);
                    *gesture = TouchGestureState::Pinching(PinchingTouch {
                        first_pointer_id: scrolling.pointer_id,
                        second_pointer_id: pointer_id,
                        first_position: scrolling.last,
                        second_position: position,
                        began: false,
                    });
                }
                // A third finger changes nothing.
                other => *gesture = other,
            }
        }

        if let Some(anchor) = interrupted_scroll_anchor {
            self.dispatch_input(PlatformInput::ScrollWheel(ScrollWheelEvent {
                position: anchor,
                delta: ScrollDelta::Pixels(Point::default()),
                modifiers: Modifiers::default(),
                touch_phase: TouchPhase::Ended,
            }));
        }
        if started_scroll {
            self.dispatch_input(PlatformInput::ScrollWheel(ScrollWheelEvent {
                position,
                delta: ScrollDelta::Pixels(Point::default()),
                modifiers: Modifiers::default(),
                touch_phase: TouchPhase::Started,
            }));
        }
    }

    fn handle_touch_move(&self, event: &web_sys::PointerEvent) {
        let tuning = GestureTuning::default();
        let pointer_id = event.pointer_id();
        let position = pointer_position_in_element(event);
        let time = monotonic_time_ms(&self.browser_window);

        let input = {
            let mut gesture = self.touch_gesture.borrow_mut();
            match &mut *gesture {
                TouchGestureState::Pending(pending) if pending.pointer_id == pointer_id => {
                    push_touch_sample(&mut pending.samples, time, position);
                    let travel = distance_between(position, pending.start);
                    if travel > f32::from(tuning.touch_slop) {
                        let anchor = pending.start;
                        // The opening delta is the full drag displacement
                        // with the slop consumed along it. GPUI derives the
                        // gesture's scroll axis from this event, so it must
                        // carry the drag's direction; the delta between the
                        // last two sub-slop moves is noise-dominated and
                        // frequently locked a horizontal drag to the
                        // vertical axis.
                        let displacement = position - anchor;
                        let slop_consumed = (travel - f32::from(tuning.touch_slop)) / travel;
                        let delta = point(
                            displacement.x * slop_consumed,
                            displacement.y * slop_consumed,
                        );
                        let samples = std::mem::take(&mut pending.samples);
                        *gesture = TouchGestureState::Scrolling(ScrollingTouch {
                            pointer_id,
                            anchor,
                            last: position,
                            last_time: time,
                            lead: point(0.0, 0.0),
                            samples,
                        });
                        Some(PlatformInput::ScrollWheel(ScrollWheelEvent {
                            position: anchor,
                            delta: ScrollDelta::Pixels(delta),
                            modifiers: Modifiers::default(),
                            touch_phase: TouchPhase::Started,
                        }))
                    } else {
                        None
                    }
                }
                TouchGestureState::Scrolling(scrolling) if scrolling.pointer_id == pointer_id => {
                    push_touch_sample(&mut scrolling.samples, time, position);
                    let velocity = velocity_from_samples(&scrolling.samples, time);
                    let elapsed_ms = (time - scrolling.last_time).max(0.0) as f32;
                    scrolling.last_time = time;
                    let target_lead = point(
                        velocity.x * TOUCH_EXTRAPOLATION_LEAD_MS / 1000.0,
                        velocity.y * TOUCH_EXTRAPOLATION_LEAD_MS / 1000.0,
                    );
                    let alpha = elapsed_ms / (elapsed_ms + TOUCH_LEAD_SMOOTHING_MS);
                    scrolling.lead.x += (target_lead.x - scrolling.lead.x) * alpha;
                    scrolling.lead.y += (target_lead.y - scrolling.lead.y) * alpha;
                    let display_position = point(
                        position.x + px(scrolling.lead.x),
                        position.y + px(scrolling.lead.y),
                    );
                    let delta = display_position - scrolling.last;
                    scrolling.last = display_position;
                    Some(PlatformInput::ScrollWheel(ScrollWheelEvent {
                        position: scrolling.anchor,
                        delta: ScrollDelta::Pixels(delta),
                        modifiers: Modifiers::default(),
                        touch_phase: TouchPhase::Moved,
                    }))
                }
                TouchGestureState::Pinching(pinching)
                    if pinching.first_pointer_id == pointer_id
                        || pinching.second_pointer_id == pointer_id =>
                {
                    let previous_distance =
                        distance_between(pinching.first_position, pinching.second_position);
                    if pinching.first_pointer_id == pointer_id {
                        pinching.first_position = position;
                    } else {
                        pinching.second_position = position;
                    }
                    let new_distance =
                        distance_between(pinching.first_position, pinching.second_position);
                    if previous_distance > 0.0 && new_distance > 0.0 {
                        let phase = if pinching.began {
                            TouchPhase::Moved
                        } else {
                            TouchPhase::Started
                        };
                        pinching.began = true;
                        Some(PlatformInput::Pinch(PinchEvent {
                            position: midpoint(pinching.first_position, pinching.second_position),
                            delta: new_distance / previous_distance - 1.0,
                            modifiers: Modifiers::default(),
                            phase,
                        }))
                    } else {
                        None
                    }
                }
                _ => None,
            }
        };

        if let Some(input) = input {
            self.dispatch_input(input);
        }
    }

    fn handle_touch_up(&self, event: &web_sys::PointerEvent) {
        let pointer_id = event.pointer_id();
        if !self.touch_gesture.borrow().involves_pointer(pointer_id) {
            return;
        }

        let tuning = GestureTuning::default();
        // Taken in a standalone statement: a `match` on the take expression
        // would hold the `RefCell` borrow for all arms, and the pinch arm
        // re-borrows to store the remaining finger's state.
        let gesture = std::mem::take(&mut *self.touch_gesture.borrow_mut());
        match gesture {
            TouchGestureState::Pending(pending) => {
                self.dispatch_touch_tap(
                    pending.start,
                    modifiers_from_mouse_event(event, self.is_mac),
                );
            }
            TouchGestureState::Scrolling(scrolling) => {
                let velocity = velocity_from_samples(
                    &scrolling.samples,
                    monotonic_time_ms(&self.browser_window),
                );
                let speed = (velocity.x.powi(2) + velocity.y.powi(2)).sqrt();
                if speed >= tuning.min_fling_velocity {
                    self.begin_fling(scrolling.anchor, velocity);
                } else {
                    self.dispatch_input(PlatformInput::ScrollWheel(ScrollWheelEvent {
                        position: scrolling.anchor,
                        delta: ScrollDelta::Pixels(Point::default()),
                        modifiers: Modifiers::default(),
                        touch_phase: TouchPhase::Ended,
                    }));
                }
            }
            TouchGestureState::Pinching(pinching) => {
                self.finish_pinch(&pinching, TouchPhase::Ended);
                // The remaining finger stays consumed: like a fling-catch,
                // lifting it must not click, and resuming a scroll mid-pinch
                // reads as jumpy rather than helpful.
                let remaining_pointer_id = if pinching.first_pointer_id == pointer_id {
                    pinching.second_pointer_id
                } else {
                    pinching.first_pointer_id
                };
                *self.touch_gesture.borrow_mut() = TouchGestureState::Consumed {
                    pointer_id: remaining_pointer_id,
                };
            }
            TouchGestureState::Consumed { .. } | TouchGestureState::None => {}
        }
    }

    /// The browser took the touch (system edge gesture, etc.); unwind without
    /// synthesizing a tap or a fling.
    fn handle_touch_cancel(&self, event: &web_sys::PointerEvent) {
        if !self
            .touch_gesture
            .borrow()
            .involves_pointer(event.pointer_id())
        {
            return;
        }
        let gesture = std::mem::take(&mut *self.touch_gesture.borrow_mut());
        match gesture {
            TouchGestureState::Scrolling(scrolling) => {
                self.dispatch_input(PlatformInput::ScrollWheel(ScrollWheelEvent {
                    position: scrolling.anchor,
                    delta: ScrollDelta::Pixels(Point::default()),
                    modifiers: Modifiers::default(),
                    touch_phase: TouchPhase::Cancelled,
                }));
            }
            TouchGestureState::Pinching(pinching) => {
                self.finish_pinch(&pinching, TouchPhase::Cancelled);
            }
            _ => {}
        }
    }

    fn finish_pinch(&self, pinching: &PinchingTouch, phase: TouchPhase) {
        if !pinching.began {
            return;
        }
        self.dispatch_input(PlatformInput::Pinch(PinchEvent {
            position: midpoint(pinching.first_position, pinching.second_position),
            delta: 0.0,
            modifiers: Modifiers::default(),
            phase,
        }));
    }

    fn start_long_press_timer(self: &Rc<Self>, pointer_id: i32) -> Option<LongPressTimer> {
        // Weak, so a dropped window is not kept alive by an in-flight timer.
        let weak_window = Rc::downgrade(self);
        let closure = Closure::<dyn FnMut()>::new(move || {
            if let Some(window) = weak_window.upgrade() {
                window.recognize_long_press(pointer_id);
            }
        });
        let handle = self
            .browser_window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                closure.as_ref().unchecked_ref(),
                GestureTuning::default().long_press_duration.as_millis() as i32,
            )
            .ok()?;
        Some(LongPressTimer {
            browser_window: self.browser_window.clone(),
            handle,
            _closure: closure,
        })
    }

    /// Long press is touch's secondary activation: it is synthesized as a
    /// right click so context menus open. This timer is the only long-press
    /// signal available on the web: iOS Safari never fires `contextmenu` for
    /// touches, and the canvas suppresses the browser's own touch callouts
    /// with `touch-action: none`.
    fn recognize_long_press(&self, pointer_id: i32) {
        let position = {
            let mut gesture = self.touch_gesture.borrow_mut();
            let TouchGestureState::Pending(pending) = &*gesture else {
                return;
            };
            if pending.pointer_id != pointer_id {
                return;
            }
            let position = pending.start;
            *gesture = TouchGestureState::Consumed { pointer_id };
            position
        };

        self.state.borrow_mut().mouse_position = position;
        self.dispatch_input(PlatformInput::MouseMove(MouseMoveEvent {
            position,
            pressed_button: None,
            modifiers: Modifiers::default(),
        }));
        self.dispatch_input(PlatformInput::MouseDown(MouseDownEvent {
            button: MouseButton::Right,
            position,
            modifiers: Modifiers::default(),
            click_count: 1,
            first_mouse: false,
        }));
        self.dispatch_input(PlatformInput::MouseUp(MouseUpEvent {
            button: MouseButton::Right,
            position,
            modifiers: Modifiers::default(),
            click_count: 1,
        }));
    }

    /// A tap is synthesized as a full left-click sequence at the touch's
    /// *starting* position: a MouseMove first so hover and hit-test state are
    /// established there, then MouseDown + MouseUp.
    fn dispatch_touch_tap(&self, position: Point<Pixels>, modifiers: Modifiers) {
        let tuning = GestureTuning::default();
        let click_count = self.click_state.borrow_mut().register_click(
            position,
            monotonic_time_ms(&self.browser_window),
            f32::from(tuning.multi_tap_slop),
        );

        {
            let mut current_state = self.state.borrow_mut();
            current_state.mouse_position = position;
            current_state.modifiers = modifiers;
        }

        self.dispatch_input(PlatformInput::MouseMove(MouseMoveEvent {
            position,
            pressed_button: None,
            modifiers,
        }));
        self.dispatch_input(PlatformInput::MouseDown(MouseDownEvent {
            button: MouseButton::Left,
            position,
            modifiers,
            click_count,
            first_mouse: false,
        }));
        self.dispatch_input(PlatformInput::MouseUp(MouseUpEvent {
            button: MouseButton::Left,
            position,
            modifiers,
            click_count,
        }));

        self.sync_virtual_keyboard();
    }

    /// Aligns the software keyboard with GPUI's focus after a tap has been
    /// dispatched. Mobile browsers show the keyboard only when an editable
    /// element is focused from within a user gesture, so this runs while the
    /// tap's `pointerup` is still on the stack. `readOnly` suppresses the
    /// keyboard while keeping hardware key events flowing to the hidden
    /// input; the blur/focus cycle is what makes the browser re-evaluate
    /// keyboard visibility, since `focus()` on an already-focused element is
    /// a no-op.
    fn sync_virtual_keyboard(&self) {
        let editable = self.state.borrow().input_handler.is_some();
        let was_editable = !self.input_element.read_only();
        self.input_element.set_read_only(!editable);
        if editable || was_editable {
            self.input_element.blur().ok();
            self.input_element.focus().ok();
        }
    }

    fn begin_fling(&self, position: Point<Pixels>, velocity: Point<f32>) {
        let speed = (velocity.x.powi(2) + velocity.y.powi(2)).sqrt();
        let curve = if self.is_android && speed > 0.0 {
            let duration_seconds = android_fling_duration_seconds(speed);
            FlingCurve::AndroidScroller {
                direction: point(velocity.x / speed, velocity.y / speed),
                distance: speed * duration_seconds / ANDROID_INITIAL_VELOCITY_PENETRATION,
                duration_ms: duration_seconds * 1000.0,
            }
        } else {
            FlingCurve::ExponentialDecay {
                initial_velocity: velocity,
            }
        };
        *self.fling.borrow_mut() = Some(FlingState {
            position,
            start_time: monotonic_time_ms(&self.browser_window),
            curve,
            dispatched_offset: point(0.0, 0.0),
        });
    }

    /// Ends any momentum scroll in flight, closing out its scroll gesture.
    /// Returns whether a fling was actually in progress.
    fn stop_fling(&self) -> bool {
        let Some(fling) = self.fling.borrow_mut().take() else {
            return false;
        };
        self.dispatch_input(PlatformInput::ScrollWheel(ScrollWheelEvent {
            position: fling.position,
            delta: ScrollDelta::Pixels(Point::default()),
            modifiers: Modifiers::default(),
            touch_phase: TouchPhase::Ended,
        }));
        true
    }

    /// Advances any active fling. Called from the platform's render
    /// `requestAnimationFrame` loop, immediately before the frame is drawn,
    /// so each frame's scroll delta is always applied in the same frame it
    /// was computed for; a separate animation-frame chain would race the
    /// render loop and alternate between leading and trailing it by a frame.
    ///
    /// `frame_time_ms` is the animation frame's own timestamp. Sampling the
    /// curve there rather than at callback execution time matters: callbacks
    /// run at a jittery offset into the frame under main-thread load, and at
    /// fling speeds a few milliseconds of sampling noise is a visible
    /// stutter. The frame timestamp is vsync-aligned and monotonic per frame.
    pub(crate) fn fling_tick(&self, frame_time_ms: f64) {
        let tuning = GestureTuning::default();
        let Some(mut fling) = self.fling.borrow_mut().take() else {
            return;
        };

        let elapsed_ms = (frame_time_ms - fling.start_time).max(0.0) as f32;
        let (offset, finished) = match &fling.curve {
            FlingCurve::ExponentialDecay { initial_velocity } => {
                let decay = tuning.momentum_decay_per_ms.powf(elapsed_ms);
                // Integral of `v0 * decay_per_ms^t` from 0 to t, in milliseconds.
                let curve_ms = (decay - 1.0) / tuning.momentum_decay_per_ms.ln();
                let offset = point(
                    initial_velocity.x / 1000.0 * curve_ms,
                    initial_velocity.y / 1000.0 * curve_ms,
                );
                let speed =
                    (initial_velocity.x.powi(2) + initial_velocity.y.powi(2)).sqrt() * decay;
                (offset, speed < tuning.min_fling_velocity)
            }
            FlingCurve::AndroidScroller {
                direction,
                distance,
                duration_ms,
            } => {
                let t = (elapsed_ms / duration_ms).min(1.0);
                let travelled = distance * android_fling_distance_penetration(t);
                (
                    point(direction.x * travelled, direction.y * travelled),
                    t >= 1.0,
                )
            }
        };
        let delta = point(
            px(offset.x - fling.dispatched_offset.x),
            px(offset.y - fling.dispatched_offset.y),
        );
        fling.dispatched_offset = offset;
        let position = fling.position;

        if finished {
            self.dispatch_input(PlatformInput::ScrollWheel(ScrollWheelEvent {
                position,
                delta: ScrollDelta::Pixels(delta),
                modifiers: Modifiers::default(),
                touch_phase: TouchPhase::Moved,
            }));
            self.dispatch_input(PlatformInput::ScrollWheel(ScrollWheelEvent {
                position,
                delta: ScrollDelta::Pixels(Point::default()),
                modifiers: Modifiers::default(),
                touch_phase: TouchPhase::Ended,
            }));
            return;
        }

        // Restore the state before dispatching so a handler that re-enters
        // (e.g. by stopping the fling) sees it.
        *self.fling.borrow_mut() = Some(fling);
        self.dispatch_input(PlatformInput::ScrollWheel(ScrollWheelEvent {
            position,
            delta: ScrollDelta::Pixels(delta),
            modifiers: Modifiers::default(),
            touch_phase: TouchPhase::Moved,
        }));
    }

    fn register_wheel(self: &Rc<Self>) -> EventListenerHandle {
        let this = Rc::clone(self);
        self.listen_non_passive("wheel", move |event: JsValue| {
            let event: web_sys::WheelEvent = event.unchecked_into();
            event.prevent_default();

            this.stop_fling();

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

    /// Paste is delivered through the DOM `paste` event rather than
    /// `Platform::read_from_clipboard`: the browser's asynchronous clipboard
    /// read API cannot fit that synchronous signature, while `ClipboardEvent`
    /// exposes `clipboardData` synchronously inside the event. It fires for
    /// any browser-initiated paste (keyboard, menu bar, context menu).
    fn register_paste(self: &Rc<Self>) -> EventListenerHandle {
        let this = Rc::clone(self);
        self.listen_input("paste", move |event: JsValue| {
            let event: web_sys::ClipboardEvent = event.unchecked_into();
            let Some(clipboard_data) = event.clipboard_data() else {
                return;
            };
            let Ok(text) = clipboard_data.get_data("text/plain") else {
                return;
            };
            if text.is_empty() {
                return;
            }

            event.prevent_default();
            this.with_input_handler(|handler| {
                handler.replace_text_in_range(None, &text);
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
                handler.replace_text_in_range(None, &data);
                handler.unmark_text();
            });
            this.input_element.set_value("");
        })
    }

    fn register_focus(self: &Rc<Self>) -> EventListenerHandle {
        let this = Rc::clone(self);
        self.listen_input("focus", move |_event: JsValue| {
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

pub(crate) fn is_android_platform(browser_window: &web_sys::Window) -> bool {
    browser_window
        .navigator()
        .user_agent()
        .is_ok_and(|user_agent| user_agent.contains("Android"))
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
