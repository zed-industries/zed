use std::{
    collections::{BTreeMap, BTreeSet, hash_map},
    sync::Arc,
    time::Duration,
};

use client::{proto, zed_urls};
use collections::{HashMap, HashSet};
use editor::{
    Editor, EditorEvent,
    actions::{RestartLanguageServer, StopLanguageServer},
};
use gpui::{Corner, DismissEvent, Entity, Focusable, MouseButton, Subscription, Task, WeakEntity};
use itertools::Itertools;
use language::BufferId;
use lsp::{LanguageServerId, LanguageServerName};
use picker::{Picker, PickerDelegate, popover_menu::PickerPopoverMenu};
use project::{LspStore, LspStoreEvent, WorktreeId, project_settings::ProjectSettings};
use settings::{Settings as _, SettingsStore};
use ui::{Context, IconButtonShape, Indicator, KeyBinding, Tooltip, Window, prelude::*};
use util::truncate_and_trailoff;
use workspace::{StatusItemView, Workspace};

use crate::{LogStore, lsp_log::GlobalLogStore};

pub struct LspTool {
    workspace: WeakEntity<Workspace>,
    lsp_store: WeakEntity<LspStore>,
    active_editor: Option<ActiveEditor>,
    language_servers: LanguageServers,
    lsp_picker: Option<Entity<Picker<LspPickerDelegate>>>,
    _subscriptions: Vec<Subscription>,
}

struct ActiveEditor {
    editor: WeakEntity<Editor>,
    _editor_subscription: Subscription,
    editor_buffers: HashSet<(WorktreeId, BufferId)>,
}

struct LspPickerDelegate {
    language_servers: LanguageServers,
    active_editor: WeakEntity<Editor>,
    workspace: WeakEntity<Workspace>,
    lsp_store: WeakEntity<LspStore>,
    lsp_logs: WeakEntity<LogStore>,
    editor_buffers: HashSet<(WorktreeId, BufferId)>,
    selected_index: usize,
    items: Vec<LspItem>,
}

#[derive(Debug)]
enum LspItem {
    Header {
        server_id: LanguageServerId,
        server_name: LanguageServerName,
        status: LanguageServerStatus,
        message: Option<(SharedString, Severity)>,
    },
    Item {
        server_id: LanguageServerId,
        status: LanguageServerStatus,
    },
}

#[derive(Debug, Default, Clone)]
struct LanguageServers {
    servers: HashMap<LanguageServerId, LanguageServerState>,
    // TODO kb all wrong: `BufferId` is not persistent across e.g. file reopens; need to use PathBuf
    servers_per_worktree: HashMap<WorktreeId, HashMap<BufferId, HashSet<LanguageServerId>>>,
}

impl LanguageServers {
    fn remove(&mut self, id: LanguageServerId) {
        self.servers.remove(&id);
        self.servers_per_worktree.retain(|_, worktree_servers| {
            worktree_servers.retain(|_, servers| {
                servers.remove(&id);
                !servers.is_empty()
            });
            !worktree_servers.is_empty()
        });
    }

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
                        id,
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
                        .or_insert_with(|| BTreeMap::new())
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
}

