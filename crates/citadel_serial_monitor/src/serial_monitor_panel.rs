use crate::serial_connection::{
    DEFAULT_BAUD_RATE, GlobalSerialConnection, SerialConnection, SerialConnectionError,
    SerialLineReceived, default_port_name,
};
use crate::show_error_toast_in_workspace;
use editor::Editor;
use gpui::{
    Action, App, Context, Entity, EventEmitter, FocusHandle, Focusable, Pixels, Render,
    Subscription, WeakEntity, Window, actions, px, uniform_list,
};
use ui::prelude::*;
use workspace::Workspace;
use workspace::dock::{DockPosition, Panel, PanelEvent};

actions!(
    citadel_serial_monitor_panel,
    [
        /// Connects (or disconnects) the Serial Monitor's connection.
        ToggleConnection,
        /// Sends the send box's contents to the connected device.
        SendToDevice
    ]
);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LineEnding {
    None,
    Newline,
    CarriageReturn,
    CarriageReturnNewline,
}

impl LineEnding {
    fn as_bytes(self) -> &'static [u8] {
        match self {
            LineEnding::None => b"",
            LineEnding::Newline => b"\n",
            LineEnding::CarriageReturn => b"\r",
            LineEnding::CarriageReturnNewline => b"\r\n",
        }
    }

    fn label(self) -> &'static str {
        match self {
            LineEnding::None => "No line ending",
            LineEnding::Newline => "Newline",
            LineEnding::CarriageReturn => "Carriage return",
            LineEnding::CarriageReturnNewline => "Both NL & CR",
        }
    }

    fn next(self) -> Self {
        match self {
            LineEnding::None => LineEnding::Newline,
            LineEnding::Newline => LineEnding::CarriageReturn,
            LineEnding::CarriageReturn => LineEnding::CarriageReturnNewline,
            LineEnding::CarriageReturnNewline => LineEnding::None,
        }
    }
}

pub struct SerialMonitorPanel {
    focus_handle: FocusHandle,
    workspace: WeakEntity<Workspace>,
    connection: Entity<SerialConnection>,
    port_editor: Entity<Editor>,
    baud_editor: Entity<Editor>,
    send_editor: Entity<Editor>,
    line_ending: LineEnding,
    position: DockPosition,
    _subscriptions: Vec<Subscription>,
}

impl SerialMonitorPanel {
    pub async fn load(
        workspace: WeakEntity<Workspace>,
        mut cx: gpui::AsyncWindowContext,
    ) -> anyhow::Result<Entity<Self>> {
        workspace.update_in(&mut cx, |workspace, window, cx| {
            Self::new(workspace, window, cx)
        })
    }

