use gpui::{App, Window, actions};

actions!(
    dev,
    [
        DumpInputLatencyHistogram,
    ]
);

pub fn format_input_latency_report(_window: &Window, _cx: &mut App) -> String {
    String::new()
}

pub fn report_input_latency_telemetry(_window: &Window, _cx: &mut App) {}
