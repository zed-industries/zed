use std::path::PathBuf;

use client::proto;
use collections::{HashMap, HashSet};
use editor::{
    Editor, EditorEvent,
    actions::{RestartLanguageServer, StopLanguageServer},
};
use gpui::{Entity, Subscription, WeakEntity, actions};
use itertools::Itertools as _;
use language::{BinaryStatus, BufferId, LocalFile, ServerHealth};
use lsp::{LanguageServerId, LanguageServerName};
use project::{LspStore, LspStoreEvent, project_settings::ProjectSettings};
use settings::{Settings as _, SettingsStore};
use ui::{
    Context, ContextMenu, ContextMenuEntry, IconButtonShape, Indicator, PopoverMenu, Tooltip,
    Window, prelude::*,
};

use workspace::{StatusItemView, Workspace};

use crate::{LogStore, lsp_log::GlobalLogStore};

actions!(lsp_tool, [ToggleMenu]);

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
    binary_statuses: HashMap<LanguageServerName, LanguageServerBinaryStatus>,
    servers_per_buffer_abs_path: HashMap<PathBuf, HashSet<LanguageServerId>>,
}

#[derive(Debug, Clone)]
struct LanguageServerState {
    name: LanguageServerName,
    health: Option<(Option<SharedString>, ServerHealth)>,
}

#[derive(Debug, Clone)]
struct LanguageServerBinaryStatus {
    status: BinaryStatus,
    message: Option<SharedString>,
}

impl LanguageServerState {
    fn health(&self) -> Option<ServerHealth> {
        self.health.as_ref().map(|(_, health)| *health)
    }

    fn message(&self) -> Option<SharedString> {
        self.health
            .as_ref()
            .and_then(|(message, _)| message.clone())
    }
}

impl LanguageServers {
    fn update_binary_status(
        &mut self,
        binary_status: BinaryStatus,
        message: Option<&str>,
        name: LanguageServerName,
    ) {
        let binary_status_message = message.map(SharedString::new);
        if matches!(
            binary_status,
            BinaryStatus::Stopped | BinaryStatus::Failed { .. }
        ) {
            self.servers.retain(|_, server| server.name != name);
        }
        self.binary_statuses.insert(
            name,
            LanguageServerBinaryStatus {
                status: binary_status,
                message: binary_status_message,
            },
        );
    }

