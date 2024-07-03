use super::{SlashCommand, SlashCommandOutput};
use crate::{CompletionProvider, LanguageModelRequest, LanguageModelRequestMessage, Role};
use anyhow::anyhow;
use anyhow::Result;
use futures::FutureExt;
use futures::StreamExt;
use gpui::{AppContext, Task, WeakView};
use language::LspAdapterDelegate;
use std::sync::{atomic::AtomicBool, Arc};
use ui::WindowContext;
use workspace::Workspace;

pub(crate) struct AutoCommand;

impl SlashCommand for AutoCommand {
    fn name(&self) -> String {
        "auto".into()
    }

    fn description(&self) -> String {
        "Automatically infer what context to add, based on your prompt".into()
    }

    fn menu_text(&self) -> String {
        "Automatically Infer Context".into()
    }

    fn requires_argument(&self) -> bool {
        false
    }

    fn complete_argument(
        self: Arc<Self>,
        _query: String,
        _cancel: Arc<AtomicBool>,
        _workspace: Option<WeakView<Workspace>>,
        _cx: &mut AppContext,
    ) -> Task<Result<Vec<String>>> {
        Task::ready(Err(anyhow!("this command does not require argument")))
    }

    fn run(
        self: Arc<Self>,
        _argument: Option<&str>,
        _workspace: WeakView<Workspace>,
        _delegate: Arc<dyn LspAdapterDelegate>,
        cx: &mut WindowContext,
    ) -> Task<Result<SlashCommandOutput>> {
        let request = LanguageModelRequest {
            model: CompletionProvider::global(cx).model(),
            messages: vec![LanguageModelRequestMessage {
                role: Role::User,
                content: "please tell me a story".into(),
            }],
            stop: vec![],
            temperature: 1.0,
        };

        let stream = CompletionProvider::global(cx).complete(request);

        cx.spawn(|_cx| async move {
            let stream_completion = async {
                let mut messages = stream.await?;

                while let Some(message) = messages.next().await {
                    let text = message?;

                    dbg!(&text);

                    smol::future::yield_now().await;
                }

                anyhow::Ok(())
            };

            let result = stream_completion.await;

            dbg!(&result);
        });

        Task::ready(Ok(SlashCommandOutput::default()))
    }
}
