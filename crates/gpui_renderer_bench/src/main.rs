use std::{
    cell::RefCell,
    collections::BTreeMap,
    env,
    ffi::OsString,
    fs,
    path::{Path, PathBuf},
    rc::Rc,
    thread,
    time::{Duration, Instant},
};

use anyhow::{Context as _, Result, anyhow, bail};
use gpui::{
    App, Bounds, Context, Entity, Focusable as _, GpuSpecs, Render, SharedString, Window,
    WindowBounds, WindowOptions, div,
    prelude::*,
    profiler::{FrameEvent, FrameTimingCollector},
    px, rgb, size,
};
use serde::{Deserialize, Serialize};

#[cfg(any(target_os = "linux", target_os = "windows"))]
use smol::process::{Child, Command};
#[cfg(any(target_os = "linux", target_os = "windows"))]
use std::process::ExitStatus;

const DEFAULT_WIDTH: f32 = 1600.0;
const DEFAULT_HEIGHT: f32 = 900.0;
const DEFAULT_WARMUP_FRAMES: usize = 30;
const DEFAULT_MEASURE_FRAMES: usize = 180;
#[cfg(any(target_os = "linux", target_os = "windows"))]
const DEFAULT_CHILD_TIMEOUT: Duration = Duration::from_secs(240);
const STABILIZATION_FRAMES: usize = 10;

fn main() {
    if let Err(error) = run_cli() {
        eprintln!("gpui-renderer-bench: {error:#}");
        std::process::exit(1);
    }
}

fn run_cli() -> Result<()> {
    let mut arguments = env::args_os();
    let _executable = arguments.next();
    let Some(command) = arguments.next() else {
        print_usage();
        return Ok(());
    };
    let arguments = arguments.collect::<Vec<_>>();
    match command.to_string_lossy().as_ref() {
        "run" => run_application(parse_run_options(&arguments)?),
        "compare" => compare(parse_compare_options(&arguments)?),
        "summarize" => summarize(parse_summarize_options(&arguments)?),
        "help" | "--help" | "-h" => {
            print_usage();
            Ok(())
        }
        value => bail!("unknown command {value:?}"),
    }
}

fn print_usage() {
    println!(
        "\
Usage:
  gpui-renderer-bench run --output PATH [--width PX] [--height PX]
      [--warmup-frames N] [--measure-frames N] [--revision LABEL]
  gpui-renderer-bench compare --output-dir PATH [--rounds N]
      [--baseline-exe PATH] [--candidate-exe PATH]
      [--baseline-revision LABEL] [--candidate-revision LABEL]
      [--width PX] [--height PX] [--warmup-frames N] [--measure-frames N]
  gpui-renderer-bench summarize --input-dir PATH [--output-dir PATH]

On Windows, compare forces DirectX/WARP for a same-executable baseline. On Linux,
it forces WGPU and requires a software-emulated adapter on Wayland. If
--baseline-exe is supplied, that executable runs with GPUI_RENDERER unset so an
unmodified upstream build chooses its normal fallback. The candidate always runs
with GPUI_RENDERER=software."
    );
}

#[derive(Clone)]
struct RunOptions {
    output: PathBuf,
    width: f32,
    height: f32,
    warmup_frames: usize,
    measure_frames: usize,
    revision: Option<String>,
}

struct CompareOptions {
    output_dir: PathBuf,
    rounds: usize,
    baseline_executable: Option<PathBuf>,
    candidate_executable: Option<PathBuf>,
    baseline_revision: String,
    candidate_revision: String,
    run: RunOptions,
}

struct SummarizeOptions {
    input_dir: PathBuf,
    output_dir: PathBuf,
}

fn parse_run_options(arguments: &[OsString]) -> Result<RunOptions> {
    let values = parse_named_arguments(arguments)?;
    Ok(RunOptions {
        output: required_path(&values, "output")?,
        width: optional_number(&values, "width", DEFAULT_WIDTH)?,
        height: optional_number(&values, "height", DEFAULT_HEIGHT)?,
        warmup_frames: optional_number(&values, "warmup-frames", DEFAULT_WARMUP_FRAMES)?,
        measure_frames: optional_number(&values, "measure-frames", DEFAULT_MEASURE_FRAMES)?,
        revision: values
            .get("revision")
            .map(|value| value.to_string_lossy().into()),
    })
}

fn parse_compare_options(arguments: &[OsString]) -> Result<CompareOptions> {
    let values = parse_named_arguments(arguments)?;
    let output_dir = required_path(&values, "output-dir")?;
    Ok(CompareOptions {
        output_dir: output_dir.clone(),
        rounds: optional_number(&values, "rounds", 3)?,
        baseline_executable: values.get("baseline-exe").map(PathBuf::from),
        candidate_executable: values.get("candidate-exe").map(PathBuf::from),
        baseline_revision: values
            .get("baseline-revision")
            .map(|value| value.to_string_lossy().into())
            .unwrap_or_else(|| "upstream-or-legacy".to_owned()),
        candidate_revision: values
            .get("candidate-revision")
            .map(|value| value.to_string_lossy().into())
            .unwrap_or_else(|| "gpui-software".to_owned()),
        run: RunOptions {
            output: output_dir.join("unused.json"),
            width: optional_number(&values, "width", DEFAULT_WIDTH)?,
            height: optional_number(&values, "height", DEFAULT_HEIGHT)?,
            warmup_frames: optional_number(&values, "warmup-frames", DEFAULT_WARMUP_FRAMES)?,
            measure_frames: optional_number(&values, "measure-frames", DEFAULT_MEASURE_FRAMES)?,
            revision: None,
        },
    })
}

fn parse_summarize_options(arguments: &[OsString]) -> Result<SummarizeOptions> {
    let values = parse_named_arguments(arguments)?;
    let input_dir = required_path(&values, "input-dir")?;
    let output_dir = values
        .get("output-dir")
        .map(PathBuf::from)
        .unwrap_or_else(|| input_dir.clone());
    Ok(SummarizeOptions {
        input_dir,
        output_dir,
    })
}

fn parse_named_arguments(arguments: &[OsString]) -> Result<BTreeMap<String, OsString>> {
    let mut values = BTreeMap::new();
    let mut arguments = arguments.iter();
    while let Some(name) = arguments.next() {
        let name = name.to_string_lossy();
        let Some(name) = name.strip_prefix("--") else {
            bail!("expected a named argument, got {name:?}");
        };
        let value = arguments
            .next()
            .ok_or_else(|| anyhow!("missing value for --{name}"))?;
        if values.insert(name.to_owned(), value.clone()).is_some() {
            bail!("--{name} was specified more than once");
        }
    }
    Ok(values)
}

