/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_smithy_runtime_api::client::interceptors::context::InterceptorContext;
use aws_smithy_runtime_api::client::retries::classifiers::{
    ClassifyRetry, RetryAction, RetryClassifierPriority, SharedRetryClassifier,
};
use aws_smithy_types::retry::ProvideErrorKind;
use std::borrow::Cow;
use std::error::Error as StdError;
use std::marker::PhantomData;

/// A retry classifier for checking if an error is modeled as retryable.
#[derive(Debug, Default)]
pub struct ModeledAsRetryableClassifier<E> {
    _inner: PhantomData<E>,
}

impl<E> ModeledAsRetryableClassifier<E> {
    /// Create a new `ModeledAsRetryableClassifier`
    pub fn new() -> Self {
        Self {
            _inner: PhantomData,
        }
    }

    /// Return the priority of this retry classifier.
    pub fn priority() -> RetryClassifierPriority {
        RetryClassifierPriority::modeled_as_retryable_classifier()
    }
}

impl<E> ClassifyRetry for ModeledAsRetryableClassifier<E>
where
    E: StdError + ProvideErrorKind + Send + Sync + 'static,
{
    fn classify_retry(&self, ctx: &InterceptorContext) -> RetryAction {
        // Check for a result
        let output_or_error = ctx.output_or_error();
        // Check for an error
        let error = match output_or_error {
            Some(Ok(_)) | None => return RetryAction::NoActionIndicated,
            Some(Err(err)) => err,
        };
        // Check that the error is an operation error
        error
            .as_operation_error()
            // Downcast the error
            .and_then(|err| err.downcast_ref::<E>())
            // Check if the error is retryable
            .and_then(|err| err.retryable_error_kind().map(RetryAction::retryable_error))
            .unwrap_or_default()
    }

    fn name(&self) -> &'static str {
        "Errors Modeled As Retryable"
    }

    fn priority(&self) -> RetryClassifierPriority {
        Self::priority()
    }
}

/// Classifies response, timeout, and connector errors as retryable or not.
#[derive(Debug, Default)]
pub struct TransientErrorClassifier<E> {
    _inner: PhantomData<E>,
}

impl<E> TransientErrorClassifier<E> {
    /// Create a new `TransientErrorClassifier`
    pub fn new() -> Self {
        Self {
            _inner: PhantomData,
        }
    }

    /// Return the priority of this retry classifier.
    pub fn priority() -> RetryClassifierPriority {
        RetryClassifierPriority::transient_error_classifier()
    }
}

impl<E> ClassifyRetry for TransientErrorClassifier<E>
where
    E: StdError + Send + Sync + 'static,
{
    fn classify_retry(&self, ctx: &InterceptorContext) -> RetryAction {
        // Check for a result
        let output_or_error = ctx.output_or_error();
        // Check for an error
        let error = match output_or_error {
            Some(Ok(_)) | None => return RetryAction::NoActionIndicated,
            Some(Err(err)) => err,
        };

        if error.is_response_error() || error.is_timeout_error() {
            RetryAction::transient_error()
        } else if let Some(error) = error.as_connector_error() {
            if error.is_timeout() || error.is_io() {
                RetryAction::transient_error()
            } else {
                error
                    .as_other()
                    .map(RetryAction::retryable_error)
                    .unwrap_or_default()
            }
        } else {
            RetryAction::NoActionIndicated
        }
    }

    fn name(&self) -> &'static str {
        "Retryable Smithy Errors"
    }

    fn priority(&self) -> RetryClassifierPriority {
        Self::priority()
    }
}

const TRANSIENT_ERROR_STATUS_CODES: &[u16] = &[500, 502, 503, 504];

/// A retry classifier that will treat HTTP response with those status codes as retryable.
/// The `Default` version will retry 500, 502, 503, and 504 errors.
#[derive(Debug)]
pub struct HttpStatusCodeClassifier {
    retryable_status_codes: Cow<'static, [u16]>,
}

impl Default for HttpStatusCodeClassifier {
    fn default() -> Self {
        Self::new_from_codes(TRANSIENT_ERROR_STATUS_CODES.to_owned())
    }
}

impl HttpStatusCodeClassifier {
    /// Given a `Vec<u16>` where the `u16`s represent status codes, create a `HttpStatusCodeClassifier`
    /// that will treat HTTP response with those status codes as retryable. The `Default` version
    /// will retry 500, 502, 503, and 504 errors.
    pub fn new_from_codes(retryable_status_codes: impl Into<Cow<'static, [u16]>>) -> Self {
        Self {
            retryable_status_codes: retryable_status_codes.into(),
        }
    }

    /// Return the priority of this retry classifier.
    pub fn priority() -> RetryClassifierPriority {
        RetryClassifierPriority::http_status_code_classifier()
    }
}

impl ClassifyRetry for HttpStatusCodeClassifier {
    fn classify_retry(&self, ctx: &InterceptorContext) -> RetryAction {
        let is_retryable = ctx
            .response()
            .map(|res| res.status().into())
            .map(|status| self.retryable_status_codes.contains(&status))
            .unwrap_or_default();

        if is_retryable {
            RetryAction::transient_error()
        } else {
            RetryAction::NoActionIndicated
        }
    }

