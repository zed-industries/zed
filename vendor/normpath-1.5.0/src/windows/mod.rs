use std::borrow::Cow;
use std::env;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::io;
use std::ops::Not;
use std::os::windows::ffi::OsStrExt;
use std::os::windows::ffi::OsStringExt;
use std::path::Component;
use std::path::Path;
use std::path::Prefix;
use std::path::PrefixComponent;
use std::ptr;

use windows_sys::Win32::Storage::FileSystem::GetFullPathNameW;
use windows_sys::Win32::Storage::FileSystem::GetLongPathNameW;
use windows_sys::Win32::Storage::FileSystem::GetShortPathNameW;

use crate::BasePath;
use crate::BasePathBuf;

#[cfg(feature = "localization")]
pub(super) mod localize;

macro_rules! static_assert {
    ( $condition:expr ) => {
        const _: () = assert!($condition, "static assertion failed");
    };
}

fn is_separator(byte: &u8) -> bool {
    [b'/', b'\\'].contains(byte)
}

pub(crate) fn is_base(path: &Path) -> bool {
    matches!(path.components().next(), Some(Component::Prefix(_)))
}

pub(crate) fn to_base(path: &Path) -> io::Result<BasePathBuf> {
    let base = env::current_dir()?;
    debug_assert!(is_base(&base));

    let mut base = BasePathBuf(base);
    base.push(path);
    Ok(base)
}

#[inline(always)]
const fn u32_to_usize(n: u32) -> usize {
    // This assertion should never fail.
    static_assert!(size_of::<u32>() <= size_of::<usize>());
    n as usize
}

fn winapi_buffered<F>(mut call_fn: F) -> io::Result<Vec<u16>>
where
    F: FnMut(*mut u16, u32) -> u32,
{
    let mut buffer = Vec::new();
    let mut capacity = 0;
    loop {
        capacity = call_fn(buffer.as_mut_ptr(), capacity);
        if capacity == 0 {
            break Err(io::Error::last_os_error());
        }

        let length = u32_to_usize(capacity);
        let Some(mut additional_capacity) =
            length.checked_sub(buffer.capacity())
        else {
            // SAFETY: These characters were initialized by the syscall.
            unsafe {
                buffer.set_len(length);
            }
            return Ok(buffer);
        };
        assert_ne!(0, additional_capacity);

        // WinAPI can recommend an insufficient capacity that causes it to
        // return incorrect results, so extra space is reserved as a
        // workaround.
        let extra_capacity = 2.min(capacity.not());
        capacity += extra_capacity;
        additional_capacity += u32_to_usize(extra_capacity);

        buffer.reserve(additional_capacity);
    }
}

fn winapi_path(
    path: &Path,
    call_fn: fn(*const u16, *mut u16, u32) -> u32,
) -> io::Result<Cow<'_, Path>> {
    if path.as_os_str().as_encoded_bytes().contains(&0) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "strings passed to WinAPI cannot contains NULs",
        ));
    }

    match path.components().next() {
        // Verbatim paths should not be modified.
        Some(Component::Prefix(prefix)) if prefix.kind().is_verbatim() => {
            return Ok(Cow::Borrowed(path));
        }
        Some(Component::RootDir)
            if path
                .as_os_str()
                .as_encoded_bytes()
                .get(1)
                .is_some_and(is_separator) =>
        {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "partial UNC prefixes are invalid",
            ));
        }
        _ => {}
    }

    let mut path: Vec<_> = path.as_os_str().encode_wide().collect();
    debug_assert!(!path.contains(&0));
    path.push(0);

    path = winapi_buffered(|buffer, capacity| {
        call_fn(path.as_ptr(), buffer, capacity)
    })?;

    Ok(Cow::Owned(OsString::from_wide(&path).into()))
}

