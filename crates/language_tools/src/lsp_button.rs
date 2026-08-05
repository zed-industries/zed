use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap},
    path::{Path, PathBuf},
    rc::Rc,
    time::{Duration, Instant},
};

use sysinfo::{Pid, ProcessRefreshKind, RefreshKind, System};

use language::language_settings::{EditPredictionProvider, all_language_settings};

use client::proto;
use collections::HashSet;
use editor::{Editor, EditorEvent};
use gpui::{Action as _, Anchor, App, Entity, Subscription, Task, TaskExt, WeakEntity, actions};
use language::{BinaryStatus, BufferId, ServerHealth};
use lsp::{LanguageServerId, LanguageServerName, LanguageServerSelector};
use project::{
    LspStore, LspStoreEvent, Worktree, WorktreeId, lsp_store::log_store::GlobalLogStore,
    project_settings::ProjectSettings, trusted_worktrees::TrustedWorktrees,
    worktree_store::WorktreeStore,
};
use settings::{Settings as _, SettingsStore};
use ui::{
    ContextMenu, ContextMenuEntry, Indicator, PopoverMenu, PopoverMenuHandle, Tooltip, prelude::*,
};

use util::paths::PathExt;
use workspace::{StatusItemView, ToggleWorktreeSecurity, Workspace};

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

struct LanguageServerState {
    items: Vec<LspMenuItem>,
    workspace: WeakEntity<Workspace>,
    lsp_store: WeakEntity<LspStore>,
    active_editor: Option<ActiveEditor>,
    language_servers: LanguageServers,
    process_memory_cache: Rc<RefCell<ProcessMemoryCache>>,
}

impl std::fmt::Debug for LanguageServerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LanguageServerState")
            .field("items", &self.items)
            .field("workspace", &self.workspace)
            .field("lsp_store", &self.lsp_store)
            .field("active_editor", &self.active_editor)
            .field("language_servers", &self.language_servers)
            .finish_non_exhaustive()
    }
}

const PROCESS_MEMORY_CACHE_DURATION: Duration = Duration::from_secs(5);

struct ProcessMemoryCache {
    system: System,
    memory_usage: HashMap<u32, u64>,
    last_refresh: Option<Instant>,
}

impl ProcessMemoryCache {
    fn new() -> Self {
        Self {
            system: System::new(),
            memory_usage: HashMap::new(),
            last_refresh: None,
        }
    }

    fn get_memory_usage(&mut self, process_id: u32) -> u64 {
        let cache_expired = self
            .last_refresh
            .map(|last| last.elapsed() >= PROCESS_MEMORY_CACHE_DURATION)
            .unwrap_or(true);

        if cache_expired {
            let refresh_kind = RefreshKind::nothing()
                .with_processes(ProcessRefreshKind::nothing().without_tasks().with_memory());
            self.system.refresh_specifics(refresh_kind);
            self.memory_usage.clear();
            self.last_refresh = Some(Instant::now());
        }

        if let Some(&memory) = self.memory_usage.get(&process_id) {
            return memory;
        }

        let root_pid = Pid::from_u32(process_id);

        let parent_map: HashMap<Pid, Pid> = self
            .system
            .processes()
            .iter()
            .filter_map(|(&pid, process)| Some((pid, process.parent()?)))
            .collect();

        let total_memory = self
            .system
            .processes()
            .iter()
            .filter(|(pid, _)| self.is_descendant_of(**pid, root_pid, &parent_map))
            .map(|(_, process)| process.memory())
            .sum();

        self.memory_usage.insert(process_id, total_memory);
        total_memory
    }

    fn is_descendant_of(&self, pid: Pid, root_pid: Pid, parent_map: &HashMap<Pid, Pid>) -> bool {
        let mut current = pid;
        let mut visited = HashSet::default();
        while current != root_pid {
            if !visited.insert(current) {
                return false;
            }
            match parent_map.get(&current) {
                Some(&parent) => current = parent,
                None => return false,
            }
        }
        true
    }
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
    binary_statuses: HashMap<LanguageServerId, LanguageServerBinaryStatus>,
    servers_per_buffer_abs_path: HashMap<PathBuf, ServersForPath>,
    last_known_location_by_server_id: HashMap<LanguageServerId, LastKnownServerLocation>,
}

#[derive(Debug, Clone)]
struct LastKnownServerLocation {
    worktree_store: Entity<WorktreeStore>,
    worktree_id: WorktreeId,
    server_id: LanguageServerId,
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
    name: LanguageServerName,
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

        let is_restricted = self
            .workspace
            .upgrade()
            .map(|workspace| {
                let worktree_store = workspace.read(cx).project().read(cx).worktree_store();
                TrustedWorktrees::has_restricted_worktrees(&worktree_store, cx)
            })
            .unwrap_or(false);

