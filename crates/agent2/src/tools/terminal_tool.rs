use agent_client_protocol as acp;
use anyhow::Result;
use futures::{FutureExt as _, future::Shared};
use gpui::{App, AppContext, Entity, SharedString, Task};
use project::{Project, terminals::TerminalKind};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use util::{ResultExt, get_system_shell, markdown::MarkdownInlineCode};

use crate::{AgentTool, ToolCallEventStream};

const COMMAND_OUTPUT_LIMIT: usize = 16 * 1024;

/// Executes a shell one-liner and returns the combined output.
///
/// This tool spawns a process using the user's shell, reads from stdout and stderr (preserving the order of writes), and returns a string with the combined output result.
///
/// The output results will be shown to the user already, only list it again if necessary, avoid being redundant.
///
/// Make sure you use the `cd` parameter to navigate to one of the root directories of the project. NEVER do it as part of the `command` itself, otherwise it will error.
///
/// Do not use this tool for commands that run indefinitely, such as servers (like `npm run start`, `npm run dev`, `python -m http.server`, etc) or file watchers that don't terminate on their own.
///
/// Remember that each invocation of this tool will spawn a new shell process, so you can't rely on any state from previous invocations.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct TerminalToolInput {
    /// The one-liner command to execute.
    command: String,
    /// Working directory for the command. This must be one of the root directories of the project.
    cd: String,
}

pub struct TerminalTool {
    project: Entity<Project>,
    determine_shell: Shared<Task<String>>,
}

impl TerminalTool {
    pub fn new(project: Entity<Project>, cx: &mut App) -> Self {
        let determine_shell = cx.background_spawn(async move {
            if cfg!(windows) {
                return get_system_shell();
            }

            if which::which("bash").is_ok() {
                "bash".into()
            } else {
                get_system_shell()
            }
        });
        Self {
            project,
            determine_shell: determine_shell.shared(),
        }
    }
}

impl AgentTool for TerminalTool {
    type Input = TerminalToolInput;
    type Output = String;

    fn name(&self) -> SharedString {
        "terminal".into()
    }

    fn kind(&self) -> acp::ToolKind {
        acp::ToolKind::Execute
    }

    fn initial_title(&self, input: Result<Self::Input, serde_json::Value>) -> SharedString {
        if let Ok(input) = input {
            let mut lines = input.command.lines();
            let first_line = lines.next().unwrap_or_default();
            let remaining_line_count = lines.count();
            match remaining_line_count {
                0 => MarkdownInlineCode(first_line).to_string().into(),
                1 => MarkdownInlineCode(&format!(
                    "{} - {} more line",
                    first_line, remaining_line_count
                ))
                .to_string()
                .into(),
                n => MarkdownInlineCode(&format!("{} - {} more lines", first_line, n))
                    .to_string()
                    .into(),
            }
        } else {
            "Run terminal command".into()
        }
    }

