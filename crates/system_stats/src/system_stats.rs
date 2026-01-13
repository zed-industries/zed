use gpui::Task;
use settings::Settings;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use sysinfo::{Networks, System};
use ui::{
    div, h_flex, Button, ButtonCommon, Context, IntoElement,
    LabelSize, ParentElement, Render, SharedString, Styled, Tooltip, Window,
};
use workspace::{StatusBarSettings, StatusItemView, item::ItemHandle};

const UPDATE_INTERVAL: Duration = Duration::from_secs(2);

pub struct SystemStatsIndicator {
    cpu_usage: f32,
    memory_used: u64,
    memory_total: u64,
    network_rx_speed: u64,
    network_tx_speed: u64,
    system: Arc<Mutex<System>>,
    networks: Arc<Mutex<Networks>>,
    last_rx_bytes: u64,
    last_tx_bytes: u64,
    _update_task: Task<()>,
}

impl SystemStatsIndicator {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let system = Arc::new(Mutex::new(System::new_all()));
        let networks = Arc::new(Mutex::new(Networks::new_with_refreshed_list()));

        let (memory_used, memory_total, cpu_usage) = {
            let mut sys = system.lock().unwrap();
            sys.refresh_all();
            (sys.used_memory(), sys.total_memory(), sys.global_cpu_usage())
        };

        let (last_rx_bytes, last_tx_bytes) = {
            let nets = networks.lock().unwrap();
            let mut rx = 0u64;
            let mut tx = 0u64;
            for (_, data) in nets.iter() {
                rx += data.total_received();
                tx += data.total_transmitted();
            }
            (rx, tx)
        };

        let mut this = Self {
            cpu_usage,
            memory_used,
            memory_total,
            network_rx_speed: 0,
            network_tx_speed: 0,
            system,
            networks,
            last_rx_bytes,
            last_tx_bytes,
            _update_task: Task::ready(()),
        };

        this.start_update_task(window, cx);
        this
    }

    fn start_update_task(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let system = self.system.clone();
        let networks = self.networks.clone();

        self._update_task = cx.spawn_in(window, async move |this, cx| {
            loop {
                cx.background_executor().timer(UPDATE_INTERVAL).await;

                let (cpu_usage, memory_used, memory_total) = {
                    let mut sys = system.lock().unwrap();
                    sys.refresh_cpu_usage();
                    sys.refresh_memory();
                    (sys.global_cpu_usage(), sys.used_memory(), sys.total_memory())
                };

                let (current_rx, current_tx) = {
                    let mut nets = networks.lock().unwrap();
                    nets.refresh(true);
                    let mut rx = 0u64;
                    let mut tx = 0u64;
                    for (_, data) in nets.iter() {
                        rx += data.total_received();
                        tx += data.total_transmitted();
                    }
                    (rx, tx)
                };

                let update_result = this.update(cx, |this, cx| {
                    let rx_diff = current_rx.saturating_sub(this.last_rx_bytes);
                    let tx_diff = current_tx.saturating_sub(this.last_tx_bytes);

                    this.network_rx_speed = rx_diff / UPDATE_INTERVAL.as_secs();
                    this.network_tx_speed = tx_diff / UPDATE_INTERVAL.as_secs();
                    this.last_rx_bytes = current_rx;
                    this.last_tx_bytes = current_tx;

                    this.cpu_usage = cpu_usage;
                    this.memory_used = memory_used;
                    this.memory_total = memory_total;
                    cx.notify();
                });

                if update_result.is_err() {
                    break;
                }
            }
        });
    }

    fn format_speed(bytes_per_sec: u64) -> String {
        const KB: u64 = 1024;
        const MB: u64 = KB * 1024;

        if bytes_per_sec >= MB {
            format!("{:.1} MB/s", bytes_per_sec as f64 / MB as f64)
        } else if bytes_per_sec >= KB {
            format!("{:.0} KB/s", bytes_per_sec as f64 / KB as f64)
        } else {
            format!("{} B/s", bytes_per_sec)
        }
    }

    fn format_memory_gb(bytes: u64) -> String {
        const GB: u64 = 1024 * 1024 * 1024;
        format!("{:.2}", bytes as f64 / GB as f64)
    }
}

impl Render for SystemStatsIndicator {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !StatusBarSettings::get_global(cx).system_stats_button {
            return div().into_any_element();
        }

        let memory_percent = if self.memory_total > 0 {
            (self.memory_used as f64 / self.memory_total as f64 * 100.0) as u32
        } else {
            0
        };

        let cpu_text: SharedString = format!("🖥 {:.0}%", self.cpu_usage).into();
        let memory_text: SharedString = format!(
            "💾 {}/{} GB, {}%",
            Self::format_memory_gb(self.memory_used),
            Self::format_memory_gb(self.memory_total),
            memory_percent
        ).into();
        let network_text: SharedString = format!(
            "🌐 ↑{} ↓{}",
            Self::format_speed(self.network_tx_speed),
            Self::format_speed(self.network_rx_speed)
        ).into();

        h_flex()
            .gap_3()
            .child(
                Button::new("cpu-stats", cpu_text)
                    .label_size(LabelSize::Small)
                    .tooltip(|window, cx| Tooltip::text("CPU Usage")(window, cx))
            )
            .child(
                Button::new("memory-stats", memory_text)
                    .label_size(LabelSize::Small)
                    .tooltip(|window, cx| Tooltip::text("Memory Usage")(window, cx))
            )
            .child(
                Button::new("network-stats", network_text)
                    .label_size(LabelSize::Small)
                    .tooltip(|window, cx| Tooltip::text("Network: Upload / Download")(window, cx))
            )
            .into_any_element()
    }
}

impl StatusItemView for SystemStatsIndicator {
    fn set_active_pane_item(
        &mut self,
        _active_pane_item: Option<&dyn ItemHandle>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }
}
