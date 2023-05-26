use anyhow::{anyhow, Result};
use assets::Assets;
use collections::HashMap;
use editor::Editor;
use futures::AsyncBufReadExt;
use futures::{io::BufReader, AsyncReadExt, Stream, StreamExt};
use gpui::executor::Background;
use gpui::{actions, AppContext, Task, ViewContext};
use isahc::prelude::*;
use isahc::{http::StatusCode, Request};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::fs;
use std::rc::Rc;
use std::{io, sync::Arc};
use util::channel::{ReleaseChannel, RELEASE_CHANNEL};
use util::{ResultExt, TryFutureExt};

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
    if *RELEASE_CHANNEL == ReleaseChannel::Stable {
        return;
    }

    let assistant = Rc::new(Assistant::default());
    cx.add_action({
        let assistant = assistant.clone();
        move |editor: &mut Editor, _: &Assist, cx: &mut ViewContext<Editor>| {
            assistant.assist(editor, cx).log_err();
        }
    });
    cx.capture_action({
        let assistant = assistant.clone();
        move |_: &mut Editor, _: &editor::Cancel, cx: &mut ViewContext<Editor>| {
            if !assistant.cancel_last_assist(cx.view_id()) {
                cx.propagate_action();
            }
        }
    });
}

type CompletionId = usize;

#[derive(Default)]
struct Assistant(RefCell<AssistantState>);

#[derive(Default)]
struct AssistantState {
    assist_stacks: HashMap<usize, Vec<(CompletionId, Task<Option<()>>)>>,
    next_completion_id: CompletionId,
}

impl Assistant {
    fn assist(self: &Rc<Self>, editor: &mut Editor, cx: &mut ViewContext<Editor>) -> Result<()> {
        let api_key = std::env::var("OPENAI_API_KEY")?;

        let selections = editor.selections.all(cx);
        let (user_message, insertion_site) = editor.buffer().update(cx, |buffer, cx| {
            // Insert markers around selected text as described in the system prompt above.
            let snapshot = buffer.snapshot(cx);
            let mut user_message = String::new();
            let mut user_message_suffix = String::new();
            let mut buffer_offset = 0;
            for selection in selections {
                if !selection.is_empty() {
                    if user_message_suffix.is_empty() {
                        user_message_suffix.push_str("\n\n");
                    }
                    user_message_suffix.push_str("[Selected excerpt from above]\n");
                    user_message_suffix
                        .extend(snapshot.text_for_range(selection.start..selection.end));
                    user_message_suffix.push_str("\n\n");
                }

                user_message.extend(snapshot.text_for_range(buffer_offset..selection.start));
                user_message.push_str("[SELECTION_START]");
                user_message.extend(snapshot.text_for_range(selection.start..selection.end));
                buffer_offset = selection.end;
                user_message.push_str("[SELECTION_END]");
            }
            if buffer_offset < snapshot.len() {
                user_message.extend(snapshot.text_for_range(buffer_offset..snapshot.len()));
            }
            user_message.push_str(&user_message_suffix);

            // Ensure the document ends with 4 trailing newlines.
            let trailing_newline_count = snapshot
                .reversed_chars_at(snapshot.len())
                .take_while(|c| *c == '\n')
                .take(4);
            let buffer_suffix = "\n".repeat(4 - trailing_newline_count.count());
            buffer.edit([(snapshot.len()..snapshot.len(), buffer_suffix)], None, cx);

            let snapshot = buffer.snapshot(cx); // Take a new snapshot after editing.
            let insertion_site = snapshot.anchor_after(snapshot.len() - 2);

            (user_message, insertion_site)
        });

        let this = self.clone();
        let buffer = editor.buffer().clone();
        let executor = cx.background_executor().clone();
        let editor_id = cx.view_id();
        let assist_id = util::post_inc(&mut self.0.borrow_mut().next_completion_id);
        let assist_task = cx.spawn(|_, mut cx| {
            async move {
                // TODO: We should have a get_string method on assets. This is repateated elsewhere.
                let content = Assets::get("contexts/system.zmd").unwrap();
                let mut system_message = std::str::from_utf8(content.data.as_ref())
                    .unwrap()
                    .to_string();

                if let Ok(custom_system_message_path) =
                    std::env::var("ZED_ASSISTANT_SYSTEM_PROMPT_PATH")
                {
                    system_message.push_str(
                        "\n\nAlso consider the following user-defined system prompt:\n\n",
                    );
                    // TODO: Replace this with our file system trait object.
                    system_message.push_str(
                        &cx.background()
                            .spawn(async move { fs::read_to_string(custom_system_message_path) })
                            .await?,
                    );
                }

                let stream = stream_completion(
                    api_key,
                    executor,
                    OpenAIRequest {
                        model: "gpt-4".to_string(),
                        messages: vec![
                            RequestMessage {
                                role: Role::System,
                                content: system_message.to_string(),
                            },
                            RequestMessage {
                                role: Role::User,
                                content: user_message,
                            },
                        ],
                        stream: false,
                    },
                );

                let mut messages = stream.await?;
                while let Some(message) = messages.next().await {
                    let mut message = message?;
                    if let Some(choice) = message.choices.pop() {
                        buffer.update(&mut cx, |buffer, cx| {
                            let text: Arc<str> = choice.delta.content?.into();
                            buffer.edit([(insertion_site.clone()..insertion_site, text)], None, cx);
                            Some(())
                        });
                    }
                }

                this.0
                    .borrow_mut()
                    .assist_stacks
                    .get_mut(&editor_id)
                    .unwrap()
                    .retain(|(id, _)| *id != assist_id);

                anyhow::Ok(())
            }
            .log_err()
        });

        self.0
            .borrow_mut()
            .assist_stacks
            .entry(cx.view_id())
            .or_default()
            .push((assist_id, assist_task));

        Ok(())
    }

    fn cancel_last_assist(self: &Rc<Self>, editor_id: usize) -> bool {
        self.0
            .borrow_mut()
            .assist_stacks
            .get_mut(&editor_id)
            .and_then(|assists| assists.pop())
            .is_some()
    }
}

async fn stream_completion(
    api_key: String,
    executor: Arc<Background>,
    mut request: OpenAIRequest,
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
