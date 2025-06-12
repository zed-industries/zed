use std::{
    collections::{BTreeMap, BTreeSet},
    path::PathBuf,
};

use client::proto;
use collections::{HashMap, HashSet, hash_map};
use editor::{
    Editor, EditorEvent,
    actions::{RestartLanguageServer, StopLanguageServer},
};
use gpui::{Entity, Subscription, WeakEntity};
use itertools::Itertools as _;
use language::{BufferId, LocalFile};
use lsp::{LanguageServerId, LanguageServerName};
use project::{LspStore, LspStoreEvent, project_settings::ProjectSettings};
use settings::{Settings as _, SettingsStore};
use ui::{
    Context, ContextMenu, ContextMenuEntry, IconButtonShape, Indicator, PopoverMenu, Tooltip,
    Window, prelude::*,
};

use workspace::{StatusItemView, Workspace};

use crate::{LogStore, lsp_log::GlobalLogStore};

pub struct LspTool {
    workspace: WeakEntity<Workspace>,
    lsp_store: WeakEntity<LspStore>,
    active_editor: Option<ActiveEditor>,
    language_servers: LanguageServers,
    _subscriptions: Vec<Subscription>,
}

struct ActiveEditor {
    editor: WeakEntity<Editor>,
    _editor_subscription: Subscription,
    editor_buffers: HashSet<BufferId>,
}

#[derive(Debug, Default, Clone)]
struct LanguageServers {
    servers: HashMap<LanguageServerId, LanguageServerState>,
    servers_per_buffer_abs_path: HashMap<PathBuf, HashSet<LanguageServerId>>,
}

