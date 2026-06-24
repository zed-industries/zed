//! The text_finder is a minimal modal interface to the project_search. It is
//! targeted towards search for exploration. It can also be used as a filter
//! step to the project_search.
//!
//! Basic interaction:
//!
//! ```txt
//! Open text finder --- Open file ---> File tab
//!
//!                     (text_finder action)
//! Open text finder --- ToProjectSearch ---> Project search tab
//!
//! Can also have a little loop where the user uses the ProjectSearch filters etc
//! to refine the search:
//!
//!                     (project search tab)
//!                  (removes tab, opens modal)
//! Project search tab --- ToTextFinder ---> Text finder modal
//!                             ^                  |
//!                             |             ToProjectSearch (adds tab,
//!                             |                  |          closes modal)
//!                             |                  V
//!                             . --------  Project search tab
//! ```
use std::ops::ControlFlow;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{ops::Range, time::Duration};

use collections::{HashMap, HashSet};
use editor::{MultiBufferSnapshot, PathKey, multibuffer_context_lines};
use file_icons::FileIcons;
use futures::StreamExt;
use gpui::{
    AnyElement, AppContext, AsyncApp, DismissEvent, EntityId, HighlightStyle, StyledText, Task,
    TextStyle, prelude::*,
};
use gpui::{Entity, FocusHandle};
use language::{Buffer, LanguageAwareStyling};
use picker::{Picker, PickerDelegate};
use project::{Project, ProjectPath};
use project::{SearchResults, search::SearchQuery, search::SearchResult};
use settings::Settings;
use smol::future::yield_now;
use text::Anchor;
use theme_settings::ThemeSettings;
use ui::{
    Divider, FluentBuilder, IconButtonShape, ListItem, ListItemSpacing, Toggleable, Tooltip,
    prelude::*,
};
use util::ResultExt;
use workspace::SplitDirection;
use workspace::Workspace;
use workspace::item::ItemSettings;

use super::SearchMatch;
use crate::project_search::{ActiveSettings, ProjectSearch};
use crate::{ProjectSearchView, SearchOption, SearchOptions};

pub struct Delegate {
    pub(crate) project_search_view: Entity<ProjectSearchView>,
    pub(crate) focus_handle: FocusHandle,
    /// Flat list of every match, in result order. This is the canonical list
    /// handed off to the project search; [`Self::entries`] is a grouped view
    /// derived from it for rendering.
    pub(crate) matches: Vec<SearchMatch>,
    /// Display rows derived from [`Self::matches`]: a non-selectable header per
    /// file, its matches, and separators between groups. Rebuilt via
    /// [`Delegate::rebuild_entries`] whenever `matches` changes. `selected_index`
    /// indexes into this list.
    pub(crate) entries: Vec<Entry>,
    pub(crate) selected_index: usize,
    pub(crate) cancel_flag: Arc<AtomicBool>,
    pub(crate) text_finder_turning_into_project_search: Arc<AtomicBool>,
    pub(crate) last_selection_change_time: Option<std::time::Instant>,
    pub(crate) last_click: Option<(usize, std::time::Instant)>,
    pub(crate) search_options: SearchOptions,
    /// Kept around for switching to project search
    pub(crate) active_query: Option<SearchQuery>,
    pub(crate) imported_from_project_search: bool,
    /// When `is_ready` there is not a search in progress
    pub(crate) in_progress_search: InProgressSearch,
    pub(crate) unique_files: HashSet<ProjectPath>,
    /// Largest line number across [`Self::matches`], used to size the line-number
    /// column so every row's number right-aligns to the widest one. Recomputed in
    /// [`Delegate::rebuild_entries`].
    pub(crate) max_line_number: u32,
}

pub(crate) enum Entry {
    Header(ProjectPath),
    Match(usize),
    Separator,
}

async fn get_ongoing_search(
    project_search_view: &Entity<ProjectSearchView>,
    cx: &mut AsyncApp,
) -> Option<SearchResults<SearchResult>> {
    let ongoing_search = project_search_view.update(cx, |view, cx| {
        view.entity.update(cx, |search, _| {
            search.pending_search.take().inspect(|_| {
                search
                    .project_search_turning_into_text_finder
                    .store(true, Ordering::Relaxed);
            })
        })
    })?;

    ongoing_search.await
}