fn required_path(values: &BTreeMap<String, OsString>, name: &str) -> Result<PathBuf> {
    values
        .get(name)
        .map(PathBuf::from)
        .ok_or_else(|| anyhow!("missing required --{name} argument"))
}

fn optional_number<T>(values: &BTreeMap<String, OsString>, name: &str, default: T) -> Result<T>
where
    T: std::str::FromStr,
    T::Err: std::error::Error + Send + Sync + 'static,
{
    values.get(name).map_or(Ok(default), |value| {
        value
            .to_string_lossy()
            .parse()
            .with_context(|| format!("invalid value for --{name}"))
    })
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
enum Scenario {
    Caret,
    SingleLine,
    Scroll,
    FullFrame,
}

impl Scenario {
    const ALL: [Self; 4] = [Self::Caret, Self::SingleLine, Self::Scroll, Self::FullFrame];

    fn name(self) -> &'static str {
        match self {
            Self::Caret => "caret",
            Self::SingleLine => "single_line",
            Self::Scroll => "scroll",
            Self::FullFrame => "full_frame",
        }
    }
}

#[derive(Deserialize, Serialize)]
struct BenchmarkReport {
    schema_version: u32,
    label: String,
    revision: String,
    executable: PathBuf,
    operating_system: String,
    architecture: String,
    logical_cpu_count: usize,
    #[serde(default)]
    display_server: Option<String>,
    requested_renderer: Option<String>,
    requested_directx_adapter: Option<String>,
    gpu: Option<GpuReport>,
    window: WindowReport,
    warmup_frames: usize,
    measure_frames: usize,
    scenarios: Vec<ScenarioReport>,
}

#[derive(Deserialize, Serialize, PartialEq, Eq)]
struct GpuReport {
    is_software_emulated: bool,
    device_name: String,
    driver_name: String,
    driver_info: String,
}

impl From<GpuSpecs> for GpuReport {
    fn from(specifications: GpuSpecs) -> Self {
        Self {
            is_software_emulated: specifications.is_software_emulated,
            device_name: specifications.device_name,
            driver_name: specifications.driver_name,
            driver_info: specifications.driver_info,
        }
    }
}

#[derive(Deserialize, Serialize)]
struct WindowReport {
    requested_width: f32,
    requested_height: f32,
    viewport_width: f32,
    viewport_height: f32,
    active: bool,
}

#[derive(Deserialize, Serialize)]
struct ScenarioReport {
    scenario: Scenario,
    samples: FrameSamples,
    wall_time_ns: u64,
    process_cpu_time_ns: Option<u64>,
    process_user_time_ns: Option<u64>,
    process_kernel_time_ns: Option<u64>,
    process_cpu_cores: Option<f64>,
}

#[derive(Default, Deserialize, Serialize)]
struct FrameSamples {
    draw_ns: Vec<u64>,
    renderer_present_ns: Vec<u64>,
    dirty_to_present_ns: Vec<u64>,
    animation_interval_ns: Vec<u64>,
}

#[derive(Clone, Copy)]
struct ProcessCpuTime {
    user: Duration,
    kernel: Duration,
}

impl ProcessCpuTime {
    fn total(self) -> Duration {
        self.user.saturating_add(self.kernel)
    }

    fn saturating_sub(self, earlier: Self) -> Self {
        Self {
            user: self.user.saturating_sub(earlier.user),
            kernel: self.kernel.saturating_sub(earlier.kernel),
        }
    }
}

struct BenchmarkDriver {
    options: RunOptions,
    output_error: Rc<RefCell<Option<String>>>,
    collector: FrameTimingCollector,
    gpu: Option<GpuReport>,
    window: Option<WindowReport>,
    stabilization_completed: usize,
    scenario_index: usize,
    warmup_completed: usize,
    measure_completed: usize,
    pending: Option<PendingFrame>,
    scenario_started_at: Option<Instant>,
    scenario_started_cpu: Option<ProcessCpuTime>,
    samples: FrameSamples,
    scenario_reports: Vec<ScenarioReport>,
}

#[derive(Clone, Copy)]
struct PendingFrame {
    measured: bool,
}

enum DriverAction {
    AwaitActivation,
    Render { scenario: Scenario },
    Finish,
}

impl BenchmarkDriver {
    fn new(options: RunOptions, output_error: Rc<RefCell<Option<String>>>) -> Self {
        Self {
            options,
            output_error,
            collector: FrameTimingCollector::new(),
            gpu: None,
            window: None,
            stabilization_completed: 0,
            scenario_index: 0,
            warmup_completed: 0,
            measure_completed: 0,
            pending: None,
            scenario_started_at: None,
            scenario_started_cpu: None,
            samples: FrameSamples::default(),
            scenario_reports: Vec::new(),
        }
    }

    fn on_frame(&mut self, window: &Window) -> Result<DriverAction> {
        let events = self.collector.collect_unseen();
        if self.gpu.is_none() {
            self.gpu = window.gpu_specs().map(Into::into);
        }
        if self.window.is_none() {
            let viewport = window.viewport_size();
            self.window = Some(WindowReport {
                requested_width: self.options.width,
                requested_height: self.options.height,
                viewport_width: viewport.width.as_f32(),
                viewport_height: viewport.height.as_f32(),
                active: window.is_window_active(),
            });
        } else if let Some(window_report) = &mut self.window {
            window_report.active = window.is_window_active();
        }

        if !window.is_window_active() {
            self.pending = None;
            return Ok(DriverAction::AwaitActivation);
        }

        if let Some(pending) = self.pending.take() {
            let observation = observe_frame(events)?;
            if pending.measured {
                self.samples.draw_ns.push(observation.draw_ns);
                self.samples
                    .renderer_present_ns
                    .push(observation.renderer_present_ns);
                self.samples
                    .dirty_to_present_ns
                    .push(observation.dirty_to_present_ns);
                if let Some(interval) = observation.animation_interval_ns {
                    self.samples.animation_interval_ns.push(interval);
                }
                self.measure_completed += 1;
            } else if self.stabilization_completed < STABILIZATION_FRAMES {
                self.stabilization_completed += 1;
            } else {
                self.warmup_completed += 1;
            }
        }

        if self.stabilization_completed < STABILIZATION_FRAMES {
            return self.issue(Scenario::FullFrame, false);
        }

        loop {
            let Some(scenario) = Scenario::ALL.get(self.scenario_index).copied() else {
                self.write_report()?;
                return Ok(DriverAction::Finish);
            };
            if self.warmup_completed < self.options.warmup_frames {
                return self.issue(scenario, false);
            }
            if self.measure_completed < self.options.measure_frames {
                if self.measure_completed == 0 && self.scenario_started_at.is_none() {
                    self.collector.collect_unseen();
                    self.scenario_started_at = Some(Instant::now());
                    self.scenario_started_cpu = process_cpu_time();
                }
                return self.issue(scenario, true);
            }
            self.finish_scenario(scenario)?;
            self.scenario_index += 1;
            self.warmup_completed = 0;
            self.measure_completed = 0;
            self.samples = FrameSamples::default();
            self.scenario_started_at = None;
            self.scenario_started_cpu = None;
        }
    }

