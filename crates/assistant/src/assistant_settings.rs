use std::sync::Arc;

use anthropic::Model as AnthropicModel;
use fs::Fs;
use gpui::{AppContext, Pixels};
use language_model::{settings::AllLanguageModelSettings, CloudModel, LanguageModel};
use ollama::Model as OllamaModel;
use open_ai::Model as OpenAiModel;
use schemars::{schema::Schema, JsonSchema};
use serde::{Deserialize, Serialize};
use settings::{update_settings_file, Settings, SettingsSources};

#[derive(Copy, Clone, Default, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AssistantDockPosition {
    Left,
    #[default]
    Right,
    Bottom,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(tag = "name", rename_all = "snake_case")]
pub enum AssistantProviderContentV1 {
    #[serde(rename = "zed.dev")]
    ZedDotDev { default_model: Option<CloudModel> },
    #[serde(rename = "openai")]
    OpenAi {
        default_model: Option<OpenAiModel>,
        api_url: Option<String>,
        low_speed_timeout_in_seconds: Option<u64>,
        available_models: Option<Vec<OpenAiModel>>,
    },
    #[serde(rename = "anthropic")]
    Anthropic {
        default_model: Option<AnthropicModel>,
        api_url: Option<String>,
        low_speed_timeout_in_seconds: Option<u64>,
    },
    #[serde(rename = "ollama")]
    Ollama {
        default_model: Option<OllamaModel>,
        api_url: Option<String>,
        low_speed_timeout_in_seconds: Option<u64>,
    },
}

#[derive(Debug, Default)]
pub struct AssistantSettings {
    pub enabled: bool,
    pub button: bool,
    pub dock: AssistantDockPosition,
    pub default_width: Pixels,
    pub default_height: Pixels,
    pub default_model: LanguageModelSelection,
    pub using_outdated_settings_version: bool,
}

/// Assistant panel settings
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum AssistantSettingsContent {
    Versioned(VersionedAssistantSettingsContent),
    Legacy(LegacyAssistantSettingsContent),
}

impl JsonSchema for AssistantSettingsContent {
    fn schema_name() -> String {
        VersionedAssistantSettingsContent::schema_name()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> Schema {
        VersionedAssistantSettingsContent::json_schema(gen)
    }

    fn is_referenceable() -> bool {
        VersionedAssistantSettingsContent::is_referenceable()
    }
}

impl Default for AssistantSettingsContent {
    fn default() -> Self {
        Self::Versioned(VersionedAssistantSettingsContent::default())
    }
}

impl AssistantSettingsContent {
    pub fn is_version_outdated(&self) -> bool {
        match self {
            AssistantSettingsContent::Versioned(settings) => match settings {
                VersionedAssistantSettingsContent::V1(_) => true,
                VersionedAssistantSettingsContent::V2(_) => false,
            },
            AssistantSettingsContent::Legacy(_) => true,
        }
    }

    pub fn update_file(&mut self, fs: Arc<dyn Fs>, cx: &AppContext) {
        if let AssistantSettingsContent::Versioned(settings) = self {
            if let VersionedAssistantSettingsContent::V1(settings) = settings {
                if let Some(provider) = settings.provider.clone() {
                    match provider {
                        AssistantProviderContentV1::Anthropic {
                            api_url,
                            low_speed_timeout_in_seconds,
                            ..
                        } => update_settings_file::<AllLanguageModelSettings>(
                            fs,
                            cx,
                            move |content, _| {
                                if content.anthropic.is_none() {
                                    content.anthropic =
                                        Some(language_model::settings::AnthropicSettingsContent::Versioned(
                                            language_model::settings::VersionedAnthropicSettingsContent::V1(
                                                language_model::settings::AnthropicSettingsContentV1 {
                                                    api_url,
                                                    low_speed_timeout_in_seconds,
                                                    available_models: None
                                                }
                                            )
                                        ));
                                }
                            },
                        ),
                        AssistantProviderContentV1::Ollama {
                            api_url,
                            low_speed_timeout_in_seconds,
                            ..
                        } => update_settings_file::<AllLanguageModelSettings>(
                            fs,
                            cx,
                            move |content, _| {
                                if content.ollama.is_none() {
                                    content.ollama =
                                        Some(language_model::settings::OllamaSettingsContent {
                                            api_url,
                                            low_speed_timeout_in_seconds,
                                        });
                                }
                            },
                        ),
                        AssistantProviderContentV1::OpenAi {
                            api_url,
                            low_speed_timeout_in_seconds,
                            available_models,
                            ..
                        } => update_settings_file::<AllLanguageModelSettings>(
                            fs,
                            cx,
                            move |content, _| {
                                if content.openai.is_none() {
                                    let available_models = available_models.map(|models| {
                                        models
                                            .into_iter()
                                            .filter_map(|model| match model {
                                                open_ai::Model::Custom { name, max_tokens } => {
                                                    Some(language_model::provider::open_ai::AvailableModel { name, max_tokens })
                                                }
                                                _ => None,
                                            })
                                            .collect::<Vec<_>>()
                                    });
                                    content.openai =
                                        Some(language_model::settings::OpenAiSettingsContent::Versioned(
                                            language_model::settings::VersionedOpenAiSettingsContent::V1(
                                                language_model::settings::OpenAiSettingsContentV1 {
                                                    api_url,
                                                    low_speed_timeout_in_seconds,
                                                    available_models
                                                }
                                            )
                                        ));
                                }
                            },
                        ),
                        _ => {}
                    }
                }
            }
        }

        *self = AssistantSettingsContent::Versioned(VersionedAssistantSettingsContent::V2(
            self.upgrade(),
        ));
    }

