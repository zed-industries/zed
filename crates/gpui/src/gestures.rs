//! Touch gesture recognition vocabulary.
//!
//! GPUI recognizes gestures from raw [`TouchEvent`](crate::TouchEvent)s in a
//! single, portable arena in gpui core: recognizers compete for in-flight
//! touches, winners claim them, and losers are cancelled. Recognized gestures
//! are surfaced through *existing* semantic events wherever possible — a tap
//! becomes [`ClickEvent::Touch`](crate::ClickEvent), a pan becomes
//! [`ScrollWheelEvent`](crate::ScrollWheelEvent)s carrying a
//! [`TouchPhase`](crate::TouchPhase), and a pinch becomes
//! [`PinchEvent`](crate::PinchEvent)s — so components written against
//! `on_click` and scroll containers work untouched on mobile.
//!
//! Two hard rules, established by the mobile design work:
//!
//! 1. **Never synthesize mouse events from touches.** Touches must not
//!    produce `MouseDown`/`MouseMove`/`MouseUp`, update the window's mouse
//!    position, or affect hover state or cursor style. Gesture recognition is
//!    the *only* sanctioned synthesis point, and it emits the semantic events
//!    above, never mouse-mechanical ones.
//! 2. **"The system took this touch" is a first-class outcome.** Mobile OSes
//!    steal in-flight touches (system edge gestures, incoming calls, native
//!    views claiming a gesture); recognizers observe this as
//!    [`TouchPhase::Cancelled`](crate::TouchPhase) and must fully unwind.
//!
//! The arena itself is not implemented yet; this module defines the shared
//! vocabulary that platform implementations and the arena build against.

use std::time::Duration;

use crate::{Pixels, Point, px};

/// Feel constants consumed by gesture recognizers.
///
/// Platforms provide native values through [`PlatformGestures::tuning`] so
/// that gestures feel native on each OS (Android's touch slop, iOS's scroll
/// momentum decay, and so on). Recognizers in gpui core must source all
/// disambiguation thresholds from this struct rather than hardcoding them.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GestureTuning {
    /// Distance a touch may travel before it stops being a potential tap and
    /// becomes a pan/drag. (Android `ViewConfiguration.getScaledTouchSlop`;
    /// UIKit applies an equivalent implicitly.)
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
/// There is no desktop analogue; elements opt in explicitly. The element
/// registration API ships together with the gesture arena.
#[derive(Clone, Debug, Default)]
pub struct LongPressEvent {
    /// The position of the touch that was recognized as a long press.
    pub position: Point<Pixels>,
}

/// Platform gesture recognition services: the third platform layer alongside
/// [`Platform`](crate::Platform) and [`PlatformWindow`](crate::PlatformWindow).
///
/// The gesture arena in gpui core always owns *arbitration* — which
/// recognizer claims which touch. This trait lets a platform take over
/// *detection* for the gesture kinds it can recognize with OS fidelity (for
/// example, UIKit's `UIPanGestureRecognizer`/`UIPinchGestureRecognizer` on
/// iOS). Native recognizers enter the arena as ordinary contestants: they
/// claim touches and receive and issue cancellation exactly like a core
/// recognizer, and "a native view claimed the touch" is observed by the
/// losers as [`TouchPhase::Cancelled`](crate::TouchPhase).
///
/// Gesture kinds not claimed here fall back to gpui core's portable
/// recognizers, tuned by [`Self::tuning`].
///
/// The registration and event-feed API between native recognizers and the
/// arena is deliberately unspecified until the arena's design doc lands; this
/// trait currently captures only the platform-declared capabilities.
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
