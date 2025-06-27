use std::{collections::hash_map, path::PathBuf, sync::Arc, time::Duration};

use client::proto;
use collections::{HashMap, HashSet};
use editor::{Editor, EditorEvent};
use feature_flags::FeatureFlagAppExt as _;
use gpui::{Corner, DismissEvent, Entity, Focusable as _, Subscription, Task, WeakEntity, actions};
use language::{BinaryStatus, BufferId, LocalFile, ServerHealth};
use lsp::{LanguageServerId, LanguageServerName, LanguageServerSelector};
use picker::{Picker, PickerDelegate, popover_menu::PickerPopoverMenu};
use project::{LspStore, LspStoreEvent, project_settings::ProjectSettings};
use settings::{Settings as _, SettingsStore};
use ui::{Context, Indicator, Tooltip, Window, prelude::*};

use workspace::{StatusItemView, Workspace};

use crate::lsp_log::GlobalLogStore;

actions!(lsp_tool, [ToggleMenu]);

pub struct LspTool {
    state: Entity<PickerState>,
    lsp_picker: Option<Entity<Picker<LspPickerDelegate>>>,
    _subscriptions: Vec<Subscription>,
}

struct PickerState {
    workspace: WeakEntity<Workspace>,
    lsp_store: WeakEntity<LspStore>,
    active_editor: Option<ActiveEditor>,
    language_servers: LanguageServers,
}

#[derive(Debug)]
struct LspPickerDelegate {
    state: Entity<PickerState>,
    selected_index: usize,
    items: Vec<LspItem>,
    other_servers_start_index: Option<usize>,
}

struct ActiveEditor {
    editor: WeakEntity<Editor>,
    _editor_subscription: Subscription,
    editor_buffers: HashSet<BufferId>,
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

impl LspPickerDelegate {
    fn regenerate_items(&mut self, cx: &mut Context<Picker<Self>>) {
        self.state.update(cx, |state, cx| {
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

            for (server_name, status) in state
                .language_servers
                .binary_statuses
                .iter()
                .filter(|(name, _)| !servers_with_health_checks.contains(name))
            {
                let has_matching_server = state
                    .language_servers
                    .servers_per_buffer_abs_path
                    .iter()
                    .filter(|(path, _)| editor_buffer_paths.contains(path))
                    .flat_map(|(_, server_associations)| server_associations.iter())
                    .any(|(_, name)| name.as_ref() == Some(server_name));
                if has_matching_server {
                    buffer_servers.push(ServerData::WithBinaryStatus(server_name, status));
                } else {
                    other_servers.push(ServerData::WithBinaryStatus(server_name, status));
                }
            }

            buffer_servers.sort_by_key(|data| data.name().clone());
            other_servers.sort_by_key(|data| data.name().clone());

            let mut other_servers_start_index = None;
            let mut new_lsp_items =
                Vec::with_capacity(buffer_servers.len() + other_servers.len() + 2);

            if !buffer_servers.is_empty() {
                new_lsp_items.push(LspItem::Header(SharedString::new("This Buffer")));
                new_lsp_items.extend(buffer_servers.into_iter().map(ServerData::into_lsp_item));
            }

            if !other_servers.is_empty() {
                other_servers_start_index = Some(new_lsp_items.len());
                new_lsp_items.push(LspItem::Header(SharedString::new("Other Servers")));
                new_lsp_items.extend(other_servers.into_iter().map(ServerData::into_lsp_item));
            }

            self.items = new_lsp_items;
            self.other_servers_start_index = other_servers_start_index;
        });
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
    WithBinaryStatus(&'a LanguageServerName, &'a LanguageServerBinaryStatus),
}

#[derive(Debug)]
enum LspItem {
    WithHealthCheck(
        LanguageServerId,
        LanguageServerHealthStatus,
        Option<LanguageServerBinaryStatus>,
    ),
    WithBinaryStatus(LanguageServerName, LanguageServerBinaryStatus),
    Header(SharedString),
}

impl ServerData<'_> {
    fn name(&self) -> &LanguageServerName {
        match self {
            Self::WithHealthCheck(_, state, _) => &state.name,
            Self::WithBinaryStatus(name, ..) => name,
        }
    }

    fn into_lsp_item(self) -> LspItem {
        match self {
            Self::WithHealthCheck(id, name, status) => {
                LspItem::WithHealthCheck(id, name.clone(), status.cloned())
            }
            Self::WithBinaryStatus(name, status) => {
                LspItem::WithBinaryStatus(name.clone(), status.clone())
            }
        }
    }
}

impl PickerDelegate for LspPickerDelegate {
    type ListItem = AnyElement;

