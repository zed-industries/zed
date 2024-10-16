use super::{codeblock_fence_for_path, create_label_for_command};
use anyhow::Result;
use assistant_slash_command::{
    ArgumentCompletion, SlashCommand, SlashCommandContentType, SlashCommandEvent,
    SlashCommandOutputSection, SlashCommandResult,
};
use feature_flags::FeatureFlag;
use futures::stream::{self, StreamExt};
use gpui::{AppContext, Task, WeakView};
use language::{CodeLabel, LspAdapterDelegate};
use semantic_index::{LoadedSearchResult, SemanticDb};
use std::{
    fmt::Write,
    sync::{atomic::AtomicBool, Arc},
};
use ui::{prelude::*, IconName};
use workspace::Workspace;

pub(crate) struct SearchSlashCommandFeatureFlag;

impl FeatureFlag for SearchSlashCommandFeatureFlag {
    const NAME: &'static str = "search-slash-command";
}

pub(crate) struct SearchSlashCommand;

impl SlashCommand for SearchSlashCommand {
    fn name(&self) -> String {
        "search".into()
    }

    fn label(&self, cx: &AppContext) -> CodeLabel {
        create_label_for_command("search", &["--n"], cx)
    }

    fn description(&self) -> String {
        "Search your project semantically".into()
    }

    fn menu_text(&self) -> String {
        self.description()
    }

    fn requires_argument(&self) -> bool {
        true
    }

    fn complete_argument(
        self: Arc<Self>,
        _arguments: &[String],
        _cancel: Arc<AtomicBool>,
        _workspace: Option<WeakView<Workspace>>,
        _cx: &mut WindowContext,
    ) -> Task<Result<Vec<ArgumentCompletion>>> {
        Task::ready(Ok(Vec::new()))
    }

    fn run(
        self: Arc<Self>,
        arguments: &[String],
        _context_slash_command_output_sections: &[SlashCommandOutputSection<language::Anchor>],
        _context_buffer: language::BufferSnapshot,
        workspace: WeakView<Workspace>,
        _delegate: Option<Arc<dyn LspAdapterDelegate>>,
        cx: &mut WindowContext,
    ) -> Task<SlashCommandResult> {
        let Some(workspace) = workspace.upgrade() else {
            return Task::ready(Err(anyhow::anyhow!("workspace was dropped")));
        };
        if arguments.is_empty() {
            return Task::ready(Err(anyhow::anyhow!("missing search query")));
        };

        let mut limit = None;
        let mut query = String::new();
        for part in arguments {
            if let Some(parameter) = part.strip_prefix("--") {
                if let Ok(count) = parameter.parse::<usize>() {
                    limit = Some(count);
                    continue;
                }
            }

            query.push_str(part);
            query.push(' ');
        }
        query.pop();

        if query.is_empty() {
            return Task::ready(Err(anyhow::anyhow!("missing search query")));
        }

        let project = workspace.read(cx).project().clone();
        let fs = project.read(cx).fs().clone();
        let Some(project_index) =
            cx.update_global(|index: &mut SemanticDb, cx| index.project_index(project, cx))
        else {
            return Task::ready(Err(anyhow::anyhow!("no project indexer")));
        };

        cx.spawn(|cx| async move {
            let results = project_index
                .read_with(&cx, |project_index, cx| {
                    project_index.search(vec![query.clone()], limit.unwrap_or(5), cx)
                })?
                .await?;

            let loaded_results = SemanticDb::load_results(results, &fs, &cx).await?;

            cx.background_executor()
                .spawn(async move {
                    let mut events = Vec::new();
                    events.push(SlashCommandEvent::StartSection {
                        icon: IconName::MagnifyingGlass,
                        label: SharedString::from(format!("Search results for {query}:")),
                        metadata: None,
                    });

                    for loaded_result in loaded_results {
                        add_search_result_section(&loaded_result, &mut events);
                    }

                    events.push(SlashCommandEvent::EndSection { metadata: None });

                    Ok(stream::iter(events).boxed())
                })
                .await
        })
    }
}

pub fn add_search_result_section(
    loaded_result: &LoadedSearchResult,
    events: &mut Vec<SlashCommandEvent>,
) {
    let LoadedSearchResult {
        path,
        excerpt_content,
        row_range,
        ..
    } = loaded_result;

    let mut text = codeblock_fence_for_path(Some(&path), Some(row_range.clone()));
    text.push_str(&excerpt_content);
    if !text.ends_with('\n') {
        text.push('\n');
    }
    writeln!(text, "```\n").unwrap();
    let path_str = path.to_string_lossy().to_string();
    events.push(SlashCommandEvent::StartSection {
        icon: IconName::File,
        label: path_str.into(),
        metadata: None,
    });
    events.push(SlashCommandEvent::Content(SlashCommandContentType::Text {
        text,
        run_commands_in_text: false,
    }));
    events.push(SlashCommandEvent::EndSection { metadata: None });
}