        if is_restricted {
            menu = menu.custom_entry(
                move |_window, _cx| {
                    v_flex()
                        .child(
                            h_flex()
                                .gap_1()
                                .child(
                                    Icon::new(IconName::Warning)
                                        .color(Color::Warning)
                                        .size(IconSize::XSmall),
                                )
                                .child(
                                    Label::new("Project is in Restricted Mode")
                                        .size(LabelSize::Small),
                                ),
                        )
                        .child(
                            Label::new("Language Servers can't run until you trust this project.")
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        )
                        .into_any_element()
                },
                move |window, cx| {
                    window.dispatch_action(ToggleWorktreeSecurity.boxed_clone(), cx);
                },
            );
        }

        let server_metadata = self
            .lsp_store
            .update(cx, |lsp_store, _| {
                lsp_store
                    .language_server_statuses()
                    .map(|(server_id, status)| {
                        (
                            server_id,
                            (
                                status.server_readable_version.clone(),
                                status.binary.as_ref().map(|b| b.path.clone()),
                                status.process_id,
                            ),
                        )
                    })
                    .collect::<HashMap<_, _>>()
            })
            .unwrap_or_default();

        let process_memory_cache = self.process_memory_cache.clone();

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
                                    lsp_store.restart_all_language_servers(cx);
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
            let server_worktree_id = self
                .language_servers
                .last_known_location_by_server_id
                .get(&server_info.id)
                .map(|location| location.worktree_id);
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

            let (server_version, binary_path, process_id) = server_metadata
                .get(&server_info.id)
                .map(|(version, path, process_id)| {
                    (
                        version.clone(),
                        path.as_ref()
                            .map(|p| SharedString::from(p.compact().to_string_lossy().to_string())),
                        *process_id,
                    )
                })
                .unwrap_or((None, None, None));

