use std::{
    collections::{BTreeMap, HashMap},
    path::{Path, PathBuf},
    rc::Rc,
    time::Duration,
};

use client::proto;
use collections::HashSet;
use editor::{Editor, EditorEvent};
use gpui::{Corner, Entity, Subscription, Task, WeakEntity, actions};
use language::{BinaryStatus, BufferId, ServerHealth};
use lsp::{LanguageServerId, LanguageServerName, LanguageServerSelector};
use project::{
    LspStore, LspStoreEvent, Worktree, lsp_store::log_store::GlobalLogStore,
    project_settings::ProjectSettings,
};
use settings::{Settings as _, SettingsStore};
use ui::{
    ContextMenu, ContextMenuEntry, Indicator, PopoverMenu, PopoverMenuHandle, Tooltip, prelude::*,
};

use util::{ResultExt, rel_path::RelPath};
use workspace::{StatusItemView, Workspace};

use crate::lsp_log_view;

actions!(
    lsp_tool,
    [
        /// Toggles the language server tool menu.
        ToggleMenu
    ]
);

pub struct LspButton {
    server_state: Entity<LanguageServerState>,
    popover_menu_handle: PopoverMenuHandle<ContextMenu>,
    lsp_menu: Option<Entity<ContextMenu>>,
    lsp_menu_refresh: Task<()>,
    _subscriptions: Vec<Subscription>,
}

#[derive(Debug)]
struct LanguageServerState {
    items: Vec<LspMenuItem>,
    workspace: WeakEntity<Workspace>,
    lsp_store: WeakEntity<LspStore>,
    active_editor: Option<ActiveEditor>,
    language_servers: LanguageServers,
}

struct ActiveEditor {
    editor: WeakEntity<Editor>,
    _editor_subscription: Subscription,
    editor_buffers: HashSet<BufferId>,
}

impl std::fmt::Debug for ActiveEditor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ActiveEditor")
            .field("editor", &self.editor)
            .field("editor_buffers", &self.editor_buffers)
            .finish_non_exhaustive()
    }
}

#[derive(Debug, Default, Clone)]
struct LanguageServers {
    health_statuses: HashMap<LanguageServerId, LanguageServerHealthStatus>,
    binary_statuses: HashMap<LanguageServerName, LanguageServerBinaryStatus>,
    servers_per_buffer_abs_path: HashMap<PathBuf, ServersForPath>,
}

#[derive(Debug, Clone)]
struct ServersForPath {
    servers: HashMap<LanguageServerId, Option<LanguageServerName>>,
    worktree: Option<WeakEntity<Worktree>>,
}

#[derive(Debug, Clone)]
struct LanguageServerHealthStatus {
    name: LanguageServerName,
    health: Option<(Option<SharedString>, ServerHealth)>,
}

#[derive(Debug, Clone)]
struct LanguageServerBinaryStatus {
    status: BinaryStatus,
    message: Option<SharedString>,
}

#[derive(Debug, Clone)]
struct ServerInfo {
    name: LanguageServerName,
    id: LanguageServerId,
    health: Option<ServerHealth>,
    binary_status: Option<LanguageServerBinaryStatus>,
    message: Option<SharedString>,
}

impl ServerInfo {
    fn server_selector(&self) -> LanguageServerSelector {
        LanguageServerSelector::Id(self.id)
    }

    fn can_stop(&self) -> bool {
        self.binary_status.as_ref().is_none_or(|status| {
            matches!(status.status, BinaryStatus::None | BinaryStatus::Starting)
        })
    }
}

impl LanguageServerHealthStatus {
    fn health(&self) -> Option<ServerHealth> {
        self.health.as_ref().map(|(_, health)| *health)
    }

    fn message(&self) -> Option<SharedString> {
        self.health
            .as_ref()
            .and_then(|(message, _)| message.clone())
    }
}

