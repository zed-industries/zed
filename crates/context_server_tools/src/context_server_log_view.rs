use collections::VecDeque;
use context_server::log_store::{ContextServerLogStore, Event, LogKind, Message};
use context_server::{
    ContextServerId, log_store::GlobalContextServerLogStore, types::LoggingLevel,
};
use editor::{Editor, EditorEvent, actions::MoveToEnd, scroll::Autoscroll};
use gpui::{
    App, Context, Corner, Entity, EventEmitter, FocusHandle, Focusable, IntoElement, ParentElement,
    Render, Styled, Subscription, Task, Window, actions, div,
};
use language::language_settings::SoftWrap;
use project::search::SearchQuery;
use std::{any::TypeId, borrow::Cow, sync::Arc};
use strum::IntoEnumIterator;
use ui::{Button, Checkbox, ContextMenu, Label, PopoverMenu, ToggleState, prelude::*};
use workspace::{
    SplitDirection, ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView, Workspace, WorkspaceId,
    item::{Item, ItemHandle},
    searchable::{Direction, SearchEvent, SearchToken, SearchableItem, SearchableItemHandle},
};

actions!(
    dev,
    [
        /// Opens the Context Server logs viewer.
        OpenContextServerLogs
    ]
);

pub fn open_server_logs(
    workspace: gpui::WeakEntity<Workspace>,
    server_id: ContextServerId,
    window: &mut Window,
    cx: &mut gpui::App,
) {
    let log_store = cx.global::<GlobalContextServerLogStore>().0.clone();
    workspace
        .update(cx, |workspace, cx| {
            let project = workspace.project().clone();

            let existing_view = workspace.panes().into_iter().find_map(|pane| {
                pane.read(cx)
                    .items()
                    .find_map(|item| item.downcast::<ContextServerLogView>())
            });

            let log_view = if let Some(view) = existing_view {
                workspace.activate_item(&view, true, true, window, cx);
                view
            } else {
                let new_view =
                    cx.new(|cx| ContextServerLogView::new(project, log_store, window, cx));
                workspace.split_item(
                    SplitDirection::Right,
                    ItemHandle::boxed_clone(&new_view),
                    window,
                    cx,
                );
                new_view
            };

            log_view.update(cx, |log_view, cx| {
                log_view.show_logs_for_server(server_id, window, cx);
            });
        })
        .ok();
}

pub fn init(cx: &mut App) {
    cx.observe_new(move |workspace: &mut Workspace, _, _cx| {
        workspace.register_action(move |workspace, _: &OpenContextServerLogs, window, cx| {
            let log_store = cx.global::<GlobalContextServerLogStore>().0.clone();
            let project = workspace.project().clone();
            let new_tool = cx.new(|cx| ContextServerLogView::new(project, log_store, window, cx));
            workspace.split_item(
                SplitDirection::Right,
                ItemHandle::boxed_clone(&new_tool),
                window,
                cx,
            );
        });
    })
    .detach();
}

pub struct ContextServerLogView {
    pub(crate) editor: Entity<Editor>,
    editor_subscriptions: Vec<Subscription>,
    project: Entity<project::Project>,
    log_store: Entity<ContextServerLogStore>,
    current_server_id: Option<ContextServerId>,
    active_entry_kind: LogKind,
    focus_handle: FocusHandle,
    _log_store_subscriptions: Vec<Subscription>,
}

