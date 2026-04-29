use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::Arc;

use crate::agent_connection_store::AgentConnectionStore;

use crate::thread_metadata_store::{
    ThreadId, ThreadMetadata, ThreadMetadataStore, worktree_info_from_thread_paths,
};
use crate::{Agent, ArchiveSelectedThread, DEFAULT_THREAD_TITLE, RemoveSelectedThread};

use agent::ThreadStore;
use agent_client_protocol::schema as acp;
use agent_settings::AgentSettings;
use chrono::{DateTime, Datelike as _, Local, NaiveDate, TimeDelta, Utc};
use collections::HashMap;
use editor::Editor;
use fs::Fs;
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{
    AnyElement, App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable,
    ListState, Render, SharedString, Subscription, Task, WeakEntity, Window, list, prelude::*, px,
};
use itertools::Itertools as _;
use menu::{Confirm, SelectFirst, SelectLast, SelectNext, SelectPrevious};
use picker::{
    Picker, PickerDelegate,
    highlighted_match_with_paths::{HighlightedMatch, HighlightedMatchWithPaths},
};
use project::{AgentId, AgentServerStore};
use settings::Settings as _;
use theme::ActiveTheme;
use ui::{
    AgentThreadStatus, Divider, KeyBinding, ListItem, ListItemSpacing, ListSubHeader, ScrollAxes,
    Scrollbars, Tab, ThreadItem, Tooltip, WithScrollbar, prelude::*,
    utils::platform_title_bar_height,
};
use ui_input::ErasedEditor;
use util::ResultExt;
use util::paths::PathExt;
use workspace::{
    CloseWindow, ModalView, PathList, SerializedWorkspaceLocation, Workspace, WorkspaceDb,
    WorkspaceId, resolve_worktree_workspaces,
};

use zed_actions::agents_sidebar::FocusSidebarFilter;
use zed_actions::editor::{MoveDown, MoveUp};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum ThreadFilter {
    #[default]
    All,
    ArchivedOnly,
}

#[derive(Clone)]
enum ArchiveListItem {
    BucketSeparator(TimeBucket),
    Entry {
        thread: ThreadMetadata,
        highlight_positions: Vec<usize>,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TimeBucket {
    Today,
    Yesterday,
    ThisWeek,
    PastWeek,
    Older,
}

impl TimeBucket {
    fn from_dates(reference: NaiveDate, date: NaiveDate) -> Self {
        if date == reference {
            return TimeBucket::Today;
        }
        if date == reference - TimeDelta::days(1) {
            return TimeBucket::Yesterday;
        }
        let week = date.iso_week();
        if reference.iso_week() == week {
            return TimeBucket::ThisWeek;
        }
        let last_week = (reference - TimeDelta::days(7)).iso_week();
        if week == last_week {
            return TimeBucket::PastWeek;
        }
        TimeBucket::Older
    }

    fn label(&self) -> &'static str {
        match self {
            TimeBucket::Today => "Today",
            TimeBucket::Yesterday => "Yesterday",
            TimeBucket::ThisWeek => "This Week",
            TimeBucket::PastWeek => "Past Week",
            TimeBucket::Older => "Older",
        }
    }
}

fn fuzzy_match_positions(query: &str, text: &str) -> Option<Vec<usize>> {
    let mut positions = Vec::new();
    let mut query_chars = query.chars().peekable();
    for (byte_idx, candidate_char) in text.char_indices() {
        if let Some(&query_char) = query_chars.peek() {
            if candidate_char.eq_ignore_ascii_case(&query_char) {
                positions.push(byte_idx);
                query_chars.next();
            }
        } else {
            break;
        }
    }
    if query_chars.peek().is_none() {
        Some(positions)
    } else {
        None
    }
}

pub enum ThreadsArchiveViewEvent {
    Close,
    Activate { thread: ThreadMetadata },
    CancelRestore { thread_id: ThreadId },
    Import,
}

impl EventEmitter<ThreadsArchiveViewEvent> for ThreadsArchiveView {}

pub struct ThreadsArchiveView {
    _history_subscription: Subscription,
    focus_handle: FocusHandle,
    list_state: ListState,
    items: Vec<ArchiveListItem>,
    selection: Option<usize>,
    hovered_index: Option<usize>,
    preserve_selection_on_next_update: bool,
    filter_editor: Entity<Editor>,
    _subscriptions: Vec<gpui::Subscription>,
    _refresh_history_task: Task<()>,
    workspace: WeakEntity<Workspace>,
    agent_connection_store: WeakEntity<AgentConnectionStore>,
    agent_server_store: WeakEntity<AgentServerStore>,
    restoring: HashSet<ThreadId>,
    archived_thread_ids: HashSet<ThreadId>,
    archived_branch_names: HashMap<ThreadId, HashMap<PathBuf, String>>,
    _load_branch_names_task: Task<()>,
    thread_filter: ThreadFilter,
}

impl ThreadsArchiveView {
    pub fn new(
        workspace: WeakEntity<Workspace>,
        agent_connection_store: WeakEntity<AgentConnectionStore>,
        agent_server_store: WeakEntity<AgentServerStore>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();

        let filter_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("Search all threads…", window, cx);
            editor
        });

        let filter_editor_subscription =
            cx.subscribe(&filter_editor, |this: &mut Self, _, event, cx| {
                if let editor::EditorEvent::BufferEdited = event {
                    this.update_items(cx);
                }
            });

        let filter_focus_handle = filter_editor.read(cx).focus_handle(cx);
        cx.on_focus_in(
            &filter_focus_handle,
            window,
            |this: &mut Self, _window, cx| {
                if this.selection.is_some() {
                    this.selection = None;
                    cx.notify();
                }
            },
        )
        .detach();

        let thread_metadata_store_subscription = cx.observe(
            &ThreadMetadataStore::global(cx),
            |this: &mut Self, _, cx| {
                this.update_items(cx);
                this.reload_branch_names_if_threads_changed(cx);
            },
        );

