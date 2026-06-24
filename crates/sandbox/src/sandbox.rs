//! Cross-platform sandboxing for commands run on behalf of the agent.
//!
//! The public API is intentionally platform-neutral. Internally, macOS uses
//! Seatbelt (`sandbox-exec`), Linux uses Bubblewrap (`bwrap`), and Windows uses
//! Bubblewrap inside WSL. Restricted-network policies are enforced by an
//! in-process HTTP/HTTPS proxy (the `http_proxy` crate) that this crate
//! constructs and owns; callers only describe intent via [`SandboxPolicy`].

use std::{
    collections::HashMap,
    fmt,
    path::{Path, PathBuf},
    process::Output,
};

use http_proxy::ProxyHandle;
#[cfg(not(target_os = "windows"))]
use http_proxy::{Allowlist, HostPattern, ProxyConfig, ProxyEvent, UpstreamProxy};

#[cfg(target_os = "linux")]
mod linux_bubblewrap;

#[cfg(target_os = "macos")]
mod macos_seatbelt;

#[cfg(target_os = "windows")]
mod windows_wsl;

#[cfg(target_os = "windows")]
pub(crate) const WSL_SANDBOX_UNAVAILABLE_PREFIX: &str = "Windows sandboxing via WSL is unavailable";

/// What a command is allowed to do, expressed as intent. This is the entire
/// public configuration surface; how each policy is enforced (Seatbelt rules,
/// Bubblewrap flags, a loopback proxy, …) is an implementation detail.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SandboxPolicy {
    pub fs: SandboxFsPolicy,
    pub network: SandboxNetPolicy,
    pub git: GitSandboxPolicy,
}

/// Filesystem policy for a sandboxed command.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SandboxFsPolicy {
    /// Allow unrestricted filesystem writes.
    Unrestricted,
    /// Reads are allowed everywhere; writes are confined to these directory
    /// subtrees (and the standard ephemeral locations the platform provides).
    Restricted { writable_paths: Vec<PathBuf> },
}

/// Outbound-network policy for a sandboxed command.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SandboxNetPolicy {
    /// Allow unrestricted outbound network access.
    Unrestricted,
    /// Block all outbound network access.
    Blocked,
    /// Allow outbound HTTP(S) only to these hostnames (exact hosts or
    /// leading-`*.` subdomain wildcards), enforced by an in-process proxy.
    Restricted { allowed_domains: Vec<String> },
}

/// Policy for the project's Git directories (`.git`). The agent computes the
/// directory list because locating it requires Git knowledge the sandbox layer
/// can't derive itself — a linked worktree's `.git` is a gitlink pointing at a
/// common dir elsewhere, and discovered repositories can live outside the
/// project. Both variants carry the same dirs; the variant selects how they're
/// treated.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GitSandboxPolicy {
    /// `.git` contents are protected: read-only on Linux; content reads and
    /// writes denied on macOS (metadata stays visible either way).
    Denied { git_dirs: Vec<PathBuf> },
    /// `.git` contents are writable (these dirs are made writable).
    Allowed { git_dirs: Vec<PathBuf> },
}

impl Default for GitSandboxPolicy {
    /// Git directories protected, none known — the safe default.
    fn default() -> Self {
        GitSandboxPolicy::Denied {
            git_dirs: Vec::new(),
        }
    }
}

impl GitSandboxPolicy {
    /// The `.git` directories this policy governs, regardless of variant.
    pub fn git_dirs(&self) -> &[PathBuf] {
        match self {
            GitSandboxPolicy::Denied { git_dirs } | GitSandboxPolicy::Allowed { git_dirs } => {
                git_dirs
            }
        }
    }

    /// Whether `.git` contents are writable.
    pub fn allows_writes(&self) -> bool {
        matches!(self, GitSandboxPolicy::Allowed { .. })
    }

    /// Combine two layers: `Allowed` (more permissive) wins, and the governed
    /// `.git` directories union.
    pub fn merge(self, other: GitSandboxPolicy) -> GitSandboxPolicy {
        let allowed = self.allows_writes() || other.allows_writes();
        let (GitSandboxPolicy::Denied { mut git_dirs }
        | GitSandboxPolicy::Allowed { mut git_dirs }) = self;
        let (GitSandboxPolicy::Denied {
            git_dirs: other_dirs,
        }
        | GitSandboxPolicy::Allowed {
            git_dirs: other_dirs,
        }) = other;
        for path in other_dirs {
            if !git_dirs.contains(&path) {
                git_dirs.push(path);
            }
        }
        if allowed {
            GitSandboxPolicy::Allowed { git_dirs }
        } else {
            GitSandboxPolicy::Denied { git_dirs }
        }
    }
}

