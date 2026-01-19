//! This implementation will manage the trash according to the Freedesktop Trash specification,
//! version 1.0 found at <https://specifications.freedesktop.org/trash-spec/trashspec-1.0.html>
//!
//! Most -if not all- Linux based desktop operating systems implement the Trash according to this specification.
//! In other words: I looked, but I could not find any Linux based desktop OS that used anything else than the
//! Freedesktop Trash specification.
//!

use std::{
    borrow::{Borrow, Cow},
    collections::HashSet,
    ffi::{OsStr, OsString},
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, ErrorKind, Write},
    os::unix::{
        ffi::{OsStrExt, OsStringExt},
        fs::PermissionsExt,
    },
    path::{Component, Path, PathBuf},
};

use log::{debug, warn};

use crate::{Error, TrashContext, TrashItem, TrashItemMetadata, TrashItemSize};

type FsError = (PathBuf, std::io::Error);

#[derive(Clone, Default, Debug)]
pub struct PlatformTrashContext;
impl PlatformTrashContext {
    pub const fn new() -> Self {
        PlatformTrashContext
    }
}
impl TrashContext {
    pub(crate) fn delete_all_canonicalized(&self, full_paths: Vec<PathBuf>) -> Result<(), Error> {
        let home_trash = home_trash()?;
        let sorted_mount_points = get_sorted_mount_points()?;
        let home_topdir = home_topdir(&sorted_mount_points)?;
        debug!("The home topdir is {:?}", home_topdir);
        let uid = unsafe { libc::getuid() };
        for path in full_paths {
            debug!("Deleting {:?}", path);
            let topdir = get_first_topdir_containing_path(&path, &sorted_mount_points);
            debug!("The topdir of this file is {:?}", topdir);
            if topdir == home_topdir {
                debug!("The topdir was identical to the home topdir, so moving to the home trash.");
                // Note that the following function creates the trash folder
                // and its required subfolders in case they don't exist.
                move_to_trash(path, &home_trash, topdir).map_err(|(p, e)| fs_error(p, e))?;
            } else if topdir.to_str() == Some("/var/home") && home_topdir.to_str() == Some("/") {
                debug!("The topdir is '/var/home' but the home_topdir is '/', moving to the home trash anyway.");
                move_to_trash(path, &home_trash, topdir).map_err(|(p, e)| fs_error(p, e))?;
            } else {
                execute_on_mounted_trash_folders(uid, topdir, true, true, |trash_path| {
                    move_to_trash(&path, trash_path, topdir)
                })
                .map_err(|(p, e)| fs_error(p, e))?;
            }
        }
        Ok(())
    }
}

pub fn list() -> Result<Vec<TrashItem>, Error> {
    let EvaluatedTrashFolders { trash_folders, home_error, sorted_mount_points } = eval_trash_folders()?;

    if trash_folders.is_empty() {
        warn!("No trash folder was found. The error when looking for the 'home trash' was: {:?}", home_error);
        return Ok(vec![]);
    }
    // List all items from the set of trash folders
    let mut result = Vec::new();
    for folder in &trash_folders {
        // Read the info files for every file
        let top_dir = get_first_topdir_containing_path(folder, &sorted_mount_points);
        let info_folder = folder.join("info");
        if !info_folder.is_dir() {
            warn!("The path {:?} did not point to a directory, skipping this trash folder.", info_folder);
            continue;
        }
        let read_dir = match std::fs::read_dir(&info_folder) {
            Ok(d) => d,
            Err(e) => {
                // After all the earlier checks, it's still possible that the directory does not exist at this point (or is not readable)
                // because another process may have deleted it or modified its access rights in the meantime.
                // So let's just pring a warning and continue to the rest of the folders
                warn!("The trash info folder {:?} could not be read. Error was {:?}", info_folder, e);
                continue;
            }
        };
        #[cfg_attr(not(feature = "chrono"), allow(unused_labels))]
        'trash_item: for entry in read_dir {
            let info_entry = match entry {
                Ok(entry) => entry,
                Err(e) => {
                    // Another thread or process may have removed that entry by now
                    debug!("Tried resolving the trash info `DirEntry` but it failed with: '{}'", e);
                    continue;
                }
            };
            // Entrty should really be an info file but better safe than sorry
            let file_type = match info_entry.file_type() {
                Ok(f_type) => f_type,
                Err(e) => {
                    // Another thread or process may have removed that entry by now
                    debug!("Tried getting the file type of the trash info `DirEntry` but failed with: {}", e);
                    continue;
                }
            };
            let info_path = info_entry.path();
            if !file_type.is_file() {
                warn!("Found an item that's not a file, among the trash info files. This is unexpected. The path to the item is: '{:?}'", info_path);
                continue;
            }
            let info_file = match File::open(&info_path) {
                Ok(file) => file,
                Err(e) => {
                    // Another thread or process may have removed that entry by now
                    debug!("Tried opening the trash info '{:?}' but failed with: {}", info_path, e);
                    continue;
                }
            };
            let id = info_path.clone().into();
            let mut name = None;
            let mut original_parent: Option<PathBuf> = None;
            #[cfg_attr(not(feature = "chrono"), allow(unused_mut))]
            let mut time_deleted = None;

            let info_reader = BufReader::new(info_file);
            // Skip 1 because the first line must be "[Trash Info]"
            'info_lines: for line_result in info_reader.lines().skip(1) {
                // Another thread or process may have removed the infofile by now
                let line = if let Ok(line) = line_result {
                    line
                } else {
                    break 'info_lines;
                };
                let mut split = line.split('=');

                // Just unwraping here because the system is assumed to follow the specification.
                let key = split.next().unwrap().trim();
                let value = split.next().unwrap().trim();

                if key == "Path" {
                    let value_path = {
                        let path = Path::new(value);
                        if path.is_relative() {
                            decode_uri_path(top_dir.join(path))
                        } else {
                            decode_uri_path(path)
                        }
                    };
                    name = value_path.file_name().map(|name| name.to_owned());
                    let parent = value_path.parent().expect("Absolute path to trashed item should have a parent");
                    original_parent = Some(parent.into());
                } else if key == "DeletionDate" {
                    #[cfg(feature = "chrono")]
                    {
                        use chrono::{NaiveDateTime, TimeZone};
                        let parsed_time = NaiveDateTime::parse_from_str(value, "%Y-%m-%dT%H:%M:%S");
                        let naive_local = match parsed_time {
                            Ok(t) => t,
                            Err(e) => {
                                log::error!("Failed to parse the deletion date of the trash item {:?}. The deletion date was '{}'. Parse error was: {:?}", name, value, e);
                                continue 'trash_item;
                            }
                        };
                        let time = chrono::Local.from_local_datetime(&naive_local).earliest();
                        match time {
                            Some(time) => time_deleted = Some(time.timestamp()),
                            None => {
                                log::error!(
                                    "Failed to convert the local time to a UTC time. Local time was {:?}",
                                    naive_local
                                );
                                continue 'trash_item;
                            }
                        }
                    }
                }
            }
            if let Some(name) = name {
                if let Some(original_parent) = original_parent {
                    if time_deleted.is_none() {
                        warn!("Could not determine the deletion time of the trash item. (The `DeletionDate` field is probably missing from the info file.) The info file path is: '{:?}'", info_path);
                    }
                    result.push(TrashItem { id, name, original_parent, time_deleted: time_deleted.unwrap_or(-1) });
                } else {
                    warn!("Could not determine the original parent folder of the trash item. (The `Path` field is probably missing from the info file.) The info file path is: '{:?}'", info_path);
                }
            } else {
                warn!("Could not determine the name of the trash item. (The `Path` field is probably missing from the info file.) The info file path is: '{:?}'", info_path);
            }
        }
    }
    Ok(result)
}