        cx.on_focus_out(&focus_handle, window, |this: &mut Self, _, _window, cx| {
            this.selection = None;
            cx.notify();
        })
        .detach();

        let mut this = Self {
            _history_subscription: Subscription::new(|| {}),
            focus_handle,
            list_state: ListState::new(0, gpui::ListAlignment::Top, px(1000.)),
            items: Vec::new(),
            selection: None,
            hovered_index: None,
            preserve_selection_on_next_update: false,
            filter_editor,
            _subscriptions: vec![
                filter_editor_subscription,
                thread_metadata_store_subscription,
            ],
            _refresh_history_task: Task::ready(()),
            workspace,
            agent_connection_store,
            agent_server_store,
            restoring: HashSet::default(),
            archived_thread_ids: HashSet::default(),
            archived_branch_names: HashMap::default(),
            _load_branch_names_task: Task::ready(()),
            thread_filter: ThreadFilter::All,
        };

        this.update_items(cx);
        this.reload_branch_names_if_threads_changed(cx);
        this
    }

    pub fn has_selection(&self) -> bool {
        self.selection.is_some()
    }

    pub fn clear_selection(&mut self) {
        self.selection = None;
    }

    pub fn mark_restoring(&mut self, thread_id: &ThreadId, cx: &mut Context<Self>) {
        self.restoring.insert(*thread_id);
        cx.notify();
    }

    pub fn clear_restoring(&mut self, thread_id: &ThreadId, cx: &mut Context<Self>) {
        self.restoring.remove(thread_id);
        cx.notify();
    }

    pub fn focus_filter_editor(&self, window: &mut Window, cx: &mut App) {
        let handle = self.filter_editor.read(cx).focus_handle(cx);
        handle.focus(window, cx);
    }

    pub fn is_filter_editor_focused(&self, window: &Window, cx: &App) -> bool {
        self.filter_editor
            .read(cx)
            .focus_handle(cx)
            .is_focused(window)
    }

    fn update_items(&mut self, cx: &mut Context<Self>) {
        let store = ThreadMetadataStore::global(cx).read(cx);

        // If we're filtering to archived threads but none remain (e.g. the
        // user just deleted the last one), fall back to showing all threads
        // so they aren't stranded with an empty list and a disabled toggle.
        if self.thread_filter == ThreadFilter::ArchivedOnly
            && store.archived_entries().next().is_none()
        {
            self.thread_filter = ThreadFilter::All;
        }

        let thread_filter = self.thread_filter;
        let sessions = store
            .entries()
            .filter(|t| match thread_filter {
                ThreadFilter::All => true,
                ThreadFilter::ArchivedOnly => t.archived,
            })
            .sorted_by_cached_key(|t| t.created_at.unwrap_or(t.updated_at))
            .rev()
            .cloned()
            .collect::<Vec<_>>();

        let query = self.filter_editor.read(cx).text(cx).to_lowercase();
        let today = Local::now().naive_local().date();

        let mut items = Vec::with_capacity(sessions.len() + 5);
        let mut current_bucket: Option<TimeBucket> = None;

        for session in sessions {
            let highlight_positions = if !query.is_empty() {
                match fuzzy_match_positions(
                    &query,
                    session
                        .title
                        .as_ref()
                        .map(|t| t.as_ref())
                        .unwrap_or(DEFAULT_THREAD_TITLE),
                ) {
                    Some(positions) => positions,
                    None => continue,
                }
            } else {
                Vec::new()
            };

            let entry_bucket = {
                let entry_date = session
                    .created_at
                    .unwrap_or(session.updated_at)
                    .with_timezone(&Local)
                    .naive_local()
                    .date();
                TimeBucket::from_dates(today, entry_date)
            };

            if Some(entry_bucket) != current_bucket {
                current_bucket = Some(entry_bucket);
                items.push(ArchiveListItem::BucketSeparator(entry_bucket));
            }

            items.push(ArchiveListItem::Entry {
                thread: session,
                highlight_positions,
            });
        }

        let preserve = self.preserve_selection_on_next_update;
        self.preserve_selection_on_next_update = false;

        let saved_scroll = self.list_state.logical_scroll_top();

        self.list_state.reset(items.len());
        self.items = items;

        if let Some(ix) = self.hovered_index {
            if ix >= self.items.len() || !self.is_selectable_item(ix) {
                self.hovered_index = None;
            }
        }

        self.list_state.scroll_to(saved_scroll);

        if preserve {
            if let Some(ix) = self.selection {
                let next = self.find_next_selectable(ix).or_else(|| {
                    ix.checked_sub(1)
                        .and_then(|i| self.find_previous_selectable(i))
                });
                self.selection = next;
                if let Some(next) = next {
                    self.list_state.scroll_to_reveal_item(next);
                }
            }
        } else {
            self.selection = None;
        }

        cx.notify();
    }

    fn reload_branch_names_if_threads_changed(&mut self, cx: &mut Context<Self>) {
        let current_ids: HashSet<ThreadId> = self
            .items
            .iter()
            .filter_map(|item| match item {
                ArchiveListItem::Entry { thread, .. } => Some(thread.thread_id),
                _ => None,
            })
            .collect();

        if current_ids != self.archived_thread_ids {
            self.archived_thread_ids = current_ids;
            self.load_archived_branch_names(cx);
        }
    }

    fn load_archived_branch_names(&mut self, cx: &mut Context<Self>) {
        let task = ThreadMetadataStore::global(cx)
            .read(cx)
            .get_all_archived_branch_names(cx);
        self._load_branch_names_task = cx.spawn(async move |this, cx| {
            if let Some(branch_names) = task.await.log_err() {
                this.update(cx, |this, cx| {
                    this.archived_branch_names = branch_names;
                    cx.notify();
                })
                .log_err();
            }
        });
    }

    fn reset_filter_editor_text(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.filter_editor.update(cx, |editor, cx| {
            editor.set_text("", window, cx);
        });
    }

    fn archive_thread(&mut self, thread_id: ThreadId, cx: &mut Context<Self>) {
        self.preserve_selection_on_next_update = true;
        ThreadMetadataStore::global(cx).update(cx, |store, cx| store.archive(thread_id, None, cx));
    }

    fn archive_selected_thread(
        &mut self,
        _: &ArchiveSelectedThread,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(ix) = self.selection else { return };
        let Some(ArchiveListItem::Entry { thread, .. }) = self.items.get(ix) else {
            return;
        };

        if thread.archived {
            return;
        }

        self.archive_thread(thread.thread_id, cx);
    }

    fn unarchive_thread(
        &mut self,
        thread: ThreadMetadata,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.restoring.contains(&thread.thread_id) {
            return;
        }

        if thread.folder_paths().is_empty() {
            self.show_project_picker_for_thread(thread, window, cx);
            return;
        }

        self.mark_restoring(&thread.thread_id, cx);
        self.selection = None;
        self.reset_filter_editor_text(window, cx);
        cx.emit(ThreadsArchiveViewEvent::Activate { thread });
    }

    fn show_project_picker_for_thread(
        &mut self,
        thread: ThreadMetadata,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };

        let archive_view = cx.weak_entity();
        let fs = workspace.read(cx).app_state().fs.clone();
        let current_workspace_id = workspace.read(cx).database_id();
        let sibling_workspace_ids: HashSet<WorkspaceId> = workspace
            .read(cx)
            .multi_workspace()
            .and_then(|mw| mw.upgrade())
            .map(|mw| {
                mw.read(cx)
                    .workspaces()
                    .filter_map(|ws| ws.read(cx).database_id())
                    .collect()
            })
            .unwrap_or_default();

        workspace.update(cx, |workspace, cx| {
            workspace.toggle_modal(window, cx, |window, cx| {
                ProjectPickerModal::new(
                    thread,
                    fs,
                    archive_view,
                    current_workspace_id,
                    sibling_workspace_ids,
                    window,
                    cx,
                )
            });
        });
    }

    fn is_selectable_item(&self, ix: usize) -> bool {
        matches!(self.items.get(ix), Some(ArchiveListItem::Entry { .. }))
    }

    fn find_next_selectable(&self, start: usize) -> Option<usize> {
        (start..self.items.len()).find(|&i| self.is_selectable_item(i))
    }

    fn find_previous_selectable(&self, start: usize) -> Option<usize> {
        (0..=start).rev().find(|&i| self.is_selectable_item(i))
    }

    fn editor_move_down(&mut self, _: &MoveDown, window: &mut Window, cx: &mut Context<Self>) {
        self.select_next(&SelectNext, window, cx);
        if self.selection.is_some() {
            self.focus_handle.focus(window, cx);
        }
    }

    fn editor_move_up(&mut self, _: &MoveUp, window: &mut Window, cx: &mut Context<Self>) {
        self.select_previous(&SelectPrevious, window, cx);
        if self.selection.is_some() {
            self.focus_handle.focus(window, cx);
        }
    }

    fn select_next(&mut self, _: &SelectNext, _window: &mut Window, cx: &mut Context<Self>) {
        let next = match self.selection {
            Some(ix) => self.find_next_selectable(ix + 1),
            None => self.find_next_selectable(0),
        };
        if let Some(next) = next {
            self.selection = Some(next);
            self.list_state.scroll_to_reveal_item(next);
            cx.notify();
        }
    }

    fn select_previous(&mut self, _: &SelectPrevious, window: &mut Window, cx: &mut Context<Self>) {
        match self.selection {
            Some(ix) => {
                if let Some(prev) = (ix > 0)
                    .then(|| self.find_previous_selectable(ix - 1))
                    .flatten()
                {
                    self.selection = Some(prev);
                    self.list_state.scroll_to_reveal_item(prev);
                } else {
                    self.selection = None;
                    self.focus_filter_editor(window, cx);
                }
                cx.notify();
            }
            None => {
                let last = self.items.len().saturating_sub(1);
                if let Some(prev) = self.find_previous_selectable(last) {
                    self.selection = Some(prev);
                    self.list_state.scroll_to_reveal_item(prev);
                    cx.notify();
                }
            }
        }
    }

    fn select_first(&mut self, _: &SelectFirst, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(first) = self.find_next_selectable(0) {
            self.selection = Some(first);
            self.list_state.scroll_to_reveal_item(first);
            cx.notify();
        }
    }

    fn select_last(&mut self, _: &SelectLast, _window: &mut Window, cx: &mut Context<Self>) {
        let last = self.items.len().saturating_sub(1);
        if let Some(last) = self.find_previous_selectable(last) {
            self.selection = Some(last);
            self.list_state.scroll_to_reveal_item(last);
            cx.notify();
        }
    }

    fn confirm(&mut self, _: &Confirm, window: &mut Window, cx: &mut Context<Self>) {
        let Some(ix) = self.selection else { return };
        let Some(ArchiveListItem::Entry { thread, .. }) = self.items.get(ix) else {
            return;
        };

        self.unarchive_thread(thread.clone(), window, cx);
    }

    fn render_list_entry(
        &mut self,
        ix: usize,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let Some(item) = self.items.get(ix) else {
            return div().into_any_element();
        };

        match item {
            ArchiveListItem::BucketSeparator(bucket) => div()
                .w_full()
                .px_2p5()
                .pt_3()
                .pb_1()
                .child(
                    Label::new(bucket.label())
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                )
                .into_any_element(),
            ArchiveListItem::Entry {
                thread,
                highlight_positions,
            } => {
                let id = SharedString::from(format!("archive-entry-{}", ix));

                let is_focused = self.selection == Some(ix);
                let is_hovered = self.hovered_index == Some(ix);

                let focus_handle = self.focus_handle.clone();

                let timestamp =
                    format_history_entry_timestamp(thread.created_at.unwrap_or(thread.updated_at));

                let icon_from_external_svg = self
                    .agent_server_store
                    .upgrade()
                    .and_then(|store| store.read(cx).agent_icon(&thread.agent_id));

                let icon = if thread.agent_id.as_ref() == agent::ZED_AGENT_ID.as_ref() {
                    IconName::ZedAgent
                } else {
                    IconName::Sparkle
                };

                let is_restoring = self.restoring.contains(&thread.thread_id);

                let is_archived = thread.archived;

                let branch_names_for_thread: HashMap<PathBuf, SharedString> = self
                    .archived_branch_names
                    .get(&thread.thread_id)
                    .map(|map| {
                        map.iter()
                            .map(|(k, v)| (k.clone(), SharedString::from(v.clone())))
                            .collect()
                    })
                    .unwrap_or_default();

                let worktrees = worktree_info_from_thread_paths(
                    &thread.worktree_paths,
                    &branch_names_for_thread,
                );

                let archived_color = Color::Custom(cx.theme().colors().icon_muted.opacity(0.6));

                let base = ThreadItem::new(id, thread.display_title())
                    .icon(icon)
                    .when(is_archived, |this| {
                        this.archived(true)
                            .icon_color(archived_color)
                            .title_label_color(Color::Muted)
                    })
                    .when_some(icon_from_external_svg, |this, svg| {
                        this.custom_icon_from_external_svg(svg)
                    })
                    .timestamp(timestamp)
                    .highlight_positions(highlight_positions.clone())
                    .project_paths(thread.folder_paths().paths_owned())
                    .worktrees(worktrees)
                    .focused(is_focused)
                    .hovered(is_hovered)
                    .on_hover(cx.listener(move |this, is_hovered, _window, cx| {
                        let previously_hovered = this.hovered_index;
                        this.hovered_index = if *is_hovered {
                            Some(ix)
                        } else {
                            previously_hovered.filter(|&i| i != ix)
                        };
                        if this.hovered_index != previously_hovered {
                            cx.notify();
                        }
                    }));

                if is_restoring {
                    base.status(AgentThreadStatus::Running)
                        .action_slot(
                            IconButton::new("cancel-restore", IconName::Close)
                                .icon_size(IconSize::Small)
                                .icon_color(Color::Muted)
                                .tooltip(Tooltip::text("Cancel Restore"))
                                .on_click({
                                    let thread_id = thread.thread_id;
                                    cx.listener(move |this, _, _, cx| {
                                        this.clear_restoring(&thread_id, cx);
                                        cx.emit(ThreadsArchiveViewEvent::CancelRestore {
                                            thread_id,
                                        });
                                        cx.stop_propagation();
                                    })
                                }),
                        )
                        .into_any_element()
                } else if is_archived {
                    base.action_slot(
                        IconButton::new("delete-thread", IconName::Trash)
                            .icon_size(IconSize::Small)
                            .icon_color(Color::Muted)
                            .tooltip({
                                move |_window, cx| {
                                    Tooltip::for_action_in(
                                        "Delete Thread",
                                        &RemoveSelectedThread,
                                        &focus_handle,
                                        cx,
                                    )
                                }
                            })
                            .on_click({
                                let agent = thread.agent_id.clone();
                                let thread_id = thread.thread_id;
                                let session_id = thread.session_id.clone();
                                cx.listener(move |this, _, _, cx| {
                                    this.preserve_selection_on_next_update = true;
                                    this.delete_thread(
                                        thread_id,
                                        session_id.clone(),
                                        agent.clone(),
                                        cx,
                                    );
                                    cx.stop_propagation();
                                })
                            }),
                    )
                    .on_click({
                        let thread = thread.clone();
                        cx.listener(move |this, _, window, cx| {
                            this.unarchive_thread(thread.clone(), window, cx);
                        })
                    })
                    .into_any_element()
                } else {
                    base.action_slot(
                        IconButton::new("archive-thread", IconName::Archive)
                            .icon_size(IconSize::Small)
                            .icon_color(Color::Muted)
                            .tooltip({
                                move |_window, cx| {
                                    Tooltip::for_action_in(
                                        "Archive Thread",
                                        &ArchiveSelectedThread,
                                        &focus_handle,
                                        cx,
                                    )
                                }
                            })
                            .on_click({
                                let thread_id = thread.thread_id;
                                cx.listener(move |this, _, _, cx| {
                                    this.archive_thread(thread_id, cx);
                                    cx.stop_propagation();
                                })
                            }),
                    )
                    .on_click({
                        let thread = thread.clone();
                        cx.listener(move |this, _, window, cx| {
                            let side = match AgentSettings::get_global(cx).sidebar_side() {
                                settings::SidebarSide::Left => "left",
                                settings::SidebarSide::Right => "right",
                            };
                            telemetry::event!(
                                "Archived Thread Opened",
                                agent = thread.agent_id.as_ref(),
                                side = side
                            );
                            this.unarchive_thread(thread.clone(), window, cx);
                        })
                    })
                    .into_any_element()
                }
            }
        }
    }

    fn remove_selected_thread(
        &mut self,
        _: &RemoveSelectedThread,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(ix) = self.selection else { return };
        let Some(ArchiveListItem::Entry { thread, .. }) = self.items.get(ix) else {
            return;
        };

        self.preserve_selection_on_next_update = true;
        self.delete_thread(
            thread.thread_id,
            thread.session_id.clone(),
            thread.agent_id.clone(),
            cx,
        );
    }

    fn delete_thread(
        &mut self,
        thread_id: ThreadId,
        session_id: Option<acp::SessionId>,
        agent: AgentId,
        cx: &mut Context<Self>,
    ) {
        ThreadMetadataStore::global(cx).update(cx, |store, cx| store.delete(thread_id, cx));

        let agent = Agent::from(agent);

        let Some(agent_connection_store) = self.agent_connection_store.upgrade() else {
            return;
        };
        let fs = <dyn Fs>::global(cx);

        let task = agent_connection_store.update(cx, |store, cx| {
            store
                .request_connection(agent.clone(), agent.server(fs, ThreadStore::global(cx)), cx)
                .read(cx)
                .wait_for_connection()
        });
        cx.spawn(async move |_this, cx| {
            crate::thread_worktree_archive::cleanup_thread_archived_worktrees(thread_id, cx).await;

            let state = task.await?;
            let task = cx.update(|cx| {
                if let Some(session_id) = &session_id {
                    if let Some(list) = state.connection.session_list(cx) {
                        list.delete_session(session_id, cx)
                    } else {
                        Task::ready(Ok(()))
                    }
                } else {
                    Task::ready(Ok(()))
                }
            });
            task.await
        })
        .detach_and_log_err(cx);
    }

    fn render_header(&self, window: &Window, cx: &mut Context<Self>) -> impl IntoElement {
        let has_query = !self.filter_editor.read(cx).text(cx).is_empty();
        let sidebar_on_left = matches!(
            AgentSettings::get_global(cx).sidebar_side(),
            settings::SidebarSide::Left
        );
        let sidebar_on_right = !sidebar_on_left;
        let not_fullscreen = !window.is_fullscreen();
        let traffic_lights = cfg!(target_os = "macos") && not_fullscreen && sidebar_on_left;
        let left_window_controls = !cfg!(target_os = "macos") && not_fullscreen && sidebar_on_left;
        let right_window_controls =
            !cfg!(target_os = "macos") && not_fullscreen && sidebar_on_right;
        let header_height = platform_title_bar_height(window);
        let show_focus_keybinding =
            self.selection.is_some() && !self.filter_editor.focus_handle(cx).is_focused(window);

        h_flex()
            .h(header_height)
            .mt_px()
            .pb_px()
            .when(left_window_controls, |this| {
                this.children(Self::render_left_window_controls(window, cx))
            })
            .map(|this| {
                if traffic_lights {
                    this.pl(px(ui::utils::TRAFFIC_LIGHT_PADDING))
                } else if !left_window_controls {
                    this.pl_1p5()
                } else {
                    this
                }
            })
            .when(!right_window_controls, |this| this.pr_1p5())
            .gap_1()
            .justify_between()
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .when(traffic_lights, |this| {
                this.child(Divider::vertical().color(ui::DividerColor::Border))
            })
            .child(
                h_flex()
                    .ml_1()
                    .min_w_0()
                    .w_full()
                    .gap_1()
                    .child(
                        Icon::new(IconName::MagnifyingGlass)
                            .size(IconSize::Small)
                            .color(Color::Muted),
                    )
                    .child(self.filter_editor.clone()),
            )
            .when(show_focus_keybinding, |this| {
                this.child(KeyBinding::for_action(&FocusSidebarFilter, cx))
            })
            .when(has_query, |this| {
                this.child(
                    IconButton::new("clear-filter", IconName::Close)
                        .icon_size(IconSize::Small)
                        .tooltip(Tooltip::text("Clear Search"))
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.reset_filter_editor_text(window, cx);
                            this.update_items(cx);
                        })),
                )
            })
            .when(right_window_controls, |this| {
                this.children(Self::render_right_window_controls(window, cx))
            })
    }

    fn render_left_window_controls(window: &Window, cx: &mut App) -> Option<AnyElement> {
        platform_title_bar::render_left_window_controls(
            cx.button_layout(),
            Box::new(CloseWindow),
            window,
        )
    }

    fn render_right_window_controls(window: &Window, cx: &mut App) -> Option<AnyElement> {
        platform_title_bar::render_right_window_controls(
            cx.button_layout(),
            Box::new(CloseWindow),
            window,
        )
    }

    fn render_toolbar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let entry_count = self
            .items
            .iter()
            .filter(|item| matches!(item, ArchiveListItem::Entry { .. }))
            .count();

        let has_archived_threads = {
            let store = ThreadMetadataStore::global(cx).read(cx);
            store.archived_entries().next().is_some()
        };

        let count_label = if entry_count == 1 {
            "1 thread".to_string()
        } else {
            format!("{} threads", entry_count)
        };

        h_flex()
            .mt_px()
            .pl_2p5()
            .pr_1p5()
            .h(Tab::content_height(cx))
            .justify_between()
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .child(
                Label::new(count_label)
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            )
            .child(
                h_flex()
                    .gap_1()
                    .child(
                        IconButton::new("thread-import", IconName::Download)
                            .icon_size(IconSize::Small)
                            .tooltip(Tooltip::text("Import Threads"))
                            .on_click(cx.listener(|_this, _, _, cx| {
                                cx.emit(ThreadsArchiveViewEvent::Import);
                            })),
                    )
                    .child(
                        IconButton::new("filter-archived-only", IconName::Archive)
                            .icon_size(IconSize::Small)
                            .disabled(!has_archived_threads)
                            .toggle_state(self.thread_filter == ThreadFilter::ArchivedOnly)
                            .tooltip(Tooltip::text(
                                if self.thread_filter == ThreadFilter::ArchivedOnly {
                                    "Show All Threads"
                                } else {
                                    "Show Only Archived Threads"
                                },
                            ))
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.thread_filter =
                                    if this.thread_filter == ThreadFilter::ArchivedOnly {
                                        ThreadFilter::All
                                    } else {
                                        ThreadFilter::ArchivedOnly
                                    };
                                this.update_items(cx);
                            })),
                    ),
            )
    }
}

