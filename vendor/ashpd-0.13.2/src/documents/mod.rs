//! # Examples
//!
//! ```rust,no_run
//! use std::str::FromStr;
//!
//! use ashpd::{
//!     AppID,
//!     documents::{Documents, Permission},
//! };
//!
//! async fn run() -> ashpd::Result<()> {
//!     let proxy = Documents::new().await?;
//!
//!     println!("{:#?}", proxy.mount_point().await?);
//!     let app_id = AppID::from_str("org.mozilla.firefox").unwrap();
//!     for (doc_id, host_path) in proxy.list(Some(&app_id)).await? {
//!         if doc_id == "f2ee988d".into() {
//!             let info = proxy.info(doc_id).await?;
//!             println!("{:#?}", info);
//!         }
//!     }
//!
//!     proxy
//!         .grant_permissions("f2ee988d", &app_id, &[Permission::GrantPermissions])
//!         .await?;
//!     proxy
//!         .revoke_permissions("f2ee988d", &app_id, &[Permission::Write])
//!         .await?;
//!
//!     proxy.delete("f2ee988d").await?;
//!
//!     Ok(())
//! }
//! ```

use std::{collections::HashMap, fmt, os::fd::AsFd, path::Path, str::FromStr};

use enumflags2::{BitFlags, bitflags};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use zbus::zvariant::{Fd, OwnedValue, Type};

pub use crate::app_id::DocumentID;
use crate::{AppID, Error, FilePath, proxy::Proxy};

#[bitflags]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Eq, Copy, Clone, Debug, Type)]
#[repr(u32)]
/// Document flags
pub enum DocumentFlags {
    /// Reuse the existing document store entry for the file.
    ReuseExisting,
    /// Persistent file.
    Persistent,
    /// Depends on the application needs.
    AsNeededByApp,
    /// Export a directory.
    ExportDirectory,
}

/// A [`HashMap`] mapping application IDs to the permissions for that
/// application
pub type Permissions = HashMap<AppID, Vec<Permission>>;

#[cfg_attr(feature = "glib", derive(glib::Enum))]
#[cfg_attr(feature = "glib", enum_type(name = "AshpdPermission"))]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Eq, Type)]
#[zvariant(signature = "s")]
#[serde(rename_all = "kebab-case")]
/// The possible permissions to grant to a specific application for a specific
/// document.
pub enum Permission {
    /// Read access.
    Read,
    /// Write access.
    Write,
    /// The possibility to grant new permissions to the file.
    GrantPermissions,
    /// Delete access.
    Delete,
}

impl fmt::Display for Permission {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Read => write!(f, "Read"),
            Self::Write => write!(f, "Write"),
            Self::GrantPermissions => write!(f, "Grant Permissions"),
            Self::Delete => write!(f, "Delete"),
        }
    }
}

impl AsRef<str> for Permission {
    fn as_ref(&self) -> &str {
        match self {
            Self::Read => "Read",
            Self::Write => "Write",
            Self::GrantPermissions => "Grant Permissions",
            Self::Delete => "Delete",
        }
    }
}

impl From<Permission> for &'static str {
    fn from(p: Permission) -> Self {
        match p {
            Permission::Read => "Read",
            Permission::Write => "Write",
            Permission::GrantPermissions => "Grant Permissions",
            Permission::Delete => "Delete",
        }
    }
}

impl FromStr for Permission {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Read" | "read" => Ok(Permission::Read),
            "Write" | "write" => Ok(Permission::Write),
            "GrantPermissions" | "grant-permissions" => Ok(Permission::GrantPermissions),
            "Delete" | "delete" => Ok(Permission::Delete),
            _ => Err(Error::ParseError("Failed to parse priority, invalid value")),
        }
    }
}

