/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

#![cfg(all(feature = "client", feature = "wire-mock",))]

use ::aws_smithy_runtime::client::retries::classifiers::{
    HttpStatusCodeClassifier, TransientErrorClassifier,
};
use aws_smithy_async::rt::sleep::TokioSleep;
use aws_smithy_runtime::client::http::hyper_014::HyperClientBuilder;
use aws_smithy_runtime::client::http::test_util::wire::{
    RecordedEvent, ReplayedEvent, WireMockServer,
};
use aws_smithy_runtime::client::orchestrator::operation::Operation;
use aws_smithy_runtime::test_util::capture_test_logs::capture_test_logs;
use aws_smithy_runtime::{ev, match_events};
use aws_smithy_runtime_api::client::interceptors::context::InterceptorContext;
use aws_smithy_runtime_api::client::orchestrator::OrchestratorError;
use aws_smithy_runtime_api::client::retries::classifiers::{ClassifyRetry, RetryAction};
use aws_smithy_types::body::SdkBody;
use aws_smithy_types::retry::{ErrorKind, ProvideErrorKind, ReconnectMode, RetryConfig};
use aws_smithy_types::timeout::TimeoutConfig;
use hyper_0_14::client::Builder as HyperBuilder;
use std::fmt;
use std::time::Duration;

const END_OF_TEST: &str = "end_of_test";

#[derive(Debug)]
struct OperationError(ErrorKind);

impl fmt::Display for OperationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ProvideErrorKind for OperationError {
    fn retryable_error_kind(&self) -> Option<ErrorKind> {
        Some(self.0)
    }

    fn code(&self) -> Option<&str> {
        None
    }
}

impl std::error::Error for OperationError {}

#[derive(Debug)]
struct TestRetryClassifier;

impl ClassifyRetry for TestRetryClassifier {
    fn classify_retry(&self, ctx: &InterceptorContext) -> RetryAction {
        tracing::info!("classifying retry for {ctx:?}");
        // Check for a result
        let output_or_error = ctx.output_or_error();
        // Check for an error
        let error = match output_or_error {
            Some(Ok(_)) | None => return RetryAction::NoActionIndicated,
            Some(Err(err)) => err,
        };

        let action = if let Some(err) = error.as_operation_error() {
            tracing::info!("its an operation error: {err:?}");
            let err = err.downcast_ref::<OperationError>().unwrap();
            RetryAction::retryable_error(err.0)
        } else {
            tracing::info!("its something else... using other classifiers");
            let action = TransientErrorClassifier::<OperationError>::new().classify_retry(ctx);
            if action == RetryAction::NoActionIndicated {
                HttpStatusCodeClassifier::default().classify_retry(ctx)
            } else {
                action
            }
        };

        tracing::info!("classified as {action:?}");
        action
    }

    fn name(&self) -> &'static str {
        "test"
    }
}

async fn h1_and_h2(events: Vec<ReplayedEvent>, match_clause: impl Fn(&[RecordedEvent])) {
    wire_level_test(
        events.clone(),
        |_b| {},
        ReconnectMode::ReconnectOnTransientError,
        &match_clause,
    )
    .await;
    wire_level_test(
        events,
        |b| {
            b.http2_only(true);
        },
        ReconnectMode::ReconnectOnTransientError,
        match_clause,
    )
    .await;
    tracing::info!("h2 ok!");
}

