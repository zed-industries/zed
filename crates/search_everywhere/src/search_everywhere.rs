mod query;
#[cfg(test)]
mod search_everywhere_tests;
mod text_search;

use std::{
    ops::Range,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use command_palette::humanize_action_name;
use command_palette_hooks::CommandPaletteFilter;
use editor::Editor;
use fuzzy_nucleo::{Case, LengthPenalty, PathMatch, StringMatchCandidate};
use gpui::{
    Action, App, ClickEvent, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable,
    Global, Render, SharedString, Subscription, Task, TaskExt, WeakEntity, Window, actions, rems,
};
use language::{Point, SymbolKind};
use picker::{ErasedEditor, Picker, PickerDelegate};
use project::{
    PathMatchCandidateSet, Project, ProjectPath, Symbol, WorktreeId, lsp_store::SymbolLocation,
};
use ui::{Divider, HighlightedLabel, ListItem, ListItemSpacing, prelude::*};
use util::ResultExt;
use workspace::{ModalView, Workspace};

use query::{LinePosition, parse_scope_prefix, split_line_suffix};
use text_search::TextMatch;

actions!(
    search_everywhere,
    [
        /// Opens the Search Everywhere modal, which searches classes, files,
        /// symbols, actions and text in one place.
        Toggle,
        /// Switches Search Everywhere to the next tab.
        NextTab,
        /// Switches Search Everywhere to the previous tab.
        PreviousTab,
    ]
);

const MODAL_WIDTH_REMS: f32 = 48.;
const ALL_TAB_GROUP_LIMIT: usize = 8;
const TAB_LIMIT: usize = 100;
const RECENT_FILES_LIMIT: usize = 20;
// Full-text search is expensive, so wait for the user to stop typing.
const TEXT_SEARCH_DEBOUNCE: Duration = Duration::from_millis(150);

pub fn init(cx: &mut App) {
    cx.set_global(LastSearch::default());
    cx.observe_new(SearchEverywhere::register).detach();
}

#[derive(Default)]
struct LastSearch {
    tab: SearchTab,
    query: String,
}

impl Global for LastSearch {}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SearchTab {
    #[default]
    All,
    Classes,
    Files,
    Symbols,
    Actions,
    Text,
}

impl SearchTab {
    const ALL: [SearchTab; 6] = [
        SearchTab::All,
        SearchTab::Classes,
        SearchTab::Files,
        SearchTab::Symbols,
        SearchTab::Actions,
        SearchTab::Text,
    ];

    fn label(self) -> &'static str {
        match self {
            SearchTab::All => "All",
            SearchTab::Classes => "Classes",
            SearchTab::Files => "Files",
            SearchTab::Symbols => "Symbols",
            SearchTab::Actions => "Actions",
            SearchTab::Text => "Text",
        }
    }

    fn cycle(self, delta: isize) -> Self {
        let count = Self::ALL.len() as isize;
        let index = Self::ALL.iter().position(|tab| *tab == self).unwrap_or(0) as isize;
        Self::ALL
            .get((index + delta).rem_euclid(count) as usize)
            .copied()
            .unwrap_or(self)
    }

    fn includes_files(self) -> bool {
        matches!(self, SearchTab::All | SearchTab::Files)
    }

    fn includes_symbols(self) -> bool {
        matches!(
            self,
            SearchTab::All | SearchTab::Classes | SearchTab::Symbols
        )
    }

    fn includes_actions(self) -> bool {
        matches!(self, SearchTab::All | SearchTab::Actions)
    }
}

fn is_class_like(kind: SymbolKind) -> bool {
    matches!(
        kind,
        SymbolKind::Class | SymbolKind::Struct | SymbolKind::Interface | SymbolKind::Enum
    )
}

pub struct SearchEverywhere {
    picker: Entity<Picker<SearchEverywhereDelegate>>,
    _subscriptions: Vec<Subscription>,
}

impl ModalView for SearchEverywhere {}

impl EventEmitter<DismissEvent> for SearchEverywhere {}

