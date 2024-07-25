use crate::{role::Role, LanguageModelId};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct LanguageModelRequestMessage {
    pub role: Role,
    pub content: String,
}

impl LanguageModelRequestMessage {
    pub fn to_proto(&self) -> proto::LanguageModelRequestMessage {
        proto::LanguageModelRequestMessage {
            role: self.role.to_proto() as i32,
            content: self.content.clone(),
            tool_calls: Vec::new(),
            tool_call_id: None,
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LanguageModelRequest {
    pub messages: Vec<LanguageModelRequestMessage>,
    pub stop: Vec<String>,
    pub temperature: f32,
}

impl LanguageModelRequest {
    pub fn to_proto(&self, model_id: LanguageModelId) -> proto::CompleteWithLanguageModel {
        proto::CompleteWithLanguageModel {
            model: model_id.0.to_string(),
            messages: self.messages.iter().map(|m| m.to_proto()).collect(),
            stop: self.stop.clone(),
            temperature: self.temperature,
            tool_choice: None,
            tools: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct LanguageModelResponseMessage {
    pub role: Option<Role>,
    pub content: Option<String>,
}
