//! This crate provides functions that allow moving files to the operating system's Recycle Bin or
//! Trash, or the equivalent.
//!
//! Furthermore on Linux and on Windows additional functions are available from the `os_limited`
//! module.
//!
//! ### Potential UB on Linux and FreeBSD
//!
//! When querying information about mount points, non-threadsafe versions of `libc::getmnt(info|ent)` are
//! used which can cause UB if another thread calls into the same function, _probably_ only if the mountpoints
//! changed as well.
//!
//! To neutralize the issue, the respective function in this crate has been made thread-safe with a Mutex.
//!
//! **If your crate calls into the aforementioned methods directly or indirectly from other threads,
//! rather not use this crate.**
//!
//! As the handling of UB is clearly a trade-off and certainly goes against the zero-chance-of-UB goal
//! of the Rust community, please interact with us [in the tracking issue](https://github.com/Byron/trash-rs/issues/42)
//! to help find a more permanent solution.
//!
//! ### Notes on the Linux implementation
//!
//! This library implements version 1.0 of the [Freedesktop.org
//! Trash](https://specifications.freedesktop.org/trash-spec/trashspec-1.0.html) specification and
//! aims to match the behaviour of Ubuntu 18.04 GNOME in cases of ambiguity. Most -if not all- Linux
//! distributions that ship with a desktop environment follow this specification. For example
//! GNOME, KDE, and XFCE all use this convention. This crate blindly assumes that the Linux
//! distribution it runs on, follows this specification.
//!

use std::ffi::OsString;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

use std::fmt;
use std::{env::current_dir, error};

use log::trace;

#[cfg(test)]
pub mod tests;

#[cfg(target_os = "windows")]
#[path = "windows.rs"]
mod platform;

#[cfg(all(unix, not(target_os = "macos"), not(target_os = "ios"), not(target_os = "android")))]
#[path = "freedesktop.rs"]
mod platform;

#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "macos")]
use macos as platform;

pub const DEFAULT_TRASH_CTX: TrashContext = TrashContext::new();

/// A collection of preferences for trash operations.
#[derive(Clone, Default, Debug)]
pub struct TrashContext {
    #[cfg_attr(not(target_os = "macos"), allow(dead_code))]
    platform_specific: platform::PlatformTrashContext,
}
impl TrashContext {
    pub const fn new() -> Self {
        Self { platform_specific: platform::PlatformTrashContext::new() }
    }

    /// Removes a single file or directory.
    ///
    /// When a symbolic link is provided to this function, the symbolic link will be removed and the link
    /// target will be kept intact.
    ///
    /// # Example
    ///
    /// ```
    /// use std::fs::File;
    /// use trash::delete;
    /// File::create_new("delete_me").unwrap();
    /// trash::delete("delete_me").unwrap();
    /// assert!(File::open("delete_me").is_err());
    /// ```
    pub fn delete<T: AsRef<Path>>(&self, path: T) -> Result<(), Error> {
        self.delete_all(&[path])
    }

    /// Removes all files/directories specified by the collection of paths provided as an argument.
    ///
    /// When a symbolic link is provided to this function, the symbolic link will be removed and the link
    /// target will be kept intact.
    ///
    /// # Example
    ///
    /// ```
    /// use std::fs::File;
    /// use trash::delete_all;
    /// File::create_new("delete_me_1").unwrap();
    /// File::create_new("delete_me_2").unwrap();
    /// delete_all(&["delete_me_1", "delete_me_2"]).unwrap();
    /// assert!(File::open("delete_me_1").is_err());
    /// assert!(File::open("delete_me_2").is_err());
    /// ```
    pub fn delete_all<I, T>(&self, paths: I) -> Result<(), Error>
    where
        I: IntoIterator<Item = T>,
        T: AsRef<Path>,
    {
        trace!("Starting canonicalize_paths");
        let full_paths = canonicalize_paths(paths)?;
        trace!("Finished canonicalize_paths");

        self.delete_all_canonicalized(full_paths, false).map(|_| ())
    }

