use std::path::PathBuf;

use gpui::{
    actions, div, prelude::*, px, App, Context, DismissEvent, Entity, EventEmitter,
    FocusHandle, Focusable, SharedString, Task, Window,
};
use ui::{prelude::*, Banner, Button, ButtonStyle, Icon, IconName, IconSize, Label, Severity, Tooltip};
use workspace::ModalView;

use database_core::{
    ConnectionConfig, IntrospectionLevel, SshAuthMethod, SshTunnelConfig, SslConfig, SslMode,
    create_connection,
};

actions!(
    connection_dialog,
    [
        /// Opens the connection dialog.
        ShowConnectionDialog,
        /// Tests the current connection settings.
        TestConnection,
    ]
);

#[allow(dead_code)]
pub enum ConnectionDialogEvent {
    ConnectionCreated(ConnectionConfig),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DriverTab {
    Sqlite,
    PostgreSql,
    MySql,
}

impl DriverTab {
    fn label(&self) -> &'static str {
        match self {
            DriverTab::Sqlite => "SQLite",
            DriverTab::PostgreSql => "PostgreSQL",
            DriverTab::MySql => "MySQL",
        }
    }

    fn all() -> &'static [DriverTab] {
        &[DriverTab::Sqlite, DriverTab::PostgreSql, DriverTab::MySql]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SshAuthSelection {
    Password,
    PrivateKey,
    Agent,
}

pub const CONNECTION_COLORS: &[u32] = &[
    0x3B82F6, // blue
    0x10B981, // green
    0xF59E0B, // amber
    0xEF4444, // red
    0x8B5CF6, // purple
    0xEC4899, // pink
    0x06B6D4, // cyan
    0xF97316, // orange
];

pub struct ConnectionDialog {
    focus_handle: FocusHandle,
    active_tab: DriverTab,
    name_field: Entity<editor::Editor>,
    host_field: Entity<editor::Editor>,
    port_field: Entity<editor::Editor>,
    database_field: Entity<editor::Editor>,
    user_field: Entity<editor::Editor>,
    password_field: Entity<editor::Editor>,
    path_field: Entity<editor::Editor>,
    ssl_mode: SslMode,
    selected_color: usize,
    read_only: bool,
    error: Option<String>,
    test_result: Option<Result<String, String>>,
    test_task: Task<()>,

    ssh_enabled: bool,
    ssh_host_field: Entity<editor::Editor>,
    ssh_port_field: Entity<editor::Editor>,
    ssh_user_field: Entity<editor::Editor>,
    ssh_auth_method: SshAuthSelection,
    ssh_key_path_field: Entity<editor::Editor>,
    ssh_passphrase_field: Entity<editor::Editor>,

    ssl_ca_cert_field: Entity<editor::Editor>,
    ssl_client_cert_field: Entity<editor::Editor>,
    ssl_client_key_field: Entity<editor::Editor>,

    introspection_level: IntrospectionLevel,
}

impl EventEmitter<DismissEvent> for ConnectionDialog {}
impl EventEmitter<ConnectionDialogEvent> for ConnectionDialog {}

impl ModalView for ConnectionDialog {
    fn fade_out_background(&self) -> bool {
        true
    }
}

impl Focusable for ConnectionDialog {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl ConnectionDialog {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();

        let name_field = cx.new(|cx| {
            let mut editor = editor::Editor::single_line(window, cx);
            editor.set_placeholder_text("Connection name", window, cx);
            editor
        });
        let host_field = cx.new(|cx| {
            let mut editor = editor::Editor::single_line(window, cx);
            editor.set_placeholder_text("localhost", window, cx);
            editor
        });
        let port_field = cx.new(|cx| {
            let mut editor = editor::Editor::single_line(window, cx);
            editor.set_placeholder_text("5432", window, cx);
            editor
        });
        let database_field = cx.new(|cx| {
            let mut editor = editor::Editor::single_line(window, cx);
            editor.set_placeholder_text("postgres", window, cx);
            editor
        });
        let user_field = cx.new(|cx| {
            let mut editor = editor::Editor::single_line(window, cx);
            editor.set_placeholder_text("postgres", window, cx);
            editor
        });
        let password_field = cx.new(|cx| {
            editor::Editor::single_line(window, cx)
        });
        let path_field = cx.new(|cx| {
            let mut editor = editor::Editor::single_line(window, cx);
            editor.set_placeholder_text("/path/to/database.db", window, cx);
            editor
        });

