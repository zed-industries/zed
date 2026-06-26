//! Linux sandbox integration built on Bubblewrap (`bwrap`) for filesystem and
//! network confinement.
//!
//! We can use `--bind` and `--ro-bind` (read-only) to bind host filesystem
//! paths to paths in the sandbox. If networking is restricted, we also set
//! `--unshare-net` to disable *all* network access.
//!
//! When restricting network access, we:
//! - set `--unshare-net` - any requests to `example.com` will fail
//!   - requests to `localhost` will succeed, but it will be an isolated localhost
//!     from the host system.
//! - create a unix socket, and mount it in the sandbox with `--bind`
//! - run a bridge process inside the sandbox that:
//!   - listens on `localhost:<port>` and forwards reads/writes to the socket
//!   - then, runs the untrusted command
//! - on the zed side, we listen to the socket and forward reads/writes to the
//!   internal HTTP proxy
//!
//! If networking is fully blocked or fully allowed, we don't bother with the
//! proxy/socket at all (and simply set/unset `--unshare-net`).
//!
//! This design for networking avoids needing seccomp, a fork/exec dance, and
//! eliminates a race condition involving BPF user notifications.

use anyhow::{Context as _, Result, anyhow, bail};
use std::ffi::OsString;
use std::io::{Read, Write};
use std::net::{Ipv4Addr, Shutdown, TcpListener, TcpStream};
use std::os::unix::fs::MetadataExt as _;
use std::os::unix::net::UnixStream;
use std::os::unix::process::ExitStatusExt as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;

const BRIDGE_FLAG: &str = "--zed-linux-sandbox-bridge";
const PROXY_SOCKET_SANDBOX_PATH_PREFIX: &str = "/tmp/zed-sandbox";
const SANDBOX_SETUP_FAILED_EXIT_CODE: i32 = 126;
const PUMP_BUFFER_SIZE: usize = 64 * 1024;

/// Network-access setting for a sandboxed command.
///
/// Mirrors [`crate::macos_seatbelt::NetworkAccess`] so Linux and macOS expose
/// the same public shape.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum NetworkAccess {
    /// All outbound network blocked (own network namespace).
    #[default]
    None,
    /// Outbound HTTP(S) is available through a loopback proxy port inside the
    /// sandbox. The port is bridged to a host-side Unix socket proxy that
    /// enforces the hostname allowlist.
    LocalhostPort(u16),
    /// All outbound network allowed.
    All,
}

/// Per-command relaxations of the default Bubblewrap (Linux) sandbox.
///
/// Mirrors [`crate::macos_seatbelt::SandboxPermissions`].
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SandboxPermissions {
    /// Network access policy for the command.
    pub network: NetworkAccess,
    /// Allow unrestricted filesystem writes.
    pub allow_fs_write: bool,
}

/// The outcome of preparing a Linux sandbox.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LauncherStatus {
    /// No usable `bwrap` binary was found on `PATH` (or bundled).
    BwrapNotFound,
    /// The only `bwrap` found is setuid-root, which we refuse to execute.
    SetuidRejected,
    /// `bwrap` is present but failed to set up the sandbox with our arguments.
    SandboxProbeFailed,
}

impl LauncherStatus {
    /// A human-readable explanation suitable for diagnostics.
    pub fn describe(self) -> &'static str {
        match self {
            LauncherStatus::BwrapNotFound => "no usable `bwrap` binary was found on PATH",
            LauncherStatus::SetuidRejected => {
                "the only available `bwrap` is setuid-root, which Zed refuses to run"
            }
            LauncherStatus::SandboxProbeFailed => {
                "`bwrap` is present but failed to create a sandbox (unprivileged user \
                 namespaces may be disabled)"
            }
        }
    }
}

/// Where a `bwrap` lookup ended up.
enum BwrapLocation {
    /// A usable, non-setuid `bwrap` binary.
    Found(PathBuf),
    /// `bwrap` exists but every candidate is setuid-root (which we won't run).
    OnlySetuid,
    /// No `bwrap` binary was found at all.
    NotFound,
}

