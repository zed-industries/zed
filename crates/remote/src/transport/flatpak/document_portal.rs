use ashpd::documents::{DocumentID, Documents};
use async_lock::OnceCell;
use std::path::{Component, Path, PathBuf};

/// Mount point for the XDG Document Portal's FUSE filesystem inside a Flatpak sandbox.
///
/// Ref <https://flatpak.github.io/xdg-desktop-portal/docs/documents-and-fuse.html>.
const SANDBOX_DOCUMENT_PORTAL_MOUNT: &str = "/run/flatpak/doc";

/// Connect to the XDG Document Portal. Connection is cached for performance
async fn document_portal() -> Result<&'static Documents, ashpd::Error> {
    static DOCUMENT_PORTAL: OnceCell<Documents> = OnceCell::new();
    DOCUMENT_PORTAL.get_or_try_init(Documents::new).await
}

/// Mount point for the XDG Document Portal FUSE filesystem on the host side
async fn document_portal_mount_point() -> Result<&'static PathBuf, ashpd::Error> {
    static DOCUMENT_PORTAL_MOUNT_POINT: OnceCell<PathBuf> = OnceCell::new();
    DOCUMENT_PORTAL_MOUNT_POINT
        .get_or_try_init(|| async {
            match document_portal().await?.mount_point().await {
                Ok(path) => Ok(path.as_ref().to_path_buf()),
                Err(err) => Err(err),
            }
        })
        .await
}

/// Lookup the path on the host to the file referenced in the given Path.
pub(super) async fn lookup_host_path(path: &Path) -> Result<Option<PathBuf>, ashpd::Error> {
    let portal = document_portal().await?;
    let mount_point = document_portal_mount_point().await?;

    let Some((doc_id, relative)) =
        split_doc_portal_path(Path::new(SANDBOX_DOCUMENT_PORTAL_MOUNT), path)
            .or_else(|| split_doc_portal_path(&mount_point, path))
    else {
        // Not a path managed by the Document Portal
        return Ok(None);
    };

    let host_paths = portal.host_paths(std::slice::from_ref(&doc_id)).await?;
    match host_paths.get(&doc_id) {
        Some(host_path) => Ok(Some(host_path.as_ref().join(relative))),
        None => Ok(None),
    }
}

/// Guess whether the given Path is probably a Document Portal FUSE path.
///
/// This is entirely heuristics-based and doesn't actually query the Document Portal or
/// the filesystem at all.
pub(super) fn is_likely_document_portal_path(path: &Path) -> bool {
    path.starts_with(SANDBOX_DOCUMENT_PORTAL_MOUNT)
        || path
            .strip_prefix("/run/user")
            .ok()
            .and_then(|rest| rest.components().nth(1))
            .is_some_and(|component| component.as_os_str() == "doc")
}

/// Split a Path from the XDG Document Portal FUSE filesystem into a relative path
/// and DocumentID for the shared document.
fn split_doc_portal_path(mount_point: &Path, path: &Path) -> Option<(DocumentID, PathBuf)> {
    let relative = path.strip_prefix(mount_point).ok()?;
    let mut components = relative.components();
    // First component after the mount is the document id
    let Component::Normal(doc_id) = components.next()? else {
        return None;
    };
    let doc_id = DocumentID::from(doc_id.to_string_lossy().into_owned());
    // Second component after the mount is the document's basename, which we can skip
    components.next();
    // Remaining components are relative to the document
    let relative = components.collect();
    Some((doc_id, relative))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognizes_likely_document_portal_paths() {
        // The in-sandbox mount and the host-side `/run/user/<uid>/doc` mount are
        // both recognized as likely document portal paths.
        assert!(is_likely_document_portal_path(Path::new(
            "/run/flatpak/doc/abcd1234/project"
        )));
        assert!(is_likely_document_portal_path(Path::new(
            "/run/user/1000/doc/abcd1234/project"
        )));

        // Unrelated host paths are not.
        assert!(!is_likely_document_portal_path(Path::new(
            "/home/user/project"
        )));
        assert!(!is_likely_document_portal_path(Path::new(
            "/run/user/1000/other/path"
        )));
    }

    #[test]
    fn splits_document_portal_paths() {
        let mount = Path::new("/run/user/1000/doc");

        // A folder exported directly as `<mount>/<doc_id>/<entry>` has no nested
        // sub-path: the resolved host path of the document is the answer as-is.
        let (doc_id, relative) =
            split_doc_portal_path(mount, Path::new("/run/user/1000/doc/abcd1234/myproject"))
                .unwrap();
        assert_eq!(&*doc_id, "abcd1234");
        assert_eq!(relative, PathBuf::new());

        // A file nested inside the exported folder keeps the components below the
        // entry, so they can be re-attached to the resolved host path.
        let (doc_id, relative) = split_doc_portal_path(
            mount,
            Path::new("/run/user/1000/doc/abcd1234/myproject/src/main.rs"),
        )
        .unwrap();
        assert_eq!(&*doc_id, "abcd1234");
        assert_eq!(relative, PathBuf::from("src/main.rs"));
    }

    #[test]
    fn splits_document_portal_paths_with_trailing_slash_mount() {
        // `GetMountPoint` typically reports the mount with a trailing slash; that
        // must not change how the path is split.
        let mount = Path::new("/run/user/1000/doc/");
        let (doc_id, relative) = split_doc_portal_path(
            mount,
            Path::new("/run/user/1000/doc/abcd1234/myproject/src/main.rs"),
        )
        .unwrap();
        assert_eq!(&*doc_id, "abcd1234");
        assert_eq!(relative, PathBuf::from("src/main.rs"));
    }

    #[test]
    fn splits_in_sandbox_document_portal_paths() {
        // Inside the sandbox the portal is mounted at `/run/flatpak/doc`, not the
        // host-side mount that `GetMountPoint` reports. This mirrors the path Zed
        // actually receives from the file chooser.
        let mount = Path::new(SANDBOX_DOCUMENT_PORTAL_MOUNT);
        let (doc_id, relative) =
            split_doc_portal_path(mount, Path::new("/run/flatpak/doc/abcd1234/project")).unwrap();
        assert_eq!(&*doc_id, "abcd1234");
        assert_eq!(relative, PathBuf::new());

        let (doc_id, relative) = split_doc_portal_path(
            mount,
            Path::new("/run/flatpak/doc/abcd1234/project/src/main.rs"),
        )
        .unwrap();
        assert_eq!(&*doc_id, "abcd1234");
        assert_eq!(relative, PathBuf::from("src/main.rs"));
    }

    #[test]
    fn ignores_paths_outside_the_portal_mount() {
        let mount = Path::new("/run/user/1000/doc");

        // A plain host path is not under the portal mount.
        assert!(split_doc_portal_path(mount, Path::new("/home/user/project")).is_none());

        // The mount point itself carries no document id to resolve.
        assert!(split_doc_portal_path(mount, Path::new("/run/user/1000/doc")).is_none());
    }

    #[test]
    fn reattaches_sub_path_to_resolved_host_path() {
        // End-to-end shape of the mapping: the split sub-path is joined onto the
        // host path that `host_paths(doc_id)` would return for the exported entry.
        let mount = Path::new("/run/user/1000/doc");
        let (_doc_id, relative) = split_doc_portal_path(
            mount,
            Path::new("/run/user/1000/doc/abcd1234/myproject/src/main.rs"),
        )
        .unwrap();
        let host_entry = Path::new("/home/user/dev/myproject");
        assert_eq!(
            host_entry.join(relative),
            PathBuf::from("/home/user/dev/myproject/src/main.rs"),
        );
    }
}
