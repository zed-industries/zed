#[cfg(test)]
mod file_finder_tests;

mod new_path_prompt;

use collections::{BTreeSet, HashMap};
use editor::{scroll::Autoscroll, Bias, Editor};
use fuzzy::{CharBag, PathMatch, PathMatchCandidate};
use gpui::{
    actions, impl_actions, rems, Action, AnyElement, AppContext, DismissEvent, EventEmitter,
    FocusHandle, FocusableView, Model, Modifiers, ModifiersChangedEvent, ParentElement, Render,
    Styled, Task, View, ViewContext, VisualContext, WeakView,
};
use itertools::Itertools;
use new_path_prompt::NewPathPrompt;
use picker::{Picker, PickerDelegate};
use project::{PathMatchCandidateSet, Project, ProjectPath, WorktreeId};
use settings::Settings;
use std::{
    cmp,
    path::{Path, PathBuf},
    sync::{
        atomic::{self, AtomicBool},
        Arc,
    },
};
use text::Point;
use ui::{prelude::*, HighlightedLabel, ListItem, ListItemSpacing};
use util::{paths::PathLikeWithPosition, post_inc, ResultExt};
use workspace::{item::PreviewTabsSettings, ModalView, Workspace};

actions!(file_finder, [SelectPrev]);
impl_actions!(file_finder, [Toggle]);

#[derive(Default, PartialEq, Eq, Clone, serde::Deserialize)]
pub struct Toggle {
    #[serde(default)]
    pub separate_history: bool,
}

impl ModalView for FileFinder {}

pub struct FileFinder {
    picker: View<Picker<FileFinderDelegate>>,
    init_modifiers: Option<Modifiers>,
}

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(FileFinder::register).detach();
    cx.observe_new_views(NewPathPrompt::register).detach();
}

impl FileFinder {
    fn register(workspace: &mut Workspace, _: &mut ViewContext<Workspace>) {
        workspace.register_action(|workspace, action: &Toggle, cx| {
            let Some(file_finder) = workspace.active_modal::<Self>(cx) else {
                Self::open(workspace, action.separate_history, cx);
                return;
            };

            file_finder.update(cx, |file_finder, cx| {
                file_finder.init_modifiers = Some(cx.modifiers());
                file_finder.picker.update(cx, |picker, cx| {
                    picker.cycle_selection(cx);
                });
            });
        });
    }

    fn open(workspace: &mut Workspace, separate_history: bool, cx: &mut ViewContext<Workspace>) {
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

        let history_items = workspace
            .recent_navigation_history(Some(MAX_RECENT_SELECTIONS), cx)
            .into_iter()
            .filter(|(_, history_abs_path)| match history_abs_path {
                Some(abs_path) => history_file_exists(abs_path),
                None => true,
            })
            .map(|(history_path, abs_path)| FoundPath::new(history_path, abs_path))
            .collect::<Vec<_>>();

        let project = workspace.project().clone();
        let weak_workspace = cx.view().downgrade();
        workspace.toggle_modal(cx, |cx| {
            let delegate = FileFinderDelegate::new(
                cx.view().downgrade(),
                weak_workspace,
                project,
                currently_opened_path,
                history_items,
                separate_history,
                cx,
            );

            FileFinder::new(delegate, cx)
        });
    }

    fn new(delegate: FileFinderDelegate, cx: &mut ViewContext<Self>) -> Self {
        Self {
            picker: cx.new_view(|cx| Picker::uniform_list(delegate, cx)),
            init_modifiers: cx.modifiers().modified().then_some(cx.modifiers()),
        }
    }

    fn handle_modifiers_changed(
        &mut self,
        event: &ModifiersChangedEvent,
        cx: &mut ViewContext<Self>,
    ) {
        let Some(init_modifiers) = self.init_modifiers.take() else {
            return;
        };
        if self.picker.read(cx).delegate.has_changed_selected_index {
            if !event.modified() || !init_modifiers.is_subset_of(&event) {
                self.init_modifiers = None;
                cx.dispatch_action(menu::Confirm.boxed_clone());
            }
        }
    }

    fn handle_select_prev(&mut self, _: &SelectPrev, cx: &mut ViewContext<Self>) {
        self.init_modifiers = Some(cx.modifiers());
        cx.dispatch_action(Box::new(menu::SelectPrev));
    }
}

impl EventEmitter<DismissEvent> for FileFinder {}

