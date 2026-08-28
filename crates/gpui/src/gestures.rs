//! Touch gesture recognition vocabulary.
//!
//! GPUI recognizes gestures from raw [`TouchEvent`](crate::TouchEvent)s in a
//! single, portable arena in gpui core: recognizers compete for in-flight
//! touches, winners claim them, and losers are cancelled. Recognized gestures
//! are surfaced through *existing* semantic events wherever possible, a tap
//! becomes [`ClickEvent::Touch`](crate::ClickEvent), a pan becomes
//! [`ScrollWheelEvent`](crate::ScrollWheelEvent)s carrying a
//! [`TouchPhase`](crate::TouchPhase), and a pinch becomes
//! [`PinchEvent`](crate::PinchEvent)s — so components written against
//! `on_click` and scroll containers work untouched on mobile.

use std::collections::VecDeque;
use std::mem;
use std::time::Duration;

use scheduler::Instant;
use smallvec::SmallVec;

use crate::{
    Axis, IsZero, Modifiers, MouseButton, MouseDownEvent, MouseUpEvent, Pixels, Point, ScrollDelta,
    ScrollWheelEvent, TouchEvent, TouchId, TouchPhase, point, px,
};

const SCROLL_EVENT_SEPARATION: Duration = Duration::from_millis(28);

/// Tracks the dominant axis across the events in a scroll gesture.
#[derive(Clone, Copy, Debug, Default)]
pub struct OngoingScroll {
    last_event: Option<Instant>,
    axis: Option<Axis>,
}

impl OngoingScroll {
    /// Filters the given delta to the dominant axis of the current scroll gesture.
    ///
    /// Gestures are delimited by their touch phase when available, with a timeout
    /// fallback for platforms that only emit [`TouchPhase::Moved`].
    pub fn filter(&mut self, delta: &mut Point<Pixels>, touch_phase: TouchPhase) {
        self.filter_at(delta, touch_phase, Instant::now())
    }

    fn filter_at(&mut self, delta: &mut Point<Pixels>, touch_phase: TouchPhase, now: Instant) {
        const UNLOCK_PERCENT: f32 = 1.9;
        const UNLOCK_LOWER_BOUND: Pixels = px(6.);

        if matches!(touch_phase, TouchPhase::Ended | TouchPhase::Cancelled) {
            self.last_event = None;
            self.axis = None;
            return;
        }

        let x = delta.x.abs();
        let y = delta.y.abs();
        if x.is_zero() && y.is_zero() {
            if touch_phase == TouchPhase::Started {
                self.last_event = None;
                self.axis = None;
            }
            return;
        }

        let starts_new_gesture = touch_phase == TouchPhase::Started
            || self
                .last_event
                .is_none_or(|last_event| now.duration_since(last_event) >= SCROLL_EVENT_SEPARATION);
        let mut axis = self.axis;
        if starts_new_gesture {
            axis = if x <= y {
                Some(Axis::Vertical)
            } else {
                Some(Axis::Horizontal)
            };
        } else if x.max(y) >= UNLOCK_LOWER_BOUND {
            match axis {
                Some(Axis::Vertical) if x > y && x >= y * UNLOCK_PERCENT => {
                    axis = None;
                }
                Some(Axis::Horizontal) if y > x && y >= x * UNLOCK_PERCENT => {
                    axis = None;
                }
                _ => {}
            }
        }

        self.last_event = Some(now);
        self.axis = axis;
        match axis {
            Some(Axis::Vertical) => delta.x = Pixels::ZERO,
            Some(Axis::Horizontal) => delta.y = Pixels::ZERO,
            None => {}
        }
    }
}

/// Feel constants consumed by gesture recognizers. Provided on a best-effort
/// basis, depending on each platform's support, defaulting to GPUI's own
/// (iOS flavored) values
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GestureTuning {
    /// Distance a touch may travel before it stops being a potential tap and
    /// becomes a pan/drag.
    pub touch_slop: Pixels,
    /// Maximum interval between taps for them to accumulate a tap count.
    pub multi_tap_interval: Duration,
    /// Maximum distance between taps for them to accumulate a tap count.
    pub multi_tap_slop: Pixels,
    /// How long a touch must remain within [`Self::touch_slop`] to be
    /// recognized as a long press.
    pub long_press_duration: Duration,
    /// Per-millisecond decay factor applied to scroll momentum after a fling.
    /// (`UIScrollView` uses `0.998` per millisecond for its normal
    /// deceleration rate.)
    pub momentum_decay_per_ms: f32,
    /// Minimum release velocity, in pixels per second, required to start
    /// scroll momentum.
    pub min_fling_velocity: f32,
}

