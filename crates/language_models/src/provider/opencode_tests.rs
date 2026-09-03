use super::*;
use http_client::{FakeHttpClient, Response};
use std::sync::Mutex;

#[test]
fn test_opencode_session_header_uses_thread_id() {
    let value = opencode_session_header_value(Some("thread-123"));

    assert_eq!(value, "thread-123");
}

#[test]
fn test_opencode_session_header_without_thread_id() {
    let value = opencode_session_header_value(None);

    assert!(
        !value.as_bytes().is_empty() && value.as_bytes().iter().all(|byte| byte.is_ascii_digit())
    );
}

#[test]
fn test_opencode_session_header_with_empty_thread_id() {
    let value = opencode_session_header_value(Some(""));

    assert!(
        !value.as_bytes().is_empty() && value.as_bytes().iter().all(|byte| byte.is_ascii_digit())
    );
}

#[test]
fn test_opencode_session_header_with_invalid_thread_id() {
    let value = opencode_session_header_value(Some("thread\n123"));

    assert!(
        !value.as_bytes().is_empty() && value.as_bytes().iter().all(|byte| byte.is_ascii_digit())
    );
}

#[test]
fn test_inject_header_client_adds_session_header() -> Result<()> {
    let captured_header = Arc::new(Mutex::new(None));
    let captured_header_for_handler = captured_header.clone();
    let inner = FakeHttpClient::create(move |request| {
        let captured_header = captured_header_for_handler.clone();
        async move {
            let mut captured_header = captured_header
                .lock()
                .map_err(|_| anyhow::anyhow!("captured header mutex was poisoned"))?;
            *captured_header = request.headers().get(OPENCODE_SESSION_HEADER_NAME).cloned();
            Ok(Response::builder().status(200).body(Default::default())?)
        }
    });
    let client: Arc<dyn HttpClient> = Arc::new(InjectHeaderClient {
        inner,
        name: http::HeaderName::from_static(OPENCODE_SESSION_HEADER_NAME),
        value: opencode_session_header_value(Some("thread-123")),
    });
    let request = http::Request::builder()
        .uri("https://opencode.ai/zen/v1/messages")
        .body(AsyncBody::default())?;

    futures::executor::block_on(client.send(request))?;

    let captured_header = captured_header
        .lock()
        .map_err(|_| anyhow::anyhow!("captured header mutex was poisoned"))?;
    assert_eq!(
        captured_header.as_ref().map(http::HeaderValue::as_bytes),
        Some(b"thread-123".as_slice())
    );
    Ok(())
}
