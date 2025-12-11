use std::{
    borrow::Cow,
    collections::HashMap,
    fmt::Display,
    io::{IsTerminal, Write},
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Step {
    LoadProject,
    Context,
    FormatPrompt,
    Predict,
    Score,
}

impl Step {
    pub fn label(&self) -> &'static str {
        match self {
            Step::LoadProject => "Loading",
            Step::Context => "Context",
            Step::FormatPrompt => "Formatting",
            Step::Predict => "Predicting",
            Step::Score => "Scoring",
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

impl Display for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label())
    }
}

#[derive(Clone)]
struct InProgressTask {
    step: Step,
    started_at: Instant,
    substatus: Option<String>,
}

struct CompletedTask {
    step: Step,
    example_name: String,
    duration: Duration,
}

struct ProgressInner {
    completed: Vec<CompletedTask>,
    in_progress: HashMap<String, InProgressTask>,
    is_tty: bool,
    terminal_width: usize,
}

pub struct Progress {
    inner: Mutex<ProgressInner>,
}

impl Progress {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            inner: Mutex::new(ProgressInner {
                completed: Vec::new(),
                in_progress: HashMap::new(),
                is_tty: std::io::stderr().is_terminal(),
                terminal_width: get_terminal_width(),
            }),
        })
    }

    pub fn start(self: &Arc<Self>, step: Step, example_name: &str) -> StepGuard {
        {
            let mut inner = self.inner.lock().unwrap();

            Self::clear_line(&inner);

            inner.in_progress.insert(
                example_name.to_string(),
                InProgressTask {
                    step,
                    started_at: Instant::now(),
                    substatus: None,
                },
            );

            Self::print_status_line(&inner);
        }

        StepGuard {
            progress: self.clone(),
            step,
            example_name: example_name.to_string(),
        }
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

        if inner.is_tty {
            let reset = "\x1b[0m";
            let bold = "\x1b[1m";
            let dim = "\x1b[2m";

            eprintln!(
                "{bold}{color}{label:>12}{reset} {name} {dim}({duration}){reset}",
                color = task.step.color_code(),
                label = task.step.label(),
                name = task.example_name,
            );
        } else {
            eprintln!(
                "{label:>12} {name} ({duration})",
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

        let prefix = format!("{bold}{cyan}{:>12}{reset} ", "Working");
        let prefix_visible_len = 13; // "     Working " without ANSI codes

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

    pub fn set_substatus(&self, example_name: &str, substatus: impl Into<Cow<'static, str>>) {
        let mut inner = self.inner.lock().unwrap();
        if let Some(task) = inner.in_progress.get_mut(example_name) {
            task.substatus = Some(substatus.into().into_owned());
            Self::clear_line(&inner);
            Self::print_status_line(&inner);
        }
    }

    pub fn clear_substatus(&self, example_name: &str) {
        let mut inner = self.inner.lock().unwrap();
        if let Some(task) = inner.in_progress.get_mut(example_name) {
            task.substatus = None;
            Self::clear_line(&inner);
            Self::print_status_line(&inner);
        }
    }

    pub fn clear(&self) {
        let inner = self.inner.lock().unwrap();
        Self::clear_line(&inner);
    }
}

fn truncate_with_ellipsis(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}…", &s[..max_len.saturating_sub(1)])
    }
}

fn format_duration(duration: Duration) -> String {
    let millis = duration.as_millis();
    if millis < 1000 {
        format!("{}ms", millis)
    } else {
        let secs = duration.as_secs_f64();
        if secs < 60.0 {
            format!("{:.1}s", secs)
        } else {
            let mins = secs / 60.0;
            format!("{:.1}m", mins)
        }
    }
}

pub struct StepGuard {
    progress: Arc<Progress>,
    step: Step,
    example_name: String,
}

impl Drop for StepGuard {
    fn drop(&mut self) {
        self.progress.finish(self.step, &self.example_name);
    }
}
