use parking_lot::Mutex;
use std::time::Duration;

pub use web_time::{Instant, SystemTime};

pub trait Clock {
    fn utc_now(&self) -> SystemTime;
    fn now(&self) -> Instant;
}

pub struct TestClock(Mutex<TestClockState>);

struct TestClockState {
    now: Instant,
    utc_now: SystemTime,
}

impl TestClock {
    pub fn new() -> Self {
        // 2025-07-01T23:59:58Z
        const START_TIME_UNIX_SECS: u64 = 1_751_414_398;
        let utc_now = SystemTime::UNIX_EPOCH + Duration::from_secs(START_TIME_UNIX_SECS);
        Self(Mutex::new(TestClockState {
            now: Instant::now(),
            utc_now,
        }))
    }

    pub fn set_utc_now(&self, now: SystemTime) {
        let mut state = self.0.lock();
        state.utc_now = now;
    }

    pub fn advance(&self, duration: Duration) {
        let mut state = self.0.lock();
        state.now += duration;
        state.utc_now += duration;
    }
}

impl Clock for TestClock {
    fn utc_now(&self) -> SystemTime {
        self.0.lock().utc_now
    }

    fn now(&self) -> Instant {
        self.0.lock().now
    }
}
