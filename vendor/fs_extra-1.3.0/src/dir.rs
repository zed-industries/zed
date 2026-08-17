use crate::error::*;
use std::collections::{HashMap, HashSet};
use std::fs::{create_dir, create_dir_all, read_dir, remove_dir_all, Metadata};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Options and flags which can be used to configure how a file will be copied or moved.
#[derive(Clone)]
pub struct CopyOptions {
    /// Overwrite existing files if true (default: false).
    pub overwrite: bool,
    /// Skip existing files if true (default: false).
    pub skip_exist: bool,
    /// Buffer size that specifies the amount of bytes to be moved or copied before the progress handler is called. This only affects functions with progress handlers. (default: 64000)
    pub buffer_size: usize,
    /// Recursively copy a directory with a new name or place it inside the destination (default: false, same behaviors as cp -r on Unix)
    pub copy_inside: bool,
    /// Copy only contents without a creating a new folder in the destination folder (default: false).
    pub content_only: bool,
    /// Sets levels reading. Set 0 for read all directory folder (default: 0).
    ///
    /// Warning: Work only for copy operations!
    pub depth: u64,
}

impl CopyOptions {
    /// Initialize struct CopyOptions with default value.
    ///
    /// ```rust,ignore
    /// overwrite: false
    ///
    /// skip_exist: false
    ///
    /// buffer_size: 64000 // 64kb
    ///
    /// copy_inside: false
    /// ```
    pub fn new() -> CopyOptions {
        CopyOptions {
            overwrite: false,
            skip_exist: false,
            buffer_size: 64000, // 64kb
            copy_inside: false,
            content_only: false,
            depth: 0,
        }
    }

    /// Overwrite existing files if true.
    pub fn overwrite(mut self, overwrite: bool) -> Self {
        self.overwrite = overwrite;
        self
    }

    /// Skip existing files if true.
    pub fn skip_exist(mut self, skip_exist: bool) -> Self {
        self.skip_exist = skip_exist;
        self
    }

    /// Buffer size that specifies the amount of bytes to be moved or copied before the progress handler is called. This only affects functions with progress handlers.
    pub fn buffer_size(mut self, buffer_size: usize) -> Self {
        self.buffer_size = buffer_size;
        self
    }

    /// Recursively copy a directory with a new name or place it inside the destination (default: false, same behaviors as cp -r on Unix)
    pub fn copy_inside(mut self, copy_inside: bool) -> Self {
        self.copy_inside = copy_inside;
        self
    }

    /// Copy only contents without a creating a new folder in the destination folder.
    pub fn content_only(mut self, content_only: bool) -> Self {
        self.content_only = content_only;
        self
    }

    /// Sets levels reading. Set 0 for read all directory folder
    pub fn depth(mut self, depth: u64) -> Self {
        self.depth = depth;
        self
    }
}

impl Default for CopyOptions {
    fn default() -> Self {
        CopyOptions::new()
    }
}

// Options and flags which can be used to configure how to read a directory.
#[derive(Clone, Default)]
pub struct DirOptions {
    /// Sets levels reading. Set value 0 for read all directory folder. By default 0.
    pub depth: u64,
}

impl DirOptions {
    /// Initialize struct DirOptions with default value.
    pub fn new() -> DirOptions {
        Default::default()
    }
}

/// A structure which include information about directory
pub struct DirContent {
    /// Directory size in bytes.
    pub dir_size: u64,
    /// List all files directory and sub directories.
    pub files: Vec<String>,
    /// List all folders and sub folders directory.
    pub directories: Vec<String>,
}

/// A structure which include information about the current status of the copy or move directory.
pub struct TransitProcess {
    /// Copied bytes on this time for folder
    pub copied_bytes: u64,
    /// All the bytes which should to copy or move (dir size).
    pub total_bytes: u64,
    /// Copied bytes on this time for file.
    pub file_bytes_copied: u64,
    /// Size current copied file.
    pub file_total_bytes: u64,
    /// Name current copied file.
    pub file_name: String,
    /// Transit state
    pub state: TransitState,
}

///
#[derive(Hash, Eq, PartialEq, Clone)]
pub enum TransitState {
    /// Standard state.
    Normal,
    /// Pause state when destination path exists.
    Exists,
    /// Pause state when current process does not have the permission to access from or to
    /// path.
    NoAccess,
}

/// Available returns codes for user decide
pub enum TransitProcessResult {
    /// Rewrite exist file or directory.
    Overwrite,
    /// Rewrite for all exist files or directories.
    OverwriteAll,
    /// Skip current problem file or directory.
    Skip,
    /// Skip for all problems file or directory.
    SkipAll,
    /// Retry current operation.
    Retry,
    /// Abort current operation.
    Abort,
    /// Continue execute process if process not have error and abort if process content error.
    ContinueOrAbort,
}

impl Clone for TransitProcess {
    fn clone(&self) -> TransitProcess {
        TransitProcess {
            copied_bytes: self.copied_bytes,
            total_bytes: self.total_bytes,
            file_bytes_copied: self.file_bytes_copied,
            file_total_bytes: self.file_total_bytes,
            file_name: self.file_name.clone(),
            state: self.state.clone(),
        }
    }
}

