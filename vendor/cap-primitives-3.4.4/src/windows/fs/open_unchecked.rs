//! Windows implementation of `openat` functionality.

#![allow(unsafe_code)]

use super::create_file_at_w::CreateFileAtW;
use super::{open_options_to_std, prepare_open_options_for_open};
use crate::fs::{
    errors, file_path, get_access_mode, get_creation_mode, get_flags_and_attributes,
    FollowSymlinks, OpenOptions, OpenUncheckedError, SymlinkKind,
};
use crate::{ambient_authority, AmbientAuthority};
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::os::windows::fs::MetadataExt;
use std::os::windows::io::{AsRawHandle, FromRawHandle, OwnedHandle};
use std::path::{Component, Path, PathBuf};
use std::{fs, io};
use windows_sys::Win32::Foundation::{self, ERROR_ACCESS_DENIED, HANDLE, INVALID_HANDLE_VALUE};
use windows_sys::Win32::Storage::FileSystem::{
    CreateFileW, FILE_ATTRIBUTE_DIRECTORY, FILE_FLAG_OPEN_REPARSE_POINT,
};

/// *Unsandboxed* function similar to `open`, but which does not perform
/// sandboxing.
pub(crate) fn open_unchecked(
    start: &fs::File,
    path: &Path,
    options: &OpenOptions,
) -> Result<fs::File, OpenUncheckedError> {
    let _ = ambient_authority;

    // We have the final `OpenOptions`; now prepare it for an `open`.
    let mut prepared_opts = options.clone();
    let manually_trunc = prepare_open_options_for_open(&mut prepared_opts);

    handle_open_result(
        open_at(start, path, &prepared_opts),
        options,
        manually_trunc,
    )
}

// The following is derived from Rust's library/std/src/sys/windows/fs.rs
// at revision 56888c1e9b4135b511abd2d8e907099003d12281, except with a
// directory `start` parameter added and using `CreateFileAtW` instead of
// `CreateFileW`.

fn open_at(start: &fs::File, path: &Path, opts: &OpenOptions) -> io::Result<fs::File> {
    let mut dir = start.as_raw_handle() as HANDLE;

    // `PathCchCanonicalizeEx` and friends don't seem to work with relative
    // paths. Or at least, when I tried it, they canonicalized "a" to "",
    // which isn't what we want. So we manually canonicalize `..` and `.`.
    // Hopefully there aren't other mysterious Windows path conventions that
    // we're missing here.
    let mut rebuilt = PathBuf::new();
    for component in path.components() {
        match component {
            Component::Prefix(_) | Component::RootDir => {
                rebuilt.push(component);
                dir = 0 as HANDLE;
            }
            Component::Normal(_) => {
                rebuilt.push(component);
            }
            Component::ParentDir => {
                if !rebuilt.pop() {
                    // We popped past the beginning of `path`. Substitute in
                    // the path of `start` and convert this to an ambient
                    // path by dropping the directory base. It's ok to do
                    // this because we're not sandboxing at this level of the
                    // code.
                    if dir == 0 as HANDLE {
                        return Err(io::Error::from_raw_os_error(ERROR_ACCESS_DENIED as _));
                    }
                    rebuilt = match file_path(start) {
                        Some(path) => path,
                        None => {
                            return Err(io::Error::from_raw_os_error(ERROR_ACCESS_DENIED as _));
                        }
                    };
                    dir = 0 as HANDLE;
                    // And then pop the last component of that.
                    let _ = rebuilt.pop();
                }
            }
            Component::CurDir => (),
        }
    }

    let mut wide = OsStr::encode_wide(rebuilt.as_os_str()).collect::<Vec<u16>>();

    // If we ended up re-rooting, use Windows' `CreateFileW` instead of our
    // own `CreateFileAtW` so that it does the requisite magic for absolute
    // paths.
    if dir == 0 as HANDLE {
        // We're calling the windows-sys `CreateFileW` which expects a
        // NUL-terminated filename, so add a NUL terminator.
        wide.push(0);

        let handle = unsafe {
            CreateFileW(
                wide.as_ptr(),
                get_access_mode(opts)?,
                opts.ext.share_mode,
                opts.ext.security_attributes,
                get_creation_mode(opts)?,
                get_flags_and_attributes(opts),
                0 as HANDLE,
            )
        };
        if handle != INVALID_HANDLE_VALUE {
            Ok(unsafe { fs::File::from_raw_handle(handle as _) })
        } else {
            Err(io::Error::last_os_error())
        }
    } else {
        // Our own `CreateFileAtW` is similar to `CreateFileW` except it
        // takes the filename as a Rust slice directly, so we can skip
        // the NUL terminator.
        let handle = unsafe {
            CreateFileAtW(
                dir,
                &wide,
                get_access_mode(opts)?,
                opts.ext.share_mode,
                opts.ext.security_attributes,
                get_creation_mode(opts)?,
                get_flags_and_attributes(opts),
                0 as HANDLE,
            )
        };

        if let Ok(handle) = handle.try_into() {
            Ok(<fs::File as From<OwnedHandle>>::from(handle))
        } else {
            Err(io::Error::last_os_error())
        }
    }
}

