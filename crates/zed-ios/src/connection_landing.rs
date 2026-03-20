use std::path::PathBuf;
use std::sync::Arc;

use askpass::EncryptedPassword;
use editor::{Editor, actions::{Backtab, Tab}};
use gpui::{
    App, AppContext as _, AsyncApp, ClickEvent, Context, Entity, Focusable as _, IntoElement,
    Render, SharedString, Task, Window, WindowOptions, div, prelude::*,
};
use remote::SshConnectionOptions;
use serde::{Deserialize, Serialize};
use util::ResultExt;
use theme::ActiveTheme;
use ui::{
    ButtonCommon, ButtonStyle, Clickable, Color, FixedWidth, Headline, Icon, IconButton, IconName,
    IconSize, Indicator, Label, LabelCommon, LabelSize, Vector, VectorName, h_flex, rems_from_px,
    v_flex,
};

/// On-disk representation of a saved SSH host.
#[derive(Clone, Serialize, Deserialize)]
struct SavedHostEntry {
    nickname: Option<String>,
    host: String,
    username: String,
    #[serde(default = "default_port")]
    port: u16,
    #[serde(default)]
    password: Option<String>,
}

fn default_port() -> u16 {
    22
}

fn saved_hosts_path() -> PathBuf {
    paths::config_dir().join("ssh_hosts.json")
}

fn load_saved_host_entries() -> Vec<SavedHostEntry> {
    let path = saved_hosts_path();
    match std::fs::read_to_string(&path) {
        Ok(contents) => serde_json::from_str(&contents).unwrap_or_default(),
        Err(_) => Vec::new(),
    }
}

fn save_host_entries(entries: &[SavedHostEntry]) {
    let path = saved_hosts_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).log_err();
    }
    match serde_json::to_string_pretty(entries) {
        Ok(json) => {
            std::fs::write(&path, json).log_err();
        }
        Err(error) => {
            log::error!("failed to serialize saved hosts: {error}");
        }
    }
}

/// Status of a saved host connection.
enum ConnectionStatus {
    Disconnected,
    Connecting,
    Connected { project_count: usize },
    Error(SharedString),
}

/// A saved SSH host displayed in the landing list.
struct SavedHost {
    nickname: Option<SharedString>,
    host: SharedString,
    username: SharedString,
    port: u16,
    password: Option<String>,
    status: ConnectionStatus,
}

impl SavedHost {
    fn from_entry(entry: SavedHostEntry) -> Self {
        Self {
            nickname: entry.nickname.map(SharedString::from),
            host: SharedString::from(entry.host),
            username: SharedString::from(entry.username),
            port: entry.port,
            password: entry.password,
            status: ConnectionStatus::Disconnected,
        }
    }

    fn to_entry(&self) -> SavedHostEntry {
        SavedHostEntry {
            nickname: self.nickname.as_ref().map(|s| s.to_string()),
            host: self.host.to_string(),
            username: self.username.to_string(),
            port: self.port,
            password: self.password.clone(),
        }
    }

    fn display_name(&self) -> SharedString {
        if let Some(nickname) = &self.nickname {
            nickname.clone()
        } else {
            SharedString::from(format!("{}@{}", self.username, self.host))
        }
    }

    fn address_line(&self) -> SharedString {
        if self.port == 22 {
            SharedString::from(format!("{}@{}", self.username, self.host))
        } else {
            SharedString::from(format!("{}@{}:{}", self.username, self.host, self.port))
        }
    }

    fn status_color(&self) -> Color {
        match &self.status {
            ConnectionStatus::Disconnected => Color::Muted,
            ConnectionStatus::Connecting => Color::Warning,
            ConnectionStatus::Connected { .. } => Color::Success,
            ConnectionStatus::Error(_) => Color::Error,
        }
    }

    fn status_label(&self) -> SharedString {
        match &self.status {
            ConnectionStatus::Disconnected => "Disconnected".into(),
            ConnectionStatus::Connecting => "Connecting\u{2026}".into(),
            ConnectionStatus::Connected { project_count } => {
                let suffix = if *project_count == 1 {
                    "project"
                } else {
                    "projects"
                };
                SharedString::from(format!("{project_count} {suffix}"))
            }
            ConnectionStatus::Error(message) => message.clone(),
        }
    }
}

enum LandingMode {
    Default,
    AddHost,
}