fn multibuffer_ranges_to_search_matches<'a>(
    match_ranges: &'a [Range<multi_buffer::Anchor>],
    multi_buffer: &'a editor::MultiBuffer,
    snapshot: MultiBufferSnapshot,
    cx: &'a App,
) -> impl Iterator<Item = SearchMatch> + 'a {
    match_ranges.iter().cloned().filter_map(move |mb_range| {
        let (buffer_snapshot, text_range) =
            snapshot.anchor_range_to_buffer_anchor_range(mb_range)?;

        let file = buffer_snapshot.file()?;
        let path = ProjectPath {
            worktree_id: file.worktree_id(cx),
            path: Arc::clone(file.path()),
        };
        let buffer = multi_buffer.buffer(buffer_snapshot.remote_id())?;

        let start_offset: usize = buffer_snapshot.summary_for_anchor(&text_range.start);
        let end_offset: usize = buffer_snapshot.summary_for_anchor(&text_range.end);
        let line_number = buffer_snapshot.offset_to_point(start_offset).row + 1;

        let text = buffer_snapshot.text();
        let line_start = text[..start_offset].rfind('\n').map(|i| i + 1).unwrap_or(0);
        let line_end = text[start_offset..]
            .find('\n')
            .map(|i| start_offset + i)
            .unwrap_or(text.len());
        let line_text = text[line_start..line_end].to_string();

        let relative_start = start_offset - line_start;
        let relative_end = end_offset - line_start;

        Some(SearchMatch {
            path,
            buffer,
            anchor_range: text_range,
            range: start_offset..end_offset,
            relative_range: relative_start..relative_end,
            line_text,
            line_number,
        })
    })
}

/// Stream the matches already sitting in the project search's multibuffer into
/// the picker, a chunk at a time. Inverse of [`matches_to_multibuffer`].
async fn stream_plunder_to_picker(
    project_search_view: Entity<ProjectSearchView>,
    cancel_flag: Arc<AtomicBool>,
    picker: gpui::WeakEntity<Picker<Delegate>>,
    cx: &mut AsyncApp,
) {
    let chunk_size = 1000;
    let mut n_read = 0;

    loop {
        if cancel_flag.load(Ordering::SeqCst) {
            return; // user cancelled or changed the query
        }

        let res = picker.update(cx, |picker, cx| {
            let new_matches: Vec<SearchMatch> = {
                let ps = project_search_view.read(cx).entity.read(cx);
                let len = ps.match_ranges.len();
                if n_read >= len {
                    return ControlFlow::Break(());
                }
                let end = (n_read + chunk_size).min(len);
                let chunk = &ps.match_ranges[n_read..end];
                let multi_buffer = ps.excerpts.read(cx);
                let snapshot = multi_buffer.snapshot(cx);
                let matches =
                    multibuffer_ranges_to_search_matches(chunk, multi_buffer, snapshot, cx)
                        .collect();
                n_read = end;
                matches
            };

            let delegate = &mut picker.delegate;
            delegate
                .unique_files
                .extend(new_matches.iter().map(|m| m.path.clone()));
            delegate.matches.extend(new_matches);
            delegate.rebuild_entries();
            cx.notify();
            ControlFlow::Continue(())
        });

        match res {
            Ok(ControlFlow::Continue(())) => {}
            Ok(ControlFlow::Break(())) | Err(_) => break,
        }

        // Critical or the search transformation will hold the background thread for too long
        yield_now().await;
    }
}

pub(crate) enum InProgressSearch {
    Connected(Task<Option<SearchResults<SearchResult>>>),
    Disconnected(SearchResults<SearchResult>),
    None,
}

impl InProgressSearch {
    /// If this is in disconnected state set it to None and return the search results
    fn take_disconnected(&mut self) -> Option<SearchResults<SearchResult>> {
        if matches!(self, InProgressSearch::Disconnected(_)) {
            let mut placeholder = InProgressSearch::None;
            std::mem::swap(self, &mut placeholder);
            match placeholder {
                InProgressSearch::Disconnected(results_stream) => return Some(results_stream),
                _ => unreachable!("guarded with matches! above"),
            }
        } else {
            None
        }
    }

    /// If a search is currently streaming into the picker, take its task so it
    /// can be awaited to recover the underlying result stream.
    pub(crate) fn take_connected(&mut self) -> Option<Task<Option<SearchResults<SearchResult>>>> {
        if matches!(self, InProgressSearch::Connected(_)) {
            match std::mem::replace(self, InProgressSearch::None) {
                InProgressSearch::Connected(task) => Some(task),
                _ => unreachable!("guarded with matches! above"),
            }
        } else {
            None
        }
    }
}

