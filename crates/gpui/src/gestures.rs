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

use std::time::Duration;

use crate::{Pixels, Point, px};

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
