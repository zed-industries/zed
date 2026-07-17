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
use std::os::fd::AsRawFd as _;

#[cfg(target_os = "linux")]
mod linux_bubblewrap;

#[cfg(target_os = "macos")]
mod macos_seatbelt;

#[cfg(target_os = "windows")]
mod windows_wsl;

#[cfg(target_os = "windows")]
pub(crate) const WSL_SANDBOX_UNAVAILABLE_PREFIX: &str = "Windows sandboxing via WSL is unavailable";

/// An opaque handle to a location on the **host** filesystem the sandbox may
/// grant access to or protect (for example, a writable or protected subtree).
///
/// The entire purpose of this type is to capture the *security-relevant identity*
/// of a host location once, up front, in a form the enforcement layer can use
/// without re-resolving a path string later. Re-resolving a path at enforcement
/// time is the classic time-of-check-to-time-of-use hole: a path that was
/// verified as safe can be swapped for a symlink before the sandbox actually
/// binds/allows it, redirecting the grant to an arbitrary host location.
///
/// What is captured is platform-specific:
/// - **macOS**: the fully-canonicalized path, used verbatim as the Seatbelt rule
///   literal. Seatbelt matches the *resolved* access path against this literal,
///   so a post-capture swap of a path component fails closed (denied) rather
///   than redirecting the grant.
/// - **Linux**: an `O_PATH` file descriptor pinned to the target inode. bwrap is
///   launched by a PTY that can't inherit extra fds, so we can't use bwrap's own
///   `--bind-fd`; instead the bind uses an ordinary `--bind <path>` and an
///   in-sandbox validator compares `fstat` of this descriptor against `lstat` of
///   the mounted path after the mounts, failing closed on a post-capture swap
///   (see `linux_bubblewrap::validate_binds` and `README.md`).
/// - **Windows**: nothing — a Windows process holds no Linux fds, so the real
///   capture-at-validation happens inside WSL (in the `--wsl-sandbox-helper`),
///   and the value here carries only the requested path as untrusted intent.
///
/// The type is deliberately **opaque**: it does not `Deref`, and it never hands
/// back its trusted value. The only thing readable is a *display-only* path via
/// [`HostFilesystemLocation::untrusted_path_display`], suitable for showing a
/// human but which must never be passed back into a sandbox API as the
/// location's identity. Equality reflects the actual filesystem object (same
/// inode), not the textual path.
#[derive(Clone)]
pub struct HostFilesystemLocation {
    /// macOS: the canonicalized path, resolved exactly once at capture time and
    /// used directly as the Seatbelt rule literal.
    #[cfg(target_os = "macos")]
    canonical_path: PathBuf,
    /// Linux: an `O_PATH` descriptor pinned to the captured inode. Wrapped in an
    /// `Arc` only so the surrounding policy types can stay `Clone`; cloning
    /// shares the same underlying descriptor.
    #[cfg(target_os = "linux")]
    fd: std::sync::Arc<std::os::fd::OwnedFd>,
    /// The path exactly as the caller requested it. Kept **only** so the UI can
    /// show the user which location is being granted. This is never consulted by
    /// any enforcement path — treat it as untrusted, attacker-influenced text.
    untrusted_path_for_display: PathBuf,
}

impl HostFilesystemLocation {
    /// Capture `path` as a host sandbox location, resolving its identity up front.
    ///
    /// On macOS this canonicalizes the path; on Linux it opens an `O_PATH`
    /// descriptor to it; on Windows it records nothing. The caller is
    /// responsible for having already *validated* `path` (e.g. confirmed it is
    /// inside the project, or safe to treat as a protected path) — capturing it here
    /// pins that decision against later tampering. To be race-free, capture
    /// should happen as part of, or immediately after, that validation, and the
    /// resulting value should be passed around unchanged from then on (never
    /// re-derived from a path).
    pub fn new(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let path = path.as_ref();
        let untrusted_path_for_display = path.to_path_buf();

        #[cfg(target_os = "macos")]
        {
            // `canonicalize_allowing_missing_leaf` resolves through the existing
            // parent so a not-yet-created leaf still yields the real path
            // Seatbelt will match against.
            let canonical_path = canonicalize_allowing_missing_leaf(path);
            Ok(Self {
                canonical_path,
                untrusted_path_for_display,
            })
        }
        #[cfg(target_os = "linux")]
        {
            use std::os::unix::fs::OpenOptionsExt as _;
            // `O_PATH` opens a handle that refers to the inode without granting
            // read/write on its contents, which is exactly what a bind source
            // needs. `O_CLOEXEC` keeps the descriptor from leaking into
            // unrelated children; the bind step re-publishes it deliberately
            // when launching bwrap.
            let file = std::fs::OpenOptions::new()
                .read(true)
                .custom_flags(libc::O_PATH | libc::O_CLOEXEC)
                .open(path)?;
            Ok(Self {
                fd: std::sync::Arc::new(std::os::fd::OwnedFd::from(file)),
                untrusted_path_for_display,
            })
        }
        #[cfg(not(any(target_os = "macos", target_os = "linux")))]
        {
            Ok(Self {
                untrusted_path_for_display,
            })
        }
    }

