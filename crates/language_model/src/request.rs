use crate::role::Role;
use base64::Engine;
use gpui::{AppContext, DevicePixels, ImageSource, Size};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct LanguageModelImage {
    source: String,
    size: Size<DevicePixels>,
}

impl LanguageModelImage {
    /// Resolves an image source into an LLM-ready format (base64)
    pub async fn resolve_source(source: ImageSource, cx: &mut AppContext) -> Option<Self> {
        let data = source.data(cx).await?;

        let size = data.size(0);
        // TODO: These are raw BGRA bytes, we need them in a specific format...
        let bytes = data.as_bytes(0).unwrap_or(&[]);

        Some(LanguageModelImage {
            source: base64::engine::general_purpose::STANDARD.encode(bytes),
            size,
        })
    }

    pub fn tokens(&self) -> i32 {
        // From: https://docs.anthropic.com/en/docs/build-with-claude/vision#calculate-image-costs
        // Note that are a lot of conditions on anthropic's API, and OpenAI doesn't use this,
        // so this method is more of a rough guess
        (self.size.width.raw() * self.size.height.raw()) / 750
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum LanguageModelRequestMessageContent {
    String(String),
    Image(LanguageModelImage),
}

impl LanguageModelRequestMessageContent {
    pub fn as_string(&self) -> &str {
        match self {
            LanguageModelRequestMessageContent::String(s) => s.as_str(),
            LanguageModelRequestMessageContent::Image(_) => "",
        }
    }
}

impl From<String> for LanguageModelRequestMessageContent {
    fn from(value: String) -> Self {
        LanguageModelRequestMessageContent::String(value)
    }
}

impl From<&str> for LanguageModelRequestMessageContent {
    fn from(value: &str) -> Self {
        LanguageModelRequestMessageContent::String(value.to_string())
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct LanguageModelRequestMessage {
    pub role: Role,
    pub content: Vec<LanguageModelRequestMessageContent>,
}

impl LanguageModelRequestMessage {
    pub fn string_contents(&self) -> String {
        let mut string_buffer = String::new();
        for string in self.content.iter().filter_map(|content| match content {
            LanguageModelRequestMessageContent::String(s) => Some(s),
            LanguageModelRequestMessageContent::Image(_) => None,
        }) {
            string_buffer.push_str(string.as_str())
        }
        string_buffer
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
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
                        content: msg.string_contents(),
                    },
                    Role::Assistant => open_ai::RequestMessage::Assistant {
                        content: Some(msg.string_contents()),
                        tool_calls: Vec::new(),
                    },
                    Role::System => open_ai::RequestMessage::System {
                        content: msg.string_contents(),
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
                        text: msg.string_contents(),
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
                            // TODO: is this append done properly?
                            last_message
                                .content
                                .push(LanguageModelRequestMessageContent::String(format!(
                                    "\n\n{}",
                                    message.string_contents()
                                )));
                            continue;
                        }
                    }

                    new_messages.push(message);
                }
                Role::System => {
                    if !system_message.is_empty() {
                        system_message.push_str("\n\n");
                    }
                    system_message.push_str(&message.string_contents());
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
                            text: message.string_contents(),
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
