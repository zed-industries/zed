use std::{
    collections::{BTreeMap, BTreeSet, hash_map},
    sync::Arc,
    time::Duration,
};

use client::proto;
use collections::{HashMap, HashSet};
use editor::{
    Editor, EditorEvent,
    actions::{RestartLanguageServer, StopLanguageServer},
};
use gpui::{Corner, DismissEvent, Entity, Focusable, Subscription, Task, WeakEntity};
use itertools::Itertools;
use language::BufferId;
use lsp::{LanguageServerId, LanguageServerName};
use picker::{Picker, PickerDelegate, popover_menu::PickerPopoverMenu};
use project::{LspStore, LspStoreEvent, WorktreeId};
use ui::{Context, IconButtonShape, Indicator, KeyBinding, Tooltip, Window, prelude::*};
use util::debug_panic;
use workspace::{StatusItemView, Workspace};

pub struct LspTool {
    lsp_picker: Entity<Picker<LspPickerDelegate>>,
}

struct ActiveEditor {
    editor: WeakEntity<Editor>,
    _editor_subscription: Subscription,
    editor_buffers: HashSet<(WorktreeId, BufferId)>,
}

struct LspPickerDelegate {
    language_servers: LanguageServers,
    active_editor: Option<ActiveEditor>,
    lsp_store: Entity<LspStore>,
    _lsp_store_subscription: Subscription,
    selected_index: usize,
    items: Vec<LspItem>,
}

#[derive(Debug)]
enum LspItem {
    Header(LanguageServerName, Option<(SharedString, Severity)>),
    Item(LanguageServerStatus),
}

#[derive(Debug, Default)]
struct LanguageServers {
    servers: HashMap<LanguageServerId, LanguageServerState>,
    servers_per_worktree: HashMap<WorktreeId, HashMap<BufferId, HashSet<LanguageServerId>>>,
}

#[derive(Debug)]
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

impl LspPickerDelegate {
    fn remove_server(&mut self, id: LanguageServerId) {
        self.language_servers.servers.remove(&id);
        self.language_servers
            .servers_per_worktree
            .retain(|_, worktree_servers| {
                worktree_servers.retain(|_, servers| {
                    servers.remove(&id);
                    !servers.is_empty()
                });
                !worktree_servers.is_empty()
            });
    }

    fn update_message(
        &mut self,
        id: LanguageServerId,
        message: Option<&str>,
        severity: Severity,
        name: Option<&LanguageServerName>,
    ) {
        if let Some(state) = self.language_servers.servers.get_mut(&id) {
            state.message =
                message.map(|message| (SharedString::from(message.to_owned()), severity));
            if let Some(name) = name.cloned() {
                state.name = name;
            }
        } else if let Some(message) = message {
            debug_panic!(
                "No server state for {id}, but got a message: {message} with severity: {severity:?}"
            );
        }
    }

