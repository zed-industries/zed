use super::{SlashCommand, SlashCommandOutput};
use anyhow::Result;
use gpui::{AppContext, RenderOnce, Task, WeakView};
use language::LspAdapterDelegate;
use semantic_index::SemanticIndex;
use std::sync::{atomic::AtomicBool, Arc};
use ui::{prelude::*, ButtonLike, ElevationIndex, Icon, IconName};
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
                    project_index.search(argument, 5, cx)
                })?
                .await?;

            let mut output = String::new();
            for result in results {
                let content = result
                    .worktree
                    .read_with(&cx, |worktree, _cx| {
                        let entry_abs_path = worktree.abs_path().join(&result.path);
                        async {
                            let entry_abs_path = entry_abs_path;
                            fs.load(&entry_abs_path).await
                        }
                    })?
                    .await?;

                let range_start = result.range.start.min(content.len());
                let range_end = result.range.end.min(content.len());

                let start_line = content[0..range_start].matches('\n').count() + 1;
                let end_line = content[0..range_end].matches('\n').count() + 1;
                let start_line_byte_offset = content[0..range_start]
                    .rfind('\n')
                    .map(|pos| pos + 1)
                    .unwrap_or_default();
                let end_line_byte_offset = content[range_end..]
                    .find('\n')
                    .map(|pos| range_end + pos)
                    .unwrap_or_else(|| content.len());
                output.push_str(&format!(
                    "```{}:{}-{}\n",
                    result.path.display(),
                    start_line,
                    end_line,
                ));
                output.push_str(&content[start_line_byte_offset..end_line_byte_offset]);
                output.push_str("\n```\n\n");
            }

            Ok(SlashCommandOutput {
                text: output,
                render_placeholder: Arc::new(move |id, unfold, _cx| {
                    ButtonLike::new(id)
                        .style(ButtonStyle::Filled)
                        .layer(ElevationIndex::ElevatedSurface)
                        .child(Icon::new(IconName::MagnifyingGlass))
                        .child(Label::new("Search Results"))
                        .on_click(move |_, cx| unfold(cx))
                        .into_any_element()
                }),
            })
        })
    }
}
