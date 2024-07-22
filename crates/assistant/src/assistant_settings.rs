use anthropic::Model as AnthropicModel;
use gpui::Pixels;
use language_model::{AvailableLanguageModel, CloudModel};
use ollama::Model as OllamaModel;
use open_ai::Model as OpenAiModel;
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
    pub default_model: AssistantDefaultModel,
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
                                default_model.map(|model| AssistantDefaultModel {
                                    provider: "zed.dev".to_string(),
                                    model: model.id().to_string(),
                                })
                            }
                            AssistantProviderContentV1::OpenAi { default_model, .. } => {
                                default_model.map(|model| AssistantDefaultModel {
                                    provider: "openai".to_string(),
                                    model: model.id().to_string(),
                                })
                            }
                            AssistantProviderContentV1::Anthropic { default_model, .. } => {
                                default_model.map(|model| AssistantDefaultModel {
                                    provider: "anthropic".to_string(),
                                    model: model.id().to_string(),
                                })
                            }
                            AssistantProviderContentV1::Ollama { default_model, .. } => {
                                default_model.map(|model| AssistantDefaultModel {
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
                default_model: Some(AssistantDefaultModel {
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

    pub fn set_model(&mut self, language_model: AvailableLanguageModel) {
        let provider = language_model.provider.clone();
        let name = language_model.model.name.clone();
        let mut settings = self.upgrade();
        settings.default_model = Some(AssistantDefaultModel {
            model: name.0.to_string(),
            provider: provider.0.to_string(),
        });
        *self = Self::Versioned(VersionedAssistantSettingsContent::V2(settings));
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
    default_model: Option<AssistantDefaultModel>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq)]
pub struct AssistantDefaultModel {
    pub provider: String,
    pub model: String,
}

impl Default for AssistantDefaultModel {
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
    /// This can either be the internal `zed.dev` service or an external `openai` service,
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

// #[cfg(test)]
// mod tests {
//     use gpui::{AppContext, UpdateGlobal};
//     use settings::SettingsStore;

//     use super::*;

//     #[gpui::test]
//     fn test_deserialize_assistant_settings(cx: &mut AppContext) {
//         let store = settings::SettingsStore::test(cx);
//         cx.set_global(store);

//         // Settings default to gpt-4-turbo.
//         AssistantSettings::register(cx);
//         assert_eq!(
//             AssistantSettings::get_global(cx).provider,
//             AssistantProvider::OpenAi {
//                 model: OpenAiModel::FourOmni,
//                 api_url: open_ai::OPEN_AI_API_URL.into(),
//                 low_speed_timeout_in_seconds: None,
//                 available_models: Default::default(),
//             }
//         );

//         // Ensure backward-compatibility.
//         SettingsStore::update_global(cx, |store, cx| {
//             store
//                 .set_user_settings(
//                     r#"{
//                         "assistant": {
//                             "openai_api_url": "test-url",
//                         }
//                     }"#,
//                     cx,
//                 )
//                 .unwrap();
//         });
//         assert_eq!(
//             AssistantSettings::get_global(cx).provider,
//             AssistantProvider::OpenAi {
//                 model: OpenAiModel::FourOmni,
//                 api_url: "test-url".into(),
//                 low_speed_timeout_in_seconds: None,
//                 available_models: Default::default(),
//             }
//         );
//         SettingsStore::update_global(cx, |store, cx| {
//             store
//                 .set_user_settings(
//                     r#"{
//                         "assistant": {
//                             "default_open_ai_model": "gpt-4-0613"
//                         }
//                     }"#,
//                     cx,
//                 )
//                 .unwrap();
//         });
//         assert_eq!(
//             AssistantSettings::get_global(cx).provider,
//             AssistantProvider::OpenAi {
//                 model: OpenAiModel::Four,
//                 api_url: open_ai::OPEN_AI_API_URL.into(),
//                 low_speed_timeout_in_seconds: None,
//                 available_models: Default::default(),
//             }
//         );

//         // The new version supports setting a custom model when using zed.dev.
//         SettingsStore::update_global(cx, |store, cx| {
//             store
//                 .set_user_settings(
//                     r#"{
//                         "assistant": {
//                             "version": "1",
//                             "provider": {
//                                 "name": "zed.dev",
//                                 "default_model": {
//                                     "custom": {
//                                         "name": "custom-provider"
//                                     }
//                                 }
//                             }
//                         }
//                     }"#,
//                     cx,
//                 )
//                 .unwrap();
//         });
//         assert_eq!(
//             AssistantSettings::get_global(cx).provider,
//             AssistantProvider::ZedDotDev {
//                 model: CloudModel::Custom {
//                     name: "custom-provider".into(),
//                     max_tokens: None
//                 }
//             }
//         );
//     }
// }
