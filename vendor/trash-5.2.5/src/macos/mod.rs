use std::{
    ffi::OsString,
    path::{Path, PathBuf},
    process::Command,
    time::SystemTime,
};

use log::trace;
use objc2_foundation::{NSFileManager, NSString, NSURL};

use crate::{fs_error, into_unknown, Error, TrashContext, TrashItem};

#[derive(Copy, Clone, Debug)]
/// There are 2 ways to trash files: via the ≝Finder app or via the OS NsFileManager call
///
///   | <br>Feature            |≝<br>Finder     |<br>NsFileManager |
///   |:-----------------------|:--------------:|:----------------:|
///   |Undo via "Put back"     | ✓              | ✗                |
///   |Speed                   | ✗<br>Slower    | ✓<br>Faster      |
///   |No sound                | ✗              | ✓                |
///   |No extra permissions    | ✗              | ✓                |
///
pub enum DeleteMethod {
    /// Use an `osascript`, asking the Finder application to delete the files.
    ///
    /// - Might ask the user to give additional permissions to the app
    /// - Produces the sound that Finder usually makes when deleting a file
    /// - Shows the "Put Back" option in the context menu, when using the Finder application
    ///
    Finder,

    /// Use `trashItemAtURL` from the `NSFileManager` object to delete the files.
    ///
    /// - Somewhat faster than the `Finder` method
    /// - Does *not* require additional permissions
    /// - Does *not* produce the sound that Finder usually makes when deleting a file
    /// - Does *not* show the "Put Back" option on some systems (the file may be restored by for
    ///   example dragging out from the Trash folder). This is a macOS bug. Read more about it
    ///   at:
    ///   - <https://github.com/sindresorhus/macos-trash/issues/4>
    ///   - <https://github.com/ArturKovacs/trash-rs/issues/14>
    ///
    /// This is the default.
    NsFileManager,
}
impl DeleteMethod {
    /// Returns `DeleteMethod::NsFileManager`
    pub const fn new() -> Self {
        DeleteMethod::NsFileManager
    }
}
impl Default for DeleteMethod {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Default, Debug)]
pub struct PlatformTrashContext {
    delete_method: DeleteMethod,
}
impl PlatformTrashContext {
    pub const fn new() -> Self {
        Self { delete_method: DeleteMethod::new() }
    }
}
pub trait TrashContextExtMacos {
    fn set_delete_method(&mut self, method: DeleteMethod);
    fn delete_method(&self) -> DeleteMethod;
}
impl TrashContextExtMacos for TrashContext {
    fn set_delete_method(&mut self, method: DeleteMethod) {
        self.platform_specific.delete_method = method;
    }
    fn delete_method(&self) -> DeleteMethod {
        self.platform_specific.delete_method
    }
}
impl TrashContext {
    pub(crate) fn delete_all_canonicalized(
        &self,
        full_paths: Vec<PathBuf>,
        with_info: bool,
    ) -> Result<Option<Vec<TrashItem>>, Error> {
        match self.platform_specific.delete_method {
            DeleteMethod::Finder => delete_using_finder(&full_paths, with_info),
            DeleteMethod::NsFileManager => delete_using_file_mgr(&full_paths, with_info),
        }
    }
}

fn delete_using_file_mgr<P: AsRef<Path>>(full_paths: &[P], with_info: bool) -> Result<Option<Vec<TrashItem>>, Error> {
    trace!("Starting delete_using_file_mgr");
    let file_mgr = NSFileManager::defaultManager();
    let mut trash_items = Vec::<TrashItem>::new();

    for path in full_paths {
        let path = path.as_ref().as_os_str().as_encoded_bytes();
        let path = match std::str::from_utf8(path) {
            Ok(path_utf8) => NSString::from_str(path_utf8), // utf-8 path, use as is
            Err(_) => NSString::from_str(&percent_encode(path)), // binary path, %-encode it
        };

        trace!("Starting fileURLWithPath");
        let url = NSURL::fileURLWithPath(&path);
        trace!("Finished fileURLWithPath");

        let mut trash_url = None;

        trace!("Calling trashItemAtURL");
        let res = file_mgr.trashItemAtURL_resultingItemURL_error(&url, Some(&mut trash_url));
        trace!("Finished trashItemAtURL");

        if let Err(err) = res {
            return Err(Error::Unknown {
                description: format!("While deleting '{:?}', `trashItemAtURL` failed: {err}", path),
            });
        }

        if with_info {
            trash_items.push(TrashItem {
                name: OsString::from(path.lastPathComponent().to_string()),
                original_parent: path.stringByDeletingLastPathComponent().to_string().into(),
                id: trash_url
                    .and_then(|url| url.path())
                    .map(|path| OsString::from(path.to_string()))
                    .unwrap_or_default(),
                time_deleted: SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .map(|duration| duration.as_secs() as i64)
                    .unwrap_or(-1),
            });
        }
    }

    if with_info {
        Ok(Some(trash_items))
    } else {
        Ok(None)
    }
}

