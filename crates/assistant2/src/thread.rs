use std::sync::Arc;

use futures::StreamExt as _;
use gpui::{AppContext, EventEmitter, ModelContext, Task};
use language_model::{
    LanguageModel, LanguageModelCompletionEvent, LanguageModelRequest, LanguageModelRequestMessage,
    MessageContent, Role, StopReason,
};
use util::ResultExt as _;

#[derive(Debug, Clone, Copy)]
pub enum RequestKind {
    Chat,
}

/// A message in a [`Thread`].
pub struct Message {
    pub role: Role,
    pub text: String,
}

/// A thread of conversation with the LLM.
pub struct Thread {
    messages: Vec<Message>,
    pending_completion_tasks: Vec<Task<()>>,
}

impl Thread {
    pub fn new(_cx: &mut ModelContext<Self>) -> Self {
        Self {
            messages: Vec::new(),
            pending_completion_tasks: Vec::new(),
        }
    }

    pub fn messages(&self) -> impl Iterator<Item = &Message> {
        self.messages.iter()
    }

    pub fn insert_user_message(&mut self, text: impl Into<String>) {
        self.messages.push(Message {
            role: Role::User,
            text: text.into(),
        });
    }

    pub fn to_completion_request(
        &self,
        _request_kind: RequestKind,
        _cx: &AppContext,
    ) -> LanguageModelRequest {
        let mut request = LanguageModelRequest {
            messages: vec![],
            tools: Vec::new(),
            stop: Vec::new(),
            temperature: None,
        };

        for message in &self.messages {
            let mut request_message = LanguageModelRequestMessage {
                role: message.role,
                content: Vec::new(),
                cache: false,
            };

            request_message
                .content
                .push(MessageContent::Text(message.text.clone()));

            request.messages.push(request_message);
        }

        request
    }

    pub fn stream_completion(
        &mut self,
        request: LanguageModelRequest,
        model: Arc<dyn LanguageModel>,
        cx: &mut ModelContext<Self>,
    ) {
        let task = cx.spawn(|this, mut cx| async move {
            let stream = model.stream_completion(request, &cx);
            let stream_completion = async {
                let mut events = stream.await?;
                let mut stop_reason = StopReason::EndTurn;

                while let Some(event) = events.next().await {
                    let event = event?;

                    this.update(&mut cx, |thread, cx| {
                        match event {
                            LanguageModelCompletionEvent::StartMessage { .. } => {
                                thread.messages.push(Message {
                                    role: Role::Assistant,
                                    text: String::new(),
                                });
                            }
                            LanguageModelCompletionEvent::Stop(reason) => {
                                stop_reason = reason;
                            }
                            LanguageModelCompletionEvent::Text(chunk) => {
                                if let Some(last_message) = thread.messages.last_mut() {
                                    if last_message.role == Role::Assistant {
                                        last_message.text.push_str(&chunk);
                                    }
                                }
                            }
                            LanguageModelCompletionEvent::ToolUse(_tool_use) => {}
                        }

                        cx.emit(ThreadEvent::StreamedCompletion);
                        cx.notify();
                    })?;

                    smol::future::yield_now().await;
                }

                anyhow::Ok(stop_reason)
            };

            let result = stream_completion.await;
            let _ = result.log_err();
        });

        self.pending_completion_tasks.push(task);
    }
}

#[derive(Debug, Clone)]
pub enum ThreadEvent {
    StreamedCompletion,
}

impl EventEmitter<ThreadEvent> for Thread {}
