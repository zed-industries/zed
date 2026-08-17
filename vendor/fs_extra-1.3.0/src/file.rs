use crate::error::{Error, ErrorKind, Result};
use std;
use std::fs::{remove_file, File};
use std::io::{Read, Write};
use std::path::Path;

// Options and flags which can be used to configure how a file will be  copied  or moved.
pub struct CopyOptions {
    /// Sets the option true for overwrite existing files.
    pub overwrite: bool,
    /// Sets the option true for skip existing files.
    pub skip_exist: bool,
    /// Sets buffer size for copy/move work only with receipt information about process work.
    pub buffer_size: usize,
}

impl CopyOptions {
    /// Initialize struct CopyOptions with default value.
    ///
    /// ```rust,ignore
    ///
    /// overwrite: false
    ///
    /// skip_exist: false
    ///
    /// buffer_size: 64000 //64kb
    /// ```
    pub fn new() -> CopyOptions {
        CopyOptions {
            overwrite: false,
            skip_exist: false,
            buffer_size: 64000, //64kb
        }
    }

    /// Sets the option true for overwrite existing files.
    pub fn overwrite(mut self, overwrite: bool) -> Self {
        self.overwrite = overwrite;
        self
    }

    /// Sets the option true for skip existing files.
    pub fn skip_exist(mut self, skip_exist: bool) -> Self {
        self.skip_exist = skip_exist;
        self
    }

    /// Sets buffer size for copy/move work only with receipt information about process work.
    pub fn buffer_size(mut self, buffer_size: usize) -> Self {
        self.buffer_size = buffer_size;
        self
    }
}

impl Default for CopyOptions {
    fn default() -> Self {
        CopyOptions::new()
    }
}

/// A structure which stores information about the current status of a file that's copied or moved. .
pub struct TransitProcess {
    /// Copied bytes on this time.
    pub copied_bytes: u64,
    /// All the bytes which should to copy or move.
    pub total_bytes: u64,
}

/// Copies the contents of one file to another. This function will also copy the permission
/// bits of the original file to the destination file.
///
/// # Errors
///
/// This function will return an error in the following situations, but is not limited to just
/// these cases:
///
/// * This `from` path is not a file.
/// * This `from` file does not exist.
/// * The current process does not have the permission to access `from` or write `to`.
///
/// # Example
///
/// ```rust,ignore
/// extern crate fs_extra;
/// use fs_extra::file::copy;
///
/// let options = CopyOptions::new(); //Initialize default values for CopyOptions
/// copy("dir1/foo.txt", "dir2/bar.txt", &options)?; // Copy dir1/foo.txt to dir2/bar.txt
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
            "Path does not exist or you don't have access!",
            ErrorKind::NotFound
        );
    }

    if !from.is_file() {
        if let Some(msg) = from.to_str() {
            let msg = format!("Path \"{}\" is not a file!", msg);
            err!(&msg, ErrorKind::InvalidFile);
        }
        err!("Path is not a file!", ErrorKind::InvalidFile);
    }

    if !options.overwrite && to.as_ref().exists() {
        if options.skip_exist {
            return Ok(0);
        }

        if let Some(msg) = to.as_ref().to_str() {
            let msg = format!("Path \"{}\" exists", msg);
            err!(&msg, ErrorKind::AlreadyExists);
        }
    }

    Ok(std::fs::copy(from, to)?)
}

/// Copies the contents of one file to another file with information about progress.
/// This function will also copy the permission bits of the original file to the
/// destination file.
///
/// # Errors
///
/// This function will return an error in the following situations, but is not limited to just
/// these cases:
///
/// * This `from` path is not a file.
/// * This `from` file does not exist.
/// * The current process does not have the permission to access `from` or write `to`.
///
/// # Example
/// ```rust,ignore
/// extern crate fs_extra;
/// use fs_extra::file::copy_with_progress;
///
/// let options = CopyOptions::new(); //Initialize default values for CopyOptions
/// let handle = |process_info: TransitProcess|  println!("{}", process_info.total_bytes);
///
/// // Copy dir1/foo.txt to dir2/foo.txt
/// copy_with_progress("dir1/foo.txt", "dir2/foo.txt", &options, handle)?;
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
    F: FnMut(TransitProcess),
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

    if !from.is_file() {
        if let Some(msg) = from.to_str() {
            let msg = format!("Path \"{}\" is not a file!", msg);
            err!(&msg, ErrorKind::InvalidFile);
        }
        err!("Path is not a file!", ErrorKind::InvalidFile);
    }

    if !options.overwrite && to.as_ref().exists() {
        if options.skip_exist {
            return Ok(0);
        }

        if let Some(msg) = to.as_ref().to_str() {
            let msg = format!("Path \"{}\" exists", msg);
            err!(&msg, ErrorKind::AlreadyExists);
        }
    }
    let mut file_from = File::open(from)?;
    let mut buf = vec![0; options.buffer_size];
    let file_size = file_from.metadata()?.len();
    let mut copied_bytes: u64 = 0;

    let mut file_to = File::create(to)?;
    while !buf.is_empty() {
        match file_from.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                let written_bytes = file_to.write(&buf[..n])?;
                if written_bytes != n {
                    err!("Couldn't write the whole buffer to file", ErrorKind::Other);
                }
                copied_bytes += n as u64;
                let data = TransitProcess {
                    copied_bytes,
                    total_bytes: file_size,
                };
                progress_handler(data);
            }
            Err(ref e) if e.kind() == ::std::io::ErrorKind::Interrupted => {}
            Err(e) => return Err(::std::convert::From::from(e)),
        }
    }
    Ok(file_size)
}

