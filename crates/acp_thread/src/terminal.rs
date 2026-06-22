use agent_client_protocol::schema::v1 as acp;
#[cfg(target_os = "linux")]
use anyhow::Context as _;
use anyhow::Result;
use collections::HashMap;
use futures::{FutureExt as _, future::Shared};
use gpui::{App, AppContext, AsyncApp, Context, Entity, Task};
use http_proxy::{Allowlist, ProxyConfig, ProxyEvent, ProxyHandle, UpstreamProxy};
use language::LanguageRegistry;
use markdown::Markdown;
use project::Project;
use serde::{Deserialize, Serialize};
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
/// mechanism is platform-specific (macOS Seatbelt; Linux Bubblewrap; a no-op
/// on other platforms), so callers describe the *intent* with plain data here
/// rather than constructing platform-specific types directly.
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
    pub extra_write_paths: Vec<PathBuf>,
    /// Outbound network access explicitly approved for this command.
    pub network: SandboxNetworkAccess,
    /// Allow unrestricted filesystem writes (ignores all writable paths).
    pub allow_fs_write: bool,
    /// Whether the project (and therefore this terminal) is local. The
    /// enforcing proxy binds a loopback port on this host, so it can only
    /// confine local commands; a remote terminal can't reach it.
    pub is_local: bool,
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

impl SandboxNetworkAccess {
    fn restricted_allowlist(&self) -> Option<&Allowlist> {
        match self {
            Self::Restricted(allowlist) => Some(allowlist),
            Self::None | Self::All => None,
        }
    }
}

/// A structured, serializable reason the OS sandbox could not be created for a
/// command. Mirrors the Linux/WSL launcher's failure modes (Bubblewrap);
/// surfaced to the user (and persisted in tool-call metadata) so the UI can
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
    pub fn can_create_sandbox(
        &self,
        cwd: Option<&std::path::Path>,
    ) -> Result<(), LinuxWslSandboxError> {
        #[cfg(target_os = "linux")]
        {
            use sandbox::linux_bubblewrap::LauncherStatus;

            let writable: Vec<&std::path::Path> = self
                .writable_paths
                .iter()
                .chain(self.extra_write_paths.iter())
                .map(|path| path.as_path())
                .collect();
            let allow_network = !matches!(self.network, SandboxNetworkAccess::None);
            let permissions = sandbox::SandboxPermissions {
                allow_network,
                allow_fs_write: self.allow_fs_write,
            };
            sandbox::linux_bubblewrap::check_can_create_sandbox(&writable, permissions, cwd)
                .map_err(|status| match status {
                    LauncherStatus::BwrapNotFound => LinuxWslSandboxError::BwrapNotFound,
                    LauncherStatus::SetuidRejected => LinuxWslSandboxError::SetuidRejected,
                    LauncherStatus::SandboxProbeFailed => LinuxWslSandboxError::SandboxProbeFailed,
                    // `Success` never appears in the `Err` arm; map defensively.
                    LauncherStatus::Success => {
                        LinuxWslSandboxError::Other(status.describe().to_string())
                    }
                })
        }
        #[cfg(not(target_os = "linux"))]
        {
            let _ = cwd;
            Ok(())
        }
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
    /// Unsandboxed execution is permanently allowed via the `allow_unsandboxed`
    /// setting.
    DisabledForever,
    /// The user allowed unsandboxed execution for the rest of this thread after
    /// an earlier sandbox failure. There is always a preceding tool call whose
    /// reason is [`SandboxNotAppliedReason::ErrorLinuxWsl`].
    DisabledForThisThread,
    /// The Linux/WSL (Bubblewrap) sandbox could not be created for this command.
    ErrorLinuxWsl(LinuxWslSandboxError),
}

/// Opaque RAII handle the sandbox implementation hands back to keep its
/// per-command resources (e.g. an on-disk Seatbelt config file) alive for
/// the duration of the spawned command. `Terminal` holds it in a field
/// whose only job is to drop with the entity.
pub type SandboxConfigHandle = Box<dyn std::any::Any + Send>;

/// The outbound-network policy resolved for a sandboxed command.
pub(crate) enum NetworkPolicy {
    /// The command requested no outbound network.
    Denied,
    /// Egress is confined to the in-process proxy on this loopback port.
    Proxied(u16),
    /// The command explicitly requested, and the user approved, unrestricted
    /// outbound network access.
    Unrestricted,
}

