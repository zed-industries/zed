use crate::RemoteSettings;
use editor::Editor;
use futures::channel::oneshot;
use gpui::{
    App, Context, DismissEvent, Div, Entity, EventEmitter, FocusHandle, Focusable, IntoElement,
    ParentElement, PromptLevel, Render, Role, SharedString, Styled, Subscription, WeakEntity,
    Window, div, px,
};
use project::Fs;
use remote::{SshConnectionOptions, SshPortForwardOption};
use settings::{Settings as _, SettingsStore, SshConnection, update_settings_file_with_completion};
use std::sync::Arc;
use ui::{
    Button, Color, Icon, IconButton, IconButtonShape, IconName, IconSize, Label, LabelSize,
    Tooltip, prelude::*,
};
use util::ResultExt;
use workspace::{Toast, Workspace, notifications::NotificationId};

const INVALID_FORWARD_MESSAGE: &str =
    "Invalid port forward. Use port or host:port. Expected port or host:port";

#[derive(Clone)]
struct SshConnectionIdentity {
    host: String,
    username: Option<String>,
    port: Option<u16>,
}

impl From<&SshConnectionOptions> for SshConnectionIdentity {
    fn from(options: &SshConnectionOptions) -> Self {
        Self {
            host: options.host.to_string(),
            username: options.username.clone(),
            port: options.port,
        }
    }
}

impl SshConnectionIdentity {
    fn matches(&self, connection: &SshConnection) -> bool {
        self.host == connection.host
            && self.username == connection.username
            && self.port == connection.port
    }
}

struct ForwardEditor {
    original: Option<SshPortForwardOption>,
    local: Entity<Editor>,
    remote: Entity<Editor>,
}

fn port_forward_endpoint_input(
    editor: &Entity<Editor>,
    element_id: &'static str,
    accessible_name: &'static str,
    is_narrow: bool,
    cx: &App,
) -> impl IntoElement {
    let unfocused_border_color = cx.theme().colors().panel_background;
    let focused_border_color = cx.theme().colors().border_focused;
    let value = editor.read(cx).text(cx);

    div()
        .when(is_narrow, |this| this.w_full())
        .when(!is_narrow, |this| this.flex_1())
        .min_w_0()
        .id(element_id)
        .role(Role::TextInput)
        .track_focus(&editor.focus_handle(cx).tab_stop(true))
        .aria_label(accessible_name)
        .aria_value(value)
        .aria_placeholder("localhost:port")
        .rounded_sm()
        .border_1()
        .border_color(unfocused_border_color)
        .focus_visible(|this| this.border_color(focused_border_color))
        .child(editor.clone())
}

#[derive(Clone)]
enum ForwardChange {
    Upsert {
        original: Option<SshPortForwardOption>,
        replacement: SshPortForwardOption,
    },
    Delete {
        original: SshPortForwardOption,
    },
}

#[derive(Debug, PartialEq, Eq)]
enum ForwardInputError {
    Empty,
    InvalidHost,
    InvalidPort,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum ForwardUpdateError {
    AmbiguousConnection,
    AmbiguousForward,
    ConnectionChanged,
    ForwardChanged,
    LocalPortInUse,
}

impl ForwardUpdateError {
    fn message(self) -> &'static str {
        match self {
            Self::AmbiguousConnection => {
                "More than one SSH settings entry matches this connection."
            }
            Self::AmbiguousForward => {
                "This port forward appears more than once in settings and cannot be edited unambiguously."
            }
            Self::ConnectionChanged => {
                "The SSH settings entry changed while the popover was open. Reopen it and try again."
            }
            Self::ForwardChanged => {
                "This port forward changed in settings. Reopen the popover and try again."
            }
            Self::LocalPortInUse => "A port forward already uses this local host and port.",
        }
    }
}

pub struct PortForwardingPopover {
    connection: SshConnectionIdentity,
    connection_name: SharedString,
    fs: Arc<dyn Fs>,
    workspace: WeakEntity<Workspace>,
    preview_only: bool,
    focus_handle: FocusHandle,
    forwards: Vec<SshPortForwardOption>,
    editor: Option<ForwardEditor>,
    is_saving: bool,
    message: Option<SharedString>,
    _subscriptions: Vec<Subscription>,
}

impl PortForwardingPopover {
    pub fn popover(
        options: SshConnectionOptions,
        workspace: WeakEntity<Workspace>,
        fs: Arc<dyn Fs>,
        preview_only: bool,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<Self> {
        cx.new(|cx| Self::new(options, workspace, fs, preview_only, window, cx))
    }

    fn new(
        options: SshConnectionOptions,
        workspace: WeakEntity<Workspace>,
        fs: Arc<dyn Fs>,
        preview_only: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let connection = SshConnectionIdentity::from(&options);
        let connection_name: SharedString = options
            .nickname
            .clone()
            .unwrap_or_else(|| options.host.to_string())
            .into();
        let (forwards, message) = if preview_only {
            (
                vec![SshPortForwardOption {
                    local_host: None,
                    local_port: 80,
                    remote_host: None,
                    remote_port: 80,
                }],
                None,
            )
        } else {
            match read_forwards(&connection, cx) {
                Ok(forwards) => (forwards, None),
                Err(error) => (Vec::new(), Some(error.message().into())),
            }
        };

        let mut subscriptions = Vec::new();
        if !preview_only {
            subscriptions.push(cx.observe_global_in::<SettingsStore>(
                window,
                move |this, _window, cx| {
                    this.refresh_forwards(cx);
                    cx.notify();
                },
            ));
        }

        let focus_handle = cx.focus_handle();

        Self {
            connection,
            connection_name,
            fs,
            workspace,
            preview_only,
            focus_handle,
            forwards,
            editor: None,
            is_saving: false,
            message,
            _subscriptions: subscriptions,
        }
    }

    fn refresh_forwards(&mut self, cx: &App) {
        match read_forwards(&self.connection, cx) {
            Ok(forwards) => {
                self.forwards = forwards;
                self.message = None;
            }
            Err(error) => self.message = Some(error.message().into()),
        }
    }

    fn begin_add(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.begin_edit(None, window, cx);
    }

    fn begin_edit(
        &mut self,
        original: Option<SshPortForwardOption>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.is_saving {
            return;
        }

        let local_text = original
            .as_ref()
            .map(|forward| format_endpoint(&forward.local_host, forward.local_port))
            .unwrap_or_default();
        let remote_text = original
            .as_ref()
            .map(|forward| format_endpoint(&forward.remote_host, forward.remote_port))
            .unwrap_or_default();
        let local = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("localhost:port", window, cx);
            if !local_text.is_empty() {
                editor.set_text(local_text, window, cx);
            }
            editor
        });
        let remote = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("localhost:port", window, cx);
            if !remote_text.is_empty() {
                editor.set_text(remote_text, window, cx);
            }
            editor
        });

        local.focus_handle(cx).focus(window, cx);
        self.editor = Some(ForwardEditor {
            original,
            local,
            remote,
        });
        self.message = None;
        cx.notify();
    }

    fn cancel_edit(&mut self, cx: &mut Context<Self>) {
        self.editor = None;
        self.message = None;
        cx.notify();
    }

    fn save_edit(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.is_saving {
            return;
        }

        let Some(editor) = &self.editor else {
            return;
        };
        let original = editor.original.clone();
        let local_text = editor.local.read(cx).text(cx);
        let remote_text = editor.remote.read(cx).text(cx);

        let replacement = match parse_forward(&local_text, &remote_text) {
            Ok(forward) => forward,
            Err(_) => {
                self.show_error(INVALID_FORWARD_MESSAGE, cx);
                return;
            }
        };
        let change = ForwardChange::Upsert {
            original,
            replacement,
        };

        if self.preview_only {
            match apply_forward_change(self.forwards.clone(), change) {
                Ok(forwards) => {
                    self.forwards = forwards;
                    self.editor = None;
                    self.message = None;
                    cx.notify();
                }
                Err(error) => self.show_error(error.message(), cx),
            }
            return;
        }

        self.persist_change(change, window, cx);
    }

    fn confirm_delete(
        &mut self,
        forward: SshPortForwardOption,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.is_saving {
            return;
        }

        let prompt_message = format!("Remove port forward ‘{}’?", format_forward(&forward));
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
                    this.delete_forward(forward, window, cx);
                })
                .log_err();
            }
        })
        .detach();
    }

    fn delete_forward(
        &mut self,
        forward: SshPortForwardOption,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let change = ForwardChange::Delete { original: forward };
        if self.preview_only {
            match apply_forward_change(self.forwards.clone(), change) {
                Ok(forwards) => {
                    self.forwards = forwards;
                    self.message = None;
                    cx.notify();
                }
                Err(error) => self.show_error(error.message(), cx),
            }
        } else {
            self.persist_change(change, window, cx);
        }
    }

    fn persist_change(
        &mut self,
        change: ForwardChange,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.is_saving {
            return;
        }

        self.is_saving = true;
        self.message = None;
        cx.notify();

        let identity = self.connection.clone();
        let (update_sender, update_receiver) =
            oneshot::channel::<Result<Vec<SshPortForwardOption>, String>>();
        let completion =
            update_settings_file_with_completion(self.fs.clone(), cx, move |settings, _| {
                let result = apply_settings_change(&mut settings.remote, &identity, change);
                if let Err(result) = update_sender.send(result) {
                    log::debug!("port forward settings update was no longer observed: {result:?}");
                }
            });

        cx.spawn_in(window, async move |this, cx| {
            let update_result = update_receiver.await;
            let write_result = completion.await;
            let result = match update_result {
                Ok(Ok(forwards)) => match write_result {
                    Ok(Ok(())) => Ok(forwards),
                    Ok(Err(error)) => Err(error.to_string()),
                    Err(_) => Err("Failed to save SSH settings.".to_owned()),
                },
                Ok(Err(error)) => Err(error),
                Err(_) => Err("Failed to update SSH settings.".to_owned()),
            };

            this.update_in(cx, |this, _window, cx| {
                this.is_saving = false;
                match result {
                    Ok(forwards) => {
                        this.forwards = forwards;
                        this.editor = None;
                        this.message = None;
                    }
                    Err(error) => this.show_error(&error, cx),
                }
                cx.notify();
            })
            .log_err();
        })
        .detach();
    }

    fn show_error(&mut self, message: &str, cx: &mut Context<Self>) {
        self.message = Some(message.into());
        let workspace = self.workspace.clone();
        workspace
            .update(cx, |workspace, cx| {
                workspace.show_toast(
                    Toast::new(
                        NotificationId::unique::<PortForwardingPopover>(),
                        message.to_owned(),
                    ),
                    cx,
                );
            })
            .log_err();
        cx.notify();
    }
}