/// Landing screen shown on iPad launch. Lists saved SSH hosts and provides
/// an "Add Host" entry point. This replaces the desktop welcome page — the
/// thin client has no local filesystem, so the first thing a user does is
/// pick a remote host.
pub struct ConnectionLanding {
    focus_handle: gpui::FocusHandle,
    mode: LandingMode,
    editing_hosts: bool,
    saved_hosts: Vec<SavedHost>,
    name_editor: Entity<Editor>,
    host_editor: Entity<Editor>,
    username_editor: Entity<Editor>,
    port_editor: Entity<Editor>,
    password_editor: Entity<Editor>,
    _connect_task: Option<Task<()>>,
}

impl ConnectionLanding {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let name_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("optional display name", window, cx);
            editor
        });
        let host_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("hostname or IP address", window, cx);
            editor
        });
        let username_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("username", window, cx);
            editor
        });
        let port_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("22", window, cx);
            editor
        });
        let password_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("leave blank for key auth", window, cx);
            editor.set_masked(true, cx);
            editor
        });

        let saved_hosts = load_saved_host_entries()
            .into_iter()
            .map(SavedHost::from_entry)
            .collect();

        Self {
            focus_handle: cx.focus_handle(),
            mode: LandingMode::Default,
            editing_hosts: false,
            saved_hosts,
            name_editor,
            host_editor,
            username_editor,
            port_editor,
            password_editor,
            _connect_task: None,
        }
    }

    /// Open the connection landing screen in a new window.
    pub fn open(cx: &mut App) -> anyhow::Result<()> {
        cx.open_window(WindowOptions::default(), |window, cx| {
            let landing = cx.new(|cx| Self::new(window, cx));
            landing.focus_handle(cx).focus(window, cx);
            landing
        })?;
        Ok(())
    }

    fn persist_hosts(&self) {
        let entries: Vec<SavedHostEntry> = self.saved_hosts.iter().map(|h| h.to_entry()).collect();
        save_host_entries(&entries);
    }

    fn switch_to_add_host(
        &mut self,
        _event: &ClickEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.mode = LandingMode::AddHost;
        self.name_editor.update(cx, |editor, cx| {
            editor.set_text("", window, cx);
        });
        self.host_editor.update(cx, |editor, cx| {
            editor.set_text("", window, cx);
        });
        self.username_editor.update(cx, |editor, cx| {
            editor.set_text("", window, cx);
        });
        self.port_editor.update(cx, |editor, cx| {
            editor.set_text("", window, cx);
        });
        self.password_editor.update(cx, |editor, cx| {
            editor.set_text("", window, cx);
        });
        self.name_editor.focus_handle(cx).focus(window, cx);
        cx.notify();
    }

    fn cancel_add_host(
        &mut self,
        _event: &ClickEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.mode = LandingMode::Default;
        self.focus_handle.focus(window, cx);
        cx.notify();
    }

    fn confirm_add_host(
        &mut self,
        _event: &ClickEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let name = self.name_editor.read(cx).text(cx);
        let host = self.host_editor.read(cx).text(cx);
        let username = self.username_editor.read(cx).text(cx);
        let port_text = self.port_editor.read(cx).text(cx);
        let port: u16 = port_text.parse().unwrap_or(22);
        let password_text = self.password_editor.read(cx).text(cx);
        let password = if password_text.is_empty() {
            None
        } else {
            Some(password_text.to_string())
        };

        if host.is_empty() || username.is_empty() {
            return;
        }

        let nickname = if name.is_empty() {
            None
        } else {
            Some(SharedString::from(name))
        };

        self.saved_hosts.push(SavedHost {
            nickname,
            host: SharedString::from(host),
            username: SharedString::from(username),
            port,
            password,
            status: ConnectionStatus::Disconnected,
        });
        self.persist_hosts();

        self.mode = LandingMode::Default;
        self.focus_handle.focus(window, cx);
        cx.notify();
    }

    fn connect_host(&mut self, index: usize, _window: &mut Window, cx: &mut Context<Self>) {
        let Some(host) = self.saved_hosts.get_mut(index) else {
            return;
        };
        host.status = ConnectionStatus::Connecting;
        cx.notify();

        let connection_options = SshConnectionOptions {
            host: host.host.to_string().into(),
            username: Some(host.username.to_string()),
            port: Some(host.port),
            password: host.password.clone(),
            ..Default::default()
        };

        let app_state = match crate::ios::app_state() {
            Some(state) => state,
            None => {
                log::error!("[zed-ios] app_state not available for SSH connection");
                if let Some(host) = self.saved_hosts.get_mut(index) {
                    host.status =
                        ConnectionStatus::Error("App not initialized".into());
                    cx.notify();
                }
                return;
            }
        };

        let delegate = Arc::new(IosRemoteClientDelegate);

        self._connect_task = Some(cx.spawn(async move |this, cx| {
            let result = Self::do_connect(
                connection_options,
                delegate.clone(),
                app_state,
                cx,
            )
            .await;

            match result {
                Ok(()) => {
                    this.update(cx, |this, cx| {
                        if let Some(host) = this.saved_hosts.get_mut(index) {
                            host.status =
                                ConnectionStatus::Connected { project_count: 0 };
                            cx.notify();
                        }
                    })
                    .log_err();
                }
                Err(error) => {
                    log::error!("[zed-ios] SSH connection failed: {error:#}");
                    this.update(cx, |this, cx| {
                        if let Some(host) = this.saved_hosts.get_mut(index) {
                            host.status = ConnectionStatus::Error(
                                SharedString::from(format!("{error:#}")),
                            );
                            cx.notify();
                        }
                    })
                    .log_err();
                }
            }
        }));
    }

    async fn do_connect(
        connection_options: SshConnectionOptions,
        delegate: Arc<dyn remote::RemoteClientDelegate>,
        app_state: Arc<workspace::AppState>,
        cx: &mut AsyncApp,
    ) -> anyhow::Result<()> {
        use remote::RusshRemoteConnection;

        let connection = RusshRemoteConnection::new(
            connection_options.clone(),
            delegate.clone(),
            cx,
        )
        .await?;

        let remote_connection: Arc<dyn remote::RemoteConnection> = Arc::new(connection);

        // Open a new workspace window with the remote project
        let (cancel_tx, cancel_rx) = futures::channel::oneshot::channel::<()>();
        // Keep cancel_tx alive so the connection isn't cancelled
        std::mem::forget(cancel_tx);

        let window = cx.open_window(WindowOptions::default(), |window, cx| {
            let project = project::Project::local(
                app_state.client.clone(),
                app_state.node_runtime.clone(),
                app_state.user_store.clone(),
                app_state.languages.clone(),
                app_state.fs.clone(),
                None,
                project::LocalProjectFlags::default(),
                cx,
            );
            let workspace_entity =
                cx.new(|cx| workspace::Workspace::new(None, project, app_state.clone(), window, cx));
            cx.new(|cx| workspace::MultiWorkspace::new(workspace_entity, window, cx))
        })?;

        // Open the remote project in the new window using
        // the home directory on the remote as initial path
        let paths = vec![PathBuf::from("~")];
        cx.update(|cx| {
            workspace::open_remote_project_with_new_connection(
                window,
                remote_connection,
                cancel_rx,
                delegate,
                app_state,
                paths,
                cx,
            )
        })
        .await?;

        Ok(())
    }

    fn toggle_editing_hosts(
        &mut self,
        _event: &ClickEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.editing_hosts = !self.editing_hosts;
        cx.notify();
    }

    fn remove_host(&mut self, index: usize, _window: &mut Window, cx: &mut Context<Self>) {
        if index < self.saved_hosts.len() {
            self.saved_hosts.remove(index);
            self.persist_hosts();
            cx.notify();
        }
    }

    fn render_header(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .items_center()
            .gap_4()
            .child(
                h_flex()
                    .justify_center()
                    .gap_4()
                    .child(Vector::square(VectorName::ZedLogo, rems_from_px(45.)))
                    .child(
                        v_flex()
                            .child(Headline::new("Welcome to Zed"))
                            .child(
                                Label::new("The editor for what's next")
                                    .size(LabelSize::Small)
                                    .color(Color::Muted)
                                    .italic(),
                            ),
                    ),
            )
            .child(
                Label::new("Connect to a remote host to start editing").color(Color::Muted),
            )
    }

    fn render_host_entry(
        &self,
        index: usize,
        host: &SavedHost,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let colors = cx.theme().colors();
        let display_name = host.display_name();
        let address_line = host.address_line();
        let status_color = host.status_color();
        let status_label = host.status_label();
        let is_connectable = matches!(
            host.status,
            ConnectionStatus::Disconnected | ConnectionStatus::Error(_)
        );

        let hover_bg = colors.ghost_element_hover;
        let focus_border = colors.border_focused;

        div()
            .id(SharedString::from(format!("host-{index}")))
            .tab_index(index as isize)
            .w_full()
            .px_4()
            .py_3()
            .flex()
            .items_center()
            .justify_between()
            .cursor_pointer()
            .rounded_md()
            .border_2()
            .border_color(gpui::transparent_black())
            .hover(move |style| style.bg(hover_bg))
            .focus(move |style| style.border_color(focus_border))
            .when(is_connectable, |this| {
                this.on_click(cx.listener(move |this, _event, window, cx| {
                    this.connect_host(index, window, cx);
                }))
            })
            .child(
                h_flex()
                    .gap_3()
                    .items_center()
                    .child(
                        div().flex_shrink_0().child(
                            Icon::new(IconName::Server)
                                .size(IconSize::Small)
                                .color(Color::Muted),
                        ),
                    )
                    .child(
                        v_flex()
                            .child(Label::new(display_name).color(Color::Default))
                            .child(
                                Label::new(address_line)
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            ),
                    ),
            )
            .child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .child(Indicator::dot().color(status_color))
                    .child(
                        Label::new(status_label)
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    )
                    .when(self.editing_hosts, |this| {
                        this.child(
                            IconButton::new(
                                SharedString::from(format!("remove-host-{index}")),
                                IconName::Trash,
                            )
                            .size(ui::ButtonSize::Compact)
                            .icon_size(IconSize::Small)
                            .icon_color(Color::Muted)
                            .style(ButtonStyle::Transparent)
                            .on_click(cx.listener(move |this, _event, window, cx| {
                                this.remove_host(index, window, cx);
                            })),
                        )
                    }),
            )
    }

    fn render_hosts_list(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = cx.theme().colors();
        let host_count = self.saved_hosts.len();

        let editing = self.editing_hosts;
        let has_hosts = !self.saved_hosts.is_empty();

        let mut list = v_flex().w_96().gap_2().child(
            h_flex()
                .justify_between()
                .items_center()
                .child(
                    Label::new("SAVED HOSTS")
                        .size(LabelSize::XSmall)
                        .color(Color::Muted),
                )
                .when(has_hosts, |this| {
                    this.child(
                        IconButton::new("edit-hosts-btn", IconName::Pencil)
                            .size(ui::ButtonSize::Compact)
                            .icon_size(IconSize::XSmall)
                            .icon_color(if editing { Color::Accent } else { Color::Muted })
                            .style(ButtonStyle::Transparent)
                            .on_click(cx.listener(Self::toggle_editing_hosts)),
                    )
                }),
        );

        if self.saved_hosts.is_empty() {
            list = list.child(
                div()
                    .rounded_lg()
                    .border_1()
                    .border_color(colors.border)
                    .bg(colors.surface_background)
                    .p_4()
                    .child(Label::new("No saved hosts yet").color(Color::Muted)),
            );
        } else {
            let border = colors.border;
            let surface_bg = colors.surface_background;

            let mut entries = div()
                .tab_group()
                .rounded_lg()
                .border_1()
                .border_color(border)
                .bg(surface_bg)
                .overflow_hidden();

            for index in 0..self.saved_hosts.len() {
                if index > 0 {
                    entries = entries.child(div().mx_4().h_px().bg(border));
                }
                entries = entries.child(self.render_host_entry(index, &self.saved_hosts[index], cx));
            }

            list = list.child(entries);
        }

        list.child(
            ui::Button::new("add-host-btn", "Connect SSH Server")
                .tab_index(host_count as isize)
                .start_icon(Icon::new(IconName::Plus))
                .full_width()
                .style(ButtonStyle::Filled)
                .on_click(cx.listener(Self::switch_to_add_host)),
        )
    }

    fn render_add_host_form(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = cx.theme().colors();
        let name_focus = self.name_editor.focus_handle(cx).tab_index(0).tab_stop(true);
        let host_focus = self.host_editor.focus_handle(cx).tab_index(1).tab_stop(true);
        let username_focus = self.username_editor.focus_handle(cx).tab_index(2).tab_stop(true);
        let port_focus = self.port_editor.focus_handle(cx).tab_index(3).tab_stop(true);
        let password_focus = self.password_editor.focus_handle(cx).tab_index(4).tab_stop(true);

        v_flex()
            .id("add-host-form")
            .w_96()
            .gap_4()
            .child(
                Label::new("NEW CONNECTION")
                    .size(LabelSize::XSmall)
                    .color(Color::Muted),
            )
            .child(
                div()
                    .tab_group()
                    .rounded_lg()
                    .border_1()
                    .border_color(colors.border)
                    .bg(colors.surface_background)
                    .p_4()
                    .flex()
                    .flex_col()
                    .gap_3()
                    // Name field (optional)
                    .child(
                        v_flex()
                            .gap_1()
                            .child(
                                Label::new("Name")
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            )
                            .child(
                                div()
                                    .id("name-field")
                                    .track_focus(&name_focus)
                                    .rounded_md()
                                    .border_1()
                                    .border_color(colors.border)
                                    .bg(colors.editor_background)
                                    .px_2()
                                    .py_1()
                                    .child(self.name_editor.clone()),
                            ),
                    )
                    .child(
                        v_flex()
                            .gap_1()
                            .child(
                                Label::new("Host")
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            )
                            .child(
                                div()
                                    .id("host-field")
                                    .track_focus(&host_focus)
                                    .rounded_md()
                                    .border_1()
                                    .border_color(colors.border)
                                    .bg(colors.editor_background)
                                    .px_2()
                                    .py_1()
                                    .child(self.host_editor.clone()),
                            ),
                    )
                    .child(
                        v_flex()
                            .gap_1()
                            .child(
                                Label::new("Username")
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            )
                            .child(
                                div()
                                    .id("username-field")
                                    .track_focus(&username_focus)
                                    .rounded_md()
                                    .border_1()
                                    .border_color(colors.border)
                                    .bg(colors.editor_background)
                                    .px_2()
                                    .py_1()
                                    .child(self.username_editor.clone()),
                            ),
                    )
                    .child(
                        v_flex()
                            .gap_1()
                            .child(
                                Label::new("Port")
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            )
                            .child(
                                div()
                                    .id("port-field")
                                    .track_focus(&port_focus)
                                    .rounded_md()
                                    .border_1()
                                    .border_color(colors.border)
                                    .bg(colors.editor_background)
                                    .px_2()
                                    .py_1()
                                    .child(self.port_editor.clone()),
                            ),
                    )
                    .child(
                        v_flex()
                            .gap_1()
                            .child(
                                Label::new("Password")
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            )
                            .child(
                                div()
                                    .id("password-field")
                                    .track_focus(&password_focus)
                                    .rounded_md()
                                    .border_1()
                                    .border_color(colors.border)
                                    .bg(colors.editor_background)
                                    .px_2()
                                    .py_1()
                                    .child(self.password_editor.clone()),
                            ),
                    ),
            )
            .child(
                h_flex()
                    .gap_2()
                    .justify_end()
                    .child(
                        ui::Button::new("cancel-btn", "Cancel")
                            .tab_index(5_isize)
                            .style(ButtonStyle::Subtle)
                            .on_click(cx.listener(Self::cancel_add_host)),
                    )
                    .child(
                        ui::Button::new("connect-btn", "Connect")
                            .tab_index(6_isize)
                            .style(ButtonStyle::Filled)
                            .on_click(cx.listener(Self::confirm_add_host)),
                    ),
            )
    }
}

