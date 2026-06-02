//! Linux Landlock sandbox integration.
//!
//! This module restricts a command using the Landlock LSM (Linux Security
//! Module), available since Linux 5.13. Unlike macOS's Seatbelt — which is
//! a launcher (`sandbox-exec`) that reads a policy file and execs the
//! target — Landlock has no launcher binary. A process restricts *itself*
//! (and its future children) by building a ruleset and calling
//! `restrict_self`. The restriction is irreversible and only ever narrows
//! access.
//!
//! Because of that model, the integration point is different from macOS. The
//! terminal integration can only rewrite a command into `(program, args)`,
//! so we use a *self-exec launcher*: [`wrap_invocation`] re-execs this very
//! binary with [`SANDBOX_LAUNCHER_FLAG`] and an encoded policy, and
//! [`run_launcher_if_invoked`] (called early in `main`) recognizes the
//! marker, applies the ruleset to itself via [`restrict_current_thread`],
//! and `exec`s the wrapped command. This mirrors macOS's launcher shape
//! while still using Landlock's restrict-self model.
//!
//! Doing the Landlock setup in the freshly re-exec'd, single-threaded
//! process (rather than in a `pre_exec` hook of multithreaded Zed) also
//! avoids running allocating code in a forked-but-not-yet-exec'd child,
//! which is the classic fork-in-a-threaded-program deadlock hazard.
//!
//! Reads are permitted everywhere by default; writes are restricted to a
//! caller-provided list of directories; network access (TCP bind/connect)
//! and unrestricted writes must be opted into per command.
//!
//! Landlock is best-effort here: features the running kernel lacks are
//! silently skipped (see [`landlock::CompatLevel::BestEffort`]), so on an
//! older kernel a command may be less restricted than requested — or, on a
//! kernel without Landlock at all, not restricted by Landlock. Callers that
//! need a hard guarantee should inspect the returned [`RestrictionStatus`].

use std::ffi::OsString;
use std::os::unix::process::CommandExt as _;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context as _, Result, bail};
use landlock::{
    ABI, Access, AccessFs, AccessNet, CompatLevel, Compatible as _, PathBeneath, PathFd,
    RestrictionStatus, Ruleset, RulesetAttr as _, RulesetCreatedAttr as _,
};

use crate::SandboxPermissions;

/// Landlock ABI level we target. The set of restrictions actually enforced
/// is the intersection of this and what the running kernel supports (we use
/// best-effort compatibility), so targeting a recent ABI is safe: newer
/// features simply get skipped on older kernels.
const ABI_TARGET: ABI = ABI::V5;

/// Whether Landlock is available *and enabled* on the running kernel — i.e.
/// whether [`restrict_current_thread`] would actually enforce restrictions
/// rather than silently being a no-op.
///
/// Checks the kernel's active LSM list (`/sys/kernel/security/lsm`) for
/// `landlock`. It won't appear there if Landlock isn't built into the kernel
/// or isn't enabled at boot. The answer can't change while the process runs,
/// so it's cached.
///
/// This is intended to be called right before attempting to sandbox a
/// command, so callers can fall back to running unsandboxed when it returns
/// `false`.
pub fn is_supported() -> bool {
    static SUPPORTED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *SUPPORTED.get_or_init(|| {
        std::fs::read_to_string("/sys/kernel/security/lsm")
            .map(|lsms| lsms.split(',').any(|lsm| lsm.trim() == "landlock"))
            .unwrap_or(false)
    })
}

