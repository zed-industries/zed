use gpui::{Context, Task, WeakEntity};
use livekit_client::{ConnectionQuality, RemoteAudioPlaybackStats};
use serde::{Serialize, Serializer};
use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
    time::{Duration, Instant},
};

use super::room::Room;

const MAX_HISTORY_SAMPLES: usize = 600;

#[derive(Clone, Default, Serialize)]
pub struct CallStats {
    #[serde(serialize_with = "serialize_connection_quality")]
    pub connection_quality: Option<ConnectionQuality>,
    #[serde(serialize_with = "serialize_connection_quality")]
    pub effective_quality: Option<ConnectionQuality>,
    pub latency_ms: Option<f64>,
    pub jitter_ms: Option<f64>,
    pub packet_loss_pct: Option<f64>,
    #[serde(
        rename = "input_lag_ms",
        serialize_with = "serialize_optional_duration_ms"
    )]
    pub input_lag: Option<Duration>,
}

#[derive(Clone, Default, Serialize)]
pub struct RemoteAudioDiagnostics {
    pub participant_id: String,
    #[serde(skip)]
    pub participant_name: String,
    pub track_id: String,
    pub packets_received: u64,
    pub packets_lost: i64,
    pub packet_loss_pct: Option<f64>,
    pub jitter_ms: f64,
    pub jitter_buffer_delay_ms: Option<f64>,
    pub concealed_samples: u64,
    pub concealment_events: u64,
    pub inserted_samples_for_deceleration: u64,
    pub removed_samples_for_acceleration: u64,
    pub frames_received: u64,
    pub frames_dropped: u64,
    pub queue_underflows: u64,
    pub current_queue_depth: u64,
    pub maximum_queue_depth: u64,
}

#[derive(Clone, Serialize)]
pub struct CallDiagnosticsSnapshot {
    #[serde(rename = "elapsed_ms", serialize_with = "serialize_duration_ms")]
    pub elapsed: Duration,
    pub stats: CallStats,
    pub remote_audio: Vec<RemoteAudioDiagnostics>,
}

#[derive(Serialize)]
pub struct CallDiagnosticsReport {
    schema_version: u32,
    samples: Vec<Arc<CallDiagnosticsSnapshot>>,
}

pub struct CallDiagnostics {
    stats: CallStats,
    history: VecDeque<Arc<CallDiagnosticsSnapshot>>,
    previous_inbound: HashMap<String, InboundCounters>,
    participant_ids: HashMap<u64, String>,
    track_ids: HashMap<String, String>,
    started_at: Instant,
    room: WeakEntity<Room>,
    poll_task: Option<Task<()>>,
}

impl CallDiagnostics {
    pub fn new(room: WeakEntity<Room>, cx: &mut Context<Self>) -> Self {
        let mut this = Self {
            stats: CallStats::default(),
            history: VecDeque::with_capacity(MAX_HISTORY_SAMPLES),
            previous_inbound: HashMap::default(),
            participant_ids: HashMap::default(),
            track_ids: HashMap::default(),
            started_at: Instant::now(),
            room,
            poll_task: None,
        };
        this.start_polling(cx);
        this
    }

    pub fn stats(&self) -> &CallStats {
        &self.stats
    }

    pub fn latest(&self) -> Option<&CallDiagnosticsSnapshot> {
        self.history.back().map(Arc::as_ref)
    }

    pub fn history(&self) -> &VecDeque<Arc<CallDiagnosticsSnapshot>> {
        &self.history
    }

    pub fn report(&self) -> CallDiagnosticsReport {
        CallDiagnosticsReport {
            schema_version: 1,
            samples: self.history.iter().cloned().collect(),
        }
    }

    fn start_polling(&mut self, cx: &mut Context<Self>) {
        self.poll_task = Some(cx.spawn(async move |this, cx| {
            loop {
                let poll_task = match this.update(cx, |this, cx| this.poll_stats(cx)) {
                    Ok(Some(poll_task)) => poll_task,
                    _ => break,
                };
                let result = poll_task.await;
                if this
                    .update(cx, |this, cx| this.apply_poll_result(result, cx))
                    .is_err()
                {
                    break;
                }
                cx.background_executor().timer(Duration::from_secs(1)).await;
            }
        }));
    }

