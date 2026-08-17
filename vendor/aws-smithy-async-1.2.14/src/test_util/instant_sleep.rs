/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::time::{SharedTimeSource, TimeSource};
use crate::{
    rt::sleep::{AsyncSleep, Sleep},
    test_util::ManualTimeSource,
};
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

/// A sleep implementation where calls to [`AsyncSleep::sleep`] will complete instantly.
///
/// Create a [`InstantSleep`] with [`instant_time_and_sleep`]
#[derive(Debug, Clone)]
pub struct InstantSleep {
    log: Arc<Mutex<Vec<Duration>>>,
}

impl AsyncSleep for InstantSleep {
    fn sleep(&self, duration: Duration) -> Sleep {
        let log = self.log.clone();
        Sleep::new(async move {
            log.lock().unwrap().push(duration);
        })
    }
}

impl InstantSleep {
    /// Given a shared log for sleep durations, create a new `InstantSleep`.
    pub fn new(log: Arc<Mutex<Vec<Duration>>>) -> Self {
        Self { log }
    }

    /// Create an `InstantSleep` without passing in a shared log.
    pub fn unlogged() -> Self {
        Self {
            log: Default::default(),
        }
    }

    /// Return the sleep durations that were logged by this `InstantSleep`.
    pub fn logs(&self) -> Vec<Duration> {
        self.log.lock().unwrap().iter().cloned().collect()
    }

    /// Return the total sleep duration that was logged by this `InstantSleep`.
    pub fn total_duration(&self) -> Duration {
        self.log.lock().unwrap().iter().sum()
    }
}

/// Returns a duo of tools to test interactions with time. Sleeps will end instantly, but the
/// desired length of the sleeps will be recorded for later verification.
pub fn instant_time_and_sleep(start_time: SystemTime) -> (ManualTimeSource, InstantSleep) {
    let log = Arc::new(Mutex::new(vec![]));
    let sleep = InstantSleep::new(log.clone());
    (ManualTimeSource { start_time, log }, sleep)
}

impl TimeSource for SystemTime {
    fn now(&self) -> SystemTime {
        *self
    }
}

impl From<SystemTime> for SharedTimeSource {
    fn from(value: SystemTime) -> Self {
        SharedTimeSource::new(value)
    }
}

impl From<ManualTimeSource> for SharedTimeSource {
    fn from(value: ManualTimeSource) -> Self {
        SharedTimeSource::new(value)
    }
}
