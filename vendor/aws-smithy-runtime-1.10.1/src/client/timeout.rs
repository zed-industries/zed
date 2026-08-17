/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_smithy_async::future::timeout::Timeout;
use aws_smithy_async::rt::sleep::{AsyncSleep, SharedAsyncSleep, Sleep};
use aws_smithy_runtime_api::client::orchestrator::HttpResponse;
use aws_smithy_runtime_api::client::result::SdkError;
use aws_smithy_runtime_api::client::runtime_components::RuntimeComponents;
use aws_smithy_types::config_bag::ConfigBag;
use aws_smithy_types::timeout::TimeoutConfig;
use pin_project_lite::pin_project;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

#[derive(Debug)]
struct MaybeTimeoutError {
    kind: TimeoutKind,
    duration: Duration,
}

impl MaybeTimeoutError {
    fn new(kind: TimeoutKind, duration: Duration) -> Self {
        Self { kind, duration }
    }
}

impl std::fmt::Display for MaybeTimeoutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} occurred after {:?}",
            match self.kind {
                TimeoutKind::Operation => "operation timeout (all attempts including retries)",
                TimeoutKind::OperationAttempt => "operation attempt timeout (single attempt)",
            },
            self.duration
        )
    }
}

impl std::error::Error for MaybeTimeoutError {}

pin_project! {
    #[non_exhaustive]
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    // This allow is needed because otherwise Clippy will get mad we didn't document the
    // generated MaybeTimeoutFutureProj
    #[allow(missing_docs)]
    #[project = MaybeTimeoutFutureProj]
    /// A timeout future that may or may not have a timeout depending on
    /// whether or not one was set. A `kind` can be set so that when a timeout occurs, there
    /// is additional context attached to the error.
    pub(super) enum MaybeTimeoutFuture<F> {
        /// A wrapper around an inner future that will output an [`SdkError`] if it runs longer than
        /// the given duration
        Timeout {
            #[pin]
            future: Timeout<F, Sleep>,
            timeout_kind: TimeoutKind,
            duration: Duration,
        },
        /// A thin wrapper around an inner future that will never time out
        NoTimeout {
            #[pin]
            future: F
        }
    }
}

