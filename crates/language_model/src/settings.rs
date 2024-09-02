use std::{sync::Arc, time::Duration};

use anyhow::Result;
use gpui::AppContext;
use project::Fs;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{update_settings_file, Settings, SettingsSources};

use crate::{
    provider::{
        self,
        anthropic::AnthropicSettings,
        cloud::{self, ZedDotDevSettings},
        copilot_chat::CopilotChatSettings,
        google::GoogleSettings,
        ollama::OllamaSettings,
        open_ai::OpenAiSettings,
    },
    LanguageModelCacheConfiguration,
};

/// Initializes the language model settings.
pub fn init(fs: Arc<dyn Fs>, cx: &mut AppContext) {
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
    pub ollama: OllamaSettings,
    pub openai: OpenAiSettings,
    pub zed_dot_dev: ZedDotDevSettings,
    pub google: GoogleSettings,
    pub copilot_chat: CopilotChatSettings,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct AllLanguageModelSettingsContent {
    pub anthropic: Option<AnthropicSettingsContent>,
    pub ollama: Option<OllamaSettingsContent>,
    pub openai: Option<OpenAiSettingsContent>,
    #[serde(rename = "zed.dev")]
    pub zed_dot_dev: Option<ZedDotDevSettingsContent>,
    pub google: Option<GoogleSettingsContent>,
    pub copilot_chat: Option<CopilotChatSettingsContent>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
#[serde(untagged)]
pub enum AnthropicSettingsContent {
    Legacy(LegacyAnthropicSettingsContent),
    Versioned(VersionedAnthropicSettingsContent),
}

impl AnthropicSettingsContent {
    pub fn upgrade(self) -> (AnthropicSettingsContentV1, bool) {
        match self {
            AnthropicSettingsContent::Legacy(content) => (
                AnthropicSettingsContentV1 {
                    api_url: content.api_url,
                    low_speed_timeout_in_seconds: content.low_speed_timeout_in_seconds,
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
    pub low_speed_timeout_in_seconds: Option<u64>,
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
    pub low_speed_timeout_in_seconds: Option<u64>,
    pub available_models: Option<Vec<provider::anthropic::AvailableModel>>,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct OllamaSettingsContent {
    pub api_url: Option<String>,
    pub low_speed_timeout_in_seconds: Option<u64>,
    pub available_models: Option<Vec<provider::ollama::AvailableModel>>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
#[serde(untagged)]
pub enum OpenAiSettingsContent {
    Legacy(LegacyOpenAiSettingsContent),
    Versioned(VersionedOpenAiSettingsContent),
}

impl OpenAiSettingsContent {
    pub fn upgrade(self) -> (OpenAiSettingsContentV1, bool) {
        match self {
            OpenAiSettingsContent::Legacy(content) => (
                OpenAiSettingsContentV1 {
                    api_url: content.api_url,
                    low_speed_timeout_in_seconds: content.low_speed_timeout_in_seconds,
                    available_models: content.available_models.map(|models| {
                        models
                            .into_iter()
                            .filter_map(|model| match model {
                                open_ai::Model::Custom {
                                    name,
                                    max_tokens,
                                    max_output_tokens,
                                } => Some(provider::open_ai::AvailableModel {
                                    name,
                                    max_tokens,
                                    max_output_tokens,
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
    pub low_speed_timeout_in_seconds: Option<u64>,
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
    pub low_speed_timeout_in_seconds: Option<u64>,
    pub available_models: Option<Vec<provider::open_ai::AvailableModel>>,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct GoogleSettingsContent {
    pub api_url: Option<String>,
    pub low_speed_timeout_in_seconds: Option<u64>,
    pub available_models: Option<Vec<provider::google::AvailableModel>>,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct ZedDotDevSettingsContent {
    available_models: Option<Vec<cloud::AvailableModel>>,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct CopilotChatSettingsContent {
    low_speed_timeout_in_seconds: Option<u64>,
}

impl settings::Settings for AllLanguageModelSettings {
    const KEY: Option<&'static str> = Some("language_models");

    const PRESERVED_KEYS: Option<&'static [&'static str]> = Some(&["version"]);

    type FileContent = AllLanguageModelSettingsContent;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut AppContext) -> Result<Self> {
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
            if let Some(low_speed_timeout_in_seconds) = anthropic
                .as_ref()
                .and_then(|s| s.low_speed_timeout_in_seconds)
            {
                settings.anthropic.low_speed_timeout =
                    Some(Duration::from_secs(low_speed_timeout_in_seconds));
            }
            merge(
                &mut settings.anthropic.available_models,
                anthropic.as_ref().and_then(|s| s.available_models.clone()),
            );

            // Ollama
            let ollama = value.ollama.clone();

            merge(
                &mut settings.ollama.api_url,
                value.ollama.as_ref().and_then(|s| s.api_url.clone()),
            );
            if let Some(low_speed_timeout_in_seconds) = value
                .ollama
                .as_ref()
                .and_then(|s| s.low_speed_timeout_in_seconds)
            {
                settings.ollama.low_speed_timeout =
                    Some(Duration::from_secs(low_speed_timeout_in_seconds));
            }
            merge(
                &mut settings.ollama.available_models,
                ollama.as_ref().and_then(|s| s.available_models.clone()),
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
            if let Some(low_speed_timeout_in_seconds) =
                openai.as_ref().and_then(|s| s.low_speed_timeout_in_seconds)
            {
                settings.openai.low_speed_timeout =
                    Some(Duration::from_secs(low_speed_timeout_in_seconds));
            }
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
            if let Some(low_speed_timeout_in_seconds) = value
                .google
                .as_ref()
                .and_then(|s| s.low_speed_timeout_in_seconds)
            {
                settings.google.low_speed_timeout =
                    Some(Duration::from_secs(low_speed_timeout_in_seconds));
            }
            merge(
                &mut settings.google.available_models,
                value
                    .google
                    .as_ref()
                    .and_then(|s| s.available_models.clone()),
            );

            if let Some(low_speed_timeout) = value
                .copilot_chat
                .as_ref()
                .and_then(|s| s.low_speed_timeout_in_seconds)
            {
                settings.copilot_chat.low_speed_timeout =
                    Some(Duration::from_secs(low_speed_timeout));
            }
        }

        Ok(settings)
    }
}