        let ssh_host_field = cx.new(|cx| {
            let mut editor = editor::Editor::single_line(window, cx);
            editor.set_placeholder_text("ssh.example.com", window, cx);
            editor
        });
        let ssh_port_field = cx.new(|cx| {
            let mut editor = editor::Editor::single_line(window, cx);
            editor.set_placeholder_text("22", window, cx);
            editor
        });
        let ssh_user_field = cx.new(|cx| {
            let mut editor = editor::Editor::single_line(window, cx);
            editor.set_placeholder_text("ssh-user", window, cx);
            editor
        });
        let ssh_key_path_field = cx.new(|cx| {
            let mut editor = editor::Editor::single_line(window, cx);
            editor.set_placeholder_text("~/.ssh/id_rsa", window, cx);
            editor
        });
        let ssh_passphrase_field = cx.new(|cx| {
            editor::Editor::single_line(window, cx)
        });

        let ssl_ca_cert_field = cx.new(|cx| {
            let mut editor = editor::Editor::single_line(window, cx);
            editor.set_placeholder_text("/path/to/ca-cert.pem", window, cx);
            editor
        });
        let ssl_client_cert_field = cx.new(|cx| {
            let mut editor = editor::Editor::single_line(window, cx);
            editor.set_placeholder_text("/path/to/client-cert.pem", window, cx);
            editor
        });
        let ssl_client_key_field = cx.new(|cx| {
            let mut editor = editor::Editor::single_line(window, cx);
            editor.set_placeholder_text("/path/to/client-key.pem", window, cx);
            editor
        });

        Self {
            focus_handle,
            active_tab: DriverTab::PostgreSql,
            name_field,
            host_field,
            port_field,
            database_field,
            user_field,
            password_field,
            path_field,
            ssl_mode: SslMode::Disable,
            selected_color: 0,
            read_only: false,
            error: None,
            test_result: None,
            test_task: Task::ready(()),

            ssh_enabled: false,
            ssh_host_field,
            ssh_port_field,
            ssh_user_field,
            ssh_auth_method: SshAuthSelection::Agent,
            ssh_key_path_field,
            ssh_passphrase_field,

            ssl_ca_cert_field,
            ssl_client_cert_field,
            ssl_client_key_field,

            introspection_level: IntrospectionLevel::default(),
        }
    }

    fn dismiss(&mut self, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }

    fn set_tab(&mut self, tab: DriverTab, window: &mut Window, cx: &mut Context<Self>) {
        self.active_tab = tab;
        self.error = None;
        self.test_result = None;

        self.port_field.update(cx, |editor, cx| {
            let placeholder = match tab {
                DriverTab::PostgreSql => "5432",
                DriverTab::MySql => "3306",
                DriverTab::Sqlite => "",
            };
            editor.set_placeholder_text(placeholder, window, cx);
        });

        self.database_field.update(cx, |editor, cx| {
            let placeholder = match tab {
                DriverTab::PostgreSql => "postgres",
                DriverTab::MySql => "mysql",
                DriverTab::Sqlite => "",
            };
            editor.set_placeholder_text(placeholder, window, cx);
        });

        cx.notify();
    }

    fn cycle_ssl_mode(&mut self, cx: &mut Context<Self>) {
        self.ssl_mode = match self.ssl_mode {
            SslMode::Disable => SslMode::Prefer,
            SslMode::Prefer => SslMode::Require,
            SslMode::Require => SslMode::VerifyCa,
            SslMode::VerifyCa => SslMode::VerifyFull,
            SslMode::VerifyFull => SslMode::Disable,
        };
        cx.notify();
    }

