#[cfg(test)]
mod file_finder_tests;
#[cfg(test)]
mod open_path_prompt_tests;

pub mod file_finder_settings;
mod open_path_prompt;

use futures::future::join_all;
pub use open_path_prompt::OpenPathDelegate;

use collections::HashMap;
use editor::Editor;
use file_finder_settings::{FileFinderSettings, FileFinderWidth};
use file_icons::FileIcons;
use fuzzy::{CharBag, PathMatch, PathMatchCandidate};
use gpui::{
    Action, AnyElement, App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable,
    KeyContext, Modifiers, ModifiersChangedEvent, ParentElement, Render, Styled, Task, WeakEntity,
    Window, actions, rems,
};
use open_path_prompt::OpenPathPrompt;
use picker::{Picker, PickerDelegate};
use project::{PathMatchCandidateSet, Project, ProjectPath, WorktreeId};
use search::ToggleIncludeIgnored;
use settings::Settings;
use std::{
    borrow::Cow,
    cmp,
    ops::Range,
    path::{Component, Path, PathBuf},
    sync::{
        Arc,
        atomic::{self, AtomicBool},
    },
};
use text::Point;
use ui::{
    ButtonLike, ContextMenu, HighlightedLabel, Indicator, KeyBinding, ListItem, ListItemSpacing,
    PopoverMenu, PopoverMenuHandle, TintColor, Tooltip, prelude::*,
};
use util::{
    ResultExt, maybe,
    paths::{PathStyle, PathWithPosition},
    post_inc,
    rel_path::RelPath,
};
use workspace::{
    ModalView, OpenOptions, OpenVisible, SplitDirection, Workspace, item::PreviewTabsSettings,
    notifications::NotifyResultExt, pane,
};

actions!(
    file_finder,
    [
        /// Selects the previous item in the file finder.
        SelectPrevious,
        /// Toggles the file filter menu.
        ToggleFilterMenu,
        /// Toggles the split direction menu.
        ToggleSplitMenu
    ]
);