pub fn is_empty() -> Result<bool, Error> {
    let trash_folders = trash_folders()?;

    if trash_folders.is_empty() {
        return Ok(true);
    }

    for folder in trash_folders {
        // We're only concerned if the trash contains any files
        // Therefore, we only need to check if the bin itself is empty
        let bin = folder.join("files");
        match bin.read_dir() {
            Ok(mut entries) => {
                if let Some(Ok(_)) = entries.next() {
                    return Ok(false);
                }
            }
            Err(e) => {
                warn!("The trash files folder {:?} could not be read. Error was {:?}", bin, e);
            }
        }
    }

    Ok(true)
}

pub fn trash_folders() -> Result<HashSet<PathBuf>, Error> {
    let EvaluatedTrashFolders { trash_folders, home_error, .. } = eval_trash_folders()?;

    if trash_folders.is_empty() {
        return match home_error {
            Some(e) => Err(e),
            None => Err(Error::Unknown {
                description: "Could not find a valid 'home trash' nor valid trashes on other mount points".into(),
            }),
        };
    }

    Ok(trash_folders)
}

struct EvaluatedTrashFolders {
    trash_folders: HashSet<PathBuf>,
    home_error: Option<Error>,
    sorted_mount_points: Vec<MountPoint>,
}

fn eval_trash_folders() -> Result<EvaluatedTrashFolders, Error> {
    let mut trash_folders = HashSet::new();
    // Get home trash folder and add it to the set of trash folders.
    // It may not exist and that's completely fine as long as there are other trash folders.
    let home_error;
    match home_trash() {
        Ok(home_trash) => {
            if !home_trash.is_dir() {
                home_error = Some(Error::Unknown {
                    description:
                        "The 'home trash' either does not exist or is not a directory (or a link pointing to a dir)"
                            .into(),
                });
            } else {
                trash_folders.insert(home_trash);
                home_error = None;
            }
        }
        Err(e) => {
            home_error = Some(e);
        }
    }

    // Get all mount-points and attempt to find a trash folder in each adding them to the SET of
    // trash folders when found one.
    let uid = unsafe { libc::getuid() };
    let sorted_mount_points = get_sorted_mount_points()?;
    for mount in &sorted_mount_points {
        execute_on_mounted_trash_folders(uid, &mount.mnt_dir, false, false, |trash_path| {
            trash_folders.insert(trash_path);
            Ok(())
        })
        .map_err(|(p, e)| fs_error(p, e))?;
    }

    Ok(EvaluatedTrashFolders { trash_folders, home_error, sorted_mount_points })
}
pub fn metadata(item: &TrashItem) -> Result<TrashItemMetadata, Error> {
    // When purging an item the "in-trash" filename must be parsed from the trashinfo filename
    // which is the filename in the `id` field.
    let info_file = &item.id;

    let file = restorable_file_in_trash_from_info_file(info_file);
    assert!(virtually_exists(&file).map_err(|e| fs_error(&file, e))?);
    let metadata = fs::symlink_metadata(&file).map_err(|e| fs_error(&file, e))?;
    let is_dir = metadata.is_dir();
    let size = if is_dir {
        TrashItemSize::Entries(fs::read_dir(&file).map_err(|e| fs_error(&file, e))?.count())
    } else {
        TrashItemSize::Bytes(metadata.len())
    };
    Ok(TrashItemMetadata { size })
}

/// The path points to:
/// - existing file | directory | symlink => Ok(true)
/// - broken symlink => Ok(true)
/// - nothing => Ok(false)
/// - I/O Error => Err(Io)
#[inline]
fn virtually_exists(path: &Path) -> std::io::Result<bool> {
    Ok(path.try_exists()? || path.is_symlink())
}

