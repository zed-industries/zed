use agent_client_protocol::schema::v1 as acp;
use anyhow::Result;
use collections::HashMap;
use futures::{FutureExt as _, future::Shared};
use gpui::{App, AppContext, AsyncApp, Context, Entity, Task};
use http_proxy::Allowlist;
use language::LanguageRegistry;
use markdown::Markdown;
use project::Project;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap as StdHashMap,
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
/// mechanism is platform-specific (macOS Seatbelt; Linux Bubblewrap; Windows
/// via Bubblewrap inside WSL), so callers describe the *intent* with plain data
/// here rather than constructing platform-specific types directly.
///
/// Default is the fully-sandboxed run (no network, project-only writes).
/// Setting `network` / `allow_fs_write` requests a relaxation; the caller is
/// responsible for having obtained user approval before reaching this point.
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
    ///
    /// Each grant carries the canonical target it resolved to at approval
    /// time; enforcement rebuilds the location via a verifying reopen (see
    /// [`granted_write_path_to_location`]) rather than re-resolving the bare
    /// requested path, which closes a symlink TOCTOU.
    pub extra_write_paths: Vec<settings::GrantedWritePath>,
    /// Outbound network access explicitly approved for this command.
    pub network: SandboxNetworkAccess,
    /// Additional paths that should remain readable but not writable, even when
    /// they fall under writable paths.
    pub protected_paths: Vec<PathBuf>,
    /// Allow unrestricted filesystem writes except for protected paths (ignores
    /// ordinary writable paths).
    pub allow_fs_write: bool,
    /// Whether the project (and therefore this terminal) is local. The
    /// enforcing proxy binds a loopback port on this host, so it can only
    /// confine local commands; a remote terminal can't reach it.
    pub is_local: bool,
    /// Windows/WSL only: `(release channel, version)` of the Linux `zed` to
    /// provision inside WSL as the sandbox helper (version `latest` for dev
    /// builds). Resolved by the agent (which can read the running app's release
    /// info) and forwarded to the sandbox. `None` on other platforms, or when
    /// the release can't be determined, in which case the WSL backend falls back
    /// to running bwrap without in-sandbox bind validation.
    pub wsl_zed_release: Option<(String, String)>,
}

#[derive(Clone, Debug, Default)]
pub enum SandboxNetworkAccess {
    /// Block all outbound network access.
    #[default]
    None,
    /// Allow only hosts in this allowlist, enforced by routing HTTP/HTTPS
    /// through an in-process proxy and confining the command to the proxy's
    /// loopback port.
    Restricted(Allowlist),
    /// Allow unrestricted outbound network access.
    All,
}

/// A structured, serializable reason the OS sandbox could not be created for a
/// command. Mirrors the Linux/WSL Bubblewrap failure modes; surfaced to the user
/// (and persisted in tool-call metadata) so the UI can
/// explain what went wrong.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LinuxWslSandboxError {
    /// No usable `bwrap` binary was found on `PATH`.
    BwrapNotFound,
    /// The only `bwrap` found is setuid-root, which Zed refuses to run.
    SetuidRejected,
    /// `bwrap` is present but couldn't set up the sandbox (typically because
    /// unprivileged user namespaces are disabled).
    SandboxProbeFailed,
    /// Any other failure, with a human-readable description.
    Other(String),
}

impl From<sandbox::SandboxError> for LinuxWslSandboxError {
    fn from(error: sandbox::SandboxError) -> Self {
        match error {
            sandbox::SandboxError::BwrapNotFound => Self::BwrapNotFound,
            sandbox::SandboxError::BwrapSetuidRejected => Self::SetuidRejected,
            sandbox::SandboxError::SandboxProbeFailed => Self::SandboxProbeFailed,
            error => Self::Other(error.to_string()),
        }
    }
}

