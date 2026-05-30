use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

use editor::Editor;
use gpui::{App, Context, Entity, EventEmitter, FocusHandle, Focusable, SharedString, Window, prelude::*};
use theme::ActiveTheme;
use ui::{Button, Color, Icon, IconName, Label, LabelSize, h_flex, prelude::*, v_flex};
use workspace::{Item, Workspace};

const REQUEST_LIMIT: usize = 100;

pub fn init(cx: &mut App) {
    cx.observe_new(
        |workspace: &mut Workspace, _window, _cx: &mut Context<Workspace>| {
            workspace.register_action(|workspace, _: &zed_actions::OpenZedApi, window, cx| {
                let view = cx.new(|cx| ZedApiView::new(window, cx));
                workspace.add_item_to_active_pane(Box::new(view), None, true, window, cx);
            });
        },
    )
    .detach();
}

struct HealthStatus {
    ok: bool,
    message: SharedString,
}

pub struct ZedApiView {
    focus_handle: FocusHandle,
    port_editor: Entity<Editor>,
    health_status: Option<HealthStatus>,
    requests: Vec<agent::AgentHttpRequestLogEntry>,
}

impl ZedApiView {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let port_editor = cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text("API Port", window, cx);
            editor.set_text(agent::configured_agent_http_port().to_string(), window, cx);
            editor
        });

        Self {
            focus_handle: cx.focus_handle(),
            port_editor,
            health_status: None,
            requests: agent::recent_agent_http_requests(REQUEST_LIMIT),
        }
    }

    fn selected_port(&self, cx: &App) -> Result<u16, SharedString> {
        let text = self.port_editor.read(cx).text(cx);
        let port = text
            .trim()
            .parse::<u16>()
            .map_err(|_| "Port must be a valid number".to_string())?;
        if port == 0 {
            return Err("Port must be greater than 0".into());
        }
        Ok(port)
    }

    fn apply_port(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        match self.selected_port(cx) {
            Ok(port) => {
                agent::set_configured_agent_http_port(port);
                self.health_status = Some(HealthStatus {
                    ok: true,
                    message: format!("Port set to {port} (restart may be required)").into(),
                });
            }
            Err(message) => {
                self.health_status = Some(HealthStatus { ok: false, message });
                self.port_editor
                    .update(cx, |editor, cx| editor.focus_handle(cx).focus(window, cx));
            }
        }
        cx.notify();
    }

    fn refresh_requests(&mut self, cx: &mut Context<Self>) {
        self.requests = agent::recent_agent_http_requests(REQUEST_LIMIT);
        cx.notify();
    }

    fn run_health_check(&mut self, cx: &mut Context<Self>) {
        let port = match self.selected_port(cx) {
            Ok(port) => port,
            Err(message) => {
                self.health_status = Some(HealthStatus { ok: false, message });
                cx.notify();
                return;
            }
        };

        let addr = SocketAddr::from(([127, 0, 0, 1], port));
        let status = match TcpStream::connect_timeout(&addr, Duration::from_millis(1500)) {
            Ok(mut stream) => {
                let _ = stream.set_read_timeout(Some(Duration::from_millis(1500)));
                let _ = stream.set_write_timeout(Some(Duration::from_millis(1500)));
                let request = format!(
                    "GET /healthz HTTP/1.1\r\nHost: 127.0.0.1:{port}\r\nConnection: close\r\n\r\n"
                );
                if stream.write_all(request.as_bytes()).is_err() {
                    HealthStatus {
                        ok: false,
                        message: "Failed to write health check request".into(),
                    }
                } else {
                    let mut response = String::new();
                    if stream.read_to_string(&mut response).is_err() {
                        HealthStatus {
                            ok: false,
                            message: "No response received from API".into(),
                        }
                    } else if response.contains("200") && response.to_lowercase().contains("ok") {
                        HealthStatus {
                            ok: true,
                            message: format!("Healthy on :{port}").into(),
                        }
                    } else {
                        let headline = response.lines().next().unwrap_or("Unexpected response");
                        HealthStatus {
                            ok: false,
                            message: format!("Health check failed: {headline}").into(),
                        }
                    }
                }
            }
            Err(err) => HealthStatus {
                ok: false,
                message: format!("Connection failed: {err}").into(),
            },
        };

        self.health_status = Some(status);
        self.refresh_requests(cx);
    }
}

impl EventEmitter<()> for ZedApiView {}

impl Focusable for ZedApiView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for ZedApiView {
    type Event = ();

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        "Zed API".into()
    }

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {
        Some(Icon::new(IconName::Server))
    }
}

impl Render for ZedApiView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let status = self.health_status.as_ref();

        v_flex()
            .track_focus(&self.focus_handle)
            .size_full()
            .p_4()
            .gap_3()
            .bg(cx.theme().colors().editor_background)
            .child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .child(Label::new("Port").size(LabelSize::Small))
                    .child(div().w(px(120.)).child(self.port_editor.clone()))
                    .child(
                        Button::new("zed-api-apply-port", "Apply")
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.apply_port(window, cx);
                            })),
                    )
                    .child(
                        Button::new("zed-api-health-check", "Health Check")
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.run_health_check(cx);
                            })),
                    )
                    .child(
                        Button::new("zed-api-refresh-requests", "Refresh Requests")
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.refresh_requests(cx);
                            })),
                    ),
            )
            .child(
                h_flex().gap_2().items_center().child(
                    Label::new(
                        status
                            .map(|s| s.message.clone())
                            .unwrap_or_else(|| "Run health check to verify API status".into()),
                    )
                    .size(LabelSize::Small)
                    .color(if status.is_some_and(|s| s.ok) {
                        Color::Success
                    } else {
                        Color::Muted
                    }),
                ),
            )
            .child(Label::new("Recent Requests").size(LabelSize::Small))
            .child(
                v_flex()
                    .flex_1()
                    .children(self.requests.iter().map(|entry| {
                        let timestamp = entry.timestamp.format("%H:%M:%S").to_string();
                        let method = entry.method.clone();
                        let path = entry.path.clone();
                        let status = entry.status_code;
                        h_flex()
                            .w_full()
                            .gap_3()
                            .py_1()
                            .border_b_1()
                            .border_color(cx.theme().colors().border_variant)
                            .child(Label::new(timestamp).size(LabelSize::XSmall).color(Color::Muted))
                            .child(Label::new(method).size(LabelSize::XSmall))
                            .child(Label::new(path).size(LabelSize::XSmall).color(Color::Muted))
                            .child(
                                Label::new(status.to_string())
                                    .size(LabelSize::XSmall)
                                    .color(if (200..300).contains(&status) {
                                        Color::Success
                                    } else {
                                        Color::Error
                                    }),
                            )
                    })),
            )
    }
}