impl LanguageServerState {
    fn fill_menu(&self, mut menu: ContextMenu, cx: &mut Context<Self>) -> ContextMenu {
        let lsp_logs = cx
            .try_global::<GlobalLogStore>()
            .map(|lsp_logs| lsp_logs.0.clone());
        let Some(lsp_logs) = lsp_logs else {
            return menu;
        };

        let server_versions = self
            .lsp_store
            .update(cx, |lsp_store, _| {
                lsp_store
                    .language_server_statuses()
                    .map(|(server_id, status)| (server_id, status.server_version.clone()))
                    .collect::<HashMap<_, _>>()
            })
            .unwrap_or_default();

        let mut first_button_encountered = false;
        for item in &self.items {
            if let LspMenuItem::ToggleServersButton { restart } = item {
                let label = if *restart {
                    "Restart All Servers"
                } else {
                    "Stop All Servers"
                };

                let restart = *restart;

                let button = ContextMenuEntry::new(label).handler({
                    let state = cx.entity();
                    move |_, cx| {
                        let lsp_store = state.read(cx).lsp_store.clone();
                        lsp_store
                            .update(cx, |lsp_store, cx| {
                                if restart {
                                    let Some(workspace) = state.read(cx).workspace.upgrade() else {
                                        return;
                                    };
                                    let project = workspace.read(cx).project().clone();
                                    let path_style = project.read(cx).path_style(cx);
                                    let buffer_store = project.read(cx).buffer_store().clone();
                                    let buffers = state
                                        .read(cx)
                                        .language_servers
                                        .servers_per_buffer_abs_path
                                        .iter()
                                        .filter_map(|(abs_path, servers)| {
                                            let worktree =
                                                servers.worktree.as_ref()?.upgrade()?.read(cx);
                                            let relative_path =
                                                abs_path.strip_prefix(&worktree.abs_path()).ok()?;
                                            let relative_path =
                                                RelPath::new(relative_path, path_style)
                                                    .log_err()?;
                                            let entry = worktree.entry_for_path(&relative_path)?;
                                            let project_path =
                                                project.read(cx).path_for_entry(entry.id, cx)?;
                                            buffer_store.read(cx).get_by_path(&project_path)
                                        })
                                        .collect();
                                    let selectors = state
                                        .read(cx)
                                        .items
                                        .iter()
                                        // Do not try to use IDs as we have stopped all servers already, when allowing to restart them all
                                        .flat_map(|item| match item {
                                            LspMenuItem::Header { .. } => None,
                                            LspMenuItem::ToggleServersButton { .. } => None,
                                            LspMenuItem::WithHealthCheck { health, .. } => Some(
                                                LanguageServerSelector::Name(health.name.clone()),
                                            ),
                                            LspMenuItem::WithBinaryStatus {
                                                server_name, ..
                                            } => Some(LanguageServerSelector::Name(
                                                server_name.clone(),
                                            )),
                                        })
                                        .collect();
                                    lsp_store.restart_language_servers_for_buffers(
                                        buffers, selectors, cx,
                                    );
                                } else {
                                    lsp_store.stop_all_language_servers(cx);
                                }
                            })
                            .ok();
                    }
                });

                if !first_button_encountered {
                    menu = menu.separator();
                    first_button_encountered = true;
                }

                menu = menu.item(button);
                continue;
            } else if let LspMenuItem::Header { header, separator } = item {
                menu = menu
                    .when(*separator, |menu| menu.separator())
                    .when_some(header.as_ref(), |menu, header| menu.header(header));
                continue;
            }

            let Some(server_info) = item.server_info() else {
                continue;
            };
            let server_selector = server_info.server_selector();
            let is_remote = self
                .lsp_store
                .update(cx, |lsp_store, _| lsp_store.as_remote().is_some())
                .unwrap_or(false);
            let has_logs = is_remote || lsp_logs.read(cx).has_server_logs(&server_selector);

            let (status_color, status_label) = server_info
                .binary_status
                .as_ref()
                .and_then(|binary_status| match binary_status.status {
                    BinaryStatus::None => None,
                    BinaryStatus::CheckingForUpdate
                    | BinaryStatus::Downloading
                    | BinaryStatus::Starting => Some((Color::Modified, "Starting…")),
                    BinaryStatus::Stopping | BinaryStatus::Stopped => {
                        Some((Color::Disabled, "Stopped"))
                    }
                    BinaryStatus::Failed { .. } => Some((Color::Error, "Error")),
                })
                .or_else(|| {
                    Some(match server_info.health? {
                        ServerHealth::Ok => (Color::Success, "Running"),
                        ServerHealth::Warning => (Color::Warning, "Warning"),
                        ServerHealth::Error => (Color::Error, "Error"),
                    })
                })
                .unwrap_or((Color::Success, "Running"));

            let message = server_info
                .message
                .as_ref()
                .or_else(|| server_info.binary_status.as_ref()?.message.as_ref())
                .cloned();

            let server_version = server_versions
                .get(&server_info.id)
                .and_then(|version| version.clone());

            let truncated_message = message.as_ref().and_then(|message| {
                message
                    .lines()
                    .filter(|line| !line.trim().is_empty())
                    .map(SharedString::new)
                    .next()
            });

            let metadata_label = match (&server_version, &truncated_message) {
                (None, None) => None,
                (Some(version), None) => Some(SharedString::from(format!("v{}", version.as_ref()))),
                (None, Some(message)) => Some(message.clone()),
                (Some(version), Some(message)) => Some(SharedString::from(format!(
                    "v{}\n\n{}",
                    version.as_ref(),
                    message.as_ref()
                ))),
            };

            let submenu_server_name = server_info.name.clone();
            let submenu_server_info = server_info.clone();

            menu = menu.submenu_with_colored_icon(
                server_info.name.0.clone(),
                IconName::Circle,
                status_color,
                {
                    let lsp_logs = lsp_logs.clone();
                    let message = message.clone();
                    let server_selector = server_selector.clone();
                    let workspace = self.workspace.clone();
                    let lsp_store = self.lsp_store.clone();
                    let state = cx.entity().downgrade();
                    let can_stop = submenu_server_info.can_stop();

                    move |menu, _window, _cx| {
                        let mut submenu = menu;

                        if let Some(ref message) = message {
                            let workspace_for_message = workspace.clone();
                            let message_for_handler = message.clone();
                            let server_name_for_message = submenu_server_name.clone();
                            submenu = submenu.entry("View Message", None, move |window, cx| {
                                let Some(create_buffer) = workspace_for_message
                                    .update(cx, |workspace, cx| {
                                        workspace.project().update(cx, |project, cx| {
                                            project.create_buffer(None, false, cx)
                                        })
                                    })
                                    .ok()
                                else {
                                    return;
                                };

                                let window_handle = window.window_handle();
                                let workspace = workspace_for_message.clone();
                                let message = message_for_handler.clone();
                                let server_name = server_name_for_message.clone();
                                cx.spawn(async move |cx| {
                                    let buffer = create_buffer.await?;
                                    buffer.update(cx, |buffer, cx| {
                                        buffer.edit(
                                            [(
                                                0..0,
                                                format!(
                                                    "Language server {server_name}:\n\n{message}"
                                                ),
                                            )],
                                            None,
                                            cx,
                                        );
                                        buffer.set_capability(language::Capability::ReadOnly, cx);
                                    });

                                    workspace.update(cx, |workspace, cx| {
                                        window_handle.update(cx, |_, window, cx| {
                                            workspace.add_item_to_active_pane(
                                                Box::new(cx.new(|cx| {
                                                    let mut editor = Editor::for_buffer(
                                                        buffer, None, window, cx,
                                                    );
                                                    editor.set_read_only(true);
                                                    editor
                                                })),
                                                None,
                                                true,
                                                window,
                                                cx,
                                            );
                                        })
                                    })??;

                                    anyhow::Ok(())
                                })
                                .detach();
                            });
                        }

                        if has_logs {
                            let lsp_logs_for_debug = lsp_logs.clone();
                            let workspace_for_debug = workspace.clone();
                            let server_selector_for_debug = server_selector.clone();
                            submenu = submenu.entry("View Logs", None, move |window, cx| {
                                lsp_log_view::open_server_trace(
                                    &lsp_logs_for_debug,
                                    workspace_for_debug.clone(),
                                    server_selector_for_debug.clone(),
                                    window,
                                    cx,
                                );
                            });
                        }

                        let state_for_restart = state.clone();
                        let workspace_for_restart = workspace.clone();
                        let lsp_store_for_restart = lsp_store.clone();
                        let server_name_for_restart = submenu_server_name.clone();
                        submenu = submenu.entry("Restart Server", None, move |_window, cx| {
                            let Some(workspace) = workspace_for_restart.upgrade() else {
                                return;
                            };

                            let project = workspace.read(cx).project().clone();
                            let path_style = project.read(cx).path_style(cx);
                            let buffer_store = project.read(cx).buffer_store().clone();

                            let buffers = state_for_restart
                                .update(cx, |state, cx| {
                                    let server_buffers = state
                                        .language_servers
                                        .servers_per_buffer_abs_path
                                        .iter()
                                        .filter_map(|(abs_path, servers)| {
                                            // Check if this server is associated with this path
                                            let has_server = servers.servers.values().any(|name| {
                                                name.as_ref() == Some(&server_name_for_restart)
                                            });

                                            if !has_server {
                                                return None;
                                            }

                                            let worktree = servers.worktree.as_ref()?.upgrade()?;
                                            let worktree_ref = worktree.read(cx);
                                            let relative_path = abs_path
                                                .strip_prefix(&worktree_ref.abs_path())
                                                .ok()?;
                                            let relative_path =
                                                RelPath::new(relative_path, path_style)
                                                    .log_err()?;
                                            let entry =
                                                worktree_ref.entry_for_path(&relative_path)?;
                                            let project_path =
                                                project.read(cx).path_for_entry(entry.id, cx)?;

                                            buffer_store.read(cx).get_by_path(&project_path)
                                        })
                                        .collect::<Vec<_>>();

                                    if server_buffers.is_empty() {
                                        state
                                            .language_servers
                                            .servers_per_buffer_abs_path
                                            .iter()
                                            .filter_map(|(abs_path, servers)| {
                                                let worktree =
                                                    servers.worktree.as_ref()?.upgrade()?.read(cx);
                                                let relative_path = abs_path
                                                    .strip_prefix(&worktree.abs_path())
                                                    .ok()?;
                                                let relative_path =
                                                    RelPath::new(relative_path, path_style)
                                                        .log_err()?;
                                                let entry =
                                                    worktree.entry_for_path(&relative_path)?;
                                                let project_path = project
                                                    .read(cx)
                                                    .path_for_entry(entry.id, cx)?;
                                                buffer_store.read(cx).get_by_path(&project_path)
                                            })
                                            .collect()
                                    } else {
                                        server_buffers
                                    }
                                })
                                .unwrap_or_default();

                            if !buffers.is_empty() {
                                lsp_store_for_restart
                                    .update(cx, |lsp_store, cx| {
                                        lsp_store.restart_language_servers_for_buffers(
                                            buffers,
                                            HashSet::from_iter([LanguageServerSelector::Name(
                                                server_name_for_restart.clone(),
                                            )]),
                                            cx,
                                        );
                                    })
                                    .ok();
                            }
                        });

                        if can_stop {
                            let lsp_store_for_stop = lsp_store.clone();
                            let server_selector_for_stop = server_selector.clone();

                            submenu = submenu.entry("Stop Server", None, move |_window, cx| {
                                lsp_store_for_stop
                                    .update(cx, |lsp_store, cx| {
                                        lsp_store
                                            .stop_language_servers_for_buffers(
                                                Vec::new(),
                                                HashSet::from_iter([
                                                    server_selector_for_stop.clone()
                                                ]),
                                                cx,
                                            )
                                            .detach_and_log_err(cx);
                                    })
                                    .ok();
                            });
                        }

                        submenu = submenu.separator().custom_row({
                            let metadata_label = metadata_label.clone();
                            move |_, _| {
                                h_flex()
                                    .id("metadata-container")
                                    .ml_neg_1()
                                    .gap_1()
                                    .max_w(rems(164.))
                                    .child(
                                        Icon::new(IconName::Circle)
                                            .color(status_color)
                                            .size(IconSize::Small),
                                    )
                                    .child(
                                        Label::new(status_label)
                                            .size(LabelSize::Small)
                                            .color(Color::Muted),
                                    )
                                    .when_some(metadata_label.as_ref(), |submenu, metadata| {
                                        submenu
                                            .child(
                                                Icon::new(IconName::Dash)
                                                    .color(Color::Disabled)
                                                    .size(IconSize::XSmall),
                                            )
                                            .child(
                                                Label::new(metadata)
                                                    .size(LabelSize::Small)
                                                    .color(Color::Muted)
                                                    .truncate(),
                                            )
                                    })
                                    .into_any_element()
                            }
                        });

                        submenu
                    }
                },
            );
        }
        menu
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
            self.health_statuses.retain(|_, server| server.name != name);
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
        if let Some(state) = self.health_statuses.get_mut(&id) {
            state.health = Some((message.map(SharedString::new), health));
            if let Some(name) = name {
                state.name = name;
            }
        } else if let Some(name) = name {
            self.health_statuses.insert(
                id,
                LanguageServerHealthStatus {
                    health: Some((message.map(SharedString::new), health)),
                    name,
                },
            );
        }
    }