/// Available attributes for get information about directory entry.
#[derive(Hash, Eq, PartialEq, Clone)]
pub enum DirEntryAttr {
    /// Folder name or file name without extension.
    Name,
    /// File extension.
    Ext,
    /// Folder name or file name with extension.
    FullName,
    /// Path to file or directory.
    Path,
    /// Dos path to file or directory.
    DosPath,
    /// File size in bytes.
    FileSize,
    /// Size file or directory in bytes.
    ///
    /// `Attention!`: This operation very expensive and sometimes required additional rights.
    Size,
    /// Return whether entry is directory or not.
    IsDir,
    /// Return whether entry is file or not.
    IsFile,
    /// Last modification time for directory entry.
    Modified,
    /// Last access time for directory entry.
    Accessed,
    /// Created time for directory entry.
    ///
    /// `Attention!`: Not supported UNIX platform.
    Created,
    /// Return or not return base information target folder.
    BaseInfo,
}

/// Available types for directory entry.
pub enum DirEntryValue {
    /// String type
    String(String),
    /// Boolean type
    Boolean(bool),
    /// SystemTime type
    SystemTime(SystemTime),
    /// u64 type
    U64(u64),
}

/// Result returned by the `ls` function.
pub struct LsResult {
    /// Base folder target path
    pub base: HashMap<DirEntryAttr, DirEntryValue>,
    /// Collection directory entry with information.
    pub items: Vec<HashMap<DirEntryAttr, DirEntryValue>>,
}

/// Returned information about directory entry with information which you choose in config.
///
/// This function takes to arguments:
///
/// * `path` - Path to directory.
///
/// * `config` - Set attributes which you want see inside return data.
///
/// # Errors
///
/// This function will return an error in the following situations, but is not limited to just
/// these cases:
///
/// * This `path` does not exist.
/// * Invalid `path`.
/// * The current process does not have the permission to access `path`.
///
/// #Examples
///
/// ```rust,ignore
/// extern crate fs_extra;
/// use fs_extra::dir::{get_details_entry, DirEntryAttr};
/// use std::collections::{HashMap, HashSet};
///
/// let mut config = HashSet::new();
/// config.insert(DirEntryAttr::Name);
/// config.insert(DirEntryAttr::Size);
///
/// let entry_info = get_details_entry("test", &config);
/// assert_eq!(2, entry_info.len());
/// ```
pub fn get_details_entry<P>(
    path: P,
    config: &HashSet<DirEntryAttr>,
) -> Result<HashMap<DirEntryAttr, DirEntryValue>>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let metadata = path.metadata()?;
    get_details_entry_with_meta(path, config, metadata)
}

fn get_details_entry_with_meta<P>(
    path: P,
    config: &HashSet<DirEntryAttr>,
    metadata: Metadata,
) -> Result<HashMap<DirEntryAttr, DirEntryValue>>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let mut item = HashMap::new();
    if config.contains(&DirEntryAttr::Name) {
        if metadata.is_dir() {
            if let Some(file_name) = path.file_name() {
                item.insert(
                    DirEntryAttr::Name,
                    DirEntryValue::String(file_name.to_os_string().into_string()?),
                );
            } else {
                item.insert(DirEntryAttr::Name, DirEntryValue::String(String::new()));
            }
        } else if let Some(file_stem) = path.file_stem() {
            item.insert(
                DirEntryAttr::Name,
                DirEntryValue::String(file_stem.to_os_string().into_string()?),
            );
        } else {
            item.insert(DirEntryAttr::Name, DirEntryValue::String(String::new()));
        }
    }
    if config.contains(&DirEntryAttr::Ext) {
        if let Some(value) = path.extension() {
            item.insert(
                DirEntryAttr::Ext,
                DirEntryValue::String(value.to_os_string().into_string()?),
            );
        } else {
            item.insert(DirEntryAttr::Ext, DirEntryValue::String(String::from("")));
        }
    }
    if config.contains(&DirEntryAttr::FullName) {
        if let Some(file_name) = path.file_name() {
            item.insert(
                DirEntryAttr::FullName,
                DirEntryValue::String(file_name.to_os_string().into_string()?),
            );
        } else {
            item.insert(DirEntryAttr::FullName, DirEntryValue::String(String::new()));
        }
    }
    if config.contains(&DirEntryAttr::Path) {
        let mut result_path: PathBuf;
        match path.canonicalize() {
            Ok(new_path) => {
                result_path = new_path;
            }
            Err(_) => {
                if let Some(parent_path) = path.parent() {
                    if let Some(name) = path.file_name() {
                        result_path = parent_path.canonicalize()?;
                        result_path.push(name);
                    } else {
                        err!("Error get part name path", ErrorKind::Other);
                    }
                } else {
                    err!("Error get parent path", ErrorKind::Other);
                }
            }
        }
        let mut path = result_path.as_os_str().to_os_string().into_string()?;
        if path.find("\\\\?\\") == Some(0) {
            path = path[4..].to_string();
        }
        item.insert(DirEntryAttr::Path, DirEntryValue::String(path));
    }
    if config.contains(&DirEntryAttr::DosPath) {
        let mut result_path: PathBuf;
        match path.canonicalize() {
            Ok(new_path) => {
                result_path = new_path;
            }
            Err(_) => {
                if let Some(parent_path) = path.parent() {
                    if let Some(name) = path.file_name() {
                        result_path = parent_path.canonicalize()?;
                        result_path.push(name);
                    } else {
                        err!("Error get part name path", ErrorKind::Other);
                    }
                } else {
                    err!("Error get parent path", ErrorKind::Other);
                }
            }
        }
        let path = result_path.as_os_str().to_os_string().into_string()?;
        item.insert(DirEntryAttr::DosPath, DirEntryValue::String(path));
    }
    if config.contains(&DirEntryAttr::Size) {
        item.insert(DirEntryAttr::Size, DirEntryValue::U64(get_size(&path)?));
    }
    if config.contains(&DirEntryAttr::FileSize) {
        item.insert(DirEntryAttr::FileSize, DirEntryValue::U64(metadata.len()));
    }
    if config.contains(&DirEntryAttr::IsDir) {
        item.insert(
            DirEntryAttr::IsDir,
            DirEntryValue::Boolean(metadata.is_dir()),
        );
    }
    if config.contains(&DirEntryAttr::IsFile) {
        item.insert(
            DirEntryAttr::IsFile,
            DirEntryValue::Boolean(metadata.is_file()),
        );
    }
    if config.contains(&DirEntryAttr::Modified) {
        item.insert(
            DirEntryAttr::Modified,
            DirEntryValue::SystemTime(metadata.modified()?),
        );
    }
    if config.contains(&DirEntryAttr::Accessed) {
        item.insert(
            DirEntryAttr::Accessed,
            DirEntryValue::SystemTime(metadata.accessed()?),
        );
    }
    if config.contains(&DirEntryAttr::Created) {
        item.insert(
            DirEntryAttr::Created,
            DirEntryValue::SystemTime(metadata.created()?),
        );
    }
    Ok(item)
}