pub struct ContextServerLogToolbarItemView {
    log_view: Option<Entity<ContextServerLogView>>,
    _log_view_subscription: Option<Subscription>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct LogMenuItem {
    pub server_id: ContextServerId,
    pub server_name: String,
    pub rpc_trace_enabled: bool,
    pub has_stderr: bool,
    pub selected_entry: LogKind,
}

impl ContextServerLogView {
    pub fn new(
        project: Entity<project::Project>,
        log_store: Entity<ContextServerLogStore>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let server_id = log_store.read(cx).servers.keys().next().cloned();

        let model_changes_subscription =
            cx.observe_in(&log_store, window, move |this, store, window, cx| {
                let first_server_id = store.read(cx).servers.keys().next();
                if let Some(current_server) = &this.current_server_id {
                    if !store.read(cx).servers.contains_key(current_server) {
                        if let Some(server_id) = first_server_id {
                            match this.active_entry_kind {
                                LogKind::Rpc => {
                                    this.show_rpc_trace_for_server(server_id.clone(), window, cx)
                                }
                                LogKind::Stderr => {
                                    this.show_stderr_for_server(server_id.clone(), window, cx)
                                }
                                LogKind::Logs | LogKind::Trace | LogKind::ServerInfo => {
                                    this.show_logs_for_server(server_id.clone(), window, cx)
                                }
                            }
                        }
                    }
                } else if let Some(server_id) = first_server_id {
                    match this.active_entry_kind {
                        LogKind::Rpc => {
                            this.show_rpc_trace_for_server(server_id.clone(), window, cx)
                        }
                        LogKind::Stderr => {
                            this.show_stderr_for_server(server_id.clone(), window, cx)
                        }
                        LogKind::Logs | LogKind::Trace | LogKind::ServerInfo => {
                            this.show_logs_for_server(server_id.clone(), window, cx)
                        }
                    }
                }

                cx.notify();
            });

        let events_subscriptions = cx.subscribe_in(
            &log_store,
            window,
            move |log_view, _, e, window, cx| match e {
                Event::NewContextServerLogEntry { id, kind, text } => {
                    if log_view.current_server_id.as_ref() == Some(id)
                        && *kind == log_view.active_entry_kind
                    {
                        log_view.editor.update(cx, |editor, cx| {
                            editor.set_read_only(false);
                            let last_offset = editor.buffer().read(cx).len(cx);
                            let newest_cursor_is_at_end = editor
                                .selections
                                .newest::<editor::MultiBufferOffset>(&editor.display_snapshot(cx))
                                .start
                                >= last_offset;
                            editor.edit(
                                vec![
                                    (last_offset..last_offset, text.as_str()),
                                    (last_offset..last_offset, "\n"),
                                ],
                                cx,
                            );
                            if text.len() > 1024 {
                                let b = editor.buffer().read(cx).as_singleton().unwrap().read(cx);
                                let fold_offset =
                                    b.as_rope().ceil_char_boundary(last_offset.0 + 1024);
                                editor.fold_ranges(
                                    vec![
                                        editor::MultiBufferOffset(fold_offset)
                                            ..editor::MultiBufferOffset(b.as_rope().len()),
                                    ],
                                    false,
                                    window,
                                    cx,
                                );
                            }

                            if newest_cursor_is_at_end {
                                editor.request_autoscroll(Autoscroll::bottom(), cx);
                            }
                            editor.set_read_only(true);
                        });
                    }
                }
            },
        );
        let (editor, editor_subscriptions) = Self::editor_for_logs(String::new(), window, cx);

        let focus_handle = cx.focus_handle();
        let focus_subscription = cx.on_focus(&focus_handle, window, |log_view, window, cx| {
            window.focus(&log_view.editor.focus_handle(cx), cx);
        });

        let mut mcp_log_view = Self {
            focus_handle,
            editor,
            editor_subscriptions,
            project,
            log_store,
            current_server_id: None,
            active_entry_kind: LogKind::Logs,
            _log_store_subscriptions: vec![
                model_changes_subscription,
                events_subscriptions,
                focus_subscription,
            ],
        };
        if let Some(server_id) = server_id {
            mcp_log_view.show_logs_for_server(server_id, window, cx);
        }
        mcp_log_view
    }

    pub(crate) fn menu_items(&self, cx: &mut App) -> Option<Vec<LogMenuItem>> {
        let log_store = self.log_store.read(cx);

        let mut rows = log_store
            .servers
            .iter()
            .map(|(server_id, state)| LogMenuItem {
                server_id: server_id.clone(),
                server_name: format_server_name(
                    &state.name,
                    &self
                        .project
                        .read(cx)
                        .worktree_root_names(cx)
                        .collect::<Vec<_>>(),
                ),
                rpc_trace_enabled: state.rpc_state.is_some(),
                has_stderr: !state.stderr_messages.is_empty(),
                selected_entry: self.active_entry_kind,
            })
            .collect::<Vec<_>>();
        rows.sort_by(|a, b| a.server_name.cmp(&b.server_name));
        Some(rows)
    }

