use std::{
    borrow::Cow,
    collections::HashMap,
    io::{IsTerminal, Write},
    sync::{Arc, Mutex, OnceLock},
    time::{Duration, Instant},
};

use log::{Level, Log, Metadata, Record};

pub struct Progress {
    inner: Mutex<ProgressInner>,
}

struct ProgressInner {
    completed: Vec<CompletedTask>,
    in_progress: HashMap<String, InProgressTask>,
    is_tty: bool,
    terminal_width: usize,
    max_example_name_len: usize,
    status_lines_displayed: usize,
    total_steps: usize,
    failed_examples: usize,
    last_line_is_logging: bool,
    ticker: Option<std::thread::JoinHandle<()>>,
}

#[derive(Clone)]
struct InProgressTask {
    step: Step,
    started_at: Instant,
    substatus: Option<String>,
    info: Option<(String, InfoStyle)>,
}

struct CompletedTask {
    step: Step,
    example_name: String,
    duration: Duration,
    info: Option<(String, InfoStyle)>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Step {
    LoadProject,
    Context,
    FormatPrompt,
    Predict,
    Score,
    Synthesize,
    PullExamples,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InfoStyle {
    Normal,
    Warning,
}

impl Step {
    pub fn label(&self) -> &'static str {
        match self {
            Step::LoadProject => "Load",
            Step::Context => "Context",
            Step::FormatPrompt => "Format",
            Step::Predict => "Predict",
            Step::Score => "Score",
            Step::Synthesize => "Synthesize",
            Step::PullExamples => "Pull",
        }
    }

    fn color_code(&self) -> &'static str {
        match self {
            Step::LoadProject => "\x1b[33m",
            Step::Context => "\x1b[35m",
            Step::FormatPrompt => "\x1b[34m",
            Step::Predict => "\x1b[32m",
            Step::Score => "\x1b[31m",
            Step::Synthesize => "\x1b[36m",
            Step::PullExamples => "\x1b[36m",
        }
    }
}

static GLOBAL: OnceLock<Arc<Progress>> = OnceLock::new();
static LOGGER: ProgressLogger = ProgressLogger;

const MARGIN: usize = 4;
const MAX_STATUS_LINES: usize = 10;
const STATUS_TICK_INTERVAL: Duration = Duration::from_millis(300);

impl Progress {
    /// Returns the global Progress instance, initializing it if necessary.
    pub fn global() -> Arc<Progress> {
        GLOBAL
            .get_or_init(|| {
                let progress = Arc::new(Self {
                    inner: Mutex::new(ProgressInner {
                        completed: Vec::new(),
                        in_progress: HashMap::new(),
                        is_tty: std::io::stderr().is_terminal(),
                        terminal_width: get_terminal_width(),
                        max_example_name_len: 0,
                        status_lines_displayed: 0,
                        total_steps: 0,
                        failed_examples: 0,
                        last_line_is_logging: false,
                        ticker: None,
                    }),
                });
                let _ = log::set_logger(&LOGGER);
                log::set_max_level(log::LevelFilter::Error);
                progress
            })
            .clone()
    }

    pub fn set_total_steps(&self, total: usize) {
        let mut inner = self.inner.lock().unwrap();
        inner.total_steps = total;
    }

    pub fn increment_failed(&self) {
        let mut inner = self.inner.lock().unwrap();
        inner.failed_examples += 1;
    }

    /// Prints a message to stderr, clearing and redrawing status lines to avoid corruption.
    /// This should be used for any output that needs to appear above the status lines.
    fn log(&self, message: &str) {
        let mut inner = self.inner.lock().unwrap();
        Self::clear_status_lines(&mut inner);

        if !inner.last_line_is_logging {
            let reset = "\x1b[0m";
            let dim = "\x1b[2m";
            let divider = "─".repeat(inner.terminal_width.saturating_sub(MARGIN));
            eprintln!("{dim}{divider}{reset}");
            inner.last_line_is_logging = true;
        }

        eprintln!("{}", message);
    }

