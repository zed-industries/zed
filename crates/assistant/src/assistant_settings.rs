use std::{sync::Arc, time::Duration};

use anthropic::Model as AnthropicModel;
use client::Client;
use completion::{
    AnthropicCompletionProvider, CloudCompletionProvider, CompletionProvider,
    LanguageModelCompletionProvider, OllamaCompletionProvider, OpenAiCompletionProvider,
};
use gpui::{AppContext, Pixels};
use language_model::{CloudModel, LanguageModel};
use ollama::Model as OllamaModel;
use open_ai::Model as OpenAiModel;
use parking_lot::RwLock;
use schemars::{schema::Schema, JsonSchema};
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};

#[derive(Copy, Clone, Default, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AssistantDockPosition {
    Left,
    #[default]
    Right,
    Bottom,
}

#[derive(Debug, PartialEq)]
pub enum AssistantProvider {
    ZedDotDev {
        model: CloudModel,
    },
    OpenAi {
        model: OpenAiModel,
        api_url: String,
        low_speed_timeout_in_seconds: Option<u64>,
        available_models: Vec<OpenAiModel>,
    },
    Anthropic {
        model: AnthropicModel,
        api_url: String,
        low_speed_timeout_in_seconds: Option<u64>,
    },
    Ollama {
        model: OllamaModel,
        api_url: String,
        low_speed_timeout_in_seconds: Option<u64>,
    },
}