/// Returns a collection of directory entries with attributes specifying the information that should be returned.
///
/// This function takes to arguments:
///
/// * `path` - Path to directory.
///
/// * `config` - Set attributes which you want see in return data.
///
/// # Errors
///
/// This function will return an error in the following situations, but is not limited to just
/// these cases:
///
/// * This `path` directory does not exist.
/// * Invalid `path`.
/// * The current process does not have the permission to access `path`.
///
/// #Examples
///
/// ```rust,ignore
/// extern crate fs_extra;
/// use fs_extra::dir::{ls, DirEntryAttr, LsResult};
/// use std::collections::HashSet;
///
/// let mut config = HashSet::new();
/// config.insert(DirEntryAttr::Name);
/// config.insert(DirEntryAttr::Size);
/// config.insert(DirEntryAttr::BaseInfo);
///
/// let result = ls("test", &config);
/// assert_eq!(2, ls_result.items.len());
/// assert_eq!(2, ls_result.base.len());
/// ```
pub fn ls<P>(path: P, config: &HashSet<DirEntryAttr>) -> Result<LsResult>
where
    P: AsRef<Path>,
{
    let mut items = Vec::new();
    let path = path.as_ref();
    if !path.is_dir() {
        err!("Path does not directory", ErrorKind::InvalidFolder);
    }
    for entry in read_dir(&path)? {
        let entry = entry?;
        let path = entry.path();
        let metadata = entry.metadata()?;
        let item = get_details_entry_with_meta(path, &config, metadata)?;
        items.push(item);
    }
    let mut base = HashMap::new();
    if config.contains(&DirEntryAttr::BaseInfo) {
        base = get_details_entry(&path, &config)?;
    }
    Ok(LsResult { items, base })
}

/// Creates a new, empty directory at the provided path.
///
/// This function takes to arguments:
///
/// * `path` - Path to new directory.
///
/// * `erase` - If set true and folder exist, then folder will be erased.
///
/// #Errors
///
/// This function will return an error in the following situations,
/// but is not limited to just these cases:
///
/// * User lacks permissions to create directory at `path`.
///
/// * `path` already exists if `erase` set false.
///
/// #Examples
///
/// ```rust,ignore
/// extern crate fs_extra;
/// use fs_extra::dir::create;
///
/// create("dir", false); // create directory
/// ```
pub fn create<P>(path: P, erase: bool) -> Result<()>
where
    P: AsRef<Path>,
{
    if erase && path.as_ref().exists() {
        remove(&path)?;
    }
    Ok(create_dir(&path)?)
}

/// Recursively create a directory and all of its parent components if they are missing.
///
/// This function takes to arguments:
///
/// * `path` - Path to new directory.
///
/// * `erase` - If set true and folder exist, then folder will be erased.
///
///#Errors
///
/// This function will return an error in the following situations,
/// but is not limited to just these cases:
///
/// * User lacks permissions to create directory at `path`.
///
/// * `path` already exists if `erase` set false.
///
/// #Examples
///
/// ```rust,ignore
/// extern crate fs_extra;
/// use fs_extra::dir::create_all;
///
/// create_all("/some/dir", false); // create directory some and dir
pub fn create_all<P>(path: P, erase: bool) -> Result<()>
where
    P: AsRef<Path>,
{
    if erase && path.as_ref().exists() {
        remove(&path)?;
    }
    Ok(create_dir_all(&path)?)
}

