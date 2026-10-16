use crate::RemoteSettings;
use editor::Editor;
use futures::channel::oneshot;
use gpui::{
    App, Context, DismissEvent, Div, Entity, EventEmitter, FocusHandle, Focusable, IntoElement,
    ParentElement, PromptLevel, Render, SharedString, Styled, Subscription, WeakEntity, Window,
    div, px,
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

fn port_forward_endpoint_input(editor: &Entity<Editor>, cx: &App) -> Div {
    div()
        .track_focus(&editor.focus_handle(cx).tab_stop(true))
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
                    list = list.child(self.render_editor(editor, window, cx));
                    editor_rendered = true;
                }
            } else {
                list = list.child(self.render_forward_row(index, forward, window, cx));
            }
        }

        if !editor_rendered {
            if let Some(editor) = &self.editor {
                list = list.child(self.render_editor(editor, window, cx));
            }
        }

        v_flex()
            .id("port-forwarding-popover")
            .track_focus(&self.focus_handle)
            .tab_group()
            .w(px(384.))
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
                    Label::new(message)
                        .size(LabelSize::Small)
                        .color(Color::Error),
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
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        h_flex()
            .id("port-forward-editor")
            .w_full()
            .gap_1()
            .child(
                port_forward_endpoint_input(&editor.local, cx)
                    .flex_1()
                    .min_w_0(),
            )
            .child(Icon::new(IconName::ArrowRight).color(Color::Muted))
            .child(
                port_forward_endpoint_input(&editor.remote, cx)
                    .flex_1()
                    .min_w_0(),
            )
            .child(
                IconButton::new("save-port-forward", IconName::Check)
                    .shape(IconButtonShape::Square)
                    .icon_size(IconSize::Small)
                    .aria_label("Save port forward")
                    .tooltip(Tooltip::text("Save Port Forward"))
                    .disabled(self.is_saving)
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.save_edit(window, cx);
                    })),
            )
            .child(
                IconButton::new("cancel-port-forward", IconName::Close)
                    .shape(IconButtonShape::Square)
                    .icon_size(IconSize::Small)
                    .aria_label("Cancel port forward edit")
                    .tooltip(Tooltip::text("Cancel"))
                    .disabled(self.is_saving)
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.cancel_edit(cx);
                    })),
            )
            .on_action(cx.listener(|this, _: &menu::Confirm, window, cx| {
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

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{KeyBinding, TestAppContext};

    struct EndpointFocusFixture {
        focus_handle: FocusHandle,
        local_editor: Entity<Editor>,
        remote_editor: Entity<Editor>,
    }

    impl Render for EndpointFocusFixture {
        fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
            v_flex()
                .id("port-forward-endpoint-focus-test")
                .track_focus(&self.focus_handle)
                .tab_group()
                .on_action(|_: &menu::SelectNext, window, cx| window.focus_next(cx))
                .on_action(|_: &menu::SelectPrevious, window, cx| window.focus_prev(cx))
                .on_action(|_: &editor::actions::Tab, window, cx| window.focus_next(cx))
                .on_action(|_: &editor::actions::Backtab, window, cx| window.focus_prev(cx))
                .child(Button::new("existing-forward-action", "Existing forward").tab_index(0))
                .child(port_forward_endpoint_input(&self.local_editor, cx))
                .child(port_forward_endpoint_input(&self.remote_editor, cx))
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

    #[gpui::test]
    fn tab_moves_between_port_forward_endpoint_inputs(cx: &mut TestAppContext) {
        cx.update(|_, cx| {
            cx.bind_keys([
                KeyBinding::new("tab", menu::SelectNext, None),
                KeyBinding::new("shift-tab", menu::SelectPrevious, None),
                KeyBinding::new("tab", editor::actions::Tab, Some("Editor")),
                KeyBinding::new("shift-tab", editor::actions::Backtab, Some("Editor")),
            ]);
        });

        let (fixture, cx) = cx.add_window_view(|window, cx| EndpointFocusFixture {
            focus_handle: cx.focus_handle(),
            local_editor: cx.new(|cx| Editor::single_line(window, cx)),
            remote_editor: cx.new(|cx| Editor::single_line(window, cx)),
        });
        let (local_focus, remote_focus) = fixture.read_with(cx, |fixture, cx| {
            (
                fixture.local_editor.focus_handle(cx),
                fixture.remote_editor.focus_handle(cx),
            )
        });
        cx.update(|window, cx| window.focus(&local_focus, cx));

        cx.simulate_keystrokes("tab");
        cx.update(|window, _| {
            assert!(remote_focus.is_focused(window));
        });

        cx.simulate_keystrokes("shift-tab");
        cx.update(|window, _| {
            assert!(local_focus.is_focused(window));
        });
    }
}
