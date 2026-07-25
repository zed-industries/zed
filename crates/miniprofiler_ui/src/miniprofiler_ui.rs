use std::{
    hash::{DefaultHasher, Hash, Hasher},
    path::PathBuf,
    rc::Rc,
    time::Instant,
};

#[cfg(feature = "zed")]
use command_palette_hooks::CommandPaletteFilter;
use gpui::{
    App, AppContext, ClipboardItem, Context, Div, Entity, Hsla, InteractiveElement,
    ParentElement as _, ProfilingCollector, Render, SerializedLocation, SerializedTaskTiming,
    SerializedThreadTaskTimings, SharedString, StatefulInteractiveElement, Styled, TasksIncluded,
    ThreadTimingsDelta, UniformListScrollHandle, div, prelude::FluentBuilder, profiler, px,
    relative, uniform_list,
};
#[cfg(feature = "zed")]
use gpui::{Task, TitlebarOptions, WeakEntity, WindowBounds, WindowOptions, size};
#[cfg(feature = "zed")]
use rpc::{AnyProtoClient, proto};
#[cfg(feature = "zed")]
use settings::{RegisterSetting, Settings, SettingsContent, SettingsStore};
#[cfg(feature = "zed")]
use std::any::TypeId;
#[cfg(feature = "zed")]
use std::time::Duration;
use ui::{
    ActiveTheme, Button, ButtonCommon, ButtonStyle, Checkbox, Clickable, ContextMenu, Divider,
    DropdownMenu, ScrollAxes, ScrollableHandle as _, Scrollbars, ToggleState, Tooltip,
    WithScrollbar, h_flex, v_flex,
};
use util::ResultExt;
#[cfg(feature = "zed")]
use workspace::Workspace;
#[cfg(feature = "zed")]
use zed_actions::OpenPerformanceProfiler;

const NANOS_PER_MS: u128 = 1_000_000;
const VISIBLE_WINDOW_NANOS: u128 = 10 * 1_000_000_000;
#[cfg(feature = "zed")]
const REMOTE_POLL_INTERVAL: Duration = Duration::from_millis(500);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ProfileSource {
    Foreground,
    AllThreads,
    #[cfg(feature = "zed")]
    RemoteForeground,
    #[cfg(feature = "zed")]
    RemoteAllThreads,
}

impl ProfileSource {
    fn label(&self) -> &'static str {
        match self {
            ProfileSource::Foreground => "Foreground",
            ProfileSource::AllThreads => "All threads",
            #[cfg(feature = "zed")]
            ProfileSource::RemoteForeground => "Remote: Foreground",
            #[cfg(feature = "zed")]
            ProfileSource::RemoteAllThreads => "Remote: All threads",
        }
    }

    #[cfg(feature = "zed")]
    fn is_remote(&self) -> bool {
        match self {
            ProfileSource::Foreground | ProfileSource::AllThreads => false,
            ProfileSource::RemoteForeground | ProfileSource::RemoteAllThreads => true,
        }
    }

    fn foreground_only(&self) -> bool {
        match self {
            ProfileSource::Foreground => true,
            ProfileSource::AllThreads => false,
            #[cfg(feature = "zed")]
            ProfileSource::RemoteForeground => true,
            #[cfg(feature = "zed")]
            ProfileSource::RemoteAllThreads => false,
        }
    }
}

#[cfg(feature = "zed")]
#[derive(Clone, Copy, Debug, Default, RegisterSetting)]
struct PerformanceProfilerSettings {
    enabled: bool,
}

#[cfg(feature = "zed")]
impl Settings for PerformanceProfilerSettings {
    fn from_settings(content: &SettingsContent) -> Self {
        let instrumentation = content.instrumentation.as_ref().unwrap();
        let profiler = instrumentation.performance_profiler.as_ref().unwrap();
        Self {
            enabled: profiler.enabled.unwrap(),
        }
    }
}

