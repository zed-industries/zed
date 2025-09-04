use chrono::{DateTime, Duration, Utc};
use parking_lot::Mutex;

pub trait Clock {
    fn now(&self) -> DateTime<Utc>;
}

pub struct TestClock {
    now: Mutex<DateTime<Utc>>,
}

impl TestClock {
    pub fn new() -> Self {
        const START_TIME: &str = "2025-07-01T23:59:58-00:00";
        let now = DateTime::parse_from_rfc3339(START_TIME).unwrap().to_utc();
        Self {
            now: Mutex::new(now),
        }
    }

    pub fn set_now(&self, now: DateTime<Utc>) {
        *self.now.lock() = now;
    }

    pub fn advance(&self, duration: Duration) {
        *self.now.lock() += duration;
    }
}

impl Clock for TestClock {
    fn now(&self) -> DateTime<Utc> {
        *self.now.lock()
    }
}
