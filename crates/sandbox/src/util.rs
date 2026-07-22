//! Path-identity primitives shared by the sandbox enforcement layers.
//!
//! [`CanonicalPathBuf`] is the low-level primitive: a symlink-free canonical
//! path that, on Linux, additionally pins the exact inode it names via a live
//! `O_PATH` handle. [`HostFilesystemLocation`] wraps it into the opaque
//! host-location handle the policy layer traffics in, pairing the trusted
//! canonical identity with the untrusted, as-requested path (for display and
//! provenance).

mod canonical_path;
mod host_filesystem_location;

pub(crate) use canonical_path::CanonicalPathBuf;
#[cfg(target_os = "linux")]
pub(crate) use canonical_path::linux_fd_identity;
pub use host_filesystem_location::{
    HostFilesystemLocation, HostFilesystemLocationDisplay, normalize_host_filesystem_locations,
};

use std::path::{Path, PathBuf};

/// Resolve `path` to its canonical, symlink-free form, for recording as the
/// `resolved` half of a persisted write grant (and for showing the user the true
/// target at approval time).
///
/// This **follows** symlinks: it discovers the real target so it can be shown
/// and persisted. Protection against a *later* swap comes from rebuilding the
/// grant with [`HostFilesystemLocation::reopen`], which verifies this value.
/// Mirrors what [`HostFilesystemLocation::capture`] computes internally.
#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn resolve_canonical(path: impl AsRef<Path>) -> std::io::Result<PathBuf> {
    Ok(CanonicalPathBuf::resolve(path)?.into_path())
}

/// Non-Linux/macOS platforms capture no canonical identity of their own (the
/// real resolution happens WSL-side), so the requested path is returned as-is.
#[cfg(not(any(target_os = "linux", target_os = "macos")))]
pub fn resolve_canonical(path: impl AsRef<Path>) -> std::io::Result<PathBuf> {
    Ok(path.as_ref().to_path_buf())
}

/// Lexically normalize an absolute path (collapsing `.` and `..` and redundant
/// separators) without touching the filesystem. Returns `None` for a relative
/// path, which can't be compared to an absolute canonical path without a base.
#[cfg(any(target_os = "linux", target_os = "macos"))]
pub(crate) fn lexically_normalized_absolute(path: &Path) -> Option<PathBuf> {
    use std::path::Component;
    if !path.is_absolute() {
        return None;
    }
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                // Never pop past the root: `normalized.pop()` is a no-op at root.
                normalized.pop();
            }
            other => normalized.push(other),
        }
    }
    Some(normalized)
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