#[cfg(feature = "zed")]
pub fn init(startup_time: Instant, cx: &mut App) {
    let initial_enabled = PerformanceProfilerSettings::get_global(cx).enabled;
    profiler::set_trace_enabled(initial_enabled);
    update_command_palette_filter(initial_enabled, cx);

    cx.observe_global::<SettingsStore>(|cx| {
        let enabled = PerformanceProfilerSettings::get_global(cx).enabled;
        // `set_enabled` reports whether the value actually changed, so skip the
        // filter update and window cleanup on the common no-op path — the
        // settings observer fires for every settings change.
        if !profiler::set_trace_enabled(enabled) {
            return;
        }
        update_command_palette_filter(enabled, cx);
        if !enabled {
            for window in cx
                .windows()
                .into_iter()
                .filter_map(|window| window.downcast::<ProfilerWindow>())
            {
                window
                    .update(cx, |_, window, _| window.remove_window())
                    .ok();
            }
        }
    })
    .detach();

    cx.observe_new(move |workspace: &mut workspace::Workspace, _, cx| {
        let workspace_handle = cx.entity().downgrade();
        workspace.register_action(move |_workspace, _: &OpenPerformanceProfiler, window, cx| {
            open_performance_profiler(startup_time, workspace_handle.clone(), window, cx);
        });
    })
    .detach();
}

#[cfg(feature = "zed")]
fn update_command_palette_filter(enabled: bool, cx: &mut App) {
    CommandPaletteFilter::update_global(cx, |filter, _| {
        let action = [TypeId::of::<OpenPerformanceProfiler>()];
        if enabled {
            filter.show_action_types(&action);
        } else {
            filter.hide_action_types(&action);
        }
    });
}

#[cfg(feature = "zed")]
fn open_performance_profiler(
    startup_time: Instant,
    workspace_handle: WeakEntity<Workspace>,
    _window: &mut gpui::Window,
    cx: &mut App,
) {
    let existing_window = cx
        .windows()
        .into_iter()
        .find_map(|window| window.downcast::<ProfilerWindow>());

    if let Some(existing_window) = existing_window {
        existing_window
            .update(cx, |profiler_window, window, _cx| {
                profiler_window.workspace = Some(workspace_handle.clone());
                window.activate_window();
            })
            .log_err();
        return;
    }

    let window_background = cx.theme().window_background_appearance();
    let default_bounds = size(px(1280.), px(720.));

    cx.defer(move |cx| {
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
                window_background,
                window_decorations: None,
                window_min_size: Some(default_bounds),
                window_bounds: Some(WindowBounds::centered(default_bounds, cx)),
                ..Default::default()
            },
            |_window, cx| ProfilerWindow::new_for_workspace(startup_time, workspace_handle, cx),
        )
        .log_err();
    });
}

struct TimingBar {
    location: SerializedLocation,
    start_nanos: u128,
    duration_nanos: u128,
    color: Hsla,
}

pub struct ProfilerWindow {
    collector: ProfilingCollector,
    source: ProfileSource,
    timings: Vec<SerializedThreadTaskTimings>,
    paused: bool,
    display_timings: Rc<Vec<SerializedTaskTiming>>,
    include_self_timings: ToggleState,
    autoscroll: bool,
    scroll_handle: UniformListScrollHandle,
    #[cfg(feature = "zed")]
    workspace: Option<WeakEntity<Workspace>>,
    #[cfg(feature = "zed")]
    has_remote: bool,
    #[cfg(feature = "zed")]
    remote_now_nanos: u128,
    #[cfg(feature = "zed")]
    remote_received_at: Option<Instant>,
    #[cfg(feature = "zed")]
    _remote_poll_task: Option<Task<()>>,
}