impl Focusable for PortForwardingPopover {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<DismissEvent> for PortForwardingPopover {}

impl Render for PortForwardingPopover {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let max_width = (window.viewport_size().width - px(16.)).max(px(0.));
        let is_narrow = max_width < px(360.);
        let mut list = v_flex().gap_1();
        if self.forwards.is_empty() && self.editor.is_none() {
            list = list.child(
                Label::new("No port forwards configured.")
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            );
        }

        let mut editor_rendered = false;
        for (index, forward) in self.forwards.iter().cloned().enumerate() {
            if self
                .editor
                .as_ref()
                .is_some_and(|editor| editor.original.as_ref() == Some(&forward))
                && !editor_rendered
            {
                if let Some(editor) = &self.editor {
                    list = list.child(self.render_editor(editor, is_narrow, cx));
                    editor_rendered = true;
                }
            } else {
                list = list.child(self.render_forward_row(index, forward, window, cx));
            }
        }

        if !editor_rendered {
            if let Some(editor) = &self.editor {
                list = list.child(self.render_editor(editor, is_narrow, cx));
            }
        }

        v_flex()
            .id("port-forwarding-popover")
            .track_focus(&self.focus_handle)
            .tab_group()
            .w(px(384.))
            .max_w(max_width)
            .max_h(px(480.))
            .gap_2()
            .p_2()
            .bg(cx.theme().colors().panel_background)
            .border_1()
            .border_color(cx.theme().colors().border_variant)
            .rounded_lg()
            .child(
                h_flex()
                    .w_full()
                    .justify_between()
                    .gap_2()
                    .child(Label::new("Port forwarding").size(LabelSize::Small))
                    .child(
                        Label::new(self.connection_name.clone())
                            .size(LabelSize::Small)
                            .color(Color::Muted)
                            .truncate(),
                    ),
            )
            .child(
                div()
                    .id("port-forward-list")
                    .max_h(px(280.))
                    .overflow_y_scroll()
                    .child(list),
            )
            .when(self.editor.is_none() && !self.is_saving, |this| {
                this.child(
                    Button::new("add-port-forward", "Add port forward")
                        .start_icon(Icon::new(IconName::Plus))
                        .style(ButtonStyle::Subtle)
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.begin_add(window, cx);
                        })),
                )
            })
            .when_some(self.message.clone(), |this, message| {
                this.child(
                    div().id("port-forward-error").child(
                        Label::new(message)
                            .size(LabelSize::Small)
                            .color(Color::Error),
                    ),
                )
            })
            .child(
                Label::new("Changes apply after restarting Zed.")
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            )
            .on_action(cx.listener(|this, _: &menu::Cancel, _, cx| {
                if this.editor.is_some() {
                    this.cancel_edit(cx);
                } else {
                    cx.emit(DismissEvent);
                }
            }))
            .on_action(|_: &menu::SelectNext, window, cx| window.focus_next(cx))
            .on_action(|_: &menu::SelectPrevious, window, cx| window.focus_prev(cx))
            .on_action(|_: &editor::actions::Tab, window, cx| window.focus_next(cx))
            .on_action(|_: &editor::actions::Backtab, window, cx| window.focus_prev(cx))
            .on_mouse_down_out(cx.listener(|_, _, _, cx| cx.emit(DismissEvent)))
            .into_any_element()
    }
}

impl PortForwardingPopover {
    fn render_forward_row(
        &self,
        index: usize,
        forward: SshPortForwardOption,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let delete_forward = forward.clone();
        h_flex()
            .id(format!("port-forward-row-{index}"))
            .w_full()
            .gap_1()
            .child(
                div().flex_1().min_w_0().child(
                    Button::new(
                        format!("edit-port-forward-{index}"),
                        format_forward(&forward),
                    )
                    .full_width()
                    .label_size(LabelSize::Small)
                    .truncate(true)
                    .end_icon(Icon::new(IconName::Pencil).size(IconSize::Small))
                    .style(ButtonStyle::Subtle)
                    .disabled(self.editor.is_some() || self.is_saving)
                    .on_click(cx.listener(move |this, _, window, cx| {
                        this.begin_edit(Some(forward.clone()), window, cx);
                    })),
                ),
            )
            .child(
                IconButton::new(format!("delete-port-forward-{index}"), IconName::Trash)
                    .shape(IconButtonShape::Square)
                    .icon_size(IconSize::Small)
                    .icon_color(Color::Error)
                    .style(ButtonStyle::Subtle)
                    .aria_label("Delete port forward")
                    .tooltip(Tooltip::text("Delete Port Forward"))
                    .disabled(self.editor.is_some() || self.is_saving)
                    .on_click(cx.listener(move |this, _, window, cx| {
                        this.confirm_delete(delete_forward.clone(), window, cx);
                    })),
            )
    }

