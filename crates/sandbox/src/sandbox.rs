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
use http_proxy::{Allowlist, HostPattern, ProxyConfig, UpstreamProxy};

pub use http_proxy::{DenyReason, ProxyEvent, RequestMethod, RequestOutcome};

#[cfg(target_os = "linux")]
mod linux_bubblewrap;

#[cfg(target_os = "macos")]
mod macos_seatbelt;

#[cfg(target_os = "windows")]
mod windows_wsl;

mod util;

#[cfg(target_os = "macos")]
use util::canonicalize_allowing_missing_leaf;
#[cfg(target_os = "linux")]
use util::{CanonicalPathBuf, linux_fd_identity};
pub use util::{
    HostFilesystemLocation, HostFilesystemLocationDisplay, normalize_host_filesystem_locations,
    resolve_canonical,
};
#[cfg(target_os = "windows")]
pub use windows_wsl::{ResolvedGrant, resolve_canonical_for_grant};

#[cfg(target_os = "windows")]
pub(crate) const WSL_SANDBOX_UNAVAILABLE_PREFIX: &str = "Windows sandboxing via WSL is unavailable";

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
    /// IP networking is blocked. Connecting to a socket never permits sending
    /// packets to other machines. Only the macOS backend supports this; Linux
    /// and WSL reject a policy that lists any.
    pub allowed_unix_sockets: Vec<PathBuf>,
}

/// Filesystem policy for a sandboxed command.
///
/// In both variants, `protected_paths` stay readable but not writable, and
/// `unreadable_paths` are masked: they can be neither read, listed, nor
/// written. `readable_paths` are windows into masked subtrees: each behaves
/// as if it were outside the mask, so it is readable, and writable where the
/// rest of the policy makes it so (an unrestricted policy, or a writable path
/// containing it). Windows are the only way into a masked subtree: a writable
/// path inside an unreadable path must lie inside a window, or
/// [`Sandbox::new`] rejects the policy.
///
/// Backends resolve overlaps in this order, each layer overriding the ones
/// before it: the writable paths (or unrestricted writes), then the unreadable
/// mask, then the readable windows (writable where they overlap the writable
/// layer), then the protected paths, which nothing overrides.
///
/// Only the macOS backend supports `unreadable_paths` and `readable_paths`;
/// Linux and WSL reject a policy that lists any.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SandboxFsPolicy {
    /// Allow unrestricted filesystem writes except for protected and
    /// unreadable paths.
    Unrestricted {
        protected_paths: Vec<HostFilesystemLocation>,
        unreadable_paths: Vec<HostFilesystemLocation>,
        readable_paths: Vec<HostFilesystemLocation>,
    },
    /// Writes are confined to these locations (and the standard ephemeral
    /// locations the platform provides). Each is a [`HostFilesystemLocation`]
    /// captured at validation time, never a bare path the enforcement layer
    /// would re-resolve.
    Restricted {
        writable_paths: Vec<HostFilesystemLocation>,
        protected_paths: Vec<HostFilesystemLocation>,
        unreadable_paths: Vec<HostFilesystemLocation>,
        readable_paths: Vec<HostFilesystemLocation>,
    },
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
        /// bypassing the proxy, for protocols a CONNECT proxy cannot carry
        /// (SSH on port 22, for example).
        ///
        /// A port grant is host-wide: the sandbox cannot filter raw TCP by
        /// hostname, so granting port 22 allows SSH to *any* host. Port 0 is
        /// rejected at [`Sandbox::new`]. Only the macOS backend supports this;
        /// Linux and WSL reject a policy that lists any.
        allowed_tcp_ports: Vec<u16>,
    },
}

/// Host paths that should remain readable but not writable, even when they fall
/// under a writable subtree.
///
/// The caller computes this list because the sandbox layer does not know which
/// application-specific paths need stronger protection.
pub type ProtectedPaths = Vec<HostFilesystemLocation>;

/// Whether `path` (a host path) resides on a Windows drive reached through WSL's
/// DrvFs, where sandbox-integrity guarantees are weaker than on the distro's
/// native filesystem. Always `false` off Windows, where no such filesystem
/// exists. This is a cheap structural check on the path shape; the authoritative
/// per-grant classification is done in WSL by [`resolve_canonical_for_grant`].
pub fn path_is_on_windows_drive(path: &Path) -> bool {
    #[cfg(target_os = "windows")]
    {
        windows_wsl::path_is_on_windows_drive(path)
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = path;
        false
    }
}

/// Whether WSL is structurally present (at least one registered distro), via a
/// cheap registry read with no `wsl.exe` round-trip. Always `false` off
/// Windows. Suitable only for UI gating (e.g. suppressing DrvFs
/// sandbox-integrity warnings on machines where no WSL sandbox could ever be
/// constructed); enforcement paths must keep relying on the real in-WSL probe,
/// which is authoritative about whether a sandbox can actually be built.
pub fn wsl_distro_registered() -> bool {
    #[cfg(target_os = "windows")]
    {
        windows_wsl::wsl_distro_registered()
    }
    #[cfg(not(target_os = "windows"))]
    {
        false
    }
}