impl<InnerFuture, T, E> Future for MaybeTimeoutFuture<InnerFuture>
where
    InnerFuture: Future<Output = Result<T, SdkError<E, HttpResponse>>>,
{
    type Output = Result<T, SdkError<E, HttpResponse>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let (future, kind, duration) = match self.project() {
            MaybeTimeoutFutureProj::NoTimeout { future } => return future.poll(cx),
            MaybeTimeoutFutureProj::Timeout {
                future,
                timeout_kind,
                duration,
            } => (future, timeout_kind, duration),
        };
        match future.poll(cx) {
            Poll::Ready(Ok(response)) => Poll::Ready(response),
            Poll::Ready(Err(_timeout)) => Poll::Ready(Err(SdkError::timeout_error(
                MaybeTimeoutError::new(*kind, *duration),
            ))),
            Poll::Pending => Poll::Pending,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(super) enum TimeoutKind {
    Operation,
    OperationAttempt,
}

#[derive(Clone, Debug)]
pub(super) struct MaybeTimeoutConfig {
    sleep_impl: Option<SharedAsyncSleep>,
    timeout: Option<Duration>,
    timeout_kind: TimeoutKind,
}

impl MaybeTimeoutConfig {
    pub(super) fn new(
        runtime_components: &RuntimeComponents,
        cfg: &ConfigBag,
        timeout_kind: TimeoutKind,
    ) -> MaybeTimeoutConfig {
        if let Some(timeout_config) = cfg.load::<TimeoutConfig>() {
            let sleep_impl = runtime_components.sleep_impl();
            let timeout = match (sleep_impl.as_ref(), timeout_kind) {
                (None, _) => None,
                (Some(_), TimeoutKind::Operation) => timeout_config.operation_timeout(),
                (Some(_), TimeoutKind::OperationAttempt) => {
                    timeout_config.operation_attempt_timeout()
                }
            };
            MaybeTimeoutConfig {
                sleep_impl,
                timeout,
                timeout_kind,
            }
        } else {
            MaybeTimeoutConfig {
                sleep_impl: None,
                timeout: None,
                timeout_kind,
            }
        }
    }
}

/// Trait to conveniently wrap a future with an optional timeout.
pub(super) trait MaybeTimeout<T>: Sized {
    /// Wraps a future in a timeout if one is set.
    fn maybe_timeout(self, timeout_config: MaybeTimeoutConfig) -> MaybeTimeoutFuture<Self>;
}

impl<T> MaybeTimeout<T> for T
where
    T: Future,
{
    fn maybe_timeout(self, timeout_config: MaybeTimeoutConfig) -> MaybeTimeoutFuture<Self> {
        match timeout_config {
            MaybeTimeoutConfig {
                sleep_impl: Some(sleep_impl),
                timeout: Some(timeout),
                timeout_kind,
            } => MaybeTimeoutFuture::Timeout {
                future: Timeout::new(self, sleep_impl.sleep(timeout)),
                timeout_kind,
                duration: timeout,
            },
            _ => MaybeTimeoutFuture::NoTimeout { future: self },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aws_smithy_async::assert_elapsed;
    use aws_smithy_async::future::never::Never;
    use aws_smithy_async::rt::sleep::{AsyncSleep, SharedAsyncSleep, TokioSleep};
    use aws_smithy_runtime_api::client::orchestrator::HttpResponse;
    use aws_smithy_runtime_api::client::result::SdkError;
    use aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder;
    use aws_smithy_types::config_bag::{CloneableLayer, ConfigBag};
    use aws_smithy_types::timeout::TimeoutConfig;
    use std::time::Duration;

    #[tokio::test]
    async fn test_no_timeout() {
        let sleep_impl = SharedAsyncSleep::new(TokioSleep::new());
        let sleep_future = sleep_impl.sleep(Duration::from_millis(250));
        let underlying_future = async {
            sleep_future.await;
            Result::<_, SdkError<(), HttpResponse>>::Ok(())
        };

        let now = tokio::time::Instant::now();
        tokio::time::pause();

        let runtime_components = RuntimeComponentsBuilder::for_tests()
            .with_sleep_impl(Some(sleep_impl))
            .build()
            .unwrap();

        let mut timeout_config = CloneableLayer::new("timeout");
        timeout_config.store_put(TimeoutConfig::builder().build());
        let cfg = ConfigBag::of_layers(vec![timeout_config.into()]);

        let maybe_timeout =
            MaybeTimeoutConfig::new(&runtime_components, &cfg, TimeoutKind::Operation);
        underlying_future
            .maybe_timeout(maybe_timeout)
            .await
            .expect("success");

        assert_elapsed!(now, Duration::from_secs_f32(0.25));
    }

    #[tokio::test]
    async fn test_operation_timeout() {
        let sleep_impl = SharedAsyncSleep::new(TokioSleep::new());
        let never = Never::new();
        let underlying_future = async {
            never.await;
            Result::<_, SdkError<(), HttpResponse>>::Ok(())
        };

        let now = tokio::time::Instant::now();
        tokio::time::pause();

        let runtime_components = RuntimeComponentsBuilder::for_tests()
            .with_sleep_impl(Some(sleep_impl))
            .build()
            .unwrap();
        let mut timeout_config = CloneableLayer::new("timeout");
        timeout_config.store_put(
            TimeoutConfig::builder()
                .operation_timeout(Duration::from_millis(250))
                .build(),
        );
        let cfg = ConfigBag::of_layers(vec![timeout_config.into()]);

        let maybe_timeout =
            MaybeTimeoutConfig::new(&runtime_components, &cfg, TimeoutKind::Operation);
        let result = underlying_future.maybe_timeout(maybe_timeout).await;
        let err = result.expect_err("should have timed out");

        assert_eq!(format!("{err:?}"), "TimeoutError(TimeoutError { source: MaybeTimeoutError { kind: Operation, duration: 250ms } })");
        assert_elapsed!(now, Duration::from_secs_f32(0.25));
    }
}
