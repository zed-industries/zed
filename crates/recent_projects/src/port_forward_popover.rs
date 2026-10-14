use editor::Editor;
use gpui::{
    App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, PromptLevel,
    WeakEntity, Window,
};
use remote::{SshConnectionOptions, SshPortForwardOption, format_host_port, parse_host_port};
use settings::{Settings as _, SshConnection, update_settings_file};
use ui::{IconButtonShape, List, ListItem, ListSeparator, Tooltip, prelude::*};
use workspace::{Toast, Workspace, notifications::NotificationId};

use crate::remote_connections::RemoteSettings;

pub struct PortForwardPopover {
    ssh: SshConnectionOptions,
    workspace: WeakEntity<Workspace>,
    port_forwards: Vec<SshPortForwardOption>,
    editing: Option<PortForwardEdit>,
    focus_handle: FocusHandle,
}

struct PortForwardEdit {
    target: PortForwardEditTarget,
    local: Entity<Editor>,
    remote: Entity<Editor>,
}

#[derive(Clone, Copy)]
enum PortForwardEditTarget {
    Existing(usize),
    Draft,
}

impl PortForwardPopover {
    pub fn new(
        ssh: SshConnectionOptions,
        workspace: WeakEntity<Workspace>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let port_forwards = load_port_forwards(&ssh, cx);
        Self {
            ssh,
            workspace,
            port_forwards,
            editing: None,
            focus_handle: cx.focus_handle(),
        }
    }

    fn host_label(&self) -> String {
        self.ssh.connection_string()
    }

    fn confirm(&mut self, _: &menu::Confirm, window: &mut Window, cx: &mut Context<Self>) {
        self.commit_edit(window, cx);
    }

    fn cancel(&mut self, _: &menu::Cancel, window: &mut Window, cx: &mut Context<Self>) {
        if self.editing.is_some() {
            self.editing = None;
            self.focus_handle.focus(window, cx);
            cx.notify();
        } else {
            cx.emit(DismissEvent);
        }
    }

    fn commit_edit(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(editing) = &self.editing else {
            return;
        };
        let local = editing.local.clone();
        let remote = editing.remote.clone();
        let target = editing.target;

        match parse_from_editors(&local, &remote, cx) {
            Ok(port_forward) => {
                match target {
                    PortForwardEditTarget::Existing(i) => {
                        if let Some(slot) = self.port_forwards.get_mut(i) {
                            *slot = port_forward;
                        }
                    }
                    PortForwardEditTarget::Draft => self.port_forwards.push(port_forward),
                }
                self.editing = None;
                self.persist(cx);
                self.focus_handle.focus(window, cx);
                cx.notify();
            }
            Err(error) => {
                log::warn!("Invalid port forward: {error:#}");
                self.toast(
                    format!("Invalid port forward. Use port or host:port. {error}"),
                    cx,
                );
            }
        }
    }

    fn start_edit(
        &mut self,
        target: PortForwardEditTarget,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.editing.is_some() {
            self.commit_edit(window, cx);
            if self.editing.is_some() {
                return;
            }
        }
        let existing = match target {
            PortForwardEditTarget::Existing(i) => self.port_forwards.get(i).cloned(),
            PortForwardEditTarget::Draft => None,
        };
        let (local, remote) = port_forward_editors(existing.as_ref(), window, cx);
        self.editing = Some(PortForwardEdit {
            target,
            local,
            remote,
        });
        cx.notify();
    }

    fn delete_at(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        if index >= self.port_forwards.len() {
            return;
        }
        self.port_forwards.remove(index);
        match self.editing.as_ref().map(|editing| editing.target) {
            Some(PortForwardEditTarget::Existing(i)) if i == index => self.editing = None,
            Some(PortForwardEditTarget::Existing(i)) if i > index => {
                if let Some(editing) = self.editing.as_mut() {
                    editing.target = PortForwardEditTarget::Existing(i - 1);
                }
            }
            _ => {}
        }
        self.persist(cx);
        self.focus_handle.focus(window, cx);
        cx.notify();
    }