    fn is_empty(&self) -> bool {
        self.binary_statuses.is_empty() && self.health_statuses.is_empty()
    }
}

#[derive(Debug)]
enum ServerData<'a> {
    WithHealthCheck {
        server_id: LanguageServerId,
        health: &'a LanguageServerHealthStatus,
        binary_status: Option<&'a LanguageServerBinaryStatus>,
    },
    WithBinaryStatus {
        server_id: LanguageServerId,
        server_name: &'a LanguageServerName,
        binary_status: &'a LanguageServerBinaryStatus,
    },
}

#[derive(Debug)]
enum LspMenuItem {
    WithHealthCheck {
        server_id: LanguageServerId,
        health: LanguageServerHealthStatus,
        binary_status: Option<LanguageServerBinaryStatus>,
    },
    WithBinaryStatus {
        server_id: LanguageServerId,
        server_name: LanguageServerName,
        binary_status: LanguageServerBinaryStatus,
    },
    ToggleServersButton {
        restart: bool,
    },
    Header {
        header: Option<SharedString>,
        separator: bool,
    },
}

impl LspMenuItem {
    fn server_info(&self) -> Option<ServerInfo> {
        match self {
            Self::Header { .. } => None,
            Self::ToggleServersButton { .. } => None,
            Self::WithHealthCheck {
                server_id,
                health,
                binary_status,
                ..
            } => Some(ServerInfo {
                name: health.name.clone(),
                id: *server_id,
                health: health.health(),
                binary_status: binary_status.clone(),
                message: health.message(),
            }),
            Self::WithBinaryStatus {
                server_id,
                server_name,
                binary_status,
                ..
            } => Some(ServerInfo {
                name: server_name.clone(),
                id: *server_id,
                health: None,
                binary_status: Some(binary_status.clone()),
                message: binary_status.message.clone(),
            }),
        }
    }
}

