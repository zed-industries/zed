//! The interface lets sandboxed applications ask the user for access to files
//! outside the sandbox. The portal backend will present the user with a file
//! chooser dialog.
//!
//! Wrapper of the DBus interface: [`org.freedesktop.portal.FileChooser`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.FileChooser.html).
//!
//! ### Examples
//!
//! #### Opening a file
//!
//! ```rust,no_run
//! use ashpd::desktop::file_chooser::{Choice, FileFilter, SelectedFiles};
//!
//! async fn run() -> ashpd::Result<()> {
//!     let files = SelectedFiles::open_file()
//!         .title("open a file to read")
//!         .accept_label("read")
//!         .modal(true)
//!         .multiple(true)
//!         .choice(
//!             Choice::new("encoding", "Encoding", "latin15")
//!                 .insert("utf8", "Unicode (UTF-8)")
//!                 .insert("latin15", "Western"),
//!         )
//!         // A trick to have a checkbox
//!         .choice(Choice::boolean("re-encode", "Re-encode", false))
//!         .filter(FileFilter::new("SVG Image").mimetype("image/svg+xml"))
//!         .send()
//!         .await?
//!         .response()?;
//!
//!     println!("{:#?}", files);
//!
//!     Ok(())
//! }
//! ```
//!
//! #### Ask to save a file
//!
//! ```rust,no_run
//! use ashpd::desktop::file_chooser::{FileFilter, SelectedFiles};
//!
//! async fn run() -> ashpd::Result<()> {
//!     let files = SelectedFiles::save_file()
//!         .title("open a file to write")
//!         .accept_label("write")
//!         .current_name("image.jpg")
//!         .modal(true)
//!         .filter(FileFilter::new("JPEG Image").glob("*.jpg"))
//!         .send()
//!         .await?
//!         .response()?;
//!
//!     println!("{:#?}", files);
//!
//!     Ok(())
//! }
//! ```
//!
//! #### Ask to save multiple files
//!
//! ```rust,no_run
//! use ashpd::desktop::file_chooser::SelectedFiles;
//!
//! async fn run() -> ashpd::Result<()> {
//!     let files = SelectedFiles::save_files()
//!         .title("open files to write")
//!         .accept_label("write files")
//!         .modal(true)
//!         .current_folder("/home/bilelmoussaoui/Pictures")?
//!         .files(&["test.jpg", "awesome.png"])?
//!         .send()
//!         .await?
//!         .response()?;
//!
//!     println!("{:#?}", files);
//!
//!     Ok(())
//! }
//! ```

use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use zbus::zvariant::{
    Optional, Type,
    as_value::{self, optional},
};

use super::{HandleToken, Request};
use crate::{Error, FilePath, Uri, WindowIdentifier, proxy::Proxy};

#[derive(Clone, Serialize, Deserialize, Type, Debug, PartialEq)]
/// A file filter, to limit the available file choices to a mimetype or a glob
/// pattern.
pub struct FileFilter(String, Vec<(FilterType, String)>);

#[derive(Clone, Serialize_repr, Deserialize_repr, Debug, Type, PartialEq)]
#[repr(u32)]
enum FilterType {
    GlobPattern = 0,
    MimeType = 1,
}

impl FilterType {
    /// Whether it is a mime type filter.
    fn is_mimetype(&self) -> bool {
        matches!(self, FilterType::MimeType)
    }

    /// Whether it is a glob pattern type filter.
    fn is_pattern(&self) -> bool {
        matches!(self, FilterType::GlobPattern)
    }
}

impl FileFilter {
    /// Create a new file filter
    ///
    /// # Arguments
    ///
    /// * `label` - user-visible name of the file filter.
    pub fn new(label: &str) -> Self {
        Self(label.to_owned(), vec![])
    }

    /// Adds a mime type to the file filter.
    #[must_use]
    pub fn mimetype(mut self, mimetype: &str) -> Self {
        self.1.push((FilterType::MimeType, mimetype.to_owned()));
        self
    }

    /// Adds a glob pattern to the file filter.
    #[must_use]
    pub fn glob(mut self, pattern: &str) -> Self {
        self.1.push((FilterType::GlobPattern, pattern.to_owned()));
        self
    }
}

