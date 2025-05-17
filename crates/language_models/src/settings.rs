use std::sync::Arc;

use anyhow::Result;
use gpui::App;
use language_model::LanguageModelCacheConfiguration;
use project::Fs;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources, update_settings_file};

use crate::provider::{
    self,
    anthropic::AnthropicSettings,
    bedrock::AmazonBedrockSettings,
    cloud::{self, ZedDotDevSettings},
    copilot_chat::CopilotChatSettings,
    deepseek::DeepSeekSettings,
    google::GoogleSettings,
    lmstudio::LmStudioSettings,
    mistral::MistralSettings,
    ollama::OllamaSettings,
    open_ai::OpenAiSettings,
};

/// Initializes the language model settings.
pub fn init(fs: Arc<dyn Fs>, cx: &mut App) {
    AllLanguageModelSettings::register(cx);

    if AllLanguageModelSettings::get_global(cx)
        .openai
        .needs_setting_migration
    {
        update_settings_file::<AllLanguageModelSettings>(fs.clone(), cx, move |setting, _| {
            if let Some(settings) = setting.openai.clone() {
                let (newest_version, _) = settings.upgrade();
                setting.openai = Some(OpenAiSettingsContent::Versioned(
                    VersionedOpenAiSettingsContent::V1(newest_version),
                ));
            }
        });
    }

    if AllLanguageModelSettings::get_global(cx)
        .anthropic
        .needs_setting_migration
    {
        update_settings_file::<AllLanguageModelSettings>(fs, cx, move |setting, _| {
            if let Some(settings) = setting.anthropic.clone() {
                let (newest_version, _) = settings.upgrade();
                setting.anthropic = Some(AnthropicSettingsContent::Versioned(
                    VersionedAnthropicSettingsContent::V1(newest_version),
                ));
            }
        });
    }
}

