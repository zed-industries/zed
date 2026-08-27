use std::sync::Arc;

use collections::HashMap;
use settings::RegisterSetting;

use crate::provider::{
    aimlapi, aimlapi::AimlapiSettings, anthropic, anthropic::AnthropicSettings,
    anthropic_compatible::AnthropicCompatibleSettings, bedrock, bedrock::AmazonBedrockSettings,
    cloud::ZedDotDevSettings, deepseek::DeepSeekSettings, google::GoogleSettings,
    llama_cpp::LlamaCppSettings, lmstudio::LmStudioSettings, mistral, mistral::MistralSettings,
    ollama::OllamaSettings, open_ai::OpenAiSettings, open_ai_compatible::OpenAiCompatibleSettings,
    open_router, open_router::OpenRouterSettings, opencode, opencode::OpenCodeSettings,
    resolve_custom_headers, vercel_ai_gateway::VercelAiGatewaySettings, x_ai::XAiSettings,
};

#[derive(Debug, RegisterSetting)]
pub struct AllLanguageModelSettings {
    pub aimlapi: AimlapiSettings,
    pub anthropic: AnthropicSettings,
    pub anthropic_compatible: HashMap<Arc<str>, AnthropicCompatibleSettings>,
    pub bedrock: AmazonBedrockSettings,
    pub deepseek: DeepSeekSettings,
    pub google: GoogleSettings,
    pub llama_cpp: LlamaCppSettings,
    pub lmstudio: LmStudioSettings,
    pub mistral: MistralSettings,
    pub ollama: OllamaSettings,
    pub opencode: OpenCodeSettings,
    pub open_router: OpenRouterSettings,
    pub openai: OpenAiSettings,
    pub openai_compatible: HashMap<Arc<str>, OpenAiCompatibleSettings>,
    pub vercel_ai_gateway: VercelAiGatewaySettings,
    pub x_ai: XAiSettings,
    pub zed_dot_dev: ZedDotDevSettings,
}

fn custom_headers_from(
    provider_name: &str,
    raw: Option<HashMap<String, String>>,
    reserved: &[&str],
) -> http_client::CustomHeaders {
    raw.as_ref()
        .filter(|map| !map.is_empty())
        .map(|map| resolve_custom_headers(provider_name, map, reserved))
        .unwrap_or_default()
}

/// aimlapi.com's attribution pair, prepended to whatever custom headers the
/// user configured. Doing it here — at settings-resolution time — is what makes
/// "every aimlapi.com request is attributed" true by construction: every
/// outbound call in the provider reads these resolved headers, so a new call
/// site cannot forget them.
///
/// The pair is filtered out of the user's own map first (it is listed in the
/// provider's `RESERVED_HEADER_NAMES`), so a settings entry can neither
/// override nor duplicate it.
fn aimlapi_headers_from(raw: Option<HashMap<String, String>>) -> http_client::CustomHeaders {
    use http_client::http::{HeaderName, HeaderValue};

    let mut headers: Vec<(HeaderName, HeaderValue)> = Vec::new();
    for (name, value) in [
        (aimlapi::AIMLAPI_SOURCE_HEADER, aimlapi::AIMLAPI_SOURCE),
        (
            aimlapi::AIMLAPI_PARTNER_ID_HEADER,
            aimlapi::AIMLAPI_PARTNER_ID,
        ),
    ] {
        match (
            HeaderName::from_bytes(name.as_bytes()),
            HeaderValue::from_str(value),
        ) {
            (Ok(name), Ok(value)) => headers.push((name, value)),
            _ => log::warn!("aimlapi.com attribution header `{name}` is not a valid header"),
        }
    }
    headers.extend(
        custom_headers_from("aimlapi.com", raw, aimlapi::RESERVED_HEADER_NAMES)
            .iter()
            .map(|(name, value)| (name.clone(), value.clone())),
    );
    http_client::CustomHeaders::new(headers)
}