    fn name(&self) -> &'static str {
        "HTTP Status Code"
    }

    fn priority(&self) -> RetryClassifierPriority {
        Self::priority()
    }
}

/// Given an iterator of retry classifiers and an interceptor context, run retry classifiers on the
/// context. Each classifier is passed the classification result from the previous classifier (the
/// 'root' classifier is passed `None`.)
pub fn run_classifiers_on_ctx(
    classifiers: impl Iterator<Item = SharedRetryClassifier>,
    ctx: &InterceptorContext,
) -> RetryAction {
    // By default, don't retry
    let mut result = RetryAction::NoActionIndicated;

    for classifier in classifiers {
        let new_result = classifier.classify_retry(ctx);

        // If the result is `NoActionIndicated`, continue to the next classifier
        // without overriding any previously-set result.
        if new_result == RetryAction::NoActionIndicated {
            continue;
        }

        // Otherwise, set the result to the new result.
        tracing::trace!(
            "Classifier '{}' set the result of classification to '{}'",
            classifier.name(),
            new_result
        );
        result = new_result;

        // If the result is `RetryForbidden`, stop running classifiers.
        if result == RetryAction::RetryForbidden {
            tracing::trace!("retry classification ending early because a `RetryAction::RetryForbidden` was emitted",);
            break;
        }
    }

    result
}

#[cfg(test)]
mod test {
    use crate::client::retries::classifiers::{
        HttpStatusCodeClassifier, ModeledAsRetryableClassifier,
    };
    use aws_smithy_runtime_api::client::interceptors::context::{Error, Input, InterceptorContext};
    use aws_smithy_runtime_api::client::orchestrator::OrchestratorError;
    use aws_smithy_runtime_api::client::retries::classifiers::{ClassifyRetry, RetryAction};
    use aws_smithy_types::body::SdkBody;
    use aws_smithy_types::retry::{ErrorKind, ProvideErrorKind};
    use std::fmt;

    use super::TransientErrorClassifier;

    #[derive(Debug, PartialEq, Eq, Clone)]
    struct UnmodeledError;

    impl fmt::Display for UnmodeledError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "UnmodeledError")
        }
    }

    impl std::error::Error for UnmodeledError {}

    #[test]
    fn classify_by_response_status() {
        let policy = HttpStatusCodeClassifier::default();
        let res = http_1x::Response::builder()
            .status(500)
            .body("error!")
            .unwrap()
            .map(SdkBody::from);
        let mut ctx = InterceptorContext::new(Input::doesnt_matter());
        ctx.set_response(res.try_into().unwrap());
        assert_eq!(policy.classify_retry(&ctx), RetryAction::transient_error());
    }

    #[test]
    fn classify_by_response_status_not_retryable() {
        let policy = HttpStatusCodeClassifier::default();
        let res = http_1x::Response::builder()
            .status(408)
            .body("error!")
            .unwrap()
            .map(SdkBody::from);
        let mut ctx = InterceptorContext::new(Input::doesnt_matter());
        ctx.set_response(res.try_into().unwrap());
        assert_eq!(policy.classify_retry(&ctx), RetryAction::NoActionIndicated);
    }

    #[test]
    fn classify_by_error_kind() {
        #[derive(Debug)]
        struct RetryableError;

        impl fmt::Display for RetryableError {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "Some retryable error")
            }
        }

        impl ProvideErrorKind for RetryableError {
            fn retryable_error_kind(&self) -> Option<ErrorKind> {
                Some(ErrorKind::ClientError)
            }

            fn code(&self) -> Option<&str> {
                // code should not be called when `error_kind` is provided
                unimplemented!()
            }
        }

        impl std::error::Error for RetryableError {}

        let policy = ModeledAsRetryableClassifier::<RetryableError>::new();
        let mut ctx = InterceptorContext::new(Input::doesnt_matter());
        ctx.set_output_or_error(Err(OrchestratorError::operation(Error::erase(
            RetryableError,
        ))));

        assert_eq!(policy.classify_retry(&ctx), RetryAction::client_error(),);
    }

    #[test]
    fn classify_response_error() {
        let policy = TransientErrorClassifier::<UnmodeledError>::new();
        let mut ctx = InterceptorContext::new(Input::doesnt_matter());
        ctx.set_output_or_error(Err(OrchestratorError::response(
            "I am a response error".into(),
        )));
        assert_eq!(policy.classify_retry(&ctx), RetryAction::transient_error(),);
    }

    #[test]
    fn test_timeout_error() {
        let policy = TransientErrorClassifier::<UnmodeledError>::new();
        let mut ctx = InterceptorContext::new(Input::doesnt_matter());
        ctx.set_output_or_error(Err(OrchestratorError::timeout(
            "I am a timeout error".into(),
        )));
        assert_eq!(policy.classify_retry(&ctx), RetryAction::transient_error(),);
    }
}