impl FileFilter {
    /// The label of the filter.
    pub fn label(&self) -> &str {
        &self.0
    }

    /// List of mimetypes filters.
    pub fn mimetype_filters(&self) -> Vec<&str> {
        self.1
            .iter()
            .filter_map(|(type_, string)| type_.is_mimetype().then_some(string.as_str()))
            .collect()
    }

    /// List of glob patterns filters.
    pub fn pattern_filters(&self) -> Vec<&str> {
        self.1
            .iter()
            .filter_map(|(type_, string)| type_.is_pattern().then_some(string.as_str()))
            .collect()
    }
}

#[derive(Clone, Serialize, Deserialize, Type, Debug)]
/// Presents the user with a choice to select from or as a checkbox.
pub struct Choice(String, String, Vec<(String, String)>, String);

impl Choice {
    /// Creates a checkbox choice.
    ///
    /// # Arguments
    ///
    /// * `id` - A unique identifier of the choice.
    /// * `label` - user-visible name of the choice.
    /// * `state` - the initial state value.
    pub fn boolean(id: &str, label: &str, state: bool) -> Self {
        Self::new(id, label, &state.to_string())
    }

    /// Creates a new choice.
    ///
    /// # Arguments
    ///
    /// * `id` - A unique identifier of the choice.
    /// * `label` - user-visible name of the choice.
    /// * `initial_selection` - the initially selected value.
    pub fn new(id: &str, label: &str, initial_selection: &str) -> Self {
        Self(
            id.to_owned(),
            label.to_owned(),
            vec![],
            initial_selection.to_owned(),
        )
    }

    /// Adds a (key, value) as a choice.
    #[must_use]
    pub fn insert(mut self, key: &str, value: &str) -> Self {
        self.2.push((key.to_owned(), value.to_owned()));
        self
    }

    /// The choice's unique id
    pub fn id(&self) -> &str {
        &self.0
    }

    /// The user visible label of the choice.
    pub fn label(&self) -> &str {
        &self.1
    }

    /// Pairs of choices.
    pub fn pairs(&self) -> Vec<(&str, &str)> {
        self.2
            .iter()
            .map(|(x, y)| (x.as_str(), y.as_str()))
            .collect::<Vec<_>>()
    }

    /// The initially selected value.
    pub fn initial_selection(&self) -> &str {
        &self.3
    }
}

/// Options for opening a file.
#[derive(Serialize, Deserialize, Type, Debug, Default)]
#[zvariant(signature = "dict")]
pub struct OpenFileOptions {
    #[serde(with = "as_value", skip_deserializing)]
    handle_token: HandleToken,
    #[serde(default, with = "optional", skip_serializing_if = "Option::is_none")]
    accept_label: Option<String>,
    #[serde(default, with = "optional", skip_serializing_if = "Option::is_none")]
    modal: Option<bool>,
    #[serde(default, with = "optional", skip_serializing_if = "Option::is_none")]
    multiple: Option<bool>,
    #[serde(default, with = "optional", skip_serializing_if = "Option::is_none")]
    directory: Option<bool>,
    #[serde(default, with = "as_value", skip_serializing_if = "Vec::is_empty")]
    filters: Vec<FileFilter>,
    #[serde(default, with = "optional", skip_serializing_if = "Option::is_none")]
    current_filter: Option<FileFilter>,
    #[serde(default, with = "as_value", skip_serializing_if = "Vec::is_empty")]
    choices: Vec<Choice>,
    #[serde(default, with = "optional", skip_serializing_if = "Option::is_none")]
    current_folder: Option<FilePath>,
}

