//! Runtime GPUI invalidation and rendering diagnostics.

mod window;

use crate::{App, Bounds, ElementId, EntityId, Pixels, SceneStats, Window, WindowId};
use collections::{FxHashMap, FxHashSet};
use parking_lot::RwLock;
use scheduler::Instant;
use std::{
    collections::VecDeque,
    sync::{
        LazyLock,
        atomic::{AtomicBool, Ordering::SeqCst},
    },
    time::Duration,
};

const NOTIFICATION_CAPACITY: usize = 16384;
const FRAME_CAPACITY: usize = 2048;
const VIEW_RENDER_CAPACITY: usize = 8192;
const DIRTY_PATH_CAPACITY: usize = 4096;
const ANIMATION_CAPACITY: usize = 4096;
const WINDOW_FRAME_CAPACITY: usize = 240;
const SOURCE_WINDOW: Duration = Duration::from_secs(5);

static GPUI_DEVTOOLS_ENABLED: AtomicBool = AtomicBool::new(false);
static GPUI_DEVTOOLS: LazyLock<RwLock<GpuiDevTools>> =
    LazyLock::new(|| RwLock::new(GpuiDevTools::new()));

/// Opens the GPUI devtools window for the given source window.
pub fn open(source_window: &mut Window, cx: &mut App) {
    let was_enabled = GPUI_DEVTOOLS_ENABLED.swap(true, SeqCst);
    let window_id = source_window.handle.window_id();
    {
        let mut devtools = GPUI_DEVTOOLS.write();
        devtools.open_window(window_id);
        devtools.resume();
        if !was_enabled {
            devtools.clear_counters();
        }
    }
    window::open(window_id, cx);
}

pub(crate) fn enabled() -> bool {
    GPUI_DEVTOOLS_ENABLED.load(SeqCst)
}

pub(crate) fn forget_window(window_id: WindowId) {
    if !enabled() {
        return;
    }

    let any_window_open = {
        let mut devtools = GPUI_DEVTOOLS.write();
        devtools.forget_window(window_id);
        let any_window_open = devtools.has_open_windows();
        if !any_window_open {
            devtools.clear_counters();
            devtools.resume();
        }
        any_window_open
    };
    GPUI_DEVTOOLS_ENABLED.store(any_window_open, SeqCst);
}

pub(crate) fn record_notify(event: NotifyEvent) {
    if !enabled() {
        return;
    }

    let mut devtools = GPUI_DEVTOOLS.write();
    let source = NotifySourceKey::from(&event);
    let cause = NotifyCause::from_event(&event);
    *devtools
        .notify_source_total_counts
        .entry(source)
        .or_insert(0) += 1;
    devtools
        .latest_cause_by_entity
        .insert(event.entity_id, cause);
    devtools
        .notify_source_last_stats
        .insert(source, NotifySourceStats::from_event(&event));
    devtools.notifications.push(event);
}

pub(crate) fn record_frame(event: FrameEvent) {
    if !enabled() || !window_open(event.window_id) {
        return;
    }

    let mut devtools = GPUI_DEVTOOLS.write();
    let window_id = event.window_id;
    devtools.frames.push(event.clone());
    if devtools.paused_at.is_none() {
        devtools.window_state(window_id).recent_frames.push(event);
    }
}

pub(crate) fn record_dirty_path(event: DirtyPathEvent) {
    if !enabled() || !window_open(event.window_id) {
        return;
    }

    let mut devtools = GPUI_DEVTOOLS.write();
    let cause = devtools
        .latest_cause_by_entity
        .get(&event.invalidated_entity_id)
        .copied();
    let stale_cause =
        cause.is_some_and(|cause| !cause.is_recent_at(event.timestamp, SOURCE_WINDOW));
    let cause = cause.filter(|cause| cause.is_recent_at(event.timestamp, SOURCE_WINDOW));
    if stale_cause {
        devtools
            .latest_cause_by_entity
            .remove(&event.invalidated_entity_id);
    }
    if let Some(cause) = cause {
        let window_state = devtools.window_state(event.window_id);
        window_state
            .latest_dirty_cause_by_entity
            .insert(event.invalidated_entity_id, cause);
        for segment in &event.path {
            window_state
                .latest_dirty_cause_by_entity
                .insert(segment.entity_id, cause);
        }
    }
    devtools.dirty_paths.push(event);
}

pub(crate) fn record_view_bounds(window_id: WindowId, entity_id: EntityId, bounds: Bounds<Pixels>) {
    if !enabled() || !window_open(window_id) {
        return;
    }

    let mut devtools = GPUI_DEVTOOLS.write();
    if devtools.paused_at.is_none() {
        devtools
            .window_state(window_id)
            .view_bounds
            .insert(entity_id, bounds);
    }
}

