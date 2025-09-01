use gpui::{App, AppContext, Context, Entity};
use language::LanguageRegistry;
use markdown::Markdown;
use std::{path::PathBuf, process::ExitStatus, sync::Arc, time::Instant};

pub struct Terminal {
    command: Entity<Markdown>,
    working_dir: Option<PathBuf>,
    terminal: Entity<terminal::Terminal>,
    started_at: Instant,
    output: Option<TerminalOutput>,
}

pub struct TerminalOutput {
    pub ended_at: Instant,
    pub exit_status: Option<ExitStatus>,
    pub was_content_truncated: bool,
    pub original_content_len: usize,
    pub content_line_count: usize,
    pub finished_with_empty_output: bool,
}

impl Terminal {
    pub fn new(
        command: String,
        working_dir: Option<PathBuf>,
        terminal: Entity<terminal::Terminal>,
        language_registry: Arc<LanguageRegistry>,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            command: cx.new(|cx| {
                Markdown::new(
                    format!("```\n{}\n```", command).into(),
                    Some(language_registry.clone()),
                    None,
                    cx,
                )
            }),
            working_dir,
            terminal,
            started_at: Instant::now(),
            output: None,
        }
    }

    pub fn finish(
        &mut self,
        exit_status: Option<ExitStatus>,
        original_content_len: usize,
        truncated_content_len: usize,
        content_line_count: usize,
        finished_with_empty_output: bool,
        cx: &mut Context<Self>,
    ) {
        self.output = Some(TerminalOutput {
            ended_at: Instant::now(),
            exit_status,
            was_content_truncated: truncated_content_len < original_content_len,
            original_content_len,
            content_line_count,
            finished_with_empty_output,
        });
        cx.notify();
    }

    pub fn command(&self) -> &Entity<Markdown> {
        &self.command
    }

    pub fn working_dir(&self) -> &Option<PathBuf> {
        &self.working_dir
    }

    pub fn started_at(&self) -> Instant {
        self.started_at
    }

    pub fn output(&self) -> Option<&TerminalOutput> {
        self.output.as_ref()
    }

    pub fn inner(&self) -> &Entity<terminal::Terminal> {
        &self.terminal
    }

    pub fn to_markdown(&self, cx: &App) -> String {
        format!(
            "Terminal:\n```\n{}\n```\n",
            self.terminal.read(cx).get_content()
        )
    }
}
