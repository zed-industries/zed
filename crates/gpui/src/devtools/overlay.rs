use super::{
    FLASH_DURATION, FRAME_RATE_WINDOW, GPUI_DEVTOOLS, HUD_MAX_LINE_CHARS, PINNED_NOTIFY_SOURCE,
    TOP_SOURCE_COUNT,
    sources::{
        NotifySourceKey, RenderSourceKey, active_animation_count, file_name, format_age,
        format_duration_ms, format_notify_source, format_render_source, hidden_notify_sources,
        hidden_render_sources, pinned_notify_recent_count, render_summary, short_type_name,
        top_dirty_path, top_notify_sources, truncate_chars,
    },
    state::GpuiDevTools,
};
use crate::{
    App, BorderStyle, Bounds, DispatchPhase, Hitbox, HitboxBehavior, MouseButton, MouseDownEvent,
    Pixels, Point, SharedString, TextAlign, TextRun, Window, WindowId, fill, font, hsla, outline,
    point, px, quad, rgba, size,
};
use scheduler::Instant;

pub(super) fn prepaint_window_overlay(window: &mut Window) {
    let window_id = window.handle.window_id();
    let snapshot = overlay_snapshot(window_id);
    let prepared_overlay = prepaint_overlay(window, snapshot);

    GPUI_DEVTOOLS
        .write()
        .window_state(window_id)
        .prepared_overlay = Some(prepared_overlay);
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
pub(super) struct PreparedOverlay {
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
