use super::{
    ANIMATION_CAPACITY, DEVTOOLS_HUD_SAMPLE_CAPACITY, DEVTOOLS_RECORDING_SAMPLE_CAPACITY,
    DIRTY_PATH_CAPACITY, FRAME_CAPACITY, NOTIFICATION_CAPACITY, RENDER_HEAT_DECAY,
    RENDER_HEAT_WINDOW, REUSE_OUTLINE_DURATION, SOURCE_WINDOW, VIEW_RENDER_CAPACITY,
    WINDOW_FRAME_CAPACITY,
    events::{
        AnimationEvent, CacheMissReasons, DirtyPathEvent, FrameEvent, NotifyEvent, ViewRenderEvent,
    },
    overlay::PreparedOverlay,
    ring_buffer::RingBuffer,
    sources::{
        NotifyCause, NotifySourceKey, NotifySourceStats, RenderSourceKey, RenderSourceStats,
    },
};
use crate::{Bounds, EntityId, Pixels, Point, WindowId};
use collections::{FxHashMap, FxHashSet};
use scheduler::Instant;
use std::{collections::VecDeque, time::Duration};

#[derive(Debug)]
pub(super) struct GpuiDevTools {
    pub(super) notifications: RingBuffer<NotifyEvent>,
    pub(super) frames: RingBuffer<FrameEvent>,
    pub(super) renders: RingBuffer<ViewRenderEvent>,
    pub(super) dirty_paths: RingBuffer<DirtyPathEvent>,
    pub(super) animations: RingBuffer<AnimationEvent>,
    pub(super) open_windows: FxHashSet<WindowId>,
    pub(super) windows: FxHashMap<WindowId, WindowDevToolsState>,
    pub(super) hidden_notify_sources: FxHashSet<NotifySourceKey>,
    pub(super) hidden_render_sources: FxHashSet<RenderSourceKey>,
    pub(super) collapsed_sections: FxHashSet<HudSection>,
    pub(super) notify_source_total_counts: FxHashMap<NotifySourceKey, usize>,
    pub(super) notify_source_last_stats: FxHashMap<NotifySourceKey, NotifySourceStats>,
    pub(super) latest_cause_by_entity: FxHashMap<EntityId, NotifyCause>,
    pub(super) latest_cause_by_render_source: FxHashMap<RenderSourceKey, NotifyCause>,
    pub(super) pinned_notify_sources: FxHashSet<NotifySourceKey>,
    pub(super) render_source_last_stats: FxHashMap<RenderSourceKey, RenderSourceStats>,
    pub(super) pinned_render_sources: FxHashSet<RenderSourceKey>,
    pub(super) paused_at: Option<Instant>,
    paused_notify_source_total_counts: Option<FxHashMap<NotifySourceKey, usize>>,
    paused_notify_source_last_stats: Option<FxHashMap<NotifySourceKey, NotifySourceStats>>,
    paused_render_source_last_stats: Option<FxHashMap<RenderSourceKey, RenderSourceStats>>,
    pub(super) notify_source_limit: usize,
    pub(super) render_source_limit: usize,
    pub(super) performance: DevtoolsPerformance,
    pub(super) show_flashes: bool,
    pub(super) show_heat: bool,
}

impl GpuiDevTools {
    pub(super) fn new() -> Self {
        Self {
            notifications: RingBuffer::new(NOTIFICATION_CAPACITY),
            frames: RingBuffer::new(FRAME_CAPACITY),
            renders: RingBuffer::new(VIEW_RENDER_CAPACITY),
            dirty_paths: RingBuffer::new(DIRTY_PATH_CAPACITY),
            animations: RingBuffer::new(ANIMATION_CAPACITY),
            open_windows: FxHashSet::default(),
            windows: FxHashMap::default(),
            hidden_notify_sources: FxHashSet::default(),
            hidden_render_sources: FxHashSet::default(),
            collapsed_sections: FxHashSet::default(),
            notify_source_total_counts: FxHashMap::default(),
            notify_source_last_stats: FxHashMap::default(),
            latest_cause_by_entity: FxHashMap::default(),
            latest_cause_by_render_source: FxHashMap::default(),
            pinned_notify_sources: FxHashSet::default(),
            render_source_last_stats: FxHashMap::default(),
            pinned_render_sources: FxHashSet::default(),
            paused_at: None,
            paused_notify_source_total_counts: None,
            paused_notify_source_last_stats: None,
            paused_render_source_last_stats: None,
            notify_source_limit: super::TOP_SOURCE_COUNT,
            render_source_limit: super::TOP_SOURCE_COUNT,
            performance: DevtoolsPerformance::new(),
            show_flashes: true,
            show_heat: true,
        }
    }