pub fn purge_all<I>(items: I) -> Result<(), Error>
where
    I: IntoIterator,
    <I as IntoIterator>::Item: Borrow<TrashItem>,
{
    for item in items.into_iter() {
        // When purging an item the "in-trash" filename must be parsed from the trashinfo filename
        // which is the filename in the `id` field.
        let info_file = &item.borrow().id;

        // A bunch of unwraps here. This is fine because if any of these fail that means
        // that either there's a bug in this code or the target system didn't follow
        // the specification.
        let file = restorable_file_in_trash_from_info_file(info_file);
        if file.is_dir() {
            std::fs::remove_dir_all(&file).map_err(|e| fs_error(&file, e))?;
        // TODO Update directory size cache if there's one.
        } else {
            std::fs::remove_file(&file).map_err(|e| fs_error(&file, e))?;
        }
        std::fs::remove_file(info_file).map_err(|e| fs_error(info_file, e))?;
    }

    Ok(())
}

fn restorable_file_in_trash_from_info_file(info_file: impl AsRef<std::ffi::OsStr>) -> PathBuf {
    let info_file = info_file.as_ref();
    let trash_folder = Path::new(info_file).parent().unwrap().parent().unwrap();
    let name_in_trash = Path::new(info_file).file_stem().unwrap();
    trash_folder.join("files").join(name_in_trash)
}

pub fn restore_all<I>(items: I) -> Result<(), Error>
where
    I: IntoIterator<Item = TrashItem>,
{
    // Simply read the items' original location from the infofile and attemp to move the items there
    // and delete the infofile if the move operation was sucessful.

    let mut iter = items.into_iter();
    while let Some(item) = iter.next() {
        // The "in-trash" filename must be parsed from the trashinfo filename
        // which is the filename in the `id` field.
        let info_file = &item.id;

        // A bunch of unwraps here. This is fine because if any of these fail that means
        // that either there's a bug in this code or the target system didn't follow
        // the specification.
        let file = restorable_file_in_trash_from_info_file(info_file);
        assert!(virtually_exists(&file).map_err(|e| fs_error(&file, e))?);
        // TODO add option to forcefully replace any target at the restore location
        // if it already exists.
        let original_path = item.original_path();
        // Make sure the parent exists so that `create_dir` doesn't faile due to that.
        std::fs::create_dir_all(&item.original_parent).map_err(|e| fs_error(&item.original_parent, e))?;
        let mut collision = false;
        if file.is_dir() {
            // NOTE create_dir_all succeeds when the path already exist but create_dir
            // fails with `std::io::ErrorKind::AlreadyExists`.
            if let Err(e) = std::fs::create_dir(&original_path) {
                if e.kind() == std::io::ErrorKind::AlreadyExists {
                    collision = true;
                } else {
                    return Err(fs_error(&original_path, e));
                }
            }
        } else {
            // File or symlink
            if let Err(e) = OpenOptions::new().create_new(true).write(true).open(&original_path) {
                if e.kind() == std::io::ErrorKind::AlreadyExists {
                    collision = true;
                } else {
                    return Err(fs_error(&original_path, e));
                }
            }
        }
        if collision {
            let remaining: Vec<_> = std::iter::once(item).chain(iter).collect();
            return Err(Error::RestoreCollision { path: original_path, remaining_items: remaining });
        }
        std::fs::rename(&file, &original_path).map_err(|e| fs_error(&file, e))?;
        std::fs::remove_file(info_file).map_err(|e| fs_error(info_file, e))?;
    }
    Ok(())
}

/// According to the specification (see at the top of the file) there are two kinds of
/// trash-folders for a mounted drive or partition.
/// 1, .Trash/uid
/// 2, .Trash-uid
///
/// This function executes `op` providing it with a
/// trash-folder path that's associated with the partition mounted at `topdir`.
///
fn execute_on_mounted_trash_folders<F: FnMut(PathBuf) -> Result<(), FsError>>(
    uid: u32,
    topdir: impl AsRef<Path>,
    first_only: bool,
    create_folder: bool,
    mut op: F,
) -> Result<(), FsError> {
    // See if there's a ".Trash" directory at the mounted location
    let topdir = topdir.as_ref();
    let trash_path = topdir.join(".Trash");
    if trash_path.is_dir() {
        let validity = folder_validity(&trash_path)?;
        if validity == TrashValidity::Valid {
            let users_trash_path = trash_path.join(uid.to_string());
            if users_trash_path.exists() && users_trash_path.is_dir() {
                op(users_trash_path)?;
                if first_only {
                    return Ok(());
                }
            }
        } else {
            warn!("A Trash folder was found at '{:?}', but it's invalid because it's {:?}", trash_path, validity);
        }
    }
    // See if there's a ".Trash-$UID" directory at the mounted location
    let trash_path = topdir.join(format!(".Trash-{uid}"));
    let should_execute;
    if !trash_path.exists() || !trash_path.is_dir() {
        if create_folder {
            std::fs::create_dir(&trash_path).map_err(|e| (trash_path.to_owned(), e))?;
            should_execute = true;
        } else {
            should_execute = false;
        }
    } else {
        should_execute = true;
    }
    if should_execute {
        op(trash_path)?;
    }
    Ok(())
}

