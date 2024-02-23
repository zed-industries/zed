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
pub struct FakeSystemClock {
    now: DateTime<Utc>,
}

#[cfg(any(test, feature = "test-support"))]
impl Default for FakeSystemClock {
    fn default() -> Self {
        Self { now: Utc::now() }
    }
}

#[cfg(any(test, feature = "test-support"))]
impl FakeSystemClock {
    pub fn new(now: DateTime<Utc>) -> Self {
        Self { now }
    }

    pub fn set_now(&mut self, now: DateTime<Utc>) {
        self.now = now;
    }
}

#[cfg(any(test, feature = "test-support"))]
impl SystemClock for FakeSystemClock {
    fn utc_now(&self) -> DateTime<Utc> {
        self.now
    }
}
