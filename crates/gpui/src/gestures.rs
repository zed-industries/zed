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
    Axis, GestureEvent, InputEvent, IsZero, Modifiers, MouseButton, MouseDownEvent, MouseEvent,
    MouseUpEvent, Pixels, PlatformInput, Point, ScrollDelta, ScrollWheelEvent, TouchEvent, TouchId,
    TouchPhase, point, px, seal::Sealed,
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
    /// How scroll momentum decelerates after a fling.
    pub scroll_physics: ScrollPhysics,
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
            scroll_physics: ScrollPhysics::ios(),
            min_fling_velocity: 50.,
        }
    }
}

/// How free scrolling decelerates after a fling.
///
/// This models deceleration only. Boundary behavior — bouncing, edge glow,
/// clamping — is the scroll container's policy: the container is the one that
/// knows its extents.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ScrollPhysics {
    /// Exponential velocity decay, the `UIScrollView` model:
    /// `velocity(t) = v₀ · decay_per_msᵐˢ`.
    Exponential {
        /// Per-millisecond velocity decay factor. `UIScrollView`'s normal
        /// deceleration rate is `0.998`.
        decay_per_ms: f32,
    },
    /// The friction spline of Android's `OverScroller`: fling duration and
    /// distance follow a logarithmic deceleration law, and progress along
    /// the fling follows a cubic-Bezier ease-out curve. Transcribed from
    /// AOSP's `SplineOverScroller` (Apache-2.0).
    FrictionSpline {
        /// The scroll friction coefficient;
        /// `ViewConfiguration.getScrollFriction()` is `0.015` on Android.
        friction: f32,
        /// Pixels per physical inch of the display, in the coordinate space
        /// the fling runs in. Android folds display density into its
        /// deceleration coefficient, so the same finger speed flings
        /// further in pixels on a denser screen.
        pixels_per_inch: f32,
    },
}

impl ScrollPhysics {
    /// iOS scroll feel: `UIScrollView`'s normal deceleration rate.
    pub fn ios() -> Self {
        Self::Exponential {
            decay_per_ms: 0.998,
        }
    }

    /// Android scroll feel: `OverScroller` with stock friction, at Android's
    /// nominal density of 160 density-independent pixels per inch — the
    /// right pairing when fling distances are in logical pixels. Platforms
    /// that fling in physical pixels, or know the display's true density in
    /// their logical space, should construct
    /// [`ScrollPhysics::FrictionSpline`] directly.
    pub fn android() -> Self {
        Self::FrictionSpline {
            friction: 0.015,
            pixels_per_inch: 160.,
        }
    }

    /// How long a fling released at `speed` pixels per second coasts before
    /// it stops.
    fn fling_duration(self, speed: f32) -> Duration {
        match self {
            Self::Exponential { decay_per_ms } => {
                if speed <= MOMENTUM_STOP_VELOCITY {
                    return Duration::ZERO;
                }
                let milliseconds = (MOMENTUM_STOP_VELOCITY / speed).ln() / decay_per_ms.ln();
                Duration::from_secs_f32(milliseconds / 1000.)
            }
            Self::FrictionSpline {
                friction,
                pixels_per_inch,
            } => {
                if speed <= 0. {
                    return Duration::ZERO;
                }
                let deceleration = friction_spline::deceleration(speed, friction, pixels_per_inch);
                let seconds = (deceleration / (friction_spline::deceleration_rate() - 1.)).exp();
                Duration::from_secs_f64(seconds)
            }
        }
    }

    /// Distance traveled `elapsed` into a fling released at `speed` pixels
    /// per second, in pixels along the fling direction. Evaluated in closed
    /// form so the trajectory is independent of tick timing.
    fn fling_distance(self, speed: f32, elapsed: Duration) -> f32 {
        let duration = self.fling_duration(speed);
        if duration.is_zero() {
            return 0.;
        }
        let elapsed = elapsed.min(duration);
        match self {
            Self::Exponential { decay_per_ms } => {
                // ∫₀ᵗ v₀·kᵐˢ dms, with speed converted to pixels per
                // millisecond.
                let milliseconds = elapsed.as_secs_f32() * 1000.;
                (speed / 1000.) * (decay_per_ms.powf(milliseconds) - 1.) / decay_per_ms.ln()
            }
            Self::FrictionSpline {
                friction,
                pixels_per_inch,
            } => {
                let deceleration = friction_spline::deceleration(speed, friction, pixels_per_inch);
                let rate = friction_spline::deceleration_rate();
                let total_distance = friction as f64
                    * friction_spline::physical_coefficient(pixels_per_inch)
                    * (rate / (rate - 1.) * deceleration).exp();
                let progress = elapsed.as_secs_f64() / duration.as_secs_f64();
                total_distance as f32 * friction_spline::distance_coefficient(progress as f32)
            }
        }
    }
}

/// The fling model of Android's `OverScroller.SplineOverScroller`,
/// transcribed from AOSP (Apache-2.0). `SPLINE_TIME`, which AOSP uses for
/// programmatic scroll animations rather than flings, is intentionally not
/// transcribed.
mod friction_spline {
    use std::sync::LazyLock;

    const NB_SAMPLES: usize = 100;
    const INFLEXION: f32 = 0.35;
    const START_TENSION: f32 = 0.5;
    const END_TENSION: f32 = 1.0;
    const P1: f32 = START_TENSION * INFLEXION;
    const P2: f32 = 1.0 - END_TENSION * (1.0 - INFLEXION);

    /// Android's `DECELERATION_RATE`: `ln(0.78) / ln(0.9)`.
    pub(super) fn deceleration_rate() -> f64 {
        0.78f64.ln() / 0.9f64.ln()
    }

