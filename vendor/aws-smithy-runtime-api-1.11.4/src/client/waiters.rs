/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

/// Error types for waiters.
pub mod error {
    use crate::client::{
        orchestrator::HttpResponse,
        result::{ConstructionFailure, SdkError},
    };
    use crate::{box_error::BoxError, client::waiters::FinalPoll};
    use aws_smithy_types::error::{
        metadata::{ProvideErrorMetadata, EMPTY_ERROR_METADATA},
        ErrorMetadata,
    };
    use std::{fmt, time::Duration};

    /// An error occurred while waiting.
    ///
    /// This error type is useful for distinguishing between the max wait
    /// time being exceeded, or some other failure occurring.
    #[derive(Debug)]
    #[non_exhaustive]
    #[allow(clippy::large_enum_variant)] // For `OperationFailed` variant
    pub enum WaiterError<O, E> {
        /// An error occurred during waiter initialization.
        ///
        /// This can happen if the input/config is invalid.
        ConstructionFailure(ConstructionFailure),

        /// The maximum wait time was exceeded without completion.
        ExceededMaxWait(ExceededMaxWait),

        /// Waiting ended in a failure state.
        ///
        /// A failed waiter state can occur on a successful response from the server
        /// if, for example, that response indicates that the thing being waited for
        /// won't succeed/finish.
        ///
        /// A failure state error will only occur for successful or modeled error responses.
        /// Unmodeled error responses will never make it into this error case.
        FailureState(FailureState<O, E>),

        /// A polling operation failed while waiting.
        ///
        /// This error will only occur for unmodeled errors. Modeled errors can potentially
        /// be handled by the waiter logic, and will therefore end up in [`WaiterError::FailureState`].
        ///
        /// Note: If retry is configured, this means that the operation failed
        /// after retrying the configured number of attempts.
        OperationFailed(OperationFailed<E>),
    }

    impl<O, E> WaiterError<O, E> {
        /// Construct a waiter construction failure with the given error source.
        pub fn construction_failure(source: impl Into<BoxError>) -> Self {
            Self::ConstructionFailure(ConstructionFailure::builder().source(source).build())
        }
    }

    impl<O, E> std::error::Error for WaiterError<O, E>
    where
        O: fmt::Debug,
        E: std::error::Error + fmt::Debug + 'static,
    {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            match self {
                Self::ConstructionFailure(inner) => Some(&*inner.source),
                Self::ExceededMaxWait(_) => None,
                Self::FailureState(inner) => match &inner.final_poll.result {
                    Ok(_) => None,
                    Err(err) => Some(err),
                },
                Self::OperationFailed(inner) => Some(&inner.source),
            }
        }
    }

    impl<O, E> fmt::Display for WaiterError<O, E> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::ConstructionFailure(_) => f.write_str("failed to construct waiter"),
                Self::ExceededMaxWait(ctx) => {
                    write!(f, "exceeded max wait time ({:?})", ctx.max_wait)
                }
                Self::FailureState(_) => f.write_str("waiting failed"),
                Self::OperationFailed(_) => f.write_str("operation failed while waiting"),
            }
        }
    }

    // Implement `ProvideErrorMetadata` so that request IDs can be discovered from waiter failures.
    impl<O, E> ProvideErrorMetadata for WaiterError<O, E>
    where
        E: ProvideErrorMetadata,
    {
        fn meta(&self) -> &ErrorMetadata {
            match self {
                WaiterError::ConstructionFailure(_) | WaiterError::ExceededMaxWait(_) => {
                    &EMPTY_ERROR_METADATA
                }
                WaiterError::FailureState(inner) => inner
                    .final_poll()
                    .as_result()
                    .err()
                    .map(ProvideErrorMetadata::meta)
                    .unwrap_or(&EMPTY_ERROR_METADATA),
                WaiterError::OperationFailed(inner) => inner.error().meta(),
            }
        }
    }

    /// Error context for [`WaiterError::ExceededMaxWait`].
    #[derive(Debug)]
    pub struct ExceededMaxWait {
        max_wait: Duration,
        elapsed: Duration,
        poll_count: u32,
    }

    impl ExceededMaxWait {
        /// Creates new error context.
        pub fn new(max_wait: Duration, elapsed: Duration, poll_count: u32) -> Self {
            Self {
                max_wait,
                elapsed,
                poll_count,
            }
        }

        /// Returns the configured max wait time that was exceeded.
        pub fn max_wait(&self) -> Duration {
            self.max_wait
        }

        /// How much time actually elapsed before max wait was triggered.
        pub fn elapsed(&self) -> Duration {
            self.elapsed
        }

        /// Returns the number of polling operations that were made before exceeding the max wait time.
        pub fn poll_count(&self) -> u32 {
            self.poll_count
        }
    }

    /// Error context for [`WaiterError::FailureState`].
    #[derive(Debug)]
    #[non_exhaustive]
    pub struct FailureState<O, E> {
        final_poll: FinalPoll<O, E>,
    }

    impl<O, E> FailureState<O, E> {
        /// Creates new error context given a final poll result.
        pub fn new(final_poll: FinalPoll<O, E>) -> Self {
            Self { final_poll }
        }

        /// Returns the result of the final polling attempt.
        pub fn final_poll(&self) -> &FinalPoll<O, E> {
            &self.final_poll
        }

        /// Grants ownership of the result of the final polling attempt.
        pub fn into_final_poll(self) -> FinalPoll<O, E> {
            self.final_poll
        }
    }

    /// Error context for [`WaiterError::OperationFailed`].
    #[derive(Debug)]
    #[non_exhaustive]
    pub struct OperationFailed<E> {
        source: SdkError<E, HttpResponse>,
    }

    impl<E> OperationFailed<E> {
        /// Creates new error context given a source [`SdkError`].
        pub fn new(source: SdkError<E, HttpResponse>) -> Self {
            Self { source }
        }

        /// Returns the underlying source [`SdkError`].
        pub fn error(&self) -> &SdkError<E, HttpResponse> {
            &self.source
        }

        /// Grants ownership of the underlying source [`SdkError`].
        pub fn into_error(self) -> SdkError<E, HttpResponse> {
            self.source
        }
    }
}

/// Result of the final polling attempt made by a waiter.
///
/// Waiters make several requests ("polls") to the remote service, and this
/// struct holds the result of the final poll attempt that was made by the
/// waiter so that it can be inspected.
#[non_exhaustive]
#[derive(Debug)]
pub struct FinalPoll<O, E> {
    result: Result<O, E>,
}

impl<O, E> FinalPoll<O, E> {
    /// Creates a new `FinalPoll` from a result.
    pub fn new(result: Result<O, E>) -> Self {
        Self { result }
    }

    /// Grants ownership of the underlying result.
    pub fn into_result(self) -> Result<O, E> {
        self.result
    }

    /// Returns the underlying result.
    pub fn as_result(&self) -> Result<&O, &E> {
        self.result.as_ref()
    }

    /// Maps the operation type with a function.
    pub fn map<O2, F: FnOnce(O) -> O2>(self, mapper: F) -> FinalPoll<O2, E> {
        FinalPoll::new(self.result.map(mapper))
    }

    /// Maps the error type with a function.
    pub fn map_err<E2, F: FnOnce(E) -> E2>(self, mapper: F) -> FinalPoll<O, E2> {
        FinalPoll::new(self.result.map_err(mapper))
    }
}