impl OpenFileOptions {
    /// Sets the accept label.
    #[must_use]
    pub fn set_accept_label<'a>(mut self, accept_label: impl Into<Option<&'a str>>) -> Self {
        self.accept_label = accept_label.into().map(ToOwned::to_owned);
        self
    }

    /// Gets the accept label.
    #[cfg(feature = "backend")]
    pub fn accept_label(&self) -> Option<&str> {
        self.accept_label.as_deref()
    }

    /// Sets whether the dialog should be modal.
    #[must_use]
    pub fn set_modal(mut self, modal: impl Into<Option<bool>>) -> Self {
        self.modal = modal.into();
        self
    }

    /// Gets whether the dialog should be modal.
    #[cfg(feature = "backend")]
    pub fn modal(&self) -> Option<bool> {
        self.modal
    }

    /// Sets whether multiple files can be selected.
    #[must_use]
    pub fn set_multiple(mut self, multiple: impl Into<Option<bool>>) -> Self {
        self.multiple = multiple.into();
        self
    }

    /// Gets whether multiple files can be selected.
    #[cfg(feature = "backend")]
    pub fn multiple(&self) -> Option<bool> {
        self.multiple
    }

    /// Sets whether to select directories instead of files.
    #[must_use]
    pub fn set_directory(mut self, directory: impl Into<Option<bool>>) -> Self {
        self.directory = directory.into();
        self
    }

    /// Gets whether to select directories instead of files.
    #[cfg(feature = "backend")]
    pub fn directory(&self) -> Option<bool> {
        self.directory
    }

    /// Sets the file filters.
    #[must_use]
    pub fn set_filters(mut self, filters: impl IntoIterator<Item = FileFilter>) -> Self {
        self.filters = filters.into_iter().collect();
        self
    }

    /// Gets the file filters.
    #[cfg(feature = "backend")]
    pub fn filters(&self) -> &[FileFilter] {
        &self.filters
    }

    /// Sets the current filter.
    #[must_use]
    pub fn set_current_filter(mut self, current_filter: impl Into<Option<FileFilter>>) -> Self {
        self.current_filter = current_filter.into();
        self
    }

    /// Gets the current filter.
    #[cfg(feature = "backend")]
    pub fn current_filter(&self) -> Option<&FileFilter> {
        self.current_filter.as_ref()
    }

    /// Sets the choices.
    #[must_use]
    pub fn set_choices(mut self, choices: impl IntoIterator<Item = Choice>) -> Self {
        self.choices = choices.into_iter().collect();
        self
    }

    /// Gets the choices.
    #[cfg(feature = "backend")]
    pub fn choices(&self) -> &[Choice] {
        &self.choices
    }

    /// Sets the current folder.
    #[must_use]
    pub fn set_current_folder(mut self, current_folder: impl Into<Option<FilePath>>) -> Self {
        self.current_folder = current_folder.into();
        self
    }

    /// Gets the current folder.
    #[cfg(feature = "backend")]
    pub fn current_folder(&self) -> Option<&FilePath> {
        self.current_folder.as_ref()
    }
}

/// Options for saving a file.
#[derive(Serialize, Deserialize, Type, Debug, Default)]
#[zvariant(signature = "dict")]
pub struct SaveFileOptions {
    #[serde(with = "as_value", skip_deserializing)]
    handle_token: HandleToken,
    #[serde(default, with = "optional", skip_serializing_if = "Option::is_none")]
    accept_label: Option<String>,
    #[serde(default, with = "optional", skip_serializing_if = "Option::is_none")]
    modal: Option<bool>,
    #[serde(default, with = "optional", skip_serializing_if = "Option::is_none")]
    current_name: Option<String>,
    #[serde(default, with = "optional", skip_serializing_if = "Option::is_none")]
    current_folder: Option<FilePath>,
    #[serde(default, with = "optional", skip_serializing_if = "Option::is_none")]
    current_file: Option<FilePath>,
    #[serde(default, with = "as_value", skip_serializing_if = "Vec::is_empty")]
    filters: Vec<FileFilter>,
    #[serde(default, with = "optional", skip_serializing_if = "Option::is_none")]
    current_filter: Option<FileFilter>,
    #[serde(default, with = "as_value", skip_serializing_if = "Vec::is_empty")]
    choices: Vec<Choice>,
}

