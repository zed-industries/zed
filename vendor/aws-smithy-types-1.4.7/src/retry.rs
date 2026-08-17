/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! This module defines types that describe when to retry given a response.

use crate::config_bag::{Storable, StoreReplace};
use std::fmt;
use std::str::FromStr;
use std::time::Duration;

const VALID_RETRY_MODES: &[RetryMode] = &[RetryMode::Standard];

/// Type of error that occurred when making a request.
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
#[non_exhaustive]
pub enum ErrorKind {
    /// A connection-level error.
    ///
    /// A `TransientError` can represent conditions such as socket timeouts, socket connection errors, or TLS negotiation timeouts.
    ///
    /// `TransientError` is not modeled by Smithy and is instead determined through client-specific heuristics and response status codes.
    ///
    /// Typically these should never be applied for non-idempotent request types
    /// since in this scenario, it's impossible to know whether the operation had
    /// a side effect on the server.
    ///
    /// TransientErrors are not currently modeled. They are determined based on specific provider
    /// level errors & response status code.
    TransientError,

    /// An error where the server explicitly told the client to back off, such as a 429 or 503 HTTP error.
    ThrottlingError,

    /// Server error that isn't explicitly throttling but is considered by the client
    /// to be something that should be retried.
    ServerError,

    /// Doesn't count against any budgets. This could be something like a 401 challenge in Http.
    ClientError,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TransientError => write!(f, "transient error"),
            Self::ThrottlingError => write!(f, "throttling error"),
            Self::ServerError => write!(f, "server error"),
            Self::ClientError => write!(f, "client error"),
        }
    }
}

/// Trait that provides an `ErrorKind` and an error code.
pub trait ProvideErrorKind {
    /// Returns the `ErrorKind` when the error is modeled as retryable
    ///
    /// If the error kind cannot be determined (e.g. the error is unmodeled at the error kind depends
    /// on an HTTP status code, return `None`.
    fn retryable_error_kind(&self) -> Option<ErrorKind>;

    /// Returns the `code` for this error if one exists
    fn code(&self) -> Option<&str>;
}

/// `RetryKind` describes how a request MAY be retried for a given response
///
/// A `RetryKind` describes how a response MAY be retried; it does not mandate retry behavior.
/// The actual retry behavior is at the sole discretion of the RetryStrategy in place.
/// A RetryStrategy may ignore the suggestion for a number of reasons including but not limited to:
/// - Number of retry attempts exceeded
/// - The required retry delay exceeds the maximum backoff configured by the client
/// - No retry tokens are available due to service health
#[non_exhaustive]
#[derive(Eq, PartialEq, Debug)]
pub enum RetryKind {
    /// Retry the associated request due to a known `ErrorKind`.
    Error(ErrorKind),

    /// An Explicit retry (e.g. from `x-amz-retry-after`).
    ///
    /// Note: The specified `Duration` is considered a suggestion and may be replaced or ignored.
    Explicit(Duration),

    /// The response was a failure that should _not_ be retried.
    UnretryableFailure,

    /// The response was successful, so no retry is necessary.
    Unnecessary,
}

/// Specifies how failed requests should be retried.
#[non_exhaustive]
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum RetryMode {
    /// The standard set of retry rules across AWS SDKs. This mode includes a standard set of errors
    /// that are retried, and support for retry quotas. The default maximum number of attempts
    /// with this mode is three, unless otherwise explicitly configured with [`RetryConfig`].
    Standard,

    /// An experimental retry mode that includes the functionality of standard mode but includes
    /// automatic client-side throttling. Because this mode is experimental, it might change
    /// behavior in the future.
    Adaptive,
}

impl FromStr for RetryMode {
    type Err = RetryModeParseError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let string = string.trim();

        // eq_ignore_ascii_case is OK here because the only strings we need to check for are ASCII
        if string.eq_ignore_ascii_case("standard") {
            Ok(RetryMode::Standard)
        } else if string.eq_ignore_ascii_case("adaptive") {
            Ok(RetryMode::Adaptive)
        } else {
            Err(RetryModeParseError::new(string))
        }
    }
}

/// Failure to parse a `RetryMode` from string.
#[derive(Debug)]
pub struct RetryModeParseError {
    message: String,
}

impl RetryModeParseError {
    pub(super) fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for RetryModeParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "error parsing string '{}' as RetryMode, valid options are: {:#?}",
            self.message, VALID_RETRY_MODES
        )
    }
}

impl std::error::Error for RetryModeParseError {}

