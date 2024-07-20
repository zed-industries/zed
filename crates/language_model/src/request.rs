use crate::{
    model::{CloudModel, LanguageModel},
    role::Role,
};
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
    pub model: LanguageModel,
    pub messages: Vec<LanguageModelRequestMessage>,
    pub stop: Vec<String>,
    pub temperature: f32,
}

impl LanguageModelRequest {
    pub fn to_proto(&self) -> proto::CompleteWithLanguageModel {
        proto::CompleteWithLanguageModel {
            model: self.model.id().to_string(),
            messages: self.messages.iter().map(|m| m.to_proto()).collect(),
            stop: self.stop.clone(),
            temperature: self.temperature,
            tool_choice: None,
            tools: Vec::new(),
        }
    }

    /// Before we send the request to the server, we can perform fixups on it appropriate to the model.
    pub fn preprocess(&mut self) {
        match &self.model {
            LanguageModel::OpenAi(_) => {}
            LanguageModel::Anthropic(_) => {}
            LanguageModel::Ollama(_) => {}
            LanguageModel::Cloud(model) => match model {
                CloudModel::Claude3Opus
                | CloudModel::Claude3Sonnet
                | CloudModel::Claude3Haiku
                | CloudModel::Claude3_5Sonnet => {
                    self.preprocess_anthropic();
                }
                _ => {}
            },
        }
    }

    pub fn preprocess_anthropic(&mut self) {
        let mut new_messages: Vec<LanguageModelRequestMessage> = Vec::new();
        let mut system_message = String::new();

        for message in self.messages.drain(..) {
            if message.content.is_empty() {
                continue;
            }

            match message.role {
                Role::User | Role::Assistant => {
                    if let Some(last_message) = new_messages.last_mut() {
                        if last_message.role == message.role {
                            last_message.content.push_str("\n\n");
                            last_message.content.push_str(&message.content);
                            continue;
                        }
                    }

                    new_messages.push(message);
                }
                Role::System => {
                    if !system_message.is_empty() {
                        system_message.push_str("\n\n");
                    }
                    system_message.push_str(&message.content);
                }
            }
        }

        if !system_message.is_empty() {
            new_messages.insert(
                0,
                LanguageModelRequestMessage {
                    role: Role::System,
                    content: system_message,
                },
            );
        }

        self.messages = new_messages;
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct LanguageModelResponseMessage {
    pub role: Option<Role>,
    pub content: Option<String>,
}
