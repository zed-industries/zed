#[cfg(test)]
mod file_finder_tests;

use collections::HashMap;
use editor::{scroll::Autoscroll, Bias, Editor};
use fuzzy::{CharBag, PathMatch, PathMatchCandidate};
use gpui::{
    actions, rems, AppContext, DismissEvent, EventEmitter, FocusHandle, FocusableView, Model,
    ParentElement, Render, Styled, Task, View, ViewContext, VisualContext, WeakView,
};
use picker::{Picker, PickerDelegate};
use project::{PathMatchCandidateSet, Project, ProjectPath, WorktreeId};
use std::{
    path::{Path, PathBuf},
    sync::{
        atomic::{self, AtomicBool},
        Arc,
    },
};
use text::Point;
use ui::{prelude::*, HighlightedLabel, ListItem, ListItemSpacing};
use util::{paths::PathLikeWithPosition, post_inc, ResultExt};
use workspace::{ModalView, Workspace};

actions!(file_finder, [Toggle]);

impl ModalView for FileFinder {}

pub struct FileFinder {
    picker: View<Picker<FileFinderDelegate>>,
}

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(FileFinder::register).detach();
}

impl FileFinder {
    fn register(workspace: &mut Workspace, _: &mut ViewContext<Workspace>) {
        workspace.register_action(|workspace, _: &Toggle, cx| {
            let Some(file_finder) = workspace.active_modal::<Self>(cx) else {
                Self::open(workspace, cx);
                return;
            };

            file_finder.update(cx, |file_finder, cx| {
                file_finder
                    .picker
                    .update(cx, |picker, cx| picker.cycle_selection(cx))
            });
        });
    }

    fn open(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) {
        let project = workspace.project().read(cx);

        let currently_opened_path = workspace
            .active_item(cx)
            .and_then(|item| item.project_path(cx))
            .map(|project_path| {
                let abs_path = project
                    .worktree_for_id(project_path.worktree_id, cx)
                    .map(|worktree| worktree.read(cx).abs_path().join(&project_path.path));
                FoundPath::new(project_path, abs_path)
            });

        // if exists, bubble the currently opened path to the top
        let history_items = currently_opened_path
            .clone()
            .into_iter()
            .chain(
                workspace
                    .recent_navigation_history(Some(MAX_RECENT_SELECTIONS), cx)
                    .into_iter()
                    .filter(|(history_path, _)| {
                        Some(history_path)
                            != currently_opened_path
                                .as_ref()
                                .map(|found_path| &found_path.project)
                    })
                    .filter(|(_, history_abs_path)| {
                        history_abs_path.as_ref()
                            != currently_opened_path
                                .as_ref()
                                .and_then(|found_path| found_path.absolute.as_ref())
                    })
                    .filter(|(_, history_abs_path)| match history_abs_path {
                        Some(abs_path) => history_file_exists(abs_path),
                        None => true,
                    })
                    .map(|(history_path, abs_path)| FoundPath::new(history_path, abs_path)),
            )
            .collect();

        let project = workspace.project().clone();
        let weak_workspace = cx.view().downgrade();
        workspace.toggle_modal(cx, |cx| {
            let delegate = FileFinderDelegate::new(
                cx.view().downgrade(),
                weak_workspace,
                project,
                currently_opened_path,
                history_items,
                cx,
            );

            FileFinder::new(delegate, cx)
        });
    }

    fn new(delegate: FileFinderDelegate, cx: &mut ViewContext<Self>) -> Self {
        Self {
            picker: cx.new_view(|cx| Picker::new(delegate, cx)),
        }
    }
}

impl EventEmitter<DismissEvent> for FileFinder {}

impl FocusableView for FileFinder {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for FileFinder {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex().w(rems(34.)).child(self.picker.clone())
    }
}

pub struct FileFinderDelegate {
    file_finder: WeakView<FileFinder>,
    workspace: WeakView<Workspace>,
    project: Model<Project>,
    search_count: usize,
    latest_search_id: usize,
    latest_search_did_cancel: bool,
    latest_search_query: Option<PathLikeWithPosition<FileSearchQuery>>,
    currently_opened_path: Option<FoundPath>,
    matches: Matches,
    selected_index: Option<usize>,
    cancel_flag: Arc<AtomicBool>,
    history_items: Vec<FoundPath>,
}

#[derive(Debug, Default)]
struct Matches {
    history: Vec<(FoundPath, Option<PathMatch>)>,
    search: Vec<PathMatch>,
}

#[derive(Debug)]
enum Match<'a> {
    History(&'a FoundPath, Option<&'a PathMatch>),
    Search(&'a PathMatch),
}