impl SandboxPolicy {
    /// Combine two policies into the least-restrictive policy that satisfies
    /// both (a set union per dimension). Used to layer the user's persistent
    /// settings, this thread's grants, and a command's request into the single
    /// policy that is actually enforced.
    pub fn merge(self, other: SandboxPolicy) -> SandboxPolicy {
        SandboxPolicy {
            fs: self.fs.merge(other.fs),
            network: self.network.merge(other.network),
            git: self.git.merge(other.git),
        }
    }

    /// Replace the Git policy, keeping the fs/network policy. The UI builds the
    /// fs/network halves from settings/grants (which don't know the project's
    /// `.git` locations) and then attaches the agent-computed Git policy here.
    pub fn with_git(mut self, git: GitSandboxPolicy) -> Self {
        self.git = git;
        self
    }
}

impl SandboxFsPolicy {
    /// Unrestricted access dominates; otherwise the writable subtrees union.
    pub fn merge(self, other: SandboxFsPolicy) -> SandboxFsPolicy {
        match (self, other) {
            (SandboxFsPolicy::Unrestricted, _) | (_, SandboxFsPolicy::Unrestricted) => {
                SandboxFsPolicy::Unrestricted
            }
            (
                SandboxFsPolicy::Restricted {
                    writable_paths: mut a,
                },
                SandboxFsPolicy::Restricted { writable_paths: b },
            ) => {
                for path in b {
                    if !a.contains(&path) {
                        a.push(path);
                    }
                }
                SandboxFsPolicy::Restricted { writable_paths: a }
            }
        }
    }
}

impl SandboxNetPolicy {
    /// Unrestricted access dominates and `Blocked` is the identity; otherwise the
    /// allowed domains union.
    pub fn merge(self, other: SandboxNetPolicy) -> SandboxNetPolicy {
        match (self, other) {
            (SandboxNetPolicy::Unrestricted, _) | (_, SandboxNetPolicy::Unrestricted) => {
                SandboxNetPolicy::Unrestricted
            }
            (SandboxNetPolicy::Blocked, other) | (other, SandboxNetPolicy::Blocked) => other,
            (
                SandboxNetPolicy::Restricted {
                    allowed_domains: mut a,
                },
                SandboxNetPolicy::Restricted { allowed_domains: b },
            ) => {
                for domain in b {
                    if !a.contains(&domain) {
                        a.push(domain);
                    }
                }
                SandboxNetPolicy::Restricted { allowed_domains: a }
            }
        }
    }
}

/// A command and its execution environment, before sandbox wrapping.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CommandAndArgs {
    /// Program to execute.
    pub program: String,
    /// Arguments passed to `program`.
    pub args: Vec<String>,
    /// Environment variables for the spawned process.
    pub env: HashMap<String, String>,
    /// Working directory for the spawned process.
    pub cwd: Option<PathBuf>,
}

/// A command transformed to run inside the platform sandbox. Plain data: spawn
/// it however you like (PTY, `std::process`, …). The resources that must
/// outlive the spawned process live in the [`Sandbox`] that produced this, not
/// here — keep the `Sandbox` alive for the command's duration.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WrappedCommand {
    pub program: String,
    pub args: Vec<String>,
    pub env: HashMap<String, String>,
    pub cwd: Option<PathBuf>,
}

/// Errors returned by the sandbox abstraction.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SandboxError {
    /// No sandbox implementation is available for this platform.
    UnsupportedPlatform,
    /// No usable Bubblewrap executable was found.
    BwrapNotFound,
    /// The only Bubblewrap executable found is setuid-root, which is refused.
    BwrapSetuidRejected,
    /// Bubblewrap was found but failed to create the sandbox.
    SandboxProbeFailed,
    /// The sandbox bridge executable path could not be resolved.
    BridgeExecutableUnavailable(String),
    /// Windows sandboxing through WSL is unavailable.
    WslUnavailable(String),
    /// The requested sandbox policy is not supported on this platform.
    UnsupportedPolicy(String),
    /// The sandbox request is invalid (e.g. a malformed allowed-domain).
    InvalidRequest(String),
    /// An I/O error occurred (e.g. spawning the network proxy).
    Io(String),
    /// Any other sandbox setup failure.
    Other(String),
}