    pub fn start(self: &Arc<Self>, step: Step, example_name: &str) -> StepProgress {
        let mut inner = self.inner.lock().unwrap();

        Self::clear_status_lines(&mut inner);

        let max_name_width = inner
            .terminal_width
            .saturating_sub(MARGIN * 2)
            .saturating_div(3)
            .max(1);
        inner.max_example_name_len = inner
            .max_example_name_len
            .max(example_name.len().min(max_name_width));
        inner.in_progress.insert(
            example_name.to_string(),
            InProgressTask {
                step,
                started_at: Instant::now(),
                substatus: None,
                info: None,
            },
        );

        if inner.is_tty && inner.ticker.is_none() {
            let progress = self.clone();
            inner.ticker = Some(std::thread::spawn(move || {
                loop {
                    std::thread::sleep(STATUS_TICK_INTERVAL);

                    let mut inner = progress.inner.lock().unwrap();
                    if inner.in_progress.is_empty() {
                        break;
                    }

                    Progress::clear_status_lines(&mut inner);
                    Progress::print_status_lines(&mut inner);
                }
            }));
        }

        Self::print_status_lines(&mut inner);

        StepProgress {
            progress: self.clone(),
            step,
            example_name: example_name.to_string(),
        }
    }

    fn finish(&self, step: Step, example_name: &str) {
        let mut inner = self.inner.lock().unwrap();

        let Some(task) = inner.in_progress.remove(example_name) else {
            return;
        };

        if task.step == step {
            inner.completed.push(CompletedTask {
                step: task.step,
                example_name: example_name.to_string(),
                duration: task.started_at.elapsed(),
                info: task.info,
            });

            Self::clear_status_lines(&mut inner);
            Self::print_logging_closing_divider(&mut inner);
            if let Some(last_completed) = inner.completed.last() {
                Self::print_completed(&inner, last_completed);
            }
            Self::print_status_lines(&mut inner);
        } else {
            inner.in_progress.insert(example_name.to_string(), task);
        }
    }

    fn print_logging_closing_divider(inner: &mut ProgressInner) {
        if inner.last_line_is_logging {
            let reset = "\x1b[0m";
            let dim = "\x1b[2m";
            let divider = "─".repeat(inner.terminal_width.saturating_sub(MARGIN));
            eprintln!("{dim}{divider}{reset}");
            inner.last_line_is_logging = false;
        }
    }

    fn clear_status_lines(inner: &mut ProgressInner) {
        if inner.is_tty && inner.status_lines_displayed > 0 {
            // Move up and clear each line we previously displayed
            for _ in 0..inner.status_lines_displayed {
                eprint!("\x1b[A\x1b[K");
            }
            let _ = std::io::stderr().flush();
            inner.status_lines_displayed = 0;
        }
    }

    fn print_completed(inner: &ProgressInner, task: &CompletedTask) {
        let duration = format_duration(task.duration);
        let name_width = inner.max_example_name_len;
        let truncated_name = truncate_with_ellipsis(&task.example_name, name_width);

        if inner.is_tty {
            let reset = "\x1b[0m";
            let bold = "\x1b[1m";
            let dim = "\x1b[2m";

            let yellow = "\x1b[33m";
            let info_part = task
                .info
                .as_ref()
                .map(|(s, style)| {
                    if *style == InfoStyle::Warning {
                        format!("{yellow}{s}{reset}")
                    } else {
                        s.to_string()
                    }
                })
                .unwrap_or_default();

            let prefix = format!(
                "{bold}{color}{label:>12}{reset} {name:<name_width$} {dim}│{reset} {info_part}",
                color = task.step.color_code(),
                label = task.step.label(),
                name = truncated_name,
            );

            let duration_with_margin = format!("{duration} ");
            let padding_needed = inner
                .terminal_width
                .saturating_sub(MARGIN)
                .saturating_sub(duration_with_margin.len())
                .saturating_sub(strip_ansi_len(&prefix));
            let padding = " ".repeat(padding_needed);

            eprintln!("{prefix}{padding}{dim}{duration_with_margin}{reset}");
        } else {
            let info_part = task
                .info
                .as_ref()
                .map(|(s, _)| format!(" | {}", s))
                .unwrap_or_default();

            eprintln!(
                "{label:>12} {name:<name_width$}{info_part} {duration}",
                label = task.step.label(),
                name = truncate_with_ellipsis(&task.example_name, name_width),
            );
        }
    }