    fn build_config(&self, cx: &App) -> Result<ConnectionConfig, String> {
        let name = self.name_field.read(cx).text(cx).trim().to_string();

        let ssh_tunnel = if self.ssh_enabled {
            let ssh_host = self.ssh_host_field.read(cx).text(cx).trim().to_string();
            let ssh_port_str = self.ssh_port_field.read(cx).text(cx).trim().to_string();
            let ssh_user = self.ssh_user_field.read(cx).text(cx).trim().to_string();

            if ssh_host.is_empty() {
                return Err("SSH host is required".to_string());
            }
            if ssh_user.is_empty() {
                return Err("SSH username is required".to_string());
            }

            let ssh_port = if ssh_port_str.is_empty() {
                22
            } else {
                ssh_port_str
                    .parse::<u16>()
                    .map_err(|_| "Invalid SSH port".to_string())?
            };

            let auth_method = match self.ssh_auth_method {
                SshAuthSelection::Password => SshAuthMethod::Password,
                SshAuthSelection::PrivateKey => {
                    let key_path = self.ssh_key_path_field.read(cx).text(cx).trim().to_string();
                    if key_path.is_empty() {
                        return Err("SSH key path is required".to_string());
                    }
                    let passphrase_text = self.ssh_passphrase_field.read(cx).text(cx).to_string();
                    SshAuthMethod::PrivateKey {
                        key_path: PathBuf::from(key_path),
                        passphrase: if passphrase_text.is_empty() {
                            None
                        } else {
                            Some(passphrase_text)
                        },
                    }
                }
                SshAuthSelection::Agent => SshAuthMethod::Agent,
            };

            Some(SshTunnelConfig {
                host: ssh_host,
                port: ssh_port,
                username: ssh_user,
                auth_method,
            })
        } else {
            None
        };

        let ssl_config =
            if matches!(self.ssl_mode, SslMode::VerifyCa | SslMode::VerifyFull) {
                let ca_cert = self.ssl_ca_cert_field.read(cx).text(cx).trim().to_string();
                let client_cert = self
                    .ssl_client_cert_field
                    .read(cx)
                    .text(cx)
                    .trim()
                    .to_string();
                let client_key = self
                    .ssl_client_key_field
                    .read(cx)
                    .text(cx)
                    .trim()
                    .to_string();
                Some(SslConfig {
                    ca_cert_path: if ca_cert.is_empty() {
                        None
                    } else {
                        Some(PathBuf::from(ca_cert))
                    },
                    client_cert_path: if client_cert.is_empty() {
                        None
                    } else {
                        Some(PathBuf::from(client_cert))
                    },
                    client_key_path: if client_key.is_empty() {
                        None
                    } else {
                        Some(PathBuf::from(client_key))
                    },
                })
            } else {
                None
            };

        let mut config = match self.active_tab {
            DriverTab::Sqlite => {
                let path_str = self.path_field.read(cx).text(cx).trim().to_string();
                if path_str.is_empty() {
                    return Err("File path is required".to_string());
                }
                let path = std::path::PathBuf::from(&path_str);
                let name = if name.is_empty() {
                    path.file_name()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_else(|| path_str.clone())
                } else {
                    name
                };
                ConnectionConfig::sqlite(name, path)
            }
            DriverTab::PostgreSql => {
                let host = self.host_field.read(cx).text(cx).trim().to_string();
                let host = if host.is_empty() {
                    "localhost".to_string()
                } else {
                    host
                };
                let port_str = self.port_field.read(cx).text(cx).trim().to_string();
                let port: u16 = if port_str.is_empty() {
                    5432
                } else {
                    port_str
                        .parse()
                        .map_err(|_| "Invalid port number".to_string())?
                };
                let database = self.database_field.read(cx).text(cx).trim().to_string();
                let database = if database.is_empty() {
                    "postgres".to_string()
                } else {
                    database
                };
                let user = self.user_field.read(cx).text(cx).trim().to_string();
                let user = if user.is_empty() {
                    "postgres".to_string()
                } else {
                    user
                };
                let password = self.password_field.read(cx).text(cx).to_string();
                let name = if name.is_empty() {
                    format!("{} @ {}", database, host)
                } else {
                    name
                };
                ConnectionConfig::postgres(
                    name,
                    host,
                    port,
                    database,
                    user,
                    password,
                    self.ssl_mode,
                )
            }
            DriverTab::MySql => {
                let host = self.host_field.read(cx).text(cx).trim().to_string();
                let host = if host.is_empty() {
                    "localhost".to_string()
                } else {
                    host
                };
                let port_str = self.port_field.read(cx).text(cx).trim().to_string();
                let port: u16 = if port_str.is_empty() {
                    3306
                } else {
                    port_str
                        .parse()
                        .map_err(|_| "Invalid port number".to_string())?
                };
                let database = self.database_field.read(cx).text(cx).trim().to_string();
                let database = if database.is_empty() {
                    "mysql".to_string()
                } else {
                    database
                };
                let user = self.user_field.read(cx).text(cx).trim().to_string();
                let user = if user.is_empty() {
                    "root".to_string()
                } else {
                    user
                };
                let password = self.password_field.read(cx).text(cx).to_string();
                let name = if name.is_empty() {
                    format!("{} @ {}", database, host)
                } else {
                    name
                };
                ConnectionConfig::mysql(
                    name,
                    host,
                    port,
                    database,
                    user,
                    password,
                    self.ssl_mode,
                )
            }
        };

        config.read_only = self.read_only;
        config.color_index = self.selected_color;
        config.ssh_tunnel = ssh_tunnel;
        config.ssl_config = ssl_config;
        config.introspection_level = self.introspection_level;

        Ok(config)
    }

