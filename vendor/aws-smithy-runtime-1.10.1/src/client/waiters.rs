/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::client::waiters::backoff::{Backoff, RandomImpl};
use aws_smithy_async::{
    rt::sleep::{AsyncSleep, SharedAsyncSleep},
    time::SharedTimeSource,
};
use aws_smithy_runtime_api::client::waiters::FinalPoll;
use aws_smithy_runtime_api::client::{orchestrator::HttpResponse, result::SdkError};
use aws_smithy_runtime_api::client::{
    result::CreateUnhandledError,
    waiters::error::{ExceededMaxWait, FailureState, OperationFailed, WaiterError},
};
use std::future::Future;
use std::time::Duration;

mod backoff;

/// Waiter acceptor state
///
/// This enum (vaguely) matches the [acceptor state] from the Smithy spec.
/// It has an additional `NoAcceptorsMatched` variant to indicate the case where
/// none of the modeled waiters matched the response, which the spec mentions but
/// doesn't consider an official part of the acceptor state enum. An `Option<AcceptorState>`
/// could have been used instead, but this seemed cleaner.
///
/// [acceptor state]: https://smithy.io/2.0/additional-specs/waiters.html#acceptorstate-enum
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AcceptorState {
    /// None of the modeled acceptors matched the response.
    NoAcceptorsMatched,
    /// A `success` acceptor matched the response.
    Success,
    /// A `failure` acceptor matched the response.
    Failure,
    /// A `retry` acceptor matched the response.
    Retry,
}

/// Orchestrates waiting via polling with jittered exponential backoff.
///
/// This is meant to be used internally by the generated code to provide
/// waiter functionality.
pub struct WaiterOrchestrator<AcceptorFn, OperationFn> {
    backoff: Backoff,
    time_source: SharedTimeSource,
    sleep_impl: SharedAsyncSleep,
    acceptor_fn: AcceptorFn,
    operation_fn: OperationFn,
}

impl WaiterOrchestrator<(), ()> {
    /// Returns a builder for the waiter orchestrator.
    pub fn builder() -> WaiterOrchestratorBuilder<(), ()> {
        WaiterOrchestratorBuilder::default()
    }
}

impl<AcceptorFn, OperationFn> WaiterOrchestrator<AcceptorFn, OperationFn> {
    fn new(
        backoff: Backoff,
        time_source: SharedTimeSource,
        sleep_impl: SharedAsyncSleep,
        acceptor_fn: AcceptorFn,
        operation_fn: OperationFn,
    ) -> Self {
        WaiterOrchestrator {
            backoff,
            time_source,
            sleep_impl,
            acceptor_fn,
            operation_fn,
        }
    }
}