impl settings::Settings for AllLanguageModelSettings {
    const PRESERVED_KEYS: Option<&'static [&'static str]> = Some(&["version"]);

    fn from_settings(content: &settings::SettingsContent) -> Self {
        let language_models = content.language_models.clone().unwrap();
        let aimlapi = language_models.aimlapi.unwrap();
        let anthropic = language_models.anthropic.unwrap();
        let anthropic_compatible = language_models.anthropic_compatible.unwrap();
        let bedrock = language_models.bedrock.unwrap();
        let deepseek = language_models.deepseek.unwrap();
        let google = language_models.google.unwrap();
        let llama_cpp = language_models.llama_cpp.unwrap();
        let lmstudio = language_models.lmstudio.unwrap();
        let mistral = language_models.mistral.unwrap();
        let ollama = language_models.ollama.unwrap();
        let opencode = language_models.opencode.unwrap();
        let open_router = language_models.open_router.unwrap();
        let openai = language_models.openai.unwrap();
        let openai_compatible = language_models.openai_compatible.unwrap();
        let vercel_ai_gateway = language_models.vercel_ai_gateway.unwrap();
        let x_ai = language_models.x_ai.unwrap();
        let zed_dot_dev = language_models.zed_dot_dev.unwrap();
        Self {
            aimlapi: AimlapiSettings {
                api_url: aimlapi.api_url.unwrap(),
                available_models: aimlapi.available_models.unwrap_or_default(),
                custom_headers: aimlapi_headers_from(aimlapi.custom_headers),
            },
            anthropic: AnthropicSettings {
                api_url: anthropic.api_url.unwrap(),
                available_models: anthropic.available_models.unwrap_or_default(),
                custom_headers: custom_headers_from(
                    "Anthropic",
                    anthropic.custom_headers,
                    anthropic::RESERVED_HEADER_NAMES,
                ),
            },
            anthropic_compatible: anthropic_compatible
                .into_iter()
                .map(|(key, value)| {
                    let provider_label = format!("Anthropic Compatible ({key})");
                    (
                        key,
                        AnthropicCompatibleSettings {
                            api_url: value.api_url,
                            available_models: value.available_models,
                            custom_headers: custom_headers_from(
                                &provider_label,
                                value.custom_headers,
                                anthropic::RESERVED_HEADER_NAMES,
                            ),
                        },
                    )
                })
                .collect(),
            bedrock: AmazonBedrockSettings {
                available_models: bedrock.available_models.unwrap_or_default(),
                mantle_available_models: bedrock.mantle_available_models.unwrap_or_default(),
                custom_headers: custom_headers_from(
                    "Amazon Bedrock",
                    bedrock.custom_headers,
                    bedrock::RESERVED_HEADER_NAMES,
                ),
                region: bedrock.region,
                endpoint: bedrock.endpoint_url, // todo(should be api_url)
                profile_name: bedrock.profile,
                role_arn: None, // todo(was never a setting for this...)
                authentication_method: bedrock.authentication_method.map(Into::into),
                allow_global: bedrock.allow_global,
                guardrail_identifier: bedrock.guardrail_identifier,
                guardrail_version: bedrock.guardrail_version,
            },
            deepseek: DeepSeekSettings {
                api_url: deepseek.api_url.unwrap(),
                available_models: deepseek.available_models.unwrap_or_default(),
                custom_headers: custom_headers_from("DeepSeek", deepseek.custom_headers, &[]),
            },
            google: GoogleSettings {
                api_url: google.api_url.unwrap(),
                available_models: google.available_models.unwrap_or_default(),
                custom_headers: custom_headers_from("Google AI", google.custom_headers, &[]),
            },
            llama_cpp: LlamaCppSettings {
                api_url: llama_cpp.api_url.unwrap(),
                auto_discover: llama_cpp.auto_discover.unwrap_or(true),
                available_models: llama_cpp.available_models.unwrap_or_default(),
                context_window: llama_cpp.context_window,
                custom_headers: custom_headers_from("llama.cpp", llama_cpp.custom_headers, &[]),
            },
            lmstudio: LmStudioSettings {
                api_url: lmstudio.api_url.unwrap(),
                available_models: lmstudio.available_models.unwrap_or_default(),
                custom_headers: custom_headers_from("LM Studio", lmstudio.custom_headers, &[]),
            },
            mistral: MistralSettings {
                api_url: mistral.api_url.unwrap(),
                available_models: mistral.available_models.unwrap_or_default(),
                custom_headers: custom_headers_from(
                    "Mistral",
                    mistral.custom_headers,
                    mistral::RESERVED_HEADER_NAMES,
                ),
            },
            ollama: OllamaSettings {
                api_url: ollama.api_url.unwrap(),
                auto_discover: ollama.auto_discover.unwrap_or(true),
                available_models: ollama.available_models.unwrap_or_default(),
                context_window: ollama.context_window,
                custom_headers: custom_headers_from("Ollama", ollama.custom_headers, &[]),
            },
            opencode: OpenCodeSettings {
                api_url: opencode.api_url.unwrap(),
                available_models: opencode.available_models.unwrap_or_default(),
                custom_headers: custom_headers_from(
                    "OpenCode",
                    opencode.custom_headers,
                    opencode::RESERVED_HEADER_NAMES,
                ),
                show_zen_models: opencode.show_zen_models.unwrap_or(true),
                show_go_models: opencode.show_go_models.unwrap_or(true),
            },
            open_router: OpenRouterSettings {
                api_url: open_router.api_url.unwrap(),
                available_models: open_router.available_models.unwrap_or_default(),
                custom_headers: custom_headers_from(
                    "OpenRouter",
                    open_router.custom_headers,
                    open_router::RESERVED_HEADER_NAMES,
                ),
            },
            openai: OpenAiSettings {
                api_url: openai.api_url.unwrap(),
                available_models: openai.available_models.unwrap_or_default(),
                custom_headers: custom_headers_from("OpenAI", openai.custom_headers, &[]),
            },
            openai_compatible: openai_compatible
                .into_iter()
                .map(|(key, value)| {
                    let provider_label = format!("OpenAI Compatible ({key})");
                    (
                        key,
                        OpenAiCompatibleSettings {
                            api_url: value.api_url,
                            available_models: value.available_models,
                            custom_headers: custom_headers_from(
                                &provider_label,
                                value.custom_headers,
                                &[],
                            ),
                        },
                    )
                })
                .collect(),
            vercel_ai_gateway: VercelAiGatewaySettings {
                api_url: vercel_ai_gateway.api_url.unwrap(),
                available_models: vercel_ai_gateway.available_models.unwrap_or_default(),
                custom_headers: custom_headers_from(
                    "Vercel AI Gateway",
                    vercel_ai_gateway.custom_headers,
                    &[],
                ),
            },
            x_ai: XAiSettings {
                api_url: x_ai.api_url.unwrap(),
                available_models: x_ai.available_models.unwrap_or_default(),
                custom_headers: custom_headers_from("xAI", x_ai.custom_headers, &[]),
            },
            zed_dot_dev: ZedDotDevSettings {
                available_models: zed_dot_dev.available_models.unwrap_or_default(),
            },
        }
    }
}

