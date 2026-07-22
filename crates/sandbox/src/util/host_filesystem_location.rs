//! [`HostFilesystemLocation`]: an opaque handle to a host-filesystem location
//! the sandbox may grant access to or protect, backed by a [`CanonicalPathBuf`].

use std::fmt;
use std::path::{Path, PathBuf};

#[cfg(any(target_os = "linux", target_os = "macos"))]
use super::lexically_normalized_absolute;
use super::CanonicalPathBuf;

#[cfg(target_os = "linux")]
use std::os::fd::{BorrowedFd, OwnedFd};

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
/// A grant carries two paths: the **raw** path exactly as requested (untrusted,
/// display/provenance only) and the **canonical**, symlink-free path that is the
/// *actual* grant (a [`CanonicalPathBuf`]). The canonical path is resolved once
/// when the grant is established (and persisted across process restarts for
/// grants that must outlive an fd), and enforcement proves the object now living
/// at that path is still the one that was approved.
///
/// What the [`CanonicalPathBuf`] captures is platform-specific:
/// - **macOS**: the canonical path, used verbatim as the Seatbelt rule literal.
///   Seatbelt matches the *resolved* access path against this literal, so a
///   post-approval swap of a path component fails closed (denied) rather than
///   redirecting the grant — no re-resolution or fd is needed.
/// - **Linux**: the canonical path *plus* an `O_PATH` file descriptor pinned to
///   its inode. [`HostFilesystemLocation::reopen`] opens the fd on the persisted
///   canonical path and requires `readlink("/proc/self/fd/N") == canonical`,
///   failing closed if any component became a symlink after approval (the
///   pre-mount, host-side half of the check). bwrap is launched by a PTY that
///   can't inherit extra fds, so we can't use bwrap's own `--bind-fd`; instead
///   the bind uses an ordinary `--bind <canonical>` and an in-sandbox validator
///   compares `fstat` of this descriptor against `lstat` of the mounted path
///   after the mounts, catching a swap between the host check and the mount (the
///   post-mount half; see `linux_bubblewrap::validate_binds` and `README.md`).
/// - **Windows**: only the raw path — a Windows process holds no Linux fds, so
///   the real capture-at-validation happens inside WSL (in the
///   `--wsl-sandbox-helper`).
///
/// The type is deliberately **opaque**: it does not `Deref`. Its paths are
/// readable only through [`HostFilesystemLocation::display`] (for showing the
/// user both what was requested and the true target) and the `pub(crate)`
/// enforcement accessors. Nothing hands back a value that can be re-fed into a
/// constructor by string except the verifying [`HostFilesystemLocation::reopen`]
/// path. Equality reflects the actual filesystem object (same inode on Linux,
/// same canonical path on macOS), not the textual raw path.
#[derive(Clone)]
pub struct HostFilesystemLocation {
    /// The captured, symlink-free identity of the location — the *actual* grant.
    canonical: CanonicalPathBuf,
    /// The path exactly as requested. Display/provenance only — never consulted
    /// by enforcement. Treat as untrusted, attacker-influenced text.
    untrusted_raw_path: PathBuf,
}

/// A borrowed, display-only view of a [`HostFilesystemLocation`], exposing both
/// the raw request and the canonical target so a UI can render an informed
/// "requested → granted" disclosure. The fields are `impl Display`, so they drop
/// straight into a format string; neither is a trusted identity a caller may
/// feed back into a sandbox API.
pub struct HostFilesystemLocationDisplay<'a> {
    /// The path exactly as requested (untrusted provenance; may be relative).
    pub untrusted_raw_path: std::path::Display<'a>,
    /// The canonical target — the path that is *actually* granted. Emphasize
    /// this when rendering.
    pub canonical_path: std::path::Display<'a>,
    /// `true` when the raw request resolved through a symlink to a different
    /// target (i.e. `untrusted_raw_path` and `canonical_path` name different
    /// objects). The approval UI should highlight the redirect in this case; a
    /// mere relative-vs-absolute or `.`/`..` difference does not count.
    pub is_redirected: bool,
}

