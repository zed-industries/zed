/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use super::Throughput;
use aws_smithy_runtime_api::client::stalled_stream_protection::{
    StalledStreamProtectionConfig, DEFAULT_GRACE_PERIOD,
};
use std::time::Duration;

/// A collection of options for configuring a [`MinimumThroughputBody`](super::MinimumThroughputDownloadBody).
#[derive(Debug, Clone)]
pub struct MinimumThroughputBodyOptions {
    /// The minimum throughput that is acceptable.
    minimum_throughput: Throughput,

    /// The 'grace period' after which the minimum throughput will be enforced.
    ///
    /// If this is set to 0, the minimum throughput will be enforced immediately.
    ///
    /// If this is set to a positive value, whenever throughput is below the minimum throughput,
    /// a timer is started. If the timer expires before throughput rises above the minimum,
    /// an error is emitted.
    ///
    /// It is recommended to set this to a small value (e.g. 200ms) to avoid issues during
    /// stream-startup.
    grace_period: Duration,

    /// The period of time to consider when computing the throughput
    ///
    /// This SHOULD be longer than the check interval, or stuck-streams may evade detection.
    check_window: Duration,
}

impl MinimumThroughputBodyOptions {
    /// Create a new builder.
    pub fn builder() -> MinimumThroughputBodyOptionsBuilder {
        Default::default()
    }

    /// Convert this struct into a builder.
    pub fn to_builder(self) -> MinimumThroughputBodyOptionsBuilder {
        MinimumThroughputBodyOptionsBuilder::new()
            .minimum_throughput(self.minimum_throughput)
            .grace_period(self.grace_period)
    }

    /// The throughput check grace period.
    ///
    /// If throughput is below the minimum for longer than this period, an error is emitted.
    ///
    /// If this is set to 0, the minimum throughput will be enforced immediately.
    pub fn grace_period(&self) -> Duration {
        self.grace_period
    }

    /// The minimum acceptable throughput
    pub fn minimum_throughput(&self) -> Throughput {
        self.minimum_throughput
    }

    pub(crate) fn check_window(&self) -> Duration {
        self.check_window
    }

    /// Not used. Always returns `Duration::from_millis(500)`.
    #[deprecated(note = "No longer used. Always returns Duration::from_millis(500)")]
    pub fn check_interval(&self) -> Duration {
        Duration::from_millis(500)
    }
}

const DEFAULT_MINIMUM_THROUGHPUT: Throughput = Throughput {
    bytes_read: 1,
    per_time_elapsed: Duration::from_secs(1),
};

const DEFAULT_CHECK_WINDOW: Duration = Duration::from_secs(1);

impl Default for MinimumThroughputBodyOptions {
    fn default() -> Self {
        Self {
            minimum_throughput: DEFAULT_MINIMUM_THROUGHPUT,
            grace_period: DEFAULT_GRACE_PERIOD,
            check_window: DEFAULT_CHECK_WINDOW,
        }
    }
}

/// A builder for [`MinimumThroughputBodyOptions`]
#[derive(Debug, Default, Clone)]
pub struct MinimumThroughputBodyOptionsBuilder {
    minimum_throughput: Option<Throughput>,
    check_window: Option<Duration>,
    grace_period: Option<Duration>,
}

impl MinimumThroughputBodyOptionsBuilder {
    /// Create a new `MinimumThroughputBodyOptionsBuilder`.
    pub fn new() -> Self {
        Default::default()
    }

    /// Set the amount of time that throughput my fall below minimum before an error is emitted.
    ///
    /// If throughput rises above the minimum, the timer is reset.
    pub fn grace_period(mut self, grace_period: Duration) -> Self {
        self.set_grace_period(Some(grace_period));
        self
    }

    /// Set the amount of time that throughput my fall below minimum before an error is emitted.
    ///
    /// If throughput rises above the minimum, the timer is reset.
    pub fn set_grace_period(&mut self, grace_period: Option<Duration>) -> &mut Self {
        self.grace_period = grace_period;
        self
    }

    /// Set the minimum allowable throughput.
    pub fn minimum_throughput(mut self, minimum_throughput: Throughput) -> Self {
        self.set_minimum_throughput(Some(minimum_throughput));
        self
    }

    /// Set the minimum allowable throughput.
    pub fn set_minimum_throughput(&mut self, minimum_throughput: Option<Throughput>) -> &mut Self {
        self.minimum_throughput = minimum_throughput;
        self
    }

    /// No longer used. The check interval is now based on the check window (not currently configurable).
    #[deprecated(
        note = "No longer used. The check interval is now based on the check window (not currently configurable). Open an issue if you need to configure the check window."
    )]
    pub fn check_interval(self, _check_interval: Duration) -> Self {
        self
    }

    /// No longer used. The check interval is now based on the check window (not currently configurable).
    #[deprecated(
        note = "No longer used. The check interval is now based on the check window (not currently configurable). Open an issue if you need to configure the check window."
    )]
    pub fn set_check_interval(&mut self, _check_interval: Option<Duration>) -> &mut Self {
        self
    }

    #[allow(unused)]
    pub(crate) fn check_window(mut self, check_window: Duration) -> Self {
        self.set_check_window(Some(check_window));
        self
    }
    #[allow(unused)]
    pub(crate) fn set_check_window(&mut self, check_window: Option<Duration>) -> &mut Self {
        self.check_window = check_window;
        self
    }

    /// Build this builder, producing a [`MinimumThroughputBodyOptions`].
    ///
    /// Unset fields will be set with defaults.
    pub fn build(self) -> MinimumThroughputBodyOptions {
        MinimumThroughputBodyOptions {
            grace_period: self.grace_period.unwrap_or(DEFAULT_GRACE_PERIOD),
            minimum_throughput: self
                .minimum_throughput
                .unwrap_or(DEFAULT_MINIMUM_THROUGHPUT),
            check_window: self.check_window.unwrap_or(DEFAULT_CHECK_WINDOW),
        }
    }
}

impl From<StalledStreamProtectionConfig> for MinimumThroughputBodyOptions {
    fn from(value: StalledStreamProtectionConfig) -> Self {
        MinimumThroughputBodyOptions {
            grace_period: value.grace_period(),
            minimum_throughput: DEFAULT_MINIMUM_THROUGHPUT,
            check_window: DEFAULT_CHECK_WINDOW,
        }
    }
}