impl Delegate {
    pub fn hook_up_any_ongoing_search(
        &mut self,
        picker: gpui::WeakEntity<Picker<Delegate>>,
        cx: &App,
    ) {
        let cancel_flag = Arc::clone(&self.cancel_flag);
        let text_finder_turning_into_project_search =
            Arc::clone(&self.text_finder_turning_into_project_search);
        let project_search_view = self.project_search_view.clone();
        let ongoing = self.in_progress_search.take_disconnected();

        self.in_progress_search = InProgressSearch::Connected(cx.spawn(async move |cx| {
            stream_plunder_to_picker(project_search_view, cancel_flag.clone(), picker.clone(), cx)
                .await;

            if let Some(results_stream) = ongoing {
                return stream_results_to_picker(
                    cancel_flag,
                    text_finder_turning_into_project_search,
                    picker,
                    results_stream,
                    ImportedMatches::Yes,
                    cx,
                )
                .await;
            }
            None
        }));
    }

    pub fn new_from_project_search(
        project_search: Entity<ProjectSearchView>,
        cx: &mut AsyncApp,
    ) -> Task<Delegate> {
        cx.spawn(async move |cx| {
            let ongoing = get_ongoing_search(&project_search, cx).await;

            let in_progress_search = if let Some(results_stream) = ongoing {
                InProgressSearch::Disconnected(results_stream)
            } else {
                InProgressSearch::None
            };

            let (search_options, active_query, has_existing_matches) =
                cx.read_entity(&project_search, |ps, cx| {
                    let entity = ps.entity.read(cx);
                    (
                        ps.search_options,
                        entity.active_query.clone(),
                        !entity.match_ranges.is_empty(),
                    )
                });

            let imported_from_project_search =
                has_existing_matches || !matches!(in_progress_search, InProgressSearch::None);

            let this = cx.update(move |cx| Self {
                project_search_view: project_search,
                focus_handle: cx.focus_handle(),
                matches: Vec::new(),
                entries: Vec::new(),
                selected_index: 0,
                cancel_flag: Arc::new(AtomicBool::new(false)),
                text_finder_turning_into_project_search: Arc::new(AtomicBool::new(false)),
                last_selection_change_time: None,
                last_click: None,
                search_options,
                active_query,
                imported_from_project_search,
                in_progress_search,
                unique_files: HashSet::default(),
                max_line_number: 0,
            });

            this
        })
    }

    pub fn new(
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Task<Self> {
        let project = workspace.project().clone();
        let weak_workspace = workspace.weak_handle();
        let settings = cx
            .global::<ActiveSettings>()
            .0
            .get(&project.downgrade())
            .cloned();

        let search = cx.new(|cx| ProjectSearch::new(project, cx));
        let project_search =
            cx.new(|cx| ProjectSearchView::new(weak_workspace, search, window, cx, settings));
        cx.spawn(async move |_, cx| Self::new_from_project_search(project_search, cx).await)
    }

    pub(crate) fn project<'a>(&self, cx: &'a App) -> &'a Entity<Project> {
        &self.project_search_view.read(cx).entity.read(cx).project
    }

    /// Rebuilds the grouped [`Self::entries`] display list from the flat
    /// [`Self::matches`]. Matches arrive grouped per file (one search result
    /// per buffer), so consecutive matches share a path; we emit one header per
    /// group and a separator before every group after the first.
    ///
    /// Selection is preserved across rebuilds: if a match was selected it stays
    /// selected at its new row, otherwise we snap to the first selectable row.
    pub(crate) fn rebuild_entries(&mut self) {
        let previously_selected_match = match self.entries.get(self.selected_index) {
            Some(Entry::Match(match_index)) => Some(*match_index),
            _ => None,
        };

        let mut entries = Vec::with_capacity(self.matches.len());
        let mut last_path: Option<&ProjectPath> = None;
        for (match_index, search_match) in self.matches.iter().enumerate() {
            if last_path != Some(&search_match.path) {
                if last_path.is_some() {
                    entries.push(Entry::Separator);
                }
                entries.push(Entry::Header(search_match.path.clone()));
                last_path = Some(&search_match.path);
            }
            entries.push(Entry::Match(match_index));
        }
        self.entries = entries;
        self.max_line_number = self
            .matches
            .iter()
            .map(|search_match| search_match.line_number)
            .max()
            .unwrap_or(0);

        self.selected_index = previously_selected_match
            .and_then(|match_index| {
                self.entries
                    .iter()
                    .position(|entry| matches!(entry, Entry::Match(other) if *other == match_index))
            })
            .or_else(|| self.first_selectable_index())
            .unwrap_or(0);
    }

    fn first_selectable_index(&self) -> Option<usize> {
        self.entries
            .iter()
            .position(|entry| matches!(entry, Entry::Match(_)))
    }

    fn selected_search_match(&self) -> Option<&SearchMatch> {
        match self.entries.get(self.selected_index)? {
            Entry::Match(match_index) => self.matches.get(*match_index),
            Entry::Header(_) | Entry::Separator => None,
        }
    }

    /// Opens the selected match in a new split in `direction`, then dismisses.
    pub(crate) fn open_in_split(
        &mut self,
        direction: SplitDirection,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) {
        let Some(selected_match) = self.selected_search_match() else {
            return;
        };
        let path = selected_match.path.clone();
        let line_number = selected_match.line_number;
        let Some(workspace) = self.project_search_view.read(cx).workspace.upgrade() else {
            return;
        };
        let open_task = workspace.update(cx, |workspace, cx| {
            workspace.split_path_preview(path, false, Some(direction), window, cx)
        });
        let row = line_number.saturating_sub(1);
        cx.spawn_in(window, async move |_, cx| {
            let item = open_task.await.log_err()?;
            if let Some(active_editor) = item.downcast::<editor::Editor>() {
                active_editor
                    .downgrade()
                    .update_in(cx, |editor, window, cx| {
                        editor.go_to_singleton_buffer_point(text::Point::new(row, 0), window, cx);
                    })
                    .log_err();
            }
            Some(())
        })
        .detach();
        cx.emit(DismissEvent);
    }
}

