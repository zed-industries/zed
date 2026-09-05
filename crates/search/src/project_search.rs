use crate::{
    BufferSearchBar, EXCLUDE_PLACEHOLDER, FocusSearch, HighlightKey, INCLUDE_PLACEHOLDER,
    NextHistoryQuery, PreviousHistoryQuery, REPLACE_PLACEHOLDER, ReplaceAll, ReplaceNext,
    SearchOption, SearchOptions, SearchSource, SelectNextMatch, SelectPreviousMatch,
    ToggleCaseSensitive, ToggleIncludeIgnored, ToggleRegex, ToggleReplace, ToggleWholeWord,
    buffer_search::Deploy,
    search_bar::{
        ActionButtonState, HistoryNavigationDirection, alignment_element, input_base_styles,
        render_action_button, render_text_input, should_navigate_history,
    },
    text_finder::TextFinder,
};
use anyhow::Context as _;
use collections::HashMap;
use editor::{
    Anchor, Editor, EditorEvent, EditorSettings, MAX_TAB_TITLE_LEN, MultiBuffer, PathKey,
    SearchResultsStatus, SelectionEffects,
    actions::{Backtab, FoldAll, SelectAll, Tab, UnfoldAll},
    items::active_match_index,
    multibuffer_context_lines,
    scroll::Autoscroll,
};
use futures::{StreamExt, stream::FuturesOrdered};
use gpui::{
    Action, AnyElement, App, AsyncApp, Context, Entity, EntityId, EventEmitter, FocusHandle,
    Focusable, Global, Hsla, InteractiveElement, IntoElement, KeyContext, ParentElement, Point,
    Render, SharedString, Styled, Subscription, Task, TaskExt, UpdateGlobal, WeakEntity, Window,
    actions, div,
};
use itertools::Itertools;
use language::{Buffer, Language};
use menu::Confirm;
use multi_buffer;
use project::{
    Project, ProjectPath, SearchResults,
    search::{SearchInputKind, SearchQuery, SearchResult},
    search_history::SearchHistoryCursor,
};
use settings::Settings;
use std::{
    any::{Any, TypeId},
    iter::Peekable,
    mem,
    ops::{Not, Range},
    pin::pin,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};
use text::OffsetRangeExt;
use ui::{
    CommonAnimationExt, IconButtonShape, KeyBinding, Toggleable, Tooltip, prelude::*,
    utils::SearchInputWidth,
};
use util::{ResultExt as _, paths::PathMatcher};
use workspace::{
    DeploySearch, ItemNavHistory, NewSearch, ToolbarItemEvent, ToolbarItemLocation,
    ToolbarItemView, Workspace, WorkspaceId,
    item::{Item, ItemBufferKind, ItemEvent, ItemHandle, SaveOptions},
    searchable::{Direction, SearchEvent, SearchToken, SearchableItem, SearchableItemHandle},
};

actions!(
    project_search,
    [
        /// Searches in a new project search tab.
        SearchInNew,
        /// Toggles focus between the search bar and the search results.
        ToggleFocus,
        /// Moves to the next input field.
        NextField,
        /// Toggles the search filters panel.
        ToggleFilters,
        /// Toggles collapse/expand state of all search result excerpts.
        ToggleAllSearchResults,
        /// Open a text picker showing the current result in a modal.
        OpenTextFinder
    ]
);

fn split_glob_patterns(text: &str) -> Vec<&str> {
    let mut patterns = Vec::new();
    let mut pattern_start = 0;
    let mut brace_depth: usize = 0;
    let mut escaped = false;

    for (index, character) in text.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        match character {
            '\\' => escaped = true,
            '{' => brace_depth += 1,
            '}' => brace_depth = brace_depth.saturating_sub(1),
            ',' if brace_depth == 0 => {
                patterns.push(&text[pattern_start..index]);
                pattern_start = index + 1;
            }
            _ => {}
        }
    }
    patterns.push(&text[pattern_start..]);
    patterns
}

#[derive(Default)]
pub(crate) struct ActiveSettings(pub(crate) HashMap<WeakEntity<Project>, ProjectSearchSettings>);

impl Global for ActiveSettings {}

pub fn init(cx: &mut App) {
    cx.set_global(ActiveSettings::default());
    cx.observe_new(|workspace: &mut Workspace, _window, _cx| {
        register_workspace_action(workspace, move |search_bar, _: &Deploy, window, cx| {
            search_bar.focus_search(window, cx);
        });
        register_workspace_action(workspace, move |search_bar, _: &FocusSearch, window, cx| {
            search_bar.focus_search(window, cx);
        });
        register_workspace_action(
            workspace,
            move |search_bar, _: &ToggleFilters, window, cx| {
                search_bar.toggle_filters(window, cx);
            },
        );
        register_workspace_action(
            workspace,
            move |search_bar, _: &ToggleCaseSensitive, window, cx| {
                search_bar.toggle_search_option(SearchOptions::CASE_SENSITIVE, window, cx);
            },
        );
        register_workspace_action(
            workspace,
            move |search_bar, _: &ToggleWholeWord, window, cx| {
                search_bar.toggle_search_option(SearchOptions::WHOLE_WORD, window, cx);
            },
        );
        register_workspace_action(workspace, move |search_bar, _: &ToggleRegex, window, cx| {
            search_bar.toggle_search_option(SearchOptions::REGEX, window, cx);
        });
        register_workspace_action(
            workspace,
            move |search_bar, action: &ToggleReplace, window, cx| {
                search_bar.toggle_replace(action, window, cx)
            },
        );
        register_workspace_action(
            workspace,
            move |search_bar, action: &SelectPreviousMatch, window, cx| {
                search_bar.select_prev_match(action, window, cx)
            },
        );
        register_workspace_action(
            workspace,
            move |search_bar, action: &SelectNextMatch, window, cx| {
                search_bar.select_next_match(action, window, cx)
            },
        );

        // Only handle search_in_new if there is a search present
        register_workspace_action_for_present_search(workspace, |workspace, action, window, cx| {
            ProjectSearchView::search_in_new(workspace, action, window, cx)
        });

        register_workspace_action_for_present_search(
            workspace,
            |workspace, action: &ToggleAllSearchResults, window, cx| {
                if let Some(search_view) = workspace
                    .active_item(cx)
                    .and_then(|item| item.downcast::<ProjectSearchView>())
                {
                    search_view.update(cx, |search_view, cx| {
                        search_view.toggle_all_search_results(action, window, cx);
                    });
                }
            },
        );

        register_workspace_action_for_present_search(
            workspace,
            |workspace, _: &menu::Cancel, window, cx| {
                if let Some(project_search_bar) = workspace
                    .active_pane()
                    .read(cx)
                    .toolbar()
                    .read(cx)
                    .item_of_type::<ProjectSearchBar>()
                {
                    project_search_bar.update(cx, |project_search_bar, cx| {
                        let search_is_focused = project_search_bar
                            .active_project_search
                            .as_ref()
                            .is_some_and(|search_view| {
                                search_view
                                    .read(cx)
                                    .query_editor
                                    .read(cx)
                                    .focus_handle(cx)
                                    .is_focused(window)
                            });
                        if search_is_focused {
                            project_search_bar.move_focus_to_results(window, cx);
                        } else {
                            project_search_bar.focus_search(window, cx)
                        }
                    });
                } else {
                    cx.propagate();
                }
            },
        );

        // Both on present and dismissed search, we need to unconditionally handle those actions to focus from the editor.
        workspace.register_action(move |workspace, action: &DeploySearch, window, cx| {
            if workspace.has_active_modal(window, cx) && !workspace.hide_modal(window, cx) {
                cx.propagate();
                return;
            }
            ProjectSearchView::deploy_search(workspace, action, window, cx);
            cx.notify();
        });
        workspace.register_action(move |workspace, action: &NewSearch, window, cx| {
            if workspace.has_active_modal(window, cx) && !workspace.hide_modal(window, cx) {
                cx.propagate();
                return;
            }
            ProjectSearchView::new_search(workspace, action, window, cx);
            cx.notify();
        });
        workspace.register_action(
            move |workspace, action: &zed_actions::search::NewSearchInDirectory, window, cx| {
                ProjectSearchView::new_search_with_filter(
                    workspace,
                    action.directory.clone(),
                    window,
                    cx,
                );
                cx.notify();
            },
        );
    })
    .detach();
}

fn contains_uppercase(str: &str) -> bool {
    str.chars().any(|c| c.is_uppercase())
}

const SEARCH_ON_TYPE_DEBOUNCE: Duration = Duration::from_millis(250);