impl ServerData<'_> {
    fn into_lsp_item(self) -> LspMenuItem {
        match self {
            Self::WithHealthCheck {
                server_id,
                health,
                binary_status,
                ..
            } => LspMenuItem::WithHealthCheck {
                server_id,
                health: health.clone(),
                binary_status: binary_status.cloned(),
            },
            Self::WithBinaryStatus {
                server_id,
                server_name,
                binary_status,
                ..
            } => LspMenuItem::WithBinaryStatus {
                server_id,
                server_name: server_name.clone(),
                binary_status: binary_status.clone(),
            },
        }
    }
}

impl LspButton {
    pub fn new(
        workspace: &Workspace,
        popover_menu_handle: PopoverMenuHandle<ContextMenu>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let settings_subscription =
            cx.observe_global_in::<SettingsStore>(window, move |lsp_button, window, cx| {
                if ProjectSettings::get_global(cx).global_lsp_settings.button {
                    if lsp_button.lsp_menu.is_none() {
                        lsp_button.refresh_lsp_menu(true, window, cx);
                    }
                } else if lsp_button.lsp_menu.take().is_some() {
                    cx.notify();
                }
            });

        let lsp_store = workspace.project().read(cx).lsp_store();
        let mut language_servers = LanguageServers::default();
        for (_, status) in lsp_store.read(cx).language_server_statuses() {
            language_servers.binary_statuses.insert(
                status.name.clone(),
                LanguageServerBinaryStatus {
                    status: BinaryStatus::None,
                    message: None,
                },
            );
        }

        let lsp_store_subscription =
            cx.subscribe_in(&lsp_store, window, |lsp_button, _, e, window, cx| {
                lsp_button.on_lsp_store_event(e, window, cx)
            });

        let server_state = cx.new(|_| LanguageServerState {
            workspace: workspace.weak_handle(),
            items: Vec::new(),
            lsp_store: lsp_store.downgrade(),
            active_editor: None,
            language_servers,
        });

        let mut lsp_button = Self {
            server_state,
            popover_menu_handle,
            lsp_menu: None,
            lsp_menu_refresh: Task::ready(()),
            _subscriptions: vec![settings_subscription, lsp_store_subscription],
        };
        if !lsp_button
            .server_state
            .read(cx)
            .language_servers
            .binary_statuses
            .is_empty()
        {
            lsp_button.refresh_lsp_menu(true, window, cx);
        }

        lsp_button
    }