    pub(super) fn open_window(&mut self, window_id: WindowId) {
        self.open_windows.insert(window_id);
        self.window_state(window_id);
    }

    pub(super) fn close_window(&mut self, window_id: WindowId) {
        self.open_windows.remove(&window_id);
        if let Some(window_state) = self.windows.get_mut(&window_id) {
            window_state.prepared_overlay = None;
            window_state.hud_drag = None;
        }
    }

    pub(super) fn has_open_windows(&self) -> bool {
        !self.open_windows.is_empty()
    }

    pub(super) fn is_window_open(&self, window_id: WindowId) -> bool {
        self.open_windows.contains(&window_id)
    }

    pub(super) fn window_state(&mut self, window_id: WindowId) -> &mut WindowDevToolsState {
        self.windows
            .entry(window_id)
            .or_insert_with(WindowDevToolsState::new)
    }

    pub(super) fn forget_window(&mut self, window_id: WindowId) {
        self.open_windows.remove(&window_id);
        self.windows.remove(&window_id);
    }

    pub(super) fn pause(&mut self, now: Instant) {
        if self.paused_at.is_some() {
            return;
        }

        self.paused_at = Some(now);
        self.paused_notify_source_total_counts = Some(self.notify_source_total_counts.clone());
        self.paused_notify_source_last_stats = Some(self.notify_source_last_stats.clone());
        self.paused_render_source_last_stats = Some(self.render_source_last_stats.clone());
    }

    pub(super) fn resume(&mut self) {
        self.paused_at = None;
        self.paused_notify_source_total_counts = None;
        self.paused_notify_source_last_stats = None;
        self.paused_render_source_last_stats = None;
    }

    pub(super) fn notify_source_total_count(&self, source: NotifySourceKey) -> usize {
        self.paused_notify_source_total_counts
            .as_ref()
            .unwrap_or(&self.notify_source_total_counts)
            .get(&source)
            .copied()
            .unwrap_or(0)
    }

    pub(super) fn notify_source_last_stats(
        &self,
        source: NotifySourceKey,
    ) -> Option<NotifySourceStats> {
        self.paused_notify_source_last_stats
            .as_ref()
            .unwrap_or(&self.notify_source_last_stats)
            .get(&source)
            .copied()
    }

    pub(super) fn render_source_last_stats(
        &self,
        source: RenderSourceKey,
    ) -> Option<RenderSourceStats> {
        self.paused_render_source_last_stats
            .as_ref()
            .unwrap_or(&self.render_source_last_stats)
            .get(&source)
            .copied()
    }

    pub(super) fn record_recording_duration(&mut self, timestamp: Instant, duration: Duration) {
        self.performance.recording.push(DevtoolsDurationSample {
            timestamp,
            duration,
        });
    }

    pub(super) fn record_snapshot_duration(&mut self, timestamp: Instant, duration: Duration) {
        self.performance.snapshot.push(DevtoolsDurationSample {
            timestamp,
            duration,
        });
    }

    pub(super) fn record_prepaint_duration(&mut self, timestamp: Instant, duration: Duration) {
        self.performance.prepaint.push(DevtoolsDurationSample {
            timestamp,
            duration,
        });
    }

    pub(super) fn record_paint_duration(&mut self, timestamp: Instant, duration: Duration) {
        self.performance.paint.push(DevtoolsDurationSample {
            timestamp,
            duration,
        });
    }

    pub(super) fn performance_summary(&self, now: Instant) -> DevtoolsPerformanceSummary {
        self.performance.summary(now)
    }