    fn test_connection(&mut self, cx: &mut Context<Self>) {
        let config = match self.build_config(cx) {
            Ok(config) => config,
            Err(error) => {
                self.error = Some(error);
                cx.notify();
                return;
            }
        };

        self.error = None;
        self.test_result = None;
        cx.notify();

        self.test_task = cx.spawn(async move |this, cx| {
            let result = cx
                .background_spawn(async move {
                    let connection = create_connection(&config)?;
                    let schema = connection.fetch_schema()?;
                    Ok::<_, anyhow::Error>(schema.tables.len())
                })
                .await;

            this.update(cx, |this, cx| {
                match result {
                    Ok(table_count) => {
                        this.test_result = Some(Ok(format!(
                            "Connection successful ({} tables)",
                            table_count
                        )));
                    }
                    Err(error) => {
                        this.test_result = Some(Err(format!("{:#}", error)));
                    }
                }
                cx.notify();
            })
            .ok();
        });
    }

    fn save_connection(&mut self, cx: &mut Context<Self>) {
        let config = match self.build_config(cx) {
            Ok(config) => config,
            Err(error) => {
                self.error = Some(error);
                cx.notify();
                return;
            }
        };

        cx.emit(ConnectionDialogEvent::ConnectionCreated(config));
        cx.emit(DismissEvent);
    }

    fn render_field(
        label: &'static str,
        editor: &Entity<editor::Editor>,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        v_flex()
            .gap(px(2.0))
            .child(
                Label::new(label)
                    .size(LabelSize::Small)
                    .color(ui::Color::Muted),
            )
            .child(
                div()
                    .h(px(28.0))
                    .w_full()
                    .px_2()
                    .border_1()
                    .border_color(cx.theme().colors().border)
                    .rounded_md()
                    .bg(cx.theme().colors().editor_background)
                    .child(editor.clone()),
            )
    }

