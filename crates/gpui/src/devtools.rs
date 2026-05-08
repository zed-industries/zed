mod events;
mod overlay;
mod ring_buffer;
mod sources;
mod state;

use crate::{App, Bounds, EntityId, Pixels, Window, WindowId};
use parking_lot::RwLock;
use sources::{
    NotifySourceKey, NotifySourceStats, PinnedNotifySource, RenderSourceKey, RenderSourceStats,
    parse_pinned_notify_source,
};
use state::{FlashState, GpuiDevTools};
use std::{sync::LazyLock, time::Duration};

pub(crate) use events::{
    AnimationEvent, CacheMissReasons, DirtyPathEvent, DirtyPathSegment, FrameEvent, NotifyEvent,
    ViewRenderEvent, ViewRenderPhase, animation_element_tick_event, animation_frame_request_event,
};

const NOTIFICATION_CAPACITY: usize = 4096;
const FRAME_CAPACITY: usize = 2048;
const VIEW_RENDER_CAPACITY: usize = 8192;
const DIRTY_PATH_CAPACITY: usize = 4096;
const ANIMATION_CAPACITY: usize = 4096;
const WINDOW_FRAME_CAPACITY: usize = 240;
const FLASH_DURATION: Duration = Duration::from_millis(200);
const RENDER_HEAT_WINDOW: Duration = Duration::from_secs(1);
const RENDER_HEAT_DECAY: Duration = Duration::from_millis(500);
const REUSE_OUTLINE_DURATION: Duration = Duration::from_millis(250);
const FRAME_RATE_WINDOW: Duration = Duration::from_secs(1);
const SOURCE_WINDOW: Duration = Duration::from_secs(5);
const ANIMATION_EXPIRY: Duration = Duration::from_secs(1);
const TOP_SOURCE_COUNT: usize = 5;
const HUD_MAX_LINE_CHARS: usize = 84;

static GPUI_DEVTOOLS_ENABLED: LazyLock<bool> =
    LazyLock::new(|| std::env::var_os("ZED_GPUI_DEVTOOLS").is_some());

static GPUI_DEVTOOLS: LazyLock<RwLock<GpuiDevTools>> =
    LazyLock::new(|| RwLock::new(GpuiDevTools::new()));

static INITIAL_PINNED_NOTIFY_SOURCE: LazyLock<Option<PinnedNotifySource>> = LazyLock::new(|| {
    std::env::var("ZED_GPUI_DEVTOOLS_PIN_NOTIFY")
        .ok()
        .and_then(|source| parse_pinned_notify_source(&source))
});

pub(crate) fn enabled() -> bool {
    *GPUI_DEVTOOLS_ENABLED
}

pub(crate) fn record_notify(event: NotifyEvent) {
    if !enabled() {
        return;
    }

    let mut devtools = GPUI_DEVTOOLS.write();
    let source = NotifySourceKey::from(&event);
    *devtools
        .notify_source_total_counts
        .entry(source)
        .or_insert(0) += 1;
    devtools
        .notify_source_last_stats
        .insert(source, NotifySourceStats::from_event(&event));
    if !devtools.initial_pinned_notify_source_resolved
        && INITIAL_PINNED_NOTIFY_SOURCE
            .as_ref()
            .is_some_and(|pinned_source| pinned_source.matches(&event))
    {
        devtools.pinned_notify_sources.insert(source);
        devtools.initial_pinned_notify_source_resolved = true;
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
    if devtools.paused_at.is_none() {
        devtools.window_state(window_id).recent_frames.push(event);
    }
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

    let mut devtools = GPUI_DEVTOOLS.write();
    if devtools.paused_at.is_none() {
        devtools
            .window_state(window_id)
            .view_bounds
            .insert(entity_id, bounds);
    }
}

pub(crate) fn record_view_render(event: ViewRenderEvent) {
    if !enabled() {
        return;
    }

    let mut devtools = GPUI_DEVTOOLS.write();
    let source = RenderSourceKey::from(&event);
    devtools
        .render_source_last_stats
        .insert(source, RenderSourceStats::from_event(&event));
    let source_is_hidden = devtools.hidden_render_sources.contains(&source);
    if devtools.paused_at.is_some() {
        devtools.renders.push(event);
        return;
    }

    let window_state = devtools.window_state(event.window_id);
    let bounds = if let Some(bounds) = event.bounds {
        window_state.view_bounds.insert(event.entity_id, bounds);
        Some(bounds)
    } else {
        window_state.view_bounds.get(&event.entity_id).copied()
    };
    if event.phase.flashes() && !source_is_hidden {
        window_state.active_flashes.insert(
            event.entity_id,
            FlashState {
                timestamp: event.timestamp,
                source,
            },
        );
        window_state.record_render_heat(
            event.entity_id,
            event.timestamp,
            bounds,
            source,
            event.cache_miss_reasons,
        );
    } else if event.phase.is_reuse() && !source_is_hidden {
        window_state.record_reuse_outline(event.entity_id, event.timestamp, bounds, source);
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

    overlay::prepaint_window_overlay(window);
}

pub(crate) fn paint_window_overlay(window: &mut Window, cx: &mut App) {
    if !enabled() {
        return;
    }

    overlay::paint_window_overlay(window, cx);
}
