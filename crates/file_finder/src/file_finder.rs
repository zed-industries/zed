#[cfg(test)]
mod file_finder_tests;
#[cfg(test)]
mod open_path_prompt_tests;

pub mod file_finder_settings;
mod new_path_prompt;
mod open_path_prompt;

use futures::future::join_all;
pub use open_path_prompt::OpenPathDelegate;

use collections::HashMap;
use editor::Editor;
use file_finder_settings::{FileFinderAutoSelect, FileFinderSettings, FileFinderWidth};
use file_icons::FileIcons;
use fuzzy::{CharBag, PathMatch, PathMatchCandidate};
use gpui::{
    actions, Action, AnyElement, App, Context, DismissEvent, Entity, EventEmitter, FocusHandle,
    Focusable, KeyContext, Modifiers, ModifiersChangedEvent, ParentElement, Render, Styled, Task,
    WeakEntity, Window,
};
use new_path_prompt::NewPathPrompt;
use open_path_prompt::OpenPathPrompt;
use picker::{Picker, PickerDelegate};
use project::{PathMatchCandidateSet, Project, ProjectPath, WorktreeId};
use settings::Settings;
use std::{
    borrow::Cow,
    cmp,
    ops::Range,
    path::{Component, Path, PathBuf},
    sync::{
        atomic::{self, AtomicBool},
        Arc,
    },
};
use text::Point;
use ui::{
    prelude::*, ContextMenu, HighlightedLabel, ListItem, ListItemSpacing, PopoverMenu,
    PopoverMenuHandle,
};
use util::{maybe, paths::PathWithPosition, post_inc, ResultExt};
use workspace::{
    item::PreviewTabsSettings, notifications::NotifyResultExt, pane, ModalView, OpenOptions,
    OpenVisible, SplitDirection, Workspace,
};

actions!(file_finder, [SelectPrevious, ToggleMenu]);

impl ModalView for FileFinder {
    fn on_before_dismiss(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> workspace::DismissDecision {
        let submenu_focused = self.picker.update(cx, |picker, cx| {
            picker.delegate.popover_menu_handle.is_focused(window, cx)
        });
        workspace::DismissDecision::Dismiss(!submenu_focused)
    }
}

pub struct FileFinder {
    picker: Entity<Picker<FileFinderDelegate>>,
    picker_focus_handle: FocusHandle,
    init_modifiers: Option<Modifiers>,
}

pub fn init_settings(cx: &mut App) {
    FileFinderSettings::register(cx);
}

pub fn init(cx: &mut App) {
    init_settings(cx);
    cx.observe_new(FileFinder::register).detach();
    cx.observe_new(NewPathPrompt::register).detach();
    cx.observe_new(OpenPathPrompt::register).detach();
}

impl FileFinder {
    fn register(
        workspace: &mut Workspace,
        _window: Option<&mut Window>,
        _: &mut Context<Workspace>,
    ) {
        workspace.register_action(
            |workspace, action: &workspace::ToggleFileFinder, window, cx| {
                let Some(file_finder) = workspace.active_modal::<Self>(cx) else {
                    Self::open(workspace, action.separate_history, window, cx).detach();
                    return;
                };

                file_finder.update(cx, |file_finder, cx| {
                    file_finder.init_modifiers = Some(window.modifiers());
                    file_finder.picker.update(cx, |picker, cx| {
                        picker.cycle_selection(window, cx);
                    });
                });
            },
        );
    }

    fn open(
        workspace: &mut Workspace,
        separate_history: bool,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Task<()> {
        let project = workspace.project().read(cx);
        let fs = project.fs();

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
            .filter_map(|(project_path, abs_path)| {
                if project.entry_for_path(&project_path, cx).is_some() {
                    return Some(Task::ready(Some(FoundPath::new(project_path, abs_path))));
                }
                let abs_path = abs_path?;
                if project.is_local() {
                    let fs = fs.clone();
                    Some(cx.background_spawn(async move {
                        if fs.is_file(&abs_path).await {
                            Some(FoundPath::new(project_path, Some(abs_path)))
                        } else {
                            None
                        }
                    }))
                } else {
                    Some(Task::ready(Some(FoundPath::new(
                        project_path,
                        Some(abs_path),
                    ))))
                }
            })
            .collect::<Vec<_>>();
        cx.spawn_in(window, async move |workspace, cx| {
            let history_items = join_all(history_items).await.into_iter().flatten();

            workspace
                .update_in(cx, |workspace, window, cx| {
                    let project = workspace.project().clone();
                    let weak_workspace = cx.entity().downgrade();
                    workspace.toggle_modal(window, cx, |window, cx| {
                        let delegate = FileFinderDelegate::new(
                            cx.entity().downgrade(),
                            weak_workspace,
                            project,
                            currently_opened_path,
                            history_items.collect(),
                            separate_history,
                            window,
                            cx,
                        );

                        FileFinder::new(delegate, window, cx)
                    });
                })
                .ok();
        })
    }

    fn new(delegate: FileFinderDelegate, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx));
        let picker_focus_handle = picker.focus_handle(cx);
        picker.update(cx, |picker, _| {
            picker.delegate.focus_handle = picker_focus_handle.clone();
        });
        Self {
            picker,
            picker_focus_handle,
            init_modifiers: window.modifiers().modified().then_some(window.modifiers()),
        }
    }