/// Copies the directory contents from one place to another using recursive method.
/// This function will also copy the permission bits of the original files to
/// destination files (not for directories).
///
/// # Errors
///
/// This function will return an error in the following situations, but is not limited to just
/// these cases:
///
/// * This `from` path is not a directory.
/// * This `from` directory does not exist.
/// * Invalid folder name for `from` or `to`.
/// * The current process does not have the permission to access `from` or write `to`.
///
/// # Example
/// ```rust,ignore
/// extern crate fs_extra;
/// use fs_extra::dir::copy;
///
/// let options = CopyOptions::new(); //Initialize default values for CopyOptions
/// // options.mirror_copy = true; // To mirror copy the whole structure of the source directory
///
///
/// // copy source/dir1 to target/dir1
/// copy("source/dir1", "target/dir1", &options)?;
///
/// ```
pub fn copy<P, Q>(from: P, to: Q, options: &CopyOptions) -> Result<u64>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let from = from.as_ref();

    if !from.exists() {
        if let Some(msg) = from.to_str() {
            let msg = format!("Path \"{}\" does not exist or you don't have access!", msg);
            err!(&msg, ErrorKind::NotFound);
        }
        err!(
            "Path does not exist Or you don't have access!",
            ErrorKind::NotFound
        );
    }
    if !from.is_dir() {
        if let Some(msg) = from.to_str() {
            let msg = format!("Path \"{}\" is not a directory!", msg);
            err!(&msg, ErrorKind::InvalidFolder);
        }
        err!("Path is not a directory!", ErrorKind::InvalidFolder);
    }
    let dir_name;
    if let Some(val) = from.components().last() {
        dir_name = val.as_os_str();
    } else {
        err!("Invalid folder from", ErrorKind::InvalidFolder);
    }
    let mut to: PathBuf = to.as_ref().to_path_buf();
    if (to.exists() || !options.copy_inside) && !options.content_only {
        to.push(dir_name);
    }

    let mut read_options = DirOptions::new();
    if options.depth > 0 {
        read_options.depth = options.depth;
    }

    let dir_content = get_dir_content2(from, &read_options)?;
    for directory in dir_content.directories {
        let tmp_to = Path::new(&directory).strip_prefix(from)?;
        let dir = to.join(&tmp_to);
        if !dir.exists() {
            if options.copy_inside {
                create_all(dir, false)?;
            } else {
                create(dir, false)?;
            }
        }
    }
    let mut result: u64 = 0;
    for file in dir_content.files {
        let to = to.to_path_buf();
        let tp = Path::new(&file).strip_prefix(from)?;
        let path = to.join(&tp);

        let file_options = super::file::CopyOptions {
            overwrite: options.overwrite,
            skip_exist: options.skip_exist,
            buffer_size: options.buffer_size,
        };
        let mut result_copy: Result<u64>;
        let mut work = true;

        while work {
            result_copy = super::file::copy(&file, &path, &file_options);
            match result_copy {
                Ok(val) => {
                    result += val;
                    work = false;
                }
                Err(err) => {
                    let err_msg = err.to_string();
                    err!(err_msg.as_str(), err.kind)
                }
            }
        }
    }
    Ok(result)
}

/// Return DirContent which contains information about directory:
///
/// * Size of the directory in bytes.
/// * List of source paths of files in the directory (files inside subdirectories included too).
/// * List of source paths of all directories and subdirectories.
///
/// # Errors
///
/// This function will return an error in the following situations, but is not limited to just
/// these cases:
///
/// * This `path` directory does not exist.
/// * Invalid `path`.
/// * The current process does not have the permission to access `path`.
///
/// # Examples
/// ```rust,ignore
/// extern crate fs_extra;
/// use fs_extra::dir::get_dir_content;
///
/// let dir_content = get_dir_content("dir")?;
/// for directory in dir_content.directories {
///     println!("{}", directory); // print directory path
/// }
/// ```
///
pub fn get_dir_content<P>(path: P) -> Result<DirContent>
where
    P: AsRef<Path>,
{
    let options = DirOptions::new();
    get_dir_content2(path, &options)
}

/// Return DirContent which contains information about directory:
///
/// * Size directory.
/// * List all files source directory(files subdirectories  included too).
/// * List all directory and subdirectories source path.
///
/// # Errors
///
/// This function will return an error in the following situations, but is not limited to just
/// these cases:
///
/// * This `path` directory does not exist.
/// * Invalid `path`.
/// * The current process does not have the permission to access `path`.
///
/// # Examples
/// ```rust,ignore
/// extern crate fs_extra;
/// use fs_extra::dir::{DirOptions, get_dir_content2};
///
/// let mut options = DirOptions::new();
/// options.depth = 3; // Get 3 levels of folder.
/// let dir_content = get_dir_content2("dir", &options)?;
/// for directory in dir_content.directories {
///     println!("{}", directory); // print directory path
/// }
/// ```
///
pub fn get_dir_content2<P>(path: P, options: &DirOptions) -> Result<DirContent>
where
    P: AsRef<Path>,
{
    let mut depth = 0;
    if options.depth != 0 {
        depth = options.depth + 1;
    }
    _get_dir_content(path, depth)
}

