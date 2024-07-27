use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "provider", rename_all = "lowercase")]
pub enum CloudModel {
    Anthropic(anthropic::Model),
    OpenAi(open_ai::Model),
    Google(google_ai::Model),
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
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            CloudModel::Anthropic(model) => model.display_name(),
            CloudModel::OpenAi(model) => model.display_name(),
            CloudModel::Google(model) => model.display_name(),
        }
    }

    pub fn max_token_count(&self) -> usize {
        match self {
            CloudModel::Anthropic(model) => model.max_token_count(),
            CloudModel::OpenAi(model) => model.max_token_count(),
            CloudModel::Google(model) => model.max_token_count(),
        }
    }
}