    fn persist(&self, cx: &mut Context<Self>) {
        let Some(fs) = self
            .workspace
            .read_with(cx, |workspace, _| workspace.app_state().fs.clone())
            .ok()
        else {
            return;
        };
        let host = self.ssh.host.to_string();
        let username = self.ssh.username.clone();
        let port = self.ssh.port;
        let forwards = self.port_forwards.clone();
        update_settings_file(fs, cx, move |setting, _| {
            let connections = setting
                .remote
                .ssh_connections
                .get_or_insert_with(Default::default);
            if let Some(connection) = connections.iter_mut().find(|connection| {
                connection.host == host
                    && connection.username == username
                    && connection.port == port
            }) {
                connection.port_forwards = if forwards.is_empty() {
                    None
                } else {
                    Some(forwards)
                };
            } else {
                connections.push(SshConnection {
                    host,
                    username,
                    port,
                    port_forwards: if forwards.is_empty() {
                        None
                    } else {
                        Some(forwards)
                    },
                    ..Default::default()
                });
            }
        });
    }

    fn toast(&self, message: String, cx: &mut Context<Self>) {
        if let Some(workspace) = self.workspace.upgrade() {
            workspace.update(cx, |workspace, cx| {
                workspace.show_toast(
                    Toast::new(
                        NotificationId::composite::<Self>("invalid_port_forward_format"),
                        message,
                    ),
                    cx,
                );
            });
        }
    }
}

impl EventEmitter<DismissEvent> for PortForwardPopover {}

impl Focusable for PortForwardPopover {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for PortForwardPopover {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let editing_target = self.editing.as_ref().map(|editing| editing.target);
        let host = self.host_label();

        let editor_row = |local: Entity<Editor>, remote: Entity<Editor>| {
            h_flex()
                .px_2()
                .py_1()
                .gap_1()
                .w_full()
                .items_center()
                .child(div().flex_1().child(local))
                .child(
                    Icon::new(IconName::ArrowRight)
                        .color(Color::Muted)
                        .size(IconSize::Small),
                )
                .child(div().flex_1().child(remote))
        };

        let rows = self
            .port_forwards
            .iter()
            .enumerate()
            .map(|(ix, port_forward)| {
                if matches!(editing_target, Some(PortForwardEditTarget::Existing(i)) if i == ix)
                    && let Some(editing) = &self.editing
                {
                    return editor_row(editing.local.clone(), editing.remote.clone())
                        .into_any_element();
                }

                let label = port_forward_label(port_forward);
                let label_for_prompt = label.clone();

                ListItem::new(("port-forward-item", ix))
                    .inset(true)
                    .spacing(ui::ListItemSpacing::Sparse)
                    .child(Label::new(label).color(Color::Muted))
                    .end_slot(
                        h_flex()
                            .gap_1()
                            .child(
                                IconButton::new(("edit-port-forward", ix), IconName::Pencil)
                                    .icon_size(IconSize::Small)
                                    .shape(IconButtonShape::Square)
                                    .tooltip(Tooltip::text("Edit Port Forward"))
                                    .on_click(cx.listener(move |this, _, window, cx| {
                                        cx.stop_propagation();
                                        this.start_edit(
                                            PortForwardEditTarget::Existing(ix),
                                            window,
                                            cx,
                                        );
                                    })),
                            )
                            .child(
                                IconButton::new(("remove-port-forward", ix), IconName::Trash)
                                    .icon_size(IconSize::Small)
                                    .shape(IconButtonShape::Square)
                                    .tooltip(Tooltip::text("Remove Port Forward"))
                                    .on_click(cx.listener({
                                        let label_for_prompt = label_for_prompt.clone();
                                        move |_this, _, window, cx| {
                                            cx.stop_propagation();
                                            let prompt_message = format!(
                                                "Remove port forward `{label_for_prompt}`?"
                                            );
                                            let confirmation = window.prompt(
                                                PromptLevel::Warning,
                                                &prompt_message,
                                                None,
                                                &["Yes, remove it", "No, keep it"],
                                                cx,
                                            );
                                            cx.spawn_in(window, async move |this, cx| {
                                                if confirmation.await.ok() == Some(0) {
                                                    this.update_in(cx, |this, window, cx| {
                                                        this.delete_at(ix, window, cx);
                                                    })
                                                    .ok();
                                                }
                                            })
                                            .detach();
                                        }
                                    })),
                            ),
                    )
                    .show_end_slot_on_hover()
                    .on_click(cx.listener(move |this, _, window, cx| {
                        this.start_edit(PortForwardEditTarget::Existing(ix), window, cx);
                    }))
                    .into_any_element()
            })
            .collect::<Vec<_>>();

