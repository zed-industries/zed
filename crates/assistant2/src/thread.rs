use std::sync::Arc;

use gpui::{ModelContext, Task};
use language_model::{
    LanguageModel, LanguageModelCompletionEvent, LanguageModelRequest, Role, StopReason,
};
use smol::stream::StreamExt as _;
use util::ResultExt as _;

/// A message in a [`Thread`].
pub struct Message {
    pub role: Role,
    pub text: String,
}

/// A thread of conversation with the LLM.
pub struct Thread {
    pub messages: Vec<Message>,
    pub pending_completion_tasks: Vec<Task<()>>,
}

impl Thread {
    pub fn new(_cx: &mut ModelContext<Self>) -> Self {
        Self {
            messages: Vec::new(),
            pending_completion_tasks: Vec::new(),
        }
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

                let mut text = String::new();

                while let Some(event) = events.next().await {
                    let event = event?;
                    match event {
                        LanguageModelCompletionEvent::StartMessage { .. } => {}
                        LanguageModelCompletionEvent::Stop(reason) => {
                            stop_reason = reason;
                        }
                        LanguageModelCompletionEvent::Text(chunk) => {
                            text.push_str(&chunk);
                        }
                        LanguageModelCompletionEvent::ToolUse(_tool_use) => {}
                    }

                    smol::future::yield_now().await;
                }

                anyhow::Ok((stop_reason, text))
            };

            let result = stream_completion.await;

            this.update(&mut cx, |thread, _cx| {
                if let Some((_stop_reason, text)) = result.log_err() {
                    thread.messages.push(Message {
                        role: Role::Assistant,
                        text,
                    });
                }
            })
            .ok();
        });

        self.pending_completion_tasks.push(task);
    }
}
