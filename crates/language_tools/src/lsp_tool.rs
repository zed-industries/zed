use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use client::proto;
use collections::HashMap as CollectionsHashMap;
use editor::{
    Editor,
    actions::{RestartLanguageServer, StopLanguageServer},
};
use gpui::{Entity, Subscription, WeakEntity};
use language::BufferId;
use lsp::{LanguageServerId, LanguageServerName};
use project::{LspStore, LspStoreEvent, project_settings::ProjectSettings};
use settings::{Settings as _, SettingsStore};
use ui::{
    Context, ContextMenu, IconButtonShape, Indicator, PopoverMenu, Tooltip, Window, prelude::*,
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
    servers_per_buffer_abs_path: CollectionsHashMap<PathBuf, HashSet<LanguageServerId>>,
}

#[derive(Debug, Clone)]
struct LanguageServerState {
    name: LanguageServerName,
    message: Option<String>,
    status: LanguageServerStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LanguageServerStatus {
    Starting,
    Running,
    Stopping,
    Stopped,
}

impl LspTool {
    pub fn new(workspace: &Workspace, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let lsp_store = workspace.project().read(cx).lsp_store();

        let settings_subscription =
            cx.observe_global_in::<SettingsStore>(window, move |_lsp_tool, _window, cx| {
                cx.notify();
            });
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

    fn on_lsp_store_event(
        &mut self,
        e: &LspStoreEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match e {
            project::LspStoreEvent::LanguageServerUpdate {
                language_server_id,
                name,
                message: proto::update_language_server::Variant::StatusUpdate(status_update),
            } => {
                let status = match proto::status_update::Status::from_i32(status_update.status) {
                    Some(proto::status_update::Status::Starting) => LanguageServerStatus::Starting,
                    Some(proto::status_update::Status::Running) => LanguageServerStatus::Running,
                    Some(proto::status_update::Status::Stopping) => LanguageServerStatus::Stopping,
                    Some(proto::status_update::Status::Stopped) => LanguageServerStatus::Stopped,
                    _ => return,
                };

                let message = status_update.message.as_deref().map(|s| s.to_string());

                if let Some(state) = self.language_servers.servers.get_mut(language_server_id) {
                    state.status = status;
                    if let Some(message) = message {
                        state.message = Some(message);
                    }
                    if let Some(name) = name.as_ref() {
                        state.name = name.clone();
                    }
                } else if let Some(name) = name.as_ref() {
                    self.language_servers.servers.insert(
                        *language_server_id,
                        LanguageServerState {
                            name: name.clone(),
                            message,
                            status,
                        },
                    );
                }
                cx.notify();
            }
            project::LspStoreEvent::LanguageServerUpdate {
                language_server_id,
                message: proto::update_language_server::Variant::RegisteredForBuffer(update),
                ..
            } => {
                let buffer_abs_path = PathBuf::from(&update.buffer_abs_path);
                self.language_servers
                    .servers_per_buffer_abs_path
                    .entry(buffer_abs_path)
                    .or_default()
                    .insert(*language_server_id);
            }
            _ => {}
        };
    }

    fn build_context_menu(
        &self,
        lsp_logs: WeakEntity<LogStore>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<ContextMenu> {
        let workspace = self.workspace.clone();
        let lsp_store = self.lsp_store.clone();
        let active_editor = self.active_editor.as_ref().map(|ae| ae.editor.clone());
        let editor_buffers = self
            .active_editor
            .as_ref()
            .map(|active_editor| active_editor.editor_buffers.clone())
            .unwrap_or_default();
        let servers = self.language_servers.servers.clone();

        ContextMenu::build(window, cx, move |mut menu, _window, _cx| {
            // Add global actions first
            if let Some(active_editor) = active_editor.clone() {
                menu = menu
                    .entry("Restart All Servers", None, {
                        let active_editor = active_editor.clone();
                        move |window, cx| {
                            if let Some(editor) = active_editor.upgrade() {
                                editor.update(cx, |editor, cx| {
                                    editor.restart_language_server(
                                        &RestartLanguageServer,
                                        window,
                                        cx,
                                    );
                                });
                            }
                        }
                    })
                    .entry("Stop All Servers", None, {
                        let active_editor = active_editor.clone();
                        move |window, cx| {
                            if let Some(editor) = active_editor.upgrade() {
                                editor.update(cx, |editor, cx| {
                                    editor.stop_language_server(&StopLanguageServer, window, cx);
                                });
                            }
                        }
                    })
                    .separator();
            }

            // Add individual server entries
            for (server_id, server) in servers.iter() {
                let server_id = *server_id;
                let server_name = server.name.0.clone();
                let status = server.status;

                let status_text = match status {
                    LanguageServerStatus::Starting => "Starting",
                    LanguageServerStatus::Running => "Running",
                    LanguageServerStatus::Stopping => "Stopping",
                    LanguageServerStatus::Stopped => "Stopped",
                };

                menu = menu.header(&format!("{} ({})", server_name, status_text));

                // Add server-specific actions
                let can_stop = matches!(
                    status,
                    LanguageServerStatus::Starting | LanguageServerStatus::Running
                );
                let can_restart = matches!(
                    status,
                    LanguageServerStatus::Stopping | LanguageServerStatus::Stopped
                );

                if can_stop {
                    menu = menu.entry("Stop Server", None, {
                        let lsp_store = lsp_store.clone();
                        move |_window, cx| {
                            lsp_store
                                .update(cx, |lsp_store, cx| {
                                    lsp_store.stop_language_servers_for_buffers(
                                        Vec::new(),
                                        vec![server_id],
                                        cx,
                                    );
                                })
                                .ok();
                        }
                    });
                }

                if can_restart {
                    menu = menu.entry("Restart Server", None, {
                        let lsp_store = lsp_store.clone();
                        let workspace = workspace.clone();
                        let editor_buffers = editor_buffers.clone();
                        move |_window, cx| {
                            if let Some(workspace) = workspace.upgrade() {
                                let buffer_store =
                                    workspace.read(cx).project().read(cx).buffer_store().clone();
                                let buffers = editor_buffers
                                    .iter()
                                    .flat_map(|buffer_id| buffer_store.read(cx).get(*buffer_id))
                                    .collect::<Vec<_>>();
                                if !buffers.is_empty() {
                                    lsp_store
                                        .update(cx, |lsp_store, cx| {
                                            lsp_store.restart_language_servers_for_buffers(
                                                buffers,
                                                vec![server_id],
                                                cx,
                                            );
                                        })
                                        .ok();
                                }
                            }
                        }
                    });
                }

                // Check if server has logs
                // Add log entries without checking - they'll be disabled if no logs exist
                menu = menu
                    .entry("Open Log", None, {
                        let workspace = workspace.clone();
                        let lsp_logs = lsp_logs.clone();
                        move |window, cx| {
                            if let Some(lsp_logs) = lsp_logs.upgrade() {
                                lsp_logs.update(cx, |lsp_logs, cx| {
                                    lsp_logs.open_server_log(
                                        workspace.clone(),
                                        server_id,
                                        window,
                                        cx,
                                    );
                                });
                            }
                        }
                    })
                    .entry("Open LSP Messages", None, {
                        let workspace = workspace.clone();
                        let lsp_logs = lsp_logs.clone();
                        move |window, cx| {
                            if let Some(lsp_logs) = lsp_logs.upgrade() {
                                lsp_logs.update(cx, |lsp_logs, cx| {
                                    lsp_logs.open_server_trace(
                                        workspace.clone(),
                                        server_id,
                                        window,
                                        cx,
                                    );
                                });
                            }
                        }
                    });

                menu = menu.separator();
            }

            menu
        })
    }
}

impl StatusItemView for LspTool {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn workspace::ItemHandle>,
        _window: &mut Window,
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
                    self.active_editor = Some(ActiveEditor {
                        editor: editor.downgrade(),
                        _editor_subscription: Subscription::new(|| {}),
                        editor_buffers,
                    });
                }
            } else {
                self.active_editor = None;
            }
        } else {
            self.active_editor = None;
        }
        cx.notify();
    }
}

impl Render for LspTool {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl ui::IntoElement {
        if self.active_editor.is_none() || self.language_servers.servers.is_empty() {
            return div();
        }

        let mut has_errors = false;
        let mut has_warnings = false;
        let mut has_other_notifications = false;
        for server in self.language_servers.servers.values() {
            if let Some(ref message) = server.message {
                // Simple heuristic to determine severity from message content
                let message_lower = message.to_lowercase();
                if message_lower.contains("error") || message_lower.contains("failed") {
                    has_errors = true;
                } else if message_lower.contains("warning") || message_lower.contains("warn") {
                    has_warnings = true;
                } else {
                    has_other_notifications = true;
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

        let lsp_logs = cx.global::<GlobalLogStore>().0.downgrade();

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
                    let this = cx.weak_entity();
                    move |window, cx| {
                        this.upgrade().map(|this| {
                            this.update(cx, |this, cx| {
                                this.build_context_menu(lsp_logs.clone(), window, cx)
                            })
                        })
                    }
                }),
        )
    }
}
