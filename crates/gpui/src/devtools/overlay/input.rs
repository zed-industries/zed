use super::{
    layout::{PreparedOverlay, clamp_hud_origin},
    rows::SourceFilterAction,
};
use crate::{DispatchPhase, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, Window};
use scheduler::Instant;

use super::super::{GPUI_DEVTOOLS, state::HudDragState};

pub(super) fn register_input_handlers(window: &mut Window, prepared_overlay: &PreparedOverlay) {
    register_drag_handlers(window, prepared_overlay);

    for row_hitbox in prepared_overlay.row_hitboxes.iter().cloned() {
        let hitbox = row_hitbox.hitbox;
        let action = row_hitbox.action;
        window.on_mouse_event(move |event: &MouseDownEvent, phase, window, cx| {
            if phase == DispatchPhase::Bubble
                && event.button == MouseButton::Left
                && hitbox.is_hovered(window)
            {
                apply_filter_action(action.action);
                window.prevent_default();
                window.refresh();
                cx.stop_propagation();
            }
        });
    }
}

fn register_drag_handlers(window: &mut Window, prepared_overlay: &PreparedOverlay) {
    let hitbox = prepared_overlay.hud_hitbox.clone();
    let hud_size = prepared_overlay.hud_bounds.size;

    window.on_mouse_event({
        let hitbox = hitbox.clone();
        move |event: &MouseDownEvent, phase, window, cx| {
            if phase == DispatchPhase::Bubble
                && event.button == MouseButton::Left
                && hitbox.is_hovered(window)
            {
                let window_id = window.handle.window_id();
                let cursor_offset = event.position - hitbox.origin;
                GPUI_DEVTOOLS.write().window_state(window_id).hud_drag =
                    Some(HudDragState { cursor_offset });
                window.capture_pointer(hitbox.id);
                window.prevent_default();
                window.refresh();
                cx.stop_propagation();
            }
        }
    });

    window.on_mouse_event({
        let hitbox = hitbox.clone();
        move |event: &MouseMoveEvent, phase, window, cx| {
            if phase == DispatchPhase::Bubble && hitbox.is_hovered(window) {
                let window_id = window.handle.window_id();
                let handled = {
                    let mut devtools = GPUI_DEVTOOLS.write();
                    let window_state = devtools.window_state(window_id);
                    if let Some(drag) = window_state.hud_drag {
                        if !event.dragging() {
                            window_state.hud_drag = None;
                            false
                        } else {
                            let origin = event.position - drag.cursor_offset;
                            window_state.hud_origin =
                                Some(clamp_hud_origin(origin, window.viewport_size(), hud_size));
                            true
                        }
                    } else {
                        false
                    }
                };

                if handled {
                    window.refresh();
                    cx.stop_propagation();
                } else if !event.dragging() {
                    window.release_pointer();
                }
            }
        }
    });

    window.on_mouse_event(move |event: &MouseUpEvent, phase, window, cx| {
        if phase == DispatchPhase::Bubble
            && event.button == MouseButton::Left
            && hitbox.is_hovered(window)
        {
            let window_id = window.handle.window_id();
            GPUI_DEVTOOLS.write().window_state(window_id).hud_drag = None;
            window.release_pointer();
            window.refresh();
            cx.stop_propagation();
        }
    });
}

fn apply_filter_action(action: SourceFilterAction) {
    let mut devtools = GPUI_DEVTOOLS.write();
    match action {
        SourceFilterAction::ToggleSection(section) => {
            if !devtools.collapsed_sections.remove(&section) {
                devtools.collapsed_sections.insert(section);
            }
        }
        SourceFilterAction::PauseCollection => {
            devtools.pause(Instant::now());
        }
        SourceFilterAction::ResumeCollection => {
            devtools.resume();
        }
        SourceFilterAction::ClearCounters => {
            devtools.clear_counters();
        }
        SourceFilterAction::ToggleFlashes => {
            devtools.show_flashes = !devtools.show_flashes;
        }
        SourceFilterAction::ToggleHeat => {
            devtools.show_heat = !devtools.show_heat;
        }
        SourceFilterAction::ResetFilters => {
            devtools.hidden_notify_sources.clear();
            devtools.hidden_render_sources.clear();
        }
        SourceFilterAction::HideNotify(source) => {
            devtools.hidden_notify_sources.insert(source);
        }
        SourceFilterAction::ShowNotify(source) => {
            devtools.hidden_notify_sources.remove(&source);
        }
        SourceFilterAction::PinNotify(source) => {
            devtools.pinned_notify_sources.insert(source);
            devtools.initial_pinned_notify_source_resolved = true;
        }
        SourceFilterAction::UnpinNotify(source) => {
            devtools.pinned_notify_sources.remove(&source);
            devtools.initial_pinned_notify_source_resolved = true;
        }
        SourceFilterAction::HideRender(source) => {
            devtools.hidden_render_sources.insert(source);
            for window_state in devtools.windows.values_mut() {
                window_state
                    .active_flashes
                    .retain(|_, flash| flash.source != source);
                window_state
                    .render_heat
                    .retain(|_, heat| heat.source != source);
                window_state
                    .reuse_outlines
                    .retain(|_, outline| outline.source != source);
            }
        }
        SourceFilterAction::ShowRender(source) => {
            devtools.hidden_render_sources.remove(&source);
        }
        SourceFilterAction::PinRender(source) => {
            devtools.pinned_render_sources.insert(source);
        }
        SourceFilterAction::UnpinRender(source) => {
            devtools.pinned_render_sources.remove(&source);
        }
    }
}
