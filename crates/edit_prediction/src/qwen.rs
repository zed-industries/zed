use crate::open_ai_compatible;
use anyhow::{Context as _, Result};
use gpui::http_client;
use language::language_settings::OpenAiCompatibleEditPredictionSettings;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

/// Qwen's Partial Mode is a message-based protocol. Raw completion endpoints
/// (including self-hosted Qwen FIM models) continue to use the existing protocol.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Api {
    Chat,
    Native,
}

impl Api {
    fn from_url(url: &str) -> Option<Self> {
        let path = url.split('?').next()?.trim_end_matches('/');
        if path.ends_with("/chat/completions") {
            Some(Self::Chat)
        } else if path.ends_with("/services/aigc/text-generation/generation") {
            Some(Self::Native)
        } else {
            None
        }
    }

    fn request_body(
        self,
        model: &str,
        prefix: &str,
        suffix: &str,
        max_tokens: u32,
    ) -> Result<String> {
        let mut instruction = "Complete the code at the cursor. Output only the missing code, without Markdown fences or repeating the code before or after the cursor.".to_string();
        if !suffix.is_empty() {
            instruction.push_str("\nThe code after the cursor is:\n<suffix>\n");
            instruction.push_str(suffix);
            instruction.push_str("\n</suffix>");
        }
        let messages = json!([
            {"role": "user", "content": instruction},
            {"role": "assistant", "content": prefix, "partial": true},
        ]);
        // Chat-completion APIs may limit stop sequences to four. Do not send
        // the full cross-model FIM stop list to these endpoints.
        let stop = [
            "<|endoftext|>",
            "<|im_end|>",
            "<|fim_suffix|>",
            "<|fim_middle|>",
        ];
        let request = match self {
            Self::Chat => json!({
                "model": model,
                "messages": messages,
                "max_tokens": max_tokens,
                "stream": false,
                "stop": stop,
            }),
            Self::Native => json!({
                "model": model,
                "input": {"messages": messages},
                "parameters": {
                    "result_format": "message",
                    "max_tokens": max_tokens,
                    "stop": stop,
                },
            }),
        };
        Ok(serde_json::to_string(&request)?)
    }

    fn parse_response(self, body: &str) -> Result<(String, String)> {
        let (choices, request_id) = match self {
            Self::Chat => {
                let response: ChatResponse =
                    serde_json::from_str(body).context("Failed to parse Qwen chat response")?;
                (response.choices, response.id)
            }
            Self::Native => {
                let response: NativeResponse =
                    serde_json::from_str(body).context("Failed to parse Qwen native response")?;
                (response.output.choices, response.request_id)
            }
        };
        let choice = choices
            .into_iter()
            .next()
            .context("Qwen response contained no choices")?;
        Ok((choice.message.content, request_id))
    }
}

pub(crate) async fn try_request(
    settings: &OpenAiCompatibleEditPredictionSettings,
    prefix: &str,
    suffix: &str,
    api_key: Option<Arc<str>>,
    http_client: &Arc<dyn http_client::HttpClient>,
) -> Result<Option<(String, String)>> {
    // Decline unsupported URLs before sending so the caller can retain the
    // original protocol for raw completion servers. Request errors propagate.
    let Some(api) = Api::from_url(&settings.api_url) else {
        return Ok(None);
    };
    let request_body =
        api.request_body(&settings.model, prefix, suffix, settings.max_output_tokens)?;
    let body =
        open_ai_compatible::send_request(&settings.api_url, request_body, api_key, http_client)
            .await?;
    api.parse_response(&body).map(Some)
}

#[derive(Deserialize)]
struct ChatResponse {
    id: String,
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct NativeResponse {
    request_id: String,
    output: NativeOutput,
}

#[derive(Deserialize)]
struct NativeOutput {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Deserialize)]
struct Message {
    content: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::AsyncReadExt as _;
    use gpui::http_client::FakeHttpClient;
    use serde_json::{Value, json};

    const PREFIX: &str =
        "def calculate_fibonacci(n):\n    if n <= 1:\n        return n\n    else:\n";
    const SUFFIX: &str = "\n\nprint(calculate_fibonacci(10))\n";
    const COMPLETION: &str = "        return calculate_fibonacci(n-1) + calculate_fibonacci(n-2)";

