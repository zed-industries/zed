use crate::{Pixels, WindowId, px};
use scheduler::Instant;
use std::time::Duration;

use super::super::{
    FRAME_RATE_WINDOW, HUD_MAX_LINE_CHARS, TOP_SOURCE_COUNT_OPTIONS, event_age,
    format::{
        file_name, format_age, format_duration_ms, format_notify_cause, format_notify_source,
        format_render_source, notify_column_header, render_column_header, short_type_name,
        truncate_chars,
    },
    sources::{
        NotifySourceKey, RenderSourceKey, active_animation_count, hidden_notify_sources,
        hidden_render_sources, render_summary, top_dirty_path, top_notify_sources,
    },
    state::{GpuiDevTools, HudSection},
};

#[derive(Clone, Debug)]
pub(super) struct OverlayRow {
    pub(super) text: String,
    pub(super) kind: OverlayRowKind,
    pub(super) actions: Vec<OverlayAction>,
    /// Action indices after which a wider gap is inserted to visually
    /// separate toolbar groups (e.g. collection control vs visual toggles).
    pub(super) action_group_breaks: Vec<usize>,
}

impl OverlayRow {
    fn plain(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            kind: OverlayRowKind::Data,
            actions: Vec::new(),
            action_group_breaks: Vec::new(),
        }
    }

    fn header(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            kind: OverlayRowKind::Header,
            actions: Vec::new(),
            action_group_breaks: Vec::new(),
        }
    }

    fn column_header(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            kind: OverlayRowKind::ColumnHeader,
            actions: Vec::new(),
            action_group_breaks: Vec::new(),
        }
    }

    fn spacer() -> Self {
        Self {
            text: String::new(),
            kind: OverlayRowKind::Spacer,
            actions: Vec::new(),
            action_group_breaks: Vec::new(),
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
            action_group_breaks: Vec::new(),
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
                OverlayAction::toolbar("close", false, SourceFilterAction::CloseDevTools),
            ],
            // Split into [pause, clear] | [flashes, heat] | [reset filters, close].
            action_group_breaks: vec![1, 3],
        }
    }

    fn actions(text: impl Into<String>, actions: Vec<OverlayAction>) -> Self {
        Self {
            text: text.into(),
            kind: OverlayRowKind::Data,
            actions,
            action_group_breaks: Vec::new(),
        }
    }

    fn truncate(mut self) -> Self {
        self.text = truncate_chars(&self.text, HUD_MAX_LINE_CHARS);
        self
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum OverlayRowKind {
    Data,
    Header,
    /// Column header for a section. Painted with the same indent as
    /// data rows so column headings line up with row values.
    ColumnHeader,
    /// Empty row used to separate sections; paints no background or text.
    Spacer,
    SectionBar,
    Toolbar,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct OverlayAction {
    pub(super) label: &'static str,
    pub(super) active: bool,
    pub(super) action: SourceFilterAction,
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
            SourceFilterAction::SetNotifySourceLimit(_)
            | SourceFilterAction::SetRenderSourceLimit(_) => Self {
                label: "select",
                active: false,
                action,
            },
            SourceFilterAction::CloseDevTools => Self {
                label: "close",
                active: false,
                action,
            },
        }
    }

    pub(super) fn toolbar(label: &'static str, active: bool, action: SourceFilterAction) -> Self {
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

    pub(super) fn width(self) -> Pixels {
        // Floor of 52px keeps `hide`/`pin`/`show`/`unpin` data-row buttons at
        // a single uniform width, so column headers and pinned/unpinned rows
        // all align under the same indent.
        px((self.label.len() as f32 * 6.5 + 18.).clamp(52., 104.))
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) enum SourceFilterAction {
    ToggleSection(HudSection),
    PauseCollection,
    ResumeCollection,
    ClearCounters,
    ToggleFlashes,
    ToggleHeat,
    ResetFilters,
    SetNotifySourceLimit(usize),
    SetRenderSourceLimit(usize),
    CloseDevTools,
    HideNotify(NotifySourceKey),
    ShowNotify(NotifySourceKey),
    PinNotify(NotifySourceKey),
    UnpinNotify(NotifySourceKey),
    HideRender(RenderSourceKey),
    ShowRender(RenderSourceKey),
    PinRender(RenderSourceKey),
    UnpinRender(RenderSourceKey),
}

pub(super) fn hud_rows(
    devtools: &GpuiDevTools,
    window_id: WindowId,
    now: Instant,
) -> Vec<OverlayRow> {
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

    let mut any_section_rendered = false;
    let mut start_section = |rows: &mut Vec<OverlayRow>, started: &mut bool| {
        if *started {
            rows.push(OverlayRow::spacer());
        }
        *started = true;
    };

    if section_expanded(devtools, HudSection::Frame) {
        start_section(&mut rows, &mut any_section_rendered);

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
                "views {} updates {} ops {} quads {}",
                frame.dirty_view_count,
                frame.invalidator_update_count,
                frame.scene_stats.paint_operation_count,
                frame.scene_stats.quad_count,
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
        start_section(&mut rows, &mut any_section_rendered);

        rows.push(top_source_limit_row(
            "notify top",
            devtools.notify_source_limit,
            SourceFilterAction::SetNotifySourceLimit,
        ));

        let notify_sources = top_notify_sources(devtools, now, devtools.notify_source_limit);
        if notify_sources.is_empty() {
            rows.push(OverlayRow::plain("no notify sources in the last 5s"));
        } else {
            rows.push(OverlayRow::column_header(notify_column_header()));
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
        start_section(&mut rows, &mut any_section_rendered);

        if let Some(summary) = top_dirty_path(devtools, window_id, now) {
            let cause = summary
                .cause
                .map(|cause| format!(" {}", format_notify_cause(cause, now)))
                .unwrap_or_default();
            rows.push(OverlayRow::plain(format!(
                "top path x{:>4} {}{}",
                summary.count, summary.label, cause,
            )));
        } else {
            rows.push(OverlayRow::plain("no dirty path in the last 5s"));
        }
    }

    if section_expanded(devtools, HudSection::Render) {
        start_section(&mut rows, &mut any_section_rendered);

        rows.push(top_source_limit_row(
            "render top",
            devtools.render_source_limit,
            SourceFilterAction::SetRenderSourceLimit,
        ));

        let render_summary = render_summary(devtools, window_id, now, devtools.render_source_limit);
        rows.push(OverlayRow::plain(format!(
            "renders/s {} reuse/s {}",
            render_summary.render_count, render_summary.reuse_count
        )));
        if render_summary.top_sources.is_empty() {
            rows.push(OverlayRow::plain("no real renders in the last 1s"));
        } else {
            rows.push(OverlayRow::column_header(render_column_header()));
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
        start_section(&mut rows, &mut any_section_rendered);

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
        start_section(&mut rows, &mut any_section_rendered);

        rows.push(OverlayRow::column_header(format!(
            "{:<8} {:<18} {:<24} {:>5}",
            "kind", "source", "caller", "count"
        )));
        for (source, count) in hidden_notify_sources {
            let is_pinned = devtools.pinned_notify_sources.contains(&source);
            let pin_action = if is_pinned {
                SourceFilterAction::UnpinNotify(source)
            } else {
                SourceFilterAction::PinNotify(source)
            };
            rows.push(OverlayRow::actions(
                format!(
                    "{:<8} {:<18} {:<24} {:>5}",
                    "notify",
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
            let view = format!(
                "{}#{}",
                short_type_name(source.entity_type),
                source.entity_id.as_u64()
            );
            rows.push(OverlayRow::actions(
                format!(
                    "{:<8} {:<18} {:<24} {:>5}",
                    "render",
                    view,
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

fn top_source_limit_row(
    label: &'static str,
    active_limit: usize,
    action: fn(usize) -> SourceFilterAction,
) -> OverlayRow {
    OverlayRow::actions(
        label,
        TOP_SOURCE_COUNT_OPTIONS
            .into_iter()
            .map(|limit| {
                OverlayAction::toolbar(
                    top_source_limit_label(limit),
                    limit == active_limit,
                    action(limit),
                )
            })
            .collect(),
    )
}

fn top_source_limit_label(limit: usize) -> &'static str {
    match limit {
        5 => "5",
        10 => "10",
        25 => "25",
        _ => "?",
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_limit_rows_expose_independent_options() {
        let mut devtools = GpuiDevTools::new();
        devtools.notify_source_limit = 10;
        devtools.render_source_limit = 25;

        let rows = hud_rows(&devtools, WindowId::from(1), Instant::now());
        let notify_row = rows
            .iter()
            .find(|row| row.text == "notify top")
            .expect("expected notify source limit row");
        assert_eq!(
            source_limit_options(notify_row),
            vec![("5", false), ("10", true), ("25", false)]
        );

        let render_row = rows
            .iter()
            .find(|row| row.text == "render top")
            .expect("expected render source limit row");
        assert_eq!(
            source_limit_options(render_row),
            vec![("5", false), ("10", false), ("25", true)]
        );
    }

    fn source_limit_options(row: &OverlayRow) -> Vec<(&'static str, bool)> {
        row.actions
            .iter()
            .map(|action| (action.label, action.active))
            .collect()
    }
}