    fn handle_modifiers_changed(
        &mut self,
        event: &ModifiersChangedEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(init_modifiers) = self.init_modifiers.take() else {
            return;
        };
        if self.picker.read(cx).delegate.has_changed_selected_index {
            if !event.modified() || !init_modifiers.is_subset_of(&event) {
                self.init_modifiers = None;
                window.dispatch_action(menu::Confirm.boxed_clone(), cx);
            }
        }
    }

    fn handle_select_prev(
        &mut self,
        _: &SelectPrevious,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.init_modifiers = Some(window.modifiers());
        window.dispatch_action(Box::new(menu::SelectPrevious), cx);
    }

    fn handle_toggle_menu(&mut self, _: &ToggleMenu, window: &mut Window, cx: &mut Context<Self>) {
        self.picker.update(cx, |picker, cx| {
            let menu_handle = &picker.delegate.popover_menu_handle;
            if menu_handle.is_deployed() {
                menu_handle.hide(cx);
            } else {
                menu_handle.show(window, cx);
            }
        });
    }

    fn go_to_file_split_left(
        &mut self,
        _: &pane::SplitLeft,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.go_to_file_split_inner(SplitDirection::Left, window, cx)
    }

    fn go_to_file_split_right(
        &mut self,
        _: &pane::SplitRight,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.go_to_file_split_inner(SplitDirection::Right, window, cx)
    }

    fn go_to_file_split_up(
        &mut self,
        _: &pane::SplitUp,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.go_to_file_split_inner(SplitDirection::Up, window, cx)
    }

    fn go_to_file_split_down(
        &mut self,
        _: &pane::SplitDown,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.go_to_file_split_inner(SplitDirection::Down, window, cx)
    }

    fn go_to_file_split_inner(
        &mut self,
        split_direction: SplitDirection,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.picker.update(cx, |picker, cx| {
            let delegate = &mut picker.delegate;
            if let Some(workspace) = delegate.workspace.upgrade() {
                if let Some(m) = delegate.matches.get(delegate.selected_index()) {
                    let path = match &m {
                        Match::History { path, .. } => {
                            let worktree_id = path.project.worktree_id;
                            ProjectPath {
                                worktree_id,
                                path: Arc::clone(&path.project.path),
                            }
                        }
                        Match::Search(m) => ProjectPath {
                            worktree_id: WorktreeId::from_usize(m.0.worktree_id),
                            path: m.0.path.clone(),
                        },
                    };
                    let open_task = workspace.update(cx, move |workspace, cx| {
                        workspace.split_path_preview(path, false, Some(split_direction), window, cx)
                    });
                    open_task.detach_and_log_err(cx);
                }
            }
        })
    }

    pub fn modal_max_width(width_setting: Option<FileFinderWidth>, window: &mut Window) -> Pixels {
        let window_width = window.viewport_size().width;
        let small_width = Pixels(545.);

        match width_setting {
            None | Some(FileFinderWidth::Small) => small_width,
            Some(FileFinderWidth::Full) => window_width,
            Some(FileFinderWidth::XLarge) => (window_width - Pixels(512.)).max(small_width),
            Some(FileFinderWidth::Large) => (window_width - Pixels(768.)).max(small_width),
            Some(FileFinderWidth::Medium) => (window_width - Pixels(1024.)).max(small_width),
        }
    }
}

impl EventEmitter<DismissEvent> for FileFinder {}

impl Focusable for FileFinder {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.picker_focus_handle.clone()
    }
}

impl Render for FileFinder {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let key_context = self.picker.read(cx).delegate.key_context(window, cx);

        let file_finder_settings = FileFinderSettings::get_global(cx);
        let modal_max_width = Self::modal_max_width(file_finder_settings.modal_max_width, window);

        v_flex()
            .key_context(key_context)
            .w(modal_max_width)
            .on_modifiers_changed(cx.listener(Self::handle_modifiers_changed))
            .on_action(cx.listener(Self::handle_select_prev))
            .on_action(cx.listener(Self::handle_toggle_menu))
            .on_action(cx.listener(Self::go_to_file_split_left))
            .on_action(cx.listener(Self::go_to_file_split_right))
            .on_action(cx.listener(Self::go_to_file_split_up))
            .on_action(cx.listener(Self::go_to_file_split_down))
            .child(self.picker.clone())
    }
}

