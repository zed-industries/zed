use futures::channel::oneshot;
use gpui::{Context, Task};
use language_model::{
    LanguageModel, LanguageModelCompletionEvent, LanguageModelRequest, LanguageModelRequestMessage,
    MessageContent, Role,
};
use smol::stream::StreamExt;
use std::{future::Future, sync::Arc};
use util::ResultExt;

pub struct ThreadMessage {
    pub role: Role,
    pub content: Vec<MessageContent>,
}

pub struct Thread {
    messages: Vec<ThreadMessage>,
    streaming_completion: Option<Task<Option<()>>>,
}

impl Thread {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            streaming_completion: None,
        }
    }

    pub fn push_user_message(&mut self, text: impl Into<String>, cx: &mut Context<Self>) {
        self.messages.push(ThreadMessage {
            role: Role::User,
            content: vec![MessageContent::Text(text.into())],
        });

        cx.notify();
    }

    pub fn stream_completion(
        &mut self,
        model: Arc<dyn LanguageModel>,
        cx: &mut Context<Self>,
    ) -> impl Future<Output = ()> {
        let request = self.to_completion_request();
        let (done_tx, done_rx) = futures::channel::oneshot::channel();
        let mut done_tx = Some(done_tx);
        self.streaming_completion = Some(
            cx.spawn(async move |thread, cx| {
                let mut events = model.stream_completion(request, cx).await?;

                while let Some(event) = events.next().await {
                    if let Some(event) = event.log_err() {
                        thread
                            .update(cx, |thread, cx| {
                                thread.handle_streamed_event(event, &mut done_tx, cx)
                            })
                            .ok();
                    }
                }

                anyhow::Ok(())
            })
            .log_err_in_task(cx),
        );

        cx.notify();
        async move {
            done_rx.await.ok();
        }
    }

    fn to_completion_request(&self) -> LanguageModelRequest {
        LanguageModelRequest {
            thread_id: None,
            prompt_id: None,
            messages: self
                .messages
                .iter()
                .map(|message| LanguageModelRequestMessage {
                    role: message.role,
                    content: message.content.clone(),
                    cache: false,
                })
                .collect(),
            tools: Vec::new(),
            stop: Vec::new(),
            temperature: None,
        }
    }

    fn handle_streamed_event(
        &mut self,
        event: LanguageModelCompletionEvent,
        done_tx: &mut Option<oneshot::Sender<()>>,
        cx: &mut Context<Self>,
    ) {
        use LanguageModelCompletionEvent::*;

        match event {
            Stop(stop_reason) => {
                done_tx.take().map(|tx| tx.send(()));
            }
            Text(new_text) => {
                if let Some(last_message) = self.messages.last_mut() {
                    debug_assert!(last_message.role == Role::Assistant);
                    if let Some(MessageContent::Text(text)) = last_message.content.last_mut() {
                        text.push_str(&new_text);
                    } else {
                        last_message.content.push(MessageContent::Text(new_text));
                    }

                    cx.notify();
                } else {
                    todo!("does this happen in practice?")
                }
            }
            Thinking { text, signature } => {
                dbg!(text, signature);
            }
            ToolUse(language_model_tool_use) => {
                dbg!(language_model_tool_use);
            }
            StartMessage { message_id, role } => {
                dbg!(message_id, role);

                self.messages.push(ThreadMessage {
                    role,
                    content: Vec::new(),
                });
            }
            UsageUpdate(token_usage) => {
                dbg!(token_usage);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use client::{Client, UserStore};
    use fs::FakeFs;
    use gpui::{App, AppContext, TestAppContext};
    use language_model::LanguageModelRegistry;
    use reqwest_client::ReqwestClient;

    #[gpui::test]
    async fn test_basic_threads(cx: &mut TestAppContext) {
        let model = init_test(cx).await;
        let thread = cx.new(|_cx| Thread::new());

        thread
            .update(cx, |thread, cx| {
                thread.push_user_message("Testing: Reply with 'Hello'", cx);
                thread.stream_completion(model, cx)
            })
            .await;

        thread.update(cx, |thread, _cx| {
            assert_eq!(
                thread.messages.last().unwrap().content,
                vec![MessageContent::Text("Hello".to_string())]
            );
        });
    }

    fn init_test(cx: &mut TestAppContext) -> Task<Arc<dyn LanguageModel>> {
        cx.executor().allow_parking();
        cx.update(|cx| {
            gpui_tokio::init(cx);
            let http_client = ReqwestClient::user_agent("agent thread tests").unwrap();
            cx.set_http_client(Arc::new(http_client));

            settings::init(cx);
            client::init_settings(cx);
            let client = Client::production(cx);
            let user_store = cx.new(|cx| UserStore::new(client.clone(), cx));
            let fs = FakeFs::new(cx.background_executor().clone());
            language_model::init(client.clone(), cx);
            language_models::init(user_store.clone(), client.clone(), fs.clone(), cx);

            let registry = LanguageModelRegistry::read_global(cx);
            let model = registry
                .available_models(cx)
                .find(|model| model.id().0 == "claude-3-7-sonnet-latest")
                .unwrap();

            let provider = registry.provider(&model.provider_id()).unwrap();
            let authenticated = provider.authenticate(cx);

            cx.spawn(async move |_cx| {
                authenticated.await.unwrap();
                model
            })
        })
    }
}
