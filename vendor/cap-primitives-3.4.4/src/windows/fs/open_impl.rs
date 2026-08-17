use crate::fs::{manually, OpenOptions};
use std::ffi::OsStr;
use std::path::Path;
use std::{fs, io};
use windows_sys::Win32::Foundation::ERROR_FILE_NOT_FOUND;

pub(crate) fn open_impl(
    start: &fs::File,
    path: &Path,
    options: &OpenOptions,
) -> io::Result<fs::File> {
    // Windows reserves several special device paths. Disallow opening any
    // of them.
    // See: https://learn.microsoft.com/en-us/windows/win32/fileio/naming-a-file#naming-conventions
    if let Some(stem) = file_prefix(path) {
        if let Some(stemstr) = stem.to_str() {
            match stemstr.trim_end().to_uppercase().as_str() {
                "CON" | "PRN" | "AUX" | "NUL" | "COM0" | "COM1" | "COM2" | "COM3" | "COM4"
                | "COM5" | "COM6" | "COM7" | "COM8" | "COM9" | "COM¹" | "COM²" | "COM³"
                | "LPT0" | "LPT1" | "LPT2" | "LPT3" | "LPT4" | "LPT5" | "LPT6" | "LPT7"
                | "LPT8" | "LPT9" | "LPT¹" | "LPT²" | "LPT³" => {
                    return Err(io::Error::from_raw_os_error(ERROR_FILE_NOT_FOUND as i32));
                }
                _ => {}
            }
        }
    }

    manually::open(start, path, options)
}

// TODO: Replace this with `Path::file_prefix` once that's stable. For now,
// we use a copy of the code. This code is derived from
// https://github.com/rust-lang/rust/blob/9fe9041cc8eddaed402d17aa4facb2ce8f222e95/library/std/src/path.rs#L2648
fn file_prefix(path: &Path) -> Option<&OsStr> {
    path.file_name()
        .map(split_file_at_dot)
        .and_then(|(before, _after)| Some(before))
}

// This code is derived from
// https://github.com/rust-lang/rust/blob/9fe9041cc8eddaed402d17aa4facb2ce8f222e95/library/std/src/path.rs#L340
#[allow(unsafe_code)]
fn split_file_at_dot(file: &OsStr) -> (&OsStr, Option<&OsStr>) {
    let slice = file.as_encoded_bytes();
    if slice == b".." {
        return (file, None);
    }

    // The unsafety here stems from converting between &OsStr and &[u8]
    // and back. This is safe to do because (1) we only look at ASCII
    // contents of the encoding and (2) new &OsStr values are produced
    // only from ASCII-bounded slices of existing &OsStr values.
    let i = match slice[1..].iter().position(|b| *b == b'.') {
        Some(i) => i + 1,
        None => return (file, None),
    };
    let before = &slice[..i];
    let after = &slice[i + 1..];
    unsafe {
        (
            OsStr::from_encoded_bytes_unchecked(before),
            Some(OsStr::from_encoded_bytes_unchecked(after)),
        )
    }
}