    fn issue(&mut self, scenario: Scenario, measured: bool) -> Result<DriverAction> {
        if self.pending.is_some() {
            bail!("attempted to issue a benchmark frame while another was pending");
        }
        self.pending = Some(PendingFrame { measured });
        Ok(DriverAction::Render { scenario })
    }

    fn finish_scenario(&mut self, scenario: Scenario) -> Result<()> {
        if self.samples.draw_ns.len() != self.options.measure_frames
            || self.samples.renderer_present_ns.len() != self.options.measure_frames
            || self.samples.dirty_to_present_ns.len() != self.options.measure_frames
        {
            bail!(
                "scenario {} collected incomplete samples: draw={}, present={}, dirty-to-present={}, expected={}",
                scenario.name(),
                self.samples.draw_ns.len(),
                self.samples.renderer_present_ns.len(),
                self.samples.dirty_to_present_ns.len(),
                self.options.measure_frames
            );
        }
        let started_at = self
            .scenario_started_at
            .ok_or_else(|| anyhow!("scenario {} did not record a start time", scenario.name()))?;
        let wall_time = started_at.elapsed();
        let cpu_time = self
            .scenario_started_cpu
            .zip(process_cpu_time())
            .map(|(before, after)| after.saturating_sub(before));
        let cpu_total = cpu_time.map(ProcessCpuTime::total);
        self.scenario_reports.push(ScenarioReport {
            scenario,
            samples: std::mem::take(&mut self.samples),
            wall_time_ns: duration_ns(wall_time),
            process_cpu_time_ns: cpu_total.map(duration_ns),
            process_user_time_ns: cpu_time.map(|time| duration_ns(time.user)),
            process_kernel_time_ns: cpu_time.map(|time| duration_ns(time.kernel)),
            process_cpu_cores: cpu_total.map(|time| {
                if wall_time.is_zero() {
                    0.0
                } else {
                    time.as_secs_f64() / wall_time.as_secs_f64()
                }
            }),
        });
        Ok(())
    }

    fn write_report(&self) -> Result<()> {
        let executable = env::current_exe().context("resolving benchmark executable")?;
        let revision = self
            .options
            .revision
            .clone()
            .or_else(|| env::var("GPUI_BENCHMARK_REVISION").ok())
            .unwrap_or_else(|| "unknown".to_owned());
        let report = BenchmarkReport {
            schema_version: 3,
            label: env::var("GPUI_BENCHMARK_LABEL").unwrap_or_else(|_| "unlabeled".to_owned()),
            revision,
            executable,
            operating_system: env::consts::OS.to_owned(),
            architecture: env::consts::ARCH.to_owned(),
            logical_cpu_count: thread::available_parallelism().map_or(1, usize::from),
            display_server: display_server(),
            requested_renderer: env::var("GPUI_RENDERER").ok(),
            requested_directx_adapter: env::var("GPUI_D3D_ADAPTER").ok(),
            gpu: self.gpu.as_ref().map(|gpu| GpuReport {
                is_software_emulated: gpu.is_software_emulated,
                device_name: gpu.device_name.clone(),
                driver_name: gpu.driver_name.clone(),
                driver_info: gpu.driver_info.clone(),
            }),
            window: WindowReport {
                requested_width: self.options.width,
                requested_height: self.options.height,
                viewport_width: self
                    .window
                    .as_ref()
                    .map_or(0.0, |window| window.viewport_width),
                viewport_height: self
                    .window
                    .as_ref()
                    .map_or(0.0, |window| window.viewport_height),
                active: self.window.as_ref().is_some_and(|window| window.active),
            },
            warmup_frames: self.options.warmup_frames,
            measure_frames: self.options.measure_frames,
            scenarios: self
                .scenario_reports
                .iter()
                .map(|scenario| ScenarioReport {
                    scenario: scenario.scenario,
                    samples: FrameSamples {
                        draw_ns: scenario.samples.draw_ns.clone(),
                        renderer_present_ns: scenario.samples.renderer_present_ns.clone(),
                        dirty_to_present_ns: scenario.samples.dirty_to_present_ns.clone(),
                        animation_interval_ns: scenario.samples.animation_interval_ns.clone(),
                    },
                    wall_time_ns: scenario.wall_time_ns,
                    process_cpu_time_ns: scenario.process_cpu_time_ns,
                    process_user_time_ns: scenario.process_user_time_ns,
                    process_kernel_time_ns: scenario.process_kernel_time_ns,
                    process_cpu_cores: scenario.process_cpu_cores,
                })
                .collect(),
        };
        if let Some(parent) = self.options.output.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("creating benchmark output directory {parent:?}"))?;
        }
        let file = fs::File::create(&self.options.output)
            .with_context(|| format!("creating benchmark report {:?}", self.options.output))?;
        serde_json::to_writer_pretty(file, &report).context("serializing benchmark report")?;
        println!("wrote {}", self.options.output.display());
        Ok(())
    }
}

struct FrameObservation {
    draw_ns: u64,
    renderer_present_ns: u64,
    dirty_to_present_ns: u64,
    animation_interval_ns: Option<u64>,
}

fn observe_frame(events: Vec<FrameEvent>) -> Result<FrameObservation> {
    let mut draw = None;
    let mut present = None;
    for event in events {
        match event {
            FrameEvent::Draw(timing) => {
                if draw.replace(timing).is_some() {
                    bail!("a benchmark step produced more than one draw event");
                }
            }
            FrameEvent::Present(timing) => {
                if present.replace(timing).is_some() {
                    bail!("a benchmark step produced more than one present event");
                }
            }
        }
    }
    let draw = draw.ok_or_else(|| anyhow!("a benchmark step produced no draw event"))?;
    let present = present.ok_or_else(|| anyhow!("a benchmark step produced no present event"))?;
    if draw.window_id != present.window_id {
        bail!("draw and present events came from different windows");
    }
    let dirty_at = draw
        .dirty_at
        .ok_or_else(|| anyhow!("benchmark frame had no invalidation timestamp"))?;
    Ok(FrameObservation {
        draw_ns: duration_ns(draw.draw_duration()),
        renderer_present_ns: duration_ns(present.present_duration()),
        dirty_to_present_ns: duration_ns(present.present_end.duration_since(dirty_at)),
        animation_interval_ns: present.animation_interval.map(duration_ns),
    })
}

fn duration_ns(duration: Duration) -> u64 {
    duration.as_nanos().min(u64::MAX as u128) as u64
}

