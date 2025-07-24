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
use project::{LspStore, LspStoreEvent, Worktree, project_settings::ProjectSettings};
use settings::{Settings as _, SettingsStore};
use ui::{
    Context, ContextMenu, ContextMenuEntry, ContextMenuItem, DocumentationAside, DocumentationSide,
    Indicator, PopoverMenu, PopoverMenuHandle, Tooltip, Window, prelude::*,
};

use workspace::{StatusItemView, Workspace};

use crate::lsp_log::GlobalLogStore;

actions!(
    lsp_tool,
    [
        /// Toggles the language server tool menu.
        ToggleMenu
    ]
);

pub struct LspTool {
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

#[derive(Debug)]
struct ServerInfo {
    name: LanguageServerName,
    id: Option<LanguageServerId>,
    health: Option<ServerHealth>,
    binary_status: Option<LanguageServerBinaryStatus>,
    message: Option<SharedString>,
}

impl ServerInfo {
    fn server_selector(&self) -> LanguageServerSelector {
        self.id
            .map(LanguageServerSelector::Id)
            .unwrap_or_else(|| LanguageServerSelector::Name(self.name.clone()))
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
        menu = menu.align_popover_bottom();
        let lsp_logs = cx
            .try_global::<GlobalLogStore>()
            .and_then(|lsp_logs| lsp_logs.0.upgrade());
        let lsp_store = self.lsp_store.upgrade();
        let Some((lsp_logs, lsp_store)) = lsp_logs.zip(lsp_store) else {
            return menu;
        };

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
            // TODO currently, Zed remote does not work well with the LSP logs
            // https://github.com/zed-industries/zed/issues/28557
            let has_logs = lsp_store.read(cx).as_local().is_some()
                && lsp_logs.read(cx).has_server_logs(&server_selector);

            let status_color = server_info
                .binary_status
                .as_ref()
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
                    Some(match server_info.health? {
                        ServerHealth::Ok => Color::Success,
                        ServerHealth::Warning => Color::Warning,
                        ServerHealth::Error => Color::Error,
                    })
                })
                .unwrap_or(Color::Success);

            let message = server_info
                .message
                .as_ref()
                .or_else(|| server_info.binary_status.as_ref()?.message.as_ref())
                .cloned();
            let hover_label = if has_logs {
                Some("View Logs")
            } else if message.is_some() {
                Some("View Message")
            } else {
                None
            };

