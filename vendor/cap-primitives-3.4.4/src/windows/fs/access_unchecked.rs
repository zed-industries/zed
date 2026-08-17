use crate::fs::{open, AccessType, FollowSymlinks, OpenOptions};
use std::path::Path;
use std::{fs, io};

/// *Unsandboxed* function similar to `access`, but which does not perform
/// sandboxing.
pub(crate) fn access_unchecked(
    start: &fs::File,
    path: &Path,
    type_: AccessType,
    follow: FollowSymlinks,
) -> io::Result<()> {
    let mut options = OpenOptions::new();
    options.follow(follow);
    match type_ {
        AccessType::Exists => {
            options.read(true);
        }
        AccessType::Access(modes) => {
            if modes.readable {
                options.read(true);
            }
            if modes.writable {
                options.write(true);
            }
            if modes.executable {
                options.read(true);
            }
        }
    }
    open(start, path, &options).map(|_| ())
}
