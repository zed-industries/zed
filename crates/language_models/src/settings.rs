use std::sync::Arc;

use collections::HashMap;
use settings::RegisterSetting;

use crate::provider::{
    anthropic, anthropic::AnthropicSettings, anthropic_compatible::AnthropicCompatibleSettings,
    bedrock, bedrock::AmazonBedrockSettings, cloud::ZedDotDevSettings, deepseek::DeepSeekSettings,
    google::GoogleSettings, llama_cpp::LlamaCppSettings, lmstudio::LmStudioSettings, mistral,
    mistral::MistralSettings, ollama::OllamaSettings, open_ai::OpenAiSettings,
    open_ai_compatible::OpenAiCompatibleSettings, open_router, open_router::OpenRouterSettings,
    opencode, opencode::OpenCodeSettings, resolve_custom_headers,
    vercel_ai_gateway::VercelAiGatewaySettings, x_ai::XAiSettings,
};

#[derive(Debug, RegisterSetting)]
pub struct AllLanguageModelSettings {
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

impl settings::Settings for AllLanguageModelSettings {
    const PRESERVED_KEYS: Option<&'static [&'static str]> = Some(&["version"]);

    fn from_settings(content: &settings::SettingsContent) -> Self {
        let language_models = content.language_models.clone().unwrap();
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
                show_free_models: opencode.show_free_models.unwrap_or(true),
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
