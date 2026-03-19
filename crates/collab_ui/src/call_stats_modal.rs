use call::{ActiveCall, Room, room};
use gpui::{
    DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, FontWeight, Render, Subscription,
    Window,
};
use livekit_client::ConnectionQuality;
use ui::prelude::*;
use workspace::{ModalView, Workspace};
use zed_actions::ShowCallStats;

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _, _cx| {
        workspace.register_action(|workspace, _: &ShowCallStats, window, cx| {
            workspace.toggle_modal(window, cx, |_window, cx| CallStatsModal::new(cx));
        });
    })
    .detach();
}

pub struct CallStatsModal {
    focus_handle: FocusHandle,
    _active_call_subscription: Option<Subscription>,
    _diagnostics_subscription: Option<Subscription>,
}

impl CallStatsModal {
    fn new(cx: &mut Context<Self>) -> Self {
        let mut this = Self {
            focus_handle: cx.focus_handle(),
            _active_call_subscription: None,
            _diagnostics_subscription: None,
        };

        if let Some(active_call) = ActiveCall::try_global(cx) {
            this._active_call_subscription =
                Some(cx.subscribe(&active_call, Self::handle_call_event));
            this.observe_diagnostics(cx);
        }

        this
    }

    fn observe_diagnostics(&mut self, cx: &mut Context<Self>) {
        let diagnostics = active_room(cx).and_then(|room| room.read(cx).diagnostics().cloned());

        if let Some(diagnostics) = diagnostics {
            self._diagnostics_subscription = Some(cx.observe(&diagnostics, |_, _, cx| cx.notify()));
        } else {
            self._diagnostics_subscription = None;
        }
    }

    fn handle_call_event(
        &mut self,
        _: Entity<ActiveCall>,
        event: &room::Event,
        cx: &mut Context<Self>,
    ) {
        match event {
            room::Event::RoomJoined { .. } => {
                self.observe_diagnostics(cx);
            }
            room::Event::RoomLeft { .. } => {
                self._diagnostics_subscription = None;
                cx.notify();
            }
            _ => {}
        }
    }

    fn dismiss(&mut self, _: &menu::Cancel, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }
}

fn active_room(cx: &App) -> Option<Entity<Room>> {
    ActiveCall::try_global(cx)?.read(cx).room().cloned()
}

fn quality_label(quality: Option<ConnectionQuality>) -> (&'static str, Color) {
    match quality {
        Some(ConnectionQuality::Excellent) => ("Excellent", Color::Success),
        Some(ConnectionQuality::Good) => ("Good", Color::Success),
        Some(ConnectionQuality::Poor) => ("Poor", Color::Warning),
        Some(ConnectionQuality::Lost) => ("Lost", Color::Error),
        None => ("—", Color::Muted),
    }
}

fn metric_rating(label: &str, value_ms: f64) -> (&'static str, Color) {
    match label {
        "Latency" => {
            if value_ms < 100.0 {
                ("Normal", Color::Success)
            } else if value_ms < 300.0 {
                ("High", Color::Warning)
            } else {
                ("Poor", Color::Error)
            }
        }
        "Jitter" => {
            if value_ms < 30.0 {
                ("Normal", Color::Success)
            } else if value_ms < 75.0 {
                ("High", Color::Warning)
            } else {
                ("Poor", Color::Error)
            }
        }
        _ => ("Normal", Color::Success),
    }
}

fn input_lag_rating(value_ms: f64) -> (&'static str, Color) {
    if value_ms < 20.0 {
        ("Normal", Color::Success)
    } else if value_ms < 50.0 {
        ("High", Color::Warning)
    } else {
        ("Poor", Color::Error)
    }
}

fn packet_loss_rating(loss_pct: f64) -> (&'static str, Color) {
    if loss_pct < 1.0 {
        ("Normal", Color::Success)
    } else if loss_pct < 5.0 {
        ("High", Color::Warning)
    } else {
        ("Poor", Color::Error)
    }
}

impl EventEmitter<DismissEvent> for CallStatsModal {}
impl ModalView for CallStatsModal {}

impl Focusable for CallStatsModal {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for CallStatsModal {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let room = active_room(cx);
        let is_connected = room.is_some();
        let stats = room
            .and_then(|room| {
                let diagnostics = room.read(cx).diagnostics()?;
                Some(diagnostics.read(cx).stats().clone())
            })
            .unwrap_or_default();

        let (quality_text, quality_color) = quality_label(stats.connection_quality);

        v_flex()
            .key_context("CallStatsModal")
            .on_action(cx.listener(Self::dismiss))
            .track_focus(&self.focus_handle)
            .elevation_3(cx)
            .w(rems(24.))
            .p_4()
            .gap_3()
            .child(
                h_flex()
                    .justify_between()
                    .child(Label::new("Call Diagnostics").size(LabelSize::Large))
                    .child(
                        Label::new(quality_text)
                            .size(LabelSize::Large)
                            .color(quality_color),
                    ),
            )
            .when(!is_connected, |this| {
                this.child(
                    h_flex()
                        .justify_center()
                        .py_4()
                        .child(Label::new("Not in a call").color(Color::Muted)),
                )
            })
            .when(is_connected, |this| {
                this.child(
                    v_flex()
                        .gap_1()
                        .child(
                            h_flex()
                                .gap_2()
                                .child(Label::new("Network").weight(FontWeight::SEMIBOLD)),
                        )
                        .child(self.render_metric_row(
                            "Latency",
                            "Time for data to travel to the server",
                            stats.latency_ms,
                            |v| format!("{:.0}ms", v),
                            |v| metric_rating("Latency", v),
                        ))
                        .child(self.render_metric_row(
                            "Jitter",
                            "Variance or fluctuation in latency",
                            stats.jitter_ms,
                            |v| format!("{:.0}ms", v),
                            |v| metric_rating("Jitter", v),
                        ))
                        .child(self.render_metric_row(
                            "Packet loss",
                            "Amount of data lost during transfer",
                            stats.packet_loss_pct,
                            |v| format!("{:.1}%", v),
                            |v| packet_loss_rating(v),
                        ))
                        .child(self.render_metric_row(
                            "Input lag",
                            "Delay from audio capture to WebRTC",
                            stats.input_lag.map(|d| d.as_secs_f64() * 1000.0),
                            |v| format!("{:.1}ms", v),
                            |v| input_lag_rating(v),
                        )),
                )
            })
    }
}

impl CallStatsModal {
    fn render_metric_row(
        &self,
        title: &str,
        description: &str,
        value: Option<f64>,
        format_value: impl Fn(f64) -> String,
        rate: impl Fn(f64) -> (&'static str, Color),
    ) -> impl IntoElement {
        let (rating_text, rating_color, value_text) = match value {
            Some(v) => {
                let (rt, rc) = rate(v);
                (rt, rc, format_value(v))
            }
            None => ("—", Color::Muted, "—".to_string()),
        };

        h_flex()
            .px_2()
            .py_1()
            .rounded_md()
            .justify_between()
            .child(
                v_flex()
                    .child(Label::new(title.to_string()).size(LabelSize::Default))
                    .child(
                        Label::new(description.to_string())
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    ),
            )
            .child(
                v_flex()
                    .items_end()
                    .child(
                        Label::new(rating_text)
                            .size(LabelSize::Default)
                            .color(rating_color),
                    )
                    .child(
                        Label::new(value_text)
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    ),
            )
    }
}
