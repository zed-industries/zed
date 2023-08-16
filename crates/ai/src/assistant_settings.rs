use anyhow;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::Setting;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq)]
pub enum OpenAIModel {
    #[serde(rename = "gpt-3.5")]
    GptThreePointFive,
    #[serde(rename = "gpt-4")]
    GptFour,
}

impl<T> From<T> for OpenAIModel
where
    T: AsRef<str>,
{
    fn from(s: T) -> Self {
        match s.as_ref() {
            "gpt-3.5-turbo-0613" => OpenAIModel::GptThreePointFive,
            "gpt-4-0613" => OpenAIModel::GptFour,
            _ => panic!("Unknown OpenAI model: {}", s.as_ref()),
        }
    }
}

impl OpenAIModel {
    pub fn full_name(&self) -> &'static str {
        match self {
            OpenAIModel::GptThreePointFive => "gpt-3.5-turbo-0613",
            OpenAIModel::GptFour => "gpt-4-0613",
        }
    }

    pub fn short_name(&self) -> &'static str {
        match self {
            OpenAIModel::GptThreePointFive => "gpt-3.5",
            OpenAIModel::GptFour => "gpt-4",
        }
    }

    pub fn cycle(&mut self) -> Self {
        match self {
            OpenAIModel::GptThreePointFive => OpenAIModel::GptFour,
            OpenAIModel::GptFour => OpenAIModel::GptThreePointFive,
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
    pub dock: AssistantDockPosition,
    pub default_width: f32,
    pub default_height: f32,
    pub default_open_ai_model: OpenAIModel,
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, Debug)]
pub struct AssistantSettingsContent {
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
