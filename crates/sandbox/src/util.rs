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

/// Canonicalize `path`, resolving symlinks, even when its trailing components
/// don't exist yet.
///
/// `std::fs::canonicalize` fails if any component is missing, which would leave
/// a not-yet-created path in a non-canonical form. Canonicalizing the nearest
/// existing ancestor and re-appending the missing components keeps the child
/// consistent with that ancestor (`/tmp` -> `/private/tmp` on macOS), so a rule
/// for a path that is created later still names the real path the command ends
/// up touching; Seatbelt matches rules against real paths, so a rule for the
/// unresolved form would never apply. If no ancestor can be canonicalized, or a
/// missing component is `.` or `..` (which only a real directory could
/// resolve), the path is returned unchanged.
//
// Only the macOS Seatbelt layer uses this (Linux skips not-yet-existing
// protected paths rather than emitting a rule for them), so it's gated to macOS
// to avoid a dead-code warning elsewhere.
#[cfg(target_os = "macos")]
pub(crate) fn canonicalize_allowing_missing_leaf(path: &std::path::Path) -> std::path::PathBuf {
    use std::path::Component;

    let mut missing_components = Vec::new();
    let mut ancestor = path;
    loop {
        if let Ok(canonical_ancestor) = ancestor.canonicalize() {
            return missing_components
                .into_iter()
                .rev()
                .fold(canonical_ancestor, |resolved, component| {
                    resolved.join(component)
                });
        }
        match (ancestor.parent(), ancestor.components().next_back()) {
            (Some(parent), Some(Component::Normal(component))) => {
                missing_components.push(component);
                ancestor = parent;
            }
            _ => return path.to_path_buf(),
        }
    }
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

        // Every missing component resolves through the nearest existing
        // ancestor.
        let deeper = missing.join("nested/.git");
        assert_eq!(
            canonicalize_allowing_missing_leaf(&deeper),
            canonical_dir.join("not-created-yet/nested/.git"),
        );

        // A symlinked ancestor resolves to its target, including when components
        // beneath it are missing.
        let target = dir.path().join("target");
        std::fs::create_dir(&target).unwrap();
        std::os::unix::fs::symlink(&target, dir.path().join("link")).unwrap();
        assert_eq!(
            canonicalize_allowing_missing_leaf(&dir.path().join("link/build/out")),
            canonical_dir.join("target/build/out"),
        );

        // A `..` through a directory that doesn't exist can't be resolved, so the
        // path is returned unchanged rather than guessed.
        let unresolvable = dir.path().join("not-created-yet/../other");
        assert_eq!(
            canonicalize_allowing_missing_leaf(&unresolvable),
            unresolvable
        );
    }
}
