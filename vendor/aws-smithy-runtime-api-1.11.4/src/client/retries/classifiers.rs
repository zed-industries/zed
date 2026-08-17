/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Classifiers for determining if a retry is necessary and related code.
//!
//! When a request fails, a retry strategy should inspect the result with retry
//! classifiers to understand if and how the request should be retried.
//!
//! Because multiple classifiers are often used, and because some are more
//! specific than others in what they identify as retryable, classifiers are
//! run in a sequence that is determined by their priority.
//!
//! Classifiers that are higher priority are run **after** classifiers
//! with a lower priority. The intention is that:
//!
//! 1. Generic classifiers that look at things like the HTTP error code run
//!    first.
//! 2. More specific classifiers such as ones that check for certain error
//!    messages are run **after** the generic classifiers. This gives them the
//!    ability to override the actions set by the generic retry classifiers.
//!
//! Put another way:
//!
//! | large nets target common failures with basic behavior | run before            | small nets target specific failures with special behavior|
//! |-------------------------------------------------------|-----------------------|----------------------------------------------------------|
//! | low priority classifiers                              | results overridden by | high priority classifiers                                |

use crate::box_error::BoxError;
use crate::client::interceptors::context::InterceptorContext;
use crate::client::runtime_components::sealed::ValidateConfig;
use crate::client::runtime_components::RuntimeComponents;
use crate::impl_shared_conversions;
use aws_smithy_types::config_bag::ConfigBag;
use aws_smithy_types::retry::ErrorKind;
use std::fmt;
use std::sync::Arc;
use std::time::Duration;

/// The result of running a [`ClassifyRetry`] on a [`InterceptorContext`].
#[non_exhaustive]
#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub enum RetryAction {
    /// When a classifier can't run or has no opinion, this action is returned.
    ///
    /// For example, if a classifier requires a parsed response and response parsing failed,
    /// this action is returned. If all classifiers return this action, no retry should be
    /// attempted.
    #[default]
    NoActionIndicated,
    /// When a classifier runs and thinks a response should be retried, this action is returned.
    RetryIndicated(RetryReason),
    /// When a classifier runs and decides a response must not be retried, this action is returned.
    ///
    /// This action stops retry classification immediately, skipping any following classifiers.
    RetryForbidden,
}

impl fmt::Display for RetryAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoActionIndicated => write!(f, "no action indicated"),
            Self::RetryForbidden => write!(f, "retry forbidden"),
            Self::RetryIndicated(reason) => write!(f, "retry {reason}"),
        }
    }
}

impl RetryAction {
    /// Create a new `RetryAction` indicating that a retry is necessary.
    pub fn retryable_error(kind: ErrorKind) -> Self {
        Self::RetryIndicated(RetryReason::RetryableError {
            kind,
            retry_after: None,
        })
    }

    /// Create a new `RetryAction` indicating that a retry is necessary after an explicit delay.
    pub fn retryable_error_with_explicit_delay(kind: ErrorKind, retry_after: Duration) -> Self {
        Self::RetryIndicated(RetryReason::RetryableError {
            kind,
            retry_after: Some(retry_after),
        })
    }

    /// Create a new `RetryAction` indicating that a retry is necessary because of a transient error.
    pub fn transient_error() -> Self {
        Self::retryable_error(ErrorKind::TransientError)
    }

    /// Create a new `RetryAction` indicating that a retry is necessary because of a throttling error.
    pub fn throttling_error() -> Self {
        Self::retryable_error(ErrorKind::ThrottlingError)
    }

    /// Create a new `RetryAction` indicating that a retry is necessary because of a server error.
    pub fn server_error() -> Self {
        Self::retryable_error(ErrorKind::ServerError)
    }

    /// Create a new `RetryAction` indicating that a retry is necessary because of a client error.
    pub fn client_error() -> Self {
        Self::retryable_error(ErrorKind::ClientError)
    }

    /// Check if a retry is indicated.
    pub fn should_retry(&self) -> bool {
        match self {
            Self::NoActionIndicated | Self::RetryForbidden => false,
            Self::RetryIndicated(_) => true,
        }
    }
}