fn run_application(options: RunOptions) -> Result<()> {
    if options.width <= 0.0 || options.height <= 0.0 {
        bail!("window dimensions must be positive");
    }
    if options.measure_frames == 0 {
        bail!("--measure-frames must be greater than zero");
    }
    let output_error = Rc::new(RefCell::new(None));
    let callback_error = output_error.clone();
    gpui::profiler::set_trace_enabled(true);
    gpui_platform::application().run(move |cx: &mut App| {
        if let Err(error) = start_benchmark(options, callback_error.clone(), cx) {
            callback_error.replace(Some(format!("{error:#}")));
            cx.quit();
        }
    });
    gpui::profiler::set_trace_enabled(false);
    if let Some(error) = output_error.take() {
        bail!(error);
    }
    Ok(())
}

fn start_benchmark(
    options: RunOptions,
    output_error: Rc<RefCell<Option<String>>>,
    cx: &mut App,
) -> Result<()> {
    let bounds = Bounds::centered(None, size(px(options.width), px(options.height)), cx);
    let window_handle = cx.open_window(
        WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            ..Default::default()
        },
        |window, cx| {
            let view = cx.new(EditorBenchmarkView::new);
            window.focus(&view.focus_handle(cx), cx);
            view
        },
    )?;
    cx.activate(true);
    let view = window_handle.entity(cx)?;
    let driver = Rc::new(RefCell::new(BenchmarkDriver::new(options, output_error)));
    window_handle.update(cx, |_, window, _| {
        schedule_benchmark_frame(window, view, driver);
    })?;
    Ok(())
}

fn schedule_benchmark_frame(
    window: &Window,
    view: Entity<EditorBenchmarkView>,
    driver: Rc<RefCell<BenchmarkDriver>>,
) {
    window.on_next_frame(move |window, cx| {
        let action = driver.borrow_mut().on_frame(window);
        match action {
            Ok(DriverAction::AwaitActivation) => {
                schedule_benchmark_frame(window, view, driver);
            }
            Ok(DriverAction::Render { scenario }) => {
                view.update(cx, |view, cx| {
                    view.advance(scenario);
                    cx.notify();
                });
                schedule_benchmark_frame(window, view, driver);
            }
            Ok(DriverAction::Finish) => {
                gpui::profiler::set_trace_enabled(false);
                if !env::var("GPUI_BENCHMARK_HOLD_AFTER_REPORT").is_ok_and(|value| value == "1") {
                    cx.quit();
                }
            }
            Err(error) => {
                let output_error = driver.borrow().output_error.clone();
                output_error.replace(Some(format!("{error:#}")));
                gpui::profiler::set_trace_enabled(false);
                cx.quit();
            }
        }
    });
}

struct BenchmarkLine {
    line_number: SharedString,
    indentation: SharedString,
    keyword: SharedString,
    body: SharedString,
    comment: SharedString,
    alternate_body: SharedString,
}

struct EditorBenchmarkView {
    focus_handle: gpui::FocusHandle,
    scenario: Scenario,
    generation: usize,
    lines: Vec<BenchmarkLine>,
    files: Vec<SharedString>,
}

impl EditorBenchmarkView {
    fn new(cx: &mut Context<Self>) -> Self {
        let lines = (0..160)
            .map(|index| BenchmarkLine {
                line_number: format!("{:>4}", index + 1).into(),
                indentation: "    ".into(),
                keyword: if index % 4 == 0 { "pub fn ".into() } else { "let ".into() },
                body: format!(
                    "render_editor_row_{index:03}(workspace, project, buffer, selection, diagnostics);"
                )
                .into(),
                comment: format!(" // deterministic benchmark payload {index:03}").into(),
                alternate_body: format!(
                    "render_changed_row_{index:03}(workspace, project, buffer, selection, diagnostics);"
                )
                .into(),
            })
            .collect();
        let files = (0..32)
            .map(|index| format!("renderer_component_{index:02}.rs").into())
            .collect();
        Self {
            focus_handle: cx.focus_handle(),
            scenario: Scenario::FullFrame,
            generation: 0,
            lines,
            files,
        }
    }

    fn advance(&mut self, scenario: Scenario) {
        self.scenario = scenario;
        self.generation = self.generation.wrapping_add(1);
    }
}

impl gpui::Focusable for EditorBenchmarkView {
    fn focus_handle(&self, _cx: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for EditorBenchmarkView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let alternate = self.generation % 2 == 1;
        let full_frame_alternate = self.scenario == Scenario::FullFrame && alternate;
        let scroll_offset = if self.scenario == Scenario::Scroll && alternate {
            1
        } else {
            0
        };
        let background = if full_frame_alternate {
            rgb(0x151b26)
        } else {
            rgb(0x10151e)
        };
        let panel_background = if full_frame_alternate {
            rgb(0x202a38)
        } else {
            rgb(0x1a2230)
        };

        let mut sidebar = div()
            .flex()
            .flex_col()
            .w(px(238.0))
            .h_full()
            .bg(panel_background)
            .border_r_1()
            .border_color(rgb(0x334155))
            .px_3()
            .py_2()
            .text_size(px(12.0))
            .line_height(px(19.0))
            .text_color(rgb(0x94a3b8))
            .child(
                div()
                    .h(px(28.0))
                    .font_weight(gpui::FontWeight::BOLD)
                    .text_color(rgb(0xe2e8f0))
                    .child("GPUI SOFTWARE"),
            );
        for (index, file) in self.files.iter().enumerate() {
            sidebar = sidebar.child(
                div()
                    .flex()
                    .h(px(19.0))
                    .items_center()
                    .pl(px(((index % 3) * 8) as f32))
                    .when(index == 7, |element| element.bg(rgb(0x263244)))
                    .child(file.clone()),
            );
        }

        let mut editor_rows = div()
            .flex()
            .flex_col()
            .flex_grow(1.0)
            .overflow_hidden()
            .py_2()
            .text_size(px(13.0))
            .line_height(px(17.0));
        for visible_row in 0..50 {
            let line_index = visible_row + 34 + scroll_offset;
            let line = &self.lines[line_index];
            let changed_line = visible_row == 21;
            let body = if self.scenario == Scenario::SingleLine && changed_line && alternate {
                line.alternate_body.clone()
            } else {
                line.body.clone()
            };
            let row_background = if changed_line {
                rgb(0x182131)
            } else {
                background
            };
            let caret = self.scenario == Scenario::Caret && changed_line && alternate;
            editor_rows = editor_rows.child(
                div()
                    .flex()
                    .h(px(17.0))
                    .flex_none()
                    .items_center()
                    .bg(row_background)
                    .child(
                        div()
                            .w(px(54.0))
                            .pr_3()
                            .text_right()
                            .text_color(rgb(0x526077))
                            .child(line.line_number.clone()),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_grow(1.0)
                            .overflow_hidden()
                            .text_color(rgb(0xcbd5e1))
                            .child(line.indentation.clone())
                            .child(div().text_color(rgb(0xc084fc)).child(line.keyword.clone()))
                            .child(body)
                            .child(div().text_color(rgb(0x64748b)).child(line.comment.clone()))
                            .when(caret, |element| {
                                element.child(
                                    div().ml(px(2.0)).w(px(2.0)).h(px(14.0)).bg(rgb(0x7dd3fc)),
                                )
                            }),
                    ),
            );
        }

        let editor = div()
            .flex()
            .flex_col()
            .flex_grow(1.0)
            .h_full()
            .bg(background)
            .child(
                div()
                    .flex()
                    .h(px(36.0))
                    .flex_none()
                    .items_center()
                    .bg(panel_background)
                    .border_b_1()
                    .border_color(rgb(0x334155))
                    .px_3()
                    .text_size(px(12.0))
                    .text_color(rgb(0xcbd5e1))
                    .child("gpui_renderer_bench.rs")
                    .child(
                        div()
                            .ml_4()
                            .text_color(rgb(0x64748b))
                            .child(self.scenario.name()),
                    ),
            )
            .child(editor_rows)
            .child(
                div()
                    .flex()
                    .h(px(24.0))
                    .flex_none()
                    .items_center()
                    .justify_between()
                    .bg(panel_background)
                    .border_t_1()
                    .border_color(rgb(0x334155))
                    .px_3()
                    .text_size(px(11.0))
                    .text_color(rgb(0x94a3b8))
                    .child("main  Rust Analyzer: ready")
                    .child("Ln 56, Col 41   UTF-8   CRLF"),
            );

        div()
            .track_focus(&self.focus_handle)
            .flex()
            .size_full()
            .overflow_hidden()
            .bg(background)
            .child(sidebar)
            .child(editor)
    }
}

