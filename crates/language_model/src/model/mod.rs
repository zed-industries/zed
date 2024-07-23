pub mod cloud_model;

pub use anthropic::Model as AnthropicModel;
pub use cloud_model::*;
pub use ollama::Model as OllamaModel;
pub use open_ai::Model as OpenAiModel;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum LanguageModel {
    Cloud(CloudModel),
    OpenAi(OpenAiModel),
    Anthropic(AnthropicModel),
    Ollama(OllamaModel),
}

impl Default for LanguageModel {
    fn default() -> Self {
        LanguageModel::Cloud(CloudModel::default())
    }
}

impl LanguageModel {
    pub fn telemetry_id(&self) -> String {
        match self {
            LanguageModel::OpenAi(model) => format!("openai/{}", model.id()),
            LanguageModel::Anthropic(model) => format!("anthropic/{}", model.id()),
            LanguageModel::Cloud(model) => format!("zed.dev/{}", model.id()),
            LanguageModel::Ollama(model) => format!("ollama/{}", model.id()),
        }
    }

    pub fn display_name(&self) -> String {
        match self {
            LanguageModel::OpenAi(model) => model.display_name().into(),
            LanguageModel::Anthropic(model) => model.display_name().into(),
            LanguageModel::Cloud(model) => model.display_name().into(),
            LanguageModel::Ollama(model) => model.display_name().into(),
        }
    }

    pub fn max_token_count(&self) -> usize {
        match self {
            LanguageModel::OpenAi(model) => model.max_token_count(),
            LanguageModel::Anthropic(model) => model.max_token_count(),
            LanguageModel::Cloud(model) => model.max_token_count(),
            LanguageModel::Ollama(model) => model.max_token_count(),
        }
    }

    pub fn id(&self) -> &str {
        match self {
            LanguageModel::OpenAi(model) => model.id(),
            LanguageModel::Anthropic(model) => model.id(),
            LanguageModel::Cloud(model) => model.id(),
            LanguageModel::Ollama(model) => model.id(),
        }
    }
}