    /// The requested path, for **display only** (e.g. the permission-request UI).
    ///
    /// This intentionally returns the untrusted, as-requested path — never the
    /// captured trusted identity. Do not feed the result back into any sandbox
    /// API as if it identified this location.
    pub fn untrusted_path_display(&self) -> std::path::Display<'_> {
        self.untrusted_path_for_display.display()
    }

    /// macOS: the canonical path captured once at construction, used verbatim as
    /// the Seatbelt rule literal. Trusted — never re-resolved. Falls back to the
    /// requested path for a display-only location (which must never reach
    /// enforcement).
    #[cfg(target_os = "macos")]
    pub(crate) fn macos_canonical_path(&self) -> &Path {
        &self.canonical_path
    }

    /// Linux: a borrowed handle to the pinned inode, for deriving a bind source
    /// path and for `fstat`-based identity checks.
    #[cfg(target_os = "linux")]
    pub(crate) fn linux_fd(&self) -> std::os::fd::BorrowedFd<'_> {
        use std::os::fd::AsFd as _;
        self.fd.as_fd()
    }

    /// Linux: an independent `O_PATH` descriptor to the same pinned inode,
    /// duplicated (with `O_CLOEXEC`) so the validation server can own and send it
    /// over `SCM_RIGHTS` without affecting this location's descriptor.
    #[cfg(target_os = "linux")]
    pub(crate) fn linux_dup_fd(&self) -> std::io::Result<std::os::fd::OwnedFd> {
        use std::os::fd::AsFd as _;
        self.fd.as_fd().try_clone_to_owned()
    }

    /// Windows: the requested path, to be mapped into WSL and handed to the
    /// in-WSL helper. Windows captures no identity itself (it holds no Linux
    /// fds); the real capture-at-validation happens WSL-side in the helper, so
    /// here the requested path *is* the location.
    #[cfg(target_os = "windows")]
    pub(crate) fn windows_path(&self) -> &Path {
        &self.untrusted_path_for_display
    }
}

impl fmt::Debug for HostFilesystemLocation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Only the display path is shown; the trusted identity stays opaque.
        formatter
            .debug_struct("HostFilesystemLocation")
            .field(
                "untrusted_path_for_display",
                &self.untrusted_path_for_display,
            )
            .finish_non_exhaustive()
    }
}

impl PartialEq for HostFilesystemLocation {
    /// Two locations are equal when they refer to the **same filesystem object**,
    /// determined from the captured identity (the inode behind the `O_PATH` fd on
    /// Linux, the canonical path on macOS) — never from the textual
    /// display path. This is what lets policy bookkeeping dedupe "the same
    /// location named two different ways," and refuse to treat "two different
    /// objects that happen to share a path string" as one.
    fn eq(&self, other: &Self) -> bool {
        #[cfg(target_os = "linux")]
        {
            match (
                linux_fd_identity(self.fd.as_raw_fd()),
                linux_fd_identity(other.fd.as_raw_fd()),
            ) {
                (Some(a), Some(b)) => a == b,
                // An `fstat` on an `O_PATH` fd we own should never fail; if it
                // somehow does we can't prove identity, so report "not equal"
                // (the safe answer) and leave a trace.
                _ => {
                    log::error!(
                        "failed to fstat an O_PATH descriptor while comparing sandbox locations"
                    );
                    false
                }
            }
        }
        #[cfg(target_os = "macos")]
        {
            // Canonicalization is a bijection on real paths, so equal canonical
            // paths mean the same directory/file.
            self.canonical_path == other.canonical_path
        }
        #[cfg(not(any(target_os = "linux", target_os = "macos")))]
        {
            // No enforcement and no captured identity on these platforms; fall
            // back to the requested path purely so the type can still be used in
            // collections.
            self.untrusted_path_for_display == other.untrusted_path_for_display
        }
    }
}

impl Eq for HostFilesystemLocation {}

/// The `(device, inode)` pair behind an `O_PATH` descriptor, used to decide
/// whether two [`HostFilesystemLocation`]s refer to the same filesystem object.
#[cfg(target_os = "linux")]
fn linux_fd_identity(fd: std::os::fd::RawFd) -> Option<(u64, u64)> {
    let stat = nix::sys::stat::fstat(fd).ok()?;
    Some((stat.st_dev as u64, stat.st_ino as u64))
}

/// A path *inside the sandbox* — i.e. where a host location is exposed in the
/// sandboxed process's view of the filesystem (for example, a bind-mount
/// destination on Linux).
///
/// Unlike [`HostFilesystemLocation`], this needs no hardening and is just a thin
/// wrapper around a [`PathBuf`]. It only names a location within the sandbox's
/// own namespace: the worst a tampered sandbox-side path can do is expose the
/// (already-granted) host files at a *different* path inside the sandbox — it can
/// never widen which host files are reachable. It is therefore fine to build one
/// from an ordinary, even attacker-influenced, path.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct SandboxFilesystemLocation(PathBuf);