    fn render_editor(
        &self,
        editor: &ForwardEditor,
        is_narrow: bool,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let local_endpoint = port_forward_endpoint_input(
            &editor.local,
            "port-forward-local-endpoint",
            "Local endpoint",
            is_narrow,
            cx,
        );
        let remote_endpoint = port_forward_endpoint_input(
            &editor.remote,
            "port-forward-remote-endpoint",
            "Remote endpoint",
            is_narrow,
            cx,
        );
        let direction = if is_narrow {
            IconName::ArrowDown
        } else {
            IconName::ArrowRight
        };
        let save_button = IconButton::new("save-port-forward", IconName::Check)
            .shape(IconButtonShape::Square)
            .icon_size(IconSize::Small)
            .aria_label("Save port forward")
            .tooltip(Tooltip::text("Save Port Forward"))
            .disabled(self.is_saving)
            .on_click(cx.listener(|this, _, window, cx| {
                this.save_edit(window, cx);
            }));
        let cancel_button = IconButton::new("cancel-port-forward", IconName::Close)
            .shape(IconButtonShape::Square)
            .icon_size(IconSize::Small)
            .aria_label("Cancel port forward edit")
            .tooltip(Tooltip::text("Cancel"))
            .disabled(self.is_saving)
            .on_click(cx.listener(|this, _, _, cx| {
                this.cancel_edit(cx);
            }));

        let editor = if is_narrow {
            v_flex()
                .id("port-forward-editor")
                .w_full()
                .gap_1()
                .child(local_endpoint)
                .child(
                    h_flex()
                        .w_full()
                        .justify_center()
                        .child(Icon::new(direction).color(Color::Muted)),
                )
                .child(remote_endpoint)
                .child(
                    h_flex()
                        .w_full()
                        .justify_end()
                        .gap_1()
                        .child(save_button)
                        .child(cancel_button),
                )
        } else {
            h_flex()
                .id("port-forward-editor")
                .w_full()
                .gap_1()
                .child(local_endpoint)
                .child(Icon::new(direction).color(Color::Muted))
                .child(remote_endpoint)
                .child(save_button)
                .child(cancel_button)
        };

        editor.on_action(cx.listener(|this, _: &menu::Confirm, window, cx| {
            this.save_edit(window, cx);
        }))
    }
}

fn read_forwards(
    identity: &SshConnectionIdentity,
    cx: &App,
) -> Result<Vec<SshPortForwardOption>, ForwardUpdateError> {
    let connections = RemoteSettings::get_global(cx)
        .ssh_connections()
        .filter(|connection| identity.matches(connection))
        .collect::<Vec<_>>();

    match connections.as_slice() {
        [] => Ok(Vec::new()),
        [connection] => Ok(connection.port_forwards.clone().unwrap_or_default()),
        _ => Err(ForwardUpdateError::AmbiguousConnection),
    }
}

fn apply_settings_change(
    settings: &mut settings::RemoteSettingsContent,
    identity: &SshConnectionIdentity,
    change: ForwardChange,
) -> Result<Vec<SshPortForwardOption>, String> {
    let mut connections = settings.ssh_connections.clone().unwrap_or_default();
    let matching_indices = connections
        .iter()
        .enumerate()
        .filter_map(|(index, connection)| identity.matches(connection).then_some(index))
        .collect::<Vec<_>>();

    if matching_indices.len() > 1 {
        return Err(ForwardUpdateError::AmbiguousConnection.message().to_owned());
    }

    let result = if let Some(index) = matching_indices.first().copied() {
        let Some(connection) = connections.get_mut(index) else {
            return Err(ForwardUpdateError::ConnectionChanged.message().to_owned());
        };
        let forwards = connection.port_forwards.clone().unwrap_or_default();
        let updated_forwards =
            apply_forward_change(forwards, change).map_err(|error| error.message().to_owned())?;
        connection.port_forwards = (!updated_forwards.is_empty()).then(|| updated_forwards.clone());
        Ok(updated_forwards)
    } else {
        match change {
            ForwardChange::Upsert {
                original: None,
                replacement,
            } => {
                let forwards = apply_forward_change(
                    Vec::new(),
                    ForwardChange::Upsert {
                        original: None,
                        replacement: replacement.clone(),
                    },
                )
                .map_err(|error| error.message().to_owned())?;
                connections.push(SshConnection {
                    host: identity.host.clone(),
                    username: identity.username.clone(),
                    port: identity.port,
                    port_forwards: Some(forwards.clone()),
                    ..Default::default()
                });
                Ok(forwards)
            }
            _ => Err(ForwardUpdateError::ConnectionChanged.message().to_owned()),
        }
    };

    if result.is_ok() {
        settings.ssh_connections = (!connections.is_empty()).then_some(connections);
    }
    result
}

fn apply_forward_change(
    mut forwards: Vec<SshPortForwardOption>,
    change: ForwardChange,
) -> Result<Vec<SshPortForwardOption>, ForwardUpdateError> {
    match change {
        ForwardChange::Upsert {
            original,
            replacement,
        } => {
            let replacing_index = if let Some(original) = original {
                Some(find_unique_forward(&forwards, &original)?)
            } else {
                None
            };

            if forwards.iter().enumerate().any(|(index, forward)| {
                Some(index) != replacing_index && same_local_endpoint(forward, &replacement)
            }) {
                return Err(ForwardUpdateError::LocalPortInUse);
            }

            if let Some(index) = replacing_index {
                let Some(forward) = forwards.get_mut(index) else {
                    return Err(ForwardUpdateError::ForwardChanged);
                };
                *forward = replacement;
            } else {
                forwards.push(replacement);
            }
            Ok(forwards)
        }
        ForwardChange::Delete { original } => {
            let index = find_unique_forward(&forwards, &original)?;
            forwards.remove(index);
            Ok(forwards)
        }
    }
}

fn find_unique_forward(
    forwards: &[SshPortForwardOption],
    original: &SshPortForwardOption,
) -> Result<usize, ForwardUpdateError> {
    let matching_indices = forwards
        .iter()
        .enumerate()
        .filter_map(|(index, forward)| (forward == original).then_some(index))
        .collect::<Vec<_>>();

    match matching_indices.as_slice() {
        [index] => Ok(*index),
        [] => Err(ForwardUpdateError::ForwardChanged),
        _ => Err(ForwardUpdateError::AmbiguousForward),
    }
}

fn same_local_endpoint(left: &SshPortForwardOption, right: &SshPortForwardOption) -> bool {
    let local_host =
        |host: &Option<String>| host.as_deref().unwrap_or("localhost").to_ascii_lowercase();

    left.local_port == right.local_port
        && local_host(&left.local_host) == local_host(&right.local_host)
}

fn parse_forward(
    local_text: &str,
    remote_text: &str,
) -> Result<SshPortForwardOption, ForwardInputError> {
    let (local_host, local_port) = parse_endpoint(local_text)?;
    let (remote_host, remote_port) = parse_endpoint(remote_text)?;
    Ok(SshPortForwardOption {
        local_host,
        local_port,
        remote_host,
        remote_port,
    })
}

fn parse_endpoint(value: &str) -> Result<(Option<String>, u16), ForwardInputError> {
    let value = value.trim();
    if value.is_empty() {
        return Err(ForwardInputError::Empty);
    }

    let (host, port) = if let Some(rest) = value.strip_prefix('[') {
        let (host, port) = rest
            .split_once("]:")
            .ok_or(ForwardInputError::InvalidHost)?;
        if host.parse::<std::net::Ipv6Addr>().is_err() {
            return Err(ForwardInputError::InvalidHost);
        }
        (Some(host.to_owned()), port)
    } else if let Some((host, port)) = value.split_once(':') {
        if host.is_empty()
            || port.contains(':')
            || host
                .chars()
                .any(|character| character.is_whitespace() || character == '[' || character == ']')
        {
            return Err(ForwardInputError::InvalidHost);
        }
        (Some(host.to_owned()), port)
    } else {
        (None, value)
    };

    if port.is_empty() || port.chars().any(char::is_whitespace) {
        return Err(ForwardInputError::InvalidPort);
    }
    let port = port
        .parse::<u16>()
        .map_err(|_| ForwardInputError::InvalidPort)?;
    if port == 0 {
        return Err(ForwardInputError::InvalidPort);
    }
    Ok((host, port))
}

