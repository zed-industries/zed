/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Retry handling and token bucket.
//!
//! This code defines when and how failed requests should be retried. It also defines the behavior
//! used to limit the rate that requests are sent.

pub mod classifiers;

use crate::box_error::BoxError;
use crate::client::interceptors::context::InterceptorContext;
use crate::client::runtime_components::sealed::ValidateConfig;
use crate::client::runtime_components::RuntimeComponents;
use aws_smithy_types::config_bag::{ConfigBag, Storable, StoreReplace};
use std::fmt;
use std::sync::Arc;
use std::time::Duration;

use crate::impl_shared_conversions;
pub use aws_smithy_types::retry::ErrorKind;
#[cfg(feature = "test-util")]
pub use test_util::AlwaysRetry;

#[derive(Debug, Clone, PartialEq, Eq)]
/// An answer to the question "should I make a request attempt?"
pub enum ShouldAttempt {
    /// Yes, an attempt should be made
    Yes,
    /// No, no attempt should be made
    No,
    /// Yes, an attempt should be made, but only after the given amount of time has passed
    YesAfterDelay(Duration),
}

#[cfg(feature = "test-util")]
impl ShouldAttempt {
    /// Returns the delay duration if this is a `YesAfterDelay` variant.
    pub fn expect_delay(self) -> Duration {
        match self {
            ShouldAttempt::YesAfterDelay(delay) => delay,
            _ => panic!("Expected this to be the `YesAfterDelay` variant but it was the `{self:?}` variant instead"),
        }
    }

    /// If this isn't a `No` variant, panic.
    pub fn expect_no(self) {
        if ShouldAttempt::No == self {
            return;
        }

        panic!("Expected this to be the `No` variant but it was the `{self:?}` variant instead");
    }
}

impl_shared_conversions!(convert SharedRetryStrategy from RetryStrategy using SharedRetryStrategy::new);

/// Decider for whether or not to attempt a request, and when.
///
/// The orchestrator consults the retry strategy every time before making a request.
/// This includes the initial request, and any retry attempts thereafter. The
/// orchestrator will retry indefinitely (until success) if the retry strategy
/// always returns `ShouldAttempt::Yes` from `should_attempt_retry`.
pub trait RetryStrategy: Send + Sync + fmt::Debug {
    /// Decides if the initial attempt should be made.
    fn should_attempt_initial_request(
        &self,
        runtime_components: &RuntimeComponents,
        cfg: &ConfigBag,
    ) -> Result<ShouldAttempt, BoxError>;

    /// Decides if a retry should be done.
    ///
    /// The previous attempt's output or error are provided in the
    /// [`InterceptorContext`] when this is called.
    ///
    /// `ShouldAttempt::YesAfterDelay` can be used to add a backoff time.
    fn should_attempt_retry(
        &self,
        context: &InterceptorContext,
        runtime_components: &RuntimeComponents,
        cfg: &ConfigBag,
    ) -> Result<ShouldAttempt, BoxError>;
}

/// A shared retry strategy.
#[derive(Clone, Debug)]
pub struct SharedRetryStrategy(Arc<dyn RetryStrategy>);

impl SharedRetryStrategy {
    /// Creates a new [`SharedRetryStrategy`] from a retry strategy.
    pub fn new(retry_strategy: impl RetryStrategy + 'static) -> Self {
        Self(Arc::new(retry_strategy))
    }
}

impl RetryStrategy for SharedRetryStrategy {
    fn should_attempt_initial_request(
        &self,
        runtime_components: &RuntimeComponents,
        cfg: &ConfigBag,
    ) -> Result<ShouldAttempt, BoxError> {
        self.0
            .should_attempt_initial_request(runtime_components, cfg)
    }

    fn should_attempt_retry(
        &self,
        context: &InterceptorContext,
        runtime_components: &RuntimeComponents,
        cfg: &ConfigBag,
    ) -> Result<ShouldAttempt, BoxError> {
        self.0
            .should_attempt_retry(context, runtime_components, cfg)
    }
}

impl ValidateConfig for SharedRetryStrategy {}

/// A type to track the number of requests sent by the orchestrator for a given operation.
///
/// `RequestAttempts` is added to the `ConfigBag` by the orchestrator,
/// and holds the current attempt number.
#[derive(Debug, Clone, Copy)]
pub struct RequestAttempts {
    attempts: u32,
}

impl RequestAttempts {
    /// Creates a new [`RequestAttempts`] with the given number of attempts.
    pub fn new(attempts: u32) -> Self {
        Self { attempts }
    }

    /// Returns the number of attempts.
    pub fn attempts(&self) -> u32 {
        self.attempts
    }
}

impl From<u32> for RequestAttempts {
    fn from(attempts: u32) -> Self {
        Self::new(attempts)
    }
}

impl From<RequestAttempts> for u32 {
    fn from(value: RequestAttempts) -> Self {
        value.attempts()
    }
}

impl Storable for RequestAttempts {
    type Storer = StoreReplace<Self>;
}

#[cfg(feature = "test-util")]
mod test_util {
    use super::ErrorKind;
    use crate::client::interceptors::context::InterceptorContext;
    use crate::client::retries::classifiers::{ClassifyRetry, RetryAction};

    /// A retry classifier for testing purposes. This classifier always returns
    /// `Some(RetryAction::Error(ErrorKind))` where `ErrorKind` is the value provided when creating
    /// this classifier.
    #[derive(Debug)]
    pub struct AlwaysRetry(pub ErrorKind);

    impl ClassifyRetry for AlwaysRetry {
        fn classify_retry(&self, error: &InterceptorContext) -> RetryAction {
            tracing::debug!("Retrying error {:?} as an {:?}", error, self.0);
            RetryAction::retryable_error(self.0)
        }

        fn name(&self) -> &'static str {
            "Always Retry"
        }
    }
}
