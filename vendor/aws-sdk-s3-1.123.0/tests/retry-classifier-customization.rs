/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_sdk_s3::config::interceptors::InterceptorContext;
use aws_sdk_s3::config::retry::{ClassifyRetry, RetryAction, RetryConfig};
use aws_sdk_s3::config::SharedAsyncSleep;
use aws_smithy_async::rt::sleep::TokioSleep;
use aws_smithy_http_client::test_util::{ReplayEvent, StaticReplayClient};
use aws_smithy_runtime_api::client::retries::classifiers::RetryClassifierPriority;
use aws_smithy_types::body::SdkBody;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
struct CustomizationTestClassifier {
    counter: Arc<Mutex<u8>>,
}

impl CustomizationTestClassifier {
    pub fn new() -> Self {
        Self {
            counter: Arc::new(Mutex::new(0u8)),
        }
    }

    pub fn counter(&self) -> u8 {
        *self.counter.lock().unwrap()
    }
}

impl ClassifyRetry for CustomizationTestClassifier {
    fn classify_retry(&self, ctx: &InterceptorContext) -> RetryAction {
        *self.counter.lock().unwrap() += 1;

        // Interceptors may call this classifier before a response is received. If a response was received,
        // ensure that it has the expected status code.
        if let Some(res) = ctx.response() {
            assert_eq!(
                500,
                res.status().as_u16(),
                "expected a 500 response from test connection"
            );
        }

        RetryAction::RetryForbidden
    }

    fn name(&self) -> &'static str {
        "Custom Retry Classifier"
    }
}

fn req() -> http_1x::Request<SdkBody> {
    http_1x::Request::builder()
        .body(SdkBody::from("request body"))
        .unwrap()
}

fn ok() -> http_1x::Response<SdkBody> {
    http_1x::Response::builder()
        .status(200)
        .body(SdkBody::from("Hello!"))
        .unwrap()
}

fn err() -> http_1x::Response<SdkBody> {
    http_1x::Response::builder()
        .status(500)
        .body(SdkBody::from("This was an error"))
        .unwrap()
}

#[tokio::test]
async fn test_retry_classifier_customization_for_service() {
    let http_client = StaticReplayClient::new(vec![
        ReplayEvent::new(req(), err()),
        ReplayEvent::new(req(), ok()),
    ]);

    let customization_test_classifier = CustomizationTestClassifier::new();

    let config = aws_sdk_s3::Config::builder()
        .with_test_defaults()
        .sleep_impl(SharedAsyncSleep::new(TokioSleep::new()))
        .http_client(http_client)
        .retry_config(RetryConfig::standard())
        .retry_classifier(customization_test_classifier.clone())
        .build();

    let client = aws_sdk_s3::Client::from_conf(config);
    let _ = client
        .get_object()
        .bucket("bucket")
        .key("key")
        .send()
        .await
        .expect_err("fails without attempting a retry");

    // ensure our custom retry classifier was called at least once.
    assert_ne!(customization_test_classifier.counter(), 0);
}

#[tokio::test]
async fn test_retry_classifier_customization_for_operation() {
    let http_client = StaticReplayClient::new(vec![
        ReplayEvent::new(req(), err()),
        ReplayEvent::new(req(), ok()),
    ]);

    let customization_test_classifier = CustomizationTestClassifier::new();

    let config = aws_sdk_s3::Config::builder()
        .with_test_defaults()
        .sleep_impl(SharedAsyncSleep::new(TokioSleep::new()))
        .http_client(http_client)
        .retry_config(RetryConfig::standard())
        .build();

    let client = aws_sdk_s3::Client::from_conf(config);
    let _ = client
        .get_object()
        .bucket("bucket")
        .key("key")
        .customize()
        .config_override(
            aws_sdk_s3::config::Config::builder()
                .retry_classifier(customization_test_classifier.clone()),
        )
        .send()
        .await
        .expect_err("fails without attempting a retry");

    // ensure our custom retry classifier was called at least once.
    assert_ne!(customization_test_classifier.counter(), 0);
}

#[derive(Debug, Clone)]
struct OrderingTestClassifier {
    counter: Arc<Mutex<u8>>,
    name: &'static str,
    priority: RetryClassifierPriority,
}

