use super::{
    AnimationEventKind, DirtyPathEvent, FrameEvent, GPUI_DEVTOOLS, GpuiDevTools, NotifySourceKey,
    NotifySourceStats, RenderSourceKey, RenderSourceStats, SOURCE_WINDOW, close_source_window,
    event_age, window_open,
};
use crate::prelude::*;
use crate::{
    AnyElement, App, Context, EntityId, Pixels, Subscription, TitlebarOptions, Window,
    WindowBounds, WindowId, WindowKind, WindowOptions, div, hsla, point, px, rgba, size,
};
use collections::{FxHashMap, FxHashSet};
use scheduler::Instant;
use std::time::Duration;

const FRAME_RATE_WINDOW: Duration = Duration::from_secs(1);
const ANIMATION_EXPIRY: Duration = Duration::from_secs(1);
const TOP_SOURCE_COUNT: usize = 15;
const WINDOW_WIDTH: Pixels = px(760.);
const WINDOW_HEIGHT: Pixels = px(360.);

pub(super) fn open(source_window_id: WindowId, cx: &mut App) {
    if let Some(existing_window) = existing_devtools_window(source_window_id, cx) {
        if let Err(error) = existing_window.update(cx, |_, window, _| window.activate_window()) {
            log::debug!("failed to activate existing GPUI profiler window: {error:?}");
        }
        return;
    }

    let result = cx.open_window(
        WindowOptions {
            titlebar: Some(TitlebarOptions {
                title: Some("GPUI Profiler".into()),
                appears_transparent: false,
                traffic_light_position: Some(point(px(12.), px(12.))),
            }),
            window_bounds: Some(WindowBounds::centered(
                size(WINDOW_WIDTH, WINDOW_HEIGHT),
                cx,
            )),
            kind: WindowKind::Normal,
            is_resizable: true,
            is_minimizable: true,
            window_min_size: Some(size(px(520.), px(260.))),
            ..Default::default()
        },
        |window, cx| {
            let devtools_window_id = window.handle.window_id();
            window.on_window_should_close(cx, move |_, _| {
                close_source_window(source_window_id);
                true
            });
            cx.new(|cx| GpuiDevtoolsWindow::new(source_window_id, devtools_window_id, cx))
        },
    );

    if let Err(error) = result.and_then(|window| {
        window.update(cx, |_, window, _| {
            window.activate_window();
        })
    }) {
        log::error!("failed to open GPUI profiler window: {error:?}");
    }
}

fn existing_devtools_window(
    source_window_id: WindowId,
    cx: &App,
) -> Option<crate::WindowHandle<GpuiDevtoolsWindow>> {
    for window in cx.windows() {
        let Some(window) = window.downcast::<GpuiDevtoolsWindow>() else {
            continue;
        };

        if window
            .read(cx)
            .is_ok_and(|devtools| devtools.source_window_id == source_window_id)
        {
            return Some(window);
        }
    }
    None
}

struct GpuiDevtoolsWindow {
    source_window_id: WindowId,
    _source_window_closed: Subscription,
}

impl GpuiDevtoolsWindow {
    fn new(
        source_window_id: WindowId,
        devtools_window_id: WindowId,
        cx: &mut Context<Self>,
    ) -> Self {
        let source_window_closed = cx.on_window_closed(move |cx, closed_window_id| {
            if closed_window_id != source_window_id {
                return;
            }

            let result = cx.update_window_id(devtools_window_id, |_, window, _| {
                window.remove_window();
            });
            if let Err(error) = result {
                log::debug!("failed to close GPUI profiler window after source closed: {error:?}");
            }
        });

        Self {
            source_window_id,
            _source_window_closed: source_window_closed,
        }
    }

    fn apply_action(
        &mut self,
        action: DevtoolsActionKind,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match action {
            DevtoolsActionKind::Pause => GPUI_DEVTOOLS.write().pause(Instant::now()),
            DevtoolsActionKind::Resume => GPUI_DEVTOOLS.write().resume(),
            DevtoolsActionKind::Clear => GPUI_DEVTOOLS.write().clear_counters(),
            DevtoolsActionKind::Close => {
                close_source_window(self.source_window_id);
                window.remove_window();
            }
        }
        cx.notify();
    }