fn _get_dir_content<P>(path: P, mut depth: u64) -> Result<DirContent>
where
    P: AsRef<Path>,
{
    let mut directories = Vec::new();
    let mut files = Vec::new();
    let mut dir_size;
    let item = path.as_ref().to_str();
    if item.is_none() {
        err!("Invalid path", ErrorKind::InvalidPath);
    }
    let item = item.unwrap().to_string();

    if path.as_ref().is_dir() {
        dir_size = path.as_ref().metadata()?.len();
        directories.push(item);
        if depth == 0 || depth > 1 {
            if depth > 1 {
                depth -= 1;
            }
            for entry in read_dir(&path)? {
                let _path = entry?.path();

                match _get_dir_content(_path, depth) {
                    Ok(items) => {
                        let mut _files = items.files;
                        let mut _directories = items.directories;
                        dir_size += items.dir_size;
                        files.append(&mut _files);
                        directories.append(&mut _directories);
                    }
                    Err(err) => return Err(err),
                }
            }
        }
    } else {
        dir_size = path.as_ref().metadata()?.len();
        files.push(item);
    }
    Ok(DirContent {
        dir_size,
        files,
        directories,
    })
}

/// Returns the size of the file or directory in bytes.(!important: folders size not count)
///
/// If used on a directory, this function will recursively iterate over every file and every
/// directory inside the directory. This can be very time consuming if used on large directories.
///
/// Does not follow symlinks.
///
/// # Errors
///
/// This function will return an error in the following situations, but is not limited to just
/// these cases:
///
/// * This `path` directory does not exist.
/// * Invalid `path`.
/// * The current process does not have the permission to access `path`.
///
/// # Examples
/// ```rust,ignore
/// extern crate fs_extra;
/// use fs_extra::dir::get_size;
///
/// let folder_size = get_size("dir")?;
/// println!("{}", folder_size); // print directory size in bytes
/// ```
pub fn get_size<P>(path: P) -> Result<u64>
where
    P: AsRef<Path>,
{
    // Using `fs::symlink_metadata` since we don't want to follow symlinks,
    // as we're calculating the exact size of the requested path itself.
    let path_metadata = path.as_ref().symlink_metadata()?;

    let mut size_in_bytes = 0;

    if path_metadata.is_dir() {
        for entry in read_dir(&path)? {
            let entry = entry?;
            // `DirEntry::metadata` does not follow symlinks (unlike `fs::metadata`), so in the
            // case of symlinks, this is the size of the symlink itself, not its target.
            let entry_metadata = entry.metadata()?;

            if entry_metadata.is_dir() {
                // The size of the directory entry itself will be counted inside the `get_size()` call,
                // so we intentionally don't also add `entry_metadata.len()` to the total here.
                size_in_bytes += get_size(entry.path())?;
            } else {
                size_in_bytes += entry_metadata.len();
            }
        }
    } else {
        size_in_bytes = path_metadata.len();
    }

    Ok(size_in_bytes)
}

