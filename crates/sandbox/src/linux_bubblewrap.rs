//! Linux sandbox integration built on Bubblewrap (`bwrap`) for the
//! filesystem and seccomp for the network.
//!
//! `bwrap` is an unprivileged container launcher: it sets up mount, user,
//! pid, and other namespaces, bind-mounts the host filesystem (read-only by
//! default, read-write for an explicit set of directories), then `exec`s the
//! wrapped command inside that view. Unlike Landlock, its capabilities don't
//! scale with the kernel ABI — any kernel with unprivileged user namespaces
//! (~3.8+) gives the same filesystem isolation — at the cost of needing a
//! `bwrap` binary present and usable.
//!
//! `bwrap` only restricts the filesystem; it does not, on its own, give us
//! fine-grained network control. We deliberately keep the host network
//! namespace (so local `AF_UNIX` IPC keeps working) and instead deny outbound
//! IP networking with a seccomp filter that blocks creation of `AF_INET` /
//! `AF_INET6` sockets. Gating at socket *creation* is the only reliable point:
//! `connect`/`sendto` operate on an already-open fd whose address family is
//! kernel state seccomp can't inspect, and blocking them unconditionally would
//! also break the `AF_UNIX` sockets we want to allow.
//!
//! ## The launcher
//!
//! The PTY layer Zed spawns terminal commands through only lets us control the
//! program, argv, and env of the child — not its file descriptors and not a
//! `pre_exec` hook. seccomp must therefore be installed by a process we fully
//! control *between* the spawn and the command, so we re-exec this very binary
//! as a launcher: [`wrap_invocation`] encodes the network policy and the full
//! command to run, and [`run_launcher_if_invoked`] (called early in `main`)
//! recognizes the marker, installs the seccomp filter, and `exec`s the wrapped
//! command. Doing this in the freshly re-exec'd, single-threaded process also
//! sidesteps the fork-in-a-threaded-program hazards of a `pre_exec` hook.
//!
//! The launcher payload is intentionally generic: "install seccomp per the
//! encoded network policy, then `exec` everything after the first `--`." It
//! doesn't know about `bwrap` specifically — [`apply_sandbox_wrap`] assembles
//! the full `bwrap` command line and hands it over as the command to exec.
//! That keeps the launcher reusable as the network policy grows to need, for
//! example, a network namespace plus an egress proxy.
//!
//! [`apply_sandbox_wrap`]: ../../acp_thread/terminal/fn.apply_sandbox_wrap.html

use std::ffi::OsString;
use std::os::unix::fs::MetadataExt as _;
use std::os::unix::process::CommandExt as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::OnceLock;

use anyhow::{Context as _, Result, anyhow, bail, ensure};
use seccompiler::{
    BpfProgram, SeccompAction, SeccompCmpArgLen, SeccompCmpOp, SeccompCondition, SeccompFilter,
    SeccompRule,
};

use crate::SandboxPermissions;

/// The network policy enforced for a sandboxed command.
///
/// Kept as a small enum rather than a bool so richer policies (for example,
/// routing egress through a proxy that allows only certain hosts) can be added
/// as new variants without changing the launcher's encoding shape.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NetworkPolicy {
    /// Outbound IP networking is allowed; no seccomp filter is installed.
    Allowed,
    /// Outbound IP networking is denied (`AF_UNIX` IPC still works).
    Denied,
}

impl NetworkPolicy {
    fn from_permissions(permissions: SandboxPermissions) -> Self {
        if permissions.allow_network {
            NetworkPolicy::Allowed
        } else {
            NetworkPolicy::Denied
        }
    }
}

/// Locate a usable, non-setuid `bwrap` binary, preferring one on `PATH` and
/// falling back to a binary bundled with the application.
///
/// The result is cached: the answer can't change while the process runs, and
/// the lookup includes filesystem `stat`s we'd rather not repeat per command.
///
/// This does not probe that the binary actually *enforces* a sandbox — see
/// [`is_available`] for that.
pub fn locate_bwrap() -> Option<PathBuf> {
    static LOCATED: OnceLock<Option<PathBuf>> = OnceLock::new();
    LOCATED
        .get_or_init(|| {
            candidate_bwrap_paths()
                .into_iter()
                .find(|candidate| candidate.is_file() && !is_setuid_root(candidate))
        })
        .clone()
}