    pub fn show_logs_for_server(
        &mut self,
        server_id: ContextServerId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let log_contents = self.log_store.update(cx, |this, _| {
            let level = this
                .get_server_state(server_id.clone())
                .map(|s| s.log_level)
                .unwrap_or(LoggingLevel::Debug);

            this.server_logs(&server_id).map(|v| log_contents(v, level))
        });

        if let Some(log_contents) = log_contents {
            self.current_server_id = Some(server_id.clone());
            self.active_entry_kind = LogKind::Logs;
            let (editor, editor_subscriptions) = Self::editor_for_logs(log_contents, window, cx);
            self.set_editor_language(&editor, "Log", cx);
            self.editor = editor;
            self.editor_subscriptions = editor_subscriptions;
            cx.notify();
        }
        self.editor.read(cx).focus_handle(cx).focus(window, cx);
        self.log_store.update(cx, |log_store, _| {
            log_store.toggle_logs(server_id, true, LogKind::Logs);
        });
    }

    fn show_stderr_for_server(
        &mut self,
        server_id: ContextServerId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let stderr_contents = self.log_store.update(cx, |this, _| {
            this.server_stderr(&server_id).map(|v| log_contents(v, ()))
        });

        if let Some(stderr_contents) = stderr_contents {
            self.current_server_id = Some(server_id.clone());
            self.active_entry_kind = LogKind::Stderr;
            let (editor, editor_subscriptions) = Self::editor_for_logs(stderr_contents, window, cx);
            self.set_editor_language(&editor, "Log", cx);
            self.editor = editor;
            self.editor_subscriptions = editor_subscriptions;
            cx.notify();
        }
        self.editor.read(cx).focus_handle(cx).focus(window, cx);
        self.log_store.update(cx, |log_store, _| {
            log_store.toggle_logs(server_id, true, LogKind::Stderr);
        });
    }

    fn update_log_level(
        &mut self,
        server_id: ContextServerId,
        level: LoggingLevel,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let log_contents = self.log_store.update(cx, |this, _| {
            if let Some(state) = this.get_server_state(server_id.clone()) {
                state.log_level = level;
            }

            this.server_logs(&server_id).map(|v| log_contents(v, level))
        });

        if let Some(log_contents) = log_contents {
            self.editor.update(cx, |editor, cx| {
                editor.set_text(log_contents, window, cx);
                editor.move_to_end(&MoveToEnd, window, cx);
            });
            cx.notify();
        }

        self.editor.read(cx).focus_handle(cx).focus(window, cx);
    }

    fn show_rpc_trace_for_server(
        &mut self,
        server_id: ContextServerId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.toggle_rpc_trace_for_server(server_id.clone(), true, window, cx);
        let rpc_log = self.log_store.update(cx, |log_store, _| {
            log_store.enable_rpc_trace(server_id.clone());
            log_store
                .server_rpc(&server_id)
                .map(|messages| log_contents(messages, ()))
        });

        if let Some(rpc_log) = rpc_log {
            self.current_server_id = Some(server_id);
            self.active_entry_kind = LogKind::Rpc;
            let (editor, editor_subscriptions) = Self::editor_for_logs(rpc_log, window, cx);
            self.set_editor_language(&editor, "JSON", cx);
            self.editor = editor;
            self.editor_subscriptions = editor_subscriptions;
            cx.notify();
        }

        self.editor.read(cx).focus_handle(cx).focus(window, cx);
    }

    fn toggle_rpc_trace_for_server(
        &mut self,
        server_id: ContextServerId,
        enabled: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.log_store.update(cx, |log_store, _| {
            if enabled {
                log_store.enable_rpc_trace(server_id.clone());
            } else {
                log_store.disable_rpc_trace(server_id.clone());
            }
            log_store.toggle_logs(server_id.clone(), enabled, LogKind::Rpc);
        });
        if !enabled && Some(&server_id) == self.current_server_id.as_ref() {
            self.show_logs_for_server(server_id, window, cx);
            cx.notify();
        }
    }

