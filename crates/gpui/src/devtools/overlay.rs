use super::{
    FLASH_DURATION, FRAME_RATE_WINDOW, GPUI_DEVTOOLS, HUD_MAX_LINE_CHARS, TOP_SOURCE_COUNT,
    sources::{
        NotifySourceKey, RenderSourceKey, active_animation_count, file_name, format_age,
        format_duration_ms, format_notify_source, format_render_source, hidden_notify_sources,
        hidden_render_sources, render_summary, short_type_name, top_dirty_path, top_notify_sources,
        truncate_chars,
    },
    state::{GpuiDevTools, HudDragState, HudSection, RenderHeatCause},
};
use crate::{
    App, BorderStyle, Bounds, DispatchPhase, Hitbox, HitboxBehavior, MouseButton, MouseDownEvent,
    MouseMoveEvent, MouseUpEvent, Pixels, Point, SharedString, Size, TextAlign, TextRun, Window,
    WindowId, fill, font, hsla, outline, point, px, quad, rgba, size,
};
use scheduler::Instant;
use std::time::Duration;

pub(super) fn prepaint_window_overlay(window: &mut Window) {
    let window_id = window.handle.window_id();
    let snapshot = overlay_snapshot(window_id);
    let prepaint_started_at = Instant::now();
    let hud_origin = GPUI_DEVTOOLS.write().window_state(window_id).hud_origin;
    let prepared_overlay = prepaint_overlay(window, snapshot, hud_origin);
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
    for heat in &prepared_overlay.snapshot.heats {
        let style = heat_style(heat.rate, heat.opacity);
        window.paint_quad(quad(
            heat.bounds,
            px(2.),
            hsla(style.hue, 0.96, 0.54, style.fill_alpha),
            px(style.border_width),
            hsla(style.hue, 0.96, 0.58, style.border_alpha),
            heat.border_style,
        ));
    }

    for reuse_outline in &prepared_overlay.snapshot.reuse_outlines {
        window.paint_quad(outline(
            reuse_outline.bounds,
            hsla(0.42, 0.88, 0.62, 0.68 * reuse_outline.opacity),
            BorderStyle::Solid,
        ));
    }

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
    let paint_duration = paint_started_at.elapsed();
    GPUI_DEVTOOLS
        .write()
        .record_paint_duration(paint_started_at, paint_duration);
}

#[derive(Clone, Debug)]
struct OverlaySnapshot {
    heats: Vec<HeatOverlay>,
    reuse_outlines: Vec<ReuseOutlineOverlay>,
    flashes: Vec<FlashOverlay>,
    rows: Vec<OverlayRow>,
}

#[derive(Clone, Debug)]
struct HeatOverlay {
    bounds: Bounds<Pixels>,
    rate: usize,
    opacity: f32,
    border_style: BorderStyle,
}

#[derive(Clone, Copy, Debug)]
struct HeatStyle {
    hue: f32,
    fill_alpha: f32,
    border_alpha: f32,
    border_width: f32,
}

#[derive(Clone, Debug)]
struct ReuseOutlineOverlay {
    bounds: Bounds<Pixels>,
    opacity: f32,
}

#[derive(Clone, Debug)]
struct FlashOverlay {
    bounds: Bounds<Pixels>,
    opacity: f32,
}

#[derive(Clone, Debug)]
struct OverlayRow {
    text: String,
    kind: OverlayRowKind,
    actions: Vec<OverlayAction>,
}