    fn render_row(&self, row: DevtoolsRow, cx: &mut Context<Self>) -> AnyElement {
        let text_color = match row.kind {
            DevtoolsRowKind::Header => hsla(0.58, 0.44, 0.94, 1.),
            DevtoolsRowKind::Toolbar => hsla(0.12, 0.62, 0.76, 1.),
            DevtoolsRowKind::Data => hsla(0.58, 0.38, 0.92, 0.96),
        };
        let background = match row.kind {
            DevtoolsRowKind::Header | DevtoolsRowKind::Toolbar => rgba(0x273244aa),
            DevtoolsRowKind::Data => rgba(0x00000000),
        };

        div()
            .w_full()
            .min_h(px(22.))
            .flex()
            .flex_row()
            .items_center()
            .gap_1()
            .px_2()
            .bg(background)
            .text_color(text_color)
            .children(row.actions.into_iter().map(|action| {
                div()
                    .id(action.label)
                    .cursor_pointer()
                    .rounded_xs()
                    .border_1()
                    .border_color(hsla(0.58, 0.68, 0.68, 0.52))
                    .bg(if action.active {
                        rgba(0x0ea5e94a)
                    } else {
                        rgba(0x1f29374a)
                    })
                    .px_2()
                    .child(action.label)
                    .on_click(cx.listener(move |this, _, window, cx| {
                        this.apply_action(action.kind, window, cx);
                    }))
            }))
            .child(div().min_w_0().whitespace_nowrap().child(row.text))
            .into_any_element()
    }
}

impl Render for GpuiDevtoolsWindow {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !window_open(self.source_window_id) {
            window.remove_window();
            return div().into_any_element();
        }

        window.request_animation_frame();
        let snapshot = snapshot_window(self.source_window_id);
        div()
            .id("gpui-profiler-window")
            .size_full()
            .overflow_scroll()
            .bg(rgba(0x111827ff))
            .text_color(hsla(0.58, 0.38, 0.92, 0.96))
            .font_family(".SystemUIFont")
            .text_xs()
            .line_height(px(16.))
            .p_2()
            .flex()
            .flex_col()
            .gap_1()
            .children(
                snapshot
                    .rows
                    .into_iter()
                    .map(|row| self.render_row(row, cx)),
            )
            .into_any_element()
    }
}

#[derive(Default)]
struct RenderSummary {
    render_count: usize,
    reuse_count: usize,
    top_sources: Vec<(RenderSourceKey, RenderSourceStats)>,
}

fn top_notify_sources(
    devtools: &GpuiDevTools,
    now: Instant,
    limit: usize,
) -> Vec<(NotifySourceKey, NotifySourceStats)> {
    let mut counts = FxHashMap::default();
    for event in devtools.notifications.iter() {
        let Some(age) = event_age(now, event.timestamp) else {
            continue;
        };
        if age > SOURCE_WINDOW {
            continue;
        }

        let key = NotifySourceKey::from(event);
        let stats = counts
            .entry(key)
            .or_insert_with(|| NotifySourceStats::from_event(event));
        stats.count += 1;
        stats.update_from_event(event);
    }

    let mut counts = counts.into_iter().collect::<Vec<_>>();
    counts.sort_by(|(left_source, left), (right_source, right)| {
        right
            .count
            .cmp(&left.count)
            .then_with(|| {
                short_type_name(left_source.entity_type)
                    .cmp(short_type_name(right_source.entity_type))
            })
            .then_with(|| left_source.caller_file.cmp(right_source.caller_file))
            .then_with(|| left_source.caller_line.cmp(&right_source.caller_line))
    });
    counts.truncate(limit);
    counts
}

