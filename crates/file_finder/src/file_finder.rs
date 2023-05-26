use editor::{scroll::autoscroll::Autoscroll, Bias, Editor};
use fuzzy::PathMatch;
use gpui::{
    actions, elements::*, AppContext, ModelHandle, MouseState, Task, ViewContext, WeakViewHandle,
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
use util::{paths::PathLikeWithPosition, post_inc, ResultExt};
use workspace::Workspace;

pub type FileFinder = Picker<FileFinderDelegate>;

pub struct FileFinderDelegate {
    workspace: WeakViewHandle<Workspace>,
    project: ModelHandle<Project>,
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

#[derive(Debug)]
enum Matches {
    History(Vec<FoundPath>),
    Search(Vec<PathMatch>),
}

#[derive(Debug)]
enum Match<'a> {
    History(&'a FoundPath),
    Search(&'a PathMatch),
}

impl Matches {
    fn len(&self) -> usize {
        match self {
            Self::History(items) => items.len(),
            Self::Search(items) => items.len(),
        }
    }

    fn get(&self, index: usize) -> Option<Match<'_>> {
        match self {
            Self::History(items) => items.get(index).map(Match::History),
            Self::Search(items) => items.get(index).map(Match::Search),
        }
    }
}

impl Default for Matches {
    fn default() -> Self {
        Self::History(Vec::new())
    }
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

actions!(file_finder, [Toggle]);

pub fn init(cx: &mut AppContext) {
    cx.add_action(toggle_file_finder);
    FileFinder::init(cx);
}

const MAX_RECENT_SELECTIONS: usize = 20;

fn toggle_file_finder(workspace: &mut Workspace, _: &Toggle, cx: &mut ViewContext<Workspace>) {
    workspace.toggle_modal(cx, |workspace, cx| {
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
                    .map(|(history_path, abs_path)| FoundPath::new(history_path, abs_path)),
            )
            .collect();

        let project = workspace.project().clone();
        let workspace = cx.handle().downgrade();
        let finder = cx.add_view(|cx| {
            Picker::new(
                FileFinderDelegate::new(
                    workspace,
                    project,
                    currently_opened_path,
                    history_items,
                    cx,
                ),
                cx,
            )
        });
        finder
    });
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
        workspace: WeakViewHandle<Workspace>,
        project: ModelHandle<Project>,
        currently_opened_path: Option<FoundPath>,
        history_items: Vec<FoundPath>,
        cx: &mut ViewContext<FileFinder>,
    ) -> Self {
        cx.observe(&project, |picker, _, cx| {
            picker.update_matches(picker.query(cx), cx);
        })
        .detach();
        Self {
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
        cx: &mut ViewContext<FileFinder>,
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
                cx.background(),
            )
            .await;
            let did_cancel = cancel_flag.load(atomic::Ordering::Relaxed);
            picker
                .update(&mut cx, |picker, cx| {
                    picker
                        .delegate_mut()
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
        cx: &mut ViewContext<FileFinder>,
    ) {
        if search_id >= self.latest_search_id {
            self.latest_search_id = search_id;
            if self.latest_search_did_cancel
                && Some(query.path_like.path_query())
                    == self
                        .latest_search_query
                        .as_ref()
                        .map(|query| query.path_like.path_query())
            {
                match &mut self.matches {
                    Matches::History(_) => self.matches = Matches::Search(matches),
                    Matches::Search(search_matches) => {
                        util::extend_sorted(search_matches, matches.into_iter(), 100, |a, b| {
                            b.cmp(a)
                        })
                    }
                }
            } else {
                self.matches = Matches::Search(matches);
            }
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
            Match::History(found_path) => {
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
                self.labels_for_path_match(&PathMatch {
                    score: ix as f64,
                    positions: Vec::new(),
                    worktree_id: worktree_id.to_usize(),
                    path,
                    path_prefix: "".into(),
                    distance_to_relative_ancestor: usize::MAX,
                })
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
        let file_name_start = path_match.path_prefix.chars().count() + path_string.chars().count()
            - file_name.chars().count();
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
    fn placeholder_text(&self) -> Arc<str> {
        "Search project files...".into()
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index.unwrap_or(0)
    }

    fn set_selected_index(&mut self, ix: usize, cx: &mut ViewContext<FileFinder>) {
        self.selected_index = Some(ix);
        cx.notify();
    }

    fn update_matches(&mut self, raw_query: String, cx: &mut ViewContext<FileFinder>) -> Task<()> {
        if raw_query.is_empty() {
            let project = self.project.read(cx);
            self.latest_search_id = post_inc(&mut self.search_count);
            self.matches = Matches::History(
                self.history_items
                    .iter()
                    .filter(|history_item| {
                        project
                            .worktree_for_id(history_item.project.worktree_id, cx)
                            .is_some()
                            || (project.is_local()
                                && history_item
                                    .absolute
                                    .as_ref()
                                    .filter(|abs_path| abs_path.exists())
                                    .is_some())
                    })
                    .cloned()
                    .collect(),
            );
            cx.notify();
            Task::ready(())
        } else {
            let raw_query = &raw_query;
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

    fn confirm(&mut self, cx: &mut ViewContext<FileFinder>) {
        if let Some(m) = self.matches.get(self.selected_index()) {
            if let Some(workspace) = self.workspace.upgrade(cx) {
                let open_task = workspace.update(cx, |workspace, cx| match m {
                    Match::History(history_match) => {
                        let worktree_id = history_match.project.worktree_id;
                        if workspace
                            .project()
                            .read(cx)
                            .worktree_for_id(worktree_id, cx)
                            .is_some()
                        {
                            workspace.open_path(
                                ProjectPath {
                                    worktree_id,
                                    path: Arc::clone(&history_match.project.path),
                                },
                                None,
                                true,
                                cx,
                            )
                        } else {
                            match history_match.absolute.as_ref() {
                                Some(abs_path) => {
                                    workspace.open_abs_path(abs_path.to_path_buf(), false, cx)
                                }
                                None => workspace.open_path(
                                    ProjectPath {
                                        worktree_id,
                                        path: Arc::clone(&history_match.project.path),
                                    },
                                    None,
                                    true,
                                    cx,
                                ),
                            }
                        }
                    }
                    Match::Search(m) => workspace.open_path(
                        ProjectPath {
                            worktree_id: WorktreeId::from_usize(m.worktree_id),
                            path: m.path.clone(),
                        },
                        None,
                        true,
                        cx,
                    ),
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
                    workspace
                        .downgrade()
                        .update(&mut cx, |workspace, cx| workspace.dismiss_modal(cx))
                        .log_err();

                    Some(())
                })
                .detach();
            }
        }
    }

    fn dismissed(&mut self, _: &mut ViewContext<FileFinder>) {}

    fn render_match(
        &self,
        ix: usize,
        mouse_state: &mut MouseState,
        selected: bool,
        cx: &AppContext,
    ) -> AnyElement<Picker<Self>> {
        let path_match = self
            .matches
            .get(ix)
            .expect("Invalid matches state: no element for index {ix}");
        let theme = theme::current(cx);
        let style = theme.picker.item.style_for(mouse_state, selected);
        let (file_name, file_name_positions, full_path, full_path_positions) =
            self.labels_for_match(path_match, cx, ix);
        Flex::column()
            .with_child(
                Label::new(file_name, style.label.clone()).with_highlights(file_name_positions),
            )
            .with_child(
                Label::new(full_path, style.label.clone()).with_highlights(full_path_positions),
            )
            .flex(1., false)
            .contained()
            .with_style(style.container)
            .into_any_named("match")
    }
}

#[cfg(test)]
mod tests {
    use std::{assert_eq, collections::HashMap, path::Path, time::Duration};

    use super::*;
    use editor::Editor;
    use gpui::{TestAppContext, ViewHandle};
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
        let (window_id, workspace) = cx.add_window(|cx| Workspace::test_new(project, cx));
        cx.dispatch_action(window_id, Toggle);

        let finder = cx.read(|cx| workspace.read(cx).modal::<FileFinder>().unwrap());
        finder
            .update(cx, |finder, cx| {
                finder.delegate_mut().update_matches("bna".to_string(), cx)
            })
            .await;
        finder.read_with(cx, |finder, _| {
            assert_eq!(finder.delegate().matches.len(), 2);
        });

        let active_pane = cx.read(|cx| workspace.read(cx).active_pane().clone());
        cx.dispatch_action(window_id, SelectNext);
        cx.dispatch_action(window_id, Confirm);
        active_pane
            .condition(cx, |pane, _| pane.active_item().is_some())
            .await;
        cx.read(|cx| {
            let active_item = active_pane.read(cx).active_item().unwrap();
            assert_eq!(
                active_item
                    .as_any()
                    .downcast_ref::<Editor>()
                    .unwrap()
                    .read(cx)
                    .title(cx),
                "bandana"
            );
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
        let (window_id, workspace) = cx.add_window(|cx| Workspace::test_new(project, cx));
        cx.dispatch_action(window_id, Toggle);
        let finder = cx.read(|cx| workspace.read(cx).modal::<FileFinder>().unwrap());

        let file_query = &first_file_name[..3];
        let file_row = 1;
        let file_column = 3;
        assert!(file_column <= first_file_contents.len());
        let query_inside_file = format!("{file_query}:{file_row}:{file_column}");
        finder
            .update(cx, |finder, cx| {
                finder
                    .delegate_mut()
                    .update_matches(query_inside_file.to_string(), cx)
            })
            .await;
        finder.read_with(cx, |finder, _| {
            let finder = finder.delegate();
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

        let active_pane = cx.read(|cx| workspace.read(cx).active_pane().clone());
        cx.dispatch_action(window_id, SelectNext);
        cx.dispatch_action(window_id, Confirm);
        active_pane
            .condition(cx, |pane, _| pane.active_item().is_some())
            .await;
        let editor = cx.update(|cx| {
            let active_item = active_pane.read(cx).active_item().unwrap();
            active_item.downcast::<Editor>().unwrap()
        });
        cx.foreground().advance_clock(Duration::from_secs(2));
        cx.foreground().start_waiting();
        cx.foreground().finish_waiting();
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
        let (window_id, workspace) = cx.add_window(|cx| Workspace::test_new(project, cx));
        cx.dispatch_action(window_id, Toggle);
        let finder = cx.read(|cx| workspace.read(cx).modal::<FileFinder>().unwrap());

        let file_query = &first_file_name[..3];
        let file_row = 200;
        let file_column = 300;
        assert!(file_column > first_file_contents.len());
        let query_outside_file = format!("{file_query}:{file_row}:{file_column}");
        finder
            .update(cx, |finder, cx| {
                finder
                    .delegate_mut()
                    .update_matches(query_outside_file.to_string(), cx)
            })
            .await;
        finder.read_with(cx, |finder, _| {
            let finder = finder.delegate();
            assert_eq!(finder.matches.len(), 1);
            let latest_search_query = finder
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

        let active_pane = cx.read(|cx| workspace.read(cx).active_pane().clone());
        cx.dispatch_action(window_id, SelectNext);
        cx.dispatch_action(window_id, Confirm);
        active_pane
            .condition(cx, |pane, _| pane.active_item().is_some())
            .await;
        let editor = cx.update(|cx| {
            let active_item = active_pane.read(cx).active_item().unwrap();
            active_item.downcast::<Editor>().unwrap()
        });
        cx.foreground().advance_clock(Duration::from_secs(2));
        cx.foreground().start_waiting();
        cx.foreground().finish_waiting();
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
        let (_, workspace) = cx.add_window(|cx| Workspace::test_new(project, cx));
        let (_, finder) = cx.add_window(|cx| {
            Picker::new(
                FileFinderDelegate::new(
                    workspace.downgrade(),
                    workspace.read(cx).project().clone(),
                    None,
                    Vec::new(),
                    cx,
                ),
                cx,
            )
        });

        let query = test_path_like("hi");
        finder
            .update(cx, |f, cx| f.delegate_mut().spawn_search(query.clone(), cx))
            .await;
        finder.read_with(cx, |f, _| assert_eq!(f.delegate().matches.len(), 5));

        finder.update(cx, |finder, cx| {
            let delegate = finder.delegate_mut();
            let matches = match &delegate.matches {
                Matches::Search(path_matches) => path_matches,
                _ => panic!("Search matches expected"),
            }
            .clone();

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

            match &delegate.matches {
                Matches::Search(new_matches) => {
                    assert_eq!(new_matches.as_slice(), &matches[0..4])
                }
                _ => panic!("Search matches expected"),
            };
        });
    }

    #[gpui::test]
    async fn test_ignored_files(cx: &mut TestAppContext) {
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
        let (_, workspace) = cx.add_window(|cx| Workspace::test_new(project, cx));
        let (_, finder) = cx.add_window(|cx| {
            Picker::new(
                FileFinderDelegate::new(
                    workspace.downgrade(),
                    workspace.read(cx).project().clone(),
                    None,
                    Vec::new(),
                    cx,
                ),
                cx,
            )
        });
        finder
            .update(cx, |f, cx| {
                f.delegate_mut().spawn_search(test_path_like("hi"), cx)
            })
            .await;
        finder.read_with(cx, |f, _| assert_eq!(f.delegate().matches.len(), 7));
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
        let (_, workspace) = cx.add_window(|cx| Workspace::test_new(project, cx));
        let (_, finder) = cx.add_window(|cx| {
            Picker::new(
                FileFinderDelegate::new(
                    workspace.downgrade(),
                    workspace.read(cx).project().clone(),
                    None,
                    Vec::new(),
                    cx,
                ),
                cx,
            )
        });

        // Even though there is only one worktree, that worktree's filename
        // is included in the matching, because the worktree is a single file.
        finder
            .update(cx, |f, cx| {
                f.delegate_mut().spawn_search(test_path_like("thf"), cx)
            })
            .await;
        cx.read(|cx| {
            let finder = finder.read(cx);
            let delegate = finder.delegate();
            let matches = match &delegate.matches {
                Matches::Search(path_matches) => path_matches,
                _ => panic!("Search matches expected"),
            };
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
        finder
            .update(cx, |f, cx| {
                f.delegate_mut().spawn_search(test_path_like("thf/"), cx)
            })
            .await;
        finder.read_with(cx, |f, _| assert_eq!(f.delegate().matches.len(), 0));
    }

    #[gpui::test]
    async fn test_multiple_matches_with_same_relative_path(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        app_state
            .fs
            .as_fake()
            .insert_tree(
                "/root",
                json!({
                    "dir1": { "a.txt": "" },
                    "dir2": { "a.txt": "" }
                }),
            )
            .await;

        let project = Project::test(
            app_state.fs.clone(),
            ["/root/dir1".as_ref(), "/root/dir2".as_ref()],
            cx,
        )
        .await;
        let (_, workspace) = cx.add_window(|cx| Workspace::test_new(project, cx));

        let (_, finder) = cx.add_window(|cx| {
            Picker::new(
                FileFinderDelegate::new(
                    workspace.downgrade(),
                    workspace.read(cx).project().clone(),
                    None,
                    Vec::new(),
                    cx,
                ),
                cx,
            )
        });

        // Run a search that matches two files with the same relative path.
        finder
            .update(cx, |f, cx| {
                f.delegate_mut().spawn_search(test_path_like("a.t"), cx)
            })
            .await;

        // Can switch between different matches with the same relative path.
        finder.update(cx, |finder, cx| {
            let delegate = finder.delegate_mut();
            assert_eq!(delegate.matches.len(), 2);
            assert_eq!(delegate.selected_index(), 0);
            delegate.set_selected_index(1, cx);
            assert_eq!(delegate.selected_index(), 1);
            delegate.set_selected_index(0, cx);
            assert_eq!(delegate.selected_index(), 0);
        });
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
        let (_, workspace) = cx.add_window(|cx| Workspace::test_new(project, cx));
        let worktree_id = cx.read(|cx| {
            let worktrees = workspace.read(cx).worktrees(cx).collect::<Vec<_>>();
            assert_eq!(worktrees.len(), 1);
            WorktreeId::from_usize(worktrees[0].id())
        });

        // When workspace has an active item, sort items which are closer to that item
        // first when they have the same name. In this case, b.txt is closer to dir2's a.txt
        // so that one should be sorted earlier
        let b_path = Some(dummy_found_path(ProjectPath {
            worktree_id,
            path: Arc::from(Path::new("/root/dir2/b.txt")),
        }));
        let (_, finder) = cx.add_window(|cx| {
            Picker::new(
                FileFinderDelegate::new(
                    workspace.downgrade(),
                    workspace.read(cx).project().clone(),
                    b_path,
                    Vec::new(),
                    cx,
                ),
                cx,
            )
        });

        finder
            .update(cx, |f, cx| {
                f.delegate_mut().spawn_search(test_path_like("a.txt"), cx)
            })
            .await;

        finder.read_with(cx, |f, _| {
            let delegate = f.delegate();
            let matches = match &delegate.matches {
                Matches::Search(path_matches) => path_matches,
                _ => panic!("Search matches expected"),
            };
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
        let (_, workspace) = cx.add_window(|cx| Workspace::test_new(project, cx));
        let (_, finder) = cx.add_window(|cx| {
            Picker::new(
                FileFinderDelegate::new(
                    workspace.downgrade(),
                    workspace.read(cx).project().clone(),
                    None,
                    Vec::new(),
                    cx,
                ),
                cx,
            )
        });
        finder
            .update(cx, |f, cx| {
                f.delegate_mut().spawn_search(test_path_like("dir"), cx)
            })
            .await;
        cx.read(|cx| {
            let finder = finder.read(cx);
            assert_eq!(finder.delegate().matches.len(), 0);
        });
    }

    #[gpui::test]
    async fn test_query_history(
        deterministic: Arc<gpui::executor::Deterministic>,
        cx: &mut gpui::TestAppContext,
    ) {
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
        let (window_id, workspace) = cx.add_window(|cx| Workspace::test_new(project, cx));
        let worktree_id = cx.read(|cx| {
            let worktrees = workspace.read(cx).worktrees(cx).collect::<Vec<_>>();
            assert_eq!(worktrees.len(), 1);
            WorktreeId::from_usize(worktrees[0].id())
        });

        // Open and close panels, getting their history items afterwards.
        // Ensure history items get populated with opened items, and items are kept in a certain order.
        // The history lags one opened buffer behind, since it's updated in the search panel only on its reopen.
        //
        // TODO: without closing, the opened items do not propagate their history changes for some reason
        // it does work in real app though, only tests do not propagate.

        let initial_history = open_close_queried_buffer(
            "fir",
            1,
            "first.rs",
            window_id,
            &workspace,
            &deterministic,
            cx,
        )
        .await;
        assert!(
            initial_history.is_empty(),
            "Should have no history before opening any files"
        );

        let history_after_first = open_close_queried_buffer(
            "sec",
            1,
            "second.rs",
            window_id,
            &workspace,
            &deterministic,
            cx,
        )
        .await;
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

        let history_after_second = open_close_queried_buffer(
            "thi",
            1,
            "third.rs",
            window_id,
            &workspace,
            &deterministic,
            cx,
        )
        .await;
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

        let history_after_third = open_close_queried_buffer(
            "sec",
            1,
            "second.rs",
            window_id,
            &workspace,
            &deterministic,
            cx,
        )
        .await;
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

        let history_after_second_again = open_close_queried_buffer(
            "thi",
            1,
            "third.rs",
            window_id,
            &workspace,
            &deterministic,
            cx,
        )
        .await;
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
    async fn test_external_files_history(
        deterministic: Arc<gpui::executor::Deterministic>,
        cx: &mut gpui::TestAppContext,
    ) {
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
        deterministic.run_until_parked();

        let (window_id, workspace) = cx.add_window(|cx| Workspace::test_new(project, cx));
        let worktree_id = cx.read(|cx| {
            let worktrees = workspace.read(cx).worktrees(cx).collect::<Vec<_>>();
            assert_eq!(worktrees.len(), 1,);

            WorktreeId::from_usize(worktrees[0].id())
        });
        workspace
            .update(cx, |workspace, cx| {
                workspace.open_abs_path(PathBuf::from("/external-src/test/third.rs"), false, cx)
            })
            .detach();
        deterministic.run_until_parked();
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
                    .find(|worktree| worktree.id() != worktree_id.to_usize())
                    .expect("New worktree should have a different id")
                    .id(),
            )
        });
        close_active_item(&workspace, &deterministic, cx).await;

        let initial_history_items = open_close_queried_buffer(
            "sec",
            1,
            "second.rs",
            window_id,
            &workspace,
            &deterministic,
            cx,
        )
        .await;
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

        let updated_history_items = open_close_queried_buffer(
            "fir",
            1,
            "first.rs",
            window_id,
            &workspace,
            &deterministic,
            cx,
        )
        .await;
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

    async fn open_close_queried_buffer(
        input: &str,
        expected_matches: usize,
        expected_editor_title: &str,
        window_id: usize,
        workspace: &ViewHandle<Workspace>,
        deterministic: &gpui::executor::Deterministic,
        cx: &mut gpui::TestAppContext,
    ) -> Vec<FoundPath> {
        cx.dispatch_action(window_id, Toggle);
        let finder = cx.read(|cx| workspace.read(cx).modal::<FileFinder>().unwrap());
        finder
            .update(cx, |finder, cx| {
                finder.delegate_mut().update_matches(input.to_string(), cx)
            })
            .await;
        let history_items = finder.read_with(cx, |finder, _| {
            assert_eq!(
                finder.delegate().matches.len(),
                expected_matches,
                "Unexpected number of matches found for query {input}"
            );
            finder.delegate().history_items.clone()
        });

        let active_pane = cx.read(|cx| workspace.read(cx).active_pane().clone());
        cx.dispatch_action(window_id, SelectNext);
        cx.dispatch_action(window_id, Confirm);
        deterministic.run_until_parked();
        active_pane
            .condition(cx, |pane, _| pane.active_item().is_some())
            .await;
        cx.read(|cx| {
            let active_item = active_pane.read(cx).active_item().unwrap();
            let active_editor_title = active_item
                .as_any()
                .downcast_ref::<Editor>()
                .unwrap()
                .read(cx)
                .title(cx);
            assert_eq!(
                expected_editor_title, active_editor_title,
                "Unexpected editor title for query {input}"
            );
        });

        close_active_item(workspace, deterministic, cx).await;

        history_items
    }

    async fn close_active_item(
        workspace: &ViewHandle<Workspace>,
        deterministic: &gpui::executor::Deterministic,
        cx: &mut TestAppContext,
    ) {
        let mut original_items = HashMap::new();
        cx.read(|cx| {
            for pane in workspace.read(cx).panes() {
                let pane_id = pane.id();
                let pane = pane.read(cx);
                let insertion_result = original_items.insert(pane_id, pane.items().count());
                assert!(insertion_result.is_none(), "Pane id {pane_id} collision");
            }
        });

        let active_pane = cx.read(|cx| workspace.read(cx).active_pane().clone());
        active_pane
            .update(cx, |pane, cx| {
                pane.close_active_item(&workspace::CloseActiveItem, cx)
                    .unwrap()
            })
            .await
            .unwrap();
        deterministic.run_until_parked();
        cx.read(|cx| {
            for pane in workspace.read(cx).panes() {
                let pane_id = pane.id();
                let pane = pane.read(cx);
                match original_items.remove(&pane_id) {
                    Some(original_items) => {
                        assert_eq!(
                            pane.items().count(),
                            original_items.saturating_sub(1),
                            "Pane id {pane_id} should have item closed"
                        );
                    }
                    None => panic!("Pane id {pane_id} not found in original items"),
                }
            }
        });
        assert!(
            original_items.len() <= 1,
            "At most one panel should got closed"
        );
    }

    fn init_test(cx: &mut TestAppContext) -> Arc<AppState> {
        cx.foreground().forbid_parking();
        cx.update(|cx| {
            let state = AppState::test(cx);
            theme::init((), cx);
            language::init(cx);
            super::init(cx);
            editor::init(cx);
            workspace::init_settings(cx);
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

    fn dummy_found_path(project_path: ProjectPath) -> FoundPath {
        FoundPath {
            project: project_path,
            absolute: None,
        }
    }
}
