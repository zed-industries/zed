use anyhow::Result;
use gpui::App;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};

use crate::provider::{
    self,
    anthropic::AnthropicSettings,
    bedrock::AmazonBedrockSettings,
    cloud::{self, ZedDotDevSettings},
    deepseek::DeepSeekSettings,
    google::GoogleSettings,
    lmstudio::LmStudioSettings,
    mistral::MistralSettings,
    ollama::OllamaSettings,
    open_ai::OpenAiSettings,
    open_router::OpenRouterSettings,
    pollinations::PollinationsSettings,
    vercel::VercelSettings,
};

/// Initializes the language model settings.
pub fn init(cx: &mut App) {
    AllLanguageModelSettings::register(cx);
}

#[derive(Default)]
pub struct AllLanguageModelSettings {
    pub anthropic: AnthropicSettings,
    pub bedrock: AmazonBedrockSettings,
    pub ollama: OllamaSettings,
    pub openai: OpenAiSettings,
    pub pollinations: PollinationsSettings,

    pub open_router: OpenRouterSettings,
    pub zed_dot_dev: ZedDotDevSettings,
    pub google: GoogleSettings,
    pub vercel: VercelSettings,

    pub lmstudio: LmStudioSettings,
    pub deepseek: DeepSeekSettings,
    pub mistral: MistralSettings,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct AllLanguageModelSettingsContent {
    pub anthropic: Option<AnthropicSettingsContent>,
    pub bedrock: Option<AmazonBedrockSettingsContent>,
    pub ollama: Option<OllamaSettingsContent>,
    pub lmstudio: Option<LmStudioSettingsContent>,
    pub openai: Option<OpenAiSettingsContent>,
    pub pollinations: Option<PollinationsSettingsContent>,

    pub open_router: Option<OpenRouterSettingsContent>,
    #[serde(rename = "zed.dev")]
    pub zed_dot_dev: Option<ZedDotDevSettingsContent>,
    pub google: Option<GoogleSettingsContent>,
    pub deepseek: Option<DeepseekSettingsContent>,
    pub vercel: Option<VercelSettingsContent>,

    pub mistral: Option<MistralSettingsContent>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct AnthropicSettingsContent {
    pub api_url: Option<String>,
    pub available_models: Option<Vec<provider::anthropic::AvailableModel>>,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct AmazonBedrockSettingsContent {
    available_models: Option<Vec<provider::bedrock::AvailableModel>>,
    endpoint_url: Option<String>,
    region: Option<String>,
    profile: Option<String>,
    authentication_method: Option<provider::bedrock::BedrockAuthMethod>,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct OllamaSettingsContent {
    pub api_url: Option<String>,
    pub available_models: Option<Vec<provider::ollama::AvailableModel>>,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct LmStudioSettingsContent {
    pub api_url: Option<String>,
    pub available_models: Option<Vec<provider::lmstudio::AvailableModel>>,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct DeepseekSettingsContent {
    pub api_url: Option<String>,
    pub available_models: Option<Vec<provider::deepseek::AvailableModel>>,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct MistralSettingsContent {
    pub api_url: Option<String>,
    pub available_models: Option<Vec<provider::mistral::AvailableModel>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct OpenAiSettingsContent {
    pub api_url: Option<String>,
    pub available_models: Option<Vec<provider::open_ai::AvailableModel>>,
}

// Pollinations
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
#[serde(untagged)]
pub enum PollinationsSettingsContent {
    Versioned(VersionedPollinationsSettingsContent),
    Legacy(LegacyPollinationsSettingsContent),
}

impl PollinationsSettingsContent {
    pub fn upgrade(self) -> (PollinationsSettingsContentV1, bool) {
        match self {
            PollinationsSettingsContent::Legacy(content) => (
                PollinationsSettingsContentV1 {
                    api_url: content.api_url,
                    available_models: content.available_models.map(|models| {
                        models
                            .into_iter()
                            .filter_map(|model| match model {
                                pollinations::Model::Custom {
                                    name,
                                    display_name,
                                    max_tokens,
                                    max_output_tokens,
                                    max_completion_tokens,
                                } => Some(provider::pollinations::AvailableModel {
                                    name,
                                    max_tokens,
                                    max_output_tokens,
                                    display_name,
                                    max_completion_tokens,
                                }),
                                _ => None,
                            })
                            .collect()
                    }),
                },
                true,
            ),
            PollinationsSettingsContent::Versioned(content) => match content {
                VersionedPollinationsSettingsContent::V1(content) => (content, false),
            },
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct LegacyPollinationsSettingsContent {
    pub api_url: Option<String>,
    pub available_models: Option<Vec<pollinations::Model>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
#[serde(tag = "version")]
pub enum VersionedPollinationsSettingsContent {
    #[serde(rename = "1")]
    V1(PollinationsSettingsContentV1),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct PollinationsSettingsContentV1 {
    pub api_url: Option<String>,
    pub available_models: Option<Vec<provider::pollinations::AvailableModel>>,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct VercelSettingsContent {
    pub api_url: Option<String>,
    pub available_models: Option<Vec<provider::vercel::AvailableModel>>,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct GoogleSettingsContent {
    pub api_url: Option<String>,
    pub available_models: Option<Vec<provider::google::AvailableModel>>,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct ZedDotDevSettingsContent {
    available_models: Option<Vec<cloud::AvailableModel>>,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct OpenRouterSettingsContent {
    pub api_url: Option<String>,
    pub available_models: Option<Vec<provider::open_router::AvailableModel>>,
}

impl settings::Settings for AllLanguageModelSettings {
    const KEY: Option<&'static str> = Some("language_models");

    const PRESERVED_KEYS: Option<&'static [&'static str]> = Some(&["version"]);

    type FileContent = AllLanguageModelSettingsContent;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut App) -> Result<Self> {
        fn merge<T>(target: &mut T, value: Option<T>) {
            if let Some(value) = value {
                *target = value;
            }
        }

        let mut settings = AllLanguageModelSettings::default();

        for value in sources.defaults_and_customizations() {
            // Anthropic
            let anthropic = value.anthropic.clone();
            merge(
                &mut settings.anthropic.api_url,
                anthropic.as_ref().and_then(|s| s.api_url.clone()),
            );
            merge(
                &mut settings.anthropic.available_models,
                anthropic.as_ref().and_then(|s| s.available_models.clone()),
            );

            // Bedrock
            let bedrock = value.bedrock.clone();
            merge(
                &mut settings.bedrock.profile_name,
                bedrock.as_ref().map(|s| s.profile.clone()),
            );
            merge(
                &mut settings.bedrock.authentication_method,
                bedrock.as_ref().map(|s| s.authentication_method.clone()),
            );
            merge(
                &mut settings.bedrock.region,
                bedrock.as_ref().map(|s| s.region.clone()),
            );
            merge(
                &mut settings.bedrock.endpoint,
                bedrock.as_ref().map(|s| s.endpoint_url.clone()),
            );

            // Ollama
            let ollama = value.ollama.clone();

            merge(
                &mut settings.ollama.api_url,
                value.ollama.as_ref().and_then(|s| s.api_url.clone()),
            );
            merge(
                &mut settings.ollama.available_models,
                ollama.as_ref().and_then(|s| s.available_models.clone()),
            );

            // LM Studio
            let lmstudio = value.lmstudio.clone();

            merge(
                &mut settings.lmstudio.api_url,
                value.lmstudio.as_ref().and_then(|s| s.api_url.clone()),
            );
            merge(
                &mut settings.lmstudio.available_models,
                lmstudio.as_ref().and_then(|s| s.available_models.clone()),
            );

            // DeepSeek
            let deepseek = value.deepseek.clone();

            merge(
                &mut settings.deepseek.api_url,
                value.deepseek.as_ref().and_then(|s| s.api_url.clone()),
            );
            merge(
                &mut settings.deepseek.available_models,
                deepseek.as_ref().and_then(|s| s.available_models.clone()),
            );

            // OpenAI
            let openai = value.openai.clone();
            merge(
                &mut settings.openai.api_url,
                openai.as_ref().and_then(|s| s.api_url.clone()),
            );
            merge(
                &mut settings.openai.available_models,
                openai.as_ref().and_then(|s| s.available_models.clone()),
            );

            // OpenAI
            let openai = value.openai.clone();
            merge(
                &mut settings.openai.api_url,
                openai.as_ref().and_then(|s| s.api_url.clone()),
            );
            merge(
                &mut settings.openai.available_models,
                openai.as_ref().and_then(|s| s.available_models.clone()),
            );

            // Vercel
            let vercel = value.vercel.clone();
            merge(
                &mut settings.vercel.api_url,
                vercel.as_ref().and_then(|s| s.api_url.clone()),
            );
            merge(
                &mut settings.vercel.available_models,
                vercel.as_ref().and_then(|s| s.available_models.clone()),
            );

            merge(
                &mut settings.zed_dot_dev.available_models,
                value
                    .zed_dot_dev
                    .as_ref()
                    .and_then(|s| s.available_models.clone()),
            );
            merge(
                &mut settings.google.api_url,
                value.google.as_ref().and_then(|s| s.api_url.clone()),
            );
            merge(
                &mut settings.google.available_models,
                value
                    .google
                    .as_ref()
                    .and_then(|s| s.available_models.clone()),
            );

            // Mistral
            let mistral = value.mistral.clone();
            merge(
                &mut settings.mistral.api_url,
                mistral.as_ref().and_then(|s| s.api_url.clone()),
            );
            merge(
                &mut settings.mistral.available_models,
                mistral.as_ref().and_then(|s| s.available_models.clone()),
            );

            // OpenRouter
            let open_router = value.open_router.clone();
            merge(
                &mut settings.open_router.api_url,
                open_router.as_ref().and_then(|s| s.api_url.clone()),
            );
            merge(
                &mut settings.open_router.available_models,
                open_router
                    .as_ref()
                    .and_then(|s| s.available_models.clone()),
            );

            // Pollinations
            let (pollinations, upgraded) = match value.pollinations.clone().map(|s| s.upgrade()) {
                Some((content, upgraded)) => (Some(content), upgraded),
                None => (None, false),
            };

            if upgraded {
                settings.pollinations.needs_setting_migration = true;
            }
            merge(
                &mut settings.pollinations.api_url,
                pollinations.as_ref().and_then(|s| s.api_url.clone()),
            );
            merge(
                &mut settings.pollinations.available_models,
                pollinations
                    .as_ref()
                    .and_then(|s| s.available_models.clone()),
            );
        }

        Ok(settings)
    }

    fn import_from_vscode(_vscode: &settings::VsCodeSettings, _current: &mut Self::FileContent) {}
}
