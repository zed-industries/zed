use std::{
    ops::Range,
    path::PathBuf,
    rc::Rc,
    time::{Duration, Instant},
};

use gpui::{
    App, AppContext, ClipboardItem, Context, Div, Entity, Hsla, InteractiveElement,
    ParentElement as _, Render, SerializedTaskTiming, SharedString, StatefulInteractiveElement,
    Styled, Task, TaskTiming, TitlebarOptions, UniformListScrollHandle, WindowBounds, WindowHandle,
    WindowOptions, div, prelude::FluentBuilder, px, relative, size, uniform_list,
};
use util::ResultExt;
use workspace::{
    Workspace,
    ui::{
        ActiveTheme, Button, ButtonCommon, ButtonStyle, Checkbox, Clickable, Divider,
        ScrollableHandle as _, ToggleState, Tooltip, WithScrollbar, h_flex, v_flex,
    },
};
use zed_actions::OpenPerformanceProfiler;

pub fn init(startup_time: Instant, cx: &mut App) {
    cx.observe_new(move |workspace: &mut workspace::Workspace, _, _| {
        workspace.register_action(move |workspace, _: &OpenPerformanceProfiler, window, cx| {
            let window_handle = window
                .window_handle()
                .downcast::<Workspace>()
                .expect("Workspaces are root Windows");
            open_performance_profiler(startup_time, workspace, window_handle, cx);
        });
    })
    .detach();
}

fn open_performance_profiler(
    startup_time: Instant,
    _workspace: &mut workspace::Workspace,
    workspace_handle: WindowHandle<Workspace>,
    cx: &mut App,
) {
    let existing_window = cx
        .windows()
        .into_iter()
        .find_map(|window| window.downcast::<ProfilerWindow>());

    if let Some(existing_window) = existing_window {
        existing_window
            .update(cx, |profiler_window, window, _cx| {
                profiler_window.workspace = Some(workspace_handle);
                window.activate_window();
            })
            .log_err();
        return;
    }

    let default_bounds = size(px(1280.), px(720.)); // 16:9

    cx.open_window(
        WindowOptions {
            titlebar: Some(TitlebarOptions {
                title: Some("Profiler Window".into()),
                appears_transparent: false,
                traffic_light_position: None,
            }),
            focus: true,
            show: true,
            is_movable: true,
            kind: gpui::WindowKind::Normal,
            window_background: cx.theme().window_background_appearance(),
            window_decorations: None,
            window_min_size: Some(default_bounds),
            window_bounds: Some(WindowBounds::centered(default_bounds, cx)),
            ..Default::default()
        },
        |_window, cx| ProfilerWindow::new(startup_time, Some(workspace_handle), cx),
    )
    .log_err();
}

enum DataMode {
    Realtime(Option<Vec<TaskTiming>>),
    Snapshot(Vec<TaskTiming>),
}

struct TimingBar {
    location: &'static core::panic::Location<'static>,
    start: Instant,
    end: Instant,
    color: Hsla,
}

pub struct ProfilerWindow {
    startup_time: Instant,
    data: DataMode,
    include_self_timings: ToggleState,
    autoscroll: bool,
    scroll_handle: UniformListScrollHandle,
    workspace: Option<WindowHandle<Workspace>>,
    _refresh: Option<Task<()>>,
}

impl ProfilerWindow {
    pub fn new(
        startup_time: Instant,
        workspace_handle: Option<WindowHandle<Workspace>>,
        cx: &mut App,
    ) -> Entity<Self> {
        let entity = cx.new(|cx| ProfilerWindow {
            startup_time,
            data: DataMode::Realtime(None),
            include_self_timings: ToggleState::Unselected,
            autoscroll: true,
            scroll_handle: UniformListScrollHandle::default(),
            workspace: workspace_handle,
            _refresh: Some(Self::begin_listen(cx)),
        });

        entity
    }

