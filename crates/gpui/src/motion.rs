use std::{rc::Rc, time::Duration};

/// Creates a duration from a number of whole seconds.
pub const fn secs(seconds: u64) -> Duration {
    Duration::from_secs(seconds)
}

/// Creates a duration from a number of whole milliseconds.
pub const fn millis(milliseconds: u64) -> Duration {
    Duration::from_millis(milliseconds)
}

/// Creates a [`Motion`] from a duration.
pub trait DurationWithEasing {
    /// Uses this duration with the supplied easing function.
    fn with_easing(self, easing: impl Fn(f32) -> f32 + 'static) -> Motion;
}

impl DurationWithEasing for Duration {
    fn with_easing(self, easing: impl Fn(f32) -> f32 + 'static) -> Motion {
        Motion::new(self).with_easing(easing)
    }
}

/// Controls the duration and easing of a style transition.
#[derive(Clone)]
pub struct Motion {
    duration: Duration,
    easing: Rc<dyn Fn(f32) -> f32>,
}

impl Motion {
    /// Creates a linear motion with this duration.
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
            easing: Rc::new(crate::linear),
        }
    }

    /// Sets the easing function.
    pub fn with_easing(mut self, easing: impl Fn(f32) -> f32 + 'static) -> Self {
        self.easing = Rc::new(easing);
        self
    }

    pub(crate) fn sample(&self, elapsed: Duration) -> MotionSample {
        if self.duration.is_zero() {
            return MotionSample {
                phase: 1.0,
                is_active: false,
            };
        }

        let linear_phase = elapsed.as_secs_f32() / self.duration.as_secs_f32();

        if linear_phase >= 1.0 {
            return MotionSample {
                phase: 1.0,
                is_active: false,
            };
        }

        let phase = (self.easing)(linear_phase.clamp(0.0, 1.0));
        debug_assert!(
            phase.is_finite(),
            "motion easing must return a finite value"
        );

        MotionSample {
            phase,
            is_active: true,
        }
    }
}

impl From<Duration> for Motion {
    fn from(duration: Duration) -> Self {
        Self::new(duration)
    }
}

pub(crate) struct MotionSample {
    pub(crate) phase: f32,
    pub(crate) is_active: bool,
}