/// Builder for [`RetryConfig`].
#[non_exhaustive]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct RetryConfigBuilder {
    mode: Option<RetryMode>,
    max_attempts: Option<u32>,
    initial_backoff: Option<Duration>,
    max_backoff: Option<Duration>,
    reconnect_mode: Option<ReconnectMode>,
}

impl RetryConfigBuilder {
    /// Creates a new builder.
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets the retry mode.
    pub fn set_mode(&mut self, retry_mode: Option<RetryMode>) -> &mut Self {
        self.mode = retry_mode;
        self
    }

    /// Sets the retry mode.
    pub fn mode(mut self, mode: RetryMode) -> Self {
        self.set_mode(Some(mode));
        self
    }

    /// Set the [`ReconnectMode`] for the retry strategy
    ///
    /// By default, when a transient error is encountered, the connection in use will be poisoned.
    /// This prevents reusing a connection to a potentially bad host but may increase the load on
    /// the server.
    ///
    /// This behavior can be disabled by setting [`ReconnectMode::ReuseAllConnections`] instead.
    pub fn reconnect_mode(mut self, reconnect_mode: ReconnectMode) -> Self {
        self.set_reconnect_mode(Some(reconnect_mode));
        self
    }

    /// Set the [`ReconnectMode`] for the retry strategy
    ///
    /// By default, when a transient error is encountered, the connection in use will be poisoned.
    /// This prevents reusing a connection to a potentially bad host but may increase the load on
    /// the server.
    ///
    /// This behavior can be disabled by setting [`ReconnectMode::ReuseAllConnections`] instead.
    pub fn set_reconnect_mode(&mut self, reconnect_mode: Option<ReconnectMode>) -> &mut Self {
        self.reconnect_mode = reconnect_mode;
        self
    }

    /// Sets the max attempts. This value must be greater than zero.
    pub fn set_max_attempts(&mut self, max_attempts: Option<u32>) -> &mut Self {
        self.max_attempts = max_attempts;
        self
    }

    /// Sets the max attempts. This value must be greater than zero.
    pub fn max_attempts(mut self, max_attempts: u32) -> Self {
        self.set_max_attempts(Some(max_attempts));
        self
    }

    /// Set the initial_backoff duration. This duration should be non-zero.
    pub fn set_initial_backoff(&mut self, initial_backoff: Option<Duration>) -> &mut Self {
        self.initial_backoff = initial_backoff;
        self
    }

    /// Set the initial_backoff duration. This duration should be non-zero.
    pub fn initial_backoff(mut self, initial_backoff: Duration) -> Self {
        self.set_initial_backoff(Some(initial_backoff));
        self
    }

    /// Set the max_backoff duration. This duration should be non-zero.
    pub fn set_max_backoff(&mut self, max_backoff: Option<Duration>) -> &mut Self {
        self.max_backoff = max_backoff;
        self
    }

    /// Set the max_backoff duration. This duration should be non-zero.
    pub fn max_backoff(mut self, max_backoff: Duration) -> Self {
        self.set_max_backoff(Some(max_backoff));
        self
    }

    /// Merge two builders together. Values from `other` will only be used as a fallback for values
    /// from `self` Useful for merging configs from different sources together when you want to
    /// handle "precedence" per value instead of at the config level
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aws_smithy_types::retry::{RetryMode, RetryConfigBuilder};
    /// let a = RetryConfigBuilder::new().max_attempts(1);
    /// let b = RetryConfigBuilder::new().max_attempts(5).mode(RetryMode::Adaptive);
    /// let retry_config = a.take_unset_from(b).build();
    /// // A's value take precedence over B's value
    /// assert_eq!(retry_config.max_attempts(), 1);
    /// // A never set a retry mode so B's value was used
    /// assert_eq!(retry_config.mode(), RetryMode::Adaptive);
    /// ```
    pub fn take_unset_from(self, other: Self) -> Self {
        Self {
            mode: self.mode.or(other.mode),
            max_attempts: self.max_attempts.or(other.max_attempts),
            initial_backoff: self.initial_backoff.or(other.initial_backoff),
            max_backoff: self.max_backoff.or(other.max_backoff),
            reconnect_mode: self.reconnect_mode.or(other.reconnect_mode),
        }
    }

    /// Builds a `RetryConfig`.
    pub fn build(self) -> RetryConfig {
        RetryConfig {
            mode: self.mode.unwrap_or(RetryMode::Standard),
            max_attempts: self.max_attempts.unwrap_or(3),
            initial_backoff: self
                .initial_backoff
                .unwrap_or_else(|| Duration::from_secs(1)),
            reconnect_mode: self
                .reconnect_mode
                .unwrap_or(ReconnectMode::ReconnectOnTransientError),
            max_backoff: self.max_backoff.unwrap_or_else(|| Duration::from_secs(20)),
            use_static_exponential_base: false,
        }
    }
}

