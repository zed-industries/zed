use std::io::{Cursor, Write};

use crate::role::Role;
use crate::LanguageModelToolUse;
use base64::write::EncoderWriter;
use gpui::{
    point, size, App, AppContext as _, DevicePixels, Image, ObjectFit, RenderImage, Size, Task,
};
use image::{codecs::png::PngEncoder, imageops::resize, DynamicImage, ImageDecoder};
use serde::{Deserialize, Serialize};
use ui::{px, SharedString};
use util::ResultExt;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct LanguageModelImage {
    /// A base64-encoded PNG image.
    pub source: SharedString,
    size: Size<DevicePixels>,
}

impl std::fmt::Debug for LanguageModelImage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LanguageModelImage")
            .field("source", &format!("<{} bytes>", self.source.len()))
            .field("size", &self.size)
            .finish()
    }
}

/// Anthropic wants uploaded images to be smaller than this in both dimensions.
const ANTHROPIC_SIZE_LIMT: f32 = 1568.;

impl LanguageModelImage {
    pub fn from_image(data: Image, cx: &mut App) -> Task<Option<Self>> {
        cx.background_spawn(async move {
            match data.format() {
                gpui::ImageFormat::Png
                | gpui::ImageFormat::Jpeg
                | gpui::ImageFormat::Webp
                | gpui::ImageFormat::Gif => {}
                _ => return None,
            };

            let image = image::codecs::png::PngDecoder::new(Cursor::new(data.bytes())).log_err()?;
            let (width, height) = image.dimensions();
            let image_size = size(DevicePixels(width as i32), DevicePixels(height as i32));

            let mut base64_image = Vec::new();

            {
                let mut base64_encoder = EncoderWriter::new(
                    Cursor::new(&mut base64_image),
                    &base64::engine::general_purpose::STANDARD,
                );

                if image_size.width.0 > ANTHROPIC_SIZE_LIMT as i32
                    || image_size.height.0 > ANTHROPIC_SIZE_LIMT as i32
                {
                    let new_bounds = ObjectFit::ScaleDown.get_bounds(
                        gpui::Bounds {
                            origin: point(px(0.0), px(0.0)),
                            size: size(px(ANTHROPIC_SIZE_LIMT), px(ANTHROPIC_SIZE_LIMT)),
                        },
                        image_size,
                    );
                    let image = DynamicImage::from_decoder(image).log_err()?.resize(
                        new_bounds.size.width.0 as u32,
                        new_bounds.size.height.0 as u32,
                        image::imageops::FilterType::Triangle,
                    );

                    let mut png = Vec::new();
                    image
                        .write_with_encoder(PngEncoder::new(&mut png))
                        .log_err()?;

                    base64_encoder.write_all(png.as_slice()).log_err()?;
                } else {
                    base64_encoder.write_all(data.bytes()).log_err()?;
                }
            }

            // SAFETY: The base64 encoder should not produce non-UTF8.
            let source = unsafe { String::from_utf8_unchecked(base64_image) };

            Some(LanguageModelImage {
                size: image_size,
                source: source.into(),
            })
        })
    }

    /// Resolves image into an LLM-ready format (base64).
    pub fn from_render_image(data: &RenderImage) -> Option<Self> {
        let image_size = data.size(0);

        let mut bytes = data.as_bytes(0).unwrap_or(&[]).to_vec();
        // Convert from BGRA to RGBA.
        for pixel in bytes.chunks_exact_mut(4) {
            pixel.swap(2, 0);
        }
        let mut image = image::RgbaImage::from_vec(
            image_size.width.0 as u32,
            image_size.height.0 as u32,
            bytes,
        )
        .expect("We already know this works");

        // https://docs.anthropic.com/en/docs/build-with-claude/vision
        if image_size.width.0 > ANTHROPIC_SIZE_LIMT as i32
            || image_size.height.0 > ANTHROPIC_SIZE_LIMT as i32
        {
            let new_bounds = ObjectFit::ScaleDown.get_bounds(
                gpui::Bounds {
                    origin: point(px(0.0), px(0.0)),
                    size: size(px(ANTHROPIC_SIZE_LIMT), px(ANTHROPIC_SIZE_LIMT)),
                },
                image_size,
            );

            image = resize(
                &image,
                new_bounds.size.width.0 as u32,
                new_bounds.size.height.0 as u32,
                image::imageops::FilterType::Triangle,
            );
        }

        let mut png = Vec::new();

        image
            .write_with_encoder(PngEncoder::new(&mut png))
            .log_err()?;

        let mut base64_image = Vec::new();

        {
            let mut base64_encoder = EncoderWriter::new(
                Cursor::new(&mut base64_image),
                &base64::engine::general_purpose::STANDARD,
            );

            base64_encoder.write_all(png.as_slice()).log_err()?;
        }

        // SAFETY: The base64 encoder should not produce non-UTF8.
        let source = unsafe { String::from_utf8_unchecked(base64_image) };

        Some(LanguageModelImage {
            size: image_size,
            source: source.into(),
        })
    }

