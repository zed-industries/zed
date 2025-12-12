use std::{
    borrow::Cow,
    collections::HashMap,
    io::{IsTerminal, Write},
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

pub struct Progress {
    inner: Mutex<ProgressInner>,
}

struct ProgressInner {
    completed: Vec<CompletedTask>,
    in_progress: HashMap<String, InProgressTask>,
    is_tty: bool,
    terminal_width: usize,
    max_example_name_len: usize,
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
        }
    }

    fn color_code(&self) -> &'static str {
        match self {
            Step::LoadProject => "\x1b[33m",
            Step::Context => "\x1b[35m",
            Step::FormatPrompt => "\x1b[34m",
            Step::Predict => "\x1b[32m",
            Step::Score => "\x1b[31m",
        }
    }
}

const RIGHT_MARGIN: usize = 4;

impl Progress {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            inner: Mutex::new(ProgressInner {
                completed: Vec::new(),
                in_progress: HashMap::new(),
                is_tty: std::io::stderr().is_terminal(),
                terminal_width: get_terminal_width(),
                max_example_name_len: 0,
            }),
        })
    }

    pub fn start(self: &Arc<Self>, step: Step, example_name: &str) -> Arc<StepProgress> {
        {
            let mut inner = self.inner.lock().unwrap();

            Self::clear_line(&inner);

            inner.max_example_name_len = inner.max_example_name_len.max(example_name.len());

            inner.in_progress.insert(
                example_name.to_string(),
                InProgressTask {
                    step,
                    started_at: Instant::now(),
                    substatus: None,
                    info: None,
                },
            );

            Self::print_status_line(&inner);
        }

        Arc::new(StepProgress {
            progress: self.clone(),
            step,
            example_name: example_name.to_string(),
        })
    }

    pub fn finish(&self, step: Step, example_name: &str) {
        let mut inner = self.inner.lock().unwrap();

        let task = inner.in_progress.remove(example_name);
        if let Some(task) = task {
            if task.step == step {
                inner.completed.push(CompletedTask {
                    step: task.step,
                    example_name: example_name.to_string(),
                    duration: task.started_at.elapsed(),
                    info: task.info,
                });

                Self::clear_line(&inner);
                Self::print_completed(&inner, inner.completed.last().unwrap());
                Self::print_status_line(&inner);
            } else {
                inner
                    .in_progress
                    .insert(example_name.to_string(), task.clone());
            }
        }
    }

    fn clear_line(inner: &ProgressInner) {
        if inner.is_tty {
            eprint!("\r\x1b[K");
            let _ = std::io::stderr().flush();
        }
    }

    fn print_completed(inner: &ProgressInner, task: &CompletedTask) {
        let duration = format_duration(task.duration);
        let name_width = inner.max_example_name_len;

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
                        format!(" {dim}│{reset} {yellow}{s}{reset}")
                    } else {
                        format!(" {dim}│{reset} {s}")
                    }
                })
                .unwrap_or_default();

            let prefix = format!(
                "{bold}{color}{label:>12}{reset} {name:<name_width$}{info_part}",
                color = task.step.color_code(),
                label = task.step.label(),
                name = task.example_name,
            );

            let duration_with_margin = format!("{duration} ");
            let padding_needed = inner
                .terminal_width
                .saturating_sub(RIGHT_MARGIN)
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
                name = task.example_name,
            );
        }
    }

    fn print_status_line(inner: &ProgressInner) {
        if !inner.is_tty || inner.in_progress.is_empty() {
            return;
        }

        let reset = "\x1b[0m";
        let bold = "\x1b[1m";
        let cyan = "\x1b[36m";
        let dim = "\x1b[2m";

        let mut tasks: Vec<_> = inner.in_progress.iter().collect();
        tasks.sort_by_key(|(name, _)| *name);

        let prefix_label = format!("{:>12} ", "Working");
        let prefix_visible_len = prefix_label.len();
        let prefix = format!("{bold}{cyan}{prefix_label}{reset}");

        let mut line = prefix;
        let mut visible_len = prefix_visible_len;
        let mut shown_count = 0;
        let total_tasks = tasks.len();

        for (name, task) in tasks.iter() {
            let elapsed = format_duration(task.started_at.elapsed());
            let substatus = task
                .substatus
                .as_ref()
                .map(|s| format!(": {}", truncate_with_ellipsis(s, 20)))
                .unwrap_or_default();

            let task_str = format!(
                "{name} {dim}[{step}{substatus}, {elapsed}]{reset}",
                step = task.step.label(),
                name = truncate_with_ellipsis(name, 20),
            );

            // Calculate visible length (without ANSI codes)
            let task_visible_len = format!(
                "{name} [{step}{substatus}, {elapsed}]",
                step = task.step.label(),
                name = truncate_with_ellipsis(name, 20),
            )
            .len();

            let separator = if shown_count > 0 { ", " } else { "" };
            let separator_len = separator.len();

            // Check if adding this task would exceed terminal width
            // Leave room for potential "(+N more)" suffix
            let remaining_suffix_space = 12;
            if visible_len + separator_len + task_visible_len + remaining_suffix_space
                > inner.terminal_width
                && shown_count > 0
            {
                break;
            }

            line.push_str(separator);
            line.push_str(&task_str);
            visible_len += separator_len + task_visible_len;
            shown_count += 1;
        }

        let remaining = total_tasks - shown_count;
        if remaining > 0 {
            line.push_str(&format!(" {dim}(+{remaining} more){reset}"));
        }

        eprint!("{}", line);
        let _ = std::io::stderr().flush();
    }

    pub fn clear(&self) {
        let inner = self.inner.lock().unwrap();
        Self::clear_line(&inner);
    }

    pub fn batch_separator(&self, start_index: usize, end_index: usize, total_examples: usize) {
        let inner = self.inner.lock().unwrap();
        Self::clear_line(&inner);

        eprintln!();

        let reset = "\x1b[0m";
        let dim = "\x1b[2m";

        let label = if start_index + 1 == end_index {
            format!(" {}/{} ", start_index + 1, total_examples)
        } else {
            format!(" {}-{}/{} ", start_index + 1, end_index, total_examples)
        };

        let left_width = inner
            .terminal_width
            .saturating_sub(label.chars().count())
            .saturating_sub(RIGHT_MARGIN);
        let left_line = "─".repeat(left_width);
        let right_line = "─".repeat(RIGHT_MARGIN);

        if inner.is_tty {
            eprintln!("{dim}{left_line}{reset}{label}{dim}{right_line}{reset}");
        } else {
            eprintln!("{left_line}{label}{right_line}");
        }

        eprintln!();
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
            Progress::clear_line(&inner);
            Progress::print_status_line(&inner);
        }
    }

    pub fn clear_substatus(&self) {
        let mut inner = self.progress.inner.lock().unwrap();
        if let Some(task) = inner.in_progress.get_mut(&self.example_name) {
            task.substatus = None;
            Progress::clear_line(&inner);
            Progress::print_status_line(&inner);
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