impl OverlayRow {
    fn plain(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            kind: OverlayRowKind::Data,
            actions: Vec::new(),
        }
    }

    fn header(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            kind: OverlayRowKind::Header,
            actions: Vec::new(),
        }
    }

    fn section_bar(devtools: &GpuiDevTools) -> Self {
        Self {
            text: String::new(),
            kind: OverlayRowKind::SectionBar,
            actions: HudSection::ALL
                .into_iter()
                .map(|section| {
                    OverlayAction::section(section, !devtools.collapsed_sections.contains(&section))
                })
                .collect(),
        }
    }

    fn toolbar(devtools: &GpuiDevTools) -> Self {
        let pause_action = if devtools.paused_at.is_some() {
            OverlayAction::toolbar("resume", true, SourceFilterAction::ResumeCollection)
        } else {
            OverlayAction::toolbar("pause", false, SourceFilterAction::PauseCollection)
        };

        Self {
            text: String::new(),
            kind: OverlayRowKind::Toolbar,
            actions: vec![
                pause_action,
                OverlayAction::toolbar("clear", false, SourceFilterAction::ClearCounters),
                OverlayAction::toolbar(
                    "flashes",
                    devtools.show_flashes,
                    SourceFilterAction::ToggleFlashes,
                ),
                OverlayAction::toolbar("heat", devtools.show_heat, SourceFilterAction::ToggleHeat),
                OverlayAction::toolbar("reset filters", false, SourceFilterAction::ResetFilters),
            ],
        }
    }

    fn actions(text: impl Into<String>, actions: Vec<OverlayAction>) -> Self {
        Self {
            text: text.into(),
            kind: OverlayRowKind::Data,
            actions,
        }
    }

    fn truncate(mut self) -> Self {
        self.text = truncate_chars(&self.text, HUD_MAX_LINE_CHARS);
        self
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum OverlayRowKind {
    Data,
    Header,
    SectionBar,
    Toolbar,
}

#[derive(Clone, Copy, Debug)]
struct OverlayAction {
    label: &'static str,
    active: bool,
    action: SourceFilterAction,
}

impl OverlayAction {
    fn from(action: SourceFilterAction) -> Self {
        match action {
            SourceFilterAction::HideNotify(_) => Self {
                label: "hide",
                active: false,
                action,
            },
            SourceFilterAction::ShowNotify(_) => Self {
                label: "show",
                active: false,
                action,
            },
            SourceFilterAction::PinNotify(_) => Self {
                label: "pin",
                active: false,
                action,
            },
            SourceFilterAction::UnpinNotify(_) => Self {
                label: "unpin",
                active: false,
                action,
            },
            SourceFilterAction::HideRender(_) => Self {
                label: "hide",
                active: false,
                action,
            },
            SourceFilterAction::ShowRender(_) => Self {
                label: "show",
                active: false,
                action,
            },
            SourceFilterAction::PinRender(_) => Self {
                label: "pin",
                active: false,
                action,
            },
            SourceFilterAction::UnpinRender(_) => Self {
                label: "unpin",
                active: false,
                action,
            },
            SourceFilterAction::ToggleSection(section) => Self {
                label: section.label(),
                active: false,
                action,
            },
            SourceFilterAction::PauseCollection => Self {
                label: "pause",
                active: false,
                action,
            },
            SourceFilterAction::ResumeCollection => Self {
                label: "resume",
                active: true,
                action,
            },
            SourceFilterAction::ClearCounters => Self {
                label: "clear",
                active: false,
                action,
            },
            SourceFilterAction::ToggleFlashes => Self {
                label: "flashes",
                active: false,
                action,
            },
            SourceFilterAction::ToggleHeat => Self {
                label: "heat",
                active: false,
                action,
            },
            SourceFilterAction::ResetFilters => Self {
                label: "reset filters",
                active: false,
                action,
            },
        }
    }

    fn toolbar(label: &'static str, active: bool, action: SourceFilterAction) -> Self {
        Self {
            label,
            active,
            action,
        }
    }

    fn section(section: HudSection, active: bool) -> Self {
        Self {
            label: section.label(),
            active,
            action: SourceFilterAction::ToggleSection(section),
        }
    }

    fn width(self) -> Pixels {
        px((self.label.len() as f32 * 6.5 + 18.).clamp(42., 104.))
    }
}

#[derive(Clone, Debug)]
pub(super) struct PreparedOverlay {
    snapshot: OverlaySnapshot,
    hud_bounds: Bounds<Pixels>,
    hud_hitbox: Hitbox,
    row_hitboxes: Vec<OverlayRowHitbox>,
}

#[derive(Clone, Debug)]
struct OverlayRowHitbox {
    hitbox: Hitbox,
    action: OverlayAction,
}

#[derive(Clone, Copy, Debug)]
enum SourceFilterAction {
    ToggleSection(HudSection),
    PauseCollection,
    ResumeCollection,
    ClearCounters,
    ToggleFlashes,
    ToggleHeat,
    ResetFilters,
    HideNotify(NotifySourceKey),
    ShowNotify(NotifySourceKey),
    PinNotify(NotifySourceKey),
    UnpinNotify(NotifySourceKey),
    HideRender(RenderSourceKey),
    ShowRender(RenderSourceKey),
    PinRender(RenderSourceKey),
    UnpinRender(RenderSourceKey),
}

fn overlay_snapshot(window_id: WindowId) -> OverlaySnapshot {
    let snapshot_started_at = Instant::now();
    let mut devtools = GPUI_DEVTOOLS.write();
    let now = devtools.paused_at.unwrap_or_else(Instant::now);
    let show_flashes = devtools.show_flashes;
    let show_heat = devtools.show_heat;
    let hidden_render_sources = devtools.hidden_render_sources.clone();
    let (heats, reuse_outlines, flashes) = devtools
        .windows
        .get_mut(&window_id)
        .map(|window_state| {
            let mut expired_heat_entities = Vec::new();
            let mut heats = Vec::new();
            if show_heat {
                for (entity_id, heat) in &mut window_state.render_heat {
                    if hidden_render_sources.contains(&heat.source) || heat.expired(now) {
                        expired_heat_entities.push(*entity_id);
                        continue;
                    }

                    let current_rate = heat.prune(now);
                    let rate = current_rate.max(heat.last_rate);
                    let opacity = heat.opacity(now);
                    if rate == 0 || opacity == 0. {
                        continue;
                    }

                    let Some(bounds) = heat
                        .bounds
                        .or_else(|| window_state.view_bounds.get(entity_id).copied())
                    else {
                        continue;
                    };

                    heats.push(HeatOverlay {
                        bounds,
                        rate,
                        opacity,
                        border_style: match heat.cause {
                            RenderHeatCause::Render => BorderStyle::Solid,
                            RenderHeatCause::Refresh => BorderStyle::Dashed,
                        },
                    });
                }
            }
            for entity_id in expired_heat_entities {
                window_state.render_heat.remove(&entity_id);
            }

            let mut expired_reuse_entities = Vec::new();
            let mut reuse_outlines = Vec::new();
            if show_heat {
                for (entity_id, reuse_outline) in &window_state.reuse_outlines {
                    if hidden_render_sources.contains(&reuse_outline.source)
                        || reuse_outline.expired(now)
                    {
                        expired_reuse_entities.push(*entity_id);
                        continue;
                    }

                    let opacity = reuse_outline.opacity(now);
                    if opacity == 0. {
                        continue;
                    }

                    let Some(bounds) = reuse_outline
                        .bounds
                        .or_else(|| window_state.view_bounds.get(entity_id).copied())
                    else {
                        continue;
                    };

                    reuse_outlines.push(ReuseOutlineOverlay { bounds, opacity });
                }
            }
            for entity_id in expired_reuse_entities {
                window_state.reuse_outlines.remove(&entity_id);
            }

            window_state.active_flashes.retain(|_, flash| {
                event_age(now, flash.timestamp).is_none_or(|age| age <= FLASH_DURATION)
            });

            let flashes = if show_flashes {
                window_state
                    .active_flashes
                    .iter()
                    .filter_map(|(entity_id, flash)| {
                        if hidden_render_sources.contains(&flash.source) {
                            return None;
                        }

                        let bounds = window_state.view_bounds.get(entity_id).copied()?;
                        let elapsed = event_age(now, flash.timestamp)?;
                        let opacity = 1. - elapsed.as_secs_f32() / FLASH_DURATION.as_secs_f32();
                        Some(FlashOverlay {
                            bounds,
                            opacity: opacity.clamp(0., 1.),
                        })
                    })
                    .collect()
            } else {
                Vec::new()
            };

            (heats, reuse_outlines, flashes)
        })
        .unwrap_or_default();

    let rows = hud_rows(&devtools, window_id, now);
    let snapshot = OverlaySnapshot {
        heats,
        reuse_outlines,
        flashes,
        rows,
    };
    devtools.record_snapshot_duration(snapshot_started_at, snapshot_started_at.elapsed());
    snapshot
}

fn heat_style(rate: usize, opacity: f32) -> HeatStyle {
    let (hue, fill_alpha, border_alpha, border_width) = if rate >= 30 {
        (0.0, 0.24, 0.98, 3.0)
    } else if rate >= 15 {
        (0.025, 0.19, 0.90, 2.25)
    } else if rate >= 5 {
        (0.075, 0.14, 0.74, 1.5)
    } else {
        (0.13, 0.10, 0.58, 1.0)
    };

    HeatStyle {
        hue,
        fill_alpha: fill_alpha * opacity,
        border_alpha: border_alpha * opacity,
        border_width,
    }
}

fn format_performance_duration_ms(duration: Duration) -> String {
    let ms = duration.as_secs_f64() * 1000.;
    if ms < 0.1 {
        format!("{ms:.3}")
    } else if ms < 10. {
        format!("{ms:.2}")
    } else {
        format!("{ms:.1}")
    }
}

fn hud_rows(devtools: &GpuiDevTools, window_id: WindowId, now: Instant) -> Vec<OverlayRow> {
    let mut rows = Vec::new();
    rows.push(OverlayRow::header(if devtools.paused_at.is_some() {
        "GPUI DevTools paused"
    } else {
        "GPUI DevTools"
    }));
    rows.push(OverlayRow::toolbar(devtools));
    rows.push(OverlayRow::section_bar(devtools));

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
            .find(|frame| frame.timestamp <= now);
    }

    if section_expanded(devtools, HudSection::Frame) {
        rows.push(OverlayRow::plain(format!(
            "draw/s {:>3}  dirty/s {:>3}  frame/s {:>3}",
            draw_count, dirty_frame_count, frame_count
        )));

        if let Some(frame) = last_frame {
            let last_frame_age = event_age(now, frame.timestamp).unwrap_or_default();
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

        let performance = devtools.performance_summary(now);
        rows.push(OverlayRow::plain(format!(
            "devtools rec 5s {:>4} avg {} max {}ms",
            performance.recording.count,
            format_performance_duration_ms(performance.recording.average),
            format_performance_duration_ms(performance.recording.max),
        )));
        rows.push(OverlayRow::plain(format!(
            "devtools hud avg pre {} snap {} paint {}ms",
            format_performance_duration_ms(performance.prepaint.average),
            format_performance_duration_ms(performance.snapshot.average),
            format_performance_duration_ms(performance.paint.average),
        )));
        rows.push(OverlayRow::plain(format!(
            "devtools hud max pre {} snap {} paint {}ms",
            format_performance_duration_ms(performance.prepaint.max),
            format_performance_duration_ms(performance.snapshot.max),
            format_performance_duration_ms(performance.paint.max),
        )));
    }

    if section_expanded(devtools, HudSection::Notify) {
        let notify_sources = top_notify_sources(devtools, now, TOP_SOURCE_COUNT);
        if notify_sources.is_empty() {
            rows.push(OverlayRow::plain("no notify sources in the last 5s"));
        } else {
            rows.push(OverlayRow::plain(
                "rank source            caller               5s total age   live",
            ));
            for (index, (source, stats)) in notify_sources.into_iter().enumerate() {
                let is_pinned = devtools.pinned_notify_sources.contains(&source);
                let pin_action = if is_pinned {
                    SourceFilterAction::UnpinNotify(source)
                } else {
                    SourceFilterAction::PinNotify(source)
                };
                rows.push(OverlayRow::actions(
                    format_notify_source(
                        index + 1,
                        source,
                        stats,
                        devtools.notify_source_total_count(source),
                        now,
                    ),
                    vec![
                        OverlayAction::from(SourceFilterAction::HideNotify(source)),
                        OverlayAction::from(pin_action),
                    ],
                ));
            }
        }
    }

    if section_expanded(devtools, HudSection::Dirty) {
        if let Some((label, count)) = top_dirty_path(devtools, window_id, now) {
            rows.push(OverlayRow::plain(format!(
                "top path x{:>4} {}",
                count, label
            )));
        } else {
            rows.push(OverlayRow::plain("no dirty path in the last 5s"));
        }
    }

    if section_expanded(devtools, HudSection::Render) {
        let render_summary = render_summary(devtools, window_id, now);
        rows.push(OverlayRow::plain(format!(
            "renders/s {} reuse/s {}",
            render_summary.render_count, render_summary.reuse_count
        )));
        if render_summary.top_sources.is_empty() {
            rows.push(OverlayRow::plain("no real renders in the last 1s"));
        } else {
            rows.push(OverlayRow::plain(
                "rank view            phase       r/s reuse age   cost  miss why",
            ));
            for (index, (source, stats)) in render_summary.top_sources.into_iter().enumerate() {
                let is_pinned = devtools.pinned_render_sources.contains(&source);
                let pin_action = if is_pinned {
                    SourceFilterAction::UnpinRender(source)
                } else {
                    SourceFilterAction::PinRender(source)
                };
                rows.push(OverlayRow::actions(
                    format_render_source(index + 1, source, stats, now),
                    vec![
                        OverlayAction::from(SourceFilterAction::HideRender(source)),
                        OverlayAction::from(pin_action),
                    ],
                ));
            }
        }
    }

    if section_expanded(devtools, HudSection::Animation) {
        rows.push(OverlayRow::plain(format!(
            "active animations {}",
            active_animation_count(devtools, window_id, now)
        )));
    }

    let hidden_notify_sources = hidden_notify_sources(devtools, now);
    let hidden_render_sources = hidden_render_sources(devtools, window_id, now);
    if section_expanded(devtools, HudSection::Hidden)
        && (!hidden_notify_sources.is_empty() || !hidden_render_sources.is_empty())
    {
        for (source, count) in hidden_notify_sources {
            let is_pinned = devtools.pinned_notify_sources.contains(&source);
            let pin_action = if is_pinned {
                SourceFilterAction::UnpinNotify(source)
            } else {
                SourceFilterAction::PinNotify(source)
            };
            rows.push(OverlayRow::actions(
                format!(
                    "notify {:<16} {:<20} 5s {:>4}",
                    short_type_name(source.entity_type),
                    format!("{}:{}", file_name(source.caller_file), source.caller_line),
                    count
                ),
                vec![
                    OverlayAction::from(SourceFilterAction::ShowNotify(source)),
                    OverlayAction::from(pin_action),
                ],
            ));
        }
        for (source, count) in hidden_render_sources {
            let is_pinned = devtools.pinned_render_sources.contains(&source);
            let pin_action = if is_pinned {
                SourceFilterAction::UnpinRender(source)
            } else {
                SourceFilterAction::PinRender(source)
            };
            rows.push(OverlayRow::actions(
                format!(
                    "render {:<16} {:<14} 1s {:>4}",
                    short_type_name(source.entity_type),
                    source.phase.as_str(),
                    count
                ),
                vec![
                    OverlayAction::from(SourceFilterAction::ShowRender(source)),
                    OverlayAction::from(pin_action),
                ],
            ));
        }
    }

    rows.into_iter().map(OverlayRow::truncate).collect()
}

fn section_expanded(devtools: &GpuiDevTools, section: HudSection) -> bool {
    !devtools.collapsed_sections.contains(&section)
}

fn prepaint_overlay(
    window: &mut Window,
    snapshot: OverlaySnapshot,
    hud_origin: Option<Point<Pixels>>,
) -> PreparedOverlay {
    let hud_bounds = hud_bounds(snapshot.rows.len(), window.viewport_size(), hud_origin);
    let hud_hitbox = window.insert_hitbox(hud_bounds, HitboxBehavior::Normal);
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
        hud_hitbox,
        row_hitboxes,
    }
}