    fn match_count(&self) -> usize {
        self.items.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, _: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.selected_index = ix;
        cx.notify();
    }

    fn update_matches(
        &mut self,
        _: String,
        _: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        cx.spawn(async move |lsp_picker, cx| {
            cx.background_executor()
                .timer(Duration::from_millis(30))
                .await;
            lsp_picker
                .update(cx, |lsp_picker, cx| {
                    lsp_picker.delegate.regenerate_items(cx);
                })
                .ok();
        })
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        Arc::default()
    }

    fn confirm(&mut self, _: bool, _: &mut Window, _: &mut Context<Picker<Self>>) {}

    fn dismissed(&mut self, _: &mut Window, cx: &mut Context<Picker<Self>>) {
        cx.emit(DismissEvent);
    }

    fn render_match(
        &self,
        ix: usize,
        _: bool,
        _: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let is_other_server = self
            .other_servers_start_index
            .map_or(false, |start| ix >= start);

        let server_binary_status;
        let server_health;
        let server_message;
        let server_id;
        let server_name;

        match self.items.get(ix)? {
            LspItem::WithHealthCheck(
                language_server_id,
                language_server_health_status,
                language_server_binary_status,
            ) => {
                server_binary_status = language_server_binary_status.as_ref();
                server_health = language_server_health_status.health();
                server_message = language_server_health_status.message();
                server_id = Some(*language_server_id);
                server_name = language_server_health_status.name.clone();
            }
            LspItem::WithBinaryStatus(language_server_name, language_server_binary_status) => {
                server_binary_status = Some(language_server_binary_status);
                server_health = None;
                server_message = language_server_binary_status.message.clone();
                server_id = None;
                server_name = language_server_name.clone();
            }
            LspItem::Header(header) => {
                return Some(
                    div()
                        .px_2p5()
                        .mb_1()
                        .child(
                            Label::new(header.clone())
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        )
                        .into_any_element(),
                );
            }
        };

        let workspace = self.state.read(cx).workspace.clone();
        let lsp_logs = cx.global::<GlobalLogStore>().0.upgrade()?;
        let lsp_store = self.state.read(cx).lsp_store.upgrade()?;
        let server_selector = server_id
            .map(LanguageServerSelector::Id)
            .unwrap_or_else(|| LanguageServerSelector::Name(server_name.clone()));
        let can_stop = server_binary_status.is_none_or(|status| {
            matches!(status.status, BinaryStatus::None | BinaryStatus::Starting)
        });

        // TODO currently, Zed remote does not work well with the LSP logs
        // https://github.com/zed-industries/zed/issues/28557
        let has_logs = lsp_store.read(cx).as_local().is_some()
            && lsp_logs.read(cx).has_server_logs(&server_selector);

        let status_color = server_binary_status
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
                Some(match server_health? {
                    ServerHealth::Ok => Color::Success,
                    ServerHealth::Warning => Color::Warning,
                    ServerHealth::Error => Color::Error,
                })
            })
            .unwrap_or(Color::Success);

