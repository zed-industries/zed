use gpui::{Context, Task, WeakEntity};
use livekit_client::ConnectionQuality;
use std::time::Duration;

use super::room::Room;

#[derive(Clone, Default)]
pub struct CallStats {
    pub connection_quality: Option<ConnectionQuality>,
    pub effective_quality: Option<ConnectionQuality>,
    pub latency_ms: Option<f64>,
    pub jitter_ms: Option<f64>,
    pub packet_loss_pct: Option<f64>,
    pub input_lag: Option<Duration>,
}

pub struct CallDiagnostics {
    stats: CallStats,
    room: WeakEntity<Room>,
    poll_task: Option<Task<()>>,
    stats_update_task: Option<Task<()>>,
}

impl CallDiagnostics {
    pub fn new(room: WeakEntity<Room>, cx: &mut Context<Self>) -> Self {
        let mut this = Self {
            stats: CallStats::default(),
            room,
            poll_task: None,
            stats_update_task: None,
        };
        this.start_polling(cx);
        this
    }

    pub fn stats(&self) -> &CallStats {
        &self.stats
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

    fn poll_stats(&mut self, cx: &mut Context<Self>) {
        let Some(room) = self.room.upgrade() else {
            return;
        };

        let connection_quality = room.read(cx).connection_quality();
        self.stats.connection_quality = Some(connection_quality);
        self.stats.input_lag = room.read(cx).input_lag();

        let stats_future = room.read(cx).get_stats(cx);

        let background_task = cx.background_executor().spawn(async move {
            let session_stats = stats_future.await;
            session_stats.map(|stats| compute_network_stats(&stats))
        });

        self.stats_update_task = Some(cx.spawn(async move |this, cx| {
            let result = background_task.await;
            this.update(cx, |this, cx| {
                if let Some(computed) = result {
                    this.stats.latency_ms = computed.latency_ms;
                    this.stats.jitter_ms = computed.jitter_ms;
                    this.stats.packet_loss_pct = computed.packet_loss_pct;
                }
                let quality = this
                    .stats
                    .connection_quality
                    .unwrap_or(ConnectionQuality::Lost);
                this.stats.effective_quality =
                    Some(effective_connection_quality(quality, &this.stats));
                cx.notify();
            })
            .ok();
        }));
    }
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
