//! Platform gesture vocabulary shared by `gpui` and its platform backends.

use gpui_types::{Pixels, px};
use std::time::Duration;

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
}

/// The gesture kinds a platform recognizes natively, rather than leaving to
/// gpui core's portable recognizers.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GestureKinds {
    /// Tap (and multi-tap), surfaced as `ClickEvent::Touch`.
    pub tap: bool,
    /// Long press, surfaced as `LongPressEvent`.
    pub long_press: bool,
    /// Pan/scroll (including fling momentum), surfaced as `ScrollWheelEvent`s.
    pub pan: bool,
    /// Pinch to zoom, surfaced as `PinchEvent`s.
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