/// Retry configuration for requests.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct RetryConfig {
    mode: RetryMode,
    max_attempts: u32,
    initial_backoff: Duration,
    max_backoff: Duration,
    reconnect_mode: ReconnectMode,
    use_static_exponential_base: bool,
}

impl Storable for RetryConfig {
    type Storer = StoreReplace<RetryConfig>;
}

/// Mode for connection re-establishment
///
/// By default, when a transient error is encountered, the connection in use will be poisoned. This
/// behavior can be disabled by setting [`ReconnectMode::ReuseAllConnections`] instead.
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum ReconnectMode {
    /// Reconnect on [`ErrorKind::TransientError`]
    ReconnectOnTransientError,

    /// Disable reconnect on error
    ///
    /// When this setting is applied, 503s, timeouts, and other transient errors will _not_
    /// lead to a new connection being established unless the connection is closed by the remote.
    ReuseAllConnections,
}

impl Storable for ReconnectMode {
    type Storer = StoreReplace<ReconnectMode>;
}

impl RetryConfig {
    /// Creates a default `RetryConfig` with `RetryMode::Standard` and max attempts of three.
    pub fn standard() -> Self {
        Self {
            mode: RetryMode::Standard,
            max_attempts: 3,
            initial_backoff: Duration::from_secs(1),
            reconnect_mode: ReconnectMode::ReconnectOnTransientError,
            max_backoff: Duration::from_secs(20),
            use_static_exponential_base: false,
        }
    }

    /// Creates a default `RetryConfig` with `RetryMode::Adaptive` and max attempts of three.
    pub fn adaptive() -> Self {
        Self {
            mode: RetryMode::Adaptive,
            max_attempts: 3,
            initial_backoff: Duration::from_secs(1),
            reconnect_mode: ReconnectMode::ReconnectOnTransientError,
            max_backoff: Duration::from_secs(20),
            use_static_exponential_base: false,
        }
    }

    /// Creates a `RetryConfig` that has retries disabled.
    pub fn disabled() -> Self {
        Self::standard().with_max_attempts(1)
    }

    /// Set this config's [retry mode](RetryMode).
    pub fn with_retry_mode(mut self, retry_mode: RetryMode) -> Self {
        self.mode = retry_mode;
        self
    }

    /// Set the maximum number of times a request should be tried, including the initial attempt.
    /// This value must be greater than zero.
    pub fn with_max_attempts(mut self, max_attempts: u32) -> Self {
        self.max_attempts = max_attempts;
        self
    }

    /// Set the [`ReconnectMode`] for the retry strategy
    ///
    /// By default, when a transient error is encountered, the connection in use will be poisoned.
    /// This prevents reusing a connection to a potentially bad host but may increase the load on
    /// the server.
    ///
    /// This behavior can be disabled by setting [`ReconnectMode::ReuseAllConnections`] instead.
    pub fn with_reconnect_mode(mut self, reconnect_mode: ReconnectMode) -> Self {
        self.reconnect_mode = reconnect_mode;
        self
    }

    /// Set the multiplier used when calculating backoff times as part of an
    /// [exponential backoff with jitter](https://aws.amazon.com/blogs/architecture/exponential-backoff-and-jitter/)
    /// strategy. Most services should work fine with the default duration of 1 second, but if you
    /// find that your requests are taking too long due to excessive retry backoff, try lowering
    /// this value.
    ///
    /// ## Example
    ///
    /// *For a request that gets retried 3 times, when initial_backoff is 1 seconds:*
    /// - the first retry will occur after 0 to 1 seconds
    /// - the second retry will occur after 0 to 2 seconds
    /// - the third retry will occur after 0 to 4 seconds
    ///
    /// *For a request that gets retried 3 times, when initial_backoff is 30 milliseconds:*
    /// - the first retry will occur after 0 to 30 milliseconds
    /// - the second retry will occur after 0 to 60 milliseconds
    /// - the third retry will occur after 0 to 120 milliseconds
    pub fn with_initial_backoff(mut self, initial_backoff: Duration) -> Self {
        self.initial_backoff = initial_backoff;
        self
    }

    /// Set the maximum backoff time.
    pub fn with_max_backoff(mut self, max_backoff: Duration) -> Self {
        self.max_backoff = max_backoff;
        self
    }

