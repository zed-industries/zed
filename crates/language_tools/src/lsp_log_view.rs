use collections::{HashMap, HashSet, VecDeque};
use edit_prediction::EditPredictionStore;
use editor::{Editor, EditorEvent, MultiBufferOffset, actions::MoveToEnd, scroll::Autoscroll};
use gpui::{
    Anchor, App, Context, Entity, EventEmitter, FocusHandle, Focusable, IntoElement, ParentElement,
    Render, Styled, Subscription, Task, WeakEntity, Window, actions, div,
};
use itertools::Itertools as _;
use language::{LanguageServerId, language_settings::SoftWrap};
use lsp::{
    LanguageServer, LanguageServerName, LanguageServerSelector, MessageType, SetTraceParams,
    TraceValue, notification::SetTrace,
};
use project::{
    LanguageServerStatus, Project,
    lsp_store::log_store::{
        self, Event, LanguageServerKind, LanguageServerLogKey, LogKind, LogStore, Message,
    },
    search::SearchQuery,
};
use proto::toggle_lsp_logs::LogType;
use settings::SeedQuerySetting;
use std::{any::TypeId, borrow::Cow, sync::Arc};
use ui::{Checkbox, ContextMenu, PopoverMenu, ToggleState, prelude::*};
use util::ResultExt as _;
use workspace::{
    SplitDirection, ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView, Workspace, WorkspaceId,
    item::{Item, ItemHandle},
    searchable::{Direction, SearchEvent, SearchToken, SearchableItem, SearchableItemHandle},
};

use crate::get_or_create_tool;

pub fn open(
    log_store: &Entity<LogStore>,
    workspace: WeakEntity<Workspace>,
    server: LanguageServerSelector,
    window: &mut Window,
    cx: &mut App,
) {
    log_store.update(cx, |_, cx| {
        cx.spawn_in(window, async move |log_store, cx| {
            let Some(log_store) = log_store.upgrade() else {
                return;
            };
            workspace
                .update_in(cx, |workspace, window, cx| {
                    let project = workspace.project().clone();
                    let weak_project = project.downgrade();
                    let project_is_local = project.read(cx).is_local();
                    let weak_lsp_store = project.read(cx).lsp_store().downgrade();
                    let tool_log_store = log_store.clone();
                    let log_view = get_or_create_tool(
                        workspace,
                        SplitDirection::Right,
                        window,
                        cx,
                        move |window, cx| LspLogView::new(project, tool_log_store, window, cx),
                    );
                    log_view.update(cx, |log_view, cx| {
                        let key_priority = |key: &LanguageServerLogKey| match &key.kind {
                            LanguageServerKind::Local { .. } if project_is_local => 0,
                            LanguageServerKind::Remote { .. } if !project_is_local => 0,
                            _ => 1,
                        };
                        let server_key = match server {
                            LanguageServerSelector::Id(id) => log_store
                                .read(cx)
                                .language_servers
                                .keys()
                                .filter(|key| {
                                    key.server_id == id
                                        && key.is_for_project(&weak_project, &weak_lsp_store)
                                })
                                .min_by_key(|key| key_priority(key))
                                .cloned(),
                            LanguageServerSelector::Name(name) => log_store
                                .read(cx)
                                .language_servers
                                .iter()
                                .filter(|(key, state)| {
                                    key.is_for_project(&weak_project, &weak_lsp_store)
                                        && state.name.as_ref() == Some(&name)
                                })
                                .min_by_key(|(key, _)| key_priority(key))
                                .map(|(key, _)| key.clone()),
                        };
                        if let Some(server_key) = server_key {
                            log_view.show_logs_for_server(server_key, window, cx);
                        }
                    });
                })
                .ok();
        })
        .detach();
    })
}

pub struct LspLogView {
    pub(crate) editor: Entity<Editor>,
    editor_subscriptions: Vec<Subscription>,
    log_store: Entity<LogStore>,
    current_server_key: Option<LanguageServerLogKey>,
    active_entry_kind: LogKind,
    enabled_streams: HashMap<LanguageServerLogKey, HashSet<LogKind>>,
    project: Entity<Project>,
    focus_handle: FocusHandle,
    _log_store_subscriptions: Vec<Subscription>,
}