pub(crate) enum PopulateProjectSearch {
    Completed,
    SupersededByNewSearch,
}

/// Convert the picker's list of matches into multibuffer. Inverse of
/// [`plunder_multibuffer`].
pub(crate) async fn matches_to_multibuffer(
    project_search_view: &Entity<ProjectSearchView>,
    matches: &[SearchMatch],
    cx: &mut AsyncApp,
) -> PopulateProjectSearch {
    let mut buffer_order_in_text_finder: Vec<EntityId> = Vec::new();
    let mut by_buffer: HashMap<_, (_, Vec<_>)> = HashMap::default();

    for m in matches {
        let buffer = Entity::clone(&m.buffer);
        by_buffer
            .entry(buffer.entity_id())
            .and_modify(|(_, ranges)| ranges.push(m.anchor_range.clone()))
            .or_insert_with(|| {
                buffer_order_in_text_finder.push(buffer.entity_id());
                (buffer, vec![m.anchor_range.clone()])
            });
    }

    let excerpts =
        project_search_view.read_with(cx, |view, cx| view.entity.read(cx).excerpts.clone());
    excerpts.update(cx, |excerpts, cx| excerpts.clear(cx));

    // Every await point is a place where the user could type a search
    // query in which case we gotta abort. Store the search id so we
    // can check if that happened.
    let search_id = project_search_view.update(cx, |view, cx| {
        view.entity.update(cx, |search, _| {
            search.match_ranges.clear();
            search.search_id
        })
    });

    let context_lines = cx.update(|cx| multibuffer_context_lines(cx));

    let still_current = |cx: &mut AsyncApp| {
        project_search_view.update(cx, |view, cx| view.entity.read(cx).search_id == search_id)
    };

    let mut excerpts_added = 0;
    for buffer_id in buffer_order_in_text_finder {
        if !still_current(cx) {
            return PopulateProjectSearch::SupersededByNewSearch;
        }
        let (buffer, ranges) = by_buffer.remove(&buffer_id).expect("just put them in");
        excerpts_added += ranges.len();
        let new_ranges = excerpts
            .update(cx, |excerpts, cx| {
                excerpts.set_anchored_excerpts_for_path(
                    PathKey::for_buffer(&buffer, cx),
                    buffer,
                    ranges,
                    context_lines,
                    cx,
                )
            })
            .await;

        if !still_current(cx) {
            return PopulateProjectSearch::SupersededByNewSearch;
        }
        project_search_view.update(cx, |view, cx| {
            view.entity.update(cx, |search, cx| {
                search.match_ranges.extend(new_ranges);
                cx.notify();
            })
        });

        // Adding items to the multibuffer can take time. Be sure to not hold
        // the foreground hostage.
        if excerpts_added > 100 {
            yield_now().await;
            excerpts_added = 0;
        }
    }
    PopulateProjectSearch::Completed
}