pub fn format_history_entry_timestamp(entry_time: DateTime<Utc>) -> String {
    let now = Utc::now();
    let duration = now.signed_duration_since(entry_time);

    let minutes = duration.num_minutes();
    let hours = duration.num_hours();
    let days = duration.num_days();
    let weeks = days / 7;
    let months = days / 30;

    if minutes < 60 {
        format!("{}m", minutes.max(1))
    } else if hours < 24 {
        format!("{}h", hours.max(1))
    } else if days < 7 {
        format!("{}d", days.max(1))
    } else if weeks < 4 {
        format!("{}w", weeks.max(1))
    } else {
        format!("{}mo", months.max(1))
    }
}

impl Focusable for ThreadsArchiveView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for ThreadsArchiveView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_empty = self.items.is_empty();
        let has_query = !self.filter_editor.read(cx).text(cx).is_empty();

        let content = if is_empty {
            let message = if has_query {
                "No threads match your search."
            } else {
                "No threads yet."
            };

            v_flex()
                .flex_1()
                .justify_center()
                .items_center()
                .child(
                    Label::new(message)
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                )
                .into_any_element()
        } else {
            v_flex()
                .flex_1()
                .overflow_hidden()
                .child(
                    list(
                        self.list_state.clone(),
                        cx.processor(Self::render_list_entry),
                    )
                    .flex_1()
                    .size_full(),
                )
                .custom_scrollbars(
                    Scrollbars::new(ScrollAxes::Vertical).tracked_scroll_handle(&self.list_state),
                    window,
                    cx,
                )
                .into_any_element()
        };

        v_flex()
            .key_context("ThreadsArchiveView")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::select_next))
            .on_action(cx.listener(Self::select_previous))
            .on_action(cx.listener(Self::editor_move_down))
            .on_action(cx.listener(Self::editor_move_up))
            .on_action(cx.listener(Self::select_first))
            .on_action(cx.listener(Self::select_last))
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(Self::remove_selected_thread))
            .on_action(cx.listener(Self::archive_selected_thread))
            .size_full()
            .child(self.render_header(window, cx))
            .when(!has_query, |this| this.child(self.render_toolbar(cx)))
            .child(content)
    }
}