fn move_to_trash(
    src: impl AsRef<Path>,
    trash_folder: impl AsRef<Path>,
    _topdir: impl AsRef<Path>,
) -> Result<(), FsError> {
    let src = src.as_ref();
    let trash_folder = trash_folder.as_ref();
    let files_folder = trash_folder.join("files");
    let info_folder = trash_folder.join("info");

    // Ensure the `files` and `info` folders exist
    std::fs::create_dir_all(&files_folder).map_err(|e| (files_folder.to_owned(), e))?;
    std::fs::create_dir_all(&info_folder).map_err(|e| (info_folder.to_owned(), e))?;

    // This kind of validity must only apply ot administrator style trash folders
    // See Trash directories, (1) at https://specifications.freedesktop.org/trash-spec/trashspec-1.0.html
    //assert_eq!(folder_validity(trash_folder)?, TrashValidity::Valid);

    // When trashing a file one must make sure that every trashed item is uniquely named.
    // However the `rename` function -that is used in *nix systems to move files- by default
    // overwrites the destination. Therefore when multiple threads are removing items with identical
    // names, an implementation might accidently overwrite an item that was just put into the trash
    // if it's not careful enough.
    //
    // The strategy here is to use the `create_new` parameter of `OpenOptions` to
    // try creating a placeholder file in the trash but don't do so if one with an identical name
    // already exist. This newly created empty file can then be safely overwritten by the src file
    // using the `rename` function.
    let filename = src.file_name().unwrap();
    let mut appendage = 0usize;
    loop {
        appendage += 1;
        let in_trash_name: Cow<'_, OsStr> = if appendage > 1 {
            let mut trash_name = filename.to_owned();
            trash_name.push(format!(".{appendage}"));
            trash_name.into()
        } else {
            filename.into()
        };
        // Length of name + length of '.trashinfo'
        let mut info_name = OsString::with_capacity(in_trash_name.len() + 10);
        info_name.push(&in_trash_name);
        info_name.push(".trashinfo");
        let info_file_path = info_folder.join(&info_name);
        let info_result = OpenOptions::new().create_new(true).write(true).open(&info_file_path);
        match info_result {
            Err(error) => {
                if error.kind() == std::io::ErrorKind::AlreadyExists {
                    continue;
                } else {
                    debug!("Failed to create the new file {:?}", info_file_path);
                    return Err((info_file_path, error));
                }
            }
            Ok(mut file) => {
                debug!("Successfully created {:?}", info_file_path);
                // Write the info file before actually moving anything
                writeln!(file, "[Trash Info]")
                    .and_then(|_| {
                        let absolute_uri = encode_uri_path(src);
                        writeln!(file, "Path={absolute_uri}").and_then(|_| {
                            #[cfg(feature = "chrono")]
                            {
                                let now = chrono::Local::now();
                                writeln!(file, "DeletionDate={}", now.format("%Y-%m-%dT%H:%M:%S"))
                            }
                            #[cfg(not(feature = "chrono"))]
                            {
                                Ok(())
                            }
                        })
                    })
                    .map_err(|e| (info_file_path.to_owned(), e))?;
            }
        }
        let path = files_folder.join(&in_trash_name);
        match move_items_no_replace(src, &path) {
            Err((path, error)) => {
                debug!("Failed moving item to the trash (this is usually OK). {:?}", error);
                // Try to delete the info file
                if let Err(info_err) = std::fs::remove_file(info_file_path) {
                    warn!("Created the trash info file, then failed to move the item to the trash. So far it's OK, but then failed remove the initial info file. There's either a bug in this program or another faulty program is manupulating the Trash. The error was: {:?}", info_err);
                }
                if error.kind() == std::io::ErrorKind::AlreadyExists {
                    continue;
                } else {
                    return Err((path, error));
                }
            }
            Ok(_) => {
                // We did it!
                break;
            }
        }
    }

    Ok(())
}

