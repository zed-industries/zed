use std::{fmt, time::Duration};

use crate::{
    preprocess_anthropic_request, AnthropicSettings, LanguageModel, LanguageModelRequest,
    LanguageModelSettings, OllamaSettings, OpenAiSettings,
};
pub use anthropic::Model as AnthropicModel;
use gpui::Pixels;
pub use ollama::Model as OllamaModel;
pub use open_ai::Model as OpenAiModel;
use schemars::{
    schema::{InstanceType, Metadata, Schema, SchemaObject},
    JsonSchema,
};
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use settings::{Settings, SettingsSources};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Clone, Debug, Default, PartialEq, EnumIter)]
pub enum CloudModel {
    Gpt3Point5Turbo,
    Gpt4,
    Gpt4Turbo,
    #[default]
    Gpt4Omni,
    Claude3_5Sonnet,
    Claude3Opus,
    Claude3Sonnet,
    Claude3Haiku,
    Custom(String),
}

impl Serialize for CloudModel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.id())
    }
}

impl<'de> Deserialize<'de> for CloudModel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ZedDotDevModelVisitor;

        impl<'de> Visitor<'de> for ZedDotDevModelVisitor {
            type Value = CloudModel;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string for a ZedDotDevModel variant or a custom model")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let model = CloudModel::iter()
                    .find(|model| model.id() == value)
                    .unwrap_or_else(|| CloudModel::Custom(value.to_string()));
                Ok(model)
            }
        }

        deserializer.deserialize_str(ZedDotDevModelVisitor)
    }
}

impl JsonSchema for CloudModel {
    fn schema_name() -> String {
        "ZedDotDevModel".to_owned()
    }

