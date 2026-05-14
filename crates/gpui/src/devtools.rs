//! Runtime GPUI invalidation and rendering diagnostics.

use crate::{
    App, BorderStyle, Bounds, DispatchPhase, ElementId, EntityId, Hitbox, HitboxBehavior, Hsla,
    MouseButton, MouseDownEvent, Pixels, Point, SceneStats, SharedString, TextAlign, TextRun,
    Window, WindowId, fill, font, hsla, outline, point, px, rgba, size,
};
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

const NOTIFICATION_CAPACITY: usize = 4096;
const FRAME_CAPACITY: usize = 2048;
const VIEW_RENDER_CAPACITY: usize = 8192;
const DIRTY_PATH_CAPACITY: usize = 4096;
const ANIMATION_CAPACITY: usize = 4096;
const WINDOW_FRAME_CAPACITY: usize = 240;
const FRAME_RATE_WINDOW: Duration = Duration::from_secs(1);
const SOURCE_WINDOW: Duration = Duration::from_secs(5);
const ANIMATION_EXPIRY: Duration = Duration::from_secs(1);
const TOP_SOURCE_COUNT: usize = 3;
const HUD_MAX_LINE_CHARS: usize = 96;

static GPUI_DEVTOOLS_ENABLED: AtomicBool = AtomicBool::new(false);
static GPUI_DEVTOOLS: LazyLock<RwLock<GpuiDevTools>> =
    LazyLock::new(|| RwLock::new(GpuiDevTools::new()));

