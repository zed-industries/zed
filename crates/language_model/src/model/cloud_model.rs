use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "provider", rename_all = "lowercase")]
pub enum CloudModel {
    Anthropic(anthropic::Model),
    OpenAi(open_ai::Model),
    Google(google_ai::Model),
    Zed(ZedModel),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, EnumIter)]
pub enum ZedModel {
    #[serde(rename = "qwen2-7b-instruct")]
    Qwen2_7bInstruct,
}

impl ZedModel {
    pub fn id(&self) -> &str {
        match self {
            ZedModel::Qwen2_7bInstruct => "qwen2-7b-instruct",
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            ZedModel::Qwen2_7bInstruct => "Qwen2 7B Instruct",
        }
    }

    pub fn max_token_count(&self) -> usize {
        match self {
            ZedModel::Qwen2_7bInstruct => 28000,
        }
    }
}

impl Default for CloudModel {
    fn default() -> Self {
        Self::Anthropic(anthropic::Model::default())
    }
}

impl CloudModel {
    pub fn id(&self) -> &str {
        match self {
            CloudModel::Anthropic(model) => model.id(),
            CloudModel::OpenAi(model) => model.id(),
            CloudModel::Google(model) => model.id(),
            CloudModel::Zed(model) => model.id(),
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            CloudModel::Anthropic(model) => model.display_name(),
            CloudModel::OpenAi(model) => model.display_name(),
            CloudModel::Google(model) => model.display_name(),
            CloudModel::Zed(model) => model.display_name(),
        }
    }

    pub fn max_token_count(&self) -> usize {
        match self {
            CloudModel::Anthropic(model) => model.max_token_count(),
            CloudModel::OpenAi(model) => model.max_token_count(),
            CloudModel::Google(model) => model.max_token_count(),
            CloudModel::Zed(model) => model.max_token_count(),
        }
    }
}
