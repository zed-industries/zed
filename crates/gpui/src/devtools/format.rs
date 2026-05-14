use super::{
    event_age,
    sources::{
        NotifyCause, NotifySourceKey, NotifySourceStats, RenderSourceKey, RenderSourceStats,
    },
};
use scheduler::Instant;
use std::time::Duration;

// Column widths must match `format_notify_source`.
pub(super) fn notify_column_header() -> String {
    format!(
        "{:<3} {:<18} {:<24} {:>4} {:>6} {:>7} {:>9} {}",
        "#", "source", "caller", "5s", "total", "age", "live", "id",
    )
}

pub(super) fn format_notify_source(
    index: usize,
    source: NotifySourceKey,
    stats: NotifySourceStats,
    total_count: usize,
    now: Instant,
) -> String {
    let caller = format!(
        "{}:{}:{}",
        file_name(source.caller_file),
        source.caller_line,
        stats.caller_column
    );
    let age = stats
        .last_timestamp
        .and_then(|timestamp| event_age(now, timestamp))
        .map(format_age)
        .unwrap_or_else(|| "--".to_string());
    let live = format!(
        "{}/{}",
        stats.live_window_count, stats.registered_window_count
    );
    format!(
        "{:<3} {:<18} {:<24} {:>4} {:>6} {:>7} {:>9} {}",
        index,
        short_type_name(source.entity_type),
        caller,
        stats.count,
        total_count,
        age,
        live,
        stats.entity_id.as_u64(),
    )
}

// Column widths must match `format_render_source`.
pub(super) fn render_column_header() -> String {
    format!(
        "{:<3} {:<28} {:<14} {:>4} {:>5} {:>7} {:>7} {:>8} {:>5}",
        "#", "view", "phase", "r/s", "reuse", "age", "avg", "sum", "miss",
    )
}

pub(super) fn format_render_source(
    index: usize,
    source: RenderSourceKey,
    stats: RenderSourceStats,
    now: Instant,
) -> String {
    let age = stats
        .last_timestamp
        .and_then(|timestamp| event_age(now, timestamp))
        .map(format_age)
        .unwrap_or_else(|| "--".to_string());
    let total = stats.count + stats.reuse_count;
    let miss = (stats.count * 100)
        .checked_div(total)
        .map(|percent| format!("{percent}%"))
        .unwrap_or_else(|| "--".to_string());
    let average = stats.average_duration();
    let average_cost = if average.is_zero() {
        "--".to_string()
    } else {
        format!("{}ms", format_duration_ms(average))
    };
    let total_cost = if stats.duration.is_zero() {
        "--".to_string()
    } else {
        format!("{}ms", format_duration_ms(stats.duration))
    };
    let view = format!(
        "{}#{}",
        short_type_name(source.entity_type),
        source.entity_id.as_u64()
    );
    let mut label = format!(
        "{:<3} {:<28} {:<14} {:>4} {:>5} {:>7} {:>7} {:>8} {:>5}",
        index,
        view,
        source.phase.as_str(),
        stats.count,
        stats.reuse_count,
        age,
        average_cost,
        total_cost,
        miss,
    );

    let chips = cache_miss_chips(stats);
    if !chips.is_empty() {
        label.push(' ');
        label.push_str(&chips);
    }
    if let Some(bounds) = stats.bounds {
        label.push_str(&format!(
            " {:.0}x{:.0}",
            bounds.size.width.0, bounds.size.height.0
        ));
    }
    if let Some(cause) = stats.cause {
        label.push(' ');
        label.push_str(&format_notify_cause(cause, now));
    }
    label
}

fn cache_miss_chips(stats: RenderSourceStats) -> String {
    let mut chips = stats
        .cache_miss_reasons
        .labels()
        .into_iter()
        .map(|label| format!("[{}]", label))
        .collect::<String>();
    if stats.caching_disabled_by_inspector {
        chips.push_str("[inspector]");
    }
    chips
}

pub(super) fn format_duration_ms(duration: Duration) -> String {
    format!("{:.1}", duration_ms(duration))
}

pub(super) fn format_age(duration: Duration) -> String {
    if duration < Duration::from_secs(1) {
        format!("{}ms", duration.as_millis())
    } else {
        format!("{:.1}s", duration.as_secs_f64())
    }
}

pub(super) fn format_notify_cause(cause: NotifyCause, now: Instant) -> String {
    let age = event_age(now, cause.timestamp)
        .map(format_age)
        .unwrap_or_else(|| "--".to_string());
    format!(
        "cause {}#{} {}:{}:{} {}",
        short_type_name(cause.source.entity_type),
        cause.entity_id.as_u64(),
        file_name(cause.source.caller_file),
        cause.source.caller_line,
        cause.caller_column,
        age,
    )
}