#[derive(Debug, Clone)]
struct LanguageServerState {
    name: LanguageServerName,
    message: Option<(SharedString, Severity)>,
    status: LanguageServerStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Severity {
    Ok,
    Info,
    Warning,
    Error,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LanguageServerStatus {
    Starting,
    Running,
    Stopping,
    Stopped,
}

impl LanguageServers {
    fn update_status(
        &mut self,
        id: LanguageServerId,
        status: LanguageServerStatus,
        message: Option<&str>,
        name: Option<LanguageServerName>,
    ) {
        match self.servers.entry(id) {
            hash_map::Entry::Occupied(mut o) => {
                let state = o.get_mut();
                if let Some(name) = name {
                    state.name = name;
                }
                state.status = status;
                state.message = message
                    .map(|message| (SharedString::from(message.to_owned()), Severity::Other));
            }
            hash_map::Entry::Vacant(v) => {
                if let Some(name) = name {
                    v.insert(LanguageServerState {
                        name,
                        message: message.map(|message| {
                            (SharedString::from(message.to_owned()), Severity::Other)
                        }),
                        status,
                    });
                }
            }
        }

        let duplicate_server_statuses =
            self.servers
                .iter()
                .fold(HashMap::default(), |mut acc, (id, state)| {
                    acc.entry(state.name.clone())
                        .or_insert_with(BTreeMap::new)
                        .insert(*id, state.status);
                    acc
                });

        for duplicate_statuses in duplicate_server_statuses.into_values() {
            if duplicate_statuses.len() < 2 {
                continue;
            }

            let mut stopped_servers = BTreeSet::new();
            let mut not_stopped = Vec::new();
            for (id, status) in duplicate_statuses {
                if status == LanguageServerStatus::Stopped {
                    stopped_servers.insert(id);
                } else {
                    not_stopped.push(id);
                }
            }

            if not_stopped.is_empty() {
                if stopped_servers.len() > 1 {
                    for id in stopped_servers.into_iter().rev().skip(1) {
                        self.remove(id);
                    }
                }
            } else {
                for id in stopped_servers {
                    self.remove(id);
                }
            }
        }
    }

    fn update_message(
        &mut self,
        id: LanguageServerId,
        message: Option<&str>,
        severity: Severity,
        name: Option<LanguageServerName>,
    ) {
        if let Some(state) = self.servers.get_mut(&id) {
            state.message =
                message.map(|message| (SharedString::from(message.to_owned()), severity));
            if let Some(name) = name {
                state.name = name;
            }
        } else if let Some((message, name)) = message.zip(name) {
            self.servers.insert(
                id,
                LanguageServerState {
                    message: Some((SharedString::from(message.to_owned()), severity)),
                    name,
                    status: LanguageServerStatus::Running,
                },
            );
        }
    }

    fn remove(&mut self, id: LanguageServerId) {
        self.servers.remove(&id);
        self.servers_per_buffer_abs_path.retain(|_, servers| {
            servers.remove(&id);
            !servers.is_empty()
        });
    }
}

impl LspTool {
    pub fn new(workspace: &Workspace, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let settings_subscription =
            cx.observe_global_in::<SettingsStore>(window, move |_lsp_tool, _, cx| {
                cx.notify();
            });

        let lsp_store = workspace.project().read(cx).lsp_store();
        let lsp_store_subscription =
            cx.subscribe_in(&lsp_store, window, |lsp_tool, _, e, window, cx| {
                lsp_tool.on_lsp_store_(e, window, cx)
            });

        Self {
            workspace: workspace.weak_handle(),
            lsp_store: lsp_store.downgrade(),
            active_editor: None,
            _subscriptions: vec![settings_subscription, lsp_store_subscription],
            language_servers: LanguageServers::default(),
        }
    }

    fn on_lsp_store_(&mut self, e: &LspStoreEvent, _: &mut Window, cx: &mut Context<Self>) {
        match e {
            project::LspStoreEvent::LanguageServerUpdate {
                language_server_id,
                name,
                message: proto::update_language_server::Variant::StatusUpdate(status_update),
            } => match proto::status_update::Status::from_i32(status_update.status) {
                Some(proto::status_update::Status::Starting) => {
                    self.language_servers.update_status(
                        *language_server_id,
                        LanguageServerStatus::Starting,
                        status_update.message.as_deref(),
                        name.clone(),
                    );
                    cx.notify();
                }
                Some(proto::status_update::Status::Running) => {
                    self.language_servers.update_status(
                        *language_server_id,
                        LanguageServerStatus::Running,
                        status_update.message.as_deref(),
                        name.clone(),
                    );
                    cx.notify();
                }
                Some(proto::status_update::Status::Stopping) => {
                    self.language_servers.update_status(
                        *language_server_id,
                        LanguageServerStatus::Stopping,
                        status_update.message.as_deref(),
                        name.clone(),
                    );
                    cx.notify();
                }
                Some(proto::status_update::Status::Stopped) => {
                    self.language_servers.update_status(
                        *language_server_id,
                        LanguageServerStatus::Stopped,
                        status_update.message.as_deref(),
                        name.clone(),
                    );
                    cx.notify();
                }

                Some(proto::status_update::Status::Ok) => {
                    self.language_servers.update_message(
                        *language_server_id,
                        status_update.message.as_deref(),
                        Severity::Ok,
                        name.clone(),
                    );
                    cx.notify();
                }
                Some(proto::status_update::Status::Info) => {
                    self.language_servers.update_message(
                        *language_server_id,
                        status_update.message.as_deref(),
                        Severity::Info,
                        name.clone(),
                    );
                    cx.notify();
                }
                Some(proto::status_update::Status::Warning) => {
                    self.language_servers.update_message(
                        *language_server_id,
                        status_update.message.as_deref(),
                        Severity::Warning,
                        name.clone(),
                    );
                    cx.notify();
                }
                Some(proto::status_update::Status::Error) => {
                    self.language_servers.update_message(
                        *language_server_id,
                        status_update.message.as_deref(),
                        Severity::Error,
                        name.clone(),
                    );
                    cx.notify();
                }
                Some(proto::status_update::Status::Other) => {
                    self.language_servers.update_message(
                        *language_server_id,
                        status_update.message.as_deref(),
                        Severity::Other,
                        name.clone(),
                    );
                    cx.notify();
                }
                None => {
                    log::error!("Unexpected status update {}", status_update.status);
                }
            },
            project::LspStoreEvent::LanguageServerUpdate {
                language_server_id,
                message: proto::update_language_server::Variant::RegisteredForBuffer(update),
                ..
            } => {
                self.language_servers
                    .servers_per_buffer_abs_path
                    .entry(PathBuf::from(&update.buffer_abs_path))
                    .or_default()
                    .insert(*language_server_id);
                cx.notify();
            }
            _ => {}
        };
    }

    fn build_context_menu(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<ContextMenu> {
        let active_editor = self.active_editor.as_ref().map(|ae| ae.editor.clone());
        let editor_buffers = self
            .active_editor
            .as_ref()
            .map(|active_editor| active_editor.editor_buffers.clone())
            .unwrap_or_default();

        let mut buffer_servers = Vec::with_capacity(self.language_servers.servers.len());
        let mut other_servers = Vec::with_capacity(self.language_servers.servers.len());
        let buffer_server_ids = editor_buffers
            .iter()
            .filter_map(|buffer_id| {
                let buffer_path = self
                    .lsp_store
                    .update(cx, |lsp_store, cx| {
                        Some(
                            project::File::from_dyn(
                                lsp_store
                                    .buffer_store()
                                    .read(cx)
                                    .get(*buffer_id)?
                                    .read(cx)
                                    .file(),
                            )?
                            .abs_path(cx),
                        )
                    })
                    .ok()??;
                self.language_servers
                    .servers_per_buffer_abs_path
                    .get(&buffer_path)
            })
            .flatten()
            .unique()
            .copied()
            .collect::<HashSet<_>>();
        for (server_id, server_state) in &self.language_servers.servers {
            if buffer_server_ids.contains(server_id) {
                buffer_servers.push((*server_id, server_state));
            } else {
                other_servers.push((*server_id, server_state));
            }
        }
        buffer_servers.sort_by_key(|(_, state)| state.name.clone());
        other_servers.sort_by_key(|(_, state)| state.name.clone());

        let workspace = self.workspace.clone();
        let lsp_store = self.lsp_store.clone();
        let lsp_logs = cx.global::<GlobalLogStore>().0.downgrade();
        ContextMenu::build(window, cx, move |mut menu, _, _| {
            if active_editor.is_none() {
                return empty_context_menu(
                    menu,
                    "No active editor - open a file to manage language servers",
                );
            } else if buffer_servers.is_empty() && other_servers.is_empty() {
                return empty_context_menu(menu, "No language servers are currently running");
            }

            if !buffer_servers.is_empty() {
                menu = fill_servers(
                    menu.header("Current Buffer"),
                    &workspace,
                    &lsp_store,
                    &editor_buffers,
                    buffer_servers,
                    &lsp_logs,
                );
            }
            if !other_servers.is_empty() {
                menu = fill_servers(
                    menu.header("Other Active Servers"),
                    &workspace,
                    &lsp_store,
                    &editor_buffers,
                    other_servers,
                    &lsp_logs,
                );
            }
            if let Some(active_editor) = &active_editor {
                menu = menu
                    .entry(
                        "Restart All Servers",
                        Some(Box::new(RestartLanguageServer)),
                        {
                            let active_editor = active_editor.clone();
                            move |window, cx| {
                                active_editor
                                    .update(cx, |editor, cx| {
                                        editor.restart_language_server(
                                            &RestartLanguageServer,
                                            window,
                                            cx,
                                        );
                                    })
                                    .ok();
                            }
                        },
                    )
                    .entry("Stop All Servers", Some(Box::new(StopLanguageServer)), {
                        let active_editor = active_editor.clone();
                        move |window, cx| {
                            active_editor
                                .update(cx, |editor, cx| {
                                    editor.stop_language_server(&StopLanguageServer, window, cx);
                                })
                                .ok();
                        }
                    });
            }

            menu.separator()
        })
    }
}

fn fill_servers(
    mut menu: ContextMenu,
    workspace: &WeakEntity<Workspace>,
    lsp_store: &WeakEntity<LspStore>,
    editor_buffers: &HashSet<BufferId>,
    servers: Vec<(LanguageServerId, &LanguageServerState)>,
    lsp_logs: &WeakEntity<LogStore>,
) -> ContextMenu {
    for (server_id, server) in servers {
        let status_color = match server.status {
            LanguageServerStatus::Running => Color::Success,
            LanguageServerStatus::Starting => Color::Modified,
            LanguageServerStatus::Stopping => Color::Warning,
            LanguageServerStatus::Stopped => Color::Error,
        };

        menu = menu.custom_entry(
            {
                let server_name = server.name.0.clone();
                let lsp_store = lsp_store.clone();
                let workspace = workspace.clone();
                let editor_buffers = editor_buffers.clone();
                let lsp_logs = lsp_logs.clone();
                let server_status = server.status;
                move |_, _| {
                    let can_stop = matches!(
                        server_status,
                        LanguageServerStatus::Starting | LanguageServerStatus::Running
                    );
                    let can_restart = matches!(
                        server_status,
                        LanguageServerStatus::Stopping | LanguageServerStatus::Stopped
                    );

                    h_flex()
                        .w_full()
                        .justify_between()
                        .gap_2()
                        .child(
                            h_flex()
                                .gap_2()
                                .child(Indicator::dot().color(status_color))
                                .child(Label::new(server_name.clone()))
                        )
                        .child(
                            h_flex()
                                .gap_1()
                                .when(can_restart, |div| {
                                    div.child(
                                        IconButton::new("restart-server", IconName::Rerun)
                                            .icon_size(IconSize::XSmall)
                                            .tooltip(|_, cx| Tooltip::simple("Restart server", cx))
                                            .on_click({
                                                let lsp_store = lsp_store.clone();
                                                let workspace = workspace.clone();
                                                let editor_buffers = editor_buffers.clone();
                                                move |_, _, cx| {
                                                    if let Some(workspace) = workspace.upgrade() {
                                                        let buffer_store = workspace.read(cx).project().read(cx).buffer_store().clone();
                                                        let buffers = editor_buffers
                                                            .iter()
                                                            .flat_map(|buffer_id| buffer_store.read(cx).get(*buffer_id))
                                                            .collect::<Vec<_>>();
                                                        if !buffers.is_empty() {
                                                            lsp_store.update(cx, |lsp_store, cx| {
                                                                lsp_store.restart_language_servers_for_buffers(
                                                                    buffers,
                                                                    vec![server_id],
                                                                    cx,
                                                                );
                                                            }).ok();
                                                        }
                                                    }
                                                }
                                            })
                                    )
                                })
                                .when(can_stop, |div| {
                                    div.child(
                                        IconButton::new("stop-server", IconName::Stop)
                                            .icon_size(IconSize::XSmall)
                                            .tooltip(|_, cx| Tooltip::simple("Stop server", cx))
                                            .on_click({
                                                let lsp_store = lsp_store.clone();
                                                move |_, _, cx| {
                                                    lsp_store.update(cx, |lsp_store, cx| {
                                                        lsp_store.stop_language_servers_for_buffers(
                                                            Vec::new(),
                                                            vec![server_id],
                                                            cx,
                                                        );
                                                    }).ok();
                                                }
                                            })
                                    )
                                })
                                .child(
                                    IconButton::new("open-logs", IconName::FileText)
                                        .icon_size(IconSize::XSmall)
                                        .tooltip(|_, cx| Tooltip::simple("Open logs", cx))
                                        .on_click({
                                            let workspace = workspace.clone();
                                            let lsp_logs = lsp_logs.clone();
                                            move |_, window, cx| {
                                                lsp_logs.update(cx, |lsp_logs, cx| {
                                                    lsp_logs.open_server_log(
                                                        workspace.clone(),
                                                        server_id,
                                                        window,
                                                        cx,
                                                    );
                                                }).ok();
                                            }
                                        })
                                )
                                .child(
                                    IconButton::new("open-lsp-messages-current", IconName::MessageBubbles)
                                        .icon_size(IconSize::XSmall)
                                        .tooltip(|_, cx| Tooltip::simple("Open LSP messages", cx))
                                        .on_click({
                                            let workspace = workspace.clone();
                                            let lsp_logs = lsp_logs.clone();
                                            move |_, window, cx| {
                                                lsp_logs.update(cx, |lsp_logs, cx| {
                                                    lsp_logs.open_server_trace(
                                                        workspace.clone(),
                                                        server_id,
                                                        window,
                                                        cx,
                                                    );
                                                }).ok();
                                            }
                                        })
                                )
                        )
                        .into_any_element()
                }
            },
            |_, _| {},
        );
    }
    menu
}

fn empty_context_menu(menu: ContextMenu, message: &'static str) -> ContextMenu {
    menu.item(ContextMenuEntry::new(message).disabled(true))
        .separator()
        .item(
            ContextMenuEntry::new("Restart All Servers")
                .disabled(true)
                .handler(|_, _| {}),
        )
        .item(
            ContextMenuEntry::new("Stop All Servers")
                .disabled(true)
                .handler(|_, _| {}),
        )
        .separator()
}

impl StatusItemView for LspTool {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn workspace::ItemHandle>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if ProjectSettings::get_global(cx).global_lsp_settings.button {
            if let Some(editor) = active_pane_item.and_then(|item| item.downcast::<Editor>()) {
                if Some(&editor)
                    != self
                        .active_editor
                        .as_ref()
                        .and_then(|active_editor| active_editor.editor.upgrade())
                        .as_ref()
                {
                    let editor_buffers =
                        HashSet::from_iter(editor.read(cx).buffer().read(cx).excerpt_buffer_ids());
                    let _editor_subscription =
                        cx.subscribe(&editor, |lsp_tool, _, e: &EditorEvent, cx| match e {
                            EditorEvent::ExcerptsAdded { buffer, .. } => {
                                if let Some(active_editor) = lsp_tool.active_editor.as_mut() {
                                    let buffer_id = buffer.read(cx).remote_id();
                                    if active_editor.editor_buffers.insert(buffer_id) {
                                        cx.notify();
                                    }
                                }
                            }
                            EditorEvent::ExcerptsRemoved {
                                removed_buffer_ids, ..
                            } => {
                                if let Some(active_editor) = lsp_tool.active_editor.as_mut() {
                                    let mut removed = false;
                                    for id in removed_buffer_ids {
                                        active_editor.editor_buffers.retain(|buffer_id| {
                                            let retain = buffer_id != id;
                                            removed |= !retain;
                                            retain
                                        });
                                    }
                                    if removed {
                                        cx.notify();
                                    }
                                }
                            }
                            _ => {}
                        });
                    self.active_editor = Some(ActiveEditor {
                        editor: editor.downgrade(),
                        _editor_subscription,
                        editor_buffers,
                    });
                    cx.notify();
                }
            } else if self.active_editor.is_some() {
                self.active_editor = None;
                cx.notify();
            }
        } else if self.active_editor.is_some() {
            self.active_editor = None;
            cx.notify();
        }
    }
}

impl Render for LspTool {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl ui::IntoElement {
        let mut has_errors = false;
        let mut has_warnings = false;
        let mut has_other_notifications = false;
        for server in self.language_servers.servers.values() {
            if let Some((message, severity)) = &server.message {
                match severity {
                    Severity::Error => has_errors = true,
                    Severity::Warning => has_warnings = true,
                    Severity::Info | Severity::Ok => has_other_notifications = true,
                    Severity::Other => {
                        let message_lower = message.to_lowercase();
                        if message_lower.contains("error") || message_lower.contains("failed") {
                            has_errors = true;
                        } else if message_lower.contains("warn") {
                            has_warnings = true;
                        } else {
                            has_other_notifications = true;
                        }
                    }
                }
            }
        }

        let indicator = if has_errors {
            Some(Indicator::dot().color(Color::Error))
        } else if has_warnings {
            Some(Indicator::dot().color(Color::Warning))
        } else if has_other_notifications {
            Some(Indicator::dot().color(Color::Modified))
        } else {
            None
        };

        div().child(
            PopoverMenu::new("lsp-tool-menu")
                .trigger(
                    IconButton::new("zed-lsp-tool-button", IconName::Bolt)
                        .when_some(indicator, IconButton::indicator)
                        .shape(IconButtonShape::Square)
                        .icon_size(IconSize::XSmall)
                        .indicator_border_color(Some(cx.theme().colors().status_bar_background))
                        .tooltip(move |_, cx| Tooltip::simple("Language servers", cx)),
                )
                .menu({
                    let lsp_tool = cx.weak_entity();
                    move |window, cx| {
                        lsp_tool
                            .update(cx, |lsp_tool, cx| lsp_tool.build_context_menu(window, cx))
                            .ok()
                    }
                }),
        )
    }
}
