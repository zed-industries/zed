use collections::HashMap;
use editor::{scroll::autoscroll::Autoscroll, Bias, Editor};
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
        v_stack().w(rems(34.)).child(self.picker.clone())
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
            //todo!() We should probably not re-render on every project anything
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
            .map(|worktree| PathMatchCandidateSet {
                snapshot: worktree.read(cx).snapshot(),
                include_ignored: true,
                include_root_name,
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
}

impl PickerDelegate for FileFinderDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self) -> SharedString {
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
            self.spawn_search(query, cx)
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
                    v_stack()
                        .child(HighlightedLabel::new(file_name, file_name_positions))
                        .child(HighlightedLabel::new(full_path, full_path_positions)),
                ),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::{assert_eq, path::Path, time::Duration};

    use super::*;
    use editor::Editor;
    use gpui::{Entity, TestAppContext, VisualTestContext};
    use menu::{Confirm, SelectNext};
    use serde_json::json;
    use workspace::{AppState, Workspace};

    #[ctor::ctor]
    fn init_logger() {
        if std::env::var("RUST_LOG").is_ok() {
            env_logger::init();
        }
    }

    #[gpui::test]
    async fn test_matching_paths(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        app_state
            .fs
            .as_fake()
            .insert_tree(
                "/root",
                json!({
                    "a": {
                        "banana": "",
                        "bandana": "",
                    }
                }),
            )
            .await;

        let project = Project::test(app_state.fs.clone(), ["/root".as_ref()], cx).await;

        let (picker, workspace, cx) = build_find_picker(project, cx);

        cx.simulate_input("bna");
        picker.update(cx, |picker, _| {
            assert_eq!(picker.delegate.matches.len(), 2);
        });
        cx.dispatch_action(SelectNext);
        cx.dispatch_action(Confirm);
        cx.read(|cx| {
            let active_editor = workspace.read(cx).active_item_as::<Editor>(cx).unwrap();
            assert_eq!(active_editor.read(cx).title(cx), "bandana");
        });

        for bandana_query in [
            "bandana",
            " bandana",
            "bandana ",
            " bandana ",
            " ndan ",
            " band ",
        ] {
            picker
                .update(cx, |picker, cx| {
                    picker
                        .delegate
                        .update_matches(bandana_query.to_string(), cx)
                })
                .await;
            picker.update(cx, |picker, _| {
                assert_eq!(
                    picker.delegate.matches.len(),
                    1,
                    "Wrong number of matches for bandana query '{bandana_query}'"
                );
            });
            cx.dispatch_action(SelectNext);
            cx.dispatch_action(Confirm);
            cx.read(|cx| {
                let active_editor = workspace.read(cx).active_item_as::<Editor>(cx).unwrap();
                assert_eq!(
                    active_editor.read(cx).title(cx),
                    "bandana",
                    "Wrong match for bandana query '{bandana_query}'"
                );
            });
        }
    }

    #[gpui::test]
    async fn test_complex_path(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        app_state
            .fs
            .as_fake()
            .insert_tree(
                "/root",
                json!({
                    "其他": {
                        "S数据表格": {
                            "task.xlsx": "some content",
                        },
                    }
                }),
            )
            .await;

        let project = Project::test(app_state.fs.clone(), ["/root".as_ref()], cx).await;

        let (picker, workspace, cx) = build_find_picker(project, cx);

        cx.simulate_input("t");
        picker.update(cx, |picker, _| {
            assert_eq!(picker.delegate.matches.len(), 1);
            assert_eq!(
                collect_search_results(picker),
                vec![PathBuf::from("其他/S数据表格/task.xlsx")],
            )
        });
        cx.dispatch_action(SelectNext);
        cx.dispatch_action(Confirm);
        cx.read(|cx| {
            let active_editor = workspace.read(cx).active_item_as::<Editor>(cx).unwrap();
            assert_eq!(active_editor.read(cx).title(cx), "task.xlsx");
        });
    }

    #[gpui::test]
    async fn test_row_column_numbers_query_inside_file(cx: &mut TestAppContext) {
        let app_state = init_test(cx);

        let first_file_name = "first.rs";
        let first_file_contents = "// First Rust file";
        app_state
            .fs
            .as_fake()
            .insert_tree(
                "/src",
                json!({
                    "test": {
                        first_file_name: first_file_contents,
                        "second.rs": "// Second Rust file",
                    }
                }),
            )
            .await;

        let project = Project::test(app_state.fs.clone(), ["/src".as_ref()], cx).await;

        let (picker, workspace, cx) = build_find_picker(project, cx);

        let file_query = &first_file_name[..3];
        let file_row = 1;
        let file_column = 3;
        assert!(file_column <= first_file_contents.len());
        let query_inside_file = format!("{file_query}:{file_row}:{file_column}");
        picker
            .update(cx, |finder, cx| {
                finder
                    .delegate
                    .update_matches(query_inside_file.to_string(), cx)
            })
            .await;
        picker.update(cx, |finder, _| {
            let finder = &finder.delegate;
            assert_eq!(finder.matches.len(), 1);
            let latest_search_query = finder
                .latest_search_query
                .as_ref()
                .expect("Finder should have a query after the update_matches call");
            assert_eq!(latest_search_query.path_like.raw_query, query_inside_file);
            assert_eq!(
                latest_search_query.path_like.file_query_end,
                Some(file_query.len())
            );
            assert_eq!(latest_search_query.row, Some(file_row));
            assert_eq!(latest_search_query.column, Some(file_column as u32));
        });

        cx.dispatch_action(SelectNext);
        cx.dispatch_action(Confirm);

        let editor = cx.update(|cx| workspace.read(cx).active_item_as::<Editor>(cx).unwrap());
        cx.executor().advance_clock(Duration::from_secs(2));

        editor.update(cx, |editor, cx| {
                let all_selections = editor.selections.all_adjusted(cx);
                assert_eq!(
                    all_selections.len(),
                    1,
                    "Expected to have 1 selection (caret) after file finder confirm, but got: {all_selections:?}"
                );
                let caret_selection = all_selections.into_iter().next().unwrap();
                assert_eq!(caret_selection.start, caret_selection.end,
                    "Caret selection should have its start and end at the same position");
                assert_eq!(file_row, caret_selection.start.row + 1,
                    "Query inside file should get caret with the same focus row");
                assert_eq!(file_column, caret_selection.start.column as usize + 1,
                    "Query inside file should get caret with the same focus column");
            });
    }

    #[gpui::test]
    async fn test_row_column_numbers_query_outside_file(cx: &mut TestAppContext) {
        let app_state = init_test(cx);

        let first_file_name = "first.rs";
        let first_file_contents = "// First Rust file";
        app_state
            .fs
            .as_fake()
            .insert_tree(
                "/src",
                json!({
                    "test": {
                        first_file_name: first_file_contents,
                        "second.rs": "// Second Rust file",
                    }
                }),
            )
            .await;

        let project = Project::test(app_state.fs.clone(), ["/src".as_ref()], cx).await;

        let (picker, workspace, cx) = build_find_picker(project, cx);

        let file_query = &first_file_name[..3];
        let file_row = 200;
        let file_column = 300;
        assert!(file_column > first_file_contents.len());
        let query_outside_file = format!("{file_query}:{file_row}:{file_column}");
        picker
            .update(cx, |picker, cx| {
                picker
                    .delegate
                    .update_matches(query_outside_file.to_string(), cx)
            })
            .await;
        picker.update(cx, |finder, _| {
            let delegate = &finder.delegate;
            assert_eq!(delegate.matches.len(), 1);
            let latest_search_query = delegate
                .latest_search_query
                .as_ref()
                .expect("Finder should have a query after the update_matches call");
            assert_eq!(latest_search_query.path_like.raw_query, query_outside_file);
            assert_eq!(
                latest_search_query.path_like.file_query_end,
                Some(file_query.len())
            );
            assert_eq!(latest_search_query.row, Some(file_row));
            assert_eq!(latest_search_query.column, Some(file_column as u32));
        });

        cx.dispatch_action(SelectNext);
        cx.dispatch_action(Confirm);

        let editor = cx.update(|cx| workspace.read(cx).active_item_as::<Editor>(cx).unwrap());
        cx.executor().advance_clock(Duration::from_secs(2));

        editor.update(cx, |editor, cx| {
                let all_selections = editor.selections.all_adjusted(cx);
                assert_eq!(
                    all_selections.len(),
                    1,
                    "Expected to have 1 selection (caret) after file finder confirm, but got: {all_selections:?}"
                );
                let caret_selection = all_selections.into_iter().next().unwrap();
                assert_eq!(caret_selection.start, caret_selection.end,
                    "Caret selection should have its start and end at the same position");
                assert_eq!(0, caret_selection.start.row,
                    "Excessive rows (as in query outside file borders) should get trimmed to last file row");
                assert_eq!(first_file_contents.len(), caret_selection.start.column as usize,
                    "Excessive columns (as in query outside file borders) should get trimmed to selected row's last column");
            });
    }

    #[gpui::test]
    async fn test_matching_cancellation(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        app_state
            .fs
            .as_fake()
            .insert_tree(
                "/dir",
                json!({
                    "hello": "",
                    "goodbye": "",
                    "halogen-light": "",
                    "happiness": "",
                    "height": "",
                    "hi": "",
                    "hiccup": "",
                }),
            )
            .await;

        let project = Project::test(app_state.fs.clone(), ["/dir".as_ref()], cx).await;

        let (picker, _, cx) = build_find_picker(project, cx);

        let query = test_path_like("hi");
        picker
            .update(cx, |picker, cx| {
                picker.delegate.spawn_search(query.clone(), cx)
            })
            .await;

        picker.update(cx, |picker, _cx| {
            assert_eq!(picker.delegate.matches.len(), 5)
        });

        picker.update(cx, |picker, cx| {
            let delegate = &mut picker.delegate;
            assert!(
                delegate.matches.history.is_empty(),
                "Search matches expected"
            );
            let matches = delegate.matches.search.clone();

            // Simulate a search being cancelled after the time limit,
            // returning only a subset of the matches that would have been found.
            drop(delegate.spawn_search(query.clone(), cx));
            delegate.set_search_matches(
                delegate.latest_search_id,
                true, // did-cancel
                query.clone(),
                vec![matches[1].clone(), matches[3].clone()],
                cx,
            );

            // Simulate another cancellation.
            drop(delegate.spawn_search(query.clone(), cx));
            delegate.set_search_matches(
                delegate.latest_search_id,
                true, // did-cancel
                query.clone(),
                vec![matches[0].clone(), matches[2].clone(), matches[3].clone()],
                cx,
            );

            assert!(
                delegate.matches.history.is_empty(),
                "Search matches expected"
            );
            assert_eq!(delegate.matches.search.as_slice(), &matches[0..4]);
        });
    }

    #[gpui::test]
    async fn test_ignored_root(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        app_state
            .fs
            .as_fake()
            .insert_tree(
                "/ancestor",
                json!({
                    ".gitignore": "ignored-root",
                    "ignored-root": {
                        "happiness": "",
                        "height": "",
                        "hi": "",
                        "hiccup": "",
                    },
                    "tracked-root": {
                        ".gitignore": "height",
                        "happiness": "",
                        "height": "",
                        "hi": "",
                        "hiccup": "",
                    },
                }),
            )
            .await;

        let project = Project::test(
            app_state.fs.clone(),
            [
                "/ancestor/tracked-root".as_ref(),
                "/ancestor/ignored-root".as_ref(),
            ],
            cx,
        )
        .await;

        let (picker, _, cx) = build_find_picker(project, cx);

        picker
            .update(cx, |picker, cx| {
                picker.delegate.spawn_search(test_path_like("hi"), cx)
            })
            .await;
        picker.update(cx, |picker, _| {
            assert_eq!(
                collect_search_results(picker),
                vec![
                    PathBuf::from("ignored-root/happiness"),
                    PathBuf::from("ignored-root/height"),
                    PathBuf::from("ignored-root/hi"),
                    PathBuf::from("ignored-root/hiccup"),
                    PathBuf::from("tracked-root/happiness"),
                    PathBuf::from("tracked-root/height"),
                    PathBuf::from("tracked-root/hi"),
                    PathBuf::from("tracked-root/hiccup"),
                ],
                "All files in all roots (including gitignored) should be searched"
            )
        });
    }

    #[gpui::test]
    async fn test_ignored_files(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        app_state
            .fs
            .as_fake()
            .insert_tree(
                "/root",
                json!({
                    ".git": {},
                    ".gitignore": "ignored_a\n.env\n",
                    "a": {
                        "banana_env": "11",
                        "bandana_env": "12",
                    },
                    "ignored_a": {
                        "ignored_banana_env": "21",
                        "ignored_bandana_env": "22",
                        "ignored_nested": {
                            "ignored_nested_banana_env": "31",
                            "ignored_nested_bandana_env": "32",
                        },
                    },
                    ".env": "something",
                }),
            )
            .await;

        let project = Project::test(app_state.fs.clone(), ["/root".as_ref()], cx).await;

        let (picker, workspace, cx) = build_find_picker(project, cx);

        cx.simulate_input("env");
        picker.update(cx, |picker, _| {
            assert_eq!(
                collect_search_results(picker),
                vec![
                    PathBuf::from(".env"),
                    PathBuf::from("a/banana_env"),
                    PathBuf::from("a/bandana_env"),
                ],
                "Root gitignored files and all non-gitignored files should be searched"
            )
        });

        let _ = workspace
            .update(cx, |workspace, cx| {
                workspace.open_abs_path(
                    PathBuf::from("/root/ignored_a/ignored_banana_env"),
                    true,
                    cx,
                )
            })
            .await
            .unwrap();
        cx.run_until_parked();
        cx.simulate_input("env");
        picker.update(cx, |picker, _| {
            assert_eq!(
                collect_search_results(picker),
                vec![
                    PathBuf::from(".env"),
                    PathBuf::from("a/banana_env"),
                    PathBuf::from("a/bandana_env"),
                    PathBuf::from("ignored_a/ignored_banana_env"),
                    PathBuf::from("ignored_a/ignored_bandana_env"),
                ],
                "Root gitignored dir got listed and its entries got into worktree, but all gitignored dirs below it were not listed. Old entries + new listed gitignored entries should be searched"
            )
        });
    }

    #[gpui::test]
    async fn test_single_file_worktrees(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        app_state
            .fs
            .as_fake()
            .insert_tree("/root", json!({ "the-parent-dir": { "the-file": "" } }))
            .await;

        let project = Project::test(
            app_state.fs.clone(),
            ["/root/the-parent-dir/the-file".as_ref()],
            cx,
        )
        .await;

        let (picker, _, cx) = build_find_picker(project, cx);

        // Even though there is only one worktree, that worktree's filename
        // is included in the matching, because the worktree is a single file.
        picker
            .update(cx, |picker, cx| {
                picker.delegate.spawn_search(test_path_like("thf"), cx)
            })
            .await;
        cx.read(|cx| {
            let picker = picker.read(cx);
            let delegate = &picker.delegate;
            assert!(
                delegate.matches.history.is_empty(),
                "Search matches expected"
            );
            let matches = delegate.matches.search.clone();
            assert_eq!(matches.len(), 1);

            let (file_name, file_name_positions, full_path, full_path_positions) =
                delegate.labels_for_path_match(&matches[0]);
            assert_eq!(file_name, "the-file");
            assert_eq!(file_name_positions, &[0, 1, 4]);
            assert_eq!(full_path, "the-file");
            assert_eq!(full_path_positions, &[0, 1, 4]);
        });

        // Since the worktree root is a file, searching for its name followed by a slash does
        // not match anything.
        picker
            .update(cx, |f, cx| {
                f.delegate.spawn_search(test_path_like("thf/"), cx)
            })
            .await;
        picker.update(cx, |f, _| assert_eq!(f.delegate.matches.len(), 0));
    }

    #[gpui::test]
    async fn test_path_distance_ordering(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        app_state
            .fs
            .as_fake()
            .insert_tree(
                "/root",
                json!({
                    "dir1": { "a.txt": "" },
                    "dir2": {
                        "a.txt": "",
                        "b.txt": ""
                    }
                }),
            )
            .await;

        let project = Project::test(app_state.fs.clone(), ["/root".as_ref()], cx).await;
        let (workspace, cx) = cx.add_window_view(|cx| Workspace::test_new(project, cx));

        let worktree_id = cx.read(|cx| {
            let worktrees = workspace.read(cx).worktrees(cx).collect::<Vec<_>>();
            assert_eq!(worktrees.len(), 1);
            WorktreeId::from_usize(worktrees[0].entity_id().as_u64() as usize)
        });

        // When workspace has an active item, sort items which are closer to that item
        // first when they have the same name. In this case, b.txt is closer to dir2's a.txt
        // so that one should be sorted earlier
        let b_path = ProjectPath {
            worktree_id,
            path: Arc::from(Path::new("dir2/b.txt")),
        };
        workspace
            .update(cx, |workspace, cx| {
                workspace.open_path(b_path, None, true, cx)
            })
            .await
            .unwrap();
        let finder = open_file_picker(&workspace, cx);
        finder
            .update(cx, |f, cx| {
                f.delegate.spawn_search(test_path_like("a.txt"), cx)
            })
            .await;

        finder.update(cx, |f, _| {
            let delegate = &f.delegate;
            assert!(
                delegate.matches.history.is_empty(),
                "Search matches expected"
            );
            let matches = delegate.matches.search.clone();
            assert_eq!(matches[0].path.as_ref(), Path::new("dir2/a.txt"));
            assert_eq!(matches[1].path.as_ref(), Path::new("dir1/a.txt"));
        });
    }

    #[gpui::test]
    async fn test_search_worktree_without_files(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        app_state
            .fs
            .as_fake()
            .insert_tree(
                "/root",
                json!({
                    "dir1": {},
                    "dir2": {
                        "dir3": {}
                    }
                }),
            )
            .await;

        let project = Project::test(app_state.fs.clone(), ["/root".as_ref()], cx).await;
        let (picker, _workspace, cx) = build_find_picker(project, cx);

        picker
            .update(cx, |f, cx| {
                f.delegate.spawn_search(test_path_like("dir"), cx)
            })
            .await;
        cx.read(|cx| {
            let finder = picker.read(cx);
            assert_eq!(finder.delegate.matches.len(), 0);
        });
    }

    #[gpui::test]
    async fn test_query_history(cx: &mut gpui::TestAppContext) {
        let app_state = init_test(cx);

        app_state
            .fs
            .as_fake()
            .insert_tree(
                "/src",
                json!({
                    "test": {
                        "first.rs": "// First Rust file",
                        "second.rs": "// Second Rust file",
                        "third.rs": "// Third Rust file",
                    }
                }),
            )
            .await;

        let project = Project::test(app_state.fs.clone(), ["/src".as_ref()], cx).await;
        let (workspace, cx) = cx.add_window_view(|cx| Workspace::test_new(project, cx));
        let worktree_id = cx.read(|cx| {
            let worktrees = workspace.read(cx).worktrees(cx).collect::<Vec<_>>();
            assert_eq!(worktrees.len(), 1);
            WorktreeId::from_usize(worktrees[0].entity_id().as_u64() as usize)
        });

        // Open and close panels, getting their history items afterwards.
        // Ensure history items get populated with opened items, and items are kept in a certain order.
        // The history lags one opened buffer behind, since it's updated in the search panel only on its reopen.
        //
        // TODO: without closing, the opened items do not propagate their history changes for some reason
        // it does work in real app though, only tests do not propagate.
        workspace.update(cx, |_, cx| cx.focused());

        let initial_history = open_close_queried_buffer("fir", 1, "first.rs", &workspace, cx).await;
        assert!(
            initial_history.is_empty(),
            "Should have no history before opening any files"
        );

        let history_after_first =
            open_close_queried_buffer("sec", 1, "second.rs", &workspace, cx).await;
        assert_eq!(
            history_after_first,
            vec![FoundPath::new(
                ProjectPath {
                    worktree_id,
                    path: Arc::from(Path::new("test/first.rs")),
                },
                Some(PathBuf::from("/src/test/first.rs"))
            )],
            "Should show 1st opened item in the history when opening the 2nd item"
        );

        let history_after_second =
            open_close_queried_buffer("thi", 1, "third.rs", &workspace, cx).await;
        assert_eq!(
            history_after_second,
            vec![
                FoundPath::new(
                    ProjectPath {
                        worktree_id,
                        path: Arc::from(Path::new("test/second.rs")),
                    },
                    Some(PathBuf::from("/src/test/second.rs"))
                ),
                FoundPath::new(
                    ProjectPath {
                        worktree_id,
                        path: Arc::from(Path::new("test/first.rs")),
                    },
                    Some(PathBuf::from("/src/test/first.rs"))
                ),
            ],
            "Should show 1st and 2nd opened items in the history when opening the 3rd item. \
    2nd item should be the first in the history, as the last opened."
        );

        let history_after_third =
            open_close_queried_buffer("sec", 1, "second.rs", &workspace, cx).await;
        assert_eq!(
                history_after_third,
                vec![
                    FoundPath::new(
                        ProjectPath {
                            worktree_id,
                            path: Arc::from(Path::new("test/third.rs")),
                        },
                        Some(PathBuf::from("/src/test/third.rs"))
                    ),
                    FoundPath::new(
                        ProjectPath {
                            worktree_id,
                            path: Arc::from(Path::new("test/second.rs")),
                        },
                        Some(PathBuf::from("/src/test/second.rs"))
                    ),
                    FoundPath::new(
                        ProjectPath {
                            worktree_id,
                            path: Arc::from(Path::new("test/first.rs")),
                        },
                        Some(PathBuf::from("/src/test/first.rs"))
                    ),
                ],
                "Should show 1st, 2nd and 3rd opened items in the history when opening the 2nd item again. \
    3rd item should be the first in the history, as the last opened."
            );

        let history_after_second_again =
            open_close_queried_buffer("thi", 1, "third.rs", &workspace, cx).await;
        assert_eq!(
                history_after_second_again,
                vec![
                    FoundPath::new(
                        ProjectPath {
                            worktree_id,
                            path: Arc::from(Path::new("test/second.rs")),
                        },
                        Some(PathBuf::from("/src/test/second.rs"))
                    ),
                    FoundPath::new(
                        ProjectPath {
                            worktree_id,
                            path: Arc::from(Path::new("test/third.rs")),
                        },
                        Some(PathBuf::from("/src/test/third.rs"))
                    ),
                    FoundPath::new(
                        ProjectPath {
                            worktree_id,
                            path: Arc::from(Path::new("test/first.rs")),
                        },
                        Some(PathBuf::from("/src/test/first.rs"))
                    ),
                ],
                "Should show 1st, 2nd and 3rd opened items in the history when opening the 3rd item again. \
    2nd item, as the last opened, 3rd item should go next as it was opened right before."
            );
    }

    #[gpui::test]
    async fn test_external_files_history(cx: &mut gpui::TestAppContext) {
        let app_state = init_test(cx);

        app_state
            .fs
            .as_fake()
            .insert_tree(
                "/src",
                json!({
                    "test": {
                        "first.rs": "// First Rust file",
                        "second.rs": "// Second Rust file",
                    }
                }),
            )
            .await;

        app_state
            .fs
            .as_fake()
            .insert_tree(
                "/external-src",
                json!({
                    "test": {
                        "third.rs": "// Third Rust file",
                        "fourth.rs": "// Fourth Rust file",
                    }
                }),
            )
            .await;

        let project = Project::test(app_state.fs.clone(), ["/src".as_ref()], cx).await;
        cx.update(|cx| {
            project.update(cx, |project, cx| {
                project.find_or_create_local_worktree("/external-src", false, cx)
            })
        })
        .detach();
        cx.background_executor.run_until_parked();

        let (workspace, cx) = cx.add_window_view(|cx| Workspace::test_new(project, cx));
        let worktree_id = cx.read(|cx| {
            let worktrees = workspace.read(cx).worktrees(cx).collect::<Vec<_>>();
            assert_eq!(worktrees.len(), 1,);

            WorktreeId::from_usize(worktrees[0].entity_id().as_u64() as usize)
        });
        workspace
            .update(cx, |workspace, cx| {
                workspace.open_abs_path(PathBuf::from("/external-src/test/third.rs"), false, cx)
            })
            .detach();
        cx.background_executor.run_until_parked();
        let external_worktree_id = cx.read(|cx| {
            let worktrees = workspace.read(cx).worktrees(cx).collect::<Vec<_>>();
            assert_eq!(
                worktrees.len(),
                2,
                "External file should get opened in a new worktree"
            );

            WorktreeId::from_usize(
                worktrees
                    .into_iter()
                    .find(|worktree| {
                        worktree.entity_id().as_u64() as usize != worktree_id.to_usize()
                    })
                    .expect("New worktree should have a different id")
                    .entity_id()
                    .as_u64() as usize,
            )
        });
        cx.dispatch_action(workspace::CloseActiveItem { save_intent: None });

        let initial_history_items =
            open_close_queried_buffer("sec", 1, "second.rs", &workspace, cx).await;
        assert_eq!(
            initial_history_items,
            vec![FoundPath::new(
                ProjectPath {
                    worktree_id: external_worktree_id,
                    path: Arc::from(Path::new("")),
                },
                Some(PathBuf::from("/external-src/test/third.rs"))
            )],
            "Should show external file with its full path in the history after it was open"
        );

        let updated_history_items =
            open_close_queried_buffer("fir", 1, "first.rs", &workspace, cx).await;
        assert_eq!(
            updated_history_items,
            vec![
                FoundPath::new(
                    ProjectPath {
                        worktree_id,
                        path: Arc::from(Path::new("test/second.rs")),
                    },
                    Some(PathBuf::from("/src/test/second.rs"))
                ),
                FoundPath::new(
                    ProjectPath {
                        worktree_id: external_worktree_id,
                        path: Arc::from(Path::new("")),
                    },
                    Some(PathBuf::from("/external-src/test/third.rs"))
                ),
            ],
            "Should keep external file with history updates",
        );
    }

    #[gpui::test]
    async fn test_toggle_panel_new_selections(cx: &mut gpui::TestAppContext) {
        let app_state = init_test(cx);

        app_state
            .fs
            .as_fake()
            .insert_tree(
                "/src",
                json!({
                    "test": {
                        "first.rs": "// First Rust file",
                        "second.rs": "// Second Rust file",
                        "third.rs": "// Third Rust file",
                    }
                }),
            )
            .await;

        let project = Project::test(app_state.fs.clone(), ["/src".as_ref()], cx).await;
        let (workspace, cx) = cx.add_window_view(|cx| Workspace::test_new(project, cx));

        // generate some history to select from
        open_close_queried_buffer("fir", 1, "first.rs", &workspace, cx).await;
        cx.executor().run_until_parked();
        open_close_queried_buffer("sec", 1, "second.rs", &workspace, cx).await;
        open_close_queried_buffer("thi", 1, "third.rs", &workspace, cx).await;
        let current_history =
            open_close_queried_buffer("sec", 1, "second.rs", &workspace, cx).await;

        for expected_selected_index in 0..current_history.len() {
            cx.dispatch_action(Toggle);
            let picker = active_file_picker(&workspace, cx);
            let selected_index = picker.update(cx, |picker, _| picker.delegate.selected_index());
            assert_eq!(
                selected_index, expected_selected_index,
                "Should select the next item in the history"
            );
        }

        cx.dispatch_action(Toggle);
        let selected_index = workspace.update(cx, |workspace, cx| {
            workspace
                .active_modal::<FileFinder>(cx)
                .unwrap()
                .read(cx)
                .picker
                .read(cx)
                .delegate
                .selected_index()
        });
        assert_eq!(
            selected_index, 0,
            "Should wrap around the history and start all over"
        );
    }

    #[gpui::test]
    async fn test_search_preserves_history_items(cx: &mut gpui::TestAppContext) {
        let app_state = init_test(cx);

        app_state
            .fs
            .as_fake()
            .insert_tree(
                "/src",
                json!({
                    "test": {
                        "first.rs": "// First Rust file",
                        "second.rs": "// Second Rust file",
                        "third.rs": "// Third Rust file",
                        "fourth.rs": "// Fourth Rust file",
                    }
                }),
            )
            .await;

        let project = Project::test(app_state.fs.clone(), ["/src".as_ref()], cx).await;
        let (workspace, cx) = cx.add_window_view(|cx| Workspace::test_new(project, cx));
        let worktree_id = cx.read(|cx| {
            let worktrees = workspace.read(cx).worktrees(cx).collect::<Vec<_>>();
            assert_eq!(worktrees.len(), 1,);

            WorktreeId::from_usize(worktrees[0].entity_id().as_u64() as usize)
        });

        // generate some history to select from
        open_close_queried_buffer("fir", 1, "first.rs", &workspace, cx).await;
        open_close_queried_buffer("sec", 1, "second.rs", &workspace, cx).await;
        open_close_queried_buffer("thi", 1, "third.rs", &workspace, cx).await;
        open_close_queried_buffer("sec", 1, "second.rs", &workspace, cx).await;

        let finder = open_file_picker(&workspace, cx);
        let first_query = "f";
        finder
            .update(cx, |finder, cx| {
                finder.delegate.update_matches(first_query.to_string(), cx)
            })
            .await;
        finder.update(cx, |finder, _| {
            let delegate = &finder.delegate;
            assert_eq!(delegate.matches.history.len(), 1, "Only one history item contains {first_query}, it should be present and others should be filtered out");
            let history_match = delegate.matches.history.first().unwrap();
            assert!(history_match.1.is_some(), "Should have path matches for history items after querying");
            assert_eq!(history_match.0, FoundPath::new(
                ProjectPath {
                    worktree_id,
                    path: Arc::from(Path::new("test/first.rs")),
                },
                Some(PathBuf::from("/src/test/first.rs"))
            ));
            assert_eq!(delegate.matches.search.len(), 1, "Only one non-history item contains {first_query}, it should be present");
            assert_eq!(delegate.matches.search.first().unwrap().path.as_ref(), Path::new("test/fourth.rs"));
        });

        let second_query = "fsdasdsa";
        let finder = active_file_picker(&workspace, cx);
        finder
            .update(cx, |finder, cx| {
                finder.delegate.update_matches(second_query.to_string(), cx)
            })
            .await;
        finder.update(cx, |finder, _| {
            let delegate = &finder.delegate;
            assert!(
                delegate.matches.history.is_empty(),
                "No history entries should match {second_query}"
            );
            assert!(
                delegate.matches.search.is_empty(),
                "No search entries should match {second_query}"
            );
        });

        let first_query_again = first_query;

        let finder = active_file_picker(&workspace, cx);
        finder
            .update(cx, |finder, cx| {
                finder
                    .delegate
                    .update_matches(first_query_again.to_string(), cx)
            })
            .await;
        finder.update(cx, |finder, _| {
            let delegate = &finder.delegate;
            assert_eq!(delegate.matches.history.len(), 1, "Only one history item contains {first_query_again}, it should be present and others should be filtered out, even after non-matching query");
            let history_match = delegate.matches.history.first().unwrap();
            assert!(history_match.1.is_some(), "Should have path matches for history items after querying");
            assert_eq!(history_match.0, FoundPath::new(
                ProjectPath {
                    worktree_id,
                    path: Arc::from(Path::new("test/first.rs")),
                },
                Some(PathBuf::from("/src/test/first.rs"))
            ));
            assert_eq!(delegate.matches.search.len(), 1, "Only one non-history item contains {first_query_again}, it should be present, even after non-matching query");
            assert_eq!(delegate.matches.search.first().unwrap().path.as_ref(), Path::new("test/fourth.rs"));
        });
    }

    #[gpui::test]
    async fn test_history_items_vs_very_good_external_match(cx: &mut gpui::TestAppContext) {
        let app_state = init_test(cx);

        app_state
            .fs
            .as_fake()
            .insert_tree(
                "/src",
                json!({
                    "collab_ui": {
                        "first.rs": "// First Rust file",
                        "second.rs": "// Second Rust file",
                        "third.rs": "// Third Rust file",
                        "collab_ui.rs": "// Fourth Rust file",
                    }
                }),
            )
            .await;

        let project = Project::test(app_state.fs.clone(), ["/src".as_ref()], cx).await;
        let (workspace, cx) = cx.add_window_view(|cx| Workspace::test_new(project, cx));
        // generate some history to select from
        open_close_queried_buffer("fir", 1, "first.rs", &workspace, cx).await;
        open_close_queried_buffer("sec", 1, "second.rs", &workspace, cx).await;
        open_close_queried_buffer("thi", 1, "third.rs", &workspace, cx).await;
        open_close_queried_buffer("sec", 1, "second.rs", &workspace, cx).await;

        let finder = open_file_picker(&workspace, cx);
        let query = "collab_ui";
        cx.simulate_input(query);
        finder.update(cx, |finder, _| {
            let delegate = &finder.delegate;
            assert!(
                delegate.matches.history.is_empty(),
                "History items should not math query {query}, they should be matched by name only"
            );

            let search_entries = delegate
                .matches
                .search
                .iter()
                .map(|path_match| path_match.path.to_path_buf())
                .collect::<Vec<_>>();
            assert_eq!(
                search_entries,
                vec![
                    PathBuf::from("collab_ui/collab_ui.rs"),
                    PathBuf::from("collab_ui/third.rs"),
                    PathBuf::from("collab_ui/first.rs"),
                    PathBuf::from("collab_ui/second.rs"),
                ],
                "Despite all search results having the same directory name, the most matching one should be on top"
            );
        });
    }

    #[gpui::test]
    async fn test_nonexistent_history_items_not_shown(cx: &mut gpui::TestAppContext) {
        let app_state = init_test(cx);

        app_state
            .fs
            .as_fake()
            .insert_tree(
                "/src",
                json!({
                    "test": {
                        "first.rs": "// First Rust file",
                        "nonexistent.rs": "// Second Rust file",
                        "third.rs": "// Third Rust file",
                    }
                }),
            )
            .await;

        let project = Project::test(app_state.fs.clone(), ["/src".as_ref()], cx).await;
        let (workspace, cx) = cx.add_window_view(|cx| Workspace::test_new(project, cx)); // generate some history to select from
        open_close_queried_buffer("fir", 1, "first.rs", &workspace, cx).await;
        open_close_queried_buffer("non", 1, "nonexistent.rs", &workspace, cx).await;
        open_close_queried_buffer("thi", 1, "third.rs", &workspace, cx).await;
        open_close_queried_buffer("fir", 1, "first.rs", &workspace, cx).await;

        let picker = open_file_picker(&workspace, cx);
        cx.simulate_input("rs");

        picker.update(cx, |finder, _| {
            let history_entries = finder.delegate
                .matches
                .history
                .iter()
                .map(|(_, path_match)| path_match.as_ref().expect("should have a path match").path.to_path_buf())
                .collect::<Vec<_>>();
            assert_eq!(
                history_entries,
                vec![
                    PathBuf::from("test/first.rs"),
                    PathBuf::from("test/third.rs"),
                ],
                "Should have all opened files in the history, except the ones that do not exist on disk"
            );
        });
    }

    async fn open_close_queried_buffer(
        input: &str,
        expected_matches: usize,
        expected_editor_title: &str,
        workspace: &View<Workspace>,
        cx: &mut gpui::VisualTestContext,
    ) -> Vec<FoundPath> {
        let picker = open_file_picker(&workspace, cx);
        cx.simulate_input(input);

        let history_items = picker.update(cx, |finder, _| {
            assert_eq!(
                finder.delegate.matches.len(),
                expected_matches,
                "Unexpected number of matches found for query {input}"
            );
            finder.delegate.history_items.clone()
        });

        cx.dispatch_action(SelectNext);
        cx.dispatch_action(Confirm);

        cx.read(|cx| {
            let active_editor = workspace.read(cx).active_item_as::<Editor>(cx).unwrap();
            let active_editor_title = active_editor.read(cx).title(cx);
            assert_eq!(
                expected_editor_title, active_editor_title,
                "Unexpected editor title for query {input}"
            );
        });

        cx.dispatch_action(workspace::CloseActiveItem { save_intent: None });

        history_items
    }

    fn init_test(cx: &mut TestAppContext) -> Arc<AppState> {
        cx.update(|cx| {
            let state = AppState::test(cx);
            theme::init(theme::LoadThemes::JustBase, cx);
            language::init(cx);
            super::init(cx);
            editor::init(cx);
            workspace::init_settings(cx);
            Project::init_settings(cx);
            state
        })
    }

    fn test_path_like(test_str: &str) -> PathLikeWithPosition<FileSearchQuery> {
        PathLikeWithPosition::parse_str(test_str, |path_like_str| {
            Ok::<_, std::convert::Infallible>(FileSearchQuery {
                raw_query: test_str.to_owned(),
                file_query_end: if path_like_str == test_str {
                    None
                } else {
                    Some(path_like_str.len())
                },
            })
        })
        .unwrap()
    }

    fn build_find_picker(
        project: Model<Project>,
        cx: &mut TestAppContext,
    ) -> (
        View<Picker<FileFinderDelegate>>,
        View<Workspace>,
        &mut VisualTestContext,
    ) {
        let (workspace, cx) = cx.add_window_view(|cx| Workspace::test_new(project, cx));
        let picker = open_file_picker(&workspace, cx);
        (picker, workspace, cx)
    }

    #[track_caller]
    fn open_file_picker(
        workspace: &View<Workspace>,
        cx: &mut VisualTestContext,
    ) -> View<Picker<FileFinderDelegate>> {
        cx.dispatch_action(Toggle);
        active_file_picker(workspace, cx)
    }

    #[track_caller]
    fn active_file_picker(
        workspace: &View<Workspace>,
        cx: &mut VisualTestContext,
    ) -> View<Picker<FileFinderDelegate>> {
        workspace.update(cx, |workspace, cx| {
            workspace
                .active_modal::<FileFinder>(cx)
                .unwrap()
                .read(cx)
                .picker
                .clone()
        })
    }

    fn collect_search_results(picker: &Picker<FileFinderDelegate>) -> Vec<PathBuf> {
        let matches = &picker.delegate.matches;
        assert!(
            matches.history.is_empty(),
            "Should have no history matches, but got: {:?}",
            matches.history
        );
        let mut results = matches
            .search
            .iter()
            .map(|path_match| Path::new(path_match.path_prefix.as_ref()).join(&path_match.path))
            .collect::<Vec<_>>();
        results.sort();
        results
    }
}