#[derive(Debug, Clone)]
struct LanguageServerState {
    id: LanguageServerId,
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

impl LspPickerDelegate {
    fn render_server_header(
        &self,
        server_id: LanguageServerId,
        language_server_name: &LanguageServerName,
        status: LanguageServerStatus,
        lsp_status: &Option<(SharedString, Severity)>,
        cx: &mut Context<Picker<Self>>,
    ) -> Div {
        let lsp_store = self.lsp_store.clone();
        let Ok(buffer_store) = self.workspace.update(cx, |workspace, cx| {
            workspace.project().read(cx).buffer_store().clone()
        }) else {
            return div();
        };

        let buffers = self
            .editor_buffers
            .iter()
            .flat_map(|(_, buffer_id)| buffer_store.read(cx).get(*buffer_id))
            .collect::<Vec<_>>();

        let restart_button = IconButton::new("restart-server", IconName::Rerun)
            .icon_size(IconSize::Small)
            .size(ButtonSize::Compact)
            .icon_color(Color::Default)
            .shape(ui::IconButtonShape::Square)
            .tooltip(move |window, cx| {
                Tooltip::for_action(
                    format!("Restart server ({status:?})"),
                    &RestartLanguageServer,
                    window,
                    cx,
                )
            })
            .when(!buffers.is_empty(), |button| {
                button.on_click({
                    move |_, _, cx| {
                        lsp_store
                            .update(cx, |lsp_store, cx| {
                                lsp_store.restart_language_servers_for_buffers(
                                    buffers.clone(),
                                    vec![server_id],
                                    cx,
                                )
                            })
                            .ok();
                    }
                })
            });
        let (icon, icon_color) = match status {
            LanguageServerStatus::Running => (IconName::Play, Color::Success),
            LanguageServerStatus::Starting => (IconName::Play, Color::Modified),
            LanguageServerStatus::Stopping => (IconName::StopFilled, Color::Modified),
            LanguageServerStatus::Stopped => (IconName::StopFilled, Color::Disabled),
        };

        v_flex()
            .p_1()
            .w_full()
            .group("lsp-status")
            .child(
                h_flex()
                    .w_full()
                    .gap_2()
                    .child(
                        h_flex()
                            .group("lsp-status")
                            .child(
                                div()
                                    .hover(|style| style.invisible().w_0())
                                    .child(Icon::new(icon).color(icon_color)),
                            )
                            .child(
                                div()
                                    .absolute()
                                    .visible_on_hover("lsp-status")
                                    .child(restart_button),
                            ),
                    )
                    .child(Label::new(language_server_name.0.clone()).color(Color::Muted)),
            )
            .when_some(lsp_status.as_ref(), |header, (message, severity)| {
                header.child(Self::render_server_message(
                    server_id, message, severity, cx,
                ))
            })
            .cursor_default()
    }

