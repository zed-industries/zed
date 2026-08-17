//! # Examples
//!
//! ```rust,no_run
//! use std::{fs::File, os::fd::AsFd};
//!
//! use ashpd::documents::file_transfer::{FileTransfer, StartTransferOptions};
//!
//! async fn run() -> ashpd::Result<()> {
//!     let proxy = FileTransfer::new().await?;
//!
//!     let key = proxy
//!         .start_transfer(
//!             StartTransferOptions::default()
//!                 .set_writeable(true)
//!                 .set_auto_stop(true),
//!         )
//!         .await?;
//!     let file = File::open("/home/bilelmoussaoui/Downloads/adwaita-night.jpg").unwrap();
//!     proxy
//!         .add_files(&key, &[&file.as_fd()], Default::default())
//!         .await?;
//!
//!     // The files would be retrieved by another process
//!     let files = proxy.retrieve_files(&key, Default::default()).await?;
//!     println!("{:#?}", files);
//!
//!     proxy.stop_transfer(&key).await?;
//!
//!     Ok(())
//! }
//! ```

use std::os::fd::AsFd;

use futures_util::Stream;
use serde::Serialize;
use zbus::zvariant::{Fd, Type, as_value::optional};

use crate::{Error, proxy::Proxy};

#[derive(Serialize, Type, Debug, Default)]
/// Specified options for a [`FileTransfer::start_transfer`] request.
#[zvariant(signature = "dict")]
pub struct StartTransferOptions {
    #[serde(with = "optional", skip_serializing_if = "Option::is_none")]
    writeable: Option<bool>,
    #[serde(
        rename = "autostop",
        with = "optional",
        skip_serializing_if = "Option::is_none"
    )]
    auto_stop: Option<bool>,
}

impl StartTransferOptions {
    /// Sets whether the chosen application can write to the files or not.
    #[must_use]
    pub fn set_writeable(mut self, writeable: impl Into<Option<bool>>) -> Self {
        self.writeable = writeable.into();
        self
    }

    /// Whether to stop the transfer automatically after the first
    /// [`retrieve_files()`][`FileTransfer::retrieve_files`] call.
    #[must_use]
    pub fn set_auto_stop(mut self, auto_stop: impl Into<Option<bool>>) -> Self {
        self.auto_stop = auto_stop.into();
        self
    }
}

#[derive(Default, Debug, Serialize, Type)]
#[zvariant(signature = "dict")]
/// Specified options for a [`FileTransfer::add_files`] request.
pub struct AddFilesOptions {}

#[derive(Default, Debug, Serialize, Type)]
#[zvariant(signature = "dict")]
/// Specified options for a [`FileTransfer::retrieve_files`] request.
pub struct RetrieveFilesOptions {}