impl SandboxPolicy {
    /// Combine two policy layers. Filesystem/network grants are unioned into the
    /// least-restrictive policy that satisfies both layers, while protected and
    /// unreadable paths are unioned so every layer's restrictions remain in
    /// force.
    pub fn merge(self, other: SandboxPolicy) -> SandboxPolicy {
        SandboxPolicy {
            fs: self.fs.merge(other.fs),
            network: self.network.merge(other.network),
            allowed_unix_sockets: union_in_order(
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
}

/// Append each item of `other` that `items` doesn't already contain.
fn union_in_order<T: PartialEq>(mut items: Vec<T>, other: Vec<T>) -> Vec<T> {
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

/// A writable path inside a masked subtree is re-allowed after the mask, so it
/// must sit inside a window; otherwise the mask would hide a subtree the command
/// could still write.
fn validate_writable_paths_inside_unreadable_paths_are_windowed(
    writable_paths: &[HostFilesystemLocation],
    unreadable_paths: &[HostFilesystemLocation],
    readable_paths: &[HostFilesystemLocation],
) -> Result<(), SandboxError> {
    for writable_path in writable_paths {
        for unreadable_path in unreadable_paths {
            if writable_path.is_within(unreadable_path)
                && !readable_paths
                    .iter()
                    .any(|readable_path| writable_path.is_within(readable_path))
            {
                return Err(SandboxError::InvalidRequest(format!(
                    "writable sandbox path `{}` lies inside unreadable path `{}` without a readable path containing it",
                    writable_path.untrusted_path_display(),
                    unreadable_path.untrusted_path_display()
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
        .untrusted_raw_path()
        .starts_with(protected_path.untrusted_raw_path())
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
    /// Unrestricted writes dominate; otherwise the writable subtrees union.
    /// Protected, unreadable, and readable paths always union.
    pub fn merge(self, other: SandboxFsPolicy) -> SandboxFsPolicy {
        match (self, other) {
            (
                SandboxFsPolicy::Restricted {
                    writable_paths: writable_a,
                    protected_paths: protected_a,
                    unreadable_paths: unreadable_a,
                    readable_paths: readable_a,
                },
                SandboxFsPolicy::Restricted {
                    writable_paths: writable_b,
                    protected_paths: protected_b,
                    unreadable_paths: unreadable_b,
                    readable_paths: readable_b,
                },
            ) => SandboxFsPolicy::Restricted {
                writable_paths: merge_locations(writable_a, writable_b),
                protected_paths: merge_locations(protected_a, protected_b),
                unreadable_paths: merge_locations(unreadable_a, unreadable_b),
                readable_paths: merge_locations(readable_a, readable_b),
            },
            (
                SandboxFsPolicy::Unrestricted {
                    protected_paths: protected_a,
                    unreadable_paths: unreadable_a,
                    readable_paths: readable_a,
                }
                | SandboxFsPolicy::Restricted {
                    protected_paths: protected_a,
                    unreadable_paths: unreadable_a,
                    readable_paths: readable_a,
                    ..
                },
                SandboxFsPolicy::Unrestricted {
                    protected_paths: protected_b,
                    unreadable_paths: unreadable_b,
                    readable_paths: readable_b,
                }
                | SandboxFsPolicy::Restricted {
                    protected_paths: protected_b,
                    unreadable_paths: unreadable_b,
                    readable_paths: readable_b,
                    ..
                },
            ) => SandboxFsPolicy::Unrestricted {
                protected_paths: merge_locations(protected_a, protected_b),
                unreadable_paths: merge_locations(unreadable_a, unreadable_b),
                readable_paths: merge_locations(readable_a, readable_b),
            },
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
                    allowed_domains,
                    allowed_tcp_ports,
                },
                SandboxNetPolicy::Restricted {
                    allowed_domains: other_domains,
                    allowed_tcp_ports: other_ports,
                },
            ) => SandboxNetPolicy::Restricted {
                allowed_domains: union_in_order(allowed_domains, other_domains),
                allowed_tcp_ports: union_in_order(allowed_tcp_ports, other_ports),
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
    #[cfg(target_os = "macos")]
    unreadable_paths: Vec<HostFilesystemLocation>,
    #[cfg(target_os = "macos")]
    readable_paths: Vec<HostFilesystemLocation>,
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
    /// Each allowed socket as requested plus, where it differs, its canonical
    /// form, since it isn't guaranteed which one Seatbelt matches against.
    #[cfg(target_os = "macos")]
    allowed_unix_socket_paths: Vec<PathBuf>,
    /// In-process network proxy for the restricted-network case, spawned on the
    /// first `wrap`. Dropped on a background thread (the join blocks).
    proxy: Option<ProxyHandle>,
    /// Where the proxy reports its events; `None` falls back to debug logging.
    #[cfg(not(target_os = "windows"))]
    network_events: Option<futures::channel::mpsc::UnboundedSender<ProxyEvent>>,
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
        Self::build(policy, None)
    }

    /// Like [`Sandbox::new`], but report the network proxy's events (the
    /// `Ready` port, then each allowed or denied request) to `network_events`
    /// instead of logging them, so denied hosts can be turned into user-facing
    /// approval prompts. Events only exist for a
    /// [`SandboxNetPolicy::Restricted`] policy; for the other network policies
    /// the sender is never used.
    pub fn with_network_events(
        policy: SandboxPolicy,
        network_events: futures::channel::mpsc::UnboundedSender<ProxyEvent>,
    ) -> Result<Self, SandboxError> {
        Self::build(policy, Some(network_events))
    }

    fn build(
        policy: SandboxPolicy,
        network_events: Option<futures::channel::mpsc::UnboundedSender<ProxyEvent>>,
    ) -> Result<Self, SandboxError> {
        #[cfg(any(target_os = "linux", target_os = "windows"))]
        reject_unsupported_policy_features(&policy)?;

        let fs = match policy.fs {
            SandboxFsPolicy::Unrestricted {
                protected_paths,
                unreadable_paths,
                readable_paths,
            } => {
                #[cfg(not(target_os = "macos"))]
                drop((unreadable_paths, readable_paths));
                FsSetup {
                    allow_fs_write: true,
                    writable_paths: Vec::new(),
                    protected_paths,
                    #[cfg(target_os = "macos")]
                    unreadable_paths,
                    #[cfg(target_os = "macos")]
                    readable_paths,
                }
            }
            SandboxFsPolicy::Restricted {
                writable_paths,
                protected_paths,
                unreadable_paths,
                readable_paths,
            } => {
                validate_writable_paths_do_not_overlap_protected_paths(
                    &writable_paths,
                    &protected_paths,
                )?;
                validate_writable_paths_inside_unreadable_paths_are_windowed(
                    &writable_paths,
                    &unreadable_paths,
                    &readable_paths,
                )?;
                #[cfg(not(target_os = "macos"))]
                drop((unreadable_paths, readable_paths));
                FsSetup {
                    allow_fs_write: false,
                    writable_paths,
                    protected_paths,
                    #[cfg(target_os = "macos")]
                    unreadable_paths,
                    #[cfg(target_os = "macos")]
                    readable_paths,
                }
            }
        };

        let network = match policy.network {
            SandboxNetPolicy::Unrestricted => NetSetup::Unrestricted,
            SandboxNetPolicy::Blocked => NetSetup::Blocked,
            SandboxNetPolicy::Restricted {
                allowed_domains,
                allowed_tcp_ports,
            } => resolve_restricted_network(&allowed_domains, allowed_tcp_ports)?,
        };

        #[cfg(target_os = "windows")]
        drop(network_events);

        Ok(Self {
            fs,
            network,
            #[cfg(target_os = "macos")]
            allowed_unix_socket_paths: resolve_allowed_unix_sockets(policy.allowed_unix_sockets),
            proxy: None,
            #[cfg(not(target_os = "windows"))]
            network_events,
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
            reject_unsupported_policy_features(policy)?;
            let permissions = linux_bubblewrap::SandboxPermissions {
                network: linux_probe_network(&policy.network),
                allow_fs_write: matches!(policy.fs, SandboxFsPolicy::Unrestricted { .. }),
            };
            linux_bubblewrap::check_can_create_sandbox(permissions).map_err(map_linux_status)
        }
        #[cfg(target_os = "windows")]
        {
            reject_unsupported_policy_features(policy)?;
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
        let NetSetup::Restricted { allowlist, .. } = &self.network else {
            return Ok(None);
        };

        if self.proxy.is_none() {
            let upstream = upstream_proxy_from_env(env);
            let events = match &self.network_events {
                Some(sender) => sender.clone(),
                None => {
                    let (sender, receiver) = futures::channel::mpsc::unbounded();
                    spawn_proxy_event_logger(receiver);
                    sender
                }
            };
            let config = ProxyConfig {
                allowlist: allowlist.clone(),
                upstream,
                events,
            };
            #[cfg(target_os = "linux")]
            let handle = ProxyHandle::spawn_unix_temp(config);
            #[cfg(not(target_os = "linux"))]
            let handle = ProxyHandle::spawn(config);
            let handle = handle.map_err(|error| {
                SandboxError::Io(format!("failed to start network proxy: {error:#}"))
            })?;
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
        let unreadable: Vec<&Path> = self
            .fs
            .unreadable_paths
            .iter()
            .map(HostFilesystemLocation::macos_canonical_path)
            .collect();
        let readable: Vec<&Path> = self
            .fs
            .readable_paths
            .iter()
            .map(HostFilesystemLocation::macos_canonical_path)
            .collect();
        let allowed_unix_socket_paths: Vec<&Path> = self
            .allowed_unix_socket_paths
            .iter()
            .map(PathBuf::as_path)
            .collect();

        let (program, args, config) = macos_seatbelt::wrap_invocation(
            &command.program,
            &command.args,
            macos_seatbelt::SeatbeltConfigInputs {
                writable_directories: &writable,
                protected_paths: &protected,
                unreadable_paths: &unreadable,
                readable_paths: &readable,
                allowed_unix_socket_paths: &allowed_unix_socket_paths,
                allowed_tcp_ports,
                permissions,
            },
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
        let writable_paths = self
            .fs
            .writable_paths
            .iter()
            .map(|location| windows_wsl::WritablePath {
                requested: location.windows_requested_path().to_path_buf(),
                canonical: location.windows_canonical_path().to_path_buf(),
            })
            .collect();
        let protected_paths: Vec<PathBuf> = self
            .protected_paths()
            .iter()
            .map(|location| location.windows_requested_path().to_path_buf())
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
#[cfg(any(target_os = "linux", target_os = "windows"))]
pub(crate) const WSL_SANDBOX_RESOLVE_FLAG: &str = "--wsl-sandbox-resolve-path";

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

/// Canonicalizes once, at construction, for the same reason
/// [`HostFilesystemLocation`] captures its identity up front: resolving a socket
/// path again at wrap time would let a symlink planted in between redirect the
/// grant.
#[cfg(target_os = "macos")]
fn resolve_allowed_unix_sockets(socket_paths: Vec<PathBuf>) -> Vec<PathBuf> {
    let mut allowed_unix_socket_paths = Vec::new();
    for socket_path in socket_paths {
        let canonical_socket_path = canonicalize_allowing_missing_leaf(&socket_path);
        if !allowed_unix_socket_paths.contains(&socket_path) {
            allowed_unix_socket_paths.push(socket_path);
        }
        if !allowed_unix_socket_paths.contains(&canonical_socket_path) {
            allowed_unix_socket_paths.push(canonical_socket_path);
        }
    }
    allowed_unix_socket_paths
}

#[cfg(any(target_os = "linux", target_os = "windows"))]
fn reject_unsupported_policy_features(policy: &SandboxPolicy) -> Result<(), SandboxError> {
    let unsupported_features = unsupported_policy_features(policy);
    if unsupported_features.is_empty() {
        return Ok(());
    }
    let platform = if cfg!(target_os = "windows") {
        "Windows (WSL)"
    } else {
        "Linux"
    };
    Err(SandboxError::UnsupportedPolicy(format!(
        "the {platform} sandbox does not support {}",
        unsupported_features.join(", ")
    )))
}

/// The policy features that only the macOS backend enforces. The Linux and WSL
/// backends must refuse these rather than ignore them, because running a command
/// with less isolation than the policy describes is a sandbox escape.
#[cfg(any(target_os = "linux", target_os = "windows", test))]
fn unsupported_policy_features(policy: &SandboxPolicy) -> Vec<&'static str> {
    let (unreadable_paths, readable_paths) = match &policy.fs {
        SandboxFsPolicy::Unrestricted {
            unreadable_paths,
            readable_paths,
            ..
        }
        | SandboxFsPolicy::Restricted {
            unreadable_paths,
            readable_paths,
            ..
        } => (unreadable_paths, readable_paths),
    };
    let allows_tcp_ports = matches!(
        &policy.network,
        SandboxNetPolicy::Restricted { allowed_tcp_ports, .. } if !allowed_tcp_ports.is_empty()
    );

    let mut features = Vec::new();
    if !unreadable_paths.is_empty() {
        features.push("unreadable paths");
    }
    if !readable_paths.is_empty() {
        features.push("readable paths");
    }
    if allows_tcp_ports {
        features.push("raw TCP port grants");
    }
    if !policy.allowed_unix_sockets.is_empty() {
        features.push("Unix socket grants");
    }
    features
}

#[cfg(not(target_os = "windows"))]
fn resolve_restricted_network(
    allowed_domains: &[String],
    allowed_tcp_ports: Vec<u16>,
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
            "allowed TCP port 0 is not a valid destination port".to_string(),
        ));
    }
    #[cfg(not(target_os = "macos"))]
    drop(allowed_tcp_ports);
    Ok(NetSetup::Restricted {
        allowlist: Allowlist::from_patterns(patterns),
        #[cfg(target_os = "macos")]
        allowed_tcp_ports: union_in_order(Vec::new(), allowed_tcp_ports),
    })
}

#[cfg(target_os = "windows")]
fn resolve_restricted_network(
    _allowed_domains: &[String],
    _allowed_tcp_ports: Vec<u16>,
) -> Result<NetSetup, SandboxError> {
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

        let protected_git = HostFilesystemLocation::capture(&git_dir).expect("capture git dir");
        let writable_git = HostFilesystemLocation::capture(&git_dir).expect("capture git dir");
        let writable_hooks =
            HostFilesystemLocation::capture(&hooks_dir).expect("capture hooks dir");

        for writable_path in [writable_git, writable_hooks] {
            let result = Sandbox::new(SandboxPolicy {
                fs: SandboxFsPolicy::Restricted {
                    writable_paths: vec![writable_path],
                    protected_paths: vec![protected_git.clone()],
                    unreadable_paths: Vec::new(),
                    readable_paths: Vec::new(),
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
                unreadable_paths: Vec::new(),
                readable_paths: Vec::new(),
            },
            network: SandboxNetPolicy::Blocked,
            allowed_unix_sockets: Vec::new(),
        })
        .expect("captured writable fd is outside protected metadata");
    }

    #[test]
    fn fs_merge_unrestricted_dominates_else_unions_paths() {
        // Writable paths are captured as real `HostFilesystemLocation`s (keyed on
        // the inode), so the union/dedup test needs distinct real dirs.
        let dir_a = tempfile::tempdir().expect("create temp dir a");
        let dir_b = tempfile::tempdir().expect("create temp dir b");
        let dir_c = tempfile::tempdir().expect("create temp dir c");
        let dir_d = tempfile::tempdir().expect("create temp dir d");
        let dir_e = tempfile::tempdir().expect("create temp dir e");
        let window_e = dir_e.path().join("window");
        std::fs::create_dir(&window_e).expect("create window in e");
        let location = |dir: &tempfile::TempDir| {
            HostFilesystemLocation::capture(dir.path()).expect("capture temp dir")
        };
        let window_location =
            || HostFilesystemLocation::capture(&window_e).expect("capture window");

        let a = SandboxFsPolicy::Restricted {
            writable_paths: vec![location(&dir_a), location(&dir_b)],
            protected_paths: vec![location(&dir_c)],
            unreadable_paths: vec![location(&dir_e)],
            readable_paths: vec![window_location()],
        };
        let b = SandboxFsPolicy::Restricted {
            writable_paths: vec![location(&dir_b), location(&dir_c)],
            protected_paths: vec![location(&dir_c), location(&dir_d)],
            unreadable_paths: vec![location(&dir_e), location(&dir_d)],
            readable_paths: vec![window_location(), location(&dir_d)],
        };
        assert_eq!(
            a.clone().merge(b),
            SandboxFsPolicy::Restricted {
                writable_paths: vec![location(&dir_a), location(&dir_b), location(&dir_c)],
                protected_paths: vec![location(&dir_c), location(&dir_d)],
                unreadable_paths: vec![location(&dir_e), location(&dir_d)],
                readable_paths: vec![window_location(), location(&dir_d)],
            }
        );
        assert_eq!(
            a.merge(SandboxFsPolicy::Unrestricted {
                protected_paths: vec![location(&dir_a)],
                unreadable_paths: Vec::new(),
                readable_paths: Vec::new(),
            }),
            SandboxFsPolicy::Unrestricted {
                protected_paths: vec![location(&dir_c), location(&dir_a)],
                unreadable_paths: vec![location(&dir_e)],
                readable_paths: vec![window_location()],
            }
        );
        assert_eq!(
            SandboxFsPolicy::Unrestricted {
                protected_paths: vec![location(&dir_d)],
                unreadable_paths: Vec::new(),
                readable_paths: Vec::new(),
            }
            .merge(SandboxFsPolicy::Restricted {
                writable_paths: vec![location(&dir_a)],
                protected_paths: vec![location(&dir_c)],
                unreadable_paths: vec![location(&dir_e)],
                readable_paths: vec![window_location()],
            }),
            SandboxFsPolicy::Unrestricted {
                protected_paths: vec![location(&dir_d), location(&dir_c)],
                unreadable_paths: vec![location(&dir_e)],
                readable_paths: vec![window_location()],
            }
        );
    }

    #[test]
    fn net_merge_unrestricted_dominates_blocked_is_identity_else_unions() {
        let restricted = |domains: &[&str], ports: &[u16]| SandboxNetPolicy::Restricted {
            allowed_domains: domains.iter().map(|host| host.to_string()).collect(),
            allowed_tcp_ports: ports.to_vec(),
        };
        assert_eq!(
            restricted(&["a.com", "b.com"], &[22])
                .merge(restricted(&["b.com", "c.com"], &[22, 5432])),
            restricted(&["a.com", "b.com", "c.com"], &[22, 5432])
        );
        assert_eq!(
            SandboxNetPolicy::Blocked.merge(restricted(&["a.com"], &[22])),
            restricted(&["a.com"], &[22])
        );
        assert_eq!(
            restricted(&["a.com"], &[22]).merge(SandboxNetPolicy::Blocked),
            restricted(&["a.com"], &[22])
        );
        assert_eq!(
            SandboxNetPolicy::Blocked.merge(SandboxNetPolicy::Blocked),
            SandboxNetPolicy::Blocked
        );
        assert_eq!(
            restricted(&["a.com"], &[22]).merge(SandboxNetPolicy::Unrestricted),
            SandboxNetPolicy::Unrestricted
        );
    }

    #[test]
    fn policy_merge_unions_unix_sockets_in_order() {
        let policy = |sockets: &[&str]| SandboxPolicy {
            fs: SandboxFsPolicy::Unrestricted {
                protected_paths: Vec::new(),
                unreadable_paths: Vec::new(),
                readable_paths: Vec::new(),
            },
            network: SandboxNetPolicy::Blocked,
            allowed_unix_sockets: sockets.iter().map(PathBuf::from).collect(),
        };
        assert_eq!(
            policy(&["/tmp/app.sock"]).merge(policy(&["/tmp/ssh.sock", "/tmp/app.sock"])),
            policy(&["/tmp/app.sock", "/tmp/ssh.sock"])
        );
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn writable_paths_inside_unreadable_paths_need_a_readable_window() {
        let data_dir = tempfile::tempdir().expect("create data dir");
        let checkout_dir = data_dir.path().join("worktrees/checkout");
        let other_dir = data_dir.path().join("worktrees/other");
        std::fs::create_dir_all(&checkout_dir).expect("create checkout dir");
        std::fs::create_dir_all(&other_dir).expect("create other dir");
        let location = |path: &Path| HostFilesystemLocation::capture(path).expect("capture path");
        let restricted = |writable_path: &Path, readable_path: &Path| SandboxPolicy {
            fs: SandboxFsPolicy::Restricted {
                writable_paths: vec![location(writable_path)],
                protected_paths: Vec::new(),
                unreadable_paths: vec![location(data_dir.path())],
                readable_paths: vec![location(readable_path)],
            },
            network: SandboxNetPolicy::Blocked,
            allowed_unix_sockets: Vec::new(),
        };

        assert!(Sandbox::new(restricted(&checkout_dir, &checkout_dir)).is_ok());
        assert!(matches!(
            Sandbox::new(restricted(&other_dir, &checkout_dir)),
            Err(SandboxError::InvalidRequest(_))
        ));
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn restricted_network_rejects_port_zero() {
        let policy = |ports: &[u16]| SandboxPolicy {
            fs: SandboxFsPolicy::Unrestricted {
                protected_paths: Vec::new(),
                unreadable_paths: Vec::new(),
                readable_paths: Vec::new(),
            },
            network: SandboxNetPolicy::Restricted {
                allowed_domains: Vec::new(),
                allowed_tcp_ports: ports.to_vec(),
            },
            allowed_unix_sockets: Vec::new(),
        };
        assert!(Sandbox::new(policy(&[22])).is_ok());
        assert!(matches!(
            Sandbox::new(policy(&[22, 0])),
            Err(SandboxError::InvalidRequest(_))
        ));
    }

    #[test]
    fn unsupported_policy_features_lists_each_feature_a_policy_uses() {
        let dir = tempfile::tempdir().expect("create temp dir");
        let location = || HostFilesystemLocation::capture(dir.path()).expect("capture dir");
        let unrestricted_fs = || SandboxFsPolicy::Unrestricted {
            protected_paths: Vec::new(),
            unreadable_paths: Vec::new(),
            readable_paths: Vec::new(),
        };

        assert!(
            unsupported_policy_features(&SandboxPolicy {
                fs: SandboxFsPolicy::Restricted {
                    writable_paths: vec![location()],
                    protected_paths: Vec::new(),
                    unreadable_paths: Vec::new(),
                    readable_paths: Vec::new(),
                },
                network: SandboxNetPolicy::Restricted {
                    allowed_domains: vec!["example.com".to_string()],
                    allowed_tcp_ports: Vec::new(),
                },
                allowed_unix_sockets: Vec::new(),
            })
            .is_empty()
        );

        assert_eq!(
            unsupported_policy_features(&SandboxPolicy {
                fs: SandboxFsPolicy::Restricted {
                    writable_paths: Vec::new(),
                    protected_paths: Vec::new(),
                    unreadable_paths: vec![location()],
                    readable_paths: vec![location()],
                },
                network: SandboxNetPolicy::Blocked,
                allowed_unix_sockets: Vec::new(),
            }),
            ["unreadable paths", "readable paths"]
        );
        assert_eq!(
            unsupported_policy_features(&SandboxPolicy {
                fs: SandboxFsPolicy::Unrestricted {
                    protected_paths: Vec::new(),
                    unreadable_paths: vec![location()],
                    readable_paths: Vec::new(),
                },
                network: SandboxNetPolicy::Blocked,
                allowed_unix_sockets: Vec::new(),
            }),
            ["unreadable paths"]
        );
        assert_eq!(
            unsupported_policy_features(&SandboxPolicy {
                fs: unrestricted_fs(),
                network: SandboxNetPolicy::Restricted {
                    allowed_domains: Vec::new(),
                    allowed_tcp_ports: vec![22],
                },
                allowed_unix_sockets: Vec::new(),
            }),
            ["raw TCP port grants"]
        );
        assert_eq!(
            unsupported_policy_features(&SandboxPolicy {
                fs: unrestricted_fs(),
                network: SandboxNetPolicy::Blocked,
                allowed_unix_sockets: vec![PathBuf::from("/tmp/app.sock")],
            }),
            ["Unix socket grants"]
        );
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn linux_rejects_policies_using_macos_only_features() {
        let dir = tempfile::tempdir().expect("create temp dir");
        let policy = SandboxPolicy {
            fs: SandboxFsPolicy::Unrestricted {
                protected_paths: Vec::new(),
                unreadable_paths: vec![
                    HostFilesystemLocation::capture(dir.path()).expect("capture dir"),
                ],
                readable_paths: Vec::new(),
            },
            network: SandboxNetPolicy::Blocked,
            allowed_unix_sockets: Vec::new(),
        };
        assert!(matches!(
            Sandbox::can_create(&policy),
            Err(SandboxError::UnsupportedPolicy(_))
        ));
        assert!(matches!(
            Sandbox::new(policy),
            Err(SandboxError::UnsupportedPolicy(_))
        ));
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

#[cfg(all(test, target_os = "macos"))]
#[allow(
    clippy::disallowed_methods,
    reason = "tests run sandbox-exec synchronously to observe what the generated profile permits"
)]
mod macos_policy_tests {
    use super::*;
    use std::io::ErrorKind;
    use std::net::TcpListener;
    use std::os::unix::net::UnixListener;
    use std::process::{Command, Stdio};
    use std::time::{Duration, Instant};

    use futures::StreamExt as _;

    #[test]
    fn restricted_fs_confines_writes_and_hides_unreadable_paths() {
        let project_dir = tempfile::tempdir().expect("create project dir");
        // A quote and a backslash in the name exercise profile-literal escaping:
        // an unescaped path would make `sandbox-exec` reject the profile outright.
        let scratch_dir = tempfile::Builder::new()
            .prefix("q\"uo\\te-")
            .tempdir()
            .expect("create scratch dir");
        let protected_file = project_dir.path().join(".gitignore");
        std::fs::write(&protected_file, "target\n").expect("write protected file");
        let secrets_dir = project_dir.path().join("secrets");
        std::fs::create_dir(&secrets_dir).expect("create secrets dir");
        let secret_file = secrets_dir.join("id_ed25519");
        std::fs::write(&secret_file, "private\n").expect("write secret");
        let kept_dir = secrets_dir.join("kept");
        std::fs::create_dir(&kept_dir).expect("create kept dir");
        let kept_file = kept_dir.join("notes");
        std::fs::write(&kept_file, "kept\n").expect("write kept file");
        let readme_file = project_dir.path().join("README");
        std::fs::write(&readme_file, "public\n").expect("write readme");

        let mut sandbox = Sandbox::new(policy(
            restricted_fs(
                &[project_dir.path(), scratch_dir.path(), &kept_dir],
                &[&protected_file],
                &[&secrets_dir],
                &[&kept_dir],
            ),
            SandboxNetPolicy::Blocked,
        ))
        .expect("create sandbox");

        let scratch_written = scratch_dir.path().join("written");
        let output = run_shell(
            &mut sandbox,
            &format!("echo hello > '{}'", scratch_written.display()),
        );
        assert!(output.status.success(), "{}", failure(&output));

        let output = run_shell(&mut sandbox, &format!("cat '{}'", protected_file.display()));
        assert_eq!(String::from_utf8_lossy(&output.stdout), "target\n");
        let output = run_shell(
            &mut sandbox,
            &format!("echo changed > '{}'", protected_file.display()),
        );
        assert!(!output.status.success(), "write to protected file");
        assert_eq!(
            std::fs::read_to_string(&protected_file).expect("read protected file"),
            "target\n"
        );

        let output = run_shell(&mut sandbox, &format!("cat '{}'", readme_file.display()));
        assert_eq!(String::from_utf8_lossy(&output.stdout), "public\n");
        let output = run_shell(&mut sandbox, &format!("cat '{}'", secret_file.display()));
        assert!(!output.status.success(), "read under unreadable path");
        assert!(output.stdout.is_empty(), "no secret bytes may leak");
        let output = run_shell(&mut sandbox, &format!("ls '{}'", secrets_dir.display()));
        assert!(!output.status.success(), "list unreadable directory");

        // A kept path inside the unreadable directory is readable, and writable
        // because it is also a writable path; its siblings stay hidden.
        let output = run_shell(&mut sandbox, &format!("cat '{}'", kept_file.display()));
        assert_eq!(String::from_utf8_lossy(&output.stdout), "kept\n");
        let kept_written = kept_dir.join("written");
        let output = run_shell(
            &mut sandbox,
            &format!("echo hello > '{}'", kept_written.display()),
        );
        assert!(output.status.success(), "{}", failure(&output));
        assert_eq!(
            std::fs::read_to_string(&kept_written).expect("read kept written file"),
            "hello\n"
        );
        let hidden_written = secrets_dir.join("written");
        let output = run_shell(
            &mut sandbox,
            &format!("echo hello > '{}'", hidden_written.display()),
        );
        assert!(!output.status.success(), "write under unreadable path");
        assert!(!hidden_written.exists());
    }

    #[test]
    fn unrestricted_fs_writes_stop_at_unreadable_paths_but_reach_windows() {
        let scratch_dir = tempfile::Builder::new()
            .prefix("zed-sandbox-write-test-")
            .tempdir_in("/tmp")
            .expect("create scratch dir in /tmp");
        let repo_dir = tempfile::tempdir().expect("create repo dir");
        let protected_file = repo_dir.path().join(".gitignore");
        std::fs::write(&protected_file, "target\n").expect("write protected file");
        let secrets_dir = repo_dir.path().join("secrets");
        let kept_dir = secrets_dir.join("kept");
        std::fs::create_dir_all(&kept_dir).expect("create kept dir");

        let mut sandbox = Sandbox::new(policy(
            SandboxFsPolicy::Unrestricted {
                protected_paths: vec![location(&protected_file)],
                unreadable_paths: vec![location(&secrets_dir)],
                readable_paths: vec![location(&kept_dir)],
            },
            SandboxNetPolicy::Blocked,
        ))
        .expect("create sandbox");

        let written = scratch_dir.path().join("written");
        let output = run_shell(
            &mut sandbox,
            &format!("echo hello > '{}'", written.display()),
        );
        assert!(output.status.success(), "{}", failure(&output));

        let output = run_shell(
            &mut sandbox,
            &format!("echo changed > '{}'", protected_file.display()),
        );
        assert!(!output.status.success(), "write to protected file");

        let hidden_written = secrets_dir.join("written");
        let output = run_shell(
            &mut sandbox,
            &format!("echo hello > '{}'", hidden_written.display()),
        );
        assert!(!output.status.success(), "write under unreadable path");
        assert!(!hidden_written.exists());

        let kept_written = kept_dir.join("written");
        let output = run_shell(
            &mut sandbox,
            &format!("echo hello > '{}'", kept_written.display()),
        );
        assert!(output.status.success(), "{}", failure(&output));
        assert_eq!(
            std::fs::read_to_string(&kept_written).expect("read kept written file"),
            "hello\n"
        );
    }

    #[test]
    fn protected_paths_win_over_a_readable_window() {
        let project_dir = tempfile::tempdir().expect("create project dir");
        let secrets_dir = project_dir.path().join("secrets");
        let kept_dir = secrets_dir.join("kept");
        std::fs::create_dir_all(&kept_dir).expect("create kept dir");
        let protected_file = kept_dir.join("locked");
        std::fs::write(&protected_file, "locked\n").expect("write protected file");

        let mut sandbox = Sandbox::new(policy(
            restricted_fs(
                &[&kept_dir],
                &[&protected_file],
                &[&secrets_dir],
                &[&kept_dir],
            ),
            SandboxNetPolicy::Blocked,
        ))
        .expect("create sandbox");

        let output = run_shell(&mut sandbox, &format!("cat '{}'", protected_file.display()));
        assert_eq!(String::from_utf8_lossy(&output.stdout), "locked\n");
        let output = run_shell(
            &mut sandbox,
            &format!("echo changed > '{}'", protected_file.display()),
        );
        assert!(!output.status.success(), "write to protected file");
        assert_eq!(
            std::fs::read_to_string(&protected_file).expect("read protected file"),
            "locked\n"
        );
    }

    #[test]
    fn blocked_network_allows_only_allowlisted_unix_sockets() {
        // Bind under `/tmp` rather than the default temp dir: macOS temp paths
        // (`/var/folders/...`) overflow the `sun_path` limit for Unix sockets.
        // `/tmp` is also a symlink, so the requested path differs from the one
        // Seatbelt matches against.
        let temp_dir = tempfile::Builder::new()
            .prefix("zed-sock-")
            .tempdir_in("/tmp")
            .expect("create temp dir");
        let allowed_socket = temp_dir.path().join("allowed.sock");
        let other_socket = temp_dir.path().join("other.sock");
        let allowed_listener = UnixListener::bind(&allowed_socket).expect("bind allowed socket");
        let other_listener = UnixListener::bind(&other_socket).expect("bind other socket");
        other_listener
            .set_nonblocking(true)
            .expect("other listener should switch to non-blocking");

        let mut sandbox = Sandbox::new(SandboxPolicy {
            fs: restricted_fs(&[], &[], &[], &[]),
            network: SandboxNetPolicy::Blocked,
            allowed_unix_sockets: vec![allowed_socket.clone()],
        })
        .expect("create sandbox");

        let accepted = accept_one_within(allowed_listener, Duration::from_secs(10));
        let output = run_netcat(&mut sandbox, &["-U", &allowed_socket.display().to_string()]);
        assert!(
            accepted.join().expect("accept thread"),
            "allowlisted socket was not reached: {}",
            failure(&output)
        );

        let output = run_netcat(&mut sandbox, &["-U", &other_socket.display().to_string()]);
        assert!(
            !output.status.success(),
            "connected to a socket outside the allowlist"
        );
        assert!(matches!(
            other_listener.accept(),
            Err(error) if error.kind() == ErrorKind::WouldBlock
        ));
    }

    #[test]
    fn restricted_network_allows_only_the_proxy_and_granted_ports() {
        // The kernel completes `nc -z`'s connect from the listen backlog, so the
        // listeners never need to accept.
        let granted_listener = TcpListener::bind("127.0.0.1:0").expect("bind granted listener");
        let other_listener = TcpListener::bind("127.0.0.1:0").expect("bind other listener");
        let granted_port = granted_listener
            .local_addr()
            .expect("granted listener address")
            .port();
        let other_port = other_listener
            .local_addr()
            .expect("other listener address")
            .port();

        let (events_sender, mut events) = futures::channel::mpsc::unbounded();
        let mut sandbox = Sandbox::with_network_events(
            policy(
                restricted_fs(&[], &[], &[], &[]),
                SandboxNetPolicy::Restricted {
                    allowed_domains: vec!["example.com".to_string()],
                    allowed_tcp_ports: vec![granted_port],
                },
            ),
            events_sender,
        )
        .expect("create sandbox");

        let wrapped = wrap(&mut sandbox, &CommandAndArgs::default());
        let proxy_port = match futures::executor::block_on(events.next()) {
            Some(ProxyEvent::Ready { port }) => port,
            other => panic!("expected the proxy's Ready event, got {other:?}"),
        };

        // A raw TCP grant means the command resolves hostnames itself, which on
        // macOS goes through the mDNSResponder socket; that can't be observed
        // offline, so check the rule.
        assert!(profile_rules(&wrapped).contains("(literal \"/private/var/run/mDNSResponder\")"));

        assert!(tcp_connects(&mut sandbox, proxy_port), "proxy port");
        assert!(tcp_connects(&mut sandbox, granted_port), "granted port");
        assert!(
            !tcp_connects(&mut sandbox, other_port),
            "reached a port that was not granted"
        );

        sandbox.drop_on_current_thread();
    }

    #[test]
    fn restricted_network_without_port_grants_has_no_dns_socket_rule() {
        let mut sandbox = Sandbox::new(policy(
            restricted_fs(&[], &[], &[], &[]),
            SandboxNetPolicy::Restricted {
                allowed_domains: Vec::new(),
                allowed_tcp_ports: Vec::new(),
            },
        ))
        .expect("create sandbox");

        let wrapped = wrap(&mut sandbox, &CommandAndArgs::default());
        let rules = profile_rules(&wrapped);
        assert!(wrapped.env.contains_key("HTTPS_PROXY"));
        assert!(rules.contains("(allow network-bind (local ip \"localhost:*\"))"));
        assert!(!rules.contains("mDNSResponder"));

        sandbox.drop_on_current_thread();
    }

    fn location(path: &Path) -> HostFilesystemLocation {
        HostFilesystemLocation::capture(path).expect("capture location")
    }

    fn restricted_fs(
        writable_paths: &[&Path],
        protected_paths: &[&Path],
        unreadable_paths: &[&Path],
        readable_paths: &[&Path],
    ) -> SandboxFsPolicy {
        SandboxFsPolicy::Restricted {
            writable_paths: writable_paths.iter().map(|path| location(path)).collect(),
            protected_paths: protected_paths.iter().map(|path| location(path)).collect(),
            unreadable_paths: unreadable_paths.iter().map(|path| location(path)).collect(),
            readable_paths: readable_paths.iter().map(|path| location(path)).collect(),
        }
    }

    fn policy(fs: SandboxFsPolicy, network: SandboxNetPolicy) -> SandboxPolicy {
        SandboxPolicy {
            fs,
            network,
            allowed_unix_sockets: Vec::new(),
        }
    }

    fn wrap(sandbox: &mut Sandbox, command: &CommandAndArgs) -> WrappedCommand {
        futures::executor::block_on(sandbox.wrap(command)).expect("wrap command")
    }

    /// The rules of the profile a wrapped command runs under, with SBPL comment
    /// lines stripped so assertions can't match on documentation.
    fn profile_rules(wrapped: &WrappedCommand) -> String {
        let profile_path = wrapped.args.get(1).expect("sandbox-exec profile argument");
        std::fs::read_to_string(profile_path)
            .expect("read generated profile")
            .lines()
            .filter(|line| !line.trim_start().starts_with(';'))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn run_sandboxed(sandbox: &mut Sandbox, program: &str, args: &[&str]) -> Output {
        let wrapped = wrap(
            sandbox,
            &CommandAndArgs {
                program: program.to_string(),
                args: args.iter().map(|argument| argument.to_string()).collect(),
                env: HashMap::new(),
                cwd: None,
            },
        );
        Command::new(&wrapped.program)
            .args(&wrapped.args)
            .envs(&wrapped.env)
            .stdin(Stdio::null())
            .output()
            .expect("failed to execute sandbox-exec")
    }

    fn run_shell(sandbox: &mut Sandbox, script: &str) -> Output {
        run_sandboxed(sandbox, "/bin/sh", &["-c", script])
    }

    fn run_netcat(sandbox: &mut Sandbox, args: &[&str]) -> Output {
        let mut all_args = vec!["-w", "5"];
        all_args.extend_from_slice(args);
        run_sandboxed(sandbox, "/usr/bin/nc", &all_args)
    }

    fn tcp_connects(sandbox: &mut Sandbox, port: u16) -> bool {
        run_netcat(sandbox, &["-z", "127.0.0.1", &port.to_string()])
            .status
            .success()
    }

    /// Accept (and immediately drop) one connection on a background thread,
    /// giving up after `timeout` so a wrongly denied connection fails the test
    /// instead of hanging it.
    fn accept_one_within(
        listener: UnixListener,
        timeout: Duration,
    ) -> std::thread::JoinHandle<bool> {
        listener
            .set_nonblocking(true)
            .expect("listener should switch to non-blocking");
        std::thread::spawn(move || {
            let deadline = Instant::now() + timeout;
            loop {
                match listener.accept() {
                    Ok(_) => return true,
                    Err(error) if error.kind() == ErrorKind::WouldBlock => {
                        if Instant::now() >= deadline {
                            return false;
                        }
                        std::thread::sleep(Duration::from_millis(20));
                    }
                    Err(_) => return false,
                }
            }
        })
    }

    fn failure(output: &Output) -> String {
        format!(
            "exit={:?} stdout={} stderr={}",
            output.status.code(),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )
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
        #[cfg(not(any(target_os = "linux", target_os = "macos")))]
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
    Ok(CanonicalPathBuf::resolve(path)?.into_path())
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