impl Render for ConnectionLanding {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = cx.theme().colors();

        let mut content = div()
            .id("connection-landing")
            .track_focus(&self.focus_handle)
            .on_action(|_: &Tab, window, cx| {
                window.focus_next(cx);
            })
            .on_action(|_: &Backtab, window, cx| {
                window.focus_prev(cx);
            })
            .size_full()
            .bg(colors.background)
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_6()
            .child(self.render_header(cx));

        match &self.mode {
            LandingMode::Default => {
                content = content.child(self.render_hosts_list(cx));
            }
            LandingMode::AddHost => {
                content = content.child(self.render_add_host_form(cx));
            }
        }

        content
    }
}

impl gpui::Focusable for ConnectionLanding {
    fn focus_handle(&self, _cx: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

/// A simple password prompt that opens in its own window.
/// Used by the SSH delegate when key auth fails and no saved password exists.
struct PasswordPromptView {
    prompt_text: SharedString,
    editor: Entity<Editor>,
    focus_handle: gpui::FocusHandle,
    tx: Option<futures::channel::oneshot::Sender<EncryptedPassword>>,
}

impl PasswordPromptView {
    fn open(
        prompt: String,
        tx: futures::channel::oneshot::Sender<EncryptedPassword>,
        cx: &mut App,
    ) -> anyhow::Result<()> {
        cx.open_window(WindowOptions::default(), |window, cx| {
            let view = cx.new(|cx| {
                let editor = cx.new(|cx| {
                    let mut editor = Editor::single_line(window, cx);
                    editor.set_placeholder_text("enter password", window, cx);
                    editor.set_masked(true, cx);
                    editor
                });

                let focus_handle = cx.focus_handle();
                editor.focus_handle(cx).focus(window, cx);

                Self {
                    prompt_text: SharedString::from(prompt),
                    editor,
                    focus_handle,
                    tx: Some(tx),
                }
            });
            view
        })?;
        Ok(())
    }

    fn submit(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(tx) = self.tx.take() {
            let password_text = self.editor.read(cx).text(cx);
            match EncryptedPassword::try_from(password_text.as_ref()) {
                Ok(encrypted) => {
                    tx.send(encrypted).ok();
                }
                Err(error) => {
                    log::error!("[zed-ios] Failed to encrypt password: {error}");
                }
            }
        }
        window.remove_window();
    }

    fn cancel(&mut self, window: &mut Window, _cx: &mut Context<Self>) {
        self.tx.take();
        window.remove_window();
    }
}

impl Render for PasswordPromptView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = cx.theme().colors();
        let _editor_focus = self.editor.focus_handle(cx);

        div()
            .id("password-prompt")
            .track_focus(&self.focus_handle)
            .size_full()
            .bg(colors.background)
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_4()
            .child(
                Label::new(self.prompt_text.clone()).color(Color::Default),
            )
            .child(
                div()
                    .w_80()
                    .rounded_md()
                    .border_1()
                    .border_color(colors.border)
                    .bg(colors.editor_background)
                    .px_2()
                    .py_1()
                    .child(self.editor.clone()),
            )
            .child(
                h_flex()
                    .gap_2()
                    .child(
                        ui::Button::new("pw-cancel", "Cancel")
                            .style(ButtonStyle::Subtle)
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.cancel(window, cx);
                            })),
                    )
                    .child(
                        ui::Button::new("pw-submit", "Submit")
                            .style(ButtonStyle::Filled)
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.submit(window, cx);
                            })),
                    ),
            )
    }
}