pub struct LspLogToolbarItemView {
    log_view: Option<Entity<LspLogView>>,
    _log_view_subscription: Option<Subscription>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct LogMenuItem {
    pub server_id: LanguageServerId,
    pub server_name: LanguageServerName,
    pub worktree_root_name: String,
    pub rpc_trace_enabled: bool,
    pub selected_entry: LogKind,
    pub trace_level: lsp::TraceValue,
    pub server_kind: LanguageServerKind,
}

impl LogMenuItem {
    fn key(&self) -> LanguageServerLogKey {
        LanguageServerLogKey::new(self.server_kind.clone(), self.server_id)
    }
}

actions!(
    dev,
    [
        /// Opens the language server protocol logs viewer.
        OpenLanguageServerLogs
    ]
);

pub fn init(on_headless_host: bool, cx: &mut App) {
    let log_store = log_store::init(on_headless_host, cx);

    cx.observe_new(move |workspace: &mut Workspace, _, cx| {
        log_store.update(cx, |store, cx| {
            store.add_project(workspace.project(), cx);
        });

        let log_store = log_store.clone();
        workspace.register_action(move |workspace, _: &OpenLanguageServerLogs, window, cx| {
            let log_store = log_store.clone();
            let project = workspace.project().clone();
            get_or_create_tool(
                workspace,
                SplitDirection::Right,
                window,
                cx,
                move |window, cx| LspLogView::new(project, log_store, window, cx),
            );
        });
    })
    .detach();
}

impl LspLogView {
    pub fn new(
        project: Entity<Project>,
        log_store: Entity<LogStore>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let weak_project = project.downgrade();
        let weak_lsp_store = project.read(cx).lsp_store().downgrade();
        let server_key = log_store
            .read(cx)
            .server_keys_for_project(&weak_project, &weak_lsp_store)
            .next();
        let model_changes_subscription =
            cx.observe_in(&log_store, window, move |this, store, window, cx| {
                let first_server_key_for_project = store
                    .read(cx)
                    .server_keys_for_project(&weak_project, &weak_lsp_store)
                    .next();
                if let Some(current_key) = this.current_server_key.as_ref() {
                    let current_server_is_for_project =
                        store.read(cx).language_servers.contains_key(current_key)
                            && current_key.is_for_project(&weak_project, &weak_lsp_store);
                    if !current_server_is_for_project
                        && let Some(server_key) = first_server_key_for_project
                    {
                        match this.active_entry_kind {
                            LogKind::Rpc => this.show_rpc_trace_for_server(server_key, window, cx),
                            LogKind::Trace => this.show_trace_for_server(server_key, window, cx),
                            LogKind::Logs => this.show_logs_for_server(server_key, window, cx),
                            LogKind::ServerInfo => this.show_server_info(server_key, window, cx),
                        }
                    }
                } else if let Some(server_key) = first_server_key_for_project {
                    match this.active_entry_kind {
                        LogKind::Rpc => this.show_rpc_trace_for_server(server_key, window, cx),
                        LogKind::Trace => this.show_trace_for_server(server_key, window, cx),
                        LogKind::Logs => this.show_logs_for_server(server_key, window, cx),
                        LogKind::ServerInfo => this.show_server_info(server_key, window, cx),
                    }
                }

                cx.notify();
            });

        let events_subscriptions = cx.subscribe_in(
            &log_store,
            window,
            move |log_view, _, e, window, cx| match e {
                Event::NewServerLogEntry { key, kind, text } => {
                    if log_view.current_server_key.as_ref() == Some(key)
                        && LogKind::from_server_log_type(kind) == log_view.active_entry_kind
                    {
                        log_view.editor.update(cx, |editor, cx| {
                            editor.set_read_only(false);
                            let last_offset = editor.buffer().read(cx).len(cx);
                            let newest_cursor_is_at_end = editor
                                .selections
                                .newest::<MultiBufferOffset>(&editor.display_snapshot(cx))
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
                                        MultiBufferOffset(fold_offset)
                                            ..MultiBufferOffset(b.as_rope().len()),
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

        cx.on_release(|log_view, cx| {
            let enabled_streams = log_view
                .enabled_streams
                .iter()
                .flat_map(|(key, log_kinds)| {
                    log_kinds
                        .iter()
                        .map(|log_kind| (key.clone(), *log_kind))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            for (key, log_kind) in enabled_streams {
                log_view.set_stream_enabled(&key, log_kind, false, cx);
            }
        })
        .detach();

        let mut lsp_log_view = Self {
            focus_handle,
            editor,
            editor_subscriptions,
            project,
            log_store,
            current_server_key: None,
            active_entry_kind: LogKind::Logs,
            enabled_streams: HashMap::default(),
            _log_store_subscriptions: vec![
                model_changes_subscription,
                events_subscriptions,
                focus_subscription,
            ],
        };
        if let Some(server_key) = server_key {
            lsp_log_view.show_logs_for_server(server_key, window, cx);
        }
        lsp_log_view
    }

    fn editor_for_logs(
        log_contents: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> (Entity<Editor>, Vec<Subscription>) {
        let editor = initialize_new_editor(log_contents, true, window, cx);
        let editor_subscription = cx.subscribe(
            &editor,
            |_, _, event: &EditorEvent, cx: &mut Context<LspLogView>| cx.emit(event.clone()),
        );
        let search_subscription = cx.subscribe(
            &editor,
            |_, _, event: &SearchEvent, cx: &mut Context<LspLogView>| cx.emit(event.clone()),
        );
        (editor, vec![editor_subscription, search_subscription])
    }

    fn editor_for_server_info(
        info: ServerInfo,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> (Entity<Editor>, Vec<Subscription>) {
        let server_info = format!(
            "* Server: {NAME} (id {ID})

* Version: {VERSION}

* Binary: {BINARY}

* Registered workspace folders:
{WORKSPACE_FOLDERS}

* Capabilities: {CAPABILITIES}

* Configuration: {CONFIGURATION}",
            NAME = info.status.name,
            ID = info.id,
            VERSION = info
                .status
                .server_version
                .as_ref()
                .map(|version| version.as_ref())
                .unwrap_or("Unknown"),
            BINARY = info
                .status
                .binary
                .as_ref()
                .map_or_else(|| "Unknown".to_string(), |binary| format!("{:#?}", binary)),
            WORKSPACE_FOLDERS = info
                .status
                .workspace_folders
                .iter()
                .filter_map(|uri| uri.to_file_path().ok())
                .map(|path| path.to_string_lossy().into_owned())
                .join(", "),
            CAPABILITIES = serde_json::to_string_pretty(&info.capabilities)
                .unwrap_or_else(|e| format!("Failed to serialize capabilities: {e}")),
            CONFIGURATION = info
                .status
                .configuration
                .map(|configuration| serde_json::to_string_pretty(&configuration))
                .transpose()
                .unwrap_or_else(|e| Some(format!("Failed to serialize configuration: {e}")))
                .unwrap_or_else(|| "Unknown".to_string()),
        );
        let editor = initialize_new_editor(server_info, false, window, cx);
        let editor_subscription = cx.subscribe(
            &editor,
            |_, _, event: &EditorEvent, cx: &mut Context<LspLogView>| cx.emit(event.clone()),
        );
        let search_subscription = cx.subscribe(
            &editor,
            |_, _, event: &SearchEvent, cx: &mut Context<LspLogView>| cx.emit(event.clone()),
        );
        (editor, vec![editor_subscription, search_subscription])
    }
    pub(crate) fn try_ensure_copilot_for_project(&self, cx: &mut App) {
        self.log_store.update(cx, |this, cx| {
            let copilot = EditPredictionStore::try_global(cx)
                .and_then(|store| store.read(cx).copilot_for_project(&self.project))?;
            let server = copilot.read(cx).language_server()?.clone();
            let log_subscription = this.copilot_state_for_project(&self.project.downgrade());
            if let Some(subscription_slot @ None) = log_subscription {
                let weak_lsp_store = cx.weak_entity();
                let server_id = server.server_id();
                let server_kind = LanguageServerKind::Supplementary {
                    project: self.project.downgrade(),
                };
                let server_key = LanguageServerLogKey::new(server_kind.clone(), server_id);

                let name = LanguageServerName::new_static("copilot");
                *subscription_slot =
                    Some(server.on_notification::<lsp::notification::LogMessage, _>(
                        move |params, cx| {
                            weak_lsp_store
                                .update(cx, |lsp_store, cx| {
                                    lsp_store.add_language_server_log(
                                        &server_key,
                                        MessageType::LOG,
                                        &params.message,
                                        cx,
                                    );
                                })
                                .ok();
                        },
                    ));
                this.add_language_server(
                    server_kind,
                    server_id,
                    Some(name),
                    None,
                    Some(server),
                    cx,
                );
            }

            Some(())
        });
    }
    pub(crate) fn menu_items(&self, cx: &mut App) -> Option<Vec<LogMenuItem>> {
        self.try_ensure_copilot_for_project(cx);
        let log_store = self.log_store.read(cx);

        let unknown_server = LanguageServerName::new_static("unknown server");
        let project = self.project.downgrade();
        let lsp_store = self.project.read(cx).lsp_store().downgrade();

        let mut rows = log_store
            .language_servers
            .iter()
            .filter(|(key, _)| key.is_for_project(&project, &lsp_store))
            .map(|(key, state)| match &key.kind {
                LanguageServerKind::Local { .. }
                | LanguageServerKind::Remote { .. }
                | LanguageServerKind::LocalSsh { .. } => {
                    let worktree_root_name = state
                        .worktree_id
                        .and_then(|id| self.project.read(cx).worktree_for_id(id, cx))
                        .map(|worktree| worktree.read(cx).root_name_str().to_string())
                        .unwrap_or_else(|| "Unknown worktree".to_string());

                    LogMenuItem {
                        server_id: key.server_id,
                        server_name: state.name.clone().unwrap_or(unknown_server.clone()),
                        server_kind: key.kind.clone(),
                        worktree_root_name,
                        rpc_trace_enabled: state.rpc_state.is_some(),
                        selected_entry: self.active_entry_kind,
                        trace_level: lsp::TraceValue::Off,
                    }
                }

                LanguageServerKind::Supplementary { .. } => LogMenuItem {
                    server_id: key.server_id,
                    server_name: state.name.clone().unwrap_or(unknown_server.clone()),
                    server_kind: key.kind.clone(),
                    worktree_root_name: "supplementary".to_string(),
                    rpc_trace_enabled: state.rpc_state.is_some(),
                    selected_entry: self.active_entry_kind,
                    trace_level: lsp::TraceValue::Off,
                },
            })
            .collect::<Vec<_>>();
        rows.sort_by_key(|row| row.server_id);
        Some(rows)
    }

    fn set_stream_enabled(
        &mut self,
        key: &LanguageServerLogKey,
        log_kind: LogKind,
        enabled: bool,
        cx: &mut App,
    ) {
        if enabled {
            let already_enabled = self
                .enabled_streams
                .get(key)
                .is_some_and(|log_kinds| log_kinds.contains(&log_kind));
            if already_enabled {
                return;
            }
            let Some(is_first) = self.log_store.update(cx, |log_store, _| {
                log_store.retain_view_log_stream(key, log_kind)
            }) else {
                return;
            };
            self.enabled_streams
                .entry(key.clone())
                .or_default()
                .insert(log_kind);
            if is_first && let Some(log_type) = log_type(log_kind) {
                send_toggle_log_message(key, true, log_type, cx);
            }
        } else {
            let Some(log_kinds) = self.enabled_streams.get_mut(key) else {
                return;
            };
            if !log_kinds.remove(&log_kind) {
                return;
            }
            if log_kinds.is_empty() {
                self.enabled_streams.remove(key);
            }
            let is_last = self
                .log_store
                .update(cx, |log_store, _| {
                    log_store.release_view_log_stream(key, log_kind)
                })
                .unwrap_or(true);
            if is_last && let Some(log_type) = log_type(log_kind) {
                send_toggle_log_message(key, false, log_type, cx);
            }
        }
    }

    fn disable_streams_for_server(&mut self, key: &LanguageServerLogKey, cx: &mut App) {
        let log_kinds = self.enabled_streams.get(key).cloned().unwrap_or_default();
        for log_kind in log_kinds {
            self.set_stream_enabled(key, log_kind, false, cx);
        }
    }

    fn show_logs_for_server(
        &mut self,
        key: LanguageServerLogKey,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let typ = self
            .log_store
            .read(cx)
            .language_servers
            .get(&key)
            .map(|v| v.log_level)
            .unwrap_or(MessageType::LOG);
        let log_contents = self
            .log_store
            .read(cx)
            .server_logs(&key)
            .map(|v| log_contents(v, typ));
        if let Some(log_contents) = log_contents {
            self.current_server_key = Some(key.clone());
            self.active_entry_kind = LogKind::Logs;
            let (editor, editor_subscriptions) = Self::editor_for_logs(log_contents, window, cx);
            self.editor = editor;
            self.editor_subscriptions = editor_subscriptions;
            cx.notify();
        }
        self.editor.read(cx).focus_handle(cx).focus(window, cx);
        self.set_stream_enabled(&key, LogKind::Logs, true, cx);
    }

    fn update_log_level(
        &self,
        key: LanguageServerLogKey,
        level: MessageType,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let log_contents = self.log_store.update(cx, |this, _| {
            if let Some(state) = this.get_language_server_state(&key) {
                state.log_level = level;
            }

            this.server_logs(&key).map(|v| log_contents(v, level))
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

    fn show_trace_for_server(
        &mut self,
        key: LanguageServerLogKey,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let trace_level = self
            .log_store
            .update(cx, |log_store, _| {
                Some(log_store.get_language_server_state(&key)?.trace_level)
            })
            .unwrap_or(TraceValue::Messages);
        let log_contents = self
            .log_store
            .read(cx)
            .server_trace(&key)
            .map(|v| log_contents(v, trace_level));
        if let Some(log_contents) = log_contents {
            self.current_server_key = Some(key.clone());
            self.active_entry_kind = LogKind::Trace;
            let (editor, editor_subscriptions) = Self::editor_for_logs(log_contents, window, cx);
            self.editor = editor;
            self.editor_subscriptions = editor_subscriptions;
            self.set_stream_enabled(&key, LogKind::Trace, true, cx);
            cx.notify();
        }
        self.editor.read(cx).focus_handle(cx).focus(window, cx);
    }

    fn show_rpc_trace_for_server(
        &mut self,
        key: LanguageServerLogKey,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.toggle_rpc_trace_for_server(key.clone(), true, window, cx);
        let rpc_log = self.log_store.update(cx, |log_store, _| {
            log_store
                .get_language_server_state(&key)?
                .rpc_state
                .as_ref()
                .map(|state| log_contents(&state.rpc_messages, ()))
        });
        if let Some(rpc_log) = rpc_log {
            self.current_server_key = Some(key);
            self.active_entry_kind = LogKind::Rpc;
            let (editor, editor_subscriptions) = Self::editor_for_logs(rpc_log, window, cx);
            let language = self.project.read(cx).languages().language_for_name("JSON");
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
                            let language = language.await.ok();
                            buffer.update(cx, |buffer, cx| {
                                buffer.set_language(language, cx);
                            });
                        }
                    })
                    .detach();
                });

            self.editor = editor;
            self.editor_subscriptions = editor_subscriptions;
            cx.notify();
        }

        self.editor.read(cx).focus_handle(cx).focus(window, cx);
    }

    fn toggle_rpc_trace_for_server(
        &mut self,
        key: LanguageServerLogKey,
        enabled: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.set_stream_enabled(&key, LogKind::Rpc, enabled, cx);
        if !enabled && self.current_server_key.as_ref() == Some(&key) {
            self.show_logs_for_server(key, window, cx);
            cx.notify();
        }
    }

    fn update_trace_level(
        &self,
        key: LanguageServerLogKey,
        level: TraceValue,
        cx: &mut Context<Self>,
    ) {
        let server = self
            .log_store
            .read(cx)
            .language_servers
            .get(&key)
            .and_then(|state| state.server.clone())
            .or_else(|| {
                self.project
                    .read(cx)
                    .lsp_store()
                    .read(cx)
                    .language_server_for_id(key.server_id)
            });
        if let Some(server) = server {
            self.log_store.update(cx, |this, _| {
                if let Some(state) = this.get_language_server_state(&key) {
                    state.trace_level = level;
                }
            });

            server
                .notify::<SetTrace>(SetTraceParams { value: level })
                .ok();
        }
    }

    fn show_server_info(
        &mut self,
        key: LanguageServerLogKey,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let server_id = key.server_id;
        let server_info = self
            .log_store
            .read(cx)
            .language_servers
            .get(&key)
            .and_then(|state| state.server.as_deref().map(ServerInfo::new));
        let server_info = server_info.or_else(|| {
            self.project
                .read(cx)
                .lsp_store()
                .update(cx, |lsp_store, _| {
                    lsp_store
                        .language_server_for_id(server_id)
                        .as_ref()
                        .map(|language_server| ServerInfo::new(language_server))
                        .or_else(move || {
                            let capabilities =
                                lsp_store.lsp_server_capabilities.get(&server_id)?.clone();
                            let status =
                                lsp_store.language_server_statuses.get(&server_id)?.clone();

                            Some(ServerInfo {
                                id: server_id,
                                capabilities,
                                status,
                            })
                        })
                })
        });
        let Some(server_info) = server_info else {
            return;
        };
        self.current_server_key = Some(key.clone());
        self.active_entry_kind = LogKind::ServerInfo;
        let (editor, editor_subscriptions) = Self::editor_for_server_info(server_info, window, cx);
        self.editor = editor;
        self.editor_subscriptions = editor_subscriptions;
        cx.notify();
        self.editor.read(cx).focus_handle(cx).focus(window, cx);
        self.disable_streams_for_server(&key, cx);
    }
}

fn log_type(log_kind: LogKind) -> Option<LogType> {
    match log_kind {
        LogKind::Rpc => Some(LogType::Rpc),
        LogKind::Trace => Some(LogType::Trace),
        LogKind::Logs => Some(LogType::Log),
        LogKind::ServerInfo => None,
    }
}

fn send_toggle_log_message(
    key: &LanguageServerLogKey,
    enabled: bool,
    log_type: LogType,
    cx: &mut App,
) {
    if let LanguageServerKind::Remote { project } = &key.kind {
        project
            .update(cx, |project, cx| {
                if let Some((client, project_id)) = project.lsp_store().read(cx).upstream_client() {
                    client
                        .send(proto::ToggleLspLogs {
                            project_id,
                            log_type: log_type as i32,
                            server_id: key.server_id.to_proto(),
                            enabled,
                        })
                        .log_err();
                }
            })
            .ok();
    }
}

fn log_contents<T: Message>(lines: &VecDeque<T>, level: <T as Message>::Level) -> String {
    lines
        .iter()
        .filter(|message| message.should_include(level))
        .flat_map(|message| [message.as_ref(), "\n"])
        .collect()
}

impl Render for LspLogView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.editor.update(cx, |editor, cx| {
            editor.render(window, cx).into_any_element()
        })
    }
}

impl Focusable for LspLogView {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for LspLogView {
    type Event = EditorEvent;

    fn to_item_events(event: &Self::Event, f: &mut dyn FnMut(workspace::item::ItemEvent)) {
        Editor::to_item_events(event, f)
    }

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        "LSP Logs".into()
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
            if let Some(server_key) = self.current_server_key.clone() {
                match self.active_entry_kind {
                    LogKind::Rpc => new_view.show_rpc_trace_for_server(server_key, window, cx),
                    LogKind::Trace => new_view.show_trace_for_server(server_key, window, cx),
                    LogKind::Logs => new_view.show_logs_for_server(server_key, window, cx),
                    LogKind::ServerInfo => new_view.show_server_info(server_key, window, cx),
                }
            }
            new_view
        })))
    }
}

impl SearchableItem for LspLogView {
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

    fn query_suggestion(
        &mut self,
        seed_query_override: Option<SeedQuerySetting>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> String {
        self.editor.update(cx, |e, cx| {
            e.query_suggestion(seed_query_override, window, cx)
        })
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
        query: Arc<project::search::SearchQuery>,
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
        // Since LSP Log is read-only, it doesn't make sense to support replace operation.
    }
    fn supported_options(&self) -> workspace::searchable::SearchOptions {
        workspace::searchable::SearchOptions {
            case: true,
            word: true,
            regex: true,
            find_in_results: false,
            // LSP log is read-only.
            replacement: false,
            selection: false,
            select_all: true,
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

impl EventEmitter<ToolbarItemEvent> for LspLogToolbarItemView {}

impl ToolbarItemView for LspLogToolbarItemView {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> workspace::ToolbarItemLocation {
        if let Some(item) = active_pane_item
            && let Some(log_view) = item.downcast::<LspLogView>()
        {
            self.log_view = Some(log_view.clone());
            self._log_view_subscription = Some(cx.observe(&log_view, |_, _, cx| {
                cx.notify();
            }));
            return ToolbarItemLocation::PrimaryLeft;
        }
        self.log_view = None;
        self._log_view_subscription = None;
        ToolbarItemLocation::Hidden
    }
}

impl Render for LspLogToolbarItemView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(log_view) = self.log_view.clone() else {
            return div();
        };

        let (menu_rows, current_server_key) = log_view.update(cx, |log_view, cx| {
            let menu_rows = log_view.menu_items(cx).unwrap_or_default();
            let current_server_key = log_view.current_server_key.clone();
            (menu_rows, current_server_key)
        });

        let current_server = current_server_key.and_then(|current_server_key| {
            menu_rows
                .iter()
                .find(|row| row.key() == current_server_key)
                .cloned()
        });

        let available_language_servers: Vec<_> = menu_rows
            .into_iter()
            .map(|row| {
                (
                    row.key(),
                    row.server_name,
                    row.worktree_root_name,
                    row.selected_entry,
                )
            })
            .collect();

        let log_toolbar_view = cx.weak_entity();

        let lsp_menu = PopoverMenu::new("LspLogView")
            .anchor(Anchor::TopLeft)
            .trigger(
                Button::new(
                    "language_server_menu_header",
                    current_server
                        .as_ref()
                        .map(|row| {
                            Cow::Owned(format!(
                                "{} ({})",
                                row.server_name.0, row.worktree_root_name,
                            ))
                        })
                        .unwrap_or_else(|| "No server selected".into()),
                )
                .end_icon(
                    Icon::new(IconName::ChevronDown)
                        .size(IconSize::Small)
                        .color(Color::Muted),
                ),
            )
            .menu({
                let log_view = log_view.clone();
                move |window, cx| {
                    let log_view = log_view.clone();
                    ContextMenu::build(window, cx, |mut menu, window, _| {
                        for (server_key, name, worktree_root, active_entry_kind) in
                            available_language_servers.iter()
                        {
                            let label = format!("{name} ({worktree_root})");
                            let server_key = server_key.clone();
                            let active_entry_kind = *active_entry_kind;
                            menu = menu.entry(
                                label,
                                None,
                                window.handler_for(&log_view, move |view, window, cx| {
                                    view.current_server_key = Some(server_key.clone());
                                    view.active_entry_kind = active_entry_kind;
                                    match view.active_entry_kind {
                                        LogKind::Rpc => {
                                            view.toggle_rpc_trace_for_server(
                                                server_key.clone(),
                                                true,
                                                window,
                                                cx,
                                            );
                                            view.show_rpc_trace_for_server(
                                                server_key.clone(),
                                                window,
                                                cx,
                                            );
                                        }
                                        LogKind::Trace => view.show_trace_for_server(
                                            server_key.clone(),
                                            window,
                                            cx,
                                        ),
                                        LogKind::Logs => view.show_logs_for_server(
                                            server_key.clone(),
                                            window,
                                            cx,
                                        ),
                                        LogKind::ServerInfo => {
                                            view.show_server_info(server_key.clone(), window, cx)
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
            let server_key = server.key();
            let rpc_trace_enabled = server.rpc_trace_enabled;
            let log_view = log_view.clone();
            let label = match server.selected_entry {
                LogKind::Rpc => RPC_MESSAGES,
                LogKind::Trace => SERVER_TRACE,
                LogKind::Logs => SERVER_LOGS,
                LogKind::ServerInfo => SERVER_INFO,
            };
            PopoverMenu::new("LspViewSelector")
                .anchor(Anchor::TopLeft)
                .trigger(
                    Button::new("language_server_menu_header", label).end_icon(
                        Icon::new(IconName::ChevronDown)
                            .size(IconSize::Small)
                            .color(Color::Muted),
                    ),
                )
                .menu(move |window, cx| {
                    let log_toolbar_view = log_toolbar_view.upgrade()?;
                    let log_view = log_view.clone();
                    let server_key = server_key.clone();
                    Some(ContextMenu::build(window, cx, move |this, window, _| {
                        this.entry(
                            SERVER_LOGS,
                            None,
                            window.handler_for(&log_view, {
                                let server_key = server_key.clone();
                                move |view, window, cx| {
                                    view.show_logs_for_server(server_key.clone(), window, cx);
                                }
                            }),
                        )
                        .entry(
                            SERVER_TRACE,
                            None,
                            window.handler_for(&log_view, {
                                let server_key = server_key.clone();
                                move |view, window, cx| {
                                    view.show_trace_for_server(server_key.clone(), window, cx);
                                }
                            }),
                        )
                        .custom_entry(
                            {
                                let log_toolbar_view = log_toolbar_view.clone();
                                let server_key = server_key.clone();
                                move |window, _| {
                                    let server_key = server_key.clone();
                                    h_flex()
                                        .w_full()
                                        .justify_between()
                                        .child(Label::new(RPC_MESSAGES))
                                        .child(
                                            div().child(
                                                Checkbox::new(
                                                    "LspLogEnableRpcTrace",
                                                    if rpc_trace_enabled {
                                                        ToggleState::Selected
                                                    } else {
                                                        ToggleState::Unselected
                                                    },
                                                )
                                                .on_click(window.listener_for(
                                                    &log_toolbar_view,
                                                    move |view, selection, window, cx| {
                                                        let enabled = matches!(
                                                            selection,
                                                            ToggleState::Selected
                                                        );
                                                        view.toggle_rpc_logging_for_server(
                                                            server_key.clone(),
                                                            enabled,
                                                            window,
                                                            cx,
                                                        );
                                                        cx.stop_propagation();
                                                    },
                                                )),
                                            ),
                                        )
                                        .into_any_element()
                                }
                            },
                            window.handler_for(&log_view, {
                                let server_key = server_key.clone();
                                move |view, window, cx| {
                                    view.show_rpc_trace_for_server(server_key.clone(), window, cx);
                                }
                            }),
                        )
                        .entry(
                            SERVER_INFO,
                            None,
                            window.handler_for(&log_view, {
                                let server_key = server_key.clone();
                                move |view, window, cx| {
                                    view.show_server_info(server_key.clone(), window, cx);
                                }
                            }),
                        )
                    }))
                })
        });

        h_flex()
            .size_full()
            .gap_1()
            .justify_between()
            .child(
                h_flex()
                    .gap_0p5()
                    .child(lsp_menu)
                    .children(view_selector)
                    .child(
                        log_view.update(cx, |this, _cx| match this.active_entry_kind {
                            LogKind::Trace => {
                                let log_view = log_view.clone();
                                div().child(
                                    PopoverMenu::new("lsp-trace-level-menu")
                                        .anchor(Anchor::TopLeft)
                                        .trigger(
                                            Button::new(
                                                "language_server_trace_level_selector",
                                                "Trace level",
                                            )
                                            .end_icon(
                                                Icon::new(IconName::ChevronDown)
                                                    .size(IconSize::Small)
                                                    .color(Color::Muted),
                                            ),
                                        )
                                        .menu({
                                            let log_view = log_view;

                                            move |window, cx| {
                                                let key =
                                                    log_view.read(cx).current_server_key.clone()?;

                                                let trace_level =
                                                    log_view.update(cx, |this, cx| {
                                                        this.log_store.update(cx, |this, _| {
                                                            Some(
                                                                this.get_language_server_state(
                                                                    &key,
                                                                )?
                                                                .trace_level,
                                                            )
                                                        })
                                                    })?;

                                                ContextMenu::build(
                                                    window,
                                                    cx,
                                                    |mut menu, window, cx| {
                                                        let log_view = log_view.clone();

                                                        for (option, label) in [
                                                            (TraceValue::Off, "Off"),
                                                            (TraceValue::Messages, "Messages"),
                                                            (TraceValue::Verbose, "Verbose"),
                                                        ] {
                                                            menu = menu.entry(label, None, {
                                                                let log_view = log_view.clone();
                                                                move |_, cx| {
                                                                    log_view.update(cx, |this, cx| {
                                                                    if let Some(key) = this
                                                                        .current_server_key
                                                                        .clone()
                                                                    {
                                                                        this.update_trace_level(
                                                                            key, option, cx,
                                                                        );
                                                                    }
                                                                });
                                                                }
                                                            });
                                                            if option == trace_level {
                                                                menu.select_last(window, cx);
                                                            }
                                                        }

                                                        menu
                                                    },
                                                )
                                                .into()
                                            }
                                        }),
                                )
                            }
                            LogKind::Logs => {
                                let log_view = log_view.clone();
                                div().child(
                                    PopoverMenu::new("lsp-log-level-menu")
                                        .anchor(Anchor::TopLeft)
                                        .trigger(
                                            Button::new(
                                                "language_server_log_level_selector",
                                                "Log level",
                                            )
                                            .end_icon(
                                                Icon::new(IconName::ChevronDown)
                                                    .size(IconSize::Small)
                                                    .color(Color::Muted),
                                            ),
                                        )
                                        .menu({
                                            let log_view = log_view;

                                            move |window, cx| {
                                                let key =
                                                    log_view.read(cx).current_server_key.clone()?;

                                                let log_level =
                                                    log_view.update(cx, |this, cx| {
                                                        this.log_store.update(cx, |this, _| {
                                                            Some(
                                                                this.get_language_server_state(
                                                                    &key,
                                                                )?
                                                                .log_level,
                                                            )
                                                        })
                                                    })?;

                                                ContextMenu::build(
                                                    window,
                                                    cx,
                                                    |mut menu, window, cx| {
                                                        let log_view = log_view.clone();

                                                        for (option, label) in [
                                                            (MessageType::LOG, "Log"),
                                                            (MessageType::INFO, "Info"),
                                                            (MessageType::WARNING, "Warning"),
                                                            (MessageType::ERROR, "Error"),
                                                        ] {
                                                            menu = menu.entry(label, None, {
                                                                let log_view = log_view.clone();
                                                                move |window, cx| {
                                                                    log_view.update(cx, |this, cx| {
                                                                    if let Some(key) = this
                                                                        .current_server_key
                                                                        .clone()
                                                                    {
                                                                        this.update_log_level(
                                                                            key, option, window, cx,
                                                                        );
                                                                    }
                                                                });
                                                                }
                                                            });
                                                            if option == log_level {
                                                                menu.select_last(window, cx);
                                                            }
                                                        }

                                                        menu
                                                    },
                                                )
                                                .into()
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
        editor.set_show_bookmarks(false, cx);
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

const RPC_MESSAGES: &str = "RPC Messages";
const SERVER_LOGS: &str = "Server Logs";
const SERVER_TRACE: &str = "Server Trace";
const SERVER_INFO: &str = "Server Info";

impl LspLogToolbarItemView {
    pub fn new() -> Self {
        Self {
            log_view: None,
            _log_view_subscription: None,
        }
    }

    fn toggle_rpc_logging_for_server(
        &mut self,
        key: LanguageServerLogKey,
        enabled: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(log_view) = &self.log_view {
            log_view.update(cx, |log_view, cx| {
                log_view.toggle_rpc_trace_for_server(key.clone(), enabled, window, cx);
                if !enabled && log_view.current_server_key.as_ref() == Some(&key) {
                    log_view.show_logs_for_server(key.clone(), window, cx);
                    cx.notify();
                } else if enabled {
                    log_view.show_rpc_trace_for_server(key.clone(), window, cx);
                    cx.notify();
                }
                window.focus(&log_view.focus_handle, cx);
            });
        }
        cx.notify();
    }
}

struct ServerInfo {
    id: LanguageServerId,
    capabilities: lsp::ServerCapabilities,
    status: LanguageServerStatus,
}

impl ServerInfo {
    fn new(server: &LanguageServer) -> Self {
        Self {
            id: server.server_id(),
            capabilities: server.capabilities(),
            status: LanguageServerStatus {
                name: server.name(),
                language_name: None,
                server_version: server.version(),
                server_readable_version: server.readable_version(),
                pending_work: Default::default(),
                has_pending_diagnostic_updates: false,
                progress_tokens: Default::default(),
                worktree: None,
                binary: Some(server.binary().clone()),
                configuration: Some(server.configuration().clone()),
                workspace_folders: server.workspace_folders(),
                process_id: server.process_id(),
            },
        }
    }
}

impl EventEmitter<EditorEvent> for LspLogView {}
impl EventEmitter<SearchEvent> for LspLogView {}