impl SaveFileOptions {
    /// Sets the accept label.
    #[must_use]
    pub fn set_accept_label<'a>(mut self, accept_label: impl Into<Option<&'a str>>) -> Self {
        self.accept_label = accept_label.into().map(ToOwned::to_owned);
        self
    }

    /// Gets the accept label.
    #[cfg(feature = "backend")]
    pub fn accept_label(&self) -> Option<&str> {
        self.accept_label.as_deref()
    }

    /// Sets whether the dialog should be modal.
    #[must_use]
    pub fn set_modal(mut self, modal: impl Into<Option<bool>>) -> Self {
        self.modal = modal.into();
        self
    }

    /// Gets whether the dialog should be modal.
    #[cfg(feature = "backend")]
    pub fn modal(&self) -> Option<bool> {
        self.modal
    }

    /// Sets the current name.
    #[must_use]
    pub fn set_current_name<'a>(mut self, current_name: impl Into<Option<&'a str>>) -> Self {
        self.current_name = current_name.into().map(ToOwned::to_owned);
        self
    }

    /// Gets the current name.
    #[cfg(feature = "backend")]
    pub fn current_name(&self) -> Option<&str> {
        self.current_name.as_deref()
    }

    /// Sets the current folder.
    #[must_use]
    pub fn set_current_folder(mut self, current_folder: impl Into<Option<FilePath>>) -> Self {
        self.current_folder = current_folder.into();
        self
    }

    /// Gets the current folder.
    #[cfg(feature = "backend")]
    pub fn current_folder(&self) -> Option<&FilePath> {
        self.current_folder.as_ref()
    }

    /// Sets the current file.
    #[must_use]
    pub fn set_current_file(mut self, current_file: impl Into<Option<FilePath>>) -> Self {
        self.current_file = current_file.into();
        self
    }

    /// Gets the current file.
    #[cfg(feature = "backend")]
    pub fn current_file(&self) -> Option<&FilePath> {
        self.current_file.as_ref()
    }

    /// Sets the file filters.
    #[must_use]
    pub fn set_filters(mut self, filters: impl IntoIterator<Item = FileFilter>) -> Self {
        self.filters = filters.into_iter().collect();
        self
    }

    /// Gets the file filters.
    #[cfg(feature = "backend")]
    pub fn filters(&self) -> &[FileFilter] {
        &self.filters
    }

    /// Sets the current filter.
    #[must_use]
    pub fn set_current_filter(mut self, current_filter: impl Into<Option<FileFilter>>) -> Self {
        self.current_filter = current_filter.into();
        self
    }

    /// Gets the current filter.
    #[cfg(feature = "backend")]
    pub fn current_filter(&self) -> Option<&FileFilter> {
        self.current_filter.as_ref()
    }

    /// Sets the choices.
    #[must_use]
    pub fn set_choices(mut self, choices: impl IntoIterator<Item = Choice>) -> Self {
        self.choices = choices.into_iter().collect();
        self
    }

    /// Gets the choices.
    #[cfg(feature = "backend")]
    pub fn choices(&self) -> &[Choice] {
        &self.choices
    }
}

/// Options for saving multiple files.
#[derive(Serialize, Deserialize, Type, Debug, Default)]
#[zvariant(signature = "dict")]
pub struct SaveFilesOptions {
    #[serde(with = "as_value", skip_deserializing)]
    handle_token: HandleToken,
    #[serde(default, with = "optional", skip_serializing_if = "Option::is_none")]
    accept_label: Option<String>,
    #[serde(default, with = "optional", skip_serializing_if = "Option::is_none")]
    modal: Option<bool>,
    #[serde(default, with = "as_value", skip_serializing_if = "Vec::is_empty")]
    choices: Vec<Choice>,
    #[serde(default, with = "optional", skip_serializing_if = "Option::is_none")]
    current_folder: Option<FilePath>,
    #[serde(default, with = "as_value", skip_serializing_if = "Vec::is_empty")]
    files: Vec<FilePath>,
}