    /// Same as `delete`, but returns a `TrashItem` describing where the file
    /// ended up in trash.
    pub fn delete_with_info<T: AsRef<Path>>(&self, path: T) -> Result<TrashItem, Error> {
        self.delete_all_with_info(&[path])?
            .pop()
            .ok_or(Error::Unknown { description: "delete_with_info did not return trash item information".into() })
    }

    /// Same as `delete_all` but returns `TrashItem`s describing where files
    /// ended up in the trash.
    pub fn delete_all_with_info<I, T>(&self, paths: I) -> Result<Vec<TrashItem>, Error>
    where
        I: IntoIterator<Item = T>,
        T: AsRef<Path>,
    {
        let full_paths = canonicalize_paths(paths)?;

        self.delete_all_canonicalized(full_paths, true).and_then(|items| {
            items.ok_or(Error::Unknown {
                description: "delete_all_with_info did not return trash item information".into(),
            })
        })
    }
}

/// Convenience method for `DEFAULT_TRASH_CTX.delete()`.
///
/// See: [`TrashContext::delete`](TrashContext::delete)
pub fn delete<T: AsRef<Path>>(path: T) -> Result<(), Error> {
    DEFAULT_TRASH_CTX.delete(path)
}

/// Convenience method for `DEFAULT_TRASH_CTX.delete_with_info()`.
///
/// See: [`TrashContext::delete_with_info`](TrashContext::delete_with_info)
pub fn delete_with_info<T: AsRef<Path>>(path: T) -> Result<TrashItem, Error> {
    DEFAULT_TRASH_CTX.delete_with_info(path)
}

/// Convenience method for `DEFAULT_TRASH_CTX.delete_all()`.
///
/// See: [`TrashContext::delete_all`](TrashContext::delete_all)
pub fn delete_all<I, T>(paths: I) -> Result<(), Error>
where
    I: IntoIterator<Item = T>,
    T: AsRef<Path>,
{
    DEFAULT_TRASH_CTX.delete_all(paths)
}

/// Convenience method for `DEFAULT_TRASH_CTX.delete_all_with_info()`.
///
/// See: [`TrashContext::delete_all_with_info`](TrashContext::delete_all_with_info)
pub fn delete_all_with_info<I, T>(paths: I) -> Result<Vec<TrashItem>, Error>
where
    I: IntoIterator<Item = T>,
    T: AsRef<Path>,
{
    DEFAULT_TRASH_CTX.delete_all_with_info(paths)
}

/// Provides information about an error.
#[derive(Debug)]
pub enum Error {
    Unknown {
        description: String,
    },

    Os {
        code: i32,
        description: String,
    },

    /// Error coming from file system
    #[cfg(all(unix, not(target_os = "ios"), not(target_os = "android")))]
    FileSystem {
        path: PathBuf,
        source: std::io::Error,
    },

    /// The file got removed from trash. Either the trash was emptied or this
    /// specific file was removed.
    NoLongerInTrash,

    /// One of the target items was a root folder.
    /// If a list of items are requested to be removed by a single function call (e.g. `delete_all`)
    /// and this error is returned, then it's guaranteed that none of the items is removed.
    TargetedRoot,

    /// The `target` does not exist or the process has insufficient permissions to access it.
    CouldNotAccess {
        target: String,
    },

    /// Error while canonicalizing path.
    CanonicalizePath {
        /// Path that triggered the error.
        original: PathBuf,
    },

    /// Error while converting an [`OsString`] to a [`String`].
    ///
    /// This may also happen when converting a [`Path`] or [`PathBuf`] to an [`OsString`].
    ConvertOsString {
        /// The string that was attempted to be converted.
        original: OsString,
    },

