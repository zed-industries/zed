/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_smithy_runtime_api::client::interceptors::context::InterceptorContext;
use aws_smithy_runtime_api::client::orchestrator::OrchestratorError;
use aws_smithy_runtime_api::client::retries::classifiers::{
    ClassifyRetry, RetryAction, RetryClassifierPriority, RetryReason,
};
use aws_smithy_types::error::metadata::ProvideErrorMetadata;
use aws_smithy_types::retry::ErrorKind;
use std::borrow::Cow;
use std::error::Error as StdError;
use std::marker::PhantomData;

/// AWS error codes that represent throttling errors.
pub const THROTTLING_ERRORS: &[&str] = &[
    "Throttling",
    "ThrottlingException",
    "ThrottledException",
    "RequestThrottledException",
    "TooManyRequestsException",
    "ProvisionedThroughputExceededException",
    "TransactionInProgressException",
    "RequestLimitExceeded",
    "BandwidthLimitExceeded",
    "LimitExceededException",
    "RequestThrottled",
    "SlowDown",
    "PriorRequestNotComplete",
    "EC2ThrottledException",
];

/// AWS error codes that represent transient errors.
pub const TRANSIENT_ERRORS: &[&str] = &["RequestTimeout", "RequestTimeoutException"];

/// A retry classifier for determining if the response sent by an AWS service requires a retry.
#[derive(Debug)]
pub struct AwsErrorCodeClassifier<E> {
    throttling_errors: Cow<'static, [&'static str]>,
    transient_errors: Cow<'static, [&'static str]>,
    _inner: PhantomData<E>,
}

impl<E> Default for AwsErrorCodeClassifier<E> {
    fn default() -> Self {
        Self {
            throttling_errors: THROTTLING_ERRORS.into(),
            transient_errors: TRANSIENT_ERRORS.into(),
            _inner: PhantomData,
        }
    }
}

/// Builder for [`AwsErrorCodeClassifier`]
#[derive(Debug)]
pub struct AwsErrorCodeClassifierBuilder<E> {
    throttling_errors: Option<Cow<'static, [&'static str]>>,
    transient_errors: Option<Cow<'static, [&'static str]>>,
    _inner: PhantomData<E>,
}

impl<E> AwsErrorCodeClassifierBuilder<E> {
    /// Set `transient_errors` for the builder
    pub fn transient_errors(
        mut self,
        transient_errors: impl Into<Cow<'static, [&'static str]>>,
    ) -> Self {
        self.transient_errors = Some(transient_errors.into());
        self
    }

    /// Build a new [`AwsErrorCodeClassifier`]
    pub fn build(self) -> AwsErrorCodeClassifier<E> {
        AwsErrorCodeClassifier {
            throttling_errors: self.throttling_errors.unwrap_or(THROTTLING_ERRORS.into()),
            transient_errors: self.transient_errors.unwrap_or(TRANSIENT_ERRORS.into()),
            _inner: self._inner,
        }
    }
}

impl<E> AwsErrorCodeClassifier<E> {
    /// Create a new [`AwsErrorCodeClassifier`]
    pub fn new() -> Self {
        Self::default()
    }

    /// Return a builder that can create a new [`AwsErrorCodeClassifier`]
    pub fn builder() -> AwsErrorCodeClassifierBuilder<E> {
        AwsErrorCodeClassifierBuilder {
            throttling_errors: None,
            transient_errors: None,
            _inner: PhantomData,
        }
    }
}

