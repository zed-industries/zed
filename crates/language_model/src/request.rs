use crate::role::Role;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Hash)]
pub struct LanguageModelRequestMessage {
    pub role: Role,
    pub content: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct LanguageModelRequest {
    pub messages: Vec<LanguageModelRequestMessage>,
    pub stop: Vec<String>,
    pub temperature: f32,
}

impl LanguageModelRequest {
    pub fn into_open_ai(self, model: String) -> open_ai::Request {
        open_ai::Request {
            model,
            messages: self
                .messages
                .into_iter()
                .map(|msg| match msg.role {
                    Role::User => open_ai::RequestMessage::User {
                        content: msg.content,
                    },
                    Role::Assistant => open_ai::RequestMessage::Assistant {
                        content: Some(msg.content),
                        tool_calls: Vec::new(),
                    },
                    Role::System => open_ai::RequestMessage::System {
                        content: msg.content,
                    },
                })
                .collect(),
            stream: true,
            stop: self.stop,
            temperature: self.temperature,
            max_tokens: None,
            tools: Vec::new(),
            tool_choice: None,
        }
    }

    pub fn into_google(self, model: String) -> google_ai::GenerateContentRequest {
        google_ai::GenerateContentRequest {
            model,
            contents: self
                .messages
                .into_iter()
                .map(|msg| google_ai::Content {
                    parts: vec![google_ai::Part::TextPart(google_ai::TextPart {
                        text: msg.content,
                    })],
                    role: match msg.role {
                        Role::User => google_ai::Role::User,
                        Role::Assistant => google_ai::Role::Model,
                        Role::System => google_ai::Role::User, // Google AI doesn't have a system role
                    },
                })
                .collect(),
            generation_config: Some(google_ai::GenerationConfig {
                candidate_count: Some(1),
                stop_sequences: Some(self.stop),
                max_output_tokens: None,
                temperature: Some(self.temperature as f64),
                top_p: None,
                top_k: None,
            }),
            safety_settings: None,
        }
    }

    pub fn into_anthropic(self, model: String) -> anthropic::Request {
        let mut new_messages: Vec<LanguageModelRequestMessage> = Vec::new();
        let mut system_message = String::new();

        for message in self.messages {
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

        anthropic::Request {
            model,
            messages: new_messages
                .into_iter()
                .filter_map(|message| {
                    Some(anthropic::Message {
                        role: match message.role {
                            Role::User => anthropic::Role::User,
                            Role::Assistant => anthropic::Role::Assistant,
                            Role::System => return None,
                        },
                        content: vec![anthropic::Content::Text {
                            text: message.content,
                        }],
                    })
                })
                .collect(),
            max_tokens: 4092,
            system: Some(system_message),
            tools: Vec::new(),
            tool_choice: None,
            metadata: None,
            stop_sequences: Vec::new(),
            temperature: None,
            top_k: None,
            top_p: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct LanguageModelResponseMessage {
    pub role: Option<Role>,
    pub content: Option<String>,
}
