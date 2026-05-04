mod events;
mod overlay;
mod ring_buffer;
mod sources;
mod state;

use crate::{App, Bounds, EntityId, Pixels, Window, WindowId};
use parking_lot::RwLock;
use sources::{PinnedNotifySource, RenderSourceKey, parse_pinned_notify_source};
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

    overlay::prepaint_window_overlay(window);
}

pub(crate) fn paint_window_overlay(window: &mut Window, cx: &mut App) {
    if !enabled() {
        return;
    }

    overlay::paint_window_overlay(window, cx);
}