/// The reason for a retry.
#[non_exhaustive]
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum RetryReason {
    /// When an error is received that should be retried, this reason is returned.
    RetryableError {
        /// The kind of error.
        kind: ErrorKind,
        /// A server may tell us to retry only after a specific time has elapsed.
        retry_after: Option<Duration>,
    },
}

impl fmt::Display for RetryReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RetryableError { kind, retry_after } => {
                let after = retry_after
                    .map(|d| format!(" after {d:?}"))
                    .unwrap_or_default();
                write!(f, "{kind} error{after}")
            }
        }
    }
}

/// The priority of a retry classifier. Classifiers with a higher priority will
/// run **after** classifiers with a lower priority and may override their
/// result. Classifiers with equal priorities make no guarantees about which
/// will run first.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RetryClassifierPriority {
    inner: Inner,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Inner {
    /// The default priority for the `HttpStatusCodeClassifier`.
    HttpStatusCodeClassifier,
    /// The default priority for the `ModeledAsRetryableClassifier`.
    ModeledAsRetryableClassifier,
    /// The default priority for the `TransientErrorClassifier`.
    TransientErrorClassifier,
    /// The priority of some other classifier.
    Other(i8),
}

impl PartialOrd for RetryClassifierPriority {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RetryClassifierPriority {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_i8().cmp(&other.as_i8())
    }
}

impl RetryClassifierPriority {
    /// Create a new `RetryClassifierPriority` with the default priority for the `HttpStatusCodeClassifier`.
    pub fn http_status_code_classifier() -> Self {
        Self {
            inner: Inner::HttpStatusCodeClassifier,
        }
    }

    /// Create a new `RetryClassifierPriority` with the default priority for the `ModeledAsRetryableClassifier`.
    pub fn modeled_as_retryable_classifier() -> Self {
        Self {
            inner: Inner::ModeledAsRetryableClassifier,
        }
    }

    /// Create a new `RetryClassifierPriority` with the default priority for the `TransientErrorClassifier`.
    pub fn transient_error_classifier() -> Self {
        Self {
            inner: Inner::TransientErrorClassifier,
        }
    }

    #[deprecated = "use the less-confusingly-named `RetryClassifierPriority::run_before` instead"]
    /// Create a new `RetryClassifierPriority` with lower priority than the given priority.
    pub fn with_lower_priority_than(other: Self) -> Self {
        Self::run_before(other)
    }

    /// Create a new `RetryClassifierPriority` that can be overridden by the given priority.
    ///
    /// Retry classifiers are run in order from lowest to highest priority. A classifier that
    /// runs later can override a decision from a classifier that runs earlier.
    pub fn run_before(other: Self) -> Self {
        Self {
            inner: Inner::Other(other.as_i8() - 1),
        }
    }

    #[deprecated = "use the less-confusingly-named `RetryClassifierPriority::run_after` instead"]
    /// Create a new `RetryClassifierPriority` with higher priority than the given priority.
    pub fn with_higher_priority_than(other: Self) -> Self {
        Self::run_after(other)
    }

    /// Create a new `RetryClassifierPriority` that can override the given priority.
    ///
    /// Retry classifiers are run in order from lowest to highest priority. A classifier that
    /// runs later can override a decision from a classifier that runs earlier.
    pub fn run_after(other: Self) -> Self {
        Self {
            inner: Inner::Other(other.as_i8() + 1),
        }
    }

    fn as_i8(&self) -> i8 {
        match self.inner {
            Inner::HttpStatusCodeClassifier => 0,
            Inner::ModeledAsRetryableClassifier => 10,
            Inner::TransientErrorClassifier => 20,
            Inner::Other(i) => i,
        }
    }
}

impl Default for RetryClassifierPriority {
    fn default() -> Self {
        Self {
            inner: Inner::Other(0),
        }
    }
}