fn compare(options: CompareOptions) -> Result<()> {
    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    {
        let CompareOptions {
            output_dir,
            rounds,
            baseline_executable,
            candidate_executable,
            baseline_revision,
            candidate_revision,
            run,
        } = options;
        let _ = (
            output_dir,
            rounds,
            baseline_executable,
            candidate_executable,
            baseline_revision,
            candidate_revision,
            run,
        );
        bail!(
            "the no-GPU legacy-versus-gpui_software comparison is only valid on Windows or Linux"
        );
    }
    #[cfg(any(target_os = "linux", target_os = "windows"))]
    compare_supported_platform(options)
}

#[cfg(any(target_os = "linux", target_os = "windows"))]
fn compare_supported_platform(options: CompareOptions) -> Result<()> {
    if options.rounds == 0 {
        bail!("--rounds must be greater than zero");
    }
    fs::create_dir_all(&options.output_dir).with_context(|| {
        format!(
            "creating benchmark output directory {:?}",
            options.output_dir
        )
    })?;
    let current_executable = env::current_exe().context("resolving current executable")?;
    let baseline_is_current = options.baseline_executable.is_none();
    let baseline_executable = options
        .baseline_executable
        .clone()
        .unwrap_or_else(|| current_executable.clone());
    let candidate_executable = options
        .candidate_executable
        .clone()
        .unwrap_or(current_executable);

    for round in 0..options.rounds {
        let order = if round % 2 == 0 {
            ["baseline", "candidate"]
        } else {
            ["candidate", "baseline"]
        };
        for label in order {
            let output = options
                .output_dir
                .join(format!("{label}-round-{:02}.json", round + 1));
            let (executable, renderer, directx_adapter, revision) = if label == "baseline" {
                (
                    &baseline_executable,
                    baseline_is_current.then_some(baseline_renderer()),
                    baseline_directx_adapter(),
                    options.baseline_revision.as_str(),
                )
            } else {
                (
                    &candidate_executable,
                    Some("software"),
                    None,
                    options.candidate_revision.as_str(),
                )
            };
            run_child(
                executable,
                renderer,
                directx_adapter,
                label,
                revision,
                &output,
                &options.run,
            )?;
            let report = read_report(&output)?;
            validate_comparison_report(label, &report)?;
        }
    }
    summarize(SummarizeOptions {
        input_dir: options.output_dir.clone(),
        output_dir: options.output_dir,
    })
}

#[cfg(any(target_os = "linux", target_os = "windows"))]
fn run_child(
    executable: &Path,
    renderer: Option<&str>,
    directx_adapter: Option<&str>,
    label: &str,
    revision: &str,
    output: &Path,
    options: &RunOptions,
) -> Result<()> {
    let mut command = Command::new(executable);
    command
        .arg("run")
        .arg("--output")
        .arg(output)
        .arg("--width")
        .arg(options.width.to_string())
        .arg("--height")
        .arg(options.height.to_string())
        .arg("--warmup-frames")
        .arg(options.warmup_frames.to_string())
        .arg("--measure-frames")
        .arg(options.measure_frames.to_string())
        .arg("--revision")
        .arg(revision)
        .env("GPUI_BENCHMARK_LABEL", label);
    if let Some(renderer) = renderer {
        command.env("GPUI_RENDERER", renderer);
    } else {
        command.env_remove("GPUI_RENDERER");
    }
    if let Some(directx_adapter) = directx_adapter {
        command.env("GPUI_D3D_ADAPTER", directx_adapter);
    } else {
        command.env_remove("GPUI_D3D_ADAPTER");
    }
    let child = command
        .spawn()
        .with_context(|| format!("launching benchmark executable {executable:?}"))?;
    let status = wait_for_child(child, DEFAULT_CHILD_TIMEOUT)?;
    if !status.success() {
        bail!("{label} benchmark exited with {status}");
    }
    Ok(())
}

#[cfg(any(target_os = "linux", target_os = "windows"))]
fn wait_for_child(mut child: Child, timeout: Duration) -> Result<ExitStatus> {
    let started_at = Instant::now();
    loop {
        if let Some(status) = child.try_status().context("checking benchmark child")? {
            return Ok(status);
        }
        if started_at.elapsed() >= timeout {
            child
                .kill()
                .context("terminating timed-out benchmark child")?;
            smol::block_on(child.status()).context("reaping timed-out benchmark child")?;
            bail!(
                "benchmark child exceeded the {} second timeout",
                timeout.as_secs()
            );
        }
        thread::sleep(Duration::from_millis(100));
    }
}