    fn set_editor_language(
        &self,
        editor: &Entity<Editor>,
        language_name: &str,
        cx: &mut Context<Self>,
    ) {
        let language = self
            .project
            .read(cx)
            .languages()
            .language_for_name(language_name);
        editor
            .read(cx)
            .buffer()
            .read(cx)
            .as_singleton()
            .expect("log buffer should be a singleton")
            .update(cx, |_, cx| {
                cx.spawn({
                    let buffer = cx.entity();
                    async move |_, cx| {
                        // Language may not be available if an extension is not installed.
                        if let Ok(language) = language.await {
                            buffer.update(cx, |buffer, cx| {
                                buffer.set_language(Some(language), cx);
                            });
                        }
                    }
                })
                .detach();
            });
    }

    fn editor_for_logs(
        log_contents: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> (Entity<Editor>, Vec<Subscription>) {
        let editor = initialize_new_editor(log_contents, true, window, cx);
        let editor_subscription = cx.subscribe(&editor, |_, _, event: &EditorEvent, cx| {
            cx.emit(event.clone())
        });
        let search_subscription = cx.subscribe(&editor, |_, _, event: &SearchEvent, cx| {
            cx.emit(event.clone())
        });
        (editor, vec![editor_subscription, search_subscription])
    }
}

fn initialize_new_editor(
    content: String,
    move_to_end: bool,
    window: &mut Window,
    cx: &mut App,
) -> Entity<Editor> {
    cx.new(|cx| {
        let mut editor = Editor::multi_line(window, cx);
        editor.hide_minimap_by_default(window, cx);
        editor.set_text(content, window, cx);
        editor.set_show_git_diff_gutter(false, cx);
        editor.set_show_runnables(false, cx);
        editor.set_show_breakpoints(false, cx);
        editor.set_read_only(true);
        editor.set_show_edit_predictions(Some(false), window, cx);
        editor.set_soft_wrap_mode(SoftWrap::EditorWidth, cx);
        if move_to_end {
            editor.move_to_end(&MoveToEnd, window, cx);
        }
        editor
    })
}

fn log_contents<T: Message>(lines: &VecDeque<T>, level: <T as Message>::Level) -> String {
    lines
        .iter()
        .filter(|message| message.should_include(level))
        .flat_map(|message| [message.as_ref(), "\n"])
        .collect()
}

impl Render for ContextServerLogView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.editor.update(cx, |editor, cx| {
            editor.render(window, cx).into_any_element()
        })
    }
}

impl Focusable for ContextServerLogView {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for ContextServerLogView {
    type Event = EditorEvent;

    fn to_item_events(event: &Self::Event, f: &mut dyn FnMut(workspace::item::ItemEvent)) {
        Editor::to_item_events(event, f)
    }

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        "Context Server Logs".into()
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        None
    }

    fn as_searchable(
        &self,
        handle: &Entity<Self>,
        _: &App,
    ) -> Option<Box<dyn SearchableItemHandle>> {
        Some(Box::new(handle.clone()))
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
            Some(self.editor.clone().into())
        } else {
            None
        }
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
        Task::ready(Some(cx.new(|cx| {
            let mut new_view = Self::new(self.project.clone(), self.log_store.clone(), window, cx);
            if let Some(server_id) = &self.current_server_id {
                match self.active_entry_kind {
                    LogKind::Rpc => {
                        new_view.show_rpc_trace_for_server(server_id.clone(), window, cx)
                    }
                    LogKind::Stderr => {
                        new_view.show_stderr_for_server(server_id.clone(), window, cx)
                    }
                    LogKind::Logs | LogKind::Trace | LogKind::ServerInfo => {
                        new_view.show_logs_for_server(server_id.clone(), window, cx)
                    }
                }
            }
            new_view
        })))
    }
}

impl SearchableItem for ContextServerLogView {
    type Match = <Editor as SearchableItem>::Match;

    fn clear_matches(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.editor.update(cx, |e, cx| e.clear_matches(window, cx))
    }