impl fmt::Display for SandboxError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SandboxError::UnsupportedPlatform => {
                write!(formatter, "sandboxing is not supported on this platform")
            }
            SandboxError::BwrapNotFound => {
                write!(formatter, "no usable `bwrap` binary was found on PATH")
            }
            SandboxError::BwrapSetuidRejected => write!(
                formatter,
                "the only available `bwrap` is setuid-root, which Zed refuses to run"
            ),
            SandboxError::SandboxProbeFailed => {
                write!(
                    formatter,
                    "`bwrap` is present but failed to create a sandbox"
                )
            }
            SandboxError::BridgeExecutableUnavailable(message) => write!(
                formatter,
                "failed to resolve sandbox bridge executable: {message}"
            ),
            SandboxError::WslUnavailable(message) => write!(formatter, "{message}"),
            SandboxError::UnsupportedPolicy(message) => write!(formatter, "{message}"),
            SandboxError::InvalidRequest(message) => write!(formatter, "{message}"),
            SandboxError::Io(message) => write!(formatter, "{message}"),
            SandboxError::Other(message) => write!(formatter, "{message}"),
        }
    }
}

impl std::error::Error for SandboxError {}

/// Resolved filesystem setup derived from [`SandboxFsPolicy`].
struct FsSetup {
    allow_fs_write: bool,
    writable_paths: Vec<PathBuf>,
}

/// Resolved network plan derived from [`SandboxNetPolicy`]. For the restricted
/// case the allowlist is parsed up front; the proxy itself is spawned lazily on
/// the first [`Sandbox::wrap`] so its upstream-proxy chaining can read the
/// command's environment.
enum NetSetup {
    Unrestricted,
    Blocked,
    // Restricted networking is rejected up front on Windows, so this variant is
    // never constructed there.
    #[cfg(not(target_os = "windows"))]
    Restricted {
        allowlist: Allowlist,
    },
}

/// A live sandbox: it owns the per-policy resources (the network proxy, and on
/// macOS the temporary Seatbelt policy file) and produces sandboxed command
/// invocations via [`Sandbox::wrap`] / [`Sandbox::execute`].
///
/// Keep it alive for as long as commands wrapped by it are running; dropping it
/// tears down the proxy.
pub struct Sandbox {
    fs: FsSetup,
    network: NetSetup,
    /// The project's `.git` directories and whether they're writable. Enforced
    /// on macOS/Linux; ignored by the WSL sandbox for now.
    git: GitSandboxPolicy,
    /// In-process network proxy for the restricted-network case, spawned on the
    /// first `wrap`. Dropped on a background thread (the join blocks).
    proxy: Option<ProxyHandle>,
    #[cfg(target_os = "macos")]
    seatbelt_config: Option<macos_seatbelt::SeatbeltConfigFile>,
}

impl Sandbox {
    /// Create a sandbox for `policy`, validating it for the current platform.
    ///
    /// This does not spawn the network proxy or probe `bwrap`; the proxy is
    /// created lazily on the first [`Sandbox::wrap`] (so it can chain through an
    /// upstream proxy named in the command's environment), and `bwrap`
    /// availability is checked separately via [`Sandbox::can_create`].
    pub fn new(policy: SandboxPolicy) -> Result<Self, SandboxError> {
        let fs = match policy.fs {
            SandboxFsPolicy::Unrestricted => FsSetup {
                allow_fs_write: true,
                writable_paths: Vec::new(),
            },
            SandboxFsPolicy::Restricted { writable_paths } => FsSetup {
                allow_fs_write: false,
                writable_paths,
            },
        };

        let network = match policy.network {
            SandboxNetPolicy::Unrestricted => NetSetup::Unrestricted,
            SandboxNetPolicy::Blocked => NetSetup::Blocked,
            SandboxNetPolicy::Restricted { allowed_domains } => {
                resolve_restricted_network(&allowed_domains)?
            }
        };

        Ok(Self {
            fs,
            network,
            git: policy.git,
            proxy: None,
            #[cfg(target_os = "macos")]
            seatbelt_config: None,
        })
    }

