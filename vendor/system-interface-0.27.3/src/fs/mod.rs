//! Filesystem extension traits.

mod fd_flags;
mod file_io_ext;

pub use fd_flags::{FdFlags, GetSetFdFlags, SetFdFlags};
pub use file_io_ext::{Advice, FileIoExt};

// Windows quirks:
//  - Open dir can't be renamed or deleted
//  - symlinks are different
//  - seek past end of file files unspec rather than zero

// TODO: test that remove_dir can remove symlink_dirs, and remove_file files.

// TODO: poll