    fn poll_stats(&mut self, cx: &mut Context<Self>) -> Option<Task<PollResult>> {
        let room = self.room.upgrade()?;
        let connection_quality = room.read(cx).connection_quality();
        let input_lag = room.read(cx).input_lag();
        let stats_future = room.read(cx).get_stats(cx);

        let mut remote_tracks = Vec::new();
        for (user_id, participant) in room.read(cx).remote_participants() {
            let next_participant_number = self.participant_ids.len() + 1;
            let participant_id = self
                .participant_ids
                .entry(*user_id)
                .or_insert_with(|| format!("participant-{next_participant_number}"))
                .clone();
            for (track_id, (track, stream)) in &participant.audio_tracks {
                let track_id = track_id.to_string();
                let next_track_number = self.track_ids.len() + 1;
                let track_id = self
                    .track_ids
                    .entry(track_id)
                    .or_insert_with(|| format!("audio-track-{next_track_number}"))
                    .clone();
                remote_tracks.push(TrackContext {
                    participant_id: participant_id.clone(),
                    participant_name: participant.user.username.to_string(),
                    track_id,
                    rtc_track_id: track.rtc_track_id(),
                    playback: stream.remote_playback_stats().unwrap_or_default(),
                });
            }
        }

        let previous_inbound = std::mem::take(&mut self.previous_inbound);
        let started_at = self.started_at;
        Some(cx.background_executor().spawn(async move {
            let Some(session_stats) = stats_future.await else {
                return PollResult {
                    snapshot: None,
                    previous_inbound,
                };
            };
            let computed = compute_network_stats(&session_stats);
            let (remote_audio, previous_inbound) =
                compute_remote_audio_stats(&session_stats, &remote_tracks, previous_inbound);
            let mut stats = CallStats {
                connection_quality: Some(connection_quality),
                effective_quality: None,
                latency_ms: computed.latency_ms,
                jitter_ms: computed.jitter_ms,
                packet_loss_pct: computed.packet_loss_pct,
                input_lag,
            };
            stats.effective_quality =
                Some(effective_connection_quality(connection_quality, &stats));
            PollResult {
                snapshot: Some(CallDiagnosticsSnapshot {
                    elapsed: started_at.elapsed(),
                    stats,
                    remote_audio,
                }),
                previous_inbound,
            }
        }))
    }

    fn apply_poll_result(&mut self, result: PollResult, cx: &mut Context<Self>) {
        self.previous_inbound = result.previous_inbound;
        let Some(snapshot) = result.snapshot else {
            return;
        };
        self.stats = snapshot.stats.clone();
        self.history.push_back(Arc::new(snapshot));
        if self.history.len() > MAX_HISTORY_SAMPLES {
            self.history.pop_front();
        }
        cx.notify();
    }
}

fn serialize_connection_quality<S>(
    quality: &Option<ConnectionQuality>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    quality
        .map(|quality| match quality {
            ConnectionQuality::Excellent => "excellent",
            ConnectionQuality::Good => "good",
            ConnectionQuality::Poor => "poor",
            ConnectionQuality::Lost => "lost",
        })
        .serialize(serializer)
}

fn serialize_duration_ms<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    duration.as_millis().serialize(serializer)
}

fn serialize_optional_duration_ms<S>(
    duration: &Option<Duration>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    duration
        .map(|duration| duration.as_secs_f64() * 1000.0)
        .serialize(serializer)
}

#[cfg_attr(any(test, feature = "test-support"), allow(dead_code))]
struct TrackContext {
    participant_id: String,
    participant_name: String,
    track_id: String,
    rtc_track_id: String,
    playback: RemoteAudioPlaybackStats,
}

struct PollResult {
    snapshot: Option<CallDiagnosticsSnapshot>,
    previous_inbound: HashMap<String, InboundCounters>,
}

#[derive(Clone, Copy)]
#[cfg_attr(any(test, feature = "test-support"), allow(dead_code))]
struct InboundCounters {
    packets_received: u64,
    packets_lost: i64,
    jitter_buffer_delay: f64,
    jitter_buffer_emitted_count: u64,
    concealed_samples: u64,
    concealment_events: u64,
    inserted_samples_for_deceleration: u64,
    removed_samples_for_acceleration: u64,
    playback: RemoteAudioPlaybackStats,
}

struct ComputedNetworkStats {
    latency_ms: Option<f64>,
    jitter_ms: Option<f64>,
    packet_loss_pct: Option<f64>,
}