        let draft_row = matches!(editing_target, Some(PortForwardEditTarget::Draft))
            .then(|| {
                self.editing.as_ref().map(|editing| {
                    editor_row(editing.local.clone(), editing.remote.clone()).into_any_element()
                })
            })
            .flatten();

        v_flex()
            .id("port-forward-popover")
            .key_context("PortForwardPopover")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::confirm))
            .on_action(cx.listener(Self::cancel))
            .elevation_2(cx)
            .w(rems(24.))
            .child(
                h_flex()
                    .px_2()
                    .py_1()
                    .justify_between()
                    .child(Label::new("Port forwarding").size(LabelSize::Small))
                    .child(
                        Label::new(host)
                            .size(LabelSize::Small)
                            .color(Color::Muted)
                            .truncate(),
                    ),
            )
            .child(ListSeparator)
            .child(
                List::new()
                    .empty_message(
                        div()
                            .px_3()
                            .py_1()
                            .child(
                                Label::new("No port forwards configured.")
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            )
                            .into_any_element(),
                    )
                    .children(rows)
                    .children(draft_row),
            )
            .child(ListSeparator)
            .child(
                ListItem::new("add-port-forward")
                    .inset(true)
                    .spacing(ui::ListItemSpacing::Sparse)
                    .start_slot(Icon::new(IconName::Plus).color(Color::Muted))
                    .child(Label::new("Add port forward"))
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.start_edit(PortForwardEditTarget::Draft, window, cx);
                    })),
            )
            .when(cfg!(windows), |this| {
                this.child(
                    div().px_3().py_1().child(
                        Label::new("Live updates apply on reconnect on Windows.")
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    ),
                )
            })
    }
}

fn load_port_forwards(ssh: &SshConnectionOptions, cx: &App) -> Vec<SshPortForwardOption> {
    let host = ssh.host.to_string();
    RemoteSettings::get_global(cx)
        .ssh_connections()
        .find(|connection| {
            connection.host == host
                && connection.username == ssh.username
                && connection.port == ssh.port
        })
        .and_then(|connection| connection.port_forwards.clone())
        .or_else(|| ssh.port_forwards.clone())
        .unwrap_or_default()
}

fn port_forward_editors(
    existing: Option<&SshPortForwardOption>,
    window: &mut Window,
    cx: &mut App,
) -> (Entity<Editor>, Entity<Editor>) {
    let local = cx.new(|cx| Editor::single_line(window, cx));
    let remote = cx.new(|cx| Editor::single_line(window, cx));
    local.update(cx, |editor, cx| {
        editor.set_placeholder_text("localhost:port", window, cx);
        if let Some(existing) = existing {
            editor.set_text(
                format_host_port(existing.local_host.as_deref(), existing.local_port),
                window,
                cx,
            );
        }
    });
    remote.update(cx, |editor, cx| {
        editor.set_placeholder_text("localhost:port", window, cx);
        if let Some(existing) = existing {
            editor.set_text(
                format_host_port(existing.remote_host.as_deref(), existing.remote_port),
                window,
                cx,
            );
        }
    });
    local.focus_handle(cx).focus(window, cx);
    (local, remote)
}

fn parse_from_editors(
    local: &Entity<Editor>,
    remote: &Entity<Editor>,
    cx: &App,
) -> anyhow::Result<SshPortForwardOption> {
    let (local_host, local_port) = parse_host_port(&local.read(cx).text(cx))?;
    let (remote_host, remote_port) = parse_host_port(&remote.read(cx).text(cx))?;
    Ok(SshPortForwardOption {
        local_host,
        local_port,
        remote_host,
        remote_port,
    })
}

fn port_forward_label(pf: &SshPortForwardOption) -> String {
    format!(
        "{} → {}",
        format_host_port(pf.local_host.as_deref(), pf.local_port),
        format_host_port(pf.remote_host.as_deref(), pf.remote_port),
    )
}

pub fn port_forward_count_for_ssh(ssh: &SshConnectionOptions, cx: &App) -> usize {
    load_port_forwards(ssh, cx).len()
}