impl LinuxWslSandboxError {
    /// A short, user-facing explanation of why the sandbox couldn't be created,
    /// suitable for display in the agent panel.
    pub fn user_facing_message(&self) -> String {
        match self {
            LinuxWslSandboxError::BwrapNotFound => {
                "No usable `bwrap` binary was found on your PATH. Install Bubblewrap to let \
                 the agent sandbox terminal commands."
                    .to_string()
            }
            LinuxWslSandboxError::SetuidRejected => {
                "The only `bwrap` available is setuid-root, which Zed refuses to run. Install \
                 a non-setuid Bubblewrap to let the agent sandbox terminal commands."
                    .to_string()
            }
            LinuxWslSandboxError::SandboxProbeFailed => {
                "`bwrap` is installed but couldn't create a sandbox, likely because \
                 unprivileged user namespaces are disabled on this system."
                    .to_string()
            }
            LinuxWslSandboxError::Other(message) => message.clone(),
        }
    }

    /// The slug of the sandboxing docs section that best explains how to resolve
    /// this failure, for deep-linking from the UI. Pair with
    /// `client::zed_urls::sandboxing_docs`.
    pub fn docs_section(&self) -> &'static str {
        match self {
            // Both "no bwrap" and "only a setuid-root bwrap" are resolved by
            // installing a non-setuid Bubblewrap.
            LinuxWslSandboxError::BwrapNotFound | LinuxWslSandboxError::SetuidRejected => {
                "installing-bubblewrap"
            }
            // A failed probe on Linux is almost always disabled unprivileged
            // user namespaces, which the Ubuntu-specific section covers.
            LinuxWslSandboxError::SandboxProbeFailed => "installing-bubblewrap-ubuntu",
            // Catch-all (includes WSL/Windows messages): point at the platform
            // overview for the current OS.
            LinuxWslSandboxError::Other(_) => {
                if cfg!(target_os = "windows") {
                    "windows"
                } else {
                    "linux"
                }
            }
        }
    }
}

/// Rebuild a user-approved write grant into an enforceable
/// [`sandbox::HostFilesystemLocation`].
///
/// When the grant carries a resolved canonical (the normal case, established at
/// approval time), the location is rebuilt via a verifying
/// [`sandbox::HostFilesystemLocation::reopen`] — the load-bearing step of the
/// TOCTOU fix. A legacy bare-string grant (no resolved canonical) falls back to
/// a fresh [`sandbox::HostFilesystemLocation::capture`].
pub fn granted_write_path_to_location(
    granted: &settings::GrantedWritePath,
) -> std::io::Result<sandbox::HostFilesystemLocation> {
    match &granted.resolved {
        Some(resolved) => sandbox::HostFilesystemLocation::reopen(&granted.requested, resolved),
        None => sandbox::HostFilesystemLocation::capture(&granted.requested),
    }
}

/// Rebuild a grant for enforcement, or log and drop it (fail-closed) if it
/// can't be verified.
///
/// A failure here is frequently the symlink-TOCTOU defense firing: the grant's
/// canonical was redirected or replaced by a symlink since approval, so
/// [`sandbox::HostFilesystemLocation::reopen`] refuses it. That is a
/// security-relevant event, so it must be logged rather than silently
/// swallowed. The grant is dropped (the command runs without it) rather than
/// bound unverified.
///
/// Only for **display** policies (the sandbox-status UI), where a stale grant
/// should simply not be shown. Enforcement must not drop grants silently — a
/// command would run with less access than the user approved with no signal —
/// so [`SandboxWrap::to_policy`] uses the erroring
/// [`granted_write_path_to_location`] instead.
pub fn granted_write_path_to_location_or_log(
    granted: &settings::GrantedWritePath,
) -> Option<sandbox::HostFilesystemLocation> {
    granted_write_path_to_location(granted)
        .inspect_err(|error| {
            log::warn!(
                "dropping sandbox write grant {}: {error}",
                granted.requested.display()
            );
        })
        .ok()
}

impl SandboxWrap {
    /// Whether the OS sandbox for this request can actually be created right now,
    /// returning a structured [`LinuxWslSandboxError`] when it can't.
    ///
    /// The sandbox implementation never runs a command unsandboxed on its own —
    /// it aborts if it can't create the sandbox. This lets a caller decide, up
    /// front, whether to run sandboxed, fall back to an unsandboxed run
    /// (fail-open), or refuse (fail-closed). It runs a brief probe subprocess on
    /// Linux, so call it off the main thread. On platforms whose sandbox can't
    /// fail to set up this way it always returns `Ok`.
    pub fn can_create_sandbox(&self) -> Result<(), LinuxWslSandboxError> {
        let policy = self
            .to_policy()
            .map_err(|error| LinuxWslSandboxError::Other(format!("{error:#}")))?;
        sandbox::Sandbox::can_create(&policy).map_err(LinuxWslSandboxError::from)
    }