#[cfg(any(target_os = "linux", target_os = "windows"))]
fn validate_comparison_report(label: &str, report: &BenchmarkReport) -> Result<()> {
    let gpu = report
        .gpu
        .as_ref()
        .ok_or_else(|| anyhow!("{label} report did not include renderer information"))?;
    #[cfg(target_os = "linux")]
    if report.display_server.as_deref() != Some("wayland") {
        bail!(
            "{label} did not run on Wayland; display server was {:?}",
            report.display_server
        );
    }
    match label {
        #[cfg(target_os = "windows")]
        "baseline" if report.requested_directx_adapter.as_deref() != Some("warp") => {
            bail!("baseline did not request the WARP adapter")
        }
        "baseline" if !gpu.is_software_emulated => bail!(
            "baseline used hardware renderer {:?}; this is not a valid no-GPU comparison",
            gpu.device_name
        ),
        "candidate" if gpu.driver_name != "gpui_software" => bail!(
            "candidate did not use gpui_software; it reported {:?} / {:?}",
            gpu.device_name,
            gpu.driver_name
        ),
        _ => Ok(()),
    }
}

#[cfg(target_os = "windows")]
fn baseline_renderer() -> &'static str {
    "directx"
}

#[cfg(target_os = "linux")]
fn baseline_renderer() -> &'static str {
    "wgpu"
}

#[cfg(target_os = "windows")]
fn baseline_directx_adapter() -> Option<&'static str> {
    Some("warp")
}

#[cfg(target_os = "linux")]
fn baseline_directx_adapter() -> Option<&'static str> {
    None
}

fn read_report(path: &Path) -> Result<BenchmarkReport> {
    let bytes = fs::read(path).with_context(|| format!("reading report {path:?}"))?;
    serde_json::from_slice(&bytes).with_context(|| format!("parsing report {path:?}"))
}

#[derive(Serialize)]
struct ComparisonSummary {
    schema_version: u32,
    reports: usize,
    runs: Vec<RunSummary>,
    groups: Vec<SummaryGroup>,
    comparisons: Vec<ComparisonDelta>,
}

#[derive(Serialize)]
struct RunSummary {
    label: String,
    revision: String,
    report_count: usize,
    operating_system: String,
    architecture: String,
    logical_cpu_count: usize,
    display_server: Option<String>,
    requested_renderer: Option<String>,
    requested_directx_adapter: Option<String>,
    gpu: Option<GpuReport>,
    viewport_width: f32,
    viewport_height: f32,
    warmup_frames: usize,
    measure_frames: usize,
}

#[derive(Serialize)]
struct SummaryGroup {
    label: String,
    scenario: Scenario,
    report_count: usize,
    sample_count: usize,
    draw_ms: Distribution,
    renderer_present_ms: Distribution,
    dirty_to_present_ms: Distribution,
    animation_interval_ms: Option<Distribution>,
    mean_wall_ms_per_frame: f64,
    mean_process_cpu_ms_per_frame: Option<f64>,
    mean_process_cpu_cores: Option<f64>,
}

#[derive(Serialize)]
struct Distribution {
    minimum: f64,
    p50: f64,
    p90: f64,
    p95: f64,
    p99: f64,
    maximum: f64,
    mean: f64,
}

#[derive(Serialize)]
struct ComparisonDelta {
    scenario: Scenario,
    metric: &'static str,
    baseline: f64,
    candidate: f64,
    candidate_change_percent: f64,
    baseline_over_candidate: f64,
}

fn summarize(options: SummarizeOptions) -> Result<()> {
    let mut reports = Vec::new();
    for entry in fs::read_dir(&options.input_dir)
        .with_context(|| format!("reading benchmark directory {:?}", options.input_dir))?
    {
        let path = entry?.path();
        let file_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("");
        if path.extension().and_then(|extension| extension.to_str()) == Some("json")
            && file_name != "summary.json"
        {
            reports.push(read_report(&path)?);
        }
    }
    if reports.is_empty() {
        bail!("no benchmark JSON reports found in {:?}", options.input_dir);
    }
    let summary = build_summary(&reports)?;
    fs::create_dir_all(&options.output_dir)
        .with_context(|| format!("creating summary directory {:?}", options.output_dir))?;
    let json_path = options.output_dir.join("summary.json");
    let json_file =
        fs::File::create(&json_path).with_context(|| format!("creating summary {json_path:?}"))?;
    serde_json::to_writer_pretty(json_file, &summary).context("serializing summary JSON")?;
    let markdown_path = options.output_dir.join("summary.md");
    fs::write(&markdown_path, summary_markdown(&summary))
        .with_context(|| format!("writing summary {markdown_path:?}"))?;
    println!("wrote {}", json_path.display());
    println!("wrote {}", markdown_path.display());
    Ok(())
}

fn build_summary(reports: &[BenchmarkReport]) -> Result<ComparisonSummary> {
    let runs = build_run_summaries(reports)?;
    let mut grouped: BTreeMap<(String, Scenario), Vec<&ScenarioReport>> = BTreeMap::new();
    for report in reports {
        for scenario in &report.scenarios {
            grouped
                .entry((report.label.clone(), scenario.scenario))
                .or_default()
                .push(scenario);
        }
    }
    let mut groups = Vec::new();
    for ((label, scenario), scenario_reports) in grouped {
        let draw = concatenate_samples(&scenario_reports, |samples| &samples.draw_ns);
        let renderer_present =
            concatenate_samples(&scenario_reports, |samples| &samples.renderer_present_ns);
        let dirty_to_present =
            concatenate_samples(&scenario_reports, |samples| &samples.dirty_to_present_ns);
        let animation_interval =
            concatenate_samples(&scenario_reports, |samples| &samples.animation_interval_ns);
        let total_frames = scenario_reports
            .iter()
            .map(|report| report.samples.draw_ns.len())
            .sum::<usize>();
        if total_frames == 0 {
            bail!("{label}/{} contains no frame samples", scenario.name());
        }
        let wall_time_ns = scenario_reports
            .iter()
            .map(|report| report.wall_time_ns as u128)
            .sum::<u128>();
        let cpu_reports = scenario_reports
            .iter()
            .filter_map(|report| {
                report
                    .process_cpu_time_ns
                    .map(|cpu_time| (cpu_time, report.samples.draw_ns.len()))
            })
            .collect::<Vec<_>>();
        let cpu_cores = scenario_reports
            .iter()
            .filter_map(|report| report.process_cpu_cores)
            .collect::<Vec<_>>();
        groups.push(SummaryGroup {
            label,
            scenario,
            report_count: scenario_reports.len(),
            sample_count: total_frames,
            draw_ms: distribution_ms(&draw)?,
            renderer_present_ms: distribution_ms(&renderer_present)?,
            dirty_to_present_ms: distribution_ms(&dirty_to_present)?,
            animation_interval_ms: (!animation_interval.is_empty())
                .then(|| distribution_ms(&animation_interval))
                .transpose()?,
            mean_wall_ms_per_frame: wall_time_ns as f64 / total_frames as f64 / 1_000_000.0,
            mean_process_cpu_ms_per_frame: (!cpu_reports.is_empty()).then(|| {
                let cpu_time = cpu_reports
                    .iter()
                    .map(|(cpu_time, _)| *cpu_time as f64)
                    .sum::<f64>();
                let cpu_frames = cpu_reports
                    .iter()
                    .map(|(_, frame_count)| *frame_count)
                    .sum::<usize>();
                cpu_time / cpu_frames as f64 / 1_000_000.0
            }),
            mean_process_cpu_cores: (!cpu_cores.is_empty())
                .then(|| cpu_cores.iter().sum::<f64>() / cpu_cores.len() as f64),
        });
    }
    let comparisons = build_comparisons(&groups);
    Ok(ComparisonSummary {
        schema_version: 3,
        reports: reports.len(),
        runs,
        groups,
        comparisons,
    })
}