    /// `SPLINE_POSITION` from AOSP's static initializer: fractional fling
    /// distance sampled at 100 evenly spaced fractions of the fling
    /// duration, from a cubic Bezier with control points shaped by
    /// `INFLEXION` and the start/end tensions.
    static SPLINE_POSITION: LazyLock<[f32; NB_SAMPLES + 1]> = LazyLock::new(|| {
        let mut spline_position = [0f32; NB_SAMPLES + 1];
        let mut x_min = 0f32;
        for (i, sample) in spline_position.iter_mut().take(NB_SAMPLES).enumerate() {
            let alpha = i as f32 / NB_SAMPLES as f32;
            let mut x_max = 1f32;
            let (x, coefficient) = loop {
                let x = x_min + (x_max - x_min) / 2.;
                let coefficient = 3. * x * (1. - x);
                let time = coefficient * ((1. - x) * P1 + x * P2) + x * x * x;
                if (time - alpha).abs() < 1e-5 {
                    break (x, coefficient);
                }
                if time > alpha {
                    x_max = x;
                } else {
                    x_min = x;
                }
            };
            *sample = coefficient * ((1. - x) * START_TENSION + x) + x * x * x;
        }
        spline_position[NB_SAMPLES] = 1.;
        spline_position
    });

    /// `SensorManager.GRAVITY_EARTH · 39.37 in/m · ppi · 0.84`, AOSP's
    /// `mPhysicalCoeff`: gravity expressed in pixels, times an empirical
    /// "look and feel" tuning factor.
    pub(super) fn physical_coefficient(pixels_per_inch: f32) -> f64 {
        9.80665 * 39.37 * pixels_per_inch as f64 * 0.84
    }

    /// AOSP's `getSplineDeceleration`.
    pub(super) fn deceleration(speed: f32, friction: f32, pixels_per_inch: f32) -> f64 {
        (INFLEXION as f64 * speed as f64
            / (friction as f64 * physical_coefficient(pixels_per_inch)))
        .ln()
    }

    /// Fraction of the total fling distance covered at fraction `time` of
    /// the fling duration: table lookup plus linear interpolation, as in
    /// `SplineOverScroller.update`.
    pub(super) fn distance_coefficient(time: f32) -> f32 {
        if time >= 1. {
            return 1.;
        }
        let index = ((NB_SAMPLES as f32 * time) as usize).min(NB_SAMPLES - 1);
        let time_lower = index as f32 / NB_SAMPLES as f32;
        let time_upper = (index + 1) as f32 / NB_SAMPLES as f32;
        let distance_lower = SPLINE_POSITION[index];
        let distance_upper = SPLINE_POSITION[index + 1];
        let velocity_coefficient = (distance_upper - distance_lower) / (time_upper - time_lower);
        distance_lower + (time - time_lower) * velocity_coefficient
    }

    #[cfg(test)]
    pub(super) fn bezier_time_and_position(parameter: f32) -> (f32, f32) {
        let coefficient = 3. * parameter * (1. - parameter);
        let cubed = parameter * parameter * parameter;
        (
            coefficient * ((1. - parameter) * P1 + parameter * P2) + cubed,
            coefficient * ((1. - parameter) * START_TENSION + parameter) + cubed,
        )
    }

    #[cfg(test)]
    pub(super) fn spline_position_samples() -> &'static [f32; NB_SAMPLES + 1] {
        &SPLINE_POSITION
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

/// A phased long-press gesture recognized from a touch.
#[derive(Clone, Debug)]
pub struct LongPressEvent {
    /// The phase of the long press.
    pub phase: TouchPhase,
    /// The position where the touch started.
    pub start_position: Point<Pixels>,
    /// The touch's current position.
    pub position: Point<Pixels>,
}

impl Default for LongPressEvent {
    fn default() -> Self {
        Self {
            phase: TouchPhase::Started,
            start_position: Point::default(),
            position: Point::default(),
        }
    }
}

impl Sealed for LongPressEvent {}
impl InputEvent for LongPressEvent {
    fn to_platform_input(self) -> PlatformInput {
        PlatformInput::LongPress(self)
    }
}
impl GestureEvent for LongPressEvent {}
impl MouseEvent for LongPressEvent {}

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

/// Momentum below this speed, in pixels per second, is imperceptible. The
/// exponential model, which never mathematically stops, treats reaching this
/// speed as the end of the fling. (The friction spline has a finite duration
/// of its own.)
const MOMENTUM_STOP_VELOCITY: f32 = 10.;

/// How far back the release-velocity estimate looks. Samples older than this
/// reflect an earlier part of the gesture, not the speed at release.
const VELOCITY_WINDOW: Duration = Duration::from_millis(100);

/// A pause between samples longer than this means the finger stopped:
/// anything before the pause describes an earlier motion, not the release
/// (Flutter's `kAssumePointerMoveStoppedMilliseconds`). Touch hardware
/// reports movement every 8–16ms while the finger is in motion.
const VELOCITY_ASSUME_STOPPED_GAP: Duration = Duration::from_millis(40);

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
/// Pinch recognition is not implemented yet, and additional touches are ignored
/// while one is being recognized.
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
    LongPress(LongPressEvent),
}

enum TouchGestureState {
    Idle,
    /// The touch is still within `touch_slop` of where it started: it can
    /// still resolve into either a tap or a pan.
    Pending {
        touch: ActiveTouch,
        deadline: Instant,
        offered: bool,
    },
    /// The touch exceeded `touch_slop`: it is a pan until it ends, and its
    /// movement flows out as scroll events.
    Panning(ActiveTouch),
    LongPressing(ActiveTouch),
}

