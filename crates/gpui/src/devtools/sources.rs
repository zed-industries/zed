use super::{
    ANIMATION_EXPIRY, FRAME_RATE_WINDOW, SOURCE_WINDOW, TOP_SOURCE_COUNT, event_age,
    events::{
        AnimationEventKind, CacheMissReasons, DirtyPathEvent, NotifyEvent, ViewRenderEvent,
        ViewRenderPhase,
    },
    format::short_type_name,
    state::GpuiDevTools,
};
use crate::{Bounds, EntityId, Pixels, WindowId};
use collections::{FxHashMap, FxHashSet};
use scheduler::Instant;
use std::time::Duration;

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
pub(super) struct NotifyCause {
    pub(super) source: NotifySourceKey,
    pub(super) entity_id: EntityId,
    pub(super) caller_column: u32,
    pub(super) timestamp: Instant,
}

impl NotifyCause {
    pub(super) fn from_event(event: &NotifyEvent) -> Self {
        Self {
            source: NotifySourceKey::from(event),
            entity_id: event.entity_id,
            caller_column: event.caller_column,
            timestamp: event.timestamp,
        }
    }

    pub(super) fn is_recent_at(self, timestamp: Instant, max_age: Duration) -> bool {
        event_age(timestamp, self.timestamp).is_some_and(|age| age <= max_age)
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) struct NotifySourceStats {
    pub(super) count: usize,
    pub(super) entity_id: EntityId,
    pub(super) caller_column: u32,
    pub(super) registered_window_count: usize,
    pub(super) live_window_count: usize,
    pub(super) last_timestamp: Option<Instant>,
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

#[derive(Clone, Debug)]
pub(super) struct DirtyPathSummary {
    pub(super) label: String,
    pub(super) count: usize,
    pub(super) cause: Option<NotifyCause>,
}

#[derive(Default)]
struct DirtyPathStats {
    count: usize,
    latest_timestamp: Option<Instant>,
    cause: Option<NotifyCause>,
}

pub(super) fn top_dirty_path(
    devtools: &GpuiDevTools,
    window_id: WindowId,
    now: Instant,
) -> Option<DirtyPathSummary> {
    let mut counts = FxHashMap::default();
    for event in devtools.dirty_paths.iter() {
        let Some(age) = event_age(now, event.timestamp) else {
            continue;
        };
        if event.window_id != window_id || age > SOURCE_WINDOW {
            continue;
        }

        let label = dirty_path_label(event);
        let stats = counts.entry(label).or_insert_with(DirtyPathStats::default);
        stats.count += 1;
        if stats
            .latest_timestamp
            .is_none_or(|latest_timestamp| event.timestamp >= latest_timestamp)
        {
            stats.latest_timestamp = Some(event.timestamp);
            stats.cause = devtools
                .windows
                .get(&window_id)
                .and_then(|window_state| {
                    window_state
                        .latest_dirty_cause_by_entity
                        .get(&event.invalidated_entity_id)
                })
                .copied()
                .filter(|cause| cause.is_recent_at(event.timestamp, SOURCE_WINDOW));
        }
    }
    counts
        .into_iter()
        .max_by_key(|(_, stats)| stats.count)
        .map(|(label, stats)| DirtyPathSummary {
            label,
            count: stats.count,
            cause: stats.cause,
        })
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
        stats.cause = devtools
            .latest_cause_by_render_source
            .get(source)
            .copied()
            .filter(|cause| cause.is_recent_at(now, SOURCE_WINDOW))
            .filter(|cause| {
                stats
                    .last_timestamp
                    .is_none_or(|last_timestamp| cause.timestamp <= last_timestamp)
            });
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
    pub(super) count: usize,
    pub(super) reuse_count: usize,
    pub(super) duration: Duration,
    pub(super) sample_entity_id: EntityId,
    pub(super) bounds: Option<Bounds<Pixels>>,
    pub(super) cache_miss_reasons: CacheMissReasons,
    pub(super) caching_disabled_by_inspector: bool,
    pub(super) last_timestamp: Option<Instant>,
    pub(super) cause: Option<NotifyCause>,
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
            cause: None,
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
            cause: None,
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

                sources.insert(ActiveAnimationSource::FrameRequest {
                    entity_type: event.entity_type,
                    caller_file,
                    caller_line: *caller_line,
                    caller_column: *caller_column,
                });
            }
            AnimationEventKind::ElementTick {
                element_id,
                animation_index,
                duration,
                repeats,
            } => {
                if *repeats {
                    sources.insert(ActiveAnimationSource::ElementTick {
                        entity_id: event.entity_id,
                        element_id,
                        animation_index: *animation_index,
                        duration: *duration,
                    });
                }
            }
        }
    }
    sources.len()
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum ActiveAnimationSource<'a> {
    FrameRequest {
        entity_type: &'static str,
        caller_file: &'static str,
        caller_line: u32,
        caller_column: u32,
    },
    ElementTick {
        entity_id: EntityId,
        element_id: &'a str,
        animation_index: usize,
        duration: Duration,
    },
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