fn build_run_summaries(reports: &[BenchmarkReport]) -> Result<Vec<RunSummary>> {
    let first_report = reports
        .first()
        .ok_or_else(|| anyhow!("cannot summarize an empty report set"))?;
    let mut by_label: BTreeMap<&str, Vec<&BenchmarkReport>> = BTreeMap::new();
    for report in reports {
        if report.operating_system != first_report.operating_system
            || report.architecture != first_report.architecture
            || report.logical_cpu_count != first_report.logical_cpu_count
            || report.display_server != first_report.display_server
            || report.window.requested_width != first_report.window.requested_width
            || report.window.requested_height != first_report.window.requested_height
            || report.warmup_frames != first_report.warmup_frames
            || report.measure_frames != first_report.measure_frames
        {
            bail!("benchmark reports contain incompatible host or workload configurations");
        }
        if !report.window.active {
            bail!("{} benchmark window was not active", report.label);
        }
        by_label.entry(&report.label).or_default().push(report);
    }

    by_label
        .into_iter()
        .map(|(label, reports)| {
            let first = reports
                .first()
                .ok_or_else(|| anyhow!("{label} contains no benchmark reports"))?;
            if reports.iter().any(|report| {
                report.revision != first.revision
                    || report.requested_renderer != first.requested_renderer
                    || report.requested_directx_adapter != first.requested_directx_adapter
                    || report.gpu != first.gpu
                    || report.window.viewport_width != first.window.viewport_width
                    || report.window.viewport_height != first.window.viewport_height
            }) {
                bail!("{label} reports contain incompatible renderer configurations");
            }
            Ok(RunSummary {
                label: label.to_owned(),
                revision: first.revision.clone(),
                report_count: reports.len(),
                operating_system: first.operating_system.clone(),
                architecture: first.architecture.clone(),
                logical_cpu_count: first.logical_cpu_count,
                display_server: first.display_server.clone(),
                requested_renderer: first.requested_renderer.clone(),
                requested_directx_adapter: first.requested_directx_adapter.clone(),
                gpu: first.gpu.as_ref().map(|gpu| GpuReport {
                    is_software_emulated: gpu.is_software_emulated,
                    device_name: gpu.device_name.clone(),
                    driver_name: gpu.driver_name.clone(),
                    driver_info: gpu.driver_info.clone(),
                }),
                viewport_width: first.window.viewport_width,
                viewport_height: first.window.viewport_height,
                warmup_frames: first.warmup_frames,
                measure_frames: first.measure_frames,
            })
        })
        .collect()
}

fn build_comparisons(groups: &[SummaryGroup]) -> Vec<ComparisonDelta> {
    let mut comparisons = Vec::new();
    for scenario in Scenario::ALL {
        let baseline = groups
            .iter()
            .find(|group| group.label == "baseline" && group.scenario == scenario);
        let candidate = groups
            .iter()
            .find(|group| group.label == "candidate" && group.scenario == scenario);
        let (Some(baseline), Some(candidate)) = (baseline, candidate) else {
            continue;
        };
        add_comparison(
            &mut comparisons,
            scenario,
            "render/submit p50 (ms)",
            baseline.renderer_present_ms.p50,
            candidate.renderer_present_ms.p50,
        );
        add_comparison(
            &mut comparisons,
            scenario,
            "render/submit p95 (ms)",
            baseline.renderer_present_ms.p95,
            candidate.renderer_present_ms.p95,
        );
        add_comparison(
            &mut comparisons,
            scenario,
            "dirty-to-submit p95 (ms)",
            baseline.dirty_to_present_ms.p95,
            candidate.dirty_to_present_ms.p95,
        );
        if let (Some(baseline), Some(candidate)) = (
            baseline.mean_process_cpu_ms_per_frame,
            candidate.mean_process_cpu_ms_per_frame,
        ) {
            add_comparison(
                &mut comparisons,
                scenario,
                "process CPU (ms/frame)",
                baseline,
                candidate,
            );
        }
        if let (Some(baseline), Some(candidate)) = (
            baseline
                .animation_interval_ms
                .as_ref()
                .map(|distribution| distribution.p95),
            candidate
                .animation_interval_ms
                .as_ref()
                .map(|distribution| distribution.p95),
        ) {
            add_comparison(
                &mut comparisons,
                scenario,
                "frame interval p95 (ms)",
                baseline,
                candidate,
            );
        }
    }
    comparisons
}

fn add_comparison(
    comparisons: &mut Vec<ComparisonDelta>,
    scenario: Scenario,
    metric: &'static str,
    baseline: f64,
    candidate: f64,
) {
    if baseline <= 0.0 || candidate <= 0.0 {
        return;
    }
    comparisons.push(ComparisonDelta {
        scenario,
        metric,
        baseline,
        candidate,
        candidate_change_percent: (candidate / baseline - 1.0) * 100.0,
        baseline_over_candidate: baseline / candidate,
    });
}

fn concatenate_samples<'a>(
    reports: &'a [&ScenarioReport],
    select: impl Fn(&'a FrameSamples) -> &'a [u64],
) -> Vec<u64> {
    reports
        .iter()
        .flat_map(|report| select(&report.samples).iter().copied())
        .collect()
}

fn distribution_ms(values: &[u64]) -> Result<Distribution> {
    if values.is_empty() {
        bail!("cannot summarize an empty timing distribution");
    }
    let mut values = values.to_vec();
    values.sort_unstable();
    let to_ms = |value: u64| value as f64 / 1_000_000.0;
    Ok(Distribution {
        minimum: to_ms(values[0]),
        p50: to_ms(percentile(&values, 50)),
        p90: to_ms(percentile(&values, 90)),
        p95: to_ms(percentile(&values, 95)),
        p99: to_ms(percentile(&values, 99)),
        maximum: to_ms(*values.last().expect("distribution is non-empty")),
        mean: values.iter().map(|value| *value as f64).sum::<f64>()
            / values.len() as f64
            / 1_000_000.0,
    })
}

