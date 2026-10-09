use anyhow::{Context as _, Result};
use cloud_llm_client::predict_edits_v3::{RawCompletionRequest, RawCompletionResponse};
use futures::AsyncReadExt as _;
use gpui::{App, AppContext as _, Entity, Global, SharedString, Task, http_client};
use language::language_settings::{OpenAiCompatibleEditPredictionSettings, all_language_settings};
use language_model::{ApiKeyState, EnvVar, env_var};
use std::sync::Arc;

pub fn open_ai_compatible_api_url(cx: &App) -> SharedString {
    all_language_settings(None, cx)
        .edit_predictions
        .open_ai_compatible_api
        .as_ref()
        .map(|settings| settings.api_url.clone())
        .unwrap_or_default()
        .into()
}

pub const OPEN_AI_COMPATIBLE_CREDENTIALS_USERNAME: &str = "openai-compatible-api-token";
pub static OPEN_AI_COMPATIBLE_TOKEN_ENV_VAR: std::sync::LazyLock<EnvVar> =
    env_var!("ZED_OPEN_AI_COMPATIBLE_EDIT_PREDICTION_API_KEY");

struct GlobalOpenAiCompatibleApiKey(Entity<ApiKeyState>);

impl Global for GlobalOpenAiCompatibleApiKey {}

pub fn open_ai_compatible_api_token(cx: &mut App) -> Entity<ApiKeyState> {
    if let Some(global) = cx.try_global::<GlobalOpenAiCompatibleApiKey>() {
        return global.0.clone();
    }

    let entity = cx.new(|cx| {
        ApiKeyState::new(
            open_ai_compatible_api_url(cx),
            OPEN_AI_COMPATIBLE_TOKEN_ENV_VAR.clone(),
        )
    });
    cx.set_global(GlobalOpenAiCompatibleApiKey(entity.clone()));
    entity
}

pub fn load_open_ai_compatible_api_token(
    cx: &mut App,
) -> Task<Result<(), language_model::AuthenticateError>> {
    let credentials_provider = zed_credentials_provider::global(cx);
    let api_url = open_ai_compatible_api_url(cx);
    open_ai_compatible_api_token(cx).update(cx, |key_state, cx| {
        key_state.load_if_needed(api_url, |s| s, credentials_provider, cx)
    })
}

pub fn load_open_ai_compatible_api_key_if_needed(
    provider: settings::EditPredictionProvider,
    cx: &mut App,
) -> Option<Arc<str>> {
    if provider != settings::EditPredictionProvider::OpenAiCompatibleApi {
        return None;
    }
    _ = load_open_ai_compatible_api_token(cx);
    let url = open_ai_compatible_api_url(cx);
    return open_ai_compatible_api_token(cx).read(cx).key(&url);
}

pub(crate) async fn send_custom_server_request(
    provider: settings::EditPredictionProvider,
    settings: &OpenAiCompatibleEditPredictionSettings,
    prompt: String,
    max_tokens: u32,
    stop_tokens: Vec<String>,
    api_key: Option<Arc<str>>,
    http_client: &Arc<dyn http_client::HttpClient>,
) -> Result<(String, String)> {
    match provider {
        settings::EditPredictionProvider::Ollama => {
            let response = crate::ollama::make_request(
                settings.clone(),
                prompt,
                stop_tokens,
                http_client.clone(),
            )
            .await?;
            Ok((response.response, response.created_at))
        }
        _ => {
            let request = RawCompletionRequest {
                model: settings.model.clone(),
                prompt,
                max_tokens: Some(max_tokens),
                temperature: None,
                stop: stop_tokens
                    .into_iter()
                    .map(std::borrow::Cow::Owned)
                    .collect(),
                environment: None,
            };

            let request_body = serde_json::to_string(&request)?;
            let body = send_request(&settings.api_url, request_body, api_key, http_client).await?;

            let parsed: RawCompletionResponse =
                serde_json::from_str(&body).context("Failed to parse completion response")?;
            let text = parsed
                .choices
                .into_iter()
                .next()
                .map(|choice| choice.text)
                .unwrap_or_default();
            Ok((text, parsed.id))
        }
    }
}

pub(crate) async fn send_qwen_server_request(
    api: crate::qwen::Api,
    settings: &OpenAiCompatibleEditPredictionSettings,
    prefix: &str,
    suffix: &str,
    api_key: Option<Arc<str>>,
    http_client: &Arc<dyn http_client::HttpClient>,
) -> Result<(String, String)> {
    let request_body =
        api.request_body(&settings.model, prefix, suffix, settings.max_output_tokens)?;
    let body = send_request(&settings.api_url, request_body, api_key, http_client).await?;
    api.parse_response(&body)
}