/// Moves a file from one place to another. This function will also copy the permission
/// bits of the original file to the destination file.
///
/// # Errors
///
/// This function will return an error in the following situations, but is not limited to just
/// these cases:
///
/// * This `from` path is not a file.
/// * This `from` file does not exist.
/// * The current process does not have the permission to access `from` or write `to`.
///
/// # Example
/// ```rust,ignore
/// extern crate fs_extra;
/// use fs_extra::file::move_file;
///
/// let options = CopyOptions::new(); //Initialize default values for CopyOptions
/// move_file("dir1/foo.txt", "dir2/foo.txt", &options)?; // Move dir1/foo.txt to dir2/foo.txt
///
/// ```
pub fn move_file<P, Q>(from: P, to: Q, options: &CopyOptions) -> Result<u64>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let mut is_remove = true;
    if options.skip_exist && to.as_ref().exists() && !options.overwrite {
        is_remove = false;
    }
    let result = copy(&from, to, options)?;
    if is_remove {
        remove(from)?;
    }

    Ok(result)
}

/// Moves a file from one place to another with information about progress.
/// This function will also copy the permission bits of the original file to the
/// destination file.
///
/// # Errors
///
/// This function will return an error in the following situations, but is not limited to just
/// these cases:
///
/// * This `from` path is not a file.
/// * This `from` file does not exist.
/// * The current process does not have the permission to access `from` or write `to`.
///
/// # Example
/// ```rust,ignore
/// extern crate fs_extra;
/// use fs_extra::file::move_file;
///
/// let options = CopyOptions::new(); //Initialize default values for CopyOptions
/// let handle = |process_info: TransitProcess|  println!("{}", process_info.total_bytes);
///
/// // Move dir1/foo.txt to dir2/foo.txt
/// move_file("dir1/foo.txt", "dir2/foo.txt", &options, handle)?;
///
/// ```
pub fn move_file_with_progress<P, Q, F>(
    from: P,
    to: Q,
    options: &CopyOptions,
    progress_handler: F,
) -> Result<u64>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
    F: FnMut(TransitProcess),
{
    let mut is_remove = true;
    if options.skip_exist && to.as_ref().exists() && !options.overwrite {
        is_remove = false;
    }
    let result = copy_with_progress(&from, to, options, progress_handler)?;
    if is_remove {
        remove(from)?;
    }

    Ok(result)
}

/// Removes a file from the filesystem.
///
/// # Errors
///
/// This function will return an error in the following situations, but is not limited to just
/// these cases:
///
/// * The current process does not have the permission to access `path`.
///
/// # Example
/// ```rust,ignore
/// extern crate fs_extra;
/// use fs_extra::file::remove;
///
/// remove("foo.txt" )?; // Remove foo.txt
///
/// ```
pub fn remove<P>(path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    if path.as_ref().exists() {
        Ok(remove_file(path)?)
    } else {
        Ok(())
    }
}

/// Read file contents, placing them into `String`.
///
/// # Errors
///
/// This function will return an error in the following situations, but is not limited to just
/// these cases:
///
/// * This `path` is not a file.
/// * This `path` file does not exist.
/// * The current process does not have the permission to access `path`.
///
/// # Example
/// ```rust,ignore
/// extern crate fs_extra;
/// use fs_extra::file::read_to_string;
///
/// let file_content = read_to_string("foo.txt" )?; // Get file content from foo.txt
/// println!("{}", file_content);
///
/// ```
pub fn read_to_string<P>(path: P) -> Result<String>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    if path.exists() && !path.is_file() {
        if let Some(msg) = path.to_str() {
            let msg = format!("Path \"{}\" is not a file!", msg);
            err!(&msg, ErrorKind::InvalidFile);
        }
        err!("Path is not a file!", ErrorKind::InvalidFile);
    }

    let mut file = File::open(path)?;
    let mut result = String::new();
    file.read_to_string(&mut result)?;

    Ok(result)
}

/// Write `String` content into file.
///
/// # Errors
///
/// This function will return an error in the following situations, but is not limited to just
/// these cases:
///
/// * This `path` is not a file.
/// * This `path` file does not exist.
/// * The current process does not have the permission to access `path`.
///
/// # Example
/// ```rust,ignore
/// extern crate fs_extra;
/// use fs_extra::file::read_to_string;
///
/// write_all("foo.txt", "contents" )?; // Create file foo.txt and send content inside
///
/// ```
pub fn write_all<P>(path: P, content: &str) -> Result<()>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    if path.exists() && !path.is_file() {
        if let Some(msg) = path.to_str() {
            let msg = format!("Path \"{}\" is not a file!", msg);
            err!(&msg, ErrorKind::InvalidFile);
        }
        err!("Path is not a file!", ErrorKind::InvalidFile);
    }

    let mut f = File::create(path)?;

    Ok(f.write_all(content.as_bytes())?)
}