impl<E> ClassifyRetry for AwsErrorCodeClassifier<E>
where
    E: StdError + ProvideErrorMetadata + Send + Sync + 'static,
{
    fn classify_retry(&self, ctx: &InterceptorContext) -> RetryAction {
        // Check for a result
        let output_or_error = ctx.output_or_error();
        // Check for an error
        let error = match output_or_error {
            Some(Ok(_)) | None => return RetryAction::NoActionIndicated,
            Some(Err(err)) => err,
        };

        let retry_after = ctx
            .response()
            .and_then(|res| res.headers().get("x-amz-retry-after"))
            .and_then(|header| header.parse::<u64>().ok())
            .map(std::time::Duration::from_millis);

        let error_code = OrchestratorError::as_operation_error(error)
            .and_then(|err| err.downcast_ref::<E>())
            .and_then(|err| err.code());

        if let Some(error_code) = error_code {
            if self.throttling_errors.contains(&error_code) {
                return RetryAction::RetryIndicated(RetryReason::RetryableError {
                    kind: ErrorKind::ThrottlingError,
                    retry_after,
                });
            }
            if self.transient_errors.contains(&error_code) {
                return RetryAction::RetryIndicated(RetryReason::RetryableError {
                    kind: ErrorKind::TransientError,
                    retry_after,
                });
            }
        };

        debug_assert!(
            retry_after.is_none(),
            "retry_after should be None if the error wasn't an identifiable AWS error"
        );

        RetryAction::NoActionIndicated
    }

    fn name(&self) -> &'static str {
        "AWS Error Code"
    }

    fn priority(&self) -> RetryClassifierPriority {
        RetryClassifierPriority::run_before(
            RetryClassifierPriority::modeled_as_retryable_classifier(),
        )
    }
}

#[cfg(test)]
mod test {
    use crate::retries::classifiers::AwsErrorCodeClassifier;
    use aws_smithy_runtime_api::client::interceptors::context::InterceptorContext;
    use aws_smithy_runtime_api::client::interceptors::context::{Error, Input};
    use aws_smithy_runtime_api::client::orchestrator::OrchestratorError;
    use aws_smithy_runtime_api::client::retries::classifiers::{ClassifyRetry, RetryAction};
    use aws_smithy_types::body::SdkBody;
    use aws_smithy_types::error::metadata::ProvideErrorMetadata;
    use aws_smithy_types::error::ErrorMetadata;
    use aws_smithy_types::retry::ErrorKind;
    use std::fmt;
    use std::time::Duration;

    #[derive(Debug)]
    struct CodedError {
        metadata: ErrorMetadata,
    }

    impl CodedError {
        fn new(code: &'static str) -> Self {
            Self {
                metadata: ErrorMetadata::builder().code(code).build(),
            }
        }
    }

    impl fmt::Display for CodedError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Coded Error")
        }
    }

    impl std::error::Error for CodedError {}

    impl ProvideErrorMetadata for CodedError {
        fn meta(&self) -> &ErrorMetadata {
            &self.metadata
        }
    }

    #[test]
    fn classify_by_error_code() {
        let policy = AwsErrorCodeClassifier::<CodedError>::new();
        let mut ctx = InterceptorContext::new(Input::doesnt_matter());
        ctx.set_output_or_error(Err(OrchestratorError::operation(Error::erase(
            CodedError::new("Throttling"),
        ))));

        assert_eq!(policy.classify_retry(&ctx), RetryAction::throttling_error());

        let mut ctx = InterceptorContext::new(Input::doesnt_matter());
        ctx.set_output_or_error(Err(OrchestratorError::operation(Error::erase(
            CodedError::new("RequestTimeout"),
        ))));
        assert_eq!(policy.classify_retry(&ctx), RetryAction::transient_error())
    }

    #[test]
    fn classify_generic() {
        let policy = AwsErrorCodeClassifier::<ErrorMetadata>::new();
        let err = ErrorMetadata::builder().code("SlowDown").build();
        let test_response = http_1x::Response::new("OK").map(SdkBody::from);

        let mut ctx = InterceptorContext::new(Input::doesnt_matter());
        ctx.set_response(test_response.try_into().unwrap());
        ctx.set_output_or_error(Err(OrchestratorError::operation(Error::erase(err))));

        assert_eq!(policy.classify_retry(&ctx), RetryAction::throttling_error());
    }

    #[test]
    fn test_retry_after_header() {
        let policy = AwsErrorCodeClassifier::<ErrorMetadata>::new();
        let err = ErrorMetadata::builder().code("SlowDown").build();
        let res = http_1x::Response::builder()
            .header("x-amz-retry-after", "5000")
            .body("retry later")
            .unwrap()
            .map(SdkBody::from);
        let mut ctx = InterceptorContext::new(Input::doesnt_matter());
        ctx.set_response(res.try_into().unwrap());
        ctx.set_output_or_error(Err(OrchestratorError::operation(Error::erase(err))));

        assert_eq!(
            policy.classify_retry(&ctx),
            RetryAction::retryable_error_with_explicit_delay(
                ErrorKind::ThrottlingError,
                Duration::from_secs(5)
            )
        );
    }
}