/// Opens the GPUI devtools overlay for the given window.
pub fn open(window: &mut Window) {
    let was_enabled = GPUI_DEVTOOLS_ENABLED.swap(true, SeqCst);
    let window_id = window.handle.window_id();
    {
        let mut devtools = GPUI_DEVTOOLS.write();
        devtools.open_window(window_id);
        devtools.resume();
        if !was_enabled {
            devtools.clear_counters();
        }
    }
    window.refresh();
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
        devtools.has_open_windows()
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

pub(crate) fn prepaint_window_overlay(window: &mut Window) {
    if !enabled() || !window_open(window.handle.window_id()) {
        return;
    }

    let window_id = window.handle.window_id();
    let snapshot = {
        let devtools = GPUI_DEVTOOLS.read();
        snapshot_overlay(&devtools, window_id)
    };
    let prepared_overlay = prepare_overlay(window, snapshot);
    GPUI_DEVTOOLS
        .write()
        .window_state(window_id)
        .prepared_overlay = Some(prepared_overlay);
}

pub(crate) fn paint_window_overlay(window: &mut Window, cx: &mut App) {
    if !enabled() || !window_open(window.handle.window_id()) {
        return;
    }

    let window_id = window.handle.window_id();
    let prepared_overlay = GPUI_DEVTOOLS
        .read()
        .windows
        .get(&window_id)
        .and_then(|window_state| window_state.prepared_overlay.clone());
    let Some(prepared_overlay) = prepared_overlay else {
        return;
    };

    paint_overlay(window, cx, &prepared_overlay);
    register_input_handlers(window, &prepared_overlay);
}

fn close_window(window: &mut Window) {
    let window_id = window.handle.window_id();
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
    window.refresh();
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
        if let Some(window_state) = self.windows.get_mut(&window_id) {
            window_state.prepared_overlay = None;
        }
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
    prepared_overlay: Option<PreparedOverlay>,
}

impl WindowDevToolsState {
    fn new() -> Self {
        Self {
            recent_frames: RingBuffer::new(WINDOW_FRAME_CAPACITY),
            view_bounds: FxHashMap::default(),
            latest_dirty_cause_by_entity: FxHashMap::default(),
            prepared_overlay: None,
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

#[derive(Default)]
struct RenderSummary {
    render_count: usize,
    reuse_count: usize,
    top_sources: Vec<(RenderSourceKey, RenderSourceStats)>,
}

fn top_notify_sources(
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
        let stats = counts
            .entry(key)
            .or_insert_with(|| NotifySourceStats::from_event(event));
        stats.count += 1;
        stats.update_from_event(event);
    }

    let mut counts = counts.into_iter().collect::<Vec<_>>();
    counts.sort_by(|(left_source, left), (right_source, right)| {
        right
            .count
            .cmp(&left.count)
            .then_with(|| {
                short_type_name(left_source.entity_type)
                    .cmp(short_type_name(right_source.entity_type))
            })
            .then_with(|| left_source.caller_file.cmp(right_source.caller_file))
            .then_with(|| left_source.caller_line.cmp(&right_source.caller_line))
    });
    counts.truncate(limit);
    counts
}

fn render_summary(
    devtools: &GpuiDevTools,
    window_id: WindowId,
    now: Instant,
    limit: usize,
) -> RenderSummary {
    let mut summary = RenderSummary::default();
    let mut counts: FxHashMap<RenderSourceKey, RenderSourceStats> = FxHashMap::default();
    let mut reuse_counts_by_entity = FxHashMap::default();

    for event in devtools.renders.iter() {
        let Some(age) = event_age(now, event.timestamp) else {
            continue;
        };
        if event.window_id != window_id || age > FRAME_RATE_WINDOW {
            continue;
        }

        if event.phase.is_reuse() {
            summary.reuse_count += 1;
            *reuse_counts_by_entity.entry(event.entity_id).or_insert(0) += 1;
        } else if event.phase.records_render() {
            summary.render_count += 1;
            counts
                .entry(RenderSourceKey::from(event))
                .or_insert_with(|| RenderSourceStats::from_event(event))
                .record_event(event);
        }
    }

    for (source, stats) in &mut counts {
        stats.reuse_count = reuse_counts_by_entity
            .get(&source.entity_id)
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
        right
            .duration
            .cmp(&left.duration)
            .then_with(|| right.count.cmp(&left.count))
            .then_with(|| {
                short_type_name(left_source.entity_type)
                    .cmp(short_type_name(right_source.entity_type))
            })
            .then_with(|| {
                left_source
                    .entity_id
                    .as_u64()
                    .cmp(&right_source.entity_id.as_u64())
            })
            .then_with(|| left_source.phase.as_str().cmp(right_source.phase.as_str()))
    });
    counts.truncate(limit);
    summary.top_sources = counts;

    summary
}

fn active_animation_count(devtools: &GpuiDevTools, window_id: WindowId, now: Instant) -> usize {
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

#[derive(Clone, Debug)]
struct OverlaySnapshot {
    rows: Vec<OverlayRow>,
}

#[derive(Clone, Debug)]
struct PreparedOverlay {
    snapshot: OverlaySnapshot,
    hud_bounds: Bounds<Pixels>,
    row_hitboxes: Vec<OverlayRowHitbox>,
}

#[derive(Clone, Debug)]
struct OverlayRowHitbox {
    hitbox: Hitbox,
    action: OverlayAction,
}

#[derive(Clone, Debug)]
struct OverlayRow {
    text: String,
    kind: OverlayRowKind,
    actions: Vec<OverlayAction>,
}

impl OverlayRow {
    fn header(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            kind: OverlayRowKind::Header,
            actions: Vec::new(),
        }
    }

    fn toolbar(devtools: &GpuiDevTools) -> Self {
        let pause_action = if devtools.paused_at.is_some() {
            OverlayAction::toolbar("resume", true, OverlayActionKind::Resume)
        } else {
            OverlayAction::toolbar("pause", false, OverlayActionKind::Pause)
        };

        Self {
            text: String::new(),
            kind: OverlayRowKind::Toolbar,
            actions: vec![
                pause_action,
                OverlayAction::toolbar("clear", false, OverlayActionKind::Clear),
                OverlayAction::toolbar("close", false, OverlayActionKind::Close),
            ],
        }
    }

    fn plain(text: impl Into<String>) -> Self {
        Self {
            text: truncate_chars(&text.into(), HUD_MAX_LINE_CHARS),
            kind: OverlayRowKind::Data,
            actions: Vec::new(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum OverlayRowKind {
    Header,
    Toolbar,
    Data,
}

#[derive(Clone, Copy, Debug)]
struct OverlayAction {
    label: &'static str,
    active: bool,
    kind: OverlayActionKind,
}

impl OverlayAction {
    fn toolbar(label: &'static str, active: bool, kind: OverlayActionKind) -> Self {
        Self {
            label,
            active,
            kind,
        }
    }

    fn width(self) -> Pixels {
        px((self.label.len() as f32 * 6.5 + 18.).clamp(52., 104.))
    }
}

#[derive(Clone, Copy, Debug)]
enum OverlayActionKind {
    Pause,
    Resume,
    Clear,
    Close,
}

fn snapshot_overlay(devtools: &GpuiDevTools, window_id: WindowId) -> OverlaySnapshot {
    let now = devtools.paused_at.unwrap_or_else(Instant::now);
    let mut rows = Vec::new();
    rows.push(OverlayRow::header(if devtools.paused_at.is_some() {
        "GPUI profiler paused"
    } else {
        "GPUI profiler"
    }));
    rows.push(OverlayRow::toolbar(devtools));

    let (frame_count, draw_count, dirty_frame_count, last_frame) =
        frame_summary(devtools, window_id, now);
    rows.push(OverlayRow::plain(format!(
        "draw/s {:>3} dirty/s {:>3} frame/s {:>3}",
        draw_count, dirty_frame_count, frame_count
    )));
    if let Some(frame) = last_frame {
        let draw_duration = frame
            .draw_duration
            .map(format_duration_ms)
            .unwrap_or_else(|| "--".to_string());
        rows.push(OverlayRow::plain(format!(
            "last {} draw {}ms present {}ms views {} updates {} ops {} quads {}",
            frame.reason,
            draw_duration,
            format_duration_ms(frame.present_duration),
            frame.dirty_view_count,
            frame.invalidator_update_count,
            frame.scene_stats.paint_operation_count,
            frame.scene_stats.quad_count
        )));
    } else {
        rows.push(OverlayRow::plain("last frame --"));
    }

    let render_summary = render_summary(devtools, window_id, now, TOP_SOURCE_COUNT);
    rows.push(OverlayRow::plain(format!(
        "renders/s {} reuse/s {}",
        render_summary.render_count, render_summary.reuse_count
    )));
    if render_summary.top_sources.is_empty() {
        rows.push(OverlayRow::plain("render: no recent uncached view renders"));
    } else {
        for (index, (source, stats)) in render_summary.top_sources.into_iter().enumerate() {
            let mut labels = stats.cache_miss_reasons.labels();
            if stats.caching_disabled_by_inspector {
                labels.push("inspector");
            }
            let labels = if labels.is_empty() {
                String::new()
            } else {
                format!(" [{}]", labels.join(","))
            };
            let cause = stats
                .cause
                .map(|cause| {
                    format!(
                        " cause {} {}:{}:{}",
                        view_label(cause.source.entity_type, cause.entity_id),
                        file_name(cause.source.caller_file),
                        cause.source.caller_line,
                        cause.caller_column
                    )
                })
                .unwrap_or_default();
            rows.push(OverlayRow::plain(format!(
                "render #{:<2} {:<24} {:<12} x{:<3} avg {:>6}ms sum {:>6}ms reuse {:>3}{}{}",
                index + 1,
                view_label(source.entity_type, source.entity_id),
                source.phase.as_str(),
                stats.count,
                format_duration_ms(stats.average_duration()),
                format_duration_ms(stats.duration),
                stats.reuse_count,
                labels,
                cause
            )));
        }
    }

    let notify_sources = top_notify_sources(devtools, now, TOP_SOURCE_COUNT);
    if notify_sources.is_empty() {
        rows.push(OverlayRow::plain("notify: no recent notifications"));
    } else {
        for (index, (source, stats)) in notify_sources.into_iter().enumerate() {
            rows.push(OverlayRow::plain(format!(
                "notify #{:<2} {:<24} x{:<3} {}:{}:{} live {}/{} total {}",
                index + 1,
                view_label(source.entity_type, stats.entity_id),
                stats.count,
                file_name(source.caller_file),
                source.caller_line,
                stats.caller_column,
                stats.live_window_count,
                stats.registered_window_count,
                devtools
                    .notify_source_total_counts
                    .get(&source)
                    .copied()
                    .unwrap_or(0)
            )));
        }
    }

    let dirty_path_summary = recent_dirty_path_summary(devtools, window_id, now);
    rows.push(OverlayRow::plain(format!(
        "dirty paths/s {} {}active animations {}",
        dirty_path_summary.count,
        dirty_path_summary
            .last_label
            .as_ref()
            .map(|label| format!("last {label} "))
            .unwrap_or_default(),
        active_animation_count(devtools, window_id, now)
    )));

    OverlaySnapshot { rows }
}

fn frame_summary(
    devtools: &GpuiDevTools,
    window_id: WindowId,
    now: Instant,
) -> (usize, usize, usize, Option<FrameEvent>) {
    let mut frame_count = 0;
    let mut draw_count = 0;
    let mut dirty_frame_count = 0;
    let mut last_frame = None;
    if let Some(window_state) = devtools.windows.get(&window_id) {
        for frame in window_state.recent_frames.iter() {
            let Some(age) = event_age(now, frame.timestamp) else {
                continue;
            };
            if age <= FRAME_RATE_WINDOW {
                frame_count += 1;
                draw_count += usize::from(frame.rebuilt_scene);
                dirty_frame_count += usize::from(frame.dirty_before_frame);
            }
        }
        last_frame = window_state
            .recent_frames
            .iter()
            .rev()
            .find(|frame| frame.timestamp <= now)
            .cloned();
    }
    (frame_count, draw_count, dirty_frame_count, last_frame)
}

#[derive(Default)]
struct DirtyPathSummary {
    count: usize,
    last_label: Option<String>,
}

fn recent_dirty_path_summary(
    devtools: &GpuiDevTools,
    window_id: WindowId,
    now: Instant,
) -> DirtyPathSummary {
    let mut summary = DirtyPathSummary::default();
    for event in devtools.dirty_paths.iter() {
        if event.window_id != window_id
            || !event_age(now, event.timestamp).is_some_and(|age| age <= FRAME_RATE_WINDOW)
        {
            continue;
        }

        summary.count += 1;
        summary.last_label = Some(dirty_path_label(event));
    }
    summary
}

fn dirty_path_label(event: &DirtyPathEvent) -> String {
    if event.path.is_empty() {
        return format!(
            "{}#{}",
            short_type_name(event.invalidated_entity_type),
            event.invalidated_entity_id.as_u64()
        );
    }

    let mut path = event
        .path
        .iter()
        .take(2)
        .map(|segment| view_label(segment.entity_type, segment.entity_id))
        .collect::<Vec<_>>()
        .join("<");
    if event.path.len() > 2 {
        path.push_str("<...");
    }
    format!(
        "{}#{} {}",
        short_type_name(event.invalidated_entity_type),
        event.invalidated_entity_id.as_u64(),
        path
    )
}

fn prepare_overlay(window: &mut Window, snapshot: OverlaySnapshot) -> PreparedOverlay {
    let hud_bounds = hud_bounds(snapshot.rows.len(), window.viewport_size());
    let mut row_hitboxes = Vec::new();
    for (row_index, row) in snapshot.rows.iter().enumerate() {
        for (action_index, action) in row.actions.iter().copied().enumerate() {
            let hitbox = window.insert_hitbox(
                hud_button_bounds(hud_bounds, row_index, &row.actions, action_index),
                HitboxBehavior::BlockMouse,
            );
            row_hitboxes.push(OverlayRowHitbox { hitbox, action });
        }
    }

    PreparedOverlay {
        snapshot,
        hud_bounds,
        row_hitboxes,
    }
}

fn paint_overlay(window: &mut Window, cx: &mut App, prepared_overlay: &PreparedOverlay) {
    if prepared_overlay.snapshot.rows.is_empty() {
        return;
    }

    let padding = hud_padding();
    let line_height = hud_line_height();
    let bounds = prepared_overlay.hud_bounds;

    window.paint_quad(fill(bounds, rgba(0x111827dd)));
    window.paint_quad(outline(
        bounds,
        hsla(0.58, 0.68, 0.68, 0.72),
        BorderStyle::default(),
    ));

    for (line_index, row) in prepared_overlay.snapshot.rows.iter().enumerate() {
        let row_bounds = hud_row_bounds(bounds, line_index);
        match row.kind {
            OverlayRowKind::Header | OverlayRowKind::Toolbar => {
                window.paint_quad(fill(row_bounds, rgba(0x273244aa)));
            }
            OverlayRowKind::Data => {}
        }
    }

    for row_hitbox in &prepared_overlay.row_hitboxes {
        let fill_color = if row_hitbox.hitbox.is_hovered(window) {
            rgba(0x38bdf84a)
        } else if row_hitbox.action.active {
            rgba(0x0ea5e94a)
        } else {
            rgba(0x1f29374a)
        };
        window.paint_quad(fill(row_hitbox.hitbox.bounds, fill_color));
        window.paint_quad(outline(
            row_hitbox.hitbox.bounds,
            hsla(0.58, 0.68, 0.68, 0.52),
            BorderStyle::default(),
        ));
        paint_text_line_with_color(
            window,
            cx,
            point(
                row_hitbox.hitbox.origin.x + px(5.),
                row_hitbox.hitbox.origin.y + px(1.),
            ),
            row_hitbox.action.label,
            line_height,
            hsla(0.58, 0.38, 0.98, 0.98),
        );
    }

    for (line_index, row) in prepared_overlay.snapshot.rows.iter().enumerate() {
        let text_color = match row.kind {
            OverlayRowKind::Header => hsla(0.58, 0.44, 0.94, 1.),
            OverlayRowKind::Toolbar => hsla(0.12, 0.62, 0.76, 1.),
            OverlayRowKind::Data => hsla(0.58, 0.38, 0.92, 0.96),
        };
        let origin = point(
            bounds.origin.x + padding + hud_action_text_offset(&row.actions),
            bounds.origin.y + padding + line_height * (line_index as f32),
        );
        paint_text_line_with_color(window, cx, origin, &row.text, line_height, text_color);
    }
}

fn register_input_handlers(window: &mut Window, prepared_overlay: &PreparedOverlay) {
    for row_hitbox in prepared_overlay.row_hitboxes.iter().cloned() {
        let hitbox = row_hitbox.hitbox;
        let action = row_hitbox.action;
        window.on_mouse_event(move |event: &MouseDownEvent, phase, window, cx| {
            if phase == DispatchPhase::Bubble
                && event.button == MouseButton::Left
                && hitbox.is_hovered(window)
            {
                apply_overlay_action(action.kind, window);
                window.prevent_default();
                window.refresh();
                cx.stop_propagation();
            }
        });
    }
}

fn apply_overlay_action(action: OverlayActionKind, window: &mut Window) {
    match action {
        OverlayActionKind::Pause => GPUI_DEVTOOLS.write().pause(Instant::now()),
        OverlayActionKind::Resume => GPUI_DEVTOOLS.write().resume(),
        OverlayActionKind::Clear => GPUI_DEVTOOLS.write().clear_counters(),
        OverlayActionKind::Close => close_window(window),
    }
}

fn hud_bounds(row_count: usize, viewport_size: crate::Size<Pixels>) -> Bounds<Pixels> {
    let margin = px(12.);
    let padding = hud_padding();
    let hud_width = px(560.);
    let line_height = hud_line_height();
    let hud_height = padding * 2. + line_height * (row_count as f32);
    let hud_size = size(hud_width, hud_height);
    let origin = point(
        (viewport_size.width - hud_width - margin).max(margin),
        margin,
    );
    Bounds::new(origin, hud_size)
}

fn hud_button_bounds(
    hud_bounds: Bounds<Pixels>,
    row_index: usize,
    actions: &[OverlayAction],
    action_index: usize,
) -> Bounds<Pixels> {
    let padding = hud_padding();
    let line_height = hud_line_height();
    let action_offset = (0..action_index).fold(px(0.), |offset, i| {
        offset + actions[i].width() + hud_button_gap()
    });
    let button_width = actions
        .get(action_index)
        .map(|action| action.width())
        .unwrap_or(px(0.));
    Bounds::new(
        point(
            hud_bounds.origin.x + padding - px(2.) + action_offset,
            hud_bounds.origin.y + padding + line_height * (row_index as f32) - px(1.),
        ),
        size(button_width, line_height),
    )
}

fn hud_row_bounds(hud_bounds: Bounds<Pixels>, row_index: usize) -> Bounds<Pixels> {
    let padding = hud_padding();
    let line_height = hud_line_height();
    Bounds::new(
        point(
            hud_bounds.origin.x + padding - px(2.),
            hud_bounds.origin.y + padding + line_height * (row_index as f32) - px(1.),
        ),
        size(hud_bounds.size.width - padding * 2. + px(4.), line_height),
    )
}

fn hud_action_text_offset(actions: &[OverlayAction]) -> Pixels {
    if actions.is_empty() {
        px(0.)
    } else {
        actions.iter().fold(px(0.), |offset, action| {
            offset + action.width() + hud_button_gap()
        }) + px(3.)
    }
}

fn hud_padding() -> Pixels {
    px(8.)
}

fn hud_line_height() -> Pixels {
    px(14.)
}

fn hud_button_gap() -> Pixels {
    px(4.)
}

fn paint_text_line_with_color(
    window: &mut Window,
    cx: &mut App,
    origin: Point<Pixels>,
    line: &str,
    line_height: Pixels,
    color: Hsla,
) {
    let font_size = px(11.);
    let text_run = TextRun {
        len: line.len(),
        font: font(".SystemUIFont"),
        color,
        ..TextRun::default()
    };
    let shaped_line = window.text_system().shape_line(
        SharedString::from(line.to_string()),
        font_size,
        &[text_run],
        None,
    );
    if let Err(error) = shaped_line.paint(origin, line_height, TextAlign::Left, None, window, cx) {
        log::debug!("failed to paint GPUI profiler HUD text: {error:?}");
    }
}

fn format_duration_ms(duration: Duration) -> String {
    let ms = duration.as_secs_f64() * 1000.;
    if ms < 0.1 {
        format!("{ms:.2}")
    } else if ms < 10. {
        format!("{ms:.1}")
    } else {
        format!("{ms:.0}")
    }
}

fn view_label(entity_type: &'static str, entity_id: EntityId) -> String {
    format!("{}#{}", short_type_name(entity_type), entity_id.as_u64())
}

fn short_type_name(name: &'static str) -> &'static str {
    name.rsplit("::").next().unwrap_or(name)
}

fn file_name(path: &'static str) -> &'static str {
    path.rsplit('/').next().unwrap_or(path)
}

fn truncate_chars(text: &str, max_chars: usize) -> String {
    if text.chars().count() <= max_chars {
        text.to_string()
    } else {
        let mut truncated = text
            .chars()
            .take(max_chars.saturating_sub(3))
            .collect::<String>();
        truncated.push_str("...");
        truncated
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

    #[test]
    fn devtools_render_summary_keeps_same_type_views_separate() {
        let mut devtools = GpuiDevTools::new();
        let now = Instant::now();
        let window_id = WindowId::from(1);
        for (entity_id, duration) in [(1, 1), (1, 1), (2, 3)] {
            devtools.renders.push(ViewRenderEvent {
                window_id,
                entity_id: EntityId::from(entity_id),
                entity_type: "Editor",
                phase: ViewRenderPhase::UncachedRender,
                duration: Some(Duration::from_millis(duration)),
                cache_miss_reasons: CacheMissReasons::empty(),
                bounds: None,
                caching_disabled_by_inspector: false,
                timestamp: now,
            });
        }

        let summary = render_summary(&devtools, window_id, now, TOP_SOURCE_COUNT);
        assert_eq!(summary.render_count, 3);
        assert_eq!(summary.top_sources.len(), 2);
        assert_eq!(summary.top_sources[0].0.entity_id, EntityId::from(2));
        assert_eq!(summary.top_sources[1].0.entity_id, EntityId::from(1));
    }

    #[test]
    fn devtools_notify_sources_respect_limit() {
        let mut devtools = GpuiDevTools::new();
        let now = Instant::now();
        for index in 0..4 {
            devtools.notifications.push(NotifyEvent {
                entity_id: EntityId::from((index + 1) as u64),
                entity_type: "Editor",
                caller_file: "crates/editor/src/editor.rs",
                caller_line: index as u32,
                caller_column: 1,
                registered_window_count: 1,
                live_window_count: 1,
                timestamp: now,
            });
        }

        assert_eq!(top_notify_sources(&devtools, now, 3).len(), 3);
    }
}
