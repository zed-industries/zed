//! Cross-platform sandboxing for commands run on behalf of the agent.
//!
//! The public API is intentionally platform-neutral. Internally, macOS uses
//! Seatbelt (`sandbox-exec`), Linux uses Bubblewrap (`bwrap`), and Windows uses
//! Bubblewrap inside WSL.

use std::{
    collections::HashMap,
    fmt,
    path::{Path, PathBuf},
    process::{Command, Output},
};

#[cfg(target_os = "linux")]
mod linux_bubblewrap;

#[cfg(target_os = "macos")]
mod macos_seatbelt;

#[cfg(target_os = "windows")]
mod windows_wsl;

#[cfg(target_os = "windows")]
pub(crate) const WSL_SANDBOX_UNAVAILABLE_PREFIX: &str = "Windows sandboxing via WSL is unavailable";

/// Configuration for running a command inside the platform sandbox.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SandboxConfiguration {
    /// Directory subtrees where writes are allowed when
    /// [`SandboxPermissions::allow_fs_write`] is false.
    pub writable_paths: Vec<PathBuf>,
    /// Per-command sandbox relaxations.
    pub permissions: SandboxPermissions,
}

impl SandboxConfiguration {
    /// Check whether the platform sandbox can be created for this configuration.
    ///
    /// This performs the same availability probe used when preparing a command,
    /// without requiring a real command or proxy socket. On platforms where the
    /// current sandbox implementation cannot be probed synchronously, this is a
    /// best-effort check.
    pub fn can_create_sandbox(&self, cwd: Option<&Path>) -> Result<(), SandboxError> {
        can_create_sandbox(self, cwd)
    }
}

/// Per-command relaxations of the default sandbox.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SandboxPermissions {
    /// Outbound network policy.
    pub network: NetworkAccess,
    /// Allow unrestricted filesystem writes.
    pub allow_fs_write: bool,
}

/// Network policy for a sandboxed command.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum NetworkAccess {
    /// Block outbound network access.
    #[default]
    None,
    /// Allow outbound HTTP(S) via a loopback proxy port.
    ///
    /// Linux additionally needs the host-side Unix socket path backing that
    /// proxy so Bubblewrap can bind it into the sandbox for the in-namespace
    /// bridge. macOS ignores `proxy_socket_path`.
    LocalhostPort {
        port: u16,
        proxy_socket_path: Option<PathBuf>,
    },
    /// Allow unrestricted outbound network access.
    All,
}

/// A command and its execution environment.
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

/// A command transformed to run inside the platform sandbox.
///
/// Keep this value alive for the duration of the spawned process. On platforms
/// that need per-command resources, such as macOS's temporary Seatbelt policy
/// file, those resources are owned privately by this value.
#[derive(Debug)]
pub struct PreparedSandboxCommand {
    /// Program to execute.
    pub program: String,
    /// Arguments passed to `program`.
    pub args: Vec<String>,
    /// Environment variables for the spawned process.
    pub env: HashMap<String, String>,
    /// Working directory for the spawned process.
    pub cwd: Option<PathBuf>,
    #[cfg(target_os = "macos")]
    _handle: Option<macos_seatbelt::SeatbeltConfigFile>,
}

impl PreparedSandboxCommand {
    fn new(
        program: String,
        args: Vec<String>,
        env: HashMap<String, String>,
        cwd: Option<PathBuf>,
    ) -> Self {
        Self {
            program,
            args,
            env,
            cwd,
            #[cfg(target_os = "macos")]
            _handle: None,
        }
    }