struct ActiveTouch {
    id: TouchId,
    start_position: Point<Pixels>,
    /// The latest raw position reported for this touch.
    last_position: Point<Pixels>,
    /// The position pan output has scrolled to so far. While panning this
    /// may run ahead of the raw touch by the event's predicted position;
    /// the release event targets the raw position again, so the total
    /// scrolled distance always converges to the finger's actual travel.
    emitted_position: Point<Pixels>,
    velocity_tracker: VelocityTracker,
}

struct CompletedTap {
    position: Point<Pixels>,
    time: Instant,
    count: usize,
}

/// One fling in progress. The trajectory is a closed-form curve of elapsed
/// time — each tick evaluates it and emits the increment — so the fling is
/// exactly frame-rate independent: a stalled frame simply resumes further
/// along the same curve.
struct Momentum {
    /// Where the pan started; synthesized scroll events keep hit-testing
    /// there so momentum stays with the container the gesture began on.
    position: Point<Pixels>,
    /// Unit vector of the release velocity.
    direction: Point<f32>,
    /// Release speed in pixels per second.
    speed: f32,
    started_at: Instant,
    duration: Duration,
    /// Distance already emitted along `direction`, in pixels.
    emitted_distance: f32,
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
                let caught_fling = if let Some(momentum) = self.momentum.take() {
                    recognized.push(RecognizedTouchGesture::Scroll(scroll_event(
                        momentum.position,
                        Point::default(),
                        TouchPhase::Ended,
                    )));
                    true
                } else {
                    false
                };
                if matches!(self.state, TouchGestureState::Idle) {
                    let mut velocity_tracker = VelocityTracker::default();
                    velocity_tracker.push(now, event.position);
                    let touch = ActiveTouch {
                        id: event.id,
                        start_position: event.position,
                        last_position: event.position,
                        emitted_position: event.position,
                        velocity_tracker,
                    };
                    if caught_fling {
                        // A touch that catches a fling is a drag from the
                        // first pixel: waiting out the slop would freeze the
                        // content mid-scroll and then jump. It can also never
                        // be a tap; releasing it just leaves the content
                        // stopped, as on Android and iOS.
                        recognized.push(RecognizedTouchGesture::Scroll(scroll_event(
                            touch.start_position,
                            Point::default(),
                            TouchPhase::Started,
                        )));
                        self.state = TouchGestureState::Panning(touch);
                    } else {
                        self.state = TouchGestureState::Pending {
                            touch,
                            deadline: now + self.tuning.long_press_duration,
                            offered: false,
                        };
                    }
                }
            }
            TouchPhase::Moved => match mem::replace(&mut self.state, TouchGestureState::Idle) {
                TouchGestureState::Pending {
                    mut touch,
                    deadline,
                    offered,
                } if touch.id == event.id => {
                    touch.velocity_tracker.push(now, event.position);
                    touch.last_position = event.position;
                    let accumulated = event.position - touch.start_position;
                    if accumulated.magnitude() > f64::from(self.tuning.touch_slop) {
                        // Carry the full movement so far into the first scroll
                        // step: the content catches up to the finger instead
                        // of losing the slop distance.
                        let target = event.predicted_position.unwrap_or(event.position);
                        touch.emitted_position = target;
                        recognized.push(RecognizedTouchGesture::Scroll(scroll_event(
                            touch.start_position,
                            target - touch.start_position,
                            TouchPhase::Started,
                        )));
                        self.state = TouchGestureState::Panning(touch);
                    } else {
                        self.state = TouchGestureState::Pending {
                            touch,
                            deadline,
                            offered,
                        };
                    }
                }
                TouchGestureState::Panning(mut touch) if touch.id == event.id => {
                    touch.velocity_tracker.push(now, event.position);
                    touch.last_position = event.position;
                    let target = event.predicted_position.unwrap_or(event.position);
                    let delta = target - touch.emitted_position;
                    touch.emitted_position = target;
                    recognized.push(RecognizedTouchGesture::Scroll(scroll_event(
                        touch.start_position,
                        delta,
                        TouchPhase::Moved,
                    )));
                    self.state = TouchGestureState::Panning(touch);
                }
                TouchGestureState::LongPressing(mut touch) if touch.id == event.id => {
                    touch.last_position = event.position;
                    recognized.push(RecognizedTouchGesture::LongPress(LongPressEvent {
                        phase: TouchPhase::Moved,
                        start_position: touch.start_position,
                        position: event.position,
                    }));
                    self.state = TouchGestureState::LongPressing(touch);
                }
                other => self.state = other,
            },
            TouchPhase::Ended => match mem::replace(&mut self.state, TouchGestureState::Idle) {
                TouchGestureState::Pending { touch, .. } if touch.id == event.id => {
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
                TouchGestureState::Panning(touch) if touch.id == event.id => {
                    // The release deliberately contributes no velocity
                    // sample: it usually repeats the last movement's position
                    // with a later timestamp, which would dilute the
                    // estimate. But a release long after the last movement
                    // means the finger had already stopped, so nothing
                    // flings.
                    let finger_stopped =
                        touch
                            .velocity_tracker
                            .latest_sample_time()
                            .is_none_or(|latest| {
                                now.duration_since(latest) > VELOCITY_ASSUME_STOPPED_GAP
                            });
                    let velocity = if finger_stopped {
                        Point::default()
                    } else {
                        touch.velocity_tracker.velocity()
                    };
                    let speed = (velocity.x.powi(2) + velocity.y.powi(2)).sqrt();
                    let mut release_delta = event.position - touch.emitted_position;
                    if speed >= self.tuning.min_fling_velocity {
                        let direction = point(velocity.x / speed, velocity.y / speed);
                        let speed = speed.min(MAX_FLING_VELOCITY);
                        let duration = self.tuning.scroll_physics.fling_duration(speed);
                        if !duration.is_zero() {
                            let total_distance =
                                self.tuning.scroll_physics.fling_distance(speed, duration);
                            // Prediction may have left the content ahead of
                            // the raw release position. Emitting that
                            // correction here would visibly snap the content
                            // backwards just as the fling launches, so fold
                            // it into the fling instead: start the curve
                            // already advanced by the overshoot, keeping the
                            // total travel exact while staying monotonic.
                            let overshoot = -(f32::from(release_delta.x) * direction.x
                                + f32::from(release_delta.y) * direction.y);
                            let emitted_distance = if overshoot > 0. && overshoot < total_distance {
                                release_delta +=
                                    point(px(direction.x * overshoot), px(direction.y * overshoot));
                                overshoot
                            } else {
                                0.
                            };
                            self.momentum = Some(Momentum {
                                position: touch.start_position,
                                direction,
                                speed,
                                started_at: now,
                                duration,
                                emitted_distance,
                            });
                        }
                    }
                    recognized.push(RecognizedTouchGesture::Scroll(scroll_event(
                        touch.start_position,
                        release_delta,
                        TouchPhase::Ended,
                    )));
                }
                TouchGestureState::LongPressing(touch) if touch.id == event.id => {
                    recognized.push(RecognizedTouchGesture::LongPress(LongPressEvent {
                        phase: TouchPhase::Ended,
                        start_position: touch.start_position,
                        position: event.position,
                    }));
                }
                other => self.state = other,
            },
            TouchPhase::Cancelled => match mem::replace(&mut self.state, TouchGestureState::Idle) {
                TouchGestureState::Pending { touch, .. } if touch.id == event.id => {}
                TouchGestureState::Panning(touch) if touch.id == event.id => {
                    recognized.push(RecognizedTouchGesture::Scroll(scroll_event(
                        touch.start_position,
                        Point::default(),
                        TouchPhase::Cancelled,
                    )));
                }
                TouchGestureState::LongPressing(touch) if touch.id == event.id => {
                    recognized.push(RecognizedTouchGesture::LongPress(LongPressEvent {
                        phase: TouchPhase::Cancelled,
                        start_position: touch.start_position,
                        position: event.position,
                    }));
                }
                other => self.state = other,
            },
        }
        recognized
    }

    pub(crate) fn pending_long_press(&self) -> Option<(TouchId, Duration)> {
        let TouchGestureState::Pending {
            touch,
            deadline,
            offered: false,
        } = &self.state
        else {
            return None;
        };
        Some((touch.id, deadline.saturating_duration_since(Instant::now())))
    }

    pub(crate) fn offer_long_press(&mut self, id: TouchId) -> Option<RecognizedTouchGesture> {
        let TouchGestureState::Pending { touch, offered, .. } = &mut self.state else {
            return None;
        };
        if touch.id != id || *offered {
            return None;
        }
        *offered = true;
        Some(RecognizedTouchGesture::LongPress(LongPressEvent {
            phase: TouchPhase::Started,
            start_position: touch.start_position,
            position: touch.last_position,
        }))
    }

    pub(crate) fn resolve_long_press(&mut self, claimed: bool) {
        if !claimed {
            return;
        }
        let state = mem::replace(&mut self.state, TouchGestureState::Idle);
        self.state = match state {
            TouchGestureState::Pending {
                touch,
                offered: true,
                ..
            } => TouchGestureState::LongPressing(touch),
            other => other,
        };
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
        let elapsed = now.duration_since(momentum.started_at);
        let distance = self
            .tuning
            .scroll_physics
            .fling_distance(momentum.speed, elapsed);
        let step = distance - momentum.emitted_distance;
        momentum.emitted_distance = distance;
        let delta = point(
            px(momentum.direction.x * step),
            px(momentum.direction.y * step),
        );
        let position = momentum.position;
        if elapsed >= momentum.duration {
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

/// Estimates the velocity a touch had at its newest sample.
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
    }

    fn latest_sample_time(&self) -> Option<Instant> {
        self.samples.back().map(|(time, _)| *time)
    }

    /// The velocity at the newest sample, in pixels per second.
    ///
    /// Fits a second-degree polynomial by least squares over the trailing
    /// [`VELOCITY_WINDOW`] and takes its derivative at the newest sample,
    /// like Flutter's `VelocityTracker` and Android's `lsq2` strategy. An
    /// endpoint difference over the same window would report the window's
    /// *average* speed, which for a flick — still accelerating at lift-off —
    /// is roughly half the speed the finger actually had at release.
    fn velocity(&self) -> Point<f32> {
        let Some((newest_time, _)) = self.samples.back() else {
            return Point::default();
        };
        let mut times_seconds: SmallVec<[f64; VELOCITY_MAX_SAMPLES]> = SmallVec::new();
        let mut horizontal: SmallVec<[f64; VELOCITY_MAX_SAMPLES]> = SmallVec::new();
        let mut vertical: SmallVec<[f64; VELOCITY_MAX_SAMPLES]> = SmallVec::new();
        let mut previous_time = *newest_time;
        for (time, position) in self.samples.iter().rev() {
            let age = newest_time.duration_since(*time);
            if age > VELOCITY_WINDOW
                || previous_time.duration_since(*time) > VELOCITY_ASSUME_STOPPED_GAP
            {
                break;
            }
            previous_time = *time;
            times_seconds.push(-age.as_secs_f64());
            horizontal.push(f64::from(f32::from(position.x)));
            vertical.push(f64::from(f32::from(position.y)));
        }

        let endpoint_estimate = |values: &[f64]| -> f32 {
            let elapsed = -times_seconds.last().copied().unwrap_or(0.);
            if elapsed <= f64::EPSILON {
                return 0.;
            }
            ((values.first().copied().unwrap_or(0.) - values.last().copied().unwrap_or(0.))
                / elapsed) as f32
        };
        if times_seconds.len() < 3 {
            return point(endpoint_estimate(&horizontal), endpoint_estimate(&vertical));
        }
        point(
            quadratic_velocity_at_newest(&times_seconds, &horizontal).map_or_else(
                || endpoint_estimate(&horizontal),
                |velocity| velocity as f32,
            ),
            quadratic_velocity_at_newest(&times_seconds, &vertical)
                .map_or_else(|| endpoint_estimate(&vertical), |velocity| velocity as f32),
        )
    }
}

