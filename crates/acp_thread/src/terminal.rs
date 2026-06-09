use agent_client_protocol::schema as acp;
#[cfg(target_os = "linux")]
use anyhow::Context as _;
use anyhow::Result;
use futures::{FutureExt as _, future::Shared};
use gpui::{App, AppContext, AsyncApp, Context, Entity, Task};
use language::LanguageRegistry;
use markdown::Markdown;
use project::Project;
use std::{
    path::PathBuf,
    process::ExitStatus,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Instant,
};
use task::Shell;
use util::get_default_system_shell_preferring_bash;

/// Request to run a terminal command inside an OS-level sandbox.
///
/// Passed to [`super::AcpThread::create_terminal`]. The actual sandboxing
/// mechanism is platform-specific (macOS Seatbelt; Linux Bubblewrap +
/// seccomp; a no-op on other platforms), so callers describe the *intent*
/// with plain data here rather than constructing platform-specific types
/// directly.
///
/// All-zero defaults are the fully-sandboxed run. Setting `allow_network` /
/// `allow_fs_write` requests a relaxation; the caller is responsible for
/// having obtained user approval before reaching this point.
#[derive(Clone, Debug, Default)]
pub struct SandboxWrap {
    /// Directory subtrees the sandbox should allow writes to. Pass the
    /// project's worktree paths (and any per-command scratch directory)
    /// here — *not* the command's working directory, which is model-
    /// controlled and would let the model widen its own writable scope.
    pub writable_paths: Vec<PathBuf>,
    /// Additional write subtrees the user explicitly approved for this
    /// command (per-path write grants). Kept separate from `writable_paths`
    /// to make the trust boundary explicit: these originate from
    /// model-requested paths that passed a user-approval prompt. They are
    /// merged with `writable_paths` when generating the sandbox policy.
    pub extra_write_paths: Vec<PathBuf>,
    /// Allow outbound network access for this command.
    pub allow_network: bool,
    /// Allow unrestricted filesystem writes (ignores all writable paths).
    pub allow_fs_write: bool,
}

/// Opaque RAII handle the sandbox implementation hands back to keep its
/// per-command resources (e.g. an on-disk Seatbelt config file) alive for
/// the duration of the spawned command. `Terminal` holds it in a field
/// whose only job is to drop with the entity.
pub type SandboxConfigHandle = Box<dyn std::any::Any + Send>;