impl Default for GestureTuning {
    fn default() -> Self {
        Self {
            touch_slop: px(8.),
            multi_tap_interval: Duration::from_millis(400),
            multi_tap_slop: px(16.),
            long_press_duration: Duration::from_millis(500),
            momentum_decay_per_ms: 0.998,
            min_fling_velocity: 50.,
        }
    }
}

/// The set of gesture kinds that participate in recognition.
///
/// Used by [`PlatformGestures::native_recognizers`] to declare which gestures
/// the platform recognizes natively rather than leaving to gpui core's
/// portable recognizers.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GestureKinds {
    /// Tap (and multi-tap), surfaced as [`ClickEvent::Touch`](crate::ClickEvent).
    pub tap: bool,
    /// Long press, surfaced as [`LongPressEvent`].
    pub long_press: bool,
    /// Pan/scroll (including fling momentum), surfaced as
    /// [`ScrollWheelEvent`](crate::ScrollWheelEvent)s.
    pub pan: bool,
    /// Pinch to zoom, surfaced as [`PinchEvent`](crate::PinchEvent)s.
    pub pinch: bool,
}

impl GestureKinds {
    /// No gestures; gpui core's portable recognizers handle everything.
    pub const NONE: Self = Self {
        tap: false,
        long_press: false,
        pan: false,
        pinch: false,
    };

    /// All gesture kinds.
    pub const ALL: Self = Self {
        tap: true,
        long_press: true,
        pan: true,
        pinch: true,
    };
}

/// A long-press gesture, mobile's context-menu trigger.
///
/// A bare long press is surfaced as a [`ClickEvent`](crate::ClickEvent) with
/// `long_press: true`, delivered to aux-click listeners alongside right
/// clicks. This event is the raw hook for elements that need the gesture
/// itself (e.g. long-press to start a drag); the registration API ships
/// together with the gesture arena.
#[derive(Clone, Debug, Default)]
pub struct LongPressEvent {
    /// The position of the touch that was recognized as a long press.
    pub position: Point<Pixels>,
}

/// Platform gesture recognition services.
///
/// If your mobile platform supports native gesture recognition, use this
/// to share it with GPUI.
pub trait PlatformGestures {
    /// Feel constants for the portable recognizers on this platform.
    fn tuning(&self) -> GestureTuning {
        GestureTuning::default()
    }

    /// The gesture kinds this platform recognizes natively.
    fn native_recognizers(&self) -> GestureKinds {
        GestureKinds::NONE
    }
}

/// A no-op [`PlatformGestures`] implementation: no native recognizers and
/// default tuning. Suitable for desktop platforms and tests.
pub struct NullPlatformGestures;

impl PlatformGestures for NullPlatformGestures {}

/// Ceiling on recognized fling velocity, in pixels per second (matches
/// Flutter's `kMaxFlingVelocity`).
const MAX_FLING_VELOCITY: f32 = 8000.;

/// Momentum below this speed, in pixels per second, is imperceptible: the
/// fling stops and the synthetic scroll stream is closed.
const MOMENTUM_STOP_VELOCITY: f32 = 10.;

/// Upper bound on the time a single momentum tick may integrate, so a stalled
/// frame loop (backgrounded window, long pause) resumes without a huge jump.
const MOMENTUM_MAX_TICK: Duration = Duration::from_millis(50);

/// How far back the release-velocity estimate looks. Samples older than this
/// reflect an earlier part of the gesture, not the speed at release.
const VELOCITY_WINDOW: Duration = Duration::from_millis(100);

const VELOCITY_MAX_SAMPLES: usize = 20;

/// The portable recognizer behind raw touch input: it watches the
/// [`TouchEvent`] stream for one touch at a time and resolves it into either
/// a tap or a pan, following the competition model described in the module
/// docs. Pans continue into post-release momentum when the touch lifts at
/// speed; the window drives that phase through [`Self::tick_momentum`].
///
/// Taps are currently surfaced as synthesized mouse presses rather than
/// [`ClickEvent::Touch`](crate::ClickEvent), which keeps every existing
/// mouse-driven behavior (click listeners, caret placement, double-tap
/// selection) working before elements grow a direct tap-delivery path.
/// Long-press and pinch recognition are not implemented yet, and additional
/// touches are ignored while one is being recognized.
pub(crate) struct TouchGestureRecognizer {
    tuning: GestureTuning,
    state: TouchGestureState,
    momentum: Option<Momentum>,
    last_tap: Option<CompletedTap>,
}