impl ProfilerWindow {
    pub fn new(startup_time: Instant, cx: &mut App) -> Entity<Self> {
        cx.new(|_cx| ProfilerWindow {
            collector: ProfilingCollector::new(startup_time),
            source: ProfileSource::Foreground,
            timings: Vec::new(),
            paused: false,
            display_timings: Rc::new(Vec::new()),
            include_self_timings: ToggleState::Unselected,
            autoscroll: true,
            scroll_handle: UniformListScrollHandle::default(),
            #[cfg(feature = "zed")]
            workspace: None,
            #[cfg(feature = "zed")]
            has_remote: false,
            #[cfg(feature = "zed")]
            remote_now_nanos: 0,
            #[cfg(feature = "zed")]
            remote_received_at: None,
            #[cfg(feature = "zed")]
            _remote_poll_task: None,
        })
    }

    #[cfg(feature = "zed")]
    fn new_for_workspace(
        startup_time: Instant,
        workspace_handle: WeakEntity<Workspace>,
        cx: &mut App,
    ) -> Entity<Self> {
        let profiler = Self::new(startup_time, cx);
        profiler.update(cx, |profiler, _| {
            profiler.workspace = Some(workspace_handle);
        });
        profiler
    }

    fn poll_timings(&mut self, cx: &App) {
        #[cfg(not(feature = "zed"))]
        let _ = cx;
        #[cfg(feature = "zed")]
        {
            self.has_remote = self.remote_proto_client(cx).is_some();
        }
        match self.source {
            ProfileSource::Foreground => {
                let current_thread =
                    profiler::get_current_thread_timings(TasksIncluded::OnlyCompleted);
                let deltas = self.collector.collect_unseen(vec![current_thread]);
                self.apply_deltas(deltas);
            }
            ProfileSource::AllThreads => {
                let all_timings = profiler::get_all_timings(TasksIncluded::OnlyCompleted);
                let deltas = self.collector.collect_unseen(all_timings);
                self.apply_deltas(deltas);
            }
            #[cfg(feature = "zed")]
            ProfileSource::RemoteForeground | ProfileSource::RemoteAllThreads => {
                // Remote timings arrive asynchronously via apply_remote_response.
            }
        }
        self.rebuild_display_timings();
    }

    fn rebuild_display_timings(&mut self) {
        let include_self = self.include_self_timings.selected();
        let cutoff_nanos = self.now_nanos().saturating_sub(VISIBLE_WINDOW_NANOS);

        let per_thread: Vec<Vec<SerializedTaskTiming>> = self
            .timings
            .iter()
            .map(|thread| {
                let visible = visible_tail(&thread.timings, cutoff_nanos);
                filter_timings(visible.iter().cloned(), include_self)
            })
            .collect();
        self.display_timings = Rc::new(kway_merge(per_thread));
    }

    fn now_nanos(&self) -> u128 {
        #[cfg(feature = "zed")]
        {
            if self.source.is_remote() {
                let elapsed_since_poll = self
                    .remote_received_at
                    .map(|at| Instant::now().duration_since(at).as_nanos())
                    .unwrap_or(0);
                return self.remote_now_nanos + elapsed_since_poll;
            }
        }

        Instant::now()
            .duration_since(self.collector.startup_time())
            .as_nanos()
    }

    fn set_source(&mut self, source: ProfileSource, cx: &mut Context<Self>) {
        #[cfg(not(feature = "zed"))]
        let _ = cx;
        if self.source == source {
            return;
        }

        self.source = source;

        self.timings.clear();
        self.collector.reset();
        self.display_timings = Rc::new(Vec::new());
        #[cfg(feature = "zed")]
        {
            self.remote_now_nanos = 0;
            self.remote_received_at = None;
            self.has_remote = self.remote_proto_client(cx).is_some();

            if source.is_remote() {
                self.start_remote_polling(cx);
            } else {
                self._remote_poll_task = None;
            }
        }
    }

    #[cfg(feature = "zed")]
    fn remote_proto_client(&self, cx: &App) -> Option<AnyProtoClient> {
        let workspace = self.workspace.as_ref()?;
        workspace
            .read_with(cx, |workspace, cx| {
                let project = workspace.project().read(cx);
                let remote_client = project.remote_client()?;
                Some(remote_client.read(cx).proto_client())
            })
            .log_err()
            .flatten()
    }