    fn update_status(
        &mut self,
        id: LanguageServerId,
        status: LanguageServerStatus,
        message: Option<&str>,
        name: Option<&LanguageServerName>,
    ) {
        match self.language_servers.servers.entry(id) {
            hash_map::Entry::Occupied(mut o) => {
                let state = o.get_mut();
                if let Some(name) = name.cloned() {
                    state.name = name;
                }
                state.status = status;
                state.message = message
                    .map(|message| (SharedString::from(message.to_owned()), Severity::Other));
            }
            hash_map::Entry::Vacant(v) => {
                if let Some(name) = name.cloned() {
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

        let duplicate_server_statuses = self.language_servers.servers.iter().fold(
            HashMap::default(),
            |mut acc, (id, state)| {
                acc.entry(state.name.clone())
                    .or_insert_with(|| BTreeMap::new())
                    .insert(*id, state.status);
                acc
            },
        );

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
                        self.remove_server(id);
                    }
                }
            } else {
                for id in stopped_servers {
                    self.remove_server(id);
                }
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
                    lsp_picker.delegate.items.clear();
                    lsp_picker.delegate.selected_index = 0;

                    let Some(buffers) = lsp_picker
                        .delegate
                        .active_editor
                        .as_ref()
                        .and_then(|editor| Some(&editor.editor_buffers))
                    else {
                        return;
                    };

                    lsp_picker.delegate.items = buffers
                        .iter()
                        .filter_map(|(worktree_id, buffer_id)| {
                            Some(
                                lsp_picker
                                    .delegate
                                    .language_servers
                                    .servers_per_worktree
                                    .get(worktree_id)?
                                    .get(buffer_id)?,
                            )
                        })
                        .flatten()
                        .unique()
                        .filter_map(|id| {
                            let adapter = lsp_picker
                                .delegate
                                .lsp_store
                                .read(cx)
                                .language_server_adapter_for_id(*id)?;
                            let state = lsp_picker.delegate.language_servers.servers.get(id)?;
                            Some((adapter, state))
                        })
                        .flat_map(|(adapter, state)| {
                            [
                                LspItem::Header(adapter.name(), state.message.clone()),
                                LspItem::Item(state.status),
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
        _: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        Some(
            match self.items.get(ix)? {
                LspItem::Header(language_server_name, lsp_status) => v_flex()
                    .justify_center()
                    .child(
                        h_flex()
                            .w_full()
                            .justify_center()
                            .child(Label::new(language_server_name.0.clone()).color(Color::Muted)),
                    )
                    .when_some(lsp_status.as_ref(), |header, (message, severity)| {
                        header.child(
                            Label::new(format!("TODO kb status: {message} | {severity:?}"))
                                .color(Color::Warning),
                        )
                    })
                    .cursor_default(),
                LspItem::Item(status) => h_flex()
                    .gap_2()
                    .justify_between()
                    .child(Label::new(format!("{status:?}")))
                    .child(
                        Button::new("open-server-log", "Open Log").on_click(move |_, _, _| {
                            dbg!("TODO kb: open log");
                        }),
                    )
                    .child(
                        Button::new("restart-server", "Restart").on_click(move |_, _, _| {
                            dbg!("TODO kb: restart");
                        }),
                    )
                    .child(
                        Button::new("disable-server", "Disable").on_click(move |_, _, _| {
                            dbg!("TODO kb: disable");
                        }),
                    ),
            }
            .into_any_element(),
        )
    }

    fn can_select(
        &mut self,
        _ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> bool {
        true
    }

    fn render_editor(
        &self,
        editor: &Entity<Editor>,
        _: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Div {
        div().child(div().track_focus(&editor.focus_handle(cx)))
    }

    fn render_footer(
        &self,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<AnyElement> {
        let editor = self.active_editor.as_ref().map(|e| &e.editor)?;
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
}

impl LspTool {
    pub fn new(workspace: &Workspace, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let lsp_store = workspace.project().read(cx).lsp_store();
        let lsp_store_subscription =
            cx.subscribe_in(&lsp_store, window, |lsp_tool, _, e, window, cx| {
                lsp_tool.on_lsp_store_event(e, window, cx)
            });

        Self {
            lsp_picker: cx.new(|cx| {
                Picker::uniform_list(
                    LspPickerDelegate {
                        selected_index: 0,
                        items: Vec::new(),
                        language_servers: LanguageServers::default(),
                        active_editor: None,
                        lsp_store,
                        _lsp_store_subscription: lsp_store_subscription,
                    },
                    window,
                    cx,
                )
            }),
        }
    }

    fn on_lsp_store_event(
        &mut self,
        e: &LspStoreEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let mut updated = true;
        match e {
            project::LspStoreEvent::LanguageServerAdded(
                language_server_id,
                language_server_name,
                ..,
            ) => {
                self.lsp_picker.update(cx, |picker, _| {
                    match picker
                        .delegate
                        .language_servers
                        .servers
                        .entry(*language_server_id)
                    {
                        hash_map::Entry::Occupied(mut o) => {
                            let state = o.get_mut();
                            match state.status {
                                LanguageServerStatus::Running | LanguageServerStatus::Starting => {}
                                LanguageServerStatus::Stopping | LanguageServerStatus::Stopped => {
                                    state.status = LanguageServerStatus::Starting;
                                    state.message = None;
                                }
                            }
                        }
                        hash_map::Entry::Vacant(v) => {
                            v.insert(LanguageServerState {
                                name: language_server_name.clone(),
                                message: None,
                                status: LanguageServerStatus::Starting,
                            });
                        }
                    }
                });
            }
            project::LspStoreEvent::LanguageServerRemoved(language_server_id) => {
                self.lsp_picker.update(cx, |picker, _| {
                    picker.delegate.remove_server(*language_server_id);
                });
            }
            project::LspStoreEvent::LanguageServerUpdate {
                language_server_id,
                name,
                message: proto::update_language_server::Variant::StatusUpdate(status_update),
            } => {
                self.lsp_picker.update(
                    cx,
                    |picker, _| match proto::status_update::Status::from_i32(status_update.status) {
                        Some(proto::status_update::Status::Starting) => {
                            picker.delegate.update_status(
                                *language_server_id,
                                LanguageServerStatus::Starting,
                                status_update.message.as_deref(),
                                name.as_ref(),
                            );
                        }
                        Some(proto::status_update::Status::Running) => {
                            picker.delegate.update_status(
                                *language_server_id,
                                LanguageServerStatus::Running,
                                status_update.message.as_deref(),
                                name.as_ref(),
                            );
                        }
                        Some(proto::status_update::Status::Stopping) => {
                            picker.delegate.update_status(
                                *language_server_id,
                                LanguageServerStatus::Stopping,
                                status_update.message.as_deref(),
                                name.as_ref(),
                            );
                        }
                        Some(proto::status_update::Status::Stopped) => {
                            picker.delegate.update_status(
                                *language_server_id,
                                LanguageServerStatus::Stopped,
                                status_update.message.as_deref(),
                                name.as_ref(),
                            );
                        }

                        Some(proto::status_update::Status::Ok) => {
                            picker.delegate.update_message(
                                *language_server_id,
                                status_update.message.as_deref(),
                                Severity::Ok,
                                name.as_ref(),
                            );
                        }
                        Some(proto::status_update::Status::Info) => {
                            picker.delegate.update_message(
                                *language_server_id,
                                status_update.message.as_deref(),
                                Severity::Info,
                                name.as_ref(),
                            );
                        }
                        Some(proto::status_update::Status::Warning) => {
                            picker.delegate.update_message(
                                *language_server_id,
                                status_update.message.as_deref(),
                                Severity::Warning,
                                name.as_ref(),
                            );
                        }
                        Some(proto::status_update::Status::Error) => {
                            picker.delegate.update_message(
                                *language_server_id,
                                status_update.message.as_deref(),
                                Severity::Error,
                                name.as_ref(),
                            );
                        }
                        Some(proto::status_update::Status::Other) => {
                            picker.delegate.update_message(
                                *language_server_id,
                                status_update.message.as_deref(),
                                Severity::Other,
                                name.as_ref(),
                            );
                        }
                        None => {
                            log::error!("Unexpected status update {}", status_update.status);
                        }
                    },
                );
            }
            // TODO kb events are sent twice?
            project::LspStoreEvent::LanguageServerUpdate {
                language_server_id,
                message: proto::update_language_server::Variant::RegisteredForBuffer(update),
                ..
            } => {
                if let Ok(buffer_id) = BufferId::new(update.buffer_id) {
                    self.lsp_picker.update(cx, |picker, _| {
                        picker
                            .delegate
                            .language_servers
                            .servers_per_worktree
                            .entry(WorktreeId::from_proto(update.worktree_id))
                            .or_default()
                            .entry(buffer_id)
                            .or_default()
                            .insert(*language_server_id);
                    });
                }
            }
            _ => updated = false,
        };

        if updated {
            self.lsp_picker.update(cx, |lsp_picker, cx| {
                lsp_picker.refresh(window, cx);
            })
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
        let editor = active_pane_item.and_then(|item| item.downcast::<Editor>());
        self.lsp_picker.update(cx, |picker, cx| {
            picker.delegate.active_editor = editor.as_ref().map(|editor| ActiveEditor {
                editor: editor.downgrade(),
                _editor_subscription: cx.subscribe_in(
                    &editor,
                    window,
                    |picker, _, e: &EditorEvent, window, cx| match e {
                        EditorEvent::ExcerptsAdded { buffer, .. } => {
                            if let Some(worktree_id) =
                                buffer.read(cx).file().map(|f| f.worktree_id(cx))
                            {
                                if let Some(active_editor) = picker.delegate.active_editor.as_mut()
                                {
                                    if active_editor
                                        .editor_buffers
                                        .insert((worktree_id, buffer.read(cx).remote_id()))
                                    {
                                        picker.refresh(window, cx);
                                    }
                                }
                            }
                        }
                        EditorEvent::ExcerptsRemoved {
                            removed_buffer_ids, ..
                        } => {
                            if let Some(active_editor) = picker.delegate.active_editor.as_mut() {
                                let mut removed = false;
                                for id in removed_buffer_ids {
                                    active_editor.editor_buffers.retain(|(_, buffer_id)| {
                                        let retain = buffer_id != id;
                                        removed |= !retain;
                                        retain
                                    });
                                }
                                if removed {
                                    picker.refresh(window, cx);
                                }
                            }
                        }
                        _ => {}
                    },
                ),
                editor_buffers: editor
                    .read(cx)
                    .buffer()
                    .read(cx)
                    .all_buffers()
                    .into_iter()
                    .filter_map(|buffer| {
                        let buffer = buffer.read(cx);
                        Some((buffer.file()?.worktree_id(cx), buffer.remote_id()))
                    })
                    .collect(),
            });

            picker.refresh(window, cx);
        });
    }
}

impl Render for LspTool {
    // TODO kb add a setting to remove this button out of the status bar
    // TODO kb add scrollbar + max width and height
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl ui::IntoElement {
        let delegate = &self.lsp_picker.read(cx).delegate;
        if delegate.active_editor.is_none() || delegate.items.is_empty() {
            return div();
        }

        let mut has_errors = false;
        let mut has_warnings = false;
        for item in &self.lsp_picker.read(cx).delegate.items {
            match item {
                LspItem::Header(_, Some((_, Severity::Error))) => has_errors = true,
                LspItem::Header(_, Some((_, Severity::Warning))) => has_warnings = true,
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
                self.lsp_picker.clone(),
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