impl SaveFilesOptions {
    /// Sets the accept label.
    #[must_use]
    pub fn set_accept_label<'a>(mut self, accept_label: impl Into<Option<&'a str>>) -> Self {
        self.accept_label = accept_label.into().map(ToOwned::to_owned);
        self
    }

    /// Gets the accept label.
    #[cfg(feature = "backend")]
    pub fn accept_label(&self) -> Option<&str> {
        self.accept_label.as_deref()
    }

    /// Sets whether the dialog should be modal.
    #[must_use]
    pub fn set_modal(mut self, modal: impl Into<Option<bool>>) -> Self {
        self.modal = modal.into();
        self
    }

    /// Gets whether the dialog should be modal.
    #[cfg(feature = "backend")]
    pub fn modal(&self) -> Option<bool> {
        self.modal
    }

    /// Sets the choices.
    #[must_use]
    pub fn set_choices(mut self, choices: impl IntoIterator<Item = Choice>) -> Self {
        self.choices = choices.into_iter().collect();
        self
    }

    /// Gets the choices.
    #[cfg(feature = "backend")]
    pub fn choices(&self) -> &[Choice] {
        &self.choices
    }

    /// Sets the current folder.
    #[must_use]
    pub fn set_current_folder(mut self, current_folder: impl Into<Option<FilePath>>) -> Self {
        self.current_folder = current_folder.into();
        self
    }

    /// Gets the current folder.
    #[cfg(feature = "backend")]
    pub fn current_folder(&self) -> Option<&FilePath> {
        self.current_folder.as_ref()
    }

    /// Sets the files.
    #[must_use]
    pub fn set_files(mut self, files: impl IntoIterator<Item = FilePath>) -> Self {
        self.files = files.into_iter().collect();
        self
    }

    /// Gets the files.
    #[cfg(feature = "backend")]
    pub fn files(&self) -> &[FilePath] {
        &self.files
    }
}

#[derive(Serialize, Deserialize, Type, Debug, Default)]
/// A response of [`OpenFileRequest`], [`SaveFileRequest`] or
/// [`SaveFilesRequest`].
#[zvariant(signature = "dict")]
pub struct SelectedFiles {
    #[serde(default, with = "as_value", skip_serializing_if = "Vec::is_empty")]
    uris: Vec<Uri>,
    #[serde(default, with = "as_value", skip_serializing_if = "Vec::is_empty")]
    choices: Vec<(String, String)>,
    // Backend-only fields
    /// Not relevant for SaveFiles
    #[serde(
        default,
        with = "optional",
        skip_serializing_if = "Option::is_none",
        skip_deserializing
    )]
    current_filter: Option<FileFilter>,
    /// Only relevant for OpenFile
    #[serde(
        default,
        with = "optional",
        skip_serializing_if = "Option::is_none",
        skip_deserializing
    )]
    writable: Option<bool>,
}

impl SelectedFiles {
    /// Start an open file request.
    pub fn open_file() -> OpenFileRequest {
        OpenFileRequest::default()
    }

    /// Start a save file request.
    pub fn save_file() -> SaveFileRequest {
        SaveFileRequest::default()
    }

    /// Start a save files request.
    pub fn save_files() -> SaveFilesRequest {
        SaveFilesRequest::default()
    }

    /// The selected files uris.
    pub fn uris(&self) -> &[Uri] {
        self.uris.as_slice()
    }

    /// The selected value of each choice as a tuple of (key, value)
    pub fn choices(&self) -> &[(String, String)] {
        &self.choices
    }

    /// Adds a URI to the selected files.
    #[cfg(feature = "backend")]
    pub fn uri(mut self, value: Uri) -> Self {
        self.uris.push(value);
        self
    }

    /// Adds a choice to the selected files.
    #[cfg(feature = "backend")]
    pub fn choice(mut self, choice_key: &str, choice_value: &str) -> Self {
        self.choices
            .push((choice_key.to_owned(), choice_value.to_owned()));
        self
    }

    /// Sets the current filter.
    #[cfg(feature = "backend")]
    pub fn current_filter(mut self, value: impl Into<Option<FileFilter>>) -> Self {
        self.current_filter = value.into();
        self
    }

    /// Sets whether the file is writable.
    #[cfg(feature = "backend")]
    pub fn writable(mut self, value: impl Into<Option<bool>>) -> Self {
        self.writable = value.into();
        self
    }
}