    #[cfg(feature = "zed")]
    fn start_remote_polling(&mut self, cx: &mut Context<Self>) {
        let Some(proto_client) = self.remote_proto_client(cx) else {
            return;
        };

        let source_foreground_only = self.source.foreground_only();
        let weak = cx.weak_entity();
        self._remote_poll_task = Some(cx.spawn(async move |_this, cx| {
            loop {
                let response = proto_client
                    .request(proto::GetRemoteProfilingData {
                        project_id: proto::REMOTE_SERVER_PROJECT_ID,
                        foreground_only: source_foreground_only,
                    })
                    .await;

                match response {
                    Ok(response) => {
                        let ok = weak.update(&mut cx.clone(), |this, cx| {
                            this.apply_remote_response(response);
                            cx.notify();
                        });
                        if ok.is_err() {
                            break;
                        }
                    }
                    Err(error) => {
                        Err::<(), _>(error).log_err();
                    }
                }

                cx.background_executor().timer(REMOTE_POLL_INTERVAL).await;
            }
        }));
    }

    #[cfg(feature = "zed")]
    fn apply_remote_response(&mut self, response: proto::GetRemoteProfilingDataResponse) {
        self.has_remote = true;
        self.remote_now_nanos = response.now_nanos as u128;
        self.remote_received_at = Some(Instant::now());
        let deltas = response
            .threads
            .into_iter()
            .map(|thread| {
                let new_timings = thread
                    .timings
                    .into_iter()
                    .map(|t| {
                        let location = t.location.unwrap_or_default();
                        SerializedTaskTiming {
                            location: SerializedLocation {
                                file: SharedString::from(location.file),
                                line: location.line,
                                column: location.column,
                            },
                            start: t.start_nanos as u128,
                            duration: t.duration_nanos as u128,
                        }
                    })
                    .collect();
                ThreadTimingsDelta {
                    thread_id: thread.thread_id,
                    thread_name: thread.thread_name,
                    new_timings,
                }
            })
            .collect();

        self.apply_deltas(deltas);
        self.rebuild_display_timings();
    }

    fn apply_deltas(&mut self, deltas: Vec<ThreadTimingsDelta>) {
        for delta in deltas {
            append_to_thread(
                &mut self.timings,
                delta.thread_id,
                delta.thread_name,
                delta.new_timings,
            );
        }
    }

    fn save_directory(&self, cx: &App) -> PathBuf {
        #[cfg(feature = "zed")]
        if let Some(active_path) = self
            .workspace
            .as_ref()
            .and_then(|workspace| {
                workspace
                    .read_with(cx, |workspace, cx| workspace.most_recent_active_path(cx))
                    .log_err()
                    .flatten()
            })
            .and_then(|path| path.parent().map(PathBuf::from))
        {
            return active_path;
        }

        #[cfg(not(feature = "zed"))]
        let _ = cx;
        PathBuf::default()
    }

    fn render_source_dropdown(
        &self,
        window: &mut gpui::Window,
        cx: &mut Context<Self>,
    ) -> DropdownMenu {
        let weak = cx.weak_entity();
        let current_source = self.source;
        #[cfg(feature = "zed")]
        let has_remote = self.has_remote;

        #[cfg(not(feature = "zed"))]
        let sources = vec![ProfileSource::Foreground, ProfileSource::AllThreads];
        #[cfg(feature = "zed")]
        let mut sources = vec![ProfileSource::Foreground, ProfileSource::AllThreads];
        #[cfg(feature = "zed")]
        if has_remote {
            sources.push(ProfileSource::RemoteForeground);
            sources.push(ProfileSource::RemoteAllThreads);
        }

        DropdownMenu::new(
            "profile-source",
            current_source.label(),
            ContextMenu::build(window, cx, move |mut menu, window, cx| {
                for source in &sources {
                    let source = *source;
                    let weak = weak.clone();
                    menu = menu.entry(source.label(), None, move |_, cx| {
                        weak.update(cx, |this, cx| {
                            this.set_source(source, cx);
                            cx.notify();
                        })
                        .log_err();
                    });
                }
                if let Some(index) = sources.iter().position(|s| *s == current_source) {
                    for _ in 0..=index {
                        menu.select_next(&Default::default(), window, cx);
                    }
                }
                menu
            }),
        )
    }