    /// Check whether the platform sandbox can be created for `policy` without
    /// actually building a command or spawning the proxy. On Linux this runs a
    /// brief `bwrap` probe (call it off the main thread).
    pub fn can_create(policy: &SandboxPolicy, cwd: Option<&Path>) -> Result<(), SandboxError> {
        #[cfg(target_os = "linux")]
        {
            let writable = policy_writable_paths(policy);
            let writable: Vec<&Path> = writable.iter().map(PathBuf::as_path).collect();
            let permissions = linux_bubblewrap::SandboxPermissions {
                network: linux_probe_network(&policy.network),
                allow_fs_write: matches!(policy.fs, SandboxFsPolicy::Unrestricted),
            };
            linux_bubblewrap::check_can_create_sandbox(&writable, permissions, cwd)
                .map_err(map_linux_status)
        }
        #[cfg(target_os = "windows")]
        {
            let _ = cwd;
            if matches!(policy.network, SandboxNetPolicy::Restricted { .. }) {
                return Err(unsupported_restricted_network_on_windows());
            }
            Ok(())
        }
        #[cfg(not(any(target_os = "linux", target_os = "windows")))]
        {
            let _ = (policy, cwd);
            Ok(())
        }
    }

    /// Transform `command` into the invocation that runs it inside this sandbox.
    /// The returned [`WrappedCommand`] is plain data; keep `self` alive while it
    /// runs (it owns the proxy and any per-command policy file).
    pub async fn wrap(&mut self, command: &CommandAndArgs) -> Result<WrappedCommand, SandboxError> {
        #[cfg(target_os = "linux")]
        {
            self.wrap_linux(command)
        }
        #[cfg(target_os = "macos")]
        {
            self.wrap_macos(command)
        }
        #[cfg(target_os = "windows")]
        {
            self.wrap_windows(command).await
        }
        #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
        {
            let _ = command;
            Err(SandboxError::UnsupportedPlatform)
        }
    }

