use anyhow::Result;
use call::{
    ActiveCall,
    diagnostics::{CallDiagnostics, RemoteAudioDiagnostics},
    room,
};
use gpui::{
    ClipboardItem, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, FontWeight, Render,
    Subscription, Task, TaskExt as _, Window,
};
use livekit_client::ConnectionQuality;
use release_channel::{AppVersion, ReleaseChannel};
use serde::Serialize;
use std::{cmp::Reverse, path::PathBuf};
use ui::prelude::*;
use workspace::{ModalView, Workspace};
use zed_actions::ShowCallStats;

const WEBRTC_AUDIO_SAMPLES_PER_MILLISECOND: f64 = 48.0;
const PLAYBACK_FRAME_DURATION_MILLISECONDS: u64 = 10;

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

#[derive(Serialize)]
struct ExportedCallDiagnostics {
    application_version: String,
    release_channel: String,
    operating_system: &'static str,
    architecture: &'static str,
    diagnostics: call::diagnostics::CallDiagnosticsReport,
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
        let diagnostics = call_diagnostics(cx);

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
                self.observe_diagnostics(cx);
                cx.notify();
            }
            _ => {}
        }
    }

    fn dismiss(&mut self, _: &menu::Cancel, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }

    fn report(cx: &App) -> Option<ExportedCallDiagnostics> {
        let diagnostics = call_diagnostics(cx)?.read(cx).report();
        let release_channel = ReleaseChannel::try_global(cx)
            .map(|channel| channel.dev_name().to_string())
            .unwrap_or_else(|| "unknown".to_string());
        Some(ExportedCallDiagnostics {
            application_version: AppVersion::global(cx).to_string(),
            release_channel,
            operating_system: std::env::consts::OS,
            architecture: std::env::consts::ARCH,
            diagnostics,
        })
    }

    fn serialize_report(cx: &App) -> Option<Task<Result<String>>> {
        let report = Self::report(cx)?;
        Some(
            cx.background_executor()
                .spawn(async move { Ok(serde_json::to_string_pretty(&report)?) }),
        )
    }

    fn copy_report(&mut self, cx: &mut Context<Self>) {
        let Some(report) = Self::serialize_report(cx) else {
            return;
        };
        cx.spawn(async move |_, cx| {
            let report = report.await?;
            cx.update(|cx| {
                cx.write_to_clipboard(ClipboardItem::new_string(report));
            });
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
    }

    fn save_report(&mut self, cx: &mut Context<Self>) {
        let Some(report) = Self::report(cx) else {
            return;
        };
        let save_dialog =
            cx.prompt_for_new_path(&PathBuf::default(), Some("zed-call-diagnostics.json"));
        cx.spawn(async move |_, cx| {
            let Some(path) = save_dialog.await?? else {
                return anyhow::Ok(());
            };
            cx.background_spawn(async move {
                let report = serde_json::to_string_pretty(&report)?;
                smol::fs::write(path, report).await?;
                anyhow::Ok(())
            })
            .await
        })
        .detach_and_log_err(cx);
    }
}

fn call_diagnostics(cx: &App) -> Option<Entity<CallDiagnostics>> {
    ActiveCall::try_global(cx)?.read(cx).call_diagnostics(cx)
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

fn input_lag_rating(value_ms: u128) -> (&'static str, Color) {
    if value_ms < 20 {
        ("Normal", Color::Success)
    } else if value_ms < 50 {
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

fn audio_issue_score(audio: &RemoteAudioDiagnostics) -> u64 {
    audio
        .concealment_events
        .saturating_add(audio.frames_dropped)
        .saturating_add(audio.queue_underflows)
        .saturating_add(u64::from(
            audio.packet_loss_pct.is_some_and(|loss| loss >= 1.0),
        ))
}

fn format_audio_duration(milliseconds: f64) -> String {
    if milliseconds > 0.0 && milliseconds < 1.0 {
        "<1ms".to_string()
    } else if milliseconds < 10.0 && milliseconds.fract() != 0.0 {
        format!("{milliseconds:.1}ms")
    } else {
        format!("{milliseconds:.0}ms")
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
        let is_connected = ActiveCall::try_global(cx)
            .is_some_and(|active_call| active_call.read(cx).room().is_some());
        let diagnostics = call_diagnostics(cx);
        let (latest, sample_count, retained_duration, recent_issue_count) = diagnostics
            .as_ref()
            .map(|diagnostics| {
                let diagnostics = diagnostics.read(cx);
                let retained_duration = diagnostics
                    .history()
                    .front()
                    .zip(diagnostics.history().back())
                    .and_then(|(first, last)| last.elapsed.0.checked_sub(first.elapsed.0))
                    .unwrap_or_default();
                let recent_issue_count = diagnostics
                    .history()
                    .iter()
                    .rev()
                    .take(60)
                    .filter(|snapshot| {
                        snapshot
                            .remote_audio
                            .iter()
                            .any(|audio| audio_issue_score(audio) > 0)
                    })
                    .count();
                (
                    diagnostics.latest().cloned(),
                    diagnostics.history().len(),
                    retained_duration,
                    recent_issue_count,
                )
            })
            .unwrap_or_default();
        let stats = latest
            .as_ref()
            .map(|snapshot| snapshot.stats.clone())
            .unwrap_or_default();
        let mut remote_audio = latest
            .map(|snapshot| snapshot.remote_audio)
            .unwrap_or_default();
        remote_audio.sort_by_key(|audio| Reverse(audio_issue_score(audio)));

        let (quality_text, quality_color) =
            quality_label(stats.connection_quality.map(|inner| inner.0));
        let has_diagnostics = sample_count > 0;

        v_flex()
            .key_context("CallStatsModal")
            .on_action(cx.listener(Self::dismiss))
            .track_focus(&self.focus_handle)
            .elevation_3(cx)
            .w(rems(36.))
            .max_h(rems(42.))
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
            .when(!is_connected && has_diagnostics, |this| {
                this.child(
                    h_flex()
                        .justify_center()
                        .child(Label::new("Showing diagnostics from the most recent call").color(Color::Muted)),
                )
            })
            .when(!has_diagnostics, |this| {
                this.child(
                    h_flex()
                        .justify_center()
                        .py_4()
                        .child(Label::new("No call diagnostics available").color(Color::Muted)),
                )
            })
            .when(has_diagnostics, |this| {
                this.child(
                    v_flex()
                        .id("call-diagnostics-scroll")
                        .gap_3()
                        .max_h(rems(32.))
                        .overflow_y_scroll()
                        .child(
                            Label::new(format!(
                                "{sample_count} samples · {:.0}s retained · {recent_issue_count} affected intervals in the last 60s",
                                retained_duration.as_secs_f64()
                            ))
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                        )
                        .child(
                            v_flex()
                                .gap_1()
                                .child(Label::new("Network").weight(FontWeight::SEMIBOLD))
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
                                    packet_loss_rating,
                                ))
                                .child(self.render_metric_row(
                                    "Input lag",
                                    "Delay from audio capture to WebRTC",
                                    stats.input_lag.map(|d| d.0.as_millis()),
                                    |v| format!("{}ms", v),
                                    input_lag_rating,
                                )),
                        )
                        .child(
                            v_flex()
                                .gap_1()
                                .child(Label::new("Inbound audio").weight(FontWeight::SEMIBOLD))
                                .when(remote_audio.is_empty(), |this| {
                                    this.child(
                                        Label::new("Waiting for inbound audio statistics")
                                            .color(Color::Muted),
                                    )
                                })
                                .children(
                                    remote_audio
                                        .into_iter()
                                        .map(|audio| self.render_remote_audio(audio)),
                                ),
                        ),
                )
            })
            .when(has_diagnostics, |this| {
                this.child(
                    h_flex()
                        .justify_end()
                        .gap_2()
                        .child(
                            Button::new("copy-call-diagnostics", "Copy Report")
                                .on_click(cx.listener(|this, _, _, cx| this.copy_report(cx))),
                        )
                        .child(
                            Button::new("save-call-diagnostics", "Save Report…")
                                .on_click(cx.listener(|this, _, _, cx| this.save_report(cx))),
                        ),
                )
            })
    }
}

impl CallStatsModal {
    fn render_remote_audio(&self, audio: RemoteAudioDiagnostics) -> impl IntoElement {
        let issue_score = audio_issue_score(&audio);
        let (status, color) = if issue_score > 0 {
            ("Affected", Color::Warning)
        } else {
            ("Healthy", Color::Success)
        };
        let packet_loss = audio
            .packet_loss_pct
            .map(|loss| format!("{loss:.1}%"))
            .unwrap_or_else(|| "—".to_string());
        let jitter_buffer_delay = audio
            .jitter_buffer_delay_ms
            .map(|delay| format!("{delay:.1}ms"))
            .unwrap_or_else(|| "—".to_string());
        let repaired_audio_duration = format_audio_duration(
            audio.concealed_samples as f64 / WEBRTC_AUDIO_SAMPLES_PER_MILLISECOND,
        );
        let starved_audio_duration = format_audio_duration(
            audio
                .queue_underflows
                .saturating_mul(PLAYBACK_FRAME_DURATION_MILLISECONDS) as f64,
        );
        let dropped_audio_duration = format_audio_duration(
            audio
                .frames_dropped
                .saturating_mul(PLAYBACK_FRAME_DURATION_MILLISECONDS) as f64,
        );
        let buffered_audio_duration = format_audio_duration(
            audio
                .current_queue_depth
                .saturating_mul(PLAYBACK_FRAME_DURATION_MILLISECONDS) as f64,
        );
        let peak_buffered_audio_duration = format_audio_duration(
            audio
                .maximum_queue_depth
                .saturating_mul(PLAYBACK_FRAME_DURATION_MILLISECONDS) as f64,
        );
        let repair_event_label = if audio.concealment_events == 1 {
            "event"
        } else {
            "events"
        };

        v_flex()
            .px_2()
            .py_1()
            .gap_1()
            .rounded_md()
            .child(
                h_flex()
                    .justify_between()
                    .child(
                        h_flex()
                            .gap_1()
                            .child(Label::new(audio.participant_name))
                            .child(
                                Label::new(format!(
                                    "{} · {}",
                                    audio.participant_id, audio.track_id
                                ))
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                            ),
                    )
                    .child(Label::new(status).color(color)),
            )
            .child(
                Label::new(format!(
                    "Loss {packet_loss} · jitter {:.1}ms · jitter buffer {jitter_buffer_delay}",
                    audio.jitter_ms
                ))
                .size(LabelSize::Small)
                .color(Color::Muted),
            )
            .child(
                Label::new(format!(
                    "WebRTC repaired {repaired_audio_duration} in {} {repair_event_label}",
                    audio.concealment_events,
                ))
                .size(LabelSize::Small)
                .color(Color::Muted),
            )
            .child(
                Label::new(format!(
                    "Local playback starved for {starved_audio_duration} · dropped {dropped_audio_duration} · buffered {buffered_audio_duration} (peak {peak_buffered_audio_duration})",
                ))
                .size(LabelSize::Small)
                .color(Color::Muted),
            )
    }

    fn render_metric_row<T: Copy + Clone>(
        &self,
        title: &str,
        description: &str,
        value: Option<T>,
        format_value: impl Fn(T) -> String,
        rate: impl Fn(T) -> (&'static str, Color),
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