    fn update_server_health(
        &mut self,
        id: LanguageServerId,
        health: ServerHealth,
        message: Option<&str>,
        name: Option<LanguageServerName>,
    ) {
        if let Some(state) = self.servers.get_mut(&id) {
            state.health = Some((message.map(SharedString::new), health));
            if let Some(name) = name {
                state.name = name;
            }
        } else if let Some(name) = name {
            self.servers.insert(
                id,
                LanguageServerState {
                    health: Some((message.map(SharedString::new), health)),
                    name,
                },
            );
        }
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
                lsp_tool.on_lsp_store_event(e, window, cx)
            });

        Self {
            workspace: workspace.weak_handle(),
            lsp_store: lsp_store.downgrade(),
            active_editor: None,
            _subscriptions: vec![settings_subscription, lsp_store_subscription],
            language_servers: LanguageServers::default(),
        }
    }

    fn on_lsp_store_event(&mut self, e: &LspStoreEvent, _: &mut Window, cx: &mut Context<Self>) {
        match e {
            project::LspStoreEvent::LanguageServerUpdate {
                language_server_id,
                name,
                message: proto::update_language_server::Variant::StatusUpdate(status_update),
            } => match status_update.status {
                Some(proto::status_update::Status::Binary(binary_status)) => {
                    let Some(name) = name.as_ref() else {
                        return;
                    };
                    if let Some(binary_status) = proto::ServerBinaryStatus::from_i32(binary_status)
                    {
                        let binary_status = match binary_status {
                            proto::ServerBinaryStatus::None => BinaryStatus::None,
                            proto::ServerBinaryStatus::CheckingForUpdate => {
                                BinaryStatus::CheckingForUpdate
                            }
                            proto::ServerBinaryStatus::Downloading => BinaryStatus::Downloading,
                            proto::ServerBinaryStatus::Starting => BinaryStatus::Starting,
                            proto::ServerBinaryStatus::Stopping => BinaryStatus::Stopping,
                            proto::ServerBinaryStatus::Stopped => BinaryStatus::Stopped,
                            proto::ServerBinaryStatus::Failed => {
                                let Some(error) = status_update.message.clone() else {
                                    return;
                                };
                                BinaryStatus::Failed { error }
                            }
                        };
                        self.language_servers.update_binary_status(
                            binary_status,
                            status_update.message.as_deref(),
                            name.clone(),
                        );
                        cx.notify();
                    };
                }
                Some(proto::status_update::Status::Health(health_status)) => {
                    if let Some(health) = proto::ServerHealth::from_i32(health_status) {
                        let health = match health {
                            proto::ServerHealth::Ok => ServerHealth::Ok,
                            proto::ServerHealth::Warning => ServerHealth::Warning,
                            proto::ServerHealth::Error => ServerHealth::Error,
                        };
                        self.language_servers.update_server_health(
                            *language_server_id,
                            health,
                            status_update.message.as_deref(),
                            name.clone(),
                        );
                        cx.notify();
                    }
                }
                None => {}
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
            let binary_status = self
                .language_servers
                .binary_statuses
                .get(&server_state.name);
            if buffer_server_ids.contains(server_id) {
                buffer_servers.push((*server_id, server_state, binary_status));
            } else {
                other_servers.push((*server_id, server_state, binary_status));
            }
        }
        buffer_servers.sort_by_key(|(_, state, _)| state.name.clone());
        other_servers.sort_by_key(|(_, state, _)| state.name.clone());

        let workspace = self.workspace.clone();
        let lsp_store = self.lsp_store.clone();
        let lsp_logs = cx.global::<GlobalLogStore>().0.clone();
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
    servers: Vec<(
        LanguageServerId,
        &LanguageServerState,
        Option<&LanguageServerBinaryStatus>,
    )>,
    lsp_logs: &WeakEntity<LogStore>,
) -> ContextMenu {
    for (server_id, server, binary_status) in servers {
        let can_restart = binary_status.is_some_and(|status| status.status == BinaryStatus::None);
        let status_color = status_color(server, binary_status);
        let server_message = server.message();

        menu = menu.custom_entry(
            {
                let server_name = server.name.0.clone();
                let lsp_store = lsp_store.clone();
                let workspace = workspace.clone();
                let editor_buffers = editor_buffers.clone();
                let lsp_logs = lsp_logs.clone();
                move |_, _| {
                    h_flex()
                        .w_full()
                        .justify_between()
                        .gap_2()
                        .child(
                            h_flex()
                                .id("server-status-indicator")
                                .gap_2()
                                .child(Indicator::dot().color(status_color))
                                .child(Label::new(server_name.clone()))
                                .when_some(server_message.clone(), |div, server_message| div.tooltip(move |_, cx| Tooltip::simple(server_message.clone(), cx)))
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
                                    ).child(
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
                                            }))
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

fn status_color(
    server: &LanguageServerState,
    binary_status: Option<&LanguageServerBinaryStatus>,
) -> Color {
    let status_color = binary_status
        .and_then(|binary_status| match binary_status.status {
            BinaryStatus::None => None,
            BinaryStatus::CheckingForUpdate
            | BinaryStatus::Downloading
            | BinaryStatus::Starting => Some(Color::Modified),
            BinaryStatus::Stopping => Some(Color::Disabled),
            BinaryStatus::Stopped => Some(Color::Disabled),
            BinaryStatus::Failed { .. } => Some(Color::Error),
        })
        .or_else(|| {
            Some(match server.health()? {
                ServerHealth::Ok => Color::Success,
                ServerHealth::Warning => Color::Warning,
                ServerHealth::Error => Color::Error,
            })
        })
        .unwrap_or(Color::Success);
    status_color
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
            if let Some(binary_status) = &self.language_servers.binary_statuses.get(&server.name) {
                has_errors |= matches!(binary_status.status, BinaryStatus::Failed { .. });
                has_other_notifications |= binary_status.message.is_some();
            }

            if let Some((message, health)) = &server.health {
                has_other_notifications |= message.is_some();
                match health {
                    ServerHealth::Ok => {}
                    ServerHealth::Warning => has_warnings = true,
                    ServerHealth::Error => has_errors = true,
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
                        .tooltip(move |window, cx| {
                            Tooltip::for_action("Language servers", &ToggleMenu, window, cx)
                        }),
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