/// *Unsandboxed* function similar to `open_unchecked`, but which just operates
/// on a bare path, rather than starting with a handle.
pub(crate) fn open_ambient_impl(
    path: &Path,
    options: &OpenOptions,
    ambient_authority: AmbientAuthority,
) -> Result<fs::File, OpenUncheckedError> {
    let _ = ambient_authority;
    let (std_opts, manually_trunc) = open_options_to_std(options);
    handle_open_result(std_opts.open(path), options, manually_trunc)
}

fn handle_open_result(
    result: io::Result<fs::File>,
    options: &OpenOptions,
    manually_trunc: bool,
) -> Result<fs::File, OpenUncheckedError> {
    match result {
        Ok(f) => {
            let enforce_dir = options.dir_required;
            let enforce_nofollow = options.follow == FollowSymlinks::No
                && (options.ext.custom_flags & FILE_FLAG_OPEN_REPARSE_POINT) == 0;

            if enforce_dir || enforce_nofollow {
                let metadata = f.metadata().map_err(OpenUncheckedError::Other)?;

                if enforce_dir {
                    // Require a directory. It may seem possible to eliminate
                    // this `metadata()` call by appending a slash to the path
                    // before opening it so that the OS requires a directory
                    // for us, however on Windows in some circumstances this
                    // leads to "The filename, directory name, or volume label
                    // syntax is incorrect." errors.
                    //
                    // We check `file_attributes()` instead of using `is_dir()`
                    // since the latter returns false if we're looking at a
                    // directory symlink.
                    if metadata.file_attributes() & FILE_ATTRIBUTE_DIRECTORY == 0 {
                        return Err(OpenUncheckedError::Other(errors::is_not_directory()));
                    }
                }

                if enforce_nofollow {
                    // Windows doesn't have a way to return errors like
                    // `O_NOFOLLOW`, so if we're not following symlinks and
                    // we're not using `FILE_FLAG_OPEN_REPARSE_POINT` manually
                    // to open a symlink itself, check for symlinks and report
                    // them as a distinct error.
                    if metadata.file_type().is_symlink() {
                        return Err(OpenUncheckedError::Symlink(
                            io::Error::from_raw_os_error(
                                Foundation::ERROR_STOPPED_ON_SYMLINK as i32,
                            ),
                            if metadata.file_attributes() & FILE_ATTRIBUTE_DIRECTORY
                                == FILE_ATTRIBUTE_DIRECTORY
                            {
                                SymlinkKind::Dir
                            } else {
                                SymlinkKind::File
                            },
                        ));
                    }
                }
            }

            // Windows truncates symlinks into normal files, so truncation
            // may be disabled above; do it manually if needed. Note that this
            // is expected to always succeed for normal files, but this will
            // fail if a directory was opened as directories don't support
            // truncation.
            if manually_trunc {
                if let Err(e) = f.set_len(0) {
                    return Err(OpenUncheckedError::Other(e));
                }
            }
            Ok(f)
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => Err(OpenUncheckedError::NotFound(e)),
        Err(e) => match e.raw_os_error() {
            Some(code) => match code as u32 {
                Foundation::ERROR_FILE_NOT_FOUND | Foundation::ERROR_PATH_NOT_FOUND => {
                    Err(OpenUncheckedError::NotFound(e))
                }
                _ => Err(OpenUncheckedError::Other(e)),
            },
            None => Err(OpenUncheckedError::Other(e)),
        },
    }
}
