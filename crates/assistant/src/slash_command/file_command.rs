use super::{SlashCommand, SlashCommandOutput};
use anyhow::{anyhow, Result};
use assistant_slash_command::SlashCommandOutputSection;
use fs::Fs;
use fuzzy::PathMatch;
use gpui::{AppContext, Model, RenderOnce, SharedString, Task, View, WeakView};
use language::{LineEnding, LspAdapterDelegate};
use project::{PathMatchCandidateSet, Worktree};
use std::{
    ops::Range,
    path::{Path, PathBuf},
    sync::{atomic::AtomicBool, Arc},
};
use ui::{prelude::*, ButtonLike, ElevationIndex};
use util::{paths::PathMatcher, ResultExt};
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
                        candidates: project::Candidates::Entries,
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
        workspace: Option<WeakView<Workspace>>,
        cx: &mut AppContext,
    ) -> Task<Result<Vec<String>>> {
        let Some(workspace) = workspace.and_then(|workspace| workspace.upgrade()) else {
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

        let fs = workspace.read(cx).app_state().fs.clone();
        let task = collect_files(
            workspace.read(cx).visible_worktrees(cx).collect(),
            argument,
            fs,
            cx,
        );

        cx.foreground_executor().spawn(async move {
            let (text, ranges) = task.await?;
            Ok(SlashCommandOutput {
                text,
                sections: ranges
                    .into_iter()
                    .map(|(range, path)| SlashCommandOutputSection {
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
                    })
                    .collect(),
                run_commands_in_text: false,
            })
        })
    }
}

fn collect_files(
    worktrees: Vec<Model<Worktree>>,
    glob_input: &str,
    fs: Arc<dyn Fs>,
    cx: &mut AppContext,
) -> Task<Result<(String, Vec<(Range<usize>, PathBuf)>)>> {
    let Ok(matcher) = PathMatcher::new(glob_input) else {
        return Task::ready(Err(anyhow!("invalid path")));
    };

    let path = PathBuf::try_from(glob_input).ok();
    let file_path = if let Some(path) = path {
        worktrees.iter().find_map(|worktree| {
            let worktree = worktree.read(cx);
            let worktree_root_path = Path::new(worktree.root_name());
            let relative_path = path.strip_prefix(worktree_root_path).ok()?;
            worktree.absolutize(&relative_path).ok()
        })
    } else {
        None
    };

    if let Some(abs_path) = file_path {
        if abs_path.is_file() {
            let filename = abs_path.to_string_lossy().to_string();
            return cx.background_executor().spawn(async move {
                let text =
                    collect_file_content(fs, filename.clone(), abs_path.clone().into()).await?;
                let text_range = 0..text.len();
                Ok((text, vec![(text_range, abs_path)]))
            });
        }
    }

    let snapshots = worktrees
        .iter()
        .map(|worktree| worktree.read(cx).snapshot())
        .collect::<Vec<_>>();
    cx.background_executor().spawn(async move {
        let mut text = String::new();
        let mut ranges = Vec::new();
        for snapshot in snapshots {
            for entry in snapshot.entries(false, 0) {
                if !matcher.is_match(&entry.path) {
                    continue;
                }
                if entry.is_file() {
                    if let Some(abs_path) = snapshot.absolutize(&entry.path).log_err() {
                        let prev_len = text.len();
                        let filename = entry.path.to_string_lossy().to_string();
                        let file_contents =
                            collect_file_content(fs.clone(), filename, abs_path.into()).await?;
                        text.push_str(&file_contents);
                        ranges.push((prev_len..text.len(), entry.path.to_path_buf()));
                        text.push('\n');
                    }
                }
            }
        }
        Ok((text, ranges))
    })
}

async fn collect_file_content(
    fs: Arc<dyn Fs>,
    filename: String,
    abs_path: Arc<Path>,
) -> Result<String> {
    let mut content = fs.load(&abs_path).await?;
    LineEnding::normalize(&mut content);
    let mut output = String::with_capacity(filename.len() + content.len() + 9);
    output.push_str("```");
    output.push_str(&filename);
    output.push('\n');
    output.push_str(&content);
    if !output.ends_with('\n') {
        output.push('\n');
    }
    output.push_str("```");
    anyhow::Ok(output)
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