impl HostFilesystemLocation {
    /// Resolve `raw` to its canonical target and capture it, **following**
    /// symlinks.
    ///
    /// Use this for locations that are *not* re-established from persisted state:
    /// the project's own worktree roots and protected paths (whose parents a
    /// sandboxed command can't tamper with), and to resolve a user-requested
    /// path at approval time so the real target can be shown and persisted.
    ///
    /// Following symlinks is intentional here — the goal is to discover the true
    /// target. Protection against a *later* swap comes from persisting the
    /// resolved canonical path and rebuilding the grant with [`Self::reopen`],
    /// which verifies it.
    pub fn capture(raw: impl AsRef<Path>) -> std::io::Result<Self> {
        let raw = raw.as_ref();
        Ok(Self {
            canonical: CanonicalPathBuf::resolve(raw)?,
            untrusted_raw_path: raw.to_path_buf(),
        })
    }

    /// Rebuild a grant from a persisted `(raw, canonical)` pair, proving the
    /// object now living at `canonical` is the one that was approved.
    ///
    /// This is the load-bearing reconstruction for user-approved write grants,
    /// which must survive process restarts and therefore can't keep the
    /// approval-time fd alive. `raw` is carried for display only.
    ///
    /// On Linux this opens an `O_PATH` fd on `canonical`, rejects a symlink leaf
    /// (`O_NOFOLLOW` + an `S_IFLNK` check), and requires the fd's real path to
    /// still equal `canonical` — so any component swapped for a symlink after
    /// approval fails closed. On macOS the persisted canonical is trusted
    /// verbatim (Seatbelt matches the resolved access path, so a later swap is
    /// denied at syscall time, not redirected).
    pub fn reopen(raw: impl AsRef<Path>, canonical: impl AsRef<Path>) -> std::io::Result<Self> {
        let raw = raw.as_ref();
        Ok(Self {
            canonical: CanonicalPathBuf::from_canonical(canonical.as_ref().to_path_buf())?,
            untrusted_raw_path: raw.to_path_buf(),
        })
    }

    /// The requested path, for **display only** (e.g. in error messages).
    ///
    /// This intentionally returns the untrusted, as-requested path — never the
    /// captured trusted identity. Do not feed the result back into any sandbox
    /// API as if it identified this location. Prefer [`Self::display`] for UI.
    pub fn untrusted_path_display(&self) -> std::path::Display<'_> {
        self.untrusted_raw_path.display()
    }

    /// A borrowed view exposing both the raw request and the canonical target,
    /// plus whether the request was redirected through a symlink, for an
    /// informed "requested → granted" disclosure in the approval UI.
    pub fn display(&self) -> HostFilesystemLocationDisplay<'_> {
        HostFilesystemLocationDisplay {
            untrusted_raw_path: self.untrusted_raw_path.display(),
            canonical_path: self.canonical_or_raw_path().display(),
            is_redirected: self.is_redirected(),
        }
    }

    /// The captured canonical path (Linux/macOS), falling back to the raw path
    /// on platforms that capture no canonical identity of their own (Windows).
    ///
    /// Used both for display and as the subtree key in
    /// [`normalize_host_filesystem_locations`]. On Linux/macOS this is the
    /// symlink-free path proven at capture time, so `Path::starts_with` on it
    /// models real filesystem containment.
    fn canonical_or_raw_path(&self) -> &Path {
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        {
            self.canonical.path()
        }
        #[cfg(not(any(target_os = "linux", target_os = "macos")))]
        {
            &self.untrusted_raw_path
        }
    }

    /// Whether the raw request resolved through a symlink to a different target.
    /// Compares the *lexically*-normalized absolute raw path against the
    /// canonical path, so a purely relative or `.`/`..`-laden request that
    /// resolves to the same object does not count as a redirect.
    fn is_redirected(&self) -> bool {
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        {
            match lexically_normalized_absolute(&self.untrusted_raw_path) {
                Some(normalized) => normalized != self.canonical.path(),
                // A relative request can't be compared without a base; report no
                // redirect rather than a spurious one.
                None => false,
            }
        }
        #[cfg(not(any(target_os = "linux", target_os = "macos")))]
        {
            false
        }
    }

    /// The requested path, for enforcement branches on platforms with no
    /// captured identity of their own (the `not(linux/macos)` overlap check).
    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    pub(crate) fn untrusted_raw_path(&self) -> &Path {
        &self.untrusted_raw_path
    }

    /// macOS: the canonical path, used verbatim as the Seatbelt rule literal.
    /// Trusted — never re-resolved.
    #[cfg(target_os = "macos")]
    pub(crate) fn macos_canonical_path(&self) -> &Path {
        self.canonical.path()
    }

    /// Linux: the verified canonical path, used as the bind source/destination.
    #[cfg(target_os = "linux")]
    pub(crate) fn linux_canonical_path(&self) -> &Path {
        self.canonical.path()
    }

    /// Linux: a borrowed handle to the pinned inode, for `fstat`-based identity
    /// checks.
    #[cfg(target_os = "linux")]
    pub(crate) fn linux_fd(&self) -> BorrowedFd<'_> {
        self.canonical.fd()
    }

    /// Linux: an independent `O_PATH` descriptor to the same pinned inode,
    /// duplicated (with `O_CLOEXEC`) so the validation server can own and send it
    /// over `SCM_RIGHTS` without affecting this location's descriptor.
    #[cfg(target_os = "linux")]
    pub(crate) fn linux_dup_fd(&self) -> std::io::Result<OwnedFd> {
        self.canonical.dup_fd()
    }

    /// Windows: the requested path, to be mapped into WSL and handed to the
    /// in-WSL helper. Windows captures no identity itself (it holds no Linux
    /// fds); the real capture-at-validation happens WSL-side in the helper, so
    /// here the requested path *is* the location. (WSL currently binds the
    /// requested path; moving this to the canonical is a separate future change.)
    #[cfg(target_os = "windows")]
    pub(crate) fn windows_path(&self) -> &Path {
        &self.untrusted_raw_path
    }
}