pub struct FileFinderDelegate {
    file_finder: WeakEntity<FileFinder>,
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    search_count: usize,
    latest_search_id: usize,
    latest_search_did_cancel: bool,
    latest_search_query: Option<FileSearchQuery>,
    currently_opened_path: Option<FoundPath>,
    matches: Matches,
    selected_index: usize,
    has_changed_selected_index: bool,
    cancel_flag: Arc<AtomicBool>,
    history_items: Vec<FoundPath>,
    separate_history: bool,
    first_update: bool,
    popover_menu_handle: PopoverMenuHandle<ContextMenu>,
    focus_handle: FocusHandle,
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
    History {
        path: FoundPath,
        panel_match: Option<ProjectPanelOrdMatch>,
    },
    Search(ProjectPanelOrdMatch),
}

impl Match {
    fn path(&self) -> &Arc<Path> {
        match self {
            Match::History { path, .. } => &path.project.path,
            Match::Search(panel_match) => &panel_match.0.path,
        }
    }

    fn panel_match(&self) -> Option<&ProjectPanelOrdMatch> {
        match self {
            Match::History { panel_match, .. } => panel_match.as_ref(),
            Match::Search(panel_match) => Some(&panel_match),
        }
    }
}

impl Matches {
    fn len(&self) -> usize {
        self.matches.len()
    }

    fn get(&self, index: usize) -> Option<&Match> {
        self.matches.get(index)
    }

    fn position(
        &self,
        entry: &Match,
        currently_opened: Option<&FoundPath>,
    ) -> Result<usize, usize> {
        if let Match::History {
            path,
            panel_match: None,
        } = entry
        {
            // Slow case: linear search by path. Should not happen actually,
            // since we call `position` only if matches set changed, but the query has not changed.
            // And History entries do not have panel_match if query is empty, so there's no
            // reason for the matches set to change.
            self.matches
                .iter()
                .position(|m| path.project.path == *m.path())
                .ok_or(0)
        } else {
            self.matches.binary_search_by(|m| {
                // `reverse()` since if cmp_matches(a, b) == Ordering::Greater, then a is better than b.
                // And we want the better entries go first.
                Self::cmp_matches(self.separate_history, currently_opened, &m, &entry).reverse()
            })
        }
    }

    fn push_new_matches<'a>(
        &'a mut self,
        history_items: impl IntoIterator<Item = &'a FoundPath> + Clone,
        currently_opened: Option<&'a FoundPath>,
        query: Option<&FileSearchQuery>,
        new_search_matches: impl Iterator<Item = ProjectPanelOrdMatch>,
        extend_old_matches: bool,
    ) {
        let Some(query) = query else {
            // assuming that if there's no query, then there's no search matches.
            self.matches.clear();
            let path_to_entry = |found_path: &FoundPath| Match::History {
                path: found_path.clone(),
                panel_match: None,
            };
            self.matches
                .extend(currently_opened.into_iter().map(path_to_entry));

            self.matches.extend(
                history_items
                    .into_iter()
                    .filter(|found_path| Some(*found_path) != currently_opened)
                    .map(path_to_entry),
            );
            return;
        };

        let new_history_matches = matching_history_items(history_items, currently_opened, query);
        let new_search_matches: Vec<Match> = new_search_matches
            .filter(|path_match| !new_history_matches.contains_key(&path_match.0.path))
            .map(Match::Search)
            .collect();

        if extend_old_matches {
            // since we take history matches instead of new search matches
            // and history matches has not changed(since the query has not changed and we do not extend old matches otherwise),
            // old matches can't contain paths present in history_matches as well.
            self.matches.retain(|m| matches!(m, Match::Search(_)));
        } else {
            self.matches.clear();
        }

        // At this point we have an unsorted set of new history matches, an unsorted set of new search matches
        // and a sorted set of old search matches.
        // It is possible that the new search matches' paths contain some of the old search matches' paths.
        // History matches' paths are unique, since store in a HashMap by path.
        // We build a sorted Vec<Match>, eliminating duplicate search matches.
        // Search matches with the same paths should have equal `ProjectPanelOrdMatch`, so we should
        // not have any duplicates after building the final list.
        for new_match in new_history_matches
            .into_values()
            .chain(new_search_matches.into_iter())
        {
            match self.position(&new_match, currently_opened) {
                Ok(_duplicate) => continue,
                Err(i) => {
                    self.matches.insert(i, new_match);
                    if self.matches.len() == 100 {
                        break;
                    }
                }
            }
        }
    }

    /// If a < b, then a is a worse match, aligning with the `ProjectPanelOrdMatch` ordering.
    fn cmp_matches(
        separate_history: bool,
        currently_opened: Option<&FoundPath>,
        a: &Match,
        b: &Match,
    ) -> cmp::Ordering {
        debug_assert!(a.panel_match().is_some() && b.panel_match().is_some());

        match (&a, &b) {
            // bubble currently opened files to the top
            (Match::History { path, .. }, _) if Some(path) == currently_opened => {
                cmp::Ordering::Greater
            }
            (_, Match::History { path, .. }) if Some(path) == currently_opened => {
                cmp::Ordering::Less
            }

            (Match::History { .. }, Match::Search(_)) if separate_history => cmp::Ordering::Greater,
            (Match::Search(_), Match::History { .. }) if separate_history => cmp::Ordering::Less,

            _ => a.panel_match().cmp(&b.panel_match()),
        }
    }
}

