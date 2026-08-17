macro_rules! err {
    ($text:expr, $kind:expr) => {
        return Err(Error::new($kind, $text))
    };

    ($text:expr) => {
        err!($text, ErrorKind::Other)
    };
}

/// The error type for fs_extra operations on files and directories.
pub mod error;
/// This module includes additional methods for working with files.
///
/// One of the distinguishing features is receipt information
/// about process work with files.
///
/// # Example
/// ```rust,ignore
/// use std::path::Path;
/// use std::{thread, time};
/// use std::sync::mpsc::{self, TryRecvError};
///
/// extern crate fs_extra;
/// use fs_extra::file::*;
/// use fs_extra::error::*;
///
/// fn example_copy() -> Result<()> {
///     let path_from = Path::new("./temp");
///     let path_to = path_from.join("out");
///     let test_file = (path_from.join("test_file.txt"), path_to.join("test_file.txt"));
///
///
///     fs_extra::dir::create_all(&path_from, true)?;
///     fs_extra::dir::create_all(&path_to, true)?;
///
///     write_all(&test_file.0, "test_data")?;
///     assert!(test_file.0.exists());
///     assert!(!test_file.1.exists());
///
///
///     let options = CopyOptions {
///         buffer_size: 1,
///         ..Default::default()
///     }
///     let (tx, rx) = mpsc::channel();
///     thread::spawn(move || {
///         let handler = |process_info: TransitProcess| {
///             tx.send(process_info).unwrap();
///             thread::sleep(time::Duration::from_millis(500));
///         };
///         copy_with_progress(&test_file.0, &test_file.1, &options, handler).unwrap();
///         assert!(test_file.0.exists());
///         assert!(test_file.1.exists());
///
///     });
///     loop {
///         match rx.try_recv() {
///             Ok(process_info) => {
///                 println!("{} of {} bytes",
///                          process_info.copied_bytes,
///                          process_info.total_bytes);
///             }
///             Err(TryRecvError::Disconnected) => {
///                 println!("finished");
///                 break;
///             }
///             Err(TryRecvError::Empty) => {}
///         }
///     }
///     Ok(())
///
/// }
///
///
/// fn main() {
///     example_copy();
/// }
///
/// ```
pub mod file;

/// This module includes additional methods for working with directories.
///
/// One of the additional features is information
/// about process and recursion operations.
///
/// # Example
/// ```rust,ignore
/// use std::path::Path;
/// use std::{thread, time};
/// use std::sync::mpsc::{self, TryRecvError};
///
/// extern crate fs_extra;
/// use fs_extra::dir::*;
/// use fs_extra::error::*;
///
/// fn example_copy() -> Result<()> {
///
///     let path_from = Path::new("./temp");
///     let path_to = path_from.join("out");
///     let test_folder = path_from.join("test_folder");
///     let dir = test_folder.join("dir");
///     let sub = dir.join("sub");
///     let file1 = dir.join("file1.txt");
///     let file2 = sub.join("file2.txt");
///
///     create_all(&sub, true)?;
///     create_all(&path_to, true)?;
///     fs_extra::file::write_all(&file1, "content1")?;
///     fs_extra::file::write_all(&file2, "content2")?;
///
///     assert!(dir.exists());
///     assert!(sub.exists());
///     assert!(file1.exists());
///     assert!(file2.exists());
///
///
///     let options = CopyOptions {
///         buffer_size: 1,
///         ..Default::default(),
///     };
///     let (tx, rx) = mpsc::channel();
///     thread::spawn(move || {
///         let handler = |process_info: TransitProcess| {
///             tx.send(process_info).unwrap();
///             thread::sleep(time::Duration::from_millis(500));
///         };
///         copy_with_progress(&test_folder, &path_to, &options, handler).unwrap();
///     });
///
///     loop {
///         match rx.try_recv() {
///             Ok(process_info) => {
///                 println!("{} of {} bytes",
///                          process_info.copied_bytes,
///                          process_info.total_bytes);
///             }
///             Err(TryRecvError::Disconnected) => {
///                 println!("finished");
///                 break;
///             }
///             Err(TryRecvError::Empty) => {}
///         }
///     }
///     Ok(())
///
/// }
/// fn main() {
///     example_copy();
/// }
/// ```
///
pub mod dir;