    /// This kind of error happens when a trash item's original parent already contains an item with
    /// the same name and type (file or folder). In this case an error is produced and the
    /// restoration of the files is halted meaning that there may be files that could be restored
    /// but were left in the trash due to the error.
    ///
    /// One should not assume any relationship between the order that the items were supplied and
    /// the list of remaining items. That is to say, it may be that the item that collided was in
    /// the middle of the provided list but the remaining items' list contains all the provided
    /// items.
    ///
    /// `path`: The path of the file that's blocking the trash item from being restored.
    ///
    /// `remaining_items`: All items that were not restored in the order they were provided,
    /// starting with the item that triggered the error.
    RestoreCollision {
        path: PathBuf,
        remaining_items: Vec<TrashItem>,
    },

    /// This sort of error is returned when multiple items with the same `original_path` were
    /// requested to be restored. These items are referred to as twins here. If there are twins
    /// among the items, then none of the items are restored.
    ///
    /// `path`: The `original_path` of the twins.
    ///
    /// `items`: The complete list of items that were handed over to the `restore_all` function.
    RestoreTwins {
        path: PathBuf,
        items: Vec<TrashItem>,
    },
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error during a `trash` operation: {self:?}")
    }
}
impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            #[cfg(all(unix, not(target_os = "macos"), not(target_os = "ios"), not(target_os = "android")))]
            Self::FileSystem { path: _, source: e } => e.source(),
            _ => None,
        }
    }
}
pub fn into_unknown<E: std::fmt::Display>(err: E) -> Error {
    Error::Unknown { description: format!("{err}") }
}

#[cfg(all(unix, not(target_os = "ios"), not(target_os = "android")))]
pub fn fs_error(path: impl Into<PathBuf>, source: std::io::Error) -> Error {
    Error::FileSystem { path: path.into(), source }
}

pub(crate) fn canonicalize_paths<I, T>(paths: I) -> Result<Vec<PathBuf>, Error>
where
    I: IntoIterator<Item = T>,
    T: AsRef<Path>,
{
    let paths = paths.into_iter();
    paths
        .map(|x| {
            let target_ref = x.as_ref();
            if target_ref.as_os_str().is_empty() {
                return Err(Error::CanonicalizePath { original: target_ref.to_owned() });
            }
            let target = if target_ref.is_relative() {
                let curr_dir = current_dir()
                    .map_err(|_| Error::CouldNotAccess { target: "[Current working directory]".into() })?;
                curr_dir.join(target_ref)
            } else {
                target_ref.to_owned()
            };
            let parent = target.parent().ok_or(Error::TargetedRoot)?;
            let canonical_parent =
                parent.canonicalize().map_err(|_| Error::CanonicalizePath { original: parent.to_owned() })?;
            if let Some(file_name) = target.file_name() {
                Ok(canonical_parent.join(file_name))
            } else {
                // `file_name` is none if the path ends with `..`
                Ok(canonical_parent)
            }
        })
        .collect::<Result<Vec<_>, _>>()
}

/// This struct holds information about a single item within the trash.
///
/// A trash item can be a file or folder or any other object that the target
/// operating system allows to put into the trash.
#[derive(Debug, Clone)]
pub struct TrashItem {
    /// A system specific identifier of the item in the trash.
    ///
    /// On Windows it is the string returned by `IShellItem::GetDisplayName`
    /// with the `SIGDN_DESKTOPABSOLUTEPARSING` flag.
    ///
    /// On Linux it is an absolute path to the `.trashinfo` file associated with
    /// the item.
    pub id: OsString,

    /// The name of the item. For example if the folder '/home/user/New Folder'
    /// was deleted, its `name` is 'New Folder'
    pub name: OsString,

    /// The path to the parent folder of this item before it was put inside the
    /// trash. For example if the folder '/home/user/New Folder' is in the
    /// trash, its `original_parent` is '/home/user'.
    ///
    /// To get the full path to the file in its original location use the
    /// `original_path` function.
    pub original_parent: PathBuf,

    /// The number of non-leap seconds elapsed between the UNIX Epoch and the
    /// moment the file was deleted.
    /// Without the "chrono" feature, this will be a negative number on linux only.
    pub time_deleted: i64,
}