fn matching_history_items<'a>(
    history_items: impl IntoIterator<Item = &'a FoundPath>,
    currently_opened: Option<&'a FoundPath>,
    query: &FileSearchQuery,
) -> HashMap<Arc<Path>, Match> {
    let mut candidates_paths = HashMap::default();

    let history_items_by_worktrees = history_items
        .into_iter()
        .chain(currently_opened)
        .filter_map(|found_path| {
            let candidate = PathMatchCandidate {
                is_dir: false, // You can't open directories as project items
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
            candidates_paths.insert(&found_path.project, found_path);
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
                query.path_query(),
                false,
                max_results,
            )
            .into_iter()
            .filter_map(|path_match| {
                candidates_paths
                    .remove_entry(&ProjectPath {
                        worktree_id: WorktreeId::from_usize(path_match.worktree_id),
                        path: Arc::clone(&path_match.path),
                    })
                    .map(|(_, found_path)| {
                        (
                            Arc::clone(&path_match.path),
                            Match::History {
                                path: found_path.clone(),
                                panel_match: Some(ProjectPanelOrdMatch(path_match)),
                            },
                        )
                    })
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

pub enum Event {
    Selected(ProjectPath),
    Dismissed,
}

#[derive(Debug, Clone)]
struct FileSearchQuery {
    raw_query: String,
    file_query_end: Option<usize>,
    path_position: PathWithPosition,
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
        file_finder: WeakEntity<FileFinder>,
        workspace: WeakEntity<Workspace>,
        project: Entity<Project>,
        currently_opened_path: Option<FoundPath>,
        history_items: Vec<FoundPath>,
        separate_history: bool,
        window: &mut Window,
        cx: &mut Context<FileFinder>,
    ) -> Self {
        Self::subscribe_to_updates(&project, window, cx);
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
            first_update: true,
            popover_menu_handle: PopoverMenuHandle::default(),
            focus_handle: cx.focus_handle(),
        }
    }

    fn subscribe_to_updates(
        project: &Entity<Project>,
        window: &mut Window,
        cx: &mut Context<FileFinder>,
    ) {
        cx.subscribe_in(project, window, |file_finder, _, event, window, cx| {
            match event {
                project::Event::WorktreeUpdatedEntries(_, _)
                | project::Event::WorktreeAdded(_)
                | project::Event::WorktreeRemoved(_) => file_finder
                    .picker
                    .update(cx, |picker, cx| picker.refresh(window, cx)),
                _ => {}
            };
        })
        .detach();
    }

    fn spawn_search(
        &mut self,
        query: FileSearchQuery,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
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
                    candidates: project::Candidates::Files,
                }
            })
            .collect::<Vec<_>>();

        let search_id = util::post_inc(&mut self.search_count);
        self.cancel_flag.store(true, atomic::Ordering::Relaxed);
        self.cancel_flag = Arc::new(AtomicBool::new(false));
        let cancel_flag = self.cancel_flag.clone();
        cx.spawn_in(window, async move |picker, cx| {
            let matches = fuzzy::match_path_sets(
                candidate_sets.as_slice(),
                query.path_query(),
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
                .update(cx, |picker, cx| {
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
        query: FileSearchQuery,
        matches: impl IntoIterator<Item = ProjectPanelOrdMatch>,
        cx: &mut Context<Picker<Self>>,
    ) {
        if search_id >= self.latest_search_id {
            self.latest_search_id = search_id;
            let query_changed = Some(query.path_query())
                != self
                    .latest_search_query
                    .as_ref()
                    .map(|query| query.path_query());
            let extend_old_matches = self.latest_search_did_cancel && !query_changed;

            let selected_match = if query_changed {
                None
            } else {
                self.matches.get(self.selected_index).cloned()
            };

            self.matches.push_new_matches(
                &self.history_items,
                self.currently_opened_path.as_ref(),
                Some(&query),
                matches.into_iter(),
                extend_old_matches,
            );

            self.selected_index = selected_match.map_or_else(
                || self.calculate_selected_index(cx),
                |m| {
                    self.matches
                        .position(&m, self.currently_opened_path.as_ref())
                        .unwrap_or(0)
                },
            );

            self.latest_search_query = Some(query);
            self.latest_search_did_cancel = did_cancel;

            cx.notify();
        }
    }

    fn labels_for_match(
        &self,
        path_match: &Match,
        window: &mut Window,
        cx: &App,
        ix: usize,
    ) -> (HighlightedLabel, HighlightedLabel) {
        let (file_name, file_name_positions, mut full_path, mut full_path_positions) =
            match &path_match {
                Match::History {
                    path: entry_path,
                    panel_match,
                } => {
                    let worktree_id = entry_path.project.worktree_id;
                    let project_relative_path = &entry_path.project.path;
                    let has_worktree = self
                        .project
                        .read(cx)
                        .worktree_for_id(worktree_id, cx)
                        .is_some();

                    if let Some(absolute_path) =
                        entry_path.absolute.as_ref().filter(|_| !has_worktree)
                    {
                        (
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
                        )
                    } else {
                        let mut path = Arc::clone(project_relative_path);
                        if project_relative_path.as_ref() == Path::new("") {
                            if let Some(absolute_path) = &entry_path.absolute {
                                path = Arc::from(absolute_path.as_path());
                            }
                        }

                        let mut path_match = PathMatch {
                            score: ix as f64,
                            positions: Vec::new(),
                            worktree_id: worktree_id.to_usize(),
                            path,
                            is_dir: false, // File finder doesn't support directories
                            path_prefix: "".into(),
                            distance_to_relative_ancestor: usize::MAX,
                        };
                        if let Some(found_path_match) = &panel_match {
                            path_match
                                .positions
                                .extend(found_path_match.0.positions.iter())
                        }

                        self.labels_for_path_match(&path_match)
                    }
                }
                Match::Search(path_match) => self.labels_for_path_match(&path_match.0),
            };

        if file_name_positions.is_empty() {
            if let Some(user_home_path) = std::env::var("HOME").ok() {
                let user_home_path = user_home_path.trim();
                if !user_home_path.is_empty() {
                    if (&full_path).starts_with(user_home_path) {
                        full_path.replace_range(0..user_home_path.len(), "~");
                        full_path_positions.retain_mut(|pos| {
                            if *pos >= user_home_path.len() {
                                *pos -= user_home_path.len();
                                *pos += 1;
                                true
                            } else {
                                false
                            }
                        })
                    }
                }
            }
        }

        if full_path.is_ascii() {
            let file_finder_settings = FileFinderSettings::get_global(cx);
            let max_width =
                FileFinder::modal_max_width(file_finder_settings.modal_max_width, window);
            let (normal_em, small_em) = {
                let style = window.text_style();
                let font_id = window.text_system().resolve_font(&style.font());
                let font_size = TextSize::Default.rems(cx).to_pixels(window.rem_size());
                let normal = cx
                    .text_system()
                    .em_width(font_id, font_size)
                    .unwrap_or(px(16.));
                let font_size = TextSize::Small.rems(cx).to_pixels(window.rem_size());
                let small = cx
                    .text_system()
                    .em_width(font_id, font_size)
                    .unwrap_or(px(10.));
                (normal, small)
            };
            let budget = full_path_budget(&file_name, normal_em, small_em, max_width);
            // If the computed budget is zero, we certainly won't be able to achieve it,
            // so no point trying to elide the path.
            if budget > 0 && full_path.len() > budget {
                let components = PathComponentSlice::new(&full_path);
                if let Some(elided_range) =
                    components.elision_range(budget - 1, &full_path_positions)
                {
                    let elided_len = elided_range.end - elided_range.start;
                    let placeholder = "…";
                    full_path_positions.retain_mut(|mat| {
                        if *mat >= elided_range.end {
                            *mat -= elided_len;
                            *mat += placeholder.len();
                        } else if *mat >= elided_range.start {
                            return false;
                        }
                        true
                    });
                    full_path.replace_range(elided_range, placeholder);
                }
            }
        }

        (
            HighlightedLabel::new(file_name, file_name_positions),
            HighlightedLabel::new(full_path, full_path_positions)
                .size(LabelSize::Small)
                .color(Color::Muted),
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
        query: FileSearchQuery,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        cx.spawn_in(window, async move |picker, cx| {
            let Some(project) = picker
                .update(cx, |picker, _| picker.delegate.project.clone())
                .log_err()
            else {
                return;
            };

            let query_path = Path::new(query.path_query());
            let mut path_matches = Vec::new();

            let abs_file_exists = if let Ok(task) = project.update(cx, |this, cx| {
                this.resolve_abs_file_path(query.path_query(), cx)
            }) {
                task.await.is_some()
            } else {
                false
            };

            if abs_file_exists {
                let update_result = project
                    .update(cx, |project, cx| {
                        if let Some((worktree, relative_path)) =
                            project.find_worktree(query_path, cx)
                        {
                            path_matches.push(ProjectPanelOrdMatch(PathMatch {
                                score: 1.0,
                                positions: Vec::new(),
                                worktree_id: worktree.read(cx).id().to_usize(),
                                path: Arc::from(relative_path),
                                path_prefix: "".into(),
                                is_dir: false, // File finder doesn't support directories
                                distance_to_relative_ancestor: usize::MAX,
                            }));
                        }
                    })
                    .log_err();
                if update_result.is_none() {
                    return;
                }
            }

            picker
                .update_in(cx, |picker, _, cx| {
                    let picker_delegate = &mut picker.delegate;
                    let search_id = util::post_inc(&mut picker_delegate.search_count);
                    picker_delegate.set_search_matches(search_id, false, query, path_matches, cx);

                    anyhow::Ok(())
                })
                .log_err();
        })
    }

    /// Skips first history match (that is displayed topmost) if it's currently opened.
    fn calculate_selected_index(&self, cx: &mut Context<Picker<Self>>) -> usize {
        if FileFinderSettings::get_global(cx).auto_select == FileFinderAutoSelect::SkipActive {
            if let Some(Match::History { path, .. }) = self.matches.get(0) {
                if Some(path) == self.currently_opened_path.as_ref() {
                    let elements_after_first = self.matches.len() - 1;
                    if elements_after_first > 0 {
                        return 1;
                    }
                }
            }
        }

        0
    }

    fn key_context(&self, window: &Window, cx: &App) -> KeyContext {
        let mut key_context = KeyContext::new_with_defaults();
        key_context.add("FileFinder");
        if self.popover_menu_handle.is_focused(window, cx) {
            key_context.add("menu_open");
        }
        key_context
    }
}

fn full_path_budget(
    file_name: &str,
    normal_em: Pixels,
    small_em: Pixels,
    max_width: Pixels,
) -> usize {
    (((max_width / 0.8) - file_name.len() * normal_em) / small_em) as usize
}

impl PickerDelegate for FileFinderDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Search project files...".into()
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, _: &mut Window, cx: &mut Context<Picker<Self>>) {
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
                .find(|(_, m)| !matches!(m, Match::History { .. }))
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
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        let raw_query = raw_query.replace(' ', "");
        let raw_query = raw_query.trim();
        if raw_query.is_empty() {
            // if there was no query before, and we already have some (history) matches
            // there's no need to update anything, since nothing has changed.
            // We also want to populate matches set from history entries on the first update.
            if self.latest_search_query.is_some() || self.first_update {
                let project = self.project.read(cx);

                self.latest_search_id = post_inc(&mut self.search_count);
                self.latest_search_query = None;
                self.matches = Matches {
                    separate_history: self.separate_history,
                    ..Matches::default()
                };
                self.matches.push_new_matches(
                    self.history_items.iter().filter(|history_item| {
                        project
                            .worktree_for_id(history_item.project.worktree_id, cx)
                            .is_some()
                            || ((project.is_local() || project.is_via_ssh())
                                && history_item.absolute.is_some())
                    }),
                    self.currently_opened_path.as_ref(),
                    None,
                    None.into_iter(),
                    false,
                );

                self.first_update = false;
                self.selected_index = 0;
            }
            cx.notify();
            Task::ready(())
        } else {
            let path_position = PathWithPosition::parse_str(&raw_query);

            let query = FileSearchQuery {
                raw_query: raw_query.trim().to_owned(),
                file_query_end: if path_position.path.to_str().unwrap_or(raw_query) == raw_query {
                    None
                } else {
                    // Safe to unwrap as we won't get here when the unwrap in if fails
                    Some(path_position.path.to_str().unwrap().len())
                },
                path_position,
            };

            if Path::new(query.path_query()).is_absolute() {
                self.lookup_absolute_path(query, window, cx)
            } else {
                self.spawn_search(query, window, cx)
            }
        }
    }

    fn confirm(
        &mut self,
        secondary: bool,
        window: &mut Window,
        cx: &mut Context<Picker<FileFinderDelegate>>,
    ) {
        if let Some(m) = self.matches.get(self.selected_index()) {
            if let Some(workspace) = self.workspace.upgrade() {
                let open_task = workspace.update(cx, |workspace, cx| {
                    let split_or_open =
                        |workspace: &mut Workspace,
                         project_path,
                         window: &mut Window,
                         cx: &mut Context<Workspace>| {
                            let allow_preview =
                                PreviewTabsSettings::get_global(cx).enable_preview_from_file_finder;
                            if secondary {
                                workspace.split_path_preview(
                                    project_path,
                                    allow_preview,
                                    None,
                                    window,
                                    cx,
                                )
                            } else {
                                workspace.open_path_preview(
                                    project_path,
                                    None,
                                    true,
                                    allow_preview,
                                    true,
                                    window,
                                    cx,
                                )
                            }
                        };
                    match &m {
                        Match::History { path, .. } => {
                            let worktree_id = path.project.worktree_id;
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
                                        path: Arc::clone(&path.project.path),
                                    },
                                    window,
                                    cx,
                                )
                            } else {
                                match path.absolute.as_ref() {
                                    Some(abs_path) => {
                                        if secondary {
                                            workspace.split_abs_path(
                                                abs_path.to_path_buf(),
                                                false,
                                                window,
                                                cx,
                                            )
                                        } else {
                                            workspace.open_abs_path(
                                                abs_path.to_path_buf(),
                                                OpenOptions {
                                                    visible: Some(OpenVisible::None),
                                                    ..Default::default()
                                                },
                                                window,
                                                cx,
                                            )
                                        }
                                    }
                                    None => split_or_open(
                                        workspace,
                                        ProjectPath {
                                            worktree_id,
                                            path: Arc::clone(&path.project.path),
                                        },
                                        window,
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
                            window,
                            cx,
                        ),
                    }
                });

                let row = self
                    .latest_search_query
                    .as_ref()
                    .and_then(|query| query.path_position.row)
                    .map(|row| row.saturating_sub(1));
                let col = self
                    .latest_search_query
                    .as_ref()
                    .and_then(|query| query.path_position.column)
                    .unwrap_or(0)
                    .saturating_sub(1);
                let finder = self.file_finder.clone();

                cx.spawn_in(window, async move |_, cx| {
                    let item = open_task.await.notify_async_err(cx)?;
                    if let Some(row) = row {
                        if let Some(active_editor) = item.downcast::<Editor>() {
                            active_editor
                                .downgrade()
                                .update_in(cx, |editor, window, cx| {
                                    editor.go_to_singleton_buffer_point(
                                        Point::new(row, col),
                                        window,
                                        cx,
                                    );
                                })
                                .log_err();
                        }
                    }
                    finder.update(cx, |_, cx| cx.emit(DismissEvent)).ok()?;

                    Some(())
                })
                .detach();
            }
        }
    }

    fn dismissed(&mut self, _: &mut Window, cx: &mut Context<Picker<FileFinderDelegate>>) {
        self.file_finder
            .update(cx, |_, cx| cx.emit(DismissEvent))
            .log_err();
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let settings = FileFinderSettings::get_global(cx);

        let path_match = self
            .matches
            .get(ix)
            .expect("Invalid matches state: no element for index {ix}");

        let history_icon = match &path_match {
            Match::History { .. } => Icon::new(IconName::HistoryRerun)
                .color(Color::Muted)
                .size(IconSize::Small)
                .into_any_element(),
            Match::Search(_) => v_flex()
                .flex_none()
                .size(IconSize::Small.rems())
                .into_any_element(),
        };
        let (file_name_label, full_path_label) = self.labels_for_match(path_match, window, cx, ix);

        let file_icon = maybe!({
            if !settings.file_icons {
                return None;
            }
            let file_name = path_match.path().file_name()?;
            let icon = FileIcons::get_icon(file_name.as_ref(), cx)?;
            Some(Icon::from_path(icon).color(Color::Muted))
        });

        Some(
            ListItem::new(ix)
                .spacing(ListItemSpacing::Sparse)
                .start_slot::<Icon>(file_icon)
                .end_slot::<AnyElement>(history_icon)
                .inset(true)
                .toggle_state(selected)
                .child(
                    h_flex()
                        .gap_2()
                        .py_px()
                        .child(file_name_label)
                        .child(full_path_label),
                ),
        )
    }

    fn render_footer(&self, _: &mut Window, cx: &mut Context<Picker<Self>>) -> Option<AnyElement> {
        let context = self.focus_handle.clone();
        Some(
            h_flex()
                .w_full()
                .p_2()
                .gap_2()
                .justify_end()
                .border_t_1()
                .border_color(cx.theme().colors().border_variant)
                .child(
                    Button::new("open-selection", "Open").on_click(|_, window, cx| {
                        window.dispatch_action(menu::Confirm.boxed_clone(), cx)
                    }),
                )
                .child(
                    PopoverMenu::new("menu-popover")
                        .with_handle(self.popover_menu_handle.clone())
                        .attach(gpui::Corner::TopRight)
                        .anchor(gpui::Corner::BottomRight)
                        .trigger(
                            Button::new("actions-trigger", "Split…")
                                .selected_label_color(Color::Accent),
                        )
                        .menu({
                            move |window, cx| {
                                Some(ContextMenu::build(window, cx, {
                                    let context = context.clone();
                                    move |menu, _, _| {
                                        menu.context(context)
                                            .action("Split Left", pane::SplitLeft.boxed_clone())
                                            .action("Split Right", pane::SplitRight.boxed_clone())
                                            .action("Split Up", pane::SplitUp.boxed_clone())
                                            .action("Split Down", pane::SplitDown.boxed_clone())
                                    }
                                }))
                            }
                        }),
                )
                .into_any(),
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct PathComponentSlice<'a> {
    path: Cow<'a, Path>,
    path_str: Cow<'a, str>,
    component_ranges: Vec<(Component<'a>, Range<usize>)>,
}

impl<'a> PathComponentSlice<'a> {
    fn new(path: &'a str) -> Self {
        let trimmed_path = Path::new(path).components().as_path().as_os_str();
        let mut component_ranges = Vec::new();
        let mut components = Path::new(trimmed_path).components();
        let len = trimmed_path.as_encoded_bytes().len();
        let mut pos = 0;
        while let Some(component) = components.next() {
            component_ranges.push((component, pos..0));
            pos = len - components.as_path().as_os_str().as_encoded_bytes().len();
        }
        for ((_, range), ancestor) in component_ranges
            .iter_mut()
            .rev()
            .zip(Path::new(trimmed_path).ancestors())
        {
            range.end = ancestor.as_os_str().as_encoded_bytes().len();
        }
        Self {
            path: Cow::Borrowed(Path::new(path)),
            path_str: Cow::Borrowed(path),
            component_ranges,
        }
    }

    fn elision_range(&self, budget: usize, matches: &[usize]) -> Option<Range<usize>> {
        let eligible_range = {
            assert!(matches.windows(2).all(|w| w[0] <= w[1]));
            let mut matches = matches.iter().copied().peekable();
            let mut longest: Option<Range<usize>> = None;
            let mut cur = 0..0;
            let mut seen_normal = false;
            for (i, (component, range)) in self.component_ranges.iter().enumerate() {
                let is_normal = matches!(component, Component::Normal(_));
                let is_first_normal = is_normal && !seen_normal;
                seen_normal |= is_normal;
                let is_last = i == self.component_ranges.len() - 1;
                let contains_match = matches.peek().is_some_and(|mat| range.contains(mat));
                if contains_match {
                    matches.next();
                }
                if is_first_normal || is_last || !is_normal || contains_match {
                    if longest
                        .as_ref()
                        .is_none_or(|old| old.end - old.start <= cur.end - cur.start)
                    {
                        longest = Some(cur);
                    }
                    cur = i + 1..i + 1;
                } else {
                    cur.end = i + 1;
                }
            }
            if longest
                .as_ref()
                .is_none_or(|old| old.end - old.start <= cur.end - cur.start)
            {
                longest = Some(cur);
            }
            longest
        };

        let eligible_range = eligible_range?;
        assert!(eligible_range.start <= eligible_range.end);
        if eligible_range.is_empty() {
            return None;
        }

        let elided_range: Range<usize> = {
            let byte_range = self.component_ranges[eligible_range.start].1.start
                ..self.component_ranges[eligible_range.end - 1].1.end;
            let midpoint = self.path_str.len() / 2;
            let distance_from_start = byte_range.start.abs_diff(midpoint);
            let distance_from_end = byte_range.end.abs_diff(midpoint);
            let pick_from_end = distance_from_start > distance_from_end;
            let mut len_with_elision = self.path_str.len();
            let mut i = eligible_range.start;
            while i < eligible_range.end {
                let x = if pick_from_end {
                    eligible_range.end - i + eligible_range.start - 1
                } else {
                    i
                };
                len_with_elision -= self.component_ranges[x]
                    .0
                    .as_os_str()
                    .as_encoded_bytes()
                    .len()
                    + 1;
                if len_with_elision <= budget {
                    break;
                }
                i += 1;
            }
            if len_with_elision > budget {
                return None;
            } else if pick_from_end {
                let x = eligible_range.end - i + eligible_range.start - 1;
                x..eligible_range.end
            } else {
                let x = i;
                eligible_range.start..x + 1
            }
        };

        let byte_range = self.component_ranges[elided_range.start].1.start
            ..self.component_ranges[elided_range.end - 1].1.end;
        Some(byte_range)
    }
}
