//! Move a file to the trash.
//!
//! # Examples
//!
//!
//! ```rust,no_run
//! use std::{fs::File, os::fd::AsFd};
//!
//! use ashpd::desktop::trash;
//!
//! async fn run() -> ashpd::Result<()> {
//!     let path = "/home/bilelmoussaoui/adwaita-night.jpg";
//!     let file = std::fs::OpenOptions::new()
//!         .write(true)
//!         .read(true)
//!         .open(path)
//!         .unwrap();
//!     trash::trash_file(&file.as_fd()).await?;
//!     Ok(())
//! }
//! ```
//!
//! Or by using the Proxy directly
//!
//! ```rust,no_run
//! use std::{fs::File, os::fd::AsFd};
//!
//! use ashpd::desktop::trash::TrashProxy;
//!
//! async fn run() -> ashpd::Result<()> {
//!     let path = "/home/bilelmoussaoui/adwaita-night.jpg";
//!     let file = std::fs::OpenOptions::new()
//!         .write(true)
//!         .read(true)
//!         .open(path)
//!         .unwrap();
//!     let proxy = TrashProxy::new().await?;
//!     proxy.trash_file(&file.as_fd()).await?;
//!     Ok(())
//! }
//! ```

use std::os::fd::AsFd;

use serde_repr::{Deserialize_repr, Serialize_repr};
use zbus::zvariant::{Fd, Type};

use crate::{Error, error::PortalError, proxy::Proxy};

#[derive(Debug, Deserialize_repr, Serialize_repr, PartialEq, Type)]
#[repr(u32)]
enum TrashStatus {
    Failed = 0,
    Succeeded = 1,
}

/// The interface lets sandboxed applications send files to the trashcan.
///
/// Wrapper of the DBus interface: [`org.freedesktop.portal.Trash`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Trash.html).
#[derive(Debug)]
#[doc(alias = "org.freedesktop.portal.Trash")]
pub struct TrashProxy(Proxy<'static>);

impl TrashProxy {
    /// Create a new instance of [`TrashProxy`].
    pub async fn new() -> Result<Self, Error> {
        let proxy = Proxy::new_desktop("org.freedesktop.portal.Trash").await?;
        Ok(Self(proxy))
    }

    /// Create a new instance of [`TrashProxy`].
    pub async fn with_connection(connection: zbus::Connection) -> Result<Self, Error> {
        let proxy =
            Proxy::new_desktop_with_connection(connection, "org.freedesktop.portal.Trash").await?;
        Ok(Self(proxy))
    }

    /// Returns the version of the portal interface.
    pub fn version(&self) -> u32 {
        self.0.version()
    }

    /// Sends a file to the trashcan.
    /// Applications are allowed to trash a file if they can open it in
    /// read/write mode.
    ///
    /// # Arguments
    ///
    /// * `fd` - The file descriptor.
    ///
    /// # Specifications
    ///
    /// See also [`TrashFile`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Trash.html#org-freedesktop-portal-trash-trashfile).
    #[doc(alias = "TrashFile")]
    #[doc(alias = "xdp_portal_trash_file")]
    pub async fn trash_file(&self, fd: &impl AsFd) -> Result<(), Error> {
        let status = self.0.call("TrashFile", &(Fd::from(fd))).await?;
        match status {
            TrashStatus::Failed => Err(Error::Portal(PortalError::Failed(
                "Failed to trash file".to_string(),
            ))),
            TrashStatus::Succeeded => Ok(()),
        }
    }
}

impl std::ops::Deref for TrashProxy {
    type Target = zbus::Proxy<'static>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[doc(alias = "xdp_portal_trash_file")]
/// A handy wrapper around [`TrashProxy::trash_file`].
pub async fn trash_file(fd: &impl AsFd) -> Result<(), Error> {
    let proxy = TrashProxy::new().await?;
    proxy.trash_file(fd).await
}

#[cfg(test)]
mod test {
    use super::TrashStatus;

    #[test]
    fn status_serde() {
        #[derive(serde::Serialize, serde::Deserialize)]
        struct Test {
            status: TrashStatus,
        }

        let status = Test {
            status: TrashStatus::Failed,
        };

        let x = serde_json::to_string(&status).unwrap();
        let y: Test = serde_json::from_str(&x).unwrap();
        assert_eq!(y.status, TrashStatus::Failed);
    }
}