impl Matches {
    fn len(&self) -> usize {
        self.history.len() + self.search.len()
    }

    fn get(&self, index: usize) -> Option<Match<'_>> {
        if index < self.history.len() {
            self.history
                .get(index)
                .map(|(path, path_match)| Match::History(path, path_match.as_ref()))
        } else {
            self.search
                .get(index - self.history.len())
                .map(Match::Search)
        }
    }

    fn push_new_matches(
        &mut self,
        history_items: &Vec<FoundPath>,
        query: &PathLikeWithPosition<FileSearchQuery>,
        mut new_search_matches: Vec<PathMatch>,
        extend_old_matches: bool,
    ) {
        let matching_history_paths = matching_history_item_paths(history_items, query);
        new_search_matches
            .retain(|path_match| !matching_history_paths.contains_key(&path_match.path));
        let history_items_to_show = history_items
            .iter()
            .filter_map(|history_item| {
                Some((
                    history_item.clone(),
                    Some(
                        matching_history_paths
                            .get(&history_item.project.path)?
                            .clone(),
                    ),
                ))
            })
            .collect::<Vec<_>>();
        self.history = history_items_to_show;
        if extend_old_matches {
            self.search
                .retain(|path_match| !matching_history_paths.contains_key(&path_match.path));
            util::extend_sorted(
                &mut self.search,
                new_search_matches.into_iter(),
                100,
                |a, b| b.cmp(a),
            )
        } else {
            self.search = new_search_matches;
        }
    }
}