    /// Run `command` inside the sandbox to completion and collect its output.
    /// Convenience for non-interactive callers; interactive callers (PTYs) use
    /// [`Sandbox::wrap`].
    #[allow(
        clippy::disallowed_methods,
        reason = "this is the blocking convenience API; interactive callers use `wrap`"
    )]
    pub async fn execute(&mut self, command: &CommandAndArgs) -> Result<Output, SandboxError> {
        let wrapped = self.wrap(command).await?;
        let mut process = std::process::Command::new(&wrapped.program);
        process.args(&wrapped.args).envs(&wrapped.env);
        if let Some(cwd) = &wrapped.cwd {
            process.current_dir(cwd);
        }
        process
            .output()
            .map_err(|error| SandboxError::Io(error.to_string()))
    }

    /// Drop this sandbox on the *current* thread, tearing down the network proxy
    /// inline (its `Drop` joins a listener thread after a loopback wakeup
    /// connect).
    ///
    /// The blanket [`Drop`] impl instead offloads that join to a fresh thread,
    /// so an accidental drop on a latency-sensitive thread (e.g. the UI thread)
    /// can never block. Call this only when you are *already* on a background
    /// thread or executor and want the teardown to finish before the surrounding
    /// task completes — it avoids spawning a throwaway thread to do work the
    /// current one can do.
    pub fn drop_on_current_thread(mut self) {
        // Drop the proxy here, synchronously, so the `Drop` impl below sees
        // `None` and doesn't spawn a thread to repeat the work.
        drop(self.proxy.take());
    }

    /// Spawn the restricted-network proxy if it isn't running yet, point the
    /// command env at it, and return its `(port, host socket path)`. Returns
    /// `None` for non-restricted network policies.
    #[cfg(not(target_os = "windows"))]
    fn ensure_restricted_proxy(
        &mut self,
        env: &mut HashMap<String, String>,
    ) -> Result<Option<(u16, Option<PathBuf>)>, SandboxError> {
        let NetSetup::Restricted { allowlist } = &self.network else {
            return Ok(None);
        };

        if self.proxy.is_none() {
            let upstream = upstream_proxy_from_env(env);
            let (events_tx, events_rx) = futures::channel::mpsc::unbounded();
            let config = ProxyConfig {
                allowlist: allowlist.clone(),
                upstream,
                events: events_tx,
            };
            #[cfg(target_os = "linux")]
            let handle = ProxyHandle::spawn_unix_temp(config);
            #[cfg(not(target_os = "linux"))]
            let handle = ProxyHandle::spawn(config);
            let handle = handle.map_err(|error| {
                SandboxError::Io(format!("failed to start network proxy: {error:#}"))
            })?;
            spawn_proxy_event_logger(events_rx);
            self.proxy = Some(handle);
        }

        let proxy = self
            .proxy
            .as_ref()
            .expect("proxy was just ensured to be present");
        let port = proxy.port();
        apply_proxy_env(env, port);
        Ok(Some((port, proxy.socket_path().map(PathBuf::from))))
    }

    /// Split the policy's `.git` directories into (writable, protected) sets for
    /// the enforcement layers. When the whole filesystem is writable the split
    /// is empty (Git protection is moot); otherwise `Allowed` git dirs are
    /// writable and `Denied` git dirs are protected.
    #[cfg(any(target_os = "macos", target_os = "linux"))]
    fn git_path_split(&self) -> (Vec<PathBuf>, Vec<PathBuf>) {
        if self.fs.allow_fs_write {
            return (Vec::new(), Vec::new());
        }
        match &self.git {
            GitSandboxPolicy::Allowed { git_dirs } => (git_dirs.clone(), Vec::new()),
            GitSandboxPolicy::Denied { git_dirs } => (Vec::new(), git_dirs.clone()),
        }
    }

    #[cfg(target_os = "linux")]
    fn wrap_linux(&mut self, command: &CommandAndArgs) -> Result<WrappedCommand, SandboxError> {
        let mut env = command.env.clone();
        let proxy = self.ensure_restricted_proxy(&mut env)?;

        let network = match &self.network {
            NetSetup::Unrestricted => linux_bubblewrap::NetworkAccess::All,
            NetSetup::Blocked => linux_bubblewrap::NetworkAccess::None,
            NetSetup::Restricted { .. } => linux_bubblewrap::NetworkAccess::LocalhostPort(
                proxy.as_ref().map(|(port, _)| *port).unwrap_or(0),
            ),
        };
        let proxy_socket_path = proxy.as_ref().and_then(|(_, socket)| socket.clone());

        let permissions = linux_bubblewrap::SandboxPermissions {
            network,
            allow_fs_write: self.fs.allow_fs_write,
        };
        let (git_writable, git_protected) = self.git_path_split();
        let mut writable: Vec<&Path> = self
            .fs
            .writable_paths
            .iter()
            .map(PathBuf::as_path)
            .collect();
        writable.extend(git_writable.iter().map(PathBuf::as_path));
        let protected_git_dirs: Vec<&Path> = git_protected.iter().map(PathBuf::as_path).collect();

        let bridge_program = std::env::current_exe()
            .map_err(|error| SandboxError::BridgeExecutableUnavailable(error.to_string()))?;
        let bridge_program = bridge_program.to_str().ok_or_else(|| {
            SandboxError::BridgeExecutableUnavailable(format!(
                "current executable path contains invalid UTF-8: {}",
                bridge_program.display()
            ))
        })?;

        let (program, args) = linux_bubblewrap::wrap_invocation(
            bridge_program,
            permissions,
            &writable,
            &protected_git_dirs,
            command.cwd.as_deref(),
            &command.program,
            &command.args,
            proxy_socket_path.as_deref(),
        )
        .map_err(map_anyhow_error)?;

        Ok(WrappedCommand {
            program,
            args,
            env,
            cwd: command.cwd.clone(),
        })
    }

    #[cfg(target_os = "macos")]
    fn wrap_macos(&mut self, command: &CommandAndArgs) -> Result<WrappedCommand, SandboxError> {
        let mut env = command.env.clone();
        let proxy = self.ensure_restricted_proxy(&mut env)?;

        let network = match &self.network {
            NetSetup::Unrestricted => macos_seatbelt::NetworkAccess::All,
            NetSetup::Blocked => macos_seatbelt::NetworkAccess::None,
            NetSetup::Restricted { .. } => macos_seatbelt::NetworkAccess::LocalhostPort(
                proxy.as_ref().map(|(port, _)| *port).unwrap_or(0),
            ),
        };

        let permissions = macos_seatbelt::SandboxPermissions {
            network,
            allow_fs_write: self.fs.allow_fs_write,
        };
        let (git_writable, git_protected) = self.git_path_split();
        let mut writable: Vec<&Path> = self
            .fs
            .writable_paths
            .iter()
            .map(PathBuf::as_path)
            .collect();
        writable.extend(git_writable.iter().map(PathBuf::as_path));
        let protected: Vec<&Path> = git_protected.iter().map(PathBuf::as_path).collect();

        // SSH-agent socket handling (commit signing) is deferred, so no unix
        // sockets are allowed for now.
        let (program, args, config) = macos_seatbelt::wrap_invocation(
            &command.program,
            &command.args,
            &writable,
            &protected,
            &[],
            permissions,
        )
        .map_err(map_anyhow_error)?;
        // Keep the temporary Seatbelt policy file alive for the command's life.
        self.seatbelt_config = Some(config);

        Ok(WrappedCommand {
            program,
            args,
            env,
            cwd: command.cwd.clone(),
        })
    }

    #[cfg(target_os = "windows")]
    async fn wrap_windows(
        &mut self,
        command: &CommandAndArgs,
    ) -> Result<WrappedCommand, SandboxError> {
        // Restricted host network access is rejected at `new` time on Windows.
        // Git-dir protection isn't enforced for the WSL sandbox yet, so the
        // policy's Git directories are intentionally ignored here.
        let _ = self.git.git_dirs();
        let permissions = windows_wsl::SandboxPermissions {
            allow_network: matches!(self.network, NetSetup::Unrestricted),
            allow_fs_write: self.fs.allow_fs_write,
        };
        let (program, args) = windows_wsl::wrap_invocation(
            command.program.clone(),
            command.args.clone(),
            self.fs.writable_paths.clone(),
            permissions,
            command.cwd.clone(),
            command.env.clone(),
        )
        .await
        .map_err(map_anyhow_error)?;

        Ok(WrappedCommand {
            program,
            args,
            env: command.env.clone(),
            cwd: command.cwd.clone(),
        })
    }
}

