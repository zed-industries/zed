use super::{SlashCommand, SlashCommandOutput};
use anyhow::{anyhow, Result};
use assistant_slash_command::SlashCommandOutputSection;
use fuzzy::PathMatch;
use gpui::{AppContext, RenderOnce, SharedString, Task, View, WeakView};
use language::{LineEnding, LspAdapterDelegate};
use project::PathMatchCandidateSet;
use std::{
    ops::Range,
    path::{Path, PathBuf},
    sync::{atomic::AtomicBool, Arc},
};
use ui::{prelude::*, ButtonLike, ElevationIndex};
use workspace::Workspace;

pub(crate) struct FileSlashCommand;

impl FileSlashCommand {
    fn search_paths(
        &self,
        query: String,
        cancellation_flag: Arc<AtomicBool>,
        workspace: &View<Workspace>,
        cx: &mut AppContext,
    ) -> Task<Vec<PathMatch>> {
        if query.is_empty() {
            let workspace = workspace.read(cx);
            let project = workspace.project().read(cx);
            let entries = workspace.recent_navigation_history(Some(10), cx);
            let path_prefix: Arc<str> = "".into();
            Task::ready(
                entries
                    .into_iter()
                    .filter_map(|(entry, _)| {
                        let worktree = project.worktree_for_id(entry.worktree_id, cx)?;
                        let mut full_path = PathBuf::from(worktree.read(cx).root_name());
                        full_path.push(&entry.path);
                        Some(PathMatch {
                            score: 0.,
                            positions: Vec::new(),
                            worktree_id: entry.worktree_id.to_usize(),
                            path: full_path.into(),
                            path_prefix: path_prefix.clone(),
                            distance_to_relative_ancestor: 0,
                        })
                    })
                    .collect(),
            )
        } else {
            let worktrees = workspace.read(cx).visible_worktrees(cx).collect::<Vec<_>>();
            let candidate_sets = worktrees
                .into_iter()
                .map(|worktree| {
                    let worktree = worktree.read(cx);
                    PathMatchCandidateSet {
                        snapshot: worktree.snapshot(),
                        include_ignored: worktree
                            .root_entry()
                            .map_or(false, |entry| entry.is_ignored),
                        include_root_name: true,
                        directories_only: false,
                    }
                })
                .collect::<Vec<_>>();

            let executor = cx.background_executor().clone();
            cx.foreground_executor().spawn(async move {
                fuzzy::match_path_sets(
                    candidate_sets.as_slice(),
                    query.as_str(),
                    None,
                    false,
                    100,
                    &cancellation_flag,
                    executor,
                )
                .await
            })
        }
    }
}

impl SlashCommand for FileSlashCommand {
    fn name(&self) -> String {
        "file".into()
    }

    fn description(&self) -> String {
        "insert file".into()
    }

    fn menu_text(&self) -> String {
        "Insert File".into()
    }

    fn requires_argument(&self) -> bool {
        true
    }

    fn complete_argument(
        &self,
        query: String,
        cancellation_flag: Arc<AtomicBool>,
        workspace: WeakView<Workspace>,
        cx: &mut AppContext,
    ) -> Task<Result<Vec<String>>> {
        let Some(workspace) = workspace.upgrade() else {
            return Task::ready(Err(anyhow!("workspace was dropped")));
        };

        let paths = self.search_paths(query, cancellation_flag, &workspace, cx);
        cx.background_executor().spawn(async move {
            Ok(paths
                .await
                .into_iter()
                .map(|path_match| {
                    format!(
                        "{}{}",
                        path_match.path_prefix,
                        path_match.path.to_string_lossy()
                    )
                })
                .collect())
        })
    }

    fn run(
        self: Arc<Self>,
        argument: Option<&str>,
        workspace: WeakView<Workspace>,
        _delegate: Arc<dyn LspAdapterDelegate>,
        cx: &mut WindowContext,
    ) -> Task<Result<SlashCommandOutput>> {
        let Some(workspace) = workspace.upgrade() else {
            return Task::ready(Err(anyhow!("workspace was dropped")));
        };

        let Some(argument) = argument else {
            return Task::ready(Err(anyhow!("missing path")));
        };

        let path = PathBuf::from(argument);
        let abs_path = workspace
            .read(cx)
            .visible_worktrees(cx)
            .find_map(|worktree| {
                let worktree = worktree.read(cx);
                let worktree_root_path = Path::new(worktree.root_name());
                let relative_path = path.strip_prefix(worktree_root_path).ok()?;
                worktree.absolutize(&relative_path).ok()
            });

        let Some(abs_path) = abs_path else {
            return Task::ready(Err(anyhow!("missing path")));
        };

        let fs = workspace.read(cx).app_state().fs.clone();
        let argument = argument.to_string();
        let text = cx.background_executor().spawn(async move {
            let mut content = fs.load(&abs_path).await?;
            LineEnding::normalize(&mut content);
            let mut output = String::with_capacity(argument.len() + content.len() + 9);
            output.push_str("```");
            output.push_str(&argument);
            output.push('\n');
            output.push_str(&content);
            if !output.ends_with('\n') {
                output.push('\n');
            }
            output.push_str("```");
            anyhow::Ok(output)
        });
        cx.foreground_executor().spawn(async move {
            let text = text.await?;
            let range = 0..text.len();
            Ok(SlashCommandOutput {
                text,
                sections: vec![SlashCommandOutputSection {
                    range,
                    render_placeholder: Arc::new(move |id, unfold, _cx| {
                        FilePlaceholder {
                            path: Some(path.clone()),
                            line_range: None,
                            id,
                            unfold,
                        }
                        .into_any_element()
                    }),
                }],
            })
        })
    }
}

#[derive(IntoElement)]
pub struct FilePlaceholder {
    pub path: Option<PathBuf>,
    pub line_range: Option<Range<u32>>,
    pub id: ElementId,
    pub unfold: Arc<dyn Fn(&mut WindowContext)>,
}

impl RenderOnce for FilePlaceholder {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        let unfold = self.unfold;
        let title = if let Some(path) = self.path.as_ref() {
            SharedString::from(path.to_string_lossy().to_string())
        } else {
            SharedString::from("untitled")
        };

        ButtonLike::new(self.id)
            .style(ButtonStyle::Filled)
            .layer(ElevationIndex::ElevatedSurface)
            .child(Icon::new(IconName::File))
            .child(Label::new(title))
            .when_some(self.line_range, |button, line_range| {
                button.child(Label::new(":")).child(Label::new(format!(
                    "{}-{}",
                    line_range.start, line_range.end
                )))
            })
            .on_click(move |_, cx| unfold(cx))
    }
}