/// Install a `pre_exec` hook on `command` that applies a Landlock ruleset
/// built from `permissions` to the child process before it execs.
///
/// The hook runs in the forked child, after `fork` and before `exec`, where
/// the process is single-threaded — the only context in which `restrict_self`
/// can fully protect the resulting program.
///
/// # Safety / behavior notes
/// * The hook runs between `fork` and `exec`. It only performs Landlock
///   syscalls and path opens, which is safe in that context.
/// * If applying the ruleset fails, the child fails to spawn and the error
///   surfaces from the eventual `Command::spawn`/`output` call.
/// * Landlock restrictions are best-effort; see the module docs.
///
/// # Arguments
/// * `command` - The command to restrict before it runs.
/// * `writable_directories` - Directory subtrees where the command is
///   allowed to write when `permissions.allow_fs_write` is false. Pass the
///   project's worktree paths here, not the working directory of the
///   command (the working directory is model-controlled, and using it as
///   the writable scope would let the model write outside the project).
/// * `permissions` - Sandbox relaxations requested for this command.
pub fn restrict_command(
    command: &mut Command,
    writable_directories: &[&Path],
    permissions: SandboxPermissions,
) {
    // `pre_exec` requires a `'static` closure, so own the paths up front.
    let writable_directories: Vec<PathBuf> = writable_directories
        .iter()
        .map(|path| path.to_path_buf())
        .collect();

    // SAFETY: the closure only performs Landlock syscalls and path opens,
    // which are valid to run in the post-`fork`, pre-`exec` child.
    unsafe {
        command.pre_exec(move || {
            let writable: Vec<&Path> = writable_directories.iter().map(PathBuf::as_path).collect();
            restrict_current_thread(&writable, permissions)
                .map(|_status| ())
                .map_err(std::io::Error::other)
        });
    }
}

/// Build a Landlock ruleset from `permissions` and apply it to the calling
/// thread (and its future children) via `restrict_self`.
///
/// This is irreversible: once applied, access can only be further narrowed.
/// It must be called from a single-threaded context (e.g. a `pre_exec`
/// hook) to protect the whole resulting process; other threads in a
/// multi-threaded process would remain unrestricted. Prefer
/// [`restrict_command`] for the common spawn-a-child case.
pub fn restrict_current_thread(
    writable_directories: &[&Path],
    permissions: SandboxPermissions,
) -> Result<RestrictionStatus> {
    let abi = ABI_TARGET;

    // Resolve each writable path to the nearest ancestor that exists, then
    // canonicalize it to resolve symlinks. A writable target may not have
    // been created yet (e.g. a command that creates `/tmp/foo.txt`), but
    // Landlock can only add a rule for a path it can `open()`. Granting the
    // rule on the closest existing parent lets the command create the path
    // beneath it — the same end effect as macOS Seatbelt's prefix-based
    // `subpath` rules, which already cover not-yet-created descendants.
    let canonical_writable_directories: Vec<PathBuf> = writable_directories
        .iter()
        .filter_map(|path| nearest_existing_ancestor(path))
        .map(|path| path.canonicalize().unwrap_or(path))
        .collect();

    // Handle every filesystem access right: anything not explicitly allowed
    // by a rule below is denied. With best-effort compatibility, rights the
    // running kernel doesn't know about are dropped instead of erroring.
    let mut ruleset = Ruleset::default()
        .set_compatibility(CompatLevel::BestEffort)
        .handle_access(AccessFs::from_all(abi))
        .context("failed to configure Landlock filesystem access")?;

    if !permissions.allow_network {
        // Handle network access too, and add no network rules below, so all
        // TCP bind/connect is denied.
        ruleset = ruleset
            .handle_access(AccessNet::from_all(abi))
            .context("failed to configure Landlock network access")?;
    }

    let mut ruleset = ruleset
        .create()
        .context("failed to create Landlock ruleset")?;

    // Allow reading from the entire filesystem.
    ruleset = ruleset
        .add_rule(PathBeneath::new(
            PathFd::new("/").context("failed to open / for Landlock")?,
            AccessFs::from_read(abi),
        ))
        .context("failed to add Landlock read rule for /")?;

    if permissions.allow_fs_write {
        // Allow writing anywhere by granting full access beneath /.
        ruleset = ruleset
            .add_rule(PathBeneath::new(
                PathFd::new("/").context("failed to open / for Landlock")?,
                AccessFs::from_all(abi),
            ))
            .context("failed to add Landlock write rule for /")?;
    } else {
        for directory in &canonical_writable_directories {
            let path_fd = PathFd::new(directory).with_context(|| {
                format!(
                    "failed to open writable directory for Landlock: {}",
                    directory.display()
                )
            })?;
            ruleset = ruleset
                .add_rule(PathBeneath::new(path_fd, AccessFs::from_all(abi)))
                .with_context(|| {
                    format!(
                        "failed to add Landlock write rule for {}",
                        directory.display()
                    )
                })?;
        }
    }

    ruleset
        .restrict_self()
        .context("failed to apply Landlock ruleset")
}