#[derive(Default)]
pub struct AllLanguageModelSettings {
    pub anthropic: AnthropicSettings,
    pub bedrock: AmazonBedrockSettings,
    pub ollama: OllamaSettings,
    pub openai: OpenAiSettings,
    pub zed_dot_dev: ZedDotDevSettings,
    pub google: GoogleSettings,
    pub copilot_chat: CopilotChatSettings,
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
    #[serde(rename = "zed.dev")]
    pub zed_dot_dev: Option<ZedDotDevSettingsContent>,
    pub google: Option<GoogleSettingsContent>,
    pub deepseek: Option<DeepseekSettingsContent>,
    pub copilot_chat: Option<CopilotChatSettingsContent>,
    pub mistral: Option<MistralSettingsContent>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
#[serde(untagged)]
pub enum AnthropicSettingsContent {
    Versioned(VersionedAnthropicSettingsContent),
    Legacy(LegacyAnthropicSettingsContent),
}

impl AnthropicSettingsContent {
    pub fn upgrade(self) -> (AnthropicSettingsContentV1, bool) {
        match self {
            AnthropicSettingsContent::Legacy(content) => (
                AnthropicSettingsContentV1 {
                    api_url: content.api_url,
                    available_models: content.available_models.map(|models| {
                        models
                            .into_iter()
                            .filter_map(|model| match model {
                                anthropic::Model::Custom {
                                    name,
                                    display_name,
                                    max_tokens,
                                    tool_override,
                                    cache_configuration,
                                    max_output_tokens,
                                    default_temperature,
                                    extra_beta_headers,
                                    mode,
                                } => Some(provider::anthropic::AvailableModel {
                                    name,
                                    display_name,
                                    max_tokens,
                                    tool_override,
                                    cache_configuration: cache_configuration.as_ref().map(
                                        |config| LanguageModelCacheConfiguration {
                                            max_cache_anchors: config.max_cache_anchors,
                                            should_speculate: config.should_speculate,
                                            min_total_token: config.min_total_token,
                                        },
                                    ),
                                    max_output_tokens,
                                    default_temperature,
                                    extra_beta_headers,
                                    mode: Some(mode.into()),
                                }),
                                _ => None,
                            })
                            .collect()
                    }),
                },
                true,
            ),
            AnthropicSettingsContent::Versioned(content) => match content {
                VersionedAnthropicSettingsContent::V1(content) => (content, false),
            },
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct LegacyAnthropicSettingsContent {
    pub api_url: Option<String>,
    pub available_models: Option<Vec<anthropic::Model>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
#[serde(tag = "version")]
pub enum VersionedAnthropicSettingsContent {
    #[serde(rename = "1")]
    V1(AnthropicSettingsContentV1),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct AnthropicSettingsContentV1 {
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
    pub active_server: Option<String>,
    pub servers: Option<Vec<provider::lmstudio::LmStudioServer>>,
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
#[serde(untagged)]
pub enum OpenAiSettingsContent {
    Versioned(VersionedOpenAiSettingsContent),
    Legacy(LegacyOpenAiSettingsContent),
}

impl OpenAiSettingsContent {
    pub fn upgrade(self) -> (OpenAiSettingsContentV1, bool) {
        match self {
            OpenAiSettingsContent::Legacy(content) => (
                OpenAiSettingsContentV1 {
                    api_url: content.api_url,
                    available_models: content.available_models.map(|models| {
                        models
                            .into_iter()
                            .filter_map(|model| match model {
                                open_ai::Model::Custom {
                                    name,
                                    display_name,
                                    max_tokens,
                                    max_output_tokens,
                                    max_completion_tokens,
                                } => Some(provider::open_ai::AvailableModel {
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
            OpenAiSettingsContent::Versioned(content) => match content {
                VersionedOpenAiSettingsContent::V1(content) => (content, false),
            },
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct LegacyOpenAiSettingsContent {
    pub api_url: Option<String>,
    pub available_models: Option<Vec<open_ai::Model>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
#[serde(tag = "version")]
pub enum VersionedOpenAiSettingsContent {
    #[serde(rename = "1")]
    V1(OpenAiSettingsContentV1),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct OpenAiSettingsContentV1 {
    pub api_url: Option<String>,
    pub available_models: Option<Vec<provider::open_ai::AvailableModel>>,
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
pub struct CopilotChatSettingsContent {}

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
            let (anthropic, upgraded) = match value.anthropic.clone().map(|s| s.upgrade()) {
                Some((content, upgraded)) => (Some(content), upgraded),
                None => (None, false),
            };

            if upgraded {
                settings.anthropic.needs_setting_migration = true;
            }

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

            // Handle migration from old structure to new structure
            if let Some(api_url) = lmstudio.as_ref().and_then(|s| s.api_url.clone()) {
                if !api_url.is_empty() && settings.lmstudio.servers.is_empty() {
                    // Migrate from old api_url to new server structure
                    settings.lmstudio = provider::lmstudio::LmStudioSettings::migrate_from_legacy(&api_url);
                }
            }

            // Update the servers list from settings
            if let Some(servers) = lmstudio.as_ref().and_then(|s| s.servers.clone()) {
                settings.lmstudio.servers = servers;
            }

            // Handle legacy model list and migrate to server-specific models if needed
            if let Some(models) = lmstudio.as_ref().and_then(|s| s.available_models.clone()) {
                // If we have servers and legacy models, add models to first server
                if !settings.lmstudio.servers.is_empty() {
                    // Get first server and ensure it has an available_models collection
                    let first_server = &mut settings.lmstudio.servers[0];
                    if first_server.available_models.is_none() {
                        first_server.available_models = Some(Vec::new());
                    }
                    
                    // Only add models that don't already exist
                    if let Some(server_models) = &mut first_server.available_models {
                        for model in models {
                            if !server_models.iter().any(|m| m.name == model.name) {
                                server_models.push(model);
                            }
                        }
                    }
                }
            }

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
            let (openai, upgraded) = match value.openai.clone().map(|s| s.upgrade()) {
                Some((content, upgraded)) => (Some(content), upgraded),
                None => (None, false),
            };

            if upgraded {
                settings.openai.needs_setting_migration = true;
            }

            merge(
                &mut settings.openai.api_url,
                openai.as_ref().and_then(|s| s.api_url.clone()),
            );
            merge(
                &mut settings.openai.available_models,
                openai.as_ref().and_then(|s| s.available_models.clone()),
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
        }

        Ok(settings)
    }

    fn import_from_vscode(_vscode: &settings::VsCodeSettings, _current: &mut Self::FileContent) {}
}