/// Wrapper of the DBus interface: [`org.freedesktop.portal.FileChooser`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.FileChooser.html).
#[doc(alias = "org.freedesktop.portal.FileChooser")]
pub struct FileChooserProxy(Proxy<'static>);

impl FileChooserProxy {
    /// Create a new instance of [`FileChooserProxy`].
    pub async fn new() -> Result<Self, Error> {
        let proxy = Proxy::new_desktop("org.freedesktop.portal.FileChooser").await?;
        Ok(Self(proxy))
    }

    /// Create a new instance of [`FileChooserProxy`] with a specific
    /// connection.
    pub async fn with_connection(connection: zbus::Connection) -> Result<Self, Error> {
        let proxy =
            Proxy::new_desktop_with_connection(connection, "org.freedesktop.portal.FileChooser")
                .await?;
        Ok(Self(proxy))
    }

    /// Returns the portal interface version.
    pub fn version(&self) -> u32 {
        self.0.version()
    }

    /// Asks to open one or more files.
    #[doc(alias = "OpenFile")]
    pub async fn open_file(
        &self,
        identifier: Option<&WindowIdentifier>,
        title: &str,
        options: OpenFileOptions,
    ) -> Result<Request<SelectedFiles>, Error> {
        let identifier = Optional::from(identifier);
        self.0
            .request(
                &options.handle_token,
                "OpenFile",
                &(identifier, title, &options),
            )
            .await
    }

    /// Asks for a location to save a file.
    #[doc(alias = "SaveFile")]
    pub async fn save_file(
        &self,
        identifier: Option<&WindowIdentifier>,
        title: &str,
        options: SaveFileOptions,
    ) -> Result<Request<SelectedFiles>, Error> {
        let identifier = Optional::from(identifier);
        self.0
            .request(
                &options.handle_token,
                "SaveFile",
                &(identifier, title, &options),
            )
            .await
    }

    /// Asks for a location to save one or more files.
    #[doc(alias = "SaveFiles")]
    pub async fn save_files(
        &self,
        identifier: Option<&WindowIdentifier>,
        title: &str,
        options: SaveFilesOptions,
    ) -> Result<Request<SelectedFiles>, Error> {
        let identifier = Optional::from(identifier);
        self.0
            .request(
                &options.handle_token,
                "SaveFiles",
                &(identifier, title, &options),
            )
            .await
    }
}

impl std::ops::Deref for FileChooserProxy {
    type Target = zbus::Proxy<'static>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Default)]
#[doc(alias = "xdp_portal_open_file")]
/// A [builder-pattern] type to open a file.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
pub struct OpenFileRequest {
    identifier: Option<WindowIdentifier>,
    title: String,
    options: OpenFileOptions,
    connection: Option<zbus::Connection>,
}

impl OpenFileRequest {
    #[must_use]
    /// Sets a window identifier.
    pub fn identifier(mut self, identifier: impl Into<Option<WindowIdentifier>>) -> Self {
        self.identifier = identifier.into();
        self
    }

    /// Sets a title for the file chooser dialog.
    #[must_use]
    pub fn title<'a>(mut self, title: impl Into<Option<&'a str>>) -> Self {
        self.title = title.into().map(ToOwned::to_owned).unwrap_or_default();
        self
    }

    /// Sets a user-visible string to the "accept" button.
    #[must_use]
    pub fn accept_label<'a>(mut self, accept_label: impl Into<Option<&'a str>>) -> Self {
        self.options.accept_label = accept_label.into().map(ToOwned::to_owned);
        self
    }

    /// Sets whether the dialog should be a modal.
    #[must_use]
    pub fn modal(mut self, modal: impl Into<Option<bool>>) -> Self {
        self.options.modal = modal.into();
        self
    }

    /// Sets whether to allow multiple files selection.
    #[must_use]
    pub fn multiple(mut self, multiple: impl Into<Option<bool>>) -> Self {
        self.options.multiple = multiple.into();
        self
    }

    /// Sets whether to select directories or not.
    #[must_use]
    pub fn directory(mut self, directory: impl Into<Option<bool>>) -> Self {
        self.options.directory = directory.into();
        self
    }

    /// Adds a files filter.
    #[must_use]
    pub fn filter(mut self, filter: FileFilter) -> Self {
        self.options.filters.push(filter);
        self
    }

    #[must_use]
    /// Adds a list of files filters.
    pub fn filters(mut self, filters: impl IntoIterator<Item = FileFilter>) -> Self {
        self.options.filters = filters.into_iter().collect();
        self
    }

    /// Specifies the default filter.
    #[must_use]
    pub fn current_filter(mut self, current_filter: impl Into<Option<FileFilter>>) -> Self {
        self.options.current_filter = current_filter.into();
        self
    }

    /// Adds a choice.
    #[must_use]
    pub fn choice(mut self, choice: Choice) -> Self {
        self.options.choices.push(choice);
        self
    }

    #[must_use]
    /// Adds a list of choices.
    pub fn choices(mut self, choices: impl IntoIterator<Item = Choice>) -> Self {
        self.options.choices = choices.into_iter().collect();
        self
    }

    /// Specifies the current folder path.
    pub fn current_folder<P: AsRef<Path>>(
        mut self,
        current_folder: impl Into<Option<P>>,
    ) -> Result<Self, crate::Error> {
        self.options.current_folder = current_folder
            .into()
            .map(|c| FilePath::new(c))
            .transpose()?;
        Ok(self)
    }

    #[must_use]
    /// Sets a connection to use other than the internal one.
    pub fn connection(mut self, connection: Option<zbus::Connection>) -> Self {
        self.connection = connection;
        self
    }

    /// Send the request.
    pub async fn send(self) -> Result<Request<SelectedFiles>, Error> {
        let proxy = if let Some(connection) = self.connection {
            FileChooserProxy::with_connection(connection).await?
        } else {
            FileChooserProxy::new().await?
        };
        proxy
            .open_file(self.identifier.as_ref(), &self.title, self.options)
            .await
    }
}