impl TrashItem {
    /// Joins the `original_parent` and `name` fields to obtain the full path to
    /// the original file.
    pub fn original_path(&self) -> PathBuf {
        self.original_parent.join(&self.name)
    }
}
impl PartialEq for TrashItem {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for TrashItem {}
impl Hash for TrashItem {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

/// Size of a [`TrashItem`] in bytes or entries
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum TrashItemSize {
    /// Number of bytes in a file
    Bytes(u64),
    /// Number of entries in a directory, non-recursive
    Entries(usize),
}

impl TrashItemSize {
    /// The size of a file in bytes, if this item is a file.
    pub fn size(&self) -> Option<u64> {
        match self {
            TrashItemSize::Bytes(s) => Some(*s),
            TrashItemSize::Entries(_) => None,
        }
    }

    /// The amount of entries in the directory, if this is a directory.
    pub fn entries(&self) -> Option<usize> {
        match self {
            TrashItemSize::Bytes(_) => None,
            TrashItemSize::Entries(e) => Some(*e),
        }
    }
}

/// Metadata about a [`TrashItem`]
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct TrashItemMetadata {
    /// The size of the item, depending on whether or not it is a directory.
    pub size: TrashItemSize,
}

#[cfg(all(not(target_os = "ios"), not(target_os = "android")))]
/// Restores all the provided [`TrashItem`] to their original location.
///
/// This function consumes the provided items.
///
/// # Errors
///
/// Errors this function may return include but are not limited to the following.
///
/// It may be the case that when restoring a file or a folder, the `original_path` already has
/// a new item with the same name. When such a collision happens this function returns a
/// [`RestoreCollision`] kind of error.
///
/// If two or more of the provided items have identical `original_path`s then a
/// [`RestoreTwins`] kind of error is returned.
///
/// # Example
///
/// Basic usage:
///
/// ```
/// use std::fs::File;
/// use trash::{delete_with_info, restore_all};
///
/// let filename = "trash-restore_all-example";
/// File::create_new(filename).unwrap();
/// let item = delete_with_info(filename).unwrap();
/// restore_all([item]).unwrap();
/// std::fs::remove_file(filename).unwrap();
/// ```
///
/// Retry restoring when encountering [`RestoreCollision`] error:
///
/// ```no_run
/// use trash::{delete_with_info, restore_all};
/// use trash::Error::RestoreCollision;
///
/// let items = vec![delete_with_info("some-file").unwrap()];
/// if let Err(RestoreCollision { path, mut remaining_items }) = restore_all(items) {
///     // keep all except the one(s) that couldn't be restored
///     remaining_items.retain(|e| e.original_path() != path);
///     restore_all(remaining_items).unwrap();
/// }
/// ```
///
/// [`RestoreCollision`]: Error::RestoreCollision
/// [`RestoreTwins`]: Error::RestoreTwins
pub fn restore_all<I>(items: I) -> Result<(), Error>
where
    I: IntoIterator<Item = TrashItem>,
{
    use std::collections::HashSet;

    // Check for twins here cause that's pretty platform independent.
    struct ItemWrapper<'a>(&'a TrashItem);
    impl PartialEq for ItemWrapper<'_> {
        fn eq(&self, other: &Self) -> bool {
            self.0.original_path() == other.0.original_path()
        }
    }
    impl Eq for ItemWrapper<'_> {}
    impl Hash for ItemWrapper<'_> {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.0.original_path().hash(state);
        }
    }
    let items = items.into_iter().collect::<Vec<_>>();
    let mut item_set = HashSet::with_capacity(items.len());
    for item in items.iter() {
        if !item_set.insert(ItemWrapper(item)) {
            return Err(Error::RestoreTwins { path: item.original_path(), items });
        }
    }
    platform::restore_all(items)
}

#[cfg(any(
    target_os = "windows",
    all(unix, not(target_os = "macos"), not(target_os = "ios"), not(target_os = "android"))
))]
pub mod os_limited {
    //! This module provides functionality which is only supported on Windows and
    //! Linux or other Freedesktop Trash compliant environment.

    use std::borrow::Borrow;

