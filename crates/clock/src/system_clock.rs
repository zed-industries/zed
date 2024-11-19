use std::time::Instant;

pub trait SystemClock: Send + Sync {
    /// Returns the current date and time in UTC.
    fn utc_now(&self) -> Instant;
}

pub struct RealSystemClock;

impl SystemClock for RealSystemClock {
    fn utc_now(&self) -> Instant {
        Instant::now()
    }
}

#[cfg(any(test, feature = "test-support"))]
pub struct FakeSystemClockState {
    now: Instant,
}

#[cfg(any(test, feature = "test-support"))]
pub struct FakeSystemClock {
    // Use an unfair lock to ensure tests are deterministic.
    state: parking_lot::Mutex<FakeSystemClockState>,
}

#[cfg(any(test, feature = "test-support"))]
impl FakeSystemClock {
    pub fn new() -> Self {
        let state = FakeSystemClockState {
            now: Instant::now(),
        };

        Self {
            state: parking_lot::Mutex::new(state),
        }
    }

    pub fn set_now(&self, now: Instant) {
        self.state.lock().now = now;
    }

    pub fn advance(&self, duration: std::time::Duration) {
        self.state.lock().now += duration;
    }
}

#[cfg(any(test, feature = "test-support"))]
impl SystemClock for FakeSystemClock {
    fn utc_now(&self) -> Instant {
        self.state.lock().now
    }
}
