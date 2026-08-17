/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Time source abstraction to support WASM and testing
use std::fmt::Debug;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Trait with a `now()` function returning the current time
pub trait TimeSource: Debug + Send + Sync {
    /// Returns the current time
    fn now(&self) -> SystemTime;
}

/// Time source that delegates to [`SystemTime::now`]
#[non_exhaustive]
#[derive(Debug, Default)]
pub struct SystemTimeSource;

impl SystemTimeSource {
    /// Creates a new SystemTimeSource
    pub fn new() -> Self {
        SystemTimeSource
    }
}

impl TimeSource for SystemTimeSource {
    fn now(&self) -> SystemTime {
        // this is the one OK usage
        #[allow(clippy::disallowed_methods)]
        SystemTime::now()
    }
}

impl Default for SharedTimeSource {
    fn default() -> Self {
        SharedTimeSource(Arc::new(SystemTimeSource))
    }
}

/// Time source that always returns the same time
#[derive(Debug)]
pub struct StaticTimeSource {
    time: SystemTime,
}

impl StaticTimeSource {
    /// Creates a new static time source that always returns the same time
    pub fn new(time: SystemTime) -> Self {
        Self { time }
    }

    /// Creates a new static time source from the provided number of seconds since the UNIX epoch
    pub fn from_secs(epoch_secs: u64) -> Self {
        Self::new(UNIX_EPOCH + Duration::from_secs(epoch_secs))
    }
}

impl TimeSource for StaticTimeSource {
    fn now(&self) -> SystemTime {
        self.time
    }
}

impl From<StaticTimeSource> for SharedTimeSource {
    fn from(value: StaticTimeSource) -> Self {
        SharedTimeSource::new(value)
    }
}

#[derive(Debug, Clone)]
/// Time source structure used inside SDK
///
/// This implements Default—the default implementation will use `SystemTime::now()`
pub struct SharedTimeSource(Arc<dyn TimeSource>);

impl SharedTimeSource {
    /// Returns the current time
    pub fn now(&self) -> SystemTime {
        self.0.now()
    }

    /// Creates a new shared time source
    pub fn new(source: impl TimeSource + 'static) -> Self {
        Self(Arc::new(source))
    }
}

impl TimeSource for SharedTimeSource {
    fn now(&self) -> SystemTime {
        self.0.now()
    }
}