impl FocusableView for FileFinder {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for FileFinder {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .key_context("FileFinder")
            .w(rems(34.))
            .on_modifiers_changed(cx.listener(Self::handle_modifiers_changed))
            .on_action(cx.listener(Self::handle_select_prev))
            .child(self.picker.clone())
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
    selected_index: usize,
    has_changed_selected_index: bool,
    cancel_flag: Arc<AtomicBool>,
    history_items: Vec<FoundPath>,
    separate_history: bool,
}

/// Use a custom ordering for file finder: the regular one
/// defines max element with the highest score and the latest alphanumerical path (in case of a tie on other params), e.g:
/// `[{score: 0.5, path = "c/d" }, { score: 0.5, path = "/a/b" }]`
///
/// In the file finder, we would prefer to have the max element with the highest score and the earliest alphanumerical path, e.g:
/// `[{ score: 0.5, path = "/a/b" }, {score: 0.5, path = "c/d" }]`
/// as the files are shown in the project panel lists.
#[derive(Debug, Clone, PartialEq, Eq)]
struct ProjectPanelOrdMatch(PathMatch);

impl Ord for ProjectPanelOrdMatch {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.0
            .score
            .partial_cmp(&other.0.score)
            .unwrap_or(cmp::Ordering::Equal)
            .then_with(|| self.0.worktree_id.cmp(&other.0.worktree_id))
            .then_with(|| {
                other
                    .0
                    .distance_to_relative_ancestor
                    .cmp(&self.0.distance_to_relative_ancestor)
            })
            .then_with(|| self.0.path.cmp(&other.0.path).reverse())
    }
}

impl PartialOrd for ProjectPanelOrdMatch {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Default)]
struct Matches {
    separate_history: bool,
    matches: Vec<Match>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum Match {
    History(FoundPath, Option<ProjectPanelOrdMatch>),
    Search(ProjectPanelOrdMatch),
}

impl Matches {
    fn len(&self) -> usize {
        self.matches.len()
    }

    fn get(&self, index: usize) -> Option<&Match> {
        self.matches.get(index)
    }

    fn push_new_matches<'a>(
        &'a mut self,
        history_items: impl IntoIterator<Item = &'a FoundPath> + Clone,
        currently_opened: Option<&'a FoundPath>,
        query: Option<&PathLikeWithPosition<FileSearchQuery>>,
        new_search_matches: impl Iterator<Item = ProjectPanelOrdMatch>,
        extend_old_matches: bool,
    ) {
        let no_history_score = 0;
        let matching_history_paths =
            matching_history_item_paths(history_items.clone(), currently_opened, query);
        let new_search_matches = new_search_matches
            .filter(|path_match| !matching_history_paths.contains_key(&path_match.0.path))
            .map(Match::Search)
            .map(|m| (no_history_score, m));
        let old_search_matches = self
            .matches
            .drain(..)
            .filter(|_| extend_old_matches)
            .filter(|m| matches!(m, Match::Search(_)))
            .map(|m| (no_history_score, m));
        let history_matches = history_items
            .into_iter()
            .chain(currently_opened)
            .enumerate()
            .filter_map(|(i, history_item)| {
                let query_match = matching_history_paths
                    .get(&history_item.project.path)
                    .cloned();
                let query_match = if query.is_some() {
                    query_match?
                } else {
                    query_match.flatten()
                };
                Some((i + 1, Match::History(history_item.clone(), query_match)))
            });

        let mut unique_matches = BTreeSet::new();
        self.matches = old_search_matches
            .chain(history_matches)
            .chain(new_search_matches)
            .filter(|(_, m)| unique_matches.insert(m.clone()))
            .sorted_by(|(history_score_a, a), (history_score_b, b)| {
                match (a, b) {
                    // bubble currently opened files to the top
                    (Match::History(path, _), _) if Some(path) == currently_opened => {
                        cmp::Ordering::Less
                    }
                    (_, Match::History(path, _)) if Some(path) == currently_opened => {
                        cmp::Ordering::Greater
                    }

                    (Match::History(_, _), Match::Search(_)) if self.separate_history => {
                        cmp::Ordering::Less
                    }
                    (Match::Search(_), Match::History(_, _)) if self.separate_history => {
                        cmp::Ordering::Greater
                    }

                    (Match::History(_, match_a), Match::History(_, match_b)) => {
                        match_b.cmp(match_a)
                    }
                    (Match::History(_, match_a), Match::Search(match_b)) => {
                        Some(match_b).cmp(&match_a.as_ref())
                    }
                    (Match::Search(match_a), Match::History(_, match_b)) => {
                        match_b.as_ref().cmp(&Some(match_a))
                    }
                    (Match::Search(match_a), Match::Search(match_b)) => match_b.cmp(match_a),
                }
                .then(history_score_a.cmp(history_score_b))
            })
            .take(100)
            .map(|(_, m)| m)
            .collect();
    }
}