    fn run(
        self: Arc<Self>,
        input: Self::Input,
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output>> {
        let language_registry = self.project.read(cx).languages().clone();
        let working_dir = match working_dir(&input, &self.project, cx) {
            Ok(dir) => dir,
            Err(err) => return Task::ready(Err(err)),
        };
        let program = self.determine_shell.clone();
        let command = if cfg!(windows) {
            format!("$null | & {{{}}}", input.command.replace("\"", "'"))
        } else if let Some(cwd) = working_dir
            .as_ref()
            .and_then(|cwd| cwd.as_os_str().to_str())
        {
            // Make sure once we're *inside* the shell, we cd into `cwd`
            format!("(cd {cwd}; {}) </dev/null", input.command)
        } else {
            format!("({}) </dev/null", input.command)
        };
        let args = vec!["-c".into(), command];

        let env = match &working_dir {
            Some(dir) => self.project.update(cx, |project, cx| {
                project.directory_environment(dir.as_path().into(), cx)
            }),
            None => Task::ready(None).shared(),
        };

        let env = cx.spawn(async move |_| {
            let mut env = env.await.unwrap_or_default();
            if cfg!(unix) {
                env.insert("PAGER".into(), "cat".into());
            }
            env
        });

        let authorize = event_stream.authorize(self.initial_title(Ok(input.clone())), cx);

        cx.spawn({
            async move |cx| {
                authorize.await?;

                let program = program.await;
                let env = env.await;
                let terminal = self
                    .project
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
                let acp_terminal = cx.new(|cx| {
                    acp_thread::Terminal::new(
                        input.command.clone(),
                        working_dir.clone(),
                        terminal.clone(),
                        language_registry,
                        cx,
                    )
                })?;
                event_stream.update_terminal(acp_terminal.clone());

                let exit_status = terminal
                    .update(cx, |terminal, cx| terminal.wait_for_completed_task(cx))?
                    .await;
                let (content, content_line_count) = terminal.read_with(cx, |terminal, _| {
                    (terminal.get_content(), terminal.total_lines())
                })?;

                let (processed_content, finished_with_empty_output) = process_content(
                    &content,
                    &input.command,
                    exit_status.map(portable_pty::ExitStatus::from),
                );

                acp_terminal
                    .update(cx, |terminal, cx| {
                        terminal.finish(
                            exit_status,
                            content.len(),
                            processed_content.len(),
                            content_line_count,
                            finished_with_empty_output,
                            cx,
                        );
                    })
                    .log_err();

                Ok(processed_content)
            }
        })
    }
}

fn process_content(
    content: &str,
    command: &str,
    exit_status: Option<portable_pty::ExitStatus>,
) -> (String, bool) {
    let should_truncate = content.len() > COMMAND_OUTPUT_LIMIT;

    let content = if should_truncate {
        let mut end_ix = COMMAND_OUTPUT_LIMIT.min(content.len());
        while !content.is_char_boundary(end_ix) {
            end_ix -= 1;
        }
        // Don't truncate mid-line, clear the remainder of the last line
        end_ix = content[..end_ix].rfind('\n').unwrap_or(end_ix);
        &content[..end_ix]
    } else {
        content
    };
    let content = content.trim();
    let is_empty = content.is_empty();
    let content = format!("```\n{content}\n```");
    let content = if should_truncate {
        format!(
            "Command output too long. The first {} bytes:\n\n{content}",
            content.len(),
        )
    } else {
        content
    };

    let content = match exit_status {
        Some(exit_status) if exit_status.success() => {
            if is_empty {
                "Command executed successfully.".to_string()
            } else {
                content
            }
        }
        Some(exit_status) => {
            if is_empty {
                format!(
                    "Command \"{command}\" failed with exit code {}.",
                    exit_status.exit_code()
                )
            } else {
                format!(
                    "Command \"{command}\" failed with exit code {}.\n\n{content}",
                    exit_status.exit_code()
                )
            }
        }
        None => {
            format!(
                "Command failed or was interrupted.\nPartial output captured:\n\n{}",
                content,
            )
        }
    };
    (content, is_empty)
}

fn working_dir(
    input: &TerminalToolInput,
    project: &Entity<Project>,
    cx: &mut App,
) -> Result<Option<PathBuf>> {
    let project = project.read(cx);
    let cd = &input.cd;

    if cd == "." || cd.is_empty() {
        // Accept "." or "" as meaning "the one worktree" if we only have one worktree.
        let mut worktrees = project.worktrees(cx);

        match worktrees.next() {
            Some(worktree) => {
                anyhow::ensure!(
                    worktrees.next().is_none(),
                    "'.' is ambiguous in multi-root workspaces. Please specify a root directory explicitly.",
                );
                Ok(Some(worktree.read(cx).abs_path().to_path_buf()))
            }
            None => Ok(None),
        }
    } else {
        let input_path = Path::new(cd);

        if input_path.is_absolute() {
            // Absolute paths are allowed, but only if they're in one of the project's worktrees.
            if project
                .worktrees(cx)
                .any(|worktree| input_path.starts_with(&worktree.read(cx).abs_path()))
            {
                return Ok(Some(input_path.into()));
            }
        } else if let Some(worktree) = project.worktree_for_root_name(cd, cx) {
            return Ok(Some(worktree.read(cx).abs_path().to_path_buf()));
        }

        anyhow::bail!("`cd` directory {cd:?} was not in any of the project's worktrees.");
    }
}

#[cfg(test)]
mod tests {
    use agent_settings::AgentSettings;
    use editor::EditorSettings;
    use fs::RealFs;
    use gpui::{BackgroundExecutor, TestAppContext};
    use pretty_assertions::assert_eq;
    use serde_json::json;
    use settings::{Settings, SettingsStore};
    use terminal::terminal_settings::TerminalSettings;
    use theme::ThemeSettings;
    use util::test::TempTree;