    #[cfg(all(unix, not(target_os = "macos"), not(target_os = "ios"), not(target_os = "android")))]
    use std::collections::HashSet;

    use super::{platform, Error, TrashItem, TrashItemMetadata};

    /// Returns all [`TrashItem`]s that are currently in the trash.
    ///
    /// The items are in no particular order and must be sorted when any kind of ordering is required.
    ///
    /// # Example
    ///
    /// ```
    /// use trash::os_limited::list;
    /// let trash_items = list().unwrap();
    /// println!("{:#?}", trash_items);
    /// ```
    pub fn list() -> Result<Vec<TrashItem>, Error> {
        platform::list()
    }

    /// Returns whether the trash is empty or has at least one item.
    ///
    /// Unlike calling [`list`], this function short circuits without evaluating every item.
    ///
    /// # Example
    ///
    /// ```
    /// use trash::os_limited::is_empty;
    /// if is_empty().unwrap_or(true) {
    ///     println!("Trash is empty");
    /// } else {
    ///     println!("Trash contains at least one item");
    /// }
    /// ```
    pub fn is_empty() -> Result<bool, Error> {
        platform::is_empty()
    }

    /// Returns all valid trash bins on supported Unix platforms.
    ///
    /// Valid trash folders include the user's personal "home trash" as well as designated trash
    /// bins across mount points. Some, or all of these, may not exist or be invalid in some way.
    ///
    /// # Example
    ///
    /// ```
    /// # #[cfg(all(unix, not(target_os = "macos"), not(target_os = "ios"), not(target_os = "android")))] {
    /// use trash::os_limited::trash_folders;
    /// let trash_bins = trash_folders()?;
    /// println!("{trash_bins:#?}");
    /// # }
    /// # Ok::<(), trash::Error>(())
    /// ```
    #[cfg(all(unix, not(target_os = "macos"), not(target_os = "ios"), not(target_os = "android")))]
    pub fn trash_folders() -> Result<HashSet<std::path::PathBuf>, Error> {
        platform::trash_folders()
    }

    /// Returns the [`TrashItemMetadata`] for a [`TrashItem`]
    ///
    /// # Example
    ///
    /// ```
    /// use trash::os_limited::{list, metadata};
    /// let trash_items = list().unwrap();
    /// for item in trash_items {
    ///     println!("{:#?}", metadata(&item).unwrap());
    /// }
    /// ```
    pub fn metadata(item: &TrashItem) -> Result<TrashItemMetadata, Error> {
        platform::metadata(item)
    }

    /// Deletes all the provided [`TrashItem`]s permanently.
    ///
    /// This function consumes the provided items.
    ///
    /// # Example
    ///
    /// Taking items' ownership:
    ///
    /// ```
    /// use std::fs::File;
    /// use trash::{delete, os_limited::{list, purge_all}};
    ///
    /// let filename = "trash-purge_all-example-ownership";
    /// File::create_new(filename).unwrap();
    /// delete(filename).unwrap();
    /// // Collect the filtered list just so that we can make sure there's exactly one element.
    /// // There's no need to `collect` it otherwise.
    /// let selected: Vec<_> = list().unwrap().into_iter().filter(|x| x.name == filename).collect();
    /// assert_eq!(selected.len(), 1);
    /// purge_all(selected).unwrap();
    /// ```
    ///
    /// Taking items' reference:
    ///
    /// ```
    /// use std::fs::File;
    /// use trash::{delete, os_limited::{list, purge_all}};
    ///
    /// let filename = "trash-purge_all-example-reference";
    /// File::create_new(filename).unwrap();
    /// delete(filename).unwrap();
    /// let mut selected = list().unwrap();
    /// selected.retain(|x| x.name == filename);
    /// assert_eq!(selected.len(), 1);
    /// purge_all(&selected).unwrap();
    /// ```
    pub fn purge_all<I>(items: I) -> Result<(), Error>
    where
        I: IntoIterator,
        <I as IntoIterator>::Item: Borrow<TrashItem>,
    {
        platform::purge_all(items)
    }
}
