mod input;
mod layout;
mod paint;
mod rows;
mod snapshot;

pub(super) use layout::PreparedOverlay;

use super::GPUI_DEVTOOLS;
use crate::{App, Window};
use scheduler::Instant;

pub(super) fn prepaint_window_overlay(window: &mut Window) {
    let window_id = window.handle.window_id();
    let snapshot = snapshot::overlay_snapshot(window_id);
    let prepaint_started_at = Instant::now();
    let hud_origin = GPUI_DEVTOOLS.write().window_state(window_id).hud_origin;
    let prepared_overlay = layout::prepaint_overlay(window, snapshot, hud_origin);
    let prepaint_duration = prepaint_started_at.elapsed();

    let mut devtools = GPUI_DEVTOOLS.write();
    devtools.record_prepaint_duration(prepaint_started_at, prepaint_duration);
    devtools.window_state(window_id).prepared_overlay = Some(prepared_overlay);
}

pub(super) fn paint_window_overlay(window: &mut Window, cx: &mut App) {
    let window_id = window.handle.window_id();
    let prepared_overlay = GPUI_DEVTOOLS
        .write()
        .windows
        .get_mut(&window_id)
        .and_then(|window_state| window_state.prepared_overlay.take());

    let Some(prepared_overlay) = prepared_overlay else {
        return;
    };

    let paint_started_at = Instant::now();
    paint::paint_prepared_overlay(window, cx, &prepared_overlay);
    let paint_duration = paint_started_at.elapsed();
    GPUI_DEVTOOLS
        .write()
        .record_paint_duration(paint_started_at, paint_duration);
}

fn event_age(now: Instant, timestamp: Instant) -> Option<std::time::Duration> {
    if timestamp > now {
        None
    } else {
        Some(now.duration_since(timestamp))
    }
}