fn matching_history_item_paths(
    history_items: &Vec<FoundPath>,
    query: &PathLikeWithPosition<FileSearchQuery>,
) -> HashMap<Arc<Path>, PathMatch> {
    let history_items_by_worktrees = history_items
        .iter()
        .filter_map(|found_path| {
            let candidate = PathMatchCandidate {
                path: &found_path.project.path,
                // Only match history items names, otherwise their paths may match too many queries, producing false positives.
                // E.g. `foo` would match both `something/foo/bar.rs` and `something/foo/foo.rs` and if the former is a history item,
                // it would be shown first always, despite the latter being a better match.
                char_bag: CharBag::from_iter(
                    found_path
                        .project
                        .path
                        .file_name()?
                        .to_string_lossy()
                        .to_lowercase()
                        .chars(),
                ),
            };
            Some((found_path.project.worktree_id, candidate))
        })
        .fold(
            HashMap::default(),
            |mut candidates, (worktree_id, new_candidate)| {
                candidates
                    .entry(worktree_id)
                    .or_insert_with(Vec::new)
                    .push(new_candidate);
                candidates
            },
        );
    let mut matching_history_paths = HashMap::default();
    for (worktree, candidates) in history_items_by_worktrees {
        let max_results = candidates.len() + 1;
        matching_history_paths.extend(
            fuzzy::match_fixed_path_set(
                candidates,
                worktree.to_usize(),
                query.path_like.path_query(),
                false,
                max_results,
            )
            .into_iter()
            .map(|path_match| (Arc::clone(&path_match.path), path_match)),
        );
    }
    matching_history_paths
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FoundPath {
    project: ProjectPath,
    absolute: Option<PathBuf>,
}

impl FoundPath {
    fn new(project: ProjectPath, absolute: Option<PathBuf>) -> Self {
        Self { project, absolute }
    }
}

const MAX_RECENT_SELECTIONS: usize = 20;

#[cfg(not(test))]
fn history_file_exists(abs_path: &PathBuf) -> bool {
    abs_path.exists()
}

#[cfg(test)]
fn history_file_exists(abs_path: &PathBuf) -> bool {
    !abs_path.ends_with("nonexistent.rs")
}

pub enum Event {
    Selected(ProjectPath),
    Dismissed,
}

#[derive(Debug, Clone)]
struct FileSearchQuery {
    raw_query: String,
    file_query_end: Option<usize>,
}

impl FileSearchQuery {
    fn path_query(&self) -> &str {
        match self.file_query_end {
            Some(file_path_end) => &self.raw_query[..file_path_end],
            None => &self.raw_query,
        }
    }
}

impl FileFinderDelegate {
    fn new(
        file_finder: WeakView<FileFinder>,
        workspace: WeakView<Workspace>,
        project: Model<Project>,
        currently_opened_path: Option<FoundPath>,
        history_items: Vec<FoundPath>,
        cx: &mut ViewContext<FileFinder>,
    ) -> Self {
        cx.observe(&project, |file_finder, _, cx| {
            //todo We should probably not re-render on every project anything
            file_finder
                .picker
                .update(cx, |picker, cx| picker.refresh(cx))
        })
        .detach();

        Self {
            file_finder,
            workspace,
            project,
            search_count: 0,
            latest_search_id: 0,
            latest_search_did_cancel: false,
            latest_search_query: None,
            currently_opened_path,
            matches: Matches::default(),
            selected_index: None,
            cancel_flag: Arc::new(AtomicBool::new(false)),
            history_items,
        }
    }

    fn spawn_search(
        &mut self,
        query: PathLikeWithPosition<FileSearchQuery>,
        cx: &mut ViewContext<Picker<Self>>,
    ) -> Task<()> {
        let relative_to = self
            .currently_opened_path
            .as_ref()
            .map(|found_path| Arc::clone(&found_path.project.path));
        let worktrees = self
            .project
            .read(cx)
            .visible_worktrees(cx)
            .collect::<Vec<_>>();
        let include_root_name = worktrees.len() > 1;
        let candidate_sets = worktrees
            .into_iter()
            .map(|worktree| {
                let worktree = worktree.read(cx);
                PathMatchCandidateSet {
                    snapshot: worktree.snapshot(),
                    include_ignored: worktree
                        .root_entry()
                        .map_or(false, |entry| entry.is_ignored),
                    include_root_name,
                }
            })
            .collect::<Vec<_>>();

        let search_id = util::post_inc(&mut self.search_count);
        self.cancel_flag.store(true, atomic::Ordering::Relaxed);
        self.cancel_flag = Arc::new(AtomicBool::new(false));
        let cancel_flag = self.cancel_flag.clone();
        cx.spawn(|picker, mut cx| async move {
            let matches = fuzzy::match_path_sets(
                candidate_sets.as_slice(),
                query.path_like.path_query(),
                relative_to,
                false,
                100,
                &cancel_flag,
                cx.background_executor().clone(),
            )
            .await;
            let did_cancel = cancel_flag.load(atomic::Ordering::Relaxed);
            picker
                .update(&mut cx, |picker, cx| {
                    picker.delegate.selected_index.take();
                    picker
                        .delegate
                        .set_search_matches(search_id, did_cancel, query, matches, cx)
                })
                .log_err();
        })
    }

    fn set_search_matches(
        &mut self,
        search_id: usize,
        did_cancel: bool,
        query: PathLikeWithPosition<FileSearchQuery>,
        matches: Vec<PathMatch>,
        cx: &mut ViewContext<Picker<Self>>,
    ) {
        if search_id >= self.latest_search_id {
            self.latest_search_id = search_id;
            let extend_old_matches = self.latest_search_did_cancel
                && Some(query.path_like.path_query())
                    == self
                        .latest_search_query
                        .as_ref()
                        .map(|query| query.path_like.path_query());
            self.matches
                .push_new_matches(&self.history_items, &query, matches, extend_old_matches);
            self.latest_search_query = Some(query);
            self.latest_search_did_cancel = did_cancel;
            cx.notify();
        }
    }

    fn labels_for_match(
        &self,
        path_match: Match,
        cx: &AppContext,
        ix: usize,
    ) -> (String, Vec<usize>, String, Vec<usize>) {
        let (file_name, file_name_positions, full_path, full_path_positions) = match path_match {
            Match::History(found_path, found_path_match) => {
                let worktree_id = found_path.project.worktree_id;
                let project_relative_path = &found_path.project.path;
                let has_worktree = self
                    .project
                    .read(cx)
                    .worktree_for_id(worktree_id, cx)
                    .is_some();

                if !has_worktree {
                    if let Some(absolute_path) = &found_path.absolute {
                        return (
                            absolute_path
                                .file_name()
                                .map_or_else(
                                    || project_relative_path.to_string_lossy(),
                                    |file_name| file_name.to_string_lossy(),
                                )
                                .to_string(),
                            Vec::new(),
                            absolute_path.to_string_lossy().to_string(),
                            Vec::new(),
                        );
                    }
                }

                let mut path = Arc::clone(project_relative_path);
                if project_relative_path.as_ref() == Path::new("") {
                    if let Some(absolute_path) = &found_path.absolute {
                        path = Arc::from(absolute_path.as_path());
                    }
                }

                let mut path_match = PathMatch {
                    score: ix as f64,
                    positions: Vec::new(),
                    worktree_id: worktree_id.to_usize(),
                    path,
                    path_prefix: "".into(),
                    distance_to_relative_ancestor: usize::MAX,
                };
                if let Some(found_path_match) = found_path_match {
                    path_match
                        .positions
                        .extend(found_path_match.positions.iter())
                }

                self.labels_for_path_match(&path_match)
            }
            Match::Search(path_match) => self.labels_for_path_match(path_match),
        };

        if file_name_positions.is_empty() {
            if let Some(user_home_path) = std::env::var("HOME").ok() {
                let user_home_path = user_home_path.trim();
                if !user_home_path.is_empty() {
                    if (&full_path).starts_with(user_home_path) {
                        return (
                            file_name,
                            file_name_positions,
                            full_path.replace(user_home_path, "~"),
                            full_path_positions,
                        );
                    }
                }
            }
        }

        (
            file_name,
            file_name_positions,
            full_path,
            full_path_positions,
        )
    }

    fn labels_for_path_match(
        &self,
        path_match: &PathMatch,
    ) -> (String, Vec<usize>, String, Vec<usize>) {
        let path = &path_match.path;
        let path_string = path.to_string_lossy();
        let full_path = [path_match.path_prefix.as_ref(), path_string.as_ref()].join("");
        let path_positions = path_match.positions.clone();

        let file_name = path.file_name().map_or_else(
            || path_match.path_prefix.to_string(),
            |file_name| file_name.to_string_lossy().to_string(),
        );
        let file_name_start = path_match.path_prefix.len() + path_string.len() - file_name.len();
        let file_name_positions = path_positions
            .iter()
            .filter_map(|pos| {
                if pos >= &file_name_start {
                    Some(pos - file_name_start)
                } else {
                    None
                }
            })
            .collect();

        (file_name, file_name_positions, full_path, path_positions)
    }

    fn lookup_absolute_path(
        &self,
        query: PathLikeWithPosition<FileSearchQuery>,
        cx: &mut ViewContext<'_, Picker<Self>>,
    ) -> Task<()> {
        cx.spawn(|picker, mut cx| async move {
            let Some((project, fs)) = picker
                .update(&mut cx, |picker, cx| {
                    let fs = Arc::clone(&picker.delegate.project.read(cx).fs());
                    (picker.delegate.project.clone(), fs)
                })
                .log_err()
            else {
                return;
            };

            let query_path = Path::new(query.path_like.path_query());
            let mut path_matches = Vec::new();
            match fs.metadata(query_path).await.log_err() {
                Some(Some(_metadata)) => {
                    let update_result = project
                        .update(&mut cx, |project, cx| {
                            if let Some((worktree, relative_path)) =
                                project.find_local_worktree(query_path, cx)
                            {
                                path_matches.push(PathMatch {
                                    score: 0.0,
                                    positions: Vec::new(),
                                    worktree_id: worktree.read(cx).id().to_usize(),
                                    path: Arc::from(relative_path),
                                    path_prefix: "".into(),
                                    distance_to_relative_ancestor: usize::MAX,
                                });
                            }
                        })
                        .log_err();
                    if update_result.is_none() {
                        return;
                    }
                }
                Some(None) => {}
                None => return,
            }

            picker
                .update(&mut cx, |picker, cx| {
                    let picker_delegate = &mut picker.delegate;
                    let search_id = util::post_inc(&mut picker_delegate.search_count);
                    picker_delegate.set_search_matches(search_id, false, query, path_matches, cx);

                    anyhow::Ok(())
                })
                .log_err();
        })
    }
}

impl PickerDelegate for FileFinderDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self) -> Arc<str> {
        "Search project files...".into()
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index.unwrap_or(0)
    }

    fn set_selected_index(&mut self, ix: usize, cx: &mut ViewContext<Picker<Self>>) {
        self.selected_index = Some(ix);
        cx.notify();
    }

    fn separators_after_indices(&self) -> Vec<usize> {
        let history_items = self.matches.history.len();
        if history_items == 0 || self.matches.search.is_empty() {
            Vec::new()
        } else {
            vec![history_items - 1]
        }
    }

    fn update_matches(
        &mut self,
        raw_query: String,
        cx: &mut ViewContext<Picker<Self>>,
    ) -> Task<()> {
        let raw_query = raw_query.trim();
        if raw_query.is_empty() {
            let project = self.project.read(cx);
            self.latest_search_id = post_inc(&mut self.search_count);
            self.selected_index.take();
            self.matches = Matches {
                history: self
                    .history_items
                    .iter()
                    .filter(|history_item| {
                        project
                            .worktree_for_id(history_item.project.worktree_id, cx)
                            .is_some()
                            || (project.is_local() && history_item.absolute.is_some())
                    })
                    .cloned()
                    .map(|p| (p, None))
                    .collect(),
                search: Vec::new(),
            };
            cx.notify();
            Task::ready(())
        } else {
            let query = PathLikeWithPosition::parse_str(raw_query, |path_like_str| {
                Ok::<_, std::convert::Infallible>(FileSearchQuery {
                    raw_query: raw_query.to_owned(),
                    file_query_end: if path_like_str == raw_query {
                        None
                    } else {
                        Some(path_like_str.len())
                    },
                })
            })
            .expect("infallible");

            if Path::new(query.path_like.path_query()).is_absolute() {
                self.lookup_absolute_path(query, cx)
            } else {
                self.spawn_search(query, cx)
            }
        }
    }

    fn confirm(&mut self, secondary: bool, cx: &mut ViewContext<Picker<FileFinderDelegate>>) {
        if let Some(m) = self.matches.get(self.selected_index()) {
            if let Some(workspace) = self.workspace.upgrade() {
                let open_task = workspace.update(cx, move |workspace, cx| {
                    let split_or_open = |workspace: &mut Workspace, project_path, cx| {
                        if secondary {
                            workspace.split_path(project_path, cx)
                        } else {
                            workspace.open_path(project_path, None, true, cx)
                        }
                    };
                    match m {
                        Match::History(history_match, _) => {
                            let worktree_id = history_match.project.worktree_id;
                            if workspace
                                .project()
                                .read(cx)
                                .worktree_for_id(worktree_id, cx)
                                .is_some()
                            {
                                split_or_open(
                                    workspace,
                                    ProjectPath {
                                        worktree_id,
                                        path: Arc::clone(&history_match.project.path),
                                    },
                                    cx,
                                )
                            } else {
                                match history_match.absolute.as_ref() {
                                    Some(abs_path) => {
                                        if secondary {
                                            workspace.split_abs_path(
                                                abs_path.to_path_buf(),
                                                false,
                                                cx,
                                            )
                                        } else {
                                            workspace.open_abs_path(
                                                abs_path.to_path_buf(),
                                                false,
                                                cx,
                                            )
                                        }
                                    }
                                    None => split_or_open(
                                        workspace,
                                        ProjectPath {
                                            worktree_id,
                                            path: Arc::clone(&history_match.project.path),
                                        },
                                        cx,
                                    ),
                                }
                            }
                        }
                        Match::Search(m) => split_or_open(
                            workspace,
                            ProjectPath {
                                worktree_id: WorktreeId::from_usize(m.worktree_id),
                                path: m.path.clone(),
                            },
                            cx,
                        ),
                    }
                });

                let row = self
                    .latest_search_query
                    .as_ref()
                    .and_then(|query| query.row)
                    .map(|row| row.saturating_sub(1));
                let col = self
                    .latest_search_query
                    .as_ref()
                    .and_then(|query| query.column)
                    .unwrap_or(0)
                    .saturating_sub(1);
                let finder = self.file_finder.clone();

                cx.spawn(|_, mut cx| async move {
                    let item = open_task.await.log_err()?;
                    if let Some(row) = row {
                        if let Some(active_editor) = item.downcast::<Editor>() {
                            active_editor
                                .downgrade()
                                .update(&mut cx, |editor, cx| {
                                    let snapshot = editor.snapshot(cx).display_snapshot;
                                    let point = snapshot
                                        .buffer_snapshot
                                        .clip_point(Point::new(row, col), Bias::Left);
                                    editor.change_selections(Some(Autoscroll::center()), cx, |s| {
                                        s.select_ranges([point..point])
                                    });
                                })
                                .log_err();
                        }
                    }
                    finder.update(&mut cx, |_, cx| cx.emit(DismissEvent)).ok()?;

                    Some(())
                })
                .detach();
            }
        }
    }

    fn dismissed(&mut self, cx: &mut ViewContext<Picker<FileFinderDelegate>>) {
        self.file_finder
            .update(cx, |_, cx| cx.emit(DismissEvent))
            .log_err();
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        cx: &mut ViewContext<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let path_match = self
            .matches
            .get(ix)
            .expect("Invalid matches state: no element for index {ix}");

        let (file_name, file_name_positions, full_path, full_path_positions) =
            self.labels_for_match(path_match, cx, ix);

        Some(
            ListItem::new(ix)
                .spacing(ListItemSpacing::Sparse)
                .inset(true)
                .selected(selected)
                .child(
                    v_flex()
                        .child(HighlightedLabel::new(file_name, file_name_positions))
                        .child(HighlightedLabel::new(full_path, full_path_positions)),
                ),
        )
    }
}
