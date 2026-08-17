use crate::fs::{AccessType, FollowSymlinks};
use rustix::fs::{Access, AtFlags};
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
    let mut access_type = Access::empty();
    match type_ {
        AccessType::Exists => access_type |= Access::EXISTS,
        AccessType::Access(modes) => {
            if modes.readable {
                access_type |= Access::READ_OK;
            }
            if modes.writable {
                access_type |= Access::WRITE_OK;
            }
            if modes.executable {
                access_type |= Access::EXEC_OK;
            }
        }
    }

    let atflags = match follow {
        FollowSymlinks::Yes => AtFlags::empty(),
        FollowSymlinks::No => AtFlags::SYMLINK_NOFOLLOW,
    };

    rustix::fs::accessat(start, path, access_type, atflags)?;

    Ok(())
}