    fn render_tabs(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let mut tabs = h_flex().gap_1().w_full().pb_2();

        for tab in DriverTab::all() {
            let is_active = self.active_tab == *tab;
            let tab_value = *tab;

            tabs = tabs.child(
                Button::new(
                    SharedString::from(format!("tab-{}", tab.label())),
                    tab.label(),
                )
                .style(if is_active {
                    ButtonStyle::Filled
                } else {
                    ButtonStyle::Subtle
                })
                .label_size(LabelSize::Small)
                .on_click(cx.listener(move |this, _, window, cx| {
                    this.set_tab(tab_value, window, cx);
                })),
            );
        }

        tabs
    }

    fn render_color_picker(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let mut picker = h_flex().gap_1().items_center().child(
            Label::new("Color")
                .size(LabelSize::Small)
                .color(ui::Color::Muted),
        );

        for (index, &color) in CONNECTION_COLORS.iter().enumerate() {
            let is_selected = self.selected_color == index;

            picker = picker.child(
                div()
                    .id(SharedString::from(format!("color-{}", index)))
                    .w(px(18.0))
                    .h(px(18.0))
                    .rounded_full()
                    .cursor_pointer()
                    .bg(gpui::Rgba {
                        r: ((color >> 16) & 0xFF) as f32 / 255.0,
                        g: ((color >> 8) & 0xFF) as f32 / 255.0,
                        b: (color & 0xFF) as f32 / 255.0,
                        a: 1.0,
                    })
                    .when(is_selected, |this| this.border_2().border_color(gpui::white()))
                    .on_click(cx.listener(move |this, _, _window, cx| {
                        this.selected_color = index;
                        cx.notify();
                    })),
            );
        }

        picker
    }

    fn render_read_only_toggle(&self, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .gap_2()
            .items_center()
            .child(
                div()
                    .id("read-only-toggle")
                    .w(px(16.0))
                    .h(px(16.0))
                    .rounded(px(3.0))
                    .border_1()
                    .border_color(cx.theme().colors().border)
                    .cursor_pointer()
                    .when(self.read_only, |this| {
                        this.bg(cx.theme().colors().icon_accent)
                            .child(
                                div()
                                    .w_full()
                                    .h_full()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child(
                                        Icon::new(IconName::Check)
                                            .size(IconSize::XSmall)
                                            .color(ui::Color::Default),
                                    ),
                            )
                    })
                    .on_click(cx.listener(|this, _, _window, cx| {
                        this.read_only = !this.read_only;
                        cx.notify();
                    })),
            )
            .child(
                Label::new("Read-only")
                    .size(LabelSize::Small)
                    .color(ui::Color::Muted),
            )
    }