/// Return the nearest ancestor of `path` (including `path` itself) that
/// exists on disk, or `None` if nothing up to the root exists.
///
/// Used to grant a Landlock rule for a writable target that may not have
/// been created yet: the rule is added on the closest existing parent, and
/// Landlock's subtree semantics extend it to descendants created later.
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
/// Landlock sandbox launcher (see [`wrap_invocation`]). It's intentionally
/// unlikely to collide with a real argument.
pub const SANDBOX_LAUNCHER_FLAG: &str = "--zed-linux-landlock-exec";

/// A decoded launcher invocation: the policy plus the wrapped command.
struct LauncherInvocation {
    permissions: SandboxPermissions,
    writable_directories: Vec<PathBuf>,
    program: OsString,
    args: Vec<OsString>,
}

/// Build a self-exec launcher invocation.
///
/// Returns `(launcher_program, launcher_args)` where the first element is
/// `launcher_program` (typically this process's own executable) and the
/// args encode `permissions`, `writable_directories`, and the wrapped
/// `program`/`args`. When the resulting command is run, the binary must call
/// [`run_launcher_if_invoked`] early in `main` so it recognizes the marker,
/// applies the Landlock ruleset to itself, and `exec`s the wrapped command.
///
/// The encoding is positional: `[FLAG, network, fs_write, count, paths*,
/// program, args*]`. Each element is its own argv entry, so paths and args
/// containing spaces or separators round-trip without escaping.
///
/// See [`restrict_current_thread`] for the meaning of `writable_directories`.
pub fn wrap_invocation(
    launcher_program: &str,
    program: &str,
    args: &[String],
    writable_directories: &[&Path],
    permissions: SandboxPermissions,
) -> (String, Vec<String>) {
    let mut launcher_args = Vec::with_capacity(args.len() + writable_directories.len() + 5);
    launcher_args.push(SANDBOX_LAUNCHER_FLAG.to_string());
    launcher_args.push(encode_bool(permissions.allow_network));
    launcher_args.push(encode_bool(permissions.allow_fs_write));
    launcher_args.push(writable_directories.len().to_string());
    for directory in writable_directories {
        launcher_args.push(directory.to_string_lossy().into_owned());
    }
    launcher_args.push(program.to_string());
    launcher_args.extend(args.iter().cloned());
    (launcher_program.to_string(), launcher_args)
}

/// If this process was re-executed as a Landlock sandbox launcher (its first
/// argument is [`SANDBOX_LAUNCHER_FLAG`]), apply the encoded ruleset to the
/// current process and `exec` the wrapped command.
///
/// On success this never returns — `exec` replaces the process image. On any
/// failure it prints the error and exits non-zero rather than running the
/// command unsandboxed. If the marker is absent it returns immediately, so
/// normal startup proceeds.
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
            eprintln!("zed: failed to apply Landlock sandbox: {error:#}");
            std::process::exit(127);
        }
    }
}

/// `exec` replaces the process fully, so control flow never returns from
/// [`run_launcher`], except in the error case
enum Uninhabited {}