/// An error may mean that a collision was found.
fn move_items_no_replace(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<(), FsError> {
    let src = src.as_ref();
    let dst = dst.as_ref();

    try_creating_placeholders(src, dst)?;

    // Try to rename first (fastest option for same filesystem)
    let Err(e) = std::fs::rename(src, dst) else { return Ok(()) };

    let needs_cross_device_copy = e.kind() == ErrorKind::CrossesDevices;
    if !needs_cross_device_copy {
        return Err((src.to_owned(), e));
    }

    debug!("Cross-device move detected, falling back to copy+delete for {:?}", src);

    // Copy the file/directory
    if src.is_dir() {
        copy_dir_all(src, dst)?;
    } else {
        std::fs::copy(src, dst).map_err(|e| (src.to_owned(), e))?;
    }

    // Remove the source
    if src.is_dir() {
        std::fs::remove_dir_all(src).map_err(|e| (src.to_owned(), e))?;
    } else {
        std::fs::remove_file(src).map_err(|e| (src.to_owned(), e))?;
    }

    Ok(())
}

fn try_creating_placeholders(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<(), FsError> {
    let src = src.as_ref();
    let dst = dst.as_ref();
    let metadata = src.symlink_metadata().map_err(|e| (src.to_owned(), e))?;
    if metadata.is_dir() {
        // NOTE create_dir fails if the directory already exists
        std::fs::create_dir(dst).map_err(|e| (dst.to_owned(), e))?;
    } else {
        // Symlink or file
        OpenOptions::new().create_new(true).write(true).open(dst).map_err(|e| (dst.to_owned(), e))?;
    }
    Ok(())
}

/// Helper function to recursively copy a directory
fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<(), FsError> {
    let src = src.as_ref();
    let dst = dst.as_ref();

    std::fs::create_dir_all(dst).map_err(|e| (dst.to_owned(), e))?;

    for entry in std::fs::read_dir(src).map_err(|e| (src.to_owned(), e))? {
        let entry = entry.map_err(|e| (src.to_owned(), e))?;
        let file_type = entry.file_type().map_err(|e| (entry.path(), e))?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if file_type.is_dir() {
            copy_dir_all(&src_path, &dst_path)?;
        } else if file_type.is_symlink() {
            // Handle symlinks by copying the symlink itself, not the target
            let target = std::fs::read_link(&src_path).map_err(|e| (src_path.clone(), e))?;
            std::os::unix::fs::symlink(&target, &dst_path).map_err(|e| (dst_path.clone(), e))?;
        } else {
            std::fs::copy(&src_path, &dst_path).map_err(|e| (src_path.clone(), e))?;
        }
    }

    Ok(())
}

fn decode_uri_path(path: impl AsRef<Path>) -> PathBuf {
    // Paths may be invalid Unicode on most Unixes so they should be treated as byte strings
    // A higher level crate, such as `url`, can't be used directly since its API intakes valid Rust
    // strings. Thus, the easiest way is to manually decode each segment of the path and recombine.
    path.as_ref().iter().map(|part| OsString::from_vec(urlencoding::decode_binary(part.as_bytes()).to_vec())).collect()
}

fn encode_uri_path(path: impl AsRef<Path>) -> String {
    // `iter()` cannot be used here because it yields '/' in certain situations, such as
    // for root directories.
    // Slashes would be encoded and thus mess up the path
    let path: PathBuf = path
        .as_ref()
        .components()
        .map(|component| {
            // Only encode names and not '/', 'C:\', et cetera
            if let Component::Normal(part) = component {
                urlencoding::encode_binary(part.as_bytes()).to_string()
            } else {
                component.as_os_str().to_str().expect("Path components such as '/' are valid Unicode").to_owned()
            }
        })
        .collect();

    path.to_str().expect("URL encoded bytes is valid Unicode").to_owned()
}

#[derive(Eq, PartialEq, Debug)]
enum TrashValidity {
    Valid,
    InvalidSymlink,
    InvalidNotSticky,
}

fn folder_validity(path: impl AsRef<Path>) -> Result<TrashValidity, FsError> {
    /// Mask for the sticky bit
    /// Taken from: http://man7.org/linux/man-pages/man7/inode.7.html
    const S_ISVTX: u32 = 0o1000;

    let path = path.as_ref();
    let metadata = path.symlink_metadata().map_err(|e| (path.to_owned(), e))?;
    if metadata.file_type().is_symlink() {
        return Ok(TrashValidity::InvalidSymlink);
    }
    let mode = metadata.permissions().mode();
    let no_sticky_bit = (mode & S_ISVTX) == 0;
    if no_sticky_bit {
        return Ok(TrashValidity::InvalidNotSticky);
    }
    Ok(TrashValidity::Valid)
}

/// Corresponds to the definition of "home_trash" from
/// https://specifications.freedesktop.org/trash-spec/trashspec-1.0.html
fn home_trash() -> Result<PathBuf, Error> {
    if let Some(data_home) = std::env::var_os("XDG_DATA_HOME") {
        if !data_home.is_empty() {
            let data_home_path = AsRef::<Path>::as_ref(data_home.as_os_str());
            return Ok(data_home_path.join("Trash"));
        }
    }
    if let Some(home) = std::env::var_os("HOME") {
        if !home.is_empty() {
            let home_path = AsRef::<Path>::as_ref(home.as_os_str());
            return Ok(home_path.join(".local/share/Trash"));
        }
    }
    Err(Error::Unknown { description: "Neither the XDG_DATA_HOME nor the HOME environment variable was found".into() })
}

fn home_topdir(mnt_points: &[MountPoint]) -> Result<PathBuf, Error> {
    if let Some(data_home) = std::env::var_os("XDG_DATA_HOME") {
        if !data_home.is_empty() {
            let data_home_path = AsRef::<Path>::as_ref(data_home.as_os_str());
            return Ok(get_first_topdir_containing_path(data_home_path, mnt_points).to_owned());
        }
    }
    if let Some(home) = std::env::var_os("HOME") {
        if !home.is_empty() {
            let home_path = AsRef::<Path>::as_ref(home.as_os_str());
            return Ok(get_first_topdir_containing_path(home_path, mnt_points).to_owned());
        }
    }
    Err(Error::Unknown { description: "Neither the XDG_DATA_HOME nor the HOME environment variable was found".into() })
}

fn get_first_topdir_containing_path<'a>(path: &Path, mnt_points: &'a [MountPoint]) -> &'a Path {
    let root: &'static Path = Path::new("/");
    mnt_points.iter().map(|mp| mp.mnt_dir.as_path()).find(|mount_path| path.starts_with(mount_path)).unwrap_or(root)
}

struct MountPoint {
    mnt_dir: PathBuf,
    _mnt_type: String,
    _mnt_fsname: String,
}

/// Sorted by longest path first
fn get_sorted_mount_points() -> Result<Vec<MountPoint>, Error> {
    let mut mount_points = get_mount_points()?;
    mount_points.sort_unstable_by(|a, b| {
        let a = a.mnt_dir.as_os_str().as_bytes().len();
        let b = b.mnt_dir.as_os_str().as_bytes().len();
        a.cmp(&b).reverse()
    });
    Ok(mount_points)
}

#[cfg(target_os = "linux")]
fn get_mount_points() -> Result<Vec<MountPoint>, Error> {
    use once_cell::sync::Lazy;
    use scopeguard::defer;
    use std::ffi::{CStr, CString};
    use std::sync::Mutex;

    // The getmntinfo() function writes the array of structures to an internal
    // static object and returns a pointer to that object.  Subsequent calls to
    // getmntent() will modify the same object. This means that the function is
    // not threadsafe. To help prevent multiple threads using it concurrently
    // via get_mount_points a Mutex is used.
    // We understand that threads can still call `libc::getmntent(…)` directly
    // to bypass the lock and trigger UB.
    static LOCK: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));
    let _lock = LOCK.lock().unwrap();

    //let file;
    let read_arg = CString::new("r").unwrap();
    let mounts_path = CString::new("/proc/mounts").unwrap();
    let mut file = unsafe { libc::fopen(mounts_path.as_c_str().as_ptr(), read_arg.as_c_str().as_ptr()) };
    if file.is_null() {
        let mtab_path = CString::new("/etc/mtab").unwrap();
        file = unsafe { libc::fopen(mtab_path.as_c_str().as_ptr(), read_arg.as_c_str().as_ptr()) };
    }
    if file.is_null() {
        return Err(Error::Unknown { description: "Neither '/proc/mounts' nor '/etc/mtab' could be opened.".into() });
    }
    defer! { unsafe { libc::fclose(file); } }
    let mut result = Vec::new();
    loop {
        let mntent = unsafe { libc::getmntent(file) };
        if mntent.is_null() {
            break;
        }
        let dir = unsafe { CStr::from_ptr((*mntent).mnt_dir).to_str().unwrap() };
        if dir.is_empty() {
            continue;
        }
        let mount_point = unsafe {
            MountPoint {
                mnt_dir: dir.into(),
                _mnt_fsname: CStr::from_ptr((*mntent).mnt_fsname).to_str().unwrap().into(),
                _mnt_type: CStr::from_ptr((*mntent).mnt_type).to_str().unwrap().into(),
            }
        };
        result.push(mount_point);
    }
    if result.is_empty() {
        return Err(Error::Unknown {
            description: "A mount points file could be opened, but the call to `getmntent` returned NULL.".into(),
        });
    }
    Ok(result)
}