    fn render_ssl_selector(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let ssl_label = match self.ssl_mode {
            SslMode::Disable => "Disable",
            SslMode::Prefer => "Prefer",
            SslMode::Require => "Require",
            SslMode::VerifyCa => "Verify CA",
            SslMode::VerifyFull => "Verify Full",
        };

        v_flex()
            .gap_2()
            .child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .child(
                        Label::new("SSL Mode")
                            .size(LabelSize::Small)
                            .color(ui::Color::Muted),
                    )
                    .child(
                        Button::new("ssl-mode-btn", ssl_label)
                            .style(ButtonStyle::Subtle)
                            .label_size(LabelSize::Small)
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.cycle_ssl_mode(cx);
                            })),
                    ),
            )
            .when(
                matches!(self.ssl_mode, SslMode::VerifyCa | SslMode::VerifyFull),
                |element| element.child(self.render_ssl_cert_fields(cx)),
            )
    }

    fn render_ssl_cert_fields(&self, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_2()
            .pl_4()
            .child(Self::render_field("CA Certificate", &self.ssl_ca_cert_field, cx))
            .child(Self::render_field(
                "Client Certificate",
                &self.ssl_client_cert_field,
                cx,
            ))
            .child(Self::render_field(
                "Client Key",
                &self.ssl_client_key_field,
                cx,
            ))
    }

    fn render_ssh_section(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let toggle_label = if self.ssh_enabled {
            "SSH Tunnel (Enabled)"
        } else {
            "SSH Tunnel (Disabled)"
        };

        v_flex()
            .gap_2()
            .child(
                Button::new("ssh_toggle", toggle_label)
                    .style(ButtonStyle::Subtle)
                    .label_size(LabelSize::Small)
                    .on_click(cx.listener(|this, _event, _window, cx| {
                        this.ssh_enabled = !this.ssh_enabled;
                        cx.notify();
                    })),
            )
            .when(self.ssh_enabled, |element| {
                element.child(self.render_ssh_fields(cx))
            })
    }

    fn render_ssh_fields(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let mut fields = v_flex()
            .gap_2()
            .pl_4()
            .child(
                h_flex()
                    .gap_2()
                    .child(
                        div()
                            .flex_grow()
                            .child(Self::render_field("SSH Host", &self.ssh_host_field, cx)),
                    )
                    .child(
                        div()
                            .w(px(80.0))
                            .child(Self::render_field("SSH Port", &self.ssh_port_field, cx)),
                    ),
            )
            .child(Self::render_field("SSH Username", &self.ssh_user_field, cx))
            .child(self.render_ssh_auth_selector(cx));

        if self.ssh_auth_method == SshAuthSelection::PrivateKey {
            fields = fields
                .child(Self::render_field(
                    "Key Path",
                    &self.ssh_key_path_field,
                    cx,
                ))
                .child(Self::render_field(
                    "Passphrase",
                    &self.ssh_passphrase_field,
                    cx,
                ));
        }

        fields
    }

    fn render_ssh_auth_selector(&self, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap(px(2.0))
            .child(
                Label::new("Auth Method")
                    .size(LabelSize::Small)
                    .color(ui::Color::Muted),
            )
            .child(
                h_flex()
                    .gap_1()
                    .child(
                        Button::new("ssh-auth-password", "Password")
                            .style(if self.ssh_auth_method == SshAuthSelection::Password {
                                ButtonStyle::Filled
                            } else {
                                ButtonStyle::Subtle
                            })
                            .label_size(LabelSize::Small)
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.ssh_auth_method = SshAuthSelection::Password;
                                cx.notify();
                            })),
                    )
                    .child(
                        Button::new("ssh-auth-key", "Private Key")
                            .style(if self.ssh_auth_method == SshAuthSelection::PrivateKey {
                                ButtonStyle::Filled
                            } else {
                                ButtonStyle::Subtle
                            })
                            .label_size(LabelSize::Small)
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.ssh_auth_method = SshAuthSelection::PrivateKey;
                                cx.notify();
                            })),
                    )
                    .child(
                        Button::new("ssh-auth-agent", "SSH Agent")
                            .style(if self.ssh_auth_method == SshAuthSelection::Agent {
                                ButtonStyle::Filled
                            } else {
                                ButtonStyle::Subtle
                            })
                            .label_size(LabelSize::Small)
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.ssh_auth_method = SshAuthSelection::Agent;
                                cx.notify();
                            })),
                    ),
            )
    }

    fn render_introspection_selector(&self, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap(px(2.0))
            .child(
                Label::new("Introspection Level")
                    .size(LabelSize::Small)
                    .color(ui::Color::Muted),
            )
            .child(
                h_flex()
                    .gap_1()
                    .child(
                        Button::new("introspection-names", "Names")
                            .style(
                                if self.introspection_level == IntrospectionLevel::Names {
                                    ButtonStyle::Filled
                                } else {
                                    ButtonStyle::Subtle
                                },
                            )
                            .label_size(LabelSize::Small)
                            .tooltip(Tooltip::text("Fast - object names only"))
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.introspection_level = IntrospectionLevel::Names;
                                cx.notify();
                            })),
                    )
                    .child(
                        Button::new("introspection-metadata", "Metadata")
                            .style(
                                if self.introspection_level == IntrospectionLevel::Metadata {
                                    ButtonStyle::Filled
                                } else {
                                    ButtonStyle::Subtle
                                },
                            )
                            .label_size(LabelSize::Small)
                            .tooltip(Tooltip::text("Columns, types, and constraints"))
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.introspection_level = IntrospectionLevel::Metadata;
                                cx.notify();
                            })),
                    )
                    .child(
                        Button::new("introspection-full-ddl", "Full DDL")
                            .style(
                                if self.introspection_level == IntrospectionLevel::FullDdl {
                                    ButtonStyle::Filled
                                } else {
                                    ButtonStyle::Subtle
                                },
                            )
                            .label_size(LabelSize::Small)
                            .tooltip(Tooltip::text("Complete DDL source"))
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.introspection_level = IntrospectionLevel::FullDdl;
                                cx.notify();
                            })),
                    ),
            )
    }
}