    fn upgrade(&self) -> AssistantSettingsContentV2 {
        match self {
            AssistantSettingsContent::Versioned(settings) => match settings {
                VersionedAssistantSettingsContent::V1(settings) => AssistantSettingsContentV2 {
                    enabled: settings.enabled,
                    button: settings.button,
                    dock: settings.dock,
                    default_width: settings.default_width,
                    default_height: settings.default_width,
                    default_model: settings
                        .provider
                        .clone()
                        .and_then(|provider| match provider {
                            AssistantProviderContentV1::ZedDotDev { default_model } => {
                                default_model.map(|model| LanguageModelSelection {
                                    provider: "zed.dev".to_string(),
                                    model: model.id().to_string(),
                                })
                            }
                            AssistantProviderContentV1::OpenAi { default_model, .. } => {
                                default_model.map(|model| LanguageModelSelection {
                                    provider: "openai".to_string(),
                                    model: model.id().to_string(),
                                })
                            }
                            AssistantProviderContentV1::Anthropic { default_model, .. } => {
                                default_model.map(|model| LanguageModelSelection {
                                    provider: "anthropic".to_string(),
                                    model: model.id().to_string(),
                                })
                            }
                            AssistantProviderContentV1::Ollama { default_model, .. } => {
                                default_model.map(|model| LanguageModelSelection {
                                    provider: "ollama".to_string(),
                                    model: model.id().to_string(),
                                })
                            }
                        }),
                },
                VersionedAssistantSettingsContent::V2(settings) => settings.clone(),
            },
            AssistantSettingsContent::Legacy(settings) => AssistantSettingsContentV2 {
                enabled: None,
                button: settings.button,
                dock: settings.dock,
                default_width: settings.default_width,
                default_height: settings.default_height,
                default_model: Some(LanguageModelSelection {
                    provider: "openai".to_string(),
                    model: settings
                        .default_open_ai_model
                        .clone()
                        .unwrap_or_default()
                        .id()
                        .to_string(),
                }),
            },
        }
    }

    pub fn set_dock(&mut self, dock: AssistantDockPosition) {
        match self {
            AssistantSettingsContent::Versioned(settings) => match settings {
                VersionedAssistantSettingsContent::V1(settings) => {
                    settings.dock = Some(dock);
                }
                VersionedAssistantSettingsContent::V2(settings) => {
                    settings.dock = Some(dock);
                }
            },
            AssistantSettingsContent::Legacy(settings) => {
                settings.dock = Some(dock);
            }
        }
    }