#[cfg(test)]
mod tests {
    use super::super::events::DirtyPathSegment;
    use super::*;

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
    fn dirty_path_summary_includes_notify_cause() {
        let mut devtools = GpuiDevTools::new();
        let now = Instant::now();
        let window_id = WindowId::from(1);
        let notify = NotifyEvent {
            entity_id: EntityId::from(7),
            entity_type: "TerminalView",
            caller_file: "crates/terminal/src/terminal_view.rs",
            caller_line: 1011,
            caller_column: 68,
            registered_window_count: 1,
            live_window_count: 1,
            timestamp: now - Duration::from_millis(10),
        };
        let cause = NotifyCause::from_event(&notify);
        devtools
            .window_state(window_id)
            .latest_dirty_cause_by_entity
            .insert(notify.entity_id, cause);
        devtools.dirty_paths.push(DirtyPathEvent {
            window_id,
            invalidated_entity_id: notify.entity_id,
            invalidated_entity_type: notify.entity_type,
            path: vec![DirtyPathSegment {
                entity_id: EntityId::from(9),
                entity_type: "Dock",
            }],
            timestamp: now,
        });

        let Some(summary) = top_dirty_path(&devtools, window_id, now) else {
            panic!("expected dirty path summary");
        };
        assert_eq!(summary.count, 1);
        assert!(
            summary
                .label
                .contains(&format!("TerminalView#{}", notify.entity_id.as_u64()))
        );
        let Some(summary_cause) = summary.cause else {
            panic!("expected dirty path cause");
        };
        assert_eq!(summary_cause.source, NotifySourceKey::from(&notify));
        assert_eq!(summary_cause.entity_id, notify.entity_id);
    }

    #[test]
    fn render_summary_includes_recent_notify_cause() {
        let mut devtools = GpuiDevTools::new();
        let now = Instant::now();
        let window_id = WindowId::from(1);
        let notify = NotifyEvent {
            entity_id: EntityId::from(7),
            entity_type: "TerminalView",
            caller_file: "crates/terminal/src/terminal_view.rs",
            caller_line: 1011,
            caller_column: 68,
            registered_window_count: 1,
            live_window_count: 1,
            timestamp: now - Duration::from_millis(10),
        };
        let render = ViewRenderEvent {
            window_id,
            entity_id: EntityId::from(9),
            entity_type: "Dock",
            phase: ViewRenderPhase::UncachedRender,
            duration: None,
            cache_miss_reasons: CacheMissReasons::empty(),
            bounds: None,
            caching_disabled_by_inspector: false,
            timestamp: now,
        };
        let render_source = RenderSourceKey::from(&render);
        devtools.renders.push(render);
        devtools
            .latest_cause_by_render_source
            .insert(render_source, NotifyCause::from_event(&notify));

        let summary = render_summary(&devtools, window_id, now);
        let Some((_, stats)) = summary.top_sources.first() else {
            panic!("expected render source summary");
        };
        let Some(cause) = stats.cause else {
            panic!("expected render source cause");
        };
        assert_eq!(cause.source, NotifySourceKey::from(&notify));
        assert_eq!(cause.entity_id, notify.entity_id);
    }

    #[test]
    fn render_summary_ignores_stale_notify_cause() {
        let mut devtools = GpuiDevTools::new();
        let now = Instant::now();
        let window_id = WindowId::from(1);
        let notify = NotifyEvent {
            entity_id: EntityId::from(7),
            entity_type: "TerminalView",
            caller_file: "crates/terminal/src/terminal_view.rs",
            caller_line: 1011,
            caller_column: 68,
            registered_window_count: 1,
            live_window_count: 1,
            timestamp: now - SOURCE_WINDOW - Duration::from_millis(1),
        };
        let render = ViewRenderEvent {
            window_id,
            entity_id: EntityId::from(9),
            entity_type: "Dock",
            phase: ViewRenderPhase::UncachedRender,
            duration: None,
            cache_miss_reasons: CacheMissReasons::empty(),
            bounds: None,
            caching_disabled_by_inspector: false,
            timestamp: now,
        };
        let render_source = RenderSourceKey::from(&render);
        devtools.renders.push(render);
        devtools
            .latest_cause_by_render_source
            .insert(render_source, NotifyCause::from_event(&notify));

        let summary = render_summary(&devtools, window_id, now);
        let Some((_, stats)) = summary.top_sources.first() else {
            panic!("expected render source summary");
        };
        assert!(stats.cause.is_none());
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