        Some(
            h_flex()
                .px_1()
                .gap_1()
                .justify_between()
                .child(
                    h_flex()
                        .id("server-status-indicator")
                        .px_2()
                        .gap_2()
                        .child(Indicator::dot().color(status_color))
                        .child(Label::new(server_name.0.clone()))
                        .when_some(server_message.clone(), |div, server_message| {
                            div.tooltip(Tooltip::text(server_message.clone()))
                        }),
                )
                .child(
                    h_flex()
                        .when(has_logs, |button_list| {
                            button_list.child(
                                IconButton::new("debug-language-server", IconName::LspDebug)
                                    .icon_size(IconSize::Small)
                                    .alpha(0.8)
                                    .tooltip(Tooltip::text("Debug Language Server"))
                                    .on_click({
                                        let workspace = workspace.clone();
                                        let lsp_logs = lsp_logs.downgrade();
                                        let server_selector = server_selector.clone();
                                        move |_, window, cx| {
                                            lsp_logs
                                                .update(cx, |lsp_logs, cx| {
                                                    lsp_logs.open_server_trace(
                                                        workspace.clone(),
                                                        server_selector.clone(),
                                                        window,
                                                        cx,
                                                    );
                                                })
                                                .ok();
                                        }
                                    }),
                            )
                        })
                        .when(can_stop, |button_list| {
                            button_list.child(
                                IconButton::new("stop-server", IconName::LspStop)
                                    .icon_size(IconSize::Small)
                                    .alpha(0.8)
                                    .tooltip(Tooltip::text("Stop Server"))
                                    .on_click({
                                        let lsp_store = lsp_store.downgrade();
                                        let server_selector = server_selector.clone();
                                        move |_, _, cx| {
                                            lsp_store
                                                .update(cx, |lsp_store, cx| {
                                                    lsp_store.stop_language_servers_for_buffers(
                                                        Vec::new(),
                                                        HashSet::from_iter([
                                                            server_selector.clone()
                                                        ]),
                                                        cx,
                                                    );
                                                })
                                                .ok();
                                        }
                                    }),
                            )
                        })
                        .child(
                            IconButton::new("restart-server", IconName::LspRestart)
                                .icon_size(IconSize::Small)
                                .alpha(0.8)
                                .tooltip(Tooltip::text("Restart Server"))
                                .on_click({
                                    let state = self.state.clone();
                                    let workspace = workspace.clone();
                                    let lsp_store = lsp_store.downgrade();
                                    let editor_buffers = state
                                        .read(cx)
                                        .active_editor
                                        .as_ref()
                                        .map(|active_editor| active_editor.editor_buffers.clone())
                                        .unwrap_or_default();
                                    let server_selector = server_selector.clone();
                                    move |_, _, cx| {
                                        if let Some(workspace) = workspace.upgrade() {
                                            let project = workspace.read(cx).project().clone();
                                            let buffer_store =
                                                project.read(cx).buffer_store().clone();
                                            let buffers = if is_other_server {
                                                let worktree_store =
                                                    project.read(cx).worktree_store();
                                                state
                                                    .read(cx)
                                                    .language_servers
                                                    .servers_per_buffer_abs_path
                                                    .iter()
                                                    .filter_map(|(abs_path, servers)| {
                                                        if servers.values().any(|server| {
                                                            server.as_ref() == Some(&server_name)
                                                        }) {
                                                            worktree_store
                                                                .read(cx)
                                                                .find_worktree(abs_path, cx)
                                                        } else {
                                                            None
                                                        }
                                                    })
                                                    .filter_map(|(worktree, relative_path)| {
                                                        let entry = worktree
                                                            .read(cx)
                                                            .entry_for_path(&relative_path)?;
                                                        project
                                                            .read(cx)
                                                            .path_for_entry(entry.id, cx)
                                                    })
                                                    .filter_map(|project_path| {
                                                        buffer_store
                                                            .read(cx)
                                                            .get_by_path(&project_path)
                                                    })
                                                    .collect::<Vec<_>>()
                                            } else {
                                                editor_buffers
                                                    .iter()
                                                    .flat_map(|buffer_id| {
                                                        buffer_store.read(cx).get(*buffer_id)
                                                    })
                                                    .collect::<Vec<_>>()
                                            };
                                            if !buffers.is_empty() {
                                                lsp_store
                                                    .update(cx, |lsp_store, cx| {
                                                        lsp_store
                                                            .restart_language_servers_for_buffers(
                                                                buffers,
                                                                HashSet::from_iter([
                                                                    server_selector.clone(),
                                                                ]),
                                                                cx,
                                                            );
                                                    })
                                                    .ok();
                                            }
                                        }
                                    }
                                }),
                        ),
                )
                .into_any_element(),
        )
    }

    fn render_editor(
        &self,
        editor: &Entity<Editor>,
        _: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Div {
        div().child(div().track_focus(&editor.focus_handle(cx)))
    }

    fn render_footer(&self, _: &mut Window, cx: &mut Context<Picker<Self>>) -> Option<AnyElement> {
        let lsp_store = self.state.read(cx).lsp_store.clone();

        Some(
            div()
                .p_1()
                .border_t_1()
                .border_color(cx.theme().colors().border_variant)
                .child(
                    Button::new("stop-all-servers", "Stop All Servers")
                        .disabled(self.items.is_empty())
                        .on_click({
                            move |_, _, cx| {
                                lsp_store
                                    .update(cx, |lsp_store, cx| {
                                        lsp_store.stop_all_language_servers(cx);
                                    })
                                    .ok();
                            }
                        }),
                )
                .into_any_element(),
        )
    }
}

// TODO kb keyboard story
impl LspTool {
    pub fn new(workspace: &Workspace, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let settings_subscription =
            cx.observe_global_in::<SettingsStore>(window, move |lsp_tool, window, cx| {
                if ProjectSettings::get_global(cx).global_lsp_settings.button {
                    if lsp_tool.lsp_picker.is_none() {
                        lsp_tool.lsp_picker =
                            Some(Self::new_lsp_picker(lsp_tool.state.clone(), window, cx));
                        cx.notify();
                        return;
                    }
                } else if lsp_tool.lsp_picker.take().is_some() {
                    cx.notify();
                }
            });

        let lsp_store = workspace.project().read(cx).lsp_store();
        let lsp_store_subscription =
            cx.subscribe_in(&lsp_store, window, |lsp_tool, _, e, window, cx| {
                lsp_tool.on_lsp_store_event(e, window, cx)
            });

        let state = cx.new(|_| PickerState {
            workspace: workspace.weak_handle(),
            lsp_store: lsp_store.downgrade(),
            active_editor: None,
            language_servers: LanguageServers::default(),
        });

        Self {
            state,
            lsp_picker: None,
            _subscriptions: vec![settings_subscription, lsp_store_subscription],
        }
    }