impl<AcceptorFn, OperationFn, O, E, Fut> WaiterOrchestrator<AcceptorFn, OperationFn>
where
    AcceptorFn: Fn(Result<&O, &E>) -> AcceptorState,
    OperationFn: Fn() -> Fut,
    Fut: Future<Output = Result<O, SdkError<E, HttpResponse>>>,
    E: CreateUnhandledError + std::error::Error + Send + Sync + 'static,
{
    /// Orchestrates waiting via polling with jittered exponential backoff.
    pub async fn orchestrate(
        self,
    ) -> Result<FinalPoll<O, SdkError<E, HttpResponse>>, WaiterError<O, E>> {
        let start_time = self.time_source.now();
        let mut attempt = 0;
        let mut done_retrying = false;
        loop {
            tracing::debug!("executing waiter poll attempt #{}", attempt + 1);
            let result = (self.operation_fn)().await;
            let error = result.is_err();

            // "acceptable result" in this context means "an acceptor's matcher can match this result type"
            let acceptable_result = result.as_ref().map_err(|err| err.as_service_error());
            let acceptor_state = match acceptable_result {
                Ok(output) => (self.acceptor_fn)(Ok(output)),
                Err(Some(err)) => (self.acceptor_fn)(Err(err)),
                _ => {
                    // If we got an unmatchable failure (basically anything unmodeled), then just immediately exit
                    return Err(WaiterError::OperationFailed(OperationFailed::new(
                        result.err().expect("can only be an err in this branch"),
                    )));
                }
            };

            tracing::debug!("waiter acceptor state: {acceptor_state:?}");
            match acceptor_state {
                AcceptorState::Success => return Ok(FinalPoll::new(result)),
                AcceptorState::Failure => {
                    return Err(WaiterError::FailureState(FailureState::new(
                        FinalPoll::new(result.map_err(|err| err.into_service_error())),
                    )))
                }
                // This occurs when there was a modeled error response, but none of the acceptors matched it
                AcceptorState::NoAcceptorsMatched if error => {
                    return Err(WaiterError::OperationFailed(OperationFailed::new(
                        result.err().expect("checked above"),
                    )))
                }
                AcceptorState::Retry | AcceptorState::NoAcceptorsMatched => {
                    attempt += 1;

                    let now = self.time_source.now();
                    let elapsed = now.duration_since(start_time).unwrap_or_default();
                    if !done_retrying && elapsed <= self.backoff.max_wait() {
                        let delay = self.backoff.delay(attempt, elapsed);

                        // The backoff function returns a zero delay when it is min_delay time away
                        // from max_time. If we didn't detect this and stop polling, then we could
                        // slam the server at the very end of the wait period for servers that are
                        // really fast (for example, a few milliseconds total round-trip latency).
                        if delay.is_zero() {
                            tracing::debug!(
                                "delay calculated for attempt #{attempt}; elapsed ({elapsed:?}); waiter is close to max time; will immediately poll one last time"
                            );
                            done_retrying = true;
                        } else {
                            tracing::debug!(
                                "delay calculated for attempt #{attempt}; elapsed ({elapsed:?}); waiter will poll again in {delay:?}"
                            );
                            self.sleep_impl.sleep(delay).await;
                        }
                    } else {
                        tracing::debug!(
                            "waiter exceeded max wait time of {:?}",
                            self.backoff.max_wait()
                        );
                        return Err(WaiterError::ExceededMaxWait(ExceededMaxWait::new(
                            self.backoff.max_wait(),
                            elapsed,
                            attempt,
                        )));
                    }
                }
            }
        }
    }
}

/// Builder for [`WaiterOrchestrator`].
#[derive(Default)]
pub struct WaiterOrchestratorBuilder<AcceptorFn = (), OperationFn = ()> {
    min_delay: Option<Duration>,
    max_delay: Option<Duration>,
    max_wait: Option<Duration>,
    time_source: Option<SharedTimeSource>,
    sleep_impl: Option<SharedAsyncSleep>,
    random_fn: RandomImpl,
    acceptor_fn: Option<AcceptorFn>,
    operation_fn: Option<OperationFn>,
}

impl<AcceptorFn, OperationFn> WaiterOrchestratorBuilder<AcceptorFn, OperationFn> {
    /// Set the minimum delay time for the waiter.
    pub fn min_delay(mut self, min_delay: Duration) -> Self {
        self.min_delay = Some(min_delay);
        self
    }

    /// Set the maximum delay time for the waiter.
    pub fn max_delay(mut self, max_delay: Duration) -> Self {
        self.max_delay = Some(max_delay);
        self
    }

    /// Set the maximum total wait time for the waiter.
    pub fn max_wait(mut self, max_wait: Duration) -> Self {
        self.max_wait = Some(max_wait);
        self
    }

    #[cfg(all(test, any(feature = "test-util", feature = "legacy-test-util")))]
    fn random(mut self, random_fn: impl Fn(u64, u64) -> u64 + Send + Sync + 'static) -> Self {
        self.random_fn = RandomImpl::Override(Box::new(random_fn));
        self
    }

    /// Set the time source the waiter will use.
    pub fn time_source(mut self, time_source: SharedTimeSource) -> Self {
        self.time_source = Some(time_source);
        self
    }

    /// Set the async sleep implementation the waiter will use to delay.
    pub fn sleep_impl(mut self, sleep_impl: SharedAsyncSleep) -> Self {
        self.sleep_impl = Some(sleep_impl);
        self
    }