    /// Hint to the retry strategy whether to use a static exponential base.
    ///
    /// When a retry strategy uses exponential backoff, it calculates a random base. This causes the
    /// retry delay to be slightly random, and helps prevent "thundering herd" scenarios. However,
    /// it's often useful during testing to know exactly how long the delay will be.
    ///
    /// Therefore, if you're writing a test and asserting an expected retry delay,
    /// set this to `true`.
    #[cfg(feature = "test-util")]
    pub fn with_use_static_exponential_base(mut self, use_static_exponential_base: bool) -> Self {
        self.use_static_exponential_base = use_static_exponential_base;
        self
    }

    /// Returns the retry mode.
    pub fn mode(&self) -> RetryMode {
        self.mode
    }

    /// Returns the [`ReconnectMode`]
    pub fn reconnect_mode(&self) -> ReconnectMode {
        self.reconnect_mode
    }

    /// Returns the max attempts.
    pub fn max_attempts(&self) -> u32 {
        self.max_attempts
    }

    /// Returns the backoff multiplier duration.
    pub fn initial_backoff(&self) -> Duration {
        self.initial_backoff
    }

    /// Returns the max backoff duration.
    pub fn max_backoff(&self) -> Duration {
        self.max_backoff
    }

    /// Returns true if retry is enabled with this config
    pub fn has_retry(&self) -> bool {
        self.max_attempts > 1
    }

    /// Returns `true` if retry strategies should use a static exponential base instead of the
    /// default random base.
    ///
    /// To set this value, the `test-util` feature must be enabled.
    pub fn use_static_exponential_base(&self) -> bool {
        self.use_static_exponential_base
    }
}

#[cfg(test)]
mod tests {
    use crate::retry::{RetryConfigBuilder, RetryMode};
    use std::str::FromStr;

    #[test]
    fn retry_config_builder_merge_with_favors_self_values_over_other_values() {
        let self_builder = RetryConfigBuilder::new()
            .max_attempts(1)
            .mode(RetryMode::Adaptive);
        let other_builder = RetryConfigBuilder::new()
            .max_attempts(5)
            .mode(RetryMode::Standard);
        let retry_config = self_builder.take_unset_from(other_builder).build();

        assert_eq!(retry_config.max_attempts, 1);
        assert_eq!(retry_config.mode, RetryMode::Adaptive);
    }

    #[test]
    fn retry_mode_from_str_parses_valid_strings_regardless_of_casing() {
        assert_eq!(
            RetryMode::from_str("standard").ok(),
            Some(RetryMode::Standard)
        );
        assert_eq!(
            RetryMode::from_str("STANDARD").ok(),
            Some(RetryMode::Standard)
        );
        assert_eq!(
            RetryMode::from_str("StAnDaRd").ok(),
            Some(RetryMode::Standard)
        );
        // assert_eq!(
        //     RetryMode::from_str("adaptive").ok(),
        //     Some(RetryMode::Adaptive)
        // );
        // assert_eq!(
        //     RetryMode::from_str("ADAPTIVE").ok(),
        //     Some(RetryMode::Adaptive)
        // );
        // assert_eq!(
        //     RetryMode::from_str("aDaPtIvE").ok(),
        //     Some(RetryMode::Adaptive)
        // );
    }

    #[test]
    fn retry_mode_from_str_ignores_whitespace_before_and_after() {
        assert_eq!(
            RetryMode::from_str("  standard ").ok(),
            Some(RetryMode::Standard)
        );
        assert_eq!(
            RetryMode::from_str("   STANDARD  ").ok(),
            Some(RetryMode::Standard)
        );
        assert_eq!(
            RetryMode::from_str("  StAnDaRd   ").ok(),
            Some(RetryMode::Standard)
        );
        // assert_eq!(
        //     RetryMode::from_str("  adaptive  ").ok(),
        //     Some(RetryMode::Adaptive)
        // );
        // assert_eq!(
        //     RetryMode::from_str("   ADAPTIVE ").ok(),
        //     Some(RetryMode::Adaptive)
        // );
        // assert_eq!(
        //     RetryMode::from_str("  aDaPtIvE    ").ok(),
        //     Some(RetryMode::Adaptive)
        // );
    }

    #[test]
    fn retry_mode_from_str_wont_parse_invalid_strings() {
        assert_eq!(RetryMode::from_str("std").ok(), None);
        assert_eq!(RetryMode::from_str("aws").ok(), None);
        assert_eq!(RetryMode::from_str("s t a n d a r d").ok(), None);
        assert_eq!(RetryMode::from_str("a d a p t i v e").ok(), None);
    }
}
