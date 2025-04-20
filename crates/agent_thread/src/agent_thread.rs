use gpui::{Context, Task};
use std::{future::Future, sync::Arc};
use util::ResultExt;

use language_model::{
    LanguageModel, LanguageModelRequest, LanguageModelRequestMessage, MessageContent, Role,
};
use smol::stream::StreamExt;

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
        let request = self.build_completion_request();
        let (done_tx, done_rx) = futures::channel::oneshot::channel();
        let mut done_tx = Some(done_tx);
        self.streaming_completion = Some(
            cx.spawn(async move |thread, cx| {
                let mut events = model.stream_completion(request, cx).await?;

                while let Some(event) = events.next().await {
                    if let Some(event) = event.log_err() {
                        dbg!(&event);
                        match event {
                            language_model::LanguageModelCompletionEvent::Stop(stop_reason) => {
                                dbg!(stop_reason);
                                done_tx.take().map(|tx| tx.send(()));
                            }
                            language_model::LanguageModelCompletionEvent::Text(txt) => {
                                dbg!(txt);
                            }
                            language_model::LanguageModelCompletionEvent::Thinking {
                                text,
                                signature,
                            } => {
                                dbg!(text, signature);
                            }
                            language_model::LanguageModelCompletionEvent::ToolUse(
                                language_model_tool_use,
                            ) => {
                                dbg!(language_model_tool_use);
                            }
                            language_model::LanguageModelCompletionEvent::StartMessage {
                                message_id,
                                role,
                            } => {
                                dbg!(message_id, role);

                                thread
                                    .update(cx, |thread, cx| {
                                        thread.messages.push(ThreadMessage {
                                            role,
                                            content: Vec::new(),
                                        });
                                    })
                                    .ok();
                            }
                            language_model::LanguageModelCompletionEvent::UsageUpdate(
                                token_usage,
                            ) => {
                                dbg!(token_usage);
                            }
                        }
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

    fn build_completion_request(&self) -> LanguageModelRequest {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use client::{Client, UserStore};
    use fs::FakeFs;
    use gpui::{App, AppContext, TestAppContext};
    use language_model::LanguageModelRegistry;
    use language_models::AllLanguageModelSettings;
    use reqwest_client::ReqwestClient;
    use settings::Settings;

    #[gpui::test]
    async fn test_basic_threads(cx: &mut TestAppContext) {
        cx.executor().allow_parking();

        let model = cx.update(init_test).await;
        let thread = cx.new(|cx| Thread::new());

        let result = thread
            .update(cx, |thread, cx| {
                thread.push_user_message("Testing: Reply with 'Hello'", cx);
                thread.stream_completion(model, cx)
            })
            .await;

        dbg!(result);
    }

    fn init_test(cx: &mut App) -> Task<Arc<dyn LanguageModel>> {
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
    }
}