fn format_endpoint(host: &Option<String>, port: u16) -> String {
    match host {
        Some(host) if host.contains(':') => format!("[{host}]:{port}"),
        Some(host) => format!("{host}:{port}"),
        None => port.to_string(),
    }
}

fn format_forward(forward: &SshPortForwardOption) -> String {
    format!(
        "{} → {}",
        format_endpoint(&forward.local_host, forward.local_port),
        format_endpoint(&forward.remote_host, forward.remote_port),
    )
}

// TestAppContext cannot expose the accessibility tree; manually verify input roles, names, values, and placeholders with a screen reader, and verify the keyboard focus outline visually.
#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{KeyBinding, TestAppContext};
    use project::Project;
    use std::{cell::Cell, ops::Deref, rc::Rc};
    use workspace::AppState;

    struct PortForwardingTestRoot {
        _workspace: Entity<Workspace>,
        popover: Entity<PortForwardingPopover>,
        _dismiss_subscription: Option<Subscription>,
    }

    struct PortForwardingMenuTestRoot {
        _workspace: Entity<Workspace>,
        workspace: WeakEntity<Workspace>,
        fs: Arc<dyn Fs>,
        options: SshConnectionOptions,
    }

    impl Render for PortForwardingMenuTestRoot {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            let workspace = self.workspace.clone();
            let fs = self.fs.clone();
            let options = self.options.clone();
            ui::PopoverMenu::new("test-port-forwarding-menu")
                .menu(move |window, cx| {
                    workspace.upgrade()?;
                    Some(PortForwardingPopover::popover(
                        options.clone(),
                        workspace.clone(),
                        fs.clone(),
                        true,
                        window,
                        cx,
                    ))
                })
                .trigger(Button::new(
                    "test-port-forwarding-trigger",
                    "Port forwarding",
                ))
        }
    }

    impl Render for PortForwardingTestRoot {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div().child(self.popover.clone())
        }
    }

    fn forward(
        local_host: Option<&str>,
        local_port: u16,
        remote_host: Option<&str>,
        remote_port: u16,
    ) -> SshPortForwardOption {
        SshPortForwardOption {
            local_host: local_host.map(str::to_owned),
            local_port,
            remote_host: remote_host.map(str::to_owned),
            remote_port,
        }
    }

    fn read_ssh_connections(cx: &App) -> Vec<SshConnection> {
        RemoteSettings::get_global(cx)
            .ssh_connections()
            .cloned()
            .collect()
    }

    fn assert_persisted_connections(
        cx: &mut gpui::VisualTestContext,
        identity: &SshConnectionIdentity,
        target_forwards: &[SshPortForwardOption],
    ) {
        let connections = cx.update(|_, cx| read_ssh_connections(cx));
        assert_eq!(connections.len(), 2);
        let Some(target_connection) = connections
            .iter()
            .find(|connection| identity.matches(connection))
        else {
            panic!("target SSH settings entry was not preserved");
        };
        assert_eq!(
            target_connection.port_forwards.as_deref(),
            Some(target_forwards)
        );
        assert_eq!(
            target_connection.nickname.as_deref(),
            Some("Development server")
        );
        assert_eq!(target_connection.args, vec!["-C".to_owned()]);

        let Some(unrelated_connection) = connections
            .iter()
            .find(|connection| connection.host == "unrelated.example.com")
        else {
            panic!("unrelated SSH settings entry was not preserved");
        };
        assert_eq!(unrelated_connection.username.as_deref(), Some("other"));
        assert_eq!(unrelated_connection.port, Some(2201));
        assert_eq!(
            unrelated_connection.nickname.as_deref(),
            Some("Other server")
        );
        assert_eq!(unrelated_connection.args, vec!["-A".to_owned()]);
        assert_eq!(
            unrelated_connection.port_forwards,
            Some(vec![forward(Some("localhost"), 7070, Some("other"), 70)])
        );
    }

    #[test]
    fn parses_port_only_and_host_port_endpoints() {
        assert_eq!(
            parse_forward("80", "localhost:8080"),
            Ok(forward(None, 80, Some("localhost"), 8080))
        );
    }

    #[test]
    fn parses_bracketed_ipv6_endpoints() {
        assert_eq!(
            parse_forward("[::1]:8080", "[2001:db8::1]:80"),
            Ok(forward(Some("::1"), 8080, Some("2001:db8::1"), 80))
        );
    }

    #[test]
    fn rejects_invalid_or_ambiguous_endpoints() {
        for value in [
            "",
            "0",
            "65536",
            "localhost:",
            "::1:80",
            "[localhost]:80",
            "[::1]80",
            "host]:80",
        ] {
            assert!(parse_endpoint(value).is_err(), "accepted {value:?}");
        }
    }

    #[test]
    fn formats_omitted_and_explicit_hosts_without_losing_their_meaning() {
        assert_eq!(format_forward(&forward(None, 80, None, 80)), "80 → 80");
        assert_eq!(
            format_forward(&forward(Some("localhost"), 8080, None, 80)),
            "localhost:8080 → 80"
        );
        assert_eq!(
            format_forward(&forward(Some("::1"), 8080, Some("::1"), 80)),
            "[::1]:8080 → [::1]:80"
        );
    }

    #[test]
    fn editing_requires_the_original_entry_and_rejects_local_bind_conflicts() {
        let existing = forward(None, 80, None, 80);
        let updated = forward(Some("localhost"), 80, None, 90);
        assert_eq!(
            apply_forward_change(
                vec![existing.clone()],
                ForwardChange::Upsert {
                    original: Some(existing.clone()),
                    replacement: updated.clone(),
                },
            ),
            Ok(vec![updated.clone()])
        );
        assert!(matches!(
            apply_forward_change(
                vec![existing.clone()],
                ForwardChange::Upsert {
                    original: None,
                    replacement: updated,
                },
            ),
            Err(ForwardUpdateError::LocalPortInUse)
        ));
        assert!(matches!(
            apply_forward_change(Vec::new(), ForwardChange::Delete { original: existing },),
            Err(ForwardUpdateError::ForwardChanged)
        ));
    }

    #[test]
    fn settings_edits_preserve_other_connection_fields() {
        let identity = SshConnectionIdentity {
            host: "example.com".to_owned(),
            username: Some("zed".to_owned()),
            port: Some(2222),
        };
        let existing = forward(None, 8080, None, 80);
        let replacement = forward(Some("localhost"), 8081, Some("service"), 80);
        let mut settings_content = settings::RemoteSettingsContent {
            ssh_connections: Some(vec![SshConnection {
                host: identity.host.clone(),
                username: identity.username.clone(),
                port: identity.port,
                nickname: Some("Development server".to_owned()),
                args: vec!["-C".to_owned()],
                port_forwards: Some(vec![existing.clone()]),
                ..Default::default()
            }]),
            ..Default::default()
        };

        assert_eq!(
            apply_settings_change(
                &mut settings_content,
                &identity,
                ForwardChange::Upsert {
                    original: Some(existing),
                    replacement: replacement.clone(),
                },
            ),
            Ok(vec![replacement.clone()])
        );
        assert!(
            settings_content
                .ssh_connections
                .as_ref()
                .and_then(|connections| connections.first())
                .is_some_and(|connection| {
                    connection.nickname.as_deref() == Some("Development server")
                        && connection.args.len() == 1
                        && connection
                            .args
                            .first()
                            .is_some_and(|argument| argument.as_str() == "-C")
                        && connection.port_forwards == Some(vec![replacement])
                })
        );
    }

    #[test]
    fn adding_a_forward_creates_a_minimal_settings_entry_when_needed() {
        let identity = SshConnectionIdentity {
            host: "example.com".to_owned(),
            username: Some("zed".to_owned()),
            port: None,
        };
        let replacement = forward(None, 8080, None, 80);
        let mut settings_content = settings::RemoteSettingsContent::default();

        assert_eq!(
            apply_settings_change(
                &mut settings_content,
                &identity,
                ForwardChange::Upsert {
                    original: None,
                    replacement: replacement.clone(),
                },
            ),
            Ok(vec![replacement.clone()])
        );
        assert!(
            settings_content
                .ssh_connections
                .as_ref()
                .is_some_and(|connections| {
                    connections.len() == 1
                        && connections.first().is_some_and(|connection| {
                            identity.matches(connection)
                                && connection.port_forwards == Some(vec![replacement])
                        })
                })
        );
    }

    #[test]
    fn failed_settings_edits_leave_the_settings_list_unchanged() {
        let identity = SshConnectionIdentity {
            host: "example.com".to_owned(),
            username: None,
            port: None,
        };
        let current = forward(None, 8080, None, 80);
        let stale = forward(None, 8081, None, 80);
        let mut settings_content = settings::RemoteSettingsContent {
            ssh_connections: Some(vec![SshConnection {
                host: identity.host.clone(),
                port_forwards: Some(vec![current]),
                ..Default::default()
            }]),
            ..Default::default()
        };
        let original_settings = settings_content.ssh_connections.clone();

        assert!(
            apply_settings_change(
                &mut settings_content,
                &identity,
                ForwardChange::Upsert {
                    original: Some(stale),
                    replacement: forward(None, 8082, None, 80),
                },
            )
            .is_err()
        );
        assert_eq!(settings_content.ssh_connections, original_settings);
    }

    #[test]
    fn duplicate_connection_and_forward_entries_are_rejected_as_ambiguous() {
        let identity = SshConnectionIdentity {
            host: "example.com".to_owned(),
            username: None,
            port: None,
        };
        let mut settings_content = settings::RemoteSettingsContent {
            ssh_connections: Some(vec![
                SshConnection {
                    host: identity.host.clone(),
                    ..Default::default()
                },
                SshConnection {
                    host: identity.host.clone(),
                    ..Default::default()
                },
            ]),
            ..Default::default()
        };
        assert_eq!(
            apply_settings_change(
                &mut settings_content,
                &identity,
                ForwardChange::Upsert {
                    original: None,
                    replacement: forward(None, 8080, None, 80),
                },
            ),
            Err(ForwardUpdateError::AmbiguousConnection.message().to_owned())
        );

        let existing = forward(None, 8080, None, 80);
        assert_eq!(
            apply_forward_change(
                vec![existing.clone(), existing.clone()],
                ForwardChange::Delete { original: existing },
            ),
            Err(ForwardUpdateError::AmbiguousForward)
        );
    }

    #[gpui::test]
    async fn tab_moves_between_port_forward_endpoint_inputs(cx: &mut TestAppContext) {
        let app_state = cx.update(|cx| {
            let app_state = AppState::test(cx);
            crate::init(cx);
            editor::init(cx);
            app_state
        });
        let fs = app_state.fs.as_fake();
        let project = Project::test(fs.clone(), [], cx).await;

        cx.update(|cx| {
            cx.bind_keys([
                KeyBinding::new("tab", menu::SelectNext, None),
                KeyBinding::new("shift-tab", menu::SelectPrevious, None),
                KeyBinding::new("tab", editor::actions::Tab, Some("Editor")),
                KeyBinding::new("shift-tab", editor::actions::Backtab, Some("Editor")),
            ]);
        });

        let options = SshConnectionOptions {
            host: "for-testing-only".to_owned().into(),
            nickname: Some("for-testing-only".to_owned()),
            ..Default::default()
        };
        let (root, cx) = cx.add_window_view(move |window, cx| {
            let workspace = cx.new(|cx| Workspace::test_new(project, window, cx));
            let popover = cx.new(|cx| {
                PortForwardingPopover::new(options, workspace.downgrade(), fs, true, window, cx)
            });
            PortForwardingTestRoot {
                _workspace: workspace,
                popover,
                _dismiss_subscription: None,
            }
        });
        let popover = root.read_with(cx, |root, _| root.popover.clone());

        let Some(add_bounds) = cx.debug_bounds("add-port-forward") else {
            panic!("add port forward button was not rendered");
        };
        cx.simulate_click(add_bounds.center(), gpui::Modifiers::default());
        cx.run_until_parked();
        assert_tab_moves_between_endpoint_fields(&popover, cx);

        let Some(cancel_bounds) = cx.debug_bounds("cancel-port-forward") else {
            panic!("cancel port forward button was not rendered");
        };
        cx.simulate_click(cancel_bounds.center(), gpui::Modifiers::default());
        cx.run_until_parked();
        assert_eq!(
            popover.read_with(cx, |popover, _| popover.forwards.clone()),
            vec![forward(None, 80, None, 80)]
        );

        let Some(edit_bounds) = cx.debug_bounds("edit-port-forward-0") else {
            panic!("existing port forward edit button was not rendered");
        };
        cx.simulate_click(edit_bounds.center(), gpui::Modifiers::default());
        cx.run_until_parked();
        assert_tab_moves_between_endpoint_fields(&popover, cx);
    }

    #[gpui::test]
    async fn port_forwarding_popover_adapts_to_narrow_windows(cx: &mut TestAppContext) {
        let app_state = cx.update(|cx| {
            let app_state = AppState::test(cx);
            crate::init(cx);
            editor::init(cx);
            app_state
        });
        let fs = app_state.fs.as_fake();
        let project = Project::test(fs.clone(), [], cx).await;
        let options = SshConnectionOptions {
            host: "for-testing-only".to_owned().into(),
            nickname: Some("for-testing-only".to_owned()),
            ..Default::default()
        };

        let window_handle = cx.open_window(
            gpui::size(gpui::px(350.), gpui::px(600.)),
            move |window, cx| {
                let workspace = cx.new(|cx| Workspace::test_new(project, window, cx));
                let popover = cx.new(|cx| {
                    PortForwardingPopover::new(options, workspace.downgrade(), fs, true, window, cx)
                });
                PortForwardingTestRoot {
                    _workspace: workspace,
                    popover,
                    _dismiss_subscription: None,
                }
            },
        );
        let mut visual_cx = gpui::VisualTestContext::from_window(*window_handle.deref(), cx);
        visual_cx.run_until_parked();

        let Some(popover_bounds) = visual_cx.debug_bounds("port-forwarding-popover") else {
            panic!("port forwarding popover was not rendered");
        };
        assert!(
            popover_bounds.size.width <= gpui::px(334.),
            "popover width should respect the narrow window's side margins"
        );

        let Some(add_bounds) = visual_cx.debug_bounds("add-port-forward") else {
            panic!("add port forward button was not rendered");
        };
        visual_cx.simulate_click(add_bounds.center(), gpui::Modifiers::default());
        visual_cx.run_until_parked();

        let Some(local_bounds) = visual_cx.debug_bounds("port-forward-local-endpoint") else {
            panic!("local endpoint field was not rendered");
        };
        let Some(remote_bounds) = visual_cx.debug_bounds("port-forward-remote-endpoint") else {
            panic!("remote endpoint field was not rendered");
        };
        assert!(
            remote_bounds.origin.y >= local_bounds.bottom(),
            "narrow layout should stack the remote endpoint below the local endpoint"
        );
    }

    #[gpui::test]
    async fn port_forwarding_popover_keeps_row_layout_at_breakpoint(cx: &mut TestAppContext) {
        let app_state = cx.update(|cx| {
            let app_state = AppState::test(cx);
            crate::init(cx);
            editor::init(cx);
            app_state
        });
        let fs = app_state.fs.as_fake();
        let project = Project::test(fs.clone(), [], cx).await;
        let options = SshConnectionOptions {
            host: "for-testing-only".to_owned().into(),
            nickname: Some("for-testing-only".to_owned()),
            ..Default::default()
        };
        let window_handle = cx.open_window(
            gpui::size(gpui::px(376.), gpui::px(600.)),
            move |window, cx| {
                let workspace = cx.new(|cx| Workspace::test_new(project, window, cx));
                let popover = cx.new(|cx| {
                    PortForwardingPopover::new(options, workspace.downgrade(), fs, true, window, cx)
                });
                PortForwardingTestRoot {
                    _workspace: workspace,
                    popover,
                    _dismiss_subscription: None,
                }
            },
        );
        let mut visual_cx = gpui::VisualTestContext::from_window(*window_handle.deref(), cx);
        visual_cx.run_until_parked();

        let Some(popover_bounds) = visual_cx.debug_bounds("port-forwarding-popover") else {
            panic!("port forwarding popover was not rendered");
        };
        assert!(popover_bounds.size.width <= gpui::px(360.));

        click_port_forwarding_button(&mut visual_cx, "add-port-forward");
        let Some(local_bounds) = visual_cx.debug_bounds("port-forward-local-endpoint") else {
            panic!("local endpoint field was not rendered");
        };
        let Some(remote_bounds) = visual_cx.debug_bounds("port-forward-remote-endpoint") else {
            panic!("remote endpoint field was not rendered");
        };
        assert_eq!(
            local_bounds.origin.y, remote_bounds.origin.y,
            "the editor should remain horizontal at 360px of available width"
        );
    }

    #[gpui::test]
    async fn add_edit_and_validate_port_forward_inputs(cx: &mut TestAppContext) {
        let app_state = cx.update(|cx| {
            let app_state = AppState::test(cx);
            crate::init(cx);
            editor::init(cx);
            app_state
        });
        let fs = app_state.fs.as_fake();
        let project = Project::test(fs.clone(), [], cx).await;
        let options = SshConnectionOptions {
            host: "for-testing-only".to_owned().into(),
            nickname: Some("for-testing-only".to_owned()),
            ..Default::default()
        };
        let (root, cx) = cx.add_window_view(move |window, cx| {
            let workspace = cx.new(|cx| Workspace::test_new(project, window, cx));
            let popover = cx.new(|cx| {
                PortForwardingPopover::new(options, workspace.downgrade(), fs, true, window, cx)
            });
            PortForwardingTestRoot {
                _workspace: workspace,
                popover,
                _dismiss_subscription: None,
            }
        });
        let popover = root.read_with(cx, |root, _| root.popover.clone());
        let added = forward(Some("localhost"), 8080, Some("service"), 9000);
        let edited = forward(Some("localhost"), 8081, Some("service"), 9001);

        click_port_forwarding_button(cx, "add-port-forward");
        set_endpoint_values(&popover, "localhost:8080", "service:9000", cx);
        click_port_forwarding_button(cx, "save-port-forward");
        assert_eq!(
            popover.read_with(cx, |popover, _| popover.forwards.clone()),
            vec![forward(None, 80, None, 80), added.clone()]
        );
        assert!(
            popover.read_with(cx, |popover, _| popover.editor.is_none()),
            "saving a valid forward should close the editor"
        );

        click_port_forwarding_button(cx, "edit-port-forward-1");
        set_endpoint_values(&popover, "localhost:8081", "service:9001", cx);
        click_port_forwarding_button(cx, "save-port-forward");
        assert_eq!(
            popover.read_with(cx, |popover, _| popover.forwards.clone()),
            vec![forward(None, 80, None, 80), edited]
        );

        click_port_forwarding_button(cx, "add-port-forward");
        set_endpoint_values(&popover, "invalid input", "80", cx);
        click_port_forwarding_button(cx, "save-port-forward");
        assert_eq!(
            popover.read_with(cx, |popover, _| {
                popover.message.as_deref().map(str::to_owned)
            }),
            Some(INVALID_FORWARD_MESSAGE.to_owned())
        );
        assert!(
            popover.read_with(cx, |popover, _| popover.editor.is_some()),
            "invalid input should remain editable"
        );
        assert!(
            cx.debug_bounds("port-forward-error").is_some(),
            "invalid input should render an inline error"
        );

        click_port_forwarding_button(cx, "cancel-port-forward");
        assert_eq!(
            popover.read_with(cx, |popover, _| popover.forwards.clone()),
            vec![
                forward(None, 80, None, 80),
                forward(Some("localhost"), 8081, Some("service"), 9001)
            ]
        );
    }

    #[gpui::test]
    async fn non_preview_add_edit_and_delete_only_update_the_matching_ssh_entry(
        cx: &mut TestAppContext,
    ) {
        let app_state = cx.update(|cx| {
            let app_state = AppState::test(cx);
            crate::init(cx);
            editor::init(cx);
            app_state
        });
        let fs = app_state.fs.as_fake();
        let target_identity = SshConnectionIdentity {
            host: "example.com".to_owned(),
            username: Some("zed".to_owned()),
            port: Some(2222),
        };
        let settings_identity = target_identity.clone();
        let initial_forward = forward(None, 8080, None, 80);
        let unrelated_forward = forward(Some("localhost"), 7070, Some("other"), 70);
        let initial_settings_write = cx.update(|cx| {
            update_settings_file_with_completion(fs.clone(), cx, move |settings, _| {
                settings.remote.ssh_connections = Some(vec![
                    SshConnection {
                        host: settings_identity.host,
                        username: settings_identity.username,
                        port: settings_identity.port,
                        nickname: Some("Development server".to_owned()),
                        args: vec!["-C".to_owned()],
                        port_forwards: Some(vec![initial_forward]),
                        ..Default::default()
                    },
                    SshConnection {
                        host: "unrelated.example.com".to_owned(),
                        username: Some("other".to_owned()),
                        port: Some(2201),
                        nickname: Some("Other server".to_owned()),
                        args: vec!["-A".to_owned()],
                        port_forwards: Some(vec![unrelated_forward]),
                        ..Default::default()
                    },
                ]);
            })
        });
        initial_settings_write
            .await
            .expect("initial settings update should complete")
            .expect("initial settings should be written");
        let project = Project::test(fs.clone(), [], cx).await;
        let options = SshConnectionOptions {
            host: "example.com".to_owned().into(),
            username: Some("zed".to_owned()),
            port: Some(2222),
            nickname: Some("Development server".to_owned()),
            ..Default::default()
        };
        let identity = target_identity;
        let (root, cx) = cx.add_window_view(move |window, cx| {
            let workspace = cx.new(|cx| Workspace::test_new(project, window, cx));
            let popover = cx.new(|cx| {
                PortForwardingPopover::new(options, workspace.downgrade(), fs, false, window, cx)
            });
            PortForwardingTestRoot {
                _workspace: workspace,
                popover,
                _dismiss_subscription: None,
            }
        });
        let popover = root.read_with(cx, |root, _| root.popover.clone());
        let edited_forward = forward(Some("localhost"), 8081, Some("service"), 9001);
        let added_forward = forward(Some("localhost"), 8082, Some("service"), 9002);

        click_port_forwarding_button(cx, "edit-port-forward-0");
        set_endpoint_values(&popover, "localhost:8081", "service:9001", cx);
        click_port_forwarding_button(cx, "save-port-forward");
        assert_eq!(
            popover.read_with(cx, |popover, _| popover.forwards.clone()),
            vec![edited_forward.clone()]
        );
        assert_persisted_connections(cx, &identity, &[edited_forward.clone()]);

        click_port_forwarding_button(cx, "add-port-forward");
        set_endpoint_values(&popover, "localhost:8082", "service:9002", cx);
        click_port_forwarding_button(cx, "save-port-forward");
        assert_eq!(
            popover.read_with(cx, |popover, _| popover.forwards.clone()),
            vec![edited_forward.clone(), added_forward.clone()]
        );
        assert_persisted_connections(cx, &identity, &[edited_forward, added_forward.clone()]);

        click_port_forwarding_button(cx, "delete-port-forward-0");
        assert!(cx.has_pending_prompt());
        cx.simulate_prompt_answer("Yes, remove it");
        cx.run_until_parked();
        assert_eq!(
            popover.read_with(cx, |popover, _| popover.forwards.clone()),
            vec![added_forward.clone()]
        );
        assert_persisted_connections(cx, &identity, &[added_forward]);
    }

    #[gpui::test]
    async fn non_preview_cancel_invalid_stale_and_ambiguous_settings_do_not_change_forwards(
        cx: &mut TestAppContext,
    ) {
        let app_state = cx.update(|cx| {
            let app_state = AppState::test(cx);
            crate::init(cx);
            editor::init(cx);
            app_state
        });
        let fs = app_state.fs.as_fake();
        let identity = SshConnectionIdentity {
            host: "example.com".to_owned(),
            username: Some("zed".to_owned()),
            port: Some(2222),
        };
        let original_forward = forward(None, 8080, None, 80);
        let initial_connection = SshConnection {
            host: identity.host.clone(),
            username: identity.username.clone(),
            port: identity.port,
            nickname: Some("Development server".to_owned()),
            args: vec!["-C".to_owned()],
            port_forwards: Some(vec![original_forward.clone()]),
            ..Default::default()
        };
        let settings_connection = initial_connection.clone();
        let initial_settings_write = cx.update(|cx| {
            update_settings_file_with_completion(fs.clone(), cx, move |settings, _| {
                settings.remote.ssh_connections = Some(vec![settings_connection]);
            })
        });
        initial_settings_write
            .await
            .expect("initial settings update should complete")
            .expect("initial settings should be written");
        let project = Project::test(fs.clone(), [], cx).await;
        let options = SshConnectionOptions {
            host: identity.host.clone().into(),
            username: identity.username.clone(),
            port: identity.port,
            nickname: Some("Development server".to_owned()),
            ..Default::default()
        };
        let (root, cx) = cx.add_window_view(move |window, cx| {
            let workspace = cx.new(|cx| Workspace::test_new(project, window, cx));
            let popover = cx.new(|cx| {
                PortForwardingPopover::new(options, workspace.downgrade(), fs, false, window, cx)
            });
            PortForwardingTestRoot {
                _workspace: workspace,
                popover,
                _dismiss_subscription: None,
            }
        });
        let popover = root.read_with(cx, |root, _| root.popover.clone());

        click_port_forwarding_button(cx, "add-port-forward");
        click_port_forwarding_button(cx, "cancel-port-forward");
        assert_eq!(read_ssh_connections(cx), vec![initial_connection.clone()]);

        click_port_forwarding_button(cx, "add-port-forward");
        set_endpoint_values(&popover, "invalid input", "80", cx);
        click_port_forwarding_button(cx, "save-port-forward");
        assert_eq!(
            popover.read_with(cx, |popover, _| {
                popover.message.as_deref().map(str::to_owned)
            }),
            Some(INVALID_FORWARD_MESSAGE.to_owned())
        );
        assert!(
            cx.debug_bounds("port-forward-error").is_some(),
            "invalid input should render an inline error"
        );
        assert_eq!(read_ssh_connections(cx), vec![initial_connection.clone()]);
        click_port_forwarding_button(cx, "cancel-port-forward");

        click_port_forwarding_button(cx, "edit-port-forward-0");
        set_endpoint_values(&popover, "localhost:8081", "service:9001", cx);
        let stale_forward = forward(None, 8081, None, 80);
        let stale_connection = SshConnection {
            port_forwards: Some(vec![stale_forward.clone()]),
            ..initial_connection.clone()
        };
        write_remote_connections(cx, fs.clone(), vec![stale_connection.clone()]).await;
        assert_eq!(
            popover.read_with(cx, |popover, _| popover.forwards.clone()),
            vec![stale_forward.clone()]
        );

        click_port_forwarding_button(cx, "save-port-forward");
        assert_eq!(
            popover.read_with(cx, |popover, _| {
                popover.message.as_deref().map(str::to_owned)
            }),
            Some(ForwardUpdateError::ForwardChanged.message().to_owned())
        );
        assert_eq!(read_ssh_connections(cx), vec![stale_connection.clone()]);
        assert!(
            popover.read_with(cx, |popover, _| popover.editor.is_some()),
            "stale edits should remain open for the user to review"
        );
        click_port_forwarding_button(cx, "cancel-port-forward");

        let ambiguous_connection = SshConnection {
            port_forwards: Some(vec![forward(None, 8082, None, 80)]),
            ..initial_connection.clone()
        };
        let ambiguous_connections = vec![stale_connection, ambiguous_connection];
        write_remote_connections(cx, fs.clone(), ambiguous_connections.clone()).await;
        assert_eq!(
            popover.read_with(cx, |popover, _| {
                popover.message.as_deref().map(str::to_owned)
            }),
            Some(ForwardUpdateError::AmbiguousConnection.message().to_owned())
        );
        click_port_forwarding_button(cx, "add-port-forward");
        set_endpoint_values(&popover, "localhost:8083", "service:9003", cx);
        click_port_forwarding_button(cx, "save-port-forward");
        assert_eq!(
            popover.read_with(cx, |popover, _| {
                popover.message.as_deref().map(str::to_owned)
            }),
            Some(ForwardUpdateError::AmbiguousConnection.message().to_owned())
        );
        assert_eq!(read_ssh_connections(cx), ambiguous_connections);
        assert!(
            cx.debug_bounds("port-forward-error").is_some(),
            "ambiguous settings should render an inline error"
        );
    }

    #[gpui::test]
    async fn settings_write_failure_keeps_the_forward_and_shows_an_error(cx: &mut TestAppContext) {
        let app_state = cx.update(|cx| {
            let app_state = AppState::test(cx);
            crate::init(cx);
            editor::init(cx);
            app_state
        });
        let fs = app_state.fs.as_fake();
        let initial_forward = forward(None, 8080, None, 80);
        let initial_connection = SshConnection {
            host: "example.com".to_owned(),
            port_forwards: Some(vec![initial_forward.clone()]),
            ..Default::default()
        };
        let settings_connection = initial_connection.clone();
        let initial_settings_write = cx.update(|cx| {
            update_settings_file_with_completion(fs.clone(), cx, move |settings, _| {
                settings.remote.ssh_connections = Some(vec![settings_connection]);
            })
        });
        initial_settings_write
            .await
            .expect("initial settings update should complete")
            .expect("initial settings should be written");
        let initial_settings_text = fs
            .load(paths::settings_file())
            .await
            .expect("initial settings should be readable");
        let project = Project::test(fs.clone(), [], cx).await;
        let options = SshConnectionOptions {
            host: "example.com".to_owned().into(),
            ..Default::default()
        };
        let (root, cx) = cx.add_window_view(move |window, cx| {
            let workspace = cx.new(|cx| Workspace::test_new(project, window, cx));
            let popover = cx.new(|cx| {
                PortForwardingPopover::new(options, workspace.downgrade(), fs, false, window, cx)
            });
            PortForwardingTestRoot {
                _workspace: workspace,
                popover,
                _dismiss_subscription: None,
            }
        });
        let popover = root.read_with(cx, |root, _| root.popover.clone());
        fs.fail_next_atomic_write("simulated settings write failure");

        click_port_forwarding_button(cx, "edit-port-forward-0");
        set_endpoint_values(&popover, "localhost:8081", "service:9001", cx);
        click_port_forwarding_button(cx, "save-port-forward");

        assert_eq!(
            popover.read_with(cx, |popover, _| popover.forwards.clone()),
            vec![initial_forward]
        );
        assert!(
            popover.read_with(cx, |popover, _| popover.editor.is_some()),
            "a failed write should keep the editor open"
        );
        assert!(
            !popover.read_with(cx, |popover, _| popover.is_saving),
            "saving state should clear after a failed write"
        );
        let error_message = popover.read_with(cx, |popover, _| popover.message.clone());
        assert!(
            error_message
                .as_deref()
                .is_some_and(|message| message.contains("simulated settings write failure")),
            "the write failure should be shown to the user"
        );
        assert!(cx.debug_bounds("port-forward-error").is_some());
        assert_eq!(
            fs.load(paths::settings_file())
                .await
                .expect("settings should remain readable"),
            initial_settings_text
        );
        assert_eq!(read_ssh_connections(cx), vec![initial_connection]);
    }

    #[gpui::test]
    async fn deleting_a_port_forward_requires_confirmation(cx: &mut TestAppContext) {
        let app_state = cx.update(|cx| {
            let app_state = AppState::test(cx);
            crate::init(cx);
            editor::init(cx);
            app_state
        });
        let fs = app_state.fs.as_fake();
        let project = Project::test(fs.clone(), [], cx).await;
        let options = SshConnectionOptions {
            host: "for-testing-only".to_owned().into(),
            nickname: Some("for-testing-only".to_owned()),
            ..Default::default()
        };
        let (root, cx) = cx.add_window_view(move |window, cx| {
            let workspace = cx.new(|cx| Workspace::test_new(project, window, cx));
            let popover = cx.new(|cx| {
                PortForwardingPopover::new(options, workspace.downgrade(), fs, true, window, cx)
            });
            PortForwardingTestRoot {
                _workspace: workspace,
                popover,
                _dismiss_subscription: None,
            }
        });
        let popover = root.read_with(cx, |root, _| root.popover.clone());

        assert!(cx.debug_bounds("edit-port-forward-0").is_some());
        assert!(cx.debug_bounds("delete-port-forward-0").is_some());
        click_port_forwarding_button(cx, "delete-port-forward-0");
        assert!(cx.has_pending_prompt());
        cx.simulate_prompt_answer("No, keep it");
        cx.run_until_parked();
        assert_eq!(
            popover.read_with(cx, |popover, _| popover.forwards.len()),
            1,
            "declining confirmation should keep the forward"
        );

        click_port_forwarding_button(cx, "delete-port-forward-0");
        assert!(cx.has_pending_prompt());
        cx.simulate_prompt_answer("Yes, remove it");
        cx.run_until_parked();
        assert!(popover.read_with(cx, |popover, _| popover.forwards.is_empty()));
    }

    #[gpui::test]
    async fn clicking_outside_the_port_forwarding_popover_emits_dismiss(cx: &mut TestAppContext) {
        let app_state = cx.update(|cx| {
            let app_state = AppState::test(cx);
            crate::init(cx);
            editor::init(cx);
            app_state
        });
        let fs = app_state.fs.as_fake();
        let project = Project::test(fs.clone(), [], cx).await;
        let options = SshConnectionOptions {
            host: "for-testing-only".to_owned().into(),
            nickname: Some("for-testing-only".to_owned()),
            ..Default::default()
        };
        let dismiss_event_received = Rc::new(Cell::new(false));
        let dismiss_event_for_subscription = dismiss_event_received.clone();
        let (_root, cx) = cx.add_window_view(move |window, cx| {
            let workspace = cx.new(|cx| Workspace::test_new(project, window, cx));
            let popover = cx.new(|cx| {
                PortForwardingPopover::new(options, workspace.downgrade(), fs, true, window, cx)
            });
            let dismiss_subscription = cx.subscribe(&popover, move |_, _, _: &DismissEvent, _| {
                dismiss_event_for_subscription.set(true);
            });
            PortForwardingTestRoot {
                _workspace: workspace,
                popover,
                _dismiss_subscription: Some(dismiss_subscription),
            }
        });
        let Some(popover_bounds) = cx.debug_bounds("port-forwarding-popover") else {
            panic!("port forwarding popover was not rendered");
        };

        cx.simulate_click(
            gpui::point(
                popover_bounds.right() + gpui::px(20.),
                popover_bounds.bottom() + gpui::px(20.),
            ),
            gpui::Modifiers::default(),
        );
        cx.run_until_parked();
        assert!(dismiss_event_received.get());
    }

    #[gpui::test]
    async fn mounted_port_forwarding_menu_closes_after_outside_click(cx: &mut TestAppContext) {
        let app_state = cx.update(|cx| {
            let app_state = AppState::test(cx);
            crate::init(cx);
            editor::init(cx);
            app_state
        });
        let fs = app_state.fs.as_fake();
        let project = Project::test(fs.clone(), [], cx).await;
        let options = SshConnectionOptions {
            host: "for-testing-only".to_owned().into(),
            nickname: Some("for-testing-only".to_owned()),
            ..Default::default()
        };
        let (_root, cx) = cx.add_window_view(move |window, cx| {
            let workspace = cx.new(|cx| Workspace::test_new(project, window, cx));
            PortForwardingMenuTestRoot {
                _workspace: workspace.clone(),
                workspace: workspace.downgrade(),
                fs,
                options,
            }
        });

        click_port_forwarding_button(cx, "test-port-forwarding-trigger");
        let Some(popover_bounds) = cx.debug_bounds("port-forwarding-popover") else {
            panic!("mounted port forwarding popover was not opened");
        };
        cx.simulate_click(
            gpui::point(
                popover_bounds.right() + gpui::px(20.),
                popover_bounds.bottom() + gpui::px(20.),
            ),
            gpui::Modifiers::default(),
        );
        cx.run_until_parked();
        assert!(
            cx.debug_bounds("port-forwarding-popover").is_none(),
            "the mounted menu should close after an outside click"
        );

        click_port_forwarding_button(cx, "test-port-forwarding-trigger");
        assert!(cx.debug_bounds("port-forwarding-popover").is_some());
    }

    fn click_port_forwarding_button(cx: &mut gpui::VisualTestContext, button_id: &'static str) {
        let Some(bounds) = cx.debug_bounds(button_id) else {
            panic!("port forwarding button {button_id:?} was not rendered");
        };
        cx.simulate_click(bounds.center(), gpui::Modifiers::default());
        cx.run_until_parked();
    }

    async fn write_remote_connections(
        cx: &mut gpui::VisualTestContext,
        fs: Arc<dyn Fs>,
        connections: Vec<SshConnection>,
    ) {
        let completion = cx.update(|_, cx| {
            update_settings_file_with_completion(fs, cx, move |settings, _| {
                settings.remote.ssh_connections = Some(connections);
            })
        });
        completion
            .await
            .expect("SSH settings update should complete")
            .expect("SSH settings should be written");
        cx.run_until_parked();
    }

    fn set_endpoint_values(
        popover: &Entity<PortForwardingPopover>,
        local_text: &str,
        remote_text: &str,
        cx: &mut gpui::VisualTestContext,
    ) {
        let editors = popover.read_with(cx, |popover, _| {
            popover
                .editor
                .as_ref()
                .map(|editor| (editor.local.clone(), editor.remote.clone()))
        });
        let Some((local_editor, remote_editor)) = editors else {
            panic!("port forward endpoint editors were not rendered");
        };
        let local_text = local_text.to_owned();
        let remote_text = remote_text.to_owned();

        cx.update(|window, cx| {
            local_editor.update(cx, |editor, cx| {
                editor.set_text(local_text, window, cx);
            });
            remote_editor.update(cx, |editor, cx| {
                editor.set_text(remote_text, window, cx);
            });
        });
        cx.run_until_parked();
    }

    fn assert_tab_moves_between_endpoint_fields(
        popover: &Entity<PortForwardingPopover>,
        cx: &mut gpui::VisualTestContext,
    ) {
        let focus_handles = popover.read_with(cx, |popover, cx| {
            popover.editor.as_ref().map(|editor| {
                (
                    editor.local.focus_handle(cx),
                    editor.remote.focus_handle(cx),
                )
            })
        });
        let Some((local_focus, remote_focus)) = focus_handles else {
            panic!("port forward endpoint editors were not rendered");
        };
        let Some(local_bounds) = cx.debug_bounds("port-forward-local-endpoint") else {
            panic!("local endpoint field was not rendered");
        };
        let Some(remote_bounds) = cx.debug_bounds("port-forward-remote-endpoint") else {
            panic!("remote endpoint field was not rendered");
        };
        assert_eq!(
            local_bounds.origin.y, remote_bounds.origin.y,
            "wide layout should keep the endpoint fields on one row"
        );

        cx.update(|window, _| assert!(local_focus.is_focused(window)));
        cx.simulate_keystrokes("tab");
        cx.update(|window, _| assert!(remote_focus.is_focused(window)));
        cx.simulate_keystrokes("shift-tab");
        cx.update(|window, _| assert!(local_focus.is_focused(window)));
    }
}