#[cfg(any(target_os = "dragonfly", target_os = "freebsd", target_os = "openbsd"))]
fn get_mount_points() -> Result<Vec<MountPoint>, Error> {
    use once_cell::sync::Lazy;
    use std::sync::Mutex;

    // The getmntinfo() function writes the array of structures to an internal
    // static object and returns a pointer to that object.  Subsequent calls to
    // getmntinfo() will modify the same object. This means that the function is
    // not threadsafe. To help prevent multiple threads using it concurrently
    // via get_mount_points a Mutex is used.
    // We understand that threads can still call `libc::getmntinfo(…)` directly
    // to bypass the lock and trigger UB.
    static LOCK: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));
    let _lock = LOCK.lock().unwrap();

    fn c_buf_to_str(buf: &[libc::c_char]) -> Option<&str> {
        let buf: &[u8] = unsafe { std::slice::from_raw_parts(buf.as_ptr() as _, buf.len()) };
        if let Some(pos) = buf.iter().position(|x| *x == 0) {
            // Shrink buffer to omit the null bytes
            std::str::from_utf8(&buf[..pos]).ok()
        } else {
            std::str::from_utf8(buf).ok()
        }
    }
    let mut fs_infos: *mut libc::statfs = std::ptr::null_mut();
    let count = unsafe { libc::getmntinfo(&mut fs_infos, libc::MNT_WAIT) };
    if count < 1 {
        return Ok(Vec::new());
    }
    let fs_infos: &[libc::statfs] = unsafe { std::slice::from_raw_parts(fs_infos as _, count as _) };

    let mut result = Vec::new();
    for fs_info in fs_infos {
        if fs_info.f_mntfromname[0] == 0 || fs_info.f_mntonname[0] == 0 {
            // If we have missing information, no need to look any further...
            continue;
        }
        let fs_type = c_buf_to_str(&fs_info.f_fstypename).unwrap_or_default();
        let mount_to = match c_buf_to_str(&fs_info.f_mntonname) {
            Some(m) => m,
            None => {
                debug!("Cannot get disk mount point, ignoring it.");
                continue;
            }
        };
        let mount_from = c_buf_to_str(&fs_info.f_mntfromname).unwrap_or_default();

        let mount_point =
            MountPoint { mnt_dir: mount_to.into(), _mnt_fsname: mount_from.into(), _mnt_type: fs_type.into() };
        result.push(mount_point);
    }
    Ok(result)
}

#[cfg(target_os = "netbsd")]
fn get_mount_points() -> Result<Vec<MountPoint>, Error> {
    use once_cell::sync::Lazy;
    use std::sync::Mutex;

    // The getmntinfo() function writes the array of structures to an internal
    // static object and returns a pointer to that object.  Subsequent calls to
    // getmntinfo() will modify the same object. This means that the function is
    // not threadsafe. To help prevent multiple threads using it concurrently
    // via get_mount_points a Mutex is used.
    // We understand that threads can still call `libc::getmntinfo(…)` directly
    // to bypass the lock and trigger UB.
    // NetBSD does not support statfs since 2005, so we need to use statvfs instead.
    static LOCK: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));
    let _lock = LOCK.lock().unwrap();

    fn c_buf_to_str(buf: &[libc::c_char]) -> Option<&str> {
        let buf: &[u8] = unsafe { std::slice::from_raw_parts(buf.as_ptr() as _, buf.len()) };
        if let Some(pos) = buf.iter().position(|x| *x == 0) {
            // Shrink buffer to omit the null bytes
            std::str::from_utf8(&buf[..pos]).ok()
        } else {
            std::str::from_utf8(buf).ok()
        }
    }
    let mut fs_infos: *mut libc::statvfs = std::ptr::null_mut();
    let count = unsafe { libc::getmntinfo(&mut fs_infos, libc::MNT_WAIT) };
    if count < 1 {
        return Ok(Vec::new());
    }
    let fs_infos: &[libc::statvfs] = unsafe { std::slice::from_raw_parts(fs_infos as _, count as _) };

    let mut result = Vec::new();
    for fs_info in fs_infos {
        if fs_info.f_mntfromname[0] == 0 || fs_info.f_mntonname[0] == 0 {
            // If we have missing information, no need to look any further...
            continue;
        }
        let fs_type = c_buf_to_str(&fs_info.f_fstypename).unwrap_or_default();
        let mount_to = match c_buf_to_str(&fs_info.f_mntonname) {
            Some(m) => m,
            None => {
                debug!("Cannot get disk mount point, ignoring it.");
                continue;
            }
        };
        let mount_from = c_buf_to_str(&fs_info.f_mntfromname).unwrap_or_default();

        let mount_point =
            MountPoint { mnt_dir: mount_to.into(), _mnt_fsname: mount_from.into(), _mnt_type: fs_type.into() };
        result.push(mount_point);
    }
    Ok(result)
}

