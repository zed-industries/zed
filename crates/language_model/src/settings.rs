use std::time::Duration;

use anyhow::Result;
use gpui::AppContext;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};

use crate::provider::{
    anthropic::AnthropicSettings, ollama::OllamaSettings, open_ai::OpenAiSettings,
};

/// Initializes the language model settings.
pub fn init(cx: &mut AppContext) {
    AllLanguageModelSettings::register(cx);
}

#[derive(Default)]
pub struct AllLanguageModelSettings {
    pub open_ai: OpenAiSettings,
    pub anthropic: AnthropicSettings,
    pub ollama: OllamaSettings,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct AllLanguageModelSettingsContent {
    anthropic: Option<AnthropicSettingsContent>,
    ollama: Option<OllamaSettingsContent>,
    open_ai: Option<OpenAiSettingsContent>,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct AnthropicSettingsContent {
    api_url: Option<String>,
    low_speed_timeout_in_seconds: Option<u64>,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct OllamaSettingsContent {
    api_url: Option<String>,
    low_speed_timeout_in_seconds: Option<u64>,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq, JsonSchema)]
pub struct OpenAiSettingsContent {
    api_url: Option<String>,
    low_speed_timeout_in_seconds: Option<u64>,
    available_models: Option<Vec<open_ai::Model>>,
}

impl settings::Settings for AllLanguageModelSettings {
    const KEY: Option<&'static str> = Some("language_models");

    type FileContent = AllLanguageModelSettingsContent;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut AppContext) -> Result<Self> {
        fn merge<T>(target: &mut T, value: Option<T>) {
            if let Some(value) = value {
                *target = value;
            }
        }

        let mut settings = AllLanguageModelSettings::default();

        for value in sources.defaults_and_customizations() {
            merge(
                &mut settings.anthropic.api_url,
                value.anthropic.as_ref().and_then(|s| s.api_url.clone()),
            );
            if let Some(low_speed_timeout_in_seconds) = value
                .anthropic
                .as_ref()
                .and_then(|s| s.low_speed_timeout_in_seconds.clone())
            {
                settings.anthropic.low_speed_timeout =
                    Some(Duration::from_secs(low_speed_timeout_in_seconds));
            }

            merge(
                &mut settings.ollama.api_url,
                value.ollama.as_ref().and_then(|s| s.api_url.clone()),
            );
            if let Some(low_speed_timeout_in_seconds) = value
                .ollama
                .as_ref()
                .and_then(|s| s.low_speed_timeout_in_seconds.clone())
            {
                settings.ollama.low_speed_timeout =
                    Some(Duration::from_secs(low_speed_timeout_in_seconds));
            }

            merge(
                &mut settings.open_ai.api_url,
                value.open_ai.as_ref().and_then(|s| s.api_url.clone()),
            );
            if let Some(low_speed_timeout_in_seconds) = value
                .open_ai
                .as_ref()
                .and_then(|s| s.low_speed_timeout_in_seconds.clone())
            {
                settings.open_ai.low_speed_timeout =
                    Some(Duration::from_secs(low_speed_timeout_in_seconds));
            }
            merge(
                &mut settings.open_ai.available_models,
                value
                    .open_ai
                    .as_ref()
                    .and_then(|s| s.available_models.clone()),
            );
        }

        Ok(settings)
    }
}
