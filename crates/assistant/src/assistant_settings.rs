use ai::providers::open_ai::{
    AzureOpenAiApiVersion, OpenAiCompletionProviderKind, OPEN_AI_API_URL,
};
use anyhow::anyhow;
use gpui::Pixels;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::Settings;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OpenAiModel {
    #[serde(rename = "gpt-3.5-turbo-0613")]
    ThreePointFiveTurbo,
    #[serde(rename = "gpt-4-0613")]
    Four,
    #[serde(rename = "gpt-4-1106-preview")]
    FourTurbo,
}

impl OpenAiModel {
    pub fn full_name(&self) -> &'static str {
        match self {
            Self::ThreePointFiveTurbo => "gpt-3.5-turbo-0613",
            Self::Four => "gpt-4-0613",
            Self::FourTurbo => "gpt-4-1106-preview",
        }
    }

    pub fn short_name(&self) -> &'static str {
        match self {
            Self::ThreePointFiveTurbo => "gpt-3.5-turbo",
            Self::Four => "gpt-4",
            Self::FourTurbo => "gpt-4-turbo",
        }
    }

    pub fn cycle(&self) -> Self {
        match self {
            Self::ThreePointFiveTurbo => Self::Four,
            Self::Four => Self::FourTurbo,
            Self::FourTurbo => Self::ThreePointFiveTurbo,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AssistantDockPosition {
    Left,
    Right,
    Bottom,
}

#[derive(Debug, Deserialize)]
pub struct AssistantSettings {
    /// Whether to show the assistant panel button in the status bar.
    pub button: bool,
    /// Where to dock the assistant.
    pub dock: AssistantDockPosition,
    /// Default width in pixels when the assistant is docked to the left or right.
    pub default_width: Pixels,
    /// Default height in pixels when the assistant is docked to the bottom.
    pub default_height: Pixels,
    /// The default OpenAI model to use when starting new conversations.
    #[deprecated = "Please use `provider.default_model` instead."]
    pub default_open_ai_model: OpenAiModel,
    /// OpenAI API base URL to use when starting new conversations.
    #[deprecated = "Please use `provider.api_url` instead."]
    pub openai_api_url: String,
    /// The settings for the AI provider.
    pub provider: AiProviderSettings,
}

impl AssistantSettings {
    pub fn provider_kind(&self) -> anyhow::Result<OpenAiCompletionProviderKind> {
        match &self.provider {
            AiProviderSettings::OpenAi(_) => Ok(OpenAiCompletionProviderKind::OpenAi),
            AiProviderSettings::AzureOpenAi(settings) => {
                let deployment_id = settings
                    .deployment_id
                    .clone()
                    .ok_or_else(|| anyhow!("no Azure OpenAI deployment ID"))?;
                let api_version = settings
                    .api_version
                    .ok_or_else(|| anyhow!("no Azure OpenAI API version"))?;

                Ok(OpenAiCompletionProviderKind::AzureOpenAi {
                    deployment_id,
                    api_version,
                })
            }
        }
    }

    pub fn provider_api_url(&self) -> anyhow::Result<String> {
        match &self.provider {
            AiProviderSettings::OpenAi(settings) => Ok(settings
                .api_url
                .clone()
                .unwrap_or_else(|| OPEN_AI_API_URL.to_string())),
            AiProviderSettings::AzureOpenAi(settings) => settings
                .api_url
                .clone()
                .ok_or_else(|| anyhow!("no Azure OpenAI API URL")),
        }
    }

    pub fn provider_model(&self) -> anyhow::Result<OpenAiModel> {
        match &self.provider {
            AiProviderSettings::OpenAi(settings) => {
                Ok(settings.default_model.unwrap_or(OpenAiModel::FourTurbo))
            }
            AiProviderSettings::AzureOpenAi(settings) => {
                let deployment_id = settings
                    .deployment_id
                    .as_deref()
                    .ok_or_else(|| anyhow!("no Azure OpenAI deployment ID"))?;

                match deployment_id {
                    // https://learn.microsoft.com/en-us/azure/ai-services/openai/concepts/models#gpt-4-and-gpt-4-turbo-preview
                    "gpt-4" | "gpt-4-32k" => Ok(OpenAiModel::Four),
                    // https://learn.microsoft.com/en-us/azure/ai-services/openai/concepts/models#gpt-35
                    "gpt-35-turbo" | "gpt-35-turbo-16k" | "gpt-35-turbo-instruct" => {
                        Ok(OpenAiModel::ThreePointFiveTurbo)
                    }
                    _ => Err(anyhow!(
                        "no matching OpenAI model found for deployment ID: '{deployment_id}'"
                    )),
                }
            }
        }
    }

    pub fn provider_model_name(&self) -> anyhow::Result<String> {
        match &self.provider {
            AiProviderSettings::OpenAi(settings) => Ok(settings
                .default_model
                .unwrap_or(OpenAiModel::FourTurbo)
                .full_name()
                .to_string()),
            AiProviderSettings::AzureOpenAi(settings) => settings
                .deployment_id
                .clone()
                .ok_or_else(|| anyhow!("no Azure OpenAI deployment ID")),
        }
    }
}

impl Settings for AssistantSettings {
    const KEY: Option<&'static str> = Some("assistant");

    type FileContent = AssistantSettingsContent;

    fn load(
        default_value: &Self::FileContent,
        user_values: &[&Self::FileContent],
        _: &mut gpui::AppContext,
    ) -> anyhow::Result<Self> {
        Self::load_via_json_merge(default_value, user_values)
    }
}

/// Assistant panel settings
#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, Debug)]
pub struct AssistantSettingsContent {
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
    /// Deprecated: Please use `provider.default_model` instead.
    /// The default OpenAI model to use when starting new conversations.
    ///
    /// Default: gpt-4-1106-preview
    #[deprecated = "Please use `provider.default_model` instead."]
    pub default_open_ai_model: Option<OpenAiModel>,
    /// Deprecated: Please use `provider.api_url` instead.
    /// OpenAI API base URL to use when starting new conversations.
    ///
    /// Default: https://api.openai.com/v1
    #[deprecated = "Please use `provider.api_url` instead."]
    pub openai_api_url: Option<String>,
    /// The settings for the AI provider.
    #[serde(default)]
    pub provider: AiProviderSettingsContent,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AiProviderSettings {
    /// The settings for the OpenAI provider.
    #[serde(rename = "openai")]
    OpenAi(OpenAiProviderSettings),
    /// The settings for the Azure OpenAI provider.
    #[serde(rename = "azure_openai")]
    AzureOpenAi(AzureOpenAiProviderSettings),
}

/// The settings for the AI provider used by the Zed Assistant.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AiProviderSettingsContent {
    /// The settings for the OpenAI provider.
    #[serde(rename = "openai")]
    OpenAi(OpenAiProviderSettingsContent),
    /// The settings for the Azure OpenAI provider.
    #[serde(rename = "azure_openai")]
    AzureOpenAi(AzureOpenAiProviderSettingsContent),
}

impl Default for AiProviderSettingsContent {
    fn default() -> Self {
        Self::OpenAi(OpenAiProviderSettingsContent::default())
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct OpenAiProviderSettings {
    /// The OpenAI API base URL to use when starting new conversations.
    pub api_url: Option<String>,
    /// The default OpenAI model to use when starting new conversations.
    pub default_model: Option<OpenAiModel>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, JsonSchema)]
pub struct OpenAiProviderSettingsContent {
    /// The OpenAI API base URL to use when starting new conversations.
    ///
    /// Default: https://api.openai.com/v1
    pub api_url: Option<String>,
    /// The default OpenAI model to use when starting new conversations.
    ///
    /// Default: gpt-4-1106-preview
    pub default_model: Option<OpenAiModel>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AzureOpenAiProviderSettings {
    /// The Azure OpenAI API base URL to use when starting new conversations.
    pub api_url: Option<String>,
    /// The Azure OpenAI API version.
    pub api_version: Option<AzureOpenAiApiVersion>,
    /// The Azure OpenAI API deployment ID.
    pub deployment_id: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AzureOpenAiProviderSettingsContent {
    /// The Azure OpenAI API base URL to use when starting new conversations.
    pub api_url: Option<String>,
    /// The Azure OpenAI API version.
    pub api_version: Option<AzureOpenAiApiVersion>,
    /// The Azure OpenAI deployment ID.
    pub deployment_id: Option<String>,
}
