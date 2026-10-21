use super::*;
use anyhow::Result;
use futures::{AsyncReadExt, TryStreamExt};
use http_client::{AsyncBody, FakeHttpClient, Response, http};
use language_model_core::{
    LanguageModelCompletionEvent, LanguageModelRequestMessage, MessageContent, Role,
};

#[test]
fn completion_dispatches_protocols_and_maps_streamed_text() -> Result<()> {
    for (protocol, path, response) in [
        (
            ApiProtocol::Anthropic,
            "/zen/go/v1/messages",
            "event: content_block_delta\ndata: {\"type\":\"content_block_delta\",\"index\":0,\"delta\":{\"type\":\"text_delta\",\"text\":\"Hello\"}}\n\n",
        ),
        (
            ApiProtocol::OpenAiChat,
            "/zen/go/v1/chat/completions",
            "data: {\"id\":\"chat-1\",\"object\":\"chat.completion.chunk\",\"created\":0,\"model\":\"model\",\"choices\":[{\"index\":0,\"delta\":{\"content\":\"Hello\"},\"finish_reason\":null}]}\n\ndata: [DONE]\n\n",
        ),
        (
            ApiProtocol::OpenAiResponses,
            "/zen/go/v1/responses",
            "event: response.output_text.delta\ndata: {\"type\":\"response.output_text.delta\",\"item_id\":\"msg-1\",\"output_index\":0,\"content_index\":0,\"delta\":\"Hello\"}\n\n",
        ),
        (
            ApiProtocol::Google,
            "/zen/go/v1/models/model:streamGenerateContent",
            "data: {\"candidates\":[{\"content\":{\"role\":\"model\",\"parts\":[{\"text\":\"Hello\"}]},\"finishReason\":\"STOP\"}]}\n\n",
        ),
    ] {
        futures::executor::block_on(async {
            let client = FakeHttpClient::create(move |mut request| async move {
                assert_eq!(request.uri().path(), path);
                assert_eq!(
                    request.headers().get("x-custom"),
                    Some(&http::HeaderValue::from_static("value"))
                );
                let mut body = String::new();
                request.body_mut().read_to_string(&mut body).await?;
                let body: serde_json::Value = serde_json::from_str(&body)?;
                if protocol != ApiProtocol::Google {
                    assert_eq!(body["model"], "model");
                }
                Ok(Response::builder()
                    .status(200)
                    .body(AsyncBody::from(response))?)
            });
            let model = DiscoveredModel {
                model: crate::Model::new(
                    "model".into(),
                    None,
                    1000,
                    Some(100),
                    protocol,
                    None,
                    None,
                    false,
                ),
                supports_images: false,
                supports_thinking: false,
            };
            let headers = CustomHeaders::new(vec![(
                http::HeaderName::from_static("x-custom"),
                http::HeaderValue::from_static("value"),
            )]);
            let request = LanguageModelRequest {
                messages: vec![LanguageModelRequestMessage {
                    role: Role::User,
                    content: vec![MessageContent::Text("Hi".into())],
                    cache: false,
                    reasoning_details: None,
                }],
                ..Default::default()
            };
            let events = stream_completion(
                client.as_ref(),
                CompletionConfig {
                    model: &model,
                    api_url: "https://opencode.ai/zen/go/",
                    api_key: "test-key",
                    provider_id: LanguageModelProviderId::new("opencode-go"),
                    provider_name: LanguageModelProviderName::new("OpenCode Go"),
                    custom_headers: &headers,
                },
                request,
            )
            .await?
            .try_collect::<Vec<_>>()
            .await?;
            let text = events
                .into_iter()
                .filter_map(|event| match event {
                    LanguageModelCompletionEvent::Text(text) => Some(text),
                    _ => None,
                })
                .collect::<String>();
            assert_eq!(text, "Hello", "{protocol:?}");
            anyhow::Ok(())
        })?;
    }
    Ok(())
}