fn locate_bwrap() -> BwrapLocation {
    let mut saw_setuid = false;
    for candidate in candidate_bwrap_paths() {
        if !candidate.is_file() {
            continue;
        }
        if is_setuid_root(&candidate) {
            saw_setuid = true;
            continue;
        }
        return BwrapLocation::Found(candidate);
    }
    if saw_setuid {
        BwrapLocation::OnlySetuid
    } else {
        BwrapLocation::NotFound
    }
}

fn candidate_bwrap_paths() -> Vec<PathBuf> {
    let mut candidates = Vec::new();
    if let Some(system) = system_bwrap_path() {
        candidates.push(system);
    }
    if let Some(bundled) = bundled_bwrap_path() {
        candidates.push(bundled);
    }
    candidates
}

fn system_bwrap_path() -> Option<PathBuf> {
    let path = std::env::var_os("PATH")?;
    std::env::split_paths(&path)
        .map(|directory| directory.join("bwrap"))
        .find(|candidate| candidate.is_file())
}

fn bundled_bwrap_path() -> Option<PathBuf> {
    None
}

fn is_setuid_root(path: &Path) -> bool {
    match std::fs::metadata(path) {
        Ok(metadata) => (metadata.mode() & libc::S_ISUID != 0) && metadata.uid() == 0,
        Err(_) => false,
    }
}