    fn render_timing(
        window_start_nanos: u128,
        window_duration_nanos: u128,
        item: TimingBar,
        cx: &App,
    ) -> Div {
        let time_ms = item.duration_nanos as f32 / NANOS_PER_MS as f32;

        let start_fraction = if item.start_nanos >= window_start_nanos {
            (item.start_nanos - window_start_nanos) as f32 / window_duration_nanos as f32
        } else {
            0.0
        };

        let end_nanos = item.start_nanos + item.duration_nanos;
        let end_fraction = if end_nanos >= window_start_nanos {
            (end_nanos - window_start_nanos) as f32 / window_duration_nanos as f32
        } else {
            0.0
        };

        let start_fraction = start_fraction.clamp(0.0, 1.0);
        let end_fraction = end_fraction.clamp(0.0, 1.0);
        let bar_width = (end_fraction - start_fraction).max(0.0);

        let file_str: &str = &item.location.file;
        let basename = file_str.rsplit_once("/").unwrap_or(("", file_str)).1;
        let basename = basename.rsplit_once("\\").unwrap_or(("", basename)).1;

        let label = SharedString::from(format!(
            "{}:{}:{}",
            basename, item.location.line, item.location.column
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
                            .left(relative(start_fraction.max(0.0)))
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
        let ui_font = theme_settings::setup_ui_font(window, cx);
        if !self.paused {
            self.poll_timings(cx);
            window.request_animation_frame();
        }

        let scroll_offset = self.scroll_handle.offset();
        let max_offset = self.scroll_handle.max_offset();
        self.autoscroll = -scroll_offset.y >= (max_offset.y - px(24.));
        if self.autoscroll {
            self.scroll_handle.scroll_to_bottom();
        }

        let display_timings = self.display_timings.clone();

        v_flex()
            .id("profiler")
            .font(ui_font)
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
                            .child(self.render_source_dropdown(window, cx))
                            .child(
                                Button::new(
                                    "switch-mode",
                                    if self.paused { "Resume" } else { "Pause" },
                                )
                                .style(ButtonStyle::Filled)
                                .on_click(cx.listener(
                                    |this, _, _window, cx| {
                                        this.paused = !this.paused;
                                        #[cfg(feature = "zed")]
                                        if !this.paused && this.source.is_remote() {
                                            this.start_remote_polling(cx);
                                        } else if this.paused && this.source.is_remote() {
                                            this._remote_poll_task = None;
                                        }
                                        cx.notify();
                                    },
                                )),
                            )
                            .child(
                                Button::new("export-data", "Save")
                                    .style(ButtonStyle::Filled)
                                    .on_click(cx.listener(|this, _, _window, cx| {
                                        if this.timings.iter().all(|t| t.timings.is_empty()) {
                                            return;
                                        }

                                        let serialized = if this.source.foreground_only() {
                                            let flat: Vec<&SerializedTaskTiming> = this
                                                .timings
                                                .iter()
                                                .flat_map(|t| &t.timings)
                                                .collect();
                                            serde_json::to_string(&flat)
                                        } else {
                                            serde_json::to_string(&this.timings)
                                        };

                                        let Some(serialized) = serialized.log_err() else {
                                            return;
                                        };

                                        let active_path = this.save_directory(cx);

                                        let path = cx.prompt_for_new_path(
                                            &active_path,
                                            Some("performance_profile.miniprof.json"),
                                        );

                                        cx.background_spawn(async move {
                                            let path = path.await;
                                            let path =
                                                path.log_err().and_then(|p| p.log_err()).flatten();

                                            let Some(path) = path else {
                                                return;
                                            };

                                            smol::fs::write(path, &serialized).await.log_err();
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
            .when(!display_timings.is_empty(), |div| {
                let now_nanos = self.now_nanos();

                let window_start_nanos = now_nanos.saturating_sub(VISIBLE_WINDOW_NANOS);
                let window_duration_nanos = VISIBLE_WINDOW_NANOS;

                div.child(Divider::horizontal()).child(
                    v_flex()
                        .id("timings.bars")
                        .w_full()
                        .h_full()
                        .gap_2()
                        .child(
                            uniform_list("list", display_timings.len(), {
                                let timings = display_timings.clone();
                                move |visible_range, _, cx| {
                                    let mut items = vec![];
                                    for i in visible_range {
                                        let timing = &timings[i];
                                        items.push(Self::render_timing(
                                            window_start_nanos,
                                            window_duration_nanos,
                                            TimingBar {
                                                location: timing.location.clone(),
                                                start_nanos: timing.start,
                                                duration_nanos: timing.duration,
                                                color: cx.theme().accents().color_for_index(
                                                    location_color_index(&timing.location),
                                                ),
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
                        .custom_scrollbars(
                            Scrollbars::always_visible(ScrollAxes::Vertical)
                                .tracked_scroll_handle(&self.scroll_handle),
                            window,
                            cx,
                        ),
                )
            })
    }
}

const MAX_VISIBLE_PER_THREAD: usize = 10_000;

fn visible_tail(timings: &[SerializedTaskTiming], cutoff_nanos: u128) -> &[SerializedTaskTiming] {
    let len = timings.len();
    let limit = len.min(MAX_VISIBLE_PER_THREAD);
    let search_start = len - limit;
    let tail = &timings[search_start..];

    let mut first_visible = 0;
    for (i, timing) in tail.iter().enumerate().rev() {
        if timing.start + timing.duration < cutoff_nanos {
            first_visible = i + 1;
            break;
        }
    }
    &tail[first_visible..]
}

fn filter_timings(
    timings: impl Iterator<Item = SerializedTaskTiming>,
    include_self: bool,
) -> Vec<SerializedTaskTiming> {
    timings
        .filter(|t| t.duration / NANOS_PER_MS >= 1)
        .filter(|t| include_self || !t.location.file.ends_with("miniprofiler_ui.rs"))
        .collect()
}

fn location_color_index(location: &SerializedLocation) -> u32 {
    let mut hasher = DefaultHasher::new();
    location.file.hash(&mut hasher);
    location.line.hash(&mut hasher);
    location.column.hash(&mut hasher);
    hasher.finish() as u32
}

/// Merge K sorted `Vec<SerializedTaskTiming>` into a single sorted vec.
/// Each input vec must already be sorted by `start`.
fn kway_merge(lists: Vec<Vec<SerializedTaskTiming>>) -> Vec<SerializedTaskTiming> {
    let total_len: usize = lists.iter().map(|l| l.len()).sum();
    let mut result = Vec::with_capacity(total_len);
    let mut cursors = vec![0usize; lists.len()];

    loop {
        let mut min_start = u128::MAX;
        let mut min_list = None;

        for (list_idx, list) in lists.iter().enumerate() {
            let cursor = cursors[list_idx];
            if let Some(timing) = list.get(cursor) {
                if timing.start < min_start {
                    min_start = timing.start;
                    min_list = Some(list_idx);
                }
            }
        }

        match min_list {
            Some(idx) => {
                result.push(lists[idx][cursors[idx]].clone());
                cursors[idx] += 1;
            }
            None => break,
        }
    }

    result
}

fn append_to_thread(
    threads: &mut Vec<SerializedThreadTaskTimings>,
    thread_id: u64,
    thread_name: Option<String>,
    new_timings: Vec<SerializedTaskTiming>,
) {
    if let Some(existing) = threads.iter_mut().find(|t| t.thread_id == thread_id) {
        existing.timings.extend(new_timings);
        if existing.thread_name.is_none() {
            existing.thread_name = thread_name;
        }
    } else {
        threads.push(SerializedThreadTaskTimings {
            thread_name,
            thread_id,
            timings: new_timings,
        });
    }
}