impl ModalView for FileFinder {
    fn on_before_dismiss(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> workspace::DismissDecision {
        let submenu_focused = self.picker.update(cx, |picker, cx| {
            picker
                .delegate
                .filter_popover_menu_handle
                .is_focused(window, cx)
                || picker
                    .delegate
                    .split_popover_menu_handle
                    .is_focused(window, cx)
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
    cx.observe_new(OpenPathPrompt::register).detach();
    cx.observe_new(OpenPathPrompt::register_new_path).detach();
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

        let currently_opened_path = workspace.active_item(cx).and_then(|item| {
            let project_path = item.project_path(cx)?;
            let abs_path = project
                .worktree_for_id(project_path.worktree_id, cx)?
                .read(cx)
                .absolutize(&project_path.path);
            Some(FoundPath::new(project_path, abs_path))
        });

        let history_items = workspace
            .recent_navigation_history(Some(MAX_RECENT_SELECTIONS), cx)
            .into_iter()
            .filter_map(|(project_path, abs_path)| {
                if project.entry_for_path(&project_path, cx).is_some() {
                    return Some(Task::ready(Some(FoundPath::new(project_path, abs_path?))));
                }
                let abs_path = abs_path?;
                if project.is_local() {
                    let fs = fs.clone();
                    Some(cx.background_spawn(async move {
                        if fs.is_file(&abs_path).await {
                            Some(FoundPath::new(project_path, abs_path))
                        } else {
                            None
                        }
                    }))
                } else {
                    Some(Task::ready(Some(FoundPath::new(project_path, abs_path))))
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
        if self.picker.read(cx).delegate.has_changed_selected_index
            && (!event.modified() || !init_modifiers.is_subset_of(event))
        {
            self.init_modifiers = None;
            window.dispatch_action(menu::Confirm.boxed_clone(), cx);
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

    fn handle_filter_toggle_menu(
        &mut self,
        _: &ToggleFilterMenu,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.picker.update(cx, |picker, cx| {
            let menu_handle = &picker.delegate.filter_popover_menu_handle;
            if menu_handle.is_deployed() {
                menu_handle.hide(cx);
            } else {
                menu_handle.show(window, cx);
            }
        });
    }

    fn handle_split_toggle_menu(
        &mut self,
        _: &ToggleSplitMenu,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.picker.update(cx, |picker, cx| {
            let menu_handle = &picker.delegate.split_popover_menu_handle;
            if menu_handle.is_deployed() {
                menu_handle.hide(cx);
            } else {
                menu_handle.show(window, cx);
            }
        });
    }

    fn handle_toggle_ignored(
        &mut self,
        _: &ToggleIncludeIgnored,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.picker.update(cx, |picker, cx| {
            picker.delegate.include_ignored = match picker.delegate.include_ignored {
                Some(true) => FileFinderSettings::get_global(cx)
                    .include_ignored
                    .map(|_| false),
                Some(false) => Some(true),
                None => Some(true),
            };
            picker.delegate.include_ignored_refresh =
                picker.delegate.update_matches(picker.query(cx), window, cx);
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
            if let Some(workspace) = delegate.workspace.upgrade()
                && let Some(m) = delegate.matches.get(delegate.selected_index())
            {
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
                    Match::CreateNew(p) => p.clone(),
                };
                let open_task = workspace.update(cx, move |workspace, cx| {
                    workspace.split_path_preview(path, false, Some(split_direction), window, cx)
                });
                open_task.detach_and_log_err(cx);
            }
        })
    }

    pub fn modal_max_width(width_setting: FileFinderWidth, window: &mut Window) -> Pixels {
        let window_width = window.viewport_size().width;
        let small_width = rems(34.).to_pixels(window.rem_size());

        match width_setting {
            FileFinderWidth::Small => small_width,
            FileFinderWidth::Full => window_width,
            FileFinderWidth::XLarge => (window_width - Pixels(512.)).max(small_width),
            FileFinderWidth::Large => (window_width - Pixels(768.)).max(small_width),
            FileFinderWidth::Medium => (window_width - Pixels(1024.)).max(small_width),
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
            .on_action(cx.listener(Self::handle_filter_toggle_menu))
            .on_action(cx.listener(Self::handle_split_toggle_menu))
            .on_action(cx.listener(Self::handle_toggle_ignored))
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
    filter_popover_menu_handle: PopoverMenuHandle<ContextMenu>,
    split_popover_menu_handle: PopoverMenuHandle<ContextMenu>,
    focus_handle: FocusHandle,
    include_ignored: Option<bool>,
    include_ignored_refresh: Task<()>,
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
    CreateNew(ProjectPath),
}

impl Match {
    fn relative_path(&self) -> Option<&Arc<RelPath>> {
        match self {
            Match::History { path, .. } => Some(&path.project.path),
            Match::Search(panel_match) => Some(&panel_match.0.path),
            Match::CreateNew(_) => None,
        }
    }

    fn abs_path(&self, project: &Entity<Project>, cx: &App) -> Option<PathBuf> {
        match self {
            Match::History { path, .. } => Some(path.absolute.clone()),
            Match::Search(ProjectPanelOrdMatch(path_match)) => Some(
                project
                    .read(cx)
                    .worktree_for_id(WorktreeId::from_usize(path_match.worktree_id), cx)?
                    .read(cx)
                    .absolutize(&path_match.path),
            ),
            Match::CreateNew(_) => None,
        }
    }

    fn panel_match(&self) -> Option<&ProjectPanelOrdMatch> {
        match self {
            Match::History { panel_match, .. } => panel_match.as_ref(),
            Match::Search(panel_match) => Some(panel_match),
            Match::CreateNew(_) => None,
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
                .position(|m| match m.relative_path() {
                    Some(p) => path.project.path == *p,
                    None => false,
                })
                .ok_or(0)
        } else {
            self.matches.binary_search_by(|m| {
                // `reverse()` since if cmp_matches(a, b) == Ordering::Greater, then a is better than b.
                // And we want the better entries go first.
                Self::cmp_matches(self.separate_history, currently_opened, m, entry).reverse()
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
                .extend(history_items.into_iter().map(path_to_entry));
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
        // Handle CreateNew variant - always put it at the end
        match (a, b) {
            (Match::CreateNew(_), _) => return cmp::Ordering::Less,
            (_, Match::CreateNew(_)) => return cmp::Ordering::Greater,
            _ => {}
        }
        debug_assert!(a.panel_match().is_some() && b.panel_match().is_some());

        match (&a, &b) {
            // bubble currently opened files to the top
            (Match::History { path, .. }, _) if Some(path) == currently_opened => {
                return cmp::Ordering::Greater;
            }
            (_, Match::History { path, .. }) if Some(path) == currently_opened => {
                return cmp::Ordering::Less;
            }

            _ => {}
        }

        if separate_history {
            match (a, b) {
                (Match::History { .. }, Match::Search(_)) => return cmp::Ordering::Greater,
                (Match::Search(_), Match::History { .. }) => return cmp::Ordering::Less,

                _ => {}
            }
        }

        let a_panel_match = match a.panel_match() {
            Some(pm) => pm,
            None => {
                return if b.panel_match().is_some() {
                    cmp::Ordering::Less
                } else {
                    cmp::Ordering::Equal
                };
            }
        };

        let b_panel_match = match b.panel_match() {
            Some(pm) => pm,
            None => return cmp::Ordering::Greater,
        };

        let a_in_filename = Self::is_filename_match(a_panel_match);
        let b_in_filename = Self::is_filename_match(b_panel_match);

        match (a_in_filename, b_in_filename) {
            (true, false) => return cmp::Ordering::Greater,
            (false, true) => return cmp::Ordering::Less,
            _ => {} // Both are filename matches or both are path matches
        }

        a_panel_match.cmp(b_panel_match)
    }

    /// Determines if the match occurred within the filename rather than in the path
    fn is_filename_match(panel_match: &ProjectPanelOrdMatch) -> bool {
        if panel_match.0.positions.is_empty() {
            return false;
        }

        if let Some(filename) = panel_match.0.path.file_name() {
            let path_str = panel_match.0.path.as_unix_str();

            if let Some(filename_pos) = path_str.rfind(filename)
                && panel_match.0.positions[0] >= filename_pos
            {
                let mut prev_position = panel_match.0.positions[0];
                for p in &panel_match.0.positions[1..] {
                    if *p != prev_position + 1 {
                        return false;
                    }
                    prev_position = *p;
                }
                return true;
            }
        }

        false
    }
}

fn matching_history_items<'a>(
    history_items: impl IntoIterator<Item = &'a FoundPath>,
    currently_opened: Option<&'a FoundPath>,
    query: &FileSearchQuery,
) -> HashMap<Arc<RelPath>, Match> {
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
                        .to_string()
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
    absolute: PathBuf,
}

impl FoundPath {
    fn new(project: ProjectPath, absolute: PathBuf) -> Self {
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
            filter_popover_menu_handle: PopoverMenuHandle::default(),
            split_popover_menu_handle: PopoverMenuHandle::default(),
            focus_handle: cx.focus_handle(),
            include_ignored: FileFinderSettings::get_global(cx).include_ignored,
            include_ignored_refresh: Task::ready(()),
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
                    include_ignored: self.include_ignored.unwrap_or_else(|| {
                        worktree.root_entry().is_some_and(|entry| entry.is_ignored)
                    }),
                    include_root_name,
                    candidates: project::Candidates::Files,
                }
            })
            .collect::<Vec<_>>();

        let search_id = util::post_inc(&mut self.search_count);
        self.cancel_flag.store(true, atomic::Ordering::Release);
        self.cancel_flag = Arc::new(AtomicBool::new(false));
        let cancel_flag = self.cancel_flag.clone();
        cx.spawn_in(window, async move |picker, cx| {
            let matches = fuzzy::match_path_sets(
                candidate_sets.as_slice(),
                query.path_query(),
                &relative_to,
                false,
                100,
                &cancel_flag,
                cx.background_executor().clone(),
            )
            .await
            .into_iter()
            .map(ProjectPanelOrdMatch);
            let did_cancel = cancel_flag.load(atomic::Ordering::Acquire);
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

            let path_style = self.project.read(cx).path_style(cx);
            let query_path = query.raw_query.as_str();
            if let Ok(mut query_path) = RelPath::new(Path::new(query_path), path_style) {
                let available_worktree = self
                    .project
                    .read(cx)
                    .visible_worktrees(cx)
                    .filter(|worktree| !worktree.read(cx).is_single_file())
                    .collect::<Vec<_>>();
                let worktree_count = available_worktree.len();
                let mut expect_worktree = available_worktree.first().cloned();
                for worktree in available_worktree {
                    let worktree_root = worktree.read(cx).root_name();
                    if worktree_count > 1 {
                        if let Ok(suffix) = query_path.strip_prefix(worktree_root) {
                            query_path = Cow::Owned(suffix.to_owned());
                            expect_worktree = Some(worktree);
                            break;
                        }
                    }
                }

                if let Some(FoundPath { ref project, .. }) = self.currently_opened_path {
                    let worktree_id = project.worktree_id;
                    expect_worktree = self.project.read(cx).worktree_for_id(worktree_id, cx);
                }

                if let Some(worktree) = expect_worktree {
                    let worktree = worktree.read(cx);
                    if worktree.entry_for_path(&query_path).is_none()
                        && !query.raw_query.ends_with("/")
                        && !(path_style.is_windows() && query.raw_query.ends_with("\\"))
                    {
                        self.matches.matches.push(Match::CreateNew(ProjectPath {
                            worktree_id: worktree.id(),
                            path: query_path.into_arc(),
                        }));
                    }
                }
            }

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
    ) -> (HighlightedLabel, HighlightedLabel) {
        let path_style = self.project.read(cx).path_style(cx);
        let (file_name, file_name_positions, mut full_path, mut full_path_positions) =
            match &path_match {
                Match::History {
                    path: entry_path,
                    panel_match,
                } => {
                    let worktree_id = entry_path.project.worktree_id;
                    let worktree = self
                        .project
                        .read(cx)
                        .worktree_for_id(worktree_id, cx)
                        .filter(|worktree| worktree.read(cx).is_visible());

                    if let Some(panel_match) = panel_match {
                        self.labels_for_path_match(&panel_match.0, path_style)
                    } else if let Some(worktree) = worktree {
                        let full_path =
                            worktree.read(cx).root_name().join(&entry_path.project.path);
                        let mut components = full_path.components();
                        let filename = components.next_back().unwrap_or("");
                        let prefix = components.rest();
                        (
                            filename.to_string(),
                            Vec::new(),
                            prefix.display(path_style).to_string() + path_style.separator(),
                            Vec::new(),
                        )
                    } else {
                        (
                            entry_path
                                .absolute
                                .file_name()
                                .map_or(String::new(), |f| f.to_string_lossy().into_owned()),
                            Vec::new(),
                            entry_path.absolute.parent().map_or(String::new(), |path| {
                                path.to_string_lossy().into_owned() + path_style.separator()
                            }),
                            Vec::new(),
                        )
                    }
                }
                Match::Search(path_match) => self.labels_for_path_match(&path_match.0, path_style),
                Match::CreateNew(project_path) => (
                    format!("Create file: {}", project_path.path.display(path_style)),
                    vec![],
                    String::from(""),
                    vec![],
                ),
            };

        if file_name_positions.is_empty() {
            let user_home_path = util::paths::home_dir().to_string_lossy();
            if !user_home_path.is_empty() && full_path.starts_with(&*user_home_path) {
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
        path_style: PathStyle,
    ) -> (String, Vec<usize>, String, Vec<usize>) {
        let full_path = path_match.path_prefix.join(&path_match.path);
        let mut path_positions = path_match.positions.clone();

        let file_name = full_path.file_name().unwrap_or("");
        let file_name_start = full_path.as_unix_str().len() - file_name.len();
        let file_name_positions = path_positions
            .iter()
            .filter_map(|pos| {
                if pos >= &file_name_start {
                    Some(pos - file_name_start)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let full_path = full_path
            .display(path_style)
            .trim_end_matches(&file_name)
            .to_string();
        path_positions.retain(|idx| *idx < full_path.len());

        debug_assert!(
            file_name_positions
                .iter()
                .all(|ix| file_name[*ix..].chars().next().is_some()),
            "invalid file name positions {file_name:?} {file_name_positions:?}"
        );
        debug_assert!(
            path_positions
                .iter()
                .all(|ix| full_path[*ix..].chars().next().is_some()),
            "invalid path positions {full_path:?} {path_positions:?}"
        );

        (
            file_name.to_string(),
            file_name_positions,
            full_path,
            path_positions,
        )
    }

    fn lookup_absolute_path(
        &self,
        query: FileSearchQuery,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        cx.spawn_in(window, async move |picker, cx| {
            let Some(project) = picker
                .read_with(cx, |picker, _| picker.delegate.project.clone())
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
                                path: relative_path,
                                path_prefix: RelPath::empty().into(),
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
        if FileFinderSettings::get_global(cx).skip_focus_for_active_in_search
            && let Some(Match::History { path, .. }) = self.matches.get(0)
            && Some(path) == self.currently_opened_path.as_ref()
        {
            let elements_after_first = self.matches.len() - 1;
            if elements_after_first > 0 {
                return 1;
            }
        }

        0
    }

    fn key_context(&self, window: &Window, cx: &App) -> KeyContext {
        let mut key_context = KeyContext::new_with_defaults();
        key_context.add("FileFinder");

        if self.filter_popover_menu_handle.is_focused(window, cx) {
            key_context.add("filter_menu_open");
        }

        if self.split_popover_menu_handle.is_focused(window, cx) {
            key_context.add("split_menu_open");
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
            if let Some(first_non_history_index) = first_non_history_index
                && first_non_history_index > 0
            {
                return vec![first_non_history_index - 1];
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

        let raw_query = match &raw_query.get(0..2) {
            Some(".\\" | "./") => &raw_query[2..],
            Some(prefix @ ("a\\" | "a/" | "b\\" | "b/")) => {
                if self
                    .workspace
                    .upgrade()
                    .into_iter()
                    .flat_map(|workspace| workspace.read(cx).worktrees(cx))
                    .all(|worktree| {
                        worktree
                            .read(cx)
                            .entry_for_path(RelPath::unix(prefix.split_at(1).0).unwrap())
                            .is_none_or(|entry| !entry.is_dir())
                    })
                {
                    &raw_query[2..]
                } else {
                    raw_query
                }
            }
            _ => raw_query,
        };

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
                            || project.is_local()
                            || project.is_via_remote_server()
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
            let path_position = PathWithPosition::parse_str(raw_query);
            let raw_query = raw_query.trim().trim_end_matches(':').to_owned();
            let path = path_position.path.to_str();
            let path_trimmed = path.unwrap_or(&raw_query).trim_end_matches(':');
            let file_query_end = if path_trimmed == raw_query {
                None
            } else {
                // Safe to unwrap as we won't get here when the unwrap in if fails
                Some(path.unwrap().len())
            };

            let query = FileSearchQuery {
                raw_query,
                file_query_end,
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
        if let Some(m) = self.matches.get(self.selected_index())
            && let Some(workspace) = self.workspace.upgrade()
        {
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
                    Match::CreateNew(project_path) => {
                        // Create a new file with the given filename
                        if secondary {
                            workspace.split_path_preview(
                                project_path.clone(),
                                false,
                                None,
                                window,
                                cx,
                            )
                        } else {
                            workspace.open_path_preview(
                                project_path.clone(),
                                None,
                                true,
                                false,
                                true,
                                window,
                                cx,
                            )
                        }
                    }

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
                        } else if secondary {
                            workspace.split_abs_path(path.absolute.clone(), false, window, cx)
                        } else {
                            workspace.open_abs_path(
                                path.absolute.clone(),
                                OpenOptions {
                                    visible: Some(OpenVisible::None),
                                    ..Default::default()
                                },
                                window,
                                cx,
                            )
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
                if let Some(row) = row
                    && let Some(active_editor) = item.downcast::<Editor>()
                {
                    active_editor
                        .downgrade()
                        .update_in(cx, |editor, window, cx| {
                            editor.go_to_singleton_buffer_point(Point::new(row, col), window, cx);
                        })
                        .log_err();
                }
                finder.update(cx, |_, cx| cx.emit(DismissEvent)).ok()?;

                Some(())
            })
            .detach();
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

        let path_match = self.matches.get(ix)?;

        let history_icon = match &path_match {
            Match::History { .. } => Icon::new(IconName::HistoryRerun)
                .color(Color::Muted)
                .size(IconSize::Small)
                .into_any_element(),
            Match::Search(_) => v_flex()
                .flex_none()
                .size(IconSize::Small.rems())
                .into_any_element(),
            Match::CreateNew(_) => Icon::new(IconName::Plus)
                .color(Color::Muted)
                .size(IconSize::Small)
                .into_any_element(),
        };
        let (file_name_label, full_path_label) = self.labels_for_match(path_match, window, cx);

        let file_icon = maybe!({
            if !settings.file_icons {
                return None;
            }
            let abs_path = path_match.abs_path(&self.project, cx)?;
            let file_name = abs_path.file_name()?;
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

    fn render_footer(
        &self,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<AnyElement> {
        let focus_handle = self.focus_handle.clone();

        Some(
            h_flex()
                .w_full()
                .p_1p5()
                .justify_between()
                .border_t_1()
                .border_color(cx.theme().colors().border_variant)
                .child(
                    PopoverMenu::new("filter-menu-popover")
                        .with_handle(self.filter_popover_menu_handle.clone())
                        .attach(gpui::Corner::BottomRight)
                        .anchor(gpui::Corner::BottomLeft)
                        .offset(gpui::Point {
                            x: px(1.0),
                            y: px(1.0),
                        })
                        .trigger_with_tooltip(
                            IconButton::new("filter-trigger", IconName::Sliders)
                                .icon_size(IconSize::Small)
                                .icon_size(IconSize::Small)
                                .toggle_state(self.include_ignored.unwrap_or(false))
                                .when(self.include_ignored.is_some(), |this| {
                                    this.indicator(Indicator::dot().color(Color::Info))
                                }),
                            {
                                let focus_handle = focus_handle.clone();
                                move |window, cx| {
                                    Tooltip::for_action_in(
                                        "Filter Options",
                                        &ToggleFilterMenu,
                                        &focus_handle,
                                        window,
                                        cx,
                                    )
                                }
                            },
                        )
                        .menu({
                            let focus_handle = focus_handle.clone();
                            let include_ignored = self.include_ignored;

                            move |window, cx| {
                                Some(ContextMenu::build(window, cx, {
                                    let focus_handle = focus_handle.clone();
                                    move |menu, _, _| {
                                        menu.context(focus_handle.clone())
                                            .header("Filter Options")
                                            .toggleable_entry(
                                                "Include Ignored Files",
                                                include_ignored.unwrap_or(false),
                                                ui::IconPosition::End,
                                                Some(ToggleIncludeIgnored.boxed_clone()),
                                                move |window, cx| {
                                                    window.focus(&focus_handle);
                                                    window.dispatch_action(
                                                        ToggleIncludeIgnored.boxed_clone(),
                                                        cx,
                                                    );
                                                },
                                            )
                                    }
                                }))
                            }
                        }),
                )
                .child(
                    h_flex()
                        .gap_0p5()
                        .child(
                            PopoverMenu::new("split-menu-popover")
                                .with_handle(self.split_popover_menu_handle.clone())
                                .attach(gpui::Corner::BottomRight)
                                .anchor(gpui::Corner::BottomLeft)
                                .offset(gpui::Point {
                                    x: px(1.0),
                                    y: px(1.0),
                                })
                                .trigger(
                                    ButtonLike::new("split-trigger")
                                        .child(Label::new("Split…"))
                                        .selected_style(ButtonStyle::Tinted(TintColor::Accent))
                                        .children(
                                            KeyBinding::for_action_in(
                                                &ToggleSplitMenu,
                                                &focus_handle,
                                                window,
                                                cx,
                                            )
                                            .map(|kb| kb.size(rems_from_px(12.))),
                                        ),
                                )
                                .menu({
                                    let focus_handle = focus_handle.clone();

                                    move |window, cx| {
                                        Some(ContextMenu::build(window, cx, {
                                            let focus_handle = focus_handle.clone();
                                            move |menu, _, _| {
                                                menu.context(focus_handle)
                                                    .action(
                                                        "Split Left",
                                                        pane::SplitLeft.boxed_clone(),
                                                    )
                                                    .action(
                                                        "Split Right",
                                                        pane::SplitRight.boxed_clone(),
                                                    )
                                                    .action("Split Up", pane::SplitUp.boxed_clone())
                                                    .action(
                                                        "Split Down",
                                                        pane::SplitDown.boxed_clone(),
                                                    )
                                            }
                                        }))
                                    }
                                }),
                        )
                        .child(
                            Button::new("open-selection", "Open")
                                .key_binding(
                                    KeyBinding::for_action_in(
                                        &menu::Confirm,
                                        &focus_handle,
                                        window,
                                        cx,
                                    )
                                    .map(|kb| kb.size(rems_from_px(12.))),
                                )
                                .on_click(|_, window, cx| {
                                    window.dispatch_action(menu::Confirm.boxed_clone(), cx)
                                }),
                        ),
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