#[cfg(not(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "openbsd",
    target_os = "netbsd"
)))]
fn get_mount_points() -> Result<Vec<MountPoint>, Error> {
    // On platforms that don't have support yet, return an error
    Err(Error::Unknown { description: "Mount points cannot be determined on this operating system".into() })
}

#[cfg(test)]
mod tests {
    use serial_test::serial;
    use std::{
        collections::{hash_map::Entry, HashMap},
        env,
        ffi::{OsStr, OsString},
        fmt,
        fs::File,
        os::unix::{self, ffi::OsStringExt},
        path::{Path, PathBuf},
        process::Command,
    };

    use log::warn;

    use crate::{
        canonicalize_paths, delete, delete_all,
        os_limited::{list, purge_all, restore_all},
        platform::encode_uri_path,
        tests::get_unique_name,
        Error,
    };

    use super::decode_uri_path;

    #[test]
    #[serial]
    fn test_list() {
        crate::tests::init_logging();

        let file_name_prefix = get_unique_name();
        let batches: usize = 2;
        let files_per_batch: usize = 3;
        let names: Vec<OsString> = (0..files_per_batch).map(|i| format!("{}#{}", file_name_prefix, i).into()).collect();
        for _ in 0..batches {
            for path in &names {
                File::create_new(path).unwrap();
            }
            // eprintln!("Deleting {:?}", names);
            let result = delete_all_using_system_program(&names);
            if let Err(SystemTrashError::NoTrashProgram) = &result {
                // For example may be the case on build systems that don't have a destop environment
                warn!("No system default trashing utility was found, using this crate's implementation");
                delete_all(&names).unwrap();
            } else {
                result.unwrap();
            }
        }
        let items = list().unwrap();
        let items: HashMap<_, Vec<_>> = items
            .into_iter()
            .filter(|x| x.name.as_encoded_bytes().starts_with(file_name_prefix.as_bytes()))
            .fold(HashMap::new(), |mut map, x| {
                match map.entry(x.name.clone()) {
                    Entry::Occupied(mut entry) => {
                        entry.get_mut().push(x);
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(vec![x]);
                    }
                }
                map
            });
        for name in names {
            match items.get(&name) {
                Some(items) => assert_eq!(items.len(), batches),
                None => panic!("ERROR Could not find '{:?}' in {:#?}", name, items),
            }
        }

        // Let's try to purge all the items we just created but ignore any errors
        // as this test should succeed as long as `list` works properly.
        let _ = purge_all(items.into_values().flatten());
    }

    #[test]
    #[serial]
    fn test_broken_symlinks() {
        crate::tests::init_logging();

        let file_name_prefix = get_unique_name();
        let file_count = 2;
        let names: Vec<OsString> = (0..file_count).map(|i| format!("{}#{}", file_name_prefix, i).into()).collect();
        let symlink_names: Vec<OsString> = names.iter().map(|name| format!("{:?}-symlink", name).into()).collect();

        // Test file symbolic link and directory symbolic link
        File::create_new(&names[0]).unwrap();
        std::fs::create_dir(&names[1]).unwrap();

        for (i, (name, symlink)) in names.iter().zip(&symlink_names).enumerate() {
            // Create symbolic link
            unix::fs::symlink(name, symlink).unwrap();

            // Break the symbolic link
            if i == 0 {
                std::fs::remove_file(name).unwrap();
            } else {
                std::fs::remove_dir(name).unwrap();
            }

            // Delete and Restore it without errors
            delete(symlink).unwrap();
            let items = list().unwrap();
            let item = items.into_iter().find(|it| it.name == *symlink).unwrap();
            restore_all([item.clone()]).expect("The broken symbolic link should be restored successfully.");

            // Delete and Purge it without errors
            delete(symlink).unwrap();
            purge_all([item]).expect("The broken symbolic link should be purged successfully.");
        }
    }

    #[test]
    fn uri_enc_dec_roundtrip() {
        let fake = format!("/tmp/{}", get_unique_name());
        let path = decode_uri_path(&fake);

        assert_eq!(path.to_str().expect("Path is valid Unicode"), fake, "Decoded path shouldn't be different");

        let encoded = encode_uri_path(&path);
        assert_eq!(encoded, fake, "URL encoded alphanumeric String shouldn't change");
    }

    #[test]
    fn uri_enc_dec_roundtrip_invalid_unicode() {
        let base = OsStr::new(&format!("/tmp/{}/", get_unique_name())).to_os_string();

        // Add invalid UTF-8 byte
        let mut bytes = base.into_encoded_bytes();
        bytes.push(168);
        let fake = OsString::from_vec(bytes);
        assert!(fake.to_str().is_none(), "Invalid Unicode cannot be a Rust String");

        let path = decode_uri_path(&fake);
        assert_eq!(path.as_os_str().as_encoded_bytes(), fake.as_encoded_bytes());

        // Shouldn't panic
        encode_uri_path(&path);
    }

    //////////////////////////////////////////////////////////////////////////////////////
    /// System
    //////////////////////////////////////////////////////////////////////////////////////

