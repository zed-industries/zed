use crate::{
    App, BorderStyle, Bounds, DispatchPhase, ElementId, EntityId, Hitbox, HitboxBehavior,
    MouseButton, MouseDownEvent, Pixels, Point, SceneStats, SharedString, TextAlign, TextRun,
    Window, WindowId, fill, font, hsla, outline, point, px, quad, rgba, size,
};
use collections::{FxHashMap, FxHashSet, VecDeque};
use parking_lot::RwLock;
use scheduler::Instant;
use std::{sync::LazyLock, time::Duration};

const NOTIFICATION_CAPACITY: usize = 4096;
const FRAME_CAPACITY: usize = 2048;
const VIEW_RENDER_CAPACITY: usize = 8192;
const DIRTY_PATH_CAPACITY: usize = 4096;
const ANIMATION_CAPACITY: usize = 4096;
const WINDOW_FRAME_CAPACITY: usize = 240;
const FLASH_DURATION: Duration = Duration::from_millis(200);
const FRAME_RATE_WINDOW: Duration = Duration::from_secs(1);
const SOURCE_WINDOW: Duration = Duration::from_secs(5);
const ANIMATION_EXPIRY: Duration = Duration::from_secs(1);
const TOP_SOURCE_COUNT: usize = 3;
const HUD_MAX_LINE_CHARS: usize = 68;
const DEFAULT_PINNED_NOTIFY_SOURCE: &str = "Editor editor.rs:2111";

static GPUI_DEVTOOLS_ENABLED: LazyLock<bool> =
    LazyLock::new(|| std::env::var_os("ZED_GPUI_DEVTOOLS").is_some());

static GPUI_DEVTOOLS: LazyLock<RwLock<GpuiDevTools>> =
    LazyLock::new(|| RwLock::new(GpuiDevTools::new()));

static PINNED_NOTIFY_SOURCE: LazyLock<Option<PinnedNotifySource>> = LazyLock::new(|| {
    let source = std::env::var("ZED_GPUI_DEVTOOLS_PIN_NOTIFY")
        .unwrap_or_else(|_| DEFAULT_PINNED_NOTIFY_SOURCE.to_string());
    parse_pinned_notify_source(&source)
});

pub(crate) fn enabled() -> bool {
    *GPUI_DEVTOOLS_ENABLED
}

pub(crate) fn record_notify(event: NotifyEvent) {
    if !enabled() {
        return;
    }

    let mut devtools = GPUI_DEVTOOLS.write();
    if PINNED_NOTIFY_SOURCE
        .as_ref()
        .is_some_and(|pinned_source| pinned_source.matches(&event))
    {
        devtools.pinned_notify_total_count += 1;
    }
    devtools.notifications.push(event);
}

pub(crate) fn record_frame(event: FrameEvent) {
    if !enabled() {
        return;
    }

    let mut devtools = GPUI_DEVTOOLS.write();
    let window_id = event.window_id;
    devtools.frames.push(event.clone());
    devtools.window_state(window_id).recent_frames.push(event);
}

pub(crate) fn record_dirty_path(event: DirtyPathEvent) {
    if !enabled() {
        return;
    }

    GPUI_DEVTOOLS.write().dirty_paths.push(event);
}

pub(crate) fn record_view_bounds(window_id: WindowId, entity_id: EntityId, bounds: Bounds<Pixels>) {
    if !enabled() {
        return;
    }

    GPUI_DEVTOOLS
        .write()
        .window_state(window_id)
        .view_bounds
        .insert(entity_id, bounds);
}

pub(crate) fn record_view_render(event: ViewRenderEvent) {
    if !enabled() {
        return;
    }

    let mut devtools = GPUI_DEVTOOLS.write();
    let source = RenderSourceKey::from(&event);
    let source_is_hidden = devtools.hidden_render_sources.contains(&source);
    let window_state = devtools.window_state(event.window_id);
    if let Some(bounds) = event.bounds {
        window_state.view_bounds.insert(event.entity_id, bounds);
    }
    if event.phase.flashes() && !source_is_hidden {
        window_state.active_flashes.insert(
            event.entity_id,
            FlashState {
                timestamp: event.timestamp,
                source,
            },
        );
    }
    devtools.renders.push(event);
}

pub(crate) fn record_animation(event: AnimationEvent) {
    if !enabled() {
        return;
    }

    GPUI_DEVTOOLS.write().animations.push(event);
}