fn delete_using_finder<P: AsRef<Path>>(full_paths: &[P], with_info: bool) -> Result<Option<Vec<TrashItem>>, Error> {
    // AppleScript command to move files (or directories) to Trash looks like
    // the snippet below, with `-e` being used to execute only one line of
    // AppleScript.
    //
    // ```
    // osascript -e 'tell application "Finder" to delete { POSIX file "file1", POSIX "file2" }'
    // ```
    let mut command = Command::new("osascript");
    let posix_files = full_paths
        .iter()
        .map(|path| {
            let path_bytes = path.as_ref().as_os_str().as_encoded_bytes();

            match std::str::from_utf8(path_bytes) {
                Ok(path_utf8) => format!(r#"POSIX file "{}""#, esc_quote(path_utf8)), // utf-8 path, escape \"
                Err(_) => format!(r#"POSIX file "{}""#, esc_quote(&percent_encode(path_bytes))), // binary path, %-encode it and escape \"
            }
        })
        .collect::<Vec<String>>()
        .join(", ");

    // When `with_info` is requested, we convert the Finder object references
    // returned by `delete` into POSIX paths using `as alias`. The results are
    // newline-delimited so we can split them reliably (commas would be
    // ambiguous for filenames containing commas).
    let script = if with_info {
        format!(
            r#"tell application "Finder"
    set trashedItems to delete {{ {posix_files} }}
    if class of trashedItems is not list then set trashedItems to {{trashedItems}}
    set posixPaths to ""
    repeat with t in trashedItems
        if posixPaths is not "" then set posixPaths to posixPaths & linefeed
        set posixPaths to posixPaths & POSIX path of (t as alias)
    end repeat
    return posixPaths
end tell"#
        )
    } else {
        format!("tell application \"Finder\" to delete {{ {posix_files} }}")
    };

    let argv: Vec<OsString> = vec!["-e".into(), script.into()];
    command.args(argv);

    // Execute command
    let result = command.output().map_err(into_unknown)?;
    if !result.status.success() {
        let stderr = String::from_utf8_lossy(&result.stderr);
        match result.status.code() {
            None => {
                return Err(Error::Unknown {
                    description: format!("The AppleScript exited with error. stderr: {}", stderr),
                })
            }

            Some(code) => {
                return Err(Error::Os {
                    code,
                    description: format!("The AppleScript exited with error. stderr: {}", stderr),
                })
            }
        };
    }

    if with_info {
        let stdout = String::from_utf8_lossy(&result.stdout);
        let time_deleted = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|duration| duration.as_secs() as i64)
            .unwrap_or(-1);

        // In practice, the Finder `delete` command returns results in the same
        // order as the input. We rely on this to pair trash paths with their
        // original paths via `.zip()`.
        let trash_items: Vec<TrashItem> = stdout
            .lines()
            .zip(full_paths.iter())
            .map(|(trash_path, original_path)| {
                let trash_path = Path::new(trash_path);
                let original = original_path.as_ref();

                TrashItem {
                    id: trash_path.as_os_str().to_os_string(),
                    name: original.file_name().map(|name| name.to_os_string()).unwrap_or_default(),
                    original_parent: original.parent().map(Path::to_owned).unwrap_or_default(),
                    time_deleted,
                }
            })
            .collect();

        Ok(Some(trash_items))
    } else {
        Ok(None)
    }
}

/// std's from_utf8_lossy, but non-utf8 byte sequences are %-encoded instead of being replaced by a special symbol.
/// Valid utf8, including `%`, are not escaped.
use std::borrow::Cow;
fn percent_encode(input: &[u8]) -> Cow<'_, str> {
    use percent_encoding::percent_encode_byte as b2pc;

    let mut iter = input.utf8_chunks().peekable();
    if let Some(chunk) = iter.peek() {
        if chunk.invalid().is_empty() {
            return Cow::Borrowed(chunk.valid());
        }
    } else {
        return Cow::Borrowed("");
    };

    let mut res = String::with_capacity(input.len());
    for chunk in iter {
        res.push_str(chunk.valid());
        let invalid = chunk.invalid();
        if !invalid.is_empty() {
            for byte in invalid {
                res.push_str(b2pc(*byte));
            }
        }
    }
    Cow::Owned(res)
}

/// Escapes `"` or `\` with `\` for use in AppleScript text
fn esc_quote(s: &str) -> Cow<'_, str> {
    if s.contains(['"', '\\']) {
        let mut r = String::with_capacity(s.len());
        let chars = s.chars();
        for c in chars {
            match c {
                '"' | '\\' => {
                    r.push('\\');
                    r.push(c);
                } // escapes quote/escape char
                _ => {
                    r.push(c);
                } // no escape required
            }
        }
        Cow::Owned(r)
    } else {
        Cow::Borrowed(s)
    }
}

/// Does a basic restore using file renaming, ignoring whether the
/// `DeleteMethod::NSFileManager` or `DeleteMethod::Finder` was used when
/// deleting the file, which means that files deleted with
/// `DeleteMethod::Finder` will not correctly update the `.DS_Store` file that
/// is kept in macOS' trash.
pub fn restore_all<I>(items: I) -> Result<(), Error>
where
    I: IntoIterator<Item = TrashItem>,
{
    let mut iter = items.into_iter();
    while let Some(item) = iter.next() {
        let original_path = item.original_path();
        let trash_path = Path::new(&item.id);

        // Ensure that both the trash item still exists, as well as that the
        // there's no collision on the original path before proceeding.
        if !std::fs::exists(&item.id).map_err(into_unknown)? {
            return Err(Error::Unknown { description: format!("Trash item not found at {:?}", item.id) });
        }

        if std::fs::exists(&original_path).map_err(|error| fs_error(&original_path, error))? {
            return Err(Error::RestoreCollision {
                path: original_path,
                remaining_items: std::iter::once(item).chain(iter).collect::<Vec<_>>(),
            });
        }

        std::fs::create_dir_all(&item.original_parent).map_err(|error| fs_error(&original_path, error))?;
        std::fs::rename(trash_path, &original_path).map_err(|error| fs_error(&original_path, error))?;
    }

    Ok(())
}

#[cfg(test)]
mod tests;
