use std::io::{Cursor, Write};

use crate::role::Role;
use base64::write::EncoderWriter;
use gpui::{point, size, AppContext, DevicePixels, Image, ObjectFit, RenderImage, Size, Task};
use image::{codecs::png::PngEncoder, imageops::resize, DynamicImage, ImageDecoder};
use serde::{Deserialize, Serialize};
use ui::{px, SharedString};
use util::ResultExt;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug, Hash)]
pub struct LanguageModelImage {
    // A base64 encoded PNG image
    pub source: SharedString,
    size: Size<DevicePixels>,
}

const ANTHROPIC_SIZE_LIMT: f32 = 1568.0; // Anthropic wants uploaded images to be smaller than this in both dimensions

impl LanguageModelImage {
    pub fn from_image(data: Image, cx: &mut AppContext) -> Task<Option<Self>> {
        cx.background_executor().spawn(async move {
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

            // SAFETY: The base64 encoder should not produce non-UTF8
            let source = unsafe { String::from_utf8_unchecked(base64_image) };

            Some(LanguageModelImage {
                size: image_size,
                source: source.into(),
            })
        })
    }

    /// Resolves image into an LLM-ready format (base64)
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

        // SAFETY: The base64 encoder should not produce non-UTF8
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
        // Note that are a lot of conditions on anthropic's API, and OpenAI doesn't use this,
        // so this method is more of a rough guess
        (width * height) / 750
    }
}

#[derive(Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum MessageContent {
    Text(String),
    Image(LanguageModelImage),
}

impl std::fmt::Debug for MessageContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageContent::Text(t) => f.debug_struct("MessageContent").field("text", t).finish(),
            MessageContent::Image(i) => f
                .debug_struct("MessageContent")
                .field("image", &i.source.len())
                .finish(),
        }
    }
}

impl MessageContent {
    pub fn as_string(&self) -> &str {
        match self {
            MessageContent::Text(s) => s.as_str(),
            MessageContent::Image(_) => "",
        }
    }
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
            MessageContent::Text(s) => Some(s),
            MessageContent::Image(_) => None,
        }) {
            string_buffer.push_str(string.as_str())
        }
        string_buffer
    }

    pub fn contents_empty(&self) -> bool {
        self.content.is_empty()
            || self
                .content
                .get(0)
                .map(|content| match content {
                    MessageContent::Text(s) => s.trim().is_empty(),
                    MessageContent::Image(_) => true,
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
    pub temperature: f32,
}

impl LanguageModelRequest {
    pub fn into_open_ai(self, model: String, max_output_tokens: Option<u32>) -> open_ai::Request {
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
            max_tokens: max_output_tokens,
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

    pub fn into_anthropic(self, model: String, max_output_tokens: u32) -> anthropic::Request {
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
                            MessageContent::Text(t) if !t.is_empty() => {
                                Some(anthropic::RequestContent::Text {
                                    text: t,
                                    cache_control,
                                })
                            }
                            MessageContent::Image(i) => Some(anthropic::RequestContent::Image {
                                source: anthropic::ImageSource {
                                    source_type: "base64".to_string(),
                                    media_type: "image/png".to_string(),
                                    data: i.source.to_string(),
                                },
                                cache_control,
                            }),
                            _ => None,
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
