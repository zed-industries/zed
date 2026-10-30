use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use http_client::{
    AsyncBody, FakeHttpClient, HttpClient, Response, StatusCode,
    http::{HeaderMap, header},
};
use parking_lot::Mutex;
use serde_json::{Value, json};

use crate::{
    real_crates_io_client::{MIN_REQUEST_INTERVAL, RequestThrottle},
    *,
};

struct RecordedRequest {
    headers: HeaderMap,
}

/// Returns a fake HTTP client that records each request and answers it with
/// the status and JSON body `respond` returns for the request's URI.
fn recording_fake_http_client(
    respond: impl Fn(&str) -> (StatusCode, Value) + Send + Sync + 'static,
) -> (Arc<dyn HttpClient>, Arc<Mutex<Vec<RecordedRequest>>>) {
    let requests = Arc::new(Mutex::new(Vec::new()));
    let respond = Arc::new(respond);
    let fake_http_client = FakeHttpClient::create({
        let requests = requests.clone();
        move |request| {
            let requests = requests.clone();
            let respond = respond.clone();
            async move {
                let (status, response_body) = respond(&request.uri().to_string());
                requests.lock().push(RecordedRequest {
                    headers: request.headers().clone(),
                });
                Ok(Response::builder()
                    .status(status)
                    .body(AsyncBody::from(response_body.to_string()))?)
            }
        }
    });
    (fake_http_client, requests)
}

#[test]
fn test_requests_identify_zed_and_authenticate_with_api_token() {
    let (fake_http_client, requests) = recording_fake_http_client(|_| {
        let site_metadata = json!({
            "deployed_sha": "abc",
            "commit": "abc",
            "cdn_base": "https://static.crates.io",
            "read_only": false,
        });
        (StatusCode::OK, site_metadata)
    });
    let client = RealCratesIoClient::new(fake_http_client, ApiToken::new("secret"));

    smol::block_on(client.get_site_metadata()).unwrap();

    let requests = requests.lock();
    assert_eq!(
        requests[0].headers[header::USER_AGENT],
        "Zed GPUI releases (https://github.com/zed-industries/zed)"
    );
    assert_eq!(requests[0].headers[header::AUTHORIZATION], "secret");
}

#[test]
fn test_error_responses_distinguish_permission_and_upstream_failures() {
    let error_for = |status: StatusCode| {
        let (fake_http_client, _requests) = recording_fake_http_client(move |_| {
            let errors = json!({ "errors": [{ "detail": "first" }, { "detail": "second" }] });
            (status, errors)
        });
        let client = RealCratesIoClient::new(fake_http_client, ApiToken::new("secret"));
        smol::block_on(client.get_site_metadata()).unwrap_err()
    };

    assert!(matches!(
        error_for(StatusCode::FORBIDDEN),
        CratesIoError::PermissionDenied { status: StatusCode::FORBIDDEN, message } if message == "first; second"
    ));
    assert!(matches!(
        error_for(StatusCode::UNAUTHORIZED),
        CratesIoError::PermissionDenied { .. }
    ));
    assert!(matches!(
        error_for(StatusCode::NOT_FOUND),
        CratesIoError::NotFound { .. }
    ));
    assert!(matches!(
        error_for(StatusCode::SERVICE_UNAVAILABLE),
        CratesIoError::Upstream {
            status: StatusCode::SERVICE_UNAVAILABLE,
            ..
        }
    ));
}

#[test]
fn test_throttle_spaces_requests_by_minimum_interval() {
    let throttle = RequestThrottle::new();
    let start = Instant::now();

    assert_eq!(throttle.reserve(start), start);
    assert_eq!(throttle.reserve(start), start + MIN_REQUEST_INTERVAL);
    assert_eq!(throttle.reserve(start), start + 2 * MIN_REQUEST_INTERVAL);
    let later = start + Duration::from_secs(10);
    assert_eq!(throttle.reserve(later), later);
}
