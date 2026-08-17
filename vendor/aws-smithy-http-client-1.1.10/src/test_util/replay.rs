/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use aws_smithy_protocol_test::{assert_ok, validate_body, MediaType};
use aws_smithy_runtime_api::client::connector_metadata::ConnectorMetadata;
use aws_smithy_runtime_api::client::http::{
    HttpClient, HttpConnector, HttpConnectorFuture, HttpConnectorSettings, SharedHttpConnector,
};
use aws_smithy_runtime_api::client::orchestrator::{HttpRequest, HttpResponse};
use aws_smithy_runtime_api::client::result::ConnectorError;
use aws_smithy_runtime_api::client::runtime_components::RuntimeComponents;
use aws_smithy_runtime_api::shared::IntoShared;
use http_1x::header::CONTENT_TYPE;
use std::ops::Deref;
use std::sync::{Arc, Mutex, MutexGuard};

type ReplayEvents = Vec<ReplayEvent>;

pub(crate) const DEFAULT_RELAXED_HEADERS: &[&str] = &["x-amz-user-agent", "authorization"];

/// Test data for the [`StaticReplayClient`].
///
/// Each `ReplayEvent` represents one HTTP request and response
/// through the connector.
#[derive(Debug)]
pub struct ReplayEvent {
    request: HttpRequest,
    response: HttpResponse,
}

impl ReplayEvent {
    /// Creates a new `ReplayEvent`.
    pub fn new(request: impl TryInto<HttpRequest>, response: impl TryInto<HttpResponse>) -> Self {
        Self {
            request: request.try_into().ok().expect("invalid request"),
            response: response.try_into().ok().expect("invalid response"),
        }
    }

    /// Returns the test request.
    pub fn request(&self) -> &HttpRequest {
        &self.request
    }

    /// Returns the test response.
    pub fn response(&self) -> &HttpResponse {
        &self.response
    }
}

impl From<(HttpRequest, HttpResponse)> for ReplayEvent {
    fn from((request, response): (HttpRequest, HttpResponse)) -> Self {
        Self::new(request, response)
    }
}

#[derive(Debug)]
struct ValidateRequest {
    expected: HttpRequest,
    actual: HttpRequest,
}

impl ValidateRequest {
    fn assert_matches(&self, index: usize, ignore_headers: &[&str]) {
        let (actual, expected) = (&self.actual, &self.expected);
        assert_eq!(
            expected.uri(),
            actual.uri(),
            "request[{index}] - URI doesn't match expected value"
        );
        for (name, value) in expected.headers() {
            if !ignore_headers.contains(&name) {
                let actual_header = actual
                    .headers()
                    .get(name)
                    .unwrap_or_else(|| panic!("Request #{index} - Header {name:?} is missing"));
                assert_eq!(
                    value, actual_header,
                    "request[{index}] - Header {name:?} doesn't match expected value",
                );
            }
        }
        let actual_str = std::str::from_utf8(actual.body().bytes().unwrap_or(&[]));
        let expected_str = std::str::from_utf8(expected.body().bytes().unwrap_or(&[]));
        let media_type = if actual
            .headers()
            .get(CONTENT_TYPE)
            .map(|v| v.contains("json"))
            .unwrap_or(false)
        {
            MediaType::Json
        } else {
            MediaType::Other("unknown".to_string())
        };
        match (actual_str, expected_str) {
            (Ok(actual), Ok(expected)) => assert_ok(validate_body(actual, expected, media_type)),
            _ => assert_eq!(
                expected.body().bytes(),
                actual.body().bytes(),
                "request[{index}] - Body contents didn't match expected value"
            ),
        };
    }
}

/// Request/response replaying client for use in tests.
///
/// This mock client takes a list of request/response pairs named [`ReplayEvent`]. While the client
/// is in use, the responses will be given in the order they appear in the list regardless of what
/// the actual request was. The actual request is recorded, but otherwise not validated against what
/// is in the [`ReplayEvent`]. Later, after the client is finished being used, the
/// [`assert_requests_match`] method can be used to validate the requests.
///
/// This utility is simpler than [DVR], and thus, is good for tests that don't need
/// to record and replay real traffic.
///
/// # Example
///
/// ```no_run
/// # use http_1x as http;
/// use aws_smithy_http_client::test_util::{ReplayEvent, StaticReplayClient};
/// use aws_smithy_types::body::SdkBody;
///
/// let http_client = StaticReplayClient::new(vec![
///     // Event that covers the first request/response
///     ReplayEvent::new(
///         // If `assert_requests_match` is called later, then this request will be matched
///         // against the actual request that was made.
///         http::Request::builder().uri("http://localhost:1234/foo").body(SdkBody::empty()).unwrap(),
///         // This response will be given to the first request regardless of whether it matches the request above.
///         http::Response::builder().status(200).body(SdkBody::empty()).unwrap(),
///     ),
///     // The next ReplayEvent covers the second request/response pair...
/// ]);
///
/// # /*
/// let config = my_generated_client::Config::builder()
///     .http_client(http_client.clone())
///     .build();
/// let client = my_generated_client::Client::from_conf(config);
/// # */
///
/// // Do stuff with client...
///
/// // When you're done, assert the requests match what you expected
/// http_client.assert_requests_match(&[]);
/// ```
///
/// [`assert_requests_match`]: StaticReplayClient::assert_requests_match
/// [DVR]: crate::test_util::dvr
#[derive(Clone, Debug)]
pub struct StaticReplayClient {
    data: Arc<Mutex<ReplayEvents>>,
    requests: Arc<Mutex<Vec<ValidateRequest>>>,
}

