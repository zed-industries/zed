//! The [`CanonicalPathBuf`] primitive: a symlink-free canonical path that, on
//! Linux, is backed by a **live `O_PATH` handle pinning the exact inode** it
//! names — not merely a path string.

use std::{
    io,
    path::{Path, PathBuf},
};

#[cfg(target_os = "linux")]
use std::os::fd::{AsFd as _, AsRawFd as _, BorrowedFd, OwnedFd};

/// A canonical, symlink-free filesystem path.
///
/// On **Linux** this is not merely a path string: it also owns a live `O_PATH`
/// file descriptor pinning the exact inode the path named at construction time.
/// The descriptor — not the text — is the security-relevant identity, so
/// enforcement can prove the object living at the path *now* is still the one
/// that was captured, closing the classic time-of-check-to-time-of-use hole
/// where a verified path is swapped for a symlink before it is actually used.
/// On other platforms it is a plain canonical path (see the constructors for the
/// per-platform rationale).
///
/// Invariant: [`path`](Self::path) is the fd's symlink-free realpath; on Linux
/// this is *proven* at construction (`resolve` reads it back from
/// `/proc/self/fd`, `from_canonical` re-verifies a claimed value against the
/// reopened fd).
#[derive(Clone)]
pub(crate) struct CanonicalPathBuf {
    path: PathBuf,
    /// An `O_PATH` descriptor pinning the inode of the canonical target. Wrapped
    /// in an `Arc` only so the surrounding policy types can stay `Clone`;
    /// cloning shares the same underlying descriptor.
    #[cfg(target_os = "linux")]
    fd: std::sync::Arc<OwnedFd>,
}

impl CanonicalPathBuf {
    /// Resolve `path` to its canonical target, **following** symlinks to
    /// discover the real object it names.
    ///
    /// Following symlinks is intentional: the goal is to find the true target so
    /// it can be shown, persisted, and pinned. Protection against a *later* swap
    /// comes from persisting the resolved path and rebuilding with
    /// [`Self::from_canonical`], which verifies it.
    pub(crate) fn resolve(path: impl AsRef<Path>) -> io::Result<Self> {
        let path = path.as_ref();

        #[cfg(target_os = "macos")]
        {
            // `canonicalize_allowing_missing_leaf` resolves through the existing
            // parent so a not-yet-created leaf still yields the real path
            // Seatbelt will match against.
            Ok(Self {
                path: super::canonicalize_allowing_missing_leaf(path),
            })
        }
        #[cfg(target_os = "linux")]
        {
            use std::os::unix::fs::OpenOptionsExt as _;
            // `O_PATH` opens a handle that refers to the inode without granting
            // read/write on its contents, which is exactly what a bind source
            // needs. Symlinks are followed (no `O_NOFOLLOW`) so the fd pins the
            // real target; its canonical path is then read back from
            // `/proc/self/fd`.
            let file = std::fs::OpenOptions::new()
                .read(true)
                .custom_flags(libc::O_PATH | libc::O_CLOEXEC)
                .open(path)?;
            let fd = OwnedFd::from(file);
            let path = std::fs::read_link(format!("/proc/self/fd/{}", fd.as_raw_fd()))?;
            Ok(Self {
                path,
                fd: std::sync::Arc::new(fd),
            })
        }
        #[cfg(not(any(target_os = "macos", target_os = "linux")))]
        {
            Ok(Self {
                path: path.to_path_buf(),
            })
        }
    }

