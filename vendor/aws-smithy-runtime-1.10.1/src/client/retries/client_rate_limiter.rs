/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! A rate limiter for controlling the rate at which AWS requests are made. The rate changes based
//! on the number of throttling errors encountered.

#![allow(dead_code)]

use crate::client::retries::RetryPartition;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tracing::debug;

/// Represents a partition for the rate limiter, e.g. an endpoint, a region
#[non_exhaustive]
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct ClientRateLimiterPartition {
    retry_partition: RetryPartition,
}

impl ClientRateLimiterPartition {
    /// Creates a `ClientRateLimiterPartition` from the given [`RetryPartition`]
    pub fn new(retry_partition: RetryPartition) -> Self {
        Self { retry_partition }
    }
}

const RETRY_COST: f64 = 5.0;
const RETRY_TIMEOUT_COST: f64 = RETRY_COST * 2.0;
const INITIAL_REQUEST_COST: f64 = 1.0;

const MIN_FILL_RATE: f64 = 0.5;
const MIN_CAPACITY: f64 = 1.0;
const SMOOTH: f64 = 0.8;
/// How much to scale back after receiving a throttling response
const BETA: f64 = 0.7;
/// Controls how aggressively we scale up after being throttled
const SCALE_CONSTANT: f64 = 0.4;

/// Rate limiter for adaptive retry.
#[derive(Clone, Debug)]
pub struct ClientRateLimiter {
    pub(crate) inner: Arc<Mutex<Inner>>,
}

#[derive(Debug)]
pub(crate) struct Inner {
    /// The rate at which token are replenished.
    fill_rate: f64,
    /// The maximum capacity allowed in the token bucket.
    max_capacity: f64,
    /// The current capacity of the token bucket. The minimum this can be is 1.0
    current_capacity: f64,
    /// The last time the token bucket was refilled.
    last_timestamp: Option<f64>,
    /// Boolean indicating if the token bucket is enabled.
    /// The token bucket is initially disabled.
    /// When a throttling error is encountered it is enabled.
    enabled: bool,
    /// The smoothed rate which tokens are being retrieved.
    measured_tx_rate: f64,
    /// The last half second time bucket used.
    last_tx_rate_bucket: f64,
    /// The number of requests seen within the current time bucket.
    request_count: u64,
    /// The maximum rate when the client was last throttled.
    last_max_rate: f64,
    /// The last time when the client was throttled.
    time_of_last_throttle: f64,
}

pub(crate) enum RequestReason {
    Retry,
    RetryTimeout,
    InitialRequest,
}

impl Default for ClientRateLimiter {
    fn default() -> Self {
        Self::builder().build()
    }
}

impl ClientRateLimiter {
    /// Creates a new `ClientRateLimiter`
    pub fn new(seconds_since_unix_epoch: f64) -> Self {
        Self::builder()
            .tokens_retrieved_per_second(MIN_FILL_RATE)
            .time_of_last_throttle(seconds_since_unix_epoch)
            .previous_time_bucket(seconds_since_unix_epoch.floor())
            .build()
    }

    /// Creates a new `ClientRateLimiterBuilder`
    pub fn builder() -> ClientRateLimiterBuilder {
        ClientRateLimiterBuilder::new()
    }

    pub(crate) fn acquire_permission_to_send_a_request(
        &self,
        seconds_since_unix_epoch: f64,
        kind: RequestReason,
    ) -> Result<(), Duration> {
        let mut it = self.inner.lock().unwrap();

        if !it.enabled {
            // return early if we haven't encountered a throttling error yet
            return Ok(());
        }
        let amount = match kind {
            RequestReason::Retry => RETRY_COST,
            RequestReason::RetryTimeout => RETRY_TIMEOUT_COST,
            RequestReason::InitialRequest => INITIAL_REQUEST_COST,
        };

        it.refill(seconds_since_unix_epoch);

        let res = if amount > it.current_capacity {
            let sleep_time = (amount - it.current_capacity) / it.fill_rate;
            debug!(
                amount,
                it.current_capacity,
                it.fill_rate,
                sleep_time,
                "client rate limiter delayed a request"
            );

            Err(Duration::from_secs_f64(sleep_time))
        } else {
            Ok(())
        };

        it.current_capacity -= amount;
        res
    }