    pub(super) fn clear_counters(&mut self) {
        self.notifications.clear();
        self.frames.clear();
        self.renders.clear();
        self.dirty_paths.clear();
        self.animations.clear();
        self.performance.clear();
        self.notify_source_total_counts.clear();
        self.latest_cause_by_entity.clear();
        self.latest_cause_by_render_source.clear();

        let notify_source_last_stats = self
            .pinned_notify_sources
            .iter()
            .filter_map(|source| {
                self.notify_source_last_stats
                    .get(source)
                    .copied()
                    .map(|stats| (*source, stats))
            })
            .collect();
        self.notify_source_last_stats = notify_source_last_stats;

        let render_source_last_stats = self
            .pinned_render_sources
            .iter()
            .filter_map(|source| {
                self.render_source_last_stats
                    .get(source)
                    .copied()
                    .map(|stats| (*source, stats))
            })
            .collect();
        self.render_source_last_stats = render_source_last_stats;

        if self.paused_at.is_some() {
            self.paused_notify_source_total_counts = Some(self.notify_source_total_counts.clone());
            self.paused_notify_source_last_stats = Some(self.notify_source_last_stats.clone());
            self.paused_render_source_last_stats = Some(self.render_source_last_stats.clone());
        }

        for window_state in self.windows.values_mut() {
            window_state.clear_counters();
        }
    }
}

#[derive(Debug)]
pub(super) struct DevtoolsPerformance {
    recording: RingBuffer<DevtoolsDurationSample>,
    snapshot: RingBuffer<DevtoolsDurationSample>,
    prepaint: RingBuffer<DevtoolsDurationSample>,
    paint: RingBuffer<DevtoolsDurationSample>,
}

impl DevtoolsPerformance {
    fn new() -> Self {
        Self {
            recording: RingBuffer::new(DEVTOOLS_RECORDING_SAMPLE_CAPACITY),
            snapshot: RingBuffer::new(DEVTOOLS_HUD_SAMPLE_CAPACITY),
            prepaint: RingBuffer::new(DEVTOOLS_HUD_SAMPLE_CAPACITY),
            paint: RingBuffer::new(DEVTOOLS_HUD_SAMPLE_CAPACITY),
        }
    }

    fn clear(&mut self) {
        self.recording.clear();
        self.snapshot.clear();
        self.prepaint.clear();
        self.paint.clear();
    }

