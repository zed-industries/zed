use super::{SlashCommand, SlashCommandOutput};
use anyhow::{anyhow, Result};
use assistant_slash_command::SlashCommandOutputSection;
use fs::Fs;
use fuzzy::PathMatch;
use gpui::{AppContext, Model, RenderOnce, SharedString, Task, View, WeakView};
use language::{LineEnding, LspAdapterDelegate};
use project::{PathMatchCandidateSet, Worktree};
use std::{
    fmt::Write,
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
                    .map(|(range, path, entry_type)| SlashCommandOutputSection {
                        range,
                        render_placeholder: Arc::new(move |id, unfold, _cx| {
                            EntryPlaceholder {
                                path: Some(path.clone()),
                                is_directory: entry_type == EntryType::Directory,
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

#[derive(Clone, Copy, PartialEq)]
enum EntryType {
    File,
    Directory,
}

fn collect_files(
    worktrees: Vec<Model<Worktree>>,
    glob_input: &str,
    fs: Arc<dyn Fs>,
    cx: &mut AppContext,
) -> Task<Result<(String, Vec<(Range<usize>, PathBuf, EntryType)>)>> {
    let Ok(matcher) = PathMatcher::new(glob_input) else {
        return Task::ready(Err(anyhow!("invalid path")));
    };

    let path = PathBuf::try_from(glob_input).ok();
    let file_path = if let Some(path) = &path {
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
            let filename = path
                .as_ref()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_default();
            return cx.background_executor().spawn(async move {
                let mut text = String::new();
                collect_file_content(&mut text, fs, filename.clone(), abs_path.clone().into())
                    .await?;
                let text_range = 0..text.len();
                Ok((
                    text,
                    vec![(text_range, path.unwrap_or_default(), EntryType::File)],
                ))
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
            let mut directory_stack: Vec<(Arc<Path>, String, usize)> = Vec::new();
            let mut folded_directory_names_stack = Vec::new();
            let mut is_top_level_directory = true;
            for entry in snapshot.entries(false, 0) {
                let mut path_buf = PathBuf::new();
                path_buf.push(snapshot.root_name());
                path_buf.push(&entry.path);
                if !matcher.is_match(&path_buf) {
                    continue;
                }

                while let Some((dir, _, _)) = directory_stack.last() {
                    if entry.path.starts_with(dir) {
                        break;
                    }
                    let (_, entry_name, start) = directory_stack.pop().unwrap();
                    ranges.push((
                        start..text.len().saturating_sub(1),
                        PathBuf::from(entry_name),
                        EntryType::Directory,
                    ));
                }

                let filename = entry
                    .path
                    .file_name()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default()
                    .to_string();

                if entry.is_dir() {
                    // Auto-fold directories that contain no files
                    let mut child_entries = snapshot.child_entries(&entry.path);
                    if let Some(child) = child_entries.next() {
                        if child_entries.next().is_none() && child.kind.is_dir() {
                            if is_top_level_directory {
                                is_top_level_directory = false;
                                folded_directory_names_stack
                                    .push(path_buf.to_string_lossy().to_string());
                            } else {
                                folded_directory_names_stack.push(filename.to_string());
                            }
                            continue;
                        }
                    } else {
                        // Skip empty directories
                        folded_directory_names_stack.clear();
                        continue;
                    }
                    let prefix_paths = folded_directory_names_stack.drain(..).as_slice().join("/");
                    let entry_start = text.len();
                    if prefix_paths.is_empty() {
                        if is_top_level_directory {
                            text.push_str(&path_buf.to_string_lossy());
                            is_top_level_directory = false;
                        } else {
                            text.push_str(&filename);
                        }
                        directory_stack.push((entry.path.clone(), filename, entry_start));
                    } else {
                        let entry_name = format!("{}/{}", prefix_paths, &filename);
                        text.push_str(&entry_name);
                        directory_stack.push((entry.path.clone(), entry_name, entry_start));
                    }
                    text.push('\n');
                } else if entry.is_file() {
                    if let Some(abs_path) = snapshot.absolutize(&entry.path).log_err() {
                        let prev_len = text.len();
                        collect_file_content(
                            &mut text,
                            fs.clone(),
                            filename.clone(),
                            abs_path.into(),
                        )
                        .await?;
                        ranges.push((
                            prev_len..text.len(),
                            PathBuf::from(filename),
                            EntryType::File,
                        ));
                        text.push('\n');
                    }
                }
            }

            while let Some((dir, _, start)) = directory_stack.pop() {
                let mut root_path = PathBuf::new();
                root_path.push(snapshot.root_name());
                root_path.push(&dir);
                ranges.push((start..text.len(), root_path, EntryType::Directory));
            }
        }
        Ok((text, ranges))
    })
}

async fn collect_file_content(
    buffer: &mut String,
    fs: Arc<dyn Fs>,
    filename: String,
    abs_path: Arc<Path>,
) -> Result<()> {
    let mut content = fs.load(&abs_path).await?;
    LineEnding::normalize(&mut content);
    buffer.reserve(filename.len() + content.len() + 9);
    buffer.push_str(&codeblock_fence_for_path(
        Some(&PathBuf::from(filename)),
        None,
    ));
    buffer.push_str(&content);
    if !buffer.ends_with('\n') {
        buffer.push('\n');
    }
    buffer.push_str("```");
    anyhow::Ok(())
}

#[derive(IntoElement)]
pub struct EntryPlaceholder {
    pub path: Option<PathBuf>,
    pub is_directory: bool,
    pub line_range: Option<Range<u32>>,
    pub id: ElementId,
    pub unfold: Arc<dyn Fn(&mut WindowContext)>,
}

impl RenderOnce for EntryPlaceholder {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        let unfold = self.unfold;
        let title = if let Some(path) = self.path.as_ref() {
            SharedString::from(path.to_string_lossy().to_string())
        } else {
            SharedString::from("untitled")
        };
        let icon = if self.is_directory {
            IconName::Folder
        } else {
            IconName::File
        };

        ButtonLike::new(self.id)
            .style(ButtonStyle::Filled)
            .layer(ElevationIndex::ElevatedSurface)
            .child(Icon::new(icon))
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

pub fn codeblock_fence_for_path(path: Option<&Path>, row_range: Option<Range<u32>>) -> String {
    let mut text = String::new();
    write!(text, "```").unwrap();

    if let Some(path) = path {
        if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
            write!(text, "{} ", extension).unwrap();
        }

        write!(text, "{}", path.display()).unwrap();
    } else {
        write!(text, "untitled").unwrap();
    }

    if let Some(row_range) = row_range {
        write!(text, ":{}-{}", row_range.start + 1, row_range.end + 1).unwrap();
    }

    text.push('\n');
    text
}