    pub(crate) fn update_rate_limiter(
        &self,
        seconds_since_unix_epoch: f64,
        is_throttling_error: bool,
    ) {
        let mut it = self.inner.lock().unwrap();
        it.update_tokens_retrieved_per_second(seconds_since_unix_epoch);

        let calculated_rate;
        if is_throttling_error {
            let rate_to_use = if it.enabled {
                f64::min(it.measured_tx_rate, it.fill_rate)
            } else {
                it.measured_tx_rate
            };

            // The fill_rate is from the token bucket
            it.last_max_rate = rate_to_use;
            it.calculate_time_window();
            it.time_of_last_throttle = seconds_since_unix_epoch;
            calculated_rate = cubic_throttle(rate_to_use);
            it.enable_token_bucket();
        } else {
            it.calculate_time_window();
            calculated_rate = it.cubic_success(seconds_since_unix_epoch);
        }

        let new_rate = f64::min(calculated_rate, 2.0 * it.measured_tx_rate);
        it.update_bucket_refill_rate(seconds_since_unix_epoch, new_rate);
    }
}

impl Inner {
    fn refill(&mut self, seconds_since_unix_epoch: f64) {
        if let Some(last_timestamp) = self.last_timestamp {
            let fill_amount = (seconds_since_unix_epoch - last_timestamp) * self.fill_rate;
            self.current_capacity =
                f64::min(self.max_capacity, self.current_capacity + fill_amount);
            debug!(
                fill_amount,
                self.current_capacity, self.max_capacity, "refilling client rate limiter tokens"
            );
        }
        self.last_timestamp = Some(seconds_since_unix_epoch);
    }

    fn update_bucket_refill_rate(&mut self, seconds_since_unix_epoch: f64, new_fill_rate: f64) {
        // Refill based on our current rate before we update to the new fill rate.
        self.refill(seconds_since_unix_epoch);

        self.fill_rate = f64::max(new_fill_rate, MIN_FILL_RATE);
        self.max_capacity = f64::max(new_fill_rate, MIN_CAPACITY);

        debug!(
            fill_rate = self.fill_rate,
            max_capacity = self.max_capacity,
            current_capacity = self.current_capacity,
            measured_tx_rate = self.measured_tx_rate,
            "client rate limiter state has been updated"
        );

        // When we scale down we can't have a current capacity that exceeds our max_capacity.
        self.current_capacity = f64::min(self.current_capacity, self.max_capacity);
    }

    fn enable_token_bucket(&mut self) {
        // If throttling wasn't already enabled, note that we're now enabling it.
        if !self.enabled {
            debug!("client rate limiting has been enabled");
        }
        self.enabled = true;
    }

    fn update_tokens_retrieved_per_second(&mut self, seconds_since_unix_epoch: f64) {
        let next_time_bucket = (seconds_since_unix_epoch * 2.0).floor() / 2.0;
        self.request_count += 1;

        if next_time_bucket > self.last_tx_rate_bucket {
            let current_rate =
                self.request_count as f64 / (next_time_bucket - self.last_tx_rate_bucket);
            self.measured_tx_rate = current_rate * SMOOTH + self.measured_tx_rate * (1.0 - SMOOTH);
            self.request_count = 0;
            self.last_tx_rate_bucket = next_time_bucket;
        }
    }

    fn calculate_time_window(&self) -> f64 {
        let base = (self.last_max_rate * (1.0 - BETA)) / SCALE_CONSTANT;
        base.powf(1.0 / 3.0)
    }

    fn cubic_success(&self, seconds_since_unix_epoch: f64) -> f64 {
        let dt =
            seconds_since_unix_epoch - self.time_of_last_throttle - self.calculate_time_window();
        (SCALE_CONSTANT * dt.powi(3)) + self.last_max_rate
    }
}

fn cubic_throttle(rate_to_use: f64) -> f64 {
    rate_to_use * BETA
}

