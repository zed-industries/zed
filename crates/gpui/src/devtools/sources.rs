use super::{
    ANIMATION_EXPIRY, FRAME_RATE_WINDOW, SOURCE_WINDOW, TOP_SOURCE_COUNT,
    events::{
        AnimationEventKind, CacheMissReasons, DirtyPathEvent, NotifyEvent, ViewRenderEvent,
        ViewRenderPhase,
    },
    state::GpuiDevTools,
};
use crate::{Bounds, EntityId, Pixels, WindowId};
use collections::{FxHashMap, FxHashSet};
use scheduler::Instant;
use std::time::Duration;

#[derive(Clone, Debug)]
pub(super) struct PinnedNotifySource {
    entity_type: String,
    caller_file: String,
    caller_line: u32,
}

impl PinnedNotifySource {
    pub(super) fn matches(&self, event: &NotifyEvent) -> bool {
        event.caller_line == self.caller_line
            && (event.entity_type == self.entity_type
                || short_type_name(event.entity_type) == self.entity_type)
            && (event.caller_file.ends_with(&self.caller_file)
                || file_name(event.caller_file) == self.caller_file)
    }
}

pub(super) fn parse_pinned_notify_source(source: &str) -> Option<PinnedNotifySource> {
    let source = source.trim();
    if source.is_empty()
        || source.eq_ignore_ascii_case("none")
        || source.eq_ignore_ascii_case("off")
    {
        return None;
    }

    let source = source.replace([',', ':'], " ");
    let mut parts = source.split_whitespace();
    let entity_type = parts.next()?.to_string();
    let caller_file = parts.next()?.to_string();
    let caller_line = parts.next()?.parse().ok()?;

    Some(PinnedNotifySource {
        entity_type,
        caller_file,
        caller_line,
    })
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub(super) struct NotifySourceKey {
    pub(super) entity_type: &'static str,
    pub(super) caller_file: &'static str,
    pub(super) caller_line: u32,
}

impl NotifySourceKey {
    pub(super) fn from(event: &NotifyEvent) -> Self {
        Self {
            entity_type: event.entity_type,
            caller_file: event.caller_file,
            caller_line: event.caller_line,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) struct NotifySourceStats {
    count: usize,
    entity_id: EntityId,
    caller_column: u32,
    registered_window_count: usize,
    live_window_count: usize,
    last_timestamp: Option<Instant>,
}

impl NotifySourceStats {
    pub(super) fn from_event(event: &NotifyEvent) -> Self {
        Self {
            count: 0,
            entity_id: event.entity_id,
            caller_column: event.caller_column,
            registered_window_count: event.registered_window_count,
            live_window_count: event.live_window_count,
            last_timestamp: Some(event.timestamp),
        }
    }

    fn empty() -> Self {
        Self {
            count: 0,
            entity_id: EntityId::from(0),
            caller_column: 0,
            registered_window_count: 0,
            live_window_count: 0,
            last_timestamp: None,
        }
    }

    fn update_from_event(&mut self, event: &NotifyEvent) {
        self.entity_id = event.entity_id;
        self.caller_column = event.caller_column;
        self.registered_window_count = event.registered_window_count;
        self.live_window_count = event.live_window_count;
        self.last_timestamp = Some(event.timestamp);
    }
}

pub(super) fn top_notify_sources(
    devtools: &GpuiDevTools,
    now: Instant,
    limit: usize,
) -> Vec<(NotifySourceKey, NotifySourceStats)> {
    let mut counts = FxHashMap::default();
    for event in devtools.notifications.iter() {
        let Some(age) = event_age(now, event.timestamp) else {
            continue;
        };
        if age > SOURCE_WINDOW {
            continue;
        }

        let key = NotifySourceKey::from(event);
        if devtools.hidden_notify_sources.contains(&key) {
            continue;
        }

        let stats = counts
            .entry(key)
            .or_insert_with(|| NotifySourceStats::from_event(event));
        stats.count += 1;
        stats.update_from_event(event);
    }

    for source in &devtools.pinned_notify_sources {
        if devtools.hidden_notify_sources.contains(source) {
            continue;
        }

        counts.entry(*source).or_insert_with(|| {
            devtools
                .notify_source_last_stats(*source)
                .unwrap_or_else(NotifySourceStats::empty)
        });
    }

    let mut counts = counts.into_iter().collect::<Vec<_>>();
    counts.sort_by(|(left_source, left), (right_source, right)| {
        let left_pinned = devtools.pinned_notify_sources.contains(left_source);
        let right_pinned = devtools.pinned_notify_sources.contains(right_source);
        right_pinned
            .cmp(&left_pinned)
            .then_with(|| right.count.cmp(&left.count))
            .then_with(|| {
                short_type_name(left_source.entity_type)
                    .cmp(short_type_name(right_source.entity_type))
            })
            .then_with(|| left_source.caller_file.cmp(right_source.caller_file))
            .then_with(|| left_source.caller_line.cmp(&right_source.caller_line))
    });
    let visible_pinned_count = counts
        .iter()
        .filter(|(source, _)| devtools.pinned_notify_sources.contains(source))
        .count();
    counts.truncate(limit + visible_pinned_count);
    counts
}

pub(super) fn hidden_notify_sources(
    devtools: &GpuiDevTools,
    now: Instant,
) -> Vec<(NotifySourceKey, usize)> {
    let mut counts = devtools
        .hidden_notify_sources
        .iter()
        .copied()
        .map(|source| (source, 0))
        .collect::<FxHashMap<_, _>>();

    for event in devtools.notifications.iter() {
        let Some(age) = event_age(now, event.timestamp) else {
            continue;
        };
        if age > SOURCE_WINDOW {
            continue;
        }

        let key = NotifySourceKey::from(event);
        if let Some(count) = counts.get_mut(&key) {
            *count += 1;
        }
    }

    let mut counts = counts.into_iter().collect::<Vec<_>>();
    counts.sort_by(|(left_source, left_count), (right_source, right_count)| {
        right_count
            .cmp(left_count)
            .then_with(|| {
                short_type_name(left_source.entity_type)
                    .cmp(short_type_name(right_source.entity_type))
            })
            .then_with(|| left_source.caller_file.cmp(right_source.caller_file))
            .then_with(|| left_source.caller_line.cmp(&right_source.caller_line))
    });
    counts
}

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

pub(super) fn top_dirty_path(
    devtools: &GpuiDevTools,
    window_id: WindowId,
    now: Instant,
) -> Option<(String, usize)> {
    let mut counts = FxHashMap::default();
    for event in devtools.dirty_paths.iter() {
        let Some(age) = event_age(now, event.timestamp) else {
            continue;
        };
        if event.window_id != window_id || age > SOURCE_WINDOW {
            continue;
        }

        *counts.entry(dirty_path_label(event)).or_insert(0) += 1;
    }
    counts.into_iter().max_by_key(|(_, count)| *count)
}

#[derive(Default)]
pub(super) struct RenderSummary {
    pub(super) render_count: usize,
    pub(super) reuse_count: usize,
    pub(super) top_sources: Vec<(RenderSourceKey, RenderSourceStats)>,
}

pub(super) fn render_summary(
    devtools: &GpuiDevTools,
    window_id: WindowId,
    now: Instant,
) -> RenderSummary {
    let mut summary = RenderSummary::default();
    let mut counts: FxHashMap<RenderSourceKey, RenderSourceStats> = FxHashMap::default();
    let mut reuse_counts_by_type: FxHashMap<&'static str, usize> = FxHashMap::default();

    for event in devtools.renders.iter() {
        let Some(age) = event_age(now, event.timestamp) else {
            continue;
        };
        if event.window_id != window_id || age > FRAME_RATE_WINDOW {
            continue;
        }

        if event.phase.is_reuse() {
            summary.reuse_count += 1;
            *reuse_counts_by_type.entry(event.entity_type).or_insert(0) += 1;
        } else if event.phase.flashes() {
            let key = RenderSourceKey::from(event);
            if devtools.hidden_render_sources.contains(&key) {
                continue;
            }

            summary.render_count += 1;

            counts
                .entry(key)
                .or_insert_with(|| RenderSourceStats::from_event(event))
                .record_event(event);
        }
    }

    for source in &devtools.pinned_render_sources {
        if devtools.hidden_render_sources.contains(source) {
            continue;
        }

        counts.entry(*source).or_insert_with(|| {
            devtools
                .render_source_last_stats(*source)
                .unwrap_or_else(RenderSourceStats::empty)
        });
    }

    for (source, stats) in &mut counts {
        stats.reuse_count = reuse_counts_by_type
            .get(&source.entity_type)
            .copied()
            .unwrap_or(0);
    }

    let mut counts = counts.into_iter().collect::<Vec<_>>();
    counts.sort_by(|(left_source, left), (right_source, right)| {
        let left_pinned = devtools.pinned_render_sources.contains(left_source);
        let right_pinned = devtools.pinned_render_sources.contains(right_source);
        right_pinned
            .cmp(&left_pinned)
            .then_with(|| right.count.cmp(&left.count))
            .then_with(|| {
                short_type_name(left_source.entity_type)
                    .cmp(short_type_name(right_source.entity_type))
            })
            .then_with(|| left_source.phase.as_str().cmp(right_source.phase.as_str()))
    });
    let visible_pinned_count = counts
        .iter()
        .filter(|(source, _)| devtools.pinned_render_sources.contains(source))
        .count();
    counts.truncate(TOP_SOURCE_COUNT + visible_pinned_count);
    summary.top_sources = counts;

    summary
}

pub(super) fn hidden_render_sources(
    devtools: &GpuiDevTools,
    window_id: WindowId,
    now: Instant,
) -> Vec<(RenderSourceKey, usize)> {
    let mut counts = devtools
        .hidden_render_sources
        .iter()
        .copied()
        .map(|source| (source, 0))
        .collect::<FxHashMap<_, _>>();

    for event in devtools.renders.iter() {
        let Some(age) = event_age(now, event.timestamp) else {
            continue;
        };
        if event.window_id != window_id || age > FRAME_RATE_WINDOW {
            continue;
        }

        let key = RenderSourceKey::from(event);
        if let Some(count) = counts.get_mut(&key) {
            *count += 1;
        }
    }

    let mut counts = counts.into_iter().collect::<Vec<_>>();
    counts.sort_by(|(left_source, left_count), (right_source, right_count)| {
        right_count
            .cmp(left_count)
            .then_with(|| {
                short_type_name(left_source.entity_type)
                    .cmp(short_type_name(right_source.entity_type))
            })
            .then_with(|| left_source.phase.as_str().cmp(right_source.phase.as_str()))
    });
    counts
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub(super) struct RenderSourceKey {
    pub(super) entity_type: &'static str,
    pub(super) phase: ViewRenderPhase,
}

impl RenderSourceKey {
    pub(super) fn from(event: &ViewRenderEvent) -> Self {
        Self {
            entity_type: event.entity_type,
            phase: event.phase,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) struct RenderSourceStats {
    count: usize,
    reuse_count: usize,
    duration: Duration,
    sample_entity_id: EntityId,
    bounds: Option<Bounds<Pixels>>,
    cache_miss_reasons: CacheMissReasons,
    caching_disabled_by_inspector: bool,
    last_timestamp: Option<Instant>,
}

impl RenderSourceStats {
    pub(super) fn from_event(event: &ViewRenderEvent) -> Self {
        Self {
            count: 0,
            reuse_count: 0,
            duration: Duration::default(),
            sample_entity_id: event.entity_id,
            bounds: event.bounds,
            cache_miss_reasons: event.cache_miss_reasons,
            caching_disabled_by_inspector: event.caching_disabled_by_inspector,
            last_timestamp: Some(event.timestamp),
        }
    }

    fn empty() -> Self {
        Self {
            count: 0,
            reuse_count: 0,
            duration: Duration::default(),
            sample_entity_id: EntityId::from(0),
            bounds: None,
            cache_miss_reasons: CacheMissReasons::empty(),
            caching_disabled_by_inspector: false,
            last_timestamp: None,
        }
    }

    fn record_event(&mut self, event: &ViewRenderEvent) {
        self.count += 1;
        if let Some(duration) = event.duration {
            self.duration += duration;
        }
        self.sample_entity_id = event.entity_id;
        self.bounds = event.bounds;
        self.cache_miss_reasons = event.cache_miss_reasons;
        self.caching_disabled_by_inspector = event.caching_disabled_by_inspector;
        self.last_timestamp = Some(event.timestamp);
    }
}

// Column widths must match `format_render_source`.
pub(super) fn render_column_header() -> String {
    format!(
        "{:<3} {:<28} {:<14} {:>4} {:>5} {:>7} {:>8} {:>5}",
        "#", "view", "phase", "r/s", "reuse", "age", "cost", "miss",
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
    let cost = if stats.duration.is_zero() {
        "--".to_string()
    } else {
        format!("{}ms", format_duration_ms(stats.duration))
    };
    let view = format!(
        "{}#{}",
        short_type_name(source.entity_type),
        stats.sample_entity_id.as_u64()
    );
    let mut label = format!(
        "{:<3} {:<28} {:<14} {:>4} {:>5} {:>7} {:>8} {:>5}",
        index,
        view,
        source.phase.as_str(),
        stats.count,
        stats.reuse_count,
        age,
        cost,
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

pub(super) fn active_animation_count(
    devtools: &GpuiDevTools,
    window_id: WindowId,
    now: Instant,
) -> usize {
    let mut sources = FxHashSet::default();
    for event in devtools.animations.iter() {
        let Some(age) = event_age(now, event.timestamp) else {
            continue;
        };
        if event.window_id != window_id || age > ANIMATION_EXPIRY {
            continue;
        }

        match &event.kind {
            AnimationEventKind::FrameRequest {
                caller_file,
                caller_line,
                caller_column,
            } => {
                if caller_file.ends_with("elements/animation.rs") {
                    continue;
                }

                sources.insert(format!(
                    "frame:{}:{}:{}:{}",
                    short_type_name(event.entity_type),
                    file_name(caller_file),
                    caller_line,
                    caller_column
                ));
            }
            AnimationEventKind::ElementTick {
                element_id,
                animation_index,
                duration,
                repeats,
            } => {
                if *repeats {
                    sources.insert(format!(
                        "element:{}:{}:{}:{:.0}",
                        event.entity_id.as_u64(),
                        element_id,
                        animation_index,
                        duration_ms(*duration)
                    ));
                }
            }
        }
    }
    sources.len()
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

fn dirty_path_label(event: &DirtyPathEvent) -> String {
    if event.path.is_empty() {
        return format!(
            "{}#{} no path",
            short_type_name(event.invalidated_entity_type),
            event.invalidated_entity_id.as_u64()
        );
    }

    let mut path = event
        .path
        .iter()
        .take(3)
        .map(|segment| {
            format!(
                "{}#{}",
                short_type_name(segment.entity_type),
                segment.entity_id.as_u64()
            )
        })
        .collect::<Vec<_>>()
        .join("<");
    if event.path.len() > 3 {
        path.push_str("<...");
    }
    format!(
        "{}#{} {}",
        short_type_name(event.invalidated_entity_type),
        event.invalidated_entity_id.as_u64(),
        path
    )
}

fn duration_ms(duration: Duration) -> f64 {
    duration.as_secs_f64() * 1000.
}

fn event_age(now: Instant, timestamp: Instant) -> Option<Duration> {
    if timestamp > now {
        None
    } else {
        Some(now.duration_since(timestamp))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncate_chars_reserves_room_for_suffix() {
        assert_eq!(
            truncate_chars("abcdefghijklmnopqrstuvwxyz", 10),
            "abcdefg..."
        );
        assert_eq!(truncate_chars("short", 10), "short");
    }

    #[test]
    fn parses_pinned_notify_source() {
        let Some(source) = parse_pinned_notify_source("Editor editor.rs:2111") else {
            panic!("expected pinned notify source to parse");
        };
        assert_eq!(source.entity_type, "Editor");
        assert_eq!(source.caller_file, "editor.rs");
        assert_eq!(source.caller_line, 2111);

        let Some(source) = parse_pinned_notify_source("Editor,crates/editor/src/editor.rs,2111")
        else {
            panic!("expected comma-separated pinned notify source to parse");
        };
        assert_eq!(source.entity_type, "Editor");
        assert_eq!(source.caller_file, "crates/editor/src/editor.rs");
        assert_eq!(source.caller_line, 2111);

        assert!(parse_pinned_notify_source("off").is_none());
    }

    #[test]
    fn hidden_notify_sources_are_excluded_from_top_sources() {
        let mut devtools = GpuiDevTools::new();
        let now = Instant::now();

        devtools.notifications.push(NotifyEvent {
            entity_id: EntityId::from(1),
            entity_type: "Editor",
            caller_file: "crates/editor/src/editor.rs",
            caller_line: 2111,
            caller_column: 17,
            registered_window_count: 1,
            live_window_count: 1,
            timestamp: now,
        });
        devtools.notifications.push(NotifyEvent {
            entity_id: EntityId::from(2),
            entity_type: "Workspace",
            caller_file: "crates/workspace/src/workspace.rs",
            caller_line: 42,
            caller_column: 5,
            registered_window_count: 1,
            live_window_count: 1,
            timestamp: now,
        });
        let hidden_source = NotifySourceKey {
            entity_type: "Editor",
            caller_file: "crates/editor/src/editor.rs",
            caller_line: 2111,
        };
        devtools.hidden_notify_sources.insert(hidden_source);

        let top_sources = top_notify_sources(&devtools, now, TOP_SOURCE_COUNT);
        assert_eq!(top_sources.len(), 1);
        assert_eq!(top_sources[0].0.entity_type, "Workspace");

        let hidden_sources = hidden_notify_sources(&devtools, now);
        assert_eq!(hidden_sources, vec![(hidden_source, 1)]);
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
        assert_eq!(column_starts(&header).len(), column_starts(&label).len());
    }

    #[test]
    fn notify_sources_ignore_events_after_snapshot_time() {
        let mut devtools = GpuiDevTools::new();
        let now = Instant::now();

        devtools.notifications.push(NotifyEvent {
            entity_id: EntityId::from(1),
            entity_type: "Editor",
            caller_file: "crates/editor/src/editor.rs",
            caller_line: 2111,
            caller_column: 17,
            registered_window_count: 1,
            live_window_count: 1,
            timestamp: now + Duration::from_secs(1),
        });

        assert!(top_notify_sources(&devtools, now, TOP_SOURCE_COUNT).is_empty());
    }

    #[test]
    fn pinned_notify_sources_stay_at_top_even_when_not_recent() {
        let mut devtools = GpuiDevTools::new();
        let now = Instant::now();
        let pinned_event = NotifyEvent {
            entity_id: EntityId::from(1),
            entity_type: "Editor",
            caller_file: "crates/editor/src/editor.rs",
            caller_line: 2111,
            caller_column: 17,
            registered_window_count: 1,
            live_window_count: 1,
            timestamp: now - SOURCE_WINDOW - Duration::from_secs(1),
        };
        let pinned_source = NotifySourceKey::from(&pinned_event);
        devtools
            .notify_source_last_stats
            .insert(pinned_source, NotifySourceStats::from_event(&pinned_event));
        devtools
            .notify_source_total_counts
            .insert(pinned_source, 12);
        devtools.pinned_notify_sources.insert(pinned_source);
        devtools.notifications.push(pinned_event);

        for index in 0..TOP_SOURCE_COUNT + 1 {
            devtools.notifications.push(NotifyEvent {
                entity_id: EntityId::from((index + 2) as u64),
                entity_type: "Workspace",
                caller_file: "crates/workspace/src/workspace.rs",
                caller_line: 40 + index as u32,
                caller_column: 5,
                registered_window_count: 1,
                live_window_count: 1,
                timestamp: now,
            });
        }

        let top_sources = top_notify_sources(&devtools, now, TOP_SOURCE_COUNT);
        assert_eq!(top_sources.len(), TOP_SOURCE_COUNT + 1);
        assert_eq!(top_sources[0].0, pinned_source);
        assert_eq!(top_sources[0].1.count, 0);
        assert!(
            top_sources[1..]
                .iter()
                .all(|(source, _)| *source != pinned_source)
        );
    }

    #[test]
    fn hidden_render_sources_are_excluded_from_render_summary() {
        let mut devtools = GpuiDevTools::new();
        let now = Instant::now();
        let window_id = WindowId::from(1);

        devtools.renders.push(ViewRenderEvent {
            window_id,
            entity_id: EntityId::from(1),
            entity_type: "Editor",
            phase: ViewRenderPhase::UncachedRender,
            duration: None,
            cache_miss_reasons: CacheMissReasons::empty(),
            bounds: None,
            caching_disabled_by_inspector: false,
            timestamp: now,
        });
        devtools.renders.push(ViewRenderEvent {
            window_id,
            entity_id: EntityId::from(2),
            entity_type: "Workspace",
            phase: ViewRenderPhase::UncachedRender,
            duration: None,
            cache_miss_reasons: CacheMissReasons::empty(),
            bounds: None,
            caching_disabled_by_inspector: false,
            timestamp: now,
        });
        let hidden_source = RenderSourceKey {
            entity_type: "Editor",
            phase: ViewRenderPhase::UncachedRender,
        };
        devtools.hidden_render_sources.insert(hidden_source);

        let summary = render_summary(&devtools, window_id, now);
        assert_eq!(summary.render_count, 1);
        assert_eq!(summary.top_sources.len(), 1);
        assert_eq!(summary.top_sources[0].0.entity_type, "Workspace");
        assert_eq!(
            hidden_render_sources(&devtools, window_id, now),
            vec![(hidden_source, 1)]
        );
    }

    #[test]
    fn render_summary_tracks_reuse_count_by_view_type() {
        let mut devtools = GpuiDevTools::new();
        let now = Instant::now();
        let window_id = WindowId::from(1);

        devtools.renders.push(ViewRenderEvent {
            window_id,
            entity_id: EntityId::from(1),
            entity_type: "Editor",
            phase: ViewRenderPhase::UncachedRender,
            duration: None,
            cache_miss_reasons: CacheMissReasons::empty(),
            bounds: None,
            caching_disabled_by_inspector: false,
            timestamp: now,
        });
        for entity_id in [2, 3] {
            devtools.renders.push(ViewRenderEvent {
                window_id,
                entity_id: EntityId::from(entity_id),
                entity_type: "Editor",
                phase: ViewRenderPhase::PrepaintReuse,
                duration: None,
                cache_miss_reasons: CacheMissReasons::empty(),
                bounds: None,
                caching_disabled_by_inspector: false,
                timestamp: now,
            });
        }

        let summary = render_summary(&devtools, window_id, now);
        assert_eq!(summary.top_sources[0].1.count, 1);
        assert_eq!(summary.top_sources[0].1.reuse_count, 2);
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
        stats.record_event(&event);
        stats.reuse_count = 4;

        let label = format_render_source(1, RenderSourceKey::from(&event), stats, now);
        assert!(
            label.starts_with("1   Editor#42"),
            "expected rank+view prefix, got: {label:?}"
        );
        assert!(label.contains("render"));
        assert!(label.contains("25ms"));
        assert!(label.contains("1.2ms"));
        assert!(label.contains("20%"));
        assert!(label.contains("[bounds][dirty][inspector]"));
    }

    #[test]
    fn render_summary_ignores_events_after_snapshot_time() {
        let mut devtools = GpuiDevTools::new();
        let now = Instant::now();
        let window_id = WindowId::from(1);

        devtools.renders.push(ViewRenderEvent {
            window_id,
            entity_id: EntityId::from(1),
            entity_type: "Editor",
            phase: ViewRenderPhase::UncachedRender,
            duration: None,
            cache_miss_reasons: CacheMissReasons::empty(),
            bounds: None,
            caching_disabled_by_inspector: false,
            timestamp: now + Duration::from_secs(1),
        });

        let summary = render_summary(&devtools, window_id, now);
        assert_eq!(summary.render_count, 0);
        assert!(summary.top_sources.is_empty());
    }

    #[test]
    fn pinned_render_sources_stay_at_top_even_when_not_recent() {
        let mut devtools = GpuiDevTools::new();
        let now = Instant::now();
        let window_id = WindowId::from(1);
        let pinned_event = ViewRenderEvent {
            window_id,
            entity_id: EntityId::from(1),
            entity_type: "Editor",
            phase: ViewRenderPhase::UncachedRender,
            duration: None,
            cache_miss_reasons: CacheMissReasons::empty(),
            bounds: None,
            caching_disabled_by_inspector: false,
            timestamp: now - FRAME_RATE_WINDOW - Duration::from_secs(1),
        };
        let pinned_source = RenderSourceKey::from(&pinned_event);
        devtools
            .render_source_last_stats
            .insert(pinned_source, RenderSourceStats::from_event(&pinned_event));
        devtools.pinned_render_sources.insert(pinned_source);
        devtools.renders.push(pinned_event);

        let entity_types = [
            "WorkspaceA",
            "WorkspaceB",
            "WorkspaceC",
            "WorkspaceD",
            "WorkspaceE",
            "WorkspaceF",
        ];
        for index in 0..TOP_SOURCE_COUNT + 1 {
            devtools.renders.push(ViewRenderEvent {
                window_id,
                entity_id: EntityId::from((index + 2) as u64),
                entity_type: entity_types[index],
                phase: ViewRenderPhase::UncachedRender,
                duration: None,
                cache_miss_reasons: CacheMissReasons::empty(),
                bounds: None,
                caching_disabled_by_inspector: false,
                timestamp: now,
            });
        }

        let summary = render_summary(&devtools, window_id, now);
        assert_eq!(summary.top_sources.len(), TOP_SOURCE_COUNT + 1);
        assert_eq!(summary.top_sources[0].0, pinned_source);
        assert_eq!(summary.top_sources[0].1.count, 0);
        assert!(
            summary.top_sources[1..]
                .iter()
                .all(|(source, _)| *source != pinned_source)
        );
    }
}
