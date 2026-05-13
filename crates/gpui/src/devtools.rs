//! Runtime GPUI invalidation and rendering diagnostics.

mod events;
mod format;
mod overlay;
mod ring_buffer;
mod sources;
mod state;

use crate::{App, Bounds, EntityId, Pixels, Window, WindowId};
use parking_lot::RwLock;
use scheduler::Instant;
use sources::{
    NotifyCause, NotifySourceKey, NotifySourceStats, RenderSourceKey, RenderSourceStats,
};
use state::{FlashState, GpuiDevTools};
use std::{
    sync::{
        LazyLock,
        atomic::{AtomicBool, Ordering::SeqCst},
    },
    time::Duration,
};

pub(crate) use events::{
    AnimationEvent, CacheMissReasons, DirtyPathEvent, DirtyPathSegment, FrameEvent, NotifyEvent,
    ViewRenderEvent, ViewRenderPhase, animation_element_tick_event, animation_frame_request_event,
};

const NOTIFICATION_CAPACITY: usize = 4096;
const FRAME_CAPACITY: usize = 2048;
const VIEW_RENDER_CAPACITY: usize = 8192;
const DIRTY_PATH_CAPACITY: usize = 4096;
const ANIMATION_CAPACITY: usize = 4096;
const DEVTOOLS_RECORDING_SAMPLE_CAPACITY: usize = 16384;
const DEVTOOLS_HUD_SAMPLE_CAPACITY: usize = 512;
const WINDOW_FRAME_CAPACITY: usize = 240;
const FLASH_DURATION: Duration = Duration::from_millis(200);
const RENDER_HEAT_WINDOW: Duration = Duration::from_secs(1);
const RENDER_HEAT_DECAY: Duration = Duration::from_millis(500);
const REUSE_OUTLINE_DURATION: Duration = Duration::from_millis(250);
const FRAME_RATE_WINDOW: Duration = Duration::from_secs(1);
const SOURCE_WINDOW: Duration = Duration::from_secs(5);
const ANIMATION_EXPIRY: Duration = Duration::from_secs(1);
const TOP_SOURCE_COUNT: usize = 5;
const HUD_MAX_LINE_CHARS: usize = 110;

static GPUI_DEVTOOLS_ENABLED: AtomicBool = AtomicBool::new(false);

static GPUI_DEVTOOLS: LazyLock<RwLock<GpuiDevTools>> =
    LazyLock::new(|| RwLock::new(GpuiDevTools::new()));

pub(crate) fn enabled() -> bool {
    GPUI_DEVTOOLS_ENABLED.load(SeqCst)
}

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

pub(crate) fn close_window(window: &mut Window) {
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

pub(super) fn event_age(now: Instant, timestamp: Instant) -> Option<Duration> {
    if timestamp > now {
        None
    } else {
        Some(now.duration_since(timestamp))
    }
}

pub(crate) fn record_notify(event: NotifyEvent) {
    if !enabled() {
        return;
    }

    let started_at = Instant::now();
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
    devtools.record_recording_duration(started_at, started_at.elapsed());
}

pub(crate) fn record_frame(event: FrameEvent) {
    if !enabled() || !window_open(event.window_id) {
        return;
    }

    let started_at = Instant::now();
    let mut devtools = GPUI_DEVTOOLS.write();
    let window_id = event.window_id;
    devtools.frames.push(event.clone());
    if devtools.paused_at.is_none() {
        devtools.window_state(window_id).recent_frames.push(event);
    }
    devtools.record_recording_duration(started_at, started_at.elapsed());
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

pub(crate) fn record_dirty_path(event: DirtyPathEvent) {
    if !enabled() || !window_open(event.window_id) {
        return;
    }

    let started_at = Instant::now();
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
    devtools.record_recording_duration(started_at, started_at.elapsed());
}

pub(crate) fn record_view_bounds(window_id: WindowId, entity_id: EntityId, bounds: Bounds<Pixels>) {
    if !enabled() || !window_open(window_id) {
        return;
    }

    let started_at = Instant::now();
    let mut devtools = GPUI_DEVTOOLS.write();
    if devtools.paused_at.is_none() {
        devtools
            .window_state(window_id)
            .view_bounds
            .insert(entity_id, bounds);
    }
    devtools.record_recording_duration(started_at, started_at.elapsed());
}

pub(crate) fn record_view_render(event: ViewRenderEvent) {
    if !enabled() || !window_open(event.window_id) {
        return;
    }

    let started_at = Instant::now();
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
    let source_is_hidden = devtools.hidden_render_sources.contains(&source);
    if devtools.paused_at.is_some() {
        devtools.renders.push(event);
        devtools.record_recording_duration(started_at, started_at.elapsed());
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
    devtools.record_recording_duration(started_at, started_at.elapsed());
}

pub(crate) fn record_animation(event: AnimationEvent) {
    if !enabled() || !window_open(event.window_id) {
        return;
    }

    let started_at = Instant::now();
    let mut devtools = GPUI_DEVTOOLS.write();
    devtools.animations.push(event);
    devtools.record_recording_duration(started_at, started_at.elapsed());
}

pub(crate) fn prepaint_window_overlay(window: &mut Window) {
    if !enabled() || !window_open(window.handle.window_id()) {
        return;
    }

    overlay::prepaint_window_overlay(window);
}

pub(crate) fn paint_window_overlay(window: &mut Window, cx: &mut App) {
    if !enabled() || !window_open(window.handle.window_id()) {
        return;
    }

    overlay::paint_window_overlay(window, cx);
}