/// The interface lets sandboxed applications make files from the outside world
/// available to sandboxed applications in a controlled way.
///
/// Exported files will be made accessible to the application via a fuse
/// filesystem that gets mounted at `/run/user/$UID/doc/`. The filesystem gets
/// mounted both outside and inside the sandbox, but the view inside the sandbox
/// is restricted to just those files that the application is allowed to access.
///
/// Individual files will appear at `/run/user/$UID/doc/$DOC_ID/filename`,
/// where `$DOC_ID` is the ID of the file in the document store.
/// It is returned by the [`Documents::add`] and
/// [`Documents::add_named`] calls.
///
/// The permissions that the application has for a document store entry (see
/// [`Documents::grant_permissions`]) are reflected in the POSIX mode bits
/// in the fuse filesystem.
///
/// Wrapper of the DBus interface: [`org.freedesktop.portal.Documents`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Documents.html).
#[derive(Debug)]
#[doc(alias = "org.freedesktop.portal.Documents")]
pub struct Documents(Proxy<'static>);

impl Documents {
    /// Create a new instance of [`Documents`].
    pub async fn new() -> Result<Self, Error> {
        let proxy = Proxy::new_documents("org.freedesktop.portal.Documents").await?;
        Ok(Self(proxy))
    }

    /// Create a new instance of [`Documents`].
    pub async fn with_connection(connection: zbus::Connection) -> Result<Self, Error> {
        let proxy =
            Proxy::new_documents_with_connection(connection, "org.freedesktop.portal.Documents")
                .await?;
        Ok(Self(proxy))
    }

    /// Returns the version of the portal interface.
    pub fn version(&self) -> u32 {
        self.0.version()
    }

    /// Adds a file to the document store.
    /// The file is passed in the form of an open file descriptor
    /// to prove that the caller has access to the file.
    ///
    /// # Arguments
    ///
    /// * `o_path_fd` - Open file descriptor for the file to add.
    /// * `reuse_existing` - Whether to reuse an existing document store entry
    ///   for the file.
    /// * `persistent` - Whether to add the file only for this session or
    ///   permanently.
    ///
    /// # Returns
    ///
    /// The ID of the file in the document store.
    ///
    /// # Specifications
    ///
    /// See also [`Add`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Documents.html#org-freedesktop-portal-documents-add).
    #[doc(alias = "Add")]
    pub async fn add(
        &self,
        o_path_fd: &impl AsFd,
        reuse_existing: bool,
        persistent: bool,
    ) -> Result<DocumentID, Error> {
        self.0
            .call("Add", &(Fd::from(o_path_fd), reuse_existing, persistent))
            .await
    }

    /// Adds multiple files to the document store.
    /// The files are passed in the form of an open file descriptor
    /// to prove that the caller has access to the file.
    ///
    /// # Arguments
    ///
    /// * `o_path_fds` - Open file descriptors for the files to export.
    /// * `flags` - A [`DocumentFlags`].
    /// * `app_id` - An application ID, or `None`.
    /// * `permissions` - The permissions to grant.
    ///
    /// # Returns
    ///
    /// The IDs of the files in the document store along with other extra info.
    ///
    /// # Required version
    ///
    /// The method requires the 2nd version implementation of the portal and
    /// would fail with [`Error::RequiresVersion`] otherwise.
    ///
    /// # Specifications
    ///
    /// See also [`AddFull`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Documents.html#org-freedesktop-portal-documents-addfull).
    #[doc(alias = "AddFull")]
    pub async fn add_full(
        &self,
        o_path_fds: &[impl AsFd],
        flags: BitFlags<DocumentFlags>,
        app_id: Option<&AppID>,
        permissions: &[Permission],
    ) -> Result<(Vec<DocumentID>, HashMap<String, OwnedValue>), Error> {
        let o_path: Vec<Fd> = o_path_fds.iter().map(Fd::from).collect();
        let app_id = app_id.map(|id| id.as_ref()).unwrap_or("");
        self.0
            .call_versioned("AddFull", &(o_path, flags, app_id, permissions), 2)
            .await
    }

    /// Creates an entry in the document store for writing a new file.
    ///
    /// # Arguments
    ///
    /// * `o_path_parent_fd` - Open file descriptor for the parent directory.
    /// * `filename` - The basename for the file.
    /// * `reuse_existing` - Whether to reuse an existing document store entry
    ///   for the file.
    /// * `persistent` - Whether to add the file only for this session or
    ///   permanently.
    ///
    /// # Returns
    ///
    /// The ID of the file in the document store.
    ///
    /// # Specifications
    ///
    /// See also [`AddNamed`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Documents.html#org-freedesktop-portal-documents-addnamed).
    #[doc(alias = "AddNamed")]
    pub async fn add_named(
        &self,
        o_path_parent_fd: &impl AsFd,
        filename: impl AsRef<Path>,
        reuse_existing: bool,
        persistent: bool,
    ) -> Result<DocumentID, Error> {
        let filename = FilePath::new(filename)?;
        self.0
            .call(
                "AddNamed",
                &(
                    Fd::from(o_path_parent_fd),
                    filename,
                    reuse_existing,
                    persistent,
                ),
            )
            .await
    }

    /// Adds multiple files to the document store.
    /// The files are passed in the form of an open file descriptor
    /// to prove that the caller has access to the file.
    ///
    /// # Arguments
    ///
    /// * `o_path_fd` - Open file descriptor for the parent directory.
    /// * `filename` - The basename for the file.
    /// * `flags` - A [`DocumentFlags`].
    /// * `app_id` - An application ID, or `None`.
    /// * `permissions` - The permissions to grant.
    ///
    /// # Returns
    ///
    /// The ID of the file in the document store along with other extra info.
    ///
    /// # Required version
    ///
    /// The method requires the 3nd version implementation of the portal and
    /// would fail with [`Error::RequiresVersion`] otherwise.
    ///
    /// # Specifications
    ///
    /// See also [`AddNamedFull`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Documents.html#org-freedesktop-portal-documents-addnamedfull).
    #[doc(alias = "AddNamedFull")]
    pub async fn add_named_full(
        &self,
        o_path_fd: &impl AsFd,
        filename: impl AsRef<Path>,
        flags: BitFlags<DocumentFlags>,
        app_id: Option<&AppID>,
        permissions: &[Permission],
    ) -> Result<(DocumentID, HashMap<String, OwnedValue>), Error> {
        let app_id = app_id.map(|id| id.as_ref()).unwrap_or("");
        let filename = FilePath::new(filename)?;
        self.0
            .call_versioned(
                "AddNamedFull",
                &(Fd::from(o_path_fd), filename, flags, app_id, permissions),
                3,
            )
            .await
    }

    /// Removes an entry from the document store. The file itself is not
    /// deleted.
    ///
    /// **Note** This call is available inside the sandbox if the
    /// application has the [`Permission::Delete`] for the document.
    ///
    /// # Arguments
    ///
    /// * `doc_id` - The ID of the file in the document store.
    ///
    /// # Specifications
    ///
    /// See also [`Delete`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Documents.html#org-freedesktop-portal-documents-delete).
    #[doc(alias = "Delete")]
    pub async fn delete(&self, doc_id: impl Into<DocumentID>) -> Result<(), Error> {
        self.0.call("Delete", &(doc_id.into())).await
    }

    /// Returns the path at which the document store fuse filesystem is mounted.
    /// This will typically be `/run/user/$UID/doc/`.
    ///
    /// # Specifications
    ///
    /// See also [`GetMountPoint`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Documents.html#org-freedesktop-portal-documents-getmountpoint).
    #[doc(alias = "GetMountPoint")]
    #[doc(alias = "get_mount_point")]
    pub async fn mount_point(&self) -> Result<FilePath, Error> {
        self.0.call("GetMountPoint", &()).await
    }

    /// Grants access permissions for a file in the document store to an
    /// application.
    ///
    /// **Note** This call is available inside the sandbox if the
    /// application has the [`Permission::GrantPermissions`] for the document.
    ///
    /// # Arguments
    ///
    /// * `doc_id` - The ID of the file in the document store.
    /// * `app_id` - The ID of the application to which permissions are granted.
    /// * `permissions` - The permissions to grant.
    ///
    /// # Specifications
    ///
    /// See also [`GrantPermissions`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Documents.html#org-freedesktop-portal-documents-grantpermissions).
    #[doc(alias = "GrantPermissions")]
    pub async fn grant_permissions(
        &self,
        doc_id: impl Into<DocumentID>,
        app_id: &AppID,
        permissions: &[Permission],
    ) -> Result<(), Error> {
        self.0
            .call("GrantPermissions", &(doc_id.into(), app_id, permissions))
            .await
    }

    /// Gets the filesystem path and application permissions for a document
    /// store entry.
    ///
    /// **Note** This call is not available inside the sandbox.
    ///
    /// # Arguments
    ///
    /// * `doc_id` - The ID of the file in the document store.
    ///
    /// # Returns
    ///
    /// The path of the file in the host filesystem along with the
    /// [`Permissions`].
    ///
    /// # Specifications
    ///
    /// See also [`Info`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Documents.html#org-freedesktop-portal-documents-info).
    #[doc(alias = "Info")]
    pub async fn info(
        &self,
        doc_id: impl Into<DocumentID>,
    ) -> Result<(FilePath, Permissions), Error> {
        self.0.call("Info", &(doc_id.into())).await
    }

    /// Lists documents in the document store for an application (or for all
    /// applications).
    ///
    /// **Note** This call is not available inside the sandbox.
    ///
    /// # Arguments
    ///
    /// * `app-id` - The application ID, or `None` to list all documents.
    ///
    /// # Returns
    ///
    /// [`HashMap`] mapping document IDs to their filesystem path on the host
    /// system.
    ///
    /// # Specifications
    ///
    /// See also [`List`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Documents.html#org-freedesktop-portal-documents-list).
    #[doc(alias = "List")]
    pub async fn list(
        &self,
        app_id: Option<&AppID>,
    ) -> Result<HashMap<DocumentID, FilePath>, Error> {
        let app_id = app_id.map(|id| id.as_ref()).unwrap_or("");
        let response: HashMap<String, FilePath> = self.0.call("List", &(app_id)).await?;

        let mut new_response: HashMap<DocumentID, FilePath> = HashMap::new();
        for (key, file_name) in response {
            new_response.insert(DocumentID::from(key), file_name);
        }

        Ok(new_response)
    }

    /// Looks up the document ID for a file.
    ///
    /// **Note** This call is not available inside the sandbox.
    ///
    /// # Arguments
    ///
    /// * `filename` - A path in the host filesystem.
    ///
    /// # Returns
    ///
    /// The ID of the file in the document store, or [`None`] if the file is not
    /// in the document store.
    ///
    /// # Specifications
    ///
    /// See also [`Lookup`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Documents.html#org-freedesktop-portal-documents-lookup).
    #[doc(alias = "Lookup")]
    pub async fn lookup(&self, filename: impl AsRef<Path>) -> Result<Option<DocumentID>, Error> {
        let filename = FilePath::new(filename)?;
        let doc_id: String = self.0.call("Lookup", &(filename)).await?;
        if doc_id.is_empty() {
            Ok(None)
        } else {
            Ok(Some(doc_id.into()))
        }
    }

    /// Revokes access permissions for a file in the document store from an
    /// application.
    ///
    /// **Note** This call is available inside the sandbox if the
    /// application has the [`Permission::GrantPermissions`] for the document.
    ///
    /// # Arguments
    ///
    /// * `doc_id` - The ID of the file in the document store.
    /// * `app_id` - The ID of the application from which the permissions are
    ///   revoked.
    /// * `permissions` - The permissions to revoke.
    ///
    /// # Specifications
    ///
    /// See also [`RevokePermissions`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Documents.html#org-freedesktop-portal-documents-revokepermissions).
    #[doc(alias = "RevokePermissions")]
    pub async fn revoke_permissions(
        &self,
        doc_id: impl Into<DocumentID>,
        app_id: &AppID,
        permissions: &[Permission],
    ) -> Result<(), Error> {
        self.0
            .call("RevokePermissions", &(doc_id.into(), app_id, permissions))
            .await
    }

    /// Retrieves the host filesystem paths from their document IDs.
    ///
    /// # Arguments
    ///
    /// * `doc_ids` - A list of file IDs in the document store.
    ///
    /// # Returns
    ///
    /// A dictionary mapping document IDs to the paths in the host filesystem
    ///
    /// # Specifications
    ///
    /// See also [`GetHostPaths`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Documents.html#org-freedesktop-portal-documents-gethostpaths).
    #[doc(alias = "GetHostPaths")]
    pub async fn host_paths(
        &self,
        doc_ids: &[DocumentID],
    ) -> Result<HashMap<DocumentID, FilePath>, Error> {
        self.0.call_versioned("GetHostPaths", &(doc_ids,), 5).await
    }
}

impl std::ops::Deref for Documents {
    type Target = zbus::Proxy<'static>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Interact with `org.freedesktop.portal.FileTransfer` interface.
pub mod file_transfer;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use zbus::zvariant::Type;

    use crate::{FilePath, app_id::DocumentID, documents::Permission};

    #[test]
    fn serialize_deserialize() {
        let permission = Permission::GrantPermissions;
        let string = serde_json::to_string(&permission).unwrap();
        assert_eq!(string, "\"grant-permissions\"");

        let decoded = serde_json::from_str(&string).unwrap();
        assert_eq!(permission, decoded);

        assert_eq!(HashMap::<DocumentID, FilePath>::SIGNATURE, "a{say}");
    }
}
