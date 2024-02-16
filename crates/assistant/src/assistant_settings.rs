use anyhow::Result;
use gpui::Pixels;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::Settings;
use tiktoken_rs::ChatCompletionRequestMessage;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq)]
#[serde(untagged)]
pub enum LanguageModel {
    OpenAi(OpenAiModel),
}

impl Default for LanguageModel {
    fn default() -> Self {
        LanguageModel::OpenAi(OpenAiModel::ThreePointFiveTurbo)
    }
}

impl LanguageModel {
    pub fn id(&self) -> String {
        match self {
            LanguageModel::OpenAi(model) => format!("openai/{}", model.full_name()),
        }
    }

    pub fn display_name(&self) -> String {
        match self {
            LanguageModel::OpenAi(model) => format!("openai/{}", model.short_name()),
        }
    }

    pub fn max_token_count(&self) -> usize {
        match self {
            LanguageModel::OpenAi(model) => tiktoken_rs::model::get_context_size(model.full_name()),
        }
    }

    pub fn count_tokens(&self, messages: &[ChatCompletionRequestMessage]) -> Result<usize> {
        match self {
            LanguageModel::OpenAi(model) => {
                tiktoken_rs::num_tokens_from_messages(&model.full_name(), &messages)
            }
        }
    }

    pub fn cycle(&self) -> Self {
        match self {
            LanguageModel::OpenAi(model) => LanguageModel::OpenAi(model.cycle()),
        }
    }
}

#[derive(Clone, Default, Debug, Serialize, Deserialize, JsonSchema, PartialEq)]
pub enum OpenAiModel {
    #[serde(rename = "gpt-3.5-turbo-0613")]
    ThreePointFiveTurbo,
    #[serde(rename = "gpt-4-0613")]
    Four,
    #[serde(rename = "gpt-4-1106-preview")]
    #[default]
    FourTurbo,
}

impl OpenAiModel {
    pub fn full_name(&self) -> &'static str {
        match self {
            OpenAiModel::ThreePointFiveTurbo => "gpt-3.5-turbo-0613",
            OpenAiModel::Four => "gpt-4-0613",
            OpenAiModel::FourTurbo => "gpt-4-1106-preview",
        }
    }

    pub fn short_name(&self) -> &'static str {
        match self {
            OpenAiModel::ThreePointFiveTurbo => "gpt-3.5-turbo",
            OpenAiModel::Four => "gpt-4",
            OpenAiModel::FourTurbo => "gpt-4-turbo",
        }
    }

    pub fn cycle(&self) -> Self {
        match self {
            OpenAiModel::ThreePointFiveTurbo => OpenAiModel::Four,
            OpenAiModel::Four => OpenAiModel::FourTurbo,
            OpenAiModel::FourTurbo => OpenAiModel::ThreePointFiveTurbo,
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

#[derive(Clone, Serialize, Deserialize, JsonSchema, Debug)]
pub enum AssistantProvider {
    ZedDotDev {
        default_model: LanguageModel,
    },
    OpenAi {
        default_model: OpenAiModel,
        api_url: String,
    },
}

impl Default for AssistantProvider {
    fn default() -> Self {
        AssistantProvider::ZedDotDev {
            default_model: LanguageModel::default(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct AssistantSettings {
    pub button: bool,
    pub dock: AssistantDockPosition,
    pub default_width: Pixels,
    pub default_height: Pixels,
    pub provider: AssistantProvider,
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

    provider: AssistantProvider,
}

/// Assistant panel settings
#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, Debug)]
pub struct AssistantSettingsContentV0 {
    pub button: Option<bool>,
    pub dock: Option<AssistantDockPosition>,
    pub default_width: Option<f32>,
    pub default_height: Option<f32>,
    pub default_open_ai_model: Option<OpenAiModel>,
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
        Self::load_via_json_merge(default_value, user_values)
    }
}
