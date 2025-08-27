use agent_client_protocol as acp;
use anyhow::Result;
use futures::FutureExt as _;
use gpui::{App, AppContext, Context, Entity, Task};
use language::LanguageRegistry;
use markdown::Markdown;
use project::{Project, terminals::TerminalKind};
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

    pub fn new2(
        command: String,
        extra_env: Vec<acp::EnvVariable>,
        working_dir: Option<PathBuf>,
        project: Entity<Project>,
        cx: &mut App,
    ) -> Task<Result<Self>> {
        let language_registry = project.read_with(cx, |p, _| p.languages().clone());

        let command_md = cx.new(|cx| {
            Markdown::new(
                format!("```\n{}\n```", command).into(),
                Some(language_registry.clone()),
                None,
                cx,
            )
        });

        // let program = self.determine_shell.clone();
        // todo!
        let program = "bash".to_string();
        let command = if cfg!(windows) {
            format!("$null | & {{{}}}", command.replace("\"", "'"))
        } else if let Some(cwd) = working_dir
            .as_ref()
            .and_then(|cwd| cwd.as_os_str().to_str())
        {
            // Make sure once we're *inside* the shell, we cd into `cwd`
            format!("(cd {cwd}; {}) </dev/null", command)
        } else {
            format!("({}) </dev/null", command)
        };
        let args = vec!["-c".into(), command.clone()];

        let env = match &working_dir {
            Some(dir) => project.update(cx, |project, cx| {
                project.directory_environment(dir.as_path().into(), cx)
            }),
            None => Task::ready(None).shared(),
        };

        let env = cx.spawn(async move |_| {
            let mut env = env.await.unwrap_or_default();
            if cfg!(unix) {
                env.insert("PAGER".into(), "cat".into());
            }
            for var in extra_env {
                env.insert(var.name, var.value);
            }
            env
        });

        cx.spawn(async move |cx| {
            let env = env.await;
            let terminal = project
                .update(cx, |project, cx| {
                    project.create_terminal(
                        TerminalKind::Task(task::SpawnInTerminal {
                            command: Some(program),
                            args,
                            cwd: working_dir.clone(),
                            env,
                            ..Default::default()
                        }),
                        cx,
                    )
                })?
                .await?;

            Ok(Self {
                command: command_md,
                working_dir,
                terminal,
                started_at: Instant::now(),
                output: None,
            })
        })
    }

    pub fn output2(
        &mut self,
        limit: Option<usize>,
        cx: &mut Context<Self>,
    ) -> Task<Result<acp::TerminalOutputResponse>> {
        let terminal = self.terminal.downgrade();
        let exit_status = self
            .terminal
            .update(cx, |terminal, cx| terminal.wait_for_completed_task(cx));

        cx.spawn(async move |this, cx| {
            let exit_status = exit_status.await;

            let (mut content, content_line_count) = terminal.read_with(cx, |terminal, _| {
                (terminal.get_content(), terminal.total_lines())
            })?;

            let original_content_len = content.len();

            if let Some(limit) = limit
                && content.len() > limit
            {
                let mut end_ix = limit.min(content.len());
                while !content.is_char_boundary(end_ix) {
                    end_ix -= 1;
                }
                // Don't truncate mid-line, clear the remainder of the last line
                end_ix = content[..end_ix].rfind('\n').unwrap_or(end_ix);
                content.truncate(end_ix + 1);
            }

            let truncated = content.len() < original_content_len;

            this.update(cx, |this, cx| {
                this.output = Some(TerminalOutput {
                    ended_at: Instant::now(),
                    exit_status,
                    // todo! do we need this?
                    was_content_truncated: truncated,
                    original_content_len,
                    content_line_count,
                    // todo! do we need this?
                    finished_with_empty_output: content.is_empty(),
                });
                cx.notify();
            })
            .ok();
            let exit_status = exit_status.map(portable_pty::ExitStatus::from);

            Ok(acp::TerminalOutputResponse {
                exit_code: exit_status.as_ref().map(|e| e.exit_code()),
                signal: exit_status.and_then(|e| e.signal().map(Into::into)),
                output: content,
                truncated,
            })
        })
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
