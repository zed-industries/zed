use crate::fs::{target_o_path, FollowSymlinks, OpenOptions};
use rustix::fs::OFlags;
use std::io;

pub(in super::super) fn compute_oflags(options: &OpenOptions) -> io::Result<OFlags> {
    let mut oflags = OFlags::CLOEXEC;
    oflags |= get_access_mode(options)?;
    oflags |= get_creation_mode(options)?;
    if options.follow == FollowSymlinks::No {
        oflags |= OFlags::NOFOLLOW;
    }
    if options.sync {
        oflags |= OFlags::SYNC;
    }
    if options.dsync {
        #[cfg(not(target_os = "freebsd"))]
        {
            oflags |= OFlags::DSYNC;
        }

        // Where needed, approximate `DSYNC` with `SYNC`.
        #[cfg(target_os = "freebsd")]
        {
            oflags |= OFlags::SYNC;
        }
    }
    #[cfg(not(any(
        target_os = "ios",
        target_os = "macos",
        target_os = "tvos",
        target_os = "watchos",
        target_os = "visionos",
        target_os = "freebsd",
        target_os = "fuchsia"
    )))]
    if options.rsync {
        oflags |= OFlags::RSYNC;
    }
    if options.nonblock {
        oflags |= OFlags::NONBLOCK;
    }
    if options.dir_required {
        oflags |= OFlags::DIRECTORY;

        // If the target has `O_PATH`, we don't need to read the directory
        // entries, and we're not requesting write access (which need to
        // fail on a directory), use it.
        if !options.readdir_required && !options.write && !options.append {
            oflags |= target_o_path();
        }
    }
    // Use `RWMODE` here instead of `ACCMODE` so that we preserve the `O_PATH`
    // flag.
    #[cfg(not(target_os = "wasi"))]
    {
        oflags |= OFlags::from_bits(options.ext.custom_flags as _).expect("unrecognized OFlags")
            & !OFlags::RWMODE;
    }
    Ok(oflags)
}

// `OpenOptions` translation code derived from Rust's
// library/std/src/sys/unix/fs.rs at revision
// 108e90ca78f052c0c1c49c42a22c85620be19712.

pub(crate) fn get_access_mode(options: &OpenOptions) -> io::Result<OFlags> {
    match (options.read, options.write, options.append) {
        (true, false, false) => Ok(OFlags::RDONLY),
        (false, true, false) => Ok(OFlags::WRONLY),
        (true, true, false) => Ok(OFlags::RDWR),
        (false, _, true) => Ok(OFlags::WRONLY | OFlags::APPEND),
        (true, _, true) => Ok(OFlags::RDWR | OFlags::APPEND),
        (false, false, false) => Err(rustix::io::Errno::INVAL.into()),
    }
}

pub(crate) fn get_creation_mode(options: &OpenOptions) -> io::Result<OFlags> {
    match (options.write, options.append) {
        (true, false) => {}
        (false, false) => {
            if options.truncate || options.create || options.create_new {
                return Err(rustix::io::Errno::INVAL.into());
            }
        }
        (_, true) => {
            if options.truncate && !options.create_new {
                return Err(rustix::io::Errno::INVAL.into());
            }
        }
    }

    Ok(
        match (options.create, options.truncate, options.create_new) {
            (false, false, false) => OFlags::empty(),
            (true, false, false) => OFlags::CREATE,
            (false, true, false) => OFlags::TRUNC,
            (true, true, false) => OFlags::CREATE | OFlags::TRUNC,
            (_, _, true) => OFlags::CREATE | OFlags::EXCL,
        },
    )
}