fn render_summary(
    devtools: &GpuiDevTools,
    window_id: WindowId,
    now: Instant,
    limit: usize,
) -> RenderSummary {
    let mut summary = RenderSummary::default();
    let mut counts: FxHashMap<RenderSourceKey, RenderSourceStats> = FxHashMap::default();
    let mut reuse_counts_by_entity = FxHashMap::default();

    for event in devtools.renders.iter() {
        let Some(age) = event_age(now, event.timestamp) else {
            continue;
        };
        if event.window_id != window_id || age > FRAME_RATE_WINDOW {
            continue;
        }

        if event.phase.is_reuse() {
            summary.reuse_count += 1;
            *reuse_counts_by_entity.entry(event.entity_id).or_insert(0) += 1;
        } else if event.phase.records_render() {
            summary.render_count += 1;
            counts
                .entry(RenderSourceKey::from(event))
                .or_insert_with(|| RenderSourceStats::from_event(event))
                .record_event(event);
        }
    }

    for (source, stats) in &mut counts {
        stats.reuse_count = reuse_counts_by_entity
            .get(&source.entity_id)
            .copied()
            .unwrap_or(0);
        stats.cause = devtools
            .latest_cause_by_render_source
            .get(source)
            .copied()
            .filter(|cause| cause.is_recent_at(now, SOURCE_WINDOW))
            .filter(|cause| {
                stats
                    .last_timestamp
                    .is_none_or(|last_timestamp| cause.timestamp <= last_timestamp)
            });
    }

    let mut counts = counts.into_iter().collect::<Vec<_>>();
    counts.sort_by(|(left_source, left), (right_source, right)| {
        right
            .duration
            .cmp(&left.duration)
            .then_with(|| right.count.cmp(&left.count))
            .then_with(|| {
                short_type_name(left_source.entity_type)
                    .cmp(short_type_name(right_source.entity_type))
            })
            .then_with(|| {
                left_source
                    .entity_id
                    .as_u64()
                    .cmp(&right_source.entity_id.as_u64())
            })
            .then_with(|| left_source.phase.as_str().cmp(right_source.phase.as_str()))
    });
    counts.truncate(limit);
    summary.top_sources = counts;

    summary
}

fn active_animation_count(devtools: &GpuiDevTools, window_id: WindowId, now: Instant) -> usize {
    let mut sources = FxHashSet::default();
    for event in devtools.animations.iter() {
        let Some(age) = event_age(now, event.timestamp) else {
            continue;
        };
        if event.window_id != window_id || age > ANIMATION_EXPIRY {
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

                sources.insert(ActiveAnimationSource::FrameRequest {
                    entity_type: event.entity_type,
                    caller_file,
                    caller_line: *caller_line,
                    caller_column: *caller_column,
                });
            }
            AnimationEventKind::ElementTick {
                element_id,
                animation_index,
                duration,
                repeats,
            } => {
                if *repeats {
                    sources.insert(ActiveAnimationSource::ElementTick {
                        entity_id: event.entity_id,
                        element_id,
                        animation_index: *animation_index,
                        duration: *duration,
                    });
                }
            }
        }
    }
    sources.len()
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum ActiveAnimationSource<'a> {
    FrameRequest {
        entity_type: &'static str,
        caller_file: &'static str,
        caller_line: u32,
        caller_column: u32,
    },
    ElementTick {
        entity_id: EntityId,
        element_id: &'a str,
        animation_index: usize,
        duration: Duration,
    },
}

#[derive(Clone, Debug)]
struct DevtoolsSnapshot {
    rows: Vec<DevtoolsRow>,
}

#[derive(Clone, Debug)]
struct DevtoolsRow {
    text: String,
    kind: DevtoolsRowKind,
    actions: Vec<DevtoolsAction>,
}