pub struct ProjectSearch {
    pub(crate) project: Entity<Project>,
    workspace: WeakEntity<Workspace>,
    pub excerpts: Entity<MultiBuffer>,
    pub pending_search: Option<Task<Option<SearchResults<SearchResult>>>>,
    pub match_ranges: Vec<Range<Anchor>>,
    pub(crate) active_query: Option<SearchQuery>,
    last_search_query_text: Option<String>,
    pub search_id: usize,
    search_state: SearchState,
    phase: SearchPhase,
    reuses_excerpts: bool,
    results_refreshed: bool,
    search_history_cursor: SearchHistoryCursor,
    search_included_history_cursor: SearchHistoryCursor,
    search_excluded_history_cursor: SearchHistoryCursor,
    pub project_search_turning_into_text_finder: Arc<AtomicBool>,
    _excerpts_subscription: Subscription,
    _workspace_subscription: Option<Subscription>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SearchMode {
    Manual,
    OnType,
    Refresh,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct SearchSignature {
    query: String,
    included_files: String,
    excluded_files: String,
    options: SearchOptions,
    filters_enabled: bool,
    included_opened_only: bool,
    opened_buffers: Option<Vec<EntityId>>,
}

/// Tracks how the current results were produced, so the view knows whether to preserve the
/// user's scroll/selection (mid-typing) or take focus and jump to the first match (confirmed).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum SearchPhase {
    #[default]
    Idle,
    /// A search-on-type run whose results the user has not confirmed yet.
    Typing,
    /// A manual search, or an on-type search the user confirmed with Enter.
    Confirmed,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum SearchState {
    #[default]
    Idle,
    Running {
        activity: SearchActivity,
        previous_completion: Option<SearchCompletion>,
    },
    Completed(SearchCompletion),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SearchActivity {
    Searching,
    WaitingForScan,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SearchCompletion {
    NoResults {
        deferred_dirs: u32,
    },
    Results {
        limit_reached: bool,
        deferred_dirs: u32,
    },
}

impl Default for SearchCompletion {
    fn default() -> Self {
        SearchCompletion::NoResults { deferred_dirs: 0 }
    }
}

fn partial_index_message(deferred_dirs: u32) -> String {
    format!(
        "Results may be incomplete — {deferred_dirs} director{} {} not indexed due to `file_scan_depth`. Add paths to `file_scan_inclusions` or raise `file_scan_depth` to search them.",
        if deferred_dirs == 1 { "y" } else { "ies" },
        if deferred_dirs == 1 { "was" } else { "were" },
    )
}

#[cfg(test)]
mod partial_index_message_tests {
    use super::partial_index_message;

    #[test]
    fn singular() {
        assert_eq!(
            partial_index_message(1),
            "Results may be incomplete — 1 directory was not indexed due to `file_scan_depth`. Add paths to `file_scan_inclusions` or raise `file_scan_depth` to search them."
        );
    }

    #[test]
    fn plural() {
        assert_eq!(
            partial_index_message(42),
            "Results may be incomplete — 42 directories were not indexed due to `file_scan_depth`. Add paths to `file_scan_inclusions` or raise `file_scan_depth` to search them."
        );
    }
}

impl SearchState {
    fn completion(self) -> Option<SearchCompletion> {
        match self {
            SearchState::Idle => None,
            SearchState::Running {
                previous_completion,
                ..
            } => previous_completion,
            SearchState::Completed(completion) => Some(completion),
        }
    }

    fn no_results_so_far(self) -> bool {
        matches!(self.completion(), Some(SearchCompletion::NoResults { .. }))
    }

    fn limit_reached(self) -> bool {
        matches!(
            self.completion(),
            Some(SearchCompletion::Results {
                limit_reached: true,
                ..
            })
        )
    }

    fn deferred_dirs(self) -> u32 {
        match self.completion() {
            Some(SearchCompletion::NoResults { deferred_dirs })
            | Some(SearchCompletion::Results { deferred_dirs, .. }) => deferred_dirs,
            None => 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum InputPanel {
    Query,
    Replacement,
    Exclude,
    Include,
}

pub struct ProjectSearchView {
    pub(crate) workspace: WeakEntity<Workspace>,
    focus_handle: FocusHandle,
    pub(crate) entity: Entity<ProjectSearch>,
    query_editor: Entity<Editor>,
    replacement_editor: Entity<Editor>,
    results_editor: Entity<Editor>,
    pub(crate) search_options: SearchOptions,
    panels_with_errors: HashMap<InputPanel, String>,
    active_match_index: Option<usize>,
    search_id: usize,
    included_files_editor: Entity<Editor>,
    excluded_files_editor: Entity<Editor>,
    filters_enabled: bool,
    replace_enabled: bool,
    pending_replace_all: bool,
    pending_replace_next: bool,
    included_opened_only: bool,
    regex_language: Option<Arc<Language>>,
    debounced_search: Option<Task<()>>,
    last_search_signature: Option<SearchSignature>,
    _subscriptions: Vec<Subscription>,
}

#[derive(Debug, Clone)]
pub struct ProjectSearchSettings {
    search_options: SearchOptions,
    filters_enabled: bool,
}

pub struct ProjectSearchBar {
    active_project_search: Option<Entity<ProjectSearchView>>,
    subscription: Option<Subscription>,
}

impl ProjectSearch {
    pub fn new(
        project: Entity<Project>,
        workspace: WeakEntity<Workspace>,
        cx: &mut Context<Self>,
    ) -> Self {
        let capability = project.read(cx).capability();
        let excerpts = cx.new(|_| MultiBuffer::new(capability));
        let excerpts_subscription = Self::subscribe_to_excerpts(&excerpts, cx);
        let workspace_subscription = Self::subscribe_to_workspace(&workspace, cx);

        Self {
            project,
            workspace,
            excerpts,
            pending_search: Default::default(),
            match_ranges: Default::default(),
            active_query: None,
            last_search_query_text: None,
            search_id: 0,
            search_state: SearchState::Idle,
            phase: SearchPhase::Idle,
            reuses_excerpts: false,
            results_refreshed: true,
            search_history_cursor: Default::default(),
            search_included_history_cursor: Default::default(),
            search_excluded_history_cursor: Default::default(),
            project_search_turning_into_text_finder: Arc::new(AtomicBool::new(false)),
            _excerpts_subscription: excerpts_subscription,
            _workspace_subscription: workspace_subscription,
        }
    }

    fn clone(&self, cx: &mut Context<Self>) -> Entity<Self> {
        cx.new(|cx| {
            let excerpts = self
                .excerpts
                .update(cx, |excerpts, cx| cx.new(|cx| excerpts.clone(cx)));
            let excerpts_subscription = Self::subscribe_to_excerpts(&excerpts, cx);
            let workspace_subscription = Self::subscribe_to_workspace(&self.workspace, cx);

            Self {
                project: self.project.clone(),
                workspace: self.workspace.clone(),
                excerpts,
                pending_search: Default::default(),
                match_ranges: self.match_ranges.clone(),
                active_query: self.active_query.clone(),
                last_search_query_text: self.last_search_query_text.clone(),
                search_id: self.search_id,
                search_state: if self.pending_search.is_some() {
                    SearchState::Idle
                } else {
                    self.search_state
                },
                phase: if self.phase == SearchPhase::Confirmed {
                    SearchPhase::Confirmed
                } else {
                    SearchPhase::Idle
                },
                reuses_excerpts: false,
                results_refreshed: true,
                search_history_cursor: self.search_history_cursor.clone(),
                search_included_history_cursor: self.search_included_history_cursor.clone(),
                search_excluded_history_cursor: self.search_excluded_history_cursor.clone(),
                project_search_turning_into_text_finder: Arc::new(AtomicBool::new(false)),
                _excerpts_subscription: excerpts_subscription,
                _workspace_subscription: workspace_subscription,
            }
        })
    }
    fn subscribe_to_excerpts(
        excerpts: &Entity<MultiBuffer>,
        cx: &mut Context<Self>,
    ) -> Subscription {
        cx.subscribe(excerpts, |this, _, event, cx| {
            if matches!(event, multi_buffer::Event::FileHandleChanged) {
                this.remove_deleted_buffers(cx);
            }
        })
    }

    fn subscribe_to_workspace(
        workspace: &WeakEntity<Workspace>,
        cx: &mut Context<Self>,
    ) -> Option<Subscription> {
        workspace.upgrade().map(|workspace| {
            cx.subscribe(&workspace, |this, _, event, cx| {
                if matches!(event, workspace::Event::ItemRemoved { .. }) {
                    this.remove_closed_untitled_buffers(cx);
                }
            })
        })
    }

    fn remove_deleted_buffers(&mut self, cx: &mut Context<Self>) {
        self.remove_stale_buffers(None, cx);
    }

    fn remove_closed_untitled_buffers(&mut self, cx: &mut Context<Self>) {
        self.remove_stale_buffers(self.workspace.upgrade().as_ref(), cx);
    }

    fn remove_stale_buffers(
        &mut self,
        workspace: Option<&Entity<Workspace>>,
        cx: &mut Context<Self>,
    ) {
        let stale_buffer_ids = self
            .excerpts
            .read(cx)
            .all_buffers_iter()
            .filter(|buffer| is_buffer_stale(&self.project, workspace, buffer, cx))
            .map(|buffer| buffer.read(cx).remote_id())
            .collect::<Vec<_>>();

        if stale_buffer_ids.is_empty() {
            return;
        }

        let snapshot = self.excerpts.update(cx, |excerpts, cx| {
            for buffer_id in stale_buffer_ids {
                excerpts.remove_excerpts_for_buffer(buffer_id, cx);
            }
            excerpts.snapshot(cx)
        });

        if self.pending_search.is_none() {
            self.match_ranges
                .retain(|range| snapshot.anchor_to_buffer_anchor(range.start).is_some());
        }

        cx.notify();
    }

    fn cursor(&self, kind: SearchInputKind) -> &SearchHistoryCursor {
        match kind {
            SearchInputKind::Query => &self.search_history_cursor,
            SearchInputKind::Include => &self.search_included_history_cursor,
            SearchInputKind::Exclude => &self.search_excluded_history_cursor,
        }
    }
    fn cursor_mut(&mut self, kind: SearchInputKind) -> &mut SearchHistoryCursor {
        match kind {
            SearchInputKind::Query => &mut self.search_history_cursor,
            SearchInputKind::Include => &mut self.search_included_history_cursor,
            SearchInputKind::Exclude => &mut self.search_excluded_history_cursor,
        }
    }

    fn search(
        &mut self,
        query: SearchQuery,
        mode: SearchMode,
        retains_verdict: bool,
        cx: &mut Context<Self>,
    ) {
        let project_search_turning_into_text_finder =
            Arc::clone(&self.project_search_turning_into_text_finder);
        let search = self
            .project
            .update(cx, |project, cx| project.search(query.clone(), cx));
        self.last_search_query_text = Some(query.as_str().to_string());
        self.search_id += 1;
        self.active_query = Some(query);
        match mode {
            SearchMode::Manual => {
                self.record_search_history(cx);
                self.phase = SearchPhase::Confirmed;
            }
            SearchMode::Refresh if self.phase == SearchPhase::Confirmed => {}
            SearchMode::OnType | SearchMode::Refresh => self.phase = SearchPhase::Typing,
        }
        self.reuses_excerpts = true;
        self.results_refreshed = false;
        self.search_state = SearchState::Running {
            activity: SearchActivity::Searching,
            previous_completion: if retains_verdict {
                self.search_state.completion()
            } else {
                None
            },
        };
        self.pending_search = Some(cx.spawn(async move |project_search, cx| {
            consume_search_stream(
                project_search,
                search,
                project_search_turning_into_text_finder,
                cx,
            )
            .await
        }));
        cx.notify();
    }

    fn clear(&mut self, cx: &mut Context<Self>) {
        self.pending_search = None;
        self.match_ranges.clear();
        self.excerpts.update(cx, |excerpts, cx| excerpts.clear(cx));
        self.search_state = SearchState::Idle;
        self.phase = SearchPhase::Idle;
        self.reuses_excerpts = false;
        self.results_refreshed = true;
        self.active_query = None;
        self.last_search_query_text = None;
        cx.notify();
    }

    fn record_search_history(&mut self, cx: &mut Context<Self>) {
        let Some(query) = self.active_query.clone() else {
            return;
        };
        self.project.update(cx, |project, _| {
            project
                .search_history_mut(SearchInputKind::Query)
                .add(&mut self.search_history_cursor, query.as_str().to_string());
            let included = query.as_inner().files_to_include().sources().join(",");
            if !included.is_empty() {
                project
                    .search_history_mut(SearchInputKind::Include)
                    .add(&mut self.search_included_history_cursor, included);
            }
            let excluded = query.as_inner().files_to_exclude().sources().join(",");
            if !excluded.is_empty() {
                project
                    .search_history_mut(SearchInputKind::Exclude)
                    .add(&mut self.search_excluded_history_cursor, excluded);
            }
        });
    }

    // At the point this is called the multibuffer has already been filled with
    // plundered results from the text finder
    pub(crate) fn hook_up_ongoing_search(
        &mut self,
        search_results: SearchResults<SearchResult>,
        cx: &mut Context<Self>,
    ) {
        let project_search_turning_into_text_finder =
            Arc::clone(&self.project_search_turning_into_text_finder);

        self.reuses_excerpts = false;
        self.results_refreshed = true;
        self.pending_search = Some(cx.spawn(async move |project_search, cx| {
            consume_search_stream(
                project_search,
                search_results,
                project_search_turning_into_text_finder,
                cx,
            )
            .await
        }));
        cx.notify();
    }
}

/// Drain a search result stream into the project search's multibuffer.
///
/// When the model is set to reuse excerpts, the previous matches are kept grouped by path and
/// edited in place as the (PathKey-sorted) results stream in: unchanged files keep their excerpts
/// and highlights, replaced files are updated, and files that stopped matching are pruned. The
/// model's `match_ranges` is refreshed per chunk from that grouping, so highlights track the
/// results as they arrive instead of blinking in only once the stream finishes.
async fn consume_search_stream(
    project_search: WeakEntity<ProjectSearch>,
    search_results: SearchResults<SearchResult>,
    project_search_turning_into_text_finder: Arc<AtomicBool>,
    cx: &mut AsyncApp,
) -> Option<SearchResults<SearchResult>> {
    let reuse_excerpts = project_search
        .read_with(cx, |project_search, _| project_search.reuses_excerpts)
        .ok()?;
    // Note: is cancel safe
    let mut matches = pin!(search_results.rx.clone().ready_chunks(1024));

    let mut limit_reached = false;
    let mut partial_deferred_dirs: u32 = 0;
    let mut reused_results = if reuse_excerpts {
        Some(project_search.read_with(cx, ReusedResults::new).ok()?)
    } else {
        None
    };
    while let Some(results) = matches.next().await {
        let (buffers_with_ranges, has_reached_limit, search_activity, partial_index) = cx
            .background_executor()
            .spawn(async move {
                let mut limit_reached = false;
                let mut search_activity = None;
                let mut partial_index: u32 = 0;
                let mut buffers_with_ranges = Vec::with_capacity(results.len());
                for result in results {
                    match result {
                        project::search::SearchResult::Buffer { buffer, ranges } => {
                            buffers_with_ranges.push((buffer, ranges));
                        }
                        project::search::SearchResult::LimitReached => {
                            limit_reached = true;
                        }
                        project::search::SearchResult::WaitingForScan => {
                            search_activity = Some(SearchActivity::WaitingForScan);
                        }
                        project::search::SearchResult::Searching => {
                            search_activity = Some(SearchActivity::Searching);
                        }
                        project::search::SearchResult::PartialIndex { deferred_dirs } => {
                            partial_index = partial_index.saturating_add(deferred_dirs);
                        }
                    }
                }
                (
                    buffers_with_ranges,
                    limit_reached,
                    search_activity,
                    partial_index,
                )
            })
            .await;
        limit_reached |= has_reached_limit;
        partial_deferred_dirs = partial_deferred_dirs.saturating_add(partial_index);
        if let Some(search_activity) = search_activity {
            project_search
                .update(cx, |project_search, cx| {
                    project_search.search_state = SearchState::Running {
                        activity: search_activity,
                        previous_completion: project_search.search_state.completion(),
                    };
                    cx.notify();
                })
                .ok()?;
        }

        if let Some(reused_results) = &mut reused_results {
            apply_reused_chunk(&project_search, buffers_with_ranges, reused_results, cx).await?;
        } else {
            let mut new_ranges = project_search
                .update(cx, |project_search, cx| {
                    project_search.excerpts.update(cx, |excerpts, cx| {
                        buffers_with_ranges
                            .into_iter()
                            .map(|(buffer, ranges)| {
                                let new_ranges = excerpts.set_anchored_excerpts_for_path(
                                    PathKey::for_buffer(&buffer, cx),
                                    buffer.clone(),
                                    ranges,
                                    multibuffer_context_lines(cx),
                                    cx,
                                );
                                async move { (buffer, new_ranges.await) }
                            })
                            .collect::<FuturesOrdered<_>>()
                    })
                })
                .ok()?;
            while let Some((buffer, new_ranges)) = new_ranges.next().await {
                // `new_ranges.next().await` likely never gets hit while still pending so `async_task`
                // will not reschedule, starving other front end tasks, insert a yield point for that here
                smol::future::yield_now().await;
                project_search
                    .update(cx, |project_search, cx| {
                        let workspace = project_search.workspace.upgrade();
                        if !is_buffer_stale(
                            &project_search.project,
                            workspace.as_ref(),
                            &buffer,
                            cx,
                        ) {
                            project_search.match_ranges.extend(new_ranges);
                        } else {
                            let buffer_id = buffer.read(cx).remote_id();
                            project_search.excerpts.update(cx, |excerpts, cx| {
                                excerpts.remove_excerpts_for_buffer(buffer_id, cx)
                            });
                        }
                        cx.notify();
                    })
                    .ok()?;
            }
        }

        // We do not want to end the task before all the results taken
        // from the mpsc rx are in
        if project_search_turning_into_text_finder.load(Ordering::Relaxed) {
            break;
        }
    }

    if project_search_turning_into_text_finder.load(Ordering::Relaxed) {
        project_search_turning_into_text_finder.store(false, Ordering::Relaxed); // reset
        if let Some(reused_results) = reused_results {
            project_search
                .update(cx, |project_search, cx| {
                    reused_results.finish(project_search, cx);
                    cx.notify();
                })
                .ok()?;
        }
        return Some(search_results);
    }

    project_search
        .update(cx, |project_search, cx| {
            if let Some(reused_results) = reused_results {
                reused_results.finish(project_search, cx);
            }
            project_search.search_state = if project_search.match_ranges.is_empty() {
                SearchState::Completed(SearchCompletion::NoResults {
                    deferred_dirs: partial_deferred_dirs,
                })
            } else {
                SearchState::Completed(SearchCompletion::Results {
                    limit_reached,
                    deferred_dirs: partial_deferred_dirs,
                })
            };
            project_search.pending_search.take();
            cx.notify();
        })
        .ok()?;

    None
}

struct ReusedResults {
    previous_groups: Peekable<std::vec::IntoIter<(PathKey, Range<usize>)>>,
    previous_ranges: Vec<Range<Anchor>>,
    suffix_start: usize,
    confirmed_ranges: Vec<Range<Anchor>>,
    last_seen_path: Option<PathKey>,
    reported_out_of_order: bool,
}

impl ReusedResults {
    fn new(project_search: &ProjectSearch, cx: &App) -> Self {
        let previous_ranges = project_search.match_ranges.clone();
        let previous_groups = project_search
            .excerpts
            .read(cx)
            .ranges_grouped_by_excerpt_path(&previous_ranges)
            .into_iter()
            .peekable();
        Self {
            previous_groups,
            previous_ranges,
            suffix_start: 0,
            confirmed_ranges: Vec::new(),
            last_seen_path: None,
            reported_out_of_order: false,
        }
    }

    fn prune_stale_paths_before(
        &mut self,
        path_key: Option<&PathKey>,
        excerpts: &mut MultiBuffer,
        cx: &mut Context<MultiBuffer>,
    ) -> bool {
        let mut stale_paths = Vec::new();
        while let Some((stale_path, span)) = self
            .previous_groups
            .next_if(|(previous_path, _)| path_key.is_none_or(|path| previous_path < path))
        {
            stale_paths.push(stale_path);
            self.suffix_start = span.end;
        }
        if stale_paths.is_empty() {
            return false;
        }
        excerpts.remove_excerpts_for_paths(stale_paths, cx);
        true
    }

    fn previous_ranges_for(&mut self, path_key: &PathKey) -> Option<Range<usize>> {
        let (_, span) = self
            .previous_groups
            .next_if(|(previous_path, _)| previous_path == path_key)?;
        self.suffix_start = span.end;
        Some(span)
    }

    fn current_match_ranges(&self) -> Vec<Range<Anchor>> {
        let suffix = self.previous_ranges.get(self.suffix_start..).unwrap_or(&[]);
        let mut match_ranges = Vec::with_capacity(self.confirmed_ranges.len() + suffix.len());
        match_ranges.extend_from_slice(&self.confirmed_ranges);
        match_ranges.extend_from_slice(suffix);
        match_ranges
    }

    fn finish(mut self, project_search: &mut ProjectSearch, cx: &mut Context<ProjectSearch>) {
        project_search.excerpts.update(cx, |excerpts, cx| {
            self.prune_stale_paths_before(None, excerpts, cx);
        });
        project_search.match_ranges = self.confirmed_ranges;
    }
}

async fn apply_reused_chunk(
    project_search: &WeakEntity<ProjectSearch>,
    buffers_with_ranges: Vec<(Entity<language::Buffer>, Vec<Range<text::Anchor>>)>,
    reused_results: &mut ReusedResults,
    cx: &mut AsyncApp,
) -> Option<()> {
    const FOREGROUND_BATCH_SIZE: usize = 64;
    let buffers_with_ranges = buffers_with_ranges
        .into_iter()
        .filter(|(_, ranges)| !ranges.is_empty())
        .collect::<Vec<_>>();
    if buffers_with_ranges.is_empty() {
        return Some(());
    }
    let (context_line_count, chunk) = project_search
        .read_with(cx, |_, cx| {
            (
                multibuffer_context_lines(cx),
                buffers_with_ranges
                    .into_iter()
                    .map(|(buffer, ranges)| {
                        let path_key = PathKey::for_buffer(&buffer, cx);
                        let buffer_snapshot = buffer.read(cx).snapshot();
                        (path_key, buffer, buffer_snapshot, ranges)
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .ok()?;
    let chunk = cx
        .background_spawn(async move {
            chunk
                .into_iter()
                .map(|(path_key, buffer, buffer_snapshot, ranges)| {
                    let point_ranges = ranges.iter().map(|range| range.to_point(&buffer_snapshot));
                    let excerpt_ranges = multi_buffer::build_excerpt_ranges(
                        point_ranges,
                        context_line_count,
                        &buffer_snapshot,
                    );
                    (path_key, buffer, buffer_snapshot, excerpt_ranges, ranges)
                })
                .collect::<Vec<_>>()
        })
        .await;
    let mut chunk = chunk.into_iter().peekable();
    let mut chunk_changed = false;
    while chunk.peek().is_some() {
        let batch = chunk
            .by_ref()
            .take(FOREGROUND_BATCH_SIZE)
            .collect::<Vec<_>>();
        let batch_changed = project_search
            .update(cx, |project_search, cx| {
                let workspace = project_search.workspace.upgrade();
                let batch_changed = project_search.excerpts.update(cx, |excerpts, cx| {
                    let mut batch_changed = false;
                    let mut applied = Vec::with_capacity(batch.len());
                    for (path_key, buffer, buffer_snapshot, excerpt_ranges, ranges) in batch {
                        let in_order = reused_results
                            .last_seen_path
                            .as_ref()
                            .is_none_or(|last| *last < path_key);
                        if in_order {
                            reused_results.last_seen_path = Some(path_key.clone());
                            batch_changed |= reused_results.prune_stale_paths_before(
                                Some(&path_key),
                                excerpts,
                                cx,
                            );
                        } else if !reused_results.reported_out_of_order {
                            reused_results.reported_out_of_order = true;
                            log::warn!(
                                "search results should arrive PathKey-sorted, one result per \
                                 buffer; updating {path_key:?} without reusing its excerpts"
                            );
                        }
                        let previous_span = if in_order {
                            reused_results.previous_ranges_for(&path_key)
                        } else {
                            None
                        };
                        if is_buffer_stale(&project_search.project, workspace.as_ref(), &buffer, cx)
                        {
                            excerpts.remove_excerpts_for_paths(vec![path_key], cx);
                            batch_changed |= previous_span.is_some();
                            continue;
                        }
                        excerpts.set_excerpt_ranges_for_path(
                            path_key,
                            buffer,
                            &buffer_snapshot,
                            excerpt_ranges,
                            cx,
                        );
                        applied.push((previous_span, ranges, in_order));
                    }
                    let snapshot = excerpts.snapshot(cx);
                    for (previous_span, ranges, in_order) in applied {
                        let anchor_ranges = ranges
                            .into_iter()
                            .filter_map(|range| snapshot.anchor_range_in_buffer(range))
                            .collect::<Vec<_>>();
                        batch_changed |= previous_span.is_none_or(|span| {
                            reused_results.previous_ranges.get(span)
                                != Some(anchor_ranges.as_slice())
                        });
                        if in_order {
                            reused_results.confirmed_ranges.extend(anchor_ranges);
                        } else if let Some(first) = anchor_ranges.first() {
                            let buffer_id = first.start.buffer_id();
                            reused_results
                                .confirmed_ranges
                                .retain(|range| range.start.buffer_id() != buffer_id);
                            let insert_at =
                                reused_results.confirmed_ranges.partition_point(|range| {
                                    range.start.cmp(&first.start, &snapshot).is_lt()
                                });
                            reused_results
                                .confirmed_ranges
                                .splice(insert_at..insert_at, anchor_ranges);
                        }
                    }
                    batch_changed
                });
                if batch_changed {
                    project_search.match_ranges = reused_results.current_match_ranges();
                    cx.notify();
                }
                batch_changed
            })
            .ok()?;
        chunk_changed |= batch_changed;
        smol::future::yield_now().await;
    }
    project_search
        .update(cx, |project_search, cx| {
            let newly_refreshed = !mem::replace(&mut project_search.results_refreshed, true);
            if chunk_changed || newly_refreshed {
                cx.notify();
            }
        })
        .ok()?;
    Some(())
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ViewEvent {
    UpdateTab,
    Activate,
    EditorEvent(editor::EditorEvent),
    Dismiss,
}

impl EventEmitter<ViewEvent> for ProjectSearchView {}

impl Render for ProjectSearchView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let mut key_context = KeyContext::default();
        key_context.add("ProjectSearchView");

        if self.has_matches() {
            div()
                .key_context(key_context)
                .on_action(cx.listener(Self::open_text_finder))
                .flex_1()
                .size_full()
                .track_focus(&self.focus_handle(cx))
                .child(self.results_editor.clone())
        } else {
            let model = self.entity.read(cx);

            let heading_text = match model.search_state {
                SearchState::Running { .. } if model.search_state.no_results_so_far() => {
                    "No Results"
                }
                SearchState::Running {
                    activity: SearchActivity::WaitingForScan,
                    ..
                } => "Loading project…",
                SearchState::Running {
                    activity: SearchActivity::Searching,
                    ..
                } => "Searching…",
                SearchState::Completed(SearchCompletion::NoResults { .. }) => "No Results",
                _ => "Search All Files",
            };

            let heading_text = div()
                .justify_center()
                .child(Label::new(heading_text).size(LabelSize::Large));

            let page_content: Option<AnyElement> = match model.search_state {
                SearchState::Idle => Some(self.landing_text_minor(cx).into_any_element()),
                _ if model.search_state.no_results_so_far() => {
                    let deferred_dirs = model.search_state.deferred_dirs();
                    let mut elements: Vec<AnyElement> = Vec::new();
                    elements.push(
                        Label::new("No results found in this project for the provided query")
                            .size(LabelSize::Small)
                            .into_any_element(),
                    );
                    if deferred_dirs > 0 {
                        elements.push(
                            Label::new(partial_index_message(deferred_dirs))
                                .size(LabelSize::Small)
                                .color(Color::Warning)
                                .into_any_element(),
                        );
                    }
                    Some(v_flex().gap_1().children(elements).into_any_element())
                }
                _ => None,
            };

            let page_content = page_content.map(|text| div().child(text));

            h_flex()
                .key_context(key_context)
                .on_action(cx.listener(Self::open_text_finder))
                .size_full()
                .items_center()
                .justify_center()
                .overflow_hidden()
                .bg(cx.theme().colors().editor_background)
                .track_focus(&self.focus_handle(cx))
                .child(
                    v_flex()
                        .id("project-search-landing-page")
                        .overflow_y_scroll()
                        .gap_1()
                        .child(heading_text)
                        .children(page_content),
                )
        }
    }
}

impl Focusable for ProjectSearchView {
    fn focus_handle(&self, _: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for ProjectSearchView {
    type Event = ViewEvent;
    fn tab_tooltip_text(&self, cx: &App) -> Option<SharedString> {
        let query_text = self.query_editor.read(cx).text(cx);

        query_text
            .is_empty()
            .not()
            .then(|| query_text.into())
            .or_else(|| Some("Project Search".into()))
    }

    fn act_as_type<'a>(
        &'a self,
        type_id: TypeId,
        self_handle: &'a Entity<Self>,
        _: &'a App,
    ) -> Option<gpui::AnyEntity> {
        if type_id == TypeId::of::<Self>() {
            Some(self_handle.clone().into())
        } else if type_id == TypeId::of::<Editor>() {
            Some(self.results_editor.clone().into())
        } else {
            None
        }
    }
    fn as_searchable(&self, _: &Entity<Self>, _: &App) -> Option<Box<dyn SearchableItemHandle>> {
        Some(Box::new(self.results_editor.clone()))
    }

    fn deactivated(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.results_editor
            .update(cx, |editor, cx| editor.deactivated(window, cx));
    }

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {
        Some(Icon::new(IconName::MagnifyingGlass))
    }

    fn tab_content_text(&self, _detail: usize, cx: &App) -> SharedString {
        let last_query: Option<SharedString> = self
            .entity
            .read(cx)
            .last_search_query_text
            .as_ref()
            .map(|query| {
                let query = query.replace('\n', "");
                let query_text = util::truncate_and_trailoff(&query, MAX_TAB_TITLE_LEN);
                query_text.into()
            });

        last_query
            .filter(|query| !query.is_empty())
            .unwrap_or_else(|| "Project Search".into())
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("Project Search Opened")
    }

    fn for_each_project_item(
        &self,
        cx: &App,
        f: &mut dyn FnMut(EntityId, &dyn project::ProjectItem),
    ) {
        self.results_editor.for_each_project_item(cx, f)
    }

    fn active_project_path(&self, cx: &App) -> Option<ProjectPath> {
        self.results_editor.read(cx).active_project_path(cx)
    }

    fn can_save(&self, _: &App) -> bool {
        true
    }

    fn is_dirty(&self, cx: &App) -> bool {
        self.results_editor.read(cx).is_dirty(cx)
    }

    fn has_conflict(&self, cx: &App) -> bool {
        self.results_editor.read(cx).has_conflict(cx)
    }

    fn save(
        &mut self,
        options: SaveOptions,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<anyhow::Result<()>> {
        self.results_editor
            .update(cx, |editor, cx| editor.save(options, project, window, cx))
    }

    fn save_as(
        &mut self,
        _: Entity<Project>,
        _: ProjectPath,
        _window: &mut Window,
        _: &mut Context<Self>,
    ) -> Task<anyhow::Result<()>> {
        unreachable!("save_as should not have been called")
    }

    fn reload(
        &mut self,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<anyhow::Result<()>> {
        self.results_editor
            .update(cx, |editor, cx| editor.reload(project, window, cx))
    }

    fn can_split(&self) -> bool {
        true
    }

    fn clone_on_split(
        &self,
        _workspace_id: Option<WorkspaceId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Option<Entity<Self>>>
    where
        Self: Sized,
    {
        let model = self.entity.update(cx, |model, cx| model.clone(cx));
        Task::ready(Some(cx.new(|cx| {
            Self::new(self.workspace.clone(), model, window, cx, None)
        })))
    }

    fn added_to_workspace(
        &mut self,
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.results_editor.update(cx, |editor, cx| {
            editor.added_to_workspace(workspace, window, cx)
        });
    }

    fn set_nav_history(
        &mut self,
        nav_history: ItemNavHistory,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.results_editor.update(cx, |editor, _| {
            editor.set_nav_history(Some(nav_history));
        });
    }

    fn navigate(
        &mut self,
        data: Arc<dyn Any + Send>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        self.results_editor
            .update(cx, |editor, cx| editor.navigate(data, window, cx))
    }

    fn to_item_events(event: &Self::Event, f: &mut dyn FnMut(ItemEvent)) {
        match event {
            ViewEvent::UpdateTab => {
                f(ItemEvent::UpdateBreadcrumbs);
                f(ItemEvent::UpdateTab);
            }
            ViewEvent::EditorEvent(editor_event) => {
                Editor::to_item_events(editor_event, f);
            }
            ViewEvent::Dismiss => f(ItemEvent::CloseItem),
            _ => {}
        }
    }
}

impl ProjectSearchView {
    pub fn get_matches(&self, cx: &App) -> Vec<Range<Anchor>> {
        self.entity.read(cx).match_ranges.clone()
    }

    fn open_text_finder(
        &mut self,
        _: &OpenTextFinder,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        TextFinder::open_from_project_search(cx.entity(), window, cx).detach();
    }

    fn toggle_filters(&mut self, cx: &mut Context<Self>) {
        self.filters_enabled = !self.filters_enabled;
        ActiveSettings::update_global(cx, |settings, cx| {
            settings.0.insert(
                self.entity.read(cx).project.downgrade(),
                self.current_settings(),
            );
        });
    }

    fn current_settings(&self) -> ProjectSearchSettings {
        ProjectSearchSettings {
            search_options: self.search_options,
            filters_enabled: self.filters_enabled,
        }
    }

    fn set_search_option_enabled(
        &mut self,
        option: SearchOptions,
        enabled: bool,
        cx: &mut Context<Self>,
    ) {
        if self.search_options.contains(option) != enabled {
            self.toggle_search_option(option, cx);
        }
    }

    fn toggle_search_option(&mut self, option: SearchOptions, cx: &mut Context<Self>) {
        self.search_options.toggle(option);
        ActiveSettings::update_global(cx, |settings, cx| {
            settings.0.insert(
                self.entity.read(cx).project.downgrade(),
                self.current_settings(),
            );
        });
        self.adjust_query_regex_language(cx);
    }

    fn toggle_opened_only(&mut self, _window: &mut Window, _cx: &mut Context<Self>) {
        self.included_opened_only = !self.included_opened_only;
    }

    pub fn replacement(&self, cx: &App) -> String {
        self.replacement_editor.read(cx).text(cx)
    }

    fn replace_next(&mut self, _: &ReplaceNext, window: &mut Window, cx: &mut Context<Self>) {
        if self.entity.read(cx).pending_search.is_some() {
            self.pending_replace_next = self.entity.read(cx).phase == SearchPhase::Confirmed;
            return;
        }
        self.pending_replace_next = false;
        if let Some(last_search_query_text) = &self.entity.read(cx).last_search_query_text
            && self.query_editor.read(cx).text(cx) != *last_search_query_text
        {
            // search query has changed, restart search and bail
            self.search(SearchMode::Manual, cx);
            return;
        }
        if self.entity.read(cx).match_ranges.is_empty() {
            return;
        }
        let Some(active_index) = self.active_match_index else {
            return;
        };

        let query = self.entity.read(cx).active_query.clone();
        if let Some(query) = query {
            self.confirm_active_search(cx);
            let query = query.with_replacement(self.replacement(cx));

            let mat = self.entity.read(cx).match_ranges.get(active_index).cloned();
            self.results_editor.update(cx, |editor, cx| {
                if let Some(mat) = mat.as_ref() {
                    editor.replace(mat, &query, SearchToken::default(), window, cx);
                }
            });
            self.select_match(Direction::Next, window, cx)
        }
    }

    fn replace_all(&mut self, _: &ReplaceAll, window: &mut Window, cx: &mut Context<Self>) {
        if self.entity.read(cx).pending_search.is_some() {
            self.pending_replace_all = self.entity.read(cx).phase == SearchPhase::Confirmed;
            return;
        }
        let query_text = self.query_editor.read(cx).text(cx);
        let query_is_stale =
            self.entity.read(cx).last_search_query_text.as_deref() != Some(query_text.as_str());
        if query_is_stale {
            self.search(SearchMode::Manual, cx);
            self.pending_replace_all = self.entity.read(cx).pending_search.is_some();
            return;
        }
        self.pending_replace_all = false;
        if self.active_match_index.is_none() {
            return;
        }
        let Some(query) = self.entity.read(cx).active_query.as_ref() else {
            return;
        };
        let query = query.clone().with_replacement(self.replacement(cx));
        self.confirm_active_search(cx);

        let match_ranges = self
            .entity
            .update(cx, |model, _| mem::take(&mut model.match_ranges));
        if match_ranges.is_empty() {
            return;
        }

        self.results_editor.update(cx, |editor, cx| {
            editor.replace_all(
                &mut match_ranges.iter(),
                &query,
                SearchToken::default(),
                window,
                cx,
            );
        });

        self.entity.update(cx, |model, _cx| {
            model.match_ranges = match_ranges;
        });
    }

    fn toggle_all_search_results(
        &mut self,
        _: &ToggleAllSearchResults,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.update_results_visibility(window, cx);
    }

    fn update_results_visibility(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let has_any_folded = self.results_editor.read(cx).has_any_buffer_folded(cx);
        self.results_editor.update(cx, |editor, cx| {
            if has_any_folded {
                editor.unfold_all(&UnfoldAll, window, cx);
            } else {
                editor.fold_all(&FoldAll, window, cx);
            }
        });
        cx.notify();
    }

    pub fn new(
        workspace: WeakEntity<Workspace>,
        entity: Entity<ProjectSearch>,
        window: &mut Window,
        cx: &mut Context<Self>,
        settings: Option<ProjectSearchSettings>,
    ) -> Self {
        let project;
        let excerpts;
        let mut replacement_text = None;
        let mut query_text = String::new();
        let mut subscriptions = Vec::new();

        // Read in settings if available
        let (mut options, filters_enabled) = if let Some(settings) = settings {
            (settings.search_options, settings.filters_enabled)
        } else {
            let search_options =
                SearchOptions::from_settings(&EditorSettings::get_global(cx).search);
            (search_options, false)
        };

        {
            let entity = entity.read(cx);
            project = entity.project.clone();
            excerpts = entity.excerpts.clone();
            if let Some(active_query) = entity.active_query.as_ref() {
                query_text = active_query.as_str().to_string();
                replacement_text = active_query.replacement().map(ToOwned::to_owned);
                options = SearchOptions::from_query(active_query);
            }
        }
        subscriptions.push(cx.observe_in(&entity, window, |this, _, window, cx| {
            this.entity_changed(window, cx)
        }));

        let query_editor = cx.new(|cx| {
            let mut editor = Editor::auto_height(1, 4, window, cx);
            editor.set_placeholder_text("Search all files…", window, cx);
            editor.set_use_autoclose(false);
            editor.set_use_selection_highlight(false);
            editor.set_text(query_text, window, cx);
            editor
        });
        // Subscribe to query_editor in order to reraise editor events for workspace item activation purposes
        subscriptions.push(cx.subscribe_in(
            &query_editor,
            window,
            |this, _, event: &EditorEvent, window, cx| {
                if let EditorEvent::Edited { .. } = event {
                    if EditorSettings::get_global(cx).use_smartcase_search {
                        let query = this.search_query_text(cx);
                        if !query.is_empty()
                            && this.search_options.contains(SearchOptions::CASE_SENSITIVE)
                                != contains_uppercase(&query)
                        {
                            this.toggle_search_option(SearchOptions::CASE_SENSITIVE, cx);
                        }
                    }

                    if EditorSettings::get_global(cx).search.search_on_type && !this.is_dirty(cx) {
                        if this.query_editor.read(cx).is_empty(cx) {
                            this.debounced_search =
                                Some(cx.spawn_in(window, async move |this, cx| {
                                    cx.background_executor()
                                        .timer(SEARCH_ON_TYPE_DEBOUNCE)
                                        .await;
                                    this.update_in(cx, |this, window, cx| {
                                        if this.query_editor.read(cx).is_empty(cx)
                                            && !this.is_dirty(cx)
                                        {
                                            this.last_search_signature = None;
                                            this.pending_replace_next = false;
                                            this.pending_replace_all = false;
                                            if this
                                                .panels_with_errors
                                                .remove(&InputPanel::Query)
                                                .is_some()
                                            {
                                                cx.notify();
                                            }
                                            this.entity.update(cx, |model, cx| model.clear(cx));
                                            this.results_editor.update(cx, |editor, cx| {
                                                editor.scroll(Point::default(), window, cx);
                                            });
                                        }
                                    })
                                    .ok();
                                }));
                        } else {
                            this.schedule_search_on_type(cx);
                        }
                    }
                }
                cx.emit(ViewEvent::EditorEvent(event.clone()))
            },
        ));
        let replacement_editor = cx.new(|cx| {
            let mut editor = Editor::auto_height(1, 4, window, cx);
            editor.set_placeholder_text(REPLACE_PLACEHOLDER, window, cx);
            if let Some(text) = replacement_text {
                editor.set_text(text, window, cx);
            }
            editor
        });
        let results_editor = cx.new(|cx| {
            let mut editor = Editor::for_multibuffer(excerpts, Some(project.clone()), window, cx);
            editor.set_searchable(false);
            editor.set_in_project_search(true);
            editor
        });
        subscriptions.push(cx.observe(&results_editor, |_, _, cx| cx.emit(ViewEvent::UpdateTab)));
        subscriptions.push(
            cx.on_focus(&results_editor.focus_handle(cx), window, |this, _, cx| {
                this.confirm_active_search(cx);
            }),
        );

        subscriptions.push(
            cx.subscribe(&results_editor, |this, _, event: &EditorEvent, cx| {
                if matches!(event, editor::EditorEvent::SelectionsChanged { .. }) {
                    this.update_match_index(cx);
                }
                // Reraise editor events for workspace item activation purposes
                cx.emit(ViewEvent::EditorEvent(event.clone()));
            }),
        );
        subscriptions.push(cx.subscribe(
            &results_editor,
            |_this, _editor, _event: &SearchEvent, cx| cx.notify(),
        ));

        let included_files_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text(INCLUDE_PLACEHOLDER, window, cx);

            editor
        });
        // Subscribe to include_files_editor in order to reraise editor events for workspace item activation purposes
        subscriptions.push(cx.subscribe(
            &included_files_editor,
            |this, _, event: &EditorEvent, cx| {
                this.schedule_search_on_type_for_filter_edit(event, cx);
                cx.emit(ViewEvent::EditorEvent(event.clone()))
            },
        ));

        let excluded_files_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text(EXCLUDE_PLACEHOLDER, window, cx);

            editor
        });
        // Subscribe to excluded_files_editor in order to reraise editor events for workspace item activation purposes
        subscriptions.push(cx.subscribe(
            &excluded_files_editor,
            |this, _, event: &EditorEvent, cx| {
                this.schedule_search_on_type_for_filter_edit(event, cx);
                cx.emit(ViewEvent::EditorEvent(event.clone()))
            },
        ));

        let focus_handle = cx.focus_handle();
        subscriptions.push(cx.on_focus(&focus_handle, window, |_, window, cx| {
            cx.on_next_frame(window, |this, window, cx| {
                if this.focus_handle.is_focused(window) {
                    if this.has_matches() {
                        this.results_editor.focus_handle(cx).focus(window, cx);
                    } else {
                        this.query_editor.focus_handle(cx).focus(window, cx);
                    }
                }
            });
        }));

        let languages = project.read(cx).languages().clone();
        cx.spawn(async move |project_search_view, cx| {
            let regex_language = languages
                .language_for_name("regex")
                .await
                .context("loading regex language")?;
            project_search_view
                .update(cx, |project_search_view, cx| {
                    project_search_view.regex_language = Some(regex_language);
                    project_search_view.adjust_query_regex_language(cx);
                })
                .ok();
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);

        // Check if Worktrees have all been previously indexed
        let mut this = ProjectSearchView {
            workspace,
            focus_handle,
            replacement_editor,
            search_id: entity.read(cx).search_id,
            entity,
            query_editor,
            results_editor,
            search_options: options,
            panels_with_errors: HashMap::default(),
            active_match_index: None,
            included_files_editor,
            excluded_files_editor,
            filters_enabled,
            replace_enabled: false,
            pending_replace_all: false,
            pending_replace_next: false,
            included_opened_only: false,
            regex_language: None,
            debounced_search: None,
            last_search_signature: None,
            _subscriptions: subscriptions,
        };

        this.entity_changed(window, cx);
        this
    }

    pub fn new_search_with_filter(
        workspace: &mut Workspace,
        filter_str: String,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        let weak_workspace = cx.entity().downgrade();

        let entity = cx
            .new(|cx| ProjectSearch::new(workspace.project().clone(), weak_workspace.clone(), cx));
        let search = cx.new(|cx| ProjectSearchView::new(weak_workspace, entity, window, cx, None));
        workspace.add_item_to_active_pane(Box::new(search.clone()), None, true, window, cx);
        search.update(cx, |search, cx| {
            search
                .included_files_editor
                .update(cx, |editor, cx| editor.set_text(filter_str, window, cx));
            search.filters_enabled = true;
            search.focus_query_editor(window, cx)
        });
    }

    /// Re-activate the most recently activated search in this pane or the most recent if it has been closed.
    /// If no search exists in the workspace, create a new one.
    pub fn deploy_search(
        workspace: &mut Workspace,
        action: &workspace::DeploySearch,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        let existing = workspace
            .active_pane()
            .read(cx)
            .items()
            .find_map(|item| item.downcast::<ProjectSearchView>());

        Self::existing_or_new_search(workspace, existing, action, window, cx);
    }

    fn search_in_new(
        workspace: &mut Workspace,
        _: &SearchInNew,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        if let Some(search_view) = workspace
            .active_item(cx)
            .and_then(|item| item.downcast::<ProjectSearchView>())
        {
            let new_query = search_view.update(cx, |search_view, cx| {
                let open_buffers = if search_view.included_opened_only {
                    Some(search_view.open_buffers(cx, workspace))
                } else {
                    None
                };
                let new_query = search_view.build_search_query(cx, open_buffers);
                if new_query.is_some()
                    && let Some(old_query) = search_view.entity.read(cx).active_query.clone()
                {
                    search_view.query_editor.update(cx, |editor, cx| {
                        editor.set_text(old_query.as_str(), window, cx);
                    });
                    search_view.search_options = SearchOptions::from_query(&old_query);
                    search_view.adjust_query_regex_language(cx);
                }
                new_query
            });
            if let Some(new_query) = new_query {
                let weak_workspace = cx.entity().downgrade();
                let entity = cx.new(|cx| {
                    let mut entity =
                        ProjectSearch::new(workspace.project().clone(), weak_workspace.clone(), cx);
                    entity.search(new_query, SearchMode::Manual, false, cx);
                    entity
                });
                workspace.add_item_to_active_pane(
                    Box::new(cx.new(|cx| {
                        ProjectSearchView::new(weak_workspace, entity, window, cx, None)
                    })),
                    None,
                    true,
                    window,
                    cx,
                );
            }
        }
    }

    // Add another search tab to the workspace.
    fn new_search(
        workspace: &mut Workspace,
        _: &workspace::NewSearch,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        Self::existing_or_new_search(workspace, None, &DeploySearch::default(), window, cx)
    }

    fn existing_or_new_search(
        workspace: &mut Workspace,
        existing: Option<Entity<ProjectSearchView>>,
        action: &workspace::DeploySearch,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        enum QuerySeed {
            /// Content of the buffer search bar: already query syntax, with
            /// escaping already applied if it was seeded in regex mode, so it
            /// must never be re-escaped. It's carried over verbatim even if
            /// the buffer search's mode differs from the project search's.
            Query(String),
            /// Raw text from the editor's selection or the word under the
            /// cursor, so it gets escaped when entering a regex query.
            Text(String),
        }

        let query_seed = workspace.active_item(cx).and_then(|item| {
            if let Some(buffer_search_query) = buffer_search_query(workspace, item.as_ref(), cx) {
                return Some(QuerySeed::Query(buffer_search_query));
            }

            let editor = item.act_as::<Editor>(cx)?;
            let query = editor.query_suggestion(None, window, cx);
            if query.is_empty() {
                None
            } else {
                Some(QuerySeed::Text(query))
            }
        });

        let search = if let Some(existing) = existing {
            workspace.activate_item(&existing, true, true, window, cx);
            existing
        } else {
            let settings = cx
                .global::<ActiveSettings>()
                .0
                .get(&workspace.project().downgrade());

            let settings = settings.cloned();

            let weak_workspace = cx.entity().downgrade();

            let project_search = cx.new(|cx| {
                ProjectSearch::new(workspace.project().clone(), weak_workspace.clone(), cx)
            });
            let project_search_view = cx.new(|cx| {
                ProjectSearchView::new(weak_workspace, project_search, window, cx, settings)
            });

            workspace.add_item_to_active_pane(
                Box::new(project_search_view.clone()),
                None,
                true,
                window,
                cx,
            );
            project_search_view
        };

        search.update(cx, |search, cx| {
            search.replace_enabled |= action.replace_enabled;
            if let Some(regex) = action.regex {
                search.set_search_option_enabled(SearchOptions::REGEX, regex, cx);
            }
            if let Some(case_sensitive) = action.case_sensitive {
                search.set_search_option_enabled(SearchOptions::CASE_SENSITIVE, case_sensitive, cx);
            }
            if let Some(whole_word) = action.whole_word {
                search.set_search_option_enabled(SearchOptions::WHOLE_WORD, whole_word, cx);
            }
            if let Some(include_ignored) = action.include_ignored {
                search.set_search_option_enabled(
                    SearchOptions::INCLUDE_IGNORED,
                    include_ignored,
                    cx,
                );
            }
            if let Some(query) = action.query.as_deref().filter(|query| !query.is_empty()) {
                search.set_query(query, window, cx);
            } else if let Some(query_seed) = query_seed {
                let query = match query_seed {
                    QuerySeed::Query(query) => query,
                    QuerySeed::Text(text)
                        if search.search_options.contains(SearchOptions::REGEX) =>
                    {
                        regex::escape(&text)
                    }
                    QuerySeed::Text(text) => text,
                };
                search.set_query(&query, window, cx);
            }
            if let Some(included_files) = action.included_files.as_deref() {
                search
                    .included_files_editor
                    .update(cx, |editor, cx| editor.set_text(included_files, window, cx));
                search.filters_enabled = true;
            }
            if let Some(excluded_files) = action.excluded_files.as_deref() {
                search
                    .excluded_files_editor
                    .update(cx, |editor, cx| editor.set_text(excluded_files, window, cx));
                search.filters_enabled = true;
            }
            search.focus_query_editor(window, cx)
        });
    }

    fn prompt_to_save_if_dirty_then_search(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<anyhow::Result<()>> {
        let project = self.entity.read(cx).project.clone();

        let can_autosave = self.results_editor.can_autosave(cx);
        let autosave_setting = self.results_editor.workspace_settings(cx).autosave;

        let will_autosave = can_autosave && autosave_setting.should_save_on_close();

        let is_dirty = self.is_dirty(cx);

        cx.spawn_in(window, async move |this, cx| {
            let skip_save_on_close = this
                .read_with(cx, |this, cx| {
                    this.workspace.read_with(cx, |workspace, cx| {
                        workspace::Pane::skip_save_on_close(&this.results_editor, workspace, cx)
                    })
                })?
                .unwrap_or(false);

            let should_prompt_to_save = !skip_save_on_close && !will_autosave && is_dirty;

            let should_search = if should_prompt_to_save {
                let options = &["Save", "Don't Save", "Cancel"];
                let result_channel = this.update_in(cx, |_, window, cx| {
                    window.prompt(
                        gpui::PromptLevel::Warning,
                        "Project search buffer contains unsaved edits. Do you want to save it?",
                        None,
                        options,
                        cx,
                    )
                })?;
                let result = result_channel.await?;
                let should_save = result == 0;
                if should_save {
                    this.update_in(cx, |this, window, cx| {
                        this.save(
                            SaveOptions {
                                format: true,
                                force_format: false,
                                autosave: false,
                            },
                            project,
                            window,
                            cx,
                        )
                    })?
                    .await
                    .log_err();
                }

                result != 2
            } else {
                true
            };
            if should_search {
                this.update(cx, |this, cx| {
                    this.search(SearchMode::Manual, cx);
                })?;
            }
            anyhow::Ok(())
        })
    }

    fn search(&mut self, mode: SearchMode, cx: &mut Context<Self>) {
        let open_buffers = self.open_buffers_for_search(cx);
        let signature = self.search_signature_for(open_buffers.as_deref(), cx);
        self.search_with_signature(mode, signature, open_buffers, cx);
    }

    fn search_with_signature(
        &mut self,
        mode: SearchMode,
        signature: SearchSignature,
        open_buffers: Option<Vec<Entity<Buffer>>>,
        cx: &mut Context<Self>,
    ) {
        if let Some(query) = self.build_search_query(cx, open_buffers) {
            self.debounced_search = None;
            let same_inputs = self.last_search_signature.as_ref() == Some(&signature);
            if !same_inputs {
                self.pending_replace_next = false;
                self.pending_replace_all = false;
            }
            self.last_search_signature = Some(signature);
            self.entity
                .update(cx, |model, cx| model.search(query, mode, same_inputs, cx));
        }
    }

    fn confirm_active_search(&mut self, cx: &mut Context<Self>) {
        let model = self.entity.read(cx);
        if model.phase == SearchPhase::Typing {
            self.search_id = model.search_id;
            self.entity.update(cx, |model, cx| {
                model.phase = SearchPhase::Confirmed;
                model.record_search_history(cx);
            });
            self.sync_search_results_status(cx);
        }
    }

    fn sync_search_results_status(&self, cx: &mut Context<Self>) {
        let model = self.entity.read(cx);
        let pending = model.pending_search.is_some();
        let status = SearchResultsStatus {
            pending,
            results_stale: pending && !model.results_refreshed,
            query_confirmed: model.phase == SearchPhase::Confirmed,
        };
        self.results_editor.update(cx, |editor, cx| {
            editor.set_search_results_status(status, cx)
        });
    }

    /// The signature captures search inputs, not project content; returning to a previously
    /// searched query shows the old results until the user confirms with Enter, which always
    /// forces a refresh.
    fn schedule_search_on_type(&mut self, cx: &mut Context<Self>) {
        if self.is_dirty(cx) {
            return;
        }
        self.debounced_search = Some(cx.spawn(async move |this, cx| {
            cx.background_executor()
                .timer(SEARCH_ON_TYPE_DEBOUNCE)
                .await;
            this.update(cx, |this, cx| {
                if this.is_dirty(cx) {
                    return;
                }
                let open_buffers = this.open_buffers_for_search(cx);
                let signature = this.search_signature_for(open_buffers.as_deref(), cx);
                if this.last_search_signature.as_ref() != Some(&signature) {
                    this.search_with_signature(SearchMode::OnType, signature, open_buffers, cx);
                }
            })
            .ok();
        }));
    }

    fn search_signature(&self, cx: &App) -> SearchSignature {
        let open_buffers = self.open_buffers_for_search(cx);
        self.search_signature_for(open_buffers.as_deref(), cx)
    }

    fn open_buffers_for_search(&self, cx: &App) -> Option<Vec<Entity<Buffer>>> {
        self.included_opened_only.then(|| {
            self.workspace
                .read_with(cx, |workspace, cx| self.open_buffers(cx, workspace))
                .unwrap_or_default()
        })
    }

    fn search_signature_for(
        &self,
        open_buffers: Option<&[Entity<Buffer>]>,
        cx: &App,
    ) -> SearchSignature {
        SearchSignature {
            query: self.search_query_text(cx),
            included_files: self
                .filters_enabled
                .then(|| self.included_files_editor.read(cx).text(cx))
                .unwrap_or_default(),
            excluded_files: self
                .filters_enabled
                .then(|| self.excluded_files_editor.read(cx).text(cx))
                .unwrap_or_default(),
            options: self.search_options,
            filters_enabled: self.filters_enabled,
            included_opened_only: self.included_opened_only,
            opened_buffers: self.included_opened_only.then(|| {
                let mut buffer_ids = open_buffers
                    .unwrap_or_default()
                    .iter()
                    .map(|buffer| buffer.entity_id())
                    .collect::<Vec<_>>();
                buffer_ids.sort_unstable();
                buffer_ids.dedup();
                buffer_ids
            }),
        }
    }

    fn schedule_search_on_type_for_filter_edit(
        &mut self,
        event: &EditorEvent,
        cx: &mut Context<Self>,
    ) {
        if matches!(event, EditorEvent::Edited { .. })
            && EditorSettings::get_global(cx).search.search_on_type
            && self.filters_enabled
            && !self.query_editor.read(cx).is_empty(cx)
        {
            self.schedule_search_on_type(cx);
        }
    }

    pub fn search_query_text(&self, cx: &App) -> String {
        self.query_editor.read(cx).text(cx)
    }

    fn build_search_query(
        &mut self,
        cx: &mut Context<Self>,
        open_buffers: Option<Vec<Entity<Buffer>>>,
    ) -> Option<SearchQuery> {
        // Do not bail early in this function, as we want to fill out `self.panels_with_errors`.

        let text = self.search_query_text(cx);
        let included_files = self
            .filters_enabled
            .then(|| {
                match self.parse_path_matches(self.included_files_editor.read(cx).text(cx), cx) {
                    Ok(included_files) => {
                        let should_unmark_error =
                            self.panels_with_errors.remove(&InputPanel::Include);
                        if should_unmark_error.is_some() {
                            cx.notify();
                        }
                        included_files
                    }
                    Err(e) => {
                        let should_mark_error = self
                            .panels_with_errors
                            .insert(InputPanel::Include, e.to_string());
                        if should_mark_error.is_none() {
                            cx.notify();
                        }
                        PathMatcher::default()
                    }
                }
            })
            .unwrap_or(PathMatcher::default());
        let excluded_files = self
            .filters_enabled
            .then(|| {
                match self.parse_path_matches(self.excluded_files_editor.read(cx).text(cx), cx) {
                    Ok(excluded_files) => {
                        let should_unmark_error =
                            self.panels_with_errors.remove(&InputPanel::Exclude);
                        if should_unmark_error.is_some() {
                            cx.notify();
                        }

                        excluded_files
                    }
                    Err(e) => {
                        let should_mark_error = self
                            .panels_with_errors
                            .insert(InputPanel::Exclude, e.to_string());
                        if should_mark_error.is_none() {
                            cx.notify();
                        }
                        PathMatcher::default()
                    }
                }
            })
            .unwrap_or(PathMatcher::default());

        // If the project contains multiple visible worktrees, we match the
        // include/exclude patterns against full paths to allow them to be
        // disambiguated. For single worktree projects we use worktree relative
        // paths for convenience.
        let match_full_paths = self
            .entity
            .read(cx)
            .project
            .read(cx)
            .visible_worktrees(cx)
            .count()
            > 1;

        let query = match self.search_options.build_query(
            text,
            included_files,
            excluded_files,
            match_full_paths,
            open_buffers,
        ) {
            Ok(query) => {
                let should_unmark_error = self.panels_with_errors.remove(&InputPanel::Query);
                if should_unmark_error.is_some() {
                    cx.notify();
                }

                Some(query)
            }
            Err(e) => {
                let should_mark_error = self
                    .panels_with_errors
                    .insert(InputPanel::Query, e.to_string());
                if should_mark_error.is_none() {
                    cx.notify();
                }

                None
            }
        };
        if !self.panels_with_errors.is_empty() {
            return None;
        }
        if query.as_ref().is_some_and(|query| query.is_empty()) {
            return None;
        }
        query
    }

    fn open_buffers(&self, cx: &App, workspace: &Workspace) -> Vec<Entity<Buffer>> {
        let mut buffers = Vec::new();
        for editor in workspace.items_of_type::<Editor>(cx) {
            if let Some(buffer) = editor.read(cx).buffer().read(cx).as_singleton() {
                buffers.push(buffer);
            }
        }
        buffers
    }

    /// The include/exclude path matchers currently configured on this view,
    /// honoring `filters_enabled`. Read-only (unlike `build_search_query` it does
    /// not record parse errors in `panels_with_errors`); invalid globs fall back
    /// to a default (match-all) matcher. Shared with the text finder, which is
    /// backed by the same view.
    pub(crate) fn file_path_filters(&self, cx: &App) -> (PathMatcher, PathMatcher) {
        if !self.filters_enabled {
            return (PathMatcher::default(), PathMatcher::default());
        }
        let included = self
            .parse_path_matches(self.included_files_editor.read(cx).text(cx), cx)
            .unwrap_or_default();
        let excluded = self
            .parse_path_matches(self.excluded_files_editor.read(cx).text(cx), cx)
            .unwrap_or_default();
        (included, excluded)
    }

    fn parse_path_matches(&self, text: String, cx: &App) -> anyhow::Result<PathMatcher> {
        let path_style = self.entity.read(cx).project.read(cx).path_style(cx);
        let queries = split_glob_patterns(&text)
            .into_iter()
            .map(str::trim)
            .filter(|maybe_glob_str| !maybe_glob_str.is_empty())
            .map(str::to_owned)
            .collect::<Vec<_>>();
        Ok(PathMatcher::new(&queries, path_style)?)
    }

    fn select_match(&mut self, direction: Direction, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(index) = self.active_match_index {
            let match_ranges = self.entity.read(cx).match_ranges.clone();
            if match_ranges.is_empty() {
                return;
            }

            if !EditorSettings::get_global(cx).search_wrap
                && ((direction == Direction::Next && index + 1 >= match_ranges.len())
                    || (direction == Direction::Prev && index == 0))
            {
                crate::show_no_more_matches(window, cx);
                return;
            }

            let new_index = self.results_editor.update(cx, |editor, cx| {
                editor.match_index_for_direction(
                    &match_ranges,
                    index,
                    direction,
                    1,
                    SearchToken::default(),
                    window,
                    cx,
                )
            });

            let range_to_select = match_ranges[new_index].clone();
            self.results_editor.update(cx, |editor, cx| {
                let range_to_select = editor.range_for_match(&range_to_select);
                let autoscroll = if EditorSettings::get_global(cx).search.center_on_match {
                    Autoscroll::center()
                } else {
                    Autoscroll::fit()
                };
                editor.unfold_ranges(std::slice::from_ref(&range_to_select), false, true, cx);
                editor.change_selections(SelectionEffects::scroll(autoscroll), window, cx, |s| {
                    s.select_ranges([range_to_select])
                });
            });
            self.highlight_matches(&match_ranges, Some(new_index), cx);
        }
    }

    fn focus_query_editor(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.query_editor.update(cx, |query_editor, cx| {
            query_editor.select_all(&SelectAll, window, cx);
        });
        let editor_handle = self.query_editor.focus_handle(cx);
        window.focus(&editor_handle, cx);
    }

    /// Apply some state (from the textfinder) to the project search UI
    pub(crate) fn adopt_text_finder_state(
        &mut self,
        search_options: SearchOptions,
        active_query: Option<SearchQuery>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.search_options = search_options;
        self.adjust_query_regex_language(cx);
        if let Some(query) = active_query {
            let query_text = query.as_str().to_string();
            self.entity.update(cx, |search, _| {
                search.active_query = Some(query.clone());
                search.last_search_query_text = Some(query_text.clone());
                search.phase = SearchPhase::Confirmed;
                // Force `entity_changed` to treat this as a new search so the
                // first match gets selected and scrolled into view. The text
                // finder ran its searches via `project.search` directly, so the
                // entity's `search_id` was never advanced.
                search.search_id += 1;
            });
            self.set_search_editor(SearchInputKind::Query, &query_text, window, cx);
            self.last_search_signature = Some(self.search_signature(cx));
            self.focus_results_editor(window, cx);
        } else {
            self.focus_query_editor(window, cx);
        }
        self.entity_changed(window, cx);
    }

    fn set_query(&mut self, query: &str, window: &mut Window, cx: &mut Context<Self>) {
        self.set_search_editor(SearchInputKind::Query, query, window, cx);
        if EditorSettings::get_global(cx).use_smartcase_search
            && !query.is_empty()
            && self.search_options.contains(SearchOptions::CASE_SENSITIVE)
                != contains_uppercase(query)
        {
            self.toggle_search_option(SearchOptions::CASE_SENSITIVE, cx)
        }
    }

    fn set_search_editor(
        &mut self,
        kind: SearchInputKind,
        text: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let editor = match kind {
            SearchInputKind::Query => &self.query_editor,
            SearchInputKind::Include => &self.included_files_editor,

            SearchInputKind::Exclude => &self.excluded_files_editor,
        };
        editor.update(cx, |editor, cx| {
            editor.set_text(text, window, cx);
            editor.request_autoscroll(Autoscroll::fit(), cx);
        });
    }

    fn focus_results_editor(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.query_editor.update(cx, |query_editor, cx| {
            let cursor = query_editor.selections.newest_anchor().head();
            query_editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                s.select_ranges([cursor..cursor])
            });
        });
        let results_handle = self.results_editor.focus_handle(cx);
        window.focus(&results_handle, cx);
    }

    fn entity_changed(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let model = self.entity.read(cx);
        let phase = model.phase;
        let concluded_no_results = model.search_state.no_results_so_far();
        let search_pending = model.pending_search.is_some();
        let results_stale = search_pending && !model.results_refreshed;
        let preserve_view = phase == SearchPhase::Typing || results_stale;
        let match_ranges = model.match_ranges.clone();
        self.sync_search_results_status(cx);

        if phase == SearchPhase::Typing
            && let Some(first_match) = match_ranges.first()
        {
            self.results_editor.update(cx, |editor, cx| {
                let range_to_select = editor.range_for_match(first_match);
                if editor.selections.newest_anchor().range() != range_to_select {
                    editor.change_selections(
                        SelectionEffects::no_scroll().nav_history(false),
                        window,
                        cx,
                        |s| s.select_ranges([range_to_select]),
                    );
                }
            });
        }

        if match_ranges.is_empty() {
            self.active_match_index = None;
            if !results_stale {
                self.results_editor.update(cx, |editor, cx| {
                    editor.clear_background_highlights(HighlightKey::ProjectSearchView, cx);
                    if concluded_no_results {
                        editor.scroll(Point::default(), window, cx);
                    }
                });
            }
        } else {
            self.active_match_index = Some(0);
            self.update_match_index(cx);
            // While typing, do not advance `search_id` either: the old results are still on screen,
            // so a premature bump would make the eventual confirmed swap look like a stale search
            // and skip selecting/scrolling to the first match.
            if !preserve_view {
                let prev_search_id =
                    mem::replace(&mut self.search_id, self.entity.read(cx).search_id);
                let is_new_search = self.search_id != prev_search_id;
                if is_new_search {
                    self.results_editor.update(cx, |editor, cx| {
                        let range_to_select = match_ranges
                            .first()
                            .map(|range| editor.range_for_match(range));
                        editor.change_selections(Default::default(), window, cx, |s| {
                            s.select_ranges(range_to_select)
                        });
                        editor.scroll(Point::default(), window, cx);
                    });
                    if phase == SearchPhase::Confirmed
                        && self.query_editor.focus_handle(cx).is_focused(window)
                    {
                        self.focus_results_editor(window, cx);
                    }
                }
            }
        }

        cx.emit(ViewEvent::UpdateTab);
        cx.notify();

        if self.entity.read(cx).pending_search.is_none() {
            if self.pending_replace_all {
                self.replace_all(&ReplaceAll, window, cx);
            } else if self.pending_replace_next {
                self.replace_next(&ReplaceNext, window, cx);
            }
        }
    }

    fn update_match_index(&mut self, cx: &mut Context<Self>) {
        let results_editor = self.results_editor.read(cx);
        let newest_anchor = results_editor.selections.newest_anchor().head();
        let buffer_snapshot = results_editor.buffer().read(cx).snapshot(cx);
        let new_index = self.entity.update(cx, |this, cx| {
            let new_index = active_match_index(
                Direction::Next,
                &this.match_ranges,
                &newest_anchor,
                &buffer_snapshot,
            );

            self.highlight_matches(&this.match_ranges, new_index, cx);
            new_index
        });

        if self.active_match_index != new_index {
            self.active_match_index = new_index;
            cx.notify();
        }
    }

    #[ztracing::instrument(skip_all)]
    fn highlight_matches(
        &self,
        match_ranges: &[Range<Anchor>],
        active_index: Option<usize>,
        cx: &mut App,
    ) {
        self.results_editor.update(cx, |editor, cx| {
            editor.highlight_background(
                HighlightKey::ProjectSearchView,
                match_ranges,
                move |index, theme| {
                    if active_index == Some(*index) {
                        theme.colors().search_active_match_background
                    } else {
                        theme.colors().search_match_background
                    }
                },
                cx,
            );
        });
    }

    pub fn has_matches(&self) -> bool {
        self.active_match_index.is_some()
    }

    fn landing_text_minor(&self, cx: &App) -> impl IntoElement {
        let focus_handle = self.focus_handle.clone();
        v_flex()
            .gap_1()
            .child(
                Label::new(if EditorSettings::get_global(cx).search.search_on_type {
                    "Start typing to search. For more options:"
                } else {
                    "Hit enter to search. For more options:"
                })
                .color(Color::Muted)
                .mb_2(),
            )
            .child(
                Button::new("filter-paths", "Include/exclude specific paths")
                    .start_icon(Icon::new(IconName::Filter).size(IconSize::Small))
                    .key_binding(KeyBinding::for_action_in(&ToggleFilters, &focus_handle, cx))
                    .on_click(|_event, window, cx| {
                        window.dispatch_action(ToggleFilters.boxed_clone(), cx)
                    }),
            )
            .child(
                Button::new("find-replace", "Find and replace")
                    .start_icon(Icon::new(IconName::Replace).size(IconSize::Small))
                    .key_binding(KeyBinding::for_action_in(&ToggleReplace, &focus_handle, cx))
                    .on_click(|_event, window, cx| {
                        window.dispatch_action(ToggleReplace.boxed_clone(), cx)
                    }),
            )
            .child(
                Button::new("regex", "Match with regex")
                    .start_icon(Icon::new(IconName::Regex).size(IconSize::Small))
                    .key_binding(KeyBinding::for_action_in(&ToggleRegex, &focus_handle, cx))
                    .on_click(|_event, window, cx| {
                        window.dispatch_action(ToggleRegex.boxed_clone(), cx)
                    }),
            )
            .child(
                Button::new("match-case", "Match case")
                    .start_icon(Icon::new(IconName::CaseSensitive).size(IconSize::Small))
                    .key_binding(KeyBinding::for_action_in(
                        &ToggleCaseSensitive,
                        &focus_handle,
                        cx,
                    ))
                    .on_click(|_event, window, cx| {
                        window.dispatch_action(ToggleCaseSensitive.boxed_clone(), cx)
                    }),
            )
            .child(
                Button::new("match-whole-words", "Match whole words")
                    .start_icon(Icon::new(IconName::WholeWord).size(IconSize::Small))
                    .key_binding(KeyBinding::for_action_in(
                        &ToggleWholeWord,
                        &focus_handle,
                        cx,
                    ))
                    .on_click(|_event, window, cx| {
                        window.dispatch_action(ToggleWholeWord.boxed_clone(), cx)
                    }),
            )
    }

    fn border_color_for(&self, panel: InputPanel, cx: &App) -> Hsla {
        if self.panels_with_errors.contains_key(&panel) {
            Color::Error.color(cx)
        } else {
            cx.theme().colors().border
        }
    }

    fn move_focus_to_results(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.results_editor.focus_handle(cx).is_focused(window)
            && !self.entity.read(cx).match_ranges.is_empty()
        {
            cx.stop_propagation();
            self.focus_results_editor(window, cx)
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn results_editor(&self) -> &Entity<Editor> {
        &self.results_editor
    }

    fn adjust_query_regex_language(&self, cx: &mut App) {
        let enable = self.search_options.contains(SearchOptions::REGEX);
        let query_buffer = self
            .query_editor
            .read(cx)
            .buffer()
            .read(cx)
            .as_singleton()
            .expect("query editor should be backed by a singleton buffer");
        if enable {
            if let Some(regex_language) = self.regex_language.clone() {
                query_buffer.update(cx, |query_buffer, cx| {
                    query_buffer.set_language(Some(regex_language), cx);
                })
            }
        } else {
            query_buffer.update(cx, |query_buffer, cx| {
                query_buffer.set_language(None, cx);
            })
        }
    }
}

pub(crate) fn buffer_search_query(
    workspace: &mut Workspace,
    item: &dyn ItemHandle,
    cx: &mut Context<Workspace>,
) -> Option<String> {
    let buffer_search_bar = workspace
        .pane_for(item)
        .and_then(|pane| {
            pane.read(cx)
                .toolbar()
                .read(cx)
                .item_of_type::<BufferSearchBar>()
        })?
        .read(cx);
    if buffer_search_bar.query_editor_focused() {
        let buffer_search_query = buffer_search_bar.query(cx);
        if !buffer_search_query.is_empty() {
            return Some(buffer_search_query);
        }
    }
    None
}

impl Default for ProjectSearchBar {
    fn default() -> Self {
        Self::new()
    }
}

impl ProjectSearchBar {
    pub fn new() -> Self {
        Self {
            active_project_search: None,
            subscription: None,
        }
    }

    fn confirm(&mut self, _: &Confirm, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(search_view) = self.active_project_search.as_ref() {
            search_view.update(cx, |search_view, cx| {
                if search_view
                    .replacement_editor
                    .focus_handle(cx)
                    .is_focused(window)
                {
                    return;
                }

                cx.stop_propagation();
                if EditorSettings::get_global(cx).search.search_on_type {
                    if search_view.query_editor.read(cx).is_empty(cx) {
                        return;
                    }
                    search_view.debounced_search = None;
                    if search_view.is_dirty(cx) {
                        search_view
                            .prompt_to_save_if_dirty_then_search(window, cx)
                            .detach_and_log_err(cx);
                    } else {
                        search_view.search(SearchMode::Manual, cx);
                    }
                } else {
                    search_view
                        .prompt_to_save_if_dirty_then_search(window, cx)
                        .detach_and_log_err(cx);
                }
            });
        }
    }

    fn tab(&mut self, _: &Tab, window: &mut Window, cx: &mut Context<Self>) {
        self.cycle_field(Direction::Next, window, cx);
    }

    fn backtab(&mut self, _: &Backtab, window: &mut Window, cx: &mut Context<Self>) {
        self.cycle_field(Direction::Prev, window, cx);
    }

    fn focus_search(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(search_view) = self.active_project_search.as_ref() {
            search_view.update(cx, |search_view, cx| {
                search_view.focus_query_editor(window, cx);
            });
        }
    }

    fn cycle_field(&mut self, direction: Direction, window: &mut Window, cx: &mut Context<Self>) {
        let active_project_search = match &self.active_project_search {
            Some(active_project_search) => active_project_search,
            None => return,
        };

        active_project_search.update(cx, |project_view, cx| {
            let mut views = vec![project_view.query_editor.focus_handle(cx)];
            if project_view.replace_enabled {
                views.push(project_view.replacement_editor.focus_handle(cx));
            }
            if project_view.filters_enabled {
                views.extend([
                    project_view.included_files_editor.focus_handle(cx),
                    project_view.excluded_files_editor.focus_handle(cx),
                ]);
            }
            let current_index = match views.iter().position(|focus| focus.is_focused(window)) {
                Some(index) => index,
                None => return,
            };

            let new_index = match direction {
                Direction::Next => (current_index + 1) % views.len(),
                Direction::Prev if current_index == 0 => views.len() - 1,
                Direction::Prev => (current_index - 1) % views.len(),
            };
            let next_focus_handle = &views[new_index];
            window.focus(next_focus_handle, cx);
            cx.stop_propagation();
        });
    }

    pub(crate) fn toggle_search_option(
        &mut self,
        option: SearchOptions,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        if self.active_project_search.is_none() {
            return false;
        }

        cx.spawn_in(window, async move |this, cx| {
            let task = this.update_in(cx, |this, window, cx| {
                let search_view = this.active_project_search.as_ref()?;
                search_view.update(cx, |search_view, cx| {
                    search_view.toggle_search_option(option, cx);
                    if search_view.entity.read(cx).active_query.is_none() {
                        return None;
                    }
                    if EditorSettings::get_global(cx).search.search_on_type
                        && !search_view.is_dirty(cx)
                    {
                        search_view.search(SearchMode::Refresh, cx);
                        None
                    } else {
                        Some(search_view.prompt_to_save_if_dirty_then_search(window, cx))
                    }
                })
            })?;
            if let Some(task) = task {
                task.await?;
            }
            this.update(cx, |_, cx| {
                cx.notify();
            })?;
            anyhow::Ok(())
        })
        .detach();
        true
    }

    fn toggle_replace(&mut self, _: &ToggleReplace, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(search) = &self.active_project_search {
            search.update(cx, |this, cx| {
                this.replace_enabled = !this.replace_enabled;
                let editor_to_focus = if this.replace_enabled {
                    this.replacement_editor.focus_handle(cx)
                } else {
                    this.query_editor.focus_handle(cx)
                };
                window.focus(&editor_to_focus, cx);
                cx.notify();
            });
        }
    }

    fn toggle_filters(&mut self, window: &mut Window, cx: &mut Context<Self>) -> bool {
        if let Some(search_view) = self.active_project_search.as_ref() {
            search_view.update(cx, |search_view, cx| {
                search_view.toggle_filters(cx);
                search_view
                    .included_files_editor
                    .update(cx, |_, cx| cx.notify());
                search_view
                    .excluded_files_editor
                    .update(cx, |_, cx| cx.notify());
                if EditorSettings::get_global(cx).search.search_on_type
                    && search_view.entity.read(cx).active_query.is_some()
                    && !search_view.is_dirty(cx)
                {
                    search_view.search(SearchMode::Refresh, cx);
                }
                window.refresh();
                cx.notify();
            });
            cx.notify();
            true
        } else {
            false
        }
    }

    fn toggle_opened_only(&mut self, window: &mut Window, cx: &mut Context<Self>) -> bool {
        if self.active_project_search.is_none() {
            return false;
        }

        cx.spawn_in(window, async move |this, cx| {
            let task = this.update_in(cx, |this, window, cx| {
                let search_view = this.active_project_search.as_ref()?;
                search_view.update(cx, |search_view, cx| {
                    search_view.toggle_opened_only(window, cx);
                    if search_view.entity.read(cx).active_query.is_none() {
                        return None;
                    }
                    if EditorSettings::get_global(cx).search.search_on_type
                        && !search_view.is_dirty(cx)
                    {
                        search_view.search(SearchMode::Refresh, cx);
                        None
                    } else {
                        Some(search_view.prompt_to_save_if_dirty_then_search(window, cx))
                    }
                })
            })?;
            if let Some(task) = task {
                task.await?;
            }
            this.update(cx, |_, cx| {
                cx.notify();
            })?;
            anyhow::Ok(())
        })
        .detach();
        true
    }

    fn is_opened_only_enabled(&self, cx: &App) -> bool {
        if let Some(search_view) = self.active_project_search.as_ref() {
            search_view.read(cx).included_opened_only
        } else {
            false
        }
    }

    fn move_focus_to_results(&self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(search_view) = self.active_project_search.as_ref() {
            search_view.update(cx, |search_view, cx| {
                search_view.move_focus_to_results(window, cx);
            });
            cx.notify();
        }
    }

    fn next_history_query(
        &mut self,
        _: &NextHistoryQuery,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(search_view) = self.active_project_search.as_ref() {
            search_view.update(cx, |search_view, cx| {
                for (editor, kind) in [
                    (search_view.query_editor.clone(), SearchInputKind::Query),
                    (
                        search_view.included_files_editor.clone(),
                        SearchInputKind::Include,
                    ),
                    (
                        search_view.excluded_files_editor.clone(),
                        SearchInputKind::Exclude,
                    ),
                ] {
                    if editor.focus_handle(cx).is_focused(window) {
                        if !should_navigate_history(&editor, HistoryNavigationDirection::Next, cx) {
                            cx.propagate();
                            return;
                        }

                        let new_query = search_view.entity.update(cx, |model, cx| {
                            let project = model.project.clone();

                            if let Some(new_query) = project.update(cx, |project, _| {
                                project
                                    .search_history_mut(kind)
                                    .next(model.cursor_mut(kind))
                                    .map(str::to_string)
                            }) {
                                Some(new_query)
                            } else {
                                model.cursor_mut(kind).take_draft()
                            }
                        });
                        if let Some(new_query) = new_query {
                            search_view.set_search_editor(kind, &new_query, window, cx);
                        }
                    }
                }
            });
        }
    }

    fn previous_history_query(
        &mut self,
        _: &PreviousHistoryQuery,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(search_view) = self.active_project_search.as_ref() {
            search_view.update(cx, |search_view, cx| {
                for (editor, kind) in [
                    (search_view.query_editor.clone(), SearchInputKind::Query),
                    (
                        search_view.included_files_editor.clone(),
                        SearchInputKind::Include,
                    ),
                    (
                        search_view.excluded_files_editor.clone(),
                        SearchInputKind::Exclude,
                    ),
                ] {
                    if editor.focus_handle(cx).is_focused(window) {
                        if !should_navigate_history(
                            &editor,
                            HistoryNavigationDirection::Previous,
                            cx,
                        ) {
                            cx.propagate();
                            return;
                        }

                        if editor.read(cx).text(cx).is_empty()
                            && let Some(new_query) = search_view
                                .entity
                                .read(cx)
                                .project
                                .read(cx)
                                .search_history(kind)
                                .current(search_view.entity.read(cx).cursor(kind))
                                .map(str::to_string)
                        {
                            search_view.set_search_editor(kind, &new_query, window, cx);
                            return;
                        }

                        let current_query = editor.read(cx).text(cx);
                        if let Some(new_query) = search_view.entity.update(cx, |model, cx| {
                            let project = model.project.clone();
                            project.update(cx, |project, _| {
                                project
                                    .search_history_mut(kind)
                                    .previous(model.cursor_mut(kind), &current_query)
                                    .map(str::to_string)
                            })
                        }) {
                            search_view.set_search_editor(kind, &new_query, window, cx);
                        }
                    }
                }
            });
        }
    }

    fn select_next_match(
        &mut self,
        _: &SelectNextMatch,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(search) = self.active_project_search.as_ref() {
            search.update(cx, |this, cx| {
                this.select_match(Direction::Next, window, cx);
            })
        }
    }

    fn select_prev_match(
        &mut self,
        _: &SelectPreviousMatch,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(search) = self.active_project_search.as_ref() {
            search.update(cx, |this, cx| {
                this.select_match(Direction::Prev, window, cx);
            })
        }
    }

    fn open_text_finder(
        &mut self,
        _: &OpenTextFinder,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(search) = &self.active_project_search else {
            tracing::warn!("active_project_search was none");
            return;
        };

        TextFinder::open_from_project_search(Entity::clone(search), window, cx).detach();
    }
}

impl Render for ProjectSearchBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(search) = self.active_project_search.clone() else {
            return div().into_any_element();
        };
        let search = search.read(cx);
        let focus_handle = search.focus_handle(cx);

        let container_width = window.viewport_size().width;
        let input_width = SearchInputWidth::calc_width(container_width);

        let input_base_styles = |panel: InputPanel| {
            input_base_styles(search.border_color_for(panel, cx), |div| match panel {
                InputPanel::Query | InputPanel::Replacement => div.w(input_width),
                InputPanel::Include | InputPanel::Exclude => div.flex_grow_1(),
            })
        };
        let theme_colors = cx.theme().colors();
        let project_search = search.entity.read(cx);
        let limit_reached = project_search.search_state.limit_reached();
        let deferred_dirs = project_search.search_state.deferred_dirs();
        let is_search_underway = project_search.pending_search.is_some();

        let color_override = match (
            &project_search.active_query,
            &project_search.last_search_query_text,
        ) {
            (Some(query), Some(previous_query))
                if query.as_str() == previous_query
                    && project_search.search_state.no_results_so_far()
                    && project_search.match_ranges.is_empty() =>
            {
                Some(Color::Error)
            }
            _ => None,
        };

        let match_text = search
            .active_match_index
            .and_then(|index| {
                let index = index + 1;
                let match_quantity = project_search.match_ranges.len();
                if match_quantity > 0 {
                    debug_assert!(match_quantity >= index);
                    if limit_reached {
                        Some(format!("{index}/{match_quantity}+"))
                    } else {
                        Some(format!("{index}/{match_quantity}"))
                    }
                } else {
                    None
                }
            })
            .unwrap_or_else(|| "0/0".to_string());

        let query_focus = search.query_editor.focus_handle(cx);

        let query_column = input_base_styles(InputPanel::Query)
            .on_action(cx.listener(|this, action, window, cx| this.confirm(action, window, cx)))
            .on_action(cx.listener(|this, action, window, cx| {
                this.previous_history_query(action, window, cx)
            }))
            .on_action(
                cx.listener(|this, action, window, cx| this.next_history_query(action, window, cx)),
            )
            .child(div().flex_1().py_1().child(render_text_input(
                &search.query_editor,
                color_override,
                cx,
            )))
            .child(
                h_flex()
                    .gap_1()
                    .child(SearchOption::CaseSensitive.as_button(
                        search.search_options,
                        SearchSource::Project(cx),
                        focus_handle.clone(),
                    ))
                    .child(SearchOption::WholeWord.as_button(
                        search.search_options,
                        SearchSource::Project(cx),
                        focus_handle.clone(),
                    ))
                    .child(SearchOption::Regex.as_button(
                        search.search_options,
                        SearchSource::Project(cx),
                        focus_handle.clone(),
                    )),
            );

        let matches_column = h_flex()
            .ml_1()
            .pl_1p5()
            .border_l_1()
            .border_color(theme_colors.border_variant)
            .child(render_action_button(
                "project-search-nav-button",
                IconName::ChevronLeft,
                search
                    .active_match_index
                    .is_none()
                    .then_some(ActionButtonState::Disabled),
                "Select Previous Match",
                &SelectPreviousMatch,
                query_focus.clone(),
            ))
            .child(render_action_button(
                "project-search-nav-button",
                IconName::ChevronRight,
                search
                    .active_match_index
                    .is_none()
                    .then_some(ActionButtonState::Disabled),
                "Select Next Match",
                &SelectNextMatch,
                query_focus.clone(),
            ))
            .child(
                div()
                    .id("matches")
                    .ml_2()
                    .min_w(rems_from_px(40_f32))
                    .child(
                        h_flex()
                            .gap_1p5()
                            .child(
                                Label::new(match_text)
                                    .size(LabelSize::Small)
                                    .when(search.active_match_index.is_some(), |this| {
                                        this.color(Color::Disabled)
                                    }),
                            )
                            .when(is_search_underway, |this| {
                                this.child(
                                    Icon::new(IconName::ArrowCircle)
                                        .color(Color::Accent)
                                        .size(IconSize::Small)
                                        .with_rotate_animation(2)
                                        .into_any_element(),
                                )
                            }),
                    )
                    .when(limit_reached, |this| {
                        this.tooltip(Tooltip::text(
                            "Search Limits Reached\nTry narrowing your search",
                        ))
                    }),
            )
            .when(deferred_dirs > 0, |this| {
                this.child(
                    div()
                        .id("partial-index-warning")
                        .ml_1()
                        .child(
                            Icon::new(IconName::Warning)
                                .color(Color::Warning)
                                .size(IconSize::Small)
                                .into_any_element(),
                        )
                        .tooltip(Tooltip::text(partial_index_message(deferred_dirs))),
                )
            });

        let mode_column = h_flex()
            .gap_1()
            .min_w_64()
            .child(
                IconButton::new("project-search-filter-button", IconName::Filter)
                    .shape(IconButtonShape::Square)
                    .tooltip(|_window, cx| {
                        Tooltip::for_action("Toggle Filters", &ToggleFilters, cx)
                    })
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.toggle_filters(window, cx);
                    }))
                    .toggle_state(
                        self.active_project_search
                            .as_ref()
                            .map(|search| search.read(cx).filters_enabled)
                            .unwrap_or_default(),
                    )
                    .tooltip({
                        let focus_handle = focus_handle.clone();
                        move |_window, cx| {
                            Tooltip::for_action_in(
                                "Toggle Filters",
                                &ToggleFilters,
                                &focus_handle,
                                cx,
                            )
                        }
                    }),
            )
            .child(render_action_button(
                "project-search",
                IconName::Replace,
                self.active_project_search
                    .as_ref()
                    .map(|search| search.read(cx).replace_enabled)
                    .and_then(|enabled| enabled.then_some(ActionButtonState::Toggled)),
                "Toggle Replace",
                &ToggleReplace,
                focus_handle.clone(),
            ))
            .child(matches_column);

        let is_collapsed = search.results_editor.read(cx).has_any_buffer_folded(cx);

        let (icon, tooltip_label) = if is_collapsed {
            (IconName::ChevronUpDown, "Expand All Search Results")
        } else {
            (IconName::ChevronDownUp, "Collapse All Search Results")
        };

        let expand_button = IconButton::new("project-search-collapse-expand", icon)
            .shape(IconButtonShape::Square)
            .tooltip(move |_, cx| {
                Tooltip::for_action_in(
                    tooltip_label,
                    &ToggleAllSearchResults,
                    &query_focus.clone(),
                    cx,
                )
            })
            .on_click(cx.listener(|this, _, window, cx| {
                if let Some(active_view) = &this.active_project_search {
                    active_view.update(cx, |active_view, cx| {
                        active_view.toggle_all_search_results(&ToggleAllSearchResults, window, cx);
                    })
                }
            }));

        let search_line = h_flex()
            .pl_0p5()
            .w_full()
            .gap_2()
            .child(expand_button)
            .child(query_column)
            .child(mode_column);

        let replace_line = search.replace_enabled.then(|| {
            let replace_column = input_base_styles(InputPanel::Replacement).child(
                div().flex_1().py_1().child(render_text_input(
                    &search.replacement_editor,
                    None,
                    cx,
                )),
            );

            let focus_handle = search.replacement_editor.read(cx).focus_handle(cx);
            let replace_actions = h_flex()
                .min_w_64()
                .gap_1()
                .child(render_action_button(
                    "project-search-replace-button",
                    IconName::ReplaceNext,
                    is_search_underway.then_some(ActionButtonState::Disabled),
                    "Replace Next Match",
                    &ReplaceNext,
                    focus_handle.clone(),
                ))
                .child(render_action_button(
                    "project-search-replace-button",
                    IconName::ReplaceAll,
                    Default::default(),
                    "Replace All Matches",
                    &ReplaceAll,
                    focus_handle,
                ));

            h_flex()
                .w_full()
                .gap_2()
                .child(alignment_element())
                .child(replace_column)
                .child(replace_actions)
        });

        let filter_line = search.filters_enabled.then(|| {
            let include = input_base_styles(InputPanel::Include)
                .on_action(cx.listener(|this, action, window, cx| {
                    this.previous_history_query(action, window, cx)
                }))
                .on_action(cx.listener(|this, action, window, cx| {
                    this.next_history_query(action, window, cx)
                }))
                .child(render_text_input(&search.included_files_editor, None, cx));
            let exclude = input_base_styles(InputPanel::Exclude)
                .on_action(cx.listener(|this, action, window, cx| {
                    this.previous_history_query(action, window, cx)
                }))
                .on_action(cx.listener(|this, action, window, cx| {
                    this.next_history_query(action, window, cx)
                }))
                .child(render_text_input(&search.excluded_files_editor, None, cx));
            let mode_column = h_flex()
                .gap_1()
                .min_w_64()
                .child(
                    IconButton::new("project-search-opened-only", IconName::FolderSearch)
                        .shape(IconButtonShape::Square)
                        .toggle_state(self.is_opened_only_enabled(cx))
                        .tooltip(Tooltip::text("Only Search Open Files"))
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.toggle_opened_only(window, cx);
                        })),
                )
                .child(SearchOption::IncludeIgnored.as_button(
                    search.search_options,
                    SearchSource::Project(cx),
                    focus_handle,
                ));

            h_flex()
                .w_full()
                .gap_2()
                .child(alignment_element())
                .child(
                    h_flex()
                        .w(input_width)
                        .gap_2()
                        .child(include)
                        .child(exclude),
                )
                .child(mode_column)
        });

        let mut key_context = KeyContext::default();
        key_context.add("ProjectSearchBar");
        if search
            .replacement_editor
            .focus_handle(cx)
            .is_focused(window)
        {
            key_context.add("in_replace");
        }

        let query_error_line = search
            .panels_with_errors
            .get(&InputPanel::Query)
            .map(|error| {
                Label::new(error)
                    .size(LabelSize::Small)
                    .color(Color::Error)
                    .mt_neg_1()
                    .ml_2()
            });

        let filter_error_line = search
            .panels_with_errors
            .get(&InputPanel::Include)
            .or_else(|| search.panels_with_errors.get(&InputPanel::Exclude))
            .map(|error| {
                Label::new(error)
                    .size(LabelSize::Small)
                    .color(Color::Error)
                    .mt_neg_1()
                    .ml_2()
            });

        v_flex()
            .gap_2()
            .w_full()
            .key_context(key_context)
            .on_action(cx.listener(|this, _: &ToggleFocus, window, cx| {
                this.move_focus_to_results(window, cx)
            }))
            .on_action(cx.listener(|this, _: &ToggleFilters, window, cx| {
                this.toggle_filters(window, cx);
            }))
            .capture_action(cx.listener(Self::tab))
            .capture_action(cx.listener(Self::backtab))
            .on_action(cx.listener(|this, action, window, cx| this.confirm(action, window, cx)))
            .on_action(cx.listener(|this, action, window, cx| {
                this.toggle_replace(action, window, cx);
            }))
            .on_action(cx.listener(|this, _: &ToggleWholeWord, window, cx| {
                this.toggle_search_option(SearchOptions::WHOLE_WORD, window, cx);
            }))
            .on_action(cx.listener(|this, _: &ToggleCaseSensitive, window, cx| {
                this.toggle_search_option(SearchOptions::CASE_SENSITIVE, window, cx);
            }))
            .on_action(cx.listener(|this, action, window, cx| {
                if let Some(search) = this.active_project_search.as_ref() {
                    search.update(cx, |this, cx| {
                        this.replace_next(action, window, cx);
                    })
                }
            }))
            .on_action(cx.listener(|this, action, window, cx| {
                if let Some(search) = this.active_project_search.as_ref() {
                    search.update(cx, |this, cx| {
                        this.replace_all(action, window, cx);
                    })
                }
            }))
            .when(search.filters_enabled, |this| {
                this.on_action(cx.listener(|this, _: &ToggleIncludeIgnored, window, cx| {
                    this.toggle_search_option(SearchOptions::INCLUDE_IGNORED, window, cx);
                }))
            })
            .on_action(cx.listener(Self::select_next_match))
            .on_action(cx.listener(Self::select_prev_match))
            .on_action(cx.listener(Self::open_text_finder))
            .child(search_line)
            .children(query_error_line)
            .children(replace_line)
            .children(filter_line)
            .children(filter_error_line)
            .into_any_element()
    }
}

impl EventEmitter<ToolbarItemEvent> for ProjectSearchBar {}

impl ToolbarItemView for ProjectSearchBar {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> ToolbarItemLocation {
        cx.notify();
        self.subscription = None;
        self.active_project_search = None;
        if let Some(search) = active_pane_item.and_then(|i| i.downcast::<ProjectSearchView>()) {
            self.subscription = Some(cx.observe(&search, |_, _, cx| cx.notify()));
            self.active_project_search = Some(search);
            ToolbarItemLocation::PrimaryLeft {}
        } else {
            ToolbarItemLocation::Hidden
        }
    }
}

fn register_workspace_action<A: Action>(
    workspace: &mut Workspace,
    callback: fn(&mut ProjectSearchBar, &A, &mut Window, &mut Context<ProjectSearchBar>),
) {
    workspace.register_action(move |workspace, action: &A, window, cx| {
        if workspace.has_active_modal(window, cx) && !workspace.hide_modal(window, cx) {
            cx.propagate();
            return;
        }

        workspace.active_pane().update(cx, |pane, cx| {
            pane.toolbar().update(cx, move |workspace, cx| {
                if let Some(search_bar) = workspace.item_of_type::<ProjectSearchBar>() {
                    search_bar.update(cx, move |search_bar, cx| {
                        if search_bar.active_project_search.is_some() {
                            callback(search_bar, action, window, cx);
                            cx.notify();
                        } else {
                            cx.propagate();
                        }
                    });
                }
            });
        })
    });
}

fn register_workspace_action_for_present_search<A: Action>(
    workspace: &mut Workspace,
    callback: fn(&mut Workspace, &A, &mut Window, &mut Context<Workspace>),
) {
    workspace.register_action(move |workspace, action: &A, window, cx| {
        if workspace.has_active_modal(window, cx) && !workspace.hide_modal(window, cx) {
            cx.propagate();
            return;
        }

        let should_notify = workspace
            .active_pane()
            .read(cx)
            .toolbar()
            .read(cx)
            .item_of_type::<ProjectSearchBar>()
            .map(|search_bar| search_bar.read(cx).active_project_search.is_some())
            .unwrap_or(false);
        if should_notify {
            callback(workspace, action, window, cx);
            cx.notify();
        } else {
            cx.propagate();
        }
    });
}

fn is_buffer_stale(
    project: &Entity<Project>,
    workspace: Option<&Entity<Workspace>>,
    buffer: &Entity<Buffer>,
    cx: &App,
) -> bool {
    let buffer_entity_id = buffer.entity_id();
    let buffer = buffer.read(cx);
    if let Some(file) = buffer.file() {
        file.disk_state().is_deleted()
    } else if let Some(workspace) = workspace {
        !workspace.read(cx).items(cx).any(|item| {
            item.buffer_kind(cx) == ItemBufferKind::Singleton
                && item.project_item_model_ids(cx).contains(&buffer_entity_id)
        }) && !project
            .read(cx)
            .buffer_store()
            .read(cx)
            .is_shared(buffer.remote_id(), cx)
    } else {
        false
    }
}

#[cfg(any(test, feature = "test-support"))]
pub fn perform_project_search(
    search_view: &Entity<ProjectSearchView>,
    text: impl Into<std::sync::Arc<str>>,
    cx: &mut gpui::VisualTestContext,
) {
    cx.run_until_parked();
    search_view.update_in(cx, |search_view, window, cx| {
        search_view.query_editor.update(cx, |query_editor, cx| {
            query_editor.set_text(text, window, cx)
        });
        search_view.search(SearchMode::Manual, cx);
    });
    cx.run_until_parked();
}

#[cfg(test)]
pub mod tests {
    use std::{
        cell::RefCell,
        path::PathBuf,
        rc::Rc,
        sync::{
            Arc,
            atomic::{self, AtomicUsize},
        },
        time::Duration,
    };

    use super::*;
    use editor::{DisplayPoint, ToPoint, display_map::DisplayRow};
    use gpui::{Action, TestAppContext, VisualTestContext, WindowHandle};
    use language::{FakeLspAdapter, Point as BufferPoint, rust_lang};
    use pretty_assertions::assert_eq;
    use project::{FakeFs, Fs};
    use serde_json::json;
    use settings::{
        InlayHintSettingsContent, SearchSettingsContent, SeedQuerySetting, SettingsStore,
        SplicingVec, ThemeColorsContent, ThemeStyleContent,
    };
    use util::{path, paths::PathStyle, rel_path::rel_path};
    use util_macros::perf;
    use workspace::{DeploySearch, MultiWorkspace, SaveIntent};

    #[test]
    fn test_split_glob_patterns() {
        assert_eq!(split_glob_patterns("a,b,c"), vec!["a", "b", "c"]);
        assert_eq!(split_glob_patterns("a, b, c"), vec!["a", " b", " c"]);
        assert_eq!(
            split_glob_patterns("src/{a,b}/**/*.rs"),
            vec!["src/{a,b}/**/*.rs"]
        );
        assert_eq!(
            split_glob_patterns("src/{a,b}/*.rs, tests/**/*.rs"),
            vec!["src/{a,b}/*.rs", " tests/**/*.rs"]
        );
        assert_eq!(split_glob_patterns("{a,b},{c,d}"), vec!["{a,b}", "{c,d}"]);
        assert_eq!(split_glob_patterns("{{a,b},{c,d}}"), vec!["{{a,b},{c,d}}"]);
        assert_eq!(split_glob_patterns(""), vec![""]);
        assert_eq!(split_glob_patterns("a"), vec!["a"]);
        // Escaped characters should not be treated as special
        assert_eq!(split_glob_patterns(r"a\,b,c"), vec![r"a\,b", "c"]);
        assert_eq!(split_glob_patterns(r"\{a,b\}"), vec![r"\{a", r"b\}"]);
        assert_eq!(split_glob_patterns(r"a\\,b"), vec![r"a\\", "b"]);
        assert_eq!(split_glob_patterns(r"a\\\,b"), vec![r"a\\\,b"]);
    }

    #[perf]
    #[gpui::test]
    async fn test_ignored_dot_git_directory_results_follow_include_ignored_option(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/dir"),
            json!({
                ".gitignore": "log/\n",
                "app": {
                    "a.txt": "hello",
                },
                "log": {
                    ".git": {},
                    "b.txt": "hello",
                },
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let window =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let search = cx.new(|cx| ProjectSearch::new(project, workspace.downgrade(), cx));
        let search_view = cx.add_window(|window, cx| {
            ProjectSearchView::new(workspace.downgrade(), search, window, cx, None)
        });

        perform_search(search_view, "hello", cx);
        assert_eq!(
            search_view
                .update(cx, |search_view, _, cx| {
                    search_view.entity.read(cx).match_ranges.len()
                })
                .unwrap(),
            1
        );

        search_view
            .update(cx, |search_view, _, cx| {
                search_view.toggle_search_option(SearchOptions::INCLUDE_IGNORED, cx);
            })
            .unwrap();
        perform_search(search_view, "hello", cx);
        assert_eq!(
            search_view
                .update(cx, |search_view, _, cx| {
                    search_view.entity.read(cx).match_ranges.len()
                })
                .unwrap(),
            2
        );

        search_view
            .update(cx, |search_view, _, cx| {
                search_view.toggle_search_option(SearchOptions::INCLUDE_IGNORED, cx);
            })
            .unwrap();
        perform_search(search_view, "hello", cx);
        assert_eq!(
            search_view
                .update(cx, |search_view, _, cx| {
                    search_view.entity.read(cx).match_ranges.len()
                })
                .unwrap(),
            1
        );
    }

    #[perf]
    #[gpui::test]
    async fn test_unignored_dot_git_directory_results_are_included(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/dir"),
            json!({
                "app": {
                    "a.txt": "hello",
                },
                "log": {
                    ".git": {},
                    "b.txt": "hello",
                },
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let window =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let search = cx.new(|cx| ProjectSearch::new(project, workspace.downgrade(), cx));
        let search_view = cx.add_window(|window, cx| {
            ProjectSearchView::new(workspace.downgrade(), search, window, cx, None)
        });

        perform_search(search_view, "hello", cx);
        assert_eq!(
            search_view
                .update(cx, |search_view, _, cx| {
                    search_view.entity.read(cx).match_ranges.len()
                })
                .unwrap(),
            2
        );
    }

    #[perf]
    #[gpui::test]
    async fn test_nested_gitignore_results_follow_include_ignored_option(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/dir"),
            json!({
                "app": {
                    "a.txt": "hello",
                },
                "log": {
                    ".git": {},
                    ".gitignore": "b.txt\n",
                    "b.txt": "hello",
                },
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let window =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let search = cx.new(|cx| ProjectSearch::new(project, workspace.downgrade(), cx));
        let search_view = cx.add_window(|window, cx| {
            ProjectSearchView::new(workspace.downgrade(), search, window, cx, None)
        });

        perform_search(search_view, "hello", cx);
        assert_eq!(
            search_view
                .update(cx, |search_view, _, cx| {
                    search_view.entity.read(cx).match_ranges.len()
                })
                .unwrap(),
            1
        );

        search_view
            .update(cx, |search_view, _, cx| {
                search_view.toggle_search_option(SearchOptions::INCLUDE_IGNORED, cx);
            })
            .unwrap();
        perform_search(search_view, "hello", cx);
        assert_eq!(
            search_view
                .update(cx, |search_view, _, cx| {
                    search_view.entity.read(cx).match_ranges.len()
                })
                .unwrap(),
            2
        );
    }

    #[perf]
    #[gpui::test]
    async fn test_project_search(cx: &mut TestAppContext) {
        fn dp(row: u32, col: u32) -> DisplayPoint {
            DisplayPoint::new(DisplayRow(row), col)
        }

        fn assert_active_match_index(
            search_view: &WindowHandle<ProjectSearchView>,
            cx: &mut TestAppContext,
            expected_index: usize,
        ) {
            search_view
                .update(cx, |search_view, _window, _cx| {
                    assert_eq!(search_view.active_match_index, Some(expected_index));
                })
                .unwrap();
        }

        fn assert_selection_range(
            search_view: &WindowHandle<ProjectSearchView>,
            cx: &mut TestAppContext,
            expected_range: Range<DisplayPoint>,
        ) {
            search_view
                .update(cx, |search_view, _window, cx| {
                    assert_eq!(
                        search_view.results_editor.update(cx, |editor, cx| editor
                            .selections
                            .display_ranges(&editor.display_snapshot(cx))),
                        [expected_range]
                    );
                })
                .unwrap();
        }

        fn assert_highlights(
            search_view: &WindowHandle<ProjectSearchView>,
            cx: &mut TestAppContext,
            expected_highlights: Vec<(Range<DisplayPoint>, &str)>,
        ) {
            search_view
                .update(cx, |search_view, window, cx| {
                    let match_bg = cx.theme().colors().search_match_background;
                    let active_match_bg = cx.theme().colors().search_active_match_background;
                    let selection_bg = cx
                        .theme()
                        .colors()
                        .editor_document_highlight_bracket_background;

                    let highlights: Vec<_> = expected_highlights
                        .into_iter()
                        .map(|(range, color_type)| {
                            let color = match color_type {
                                "active" => active_match_bg,
                                "match" => match_bg,
                                "selection" => selection_bg,
                                _ => panic!("Unknown color type"),
                            };
                            (range, color)
                        })
                        .collect();

                    assert_eq!(
                        search_view.results_editor.update(cx, |editor, cx| editor
                            .all_text_background_highlights(window, cx)),
                        highlights.as_slice()
                    );
                })
                .unwrap();
        }

        fn select_match(
            search_view: &WindowHandle<ProjectSearchView>,
            cx: &mut TestAppContext,
            direction: Direction,
        ) {
            search_view
                .update(cx, |search_view, window, cx| {
                    search_view.select_match(direction, window, cx);
                })
                .unwrap();
        }

        init_test(cx);

        // Override active search match color since the fallback theme uses the same color
        // for normal search match and active one, which can make this test less robust.
        cx.update(|cx| {
            SettingsStore::update_global(cx, |settings, cx| {
                settings.update_user_settings(cx, |settings| {
                    settings.theme.experimental_theme_overrides = Some(ThemeStyleContent {
                        colors: ThemeColorsContent {
                            search_active_match_background: Some("#ff0000ff".into()),
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });
            });
        });

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/dir"),
            json!({
                "one.rs": "const ONE: usize = 1;",
                "two.rs": "const TWO: usize = one::ONE + one::ONE;",
                "three.rs": "const THREE: usize = one::ONE + two::TWO;",
                "four.rs": "const FOUR: usize = one::ONE + three::THREE;",
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let window =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let search = cx.new(|cx| ProjectSearch::new(project.clone(), workspace.downgrade(), cx));
        let search_view = cx.add_window(|window, cx| {
            ProjectSearchView::new(workspace.downgrade(), search.clone(), window, cx, None)
        });

        perform_search(search_view, "TWO", cx);
        cx.run_until_parked();

        search_view
            .update(cx, |search_view, _window, cx| {
                assert_eq!(
                    search_view
                        .results_editor
                        .update(cx, |editor, cx| editor.display_text(cx)),
                    "\n\nconst THREE: usize = one::ONE + two::TWO;\n\n\nconst TWO: usize = one::ONE + one::ONE;"
                );
            })
            .unwrap();

        assert_active_match_index(&search_view, cx, 0);
        assert_selection_range(&search_view, cx, dp(2, 32)..dp(2, 35));
        assert_highlights(
            &search_view,
            cx,
            vec![
                (dp(2, 32)..dp(2, 35), "active"),
                (dp(2, 37)..dp(2, 40), "selection"),
                (dp(2, 37)..dp(2, 40), "match"),
                (dp(5, 6)..dp(5, 9), "selection"),
                (dp(5, 6)..dp(5, 9), "match"),
            ],
        );
        select_match(&search_view, cx, Direction::Next);
        cx.run_until_parked();

        assert_active_match_index(&search_view, cx, 1);
        assert_selection_range(&search_view, cx, dp(2, 37)..dp(2, 40));
        assert_highlights(
            &search_view,
            cx,
            vec![
                (dp(2, 32)..dp(2, 35), "selection"),
                (dp(2, 32)..dp(2, 35), "match"),
                (dp(2, 37)..dp(2, 40), "active"),
                (dp(5, 6)..dp(5, 9), "selection"),
                (dp(5, 6)..dp(5, 9), "match"),
            ],
        );
        select_match(&search_view, cx, Direction::Next);
        cx.run_until_parked();

        assert_active_match_index(&search_view, cx, 2);
        assert_selection_range(&search_view, cx, dp(5, 6)..dp(5, 9));
        assert_highlights(
            &search_view,
            cx,
            vec![
                (dp(2, 32)..dp(2, 35), "selection"),
                (dp(2, 32)..dp(2, 35), "match"),
                (dp(2, 37)..dp(2, 40), "selection"),
                (dp(2, 37)..dp(2, 40), "match"),
                (dp(5, 6)..dp(5, 9), "active"),
            ],
        );
        select_match(&search_view, cx, Direction::Next);
        cx.run_until_parked();

        assert_active_match_index(&search_view, cx, 0);
        assert_selection_range(&search_view, cx, dp(2, 32)..dp(2, 35));
        assert_highlights(
            &search_view,
            cx,
            vec![
                (dp(2, 32)..dp(2, 35), "active"),
                (dp(2, 37)..dp(2, 40), "selection"),
                (dp(2, 37)..dp(2, 40), "match"),
                (dp(5, 6)..dp(5, 9), "selection"),
                (dp(5, 6)..dp(5, 9), "match"),
            ],
        );
        select_match(&search_view, cx, Direction::Prev);
        cx.run_until_parked();

        assert_active_match_index(&search_view, cx, 2);
        assert_selection_range(&search_view, cx, dp(5, 6)..dp(5, 9));
        assert_highlights(
            &search_view,
            cx,
            vec![
                (dp(2, 32)..dp(2, 35), "selection"),
                (dp(2, 32)..dp(2, 35), "match"),
                (dp(2, 37)..dp(2, 40), "selection"),
                (dp(2, 37)..dp(2, 40), "match"),
                (dp(5, 6)..dp(5, 9), "active"),
            ],
        );
        select_match(&search_view, cx, Direction::Prev);
        cx.run_until_parked();

        assert_active_match_index(&search_view, cx, 1);
        assert_selection_range(&search_view, cx, dp(2, 37)..dp(2, 40));
        assert_highlights(
            &search_view,
            cx,
            vec![
                (dp(2, 32)..dp(2, 35), "selection"),
                (dp(2, 32)..dp(2, 35), "match"),
                (dp(2, 37)..dp(2, 40), "active"),
                (dp(5, 6)..dp(5, 9), "selection"),
                (dp(5, 6)..dp(5, 9), "match"),
            ],
        );
        search_view
            .update(cx, |search_view, window, cx| {
                search_view.results_editor.update(cx, |editor, cx| {
                    editor.fold_all(&FoldAll, window, cx);
                })
            })
            .expect("Should fold fine");
        cx.run_until_parked();

        let results_collapsed = search_view
            .read_with(cx, |search_view, cx| {
                search_view
                    .results_editor
                    .read(cx)
                    .has_any_buffer_folded(cx)
            })
            .expect("got results_collapsed");

        assert!(results_collapsed);
        search_view
            .update(cx, |search_view, window, cx| {
                search_view.results_editor.update(cx, |editor, cx| {
                    editor.unfold_all(&UnfoldAll, window, cx);
                })
            })
            .expect("Should unfold fine");
        cx.run_until_parked();

        let results_collapsed = search_view
            .read_with(cx, |search_view, cx| {
                search_view
                    .results_editor
                    .read(cx)
                    .has_any_buffer_folded(cx)
            })
            .expect("got results_collapsed");

        assert!(!results_collapsed);
    }

    #[gpui::test]
    async fn test_search_results_do_not_read_closed_untitled_buffer(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/dir"),
            json!({
                "one.rs": "const ONE: usize = 1;",
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let window =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();

        let untitled_buffer = project.update(cx, |project, cx| {
            project.create_local_buffer("const TWO: usize = one::ONE;\n", None, true, cx)
        });
        let editor = window
            .update(cx, |_, window, cx| {
                let multibuffer = MultiBuffer::build_from_buffer(untitled_buffer.clone(), cx);
                let editor = cx.new(|cx| {
                    Editor::new(
                        editor::EditorMode::full(),
                        multibuffer,
                        Some(project.clone()),
                        window,
                        cx,
                    )
                });
                workspace.update(cx, |workspace, cx| {
                    workspace.add_item_to_center(Box::new(editor.clone()), window, cx);
                });
                editor
            })
            .unwrap();

        let search = cx.new(|cx| ProjectSearch::new(project, workspace.downgrade(), cx));
        let search_view = cx.add_window(|window, cx| {
            ProjectSearchView::new(workspace.downgrade(), search.clone(), window, cx, None)
        });

        perform_search(search_view, "const", cx);

        search_view
            .update(cx, |search_view, _window, cx| {
                let results_text = search_view
                    .results_editor
                    .update(cx, |editor, cx| editor.display_text(cx));

                assert_eq!(
                    "\n\nconst TWO: usize = one::ONE;\n\n\n\nconst ONE: usize = 1;",
                    results_text
                );
            })
            .unwrap();

        let pane = cx.read(|cx| workspace.read(cx).active_pane().clone());
        let close_task = window
            .update(cx, |_, window, cx| {
                pane.update(cx, |pane, cx| {
                    pane.close_item_by_id(editor.entity_id(), SaveIntent::Skip, window, cx)
                })
            })
            .unwrap();
        close_task.await.unwrap();
        cx.run_until_parked();

        search_view
            .update(cx, |search_view, _window, cx| {
                let results_text = search_view
                    .results_editor
                    .update(cx, |editor, cx| editor.display_text(cx));
                assert_eq!("\n\nconst ONE: usize = 1;", results_text);
            })
            .unwrap();

        // Re-run the search and verify the closed untitled buffer stays gone
        perform_search(search_view, "const", cx);

        search_view
            .update(cx, |search_view, _window, cx| {
                let results_text = search_view
                    .results_editor
                    .update(cx, |editor, cx| editor.display_text(cx));
                assert_eq!("\n\nconst ONE: usize = 1;", results_text);
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_search_results_keep_peer_shared_untitled_buffers(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/dir"),
            json!({
                "one.rs": "const ONE: usize = 1;",
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let window =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();

        let untitled_buffer = project.update(cx, |project, cx| {
            project.create_local_buffer("const TWO: usize = one::ONE;\n", None, true, cx)
        });
        project.update(cx, |project, cx| {
            project.buffer_store().update(cx, |buffer_store, cx| {
                buffer_store
                    .create_buffer_for_peer(
                        &untitled_buffer,
                        proto::PeerId { owner_id: 0, id: 1 },
                        cx,
                    )
                    .detach_and_log_err(cx);
            });
        });

        let search = cx.new(|cx| ProjectSearch::new(project, workspace.downgrade(), cx));
        let search_view = cx.add_window(|window, cx| {
            ProjectSearchView::new(workspace.downgrade(), search.clone(), window, cx, None)
        });

        perform_search(search_view, "const", cx);
        search_view
            .update(cx, |search_view, _window, cx| {
                let results_text = search_view
                    .results_editor
                    .update(cx, |editor, cx| editor.display_text(cx));
                assert_eq!(
                    "\n\nconst TWO: usize = one::ONE;\n\n\n\nconst ONE: usize = 1;",
                    results_text
                );
            })
            .unwrap();

        search.update(cx, |search, cx| search.remove_closed_untitled_buffers(cx));
        cx.run_until_parked();

        search_view
            .update(cx, |search_view, _window, cx| {
                let results_text = search_view
                    .results_editor
                    .update(cx, |editor, cx| editor.display_text(cx));
                assert_eq!(
                    "\n\nconst TWO: usize = one::ONE;\n\n\n\nconst ONE: usize = 1;",
                    results_text
                );
            })
            .unwrap();

        perform_search(search_view, "const", cx);

        search_view
            .update(cx, |search_view, _window, cx| {
                let results_text = search_view
                    .results_editor
                    .update(cx, |editor, cx| editor.display_text(cx));
                assert_eq!(
                    "\n\nconst TWO: usize = one::ONE;\n\n\n\nconst ONE: usize = 1;",
                    results_text
                );
            })
            .unwrap();
    }

    #[perf]
    #[gpui::test]
    async fn test_collapse_state_syncs_after_manual_buffer_fold(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/dir"),
            json!({
                "one.rs": "const ONE: usize = 1;",
                "two.rs": "const TWO: usize = one::ONE + one::ONE;",
                "three.rs": "const THREE: usize = one::ONE + two::TWO;",
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let window =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let search = cx.new(|cx| ProjectSearch::new(project.clone(), workspace.downgrade(), cx));
        let search_view = cx.add_window(|window, cx| {
            ProjectSearchView::new(workspace.downgrade(), search.clone(), window, cx, None)
        });

        // Search for "ONE" which appears in all 3 files
        perform_search(search_view, "ONE", cx);

        // Verify initial state: no folds
        let has_any_folded = search_view
            .read_with(cx, |search_view, cx| {
                search_view
                    .results_editor
                    .read(cx)
                    .has_any_buffer_folded(cx)
            })
            .expect("should read state");
        assert!(!has_any_folded, "No buffers should be folded initially");

        // Fold all via fold_all
        search_view
            .update(cx, |search_view, window, cx| {
                search_view.results_editor.update(cx, |editor, cx| {
                    editor.fold_all(&FoldAll, window, cx);
                })
            })
            .expect("Should fold fine");
        cx.run_until_parked();

        let has_any_folded = search_view
            .read_with(cx, |search_view, cx| {
                search_view
                    .results_editor
                    .read(cx)
                    .has_any_buffer_folded(cx)
            })
            .expect("should read state");
        assert!(
            has_any_folded,
            "All buffers should be folded after fold_all"
        );

        // Manually unfold one buffer (simulating a chevron click)
        let first_buffer_id = search_view
            .read_with(cx, |search_view, cx| {
                search_view
                    .results_editor
                    .read(cx)
                    .buffer()
                    .read(cx)
                    .snapshot(cx)
                    .excerpts()
                    .next()
                    .unwrap()
                    .context
                    .start
                    .buffer_id
            })
            .expect("should read buffer ids");

        search_view
            .update(cx, |search_view, _window, cx| {
                search_view.results_editor.update(cx, |editor, cx| {
                    editor.unfold_buffer(first_buffer_id, cx);
                })
            })
            .expect("Should unfold one buffer");

        let has_any_folded = search_view
            .read_with(cx, |search_view, cx| {
                search_view
                    .results_editor
                    .read(cx)
                    .has_any_buffer_folded(cx)
            })
            .expect("should read state");
        assert!(
            has_any_folded,
            "Should still report folds when only one buffer is unfolded"
        );

        // Unfold all via unfold_all
        search_view
            .update(cx, |search_view, window, cx| {
                search_view.results_editor.update(cx, |editor, cx| {
                    editor.unfold_all(&UnfoldAll, window, cx);
                })
            })
            .expect("Should unfold fine");
        cx.run_until_parked();

        let has_any_folded = search_view
            .read_with(cx, |search_view, cx| {
                search_view
                    .results_editor
                    .read(cx)
                    .has_any_buffer_folded(cx)
            })
            .expect("should read state");
        assert!(!has_any_folded, "No folds should remain after unfold_all");

        // Manually fold one buffer back (simulating a chevron click)
        search_view
            .update(cx, |search_view, _window, cx| {
                search_view.results_editor.update(cx, |editor, cx| {
                    editor.fold_buffer(first_buffer_id, cx);
                })
            })
            .expect("Should fold one buffer");

        let has_any_folded = search_view
            .read_with(cx, |search_view, cx| {
                search_view
                    .results_editor
                    .read(cx)
                    .has_any_buffer_folded(cx)
            })
            .expect("should read state");
        assert!(
            has_any_folded,
            "Should report folds after manually folding one buffer"
        );
    }

    #[perf]
    #[gpui::test]
    async fn test_deploy_project_search_focus(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            "/dir",
            json!({
                "one.rs": "const ONE: usize = 1;",
                "two.rs": "const TWO: usize = one::ONE + one::ONE;",
                "three.rs": "const THREE: usize = one::ONE + two::TWO;",
                "four.rs": "const FOUR: usize = one::ONE + three::THREE;",
            }),
        )
        .await;
        let project = Project::test(fs.clone(), ["/dir".as_ref()], cx).await;
        let window = cx.add_window(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let workspace = window
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let cx = &mut VisualTestContext::from_window(window.into(), cx);
        let search_bar = window.build_entity(cx, |_, _| ProjectSearchBar::new());

        let active_item = cx.read(|cx| {
            workspace
                .read(cx)
                .active_pane()
                .read(cx)
                .active_item()
                .and_then(|item| item.downcast::<ProjectSearchView>())
        });
        assert!(
            active_item.is_none(),
            "Expected no search panel to be active"
        );

        workspace.update_in(cx, move |workspace, window, cx| {
            assert_eq!(workspace.panes().len(), 1);
            workspace.panes()[0].update(cx, |pane, cx| {
                pane.toolbar()
                    .update(cx, |toolbar, cx| toolbar.add_item(search_bar, window, cx))
            });

            ProjectSearchView::deploy_search(
                workspace,
                &workspace::DeploySearch::default(),
                window,
                cx,
            )
        });

        let Some(search_view) = cx.read(|cx| {
            workspace
                .read(cx)
                .active_pane()
                .read(cx)
                .active_item()
                .and_then(|item| item.downcast::<ProjectSearchView>())
        }) else {
            panic!("Search view expected to appear after new search event trigger")
        };

        cx.spawn(|mut cx| async move {
            window
                .update(&mut cx, |_, window, cx| {
                    window.dispatch_action(ToggleFocus.boxed_clone(), cx)
                })
                .unwrap();
        })
        .detach();
        cx.background_executor.run_until_parked();
        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    assert!(
                        search_view.query_editor.focus_handle(cx).is_focused(window),
                        "Empty search view should be focused after the toggle focus event: no results panel to focus on",
                    );
                });
        }).unwrap();

        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    let query_editor = &search_view.query_editor;
                    assert!(
                        query_editor.focus_handle(cx).is_focused(window),
                        "Search view should be focused after the new search view is activated",
                    );
                    let query_text = query_editor.read(cx).text(cx);
                    assert!(
                        query_text.is_empty(),
                        "New search query should be empty but got '{query_text}'",
                    );
                    let results_text = search_view
                        .results_editor
                        .update(cx, |editor, cx| editor.display_text(cx));
                    assert!(
                        results_text.is_empty(),
                        "Empty search view should have no results but got '{results_text}'"
                    );
                });
            })
            .unwrap();

        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    search_view.query_editor.update(cx, |query_editor, cx| {
                        query_editor.set_text("sOMETHINGtHATsURELYdOESnOTeXIST", window, cx)
                    });
                    search_view.search(SearchMode::Manual, cx);
                });
            })
            .unwrap();
        cx.background_executor.run_until_parked();
        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    let results_text = search_view
                        .results_editor
                        .update(cx, |editor, cx| editor.display_text(cx));
                    assert!(
                        results_text.is_empty(),
                        "Search view for mismatching query should have no results but got '{results_text}'"
                    );
                    assert!(
                        search_view.query_editor.focus_handle(cx).is_focused(window),
                        "Search view should be focused after mismatching query had been used in search",
                    );
                });
            }).unwrap();

        cx.spawn(|mut cx| async move {
            window.update(&mut cx, |_, window, cx| {
                window.dispatch_action(ToggleFocus.boxed_clone(), cx)
            })
        })
        .detach();
        cx.background_executor.run_until_parked();
        window.update(cx, |_, window, cx| {
            search_view.update(cx, |search_view, cx| {
                assert!(
                    search_view.query_editor.focus_handle(cx).is_focused(window),
                    "Search view with mismatching query should be focused after the toggle focus event: still no results panel to focus on",
                );
            });
        }).unwrap();

        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    search_view.query_editor.update(cx, |query_editor, cx| {
                        query_editor.set_text("TWO", window, cx)
                    });
                    search_view.search(SearchMode::Manual, cx);
                });
            })
            .unwrap();
        cx.background_executor.run_until_parked();
        window.update(cx, |_, window, cx| {
            search_view.update(cx, |search_view, cx| {
                assert_eq!(
                    search_view
                        .results_editor
                        .update(cx, |editor, cx| editor.display_text(cx)),
                    "\n\nconst THREE: usize = one::ONE + two::TWO;\n\n\nconst TWO: usize = one::ONE + one::ONE;",
                    "Search view results should match the query"
                );
                assert!(
                    search_view.results_editor.focus_handle(cx).is_focused(window),
                    "Search view with mismatching query should be focused after search results are available",
                );
            });
        }).unwrap();
        cx.spawn(|mut cx| async move {
            window
                .update(&mut cx, |_, window, cx| {
                    window.dispatch_action(ToggleFocus.boxed_clone(), cx)
                })
                .unwrap();
        })
        .detach();
        cx.background_executor.run_until_parked();
        window.update(cx, |_, window, cx| {
            search_view.update(cx, |search_view, cx| {
                assert!(
                    search_view.results_editor.focus_handle(cx).is_focused(window),
                    "Search view with matching query should still have its results editor focused after the toggle focus event",
                );
            });
        }).unwrap();

        workspace.update_in(cx, |workspace, window, cx| {
            ProjectSearchView::deploy_search(
                workspace,
                &workspace::DeploySearch::default(),
                window,
                cx,
            )
        });
        window.update(cx, |_, window, cx| {
            search_view.update(cx, |search_view, cx| {
                assert_eq!(search_view.query_editor.read(cx).text(cx), "two", "Query should be updated to first search result after search view 2nd open in a row");
                assert_eq!(
                    search_view
                        .results_editor
                        .update(cx, |editor, cx| editor.display_text(cx)),
                    "\n\nconst THREE: usize = one::ONE + two::TWO;\n\n\nconst TWO: usize = one::ONE + one::ONE;",
                    "Results should be unchanged after search view 2nd open in a row"
                );
                assert!(
                    search_view.query_editor.focus_handle(cx).is_focused(window),
                    "Focus should be moved into query editor again after search view 2nd open in a row"
                );
            });
        }).unwrap();

        cx.spawn(|mut cx| async move {
            window
                .update(&mut cx, |_, window, cx| {
                    window.dispatch_action(ToggleFocus.boxed_clone(), cx)
                })
                .unwrap();
        })
        .detach();
        cx.background_executor.run_until_parked();
        window.update(cx, |_, window, cx| {
            search_view.update(cx, |search_view, cx| {
                assert!(
                    search_view.results_editor.focus_handle(cx).is_focused(window),
                    "Search view with matching query should switch focus to the results editor after the toggle focus event",
                );
            });
        }).unwrap();
    }

    #[perf]
    #[gpui::test]
    async fn test_filters_consider_toggle_state(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            "/dir",
            json!({
                "one.rs": "const ONE: usize = 1;",
                "two.rs": "const TWO: usize = one::ONE + one::ONE;",
                "three.rs": "const THREE: usize = one::ONE + two::TWO;",
                "four.rs": "const FOUR: usize = one::ONE + three::THREE;",
            }),
        )
        .await;
        let project = Project::test(fs.clone(), ["/dir".as_ref()], cx).await;
        let window = cx.add_window(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let workspace = window
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let cx = &mut VisualTestContext::from_window(window.into(), cx);
        let search_bar = window.build_entity(cx, |_, _| ProjectSearchBar::new());

        workspace.update_in(cx, move |workspace, window, cx| {
            workspace.panes()[0].update(cx, |pane, cx| {
                pane.toolbar()
                    .update(cx, |toolbar, cx| toolbar.add_item(search_bar, window, cx))
            });

            ProjectSearchView::deploy_search(
                workspace,
                &workspace::DeploySearch::default(),
                window,
                cx,
            )
        });

        let Some(search_view) = cx.read(|cx| {
            workspace
                .read(cx)
                .active_pane()
                .read(cx)
                .active_item()
                .and_then(|item| item.downcast::<ProjectSearchView>())
        }) else {
            panic!("Search view expected to appear after new search event trigger")
        };

        cx.spawn(|mut cx| async move {
            window
                .update(&mut cx, |_, window, cx| {
                    window.dispatch_action(ToggleFocus.boxed_clone(), cx)
                })
                .unwrap();
        })
        .detach();
        cx.background_executor.run_until_parked();

        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    search_view.query_editor.update(cx, |query_editor, cx| {
                        query_editor.set_text("const FOUR", window, cx)
                    });
                    search_view.toggle_filters(cx);
                    search_view
                        .excluded_files_editor
                        .update(cx, |exclude_editor, cx| {
                            exclude_editor.set_text("four.rs", window, cx)
                        });
                    search_view.search(SearchMode::Manual, cx);
                });
            })
            .unwrap();
        cx.background_executor.run_until_parked();
        window
            .update(cx, |_, _, cx| {
                search_view.update(cx, |search_view, cx| {
                    let results_text = search_view
                        .results_editor
                        .update(cx, |editor, cx| editor.display_text(cx));
                    assert!(
                        results_text.is_empty(),
                        "Search view for query with the only match in an excluded file should have no results but got '{results_text}'"
                    );
                });
            }).unwrap();

        cx.spawn(|mut cx| async move {
            window.update(&mut cx, |_, window, cx| {
                window.dispatch_action(ToggleFocus.boxed_clone(), cx)
            })
        })
        .detach();
        cx.background_executor.run_until_parked();

        window
            .update(cx, |_, _, cx| {
                search_view.update(cx, |search_view, cx| {
                    search_view.toggle_filters(cx);
                    search_view.search(SearchMode::Manual, cx);
                });
            })
            .unwrap();
        cx.background_executor.run_until_parked();
        window
            .update(cx, |_, _, cx| {
                search_view.update(cx, |search_view, cx| {
                assert_eq!(
                    search_view
                        .results_editor
                        .update(cx, |editor, cx| editor.display_text(cx)),
                    "\n\nconst FOUR: usize = one::ONE + three::THREE;",
                    "Search view results should contain the queried result in the previously excluded file with filters toggled off"
                );
            });
            })
            .unwrap();
    }

    #[perf]
    #[gpui::test]
    async fn test_new_project_search_focus(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/dir"),
            json!({
                "one.rs": "const ONE: usize = 1;",
                "two.rs": "const TWO: usize = one::ONE + one::ONE;",
                "three.rs": "const THREE: usize = one::ONE + two::TWO;",
                "four.rs": "const FOUR: usize = one::ONE + three::THREE;",
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let window = cx.add_window(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let workspace = window
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let cx = &mut VisualTestContext::from_window(window.into(), cx);
        let search_bar = window.build_entity(cx, |_, _| ProjectSearchBar::new());

        let active_item = cx.read(|cx| {
            workspace
                .read(cx)
                .active_pane()
                .read(cx)
                .active_item()
                .and_then(|item| item.downcast::<ProjectSearchView>())
        });
        assert!(
            active_item.is_none(),
            "Expected no search panel to be active"
        );

        workspace.update_in(cx, move |workspace, window, cx| {
            assert_eq!(workspace.panes().len(), 1);
            workspace.panes()[0].update(cx, |pane, cx| {
                pane.toolbar()
                    .update(cx, |toolbar, cx| toolbar.add_item(search_bar, window, cx))
            });

            ProjectSearchView::new_search(workspace, &workspace::NewSearch, window, cx)
        });

        let Some(search_view) = cx.read(|cx| {
            workspace
                .read(cx)
                .active_pane()
                .read(cx)
                .active_item()
                .and_then(|item| item.downcast::<ProjectSearchView>())
        }) else {
            panic!("Search view expected to appear after new search event trigger")
        };

        cx.spawn(|mut cx| async move {
            window
                .update(&mut cx, |_, window, cx| {
                    window.dispatch_action(ToggleFocus.boxed_clone(), cx)
                })
                .unwrap();
        })
        .detach();
        cx.background_executor.run_until_parked();

        window.update(cx, |_, window, cx| {
            search_view.update(cx, |search_view, cx| {
                    assert!(
                        search_view.query_editor.focus_handle(cx).is_focused(window),
                        "Empty search view should be focused after the toggle focus event: no results panel to focus on",
                    );
                });
        }).unwrap();

        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    let query_editor = &search_view.query_editor;
                    assert!(
                        query_editor.focus_handle(cx).is_focused(window),
                        "Search view should be focused after the new search view is activated",
                    );
                    let query_text = query_editor.read(cx).text(cx);
                    assert!(
                        query_text.is_empty(),
                        "New search query should be empty but got '{query_text}'",
                    );
                    let results_text = search_view
                        .results_editor
                        .update(cx, |editor, cx| editor.display_text(cx));
                    assert!(
                        results_text.is_empty(),
                        "Empty search view should have no results but got '{results_text}'"
                    );
                });
            })
            .unwrap();

        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    search_view.query_editor.update(cx, |query_editor, cx| {
                        query_editor.set_text("sOMETHINGtHATsURELYdOESnOTeXIST", window, cx)
                    });
                    search_view.search(SearchMode::Manual, cx);
                });
            })
            .unwrap();

        cx.background_executor.run_until_parked();
        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    let results_text = search_view
                        .results_editor
                        .update(cx, |editor, cx| editor.display_text(cx));
                    assert!(
                results_text.is_empty(),
                "Search view for mismatching query should have no results but got '{results_text}'"
            );
                    assert!(
                search_view.query_editor.focus_handle(cx).is_focused(window),
                "Search view should be focused after mismatching query had been used in search",
            );
                });
            })
            .unwrap();
        cx.spawn(|mut cx| async move {
            window.update(&mut cx, |_, window, cx| {
                window.dispatch_action(ToggleFocus.boxed_clone(), cx)
            })
        })
        .detach();
        cx.background_executor.run_until_parked();
        window.update(cx, |_, window, cx| {
            search_view.update(cx, |search_view, cx| {
                    assert!(
                        search_view.query_editor.focus_handle(cx).is_focused(window),
                        "Search view with mismatching query should be focused after the toggle focus event: still no results panel to focus on",
                    );
                });
        }).unwrap();

        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    search_view.query_editor.update(cx, |query_editor, cx| {
                        query_editor.set_text("TWO", window, cx)
                    });
                    search_view.search(SearchMode::Manual, cx);
                })
            })
            .unwrap();
        cx.background_executor.run_until_parked();
        window.update(cx, |_, window, cx|
        search_view.update(cx, |search_view, cx| {
                assert_eq!(
                    search_view
                        .results_editor
                        .update(cx, |editor, cx| editor.display_text(cx)),
                    "\n\nconst THREE: usize = one::ONE + two::TWO;\n\n\nconst TWO: usize = one::ONE + one::ONE;",
                    "Search view results should match the query"
                );
                assert!(
                    search_view.results_editor.focus_handle(cx).is_focused(window),
                    "Search view with mismatching query should be focused after search results are available",
                );
            })).unwrap();
        cx.spawn(|mut cx| async move {
            window
                .update(&mut cx, |_, window, cx| {
                    window.dispatch_action(ToggleFocus.boxed_clone(), cx)
                })
                .unwrap();
        })
        .detach();
        cx.background_executor.run_until_parked();
        window.update(cx, |_, window, cx| {
            search_view.update(cx, |search_view, cx| {
                    assert!(
                        search_view.results_editor.focus_handle(cx).is_focused(window),
                        "Search view with matching query should still have its results editor focused after the toggle focus event",
                    );
                });
        }).unwrap();

        workspace.update_in(cx, |workspace, window, cx| {
            ProjectSearchView::new_search(workspace, &workspace::NewSearch, window, cx)
        });
        cx.background_executor.run_until_parked();
        let Some(search_view_2) = cx.read(|cx| {
            workspace
                .read(cx)
                .active_pane()
                .read(cx)
                .active_item()
                .and_then(|item| item.downcast::<ProjectSearchView>())
        }) else {
            panic!("Search view expected to appear after new search event trigger")
        };
        assert!(
            search_view_2 != search_view,
            "New search view should be open after `workspace::NewSearch` event"
        );

        window.update(cx, |_, window, cx| {
            search_view.update(cx, |search_view, cx| {
                    assert_eq!(search_view.query_editor.read(cx).text(cx), "TWO", "First search view should not have an updated query");
                    assert_eq!(
                        search_view
                            .results_editor
                            .update(cx, |editor, cx| editor.display_text(cx)),
                        "\n\nconst THREE: usize = one::ONE + two::TWO;\n\n\nconst TWO: usize = one::ONE + one::ONE;",
                        "Results of the first search view should not update too"
                    );
                    assert!(
                        !search_view.query_editor.focus_handle(cx).is_focused(window),
                        "Focus should be moved away from the first search view"
                    );
                });
        }).unwrap();

        window.update(cx, |_, window, cx| {
            search_view_2.update(cx, |search_view_2, cx| {
                    assert_eq!(
                        search_view_2.query_editor.read(cx).text(cx),
                        "two",
                        "New search view should get the query from the text cursor was at during the event spawn (first search view's first result)"
                    );
                    assert_eq!(
                        search_view_2
                            .results_editor
                            .update(cx, |editor, cx| editor.display_text(cx)),
                        "",
                        "No search results should be in the 2nd view yet, as we did not spawn a search for it"
                    );
                    assert!(
                        search_view_2.query_editor.focus_handle(cx).is_focused(window),
                        "Focus should be moved into query editor of the new window"
                    );
                });
        }).unwrap();

        window
            .update(cx, |_, window, cx| {
                search_view_2.update(cx, |search_view_2, cx| {
                    search_view_2.query_editor.update(cx, |query_editor, cx| {
                        query_editor.set_text("FOUR", window, cx)
                    });
                    search_view_2.search(SearchMode::Manual, cx);
                });
            })
            .unwrap();

        cx.background_executor.run_until_parked();
        window.update(cx, |_, window, cx| {
            search_view_2.update(cx, |search_view_2, cx| {
                    assert_eq!(
                        search_view_2
                            .results_editor
                            .update(cx, |editor, cx| editor.display_text(cx)),
                        "\n\nconst FOUR: usize = one::ONE + three::THREE;",
                        "New search view with the updated query should have new search results"
                    );
                    assert!(
                        search_view_2.results_editor.focus_handle(cx).is_focused(window),
                        "Search view with mismatching query should be focused after search results are available",
                    );
                });
        }).unwrap();

        cx.spawn(|mut cx| async move {
            window
                .update(&mut cx, |_, window, cx| {
                    window.dispatch_action(ToggleFocus.boxed_clone(), cx)
                })
                .unwrap();
        })
        .detach();
        cx.background_executor.run_until_parked();
        window.update(cx, |_, window, cx| {
            search_view_2.update(cx, |search_view_2, cx| {
                    assert!(
                        search_view_2.results_editor.focus_handle(cx).is_focused(window),
                        "Search view with matching query should switch focus to the results editor after the toggle focus event",
                    );
                });}).unwrap();
    }

    #[perf]
    #[gpui::test]
    async fn test_new_project_search_in_directory(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/dir"),
            json!({
                "a": {
                    "one.rs": "const ONE: usize = 1;",
                    "two.rs": "const TWO: usize = one::ONE + one::ONE;",
                },
                "b": {
                    "three.rs": "const THREE: usize = one::ONE + two::TWO;",
                    "four.rs": "const FOUR: usize = one::ONE + three::THREE;",
                },
            }),
        )
        .await;
        let project = Project::test(fs.clone(), ["/dir".as_ref()], cx).await;
        let worktree_id = project.read_with(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        let window = cx.add_window(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let workspace = window
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let cx = &mut VisualTestContext::from_window(window.into(), cx);
        let search_bar = window.build_entity(cx, |_, _| ProjectSearchBar::new());

        let active_item = cx.read(|cx| {
            workspace
                .read(cx)
                .active_pane()
                .read(cx)
                .active_item()
                .and_then(|item| item.downcast::<ProjectSearchView>())
        });
        assert!(
            active_item.is_none(),
            "Expected no search panel to be active"
        );

        workspace.update_in(cx, move |workspace, window, cx| {
            assert_eq!(workspace.panes().len(), 1);
            workspace.panes()[0].update(cx, move |pane, cx| {
                pane.toolbar()
                    .update(cx, |toolbar, cx| toolbar.add_item(search_bar, window, cx))
            });
        });

        let a_dir_entry = cx.update(|_, cx| {
            workspace
                .read(cx)
                .project()
                .read(cx)
                .entry_for_path(&(worktree_id, rel_path("a")).into(), cx)
                .expect("no entry for /a/ directory")
                .clone()
        });
        assert!(a_dir_entry.is_dir());
        let directory = cx.update(|_, cx| {
            a_dir_entry
                .path
                .display(workspace.read(cx).path_style(cx))
                .into_owned()
        });
        window
            .update(cx, |_, window, cx| {
                window.dispatch_action(
                    Box::new(zed_actions::search::NewSearchInDirectory { directory }),
                    cx,
                );
            })
            .unwrap();

        let Some(search_view) = cx.read(|cx| {
            workspace
                .read(cx)
                .active_pane()
                .read(cx)
                .active_item()
                .and_then(|item| item.downcast::<ProjectSearchView>())
        }) else {
            panic!("Search view expected to appear after new search in directory event trigger")
        };
        cx.background_executor.run_until_parked();
        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    assert!(
                        search_view.query_editor.focus_handle(cx).is_focused(window),
                        "On new search in directory, focus should be moved into query editor"
                    );
                    search_view.excluded_files_editor.update(cx, |editor, cx| {
                        assert!(
                            editor.display_text(cx).is_empty(),
                            "New search in directory should not have any excluded files"
                        );
                    });
                    search_view.included_files_editor.update(cx, |editor, cx| {
                        assert_eq!(
                            editor.display_text(cx),
                            a_dir_entry.path.display(PathStyle::local()),
                            "New search in directory should have included dir entry path"
                        );
                    });
                });
            })
            .unwrap();
        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    search_view.query_editor.update(cx, |query_editor, cx| {
                        query_editor.set_text("const", window, cx)
                    });
                    search_view.search(SearchMode::Manual, cx);
                });
            })
            .unwrap();
        cx.background_executor.run_until_parked();
        window
            .update(cx, |_, _, cx| {
                search_view.update(cx, |search_view, cx| {
                    assert_eq!(
                search_view
                    .results_editor
                    .update(cx, |editor, cx| editor.display_text(cx)),
                "\n\nconst ONE: usize = 1;\n\n\nconst TWO: usize = one::ONE + one::ONE;",
                "New search in directory should have a filter that matches a certain directory"
            );
                })
            })
            .unwrap();
    }

    #[perf]
    #[gpui::test]
    async fn test_search_query_history(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/dir"),
            json!({
                "one.rs": "const ONE: usize = 1;",
                "two.rs": "const TWO: usize = one::ONE + one::ONE;",
                "three.rs": "const THREE: usize = one::ONE + two::TWO;",
                "four.rs": "const FOUR: usize = one::ONE + three::THREE;",
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let window = cx.add_window(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let workspace = window
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let cx = &mut VisualTestContext::from_window(window.into(), cx);
        let search_bar = window.build_entity(cx, |_, _| ProjectSearchBar::new());

        workspace.update_in(cx, {
            let search_bar = search_bar.clone();
            |workspace, window, cx| {
                assert_eq!(workspace.panes().len(), 1);
                workspace.panes()[0].update(cx, |pane, cx| {
                    pane.toolbar()
                        .update(cx, |toolbar, cx| toolbar.add_item(search_bar, window, cx))
                });

                ProjectSearchView::new_search(workspace, &workspace::NewSearch, window, cx)
            }
        });

        let search_view = cx.read(|cx| {
            workspace
                .read(cx)
                .active_pane()
                .read(cx)
                .active_item()
                .and_then(|item| item.downcast::<ProjectSearchView>())
                .expect("Search view expected to appear after new search event trigger")
        });

        // Add 3 search items into the history + another unsubmitted one.
        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    search_view.search_options = SearchOptions::CASE_SENSITIVE;
                    search_view.query_editor.update(cx, |query_editor, cx| {
                        query_editor.set_text("ONE", window, cx)
                    });
                    search_view.search(SearchMode::Manual, cx);
                });
            })
            .unwrap();

        cx.background_executor.run_until_parked();
        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    search_view.query_editor.update(cx, |query_editor, cx| {
                        query_editor.set_text("TWO", window, cx)
                    });
                    search_view.search(SearchMode::Manual, cx);
                });
            })
            .unwrap();
        cx.background_executor.run_until_parked();
        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    search_view.query_editor.update(cx, |query_editor, cx| {
                        query_editor.set_text("THREE", window, cx)
                    });
                    search_view.search(SearchMode::Manual, cx);
                })
            })
            .unwrap();
        cx.background_executor.run_until_parked();
        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    search_view.query_editor.update(cx, |query_editor, cx| {
                        query_editor.set_text("JUST_TEXT_INPUT", window, cx)
                    });
                })
            })
            .unwrap();
        cx.background_executor.run_until_parked();

        // Ensure that the latest input with search settings is active.
        window
            .update(cx, |_, _, cx| {
                search_view.update(cx, |search_view, cx| {
                    assert_eq!(
                        search_view.query_editor.read(cx).text(cx),
                        "JUST_TEXT_INPUT"
                    );
                    assert_eq!(search_view.search_options, SearchOptions::CASE_SENSITIVE);
                });
            })
            .unwrap();

        // Next history query after the latest should preserve the current query.
        window
            .update(cx, |_, window, cx| {
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.focus_search(window, cx);
                    search_bar.next_history_query(&NextHistoryQuery, window, cx);
                })
            })
            .unwrap();
        window
            .update(cx, |_, _, cx| {
                search_view.update(cx, |search_view, cx| {
                    assert_eq!(
                        search_view.query_editor.read(cx).text(cx),
                        "JUST_TEXT_INPUT"
                    );
                    assert_eq!(search_view.search_options, SearchOptions::CASE_SENSITIVE);
                });
            })
            .unwrap();
        window
            .update(cx, |_, window, cx| {
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.focus_search(window, cx);
                    search_bar.next_history_query(&NextHistoryQuery, window, cx);
                })
            })
            .unwrap();
        window
            .update(cx, |_, _, cx| {
                search_view.update(cx, |search_view, cx| {
                    assert_eq!(
                        search_view.query_editor.read(cx).text(cx),
                        "JUST_TEXT_INPUT"
                    );
                    assert_eq!(search_view.search_options, SearchOptions::CASE_SENSITIVE);
                });
            })
            .unwrap();

        // Previous query should navigate backwards through history.
        window
            .update(cx, |_, window, cx| {
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.focus_search(window, cx);
                    search_bar.previous_history_query(&PreviousHistoryQuery, window, cx);
                });
            })
            .unwrap();
        window
            .update(cx, |_, _, cx| {
                search_view.update(cx, |search_view, cx| {
                    assert_eq!(search_view.query_editor.read(cx).text(cx), "TWO");
                    assert_eq!(search_view.search_options, SearchOptions::CASE_SENSITIVE);
                });
            })
            .unwrap();

        // Further previous items should go over the history in reverse order.
        window
            .update(cx, |_, window, cx| {
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.focus_search(window, cx);
                    search_bar.previous_history_query(&PreviousHistoryQuery, window, cx);
                });
            })
            .unwrap();
        window
            .update(cx, |_, _, cx| {
                search_view.update(cx, |search_view, cx| {
                    assert_eq!(search_view.query_editor.read(cx).text(cx), "ONE");
                    assert_eq!(search_view.search_options, SearchOptions::CASE_SENSITIVE);
                });
            })
            .unwrap();

        // Previous items should never go behind the first history item.
        window
            .update(cx, |_, window, cx| {
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.focus_search(window, cx);
                    search_bar.previous_history_query(&PreviousHistoryQuery, window, cx);
                });
            })
            .unwrap();
        window
            .update(cx, |_, _, cx| {
                search_view.update(cx, |search_view, cx| {
                    assert_eq!(search_view.query_editor.read(cx).text(cx), "ONE");
                    assert_eq!(search_view.search_options, SearchOptions::CASE_SENSITIVE);
                });
            })
            .unwrap();
        window
            .update(cx, |_, window, cx| {
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.focus_search(window, cx);
                    search_bar.previous_history_query(&PreviousHistoryQuery, window, cx);
                });
            })
            .unwrap();
        window
            .update(cx, |_, _, cx| {
                search_view.update(cx, |search_view, cx| {
                    assert_eq!(search_view.query_editor.read(cx).text(cx), "ONE");
                    assert_eq!(search_view.search_options, SearchOptions::CASE_SENSITIVE);
                });
            })
            .unwrap();

        // Next items should go over the history in the original order.
        window
            .update(cx, |_, window, cx| {
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.focus_search(window, cx);
                    search_bar.next_history_query(&NextHistoryQuery, window, cx);
                });
            })
            .unwrap();
        window
            .update(cx, |_, _, cx| {
                search_view.update(cx, |search_view, cx| {
                    assert_eq!(search_view.query_editor.read(cx).text(cx), "TWO");
                    assert_eq!(search_view.search_options, SearchOptions::CASE_SENSITIVE);
                });
            })
            .unwrap();

        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    search_view.query_editor.update(cx, |query_editor, cx| {
                        query_editor.set_text("TWO_NEW", window, cx)
                    });
                    search_view.search(SearchMode::Manual, cx);
                });
            })
            .unwrap();
        cx.background_executor.run_until_parked();
        window
            .update(cx, |_, _, cx| {
                search_view.update(cx, |search_view, cx| {
                    assert_eq!(search_view.query_editor.read(cx).text(cx), "TWO_NEW");
                    assert_eq!(search_view.search_options, SearchOptions::CASE_SENSITIVE);
                });
            })
            .unwrap();

        // New search input should add another entry to history and move the selection to the end of the history.
        window
            .update(cx, |_, window, cx| {
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.focus_search(window, cx);
                    search_bar.previous_history_query(&PreviousHistoryQuery, window, cx);
                });
            })
            .unwrap();
        window
            .update(cx, |_, _, cx| {
                search_view.update(cx, |search_view, cx| {
                    assert_eq!(search_view.query_editor.read(cx).text(cx), "THREE");
                    assert_eq!(search_view.search_options, SearchOptions::CASE_SENSITIVE);
                });
            })
            .unwrap();
        window
            .update(cx, |_, window, cx| {
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.focus_search(window, cx);
                    search_bar.previous_history_query(&PreviousHistoryQuery, window, cx);
                });
            })
            .unwrap();
        window
            .update(cx, |_, _, cx| {
                search_view.update(cx, |search_view, cx| {
                    assert_eq!(search_view.query_editor.read(cx).text(cx), "TWO");
                    assert_eq!(search_view.search_options, SearchOptions::CASE_SENSITIVE);
                });
            })
            .unwrap();
        window
            .update(cx, |_, window, cx| {
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.focus_search(window, cx);
                    search_bar.next_history_query(&NextHistoryQuery, window, cx);
                });
            })
            .unwrap();
        window
            .update(cx, |_, _, cx| {
                search_view.update(cx, |search_view, cx| {
                    assert_eq!(search_view.query_editor.read(cx).text(cx), "THREE");
                    assert_eq!(search_view.search_options, SearchOptions::CASE_SENSITIVE);
                });
            })
            .unwrap();
        window
            .update(cx, |_, window, cx| {
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.focus_search(window, cx);
                    search_bar.next_history_query(&NextHistoryQuery, window, cx);
                });
            })
            .unwrap();
        window
            .update(cx, |_, _, cx| {
                search_view.update(cx, |search_view, cx| {
                    assert_eq!(search_view.query_editor.read(cx).text(cx), "TWO_NEW");
                    assert_eq!(search_view.search_options, SearchOptions::CASE_SENSITIVE);
                });
            })
            .unwrap();
        window
            .update(cx, |_, window, cx| {
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.focus_search(window, cx);
                    search_bar.next_history_query(&NextHistoryQuery, window, cx);
                });
            })
            .unwrap();
        window
            .update(cx, |_, _, cx| {
                search_view.update(cx, |search_view, cx| {
                    assert_eq!(search_view.query_editor.read(cx).text(cx), "TWO_NEW");
                    assert_eq!(search_view.search_options, SearchOptions::CASE_SENSITIVE);
                });
            })
            .unwrap();

        // Typing text without running a search, then navigating history, should allow
        // restoring the draft when pressing next past the end.
        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    search_view.query_editor.update(cx, |query_editor, cx| {
                        query_editor.set_text("unsaved draft", window, cx)
                    });
                })
            })
            .unwrap();
        cx.background_executor.run_until_parked();

        // Navigate up into history — the draft should be stashed.
        window
            .update(cx, |_, window, cx| {
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.focus_search(window, cx);
                    search_bar.previous_history_query(&PreviousHistoryQuery, window, cx);
                });
            })
            .unwrap();
        window
            .update(cx, |_, _, cx| {
                search_view.update(cx, |search_view, cx| {
                    assert_eq!(search_view.query_editor.read(cx).text(cx), "THREE");
                });
            })
            .unwrap();

        // Navigate forward through history.
        window
            .update(cx, |_, window, cx| {
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.focus_search(window, cx);
                    search_bar.next_history_query(&NextHistoryQuery, window, cx);
                });
            })
            .unwrap();
        window
            .update(cx, |_, _, cx| {
                search_view.update(cx, |search_view, cx| {
                    assert_eq!(search_view.query_editor.read(cx).text(cx), "TWO_NEW");
                });
            })
            .unwrap();

        // Navigate past the end — the draft should be restored.
        window
            .update(cx, |_, window, cx| {
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.focus_search(window, cx);
                    search_bar.next_history_query(&NextHistoryQuery, window, cx);
                });
            })
            .unwrap();
        window
            .update(cx, |_, _, cx| {
                search_view.update(cx, |search_view, cx| {
                    assert_eq!(search_view.query_editor.read(cx).text(cx), "unsaved draft");
                });
            })
            .unwrap();
    }

    #[perf]
    #[gpui::test]
    async fn test_search_query_history_with_multiple_views(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/dir"),
            json!({
                "one.rs": "const ONE: usize = 1;",
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let worktree_id = project.update(cx, |this, cx| {
            this.worktrees(cx).next().unwrap().read(cx).id()
        });

        let window = cx.add_window(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let workspace = window
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let cx = &mut VisualTestContext::from_window(window.into(), cx);

        let panes: Vec<_> = workspace.update_in(cx, |this, _, _| this.panes().to_owned());

        let search_bar_1 = window.build_entity(cx, |_, _| ProjectSearchBar::new());
        let search_bar_2 = window.build_entity(cx, |_, _| ProjectSearchBar::new());

        assert_eq!(panes.len(), 1);
        let first_pane = panes.first().cloned().unwrap();
        assert_eq!(cx.update(|_, cx| first_pane.read(cx).items_len()), 0);
        workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.open_path(
                    (worktree_id, rel_path("one.rs")),
                    Some(first_pane.downgrade()),
                    true,
                    window,
                    cx,
                )
            })
            .await
            .unwrap();
        assert_eq!(cx.update(|_, cx| first_pane.read(cx).items_len()), 1);

        // Add a project search item to the first pane
        workspace.update_in(cx, {
            let search_bar = search_bar_1.clone();
            |workspace, window, cx| {
                first_pane.update(cx, |pane, cx| {
                    pane.toolbar()
                        .update(cx, |toolbar, cx| toolbar.add_item(search_bar, window, cx))
                });

                ProjectSearchView::new_search(workspace, &workspace::NewSearch, window, cx)
            }
        });
        let search_view_1 = cx.read(|cx| {
            workspace
                .read(cx)
                .active_item(cx)
                .and_then(|item| item.downcast::<ProjectSearchView>())
                .expect("Search view expected to appear after new search event trigger")
        });

        let second_pane = workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.split_and_clone(
                    first_pane.clone(),
                    workspace::SplitDirection::Right,
                    window,
                    cx,
                )
            })
            .await
            .unwrap();
        assert_eq!(cx.update(|_, cx| second_pane.read(cx).items_len()), 1);

        assert_eq!(cx.update(|_, cx| second_pane.read(cx).items_len()), 1);
        assert_eq!(cx.update(|_, cx| first_pane.read(cx).items_len()), 2);

        // Add a project search item to the second pane
        workspace.update_in(cx, {
            let search_bar = search_bar_2.clone();
            let pane = second_pane.clone();
            move |workspace, window, cx| {
                assert_eq!(workspace.panes().len(), 2);
                pane.update(cx, |pane, cx| {
                    pane.toolbar()
                        .update(cx, |toolbar, cx| toolbar.add_item(search_bar, window, cx))
                });

                ProjectSearchView::new_search(workspace, &workspace::NewSearch, window, cx)
            }
        });

        let search_view_2 = cx.read(|cx| {
            workspace
                .read(cx)
                .active_item(cx)
                .and_then(|item| item.downcast::<ProjectSearchView>())
                .expect("Search view expected to appear after new search event trigger")
        });

        cx.run_until_parked();
        assert_eq!(cx.update(|_, cx| first_pane.read(cx).items_len()), 2);
        assert_eq!(cx.update(|_, cx| second_pane.read(cx).items_len()), 2);

        let update_search_view =
            |search_view: &Entity<ProjectSearchView>, query: &str, cx: &mut TestAppContext| {
                window
                    .update(cx, |_, window, cx| {
                        search_view.update(cx, |search_view, cx| {
                            search_view.query_editor.update(cx, |query_editor, cx| {
                                query_editor.set_text(query, window, cx)
                            });
                            search_view.search(SearchMode::Manual, cx);
                        });
                    })
                    .unwrap();
            };

        let active_query =
            |search_view: &Entity<ProjectSearchView>, cx: &mut TestAppContext| -> String {
                window
                    .update(cx, |_, _, cx| {
                        search_view.update(cx, |search_view, cx| {
                            search_view.query_editor.read(cx).text(cx)
                        })
                    })
                    .unwrap()
            };

        let select_prev_history_item =
            |search_bar: &Entity<ProjectSearchBar>, cx: &mut TestAppContext| {
                window
                    .update(cx, |_, window, cx| {
                        search_bar.update(cx, |search_bar, cx| {
                            search_bar.focus_search(window, cx);
                            search_bar.previous_history_query(&PreviousHistoryQuery, window, cx);
                        })
                    })
                    .unwrap();
            };

        let select_next_history_item =
            |search_bar: &Entity<ProjectSearchBar>, cx: &mut TestAppContext| {
                window
                    .update(cx, |_, window, cx| {
                        search_bar.update(cx, |search_bar, cx| {
                            search_bar.focus_search(window, cx);
                            search_bar.next_history_query(&NextHistoryQuery, window, cx);
                        })
                    })
                    .unwrap();
            };

        update_search_view(&search_view_1, "ONE", cx);
        cx.background_executor.run_until_parked();

        update_search_view(&search_view_2, "TWO", cx);
        cx.background_executor.run_until_parked();

        assert_eq!(active_query(&search_view_1, cx), "ONE");
        assert_eq!(active_query(&search_view_2, cx), "TWO");

        // Selecting previous history item should select the query from search view 1.
        select_prev_history_item(&search_bar_2, cx);
        assert_eq!(active_query(&search_view_2, cx), "ONE");

        // Selecting the previous history item should not change the query as it is already the first item.
        select_prev_history_item(&search_bar_2, cx);
        assert_eq!(active_query(&search_view_2, cx), "ONE");

        // Changing the query in search view 2 should not affect the history of search view 1.
        assert_eq!(active_query(&search_view_1, cx), "ONE");

        // Deploying a new search in search view 2
        update_search_view(&search_view_2, "THREE", cx);
        cx.background_executor.run_until_parked();

        select_next_history_item(&search_bar_2, cx);
        assert_eq!(active_query(&search_view_2, cx), "THREE");

        select_prev_history_item(&search_bar_2, cx);
        assert_eq!(active_query(&search_view_2, cx), "TWO");

        select_prev_history_item(&search_bar_2, cx);
        assert_eq!(active_query(&search_view_2, cx), "ONE");

        select_prev_history_item(&search_bar_2, cx);
        assert_eq!(active_query(&search_view_2, cx), "ONE");

        select_prev_history_item(&search_bar_2, cx);
        assert_eq!(active_query(&search_view_2, cx), "ONE");

        // Search view 1 should now see the query from search view 2.
        assert_eq!(active_query(&search_view_1, cx), "ONE");

        select_next_history_item(&search_bar_2, cx);
        assert_eq!(active_query(&search_view_2, cx), "TWO");

        // Here is the new query from search view 2
        select_next_history_item(&search_bar_2, cx);
        assert_eq!(active_query(&search_view_2, cx), "THREE");

        select_next_history_item(&search_bar_2, cx);
        assert_eq!(active_query(&search_view_2, cx), "THREE");

        select_next_history_item(&search_bar_1, cx);
        assert_eq!(active_query(&search_view_1, cx), "TWO");

        select_next_history_item(&search_bar_1, cx);
        assert_eq!(active_query(&search_view_1, cx), "THREE");

        select_next_history_item(&search_bar_1, cx);
        assert_eq!(active_query(&search_view_1, cx), "THREE");
    }

    #[perf]
    #[gpui::test]
    async fn test_deploy_search_with_multiple_panes(cx: &mut TestAppContext) {
        init_test(cx);

        // Setup 2 panes, both with a file open and one with a project search.
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/dir"),
            json!({
                "one.rs": "const ONE: usize = 1;",
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let worktree_id = project.update(cx, |this, cx| {
            this.worktrees(cx).next().unwrap().read(cx).id()
        });
        let window = cx.add_window(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let workspace = window
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let cx = &mut VisualTestContext::from_window(window.into(), cx);
        let panes: Vec<_> = workspace.update_in(cx, |this, _, _| this.panes().to_owned());
        assert_eq!(panes.len(), 1);
        let first_pane = panes.first().cloned().unwrap();
        assert_eq!(cx.update(|_, cx| first_pane.read(cx).items_len()), 0);
        workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.open_path(
                    (worktree_id, rel_path("one.rs")),
                    Some(first_pane.downgrade()),
                    true,
                    window,
                    cx,
                )
            })
            .await
            .unwrap();
        assert_eq!(cx.update(|_, cx| first_pane.read(cx).items_len()), 1);
        let second_pane = workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.split_and_clone(
                    first_pane.clone(),
                    workspace::SplitDirection::Right,
                    window,
                    cx,
                )
            })
            .await
            .unwrap();
        assert_eq!(cx.update(|_, cx| second_pane.read(cx).items_len()), 1);
        assert!(
            window
                .update(cx, |_, window, cx| second_pane
                    .focus_handle(cx)
                    .contains_focused(window, cx))
                .unwrap()
        );
        let search_bar = window.build_entity(cx, |_, _| ProjectSearchBar::new());
        workspace.update_in(cx, {
            let search_bar = search_bar.clone();
            let pane = first_pane.clone();
            move |workspace, window, cx| {
                assert_eq!(workspace.panes().len(), 2);
                pane.update(cx, move |pane, cx| {
                    pane.toolbar()
                        .update(cx, |toolbar, cx| toolbar.add_item(search_bar, window, cx))
                });
            }
        });

        // Add a project search item to the second pane
        workspace.update_in(cx, {
            |workspace, window, cx| {
                assert_eq!(workspace.panes().len(), 2);
                second_pane.update(cx, |pane, cx| {
                    pane.toolbar()
                        .update(cx, |toolbar, cx| toolbar.add_item(search_bar, window, cx))
                });

                ProjectSearchView::new_search(workspace, &workspace::NewSearch, window, cx)
            }
        });

        cx.run_until_parked();
        assert_eq!(cx.update(|_, cx| second_pane.read(cx).items_len()), 2);
        assert_eq!(cx.update(|_, cx| first_pane.read(cx).items_len()), 1);

        // Focus the first pane
        workspace.update_in(cx, |workspace, window, cx| {
            assert_eq!(workspace.active_pane(), &second_pane);
            second_pane.update(cx, |this, cx| {
                assert_eq!(this.active_item_index(), 1);
                this.activate_previous_item(&Default::default(), window, cx);
                assert_eq!(this.active_item_index(), 0);
            });
            workspace.activate_pane_in_direction(workspace::SplitDirection::Left, window, cx);
        });
        workspace.update_in(cx, |workspace, _, cx| {
            assert_eq!(workspace.active_pane(), &first_pane);
            assert_eq!(first_pane.read(cx).items_len(), 1);
            assert_eq!(second_pane.read(cx).items_len(), 2);
        });

        // Deploy a new search
        cx.dispatch_action(DeploySearch::default());

        // Both panes should now have a project search in them
        workspace.update_in(cx, |workspace, window, cx| {
            assert_eq!(workspace.active_pane(), &first_pane);
            first_pane.read_with(cx, |this, _| {
                assert_eq!(this.active_item_index(), 1);
                assert_eq!(this.items_len(), 2);
            });
            second_pane.update(cx, |this, cx| {
                assert!(!cx.focus_handle().contains_focused(window, cx));
                assert_eq!(this.items_len(), 2);
            });
        });

        // Focus the second pane's non-search item
        window
            .update(cx, |_workspace, window, cx| {
                second_pane.update(cx, |pane, cx| {
                    pane.activate_next_item(&Default::default(), window, cx)
                });
            })
            .unwrap();

        // Deploy a new search
        cx.dispatch_action(DeploySearch::default());

        // The project search view should now be focused in the second pane
        // And the number of items should be unchanged.
        window
            .update(cx, |_workspace, _, cx| {
                second_pane.update(cx, |pane, _cx| {
                    assert!(
                        pane.active_item()
                            .unwrap()
                            .downcast::<ProjectSearchView>()
                            .is_some()
                    );

                    assert_eq!(pane.items_len(), 2);
                });
            })
            .unwrap();
    }

    #[perf]
    #[gpui::test]
    async fn test_scroll_search_results_to_top(cx: &mut TestAppContext) {
        init_test(cx);

        // We need many lines in the search results to be able to scroll the window
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/dir"),
            json!({
                "1.txt": "\n\n\n\n\n A \n\n\n\n\n",
                "2.txt": "\n\n\n\n\n A \n\n\n\n\n",
                "3.rs": "\n\n\n\n\n A \n\n\n\n\n",
                "4.rs": "\n\n\n\n\n A \n\n\n\n\n",
                "5.rs": "\n\n\n\n\n A \n\n\n\n\n",
                "6.rs": "\n\n\n\n\n A \n\n\n\n\n",
                "7.rs": "\n\n\n\n\n A \n\n\n\n\n",
                "8.rs": "\n\n\n\n\n A \n\n\n\n\n",
                "9.rs": "\n\n\n\n\n A \n\n\n\n\n",
                "a.rs": "\n\n\n\n\n A \n\n\n\n\n",
                "b.rs": "\n\n\n\n\n B \n\n\n\n\n",
                "c.rs": "\n\n\n\n\n B \n\n\n\n\n",
                "d.rs": "\n\n\n\n\n B \n\n\n\n\n",
                "e.rs": "\n\n\n\n\n B \n\n\n\n\n",
                "f.rs": "\n\n\n\n\n B \n\n\n\n\n",
                "g.rs": "\n\n\n\n\n B \n\n\n\n\n",
                "h.rs": "\n\n\n\n\n B \n\n\n\n\n",
                "i.rs": "\n\n\n\n\n B \n\n\n\n\n",
                "j.rs": "\n\n\n\n\n B \n\n\n\n\n",
                "k.rs": "\n\n\n\n\n B \n\n\n\n\n",
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let window =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let search = cx.new(|cx| ProjectSearch::new(project, workspace.downgrade(), cx));
        let search_view = cx.add_window(|window, cx| {
            ProjectSearchView::new(workspace.downgrade(), search.clone(), window, cx, None)
        });

        // First search
        perform_search(search_view, "A", cx);
        search_view
            .update(cx, |search_view, window, cx| {
                search_view.results_editor.update(cx, |results_editor, cx| {
                    // Results are correct and scrolled to the top
                    assert_eq!(
                        results_editor.display_text(cx).match_indices(" A ").count(),
                        10
                    );
                    assert_eq!(results_editor.scroll_position(cx), Point::default());

                    // Scroll results all the way down
                    results_editor.scroll(Point::new(0., f64::MAX), window, cx);
                });
            })
            .expect("unable to update search view");

        // Second search
        perform_search(search_view, "B", cx);
        search_view
            .update(cx, |search_view, _, cx| {
                search_view.results_editor.update(cx, |results_editor, cx| {
                    // Results are correct...
                    assert_eq!(
                        results_editor.display_text(cx).match_indices(" B ").count(),
                        10
                    );
                    // ...and scrolled back to the top
                    assert_eq!(results_editor.scroll_position(cx), Point::default());
                });
            })
            .expect("unable to update search view");
    }

    #[gpui::test]
    async fn test_seeded_project_search_query_is_escaped_in_regex_mode(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.editor.seed_search_query_from_cursor =
                        Some(SeedQuerySetting::Selection);
                    settings.editor.search = Some(SearchSettingsContent {
                        regex: Some(true),
                        ..Default::default()
                    });
                });
            });
        });

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/dir"),
            json!({
                "one.rs": "z.d\nzed\n",
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project
                .worktrees(cx)
                .next()
                .expect("project should have a worktree")
                .read(cx)
                .id()
        });
        let window =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window
            .read_with(cx, |multi_workspace, _| multi_workspace.workspace().clone())
            .expect("window should contain a workspace");
        let mut cx = VisualTestContext::from_window(window.into(), cx);

        let editor = workspace
            .update_in(&mut cx, |workspace, window, cx| {
                workspace.open_path((worktree_id, rel_path("one.rs")), None, true, window, cx)
            })
            .await
            .expect("should open test file")
            .downcast::<Editor>()
            .expect("opened item should be an editor");
        cx.run_until_parked();

        editor.update_in(&mut cx, |editor, window, cx| {
            editor.change_selections(SelectionEffects::no_scroll(), window, cx, |selections| {
                selections.select_ranges([BufferPoint::new(0, 0)..BufferPoint::new(0, 3)])
            });
        });

        workspace.update_in(&mut cx, |workspace, window, cx| {
            ProjectSearchView::deploy_search(workspace, &DeploySearch::default(), window, cx)
        });
        cx.run_until_parked();

        let project_search_view = workspace
            .read_with(&cx, |workspace, cx| {
                workspace
                    .active_pane()
                    .read(cx)
                    .active_item()
                    .and_then(|item| item.downcast::<ProjectSearchView>())
            })
            .expect("should open a project search view");
        project_search_view.update(&mut cx, |search_view, cx| {
            assert_eq!(search_view.search_query_text(cx), r"z\.d");
            search_view.search(SearchMode::Manual, cx);
        });
        cx.run_until_parked();

        project_search_view.update(&mut cx, |search_view, cx| {
            assert_eq!(search_view.entity.read(cx).match_ranges.len(), 1);
        });

        workspace.update_in(&mut cx, |workspace, window, cx| {
            ProjectSearchView::deploy_search(
                workspace,
                &DeploySearch {
                    query: Some("z.d".into()),
                    regex: Some(true),
                    ..Default::default()
                },
                window,
                cx,
            )
        });
        project_search_view.update(&mut cx, |search_view, cx| {
            assert_eq!(search_view.search_query_text(cx), "z.d");
            search_view.search(SearchMode::Manual, cx);
        });
        cx.run_until_parked();

        project_search_view.update(&mut cx, |search_view, cx| {
            assert_eq!(search_view.entity.read(cx).match_ranges.len(), 2);
        });
    }

    #[perf]
    #[gpui::test]
    async fn test_buffer_search_query_reused(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/dir"),
            json!({
                "one.rs": "const ONE: usize = 1;",
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let worktree_id = project.update(cx, |this, cx| {
            this.worktrees(cx).next().unwrap().read(cx).id()
        });
        let window =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let mut cx = VisualTestContext::from_window(window.into(), cx);

        let editor = workspace
            .update_in(&mut cx, |workspace, window, cx| {
                workspace.open_path((worktree_id, rel_path("one.rs")), None, true, window, cx)
            })
            .await
            .unwrap()
            .downcast::<Editor>()
            .unwrap();

        // Wait for the unstaged changes to be loaded
        cx.run_until_parked();

        let buffer_search_bar = cx.new_window_entity(|window, cx| {
            let mut search_bar =
                BufferSearchBar::new(Some(project.read(cx).languages().clone()), window, cx);
            search_bar.set_active_pane_item(Some(&editor), window, cx);
            search_bar.show(window, cx);
            search_bar
        });

        let panes: Vec<_> = workspace.update_in(&mut cx, |this, _, _| this.panes().to_owned());
        assert_eq!(panes.len(), 1);
        let pane = panes.first().cloned().unwrap();
        pane.update_in(&mut cx, |pane, window, cx| {
            pane.toolbar().update(cx, |toolbar, cx| {
                toolbar.add_item(buffer_search_bar.clone(), window, cx);
            })
        });

        let buffer_search_query = "search bar query";
        buffer_search_bar
            .update_in(&mut cx, |buffer_search_bar, window, cx| {
                buffer_search_bar.focus_handle(cx).focus(window, cx);
                buffer_search_bar.search(buffer_search_query, None, true, window, cx)
            })
            .await
            .unwrap();

        workspace.update_in(&mut cx, |workspace, window, cx| {
            ProjectSearchView::new_search(workspace, &workspace::NewSearch, window, cx)
        });
        cx.run_until_parked();
        let project_search_view = pane
            .read_with(&cx, |pane, _| {
                pane.active_item()
                    .and_then(|item| item.downcast::<ProjectSearchView>())
            })
            .expect("should open a project search view after spawning a new search");
        project_search_view.update(&mut cx, |search_view, cx| {
            assert_eq!(
                search_view.search_query_text(cx),
                buffer_search_query,
                "Project search should take the query from the buffer search bar since it got focused and had a query inside"
            );
        });
    }

    #[gpui::test]
    async fn test_search_dismisses_modal(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/dir"),
            json!({
                "one.rs": "const ONE: usize = 1;",
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let window =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let cx = &mut VisualTestContext::from_window(window.into(), cx);

        struct EmptyModalView {
            focus_handle: gpui::FocusHandle,
        }
        impl EventEmitter<gpui::DismissEvent> for EmptyModalView {}
        impl Render for EmptyModalView {
            fn render(&mut self, _: &mut Window, _: &mut Context<'_, Self>) -> impl IntoElement {
                div()
            }
        }
        impl Focusable for EmptyModalView {
            fn focus_handle(&self, _cx: &App) -> gpui::FocusHandle {
                self.focus_handle.clone()
            }
        }
        impl workspace::ModalView for EmptyModalView {}

        workspace.update_in(cx, |workspace, window, cx| {
            workspace.toggle_modal(window, cx, |_, cx| EmptyModalView {
                focus_handle: cx.focus_handle(),
            });
            assert!(workspace.has_active_modal(window, cx));
        });

        cx.dispatch_action(Deploy::find());

        workspace.update_in(cx, |workspace, window, cx| {
            assert!(!workspace.has_active_modal(window, cx));
            workspace.toggle_modal(window, cx, |_, cx| EmptyModalView {
                focus_handle: cx.focus_handle(),
            });
            assert!(workspace.has_active_modal(window, cx));
        });

        cx.dispatch_action(DeploySearch::default());

        workspace.update_in(cx, |workspace, window, cx| {
            assert!(!workspace.has_active_modal(window, cx));
        });
    }

    #[perf]
    #[gpui::test]
    async fn test_search_with_inlays(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.project.all_languages.defaults.inlay_hints =
                        Some(InlayHintSettingsContent {
                            enabled: Some(true),
                            ..InlayHintSettingsContent::default()
                        })
                });
            });
        });

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/dir"),
            // `\n` , a trailing line on the end, is important for the test case
            json!({
                "main.rs": "fn main() { let a = 2; }\n",
            }),
        )
        .await;

        let requests_count = Arc::new(AtomicUsize::new(0));
        let closure_requests_count = requests_count.clone();
        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        let language = rust_lang();
        language_registry.add(language);
        let mut fake_servers = language_registry.register_fake_lsp(
            "Rust",
            FakeLspAdapter {
                capabilities: lsp::ServerCapabilities {
                    inlay_hint_provider: Some(lsp::OneOf::Left(true)),
                    ..lsp::ServerCapabilities::default()
                },
                initializer: Some(Box::new(move |fake_server| {
                    let requests_count = closure_requests_count.clone();
                    fake_server.set_request_handler::<lsp::request::InlayHintRequest, _, _>({
                        move |_, _| {
                            let requests_count = requests_count.clone();
                            async move {
                                requests_count.fetch_add(1, atomic::Ordering::Release);
                                Ok(Some(vec![lsp::InlayHint {
                                    position: lsp::Position::new(0, 17),
                                    label: lsp::InlayHintLabel::String(": i32".to_owned()),
                                    kind: Some(lsp::InlayHintKind::TYPE),
                                    text_edits: None,
                                    tooltip: None,
                                    padding_left: None,
                                    padding_right: None,
                                    data: None,
                                }]))
                            }
                        }
                    });
                })),
                ..FakeLspAdapter::default()
            },
        );

        let window =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let cx = &mut VisualTestContext::from_window(window.into(), cx);
        let search = cx.new(|cx| ProjectSearch::new(project.clone(), workspace.downgrade(), cx));
        let search_view = cx.add_window(|window, cx| {
            ProjectSearchView::new(workspace.downgrade(), search.clone(), window, cx, None)
        });

        perform_search(search_view, "let ", cx);
        let fake_server = fake_servers.next().await.unwrap();
        cx.executor().advance_clock(Duration::from_secs(1));
        cx.executor().run_until_parked();
        search_view
            .update(cx, |search_view, _, cx| {
                assert_eq!(
                    search_view
                        .results_editor
                        .update(cx, |editor, cx| editor.display_text(cx)),
                    "\n\nfn main() { let a: i32 = 2; }\n"
                );
            })
            .unwrap();
        assert_eq!(
            requests_count.load(atomic::Ordering::Acquire),
            1,
            "New hints should have been queried",
        );

        // Can do the 2nd search without any panics
        perform_search(search_view, "let ", cx);
        cx.executor().advance_clock(Duration::from_secs(1));
        cx.executor().run_until_parked();
        search_view
            .update(cx, |search_view, _, cx| {
                assert_eq!(
                    search_view
                        .results_editor
                        .update(cx, |editor, cx| editor.display_text(cx)),
                    "\n\nfn main() { let a: i32 = 2; }\n"
                );
            })
            .unwrap();
        assert_eq!(
            requests_count.load(atomic::Ordering::Acquire),
            1,
            "Re-searching the same query reuses the excerpts and their buffer, so the cached hints stay valid",
        );

        let singleton_editor = workspace
            .update_in(cx, |workspace, window, cx| {
                workspace.open_abs_path(
                    PathBuf::from(path!("/dir/main.rs")),
                    workspace::OpenOptions::default(),
                    window,
                    cx,
                )
            })
            .await
            .unwrap()
            .downcast::<Editor>()
            .unwrap();
        cx.executor().advance_clock(Duration::from_millis(100));
        cx.executor().run_until_parked();
        singleton_editor.update(cx, |editor, cx| {
            assert_eq!(
                editor.display_text(cx),
                "fn main() { let a: i32 = 2; }\n",
                "Newly opened editor should have the correct text with hints",
            );
        });
        assert_eq!(
            requests_count.load(atomic::Ordering::Acquire),
            1,
            "Opening the same buffer again should reuse the cached hints",
        );

        window
            .update(cx, |_, window, cx| {
                singleton_editor.update(cx, |editor, cx| {
                    editor.handle_input("test", window, cx);
                });
            })
            .unwrap();

        cx.executor().advance_clock(Duration::from_secs(1));
        cx.executor().run_until_parked();
        singleton_editor.update(cx, |editor, cx| {
            assert_eq!(
                editor.display_text(cx),
                "testfn main() { l: i32et a = 2; }\n",
                "Newly opened editor should have the correct text with hints",
            );
        });
        assert_eq!(
            requests_count.load(atomic::Ordering::Acquire),
            2,
            "We have edited the buffer and should send a new request",
        );

        window
            .update(cx, |_, window, cx| {
                singleton_editor.update(cx, |editor, cx| {
                    editor.undo(&editor::actions::Undo, window, cx);
                });
            })
            .unwrap();
        cx.executor().advance_clock(Duration::from_secs(1));
        cx.executor().run_until_parked();
        assert_eq!(
            requests_count.load(atomic::Ordering::Acquire),
            3,
            "We have edited the buffer again and should send a new request again",
        );
        singleton_editor.update(cx, |editor, cx| {
            assert_eq!(
                editor.display_text(cx),
                "fn main() { let a: i32 = 2; }\n",
                "Newly opened editor should have the correct text with hints",
            );
        });
        fake_server
            .request::<lsp::request::InlayHintRefreshRequest>((), lsp::DEFAULT_LSP_REQUEST_TIMEOUT)
            .await
            .into_response()
            .unwrap();
        cx.executor().advance_clock(Duration::from_secs(1));
        cx.executor().run_until_parked();
        assert_eq!(
            requests_count.load(atomic::Ordering::Acquire),
            4,
            "After a server refresh request, we should have sent another request",
        );

        perform_search(search_view, "let ", cx);
        cx.executor().advance_clock(Duration::from_secs(1));
        cx.executor().run_until_parked();
        assert_eq!(
            requests_count.load(atomic::Ordering::Acquire),
            4,
            "New project search should reuse the cached hints",
        );
        search_view
            .update(cx, |search_view, _, cx| {
                assert_eq!(
                    search_view
                        .results_editor
                        .update(cx, |editor, cx| editor.display_text(cx)),
                    "\n\nfn main() { let a: i32 = 2; }\n"
                );
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_deleted_file_removed_from_search_results(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/dir"),
            json!({
                "file_a.txt": "hello world",
                "file_b.txt": "hello universe",
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let window =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let search = cx.new(|cx| ProjectSearch::new(project.clone(), workspace.downgrade(), cx));
        let search_view = cx.add_window(|window, cx| {
            ProjectSearchView::new(workspace.downgrade(), search.clone(), window, cx, None)
        });

        perform_search(search_view, "hello", cx);

        search_view
            .update(cx, |search_view, _window, cx| {
                let match_count = search_view.entity.read(cx).match_ranges.len();
                assert_eq!(match_count, 2, "Should have matches from both files");
            })
            .unwrap();

        // Delete file_b.txt
        fs.remove_file(
            path!("/dir/file_b.txt").as_ref(),
            fs::RemoveOptions::default(),
        )
        .await
        .unwrap();
        cx.run_until_parked();

        // Verify deleted file's results are removed proactively
        search_view
            .update(cx, |search_view, _window, cx| {
                let results_text = search_view
                    .results_editor
                    .update(cx, |editor, cx| editor.display_text(cx));
                assert!(
                    !results_text.contains("universe"),
                    "Deleted file's content should be removed from results, got: {results_text}"
                );
                assert!(
                    results_text.contains("world"),
                    "Remaining file's content should still be present, got: {results_text}"
                );
            })
            .unwrap();

        // Re-run the search and verify deleted file stays gone
        perform_search(search_view, "hello", cx);

        search_view
            .update(cx, |search_view, _window, cx| {
                let results_text = search_view
                    .results_editor
                    .update(cx, |editor, cx| editor.display_text(cx));
                assert!(
                    !results_text.contains("universe"),
                    "Deleted file should not reappear after re-search, got: {results_text}"
                );
                assert!(
                    results_text.contains("world"),
                    "Remaining file should still be found, got: {results_text}"
                );
                assert_eq!(
                    search_view.entity.read(cx).match_ranges.len(),
                    1,
                    "Should only have match from the remaining file"
                );
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_deploy_search_applies_and_resets_options(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/dir"),
            json!({
                "one.rs": "const ONE: usize = 1;",
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let window = cx.add_window(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let workspace = window
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let cx = &mut VisualTestContext::from_window(window.into(), cx);
        let search_bar = window.build_entity(cx, |_, _| ProjectSearchBar::new());

        workspace.update_in(cx, |workspace, window, cx| {
            workspace.panes()[0].update(cx, |pane, cx| {
                pane.toolbar()
                    .update(cx, |toolbar, cx| toolbar.add_item(search_bar, window, cx))
            });

            ProjectSearchView::deploy_search(
                workspace,
                &workspace::DeploySearch {
                    regex: Some(true),
                    case_sensitive: Some(true),
                    whole_word: Some(true),
                    include_ignored: Some(true),
                    query: Some("Test_Query".into()),
                    ..Default::default()
                },
                window,
                cx,
            )
        });

        let search_view = cx
            .read(|cx| {
                workspace
                    .read(cx)
                    .active_pane()
                    .read(cx)
                    .active_item()
                    .and_then(|item| item.downcast::<ProjectSearchView>())
            })
            .expect("Search view should be active after deploy");

        search_view.update_in(cx, |search_view, _window, cx| {
            assert!(
                search_view.search_options.contains(SearchOptions::REGEX),
                "Regex option should be enabled"
            );
            assert!(
                search_view
                    .search_options
                    .contains(SearchOptions::CASE_SENSITIVE),
                "Case sensitive option should be enabled"
            );
            assert!(
                search_view
                    .search_options
                    .contains(SearchOptions::WHOLE_WORD),
                "Whole word option should be enabled"
            );
            assert!(
                search_view
                    .search_options
                    .contains(SearchOptions::INCLUDE_IGNORED),
                "Include ignored option should be enabled"
            );
            let query_text = search_view.query_editor.read(cx).text(cx);
            assert_eq!(
                query_text, "Test_Query",
                "Query should be set from the action"
            );
        });

        // Redeploy with only regex - unspecified options should be preserved.
        cx.dispatch_action(menu::Cancel);
        workspace.update_in(cx, |workspace, window, cx| {
            ProjectSearchView::deploy_search(
                workspace,
                &workspace::DeploySearch {
                    regex: Some(true),
                    ..Default::default()
                },
                window,
                cx,
            )
        });

        search_view.update_in(cx, |search_view, _window, _cx| {
            assert!(
                search_view.search_options.contains(SearchOptions::REGEX),
                "Regex should still be enabled"
            );
            assert!(
                search_view
                    .search_options
                    .contains(SearchOptions::CASE_SENSITIVE),
                "Case sensitive should be preserved from previous deploy"
            );
            assert!(
                search_view
                    .search_options
                    .contains(SearchOptions::WHOLE_WORD),
                "Whole word should be preserved from previous deploy"
            );
            assert!(
                search_view
                    .search_options
                    .contains(SearchOptions::INCLUDE_IGNORED),
                "Include ignored should be preserved from previous deploy"
            );
        });

        // Redeploy explicitly turning off options.
        cx.dispatch_action(menu::Cancel);
        workspace.update_in(cx, |workspace, window, cx| {
            ProjectSearchView::deploy_search(
                workspace,
                &workspace::DeploySearch {
                    regex: Some(true),
                    case_sensitive: Some(false),
                    whole_word: Some(false),
                    include_ignored: Some(false),
                    ..Default::default()
                },
                window,
                cx,
            )
        });

        search_view.update_in(cx, |search_view, _window, _cx| {
            assert_eq!(
                search_view.search_options,
                SearchOptions::REGEX,
                "Explicit Some(false) should turn off options"
            );
        });

        // Redeploy with an empty query - should not overwrite the existing query.
        cx.dispatch_action(menu::Cancel);
        workspace.update_in(cx, |workspace, window, cx| {
            ProjectSearchView::deploy_search(
                workspace,
                &workspace::DeploySearch {
                    query: Some("".into()),
                    ..Default::default()
                },
                window,
                cx,
            )
        });

        search_view.update_in(cx, |search_view, _window, cx| {
            let query_text = search_view.query_editor.read(cx).text(cx);
            assert_eq!(
                query_text, "Test_Query",
                "Empty query string should not overwrite the existing query"
            );
        });
    }

    #[gpui::test]
    async fn test_replace_all_with_shared_heading_prefix_does_not_loop(cx: &mut TestAppContext) {
        init_test(cx);

        let search_text = "## この日に作成したノート";
        let replacement_text = "## この日に関連するノート";

        let file_a_before = format!("{search_text}\n- a\n\n{search_text}\n- b\n");
        let file_b_before = format!("# Daily\n\n{search_text}\n- c\n");
        let file_a_after = format!("{replacement_text}\n- a\n\n{replacement_text}\n- b\n");
        let file_b_after = format!("# Daily\n\n{replacement_text}\n- c\n");

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/dir"),
            json!({
                "a.md": file_a_before,
                "b.md": file_b_before,
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let worktree_id = project.update(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        let window =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let search = cx.new(|cx| ProjectSearch::new(project.clone(), workspace.downgrade(), cx));
        let search_view = cx.add_window(|window, cx| {
            ProjectSearchView::new(workspace.downgrade(), search.clone(), window, cx, None)
        });

        perform_search(search_view, search_text, cx);

        search_view
            .update(cx, |search_view, _window, cx| {
                assert_eq!(search_view.entity.read(cx).match_ranges.len(), 3);
            })
            .unwrap();

        search_view
            .update(cx, |search_view, window, cx| {
                search_view.replacement_editor.update(cx, |editor, cx| {
                    editor.set_text(replacement_text, window, cx);
                });
                search_view.replace_all(&ReplaceAll, window, cx);
            })
            .unwrap();

        cx.run_until_parked();

        let buffer_a = project
            .update(cx, |project, cx| {
                project.open_buffer((worktree_id, rel_path("a.md")), cx)
            })
            .await
            .unwrap();
        let buffer_b = project
            .update(cx, |project, cx| {
                project.open_buffer((worktree_id, rel_path("b.md")), cx)
            })
            .await
            .unwrap();

        assert_eq!(
            buffer_a.read_with(cx, |buffer, _| buffer.text()),
            file_a_after
        );
        assert_eq!(
            buffer_b.read_with(cx, |buffer, _| buffer.text()),
            file_b_after
        );
    }

    #[gpui::test]
    async fn test_smartcase_overrides_explicit_case_sensitive(cx: &mut TestAppContext) {
        init_test(cx);

        cx.update(|cx| {
            cx.update_global::<SettingsStore, _>(|store, cx| {
                store.update_default_settings(cx, |settings| {
                    settings.editor.use_smartcase_search = Some(true);
                });
            });
        });

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/dir"),
            json!({
                "one.rs": "const ONE: usize = 1;",
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let window = cx.add_window(|window, cx| MultiWorkspace::test_new(project, window, cx));
        let workspace = window
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let cx = &mut VisualTestContext::from_window(window.into(), cx);
        let search_bar = window.build_entity(cx, |_, _| ProjectSearchBar::new());

        workspace.update_in(cx, |workspace, window, cx| {
            workspace.panes()[0].update(cx, |pane, cx| {
                pane.toolbar()
                    .update(cx, |toolbar, cx| toolbar.add_item(search_bar, window, cx))
            });

            ProjectSearchView::deploy_search(
                workspace,
                &workspace::DeploySearch {
                    case_sensitive: Some(true),
                    query: Some("lowercase_query".into()),
                    ..Default::default()
                },
                window,
                cx,
            )
        });

        let search_view = cx
            .read(|cx| {
                workspace
                    .read(cx)
                    .active_pane()
                    .read(cx)
                    .active_item()
                    .and_then(|item| item.downcast::<ProjectSearchView>())
            })
            .expect("Search view should be active after deploy");

        // Smartcase should override the explicit case_sensitive flag
        // because the query is all lowercase.
        search_view.update_in(cx, |search_view, _window, cx| {
            assert!(
                !search_view
                    .search_options
                    .contains(SearchOptions::CASE_SENSITIVE),
                "Smartcase should disable case sensitivity for a lowercase query, \
                 even when case_sensitive was explicitly set in the action"
            );
            let query_text = search_view.query_editor.read(cx).text(cx);
            assert_eq!(query_text, "lowercase_query");
        });

        // Now deploy with an uppercase query - smartcase should enable case sensitivity.
        workspace.update_in(cx, |workspace, window, cx| {
            ProjectSearchView::deploy_search(
                workspace,
                &workspace::DeploySearch {
                    query: Some("Uppercase_Query".into()),
                    ..Default::default()
                },
                window,
                cx,
            )
        });

        search_view.update_in(cx, |search_view, _window, cx| {
            assert!(
                search_view
                    .search_options
                    .contains(SearchOptions::CASE_SENSITIVE),
                "Smartcase should enable case sensitivity for a query containing uppercase"
            );
            let query_text = search_view.query_editor.read(cx).text(cx);
            assert_eq!(query_text, "Uppercase_Query");
        });
    }

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings = SettingsStore::test(cx);
            cx.set_global(settings);

            theme_settings::init(theme::LoadThemes::JustBase, cx);

            editor::init(cx);
            crate::init(cx);

            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings
                        .editor
                        .search
                        .get_or_insert_default()
                        .search_on_type = Some(false);
                });
            });
        });
    }

    fn perform_search(
        search_view: WindowHandle<ProjectSearchView>,
        text: impl Into<Arc<str>>,
        cx: &mut TestAppContext,
    ) {
        search_view
            .update(cx, |search_view, window, cx| {
                search_view.query_editor.update(cx, |query_editor, cx| {
                    query_editor.set_text(text, window, cx)
                });
                search_view.search(SearchMode::Manual, cx);
            })
            .unwrap();
        // Ensure editor highlights appear after the search is done
        cx.executor().advance_clock(
            editor::SELECTION_HIGHLIGHT_DEBOUNCE_TIMEOUT + Duration::from_millis(100),
        );
        cx.background_executor.run_until_parked();
    }

    #[gpui::test]
    async fn test_incremental_search_narrows_and_widens(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/dir"),
            json!({
                "one.rs": "const ONE: usize = 1;\nconst ONEROUS: usize = 2;",
                "two.rs": "const TWO: usize = one::ONE + one::ONE;",
                "three.rs": "const THREE: usize = one::ONE + two::TWO;",
                "four.rs": "const FOUR: usize = one::ONE + three::THREE;",
                "only_one.rs": "const ONLY_ONE: usize = 1;",
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let window =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let search = cx.new(|cx| ProjectSearch::new(project.clone(), workspace.downgrade(), cx));
        let search_view = cx.add_window(|window, cx| {
            ProjectSearchView::new(workspace.downgrade(), search.clone(), window, cx, None)
        });
        let expected_one_matches = vec![
            "one", "ONE", "ONE", "ONE", "ONE", "one", "ONE", "one", "ONE", "one", "ONE",
        ];

        // Initial non-incremental search for "ONE" — inserts one excerpt per file.
        perform_search(search_view, "ONE", cx);
        assert_eq!(match_texts(&search, cx), expected_one_matches);
        assert_all_highlights_match_query(&search, "ONE", cx);

        // Narrowing: "ONE" -> "ONER". Only one.rs has ONEROUS.
        perform_incremental_search(search_view, "ONER", cx);
        assert_eq!(match_texts(&search, cx), vec!["ONER"]);
        assert_all_highlights_match_query(&search, "ONER", cx);

        // Continue narrowing: "ONER" -> "ONEROUS". Still one.rs only.
        perform_incremental_search(search_view, "ONEROUS", cx);
        assert_eq!(match_texts(&search, cx), vec!["ONEROUS"]);
        assert_all_highlights_match_query(&search, "ONEROUS", cx);

        // Backspace to "ONER" — still one.rs only.
        perform_incremental_search(search_view, "ONER", cx);
        assert_eq!(match_texts(&search, cx), vec!["ONER"]);

        // Backspace to "ONE" — all files re-appear.
        perform_incremental_search(search_view, "ONE", cx);
        assert_eq!(match_texts(&search, cx), expected_one_matches);
        assert_all_highlights_match_query(&search, "ONE", cx);

        // Narrow to "ONLY_ONE" — single match in only_one.rs.
        perform_incremental_search(search_view, "ONLY_ONE", cx);
        assert_eq!(match_texts(&search, cx), vec!["ONLY_ONE"]);
        assert_all_highlights_match_query(&search, "ONLY_ONE", cx);

        // Widen back to "ONE" — all files re-appear.
        perform_incremental_search(search_view, "ONE", cx);
        assert_eq!(match_texts(&search, cx), expected_one_matches);
        assert_all_highlights_match_query(&search, "ONE", cx);
    }

    #[gpui::test]
    async fn test_incremental_search_gutter_width_never_shrinks(cx: &mut TestAppContext) {
        init_test(cx);

        let big_file = (1..=12000)
            .map(|i| {
                if i == 11000 {
                    format!("line {i}: needle_in_big")
                } else {
                    format!("line {i}: filler")
                }
            })
            .collect::<Vec<_>>()
            .join("\n");
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/dir"),
            json!({
                "small.rs": "fn needle_small() {}",
                "big.txt": big_file,
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let window =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let search = cx.new(|cx| ProjectSearch::new(project.clone(), workspace.downgrade(), cx));
        let search_view = cx.add_window(|window, cx| {
            ProjectSearchView::new(workspace.downgrade(), search.clone(), window, cx, None)
        });

        perform_search(search_view, "needle_in_big", cx);
        assert_eq!(match_count(&search, cx), 1);
        assert_eq!(reserved_gutter_digits(search_view, cx), 5);

        perform_incremental_search(search_view, "needle_small", cx);
        assert_eq!(match_count(&search, cx), 1);
        assert_eq!(reserved_gutter_digits(search_view, cx), 5);

        perform_search(search_view, "needle_small", cx);
        assert_eq!(reserved_gutter_digits(search_view, cx), 1);
    }

    #[gpui::test]
    async fn test_search_on_type_keeps_focus_confirm_shifts_it(cx: &mut TestAppContext) {
        init_test(cx);
        enable_search_on_type(cx);

        let SearchBarTest {
            window,
            search_view,
            cx,
            ..
        } = &mut setup_search_bar_test(
            json!({
                "one.rs": "const ONE: usize = 1;",
                "two.rs": "const TWO: usize = one::ONE + one::ONE;",
                "three.rs": "const THREE: usize = one::ONE + two::TWO;",
            }),
            cx,
        )
        .await;

        // Typing triggers a debounced incremental search but must not steal focus
        // from the query editor.
        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    search_view.query_editor.update(cx, |query_editor, cx| {
                        query_editor.set_text("ONE", window, cx);
                    });
                });
            })
            .unwrap();
        cx.background_executor
            .advance_clock(SEARCH_ON_TYPE_DEBOUNCE + Duration::from_millis(50));
        cx.background_executor.run_until_parked();

        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    assert!(
                        !search_view.entity.read(cx).match_ranges.is_empty(),
                        "Incremental search should have found matches",
                    );
                    assert!(
                        search_view.query_editor.focus_handle(cx).is_focused(window),
                        "Query editor should remain focused while typing with search_on_type",
                    );
                    assert!(
                        !search_view
                            .results_editor
                            .focus_handle(cx)
                            .is_focused(window),
                        "Results editor should not be focused while typing",
                    );
                });
            })
            .unwrap();

        // Confirming the search shifts focus to the results editor.
        cx.dispatch_action(Confirm);
        cx.background_executor.run_until_parked();

        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    assert!(
                        search_view
                            .results_editor
                            .focus_handle(cx)
                            .is_focused(window),
                        "Results editor should be focused after confirming",
                    );
                    assert_eq!(search_view.active_match_index, Some(0));
                });
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_confirm_while_reusing_search_pending_defers_focus(cx: &mut TestAppContext) {
        init_test(cx);
        enable_search_on_type(cx);

        let SearchBarTest {
            window,
            search_bar,
            search_view,
            cx,
            ..
        } = &mut setup_search_bar_test(
            json!({
                "one.rs": "const ONE: usize = 1;",
                "two.rs": "const TWO: usize = one::ONE + one::ONE;",
            }),
            cx,
        )
        .await;

        // Run an initial on-type search for "ONE" to completion.
        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    search_view.query_editor.update(cx, |query_editor, cx| {
                        query_editor.set_text("ONE", window, cx);
                    });
                });
            })
            .unwrap();
        cx.background_executor
            .advance_clock(SEARCH_ON_TYPE_DEBOUNCE + Duration::from_millis(50));
        cx.background_executor.run_until_parked();

        // Change the query but confirm before the debounce fires: `confirm` starts a reusing
        // search that is still pending, so `match_ranges` still holds "ONE" results.
        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    search_view.query_editor.update(cx, |query_editor, cx| {
                        query_editor.set_text("TWO", window, cx);
                    });
                });
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.confirm(&Confirm, window, cx);
                });
            })
            .unwrap();

        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    assert!(
                        search_view.entity.read(cx).pending_search.is_some(),
                        "Confirming a stale query starts a reusing search that is still pending",
                    );
                    assert!(
                        search_view.query_editor.focus_handle(cx).is_focused(window),
                        "Focus must stay on the query editor until the pending search resolves \
                         so we never select against the previous query's stale matches",
                    );
                });
            })
            .unwrap();

        // Once the confirmed search resolves, focus shifts to the fresh results.
        cx.background_executor.run_until_parked();
        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    assert!(
                        search_view
                            .results_editor
                            .focus_handle(cx)
                            .is_focused(window),
                        "Results editor should be focused after the confirmed search completes",
                    );
                    assert_eq!(search_view.active_match_index, Some(0));
                    let model = search_view.entity.read(cx);
                    let snapshot = model.excerpts.read(cx).snapshot(cx);
                    let match_texts = model
                        .match_ranges
                        .iter()
                        .map(|range| snapshot.text_for_range(range.clone()).collect::<String>())
                        .collect::<Vec<_>>();
                    assert!(
                        match_texts.iter().all(|text| text == "TWO"),
                        "Selection and results should reflect the confirmed query, got {match_texts:?}",
                    );
                });
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_incremental_search_reuses_unchanged_excerpts(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/dir"),
            json!({
                "aaa.rs": "fn needle() {}",
                "bbb.rs": "fn needle_stable() {}",
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let window =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let search = cx.new(|cx| ProjectSearch::new(project.clone(), workspace.downgrade(), cx));
        let search_view = cx.add_window(|window, cx| {
            ProjectSearchView::new(workspace.downgrade(), search.clone(), window, cx, None)
        });

        perform_search(search_view, "needle", cx);
        assert_eq!(match_texts(&search, cx), vec!["needle", "needle"]);

        let excerpts = search.read_with(cx, |search, _| search.excerpts.clone());
        let (stale_buffer_id, stable_buffer_id) = cx.read(|cx| {
            let buffer_id_for_path = |path: &util::rel_path::RelPath| {
                excerpts.read(cx).all_buffers_iter().find_map(|buffer| {
                    let buffer = buffer.read(cx);
                    (buffer.file()?.path().as_ref() == path).then(|| buffer.remote_id())
                })
            };
            (
                buffer_id_for_path(rel_path("aaa.rs")).expect("aaa.rs buffer expected"),
                buffer_id_for_path(rel_path("bbb.rs")).expect("bbb.rs buffer expected"),
            )
        });
        let excerpts_for_buffer = |buffer_id, cx: &mut TestAppContext| {
            cx.read(|cx| {
                excerpts
                    .read(cx)
                    .snapshot(cx)
                    .excerpts_for_buffer(buffer_id)
                    .collect::<Vec<_>>()
            })
        };
        let stale_excerpts_before = excerpts_for_buffer(stale_buffer_id, cx);
        let stable_excerpts_before = excerpts_for_buffer(stable_buffer_id, cx);

        let updated_paths = Rc::new(RefCell::new(Vec::new()));
        let removed_buffers = Rc::new(RefCell::new(Vec::new()));
        let _subscription = cx.update(|cx| {
            cx.subscribe(&excerpts, {
                let updated_paths = updated_paths.clone();
                let removed_buffers = removed_buffers.clone();
                move |_, event: &multi_buffer::Event, _| match event {
                    multi_buffer::Event::BufferRangesUpdated { path_key, .. } => {
                        updated_paths.borrow_mut().push(path_key.clone());
                    }
                    multi_buffer::Event::BuffersRemoved { removed_buffer_ids } => {
                        removed_buffers
                            .borrow_mut()
                            .extend(removed_buffer_ids.iter().copied());
                    }
                    _ => {}
                }
            })
        });

        // Re-running the same query incrementally must reuse every excerpt as is,
        // without editing the multibuffer at all.
        perform_incremental_search(search_view, "needle", cx);
        assert_eq!(match_texts(&search, cx), vec!["needle", "needle"]);
        assert_eq!(
            excerpts_for_buffer(stale_buffer_id, cx),
            stale_excerpts_before
        );
        assert_eq!(
            excerpts_for_buffer(stable_buffer_id, cx),
            stable_excerpts_before
        );
        assert_eq!(*updated_paths.borrow(), Vec::<PathKey>::new());
        assert_eq!(*removed_buffers.borrow(), Vec::<language::BufferId>::new());

        // Narrowing incremental search: bbb.rs keeps matching on the same line, so its
        // excerpt keeps its context range, while aaa.rs gets pruned as stale.
        perform_incremental_search(search_view, "needle_stable", cx);
        assert_eq!(match_texts(&search, cx), vec!["needle_stable"]);
        assert_eq!(*removed_buffers.borrow(), vec![stale_buffer_id]);
        assert_eq!(excerpts_for_buffer(stale_buffer_id, cx), Vec::new());
        assert_eq!(
            excerpts_for_buffer(stable_buffer_id, cx)
                .into_iter()
                .map(|range| range.context)
                .collect::<Vec<_>>(),
            stable_excerpts_before
                .into_iter()
                .map(|range| range.context)
                .collect::<Vec<_>>()
        );
    }

    #[gpui::test]
    async fn test_search_on_type_history_navigation(cx: &mut TestAppContext) {
        init_test(cx);
        enable_search_on_type(cx);

        let SearchBarTest {
            project,
            window,
            search_bar,
            search_view,
            cx,
        } = &mut setup_search_bar_test(
            json!({
                "one.rs": "const ONE: usize = 1;",
                "two.rs": "const TWO: usize = one::ONE + one::ONE;",
            }),
            cx,
        )
        .await;

        let read_query_history = |cx: &mut VisualTestContext| {
            cx.read(|cx| {
                project
                    .read(cx)
                    .search_history(SearchInputKind::Query)
                    .iter()
                    .map(str::to_string)
                    .collect::<Vec<_>>()
            })
        };
        let read_query_text = |cx: &mut VisualTestContext| {
            cx.read(|cx| search_view.read(cx).query_editor.read(cx).text(cx))
        };

        // Typing runs debounced incremental searches that must not touch the history.
        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    search_view.query_editor.update(cx, |query_editor, cx| {
                        query_editor.set_text("ONE", window, cx)
                    });
                });
            })
            .unwrap();
        cx.background_executor
            .advance_clock(SEARCH_ON_TYPE_DEBOUNCE + Duration::from_millis(50));
        cx.background_executor.run_until_parked();
        assert_eq!(read_query_history(cx), Vec::<String>::new());

        // Confirming records the query into the history.
        window
            .update(cx, |_, window, cx| {
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.confirm(&Confirm, window, cx);
                });
            })
            .unwrap();
        cx.background_executor.run_until_parked();
        assert_eq!(read_query_history(cx), vec!["ONE".to_string()]);

        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    search_view.query_editor.update(cx, |query_editor, cx| {
                        query_editor.set_text("TWO", window, cx)
                    });
                });
            })
            .unwrap();
        cx.background_executor
            .advance_clock(SEARCH_ON_TYPE_DEBOUNCE + Duration::from_millis(50));
        cx.background_executor.run_until_parked();
        assert_eq!(read_query_history(cx), vec!["ONE".to_string()]);

        window
            .update(cx, |_, window, cx| {
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.confirm(&Confirm, window, cx);
                });
            })
            .unwrap();
        cx.background_executor.run_until_parked();
        assert_eq!(
            read_query_history(cx),
            vec!["TWO".to_string(), "ONE".to_string()]
        );

        // Up walks back through the confirmed entries.
        window
            .update(cx, |_, window, cx| {
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.focus_search(window, cx);
                    search_bar.previous_history_query(&PreviousHistoryQuery, window, cx);
                });
            })
            .unwrap();
        assert_eq!(read_query_text(cx), "ONE");

        // There is nothing before the first entry.
        window
            .update(cx, |_, window, cx| {
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.focus_search(window, cx);
                    search_bar.previous_history_query(&PreviousHistoryQuery, window, cx);
                });
            })
            .unwrap();
        assert_eq!(read_query_text(cx), "ONE");

        // Down walks forward again.
        window
            .update(cx, |_, window, cx| {
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.focus_search(window, cx);
                    search_bar.next_history_query(&NextHistoryQuery, window, cx);
                });
            })
            .unwrap();
        assert_eq!(read_query_text(cx), "TWO");

        // Let the debounced searches triggered by history navigation run: they must
        // not add new history entries or reset the cursor.
        cx.background_executor
            .advance_clock(SEARCH_ON_TYPE_DEBOUNCE + Duration::from_millis(50));
        cx.background_executor.run_until_parked();
        assert_eq!(
            read_query_history(cx),
            vec!["TWO".to_string(), "ONE".to_string()]
        );
        window
            .update(cx, |_, window, cx| {
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.focus_search(window, cx);
                    search_bar.previous_history_query(&PreviousHistoryQuery, window, cx);
                });
            })
            .unwrap();
        assert_eq!(read_query_text(cx), "ONE");
    }

    #[gpui::test]
    async fn test_select_next_match_during_pending_incremental_search(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/dir"),
            json!({
                "one.rs": "const ONE: usize = 1;",
                "two.rs": "const TWO: usize = one::ONE + one::ONE;",
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let window =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let search = cx.new(|cx| ProjectSearch::new(project.clone(), workspace.downgrade(), cx));
        let search_view = cx.add_window(|window, cx| {
            ProjectSearchView::new(workspace.downgrade(), search.clone(), window, cx, None)
        });

        perform_search(search_view, "ONE", cx);
        assert_eq!(match_count(&search, cx), 5);

        search_view
            .update(cx, |search_view, _window, cx| {
                search_view.search(SearchMode::OnType, cx);
            })
            .unwrap();
        search_view
            .update(cx, |search_view, window, cx| {
                assert_eq!(
                    search_view.entity.read(cx).match_ranges.len(),
                    5,
                    "previous matches stay in place until the pending incremental search completes"
                );
                assert_eq!(search_view.active_match_index, Some(0));
                search_view.select_match(Direction::Next, window, cx);
                search_view.select_match(Direction::Prev, window, cx);
                assert_eq!(
                    search_view.active_match_index,
                    Some(0),
                    "navigation works against the preserved matches instead of an empty list"
                );
            })
            .unwrap();

        cx.background_executor.run_until_parked();
        assert_eq!(match_count(&search, cx), 5);
    }

    #[gpui::test]
    async fn test_incremental_search_preserves_scroll_position(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/dir"),
            json!({
                "1.txt": "\n\n\n\n\n A \n\n\n\n\n",
                "2.txt": "\n\n\n\n\n A \n\n\n\n\n",
                "3.rs": "\n\n\n\n\n A \n\n\n\n\n",
                "4.rs": "\n\n\n\n\n A \n\n\n\n\n",
                "5.rs": "\n\n\n\n\n A \n\n\n\n\n",
                "6.rs": "\n\n\n\n\n A \n\n\n\n\n",
                "7.rs": "\n\n\n\n\n A \n\n\n\n\n",
                "8.rs": "\n\n\n\n\n A \n\n\n\n\n",
                "9.rs": "\n\n\n\n\n A \n\n\n\n\n",
                "a.rs": "\n\n\n\n\n A \n\n\n\n\n",
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let window =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let search = cx.new(|cx| ProjectSearch::new(project, workspace.downgrade(), cx));
        let search_view = cx.add_window(|window, cx| {
            ProjectSearchView::new(workspace.downgrade(), search.clone(), window, cx, None)
        });

        perform_search(search_view, "A", cx);
        search_view
            .update(cx, |search_view, window, cx| {
                search_view.results_editor.update(cx, |results_editor, cx| {
                    assert_eq!(
                        results_editor.scroll_position(cx),
                        Point::default(),
                        "a confirmed search scrolls to the top"
                    );
                    results_editor.scroll(Point::new(0., f64::MAX), window, cx);
                });
            })
            .unwrap();

        perform_incremental_search(search_view, "A", cx);
        search_view
            .update(cx, |search_view, _, cx| {
                search_view.results_editor.update(cx, |results_editor, cx| {
                    assert!(
                        results_editor.scroll_position(cx).y > 0.,
                        "search-on-type should not scroll the results back to the top"
                    );
                });
            })
            .unwrap();

        perform_search(search_view, "A", cx);
        search_view
            .update(cx, |search_view, _, cx| {
                search_view.results_editor.update(cx, |results_editor, cx| {
                    assert_eq!(
                        results_editor.scroll_position(cx),
                        Point::default(),
                        "confirming the query scrolls back to the top"
                    );
                });
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_search_on_type_resets_scroll_after_empty_results(cx: &mut TestAppContext) {
        init_test(cx);
        enable_search_on_type(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/dir"),
            json!({
                "1.txt": "\n\n\n\n\n A \n\n\n\n\n",
                "2.txt": "\n\n\n\n\n A \n\n\n\n\n",
                "3.rs": "\n\n\n\n\n A \n\n\n\n\n",
                "4.rs": "\n\n\n\n\n A \n\n\n\n\n",
                "5.rs": "\n\n\n\n\n A \n\n\n\n\n",
                "6.rs": "\n\n\n\n\n A \n\n\n\n\n",
                "7.rs": "\n\n\n\n\n A \n\n\n\n\n",
                "8.rs": "\n\n\n\n\n A \n\n\n\n\n",
                "9.rs": "\n\n\n\n\n A \n\n\n\n\n",
                "a.rs": "\n\n\n\n\n A \n\n\n\n\n",
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let window =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let search = cx.new(|cx| ProjectSearch::new(project, workspace.downgrade(), cx));
        let search_view = cx.add_window(|window, cx| {
            ProjectSearchView::new(workspace.downgrade(), search.clone(), window, cx, None)
        });

        let type_query = |query: &str, cx: &mut TestAppContext| {
            search_view
                .update(cx, |search_view, window, cx| {
                    search_view.query_editor.update(cx, |query_editor, cx| {
                        query_editor.set_text(query, window, cx)
                    });
                })
                .unwrap();
            cx.executor()
                .advance_clock(SEARCH_ON_TYPE_DEBOUNCE + Duration::from_millis(50));
            cx.background_executor.run_until_parked();
        };

        type_query("A", cx);
        assert_eq!(match_count(&search, cx), 10);
        search_view
            .update(cx, |search_view, window, cx| {
                search_view.results_editor.update(cx, |results_editor, cx| {
                    results_editor.scroll(Point::new(0., f64::MAX), window, cx);
                    assert!(
                        results_editor.scroll_position(cx).y > 0.,
                        "results should be long enough to scroll down"
                    );
                });
            })
            .unwrap();

        type_query("", cx);
        assert_eq!(match_count(&search, cx), 0);

        type_query("A", cx);
        assert_eq!(match_count(&search, cx), 10);
        search_view
            .update(cx, |search_view, _, cx| {
                search_view.results_editor.update(cx, |results_editor, cx| {
                    assert_eq!(
                        results_editor.scroll_position(cx),
                        Point::default(),
                        "erasing the query fully must reset the scroll, \
                         a retyped query starts at the top"
                    );
                });
            })
            .unwrap();

        search_view
            .update(cx, |search_view, window, cx| {
                search_view.results_editor.update(cx, |results_editor, cx| {
                    results_editor.scroll(Point::new(0., f64::MAX), window, cx);
                });
            })
            .unwrap();

        type_query("NOTHINGmATCHEStHIS", cx);
        assert_eq!(match_count(&search, cx), 0);

        type_query("A", cx);
        assert_eq!(match_count(&search, cx), 10);
        search_view
            .update(cx, |search_view, _, cx| {
                search_view.results_editor.update(cx, |results_editor, cx| {
                    assert_eq!(
                        results_editor.scroll_position(cx),
                        Point::default(),
                        "a query with no hits destroys all excerpts and with them the scroll \
                         anchor, so widening the query back must start at the top instead of \
                         leaving the user stranded past the last result"
                    );
                });
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_search_on_type_surfaces_query_errors(cx: &mut TestAppContext) {
        init_test(cx);
        enable_search_on_type(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(path!("/dir"), json!({ "one.rs": "const ONE: usize = 1;" }))
            .await;
        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let window =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let search = cx.new(|cx| ProjectSearch::new(project.clone(), workspace.downgrade(), cx));
        let search_view = cx.add_window(|window, cx| {
            ProjectSearchView::new(workspace.downgrade(), search.clone(), window, cx, None)
        });

        search_view
            .update(cx, |search_view, window, cx| {
                search_view.search_options.insert(SearchOptions::REGEX);
                search_view.query_editor.update(cx, |query_editor, cx| {
                    query_editor.set_text("(unclosed", window, cx)
                });
                search_view.search(SearchMode::OnType, cx);
                assert!(
                    search_view
                        .panels_with_errors
                        .contains_key(&InputPanel::Query),
                    "an invalid regex must surface an error even without confirming, otherwise \
                     search-on-type results silently stop updating"
                );
            })
            .unwrap();

        search_view
            .update(cx, |search_view, window, cx| {
                search_view.query_editor.update(cx, |query_editor, cx| {
                    query_editor.set_text("closed", window, cx)
                });
                search_view.search(SearchMode::OnType, cx);
                assert_eq!(
                    search_view.panels_with_errors.get(&InputPanel::Query),
                    None,
                    "fixing the query clears the error again"
                );
            })
            .unwrap();

        search_view
            .update(cx, |search_view, window, cx| {
                search_view.query_editor.update(cx, |query_editor, cx| {
                    query_editor.set_text("(unclosed", window, cx)
                });
                search_view.search(SearchMode::OnType, cx);
            })
            .unwrap();
        search_view
            .update(cx, |search_view, window, cx| {
                search_view
                    .query_editor
                    .update(cx, |query_editor, cx| query_editor.set_text("", window, cx));
            })
            .unwrap();
        cx.executor()
            .advance_clock(SEARCH_ON_TYPE_DEBOUNCE + Duration::from_millis(50));
        cx.background_executor.run_until_parked();
        search_view
            .update(cx, |search_view, _, cx| {
                assert_eq!(
                    search_view.panels_with_errors.get(&InputPanel::Query),
                    None,
                    "erasing the query must clear its stale error, \
                     an empty input has nothing to be invalid"
                );
                assert_eq!(search_view.entity.read(cx).match_ranges.len(), 0);
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_no_results_verdict_lifecycle(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(path!("/dir"), json!({ "one.rs": "const ONE: usize = 1;" }))
            .await;
        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let window =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let search = cx.new(|cx| ProjectSearch::new(project.clone(), workspace.downgrade(), cx));
        let search_view = cx.add_window(|window, cx| {
            ProjectSearchView::new(workspace.downgrade(), search.clone(), window, cx, None)
        });

        let no_results_so_far = |cx: &mut TestAppContext| {
            search.read_with(cx, |search, _| search.search_state.no_results_so_far())
        };

        assert!(!no_results_so_far(cx), "an idle search has no verdict yet");

        perform_search(search_view, "sOMETHINGtHATsURELYdOESnOTeXIST", cx);
        assert_eq!(match_count(&search, cx), 0);
        assert!(no_results_so_far(cx));

        search_view
            .update(cx, |search_view, _, cx| {
                search_view.search(SearchMode::OnType, cx);
                assert!(
                    search_view.entity.read(cx).pending_search.is_some(),
                    "the re-search should be pending"
                );
            })
            .unwrap();
        assert!(
            no_results_so_far(cx),
            "a pending re-search must hold the previous no-results verdict instead of \
             flickering through an indeterminate state"
        );
        cx.background_executor.run_until_parked();
        assert!(no_results_so_far(cx));

        search_view
            .update(cx, |search_view, window, cx| {
                search_view.query_editor.update(cx, |query_editor, cx| {
                    query_editor.set_text("sOMETHINGtHATsURELYdOESnOTeXISTeITHER", window, cx)
                });
                search_view.search(SearchMode::OnType, cx);
            })
            .unwrap();
        assert!(
            !no_results_so_far(cx),
            "typing a different query must drop the stale no-results verdict: the new query \
             may well have matches, so asserting 'No Results' for it would be a lie"
        );
        cx.background_executor.run_until_parked();
        assert!(
            no_results_so_far(cx),
            "the new query has no matches either, so the verdict returns once the search settles"
        );

        perform_incremental_search(search_view, "ONE", cx);
        assert!(!no_results_so_far(cx));
        assert!(match_count(&search, cx) > 0);
    }

    #[gpui::test]
    async fn test_search_on_type_confirm_after_filter_change_researches(cx: &mut TestAppContext) {
        init_test(cx);
        enable_search_on_type(cx);

        let SearchBarTest {
            window,
            search_bar,
            search_view,
            cx,
            ..
        } = &mut setup_search_bar_test(
            json!({
                "one.rs": "const ONE: usize = 1;",
                "two.rs": "const ONE_IN_TWO: usize = 1;",
            }),
            cx,
        )
        .await;
        let search = cx.read(|cx| search_view.read(cx).entity.clone());

        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    search_view.query_editor.update(cx, |query_editor, cx| {
                        query_editor.set_text("ONE", window, cx)
                    });
                });
            })
            .unwrap();
        cx.background_executor
            .advance_clock(SEARCH_ON_TYPE_DEBOUNCE + Duration::from_millis(50));
        cx.background_executor.run_until_parked();
        assert_eq!(match_count(&search, cx), 2);

        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    search_view.toggle_filters(cx);
                    search_view
                        .excluded_files_editor
                        .update(cx, |editor, cx| editor.set_text("one.rs", window, cx));
                });
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.confirm(&Confirm, window, cx);
                });
            })
            .unwrap();
        cx.background_executor.run_until_parked();
        assert_eq!(match_count(&search, cx), 1);
        assert_eq!(matched_file_names(&search, cx), vec!["two.rs"]);

        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    search_view
                        .excluded_files_editor
                        .update(cx, |editor, cx| editor.set_text("", window, cx));
                });
            })
            .unwrap();
        cx.background_executor
            .advance_clock(SEARCH_ON_TYPE_DEBOUNCE + Duration::from_millis(50));
        cx.background_executor.run_until_parked();
        assert_eq!(match_count(&search, cx), 2);
        assert_eq!(matched_file_names(&search, cx), vec!["one.rs", "two.rs"]);
    }

    #[gpui::test]
    async fn test_search_on_type_confirm_after_erasing_query_researches(cx: &mut TestAppContext) {
        init_test(cx);
        enable_search_on_type(cx);

        let SearchBarTest {
            window,
            search_bar,
            search_view,
            cx,
            ..
        } = &mut setup_search_bar_test(
            json!({
                "one.rs": "const ONE: usize = 1;",
                "two.rs": "const TWO: usize = 2;",
            }),
            cx,
        )
        .await;
        let search = cx.read(|cx| search_view.read(cx).entity.clone());

        let type_query = |query: &str, cx: &mut VisualTestContext| {
            window
                .update(cx, |_, window, cx| {
                    search_view.update(cx, |search_view, cx| {
                        search_view.query_editor.update(cx, |query_editor, cx| {
                            query_editor.set_text(query, window, cx)
                        });
                    });
                })
                .unwrap();
            cx.background_executor
                .advance_clock(SEARCH_ON_TYPE_DEBOUNCE + Duration::from_millis(50));
            cx.background_executor.run_until_parked();
        };

        type_query("ONE", cx);
        assert_eq!(match_count(&search, cx), 1);

        type_query("", cx);
        assert_eq!(match_count(&search, cx), 0);

        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    search_view.query_editor.update(cx, |query_editor, cx| {
                        query_editor.set_text("ONE", window, cx)
                    });
                });
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.confirm(&Confirm, window, cx);
                });
            })
            .unwrap();
        cx.background_executor.run_until_parked();
        assert_eq!(match_count(&search, cx), 1);
        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    assert!(
                        search_view
                            .results_editor
                            .focus_handle(cx)
                            .is_focused(window),
                        "Confirming a retyped query must behave like any confirmed search",
                    );
                });
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_toggle_option_cancels_pending_debounced_search(cx: &mut TestAppContext) {
        init_test(cx);
        enable_search_on_type(cx);

        let SearchBarTest {
            window,
            search_bar,
            search_view,
            cx,
            ..
        } = &mut setup_search_bar_test(
            json!({
                "one.rs": "const ONE: usize = 1;",
                "two.rs": "const TWO: usize = 2;",
            }),
            cx,
        )
        .await;

        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    search_view.query_editor.update(cx, |query_editor, cx| {
                        query_editor.set_text("ONE", window, cx)
                    });
                });
            })
            .unwrap();
        cx.background_executor
            .advance_clock(SEARCH_ON_TYPE_DEBOUNCE + Duration::from_millis(50));
        cx.background_executor.run_until_parked();
        let initial_search_id = cx.read(|cx| search_view.read(cx).entity.read(cx).search_id);

        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    search_view.query_editor.update(cx, |query_editor, cx| {
                        query_editor.set_text("TWO", window, cx)
                    });
                });
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.toggle_search_option(SearchOptions::CASE_SENSITIVE, window, cx);
                });
            })
            .unwrap();
        cx.background_executor.run_until_parked();
        let search_id_after_toggle = cx.read(|cx| search_view.read(cx).entity.read(cx).search_id);
        assert_eq!(search_id_after_toggle, initial_search_id + 1);

        cx.background_executor
            .advance_clock(SEARCH_ON_TYPE_DEBOUNCE * 2);
        cx.background_executor.run_until_parked();
        assert_eq!(
            cx.read(|cx| search_view.read(cx).entity.read(cx).search_id),
            search_id_after_toggle,
        );
    }

    #[gpui::test]
    async fn test_option_toggle_keeps_confirmed_phase_and_defers_replace_all(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);
        enable_search_on_type(cx);

        let SearchBarTest {
            project,
            window,
            search_bar,
            search_view,
            cx,
        } = &mut setup_search_bar_test(json!({ "one.rs": "one ONE" }), cx).await;
        let search = cx.read(|cx| search_view.read(cx).entity.clone());

        let read_query_history = |cx: &mut VisualTestContext| {
            cx.read(|cx| {
                project
                    .read(cx)
                    .search_history(SearchInputKind::Query)
                    .iter()
                    .map(str::to_string)
                    .collect::<Vec<_>>()
            })
        };

        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    search_view.query_editor.update(cx, |query_editor, cx| {
                        query_editor.set_text("one", window, cx)
                    });
                    search_view
                        .replacement_editor
                        .update(cx, |editor, cx| editor.set_text("x", window, cx));
                });
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.confirm(&Confirm, window, cx);
                });
            })
            .unwrap();
        cx.background_executor.run_until_parked();
        assert_eq!(match_count(&search, cx), 2);
        assert_eq!(read_query_history(cx), vec!["one".to_string()]);

        window
            .update(cx, |_, window, cx| {
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.toggle_search_option(SearchOptions::CASE_SENSITIVE, window, cx);
                });
            })
            .unwrap();
        cx.background_executor.run_until_parked();
        assert_eq!(match_count(&search, cx), 1);
        assert_eq!(
            search.read_with(cx, |search, _| search.phase),
            SearchPhase::Confirmed,
            "an option toggle refreshes the confirmed search without demoting it to a typing one"
        );
        assert_eq!(
            read_query_history(cx),
            vec!["one".to_string()],
            "refreshing on an option toggle must not duplicate the history entry"
        );

        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    search_view.search(SearchMode::Refresh, cx);
                    assert!(search_view.entity.read(cx).pending_search.is_some());
                    assert_eq!(search_view.entity.read(cx).phase, SearchPhase::Confirmed);
                    search_view.replace_all(&ReplaceAll, window, cx);
                    assert!(
                        search_view.pending_replace_all,
                        "a replacement during a pending refresh of a confirmed search must be \
                         deferred, not dropped"
                    );
                });
            })
            .unwrap();
        cx.background_executor.run_until_parked();
        let buffer_text = search.read_with(cx, |search, cx| {
            search
                .excerpts
                .read(cx)
                .all_buffers_iter()
                .next()
                .map(|buffer| buffer.read(cx).text())
        });
        assert_eq!(
            buffer_text,
            Some("x ONE".to_string()),
            "the deferred replacement runs against the refreshed results once they settle"
        );
    }

    #[gpui::test]
    async fn test_opened_only_search_is_ordered_and_reuses_excerpts(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/dir"),
            json!({
                "aaa.rs": "fn needle_first() {}",
                "bbb.rs": "fn needle_second() {}",
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let buffer_bbb = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/dir/bbb.rs"), cx)
            })
            .await
            .unwrap();
        let buffer_aaa = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/dir/aaa.rs"), cx)
            })
            .await
            .unwrap();
        let buffer_untitled = project.update(cx, |project, cx| {
            project.create_local_buffer("fn needle_untitled() {}", None, true, cx)
        });
        let search =
            cx.new(|cx| ProjectSearch::new(project.clone(), WeakEntity::new_invalid(), cx));
        let build_query = || {
            SearchOptions::NONE
                .build_query(
                    "needle",
                    PathMatcher::default(),
                    PathMatcher::default(),
                    false,
                    Some(vec![
                        buffer_bbb.clone(),
                        buffer_untitled.clone(),
                        buffer_aaa.clone(),
                    ]),
                )
                .unwrap()
        };

        search.update(cx, |search, cx| {
            search.search(build_query(), SearchMode::Manual, false, cx);
        });
        cx.run_until_parked();
        let text = search.read_with(cx, |search, cx| {
            search.excerpts.read(cx).snapshot(cx).text()
        });
        assert_eq!(
            text,
            "fn needle_untitled() {}\nfn needle_first() {}\nfn needle_second() {}"
        );
        assert_eq!(match_count(&search, cx), 3);

        let removed_buffers = Rc::new(RefCell::new(Vec::new()));
        let excerpts = search.read_with(cx, |search, _| search.excerpts.clone());
        let _subscription = cx.update(|cx| {
            cx.subscribe(&excerpts, {
                let removed_buffers = removed_buffers.clone();
                move |_, event: &multi_buffer::Event, _| {
                    if let multi_buffer::Event::BuffersRemoved { removed_buffer_ids } = event {
                        removed_buffers
                            .borrow_mut()
                            .extend(removed_buffer_ids.iter().copied());
                    }
                }
            })
        });

        search.update(cx, |search, cx| {
            search.search(build_query(), SearchMode::OnType, true, cx);
        });
        cx.run_until_parked();
        assert_eq!(
            search.read_with(cx, |search, cx| search
                .excerpts
                .read(cx)
                .snapshot(cx)
                .text()),
            text,
        );
        assert_eq!(match_count(&search, cx), 3);
        assert_eq!(*removed_buffers.borrow(), Vec::<language::BufferId>::new());
    }

    #[gpui::test]
    async fn test_opened_only_search_deduplicates_buffers(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(path!("/dir"), json!({ "one.rs": "fn needle() {}" }))
            .await;
        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/dir/one.rs"), cx)
            })
            .await
            .unwrap();
        let search =
            cx.new(|cx| ProjectSearch::new(project.clone(), WeakEntity::new_invalid(), cx));

        let query = SearchOptions::NONE
            .build_query(
                "needle",
                PathMatcher::default(),
                PathMatcher::default(),
                false,
                Some(vec![buffer.clone(), buffer.clone(), buffer]),
            )
            .unwrap();
        search.update(cx, |search, cx| {
            search.search(query, SearchMode::Manual, false, cx);
        });
        cx.run_until_parked();

        assert_eq!(
            match_count(&search, cx),
            1,
            "a buffer open in multiple panes must be searched once, not once per pane"
        );
        assert_eq!(match_texts(&search, cx), vec!["needle"]);
    }

    #[gpui::test]
    async fn test_pruning_stale_excerpts_keeps_scroll_at_seam(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/dir"),
            json!({
                "a.txt": "\n\n\n\n\nAA\n\n\n\n\n",
                "b.txt": "\n\n\n\n\n A \n\n\n\n\n",
                "c.txt": "\n\n\n\n\n A \n\n\n\n\n",
                "d.txt": "\n\n\n\n\n A \n\n\n\n\n",
                "e.txt": "\n\n\n\n\n A \n\n\n\n\n",
                "f.txt": "\n\n\n\n\n A \n\n\n\n\n",
                "g.txt": "\n\n\n\n\n A \n\n\n\n\n",
                "h.txt": "\n\n\n\n\n A \n\n\n\n\n",
                "i.txt": "\n\n\n\n\n A \n\n\n\n\n",
                "j.txt": "\n\n\n\n\n A \n\n\n\n\n",
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let window =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let search = cx.new(|cx| ProjectSearch::new(project, workspace.downgrade(), cx));
        let search_view = cx.add_window(|window, cx| {
            ProjectSearchView::new(workspace.downgrade(), search.clone(), window, cx, None)
        });

        perform_search(search_view, "A", cx);
        search_view
            .update(cx, |search_view, window, cx| {
                search_view.results_editor.update(cx, |results_editor, cx| {
                    results_editor.scroll(Point::new(0., f64::MAX), window, cx);
                    assert!(
                        results_editor.scroll_position(cx).y > 0.,
                        "results should be long enough to scroll down"
                    );
                });
            })
            .unwrap();

        perform_incremental_search(search_view, "AA", cx);
        assert_eq!(match_count(&search, cx), 1);
        search_view
            .update(cx, |search_view, _, cx| {
                search_view.results_editor.update(cx, |results_editor, cx| {
                    assert!(
                        results_editor.scroll_position(cx).y > 0.,
                        "pruning the excerpt under the scroll anchor must not reset the scroll \
                         to the top"
                    );
                });
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_erase_and_retype_within_debounce_keeps_results(cx: &mut TestAppContext) {
        init_test(cx);
        enable_search_on_type(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/dir"),
            json!({
                "1.txt": "\n\n\n\n\n A \n\n\n\n\n",
                "2.txt": "\n\n\n\n\n A \n\n\n\n\n",
                "3.rs": "\n\n\n\n\n A \n\n\n\n\n",
                "4.rs": "\n\n\n\n\n A \n\n\n\n\n",
                "5.rs": "\n\n\n\n\n A \n\n\n\n\n",
                "6.rs": "\n\n\n\n\n A \n\n\n\n\n",
                "7.rs": "\n\n\n\n\n A \n\n\n\n\n",
                "8.rs": "\n\n\n\n\n A \n\n\n\n\n",
                "9.rs": "\n\n\n\n\n A \n\n\n\n\n",
                "a.rs": "\n\n\n\n\n A \n\n\n\n\n",
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let window =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let search = cx.new(|cx| ProjectSearch::new(project, workspace.downgrade(), cx));
        let search_view = cx.add_window(|window, cx| {
            ProjectSearchView::new(workspace.downgrade(), search.clone(), window, cx, None)
        });

        search_view
            .update(cx, |search_view, window, cx| {
                search_view.query_editor.update(cx, |query_editor, cx| {
                    query_editor.set_text("A", window, cx)
                });
            })
            .unwrap();
        cx.executor()
            .advance_clock(SEARCH_ON_TYPE_DEBOUNCE + Duration::from_millis(50));
        cx.background_executor.run_until_parked();
        assert_eq!(match_count(&search, cx), 10);
        search_view
            .update(cx, |search_view, window, cx| {
                search_view.results_editor.update(cx, |results_editor, cx| {
                    results_editor.scroll(Point::new(0., f64::MAX), window, cx);
                    assert!(
                        results_editor.scroll_position(cx).y > 0.,
                        "results should be long enough to scroll down"
                    );
                });
            })
            .unwrap();

        search_view
            .update(cx, |search_view, window, cx| {
                search_view
                    .query_editor
                    .update(cx, |query_editor, cx| query_editor.set_text("", window, cx));
            })
            .unwrap();
        search_view
            .update(cx, |search_view, window, cx| {
                search_view.query_editor.update(cx, |query_editor, cx| {
                    query_editor.set_text("A", window, cx)
                });
            })
            .unwrap();
        cx.executor()
            .advance_clock(SEARCH_ON_TYPE_DEBOUNCE + Duration::from_millis(50));
        cx.background_executor.run_until_parked();

        assert_eq!(match_count(&search, cx), 10);
        search_view
            .update(cx, |search_view, _, cx| {
                search_view.results_editor.update(cx, |results_editor, cx| {
                    assert!(
                        results_editor.scroll_position(cx).y > 0.,
                        "erasing and retyping the query within the debounce must not clear the \
                         results pane in between, so the scroll position survives"
                    );
                });
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_confirm_from_filter_editor_keeps_focus_there(cx: &mut TestAppContext) {
        init_test(cx);
        enable_search_on_type(cx);

        let SearchBarTest {
            window,
            search_bar,
            search_view,
            cx,
            ..
        } = &mut setup_search_bar_test(
            json!({
                "one.rs": "const ONE: usize = 1;",
                "two.rs": "const TWO: usize = one::ONE + one::ONE;",
            }),
            cx,
        )
        .await;

        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    search_view.toggle_filters(cx);
                    search_view.query_editor.update(cx, |query_editor, cx| {
                        query_editor.set_text("ONE", window, cx)
                    });
                });
            })
            .unwrap();
        cx.background_executor
            .advance_clock(SEARCH_ON_TYPE_DEBOUNCE + Duration::from_millis(50));
        cx.background_executor.run_until_parked();
        assert!(cx.read(|cx| !search_view.read(cx).entity.read(cx).match_ranges.is_empty()));

        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    let filter_handle = search_view.included_files_editor.focus_handle(cx);
                    window.focus(&filter_handle, cx);
                });
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.confirm(&Confirm, window, cx);
                });
            })
            .unwrap();
        cx.background_executor.run_until_parked();

        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    assert!(
                        search_view
                            .included_files_editor
                            .focus_handle(cx)
                            .is_focused(window),
                        "confirming from a filter editor must not yank focus to the results"
                    );
                    assert!(
                        !search_view
                            .results_editor
                            .focus_handle(cx)
                            .is_focused(window),
                    );
                });
            })
            .unwrap();
    }

    #[gpui::test]
    async fn test_toggling_filters_panel_researches_on_type(cx: &mut TestAppContext) {
        init_test(cx);
        enable_search_on_type(cx);

        let SearchBarTest {
            window,
            search_bar,
            search_view,
            cx,
            ..
        } = &mut setup_search_bar_test(
            json!({
                "one.rs": "const ONE: usize = 1;",
                "two.rs": "const ONE_IN_TWO: usize = 1;",
            }),
            cx,
        )
        .await;
        let search = cx.read(|cx| search_view.read(cx).entity.clone());

        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    search_view.toggle_filters(cx);
                    search_view
                        .excluded_files_editor
                        .update(cx, |editor, cx| editor.set_text("one.rs", window, cx));
                    search_view.query_editor.update(cx, |query_editor, cx| {
                        query_editor.set_text("ONE", window, cx)
                    });
                });
            })
            .unwrap();
        cx.background_executor
            .advance_clock(SEARCH_ON_TYPE_DEBOUNCE + Duration::from_millis(50));
        cx.background_executor.run_until_parked();
        assert_eq!(match_count(&search, cx), 1);
        assert_eq!(matched_file_names(&search, cx), vec!["two.rs"]);

        window
            .update(cx, |_, window, cx| {
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.toggle_filters(window, cx);
                });
            })
            .unwrap();
        cx.background_executor.run_until_parked();
        assert_eq!(
            match_count(&search, cx),
            2,
            "disabling the filters panel must re-run the search without the exclusions, \
             without waiting for Enter"
        );
        assert_eq!(matched_file_names(&search, cx), vec!["one.rs", "two.rs"]);
    }

    #[gpui::test]
    async fn test_focusing_results_records_history(cx: &mut TestAppContext) {
        init_test(cx);
        enable_search_on_type(cx);

        let SearchBarTest {
            project,
            window,
            search_view,
            cx,
            ..
        } = &mut setup_search_bar_test(json!({ "one.rs": "const ONE: usize = 1;" }), cx).await;

        let read_query_history = |cx: &mut VisualTestContext| {
            cx.read(|cx| {
                project
                    .read(cx)
                    .search_history(SearchInputKind::Query)
                    .iter()
                    .map(str::to_string)
                    .collect::<Vec<_>>()
            })
        };

        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    search_view.query_editor.update(cx, |query_editor, cx| {
                        query_editor.set_text("ONE", window, cx)
                    });
                });
            })
            .unwrap();
        cx.background_executor
            .advance_clock(SEARCH_ON_TYPE_DEBOUNCE + Duration::from_millis(50));
        cx.background_executor.run_until_parked();
        assert_eq!(read_query_history(cx), Vec::<String>::new());

        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    search_view.search(SearchMode::OnType, cx);
                    assert!(
                        search_view.entity.read(cx).pending_search.is_some(),
                        "the re-search should be pending"
                    );
                    let results_handle = search_view.results_editor.focus_handle(cx);
                    window.focus(&results_handle, cx);
                });
            })
            .unwrap();
        cx.background_executor.run_until_parked();
        assert_eq!(
            read_query_history(cx),
            vec!["ONE".to_string()],
            "engaging with the results while the search is still pending must record it in \
             the history"
        );
        cx.read(|cx| {
            let search_view = search_view.read(cx);
            let model = search_view.entity.read(cx);
            assert_eq!(
                search_view.search_id, model.search_id,
                "confirming by focus must sync the search id so the settled search does not \
                 yank the selection away from where the user clicked"
            );
            assert_eq!(model.phase, SearchPhase::Confirmed);
        });

        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    let query_handle = search_view.query_editor.focus_handle(cx);
                    window.focus(&query_handle, cx);
                    let results_handle = search_view.results_editor.focus_handle(cx);
                    window.focus(&results_handle, cx);
                });
            })
            .unwrap();
        cx.background_executor.run_until_parked();
        assert_eq!(
            read_query_history(cx),
            vec!["ONE".to_string()],
            "re-focusing the results must not duplicate the history entry"
        );
    }

    #[gpui::test]
    async fn test_confirm_ignores_filter_text_while_filters_are_disabled(cx: &mut TestAppContext) {
        init_test(cx);
        enable_search_on_type(cx);

        let SearchBarTest {
            window,
            search_bar,
            search_view,
            cx,
            ..
        } = &mut setup_search_bar_test(json!({ "one.rs": "const ONE: usize = 1;" }), cx).await;
        let search = cx.read(|cx| search_view.read(cx).entity.clone());

        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    search_view.query_editor.update(cx, |query_editor, cx| {
                        query_editor.set_text("ONE", window, cx)
                    });
                });
            })
            .unwrap();
        cx.background_executor
            .advance_clock(SEARCH_ON_TYPE_DEBOUNCE + Duration::from_millis(50));
        cx.background_executor.run_until_parked();
        let search_id_after_typing = cx.read(|cx| search_view.read(cx).entity.read(cx).search_id);

        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    search_view
                        .excluded_files_editor
                        .update(cx, |editor, cx| editor.set_text("one.rs", window, cx));
                });
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.confirm(&Confirm, window, cx);
                });
            })
            .unwrap();
        cx.background_executor.run_until_parked();
        assert_eq!(
            cx.read(|cx| search_view.read(cx).entity.read(cx).search_id),
            search_id_after_typing + 1,
            "confirming re-runs the search so it can pick up changed files"
        );
        assert_eq!(
            match_count(&search, cx),
            1,
            "filter text edited while the filters panel is disabled must not be applied"
        );
        assert_eq!(matched_file_names(&search, cx), vec!["one.rs"]);
    }

    #[gpui::test]
    async fn test_confirm_researches_the_same_query(cx: &mut TestAppContext) {
        init_test(cx);
        enable_search_on_type(cx);

        let SearchBarTest {
            project,
            window,
            search_bar,
            search_view,
            cx,
        } = &mut setup_search_bar_test(
            json!({
                "one.rs": "const ONE: usize = 1;",
                "two.rs": "const TWO: usize = 2;",
            }),
            cx,
        )
        .await;
        let search = cx.read(|cx| search_view.read(cx).entity.clone());

        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    search_view.query_editor.update(cx, |query_editor, cx| {
                        query_editor.set_text("ONE", window, cx)
                    });
                });
            })
            .unwrap();
        cx.background_executor
            .advance_clock(SEARCH_ON_TYPE_DEBOUNCE + Duration::from_millis(50));
        cx.background_executor.run_until_parked();
        assert_eq!(match_count(&search, cx), 1);

        let buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/dir/two.rs"), cx)
            })
            .await
            .unwrap();
        buffer.update(cx, |buffer, cx| {
            buffer.edit([(0..0, "const ONE_MORE: usize = 1;\n")], None, cx)
        });

        window
            .update(cx, |_, window, cx| {
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.confirm(&Confirm, window, cx);
                });
            })
            .unwrap();
        cx.background_executor.run_until_parked();
        assert_eq!(
            match_count(&search, cx),
            2,
            "confirming an unchanged query must re-run the search and pick up new matches"
        );
        assert_eq!(matched_file_names(&search, cx), vec!["one.rs", "two.rs"]);
    }

    #[gpui::test]
    async fn test_debounced_search_skips_unchanged_query(cx: &mut TestAppContext) {
        init_test(cx);
        enable_search_on_type(cx);

        let SearchBarTest {
            window,
            search_bar,
            search_view,
            cx,
            ..
        } = &mut setup_search_bar_test(json!({ "one.rs": "const ONE: usize = 1;" }), cx).await;

        let type_query = |query: &str, cx: &mut VisualTestContext| {
            window
                .update(cx, |_, window, cx| {
                    search_view.update(cx, |search_view, cx| {
                        search_view.query_editor.update(cx, |query_editor, cx| {
                            query_editor.set_text(query, window, cx)
                        });
                    });
                })
                .unwrap();
        };
        let read_search_id = |cx: &mut VisualTestContext| {
            cx.read(|cx| search_view.read(cx).entity.read(cx).search_id)
        };

        type_query("ONE", cx);
        cx.background_executor
            .advance_clock(SEARCH_ON_TYPE_DEBOUNCE + Duration::from_millis(50));
        cx.background_executor.run_until_parked();
        let search_id_after_typing = read_search_id(cx);

        type_query("ON", cx);
        type_query("ONE", cx);
        cx.background_executor
            .advance_clock(SEARCH_ON_TYPE_DEBOUNCE + Duration::from_millis(50));
        cx.background_executor.run_until_parked();
        assert_eq!(
            read_search_id(cx),
            search_id_after_typing,
            "a debounced search whose inputs ended up unchanged must be skipped, \
             the results on screen are already correct"
        );

        window
            .update(cx, |_, window, cx| {
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.confirm(&Confirm, window, cx);
                });
            })
            .unwrap();
        cx.background_executor.run_until_parked();
        assert_eq!(
            read_search_id(cx),
            search_id_after_typing + 1,
            "Enter stays a forced refresh even for an unchanged query"
        );
    }

    #[gpui::test]
    async fn test_replace_next_defers_while_search_is_pending(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/dir"),
            json!({
                "one.rs": "const ONE: usize = 1;",
                "two.rs": "const ONE_TWO: usize = 2;",
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let window =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let search = cx.new(|cx| ProjectSearch::new(project.clone(), workspace.downgrade(), cx));
        let search_view = cx.add_window(|window, cx| {
            ProjectSearchView::new(workspace.downgrade(), search.clone(), window, cx, None)
        });

        perform_search(search_view, "ONE", cx);
        assert_eq!(match_count(&search, cx), 2);

        let buffer_text = |cx: &mut TestAppContext| {
            search.read_with(cx, |search, cx| {
                search
                    .excerpts
                    .read(cx)
                    .all_buffers_iter()
                    .find_map(|buffer| {
                        let buffer = buffer.read(cx);
                        (buffer.file()?.path().as_ref() == rel_path("one.rs"))
                            .then(|| buffer.text())
                    })
            })
        };

        search_view
            .update(cx, |search_view, window, cx| {
                search_view
                    .replacement_editor
                    .update(cx, |editor, cx| editor.set_text("NEW", window, cx));
                search_view.search(SearchMode::OnType, cx);
                search_view.replace_next(&ReplaceNext, window, cx);
                assert!(
                    !search_view.pending_replace_next,
                    "a replacement during an unconfirmed on-type search must be dropped, not \
                     deferred, so it cannot fire against results the user never confirmed"
                );
            })
            .unwrap();
        cx.background_executor.run_until_parked();
        assert_eq!(
            buffer_text(cx),
            Some("const ONE: usize = 1;".to_string()),
            "no replacement must run for the dropped request"
        );

        search_view
            .update(cx, |search_view, window, cx| {
                search_view.search(SearchMode::Manual, cx);
                search_view.replace_next(&ReplaceNext, window, cx);
                assert!(search_view.pending_replace_next);
            })
            .unwrap();
        assert_eq!(
            buffer_text(cx),
            Some("const ONE: usize = 1;".to_string()),
            "the replacement must wait for the pending search instead of silently doing nothing"
        );

        cx.background_executor.run_until_parked();
        assert_eq!(
            buffer_text(cx),
            Some("const NEW: usize = 1;".to_string()),
            "the deferred replacement runs once the confirmed search settles"
        );
    }

    #[gpui::test]
    async fn test_search_on_type_pauses_while_results_are_dirty(cx: &mut TestAppContext) {
        init_test(cx);
        enable_search_on_type(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/dir"),
            json!({
                "one.rs": "const ONE: usize = 1;",
                "two.rs": "const TWO: usize = 2;",
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let window =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let search = cx.new(|cx| ProjectSearch::new(project.clone(), workspace.downgrade(), cx));
        let search_view = cx.add_window(|window, cx| {
            ProjectSearchView::new(workspace.downgrade(), search.clone(), window, cx, None)
        });

        let type_query = |query: &str, cx: &mut TestAppContext| {
            search_view
                .update(cx, |search_view, window, cx| {
                    search_view.query_editor.update(cx, |query_editor, cx| {
                        query_editor.set_text(query, window, cx)
                    });
                })
                .unwrap();
            cx.executor()
                .advance_clock(SEARCH_ON_TYPE_DEBOUNCE + Duration::from_millis(50));
            cx.background_executor.run_until_parked();
        };

        type_query("ONE", cx);
        assert_eq!(match_count(&search, cx), 1);
        let search_id_after_typing = search.read_with(cx, |search, _| search.search_id);

        search_view
            .update(cx, |search_view, window, cx| {
                search_view.results_editor.update(cx, |results_editor, cx| {
                    results_editor.insert("edited", window, cx);
                });
                assert!(search_view.is_dirty(cx));
            })
            .unwrap();

        type_query("TWO", cx);
        assert_eq!(
            search.read_with(cx, |search, _| search.search_id),
            search_id_after_typing,
            "search-on-type must pause while the results hold unsaved edits, otherwise it \
             silently replaces the dirty excerpts without any save prompt"
        );
        assert_eq!(match_count(&search, cx), 1);

        type_query("", cx);
        assert_eq!(
            match_count(&search, cx),
            1,
            "erasing the query must not clear dirty results either"
        );

        search_view
            .update(cx, |search_view, window, cx| {
                search_view.results_editor.update(cx, |results_editor, cx| {
                    results_editor.undo(&editor::actions::Undo, window, cx);
                });
                assert!(!search_view.is_dirty(cx));
                search_view.query_editor.update(cx, |query_editor, cx| {
                    query_editor.set_text("TWO", window, cx)
                });
            })
            .unwrap();
        cx.executor()
            .advance_clock(SEARCH_ON_TYPE_DEBOUNCE + Duration::from_millis(50));
        cx.background_executor.run_until_parked();
        assert_eq!(
            match_texts(&search, cx),
            vec!["TWO"],
            "once the results are clean again, search-on-type resumes"
        );
    }

    #[gpui::test]
    async fn test_dirtying_results_within_erase_debounce_keeps_them(cx: &mut TestAppContext) {
        init_test(cx);
        enable_search_on_type(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(path!("/dir"), json!({ "one.rs": "const ONE: usize = 1;" }))
            .await;
        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let window =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let search = cx.new(|cx| ProjectSearch::new(project.clone(), workspace.downgrade(), cx));
        let search_view = cx.add_window(|window, cx| {
            ProjectSearchView::new(workspace.downgrade(), search.clone(), window, cx, None)
        });

        search_view
            .update(cx, |search_view, window, cx| {
                search_view.query_editor.update(cx, |query_editor, cx| {
                    query_editor.set_text("ONE", window, cx)
                });
            })
            .unwrap();
        cx.executor()
            .advance_clock(SEARCH_ON_TYPE_DEBOUNCE + Duration::from_millis(50));
        cx.background_executor.run_until_parked();
        assert_eq!(match_count(&search, cx), 1);

        search_view
            .update(cx, |search_view, window, cx| {
                search_view
                    .query_editor
                    .update(cx, |query_editor, cx| query_editor.set_text("", window, cx));
            })
            .unwrap();
        search_view
            .update(cx, |search_view, window, cx| {
                search_view.results_editor.update(cx, |results_editor, cx| {
                    results_editor.insert("edited", window, cx);
                });
                assert!(search_view.is_dirty(cx));
            })
            .unwrap();
        cx.executor()
            .advance_clock(SEARCH_ON_TYPE_DEBOUNCE + Duration::from_millis(50));
        cx.background_executor.run_until_parked();
        assert_eq!(
            match_count(&search, cx),
            1,
            "results made dirty after the erase was scheduled but before the debounce fired \
             must survive, the clear task has to re-check for unsaved edits when it runs"
        );
        assert!(
            search.read_with(cx, |search, _| search.active_query.is_some()),
            "the model must not be cleared while it shows dirty excerpts"
        );
    }

    #[gpui::test]
    async fn test_incremental_search_merges_chunks_larger_than_a_foreground_batch(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);

        let mut files = serde_json::Map::new();
        for index in 0..70 {
            files.insert(
                format!("file_{index:03}.txt"),
                json!(format!("needle {index:03}")),
            );
        }
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(path!("/dir"), serde_json::Value::Object(files))
            .await;
        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let window =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let search = cx.new(|cx| ProjectSearch::new(project, workspace.downgrade(), cx));
        let search_view = cx.add_window(|window, cx| {
            ProjectSearchView::new(workspace.downgrade(), search.clone(), window, cx, None)
        });

        perform_search(search_view, "needle", cx);
        assert_eq!(match_count(&search, cx), 70);

        perform_incremental_search(search_view, "needle", cx);
        assert_eq!(
            match_count(&search, cx),
            70,
            "a reused chunk larger than one foreground batch must merge fully"
        );
        assert_eq!(
            matched_file_names(&search, cx).len(),
            70,
            "every file must keep its excerpt across the batched reuse"
        );

        perform_incremental_search(search_view, "needle 000", cx);
        assert_eq!(
            matched_file_names(&search, cx),
            vec!["file_000.txt"],
            "narrowing must prune the stale files across all foreground batches"
        );
    }

    #[gpui::test]
    async fn test_scroll_range_held_while_search_is_pending(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(path!("/dir"), json!({ "one.rs": "const ONE: usize = 1;" }))
            .await;
        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let window =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let search = cx.new(|cx| ProjectSearch::new(project.clone(), workspace.downgrade(), cx));
        let search_view = cx.add_window(|window, cx| {
            ProjectSearchView::new(workspace.downgrade(), search.clone(), window, cx, None)
        });

        let scroll_range_held = |cx: &mut TestAppContext| {
            search_view
                .read_with(cx, |search_view, cx| {
                    search_view
                        .results_editor
                        .read(cx)
                        .search_results_hold()
                        .map(|hold| hold.status.pending)
                })
                .unwrap()
        };

        assert_eq!(
            scroll_range_held(cx),
            Some(false),
            "the results editor opts into scroll range holds on creation and settles while idle"
        );

        search_view
            .update(cx, |search_view, window, cx| {
                search_view.query_editor.update(cx, |query_editor, cx| {
                    query_editor.set_text("ONE", window, cx)
                });
                search_view.search(SearchMode::OnType, cx);
            })
            .unwrap();
        assert_eq!(
            scroll_range_held(cx),
            Some(true),
            "the scroll range must be held for the whole pending search, so the \
             scrollbars cannot blink while excerpts churn"
        );

        cx.background_executor.run_until_parked();
        assert_eq!(
            scroll_range_held(cx),
            Some(false),
            "the scroll range settles again once the search completes"
        );
    }

    #[gpui::test]
    async fn test_incremental_search_prunes_excerpts_without_recorded_matches(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/dir"),
            json!({
                "aaa.rs": "fn needle() {}",
                "zzz.rs": "fn unrelated() {}",
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let window =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let search = cx.new(|cx| ProjectSearch::new(project.clone(), workspace.downgrade(), cx));
        let search_view = cx.add_window(|window, cx| {
            ProjectSearchView::new(workspace.downgrade(), search.clone(), window, cx, None)
        });

        perform_search(search_view, "needle", cx);
        assert_eq!(matched_file_names(&search, cx), vec!["aaa.rs"]);

        let orphan_buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/dir/zzz.rs"), cx)
            })
            .await
            .unwrap();
        search.update(cx, |search, cx| {
            search.excerpts.update(cx, |excerpts, cx| {
                excerpts.set_excerpts_for_path(
                    PathKey::for_buffer(&orphan_buffer, cx),
                    orphan_buffer.clone(),
                    vec![text::Point::new(0, 0)..text::Point::new(0, 1)],
                    2,
                    cx,
                );
            });
        });
        assert_eq!(matched_file_names(&search, cx), vec!["aaa.rs", "zzz.rs"]);

        perform_incremental_search(search_view, "needle", cx);
        assert_eq!(
            matched_file_names(&search, cx),
            vec!["aaa.rs"],
            "an excerpt present in the multibuffer but absent from the recorded matches, as \
             left behind by a cancelled search, must be pruned by the next reusing search"
        );
        assert_eq!(match_texts(&search, cx), vec!["needle"]);
    }

    #[gpui::test]
    async fn test_search_with_open_excluded_file_stays_path_key_sorted(cx: &mut TestAppContext) {
        init_test(cx);
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.project.worktree.file_scan_exclusions =
                        Some(SplicingVec::from(vec!["**/mmm.rs".to_string()]));
                });
            });
        });

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/dir"),
            json!({
                "aaa.rs": "fn needle_aa() {}",
                "mmm.rs": "fn needle_mm() {}",
                "zzz.rs": "fn needle_zz() {}",
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let _excluded_buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer(path!("/dir/mmm.rs"), cx)
            })
            .await
            .unwrap();
        project.read_with(cx, |project, cx| {
            let worktree = project.worktrees(cx).next().expect("worktree expected");
            assert_eq!(
                worktree
                    .read(cx)
                    .entry_for_path(rel_path("mmm.rs"))
                    .map(|entry| entry.id),
                None,
                "mmm.rs must have no worktree entry for this test to be meaningful"
            );
        });

        let window =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let search = cx.new(|cx| ProjectSearch::new(project.clone(), workspace.downgrade(), cx));
        let search_view = cx.add_window(|window, cx| {
            ProjectSearchView::new(workspace.downgrade(), search.clone(), window, cx, None)
        });
        search_view
            .update(cx, |search_view, _, _| {
                search_view.search_options.insert(SearchOptions::REGEX);
            })
            .unwrap();

        perform_search(search_view, "needle_[a-z]+", cx);
        assert_eq!(
            match_texts(&search, cx),
            vec!["needle_aa", "needle_mm", "needle_zz"],
            "the match for the open excluded file must land between the scanned files, in \
             PathKey order"
        );

        let updated_paths = Rc::new(RefCell::new(Vec::new()));
        let removed_buffers = Rc::new(RefCell::new(Vec::new()));
        let excerpts = search.read_with(cx, |search, _| search.excerpts.clone());
        let _subscription = cx.update(|cx| {
            cx.subscribe(&excerpts, {
                let updated_paths = updated_paths.clone();
                let removed_buffers = removed_buffers.clone();
                move |_, event: &multi_buffer::Event, _| match event {
                    multi_buffer::Event::BufferRangesUpdated { path_key, .. } => {
                        updated_paths.borrow_mut().push(path_key.clone());
                    }
                    multi_buffer::Event::BuffersRemoved { removed_buffer_ids } => {
                        removed_buffers
                            .borrow_mut()
                            .extend(removed_buffer_ids.iter().copied());
                    }
                    _ => {}
                }
            })
        });

        perform_incremental_search(search_view, "needle_[a-z]+", cx);
        assert_eq!(
            match_texts(&search, cx),
            vec!["needle_aa", "needle_mm", "needle_zz"]
        );
        assert_eq!(
            *removed_buffers.borrow(),
            Vec::<language::BufferId>::new(),
            "results arriving in PathKey order must reuse every excerpt instead of pruning \
             files ahead of the excluded file's early result"
        );
        assert_eq!(*updated_paths.borrow(), Vec::<PathKey>::new());
    }

    #[gpui::test]
    async fn test_reusing_search_tolerates_out_of_order_results(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/dir"),
            json!({
                "aaa.rs": "fn needle_aa() {}",
                "zzz.rs": "fn needle_zz() {}",
            }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let search =
            cx.new(|cx| ProjectSearch::new(project.clone(), WeakEntity::new_invalid(), cx));
        let query = SearchOptions::REGEX
            .build_query(
                "needle_[a-z]+",
                PathMatcher::default(),
                PathMatcher::default(),
                false,
                None,
            )
            .unwrap();
        search.update(cx, |search, cx| {
            search.search(query, SearchMode::Manual, false, cx);
        });
        cx.run_until_parked();
        assert_eq!(match_texts(&search, cx), vec!["needle_aa", "needle_zz"]);

        let buffer_for = |name: &str, cx: &mut TestAppContext| {
            search
                .read_with(cx, |search, cx| {
                    search.excerpts.read(cx).all_buffers_iter().find(|buffer| {
                        buffer
                            .read(cx)
                            .file()
                            .is_some_and(|file| file.path().as_ref() == rel_path(name))
                    })
                })
                .expect("buffer expected for a matched file")
        };
        let buffer_aaa = buffer_for("aaa.rs", cx);
        let buffer_zzz = buffer_for("zzz.rs", cx);
        let match_range = |buffer: &Entity<Buffer>, cx: &mut TestAppContext| {
            buffer.read_with(cx, |buffer, _| {
                buffer.snapshot().anchor_before(3)..buffer.snapshot().anchor_after(12)
            })
        };
        let range_aaa = match_range(&buffer_aaa, cx);
        let range_zzz = match_range(&buffer_zzz, cx);

        let mut reused_results = search.read_with(cx, |search, cx| ReusedResults::new(search, cx));
        let weak_search = search.downgrade();
        let mut async_cx = cx.to_async();
        apply_reused_chunk(
            &weak_search,
            vec![(buffer_zzz.clone(), vec![range_zzz.clone()])],
            &mut reused_results,
            &mut async_cx,
        )
        .await
        .expect("the search entity is alive");
        apply_reused_chunk(
            &weak_search,
            vec![(buffer_aaa, vec![range_aaa])],
            &mut reused_results,
            &mut async_cx,
        )
        .await
        .expect("the search entity is alive");
        apply_reused_chunk(
            &weak_search,
            vec![(buffer_zzz, vec![range_zzz])],
            &mut reused_results,
            &mut async_cx,
        )
        .await
        .expect("the search entity is alive");
        search.update(cx, |search, cx| reused_results.finish(search, cx));

        assert_eq!(matched_file_names(&search, cx), vec!["aaa.rs", "zzz.rs"]);
        assert_eq!(
            match_texts(&search, cx),
            vec!["needle_aa", "needle_zz"],
            "a result that arrives out of PathKey order must be spliced into its sorted \
             position instead of corrupting the match order, and a duplicate result for an \
             already confirmed buffer must replace its ranges instead of doubling them"
        );
    }

    #[gpui::test]
    async fn test_deploy_search_with_query_searches_on_type(cx: &mut TestAppContext) {
        init_test(cx);
        enable_search_on_type(cx);

        let SearchBarTest {
            project,
            window,
            search_view,
            cx,
            ..
        } = &mut setup_search_bar_test(
            json!({
                "one.rs": "const ONE: usize = 1;",
                "two.rs": "const TWO: usize = one::ONE + one::ONE;",
            }),
            cx,
        )
        .await;
        let workspace = window
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();

        workspace.update_in(cx, |workspace, window, cx| {
            ProjectSearchView::deploy_search(
                workspace,
                &DeploySearch {
                    query: Some("ONE".to_string()),
                    ..Default::default()
                },
                window,
                cx,
            );
        });
        cx.read(|cx| {
            let model = search_view.read(cx).entity.read(cx);
            assert!(
                model.pending_search.is_none(),
                "deploying with a query must only schedule the debounced search"
            );
            assert_eq!(model.match_ranges.len(), 0);
        });

        cx.background_executor
            .advance_clock(SEARCH_ON_TYPE_DEBOUNCE + Duration::from_millis(50));
        cx.background_executor.run_until_parked();

        window
            .update(cx, |_, window, cx| {
                search_view.update(cx, |search_view, cx| {
                    let model = search_view.entity.read(cx);
                    assert!(
                        !model.match_ranges.is_empty(),
                        "the seeded query must search on type after the debounce"
                    );
                    assert_eq!(
                        model.phase,
                        SearchPhase::Typing,
                        "a deploy-seeded search stays unconfirmed until the user engages with it"
                    );
                    assert!(
                        search_view.query_editor.focus_handle(cx).is_focused(window),
                        "deploying must keep the focus in the query editor"
                    );
                });
            })
            .unwrap();

        let query_history = cx.read(|cx| {
            project
                .read(cx)
                .search_history(SearchInputKind::Query)
                .iter()
                .map(str::to_string)
                .collect::<Vec<_>>()
        });
        assert_eq!(
            query_history,
            Vec::<String>::new(),
            "an unconfirmed deploy-seeded search must not enter the history"
        );
    }

    #[gpui::test]
    async fn test_on_type_search_keeps_selection_on_first_match(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        let mut files = serde_json::Map::new();
        for ix in 0..50 {
            files.insert(
                format!("file_{ix:03}.txt"),
                json!(format!("line with needle {ix}\nmore needle text {ix}")),
            );
        }
        fs.insert_tree(path!("/dir"), serde_json::Value::Object(files))
            .await;
        let project = Project::test(fs.clone(), [path!("/dir").as_ref()], cx).await;
        let window =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let search = cx.new(|cx| ProjectSearch::new(project.clone(), workspace.downgrade(), cx));
        let search_view = cx.add_window(|window, cx| {
            ProjectSearchView::new(workspace.downgrade(), search.clone(), window, cx, None)
        });

        let selection_head = |cx: &mut TestAppContext| {
            search_view
                .update(cx, |search_view, _, cx| {
                    search_view.results_editor.update(cx, |editor, cx| {
                        let snapshot = editor.display_snapshot(cx);
                        editor.selections.newest::<BufferPoint>(&snapshot).head()
                    })
                })
                .unwrap()
        };
        let first_match_head = |cx: &mut TestAppContext| {
            search_view
                .update(cx, |search_view, _, cx| {
                    let first_match = search_view
                        .entity
                        .read(cx)
                        .match_ranges
                        .first()
                        .expect("the query matches the fixture")
                        .clone();
                    search_view.results_editor.update(cx, |editor, cx| {
                        let snapshot = editor.display_snapshot(cx);
                        editor
                            .range_for_match(&first_match)
                            .end
                            .to_point(snapshot.buffer_snapshot())
                    })
                })
                .unwrap()
        };

        search_view
            .update(cx, |search_view, window, cx| {
                search_view.query_editor.update(cx, |query_editor, cx| {
                    query_editor.set_text("needle", window, cx)
                });
                search_view.search(SearchMode::Manual, cx);
            })
            .unwrap();
        cx.background_executor.run_until_parked();
        assert_eq!(
            selection_head(cx),
            first_match_head(cx),
            "a confirmed search selects the first match"
        );

        perform_incremental_search(search_view, "needle 4", cx);
        cx.background_executor.run_until_parked();
        assert_eq!(
            selection_head(cx),
            first_match_head(cx),
            "an on-type search must keep the selection on the first match too: a stale \
             anchor resolves to an arbitrary position, and relative line numbers measured \
             against it renumber unchanged excerpts on every keystroke"
        );
        assert_ne!(
            selection_head(cx),
            BufferPoint::zero(),
            "the fixture is arranged so the first match does not sit at the buffer start, \
             otherwise this test cannot tell a real selection from a collapsed stale anchor"
        );

        perform_incremental_search(search_view, "needle 41", cx);
        cx.background_executor.run_until_parked();
        assert_eq!(
            selection_head(cx),
            first_match_head(cx),
            "narrowing again keeps following the first match"
        );
    }

    fn perform_incremental_search(
        search_view: WindowHandle<ProjectSearchView>,
        text: impl Into<Arc<str>>,
        cx: &mut TestAppContext,
    ) {
        search_view
            .update(cx, |search_view, window, cx| {
                search_view.query_editor.update(cx, |query_editor, cx| {
                    query_editor.set_text(text, window, cx)
                });
                search_view.search(SearchMode::OnType, cx);
            })
            .unwrap();
        cx.executor().advance_clock(
            editor::SELECTION_HIGHLIGHT_DEBOUNCE_TIMEOUT + Duration::from_millis(100),
        );
        cx.background_executor.run_until_parked();
    }

    fn match_count(search: &Entity<ProjectSearch>, cx: &mut TestAppContext) -> usize {
        search.read_with(cx, |search, _| search.match_ranges.len())
    }

    fn match_texts(search: &Entity<ProjectSearch>, cx: &mut TestAppContext) -> Vec<String> {
        search.read_with(cx, |search, cx| {
            let snapshot = search.excerpts.read(cx).snapshot(cx);
            search
                .match_ranges
                .iter()
                .map(|range| snapshot.text_for_range(range.clone()).collect::<String>())
                .collect()
        })
    }

    fn matched_file_names(search: &Entity<ProjectSearch>, cx: &mut TestAppContext) -> Vec<String> {
        let mut file_names = search.read_with(cx, |search, cx| {
            search
                .excerpts
                .read(cx)
                .all_buffers_iter()
                .filter_map(|buffer| Some(buffer.read(cx).file()?.path().file_name()?.to_string()))
                .collect::<Vec<_>>()
        });
        file_names.sort();
        file_names
    }

    fn assert_all_highlights_match_query(
        search: &Entity<ProjectSearch>,
        query: &str,
        cx: &mut TestAppContext,
    ) {
        let match_texts = match_texts(search, cx);
        assert_eq!(
            match_texts.len(),
            match_count(search, cx),
            "match texts count should equal match_ranges count for query {query:?}"
        );
        for text in &match_texts {
            assert_eq!(
                text.to_uppercase(),
                query.to_uppercase(),
                "every highlighted range should match the query {query:?}"
            );
        }
    }

    fn reserved_gutter_digits(
        search_view: WindowHandle<ProjectSearchView>,
        cx: &mut TestAppContext,
    ) -> usize {
        search_view
            .read_with(cx, |search_view, cx| {
                search_view
                    .results_editor
                    .read(cx)
                    .search_results_hold()
                    .map_or(0, |hold| hold.min_line_number_digits)
            })
            .unwrap()
    }

    fn enable_search_on_type(cx: &mut TestAppContext) {
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings
                        .editor
                        .search
                        .get_or_insert_default()
                        .search_on_type = Some(true);
                });
            });
        });
    }

    fn active_search_view(
        workspace: &Entity<Workspace>,
        cx: &mut VisualTestContext,
    ) -> Entity<ProjectSearchView> {
        cx.read(|cx| {
            workspace
                .read(cx)
                .active_pane()
                .read(cx)
                .active_item()
                .and_then(|item| item.downcast::<ProjectSearchView>())
        })
        .expect("Search view expected to appear after new search event trigger")
    }

    struct SearchBarTest {
        project: Entity<Project>,
        window: WindowHandle<MultiWorkspace>,
        search_bar: Entity<ProjectSearchBar>,
        search_view: Entity<ProjectSearchView>,
        cx: VisualTestContext,
    }

    async fn setup_search_bar_test(
        files: serde_json::Value,
        cx: &mut TestAppContext,
    ) -> SearchBarTest {
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(path!("/dir"), files).await;
        let project = Project::test(fs, [path!("/dir").as_ref()], cx).await;
        let window =
            cx.add_window(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = window
            .read_with(cx, |mw, _| mw.workspace().clone())
            .unwrap();
        let mut cx = VisualTestContext::from_window(window.into(), cx);
        let search_bar = window.build_entity(&mut cx, |_, _| ProjectSearchBar::new());
        workspace.update_in(&mut cx, {
            let search_bar = search_bar.clone();
            |workspace, window, cx| {
                workspace.panes()[0].update(cx, |pane, cx| {
                    pane.toolbar()
                        .update(cx, |toolbar, cx| toolbar.add_item(search_bar, window, cx))
                });
                ProjectSearchView::new_search(workspace, &workspace::NewSearch, window, cx);
            }
        });
        let search_view = active_search_view(&workspace, &mut cx);
        SearchBarTest {
            project,
            window,
            search_bar,
            search_view,
            cx,
        }
    }
}