impl StaticReplayClient {
    /// Creates a new event connector.
    pub fn new(mut data: ReplayEvents) -> Self {
        data.reverse();
        StaticReplayClient {
            data: Arc::new(Mutex::new(data)),
            requests: Default::default(),
        }
    }

    /// Returns an iterator over the actual requests that were made.
    pub fn actual_requests(&self) -> impl Iterator<Item = &HttpRequest> + '_ {
        // The iterator trait doesn't allow us to specify a lifetime on `self` in the `next()` method,
        // so we have to do some unsafe code in order to actually implement this iterator without
        // angering the borrow checker.
        struct Iter<'a> {
            // We store an exclusive lock to the data so that the data is completely immutable
            _guard: MutexGuard<'a, Vec<ValidateRequest>>,
            // We store a pointer into the immutable data for accessing it later
            values: *const ValidateRequest,
            len: usize,
            next_index: usize,
        }
        impl<'a> Iterator for Iter<'a> {
            type Item = &'a HttpRequest;

            fn next(&mut self) -> Option<Self::Item> {
                // Safety: check the next index is in bounds
                if self.next_index >= self.len {
                    None
                } else {
                    // Safety: It is OK to offset into the pointer and dereference since we did a bounds check.
                    // It is OK to assign lifetime 'a to the reference since we hold the mutex guard for all of lifetime 'a.
                    let next = unsafe {
                        let offset = self.values.add(self.next_index);
                        &*offset
                    };
                    self.next_index += 1;
                    Some(&next.actual)
                }
            }
        }

        let guard = self.requests.lock().unwrap();
        Iter {
            values: guard.as_ptr(),
            len: guard.len(),
            _guard: guard,
            next_index: 0,
        }
    }

    fn requests(&self) -> impl Deref<Target = Vec<ValidateRequest>> + '_ {
        self.requests.lock().unwrap()
    }

    /// Asserts the expected requests match the actual requests.
    ///
    /// The expected requests are given as the connection events when the `EventConnector`
    /// is created. The `EventConnector` will record the actual requests and assert that
    /// they match the expected requests.
    ///
    /// A list of headers that should be ignored when comparing requests can be passed
    /// for cases where headers are non-deterministic or are irrelevant to the test.
    #[track_caller]
    pub fn assert_requests_match(&self, ignore_headers: &[&str]) {
        for (i, req) in self.requests().iter().enumerate() {
            req.assert_matches(i, ignore_headers)
        }
        let remaining_requests = self.data.lock().unwrap();
        assert!(
            remaining_requests.is_empty(),
            "Expected {} additional requests (only {} sent)",
            remaining_requests.len(),
            self.requests().len()
        );
    }

    /// Convenience method for `assert_requests_match` that excludes the pre-defined headers to
    /// be ignored
    ///
    /// The pre-defined headers to be ignored:
    /// - x-amz-user-agent
    /// - authorization
    #[track_caller]
    pub fn relaxed_requests_match(&self) {
        self.assert_requests_match(DEFAULT_RELAXED_HEADERS)
    }
}

impl HttpConnector for StaticReplayClient {
    fn call(&self, request: HttpRequest) -> HttpConnectorFuture {
        let res = if let Some(event) = self.data.lock().unwrap().pop() {
            self.requests.lock().unwrap().push(ValidateRequest {
                expected: event.request,
                actual: request,
            });

            Ok(event.response)
        } else {
            Err(ConnectorError::other(
                "StaticReplayClient: no more test data available to respond with".into(),
                None,
            ))
        };

        HttpConnectorFuture::new(async move { res })
    }
}

impl HttpClient for StaticReplayClient {
    fn http_connector(
        &self,
        _: &HttpConnectorSettings,
        _: &RuntimeComponents,
    ) -> SharedHttpConnector {
        self.clone().into_shared()
    }

    fn connector_metadata(&self) -> Option<ConnectorMetadata> {
        Some(ConnectorMetadata::new("static-replay-client", None))
    }
}

#[cfg(test)]
mod test {
    use crate::test_util::{ReplayEvent, StaticReplayClient};
    use aws_smithy_types::body::SdkBody;

    #[test]
    fn create_from_either_http_type() {
        let _client = StaticReplayClient::new(vec![ReplayEvent::new(
            http_1x::Request::builder()
                .uri("test")
                .body(SdkBody::from("hello"))
                .unwrap(),
            http_1x::Response::builder()
                .status(200)
                .body(SdkBody::from("hello"))
                .unwrap(),
        )]);
    }
}