    fn update_matches(
        &mut self,
        matches: &[Self::Match],
        active_match_index: Option<usize>,
        token: SearchToken,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.editor.update(cx, |e, cx| {
            e.update_matches(matches, active_match_index, token, window, cx)
        })
    }

    fn query_suggestion(&mut self, window: &mut Window, cx: &mut Context<Self>) -> String {
        self.editor
            .update(cx, |e, cx| e.query_suggestion(window, cx))
    }

    fn activate_match(
        &mut self,
        index: usize,
        matches: &[Self::Match],
        token: SearchToken,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.editor.update(cx, |e, cx| {
            e.activate_match(index, matches, token, window, cx)
        })
    }

    fn select_matches(
        &mut self,
        matches: &[Self::Match],
        token: SearchToken,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.editor
            .update(cx, |e, cx| e.select_matches(matches, token, window, cx))
    }

    fn find_matches(
        &mut self,
        query: Arc<SearchQuery>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> gpui::Task<Vec<Self::Match>> {
        self.editor
            .update(cx, |e, cx| e.find_matches(query, window, cx))
    }

    fn replace(
        &mut self,
        _: &Self::Match,
        _: &SearchQuery,
        _token: SearchToken,
        _window: &mut Window,
        _: &mut Context<Self>,
    ) {
    }
    fn supported_options(&self) -> workspace::searchable::SearchOptions {
        workspace::searchable::SearchOptions {
            case: true,
            word: true,
            regex: true,
            find_in_results: false,
            replacement: false,
            selection: false,
        }
    }

    fn active_match_index(
        &mut self,
        direction: Direction,
        matches: &[Self::Match],
        token: SearchToken,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<usize> {
        self.editor.update(cx, |e, cx| {
            e.active_match_index(direction, matches, token, window, cx)
        })
    }
}

const RPC_MESSAGES: &str = "RPC Messages";
const STDERR_LOGS: &str = "Stderr";
const LOGS: &str = "Logs";

impl EventEmitter<ToolbarItemEvent> for ContextServerLogToolbarItemView {}

impl ToolbarItemView for ContextServerLogToolbarItemView {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> workspace::ToolbarItemLocation {
        if let Some(item) = active_pane_item {
            if let Some(log_view) = item.downcast::<ContextServerLogView>() {
                self.log_view = Some(log_view.clone());
                self._log_view_subscription = Some(cx.observe(&log_view, |_, _, cx| {
                    cx.notify();
                }));
                return ToolbarItemLocation::PrimaryLeft;
            }
        }
        self.log_view = None;
        self._log_view_subscription = None;
        ToolbarItemLocation::Hidden
    }
}

impl Render for ContextServerLogToolbarItemView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(log_view) = self.log_view.clone() else {
            return div();
        };

        let (menu_rows, current_server_id) = log_view.update(cx, |log_view, cx| {
            let menu_rows = log_view.menu_items(cx).unwrap_or_default();
            let current_server_id = log_view.current_server_id.clone();
            (menu_rows, current_server_id)
        });

        let current_server = current_server_id.and_then(|current_server_id| {
            menu_rows
                .iter()
                .find(|e| e.server_id == current_server_id)
                .cloned()
        });

        let available_language_servers: Vec<_> = menu_rows
            .into_iter()
            .map(|row| (row.server_id, row.server_name, row.selected_entry))
            .collect();

        let log_toolbar_view = cx.weak_entity();

        let server_menu = PopoverMenu::new("ContextServerLogView")
            .anchor(Corner::TopLeft)
            .trigger(
                Button::new(
                    "context_server_menu_header",
                    current_server
                        .as_ref()
                        .map(|row| Cow::Owned(row.server_name.to_string()))
                        .unwrap_or_else(|| "No server selected".into()),
                )
                .icon(IconName::ChevronDown)
                .icon_size(IconSize::Small)
                .icon_color(Color::Muted),
            )
            .menu({
                let log_view = log_view.clone();
                move |window, cx| {
                    let log_view = log_view.clone();
                    ContextMenu::build(window, cx, |mut menu, window, _| {
                        for (server_id, name, active_entry_kind) in
                            available_language_servers.iter()
                        {
                            let label = name.clone();
                            let server_id = server_id.clone();
                            let active_entry_kind = *active_entry_kind;
                            menu = menu.entry(
                                label,
                                None,
                                window.handler_for(&log_view, move |view, window, cx| {
                                    view.current_server_id = Some(server_id.clone());
                                    view.active_entry_kind = active_entry_kind;
                                    match view.active_entry_kind {
                                        LogKind::Rpc => {
                                            view.toggle_rpc_trace_for_server(
                                                server_id.clone(),
                                                true,
                                                window,
                                                cx,
                                            );
                                            view.show_rpc_trace_for_server(
                                                server_id.clone(),
                                                window,
                                                cx,
                                            );
                                        }
                                        LogKind::Stderr => view.show_stderr_for_server(
                                            server_id.clone(),
                                            window,
                                            cx,
                                        ),
                                        LogKind::Logs | LogKind::Trace | LogKind::ServerInfo => {
                                            view.show_logs_for_server(server_id.clone(), window, cx)
                                        }
                                    }
                                    cx.notify();
                                }),
                            );
                        }
                        menu
                    })
                    .into()
                }
            });

        let view_selector = current_server.map(|server| {
            let server_id = server.server_id;
            let rpc_trace_enabled = server.rpc_trace_enabled;
            let has_stderr = server.has_stderr;
            let log_view = log_view.clone();
            let label = match server.selected_entry {
                LogKind::Rpc => RPC_MESSAGES,
                LogKind::Stderr => STDERR_LOGS,
                _ => LOGS,
            };
            PopoverMenu::new("ContextServerViewSelector")
                .anchor(Corner::TopLeft)
                .trigger(
                    Button::new("context_server_menu_header", label)
                        .icon(IconName::ChevronDown)
                        .icon_size(IconSize::Small)
                        .icon_color(Color::Muted),
                )
                .menu(move |window, cx| {
                    let log_toolbar_view = log_toolbar_view.upgrade()?;
                    let log_view = log_view.clone();
                    let server_id = server_id.clone();
                    Some(ContextMenu::build(
                        window,
                        cx,
                        move |mut this, window, _| {
                            let server_id = server_id.clone();
                            this = this.entry(
                                LOGS,
                                None,
                                window.handler_for(&log_view, {
                                    let server_id = server_id.clone();
                                    move |view, window, cx| {
                                        view.show_logs_for_server(server_id.clone(), window, cx);
                                    }
                                }),
                            );
                            if has_stderr {
                                this = this.entry(
                                    STDERR_LOGS,
                                    None,
                                    window.handler_for(&log_view, {
                                        let server_id = server_id.clone();
                                        move |view, window, cx| {
                                            view.show_stderr_for_server(
                                                server_id.clone(),
                                                window,
                                                cx,
                                            );
                                        }
                                    }),
                                );
                            }
                            this.custom_entry(
                                {
                                    let log_toolbar_view = log_toolbar_view.clone();
                                    let server_id = server_id.clone();
                                    move |window, _| {
                                        h_flex()
                                            .w_full()
                                            .justify_between()
                                            .child(Label::new(RPC_MESSAGES))
                                            .child(
                                                div().child(
                                                    Checkbox::new(
                                                        "ContextServerLogEnableRpcTrace",
                                                        if rpc_trace_enabled {
                                                            ToggleState::Selected
                                                        } else {
                                                            ToggleState::Unselected
                                                        },
                                                    )
                                                    .on_click({
                                                        let server_id = server_id.clone();
                                                        window.listener_for(
                                                            &log_toolbar_view,
                                                            move |view, selection, window, cx| {
                                                                let enabled = matches!(
                                                                    selection,
                                                                    ToggleState::Selected
                                                                );
                                                                view.toggle_rpc_logging_for_server(
                                                                    server_id.clone(),
                                                                    enabled,
                                                                    window,
                                                                    cx,
                                                                );
                                                                cx.stop_propagation();
                                                            },
                                                        )
                                                    }),
                                                ),
                                            )
                                            .into_any_element()
                                    }
                                },
                                window.handler_for(&log_view, move |view, window, cx| {
                                    view.show_rpc_trace_for_server(server_id.clone(), window, cx);
                                }),
                            )
                        },
                    ))
                })
        });

        h_flex()
            .size_full()
            .gap_1()
            .justify_between()
            .child(
                h_flex()
                    .gap_0p5()
                    .child(server_menu)
                    .children(view_selector)
                    .child(
                        log_view.update(cx, |this, _cx| match this.active_entry_kind {
                            LogKind::Logs => {
                                let log_view = log_view.clone();
                                div().child(
                                    PopoverMenu::new("context-server-log-level-menu")
                                        .anchor(Corner::TopLeft)
                                        .trigger(
                                            Button::new(
                                                "context_server_log_level_selector",
                                                "Log level",
                                            )
                                            .icon(IconName::ChevronDown)
                                            .icon_size(IconSize::Small)
                                            .icon_color(Color::Muted),
                                        )
                                        .menu({
                                            let log_view = log_view;

                                            move |window, cx| {
                                                let id = log_view.read(cx).current_server_id.clone()?;

                                                let log_level =
                                                    log_view.update(cx, |this, cx| {
                                                        this.log_store.update(cx, |this, _| {
                                                            Some(
                                                                this.get_server_state(id.clone())?
                                                                    .log_level,
                                                            )
                                                        })
                                                    })?;

                                                Some(ContextMenu::build(
                                                    window,
                                                    cx,
                                                    |mut menu, window, cx| {
                                                        let log_view = log_view.clone();

                                                        for option in LoggingLevel::iter() {
                                                            let label = option.to_string();
                                                            menu = menu.entry(label.clone(), None, {
                                                                let log_view = log_view.clone();
                                                                let id = id.clone();
                                                                move |window, cx| {
                                                                    log_view.update(cx, |this, cx| {
                                                                        this.update_log_level(
                                                                            id.clone(), option, window, cx,
                                                                        );
                                                                    });
                                                                }
                                                            });
                                                            if option == log_level {
                                                                menu.select_last(window, cx);
                                                            }
                                                        }

                                                        menu
                                                    },
                                                ))
                                            }
                                        }),
                                )
                            }
                            _ => div(),
                        }),
                    ),
            )
            .child(
                Button::new("clear_log_button", "Clear").on_click(cx.listener(
                    |this, _, window, cx| {
                        if let Some(log_view) = this.log_view.as_ref() {
                            log_view.update(cx, |log_view, cx| {
                                log_view.editor.update(cx, |editor, cx| {
                                    editor.set_read_only(false);
                                    editor.clear(window, cx);
                                    editor.set_read_only(true);
                                });
                            })
                        }
                    },
                )),
            )
    }
}