struct ProjectPickerModal {
    picker: Entity<Picker<ProjectPickerDelegate>>,
    _subscription: Subscription,
}

impl ProjectPickerModal {
    fn new(
        thread: ThreadMetadata,
        fs: Arc<dyn Fs>,
        archive_view: WeakEntity<ThreadsArchiveView>,
        current_workspace_id: Option<WorkspaceId>,
        sibling_workspace_ids: HashSet<WorkspaceId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let delegate = ProjectPickerDelegate {
            thread,
            archive_view,
            workspaces: Vec::new(),
            filtered_entries: Vec::new(),
            selected_index: 0,
            current_workspace_id,
            sibling_workspace_ids,
            focus_handle: cx.focus_handle(),
        };

        let picker = cx.new(|cx| {
            Picker::list(delegate, window, cx)
                .list_measure_all()
                .modal(false)
        });

        let picker_focus_handle = picker.focus_handle(cx);
        picker.update(cx, |picker, _| {
            picker.delegate.focus_handle = picker_focus_handle;
        });

        let _subscription =
            cx.subscribe(&picker, |_this: &mut Self, _, _event: &DismissEvent, cx| {
                cx.emit(DismissEvent);
            });

        let db = WorkspaceDb::global(cx);
        cx.spawn_in(window, async move |this, cx| {
            let workspaces = db
                .recent_project_workspaces(fs.as_ref())
                .await
                .log_err()
                .unwrap_or_default();
            let workspaces = resolve_worktree_workspaces(workspaces, fs.as_ref()).await;
            this.update_in(cx, move |this, window, cx| {
                this.picker.update(cx, move |picker, cx| {
                    picker.delegate.workspaces = workspaces;
                    picker.update_matches(picker.query(cx), window, cx)
                })
            })
            .ok();
        })
        .detach();

        picker.focus_handle(cx).focus(window, cx);

        Self {
            picker,
            _subscription,
        }
    }
}