impl SandboxFilesystemLocation {
    /// Name a location inside the sandbox's filesystem view.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self(path.into())
    }

    /// The in-sandbox path. Safe to read: this is not a trusted host identity.
    pub fn as_path(&self) -> &Path {
        &self.0
    }

    /// Consume this wrapper, yielding the underlying in-sandbox path.
    pub fn into_path_buf(self) -> PathBuf {
        self.0
    }
}

impl From<PathBuf> for SandboxFilesystemLocation {
    fn from(path: PathBuf) -> Self {
        Self(path)
    }
}

/// What a command is allowed to do, expressed as intent. This is the entire
/// public configuration surface; how each policy is enforced (Seatbelt rules,
/// Bubblewrap flags, a loopback proxy, …) is an implementation detail.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SandboxPolicy {
    pub fs: SandboxFsPolicy,
    pub network: SandboxNetPolicy,
}

/// Filesystem policy for a sandboxed command.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SandboxFsPolicy {
    /// Allow unrestricted filesystem writes except for protected paths, which
    /// remain readable but not writable.
    Unrestricted {
        protected_paths: Vec<HostFilesystemLocation>,
    },
    /// Reads are allowed everywhere; writes are confined to these locations
    /// (and the standard ephemeral locations the platform provides). Each is a
    /// [`HostFilesystemLocation`] captured at validation time, never a bare path
    /// the enforcement layer would re-resolve.
    Restricted {
        writable_paths: Vec<HostFilesystemLocation>,
        protected_paths: Vec<HostFilesystemLocation>,
    },
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

/// Host paths that should remain readable but not writable, even when they fall
/// under a writable subtree.
///
/// The caller computes this list because the sandbox layer does not know which
/// application-specific paths need stronger protection.
pub type ProtectedPaths = Vec<HostFilesystemLocation>;

impl SandboxPolicy {
    /// Combine two policy layers. Filesystem/network grants are unioned into the
    /// least-restrictive policy that satisfies both layers, while protected paths
    /// within restricted filesystem policies are unioned so every layer's
    /// protected subtrees remain protected.
    pub fn merge(self, other: SandboxPolicy) -> SandboxPolicy {
        SandboxPolicy {
            fs: self.fs.merge(other.fs),
            network: self.network.merge(other.network),
        }
    }

    /// Replace the protected paths in the filesystem policy, keeping writable
    /// paths and network policy unchanged.
    pub fn with_protected_paths(mut self, protected_paths: Vec<HostFilesystemLocation>) -> Self {
        match &mut self.fs {
            SandboxFsPolicy::Unrestricted {
                protected_paths: existing,
            }
            | SandboxFsPolicy::Restricted {
                protected_paths: existing,
                ..
            } => *existing = protected_paths,
        }
        self
    }
}

fn merge_locations(
    mut locations: Vec<HostFilesystemLocation>,
    other: Vec<HostFilesystemLocation>,
) -> Vec<HostFilesystemLocation> {
    for location in other {
        if !locations.contains(&location) {
            locations.push(location);
        }
    }
    locations
}

