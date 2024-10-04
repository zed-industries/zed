use super::{
    create_label_for_command,
    file_command::{build_entry_output_section, codeblock_fence_for_path},
    SlashCommand, SlashCommandOutput,
};
use anyhow::Result;
use assistant_slash_command::{ArgumentCompletion, SlashCommandOutputSection};
use feature_flags::FeatureFlag;
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

pub(crate) enum SearchStyle {
    Dense,
    Hybrid,
    Sparse,
}

impl SlashCommand for SearchSlashCommand {
    fn name(&self) -> String {
        "search".into()
    }

    fn label(&self, cx: &AppContext) -> CodeLabel {
        create_label_for_command("search", &["--n", "--style {*hybrid,dense,sparse}"], cx)
    }

    fn description(&self) -> String {
        "semantic search".into()
    }

    fn menu_text(&self) -> String {
        "Semantic Search".into()
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
    ) -> Task<Result<SlashCommandOutput>> {
        let Some(workspace) = workspace.upgrade() else {
            return Task::ready(Err(anyhow::anyhow!("workspace was dropped")));
        };
        if arguments.is_empty() {
            return Task::ready(Err(anyhow::anyhow!("missing search query")));
        };

        let mut limit = None;
        let mut query = Vec::new();
        let mut arg_iter = arguments.iter();
        let mut style = SearchStyle::Hybrid;

        while let Some(arg) = arg_iter.next() {
            if arg == "--n" {
                if let Some(count) = arg_iter.next() {
                    if let Ok(parsed_count) = count.parse::<usize>() {
                        limit = Some(parsed_count);
                        continue;
                    } else {
                        return Task::ready(Err(anyhow::anyhow!(
                            "Invalid count for --n parameter; should be a positive integer."
                        )));
                    }
                } else {
                    return Task::ready(Err(anyhow::anyhow!("Missing count for --n parameter")));
                }
            } else if arg == "--style" {
                if let Some(style_value) = arg_iter.next() {
                    match style_value.as_str() {
                        "dense" => style = SearchStyle::Dense,
                        "sparse" => style = SearchStyle::Sparse,
                        "hybrid" => style = SearchStyle::Hybrid,
                        _ => {
                            return Task::ready(Err(anyhow::anyhow!(
                                "Invalid style parameter; should be 'dense', 'sparse', or 'hybrid'."
                            )))
                        }
                    }
                    continue;
                } else {
                    return Task::ready(Err(anyhow::anyhow!(
                        "Missing value for --style parameter"
                    )));
                }
            }
            query.push(arg.clone());
        }

        let query = query.join(" ");

        if query.is_empty() {
            return Task::ready(Err(anyhow::anyhow!("missing search query")));
        }

        let search_param = match style {
            SearchStyle::Dense => 1.0,
            SearchStyle::Sparse => 0.0,
            SearchStyle::Hybrid => 0.7,
        };

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
                    project_index.search(vec![query.clone()], limit.unwrap_or(5), search_param, cx)
                })?
                .await?;

            let loaded_results = SemanticDb::load_results(results, &fs, &cx).await?;

            let output = cx
                .background_executor()
                .spawn(async move {
                    let mut text = format!("Search results for {query}:\n");
                    let mut sections = Vec::new();
                    for loaded_result in &loaded_results {
                        add_search_result_section(loaded_result, &mut text, &mut sections);
                    }

                    let query = SharedString::from(query);
                    sections.push(SlashCommandOutputSection {
                        range: 0..text.len(),
                        icon: IconName::MagnifyingGlass,
                        label: query,
                        metadata: None,
                    });

                    SlashCommandOutput {
                        text,
                        sections,
                        run_commands_in_text: false,
                    }
                })
                .await;

            Ok(output)
        })
    }
}

pub fn add_search_result_section(
    loaded_result: &LoadedSearchResult,
    text: &mut String,
    sections: &mut Vec<SlashCommandOutputSection<usize>>,
) {
    let LoadedSearchResult {
        path,
        full_path,
        excerpt_content,
        row_range,
        ..
    } = loaded_result;
    let section_start_ix = text.len();
    text.push_str(&codeblock_fence_for_path(
        Some(&path),
        Some(row_range.clone()),
    ));

    text.push_str(&excerpt_content);
    if !text.ends_with('\n') {
        text.push('\n');
    }
    writeln!(text, "```\n").unwrap();
    let section_end_ix = text.len() - 1;
    sections.push(build_entry_output_section(
        section_start_ix..section_end_ix,
        Some(&full_path),
        false,
        Some(row_range.start() + 1..row_range.end() + 1),
    ));
}