pub(super) fn short_type_name(type_name: &'static str) -> &'static str {
    type_name.rsplit("::").next().unwrap_or(type_name)
}

pub(super) fn file_name(path: &str) -> &str {
    path.rsplit('/').next().unwrap_or(path)
}

pub(super) fn truncate_chars(text: &str, max_chars: usize) -> String {
    let mut chars = text.chars();
    let truncated = chars.by_ref().take(max_chars).collect::<String>();
    if chars.next().is_some() {
        let mut truncated = truncated;
        let suffix = "...";
        for _ in 0..suffix.len().min(truncated.len()) {
            truncated.pop();
        }
        truncated.push_str(suffix);
        truncated
    } else {
        truncated
    }
}

fn duration_ms(duration: Duration) -> f64 {
    duration.as_secs_f64() * 1000.
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{EntityId, WindowId};

    use super::super::{
        events::{CacheMissReasons, NotifyEvent, ViewRenderEvent, ViewRenderPhase},
        sources::{
            NotifyCause, NotifySourceKey, NotifySourceStats, RenderSourceKey, RenderSourceStats,
        },
    };

    #[test]
    fn truncate_chars_reserves_room_for_suffix() {
        assert_eq!(
            truncate_chars("abcdefghijklmnopqrstuvwxyz", 10),
            "abcdefg..."
        );
        assert_eq!(truncate_chars("short", 10), "short");
    }

    #[test]
    fn notify_source_format_shows_last_age() {
        let now = Instant::now();
        let event = NotifyEvent {
            entity_id: EntityId::from(1),
            entity_type: "Editor",
            caller_file: "crates/editor/src/editor.rs",
            caller_line: 2111,
            caller_column: 17,
            registered_window_count: 2,
            live_window_count: 1,
            timestamp: now - Duration::from_millis(125),
        };
        let mut stats = NotifySourceStats::from_event(&event);
        stats.count = 3;

        let label = format_notify_source(1, NotifySourceKey::from(&event), stats, 9, now);
        assert!(
            label.starts_with("1   Editor"),
            "expected rank+type prefix, got: {label:?}"
        );
        assert!(label.contains("editor.rs:2111:17"));
        assert!(label.contains("125ms"));
        assert!(label.contains("1/2"));
        let trailing_id = format!(" {}", event.entity_id.as_u64());
        assert!(
            label.ends_with(&trailing_id),
            "expected trailing entity id, got: {label:?}"
        );

        // Column header lines up with data rows because they share format widths.
        let header = notify_column_header();
        let column_starts = |line: &str| -> Vec<usize> {
            line.match_indices(|c: char| !c.is_whitespace())
                .filter(|(i, _)| *i == 0 || line.as_bytes()[i - 1] == b' ')
                .map(|(i, _)| i)
                .collect()
        };
        let label_without_chips = label.split(" [").next().unwrap_or(label.as_str());
        assert_eq!(
            column_starts(&header).len(),
            column_starts(label_without_chips).len()
        );
    }

    #[test]
    fn render_source_format_shows_rate_age_reuse_ratio_and_chips() {
        let now = Instant::now();
        let mut reasons = CacheMissReasons::empty();
        reasons.insert_bounds_changed();
        reasons.insert_view_dirty();
        let event = ViewRenderEvent {
            window_id: WindowId::from(1),
            entity_id: EntityId::from(42),
            entity_type: "Editor",
            phase: ViewRenderPhase::UncachedRender,
            duration: Some(Duration::from_micros(1_200)),
            cache_miss_reasons: reasons,
            bounds: None,
            caching_disabled_by_inspector: true,
            timestamp: now - Duration::from_millis(25),
        };
        let mut stats = RenderSourceStats::from_event(&event);
        stats.count = 2;
        stats.duration = Duration::from_micros(2_400);
        stats.reuse_count = 6;

        let label = format_render_source(1, RenderSourceKey::from(&event), stats, now);
        assert!(
            label.starts_with("1   Editor#42"),
            "expected rank+view prefix, got: {label:?}"
        );
        assert!(label.contains("render"));
        assert!(label.contains("25ms"));
        assert!(label.contains("1.2ms"));
        assert!(label.contains("2.4ms"));
        assert!(label.contains("25%"));
        assert!(label.contains("[bounds][dirty][inspector]"));

        let header = render_column_header();
        let column_starts = |line: &str| -> Vec<usize> {
            line.match_indices(|c: char| !c.is_whitespace())
                .filter(|(i, _)| *i == 0 || line.as_bytes()[i - 1] == b' ')
                .map(|(i, _)| i)
                .collect()
        };
        let label_without_chips = label.split(" [").next().unwrap_or(label.as_str());
        assert_eq!(
            column_starts(&header).len(),
            column_starts(label_without_chips).len()
        );
    }

    #[test]
    fn render_source_format_shows_notify_cause() {
        let now = Instant::now();
        let event = ViewRenderEvent {
            window_id: WindowId::from(1),
            entity_id: EntityId::from(42),
            entity_type: "Dock",
            phase: ViewRenderPhase::UncachedRender,
            duration: None,
            cache_miss_reasons: CacheMissReasons::empty(),
            bounds: None,
            caching_disabled_by_inspector: false,
            timestamp: now,
        };
        let notify = NotifyEvent {
            entity_id: EntityId::from(7),
            entity_type: "TerminalView",
            caller_file: "crates/terminal/src/terminal_view.rs",
            caller_line: 1011,
            caller_column: 68,
            registered_window_count: 1,
            live_window_count: 1,
            timestamp: now - Duration::from_millis(25),
        };
        let mut stats = RenderSourceStats::from_event(&event);
        stats.count = 1;
        stats.cause = Some(NotifyCause::from_event(&notify));

        let label = format_render_source(1, RenderSourceKey::from(&event), stats, now);
        assert!(label.contains(&format!(
            "cause TerminalView#{} terminal_view.rs:1011:68 25ms",
            notify.entity_id.as_u64()
        )));
    }
}