/// Least-squares fit of `value = a0 + a1·t + a2·t²` returning `a1`: the
/// fitted curve's velocity at `t = 0`, which callers place at the newest
/// sample. `None` when the samples are too degenerate to fit (all
/// simultaneous, for example).
fn quadratic_velocity_at_newest(times: &[f64], values: &[f64]) -> Option<f64> {
    let count = times.len() as f64;
    let (mut sum_t1, mut sum_t2, mut sum_t3, mut sum_t4) = (0., 0., 0., 0.);
    let (mut sum_v, mut sum_vt, mut sum_vt2) = (0., 0., 0.);
    for (&time, &value) in times.iter().zip(values) {
        let time_squared = time * time;
        sum_t1 += time;
        sum_t2 += time_squared;
        sum_t3 += time_squared * time;
        sum_t4 += time_squared * time_squared;
        sum_v += value;
        sum_vt += value * time;
        sum_vt2 += value * time_squared;
    }
    // Cramer's rule on the 3×3 normal equations, solved for the linear
    // coefficient only.
    let determinant = count * (sum_t2 * sum_t4 - sum_t3 * sum_t3)
        - sum_t1 * (sum_t1 * sum_t4 - sum_t3 * sum_t2)
        + sum_t2 * (sum_t1 * sum_t3 - sum_t2 * sum_t2);
    if determinant.abs() < 1e-12 {
        return None;
    }
    let linear_determinant = count * (sum_vt * sum_t4 - sum_t3 * sum_vt2)
        - sum_v * (sum_t1 * sum_t4 - sum_t3 * sum_t2)
        + sum_t2 * (sum_t1 * sum_vt2 - sum_vt * sum_t2);
    Some(linear_determinant / determinant)
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
    fn predicted_positions_lead_the_pan_but_totals_converge_on_release() {
        let mut recognizer = TouchGestureRecognizer::new(GestureTuning::default());
        let now = Instant::now();
        let touch = TouchId(1);

        recognizer.handle_event_at(&touch_event(touch, TouchPhase::Started, 100., 100.), now);

        // The first pan step scrolls to the predicted position, not the raw one.
        let mut moved = touch_event(touch, TouchPhase::Moved, 100., 120.);
        moved.predicted_position = Some(point(px(100.), px(128.)));
        let recognized = recognizer.handle_event_at(&moved, now + Duration::from_millis(16));
        let [RecognizedTouchGesture::Scroll(scroll)] = recognized.as_slice() else {
            panic!("expected scroll, got {recognized:?}");
        };
        assert_eq!(scroll.delta.pixel_delta(px(16.)), point(px(0.), px(28.)));

        // The next step is measured from where the previous prediction left
        // the content, so an overshoot is paid back here.
        let mut moved = touch_event(touch, TouchPhase::Moved, 100., 130.);
        moved.predicted_position = Some(point(px(100.), px(134.)));
        let recognized = recognizer.handle_event_at(&moved, now + Duration::from_millis(32));
        let [RecognizedTouchGesture::Scroll(scroll)] = recognized.as_slice() else {
            panic!("expected scroll, got {recognized:?}");
        };
        assert_eq!(scroll.delta.pixel_delta(px(16.)), point(px(0.), px(6.)));

        // A release without a fling (the finger stopped long before lifting)
        // targets the raw position: the total scrolled distance equals the
        // finger's actual travel despite the predictions.
        let recognized = recognizer.handle_event_at(
            &touch_event(touch, TouchPhase::Ended, 100., 130.),
            now + Duration::from_millis(120),
        );
        let [RecognizedTouchGesture::Scroll(scroll)] = recognized.as_slice() else {
            panic!("expected scroll, got {recognized:?}");
        };
        assert_eq!(scroll.touch_phase, TouchPhase::Ended);
        assert!(!recognizer.has_momentum());
        assert_eq!(scroll.delta.pixel_delta(px(16.)), point(px(0.), px(-4.)));
    }

    #[test]
    fn predicted_overshoot_folds_into_the_fling_without_scrolling_backwards() {
        let now = Instant::now();
        let mut total_with_prediction = 0f32;
        let mut total_without_prediction = 0f32;
        for use_prediction in [true, false] {
            let mut recognizer = TouchGestureRecognizer::new(GestureTuning::default());
            let mut total = 0f32;
            let mut drain = |recognized: &[RecognizedTouchGesture], upward_only: bool| {
                for gesture in recognized {
                    let RecognizedTouchGesture::Scroll(scroll) = gesture else {
                        panic!("expected scroll, got {gesture:?}");
                    };
                    let delta = scroll.delta.pixel_delta(px(16.)).y;
                    if upward_only {
                        assert!(
                            delta <= px(0.),
                            "content moved backwards by {delta:?} during an upward gesture"
                        );
                    }
                    total += f32::from(delta);
                }
            };

            recognizer.handle_event_at(
                &touch_event(TouchId(1), TouchPhase::Started, 100., 500.),
                now,
            );
            for step in 1..=5u64 {
                let raw_y = 500. - step as f32 * 40.;
                let mut moved = touch_event(TouchId(1), TouchPhase::Moved, 100., raw_y);
                if use_prediction {
                    moved.predicted_position = Some(point(px(100.), px(raw_y - 25.)));
                }
                let recognized =
                    recognizer.handle_event_at(&moved, now + Duration::from_millis(step * 16));
                drain(&recognized, use_prediction);
            }
            // The release leaves the emitted position 25px ahead of the raw
            // one; with prediction the correction must not scroll backwards.
            let recognized = recognizer.handle_event_at(
                &touch_event(TouchId(1), TouchPhase::Ended, 100., 300.),
                now + Duration::from_millis(90),
            );
            drain(&recognized, use_prediction);
            assert!(recognizer.has_momentum());
            let mut tick = now + Duration::from_millis(90);
            while recognizer.has_momentum() {
                tick += Duration::from_millis(16);
                if let Some(gesture) = recognizer.tick_momentum_at(tick) {
                    drain(&[gesture], use_prediction);
                }
            }

            if use_prediction {
                total_with_prediction = total;
            } else {
                total_without_prediction = total;
            }
        }
        // Folding the overshoot into the fling redistributes the travel but
        // must not change where the content comes to rest.
        assert!(
            (total_with_prediction - total_without_prediction).abs() < 0.01,
            "totals diverged: {total_with_prediction} vs {total_without_prediction}"
        );
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
        // The least-squares fit may leave float residue on the motionless axis.
        assert!(
            delta.x.abs() < px(0.001),
            "expected no x motion, got {delta:?}"
        );

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
        for step in 1..=3 {
            recognizer.handle_event_at(
                &touch_event(
                    TouchId(1),
                    TouchPhase::Moved,
                    100.,
                    300. - step as f32 * 33.,
                ),
                now + Duration::from_millis(step * 16),
            );
        }
        recognizer.handle_event_at(
            &touch_event(TouchId(1), TouchPhase::Ended, 100., 200.),
            now + Duration::from_millis(64),
        );
        assert!(recognizer.has_momentum());

        let recognized = recognizer.handle_event_at(
            &touch_event(TouchId(2), TouchPhase::Started, 100., 200.),
            now + Duration::from_millis(200),
        );
        assert!(!recognizer.has_momentum());
        let [
            RecognizedTouchGesture::Scroll(closing),
            RecognizedTouchGesture::Scroll(opening),
        ] = recognized.as_slice()
        else {
            panic!("expected closing and opening scrolls, got {recognized:?}");
        };
        assert_eq!(closing.touch_phase, TouchPhase::Ended);
        assert!(closing.delta.pixel_delta(px(16.)).is_zero());
        assert_eq!(opening.touch_phase, TouchPhase::Started);
        assert!(opening.delta.pixel_delta(px(16.)).is_zero());
    }

    #[test]
    fn catching_a_fling_pans_immediately_and_never_taps() {
        let mut recognizer = TouchGestureRecognizer::new(GestureTuning::default());
        let now = Instant::now();

        recognizer.handle_event_at(
            &touch_event(TouchId(1), TouchPhase::Started, 100., 300.),
            now,
        );
        for step in 1..=3 {
            recognizer.handle_event_at(
                &touch_event(
                    TouchId(1),
                    TouchPhase::Moved,
                    100.,
                    300. - step as f32 * 33.,
                ),
                now + Duration::from_millis(step * 16),
            );
        }
        recognizer.handle_event_at(
            &touch_event(TouchId(1), TouchPhase::Ended, 100., 200.),
            now + Duration::from_millis(64),
        );
        assert!(recognizer.has_momentum());

        recognizer.handle_event_at(
            &touch_event(TouchId(2), TouchPhase::Started, 100., 200.),
            now + Duration::from_millis(200),
        );

        // A movement well within the slop scrolls immediately.
        let recognized = recognizer.handle_event_at(
            &touch_event(TouchId(2), TouchPhase::Moved, 100., 197.),
            now + Duration::from_millis(216),
        );
        let [RecognizedTouchGesture::Scroll(scroll)] = recognized.as_slice() else {
            panic!("expected scroll, got {recognized:?}");
        };
        assert_eq!(scroll.touch_phase, TouchPhase::Moved);
        assert_eq!(scroll.delta.pixel_delta(px(16.)), point(px(0.), px(-3.)));

        // Releasing the catch is not a tap.
        let recognized = recognizer.handle_event_at(
            &touch_event(TouchId(2), TouchPhase::Ended, 100., 197.),
            now + Duration::from_millis(232),
        );
        let [RecognizedTouchGesture::Scroll(scroll)] = recognized.as_slice() else {
            panic!("expected scroll, got {recognized:?}");
        };
        assert_eq!(scroll.touch_phase, TouchPhase::Ended);
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

    #[test]
    fn spline_position_table_matches_the_bezier_curve() {
        let samples = friction_spline::spline_position_samples();
        // AOSP's initializer solves sample 0 numerically like every other
        // sample, so it lands within solver tolerance of zero, not at zero.
        assert!(samples[0].abs() < 1e-4);
        assert_eq!(samples[100], 1.);
        for window in samples.windows(2) {
            assert!(window[0] < window[1], "table must be strictly increasing");
        }
        // Each table entry must lie on the defining parametric Bezier: for
        // sample i there must be a curve parameter whose time component is
        // i/100 and whose position component is the stored value.
        for (i, &stored_position) in samples.iter().enumerate().take(100) {
            let alpha = i as f32 / 100.;
            let (mut lower, mut upper) = (0f32, 1f32);
            for _ in 0..50 {
                let middle = (lower + upper) / 2.;
                let (time, _) = friction_spline::bezier_time_and_position(middle);
                if time > alpha {
                    upper = middle;
                } else {
                    lower = middle;
                }
            }
            let (time, position) = friction_spline::bezier_time_and_position((lower + upper) / 2.);
            assert!(
                (time - alpha).abs() < 1e-4,
                "sample {i}: time {time} != {alpha}"
            );
            assert!(
                (position - stored_position).abs() < 1e-3,
                "sample {i}: position {position} != stored {stored_position}"
            );
        }
    }

    #[test]
    fn fling_curves_are_sane_for_both_physics() {
        for physics in [ScrollPhysics::ios(), ScrollPhysics::android()] {
            let slow = physics.fling_duration(500.);
            let fast = physics.fling_duration(4000.);
            assert!(slow > Duration::ZERO, "{physics:?}");
            assert!(fast > slow, "faster flings must coast longer: {physics:?}");

            let halfway = physics.fling_distance(4000., fast / 2);
            let total = physics.fling_distance(4000., fast);
            assert!(halfway > 0. && halfway < total, "{physics:?}");
            assert!(
                physics.fling_distance(4000., fast * 2) == total,
                "distance must not grow past the fling duration: {physics:?}"
            );
            assert!(
                physics.fling_distance(4000., fast) > physics.fling_distance(500., slow),
                "faster flings must travel further: {physics:?}"
            );
        }
    }

    #[test]
    fn momentum_is_frame_rate_independent() {
        // The same fling ticked at 60Hz and as one huge stalled frame must
        // cover identical ground.
        let total_distance_with_tick_length = |tick: Duration| -> f32 {
            let mut recognizer = TouchGestureRecognizer::new(GestureTuning {
                scroll_physics: ScrollPhysics::android(),
                ..GestureTuning::default()
            });
            let now = Instant::now();
            recognizer.handle_event_at(
                &touch_event(TouchId(1), TouchPhase::Started, 100., 500.),
                now,
            );
            for step in 1..=3 {
                recognizer.handle_event_at(
                    &touch_event(
                        TouchId(1),
                        TouchPhase::Moved,
                        100.,
                        500. - step as f32 * 40.,
                    ),
                    now + Duration::from_millis(step * 16),
                );
            }
            recognizer.handle_event_at(
                &touch_event(TouchId(1), TouchPhase::Ended, 100., 380.),
                now + Duration::from_millis(64),
            );
            assert!(recognizer.has_momentum());

            let mut total = 0f32;
            let mut time = now + Duration::from_millis(64);
            let mut guard = 0;
            while recognizer.has_momentum() {
                time += tick;
                guard += 1;
                assert!(guard < 10_000, "momentum never stopped");
                if let Some(RecognizedTouchGesture::Scroll(scroll)) =
                    recognizer.tick_momentum_at(time)
                {
                    total += f32::from(scroll.delta.pixel_delta(px(16.)).y);
                }
            }
            total
        };

        let smooth = total_distance_with_tick_length(Duration::from_millis(16));
        let stalled = total_distance_with_tick_length(Duration::from_secs(10));
        assert!(
            (smooth - stalled).abs() < 0.01,
            "expected identical fling distance, got {smooth} vs {stalled}"
        );
    }

    #[test]
    fn flick_velocity_reflects_release_speed_not_window_average() {
        // A uniformly accelerating flick: position grows quadratically, so
        // the speed at the newest sample (2·k·t) is twice the window
        // average (k·t). The estimator must report the former.
        let mut velocity_tracker = VelocityTracker::default();
        let start = Instant::now();
        for step in 0..=6 {
            let t = step as f32 * 0.016;
            velocity_tracker.push(
                start + Duration::from_millis(step * 16),
                point(px(0.), px(1000. * t * t)),
            );
        }
        let velocity = velocity_tracker.velocity();
        let release_speed = 2. * 1000. * 0.096;
        assert!(
            (velocity.y - release_speed).abs() < 1.,
            "expected ≈{release_speed} px/s at release, got {} px/s",
            velocity.y
        );
        assert_eq!(velocity.x, 0.);
    }

    #[test]
    fn samples_before_a_pause_do_not_contribute_velocity() {
        // Fast motion, then a hold longer than the stopped-finger gap, then
        // a slow nudge: only the motion after the pause describes the
        // release.
        let mut velocity_tracker = VelocityTracker::default();
        let start = Instant::now();
        velocity_tracker.push(start, point(px(0.), px(0.)));
        velocity_tracker.push(start + Duration::from_millis(16), point(px(0.), px(50.)));
        velocity_tracker.push(start + Duration::from_millis(80), point(px(0.), px(52.)));
        velocity_tracker.push(start + Duration::from_millis(96), point(px(0.), px(54.)));
        let velocity = velocity_tracker.velocity();
        assert!(
            velocity.y < 200.,
            "pre-pause motion leaked into the estimate: {} px/s",
            velocity.y
        );
    }

    #[test]
    fn claimed_long_press_emits_phased_stream_without_tap() {
        let mut recognizer = TouchGestureRecognizer::new(GestureTuning::default());
        let touch = TouchId(1);
        let now = Instant::now();
        recognizer.handle_event_at(&touch_event(touch, TouchPhase::Started, 10., 20.), now);
        let Some(RecognizedTouchGesture::LongPress(started)) = recognizer.offer_long_press(touch)
        else {
            panic!("expected long press");
        };
        assert_eq!(started.phase, TouchPhase::Started);
        assert_eq!(started.start_position, point(px(10.), px(20.)));
        recognizer.resolve_long_press(true);

        let moved = recognizer.handle_event_at(
            &touch_event(touch, TouchPhase::Moved, 12., 21.),
            now + Duration::from_millis(510),
        );
        let [RecognizedTouchGesture::LongPress(moved)] = moved.as_slice() else {
            panic!("expected moved long press, got {moved:?}");
        };
        assert_eq!(moved.phase, TouchPhase::Moved);

        let ended = recognizer.handle_event_at(
            &touch_event(touch, TouchPhase::Ended, 12., 21.),
            now + Duration::from_millis(520),
        );
        let [RecognizedTouchGesture::LongPress(ended)] = ended.as_slice() else {
            panic!("expected ended long press, got {ended:?}");
        };
        assert_eq!(ended.phase, TouchPhase::Ended);
    }

    #[test]
    fn unclaimed_long_press_remains_a_tap_candidate() {
        let mut recognizer = TouchGestureRecognizer::new(GestureTuning::default());
        let touch = TouchId(1);
        let now = Instant::now();
        recognizer.handle_event_at(&touch_event(touch, TouchPhase::Started, 10., 20.), now);
        assert!(recognizer.offer_long_press(touch).is_some());
        recognizer.resolve_long_press(false);

        let ended = recognizer.handle_event_at(
            &touch_event(touch, TouchPhase::Ended, 10., 20.),
            now + Duration::from_millis(510),
        );
        assert!(matches!(
            ended.as_slice(),
            [RecognizedTouchGesture::Tap { .. }]
        ));
    }

    #[test]
    fn unclaimed_long_press_can_still_become_a_pan() {
        let mut recognizer = TouchGestureRecognizer::new(GestureTuning::default());
        let touch = TouchId(1);
        let now = Instant::now();
        recognizer.handle_event_at(&touch_event(touch, TouchPhase::Started, 0., 0.), now);
        assert!(recognizer.offer_long_press(touch).is_some());
        recognizer.resolve_long_press(false);

        let moved = recognizer.handle_event_at(
            &touch_event(touch, TouchPhase::Moved, 20., 0.),
            now + Duration::from_millis(510),
        );
        assert!(matches!(
            moved.as_slice(),
            [RecognizedTouchGesture::Scroll(ScrollWheelEvent {
                touch_phase: TouchPhase::Started,
                ..
            })]
        ));
    }

    #[test]
    fn long_press_offer_is_one_shot_and_specific_to_pending_touch() {
        let mut recognizer = TouchGestureRecognizer::new(GestureTuning::default());
        let touch = TouchId(1);
        recognizer.handle_event(&touch_event(touch, TouchPhase::Started, 0., 0.));

        assert!(
            recognizer
                .handle_event(&touch_event(TouchId(2), TouchPhase::Moved, 20., 0.))
                .is_empty()
        );
        assert!(recognizer.offer_long_press(TouchId(2)).is_none());
        assert!(recognizer.offer_long_press(touch).is_some());
        assert!(recognizer.offer_long_press(touch).is_none());
    }

    #[test]
    fn long_press_cannot_be_offered_after_pending_touch_resolves() {
        for phase in [TouchPhase::Ended, TouchPhase::Cancelled, TouchPhase::Moved] {
            let mut recognizer = TouchGestureRecognizer::new(GestureTuning::default());
            let touch = TouchId(1);
            let now = Instant::now();
            recognizer.handle_event_at(&touch_event(touch, TouchPhase::Started, 0., 0.), now);
            let position = if phase == TouchPhase::Moved { 20. } else { 0. };
            recognizer.handle_event_at(
                &touch_event(touch, phase, position, 0.),
                now + Duration::from_millis(10),
            );
            assert!(recognizer.offer_long_press(touch).is_none());
        }
    }

    #[test]
    fn claimed_long_press_emits_cancelled_for_its_touch_only() {
        let mut recognizer = TouchGestureRecognizer::new(GestureTuning::default());
        let touch = TouchId(1);
        recognizer.handle_event(&touch_event(touch, TouchPhase::Started, 4., 5.));
        assert!(recognizer.offer_long_press(touch).is_some());
        recognizer.resolve_long_press(true);

        assert!(
            recognizer
                .handle_event(&touch_event(TouchId(2), TouchPhase::Cancelled, 9., 9.))
                .is_empty()
        );
        let cancelled = recognizer.handle_event(&touch_event(touch, TouchPhase::Cancelled, 6., 7.));
        let [RecognizedTouchGesture::LongPress(cancelled)] = cancelled.as_slice() else {
            panic!("expected cancelled long press, got {cancelled:?}");
        };
        assert_eq!(cancelled.phase, TouchPhase::Cancelled);
        assert_eq!(cancelled.start_position, point(px(4.), px(5.)));
        assert_eq!(cancelled.position, point(px(6.), px(7.)));
    }

    #[test]
    fn unrelated_touch_cannot_end_or_cancel_pending_touch() {
        for phase in [TouchPhase::Ended, TouchPhase::Cancelled] {
            let mut recognizer = TouchGestureRecognizer::new(GestureTuning::default());
            let touch = TouchId(1);
            recognizer.handle_event(&touch_event(touch, TouchPhase::Started, 4., 5.));

            assert!(
                recognizer
                    .handle_event(&touch_event(TouchId(2), phase, 9., 9.))
                    .is_empty()
            );
            assert!(recognizer.offer_long_press(touch).is_some());
        }
    }

    #[test]
    fn completed_touch_id_cannot_claim_replacement_touch() {
        let mut recognizer = TouchGestureRecognizer::new(GestureTuning::default());
        let completed_touch = TouchId(1);
        let replacement_touch = TouchId(2);
        recognizer.handle_event(&touch_event(completed_touch, TouchPhase::Started, 0., 0.));
        recognizer.handle_event(&touch_event(completed_touch, TouchPhase::Cancelled, 0., 0.));
        recognizer.handle_event(&touch_event(replacement_touch, TouchPhase::Started, 5., 5.));

        assert!(recognizer.offer_long_press(completed_touch).is_none());
        assert!(recognizer.offer_long_press(replacement_touch).is_some());
    }

    fn touch_event(id: TouchId, phase: TouchPhase, x: f32, y: f32) -> TouchEvent {
        TouchEvent {
            id,
            phase,
            position: point(px(x), px(y)),
            predicted_position: None,
            force: None,
        }
    }
}