/// Apply the ruleset described by `invocation` to this process, then `exec`
/// the wrapped command. Returns only on error (`exec` is in-process, so the
/// restriction applies to the replacing image).
fn run_launcher(invocation: Result<LauncherInvocation>) -> Result<Uninhabited> {
    let invocation = invocation?;
    let writable: Vec<&Path> = invocation
        .writable_directories
        .iter()
        .map(PathBuf::as_path)
        .collect();
    restrict_current_thread(&writable, invocation.permissions)?;
    let error = Command::new(&invocation.program)
        .args(&invocation.args)
        .exec();
    Err(error).with_context(|| {
        format!(
            "failed to exec wrapped command: {}",
            invocation.program.to_string_lossy()
        )
    })
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
    let allow_network = decode_bool(&args.next().context("missing allow_network flag")?)?;
    let allow_fs_write = decode_bool(&args.next().context("missing allow_fs_write flag")?)?;
    let count: usize = args
        .next()
        .context("missing writable path count")?
        .to_str()
        .context("writable path count is not valid UTF-8")?
        .parse()
        .context("writable path count is not a number")?;

    let mut writable_directories = Vec::with_capacity(count);
    for _ in 0..count {
        writable_directories.push(PathBuf::from(args.next().context("missing writable path")?));
    }

    let program = args.next().context("missing wrapped program")?;
    let args: Vec<OsString> = args.collect();

    Ok(LauncherInvocation {
        permissions: SandboxPermissions {
            allow_network,
            allow_fs_write,
        },
        writable_directories,
        program,
        args,
    })
}

fn encode_bool(value: bool) -> String {
    if value { "1" } else { "0" }.to_string()
}

fn decode_bool(value: &OsString) -> Result<bool> {
    match value.to_str() {
        Some("1") => Ok(true),
        Some("0") => Ok(false),
        other => bail!("invalid boolean flag in launcher args: {other:?}"),
    }
}

#[cfg(test)]
#[allow(
    clippy::disallowed_methods,
    reason = "tests spawn child processes synchronously to verify the Landlock restrictions"
)]
mod tests {
    use super::*;
    use landlock::RulesetStatus;
    use std::process::Command;

    /// Install a `pre_exec` hook that applies the ruleset to the child before
    /// it execs. This is only sound because the test child does nothing
    /// between fork and exec but call Landlock; production code uses the
    /// re-exec launcher instead to avoid the fork-in-threaded-program hazard.
    fn restrict_in_child(
        command: &mut Command,
        writable_directories: &[&Path],
        permissions: SandboxPermissions,
    ) {
        let writable: Vec<PathBuf> = writable_directories
            .iter()
            .map(|path| path.to_path_buf())
            .collect();
        unsafe {
            command.pre_exec(move || {
                let writable: Vec<&Path> = writable.iter().map(PathBuf::as_path).collect();
                restrict_current_thread(&writable, permissions)
                    .map(|_status| ())
                    .map_err(std::io::Error::other)
            });
        }
    }

    #[test]
    fn test_read_access_is_a_subset_of_full_access() {
        let read = AccessFs::from_read(ABI_TARGET);
        let all = AccessFs::from_all(ABI_TARGET);
        assert!(all.contains(read));
    }

    #[test]
    fn test_default_permissions_are_fully_sandboxed() {
        assert_eq!(
            SandboxPermissions::default(),
            SandboxPermissions {
                allow_network: false,
                allow_fs_write: false,
            }
        );
    }

    #[test]
    fn test_denies_write_outside_writable_directories_by_default() {
        let writable = tempfile::tempdir().unwrap();
        let forbidden = tempfile::tempdir().unwrap();
        let forbidden_file = forbidden.path().join("denied");

        let mut command = Command::new("/bin/sh");
        command
            .arg("-c")
            .arg(format!("echo nope > {}", forbidden_file.to_str().unwrap()));
        restrict_in_child(
            &mut command,
            &[writable.path()],
            SandboxPermissions::default(),
        );

        let status = command.status().unwrap();
        assert!(
            !status.success(),
            "expected write outside writable dir to be denied"
        );
        assert!(!forbidden_file.exists());
    }