fn percentile(values: &[u64], percentile: usize) -> u64 {
    let index = (values.len() * percentile).div_ceil(100).saturating_sub(1);
    values[index]
}

fn summary_markdown(summary: &ComparisonSummary) -> String {
    let mut markdown = String::from(
        "# GPUI renderer benchmark\n\n## Run configuration\n\n| Renderer | Revision | OS | Display server | Architecture | Logical CPUs | Requested path | Device | Driver | Viewport | Reports |\n|---|---|---|---|---|---:|---|---|---|---|---:|\n",
    );
    for run in &summary.runs {
        let requested_path = match (
            run.requested_renderer.as_deref(),
            run.requested_directx_adapter.as_deref(),
        ) {
            (Some(renderer), Some(adapter)) => format!("{renderer} / {adapter}"),
            (Some(renderer), None) => renderer.to_owned(),
            (None, Some(adapter)) => format!("automatic / {adapter}"),
            (None, None) => "automatic".to_owned(),
        };
        let (device, driver) = run.gpu.as_ref().map_or_else(
            || ("n/a".to_owned(), "n/a".to_owned()),
            |gpu| (gpu.device_name.clone(), gpu.driver_name.clone()),
        );
        markdown.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {:.0}x{:.0} | {} |\n",
            run.label,
            run.revision,
            run.operating_system,
            run.display_server.as_deref().unwrap_or("n/a"),
            run.architecture,
            run.logical_cpu_count,
            requested_path,
            device,
            driver,
            run.viewport_width,
            run.viewport_height,
            run.report_count,
        ));
    }
    markdown.push_str(
        "\n## Results\n\nAll values are aggregated from raw per-frame samples. CPU cores is total process CPU time divided by wall time; 1.0 means one fully utilized logical core. Render/submit is time inside the platform call, not necessarily asynchronous DirectX completion time.\n\n| Renderer | Scenario | Frames | Render/submit p50 (ms) | Render/submit p95 (ms) | Dirty-to-submit p95 (ms) | Frame interval p95 (ms) | CPU ms/frame | CPU cores |\n|---|---|---:|---:|---:|---:|---:|---:|---:|\n",
    );
    for group in &summary.groups {
        markdown.push_str(&format!(
            "| {} | {} | {} | {:.3} | {:.3} | {:.3} | {} | {} | {} |\n",
            group.label,
            group.scenario.name(),
            group.sample_count,
            group.renderer_present_ms.p50,
            group.renderer_present_ms.p95,
            group.dirty_to_present_ms.p95,
            format_optional(
                group
                    .animation_interval_ms
                    .as_ref()
                    .map(|distribution| distribution.p95),
            ),
            format_optional(group.mean_process_cpu_ms_per_frame),
            format_optional(group.mean_process_cpu_cores),
        ));
    }
    if !summary.comparisons.is_empty() {
        markdown.push_str(
            "\nLower is better for every metric below. A negative change means the candidate used less time or CPU.\n\n| Scenario | Metric | Baseline | Candidate | Candidate change | Baseline / candidate |\n|---|---|---:|---:|---:|---:|\n",
        );
        for comparison in &summary.comparisons {
            markdown.push_str(&format!(
                "| {} | {} | {:.3} | {:.3} | {:+.1}% | {:.2}x |\n",
                comparison.scenario.name(),
                comparison.metric,
                comparison.baseline,
                comparison.candidate,
                comparison.candidate_change_percent,
                comparison.baseline_over_candidate,
            ));
        }
    }
    markdown
}

fn format_optional(value: Option<f64>) -> String {
    value.map_or_else(|| "n/a".to_owned(), |value| format!("{value:.3}"))
}

#[cfg(target_os = "linux")]
fn display_server() -> Option<String> {
    if env::var_os("WAYLAND_DISPLAY").is_some() {
        Some("wayland".to_owned())
    } else if env::var_os("DISPLAY").is_some() {
        Some("x11".to_owned())
    } else {
        None
    }
}

#[cfg(not(target_os = "linux"))]
fn display_server() -> Option<String> {
    None
}

#[cfg(target_os = "windows")]
fn process_cpu_time() -> Option<ProcessCpuTime> {
    use windows::Win32::{
        Foundation::FILETIME,
        System::Threading::{GetCurrentProcess, GetProcessTimes},
    };

    let mut creation = FILETIME::default();
    let mut exit = FILETIME::default();
    let mut kernel = FILETIME::default();
    let mut user = FILETIME::default();
    // GetCurrentProcess returns a process-local pseudo-handle that must not be closed.
    unsafe {
        GetProcessTimes(
            GetCurrentProcess(),
            &mut creation,
            &mut exit,
            &mut kernel,
            &mut user,
        )
        .ok()?;
    }
    Some(ProcessCpuTime {
        user: filetime_duration(user),
        kernel: filetime_duration(kernel),
    })
}

#[cfg(target_os = "windows")]
fn filetime_duration(value: windows::Win32::Foundation::FILETIME) -> Duration {
    let intervals = (u64::from(value.dwHighDateTime) << 32) | u64::from(value.dwLowDateTime);
    Duration::from_nanos(intervals.saturating_mul(100))
}

#[cfg(unix)]
fn process_cpu_time() -> Option<ProcessCpuTime> {
    let mut usage = std::mem::MaybeUninit::<libc::rusage>::uninit();
    // getrusage initializes the complete structure when it returns success.
    if unsafe { libc::getrusage(libc::RUSAGE_SELF, usage.as_mut_ptr()) } != 0 {
        return None;
    }
    let usage = unsafe { usage.assume_init() };
    Some(ProcessCpuTime {
        user: timeval_duration(usage.ru_utime),
        kernel: timeval_duration(usage.ru_stime),
    })
}

#[cfg(unix)]
fn timeval_duration(value: libc::timeval) -> Duration {
    let seconds = value.tv_sec.max(0) as u64;
    let microseconds = value.tv_usec.clamp(0, 999_999) as u32;
    Duration::new(seconds, microseconds * 1_000)
}

#[cfg(not(any(unix, target_os = "windows")))]
fn process_cpu_time() -> Option<ProcessCpuTime> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn percentile_uses_nearest_rank() {
        let values = [1, 2, 3, 4, 5];
        assert_eq!(percentile(&values, 50), 3);
        assert_eq!(percentile(&values, 95), 5);
        assert_eq!(percentile(&values, 99), 5);
    }

    #[test]
    fn comparison_reports_reduction_and_ratio() {
        let mut comparisons = Vec::new();
        add_comparison(
            &mut comparisons,
            Scenario::Caret,
            "present p95 (ms)",
            10.0,
            2.0,
        );
        let comparison = &comparisons[0];
        assert_eq!(comparison.candidate_change_percent, -80.0);
        assert_eq!(comparison.baseline_over_candidate, 5.0);
    }
}