/// Reduce a set of writable/protected [`HostFilesystemLocation`]s to a minimal
/// cover, silently dropping any location nested under another.
///
/// Containment is decided on each location's *captured canonical path* (the
/// symlink-free path pinned at approval time), never on a freshly-resolved
/// string, so the result is:
/// - an antichain — no kept location is nested under (or equal to) another; and
/// - coverage-preserving — every input location is still contained by some kept
///   location, so the union of granted subtrees is unchanged.
///
/// # Why this can't widen a grant
///
/// The output is always a *subset* of the input: this only ever drops or keeps
/// whole input locations, never synthesizes a path. Every input location was
/// already captured/reopened and (for grants) approved and verified, so whatever
/// survives was independently approved. Deduping can therefore only *reduce*
/// access, never grant something new — which is exactly why it's safe to run even
/// though an attacker might swap a component: we compare pinned canonical strings
/// fixed at capture time (not the live filesystem), and even a stale containment
/// decision can only drop a child in favor of a parent that was itself approved.
pub fn normalize_host_filesystem_locations(
    locations: impl Iterator<Item = HostFilesystemLocation>,
) -> Vec<HostFilesystemLocation> {
    let mut result: Vec<HostFilesystemLocation> = Vec::new();
    for location in locations {
        // Already covered by (equal to, or nested under) a kept location: it
        // grants nothing new, so drop it.
        if result.iter().any(|existing| {
            location
                .canonical_or_raw_path()
                .starts_with(existing.canonical_or_raw_path())
        }) {
            continue;
        }
        // This location is broader: evict any kept locations nested under it.
        result.retain(|existing| {
            !existing
                .canonical_or_raw_path()
                .starts_with(location.canonical_or_raw_path())
        });
        result.push(location);
    }
    result
}

impl fmt::Debug for HostFilesystemLocation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Only the display path is shown; the trusted identity stays opaque.
        formatter
            .debug_struct("HostFilesystemLocation")
            .field("untrusted_raw_path", &self.untrusted_raw_path)
            .finish_non_exhaustive()
    }
}

impl PartialEq for HostFilesystemLocation {
    /// Two locations are equal when they refer to the **same filesystem object**,
    /// determined from the captured identity (the inode behind the `O_PATH` fd on
    /// Linux, the canonical path on macOS) — never from the textual raw path.
    /// This is what lets policy bookkeeping dedupe "the same location named two
    /// different ways," and refuse to treat "two different objects that happen to
    /// share a path string" as one.
    fn eq(&self, other: &Self) -> bool {
        self.canonical == other.canonical
    }
}

impl Eq for HostFilesystemLocation {}

#[cfg(all(test, target_os = "linux"))]
mod host_filesystem_location_tests {
    use super::{HostFilesystemLocation, normalize_host_filesystem_locations};
    use gpui::proptest::prelude::*;
    use std::fs;
    use std::os::unix::fs::symlink;