    /// Verify a *claimed* canonical `path`, proving the object now living there
    /// is the one that was approved.
    ///
    /// This is the load-bearing reconstruction for user-approved write grants,
    /// which must survive process restarts and therefore can't keep the
    /// approval-time fd alive.
    ///
    /// On **Linux** this opens an `O_PATH` fd with `O_NOFOLLOW`, rejects a
    /// symlink leaf (`S_IFLNK` check, [`io::ErrorKind::PermissionDenied`]), and
    /// requires the fd's real path to still equal `path` (also
    /// `PermissionDenied` otherwise) — so any component swapped for a symlink
    /// after approval fails closed. On **macOS/other** the claimed path is
    /// trusted verbatim: Seatbelt re-resolves the access path at syscall time
    /// (so a later swap is denied, not redirected), and WSL/other resolve
    /// elsewhere.
    pub(crate) fn from_canonical(path: PathBuf) -> io::Result<Self> {
        #[cfg(target_os = "macos")]
        {
            Ok(Self { path })
        }
        #[cfg(target_os = "linux")]
        {
            use std::os::unix::fs::OpenOptionsExt as _;
            // `O_NOFOLLOW` makes a symlink *leaf* open the symlink itself
            // (harmless with `O_PATH`) rather than its target, so we can detect
            // and reject it below; intermediate components are still traversed
            // and caught by the canonical-path comparison.
            let file = std::fs::OpenOptions::new()
                .read(true)
                .custom_flags(libc::O_PATH | libc::O_CLOEXEC | libc::O_NOFOLLOW)
                .open(&path)?;
            let fd = OwnedFd::from(file);

            // Reject a symlink leaf outright: a grant must name a real directory,
            // and `readlink` of an `O_PATH|O_NOFOLLOW` fd on a symlink returns
            // the symlink's *own* path (equal to `path`), so the comparison
            // below wouldn't catch it.
            let stat = nix::sys::stat::fstat(fd.as_raw_fd()).map_err(io::Error::from)?;
            if stat.st_mode & libc::S_IFMT == libc::S_IFLNK {
                return Err(io::Error::new(
                    io::ErrorKind::PermissionDenied,
                    format!(
                        "sandbox write grant {} is a symlink, not a directory",
                        path.display()
                    ),
                ));
            }

            // Load-bearing: the pinned inode's real path must still be exactly
            // the approved canonical path. If any component became a symlink
            // after approval, the fd resolves elsewhere and this diverges.
            let current = std::fs::read_link(format!("/proc/self/fd/{}", fd.as_raw_fd()))?;
            if current != path {
                return Err(io::Error::new(
                    io::ErrorKind::PermissionDenied,
                    format!(
                        "sandbox write grant {} was redirected to {}",
                        path.display(),
                        current.display()
                    ),
                ));
            }

            Ok(Self {
                path,
                fd: std::sync::Arc::new(fd),
            })
        }
        #[cfg(not(any(target_os = "macos", target_os = "linux")))]
        {
            Ok(Self { path })
        }
    }

    /// The canonical, symlink-free path.
    pub(crate) fn path(&self) -> &Path {
        &self.path
    }

    /// Consume this value, returning the canonical path.
    pub(crate) fn into_path(self) -> PathBuf {
        self.path
    }

    /// Linux: a borrowed handle to the pinned inode, for `fstat`-based identity
    /// checks.
    #[cfg(target_os = "linux")]
    pub(crate) fn fd(&self) -> BorrowedFd<'_> {
        self.fd.as_fd()
    }

    /// Linux: an independent `O_PATH` descriptor to the same pinned inode,
    /// duplicated (with `O_CLOEXEC`) so a validation server can own and send it
    /// over `SCM_RIGHTS` without affecting this value's descriptor.
    #[cfg(target_os = "linux")]
    pub(crate) fn dup_fd(&self) -> io::Result<OwnedFd> {
        self.fd.as_fd().try_clone_to_owned()
    }
}

impl PartialEq for CanonicalPathBuf {
    /// Two values are equal when they refer to the **same filesystem object**:
    /// the inode behind the `O_PATH` fd on Linux, the canonical path on
    /// macOS/other — never merely equal path text where an fd is available. This
    /// is what lets policy bookkeeping dedupe "the same location named two
    /// different ways" and refuse to treat "two different objects that happen to
    /// share a path string" as one.
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
        #[cfg(not(target_os = "linux"))]
        {
            // Canonicalization is a bijection on real paths, so equal canonical
            // paths mean the same directory/file.
            self.path == other.path
        }
    }
}

impl Eq for CanonicalPathBuf {}

/// The `(device, inode)` pair behind an `O_PATH` descriptor, used to decide
/// whether two [`CanonicalPathBuf`]s (or their parents) refer to the same
/// filesystem object.
#[cfg(target_os = "linux")]
pub(crate) fn linux_fd_identity(fd: std::os::fd::RawFd) -> Option<(u64, u64)> {
    let stat = nix::sys::stat::fstat(fd).ok()?;
    Some((stat.st_dev as u64, stat.st_ino as u64))
}