    pub fn estimate_tokens(&self) -> usize {
        let width = self.size.width.0.unsigned_abs() as usize;
        let height = self.size.height.0.unsigned_abs() as usize;

        // From: https://docs.anthropic.com/en/docs/build-with-claude/vision#calculate-image-costs
        // Note that are a lot of conditions on Anthropic's API, and OpenAI doesn't use this,
        // so this method is more of a rough guess.
        (width * height) / 750
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct LanguageModelToolResult {
    pub tool_use_id: String,
    pub is_error: bool,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum MessageContent {
    Text(String),
    Image(LanguageModelImage),
    ToolUse(LanguageModelToolUse),
    ToolResult(LanguageModelToolResult),
}

impl From<String> for MessageContent {
    fn from(value: String) -> Self {
        MessageContent::Text(value)
    }
}

impl From<&str> for MessageContent {
    fn from(value: &str) -> Self {
        MessageContent::Text(value.to_string())
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Hash)]
pub struct LanguageModelRequestMessage {
    pub role: Role,
    pub content: Vec<MessageContent>,
    pub cache: bool,
}

impl LanguageModelRequestMessage {
    pub fn string_contents(&self) -> String {
        let mut string_buffer = String::new();
        for string in self.content.iter().filter_map(|content| match content {
            MessageContent::Text(text) => Some(text),
            MessageContent::ToolResult(tool_result) => Some(&tool_result.content),
            MessageContent::ToolUse(_) | MessageContent::Image(_) => None,
        }) {
            string_buffer.push_str(string.as_str())
        }
        string_buffer
    }

    pub fn contents_empty(&self) -> bool {
        self.content.is_empty()
            || self
                .content
                .first()
                .map(|content| match content {
                    MessageContent::Text(text) => text.chars().all(|c| c.is_whitespace()),
                    MessageContent::ToolResult(tool_result) => {
                        tool_result.content.chars().all(|c| c.is_whitespace())
                    }
                    MessageContent::ToolUse(_) | MessageContent::Image(_) => true,
                })
                .unwrap_or(false)
    }
}

#[derive(Debug, PartialEq, Hash, Clone, Serialize, Deserialize)]
pub struct LanguageModelRequestTool {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct LanguageModelRequest {
    pub messages: Vec<LanguageModelRequestMessage>,
    pub tools: Vec<LanguageModelRequestTool>,
    pub stop: Vec<String>,
    pub temperature: Option<f32>,
}

impl LanguageModelRequest {
    pub fn into_open_ai(self, model: String, max_output_tokens: Option<u32>) -> open_ai::Request {
        let stream = !model.starts_with("o1-");
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
            stream,
            stop: self.stop,
            temperature: self.temperature.unwrap_or(1.0),
            max_tokens: max_output_tokens,
            tools: Vec::new(),
            tool_choice: None,
        }
    }

    pub fn into_mistral(self, model: String, max_output_tokens: Option<u32>) -> mistral::Request {
        let len = self.messages.len();
        let merged_messages =
            self.messages
                .into_iter()
                .fold(Vec::with_capacity(len), |mut acc, msg| {
                    let role = msg.role;
                    let content = msg.string_contents();

                    acc.push(match role {
                        Role::User => mistral::RequestMessage::User { content },
                        Role::Assistant => mistral::RequestMessage::Assistant {
                            content: Some(content),
                            tool_calls: Vec::new(),
                        },
                        Role::System => mistral::RequestMessage::System { content },
                    });
                    acc
                });

        mistral::Request {
            model,
            messages: merged_messages,
            stream: true,
            max_tokens: max_output_tokens,
            temperature: self.temperature,
            response_format: None,
            tools: self
                .tools
                .into_iter()
                .map(|tool| mistral::ToolDefinition::Function {
                    function: mistral::FunctionDefinition {
                        name: tool.name,
                        description: Some(tool.description),
                        parameters: Some(tool.input_schema),
                    },
                })
                .collect(),
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
                temperature: self.temperature.map(|t| t as f64).or(Some(1.0)),
                top_p: None,
                top_k: None,
            }),
            safety_settings: None,
        }
    }

    pub fn into_anthropic(
        self,
        model: String,
        default_temperature: f32,
        max_output_tokens: u32,
    ) -> anthropic::Request {
        let mut new_messages: Vec<anthropic::Message> = Vec::new();
        let mut system_message = String::new();

        for message in self.messages {
            if message.contents_empty() {
                continue;
            }

            match message.role {
                Role::User | Role::Assistant => {
                    let cache_control = if message.cache {
                        Some(anthropic::CacheControl {
                            cache_type: anthropic::CacheControlType::Ephemeral,
                        })
                    } else {
                        None
                    };
                    let anthropic_message_content: Vec<anthropic::RequestContent> = message
                        .content
                        .into_iter()
                        .filter_map(|content| match content {
                            MessageContent::Text(text) => {
                                if !text.is_empty() {
                                    Some(anthropic::RequestContent::Text {
                                        text,
                                        cache_control,
                                    })
                                } else {
                                    None
                                }
                            }
                            MessageContent::Image(image) => {
                                Some(anthropic::RequestContent::Image {
                                    source: anthropic::ImageSource {
                                        source_type: "base64".to_string(),
                                        media_type: "image/png".to_string(),
                                        data: image.source.to_string(),
                                    },
                                    cache_control,
                                })
                            }
                            MessageContent::ToolUse(tool_use) => {
                                Some(anthropic::RequestContent::ToolUse {
                                    id: tool_use.id.to_string(),
                                    name: tool_use.name,
                                    input: tool_use.input,
                                    cache_control,
                                })
                            }
                            MessageContent::ToolResult(tool_result) => {
                                Some(anthropic::RequestContent::ToolResult {
                                    tool_use_id: tool_result.tool_use_id,
                                    is_error: tool_result.is_error,
                                    content: tool_result.content,
                                    cache_control,
                                })
                            }
                        })
                        .collect();
                    let anthropic_role = match message.role {
                        Role::User => anthropic::Role::User,
                        Role::Assistant => anthropic::Role::Assistant,
                        Role::System => unreachable!("System role should never occur here"),
                    };
                    if let Some(last_message) = new_messages.last_mut() {
                        if last_message.role == anthropic_role {
                            last_message.content.extend(anthropic_message_content);
                            continue;
                        }
                    }
                    new_messages.push(anthropic::Message {
                        role: anthropic_role,
                        content: anthropic_message_content,
                    });
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
            messages: new_messages,
            max_tokens: max_output_tokens,
            system: Some(system_message),
            tools: self
                .tools
                .into_iter()
                .map(|tool| anthropic::Tool {
                    name: tool.name,
                    description: tool.description,
                    input_schema: tool.input_schema,
                })
                .collect(),
            tool_choice: None,
            metadata: None,
            stop_sequences: Vec::new(),
            temperature: self.temperature.or(Some(default_temperature)),
            top_k: None,
            top_p: None,
        }
    }

    pub fn into_deepseek(self, model: String, max_output_tokens: Option<u32>) -> deepseek::Request {
        let is_reasoner = model == "deepseek-reasoner";

        let len = self.messages.len();
        let merged_messages =
            self.messages
                .into_iter()
                .fold(Vec::with_capacity(len), |mut acc, msg| {
                    let role = msg.role;
                    let content = msg.string_contents();

                    if is_reasoner {
                        if let Some(last_msg) = acc.last_mut() {
                            match (last_msg, role) {
                                (deepseek::RequestMessage::User { content: last }, Role::User) => {
                                    last.push(' ');
                                    last.push_str(&content);
                                    return acc;
                                }

                                (
                                    deepseek::RequestMessage::Assistant {
                                        content: last_content,
                                        ..
                                    },
                                    Role::Assistant,
                                ) => {
                                    *last_content = last_content
                                        .take()
                                        .map(|c| {
                                            let mut s =
                                                String::with_capacity(c.len() + content.len() + 1);
                                            s.push_str(&c);
                                            s.push(' ');
                                            s.push_str(&content);
                                            s
                                        })
                                        .or(Some(content));

                                    return acc;
                                }
                                _ => {}
                            }
                        }
                    }

                    acc.push(match role {
                        Role::User => deepseek::RequestMessage::User { content },
                        Role::Assistant => deepseek::RequestMessage::Assistant {
                            content: Some(content),
                            tool_calls: Vec::new(),
                        },
                        Role::System => deepseek::RequestMessage::System { content },
                    });
                    acc
                });

        deepseek::Request {
            model,
            messages: merged_messages,
            stream: true,
            max_tokens: max_output_tokens,
            temperature: if is_reasoner { None } else { self.temperature },
            response_format: None,
            tools: self
                .tools
                .into_iter()
                .map(|tool| deepseek::ToolDefinition::Function {
                    function: deepseek::FunctionDefinition {
                        name: tool.name,
                        description: Some(tool.description),
                        parameters: Some(tool.input_schema),
                    },
                })
                .collect(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct LanguageModelResponseMessage {
    pub role: Option<Role>,
    pub content: Option<String>,
}
