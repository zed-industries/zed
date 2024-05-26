use super::{SlashCommand, SlashCommandCleanup, SlashCommandInvocation};
use crate::prompts::prompt_library::PromptLibrary;
use anyhow::{anyhow, Context, Result};
use futures::channel::oneshot;
use fuzzy::StringMatchCandidate;
use gpui::{AppContext, Task};
use language::LspAdapterDelegate;
use std::sync::{atomic::AtomicBool, Arc};

pub(crate) struct PromptSlashCommand {
    library: Arc<PromptLibrary>,
}

impl PromptSlashCommand {
    pub fn new(library: Arc<PromptLibrary>) -> Self {
        Self { library }
    }
}

impl SlashCommand for PromptSlashCommand {
    fn name(&self) -> String {
        "prompt".into()
    }

    fn description(&self) -> String {
        "insert a prompt from the library".into()
    }

    fn requires_argument(&self) -> bool {
        true
    }

    fn complete_argument(
        &self,
        query: String,
        cancellation_flag: Arc<AtomicBool>,
        cx: &mut AppContext,
    ) -> Task<Result<Vec<String>>> {
        let library = self.library.clone();
        let executor = cx.background_executor().clone();
        cx.background_executor().spawn(async move {
            let candidates = library
                .prompts()
                .into_iter()
                .enumerate()
                .map(|(ix, prompt)| StringMatchCandidate::new(ix, prompt.1.title().to_string()))
                .collect::<Vec<_>>();
            let matches = fuzzy::match_strings(
                &candidates,
                &query,
                false,
                100,
                &cancellation_flag,
                executor,
            )
            .await;
            Ok(matches
                .into_iter()
                .map(|mat| candidates[mat.candidate_id].string.clone())
                .collect())
        })
    }

    fn run(
        self: Arc<Self>,
        title: Option<&str>,
        _delegate: Arc<dyn LspAdapterDelegate>,
        cx: &mut AppContext,
    ) -> SlashCommandInvocation {
        let Some(title) = title else {
            return SlashCommandInvocation {
                output: Task::ready(Err(anyhow!("missing prompt name"))),
                invalidated: oneshot::channel().1,
                cleanup: SlashCommandCleanup::default(),
            };
        };

        let library = self.library.clone();
        let title = title.to_string();
        let output = cx.background_executor().spawn(async move {
            let prompt = library
                .prompts()
                .into_iter()
                .find(|prompt| &prompt.1.title().to_string() == &title)
                .with_context(|| format!("no prompt found with title {:?}", title))?
                .1;
            Ok(prompt.body())
        });
        SlashCommandInvocation {
            output,
            invalidated: oneshot::channel().1,
            cleanup: SlashCommandCleanup::default(),
        }
    }
}