pub(crate) fn prepaint_window_overlay(window: &mut Window) {
    if !enabled() {
        return;
    }

    let window_id = window.handle.window_id();
    let snapshot = overlay_snapshot(window_id);
    let prepared_overlay = prepaint_overlay(window, snapshot);

    GPUI_DEVTOOLS
        .write()
        .window_state(window_id)
        .prepared_overlay = Some(prepared_overlay);
}

pub(crate) fn paint_window_overlay(window: &mut Window, cx: &mut App) {
    if !enabled() {
        return;
    }

    let window_id = window.handle.window_id();
    let prepared_overlay = GPUI_DEVTOOLS
        .write()
        .windows
        .get_mut(&window_id)
        .and_then(|window_state| window_state.prepared_overlay.take());

    let Some(prepared_overlay) = prepared_overlay else {
        return;
    };

    for flash in &prepared_overlay.snapshot.flashes {
        window.paint_quad(quad(
            flash.bounds,
            px(2.),
            hsla(0.54, 0.96, 0.52, 0.10 * flash.opacity),
            px(1.),
            hsla(0.54, 0.96, 0.62, 0.78 * flash.opacity),
            BorderStyle::default(),
        ));
    }

    paint_hud(window, cx, &prepared_overlay);
}

#[derive(Clone, Debug)]
pub(crate) struct NotifyEvent {
    pub(crate) entity_id: EntityId,
    pub(crate) entity_type: &'static str,
    pub(crate) caller_file: &'static str,
    pub(crate) caller_line: u32,
    pub(crate) caller_column: u32,
    pub(crate) registered_window_count: usize,
    pub(crate) live_window_count: usize,
    pub(crate) timestamp: Instant,
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
    pub(crate) devtools_induced: bool,
    pub(crate) timestamp: Instant,
}

#[derive(Clone, Debug)]
pub(crate) struct DirtyPathSegment {
    pub(crate) entity_id: EntityId,
    pub(crate) entity_type: &'static str,
}

#[derive(Clone, Debug)]
pub(crate) struct DirtyPathEvent {
    pub(crate) window_id: WindowId,
    pub(crate) invalidated_entity_id: EntityId,
    pub(crate) invalidated_entity_type: &'static str,
    pub(crate) path: Vec<DirtyPathSegment>,
    pub(crate) timestamp: Instant,
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
    fn flashes(self) -> bool {
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
    pub(crate) window_id: WindowId,
    pub(crate) entity_id: EntityId,
    pub(crate) entity_type: &'static str,
    pub(crate) kind: AnimationEventKind,
    pub(crate) timestamp: Instant,
}

#[derive(Clone, Debug)]
pub(crate) enum AnimationEventKind {
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

#[derive(Debug)]
struct GpuiDevTools {
    notifications: RingBuffer<NotifyEvent>,
    frames: RingBuffer<FrameEvent>,
    renders: RingBuffer<ViewRenderEvent>,
    dirty_paths: RingBuffer<DirtyPathEvent>,
    animations: RingBuffer<AnimationEvent>,
    windows: FxHashMap<WindowId, WindowDevToolsState>,
    hidden_notify_sources: FxHashSet<NotifySourceKey>,
    hidden_render_sources: FxHashSet<RenderSourceKey>,
    pinned_notify_total_count: usize,
}

impl GpuiDevTools {
    fn new() -> Self {
        Self {
            notifications: RingBuffer::new(NOTIFICATION_CAPACITY),
            frames: RingBuffer::new(FRAME_CAPACITY),
            renders: RingBuffer::new(VIEW_RENDER_CAPACITY),
            dirty_paths: RingBuffer::new(DIRTY_PATH_CAPACITY),
            animations: RingBuffer::new(ANIMATION_CAPACITY),
            windows: FxHashMap::default(),
            hidden_notify_sources: FxHashSet::default(),
            hidden_render_sources: FxHashSet::default(),
            pinned_notify_total_count: 0,
        }
    }