/// Apply a [`SandboxWrap`] to a `(program, args)` pair, substituting the
/// platform's sandboxed invocation in place of the original. The returned
/// `SandboxConfigHandle` (when `Some`) must be kept alive for the duration
/// of the spawned command — dropping it deletes any on-disk config the
/// launcher reads at startup.
///
/// There is a dedicated code path per platform:
/// * macOS wraps the command with `sandbox-exec` and a Seatbelt config file
///   (returned as the handle).
/// * Linux re-execs this binary as a launcher that installs a seccomp network
///   policy and `exec`s `bwrap` for filesystem isolation (see
///   [`sandbox::linux_bubblewrap`]); no handle is needed. When no usable
///   `bwrap` is available the command runs unsandboxed (with a logged
///   warning) rather than failing.
/// * Windows and all other platforms pass the command through unchanged —
///   we have no sandbox integration there, so the command runs with the
///   agent's ambient permissions.
pub(crate) fn apply_sandbox_wrap(
    program: String,
    args: Vec<String>,
    cwd: Option<&std::path::Path>,
    sandbox_wrap: Option<SandboxWrap>,
) -> anyhow::Result<(String, Vec<String>, Option<SandboxConfigHandle>)> {
    let Some(sandbox_wrap) = sandbox_wrap else {
        return Ok((program, args, None));
    };

    #[cfg(target_os = "macos")]
    {
        let _ = cwd;
        let writable: Vec<&std::path::Path> = sandbox_wrap
            .writable_paths
            .iter()
            .chain(sandbox_wrap.extra_write_paths.iter())
            .map(|p| p.as_path())
            .collect();
        let permissions = sandbox::SandboxPermissions {
            allow_network: sandbox_wrap.allow_network,
            allow_fs_write: sandbox_wrap.allow_fs_write,
        };
        let (new_program, new_args, config_file) =
            sandbox::macos_seatbelt::wrap_invocation(&program, &args, &writable, permissions)?;
        Ok((
            new_program,
            new_args,
            Some(Box::new(config_file) as SandboxConfigHandle),
        ))
    }
    #[cfg(target_os = "linux")]
    {
        use sandbox::linux_bubblewrap;

        // Decide once whether this environment can actually enforce a sandbox.
        // When it can't (no usable bwrap, or unprivileged user namespaces are
        // unavailable), run the command unsandboxed rather than failing.
        // TODO: surface this to the user via the UI instead of only logging.
        let Some(bwrap) =
            linux_bubblewrap::locate_bwrap().filter(|_| linux_bubblewrap::is_available())
        else {
            log::warn!(
                "no usable bwrap sandbox on this system; running terminal command \
                 without an OS sandbox"
            );
            return Ok((program, args, None));
        };
        let bwrap = bwrap
            .to_str()
            .with_context(|| format!("bwrap path contains invalid UTF-8: {}", bwrap.display()))?;

        let writable: Vec<_> = sandbox_wrap
            .writable_paths
            .iter()
            .chain(sandbox_wrap.extra_write_paths.iter())
            .map(|p| p.as_path())
            .collect();
        let permissions = sandbox::SandboxPermissions {
            allow_network: sandbox_wrap.allow_network,
            allow_fs_write: sandbox_wrap.allow_fs_write,
        };

        // Assemble the full command to run inside the sandbox:
        // `bwrap <bwrap-args> -- <program> <args>`. The launcher (re-exec'd
        // Zed) installs the seccomp policy and then `exec`s this verbatim.
        let bwrap_args = linux_bubblewrap::build_bwrap_args(&writable, permissions, cwd);
        let mut command = Vec::with_capacity(bwrap_args.len() + args.len() + 3);
        command.push(bwrap.to_string());
        command.extend(bwrap_args);
        command.push("--".to_string());
        command.push(program);
        command.extend(args);

        let launcher = std::env::current_exe()
            .context("failed to resolve current executable for sandbox launcher")?;
        let launcher = launcher.to_str().with_context(|| {
            format!(
                "current executable path contains invalid UTF-8: {}",
                launcher.display()
            )
        })?;
        let network_policy = linux_bubblewrap::network_policy_for(permissions);
        let (new_program, new_args) =
            linux_bubblewrap::wrap_invocation(launcher, network_policy, &command);
        // The sandbox applies in-process via the re-exec'd launcher, so
        // there's no on-disk resource to keep alive.
        Ok((new_program, new_args, None))
    }
    #[cfg(target_os = "windows")]
    {
        // Windows sandboxing is handled before shell expansion in
        // `create_terminal`; by this point `program` may be PowerShell or cmd,
        // which cannot be executed inside WSL's Linux sandbox.
        let _ = (sandbox_wrap, cwd);
        Ok((program, args, None))
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    {
        // No sandbox integration available; run with ambient permissions.
        let _ = (sandbox_wrap, cwd);
        Ok((program, args, None))
    }
}

#[cfg(target_os = "windows")]
pub(crate) fn apply_windows_wsl_sandbox_wrap(
    command: String,
    args: &[String],
    cwd: Option<&std::path::Path>,
    sandbox_wrap: SandboxWrap,
) -> anyhow::Result<(String, Vec<String>, Option<SandboxConfigHandle>)> {
    let (program, args) = task::ShellBuilder::new(&Shell::Program("/bin/sh".to_string()), false)
        .redirect_stdin_to_dev_null()
        .build(Some(command), args);
    let writable: Vec<_> = sandbox_wrap
        .writable_paths
        .iter()
        .chain(sandbox_wrap.extra_write_paths.iter())
        .map(|path| path.as_path())
        .collect();
    let permissions = sandbox::SandboxPermissions {
        allow_network: sandbox_wrap.allow_network,
        allow_fs_write: sandbox_wrap.allow_fs_write,
    };
    let (program, args) =
        sandbox::windows_wsl::wrap_invocation(&program, &args, &writable, permissions, cwd)?;
    Ok((program, args, None))
}

pub struct Terminal {
    id: acp::TerminalId,
    command: Entity<Markdown>,
    working_dir: Option<PathBuf>,
    terminal: Entity<terminal::Terminal>,
    started_at: Instant,
    output: Option<TerminalOutput>,
    output_byte_limit: Option<usize>,
    _output_task: Shared<Task<acp::TerminalExitStatus>>,
    /// Flag indicating whether this terminal was stopped by explicit user action
    /// (e.g., clicking the Stop button). This is set before kill() is called
    /// so that code awaiting wait_for_exit() can check it deterministically.
    user_stopped: Arc<AtomicBool>,
    /// RAII handle kept alive for the duration of the sandboxed command.
    /// `None` when the command isn't sandboxed (the common case for
    /// terminals not created by the agent).
    _sandbox_config: Option<SandboxConfigHandle>,
}

pub struct TerminalOutput {
    pub ended_at: Instant,
    pub exit_status: Option<ExitStatus>,
    pub content: String,
    pub original_content_len: usize,
    pub content_line_count: usize,
}

impl Terminal {
    pub fn new(
        id: acp::TerminalId,
        command_label: &str,
        working_dir: Option<PathBuf>,
        output_byte_limit: Option<usize>,
        terminal: Entity<terminal::Terminal>,
        language_registry: Arc<LanguageRegistry>,
        sandbox_config: Option<SandboxConfigHandle>,
        cx: &mut Context<Self>,
    ) -> Self {
        let command_task = terminal.read(cx).wait_for_completed_task(cx);
        Self {
            id,
            _sandbox_config: sandbox_config,
            command: cx.new(|cx| {
                Markdown::new(
                    format!("```\n{}\n```", command_label).into(),
                    Some(language_registry.clone()),
                    None,
                    cx,
                )
            }),
            working_dir,
            terminal,
            started_at: Instant::now(),
            output: None,
            output_byte_limit,
            user_stopped: Arc::new(AtomicBool::new(false)),
            _output_task: cx
                .spawn(async move |this, cx| {
                    let exit_status = command_task.await;

                    this.update(cx, |this, cx| {
                        let (content, original_content_len) = this.truncated_output(cx);
                        let content_line_count = this.terminal.read(cx).total_lines();

                        this.output = Some(TerminalOutput {
                            ended_at: Instant::now(),
                            exit_status,
                            content,
                            original_content_len,
                            content_line_count,
                        });
                        cx.notify();
                    })
                    .ok();

                    let exit_status = exit_status.map(portable_pty::ExitStatus::from);

                    acp::TerminalExitStatus::new()
                        .exit_code(exit_status.as_ref().map(|e| e.exit_code()))
                        .signal(exit_status.and_then(|e| e.signal().map(ToOwned::to_owned)))
                })
                .shared(),
        }
    }

    pub fn id(&self) -> &acp::TerminalId {
        &self.id
    }

    pub fn wait_for_exit(&self) -> Shared<Task<acp::TerminalExitStatus>> {
        self._output_task.clone()
    }

    pub fn kill(&mut self, cx: &mut App) {
        self.terminal.update(cx, |terminal, _cx| {
            terminal.kill_active_task();
        });
    }

    /// Marks this terminal as stopped by user action and then kills it.
    /// This should be called when the user explicitly clicks a Stop button.
    pub fn stop_by_user(&mut self, cx: &mut App) {
        self.user_stopped.store(true, Ordering::SeqCst);
        self.kill(cx);
    }

    /// Returns whether this terminal was stopped by explicit user action.
    pub fn was_stopped_by_user(&self) -> bool {
        self.user_stopped.load(Ordering::SeqCst)
    }

    pub fn current_output(&self, cx: &App) -> acp::TerminalOutputResponse {
        if let Some(output) = self.output.as_ref() {
            let exit_status = output.exit_status.map(portable_pty::ExitStatus::from);

            acp::TerminalOutputResponse::new(
                output.content.clone(),
                output.original_content_len > output.content.len(),
            )
            .exit_status(
                acp::TerminalExitStatus::new()
                    .exit_code(exit_status.as_ref().map(|e| e.exit_code()))
                    .signal(exit_status.and_then(|e| e.signal().map(ToOwned::to_owned))),
            )
        } else {
            let (current_content, original_len) = self.truncated_output(cx);
            let truncated = current_content.len() < original_len;
            acp::TerminalOutputResponse::new(current_content, truncated)
        }
    }

    fn truncated_output(&self, cx: &App) -> (String, usize) {
        let terminal = self.terminal.read(cx);
        let mut content = terminal.get_content();

        let original_content_len = content.len();

        if let Some(limit) = self.output_byte_limit
            && content.len() > limit
        {
            let mut end_ix = limit.min(content.len());
            while !content.is_char_boundary(end_ix) {
                end_ix -= 1;
            }
            // Don't truncate mid-line, clear the remainder of the last line
            end_ix = content[..end_ix].rfind('\n').unwrap_or(end_ix);
            content.truncate(end_ix);
        }

        (content, original_content_len)
    }

    pub fn command(&self) -> &Entity<Markdown> {
        &self.command
    }

    pub fn update_command_label(&self, label: &str, cx: &mut App) {
        self.command.update(cx, |command, cx| {
            command.replace(format!("```\n{}\n```", label), cx);
        });
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

pub async fn create_terminal_entity(
    command: String,
    args: &[String],
    env_vars: Vec<(String, String)>,
    cwd: Option<PathBuf>,
    project: &Entity<Project>,
    cx: &mut AsyncApp,
) -> Result<Entity<terminal::Terminal>> {
    let mut env = if let Some(dir) = &cwd {
        project
            .update(cx, |project, cx| {
                project.environment().update(cx, |env, cx| {
                    env.directory_environment(dir.clone().into(), cx)
                })
            })
            .await
            .unwrap_or_default()
    } else {
        Default::default()
    };

    // Disable pagers so agent/terminal commands don't hang behind interactive UIs
    env.insert("PAGER".into(), "".into());
    // Override user core.pager (e.g. delta) which Git prefers over PAGER
    env.insert("GIT_PAGER".into(), "cat".into());
    env.extend(env_vars);

    // Use remote shell or default system shell, as appropriate
    let shell = project
        .update(cx, |project, cx| {
            project
                .remote_client()
                .and_then(|r| r.read(cx).default_system_shell())
                .map(Shell::Program)
        })
        .unwrap_or_else(|| Shell::Program(get_default_system_shell_preferring_bash()));
    let is_windows = project.read_with(cx, |project, cx| project.path_style(cx).is_windows());
    let (task_command, task_args) = task::ShellBuilder::new(&shell, is_windows)
        .redirect_stdin_to_dev_null()
        .build(Some(command.clone()), &args);

    project
        .update(cx, |project, cx| {
            project.create_terminal_task(
                task::SpawnInTerminal {
                    command: Some(task_command),
                    args: task_args,
                    cwd,
                    env,
                    ..Default::default()
                },
                cx,
            )
        })
        .await
}
