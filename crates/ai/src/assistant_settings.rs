use anyhow;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::Setting;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq)]
pub enum OpenAIModel {
    #[serde(rename = "gpt-3.5-turbo-0613")]
    GptThreeFiveTurbo0613,
    #[serde(rename = "gpt-4-0613")]
    GptFour0613,
}

impl OpenAIModel {
    pub fn full_name(&self) -> &'static str {
        match self {
            OpenAIModel::GptThreeFiveTurbo0613 => "gpt-3.5-turbo-0613",
            OpenAIModel::GptFour0613 => "gpt-4-0613",
        }
    }

    pub fn short_name(&self) -> &'static str {
        match self {
            OpenAIModel::GptThreeFiveTurbo0613 => "gpt-3.5-turbo",
            OpenAIModel::GptFour0613 => "gpt-4",
        }
    }

    pub fn cycle(&mut self) -> Self {
        match self {
            OpenAIModel::GptThreeFiveTurbo0613 => OpenAIModel::GptFour0613,
            OpenAIModel::GptFour0613 => OpenAIModel::GptThreeFiveTurbo0613,
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

#[derive(Deserialize, Debug)]
pub struct AssistantSettings {
    pub button: bool,
    pub dock: AssistantDockPosition,
    pub default_width: f32,
    pub default_height: f32,
    pub default_open_ai_model: OpenAIModel,
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, Debug)]
pub struct AssistantSettingsContent {
    pub button: Option<bool>,
    pub dock: Option<AssistantDockPosition>,
    pub default_width: Option<f32>,
    pub default_height: Option<f32>,
    pub default_open_ai_model: Option<OpenAIModel>,
}

impl Setting for AssistantSettings {
    const KEY: Option<&'static str> = Some("assistant");

    type FileContent = AssistantSettingsContent;

    fn load(
        default_value: &Self::FileContent,
        user_values: &[&Self::FileContent],
        _: &gpui::AppContext,
    ) -> anyhow::Result<Self> {
        Self::load_via_json_merge(default_value, user_values)
    }
}
