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
    TextStyle,
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
    ActiveTheme, App, ButtonCommon, Color, Context, ContextMenu, Div, FluentBuilder, Icon,
    IconButton, IconName, IconSize, InteractiveElement, IntoElement, Label, LabelCommon, ListItem,
    ListItemSpacing, ParentElement, PopoverMenu, PopoverMenuHandle, SharedString,
    StatefulInteractiveElement, Styled, StyledTypography, Toggleable, Tooltip, Window, div, h_flex,
    relative,
};
use util::ResultExt;
use workspace::SplitDirection;
use workspace::Workspace;
use workspace::item::ItemSettings;

use super::SearchMatch;
use crate::project_search::{ActiveSettings, ProjectSearch};
use crate::{ProjectSearchView, SearchOption, SearchOptions};

/// The text_finder is a minimal modal interface to the project_search. It is
/// targeted towards search for exploration. It can also be used as a filter
/// step to the project_search.
///
/// Basic interaction:
///
/// Open text finder --- Open file ---> File tab
///
///                     (text_finder action)
/// Open text finder --- ToProjectSearch ---> Project search tab
///
/// Can also have a little loop where the user uses the ProjectSearch filters etc
/// to refine the search:
///
///                     (project seach tab)
///                  (removes tab, opens modal)
/// Project search tab --- ToTextFinder ---> Text finder modal
///                             ^                  |
///                             |             ToProjectSeach (adds tab,
///                             |                  |          closes modal)
///                             |                  V
///                             . --------  Project seach tab

pub struct Delegate {
    pub(crate) project_search_view: Entity<ProjectSearchView>,
    pub(crate) focus_handle: FocusHandle,
    pub(crate) matches: Vec<SearchMatch>,
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
    /// Whether the preview is currently shown to the side. Kept in sync by the
    /// picker via [`PickerDelegate::set_horizontal_preview`], because the
    /// delegate cannot read the picker entity while rendering.
    pub(crate) preview_layout_is_horizontal: bool,
    /// Handle for the search-options filter menu, so the picker stays open while
    /// it has focus.
    pub(crate) filter_menu_handle: PopoverMenuHandle<ContextMenu>,
}