impl DevtoolsRow {
    fn header(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            kind: DevtoolsRowKind::Header,
            actions: Vec::new(),
        }
    }

    fn toolbar(devtools: &GpuiDevTools) -> Self {
        let pause_action = if devtools.paused_at.is_some() {
            DevtoolsAction::toolbar("resume", true, DevtoolsActionKind::Resume)
        } else {
            DevtoolsAction::toolbar("pause", false, DevtoolsActionKind::Pause)
        };

        Self {
            text: String::new(),
            kind: DevtoolsRowKind::Toolbar,
            actions: vec![
                pause_action,
                DevtoolsAction::toolbar("clear", false, DevtoolsActionKind::Clear),
                DevtoolsAction::toolbar("close", false, DevtoolsActionKind::Close),
            ],
        }
    }

    fn plain(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            kind: DevtoolsRowKind::Data,
            actions: Vec::new(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum DevtoolsRowKind {
    Header,
    Toolbar,
    Data,
}

#[derive(Clone, Copy, Debug)]
struct DevtoolsAction {
    label: &'static str,
    active: bool,
    kind: DevtoolsActionKind,
}

impl DevtoolsAction {
    fn toolbar(label: &'static str, active: bool, kind: DevtoolsActionKind) -> Self {
        Self {
            label,
            active,
            kind,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum DevtoolsActionKind {
    Pause,
    Resume,
    Clear,
    Close,
}

fn snapshot_window(window_id: WindowId) -> DevtoolsSnapshot {
    let devtools = GPUI_DEVTOOLS.read();
    let now = devtools.paused_at.unwrap_or_else(Instant::now);
    let mut rows = Vec::new();
    rows.push(DevtoolsRow::header(if devtools.paused_at.is_some() {
        "GPUI profiler paused"
    } else {
        "GPUI profiler"
    }));
    rows.push(DevtoolsRow::toolbar(&devtools));

    let (frame_count, draw_count, dirty_frame_count, last_frame) =
        frame_summary(&devtools, window_id, now);
    rows.push(DevtoolsRow::plain(format!(
        "draw/s {:>3} dirty/s {:>3} frame/s {:>3}",
        draw_count, dirty_frame_count, frame_count
    )));
    if let Some(frame) = last_frame {
        let draw_duration = frame
            .draw_duration
            .map(format_duration_ms)
            .unwrap_or_else(|| "--".to_string());
        rows.push(DevtoolsRow::plain(format!(
            "last {} draw {}ms present {}ms views {} updates {} ops {} quads {}",
            frame.reason,
            draw_duration,
            format_duration_ms(frame.present_duration),
            frame.dirty_view_count,
            frame.invalidator_update_count,
            frame.scene_stats.paint_operation_count,
            frame.scene_stats.quad_count
        )));
    } else {
        rows.push(DevtoolsRow::plain("last frame --"));
    }

    let render_summary = render_summary(&devtools, window_id, now, TOP_SOURCE_COUNT);
    rows.push(DevtoolsRow::plain(format!(
        "renders/s {} reuse/s {}",
        render_summary.render_count, render_summary.reuse_count
    )));
    if render_summary.top_sources.is_empty() {
        rows.push(DevtoolsRow::plain(
            "render: no recent uncached view renders",
        ));
    } else {
        for (index, (source, stats)) in render_summary.top_sources.into_iter().enumerate() {
            let mut labels = stats.cache_miss_reasons.labels();
            if stats.caching_disabled_by_inspector {
                labels.push("inspector");
            }
            let labels = if labels.is_empty() {
                String::new()
            } else {
                format!(" [{}]", labels.join(","))
            };
            let cause = stats
                .cause
                .map(|cause| {
                    format!(
                        " cause {} {}:{}:{}",
                        view_label(cause.source.entity_type, cause.entity_id),
                        file_name(cause.source.caller_file),
                        cause.source.caller_line,
                        cause.caller_column
                    )
                })
                .unwrap_or_default();
            rows.push(DevtoolsRow::plain(format!(
                "render #{:<2} {:<24} {:<12} x{:<3} avg {:>6}ms sum {:>6}ms reuse {:>3}{}{}",
                index + 1,
                view_label(source.entity_type, source.entity_id),
                source.phase.as_str(),
                stats.count,
                format_duration_ms(stats.average_duration()),
                format_duration_ms(stats.duration),
                stats.reuse_count,
                labels,
                cause
            )));
        }
    }

    let notify_sources = top_notify_sources(&devtools, now, TOP_SOURCE_COUNT);
    if notify_sources.is_empty() {
        rows.push(DevtoolsRow::plain("notify: no recent notifications"));
    } else {
        for (index, (source, stats)) in notify_sources.into_iter().enumerate() {
            rows.push(DevtoolsRow::plain(format!(
                "notify #{:<2} {:<24} x{:<3} {}:{}:{} live {}/{} total {}",
                index + 1,
                view_label(source.entity_type, stats.entity_id),
                stats.count,
                file_name(source.caller_file),
                source.caller_line,
                stats.caller_column,
                stats.live_window_count,
                stats.registered_window_count,
                devtools
                    .notify_source_total_counts
                    .get(&source)
                    .copied()
                    .unwrap_or(0)
            )));
        }
    }

    let dirty_path_summary = recent_dirty_path_summary(&devtools, window_id, now);
    rows.push(DevtoolsRow::plain(format!(
        "dirty paths/s {} {}active animations {}",
        dirty_path_summary.count,
        dirty_path_summary
            .last_label
            .as_ref()
            .map(|label| format!("last {label} "))
            .unwrap_or_default(),
        active_animation_count(&devtools, window_id, now)
    )));

    DevtoolsSnapshot { rows }
}

fn frame_summary(
    devtools: &GpuiDevTools,
    window_id: WindowId,
    now: Instant,
) -> (usize, usize, usize, Option<FrameEvent>) {
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
            .find(|frame| frame.timestamp <= now)
            .cloned();
    }
    (frame_count, draw_count, dirty_frame_count, last_frame)
}

#[derive(Default)]
struct DirtyPathSummary {
    count: usize,
    last_label: Option<String>,
}

fn recent_dirty_path_summary(
    devtools: &GpuiDevTools,
    window_id: WindowId,
    now: Instant,
) -> DirtyPathSummary {
    let mut summary = DirtyPathSummary::default();
    for event in devtools.dirty_paths.iter() {
        if event.window_id != window_id
            || !event_age(now, event.timestamp).is_some_and(|age| age <= FRAME_RATE_WINDOW)
        {
            continue;
        }

        summary.count += 1;
        summary.last_label = Some(dirty_path_label(event));
    }
    summary
}

fn dirty_path_label(event: &DirtyPathEvent) -> String {
    if event.path.is_empty() {
        return format!(
            "{}#{}",
            short_type_name(event.invalidated_entity_type),
            event.invalidated_entity_id.as_u64()
        );
    }

    let mut path = event
        .path
        .iter()
        .take(2)
        .map(|segment| view_label(segment.entity_type, segment.entity_id))
        .collect::<Vec<_>>()
        .join("<");
    if event.path.len() > 2 {
        path.push_str("<...");
    }
    format!(
        "{}#{} {}",
        short_type_name(event.invalidated_entity_type),
        event.invalidated_entity_id.as_u64(),
        path
    )
}

fn format_duration_ms(duration: Duration) -> String {
    let ms = duration.as_secs_f64() * 1000.;
    if ms < 0.1 {
        format!("{ms:.2}")
    } else if ms < 10. {
        format!("{ms:.1}")
    } else {
        format!("{ms:.0}")
    }
}

fn view_label(entity_type: &'static str, entity_id: EntityId) -> String {
    format!("{}#{}", short_type_name(entity_type), entity_id.as_u64())
}

fn short_type_name(name: &'static str) -> &'static str {
    name.rsplit("::").next().unwrap_or(name)
}

fn file_name(path: &'static str) -> &'static str {
    path.rsplit('/').next().unwrap_or(path)
}

#[cfg(test)]
mod tests {
    use super::super::{CacheMissReasons, NotifyEvent, ViewRenderEvent, ViewRenderPhase};
    use super::*;

    #[test]
    fn devtools_render_summary_keeps_same_type_views_separate() {
        let mut devtools = GpuiDevTools::new();
        let now = Instant::now();
        let window_id = WindowId::from(1);
        for (entity_id, duration) in [(1, 1), (1, 1), (2, 3)] {
            devtools.renders.push(ViewRenderEvent {
                window_id,
                entity_id: EntityId::from(entity_id),
                entity_type: "Editor",
                phase: ViewRenderPhase::UncachedRender,
                duration: Some(Duration::from_millis(duration)),
                cache_miss_reasons: CacheMissReasons::empty(),
                bounds: None,
                caching_disabled_by_inspector: false,
                timestamp: now,
            });
        }

        let summary = render_summary(&devtools, window_id, now, TOP_SOURCE_COUNT);
        assert_eq!(summary.render_count, 3);
        assert_eq!(summary.top_sources.len(), 2);
        assert_eq!(summary.top_sources[0].0.entity_id, EntityId::from(2));
        assert_eq!(summary.top_sources[1].0.entity_id, EntityId::from(1));
    }

    #[test]
    fn devtools_notify_sources_respect_limit() {
        let mut devtools = GpuiDevTools::new();
        let now = Instant::now();
        for index in 0..20 {
            devtools.notifications.push(NotifyEvent {
                entity_id: EntityId::from((index + 1) as u64),
                entity_type: "Editor",
                caller_file: "crates/editor/src/editor.rs",
                caller_line: index as u32,
                caller_column: 1,
                registered_window_count: 1,
                live_window_count: 1,
                timestamp: now,
            });
        }

        assert_eq!(
            top_notify_sources(&devtools, now, TOP_SOURCE_COUNT).len(),
            TOP_SOURCE_COUNT
        );
    }
}