    use crate::ThreadEvent;

    use super::*;

    fn init_test(executor: &BackgroundExecutor, cx: &mut TestAppContext) {
        zlog::init_test();

        executor.allow_parking();
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            language::init(cx);
            Project::init_settings(cx);
            ThemeSettings::register(cx);
            TerminalSettings::register(cx);
            EditorSettings::register(cx);
            AgentSettings::register(cx);
        });
    }

    #[gpui::test]
    async fn test_interactive_command(executor: BackgroundExecutor, cx: &mut TestAppContext) {
        if cfg!(windows) {
            return;
        }

        init_test(&executor, cx);

        let fs = Arc::new(RealFs::new(None, executor));
        let tree = TempTree::new(json!({
            "project": {},
        }));
        let project: Entity<Project> =
            Project::test(fs, [tree.path().join("project").as_path()], cx).await;

        let input = TerminalToolInput {
            command: "cat".to_owned(),
            cd: tree
                .path()
                .join("project")
                .as_path()
                .to_string_lossy()
                .to_string(),
        };
        let (event_stream_tx, mut event_stream_rx) = ToolCallEventStream::test();
        let result = cx
            .update(|cx| Arc::new(TerminalTool::new(project, cx)).run(input, event_stream_tx, cx));

        let auth = event_stream_rx.expect_authorization().await;
        auth.response.send(auth.options[0].id.clone()).unwrap();
        event_stream_rx.expect_terminal().await;
        assert_eq!(result.await.unwrap(), "Command executed successfully.");
    }

    #[gpui::test]
    async fn test_working_directory(executor: BackgroundExecutor, cx: &mut TestAppContext) {
        if cfg!(windows) {
            return;
        }

        init_test(&executor, cx);

        let fs = Arc::new(RealFs::new(None, executor));
        let tree = TempTree::new(json!({
            "project": {},
            "other-project": {},
        }));
        let project: Entity<Project> =
            Project::test(fs, [tree.path().join("project").as_path()], cx).await;

        let check = |input, expected, cx: &mut TestAppContext| {
            let (stream_tx, mut stream_rx) = ToolCallEventStream::test();
            let result = cx.update(|cx| {
                Arc::new(TerminalTool::new(project.clone(), cx)).run(input, stream_tx, cx)
            });
            cx.run_until_parked();
            let event = stream_rx.try_next();
            if let Ok(Some(Ok(ThreadEvent::ToolCallAuthorization(auth)))) = event {
                auth.response.send(auth.options[0].id.clone()).unwrap();
            }

            cx.spawn(async move |_| {
                let output = result.await;
                assert_eq!(output.ok(), expected);
            })
        };

        check(
            TerminalToolInput {
                command: "pwd".into(),
                cd: ".".into(),
            },
            Some(format!(
                "```\n{}\n```",
                tree.path().join("project").display()
            )),
            cx,
        )
        .await;

        check(
            TerminalToolInput {
                command: "pwd".into(),
                cd: "other-project".into(),
            },
            None, // other-project is a dir, but *not* a worktree (yet)
            cx,
        )
        .await;

        // Absolute path above the worktree root
        check(
            TerminalToolInput {
                command: "pwd".into(),
                cd: tree.path().to_string_lossy().into(),
            },
            None,
            cx,
        )
        .await;

        project
            .update(cx, |project, cx| {
                project.create_worktree(tree.path().join("other-project"), true, cx)
            })
            .await
            .unwrap();

        check(
            TerminalToolInput {
                command: "pwd".into(),
                cd: "other-project".into(),
            },
            Some(format!(
                "```\n{}\n```",
                tree.path().join("other-project").display()
            )),
            cx,
        )
        .await;

        check(
            TerminalToolInput {
                command: "pwd".into(),
                cd: ".".into(),
            },
            None,
            cx,
        )
        .await;
    }
}