use crate::error::*;
use std::path::Path;

/// Copies a list of directories and files to another place recursively. This function will
/// also copy the permission bits of the original files to destination files (not for
/// directories).
///
/// # Errors
///
/// This function will return an error in the following situations, but is not limited to just
/// these case:
///
/// * List `from` contains  file or directory does not exist.
///
/// * List `from` contains  file or directory with invalid name.
///
/// * The current process does not have the permission to access to file from `lists from` or
/// `to`.
///
/// # Example
///
/// ```rust,ignore
///  extern crate fs_extra;
///  use fs_extra::dir::copy;
///
///  let options = dir::CopyOptions::new(); //Initialize default values for CopyOptions
///
///  // copy dir1 and file1.txt to target/dir1 and target/file1.txt
///  let mut from_paths = Vec::new();
///  from_paths.push("source/dir1");
///  from_paths.push("source/file.txt");
///  copy_items(&from_paths, "target", &options)?;
/// ```
///
pub fn copy_items<P, Q>(from: &[P], to: Q, options: &dir::CopyOptions) -> Result<u64>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let mut result: u64 = 0;
    if options.content_only {
        err!(
            "Options 'content_only' not acccess for copy_items function",
            ErrorKind::Other
        );
    }
    for item in from {
        let item = item.as_ref();
        if item.is_dir() {
            result += dir::copy(item, &to, options)?;
        } else if let Some(file_name) = item.file_name() {
            if let Some(file_name) = file_name.to_str() {
                let file_options = file::CopyOptions {
                    overwrite: options.overwrite,
                    skip_exist: options.skip_exist,
                    ..Default::default()
                };
                result += file::copy(item, to.as_ref().join(file_name), &file_options)?;
            }
        } else {
            err!("Invalid file name", ErrorKind::InvalidFileName);
        }
    }

    Ok(result)
}

/// A structure which includes information about the current status of copying or moving a directory.
pub struct TransitProcess {
    /// Already copied bytes
    pub copied_bytes: u64,
    /// All the bytes which should be copied or moved (dir size).
    pub total_bytes: u64,
    /// Copied bytes on this time for file.
    pub file_bytes_copied: u64,
    /// Size of currently copied file.
    pub file_total_bytes: u64,
    /// Name of currently copied file.
    pub file_name: String,
    /// Name of currently copied folder.
    pub dir_name: String,
    /// Transit state
    pub state: dir::TransitState,
}

impl Clone for TransitProcess {
    fn clone(&self) -> TransitProcess {
        TransitProcess {
            copied_bytes: self.copied_bytes,
            total_bytes: self.total_bytes,
            file_bytes_copied: self.file_bytes_copied,
            file_total_bytes: self.file_total_bytes,
            file_name: self.file_name.clone(),
            dir_name: self.dir_name.clone(),
            state: self.state.clone(),
        }
    }
}