const SEARCH_DEBOUNCE_MS: u64 = 100;
const CLICK_THRESHOLD_MS: u128 = 50;
const DOUBLE_CLICK_THRESHOLD_MS: u128 = 300;
const SEARCH_RESULTS_BATCH_SIZE: usize = 256;

impl PickerDelegate for Delegate {
    type ListItem = AnyElement;

    fn name() -> &'static str {
        "text finder"
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Search all files…".into()
    }

    fn searchbar_trailer(
        &self,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<AnyElement> {
        let active = self.search_options;
        let focus_handle = self.focus_handle.clone();
        let picker = cx.entity();

        let filter_buttons = [
            SearchOption::CaseSensitive,
            SearchOption::WholeWord,
            SearchOption::Regex,
            SearchOption::IncludeIgnored,
        ]
        .into_iter()
        .map(|option| {
            let options = option.as_options();
            let action = option.to_toggle_action();
            let label = option.label();
            let focus_handle = focus_handle.clone();
            let picker = picker.clone();
            IconButton::new(
                ("text-finder-search-option", option as usize),
                option.icon(),
            )
            .icon_size(IconSize::Small)
            .shape(IconButtonShape::Square)
            .toggle_state(active.contains(options))
            .tooltip(move |_window, cx| Tooltip::for_action_in(label, action, &focus_handle, cx))
            .on_click(move |_, window, cx| {
                picker.update(cx, |picker, cx| {
                    picker.delegate.search_options.toggle(options);
                    picker.refresh(window, cx);
                });
            })
        });

        Some(
            h_flex()
                .gap_1()
                .children(filter_buttons)
                .children(picker::parts::project_scan_indicator(
                    self.active_query.is_some(),
                    self.project(cx),
                    cx,
                ))
                .into_any_element(),
        )
    }

    fn actions_menu(
        &self,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> Vec<picker::PickerAction> {
        use gpui::Action as _;
        vec![
            picker::PickerAction::header("Split…"),
            picker::PickerAction::button(
                "Left",
                workspace::pane::SplitLeft::default().boxed_clone(),
            ),
            picker::PickerAction::button(
                "Right",
                workspace::pane::SplitRight::default().boxed_clone(),
            ),
            picker::PickerAction::button("Up", workspace::pane::SplitUp::default().boxed_clone()),
            picker::PickerAction::button(
                "Down",
                workspace::pane::SplitDown::default().boxed_clone(),
            ),
            picker::PickerAction::separator(),
            picker::PickerAction::button("Open File", menu::Confirm.boxed_clone()),
            picker::PickerAction::button("Open as Tab", super::ToProjectSearch.boxed_clone()),
        ]
    }

    fn match_count(&self) -> usize {
        self.entries.len()
    }

    fn can_select(&self, ix: usize, _window: &mut Window, _cx: &mut Context<Picker<Self>>) -> bool {
        matches!(self.entries.get(ix), Some(Entry::Match(_)))
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn select_on_hover(&self) -> bool {
        false
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) {
        self.selected_index = ix;
        self.last_selection_change_time = Some(std::time::Instant::now());
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        self.cancel_flag
            .store(true, std::sync::atomic::Ordering::SeqCst);
        self.cancel_flag = Arc::new(AtomicBool::new(false));

        let cancel_flag = Arc::clone(&self.cancel_flag);
        let text_finder_turning_into_project_search =
            Arc::clone(&self.text_finder_turning_into_project_search);

        // The picker runs `update_matches("")` once on open. When the text
        // finder was opened from an existing project search, the query editor is
        // empty but we have already plundered that search's matches. Preserve
        // them on that first call, otherwise the modal would show up empty.
        let imported_from_project_search = std::mem::take(&mut self.imported_from_project_search);

        let Some(search_query) = self.build_search_query(&query, cx) else {
            if query.is_empty() && imported_from_project_search {
                return Task::ready(());
            }
            self.matches.clear();
            self.entries.clear();
            self.unique_files.clear();
            self.selected_index = 0;
            self.active_query = None;
            cx.notify();
            return Task::ready(());
        };

        // Remember the exact query we are running so that a later switch to the
        // project search hands over a query consistent with the results.
        self.active_query = Some(search_query.clone());

        let search_results = self.project_search_view.update(cx, |ps, cx| {
            ps.entity.update(cx, |pr, cx| {
                pr.project.update(cx, |p, cx| p.search(search_query, cx))
            })
        });

        let (signal_done, match_updating_done) = futures::channel::oneshot::channel();
        self.in_progress_search =
            InProgressSearch::Connected(cx.spawn_in(window, async move |picker, cx| {
                cx.background_executor()
                    .timer(Duration::from_millis(SEARCH_DEBOUNCE_MS))
                    .await;

                if cancel_flag.load(std::sync::atomic::Ordering::SeqCst) {
                    return None;
                }

                let res = stream_results_to_picker(
                    cancel_flag,
                    text_finder_turning_into_project_search,
                    picker,
                    search_results,
                    ImportedMatches::No,
                    cx,
                )
                .await;

                // We must own the search task so we can take out the search
                // result stream in case we are transforming into project
                // search. The picker relies on the task returned
                // `PickerDelegate::update_matches` to detect when we are done
                // updating. So we have a placeholder task that completes when
                // this signal is send.
                let _ = signal_done.send(());
                res
            }));

        cx.notify();
        cx.spawn(async move |_, _| {
            let _ = match_updating_done.await;
        })
    }

    fn confirm(&mut self, _: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        // Clicks (set_selected_index called immediately before confirm) require double-click.
        // Enter key proceeds immediately.
        let now = std::time::Instant::now();
        let is_click = self
            .last_selection_change_time
            .map(|t| now.duration_since(t).as_millis() < CLICK_THRESHOLD_MS)
            .unwrap_or(false);

        if is_click {
            let is_double_click = self
                .last_click
                .map(|(ix, t)| {
                    ix == self.selected_index
                        && now.duration_since(t).as_millis() < DOUBLE_CLICK_THRESHOLD_MS
                })
                .unwrap_or(false);
            self.last_click = Some((self.selected_index, now));

            if !is_double_click {
                cx.focus_self(window);
                return;
            }
        }

        let Some(selected_match) = self.selected_search_match() else {
            return;
        };

        let path = selected_match.path.clone();
        let line_number = selected_match.line_number;

        let Some(workspace) = self.project_search_view.read(cx).workspace.upgrade() else {
            return;
        };

        let open_task = workspace.update(cx, |workspace, cx| {
            workspace.open_path_preview(path, None, true, false, true, window, cx)
        });

        let row = line_number.saturating_sub(1);
        cx.spawn_in(window, async move |_, cx| {
            let item = open_task.await.log_err()?;
            if let Some(active_editor) = item.downcast::<editor::Editor>() {
                active_editor
                    .downgrade()
                    .update_in(cx, |editor, window, cx| {
                        editor.go_to_singleton_buffer_point(text::Point::new(row, 0), window, cx);
                    })
                    .log_err();
            }
            Some(())
        })
        .detach();

        cx.emit(DismissEvent);
    }

    fn dismissed(&mut self, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        cx.emit(DismissEvent);
    }

    fn try_get_preview_data_for_match(&self, _cx: &App) -> Option<picker::PreviewUpdate> {
        let m = self.selected_search_match()?;
        Some(picker::PreviewUpdate::from_buffer(
            m.buffer.clone(),
            picker::MatchLocation {
                anchor_range: m.anchor_range.clone(),
                range: m.range.clone(),
            },
        ))
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        match self.entries.get(ix)? {
            Entry::Separator => Some(
                div()
                    .py(DynamicSpacing::Base04.rems(cx))
                    .child(Divider::horizontal())
                    .into_any_element(),
            ),
            Entry::Header(path) => {
                let path_style = self.project(cx).read(cx).path_style(cx);
                let file_name = path
                    .path
                    .file_name()
                    .map(|name| name.to_string())
                    .unwrap_or_default();
                let directory = path
                    .path
                    .parent()
                    .map(|parent| parent.display(path_style))
                    .map(SharedString::new)
                    .unwrap_or_default();
                let file_icon = ItemSettings::get_global(cx)
                    .file_icons
                    .then(|| FileIcons::get_icon(path.path.as_std_path(), cx))
                    .flatten()
                    .map(|icon| {
                        Icon::from_path(icon)
                            .color(Color::Muted)
                            .size(IconSize::Small)
                    });

                Some(
                    h_flex()
                        .w_full()
                        .min_w_0()
                        .px(DynamicSpacing::Base06.rems(cx))
                        .py_1()
                        .gap_1p5()
                        .children(file_icon)
                        .child(
                            h_flex()
                                .gap_1()
                                .child(Label::new(file_name).size(LabelSize::Small))
                                .when(!directory.is_empty(), |this| {
                                    this.child(
                                        Label::new(directory)
                                            .size(LabelSize::Small)
                                            .color(Color::Muted)
                                            .truncate_start(),
                                    )
                                }),
                        )
                        .into_any_element(),
                )
            }
            Entry::Match(match_index) => {
                let search_match = self.matches.get(*match_index)?;
                Some(
                    ListItem::new(ix)
                        .spacing(ListItemSpacing::Sparse)
                        .inset(true)
                        .toggle_state(selected)
                        .child(
                            h_flex()
                                .w_full()
                                .min_w_0()
                                .gap_2p5()
                                .text_sm()
                                .child(
                                    h_flex()
                                        .w(rems(
                                            (self.max_line_number.max(1).ilog10() + 1) as f32 * 0.5,
                                        ))
                                        .justify_end()
                                        .child(
                                            Label::new(search_match.line_number.to_string()).color(
                                                Color::Custom(
                                                    cx.theme().colors().text_muted.opacity(0.5),
                                                ),
                                            ),
                                        ),
                                )
                                .child(
                                    div()
                                        .flex_1()
                                        .min_w_0()
                                        .truncate()
                                        .child(render_matched_line(search_match, cx)),
                                ),
                        )
                        .into_any_element(),
                )
            }
        }
    }
}

enum ImportedMatches {
    No,
    Yes,
}

async fn stream_results_to_picker(
    cancel_flag: Arc<AtomicBool>,
    text_finder_turning_into_project_search: Arc<AtomicBool>,
    picker: gpui::WeakEntity<Picker<Delegate>>,
    search_results: SearchResults<SearchResult>,
    imported_matches: ImportedMatches,
    cx: &mut AsyncApp,
) -> Option<SearchResults<SearchResult>> {
    let mut results_stream = std::pin::pin!(
        search_results
            .rx
            .clone()
            .ready_chunks(SEARCH_RESULTS_BATCH_SIZE)
    );

    let mut clear_existing = matches!(imported_matches, ImportedMatches::No);
    while let Some(results) = results_stream.next().await {
        if cancel_flag.load(std::sync::atomic::Ordering::SeqCst) {
            break;
        }

        let mut batch_matches = Vec::new();
        let mut limit_reached = false;

        for result in results {
            match result {
                SearchResult::Buffer { buffer, ranges } => {
                    let matches = Delegate::process_search_result(&buffer, &ranges, cx);
                    batch_matches.extend(matches);
                }
                SearchResult::LimitReached => {
                    limit_reached = true;
                }
                SearchResult::WaitingForScan | SearchResult::Searching => {}
            }
        }

        picker
            .update(cx, |picker, cx| {
                let delegate = &mut picker.delegate;

                if clear_existing {
                    delegate.matches.clear();
                    delegate.entries.clear();
                    delegate.unique_files.clear();
                    delegate.selected_index = 0;
                    clear_existing = false;
                }

                delegate
                    .unique_files
                    .extend(batch_matches.iter().map(|m| &m.path).cloned());
                delegate.matches.extend(batch_matches);
                // Rebuild the grouped view and resnap the selection onto a
                // selectable row (the header/separator rows are not selectable).
                delegate.rebuild_entries();

                cx.notify();
            })
            .log_err();

        if limit_reached {
            break;
        }

        // Note the difference with the cancel flag. We need the results to be
        // processed before taking out the search result stream. The cancel flag
        // just needs to stop the search.
        if text_finder_turning_into_project_search.load(Ordering::Relaxed) {
            return Some(search_results);
        }

        smol::future::yield_now().await;
    }
    None
}

/// Renders the matched source line with syntax highlighting, overlaying the
/// search match with a highlighted background and bold weight.
fn render_matched_line(search_match: &SearchMatch, cx: &App) -> StyledText {
    let settings = ThemeSettings::get_global(cx);
    let text_style = TextStyle {
        color: cx.theme().colors().text,
        font_family: settings.buffer_font.family.clone(),
        font_features: settings.buffer_font.features.clone(),
        font_fallbacks: settings.buffer_font.fallbacks.clone(),
        font_size: settings.buffer_font_size(cx).into(),
        font_weight: settings.buffer_font.weight,
        line_height: relative(1.),
        ..Default::default()
    };
    let original_line = &search_match.line_text;
    let line_text = original_line.trim_start();
    let trim_offset = original_line.len() - line_text.len();

    let search_match_style = HighlightStyle {
        background_color: Some(cx.theme().colors().search_match_background),
        font_weight: Some(gpui::FontWeight::BOLD),
        ..Default::default()
    };

    let line_start_abs = search_match.range.start - search_match.relative_range.start;
    let visible_start_abs = line_start_abs + trim_offset;
    let visible_end_abs = line_start_abs + original_line.len();

    // Syntax highlights for the visible (trimmed) portion of the line, with
    // ranges relative to the start of the rendered text.
    let snapshot = search_match.buffer.read(cx).snapshot();
    let syntax_theme = cx.theme().syntax();
    let mut syntax_highlights: Vec<(Range<usize>, HighlightStyle)> = Vec::new();
    let mut current_offset = 0;
    for chunk in snapshot.chunks(
        visible_start_abs..visible_end_abs,
        LanguageAwareStyling {
            tree_sitter: true,
            diagnostics: false,
        },
    ) {
        let chunk_len = chunk.text.len();
        if let Some(style) = chunk
            .syntax_highlight_id
            .and_then(|id| syntax_theme.get(id).copied())
        {
            syntax_highlights.push((current_offset..current_offset + chunk_len, style));
        }
        current_offset += chunk_len;
    }

    // The search match range, clamped to the visible area and made relative to
    // the start of the rendered text.
    let match_start = search_match
        .range
        .start
        .clamp(visible_start_abs, visible_end_abs);
    let match_end = search_match
        .range
        .end
        .clamp(visible_start_abs, visible_end_abs);
    let match_highlight = (
        match_start - visible_start_abs..match_end - visible_start_abs,
        search_match_style,
    );

    let highlights = gpui::combine_highlights(syntax_highlights, [match_highlight]);

    StyledText::new(line_text.to_string()).with_default_highlights(&text_style, highlights)
}

impl Delegate {
    pub(crate) fn build_search_query(
        &mut self,
        query: &str,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<SearchQuery> {
        if query.is_empty() {
            return None;
        }

        // Reuse the include/exclude filters configured on the shared project
        // search view so the text finder respects them too.
        let (files_to_include, files_to_exclude) =
            self.project_search_view.read(cx).file_path_filters(cx);

        // If the project contains multiple visible worktrees, we match the
        // include/exclude patterns against full paths to allow them to be
        // disambiguated. For single worktree projects we use worktree relative
        // paths for convenience.
        let match_full_paths = self.project(cx).read(cx).visible_worktrees(cx).count() > 1;
        let open_buffers = None;

        self.search_options
            .build_query(
                query,
                files_to_include,
                files_to_exclude,
                match_full_paths,
                open_buffers,
            )
            .log_err()
    }

    /// Create things from MB
    pub(crate) fn process_search_result(
        buffer: &Entity<Buffer>,
        ranges: &[Range<Anchor>],
        cx: &AsyncApp,
    ) -> Vec<SearchMatch> {
        if ranges.is_empty() {
            return Vec::new();
        }

        buffer.read_with(cx, |buf, cx| {
            let file = buf.file();
            let path = file.map(|f| ProjectPath {
                worktree_id: f.worktree_id(cx),
                path: f.path().clone(),
            });
            let text = buf.text();

            let mut matches = Vec::new();
            for anchor_range in ranges {
                let start_offset: usize = buf.summary_for_anchor(&anchor_range.start);
                let end_offset: usize = buf.summary_for_anchor(&anchor_range.end);
                let match_row = buf.offset_to_point(start_offset).row;
                let line_number = match_row + 1;
                let line_start = text[..start_offset].rfind('\n').map(|i| i + 1).unwrap_or(0);
                let line_end = text[start_offset..]
                    .find('\n')
                    .map(|i| start_offset + i)
                    .unwrap_or(text.len());
                let line_text = text[line_start..line_end].to_string();

                let relative_start = start_offset - line_start;
                let relative_end = end_offset - line_start;

                if let Some(path) = &path {
                    matches.push(SearchMatch {
                        path: path.clone(),
                        buffer: buffer.clone(),
                        anchor_range: anchor_range.clone(),
                        range: start_offset..end_offset,
                        relative_range: relative_start..relative_end,
                        line_text,
                        line_number,
                    });
                }
            }
            matches
        })
    }
}