/// The interface operates as a middle-man between apps when transferring files
/// via drag-and-drop or copy-paste, taking care of the necessary exporting of
/// files in the document portal.
///
/// Toolkits are expected to use the application/vnd.portal.filetransfer
/// mimetype when using this mechanism for file exchange via copy-paste or
/// drag-and-drop.
///
/// The data that is transmitted with this mimetype should be the key returned
/// by the StartTransfer method. Upon receiving this mimetype, the target should
/// call RetrieveFiles with the key, to obtain the list of files. The portal
/// will take care of exporting files in the document store as necessary to make
/// them accessible to the target.
///
/// Wrapper of the DBus interface: [`org.freedesktop.portal.FileTransfer`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.FileTransfer.html).
#[derive(Debug)]
#[doc(alias = "org.freedesktop.portal.FileTransfer")]
pub struct FileTransfer(Proxy<'static>);

impl FileTransfer {
    /// Create a new instance of [`FileTransfer`].
    pub async fn new() -> Result<Self, Error> {
        let proxy = Proxy::new_documents("org.freedesktop.portal.FileTransfer").await?;
        Ok(Self(proxy))
    }

    /// Create a new instance of [`FileTransfer`].
    pub async fn with_connection(connection: zbus::Connection) -> Result<Self, Error> {
        let proxy =
            Proxy::new_documents_with_connection(connection, "org.freedesktop.portal.FileTransfer")
                .await?;
        Ok(Self(proxy))
    }

    /// Returns the version of the portal interface.
    pub fn version(&self) -> u32 {
        self.0.version()
    }

    /// Adds files to a session. This method can be called multiple times on a
    /// given session. **Note** only regular files (not directories) can be
    /// added.
    ///
    /// # Arguments
    ///
    /// * `key` - A key returned by
    ///   [`start_transfer()`][`FileTransfer::start_transfer`].
    /// * `fds` - A list of file descriptors of the files to register.
    ///
    /// # Specifications
    ///
    /// See also [`AddFiles`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.FileTransfer.html#org-freedesktop-portal-filetransfer-addfiles).
    #[doc(alias = "AddFiles")]
    pub async fn add_files(
        &self,
        key: &str,
        fds: &[impl AsFd],
        options: AddFilesOptions,
    ) -> Result<(), Error> {
        let files: Vec<Fd> = fds.iter().map(Fd::from).collect();

        self.0.call("AddFiles", &(key, files, options)).await
    }

    /// Retrieves files that were previously added to the session with
    /// [`add_files()`][`FileTransfer::add_files`]. The files will be
    /// exported in the document portal as-needed for the caller, and they
    /// will be writeable if the owner of the session allowed it.
    ///
    /// # Arguments
    ///
    /// * `key` - A key returned by
    ///   [`start_transfer()`][`FileTransfer::start_transfer`].
    ///
    /// # Returns
    ///
    /// The list of file paths.
    ///
    /// # Specifications
    ///
    /// See also [`RetrieveFiles`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.FileTransfer.html#org-freedesktop-portal-filetransfer-retrievefiles).
    #[doc(alias = "RetrieveFiles")]
    pub async fn retrieve_files(
        &self,
        key: &str,
        options: RetrieveFilesOptions,
    ) -> Result<Vec<String>, Error> {
        self.0.call("RetrieveFiles", &(key, options)).await
    }

    /// Starts a session for a file transfer.
    /// The caller should call [`add_files()`][`FileTransfer::add_files`]
    /// at least once, to add files to this session.
    ///
    /// # Arguments
    ///
    /// * `writeable` - Sets whether the chosen application can write to the
    ///   files or not.
    /// * `auto_stop` - Whether to stop the transfer automatically after the
    ///   first [`retrieve_files()`][`FileTransfer::retrieve_files`] call.
    ///
    /// # Returns
    ///
    /// Key that can be passed to
    /// [`retrieve_files()`][`FileTransfer::retrieve_files`] to obtain the
    /// files.
    ///
    /// # Specifications
    ///
    /// See also [`StartTransfer`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.FileTransfer.html#org-freedesktop-portal-filetransfer-starttransfer).
    #[doc(alias = "StartTransfer")]
    pub async fn start_transfer(&self, options: StartTransferOptions) -> Result<String, Error> {
        self.0.call("StartTransfer", &(options)).await
    }

    /// Ends the transfer.
    /// Further calls to [`add_files()`][`FileTransfer::add_files`] or
    /// [`retrieve_files()`][`FileTransfer::retrieve_files`] for this key
    /// will return an error.
    ///
    /// # Arguments
    ///
    /// * `key` - A key returned by
    ///   [`start_transfer()`][`FileTransfer::start_transfer`].
    ///
    /// # Specifications
    ///
    /// See also [`StopTransfer`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.FileTransfer.html#org-freedesktop-portal-filetransfer-stoptransfer).
    #[doc(alias = "StopTransfer")]
    pub async fn stop_transfer(&self, key: &str) -> Result<(), Error> {
        self.0.call("StopTransfer", &(key)).await
    }

    /// Emitted when the transfer is closed.
    ///
    /// # Returns
    ///
    /// * The key returned by
    ///   [`start_transfer()`][`FileTransfer::start_transfer`].
    ///
    /// # Specifications
    ///
    /// See also [`TransferClosed`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.FileTransfer.html#org-freedesktop-portal-filetransfer-transferclosed).
    #[doc(alias = "TransferClosed")]
    pub async fn transfer_closed(&self) -> Result<impl Stream<Item = String>, Error> {
        self.0.signal("TransferClosed").await
    }
}

impl std::ops::Deref for FileTransfer {
    type Target = zbus::Proxy<'static>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