fn validate_writable_paths_do_not_overlap_protected_paths(
    writable_paths: &[HostFilesystemLocation],
    protected_paths: &[HostFilesystemLocation],
) -> Result<(), SandboxError> {
    for writable_path in writable_paths {
        for protected_path in protected_paths {
            if writable_path_overlaps_protected_path(writable_path, protected_path) {
                return Err(SandboxError::InvalidRequest(format!(
                    "writable sandbox path `{}` overlaps protected path `{}`",
                    writable_path.untrusted_path_display(),
                    protected_path.untrusted_path_display()
                )));
            }
        }
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn writable_path_overlaps_protected_path(
    writable_path: &HostFilesystemLocation,
    protected_path: &HostFilesystemLocation,
) -> bool {
    linux_location_is_equal_or_descendant(writable_path, protected_path)
}

#[cfg(target_os = "macos")]
fn writable_path_overlaps_protected_path(
    writable_path: &HostFilesystemLocation,
    protected_path: &HostFilesystemLocation,
) -> bool {
    writable_path
        .macos_canonical_path()
        .starts_with(protected_path.macos_canonical_path())
}

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
fn writable_path_overlaps_protected_path(
    writable_path: &HostFilesystemLocation,
    protected_path: &HostFilesystemLocation,
) -> bool {
    writable_path
        .untrusted_path_for_display
        .starts_with(&protected_path.untrusted_path_for_display)
}

#[cfg(target_os = "linux")]
fn linux_location_is_equal_or_descendant(
    location: &HostFilesystemLocation,
    ancestor: &HostFilesystemLocation,
) -> bool {
    use std::os::fd::{AsRawFd as _, FromRawFd as _, OwnedFd};

    if location == ancestor {
        return true;
    }

    let Ok(mut current) = location.linux_dup_fd() else {
        log::warn!("failed to duplicate sandbox location fd while checking protected path overlap");
        return false;
    };

    loop {
        let parent = unsafe {
            let fd = libc::openat(
                current.as_raw_fd(),
                c"..".as_ptr(),
                libc::O_PATH | libc::O_DIRECTORY | libc::O_CLOEXEC,
            );
            if fd < 0 {
                return false;
            }
            OwnedFd::from_raw_fd(fd)
        };

        let Some(parent_identity) = linux_fd_identity(parent.as_raw_fd()) else {
            log::warn!("failed to fstat parent fd while checking protected path overlap");
            return false;
        };
        let Some(current_identity) = linux_fd_identity(current.as_raw_fd()) else {
            log::warn!("failed to fstat current fd while checking protected path overlap");
            return false;
        };
        let Some(ancestor_identity) = linux_fd_identity(ancestor.linux_fd().as_raw_fd()) else {
            log::warn!("failed to fstat protected fd while checking protected path overlap");
            return false;
        };

        if parent_identity == ancestor_identity {
            return true;
        }
        if parent_identity == current_identity {
            return false;
        }
        current = parent;
    }
}

impl SandboxFsPolicy {
    /// Unrestricted access dominates; otherwise the writable subtrees union.
    pub fn merge(self, other: SandboxFsPolicy) -> SandboxFsPolicy {
        match (self, other) {
            (
                SandboxFsPolicy::Unrestricted {
                    protected_paths: protected_a,
                },
                SandboxFsPolicy::Unrestricted {
                    protected_paths: protected_b,
                },
            ) => SandboxFsPolicy::Unrestricted {
                protected_paths: merge_locations(protected_a, protected_b),
            },
            (
                SandboxFsPolicy::Unrestricted {
                    protected_paths: protected_a,
                },
                SandboxFsPolicy::Restricted {
                    protected_paths: protected_b,
                    ..
                },
            )
            | (
                SandboxFsPolicy::Restricted {
                    protected_paths: protected_a,
                    ..
                },
                SandboxFsPolicy::Unrestricted {
                    protected_paths: protected_b,
                },
            ) => SandboxFsPolicy::Unrestricted {
                protected_paths: merge_locations(protected_a, protected_b),
            },
            (
                SandboxFsPolicy::Restricted {
                    writable_paths: writable_a,
                    protected_paths: protected_a,
                },
                SandboxFsPolicy::Restricted {
                    writable_paths: writable_b,
                    protected_paths: protected_b,
                },
            ) => SandboxFsPolicy::Restricted {
                writable_paths: merge_locations(writable_a, writable_b),
                protected_paths: merge_locations(protected_a, protected_b),
            },
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
    writable_paths: Vec<HostFilesystemLocation>,
    protected_paths: Vec<HostFilesystemLocation>,
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
    /// In-process network proxy for the restricted-network case, spawned on the
    /// first `wrap`. Dropped on a background thread (the join blocks).
    proxy: Option<ProxyHandle>,
    /// Linux only: the host endpoint that hands the in-sandbox validator the
    /// captured `O_PATH` fds over a unix socket. Runs entirely in-process (a
    /// short-lived background thread, never a separate process) and is owned by
    /// this `Sandbox` — which is created per command — so it comes up when the
    /// command is wrapped and is torn down (thread stopped, socket removed) when
    /// the command finishes. Holds the fds, keeping their inodes pinned until
    /// then. Created lazily on the wrap that first needs it (a restricted-fs run
    /// with writable binds); a `Sandbox` normally wraps a single command.
    #[cfg(target_os = "linux")]
    validation_fd_sender: Option<linux_bubblewrap::ValidationFdSender>,
    /// Windows only: `(release channel, version)` of the Linux `zed` to
    /// provision inside WSL as the `--wsl-sandbox-helper` (version `latest` for
    /// dev builds). Set by the caller (which has the running release info);
    /// `None` falls back to exec'ing bwrap directly without in-sandbox bind
    /// validation.
    #[cfg(target_os = "windows")]
    wsl_zed_release: Option<(String, String)>,
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
            SandboxFsPolicy::Unrestricted { protected_paths } => FsSetup {
                allow_fs_write: true,
                writable_paths: Vec::new(),
                protected_paths,
            },
            SandboxFsPolicy::Restricted {
                writable_paths,
                protected_paths,
            } => {
                validate_writable_paths_do_not_overlap_protected_paths(
                    &writable_paths,
                    &protected_paths,
                )?;
                FsSetup {
                    allow_fs_write: false,
                    writable_paths,
                    protected_paths,
                }
            }
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
            proxy: None,
            #[cfg(target_os = "linux")]
            validation_fd_sender: None,
            #[cfg(target_os = "windows")]
            wsl_zed_release: None,
            #[cfg(target_os = "macos")]
            seatbelt_config: None,
        })
    }

    /// Windows only: record the `(release channel, version)` of the Linux `zed`
    /// to provision inside WSL as the sandbox helper (version `latest` for dev
    /// builds). The caller resolves these from the running app's release info
    /// (which this low-level crate can't read) and sets them before `wrap`. When
    /// unset, the WSL backend falls back to exec'ing bwrap directly without
    /// in-sandbox bind validation.
    #[cfg(target_os = "windows")]
    pub fn set_wsl_zed_release(&mut self, channel: String, version: String) {
        self.wsl_zed_release = Some((channel, version));
    }

    /// Check whether the platform sandbox can be created on this host without
    /// actually building a command or spawning the proxy. On Linux this runs a
    /// brief `bwrap` probe (call it off the main thread).
    ///
    /// This answers a purely *environmental* question — is a bwrap sandbox
    /// possible here at all (a usable `bwrap`, plus the unprivileged user
    /// namespace and mount scaffolding we rely on)? It deliberately does **not**
    /// depend on any command's writable grants or working directory: the probe
    /// runs a bare, representative sandbox and runs `true` in it. (Unlike the
    /// former Landlock probe, bwrap needs no ABI-matching against the real
    /// ruleset, so there's no reason to mirror the real command's mounts.)
    pub fn can_create(policy: &SandboxPolicy) -> Result<(), SandboxError> {
        #[cfg(target_os = "linux")]
        {
            let permissions = linux_bubblewrap::SandboxPermissions {
                network: linux_probe_network(&policy.network),
                allow_fs_write: matches!(policy.fs, SandboxFsPolicy::Unrestricted { .. }),
            };
            linux_bubblewrap::check_can_create_sandbox(permissions).map_err(map_linux_status)
        }
        #[cfg(target_os = "windows")]
        {
            if matches!(policy.network, SandboxNetPolicy::Restricted { .. }) {
                return Err(unsupported_restricted_network_on_windows());
            }
            Ok(())
        }
        #[cfg(not(any(target_os = "linux", target_os = "windows")))]
        {
            let _ = policy;
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

    /// Return the protected paths for the enforcement layer. Even with broad
    /// filesystem writes, protected paths remain readable but not writable.
    #[cfg(any(target_os = "macos", target_os = "linux", target_os = "windows"))]
    fn protected_paths(&self) -> Vec<HostFilesystemLocation> {
        self.fs.protected_paths.clone()
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
        let protected_paths = self.protected_paths();
        // Build the writable binds as (captured fd, bind path) pairs in lockstep.
        // The bind *path* is derived from the pinned inode (readlink of the
        // captured `O_PATH` fd), never from an attacker-influenceable string; the
        // *fd* is what the in-sandbox validator compares the mounted inode
        // against. The two lists stay in the same order so each fd lines up with
        // its path on the validator side.
        let mut writable_owned: Vec<PathBuf> = Vec::new();
        let mut writable_fds: Vec<std::os::fd::OwnedFd> = Vec::new();
        for location in &self.fs.writable_paths {
            let Some(path) = linux_location_path(location) else {
                continue;
            };
            match location.linux_dup_fd() {
                Ok(fd) => {
                    writable_owned.push(path);
                    writable_fds.push(fd);
                }
                Err(error) => {
                    // Fail closed: a bind we can't pin a verifiable fd for is
                    // dropped rather than bound unverified.
                    log::warn!(
                        "[sandbox] could not duplicate fd for writable bind {}: {error}",
                        path.display()
                    );
                }
            }
        }
        let writable: Vec<&Path> = writable_owned.iter().map(PathBuf::as_path).collect();
        let protected_owned: Vec<PathBuf> = protected_paths
            .iter()
            .filter_map(linux_location_path)
            .collect();
        let protected_paths: Vec<&Path> = protected_owned.iter().map(PathBuf::as_path).collect();

        // Stand up the host endpoint that sends the captured fds to the
        // in-sandbox validator, when this run has writable binds to verify. It's
        // an in-process background thread owned by this (per-command) `Sandbox`,
        // so it lives only for the command's duration. The sender serves its
        // descriptors to exactly one client and then tears itself down, so each
        // wrap creates a fresh one (replacing any from a prior wrap); a
        // `Sandbox` normally wraps a single command.
        if !self.fs.allow_fs_write && !writable_fds.is_empty() {
            let sender =
                linux_bubblewrap::ValidationFdSender::spawn(writable_fds).map_err(|error| {
                    SandboxError::Io(format!("failed to start sandbox bind validator: {error}"))
                })?;
            self.validation_fd_sender = Some(sender);
        }
        let validation_socket =
            self.validation_fd_sender
                .as_ref()
                .map(|sender| linux_bubblewrap::ValidationSocket {
                    host_socket_path: sender.host_socket_path(),
                    sandbox_socket_path: sender.sandbox_socket_path(),
                });

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
            &protected_paths,
            command.cwd.as_deref(),
            &command.program,
            &command.args,
            proxy_socket_path.as_deref(),
            validation_socket,
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
        let protected_paths = self.protected_paths();
        // Each location's canonical path was resolved exactly once at capture
        // time; pass it straight through as the Seatbelt rule literal. The
        // profile generator must NOT re-canonicalize (that reopened the
        // verify-vs-enforce gap); see `generate_seatbelt_config`.
        let writable: Vec<&Path> = self
            .fs
            .writable_paths
            .iter()
            .map(HostFilesystemLocation::macos_canonical_path)
            .collect();
        let protected: Vec<&Path> = protected_paths
            .iter()
            .map(HostFilesystemLocation::macos_canonical_path)
            .collect();

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
        let permissions = windows_wsl::SandboxPermissions {
            allow_network: matches!(self.network, NetSetup::Unrestricted),
            allow_fs_write: self.fs.allow_fs_write,
        };
        // On Windows the location carries only the requested path; the in-WSL
        // helper performs the real capture-at-validation. These are mapped into
        // WSL by `wrap_invocation`.
        let writable_paths: Vec<PathBuf> = self
            .fs
            .writable_paths
            .iter()
            .map(|location| location.windows_path().to_path_buf())
            .collect();
        let protected_paths: Vec<PathBuf> = self
            .protected_paths()
            .iter()
            .map(|location| location.windows_path().to_path_buf())
            .collect();
        let (program, args) = windows_wsl::wrap_invocation(
            command.program.clone(),
            command.args.clone(),
            writable_paths,
            protected_paths,
            permissions,
            command.cwd.clone(),
            command.env.clone(),
            self.wsl_zed_release.clone(),
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

/// Argv flag that marks the WSL-side sandbox-helper re-exec. Shared so the
/// Windows side (`windows_wsl`, which builds the `wsl.exe` invocation) and the
/// Linux side (`linux_bubblewrap`, which parses it inside WSL) can't drift.
///
/// Only referenced by those two cfg-gated modules, so it's gated to match;
/// otherwise it's dead code on macOS.
#[cfg(any(target_os = "linux", target_os = "windows"))]
pub(crate) const WSL_SANDBOX_HELPER_FLAG: &str = "--wsl-sandbox-helper";

/// Handle a possible re-exec of this binary as a sandbox helper.
///
/// Two Linux re-exec modes funnel through here, neither of which returns if it
/// matches:
/// - the in-sandbox launcher (bind validator + restricted-network bridge), run
///   by bwrap before the real command; and
/// - the WSL-side helper, run inside WSL to capture fds + drive bwrap (the moral
///   equivalent of `Sandbox::wrap` on native Linux).
///
/// Call this at the top of `main`, before normal argument parsing.
#[doc(hidden)]
pub fn run_sandbox_launcher_if_invoked() {
    #[cfg(target_os = "linux")]
    {
        linux_bubblewrap::run_launcher_if_invoked();
        linux_bubblewrap::run_wsl_helper_if_invoked();
    }
}

// The createability probe only needs to know whether a sandbox *can* be built
// (namespaces, `bwrap` availability); it does not enforce any grant, so it runs
// with no writable binds rather than re-deriving paths from the opaque
// locations.

/// The current path of the inode pinned by a [`HostFilesystemLocation`]'s
/// `O_PATH` fd, via `/proc/self/fd`. This resolves to the *pinned inode*, so it
/// reflects the object captured at validation even if its name was changed,
/// rather than re-resolving an attacker-influenceable path string.
#[cfg(target_os = "linux")]
fn linux_location_path(location: &HostFilesystemLocation) -> Option<PathBuf> {
    use std::os::fd::AsRawFd as _;
    std::fs::read_link(format!("/proc/self/fd/{}", location.linux_fd().as_raw_fd())).ok()
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
    fn restricted_fs_rejects_writable_paths_inside_protected_paths() {
        let temp_dir = tempfile::tempdir().expect("create temp dir");
        let repo_dir = temp_dir.path().join("repo");
        let git_dir = repo_dir.join(".git");
        let hooks_dir = git_dir.join("hooks");
        std::fs::create_dir_all(&hooks_dir).expect("create git hooks dir");

        let protected_git = HostFilesystemLocation::new(&git_dir).expect("capture git dir");
        let writable_git = HostFilesystemLocation::new(&git_dir).expect("capture git dir");
        let writable_hooks = HostFilesystemLocation::new(&hooks_dir).expect("capture hooks dir");

        for writable_path in [writable_git, writable_hooks] {
            let result = Sandbox::new(SandboxPolicy {
                fs: SandboxFsPolicy::Restricted {
                    writable_paths: vec![writable_path],
                    protected_paths: vec![protected_git.clone()],
                },
                network: SandboxNetPolicy::Blocked,
            });

            assert!(matches!(result, Err(SandboxError::InvalidRequest(_))));
        }
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn restricted_fs_protected_overlap_check_uses_captured_fds_not_path_text() {
        use std::os::unix::fs as unix_fs;

        let temp_dir = tempfile::tempdir().expect("create temp dir");
        let repo_dir = temp_dir.path().join("repo");
        let git_dir = repo_dir.join(".git");
        let real_git_dir = repo_dir.join("real-git");
        let outside_dir = temp_dir.path().join("outside");
        let outside_hooks_dir = outside_dir.join("hooks");
        std::fs::create_dir_all(&git_dir).expect("create git dir");
        std::fs::create_dir_all(&outside_hooks_dir).expect("create outside hooks dir");

        let protected_git = HostFilesystemLocation::new(&git_dir).expect("capture git dir");
        std::fs::rename(&git_dir, &real_git_dir).expect("move git dir aside");
        unix_fs::symlink(&outside_dir, &git_dir).expect("replace git dir with symlink");
        let writable_displayed_inside_git =
            HostFilesystemLocation::new(git_dir.join("hooks")).expect("capture symlink target");

        Sandbox::new(SandboxPolicy {
            fs: SandboxFsPolicy::Restricted {
                writable_paths: vec![writable_displayed_inside_git],
                protected_paths: vec![protected_git],
            },
            network: SandboxNetPolicy::Blocked,
        })
        .expect("captured writable fd is outside protected metadata");
    }

    #[test]
    fn fs_merge_unrestricted_dominates_else_unions_paths() {
        // Writable paths are captured as real `HostFilesystemLocation`s (keyed on
        // the inode), so the union/dedup test needs three distinct real dirs.
        let dir_a = tempfile::tempdir().expect("create temp dir a");
        let dir_b = tempfile::tempdir().expect("create temp dir b");
        let dir_c = tempfile::tempdir().expect("create temp dir c");
        let dir_d = tempfile::tempdir().expect("create temp dir d");
        let location = |dir: &tempfile::TempDir| {
            HostFilesystemLocation::new(dir.path()).expect("capture temp dir")
        };

        let a = SandboxFsPolicy::Restricted {
            writable_paths: vec![location(&dir_a), location(&dir_b)],
            protected_paths: vec![location(&dir_c)],
        };
        let b = SandboxFsPolicy::Restricted {
            writable_paths: vec![location(&dir_b), location(&dir_c)],
            protected_paths: vec![location(&dir_c), location(&dir_d)],
        };
        assert_eq!(
            a.clone().merge(b),
            SandboxFsPolicy::Restricted {
                writable_paths: vec![location(&dir_a), location(&dir_b), location(&dir_c)],
                protected_paths: vec![location(&dir_c), location(&dir_d)],
            }
        );
        assert_eq!(
            a.merge(SandboxFsPolicy::Unrestricted {
                protected_paths: vec![location(&dir_a)],
            }),
            SandboxFsPolicy::Unrestricted {
                protected_paths: vec![location(&dir_c), location(&dir_a)],
            }
        );
        assert_eq!(
            SandboxFsPolicy::Unrestricted {
                protected_paths: vec![location(&dir_d)],
            }
            .merge(SandboxFsPolicy::Restricted {
                writable_paths: vec![location(&dir_a)],
                protected_paths: vec![location(&dir_c)],
            }),
            SandboxFsPolicy::Unrestricted {
                protected_paths: vec![location(&dir_d), location(&dir_c)],
            }
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

/// A directory that is about to be granted as a sandbox write path but may not
/// exist yet. Preparing one resolves the platform difference in how a
/// not-yet-existing write grant is materialized, while keeping the same security
/// property: the caller shows [`Self::canonical_path`] to the user and records
/// *that* as the grant, so approval is always against the real, symlink-resolved
/// target.
///
/// - **Linux/WSL**: bubblewrap can only bind an existing inode, so the missing
///   directory (and any missing parents) is created **eagerly**, per component,
///   and the leaf's inode is pinned to read back its canonical path. If the
///   grant is denied, [`Self::discard`] removes exactly the directories that were
///   created, deepest-first, following no symlinks (`rmdir` only removes empty
///   dirs and never traverses a swapped-in symlink).
/// - **macOS**: Seatbelt resolves paths at syscall time and can grant a missing
///   path, so nothing is created here; the directory is materialized only after
///   approval via [`Self::finalize`].
///
/// The eventual bind is still protected by the usual capture-and-revalidate path
/// (`HostFilesystemLocation`), which re-pins the inode when the command runs.
pub struct GrantableWriteDir {
    canonical_path: PathBuf,
    /// Directories created eagerly to pin the inode, shallowest-first. Empty on
    /// platforms that defer creation to [`Self::finalize`].
    eagerly_created: Vec<PathBuf>,
}

impl GrantableWriteDir {
    /// Prepare `path` for use as a sandbox write grant. `path` must be absolute.
    pub fn prepare(path: &Path) -> std::io::Result<Self> {
        #[cfg(target_os = "linux")]
        {
            let mut eagerly_created = Vec::new();
            if let Err(error) = create_missing_dirs(path, &mut eagerly_created) {
                // Roll back any partial creation so a failure leaves no litter.
                for dir in eagerly_created.iter().rev() {
                    let _ = std::fs::remove_dir(dir);
                }
                return Err(error);
            }
            let canonical_path = pinned_canonical_path(path)?;
            Ok(Self {
                canonical_path,
                eagerly_created,
            })
        }
        #[cfg(target_os = "macos")]
        {
            Ok(Self {
                canonical_path: canonicalize_allowing_missing_leaf(path),
                eagerly_created: Vec::new(),
            })
        }
        #[cfg(not(any(target_os = "linux", target_os = "macos")))]
        {
            let _ = path;
            Err(std::io::Error::new(
                std::io::ErrorKind::Unsupported,
                "granting a not-yet-existing write directory is not supported on this platform",
            ))
        }
    }

    /// The canonical, symlink-resolved path to show the user and record as the
    /// grant.
    pub fn canonical_path(&self) -> &Path {
        &self.canonical_path
    }

    /// Materialize the directory once the grant is approved. A no-op on platforms
    /// that already created it eagerly.
    pub fn finalize(&self) -> std::io::Result<()> {
        #[cfg(target_os = "macos")]
        {
            std::fs::create_dir_all(&self.canonical_path)?;
        }
        Ok(())
    }

    /// Remove exactly the directories we created (deepest-first) when the grant
    /// is denied. Best-effort; `rmdir` leaves non-empty dirs and swapped-in
    /// symlinks untouched.
    pub fn discard(self) {
        for dir in self.eagerly_created.iter().rev() {
            let _ = std::fs::remove_dir(dir);
        }
    }
}

/// Create each missing component of `path` with `create_dir` (never
/// `create_dir_all`), recording exactly the directories created so they can be
/// removed again if the grant is denied. Components that already exist are left
/// alone.
#[cfg(target_os = "linux")]
fn create_missing_dirs(path: &Path, created: &mut Vec<PathBuf>) -> std::io::Result<()> {
    let mut cur = PathBuf::new();
    for component in path.components() {
        cur.push(component);
        match std::fs::create_dir(&cur) {
            Ok(()) => created.push(cur.clone()),
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
            Err(error) => return Err(error),
        }
    }
    Ok(())
}

/// Open an `O_PATH` handle to `path` and read back the canonical path of the
/// inode it pins, so the value shown to the user reflects the real target even
/// when a component is a symlink.
#[cfg(target_os = "linux")]
fn pinned_canonical_path(path: &Path) -> std::io::Result<PathBuf> {
    use std::os::fd::AsRawFd as _;
    use std::os::unix::fs::OpenOptionsExt as _;
    let handle = std::fs::OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_PATH | libc::O_CLOEXEC)
        .open(path)?;
    std::fs::read_link(format!("/proc/self/fd/{}", handle.as_raw_fd()))
}

#[cfg(all(test, target_os = "linux"))]
mod grantable_write_dir_tests {
    use super::GrantableWriteDir;
    use std::fs;

    #[test]
    fn creates_missing_dirs_and_discard_removes_only_those() {
        let root = tempfile::tempdir().unwrap();
        let existing = root.path().join("existing");
        fs::create_dir(&existing).unwrap();
        let target = existing.join("a").join("b").join("c");

        let prepared = GrantableWriteDir::prepare(&target).unwrap();
        assert!(target.is_dir());
        assert_eq!(prepared.canonical_path(), target.canonicalize().unwrap());

        prepared.discard();
        // Everything we created is gone...
        assert!(!existing.join("a").exists());
        // ...but the pre-existing ancestor is untouched.
        assert!(existing.is_dir());
    }

    #[test]
    fn existing_dir_is_left_alone_and_not_removed_on_discard() {
        let root = tempfile::tempdir().unwrap();
        let target = root.path().join("already");
        fs::create_dir(&target).unwrap();

        let prepared = GrantableWriteDir::prepare(&target).unwrap();
        assert!(target.is_dir());
        // We created nothing, so discard removes nothing.
        prepared.discard();
        assert!(target.is_dir());
    }

    #[test]
    fn canonical_path_resolves_a_symlinked_parent() {
        let root = tempfile::tempdir().unwrap();
        let real = root.path().join("real");
        fs::create_dir(&real).unwrap();
        let link = root.path().join("link");
        std::os::unix::fs::symlink(&real, &link).unwrap();

        // Granting `link/child` where `link` legitimately points at `real` must
        // succeed and show the user the *resolved* `real/child`, not fail.
        let prepared = GrantableWriteDir::prepare(&link.join("child")).unwrap();
        assert_eq!(
            prepared.canonical_path(),
            real.canonicalize().unwrap().join("child")
        );
        assert!(real.join("child").is_dir());

        prepared.discard();
        assert!(!real.join("child").exists());
    }
}

/// Canonicalize `path`, resolving symlinks, even when its final component
/// doesn't exist yet.
///
/// `std::fs::canonicalize` fails if any component is missing, which would leave
/// a not-yet-created path in a non-canonical form. The sandbox layers
/// canonicalize the writable parent
/// (the worktree root) but, with a plain `canonicalize`, fall back to the raw
/// path for a missing child; the two then disagree when a component is a
/// symlink (`/tmp` -> `/private/tmp` on macOS), and the protection rule for the
/// child misses the real path the command ends up writing. Canonicalizing the
/// existing parent and re-appending the final component keeps the child
/// consistent with its parent. If neither the path nor its parent can be
/// canonicalized, the path is returned unchanged.
//
// Only the macOS Seatbelt layer uses this (Linux skips not-yet-existing
// protected paths rather than emitting a rule for them), so it's gated to macOS
// to avoid a dead-code warning elsewhere.
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
        // (for example, when protecting a not-yet-created child path).
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