            let server_message = message.clone();

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
                    let can_stop = submenu_server_info.can_stop();
                    let process_memory_cache = process_memory_cache.clone();

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
                                lsp_log_view::open(
                                    &lsp_logs_for_debug,
                                    workspace_for_debug.clone(),
                                    server_selector_for_debug.clone(),
                                    window,
                                    cx,
                                );
                            });
                        }

                        let workspace_for_restart = workspace.clone();
                        let lsp_store_for_restart = lsp_store.clone();
                        let server_name_for_restart = submenu_server_name.clone();
                        let server_worktree_id_for_restart = server_worktree_id;
                        submenu = submenu.entry("Restart Server", None, move |_window, cx| {
                            let Some(workspace) = workspace_for_restart.upgrade() else {
                                return;
                            };
                            let Some(server_worktree_id) = server_worktree_id_for_restart else {
                                return;
                            };

                            let project = workspace.read(cx).project().clone();
                            let buffer_store = project.read(cx).buffer_store().clone();
                            // Source buffers from the project's live buffer store, the same
                            // way `restart_all_language_servers` does in lsp_store.rs — not
                            // from `servers_per_buffer_abs_path`, which `remove_server` prunes
                            // for exactly the server being restarted, making that cache empty
                            // in precisely the case this button needs to handle.
                            let buffers = buffer_store
                                .read(cx)
                                .buffers()
                                .filter(|buffer| {
                                    buffer.read(cx).file().is_some_and(|file| {
                                        file.worktree_id(cx) == server_worktree_id
                                    })
                                })
                                .collect::<Vec<_>>();

                            if !buffers.is_empty() {
                                lsp_store_for_restart
                                    .update(cx, |lsp_store, cx| {
                                        lsp_store.restart_language_servers_for_buffers(
                                            buffers,
                                            HashSet::from_iter([LanguageServerSelector::Name(
                                                server_name_for_restart.clone(),
                                            )]),
                                            true,
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
                            let binary_path = binary_path.clone();
                            let server_version = server_version.clone();
                            let server_message = server_message.clone();
                            let process_memory_cache = process_memory_cache.clone();
                            move |_, cx| {
                                let memory_usage = process_id.map(|pid| {
                                    process_memory_cache.borrow_mut().get_memory_usage(pid)
                                });

                                let memory_label = memory_usage.map(|bytes| {
                                    if bytes >= 1024 * 1024 * 1024 {
                                        format!(
                                            "{:.1} GB",
                                            bytes as f64 / (1024.0 * 1024.0 * 1024.0)
                                        )
                                    } else {
                                        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
                                    }
                                });

                                let version_label =
                                    server_version.as_ref().map(|v| format!("v{}", v.as_ref()));

                                let separator_color =
                                    cx.theme().colors().icon_disabled.opacity(0.8);

                                v_flex()
                                    .id("metadata-container")
                                    .gap_1()
                                    .when_some(server_message.as_ref(), |this, _| {
                                        this.w(rems_from_px(240.))
                                    })
                                    .child(
                                        h_flex()
                                            .ml_neg_1()
                                            .gap_1()
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
                                            .when_some(version_label.as_ref(), |row, version| {
                                                row.child(
                                                    Icon::new(IconName::Dash)
                                                        .color(Color::Custom(separator_color))
                                                        .size(IconSize::XSmall),
                                                )
                                                .child(
                                                    Label::new(version)
                                                        .size(LabelSize::Small)
                                                        .color(Color::Muted),
                                                )
                                            })
                                            .when_some(memory_label.as_ref(), |row, memory| {
                                                row.child(
                                                    Icon::new(IconName::Dash)
                                                        .color(Color::Custom(separator_color))
                                                        .size(IconSize::XSmall),
                                                )
                                                .child(
                                                    Label::new(memory)
                                                        .size(LabelSize::Small)
                                                        .color(Color::Muted),
                                                )
                                            }),
                                    )
                                    .when_some(server_message.clone(), |container, message| {
                                        container.child(
                                            Label::new(message)
                                                .color(Color::Muted)
                                                .size(LabelSize::Small),
                                        )
                                    })
                                    .when_some(binary_path.clone(), |el, path| {
                                        el.tooltip(Tooltip::text(path))
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
    fn update_server_location(
        &mut self,
        language_server_id: LanguageServerId,
        server_name: &LanguageServerName,
        worktree_store: Entity<WorktreeStore>,
        worktree_id: WorktreeId,
    ) {
        let stale_server_ids = self
            .binary_statuses
            .iter()
            .filter_map(|(existing_server_id, status)| {
                if *existing_server_id == language_server_id || status.name != *server_name {
                    return None;
                }
                let location = self
                    .last_known_location_by_server_id
                    .get(existing_server_id)?;
                (location.worktree_store == worktree_store && location.worktree_id == worktree_id)
                    .then_some(*existing_server_id)
            })
            .collect::<Vec<_>>();
        for stale_server_id in stale_server_ids {
            self.binary_statuses.remove(&stale_server_id);
            self.last_known_location_by_server_id
                .remove(&stale_server_id);
        }
        self.last_known_location_by_server_id.insert(
            language_server_id,
            LastKnownServerLocation {
                worktree_store,
                worktree_id,
                server_id: language_server_id,
            },
        );
    }

    fn update_binary_status(
        &mut self,
        language_server_id: LanguageServerId,
        binary_status: BinaryStatus,
        message: Option<&str>,
        name: LanguageServerName,
    ) {
        let binary_status_message = message.map(SharedString::new);
        if matches!(
            binary_status,
            BinaryStatus::Stopped | BinaryStatus::Failed { .. }
        ) {
            self.health_statuses.remove(&language_server_id);
        }
        self.binary_statuses.insert(
            language_server_id,
            LanguageServerBinaryStatus {
                name,
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

    fn remove_server(&mut self, server_id: LanguageServerId) {
        self.health_statuses.remove(&server_id);
        self.servers_per_buffer_abs_path
            .retain(|_, servers_for_path| {
                servers_for_path.servers.remove(&server_id);
                !servers_for_path.servers.is_empty()
            });
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
        for (language_server_id, status) in lsp_store.read(cx).language_server_statuses() {
            language_servers.binary_statuses.insert(
                language_server_id,
                LanguageServerBinaryStatus {
                    name: status.name.clone(),
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
            process_memory_cache: Rc::new(RefCell::new(ProcessMemoryCache::new())),
        });

        let mut lsp_button = Self {
            server_state,
            popover_menu_handle,
            lsp_menu: None,
            lsp_menu_refresh: Task::ready(()),
            _subscriptions: vec![settings_subscription, lsp_store_subscription],
        };
        let is_restricted = TrustedWorktrees::has_restricted_worktrees(
            &workspace.project().read(cx).worktree_store(),
            cx,
        );

        if is_restricted
            || !lsp_button
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
                        self.server_state.update(cx, |state, cx| {
                            if let Some(lsp_store) = state.lsp_store.upgrade() {
                                let lsp_store = lsp_store.read(cx);
                                if let Some(worktree_id) =
                                    lsp_store.language_server_worktree_id(*language_server_id)
                                {
                                    state.language_servers.update_server_location(
                                        *language_server_id,
                                        name,
                                        lsp_store.worktree_store(),
                                        worktree_id,
                                    );
                                }
                            }
                            state.language_servers.update_binary_status(
                                *language_server_id,
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
            LspStoreEvent::LanguageServerAdded(
                language_server_id,
                server_name,
                Some(worktree_id),
            ) => {
                self.server_state.update(cx, |state, cx| {
                    let Some(lsp_store) = state.lsp_store.upgrade() else {
                        return;
                    };
                    let worktree_store = lsp_store.read(cx).worktree_store();
                    state.language_servers.update_server_location(
                        *language_server_id,
                        server_name,
                        worktree_store,
                        *worktree_id,
                    );
                });
                updated = true;
            }
            LspStoreEvent::LanguageServerRemoved(server_id) => {
                self.server_state.update(cx, |state, _| {
                    state.language_servers.remove_server(*server_id);
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

            let worktree_store = state
                .lsp_store
                .upgrade()
                .map(|lsp_store| lsp_store.read(cx).worktree_store());

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

            if let Some(worktree_store) = &worktree_store {
                for (server_id, worktree) in &server_ids_to_worktrees {
                    state
                        .language_servers
                        .last_known_location_by_server_id
                        .insert(
                            *server_id,
                            LastKnownServerLocation {
                                worktree_store: worktree_store.clone(),
                                worktree_id: worktree.read(cx).id(),
                                server_id: *server_id,
                            },
                        );
                }
            }

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
                servers_with_health_checks.insert(*server_id);
                let worktree_name =
                    worktree.map(|worktree| SharedString::new(worktree.read(cx).root_name_str()));

                let binary_status = state.language_servers.binary_statuses.get(server_id);
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

            for (server_id, binary_status) in state
                .language_servers
                .binary_statuses
                .iter()
                .filter(|(server_id, _)| !servers_with_health_checks.contains(server_id))
            {
                let live_location = server_ids_to_worktrees.get(server_id).map(|worktree| {
                    (
                        SharedString::new(worktree.read(cx).root_name_str()),
                        *server_id,
                    )
                });

                let location = live_location.or_else(|| {
                    let location = state
                        .language_servers
                        .last_known_location_by_server_id
                        .get(server_id)?;

                    if worktree_store.as_ref() != Some(&location.worktree_store) {
                        return None;
                    }

                    let worktree = location
                        .worktree_store
                        .read(cx)
                        .worktree_for_id(location.worktree_id, cx)?;

                    Some((
                        SharedString::new(worktree.read(cx).root_name_str()),
                        location.server_id,
                    ))
                });

                if let Some((worktree_name, server_id)) = location {
                    servers_per_worktree.entry(worktree_name).or_default().push(
                        ServerData::WithBinaryStatus {
                            server_name: &binary_status.name,
                            binary_status,
                            server_id,
                        },
                    );
                }
            }

            let mut can_stop_all = false;
            let mut can_restart_all = true;

            for server_data in servers_per_worktree.values().flatten() {
                match server_data {
                    ServerData::WithBinaryStatus { binary_status, .. } => {
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
                    }
                    ServerData::WithHealthCheck { .. } => {
                        can_stop_all = true;
                        can_restart_all = false;
                    }
                };
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
                    let editor_buffers = HashSet::from_iter(
                        editor
                            .read(cx)
                            .buffer()
                            .read(cx)
                            .snapshot(cx)
                            .excerpts()
                            .map(|excerpt| excerpt.context.start.buffer_id),
                    );
                    let _editor_subscription = cx.subscribe_in(
                        &editor,
                        window,
                        |lsp_button, _, e: &EditorEvent, window, cx| match e {
                            EditorEvent::BufferRangesUpdated { buffer, .. } => {
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
                            EditorEvent::BuffersRemoved { removed_buffer_ids } => {
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

    fn hide_setting(&self, _: &App) -> Option<workspace::HideStatusItem> {
        Some(workspace::HideStatusItem::new(|settings| {
            settings.global_lsp_settings.get_or_insert_default().button = Some(false);
        }))
    }
}

impl Render for LspButton {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl ui::IntoElement {
        let is_restricted = self
            .server_state
            .read(cx)
            .workspace
            .upgrade()
            .map(|workspace| {
                let worktree_store = workspace.read(cx).project().read(cx).worktree_store();
                TrustedWorktrees::has_restricted_worktrees(&worktree_store, cx)
            })
            .unwrap_or(false);

        if !is_restricted
            && (self.server_state.read(cx).language_servers.is_empty() || self.lsp_menu.is_none())
        {
            return div().hidden();
        }

        let state = self.server_state.read(cx);
        let is_via_ssh = state
            .workspace
            .upgrade()
            .map(|workspace| workspace.read(cx).project().read(cx).is_via_remote_server())
            .unwrap_or(false);

        let mut has_errors = false;
        let mut has_warnings = false;
        let mut has_other_notifications = false;
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

        let (indicator, description) = if is_restricted {
            (
                Some(Indicator::dot().color(Color::Warning)),
                "Restricted Mode",
            )
        } else if has_errors {
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
                .on_open(Rc::new(move |_window, cx| {
                    let copilot_enabled = all_language_settings(None, cx).edit_predictions.provider
                        == EditPredictionProvider::Copilot;
                    telemetry::event!(
                        "Toolbar Menu Opened",
                        name = "Language Servers",
                        copilot_enabled,
                        is_via_ssh,
                    );
                }))
                .menu(move |_, cx| {
                    lsp_button
                        .read_with(cx, |lsp_button, _| lsp_button.lsp_menu.clone())
                        .ok()
                        .flatten()
                })
                .anchor(Anchor::BottomLeft)
                .with_handle(self.popover_menu_handle.clone())
                .trigger_with_tooltip(
                    IconButton::new("zed-lsp-tool-button", IconName::BoltOutlined)
                        .when_some(indicator, IconButton::indicator)
                        .icon_size(IconSize::Small)
                        .tab_index(0isize)
                        .aria_label("Language Servers")
                        .when(is_restricted, |s| s.icon_color(Color::Warning))
                        .indicator_border_color(Some(cx.theme().colors().status_bar_background)),
                    move |_window, cx| {
                        Tooltip::with_meta("Language Servers", Some(&ToggleMenu), description, cx)
                    },
                ),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc, sync::Arc};

    use futures::{FutureExt, StreamExt, future};
    use gpui::{Entity, TestAppContext};
    use language::{
        FakeLspAdapter, Language, LanguageConfig, LanguageMatcher, LanguageRegistry,
        tree_sitter_rust,
    };
    use project::{FakeFs, Project, lsp_store::log_store::LogStore};
    use serde_json::json;
    use util::path;

    use super::*;

    struct StatusRecorder {
        _subscriptions: Vec<Subscription>,
    }

    fn init_test(cx: &mut TestAppContext) {
        zlog::init_test();
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            release_channel::init(semver::Version::new(0, 0, 0), cx);
        });
    }

    async fn test_project_with_lsp(
        root_path: &'static str,
        server_name: &'static str,
        cx: &mut TestAppContext,
    ) -> (Entity<Project>, Entity<LspButton>, lsp::FakeLanguageServer) {
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            Path::new(root_path),
            json!({
                "test.rs": "",
            }),
        )
        .await;

        let project = Project::test(fs, [Path::new(root_path)], cx).await;
        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        language_registry.add(Arc::new(Language::new(
            LanguageConfig {
                name: "Rust".into(),
                matcher: (LanguageMatcher {
                    path_suffixes: vec!["rs".to_string()],
                    ..Default::default()
                })
                .into(),
                ..Default::default()
            },
            Some(tree_sitter_rust::LANGUAGE.into()),
        )));

        let mut fake_rust_server = language_registry.register_fake_lsp(
            "Rust",
            FakeLspAdapter {
                name: server_name,
                ..Default::default()
            },
        );

        let log_store = cx.new(|cx| LogStore::new(false, cx));
        log_store.update(cx, |store, cx| store.add_project(&project, cx));

        let _rust_buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer_with_lsp(&Path::new(root_path).join("test.rs"), cx)
            })
            .await
            .expect("opening the test buffer should succeed");

        let mut language_server = fake_rust_server
            .next()
            .await
            .expect("opening the buffer should start a language server");
        language_server
            .receive_notification::<lsp::notification::DidOpenTextDocument>()
            .await;

        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let lsp_button = workspace.update_in(cx, |workspace, window, cx| {
            cx.new(|cx| LspButton::new(workspace, PopoverMenuHandle::default(), window, cx))
        });

        (project, lsp_button, language_server)
    }

    fn has_server(
        button: &Entity<LspButton>,
        name: &LanguageServerName,
        cx: &TestAppContext,
    ) -> bool {
        button.read_with(cx, |button, cx| {
            button.server_state.read(cx).items.iter().any(|item| {
                item.server_info()
                    .is_some_and(|server| server.name == *name)
            })
        })
    }

    fn binary_status(
        button: &Entity<LspButton>,
        name: &LanguageServerName,
        cx: &TestAppContext,
    ) -> Option<BinaryStatus> {
        button.read_with(cx, |button, cx| {
            button
                .server_state
                .read(cx)
                .language_servers
                .binary_statuses
                .values()
                .find(|status| status.name == *name)
                .map(|status| status.status.clone())
        })
    }

    #[gpui::test]
    async fn stopped_servers_remain_in_the_menu(cx: &mut TestAppContext) {
        init_test(cx);
        let (project, lsp_button, mut language_server) =
            test_project_with_lsp(path!("/the-root"), "the-rust-language-server", cx).await;
        let server_name = language_server.server.name();

        lsp_button.update(cx, |button, cx| button.regenerate_items(cx));
        assert!(has_server(&lsp_button, &server_name, cx));

        let mut shutdown_requests = language_server
            .set_request_handler::<lsp::request::Shutdown, _, _>(|_, _| future::ready(Ok(())));
        project.update(cx, |project, cx| {
            project
                .lsp_store()
                .update(cx, |lsp_store, cx| lsp_store.stop_all_language_servers(cx));
        });
        shutdown_requests
            .next()
            .await
            .expect("stopping the server should send a shutdown request");
        language_server
            .receive_notification::<lsp::notification::Exit>()
            .await;
        cx.run_until_parked();

        lsp_button.update(cx, |button, cx| button.regenerate_items(cx));
        assert!(has_server(&lsp_button, &server_name, cx));
    }

    #[gpui::test]
    async fn status_from_another_workspace_is_not_shown(cx: &mut TestAppContext) {
        init_test(cx);
        let (_, first_button, first_server) =
            test_project_with_lsp(path!("/first-root"), "first-language-server", cx).await;
        let (_, second_button, second_server) =
            test_project_with_lsp(path!("/second-root"), "second-language-server", cx).await;
        let first_server_name = first_server.server.name();
        let second_server_name = second_server.server.name();

        first_button.update(cx, |button, cx| button.regenerate_items(cx));
        second_button.update(cx, |button, cx| button.regenerate_items(cx));
        assert!(has_server(&first_button, &first_server_name, cx));
        assert!(has_server(&second_button, &second_server_name, cx));

        second_button.update(cx, |button, cx| {
            button.server_state.update(cx, |state, _| {
                state.language_servers.update_binary_status(
                    LanguageServerId(usize::MAX),
                    BinaryStatus::Stopped,
                    None,
                    first_server_name.clone(),
                );
            });
        });

        second_button.update(cx, |button, cx| button.regenerate_items(cx));
        assert!(!has_server(&second_button, &first_server_name, cx));
        assert!(has_server(&second_button, &second_server_name, cx));
    }

    #[gpui::test]
    async fn server_status_is_only_emitted_by_its_originating_lsp_store(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            Path::new("/"),
            json!({
                "first-root": {},
                "second-root": {},
            }),
        )
        .await;
        let language_registry = Arc::new(LanguageRegistry::test(cx.executor()));
        let first_project = Project::test_with_language_registry(
            fs.clone(),
            [Path::new(path!("/first-root"))],
            language_registry.clone(),
            cx,
        )
        .await;
        let second_project = Project::test_with_language_registry(
            fs,
            [Path::new(path!("/second-root"))],
            language_registry.clone(),
            cx,
        )
        .await;
        let first_lsp_store = first_project.read_with(cx, |project, _| project.lsp_store());
        let second_lsp_store = second_project.read_with(cx, |project, _| project.lsp_store());
        let first_statuses = Rc::new(RefCell::new(Vec::new()));
        let second_statuses = Rc::new(RefCell::new(Vec::new()));
        let _recorder = cx.new(|cx| StatusRecorder {
            _subscriptions: vec![
                cx.subscribe(&first_lsp_store, {
                    let first_statuses = first_statuses.clone();
                    move |_, _, event, _| record_binary_status(event, &first_statuses)
                }),
                cx.subscribe(&second_lsp_store, {
                    let second_statuses = second_statuses.clone();
                    move |_, _, event, _| record_binary_status(event, &second_statuses)
                }),
            ],
        });

        language_registry.update_lsp_binary_status_for_language_server(
            first_lsp_store.entity_id(),
            LanguageServerId(1),
            LanguageServerName("the-rust-language-server".into()),
            BinaryStatus::Stopped,
        );
        cx.run_until_parked();

        assert_eq!(
            first_statuses.borrow().as_slice(),
            &[proto::ServerBinaryStatus::Stopped as i32]
        );
        assert!(second_statuses.borrow().is_empty());

        assert!(second_statuses.borrow().is_empty());
    }

    #[gpui::test]
    async fn restarting_all_servers_only_restarts_the_current_workspace(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            Path::new("/"),
            json!({
                "first-root": { "test.rs": "" },
                "second-root": { "test.rs": "" },
            }),
        )
        .await;
        let language_registry = Arc::new(LanguageRegistry::test(cx.executor()));
        language_registry.add(Arc::new(Language::new(
            LanguageConfig {
                name: "Rust".into(),
                matcher: LanguageMatcher {
                    path_suffixes: vec!["rs".to_string()],
                    ..Default::default()
                }
                .into(),
                ..Default::default()
            },
            Some(tree_sitter_rust::LANGUAGE.into()),
        )));
        let server_name = LanguageServerName("the-rust-language-server".into());
        let mut fake_servers = language_registry.register_fake_lsp(
            "Rust",
            FakeLspAdapter {
                name: "the-rust-language-server",
                ..Default::default()
            },
        );
        let first_project = Project::test_with_language_registry(
            fs.clone(),
            [Path::new(path!("/first-root"))],
            language_registry.clone(),
            cx,
        )
        .await;
        let second_project = Project::test_with_language_registry(
            fs,
            [Path::new(path!("/second-root"))],
            language_registry,
            cx,
        )
        .await;
        let log_store = cx.new(|cx| LogStore::new(false, cx));
        log_store.update(cx, |store, cx| {
            store.add_project(&first_project, cx);
            store.add_project(&second_project, cx);
        });

        let _first_buffer = first_project
            .update(cx, |project, cx| {
                project.open_local_buffer_with_lsp(path!("/first-root/test.rs"), cx)
            })
            .await
            .expect("opening the first buffer should succeed");
        let mut first_server = fake_servers
            .next()
            .await
            .expect("the first workspace should start a server");
        first_server
            .receive_notification::<lsp::notification::DidOpenTextDocument>()
            .await;

        let _second_buffer = second_project
            .update(cx, |project, cx| {
                project.open_local_buffer_with_lsp(path!("/second-root/test.rs"), cx)
            })
            .await
            .expect("opening the second buffer should succeed");
        let mut second_server = fake_servers
            .next()
            .await
            .expect("the second workspace should start a server");
        second_server
            .receive_notification::<lsp::notification::DidOpenTextDocument>()
            .await;

        let (first_workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(first_project.clone(), window, cx));
        let first_button = first_workspace.update_in(cx, |workspace, window, cx| {
            cx.new(|cx| LspButton::new(workspace, PopoverMenuHandle::default(), window, cx))
        });
        let (second_workspace, cx) = cx
            .add_window_view(|window, cx| Workspace::test_new(second_project.clone(), window, cx));
        let second_button = second_workspace.update_in(cx, |workspace, window, cx| {
            cx.new(|cx| LspButton::new(workspace, PopoverMenuHandle::default(), window, cx))
        });
        cx.background_executor
            .timer(Duration::from_millis(30))
            .await;
        cx.run_until_parked();

        let mut first_shutdown = first_server
            .set_request_handler::<lsp::request::Shutdown, _, _>(|_, _| future::ready(Ok(())));
        let mut second_shutdown = second_server
            .set_request_handler::<lsp::request::Shutdown, _, _>(|_, _| future::ready(Ok(())));
        first_project.update(cx, |project, cx| {
            project
                .lsp_store()
                .update(cx, |store, cx| store.stop_all_language_servers(cx));
        });
        second_project.update(cx, |project, cx| {
            project
                .lsp_store()
                .update(cx, |store, cx| store.stop_all_language_servers(cx));
        });
        first_shutdown
            .next()
            .await
            .expect("the first server should receive shutdown");
        second_shutdown
            .next()
            .await
            .expect("the second server should receive shutdown");
        first_server
            .receive_notification::<lsp::notification::Exit>()
            .await;
        second_server
            .receive_notification::<lsp::notification::Exit>()
            .await;
        cx.run_until_parked();

        let first_status = binary_status(&first_button, &server_name, cx);
        let second_status = binary_status(&second_button, &server_name, cx);
        assert!(
            matches!(first_status, Some(BinaryStatus::Stopped)),
            "expected the first server to be stopped, got {first_status:?}"
        );
        assert!(
            matches!(second_status, Some(BinaryStatus::Stopped)),
            "expected the second server to be stopped, got {second_status:?}"
        );

        first_project.update(cx, |project, cx| {
            project
                .lsp_store()
                .update(cx, |store, cx| store.restart_all_language_servers(cx));
        });
        let mut restarted_server = fake_servers
            .next()
            .await
            .expect("the first workspace should restart its server");
        restarted_server
            .receive_notification::<lsp::notification::DidOpenTextDocument>()
            .await;
        cx.run_until_parked();

        let first_lsp_store = first_project.read_with(cx, |project, _| project.lsp_store());
        let second_lsp_store = second_project.read_with(cx, |project, _| project.lsp_store());
        assert_eq!(
            first_lsp_store.read_with(cx, |store, _| store.language_server_statuses().count()),
            1
        );
        assert_eq!(
            second_lsp_store.read_with(cx, |store, _| store.language_server_statuses().count()),
            0
        );
        assert!(matches!(
            binary_status(&first_button, &server_name, cx),
            Some(BinaryStatus::None)
        ));
        assert!(matches!(
            binary_status(&second_button, &server_name, cx),
            Some(BinaryStatus::Stopped)
        ));
    }

    #[gpui::test]
    async fn restarting_a_server_only_restarts_its_worktree(cx: &mut TestAppContext) {
        init_test(cx);
        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            Path::new("/"),
            json!({
                "first-root": { "test.rs": "" },
                "second-root": { "test.rs": "" },
            }),
        )
        .await;
        let project = Project::test(
            fs,
            [
                Path::new(path!("/first-root")),
                Path::new(path!("/second-root")),
            ],
            cx,
        )
        .await;
        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        language_registry.add(Arc::new(Language::new(
            LanguageConfig {
                name: "Rust".into(),
                matcher: LanguageMatcher {
                    path_suffixes: vec!["rs".to_string()],
                    ..Default::default()
                }
                .into(),
                ..Default::default()
            },
            Some(tree_sitter_rust::LANGUAGE.into()),
        )));
        let mut fake_servers = language_registry.register_fake_lsp(
            "Rust",
            FakeLspAdapter {
                name: "the-rust-language-server",
                ..Default::default()
            },
        );

        let (first_buffer, _first_lsp_handle) = project
            .update(cx, |project, cx| {
                project.open_local_buffer_with_lsp(path!("/first-root/test.rs"), cx)
            })
            .await
            .expect("opening the first buffer should succeed");
        let mut first_server = fake_servers
            .next()
            .await
            .expect("the first worktree should start a server");
        first_server
            .receive_notification::<lsp::notification::DidOpenTextDocument>()
            .await;

        let _second_buffer = project
            .update(cx, |project, cx| {
                project.open_local_buffer_with_lsp(path!("/second-root/test.rs"), cx)
            })
            .await
            .expect("opening the second buffer should succeed");
        let mut second_server = fake_servers
            .next()
            .await
            .expect("the second worktree should start a server");
        second_server
            .receive_notification::<lsp::notification::DidOpenTextDocument>()
            .await;
        let first_server_id = first_server.server.server_id();
        let second_server_id = second_server.server.server_id();
        let server_name = first_server.server.name();

        let mut first_shutdown = first_server
            .set_request_handler::<lsp::request::Shutdown, _, _>(|_, _| future::ready(Ok(())));
        let stop_task = project.update(cx, |project, cx| {
            project.lsp_store().update(cx, |store, cx| {
                store.stop_language_servers_for_buffers(
                    Vec::new(),
                    HashSet::from_iter([LanguageServerSelector::Id(first_server_id)]),
                    cx,
                )
            })
        });
        first_shutdown
            .next()
            .await
            .expect("the first server should receive shutdown");
        first_server
            .receive_notification::<lsp::notification::Exit>()
            .await;
        stop_task
            .await
            .expect("stopping the first server should succeed");

        let mut second_shutdown = second_server
            .set_request_handler::<lsp::request::Shutdown, _, _>(|_, _| future::ready(Ok(())));
        project.update(cx, |project, cx| {
            project.lsp_store().update(cx, |store, cx| {
                store.restart_language_servers_for_buffers(
                    vec![first_buffer],
                    HashSet::from_iter([LanguageServerSelector::Name(server_name)]),
                    true,
                    cx,
                );
            });
        });
        let mut restarted_server = fake_servers
            .next()
            .await
            .expect("the first worktree should restart its server");
        restarted_server
            .receive_notification::<lsp::notification::DidOpenTextDocument>()
            .await;
        cx.run_until_parked();

        let lsp_store = project.read_with(cx, |project, _| project.lsp_store());
        let running_server_ids = lsp_store.read_with(cx, |store, _| {
            store
                .language_server_statuses()
                .map(|(server_id, _)| server_id)
                .collect::<HashSet<_>>()
        });
        assert!(running_server_ids.contains(&second_server_id));
        assert!(running_server_ids.contains(&restarted_server.server.server_id()));
        assert_eq!(running_server_ids.len(), 2);
        assert!(second_shutdown.next().now_or_never().is_none());
    }

    fn record_binary_status(event: &LspStoreEvent, statuses: &Rc<RefCell<Vec<i32>>>) {
        if let LspStoreEvent::LanguageServerUpdate {
            message: proto::update_language_server::Variant::StatusUpdate(status),
            ..
        } = event
            && let Some(proto::status_update::Status::Binary(binary_status)) = status.status
        {
            statuses.borrow_mut().push(binary_status);
        }
    }
}
