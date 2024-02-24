use chrono::{DateTime, Utc};

pub trait SystemClock: Send + Sync {
    /// Returns the current date and time in UTC.
    fn utc_now(&self) -> DateTime<Utc>;
}

pub struct RealSystemClock;

impl SystemClock for RealSystemClock {
    fn utc_now(&self) -> DateTime<Utc> {
        Utc::now()
    }
}

#[cfg(any(test, feature = "test-support"))]
pub struct FakeSystemClockState {
    now: DateTime<Utc>,
}

#[cfg(any(test, feature = "test-support"))]
pub struct FakeSystemClock {
    // Use an unfair lock to ensure tests are deterministic.
    state: parking_lot::Mutex<FakeSystemClockState>,
}

#[cfg(any(test, feature = "test-support"))]
impl Default for FakeSystemClock {
    fn default() -> Self {
        Self::new(Utc::now())
    }
}

#[cfg(any(test, feature = "test-support"))]
impl FakeSystemClock {
    pub fn new(now: DateTime<Utc>) -> Self {
        let state = FakeSystemClockState { now };

        Self {
            state: parking_lot::Mutex::new(state),
        }
    }

    pub fn set_now(&self, now: DateTime<Utc>) {
        self.state.lock().now = now;
    }

    /// Advances the [`FakeSystemClock`] by the specified [`Duration`](chrono::Duration).
    pub fn advance(&self, duration: chrono::Duration) {
        self.state.lock().now += duration;
    }
}

#[cfg(any(test, feature = "test-support"))]
impl SystemClock for FakeSystemClock {
    fn utc_now(&self) -> DateTime<Utc> {
        self.state.lock().now
    }
}