    fn begin_listen(cx: &mut Context<Self>) -> Task<()> {
        cx.spawn(async move |this, cx| {
            loop {
                let data = cx
                    .foreground_executor()
                    .dispatcher
                    .get_current_thread_timings();

                this.update(cx, |this: &mut ProfilerWindow, cx| {
                    this.data = DataMode::Realtime(Some(data));
                    cx.notify();
                })
                .ok();

                // yield to the executor
                cx.background_executor()
                    .timer(Duration::from_micros(1))
                    .await;
            }
        })
    }

    fn get_timings(&self) -> Option<&Vec<TaskTiming>> {
        match &self.data {
            DataMode::Realtime(data) => data.as_ref(),
            DataMode::Snapshot(data) => Some(data),
        }
    }

    fn render_timing(value_range: Range<Instant>, item: TimingBar, cx: &App) -> Div {
        let time_ms = item.end.duration_since(item.start).as_secs_f32() * 1000f32;

        let remap = value_range
            .end
            .duration_since(value_range.start)
            .as_secs_f32()
            * 1000f32;

        let start = (item.start.duration_since(value_range.start).as_secs_f32() * 1000f32) / remap;
        let end = (item.end.duration_since(value_range.start).as_secs_f32() * 1000f32) / remap;

        let bar_width = end - start.abs();

        let location = item
            .location
            .file()
            .rsplit_once("/")
            .unwrap_or(("", item.location.file()))
            .1;
        let location = location.rsplit_once("\\").unwrap_or(("", location)).1;

        let label = SharedString::from(format!(
            "{}:{}:{}",
            location,
            item.location.line(),
            item.location.column()
        ));

        h_flex()
            .gap_2()
            .w_full()
            .h(px(32.0))
            .child(
                div()
                    .id(label.clone())
                    .w(px(200.0))
                    .flex_shrink_0()
                    .overflow_hidden()
                    .child(div().text_ellipsis().child(label.clone()))
                    .tooltip(Tooltip::text(label.clone()))
                    .on_click(move |_, _, cx| {
                        cx.write_to_clipboard(ClipboardItem::new_string(label.to_string()))
                    }),
            )
            .child(
                div()
                    .flex_1()
                    .h(px(24.0))
                    .bg(cx.theme().colors().background)
                    .rounded_md()
                    .p(px(2.0))
                    .relative()
                    .child(
                        div()
                            .absolute()
                            .h_full()
                            .rounded_sm()
                            .bg(item.color)
                            .left(relative(start.max(0f32)))
                            .w(relative(bar_width)),
                    ),
            )
            .child(
                div()
                    .min_w(px(70.))
                    .flex_shrink_0()
                    .text_right()
                    .child(format!("{:.1} ms", time_ms)),
            )
    }
}

impl Render for ProfilerWindow {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        let scroll_offset = self.scroll_handle.offset();
        let max_offset = self.scroll_handle.max_offset();
        self.autoscroll = -scroll_offset.y >= (max_offset.height - px(24.));
        if self.autoscroll {
            self.scroll_handle.scroll_to_bottom();
        }