    fn print_status_lines(inner: &mut ProgressInner) {
        if !inner.is_tty || inner.in_progress.is_empty() {
            inner.status_lines_displayed = 0;
            return;
        }

        let reset = "\x1b[0m";
        let bold = "\x1b[1m";
        let dim = "\x1b[2m";

        // Build the done/in-progress/total label
        let done_count = inner.completed.len();
        let in_progress_count = inner.in_progress.len();
        let failed_count = inner.failed_examples;

        let failed_label = if failed_count > 0 {
            format!(" {} failed ", failed_count)
        } else {
            String::new()
        };

        let range_label = format!(
            " {}/{}/{} ",
            done_count, in_progress_count, inner.total_steps
        );

        // Print a divider line with failed count on left, range label on right
        let failed_visible_len = strip_ansi_len(&failed_label);
        let range_visible_len = range_label.len();
        let middle_divider_len = inner
            .terminal_width
            .saturating_sub(MARGIN * 2)
            .saturating_sub(failed_visible_len)
            .saturating_sub(range_visible_len);
        let left_divider = "─".repeat(MARGIN);
        let middle_divider = "─".repeat(middle_divider_len);
        let right_divider = "─".repeat(MARGIN);
        eprintln!(
            "{dim}{left_divider}{reset}{failed_label}{dim}{middle_divider}{reset}{range_label}{dim}{right_divider}{reset}"
        );

        let mut tasks: Vec<_> = inner.in_progress.iter().collect();
        tasks.sort_by_key(|(name, _)| *name);

        let total_tasks = tasks.len();
        let mut lines_printed = 0;

        for (name, task) in tasks.iter().take(MAX_STATUS_LINES) {
            let elapsed = format_duration(task.started_at.elapsed());
            let substatus_part = task
                .substatus
                .as_ref()
                .map(|s| truncate_with_ellipsis(s, 30))
                .unwrap_or_default();

            let step_label = task.step.label();
            let step_color = task.step.color_code();
            let name_width = inner.max_example_name_len;
            let truncated_name = truncate_with_ellipsis(name, name_width);

            let prefix = format!(
                "{bold}{step_color}{step_label:>12}{reset} {name:<name_width$} {dim}│{reset} {substatus_part}",
                name = truncated_name,
            );

            let duration_with_margin = format!("{elapsed} ");
            let padding_needed = inner
                .terminal_width
                .saturating_sub(MARGIN)
                .saturating_sub(duration_with_margin.len())
                .saturating_sub(strip_ansi_len(&prefix));
            let padding = " ".repeat(padding_needed);

            eprintln!("{prefix}{padding}{dim}{duration_with_margin}{reset}");
            lines_printed += 1;
        }

        // Show "+N more" on its own line if there are more tasks
        if total_tasks > MAX_STATUS_LINES {
            let remaining = total_tasks - MAX_STATUS_LINES;
            eprintln!("{:>12} +{remaining} more", "");
            lines_printed += 1;
        }

        inner.status_lines_displayed = lines_printed + 1; // +1 for the divider line
        let _ = std::io::stderr().flush();
    }