#[derive(Debug, Default)]
#[doc(alias = "xdp_portal_save_files")]
/// A [builder-pattern] type to save multiple files.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
pub struct SaveFilesRequest {
    identifier: Option<WindowIdentifier>,
    title: String,
    options: SaveFilesOptions,
    connection: Option<zbus::Connection>,
}

impl SaveFilesRequest {
    #[must_use]
    /// Sets a window identifier.
    pub fn identifier(mut self, identifier: impl Into<Option<WindowIdentifier>>) -> Self {
        self.identifier = identifier.into();
        self
    }

    /// Sets a title for the file chooser dialog.
    #[must_use]
    pub fn title<'a>(mut self, title: impl Into<Option<&'a str>>) -> Self {
        self.title = title.into().map(ToOwned::to_owned).unwrap_or_default();
        self
    }

    /// Sets a user-visible string to the "accept" button.
    #[must_use]
    pub fn accept_label<'a>(mut self, accept_label: impl Into<Option<&'a str>>) -> Self {
        self.options.accept_label = accept_label.into().map(ToOwned::to_owned);
        self
    }

    /// Sets whether the dialog should be a modal.
    #[must_use]
    pub fn modal(mut self, modal: impl Into<Option<bool>>) -> Self {
        self.options.modal = modal.into();
        self
    }

    /// Adds a choice.
    #[must_use]
    pub fn choice(mut self, choice: Choice) -> Self {
        self.options.choices.push(choice);
        self
    }

    #[must_use]
    /// Adds a list of choices.
    pub fn choices(mut self, choices: impl IntoIterator<Item = Choice>) -> Self {
        self.options.choices = choices.into_iter().collect();
        self
    }

    /// Specifies the current folder path.
    pub fn current_folder<P: AsRef<Path>>(
        mut self,
        current_folder: impl Into<Option<P>>,
    ) -> Result<Self, crate::Error> {
        self.options.current_folder = current_folder
            .into()
            .map(|c| FilePath::new(c))
            .transpose()?;
        Ok(self)
    }

    /// Sets a list of files to save.
    pub fn files<P: IntoIterator<Item = impl AsRef<Path>>>(
        mut self,
        files: impl Into<Option<P>>,
    ) -> Result<Self, crate::Error> {
        if let Some(f) = files.into() {
            self.options.files = f
                .into_iter()
                .map(|s| FilePath::new(s))
                .collect::<Result<Vec<_>, _>>()?;
        }
        Ok(self)
    }

    #[must_use]
    /// Sets a connection to use other than the internal one.
    pub fn connection(mut self, connection: Option<zbus::Connection>) -> Self {
        self.connection = connection;
        self
    }

    /// Send the request.
    pub async fn send(self) -> Result<Request<SelectedFiles>, Error> {
        let proxy = if let Some(connection) = self.connection {
            FileChooserProxy::with_connection(connection).await?
        } else {
            FileChooserProxy::new().await?
        };
        proxy
            .save_files(self.identifier.as_ref(), &self.title, self.options)
            .await
    }
}