/// Apply a [`SandboxWrap`] to a `(program, args)` pair, substituting the
/// platform's sandboxed invocation in place of the original. The returned
/// `SandboxConfigHandle` (when `Some`) must be kept alive for the duration
/// of the spawned command — dropping it deletes any on-disk config the
/// launcher reads at startup.
///
/// `network_policy` is the decision resolved by [`setup_network_proxy`].
/// Unrestricted network access must be requested explicitly via
/// [`SandboxNetworkAccess::All`].
///
/// There is a dedicated code path per platform:
/// * macOS wraps the command with `sandbox-exec` and a Seatbelt config file
///   (returned as the handle).
/// * Linux re-execs this binary as a launcher that locates `bwrap` and `exec`s
///   it for filesystem and network isolation (see
///   [`sandbox::linux_bubblewrap`]); no handle is needed. The launcher reports
///   back over a status channel whether it could enforce the sandbox, and when
///   it can't (no usable `bwrap`, user namespaces disabled, …) it runs the
///   command unsandboxed and the parent logs a warning rather than failing.
/// * Windows routes the command through WSL and runs it under Bubblewrap
///   there, but that path is async (it performs `wsl.exe` round-trips), so it
///   lives in [`apply_windows_wsl_sandbox_wrap`] rather than this synchronous
///   function.
/// * All other platforms pass the command through unchanged — we have no
///   sandbox integration there, so the command runs with the agent's ambient
///   permissions.
#[cfg(not(target_os = "windows"))]
pub(crate) fn apply_sandbox_wrap(
    program: String,
    args: Vec<String>,
    cwd: Option<&std::path::Path>,
    sandbox_wrap: Option<SandboxWrap>,
    network_policy: NetworkPolicy,
) -> anyhow::Result<(String, Vec<String>, Option<SandboxConfigHandle>)> {
    let Some(sandbox_wrap) = sandbox_wrap else {
        return Ok((program, args, None));
    };

    #[cfg(target_os = "macos")]
    {
        use sandbox::macos_seatbelt::NetworkAccess;

        let _ = cwd;
        let writable: Vec<&std::path::Path> = sandbox_wrap
            .writable_paths
            .iter()
            .chain(sandbox_wrap.extra_write_paths.iter())
            .map(|p| p.as_path())
            .collect();
        let network = match network_policy {
            NetworkPolicy::Proxied(port) => NetworkAccess::LocalhostPort(port),
            NetworkPolicy::Unrestricted => NetworkAccess::All,
            NetworkPolicy::Denied => NetworkAccess::None,
        };
        let permissions = sandbox::macos_seatbelt::SandboxPermissions {
            network,
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
        use sandbox::linux_bubblewrap::{self, LauncherStatus, StatusChannel};
        use std::time::Duration;

        let writable: Vec<_> = sandbox_wrap
            .writable_paths
            .iter()
            .chain(sandbox_wrap.extra_write_paths.iter())
            .map(|p| p.as_path())
            .collect();
        let allow_network = match network_policy {
            NetworkPolicy::Denied => false,
            NetworkPolicy::Unrestricted => true,
            NetworkPolicy::Proxied(port) => {
                // Bubblewrap can only toggle network access wholesale, so it
                // can't confine egress to the proxy's loopback port.
                // `setup_network_proxy` never resolves to `Proxied` on Linux;
                // deny network rather than silently widening access.
                log::debug!(
                    "[sandbox/network] ignoring proxy port {port}; bubblewrap can't confine to a loopback port"
                );
                false
            }
        };
        let permissions = sandbox::SandboxPermissions {
            allow_network,
            allow_fs_write: sandbox_wrap.allow_fs_write,
        };

        let launcher = std::env::current_exe()
            .context("failed to resolve current executable for sandbox launcher")?;
        let launcher = launcher.to_str().with_context(|| {
            format!(
                "current executable path contains invalid UTF-8: {}",
                launcher.display()
            )
        })?;

        // Bind a status channel the launcher reports back on, so we can warn
        // when it couldn't actually enforce the sandbox. All the sandbox logic
        // (locating bwrap, probing it) lives in the launcher; the parent only
        // assembles the invocation and listens.
        let channel = StatusChannel::bind().context("failed to set up sandbox status channel")?;
        let (new_program, new_args) = linux_bubblewrap::wrap_invocation(
            launcher,
            Some(channel.name()),
            permissions,
            &writable,
            cwd,
            &program,
            &args,
        );

        // Read the launcher's report in the background, purely for diagnostics.
        // Callers are expected to check `SandboxWrap::can_create_sandbox` before
        // reaching here, so the launcher should almost always succeed; a failure
        // status means the launcher aborted (it never runs a command
        // unsandboxed), so the command did not run.
        const STATUS_TIMEOUT: Duration = Duration::from_secs(30);
        let status_thread = std::thread::Builder::new()
            .name("zed-sandbox-status".into())
            .spawn(move || match channel.recv(STATUS_TIMEOUT) {
                Some(LauncherStatus::Success) => {}
                Some(status) => log::warn!(
                    "sandbox could not be created ({}); the command was aborted",
                    status.describe()
                ),
                None => log::warn!("could not determine terminal command sandbox status"),
            })
            .context("failed to spawn sandbox status thread")?;
        // The thread is self-contained and bounded by STATUS_TIMEOUT; let it run
        // to completion on its own rather than joining here.
        drop(status_thread);

        // The sandbox applies in-process via the re-exec'd launcher, so
        // there's no on-disk resource to keep alive.
        Ok((new_program, new_args, None))
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    {
        // No sandbox integration available; run with ambient permissions.
        if let NetworkPolicy::Proxied(port) = network_policy {
            log::debug!(
                "[sandbox/network] ignoring proxy port {port} because this platform has no sandbox integration"
            );
        }
        let _ = (sandbox_wrap, cwd);
        Ok((program, args, None))
    }
}

/// Upper bound on preparing a WSL-sandboxed command (the probe and path
/// resolution `wsl.exe` round-trips in [`apply_windows_wsl_sandbox_wrap`]).
/// Deliberately generous: the first invocation after the WSL utility VM has
/// shut down (or after boot) has to start the VM and the distro, which
/// routinely takes 10-30 seconds on slow disks or under antivirus scanning.
/// The point is not latency policing but turning a wedged `wsl.exe` (a real
/// failure mode when the WSL service is unhealthy) into an actionable error
/// instead of a terminal command that never starts.
#[cfg(target_os = "windows")]
pub(crate) const WSL_SANDBOX_WRAP_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(60);

/// Wrap a terminal command so it runs under Bubblewrap inside WSL (see
/// [`sandbox::windows_wsl`]).
///
/// Async because it performs `wsl.exe` round-trips and UNC-path stats that
/// can take seconds when the WSL VM is cold; callers must run it on a
/// background executor so the UI thread is never blocked, and should bound
/// it with [`WSL_SANDBOX_WRAP_TIMEOUT`]. Parameters are owned so the future
/// is `Send + 'static`. Dropping the future (timeout or caller cancellation)
/// kills any in-flight `wsl.exe` child rather than leaking it.
///
/// The Windows sandbox (Bubblewrap inside WSL) can only toggle network access
/// wholesale, so `network_policy` collapses to allow/deny here just as it does
/// on Linux. `setup_network_proxy` never resolves to `Proxied` on Windows.
#[cfg(target_os = "windows")]
pub(crate) async fn apply_windows_wsl_sandbox_wrap(
    command: String,
    args: Vec<String>,
    cwd: Option<std::path::PathBuf>,
    sandbox_wrap: SandboxWrap,
    network_policy: NetworkPolicy,
    env: collections::HashMap<String, String>,
) -> anyhow::Result<(String, Vec<String>, Option<SandboxConfigHandle>)> {
    let allow_network = match network_policy {
        NetworkPolicy::Denied => false,
        NetworkPolicy::Unrestricted => true,
        NetworkPolicy::Proxied(port) => {
            // Bubblewrap (in WSL) can only toggle network access wholesale, so
            // it can't confine egress to the proxy's loopback port.
            // `setup_network_proxy` never resolves to `Proxied` on Windows;
            // deny network rather than silently widening access.
            log::debug!(
                "[sandbox/network] ignoring proxy port {port}; bubblewrap in WSL can't confine to a loopback port"
            );
            false
        }
    };
    let (program, args) = task::ShellBuilder::new(&Shell::Program("/bin/sh".to_string()), false)
        .non_interactive()
        .redirect_stdin_to_dev_null()
        .build(Some(command), &args);
    let writable: Vec<std::path::PathBuf> = sandbox_wrap
        .writable_paths
        .into_iter()
        .chain(sandbox_wrap.extra_write_paths)
        .collect();
    let permissions = sandbox::SandboxPermissions {
        allow_network,
        allow_fs_write: sandbox_wrap.allow_fs_write,
    };
    let (program, args) =
        sandbox::windows_wsl::wrap_invocation(program, args, writable, permissions, cwd, env)
            .await?;
    Ok((program, args, None))
}

/// Spawn the in-process network proxy for a sandboxed command with restricted
/// network access, and wire the child's environment to route through it.
///
/// Returns the proxy handle (which must outlive the command) alongside the
/// resolved [`NetworkPolicy`] the sandbox should enforce. The handle is `Some`
/// only when a proxy was actually spawned. Unrestricted network access skips
/// proxy setup and resolves to [`NetworkPolicy::Unrestricted`]. Restricted
/// network access requires a local macOS project so the sandbox can confine
/// egress to the proxy; otherwise this rejects the command instead of widening
/// it.
pub(crate) fn setup_network_proxy(
    sandbox_wrap: Option<&SandboxWrap>,
    env: &mut HashMap<String, String>,
    cx: &mut AsyncApp,
) -> Result<(Option<ProxyHandle>, NetworkPolicy)> {
    let Some(sandbox_wrap) = sandbox_wrap else {
        return Ok((None, NetworkPolicy::Denied));
    };
    let Some(allowlist) = sandbox_wrap.network.restricted_allowlist() else {
        let policy = match &sandbox_wrap.network {
            SandboxNetworkAccess::None => NetworkPolicy::Denied,
            SandboxNetworkAccess::All => NetworkPolicy::Unrestricted,
            SandboxNetworkAccess::Restricted(_) => unreachable!(),
        };
        return Ok((None, policy));
    };

    // The proxy only buys us anything when a Seatbelt sandbox confines the
    // child to its loopback port, and only works for local projects.
    if !cfg!(target_os = "macos") || !sandbox_wrap.is_local {
        anyhow::bail!("restricted network access requested, but no enforcing proxy is available");
    }

    // Chain through the user's real upstream proxy if the command's environment
    // names one. A malformed value shouldn't break the terminal, so log and skip.
    let upstream = match upstream_proxy_from_child_env(env) {
        Ok(upstream) => upstream,
        Err(error) => {
            log::warn!("[sandbox/network] ignoring upstream proxy env: {error:#}");
            None
        }
    };

    let (events_tx, events_rx) = futures::channel::mpsc::unbounded();
    let handle = ProxyHandle::spawn(ProxyConfig {
        allowlist: allowlist.clone(),
        upstream,
        events: events_tx,
    })?;
    let port = handle.port();

    apply_proxy_env(env, port);
    spawn_proxy_event_logger(events_rx, cx);

    Ok((Some(handle), NetworkPolicy::Proxied(port)))
}

fn upstream_proxy_from_child_env(env: &HashMap<String, String>) -> Result<Option<UpstreamProxy>> {
    let url = first_nonempty_env_value(
        env,
        &[
            "HTTPS_PROXY",
            "https_proxy",
            "ALL_PROXY",
            "all_proxy",
            "HTTP_PROXY",
            "http_proxy",
        ],
    );
    let no_proxy = first_nonempty_env_value(env, &["NO_PROXY", "no_proxy"]);
    UpstreamProxy::parse(url, no_proxy)
}

fn first_nonempty_env_value<'a>(
    env: &'a HashMap<String, String>,
    names: &[&str],
) -> Option<&'a str> {
    for name in names {
        if let Some(value) = env.get(*name)
            && !value.trim().is_empty()
        {
            return Some(value.as_str());
        }
    }
    None
}