/// A semantic event recognized from raw touches, ready to dispatch through
/// the window's existing input paths.
#[derive(Debug)]
pub(crate) enum RecognizedTouchGesture {
    /// One step of a pan (or of its post-release momentum), delivered to
    /// scroll listeners at the pan's starting position.
    Scroll(ScrollWheelEvent),
    /// A recognized tap, delivered as a synthesized mouse press and release.
    Tap {
        down: MouseDownEvent,
        up: MouseUpEvent,
    },
}

enum TouchGestureState {
    Idle,
    /// The touch is still within `touch_slop` of where it started: it can
    /// still resolve into either a tap or a pan.
    Pending(ActiveTouch),
    /// The touch exceeded `touch_slop`: it is a pan until it ends, and its
    /// movement flows out as scroll events.
    Panning(ActiveTouch),
}

struct ActiveTouch {
    id: TouchId,
    start_position: Point<Pixels>,
    last_position: Point<Pixels>,
    velocity_tracker: VelocityTracker,
}

struct CompletedTap {
    position: Point<Pixels>,
    time: Instant,
    count: usize,
}

struct Momentum {
    /// Where the pan started; synthesized scroll events keep hit-testing
    /// there so momentum stays with the container the gesture began on.
    position: Point<Pixels>,
    /// Pixels per second.
    velocity: Point<f32>,
    last_tick: Instant,
}

impl TouchGestureRecognizer {
    pub(crate) fn new(tuning: GestureTuning) -> Self {
        Self {
            tuning,
            state: TouchGestureState::Idle,
            momentum: None,
            last_tap: None,
        }
    }

    pub(crate) fn handle_event(
        &mut self,
        event: &TouchEvent,
    ) -> SmallVec<[RecognizedTouchGesture; 2]> {
        self.handle_event_at(event, Instant::now())
    }