    /// The canonical path of `path` after fully resolving it, for use as the
    /// persisted grant identity.
    fn canonical(path: &std::path::Path) -> std::path::PathBuf {
        path.canonicalize().unwrap()
    }

    #[test]
    fn capture_resolves_symlink_and_reports_redirect() {
        let root = tempfile::tempdir().unwrap();
        let real = root.path().join("real");
        fs::create_dir(&real).unwrap();
        let link = root.path().join("link");
        symlink(&real, &link).unwrap();

        let location = HostFilesystemLocation::capture(&link).expect("capture link");
        let display = location.display();
        // The canonical path is the resolved target, and the redirect is
        // surfaced so the approval UI can highlight it.
        assert_eq!(
            display.canonical_path.to_string(),
            canonical(&real).display().to_string()
        );
        assert!(display.is_redirected);
        assert_eq!(
            display.untrusted_raw_path.to_string(),
            link.display().to_string()
        );
    }

    #[test]
    fn capture_of_plain_directory_is_not_a_redirect() {
        let root = tempfile::tempdir().unwrap();
        let dir = root.path().join("dir");
        fs::create_dir(&dir).unwrap();

        let location = HostFilesystemLocation::capture(&dir).expect("capture dir");
        assert!(!location.display().is_redirected);
    }

    #[test]
    fn reopen_accepts_unchanged_canonical_directory() {
        let root = tempfile::tempdir().unwrap();
        let dir = root.path().join("grant");
        fs::create_dir(&dir).unwrap();
        let canonical_dir = canonical(&dir);

        let location = HostFilesystemLocation::reopen(&dir, &canonical_dir)
            .expect("reopen unchanged directory");
        assert_eq!(location.linux_canonical_path(), canonical_dir);
    }

    #[test]
    fn reopen_accepts_legitimately_symlinked_parent() {
        // The consent flow persists the *resolved* canonical, so a grant whose
        // request legitimately went through a symlink still reopens cleanly:
        // there are no symlinks left in the canonical path.
        let root = tempfile::tempdir().unwrap();
        let real = root.path().join("real");
        fs::create_dir_all(real.join("child")).unwrap();
        let link = root.path().join("link");
        symlink(&real, &link).unwrap();

        let raw = link.join("child");
        let canonical_child = canonical(&raw); // .../real/child
        let location = HostFilesystemLocation::reopen(&raw, &canonical_child)
            .expect("reopen resolved child");
        assert_eq!(location.linux_canonical_path(), canonical_child);
    }

    #[test]
    fn reopen_rejects_intermediate_component_swapped_for_symlink() {
        // This is the pre-capture TOCTOU the fix closes: the grant was approved
        // for a real directory, then an intermediate component is swapped for a
        // symlink pointing at a sensitive location before the command runs.
        let root = tempfile::tempdir().unwrap();
        let base = root.path().join("base");
        let real = base.join("real");
        fs::create_dir_all(real.join("child")).unwrap();
        let canonical_child = canonical(&real.join("child")); // approved identity

        // Attacker plants a sensitive target and redirects `base/real` at it.
        let sensitive = base.join("sensitive");
        fs::create_dir_all(sensitive.join("child")).unwrap();
        fs::remove_dir_all(&real).unwrap();
        symlink(&sensitive, &real).unwrap();

        let error = HostFilesystemLocation::reopen(real.join("child"), &canonical_child)
            .expect_err("swapped intermediate component must be rejected");
        assert_eq!(error.kind(), std::io::ErrorKind::PermissionDenied);
    }

    #[test]
    fn reopen_rejects_symlink_leaf() {
        // The grant's leaf itself is replaced by a symlink after approval.
        let root = tempfile::tempdir().unwrap();
        let grant = root.path().join("grant");
        fs::create_dir(&grant).unwrap();
        let canonical_grant = canonical(&grant);

        let elsewhere = root.path().join("elsewhere");
        fs::create_dir(&elsewhere).unwrap();
        fs::remove_dir(&grant).unwrap();
        symlink(&elsewhere, &grant).unwrap();

        let error = HostFilesystemLocation::reopen(&grant, &canonical_grant)
            .expect_err("symlink leaf must be rejected");
        assert_eq!(error.kind(), std::io::ErrorKind::PermissionDenied);
    }