fn compute_network_stats(stats: &livekit_client::SessionStats) -> ComputedNetworkStats {
    let mut min_rtt: Option<f64> = None;
    let mut max_jitter: Option<f64> = None;
    let mut total_packets_received: u64 = 0;
    let mut total_packets_lost: i64 = 0;

    let all_stats = stats
        .publisher_stats
        .iter()
        .chain(stats.subscriber_stats.iter());

    for stat in all_stats {
        extract_metrics(
            stat,
            &mut min_rtt,
            &mut max_jitter,
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
        latency_ms: min_rtt.map(|rtt| rtt * 1000.0),
        jitter_ms: max_jitter.map(|j| j * 1000.0),
        packet_loss_pct,
    }
}

fn counter_delta(current: u64, previous: u64) -> u64 {
    current.checked_sub(previous).unwrap_or_default()
}

#[cfg_attr(any(test, feature = "test-support"), allow(dead_code))]
fn signed_counter_delta(current: i64, previous: i64) -> i64 {
    current.checked_sub(previous).unwrap_or_default()
}

fn interval_packet_loss_pct(packets_received: u64, packets_lost: i64) -> Option<f64> {
    let packets_lost = packets_lost.max(0);
    let expected_packets = packets_received as i128 + i128::from(packets_lost);
    if expected_packets > 0 {
        Some((packets_lost as f64 / expected_packets as f64) * 100.0)
    } else {
        None
    }
}

fn average_jitter_buffer_delay_ms(
    current_delay: f64,
    previous_delay: f64,
    emitted_count: u64,
) -> Option<f64> {
    let delay = current_delay - previous_delay;
    if emitted_count > 0 && delay >= 0.0 {
        Some((delay / emitted_count as f64) * 1000.0)
    } else {
        None
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
fn compute_remote_audio_stats(
    _stats: &livekit_client::SessionStats,
    _remote_tracks: &[TrackContext],
    previous_inbound: HashMap<String, InboundCounters>,
) -> (
    Vec<RemoteAudioDiagnostics>,
    HashMap<String, InboundCounters>,
) {
    (Vec::new(), previous_inbound)
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
fn compute_remote_audio_stats(
    stats: &livekit_client::SessionStats,
    remote_tracks: &[TrackContext],
    previous_inbound: HashMap<String, InboundCounters>,
) -> (
    Vec<RemoteAudioDiagnostics>,
    HashMap<String, InboundCounters>,
) {
    use livekit_client::RtcStats;

    let mut diagnostics = Vec::new();
    let mut next_inbound = HashMap::default();
    for stat in &stats.subscriber_stats {
        let RtcStats::InboundRtp(inbound) = stat else {
            continue;
        };
        if inbound.stream.kind != "audio" {
            continue;
        }

        let Some(track) = remote_tracks
            .iter()
            .find(|track| track.rtc_track_id == inbound.inbound.track_identifier)
        else {
            continue;
        };

        let current = InboundCounters {
            packets_received: inbound.received.packets_received,
            packets_lost: inbound.received.packets_lost,
            jitter_buffer_delay: inbound.inbound.jitter_buffer_delay,
            jitter_buffer_emitted_count: inbound.inbound.jitter_buffer_emitted_count,
            concealed_samples: inbound.inbound.concealed_samples,
            concealment_events: inbound.inbound.concealment_events,
            inserted_samples_for_deceleration: inbound.inbound.inserted_samples_for_deceleration,
            removed_samples_for_acceleration: inbound.inbound.removed_samples_for_acceleration,
            playback: track.playback,
        };
        let previous = previous_inbound
            .get(&inbound.rtc.id)
            .copied()
            .unwrap_or(current);
        next_inbound.insert(inbound.rtc.id.clone(), current);

        let packets_received = counter_delta(current.packets_received, previous.packets_received);
        let packets_lost = signed_counter_delta(current.packets_lost, previous.packets_lost);
        let packet_loss_pct = interval_packet_loss_pct(packets_received, packets_lost);

        let emitted_count = counter_delta(
            current.jitter_buffer_emitted_count,
            previous.jitter_buffer_emitted_count,
        );
        let jitter_buffer_delay_ms = average_jitter_buffer_delay_ms(
            current.jitter_buffer_delay,
            previous.jitter_buffer_delay,
            emitted_count,
        );

        diagnostics.push(RemoteAudioDiagnostics {
            participant_id: track.participant_id.clone(),
            participant_name: track.participant_name.clone(),
            track_id: track.track_id.clone(),
            packets_received,
            packets_lost,
            packet_loss_pct,
            jitter_ms: inbound.received.jitter * 1000.0,
            jitter_buffer_delay_ms,
            concealed_samples: counter_delta(current.concealed_samples, previous.concealed_samples),
            concealment_events: counter_delta(
                current.concealment_events,
                previous.concealment_events,
            ),
            inserted_samples_for_deceleration: counter_delta(
                current.inserted_samples_for_deceleration,
                previous.inserted_samples_for_deceleration,
            ),
            removed_samples_for_acceleration: counter_delta(
                current.removed_samples_for_acceleration,
                previous.removed_samples_for_acceleration,
            ),
            frames_received: counter_delta(
                current.playback.frames_received,
                previous.playback.frames_received,
            ),
            frames_dropped: counter_delta(
                current.playback.frames_dropped,
                previous.playback.frames_dropped,
            ),
            queue_underflows: counter_delta(
                current.playback.queue_underflows,
                previous.playback.queue_underflows,
            ),
            current_queue_depth: current.playback.current_queue_depth,
            maximum_queue_depth: current.playback.maximum_queue_depth,
        });
    }

    (diagnostics, next_inbound)
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
    _min_rtt: &mut Option<f64>,
    _max_jitter: &mut Option<f64>,
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
    min_rtt: &mut Option<f64>,
    max_jitter: &mut Option<f64>,
    total_packets_received: &mut u64,
    total_packets_lost: &mut i64,
) {
    use livekit_client::RtcStats;

    match stat {
        RtcStats::CandidatePair(pair) => {
            let rtt = pair.candidate_pair.current_round_trip_time;
            if rtt > 0.0 {
                *min_rtt = Some(match *min_rtt {
                    Some(current) => current.min(rtt),
                    None => rtt,
                });
            }
        }
        RtcStats::InboundRtp(inbound) => {
            let jitter = inbound.received.jitter;
            if jitter > 0.0 {
                *max_jitter = Some(match *max_jitter {
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
                *min_rtt = Some(match *min_rtt {
                    Some(current) => current.min(rtt),
                    None => rtt,
                });
            }
        }
        _ => {}
    }
}

fn metric_quality(value: f64, warn_threshold: f64, error_threshold: f64) -> ConnectionQuality {
    if value < warn_threshold {
        ConnectionQuality::Excellent
    } else if value < error_threshold {
        ConnectionQuality::Poor
    } else {
        ConnectionQuality::Lost
    }
}

/// Computes the effective connection quality by taking the worst of the
/// LiveKit-reported quality and each individual metric rating.
fn effective_connection_quality(
    livekit_quality: ConnectionQuality,
    stats: &CallStats,
) -> ConnectionQuality {
    let mut worst = livekit_quality;

    if let Some(latency) = stats.latency_ms {
        worst = worst.max(metric_quality(latency, 100.0, 300.0));
    }
    if let Some(jitter) = stats.jitter_ms {
        worst = worst.max(metric_quality(jitter, 30.0, 75.0));
    }
    if let Some(loss) = stats.packet_loss_pct {
        worst = worst.max(metric_quality(loss, 1.0, 5.0));
    }
    if let Some(lag) = stats.input_lag {
        let lag_ms = lag.as_secs_f64() * 1000.0;
        worst = worst.max(metric_quality(lag_ms, 20.0, 50.0));
    }

    worst
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counter_delta_ignores_counter_resets() {
        assert_eq!(counter_delta(5, 10), 0);
        assert_eq!(counter_delta(15, 10), 5);
    }

    #[test]
    fn packet_loss_ignores_late_packet_recovery() {
        assert_eq!(interval_packet_loss_pct(100, -1), Some(0.0));
        assert_eq!(interval_packet_loss_pct(95, 5), Some(5.0));
    }

    #[test]
    fn jitter_buffer_delay_ignores_counter_resets() {
        assert_eq!(average_jitter_buffer_delay_ms(1.5, 1.0, 100), Some(5.0));
        assert_eq!(average_jitter_buffer_delay_ms(0.5, 1.0, 100), None);
        assert_eq!(average_jitter_buffer_delay_ms(1.5, 1.0, 0), None);
    }

    #[test]
    fn report_redacts_participant_names() -> anyhow::Result<()> {
        let report = CallDiagnosticsReport {
            schema_version: 1,
            samples: vec![Arc::new(CallDiagnosticsSnapshot {
                elapsed: Duration::from_secs(1),
                stats: CallStats::default(),
                remote_audio: vec![RemoteAudioDiagnostics {
                    participant_id: "participant-1".to_string(),
                    participant_name: "private-username".to_string(),
                    track_id: "audio-track-1".to_string(),
                    ..Default::default()
                }],
            })],
        };

        let serialized = serde_json::to_string(&report)?;
        assert!(!serialized.contains("private-username"));
        assert!(serialized.contains("participant-1"));
        assert!(serialized.contains("audio-track-1"));
        Ok(())
    }
}