    pub fn finalize(&self) {
        let ticker = {
            let mut inner = self.inner.lock().unwrap();
            inner.ticker.take()
        };

        if let Some(ticker) = ticker {
            let _ = ticker.join();
        }

        let mut inner = self.inner.lock().unwrap();
        Self::clear_status_lines(&mut inner);

        // Print summary if there were failures
        if inner.failed_examples > 0 {
            let total_processed = inner.completed.len();
            let percentage = if total_processed > 0 {
                inner.failed_examples as f64 / total_processed as f64 * 100.0
            } else {
                0.0
            };
            eprintln!(
                "\n{} of {} examples failed ({:.1}%)",
                inner.failed_examples, total_processed, percentage
            );
        }
    }
}

pub struct StepProgress {
    progress: Arc<Progress>,
    step: Step,
    example_name: String,
}

impl StepProgress {
    pub fn set_substatus(&self, substatus: impl Into<Cow<'static, str>>) {
        let mut inner = self.progress.inner.lock().unwrap();
        if let Some(task) = inner.in_progress.get_mut(&self.example_name) {
            task.substatus = Some(substatus.into().into_owned());
            Progress::clear_status_lines(&mut inner);
            Progress::print_status_lines(&mut inner);
        }
    }

    pub fn clear_substatus(&self) {
        let mut inner = self.progress.inner.lock().unwrap();
        if let Some(task) = inner.in_progress.get_mut(&self.example_name) {
            task.substatus = None;
            Progress::clear_status_lines(&mut inner);
            Progress::print_status_lines(&mut inner);
        }
    }

    pub fn set_info(&self, info: impl Into<String>, style: InfoStyle) {
        let mut inner = self.progress.inner.lock().unwrap();
        if let Some(task) = inner.in_progress.get_mut(&self.example_name) {
            task.info = Some((info.into(), style));
        }
    }
}

impl Drop for StepProgress {
    fn drop(&mut self) {
        self.progress.finish(self.step, &self.example_name);
    }
}

struct ProgressLogger;

impl Log for ProgressLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let level_color = match record.level() {
            Level::Error => "\x1b[31m",
            Level::Warn => "\x1b[33m",
            Level::Info => "\x1b[32m",
            Level::Debug => "\x1b[34m",
            Level::Trace => "\x1b[35m",
        };
        let reset = "\x1b[0m";
        let bold = "\x1b[1m";

        let level_label = match record.level() {
            Level::Error => "Error",
            Level::Warn => "Warn",
            Level::Info => "Info",
            Level::Debug => "Debug",
            Level::Trace => "Trace",
        };

        let message = format!(
            "{bold}{level_color}{level_label:>12}{reset} {}",
            record.args()
        );

        if let Some(progress) = GLOBAL.get() {
            progress.log(&message);
        } else {
            eprintln!("{}", message);
        }
    }

    fn flush(&self) {
        let _ = std::io::stderr().flush();
    }
}

#[cfg(unix)]
fn get_terminal_width() -> usize {
    unsafe {
        let mut winsize: libc::winsize = std::mem::zeroed();
        if libc::ioctl(libc::STDERR_FILENO, libc::TIOCGWINSZ, &mut winsize) == 0
            && winsize.ws_col > 0
        {
            winsize.ws_col as usize
        } else {
            80
        }
    }
}

#[cfg(not(unix))]
fn get_terminal_width() -> usize {
    80
}

fn strip_ansi_len(s: &str) -> usize {
    let mut len = 0;
    let mut in_escape = false;
    for c in s.chars() {
        if c == '\x1b' {
            in_escape = true;
        } else if in_escape {
            if c == 'm' {
                in_escape = false;
            }
        } else {
            len += 1;
        }
    }
    len
}

fn truncate_with_ellipsis(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}…", &s[..max_len.saturating_sub(1)])
    }
}

fn format_duration(duration: Duration) -> String {
    const MINUTE_IN_MILLIS: f32 = 60. * 1000.;

    let millis = duration.as_millis() as f32;
    if millis < 1000.0 {
        format!("{}ms", millis)
    } else if millis < MINUTE_IN_MILLIS {
        format!("{:.1}s", millis / 1_000.0)
    } else {
        format!("{:.1}m", millis / MINUTE_IN_MILLIS)
    }
}
