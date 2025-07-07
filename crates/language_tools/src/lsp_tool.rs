use std::{collections::hash_map, path::PathBuf, rc::Rc, time::Duration};

use client::proto;
use collections::{HashMap, HashSet};
use editor::{Editor, EditorEvent};
use feature_flags::FeatureFlagAppExt as _;
use gpui::{Corner, Entity, Subscription, Task, WeakEntity, actions};
use language::{BinaryStatus, BufferId, LocalFile, ServerHealth};
use lsp::{LanguageServerId, LanguageServerName, LanguageServerSelector};
use project::{LspStore, LspStoreEvent, project_settings::ProjectSettings};
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
    items: Vec<LspItem>,
    other_servers_start_index: Option<usize>,
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
    servers_per_buffer_abs_path:
        HashMap<PathBuf, HashMap<LanguageServerId, Option<LanguageServerName>>>,
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
        let lsp_logs = cx
            .try_global::<GlobalLogStore>()
            .and_then(|lsp_logs| lsp_logs.0.upgrade());
        let lsp_store = self.lsp_store.upgrade();
        let Some((lsp_logs, lsp_store)) = lsp_logs.zip(lsp_store) else {
            return menu;
        };

        for (i, item) in self.items.iter().enumerate() {
            if let LspItem::ToggleServersButton { restart } = item {
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
                                    let worktree_store = project.read(cx).worktree_store();

                                    let buffers = state
                                        .read(cx)
                                        .language_servers
                                        .servers_per_buffer_abs_path
                                        .keys()
                                        .filter_map(|abs_path| {
                                            worktree_store.read(cx).find_worktree(abs_path, cx)
                                        })
                                        .filter_map(|(worktree, relative_path)| {
                                            let entry =
                                                worktree.read(cx).entry_for_path(&relative_path)?;
                                            project.read(cx).path_for_entry(entry.id, cx)
                                        })
                                        .filter_map(|project_path| {
                                            buffer_store.read(cx).get_by_path(&project_path)
                                        })
                                        .collect();
                                    let selectors = state
                                        .read(cx)
                                        .items
                                        .iter()
                                        // Do not try to use IDs as we have stopped all servers already, when allowing to restart them all
                                        .flat_map(|item| match item {
                                            LspItem::ToggleServersButton { .. } => None,
                                            LspItem::WithHealthCheck(_, status, ..) => Some(
                                                LanguageServerSelector::Name(status.name.clone()),
                                            ),
                                            LspItem::WithBinaryStatus(_, server_name, ..) => Some(
                                                LanguageServerSelector::Name(server_name.clone()),
                                            ),
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
                menu = menu.separator().item(button);
                continue;
            };
            let Some(server_info) = item.server_info() else {
                continue;
            };
            let workspace = self.workspace.clone();
            let server_selector = server_info.server_selector();
            // TODO currently, Zed remote does not work well with the LSP logs
            // https://github.com/zed-industries/zed/issues/28557
            let has_logs = lsp_store.read(cx).as_local().is_some()
                && lsp_logs.read(cx).has_server_logs(&server_selector);
            let status_color = server_info
                .binary_status
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

            if self
                .other_servers_start_index
                .is_some_and(|index| index == i)
            {
                menu = menu.separator();
            }
            menu = menu.item(ContextMenuItem::custom_entry(
                move |_, _| {
                    h_flex()
                        .gap_1()
                        .w_full()
                        .child(Indicator::dot().color(status_color))
                        .child(Label::new(server_info.name.0.clone()))
                        .when(!has_logs, |div| div.cursor_default())
                        .into_any_element()
                },
                {
                    let lsp_logs = lsp_logs.clone();
                    move |window, cx| {
                        if !has_logs {
                            cx.propagate();
                            return;
                        }
                        lsp_logs.update(cx, |lsp_logs, cx| {
                            lsp_logs.open_server_trace(
                                workspace.clone(),
                                server_selector.clone(),
                                window,
                                cx,
                            );
                        });
                    }
                },
                server_info.message.map(|server_message| {
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
    WithHealthCheck(
        LanguageServerId,
        &'a LanguageServerHealthStatus,
        Option<&'a LanguageServerBinaryStatus>,
    ),
    WithBinaryStatus(
        Option<LanguageServerId>,
        &'a LanguageServerName,
        &'a LanguageServerBinaryStatus,
    ),
}

#[derive(Debug)]
enum LspItem {
    WithHealthCheck(
        LanguageServerId,
        LanguageServerHealthStatus,
        Option<LanguageServerBinaryStatus>,
    ),
    WithBinaryStatus(
        Option<LanguageServerId>,
        LanguageServerName,
        LanguageServerBinaryStatus,
    ),
    ToggleServersButton {
        restart: bool,
    },
}

impl LspItem {
    fn server_info(&self) -> Option<ServerInfo> {
        match self {
            LspItem::ToggleServersButton { .. } => None,
            LspItem::WithHealthCheck(
                language_server_id,
                language_server_health_status,
                language_server_binary_status,
            ) => Some(ServerInfo {
                name: language_server_health_status.name.clone(),
                id: Some(*language_server_id),
                health: language_server_health_status.health(),
                binary_status: language_server_binary_status.clone(),
                message: language_server_health_status.message(),
            }),
            LspItem::WithBinaryStatus(
                server_id,
                language_server_name,
                language_server_binary_status,
            ) => Some(ServerInfo {
                name: language_server_name.clone(),
                id: *server_id,
                health: None,
                binary_status: Some(language_server_binary_status.clone()),
                message: language_server_binary_status.message.clone(),
            }),
        }
    }
}

impl ServerData<'_> {
    fn name(&self) -> &LanguageServerName {
        match self {
            Self::WithHealthCheck(_, state, _) => &state.name,
            Self::WithBinaryStatus(_, name, ..) => name,
        }
    }

    fn into_lsp_item(self) -> LspItem {
        match self {
            Self::WithHealthCheck(id, name, status) => {
                LspItem::WithHealthCheck(id, name.clone(), status.cloned())
            }
            Self::WithBinaryStatus(server_id, name, status) => {
                LspItem::WithBinaryStatus(server_id, name.clone(), status.clone())
            }
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
            other_servers_start_index: None,
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
                self.server_state.update(cx, |state, _| {
                    state
                        .language_servers
                        .servers_per_buffer_abs_path
                        .entry(PathBuf::from(&update.buffer_abs_path))
                        .or_default()
                        .insert(*language_server_id, name.clone());
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
            let editor_buffers = state
                .active_editor
                .as_ref()
                .map(|active_editor| active_editor.editor_buffers.clone())
                .unwrap_or_default();
            let editor_buffer_paths = editor_buffers
                .iter()
                .filter_map(|buffer_id| {
                    let buffer_path = state
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
                    Some(buffer_path)
                })
                .collect::<Vec<_>>();

            let mut servers_with_health_checks = HashSet::default();
            let mut server_ids_with_health_checks = HashSet::default();
            let mut buffer_servers =
                Vec::with_capacity(state.language_servers.health_statuses.len());
            let mut other_servers =
                Vec::with_capacity(state.language_servers.health_statuses.len());
            let buffer_server_ids = editor_buffer_paths
                .iter()
                .filter_map(|buffer_path| {
                    state
                        .language_servers
                        .servers_per_buffer_abs_path
                        .get(buffer_path)
                })
                .flatten()
                .fold(HashMap::default(), |mut acc, (server_id, name)| {
                    match acc.entry(*server_id) {
                        hash_map::Entry::Occupied(mut o) => {
                            let old_name: &mut Option<&LanguageServerName> = o.get_mut();
                            if old_name.is_none() {
                                *old_name = name.as_ref();
                            }
                        }
                        hash_map::Entry::Vacant(v) => {
                            v.insert(name.as_ref());
                        }
                    }
                    acc
                });
            for (server_id, server_state) in &state.language_servers.health_statuses {
                let binary_status = state
                    .language_servers
                    .binary_statuses
                    .get(&server_state.name);
                servers_with_health_checks.insert(&server_state.name);
                server_ids_with_health_checks.insert(*server_id);
                if buffer_server_ids.contains_key(server_id) {
                    buffer_servers.push(ServerData::WithHealthCheck(
                        *server_id,
                        server_state,
                        binary_status,
                    ));
                } else {
                    other_servers.push(ServerData::WithHealthCheck(
                        *server_id,
                        server_state,
                        binary_status,
                    ));
                }
            }

            let mut can_stop_all = !state.language_servers.health_statuses.is_empty();
            let mut can_restart_all = state.language_servers.health_statuses.is_empty();
            for (server_name, status) in state
                .language_servers
                .binary_statuses
                .iter()
                .filter(|(name, _)| !servers_with_health_checks.contains(name))
            {
                match status.status {
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

                let matching_server_id = state
                    .language_servers
                    .servers_per_buffer_abs_path
                    .iter()
                    .filter(|(path, _)| editor_buffer_paths.contains(path))
                    .flat_map(|(_, server_associations)| server_associations.iter())
                    .find_map(|(id, name)| {
                        if name.as_ref() == Some(server_name) {
                            Some(*id)
                        } else {
                            None
                        }
                    });
                if let Some(server_id) = matching_server_id {
                    buffer_servers.push(ServerData::WithBinaryStatus(
                        Some(server_id),
                        server_name,
                        status,
                    ));
                } else {
                    other_servers.push(ServerData::WithBinaryStatus(None, server_name, status));
                }
            }

            buffer_servers.sort_by_key(|data| data.name().clone());
            other_servers.sort_by_key(|data| data.name().clone());

            let mut other_servers_start_index = None;
            let mut new_lsp_items =
                Vec::with_capacity(buffer_servers.len() + other_servers.len() + 1);
            new_lsp_items.extend(buffer_servers.into_iter().map(ServerData::into_lsp_item));
            if !new_lsp_items.is_empty() {
                other_servers_start_index = Some(new_lsp_items.len());
            }
            new_lsp_items.extend(other_servers.into_iter().map(ServerData::into_lsp_item));
            if !new_lsp_items.is_empty() {
                if can_stop_all {
                    new_lsp_items.push(LspItem::ToggleServersButton { restart: false });
                } else if can_restart_all {
                    new_lsp_items.push(LspItem::ToggleServersButton { restart: true });
                }
            }

            state.items = new_lsp_items;
            state.other_servers_start_index = other_servers_start_index;
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
                        // TODO kb will this work?
                        // what about the selections?
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
        if !cx.is_staff()
            || self.server_state.read(cx).language_servers.is_empty()
            || self.lsp_menu.is_none()
        {
            return div();
        }

        let mut has_errors = false;
        let mut has_warnings = false;
        let mut has_other_notifications = false;
        let state = self.server_state.read(cx);
        for server in state.language_servers.health_statuses.values() {
            if let Some(binary_status) = &state.language_servers.binary_statuses.get(&server.name) {
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
                        Tooltip::for_action("Language Servers", &ToggleMenu, window, cx)
                    },
                ),
        )
    }
}