impl Drop for Sandbox {
    fn drop(&mut self) {
        // Dropping a `ProxyHandle` joins its listener thread after a loopback
        // wakeup connect; do that off whatever (possibly UI) thread is dropping
        // the sandbox so a slow shutdown can't stall it. Callers already on a
        // background executor should prefer `drop_on_current_thread` to avoid
        // this throwaway thread.
        if let Some(proxy) = self.proxy.take() {
            std::thread::spawn(move || drop(proxy));
        }
    }
}

/// Handle a possible re-exec of this binary as an in-sandbox helper.
///
/// Linux restricted-network runs launch this binary in bridge mode inside the
/// sandbox network namespace before spawning the real command. Call this at the
/// top of `main`, before normal argument parsing.
#[doc(hidden)]
pub fn run_sandbox_launcher_if_invoked() {
    #[cfg(target_os = "linux")]
    linux_bubblewrap::run_launcher_if_invoked();
}

#[cfg(target_os = "linux")]
fn policy_writable_paths(policy: &SandboxPolicy) -> Vec<PathBuf> {
    match &policy.fs {
        SandboxFsPolicy::Unrestricted => Vec::new(),
        SandboxFsPolicy::Restricted { writable_paths } => writable_paths.clone(),
    }
}

#[cfg(not(target_os = "windows"))]
fn resolve_restricted_network(allowed_domains: &[String]) -> Result<NetSetup, SandboxError> {
    let mut patterns = Vec::with_capacity(allowed_domains.len());
    for domain in allowed_domains {
        let pattern = HostPattern::parse(domain).map_err(|error| {
            SandboxError::InvalidRequest(format!("invalid network host '{domain}': {error}"))
        })?;
        patterns.push(pattern);
    }
    Ok(NetSetup::Restricted {
        allowlist: Allowlist::from_patterns(patterns),
    })
}

#[cfg(target_os = "windows")]
fn resolve_restricted_network(_allowed_domains: &[String]) -> Result<NetSetup, SandboxError> {
    Err(unsupported_restricted_network_on_windows())
}

#[cfg(target_os = "windows")]
fn unsupported_restricted_network_on_windows() -> SandboxError {
    SandboxError::UnsupportedPolicy(
        "restricted host network access is not yet supported for Windows sandboxes".to_string(),
    )
}

#[cfg(target_os = "linux")]
fn linux_probe_network(network: &SandboxNetPolicy) -> linux_bubblewrap::NetworkAccess {
    match network {
        SandboxNetPolicy::Unrestricted => linux_bubblewrap::NetworkAccess::All,
        SandboxNetPolicy::Blocked => linux_bubblewrap::NetworkAccess::None,
        // The probe only needs the network namespace flag, not a real port.
        SandboxNetPolicy::Restricted { .. } => linux_bubblewrap::NetworkAccess::LocalhostPort(0),
    }
}