impl gpui::Focusable for PasswordPromptView {
    fn focus_handle(&self, _cx: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

struct IosRemoteClientDelegate;

impl remote::RemoteClientDelegate for IosRemoteClientDelegate {
    fn ask_password(
        &self,
        prompt: String,
        tx: futures::channel::oneshot::Sender<askpass::EncryptedPassword>,
        cx: &mut AsyncApp,
    ) {
        log::info!("[zed-ios] Password prompt requested: {prompt}");
        let result = cx.update(|cx| {
            PasswordPromptView::open(prompt, tx, cx)
        });
        if let Err(error) = result {
            log::error!("[zed-ios] Failed to show password prompt: {error:#}");
        }
    }

    fn get_download_url(
        &self,
        _platform: remote::RemotePlatform,
        _release_channel: release_channel::ReleaseChannel,
        _version: Option<semver::Version>,
        _cx: &mut AsyncApp,
    ) -> Task<anyhow::Result<Option<String>>> {
        Task::ready(Ok(None))
    }

    fn download_server_binary_locally(
        &self,
        _platform: remote::RemotePlatform,
        _release_channel: release_channel::ReleaseChannel,
        _version: Option<semver::Version>,
        _cx: &mut AsyncApp,
    ) -> Task<anyhow::Result<PathBuf>> {
        Task::ready(Err(anyhow::anyhow!(
            "server binary download not supported on iOS"
        )))
    }

    fn set_status(&self, status: Option<&str>, _cx: &mut AsyncApp) {
        if let Some(status) = status {
            log::info!("[zed-ios] SSH status: {status}");
        }
    }
}
