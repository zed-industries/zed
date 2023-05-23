use std::io;
use std::rc::Rc;

use anyhow::{anyhow, Result};
use editor::Editor;
use futures::AsyncBufReadExt;
use futures::{io::BufReader, AsyncReadExt, Stream, StreamExt};
use gpui::executor::Foreground;
use gpui::{actions, AppContext, Task, ViewContext};
use isahc::prelude::*;
use isahc::{http::StatusCode, Request};
use pulldown_cmark::{Event, HeadingLevel, Parser, Tag};
use serde::{Deserialize, Serialize};
use util::ResultExt;

actions!(ai, [Assist]);

// Data types for chat completion requests
#[derive(Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<RequestMessage>,
    stream: bool,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct RequestMessage {
    role: Role,
    content: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct ResponseMessage {
    role: Option<Role>,
    content: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
enum Role {
    User,
    Assistant,
    System,
}

#[derive(Deserialize, Debug)]
struct OpenAIResponseStreamEvent {
    pub id: Option<String>,
    pub object: String,
    pub created: u32,
    pub model: String,
    pub choices: Vec<ChatChoiceDelta>,
    pub usage: Option<Usage>,
}

#[derive(Deserialize, Debug)]
struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Deserialize, Debug)]
struct ChatChoiceDelta {
    pub index: u32,
    pub delta: ResponseMessage,
    pub finish_reason: Option<String>,
}

#[derive(Deserialize, Debug)]
struct OpenAIUsage {
    prompt_tokens: u64,
    completion_tokens: u64,
    total_tokens: u64,
}

#[derive(Deserialize, Debug)]
struct OpenAIChoice {
    text: String,
    index: u32,
    logprobs: Option<serde_json::Value>,
    finish_reason: Option<String>,
}

pub fn init(cx: &mut AppContext) {
    cx.add_async_action(assist)
}

fn assist(
    editor: &mut Editor,
    _: &Assist,
    cx: &mut ViewContext<Editor>,
) -> Option<Task<Result<()>>> {
    let api_key = std::env::var("OPENAI_API_KEY").log_err()?;

    let markdown = editor.text(cx);
    let prompt = parse_dialog(&markdown);
    let response = stream_completion(api_key, prompt, cx.foreground().clone());

    let range = editor.buffer().update(cx, |buffer, cx| {
        let snapshot = buffer.snapshot(cx);
        let chars = snapshot.reversed_chars_at(snapshot.len());
        let trailing_newlines = chars.take(2).take_while(|c| *c == '\n').count();
        let suffix = "\n".repeat(2 - trailing_newlines);
        let end = snapshot.len();
        buffer.edit([(end..end, suffix.clone())], None, cx);
        let snapshot = buffer.snapshot(cx);
        let start = snapshot.anchor_before(snapshot.len());
        let end = snapshot.anchor_after(snapshot.len());
        start..end
    });
    let buffer = editor.buffer().clone();

    Some(cx.spawn(|_, mut cx| async move {
        let mut stream = response.await?;
        let mut message = String::new();
        while let Some(stream_event) = stream.next().await {
            if let Some(choice) = stream_event?.choices.first() {
                if let Some(content) = &choice.delta.content {
                    message.push_str(content);
                }
            }

            buffer.update(&mut cx, |buffer, cx| {
                buffer.edit([(range.clone(), message.clone())], None, cx);
            });
        }
        Ok(())
    }))
}

fn parse_dialog(markdown: &str) -> OpenAIRequest {
    let parser = Parser::new(markdown);
    let mut messages = Vec::new();

    let mut current_role: Option<Role> = None;
    let mut buffer = String::new();
    for event in parser {
        match event {
            Event::Start(Tag::Heading(HeadingLevel::H2, _, _)) => {
                if let Some(role) = current_role.take() {
                    if !buffer.is_empty() {
                        messages.push(RequestMessage {
                            role,
                            content: buffer.trim().to_string(),
                        });
                        buffer.clear();
                    }
                }
            }
            Event::Text(text) => {
                if current_role.is_some() {
                    buffer.push_str(&text);
                } else {
                    // Determine the current role based on the H2 header text
                    let text = text.to_lowercase();
                    current_role = if text.contains("user") {
                        Some(Role::User)
                    } else if text.contains("assistant") {
                        Some(Role::Assistant)
                    } else if text.contains("system") {
                        Some(Role::System)
                    } else {
                        None
                    };
                }
            }
            _ => (),
        }
    }
    if let Some(role) = current_role {
        messages.push(RequestMessage {
            role,
            content: buffer,
        });
    }

    OpenAIRequest {
        model: "gpt-4".into(),
        messages,
        stream: true,
    }
}

async fn stream_completion(
    api_key: String,
    mut request: OpenAIRequest,
    executor: Rc<Foreground>,
) -> Result<impl Stream<Item = Result<OpenAIResponseStreamEvent>>> {
    request.stream = true;

    let (tx, rx) = futures::channel::mpsc::unbounded::<Result<OpenAIResponseStreamEvent>>();

    let json_data = serde_json::to_string(&request)?;
    let mut response = Request::post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .body(json_data)?
        .send_async()
        .await?;

    let status = response.status();
    if status == StatusCode::OK {
        executor
            .spawn(async move {
                let mut lines = BufReader::new(response.body_mut()).lines();

                fn parse_line(
                    line: Result<String, io::Error>,
                ) -> Result<Option<OpenAIResponseStreamEvent>> {
                    if let Some(data) = line?.strip_prefix("data: ") {
                        let event = serde_json::from_str(&data)?;
                        Ok(Some(event))
                    } else {
                        Ok(None)
                    }
                }

                while let Some(line) = lines.next().await {
                    if let Some(event) = parse_line(line).transpose() {
                        tx.unbounded_send(event).log_err();
                    }
                }

                anyhow::Ok(())
            })
            .detach();

        Ok(rx)
    } else {
        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;

        Err(anyhow!(
            "Failed to connect to OpenAI API: {} {}",
            response.status(),
            body,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_dialog() {
        use unindent::Unindent;

        let test_input = r#"
            ## System
            Hey there, welcome to Zed!

            ## Assintant
            Thanks! I'm excited to be here. I have much to learn, but also much to teach, and I'm growing fast.
        "#.unindent();

        let expected_output = vec![
            RequestMessage {
                role: Role::User,
                content: "Hey there, welcome to Zed!".to_string(),
            },
            RequestMessage {
                role: Role::Assistant,
                content: "Thanks! I'm excited to be here. I have much to learn, but also much to teach, and I'm growing fast.".to_string(),
            },
        ];

        assert_eq!(parse_dialog(&test_input).messages, expected_output);
    }
}