    /// Translate this request into the cross-platform [`sandbox::SandboxPolicy`].
    ///
    /// This is the enforcement-policy construction point, so it **captures** each
    /// grant as a [`sandbox::HostFilesystemLocation`] (pinning the inode / canonical
    /// path) rather than passing a re-resolvable path.
    ///
    /// This function has **no filesystem side effects**: it never creates paths,
    /// and it **fails** (rather than silently narrowing the policy) when a
    /// writable path or approved grant can't be captured — running anyway would
    /// give the command silently less access than the model and user were told
    /// it has. On Linux a writable grant that doesn't exist can't be captured
    /// (bwrap can't bind a missing path); the sanctioned way to get a grant to a
    /// new directory is the `create_directory` tool, which creates it (pinning
    /// the inode) before the grant is recorded. On macOS a missing leaf still
    /// canonicalizes, so such grants are captured directly.
    ///
    /// It is used both by the side-effect-free [`Self::can_create_sandbox`] probe
    /// and by real sandbox construction, and must behave identically.
    ///
    /// A grant failure here can also be the symlink-TOCTOU defense firing: the
    /// grant's canonical was redirected or replaced by a symlink since approval,
    /// so the verifying reopen refuses it. Failing the command surfaces that
    /// security-relevant event instead of running with the grant quietly
    /// missing.
    ///
    /// Protected paths, by contrast, are **best-effort**: we protect only the
    /// ones that exist at creation time (`capture` succeeding *is* the existence
    /// check), and silently drop the rest. Unlike a writable grant, a protection
    /// can't be materialized — you can't pin the inode of a path that isn't
    /// there — and there is an inherent, *accepted* loophole regardless: a
    /// command in a non-git directory can `git init` and write hooks into a
    /// `.git` that didn't exist when the sandbox was built. Since the protection
    /// is defeatable that way no matter what, failing sandbox creation over a
    /// currently-absent (or otherwise uncapturable) `.git` would only break
    /// legitimate cases — non-git projects, single-file worktrees whose
    /// synthesized `settings.json/.git` routes through a file — without closing
    /// the hole. So we drop and move on.
    fn to_policy(&self) -> Result<sandbox::SandboxPolicy> {
        let protected_paths = self
            .protected_paths
            .iter()
            .filter_map(|path| sandbox::HostFilesystemLocation::capture(path).ok())
            .collect::<Vec<_>>();
        let fs = if self.allow_fs_write {
            sandbox::SandboxFsPolicy::Unrestricted { protected_paths }
        } else {
            // Project worktree paths are captured fresh; user-approved grants are
            // rebuilt via the verifying reopen (or captured when legacy bare
            // strings) through `granted_write_path_to_location`. A path that
            // can't be captured fails the whole construction (never created).
            let mut locations = Vec::new();
            for path in &self.writable_paths {
                let location = sandbox::HostFilesystemLocation::capture(path).map_err(|error| {
                    anyhow::anyhow!(error).context(format!(
                        "cannot capture writable sandbox path `{}`",
                        path.display()
                    ))
                })?;
                locations.push(location);
            }
            for granted in &self.extra_write_paths {
                let location = granted_write_path_to_location(granted).map_err(|error| {
                    anyhow::anyhow!(error).context(format!(
                        "cannot re-verify approved sandbox write grant `{}` (if the \
                         directory was removed, remove the grant or recreate the \
                         directory)",
                        granted.requested.display()
                    ))
                })?;
                locations.push(location);
            }
            // Dedupe to a minimal cover on the captured canonical paths, so a
            // grant nested under a worktree root (or another grant) is dropped
            // rather than bound redundantly.
            let writable_paths =
                sandbox::normalize_host_filesystem_locations(locations.into_iter());
            sandbox::SandboxFsPolicy::Restricted {
                writable_paths,
                protected_paths,
            }
        };
        let network = match &self.network {
            SandboxNetworkAccess::None => sandbox::SandboxNetPolicy::Blocked,
            SandboxNetworkAccess::All => sandbox::SandboxNetPolicy::Unrestricted,
            SandboxNetworkAccess::Restricted(allowlist) => sandbox::SandboxNetPolicy::Restricted {
                allowed_domains: allowlist
                    .patterns()
                    .iter()
                    .map(|pattern| pattern.to_string())
                    .collect(),
            },
        };
        Ok(sandbox::SandboxPolicy { fs, network })
    }
}

