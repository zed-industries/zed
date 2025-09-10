use chrono::{DateTime, Utc};
use parking_lot::Mutex;
use std::time::{Duration, Instant};

pub trait Clock {
    fn utc_now(&self) -> DateTime<Utc>;
    fn now(&self) -> Instant;
}

pub struct TestClock(Mutex<TestClockState>);

struct TestClockState {
    now: Instant,
    utc_now: DateTime<Utc>,
}

impl TestClock {
    pub fn new() -> Self {
        const START_TIME: &str = "2025-07-01T23:59:58-00:00";
        let utc_now = DateTime::parse_from_rfc3339(START_TIME).unwrap().to_utc();
        Self(Mutex::new(TestClockState {
            now: Instant::now(),
            utc_now,
        }))
    }

    pub fn advance(&self, duration: Duration) {
        let mut state = self.0.lock();
        state.now += duration;
        state.utc_now += duration;
    }
}

impl Clock for TestClock {
    fn utc_now(&self) -> DateTime<Utc> {
        self.0.lock().utc_now
    }

    fn now(&self) -> Instant {
        self.0.lock().now
    }
}