impl Render for ConnectionDialog {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_network_driver = matches!(self.active_tab, DriverTab::PostgreSql | DriverTab::MySql);

        let mut content = v_flex()
            .w(px(420.0))
            .p_4()
            .gap_3()
            .child(
                h_flex()
                    .justify_between()
                    .child(
                        Label::new("New Connection")
                            .size(LabelSize::Large)
                            .weight(gpui::FontWeight::BOLD),
                    )
                    .child(
                        Button::new("close-dialog", "")
                            .icon(ui::IconName::Close)
                            .icon_size(ui::IconSize::Small)
                            .style(ButtonStyle::Subtle)
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.dismiss(cx);
                            })),
                    ),
            )
            .child(self.render_tabs(cx))
            .child(Self::render_field("Name", &self.name_field, cx));

        if matches!(self.active_tab, DriverTab::Sqlite) {
            content = content.child(Self::render_field("File Path", &self.path_field, cx));
        }

        if is_network_driver {
            content = content
                .child(
                    h_flex()
                        .gap_2()
                        .child(
                            div()
                                .flex_grow()
                                .child(Self::render_field("Host", &self.host_field, cx)),
                        )
                        .child(
                            div()
                                .w(px(80.0))
                                .child(Self::render_field("Port", &self.port_field, cx)),
                        ),
                )
                .child(Self::render_field("Database", &self.database_field, cx))
                .child(Self::render_field("User", &self.user_field, cx))
                .child(Self::render_field("Password", &self.password_field, cx))
                .child(self.render_ssl_selector(cx))
                .child(self.render_ssh_section(cx));
        }

        content = content
            .child(self.render_color_picker(cx))
            .child(self.render_read_only_toggle(cx))
            .child(self.render_introspection_selector(cx));

        if let Some(error) = &self.error {
            content = content.child(
                Banner::new()
                    .severity(Severity::Error)
                    .child(Label::new(SharedString::from(error.clone()))),
            );
        }

        if let Some(test_result) = &self.test_result {
            match test_result {
                Ok(message) => {
                    content = content.child(
                        Banner::new()
                            .severity(Severity::Success)
                            .child(Label::new(SharedString::from(message.clone()))),
                    );
                }
                Err(error) => {
                    content = content.child(
                        Banner::new()
                            .severity(Severity::Error)
                            .child(Label::new(SharedString::from(error.clone()))),
                    );
                }
            }
        }

        content = content.child(
            h_flex()
                .justify_between()
                .child(
                    Button::new("test-connection-btn", "Test Connection")
                        .style(ButtonStyle::Subtle)
                        .label_size(LabelSize::Small)
                        .tooltip(Tooltip::text("Test the connection settings"))
                        .on_click(cx.listener(|this, _, _window, cx| {
                            this.test_connection(cx);
                        })),
                )
                .child(
                    h_flex()
                        .gap_1()
                        .child(
                            Button::new("cancel-btn", "Cancel")
                                .style(ButtonStyle::Subtle)
                                .label_size(LabelSize::Small)
                                .on_click(cx.listener(|this, _, _window, cx| {
                                    this.dismiss(cx);
                                })),
                        )
                        .child(
                            Button::new("save-btn", "Save")
                                .style(ButtonStyle::Filled)
                                .label_size(LabelSize::Small)
                                .on_click(cx.listener(|this, _, _window, cx| {
                                    this.save_connection(cx);
                                })),
                        ),
                ),
        );

        content
    }
}