#[allow(
    clippy::disallowed_methods,
    reason = "the probe is a short-lived background operation that must block on bwrap"
)]
fn probe_bwrap(bwrap: &Path, bwrap_args: &[String]) -> bool {
    Command::new(bwrap)
        .args(bwrap_args)
        .arg("--")
        .arg("true")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

/// Build the `bwrap` argument list (everything after the `bwrap` program and
/// before the trailing `-- <command>`) for the given policy.
///
/// `proxy_socket_path` is the host pathname Unix socket used for
/// [`NetworkAccess::LocalhostPort`]. It is bind-mounted to a unique path inside
/// the sandbox where the bridge connects to it.
pub fn build_bwrap_args(
    writable_directories: &[&Path],
    protected_git_dirs: &[&Path],
    permissions: SandboxPermissions,
    cwd: Option<&Path>,
    proxy_socket_path: Option<&Path>,
) -> Vec<String> {
    let proxy_socket_sandbox_path = proxy_socket_path
        .filter(|_| matches!(permissions.network, NetworkAccess::LocalhostPort(_)))
        .map(|_| unique_proxy_socket_sandbox_path());
    build_bwrap_args_with_proxy_socket_sandbox_path(
        writable_directories,
        protected_git_dirs,
        permissions,
        cwd,
        proxy_socket_path,
        proxy_socket_sandbox_path.as_deref(),
    )
}

fn build_bwrap_args_with_proxy_socket_sandbox_path(
    writable_directories: &[&Path],
    protected_git_dirs: &[&Path],
    permissions: SandboxPermissions,
    cwd: Option<&Path>,
    proxy_socket_path: Option<&Path>,
    proxy_socket_sandbox_path: Option<&Path>,
) -> Vec<String> {
    let mut args = Vec::new();

    let root_bind = if permissions.allow_fs_write {
        "--bind"
    } else {
        "--ro-bind"
    };
    push_bind(&mut args, root_bind, "/", "/");

    args.push("--dev".to_string());
    args.push("/dev".to_string());
    args.push("--proc".to_string());
    args.push("/proc".to_string());

    if !permissions.allow_fs_write {
        args.push("--tmpfs".to_string());
        args.push("/tmp".to_string());

        for directory in writable_directories {
            // Bind each writable directory at its *exact* path. We must never
            // fall back to an existing ancestor: that would grant write access
            // to a broader tree than was requested and approved (e.g. binding
            // `$HOME` for a not-yet-created `~/.cache/...`, or re-binding the
            // host `/tmp` over the tmpfs established above). `wrap_invocation`
            // creates the requested directories before launch so they exist
            // here; any that still don't are skipped rather than widened.
            if !directory.exists() {
                continue;
            }
            let canonical = directory
                .canonicalize()
                .unwrap_or_else(|_| directory.to_path_buf());
            let path = canonical.to_string_lossy().into_owned();
            push_bind(&mut args, "--bind", &path, &path);
        }

        // Protect Git directories by re-binding them read-only *over* the rw
        // worktree binds above (order matters: later binds win). Unlike
        // Seatbelt, bwrap can't deny content reads while keeping metadata, so on
        // Linux a protected `.git` is read-only — its contents stay readable but
        // can't be written. A not-yet-existing `.git` can't be bound, so it's
        // skipped (a documented gap vs. macOS, which denies it even before it
        // exists). When Git access is granted these dirs are in
        // `writable_directories` instead and this list is empty.
        for git_dir in protected_git_dirs {
            if !git_dir.exists() {
                continue;
            }
            let canonical = git_dir
                .canonicalize()
                .unwrap_or_else(|_| git_dir.to_path_buf());
            let path = canonical.to_string_lossy().into_owned();
            push_bind(&mut args, "--ro-bind", &path, &path);
        }
    }

    for flag in [
        "--unshare-user",
        "--unshare-ipc",
        "--unshare-uts",
        "--unshare-pid",
        "--unshare-cgroup-try",
        "--die-with-parent",
    ] {
        args.push(flag.to_string());
    }

    match permissions.network {
        NetworkAccess::None => args.push("--unshare-net".to_string()),
        NetworkAccess::LocalhostPort(_) => {
            args.push("--unshare-net".to_string());
            if let Some((proxy_socket_path, proxy_socket_sandbox_path)) =
                proxy_socket_path.zip(proxy_socket_sandbox_path)
            {
                let source = proxy_socket_path.to_string_lossy().into_owned();
                let destination = proxy_socket_sandbox_path.to_string_lossy().into_owned();
                push_bind(&mut args, "--bind", &source, &destination);
            }
        }
        NetworkAccess::All => {}
    }

    if let Some(cwd) = cwd {
        args.push("--chdir".to_string());
        args.push(cwd.to_string_lossy().into_owned());
    }

    args
}

/// A unique destination path for the proxy socket bind mount *inside* the
/// sandbox. Each sandboxed command runs in its own mount namespace, so the path
/// only needs to be unique enough to avoid colliding with anything a previous
/// command in the same namespace lineage may have created; combining the pid
/// with a monotonic counter is sufficient and keeps the path predictable for
/// diagnostics.
fn unique_proxy_socket_sandbox_path() -> PathBuf {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let counter = COUNTER.fetch_add(1, Ordering::Relaxed);
    PathBuf::from(format!(
        "{PROXY_SOCKET_SANDBOX_PATH_PREFIX}-{}-{counter}.sock",
        std::process::id()
    ))
}

fn push_bind(args: &mut Vec<String>, flag: &str, source: &str, destination: &str) {
    args.push(flag.to_string());
    args.push(source.to_string());
    args.push(destination.to_string());
}

fn resolve_bwrap() -> std::result::Result<PathBuf, LauncherStatus> {
    match locate_bwrap() {
        BwrapLocation::Found(path) => Ok(path),
        BwrapLocation::OnlySetuid => Err(LauncherStatus::SetuidRejected),
        BwrapLocation::NotFound => Err(LauncherStatus::BwrapNotFound),
    }
}

fn prepare_sandbox(
    writable_dirs: &[&Path],
    permissions: SandboxPermissions,
    cwd: Option<&Path>,
    proxy_socket_path: Option<&Path>,
) -> std::result::Result<(PathBuf, Vec<String>), LauncherStatus> {
    let bwrap = resolve_bwrap()?;
    // The probe only checks whether a sandbox can be created at all, so Git
    // protection (which doesn't affect createability) is irrelevant here.
    let bwrap_args = build_bwrap_args(writable_dirs, &[], permissions, cwd, proxy_socket_path);
    if !probe_bwrap(&bwrap, &bwrap_args) {
        return Err(LauncherStatus::SandboxProbeFailed);
    }
    Ok((bwrap, bwrap_args))
}

/// Check whether an OS sandbox can be created for this policy.
pub fn check_can_create_sandbox(
    writable_dirs: &[&Path],
    permissions: SandboxPermissions,
    cwd: Option<&Path>,
) -> std::result::Result<(), LauncherStatus> {
    prepare_sandbox(writable_dirs, permissions, cwd, None).map(|_| ())
}

/// Build the final command line that runs `program` inside Bubblewrap.
///
/// `bridge_program` should be the current Zed executable. It is only used for
/// [`NetworkAccess::LocalhostPort`], where it runs in bridge mode inside the
/// sandbox before spawning the real command.
///
/// The host `proxy_socket_path` is bind-mounted to a per-invocation unique path
/// inside the sandbox, and that same in-sandbox path is handed to the bridge.
pub fn wrap_invocation(
    bridge_program: &str,
    permissions: SandboxPermissions,
    writable_dirs: &[&Path],
    protected_git_dirs: &[&Path],
    cwd: Option<&Path>,
    program: &str,
    args: &[String],
    proxy_socket_path: Option<&Path>,
) -> Result<(String, Vec<String>)> {
    if matches!(permissions.network, NetworkAccess::LocalhostPort(_)) && proxy_socket_path.is_none()
    {
        bail!("restricted Linux network access requires a proxy Unix socket path");
    }

    // Create the requested writable directories up front, with the agent's
    // ambient permissions, so each can be bind-mounted at its exact path (see
    // `build_bwrap_args`). Without this a not-yet-existing writable path could
    // not be bound, and the command could not create it either (its parent is
    // read-only inside the sandbox). Best-effort: a directory we can't create is
    // left unbound rather than widening the sandbox to an existing ancestor.
    if !permissions.allow_fs_write {
        for directory in writable_dirs {
            if let Err(error) = std::fs::create_dir_all(directory) {
                log::warn!(
                    "[sandbox] could not create writable directory {}: {error}",
                    directory.display()
                );
            }
        }
    }

    let bwrap = resolve_bwrap().map_err(|status| anyhow!(status.describe()))?;
    let proxy_socket_sandbox_path = match permissions.network {
        NetworkAccess::LocalhostPort(_) => Some(unique_proxy_socket_sandbox_path()),
        NetworkAccess::None | NetworkAccess::All => None,
    };
    let mut bwrap_args = build_bwrap_args_with_proxy_socket_sandbox_path(
        writable_dirs,
        protected_git_dirs,
        permissions,
        cwd,
        proxy_socket_path,
        proxy_socket_sandbox_path.as_deref(),
    );
    bwrap_args.push("--".to_string());

    match permissions.network {
        NetworkAccess::LocalhostPort(port) => {
            let proxy_socket_sandbox_path = proxy_socket_sandbox_path
                .as_ref()
                .context("missing in-sandbox proxy socket path")?;
            bwrap_args.push(bridge_program.to_string());
            bwrap_args.push(BRIDGE_FLAG.to_string());
            bwrap_args.push(proxy_socket_sandbox_path.to_string_lossy().into_owned());
            bwrap_args.push(port.to_string());
            bwrap_args.push("--".to_string());
        }
        NetworkAccess::None | NetworkAccess::All => {}
    }

    bwrap_args.push(program.to_string());
    bwrap_args.extend(args.iter().cloned());

    let bwrap = bwrap
        .to_str()
        .with_context(|| format!("bwrap path contains invalid UTF-8: {}", bwrap.display()))?;
    Ok((bwrap.to_string(), bwrap_args))
}

/// Handle a possible re-exec of this binary as the in-sandbox proxy bridge.
pub fn run_launcher_if_invoked() {
    let Some(invocation) = parse_bridge_args(std::env::args_os()) else {
        return;
    };
    let invocation = match invocation {
        Ok(invocation) => invocation,
        Err(error) => {
            eprintln!("zed: malformed sandbox bridge invocation: {error:#}");
            std::process::exit(127);
        }
    };
    run_bridge(invocation);
}

struct BridgeInvocation {
    socket_path: PathBuf,
    port: u16,
    program: OsString,
    args: Vec<OsString>,
}

fn parse_bridge_args(args: impl IntoIterator<Item = OsString>) -> Option<Result<BridgeInvocation>> {
    let mut args = args.into_iter();
    args.next()?;
    if args.next()?.to_str() != Some(BRIDGE_FLAG) {
        return None;
    }
    Some(decode_bridge_args(args))
}

fn decode_bridge_args(mut args: impl Iterator<Item = OsString>) -> Result<BridgeInvocation> {
    let socket_path = PathBuf::from(args.next().context("missing proxy socket path")?);
    let port = args
        .next()
        .context("missing proxy bridge port")?
        .to_str()
        .context("proxy bridge port is not valid UTF-8")?
        .parse::<u16>()
        .context("invalid proxy bridge port")?;
    let separator = args.next().context("missing bridge argument separator")?;
    if separator != "--" {
        bail!("missing bridge argument separator");
    }
    let program = args.next().context("missing program to run")?;
    let args = args.collect();
    Ok(BridgeInvocation {
        socket_path,
        port,
        program,
        args,
    })
}

#[allow(
    clippy::disallowed_methods,
    reason = "the bridge is an in-sandbox process that must synchronously spawn and wait for the command"
)]
fn run_bridge(invocation: BridgeInvocation) -> ! {
    let listener = match TcpListener::bind((Ipv4Addr::LOCALHOST, invocation.port)) {
        Ok(listener) => listener,
        Err(error) => {
            eprintln!("zed: failed to bind sandbox proxy bridge: {error}");
            std::process::exit(SANDBOX_SETUP_FAILED_EXIT_CODE);
        }
    };

    let socket_path = invocation.socket_path.clone();
    if let Err(error) = thread::Builder::new()
        .name("zed-sandbox-bridge".to_string())
        .stack_size(128 * 1024)
        .spawn(move || run_bridge_listener(listener, socket_path))
    {
        eprintln!("zed: failed to spawn sandbox proxy bridge: {error}");
        std::process::exit(SANDBOX_SETUP_FAILED_EXIT_CODE);
    }

    let mut child = match Command::new(&invocation.program)
        .args(&invocation.args)
        .spawn()
    {
        Ok(child) => child,
        Err(error) => {
            eprintln!("zed: failed to spawn sandboxed command: {error}");
            std::process::exit(SANDBOX_SETUP_FAILED_EXIT_CODE);
        }
    };

    match child.wait() {
        Ok(status) => {
            if let Some(code) = status.code() {
                std::process::exit(code);
            }
            let signal = status.signal().unwrap_or(1);
            std::process::exit(128 + signal);
        }
        Err(error) => {
            eprintln!("zed: failed to wait for sandboxed command: {error}");
            std::process::exit(SANDBOX_SETUP_FAILED_EXIT_CODE);
        }
    }
}