pub(crate) fn record_view_render(event: ViewRenderEvent) {
    if !enabled() || !window_open(event.window_id) {
        return;
    }

    let mut devtools = GPUI_DEVTOOLS.write();
    let source = RenderSourceKey::from(&event);
    let cause = devtools
        .windows
        .get(&event.window_id)
        .and_then(|window_state| {
            window_state
                .latest_dirty_cause_by_entity
                .get(&event.entity_id)
        })
        .copied();
    let stale_cause =
        cause.is_some_and(|cause| !cause.is_recent_at(event.timestamp, SOURCE_WINDOW));
    let cause = cause.filter(|cause| cause.is_recent_at(event.timestamp, SOURCE_WINDOW));
    if stale_cause && let Some(window_state) = devtools.windows.get_mut(&event.window_id) {
        window_state
            .latest_dirty_cause_by_entity
            .remove(&event.entity_id);
    }
    if let Some(cause) = cause {
        devtools.latest_cause_by_render_source.insert(source, cause);
    }
    let mut stats = RenderSourceStats::from_event(&event);
    stats.cause = cause;
    devtools.render_source_last_stats.insert(source, stats);

    if let Some(bounds) = event.bounds
        && devtools.paused_at.is_none()
    {
        devtools
            .window_state(event.window_id)
            .view_bounds
            .insert(event.entity_id, bounds);
    }

    devtools.renders.push(event);
}

pub(crate) fn record_animation(event: AnimationEvent) {
    if !enabled() || !window_open(event.window_id) {
        return;
    }

    GPUI_DEVTOOLS.write().animations.push(event);
}

fn close_source_window(window_id: WindowId) {
    let any_window_open = {
        let mut devtools = GPUI_DEVTOOLS.write();
        devtools.close_window(window_id);
        let any_window_open = devtools.has_open_windows();
        if !any_window_open {
            devtools.clear_counters();
            devtools.resume();
        }
        any_window_open
    };
    GPUI_DEVTOOLS_ENABLED.store(any_window_open, SeqCst);
}

fn window_open(window_id: WindowId) -> bool {
    GPUI_DEVTOOLS.read().is_window_open(window_id)
}

fn event_age(now: Instant, timestamp: Instant) -> Option<Duration> {
    if timestamp > now {
        None
    } else {
        Some(now.duration_since(timestamp))
    }
}

#[derive(Clone, Debug)]
pub(crate) struct NotifyEvent {
    pub(crate) entity_id: EntityId,
    entity_type: &'static str,
    caller_file: &'static str,
    caller_line: u32,
    caller_column: u32,
    registered_window_count: usize,
    live_window_count: usize,
    timestamp: Instant,
}