    fn on_lsp_store_event(
        &mut self,
        e: &LspStoreEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.lsp_menu.is_none() {
            return;
        };
        let mut updated = false;

        // TODO `LspStore` is global and reports status from all language servers, even from the other windows.
        // Also, we do not get "LSP removed" events so LSPs are never removed.
        match e {
            LspStoreEvent::LanguageServerUpdate {
                language_server_id,
                name,
                message: proto::update_language_server::Variant::StatusUpdate(status_update),
            } => match &status_update.status {
                Some(proto::status_update::Status::Binary(binary_status)) => {
                    let Some(name) = name.as_ref() else {
                        return;
                    };
                    if let Some(binary_status) = proto::ServerBinaryStatus::from_i32(*binary_status)
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
                        self.server_state.update(cx, |state, _| {
                            state.language_servers.update_binary_status(
                                binary_status,
                                status_update.message.as_deref(),
                                name.clone(),
                            );
                        });
                        updated = true;
                    };
                }
                Some(proto::status_update::Status::Health(health_status)) => {
                    if let Some(health) = proto::ServerHealth::from_i32(*health_status) {
                        let health = match health {
                            proto::ServerHealth::Ok => ServerHealth::Ok,
                            proto::ServerHealth::Warning => ServerHealth::Warning,
                            proto::ServerHealth::Error => ServerHealth::Error,
                        };
                        self.server_state.update(cx, |state, _| {
                            state.language_servers.update_server_health(
                                *language_server_id,
                                health,
                                status_update.message.as_deref(),
                                name.clone(),
                            );
                        });
                        updated = true;
                    }
                }
                None => {}
            },
            LspStoreEvent::LanguageServerUpdate {
                language_server_id,
                name,
                message: proto::update_language_server::Variant::RegisteredForBuffer(update),
                ..
            } => {
                self.server_state.update(cx, |state, cx| {
                    let Ok(worktree) = state.workspace.update(cx, |workspace, cx| {
                        workspace
                            .project()
                            .read(cx)
                            .find_worktree(Path::new(&update.buffer_abs_path), cx)
                            .map(|(worktree, _)| worktree.downgrade())
                    }) else {
                        return;
                    };
                    let entry = state
                        .language_servers
                        .servers_per_buffer_abs_path
                        .entry(PathBuf::from(&update.buffer_abs_path))
                        .or_insert_with(|| ServersForPath {
                            servers: HashMap::default(),
                            worktree: worktree.clone(),
                        });
                    entry.servers.insert(*language_server_id, name.clone());
                    if worktree.is_some() {
                        entry.worktree = worktree;
                    }
                });
                updated = true;
            }
            _ => {}
        };