/// Why the OS sandbox was *not* applied to a terminal command, even though
/// sandboxing is active for the thread. Persisted in tool-call metadata so the
/// UI can explain the situation after the fact.
///
/// This is deliberately platform-agnostic — every variant exists on every
/// platform — so the serialized form stored in the thread database never
/// depends on which OS wrote it. Today only Linux/WSL can fail to create a
/// sandbox (`ErrorLinuxWsl`), but the variant is named so macOS/Windows can
/// grow their own failure cases later without a migration.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SandboxNotAppliedReason {
    /// The user disabled the sandbox for the rest of this thread, so the command
    /// ran without one. This happens either when the user approved a
    /// model-requested `unsandboxed: true` escape "for this thread", or when
    /// they chose to run unsandboxed for the thread after a sandbox-creation
    /// failure (in which case a preceding tool call's reason is
    /// [`SandboxNotAppliedReason::ErrorLinuxWsl`]).
    DisabledForThisThread,
    /// The Linux/WSL (Bubblewrap) sandbox could not be created for this command.
    ErrorLinuxWsl(LinuxWslSandboxError),
}

/// The live sandbox kept alive for its per-command resources (the network proxy
/// and, on macOS, the Seatbelt policy file) until the terminal exits.
type SandboxConfigHandle = sandbox::Sandbox;

/// Upper bound on preparing a WSL-sandboxed command. Deliberately generous:
/// the first invocation after the WSL utility VM has shut down (or after boot)
/// has to start the VM and the distro, which routinely takes 10-30 seconds on
/// slow disks or under antivirus scanning.
#[cfg(target_os = "windows")]
pub(crate) const WSL_SANDBOX_WRAP_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(60);