#[cfg(not(target_os = "windows"))]
fn upstream_proxy_from_env(env: &HashMap<String, String>) -> Option<UpstreamProxy> {
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
    match UpstreamProxy::parse(url, no_proxy) {
        Ok(upstream) => upstream,
        Err(error) => {
            log::warn!("[sandbox/network] ignoring upstream proxy env: {error:#}");
            None
        }
    }
}

#[cfg(not(target_os = "windows"))]
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
/// Both upper- and lower-case forms are set because some clients (notably curl
/// on macOS) only honor the lowercase variant. `NO_PROXY` is blanked so all
/// egress goes through our proxy unconditionally: an inherited `NO_PROXY`
/// matching an allowlisted host would make the client attempt a direct
/// connection, which the sandbox blocks — surfacing as a confusing "connection
/// refused" instead of a clean policy decision.
#[cfg(not(target_os = "windows"))]
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

/// Drain the proxy's event channel on a background thread, logging each event.
/// v1 surfacing only; future integrations (UI, telemetry) can replace this.
/// The thread exits when the proxy is dropped and the channel closes.
#[cfg(not(target_os = "windows"))]
fn spawn_proxy_event_logger(events: futures::channel::mpsc::UnboundedReceiver<ProxyEvent>) {
    std::thread::spawn(move || {
        futures::executor::block_on(async move {
            use futures::StreamExt as _;
            let mut events = events;
            while let Some(event) = events.next().await {
                log_proxy_event(&event);
            }
        });
    });
}

#[cfg(not(target_os = "windows"))]
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

#[cfg(target_os = "linux")]
fn map_linux_status(status: linux_bubblewrap::LauncherStatus) -> SandboxError {
    match status {
        linux_bubblewrap::LauncherStatus::BwrapNotFound => SandboxError::BwrapNotFound,
        linux_bubblewrap::LauncherStatus::SetuidRejected => SandboxError::BwrapSetuidRejected,
        linux_bubblewrap::LauncherStatus::SandboxProbeFailed => SandboxError::SandboxProbeFailed,
    }
}

#[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
fn map_anyhow_error(error: anyhow::Error) -> SandboxError {
    #[cfg(target_os = "windows")]
    if let Some(error) = error.downcast_ref::<windows_wsl::WslSandboxUnavailable>() {
        return SandboxError::WslUnavailable(error.to_string());
    }

    SandboxError::Other(format!("{error:#}"))
}

#[cfg(all(test, not(target_os = "windows")))]
mod tests {
    use super::*;

    #[test]
    fn fs_merge_unrestricted_dominates_else_unions_paths() {
        let a = SandboxFsPolicy::Restricted {
            writable_paths: vec![PathBuf::from("/a"), PathBuf::from("/b")],
        };
        let b = SandboxFsPolicy::Restricted {
            writable_paths: vec![PathBuf::from("/b"), PathBuf::from("/c")],
        };
        assert_eq!(
            a.clone().merge(b),
            SandboxFsPolicy::Restricted {
                writable_paths: vec![
                    PathBuf::from("/a"),
                    PathBuf::from("/b"),
                    PathBuf::from("/c")
                ],
            }
        );
        assert_eq!(
            a.merge(SandboxFsPolicy::Unrestricted),
            SandboxFsPolicy::Unrestricted
        );
        assert_eq!(
            SandboxFsPolicy::Unrestricted.merge(SandboxFsPolicy::Restricted {
                writable_paths: vec![PathBuf::from("/a")],
            }),
            SandboxFsPolicy::Unrestricted
        );
    }

    #[test]
    fn net_merge_unrestricted_dominates_blocked_is_identity_else_unions() {
        let hosts = |list: &[&str]| SandboxNetPolicy::Restricted {
            allowed_domains: list.iter().map(|s| s.to_string()).collect(),
        };
        assert_eq!(
            hosts(&["a.com", "b.com"]).merge(hosts(&["b.com", "c.com"])),
            hosts(&["a.com", "b.com", "c.com"])
        );
        assert_eq!(
            SandboxNetPolicy::Blocked.merge(hosts(&["a.com"])),
            hosts(&["a.com"])
        );
        assert_eq!(
            hosts(&["a.com"]).merge(SandboxNetPolicy::Blocked),
            hosts(&["a.com"])
        );
        assert_eq!(
            SandboxNetPolicy::Blocked.merge(SandboxNetPolicy::Blocked),
            SandboxNetPolicy::Blocked
        );
        assert_eq!(
            hosts(&["a.com"]).merge(SandboxNetPolicy::Unrestricted),
            SandboxNetPolicy::Unrestricted
        );
    }