fn matching_history_item_paths<'a>(
    history_items: impl IntoIterator<Item = &'a FoundPath>,
    currently_opened: Option<&'a FoundPath>,
    query: Option<&PathLikeWithPosition<FileSearchQuery>>,
) -> HashMap<Arc<Path>, Option<ProjectPanelOrdMatch>> {
    let Some(query) = query else {
        return history_items
            .into_iter()
            .chain(currently_opened)
            .map(|found_path| (Arc::clone(&found_path.project.path), None))
            .collect();
    };

    let history_items_by_worktrees = history_items
        .into_iter()
        .chain(currently_opened)
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
            .map(|path_match| {
                (
                    Arc::clone(&path_match.path),
                    Some(ProjectPanelOrdMatch(path_match)),
                )
            }),
        );
    }
    matching_history_paths
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
        separate_history: bool,
        cx: &mut ViewContext<FileFinder>,
    ) -> Self {
        Self::subscribe_to_updates(&project, cx);
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
            has_changed_selected_index: false,
            selected_index: 0,
            cancel_flag: Arc::new(AtomicBool::new(false)),
            history_items,
            separate_history,
        }
    }

    fn subscribe_to_updates(project: &Model<Project>, cx: &mut ViewContext<FileFinder>) {
        cx.subscribe(project, |file_finder, _, event, cx| {
            match event {
                project::Event::WorktreeUpdatedEntries(_, _)
                | project::Event::WorktreeAdded
                | project::Event::WorktreeRemoved(_) => file_finder
                    .picker
                    .update(cx, |picker, cx| picker.refresh(cx)),
                _ => {}
            };
        })
        .detach();
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
                    directories_only: false,
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
            .await
            .into_iter()
            .map(ProjectPanelOrdMatch);
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
        matches: impl IntoIterator<Item = ProjectPanelOrdMatch>,
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
            self.matches.push_new_matches(
                &self.history_items,
                self.currently_opened_path.as_ref(),
                Some(&query),
                matches.into_iter(),
                extend_old_matches,
            );
            self.latest_search_query = Some(query);
            self.latest_search_did_cancel = did_cancel;
            self.selected_index = self.calculate_selected_index();
            cx.notify();
        }
    }

    fn labels_for_match(
        &self,
        path_match: &Match,
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
                        .extend(found_path_match.0.positions.iter())
                }

                self.labels_for_path_match(&path_match)
            }
            Match::Search(path_match) => self.labels_for_path_match(&path_match.0),
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
        let mut path_positions = path_match.positions.clone();

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

        let full_path = full_path.trim_end_matches(&file_name).to_string();
        path_positions.retain(|idx| *idx < full_path.len());

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
                                path_matches.push(ProjectPanelOrdMatch(PathMatch {
                                    score: 1.0,
                                    positions: Vec::new(),
                                    worktree_id: worktree.read(cx).id().to_usize(),
                                    path: Arc::from(relative_path),
                                    path_prefix: "".into(),
                                    distance_to_relative_ancestor: usize::MAX,
                                }));
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

    /// Skips first history match (that is displayed topmost) if it's currently opened.
    fn calculate_selected_index(&self) -> usize {
        if let Some(Match::History(path, _)) = self.matches.get(0) {
            if Some(path) == self.currently_opened_path.as_ref() {
                let elements_after_first = self.matches.len() - 1;
                if elements_after_first > 0 {
                    return 1;
                }
            }
        }
        0
    }
}