impl Focusable for SearchEverywhere {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl SearchEverywhere {
    fn register(
        workspace: &mut Workspace,
        _window: Option<&mut Window>,
        _: &mut Context<Workspace>,
    ) {
        workspace.register_action(|workspace, _: &Toggle, window, cx| {
            if workspace.active_modal::<Self>(cx).is_some() {
                workspace.hide_modal(window, cx);
                return;
            }
            let Some(previous_focus_handle) = window.focused(cx) else {
                return;
            };
            let project = workspace.project().clone();
            let weak_workspace = cx.entity().downgrade();
            // The picker runs its first search while the workspace is still being
            // updated, so it can't read the navigation history itself.
            let recent_paths = workspace
                .recent_navigation_history_iter(cx)
                .map(|(project_path, _)| project_path)
                .take(RECENT_FILES_LIMIT)
                .collect();
            workspace.toggle_modal(window, cx, move |window, cx| {
                Self::new(
                    weak_workspace,
                    project,
                    previous_focus_handle,
                    recent_paths,
                    window,
                    cx,
                )
            });
        });
    }

    fn new(
        workspace: WeakEntity<Workspace>,
        project: Entity<Project>,
        previous_focus_handle: FocusHandle,
        recent_paths: Vec<ProjectPath>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        // Collected while focus is still on the previously focused element, so we
        // get the same actions the command palette would offer.
        let filter = CommandPaletteFilter::try_global(cx);
        let mut actions: Vec<ActionEntry> = window
            .available_actions(cx)
            .into_iter()
            .filter(|action| !filter.is_some_and(|filter| filter.is_hidden(&**action)))
            .map(|action| ActionEntry {
                name: humanize_action_name(action.name()).into(),
                action,
            })
            .collect();
        actions.sort_by(|left, right| left.name.cmp(&right.name));

        let (last_tab, last_query) = cx
            .try_global::<LastSearch>()
            .map(|last_search| (last_search.tab, last_search.query.clone()))
            .unwrap_or_default();

        let mut delegate = SearchEverywhereDelegate::new(
            workspace,
            project,
            previous_focus_handle,
            actions,
            recent_paths,
        );
        delegate.tab = last_tab;
        let picker = cx.new(|cx| {
            Picker::uniform_list(delegate, window, cx).initial_width(rems(MODAL_WIDTH_REMS))
        });
        if !last_query.is_empty() {
            picker.update(cx, |picker, cx| {
                picker.set_query(&last_query, window, cx);
                picker.select_query(window, cx);
            });
        }

        let dismiss_subscription = cx.subscribe(&picker, |_, _, _: &DismissEvent, cx| {
            cx.emit(DismissEvent);
        });
        Self {
            picker,
            _subscriptions: vec![dismiss_subscription],
        }
    }

    fn next_tab(&mut self, _: &NextTab, window: &mut Window, cx: &mut Context<Self>) {
        self.picker.update(cx, |picker, cx| {
            let tab = picker.delegate.tab.cycle(1);
            switch_tab(picker, tab, window, cx);
        });
    }

    fn previous_tab(&mut self, _: &PreviousTab, window: &mut Window, cx: &mut Context<Self>) {
        self.picker.update(cx, |picker, cx| {
            let tab = picker.delegate.tab.cycle(-1);
            switch_tab(picker, tab, window, cx);
        });
    }
}

impl Render for SearchEverywhere {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("SearchEverywhere")
            .on_action(cx.listener(Self::next_tab))
            .on_action(cx.listener(Self::previous_tab))
            .child(self.picker.clone())
    }
}

fn switch_tab(
    picker: &mut Picker<SearchEverywhereDelegate>,
    tab: SearchTab,
    window: &mut Window,
    cx: &mut Context<Picker<SearchEverywhereDelegate>>,
) {
    if picker.delegate.tab == tab {
        return;
    }
    picker.delegate.tab = tab;
    picker.refresh_placeholder(window, cx);
    picker.refresh(window, cx);
}

struct ActionEntry {
    name: SharedString,
    action: Box<dyn Action>,
}