    #[cfg(target_os = "macos")]
    fn with_macos_handle(mut self, handle: macos_seatbelt::SeatbeltConfigFile) -> Self {
        self._handle = Some(handle);
        self
    }
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
    /// Restricted Linux networking requires a host-side Unix proxy socket path.
    RestrictedNetworkRequiresProxySocket,
    /// The sandbox bridge executable path could not be resolved.
    BridgeExecutableUnavailable(String),
    /// Windows sandboxing through WSL is unavailable.
    WslUnavailable(String),
    /// The requested sandbox policy is not supported on this platform.
    UnsupportedPolicy(String),
    /// The sandbox request is invalid for this platform or command.
    InvalidRequest(String),
    /// An I/O error occurred.
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
            SandboxError::SandboxProbeFailed => write!(
                formatter,
                "`bwrap` is present but failed to create a sandbox"
            ),
            SandboxError::RestrictedNetworkRequiresProxySocket => write!(
                formatter,
                "restricted Linux network access requires a proxy Unix socket path"
            ),
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

/// Prepare a command for sandboxed execution.
///
/// The returned value contains the program, argv, environment, and cwd to pass
/// to the actual process/terminal spawner. Keep the returned value alive until
/// the spawned process exits.
pub async fn prepare_sandboxed_command(
    sandbox: &SandboxConfiguration,
    command: &CommandAndArgs,
) -> Result<PreparedSandboxCommand, SandboxError> {
    #[cfg(target_os = "windows")]
    {
        prepare_sandboxed_command_windows(sandbox, command).await
    }
    #[cfg(not(target_os = "windows"))]
    {
        prepare_sandboxed_command_sync(sandbox, command)
    }
}

/// Run a command inside the platform sandbox and collect its output.
#[allow(
    clippy::disallowed_methods,
    reason = "this is the sandbox crate's blocking convenience API; interactive callers use prepare_sandboxed_command"
)]
pub fn run_sandboxed(
    sandbox: &SandboxConfiguration,
    command: &CommandAndArgs,
) -> Result<Output, SandboxError> {
    let prepared = prepare_sandboxed_command_for_output(sandbox, command)?;
    let mut child = Command::new(&prepared.program);
    child.args(&prepared.args).envs(&prepared.env);
    if let Some(cwd) = &prepared.cwd {
        child.current_dir(cwd);
    }
    child
        .output()
        .map_err(|error| SandboxError::Io(error.to_string()))
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

fn can_create_sandbox(
    sandbox: &SandboxConfiguration,
    cwd: Option<&Path>,
) -> Result<(), SandboxError> {
    #[cfg(target_os = "linux")]
    {
        let writable_paths = writable_path_refs(sandbox);
        let permissions = linux_permissions(&sandbox.permissions);
        return linux_bubblewrap::check_can_create_sandbox(&writable_paths, permissions, cwd)
            .map_err(map_linux_status);
    }

    #[cfg(target_os = "windows")]
    {
        let _ = cwd;
        if matches!(
            sandbox.permissions.network,
            NetworkAccess::LocalhostPort { .. }
        ) {
            return Err(SandboxError::UnsupportedPolicy(
                "restricted host network access is not yet supported for Windows sandboxes"
                    .to_string(),
            ));
        }
        Ok(())
    }

    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    {
        let _ = (sandbox, cwd);
        Ok(())
    }
}

#[cfg(not(target_os = "windows"))]
fn prepare_sandboxed_command_for_output(
    sandbox: &SandboxConfiguration,
    command: &CommandAndArgs,
) -> Result<PreparedSandboxCommand, SandboxError> {
    prepare_sandboxed_command_sync(sandbox, command)
}

#[cfg(target_os = "windows")]
fn prepare_sandboxed_command_for_output(
    _sandbox: &SandboxConfiguration,
    _command: &CommandAndArgs,
) -> Result<PreparedSandboxCommand, SandboxError> {
    Err(SandboxError::UnsupportedPlatform)
}

#[cfg(target_os = "linux")]
fn prepare_sandboxed_command_sync(
    sandbox: &SandboxConfiguration,
    command: &CommandAndArgs,
) -> Result<PreparedSandboxCommand, SandboxError> {
    let permissions = linux_permissions(&sandbox.permissions);
    if matches!(
        permissions.network,
        linux_bubblewrap::NetworkAccess::LocalhostPort(_)
    ) && proxy_socket_path(&sandbox.permissions.network).is_none()
    {
        return Err(SandboxError::RestrictedNetworkRequiresProxySocket);
    }

    can_create_sandbox(sandbox, command.cwd.as_deref())?;

    let bridge_program = std::env::current_exe()
        .map_err(|error| SandboxError::BridgeExecutableUnavailable(error.to_string()))?;
    let bridge_program = bridge_program.to_str().ok_or_else(|| {
        SandboxError::BridgeExecutableUnavailable(format!(
            "current executable path contains invalid UTF-8: {}",
            bridge_program.display()
        ))
    })?;
    let writable_paths = writable_path_refs(sandbox);
    let (program, args) = linux_bubblewrap::wrap_invocation(
        bridge_program,
        permissions,
        &writable_paths,
        command.cwd.as_deref(),
        &command.program,
        &command.args,
        proxy_socket_path(&sandbox.permissions.network),
    )
    .map_err(map_anyhow_error)?;

    Ok(PreparedSandboxCommand::new(
        program,
        args,
        command.env.clone(),
        command.cwd.clone(),
    ))
}

#[cfg(target_os = "macos")]
fn prepare_sandboxed_command_sync(
    sandbox: &SandboxConfiguration,
    command: &CommandAndArgs,
) -> Result<PreparedSandboxCommand, SandboxError> {
    let writable_paths = writable_path_refs(sandbox);
    let permissions = macos_permissions(&sandbox.permissions);
    let (program, args, handle) = macos_seatbelt::wrap_invocation(
        &command.program,
        &command.args,
        &writable_paths,
        permissions,
    )
    .map_err(map_anyhow_error)?;

    Ok(
        PreparedSandboxCommand::new(program, args, command.env.clone(), command.cwd.clone())
            .with_macos_handle(handle),
    )
}

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
fn prepare_sandboxed_command_sync(
    _sandbox: &SandboxConfiguration,
    _command: &CommandAndArgs,
) -> Result<PreparedSandboxCommand, SandboxError> {
    Err(SandboxError::UnsupportedPlatform)
}

#[cfg(target_os = "windows")]
async fn prepare_sandboxed_command_windows(
    sandbox: &SandboxConfiguration,
    command: &CommandAndArgs,
) -> Result<PreparedSandboxCommand, SandboxError> {
    if matches!(
        sandbox.permissions.network,
        NetworkAccess::LocalhostPort { .. }
    ) {
        return Err(SandboxError::UnsupportedPolicy(
            "restricted host network access is not yet supported for Windows sandboxes".to_string(),
        ));
    }
    let (program, args) = windows_wsl::wrap_invocation(
        command.program.clone(),
        command.args.clone(),
        sandbox.writable_paths.clone(),
        sandbox.permissions.clone(),
        command.cwd.clone(),
        command.env.clone(),
    )
    .await
    .map_err(map_anyhow_error)?;

    Ok(PreparedSandboxCommand::new(
        program,
        args,
        command.env.clone(),
        command.cwd.clone(),
    ))
}

fn writable_path_refs(sandbox: &SandboxConfiguration) -> Vec<&Path> {
    sandbox
        .writable_paths
        .iter()
        .map(|path| path.as_path())
        .collect()
}

#[cfg(target_os = "linux")]
fn linux_permissions(permissions: &SandboxPermissions) -> linux_bubblewrap::SandboxPermissions {
    linux_bubblewrap::SandboxPermissions {
        network: match permissions.network {
            NetworkAccess::None => linux_bubblewrap::NetworkAccess::None,
            NetworkAccess::LocalhostPort { port, .. } => {
                linux_bubblewrap::NetworkAccess::LocalhostPort(port)
            }
            NetworkAccess::All => linux_bubblewrap::NetworkAccess::All,
        },
        allow_fs_write: permissions.allow_fs_write,
    }
}

#[cfg(target_os = "macos")]
fn macos_permissions(permissions: &SandboxPermissions) -> macos_seatbelt::SandboxPermissions {
    macos_seatbelt::SandboxPermissions {
        network: match permissions.network {
            NetworkAccess::None => macos_seatbelt::NetworkAccess::None,
            NetworkAccess::LocalhostPort { port, .. } => {
                macos_seatbelt::NetworkAccess::LocalhostPort(port)
            }
            NetworkAccess::All => macos_seatbelt::NetworkAccess::All,
        },
        allow_fs_write: permissions.allow_fs_write,
    }
}

fn proxy_socket_path(network: &NetworkAccess) -> Option<&Path> {
    match network {
        NetworkAccess::LocalhostPort {
            proxy_socket_path, ..
        } => proxy_socket_path.as_deref(),
        NetworkAccess::None | NetworkAccess::All => None,
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