/// Wrap `(program, args)` for sandboxed execution, returning the wrapped
/// invocation (program, argv, env) plus the live [`sandbox::Sandbox`] that must
/// be kept alive for the command's duration. When `sandbox_wrap` is `None` the
/// command is returned unchanged.
///
/// The sandbox owns the network proxy (for restricted-network policies) and any
/// per-command policy file; the env it returns already routes through that
/// proxy when applicable.
pub(crate) async fn prepare_sandbox_wrap(
    program: String,
    args: Vec<String>,
    cwd: Option<PathBuf>,
    sandbox_wrap: Option<SandboxWrap>,
    env: HashMap<String, String>,
) -> anyhow::Result<(
    String,
    Vec<String>,
    HashMap<String, String>,
    Option<SandboxConfigHandle>,
)> {
    let Some(sandbox_wrap) = sandbox_wrap else {
        return Ok((program, args, env, None));
    };

    let mut sandbox =
        sandbox::Sandbox::new(sandbox_wrap.to_policy()?).map_err(anyhow::Error::new)?;
    // Windows/WSL only: tell the sandbox which Linux `zed` to provision inside
    // WSL as its `--wsl-sandbox-helper`. A no-op (and a no-op setter) elsewhere.
    #[cfg(target_os = "windows")]
    if let Some((channel, version)) = sandbox_wrap.wsl_zed_release.clone() {
        sandbox.set_wsl_zed_release(channel, version);
    }
    let command = sandbox::CommandAndArgs {
        program,
        args,
        env: env.into_iter().collect::<StdHashMap<_, _>>(),
        cwd,
    };
    let wrapped = sandbox.wrap(&command).await.map_err(anyhow::Error::new)?;
    Ok((
        wrapped.program,
        wrapped.args,
        wrapped.env.into_iter().collect(),
        Some(sandbox),
    ))
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
    /// The live sandbox (Seatbelt policy file and/or network proxy) kept alive
    /// until the sandboxed command exits. `None` when the command isn't
    /// sandboxed or after it finishes. Dropping it tears down the proxy on a
    /// background thread (see `sandbox::Sandbox`'s `Drop`).
    _sandbox: Option<SandboxConfigHandle>,
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
        sandbox: Option<SandboxConfigHandle>,
        cx: &mut Context<Self>,
    ) -> Self {
        let command_task = terminal.read(cx).wait_for_completed_task(cx);
        // Tear the sandbox down on a GPUI background thread when this entity is
        // released, rather than relying on `Sandbox`'s `Drop` (which would spawn
        // a throwaway thread) on whatever thread releases us. `on_release` hands
        // us an `App`, so we can drive the teardown through the background
        // executor with `drop_on_current_thread`.
        cx.on_release(|this, cx| {
            if let Some(sandbox) = this._sandbox.take() {
                cx.background_executor()
                    .spawn(async move { sandbox.drop_on_current_thread() })
                    .detach();
            }
        })
        .detach();
        Self {
            id,
            _sandbox: sandbox,
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
                        // Free the sandbox (and its network proxy) as soon as
                        // the command finishes, rather than holding it until
                        // this entity is released. The proxy's teardown joins a
                        // listener thread, so run it on the background executor
                        // to keep it off the foreground thread.
                        if let Some(sandbox) = this._sandbox.take() {
                            cx.background_executor()
                                .spawn(async move { sandbox.drop_on_current_thread() })
                                .detach();
                        }
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

#[cfg(all(test, target_os = "linux"))]
mod linux_tests {
    use super::*;

    /// Regression test for the bug where enforcement-policy construction
    /// *created* missing write grants — famously turning a granted
    /// `~/.config/zed/AGENTS.md` file path into a directory. A grant whose
    /// target no longer exists must fail policy construction with an error
    /// naming it, and nothing may be created — a required safety grant that
    /// can't be honored must stop the command, not silently shrink its access.
    #[test]
    fn to_policy_fails_on_missing_grant_and_never_creates_it() {
        let temp_dir = tempfile::tempdir().expect("create temp dir");
        let missing = temp_dir.path().join("AGENTS.md");

        let wrap = SandboxWrap {
            extra_write_paths: vec![settings::GrantedWritePath::resolved(
                missing.clone(),
                missing.clone(),
            )],
            ..Default::default()
        };

        let error = wrap
            .to_policy()
            .expect_err("a grant to a missing path must fail policy construction");
        assert!(
            format!("{error:#}").contains("AGENTS.md"),
            "error should name the failing grant: {error:#}"
        );
        assert!(
            !missing.exists(),
            "policy construction must never create the granted path"
        );
    }

    /// A baseline writable path (worktree root / scratch dir) that doesn't
    /// exist must also fail: silently narrowing the sandbox would hand the
    /// command less access than the model was told it has.
    #[test]
    fn to_policy_fails_on_missing_writable_path() {
        let temp_dir = tempfile::tempdir().expect("create temp dir");
        let missing = temp_dir.path().join("gone");

        let wrap = SandboxWrap {
            writable_paths: vec![missing.clone()],
            ..Default::default()
        };

        wrap.to_policy()
            .expect_err("a missing writable path must fail policy construction");
        assert!(
            !missing.exists(),
            "policy construction must never create a writable path"
        );
    }

    /// Protected paths are best-effort: an uncapturable one is dropped, never
    /// fatal. That covers a missing path (`NotFound`) and one routed through a
    /// regular file (`NotADirectory`) — the latter is the synthesized `.git` of
    /// a single-file worktree (e.g. `settings.json/.git`). Unlike a writable
    /// grant, a protection can't be materialized, and `.git` protection has an
    /// inherent accepted loophole (`git init`), so failing here would only break
    /// legitimate cases. Unit-level companion to the `settings.json/.git` NixOS
    /// check.
    #[test]
    fn to_policy_skips_uncapturable_protected_paths() {
        let temp_dir = tempfile::tempdir().expect("create temp dir");
        let writable = temp_dir.path().join("writable");
        std::fs::create_dir(&writable).expect("create writable dir");
        let single_file_root = temp_dir.path().join("settings.json");
        std::fs::write(&single_file_root, b"{}").expect("create single-file worktree root");

        let wrap = SandboxWrap {
            writable_paths: vec![writable],
            protected_paths: vec![
                temp_dir.path().join("no-such-.git"),
                single_file_root.join(".git"),
            ],
            ..Default::default()
        };

        wrap.to_policy()
            .expect("uncapturable protected paths must be dropped, not fail the policy");
    }
}