    #[derive(Debug)]
    pub enum SystemTrashError {
        NoTrashProgram,
        Other(Error),
    }
    impl fmt::Display for SystemTrashError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "SystemTrashError during a `trash` operation: {:?}", self)
        }
    }
    impl std::error::Error for SystemTrashError {}

    fn is_program_in_path(program: &str) -> bool {
        if let Some(path_vars) = std::env::var_os("PATH") {
            for path in std::env::split_paths(&path_vars) {
                let full_path = path.join(program);
                if full_path.is_file() {
                    return true;
                }
            }
        }
        false
    }

    /// This is based on the electron library's implementation.
    /// See: https://github.com/electron/electron/blob/34c4c8d5088fa183f56baea28809de6f2a427e02/shell/common/platform_util_linux.cc#L96
    pub fn delete_all_canonicalized_using_system_program(full_paths: Vec<PathBuf>) -> Result<(), SystemTrashError> {
        static DEFAULT_TRASH: &str = "gio";
        let trash = {
            // Determine desktop environment and set accordingly.
            let desktop_env = get_desktop_environment();
            if desktop_env == DesktopEnvironment::Kde4 || desktop_env == DesktopEnvironment::Kde5 {
                "kioclient5"
            } else if desktop_env == DesktopEnvironment::Kde3 {
                "kioclient"
            } else {
                DEFAULT_TRASH
            }
        };

        let mut argv = Vec::<OsString>::with_capacity(full_paths.len() + 2);

        if trash == "kioclient5" || trash == "kioclient" {
            //argv.push(trash.into());
            argv.push("move".into());
            for full_path in &full_paths {
                argv.push(full_path.into());
            }
            argv.push("trash:/".into());
        } else {
            //argv.push_back(ELECTRON_DEFAULT_TRASH);
            argv.push("trash".into());
            for full_path in &full_paths {
                argv.push(full_path.into());
            }
        }
        if !is_program_in_path(trash) {
            return Err(SystemTrashError::NoTrashProgram);
        }
        // Execute command
        let mut command = Command::new(trash);
        command.args(argv);
        let result = command.output().map_err(|e| {
            SystemTrashError::Other(Error::Unknown {
                description: format!("Tried executing: {:?} - Error was: {}", command, e),
            })
        })?;
        if !result.status.success() {
            let stderr = String::from_utf8_lossy(&result.stderr);
            return Err(SystemTrashError::Other(Error::Unknown {
                description: format!("Used '{}', stderr: {}", trash, stderr),
            }));
        }
        Ok(())
    }

    pub fn delete_all_using_system_program<I, T>(paths: I) -> Result<(), SystemTrashError>
    where
        I: IntoIterator<Item = T>,
        T: AsRef<Path>,
    {
        let full_paths = canonicalize_paths(paths).map_err(SystemTrashError::Other)?;
        delete_all_canonicalized_using_system_program(full_paths)
    }

    #[derive(PartialEq)]
    enum DesktopEnvironment {
        Other,
        Cinnamon,
        Gnome,
        // KDE3, KDE4 and KDE5 are sufficiently different that we count
        // them as different desktop environments here.
        Kde3,
        Kde4,
        Kde5,
        Pantheon,
        Unity,
        Xfce,
    }

    fn env_has_var(name: &str) -> bool {
        env::var_os(name).is_some()
    }

    /// See: https://chromium.googlesource.com/chromium/src/+/dd407d416fa941c04e33d81f2b1d8cab8196b633/base/nix/xdg_util.cc#57
    fn get_desktop_environment() -> DesktopEnvironment {
        static KDE_SESSION_ENV_VAR: &str = "KDE_SESSION_VERSION";
        // XDG_CURRENT_DESKTOP is the newest standard circa 2012.
        if let Ok(xdg_current_desktop) = env::var("XDG_CURRENT_DESKTOP") {
            // It could have multiple values separated by colon in priority order.
            for value in xdg_current_desktop.split(':') {
                let value = value.trim();
                if value.is_empty() {
                    continue;
                }
                match value {
                    "Unity" => {
                        // gnome-fallback sessions set XDG_CURRENT_DESKTOP to Unity
                        // DESKTOP_SESSION can be gnome-fallback or gnome-fallback-compiz
                        if let Ok(desktop_session) = env::var("DESKTOP_SESSION") {
                            if desktop_session.contains("gnome-fallback") {
                                return DesktopEnvironment::Gnome;
                            }
                        }
                        return DesktopEnvironment::Unity;
                    }
                    "GNOME" => {
                        return DesktopEnvironment::Gnome;
                    }
                    "X-Cinnamon" => {
                        return DesktopEnvironment::Cinnamon;
                    }
                    "KDE" => {
                        if let Ok(kde_session) = env::var(KDE_SESSION_ENV_VAR) {
                            if kde_session == "5" {
                                return DesktopEnvironment::Kde5;
                            }
                        }
                        return DesktopEnvironment::Kde4;
                    }
                    "Pantheon" => {
                        return DesktopEnvironment::Pantheon;
                    }
                    "XFCE" => {
                        return DesktopEnvironment::Xfce;
                    }
                    _ => {}
                }
            }
        }

        // DESKTOP_SESSION was what everyone  used in 2010.
        if let Ok(desktop_session) = env::var("DESKTOP_SESSION") {
            match desktop_session.as_str() {
                "gnome" | "mate" => {
                    return DesktopEnvironment::Gnome;
                }
                "kde4" | "kde-plasma" => {
                    return DesktopEnvironment::Kde4;
                }
                "kde" => {
                    // This may mean KDE4 on newer systems, so we have to check.
                    if env_has_var(KDE_SESSION_ENV_VAR) {
                        return DesktopEnvironment::Kde4;
                    }
                    return DesktopEnvironment::Kde3;
                }
                "xubuntu" => {
                    return DesktopEnvironment::Xfce;
                }
                _ => {}
            }
            if desktop_session.contains("xfce") {
                return DesktopEnvironment::Xfce;
            }
        }

        // Fall back on some older environment variables.
        // Useful particularly in the DESKTOP_SESSION=default case.
        if env_has_var("GNOME_DESKTOP_SESSION_ID") {
            return DesktopEnvironment::Gnome;
        } else if env_has_var("KDE_FULL_SESSION") {
            if env_has_var(KDE_SESSION_ENV_VAR) {
                return DesktopEnvironment::Kde4;
            }
            return DesktopEnvironment::Kde3;
        }

        DesktopEnvironment::Other
    }
}

fn fs_error(path: impl Into<PathBuf>, source: std::io::Error) -> Error {
    Error::FileSystem { path: path.into(), source }
}