/// Whether a usable `bwrap` binary is present *and* actually enforces a
/// sandbox on this system, cached for the life of the process.
///
/// Presence isn't enough: unprivileged user namespaces can be disabled by the
/// kernel or by an AppArmor/sysctl policy, in which case `bwrap` exists but
/// fails to set up its namespaces. We confirm with a real smoke test
/// ([`probe_bwrap`]) rather than trusting distro or version detection. When
/// this returns `false`, callers should run the command unsandboxed (with a
/// visible warning) instead of failing.
pub fn is_available() -> bool {
    static AVAILABLE: OnceLock<bool> = OnceLock::new();
    *AVAILABLE.get_or_init(|| locate_bwrap().is_some_and(|bwrap| probe_bwrap(&bwrap)))
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
/// We refuse to use (or even probe) such a binary: with our `NO_NEW_PRIVS` +
/// seccomp design a setuid `bwrap` buys no functionality, and *running* one
/// would mean executing root-privileged setup with argv partly derived from
/// model-influenced input — a privilege-escalation surface. Rejecting it keeps
/// the whole pipeline unprivileged and means a planted setuid binary earlier
/// on `PATH` can't trick us into executing it.
fn is_setuid_root(path: &Path) -> bool {
    match std::fs::metadata(path) {
        Ok(metadata) => (metadata.mode() & libc::S_ISUID != 0) && metadata.uid() == 0,
        Err(_) => false,
    }
}

/// Run a minimal `bwrap` invocation and report whether it succeeded, i.e.
/// whether this environment can actually set up an unprivileged sandbox.
#[allow(
    clippy::disallowed_methods,
    reason = "a one-time, cached probe that must block briefly to read the child's exit status"
)]
fn probe_bwrap(bwrap: &Path) -> bool {
    Command::new(bwrap)
        .args([
            "--unshare-user",
            "--ro-bind",
            "/",
            "/",
            "--dev",
            "/dev",
            "--",
            "true",
        ])
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
/// and these overlays/binds are omitted. The network namespace is
/// deliberately left shared — network restriction is handled by seccomp.
///
/// `writable_directories` should be the project's worktree paths (plus any
/// user-approved paths), *not* the command's working directory, which is
/// model-controlled and would let the model widen its own writable scope.
pub fn build_bwrap_args(
    writable_directories: &[&Path],
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

/// Separator between the launcher's own arguments and the command to `exec`.
const ARG_SEPARATOR: &str = "--";

/// A decoded launcher invocation: the network policy plus the command to run.
struct LauncherInvocation {
    network_policy: NetworkPolicy,
    /// The full argv to `exec` (typically `bwrap`, its arguments, `--`, and
    /// then the wrapped program and its arguments).
    command: Vec<OsString>,
}

/// Build a self-exec launcher invocation.
///
/// Returns `(launcher_program, launcher_args)`, where running
/// `launcher_program` with `launcher_args` re-execs this binary as the
/// launcher. `command` is the full argv to run inside the sandbox; the caller
/// assembles it (e.g. `[bwrap, bwrap_args.., --, program, args..]`).
///
/// The encoding is positional — `[FLAG, network_policy, --, command..]` — with
/// each element its own argv entry, so paths and arguments containing spaces
/// round-trip without escaping.
pub fn wrap_invocation(
    launcher_program: &str,
    network_policy: NetworkPolicy,
    command: &[String],
) -> (String, Vec<String>) {
    let mut launcher_args = Vec::with_capacity(command.len() + 3);
    launcher_args.push(SANDBOX_LAUNCHER_FLAG.to_string());
    launcher_args.push(encode_network_policy(network_policy));
    launcher_args.push(ARG_SEPARATOR.to_string());
    launcher_args.extend(command.iter().cloned());
    (launcher_program.to_string(), launcher_args)
}

/// If this process was re-executed as a sandbox launcher (its first argument
/// is [`SANDBOX_LAUNCHER_FLAG`]), install the encoded seccomp policy and
/// `exec` the wrapped command.
///
/// On success this never returns — `exec` replaces the process image. On any
/// failure it prints the error and exits non-zero rather than running the
/// command unsandboxed. If the marker is absent it returns immediately.
///
/// Call this at the very top of `main`, before any argument parsing: the
/// wrapped command's own arguments are appended verbatim and would otherwise
/// confuse an argument parser.
pub fn run_launcher_if_invoked() {
    let Some(invocation) = parse_launcher_args(std::env::args_os()) else {
        return;
    };

    match run_launcher(invocation) {
        Err(error) => {
            eprintln!("zed: failed to apply sandbox: {error:#}");
            std::process::exit(127);
        }
    }
}

/// `exec` replaces the process fully, so control flow never returns from
/// [`run_launcher`] except in the error case.
enum Uninhabited {}

/// Install the seccomp policy described by `invocation`, then `exec` the
/// wrapped command. Returns only on error (`exec` is in-process, so the filter
/// applies to the replacing image and its descendants).
fn run_launcher(invocation: Result<LauncherInvocation>) -> Result<Uninhabited> {
    let invocation = invocation?;
    install_seccomp(invocation.network_policy)?;
    let (program, args) = invocation
        .command
        .split_first()
        .context("launcher invocation has no command to exec")?;
    let error = Command::new(program).args(args).exec();
    Err(error).with_context(|| {
        format!(
            "failed to exec sandboxed command: {}",
            program.to_string_lossy()
        )
    })
}

/// Install the seccomp filter for `policy` on the calling thread.
///
/// `NetworkPolicy::Allowed` installs nothing. `NetworkPolicy::Denied` sets
/// `PR_SET_NO_NEW_PRIVS` (required to load a filter unprivileged) and applies
/// the network-denial BPF program. The filter survives `exec`, so it covers
/// `bwrap` and the wrapped command; it must be installed from the launcher's
/// single thread so it protects the whole resulting process.
fn install_seccomp(policy: NetworkPolicy) -> Result<()> {
    match policy {
        NetworkPolicy::Allowed => Ok(()),
        NetworkPolicy::Denied => {
            set_no_new_privs()?;
            let program = network_seccomp_program()?;
            seccompiler::apply_filter(&program).context("failed to apply seccomp filter")?;
            Ok(())
        }
    }
}

fn set_no_new_privs() -> Result<()> {
    // SAFETY: `prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0)` only sets a process
    // flag; the trailing arguments are ignored for this option.
    let result = unsafe { libc::prctl(libc::PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) };
    if result != 0 {
        return Err(std::io::Error::last_os_error())
            .context("failed to set PR_SET_NO_NEW_PRIVS");
    }
    Ok(())
}

/// Compile a seccomp program that denies outbound IP networking.
///
/// The default action is `Allow`; matched syscalls return `EPERM`. We deny
/// creating `AF_INET`/`AF_INET6` sockets (via `socket`/`socketpair`), which
/// makes IP `connect`/`sendto` impossible while leaving `AF_UNIX` (and other
/// families like `AF_NETLINK`) untouched. We also deny `ptrace`,
/// `process_vm_readv`/`writev`, and the `io_uring_*` syscalls, which could
/// otherwise be used to perform network operations or tamper with another
/// process to bypass the filter.
fn network_seccomp_program() -> Result<BpfProgram> {
    let deny_inet = |syscall: i64| -> Result<(i64, Vec<SeccompRule>)> {
        let rules = vec![
            inet_family_rule(libc::AF_INET)?,
            inet_family_rule(libc::AF_INET6)?,
        ];
        Ok((syscall, rules))
    };

    let mut rules: std::collections::BTreeMap<i64, Vec<SeccompRule>> =
        [deny_inet(libc::SYS_socket)?, deny_inet(libc::SYS_socketpair)?]
            .into_iter()
            .collect();

    // An empty rule vector matches the syscall unconditionally.
    for syscall in [
        libc::SYS_ptrace,
        libc::SYS_process_vm_readv,
        libc::SYS_process_vm_writev,
        libc::SYS_io_uring_setup,
        libc::SYS_io_uring_enter,
        libc::SYS_io_uring_register,
    ] {
        rules.insert(syscall, Vec::new());
    }

    let filter = SeccompFilter::new(
        rules,
        SeccompAction::Allow,
        SeccompAction::Errno(libc::EPERM as u32),
        std::env::consts::ARCH
            .try_into()
            .map_err(|error| anyhow!("unsupported seccomp target architecture: {error:?}"))?,
    )
    .context("failed to build seccomp filter")?;

    filter
        .try_into()
        .context("failed to compile seccomp filter")
}

/// A rule matching a `socket`/`socketpair` call whose address family (argument
/// 0) equals `family`.
fn inet_family_rule(family: libc::c_int) -> Result<SeccompRule> {
    SeccompRule::new(vec![
        SeccompCondition::new(
            0,
            SeccompCmpArgLen::Dword,
            SeccompCmpOp::Eq,
            family as u64,
        )
        .context("failed to build seccomp condition")?,
    ])
    .context("failed to build seccomp rule")
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
    let network_policy =
        decode_network_policy(&args.next().context("missing network policy")?)?;
    let separator = args.next().context("missing argument separator")?;
    ensure!(
        separator.to_str() == Some(ARG_SEPARATOR),
        "expected `{ARG_SEPARATOR}` separator in launcher args, got {separator:?}"
    );
    let command: Vec<OsString> = args.collect();
    ensure!(!command.is_empty(), "launcher invocation has no command");
    Ok(LauncherInvocation {
        network_policy,
        command,
    })
}

fn encode_network_policy(policy: NetworkPolicy) -> String {
    match policy {
        NetworkPolicy::Allowed => "allow",
        NetworkPolicy::Denied => "deny",
    }
    .to_string()
}

fn decode_network_policy(value: &OsString) -> Result<NetworkPolicy> {
    match value.to_str() {
        Some("allow") => Ok(NetworkPolicy::Allowed),
        Some("deny") => Ok(NetworkPolicy::Denied),
        other => bail!("invalid network policy in launcher args: {other:?}"),
    }
}

/// Convenience: build the network policy for a set of permissions.
pub fn network_policy_for(permissions: SandboxPermissions) -> NetworkPolicy {
    NetworkPolicy::from_permissions(permissions)
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
    fn test_network_policy_from_permissions() {
        assert_eq!(
            network_policy_for(SandboxPermissions::default()),
            NetworkPolicy::Denied
        );
        assert_eq!(
            network_policy_for(SandboxPermissions {
                allow_network: true,
                allow_fs_write: false,
            }),
            NetworkPolicy::Allowed
        );
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
            SandboxPermissions::default(),
            Some(writable.path()),
        );

        // Root is read-only, the writable dir is bound read-write, and the net
        // namespace is left shared (no `--unshare-net`).
        assert!(windows_contains(&args, &["--ro-bind", "/", "/"]));
        let writable_path = writable.path().canonicalize().unwrap();
        let writable_str = writable_path.to_string_lossy().into_owned();
        assert!(windows_contains(
            &args,
            &["--bind", &writable_str, &writable_str]
        ));
        assert!(windows_contains(&args, &["--tmpfs", "/tmp"]));
        assert!(args.iter().any(|arg| arg == "--chdir"));
        assert!(!args.iter().any(|arg| arg == "--unshare-net"));
    }

    #[test]
    fn test_build_bwrap_args_allow_fs_write_binds_root_read_write() {
        let permissions = SandboxPermissions {
            allow_network: false,
            allow_fs_write: true,
        };
        let args = build_bwrap_args(&[], permissions, None);
        assert!(windows_contains(&args, &["--bind", "/", "/"]));
        assert!(!windows_contains(&args, &["--ro-bind", "/", "/"]));
        // With unrestricted writes the host's real `/tmp` stays writable, so
        // no ephemeral tmpfs is layered over it.
        assert!(!windows_contains(&args, &["--tmpfs", "/tmp"]));
    }

    #[test]
    fn test_launcher_args_round_trip() {
        let command = vec![
            "/usr/bin/bwrap".to_string(),
            "--ro-bind".to_string(),
            "/".to_string(),
            "/".to_string(),
            "--".to_string(),
            "/bin/sh".to_string(),
            "-c".to_string(),
            "echo hi there".to_string(),
        ];
        let (launcher, args) =
            wrap_invocation("/path/to/zed", NetworkPolicy::Denied, &command);
        assert_eq!(launcher, "/path/to/zed");

        let raw = launcher_argv(launcher, args);
        let decoded = parse_launcher_args(raw)
            .expect("should be recognized as a launcher invocation")
            .expect("should decode successfully");

        assert_eq!(decoded.network_policy, NetworkPolicy::Denied);
        assert_eq!(
            decoded.command,
            command.into_iter().map(OsString::from).collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_launcher_args_round_trip_allow_network() {
        let command = vec!["/bin/true".to_string()];
        let (launcher, args) =
            wrap_invocation("/path/to/zed", NetworkPolicy::Allowed, &command);

        let raw = launcher_argv(launcher, args);
        let decoded = parse_launcher_args(raw).unwrap().unwrap();

        assert_eq!(decoded.network_policy, NetworkPolicy::Allowed);
        assert_eq!(decoded.command, vec![OsString::from("/bin/true")]);
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
    fn test_network_seccomp_program_compiles() {
        // Compiling exercises arch detection and rule assembly without needing
        // to actually install the filter (which would restrict the test
        // process).
        assert!(!network_seccomp_program().unwrap().is_empty());
    }

    /// Whether `args` contains `needle` as a contiguous run.
    fn windows_contains(args: &[String], needle: &[&str]) -> bool {
        args.windows(needle.len())
            .any(|window| window.iter().zip(needle).all(|(a, b)| a == b))
    }
}