    #[test]
    fn qwen_message_endpoints_send_authenticated_partial_requests() -> Result<()> {
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
                                    .context("Expected text in the user message")?
                                    .contains("\nprint(f(1))")
                            );
                            Ok(http_client::Response::builder()
                                .status(200)
                                .body(response_body.to_string().into())?)
                        }
                    },
                );
                let result = try_request(
                    &settings,
                    "def f(n):\n",
                    "\nprint(f(1))",
                    Some("test-key".into()),
                    &http_client,
                )
                .await?
                .context("Expected the message endpoint to support Partial Mode")?;
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
            Ok(())
        })
    }

    #[test]
    fn qwen_http_errors_preserve_status_and_server_message() -> Result<()> {
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
            let error = try_request(&settings, "", "", None, &http_client)
                .await
                .err()
                .context("Expected the Qwen request to fail with HTTP 400")?
                .to_string();
            assert!(error.contains("400 Bad Request"), "{error}");
            assert!(error.contains("invalid partial request"), "{error}");
            Ok(())
        })
    }

    #[test]
    fn raw_completion_endpoints_are_declined_without_sending() -> Result<()> {
        futures::executor::block_on(async {
            let http_client: Arc<dyn http_client::HttpClient> =
                FakeHttpClient::create(|_request| async move {
                    anyhow::bail!("Unsupported endpoint must not send a request")
                });
            let settings = OpenAiCompatibleEditPredictionSettings {
                api_url: "https://example.com/v1/completions".into(),
                ..Default::default()
            };
            assert!(
                try_request(&settings, PREFIX, SUFFIX, None, &http_client)
                    .await?
                    .is_none()
            );
            Ok(())
        })
    }

    #[test]
    fn recognizes_chat_and_native_endpoints_without_changing_raw_completions() {
        for (url, expected) in [
            (
                "https://maas.qianwenaiapi.com/compatible-mode/v1/chat/completions",
                Some(Api::Chat),
            ),
            (
                "https://maas.qianwenaiapi.com/api/v1/services/aigc/text-generation/generation",
                Some(Api::Native),
            ),
            (
                "https://example.com/v1/chat/completions/?workspace=test",
                Some(Api::Chat),
            ),
            ("http://localhost:8080/v1/completions", None),
            ("http://localhost:11434/api/generate", None),
        ] {
            assert_eq!(Api::from_url(url), expected, "{url}");
        }
    }

    #[test]
    fn chat_request_uses_partial_assistant_and_preserves_code_context() -> Result<()> {
        let request: Value = serde_json::from_str(&Api::Chat.request_body(
            "qwen3-coder-flash",
            PREFIX,
            SUFFIX,
            1000,
        )?)?;
        assert_eq!(request["model"], "qwen3-coder-flash");
        assert_eq!(request["max_tokens"], 1000);
        assert_eq!(request["stream"], false);
        assert!(request.get("prompt").is_none());
        assert!(request.get("input").is_none());
        let messages = request["messages"]
            .as_array()
            .context("Expected a messages array")?;
        assert_eq!(messages.len(), 2);
        assert_eq!(messages[0]["role"], "user");
        assert!(
            messages[0]["content"]
                .as_str()
                .context("Expected text in the user message")?
                .contains(SUFFIX)
        );
        assert_eq!(
            messages[1],
            json!({"role": "assistant", "content": PREFIX, "partial": true})
        );
        assert!(
            request["stop"]
                .as_array()
                .context("Expected a stop-sequence array")?
                .len()
                <= 4
        );
        Ok(())
    }

    #[test]
    fn native_request_matches_dashscope_envelope() -> Result<()> {
        let request: Value = serde_json::from_str(&Api::Native.request_body(
            "qwen3-coder-plus",
            PREFIX,
            "",
            256,
        )?)?;
        assert_eq!(request["model"], "qwen3-coder-plus");
        assert!(request.get("messages").is_none());
        assert!(request.get("prompt").is_none());
        assert!(request.get("max_tokens").is_none());
        assert_eq!(request["parameters"]["max_tokens"], 256);
        assert_eq!(request["parameters"]["result_format"], "message");
        assert_eq!(request["input"]["messages"][0]["role"], "user");
        assert_eq!(
            request["input"]["messages"][1],
            json!({"role": "assistant", "content": PREFIX, "partial": true})
        );
        Ok(())
    }

    #[test]
    fn empty_prefix_is_still_a_partial_assistant_message() -> Result<()> {
        let request: Value = serde_json::from_str(&Api::Chat.request_body(
            "qwen3-coder-flash",
            "",
            "剩余代码\n",
            64,
        )?)?;
        assert_eq!(
            request["messages"][1],
            json!({"role": "assistant", "content": "", "partial": true})
        );
        assert!(
            request["messages"][0]["content"]
                .as_str()
                .context("Expected text in the user message")?
                .contains("剩余代码\n")
        );
        Ok(())
    }

    #[test]
    fn chat_response_returns_only_generated_code_and_request_id() -> Result<()> {
        let response = json!({
            "id": "chatcmpl-test", "object": "chat.completion", "created": 0,
            "model": "qwen3-coder-flash",
            "choices": [{"index": 0, "message": {"role": "assistant", "content": COMPLETION}, "finish_reason": "stop", "logprobs": null}],
            "usage": {"prompt_tokens": 48, "completion_tokens": 19, "total_tokens": 67}
        });
        assert_eq!(
            Api::Chat.parse_response(&response.to_string())?,
            (COMPLETION.into(), "chatcmpl-test".into())
        );
        Ok(())
    }

    #[test]
    fn native_response_returns_only_generated_code_and_request_id() -> Result<()> {
        let response = json!({
            "request_id": "dashscope-test",
            "output": {"choices": [{"message": {"role": "assistant", "content": COMPLETION}, "finish_reason": "stop"}]},
            "usage": {"input_tokens": 48, "output_tokens": 19, "total_tokens": 67}
        });
        assert_eq!(
            Api::Native.parse_response(&response.to_string())?,
            (COMPLETION.into(), "dashscope-test".into())
        );
        Ok(())
    }

    #[test]
    fn empty_completion_is_not_an_error() -> Result<()> {
        let response = r#"{"id":"empty","choices":[{"message":{"content":""}}]}"#;
        assert_eq!(
            Api::Chat.parse_response(response)?,
            (String::new(), "empty".into())
        );
        Ok(())
    }

    #[test]
    fn missing_choices_and_malformed_responses_are_errors_not_empty_predictions() {
        for (api, response) in [
            (Api::Chat, r#"{"id":"bad","choices":[]}"#),
            (
                Api::Native,
                r#"{"request_id":"bad","output":{"choices":[]}}"#,
            ),
            (
                Api::Chat,
                r#"{"id":"bad","choices":[{"message":{"content":null}}]}"#,
            ),
            (
                Api::Native,
                r#"{"code":"InvalidParameter","message":"invalid request"}"#,
            ),
            (Api::Chat, "not json"),
        ] {
            assert!(api.parse_response(response).is_err(), "{response}");
        }
    }
}