    fn handle_event_at(
        &mut self,
        event: &TouchEvent,
        now: Instant,
    ) -> SmallVec<[RecognizedTouchGesture; 2]> {
        let mut recognized = SmallVec::new();
        match event.phase {
            TouchPhase::Started => {
                if let Some(momentum) = self.momentum.take() {
                    recognized.push(RecognizedTouchGesture::Scroll(scroll_event(
                        momentum.position,
                        Point::default(),
                        TouchPhase::Ended,
                    )));
                }
                if matches!(self.state, TouchGestureState::Idle) {
                    let mut velocity_tracker = VelocityTracker::default();
                    velocity_tracker.push(now, event.position);
                    self.state = TouchGestureState::Pending(ActiveTouch {
                        id: event.id,
                        start_position: event.position,
                        last_position: event.position,
                        velocity_tracker,
                    });
                }
            }
            TouchPhase::Moved => match mem::replace(&mut self.state, TouchGestureState::Idle) {
                TouchGestureState::Pending(mut touch) if touch.id == event.id => {
                    touch.velocity_tracker.push(now, event.position);
                    let accumulated = event.position - touch.start_position;
                    if accumulated.magnitude() > f64::from(self.tuning.touch_slop) {
                        // Carry the full movement so far into the first scroll
                        // step: the content catches up to the finger instead
                        // of losing the slop distance.
                        touch.last_position = event.position;
                        recognized.push(RecognizedTouchGesture::Scroll(scroll_event(
                            touch.start_position,
                            accumulated,
                            TouchPhase::Started,
                        )));
                        self.state = TouchGestureState::Panning(touch);
                    } else {
                        self.state = TouchGestureState::Pending(touch);
                    }
                }
                TouchGestureState::Panning(mut touch) if touch.id == event.id => {
                    touch.velocity_tracker.push(now, event.position);
                    let delta = event.position - touch.last_position;
                    touch.last_position = event.position;
                    recognized.push(RecognizedTouchGesture::Scroll(scroll_event(
                        touch.start_position,
                        delta,
                        TouchPhase::Moved,
                    )));
                    self.state = TouchGestureState::Panning(touch);
                }
                other => self.state = other,
            },
            TouchPhase::Ended => match mem::replace(&mut self.state, TouchGestureState::Idle) {
                TouchGestureState::Pending(touch) if touch.id == event.id => {
                    let tap_count = match &self.last_tap {
                        Some(tap)
                            if now.duration_since(tap.time) <= self.tuning.multi_tap_interval
                                && (event.position - tap.position).magnitude()
                                    <= f64::from(self.tuning.multi_tap_slop) =>
                        {
                            tap.count + 1
                        }
                        _ => 1,
                    };
                    self.last_tap = Some(CompletedTap {
                        position: event.position,
                        time: now,
                        count: tap_count,
                    });
                    recognized.push(RecognizedTouchGesture::Tap {
                        down: MouseDownEvent {
                            button: MouseButton::Left,
                            position: event.position,
                            modifiers: Modifiers::default(),
                            click_count: tap_count,
                            first_mouse: false,
                        },
                        up: MouseUpEvent {
                            button: MouseButton::Left,
                            position: event.position,
                            modifiers: Modifiers::default(),
                            click_count: tap_count,
                        },
                    });
                }
                TouchGestureState::Panning(mut touch) if touch.id == event.id => {
                    touch.velocity_tracker.push(now, event.position);
                    let delta = event.position - touch.last_position;
                    recognized.push(RecognizedTouchGesture::Scroll(scroll_event(
                        touch.start_position,
                        delta,
                        TouchPhase::Ended,
                    )));
                    let velocity = touch.velocity_tracker.velocity();
                    let speed = (velocity.x.powi(2) + velocity.y.powi(2)).sqrt();
                    if speed >= self.tuning.min_fling_velocity {
                        let velocity = if speed > MAX_FLING_VELOCITY {
                            velocity * (MAX_FLING_VELOCITY / speed)
                        } else {
                            velocity
                        };
                        self.momentum = Some(Momentum {
                            position: touch.start_position,
                            velocity,
                            last_tick: now,
                        });
                    }
                }
                other => self.state = other,
            },
            TouchPhase::Cancelled => match mem::replace(&mut self.state, TouchGestureState::Idle) {
                TouchGestureState::Pending(touch) if touch.id == event.id => {}
                TouchGestureState::Panning(touch) if touch.id == event.id => {
                    recognized.push(RecognizedTouchGesture::Scroll(scroll_event(
                        touch.start_position,
                        Point::default(),
                        TouchPhase::Cancelled,
                    )));
                }
                other => self.state = other,
            },
        }
        recognized
    }

    pub(crate) fn has_momentum(&self) -> bool {
        self.momentum.is_some()
    }

    /// Advances post-fling momentum by one frame, returning the scroll step
    /// to dispatch, or `None` when no momentum is in progress. The final step
    /// carries [`TouchPhase::Ended`] to close the synthetic scroll stream.
    pub(crate) fn tick_momentum(&mut self) -> Option<RecognizedTouchGesture> {
        self.tick_momentum_at(Instant::now())
    }

    fn tick_momentum_at(&mut self, now: Instant) -> Option<RecognizedTouchGesture> {
        let momentum = self.momentum.as_mut()?;
        let elapsed = now
            .duration_since(momentum.last_tick)
            .min(MOMENTUM_MAX_TICK);
        momentum.last_tick = now;
        let delta = point(
            px(momentum.velocity.x * elapsed.as_secs_f32()),
            px(momentum.velocity.y * elapsed.as_secs_f32()),
        );
        momentum.velocity *= self
            .tuning
            .momentum_decay_per_ms
            .powf(elapsed.as_secs_f32() * 1000.);
        let speed = (momentum.velocity.x.powi(2) + momentum.velocity.y.powi(2)).sqrt();
        let position = momentum.position;
        if speed < MOMENTUM_STOP_VELOCITY {
            self.momentum = None;
            Some(RecognizedTouchGesture::Scroll(scroll_event(
                position,
                delta,
                TouchPhase::Ended,
            )))
        } else {
            Some(RecognizedTouchGesture::Scroll(scroll_event(
                position,
                delta,
                TouchPhase::Moved,
            )))
        }
    }
}

fn scroll_event(
    position: Point<Pixels>,
    delta: Point<Pixels>,
    touch_phase: TouchPhase,
) -> ScrollWheelEvent {
    ScrollWheelEvent {
        position,
        delta: ScrollDelta::Pixels(delta),
        modifiers: Modifiers::default(),
        touch_phase,
    }
}

