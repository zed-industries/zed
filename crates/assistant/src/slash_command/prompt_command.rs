use super::{SlashCommand, SlashCommandOutput};
use crate::prompts::PromptLibrary;
use anyhow::{anyhow, Context, Result};
use assistant_slash_command::SlashCommandOutputSection;
use fuzzy::StringMatchCandidate;
use gpui::{AppContext, Task, WeakView};
use language::LspAdapterDelegate;
use std::sync::{atomic::AtomicBool, Arc};
use ui::{prelude::*, ButtonLike, ElevationIndex};
use workspace::Workspace;

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
        "insert prompt from library".into()
    }

    fn menu_text(&self) -> String {
        "Insert Prompt from Library".into()
    }

    fn requires_argument(&self) -> bool {
        true
    }

    fn complete_argument(
        &self,
        query: String,
        cancellation_flag: Arc<AtomicBool>,
        _workspace: WeakView<Workspace>,
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
        _workspace: WeakView<Workspace>,
        _delegate: Arc<dyn LspAdapterDelegate>,
        cx: &mut WindowContext,
    ) -> Task<Result<SlashCommandOutput>> {
        let Some(title) = title else {
            return Task::ready(Err(anyhow!("missing prompt name")));
        };

        let library = self.library.clone();
        let title = SharedString::from(title.to_string());
        let prompt = cx.background_executor().spawn({
            let title = title.clone();
            async move {
                let prompt = library
                    .prompts()
                    .into_iter()
                    .map(|prompt| (prompt.1.title(), prompt))
                    .find(|(t, _)| t == &title)
                    .with_context(|| format!("no prompt found with title {:?}", title))?
                    .1;
                anyhow::Ok(prompt.1.body())
            }
        });
        cx.foreground_executor().spawn(async move {
            let prompt = prompt.await?;
            let range = 0..prompt.len();
            Ok(SlashCommandOutput {
                text: prompt,
                sections: vec![SlashCommandOutputSection {
                    range,
                    render_placeholder: Arc::new(move |id, unfold, _cx| {
                        ButtonLike::new(id)
                            .style(ButtonStyle::Filled)
                            .layer(ElevationIndex::ElevatedSurface)
                            .child(Icon::new(IconName::Library))
                            .child(Label::new(title.clone()))
                            .on_click(move |_, cx| unfold(cx))
                            .into_any_element()
                    }),
                }],
            })
        })
    }
}