pub(crate) fn normalize_virtually(path: &Path) -> io::Result<BasePathBuf> {
    winapi_path(path, |path, buffer, capacity| unsafe {
        GetFullPathNameW(path, capacity, buffer, ptr::null_mut())
    })
    .map(|x| BasePathBuf(x.into_owned()))
}

pub(crate) fn normalize(path: &Path) -> io::Result<BasePathBuf> {
    // Trigger an error for nonexistent paths for consistency with other
    // platforms.
    let _ = path.metadata()?;
    normalize_virtually(path)
}

pub(crate) fn expand(path: &Path) -> io::Result<Cow<'_, Path>> {
    winapi_path(path, |path, buffer, capacity| unsafe {
        GetLongPathNameW(path, buffer, capacity)
    })
}

pub(crate) fn shorten(path: &Path) -> io::Result<Cow<'_, Path>> {
    winapi_path(path, |path, buffer, capacity| unsafe {
        GetShortPathNameW(path, buffer, capacity)
    })
}

fn get_prefix(base: &BasePath) -> PrefixComponent<'_> {
    if let Some(Component::Prefix(prefix)) = base.components().next() {
        prefix
    } else {
        // Base paths should always have a prefix.
        panic!(
            "base path is missing a prefix: \"{}\"",
            base.as_path().display(),
        );
    }
}

fn convert_separators(path: &Path) -> Cow<'_, OsStr> {
    let path_bytes = path.as_os_str().as_encoded_bytes();
    let mut parts = path_bytes.split(|&x| x == b'/');

    let part = parts.next().expect("split iterator is empty");
    if part.len() == path_bytes.len() {
        debug_assert_eq!(path_bytes, part);
        debug_assert_eq!(None, parts.next());
        return Cow::Borrowed(path.as_os_str());
    }

    let mut path_bytes = Vec::with_capacity(path_bytes.len());
    path_bytes.extend(part);
    for part in parts {
        path_bytes.push(b'\\');
        path_bytes.extend(part);
    }

    // SAFETY: Only UTF-8 substrings were replaced.
    Cow::Owned(unsafe { OsString::from_encoded_bytes_unchecked(path_bytes) })
}

fn push_separator(base: &mut BasePathBuf) {
    // Add a separator if necessary.
    base.0.push("");
}

pub(crate) fn push(base: &mut BasePathBuf, path: &Path) {
    let mut components = path.components();
    let mut next_component = components.next();
    match next_component {
        Some(Component::Prefix(prefix)) => {
            // Verbatim paths should not be modified.
            let mut absolute = prefix.kind().is_verbatim();
            if !absolute {
                next_component = components.next();
                // Other prefixes are absolute, except drive-relative prefixes.
                absolute = !matches!(prefix.kind(), Prefix::Disk(_))
                    || prefix.kind() != get_prefix(base).kind()
                    // Equivalent to [path.has_root()] but more efficient.
                    || next_component == Some(Component::RootDir);
            }
            if absolute {
                *base = BasePathBuf(path.to_owned());
                return;
            }
        }
        Some(Component::RootDir) => {
            let mut buffer = get_prefix(base).as_os_str().to_owned();
            buffer.push(convert_separators(path));
            *base = BasePathBuf(buffer.into());
            return;
        }
        _ => {
            while let Some(component) = next_component {
                match component {
                    Component::CurDir => {}
                    Component::ParentDir if base.pop().is_ok() => {}
                    _ => break,
                }
                next_component = components.next();
            }
        }
    }

    if let Some(component) = next_component {
        push_separator(base);
        base.0.as_mut_os_string().push(component);

        let components = components.as_path();
        if !components.as_os_str().is_empty() {
            push_separator(base);
            base.0
                .as_mut_os_string()
                .push(convert_separators(components));
        }
    }

    let path_bytes = path.as_os_str().as_encoded_bytes();
    // At least one separator should be kept.
    if path_bytes.last().is_some_and(is_separator)
        || matches!(path_bytes, [x, b'.'] if is_separator(x))
    {
        push_separator(base);
    }
}