impl OrderingTestClassifier {
    pub fn new(name: &'static str, priority: RetryClassifierPriority) -> Self {
        Self {
            counter: Arc::new(Mutex::new(0u8)),
            name,
            priority,
        }
    }

    pub fn counter(&self) -> u8 {
        *self.counter.lock().unwrap()
    }
}

impl ClassifyRetry for OrderingTestClassifier {
    fn classify_retry(&self, _ctx: &InterceptorContext) -> RetryAction {
        tracing::debug!("Running classifier {}", self.name);
        *self.counter.lock().unwrap() += 1;
        RetryAction::NoActionIndicated
    }

    fn name(&self) -> &'static str {
        "Ordering Test Retry Classifier"
    }

    fn priority(&self) -> RetryClassifierPriority {
        self.priority.clone()
    }
}

#[tracing_test::traced_test]
#[tokio::test]
async fn test_retry_classifier_customization_ordering() {
    let http_client = StaticReplayClient::new(vec![
        ReplayEvent::new(req(), err()),
        ReplayEvent::new(req(), ok()),
    ]);

    let classifier_a = OrderingTestClassifier::new("6", RetryClassifierPriority::default());
    let classifier_b = OrderingTestClassifier::new(
        "5",
        RetryClassifierPriority::run_before(classifier_a.priority()),
    );
    let classifier_c = OrderingTestClassifier::new(
        "4",
        RetryClassifierPriority::run_before(classifier_b.priority()),
    );
    let classifier_d = OrderingTestClassifier::new(
        "3",
        RetryClassifierPriority::run_before(classifier_c.priority()),
    );
    let classifier_e = OrderingTestClassifier::new(
        "2",
        RetryClassifierPriority::run_before(classifier_d.priority()),
    );
    let classifier_f = OrderingTestClassifier::new(
        "1",
        RetryClassifierPriority::run_before(classifier_e.priority()),
    );

    let config = aws_sdk_s3::Config::builder()
        .with_test_defaults()
        .sleep_impl(SharedAsyncSleep::new(TokioSleep::new()))
        .http_client(http_client)
        .retry_config(RetryConfig::standard())
        .retry_classifier(classifier_d.clone())
        .retry_classifier(classifier_b.clone())
        .retry_classifier(classifier_f.clone())
        .build();

    let client = aws_sdk_s3::Client::from_conf(config);
    let _ = client
        .get_object()
        .bucket("bucket")
        .key("key")
        .customize()
        .config_override(
            aws_sdk_s3::config::Config::builder()
                .retry_classifier(classifier_c.clone())
                .retry_classifier(classifier_a.clone())
                .retry_classifier(classifier_e.clone()),
        )
        .send()
        .await
        .expect_err("fails without attempting a retry");

    // ensure our classifiers were each called at least once.
    assert_ne!(classifier_a.counter(), 0, "classifier_a was never called");
    assert_ne!(classifier_b.counter(), 0, "classifier_b was never called");
    assert_ne!(classifier_c.counter(), 0, "classifier_c was never called");
    assert_ne!(classifier_d.counter(), 0, "classifier_d was never called");
    assert_ne!(classifier_e.counter(), 0, "classifier_e was never called");
    assert_ne!(classifier_f.counter(), 0, "classifier_f was never called");

    // ensure the classifiers were called in the correct order.
    logs_assert(|lines: &[&str]| {
        let mut found_log_a = false;
        let mut line_iter = lines.iter();

        while found_log_a == false {
            match line_iter.next() {
                Some(&line) => {
                    if line.contains("Running classifier 1") {
                        found_log_a = true;
                    }
                }
                None => {
                    return Err("Couldn't find log line for classifier 1".to_owned());
                }
            }
        }

        for i in 2..=6 {
            match line_iter.next() {
                Some(&line) => {
                    if line.contains(&format!("Running classifier {i}")) {
                        // pass
                    } else {
                        return Err(format!("Expected to find log line for classifier {i} after {} but found '{line}'", i - 1));
                    }
                }
                None => {
                    return Err(format!("Logs ended earlier than expected ({i})"));
                }
            }
        }

        Ok(())
    });
}
