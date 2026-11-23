//! Cross-platform sandboxing for commands run on behalf of the agent.
//!
//! The public API is intentionally platform-neutral. Internally, macOS uses
//! Seatbelt (`sandbox-exec`), Linux uses Bubblewrap (`bwrap`), and Windows uses
//! a Process Security Environment (an AppContainer). Restricted-network policies are enforced by an
//! in-process HTTP/HTTPS proxy (the `http_proxy` crate) that this crate
//! constructs and owns; callers only describe intent via [`SandboxPolicy`].

use std::{
    collections::HashMap,
    fmt,
    path::{Path, PathBuf},
    process::Output,
};

use futures::channel::mpsc::UnboundedSender;
use http_proxy::ProxyHandle;
use http_proxy::{Allowlist, HostPattern, ProxyConfig, UpstreamProxy};

pub use http_proxy::{DenyReason, ProxyEvent, RequestMethod, RequestOutcome};

#[cfg(target_os = "linux")]
mod linux_bubblewrap;

#[cfg(target_os = "macos")]
mod macos_seatbelt;

#[cfg(target_os = "windows")]
mod windows_psec;

mod util;

#[cfg(target_os = "windows")]
use util::CanonicalPathBuf;
#[cfg(target_os = "macos")]
use util::canonicalize_allowing_missing_leaf;
#[cfg(target_os = "linux")]
use util::{CanonicalPathBuf, linux_fd_identity};
pub use util::{
    HostFilesystemLocation, HostFilesystemLocationDisplay, normalize_host_filesystem_locations,
    resolve_canonical,
};
#[cfg(target_os = "windows")]
pub use windows_psec::can_launch_packaged_apps;

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
    /// Unix domain sockets the command may connect to for local IPC even when
    /// the network is blocked or restricted; connecting to any other Unix
    /// socket is denied unless the network is unrestricted. A socket grant
    /// never permits sending packets to other machines.
    ///
    /// Only the macOS backend can enforce this. The Linux and Windows backends
    /// reject any policy that lists a socket.
    pub allowed_unix_sockets: Vec<HostFilesystemLocation>,
}

/// Filesystem policy for a sandboxed command.
///
/// In both variants, `paths_disallowing_reads` are subtrees the command can
/// neither read (including listing and `stat`) nor write, even where a
/// writable path covers them. Each of `paths_allowing_reads` is a subtree the
/// command can use as if no path disallowing reads covered it: it is readable,
/// and writable wherever the rest of the policy makes it writable. Protected
/// paths and protected file names still apply inside it.
///
/// Where the two overlap, allowing wins: a path under both a path disallowing
/// reads and a path allowing reads is readable, however the two are nested.
/// Because a path disallowing reads would therefore have no effect at or
/// beneath a path allowing reads, [`Sandbox::new`] rejects such a policy
/// rather than silently leaving the path readable.
///
/// The macOS and Windows backends enforce both lists; the Linux backend
/// rejects any policy with an entry in one. Windows also rejects them on builds
/// whose Process Security Environment can't deny access to paths.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SandboxFsPolicy {
    /// Allow unrestricted filesystem writes except for protected paths and
    /// protected file names, which remain readable but not writable, and
    /// paths disallowing reads.
    Unrestricted {
        protected_paths: Vec<HostFilesystemLocation>,
        /// See [`ProtectedFileName`].
        protected_file_names: Vec<ProtectedFileName>,
        paths_disallowing_reads: Vec<HostFilesystemLocation>,
        paths_allowing_reads: Vec<HostFilesystemLocation>,
    },
    /// Reads are allowed everywhere except paths disallowing reads; writes are
    /// confined to these locations (and the standard ephemeral locations the platform
    /// provides). Each is a [`HostFilesystemLocation`] captured at validation
    /// time, never a bare path the enforcement layer would re-resolve.
    Restricted {
        writable_paths: Vec<HostFilesystemLocation>,
        protected_paths: Vec<HostFilesystemLocation>,
        /// See [`ProtectedFileName`].
        protected_file_names: Vec<ProtectedFileName>,
        paths_disallowing_reads: Vec<HostFilesystemLocation>,
        paths_allowing_reads: Vec<HostFilesystemLocation>,
    },
}

/// A file name whose writes are denied wherever it appears on the filesystem.
///
/// Matching is on the final path component only, anywhere on the filesystem,
/// including in directories that do not exist yet when the sandbox is built.
/// The match is ASCII case-insensitive, because the default macOS filesystem
/// is case-insensitive and would otherwise let `.ENVRC` stand in for `.envrc`.
/// Files of that name stay readable; the sandbox denies creating, writing,
/// truncating, deleting, renaming onto, and linking (hard or symbolic) to a
/// path whose final component is the name.
///
/// This complements path-based protected paths for files whose consumers look
/// them up by name, such as direnv's `.envrc`, which is read from the working
/// directory or any of its ancestors.
///
/// Only the macOS backend can enforce this today. The Linux and Windows backends
/// reject any policy that contains protected file names, pending the planned
/// Linux syscall-interception backend.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ProtectedFileName(String);

impl ProtectedFileName {
    /// Validate `name` as a single, non-empty path component.
    ///
    /// # Errors
    ///
    /// Returns [`SandboxError::InvalidRequest`] if `name` is empty, is `.` or
    /// `..`, or contains `/` or a NUL byte.
    pub fn new(name: impl Into<String>) -> Result<Self, SandboxError> {
        let name = name.into();
        let problem = if name.is_empty() {
            Some("must not be empty")
        } else if name == "." || name == ".." {
            Some("must not be `.` or `..`")
        } else if name.contains('/') {
            Some("must not contain `/`")
        } else if name.contains('\0') {
            Some("must not contain a NUL byte")
        } else {
            None
        };
        match problem {
            Some(problem) => Err(SandboxError::InvalidRequest(format!(
                "invalid protected file name {name:?}: {problem}"
            ))),
            None => Ok(Self(name)),
        }
    }

    /// The validated file name.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Outbound-network policy for a sandboxed command.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SandboxNetPolicy {
    /// Allow unrestricted outbound network access.
    Unrestricted,
    /// Block all outbound network access.
    Blocked,
    /// Allow outbound HTTP(S) to `allowed_domains` and raw outbound TCP to
    /// `allowed_tcp_ports`, and nothing else.
    Restricted {
        /// Hostnames the command may reach over HTTP(S), as exact hosts or
        /// leading-`*.` subdomain wildcards, enforced by an in-process proxy.
        allowed_domains: Vec<String>,
        /// Destination ports the command may open raw TCP connections to,
        /// bypassing the proxy, for protocols an HTTP proxy cannot carry (SSH
        /// on port 22, for example).
        ///
        /// A port grant applies to every host: the sandbox cannot filter raw
        /// TCP by hostname, so granting port 22 allows SSH to any host. Port 0
        /// is rejected. Only the macOS backend can enforce this; the Linux and
        /// Windows backends reject any policy that lists a port.
        allowed_tcp_ports: Vec<u16>,
    },
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
            allowed_unix_sockets: merge_unique(
                self.allowed_unix_sockets,
                other.allowed_unix_sockets,
            ),
        }
    }

    /// Replace the protected paths in the filesystem policy, keeping writable
    /// paths and network policy unchanged.
    pub fn with_protected_paths(mut self, protected_paths: Vec<HostFilesystemLocation>) -> Self {
        match &mut self.fs {
            SandboxFsPolicy::Unrestricted {
                protected_paths: existing,
                ..
            }
            | SandboxFsPolicy::Restricted {
                protected_paths: existing,
                ..
            } => *existing = protected_paths,
        }
        self
    }

    /// Replace the protected file names in the filesystem policy, keeping
    /// every other part of the policy unchanged.
    pub fn with_protected_file_names(
        mut self,
        protected_file_names: Vec<ProtectedFileName>,
    ) -> Self {
        match &mut self.fs {
            SandboxFsPolicy::Unrestricted {
                protected_file_names: existing,
                ..
            }
            | SandboxFsPolicy::Restricted {
                protected_file_names: existing,
                ..
            } => *existing = protected_file_names,
        }
        self
    }
}