    /// Build a waiter orchestrator.
    pub fn build(self) -> WaiterOrchestrator<AcceptorFn, OperationFn> {
        WaiterOrchestrator::new(
            Backoff::new(
                self.min_delay.expect("min delay is required"),
                self.max_delay.expect("max delay is required"),
                self.max_wait.expect("max wait is required"),
                self.random_fn,
            ),
            self.time_source.expect("time source required"),
            self.sleep_impl.expect("sleep impl required"),
            self.acceptor_fn.expect("acceptor fn required"),
            self.operation_fn.expect("operation fn required"),
        )
    }
}

impl<OperationFn> WaiterOrchestratorBuilder<(), OperationFn> {
    /// Set the acceptor function for the waiter.
    pub fn acceptor<AcceptorFn>(
        self,
        acceptor: AcceptorFn,
    ) -> WaiterOrchestratorBuilder<AcceptorFn, OperationFn> {
        WaiterOrchestratorBuilder {
            min_delay: self.min_delay,
            max_delay: self.max_delay,
            max_wait: self.max_wait,
            time_source: self.time_source,
            sleep_impl: self.sleep_impl,
            random_fn: self.random_fn,
            acceptor_fn: Some(acceptor),
            operation_fn: self.operation_fn,
        }
    }
}

impl<AcceptorFn> WaiterOrchestratorBuilder<AcceptorFn, ()> {
    /// Set the operation function for the waiter.
    pub fn operation<OperationFn>(
        self,
        operation: OperationFn,
    ) -> WaiterOrchestratorBuilder<AcceptorFn, OperationFn> {
        WaiterOrchestratorBuilder {
            min_delay: self.min_delay,
            max_delay: self.max_delay,
            max_wait: self.max_wait,
            time_source: self.time_source,
            sleep_impl: self.sleep_impl,
            random_fn: self.random_fn,
            acceptor_fn: self.acceptor_fn,
            operation_fn: Some(operation),
        }
    }
}

/// Attaches a tracing span with a semi-unique waiter ID number so that all the operations
/// made by the waiter can be correlated together in logs.
pub fn attach_waiter_tracing_span<O, E>(
    future: impl Future<Output = Result<FinalPoll<O, SdkError<E, HttpResponse>>, WaiterError<O, E>>>,
) -> impl Future<Output = Result<FinalPoll<O, SdkError<E, HttpResponse>>, WaiterError<O, E>>> {
    use tracing::Instrument;

    // Create a random seven-digit ID for the waiter so that it can be correlated in the logs.
    let span = tracing::debug_span!("waiter", waiter_id = fastrand::u32(1_000_000..10_000_000));
    future.instrument(span)
}

#[cfg(all(test, any(feature = "test-util", feature = "legacy-test-util")))]
mod tests {
    use super::*;
    use crate::test_util::capture_test_logs::show_test_logs;
    use aws_smithy_async::{
        test_util::tick_advance_sleep::tick_advance_time_and_sleep, time::TimeSource,
    };
    use aws_smithy_runtime_api::{http::StatusCode, shared::IntoShared};
    use aws_smithy_types::body::SdkBody;
    use std::{
        fmt,
        sync::{
            atomic::{AtomicUsize, Ordering},
            Arc, Mutex,
        },
        time::SystemTime,
    };