impl NotifyEvent {
    pub(crate) fn new(
        entity_id: EntityId,
        entity_type: &'static str,
        caller: &'static std::panic::Location<'static>,
        registered_window_count: usize,
        live_window_count: usize,
    ) -> Self {
        Self {
            entity_id,
            entity_type,
            caller_file: caller.file(),
            caller_line: caller.line(),
            caller_column: caller.column(),
            registered_window_count,
            live_window_count,
            timestamp: Instant::now(),
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct FrameEvent {
    pub(crate) window_id: WindowId,
    pub(crate) reason: &'static str,
    pub(crate) dirty_before_frame: bool,
    pub(crate) dirty_view_count: usize,
    pub(crate) invalidator_update_count: usize,
    pub(crate) rebuilt_scene: bool,
    pub(crate) draw_duration: Option<Duration>,
    pub(crate) present_duration: Duration,
    pub(crate) scene_stats: SceneStats,
    pub(crate) timestamp: Instant,
}

#[derive(Clone, Debug)]
pub(crate) struct DirtyPathSegment {
    pub(crate) entity_id: EntityId,
    pub(crate) entity_type: &'static str,
}

#[derive(Clone, Debug)]
pub(crate) struct DirtyPathEvent {
    window_id: WindowId,
    invalidated_entity_id: EntityId,
    invalidated_entity_type: &'static str,
    path: Vec<DirtyPathSegment>,
    timestamp: Instant,
}

impl DirtyPathEvent {
    pub(crate) fn new(
        window_id: WindowId,
        invalidated_entity_id: EntityId,
        invalidated_entity_type: &'static str,
        path: Vec<DirtyPathSegment>,
    ) -> Self {
        Self {
            window_id,
            invalidated_entity_id,
            invalidated_entity_type,
            path,
            timestamp: Instant::now(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) enum ViewRenderPhase {
    UncachedRender,
    UncachedRenderInspector,
    UncachedPrepaint,
    CachedCacheMissRefresh,
    PrepaintReuse,
    PaintReuse,
}

impl ViewRenderPhase {
    fn records_render(self) -> bool {
        matches!(
            self,
            ViewRenderPhase::UncachedRender
                | ViewRenderPhase::UncachedRenderInspector
                | ViewRenderPhase::CachedCacheMissRefresh
        )
    }

    fn is_reuse(self) -> bool {
        matches!(
            self,
            ViewRenderPhase::PrepaintReuse | ViewRenderPhase::PaintReuse
        )
    }

    fn as_str(self) -> &'static str {
        match self {
            ViewRenderPhase::UncachedRender => "render",
            ViewRenderPhase::UncachedRenderInspector => "render inspector",
            ViewRenderPhase::UncachedPrepaint => "prepaint",
            ViewRenderPhase::CachedCacheMissRefresh => "cache miss",
            ViewRenderPhase::PrepaintReuse => "reuse prepaint",
            ViewRenderPhase::PaintReuse => "reuse paint",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct CacheMissReasons(u8);

impl CacheMissReasons {
    const MISSING_CACHE: u8 = 1 << 0;
    const BOUNDS_CHANGED: u8 = 1 << 1;
    const CONTENT_MASK_CHANGED: u8 = 1 << 2;
    const TEXT_STYLE_CHANGED: u8 = 1 << 3;
    const VIEW_DIRTY: u8 = 1 << 4;
    const WINDOW_REFRESHING: u8 = 1 << 5;

    pub(crate) fn empty() -> Self {
        Self(0)
    }

    pub(crate) fn insert_missing_cache(&mut self) {
        self.0 |= Self::MISSING_CACHE;
    }

    pub(crate) fn insert_bounds_changed(&mut self) {
        self.0 |= Self::BOUNDS_CHANGED;
    }

    pub(crate) fn insert_content_mask_changed(&mut self) {
        self.0 |= Self::CONTENT_MASK_CHANGED;
    }

    pub(crate) fn insert_text_style_changed(&mut self) {
        self.0 |= Self::TEXT_STYLE_CHANGED;
    }

    pub(crate) fn insert_view_dirty(&mut self) {
        self.0 |= Self::VIEW_DIRTY;
    }

    pub(crate) fn insert_window_refreshing(&mut self) {
        self.0 |= Self::WINDOW_REFRESHING;
    }

    pub(crate) fn is_empty(self) -> bool {
        self.0 == 0
    }

    fn labels(self) -> Vec<&'static str> {
        let mut labels = Vec::new();
        if self.0 & Self::MISSING_CACHE != 0 {
            labels.push("first");
        }
        if self.0 & Self::BOUNDS_CHANGED != 0 {
            labels.push("bounds");
        }
        if self.0 & Self::CONTENT_MASK_CHANGED != 0 {
            labels.push("mask");
        }
        if self.0 & Self::TEXT_STYLE_CHANGED != 0 {
            labels.push("text");
        }
        if self.0 & Self::VIEW_DIRTY != 0 {
            labels.push("dirty");
        }
        if self.0 & Self::WINDOW_REFRESHING != 0 {
            labels.push("refresh");
        }
        labels
    }
}

#[derive(Clone, Debug)]
pub(crate) struct ViewRenderEvent {
    pub(crate) window_id: WindowId,
    pub(crate) entity_id: EntityId,
    pub(crate) entity_type: &'static str,
    pub(crate) phase: ViewRenderPhase,
    pub(crate) duration: Option<Duration>,
    pub(crate) cache_miss_reasons: CacheMissReasons,
    pub(crate) bounds: Option<Bounds<Pixels>>,
    pub(crate) caching_disabled_by_inspector: bool,
    pub(crate) timestamp: Instant,
}

#[derive(Clone, Debug)]
pub(crate) struct AnimationEvent {
    window_id: WindowId,
    entity_id: EntityId,
    entity_type: &'static str,
    kind: AnimationEventKind,
    timestamp: Instant,
}

#[derive(Clone, Debug)]
enum AnimationEventKind {
    FrameRequest {
        caller_file: &'static str,
        caller_line: u32,
        caller_column: u32,
    },
    ElementTick {
        element_id: String,
        animation_index: usize,
        duration: Duration,
        repeats: bool,
    },
}

pub(crate) fn animation_frame_request_event(
    window_id: WindowId,
    entity_id: EntityId,
    entity_type: &'static str,
    caller: &'static std::panic::Location<'static>,
) -> AnimationEvent {
    AnimationEvent {
        window_id,
        entity_id,
        entity_type,
        kind: AnimationEventKind::FrameRequest {
            caller_file: caller.file(),
            caller_line: caller.line(),
            caller_column: caller.column(),
        },
        timestamp: Instant::now(),
    }
}

pub(crate) fn animation_element_tick_event(
    window_id: WindowId,
    entity_id: EntityId,
    entity_type: &'static str,
    element_id: &ElementId,
    animation_index: usize,
    duration: Duration,
    repeats: bool,
) -> AnimationEvent {
    AnimationEvent {
        window_id,
        entity_id,
        entity_type,
        kind: AnimationEventKind::ElementTick {
            element_id: format!("{element_id:?}"),
            animation_index,
            duration,
            repeats,
        },
        timestamp: Instant::now(),
    }
}

#[derive(Debug)]
struct GpuiDevTools {
    notifications: RingBuffer<NotifyEvent>,
    frames: RingBuffer<FrameEvent>,
    renders: RingBuffer<ViewRenderEvent>,
    dirty_paths: RingBuffer<DirtyPathEvent>,
    animations: RingBuffer<AnimationEvent>,
    open_windows: FxHashSet<WindowId>,
    windows: FxHashMap<WindowId, WindowDevToolsState>,
    notify_source_total_counts: FxHashMap<NotifySourceKey, usize>,
    notify_source_last_stats: FxHashMap<NotifySourceKey, NotifySourceStats>,
    latest_cause_by_entity: FxHashMap<EntityId, NotifyCause>,
    latest_cause_by_render_source: FxHashMap<RenderSourceKey, NotifyCause>,
    render_source_last_stats: FxHashMap<RenderSourceKey, RenderSourceStats>,
    paused_at: Option<Instant>,
}

impl GpuiDevTools {
    fn new() -> Self {
        Self {
            notifications: RingBuffer::new(NOTIFICATION_CAPACITY),
            frames: RingBuffer::new(FRAME_CAPACITY),
            renders: RingBuffer::new(VIEW_RENDER_CAPACITY),
            dirty_paths: RingBuffer::new(DIRTY_PATH_CAPACITY),
            animations: RingBuffer::new(ANIMATION_CAPACITY),
            open_windows: FxHashSet::default(),
            windows: FxHashMap::default(),
            notify_source_total_counts: FxHashMap::default(),
            notify_source_last_stats: FxHashMap::default(),
            latest_cause_by_entity: FxHashMap::default(),
            latest_cause_by_render_source: FxHashMap::default(),
            render_source_last_stats: FxHashMap::default(),
            paused_at: None,
        }
    }

    fn open_window(&mut self, window_id: WindowId) {
        self.open_windows.insert(window_id);
        self.window_state(window_id);
    }

    fn close_window(&mut self, window_id: WindowId) {
        self.open_windows.remove(&window_id);
    }

    fn has_open_windows(&self) -> bool {
        !self.open_windows.is_empty()
    }

    fn is_window_open(&self, window_id: WindowId) -> bool {
        self.open_windows.contains(&window_id)
    }

    fn window_state(&mut self, window_id: WindowId) -> &mut WindowDevToolsState {
        self.windows
            .entry(window_id)
            .or_insert_with(WindowDevToolsState::new)
    }

    fn forget_window(&mut self, window_id: WindowId) {
        self.open_windows.remove(&window_id);
        self.windows.remove(&window_id);
    }

    fn pause(&mut self, now: Instant) {
        if self.paused_at.is_none() {
            self.paused_at = Some(now);
        }
    }

    fn resume(&mut self) {
        self.paused_at = None;
    }

    fn clear_counters(&mut self) {
        self.notifications.clear();
        self.frames.clear();
        self.renders.clear();
        self.dirty_paths.clear();
        self.animations.clear();
        self.notify_source_total_counts.clear();
        self.notify_source_last_stats.clear();
        self.latest_cause_by_entity.clear();
        self.latest_cause_by_render_source.clear();
        self.render_source_last_stats.clear();
        for window_state in self.windows.values_mut() {
            window_state.recent_frames.clear();
            window_state.view_bounds.clear();
            window_state.latest_dirty_cause_by_entity.clear();
        }
    }
}

#[derive(Debug)]
struct WindowDevToolsState {
    recent_frames: RingBuffer<FrameEvent>,
    view_bounds: FxHashMap<EntityId, Bounds<Pixels>>,
    latest_dirty_cause_by_entity: FxHashMap<EntityId, NotifyCause>,
}

impl WindowDevToolsState {
    fn new() -> Self {
        Self {
            recent_frames: RingBuffer::new(WINDOW_FRAME_CAPACITY),
            view_bounds: FxHashMap::default(),
            latest_dirty_cause_by_entity: FxHashMap::default(),
        }
    }
}

#[derive(Clone, Debug)]
struct RingBuffer<T> {
    capacity: usize,
    entries: VecDeque<T>,
}

impl<T> RingBuffer<T> {
    fn new(capacity: usize) -> Self {
        Self {
            capacity,
            entries: VecDeque::with_capacity(capacity),
        }
    }

    fn push(&mut self, value: T) {
        if self.capacity == 0 {
            return;
        }

        if self.entries.len() == self.capacity {
            self.entries.pop_front();
        }
        self.entries.push_back(value);
    }

    fn iter(&self) -> impl DoubleEndedIterator<Item = &T> {
        self.entries.iter()
    }

    fn clear(&mut self) {
        self.entries.clear();
    }

    #[cfg(test)]
    fn len(&self) -> usize {
        self.entries.len()
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct NotifySourceKey {
    entity_type: &'static str,
    caller_file: &'static str,
    caller_line: u32,
}

impl NotifySourceKey {
    fn from(event: &NotifyEvent) -> Self {
        Self {
            entity_type: event.entity_type,
            caller_file: event.caller_file,
            caller_line: event.caller_line,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct NotifyCause {
    source: NotifySourceKey,
    entity_id: EntityId,
    caller_column: u32,
    timestamp: Instant,
}

impl NotifyCause {
    fn from_event(event: &NotifyEvent) -> Self {
        Self {
            source: NotifySourceKey::from(event),
            entity_id: event.entity_id,
            caller_column: event.caller_column,
            timestamp: event.timestamp,
        }
    }

    fn is_recent_at(self, timestamp: Instant, max_age: Duration) -> bool {
        event_age(timestamp, self.timestamp).is_some_and(|age| age <= max_age)
    }
}

#[derive(Clone, Copy, Debug)]
struct NotifySourceStats {
    count: usize,
    entity_id: EntityId,
    caller_column: u32,
    registered_window_count: usize,
    live_window_count: usize,
    last_timestamp: Option<Instant>,
}

impl NotifySourceStats {
    fn from_event(event: &NotifyEvent) -> Self {
        Self {
            count: 0,
            entity_id: event.entity_id,
            caller_column: event.caller_column,
            registered_window_count: event.registered_window_count,
            live_window_count: event.live_window_count,
            last_timestamp: Some(event.timestamp),
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

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct RenderSourceKey {
    entity_id: EntityId,
    entity_type: &'static str,
    phase: ViewRenderPhase,
}

impl RenderSourceKey {
    fn from(event: &ViewRenderEvent) -> Self {
        Self {
            entity_id: event.entity_id,
            entity_type: event.entity_type,
            phase: event.phase,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct RenderSourceStats {
    count: usize,
    reuse_count: usize,
    duration: Duration,
    cache_miss_reasons: CacheMissReasons,
    caching_disabled_by_inspector: bool,
    last_timestamp: Option<Instant>,
    cause: Option<NotifyCause>,
}

impl RenderSourceStats {
    fn from_event(event: &ViewRenderEvent) -> Self {
        Self {
            count: 0,
            reuse_count: 0,
            duration: Duration::default(),
            cache_miss_reasons: event.cache_miss_reasons,
            caching_disabled_by_inspector: event.caching_disabled_by_inspector,
            last_timestamp: Some(event.timestamp),
            cause: None,
        }
    }

    fn record_event(&mut self, event: &ViewRenderEvent) {
        self.count += 1;
        if let Some(duration) = event.duration {
            self.duration += duration;
        }
        self.cache_miss_reasons = event.cache_miss_reasons;
        self.caching_disabled_by_inspector = event.caching_disabled_by_inspector;
        self.last_timestamp = Some(event.timestamp);
    }

    fn average_duration(self) -> Duration {
        if self.count == 0 {
            return Duration::default();
        }

        let average_nanos =
            (self.duration.as_nanos() / self.count as u128).min(u64::MAX as u128) as u64;
        Duration::from_nanos(average_nanos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn devtools_ring_buffer_drops_oldest_entries() {
        let mut buffer = RingBuffer::new(3);
        buffer.push(1);
        buffer.push(2);
        buffer.push(3);
        buffer.push(4);

        assert_eq!(buffer.len(), 3);
        assert_eq!(buffer.iter().copied().collect::<Vec<_>>(), vec![2, 3, 4]);
    }
}