    fn summary(&self, now: Instant) -> DevtoolsPerformanceSummary {
        DevtoolsPerformanceSummary {
            recording: summarize_duration_samples(&self.recording, now),
            snapshot: summarize_duration_samples(&self.snapshot, now),
            prepaint: summarize_duration_samples(&self.prepaint, now),
            paint: summarize_duration_samples(&self.paint, now),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) struct DevtoolsDurationSample {
    timestamp: Instant,
    duration: Duration,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(super) struct DevtoolsDurationSummary {
    pub(super) count: usize,
    pub(super) average: Duration,
    pub(super) max: Duration,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(super) struct DevtoolsPerformanceSummary {
    pub(super) recording: DevtoolsDurationSummary,
    pub(super) snapshot: DevtoolsDurationSummary,
    pub(super) prepaint: DevtoolsDurationSummary,
    pub(super) paint: DevtoolsDurationSummary,
}

fn summarize_duration_samples(
    samples: &RingBuffer<DevtoolsDurationSample>,
    now: Instant,
) -> DevtoolsDurationSummary {
    let mut count = 0;
    let mut total_nanos = 0_u128;
    let mut max = Duration::default();

    for sample in samples.iter() {
        if sample.timestamp > now || now.duration_since(sample.timestamp) > SOURCE_WINDOW {
            continue;
        }

        count += 1;
        total_nanos += sample.duration.as_nanos();
        max = max.max(sample.duration);
    }

    let average = if count == 0 {
        Duration::default()
    } else {
        let average_nanos = (total_nanos / count as u128).min(u64::MAX as u128) as u64;
        Duration::from_nanos(average_nanos)
    };

    DevtoolsDurationSummary {
        count,
        average,
        max,
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(super) enum HudSection {
    Frame,
    Notify,
    Dirty,
    Render,
    Animation,
    Hidden,
}

impl HudSection {
    pub(super) const ALL: [Self; 6] = [
        Self::Frame,
        Self::Notify,
        Self::Dirty,
        Self::Render,
        Self::Animation,
        Self::Hidden,
    ];

    pub(super) fn label(self) -> &'static str {
        match self {
            Self::Frame => "Frame",
            Self::Notify => "Notify",
            Self::Dirty => "Dirty",
            Self::Render => "Render",
            Self::Animation => "Anim",
            Self::Hidden => "Hidden",
        }
    }
}

#[derive(Debug)]
pub(super) struct WindowDevToolsState {
    pub(super) recent_frames: RingBuffer<FrameEvent>,
    pub(super) view_bounds: FxHashMap<EntityId, Bounds<Pixels>>,
    pub(super) active_flashes: FxHashMap<EntityId, FlashState>,
    pub(super) render_heat: FxHashMap<EntityId, RenderHeatState>,
    pub(super) reuse_outlines: FxHashMap<EntityId, ReuseOutlineState>,
    pub(super) prepared_overlay: Option<PreparedOverlay>,
    pub(super) hud_origin: Option<Point<Pixels>>,
    pub(super) hud_drag: Option<HudDragState>,
    pub(super) latest_dirty_cause_by_entity: FxHashMap<EntityId, NotifyCause>,
}

impl WindowDevToolsState {
    fn new() -> Self {
        Self {
            recent_frames: RingBuffer::new(WINDOW_FRAME_CAPACITY),
            view_bounds: FxHashMap::default(),
            active_flashes: FxHashMap::default(),
            render_heat: FxHashMap::default(),
            reuse_outlines: FxHashMap::default(),
            prepared_overlay: None,
            hud_origin: None,
            hud_drag: None,
            latest_dirty_cause_by_entity: FxHashMap::default(),
        }
    }

    pub(super) fn record_render_heat(
        &mut self,
        entity_id: EntityId,
        timestamp: Instant,
        bounds: Option<Bounds<Pixels>>,
        source: RenderSourceKey,
        cache_miss_reasons: CacheMissReasons,
    ) {
        let heat = self
            .render_heat
            .entry(entity_id)
            .or_insert_with(|| RenderHeatState::new(source));
        heat.record(timestamp, bounds, source, cache_miss_reasons);
    }

    pub(super) fn record_reuse_outline(
        &mut self,
        entity_id: EntityId,
        timestamp: Instant,
        bounds: Option<Bounds<Pixels>>,
        source: RenderSourceKey,
    ) {
        self.reuse_outlines.insert(
            entity_id,
            ReuseOutlineState {
                timestamp,
                bounds,
                source,
            },
        );
    }

    fn clear_counters(&mut self) {
        self.recent_frames.clear();
        self.active_flashes.clear();
        self.render_heat.clear();
        self.reuse_outlines.clear();
        self.latest_dirty_cause_by_entity.clear();
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) struct FlashState {
    pub(super) timestamp: Instant,
    pub(super) source: RenderSourceKey,
}

#[derive(Clone, Debug)]
pub(super) struct RenderHeatState {
    timestamps: VecDeque<Instant>,
    pub(super) last_render: Instant,
    pub(super) last_rate: usize,
    pub(super) bounds: Option<Bounds<Pixels>>,
    pub(super) source: RenderSourceKey,
    pub(super) cache_miss_reasons: CacheMissReasons,
    pub(super) cause: RenderHeatCause,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct ReuseOutlineState {
    pub(super) timestamp: Instant,
    pub(super) bounds: Option<Bounds<Pixels>>,
    pub(super) source: RenderSourceKey,
}

impl ReuseOutlineState {
    pub(super) fn expired(self, now: Instant) -> bool {
        now.duration_since(self.timestamp) > REUSE_OUTLINE_DURATION
    }

    pub(super) fn opacity(self, now: Instant) -> f32 {
        let age = now.duration_since(self.timestamp);
        (1. - age.as_secs_f32() / REUSE_OUTLINE_DURATION.as_secs_f32()).clamp(0., 1.)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum RenderHeatCause {
    Render,
    Refresh,
}

impl RenderHeatState {
    fn new(source: RenderSourceKey) -> Self {
        Self {
            timestamps: VecDeque::new(),
            last_render: Instant::now(),
            last_rate: 0,
            bounds: None,
            source,
            cache_miss_reasons: CacheMissReasons::empty(),
            cause: RenderHeatCause::Render,
        }
    }

    fn record(
        &mut self,
        timestamp: Instant,
        bounds: Option<Bounds<Pixels>>,
        source: RenderSourceKey,
        cache_miss_reasons: CacheMissReasons,
    ) {
        self.source = source;
        self.cache_miss_reasons = cache_miss_reasons;
        self.cause = if cache_miss_reasons.window_refreshing() {
            RenderHeatCause::Refresh
        } else {
            RenderHeatCause::Render
        };
        self.last_render = timestamp;
        if let Some(bounds) = bounds {
            self.bounds = Some(bounds);
        }
        self.timestamps.push_back(timestamp);
        self.prune(timestamp);
    }

    pub(super) fn prune(&mut self, now: Instant) -> usize {
        while self
            .timestamps
            .front()
            .is_some_and(|timestamp| now.duration_since(*timestamp) > RENDER_HEAT_WINDOW)
        {
            self.timestamps.pop_front();
        }

        let rate = self.timestamps.len();
        if rate > 0 {
            self.last_rate = rate;
        }
        rate
    }

    pub(super) fn expired(&self, now: Instant) -> bool {
        now.duration_since(self.last_render) > RENDER_HEAT_WINDOW + RENDER_HEAT_DECAY
    }

    pub(super) fn opacity(&self, now: Instant) -> f32 {
        let age = now.duration_since(self.last_render);
        if age <= RENDER_HEAT_WINDOW {
            1.
        } else {
            let decay_age = age - RENDER_HEAT_WINDOW;
            (1. - decay_age.as_secs_f32() / RENDER_HEAT_DECAY.as_secs_f32()).clamp(0., 1.)
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) struct HudDragState {
    pub(super) cursor_offset: Point<Pixels>,
}

#[cfg(test)]
mod tests {
    use super::{super::events::ViewRenderPhase, *};
    use std::time::Duration;

    #[test]
    fn render_heat_tracks_recent_rate_and_fades_after_activity_stops() {
        let source = RenderSourceKey {
            entity_id: EntityId::from(1),
            entity_type: "Editor",
            phase: ViewRenderPhase::UncachedRender,
        };
        let now = Instant::now();
        let mut heat = RenderHeatState::new(source);

        heat.record(now, None, source, CacheMissReasons::empty());
        assert_eq!(heat.cause, RenderHeatCause::Render);

        let mut refresh_reasons = CacheMissReasons::empty();
        refresh_reasons.insert_window_refreshing();
        heat.record(
            now + Duration::from_millis(100),
            None,
            source,
            refresh_reasons,
        );
        assert_eq!(heat.cause, RenderHeatCause::Refresh);

        assert_eq!(heat.prune(now + Duration::from_millis(100)), 2);

        let last_render = now + Duration::from_millis(100);
        let decaying = last_render + RENDER_HEAT_WINDOW + Duration::from_millis(100);
        assert_eq!(heat.prune(decaying), 0);
        assert_eq!(heat.last_rate, 2);
        assert!(heat.opacity(decaying) > 0.);
        assert!(heat.opacity(decaying) < 1.);

        assert!(heat.expired(
            last_render + RENDER_HEAT_WINDOW + RENDER_HEAT_DECAY + Duration::from_millis(1)
        ));
    }

    #[test]
    fn reuse_outline_fades_quickly_without_affecting_heat() {
        let source = RenderSourceKey {
            entity_id: EntityId::from(1),
            entity_type: "Editor",
            phase: ViewRenderPhase::PrepaintReuse,
        };
        let now = Instant::now();
        let outline = ReuseOutlineState {
            timestamp: now,
            bounds: None,
            source,
        };

        assert!(!outline.expired(now));
        assert_eq!(outline.opacity(now), 1.);

        let fading = now + Duration::from_millis(100);
        assert!(outline.opacity(fading) > 0.);
        assert!(outline.opacity(fading) < 1.);

        assert!(outline.expired(now + REUSE_OUTLINE_DURATION + Duration::from_millis(1)));
    }

    #[test]
    fn performance_summary_uses_recent_non_future_samples() {
        let mut devtools = GpuiDevTools::new();
        let base = Instant::now();
        let now = base + SOURCE_WINDOW + Duration::from_millis(100);

        devtools.record_recording_duration(base, Duration::from_millis(9));
        devtools
            .record_recording_duration(base + Duration::from_millis(100), Duration::from_millis(1));
        devtools
            .record_recording_duration(base + Duration::from_millis(200), Duration::from_millis(3));
        devtools
            .record_recording_duration(now + Duration::from_millis(1), Duration::from_millis(100));

        let summary = devtools.performance_summary(now).recording;
        assert_eq!(summary.count, 2);
        assert_eq!(summary.average, Duration::from_millis(2));
        assert_eq!(summary.max, Duration::from_millis(3));
    }

    #[test]
    fn forget_window_removes_window_state() {
        let mut devtools = GpuiDevTools::new();
        let window_id = WindowId::from(1);

        devtools.window_state(window_id);
        assert!(devtools.windows.contains_key(&window_id));

        devtools.forget_window(window_id);
        assert!(!devtools.windows.contains_key(&window_id));
    }
}
