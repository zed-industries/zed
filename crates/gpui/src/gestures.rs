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

use std::time::{Duration, Instant};

use smallvec::{SmallVec, smallvec};

use crate::{
    Axis, IsZero, Modifiers, Pixels, PlatformInput, Point, ScrollDelta, ScrollWheelEvent,
    TouchClickEvent, TouchEvent, TouchId, TouchPhase, px,
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

pub(crate) struct TouchGestureArena {
    tuning: GestureTuning,
    active_touch: Option<ActiveTouch>,
    last_tap: Option<CompletedTap>,
}

pub(crate) enum TouchGestureOutput {
    PlatformInput(PlatformInput),
    Click(TouchClickEvent),
}

struct ActiveTouch {
    id: TouchId,
    start_position: Point<Pixels>,
    last_position: Point<Pixels>,
    started_at: Instant,
    is_panning: bool,
}

struct CompletedTap {
    position: Point<Pixels>,
    completed_at: Instant,
    count: usize,
}

impl TouchGestureArena {
    pub(crate) fn new(tuning: GestureTuning) -> Self {
        Self {
            tuning,
            active_touch: None,
            last_tap: None,
        }
    }

    #[cfg(test)]
    fn handle(&mut self, event: &TouchEvent) -> SmallVec<[TouchGestureOutput; 1]> {
        self.handle_at(event, Instant::now())
    }

    pub(crate) fn handle_at(
        &mut self,
        event: &TouchEvent,
        now: Instant,
    ) -> SmallVec<[TouchGestureOutput; 1]> {
        match event.phase {
            TouchPhase::Started => {
                if self.active_touch.is_none() {
                    self.active_touch = Some(ActiveTouch {
                        id: event.id,
                        start_position: event.position,
                        last_position: event.position,
                        started_at: now,
                        is_panning: false,
                    });
                }
                SmallVec::new()
            }
            TouchPhase::Moved => {
                let Some(active_touch) = self
                    .active_touch
                    .as_mut()
                    .filter(|active_touch| active_touch.id == event.id)
                else {
                    return SmallVec::new();
                };

                let mut touch_phase = TouchPhase::Moved;
                let delta = if active_touch.is_panning {
                    event.position - active_touch.last_position
                } else if (event.position - active_touch.start_position).magnitude()
                    > self.tuning.touch_slop.into()
                {
                    active_touch.is_panning = true;
                    self.last_tap = None;
                    touch_phase = TouchPhase::Started;
                    event.position - active_touch.start_position
                } else {
                    active_touch.last_position = event.position;
                    return SmallVec::new();
                };
                active_touch.last_position = event.position;

                smallvec![TouchGestureOutput::PlatformInput(
                    PlatformInput::ScrollWheel(ScrollWheelEvent {
                        position: event.position,
                        delta: ScrollDelta::Pixels(delta),
                        modifiers: Modifiers::default(),
                        touch_phase,
                    })
                )]
            }
            TouchPhase::Ended | TouchPhase::Cancelled => {
                if self
                    .active_touch
                    .as_ref()
                    .is_none_or(|active_touch| active_touch.id != event.id)
                {
                    return SmallVec::new();
                }
                let Some(active_touch) = self.active_touch.take() else {
                    return SmallVec::new();
                };

                if active_touch.is_panning {
                    return smallvec![TouchGestureOutput::PlatformInput(
                        PlatformInput::ScrollWheel(ScrollWheelEvent {
                            position: event.position,
                            delta: ScrollDelta::Pixels(event.position - active_touch.last_position),
                            modifiers: Modifiers::default(),
                            touch_phase: event.phase,
                        })
                    )];
                }

                if event.phase == TouchPhase::Cancelled {
                    self.last_tap = None;
                    return SmallVec::new();
                }

                let long_press =
                    now.duration_since(active_touch.started_at) >= self.tuning.long_press_duration;
                let tap_count = if long_press {
                    self.last_tap = None;
                    1
                } else {
                    let tap_count = self
                        .last_tap
                        .as_ref()
                        .filter(|last_tap| {
                            now.checked_duration_since(last_tap.completed_at)
                                .is_some_and(|elapsed| elapsed <= self.tuning.multi_tap_interval)
                                && (active_touch.start_position - last_tap.position).magnitude()
                                    <= self.tuning.multi_tap_slop.into()
                        })
                        .map_or(1, |last_tap| last_tap.count + 1);
                    self.last_tap = Some(CompletedTap {
                        position: active_touch.start_position,
                        completed_at: now,
                        count: tap_count,
                    });
                    tap_count
                };
                smallvec![TouchGestureOutput::Click(TouchClickEvent {
                    position: active_touch.start_position,
                    tap_count,
                    long_press,
                })]
            }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::point;

    #[test]
    fn touch_gesture_arena_recognizes_tap() {
        let mut arena = TouchGestureArena::new(GestureTuning::default());
        let position = point(px(10.), px(20.));
        let started = TouchEvent {
            id: TouchId(1),
            phase: TouchPhase::Started,
            position,
            force: None,
        };
        assert!(arena.handle(&started).is_empty());

        let ended = TouchEvent {
            phase: TouchPhase::Ended,
            ..started
        };
        let output = arena.handle(&ended);
        let Some(TouchGestureOutput::Click(click)) = output.first() else {
            panic!("tap should produce a touch click");
        };
        assert_eq!(click.position, position);
        assert_eq!(click.tap_count, 1);
        assert!(!click.long_press);
    }

    #[test]
    fn touch_gesture_arena_counts_nearby_consecutive_taps() {
        let mut arena = TouchGestureArena::new(GestureTuning::default());
        let first_started_at = Instant::now();
        let first = TouchEvent {
            id: TouchId(1),
            phase: TouchPhase::Started,
            position: point(px(10.), px(20.)),
            force: None,
        };
        arena.handle_at(&first, first_started_at);
        arena.handle_at(
            &TouchEvent {
                phase: TouchPhase::Ended,
                ..first
            },
            first_started_at + Duration::from_millis(50),
        );

        let second_started_at = first_started_at + Duration::from_millis(150);
        let second = TouchEvent {
            id: TouchId(2),
            phase: TouchPhase::Started,
            position: point(px(12.), px(22.)),
            force: None,
        };
        arena.handle_at(&second, second_started_at);
        let output = arena.handle_at(
            &TouchEvent {
                phase: TouchPhase::Ended,
                ..second
            },
            second_started_at + Duration::from_millis(50),
        );

        let Some(TouchGestureOutput::Click(click)) = output.first() else {
            panic!("second tap should produce a touch click");
        };
        assert_eq!(click.tap_count, 2);
        assert!(!click.long_press);
    }

    #[test]
    fn touch_gesture_arena_recognizes_long_press() {
        let mut arena = TouchGestureArena::new(GestureTuning::default());
        let started_at = Instant::now();
        let started = TouchEvent {
            id: TouchId(1),
            phase: TouchPhase::Started,
            position: point(px(10.), px(20.)),
            force: None,
        };
        arena.handle_at(&started, started_at);
        let output = arena.handle_at(
            &TouchEvent {
                phase: TouchPhase::Ended,
                ..started
            },
            started_at + GestureTuning::default().long_press_duration,
        );

        let Some(TouchGestureOutput::Click(click)) = output.first() else {
            panic!("long press should produce a touch click");
        };
        assert_eq!(click.tap_count, 1);
        assert!(click.long_press);
    }

    #[test]
    fn touch_gesture_arena_promotes_movement_to_scroll() {
        let mut arena = TouchGestureArena::new(GestureTuning::default());
        let started = TouchEvent {
            id: TouchId(1),
            phase: TouchPhase::Started,
            position: point(px(10.), px(20.)),
            force: None,
        };
        arena.handle(&started);

        let moved = TouchEvent {
            phase: TouchPhase::Moved,
            position: point(px(10.), px(40.)),
            ..started.clone()
        };
        let output = arena.handle(&moved);
        let Some(TouchGestureOutput::PlatformInput(PlatformInput::ScrollWheel(scroll))) =
            output.first()
        else {
            panic!("pan should produce a scroll event");
        };
        assert_eq!(scroll.touch_phase, TouchPhase::Started);
        let ScrollDelta::Pixels(delta) = scroll.delta else {
            panic!("touch pan should scroll in pixels");
        };
        assert_eq!(delta, point(px(0.), px(20.)));

        let ended = TouchEvent {
            phase: TouchPhase::Ended,
            ..moved
        };
        let output = arena.handle(&ended);
        let Some(TouchGestureOutput::PlatformInput(PlatformInput::ScrollWheel(scroll))) =
            output.first()
        else {
            panic!("ending a pan should end the scroll");
        };
        assert_eq!(scroll.touch_phase, TouchPhase::Ended);
    }

    #[test]
    fn secondary_touch_does_not_cancel_primary_touch() {
        let mut arena = TouchGestureArena::new(GestureTuning::default());
        let primary = TouchEvent {
            id: TouchId(1),
            phase: TouchPhase::Started,
            position: point(px(10.), px(20.)),
            force: None,
        };
        arena.handle(&primary);

        let secondary_ended = TouchEvent {
            id: TouchId(2),
            phase: TouchPhase::Ended,
            ..primary.clone()
        };
        assert!(arena.handle(&secondary_ended).is_empty());

        let primary_ended = TouchEvent {
            phase: TouchPhase::Ended,
            ..primary
        };
        assert!(matches!(
            arena.handle(&primary_ended).first(),
            Some(TouchGestureOutput::Click(_))
        ));
    }

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
}