/// Point the child's proxy env vars at the in-process proxy and strip any
/// inherited `NO_PROXY`.
///
/// Both upper- and lower-case forms are set because some clients (notably
/// curl on macOS) only honor the lowercase variant. `NO_PROXY` is blanked
/// out so all egress goes through our proxy unconditionally: an inherited
/// `NO_PROXY` matching an allowlisted host would make the client attempt a
/// direct connection, which the Seatbelt rule blocks — surfacing as a
/// confusing "connection refused" instead of a clean policy decision.
fn apply_proxy_env(env: &mut HashMap<String, String>, port: u16) {
    let url = format!("http://127.0.0.1:{port}");
    for key in [
        "HTTPS_PROXY",
        "https_proxy",
        "HTTP_PROXY",
        "http_proxy",
        "ALL_PROXY",
        "all_proxy",
    ] {
        env.insert(key.to_string(), url.clone());
    }
    for key in ["NO_PROXY", "no_proxy"] {
        env.insert(key.to_string(), String::new());
    }
}

/// Drain the proxy's event channel, logging each event. v1 surfacing only;
/// future integrations (UI, telemetry) can replace or fan out this consumer.
fn spawn_proxy_event_logger(
    mut events: futures::channel::mpsc::UnboundedReceiver<ProxyEvent>,
    cx: &mut AsyncApp,
) {
    cx.background_spawn(async move {
        use futures::StreamExt as _;
        while let Some(event) = events.next().await {
            log_proxy_event(&event);
        }
    })
    .detach();
}

