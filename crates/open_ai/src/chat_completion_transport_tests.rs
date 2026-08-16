use super::*;
use futures::executor::block_on;
use http_client::{
    FakeHttpClient, Response,
    http::{HeaderName, HeaderValue},
};
use serde_json::json;
use std::sync::{Arc, Mutex};

#[test]
fn streaming_transport_serializes_custom_requests_and_reports_done() {
    let captured_request = Arc::new(Mutex::new(None));
    let captured_request_for_handler = captured_request.clone();
    let client = FakeHttpClient::create(move |mut request| {
        let captured_request = captured_request_for_handler.clone();
        async move {
            let uri = request.uri().to_string();
            let authorization = request.headers()["authorization"]
                .to_str()
                .expect("valid authorization header")
                .to_string();
            let custom_header = request.headers()["x-compatible-provider"]
                .to_str()
                .expect("valid custom header")
                .to_string();
            let mut body = String::new();
            request.body_mut().read_to_string(&mut body).await?;
            captured_request
                .lock()
                .expect("captured request lock")
                .replace((uri, authorization, custom_header, body));
            Ok(Response::builder()
                .status(200)
                .body(AsyncBody::from(concat!(
                    ": keepalive\n",
                    "event: message\n",
                    "data:{\"chunk\":1}\n\n",
                    "data: [DONE]\n\n"
                )))?)
        }
    });
    let extra_headers = CustomHeaders::new(vec![(
        HeaderName::from_static("x-compatible-provider"),
        HeaderValue::from_static("enabled"),
    )]);

    let events = block_on(async {
        stream_chat_completion(
            client.as_ref(),
            "Compatible Provider",
            "https://example.com/v1",
            " secret ",
            &extra_headers,
            &json!({"model": "custom/model", "provider_option": true}),
        )
        .await
        .expect("streaming request")
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .collect::<std::result::Result<Vec<_>, RequestError>>()
        .expect("stream events")
    });

    assert_eq!(
        events,
        vec![
            ChatCompletionStreamEvent::Data(json!({"chunk": 1})),
            ChatCompletionStreamEvent::Done,
        ]
    );
    assert_eq!(
        captured_request
            .lock()
            .expect("captured request lock")
            .as_ref(),
        Some(&(
            "https://example.com/v1/chat/completions".to_string(),
            "Bearer secret".to_string(),
            "enabled".to_string(),
            json!({"model": "custom/model", "provider_option": true}).to_string(),
        ))
    );
}

#[test]
fn streaming_transport_does_not_synthesize_done_at_eof() {
    let client = FakeHttpClient::create(|_| async move {
        Ok(Response::builder()
            .status(200)
            .body(AsyncBody::from("data: {\"chunk\":1}\n\n"))?)
    });

    let events = block_on(async {
        stream_chat_completion(
            client.as_ref(),
            "Compatible Provider",
            "https://example.com/v1",
            "secret",
            &CustomHeaders::default(),
            &json!({"model": "custom/model"}),
        )
        .await
        .expect("streaming request")
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .collect::<std::result::Result<Vec<_>, RequestError>>()
        .expect("stream events")
    });

    assert_eq!(
        events,
        vec![ChatCompletionStreamEvent::Data(json!({"chunk": 1}))]
    );
}

#[test]
fn transport_preserves_typed_send_and_deserialization_errors() {
    let client = FakeHttpClient::create(|_| async move { Err(anyhow!("network unavailable")) });
    let error = block_on(non_streaming_chat_completion(
        client.as_ref(),
        "Compatible Provider",
        "https://example.com/v1",
        "secret",
        &CustomHeaders::default(),
        &json!({"model": "custom/model"}),
    ))
    .expect_err("send error");
    assert!(matches!(
        error,
        RequestError::HttpSend { provider, .. } if provider == "Compatible Provider"
    ));

    let client = FakeHttpClient::create(|_| async move {
        Ok(Response::builder()
            .status(200)
            .body(AsyncBody::from("not JSON"))?)
    });
    let error = block_on(non_streaming_chat_completion(
        client.as_ref(),
        "Compatible Provider",
        "https://example.com/v1",
        "secret",
        &CustomHeaders::default(),
        &json!({"model": "custom/model"}),
    ))
    .expect_err("deserialization error");
    assert!(matches!(
        error,
        RequestError::DeserializeResponse { provider, .. }
            if provider == "Compatible Provider"
    ));
}

#[test]
fn non_streaming_transport_supports_custom_envelopes_and_provider_errors() {
    let client = FakeHttpClient::create(|request| async move {
        assert_eq!(request.headers()["x-compatible-provider"], "enabled");
        Ok(Response::builder()
            .status(200)
            .body(AsyncBody::from(r#"{"result":"ok"}"#))?)
    });
    let extra_headers = CustomHeaders::new(vec![(
        HeaderName::from_static("x-compatible-provider"),
        HeaderValue::from_static("enabled"),
    )]);
    let response = block_on(non_streaming_chat_completion(
        client.as_ref(),
        "Compatible Provider",
        "https://example.com/v1",
        "secret",
        &extra_headers,
        &json!({"model": "custom/model", "custom_option": "value"}),
    ))
    .expect("non-streaming request");
    assert_eq!(response, json!({"result": "ok"}));

    let client = FakeHttpClient::create(|_| async move {
        Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(AsyncBody::from("invalid custom option"))?)
    });
    let error = block_on(non_streaming_chat_completion(
        client.as_ref(),
        "Compatible Provider",
        "https://example.com/v1",
        "secret",
        &CustomHeaders::default(),
        &json!({"model": "custom/model"}),
    ))
    .expect_err("provider error");
    match error {
        RequestError::HttpResponseError {
            provider,
            status_code,
            body,
            ..
        } => {
            assert_eq!(provider, "Compatible Provider");
            assert_eq!(status_code, StatusCode::BAD_REQUEST);
            assert_eq!(body, "invalid custom option");
        }
        error => panic!("unexpected transport error: {error}"),
    }
}