        if updated {
            self.refresh_lsp_menu(false, window, cx);
        }
    }

    fn regenerate_items(&mut self, cx: &mut App) {
        self.server_state.update(cx, |state, cx| {
            let active_worktrees = state
                .active_editor
                .as_ref()
                .into_iter()
                .flat_map(|active_editor| {
                    active_editor
                        .editor
                        .upgrade()
                        .into_iter()
                        .flat_map(|active_editor| {
                            active_editor
                                .read(cx)
                                .buffer()
                                .read(cx)
                                .all_buffers()
                                .into_iter()
                                .filter_map(|buffer| {
                                    project::File::from_dyn(buffer.read(cx).file())
                                })
                                .map(|buffer_file| buffer_file.worktree.clone())
                        })
                })
                .collect::<HashSet<_>>();

            let mut server_ids_to_worktrees =
                HashMap::<LanguageServerId, Entity<Worktree>>::default();
            let mut server_names_to_worktrees = HashMap::<
                LanguageServerName,
                HashSet<(Entity<Worktree>, LanguageServerId)>,
            >::default();
            for servers_for_path in state.language_servers.servers_per_buffer_abs_path.values() {
                if let Some(worktree) = servers_for_path
                    .worktree
                    .as_ref()
                    .and_then(|worktree| worktree.upgrade())
                {
                    for (server_id, server_name) in &servers_for_path.servers {
                        server_ids_to_worktrees.insert(*server_id, worktree.clone());
                        if let Some(server_name) = server_name {
                            server_names_to_worktrees
                                .entry(server_name.clone())
                                .or_default()
                                .insert((worktree.clone(), *server_id));
                        }
                    }
                }
            }
            state
                .lsp_store
                .update(cx, |lsp_store, cx| {
                    for (server_id, status) in lsp_store.language_server_statuses() {
                        if let Some(worktree) = status.worktree.and_then(|worktree_id| {
                            lsp_store
                                .worktree_store()
                                .read(cx)
                                .worktree_for_id(worktree_id, cx)
                        }) {
                            server_ids_to_worktrees.insert(server_id, worktree.clone());
                            server_names_to_worktrees
                                .entry(status.name.clone())
                                .or_default()
                                .insert((worktree, server_id));
                        }
                    }
                })
                .ok();

            let mut servers_per_worktree = BTreeMap::<SharedString, Vec<ServerData>>::new();
            let mut servers_with_health_checks = HashSet::default();

            for (server_id, health) in &state.language_servers.health_statuses {
                let worktree = server_ids_to_worktrees.get(server_id).or_else(|| {
                    let worktrees = server_names_to_worktrees.get(&health.name)?;
                    worktrees
                        .iter()
                        .find(|(worktree, _)| active_worktrees.contains(worktree))
                        .or_else(|| worktrees.iter().next())
                        .map(|(worktree, _)| worktree)
                });
                servers_with_health_checks.insert(&health.name);
                let worktree_name =
                    worktree.map(|worktree| SharedString::new(worktree.read(cx).root_name_str()));

                let binary_status = state.language_servers.binary_statuses.get(&health.name);
                let server_data = ServerData::WithHealthCheck {
                    server_id: *server_id,
                    health,
                    binary_status,
                };
                if let Some(worktree_name) = worktree_name {
                    servers_per_worktree
                        .entry(worktree_name.clone())
                        .or_default()
                        .push(server_data);
                }
            }

            let mut can_stop_all = !state.language_servers.health_statuses.is_empty();
            let mut can_restart_all = state.language_servers.health_statuses.is_empty();
            for (server_name, binary_status) in state
                .language_servers
                .binary_statuses
                .iter()
                .filter(|(name, _)| !servers_with_health_checks.contains(name))
            {
                match binary_status.status {
                    BinaryStatus::None => {
                        can_restart_all = false;
                        can_stop_all |= true;
                    }
                    BinaryStatus::CheckingForUpdate => {
                        can_restart_all = false;
                        can_stop_all = false;
                    }
                    BinaryStatus::Downloading => {
                        can_restart_all = false;
                        can_stop_all = false;
                    }
                    BinaryStatus::Starting => {
                        can_restart_all = false;
                        can_stop_all = false;
                    }
                    BinaryStatus::Stopping => {
                        can_restart_all = false;
                        can_stop_all = false;
                    }
                    BinaryStatus::Stopped => {}
                    BinaryStatus::Failed { .. } => {}
                }

                if let Some(worktrees_for_name) = server_names_to_worktrees.get(server_name)
                    && let Some((worktree, server_id)) = worktrees_for_name
                        .iter()
                        .find(|(worktree, _)| active_worktrees.contains(worktree))
                        .or_else(|| worktrees_for_name.iter().next())
                {
                    let worktree_name = SharedString::new(worktree.read(cx).root_name_str());
                    servers_per_worktree
                        .entry(worktree_name.clone())
                        .or_default()
                        .push(ServerData::WithBinaryStatus {
                            server_name,
                            binary_status,
                            server_id: *server_id,
                        });
                }
            }

            let mut new_lsp_items = Vec::with_capacity(servers_per_worktree.len() + 1);
            for (worktree_name, worktree_servers) in servers_per_worktree {
                if worktree_servers.is_empty() {
                    continue;
                }
                new_lsp_items.push(LspMenuItem::Header {
                    header: Some(worktree_name),
                    separator: false,
                });
                new_lsp_items.extend(worktree_servers.into_iter().map(ServerData::into_lsp_item));
            }
            if !new_lsp_items.is_empty() {
                if can_stop_all {
                    new_lsp_items.push(LspMenuItem::ToggleServersButton { restart: true });
                    new_lsp_items.push(LspMenuItem::ToggleServersButton { restart: false });
                } else if can_restart_all {
                    new_lsp_items.push(LspMenuItem::ToggleServersButton { restart: true });
                }
            }

            state.items = new_lsp_items;
        });
    }

    fn refresh_lsp_menu(
        &mut self,
        create_if_empty: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if create_if_empty || self.lsp_menu.is_some() {
            let state = self.server_state.clone();
            self.lsp_menu_refresh = cx.spawn_in(window, async move |lsp_button, cx| {
                cx.background_executor()
                    .timer(Duration::from_millis(30))
                    .await;
                lsp_button
                    .update_in(cx, |lsp_button, window, cx| {
                        lsp_button.regenerate_items(cx);
                        let menu = ContextMenu::build(window, cx, |menu, _, cx| {
                            state.update(cx, |state, cx| state.fill_menu(menu, cx))
                        });
                        lsp_button.lsp_menu = Some(menu.clone());
                        lsp_button.popover_menu_handle.refresh_menu(
                            window,
                            cx,
                            Rc::new(move |_, _| Some(menu.clone())),
                        );
                        cx.notify();
                    })
                    .ok();
            });
        }
    }
}