#[cfg(test)]
mod aimlapi_tests {
    use super::*;

    fn header_pairs(headers: &http_client::CustomHeaders) -> Vec<(String, String)> {
        headers
            .iter()
            .map(|(name, value)| {
                (
                    // `HeaderName` lowercases on construction (the HTTP/2 rule
                    // the `http` crate enforces), so compare on the lowercase
                    // form rather than the constant's display casing.
                    name.as_str().to_ascii_lowercase(),
                    value.to_str().unwrap_or_default().to_string(),
                )
            })
            .collect()
    }

    fn lower(name: &str) -> String {
        name.to_ascii_lowercase()
    }

    /// `AllLanguageModelSettings::from_settings` unwraps every provider's
    /// section, so a provider missing from `default.json` is not a missing
    /// default — it is a panic on startup, before any UI exists to report it.
    /// Nothing else covers that: the type checks, the unit tests pass, and the
    /// binary dies at launch. This asserts the entry is there.
    #[test]
    fn default_json_declares_the_aimlapi_provider() {
        let default_json = include_str!("../../../assets/settings/default.json");
        let value: serde_json_lenient::Value =
            serde_json_lenient::from_str(default_json).expect("default.json should parse");

        let language_models = value
            .get("language_models")
            .expect("default.json should have 'language_models'");
        let aimlapi = language_models
            .get("aimlapi")
            .expect("language_models should declare 'aimlapi' or startup panics on unwrap");

        assert_eq!(
            aimlapi.get("api_url").and_then(|v| v.as_str()),
            Some("https://api.aimlapi.com/v1"),
        );
    }

