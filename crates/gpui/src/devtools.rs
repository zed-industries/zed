use crate::{
    App, BorderStyle, Bounds, ElementId, EntityId, Pixels, SceneStats, SharedString, TextAlign,
    TextRun, Window, WindowId, fill, font, hsla, outline, point, px, quad, rgba, size,
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
const HUD_MAX_LINE_CHARS: usize = 54;

static GPUI_DEVTOOLS_ENABLED: LazyLock<bool> =
    LazyLock::new(|| std::env::var_os("ZED_GPUI_DEVTOOLS").is_some());

static GPUI_DEVTOOLS: LazyLock<RwLock<GpuiDevTools>> =
    LazyLock::new(|| RwLock::new(GpuiDevTools::new()));

pub(crate) fn enabled() -> bool {
    *GPUI_DEVTOOLS_ENABLED
}

pub(crate) fn record_notify(event: NotifyEvent) {
    if !enabled() {
        return;
    }

    GPUI_DEVTOOLS.write().notifications.push(event);
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
    let window_state = devtools.window_state(event.window_id);
    if let Some(bounds) = event.bounds {
        window_state.view_bounds.insert(event.entity_id, bounds);
    }
    if event.phase.flashes() {
        window_state
            .active_flashes
            .insert(event.entity_id, event.timestamp);
    }
    devtools.renders.push(event);
}

pub(crate) fn record_animation(event: AnimationEvent) {
    if !enabled() {
        return;
    }

    GPUI_DEVTOOLS.write().animations.push(event);
}

pub(crate) fn paint_window_overlay(window: &mut Window, cx: &mut App) {
    if !enabled() {
        return;
    }

    let snapshot = overlay_snapshot(window.handle.window_id());

    for flash in snapshot.flashes {
        window.paint_quad(quad(
            flash.bounds,
            px(2.),
            hsla(0.54, 0.96, 0.52, 0.10 * flash.opacity),
            px(1.),
            hsla(0.54, 0.96, 0.62, 0.78 * flash.opacity),
            BorderStyle::default(),
        ));
    }

    paint_hud(window, cx, &snapshot.lines);
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    active_flashes: FxHashMap<EntityId, Instant>,
}

impl WindowDevToolsState {
    fn new() -> Self {
        Self {
            recent_frames: RingBuffer::new(WINDOW_FRAME_CAPACITY),
            view_bounds: FxHashMap::default(),
            active_flashes: FxHashMap::default(),
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
    lines: Vec<String>,
}

#[derive(Clone, Debug)]
struct FlashOverlay {
    bounds: Bounds<Pixels>,
    opacity: f32,
}

fn overlay_snapshot(window_id: WindowId) -> OverlaySnapshot {
    let now = Instant::now();
    let mut devtools = GPUI_DEVTOOLS.write();
    let flashes = devtools
        .windows
        .get_mut(&window_id)
        .map(|window_state| {
            window_state
                .active_flashes
                .retain(|_, timestamp| now.duration_since(*timestamp) <= FLASH_DURATION);

            window_state
                .active_flashes
                .iter()
                .filter_map(|(entity_id, timestamp)| {
                    let bounds = window_state.view_bounds.get(entity_id).copied()?;
                    let elapsed = now.duration_since(*timestamp);
                    let opacity = 1. - elapsed.as_secs_f32() / FLASH_DURATION.as_secs_f32();
                    Some(FlashOverlay {
                        bounds,
                        opacity: opacity.clamp(0., 1.),
                    })
                })
                .collect()
        })
        .unwrap_or_default();

    let lines = hud_lines(&devtools, window_id, now);
    OverlaySnapshot { flashes, lines }
}

fn hud_lines(devtools: &GpuiDevTools, window_id: WindowId, now: Instant) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push("GPUI DevTools".to_string());

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

    lines.push(format!(
        "draw/s {:>3}  dirty/s {:>3}  frame/s {:>3}",
        draw_count, dirty_frame_count, frame_count
    ));

    if let Some(frame) = last_frame {
        let last_frame_age = now.duration_since(frame.timestamp);
        let draw_duration = frame
            .draw_duration
            .map(format_duration_ms)
            .unwrap_or_else(|| "--".to_string());
        lines.push(format!(
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
        ));
        lines.push(format!(
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
        ));
    } else {
        lines.push("last frame --".to_string());
    }

    if let Some((source, stats)) = top_notify_source(devtools, now) {
        lines.push(format!(
            "notify {} {}:{}:{} x{} reg {} live {} id {}",
            short_type_name(source.entity_type),
            file_name(source.caller_file),
            source.caller_line,
            stats.caller_column,
            stats.count,
            stats.registered_window_count,
            stats.live_window_count,
            stats.entity_id.as_u64(),
        ));
    } else {
        lines.push("notify --".to_string());
    }

    if let Some((label, count)) = top_dirty_path(devtools, window_id, now) {
        lines.push(format!("dirty {} x{}", label, count));
    } else {
        lines.push("dirty --".to_string());
    }

    let render_summary = render_summary(devtools, window_id, now);
    lines.push(format!(
        "renders/s {} reuse/s {} top {}",
        render_summary.render_count, render_summary.reuse_count, render_summary.top_label
    ));

    lines.push(format!(
        "active animations {}",
        active_animation_count(devtools, window_id, now)
    ));

    lines
        .into_iter()
        .map(|line| truncate_chars(&line, HUD_MAX_LINE_CHARS))
        .collect()
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct NotifySourceKey {
    entity_type: &'static str,
    caller_file: &'static str,
    caller_line: u32,
}

#[derive(Clone, Copy)]
struct NotifySourceStats {
    count: usize,
    entity_id: EntityId,
    caller_column: u32,
    registered_window_count: usize,
    live_window_count: usize,
}

fn top_notify_source(
    devtools: &GpuiDevTools,
    now: Instant,
) -> Option<(NotifySourceKey, NotifySourceStats)> {
    let mut counts = FxHashMap::default();
    for event in devtools.notifications.iter() {
        if now.duration_since(event.timestamp) > SOURCE_WINDOW {
            continue;
        }

        let key = NotifySourceKey {
            entity_type: event.entity_type,
            caller_file: event.caller_file,
            caller_line: event.caller_line,
        };
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
    counts.into_iter().max_by_key(|(_, stats)| stats.count)
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
    top_label: String,
}

fn render_summary(devtools: &GpuiDevTools, window_id: WindowId, now: Instant) -> RenderSummary {
    let mut summary = RenderSummary {
        top_label: "--".to_string(),
        ..RenderSummary::default()
    };
    let mut counts: FxHashMap<String, (usize, Duration)> = FxHashMap::default();

    for event in devtools.renders.iter() {
        if event.window_id != window_id || now.duration_since(event.timestamp) > FRAME_RATE_WINDOW {
            continue;
        }

        if event.phase.is_reuse() {
            summary.reuse_count += 1;
        } else if event.phase.flashes() {
            summary.render_count += 1;
        }

        let mut label = format!(
            "{} {}",
            short_type_name(event.entity_type),
            event.phase.as_str()
        );
        if !event.cache_miss_reasons.is_empty() {
            label.push(' ');
            label.push_str(&event.cache_miss_reasons.labels().join("+"));
        }
        if event.caching_disabled_by_inspector {
            label.push_str(" inspector");
        }
        if let Some(bounds) = event.bounds {
            label.push_str(&format!(
                " {:.0}x{:.0}",
                bounds.size.width.0, bounds.size.height.0
            ));
        }

        let entry = counts.entry(label).or_insert((0, Duration::default()));
        entry.0 += 1;
        if let Some(duration) = event.duration {
            entry.1 += duration;
        }
    }

    if let Some((label, (count, duration))) =
        counts.into_iter().max_by_key(|(_, (count, _))| *count)
    {
        summary.top_label = if duration.is_zero() {
            format!("{label} x{count}")
        } else {
            format!("{label} x{count} {}ms", format_duration_ms(duration))
        };
    }

    summary
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

fn paint_hud(window: &mut Window, cx: &mut App, lines: &[String]) {
    if lines.is_empty() {
        return;
    }

    let margin = px(12.);
    let padding = px(8.);
    let hud_width = px(360.);
    let line_height = px(14.);
    let hud_height = padding * 2. + line_height * (lines.len() as f32);
    let viewport_size = window.viewport_size();
    let origin_x = (viewport_size.width - hud_width - margin).max(margin);
    let bounds = Bounds::new(point(origin_x, margin), size(hud_width, hud_height));

    window.paint_quad(fill(bounds, rgba(0x111827dd)));
    window.paint_quad(outline(
        bounds,
        hsla(0.58, 0.68, 0.68, 0.72),
        BorderStyle::default(),
    ));

    for (line_index, line) in lines.iter().enumerate() {
        let origin = point(
            bounds.origin.x + padding,
            bounds.origin.y + padding + line_height * (line_index as f32),
        );
        paint_text_line(window, cx, origin, line, line_height);
    }
}

fn paint_text_line(
    window: &mut Window,
    cx: &mut App,
    origin: crate::Point<Pixels>,
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
}
