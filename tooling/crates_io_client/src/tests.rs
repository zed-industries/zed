use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use futures::AsyncReadExt as _;
use http_client::{
    AsyncBody, FakeHttpClient, HttpClient, Method, Response, StatusCode,
    http::{HeaderMap, header},
};
use parking_lot::Mutex;
use serde_json::{Value, json};

use crate::{
    real_crates_io_client::{MIN_REQUEST_INTERVAL, RequestThrottle},
    *,
};

struct RecordedRequest {
    method: Method,
    uri: String,
    headers: HeaderMap,
    body: String,
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
        move |mut request| {
            let requests = requests.clone();
            let respond = respond.clone();
            async move {
                let mut body = String::new();
                request.body_mut().read_to_string(&mut body).await?;
                let uri = request.uri().to_string();
                let (status, response_body) = respond(&uri);
                requests.lock().push(RecordedRequest {
                    method: request.method().clone(),
                    uri,
                    headers: request.headers().clone(),
                    body,
                });
                Ok(Response::builder()
                    .status(status)
                    .body(AsyncBody::from(response_body.to_string()))?)
            }
        }
    });
    (fake_http_client, requests)
}

fn crate_json(name: &str) -> Value {
    json!({
        "name": name,
        "description": null,
        "homepage": null,
        "documentation": null,
        "repository": null,
        "created_at": "2026-01-01T00:00:00Z",
        "updated_at": "2026-01-01T00:00:00Z",
        "downloads": 1,
        "recent_downloads": 1,
        "default_version": "0.1.0",
        "num_versions": 1,
        "yanked": false,
        "trustpub_only": true,
        "keywords": null,
        "categories": null,
    })
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
fn test_list_crates_follows_next_page_until_exhausted() {
    let (fake_http_client, requests) = recording_fake_http_client(|uri| {
        let page = if uri.contains("seek=") {
            json!({
                "crates": [crate_json("gpui_util")],
                "meta": { "total": 2, "next_page": null, "prev_page": null },
            })
        } else {
            json!({
                "crates": [crate_json("gpui")],
                "meta": {
                    "total": 2,
                    "next_page": "?user_id=7&per_page=100&seek=abc",
                    "prev_page": null,
                },
            })
        };
        (StatusCode::OK, page)
    });
    let client = RealCratesIoClient::new(fake_http_client, ApiToken::new("secret"));

    let crates = smol::block_on(client.list_crates(&CrateListQuery {
        user_id: Some(UserId(7)),
        exclude_yanked: true,
        ..Default::default()
    }))
    .unwrap();

    let names = crates
        .iter()
        .map(|krate| krate.name.as_str())
        .collect::<Vec<_>>();
    assert_eq!(names, ["gpui", "gpui_util"]);
    let requests = requests.lock();
    assert_eq!(
        requests[0].uri,
        "https://crates.io/api/v1/crates?per_page=100&user_id=7&include_yanked=no"
    );
    assert_eq!(
        requests[1].uri,
        "https://crates.io/api/v1/crates?user_id=7&per_page=100&seek=abc"
    );
}

#[test]
fn test_update_crate_sends_settings_wrapped_in_crate_object() {
    let (fake_http_client, requests) =
        recording_fake_http_client(|_| (StatusCode::OK, json!({ "crate": crate_json("gpui") })));
    let client = RealCratesIoClient::new(fake_http_client, ApiToken::new("secret"));

    smol::block_on(client.update_crate(
        &CrateName::new("gpui"),
        &CrateUpdate {
            trustpub_only: Some(true),
        },
    ))
    .unwrap();

    let requests = requests.lock();
    assert_eq!(requests[0].method, Method::PATCH);
    assert_eq!(requests[0].uri, "https://crates.io/api/v1/crates/gpui");
    assert_eq!(
        serde_json::from_str::<Value>(&requests[0].body).unwrap(),
        json!({ "crate": { "trustpub_only": true } })
    );
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
