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
use path::PathStyle;
use project::{
    LspStore, LspStoreEvent, Worktree, lsp_store::log_store::GlobalLogStore,
    project_settings::ProjectSettings, trusted_worktrees::TrustedWorktrees,
};
use settings::{Settings as _, SettingsStore, WorktreeId};
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

#[derive(Default, Clone)]
struct ServerMetadata {
    server_version: Option<SharedString>,
    binary_display_path: Option<SharedString>,
    process_id: Option<u32>,
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

        let path_style = self
            .workspace
            .upgrade()
            .map(|workspace| workspace.read(cx).path_style(cx))
            .unwrap_or(PathStyle::local());

        let server_metadata =
            self.lsp_store
                .update(cx, |lsp_store, _| {
                    lsp_store
                        .language_server_statuses()
                        .map(|(server_id, status)| {
                            (
                                server_id,
                                ServerMetadata {
                                    server_version: status.server_readable_version.clone(),
                                    binary_display_path: status.binary.as_ref().map(|binary| {
                                        tooltip_for_server_binary(binary, path_style)
                                    }),
                                    process_id: status.process_id,
                                },
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
            } else if let LspMenuItem::WithStoppedStatus {
                server_name,
                worktree_id,
            } = item
            {
                let server_name = server_name.clone();
                let worktree_id = *worktree_id;
                let has_logs = lsp_logs
                    .read(cx)
                    .language_server_id_for_name_and_worktree(&server_name, worktree_id)
                    .map_or(false, |id| {
                        lsp_logs
                            .read(cx)
                            .has_server_logs(&LanguageServerSelector::Id(id))
                    });
                let state = cx.entity().downgrade();
                let workspace = self.workspace.clone();
                menu = menu.submenu_with_colored_icon(
                    server_name.0.clone(),
                    IconName::Circle,
                    Color::Disabled,
                    {
                        let state = state.clone();
                        let server_name = server_name.clone();
                        let workspace = workspace.clone();
                        let lsp_logs = lsp_logs.clone();
                        move |menu, _window, _cx| {
                            let mut submenu = menu;
                            let state_for_restart = state.clone();
                            let server_name_for_restart = server_name.clone();
                            submenu = submenu.entry("Restart Server", None, move |_window, cx| {
                                state_for_restart
                                    .update(cx, |state, cx| {
                                        state.restart_server_for_worktree(
                                            server_name_for_restart.clone(),
                                            worktree_id,
                                            cx,
                                        );
                                    })
                                    .ok();
                            });
                            if has_logs {
                                let workspace_for_logs = workspace.clone();
                                let server_name_for_logs = server_name.clone();
                                let lsp_logs_for_logs = lsp_logs.clone();
                                submenu = submenu.entry("View Logs", None, move |window, cx| {
                                    let Some(server_id) = lsp_logs_for_logs
                                        .read(cx)
                                        .language_server_id_for_name_and_worktree(
                                            &server_name_for_logs,
                                            worktree_id,
                                        )
                                    else {
                                        return;
                                    };
                                    lsp_log_view::open(
                                        &lsp_logs_for_logs,
                                        workspace_for_logs.clone(),
                                        LanguageServerSelector::Id(server_id),
                                        window,
                                        cx,
                                    );
                                });
                            }
                            submenu
                        }
                    },
                );
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

            let ServerMetadata {
                server_version,
                binary_display_path,
                process_id,
            } = server_metadata
                .get(&server_info.id)
                .cloned()
                .unwrap_or_default();

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
                    let state = cx.entity().downgrade();
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

                        let state_for_restart = state.clone();
                        let server_name_for_restart = submenu_server_name.clone();
                        submenu = submenu.entry("Restart Server", None, move |_window, cx| {
                            state_for_restart
                                .update(cx, |state, cx| {
                                    state.restart_server_by_name(
                                        server_name_for_restart.clone(),
                                        cx,
                                    );
                                })
                                .ok();
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
                            let binary_display_path = binary_display_path.clone();
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
                                        this.w(rems_from_px(240_f32))
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
                                    .when_some(binary_display_path.clone(), |el, path| {
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

    fn restart_server_by_name(&mut self, server_name: LanguageServerName, cx: &mut App) {
        self.restart_server(server_name, None, cx);
    }

    fn restart_server_for_worktree(
        &mut self,
        server_name: LanguageServerName,
        worktree_id: WorktreeId,
        cx: &mut App,
    ) {
        self.restart_server(server_name, Some(worktree_id), cx);
    }

    fn restart_server(
        &mut self,
        server_name: LanguageServerName,
        worktree_id: Option<WorktreeId>,
        cx: &mut App,
    ) {
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };
        let Some(lsp_store) = self.lsp_store.upgrade() else {
            return;
        };

        let buffers = workspace
            .read(cx)
            .project()
            .read(cx)
            .buffer_store()
            .read(cx)
            .buffers()
            .filter(|buffer| {
                worktree_id.is_none_or(|worktree_id| {
                    buffer
                        .read(cx)
                        .file()
                        .is_none_or(|file| file.worktree_id(cx) == worktree_id)
                }) && buffer.read(cx).language().is_some_and(|language| {
                    lsp_store
                        .read(cx)
                        .languages
                        .lsp_adapters(&language.name())
                        .iter()
                        .any(|adapter| adapter.name() == server_name)
                })
            })
            .collect();

        lsp_store.update(cx, |lsp_store, cx| {
            if let Some(worktree_id) = worktree_id {
                lsp_store.restart_language_server_for_worktree(
                    buffers,
                    server_name,
                    worktree_id,
                    cx,
                );
            } else {
                lsp_store.restart_language_servers_for_buffers(
                    buffers,
                    HashSet::from_iter([LanguageServerSelector::Name(server_name)]),
                    true,
                    cx,
                );
            }
        });
    }
}

fn tooltip_for_server_binary(
    server_binary: &lsp::LanguageServerBinary,
    path_style: PathStyle,
) -> SharedString {
    let runtime = path_style.file_name(&server_binary.path).and_then(|name| {
        ["node", "python"]
            .into_iter()
            .find(|runtime| name.starts_with(runtime))
    });

    let target_path = runtime
        .and_then(|_runtime| {
            server_binary
                .arguments
                .iter()
                .find(|arg| !arg.to_string_lossy().starts_with('-'))
        })
        .map(Path::new)
        .unwrap_or(&server_binary.path);

    let display_path = path_style.normalize(&target_path.compact().to_string_lossy());

    match runtime {
        Some(runtime) => format!("{display_path} ({runtime})").into(),
        None => display_path.into(),
    }
}

impl LanguageServers {
    fn update_binary_status(
        &mut self,
        binary_status: BinaryStatus,
        message: Option<&str>,
        id: LanguageServerId,
    ) {
        let binary_status_message = message.map(SharedString::new);
        if matches!(
            binary_status,
            BinaryStatus::Stopped | BinaryStatus::Failed { .. }
        ) {
            self.health_statuses.remove(&id);
        }
        self.binary_statuses.insert(
            id,
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

    /// Drop all state for a server that has been removed: its id is dead and
    /// every id-keyed record for it is garbage.
    fn remove_server(&mut self, server_id: LanguageServerId) {
        self.health_statuses.remove(&server_id);
        self.binary_statuses.remove(&server_id);
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
    WithStoppedStatus {
        server_name: &'a LanguageServerName,
        worktree_id: WorktreeId,
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
    WithStoppedStatus {
        server_name: LanguageServerName,
        worktree_id: WorktreeId,
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
            Self::WithStoppedStatus { .. } => None,
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
            Self::WithStoppedStatus {
                server_name,
                worktree_id,
            } => LspMenuItem::WithStoppedStatus {
                server_name: server_name.clone(),
                worktree_id,
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
        for (id, _) in lsp_store.read(cx).language_server_statuses() {
            language_servers.binary_statuses.insert(
                id,
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
            || !lsp_store.read(cx).stopped_language_servers().is_empty()
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
                    if let Some(binary_status) =
                        proto::ServerBinaryStatus::try_from(*binary_status).ok()
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
                                *language_server_id,
                            );
                        });
                        updated = true;
                    };
                }
                Some(proto::status_update::Status::Health(health_status)) => {
                    if let Some(health) = proto::ServerHealth::try_from(*health_status).ok() {
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
            let mut server_ids_to_worktrees =
                HashMap::<LanguageServerId, Entity<Worktree>>::default();
            let mut server_ids_to_names =
                HashMap::<LanguageServerId, LanguageServerName>::default();

            for servers_for_path in state.language_servers.servers_per_buffer_abs_path.values() {
                if let Some(worktree) = servers_for_path
                    .worktree
                    .as_ref()
                    .and_then(|worktree| worktree.upgrade())
                {
                    for (server_id, server_name) in &servers_for_path.servers {
                        server_ids_to_worktrees.insert(*server_id, worktree.clone());
                        if let Some(server_name) = server_name {
                            server_ids_to_names.insert(*server_id, server_name.clone());
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
                            server_ids_to_names.insert(server_id, status.name.clone());
                        }
                    }
                })
                .ok();

            let mut servers_per_worktree = BTreeMap::<SharedString, Vec<ServerData>>::new();
            let mut servers_with_health_checks = HashSet::<LanguageServerId>::default();

            for (server_id, health) in &state.language_servers.health_statuses {
                servers_with_health_checks.insert(*server_id);
                let Some(worktree) = server_ids_to_worktrees.get(server_id) else {
                    continue;
                };
                let worktree_name = SharedString::new(worktree.read(cx).root_name_str());
                let binary_status = state.language_servers.binary_statuses.get(server_id);
                servers_per_worktree
                    .entry(worktree_name.clone())
                    .or_default()
                    .push(ServerData::WithHealthCheck {
                        server_id: *server_id,
                        health,
                        binary_status,
                    });
            }

            for (server_id, binary_status) in state
                .language_servers
                .binary_statuses
                .iter()
                .filter(|(id, _)| !servers_with_health_checks.contains(id))
            {
                let Some(worktree) = server_ids_to_worktrees.get(server_id) else {
                    continue;
                };
                let Some(server_name) = server_ids_to_names.get(server_id) else {
                    continue;
                };
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

            let mut can_stop_all = state
                .language_servers
                .health_statuses
                .keys()
                .any(|id| server_ids_to_worktrees.contains_key(id));
            let mut can_restart_all = !can_stop_all;

            for binary_status in state.language_servers.binary_statuses.values() {
                match binary_status.status {
                    BinaryStatus::None => {
                        can_restart_all = false;
                        can_stop_all |= true;
                    }
                    BinaryStatus::CheckingForUpdate
                    | BinaryStatus::Downloading
                    | BinaryStatus::Starting
                    | BinaryStatus::Stopping => {
                        can_restart_all = false;
                        can_stop_all = false;
                        break;
                    }
                    BinaryStatus::Stopped | BinaryStatus::Failed { .. } => {}
                }
            }

            if let Some(lsp_store) = state.lsp_store.upgrade() {
                lsp_store
                    .read(cx)
                    .stopped_language_servers()
                    .iter()
                    .for_each(|(server_name, worktree_ids)| {
                        worktree_ids.iter().for_each(|worktree_id| {
                            if let Some(worktree) = lsp_store
                                .read(cx)
                                .worktree_store()
                                .read(cx)
                                .worktree_for_id(*worktree_id, cx)
                            {
                                let worktree_name =
                                    SharedString::new(worktree.read(cx).root_name_str());
                                servers_per_worktree.entry(worktree_name).or_default().push(
                                    ServerData::WithStoppedStatus {
                                        server_name,
                                        worktree_id: *worktree_id,
                                    },
                                );
                            }
                        });
                    });
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

    fn is_empty(&self, cx: &App) -> bool {
        let lsp_state = self.server_state.read(cx);

        (lsp_state
            .lsp_store
            .upgrade()
            .is_some_and(|lsp_store| lsp_store.read(cx).stopped_language_servers().is_empty())
            && lsp_state.language_servers.is_empty())
            || self.lsp_menu.is_none()
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

        if !is_restricted && self.is_empty(cx) {
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
    use super::*;
    use crate::LspLogView;
    use futures::StreamExt;
    use gpui::TestAppContext;
    use language::{Buffer, FakeLspAdapter, rust_lang};
    use project::{FakeFs, Project, lsp_store};
    use serde_json::json;
    use settings::SettingsStore;
    use util::{ResultExt as _, path};
    use workspace::Workspace;

    fn server_id(n: usize) -> LanguageServerId {
        LanguageServerId(n)
    }

    fn server_name(s: &str) -> LanguageServerName {
        LanguageServerName(s.into())
    }

    fn health_status(name: &str) -> LanguageServerHealthStatus {
        LanguageServerHealthStatus {
            name: server_name(name),
            health: Some((None, ServerHealth::Ok)),
        }
    }

    fn servers_for_path(servers: &[(LanguageServerId, &str)]) -> ServersForPath {
        ServersForPath {
            servers: servers
                .iter()
                .map(|(id, name)| (*id, Some(server_name(name))))
                .collect(),
            worktree: None,
        }
    }

    /// `remove_server` evicts the id from `health_statuses` so a restarted
    /// server's new id renders without inheriting the old one's stale entry.
    /// This is the regression test for #53627.
    #[test]
    fn remove_server_drops_health_entry_for_id() {
        let mut state = LanguageServers::default();
        state
            .health_statuses
            .insert(server_id(1), health_status("rust-analyzer"));
        state
            .health_statuses
            .insert(server_id(2), health_status("typescript-language-server"));

        state.remove_server(server_id(1));

        assert!(!state.health_statuses.contains_key(&server_id(1)));
        assert!(state.health_statuses.contains_key(&server_id(2)));
    }

    /// `remove_server` evicts the id from each per-buffer entry; entries that
    /// become empty are dropped so the map does not grow unbounded across
    /// many buffer opens/closes.
    #[test]
    fn remove_server_evicts_id_from_per_buffer_entries_and_drops_empty_entries() {
        let mut state = LanguageServers::default();
        let buffer_a = PathBuf::from("/project/a.rs");
        let buffer_b = PathBuf::from("/project/b.rs");

        state.servers_per_buffer_abs_path.insert(
            buffer_a.clone(),
            servers_for_path(&[(server_id(1), "rust-analyzer")]),
        );
        state.servers_per_buffer_abs_path.insert(
            buffer_b.clone(),
            servers_for_path(&[(server_id(1), "rust-analyzer"), (server_id(2), "typos-lsp")]),
        );

        state.remove_server(server_id(1));

        assert!(
            !state.servers_per_buffer_abs_path.contains_key(&buffer_a),
            "buffer_a's entry held only the removed server, so the entry itself should be dropped",
        );
        let buffer_b_entry = state
            .servers_per_buffer_abs_path
            .get(&buffer_b)
            .expect("buffer_b's entry has another server, so it must be retained");
        assert!(!buffer_b_entry.servers.contains_key(&server_id(1)));
        assert!(buffer_b_entry.servers.contains_key(&server_id(2)));
    }

    /// Simulates the full restart event sequence: remove old id, register
    /// new id with same name, write health for the new id. After restart
    /// only the new id should be visible — no leftover entry from the old
    /// incarnation.
    #[test]
    fn restart_sequence_leaves_only_new_server_id() {
        let mut state = LanguageServers::default();
        let buffer = PathBuf::from("/project/main.rs");
        let name = "rust-analyzer";

        // Pre-restart: server v1 is registered for the buffer with health.
        state
            .servers_per_buffer_abs_path
            .insert(buffer.clone(), servers_for_path(&[(server_id(1), name)]));
        state
            .health_statuses
            .insert(server_id(1), health_status(name));

        // Restart: old id is removed.
        state.remove_server(server_id(1));

        // New id registers for the same buffer.
        let entry = state
            .servers_per_buffer_abs_path
            .entry(buffer.clone())
            .or_insert_with(|| ServersForPath {
                servers: HashMap::default(),
                worktree: None,
            });
        entry.servers.insert(server_id(2), Some(server_name(name)));

        // Health update for the new id arrives.
        state
            .health_statuses
            .insert(server_id(2), health_status(name));

        let entry = state
            .servers_per_buffer_abs_path
            .get(&buffer)
            .expect("buffer must still be tracked");
        assert_eq!(
            entry.servers.keys().copied().collect::<Vec<_>>(),
            vec![server_id(2)],
            "exactly one server for this buffer — the new incarnation",
        );
        assert!(
            !state.health_statuses.contains_key(&server_id(1)),
            "the dead server's health entry must not linger",
        );
        assert!(
            state.health_statuses.contains_key(&server_id(2)),
            "the new server's health entry is present",
        );
    }

    fn init_test(cx: &mut TestAppContext) {
        zlog::init_test();
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
        });
        cx.executor().allow_parking();
    }

    async fn start_fake_server(
        project: &Entity<Project>,
        abs_path: impl AsRef<Path>,
        cx: &mut TestAppContext,
    ) -> (
        futures::channel::mpsc::UnboundedReceiver<lsp::FakeLanguageServer>,
        Entity<Buffer>,
        lsp_store::OpenLspBufferHandle,
    ) {
        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        language_registry.add(rust_lang());
        let fake_servers = language_registry.register_fake_lsp(
            "Rust",
            FakeLspAdapter {
                name: "the-rust-language-server",
                ..Default::default()
            },
        );
        let (buffer, handle) = project
            .update(cx, |project, cx| {
                project.open_local_buffer_with_lsp(abs_path.as_ref(), cx)
            })
            .await
            .unwrap();
        (fake_servers, buffer, handle)
    }

    fn build_lsp_button(project: &Entity<Project>, cx: &mut TestAppContext) -> Entity<LspButton> {
        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        workspace_window
            .update(cx, |workspace, window, cx| {
                cx.new(|cx| LspButton::new(workspace, PopoverMenuHandle::default(), window, cx))
            })
            .unwrap()
    }

    fn running_server_names(button: &Entity<LspButton>, cx: &TestAppContext) -> Vec<String> {
        button.read_with(cx, |button, cx| {
            button
                .server_state
                .read(cx)
                .items
                .iter()
                .filter_map(|item| match item {
                    LspMenuItem::WithHealthCheck { health, .. } => Some(health.name.0.to_string()),
                    LspMenuItem::WithBinaryStatus { server_name, .. } => {
                        Some(server_name.0.to_string())
                    }
                    _ => None,
                })
                .collect()
        })
    }

    fn stopped_server_names(button: &Entity<LspButton>, cx: &TestAppContext) -> Vec<String> {
        button.read_with(cx, |button, cx| {
            button
                .server_state
                .read(cx)
                .items
                .iter()
                .filter_map(|item| match item {
                    LspMenuItem::WithStoppedStatus { server_name, .. } => {
                        Some(server_name.0.to_string())
                    }
                    _ => None,
                })
                .collect()
        })
    }

    fn has_restart_all_button(button: &Entity<LspButton>, cx: &TestAppContext) -> bool {
        button.read_with(cx, |button, cx| {
            button
                .server_state
                .read(cx)
                .items
                .iter()
                .any(|item| matches!(item, LspMenuItem::ToggleServersButton { restart: true }))
        })
    }

    fn has_stop_all_button(button: &Entity<LspButton>, cx: &TestAppContext) -> bool {
        button.read_with(cx, |button, cx| {
            button
                .server_state
                .read(cx)
                .items
                .iter()
                .any(|item| matches!(item, LspMenuItem::ToggleServersButton { restart: false }))
        })
    }

    fn pump_lsp_menu(cx: &mut TestAppContext) {
        cx.executor().advance_clock(Duration::from_millis(30));
        cx.run_until_parked();
    }

    /// Check whether stopped servers are listed in the menu and can be restarted.
    #[gpui::test]
    async fn test_lsp_button_keeps_stopped_servers_listed_and_restarts_them(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/the-root"),
            json!({
                "main.rs": "fn main() {}",
            }),
        )
        .await;

        let project = Project::test(fs, [path!("/the-root").as_ref()], cx).await;
        let (mut fake_servers, _buffer, _handle) =
            start_fake_server(&project, path!("/the-root/main.rs"), cx).await;
        let _first_server = fake_servers.next().await.unwrap();
        cx.run_until_parked();

        let lsp_button = build_lsp_button(&project, cx);
        pump_lsp_menu(cx);

        assert_eq!(
            running_server_names(&lsp_button, cx),
            vec!["the-rust-language-server".to_string()],
            "the running server is listed",
        );
        assert!(
            has_stop_all_button(&lsp_button, cx),
            "a running server can be stopped",
        );

        let lsp_store = project.read_with(cx, |project, _| project.lsp_store());
        lsp_store.update(cx, |lsp_store, cx| lsp_store.stop_all_language_servers(cx));
        cx.run_until_parked();
        pump_lsp_menu(cx);

        assert_eq!(
            stopped_server_names(&lsp_button, cx),
            vec!["the-rust-language-server".to_string()],
            "#61896: the stopped server stays listed",
        );
        assert!(
            has_restart_all_button(&lsp_button, cx),
            "the stopped server can be restarted",
        );
        assert!(
            !has_stop_all_button(&lsp_button, cx),
            "with nothing running, the Stop All button is hidden",
        );

        lsp_store.update(cx, |lsp_store, cx| {
            lsp_store.restart_all_language_servers(cx)
        });
        let _restarted_server = fake_servers.next().await.unwrap();
        cx.run_until_parked();
        pump_lsp_menu(cx);

        assert_eq!(
            running_server_names(&lsp_button, cx),
            vec!["the-rust-language-server".to_string()],
            "the restarted server is running again",
        );
        assert!(
            stopped_server_names(&lsp_button, cx).is_empty(),
            "the stopped row is gone after a restart",
        );
        assert!(
            has_stop_all_button(&lsp_button, cx),
            "the restarted server can be stopped again",
        );
    }

    /// A server stopped by name stays listed and can be restarted by name.
    #[gpui::test]
    async fn test_lsp_button_restarts_a_stopped_server_by_name(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/the-root"),
            json!({
                "main.rs": "fn main() {}",
            }),
        )
        .await;

        let project = Project::test(fs, [path!("/the-root").as_ref()], cx).await;
        let (mut fake_servers, _buffer, _handle) =
            start_fake_server(&project, path!("/the-root/main.rs"), cx).await;
        let _first_server = fake_servers.next().await.unwrap();
        cx.run_until_parked();

        let lsp_button = build_lsp_button(&project, cx);
        pump_lsp_menu(cx);

        assert_eq!(
            running_server_names(&lsp_button, cx),
            vec!["the-rust-language-server".to_string()],
            "the running server is listed",
        );
        assert!(has_stop_all_button(&lsp_button, cx));

        let lsp_store = project.read_with(cx, |project, _| project.lsp_store());
        let stop = lsp_store.update(cx, |lsp_store, cx| {
            lsp_store.stop_language_servers_for_buffers(
                Vec::new(),
                HashSet::from_iter([LanguageServerSelector::Name(server_name(
                    "the-rust-language-server",
                ))]),
                cx,
            )
        });
        stop.await.unwrap();
        cx.run_until_parked();
        pump_lsp_menu(cx);

        assert_eq!(
            stopped_server_names(&lsp_button, cx),
            vec!["the-rust-language-server".to_string()],
            "the stopped server stays listed",
        );
        assert!(has_restart_all_button(&lsp_button, cx));
        assert!(
            !has_stop_all_button(&lsp_button, cx),
            "with nothing running, the Stop All button is hidden",
        );

        let server_state = lsp_button.read_with(cx, |button, _| button.server_state.clone());
        server_state.update(cx, |state, cx| {
            state.restart_server_by_name(server_name("the-rust-language-server"), cx)
        });
        let _restarted_server = fake_servers.next().await.unwrap();
        cx.run_until_parked();
        pump_lsp_menu(cx);

        assert_eq!(
            running_server_names(&lsp_button, cx),
            vec!["the-rust-language-server".to_string()],
            "the restarted server is running again",
        );
        assert!(
            stopped_server_names(&lsp_button, cx).is_empty(),
            "the stopped row is gone after a restart",
        );
        assert!(
            has_stop_all_button(&lsp_button, cx),
            "the restarted server can be stopped again",
        );
    }

    #[gpui::test]
    async fn test_lsp_button_restarts_only_the_stopped_worktree_server(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/the-root-a"),
            json!({
                "main.rs": "fn main() {}",
            }),
        )
        .await;
        fs.insert_tree(
            path!("/the-root-b"),
            json!({
                "main.rs": "fn main() {}",
            }),
        )
        .await;

        let project = Project::test(
            fs,
            [path!("/the-root-a").as_ref(), path!("/the-root-b").as_ref()],
            cx,
        )
        .await;
        let language_registry = project.read_with(cx, |project, _| project.languages().clone());
        language_registry.add(rust_lang());
        let mut fake_servers = language_registry.register_fake_lsp(
            "Rust",
            FakeLspAdapter {
                name: "the-rust-language-server",
                ..Default::default()
            },
        );

        let (buffer_a, _handle_a) = project
            .update(cx, |project, cx| {
                project.open_local_buffer_with_lsp(path!("/the-root-a/main.rs"), cx)
            })
            .await
            .expect("opening the first worktree buffer should succeed");
        let mut server_a = fake_servers
            .next()
            .await
            .expect("the first worktree should start a language server");
        server_a
            .receive_notification::<lsp::notification::DidOpenTextDocument>()
            .await;

        let (_buffer_b, _handle_b) = project
            .update(cx, |project, cx| {
                project.open_local_buffer_with_lsp(path!("/the-root-b/main.rs"), cx)
            })
            .await
            .expect("opening the second worktree buffer should succeed");
        let mut server_b = fake_servers
            .next()
            .await
            .expect("the second worktree should start a language server");
        server_b
            .receive_notification::<lsp::notification::DidOpenTextDocument>()
            .await;

        let worktree_a = buffer_a.read_with(cx, |buffer, cx| {
            buffer.file().map(|file| file.worktree_id(cx))
        });
        let Some(worktree_a) = worktree_a else {
            panic!("the first buffer should belong to a worktree");
        };
        let server_a_id = server_a.server.server_id();
        let server_b_id = server_b.server.server_id();
        let lsp_store = project.read_with(cx, |project, _| project.lsp_store());
        lsp_store
            .update(cx, |lsp_store, cx| {
                lsp_store.stop_language_servers_for_buffers(
                    Vec::new(),
                    HashSet::from_iter([LanguageServerSelector::Id(server_a_id)]),
                    cx,
                )
            })
            .await
            .expect("stopping the first worktree server should succeed");
        cx.run_until_parked();

        let lsp_button = build_lsp_button(&project, cx);
        pump_lsp_menu(cx);
        let server_state = lsp_button.read_with(cx, |button, _| button.server_state.clone());
        server_state.update(cx, |state, cx| {
            state.restart_server_for_worktree(
                server_name("the-rust-language-server"),
                worktree_a,
                cx,
            )
        });

        let mut restarted_server_a = fake_servers
            .next()
            .await
            .expect("the stopped worktree server should restart");
        restarted_server_a
            .receive_notification::<lsp::notification::DidOpenTextDocument>()
            .await;
        cx.run_until_parked();

        let running_server_ids = project.read_with(cx, |project, cx| {
            project
                .language_server_statuses(cx)
                .map(|(server_id, _)| server_id)
                .collect::<HashSet<_>>()
        });
        assert_eq!(running_server_ids.len(), 2);
        assert!(running_server_ids.contains(&server_b_id));
        assert!(running_server_ids.contains(&restarted_server_a.server.server_id()));
    }

    /// Stopping servers in one workspace must not affect another
    #[gpui::test]
    async fn test_lsp_button_stopping_one_workspace_does_not_affect_another(
        cx: &mut TestAppContext,
    ) {
        init_test(cx);

        let fs_a = FakeFs::new(cx.executor());
        fs_a.insert_tree(
            path!("/the-root-a"),
            json!({
                "main.rs": "fn main() {}",
            }),
        )
        .await;
        let project_a = Project::test(fs_a, [path!("/the-root-a").as_ref()], cx).await;
        let (mut servers_a, _buffer_a, _handle_a) =
            start_fake_server(&project_a, path!("/the-root-a/main.rs"), cx).await;
        let _server_a = servers_a.next().await.unwrap();
        cx.run_until_parked();

        let fs_b = FakeFs::new(cx.executor());
        fs_b.insert_tree(
            path!("/the-root-b"),
            json!({
                "main.rs": "fn main() {}",
            }),
        )
        .await;
        let project_b = Project::test(fs_b, [path!("/the-root-b").as_ref()], cx).await;
        let (mut servers_b, _buffer_b, _handle_b) =
            start_fake_server(&project_b, path!("/the-root-b/main.rs"), cx).await;
        let _server_b = servers_b.next().await.unwrap();
        cx.run_until_parked();

        let button_a = build_lsp_button(&project_a, cx);
        let button_b = build_lsp_button(&project_b, cx);
        pump_lsp_menu(cx);

        assert_eq!(
            running_server_names(&button_a, cx),
            vec!["the-rust-language-server".to_string()],
            "workspace A's server is running",
        );
        assert_eq!(
            running_server_names(&button_b, cx),
            vec!["the-rust-language-server".to_string()],
            "workspace B's server is running",
        );

        let lsp_store_a = project_a.read_with(cx, |project, _| project.lsp_store());
        lsp_store_a.update(cx, |lsp_store, cx| lsp_store.stop_all_language_servers(cx));
        cx.run_until_parked();
        pump_lsp_menu(cx);

        assert_eq!(
            stopped_server_names(&button_a, cx),
            vec!["the-rust-language-server".to_string()],
            "#61896: workspace A's stopped server stays listed",
        );
        assert!(has_restart_all_button(&button_a, cx));
        assert!(
            !has_stop_all_button(&button_a, cx),
            "workspace A has nothing running, so Stop All is hidden",
        );
        assert_eq!(
            running_server_names(&button_b, cx),
            vec!["the-rust-language-server".to_string()],
            "#62121: workspace B's server keeps running",
        );
        assert!(
            stopped_server_names(&button_b, cx).is_empty(),
            "#62121: workspace B shows no stopped server",
        );
        assert!(
            has_stop_all_button(&button_b, cx),
            "#62121: workspace B can still be stopped",
        );

        lsp_store_a.update(cx, |lsp_store, cx| {
            lsp_store.restart_all_language_servers(cx)
        });
        let _restarted_server_a = servers_a.next().await.unwrap();
        cx.run_until_parked();
        pump_lsp_menu(cx);

        assert_eq!(
            running_server_names(&button_a, cx),
            vec!["the-rust-language-server".to_string()],
            "workspace A's server is running again",
        );
        assert!(stopped_server_names(&button_a, cx).is_empty());
        assert!(
            has_stop_all_button(&button_a, cx),
            "workspace A is running again and can be stopped",
        );
        assert_eq!(
            running_server_names(&button_b, cx),
            vec!["the-rust-language-server".to_string()],
            "workspace B's server is still running",
        );
    }

    /// Ensure the stopped servers logs can be retained
    #[gpui::test]
    async fn test_lsp_button_stopped_server_logs_viewable_after_stop(cx: &mut TestAppContext) {
        init_test(cx);

        let log_store = cx.update(|cx| lsp_store::log_store::init(false, cx));

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/the-root"),
            json!({
                "main.rs": "fn main() {}",
            }),
        )
        .await;

        let project = Project::test(fs, [path!("/the-root").as_ref()], cx).await;
        log_store.update(cx, |store, cx| store.add_project(&project, cx));

        let (mut fake_servers, _buffer, _handle) =
            start_fake_server(&project, path!("/the-root/main.rs"), cx).await;
        let mut language_server = fake_servers.next().await.unwrap();
        language_server
            .receive_notification::<lsp::notification::DidOpenTextDocument>()
            .await;
        cx.run_until_parked();

        language_server.notify::<lsp::notification::LogMessage>(lsp::LogMessageParams {
            message: "hello from the server".into(),
            typ: lsp::MessageType::INFO,
        });
        cx.run_until_parked();

        let workspace_window =
            cx.add_window(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let lsp_button = workspace_window
            .update(cx, |workspace, window, cx| {
                cx.new(|cx| LspButton::new(workspace, PopoverMenuHandle::default(), window, cx))
            })
            .unwrap();
        pump_lsp_menu(cx);

        let name = LanguageServerName("the-rust-language-server".into());
        let worktree_id = project.read_with(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        let server_id = log_store
            .read_with(cx, |store, _| {
                store.language_server_id_for_name_and_worktree(&name, worktree_id)
            })
            .expect("the running server is tracked by the log store");

        let lsp_store = project.read_with(cx, |project, _| project.lsp_store());
        let stop = lsp_store.update(cx, |lsp_store, cx| {
            lsp_store.stop_language_servers_for_buffers(
                Vec::new(),
                HashSet::from_iter([LanguageServerSelector::Name(name.clone())]),
                cx,
            )
        });
        stop.await.unwrap();
        cx.run_until_parked();
        pump_lsp_menu(cx);

        assert_eq!(
            stopped_server_names(&lsp_button, cx),
            vec!["the-rust-language-server".to_string()],
            "the stopped server stays listed",
        );
        assert!(
            log_store.read_with(cx, |store, _| store.contains_language_server(server_id)),
            "the stopped server's log entry is retained",
        );
        assert!(
            log_store.read_with(cx, |store, _| {
                store.has_server_logs(&LanguageServerSelector::Id(server_id))
            }),
            "the retained entry keeps the logs recorded before the stop",
        );

        workspace_window
            .update(cx, |workspace, window, cx| {
                lsp_log_view::open(
                    &log_store,
                    workspace.weak_handle(),
                    LanguageServerSelector::Id(server_id),
                    window,
                    cx,
                );
            })
            .log_err();
        cx.executor().run_until_parked();
        cx.executor().run_until_parked();

        let log_view = workspace_window
            .read_with(cx, |workspace, cx| workspace.item_of_type::<LspLogView>(cx))
            .expect("the workspace window is alive")
            .expect("opening the logs creates the log view");
        assert_eq!(
            log_view.read_with(cx, |view, cx| view.editor.read(cx).text(cx)),
            "hello from the server\n",
            "the log view shows the stopped server's retained logs",
        );

        let server_state = lsp_button.read_with(cx, |button, _| button.server_state.clone());
        server_state.update(cx, |state, cx| {
            state.restart_server_by_name(server_name("the-rust-language-server"), cx)
        });
        let mut restarted_server = fake_servers.next().await.unwrap();
        restarted_server
            .receive_notification::<lsp::notification::DidOpenTextDocument>()
            .await;
        cx.run_until_parked();
        pump_lsp_menu(cx);

        assert!(
            stopped_server_names(&lsp_button, cx).is_empty(),
            "the stopped row is gone after a restart",
        );
        assert_eq!(
            log_view.read_with(cx, |view, cx| view.editor.read(cx).text(cx)),
            "hello from the server\n",
            "the log view follows the restarted server and keeps the retained logs",
        )
    }
    #[test]
    fn tooltip_for_server_binary_handles_runtime_and_standalone_servers() {
        let node_server = lsp::LanguageServerBinary {
            path: "/usr/bin/node".into(),
            arguments: vec![
                "/zed/languages/basedpyright/langserver.index.js".into(),
                "--stdio".into(),
            ],
            env: None,
        };
        assert_eq!(
            tooltip_for_server_binary(&node_server, PathStyle::Unix),
            "/zed/languages/basedpyright/langserver.index.js (node)"
        );

        let node_server_windows = lsp::LanguageServerBinary {
                path: "C:\\Program Files\\nodejs\\node.exe".into(),
                arguments: vec![
                    "C:\\Users\\Zed\\languages\\basedpyright\\node_modules/basedpyright/langserver.index.js".into(),
                    "--stdio".into(),
                ],
                env: None
        };
        assert_eq!(
            tooltip_for_server_binary(&node_server_windows, PathStyle::Windows),
            "C:\\Users\\Zed\\languages\\basedpyright\\node_modules\\basedpyright\\langserver.index.js (node)"
        );

        let python_server = lsp::LanguageServerBinary {
            path: "/usr/bin/python3".into(),
            arguments: vec!["/zed/languages/pylsp/pylsp".into(), "--stdio".into()],
            env: None,
        };
        assert_eq!(
            tooltip_for_server_binary(&python_server, PathStyle::Unix),
            "/zed/languages/pylsp/pylsp (python)"
        );

        let standalone_server = lsp::LanguageServerBinary {
            path: "/usr/bin/ty".into(),
            arguments: vec!["server".into()],
            env: None,
        };
        assert_eq!(
            tooltip_for_server_binary(&standalone_server, PathStyle::Unix),
            "/usr/bin/ty"
        );

        let flagged_node_server = lsp::LanguageServerBinary {
            path: "/usr/bin/node".into(),
            arguments: vec![
                "--max-old-space-size=8192".into(),
                "/zed/languages/eslint/server.js".into(),
                "--stdio".into(),
            ],
            env: None,
        };
        assert_eq!(
            tooltip_for_server_binary(&flagged_node_server, PathStyle::Unix),
            "/zed/languages/eslint/server.js (node)"
        );
    }
}