/// Estimates release velocity from recent positions. Flutter fits a
/// second-degree polynomial by least squares over an equivalent window
/// (`velocity_tracker.dart`); this endpoint difference is a simpler first
/// cut.
#[derive(Default)]
struct VelocityTracker {
    samples: VecDeque<(Instant, Point<Pixels>)>,
}

impl VelocityTracker {
    fn push(&mut self, time: Instant, position: Point<Pixels>) {
        self.samples.push_back((time, position));
        while self.samples.len() > VELOCITY_MAX_SAMPLES {
            self.samples.pop_front();
        }
        while let Some((first_time, _)) = self.samples.front() {
            if self.samples.len() > 2 && time.duration_since(*first_time) > VELOCITY_WINDOW {
                self.samples.pop_front();
            } else {
                break;
            }
        }
    }

    /// Velocity across the sampled window, in pixels per second.
    fn velocity(&self) -> Point<f32> {
        let (Some((first_time, first_position)), Some((last_time, last_position))) =
            (self.samples.front(), self.samples.back())
        else {
            return Point::default();
        };
        let elapsed = last_time.duration_since(*first_time).as_secs_f32();
        if elapsed <= f32::EPSILON {
            return Point::default();
        }
        point(
            f32::from(last_position.x - first_position.x) / elapsed,
            f32::from(last_position.y - first_position.y) / elapsed,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::point;

    #[test]
    fn ongoing_scroll_locks_to_dominant_axis() {
        let now = Instant::now();
        let mut ongoing_scroll = OngoingScroll::default();
        let mut horizontal_delta = point(px(10.), px(2.));
        ongoing_scroll.filter_at(&mut horizontal_delta, TouchPhase::Started, now);
        assert_eq!(ongoing_scroll.axis, Some(Axis::Horizontal));
        assert_eq!(horizontal_delta, point(px(10.), px(0.)));

        let mut continued_delta = point(px(3.), px(2.));
        ongoing_scroll.filter_at(
            &mut continued_delta,
            TouchPhase::Moved,
            now + Duration::from_millis(1),
        );
        assert_eq!(ongoing_scroll.axis, Some(Axis::Horizontal));
        assert_eq!(continued_delta, point(px(3.), px(0.)));
    }

    #[test]
    fn ongoing_scroll_unlocks_when_direction_changes() {
        let now = Instant::now();
        let mut ongoing_scroll = OngoingScroll::default();
        let mut horizontal_delta = point(px(10.), px(2.));
        ongoing_scroll.filter_at(&mut horizontal_delta, TouchPhase::Started, now);

        let mut vertical_delta = point(px(2.), px(10.));
        ongoing_scroll.filter_at(
            &mut vertical_delta,
            TouchPhase::Moved,
            now + Duration::from_millis(1),
        );
        assert_eq!(ongoing_scroll.axis, None);
        assert_eq!(vertical_delta, point(px(2.), px(10.)));
    }

    #[test]
    fn ongoing_scroll_starts_new_gesture_at_timeout_boundary() {
        let now = Instant::now();
        let mut ongoing_scroll = OngoingScroll::default();
        let mut horizontal_delta = point(px(10.), px(2.));
        ongoing_scroll.filter_at(&mut horizontal_delta, TouchPhase::Moved, now);

        let mut vertical_delta = point(px(2.), px(10.));
        ongoing_scroll.filter_at(
            &mut vertical_delta,
            TouchPhase::Moved,
            now + SCROLL_EVENT_SEPARATION,
        );
        assert_eq!(ongoing_scroll.axis, Some(Axis::Vertical));
        assert_eq!(vertical_delta, point(px(0.), px(10.)));
    }

    #[test]
    fn ongoing_scroll_ignores_zero_delta_and_resets_when_ended() {
        let now = Instant::now();
        let mut ongoing_scroll = OngoingScroll::default();
        let mut horizontal_delta = point(px(10.), px(2.));
        ongoing_scroll.filter_at(&mut horizontal_delta, TouchPhase::Started, now);

        let mut zero_delta = Point::default();
        ongoing_scroll.filter_at(
            &mut zero_delta,
            TouchPhase::Ended,
            now + Duration::from_millis(1),
        );
        assert_eq!(ongoing_scroll.axis, None);

        let mut vertical_delta = point(px(2.), px(3.));
        ongoing_scroll.filter_at(
            &mut vertical_delta,
            TouchPhase::Moved,
            now + Duration::from_millis(2),
        );
        assert_eq!(ongoing_scroll.axis, Some(Axis::Vertical));
        assert_eq!(vertical_delta, point(px(0.), px(3.)));
    }

    #[test]
    fn ongoing_scroll_ignores_zero_delta_movement() {
        let now = Instant::now();
        let mut ongoing_scroll = OngoingScroll::default();
        let mut horizontal_delta = point(px(10.), px(2.));
        ongoing_scroll.filter_at(&mut horizontal_delta, TouchPhase::Started, now);

        let mut zero_delta = Point::default();
        ongoing_scroll.filter_at(
            &mut zero_delta,
            TouchPhase::Moved,
            now + SCROLL_EVENT_SEPARATION,
        );

        let mut vertical_delta = point(px(2.), px(10.));
        ongoing_scroll.filter_at(
            &mut vertical_delta,
            TouchPhase::Moved,
            now + SCROLL_EVENT_SEPARATION,
        );
        assert_eq!(ongoing_scroll.axis, Some(Axis::Vertical));
        assert_eq!(vertical_delta, point(px(0.), px(10.)));
    }

    #[test]
    fn ongoing_scroll_supports_moved_only_platforms() {
        let now = Instant::now();
        let mut ongoing_scroll = OngoingScroll::default();
        let mut horizontal_delta = point(px(10.), px(2.));
        ongoing_scroll.filter_at(&mut horizontal_delta, TouchPhase::Moved, now);
        assert_eq!(ongoing_scroll.axis, Some(Axis::Horizontal));
        assert_eq!(horizontal_delta, point(px(10.), px(0.)));
    }

    #[test]
    fn touch_within_slop_resolves_to_tap() {
        let mut recognizer = TouchGestureRecognizer::new(GestureTuning::default());
        let now = Instant::now();
        let touch = TouchId(1);

        let recognized =
            recognizer.handle_event_at(&touch_event(touch, TouchPhase::Started, 10., 10.), now);
        assert!(recognized.is_empty());
        let recognized = recognizer.handle_event_at(
            &touch_event(touch, TouchPhase::Moved, 12., 11.),
            now + Duration::from_millis(20),
        );
        assert!(recognized.is_empty());

        let recognized = recognizer.handle_event_at(
            &touch_event(touch, TouchPhase::Ended, 12., 11.),
            now + Duration::from_millis(60),
        );
        let [RecognizedTouchGesture::Tap { down, up }] = recognized.as_slice() else {
            panic!("expected tap, got {recognized:?}");
        };
        assert_eq!(down.click_count, 1);
        assert_eq!(down.position, point(px(12.), px(11.)));
        assert_eq!(up.click_count, 1);
    }

    #[test]
    fn consecutive_taps_accumulate_tap_count() {
        let mut recognizer = TouchGestureRecognizer::new(GestureTuning::default());
        let now = Instant::now();

        recognizer.handle_event_at(&touch_event(TouchId(1), TouchPhase::Started, 10., 10.), now);
        recognizer.handle_event_at(
            &touch_event(TouchId(1), TouchPhase::Ended, 10., 10.),
            now + Duration::from_millis(40),
        );

        let second_down = now + Duration::from_millis(200);
        recognizer.handle_event_at(
            &touch_event(TouchId(2), TouchPhase::Started, 14., 10.),
            second_down,
        );
        let recognized = recognizer.handle_event_at(
            &touch_event(TouchId(2), TouchPhase::Ended, 14., 10.),
            second_down + Duration::from_millis(40),
        );
        let [RecognizedTouchGesture::Tap { down, .. }] = recognized.as_slice() else {
            panic!("expected tap, got {recognized:?}");
        };
        assert_eq!(down.click_count, 2);

        let late_down = second_down + Duration::from_secs(2);
        recognizer.handle_event_at(
            &touch_event(TouchId(3), TouchPhase::Started, 14., 10.),
            late_down,
        );
        let recognized = recognizer.handle_event_at(
            &touch_event(TouchId(3), TouchPhase::Ended, 14., 10.),
            late_down + Duration::from_millis(40),
        );
        let [RecognizedTouchGesture::Tap { down, .. }] = recognized.as_slice() else {
            panic!("expected tap, got {recognized:?}");
        };
        assert_eq!(down.click_count, 1);
    }

    #[test]
    fn touch_beyond_slop_resolves_to_pan() {
        let mut recognizer = TouchGestureRecognizer::new(GestureTuning::default());
        let now = Instant::now();
        let touch = TouchId(1);

        recognizer.handle_event_at(&touch_event(touch, TouchPhase::Started, 100., 100.), now);

        let recognized = recognizer.handle_event_at(
            &touch_event(touch, TouchPhase::Moved, 100., 120.),
            now + Duration::from_millis(16),
        );
        let [RecognizedTouchGesture::Scroll(scroll)] = recognized.as_slice() else {
            panic!("expected scroll, got {recognized:?}");
        };
        assert_eq!(scroll.touch_phase, TouchPhase::Started);
        assert_eq!(scroll.position, point(px(100.), px(100.)));
        assert_eq!(scroll.delta.pixel_delta(px(16.)), point(px(0.), px(20.)));

        let recognized = recognizer.handle_event_at(
            &touch_event(touch, TouchPhase::Moved, 100., 135.),
            now + Duration::from_millis(32),
        );
        let [RecognizedTouchGesture::Scroll(scroll)] = recognized.as_slice() else {
            panic!("expected scroll, got {recognized:?}");
        };
        assert_eq!(scroll.touch_phase, TouchPhase::Moved);
        assert_eq!(scroll.position, point(px(100.), px(100.)));
        assert_eq!(scroll.delta.pixel_delta(px(16.)), point(px(0.), px(15.)));

        let recognized = recognizer.handle_event_at(
            &touch_event(touch, TouchPhase::Ended, 100., 135.),
            now + Duration::from_millis(48),
        );
        let [RecognizedTouchGesture::Scroll(scroll)] = recognized.as_slice() else {
            panic!("expected scroll, got {recognized:?}");
        };
        assert_eq!(scroll.touch_phase, TouchPhase::Ended);
    }

    #[test]
    fn fast_release_starts_momentum_that_decays_to_a_stop() {
        let mut recognizer = TouchGestureRecognizer::new(GestureTuning::default());
        let now = Instant::now();
        let touch = TouchId(1);

        recognizer.handle_event_at(&touch_event(touch, TouchPhase::Started, 100., 300.), now);
        for step in 1..=5 {
            recognizer.handle_event_at(
                &touch_event(touch, TouchPhase::Moved, 100., 300. - step as f32 * 20.),
                now + Duration::from_millis(step * 16),
            );
        }
        recognizer.handle_event_at(
            &touch_event(touch, TouchPhase::Ended, 100., 200.),
            now + Duration::from_millis(6 * 16),
        );
        assert!(recognizer.has_momentum());

        let tick = now + Duration::from_millis(6 * 16 + 16);
        let recognized = recognizer.tick_momentum_at(tick);
        let Some(RecognizedTouchGesture::Scroll(scroll)) = recognized else {
            panic!("expected momentum scroll, got {recognized:?}");
        };
        assert_eq!(scroll.touch_phase, TouchPhase::Moved);
        assert_eq!(scroll.position, point(px(100.), px(300.)));
        let delta = scroll.delta.pixel_delta(px(16.));
        assert!(
            delta.y < px(0.),
            "momentum should continue upward, got {delta:?}"
        );
        assert!(delta.x.is_zero());

        let mut last_phase = TouchPhase::Moved;
        let mut ticks = 0;
        let mut time = tick;
        while recognizer.has_momentum() {
            time += Duration::from_millis(16);
            ticks += 1;
            assert!(ticks < 1000, "momentum never stopped");
            if let Some(RecognizedTouchGesture::Scroll(scroll)) = recognizer.tick_momentum_at(time)
            {
                last_phase = scroll.touch_phase;
            }
        }
        assert_eq!(last_phase, TouchPhase::Ended);
        assert!(recognizer.tick_momentum_at(time).is_none());
    }

    #[test]
    fn slow_release_does_not_start_momentum() {
        let mut recognizer = TouchGestureRecognizer::new(GestureTuning::default());
        let now = Instant::now();
        let touch = TouchId(1);

        recognizer.handle_event_at(&touch_event(touch, TouchPhase::Started, 100., 300.), now);
        recognizer.handle_event_at(
            &touch_event(touch, TouchPhase::Moved, 100., 280.),
            now + Duration::from_millis(16),
        );
        recognizer.handle_event_at(
            &touch_event(touch, TouchPhase::Moved, 100., 279.),
            now + Duration::from_millis(500),
        );
        recognizer.handle_event_at(
            &touch_event(touch, TouchPhase::Ended, 100., 279.),
            now + Duration::from_millis(600),
        );
        assert!(!recognizer.has_momentum());
    }

    #[test]
    fn new_touch_interrupts_momentum() {
        let mut recognizer = TouchGestureRecognizer::new(GestureTuning::default());
        let now = Instant::now();

        recognizer.handle_event_at(
            &touch_event(TouchId(1), TouchPhase::Started, 100., 300.),
            now,
        );
        recognizer.handle_event_at(
            &touch_event(TouchId(1), TouchPhase::Moved, 100., 200.),
            now + Duration::from_millis(50),
        );
        recognizer.handle_event_at(
            &touch_event(TouchId(1), TouchPhase::Ended, 100., 200.),
            now + Duration::from_millis(66),
        );
        assert!(recognizer.has_momentum());

        let recognized = recognizer.handle_event_at(
            &touch_event(TouchId(2), TouchPhase::Started, 100., 200.),
            now + Duration::from_millis(200),
        );
        assert!(!recognizer.has_momentum());
        let [RecognizedTouchGesture::Scroll(scroll)] = recognized.as_slice() else {
            panic!("expected closing scroll, got {recognized:?}");
        };
        assert_eq!(scroll.touch_phase, TouchPhase::Ended);
        assert!(scroll.delta.pixel_delta(px(16.)).is_zero());
    }

    #[test]
    fn cancelled_pan_emits_cancelled_scroll_and_no_tap() {
        let mut recognizer = TouchGestureRecognizer::new(GestureTuning::default());
        let now = Instant::now();
        let touch = TouchId(1);

        recognizer.handle_event_at(&touch_event(touch, TouchPhase::Started, 100., 100.), now);
        recognizer.handle_event_at(
            &touch_event(touch, TouchPhase::Moved, 100., 150.),
            now + Duration::from_millis(16),
        );
        let recognized = recognizer.handle_event_at(
            &touch_event(touch, TouchPhase::Cancelled, 100., 150.),
            now + Duration::from_millis(32),
        );
        let [RecognizedTouchGesture::Scroll(scroll)] = recognized.as_slice() else {
            panic!("expected cancelled scroll, got {recognized:?}");
        };
        assert_eq!(scroll.touch_phase, TouchPhase::Cancelled);
        assert!(!recognizer.has_momentum());

        let mut recognizer = TouchGestureRecognizer::new(GestureTuning::default());
        recognizer.handle_event_at(&touch_event(touch, TouchPhase::Started, 100., 100.), now);
        let recognized = recognizer.handle_event_at(
            &touch_event(touch, TouchPhase::Cancelled, 100., 102.),
            now + Duration::from_millis(16),
        );
        assert!(recognized.is_empty(), "cancelled tap must not click");
    }

    #[test]
    fn concurrent_touches_are_ignored_while_one_is_active() {
        let mut recognizer = TouchGestureRecognizer::new(GestureTuning::default());
        let now = Instant::now();

        recognizer.handle_event_at(
            &touch_event(TouchId(1), TouchPhase::Started, 100., 100.),
            now,
        );
        let recognized = recognizer.handle_event_at(
            &touch_event(TouchId(2), TouchPhase::Started, 200., 200.),
            now + Duration::from_millis(8),
        );
        assert!(recognized.is_empty());
        let recognized = recognizer.handle_event_at(
            &touch_event(TouchId(2), TouchPhase::Moved, 200., 300.),
            now + Duration::from_millis(16),
        );
        assert!(recognized.is_empty());
        let recognized = recognizer.handle_event_at(
            &touch_event(TouchId(2), TouchPhase::Ended, 200., 300.),
            now + Duration::from_millis(24),
        );
        assert!(recognized.is_empty());

        // The first touch still resolves normally.
        let recognized = recognizer.handle_event_at(
            &touch_event(TouchId(1), TouchPhase::Moved, 100., 150.),
            now + Duration::from_millis(32),
        );
        let [RecognizedTouchGesture::Scroll(scroll)] = recognized.as_slice() else {
            panic!("expected scroll, got {recognized:?}");
        };
        assert_eq!(scroll.touch_phase, TouchPhase::Started);
    }

    fn touch_event(id: TouchId, phase: TouchPhase, x: f32, y: f32) -> TouchEvent {
        TouchEvent {
            id,
            phase,
            position: point(px(x), px(y)),
            force: None,
        }
    }
}