fn run_bridge_listener(listener: TcpListener, socket_path: PathBuf) {
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let socket_path = socket_path.clone();
                if let Err(error) = thread::Builder::new()
                    .name("zed-sandbox-bridge-conn".to_string())
                    .stack_size(128 * 1024)
                    .spawn(move || forward_bridge_connection(stream, socket_path))
                {
                    eprintln!("zed: failed to spawn sandbox bridge connection thread: {error}");
                }
            }
            Err(error) => eprintln!("zed: sandbox bridge accept failed: {error}"),
        }
    }
}

fn forward_bridge_connection(tcp_stream: TcpStream, socket_path: PathBuf) {
    let unix_stream = match UnixStream::connect(&socket_path) {
        Ok(stream) => stream,
        Err(error) => {
            eprintln!(
                "zed: sandbox bridge failed to connect to proxy socket {}: {error}",
                socket_path.display()
            );
            return;
        }
    };
    copy_bidirectional(tcp_stream, unix_stream);
}

fn copy_bidirectional(tcp_stream: TcpStream, unix_stream: UnixStream) {
    let tcp_read = match tcp_stream.try_clone() {
        Ok(stream) => stream,
        Err(error) => {
            eprintln!("zed: sandbox bridge failed to clone TCP stream: {error}");
            return;
        }
    };
    let unix_read = match unix_stream.try_clone() {
        Ok(stream) => stream,
        Err(error) => {
            eprintln!("zed: sandbox bridge failed to clone Unix stream: {error}");
            return;
        }
    };

    let tcp_write = tcp_stream;
    let unix_write = unix_stream;
    let to_proxy = match thread::Builder::new()
        .name("zed-sandbox-bridge-out".to_string())
        .stack_size(128 * 1024)
        .spawn(move || copy_one_way(tcp_read, unix_write))
    {
        Ok(handle) => handle,
        Err(error) => {
            eprintln!("zed: failed to spawn sandbox bridge pump thread: {error}");
            return;
        }
    };
    copy_one_way(unix_read, tcp_write);
    if to_proxy.join().is_err() {
        eprintln!("zed: sandbox bridge pump thread panicked");
    }
}