/// Classifies what kind of retry is needed for a given [`InterceptorContext`].
pub trait ClassifyRetry: Send + Sync + fmt::Debug {
    /// Run this classifier on the [`InterceptorContext`] to determine if the previous request
    /// should be retried. Returns a [`RetryAction`].
    fn classify_retry(&self, ctx: &InterceptorContext) -> RetryAction;

    /// The name of this retry classifier.
    ///
    /// Used for debugging purposes.
    fn name(&self) -> &'static str;

    /// The priority of this retry classifier.
    ///
    /// Classifiers with a higher priority will override the
    /// results of classifiers with a lower priority. Classifiers with equal priorities make no
    /// guarantees about which will override the other.
    ///
    /// Retry classifiers are run in order of increasing priority. Any decision
    /// (return value other than `NoActionIndicated`) from a higher priority
    /// classifier will override the decision of a lower priority classifier with one exception:
    /// [`RetryAction::RetryForbidden`] is treated differently: If ANY classifier returns `RetryForbidden`,
    /// this request will not be retried.
    fn priority(&self) -> RetryClassifierPriority {
        RetryClassifierPriority::default()
    }
}

impl_shared_conversions!(convert SharedRetryClassifier from ClassifyRetry using SharedRetryClassifier::new);

#[derive(Debug, Clone)]
/// Retry classifier used by the retry strategy to classify responses as retryable or not.
pub struct SharedRetryClassifier(Arc<dyn ClassifyRetry>);

impl SharedRetryClassifier {
    /// Given a [`ClassifyRetry`] trait object, create a new `SharedRetryClassifier`.
    pub fn new(retry_classifier: impl ClassifyRetry + 'static) -> Self {
        Self(Arc::new(retry_classifier))
    }
}

impl ClassifyRetry for SharedRetryClassifier {
    fn classify_retry(&self, ctx: &InterceptorContext) -> RetryAction {
        self.0.classify_retry(ctx)
    }

    fn name(&self) -> &'static str {
        self.0.name()
    }

    fn priority(&self) -> RetryClassifierPriority {
        self.0.priority()
    }
}

