use super::{
    ANIMATION_CAPACITY, DIRTY_PATH_CAPACITY, FRAME_CAPACITY, NOTIFICATION_CAPACITY,
    RENDER_HEAT_DECAY, RENDER_HEAT_WINDOW, REUSE_OUTLINE_DURATION, VIEW_RENDER_CAPACITY,
    WINDOW_FRAME_CAPACITY,
    events::{
        AnimationEvent, CacheMissReasons, DirtyPathEvent, FrameEvent, NotifyEvent, ViewRenderEvent,
    },
    overlay::PreparedOverlay,
    ring_buffer::RingBuffer,
    sources::{NotifySourceKey, NotifySourceStats, RenderSourceKey, RenderSourceStats},
};
use crate::{Bounds, EntityId, Pixels, Point, WindowId};
use collections::{FxHashMap, FxHashSet};
use scheduler::Instant;
use std::collections::VecDeque;

#[derive(Debug)]
pub(super) struct GpuiDevTools {
    pub(super) notifications: RingBuffer<NotifyEvent>,
    pub(super) frames: RingBuffer<FrameEvent>,
    pub(super) renders: RingBuffer<ViewRenderEvent>,
    pub(super) dirty_paths: RingBuffer<DirtyPathEvent>,
    pub(super) animations: RingBuffer<AnimationEvent>,
    pub(super) windows: FxHashMap<WindowId, WindowDevToolsState>,
    pub(super) hidden_notify_sources: FxHashSet<NotifySourceKey>,
    pub(super) hidden_render_sources: FxHashSet<RenderSourceKey>,
    pub(super) collapsed_sections: FxHashSet<HudSection>,
    pub(super) notify_source_total_counts: FxHashMap<NotifySourceKey, usize>,
    pub(super) notify_source_last_stats: FxHashMap<NotifySourceKey, NotifySourceStats>,
    pub(super) pinned_notify_sources: FxHashSet<NotifySourceKey>,
    pub(super) render_source_last_stats: FxHashMap<RenderSourceKey, RenderSourceStats>,
    pub(super) pinned_render_sources: FxHashSet<RenderSourceKey>,
    pub(super) paused_at: Option<Instant>,
    paused_notify_source_total_counts: Option<FxHashMap<NotifySourceKey, usize>>,
    paused_notify_source_last_stats: Option<FxHashMap<NotifySourceKey, NotifySourceStats>>,
    paused_render_source_last_stats: Option<FxHashMap<RenderSourceKey, RenderSourceStats>>,
    pub(super) show_flashes: bool,
    pub(super) show_heat: bool,
    pub(super) initial_pinned_notify_source_resolved: bool,
}

impl GpuiDevTools {
    pub(super) fn new() -> Self {
        Self {
            notifications: RingBuffer::new(NOTIFICATION_CAPACITY),
            frames: RingBuffer::new(FRAME_CAPACITY),
            renders: RingBuffer::new(VIEW_RENDER_CAPACITY),
            dirty_paths: RingBuffer::new(DIRTY_PATH_CAPACITY),
            animations: RingBuffer::new(ANIMATION_CAPACITY),
            windows: FxHashMap::default(),
            hidden_notify_sources: FxHashSet::default(),
            hidden_render_sources: FxHashSet::default(),
            collapsed_sections: FxHashSet::default(),
            notify_source_total_counts: FxHashMap::default(),
            notify_source_last_stats: FxHashMap::default(),
            pinned_notify_sources: FxHashSet::default(),
            render_source_last_stats: FxHashMap::default(),
            pinned_render_sources: FxHashSet::default(),
            paused_at: None,
            paused_notify_source_total_counts: None,
            paused_notify_source_last_stats: None,
            paused_render_source_last_stats: None,
            show_flashes: true,
            show_heat: true,
            initial_pinned_notify_source_resolved: false,
        }
    }

    pub(super) fn window_state(&mut self, window_id: WindowId) -> &mut WindowDevToolsState {
        self.windows
            .entry(window_id)
            .or_insert_with(WindowDevToolsState::new)
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

    pub(super) fn clear_counters(&mut self) {
        self.notifications.clear();
        self.frames.clear();
        self.renders.clear();
        self.dirty_paths.clear();
        self.animations.clear();
        self.notify_source_total_counts.clear();

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
}