fn merge_unique<T: PartialEq>(mut items: Vec<T>, other: Vec<T>) -> Vec<T> {
    for item in other {
        if !items.contains(&item) {
            items.push(item);
        }
    }
    items
}

fn merge_locations(
    locations: Vec<HostFilesystemLocation>,
    other: Vec<HostFilesystemLocation>,
) -> Vec<HostFilesystemLocation> {
    // Union the two sets and reduce to a minimal cover, so a location nested
    // under another (in either input) is dropped rather than bound redundantly.
    normalize_host_filesystem_locations(locations.into_iter().chain(other))
}

fn validate_writable_paths_do_not_overlap_protected_paths(
    writable_paths: &[HostFilesystemLocation],
    protected_paths: &[HostFilesystemLocation],
) -> Result<(), SandboxError> {
    for writable_path in writable_paths {
        for protected_path in protected_paths {
            if location_is_within(writable_path, protected_path) {
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

/// Rejects a path disallowing reads at or beneath a path allowing reads.
/// Allowing wins where the two overlap, so such an entry would have no effect
/// and the path the caller meant to hide would stay readable.
fn validate_paths_disallowing_reads_have_effect(
    paths_disallowing_reads: &[HostFilesystemLocation],
    paths_allowing_reads: &[HostFilesystemLocation],
) -> Result<(), SandboxError> {
    for disallowing in paths_disallowing_reads {
        for allowing in paths_allowing_reads {
            if location_is_within(disallowing, allowing) {
                return Err(SandboxError::InvalidRequest(format!(
                    "sandbox path disallowing reads `{}` lies inside path allowing reads `{}`",
                    disallowing.untrusted_path_display(),
                    allowing.untrusted_path_display()
                )));
            }
        }
    }
    Ok(())
}

/// Unions two layers' paths disallowing reads. A path allowing reads from one
/// layer is kept only where the other layer leaves its whole subtree readable
/// too, so merging can never make readable a subtree that either layer
/// disallows.
fn merge_read_restrictions(
    disallowing_a: Vec<HostFilesystemLocation>,
    allowing_a: Vec<HostFilesystemLocation>,
    disallowing_b: Vec<HostFilesystemLocation>,
    allowing_b: Vec<HostFilesystemLocation>,
) -> (Vec<HostFilesystemLocation>, Vec<HostFilesystemLocation>) {
    let kept_a: Vec<_> = allowing_a
        .iter()
        .filter(|allowing| subtree_is_readable(allowing, &disallowing_b, &allowing_b))
        .cloned()
        .collect();
    let kept_b: Vec<_> = allowing_b
        .iter()
        .filter(|allowing| subtree_is_readable(allowing, &disallowing_a, &allowing_a))
        .cloned()
        .collect();
    (
        merge_locations(disallowing_a, disallowing_b),
        merge_locations(kept_a, kept_b),
    )
}

/// Whether a layer with these paths disallowing and allowing reads leaves the
/// whole subtree at `location` readable.
fn subtree_is_readable(
    location: &HostFilesystemLocation,
    paths_disallowing_reads: &[HostFilesystemLocation],
    paths_allowing_reads: &[HostFilesystemLocation],
) -> bool {
    paths_disallowing_reads.iter().all(|disallowing| {
        if location_is_within(disallowing, location) {
            false
        } else if location_is_within(location, disallowing) {
            paths_allowing_reads
                .iter()
                .any(|allowing| location_is_within(location, allowing))
        } else {
            true
        }
    })
}

/// Whether `location` is the same file or directory as `ancestor`, or lies
/// beneath it. Compares what each location resolved to when it was captured
/// rather than looking the paths up again.
#[cfg(target_os = "linux")]
pub(crate) fn location_is_within(
    location: &HostFilesystemLocation,
    ancestor: &HostFilesystemLocation,
) -> bool {
    linux_location_is_equal_or_descendant(location, ancestor)
}

#[cfg(target_os = "macos")]
pub(crate) fn location_is_within(
    location: &HostFilesystemLocation,
    ancestor: &HostFilesystemLocation,
) -> bool {
    location
        .macos_canonical_path()
        .starts_with(ancestor.macos_canonical_path())
}

#[cfg(target_os = "windows")]
pub(crate) fn location_is_within(
    location: &HostFilesystemLocation,
    ancestor: &HostFilesystemLocation,
) -> bool {
    location
        .windows_canonical_path()
        .starts_with(ancestor.windows_canonical_path())
}

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
pub(crate) fn location_is_within(
    location: &HostFilesystemLocation,
    ancestor: &HostFilesystemLocation,
) -> bool {
    location
        .untrusted_raw_path()
        .starts_with(ancestor.untrusted_raw_path())
}

#[cfg(target_os = "linux")]
fn linux_location_is_equal_or_descendant(
    location: &HostFilesystemLocation,
    ancestor: &HostFilesystemLocation,
) -> bool {
    use std::os::fd::{AsFd as _, AsRawFd as _, FromRawFd as _, OwnedFd};

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

        let Some(parent_identity) = linux_fd_identity(parent.as_fd()) else {
            log::warn!("failed to fstat parent fd while checking protected path overlap");
            return false;
        };
        let Some(current_identity) = linux_fd_identity(current.as_fd()) else {
            log::warn!("failed to fstat current fd while checking protected path overlap");
            return false;
        };
        let Some(ancestor_identity) = linux_fd_identity(ancestor.linux_fd().as_fd()) else {
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
    /// Protected paths, protected file names, and paths disallowing reads
    /// always union. A path allowing reads survives only where the other layer
    /// leaves its whole subtree readable too, so merging never makes readable
    /// what either layer disallows.
    pub fn merge(self, other: SandboxFsPolicy) -> SandboxFsPolicy {
        match (self, other) {
            (
                SandboxFsPolicy::Unrestricted {
                    protected_paths: protected_a,
                    protected_file_names: names_a,
                    paths_disallowing_reads: disallowing_a,
                    paths_allowing_reads: allowing_a,
                },
                SandboxFsPolicy::Unrestricted {
                    protected_paths: protected_b,
                    protected_file_names: names_b,
                    paths_disallowing_reads: disallowing_b,
                    paths_allowing_reads: allowing_b,
                },
            )
            | (
                SandboxFsPolicy::Unrestricted {
                    protected_paths: protected_a,
                    protected_file_names: names_a,
                    paths_disallowing_reads: disallowing_a,
                    paths_allowing_reads: allowing_a,
                },
                SandboxFsPolicy::Restricted {
                    protected_paths: protected_b,
                    protected_file_names: names_b,
                    paths_disallowing_reads: disallowing_b,
                    paths_allowing_reads: allowing_b,
                    ..
                },
            )
            | (
                SandboxFsPolicy::Restricted {
                    protected_paths: protected_a,
                    protected_file_names: names_a,
                    paths_disallowing_reads: disallowing_a,
                    paths_allowing_reads: allowing_a,
                    ..
                },
                SandboxFsPolicy::Unrestricted {
                    protected_paths: protected_b,
                    protected_file_names: names_b,
                    paths_disallowing_reads: disallowing_b,
                    paths_allowing_reads: allowing_b,
                },
            ) => {
                let (paths_disallowing_reads, paths_allowing_reads) =
                    merge_read_restrictions(disallowing_a, allowing_a, disallowing_b, allowing_b);
                SandboxFsPolicy::Unrestricted {
                    protected_paths: merge_locations(protected_a, protected_b),
                    protected_file_names: merge_unique(names_a, names_b),
                    paths_disallowing_reads,
                    paths_allowing_reads,
                }
            }
            (
                SandboxFsPolicy::Restricted {
                    writable_paths: writable_a,
                    protected_paths: protected_a,
                    protected_file_names: names_a,
                    paths_disallowing_reads: disallowing_a,
                    paths_allowing_reads: allowing_a,
                },
                SandboxFsPolicy::Restricted {
                    writable_paths: writable_b,
                    protected_paths: protected_b,
                    protected_file_names: names_b,
                    paths_disallowing_reads: disallowing_b,
                    paths_allowing_reads: allowing_b,
                },
            ) => {
                let (paths_disallowing_reads, paths_allowing_reads) =
                    merge_read_restrictions(disallowing_a, allowing_a, disallowing_b, allowing_b);
                SandboxFsPolicy::Restricted {
                    writable_paths: merge_locations(writable_a, writable_b),
                    protected_paths: merge_locations(protected_a, protected_b),
                    protected_file_names: merge_unique(names_a, names_b),
                    paths_disallowing_reads,
                    paths_allowing_reads,
                }
            }
        }
    }
}

impl SandboxNetPolicy {
    /// Unrestricted access dominates and `Blocked` is the identity; otherwise the
    /// allowed domains and TCP ports each union.
    pub fn merge(self, other: SandboxNetPolicy) -> SandboxNetPolicy {
        match (self, other) {
            (SandboxNetPolicy::Unrestricted, _) | (_, SandboxNetPolicy::Unrestricted) => {
                SandboxNetPolicy::Unrestricted
            }
            (SandboxNetPolicy::Blocked, other) | (other, SandboxNetPolicy::Blocked) => other,
            (
                SandboxNetPolicy::Restricted {
                    allowed_domains: domains_a,
                    allowed_tcp_ports: ports_a,
                },
                SandboxNetPolicy::Restricted {
                    allowed_domains: domains_b,
                    allowed_tcp_ports: ports_b,
                },
            ) => SandboxNetPolicy::Restricted {
                allowed_domains: merge_unique(domains_a, domains_b),
                allowed_tcp_ports: merge_unique(ports_a, ports_b),
            },
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
    /// Only the Seatbelt backend can enforce these; other platforms reject a
    /// non-empty list in [`Sandbox::new`].
    #[cfg(target_os = "macos")]
    protected_file_names: Vec<ProtectedFileName>,
    #[cfg(any(target_os = "macos", target_os = "windows"))]
    paths_disallowing_reads: Vec<HostFilesystemLocation>,
    #[cfg(any(target_os = "macos", target_os = "windows"))]
    paths_allowing_reads: Vec<HostFilesystemLocation>,
}

/// Resolved network plan derived from [`SandboxNetPolicy`]. For the restricted
/// case the allowlist is parsed up front; the proxy itself is spawned lazily on
/// the first [`Sandbox::wrap`] so its upstream-proxy chaining can read the
/// command's environment.
enum NetSetup {
    Unrestricted,
    Blocked,
    Restricted {
        allowlist: Allowlist,
        /// Only the Seatbelt backend can enforce these; other platforms reject
        /// a non-empty list in [`Sandbox::new`].
        #[cfg(target_os = "macos")]
        allowed_tcp_ports: Vec<u16>,
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
    #[cfg(target_os = "macos")]
    allowed_unix_sockets: Vec<HostFilesystemLocation>,
    /// In-process network proxy for the restricted-network case, spawned on the
    /// first `wrap`. Dropped on a background thread (the join blocks).
    proxy: Option<ProxyHandle>,
    /// Where proxy events are forwarded, in addition to being logged.
    network_events: Option<UnboundedSender<ProxyEvent>>,
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
    ///
    /// # Errors
    ///
    /// Returns [`SandboxError::UnsupportedPolicy`] on Linux and Windows when the
    /// policy has any [`ProtectedFileName`]s, paths disallowing or allowing
    /// reads, Unix socket grants,
    /// or TCP port grants, because those backends cannot enforce them yet.
    /// Returns [`SandboxError::InvalidRequest`] for a malformed policy, such as
    /// a writable path inside a protected path, a path disallowing reads that
    /// is also, or lies inside, a path allowing reads, or TCP port 0.
    pub fn new(policy: SandboxPolicy) -> Result<Self, SandboxError> {
        let (
            allow_fs_write,
            writable_paths,
            protected_paths,
            protected_file_names,
            paths_disallowing_reads,
            paths_allowing_reads,
        ) = match policy.fs {
            SandboxFsPolicy::Unrestricted {
                protected_paths,
                protected_file_names,
                paths_disallowing_reads,
                paths_allowing_reads,
            } => (
                true,
                Vec::new(),
                protected_paths,
                protected_file_names,
                paths_disallowing_reads,
                paths_allowing_reads,
            ),
            SandboxFsPolicy::Restricted {
                writable_paths,
                protected_paths,
                protected_file_names,
                paths_disallowing_reads,
                paths_allowing_reads,
            } => {
                validate_writable_paths_do_not_overlap_protected_paths(
                    &writable_paths,
                    &protected_paths,
                )?;
                (
                    false,
                    writable_paths,
                    protected_paths,
                    protected_file_names,
                    paths_disallowing_reads,
                    paths_allowing_reads,
                )
            }
        };
        validate_paths_disallowing_reads_have_effect(
            &paths_disallowing_reads,
            &paths_allowing_reads,
        )?;

        // Bubblewrap can only protect inodes that exist when the sandbox is
        // built, so it cannot deny a name in a directory created later. Fail
        // closed until the planned Linux syscall-interception backend lands.
        #[cfg(not(target_os = "macos"))]
        if !protected_file_names.is_empty() {
            return Err(SandboxError::UnsupportedPolicy(
                "protected file names are not yet supported by this platform's sandbox".to_string(),
            ));
        }
        // Bubblewrap can only hide paths by mounting over them, which neither
        // covers paths created later nor allows reads of paths underneath.
        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        let paths_disallowing_reads_supported = false;
        #[cfg(target_os = "windows")]
        let paths_disallowing_reads_supported =
            paths_disallowing_reads.is_empty() || windows_psec::supports_fs_deny();
        #[cfg(not(target_os = "macos"))]
        if !paths_disallowing_reads_supported
            && (!paths_disallowing_reads.is_empty() || !paths_allowing_reads.is_empty())
        {
            return Err(SandboxError::UnsupportedPolicy(
                "paths disallowing reads are not yet supported by this platform's sandbox"
                    .to_string(),
            ));
        }
        // Bubblewrap's seccomp filter denies every `AF_UNIX` socket, and its
        // network namespace only reaches the proxy, so neither grant can be
        // honoured there.
        #[cfg(not(target_os = "macos"))]
        if !policy.allowed_unix_sockets.is_empty() {
            return Err(SandboxError::UnsupportedPolicy(
                "Unix socket grants are not yet supported by this platform's sandbox".to_string(),
            ));
        }
        #[cfg(not(target_os = "macos"))]
        if let SandboxNetPolicy::Restricted {
            allowed_tcp_ports, ..
        } = &policy.network
            && !allowed_tcp_ports.is_empty()
        {
            return Err(SandboxError::UnsupportedPolicy(
                "TCP port grants are not yet supported by this platform's sandbox".to_string(),
            ));
        }

        let fs = FsSetup {
            allow_fs_write,
            writable_paths,
            protected_paths,
            #[cfg(target_os = "macos")]
            protected_file_names,
            #[cfg(any(target_os = "macos", target_os = "windows"))]
            paths_disallowing_reads,
            #[cfg(any(target_os = "macos", target_os = "windows"))]
            paths_allowing_reads,
        };

        let network = match policy.network {
            SandboxNetPolicy::Unrestricted => NetSetup::Unrestricted,
            SandboxNetPolicy::Blocked => NetSetup::Blocked,
            SandboxNetPolicy::Restricted {
                allowed_domains,
                allowed_tcp_ports,
            } => resolve_restricted_network(&allowed_domains, &allowed_tcp_ports)?,
        };

        Ok(Self {
            fs,
            network,
            #[cfg(target_os = "macos")]
            allowed_unix_sockets: policy.allowed_unix_sockets,
            proxy: None,
            network_events: None,
            #[cfg(target_os = "linux")]
            validation_fd_sender: None,
            #[cfg(target_os = "macos")]
            seatbelt_config: None,
        })
    }

    /// Like [`Sandbox::new`], but also forward every event of the network
    /// proxy to `network_events`: its `Ready` port, then each request it allows
    /// or denies (with the host and [`DenyReason`]), and each allowed request's
    /// completion. Events are still logged as with [`Sandbox::new`].
    ///
    /// Only a [`SandboxNetPolicy::Restricted`] policy runs a proxy; for any other
    /// network policy no events are sent.
    ///
    /// # Errors
    ///
    /// The same as [`Sandbox::new`].
    pub fn with_network_events(
        policy: SandboxPolicy,
        network_events: UnboundedSender<ProxyEvent>,
    ) -> Result<Self, SandboxError> {
        let mut sandbox = Self::new(policy)?;
        sandbox.network_events = Some(network_events);
        Ok(sandbox)
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
            let _ = policy;
            windows_psec::check_available().map_err(map_anyhow_error)
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
            self.wrap_windows_psec(command)
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
    fn ensure_restricted_proxy(
        &mut self,
        env: &mut HashMap<String, String>,
    ) -> Result<Option<(u16, Option<PathBuf>)>, SandboxError> {
        let NetSetup::Restricted { allowlist, .. } = &self.network else {
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
            spawn_proxy_event_logger(events_rx, self.network_events.clone());
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
    #[cfg(any(target_os = "macos", target_os = "linux"))]
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
        // The bind *path* is the verified canonical path of each location — for a
        // persisted grant, `HostFilesystemLocation::reopen` already proved this
        // path's current inode is the approved one (`readlink(fd) == canonical`)
        // and holds the fd open pinning it. The *fd* is what the in-sandbox
        // validator compares the mounted inode against, catching any swap between
        // that host-side check and the mount. The two lists stay in the same
        // order so each fd lines up with its path on the validator side.
        let mut writable_owned: Vec<PathBuf> = Vec::new();
        let mut writable_fds: Vec<std::os::fd::OwnedFd> = Vec::new();
        for location in &self.fs.writable_paths {
            let path = location.linux_canonical_path().to_path_buf();
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
            .map(|location| location.linux_canonical_path().to_path_buf())
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

        let (network, allowed_tcp_ports): (_, &[u16]) = match &self.network {
            NetSetup::Unrestricted => (macos_seatbelt::NetworkAccess::All, &[]),
            NetSetup::Blocked => (macos_seatbelt::NetworkAccess::None, &[]),
            NetSetup::Restricted {
                allowed_tcp_ports, ..
            } => (
                macos_seatbelt::NetworkAccess::LocalhostPort(
                    proxy.as_ref().map(|(port, _)| *port).unwrap_or(0),
                ),
                allowed_tcp_ports,
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
        let paths_disallowing_reads: Vec<&Path> = self
            .fs
            .paths_disallowing_reads
            .iter()
            .map(HostFilesystemLocation::macos_canonical_path)
            .collect();
        let paths_allowing_reads: Vec<&Path> = self
            .fs
            .paths_allowing_reads
            .iter()
            .map(HostFilesystemLocation::macos_canonical_path)
            .collect();
        let allowed_unix_sockets: Vec<&Path> = self
            .allowed_unix_sockets
            .iter()
            .map(HostFilesystemLocation::macos_canonical_path)
            .collect();

        let (program, args, config) = macos_seatbelt::wrap_invocation(
            &command.program,
            &command.args,
            &writable,
            &protected,
            &self.fs.protected_file_names,
            &paths_disallowing_reads,
            &paths_allowing_reads,
            &allowed_unix_sockets,
            allowed_tcp_ports,
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
    fn wrap_windows_psec(
        &mut self,
        command: &CommandAndArgs,
    ) -> Result<WrappedCommand, SandboxError> {
        let mut env = command.env.clone();
        let proxy = self.ensure_restricted_proxy(&mut env)?;
        let network = match &self.network {
            NetSetup::Unrestricted => windows_psec::NetworkAccess::All,
            NetSetup::Blocked => windows_psec::NetworkAccess::None,
            NetSetup::Restricted { .. } => proxy
                .map_or(windows_psec::NetworkAccess::None, |(port, _)| {
                    windows_psec::NetworkAccess::LocalhostPort(port)
                }),
        };
        let spec = windows_psec::build_spec(&self.fs, network).map_err(map_anyhow_error)?;

        let (program, args) =
            windows_psec::wrap_invocation(&spec, command.program.clone(), command.args.clone())
                .map_err(map_anyhow_error)?;

        Ok(WrappedCommand {
            program,
            args,
            env,
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

/// Handle a possible re-exec of Zed as the Linux in-sandbox launcher (bind
/// validator + restricted-network bridge), which bwrap runs before the real
/// command. Does not return if it matches.
///
/// Call this at the top of `main`, before normal argument parsing.
#[doc(hidden)]
pub fn run_sandbox_launcher_if_invoked() {
    #[cfg(target_os = "linux")]
    {
        linux_bubblewrap::run_launcher_if_invoked();
    }
}

/// Handle a possible invocation of the Zed CLI as the Windows launcher, which
/// starts the command in a Process Security Environment. Does not return if it
/// matches.
#[cfg(target_os = "windows")]
#[doc(hidden)]
pub use windows_psec::run_launcher_if_invoked as run_windows_sandbox_launcher_if_invoked;

// The createability probe only needs to know whether a sandbox *can* be built
// (namespaces, `bwrap` availability); it does not enforce any grant, so it runs
// with no writable binds rather than re-deriving paths from the opaque
// locations.

fn resolve_restricted_network(
    allowed_domains: &[String],
    allowed_tcp_ports: &[u16],
) -> Result<NetSetup, SandboxError> {
    let mut patterns = Vec::with_capacity(allowed_domains.len());
    for domain in allowed_domains {
        let pattern = HostPattern::parse(domain).map_err(|error| {
            SandboxError::InvalidRequest(format!("invalid network host '{domain}': {error}"))
        })?;
        patterns.push(pattern);
    }
    if allowed_tcp_ports.contains(&0) {
        return Err(SandboxError::InvalidRequest(
            "TCP port 0 is not a valid destination port".to_string(),
        ));
    }
    Ok(NetSetup::Restricted {
        allowlist: Allowlist::from_patterns(patterns),
        #[cfg(target_os = "macos")]
        allowed_tcp_ports: allowed_tcp_ports.to_vec(),
    })
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
/// on macOS) only honor the lowercase variant. Windows variable names are
/// case-insensitive, so there any existing spelling is replaced by a single
/// entry instead; otherwise an inherited `https_proxy` could win over ours when
/// the environment block is built. `NO_PROXY` is blanked so all
/// egress goes through our proxy unconditionally: an inherited `NO_PROXY`
/// matching an allowlisted host would make the client attempt a direct
/// connection, which the sandbox blocks — surfacing as a confusing "connection
/// refused" instead of a clean policy decision.
fn apply_proxy_env(env: &mut HashMap<String, String>, port: u16) {
    let url = format!("http://127.0.0.1:{port}");
    for (name, value) in [
        ("HTTPS_PROXY", url.as_str()),
        ("HTTP_PROXY", url.as_str()),
        ("ALL_PROXY", url.as_str()),
        ("NO_PROXY", ""),
    ] {
        #[cfg(target_os = "windows")]
        env.retain(|key, _| !key.eq_ignore_ascii_case(name));
        #[cfg(not(target_os = "windows"))]
        env.insert(name.to_ascii_lowercase(), value.to_string());
        env.insert(name.to_string(), value.to_string());
    }
}

/// Drain the proxy's event channel on a background thread, logging each event
/// and forwarding it to `forward_to` until that receiver is dropped. The thread
/// exits when the proxy is dropped and the channel closes.
fn spawn_proxy_event_logger(
    events: futures::channel::mpsc::UnboundedReceiver<ProxyEvent>,
    mut forward_to: Option<UnboundedSender<ProxyEvent>>,
) {
    std::thread::spawn(move || {
        futures::executor::block_on(async move {
            use futures::StreamExt as _;
            let mut events = events;
            while let Some(event) = events.next().await {
                log_proxy_event(&event);
                if let Some(sender) = &forward_to
                    && sender.unbounded_send(event).is_err()
                {
                    forward_to = None;
                }
            }
        });
    });
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
    SandboxError::Other(format!("{error:#}"))
}

#[cfg(all(test, not(target_os = "windows")))]
mod tests {
    use super::*;
    use futures::StreamExt as _;

    #[test]
    fn restricted_fs_rejects_writable_paths_inside_protected_paths() {
        let temp_dir = tempfile::tempdir().expect("create temp dir");
        let repo_dir = temp_dir.path().join("repo");
        let git_dir = repo_dir.join(".git");
        let hooks_dir = git_dir.join("hooks");
        std::fs::create_dir_all(&hooks_dir).expect("create git hooks dir");

        let protected_git = HostFilesystemLocation::capture(&git_dir).expect("capture git dir");
        let writable_git = HostFilesystemLocation::capture(&git_dir).expect("capture git dir");
        let writable_hooks =
            HostFilesystemLocation::capture(&hooks_dir).expect("capture hooks dir");

        for writable_path in [writable_git, writable_hooks] {
            let result = Sandbox::new(SandboxPolicy {
                fs: SandboxFsPolicy::Restricted {
                    writable_paths: vec![writable_path],
                    protected_paths: vec![protected_git.clone()],
                    protected_file_names: Vec::new(),
                    paths_disallowing_reads: Vec::new(),
                    paths_allowing_reads: Vec::new(),
                },
                network: SandboxNetPolicy::Blocked,
                allowed_unix_sockets: Vec::new(),
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

        let protected_git = HostFilesystemLocation::capture(&git_dir).expect("capture git dir");
        std::fs::rename(&git_dir, &real_git_dir).expect("move git dir aside");
        unix_fs::symlink(&outside_dir, &git_dir).expect("replace git dir with symlink");
        let writable_displayed_inside_git =
            HostFilesystemLocation::capture(git_dir.join("hooks")).expect("capture symlink target");

        Sandbox::new(SandboxPolicy {
            fs: SandboxFsPolicy::Restricted {
                writable_paths: vec![writable_displayed_inside_git],
                protected_paths: vec![protected_git],
                protected_file_names: Vec::new(),
                paths_disallowing_reads: Vec::new(),
                paths_allowing_reads: Vec::new(),
            },
            network: SandboxNetPolicy::Blocked,
            allowed_unix_sockets: Vec::new(),
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
            HostFilesystemLocation::capture(dir.path()).expect("capture temp dir")
        };

        let a = SandboxFsPolicy::Restricted {
            writable_paths: vec![location(&dir_a), location(&dir_b)],
            protected_paths: vec![location(&dir_c)],
            protected_file_names: Vec::new(),
            paths_disallowing_reads: Vec::new(),
            paths_allowing_reads: Vec::new(),
        };
        let b = SandboxFsPolicy::Restricted {
            writable_paths: vec![location(&dir_b), location(&dir_c)],
            protected_paths: vec![location(&dir_c), location(&dir_d)],
            protected_file_names: Vec::new(),
            paths_disallowing_reads: Vec::new(),
            paths_allowing_reads: Vec::new(),
        };
        assert_eq!(
            a.clone().merge(b),
            SandboxFsPolicy::Restricted {
                writable_paths: vec![location(&dir_a), location(&dir_b), location(&dir_c)],
                protected_paths: vec![location(&dir_c), location(&dir_d)],
                protected_file_names: Vec::new(),
                paths_disallowing_reads: Vec::new(),
                paths_allowing_reads: Vec::new(),
            }
        );
        assert_eq!(
            a.merge(SandboxFsPolicy::Unrestricted {
                protected_paths: vec![location(&dir_a)],
                protected_file_names: Vec::new(),
                paths_disallowing_reads: Vec::new(),
                paths_allowing_reads: Vec::new(),
            }),
            SandboxFsPolicy::Unrestricted {
                protected_paths: vec![location(&dir_c), location(&dir_a)],
                protected_file_names: Vec::new(),
                paths_disallowing_reads: Vec::new(),
                paths_allowing_reads: Vec::new(),
            }
        );
        assert_eq!(
            SandboxFsPolicy::Unrestricted {
                protected_paths: vec![location(&dir_d)],
                protected_file_names: Vec::new(),
                paths_disallowing_reads: Vec::new(),
                paths_allowing_reads: Vec::new(),
            }
            .merge(SandboxFsPolicy::Restricted {
                writable_paths: vec![location(&dir_a)],
                protected_paths: vec![location(&dir_c)],
                protected_file_names: Vec::new(),
                paths_disallowing_reads: Vec::new(),
                paths_allowing_reads: Vec::new(),
            }),
            SandboxFsPolicy::Unrestricted {
                protected_paths: vec![location(&dir_d), location(&dir_c)],
                protected_file_names: Vec::new(),
                paths_disallowing_reads: Vec::new(),
                paths_allowing_reads: Vec::new(),
            }
        );
    }

    #[test]
    fn protected_file_name_rejects_anything_but_one_path_component() {
        for name in ["", ".", "..", "a/b", "/", ".envrc/", "a\0b"] {
            assert!(
                matches!(
                    ProtectedFileName::new(name),
                    Err(SandboxError::InvalidRequest(_))
                ),
                "{name:?} should be rejected"
            );
        }
        for name in [".envrc", "...", "a.b+c", "with space"] {
            assert_eq!(ProtectedFileName::new(name).unwrap().as_str(), name);
        }
    }

    #[test]
    fn net_merge_unrestricted_dominates_blocked_is_identity_else_unions() {
        let hosts = |list: &[&str]| SandboxNetPolicy::Restricted {
            allowed_domains: list.iter().map(|s| s.to_string()).collect(),
            allowed_tcp_ports: Vec::new(),
        };
        assert_eq!(
            hosts(&["a.com", "b.com"]).merge(hosts(&["b.com", "c.com"])),
            hosts(&["a.com", "b.com", "c.com"])
        );
        let ports = |list: &[u16]| SandboxNetPolicy::Restricted {
            allowed_domains: Vec::new(),
            allowed_tcp_ports: list.to_vec(),
        };
        assert_eq!(ports(&[22]).merge(ports(&[5432, 22])), ports(&[22, 5432]));
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

    #[test]
    fn restricted_network_rejects_tcp_port_zero() {
        let result = Sandbox::new(SandboxPolicy {
            fs: unrestricted_fs(),
            network: SandboxNetPolicy::Restricted {
                allowed_domains: Vec::new(),
                allowed_tcp_ports: vec![0],
            },
            allowed_unix_sockets: Vec::new(),
        });
        assert!(matches!(
            result,
            Err(SandboxError::InvalidRequest(_) | SandboxError::UnsupportedPolicy(_))
        ));
    }

    #[test]
    fn network_events_report_denied_hosts() {
        let policy = SandboxPolicy {
            fs: unrestricted_fs(),
            network: SandboxNetPolicy::Restricted {
                allowed_domains: vec!["allowed.example".to_string()],
                allowed_tcp_ports: Vec::new(),
            },
            allowed_unix_sockets: Vec::new(),
        };
        if !host_can_run_sandbox(&policy) {
            return;
        }
        let (events_sender, events) = futures::channel::mpsc::unbounded();
        let mut sandbox =
            Sandbox::with_network_events(policy, events_sender).expect("create sandbox");

        // The proxy denies the host before resolving it, so this needs no
        // network access.
        run_sandboxed(
            &mut sandbox,
            "/usr/bin/curl",
            &["--silent", "--max-time", "10", "https://denied.example/"],
        );
        sandbox.drop_on_current_thread();

        let events: Vec<ProxyEvent> = futures::executor::block_on(events.collect());
        assert!(matches!(events.first(), Some(ProxyEvent::Ready { .. })));
        assert!(
            events.iter().any(|event| matches!(
                event,
                ProxyEvent::RequestAttempt {
                    host,
                    outcome: RequestOutcome::Denied {
                        reason: DenyReason::HostNotInAllowlist { .. }
                    },
                    ..
                } if host == "denied.example"
            )),
            "expected a denied attempt for denied.example, got {events:?}"
        );
    }

    #[test]
    fn restricted_network_allows_only_the_proxy_and_granted_tcp_ports() {
        // The kernel completes `nc -z`'s connect from the listen backlog, so the
        // listeners never need to accept.
        let granted_listener =
            std::net::TcpListener::bind("127.0.0.1:0").expect("bind granted listener");
        let other_listener =
            std::net::TcpListener::bind("127.0.0.1:0").expect("bind other listener");
        let granted_port = granted_listener
            .local_addr()
            .expect("granted listener address")
            .port();
        let other_port = other_listener
            .local_addr()
            .expect("other listener address")
            .port();

        let (events_sender, mut events) = futures::channel::mpsc::unbounded();
        let Some(mut sandbox) = enforced_or_refused(Sandbox::with_network_events(
            SandboxPolicy {
                fs: unrestricted_fs(),
                network: SandboxNetPolicy::Restricted {
                    allowed_domains: vec!["example.com".to_string()],
                    allowed_tcp_ports: vec![granted_port],
                },
                allowed_unix_sockets: Vec::new(),
            },
            events_sender,
        )) else {
            return;
        };

        let tcp_connects = |sandbox: &mut Sandbox, port: u16| {
            run_sandboxed(
                sandbox,
                "/usr/bin/nc",
                &["-z", "-w", "5", "127.0.0.1", &port.to_string()],
            )
            .status
            .success()
        };
        assert!(tcp_connects(&mut sandbox, granted_port), "granted port");
        let proxy_port = match futures::executor::block_on(events.next()) {
            Some(ProxyEvent::Ready { port }) => port,
            other => panic!("expected the proxy's Ready event, got {other:?}"),
        };
        assert!(tcp_connects(&mut sandbox, proxy_port), "proxy port");
        assert!(
            !tcp_connects(&mut sandbox, other_port),
            "reached a TCP port that was not granted"
        );

        sandbox.drop_on_current_thread();
    }

    #[test]
    fn only_granted_unix_sockets_are_reachable() {
        use std::io::ErrorKind;
        use std::os::unix::net::UnixListener;
        use std::time::{Duration, Instant};

        // Bind under `/tmp` rather than the default temp dir: macOS temp paths
        // (`/var/folders/...`) overflow the `sun_path` limit for Unix sockets.
        // `/tmp` is a symlink, so this also checks that the grant names the
        // resolved path Seatbelt matches against.
        let socket_dir = tempfile::Builder::new()
            .prefix("zed-sock-")
            .tempdir_in("/tmp")
            .expect("create socket dir");
        let granted_socket = socket_dir.path().join("granted.sock");
        let other_socket = socket_dir.path().join("other.sock");
        let granted_listener = UnixListener::bind(&granted_socket).expect("bind granted socket");
        let other_listener = UnixListener::bind(&other_socket).expect("bind other socket");
        granted_listener
            .set_nonblocking(true)
            .expect("make granted listener non-blocking");
        other_listener
            .set_nonblocking(true)
            .expect("make other listener non-blocking");

        for network in [
            SandboxNetPolicy::Blocked,
            SandboxNetPolicy::Restricted {
                allowed_domains: Vec::new(),
                allowed_tcp_ports: Vec::new(),
            },
        ] {
            let Some(mut sandbox) = enforced_or_refused(Sandbox::new(SandboxPolicy {
                fs: unrestricted_fs(),
                network: network.clone(),
                allowed_unix_sockets: vec![
                    HostFilesystemLocation::capture(&granted_socket).expect("capture socket"),
                ],
            })) else {
                continue;
            };

            let output = run_sandboxed(
                &mut sandbox,
                "/usr/bin/nc",
                &["-U", "-w", "1", &granted_socket.display().to_string()],
            );
            let deadline = Instant::now() + Duration::from_secs(10);
            let accepted = loop {
                match granted_listener.accept() {
                    Ok(_) => break true,
                    Err(error) if error.kind() == ErrorKind::WouldBlock => {
                        if Instant::now() >= deadline {
                            break false;
                        }
                        std::thread::sleep(Duration::from_millis(20));
                    }
                    Err(_) => break false,
                }
            };
            assert!(
                accepted,
                "granted socket was not reached with {network:?}: {}",
                String::from_utf8_lossy(&output.stderr)
            );

            let output = run_sandboxed(
                &mut sandbox,
                "/usr/bin/nc",
                &["-U", "-w", "1", &other_socket.display().to_string()],
            );
            assert!(
                !output.status.success(),
                "connected to an ungranted socket with {network:?}"
            );
            assert!(matches!(
                other_listener.accept(),
                Err(error) if error.kind() == ErrorKind::WouldBlock
            ));

            sandbox.drop_on_current_thread();
        }
    }

    #[test]
    fn policy_merge_unions_unix_sockets() {
        let socket_dir = tempfile::tempdir().expect("create socket dir");
        let socket = |name: &str| {
            let path = socket_dir.path().join(name);
            // Linux can only capture a path that exists.
            std::fs::write(&path, "").expect("create socket placeholder");
            HostFilesystemLocation::capture(path).expect("capture socket path")
        };
        let policy = |sockets: Vec<HostFilesystemLocation>| SandboxPolicy {
            fs: unrestricted_fs(),
            network: SandboxNetPolicy::Blocked,
            allowed_unix_sockets: sockets,
        };
        assert_eq!(
            policy(vec![socket("a.sock")]).merge(policy(vec![socket("b.sock"), socket("a.sock")])),
            policy(vec![socket("a.sock"), socket("b.sock")])
        );
    }

    #[test]
    fn restricted_fs_hides_paths_disallowing_reads_except_their_exceptions() {
        let project_dir = tempfile::tempdir().expect("create project dir");
        let readme_file = project_dir.path().join("README");
        std::fs::write(&readme_file, "public\n").expect("write readme");
        let secrets_dir = project_dir.path().join("secrets");
        std::fs::create_dir(&secrets_dir).expect("create secrets dir");
        let secret_file = secrets_dir.join("id_ed25519");
        std::fs::write(&secret_file, "private\n").expect("write secret");
        let kept_dir = secrets_dir.join("kept");
        std::fs::create_dir(&kept_dir).expect("create kept dir");
        let kept_file = kept_dir.join("notes");
        std::fs::write(&kept_file, "kept\n").expect("write kept file");
        let locked_file = kept_dir.join("locked");
        std::fs::write(&locked_file, "locked\n").expect("write locked file");

        let Some(mut sandbox) = enforced_or_refused(Sandbox::new(SandboxPolicy {
            fs: SandboxFsPolicy::Restricted {
                writable_paths: vec![location(project_dir.path())],
                protected_paths: vec![location(&locked_file)],
                protected_file_names: Vec::new(),
                paths_disallowing_reads: vec![location(&secrets_dir)],
                paths_allowing_reads: vec![location(&kept_dir)],
            },
            network: SandboxNetPolicy::Blocked,
            allowed_unix_sockets: Vec::new(),
        })) else {
            return;
        };

        let output = run_shell(&mut sandbox, &format!("cat '{}'", readme_file.display()));
        assert_eq!(String::from_utf8_lossy(&output.stdout), "public\n");
        let output = run_shell(&mut sandbox, &format!("cat '{}'", secret_file.display()));
        assert!(!output.status.success(), "read inside a hidden path");
        assert!(output.stdout.is_empty(), "hidden bytes leaked");
        let output = run_shell(&mut sandbox, &format!("ls '{}'", secrets_dir.display()));
        assert!(!output.status.success(), "listed a hidden path");
        let hidden_written = secrets_dir.join("written");
        let output = run_shell(
            &mut sandbox,
            &format!("echo hello > '{}'", hidden_written.display()),
        );
        assert!(!output.status.success(), "wrote inside a hidden path");
        assert!(!hidden_written.exists());

        let output = run_shell(&mut sandbox, &format!("cat '{}'", kept_file.display()));
        assert_eq!(String::from_utf8_lossy(&output.stdout), "kept\n");
        let kept_written = kept_dir.join("written");
        let output = run_shell(
            &mut sandbox,
            &format!("echo hello > '{}'", kept_written.display()),
        );
        assert!(
            output.status.success(),
            "write inside an exception: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let output = run_shell(
            &mut sandbox,
            &format!("echo changed > '{}'", locked_file.display()),
        );
        assert!(
            !output.status.success(),
            "protected path inside an exception"
        );
        assert_eq!(
            std::fs::read_to_string(&locked_file).expect("read locked file"),
            "locked\n"
        );
    }

    #[test]
    fn unrestricted_fs_hides_not_yet_created_paths_disallowing_reads() {
        let root_dir = tempfile::tempdir().expect("create root dir");
        // Two missing levels under a temp dir reached through the `/var`
        // symlink, so the rule only matches if the location was resolved
        // through its nearest existing ancestor.
        let hidden_dir = root_dir.path().join("missing/hidden");
        let exception_dir = hidden_dir.join("kept");
        // A platform that can't capture a path that doesn't exist yet refuses
        // the policy at this point, before a sandbox is built.
        let (Ok(hidden), Ok(exception)) = (
            HostFilesystemLocation::capture(&hidden_dir),
            HostFilesystemLocation::capture(&exception_dir),
        ) else {
            assert!(
                !cfg!(target_os = "macos"),
                "macOS must capture paths that don't exist yet"
            );
            return;
        };
        let Some(mut sandbox) = enforced_or_refused(Sandbox::new(SandboxPolicy {
            fs: SandboxFsPolicy::Unrestricted {
                protected_paths: Vec::new(),
                protected_file_names: Vec::new(),
                paths_disallowing_reads: vec![hidden],
                paths_allowing_reads: vec![exception],
            },
            network: SandboxNetPolicy::Blocked,
            allowed_unix_sockets: Vec::new(),
        })) else {
            return;
        };

        let ordinary_file = root_dir.path().join("missing/ordinary");
        let output = run_shell(
            &mut sandbox,
            &format!(
                "mkdir -p '{}' && echo hello > '{}'",
                root_dir.path().join("missing").display(),
                ordinary_file.display()
            ),
        );
        assert!(
            output.status.success(),
            "unrestricted write: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let output = run_shell(&mut sandbox, &format!("mkdir '{}'", hidden_dir.display()));
        assert!(!output.status.success(), "created a hidden path");
        assert!(!hidden_dir.exists());

        std::fs::create_dir_all(&exception_dir).expect("create exception dir");
        let exception_file = exception_dir.join("written");
        let output = run_shell(
            &mut sandbox,
            &format!("echo hello > '{}'", exception_file.display()),
        );
        assert!(
            output.status.success(),
            "write inside an exception: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let output = run_shell(
            &mut sandbox,
            &format!("echo hello > '{}'", hidden_dir.join("other").display()),
        );
        assert!(!output.status.success(), "wrote inside a hidden path");
    }

    #[test]
    fn paths_disallowing_reads_inside_allowing_are_rejected() {
        let root_dir = tempfile::tempdir().expect("create root dir");
        let outer = root_dir.path().join("outer");
        let exception = outer.join("exception");
        let inner = exception.join("inner");
        std::fs::create_dir_all(&inner).expect("create nested dirs");
        let result = Sandbox::new(SandboxPolicy {
            fs: SandboxFsPolicy::Unrestricted {
                protected_paths: Vec::new(),
                protected_file_names: Vec::new(),
                paths_disallowing_reads: vec![location(&outer), location(&inner)],
                paths_allowing_reads: vec![location(&exception)],
            },
            network: SandboxNetPolicy::Blocked,
            allowed_unix_sockets: Vec::new(),
        });
        assert!(matches!(result, Err(SandboxError::InvalidRequest(_))));
    }

    #[test]
    fn fs_merge_keeps_only_allowing_both_layers_leave_visible() {
        let root_dir = tempfile::tempdir().expect("create root dir");
        let data_dir = root_dir.path().join("data");
        let kept_dir = data_dir.join("kept");
        let nested_dir = kept_dir.join("nested");
        let other_dir = root_dir.path().join("other");
        std::fs::create_dir_all(&nested_dir).expect("create nested dir");
        std::fs::create_dir_all(&other_dir).expect("create other dir");
        let hiding = |hidden: &[&Path], exceptions: &[&Path]| SandboxFsPolicy::Unrestricted {
            protected_paths: Vec::new(),
            protected_file_names: Vec::new(),
            paths_disallowing_reads: hidden.iter().map(|path| location(path)).collect(),
            paths_allowing_reads: exceptions.iter().map(|path| location(path)).collect(),
        };

        // A layer that hides `data` without an exception for `kept` drops it.
        assert_eq!(
            hiding(&[&data_dir], &[&kept_dir]).merge(hiding(&[&data_dir], &[])),
            hiding(&[&data_dir], &[])
        );
        // A layer that hides something inside the exception drops it too.
        assert_eq!(
            hiding(&[&data_dir], &[&kept_dir]).merge(hiding(&[&nested_dir], &[])),
            hiding(&[&data_dir], &[])
        );
        // Exceptions survive where the other layer hides nothing of theirs.
        assert_eq!(
            hiding(&[&data_dir], &[&kept_dir]).merge(hiding(&[&other_dir], &[])),
            hiding(&[&data_dir, &other_dir], &[&kept_dir])
        );
        assert_eq!(
            hiding(&[&data_dir], &[&kept_dir]).merge(hiding(&[&data_dir], &[&kept_dir])),
            hiding(&[&data_dir], &[&kept_dir])
        );
    }

    /// The sandbox for a policy that only some platforms can enforce, or
    /// `None` where the platform refuses it.
    ///
    /// A platform must do one or the other: accepting such a policy without
    /// enforcing it would run the command with access the caller withheld. A
    /// test that gets a sandbox back goes on to check the enforcement, so the
    /// same body holds a platform to whichever of the two it claims.
    fn enforced_or_refused(result: Result<Sandbox, SandboxError>) -> Option<Sandbox> {
        match result {
            Ok(sandbox) => Some(sandbox),
            Err(SandboxError::UnsupportedPolicy(message)) => {
                assert!(
                    !cfg!(target_os = "macos"),
                    "macOS must enforce every policy: {message}"
                );
                None
            }
            Err(error) => panic!("expected a sandbox or an unsupported policy, got {error}"),
        }
    }

    /// Whether this host can run commands under `policy`, which every platform
    /// supports. A Linux host without a usable Bubblewrap cannot; the test
    /// then skips, unless `ZED_TEST_SANDBOX_REQUIRE_ENFORCED` is set, which
    /// makes that a failure.
    fn host_can_run_sandbox(policy: &SandboxPolicy) -> bool {
        match Sandbox::can_create(policy) {
            Ok(()) => true,
            Err(error) => {
                assert!(
                    std::env::var_os("ZED_TEST_SANDBOX_REQUIRE_ENFORCED").is_none(),
                    "this host cannot run the sandbox: {error}"
                );
                eprintln!("skipping: this host cannot run the sandbox: {error}");
                false
            }
        }
    }

    fn location(path: &Path) -> HostFilesystemLocation {
        HostFilesystemLocation::capture(path).expect("capture location")
    }

    fn run_shell(sandbox: &mut Sandbox, script: &str) -> Output {
        run_sandboxed(sandbox, "/bin/sh", &["-c", script])
    }

    fn unrestricted_fs() -> SandboxFsPolicy {
        SandboxFsPolicy::Unrestricted {
            protected_paths: Vec::new(),
            protected_file_names: Vec::new(),
            paths_disallowing_reads: Vec::new(),
            paths_allowing_reads: Vec::new(),
        }
    }

    fn run_sandboxed(sandbox: &mut Sandbox, program: &str, args: &[&str]) -> Output {
        let command = CommandAndArgs {
            program: program.to_string(),
            args: args.iter().map(|argument| argument.to_string()).collect(),
            env: HashMap::new(),
            cwd: None,
        };
        futures::executor::block_on(sandbox.execute(&command)).expect("run sandboxed command")
    }
}

/// A directory that is about to be granted as a sandbox write path but may not
/// exist yet. Preparing one resolves the platform difference in how a
/// not-yet-existing write grant is materialized, while keeping the same security
/// property: the caller shows [`Self::canonical_path`] to the user and records
/// *that* as the grant, so approval is always against the real, symlink-resolved
/// target.
///
/// - **Linux and Windows**: bubblewrap can only bind an existing inode, and a
///   Process Security Environment can only grant an existing directory, so the
///   missing directory (and any missing parents) is created **eagerly**, per
///   component, and its canonical path is read back. If the grant is denied,
///   [`Self::discard`] removes exactly the directories that were created,
///   deepest-first, following no symlinks (`rmdir` only removes empty dirs and
///   never traverses a swapped-in symlink).
/// - **macOS**: Seatbelt resolves paths at syscall time and can grant a missing
///   path, so nothing is created here; the directory is materialized only after
///   approval via [`Self::finalize`].
///
/// The eventual bind is still protected by the usual capture-and-revalidate path
/// (`HostFilesystemLocation`), which re-pins the inode when the command runs.
pub struct GrantableWriteDir {
    canonical_path: PathBuf,
    /// The path exactly as requested, carried so the caller can persist and
    /// display the `(raw, canonical)` pair (see [`HostFilesystemLocation`]).
    untrusted_raw_path: PathBuf,
    /// Directories created eagerly to pin the inode, shallowest-first. Empty on
    /// platforms that defer creation to [`Self::finalize`].
    eagerly_created: Vec<PathBuf>,
}

impl GrantableWriteDir {
    /// Prepare `path` for use as a sandbox write grant. `path` must be absolute.
    pub fn prepare(path: &Path) -> std::io::Result<Self> {
        let untrusted_raw_path = path.to_path_buf();
        #[cfg(any(target_os = "linux", target_os = "windows"))]
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
                untrusted_raw_path,
                eagerly_created,
            })
        }
        #[cfg(target_os = "macos")]
        {
            Ok(Self {
                canonical_path: canonicalize_allowing_missing_leaf(path),
                untrusted_raw_path,
                eagerly_created: Vec::new(),
            })
        }
        #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
        {
            let _ = untrusted_raw_path;
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

    /// The path exactly as requested, for persisting alongside the canonical
    /// path and for the "requested → granted" approval disclosure.
    pub fn untrusted_raw_path(&self) -> &Path {
        &self.untrusted_raw_path
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
#[cfg(any(target_os = "linux", target_os = "windows"))]
fn create_missing_dirs(path: &Path, created: &mut Vec<PathBuf>) -> std::io::Result<()> {
    let mut cur = PathBuf::new();
    for component in path.components() {
        cur.push(component);
        if !matches!(component, std::path::Component::Normal(_)) {
            continue;
        }
        match std::fs::create_dir(&cur) {
            Ok(()) => created.push(cur.clone()),
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
            Err(error) => return Err(error),
        }
    }
    Ok(())
}

/// Read back the canonical path of `path` (on Linux, through an `O_PATH` handle
/// to the inode), so the value shown to the user reflects the real target even
/// when a component is a symlink or junction.
#[cfg(any(target_os = "linux", target_os = "windows"))]
fn pinned_canonical_path(path: &Path) -> std::io::Result<PathBuf> {
    Ok(CanonicalPathBuf::resolve(path)?.into_path())
}

#[cfg(all(test, any(target_os = "linux", target_os = "windows")))]
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
        // Windows grant paths never carry the `\\?\` prefix `canonicalize` adds.
        #[cfg(target_os = "windows")]
        let expected = dunce::canonicalize(&target).unwrap();
        #[cfg(not(target_os = "windows"))]
        let expected = target.canonicalize().unwrap();
        assert_eq!(prepared.canonical_path(), expected);

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
    #[cfg(target_os = "linux")]
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