impl StatusItemView for LspButton {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn workspace::ItemHandle>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if ProjectSettings::get_global(cx).global_lsp_settings.button {
            if let Some(editor) = active_pane_item.and_then(|item| item.downcast::<Editor>()) {
                if Some(&editor)
                    != self
                        .server_state
                        .read(cx)
                        .active_editor
                        .as_ref()
                        .and_then(|active_editor| active_editor.editor.upgrade())
                        .as_ref()
                {
                    let editor_buffers =
                        HashSet::from_iter(editor.read(cx).buffer().read(cx).excerpt_buffer_ids());
                    let _editor_subscription = cx.subscribe_in(
                        &editor,
                        window,
                        |lsp_button, _, e: &EditorEvent, window, cx| match e {
                            EditorEvent::ExcerptsAdded { buffer, .. } => {
                                let updated = lsp_button.server_state.update(cx, |state, cx| {
                                    if let Some(active_editor) = state.active_editor.as_mut() {
                                        let buffer_id = buffer.read(cx).remote_id();
                                        active_editor.editor_buffers.insert(buffer_id)
                                    } else {
                                        false
                                    }
                                });
                                if updated {
                                    lsp_button.refresh_lsp_menu(false, window, cx);
                                }
                            }
                            EditorEvent::ExcerptsRemoved {
                                removed_buffer_ids, ..
                            } => {
                                let removed = lsp_button.server_state.update(cx, |state, _| {
                                    let mut removed = false;
                                    if let Some(active_editor) = state.active_editor.as_mut() {
                                        for id in removed_buffer_ids {
                                            active_editor.editor_buffers.retain(|buffer_id| {
                                                let retain = buffer_id != id;
                                                removed |= !retain;
                                                retain
                                            });
                                        }
                                    }
                                    removed
                                });
                                if removed {
                                    lsp_button.refresh_lsp_menu(false, window, cx);
                                }
                            }
                            _ => {}
                        },
                    );
                    self.server_state.update(cx, |state, _| {
                        state.active_editor = Some(ActiveEditor {
                            editor: editor.downgrade(),
                            _editor_subscription,
                            editor_buffers,
                        });
                    });
                    self.refresh_lsp_menu(true, window, cx);
                }
            } else if self.server_state.read(cx).active_editor.is_some() {
                self.server_state.update(cx, |state, _| {
                    state.active_editor = None;
                });
                self.refresh_lsp_menu(false, window, cx);
            }
        } else if self.server_state.read(cx).active_editor.is_some() {
            self.server_state.update(cx, |state, _| {
                state.active_editor = None;
            });
            self.refresh_lsp_menu(false, window, cx);
        }
    }
}