impl ValidateConfig for SharedRetryClassifier {
    fn validate_final_config(
        &self,
        _runtime_components: &RuntimeComponents,
        _cfg: &ConfigBag,
    ) -> Result<(), BoxError> {
        #[cfg(debug_assertions)]
        {
            // Because this is validating that the implementation is correct rather
            // than validating user input, we only want to run this in debug builds.
            let retry_classifiers = _runtime_components.retry_classifiers_slice();
            let out_of_order: Vec<_> = retry_classifiers
                .windows(2)
                .filter(|&w| w[0].value().priority() > w[1].value().priority())
                .collect();

            if !out_of_order.is_empty() {
                return Err("retry classifiers are mis-ordered; this is a bug".into());
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{ClassifyRetry, RetryAction, RetryClassifierPriority, SharedRetryClassifier};
    use crate::client::interceptors::context::InterceptorContext;

    #[test]
    fn test_preset_priorities() {
        let before_modeled_as_retryable = RetryClassifierPriority::run_before(
            RetryClassifierPriority::modeled_as_retryable_classifier(),
        );
        let mut list = vec![
            RetryClassifierPriority::modeled_as_retryable_classifier(),
            RetryClassifierPriority::http_status_code_classifier(),
            RetryClassifierPriority::transient_error_classifier(),
            before_modeled_as_retryable,
        ];
        list.sort();

        assert_eq!(
            vec![
                RetryClassifierPriority::http_status_code_classifier(),
                before_modeled_as_retryable,
                RetryClassifierPriority::modeled_as_retryable_classifier(),
                RetryClassifierPriority::transient_error_classifier(),
            ],
            list
        );
    }

    #[test]
    fn test_classifier_run_before() {
        // Ensure low-priority classifiers run *before* high-priority classifiers.
        let high_priority_classifier = RetryClassifierPriority::default();
        let mid_priority_classifier = RetryClassifierPriority::run_before(high_priority_classifier);
        let low_priority_classifier = RetryClassifierPriority::run_before(mid_priority_classifier);

        let mut list = vec![
            mid_priority_classifier,
            high_priority_classifier,
            low_priority_classifier,
        ];
        list.sort();

        assert_eq!(
            vec![
                low_priority_classifier,
                mid_priority_classifier,
                high_priority_classifier
            ],
            list
        );
    }

    #[test]
    fn test_classifier_run_after() {
        // Ensure high-priority classifiers run *after* low-priority classifiers.
        let low_priority_classifier = RetryClassifierPriority::default();
        let mid_priority_classifier = RetryClassifierPriority::run_after(low_priority_classifier);
        let high_priority_classifier = RetryClassifierPriority::run_after(mid_priority_classifier);

        let mut list = vec![
            mid_priority_classifier,
            low_priority_classifier,
            high_priority_classifier,
        ];
        list.sort();

        assert_eq!(
            vec![
                low_priority_classifier,
                mid_priority_classifier,
                high_priority_classifier
            ],
            list
        );
    }

    #[derive(Debug)]
    struct ClassifierStub {
        name: &'static str,
        priority: RetryClassifierPriority,
    }

    impl ClassifyRetry for ClassifierStub {
        fn classify_retry(&self, _ctx: &InterceptorContext) -> RetryAction {
            todo!()
        }

        fn name(&self) -> &'static str {
            self.name
        }

        fn priority(&self) -> RetryClassifierPriority {
            self.priority
        }
    }

    fn wrap(name: &'static str, priority: RetryClassifierPriority) -> SharedRetryClassifier {
        SharedRetryClassifier::new(ClassifierStub { name, priority })
    }

    #[test]
    fn test_shared_classifier_run_before() {
        // Ensure low-priority classifiers run *before* high-priority classifiers,
        // even after wrapping.
        let high_priority_classifier = RetryClassifierPriority::default();
        let mid_priority_classifier = RetryClassifierPriority::run_before(high_priority_classifier);
        let low_priority_classifier = RetryClassifierPriority::run_before(mid_priority_classifier);

        let mut list = [
            wrap("mid", mid_priority_classifier),
            wrap("high", high_priority_classifier),
            wrap("low", low_priority_classifier),
        ];
        list.sort_by_key(|rc| rc.priority());

        let actual: Vec<_> = list.iter().map(|it| it.name()).collect();
        assert_eq!(vec!["low", "mid", "high"], actual);
    }

    #[test]
    fn test_shared_classifier_run_after() {
        // Ensure high-priority classifiers run *after* low-priority classifiers,
        // even after wrapping.
        let low_priority_classifier = RetryClassifierPriority::default();
        let mid_priority_classifier = RetryClassifierPriority::run_after(low_priority_classifier);
        let high_priority_classifier = RetryClassifierPriority::run_after(mid_priority_classifier);

        let mut list = [
            wrap("mid", mid_priority_classifier),
            wrap("high", high_priority_classifier),
            wrap("low", low_priority_classifier),
        ];
        list.sort_by_key(|rc| rc.priority());

        let actual: Vec<_> = list.iter().map(|it| it.name()).collect();
        assert_eq!(vec!["low", "mid", "high"], actual);
    }

    #[test]
    fn test_shared_preset_priorities() {
        let before_modeled_as_retryable = RetryClassifierPriority::run_before(
            RetryClassifierPriority::modeled_as_retryable_classifier(),
        );
        let mut list = [
            wrap(
                "modeled as retryable",
                RetryClassifierPriority::modeled_as_retryable_classifier(),
            ),
            wrap(
                "http status code",
                RetryClassifierPriority::http_status_code_classifier(),
            ),
            wrap(
                "transient error",
                RetryClassifierPriority::transient_error_classifier(),
            ),
            wrap("before 'modeled as retryable'", before_modeled_as_retryable),
        ];
        list.sort_by_key(|rc| rc.priority());

        let actual: Vec<_> = list.iter().map(|it| it.name()).collect();
        assert_eq!(
            vec![
                "http status code",
                "before 'modeled as retryable'",
                "modeled as retryable",
                "transient error"
            ],
            actual
        );
    }
}
