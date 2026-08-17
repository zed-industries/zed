/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::time::TimeSource;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

/// Manually controlled time source
#[derive(Debug, Clone)]
pub struct ManualTimeSource {
    pub(super) start_time: SystemTime,
    pub(super) log: Arc<Mutex<Vec<Duration>>>,
}

impl ManualTimeSource {
    /// Get the number of seconds since the UNIX Epoch as an f64.
    ///
    /// ## Panics
    ///
    /// This will panic if `self.now()` returns a time that's before the UNIX Epoch.
    pub fn seconds_since_unix_epoch(&self) -> f64 {
        self.now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64()
    }

    /// Creates a new [`ManualTimeSource`]
    pub fn new(start_time: SystemTime) -> ManualTimeSource {
        Self {
            start_time,
            log: Default::default(),
        }
    }

    /// Advances the time of this time source by `duration`.
    pub fn advance(&self, duration: Duration) -> SystemTime {
        let mut log = self.log.lock().unwrap();
        log.push(duration);
        self._now(&log)
    }

    fn _now(&self, log: &[Duration]) -> SystemTime {
        self.start_time + log.iter().sum::<Duration>()
    }

    /// Sets the `time` of this manual time source.
    ///
    /// # Panics
    /// This function panics if `time` < `now()`
    pub fn set_time(&self, time: SystemTime) {
        let mut log = self.log.lock().unwrap();
        let now = self._now(&log);
        if time < now {
            panic!("Cannot move time backwards!");
        }
        log.push(time.duration_since(now).unwrap());
    }
}

impl TimeSource for ManualTimeSource {
    fn now(&self) -> SystemTime {
        self._now(&self.log.lock().unwrap())
    }
}