impl Default for AssistantProvider {
    fn default() -> Self {
        Self::OpenAi {
            model: OpenAiModel::default(),
            api_url: open_ai::OPEN_AI_API_URL.into(),
            low_speed_timeout_in_seconds: None,
            available_models: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(tag = "name", rename_all = "snake_case")]
pub enum AssistantProviderContent {
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
    pub provider: AssistantProvider,
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
    fn upgrade(&self) -> AssistantSettingsContentV1 {
        match self {
            AssistantSettingsContent::Versioned(settings) => match settings {
                VersionedAssistantSettingsContent::V1(settings) => settings.clone(),
            },
            AssistantSettingsContent::Legacy(settings) => AssistantSettingsContentV1 {
                enabled: None,
                button: settings.button,
                dock: settings.dock,
                default_width: settings.default_width,
                default_height: settings.default_height,
                provider: if let Some(open_ai_api_url) = settings.openai_api_url.as_ref() {
                    Some(AssistantProviderContent::OpenAi {
                        default_model: settings.default_open_ai_model.clone(),
                        api_url: Some(open_ai_api_url.clone()),
                        low_speed_timeout_in_seconds: None,
                        available_models: Some(Default::default()),
                    })
                } else {
                    settings.default_open_ai_model.clone().map(|open_ai_model| {
                        AssistantProviderContent::OpenAi {
                            default_model: Some(open_ai_model),
                            api_url: None,
                            low_speed_timeout_in_seconds: None,
                            available_models: Some(Default::default()),
                        }
                    })
                },
            },
        }
    }

    pub fn set_dock(&mut self, dock: AssistantDockPosition) {
        match self {
            AssistantSettingsContent::Versioned(settings) => match settings {
                VersionedAssistantSettingsContent::V1(settings) => {
                    settings.dock = Some(dock);
                }
            },
            AssistantSettingsContent::Legacy(settings) => {
                settings.dock = Some(dock);
            }
        }
    }

    pub fn set_model(&mut self, new_model: LanguageModel) {
        match self {
            AssistantSettingsContent::Versioned(settings) => match settings {
                VersionedAssistantSettingsContent::V1(settings) => match &mut settings.provider {
                    Some(AssistantProviderContent::ZedDotDev {
                        default_model: model,
                    }) => {
                        if let LanguageModel::Cloud(new_model) = new_model {
                            *model = Some(new_model);
                        }
                    }
                    Some(AssistantProviderContent::OpenAi {
                        default_model: model,
                        ..
                    }) => {
                        if let LanguageModel::OpenAi(new_model) = new_model {
                            *model = Some(new_model);
                        }
                    }
                    Some(AssistantProviderContent::Anthropic {
                        default_model: model,
                        ..
                    }) => {
                        if let LanguageModel::Anthropic(new_model) = new_model {
                            *model = Some(new_model);
                        }
                    }
                    Some(AssistantProviderContent::Ollama {
                        default_model: model,
                        ..
                    }) => {
                        if let LanguageModel::Ollama(new_model) = new_model {
                            *model = Some(new_model);
                        }
                    }
                    provider => match new_model {
                        LanguageModel::Cloud(model) => {
                            *provider = Some(AssistantProviderContent::ZedDotDev {
                                default_model: Some(model),
                            })
                        }
                        LanguageModel::OpenAi(model) => {
                            *provider = Some(AssistantProviderContent::OpenAi {
                                default_model: Some(model),
                                api_url: None,
                                low_speed_timeout_in_seconds: None,
                                available_models: Some(Default::default()),
                            })
                        }
                        LanguageModel::Anthropic(model) => {
                            *provider = Some(AssistantProviderContent::Anthropic {
                                default_model: Some(model),
                                api_url: None,
                                low_speed_timeout_in_seconds: None,
                            })
                        }
                        LanguageModel::Ollama(model) => {
                            *provider = Some(AssistantProviderContent::Ollama {
                                default_model: Some(model),
                                api_url: None,
                                low_speed_timeout_in_seconds: None,
                            })
                        }
                    },
                },
            },
            AssistantSettingsContent::Legacy(settings) => {
                if let LanguageModel::OpenAi(model) = new_model {
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
}

impl Default for VersionedAssistantSettingsContent {
    fn default() -> Self {
        Self::V1(AssistantSettingsContentV1 {
            enabled: None,
            button: None,
            dock: None,
            default_width: None,
            default_height: None,
            provider: None,
        })
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
    /// This can either be the internal `zed.dev` service or an external `openai` service,
    /// each with their respective default models and configurations.
    provider: Option<AssistantProviderContent>,
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

    type FileContent = AssistantSettingsContent;

    fn load(
        sources: SettingsSources<Self::FileContent>,
        _: &mut gpui::AppContext,
    ) -> anyhow::Result<Self> {
        let mut settings = AssistantSettings::default();

        for value in sources.defaults_and_customizations() {
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
            if let Some(provider) = value.provider.clone() {
                match (&mut settings.provider, provider) {
                    (
                        AssistantProvider::ZedDotDev { model },
                        AssistantProviderContent::ZedDotDev {
                            default_model: model_override,
                        },
                    ) => {
                        merge(model, model_override);
                    }
                    (
                        AssistantProvider::OpenAi {
                            model,
                            api_url,
                            low_speed_timeout_in_seconds,
                            available_models,
                        },
                        AssistantProviderContent::OpenAi {
                            default_model: model_override,
                            api_url: api_url_override,
                            low_speed_timeout_in_seconds: low_speed_timeout_in_seconds_override,
                            available_models: available_models_override,
                        },
                    ) => {
                        merge(model, model_override);
                        merge(api_url, api_url_override);
                        merge(available_models, available_models_override);
                        if let Some(low_speed_timeout_in_seconds_override) =
                            low_speed_timeout_in_seconds_override
                        {
                            *low_speed_timeout_in_seconds =
                                Some(low_speed_timeout_in_seconds_override);
                        }
                    }
                    (
                        AssistantProvider::Ollama {
                            model,
                            api_url,
                            low_speed_timeout_in_seconds,
                        },
                        AssistantProviderContent::Ollama {
                            default_model: model_override,
                            api_url: api_url_override,
                            low_speed_timeout_in_seconds: low_speed_timeout_in_seconds_override,
                        },
                    ) => {
                        merge(model, model_override);
                        merge(api_url, api_url_override);
                        if let Some(low_speed_timeout_in_seconds_override) =
                            low_speed_timeout_in_seconds_override
                        {
                            *low_speed_timeout_in_seconds =
                                Some(low_speed_timeout_in_seconds_override);
                        }
                    }
                    (
                        AssistantProvider::Anthropic {
                            model,
                            api_url,
                            low_speed_timeout_in_seconds,
                        },
                        AssistantProviderContent::Anthropic {
                            default_model: model_override,
                            api_url: api_url_override,
                            low_speed_timeout_in_seconds: low_speed_timeout_in_seconds_override,
                        },
                    ) => {
                        merge(model, model_override);
                        merge(api_url, api_url_override);
                        if let Some(low_speed_timeout_in_seconds_override) =
                            low_speed_timeout_in_seconds_override
                        {
                            *low_speed_timeout_in_seconds =
                                Some(low_speed_timeout_in_seconds_override);
                        }
                    }
                    (provider, provider_override) => {
                        *provider = match provider_override {
                            AssistantProviderContent::ZedDotDev {
                                default_model: model,
                            } => AssistantProvider::ZedDotDev {
                                model: model.unwrap_or_default(),
                            },
                            AssistantProviderContent::OpenAi {
                                default_model: model,
                                api_url,
                                low_speed_timeout_in_seconds,
                                available_models,
                            } => AssistantProvider::OpenAi {
                                model: model.unwrap_or_default(),
                                api_url: api_url.unwrap_or_else(|| open_ai::OPEN_AI_API_URL.into()),
                                low_speed_timeout_in_seconds,
                                available_models: available_models.unwrap_or_default(),
                            },
                            AssistantProviderContent::Anthropic {
                                default_model: model,
                                api_url,
                                low_speed_timeout_in_seconds,
                            } => AssistantProvider::Anthropic {
                                model: model.unwrap_or_default(),
                                api_url: api_url
                                    .unwrap_or_else(|| anthropic::ANTHROPIC_API_URL.into()),
                                low_speed_timeout_in_seconds,
                            },
                            AssistantProviderContent::Ollama {
                                default_model: model,
                                api_url,
                                low_speed_timeout_in_seconds,
                            } => AssistantProvider::Ollama {
                                model: model.unwrap_or_default(),
                                api_url: api_url.unwrap_or_else(|| ollama::OLLAMA_API_URL.into()),
                                low_speed_timeout_in_seconds,
                            },
                        };
                    }
                }
            }
        }

        Ok(settings)
    }
}

fn merge<T>(target: &mut T, value: Option<T>) {
    if let Some(value) = value {
        *target = value;
    }
}

pub fn update_completion_provider_settings(
    provider: &mut CompletionProvider,
    version: usize,
    cx: &mut AppContext,
) {
    let updated = match &AssistantSettings::get_global(cx).provider {
        AssistantProvider::ZedDotDev { model } => provider
            .update_current_as::<_, CloudCompletionProvider>(|provider| {
                provider.update(model.clone(), version);
            }),
        AssistantProvider::OpenAi {
            model,
            api_url,
            low_speed_timeout_in_seconds,
            available_models,
        } => provider.update_current_as::<_, OpenAiCompletionProvider>(|provider| {
            provider.update(
                choose_openai_model(&model, &available_models),
                api_url.clone(),
                low_speed_timeout_in_seconds.map(Duration::from_secs),
                version,
            );
        }),
        AssistantProvider::Anthropic {
            model,
            api_url,
            low_speed_timeout_in_seconds,
        } => provider.update_current_as::<_, AnthropicCompletionProvider>(|provider| {
            provider.update(
                model.clone(),
                api_url.clone(),
                low_speed_timeout_in_seconds.map(Duration::from_secs),
                version,
            );
        }),
        AssistantProvider::Ollama {
            model,
            api_url,
            low_speed_timeout_in_seconds,
        } => provider.update_current_as::<_, OllamaCompletionProvider>(|provider| {
            provider.update(
                model.clone(),
                api_url.clone(),
                low_speed_timeout_in_seconds.map(Duration::from_secs),
                version,
                cx,
            );
        }),
    };

    // Previously configured provider was changed to another one
    if updated.is_none() {
        provider.update_provider(|client| create_provider_from_settings(client, version, cx));
    }
}

pub(crate) fn create_provider_from_settings(
    client: Arc<Client>,
    settings_version: usize,
    cx: &mut AppContext,
) -> Arc<RwLock<dyn LanguageModelCompletionProvider>> {
    match &AssistantSettings::get_global(cx).provider {
        AssistantProvider::ZedDotDev { model } => Arc::new(RwLock::new(
            CloudCompletionProvider::new(model.clone(), client.clone(), settings_version, cx),
        )),
        AssistantProvider::OpenAi {
            model,
            api_url,
            low_speed_timeout_in_seconds,
            available_models,
        } => Arc::new(RwLock::new(OpenAiCompletionProvider::new(
            choose_openai_model(&model, &available_models),
            api_url.clone(),
            client.http_client(),
            low_speed_timeout_in_seconds.map(Duration::from_secs),
            settings_version,
            available_models.clone(),
        ))),
        AssistantProvider::Anthropic {
            model,
            api_url,
            low_speed_timeout_in_seconds,
        } => Arc::new(RwLock::new(AnthropicCompletionProvider::new(
            model.clone(),
            api_url.clone(),
            client.http_client(),
            low_speed_timeout_in_seconds.map(Duration::from_secs),
            settings_version,
        ))),
        AssistantProvider::Ollama {
            model,
            api_url,
            low_speed_timeout_in_seconds,
        } => Arc::new(RwLock::new(OllamaCompletionProvider::new(
            model.clone(),
            api_url.clone(),
            client.http_client(),
            low_speed_timeout_in_seconds.map(Duration::from_secs),
            settings_version,
            cx,
        ))),
    }
}

/// Choose which model to use for openai provider.
/// If the model is not available, try to use the first available model, or fallback to the original model.
fn choose_openai_model(
    model: &::open_ai::Model,
    available_models: &[::open_ai::Model],
) -> ::open_ai::Model {
    available_models
        .iter()
        .find(|&m| m == model)
        .or_else(|| available_models.first())
        .unwrap_or_else(|| model)
        .clone()
}

#[cfg(test)]
mod tests {
    use gpui::{AppContext, UpdateGlobal};
    use settings::SettingsStore;

    use super::*;

    #[gpui::test]
    fn test_deserialize_assistant_settings(cx: &mut AppContext) {
        let store = settings::SettingsStore::test(cx);
        cx.set_global(store);

        // Settings default to gpt-4-turbo.
        AssistantSettings::register(cx);
        assert_eq!(
            AssistantSettings::get_global(cx).provider,
            AssistantProvider::OpenAi {
                model: OpenAiModel::FourOmni,
                api_url: open_ai::OPEN_AI_API_URL.into(),
                low_speed_timeout_in_seconds: None,
                available_models: Default::default(),
            }
        );

        // Ensure backward-compatibility.
        SettingsStore::update_global(cx, |store, cx| {
            store
                .set_user_settings(
                    r#"{
                        "assistant": {
                            "openai_api_url": "test-url",
                        }
                    }"#,
                    cx,
                )
                .unwrap();
        });
        assert_eq!(
            AssistantSettings::get_global(cx).provider,
            AssistantProvider::OpenAi {
                model: OpenAiModel::FourOmni,
                api_url: "test-url".into(),
                low_speed_timeout_in_seconds: None,
                available_models: Default::default(),
            }
        );
        SettingsStore::update_global(cx, |store, cx| {
            store
                .set_user_settings(
                    r#"{
                        "assistant": {
                            "default_open_ai_model": "gpt-4-0613"
                        }
                    }"#,
                    cx,
                )
                .unwrap();
        });
        assert_eq!(
            AssistantSettings::get_global(cx).provider,
            AssistantProvider::OpenAi {
                model: OpenAiModel::Four,
                api_url: open_ai::OPEN_AI_API_URL.into(),
                low_speed_timeout_in_seconds: None,
                available_models: Default::default(),
            }
        );

        // The new version supports setting a custom model when using zed.dev.
        SettingsStore::update_global(cx, |store, cx| {
            store
                .set_user_settings(
                    r#"{
                        "assistant": {
                            "version": "1",
                            "provider": {
                                "name": "zed.dev",
                                "default_model": "custom"
                            }
                        }
                    }"#,
                    cx,
                )
                .unwrap();
        });
        assert_eq!(
            AssistantSettings::get_global(cx).provider,
            AssistantProvider::ZedDotDev {
                model: CloudModel::Custom("custom".into())
            }
        );
    }
}