async fn send_request(
    api_url: &str,
    request_body: String,
    api_key: Option<Arc<str>>,
    http_client: &Arc<dyn http_client::HttpClient>,
) -> Result<String> {
    let mut http_request_builder = http_client::Request::builder()
        .method(http_client::Method::POST)
        .uri(api_url)
        .header("Content-Type", "application/json");

    if let Some(api_key) = api_key {
        http_request_builder =
            http_request_builder.header("Authorization", format!("Bearer {}", api_key));
    }

    let http_request = http_request_builder.body(http_client::AsyncBody::from(request_body))?;
    let mut response = http_client.send(http_request).await?;
    let status = response.status();
    let mut body = String::new();
    response.body_mut().read_to_string(&mut body).await?;

    if !status.is_success() {
        anyhow::bail!("custom server error: {} - {}", status, body);
    }

    Ok(body)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::qwen::Api;
    use gpui::http_client::FakeHttpClient;
    use serde_json::{Value, json};

    #[test]
    fn qwen_message_endpoints_send_authenticated_partial_requests() {
        futures::executor::block_on(async {
            for (api, url, response_body) in [
                (
                    Api::Chat,
                    "https://maas.qianwenaiapi.com/compatible-mode/v1/chat/completions",
                    json!({"id": "chat-test", "choices": [{"message": {"role": "assistant", "content": "    return n\n"}}]}),
                ),
                (
                    Api::Native,
                    "https://maas.qianwenaiapi.com/api/v1/services/aigc/text-generation/generation",
                    json!({"request_id": "native-test", "output": {"choices": [{"message": {"role": "assistant", "content": "    return n\n"}}]}}),
                ),
            ] {
                let settings = OpenAiCompatibleEditPredictionSettings {
                    model: "qwen3-coder-flash".into(),
                    api_url: url.into(),
                    max_output_tokens: 1000,
                    ..Default::default()
                };
                let http_client: Arc<dyn http_client::HttpClient> = FakeHttpClient::create(
                    move |mut request| {
                        let response_body = response_body.clone();
                        async move {
                            assert_eq!(request.method(), http_client::Method::POST);
                            assert_eq!(request.uri().to_string(), url);
                            assert_eq!(request.headers()["Authorization"], "Bearer test-key");
                            assert_eq!(request.headers()["Content-Type"], "application/json");
                            let mut body = String::new();
                            request.body_mut().read_to_string(&mut body).await?;
                            let body: Value = serde_json::from_str(&body)?;
                            let messages = match api {
                                Api::Chat => &body["messages"],
                                Api::Native => &body["input"]["messages"],
                            };
                            assert_eq!(
                                messages[1],
                                json!({"role": "assistant", "content": "def f(n):\n", "partial": true})
                            );
                            assert!(
                                messages[0]["content"]
                                    .as_str()
                                    .unwrap()
                                    .contains("\nprint(f(1))")
                            );
                            Ok(http_client::Response::builder()
                                .status(200)
                                .body(response_body.to_string().into())?)
                        }
                    },
                );
                let result = send_qwen_server_request(
                    api,
                    &settings,
                    "def f(n):\n",
                    "\nprint(f(1))",
                    Some("test-key".into()),
                    &http_client,
                )
                .await
                .unwrap();
                assert_eq!(result.0, "    return n\n");
                assert_eq!(
                    result.1,
                    if api == Api::Chat {
                        "chat-test"
                    } else {
                        "native-test"
                    }
                );
            }
        });
    }

    #[test]
    fn qwen_http_errors_preserve_status_and_server_message() {
        futures::executor::block_on(async {
            let http_client: Arc<dyn http_client::HttpClient> =
                FakeHttpClient::create(|request| async move {
                    assert!(!request.headers().contains_key("Authorization"));
                    Ok(http_client::Response::builder()
                        .status(400)
                        .body(r#"{"error":{"message":"invalid partial request"}}"#.into())?)
                });
            let settings = OpenAiCompatibleEditPredictionSettings {
                api_url: "https://example.com/v1/chat/completions".into(),
                ..Default::default()
            };
            let error = send_qwen_server_request(Api::Chat, &settings, "", "", None, &http_client)
                .await
                .unwrap_err()
                .to_string();
            assert!(error.contains("400 Bad Request"), "{error}");
            assert!(error.contains("invalid partial request"), "{error}");
        });
    }

    #[test]
    fn raw_completion_requests_keep_the_existing_protocol() {
        futures::executor::block_on(async {
            let http_client: Arc<dyn http_client::HttpClient> = FakeHttpClient::create(
                |mut request| async move {
                    let mut body = String::new();
                    request.body_mut().read_to_string(&mut body).await?;
                    let body: Value = serde_json::from_str(&body)?;
                    assert_eq!(
                        body,
                        json!({"model": "qwen2.5-coder", "prompt": "<|fim_prefix|>before<|fim_suffix|>after<|fim_middle|>", "max_tokens": 64, "stop": ["<|endoftext|>"]})
                    );
                    Ok(http_client::Response::builder().status(200).body(json!({
                    "id": "raw-test", "object": "text_completion", "created": 0, "model": "qwen2.5-coder",
                    "choices": [{"text": "missing", "finish_reason": "stop"}],
                    "usage": {"prompt_tokens": 1, "completion_tokens": 1, "total_tokens": 2}
                }).to_string().into())?)
                },
            );
            let settings = OpenAiCompatibleEditPredictionSettings {
                model: "qwen2.5-coder".into(),
                api_url: "http://localhost:8080/v1/completions".into(),
                ..Default::default()
            };
            let result = send_custom_server_request(
                settings::EditPredictionProvider::OpenAiCompatibleApi,
                &settings,
                "<|fim_prefix|>before<|fim_suffix|>after<|fim_middle|>".into(),
                64,
                vec!["<|endoftext|>".into()],
                None,
                &http_client,
            )
            .await
            .unwrap();
            assert_eq!(result, ("missing".into(), "raw-test".into()));
        });
    }
}