    #[test]
    fn test_allows_write_to_writable_directory_by_default() {
        let writable = tempfile::tempdir().unwrap();
        let allowed_file = writable.path().join("allowed");

        let mut command = Command::new("/bin/sh");
        command
            .arg("-c")
            .arg(format!("echo ok > {}", allowed_file.to_str().unwrap()));
        restrict_in_child(
            &mut command,
            &[writable.path()],
            SandboxPermissions::default(),
        );

        let status = command.status().unwrap();
        assert!(
            status.success(),
            "expected write to writable dir to succeed"
        );
        assert!(allowed_file.exists());
    }

    #[test]
    fn test_allows_creating_a_writable_path_that_does_not_exist_yet() {
        // Grant write access to a file that does not exist yet, and confirm
        // the command can create it. Without resolving to the nearest
        // existing ancestor, `PathFd::new` on the missing path would fail and
        // the whole ruleset build would error.
        let writable = tempfile::tempdir().unwrap();
        let target = writable.path().join("created.txt");
        assert!(!target.exists());

        let mut command = Command::new("/bin/sh");
        command
            .arg("-c")
            .arg(format!("echo ok > {}", target.to_str().unwrap()));
        // Pass the not-yet-created file path itself as the writable target.
        restrict_in_child(
            &mut command,
            &[target.as_path()],
            SandboxPermissions::default(),
        );

        let status = command.status().unwrap();
        assert!(
            status.success(),
            "expected creating a granted, not-yet-existing path to succeed"
        );
        assert!(target.exists());
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
    fn test_restrict_current_thread_reports_status() {
        // Apply the ruleset in a forked child so the test process itself
        // stays unrestricted, and assert it was actually enforced.
        let writable = tempfile::tempdir().unwrap();
        let mut command = Command::new("/bin/sh");
        command.arg("-c").arg("exit 0");
        let writable_path = writable.path().to_path_buf();
        // SAFETY: only Landlock syscalls run between fork and exec.
        unsafe {
            command.pre_exec(move || {
                let status = restrict_current_thread(
                    &[writable_path.as_path()],
                    SandboxPermissions::default(),
                )
                .map_err(std::io::Error::other)?;
                if status.ruleset == RulesetStatus::NotEnforced {
                    return Err(std::io::Error::other("Landlock ruleset not enforced"));
                }
                Ok(())
            });
        }

        assert!(command.status().unwrap().success());
    }

    fn launcher_argv(launcher: String, args: Vec<String>) -> Vec<OsString> {
        // The real process argv is argv[0] (the launcher program) followed by
        // the encoded launcher args.
        std::iter::once(launcher)
            .chain(args)
            .map(OsString::from)
            .collect()
    }

    #[test]
    fn test_launcher_args_round_trip() {
        let permissions = SandboxPermissions {
            allow_network: true,
            allow_fs_write: false,
        };
        let (launcher, args) = wrap_invocation(
            "/path/to/zed",
            "/bin/sh",
            &["-c".to_string(), "echo hi there".to_string()],
            &[Path::new("/project a"), Path::new("/tmp/scratch")],
            permissions,
        );
        assert_eq!(launcher, "/path/to/zed");

        let raw = launcher_argv(launcher, args);
        let decoded = parse_launcher_args(raw)
            .expect("should be recognized as a launcher invocation")
            .expect("should decode successfully");

        assert_eq!(decoded.permissions, permissions);
        assert_eq!(
            decoded.writable_directories,
            vec![PathBuf::from("/project a"), PathBuf::from("/tmp/scratch")]
        );
        assert_eq!(decoded.program, OsString::from("/bin/sh"));
        assert_eq!(
            decoded.args,
            vec![OsString::from("-c"), OsString::from("echo hi there")]
        );
    }

    #[test]
    fn test_launcher_args_round_trip_with_no_writable_dirs_and_fs_write() {
        let permissions = SandboxPermissions {
            allow_network: false,
            allow_fs_write: true,
        };
        let (launcher, args) = wrap_invocation("/path/to/zed", "/bin/true", &[], &[], permissions);

        let raw = launcher_argv(launcher, args);
        let decoded = parse_launcher_args(raw).unwrap().unwrap();

        assert_eq!(decoded.permissions, permissions);
        assert!(decoded.writable_directories.is_empty());
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
}