impl EventEmitter<DismissEvent> for ProjectPickerModal {}

impl Focusable for ProjectPickerModal {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl ModalView for ProjectPickerModal {}

impl Render for ProjectPickerModal {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("ProjectPickerModal")
            .elevation_3(cx)
            .w(rems(34.))
            .on_action(cx.listener(|this, _: &workspace::Open, window, cx| {
                this.picker.update(cx, |picker, cx| {
                    picker.delegate.open_local_folder(window, cx)
                })
            }))
            .child(self.picker.clone())
    }
}

enum ProjectPickerEntry {
    Header(SharedString),
    Workspace(StringMatch),
}

struct ProjectPickerDelegate {
    thread: ThreadMetadata,
    archive_view: WeakEntity<ThreadsArchiveView>,
    current_workspace_id: Option<WorkspaceId>,
    sibling_workspace_ids: HashSet<WorkspaceId>,
    workspaces: Vec<(
        WorkspaceId,
        SerializedWorkspaceLocation,
        PathList,
        DateTime<Utc>,
    )>,
    filtered_entries: Vec<ProjectPickerEntry>,
    selected_index: usize,
    focus_handle: FocusHandle,
}

impl ProjectPickerDelegate {
    fn update_working_directories_and_unarchive(
        &mut self,
        paths: PathList,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) {
        self.thread.worktree_paths =
            super::thread_metadata_store::WorktreePaths::from_folder_paths(&paths);
        ThreadMetadataStore::global(cx).update(cx, |store, cx| {
            store.update_working_directories(self.thread.thread_id, paths, cx);
        });

        self.archive_view
            .update(cx, |view, cx| {
                view.selection = None;
                view.reset_filter_editor_text(window, cx);
                cx.emit(ThreadsArchiveViewEvent::Activate {
                    thread: self.thread.clone(),
                });
            })
            .log_err();
    }