    fn render_server_actions(
        &self,
        server_id: LanguageServerId,
        status: LanguageServerStatus,
        cx: &mut Context<'_, Picker<Self>>,
    ) -> Div {
        let lsp_logs = self.lsp_logs.clone();
        let can_stop = match status {
            LanguageServerStatus::Starting | LanguageServerStatus::Running => true,
            LanguageServerStatus::Stopping | LanguageServerStatus::Stopped => false,
        };
        let Ok(has_logs) = lsp_logs.update(cx, |lsp_logs, _| {
            lsp_logs.get_language_server_state(server_id).is_some()
        }) else {
            return div();
        };

        h_flex()
            .w_full()
            .gap_2()
            .when(can_stop, |div| {
                div.child(
                    IconButton::new("stop-server", IconName::StopFilled)
                        .tooltip(|_, cx| Tooltip::simple("Stop server", cx))
                        .on_click({
                            let lsp_store = self.lsp_store.clone();
                            move |_, _, cx| {
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
                        }),
                )
            })
            .when(!can_stop, |div| {
                let Ok(buffer_store) = self.workspace.update(cx, |workspace, cx| {
                    workspace.project().read(cx).buffer_store().clone()
                }) else {
                    return div;
                };
                let buffers = self
                    .editor_buffers
                    .iter()
                    .flat_map(|(_, buffer_id)| buffer_store.read(cx).get(*buffer_id))
                    .collect::<Vec<_>>();
                if buffers.is_empty() {
                    return div;
                }

                div.child(
                    IconButton::new("restart-server", IconName::Rerun)
                        .tooltip(|_, cx| Tooltip::simple("Restart server", cx))
                        .on_click({
                            let lsp_store = self.lsp_store.clone();
                            move |_, _, cx| {
                                lsp_store
                                    .update(cx, |lsp_store, cx| {
                                        lsp_store.restart_language_servers_for_buffers(
                                            buffers.clone(),
                                            vec![server_id],
                                            cx,
                                        );
                                    })
                                    .ok();
                            }
                        }),
                )
            })
            .when(has_logs, |div| {
                div.child(
                    IconButton::new("open-server-log", IconName::ListX)
                        .tooltip(|_, cx| Tooltip::simple("Open Log", cx))
                        .on_click({
                            let workspace = self.workspace.clone();
                            let lsp_logs = self.lsp_logs.clone();
                            move |_, window, cx| {
                                lsp_logs
                                    .update(cx, |lsp_logs, cx| {
                                        lsp_logs.open_server_log(
                                            workspace.clone(),
                                            server_id,
                                            window,
                                            cx,
                                        );
                                    })
                                    .ok();
                            }
                        }),
                )
                .child(
                    IconButton::new("open-lsp-messages", IconName::BoltFilled)
                        .icon_size(IconSize::Small)
                        .tooltip(|_, cx| Tooltip::simple("Open LSP messages", cx))
                        .on_click({
                            let workspace = self.workspace.clone();
                            let lsp_logs = self.lsp_logs.clone();
                            move |_, window, cx| {
                                lsp_logs
                                    .update(cx, |lsp_logs, cx| {
                                        // TODO kb none of the open_* methods focus the log input
                                        // TODO kb rpc logs are not synced remotely?
                                        lsp_logs.open_server_trace(
                                            workspace.clone(),
                                            server_id,
                                            window,
                                            cx,
                                        );
                                    })
                                    .ok();
                            }
                        }),
                )
            })
    }

    fn render_server_message(
        server_id: LanguageServerId,
        message: &SharedString,
        severity: &Severity,
        cx: &Context<'_, Picker<Self>>,
    ) -> AnyElement {
        let full_label_message = message.trim();
        let shortened_message = truncate_and_trailoff(full_label_message, 30);
        let tooltip = if full_label_message == shortened_message {
            None
        } else if full_label_message == message.as_ref() {
            Some(message.clone())
        } else {
            Some(SharedString::new(full_label_message))
        };

        h_flex()
            .id("server-message")
            .justify_center()
            .child(Label::new(shortened_message))
            .when_some(tooltip, |div, tooltip| {
                div.tooltip(move |_, cx| Tooltip::simple(tooltip.clone(), cx))
            })
            .hover(|s| s.opacity(0.6))
            .map(|div| match severity {
                Severity::Other | Severity::Ok | Severity::Info => div,
                Severity::Warning => div.border_1().border_color(Color::Warning.color(cx)),
                Severity::Error => div.border_1().border_color(Color::Error.color(cx)),
            })
            .cursor_pointer()
            .on_mouse_down(MouseButton::Left, {
                let message = message.clone();
                let severity = *severity;
                cx.listener(move |picker, _, _, cx| {
                    if let Some(server_state) =
                        picker.delegate.language_servers.servers.get_mut(&server_id)
                    {
                        if server_state.message.as_ref().is_some_and(
                            |(state_message, state_severity)| {
                                state_message == &message && state_severity == &severity
                            },
                        ) {
                            server_state.message = None;
                        }
                    }
                    if let Some(state_message) =
                        picker
                            .delegate
                            .items
                            .iter_mut()
                            .find_map(|item| match item {
                                LspItem::Header {
                                    server_id: state_server_id,
                                    message: state_message,
                                    ..
                                } => {
                                    if server_id == *state_server_id
                                        && state_message.as_ref().is_some_and(
                                            |(state_message, state_severity)| {
                                                state_message == &message
                                                    && state_severity == &severity
                                            },
                                        )
                                    {
                                        Some(state_message)
                                    } else {
                                        None
                                    }
                                }
                                LspItem::Item { .. } => None,
                            })
                    {
                        *state_message = None;
                        cx.notify();
                    }
                })
            })
            .into_any_element()
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
                .update(cx, |lsp_picker, _| {
                    lsp_picker.delegate.selected_index = 0;
                    lsp_picker.delegate.items = lsp_picker
                        .delegate
                        .language_servers
                        .servers
                        .values()
                        // TODO kb return back when stable IDs are back
                        // .iter()
                        // .filter_map(|(worktree_id, buffer_id)| {
                        //     Some(
                        //         lsp_picker
                        //             .delegate
                        //             .language_servers
                        //             .servers_per_worktree
                        //             .get(worktree_id)?
                        //             .get(buffer_id)?,
                        //     )
                        // })
                        // .flatten()
                        // .unique()
                        // .filter_map(|id| lsp_picker.delegate.language_servers.servers.get(id))
                        .sorted_by_key(|state| state.name.clone())
                        .flat_map(|state| {
                            [
                                LspItem::Header {
                                    server_id: state.id,
                                    server_name: state.name.clone(),
                                    status: state.status,
                                    message: state.message.clone(),
                                },
                                LspItem::Item {
                                    server_id: state.id,
                                    status: state.status,
                                },
                            ]
                        })
                        .collect();
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
        Some(
            match self.items.get(ix)? {
                LspItem::Header {
                    server_id,
                    server_name,
                    status,
                    message,
                } => self.render_server_header(*server_id, server_name, *status, message, cx),
                LspItem::Item { server_id, status } => {
                    self.render_server_actions(*server_id, *status, cx)
                }
            }
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

    fn render_header(
        &self,
        _window: &mut Window,
        _: &mut Context<Picker<Self>>,
    ) -> Option<AnyElement> {
        Some(
            Button::new("lsp-tool-header", "Active language servers")
                .full_width()
                .on_click(|_, _, cx| cx.open_url(&zed_urls::language_docs_url(cx)))
                .into_any_element(),
        )
    }

    fn render_footer(
        &self,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<AnyElement> {
        let editor = self.active_editor.clone();
        Some(
            h_flex()
                .w_full()
                .justify_between()
                .border_t_1()
                .border_color(cx.theme().colors().border_variant)
                .child(
                    Button::new("restart-all-servers", "Restart all servers")
                        .key_binding(KeyBinding::for_action(&RestartLanguageServer, window, cx))
                        .on_click({
                            let editor = editor.clone();
                            move |_, window, cx| {
                                editor
                                    .update(cx, |editor, cx| {
                                        editor.restart_language_server(
                                            &RestartLanguageServer,
                                            window,
                                            cx,
                                        );
                                    })
                                    .ok();
                            }
                        }),
                )
                .child(
                    Button::new("stop-all-servers", "Stop all servers")
                        .key_binding(KeyBinding::for_action(&StopLanguageServer, window, cx))
                        .on_click({
                            let editor = editor.clone();
                            move |_, window, cx| {
                                editor
                                    .update(cx, |editor, cx| {
                                        editor.stop_language_server(
                                            &StopLanguageServer,
                                            window,
                                            cx,
                                        );
                                    })
                                    .ok();
                            }
                        }),
                )
                .into_any_element(),
        )
    }

    fn separators_after_indices(&self) -> Vec<usize> {
        self.items
            .iter()
            .enumerate()
            .filter_map(|(i, item)| match item {
                LspItem::Header { .. } => None,
                LspItem::Item { .. } => Some(i),
            })
            .collect()
    }
}

impl LspTool {
    pub fn new(workspace: &Workspace, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let lsp_store = workspace.project().read(cx).lsp_store();

        let settings_workspace = workspace.weak_handle();
        let settings_lsp_store = lsp_store.downgrade();
        let settings_subscription =
            cx.observe_global_in::<SettingsStore>(window, move |lsp_tool, window, cx| {
                if ProjectSettings::get_global(cx).global_lsp_settings.button {
                    if let Some(active_editor) = lsp_tool.active_editor.as_ref() {
                        lsp_tool.lsp_picker = Some(Self::new_lsp_picker(
                            settings_workspace.clone(),
                            settings_lsp_store.clone(),
                            active_editor.editor.clone(),
                            active_editor.editor_buffers.clone(),
                            lsp_tool.language_servers.clone(),
                            window,
                            cx,
                        ));
                        cx.notify();
                        return;
                    }
                }

                if lsp_tool.lsp_picker.take().is_some() {
                    cx.notify();
                }
            });
        let lsp_store_subscription =
            cx.subscribe_in(&lsp_store, window, |lsp_tool, _, e, window, cx| {
                lsp_tool.on_lsp_store_event(e, window, cx)
            });

        Self {
            workspace: workspace.weak_handle(),
            lsp_store: lsp_store.downgrade(),
            active_editor: None,
            lsp_picker: None,
            _subscriptions: vec![settings_subscription, lsp_store_subscription],
            language_servers: LanguageServers::default(),
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

        let mut updated = true;
        match e {
            project::LspStoreEvent::LanguageServerUpdate {
                language_server_id,
                name,
                message: proto::update_language_server::Variant::StatusUpdate(status_update),
            } => match proto::status_update::Status::from_i32(status_update.status) {
                Some(proto::status_update::Status::Starting) => {
                    self.update_status(
                        *language_server_id,
                        LanguageServerStatus::Starting,
                        status_update.message.as_deref(),
                        name.as_ref(),
                        cx,
                    );
                }
                Some(proto::status_update::Status::Running) => {
                    self.update_status(
                        *language_server_id,
                        LanguageServerStatus::Running,
                        status_update.message.as_deref(),
                        name.as_ref(),
                        cx,
                    );
                }
                Some(proto::status_update::Status::Stopping) => {
                    self.update_status(
                        *language_server_id,
                        LanguageServerStatus::Stopping,
                        status_update.message.as_deref(),
                        name.as_ref(),
                        cx,
                    );
                }
                Some(proto::status_update::Status::Stopped) => {
                    self.update_status(
                        *language_server_id,
                        LanguageServerStatus::Stopped,
                        status_update.message.as_deref(),
                        name.as_ref(),
                        cx,
                    );
                }

                Some(proto::status_update::Status::Ok) => {
                    self.update_message(
                        *language_server_id,
                        status_update.message.as_deref(),
                        Severity::Ok,
                        name.as_ref(),
                        cx,
                    );
                }
                Some(proto::status_update::Status::Info) => {
                    self.update_message(
                        *language_server_id,
                        status_update.message.as_deref(),
                        Severity::Info,
                        name.as_ref(),
                        cx,
                    );
                }
                Some(proto::status_update::Status::Warning) => {
                    self.update_message(
                        *language_server_id,
                        status_update.message.as_deref(),
                        Severity::Warning,
                        name.as_ref(),
                        cx,
                    );
                }
                Some(proto::status_update::Status::Error) => {
                    self.update_message(
                        *language_server_id,
                        status_update.message.as_deref(),
                        Severity::Error,
                        name.as_ref(),
                        cx,
                    );
                }
                Some(proto::status_update::Status::Other) => {
                    self.update_message(
                        *language_server_id,
                        status_update.message.as_deref(),
                        Severity::Other,
                        name.as_ref(),
                        cx,
                    );
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
                if let Ok(buffer_id) = BufferId::new(update.buffer_id) {
                    self.language_servers
                        .servers_per_worktree
                        .entry(WorktreeId::from_proto(update.worktree_id))
                        .or_default()
                        .entry(buffer_id)
                        .or_default()
                        .insert(*language_server_id);
                    if let Some(picker) = &self.lsp_picker {
                        picker.update(cx, |picker, _| {
                            picker
                                .delegate
                                .language_servers
                                .servers_per_worktree
                                .entry(WorktreeId::from_proto(update.worktree_id))
                                .or_default()
                                .entry(buffer_id)
                                .or_default()
                                .insert(*language_server_id);
                        })
                    }
                }
            }
            _ => updated = false,
        };

        if updated {
            lsp_picker.update(cx, |lsp_picker, cx| {
                lsp_picker.refresh(window, cx);
            })
        }
    }

    fn new_lsp_picker(
        workspace: WeakEntity<Workspace>,
        lsp_store: WeakEntity<LspStore>,
        active_editor: WeakEntity<Editor>,
        editor_buffers: HashSet<(WorktreeId, BufferId)>,
        language_servers: LanguageServers,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<Picker<LspPickerDelegate>> {
        cx.new(|cx| {
            Picker::list(
                LspPickerDelegate {
                    selected_index: 0,
                    items: Vec::new(),
                    active_editor,
                    editor_buffers,
                    language_servers,
                    workspace,
                    lsp_store,
                    lsp_logs: cx.global::<GlobalLogStore>().0.downgrade(),
                },
                window,
                cx,
            )
        })
    }

    fn update_message(
        &mut self,
        id: LanguageServerId,
        message: Option<&str>,
        severity: Severity,
        name: Option<&LanguageServerName>,
        cx: &mut App,
    ) {
        if let Some(state) = self.language_servers.servers.get_mut(&id) {
            state.message =
                message.map(|message| (SharedString::from(message.to_owned()), severity));
            if let Some(name) = name.cloned() {
                state.name = name;
            }
        } else if let Some(message) = message {
            // TODO kb return back?
            // debug_panic!(
            //     "No server state for {id}, but got a message: {message} with severity: {severity:?}"
            // );
        }

        if let Some(picker) = &self.lsp_picker {
            picker.update(cx, |picker, _| {
                if let Some(state) = picker.delegate.language_servers.servers.get_mut(&id) {
                    state.message =
                        message.map(|message| (SharedString::from(message.to_owned()), severity));
                    if let Some(name) = name.cloned() {
                        state.name = name;
                    }
                }
            });
        }
    }

    fn update_status(
        &mut self,
        id: LanguageServerId,
        status: LanguageServerStatus,
        message: Option<&str>,
        name: Option<&LanguageServerName>,
        cx: &mut App,
    ) {
        self.language_servers
            .update_status(id, status, message, name.cloned());
        if let Some(picker) = &self.lsp_picker {
            picker.update(cx, |picker, _| {
                picker
                    .delegate
                    .language_servers
                    .update_status(id, status, message, name.cloned());
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
                        .active_editor
                        .as_ref()
                        .and_then(|active_editor| active_editor.editor.upgrade())
                        .as_ref()
                {
                    let editor_buffers = editor
                        .read(cx)
                        .buffer()
                        .read(cx)
                        .all_buffers()
                        .into_iter()
                        .filter_map(|buffer| {
                            let buffer = buffer.read(cx);
                            Some((buffer.file()?.worktree_id(cx), buffer.remote_id()))
                        })
                        .collect::<HashSet<_>>();
                    self.active_editor = Some(ActiveEditor {
                        editor: editor.downgrade(),
                        _editor_subscription: cx.subscribe_in(
                            &editor,
                            window,
                            |lsp_tool, _, e: &EditorEvent, window, cx| match e {
                                EditorEvent::ExcerptsAdded { buffer, .. } => {
                                    if let Some(worktree_id) =
                                        buffer.read(cx).file().map(|f| f.worktree_id(cx))
                                    {
                                        if let Some(active_editor) = lsp_tool.active_editor.as_mut()
                                        {
                                            let buffer_id = buffer.read(cx).remote_id();
                                            if active_editor
                                                .editor_buffers
                                                .insert((worktree_id, buffer_id))
                                            {
                                                if let Some(picker) = &lsp_tool.lsp_picker {
                                                    picker.update(cx, |picker, cx| {
                                                        picker
                                                            .delegate
                                                            .editor_buffers
                                                            .insert((worktree_id, buffer_id));
                                                        picker.refresh(window, cx)
                                                    });
                                                }
                                            }
                                        }
                                    }
                                }
                                EditorEvent::ExcerptsRemoved {
                                    removed_buffer_ids, ..
                                } => {
                                    if let Some(active_editor) = lsp_tool.active_editor.as_mut() {
                                        let mut removed = false;
                                        for id in removed_buffer_ids {
                                            active_editor.editor_buffers.retain(
                                                |(_, buffer_id)| {
                                                    let retain = buffer_id != id;
                                                    removed |= !retain;
                                                    retain
                                                },
                                            );
                                        }
                                        if removed {
                                            if let Some(picker) = &lsp_tool.lsp_picker {
                                                picker.update(cx, |picker, cx| {
                                                    for id in removed_buffer_ids {
                                                        picker.delegate.editor_buffers.retain(
                                                            |(_, buffer_id)| buffer_id != id,
                                                        );
                                                    }
                                                    picker.refresh(window, cx)
                                                });
                                            }
                                        }
                                    }
                                }
                                _ => {}
                            },
                        ),
                        editor_buffers: editor_buffers.clone(),
                    });

                    let lsp_picker = Self::new_lsp_picker(
                        self.workspace.clone(),
                        self.lsp_store.clone(),
                        editor.downgrade(),
                        editor_buffers,
                        self.language_servers.clone(),
                        window,
                        cx,
                    );
                    self.lsp_picker = Some(lsp_picker.clone());
                    lsp_picker.update(cx, |lsp_picker, cx| lsp_picker.refresh(window, cx));
                    return;
                }
            }
        }

        self.active_editor = None;
        self.lsp_picker = None;
    }
}

impl Render for LspTool {
    // TODO kb keyboard story: toggling the button; navigation inside it; showing keybindings (need new actions?) for each button
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl ui::IntoElement {
        let Some(lsp_picker) = &self.lsp_picker else {
            return div();
        };

        let delegate = &lsp_picker.read(cx).delegate;
        if self.active_editor.is_none() || delegate.items.is_empty() {
            return div();
        }

        let mut has_errors = false;
        let mut has_warnings = false;
        for item in &delegate.items {
            match item {
                LspItem::Header {
                    message: Some((_, Severity::Error)),
                    ..
                } => has_errors = true,
                LspItem::Header {
                    message: Some((_, Severity::Warning)),
                    ..
                } => has_warnings = true,
                _ => {}
            }
        }
        let indicator = if has_errors {
            Some(Indicator::dot().color(Color::Error))
        } else if has_warnings {
            Some(Indicator::dot().color(Color::Warning))
        } else {
            None
        };

        div().child(
            PickerPopoverMenu::new(
                lsp_picker.clone(),
                IconButton::new("zed-lsp-tool-button", IconName::Bolt)
                    .when_some(indicator, IconButton::indicator)
                    .shape(IconButtonShape::Square)
                    .icon_size(IconSize::XSmall)
                    .indicator_border_color(Some(cx.theme().colors().status_bar_background)),
                move |_, cx| Tooltip::simple("Language servers", cx),
                Corner::BottomRight,
                cx,
            )
            .render(window, cx),
        )
    }
}