/// Copies the directory contents from one place to another using recursive method,
/// with information about progress. This function will also copy the
/// permission bits of the original files to destination files (not for directories).
///
/// # Errors
///
/// This function will return an error in the following situations, but is not limited to just
/// these cases:
///
/// * This `from` path is not a directory.
/// * This `from` directory does not exist.
/// * Invalid folder name for `from` or `to`.
/// * The current process does not have the permission to access `from` or write `to`.
///
/// # Example
/// ```rust,ignore
/// extern crate fs_extra;
/// use fs_extra::dir::copy;
///
/// let options = CopyOptions::new(); //Initialize default values for CopyOptions
/// let handle = |process_info: TransitProcess|  {
///     println!("{}", process_info.total_bytes);
///     fs_extra::dir::TransitProcessResult::ContinueOrAbort
/// }
/// // copy source/dir1 to target/dir1
/// copy_with_progress("source/dir1", "target/dir1", &options, handle)?;
///
/// ```
pub fn copy_with_progress<P, Q, F>(
    from: P,
    to: Q,
    options: &CopyOptions,
    mut progress_handler: F,
) -> Result<u64>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
    F: FnMut(TransitProcess) -> TransitProcessResult,
{
    let from = from.as_ref();

    if !from.exists() {
        if let Some(msg) = from.to_str() {
            let msg = format!("Path \"{}\" does not exist or you don't have access!", msg);
            err!(&msg, ErrorKind::NotFound);
        }
        err!(
            "Path does not exist or you don't have access!",
            ErrorKind::NotFound
        );
    }

    let mut to: PathBuf = to.as_ref().to_path_buf();
    if !from.is_dir() {
        if let Some(msg) = from.to_str() {
            let msg = format!("Path \"{}\" is not a directory!", msg);
            err!(&msg, ErrorKind::InvalidFolder);
        }
        err!("Path is not a directory!", ErrorKind::InvalidFolder);
    }

    let dir_name;
    if let Some(val) = from.components().last() {
        dir_name = val.as_os_str();
    } else {
        err!("Invalid folder from", ErrorKind::InvalidFolder);
    }
    if (to.exists() || !options.copy_inside) && !options.content_only {
        to.push(dir_name);
    }

    let mut read_options = DirOptions::new();
    if options.depth > 0 {
        read_options.depth = options.depth;
    }

    let dir_content = get_dir_content2(from, &read_options)?;
    for directory in dir_content.directories {
        let tmp_to = Path::new(&directory).strip_prefix(from)?;
        let dir = to.join(&tmp_to);
        if !dir.exists() {
            if options.copy_inside {
                create_all(dir, false)?;
            } else {
                create(dir, false)?;
            }
        }
    }

    let mut result: u64 = 0;
    let mut info_process = TransitProcess {
        copied_bytes: 0,
        total_bytes: dir_content.dir_size,
        file_bytes_copied: 0,
        file_total_bytes: 0,
        file_name: String::new(),
        state: TransitState::Normal,
    };

    let mut options = options.clone();
    for file in dir_content.files {
        let mut to = to.to_path_buf();
        let tp = Path::new(&file).strip_prefix(from)?;
        let path = to.join(&tp);

        let file_name = path.file_name();
        if file_name.is_none() {
            err!("No file name");
        }
        let file_name = file_name.unwrap();
        to.push(file_name);

        let mut file_options = super::file::CopyOptions {
            overwrite: options.overwrite,
            skip_exist: options.skip_exist,
            buffer_size: options.buffer_size,
        };

        if let Some(file_name) = file_name.to_str() {
            info_process.file_name = file_name.to_string();
        } else {
            err!("Invalid file name", ErrorKind::InvalidFileName);
        }

        info_process.file_bytes_copied = 0;
        info_process.file_total_bytes = Path::new(&file).metadata()?.len();

        let mut result_copy: Result<u64>;
        let mut work = true;
        let copied_bytes = result;
        while work {
            {
                let _progress_handler = |info: super::file::TransitProcess| {
                    info_process.copied_bytes = copied_bytes + info.copied_bytes;
                    info_process.file_bytes_copied = info.copied_bytes;
                    progress_handler(info_process.clone());
                };

                result_copy =
                    super::file::copy_with_progress(&file, &path, &file_options, _progress_handler);
            }
            match result_copy {
                Ok(val) => {
                    result += val;
                    work = false;
                }
                Err(err) => match err.kind {
                    ErrorKind::AlreadyExists => {
                        let mut info_process = info_process.clone();
                        info_process.state = TransitState::Exists;
                        let user_decide = progress_handler(info_process);
                        match user_decide {
                            TransitProcessResult::Overwrite => {
                                file_options.overwrite = true;
                            }
                            TransitProcessResult::OverwriteAll => {
                                file_options.overwrite = true;
                                options.overwrite = true;
                            }
                            TransitProcessResult::Skip => {
                                file_options.skip_exist = true;
                            }
                            TransitProcessResult::SkipAll => {
                                file_options.skip_exist = true;
                                options.skip_exist = true;
                            }
                            TransitProcessResult::Retry => {}
                            TransitProcessResult::ContinueOrAbort => {
                                let err_msg = err.to_string();
                                err!(err_msg.as_str(), err.kind)
                            }
                            TransitProcessResult::Abort => {
                                let err_msg = err.to_string();
                                err!(err_msg.as_str(), err.kind)
                            }
                        }
                    }
                    ErrorKind::PermissionDenied => {
                        let mut info_process = info_process.clone();
                        info_process.state = TransitState::Exists;
                        let user_decide = progress_handler(info_process);
                        match user_decide {
                            TransitProcessResult::Overwrite => {
                                err!("Overwrite denied for this situation!", ErrorKind::Other);
                            }
                            TransitProcessResult::OverwriteAll => {
                                err!("Overwrite denied for this situation!", ErrorKind::Other);
                            }
                            TransitProcessResult::Skip => {
                                file_options.skip_exist = true;
                            }
                            TransitProcessResult::SkipAll => {
                                file_options.skip_exist = true;
                                options.skip_exist = true;
                            }
                            TransitProcessResult::Retry => {}
                            TransitProcessResult::ContinueOrAbort => {
                                let err_msg = err.to_string();
                                err!(err_msg.as_str(), err.kind)
                            }
                            TransitProcessResult::Abort => {
                                let err_msg = err.to_string();
                                err!(err_msg.as_str(), err.kind)
                            }
                        }
                    }
                    _ => {
                        let err_msg = err.to_string();
                        err!(err_msg.as_str(), err.kind)
                    }
                },
            }
        }
    }

    Ok(result)
}