    fn is_current_workspace(&self, workspace_id: WorkspaceId) -> bool {
        self.current_workspace_id == Some(workspace_id)
    }

    fn is_sibling_workspace(&self, workspace_id: WorkspaceId) -> bool {
        self.sibling_workspace_ids.contains(&workspace_id)
            && !self.is_current_workspace(workspace_id)
    }

    fn selected_match(&self) -> Option<&StringMatch> {
        match self.filtered_entries.get(self.selected_index)? {
            ProjectPickerEntry::Workspace(hit) => Some(hit),
            ProjectPickerEntry::Header(_) => None,
        }
    }

    fn open_local_folder(&mut self, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let paths_receiver = cx.prompt_for_paths(gpui::PathPromptOptions {
            files: false,
            directories: true,
            multiple: false,
            prompt: None,
        });
        cx.spawn_in(window, async move |this, cx| {
            let Ok(Ok(Some(paths))) = paths_receiver.await else {
                return;
            };
            if paths.is_empty() {
                return;
            }

            let work_dirs = PathList::new(&paths);

            this.update_in(cx, |this, window, cx| {
                this.delegate
                    .update_working_directories_and_unarchive(work_dirs, window, cx);
                cx.emit(DismissEvent);
            })
            .log_err();
        })
        .detach();
    }
}