    /// HEADERS.md requires the attribution pair on EVERY aimlapi.com request.
    /// The provider reads these resolved headers from three separate call
    /// sites, so proving they exist here proves it for all of them.
    #[test]
    fn attribution_pair_is_present_without_any_user_config() {
        let pairs = header_pairs(&aimlapi_headers_from(None));

        assert!(pairs.contains(&(
            lower(aimlapi::AIMLAPI_SOURCE_HEADER),
            "agent/zed".to_string()
        )));
        assert!(pairs.contains(&(
            lower(aimlapi::AIMLAPI_PARTNER_ID_HEADER),
            aimlapi::AIMLAPI_PARTNER_ID.to_string()
        )));
    }

    /// The pair is listed in the provider's RESERVED_HEADER_NAMES, so a user
    /// entry must not be able to spoof the partner id or blank the source —
    /// that would silently misattribute or drop the traffic.
    #[test]
    fn user_settings_cannot_override_the_attribution_pair() {
        let mut raw = HashMap::default();
        raw.insert(
            lower(aimlapi::AIMLAPI_PARTNER_ID_HEADER),
            "part_somebody_else".to_string(),
        );
        raw.insert(lower(aimlapi::AIMLAPI_SOURCE_HEADER), "web".to_string());
        raw.insert("X-Custom".to_string(), "kept".to_string());

        let pairs = header_pairs(&aimlapi_headers_from(Some(raw)));

        // the caller's own header survives
        assert!(pairs.contains(&("x-custom".to_string(), "kept".to_string())));
        // ours are untouched and appear exactly once each
        assert_eq!(
            pairs
                .iter()
                .filter(|(name, _)| name.eq_ignore_ascii_case(aimlapi::AIMLAPI_PARTNER_ID_HEADER))
                .count(),
            1
        );
        assert!(pairs.contains(&(
            lower(aimlapi::AIMLAPI_PARTNER_ID_HEADER),
            aimlapi::AIMLAPI_PARTNER_ID.to_string()
        )));
        assert!(!pairs.iter().any(|(_, value)| value == "part_somebody_else"));
    }
}

#[cfg(test)]
mod aimlapi_wire_tests {
    use super::*;
    use futures::StreamExt as _;
    use http_client::{AsyncBody, FakeHttpClient, Response, http::HeaderMap};
    use parking_lot::Mutex;
    use std::sync::Arc;

    /// The header-resolution tests above prove the pair is in the settings the
    /// provider reads. That is one step short of the claim that matters: that a
    /// request actually leaves carrying them. This drives the real
    /// `open_ai::stream_completion` — the function both provider call sites use
    /// — through a fake transport and inspects the request it built.
    #[gpui::test]
    async fn attribution_pair_reaches_the_outgoing_request() {
        let captured: Arc<Mutex<Option<HeaderMap>>> = Arc::new(Mutex::new(None));
        let sink = captured.clone();

        let client = FakeHttpClient::create(move |req| {
            let sink = sink.clone();
            async move {
                *sink.lock() = Some(req.headers().clone());
                Ok(Response::builder()
                    .status(200)
                    .body(AsyncBody::from("data: [DONE]\n\n"))
                    .unwrap())
            }
        });

        let request = open_ai::Request {
            model: "openai/gpt-5.6-terra".into(),
            messages: Vec::new(),
            stream: true,
            stream_options: None,
            max_completion_tokens: None,
            max_tokens: None,
            stop: Vec::new(),
            temperature: None,
            tool_choice: None,
            parallel_tool_calls: None,
            tools: Vec::new(),
            prompt_cache_key: None,
            reasoning_effort: None,
            service_tier: None,
        };

        let headers = aimlapi_headers_from(None);
        if let Ok(stream) = open_ai::stream_completion(
            client.as_ref(),
            "aimlapi.com",
            "https://api.aimlapi.com/v1",
            "test-key",
            request,
            &headers,
        )
        .await
        {
            // Drain so the request is definitely issued; the body is irrelevant.
            let _ = stream.collect::<Vec<_>>().await;
        }

        let sent = captured.lock().take().expect("no request was issued");
        assert_eq!(
            sent.get("x-aimlapi-source").map(|v| v.to_str().unwrap()),
            Some("agent/zed"),
        );
        assert_eq!(
            sent.get("x-aimlapi-partner-id")
                .map(|v| v.to_str().unwrap()),
            Some(crate::provider::aimlapi::AIMLAPI_PARTNER_ID),
        );
    }
}
