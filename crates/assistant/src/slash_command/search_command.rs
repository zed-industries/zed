use super::{
    create_label_for_command,
    file_command::{build_entry_output_section, codeblock_fence_for_path},
    SlashCommand, SlashCommandOutput,
};
use anyhow::Result;
use assistant_slash_command::{ArgumentCompletion, SlashCommandOutputSection};
use feature_flags::FeatureFlag;
use gpui::{AppContext, Task, WeakView};
use language::{CodeLabel, LineEnding, LspAdapterDelegate};
use semantic_index::SemanticIndex;
use std::{
    fmt::Write,
    path::PathBuf,
    sync::{atomic::AtomicBool, Arc},
};
use ui::{prelude::*, IconName};
use util::ResultExt;
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
        let project_index =
            cx.update_global(|index: &mut SemanticIndex, cx| index.project_index(project, cx));

        cx.spawn(|cx| async move {
            let results = project_index
                .read_with(&cx, |project_index, cx| {
                    project_index.search(query.clone(), limit.unwrap_or(5), cx)
                })?
                .await?;

            let mut loaded_results = Vec::new();
            for result in results {
                let (full_path, file_content) =
                    result.worktree.read_with(&cx, |worktree, _cx| {
                        let entry_abs_path = worktree.abs_path().join(&result.path);
                        let mut entry_full_path = PathBuf::from(worktree.root_name());
                        entry_full_path.push(&result.path);
                        let file_content = async {
                            let entry_abs_path = entry_abs_path;
                            fs.load(&entry_abs_path).await
                        };
                        (entry_full_path, file_content)
                    })?;
                if let Some(file_content) = file_content.await.log_err() {
                    loaded_results.push((result, full_path, file_content));
                }
            }

            let output = cx
                .background_executor()
                .spawn(async move {
                    let mut text = format!("Search results for {query}:\n");
                    let mut sections = Vec::new();
                    for (result, full_path, file_content) in loaded_results {
                        let range_start = result.range.start.min(file_content.len());
                        let range_end = result.range.end.min(file_content.len());

                        let start_row = file_content[0..range_start].matches('\n').count() as u32;
                        let end_row = file_content[0..range_end].matches('\n').count() as u32;
                        let start_line_byte_offset = file_content[0..range_start]
                            .rfind('\n')
                            .map(|pos| pos + 1)
                            .unwrap_or_default();
                        let end_line_byte_offset = file_content[range_end..]
                            .find('\n')
                            .map(|pos| range_end + pos)
                            .unwrap_or_else(|| file_content.len());

                        let section_start_ix = text.len();
                        text.push_str(&codeblock_fence_for_path(
                            Some(&result.path),
                            Some(start_row..end_row),
                        ));

                        let mut excerpt =
                            file_content[start_line_byte_offset..end_line_byte_offset].to_string();
                        LineEnding::normalize(&mut excerpt);
                        text.push_str(&excerpt);
                        writeln!(text, "\n```\n").unwrap();
                        let section_end_ix = text.len() - 1;
                        sections.push(build_entry_output_section(
                            section_start_ix..section_end_ix,
                            Some(&full_path),
                            false,
                            Some(start_row + 1..end_row + 1),
                        ));
                    }

                    let query = SharedString::from(query);
                    sections.push(SlashCommandOutputSection {
                        range: 0..text.len(),
                        icon: IconName::MagnifyingGlass,
                        label: query,
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
