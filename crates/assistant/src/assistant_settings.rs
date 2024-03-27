use std::fmt;

use gpui::Pixels;
pub use open_ai::Model as OpenAiModel;
use schemars::{
    schema::{InstanceType, Metadata, Schema, SchemaObject},
    JsonSchema,
};
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use settings::Settings;

#[derive(Clone, Debug, Default, PartialEq)]
pub enum ZedDotDevModel {
    GptThreePointFiveTurbo,
    GptFour,
    #[default]
    GptFourTurbo,
    Custom(String),
}

impl Serialize for ZedDotDevModel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.id())
    }
}

impl<'de> Deserialize<'de> for ZedDotDevModel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ZedDotDevModelVisitor;

        impl<'de> Visitor<'de> for ZedDotDevModelVisitor {
            type Value = ZedDotDevModel;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string for a ZedDotDevModel variant or a custom model")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match value {
                    "gpt-3.5-turbo" => Ok(ZedDotDevModel::GptThreePointFiveTurbo),
                    "gpt-4" => Ok(ZedDotDevModel::GptFour),
                    "gpt-4-turbo-preview" => Ok(ZedDotDevModel::GptFourTurbo),
                    _ => Ok(ZedDotDevModel::Custom(value.to_owned())),
                }
            }
        }

        deserializer.deserialize_str(ZedDotDevModelVisitor)
    }
}

impl JsonSchema for ZedDotDevModel {
    fn schema_name() -> String {
        "ZedDotDevModel".to_owned()
    }

    fn json_schema(_generator: &mut schemars::gen::SchemaGenerator) -> Schema {
        let variants = vec![
            "gpt-3.5-turbo".to_owned(),
            "gpt-4".to_owned(),
            "gpt-4-turbo-preview".to_owned(),
        ];
        Schema::Object(SchemaObject {
            instance_type: Some(InstanceType::String.into()),
            enum_values: Some(variants.into_iter().map(|s| s.into()).collect()),
            metadata: Some(Box::new(Metadata {
                title: Some("ZedDotDevModel".to_owned()),
                default: Some(serde_json::json!("gpt-4-turbo-preview")),
                examples: vec![
                    serde_json::json!("gpt-3.5-turbo"),
                    serde_json::json!("gpt-4"),
                    serde_json::json!("gpt-4-turbo-preview"),
                    serde_json::json!("custom-model-name"),
                ],
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}

impl ZedDotDevModel {
    pub fn id(&self) -> &str {
        match self {
            Self::GptThreePointFiveTurbo => "gpt-3.5-turbo",
            Self::GptFour => "gpt-4",
            Self::GptFourTurbo => "gpt-4-turbo-preview",
            Self::Custom(id) => id,
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::GptThreePointFiveTurbo => "gpt-3.5-turbo",
            Self::GptFour => "gpt-4",
            Self::GptFourTurbo => "gpt-4-turbo",
            Self::Custom(id) => id.as_str(),
        }
    }

    pub fn max_token_count(&self) -> usize {
        match self {
            Self::GptThreePointFiveTurbo => 2048,
            Self::GptFour => 4096,
            Self::GptFourTurbo => 128000,
            Self::Custom(_) => 4096, // TODO: Make this configurable
        }
    }
}

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
pub enum AssistantProvider {
    #[serde(rename = "zed.dev")]
    ZedDotDev {
        #[serde(default)]
        default_model: ZedDotDevModel,
    },
    #[serde(rename = "openai")]
    OpenAi {
        #[serde(default)]
        default_model: OpenAiModel,
        #[serde(default = "open_ai_url")]
        api_url: String,
    },
}

impl Default for AssistantProvider {
    fn default() -> Self {
        Self::ZedDotDev {
            default_model: ZedDotDevModel::default(),
        }
    }
}

fn open_ai_url() -> String {
    "https://api.openai.com/v1".into()
}

#[derive(Default, Debug, Deserialize, Serialize)]
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
                    Some(AssistantProvider::OpenAi {
                        default_model: settings.default_open_ai_model.clone().unwrap_or_default(),
                        api_url: open_ai_api_url.clone(),
                    })
                } else {
                    settings.default_open_ai_model.clone().map(|open_ai_model| {
                        AssistantProvider::OpenAi {
                            default_model: open_ai_model,
                            api_url: open_ai_url(),
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
    provider: Option<AssistantProvider>,
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
    /// The default OpenAI model to use when starting new conversations.
    ///
    /// Default: gpt-4-1106-preview
    pub default_open_ai_model: Option<OpenAiModel>,
    /// OpenAI API base URL to use when starting new conversations.
    ///
    /// Default: https://api.openai.com/v1
    pub openai_api_url: Option<String>,
}

impl Settings for AssistantSettings {
    const KEY: Option<&'static str> = Some("assistant");

    type FileContent = AssistantSettingsContent;

    fn load(
        default_value: &Self::FileContent,
        user_values: &[&Self::FileContent],
        _: &mut gpui::AppContext,
    ) -> anyhow::Result<Self> {
        let mut settings = AssistantSettings::default();

        for value in [default_value].iter().chain(user_values) {
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
                        AssistantProvider::ZedDotDev { default_model },
                        AssistantProvider::ZedDotDev {
                            default_model: default_model_override,
                        },
                    ) => {
                        *default_model = default_model_override;
                    }
                    (
                        AssistantProvider::OpenAi {
                            default_model,
                            api_url,
                        },
                        AssistantProvider::OpenAi {
                            default_model: default_model_override,
                            api_url: api_url_override,
                        },
                    ) => {
                        *default_model = default_model_override;
                        *api_url = api_url_override;
                    }
                    (merged, provider_override) => {
                        *merged = provider_override;
                    }
                }
            }
        }

        Ok(settings)
    }
}

fn merge<T: Copy>(target: &mut T, value: Option<T>) {
    if let Some(value) = value {
        *target = value;
    }
}

#[cfg(test)]
mod tests {
    use gpui::{AppContext, BorrowAppContext};
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
                default_model: OpenAiModel::FourTurbo,
                api_url: open_ai_url()
            }
        );

        // Ensure backward-compatibility.
        cx.update_global::<SettingsStore, _>(|store, cx| {
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
                default_model: OpenAiModel::FourTurbo,
                api_url: "test-url".into()
            }
        );
        cx.update_global::<SettingsStore, _>(|store, cx| {
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
                default_model: OpenAiModel::Four,
                api_url: open_ai_url()
            }
        );

        // The new version supports setting a custom model when using zed.dev.
        cx.update_global::<SettingsStore, _>(|store, cx| {
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
                default_model: ZedDotDevModel::Custom("custom".into())
            }
        );
    }
}
