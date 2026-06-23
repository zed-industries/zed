//! Per-OS sandbox integrations for terminal commands run on behalf of the
//! agent.
//!
//! Each supported operating system has its own module here, gated behind
//! its `target_os` cfg so callers reach for the right one explicitly and
//! non-host targets don't carry dead code.
//!
//! macOS has an integration ([`macos_seatbelt`]) wrapping Apple's Seatbelt
//! / `sandbox-exec` framework, and Linux has one ([`linux_bubblewrap`]) built
//! on Bubblewrap (`bwrap`) for both the filesystem and the network. Windows
//! routes commands through WSL and runs them under Bubblewrap there (see
//! [`windows_wsl`]).

#[cfg(target_os = "linux")]
pub mod linux_bubblewrap;

#[cfg(target_os = "macos")]
pub mod macos_seatbelt;

#[cfg(target_os = "windows")]
pub mod windows_wsl;

/// Marker prefix for [`windows_wsl`] errors that mean the sandboxing
/// *environment* is unavailable (WSL missing or failing to start, no usable
/// `bwrap`, the probe/path-resolution protocol breaking down) — as opposed
/// to per-request errors such as a writable path that doesn't exist, which
/// never carry this prefix.
///
/// The agent matches on this prefix to decide whether a failed sandboxed
/// command should offer the user the option of turning sandboxing off
/// (an environment that can't sandbox at all) or simply report the error
/// back to the model (a fixable bad request). Defined here rather than in
/// [`windows_wsl`] so non-Windows builds of the agent can still reference
/// it.
pub const WSL_SANDBOX_UNAVAILABLE_PREFIX: &str = "Windows sandboxing via WSL is unavailable";

/// Per-command relaxations of the default Bubblewrap (Linux) sandbox.
///
/// All-false is the default, fully-sandboxed run. Setting any field
/// requires user approval before the command is launched.
///
/// Network access is a plain on/off toggle here because Bubblewrap can only
/// enforce it wholesale (an `--unshare-net` namespace, loopback only). macOS
/// can additionally confine egress to an allowlist via Seatbelt and the
/// in-process proxy, so it uses its own richer
/// [`macos_seatbelt::SandboxPermissions`] instead of this type. Windows reuses
/// this type, mapping it onto Bubblewrap inside WSL. Some baseline operations
/// remain denied regardless of these flags; the only way to lift those is to
/// skip the sandbox entirely, which these integrations deliberately don't
/// expose.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SandboxPermissions {
    /// Allow network access for the command.
    pub allow_network: bool,
    /// Allow unrestricted filesystem writes.
    pub allow_fs_write: bool,
}

/// Handle a possible re-exec of this binary as a sandbox launcher.
///
/// On Linux, the terminal integration sandboxes commands by re-executing
/// this binary as a launcher (see
/// [`linux_bubblewrap::run_launcher_if_invoked`]); when that marker is present
/// this sets up the `bwrap` sandbox and `exec`s the wrapped command, never
/// returning. On every other platform, and for normal launches, it returns
/// immediately.
///
/// Call this at the very top of `main`, before any argument parsing.
pub fn run_sandbox_launcher_if_invoked() {
    #[cfg(target_os = "linux")]
    linux_bubblewrap::run_launcher_if_invoked();
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
#[cfg(any(target_os = "linux", target_os = "macos"))]
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

#[cfg(all(test, any(target_os = "linux", target_os = "macos")))]
mod tests {
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
