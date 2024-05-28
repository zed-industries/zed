use super::{file_command::FilePlaceholder, SlashCommand, SlashCommandOutput};
use anyhow::Result;
use assistant_slash_command::SlashCommandOutputSection;
use gpui::{AppContext, Task, WeakView};
use language::{LineEnding, LspAdapterDelegate};
use semantic_index::SemanticIndex;
use std::{
    fmt::Write,
    path::PathBuf,
    sync::{atomic::AtomicBool, Arc},
};
use ui::{prelude::*, ButtonLike, ElevationIndex, Icon, IconName};
use util::ResultExt;
use workspace::Workspace;

pub(crate) struct SearchSlashCommand;

impl SlashCommand for SearchSlashCommand {
    fn name(&self) -> String {
        "search".into()
    }

    fn description(&self) -> String {
        "semantically search files".into()
    }

    fn tooltip_text(&self) -> String {
        "search".into()
    }

    fn requires_argument(&self) -> bool {
        true
    }

    fn complete_argument(
        &self,
        _query: String,
        _cancel: Arc<AtomicBool>,
        _cx: &mut AppContext,
    ) -> Task<Result<Vec<String>>> {
        Task::ready(Ok(Vec::new()))
    }

    fn run(
        self: Arc<Self>,
        argument: Option<&str>,
        workspace: WeakView<Workspace>,
        _delegate: Arc<dyn LspAdapterDelegate>,
        cx: &mut WindowContext,
    ) -> Task<Result<SlashCommandOutput>> {
        let Some(workspace) = workspace.upgrade() else {
            return Task::ready(Err(anyhow::anyhow!("workspace was dropped")));
        };
        let Some(argument) = argument else {
            return Task::ready(Err(anyhow::anyhow!("missing search query")));
        };
        if argument.is_empty() {
            return Task::ready(Err(anyhow::anyhow!("missing search query")));
        }

        let project = workspace.read(cx).project().clone();
        let argument = argument.to_string();
        let fs = project.read(cx).fs().clone();
        let project_index =
            cx.update_global(|index: &mut SemanticIndex, cx| index.project_index(project, cx));

        cx.spawn(|cx| async move {
            let results = project_index
                .read_with(&cx, |project_index, cx| {
                    project_index.search(argument.clone(), 5, cx)
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
                    let mut text = format!("Search results for {argument}:\n");
                    let mut sections = Vec::new();
                    for (result, full_path, file_content) in loaded_results {
                        let range_start = result.range.start.min(file_content.len());
                        let range_end = result.range.end.min(file_content.len());

                        let start_line =
                            file_content[0..range_start].matches('\n').count() as u32 + 1;
                        let end_line = file_content[0..range_end].matches('\n').count() as u32 + 1;
                        let start_line_byte_offset = file_content[0..range_start]
                            .rfind('\n')
                            .map(|pos| pos + 1)
                            .unwrap_or_default();
                        let end_line_byte_offset = file_content[range_end..]
                            .find('\n')
                            .map(|pos| range_end + pos)
                            .unwrap_or_else(|| file_content.len());

                        let section_start_ix = text.len();
                        writeln!(
                            text,
                            "```{}:{}-{}",
                            result.path.display(),
                            start_line,
                            end_line,
                        )
                        .unwrap();
                        let mut excerpt =
                            file_content[start_line_byte_offset..end_line_byte_offset].to_string();
                        LineEnding::normalize(&mut excerpt);
                        text.push_str(&excerpt);
                        writeln!(text, "\n```\n").unwrap();
                        let section_end_ix = text.len() - 1;

                        sections.push(SlashCommandOutputSection {
                            range: section_start_ix..section_end_ix,
                            render_placeholder: Arc::new(move |id, unfold, _| {
                                FilePlaceholder {
                                    id,
                                    path: Some(full_path.clone()),
                                    line_range: Some(start_line..end_line),
                                    unfold,
                                }
                                .into_any_element()
                            }),
                        });
                    }

                    let argument = SharedString::from(argument);
                    sections.push(SlashCommandOutputSection {
                        range: 0..text.len(),
                        render_placeholder: Arc::new(move |id, unfold, _cx| {
                            ButtonLike::new(id)
                                .style(ButtonStyle::Filled)
                                .layer(ElevationIndex::ElevatedSurface)
                                .child(Icon::new(IconName::MagnifyingGlass))
                                .child(Label::new(argument.clone()))
                                .on_click(move |_, cx| unfold(cx))
                                .into_any_element()
                        }),
                    });

                    SlashCommandOutput { text, sections }
                })
                .await;

            Ok(output)
        })
    }
}