/// Repeatedly send test operation until `end_of_test` is received
///
/// When the test is over, match_clause is evaluated
async fn wire_level_test(
    events: Vec<ReplayedEvent>,
    hyper_builder_settings: impl Fn(&mut HyperBuilder),
    reconnect_mode: ReconnectMode,
    match_clause: impl Fn(&[RecordedEvent]),
) {
    let mut hyper_builder = hyper_0_14::Client::builder();
    hyper_builder_settings(&mut hyper_builder);

    let mock = WireMockServer::start(events).await;
    let http_client = HyperClientBuilder::new()
        .hyper_builder(hyper_builder)
        .build(hyper_0_14::client::HttpConnector::new_with_resolver(
            mock.dns_resolver(),
        ));

    let operation = Operation::builder()
        .service_name("test")
        .operation_name("test")
        .no_auth()
        .endpoint_url(&mock.endpoint_url())
        .http_client(http_client)
        .timeout_config(
            TimeoutConfig::builder()
                .operation_attempt_timeout(Duration::from_millis(100))
                .build(),
        )
        .standard_retry(&RetryConfig::standard().with_reconnect_mode(reconnect_mode))
        .retry_classifier(TestRetryClassifier)
        .sleep_impl(TokioSleep::new())
        .with_connection_poisoning()
        .serializer({
            let endpoint_url = mock.endpoint_url();
            move |_| {
                let request = http_02x::Request::builder()
                    .uri(endpoint_url.clone())
                    // Make the body non-replayable since we don't actually want to retry
                    .body(SdkBody::from_body_0_4(SdkBody::from("body")))
                    .unwrap()
                    .try_into()
                    .unwrap();
                tracing::info!("serializing request: {request:?}");
                Ok(request)
            }
        })
        .deserializer(|response| {
            tracing::info!("deserializing response: {:?}", response);
            match response.status() {
                s if s.is_success() => {
                    Ok(String::from_utf8(response.body().bytes().unwrap().into()).unwrap())
                }
                s if s.is_client_error() => Err(OrchestratorError::operation(OperationError(
                    ErrorKind::ServerError,
                ))),
                s if s.is_server_error() => Err(OrchestratorError::operation(OperationError(
                    ErrorKind::TransientError,
                ))),
                _ => panic!("unexpected status: {}", response.status()),
            }
        })
        .build();

    let mut iteration = 0;
    loop {
        tracing::info!("iteration {iteration}...");
        match operation.invoke(()).await {
            Ok(resp) => {
                tracing::info!("response: {:?}", resp);
                if resp == END_OF_TEST {
                    break;
                }
            }
            Err(e) => tracing::info!("error: {:?}", e),
        }
        iteration += 1;
        if iteration > 50 {
            panic!("probably an infinite loop; no satisfying 'end_of_test' response received");
        }
    }
    let events = mock.events();
    match_clause(&events);
    mock.shutdown();
}

#[tokio::test]
async fn non_transient_errors_no_reconnect() {
    let _logs = capture_test_logs();
    h1_and_h2(
        vec![
            ReplayedEvent::status(400),
            ReplayedEvent::with_body(END_OF_TEST),
        ],
        match_events!(ev!(dns), ev!(connect), ev!(http(400)), ev!(http(200))),
    )
    .await
}

#[tokio::test]
async fn reestablish_dns_on_503() {
    let _logs = capture_test_logs();
    h1_and_h2(
        vec![
            ReplayedEvent::status(503),
            ReplayedEvent::status(503),
            ReplayedEvent::status(503),
            ReplayedEvent::with_body(END_OF_TEST),
        ],
        match_events!(
            // first request
            ev!(dns),
            ev!(connect),
            ev!(http(503)),
            // second request
            ev!(dns),
            ev!(connect),
            ev!(http(503)),
            // third request
            ev!(dns),
            ev!(connect),
            ev!(http(503)),
            // all good
            ev!(dns),
            ev!(connect),
            ev!(http(200))
        ),
    )
    .await;
}

#[tokio::test]
async fn connection_shared_on_success() {
    let _logs = capture_test_logs();
    h1_and_h2(
        vec![
            ReplayedEvent::ok(),
            ReplayedEvent::ok(),
            ReplayedEvent::status(503),
            ReplayedEvent::with_body(END_OF_TEST),
        ],
        match_events!(
            ev!(dns),
            ev!(connect),
            ev!(http(200)),
            ev!(http(200)),
            ev!(http(503)),
            ev!(dns),
            ev!(connect),
            ev!(http(200))
        ),
    )
    .await;
}

#[tokio::test]
async fn no_reconnect_when_disabled() {
    let _logs = capture_test_logs();
    wire_level_test(
        vec![
            ReplayedEvent::status(503),
            ReplayedEvent::with_body(END_OF_TEST),
        ],
        |_b| {},
        ReconnectMode::ReuseAllConnections,
        match_events!(ev!(dns), ev!(connect), ev!(http(503)), ev!(http(200))),
    )
    .await;
}

#[tokio::test]
async fn connection_reestablished_after_timeout() {
    let _logs = capture_test_logs();
    h1_and_h2(
        vec![
            ReplayedEvent::ok(),
            ReplayedEvent::Timeout,
            ReplayedEvent::ok(),
            ReplayedEvent::Timeout,
            ReplayedEvent::with_body(END_OF_TEST),
        ],
        match_events!(
            // first connection
            ev!(dns),
            ev!(connect),
            ev!(http(200)),
            // reuse but got a timeout
            ev!(timeout),
            // so we reconnect
            ev!(dns),
            ev!(connect),
            ev!(http(200)),
            ev!(timeout),
            ev!(dns),
            ev!(connect),
            ev!(http(200))
        ),
    )
    .await;
}