    fn on_lsp_store_event(
        &mut self,
        e: &LspStoreEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(lsp_picker) = self.lsp_picker.clone() else {
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
                        self.state.update(cx, |state, _| {
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
                        self.state.update(cx, |state, _| {
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
                self.state.update(cx, |state, _| {
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
            lsp_picker.update(cx, |lsp_picker, cx| {
                lsp_picker.refresh(window, cx);
            });
        }
    }

    fn new_lsp_picker(
        state: Entity<PickerState>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<Picker<LspPickerDelegate>> {
        cx.new(|cx| {
            let mut delegate = LspPickerDelegate {
                selected_index: 0,
                other_servers_start_index: None,
                items: Vec::new(),
                state,
            };
            delegate.regenerate_items(cx);
            Picker::list(delegate, window, cx)
        })
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
                        .state
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
                                lsp_tool.state.update(cx, |state, cx| {
                                    if let Some(active_editor) = state.active_editor.as_mut() {
                                        let buffer_id = buffer.read(cx).remote_id();
                                        if active_editor.editor_buffers.insert(buffer_id) {
                                            if let Some(picker) = &lsp_tool.lsp_picker {
                                                picker.update(cx, |picker, cx| {
                                                    picker.refresh(window, cx)
                                                });
                                            }
                                        }
                                    }
                                });
                            }
                            EditorEvent::ExcerptsRemoved {
                                removed_buffer_ids, ..
                            } => {
                                lsp_tool.state.update(cx, |state, cx| {
                                    if let Some(active_editor) = state.active_editor.as_mut() {
                                        let mut removed = false;
                                        for id in removed_buffer_ids {
                                            active_editor.editor_buffers.retain(|buffer_id| {
                                                let retain = buffer_id != id;
                                                removed |= !retain;
                                                retain
                                            });
                                        }
                                        if removed {
                                            if let Some(picker) = &lsp_tool.lsp_picker {
                                                picker.update(cx, |picker, cx| {
                                                    picker.refresh(window, cx)
                                                });
                                            }
                                        }
                                    }
                                });
                            }
                            _ => {}
                        },
                    );
                    self.state.update(cx, |state, _| {
                        state.active_editor = Some(ActiveEditor {
                            editor: editor.downgrade(),
                            _editor_subscription,
                            editor_buffers,
                        });
                    });

                    let lsp_picker = Self::new_lsp_picker(self.state.clone(), window, cx);
                    self.lsp_picker = Some(lsp_picker.clone());
                    lsp_picker.update(cx, |lsp_picker, cx| lsp_picker.refresh(window, cx));
                }
            } else if self.state.read(cx).active_editor.is_some() {
                self.state.update(cx, |state, _| {
                    state.active_editor = None;
                });
                if let Some(lsp_picker) = self.lsp_picker.as_ref() {
                    lsp_picker.update(cx, |lsp_picker, cx| {
                        lsp_picker.refresh(window, cx);
                    });
                };
            }
        } else if self.state.read(cx).active_editor.is_some() {
            self.state.update(cx, |state, _| {
                state.active_editor = None;
            });
            if let Some(lsp_picker) = self.lsp_picker.as_ref() {
                lsp_picker.update(cx, |lsp_picker, cx| {
                    lsp_picker.refresh(window, cx);
                });
            }
        }
    }
}

impl Render for LspTool {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl ui::IntoElement {
        if !cx.is_staff() || self.state.read(cx).language_servers.is_empty() {
            return div();
        }

        let Some(lsp_picker) = self.lsp_picker.clone() else {
            return div();
        };

        let mut has_errors = false;
        let mut has_warnings = false;
        let mut has_other_notifications = false;
        let state = self.state.read(cx);
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

        div().child(
            PickerPopoverMenu::new(
                lsp_picker.clone(),
                IconButton::new("zed-lsp-tool-button", IconName::BoltFilledAlt)
                    .when_some(indicator, IconButton::indicator)
                    .icon_size(IconSize::Small)
                    .indicator_border_color(Some(cx.theme().colors().status_bar_background)),
                move |_, cx| Tooltip::simple("Language Servers", cx),
                Corner::BottomLeft,
                cx,
            )
            .render(window, cx),
        )
    }
}
