//! Linux sandbox integration built on Bubblewrap (`bwrap`) for both the
//! filesystem and the network.
//!
//! `bwrap` is an unprivileged container launcher: it sets up mount, user,
//! pid, and other namespaces, bind-mounts the host filesystem (read-only by
//! default, read-write for an explicit set of directories), then `exec`s the
//! wrapped command inside that view. Network access can be wholly
//! enabled/disabled using `--unshare-net`. More granular access will require
//! seccomp.
//!
//! ## The launcher
//!
//! We fork/exec the running zed binary because:
//! - previously we used landlock, which can only restrict the *current
//!   process*, so we need a subprocess running our code to restrict.
//! - seccomp is similar, and while we don't have that yet, we will in the
//!   future, so we keep the machinery for now.
//!
//! This approach also avoids `pre_exec`, which is scary.
//!
//! ## Status reporting
//!
//! There are three possible error cases relating to sandboxing:
//! - there is no `bwrap` on the path
//! - there is a `bwrap` on the path, but it's a setuid binary (which we reject
//!   for security reasons)
//! - there is a `bwrap` on the path, but it fails to create the sandbox (see
//!   [`probe_bwrap`]).
//!
//! If one of these happens, we report it back to the parent over a `SOCK_DGRAM`
//! and then *abort*: the launcher never runs the command unsandboxed on its own.
//! What to do about a failure is the caller's decision — the agent currently
//! fails open (re-runs the command without a sandbox and tells the model so via
//! [`check_can_create_sandbox`]), while the NixOS tests fail closed (treat it as
//! a hard error). There's no UI for the agent's fallback yet; that's coming
//! soon.

use std::ffi::OsString;
use std::os::linux::net::SocketAddrExt as _;
use std::os::unix::fs::MetadataExt as _;
use std::os::unix::net::{SocketAddr, UnixDatagram};
use std::os::unix::process::CommandExt as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use anyhow::{Context as _, Result, anyhow, bail};

use crate::SandboxPermissions;

/// The outcome a sandbox launcher reports back to the process that spawned it.
///
/// Only [`Success`](LauncherStatus::Success) means the command actually ran
/// fully sandboxed; every other variant means the launcher could not enforce
/// the sandbox and ran the command without one (with the reason reported here).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LauncherStatus {
    /// The sandbox was fully enforced: `bwrap` filesystem and (when requested)
    /// network isolation.
    Success,
    /// No usable `bwrap` binary was found on `PATH` (or bundled).
    BwrapNotFound,
    /// The only `bwrap` found is setuid-root, which we refuse to execute.
    SetuidRejected,
    /// `bwrap` is present but failed to set up the sandbox with our arguments
    /// (typically because unprivileged user namespaces are disabled).
    SandboxProbeFailed,
}

impl LauncherStatus {
    fn to_byte(self) -> u8 {
        match self {
            LauncherStatus::Success => 0,
            LauncherStatus::BwrapNotFound => 1,
            LauncherStatus::SetuidRejected => 2,
            LauncherStatus::SandboxProbeFailed => 3,
        }
    }

    fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            0 => Some(LauncherStatus::Success),
            1 => Some(LauncherStatus::BwrapNotFound),
            2 => Some(LauncherStatus::SetuidRejected),
            3 => Some(LauncherStatus::SandboxProbeFailed),
            _ => None,
        }
    }

    /// Whether the command ran fully sandboxed.
    pub fn is_success(self) -> bool {
        matches!(self, LauncherStatus::Success)
    }

    /// A human-readable explanation suitable for a `log::warn` on a non-success
    /// outcome.
    pub fn describe(self) -> &'static str {
        match self {
            LauncherStatus::Success => "the sandbox was created",
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

/// A one-shot datagram socket the spawning process binds so a launcher can
/// report its [`LauncherStatus`] back.
///
/// Uses a Linux *abstract* `AF_UNIX` address (no filesystem entry to create,
/// permission, or clean up). A connectionless `SOCK_DGRAM` socket is all we
/// need here: the launcher sends exactly one datagram and the parent receives
/// it once. Because the socket is bound before the launcher is spawned, the
/// kernel queues the datagram even if it arrives before [`recv`](Self::recv)
/// is called.
pub struct StatusChannel {
    socket: UnixDatagram,
    name: String,
}

impl StatusChannel {
    /// Bind a fresh status channel with a unique abstract address.
    pub fn bind() -> Result<Self> {
        let name = unique_socket_name();
        let address = SocketAddr::from_abstract_name(name.as_bytes())
            .context("failed to build abstract socket address")?;
        let socket =
            UnixDatagram::bind_addr(&address).context("failed to bind sandbox status socket")?;
        Ok(Self { socket, name })
    }

    /// The abstract address to hand to the launcher (see [`wrap_invocation`]).
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Wait up to `timeout` for the launcher's one-shot status datagram.
    ///
    /// Returns `None` if nothing arrived in time, or the launcher exited /
    /// `exec`ed without reporting (in which case the sandbox state is unknown).
    pub fn recv(self, timeout: Duration) -> Option<LauncherStatus> {
        self.socket.set_read_timeout(Some(timeout)).ok()?;
        let mut byte = [0u8; 1];
        let count = self.socket.recv(&mut byte).ok()?;
        if count == 0 {
            return None;
        }
        LauncherStatus::from_byte(byte[0])
    }
}

/// A process-unique abstract socket name. The pid plus a monotonically
/// increasing counter and the current time make collisions between concurrent
/// launches (and between Zed instances) effectively impossible.
fn unique_socket_name() -> String {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let counter = COUNTER.fetch_add(1, Ordering::Relaxed);
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or(0);
    format!(
        "zed-sandbox-status-{}-{}-{}",
        std::process::id(),
        nanos,
        counter
    )
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

/// Locate a usable `bwrap` binary, preferring one on `PATH` and falling back to
/// a binary bundled with the application.
///
/// We refuse setuid-root binaries (see [`is_setuid_root`]); if the only
/// candidates are setuid we report [`BwrapLocation::OnlySetuid`] separately
/// from "not found" so the user gets an accurate explanation.
///
/// This runs once per launcher process, so it is intentionally not cached.
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

/// `bwrap` candidates in priority order: a system binary on `PATH` first, then
/// a binary bundled with the application.
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

/// The first `bwrap` found by walking `PATH`, if any.
fn system_bwrap_path() -> Option<PathBuf> {
    let path = std::env::var_os("PATH")?;
    std::env::split_paths(&path)
        .map(|directory| directory.join("bwrap"))
        .find(|candidate| candidate.is_file())
}

/// Path to a `bwrap` binary bundled alongside the application, if one ships.
///
/// Bundling a statically linked, non-setuid `bwrap` is not yet wired up, so
/// this returns `None` for now and only system binaries are used.
fn bundled_bwrap_path() -> Option<PathBuf> {
    None
}

/// Whether `path` is a setuid-root binary.
///
/// We refuse to use (or even probe) such a binary: our sandbox is built
/// entirely on unprivileged user namespaces, so a setuid `bwrap` buys no
/// functionality, and *running* one would mean executing root-privileged setup
/// with argv partly derived from model-influenced input — a
/// privilege-escalation surface. Rejecting it keeps the whole pipeline
/// unprivileged and means a planted setuid binary earlier on `PATH` can't trick
/// us into executing it.
fn is_setuid_root(path: &Path) -> bool {
    match std::fs::metadata(path) {
        Ok(metadata) => (metadata.mode() & libc::S_ISUID != 0) && metadata.uid() == 0,
        Err(_) => false,
    }
}

/// Run `bwrap <args> -- true` and report whether it succeeded, i.e. whether
/// this environment can actually set up the sandbox we're about to use.
///
/// Probing with the *exact* argument list we'll `exec` (rather than a minimal
/// canned one) means we catch failures specific to this command's policy —
/// e.g. a writable bind that can't be mounted — not just whether user
/// namespaces work at all.
#[allow(
    clippy::disallowed_methods,
    reason = "the launcher is a short-lived, single-threaded process that must block on the probe child"
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
/// The default view binds the whole filesystem read-only, overlays fresh
/// `/dev` and `/proc`, mounts an ephemeral `tmpfs` over `/tmp` (so programs
/// that hardcode `/tmp` have somewhere to write without touching the host's
/// `/tmp`), then binds each writable directory read-write on top. When
/// `permissions.allow_fs_write` is set the root is bound read-write instead
/// and these overlays/binds are omitted. When `permissions.allow_network` is
/// false the command is placed in its own network namespace (`--unshare-net`),
/// leaving it with only loopback and no route out.
///
/// `writable_directories` should be the project's worktree paths (plus any
/// user-approved paths), *not* the command's working directory, which is
/// model-controlled and would let the model widen its own writable scope.
///
/// `protected_paths` are paths whose contents must stay inaccessible even when
/// they sit under a writable directory or the root is bound read-write (e.g.
/// `.git` metadata when Git access hasn't been approved). Each is shadowed: a
/// directory with an ephemeral `tmpfs` (real contents hidden, writes land in
/// throwaway storage) and a file with an empty read-only bind of `/dev/null`.
///
/// `allowed_unix_socket_paths` are Unix domain sockets to re-expose on top of
/// any overlay that would otherwise hide them (notably `--tmpfs /tmp`), so
/// local IPC such as the inherited SSH agent socket keeps working. AF_UNIX
/// connectivity is unaffected by `--unshare-net`, so this grants no IP network
/// access.
pub fn build_bwrap_args(
    writable_directories: &[&Path],
    protected_paths: &[&Path],
    allowed_unix_socket_paths: &[&Path],
    permissions: SandboxPermissions,
    cwd: Option<&Path>,
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
        // A writable, ephemeral `/tmp`. This sits before the writable binds
        // below so any writable directory under `/tmp` (e.g. the per-thread
        // terminal temp directory) is re-exposed on top of the tmpfs and
        // keeps its real, persistent contents.
        args.push("--tmpfs".to_string());
        args.push("/tmp".to_string());

        for directory in writable_directories {
            // bwrap can only bind a path that exists. A writable target may
            // not have been created yet (e.g. a command that creates
            // `/tmp/foo.txt`), so bind the nearest existing ancestor and let
            // the command create paths beneath it.
            let Some(existing) = nearest_existing_ancestor(directory) else {
                continue;
            };
            let canonical = existing.canonicalize().unwrap_or(existing);
            let path = canonical.to_string_lossy().into_owned();
            push_bind(&mut args, "--bind", &path, &path);
        }
    }

    // Shadow protected paths last among the filesystem binds so the overlay
    // wins over the root bind and any writable bind above it. Unlike the macOS
    // Seatbelt policy (which denies `file-read-data` while leaving metadata
    // readable), bwrap hides the real contents entirely behind an empty
    // overlay; the security goal — keeping the real path unreadable and
    // unwritable — is the same.
    for protected_path in protected_paths {
        // Resolve through an existing parent when the leaf is missing so a
        // not-yet-created `.git` (before `git init`) overlays the same real path
        // the writable bind above uses, even on a symlinked root.
        let canonical = crate::canonicalize_allowing_missing_leaf(protected_path);
        let destination = canonical.to_string_lossy().into_owned();
        match std::fs::symlink_metadata(&canonical) {
            Ok(metadata) if metadata.is_dir() => {
                args.push("--tmpfs".to_string());
                args.push(destination);
            }
            Ok(_) => {
                // A file (e.g. a linked worktree's `.git` gitlink): mask it
                // with an empty, read-only `/dev/null`.
                push_bind(&mut args, "--ro-bind", "/dev/null", &destination);
            }
            Err(_) => {
                // Doesn't exist yet: still overlay a tmpfs so a command can't
                // `git init` and write real metadata into a writable worktree.
                args.push("--tmpfs".to_string());
                args.push(destination);
            }
        }
    }

    // Re-expose explicitly allowed Unix domain sockets on top of any overlay
    // that would otherwise hide them. Done after the protected overlays and the
    // `/tmp` tmpfs so the socket always wins.
    for socket_path in allowed_unix_socket_paths {
        let canonical = socket_path
            .canonicalize()
            .unwrap_or_else(|_| socket_path.to_path_buf());
        if !canonical.exists() {
            continue;
        }
        let path = canonical.to_string_lossy().into_owned();
        push_bind(&mut args, "--bind", &path, &path);
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

    // Deny network by giving the command its own network namespace (loopback
    // only, no route out). `bwrap` brings up `lo` inside it.
    if !permissions.allow_network {
        args.push("--unshare-net".to_string());
    }

    if let Some(cwd) = cwd {
        args.push("--chdir".to_string());
        args.push(cwd.to_string_lossy().into_owned());
    }

    args
}

fn push_bind(args: &mut Vec<String>, flag: &str, source: &str, destination: &str) {
    args.push(flag.to_string());
    args.push(source.to_string());
    args.push(destination.to_string());
}

/// Return the nearest ancestor of `path` (including `path` itself) that exists
/// on disk, or `None` if nothing up to the root exists.
fn nearest_existing_ancestor(path: &Path) -> Option<PathBuf> {
    let mut candidate = Some(path);
    while let Some(current) = candidate {
        if current.exists() {
            return Some(current.to_path_buf());
        }
        candidate = current.parent();
    }
    None
}

/// Marker passed as the first argument when this binary is re-executed as a
/// sandbox launcher (see [`wrap_invocation`]). Intentionally unlikely to
/// collide with a real argument.
pub const SANDBOX_LAUNCHER_FLAG: &str = "--zed-linux-sandbox-exec";

/// A decoded launcher invocation: the raw sandbox policy plus the command to
/// run. The launcher itself turns this into a `bwrap` command line.
struct LauncherInvocation {
    /// Abstract address of the [`StatusChannel`] to report back on, if any.
    status_socket: Option<String>,
    permissions: SandboxPermissions,
    cwd: Option<PathBuf>,
    writable_dirs: Vec<PathBuf>,
    protected_paths: Vec<PathBuf>,
    allowed_unix_socket_paths: Vec<PathBuf>,
    program: OsString,
    args: Vec<OsString>,
}

/// Build a self-exec launcher invocation.
///
/// Returns `(launcher_program, launcher_args)`, where running
/// `launcher_program` with `launcher_args` re-execs this binary as the
/// launcher. The launcher locates `bwrap`, assembles the sandbox command line,
/// and reports its outcome on the [`StatusChannel`] named by
/// `status_socket_name` (when given).
///
/// The encoding is positional, one value per argv entry, so paths and
/// arguments containing spaces round-trip without escaping. Each path list is a
/// count followed by that many entries:
/// `[FLAG, socket, allow_network, allow_fs_write, cwd,
///   N_writable, writable.., N_protected, protected.., N_sockets, sockets..,
///   program, args..]`.
pub fn wrap_invocation(
    launcher_program: &str,
    status_socket_name: Option<&str>,
    permissions: SandboxPermissions,
    writable_dirs: &[&Path],
    protected_paths: &[&Path],
    allowed_unix_socket_paths: &[&Path],
    cwd: Option<&Path>,
    program: &str,
    args: &[String],
) -> (String, Vec<String>) {
    let mut launcher_args = Vec::with_capacity(
        writable_dirs.len()
            + protected_paths.len()
            + allowed_unix_socket_paths.len()
            + args.len()
            + 9,
    );
    launcher_args.push(SANDBOX_LAUNCHER_FLAG.to_string());
    launcher_args.push(status_socket_name.unwrap_or("").to_string());
    launcher_args.push(encode_bool(permissions.allow_network));
    launcher_args.push(encode_bool(permissions.allow_fs_write));
    launcher_args.push(
        cwd.map(|cwd| cwd.to_string_lossy().into_owned())
            .unwrap_or_default(),
    );
    push_path_list(&mut launcher_args, writable_dirs);
    push_path_list(&mut launcher_args, protected_paths);
    push_path_list(&mut launcher_args, allowed_unix_socket_paths);
    launcher_args.push(program.to_string());
    launcher_args.extend(args.iter().cloned());
    (launcher_program.to_string(), launcher_args)
}

/// Encode a path list as a count followed by one argv entry per path.
fn push_path_list(launcher_args: &mut Vec<String>, paths: &[&Path]) {
    launcher_args.push(paths.len().to_string());
    for path in paths {
        launcher_args.push(path.to_string_lossy().into_owned());
    }
}

/// If this process was re-executed as a sandbox launcher (its first argument
/// is [`SANDBOX_LAUNCHER_FLAG`]), set up the sandbox and `exec` the wrapped
/// command. This never returns when the marker is present.
///
/// Call this at the very top of `main`, before any argument parsing: the
/// wrapped command's own arguments are appended verbatim and would otherwise
/// confuse an argument parser.
pub fn run_launcher_if_invoked() {
    let Some(invocation) = parse_launcher_args(std::env::args_os()) else {
        return;
    };
    let invocation = match invocation {
        Ok(invocation) => invocation,
        Err(error) => {
            eprintln!("zed: malformed sandbox launcher invocation: {error:#}");
            std::process::exit(127);
        }
    };
    run_launcher(invocation);
}

/// Exit code the launcher uses when it could not create the sandbox. Distinct
/// from a normal command's exit codes so the failure is unambiguous in logs;
/// the authoritative signal to the parent is the [`LauncherStatus`] datagram.
const SANDBOX_SETUP_FAILED_EXIT_CODE: i32 = 126;

/// Set up the sandbox and `exec` the command. Never returns.
///
/// If the sandbox cannot be created for any reason, this reports the reason to
/// the parent and **aborts** — it never runs the command unsandboxed. Falling
/// back to an unsandboxed run is a policy decision that belongs to the caller
/// (see [`check_can_create_sandbox`]), not to the sandbox itself.
fn run_launcher(invocation: LauncherInvocation) -> ! {
    let socket = invocation.status_socket.clone();
    let writable: Vec<&Path> = invocation
        .writable_dirs
        .iter()
        .map(PathBuf::as_path)
        .collect();
    let protected: Vec<&Path> = invocation
        .protected_paths
        .iter()
        .map(PathBuf::as_path)
        .collect();
    let allowed_unix_sockets: Vec<&Path> = invocation
        .allowed_unix_socket_paths
        .iter()
        .map(PathBuf::as_path)
        .collect();

    let (bwrap, bwrap_args) = match prepare_sandbox(
        &writable,
        &protected,
        &allowed_unix_sockets,
        invocation.permissions,
        invocation.cwd.as_deref(),
    ) {
        Ok(prepared) => prepared,
        Err(status) => {
            report_status(socket.as_deref(), status);
            eprintln!(
                "zed: could not create sandbox: {}; aborting",
                status.describe()
            );
            std::process::exit(SANDBOX_SETUP_FAILED_EXIT_CODE);
        }
    };

    // Everything is in place: report success, then `exec` the sandboxed command.
    // A failure of the final `exec` is fail-closed (a hard error, never a silent
    // unsandboxed run).
    report_status(socket.as_deref(), LauncherStatus::Success);
    let error = Command::new(&bwrap)
        .args(&bwrap_args)
        .arg("--")
        .arg(&invocation.program)
        .args(&invocation.args)
        .exec();
    eprintln!(
        "zed: failed to exec sandboxed command via bwrap: {error}; refusing to run unsandboxed"
    );
    std::process::exit(SANDBOX_SETUP_FAILED_EXIT_CODE);
}

/// Locate a usable `bwrap` and verify it can actually create a sandbox with the
/// exact arguments implied by the policy (by running `bwrap <args> -- true`).
///
/// Returns the `bwrap` path and the assembled argument list on success, or the
/// reason it can't on failure. Shared by the launcher (which then `exec`s) and
/// by [`check_can_create_sandbox`] (which the parent uses to decide whether to
/// fall back to an unsandboxed run).
fn prepare_sandbox(
    writable_dirs: &[&Path],
    protected_paths: &[&Path],
    allowed_unix_socket_paths: &[&Path],
    permissions: SandboxPermissions,
    cwd: Option<&Path>,
) -> Result<(PathBuf, Vec<String>), LauncherStatus> {
    let bwrap = match locate_bwrap() {
        BwrapLocation::Found(path) => path,
        BwrapLocation::OnlySetuid => return Err(LauncherStatus::SetuidRejected),
        BwrapLocation::NotFound => return Err(LauncherStatus::BwrapNotFound),
    };
    let bwrap_args = build_bwrap_args(
        writable_dirs,
        protected_paths,
        allowed_unix_socket_paths,
        permissions,
        cwd,
    );
    if !probe_bwrap(&bwrap, &bwrap_args) {
        return Err(LauncherStatus::SandboxProbeFailed);
    }
    Ok((bwrap, bwrap_args))
}

/// Check whether an OS sandbox can be created for this policy, returning the
/// reason it can't (as a [`LauncherStatus`]) on failure.
///
/// This runs the same locate + probe the launcher does, so a caller can decide
/// *before* spawning whether to run sandboxed, fall back to an unsandboxed run
/// (fail-open), or refuse (fail-closed). The launcher still performs its own
/// check and aborts on failure, so this is purely advisory — the sandbox is
/// never silently skipped on the strength of this result alone.
pub fn check_can_create_sandbox(
    writable_dirs: &[&Path],
    protected_paths: &[&Path],
    allowed_unix_socket_paths: &[&Path],
    permissions: SandboxPermissions,
    cwd: Option<&Path>,
) -> std::result::Result<(), LauncherStatus> {
    prepare_sandbox(
        writable_dirs,
        protected_paths,
        allowed_unix_socket_paths,
        permissions,
        cwd,
    )
    .map(|_| ())
}

/// Send a one-shot `status` datagram to the status channel (if one was given).
///
/// Best-effort: if the parent has gone away or the send fails there is nothing
/// useful to do, and we must never block the command on a diagnostic channel.
/// This runs in the launcher (host network namespace) before `exec`, so it is
/// unaffected by the command's own `--unshare-net` isolation.
fn report_status(socket_name: Option<&str>, status: LauncherStatus) {
    let Some(name) = socket_name else {
        return;
    };
    let Ok(address) = SocketAddr::from_abstract_name(name.as_bytes()) else {
        return;
    };
    let Ok(socket) = UnixDatagram::unbound() else {
        return;
    };
    // Ignore send errors: the report is advisory and the parent may have
    // already stopped listening.
    let _ = socket.send_to_addr(&[status.to_byte()], &address);
}

/// Decode launcher arguments produced by [`wrap_invocation`].
///
/// `args` is the full process argv (including argv[0]). Returns `None` when
/// this isn't a launcher invocation, or `Some(Err(_))` when the marker is
/// present but the encoding is malformed.
fn parse_launcher_args(
    args: impl IntoIterator<Item = OsString>,
) -> Option<Result<LauncherInvocation>> {
    let mut args = args.into_iter();
    // argv[0] is the executable; the marker, if any, is argv[1].
    args.next()?;
    if args.next()?.to_str() != Some(SANDBOX_LAUNCHER_FLAG) {
        return None;
    }
    Some(decode_launcher_args(args))
}

fn decode_launcher_args(mut args: impl Iterator<Item = OsString>) -> Result<LauncherInvocation> {
    let status_socket = {
        let value = args.next().context("missing status socket argument")?;
        let value = value
            .into_string()
            .map_err(|_| anyhow!("status socket name is not valid UTF-8"))?;
        if value.is_empty() { None } else { Some(value) }
    };
    let allow_network = decode_bool(&args.next().context("missing allow_network flag")?)?;
    let allow_fs_write = decode_bool(&args.next().context("missing allow_fs_write flag")?)?;
    let cwd = {
        let value = args.next().context("missing cwd argument")?;
        if value.is_empty() {
            None
        } else {
            Some(PathBuf::from(value))
        }
    };
    let writable_dirs = decode_path_list(&mut args, "writable directory")?;
    let protected_paths = decode_path_list(&mut args, "protected path")?;
    let allowed_unix_socket_paths = decode_path_list(&mut args, "allowed unix socket")?;
    let program = args.next().context("missing program to run")?;
    let args: Vec<OsString> = args.collect();

    Ok(LauncherInvocation {
        status_socket,
        permissions: SandboxPermissions {
            allow_network,
            allow_fs_write,
        },
        cwd,
        writable_dirs,
        protected_paths,
        allowed_unix_socket_paths,
        program,
        args,
    })
}

/// Decode a path list (a count followed by that many entries) produced by
/// [`push_path_list`]. `what` names the list for error messages.
fn decode_path_list(args: &mut impl Iterator<Item = OsString>, what: &str) -> Result<Vec<PathBuf>> {
    let count = args
        .next()
        .with_context(|| format!("missing {what} count"))?
        .to_str()
        .with_context(|| format!("{what} count is not valid UTF-8"))?
        .parse::<usize>()
        .with_context(|| format!("invalid {what} count"))?;
    let mut paths = Vec::with_capacity(count);
    for _ in 0..count {
        paths.push(PathBuf::from(
            args.next()
                .with_context(|| format!("missing {what} entry"))?,
        ));
    }
    Ok(paths)
}

fn encode_bool(value: bool) -> String {
    if value { "1" } else { "0" }.to_string()
}

fn decode_bool(value: &OsString) -> Result<bool> {
    match value.to_str() {
        Some("0") => Ok(false),
        Some("1") => Ok(true),
        other => bail!("invalid boolean launcher argument: {other:?}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn launcher_argv(launcher: String, args: Vec<String>) -> Vec<OsString> {
        std::iter::once(launcher)
            .chain(args)
            .map(OsString::from)
            .collect()
    }

    #[test]
    fn test_launcher_status_byte_round_trip() {
        for status in [
            LauncherStatus::Success,
            LauncherStatus::BwrapNotFound,
            LauncherStatus::SetuidRejected,
            LauncherStatus::SandboxProbeFailed,
        ] {
            assert_eq!(LauncherStatus::from_byte(status.to_byte()), Some(status));
        }
        assert_eq!(LauncherStatus::from_byte(200), None);
    }

    #[test]
    fn test_nearest_existing_ancestor() {
        let dir = tempfile::tempdir().unwrap();
        let existing = dir.path();
        let missing = existing.join("a").join("b").join("c.txt");

        assert_eq!(
            nearest_existing_ancestor(&missing).unwrap(),
            existing.to_path_buf()
        );
        assert_eq!(
            nearest_existing_ancestor(existing).unwrap(),
            existing.to_path_buf()
        );
    }

    #[test]
    fn test_build_bwrap_args_default_binds_writable_dirs_read_write() {
        let writable = tempfile::tempdir().unwrap();
        let args = build_bwrap_args(
            &[writable.path()],
            &[],
            &[],
            SandboxPermissions::default(),
            Some(writable.path()),
        );

        // Root is read-only, the writable dir is bound read-write, and (since
        // network is denied by default) the command gets its own net namespace.
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
        let denied = build_bwrap_args(&[], &[], &[], SandboxPermissions::default(), None);
        assert!(denied.iter().any(|arg| arg == "--unshare-net"));

        let allowed = build_bwrap_args(
            &[],
            &[],
            &[],
            SandboxPermissions {
                allow_network: true,
                allow_fs_write: false,
            },
            None,
        );
        assert!(!allowed.iter().any(|arg| arg == "--unshare-net"));
    }

    #[test]
    fn test_build_bwrap_args_allow_fs_write_binds_root_read_write() {
        let permissions = SandboxPermissions {
            allow_network: false,
            allow_fs_write: true,
        };
        let args = build_bwrap_args(&[], &[], &[], permissions, None);
        assert!(windows_contains(&args, &["--bind", "/", "/"]));
        assert!(!windows_contains(&args, &["--ro-bind", "/", "/"]));
        // With unrestricted writes the host's real `/tmp` stays writable, so
        // no ephemeral tmpfs is layered over it.
        assert!(!windows_contains(&args, &["--tmpfs", "/tmp"]));
    }

    #[test]
    fn test_build_bwrap_args_shadows_protected_directory_with_tmpfs() {
        let project = tempfile::tempdir().unwrap();
        let git_dir = project.path().join(".git");
        std::fs::create_dir(&git_dir).unwrap();

        let args = build_bwrap_args(
            &[project.path()],
            &[git_dir.as_path()],
            &[],
            SandboxPermissions::default(),
            None,
        );

        let git_canonical = git_dir.canonicalize().unwrap();
        let git_str = git_canonical.to_string_lossy().into_owned();
        // The writable project is bound rw, then the `.git` directory is shadowed
        // with a tmpfs on top so its real contents can't be read or written.
        assert!(windows_contains(&args, &["--tmpfs", &git_str]));

        let project_canonical = project.path().canonicalize().unwrap();
        let project_str = project_canonical.to_string_lossy().into_owned();
        let tmpfs_index = args
            .windows(2)
            .position(|w| w == ["--tmpfs".to_string(), git_str.clone()])
            .unwrap();
        let bind_index = args
            .windows(3)
            .position(|w| {
                w == [
                    "--bind".to_string(),
                    project_str.clone(),
                    project_str.clone(),
                ]
            })
            .unwrap();
        assert!(
            bind_index < tmpfs_index,
            "the protected tmpfs must be applied after the writable bind so it wins"
        );
    }

    #[test]
    fn test_build_bwrap_args_masks_protected_file_with_dev_null() {
        let project = tempfile::tempdir().unwrap();
        let gitlink = project.path().join(".git");
        std::fs::write(&gitlink, "gitdir: /elsewhere\n").unwrap();

        let args = build_bwrap_args(
            &[project.path()],
            &[gitlink.as_path()],
            &[],
            SandboxPermissions::default(),
            None,
        );

        let gitlink_canonical = gitlink.canonicalize().unwrap();
        let gitlink_str = gitlink_canonical.to_string_lossy().into_owned();
        assert!(windows_contains(
            &args,
            &["--ro-bind", "/dev/null", &gitlink_str]
        ));
    }

    #[test]
    fn test_build_bwrap_args_rebinds_allowed_unix_socket() {
        let dir = tempfile::tempdir().unwrap();
        // `build_bwrap_args` only checks that the path exists, so a regular file
        // stands in for a socket here (binding a real socket under a long temp
        // path would overflow `sun_path`).
        let socket = dir.path().join("agent.sock");
        std::fs::write(&socket, b"").unwrap();

        let args = build_bwrap_args(
            &[],
            &[],
            &[socket.as_path()],
            SandboxPermissions::default(),
            None,
        );

        let socket_canonical = socket.canonicalize().unwrap();
        let socket_str = socket_canonical.to_string_lossy().into_owned();
        assert!(windows_contains(
            &args,
            &["--bind", &socket_str, &socket_str]
        ));
    }

    #[test]
    fn test_launcher_args_round_trip() {
        let writable = PathBuf::from("/home/user/project dir");
        let cwd = PathBuf::from("/home/user/project dir/sub");
        let program = "/bin/sh".to_string();
        let args = vec!["-c".to_string(), "echo hi there".to_string()];
        let permissions = SandboxPermissions {
            allow_network: false,
            allow_fs_write: false,
        };

        let protected = PathBuf::from("/home/user/project dir/.git");
        let socket = PathBuf::from("/tmp/ssh agent/sock");
        let (launcher, launcher_args) = wrap_invocation(
            "/path/to/zed",
            Some("zed-sandbox-status-abc"),
            permissions,
            &[writable.as_path()],
            &[protected.as_path()],
            &[socket.as_path()],
            Some(cwd.as_path()),
            &program,
            &args,
        );
        assert_eq!(launcher, "/path/to/zed");

        let decoded = parse_launcher_args(launcher_argv(launcher, launcher_args))
            .expect("should be recognized as a launcher invocation")
            .expect("should decode successfully");

        assert_eq!(
            decoded.status_socket.as_deref(),
            Some("zed-sandbox-status-abc")
        );
        assert_eq!(decoded.permissions, permissions);
        assert_eq!(decoded.cwd.as_deref(), Some(cwd.as_path()));
        assert_eq!(decoded.writable_dirs, vec![writable]);
        assert_eq!(decoded.protected_paths, vec![protected]);
        assert_eq!(decoded.allowed_unix_socket_paths, vec![socket]);
        assert_eq!(decoded.program, OsString::from(program));
        assert_eq!(
            decoded.args,
            args.into_iter().map(OsString::from).collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_launcher_args_round_trip_minimal() {
        let program = "/bin/true".to_string();
        let permissions = SandboxPermissions {
            allow_network: true,
            allow_fs_write: true,
        };
        let (launcher, launcher_args) = wrap_invocation(
            "/path/to/zed",
            None,
            permissions,
            &[],
            &[],
            &[],
            None,
            &program,
            &[],
        );

        let decoded = parse_launcher_args(launcher_argv(launcher, launcher_args))
            .unwrap()
            .unwrap();

        assert_eq!(decoded.status_socket, None);
        assert_eq!(decoded.permissions, permissions);
        assert_eq!(decoded.cwd, None);
        assert!(decoded.writable_dirs.is_empty());
        assert!(decoded.protected_paths.is_empty());
        assert!(decoded.allowed_unix_socket_paths.is_empty());
        assert_eq!(decoded.program, OsString::from("/bin/true"));
        assert!(decoded.args.is_empty());
    }

    #[test]
    fn test_parse_launcher_args_ignores_non_launcher_argv() {
        let raw: Vec<OsString> = ["/path/to/zed", "--foo", "bar"]
            .into_iter()
            .map(OsString::from)
            .collect();
        assert!(parse_launcher_args(raw).is_none());
    }

    #[test]
    fn test_status_channel_round_trip() {
        // The launcher side (`report_status`) and parent side
        // (`StatusChannel::recv`) agree over a real abstract socket.
        let channel = StatusChannel::bind().unwrap();
        let name = channel.name().to_string();
        let reader = std::thread::spawn(move || channel.recv(Duration::from_secs(5)));
        report_status(Some(&name), LauncherStatus::SandboxProbeFailed);
        assert_eq!(
            reader.join().unwrap(),
            Some(LauncherStatus::SandboxProbeFailed)
        );
    }

    #[test]
    fn test_status_channel_times_out_without_report() {
        let channel = StatusChannel::bind().unwrap();
        assert_eq!(channel.recv(Duration::from_millis(50)), None);
    }

    /// Whether `args` contains `needle` as a contiguous run.
    fn windows_contains(args: &[String], needle: &[&str]) -> bool {
        args.windows(needle.len())
            .any(|window| window.iter().zip(needle).all(|(a, b)| a == b))
    }
}