    #[test]
    fn normalize_drops_nested_child() {
        let root = tempfile::tempdir().unwrap();
        let parent = root.path().join("parent");
        let child = parent.join("child");
        fs::create_dir_all(&child).unwrap();

        let out = normalize_host_filesystem_locations(
            [
                HostFilesystemLocation::capture(&parent).unwrap(),
                HostFilesystemLocation::capture(&child).unwrap(),
            ]
            .into_iter(),
        );
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].linux_canonical_path(), canonical(&parent));
    }

    #[test]
    fn normalize_evicts_children_when_a_broader_parent_arrives() {
        let root = tempfile::tempdir().unwrap();
        let parent = root.path().join("parent");
        let child = parent.join("child");
        let sibling = parent.join("sibling");
        fs::create_dir_all(&child).unwrap();
        fs::create_dir_all(&sibling).unwrap();

        // The two children are inserted first, then the broader parent evicts
        // both.
        let out = normalize_host_filesystem_locations(
            [
                HostFilesystemLocation::capture(&child).unwrap(),
                HostFilesystemLocation::capture(&sibling).unwrap(),
                HostFilesystemLocation::capture(&parent).unwrap(),
            ]
            .into_iter(),
        );
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].linux_canonical_path(), canonical(&parent));
    }

    #[test]
    fn normalize_keeps_unrelated_siblings() {
        let root = tempfile::tempdir().unwrap();
        let a = root.path().join("a");
        let b = root.path().join("b");
        fs::create_dir_all(&a).unwrap();
        fs::create_dir_all(&b).unwrap();

        let out = normalize_host_filesystem_locations(
            [
                HostFilesystemLocation::capture(&a).unwrap(),
                HostFilesystemLocation::capture(&b).unwrap(),
            ]
            .into_iter(),
        );
        assert_eq!(out.len(), 2);
    }

    /// A path of 1..=4 components drawn from a tiny alphabet, so generated sets
    /// contain frequent nesting, siblings, and duplicates.
    fn path_specs() -> impl Strategy<Value = Vec<Vec<String>>> {
        let component = prop::sample::select(vec!["a".to_string(), "b".to_string(), "c".to_string()]);
        let path = prop::collection::vec(component, 1..=4);
        prop::collection::vec(path, 0..=8)
    }

    #[gpui::property_test]
    fn normalize_is_a_minimal_cover(#[strategy = path_specs()] specs: Vec<Vec<String>>) {
        // Materialize each generated path under a fresh root and capture it, so
        // the property runs against real pinned canonical identities rather than
        // bare strings.
        let root = tempfile::tempdir().unwrap();
        let mut inputs = Vec::new();
        for spec in &specs {
            let mut path = root.path().to_path_buf();
            path.extend(spec);
            fs::create_dir_all(&path).unwrap();
            inputs.push(HostFilesystemLocation::capture(&path).expect("capture generated dir"));
        }

        let normalized = normalize_host_filesystem_locations(inputs.iter().cloned());

        // Subset: every kept location came from the input (nothing synthesized).
        for kept in &normalized {
            assert!(inputs.contains(kept), "kept a location that wasn't an input");
        }

        // Antichain: no kept canonical is nested under (or equal to) another
        // (`starts_with` is reflexive, so this also rules out duplicates).
        for (i, a) in normalized.iter().enumerate() {
            for (j, b) in normalized.iter().enumerate() {
                if i != j {
                    assert!(
                        !a.linux_canonical_path()
                            .starts_with(b.linux_canonical_path()),
                        "{:?} covers {:?}",
                        a.linux_canonical_path(),
                        b.linux_canonical_path(),
                    );
                }
            }
        }

        // Coverage preserved: every input is still contained by some kept
        // location, so the union of granted subtrees is unchanged.
        for input in &inputs {
            assert!(
                normalized.iter().any(|kept| input
                    .linux_canonical_path()
                    .starts_with(kept.linux_canonical_path())),
                "{:?} is no longer covered",
                input.linux_canonical_path(),
            );
        }

        // Idempotent: normalizing an already-minimal set is a no-op.
        let again = normalize_host_filesystem_locations(normalized.iter().cloned());
        assert_eq!(again, normalized);
    }
}