/// Builder for `ClientRateLimiter`.
#[derive(Clone, Debug, Default)]
pub struct ClientRateLimiterBuilder {
    ///The rate at which token are replenished.
    token_refill_rate: Option<f64>,
    ///The maximum capacity allowed in the token bucket.
    maximum_bucket_capacity: Option<f64>,
    ///The current capacity of the token bucket.
    current_bucket_capacity: Option<f64>,
    ///The last time the token bucket was refilled.
    time_of_last_refill: Option<f64>,
    ///The smoothed rate which tokens are being retrieved.
    tokens_retrieved_per_second: Option<f64>,
    ///The last half second time bucket used.
    previous_time_bucket: Option<f64>,
    ///The number of requests seen within the current time bucket.
    request_count: Option<u64>,
    ///Boolean indicating if the token bucket is enabled. The token bucket is initially disabled. When a throttling error is encountered it is enabled.
    enable_throttling: Option<bool>,
    ///The maximum rate when the client was last throttled.
    tokens_retrieved_per_second_at_time_of_last_throttle: Option<f64>,
    ///The last time when the client was throttled.
    time_of_last_throttle: Option<f64>,
}

impl ClientRateLimiterBuilder {
    /// Create a new `ClientRateLimiterBuilder`.
    pub fn new() -> Self {
        ClientRateLimiterBuilder::default()
    }
    /// The rate at which token are replenished.
    pub fn token_refill_rate(mut self, token_refill_rate: f64) -> Self {
        self.set_token_refill_rate(Some(token_refill_rate));
        self
    }
    /// The rate at which token are replenished.
    pub fn set_token_refill_rate(&mut self, token_refill_rate: Option<f64>) -> &mut Self {
        self.token_refill_rate = token_refill_rate;
        self
    }
    /// The maximum capacity allowed in the token bucket
    ///
    /// The implementation of [`ClientRateLimiter`] guarantees that `current_capacity` never exceeds this value.
    pub fn maximum_bucket_capacity(mut self, maximum_bucket_capacity: f64) -> Self {
        self.set_maximum_bucket_capacity(Some(maximum_bucket_capacity));
        self
    }
    /// The maximum capacity allowed in the token bucket
    ///
    /// The implementation of [`ClientRateLimiter`] guarantees that `current_capacity` never exceeds this value.
    pub fn set_maximum_bucket_capacity(
        &mut self,
        maximum_bucket_capacity: Option<f64>,
    ) -> &mut Self {
        self.maximum_bucket_capacity = maximum_bucket_capacity;
        self
    }
    /// The current capacity of the token bucket
    ///
    /// The implementation of [`ClientRateLimiter`] guarantees that this value is always at least `1.0` when it's enabled.
    pub fn current_bucket_capacity(mut self, current_bucket_capacity: f64) -> Self {
        self.set_current_bucket_capacity(Some(current_bucket_capacity));
        self
    }
    /// The current capacity of the token bucket
    ///
    /// The implementation of [`ClientRateLimiter`] guarantees that this value is always at least `1.0` when it's enabled.
    pub fn set_current_bucket_capacity(
        &mut self,
        current_bucket_capacity: Option<f64>,
    ) -> &mut Self {
        self.current_bucket_capacity = current_bucket_capacity;
        self
    }
    // The last time the token bucket was refilled.
    fn time_of_last_refill(mut self, time_of_last_refill: f64) -> Self {
        self.set_time_of_last_refill(Some(time_of_last_refill));
        self
    }
    // The last time the token bucket was refilled.
    fn set_time_of_last_refill(&mut self, time_of_last_refill: Option<f64>) -> &mut Self {
        self.time_of_last_refill = time_of_last_refill;
        self
    }
    /// The smoothed rate which tokens are being retrieved.
    pub fn tokens_retrieved_per_second(mut self, tokens_retrieved_per_second: f64) -> Self {
        self.set_tokens_retrieved_per_second(Some(tokens_retrieved_per_second));
        self
    }
    /// The smoothed rate which tokens are being retrieved.
    pub fn set_tokens_retrieved_per_second(
        &mut self,
        tokens_retrieved_per_second: Option<f64>,
    ) -> &mut Self {
        self.tokens_retrieved_per_second = tokens_retrieved_per_second;
        self
    }
    // The last half second time bucket used.
    fn previous_time_bucket(mut self, previous_time_bucket: f64) -> Self {
        self.set_previous_time_bucket(Some(previous_time_bucket));
        self
    }
    // The last half second time bucket used.
    fn set_previous_time_bucket(&mut self, previous_time_bucket: Option<f64>) -> &mut Self {
        self.previous_time_bucket = previous_time_bucket;
        self
    }
    // The number of requests seen within the current time bucket.
    fn request_count(mut self, request_count: u64) -> Self {
        self.set_request_count(Some(request_count));
        self
    }
    // The number of requests seen within the current time bucket.
    fn set_request_count(&mut self, request_count: Option<u64>) -> &mut Self {
        self.request_count = request_count;
        self
    }
    // Boolean indicating if the token bucket is enabled. The token bucket is initially disabled. When a throttling error is encountered it is enabled.
    fn enable_throttling(mut self, enable_throttling: bool) -> Self {
        self.set_enable_throttling(Some(enable_throttling));
        self
    }
    // Boolean indicating if the token bucket is enabled. The token bucket is initially disabled. When a throttling error is encountered it is enabled.
    fn set_enable_throttling(&mut self, enable_throttling: Option<bool>) -> &mut Self {
        self.enable_throttling = enable_throttling;
        self
    }
    // The maximum rate when the client was last throttled.
    fn tokens_retrieved_per_second_at_time_of_last_throttle(
        mut self,
        tokens_retrieved_per_second_at_time_of_last_throttle: f64,
    ) -> Self {
        self.set_tokens_retrieved_per_second_at_time_of_last_throttle(Some(
            tokens_retrieved_per_second_at_time_of_last_throttle,
        ));
        self
    }
    // The maximum rate when the client was last throttled.
    fn set_tokens_retrieved_per_second_at_time_of_last_throttle(
        &mut self,
        tokens_retrieved_per_second_at_time_of_last_throttle: Option<f64>,
    ) -> &mut Self {
        self.tokens_retrieved_per_second_at_time_of_last_throttle =
            tokens_retrieved_per_second_at_time_of_last_throttle;
        self
    }
    // The last time when the client was throttled.
    fn time_of_last_throttle(mut self, time_of_last_throttle: f64) -> Self {
        self.set_time_of_last_throttle(Some(time_of_last_throttle));
        self
    }
    // The last time when the client was throttled.
    fn set_time_of_last_throttle(&mut self, time_of_last_throttle: Option<f64>) -> &mut Self {
        self.time_of_last_throttle = time_of_last_throttle;
        self
    }
    /// Build the ClientRateLimiter.
    pub fn build(self) -> ClientRateLimiter {
        ClientRateLimiter {
            inner: Arc::new(Mutex::new(Inner {
                fill_rate: self.token_refill_rate.unwrap_or_default(),
                max_capacity: self.maximum_bucket_capacity.unwrap_or(f64::MAX),
                current_capacity: self.current_bucket_capacity.unwrap_or_default(),
                last_timestamp: self.time_of_last_refill,
                enabled: self.enable_throttling.unwrap_or_default(),
                measured_tx_rate: self.tokens_retrieved_per_second.unwrap_or_default(),
                last_tx_rate_bucket: self.previous_time_bucket.unwrap_or_default(),
                request_count: self.request_count.unwrap_or_default(),
                last_max_rate: self
                    .tokens_retrieved_per_second_at_time_of_last_throttle
                    .unwrap_or_default(),
                time_of_last_throttle: self.time_of_last_throttle.unwrap_or_default(),
            })),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{cubic_throttle, ClientRateLimiter};
    use crate::client::retries::client_rate_limiter::RequestReason;
    use approx::assert_relative_eq;
    use aws_smithy_async::rt::sleep::AsyncSleep;
    use aws_smithy_async::test_util::instant_time_and_sleep;
    use std::time::{Duration, SystemTime};

    const ONE_SECOND: Duration = Duration::from_secs(1);
    const TWO_HUNDRED_MILLISECONDS: Duration = Duration::from_millis(200);

    #[test]
    fn should_match_beta_decrease() {
        let new_rate = cubic_throttle(10.0);
        assert_relative_eq!(new_rate, 7.0);

        let rate_limiter = ClientRateLimiter::builder()
            .tokens_retrieved_per_second_at_time_of_last_throttle(10.0)
            .time_of_last_throttle(1.0)
            .build();

        rate_limiter.inner.lock().unwrap().calculate_time_window();
        let new_rate = rate_limiter.inner.lock().unwrap().cubic_success(1.0);
        assert_relative_eq!(new_rate, 7.0);
    }

    #[tokio::test]
    async fn throttling_is_enabled_once_throttling_error_is_received() {
        let rate_limiter = ClientRateLimiter::builder()
            .previous_time_bucket(0.0)
            .time_of_last_throttle(0.0)
            .build();

        assert!(
            !rate_limiter.inner.lock().unwrap().enabled,
            "rate_limiter should be disabled by default"
        );
        rate_limiter.update_rate_limiter(0.0, true);
        assert!(
            rate_limiter.inner.lock().unwrap().enabled,
            "rate_limiter should be enabled after throttling error"
        );
    }

    #[tokio::test]
    async fn test_calculated_rate_with_successes() {
        let rate_limiter = ClientRateLimiter::builder()
            .time_of_last_throttle(5.0)
            .tokens_retrieved_per_second_at_time_of_last_throttle(10.0)
            .build();

        struct Attempt {
            seconds_since_unix_epoch: f64,
            expected_calculated_rate: f64,
        }

        let attempts = [
            Attempt {
                seconds_since_unix_epoch: 5.0,
                expected_calculated_rate: 7.0,
            },
            Attempt {
                seconds_since_unix_epoch: 6.0,
                expected_calculated_rate: 9.64893600966,
            },
            Attempt {
                seconds_since_unix_epoch: 7.0,
                expected_calculated_rate: 10.000030849917364,
            },
            Attempt {
                seconds_since_unix_epoch: 8.0,
                expected_calculated_rate: 10.453284520772092,
            },
            Attempt {
                seconds_since_unix_epoch: 9.0,
                expected_calculated_rate: 13.408697022224185,
            },
            Attempt {
                seconds_since_unix_epoch: 10.0,
                expected_calculated_rate: 21.26626835427364,
            },
            Attempt {
                seconds_since_unix_epoch: 11.0,
                expected_calculated_rate: 36.425998516920465,
            },
        ];

        // Think this test is a little strange? I ported the test from Go v2, and this is how it
        // was implemented. See for yourself:
        // https://github.com/aws/aws-sdk-go-v2/blob/844ff45cdc76182229ad098c95bf3f5ab8c20e9f/aws/retry/adaptive_ratelimit_test.go#L97
        for attempt in attempts {
            rate_limiter.inner.lock().unwrap().calculate_time_window();
            let calculated_rate = rate_limiter
                .inner
                .lock()
                .unwrap()
                .cubic_success(attempt.seconds_since_unix_epoch);

            assert_relative_eq!(attempt.expected_calculated_rate, calculated_rate);
        }
    }

    #[tokio::test]
    async fn test_calculated_rate_with_throttles() {
        let rate_limiter = ClientRateLimiter::builder()
            .tokens_retrieved_per_second_at_time_of_last_throttle(10.0)
            .time_of_last_throttle(5.0)
            .build();

        struct Attempt {
            throttled: bool,
            seconds_since_unix_epoch: f64,
            expected_calculated_rate: f64,
        }

        let attempts = [
            Attempt {
                throttled: false,
                seconds_since_unix_epoch: 5.0,
                expected_calculated_rate: 7.0,
            },
            Attempt {
                throttled: false,
                seconds_since_unix_epoch: 6.0,
                expected_calculated_rate: 9.64893600966,
            },
            Attempt {
                throttled: true,
                seconds_since_unix_epoch: 7.0,
                expected_calculated_rate: 6.754255206761999,
            },
            Attempt {
                throttled: true,
                seconds_since_unix_epoch: 8.0,
                expected_calculated_rate: 4.727978644733399,
            },
            Attempt {
                throttled: false,
                seconds_since_unix_epoch: 9.0,
                expected_calculated_rate: 4.670125557970046,
            },
            Attempt {
                throttled: false,
                seconds_since_unix_epoch: 10.0,
                expected_calculated_rate: 4.770870456867401,
            },
            Attempt {
                throttled: false,
                seconds_since_unix_epoch: 11.0,
                expected_calculated_rate: 6.011819748005445,
            },
            Attempt {
                throttled: false,
                seconds_since_unix_epoch: 12.0,
                expected_calculated_rate: 10.792973431384178,
            },
        ];

        // Think this test is a little strange? I ported the test from Go v2, and this is how it
        // was implemented. See for yourself:
        // https://github.com/aws/aws-sdk-go-v2/blob/844ff45cdc76182229ad098c95bf3f5ab8c20e9f/aws/retry/adaptive_ratelimit_test.go#L97
        let mut calculated_rate = 0.0;
        for attempt in attempts {
            let mut inner = rate_limiter.inner.lock().unwrap();
            inner.calculate_time_window();
            if attempt.throttled {
                calculated_rate = cubic_throttle(calculated_rate);
                inner.time_of_last_throttle = attempt.seconds_since_unix_epoch;
                inner.last_max_rate = calculated_rate;
            } else {
                calculated_rate = inner.cubic_success(attempt.seconds_since_unix_epoch);
            };

            assert_relative_eq!(attempt.expected_calculated_rate, calculated_rate);
        }
    }

    #[tokio::test]
    async fn test_client_sending_rates() {
        let (_, sleep_impl) = instant_time_and_sleep(SystemTime::UNIX_EPOCH);
        let rate_limiter = ClientRateLimiter::builder().build();

        struct Attempt {
            throttled: bool,
            seconds_since_unix_epoch: f64,
            expected_tokens_retrieved_per_second: f64,
            expected_token_refill_rate: f64,
        }

        let attempts = [
            Attempt {
                throttled: false,
                seconds_since_unix_epoch: 0.2,
                expected_tokens_retrieved_per_second: 0.000000,
                expected_token_refill_rate: 0.500000,
            },
            Attempt {
                throttled: false,
                seconds_since_unix_epoch: 0.4,
                expected_tokens_retrieved_per_second: 0.000000,
                expected_token_refill_rate: 0.500000,
            },
            Attempt {
                throttled: false,
                seconds_since_unix_epoch: 0.6,
                expected_tokens_retrieved_per_second: 4.800000000000001,
                expected_token_refill_rate: 0.500000,
            },
            Attempt {
                throttled: false,
                seconds_since_unix_epoch: 0.8,
                expected_tokens_retrieved_per_second: 4.800000000000001,
                expected_token_refill_rate: 0.500000,
            },
            Attempt {
                throttled: false,
                seconds_since_unix_epoch: 1.0,
                expected_tokens_retrieved_per_second: 4.160000,
                expected_token_refill_rate: 0.500000,
            },
            Attempt {
                throttled: false,
                seconds_since_unix_epoch: 1.2,
                expected_tokens_retrieved_per_second: 4.160000,
                expected_token_refill_rate: 0.691200,
            },
            Attempt {
                throttled: false,
                seconds_since_unix_epoch: 1.4,
                expected_tokens_retrieved_per_second: 4.160000,
                expected_token_refill_rate: 1.0975999999999997,
            },
            Attempt {
                throttled: false,
                seconds_since_unix_epoch: 1.6,
                expected_tokens_retrieved_per_second: 5.632000000000001,
                expected_token_refill_rate: 1.6384000000000005,
            },
            Attempt {
                throttled: false,
                seconds_since_unix_epoch: 1.8,
                expected_tokens_retrieved_per_second: 5.632000000000001,
                expected_token_refill_rate: 2.332800,
            },
            Attempt {
                throttled: true,
                seconds_since_unix_epoch: 2.0,
                expected_tokens_retrieved_per_second: 4.326400,
                expected_token_refill_rate: 3.0284799999999996,
            },
            Attempt {
                throttled: false,
                seconds_since_unix_epoch: 2.2,
                expected_tokens_retrieved_per_second: 4.326400,
                expected_token_refill_rate: 3.48663917347026,
            },
            Attempt {
                throttled: false,
                seconds_since_unix_epoch: 2.4,
                expected_tokens_retrieved_per_second: 4.326400,
                expected_token_refill_rate: 3.821874416040255,
            },
            Attempt {
                throttled: false,
                seconds_since_unix_epoch: 2.6,
                expected_tokens_retrieved_per_second: 5.665280,
                expected_token_refill_rate: 4.053385727709987,
            },
            Attempt {
                throttled: false,
                seconds_since_unix_epoch: 2.8,
                expected_tokens_retrieved_per_second: 5.665280,
                expected_token_refill_rate: 4.200373108479454,
            },
            Attempt {
                throttled: false,
                seconds_since_unix_epoch: 3.0,
                expected_tokens_retrieved_per_second: 4.333056,
                expected_token_refill_rate: 4.282036558348658,
            },
            Attempt {
                throttled: true,
                seconds_since_unix_epoch: 3.2,
                expected_tokens_retrieved_per_second: 4.333056,
                expected_token_refill_rate: 2.99742559084406,
            },
            Attempt {
                throttled: false,
                seconds_since_unix_epoch: 3.4,
                expected_tokens_retrieved_per_second: 4.333056,
                expected_token_refill_rate: 3.4522263943863463,
            },
        ];

        for attempt in attempts {
            sleep_impl.sleep(TWO_HUNDRED_MILLISECONDS).await;
            assert_eq!(
                attempt.seconds_since_unix_epoch,
                sleep_impl.total_duration().as_secs_f64()
            );

            rate_limiter.update_rate_limiter(attempt.seconds_since_unix_epoch, attempt.throttled);
            assert_relative_eq!(
                attempt.expected_tokens_retrieved_per_second,
                rate_limiter.inner.lock().unwrap().measured_tx_rate
            );
            assert_relative_eq!(
                attempt.expected_token_refill_rate,
                rate_limiter.inner.lock().unwrap().fill_rate
            );
        }
    }

    // This test is only testing that we don't fail basic math and panic. It does include an
    // element of randomness, but no duration between >= 0.0s and <= 1.0s will ever cause a panic.
    //
    // Because the cost of sending an individual request is 1.0, and because the minimum capacity is
    // also 1.0, we will never encounter a situation where we run out of tokens.
    #[tokio::test]
    async fn test_when_throttling_is_enabled_requests_can_still_be_sent() {
        let (time_source, sleep_impl) = instant_time_and_sleep(SystemTime::UNIX_EPOCH);
        let crl = ClientRateLimiter::builder()
            .time_of_last_throttle(0.0)
            .previous_time_bucket(0.0)
            .build();

        // Start by recording a throttling error
        crl.update_rate_limiter(0.0, true);

        for _i in 0..100 {
            // advance time by a random amount (up to 1s) each iteration
            let duration = Duration::from_secs_f64(fastrand::f64());
            sleep_impl.sleep(duration).await;
            if let Err(delay) = crl.acquire_permission_to_send_a_request(
                time_source.seconds_since_unix_epoch(),
                RequestReason::InitialRequest,
            ) {
                sleep_impl.sleep(delay).await;
            }

            // Assume all further requests succeed on the first try
            crl.update_rate_limiter(time_source.seconds_since_unix_epoch(), false);
        }

        let inner = crl.inner.lock().unwrap();
        assert!(inner.enabled, "the rate limiter should still be enabled");
        // Assert that the rate limiter respects the passage of time.
        assert_relative_eq!(
            inner.last_timestamp.unwrap(),
            sleep_impl.total_duration().as_secs_f64(),
            max_relative = 0.0001
        );
    }
}
