use editor::{Editor, actions::{Backtab, Tab}};
use gpui::{
    App, ClickEvent, Context, Entity, Focusable as _, IntoElement, Render, SharedString, Window,
    WindowOptions, div, prelude::*,
};
use theme::ActiveTheme;
use ui::{
    ButtonCommon, ButtonStyle, Clickable, Color, FixedWidth, Headline, Icon, IconName, Indicator,
    Label, LabelCommon, LabelSize, Vector, VectorName, h_flex, rems_from_px, v_flex,
};

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
    status: ConnectionStatus,
}

impl SavedHost {
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
    saved_hosts: Vec<SavedHost>,
    name_editor: Entity<Editor>,
    host_editor: Entity<Editor>,
    username_editor: Entity<Editor>,
    port_editor: Entity<Editor>,
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

        Self {
            focus_handle: cx.focus_handle(),
            mode: LandingMode::Default,
            saved_hosts: Self::dummy_hosts(),
            name_editor,
            host_editor,
            username_editor,
            port_editor,
        }
    }

    /// Open the connection landing screen in a new window.
    pub fn open(cx: &mut App) -> anyhow::Result<()> {
        cx.open_window(WindowOptions::default(), |window, cx| {
            cx.new(|cx| Self::new(window, cx))
        })?;
        Ok(())
    }

    fn dummy_hosts() -> Vec<SavedHost> {
        vec![
            SavedHost {
                nickname: Some("Dev Server".into()),
                host: "dev.example.com".into(),
                username: "dcow".into(),
                port: 22,
                status: ConnectionStatus::Connected { project_count: 2 },
            },
            SavedHost {
                nickname: None,
                host: "192.168.1.42".into(),
                username: "root".into(),
                port: 2222,
                status: ConnectionStatus::Disconnected,
            },
            SavedHost {
                nickname: Some("CI Box".into()),
                host: "ci.internal".into(),
                username: "builder".into(),
                port: 22,
                status: ConnectionStatus::Error("Connection refused".into()),
            },
        ]
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
        self.name_editor.focus_handle(cx).focus(window, cx);
        cx.notify();
    }

    fn cancel_add_host(
        &mut self,
        _event: &ClickEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.mode = LandingMode::Default;
        cx.notify();
    }

    fn confirm_add_host(
        &mut self,
        _event: &ClickEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let name = self.name_editor.read(cx).text(cx);
        let host = self.host_editor.read(cx).text(cx);
        let username = self.username_editor.read(cx).text(cx);
        let port_text = self.port_editor.read(cx).text(cx);
        let port: u16 = port_text.parse().unwrap_or(22);

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
            status: ConnectionStatus::Disconnected,
        });

        self.mode = LandingMode::Default;
        cx.notify();
    }

    fn connect_host(&mut self, index: usize, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(host) = self.saved_hosts.get_mut(index) {
            host.status = ConnectionStatus::Connecting;
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

        div()
            .id(SharedString::from(format!("host-{index}")))
            .w_full()
            .px_4()
            .py_3()
            .flex()
            .items_center()
            .justify_between()
            .cursor_pointer()
            .rounded_md()
            .hover(move |style| style.bg(hover_bg))
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
                                .size(ui::IconSize::Small)
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
                    ),
            )
    }

    fn render_hosts_list(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = cx.theme().colors();

        let mut list = v_flex().w_96().gap_2().child(
            Label::new("SAVED HOSTS")
                .size(LabelSize::XSmall)
                .color(Color::Muted),
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

        list
    }

    fn render_add_host_button(&self, cx: &mut Context<Self>) -> impl IntoElement {
        ui::Button::new("add-host-btn", "Connect SSH Server")
            .start_icon(Icon::new(IconName::Plus))
            .full_width()
            .style(ButtonStyle::Filled)
            .on_click(cx.listener(Self::switch_to_add_host))
    }

    fn render_add_host_form(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = cx.theme().colors();
        let name_focus = self.name_editor.focus_handle(cx).tab_index(0).tab_stop(true);
        let host_focus = self.host_editor.focus_handle(cx).tab_index(1).tab_stop(true);
        let username_focus = self.username_editor.focus_handle(cx).tab_index(2).tab_stop(true);
        let port_focus = self.port_editor.focus_handle(cx).tab_index(3).tab_stop(true);

        v_flex()
            .id("add-host-form")
            .key_context("FormFields")
            .w_96()
            .gap_4()
            .on_action(|_: &Tab, window, cx| {
                window.focus_next(cx);
            })
            .on_action(|_: &Backtab, window, cx| {
                window.focus_prev(cx);
            })
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
                    ),
            )
            .child(
                h_flex()
                    .gap_2()
                    .justify_end()
                    .child(
                        ui::Button::new("cancel-btn", "Cancel")
                            .tab_index(4_isize)
                            .style(ButtonStyle::Subtle)
                            .on_click(cx.listener(Self::cancel_add_host)),
                    )
                    .child(
                        ui::Button::new("connect-btn", "Connect")
                            .tab_index(5_isize)
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
                content = content
                    .child(self.render_hosts_list(cx))
                    .child(div().w_96().child(self.render_add_host_button(cx)));
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