    #[test]
    fn upstream_proxy_from_env_uses_precedence_and_no_proxy() {
        let mut env = HashMap::new();
        env.insert("HTTPS_PROXY".to_string(), " ".to_string());
        env.insert("https_proxy".to_string(), "http://lower:1111".to_string());
        env.insert("ALL_PROXY".to_string(), "http://all:2222".to_string());
        env.insert("HTTP_PROXY".to_string(), "http://http:3333".to_string());
        env.insert("NO_PROXY".to_string(), "".to_string());
        env.insert("no_proxy".to_string(), "internal.example".to_string());

        let upstream = upstream_proxy_from_env(&env).expect("should configure an upstream");
        assert_eq!(upstream.host, "lower");
        assert_eq!(upstream.port, 1111);
        assert!(upstream.bypasses("internal.example", 443));
        assert!(!upstream.bypasses("zed.dev", 443));
    }

    #[test]
    fn apply_proxy_env_points_vars_at_proxy_and_blanks_no_proxy() {
        let mut env = HashMap::new();
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
                Some("http://127.0.0.1:54321")
            );
        }
        for key in ["NO_PROXY", "no_proxy"] {
            assert_eq!(env.get(key).map(String::as_str), Some(""));
        }
        assert_eq!(env.get("PATH").map(String::as_str), Some("/usr/bin"));
    }
}

/// Canonicalize `path`, resolving symlinks, even when its final component
/// doesn't exist yet.
///
/// `std::fs::canonicalize` fails if any component is missing, which would leave
/// a not-yet-created path (e.g. a `.git` directory before `git init`) in a
/// non-canonical form. The sandbox layers canonicalize the writable parent
/// (the worktree root) but, with a plain `canonicalize`, fall back to the raw
/// path for a missing child; the two then disagree when a component is a
/// symlink (`/tmp` -> `/private/tmp` on macOS), and the protection rule for the
/// child misses the real path the command ends up writing. Canonicalizing the
/// existing parent and re-appending the final component keeps the child
/// consistent with its parent. If neither the path nor its parent can be
/// canonicalized, the path is returned unchanged.
//
// Only the macOS Seatbelt layer uses this (Linux skips not-yet-existing `.git`
// dirs rather than emitting a rule for them), so it's gated to macOS to avoid a
// dead-code warning elsewhere.
#[cfg(target_os = "macos")]
pub(crate) fn canonicalize_allowing_missing_leaf(path: &std::path::Path) -> std::path::PathBuf {
    if let Ok(canonical) = path.canonicalize() {
        return canonical;
    }
    if let (Some(parent), Some(file_name)) = (path.parent(), path.file_name())
        && let Ok(canonical_parent) = parent.canonicalize()
    {
        return canonical_parent.join(file_name);
    }
    path.to_path_buf()
}

#[cfg(all(test, target_os = "macos"))]
mod macos_tests {
    use super::canonicalize_allowing_missing_leaf;

    #[test]
    fn canonicalize_allowing_missing_leaf_resolves_existing_parent() {
        let dir = tempfile::tempdir().unwrap();
        let canonical_dir = dir.path().canonicalize().unwrap();

        // A fully existing path is canonicalized outright.
        assert_eq!(
            canonicalize_allowing_missing_leaf(dir.path()),
            canonical_dir
        );

        // A path whose leaf doesn't exist yet still resolves through its parent,
        // so it stays consistent with how the parent directory canonicalizes
        // (this is the `.git`-before-`git init` case).
        let missing = dir.path().join("not-created-yet");
        assert_eq!(
            canonicalize_allowing_missing_leaf(&missing),
            canonical_dir.join("not-created-yet"),
        );

        // A path whose parent also doesn't exist is returned unchanged.
        let deeper = missing.join(".git");
        assert_eq!(canonicalize_allowing_missing_leaf(&deeper), deeper);
    }
}