        v_flex()
            .id("profiler")
            .w_full()
            .h_full()
            .bg(cx.theme().colors().surface_background)
            .text_color(cx.theme().colors().text)
            .child(
                h_flex()
                    .py_2()
                    .px_4()
                    .w_full()
                    .justify_between()
                    .child(
                        h_flex()
                            .gap_2()
                            .child(
                                Button::new(
                                    "switch-mode",
                                    match self.data {
                                        DataMode::Snapshot { .. } => "Resume",
                                        DataMode::Realtime(_) => "Pause",
                                    },
                                )
                                .style(ButtonStyle::Filled)
                                .on_click(cx.listener(
                                    |this, _, _window, cx| {
                                        match &this.data {
                                            DataMode::Realtime(Some(data)) => {
                                                this._refresh = None;
                                                this.data = DataMode::Snapshot(data.clone());
                                            }
                                            DataMode::Snapshot { .. } => {
                                                this._refresh = Some(Self::begin_listen(cx));
                                                this.data = DataMode::Realtime(None);
                                            }
                                            _ => {}
                                        };
                                        cx.notify();
                                    },
                                )),
                            )
                            .child(
                                Button::new("export-data", "Save")
                                    .style(ButtonStyle::Filled)
                                    .on_click(cx.listener(|this, _, _window, cx| {
                                        let Some(workspace) = this.workspace else {
                                            return;
                                        };

                                        let Some(data) = this.get_timings() else {
                                            return;
                                        };
                                        let timings =
                                            SerializedTaskTiming::convert(this.startup_time, &data);

                                        let active_path = workspace
                                            .read_with(cx, |workspace, cx| {
                                                workspace.most_recent_active_path(cx)
                                            })
                                            .log_err()
                                            .flatten()
                                            .and_then(|p| p.parent().map(|p| p.to_owned()))
                                            .unwrap_or_else(|| PathBuf::default());

                                        let path = cx.prompt_for_new_path(
                                            &active_path,
                                            Some("performance_profile.miniprof"),
                                        );

                                        cx.background_spawn(async move {
                                            let path = path.await;
                                            let path =
                                                path.log_err().and_then(|p| p.log_err()).flatten();

                                            let Some(path) = path else {
                                                return;
                                            };

                                            let Some(timings) =
                                                serde_json::to_string(&timings).log_err()
                                            else {
                                                return;
                                            };

                                            smol::fs::write(path, &timings).await.log_err();
                                        })
                                        .detach();
                                    })),
                            ),
                    )
                    .child(
                        Checkbox::new("include-self", self.include_self_timings)
                            .label("Include profiler timings")
                            .on_click(cx.listener(|this, checked, _window, cx| {
                                this.include_self_timings = *checked;
                                cx.notify();
                            })),
                    ),
            )
            .when_some(self.get_timings(), |div, e| {
                if e.len() == 0 {
                    return div;
                }

                let min = e[0].start;
                let max = e[e.len() - 1].end.unwrap_or_else(|| Instant::now());
                let timings = Rc::new(
                    e.into_iter()
                        .filter(|timing| {
                            timing
                                .end
                                .unwrap_or_else(|| Instant::now())
                                .duration_since(timing.start)
                                .as_millis()
                                >= 1
                        })
                        .filter(|timing| {
                            if self.include_self_timings.selected() {
                                true
                            } else {
                                !timing.location.file().ends_with("miniprofiler_ui.rs")
                            }
                        })
                        .cloned()
                        .collect::<Vec<_>>(),
                );

                div.child(Divider::horizontal()).child(
                    v_flex()
                        .id("timings.bars")
                        .w_full()
                        .h_full()
                        .gap_2()
                        .child(
                            uniform_list("list", timings.len(), {
                                let timings = timings.clone();
                                move |visible_range, _, cx| {
                                    let mut items = vec![];
                                    for i in visible_range {
                                        let timing = &timings[i];
                                        let value_range =
                                            max.checked_sub(Duration::from_secs(10)).unwrap_or(min)
                                                ..max;
                                        items.push(Self::render_timing(
                                            value_range,
                                            TimingBar {
                                                location: timing.location,
                                                start: timing.start,
                                                end: timing.end.unwrap_or_else(|| Instant::now()),
                                                color: cx
                                                    .theme()
                                                    .accents()
                                                    .color_for_index(i as u32),
                                            },
                                            cx,
                                        ));
                                    }
                                    items
                                }
                            })
                            .p_4()
                            .on_scroll_wheel(cx.listener(|this, _, _, cx| {
                                this.autoscroll = false;
                                cx.notify();
                            }))
                            .track_scroll(&self.scroll_handle)
                            .size_full(),
                        )
                        .vertical_scrollbar_for(&self.scroll_handle, window, cx),
                )
            })
    }
}