impl PickerDelegate for FileFinderDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self, _cx: &mut WindowContext) -> Arc<str> {
        "Search project files...".into()
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, cx: &mut ViewContext<Picker<Self>>) {
        self.has_changed_selected_index = true;
        self.selected_index = ix;
        cx.notify();
    }

    fn separators_after_indices(&self) -> Vec<usize> {
        if self.separate_history {
            let first_non_history_index = self
                .matches
                .matches
                .iter()
                .enumerate()
                .find(|(_, m)| !matches!(m, Match::History(_, _)))
                .map(|(i, _)| i);
            if let Some(first_non_history_index) = first_non_history_index {
                if first_non_history_index > 0 {
                    return vec![first_non_history_index - 1];
                }
            }
        }
        Vec::new()
    }

    fn update_matches(
        &mut self,
        raw_query: String,
        cx: &mut ViewContext<Picker<Self>>,
    ) -> Task<()> {
        let raw_query = raw_query.replace(' ', "");
        let raw_query = raw_query.trim();
        if raw_query.is_empty() {
            let project = self.project.read(cx);
            self.latest_search_id = post_inc(&mut self.search_count);
            self.matches = Matches {
                separate_history: self.separate_history,
                ..Matches::default()
            };
            self.matches.push_new_matches(
                self.history_items.iter().filter(|history_item| {
                    project
                        .worktree_for_id(history_item.project.worktree_id, cx)
                        .is_some()
                        || (project.is_local() && history_item.absolute.is_some())
                }),
                self.currently_opened_path.as_ref(),
                None,
                None.into_iter(),
                false,
            );

            self.selected_index = 0;
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
                    let split_or_open =
                        |workspace: &mut Workspace,
                         project_path,
                         cx: &mut ViewContext<Workspace>| {
                            let allow_preview =
                                PreviewTabsSettings::get_global(cx).enable_preview_from_file_finder;
                            if secondary {
                                workspace.split_path_preview(project_path, allow_preview, cx)
                            } else {
                                workspace.open_path_preview(
                                    project_path,
                                    None,
                                    true,
                                    allow_preview,
                                    cx,
                                )
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
                                worktree_id: WorktreeId::from_usize(m.0.worktree_id),
                                path: m.0.path.clone(),
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

        let icon = match &path_match {
            Match::History(_, _) => Icon::new(IconName::HistoryRerun)
                .color(Color::Muted)
                .size(IconSize::Small)
                .into_any_element(),
            Match::Search(_) => v_flex()
                .flex_none()
                .size(IconSize::Small.rems())
                .into_any_element(),
        };
        let (file_name, file_name_positions, full_path, full_path_positions) =
            self.labels_for_match(path_match, cx, ix);

        Some(
            ListItem::new(ix)
                .spacing(ListItemSpacing::Sparse)
                .end_slot::<AnyElement>(Some(icon))
                .inset(true)
                .selected(selected)
                .child(
                    h_flex()
                        .gap_2()
                        .py_px()
                        .child(HighlightedLabel::new(file_name, file_name_positions))
                        .child(
                            HighlightedLabel::new(full_path, full_path_positions)
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        ),
                ),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_custom_project_search_ordering_in_file_finder() {
        let mut file_finder_sorted_output = vec![
            ProjectPanelOrdMatch(PathMatch {
                score: 0.5,
                positions: Vec::new(),
                worktree_id: 0,
                path: Arc::from(Path::new("b0.5")),
                path_prefix: Arc::from(""),
                distance_to_relative_ancestor: 0,
            }),
            ProjectPanelOrdMatch(PathMatch {
                score: 1.0,
                positions: Vec::new(),
                worktree_id: 0,
                path: Arc::from(Path::new("c1.0")),
                path_prefix: Arc::from(""),
                distance_to_relative_ancestor: 0,
            }),
            ProjectPanelOrdMatch(PathMatch {
                score: 1.0,
                positions: Vec::new(),
                worktree_id: 0,
                path: Arc::from(Path::new("a1.0")),
                path_prefix: Arc::from(""),
                distance_to_relative_ancestor: 0,
            }),
            ProjectPanelOrdMatch(PathMatch {
                score: 0.5,
                positions: Vec::new(),
                worktree_id: 0,
                path: Arc::from(Path::new("a0.5")),
                path_prefix: Arc::from(""),
                distance_to_relative_ancestor: 0,
            }),
            ProjectPanelOrdMatch(PathMatch {
                score: 1.0,
                positions: Vec::new(),
                worktree_id: 0,
                path: Arc::from(Path::new("b1.0")),
                path_prefix: Arc::from(""),
                distance_to_relative_ancestor: 0,
            }),
        ];
        file_finder_sorted_output.sort_by(|a, b| b.cmp(a));

        assert_eq!(
            file_finder_sorted_output,
            vec![
                ProjectPanelOrdMatch(PathMatch {
                    score: 1.0,
                    positions: Vec::new(),
                    worktree_id: 0,
                    path: Arc::from(Path::new("a1.0")),
                    path_prefix: Arc::from(""),
                    distance_to_relative_ancestor: 0,
                }),
                ProjectPanelOrdMatch(PathMatch {
                    score: 1.0,
                    positions: Vec::new(),
                    worktree_id: 0,
                    path: Arc::from(Path::new("b1.0")),
                    path_prefix: Arc::from(""),
                    distance_to_relative_ancestor: 0,
                }),
                ProjectPanelOrdMatch(PathMatch {
                    score: 1.0,
                    positions: Vec::new(),
                    worktree_id: 0,
                    path: Arc::from(Path::new("c1.0")),
                    path_prefix: Arc::from(""),
                    distance_to_relative_ancestor: 0,
                }),
                ProjectPanelOrdMatch(PathMatch {
                    score: 0.5,
                    positions: Vec::new(),
                    worktree_id: 0,
                    path: Arc::from(Path::new("a0.5")),
                    path_prefix: Arc::from(""),
                    distance_to_relative_ancestor: 0,
                }),
                ProjectPanelOrdMatch(PathMatch {
                    score: 0.5,
                    positions: Vec::new(),
                    worktree_id: 0,
                    path: Arc::from(Path::new("b0.5")),
                    path_prefix: Arc::from(""),
                    distance_to_relative_ancestor: 0,
                }),
            ]
        );
    }
}