impl ContextServerLogToolbarItemView {
    pub fn new() -> Self {
        Self {
            log_view: None,
            _log_view_subscription: None,
        }
    }

    fn toggle_rpc_logging_for_server(
        &mut self,
        id: ContextServerId,
        enabled: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(log_view) = &self.log_view {
            log_view.update(cx, |log_view, cx| {
                log_view.toggle_rpc_trace_for_server(id.clone(), enabled, window, cx);
                if !enabled && Some(&id) == log_view.current_server_id.as_ref() {
                    log_view.show_logs_for_server(id.clone(), window, cx);
                    cx.notify();
                } else if enabled {
                    log_view.show_rpc_trace_for_server(id, window, cx);
                    cx.notify();
                }
                window.focus(&log_view.focus_handle, cx);
            });
        }
        cx.notify();
    }
}

impl EventEmitter<EditorEvent> for ContextServerLogView {}
impl EventEmitter<SearchEvent> for ContextServerLogView {}

fn format_server_name(id: &str, worktree_root_names: &[&str]) -> String {
    if worktree_root_names.is_empty() {
        id.to_string()
    } else {
        format!("{} ({})", id, worktree_root_names.join(", "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_server_name() {
        assert_eq!(format_server_name("my-server", &[]), "my-server");
        assert_eq!(format_server_name("my-server", &["foo"]), "my-server (foo)");
        assert_eq!(
            format_server_name("my-server", &["foo", "bar"]),
            "my-server (foo, bar)"
        );
    }
}