    fn window_state(&mut self, window_id: WindowId) -> &mut WindowDevToolsState {
        self.windows
            .entry(window_id)
            .or_insert_with(WindowDevToolsState::new)
    }
}

#[derive(Debug)]
struct WindowDevToolsState {
    recent_frames: RingBuffer<FrameEvent>,
    view_bounds: FxHashMap<EntityId, Bounds<Pixels>>,
    active_flashes: FxHashMap<EntityId, FlashState>,
    prepared_overlay: Option<PreparedOverlay>,
}

impl WindowDevToolsState {
    fn new() -> Self {
        Self {
            recent_frames: RingBuffer::new(WINDOW_FRAME_CAPACITY),
            view_bounds: FxHashMap::default(),
            active_flashes: FxHashMap::default(),
            prepared_overlay: None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct FlashState {
    timestamp: Instant,
    source: RenderSourceKey,
}

#[derive(Clone, Debug)]
struct PinnedNotifySource {
    entity_type: String,
    caller_file: String,
    caller_line: u32,
}

impl PinnedNotifySource {
    fn matches(&self, event: &NotifyEvent) -> bool {
        event.caller_line == self.caller_line
            && (event.entity_type == self.entity_type
                || short_type_name(event.entity_type) == self.entity_type)
            && (event.caller_file.ends_with(&self.caller_file)
                || file_name(event.caller_file) == self.caller_file)
    }

    fn label(&self) -> String {
        format!(
            "{} {}:{}",
            self.entity_type,
            file_name(&self.caller_file),
            self.caller_line
        )
    }
}

fn parse_pinned_notify_source(source: &str) -> Option<PinnedNotifySource> {
    let source = source.trim();
    if source.is_empty()
        || source.eq_ignore_ascii_case("none")
        || source.eq_ignore_ascii_case("off")
    {
        return None;
    }

    let source = source.replace(',', " ").replace(':', " ");
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

    #[cfg(test)]
    fn len(&self) -> usize {
        self.entries.len()
    }

    fn last(&self) -> Option<&T> {
        self.entries.back()
    }
}

#[derive(Clone, Debug)]
struct OverlaySnapshot {
    flashes: Vec<FlashOverlay>,
    rows: Vec<OverlayRow>,
}

#[derive(Clone, Debug)]
struct FlashOverlay {
    bounds: Bounds<Pixels>,
    opacity: f32,
}

#[derive(Clone, Debug)]
struct OverlayRow {
    text: String,
    action: Option<SourceFilterAction>,
}

impl OverlayRow {
    fn plain(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            action: None,
        }
    }

    fn action(text: impl Into<String>, action: SourceFilterAction) -> Self {
        Self {
            text: text.into(),
            action: Some(action),
        }
    }

    fn truncate(mut self) -> Self {
        self.text = truncate_chars(&self.text, HUD_MAX_LINE_CHARS);
        self
    }
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
    action: SourceFilterAction,
}

#[derive(Clone, Copy, Debug)]
enum SourceFilterAction {
    HideNotify(NotifySourceKey),
    ShowNotify(NotifySourceKey),
    HideRender(RenderSourceKey),
    ShowRender(RenderSourceKey),
}

fn overlay_snapshot(window_id: WindowId) -> OverlaySnapshot {
    let now = Instant::now();
    let mut devtools = GPUI_DEVTOOLS.write();
    let hidden_render_sources = devtools.hidden_render_sources.clone();
    let flashes = devtools
        .windows
        .get_mut(&window_id)
        .map(|window_state| {
            window_state
                .active_flashes
                .retain(|_, flash| now.duration_since(flash.timestamp) <= FLASH_DURATION);

            window_state
                .active_flashes
                .iter()
                .filter_map(|(entity_id, flash)| {
                    if hidden_render_sources.contains(&flash.source) {
                        return None;
                    }

                    let bounds = window_state.view_bounds.get(entity_id).copied()?;
                    let elapsed = now.duration_since(flash.timestamp);
                    let opacity = 1. - elapsed.as_secs_f32() / FLASH_DURATION.as_secs_f32();
                    Some(FlashOverlay {
                        bounds,
                        opacity: opacity.clamp(0., 1.),
                    })
                })
                .collect()
        })
        .unwrap_or_default();

    let rows = hud_rows(&devtools, window_id, now);
    OverlaySnapshot { flashes, rows }
}

fn hud_rows(devtools: &GpuiDevTools, window_id: WindowId, now: Instant) -> Vec<OverlayRow> {
    let mut rows = Vec::new();
    rows.push(OverlayRow::plain("GPUI DevTools"));

    let mut frame_count = 0;
    let mut draw_count = 0;
    let mut dirty_frame_count = 0;
    let mut last_frame = None;
    if let Some(window_state) = devtools.windows.get(&window_id) {
        for frame in window_state.recent_frames.iter() {
            if now.duration_since(frame.timestamp) <= FRAME_RATE_WINDOW {
                frame_count += 1;
                draw_count += usize::from(frame.rebuilt_scene);
                dirty_frame_count += usize::from(frame.dirty_before_frame);
            }
        }
        last_frame = window_state.recent_frames.last();
    }

    rows.push(OverlayRow::plain(format!(
        "draw/s {:>3}  dirty/s {:>3}  frame/s {:>3}",
        draw_count, dirty_frame_count, frame_count
    )));

    if let Some(frame) = last_frame {
        let last_frame_age = now.duration_since(frame.timestamp);
        let draw_duration = frame
            .draw_duration
            .map(format_duration_ms)
            .unwrap_or_else(|| "--".to_string());
        rows.push(OverlayRow::plain(format!(
            "last {} age {}{} draw {}ms present {}ms",
            frame.reason,
            format_age(last_frame_age),
            if last_frame_age > FRAME_RATE_WINDOW {
                " idle"
            } else {
                ""
            },
            draw_duration,
            format_duration_ms(frame.present_duration),
        )));
        rows.push(OverlayRow::plain(format!(
            "views {} updates {} ops {} quads {}{}",
            frame.dirty_view_count,
            frame.invalidator_update_count,
            frame.scene_stats.paint_operation_count,
            frame.scene_stats.quad_count,
            if frame.devtools_induced {
                " devtools"
            } else {
                ""
            },
        )));
    } else {
        rows.push(OverlayRow::plain("last frame --"));
    }

    let notify_sources = top_notify_sources(devtools, now, TOP_SOURCE_COUNT);
    if notify_sources.is_empty() {
        rows.push(OverlayRow::plain("notify --"));
    } else {
        for (index, (source, stats)) in notify_sources.into_iter().enumerate() {
            rows.push(OverlayRow::action(
                format_notify_source(index + 1, source, stats),
                SourceFilterAction::HideNotify(source),
            ));
        }
    }

    if let Some(pinned_source) = PINNED_NOTIFY_SOURCE.as_ref() {
        rows.push(OverlayRow::plain(format!(
            "pin {} 5s {} total {}",
            pinned_source.label(),
            pinned_notify_recent_count(devtools, now, pinned_source),
            devtools.pinned_notify_total_count
        )));
    }

    if let Some((label, count)) = top_dirty_path(devtools, window_id, now) {
        rows.push(OverlayRow::plain(format!("dirty {} x{}", label, count)));
    } else {
        rows.push(OverlayRow::plain("dirty --"));
    }

    let render_summary = render_summary(devtools, window_id, now);
    rows.push(OverlayRow::plain(format!(
        "renders/s {} reuse/s {}",
        render_summary.render_count, render_summary.reuse_count
    )));
    if render_summary.top_sources.is_empty() {
        rows.push(OverlayRow::plain("render --"));
    } else {
        for (index, (source, stats)) in render_summary.top_sources.into_iter().enumerate() {
            rows.push(OverlayRow::action(
                format_render_source(index + 1, source, stats),
                SourceFilterAction::HideRender(source),
            ));
        }
    }

    rows.push(OverlayRow::plain(format!(
        "active animations {}",
        active_animation_count(devtools, window_id, now)
    )));

    let hidden_notify_sources = hidden_notify_sources(devtools, now);
    let hidden_render_sources = hidden_render_sources(devtools, window_id, now);
    if !hidden_notify_sources.is_empty() || !hidden_render_sources.is_empty() {
        rows.push(OverlayRow::plain("hidden filters"));
        for (source, count) in hidden_notify_sources {
            rows.push(OverlayRow::action(
                format!(
                    "[+] notify {} {}:{} 5s {}",
                    short_type_name(source.entity_type),
                    file_name(source.caller_file),
                    source.caller_line,
                    count
                ),
                SourceFilterAction::ShowNotify(source),
            ));
        }
        for (source, count) in hidden_render_sources {
            rows.push(OverlayRow::action(
                format!(
                    "[+] render {} {} 1s {}",
                    short_type_name(source.entity_type),
                    source.phase.as_str(),
                    count
                ),
                SourceFilterAction::ShowRender(source),
            ));
        }
    }

    rows.into_iter().map(OverlayRow::truncate).collect()
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

#[derive(Clone, Copy)]
struct NotifySourceStats {
    count: usize,
    entity_id: EntityId,
    caller_column: u32,
    registered_window_count: usize,
    live_window_count: usize,
}

fn top_notify_sources(
    devtools: &GpuiDevTools,
    now: Instant,
    limit: usize,
) -> Vec<(NotifySourceKey, NotifySourceStats)> {
    let mut counts = FxHashMap::default();
    for event in devtools.notifications.iter() {
        if now.duration_since(event.timestamp) > SOURCE_WINDOW {
            continue;
        }

        let key = NotifySourceKey::from(event);
        if devtools.hidden_notify_sources.contains(&key) {
            continue;
        }

        let stats = counts.entry(key).or_insert(NotifySourceStats {
            count: 0,
            entity_id: event.entity_id,
            caller_column: event.caller_column,
            registered_window_count: event.registered_window_count,
            live_window_count: event.live_window_count,
        });
        stats.count += 1;
        stats.entity_id = event.entity_id;
        stats.caller_column = event.caller_column;
        stats.registered_window_count = event.registered_window_count;
        stats.live_window_count = event.live_window_count;
    }

    let mut counts = counts.into_iter().collect::<Vec<_>>();
    counts.sort_by(|(_, left), (_, right)| right.count.cmp(&left.count));
    counts.truncate(limit);
    counts
}

fn hidden_notify_sources(devtools: &GpuiDevTools, now: Instant) -> Vec<(NotifySourceKey, usize)> {
    let mut counts = devtools
        .hidden_notify_sources
        .iter()
        .copied()
        .map(|source| (source, 0))
        .collect::<FxHashMap<_, _>>();

    for event in devtools.notifications.iter() {
        if now.duration_since(event.timestamp) > SOURCE_WINDOW {
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

fn format_notify_source(index: usize, source: NotifySourceKey, stats: NotifySourceStats) -> String {
    format!(
        "[-] notify {} {} {}:{}:{} x{} reg {} live {} id {}",
        index,
        short_type_name(source.entity_type),
        file_name(source.caller_file),
        source.caller_line,
        stats.caller_column,
        stats.count,
        stats.registered_window_count,
        stats.live_window_count,
        stats.entity_id.as_u64(),
    )
}

fn pinned_notify_recent_count(
    devtools: &GpuiDevTools,
    now: Instant,
    pinned_source: &PinnedNotifySource,
) -> usize {
    devtools
        .notifications
        .iter()
        .filter(|event| {
            now.duration_since(event.timestamp) <= SOURCE_WINDOW && pinned_source.matches(event)
        })
        .count()
}

fn top_dirty_path(
    devtools: &GpuiDevTools,
    window_id: WindowId,
    now: Instant,
) -> Option<(String, usize)> {
    let mut counts = FxHashMap::default();
    for event in devtools.dirty_paths.iter() {
        if event.window_id != window_id || now.duration_since(event.timestamp) > SOURCE_WINDOW {
            continue;
        }

        *counts.entry(dirty_path_label(event)).or_insert(0) += 1;
    }
    counts.into_iter().max_by_key(|(_, count)| *count)
}

#[derive(Default)]
struct RenderSummary {
    render_count: usize,
    reuse_count: usize,
    top_sources: Vec<(RenderSourceKey, RenderSourceStats)>,
}

fn render_summary(devtools: &GpuiDevTools, window_id: WindowId, now: Instant) -> RenderSummary {
    let mut summary = RenderSummary::default();
    let mut counts: FxHashMap<RenderSourceKey, RenderSourceStats> = FxHashMap::default();

    for event in devtools.renders.iter() {
        if event.window_id != window_id || now.duration_since(event.timestamp) > FRAME_RATE_WINDOW {
            continue;
        }

        if event.phase.is_reuse() {
            summary.reuse_count += 1;
        } else if event.phase.flashes() {
            let key = RenderSourceKey::from(event);
            if devtools.hidden_render_sources.contains(&key) {
                continue;
            }

            summary.render_count += 1;

            let stats = counts.entry(key).or_insert(RenderSourceStats {
                count: 0,
                duration: Duration::default(),
                sample_entity_id: event.entity_id,
                bounds: event.bounds,
                cache_miss_reasons: event.cache_miss_reasons,
                caching_disabled_by_inspector: event.caching_disabled_by_inspector,
            });
            stats.count += 1;
            if let Some(duration) = event.duration {
                stats.duration += duration;
            }
            stats.sample_entity_id = event.entity_id;
            stats.bounds = event.bounds;
            stats.cache_miss_reasons = event.cache_miss_reasons;
            stats.caching_disabled_by_inspector = event.caching_disabled_by_inspector;
        }
    }

    let mut counts = counts.into_iter().collect::<Vec<_>>();
    counts.sort_by(|(_, left), (_, right)| right.count.cmp(&left.count));
    counts.truncate(TOP_SOURCE_COUNT);
    summary.top_sources = counts;

    summary
}

fn hidden_render_sources(
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
        if event.window_id != window_id || now.duration_since(event.timestamp) > FRAME_RATE_WINDOW {
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
struct RenderSourceKey {
    entity_type: &'static str,
    phase: ViewRenderPhase,
}

impl RenderSourceKey {
    fn from(event: &ViewRenderEvent) -> Self {
        Self {
            entity_type: event.entity_type,
            phase: event.phase,
        }
    }
}

#[derive(Clone, Copy)]
struct RenderSourceStats {
    count: usize,
    duration: Duration,
    sample_entity_id: EntityId,
    bounds: Option<Bounds<Pixels>>,
    cache_miss_reasons: CacheMissReasons,
    caching_disabled_by_inspector: bool,
}

fn format_render_source(index: usize, source: RenderSourceKey, stats: RenderSourceStats) -> String {
    let mut label = format!(
        "[-] render {} {}#{} {} x{}",
        index,
        short_type_name(source.entity_type),
        stats.sample_entity_id.as_u64(),
        source.phase.as_str(),
        stats.count,
    );
    if !stats.duration.is_zero() {
        label.push_str(&format!(" {}ms", format_duration_ms(stats.duration)));
    }
    if !stats.cache_miss_reasons.is_empty() {
        label.push(' ');
        label.push_str(&stats.cache_miss_reasons.labels().join("+"));
    }
    if stats.caching_disabled_by_inspector {
        label.push_str(" inspector");
    }
    if let Some(bounds) = stats.bounds {
        label.push_str(&format!(
            " {:.0}x{:.0}",
            bounds.size.width.0, bounds.size.height.0
        ));
    }
    label
}

fn active_animation_count(devtools: &GpuiDevTools, window_id: WindowId, now: Instant) -> usize {
    let mut sources = FxHashSet::default();
    for event in devtools.animations.iter() {
        if event.window_id != window_id || now.duration_since(event.timestamp) > ANIMATION_EXPIRY {
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

fn prepaint_overlay(window: &mut Window, snapshot: OverlaySnapshot) -> PreparedOverlay {
    let hud_bounds = hud_bounds(snapshot.rows.len(), window.viewport_size());
    let row_hitboxes = snapshot
        .rows
        .iter()
        .enumerate()
        .filter_map(|(row_index, row)| {
            let action = row.action?;
            let hitbox = window.insert_hitbox(
                hud_button_bounds(hud_bounds, row_index),
                HitboxBehavior::BlockMouse,
            );
            Some(OverlayRowHitbox { hitbox, action })
        })
        .collect();

    PreparedOverlay {
        snapshot,
        hud_bounds,
        row_hitboxes,
    }
}

fn hud_bounds(row_count: usize, viewport_size: crate::Size<Pixels>) -> Bounds<Pixels> {
    let margin = px(12.);
    let padding = hud_padding();
    let hud_width = px(460.);
    let line_height = hud_line_height();
    let hud_height = padding * 2. + line_height * (row_count as f32);
    let origin_x = (viewport_size.width - hud_width - margin).max(margin);
    Bounds::new(point(origin_x, margin), size(hud_width, hud_height))
}

fn hud_button_bounds(hud_bounds: Bounds<Pixels>, row_index: usize) -> Bounds<Pixels> {
    let padding = hud_padding();
    let line_height = hud_line_height();
    Bounds::new(
        point(
            hud_bounds.origin.x + padding - px(2.),
            hud_bounds.origin.y + padding + line_height * (row_index as f32) - px(1.),
        ),
        size(px(23.), line_height),
    )
}

fn hud_padding() -> Pixels {
    px(8.)
}

fn hud_line_height() -> Pixels {
    px(14.)
}

fn paint_hud(window: &mut Window, cx: &mut App, prepared_overlay: &PreparedOverlay) {
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

    for row_hitbox in &prepared_overlay.row_hitboxes {
        let fill_color = if row_hitbox.hitbox.is_hovered(window) {
            rgba(0x38bdf84a)
        } else {
            rgba(0x1f29374a)
        };
        window.paint_quad(fill(row_hitbox.hitbox.bounds, fill_color));
        window.paint_quad(outline(
            row_hitbox.hitbox.bounds,
            hsla(0.58, 0.68, 0.68, 0.52),
            BorderStyle::default(),
        ));
    }

    for (line_index, row) in prepared_overlay.snapshot.rows.iter().enumerate() {
        let origin = point(
            bounds.origin.x + padding,
            bounds.origin.y + padding + line_height * (line_index as f32),
        );
        paint_text_line(window, cx, origin, &row.text, line_height);
    }

    for row_hitbox in prepared_overlay.row_hitboxes.iter().cloned() {
        let hitbox = row_hitbox.hitbox;
        let action = row_hitbox.action;
        window.on_mouse_event(move |event: &MouseDownEvent, phase, window, cx| {
            if phase == DispatchPhase::Bubble
                && event.button == MouseButton::Left
                && hitbox.is_hovered(window)
            {
                apply_filter_action(action);
                window.prevent_default();
                window.refresh();
                cx.stop_propagation();
            }
        });
    }
}

fn paint_text_line(
    window: &mut Window,
    cx: &mut App,
    origin: Point<Pixels>,
    line: &str,
    line_height: Pixels,
) {
    let font_size = px(11.);
    let text_run = TextRun {
        len: line.len(),
        font: font(".SystemUIFont"),
        color: hsla(0.58, 0.38, 0.92, 0.96),
        ..TextRun::default()
    };
    let shaped_line = window.text_system().shape_line(
        SharedString::from(line.to_string()),
        font_size,
        &[text_run],
        None,
    );
    if let Err(error) = shaped_line.paint(origin, line_height, TextAlign::Left, None, window, cx) {
        log::debug!("failed to paint GPUI devtools HUD text: {error:?}");
    }
}

fn apply_filter_action(action: SourceFilterAction) {
    let mut devtools = GPUI_DEVTOOLS.write();
    match action {
        SourceFilterAction::HideNotify(source) => {
            devtools.hidden_notify_sources.insert(source);
        }
        SourceFilterAction::ShowNotify(source) => {
            devtools.hidden_notify_sources.remove(&source);
        }
        SourceFilterAction::HideRender(source) => {
            devtools.hidden_render_sources.insert(source);
            for window_state in devtools.windows.values_mut() {
                window_state
                    .active_flashes
                    .retain(|_, flash| flash.source != source);
            }
        }
        SourceFilterAction::ShowRender(source) => {
            devtools.hidden_render_sources.remove(&source);
        }
    }
}

fn format_duration_ms(duration: Duration) -> String {
    format!("{:.1}", duration_ms(duration))
}

fn format_age(duration: Duration) -> String {
    if duration < Duration::from_secs(1) {
        format!("{}ms", duration.as_millis())
    } else {
        format!("{:.1}s", duration.as_secs_f64())
    }
}

fn duration_ms(duration: Duration) -> f64 {
    duration.as_secs_f64() * 1000.
}

fn short_type_name(type_name: &'static str) -> &'static str {
    type_name.rsplit("::").next().unwrap_or(type_name)
}

fn file_name(path: &str) -> &str {
    path.rsplit('/').next().unwrap_or(path)
}

fn truncate_chars(text: &str, max_chars: usize) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ring_buffer_drops_oldest_entries() {
        let mut buffer = RingBuffer::new(3);
        buffer.push(1);
        buffer.push(2);
        buffer.push(3);
        buffer.push(4);

        assert_eq!(buffer.len(), 3);
        assert_eq!(buffer.iter().copied().collect::<Vec<_>>(), vec![2, 3, 4]);
    }

    #[test]
    fn ring_buffer_zero_capacity_drops_everything() {
        let mut buffer = RingBuffer::new(0);
        buffer.push(1);

        assert_eq!(buffer.len(), 0);
    }

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
}