fn hud_bounds(
    row_count: usize,
    viewport_size: Size<Pixels>,
    hud_origin: Option<Point<Pixels>>,
) -> Bounds<Pixels> {
    let margin = px(12.);
    let padding = hud_padding();
    let hud_width = px(760.);
    let line_height = hud_line_height();
    let hud_height = padding * 2. + line_height * (row_count as f32);
    let hud_size = size(hud_width, hud_height);
    let default_origin = point(
        (viewport_size.width - hud_width - margin).max(margin),
        margin,
    );
    let origin = hud_origin.unwrap_or(default_origin);
    Bounds::new(clamp_hud_origin(origin, viewport_size, hud_size), hud_size)
}

fn clamp_hud_origin(
    origin: Point<Pixels>,
    viewport_size: Size<Pixels>,
    hud_size: Size<Pixels>,
) -> Point<Pixels> {
    let visible_handle_size = px(28.);
    let min = point(
        visible_handle_size - hud_size.width,
        visible_handle_size - hud_size.height,
    );
    let max = point(
        viewport_size.width - visible_handle_size,
        viewport_size.height - visible_handle_size,
    );
    let max = max.max(&min);
    origin.clamp(&min, &max)
}

fn hud_button_bounds(
    hud_bounds: Bounds<Pixels>,
    row_index: usize,
    actions: &[OverlayAction],
    action_index: usize,
) -> Bounds<Pixels> {
    let padding = hud_padding();
    let line_height = hud_line_height();
    let button_gap = hud_button_gap();
    let action_offset = actions
        .iter()
        .take(action_index)
        .fold(px(0.), |offset, action| {
            offset + action.width() + button_gap
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

    for (line_index, row) in prepared_overlay.snapshot.rows.iter().enumerate() {
        let row_bounds = hud_row_bounds(bounds, line_index);
        match row.kind {
            OverlayRowKind::SectionBar | OverlayRowKind::Toolbar => {
                window.paint_quad(fill(row_bounds, rgba(0x273244aa)));
            }
            OverlayRowKind::Header | OverlayRowKind::Data => {}
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
        let button_text_origin = point(
            row_hitbox.hitbox.origin.x + px(5.),
            row_hitbox.hitbox.origin.y + px(1.),
        );
        paint_text_line_with_color(
            window,
            cx,
            button_text_origin,
            row_hitbox.action.label,
            line_height,
            hsla(0.58, 0.38, 0.98, 0.98),
        );
    }

    for (line_index, row) in prepared_overlay.snapshot.rows.iter().enumerate() {
        let text_color = match row.kind {
            OverlayRowKind::Header => hsla(0.58, 0.44, 0.94, 1.),
            OverlayRowKind::SectionBar | OverlayRowKind::Toolbar => hsla(0.12, 0.62, 0.76, 1.),
            OverlayRowKind::Data => hsla(0.58, 0.38, 0.92, 0.96),
        };
        let origin = point(
            bounds.origin.x + padding + hud_action_text_offset(&row.actions),
            bounds.origin.y + padding + line_height * (line_index as f32),
        );
        paint_text_line_with_color(window, cx, origin, &row.text, line_height, text_color);
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamp_hud_origin_keeps_a_handle_visible() {
        let viewport_size = size(px(100.), px(80.));
        let hud_size = size(px(460.), px(140.));

        assert_eq!(
            clamp_hud_origin(point(px(-1000.), px(1000.)), viewport_size, hud_size),
            point(px(28.) - hud_size.width, viewport_size.height - px(28.))
        );
    }

    #[test]
    fn dragged_hud_bounds_use_the_requested_origin() {
        let bounds = hud_bounds(4, size(px(800.), px(600.)), Some(point(px(120.), px(140.))));

        assert_eq!(bounds.origin, point(px(120.), px(140.)));
    }
}

fn paint_text_line_with_color(
    window: &mut Window,
    cx: &mut App,
    origin: Point<Pixels>,
    line: &str,
    line_height: Pixels,
    color: crate::Hsla,
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
        log::debug!("failed to paint GPUI devtools HUD text: {error:?}");
    }
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

fn event_age(now: Instant, timestamp: Instant) -> Option<std::time::Duration> {
    if timestamp > now {
        None
    } else {
        Some(now.duration_since(timestamp))
    }
}