impl Render for LspButton {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl ui::IntoElement {
        if self.server_state.read(cx).language_servers.is_empty() || self.lsp_menu.is_none() {
            return div().hidden();
        }

        let mut has_errors = false;
        let mut has_warnings = false;
        let mut has_other_notifications = false;
        let state = self.server_state.read(cx);
        for binary_status in state.language_servers.binary_statuses.values() {
            has_errors |= matches!(binary_status.status, BinaryStatus::Failed { .. });
            has_other_notifications |= binary_status.message.is_some();
        }

        for server in state.language_servers.health_statuses.values() {
            if let Some((message, health)) = &server.health {
                has_other_notifications |= message.is_some();
                match health {
                    ServerHealth::Ok => {}
                    ServerHealth::Warning => has_warnings = true,
                    ServerHealth::Error => has_errors = true,
                }
            }
        }

        let (indicator, description) = if has_errors {
            (
                Some(Indicator::dot().color(Color::Error)),
                "Server with errors",
            )
        } else if has_warnings {
            (
                Some(Indicator::dot().color(Color::Warning)),
                "Server with warnings",
            )
        } else if has_other_notifications {
            (
                Some(Indicator::dot().color(Color::Modified)),
                "Server with notifications",
            )
        } else {
            (None, "All Servers Operational")
        };

        let lsp_button = cx.weak_entity();

        div().child(
            PopoverMenu::new("lsp-tool")
                .menu(move |_, cx| {
                    lsp_button
                        .read_with(cx, |lsp_button, _| lsp_button.lsp_menu.clone())
                        .ok()
                        .flatten()
                })
                .anchor(Corner::BottomLeft)
                .with_handle(self.popover_menu_handle.clone())
                .trigger_with_tooltip(
                    IconButton::new("zed-lsp-tool-button", IconName::BoltOutlined)
                        .when_some(indicator, IconButton::indicator)
                        .icon_size(IconSize::Small)
                        .indicator_border_color(Some(cx.theme().colors().status_bar_background)),
                    move |_window, cx| {
                        Tooltip::with_meta("Language Servers", Some(&ToggleMenu), description, cx)
                    },
                ),
        )
    }
}