async fn get_ongoing_search(
    project_search_view: &Entity<ProjectSearchView>,
    cx: &mut AsyncApp,
) -> Option<SearchResults<SearchResult>> {
    let ongoing_search = project_search_view.update(cx, |view, cx| {
        view.entity.update(cx, |search, _| {
            search.pending_search.take().map(|ongoing| {
                search
                    .project_search_turning_into_text_finder
                    .store(true, Ordering::Relaxed);
                ongoing
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
                preview_layout_is_horizontal: false,
                filter_menu_handle: PopoverMenuHandle::default(),
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

    /// Opens the selected match in a new split in `direction`, then dismisses.
    pub(crate) fn open_in_split(
        &mut self,
        direction: SplitDirection,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) {
        let Some(selected_match) = self.matches.get(self.selected_index) else {
            return;
        };
        let Some(workspace) = self.project_search_view.read(cx).workspace.upgrade() else {
            return;
        };
        let path = selected_match.path.clone();
        let line_number = selected_match.line_number;
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
    type ListItem = ListItem;

    fn name() -> &'static str {
        "text finder"
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Search all files...".into()
    }

    fn has_another_open_menu(&self, window: &Window, cx: &App) -> bool {
        self.filter_menu_handle.is_focused(window, cx) || self.filter_menu_handle.is_deployed()
    }

    fn search_filter(
        &self,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<AnyElement> {
        let active = self.search_options;
        let focus_handle = self.focus_handle.clone();
        let any_active = active.intersects(
            SearchOptions::CASE_SENSITIVE
                | SearchOptions::WHOLE_WORD
                | SearchOptions::REGEX
                | SearchOptions::INCLUDE_IGNORED,
        );
        Some(
            PopoverMenu::new("text-finder-filter-menu")
                .with_handle(self.filter_menu_handle.clone())
                .attach(gpui::Anchor::BottomRight)
                .anchor(gpui::Anchor::TopLeft)
                .trigger(
                    IconButton::new("text-finder-filter", IconName::Sliders)
                        .icon_size(IconSize::Small)
                        .toggle_state(any_active)
                        .tooltip(Tooltip::text("Search Options")),
                )
                .menu({
                    let picker = cx.entity();
                    move |window, cx| {
                        let picker = picker.clone();
                        let focus_handle = focus_handle.clone();
                        Some(ContextMenu::build(window, cx, move |mut menu, _, _| {
                            menu = menu.context(focus_handle.clone());
                            for option in [
                                SearchOption::CaseSensitive,
                                SearchOption::WholeWord,
                                SearchOption::Regex,
                                SearchOption::IncludeIgnored,
                            ] {
                                let options = option.as_options();
                                let is_active = active.contains(options);
                                let picker = picker.clone();
                                menu = menu.custom_entry(
                                    move |_window, _cx| {
                                        let color = if is_active {
                                            Color::Accent
                                        } else {
                                            Color::Default
                                        };
                                        h_flex()
                                            .w_full()
                                            .gap_2()
                                            .child(
                                                Icon::new(option.icon())
                                                    .size(IconSize::Small)
                                                    .color(color),
                                            )
                                            .child(Label::new(option.label()).color(color))
                                            .into_any_element()
                                    },
                                    move |window, cx| {
                                        picker.update(cx, |picker, cx| {
                                            picker.delegate.search_options.toggle(options);
                                            picker.refresh(window, cx);
                                        });
                                    },
                                );
                            }
                            menu
                        }))
                    }
                })
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
        ]
    }

    fn match_count(&self) -> usize {
        self.matches.len()
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

        let Some(selected_match) = self.matches.get(self.selected_index) else {
            return;
        };

        let Some(workspace) = self.project_search_view.read(cx).workspace.upgrade() else {
            return;
        };

        let path = selected_match.path.clone();
        let line_number = selected_match.line_number;

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

    fn try_get_match(&self, _cx: &App) -> Option<picker::PreviewUpdate> {
        let m = self.matches.get(self.selected_index)?;
        Some(picker::PreviewUpdate::from_buffer(
            m.buffer.clone(),
            picker::PreviewHighlight {
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
        let search_match = self.matches.get(ix)?;
        let path = &search_match.path.path;
        let path_style = self.project(cx).read(cx).path_style(cx);
        let file_name = path
            .file_name()
            .map(|name| name.to_string())
            .unwrap_or_default();
        let directory = path
            .parent()
            .map(|parent| parent.display(path_style))
            .map(|parent| SharedString::new(parent))
            .unwrap_or_default();
        let full_path = SharedString::new(path.display(path_style));

        let file_icon = ItemSettings::get_global(cx)
            .file_icons
            .then(|| FileIcons::get_icon(path.as_std_path(), cx))
            .flatten()
            .map(|icon| Icon::from_path(icon).color(Color::Muted));

        let file_location = h_flex()
            .flex_1()
            .min_w_0()
            .overflow_hidden()
            .id(("text-picker-path", ix))
            .tooltip(Tooltip::text(full_path))
            .child(div().flex_none().child(format!("{file_name} ")))
            .when(!directory.is_empty(), |this| {
                this.child(
                    div()
                        .min_w_0()
                        .overflow_hidden()
                        .whitespace_nowrap()
                        .text_ellipsis_start()
                        .text_color(cx.theme().colors().text_muted)
                        .child(directory),
                )
            });

        let rendered_line = if self.preview_layout_is_horizontal {
            h_flex().gap_2().py_px().child(file_location)
        } else {
            h_flex()
                .w_full()
                .gap_4()
                .justify_between()
                .font_buffer(cx)
                .text_buffer(cx)
                .when(!self.preview_layout_is_horizontal, |d| {
                    d.child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .overflow_hidden()
                            .text_ellipsis()
                            .whitespace_nowrap()
                            .child(render_matched_line(search_match, cx)),
                    )
                })
                .child(
                    h_flex()
                        .w(relative(0.35))
                        .flex_none()
                        .gap_2()
                        .child(file_location),
                )
        };

        let line_number = div()
            .flex_none()
            .pr_2()
            .text_color(cx.theme().colors().text_muted)
            .child(search_match.line_number.to_string());
        Some(
            ListItem::new(ix)
                .spacing(ListItemSpacing::Sparse)
                .start_slot::<Icon>(file_icon)
                .end_slot::<Div>(line_number)
                .inset(true)
                .toggle_state(selected)
                .child(rendered_line),
        )
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
            }
        }

        picker
            .update(cx, |picker, cx| {
                let delegate = &mut picker.delegate;

                if clear_existing {
                    delegate.matches.clear();
                    delegate.unique_files.clear();
                    delegate.selected_index = 0;
                    clear_existing = false;
                }

                delegate
                    .unique_files
                    .extend(batch_matches.iter().map(|m| &m.path).cloned());
                delegate.matches.extend(batch_matches);

                if delegate.selected_index >= delegate.matches.len() && !delegate.matches.is_empty()
                {
                    delegate.selected_index = 0;
                }

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
