//! When working on this, there are a couple of things to test:
//! * Works in Flatpak packages (in Flatpak packages the OpenURI interface is used,
//!   because FileManager1 is not available)
//! * Works with relative file paths
//! * Works with directories (and highlights them)
//! * Weird paths work: paths with spaces, unicode characters, non-unicode characters (e.g. `"\u{01}"`)
//! * Path to non-existent file generates an error for both implementations.

use crate::OpenError;
use dbus::arg::messageitem::MessageItem;
use dbus::arg::{Append, Variant};
use dbus::blocking::Connection;
use std::fs::File;
use std::path::Path;
use std::time::Duration;
use std::{error, fmt, io};
use url::Url;

const DBUS_TIMEOUT: Duration = Duration::from_secs(5);

// We should prefer the OpenURI interface, because it correctly handles runtimes such as Flatpak.
// However, OpenURI was broken in the original version of the interface (it did not highlight the items).
// This version is still in use by some distributions, which would result in degraded functionality for some users.
// That's why we're first trying to use the FileManager1 interface, falling back to the OpenURI interface.
// Source: https://chromium-review.googlesource.com/c/chromium/src/+/3009959
pub(crate) fn reveal_with_dbus(path: &Path) -> Result<(), OpenError> {
    let connection = Connection::new_session().map_err(dbus_to_open_error)?;
    reveal_with_filemanager1(path, &connection)
        .or_else(|_| reveal_with_open_uri_portal(path, &connection))
}

fn reveal_with_filemanager1(path: &Path, connection: &Connection) -> Result<(), OpenError> {
    let uri = path_to_uri(path)?;
    let proxy = connection.with_proxy(
        "org.freedesktop.FileManager1",
        "/org/freedesktop/FileManager1",
        DBUS_TIMEOUT,
    );
    proxy
        .method_call(
            "org.freedesktop.FileManager1",
            "ShowItems",
            (vec![uri.as_str()], ""),
        )
        .map_err(dbus_to_open_error)
}

fn reveal_with_open_uri_portal(path: &Path, connection: &Connection) -> Result<(), OpenError> {
    let file = File::open(path).map_err(OpenError::Io)?;
    let proxy = connection.with_proxy(
        "org.freedesktop.portal.Desktop",
        "/org/freedesktop/portal/desktop",
        DBUS_TIMEOUT,
    );
    proxy
        .method_call(
            "org.freedesktop.portal.OpenURI",
            "OpenDirectory",
            ("", file, empty_vardict()),
        )
        .map_err(dbus_to_open_error)
}

fn empty_vardict() -> impl Append {
    dbus::arg::Dict::<&'static str, Variant<MessageItem>, _>::new(std::iter::empty())
}

fn path_to_uri(path: &Path) -> Result<Url, OpenError> {
    let path = path.canonicalize().map_err(OpenError::Io)?;
    Url::from_file_path(path).map_err(|_| uri_to_open_error())
}

fn uri_to_open_error() -> OpenError {
    OpenError::Io(io::Error::new(
        io::ErrorKind::InvalidInput,
        FilePathToUriError,
    ))
}

fn dbus_to_open_error(error: dbus::Error) -> OpenError {
    OpenError::Io(io::Error::new(io::ErrorKind::Other, error))
}

#[derive(Debug)]
struct FilePathToUriError;

impl fmt::Display for FilePathToUriError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "The given file path could not be converted to a URI")
    }
}

impl error::Error for FilePathToUriError {}