    fn new(
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<Self> {
        let workspace_handle = workspace.weak_handle();

        cx.new(|cx| {
            // Invariant: citadel_serial_monitor::init(cx) (which sets this
            // global) runs during app startup, before any workspace (and
            // therefore any panel) is created -- see crates/zed/src/main.rs.
            let connection = cx.global::<GlobalSerialConnection>().0.clone();

            let default_port = default_port_name(cx).unwrap_or_default();
            let port_editor = cx.new(|cx| {
                let mut editor = Editor::single_line(window, cx);
                editor.set_text(default_port, window, cx);
                editor.set_placeholder_text("Port (e.g. /dev/ttyACM0)", window, cx);
                editor
            });
            let baud_editor = cx.new(|cx| {
                let mut editor = Editor::single_line(window, cx);
                editor.set_text(DEFAULT_BAUD_RATE.to_string(), window, cx);
                editor
            });
            let send_editor = cx.new(|cx| {
                let mut editor = Editor::single_line(window, cx);
                editor.set_placeholder_text("Send to device...", window, cx);
                editor
            });

            let mut subscriptions = Vec::new();
            subscriptions.push(cx.subscribe(
                &connection,
                |_this: &mut Self, _connection, _event: &SerialLineReceived, cx| {
                    cx.notify();
                },
            ));
            subscriptions.push(cx.subscribe_in(
                &connection,
                window,
                |this, _connection, event: &SerialConnectionError, _window, cx| {
                    let message = event.0.clone();
                    this.workspace
                        .update(cx, |workspace, cx| {
                            show_error_toast_in_workspace(workspace, message, cx);
                        })
                        .ok();
                    cx.notify();
                },
            ));

            Self {
                focus_handle: cx.focus_handle(),
                workspace: workspace_handle,
                connection,
                port_editor,
                baud_editor,
                send_editor,
                line_ending: LineEnding::None,
                position: DockPosition::Bottom,
                _subscriptions: subscriptions,
            }
        })
    }

    fn toggle_connection(
        &mut self,
        _: &ToggleConnection,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.connection.read(cx).is_open {
            self.connection
                .update(cx, |connection, cx| connection.disconnect(cx));
            return;
        }

        let port_name = self.port_editor.read(cx).text(cx).trim().to_string();
        if port_name.is_empty() {
            if let Some(workspace) = self.workspace.upgrade() {
                workspace.update(cx, |workspace, cx| {
                    show_error_toast_in_workspace(workspace, "Enter a port name first.", cx);
                });
            }
            return;
        }

        let baud_text = self.baud_editor.read(cx).text(cx);
        let Ok(baud_rate) = baud_text.trim().parse::<u32>() else {
            if let Some(workspace) = self.workspace.upgrade() {
                workspace.update(cx, |workspace, cx| {
                    show_error_toast_in_workspace(workspace, "Baud rate must be a number.", cx);
                });
            }
            return;
        };

        self.connection.update(cx, |connection, cx| {
            connection.connect(port_name, baud_rate, cx)
        });
    }

    fn send_to_device(&mut self, _: &SendToDevice, window: &mut Window, cx: &mut Context<Self>) {
        let text = self.send_editor.read(cx).text(cx);
        let mut bytes = text.into_bytes();
        bytes.extend_from_slice(self.line_ending.as_bytes());
        self.connection
            .update(cx, |connection, cx| connection.send(bytes, cx));
        self.send_editor
            .update(cx, |editor, cx| editor.set_text("", window, cx));
    }

    fn cycle_line_ending(&mut self, cx: &mut Context<Self>) {
        self.line_ending = self.line_ending.next();
        cx.notify();
    }

    fn save_log(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let lines: Vec<String> = self.connection.read(cx).lines.iter().cloned().collect();
        let start_dir = std::env::home_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
        let receiver = cx.prompt_for_new_path(&start_dir, Some("serial-log.txt"));
        let workspace = self.workspace.clone();
        cx.spawn_in(window, async move |_this, cx| {
            let Ok(Ok(Some(path))) = receiver.await else {
                return;
            };
            let contents = lines.join("\n");
            let write_result = cx
                .background_spawn(async move { std::fs::write(&path, contents) })
                .await;
            if let Err(error) = write_result {
                workspace
                    .update(cx, |workspace, cx| {
                        show_error_toast_in_workspace(workspace, error.to_string(), cx);
                    })
                    .ok();
            }
        })
        .detach();
    }
}

impl EventEmitter<PanelEvent> for SerialMonitorPanel {}

impl Focusable for SerialMonitorPanel {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Panel for SerialMonitorPanel {
    fn persistent_name() -> &'static str {
        "SerialMonitorPanel"
    }

    fn panel_key() -> &'static str {
        "SerialMonitorPanel"
    }

    fn position(&self, _window: &Window, _cx: &App) -> DockPosition {
        self.position
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(
            position,
            DockPosition::Bottom | DockPosition::Left | DockPosition::Right
        )
    }

    fn set_position(
        &mut self,
        position: DockPosition,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.position = position;
        cx.notify();
    }

    fn default_size(&self, _window: &Window, _cx: &App) -> Pixels {
        px(300.)
    }

    fn icon(&self, _window: &Window, _cx: &App) -> Option<ui::IconName> {
        Some(ui::IconName::SignalHigh)
    }

    fn icon_tooltip(&self, _window: &Window, _cx: &App) -> Option<&'static str> {
        Some("Serial Monitor")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(crate::ToggleFocus)
    }

    fn activation_priority(&self) -> u32 {
        11
    }
}

impl Render for SerialMonitorPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_open = self.connection.read(cx).is_open;
        let line_count = self.connection.read(cx).lines.len();
        let connection = self.connection.clone();

        v_flex()
            .key_context("SerialMonitorPanel")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::toggle_connection))
            .on_action(cx.listener(Self::send_to_device))
            .size_full()
            .child(
                h_flex()
                    .gap_2()
                    .p_2()
                    .child(self.port_editor.clone())
                    .child(self.baud_editor.clone())
                    .child(
                        Button::new(
                            "toggle-connection",
                            if is_open { "Disconnect" } else { "Connect" },
                        )
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.toggle_connection(&ToggleConnection, window, cx);
                        })),
                    )
                    .child(
                        Button::new("save-log", "Save Log")
                            .on_click(cx.listener(|this, _, window, cx| this.save_log(window, cx))),
                    ),
            )
            .child(
                uniform_list(
                    "serial-monitor-log",
                    line_count,
                    move |range, _window, cx| {
                        connection
                            .read(cx)
                            .lines
                            .iter()
                            .skip(range.start)
                            .take(range.end - range.start)
                            .map(|line| Label::new(line.clone()))
                            .collect()
                    },
                )
                .size_full(),
            )
            .child(
                h_flex()
                    .gap_2()
                    .p_2()
                    .child(self.send_editor.clone())
                    .child(
                        Button::new("line-ending", self.line_ending.label()).on_click(
                            cx.listener(|this, _, _window, cx| this.cycle_line_ending(cx)),
                        ),
                    )
                    .child(Button::new("send", "Send").on_click(cx.listener(
                        |this, _, window, cx| {
                            this.send_to_device(&SendToDevice, window, cx);
                        },
                    ))),
            )
    }
}