#[derive(Debug, Default)]
#[doc(alias = "xdp_portal_save_file")]
/// A [builder-pattern] type to save a file.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
pub struct SaveFileRequest {
    identifier: Option<WindowIdentifier>,
    title: String,
    options: SaveFileOptions,
    connection: Option<zbus::Connection>,
}

impl SaveFileRequest {
    #[must_use]
    /// Sets a window identifier.
    pub fn identifier(mut self, identifier: impl Into<Option<WindowIdentifier>>) -> Self {
        self.identifier = identifier.into();
        self
    }

    /// Sets a title for the file chooser dialog.
    #[must_use]
    pub fn title<'a>(mut self, title: impl Into<Option<&'a str>>) -> Self {
        self.title = title.into().map(ToOwned::to_owned).unwrap_or_default();
        self
    }

    /// Sets a user-visible string to the "accept" button.
    #[must_use]
    pub fn accept_label<'a>(mut self, accept_label: impl Into<Option<&'a str>>) -> Self {
        self.options.accept_label = accept_label.into().map(ToOwned::to_owned);
        self
    }

    /// Sets whether the dialog should be a modal.
    #[must_use]
    pub fn modal(mut self, modal: impl Into<Option<bool>>) -> Self {
        self.options.modal = modal.into();
        self
    }

    /// Sets the current file name.
    #[must_use]
    pub fn current_name<'a>(mut self, current_name: impl Into<Option<&'a str>>) -> Self {
        self.options.current_name = current_name.into().map(ToOwned::to_owned);
        self
    }

    /// Sets the current folder.
    pub fn current_folder<P: AsRef<Path>>(
        mut self,
        current_folder: impl Into<Option<P>>,
    ) -> Result<Self, crate::Error> {
        self.options.current_folder = current_folder
            .into()
            .map(|c| FilePath::new(c))
            .transpose()?;
        Ok(self)
    }

    /// Sets the absolute path of the file.
    pub fn current_file<P: AsRef<Path>>(
        mut self,
        current_file: impl Into<Option<P>>,
    ) -> Result<Self, crate::Error> {
        self.options.current_file = current_file.into().map(|c| FilePath::new(c)).transpose()?;
        Ok(self)
    }

    /// Adds a files filter.
    #[must_use]
    pub fn filter(mut self, filter: FileFilter) -> Self {
        self.options.filters.push(filter);
        self
    }

    #[must_use]
    /// Adds a list of files filters.
    pub fn filters(mut self, filters: impl IntoIterator<Item = FileFilter>) -> Self {
        self.options.filters = filters.into_iter().collect();
        self
    }

    /// Sets the default filter.
    #[must_use]
    pub fn current_filter(mut self, current_filter: impl Into<Option<FileFilter>>) -> Self {
        self.options.current_filter = current_filter.into();
        self
    }

    /// Adds a choice.
    #[must_use]
    pub fn choice(mut self, choice: Choice) -> Self {
        self.options.choices.push(choice);
        self
    }

    #[must_use]
    /// Adds a list of choices.
    pub fn choices(mut self, choices: impl IntoIterator<Item = Choice>) -> Self {
        self.options.choices = choices.into_iter().collect();
        self
    }

    #[must_use]
    /// Sets a connection to use other than the internal one.
    pub fn connection(mut self, connection: Option<zbus::Connection>) -> Self {
        self.connection = connection;
        self
    }

    /// Send the request.
    pub async fn send(self) -> Result<Request<SelectedFiles>, Error> {
        let proxy = if let Some(connection) = self.connection {
            FileChooserProxy::with_connection(connection).await?
        } else {
            FileChooserProxy::new().await?
        };
        proxy
            .save_file(self.identifier.as_ref(), &self.title, self.options)
            .await
    }
}