impl EventEmitter<DismissEvent> for ProjectPickerDelegate {}

impl PickerDelegate for ProjectPickerDelegate {
    type ListItem = AnyElement;

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        format!(
            "Associate the \"{}\" thread with...",
            self.thread
                .title
                .as_ref()
                .map(|t| t.as_ref())
                .unwrap_or(DEFAULT_THREAD_TITLE)
        )
        .into()
    }

    fn render_editor(
        &self,
        editor: &Arc<dyn ErasedEditor>,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Div {
        h_flex()
            .flex_none()
            .h_9()
            .px_2p5()
            .justify_between()
            .border_b_1()
            .border_color(cx.theme().colors().border_variant)
            .child(editor.render(window, cx))
    }

    fn match_count(&self) -> usize {
        self.filtered_entries.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) {
        self.selected_index = ix;
    }

    fn can_select(&self, ix: usize, _window: &mut Window, _cx: &mut Context<Picker<Self>>) -> bool {
        matches!(
            self.filtered_entries.get(ix),
            Some(ProjectPickerEntry::Workspace(_))
        )
    }

    fn update_matches(
        &mut self,
        query: String,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        let query = query.trim_start();
        let smart_case = query.chars().any(|c| c.is_uppercase());
        let is_empty_query = query.is_empty();

        let sibling_candidates: Vec<_> = self
            .workspaces
            .iter()
            .enumerate()
            .filter(|(_, (id, _, _, _))| self.is_sibling_workspace(*id))
            .map(|(id, (_, _, paths, _))| {
                let combined_string = paths
                    .ordered_paths()
                    .map(|path| path.compact().to_string_lossy().into_owned())
                    .collect::<Vec<_>>()
                    .join("");
                StringMatchCandidate::new(id, &combined_string)
            })
            .collect();

        let mut sibling_matches = gpui::block_on(fuzzy::match_strings(
            &sibling_candidates,
            query,
            smart_case,
            true,
            100,
            &Default::default(),
            cx.background_executor().clone(),
        ));

        sibling_matches.sort_unstable_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.candidate_id.cmp(&b.candidate_id))
        });

        let recent_candidates: Vec<_> = self
            .workspaces
            .iter()
            .enumerate()
            .filter(|(_, (id, _, _, _))| {
                !self.is_current_workspace(*id) && !self.is_sibling_workspace(*id)
            })
            .map(|(id, (_, _, paths, _))| {
                let combined_string = paths
                    .ordered_paths()
                    .map(|path| path.compact().to_string_lossy().into_owned())
                    .collect::<Vec<_>>()
                    .join("");
                StringMatchCandidate::new(id, &combined_string)
            })
            .collect();

        let mut recent_matches = gpui::block_on(fuzzy::match_strings(
            &recent_candidates,
            query,
            smart_case,
            true,
            100,
            &Default::default(),
            cx.background_executor().clone(),
        ));

        recent_matches.sort_unstable_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.candidate_id.cmp(&b.candidate_id))
        });

        let mut entries = Vec::new();

        let has_siblings_to_show = if is_empty_query {
            !sibling_candidates.is_empty()
        } else {
            !sibling_matches.is_empty()
        };

        if has_siblings_to_show {
            entries.push(ProjectPickerEntry::Header("This Window".into()));

            if is_empty_query {
                for (id, (workspace_id, _, _, _)) in self.workspaces.iter().enumerate() {
                    if self.is_sibling_workspace(*workspace_id) {
                        entries.push(ProjectPickerEntry::Workspace(StringMatch {
                            candidate_id: id,
                            score: 0.0,
                            positions: Vec::new(),
                            string: String::new(),
                        }));
                    }
                }
            } else {
                for m in sibling_matches {
                    entries.push(ProjectPickerEntry::Workspace(m));
                }
            }
        }

        let has_recent_to_show = if is_empty_query {
            !recent_candidates.is_empty()
        } else {
            !recent_matches.is_empty()
        };

        if has_recent_to_show {
            entries.push(ProjectPickerEntry::Header("Recent Projects".into()));

            if is_empty_query {
                for (id, (workspace_id, _, _, _)) in self.workspaces.iter().enumerate() {
                    if !self.is_current_workspace(*workspace_id)
                        && !self.is_sibling_workspace(*workspace_id)
                    {
                        entries.push(ProjectPickerEntry::Workspace(StringMatch {
                            candidate_id: id,
                            score: 0.0,
                            positions: Vec::new(),
                            string: String::new(),
                        }));
                    }
                }
            } else {
                for m in recent_matches {
                    entries.push(ProjectPickerEntry::Workspace(m));
                }
            }
        }

        self.filtered_entries = entries;

        self.selected_index = self
            .filtered_entries
            .iter()
            .position(|e| matches!(e, ProjectPickerEntry::Workspace(_)))
            .unwrap_or(0);

        Task::ready(())
    }

    fn confirm(&mut self, _secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let candidate_id = match self.filtered_entries.get(self.selected_index) {
            Some(ProjectPickerEntry::Workspace(hit)) => hit.candidate_id,
            _ => return,
        };
        let Some((_workspace_id, _location, paths, _)) = self.workspaces.get(candidate_id) else {
            return;
        };

        self.update_working_directories_and_unarchive(paths.clone(), window, cx);
        cx.emit(DismissEvent);
    }

    fn dismissed(&mut self, _window: &mut Window, _cx: &mut Context<Picker<Self>>) {}

    fn no_matches_text(&self, _window: &mut Window, _cx: &mut App) -> Option<SharedString> {
        let text = if self.workspaces.is_empty() {
            "No recent projects found"
        } else {
            "No matches"
        };
        Some(text.into())
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        match self.filtered_entries.get(ix)? {
            ProjectPickerEntry::Header(title) => Some(
                v_flex()
                    .w_full()
                    .gap_1()
                    .when(ix > 0, |this| this.mt_1().child(Divider::horizontal()))
                    .child(ListSubHeader::new(title.clone()).inset(true))
                    .into_any_element(),
            ),
            ProjectPickerEntry::Workspace(hit) => {
                let (_, location, paths, _) = self.workspaces.get(hit.candidate_id)?;

                let ordered_paths: Vec<_> = paths
                    .ordered_paths()
                    .map(|p| p.compact().to_string_lossy().to_string())
                    .collect();

                let tooltip_path: SharedString = ordered_paths.join("\n").into();

                let mut path_start_offset = 0;
                let match_labels: Vec<_> = paths
                    .ordered_paths()
                    .map(|p| p.compact())
                    .map(|path| {
                        let path_string = path.to_string_lossy();
                        let path_text = path_string.to_string();
                        let path_byte_len = path_text.len();

                        let path_positions: Vec<usize> = hit
                            .positions
                            .iter()
                            .copied()
                            .skip_while(|pos| *pos < path_start_offset)
                            .take_while(|pos| *pos < path_start_offset + path_byte_len)
                            .map(|pos| pos - path_start_offset)
                            .collect();

                        let file_name_match = path.file_name().map(|file_name| {
                            let file_name_text = file_name.to_string_lossy().into_owned();
                            let file_name_start = path_byte_len - file_name_text.len();
                            let highlight_positions: Vec<usize> = path_positions
                                .iter()
                                .copied()
                                .skip_while(|pos| *pos < file_name_start)
                                .take_while(|pos| *pos < file_name_start + file_name_text.len())
                                .map(|pos| pos - file_name_start)
                                .collect();
                            HighlightedMatch {
                                text: file_name_text,
                                highlight_positions,
                                color: Color::Default,
                            }
                        });

                        path_start_offset += path_byte_len;
                        file_name_match
                    })
                    .collect();

                let highlighted_match = HighlightedMatchWithPaths {
                    prefix: match location {
                        SerializedWorkspaceLocation::Remote(options) => {
                            Some(SharedString::from(options.display_name()))
                        }
                        _ => None,
                    },
                    match_label: HighlightedMatch::join(match_labels.into_iter().flatten(), ", "),
                    paths: Vec::new(),
                    active: false,
                };

                Some(
                    ListItem::new(ix)
                        .toggle_state(selected)
                        .inset(true)
                        .spacing(ListItemSpacing::Sparse)
                        .child(
                            h_flex()
                                .gap_3()
                                .flex_grow()
                                .child(highlighted_match.render(window, cx)),
                        )
                        .tooltip(Tooltip::text(tooltip_path))
                        .into_any_element(),
                )
            }
        }
    }

    fn render_footer(&self, _: &mut Window, cx: &mut Context<Picker<Self>>) -> Option<AnyElement> {
        let has_selection = self.selected_match().is_some();
        let focus_handle = self.focus_handle.clone();

        Some(
            h_flex()
                .flex_1()
                .p_1p5()
                .gap_1()
                .justify_end()
                .border_t_1()
                .border_color(cx.theme().colors().border_variant)
                .child(
                    Button::new("open_local_folder", "Choose from Local Folders")
                        .key_binding(KeyBinding::for_action_in(
                            &workspace::Open::default(),
                            &focus_handle,
                            cx,
                        ))
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.delegate.open_local_folder(window, cx);
                        })),
                )
                .child(
                    Button::new("select_project", "Select")
                        .disabled(!has_selection)
                        .key_binding(KeyBinding::for_action_in(&menu::Confirm, &focus_handle, cx))
                        .on_click(cx.listener(move |picker, _, window, cx| {
                            picker.delegate.confirm(false, window, cx);
                        })),
                )
                .into_any(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuzzy_match_positions_returns_byte_indices() {
        // "🔥abc" — the fire emoji is 4 bytes, so 'a' starts at byte 4, 'b' at 5, 'c' at 6.
        let text = "🔥abc";
        let positions = fuzzy_match_positions("ab", text).expect("should match");
        assert_eq!(positions, vec![4, 5]);

        // Verify positions are valid char boundaries (this is the assertion that
        // panicked before the fix).
        for &pos in &positions {
            assert!(
                text.is_char_boundary(pos),
                "position {pos} is not a valid UTF-8 boundary in {text:?}"
            );
        }
    }

    #[test]
    fn test_fuzzy_match_positions_ascii_still_works() {
        let positions = fuzzy_match_positions("he", "hello").expect("should match");
        assert_eq!(positions, vec![0, 1]);
    }

    #[test]
    fn test_fuzzy_match_positions_case_insensitive() {
        let positions = fuzzy_match_positions("HE", "hello").expect("should match");
        assert_eq!(positions, vec![0, 1]);
    }

    #[test]
    fn test_fuzzy_match_positions_no_match() {
        assert!(fuzzy_match_positions("xyz", "hello").is_none());
    }

    #[test]
    fn test_fuzzy_match_positions_multi_byte_interior() {
        // "café" — 'é' is 2 bytes (0xC3 0xA9), so 'f' starts at byte 4, 'é' at byte 5.
        let text = "café";
        let positions = fuzzy_match_positions("fé", text).expect("should match");
        // 'c'=0, 'a'=1, 'f'=2, 'é'=3..4 — wait, let's verify:
        // Actually: c=1 byte, a=1 byte, f=1 byte, é=2 bytes
        // So byte positions: c=0, a=1, f=2, é=3
        assert_eq!(positions, vec![2, 3]);
        for &pos in &positions {
            assert!(
                text.is_char_boundary(pos),
                "position {pos} is not a valid UTF-8 boundary in {text:?}"
            );
        }
    }
}