enum SearchMatch {
    File(PathMatch),
    Symbol {
        symbol: Symbol,
        positions: Vec<usize>,
        score: f64,
    },
    Action {
        action_index: usize,
        positions: Vec<usize>,
        score: f64,
    },
    Text(TextMatch),
    More(Group),
}

impl SearchMatch {
    fn score(&self) -> f64 {
        match self {
            SearchMatch::File(path_match) => path_match.score,
            SearchMatch::Symbol { score, .. } | SearchMatch::Action { score, .. } => *score,
            SearchMatch::Text(_) | SearchMatch::More(_) => 0.,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Group {
    Classes,
    Files,
    Symbols,
    Actions,
    Text,
}

impl Group {
    fn label(self) -> &'static str {
        match self {
            Group::Classes => "Class",
            Group::Files => "File",
            Group::Symbols => "Symbol",
            Group::Actions => "Action",
            Group::Text => "Text",
        }
    }

    fn tab(self) -> SearchTab {
        match self {
            Group::Classes => SearchTab::Classes,
            Group::Files => SearchTab::Files,
            Group::Symbols => SearchTab::Symbols,
            Group::Actions => SearchTab::Actions,
            Group::Text => SearchTab::Text,
        }
    }
}

#[derive(Default)]
struct GroupedResults {
    groups: Vec<(Group, Vec<SearchMatch>)>,
}

impl GroupedResults {
    fn push(&mut self, group: Group, items: Vec<SearchMatch>) {
        if !items.is_empty() {
            self.groups.push((group, items));
        }
    }
}

// Like IntelliJ, the group with the best match comes first. The sort is stable,
// so equally good groups keep the Classes → Files → Symbols → Actions order.
fn sort_groups_by_relevance(groups: &mut [(Group, Vec<SearchMatch>)]) {
    let best_score = |items: &[SearchMatch]| {
        items
            .iter()
            .map(SearchMatch::score)
            .fold(f64::NEG_INFINITY, f64::max)
    };
    groups.sort_by(|(_, left), (_, right)| best_score(right).total_cmp(&best_score(left)));
}

pub struct SearchEverywhereDelegate {
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    previous_focus_handle: FocusHandle,
    actions: Vec<ActionEntry>,
    recent_paths: Vec<ProjectPath>,
    tab: SearchTab,
    matches: Vec<SearchMatch>,
    groups: Vec<(Group, Range<usize>)>,
    // Lets `update_matches` tell typing apart from a refresh after switching tabs.
    last_raw_query: String,
    line_position: Option<LinePosition>,
    selected_index: usize,
    cancel_flag: Arc<AtomicBool>,
}

impl SearchEverywhereDelegate {
    fn new(
        workspace: WeakEntity<Workspace>,
        project: Entity<Project>,
        previous_focus_handle: FocusHandle,
        actions: Vec<ActionEntry>,
        recent_paths: Vec<ProjectPath>,
    ) -> Self {
        Self {
            workspace,
            project,
            previous_focus_handle,
            actions,
            recent_paths,
            tab: SearchTab::default(),
            matches: Vec::new(),
            groups: Vec::new(),
            last_raw_query: String::new(),
            line_position: None,
            selected_index: 0,
            cancel_flag: Arc::new(AtomicBool::new(false)),
        }
    }

    fn include_root_name(&self, cx: &App) -> bool {
        self.project.read(cx).visible_worktrees(cx).count() > 1
    }

    fn path_candidate_sets(&self, cx: &App) -> Vec<PathMatchCandidateSet> {
        let include_root_name = self.include_root_name(cx);
        self.project
            .read(cx)
            .visible_worktrees(cx)
            .map(|worktree| PathMatchCandidateSet {
                snapshot: worktree.read(cx).snapshot(),
                include_ignored: false,
                include_root_name,
                candidates: project::Candidates::Files,
            })
            .collect()
    }

    fn recent_files(&self, cx: &App) -> Vec<SearchMatch> {
        let include_root_name = self.include_root_name(cx);
        let project = self.project.read(cx);
        self.recent_paths
            .iter()
            .filter_map(|project_path| {
                let worktree = project.worktree_for_id(project_path.worktree_id, cx)?;
                let path_prefix = if include_root_name {
                    worktree.read(cx).root_name().into_arc()
                } else {
                    util::rel_path::RelPath::empty_arc()
                };
                Some(SearchMatch::File(PathMatch {
                    score: 0.,
                    positions: Vec::new(),
                    worktree_id: project_path.worktree_id.to_usize(),
                    path: project_path.path.clone(),
                    path_prefix,
                    is_dir: false,
                    distance_to_relative_ancestor: 0,
                }))
            })
            .take(RECENT_FILES_LIMIT)
            .collect()
    }

    fn all_actions(&self) -> Vec<SearchMatch> {
        (0..self.actions.len())
            .map(|action_index| SearchMatch::Action {
                action_index,
                positions: Vec::new(),
                score: 0.,
            })
            .collect()
    }

    fn set_results(&mut self, mut results: GroupedResults, sort_by_relevance: bool) {
        if sort_by_relevance && self.tab == SearchTab::All {
            sort_groups_by_relevance(&mut results.groups);
        }
        self.matches.clear();
        self.groups.clear();
        for (group, mut items) in results.groups {
            if self.tab == SearchTab::All && items.len() > ALL_TAB_GROUP_LIMIT {
                items.truncate(ALL_TAB_GROUP_LIMIT);
                items.push(SearchMatch::More(group));
            }
            let start = self.matches.len();
            self.matches.extend(items);
            self.groups.push((group, start..self.matches.len()));
        }
        self.selected_index = 0;
    }

    fn group_of(&self, index: usize) -> Option<Group> {
        self.groups
            .iter()
            .find(|(_, range)| range.contains(&index))
            .map(|(group, _)| *group)
    }

    fn open_project_path(
        &self,
        project_path: ProjectPath,
        range: Option<Range<Point>>,
        secondary: bool,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) {
        let Some(open_task) = self
            .workspace
            .update(cx, |workspace, cx| {
                let pane = secondary.then(|| workspace.adjacent_pane(window, cx).downgrade());
                workspace.open_path(project_path, pane, true, window, cx)
            })
            .log_err()
        else {
            return;
        };
        cx.spawn_in(window, async move |_, cx| {
            let item = open_task.await?;
            if let Some(range) = range
                && let Some(editor) = item.downcast::<Editor>()
            {
                editor.update_in(cx, |editor, window, cx| {
                    editor.go_to_singleton_buffer_range(range, window, cx);
                })?;
            }
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    fn open_symbol(
        &self,
        symbol: Symbol,
        secondary: bool,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) {
        let buffer = self.project.update(cx, |project, cx| {
            project.open_buffer_for_symbol(&symbol, cx)
        });
        let workspace = self.workspace.clone();
        cx.spawn_in(window, async move |_, cx| {
            let buffer = buffer.await?;
            workspace.update_in(cx, |workspace, window, cx| {
                let position = buffer
                    .read(cx)
                    .clip_point_utf16(symbol.range.start, editor::Bias::Left);
                let point = buffer.read(cx).snapshot().point_utf16_to_point(position);
                let pane = secondary.then(|| workspace.adjacent_pane(window, cx));
                let editor = workspace
                    .open_project_item::<Editor>(pane, buffer, true, true, true, true, window, cx);
                editor.update(cx, |editor, cx| {
                    editor.go_to_singleton_buffer_point(point, window, cx);
                });
            })?;
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    fn switch_tab_from_picker(
        &mut self,
        tab: SearchTab,
        new_query: Option<String>,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) {
        self.tab = tab;
        // The picker is borrowed by the caller, so refresh it once it's released.
        cx.defer_in(window, move |picker, window, cx| {
            picker.refresh_placeholder(window, cx);
            match new_query {
                Some(query) => picker.set_query(&query, window, cx),
                None => picker.refresh(window, cx),
            }
        });
        cx.notify();
    }
}

async fn match_symbols(
    symbols: Vec<Symbol>,
    query: &str,
    limit: usize,
    cancel_flag: &AtomicBool,
    executor: gpui::BackgroundExecutor,
) -> Vec<SearchMatch> {
    if symbols.is_empty() {
        return Vec::new();
    }
    // rust-analyzer supports `foo::Bar` path queries, but the symbol names we
    // match against only contain the last segment.
    let query = query.rsplit_once("::").map_or(query, |(_, suffix)| suffix);
    let candidates: Vec<_> = symbols
        .iter()
        .enumerate()
        .map(|(id, symbol)| StringMatchCandidate::new(id, symbol.label.filter_text()))
        .collect();
    fuzzy_nucleo::match_strings_async(
        &candidates,
        query,
        Case::smart_if_uppercase_in(query),
        LengthPenalty::On,
        limit,
        cancel_flag,
        executor,
    )
    .await
    .into_iter()
    .filter_map(|string_match| {
        let symbol = symbols.get(string_match.candidate_id)?.clone();
        // Match positions are relative to the filter text, but we highlight the
        // full label.
        let offset = symbol.label.filter_range.start;
        Some(SearchMatch::Symbol {
            positions: string_match
                .positions
                .iter()
                .map(|position| position + offset)
                .collect(),
            score: string_match.score,
            symbol,
        })
    })
    .collect()
}

fn path_match_labels(
    path_match: &PathMatch,
    path_style: util::paths::PathStyle,
) -> (String, Vec<usize>, String, Vec<usize>) {
    let full_path = path_match.path_prefix.join(&path_match.path);
    let file_name = full_path.file_name().unwrap_or("").to_string();
    let file_name_start = full_path.as_unix_str().len() - file_name.len();
    let file_name_positions = path_match
        .positions
        .iter()
        .filter(|position| **position >= file_name_start)
        .map(|position| position - file_name_start)
        .collect();
    let directory = full_path
        .display(path_style)
        .trim_end_matches(file_name.as_str())
        .to_string();
    let directory_positions = path_match
        .positions
        .iter()
        .copied()
        .filter(|position| *position < directory.len())
        .collect();
    (
        file_name,
        file_name_positions,
        directory,
        directory_positions,
    )
}

impl PickerDelegate for SearchEverywhereDelegate {
    type ListItem = ListItem;

    fn name() -> &'static str {
        "search everywhere"
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        match self.tab {
            SearchTab::All => "Search everywhere (Tab switches scope, /files, #symbol, file:42)…",
            SearchTab::Classes => "Search classes…",
            SearchTab::Files => "Search files (file.rs:42 jumps to a line)…",
            SearchTab::Symbols => "Search symbols…",
            SearchTab::Actions => "Search actions…",
            SearchTab::Text => "Search text in project…",
        }
        .into()
    }

    fn no_matches_text(&self, _window: &mut Window, _cx: &mut App) -> Option<SharedString> {
        Some("Nothing found".into())
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, index: usize, _: &mut Window, _: &mut Context<Picker<Self>>) {
        self.selected_index = index;
    }

    fn separators_after_indices(&self) -> Vec<usize> {
        let mut separators: Vec<usize> = self
            .groups
            .iter()
            .map(|(_, range)| range.end.saturating_sub(1))
            .collect();
        separators.pop();
        separators
    }

    fn render_editor(
        &self,
        editor: &Arc<dyn ErasedEditor>,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Div> {
        let active_tab = self.tab;
        Some(
            v_flex()
                .child(
                    h_flex()
                        .gap_1()
                        .px_1p5()
                        .pt_1p5()
                        .children(SearchTab::ALL.into_iter().map(|tab| {
                            Button::new(tab.label(), tab.label())
                                .toggle_state(tab == active_tab)
                                .label_size(LabelSize::Small)
                                .on_click(cx.listener(move |picker, _: &ClickEvent, window, cx| {
                                    switch_tab(picker, tab, window, cx);
                                    picker.focus(window, cx);
                                }))
                        })),
                )
                .child(
                    h_flex()
                        .h_9()
                        .px_2p5()
                        .flex_none()
                        .overflow_hidden()
                        .child(div().flex_1().child(editor.render(window, cx))),
                )
                .child(Divider::horizontal()),
        )
    }

    fn update_matches(
        &mut self,
        raw_query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        self.cancel_flag.store(true, Ordering::Release);
        self.cancel_flag = Arc::new(AtomicBool::new(false));
        let cancel_flag = self.cancel_flag.clone();

        // Prefixes only apply while typing: re-running the same query after a tab
        // switch must not bounce to another tab. In the Text tab `#include` or
        // `/* foo` are legitimate queries, so prefixes are ignored there.
        let query_changed = raw_query != self.last_raw_query;
        self.last_raw_query = raw_query.clone();
        if query_changed
            && self.tab != SearchTab::Text
            && let Some(prefix) = parse_scope_prefix(&raw_query)
        {
            let rest = prefix.rest.to_string();
            self.switch_tab_from_picker(prefix.tab, Some(rest), window, cx);
            return Task::ready(());
        }

        cx.set_global(LastSearch {
            tab: self.tab,
            query: raw_query.clone(),
        });

        let tab = self.tab;
        let trimmed_query = raw_query.trim();
        let (query, line_position) = if tab.includes_files() {
            split_line_suffix(trimmed_query)
        } else {
            (trimmed_query, None)
        };
        let query = query.to_string();
        self.line_position = line_position;

        if query.is_empty() {
            let mut results = GroupedResults::default();
            match tab {
                SearchTab::All | SearchTab::Files => {
                    results.push(Group::Files, self.recent_files(cx))
                }
                SearchTab::Actions => results.push(Group::Actions, self.all_actions()),
                SearchTab::Classes | SearchTab::Symbols | SearchTab::Text => {}
            }
            // Recent files have no score; keep them in history order.
            self.set_results(results, false);
            return Task::ready(());
        }

        // One extra item tells us whether a group needs a "More…" row.
        let limit = if tab == SearchTab::All {
            ALL_TAB_GROUP_LIMIT + 1
        } else {
            TAB_LIMIT
        };
        let executor = cx.background_executor().clone();
        let project = self.project.clone();
        let candidate_sets = if tab.includes_files() {
            self.path_candidate_sets(cx)
        } else {
            Vec::new()
        };
        let symbols_task = tab.includes_symbols().then(|| {
            self.project
                .update(cx, |project, cx| project.symbols(&query, cx))
        });
        let action_candidates: Vec<_> = if tab.includes_actions() {
            self.actions
                .iter()
                .enumerate()
                .map(|(id, entry)| StringMatchCandidate::new(id, &entry.name))
                .collect()
        } else {
            Vec::new()
        };

        cx.spawn_in(window, async move |picker, cx| {
            let mut results = GroupedResults::default();
            if tab == SearchTab::Text {
                cx.background_executor().timer(TEXT_SEARCH_DEBOUNCE).await;
                if cancel_flag.load(Ordering::Acquire) {
                    return;
                }
                let text_matches = text_search::search(project, query, limit, &cancel_flag, cx)
                    .await
                    .log_err()
                    .unwrap_or_default();
                results.push(
                    Group::Text,
                    text_matches.into_iter().map(SearchMatch::Text).collect(),
                );
            } else {
                let files: Vec<SearchMatch> = if candidate_sets.is_empty() {
                    Vec::new()
                } else {
                    fuzzy_nucleo::match_path_sets(
                        &candidate_sets,
                        &query,
                        &None,
                        Case::Ignore,
                        limit,
                        &cancel_flag,
                        executor.clone(),
                    )
                    .await
                    .into_iter()
                    .map(SearchMatch::File)
                    .collect()
                };

                let actions: Vec<SearchMatch> = fuzzy_nucleo::match_strings_async(
                    &action_candidates,
                    &query,
                    Case::Ignore,
                    LengthPenalty::On,
                    limit,
                    &cancel_flag,
                    executor.clone(),
                )
                .await
                .into_iter()
                .map(|string_match| SearchMatch::Action {
                    action_index: string_match.candidate_id,
                    positions: string_match.positions,
                    score: string_match.score,
                })
                .collect();

                let all_symbols = match symbols_task {
                    Some(task) => task.await.log_err().unwrap_or_default(),
                    None => Vec::new(),
                };
                let (class_symbols, other_symbols): (Vec<_>, Vec<_>) = all_symbols
                    .into_iter()
                    .partition(|symbol| is_class_like(symbol.kind));

                match tab {
                    SearchTab::Classes => results.push(
                        Group::Classes,
                        match_symbols(class_symbols, &query, limit, &cancel_flag, executor).await,
                    ),
                    SearchTab::Symbols => {
                        let mut symbols = class_symbols;
                        symbols.extend(other_symbols);
                        results.push(
                            Group::Symbols,
                            match_symbols(symbols, &query, limit, &cancel_flag, executor).await,
                        );
                    }
                    SearchTab::Files => results.push(Group::Files, files),
                    SearchTab::Actions => results.push(Group::Actions, actions),
                    SearchTab::All => {
                        let classes = match_symbols(
                            class_symbols,
                            &query,
                            limit,
                            &cancel_flag,
                            executor.clone(),
                        )
                        .await;
                        let symbols =
                            match_symbols(other_symbols, &query, limit, &cancel_flag, executor)
                                .await;
                        results.push(Group::Classes, classes);
                        results.push(Group::Files, files);
                        results.push(Group::Symbols, symbols);
                        results.push(Group::Actions, actions);
                    }
                    SearchTab::Text => {}
                }
            }

            if cancel_flag.load(Ordering::Acquire) {
                return;
            }
            picker
                .update(cx, |picker, _| picker.delegate.set_results(results, true))
                .log_err();
        })
    }

    fn confirm(&mut self, secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(selected) = self.matches.get(self.selected_index) else {
            cx.emit(DismissEvent);
            return;
        };
        match selected {
            SearchMatch::More(group) => {
                let tab = group.tab();
                self.switch_tab_from_picker(tab, None, window, cx);
                return;
            }
            SearchMatch::File(path_match) => {
                let project_path = ProjectPath {
                    worktree_id: WorktreeId::from_usize(path_match.worktree_id),
                    path: path_match.path.clone(),
                };
                let range = self.line_position.map(|position| {
                    let point = Point::new(
                        position.row.saturating_sub(1),
                        position.column.unwrap_or(1).saturating_sub(1),
                    );
                    point..point
                });
                self.open_project_path(project_path, range, secondary, window, cx);
            }
            SearchMatch::Text(text_match) => {
                let project_path = text_match.project_path.clone();
                let range = text_match.range.clone();
                self.open_project_path(project_path, Some(range), secondary, window, cx);
            }
            SearchMatch::Symbol { symbol, .. } => {
                let symbol = symbol.clone();
                self.open_symbol(symbol, secondary, window, cx);
            }
            SearchMatch::Action { action_index, .. } => {
                let Some(entry) = self.actions.get(*action_index) else {
                    return;
                };
                let action = entry.action.boxed_clone();
                window.focus(&self.previous_focus_handle, cx);
                cx.emit(DismissEvent);
                window.dispatch_action(action, cx);
                return;
            }
        }
        cx.emit(DismissEvent);
    }

    fn dismissed(&mut self, _: &mut Window, _: &mut Context<Picker<Self>>) {}

    fn render_match(
        &self,
        index: usize,
        selected: bool,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let search_match = self.matches.get(index)?;
        let path_style = self.project.read(cx).path_style(cx);
        let item = ListItem::new(index)
            .inset(true)
            .spacing(ListItemSpacing::Sparse)
            .toggle_state(selected);

        let (icon, label, positions, detail, detail_positions): (
            IconName,
            SharedString,
            Vec<usize>,
            Option<SharedString>,
            Vec<usize>,
        ) = match search_match {
            SearchMatch::More(group) => {
                return Some(
                    item.child(
                        Label::new(format!("More {}s…", group.label().to_lowercase()))
                            .size(LabelSize::Small)
                            .color(Color::Accent),
                    ),
                );
            }
            SearchMatch::File(path_match) => {
                let (file_name, file_name_positions, directory, directory_positions) =
                    path_match_labels(path_match, path_style);
                (
                    IconName::File,
                    file_name.into(),
                    file_name_positions,
                    (!directory.is_empty()).then(|| directory.into()),
                    directory_positions,
                )
            }
            SearchMatch::Symbol {
                symbol, positions, ..
            } => {
                let location = match &symbol.path {
                    SymbolLocation::InProject(path) => path.path.display(path_style).into_owned(),
                    SymbolLocation::OutsideProject { abs_path, .. } => {
                        abs_path.to_string_lossy().into_owned()
                    }
                };
                let icon = if is_class_like(symbol.kind) {
                    IconName::Box
                } else {
                    IconName::Code
                };
                (
                    icon,
                    symbol.label.text.clone().into(),
                    positions.clone(),
                    Some(format!("{location}:{}", symbol.range.start.0.row + 1).into()),
                    Vec::new(),
                )
            }
            SearchMatch::Action {
                action_index,
                positions,
                ..
            } => (
                IconName::Command,
                self.actions.get(*action_index)?.name.clone(),
                positions.clone(),
                None,
                Vec::new(),
            ),
            SearchMatch::Text(text_match) => (
                IconName::MagnifyingGlass,
                text_match.line.clone(),
                text_match.positions.clone(),
                Some(
                    format!(
                        "{}:{}",
                        text_match.project_path.path.display(path_style),
                        text_match.range.start.row + 1
                    )
                    .into(),
                ),
                Vec::new(),
            ),
        };

        let group_label = (self.tab == SearchTab::All)
            .then(|| self.group_of(index))
            .flatten()
            .map(|group| {
                Label::new(group.label())
                    .size(LabelSize::XSmall)
                    .color(Color::Muted)
            });

        Some(
            item.start_slot(Icon::new(icon).color(Color::Muted).size(IconSize::Small))
                .end_slot::<Label>(group_label)
                .child(
                    h_flex()
                        .gap_2()
                        .min_w_0()
                        .child(HighlightedLabel::new(label, positions).truncate())
                        .children(detail.map(|detail| {
                            HighlightedLabel::new(detail, detail_positions)
                                .size(LabelSize::Small)
                                .color(Color::Muted)
                                .truncate()
                        })),
                ),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tabs_cycle_in_both_directions() {
        assert_eq!(SearchTab::All.cycle(1), SearchTab::Classes);
        assert_eq!(SearchTab::Text.cycle(1), SearchTab::All);
        assert_eq!(SearchTab::All.cycle(-1), SearchTab::Text);
    }

    fn action(score: f64) -> SearchMatch {
        SearchMatch::Action {
            action_index: 0,
            positions: Vec::new(),
            score,
        }
    }

    #[test]
    fn groups_are_ordered_by_best_score() {
        let mut groups = vec![
            (Group::Classes, vec![action(0.2)]),
            (Group::Files, vec![action(0.1), action(0.9)]),
            (Group::Actions, vec![action(0.5)]),
        ];
        sort_groups_by_relevance(&mut groups);
        let order: Vec<Group> = groups.iter().map(|(group, _)| *group).collect();
        assert_eq!(order, vec![Group::Files, Group::Actions, Group::Classes]);
    }

    #[test]
    fn group_order_is_stable_on_ties() {
        let mut groups = vec![
            (Group::Classes, vec![action(0.5)]),
            (Group::Files, vec![action(0.5)]),
        ];
        sort_groups_by_relevance(&mut groups);
        let order: Vec<Group> = groups.iter().map(|(group, _)| *group).collect();
        assert_eq!(order, vec![Group::Classes, Group::Files]);
    }
}