/// Moves the directory contents from one place to another.
/// This function will also copy the permission bits of the original files to
/// destination files (not for directories).
///
/// # Errors
///
/// This function will return an error in the following situations, but is not limited to just
/// these cases:
///
/// * This `from` path is not a directory.
/// * This `from` directory does not exist.
/// * Invalid folder name for `from` or `to`.
/// * The current process does not have the permission to access `from` or write `to`.
///
/// # Example
/// ```rust,ignore
/// extern crate fs_extra;
/// use fs_extra::dir::move_dir;
///
/// let options = CopyOptions::new(); //Initialize default values for CopyOptions
///
/// // move source/dir1 to target/dir1
/// move_dir("source/dir1", "target/dir1", &options)?;
///
/// ```
pub fn move_dir<P, Q>(from: P, to: Q, options: &CopyOptions) -> Result<u64>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let mut is_remove = true;
    if options.skip_exist && to.as_ref().exists() && !options.overwrite {
        is_remove = false;
    }
    let from = from.as_ref();

    if !from.exists() {
        if let Some(msg) = from.to_str() {
            let msg = format!("Path \"{}\" does not exist", msg);
            err!(&msg, ErrorKind::NotFound);
        }
        err!(
            "Path does not exist or you don't have access!",
            ErrorKind::NotFound
        );
    }

    let mut to: PathBuf = to.as_ref().to_path_buf();
    if !from.is_dir() {
        if let Some(msg) = from.to_str() {
            let msg = format!(
                "Path \"{}\" is not a directory or you don't have access!",
                msg
            );
            err!(&msg, ErrorKind::InvalidFolder);
        }
        err!(
            "Path is not a directory or you don't have access!",
            ErrorKind::InvalidFolder
        );
    }
    let dir_name;
    if let Some(val) = from.components().last() {
        dir_name = val.as_os_str();
    } else {
        err!("Invalid folder from", ErrorKind::InvalidFolder);
    }

    if (to.exists() || !options.copy_inside) && !options.content_only {
        to.push(dir_name);
    }
    let dir_content = get_dir_content(from)?;
    for directory in dir_content.directories {
        let tmp_to = Path::new(&directory).strip_prefix(from)?;
        let dir = to.join(&tmp_to);
        if !dir.exists() {
            if options.copy_inside {
                create_all(dir, false)?;
            } else {
                create(dir, false)?;
            }
        }
    }
    let mut result: u64 = 0;
    for file in dir_content.files {
        let to = to.to_path_buf();
        let tp = Path::new(&file).strip_prefix(from)?;
        let path = to.join(&tp);

        let file_options = super::file::CopyOptions {
            overwrite: options.overwrite,
            skip_exist: options.skip_exist,
            buffer_size: options.buffer_size,
        };

        let mut result_copy: Result<u64>;
        let mut work = true;
        while work {
            {
                result_copy = super::file::move_file(&file, &path, &file_options);
                match result_copy {
                    Ok(val) => {
                        result += val;
                        work = false;
                    }
                    Err(err) => {
                        let err_msg = err.to_string();
                        err!(err_msg.as_str(), err.kind)
                    }
                }
            }
        }
    }
    if is_remove {
        remove(from)?;
    }

    Ok(result)
}