    #[derive(Debug)]
    struct TestError;
    impl std::error::Error for TestError {}
    impl fmt::Display for TestError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("TestError")
        }
    }
    impl CreateUnhandledError for TestError {
        fn create_unhandled_error(
            _source: Box<dyn std::error::Error + Send + Sync + 'static>,
            _meta: Option<aws_smithy_types::error::ErrorMetadata>,
        ) -> Self {
            unreachable!("If this is called, there is a bug in the orchestrator implementation. Unmodeled errors should never make it into FailureState.")
        }
    }

    fn test_orchestrator(
        sleep_impl: impl IntoShared<SharedAsyncSleep>,
        time_source: impl IntoShared<SharedTimeSource>,
    ) -> WaiterOrchestratorBuilder<(), ()> {
        let test_random = |min: u64, max: u64| (min + max) / 2;
        WaiterOrchestrator::builder()
            .min_delay(Duration::from_secs(2))
            .max_delay(Duration::from_secs(120))
            .max_wait(Duration::from_secs(300))
            .random(test_random)
            .sleep_impl(sleep_impl.into_shared())
            .time_source(time_source.into_shared())
    }

    #[tokio::test]
    async fn immediate_success() {
        let _logs = show_test_logs();
        let (time_source, sleep_impl) = tick_advance_time_and_sleep();
        let orchestrator = test_orchestrator(sleep_impl, time_source)
            .acceptor(|_result: Result<&usize, &TestError>| AcceptorState::Success)
            .operation(|| async { Result::<_, SdkError<TestError, HttpResponse>>::Ok(5usize) })
            .build();

        let result = orchestrator.orchestrate().await;
        assert!(result.is_ok());
        assert_eq!(5, *result.unwrap().as_result().unwrap());
    }

    #[tokio::test]
    async fn immediate_failure() {
        let _logs = show_test_logs();
        let (time_source, sleep_impl) = tick_advance_time_and_sleep();
        let orchestrator = test_orchestrator(sleep_impl, time_source)
            .acceptor(|_result: Result<&usize, &TestError>| AcceptorState::Failure)
            .operation(|| async { Result::<_, SdkError<TestError, HttpResponse>>::Ok(5usize) })
            .build();

        let result = orchestrator.orchestrate().await;
        assert!(
            matches!(result, Err(WaiterError::FailureState(_))),
            "expected failure state, got: {result:?}"
        );
    }

    #[tokio::test]
    async fn five_polls_then_success() {
        let _logs = show_test_logs();

        let (time_source, sleep_impl) = tick_advance_time_and_sleep();

        let acceptor = |result: Result<&usize, &TestError>| match result {
            Err(_) => unreachable!(),
            Ok(5) => AcceptorState::Success,
            _ => AcceptorState::Retry,
        };

        let times = Arc::new(Mutex::new(Vec::new()));
        let attempt = Arc::new(AtomicUsize::new(1));
        let operation = {
            let sleep_impl = sleep_impl.clone();
            let time_source = time_source.clone();
            let times = times.clone();
            move || {
                let attempt = attempt.clone();
                let sleep_impl = sleep_impl.clone();
                let time_source = time_source.clone();
                let times = times.clone();
                async move {
                    // simulate time passing for the network hop/service processing time
                    sleep_impl.sleep(Duration::from_secs(1)).await;
                    times.lock().unwrap().push(
                        time_source
                            .now()
                            .duration_since(SystemTime::UNIX_EPOCH)
                            .unwrap()
                            .as_secs(),
                    );
                    Result::<_, SdkError<TestError, HttpResponse>>::Ok(
                        attempt.fetch_add(1, Ordering::SeqCst),
                    )
                }
            }
        };

        let orchestrator = test_orchestrator(sleep_impl.clone(), time_source.clone())
            .acceptor(acceptor)
            .operation(operation)
            .build();

        let task = tokio::spawn(orchestrator.orchestrate());
        tokio::task::yield_now().await;
        time_source.tick(Duration::from_secs(500)).await;
        let result = task.await.unwrap();

        assert!(result.is_ok());
        assert_eq!(5, *result.unwrap().as_result().unwrap());
        assert_eq!(vec![1, 4, 8, 14, 24], *times.lock().unwrap());
    }

    #[tokio::test]
    async fn exceed_max_wait_time() {
        let _logs = show_test_logs();
        let (time_source, sleep_impl) = tick_advance_time_and_sleep();

        let orchestrator = test_orchestrator(sleep_impl.clone(), time_source.clone())
            .acceptor(|_result: Result<&usize, &TestError>| AcceptorState::Retry)
            .operation(|| async { Result::<_, SdkError<TestError, HttpResponse>>::Ok(1) })
            .build();

        let task = tokio::spawn(orchestrator.orchestrate());
        tokio::task::yield_now().await;
        time_source.tick(Duration::from_secs(500)).await;
        let result = task.await.unwrap();

        match result {
            Err(WaiterError::ExceededMaxWait(context)) => {
                assert_eq!(Duration::from_secs(300), context.max_wait());
                assert_eq!(300, context.elapsed().as_secs());
                assert_eq!(12, context.poll_count());
            }
            _ => panic!("expected ExceededMaxWait, got {result:?}"),
        }
    }

    #[tokio::test]
    async fn operation_timed_out() {
        let _logs = show_test_logs();
        let (time_source, sleep_impl) = tick_advance_time_and_sleep();
        let orchestrator = test_orchestrator(sleep_impl, time_source)
            .acceptor(|_result: Result<&usize, &TestError>| unreachable!())
            .operation(|| async {
                Result::<usize, SdkError<TestError, HttpResponse>>::Err(SdkError::timeout_error(
                    "test",
                ))
            })
            .build();

        match orchestrator.orchestrate().await {
            Err(WaiterError::OperationFailed(err)) => match err.error() {
                SdkError::TimeoutError(_) => { /* good */ }
                result => panic!("unexpected final poll: {result:?}"),
            },
            result => panic!("unexpected result: {result:?}"),
        }
    }

    #[tokio::test]
    async fn modeled_service_error_no_acceptors_matched() {
        let _logs = show_test_logs();
        let (time_source, sleep_impl) = tick_advance_time_and_sleep();
        let orchestrator = test_orchestrator(sleep_impl, time_source)
            .acceptor(|_result: Result<&usize, &TestError>| AcceptorState::NoAcceptorsMatched)
            .operation(|| async {
                Result::<usize, SdkError<TestError, HttpResponse>>::Err(SdkError::service_error(
                    TestError,
                    HttpResponse::new(StatusCode::try_from(400).unwrap(), SdkBody::empty()),
                ))
            })
            .build();

        match dbg!(orchestrator.orchestrate().await) {
            Err(WaiterError::OperationFailed(err)) => match err.error() {
                SdkError::ServiceError(_) => { /* good */ }
                result => panic!("unexpected result: {result:?}"),
            },
            result => panic!("unexpected result: {result:?}"),
        }
    }

    #[tokio::test]
    async fn modeled_error_matched_as_failure() {
        let _logs = show_test_logs();
        let (time_source, sleep_impl) = tick_advance_time_and_sleep();
        let orchestrator = test_orchestrator(sleep_impl, time_source)
            .acceptor(|_result: Result<&usize, &TestError>| AcceptorState::Failure)
            .operation(|| async {
                Result::<usize, SdkError<TestError, HttpResponse>>::Err(SdkError::service_error(
                    TestError,
                    HttpResponse::new(StatusCode::try_from(400).unwrap(), SdkBody::empty()),
                ))
            })
            .build();

        match orchestrator.orchestrate().await {
            Err(WaiterError::FailureState(err)) => match err.final_poll().as_result() {
                Err(TestError) => { /* good */ }
                result => panic!("unexpected final poll: {result:?}"),
            },
            result => panic!("unexpected result: {result:?}"),
        }
    }

    #[tokio::test]
    async fn modeled_error_matched_as_success() {
        let _logs = show_test_logs();
        let (time_source, sleep_impl) = tick_advance_time_and_sleep();
        let orchestrator = test_orchestrator(sleep_impl, time_source)
            .acceptor(|_result: Result<&usize, &TestError>| AcceptorState::Success)
            .operation(|| async {
                Result::<usize, SdkError<TestError, HttpResponse>>::Err(SdkError::service_error(
                    TestError,
                    HttpResponse::new(StatusCode::try_from(400).unwrap(), SdkBody::empty()),
                ))
            })
            .build();

        let result = orchestrator.orchestrate().await;
        assert!(result.is_ok());
        assert!(result.unwrap().as_result().is_err());
    }
}
