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
    #[serde(default)]
    pub language_models: Vec<LanguageModelSettingsContent>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(tag = "name", rename_all = "snake_case")]
pub enum LanguageModelSettingsContent {
    #[serde(rename = "open_ai")]
    OpenAi {
        api_url: Option<String>,
        low_speed_timeout_in_seconds: Option<u64>,
        available_models: Option<Vec<open_ai::Model>>,
    },
    #[serde(rename = "anthropic")]
    Anthropic {
        api_url: Option<String>,
        low_speed_timeout_in_seconds: Option<u64>,
    },
    #[serde(rename = "ollama")]
    Ollama {
        api_url: Option<String>,
        low_speed_timeout_in_seconds: Option<u64>,
    },
}

impl settings::Settings for AllLanguageModelSettings {
    const KEY: Option<&'static str> = None;

    type FileContent = AllLanguageModelSettingsContent;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut AppContext) -> Result<Self> {
        fn merge<T>(target: &mut T, value: Option<T>) {
            if let Some(value) = value {
                *target = value;
            }
        }

        let mut settings = AllLanguageModelSettings::default();

        for value in sources.defaults_and_customizations() {
            for setting in value.language_models.clone() {
                match setting {
                    LanguageModelSettingsContent::OpenAi {
                        api_url,
                        low_speed_timeout_in_seconds,
                        available_models,
                    } => {
                        merge(&mut settings.open_ai.api_url, api_url);
                        if let Some(low_speed_timeout_in_seconds) = low_speed_timeout_in_seconds {
                            settings.open_ai.low_speed_timeout =
                                Some(Duration::from_secs(low_speed_timeout_in_seconds));
                        }
                        merge(&mut settings.open_ai.available_models, available_models);
                    }
                    LanguageModelSettingsContent::Anthropic {
                        api_url,
                        low_speed_timeout_in_seconds,
                    } => {
                        merge(&mut settings.anthropic.api_url, api_url);
                        if let Some(low_speed_timeout_in_seconds) = low_speed_timeout_in_seconds {
                            settings.anthropic.low_speed_timeout =
                                Some(Duration::from_secs(low_speed_timeout_in_seconds));
                        }
                    }
                    LanguageModelSettingsContent::Ollama {
                        api_url,
                        low_speed_timeout_in_seconds,
                    } => {
                        merge(&mut settings.ollama.api_url, api_url);
                        if let Some(low_speed_timeout_in_seconds) = low_speed_timeout_in_seconds {
                            settings.ollama.low_speed_timeout =
                                Some(Duration::from_secs(low_speed_timeout_in_seconds));
                        }
                    }
                }
            }
        }

        Ok(settings)
    }
}