            let server_name = server_info.name.clone();
            menu = menu.item(ContextMenuItem::custom_entry(
                move |_, _| {
                    h_flex()
                        .group("menu_item")
                        .w_full()
                        .gap_2()
                        .justify_between()
                        .child(
                            h_flex()
                                .gap_2()
                                .child(Indicator::dot().color(status_color))
                                .child(Label::new(server_name.0.clone())),
                        )
                        .when_some(hover_label, |div, hover_label| {
                            div.child(
                                h_flex()
                                    .visible_on_hover("menu_item")
                                    .child(
                                        Label::new(hover_label)
                                            .size(LabelSize::Small)
                                            .color(Color::Muted),
                                    )
                                    .child(
                                        Icon::new(IconName::ChevronRight)
                                            .size(IconSize::Small)
                                            .color(Color::Muted),
                                    ),
                            )
                        })
                        .into_any_element()
                },
                {
                    let lsp_logs = lsp_logs.clone();
                    let message = message.clone();
                    let server_selector = server_selector.clone();
                    let server_name = server_info.name.clone();
                    let workspace = self.workspace.clone();
                    move |window, cx| {
                        if has_logs {
                            lsp_logs.update(cx, |lsp_logs, cx| {
                                lsp_logs.open_server_trace(
                                    workspace.clone(),
                                    server_selector.clone(),
                                    window,
                                    cx,
                                );
                            });
                        } else if let Some(message) = &message {
                            let Some(create_buffer) = workspace
                                .update(cx, |workspace, cx| {
                                    workspace
                                        .project()
                                        .update(cx, |project, cx| project.create_buffer(cx))
                                })
                                .ok()
                            else {
                                return;
                            };

                            let window = window.window_handle();
                            let workspace = workspace.clone();
                            let message = message.clone();
                            let server_name = server_name.clone();
                            cx.spawn(async move |cx| {
                                let buffer = create_buffer.await?;
                                buffer.update(cx, |buffer, cx| {
                                    buffer.edit(
                                        [(
                                            0..0,
                                            format!("Language server {server_name}:\n\n{message}"),
                                        )],
                                        None,
                                        cx,
                                    );
                                    buffer.set_capability(language::Capability::ReadOnly, cx);
                                })?;

                                workspace.update(cx, |workspace, cx| {
                                    window.update(cx, |_, window, cx| {
                                        workspace.add_item_to_active_pane(
                                            Box::new(cx.new(|cx| {
                                                let mut editor =
                                                    Editor::for_buffer(buffer, None, window, cx);
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
                        } else {
                            cx.propagate();
                            return;
                        }
                    }
                },
                message.map(|server_message| {
                    DocumentationAside::new(
                        DocumentationSide::Right,
                        Rc::new(move |_| Label::new(server_message.clone()).into_any_element()),
                    )
                }),
            ));
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
        server_id: Option<LanguageServerId>,
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
        server_id: Option<LanguageServerId>,
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
                id: Some(*server_id),
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

impl LspTool {
    pub fn new(
        workspace: &Workspace,
        popover_menu_handle: PopoverMenuHandle<ContextMenu>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let settings_subscription =
            cx.observe_global_in::<SettingsStore>(window, move |lsp_tool, window, cx| {
                if ProjectSettings::get_global(cx).global_lsp_settings.button {
                    if lsp_tool.lsp_menu.is_none() {
                        lsp_tool.refresh_lsp_menu(true, window, cx);
                        return;
                    }
                } else if lsp_tool.lsp_menu.take().is_some() {
                    cx.notify();
                }
            });

        let lsp_store = workspace.project().read(cx).lsp_store();
        let lsp_store_subscription =
            cx.subscribe_in(&lsp_store, window, |lsp_tool, _, e, window, cx| {
                lsp_tool.on_lsp_store_event(e, window, cx)
            });

        let state = cx.new(|_| LanguageServerState {
            workspace: workspace.weak_handle(),
            items: Vec::new(),
            lsp_store: lsp_store.downgrade(),
            active_editor: None,
            language_servers: LanguageServers::default(),
        });

        Self {
            server_state: state,
            popover_menu_handle,
            lsp_menu: None,
            lsp_menu_refresh: Task::ready(()),
            _subscriptions: vec![settings_subscription, lsp_store_subscription],
        }
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

            let mut servers_per_worktree = BTreeMap::<SharedString, Vec<ServerData>>::new();
            let mut servers_without_worktree = Vec::<ServerData>::new();
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
                    worktree.map(|worktree| SharedString::new(worktree.read(cx).root_name()));

                let binary_status = state.language_servers.binary_statuses.get(&health.name);
                let server_data = ServerData::WithHealthCheck {
                    server_id: *server_id,
                    health,
                    binary_status,
                };
                match worktree_name {
                    Some(worktree_name) => servers_per_worktree
                        .entry(worktree_name.clone())
                        .or_default()
                        .push(server_data),
                    None => servers_without_worktree.push(server_data),
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

                match server_names_to_worktrees.get(server_name) {
                    Some(worktrees_for_name) => {
                        match worktrees_for_name
                            .iter()
                            .find(|(worktree, _)| active_worktrees.contains(worktree))
                            .or_else(|| worktrees_for_name.iter().next())
                        {
                            Some((worktree, server_id)) => {
                                let worktree_name =
                                    SharedString::new(worktree.read(cx).root_name());
                                servers_per_worktree
                                    .entry(worktree_name.clone())
                                    .or_default()
                                    .push(ServerData::WithBinaryStatus {
                                        server_name,
                                        binary_status,
                                        server_id: Some(*server_id),
                                    });
                            }
                            None => servers_without_worktree.push(ServerData::WithBinaryStatus {
                                server_name,
                                binary_status,
                                server_id: None,
                            }),
                        }
                    }
                    None => servers_without_worktree.push(ServerData::WithBinaryStatus {
                        server_name,
                        binary_status,
                        server_id: None,
                    }),
                }
            }

            let mut new_lsp_items =
                Vec::with_capacity(servers_per_worktree.len() + servers_without_worktree.len() + 2);
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
            if !servers_without_worktree.is_empty() {
                new_lsp_items.push(LspMenuItem::Header {
                    header: Some(SharedString::from("Unknown worktree")),
                    separator: false,
                });
                new_lsp_items.extend(
                    servers_without_worktree
                        .into_iter()
                        .map(ServerData::into_lsp_item),
                );
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
            self.lsp_menu_refresh = cx.spawn_in(window, async move |lsp_tool, cx| {
                cx.background_executor()
                    .timer(Duration::from_millis(30))
                    .await;
                lsp_tool
                    .update_in(cx, |lsp_tool, window, cx| {
                        lsp_tool.regenerate_items(cx);
                        let menu = ContextMenu::build(window, cx, |menu, _, cx| {
                            state.update(cx, |state, cx| state.fill_menu(menu, cx))
                        });
                        lsp_tool.lsp_menu = Some(menu.clone());
                        lsp_tool.popover_menu_handle.refresh_menu(
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

impl StatusItemView for LspTool {
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
                        |lsp_tool, _, e: &EditorEvent, window, cx| match e {
                            EditorEvent::ExcerptsAdded { buffer, .. } => {
                                let updated = lsp_tool.server_state.update(cx, |state, cx| {
                                    if let Some(active_editor) = state.active_editor.as_mut() {
                                        let buffer_id = buffer.read(cx).remote_id();
                                        active_editor.editor_buffers.insert(buffer_id)
                                    } else {
                                        false
                                    }
                                });
                                if updated {
                                    lsp_tool.refresh_lsp_menu(false, window, cx);
                                }
                            }
                            EditorEvent::ExcerptsRemoved {
                                removed_buffer_ids, ..
                            } => {
                                let removed = lsp_tool.server_state.update(cx, |state, _| {
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
                                    lsp_tool.refresh_lsp_menu(false, window, cx);
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

impl Render for LspTool {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl ui::IntoElement {
        if self.server_state.read(cx).language_servers.is_empty() || self.lsp_menu.is_none() {
            return div();
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

        let lsp_tool = cx.entity().clone();

        div().child(
            PopoverMenu::new("lsp-tool")
                .menu(move |_, cx| lsp_tool.read(cx).lsp_menu.clone())
                .anchor(Corner::BottomLeft)
                .with_handle(self.popover_menu_handle.clone())
                .trigger_with_tooltip(
                    IconButton::new("zed-lsp-tool-button", IconName::BoltFilledAlt)
                        .when_some(indicator, IconButton::indicator)
                        .icon_size(IconSize::Small)
                        .indicator_border_color(Some(cx.theme().colors().status_bar_background)),
                    move |window, cx| {
                        Tooltip::with_meta(
                            "Language Servers",
                            Some(&ToggleMenu),
                            description,
                            window,
                            cx,
                        )
                    },
                ),
        )
    }
}