/// Moves the directory contents from one place to another with information about progress.
/// This function will also copy the permission bits of the original files to
/// destination files (not for directories).
///
/// # Errors
///
/// This function will return an error in the following situations, but is not limited to just
/// these cases:
///
/// * This `from` path is not a directory.
/// * This `from` directory does not exist.
/// * Invalid folder name for `from` or `to`.
/// * The current process does not have the permission to access `from` or write `to`.
///
/// # Example
/// ```rust,ignore
/// extern crate fs_extra;
/// use fs_extra::dir::move_dir_with_progress;
///
/// let options = CopyOptions::new(); //Initialize default values for CopyOptions
/// let handle = |process_info: TransitProcess| {
///     println!("{}", process_info.total_bytes);
///     fs_extra::dir::TransitProcessResult::ContinueOrAbort
/// }
///
/// // move source/dir1 to target/dir1
/// move_dir_with_progress("source/dir1", "target/dir1", &options, handle)?;
///
/// ```
pub fn move_dir_with_progress<P, Q, F>(
    from: P,
    to: Q,
    options: &CopyOptions,
    mut progress_handler: F,
) -> Result<u64>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
    F: FnMut(TransitProcess) -> TransitProcessResult,
{
    let mut is_remove = true;
    if options.skip_exist && to.as_ref().exists() && !options.overwrite {
        is_remove = false;
    }
    let from = from.as_ref();

    if !from.exists() {
        if let Some(msg) = from.to_str() {
            let msg = format!("Path \"{}\" does not exist or you don't have access!", msg);
            err!(&msg, ErrorKind::NotFound);
        }
        err!(
            "Path does not exist or you don't have access!",
            ErrorKind::NotFound
        );
    }

    let mut to: PathBuf = to.as_ref().to_path_buf();
    if !from.is_dir() {
        if let Some(msg) = from.to_str() {
            let msg = format!("Path \"{}\" is not a directory!", msg);
            err!(&msg, ErrorKind::InvalidFolder);
        }
        err!("Path is not a directory!", ErrorKind::InvalidFolder);
    }
    let dir_name;
    if let Some(val) = from.components().last() {
        dir_name = val.as_os_str();
    } else {
        err!("Invalid folder from", ErrorKind::InvalidFolder);
    }
    if !(options.content_only || options.copy_inside && !to.exists()) {
        to.push(dir_name);
    }

    let dir_content = get_dir_content(from)?;
    for directory in dir_content.directories {
        let tmp_to = Path::new(&directory).strip_prefix(from)?;
        let dir = to.join(&tmp_to);
        if !dir.exists() {
            if options.copy_inside {
                create_all(dir, false)?;
            } else {
                create(dir, false)?;
            }
        }
    }

    let mut result: u64 = 0;
    let mut info_process = TransitProcess {
        copied_bytes: 0,
        total_bytes: dir_content.dir_size,
        file_bytes_copied: 0,
        file_total_bytes: 0,
        file_name: String::new(),
        state: TransitState::Normal,
    };

    let mut options = options.clone();
    for file in dir_content.files {
        let mut to = to.to_path_buf();
        let tp = Path::new(&file).strip_prefix(from)?;
        let path = to.join(&tp);

        let file_name = path.file_name();
        if file_name.is_none() {
            err!("No file name");
        }
        let file_name = file_name.unwrap();
        to.push(file_name);

        let mut file_options = super::file::CopyOptions {
            overwrite: options.overwrite,
            skip_exist: options.skip_exist,
            buffer_size: options.buffer_size,
        };

        if let Some(file_name) = file_name.to_str() {
            info_process.file_name = file_name.to_string();
        } else {
            err!("Invalid file name", ErrorKind::InvalidFileName);
        }

        info_process.file_bytes_copied = 0;
        info_process.file_total_bytes = Path::new(&file).metadata()?.len();

        let mut result_copy: Result<u64>;
        let mut work = true;
        let copied_bytes = result;
        while work {
            {
                let _progress_handler = |info: super::file::TransitProcess| {
                    info_process.copied_bytes = copied_bytes + info.copied_bytes;
                    info_process.file_bytes_copied = info.copied_bytes;
                    progress_handler(info_process.clone());
                };

                result_copy = super::file::move_file_with_progress(
                    &file,
                    &path,
                    &file_options,
                    _progress_handler,
                );
            }
            match result_copy {
                Ok(val) => {
                    result += val;
                    work = false;
                }
                Err(err) => match err.kind {
                    ErrorKind::AlreadyExists => {
                        let mut info_process = info_process.clone();
                        info_process.state = TransitState::Exists;
                        let user_decide = progress_handler(info_process);
                        match user_decide {
                            TransitProcessResult::Overwrite => {
                                file_options.overwrite = true;
                            }
                            TransitProcessResult::OverwriteAll => {
                                file_options.overwrite = true;
                                options.overwrite = true;
                            }
                            TransitProcessResult::Skip => {
                                is_remove = false;
                                file_options.skip_exist = true;
                            }
                            TransitProcessResult::SkipAll => {
                                is_remove = false;
                                file_options.skip_exist = true;
                                options.skip_exist = true;
                            }
                            TransitProcessResult::Retry => {}
                            TransitProcessResult::ContinueOrAbort => {
                                let err_msg = err.to_string();
                                err!(err_msg.as_str(), err.kind)
                            }
                            TransitProcessResult::Abort => {
                                let err_msg = err.to_string();
                                err!(err_msg.as_str(), err.kind)
                            }
                        }
                    }
                    ErrorKind::PermissionDenied => {
                        let mut info_process = info_process.clone();
                        info_process.state = TransitState::Exists;
                        let user_decide = progress_handler(info_process);
                        match user_decide {
                            TransitProcessResult::Overwrite => {
                                err!("Overwrite denied for this situation!", ErrorKind::Other);
                            }
                            TransitProcessResult::OverwriteAll => {
                                err!("Overwrite denied for this situation!", ErrorKind::Other);
                            }
                            TransitProcessResult::Skip => {
                                is_remove = false;
                                file_options.skip_exist = true;
                            }
                            TransitProcessResult::SkipAll => {
                                file_options.skip_exist = true;
                                options.skip_exist = true;
                            }
                            TransitProcessResult::Retry => {}
                            TransitProcessResult::ContinueOrAbort => {
                                let err_msg = err.to_string();
                                err!(err_msg.as_str(), err.kind)
                            }
                            TransitProcessResult::Abort => {
                                let err_msg = err.to_string();
                                err!(err_msg.as_str(), err.kind)
                            }
                        }
                    }
                    _ => {
                        let err_msg = err.to_string();
                        err!(err_msg.as_str(), err.kind)
                    }
                },
            }
        }
    }
    if is_remove {
        remove(from)?;
    }

    Ok(result)
}

/// Removes directory.
///
/// # Example
/// ```rust,ignore
/// extern crate fs_extra;
/// use fs_extra::dir::remove;
///
/// remove("source/dir1"); // remove dir1
/// ```
pub fn remove<P: AsRef<Path>>(path: P) -> Result<()> {
    if path.as_ref().exists() {
        Ok(remove_dir_all(path)?)
    } else {
        Ok(())
    }
}
