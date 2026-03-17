use call::{ActiveCall, Room, room};
use gpui::{
    DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, FontWeight, Render, Subscription,
    Task, Window,
};
use livekit_client::ConnectionQuality;
use std::time::Duration;
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

#[derive(Default)]
struct NetworkStats {
    connection_quality: Option<ConnectionQuality>,
    latency_ms: Option<f64>,
    jitter_ms: Option<f64>,
    packet_loss_pct: Option<f64>,
}

/// Subset of `NetworkStats` that can be computed on a background thread.
/// Excludes `connection_quality` which is read synchronously on the main thread.
struct ComputedNetworkStats {
    latency_ms: Option<f64>,
    jitter_ms: Option<f64>,
    packet_loss_pct: Option<f64>,
}

pub struct CallStatsModal {
    focus_handle: FocusHandle,
    input_lag: Option<Duration>,
    network_stats: NetworkStats,
    poll_task: Option<Task<()>>,
    _active_call_subscription: Option<Subscription>,
}

impl CallStatsModal {
    fn new(cx: &mut Context<Self>) -> Self {
        let mut this = Self {
            focus_handle: cx.focus_handle(),
            network_stats: NetworkStats::default(),
            input_lag: None,
            poll_task: None,
            _active_call_subscription: None,
        };

        if let Some(active_call) = ActiveCall::try_global(cx) {
            this._active_call_subscription =
                Some(cx.subscribe(&active_call, Self::handle_call_event));

            if active_call.read(cx).room().is_some() {
                this.start_polling(cx);
            }
        }

        this
    }

    fn handle_call_event(
        &mut self,
        _: Entity<ActiveCall>,
        event: &room::Event,
        cx: &mut Context<Self>,
    ) {
        match event {
            room::Event::RoomJoined { .. } => {
                self.start_polling(cx);
            }
            room::Event::RoomLeft { .. } => {
                self.stop_polling();
                self.network_stats = NetworkStats::default();
                cx.notify();
            }
            _ => {}
        }
    }

    fn start_polling(&mut self, cx: &mut Context<Self>) {
        self.poll_task = Some(cx.spawn(async move |this, cx| {
            loop {
                if this.update(cx, |this, cx| this.poll_stats(cx)).is_err() {
                    break;
                }
                cx.background_executor().timer(Duration::from_secs(1)).await;
            }
        }));
    }

    fn stop_polling(&mut self) {
        self.poll_task.take();
    }

    fn poll_stats(&mut self, cx: &mut Context<Self>) {
        let Some(room) = active_room(cx) else {
            return;
        };

        let connection_quality = room.read(cx).connection_quality();
        let stats_task = room.read(cx).get_stats(cx);
        self.input_lag = room.read(cx).input_lag();

        self.network_stats.connection_quality = Some(connection_quality);

        let background_task = cx.background_executor().spawn(async move {
            let session_stats = stats_task.await;
            session_stats.map(|stats| Self::compute_network_stats(&stats))
        });

        cx.spawn(async move |this, cx| {
            let result = background_task.await;
            this.update(cx, |this, cx| {
                if let Some(computed) = result {
                    this.network_stats.latency_ms = computed.latency_ms;
                    this.network_stats.jitter_ms = computed.jitter_ms;
                    this.network_stats.packet_loss_pct = computed.packet_loss_pct;
                }
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    /// Pure computation over session stats — safe to call on any thread.
    fn compute_network_stats(stats: &livekit_client::SessionStats) -> ComputedNetworkStats {
        let mut best_rtt: Option<f64> = None;
        let mut best_jitter: Option<f64> = None;
        let mut total_packets_received: u64 = 0;
        let mut total_packets_lost: i64 = 0;

        let all_stats = stats
            .publisher_stats
            .iter()
            .chain(stats.subscriber_stats.iter());

        for stat in all_stats {
            Self::extract_metrics(
                stat,
                &mut best_rtt,
                &mut best_jitter,
                &mut total_packets_received,
                &mut total_packets_lost,
            );
        }

        let total_expected = total_packets_received as i64 + total_packets_lost;
        let packet_loss_pct = if total_expected > 0 {
            Some((total_packets_lost as f64 / total_expected as f64) * 100.0)
        } else {
            None
        };

        ComputedNetworkStats {
            latency_ms: best_rtt.map(|rtt| rtt * 1000.0),
            jitter_ms: best_jitter.map(|j| j * 1000.0),
            packet_loss_pct,
        }
    }

    #[cfg(all(
        not(rust_analyzer),
        any(
            test,
            feature = "test-support",
            all(target_os = "windows", target_env = "gnu"),
            target_os = "freebsd"
        )
    ))]
    fn extract_metrics(
        _stat: &livekit_client::RtcStats,
        _best_rtt: &mut Option<f64>,
        _best_jitter: &mut Option<f64>,
        _total_packets_received: &mut u64,
        _total_packets_lost: &mut i64,
    ) {
    }

    #[cfg(any(
        rust_analyzer,
        not(any(
            test,
            feature = "test-support",
            all(target_os = "windows", target_env = "gnu"),
            target_os = "freebsd"
        ))
    ))]
    fn extract_metrics(
        stat: &livekit_client::RtcStats,
        best_rtt: &mut Option<f64>,
        best_jitter: &mut Option<f64>,
        total_packets_received: &mut u64,
        total_packets_lost: &mut i64,
    ) {
        use livekit_client::RtcStats;

        match stat {
            RtcStats::CandidatePair(pair) => {
                let rtt = pair.candidate_pair.current_round_trip_time;
                if rtt > 0.0 {
                    *best_rtt = Some(match *best_rtt {
                        Some(current) => current.min(rtt),
                        None => rtt,
                    });
                }
            }
            RtcStats::InboundRtp(inbound) => {
                let jitter = inbound.received.jitter;
                if jitter > 0.0 {
                    *best_jitter = Some(match *best_jitter {
                        Some(current) => current.max(jitter),
                        None => jitter,
                    });
                }
                *total_packets_received += inbound.received.packets_received;
                *total_packets_lost += inbound.received.packets_lost;
            }
            RtcStats::RemoteInboundRtp(remote_inbound) => {
                let rtt = remote_inbound.remote_inbound.round_trip_time;
                if rtt > 0.0 {
                    *best_rtt = Some(match *best_rtt {
                        Some(current) => current.min(rtt),
                        None => rtt,
                    });
                }
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
        let is_connected = active_room(cx).is_some();

        let (quality_text, quality_color) = quality_label(self.network_stats.connection_quality);

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
                            self.network_stats.latency_ms,
                            |v| format!("{:.0}ms", v),
                            |v| metric_rating("Latency", v),
                        ))
                        .child(self.render_metric_row(
                            "Jitter",
                            "Variance or fluctuation in latency",
                            self.network_stats.jitter_ms,
                            |v| format!("{:.0}ms", v),
                            |v| metric_rating("Jitter", v),
                        ))
                        .child(self.render_metric_row(
                            "Packet loss",
                            "Amount of data lost during transfer",
                            self.network_stats.packet_loss_pct,
                            |v| format!("{:.1}%", v),
                            |v| packet_loss_rating(v),
                        ))
                        .child(self.render_metric_row(
                            "Input lag",
                            "Delay from audio capture to WebRTC",
                            self.input_lag.map(|d| d.as_secs_f64() * 1000.0),
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