/// Copies a list of directories and files to another place recursively, with
/// information about progress. This function will also copy the permission bits of the
/// original files to destination files (not for directories).
///
/// # Errors
///
/// This function will return an error in the following situations, but is not limited to just
/// these case:
///
/// * List `from` contains  file or directory does not exist.
///
/// * List `from` contains  file or directory with invalid name.
///
/// * The current process does not have the permission to access to file from `lists from` or
/// `to`.
///
/// # Example
/// ```rust,ignore
///
///  extern crate fs_extra;
///  use fs_extra::dir::copy;
///
///  let options = dir::CopyOptions::new(); //Initialize default values for CopyOptions
///  let handle = |process_info: TransitProcess| {
///     println!("{}", process_info.total_bytes);
///     fs_extra::dir::TransitProcessResult::ContinueOrAbort
///  }
///  // copy dir1 and file1.txt to target/dir1 and target/file1.txt
///  let mut from_paths = Vec::new();
///  from_paths.push("source/dir1");
///  from_paths.push("source/file.txt");
///  copy_items_with_progress(&from_paths, "target", &options, handle)?;
/// ```
///
pub fn copy_items_with_progress<P, Q, F>(
    from: &[P],
    to: Q,
    options: &dir::CopyOptions,
    mut progress_handler: F,
) -> Result<u64>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
    F: FnMut(TransitProcess) -> dir::TransitProcessResult,
{
    if options.content_only {
        err!(
            "Options 'content_only' not access for copy_items_with_progress function",
            ErrorKind::Other
        );
    }
    let mut total_size = 0;
    let mut list_paths = Vec::new();
    for item in from {
        let item = item.as_ref();
        total_size += dir::get_size(item)?;
        list_paths.push(item);
    }

    let mut result: u64 = 0;
    let mut info_process = TransitProcess {
        copied_bytes: 0,
        total_bytes: total_size,
        file_bytes_copied: 0,
        file_total_bytes: 0,
        file_name: String::new(),
        dir_name: String::new(),
        state: dir::TransitState::Normal,
    };

    let mut options = options.clone();
    for item in list_paths {
        if item.is_dir() {
            if let Some(dir_name) = item.components().last() {
                if let Ok(dir_name) = dir_name.as_os_str().to_os_string().into_string() {
                    info_process.dir_name = dir_name;
                } else {
                    err!("Invalid folder from", ErrorKind::InvalidFolder);
                }
            } else {
                err!("Invalid folder from", ErrorKind::InvalidFolder);
            }

            let copied_bytes = result;
            let dir_options = options.clone();
            let handler = |info: dir::TransitProcess| {
                info_process.copied_bytes = copied_bytes + info.copied_bytes;
                info_process.state = info.state;
                let result = progress_handler(info_process.clone());
                match result {
                    dir::TransitProcessResult::OverwriteAll => options.overwrite = true,
                    dir::TransitProcessResult::SkipAll => options.skip_exist = true,
                    _ => {}
                }
                result
            };
            result += dir::copy_with_progress(item, &to, &dir_options, handler)?;
        } else {
            let mut file_options = file::CopyOptions {
                overwrite: options.overwrite,
                skip_exist: options.skip_exist,
                buffer_size: options.buffer_size,
            };

            if let Some(file_name) = item.file_name() {
                if let Some(file_name) = file_name.to_str() {
                    info_process.file_name = file_name.to_string();
                } else {
                    err!("Invalid file name", ErrorKind::InvalidFileName);
                }
            } else {
                err!("Invalid file name", ErrorKind::InvalidFileName);
            }

            info_process.file_bytes_copied = 0;
            info_process.file_total_bytes = item.metadata()?.len();

            let copied_bytes = result;
            let file_name = to.as_ref().join(info_process.file_name.clone());
            let mut work = true;

            let mut result_copy: Result<u64>;
            while work {
                {
                    let handler = |info: file::TransitProcess| {
                        info_process.copied_bytes = copied_bytes + info.copied_bytes;
                        info_process.file_bytes_copied = info.copied_bytes;
                        progress_handler(info_process.clone());
                    };
                    result_copy =
                        file::copy_with_progress(item, &file_name, &file_options, handler);
                }
                match result_copy {
                    Ok(val) => {
                        result += val;
                        work = false;
                    }
                    Err(err) => match err.kind {
                        ErrorKind::AlreadyExists => {
                            let mut info_process = info_process.clone();
                            info_process.state = dir::TransitState::Exists;
                            let user_decide = progress_handler(info_process);
                            match user_decide {
                                dir::TransitProcessResult::Overwrite => {
                                    file_options.overwrite = true;
                                }
                                dir::TransitProcessResult::OverwriteAll => {
                                    file_options.overwrite = true;
                                    options.overwrite = true;
                                }
                                dir::TransitProcessResult::Skip => {
                                    file_options.skip_exist = true;
                                }
                                dir::TransitProcessResult::SkipAll => {
                                    file_options.skip_exist = true;
                                    options.skip_exist = true;
                                }
                                dir::TransitProcessResult::Retry => {}
                                dir::TransitProcessResult::ContinueOrAbort => {
                                    let err_msg = err.to_string();
                                    err!(err_msg.as_str(), err.kind)
                                }
                                dir::TransitProcessResult::Abort => {
                                    let err_msg = err.to_string();
                                    err!(err_msg.as_str(), err.kind)
                                }
                            }
                        }
                        ErrorKind::PermissionDenied => {
                            let mut info_process = info_process.clone();
                            info_process.state = dir::TransitState::Exists;
                            let user_decide = progress_handler(info_process);
                            match user_decide {
                                dir::TransitProcessResult::Overwrite => {
                                    err!("Overwrite denied for this situation!", ErrorKind::Other);
                                }
                                dir::TransitProcessResult::OverwriteAll => {
                                    err!("Overwrite denied for this situation!", ErrorKind::Other);
                                }
                                dir::TransitProcessResult::Skip => {
                                    file_options.skip_exist = true;
                                }
                                dir::TransitProcessResult::SkipAll => {
                                    file_options.skip_exist = true;
                                    options.skip_exist = true;
                                }
                                dir::TransitProcessResult::Retry => {}
                                dir::TransitProcessResult::ContinueOrAbort => {
                                    let err_msg = err.to_string();
                                    err!(err_msg.as_str(), err.kind)
                                }
                                dir::TransitProcessResult::Abort => {
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
    }

    Ok(result)
}

/// Moves a list of directories and files to another place recursively. This function will
/// also copy the permission bits of the original files to destination files (not for
/// directories).
///
/// # Errors
///
/// This function will return an error in the following situations, but is not limited to just
/// these case:
///
/// * List `from` contains  file or directory does not exist.
///
/// * List `from` contains  file or directory with invalid name.
///
/// * The current process does not have the permission to access to file from `lists from` or
/// `to`.
///
/// # Example
///
/// ```rust,ignore
///  extern crate fs_extra;
///  use fs_extra::dir::copy;
///
///  let options = dir::CopyOptions::new(); //Initialize default values for CopyOptions
///
///  // move dir1 and file1.txt to target/dir1 and target/file1.txt
///  let mut from_paths = Vec::new();
///  from_paths.push("source/dir1");
///  from_paths.push("source/file.txt");
///  move_items(&from_paths, "target", &options)?;
/// ```
///
pub fn move_items<P, Q>(from_items: &[P], to: Q, options: &dir::CopyOptions) -> Result<u64>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    if options.content_only {
        err!(
            "Options 'content_only' not access for move_items function",
            ErrorKind::Other
        );
    }
    let mut total_size = 0;
    let mut list_paths = Vec::new();
    for item in from_items {
        let item = item.as_ref();
        total_size += dir::get_size(item)?;
        list_paths.push(item);
    }

    let mut result = 0;
    let mut info_process = TransitProcess {
        copied_bytes: 0,
        total_bytes: total_size,
        file_bytes_copied: 0,
        file_total_bytes: 0,
        file_name: String::new(),
        dir_name: String::new(),
        state: dir::TransitState::Normal,
    };

    for item in list_paths {
        if item.is_dir() {
            if let Some(dir_name) = item.components().last() {
                if let Ok(dir_name) = dir_name.as_os_str().to_os_string().into_string() {
                    info_process.dir_name = dir_name;
                } else {
                    err!("Invalid folder from", ErrorKind::InvalidFolder);
                }
            } else {
                err!("Invalid folder from", ErrorKind::InvalidFolder);
            }

            result += dir::move_dir(item, &to, options)?;
        } else {
            let file_options = file::CopyOptions {
                overwrite: options.overwrite,
                skip_exist: options.skip_exist,
                buffer_size: options.buffer_size,
            };

            if let Some(file_name) = item.file_name() {
                if let Some(file_name) = file_name.to_str() {
                    info_process.file_name = file_name.to_string();
                } else {
                    err!("Invalid file name", ErrorKind::InvalidFileName);
                }
            } else {
                err!("Invalid file name", ErrorKind::InvalidFileName);
            }

            info_process.file_bytes_copied = 0;
            info_process.file_total_bytes = item.metadata()?.len();

            let file_name = to.as_ref().join(info_process.file_name.clone());
            result += file::move_file(item, &file_name, &file_options)?;
        }
    }

    Ok(result)
}

/// Moves a list of directories and files to another place recursively, with
/// information about progress. This function will also copy the permission bits of the
/// original files to destination files (not for directories).
///
/// # Errors
///
/// This function will return an error in the following situations, but is not limited to just
/// these case:
///
/// * List `from` contains  file or directory does not exist.
///
/// * List `from` contains  file or directory with invalid name.
///
/// * The current process does not have the permission to access to file from `lists from` or
/// `to`.
///
/// # Example
///
/// ```rust,ignore
///  extern crate fs_extra;
///  use fs_extra::dir::copy;
///
///  let options = dir::CopyOptions::new(); //Initialize default values for CopyOptions
///  let handle = |process_info: TransitProcess| {
///     println!("{}", process_info.total_bytes);
///     fs_extra::dir::TransitProcessResult::ContinueOrAbort
///  }
///  // move dir1 and file1.txt to target/dir1 and target/file1.txt
///  let mut from_paths = Vec::new();
///  from_paths.push("source/dir1");
///  from_paths.push("source/file.txt");
///  move_items_with_progress(&from_paths, "target", &options, handle)?;
/// ```
///
pub fn move_items_with_progress<P, Q, F>(
    from_items: &[P],
    to: Q,
    options: &dir::CopyOptions,
    mut progress_handler: F,
) -> Result<u64>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
    F: FnMut(TransitProcess) -> dir::TransitProcessResult,
{
    if options.content_only {
        err!(
            "Options 'content_only' not access for move_items_with_progress function",
            ErrorKind::Other
        );
    }
    let mut total_size = 0;
    let mut list_paths = Vec::new();
    for item in from_items {
        let item = item.as_ref();
        total_size += dir::get_size(item)?;
        list_paths.push(item);
    }

    let mut result = 0;
    let mut info_process = TransitProcess {
        copied_bytes: 0,
        total_bytes: total_size,
        file_bytes_copied: 0,
        file_total_bytes: 0,
        file_name: String::new(),
        dir_name: String::new(),
        state: dir::TransitState::Normal,
    };
    let mut options = options.clone();

    for item in list_paths {
        if item.is_dir() {
            if let Some(dir_name) = item.components().last() {
                if let Ok(dir_name) = dir_name.as_os_str().to_os_string().into_string() {
                    info_process.dir_name = dir_name;
                } else {
                    err!("Invalid folder from", ErrorKind::InvalidFolder);
                }
            } else {
                err!("Invalid folder from", ErrorKind::InvalidFolder);
            }

            let copied_bytes = result;
            let dir_options = options.clone();
            let handler = |info: dir::TransitProcess| {
                info_process.copied_bytes = copied_bytes + info.copied_bytes;
                info_process.state = info.state;
                let result = progress_handler(info_process.clone());
                match result {
                    dir::TransitProcessResult::OverwriteAll => options.overwrite = true,
                    dir::TransitProcessResult::SkipAll => options.skip_exist = true,
                    _ => {}
                }
                result
            };
            result += dir::move_dir_with_progress(item, &to, &dir_options, handler)?;
        } else {
            let mut file_options = file::CopyOptions {
                overwrite: options.overwrite,
                skip_exist: options.skip_exist,
                buffer_size: options.buffer_size,
            };

            if let Some(file_name) = item.file_name() {
                if let Some(file_name) = file_name.to_str() {
                    info_process.file_name = file_name.to_string();
                } else {
                    err!("Invalid file name", ErrorKind::InvalidFileName);
                }
            } else {
                err!("Invalid file name", ErrorKind::InvalidFileName);
            }

            info_process.file_bytes_copied = 0;
            info_process.file_total_bytes = item.metadata()?.len();

            let copied_bytes = result;
            let file_name = to.as_ref().join(info_process.file_name.clone());
            let mut work = true;

            let mut result_copy: Result<u64>;
            while work {
                {
                    let handler = |info: file::TransitProcess| {
                        info_process.copied_bytes = copied_bytes + info.copied_bytes;
                        info_process.file_bytes_copied = info.copied_bytes;
                        progress_handler(info_process.clone());
                    };
                    result_copy =
                        file::move_file_with_progress(item, &file_name, &file_options, handler);
                }
                match result_copy {
                    Ok(val) => {
                        result += val;
                        work = false;
                    }
                    Err(err) => match err.kind {
                        ErrorKind::AlreadyExists => {
                            let mut info_process = info_process.clone();
                            info_process.state = dir::TransitState::Exists;
                            let user_decide = progress_handler(info_process);
                            match user_decide {
                                dir::TransitProcessResult::Overwrite => {
                                    file_options.overwrite = true;
                                }
                                dir::TransitProcessResult::OverwriteAll => {
                                    file_options.overwrite = true;
                                    options.overwrite = true;
                                }
                                dir::TransitProcessResult::Skip => {
                                    file_options.skip_exist = true;
                                }
                                dir::TransitProcessResult::SkipAll => {
                                    file_options.skip_exist = true;
                                    options.skip_exist = true;
                                }
                                dir::TransitProcessResult::Retry => {}
                                dir::TransitProcessResult::ContinueOrAbort => {
                                    let err_msg = err.to_string();
                                    err!(err_msg.as_str(), err.kind)
                                }
                                dir::TransitProcessResult::Abort => {
                                    let err_msg = err.to_string();
                                    err!(err_msg.as_str(), err.kind)
                                }
                            }
                        }
                        ErrorKind::PermissionDenied => {
                            let mut info_process = info_process.clone();
                            info_process.state = dir::TransitState::Exists;
                            let user_decide = progress_handler(info_process);
                            match user_decide {
                                dir::TransitProcessResult::Overwrite => {
                                    err!("Overwrite denied for this situation!", ErrorKind::Other);
                                }
                                dir::TransitProcessResult::OverwriteAll => {
                                    err!("Overwrite denied for this situation!", ErrorKind::Other);
                                }
                                dir::TransitProcessResult::Skip => {
                                    file_options.skip_exist = true;
                                }
                                dir::TransitProcessResult::SkipAll => {
                                    file_options.skip_exist = true;
                                    options.skip_exist = true;
                                }
                                dir::TransitProcessResult::Retry => {}
                                dir::TransitProcessResult::ContinueOrAbort => {
                                    let err_msg = err.to_string();
                                    err!(err_msg.as_str(), err.kind)
                                }
                                dir::TransitProcessResult::Abort => {
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
    }
    Ok(result)
}

/// Removes a list of files or directories.
///
/// # Example
///
/// ```rust,ignore
///  let mut from_paths = Vec::new();
///  from_paths.push("source/dir1");
///  from_paths.push("source/file.txt");
///
///  remove_items(&from_paths).unwrap();
/// ```
///
pub fn remove_items<P>(from_items: &[P]) -> Result<()>
where
    P: AsRef<Path>,
{
    for item in from_items {
        let item = item.as_ref();
        if item.is_dir() {
            dir::remove(item)?;
        } else {
            file::remove(item)?
        }
    }

    Ok(())
}