    pub fn set_model(&mut self, language_model: Arc<dyn LanguageModel>) {
        let model = language_model.id().0.to_string();
        let provider = language_model.provider_id().0.to_string();

        match self {
            AssistantSettingsContent::Versioned(settings) => match settings {
                VersionedAssistantSettingsContent::V1(settings) => match provider.as_ref() {
                    "zed.dev" => {
                        log::warn!("attempted to set zed.dev model on outdated settings");
                    }
                    "anthropic" => {
                        let (api_url, low_speed_timeout_in_seconds) = match &settings.provider {
                            Some(AssistantProviderContentV1::Anthropic {
                                api_url,
                                low_speed_timeout_in_seconds,
                                ..
                            }) => (api_url.clone(), *low_speed_timeout_in_seconds),
                            _ => (None, None),
                        };
                        settings.provider = Some(AssistantProviderContentV1::Anthropic {
                            default_model: AnthropicModel::from_id(&model).ok(),
                            api_url,
                            low_speed_timeout_in_seconds,
                        });
                    }
                    "ollama" => {
                        let (api_url, low_speed_timeout_in_seconds) = match &settings.provider {
                            Some(AssistantProviderContentV1::Ollama {
                                api_url,
                                low_speed_timeout_in_seconds,
                                ..
                            }) => (api_url.clone(), *low_speed_timeout_in_seconds),
                            _ => (None, None),
                        };
                        settings.provider = Some(AssistantProviderContentV1::Ollama {
                            default_model: Some(ollama::Model::new(&model)),
                            api_url,
                            low_speed_timeout_in_seconds,
                        });
                    }
                    "openai" => {
                        let (api_url, low_speed_timeout_in_seconds, available_models) =
                            match &settings.provider {
                                Some(AssistantProviderContentV1::OpenAi {
                                    api_url,
                                    low_speed_timeout_in_seconds,
                                    available_models,
                                    ..
                                }) => (
                                    api_url.clone(),
                                    *low_speed_timeout_in_seconds,
                                    available_models.clone(),
                                ),
                                _ => (None, None, None),
                            };
                        settings.provider = Some(AssistantProviderContentV1::OpenAi {
                            default_model: open_ai::Model::from_id(&model).ok(),
                            api_url,
                            low_speed_timeout_in_seconds,
                            available_models,
                        });
                    }
                    _ => {}
                },
                VersionedAssistantSettingsContent::V2(settings) => {
                    settings.default_model = Some(LanguageModelSelection { provider, model });
                }
            },
            AssistantSettingsContent::Legacy(settings) => {
                if let Ok(model) = open_ai::Model::from_id(&language_model.id().0) {
                    settings.default_open_ai_model = Some(model);
                }
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize, JsonSchema, Debug)]
#[serde(tag = "version")]
pub enum VersionedAssistantSettingsContent {
    #[serde(rename = "1")]
    V1(AssistantSettingsContentV1),
    #[serde(rename = "2")]
    V2(AssistantSettingsContentV2),
}

impl Default for VersionedAssistantSettingsContent {
    fn default() -> Self {
        Self::V2(AssistantSettingsContentV2 {
            enabled: None,
            button: None,
            dock: None,
            default_width: None,
            default_height: None,
            default_model: None,
        })
    }
}

#[derive(Clone, Serialize, Deserialize, JsonSchema, Debug)]
pub struct AssistantSettingsContentV2 {
    /// Whether the Assistant is enabled.
    ///
    /// Default: true
    enabled: Option<bool>,
    /// Whether to show the assistant panel button in the status bar.
    ///
    /// Default: true
    button: Option<bool>,
    /// Where to dock the assistant.
    ///
    /// Default: right
    dock: Option<AssistantDockPosition>,
    /// Default width in pixels when the assistant is docked to the left or right.
    ///
    /// Default: 640
    default_width: Option<f32>,
    /// Default height in pixels when the assistant is docked to the bottom.
    ///
    /// Default: 320
    default_height: Option<f32>,
    /// The default model to use when creating new contexts.
    default_model: Option<LanguageModelSelection>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct LanguageModelSelection {
    #[schemars(schema_with = "providers_schema")]
    pub provider: String,
    pub model: String,
}

fn providers_schema(_: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
    schemars::schema::SchemaObject {
        enum_values: Some(vec![
            "anthropic".into(),
            "google".into(),
            "ollama".into(),
            "openai".into(),
            "zed.dev".into(),
            "copilot_chat".into(),
        ]),
        ..Default::default()
    }
    .into()
}

impl Default for LanguageModelSelection {
    fn default() -> Self {
        Self {
            provider: "openai".to_string(),
            model: "gpt-4".to_string(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, JsonSchema, Debug)]
pub struct AssistantSettingsContentV1 {
    /// Whether the Assistant is enabled.
    ///
    /// Default: true
    enabled: Option<bool>,
    /// Whether to show the assistant panel button in the status bar.
    ///
    /// Default: true
    button: Option<bool>,
    /// Where to dock the assistant.
    ///
    /// Default: right
    dock: Option<AssistantDockPosition>,
    /// Default width in pixels when the assistant is docked to the left or right.
    ///
    /// Default: 640
    default_width: Option<f32>,
    /// Default height in pixels when the assistant is docked to the bottom.
    ///
    /// Default: 320
    default_height: Option<f32>,
    /// The provider of the assistant service.
    ///
    /// This can be "openai", "anthropic", "ollama", "zed.dev"
    /// each with their respective default models and configurations.
    provider: Option<AssistantProviderContentV1>,
}

#[derive(Clone, Serialize, Deserialize, JsonSchema, Debug)]
pub struct LegacyAssistantSettingsContent {
    /// Whether to show the assistant panel button in the status bar.
    ///
    /// Default: true
    pub button: Option<bool>,
    /// Where to dock the assistant.
    ///
    /// Default: right
    pub dock: Option<AssistantDockPosition>,
    /// Default width in pixels when the assistant is docked to the left or right.
    ///
    /// Default: 640
    pub default_width: Option<f32>,
    /// Default height in pixels when the assistant is docked to the bottom.
    ///
    /// Default: 320
    pub default_height: Option<f32>,
    /// The default OpenAI model to use when creating new contexts.
    ///
    /// Default: gpt-4-1106-preview
    pub default_open_ai_model: Option<OpenAiModel>,
    /// OpenAI API base URL to use when creating new contexts.
    ///
    /// Default: https://api.openai.com/v1
    pub openai_api_url: Option<String>,
}

impl Settings for AssistantSettings {
    const KEY: Option<&'static str> = Some("assistant");

    const PRESERVED_KEYS: Option<&'static [&'static str]> = Some(&["version"]);

    type FileContent = AssistantSettingsContent;

    fn load(
        sources: SettingsSources<Self::FileContent>,
        _: &mut gpui::AppContext,
    ) -> anyhow::Result<Self> {
        let mut settings = AssistantSettings::default();

        for value in sources.defaults_and_customizations() {
            if value.is_version_outdated() {
                settings.using_outdated_settings_version = true;
            }

            let value = value.upgrade();
            merge(&mut settings.enabled, value.enabled);
            merge(&mut settings.button, value.button);
            merge(&mut settings.dock, value.dock);
            merge(
                &mut settings.default_width,
                value.default_width.map(Into::into),
            );
            merge(
                &mut settings.default_height,
                value.default_height.map(Into::into),
            );
            merge(
                &mut settings.default_model,
                value.default_model.map(Into::into),
            );
        }

        Ok(settings)
    }
}

fn merge<T>(target: &mut T, value: Option<T>) {
    if let Some(value) = value {
        *target = value;
    }
}

#[cfg(test)]
mod tests {
    use gpui::{ReadGlobal, TestAppContext};

    use super::*;

    #[gpui::test]
    async fn test_deserialize_assistant_settings_with_version(cx: &mut TestAppContext) {
        let fs = fs::FakeFs::new(cx.executor().clone());
        fs.create_dir(paths::settings_file().parent().unwrap())
            .await
            .unwrap();

        cx.update(|cx| {
            let test_settings = settings::SettingsStore::test(cx);
            cx.set_global(test_settings);
            AssistantSettings::register(cx);
        });

        cx.update(|cx| {
            assert!(!AssistantSettings::get_global(cx).using_outdated_settings_version);
            assert_eq!(
                AssistantSettings::get_global(cx).default_model,
                LanguageModelSelection {
                    provider: "zed.dev".into(),
                    model: "claude-3-5-sonnet".into(),
                }
            );
        });

        cx.update(|cx| {
            settings::SettingsStore::global(cx).update_settings_file::<AssistantSettings>(
                fs.clone(),
                |settings, _| {
                    *settings = AssistantSettingsContent::Versioned(
                        VersionedAssistantSettingsContent::V2(AssistantSettingsContentV2 {
                            default_model: Some(LanguageModelSelection {
                                provider: "test-provider".into(),
                                model: "gpt-99".into(),
                            }),
                            enabled: None,
                            button: None,
                            dock: None,
                            default_width: None,
                            default_height: None,
                        }),
                    )
                },
            );
        });

        cx.run_until_parked();

        let raw_settings_value = fs.load(paths::settings_file()).await.unwrap();
        assert!(raw_settings_value.contains(r#""version": "2""#));

        #[derive(Debug, Deserialize)]
        struct AssistantSettingsTest {
            assistant: AssistantSettingsContent,
        }

        let assistant_settings: AssistantSettingsTest =
            serde_json_lenient::from_str(&raw_settings_value).unwrap();

        assert!(!assistant_settings.assistant.is_version_outdated());
    }
}