trait BridgeStream: Read + Write {
    fn shutdown(&self, how: Shutdown) -> std::io::Result<()>;
}

impl BridgeStream for TcpStream {
    fn shutdown(&self, how: Shutdown) -> std::io::Result<()> {
        TcpStream::shutdown(self, how)
    }
}

impl BridgeStream for UnixStream {
    fn shutdown(&self, how: Shutdown) -> std::io::Result<()> {
        UnixStream::shutdown(self, how)
    }
}

fn copy_one_way(mut from: impl Read, mut to: impl BridgeStream) {
    let mut buffer = vec![0; PUMP_BUFFER_SIZE];
    loop {
        let count = match from.read(&mut buffer) {
            Ok(0) => break,
            Ok(count) => count,
            Err(_) => break,
        };
        if to.write_all(&buffer[..count]).is_err() {
            break;
        }
    }
    let _ = to.shutdown(Shutdown::Write);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bridge_argv(launcher: &str, args: Vec<&str>) -> Vec<OsString> {
        std::iter::once(launcher)
            .chain(args)
            .map(OsString::from)
            .collect()
    }

    #[test]
    fn test_build_bwrap_args_binds_exact_path_never_widens_to_ancestor() {
        // A requested writable path that doesn't exist must NOT be bound, and in
        // particular must never cause an existing ancestor (here the tempdir) to
        // be bound read-write — that was a sandbox-escape (scope widening).
        let dir = tempfile::tempdir().unwrap();
        let existing = dir.path().canonicalize().unwrap();
        let missing = existing.join("does-not-exist").join("nested");

        let args = build_bwrap_args(
            &[missing.as_path()],
            &[],
            SandboxPermissions::default(),
            None,
            None,
        );

        let existing_str = existing.to_string_lossy().into_owned();
        assert!(
            !windows_contains(&args, &["--bind", &existing_str, &existing_str]),
            "a missing writable path must not widen the bind to an existing ancestor: {args:?}"
        );
        let missing_str = missing.to_string_lossy().into_owned();
        assert!(
            !windows_contains(&args, &["--bind", &missing_str, &missing_str]),
            "a missing writable path must not be bound: {args:?}"
        );
    }

    #[test]
    fn test_build_bwrap_args_default_binds_writable_dirs_read_write() {
        let writable = tempfile::tempdir().unwrap();
        let args = build_bwrap_args(
            &[writable.path()],
            &[],
            SandboxPermissions::default(),
            Some(writable.path()),
            None,
        );

        assert!(windows_contains(&args, &["--ro-bind", "/", "/"]));
        let writable_path = writable.path().canonicalize().unwrap();
        let writable_str = writable_path.to_string_lossy().into_owned();
        assert!(windows_contains(
            &args,
            &["--bind", &writable_str, &writable_str]
        ));
        assert!(windows_contains(&args, &["--tmpfs", "/tmp"]));
        assert!(args.iter().any(|arg| arg == "--chdir"));
        assert!(args.iter().any(|arg| arg == "--unshare-net"));
    }

    #[test]
    fn test_build_bwrap_args_network_namespace_follows_permission() {
        let denied = build_bwrap_args(&[], &[], SandboxPermissions::default(), None, None);
        assert!(denied.iter().any(|arg| arg == "--unshare-net"));

        let allowed = build_bwrap_args(
            &[],
            &[],
            SandboxPermissions {
                network: NetworkAccess::All,
                allow_fs_write: false,
            },
            None,
            None,
        );
        assert!(!allowed.iter().any(|arg| arg == "--unshare-net"));

        let socket = PathBuf::from("/tmp/zed-proxy.sock");
        let restricted = build_bwrap_args(
            &[],
            &[],
            SandboxPermissions {
                network: NetworkAccess::LocalhostPort(8080),
                allow_fs_write: false,
            },
            None,
            Some(socket.as_path()),
        );
        assert!(restricted.iter().any(|arg| arg == "--unshare-net"));
        let sandbox_destination = proxy_socket_bind_destination(&restricted)
            .expect("restricted run should bind the proxy socket into the sandbox");
        assert!(sandbox_destination.starts_with(PROXY_SOCKET_SANDBOX_PATH_PREFIX));
    }

    #[test]
    fn test_build_bwrap_args_allow_fs_write_binds_root_read_write() {
        let permissions = SandboxPermissions {
            network: NetworkAccess::None,
            allow_fs_write: true,
        };
        let args = build_bwrap_args(&[], &[], permissions, None, None);
        assert!(windows_contains(&args, &["--bind", "/", "/"]));
        assert!(!windows_contains(&args, &["--ro-bind", "/", "/"]));
        assert!(!windows_contains(&args, &["--tmpfs", "/tmp"]));
    }

    #[test]
    fn test_bridge_args_round_trip() {
        let sandbox_socket_path = "/tmp/zed-sandbox-1234-0.sock";
        let argv = bridge_argv(
            "/path/to/zed",
            vec![
                BRIDGE_FLAG,
                sandbox_socket_path,
                "8080",
                "--",
                "/bin/sh",
                "-c",
                "echo hi there",
            ],
        );

        let decoded = parse_bridge_args(argv)
            .expect("should be recognized as bridge invocation")
            .expect("should decode successfully");

        assert_eq!(decoded.socket_path, PathBuf::from(sandbox_socket_path));
        assert_eq!(decoded.port, 8080);
        assert_eq!(decoded.program, OsString::from("/bin/sh"));
        assert_eq!(
            decoded.args,
            vec![OsString::from("-c"), OsString::from("echo hi there")]
        );
    }

    #[test]
    fn test_wrap_invocation_uses_bridge_for_restricted_network() {
        let socket = PathBuf::from("/tmp/zed-proxy.sock");
        let permissions = SandboxPermissions {
            network: NetworkAccess::LocalhostPort(8080),
            allow_fs_write: false,
        };
        let args = build_wrapped_args_for_test(
            "/path/to/zed",
            permissions,
            "/bin/sh",
            &["-c".to_string(), "echo hi".to_string()],
            Some(socket.as_path()),
        );

        // The bind destination inside the sandbox and the path handed to the
        // bridge must be the same unique path.
        let sandbox_destination = proxy_socket_bind_destination(&args)
            .expect("restricted run should bind the proxy socket into the sandbox");
        assert!(sandbox_destination.starts_with(PROXY_SOCKET_SANDBOX_PATH_PREFIX));
        assert!(windows_contains(
            &args,
            &[
                "/path/to/zed",
                BRIDGE_FLAG,
                &sandbox_destination,
                "8080",
                "--"
            ]
        ));
    }

    fn build_wrapped_args_for_test(
        bridge_program: &str,
        permissions: SandboxPermissions,
        program: &str,
        program_args: &[String],
        proxy_socket_path: Option<&Path>,
    ) -> Vec<String> {
        let proxy_socket_sandbox_path = match permissions.network {
            NetworkAccess::LocalhostPort(_) => Some(unique_proxy_socket_sandbox_path()),
            NetworkAccess::None | NetworkAccess::All => None,
        };
        let mut bwrap_args = build_bwrap_args_with_proxy_socket_sandbox_path(
            &[],
            &[],
            permissions,
            None,
            proxy_socket_path,
            proxy_socket_sandbox_path.as_deref(),
        );
        bwrap_args.push("--".to_string());
        if let NetworkAccess::LocalhostPort(port) = permissions.network {
            let proxy_socket_sandbox_path =
                proxy_socket_sandbox_path.expect("restricted network needs a sandbox socket path");
            bwrap_args.push(bridge_program.to_string());
            bwrap_args.push(BRIDGE_FLAG.to_string());
            bwrap_args.push(proxy_socket_sandbox_path.to_string_lossy().into_owned());
            bwrap_args.push(port.to_string());
            bwrap_args.push("--".to_string());
        }
        bwrap_args.push(program.to_string());
        bwrap_args.extend(program_args.iter().cloned());
        bwrap_args
    }

    /// Returns the in-sandbox destination of the proxy socket `--bind`, if any.
    fn proxy_socket_bind_destination(args: &[String]) -> Option<String> {
        args.windows(3).find_map(|window| {
            if window[0] == "--bind" && window[1] == "/tmp/zed-proxy.sock" {
                Some(window[2].clone())
            } else {
                None
            }
        })
    }

    fn windows_contains(haystack: &[String], needle: &[&str]) -> bool {
        haystack
            .windows(needle.len())
            .any(|window| window.iter().map(String::as_str).eq(needle.iter().copied()))
    }
}