fn log_proxy_event(event: &ProxyEvent) {
    match event {
        ProxyEvent::Ready { .. } => {}
        ProxyEvent::RequestAttempt {
            host,
            port,
            method,
            outcome,
        } => {
            log::debug!(
                "[sandbox/network] {} {host}:{port} → {outcome:?}",
                method.as_str()
            );
        }
        ProxyEvent::RequestCompleted {
            host,
            port,
            method,
            bytes_to_remote,
            bytes_from_remote,
            duration_ms,
        } => {
            log::debug!(
                "[sandbox/network] completed {} {host}:{port} sent={bytes_to_remote} recv={bytes_from_remote} duration={duration_ms}ms",
                method.as_str(),
            );
        }
    }
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
    /// Seatbelt config kept alive until the sandboxed command exits.
    /// `None` when the command isn't sandboxed or after it finishes.
    _sandbox_config: Option<SandboxConfigHandle>,
    /// In-process network proxy kept alive until the sandboxed command exits.
    _network_proxy: Option<ProxyHandle>,
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
        network_proxy: Option<ProxyHandle>,
        cx: &mut Context<Self>,
    ) -> Self {
        let command_task = terminal.read(cx).wait_for_completed_task(cx);
        Self {
            id,
            _sandbox_config: sandbox_config,
            _network_proxy: network_proxy,
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
                        // Dropping the proxy handle joins its listener thread
                        // (after a loopback wakeup connect); do that off the
                        // foreground thread so a slow/wedged shutdown can't
                        // stall the UI.
                        if let Some(proxy) = this._network_proxy.take() {
                            cx.background_spawn(async move { drop(proxy) }).detach();
                        }
                        this._sandbox_config = None;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_restricted_network_access_uses_proxy_allowlist() {
        assert!(SandboxNetworkAccess::None.restricted_allowlist().is_none());
        assert!(SandboxNetworkAccess::All.restricted_allowlist().is_none());
        assert!(
            SandboxNetworkAccess::Restricted(Allowlist::from_patterns([
                http_proxy::HostPattern::parse("example.com").unwrap()
            ]))
            .restricted_allowlist()
            .is_some()
        );
    }

    #[test]
    fn upstream_proxy_from_child_env_uses_from_env_precedence() {
        let mut env = HashMap::default();
        env.insert("HTTPS_PROXY".to_string(), " ".to_string());
        env.insert("https_proxy".to_string(), "http://lower:1111".to_string());
        env.insert("ALL_PROXY".to_string(), "http://all:2222".to_string());
        env.insert("HTTP_PROXY".to_string(), "http://http:3333".to_string());
        env.insert("NO_PROXY".to_string(), "".to_string());
        env.insert("no_proxy".to_string(), "internal.example".to_string());

        let upstream = upstream_proxy_from_child_env(&env)
            .expect("proxy env should parse")
            .expect("proxy env should configure an upstream");

        assert_eq!(upstream.host, "lower");
        assert_eq!(upstream.port, 1111);
        assert!(upstream.bypasses("internal.example", 443));
        assert!(!upstream.bypasses("zed.dev", 443));
    }

    #[test]
    fn apply_proxy_env_points_all_proxy_vars_at_proxy_and_blanks_no_proxy() {
        let mut env = HashMap::default();
        env.insert("HTTPS_PROXY".to_string(), "http://corp:3128".to_string());
        env.insert("NO_PROXY".to_string(), "internal.example".to_string());
        env.insert("PATH".to_string(), "/usr/bin".to_string());

        apply_proxy_env(&mut env, 54321);

        for key in [
            "HTTPS_PROXY",
            "https_proxy",
            "HTTP_PROXY",
            "http_proxy",
            "ALL_PROXY",
            "all_proxy",
        ] {
            assert_eq!(
                env.get(key).map(String::as_str),
                Some("http://127.0.0.1:54321"),
                "{key} should point at the in-process proxy"
            );
        }
        // An inherited NO_PROXY would make clients attempt direct
        // connections that the Seatbelt rule blocks; it must be blanked.
        for key in ["NO_PROXY", "no_proxy"] {
            assert_eq!(
                env.get(key).map(String::as_str),
                Some(""),
                "{key} should be blanked"
            );
        }
        // Unrelated variables pass through.
        assert_eq!(env.get("PATH").map(String::as_str), Some("/usr/bin"));
    }
}