    fn json_schema(_generator: &mut schemars::gen::SchemaGenerator) -> Schema {
        let variants = CloudModel::iter()
            .filter_map(|model| {
                let id = model.id();
                if id.is_empty() {
                    None
                } else {
                    Some(id.to_string())
                }
            })
            .collect::<Vec<_>>();
        Schema::Object(SchemaObject {
            instance_type: Some(InstanceType::String.into()),
            enum_values: Some(variants.iter().map(|s| s.clone().into()).collect()),
            metadata: Some(Box::new(Metadata {
                title: Some("ZedDotDevModel".to_owned()),
                default: Some(CloudModel::default().id().into()),
                examples: variants.into_iter().map(Into::into).collect(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}

impl CloudModel {
    pub fn id(&self) -> &str {
        match self {
            Self::Gpt3Point5Turbo => "gpt-3.5-turbo",
            Self::Gpt4 => "gpt-4",
            Self::Gpt4Turbo => "gpt-4-turbo-preview",
            Self::Gpt4Omni => "gpt-4o",
            Self::Claude3_5Sonnet => "claude-3-5-sonnet",
            Self::Claude3Opus => "claude-3-opus",
            Self::Claude3Sonnet => "claude-3-sonnet",
            Self::Claude3Haiku => "claude-3-haiku",
            Self::Custom(id) => id,
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::Gpt3Point5Turbo => "GPT 3.5 Turbo",
            Self::Gpt4 => "GPT 4",
            Self::Gpt4Turbo => "GPT 4 Turbo",
            Self::Gpt4Omni => "GPT 4 Omni",
            Self::Claude3_5Sonnet => "Claude 3.5 Sonnet",
            Self::Claude3Opus => "Claude 3 Opus",
            Self::Claude3Sonnet => "Claude 3 Sonnet",
            Self::Claude3Haiku => "Claude 3 Haiku",
            Self::Custom(id) => id.as_str(),
        }
    }

    pub fn max_token_count(&self) -> usize {
        match self {
            Self::Gpt3Point5Turbo => 2048,
            Self::Gpt4 => 4096,
            Self::Gpt4Turbo | Self::Gpt4Omni => 128000,
            Self::Claude3_5Sonnet
            | Self::Claude3Opus
            | Self::Claude3Sonnet
            | Self::Claude3Haiku => 200000,
            Self::Custom(_) => 4096, // TODO: Make this configurable
        }
    }

    pub fn preprocess_request(&self, request: &mut LanguageModelRequest) {
        match self {
            Self::Claude3Opus | Self::Claude3Sonnet | Self::Claude3Haiku => {
                preprocess_anthropic_request(request)
            }
            _ => {}
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

#[derive(Debug, PartialEq)]
pub enum AssistantProviderV1 {
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

impl Default for AssistantProviderV1 {
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

#[derive(Clone, Debug, PartialEq)]
pub enum AssistantProvider {
    ZedDotDev,
    OpenAi(OpenAiSettings),
    Anthropic(AnthropicSettings),
    Ollama(OllamaSettings),
}

impl AssistantProvider {
    pub fn settings(&self) -> &dyn LanguageModelSettings {
        match self {
            AssistantProvider::ZedDotDev => &(),
            AssistantProvider::OpenAi(settings) => settings,
            AssistantProvider::Anthropic(settings) => settings,
            AssistantProvider::Ollama(settings) => settings,
        }
    }
}

impl Default for AssistantProvider {
    fn default() -> Self {
        Self::OpenAi(OpenAiSettings::default())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(tag = "name", rename_all = "snake_case")]
pub enum AssistantProviderContentV2 {
    #[serde(rename = "zed.dev")]
    ZedDotDev,
    #[serde(rename = "openai")]
    OpenAi {
        api_url: Option<String>,
        low_speed_timeout_in_seconds: Option<u64>,
        available_models: Option<Vec<OpenAiModel>>,
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

#[derive(Debug, Default)]
pub struct AssistantSettings {
    pub enabled: bool,
    pub button: bool,
    pub dock: AssistantDockPosition,
    pub default_width: Pixels,
    pub default_height: Pixels,
    pub default_model: AssistentDefaultModel,
    pub providers: Vec<AssistantProvider>,
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
    fn upgrade(&self) -> AssistentSettingsContentV2 {
        match self {
            AssistantSettingsContent::Versioned(settings) => match settings {
                VersionedAssistantSettingsContent::V1(settings) => AssistentSettingsContentV2 {
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
                                default_model.map(|model| AssistentDefaultModel::Cloud { model })
                            }
                            AssistantProviderContentV1::OpenAi { default_model, .. } => {
                                default_model.map(|model| AssistentDefaultModel::OpenAi { model })
                            }
                            AssistantProviderContentV1::Anthropic { default_model, .. } => {
                                default_model
                                    .map(|model| AssistentDefaultModel::Anthropic { model })
                            }
                            AssistantProviderContentV1::Ollama { default_model, .. } => {
                                default_model.map(|model| AssistentDefaultModel::Ollama { model })
                            }
                        }),
                    providers: settings
                        .provider
                        .clone()
                        .map(|provider| match provider {
                            AssistantProviderContentV1::ZedDotDev { default_model: _ } => {
                                AssistantProviderContentV2::ZedDotDev
                            }
                            AssistantProviderContentV1::OpenAi {
                                default_model: _,
                                api_url,
                                low_speed_timeout_in_seconds,
                                available_models,
                            } => AssistantProviderContentV2::OpenAi {
                                api_url,
                                low_speed_timeout_in_seconds,
                                available_models,
                            },
                            AssistantProviderContentV1::Anthropic {
                                default_model: _,
                                api_url,
                                low_speed_timeout_in_seconds,
                            } => AssistantProviderContentV2::Anthropic {
                                api_url,
                                low_speed_timeout_in_seconds,
                            },
                            AssistantProviderContentV1::Ollama {
                                default_model: _,
                                api_url,
                                low_speed_timeout_in_seconds,
                            } => AssistantProviderContentV2::Anthropic {
                                api_url,
                                low_speed_timeout_in_seconds,
                            },
                        })
                        .into_iter()
                        .collect(),
                },
                VersionedAssistantSettingsContent::V2(settings) => settings.clone(),
            },
            AssistantSettingsContent::Legacy(settings) => AssistentSettingsContentV2 {
                enabled: None,
                button: settings.button,
                dock: settings.dock,
                default_width: settings.default_width,
                default_height: settings.default_height,
                default_model: settings
                    .default_open_ai_model
                    .clone()
                    .map(|model| AssistentDefaultModel::OpenAi { model }),
                providers: vec![AssistantProviderContentV2::OpenAi {
                    api_url: settings.openai_api_url.clone(),
                    low_speed_timeout_in_seconds: None,
                    available_models: Some(Default::default()),
                }],
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

    pub fn set_model(&mut self, new_model: LanguageModel) {
        match self {
            AssistantSettingsContent::Versioned(settings) => match settings {
                VersionedAssistantSettingsContent::V1(settings) => match &mut settings.provider {
                    Some(AssistantProviderContentV1::ZedDotDev {
                        default_model: model,
                    }) => {
                        if let LanguageModel::Cloud(new_model) = new_model {
                            *model = Some(new_model);
                        }
                    }
                    Some(AssistantProviderContentV1::OpenAi {
                        default_model: model,
                        ..
                    }) => {
                        if let LanguageModel::OpenAi(new_model) = new_model {
                            *model = Some(new_model);
                        }
                    }
                    Some(AssistantProviderContentV1::Anthropic {
                        default_model: model,
                        ..
                    }) => {
                        if let LanguageModel::Anthropic(new_model) = new_model {
                            *model = Some(new_model);
                        }
                    }
                    Some(AssistantProviderContentV1::Ollama {
                        default_model: model,
                        ..
                    }) => {
                        if let LanguageModel::Ollama(new_model) = new_model {
                            *model = Some(new_model);
                        }
                    }
                    provider => match new_model {
                        LanguageModel::Cloud(model) => {
                            *provider = Some(AssistantProviderContentV1::ZedDotDev {
                                default_model: Some(model),
                            })
                        }
                        LanguageModel::OpenAi(model) => {
                            *provider = Some(AssistantProviderContentV1::OpenAi {
                                default_model: Some(model),
                                api_url: None,
                                low_speed_timeout_in_seconds: None,
                                available_models: Some(Default::default()),
                            })
                        }
                        LanguageModel::Anthropic(model) => {
                            *provider = Some(AssistantProviderContentV1::Anthropic {
                                default_model: Some(model),
                                api_url: None,
                                low_speed_timeout_in_seconds: None,
                            })
                        }
                        LanguageModel::Ollama(model) => {
                            *provider = Some(AssistantProviderContentV1::Ollama {
                                default_model: Some(model),
                                api_url: None,
                                low_speed_timeout_in_seconds: None,
                            })
                        }
                    },
                },
                VersionedAssistantSettingsContent::V2(settings) => {
                    let model = match new_model {
                        LanguageModel::Cloud(model) => AssistentDefaultModel::Cloud { model },
                        LanguageModel::OpenAi(model) => AssistentDefaultModel::OpenAi { model },
                        LanguageModel::Anthropic(model) => {
                            AssistentDefaultModel::Anthropic { model }
                        }
                        LanguageModel::Ollama(model) => AssistentDefaultModel::Ollama { model },
                    };
                    settings.default_model = Some(model);
                }
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
    #[serde(rename = "2")]
    V2(AssistentSettingsContentV2),
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
    provider: Option<AssistantProviderContentV1>,
}

#[derive(Clone, Serialize, Deserialize, JsonSchema, Debug)]
pub struct AssistentSettingsContentV2 {
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
    default_model: Option<AssistentDefaultModel>,

    /// The list of providers of the assistant service.
    ///
    /// The list of currently supported providers are:
    /// - "zed.dev"
    /// - "openai"
    /// - "anthropic"
    /// - "ollama"
    providers: Vec<AssistantProviderContentV2>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(tag = "name", rename_all = "snake_case")]
pub enum AssistentDefaultModel {
    #[serde(rename = "zed.dev")]
    Cloud { model: CloudModel },
    #[serde(rename = "openai")]
    OpenAi { model: OpenAiModel },
    #[serde(rename = "anthropic")]
    Anthropic { model: AnthropicModel },
    #[serde(rename = "ollama")]
    Ollama { model: OllamaModel },
}

impl Default for AssistentDefaultModel {
    fn default() -> Self {
        Self::OpenAi {
            model: OpenAiModel::default(),
        }
    }
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

            let mut new_providers = Vec::new();
            for provider in value.providers.into_iter() {
                let mut found = false;
                for settings_provider in &mut settings.providers {
                    match (settings_provider, provider.clone()) {
                        (AssistantProvider::ZedDotDev, AssistantProviderContentV2::ZedDotDev) => {
                            found = true;
                        }
                        (
                            AssistantProvider::OpenAi(settings),
                            AssistantProviderContentV2::OpenAi {
                                api_url: api_url_override,
                                low_speed_timeout_in_seconds: low_speed_timeout_in_seconds_override,
                                available_models: available_models_override,
                            },
                        ) => {
                            merge(&mut settings.api_url, api_url_override);
                            merge(&mut settings.available_models, available_models_override);
                            if let Some(low_speed_timeout_in_seconds_override) =
                                low_speed_timeout_in_seconds_override
                            {
                                settings.low_speed_timeout = Some(Duration::from_secs(
                                    low_speed_timeout_in_seconds_override,
                                ));
                            }
                            found = true;
                        }
                        (
                            AssistantProvider::Ollama(settings),
                            AssistantProviderContentV2::Ollama {
                                api_url: api_url_override,
                                low_speed_timeout_in_seconds: low_speed_timeout_in_seconds_override,
                            },
                        ) => {
                            merge(&mut settings.api_url, api_url_override);
                            if let Some(low_speed_timeout_in_seconds_override) =
                                low_speed_timeout_in_seconds_override
                            {
                                settings.low_speed_timeout = Some(Duration::from_secs(
                                    low_speed_timeout_in_seconds_override,
                                ));
                            }
                            found = true;
                        }
                        (
                            AssistantProvider::Anthropic(settings),
                            AssistantProviderContentV2::Anthropic {
                                api_url: api_url_override,
                                low_speed_timeout_in_seconds: low_speed_timeout_in_seconds_override,
                            },
                        ) => {
                            merge(&mut settings.api_url, api_url_override);
                            if let Some(low_speed_timeout_in_seconds_override) =
                                low_speed_timeout_in_seconds_override
                            {
                                settings.low_speed_timeout = Some(Duration::from_secs(
                                    low_speed_timeout_in_seconds_override,
                                ));
                            }
                            found = true;
                        }
                        _ => {}
                    }
                }

                if !found {
                    new_providers.push(match provider {
                        AssistantProviderContentV2::ZedDotDev => AssistantProvider::ZedDotDev,
                        AssistantProviderContentV2::OpenAi {
                            api_url,
                            low_speed_timeout_in_seconds,
                            available_models,
                        } => AssistantProvider::OpenAi(OpenAiSettings {
                            api_url: api_url.unwrap_or_else(|| open_ai::OPEN_AI_API_URL.into()),
                            low_speed_timeout: low_speed_timeout_in_seconds
                                .map(Duration::from_secs),
                            available_models: available_models.unwrap_or_default(),
                        }),
                        AssistantProviderContentV2::Anthropic {
                            api_url,
                            low_speed_timeout_in_seconds,
                        } => AssistantProvider::Anthropic(AnthropicSettings {
                            api_url: api_url.unwrap_or_else(|| anthropic::ANTHROPIC_API_URL.into()),
                            low_speed_timeout: low_speed_timeout_in_seconds
                                .map(Duration::from_secs),
                        }),
                        AssistantProviderContentV2::Ollama {
                            api_url,
                            low_speed_timeout_in_seconds,
                        } => AssistantProvider::Ollama(OllamaSettings {
                            api_url: api_url.unwrap_or_else(|| ollama::OLLAMA_API_URL.into()),
                            low_speed_timeout: low_speed_timeout_in_seconds
                                .map(Duration::from_secs),
                        }),
                    });
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
            AssistantProviderV1::OpenAi {
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
            AssistantProviderV1::OpenAi {
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
            AssistantProviderV1::OpenAi {
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
            AssistantProviderV1::ZedDotDev {
                model: CloudModel::Custom("custom".into())
            }
        );
    }
}
